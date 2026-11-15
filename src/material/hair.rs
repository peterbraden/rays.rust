//! # Hair
//!
//! A hair is a dielectric cylinder that absorbs light inside it. Light meeting it reflects off
//! its surface (R), passes through it (TT), or passes in, reflects off the far side and comes back
//! out (TRT), and so on; the cuticle's tilted scales shift each lobe along the hair. Stephen R.
//! Marschner, Henrik Wann Jensen, Mike Cammarano, Steve Worley and Pat Hanrahan, "Light
//! Scattering from Human Hair Fibers", SIGGRAPH 2003, doi:10.1145/1201775.882345; Eugene d'Eon,
//! Guillaume François, Martin Hill, Joe Letteri and Jean-Marie Aubry, "An Energy-Conserving Hair
//! Reflectance Model", EGSR 2011, doi:10.1111/j.1467-8659.2011.01976.x; Matt Jen-Yuan Chiang,
//! Benedikt Bitterli, Chuck Tappan and Brent Burley, "A Practical and Controllable Hair and Fur
//! Model for Production Path Tracing", Computer Graphics Forum 35(2), 2016,
//! doi:10.1111/cgf.12830; as pbrt's `HairBxDF` (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 4th ed., section 9.9; cited below as PBR), which this follows term for term.
//!
//! In a frame with x along the hair, directions are given by their angle θ from the plane
//! perpendicular to it (sin θ = ω·x) and their azimuth φ round it. The BSDF is a sum over the
//! number of internal paths p:
//!
//! ```text
//! f(ω_o, ω_i) = Σ_p M_p(θ_o, θ_i) A_p(θ_o, h) N_p(φ_i - φ_o) / |cos θ_i|
//! ```
//!
//! - M_p, longitudinal scattering: d'Eon et al.'s model with a modified Bessel function I₀, of
//!   variance v_p from the longitudinal roughness β_m (Chiang et al.'s fit), each lobe's θ_o
//!   shifted by the scales' tilt α (2α, -α, -4α for R, TT, TRT).
//! - A_p, attenuation: Fresnel's reflectance at the surface, and Beer's law through the hair,
//!   T = exp(-σ_a 2 cos γ_t / cos θ_t), for the refracted path at offset h across the width
//!   (h = sin γ_o, -1 to 1); the remainder of p ≥ 3 summed as a geometric series.
//! - N_p, azimuthal scattering: a logistic distribution (of scale s from the azimuthal roughness
//!   β_n), trimmed to [-π, π], about the angle a path's refractions and reflections turn it
//!   through, Φ(p) = 2p γ_t - 2γ_o + pπ.
//!
//! σ_a comes from the concentrations of the two melanins, eumelanin (brown to black) and
//! pheomelanin (red), or from a colour, by Chiang et al.'s inversion of the multiply scattered
//! albedo of a head of hair (PBR's `SigmaAFromReflectance`).
//!
//! The model needs where across the hair a ray hit, h. Cylinder curves (`shapes::curve`) turn
//! their shading normal across their width, from facing the ray in the middle to sideways at the
//! edges; h is the normal's component across the hair. Their facing direction is the frame's z.
//! Light sampling's shadow rays leave from the side the ray arrived on, so direct light through
//! the hair (TT) from behind is shadowed by the hair itself; BSDF-sampled rays carry it.
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::bsdf::{BSDF, BSDFSample};
use crate::material::functions::{direct_lighting_mis, surface_offset};
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::material::rough_dielectric::fresnel_dielectric;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use std::f64::consts::PI;

/// Lobes modelled individually: R, TT, TRT; the rest summed.
const P_MAX: usize = 3;

fn safe_sqrt(x: f64) -> f64 {
    x.max(0.).sqrt()
}

fn safe_asin(x: f64) -> f64 {
    x.clamp(-1., 1.).asin()
}

/// The modified Bessel function of the first kind, order zero, by its series.
fn i0(x: f64) -> f64 {
    let (mut val, mut x2i, mut ifact, mut i4) = (0., 1., 1., 1.);
    for i in 0..10 {
        if i > 1 {
            ifact *= i as f64;
        }
        val += x2i / (i4 * ifact * ifact);
        x2i *= x * x;
        i4 *= 4.;
    }
    val
}

fn log_i0(x: f64) -> f64 {
    if x > 12. {
        x + 0.5 * (-(2. * PI).ln() + (1. / x).ln() + 1. / (8. * x))
    } else {
        i0(x).ln()
    }
}

/// d'Eon et al.'s longitudinal scattering of variance `v`.
fn mp(cos_i: f64, cos_o: f64, sin_i: f64, sin_o: f64, v: f64) -> f64 {
    let a = cos_i * cos_o / v;
    let b = sin_i * sin_o / v;
    if v <= 0.1 {
        (log_i0(a) - b - 1. / v + 0.6931 + (1. / (2. * v)).ln()).exp()
    } else {
        ((-b).exp() * i0(a)) / ((1. / v).sinh() * 2. * v)
    }
}

fn logistic(x: f64, s: f64) -> f64 {
    let x = x.abs();
    let e = (-x / s).exp();
    e / (s * (1. + e) * (1. + e))
}

fn logistic_cdf(x: f64, s: f64) -> f64 {
    1. / (1. + (-x / s).exp())
}

fn trimmed_logistic(x: f64, s: f64, a: f64, b: f64) -> f64 {
    logistic(x, s) / (logistic_cdf(b, s) - logistic_cdf(a, s))
}

fn sample_trimmed_logistic(u: f64, s: f64, a: f64, b: f64) -> f64 {
    let k = logistic_cdf(b, s) - logistic_cdf(a, s);
    let x = -s * (1. / (u * k + logistic_cdf(a, s)) - 1.).ln();
    x.clamp(a, b)
}

/// The azimuth a path of p internal segments turns through.
fn phi(p: usize, gamma_o: f64, gamma_t: f64) -> f64 {
    2. * p as f64 * gamma_t - 2. * gamma_o + p as f64 * PI
}

fn np(phi_: f64, p: usize, s: f64, gamma_o: f64, gamma_t: f64) -> f64 {
    let mut d = phi_ - phi(p, gamma_o, gamma_t);
    while d > PI { d -= 2. * PI; }
    while d < -PI { d += 2. * PI; }
    trimmed_logistic(d, s, -PI, PI)
}

/// σ_a of melanin concentrations (PBR's `SigmaAFromConcentration`).
pub fn sigma_a_from_melanin(eumelanin: f64, pheomelanin: f64) -> Color {
    let eu = Color::new(0.419, 0.697, 1.37);
    let pheo = Color::new(0.187, 0.4, 1.05);
    eu * eumelanin + pheo * pheomelanin
}

/// σ_a giving a head of hair colour `c` at azimuthal roughness `beta_n` (PBR's
/// `SigmaAFromReflectance`, after Chiang et al.).
pub fn sigma_a_from_color(c: Color, beta_n: f64) -> Color {
    let b = beta_n;
    let d = 5.969 - 0.215 * b + 2.532 * b * b - 10.73 * b.powi(3) + 5.574 * b.powi(4) + 0.245 * b.powi(5);
    Color { rgb: c.rgb.map(|v| (v.max(1e-6).ln() / d).powi(2)) }
}

/// The hair BSDF at one point, in a frame with x along the hair and z facing the viewer.
pub struct HairBSDF {
    pub frame: [Vector3<f64>; 3],
    pub h: f64,
    pub eta: f64,
    pub sigma_a: Color,
    v: [f64; P_MAX + 1],
    s: f64,
    sin2k_alpha: [f64; 3],
    cos2k_alpha: [f64; 3],
}

impl HairBSDF {
    /// `beta_m`, `beta_n` the longitudinal and azimuthal roughness (0 to 1), `alpha` the scales'
    /// tilt in degrees.
    pub fn new(frame: [Vector3<f64>; 3], h: f64, eta: f64, sigma_a: Color, beta_m: f64, beta_n: f64, alpha: f64) -> HairBSDF {
        let v0 = (0.726 * beta_m + 0.812 * beta_m * beta_m + 3.7 * beta_m.powi(20)).powi(2);
        let v = [v0, 0.25 * v0, 4. * v0, 4. * v0];
        let s = (PI / 8.).sqrt() * (0.265 * beta_n + 1.194 * beta_n * beta_n + 5.372 * beta_n.powi(22));
        let mut sin2k = [0.; 3];
        let mut cos2k = [0.; 3];
        sin2k[0] = alpha.to_radians().sin();
        cos2k[0] = safe_sqrt(1. - sin2k[0] * sin2k[0]);
        for i in 1..3 {
            sin2k[i] = 2. * cos2k[i - 1] * sin2k[i - 1];
            cos2k[i] = cos2k[i - 1] * cos2k[i - 1] - sin2k[i - 1] * sin2k[i - 1];
        }
        HairBSDF { frame, h: h.clamp(-1., 1.), eta, sigma_a, v, s, sin2k_alpha: sin2k, cos2k_alpha: cos2k }
    }

    fn local(&self, w: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(w.dot(&self.frame[0]), w.dot(&self.frame[1]), w.dot(&self.frame[2]))
    }

    fn world(&self, w: &Vector3<f64>) -> Vector3<f64> {
        self.frame[0] * w.x + self.frame[1] * w.y + self.frame[2] * w.z
    }

    /// θ_o shifted for lobe p by the scales' tilt.
    fn tilted(&self, p: usize, sin_o: f64, cos_o: f64) -> (f64, f64) {
        let (s, c) = match p {
            0 => (sin_o * self.cos2k_alpha[1] - cos_o * self.sin2k_alpha[1], cos_o * self.cos2k_alpha[1] + sin_o * self.sin2k_alpha[1]),
            1 => (sin_o * self.cos2k_alpha[0] + cos_o * self.sin2k_alpha[0], cos_o * self.cos2k_alpha[0] - sin_o * self.sin2k_alpha[0]),
            2 => (sin_o * self.cos2k_alpha[2] + cos_o * self.sin2k_alpha[2], cos_o * self.cos2k_alpha[2] - sin_o * self.sin2k_alpha[2]),
            _ => (sin_o, cos_o),
        };
        (s, c.abs())
    }

    /// The refracted path's γ_t and its transmittance through the hair, for view θ_o.
    fn path(&self, sin_o: f64, cos_o: f64) -> (f64, Color) {
        let sin_t = sin_o / self.eta;
        let cos_t = safe_sqrt(1. - sin_t * sin_t);
        let etap = safe_sqrt(self.eta * self.eta - sin_o * sin_o) / cos_o;
        let sin_gamma_t = self.h / etap;
        let cos_gamma_t = safe_sqrt(1. - sin_gamma_t * sin_gamma_t);
        let gamma_t = safe_asin(sin_gamma_t);
        let t = Color { rgb: self.sigma_a.rgb.map(|s| (-s * (2. * cos_gamma_t / cos_t)).exp()) };
        (gamma_t, t)
    }

    /// The attenuation of each lobe (PBR's `Ap`).
    fn ap(&self, cos_o: f64, t: Color) -> [Color; P_MAX + 1] {
        let cos_gamma_o = safe_sqrt(1. - self.h * self.h);
        let f = fresnel_dielectric(cos_o * cos_gamma_o, self.eta);
        let mut ap = [Color::black(); P_MAX + 1];
        ap[0] = Color::white() * f;
        ap[1] = t * ((1. - f) * (1. - f));
        for p in 2..P_MAX {
            ap[p] = ap[p - 1] * t * f;
        }
        let tf = t * f;
        ap[P_MAX] = Color { rgb: (ap[P_MAX - 1] * tf).rgb.zip_map(&tf.rgb, |a, b| if b < 1. { a / (1. - b) } else { 0. }) };
        ap
    }

    /// The probability of sampling each lobe: its share of the attenuation.
    fn ap_pdf(&self, cos_o: f64, sin_o: f64) -> [f64; P_MAX + 1] {
        let (_, t) = self.path(sin_o, cos_o);
        let ap = self.ap(cos_o, t);
        let sum: f64 = ap.iter().map(|a| a.rgb.sum()).sum();
        let mut pdf = [0.; P_MAX + 1];
        for p in 0..=P_MAX {
            pdf[p] = if sum > 0. { ap[p].rgb.sum() / sum } else { 0. };
        }
        pdf
    }

    /// f (not times the cosine), local directions.
    fn f(&self, o: &Vector3<f64>, i: &Vector3<f64>) -> Color {
        let (sin_o, cos_o) = (o.x, safe_sqrt(1. - o.x * o.x));
        let (sin_i, cos_i) = (i.x, safe_sqrt(1. - i.x * i.x));
        let phi_o = o.z.atan2(o.y);
        let phi_i = i.z.atan2(i.y);
        let gamma_o = safe_asin(self.h);
        let (gamma_t, t) = self.path(sin_o, cos_o);
        let ap = self.ap(cos_o, t);
        let dphi = phi_i - phi_o;
        let mut sum = Color::black();
        for p in 0..P_MAX {
            let (s, c) = self.tilted(p, sin_o, cos_o);
            sum = sum + ap[p] * (mp(cos_i, c, sin_i, s, self.v[p]) * np(dphi, p, self.s, gamma_o, gamma_t));
        }
        sum + ap[P_MAX] * (mp(cos_i, cos_o, sin_i, sin_o, self.v[P_MAX]) / (2. * PI))
    }

    fn pdf_local(&self, o: &Vector3<f64>, i: &Vector3<f64>) -> f64 {
        let (sin_o, cos_o) = (o.x, safe_sqrt(1. - o.x * o.x));
        let (sin_i, cos_i) = (i.x, safe_sqrt(1. - i.x * i.x));
        let phi_o = o.z.atan2(o.y);
        let phi_i = i.z.atan2(i.y);
        let gamma_o = safe_asin(self.h);
        let (gamma_t, _) = self.path(sin_o, cos_o);
        let ap = self.ap_pdf(cos_o, sin_o);
        let dphi = phi_i - phi_o;
        let mut pdf = 0.;
        for p in 0..P_MAX {
            let (s, c) = self.tilted(p, sin_o, cos_o);
            pdf += mp(cos_i, c, sin_i, s, self.v[p]) * ap[p] * np(dphi, p, self.s, gamma_o, gamma_t);
        }
        pdf + mp(cos_i, cos_o, sin_i, sin_o, self.v[P_MAX]) * ap[P_MAX] / (2. * PI)
    }
}

impl BSDF for HairBSDF {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        // f |cos θ_i| with pbrt's f, which is already divided by |cos θ_i|: the measure is the
        // sphere's, so f here is the full fsum.
        self.f(&self.local(wo), &self.local(wi))
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        self.pdf_local(&self.local(wo), &self.local(wi))
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        let o = self.local(wo);
        let (sin_o, cos_o) = (o.x, safe_sqrt(1. - o.x * o.x));
        let phi_o = o.z.atan2(o.y);
        let gamma_o = safe_asin(self.h);
        // Choose a lobe, and reuse what's left of the number.
        let ap = self.ap_pdf(cos_o, sin_o);
        let mut uc = u[2];
        let mut p = 0;
        while p < P_MAX && uc >= ap[p] {
            uc -= ap[p];
            p += 1;
        }
        if ap[p] <= 0. {
            return None;
        }
        uc = (uc / ap[p]).clamp(0., 1. - f64::EPSILON);
        let (s, c) = self.tilted(p, sin_o, cos_o);
        // θ_i from M_p.
        let u0 = u[0].max(1e-5);
        let cos_theta = 1. + self.v[p] * (u0 + (1. - u0) * (-2. / self.v[p]).exp()).ln();
        let sin_theta = safe_sqrt(1. - cos_theta * cos_theta);
        let cos_phi = (2. * PI * u[1]).cos();
        let sin_i = -cos_theta * s + sin_theta * cos_phi * c;
        let cos_i = safe_sqrt(1. - sin_i * sin_i);
        // Δφ from N_p.
        let (gamma_t, _) = self.path(sin_o, cos_o);
        let dphi = if p < P_MAX {
            phi(p, gamma_o, gamma_t) + sample_trimmed_logistic(uc, self.s, -PI, PI)
        } else {
            2. * PI * uc
        };
        let phi_i = phi_o + dphi;
        let i = Vector3::new(sin_i, cos_i * phi_i.cos(), cos_i * phi_i.sin());
        let wi = self.world(&i);
        let pdf = self.pdf_local(&o, &i);
        if pdf <= 0. || !pdf.is_finite() {
            return None;
        }
        Some(BSDFSample { wi, weight: self.f(&o, &i) / pdf, pdf })
    }
}

/// How a hair's colour is given.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pigment {
    Melanin { eumelanin: f64, pheomelanin: f64 },
    Color(Color),
    SigmaA(Color),
}

pub struct Hair {
    pub pigment: Pigment,
    pub eta: f64,
    pub beta_m: f64,
    pub beta_n: f64,
    pub alpha: f64,
}

impl Hair {
    fn sigma_a(&self) -> Color {
        let s = match self.pigment {
            Pigment::Melanin { eumelanin, pheomelanin } => sigma_a_from_melanin(eumelanin, pheomelanin),
            Pigment::Color(c) => sigma_a_from_color(c, self.beta_n),
            Pigment::SigmaA(s) => s,
        };
        s.as_unbounded()
    }
}

impl MaterialModel for Hair {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let wo = -r.rd.normalize();
        let n = intersection.normal;
        // Along the hair, facing the viewer, and across: h from the normal's turn across it.
        let tangent = intersection.object.geometry.tangent(&intersection.point, intersection.face)
            .unwrap_or_else(|| crate::material::phase::orthonormal_basis(&n).0);
        let facing = (wo - tangent * wo.dot(&tangent)).try_normalize(1e-12).unwrap_or(n);
        let across = facing.cross(&tangent);
        let h = n.dot(&across).clamp(-1., 1.);
        let bsdf = HairBSDF::new([tangent, across, facing], h, self.eta, self.sigma_a(), self.beta_m, self.beta_n, self.alpha);
        let emit = direct_lighting_mis(&bsdf, &intersection.point, &wo, s);
        crate::material::ggx::scattered();
        match bsdf.sample(&wo, [crate::geometry::rand(), crate::geometry::rand(), crate::geometry::rand()]) {
            Some(sample) => {
                // Through the hair: start beyond it.
                let behind = sample.wi.dot(&facing) < 0.;
                let ro = if behind { intersection.point - facing * (2. * surface_offset(intersection, s)) } else { intersection.point };
                ScatteredRay { emit, attenuate: sample.weight, ray: Some(Ray { ro, rd: sample.wi }), light_sampling: LightSampling::MIS { bsdf_pdf: sample.pdf } }
            },
            None => ScatteredRay { emit, attenuate: Color::black(), ray: None, light_sampling: LightSampling::MIS { bsdf_pdf: 0. } },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn frame() -> [Vector3<f64>; 3] {
        [Vector3::x(), Vector3::y(), Vector3::z()]
    }

    /// With no absorption, a hair loses no light: f integrated over the sphere is one, for any
    /// view, offset and roughness (PBR's white furnace test for `HairBxDF`).
    #[test]
    fn test_white_furnace() {
        let mut rng = Pcg32::new(1, 7);
        for _ in 0..10 {
            let beta_m = 0.2 + 0.8 * rng.next_f64();
            let beta_n = 0.2 + 0.8 * rng.next_f64();
            let h = -1. + 2. * rng.next_f64();
            let wo = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let hair = HairBSDF::new(frame(), h, 1.55, Color::black(), beta_m, beta_n, 0.);
            let n = 100_000;
            let sum: f64 = (0..n).map(|_| {
                let wi = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
                hair.eval(&wo, &wi).rgb.y * 4. * PI
            }).sum::<f64>() / n as f64;
            assert!((sum - 1.).abs() < 0.05, "beta_m {} beta_n {} h {}: {}", beta_m, beta_n, h, sum);
        }
    }

    /// Sampling agrees with eval and pdf, and its weights estimate the same integral as uniform
    /// sampling.
    #[test]
    fn test_sampling_is_consistent() {
        let mut rng = Pcg32::new(4, 2);
        let hair = HairBSDF::new(frame(), 0.3, 1.55, sigma_a_from_melanin(1.3, 0.), 0.3, 0.3, 2.);
        let wo = Vector3::new(0.3, 0.5, 0.8).normalize();
        let n = 200_000;
        let mut sampled = Color::black();
        for _ in 0..n {
            if let Some(s) = hair.sample(&wo, [rng.next_f64(), rng.next_f64(), rng.next_f64()]) {
                let pdf = hair.pdf(&wo, &s.wi);
                assert!((pdf - s.pdf).abs() <= 1e-9 * pdf.max(1.));
                sampled = sampled + s.weight;
            }
        }
        let sampled = sampled / n as f64;
        let uniform = (0..n).fold(Color::black(), |a, _| {
            let wi = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            a + hair.eval(&wo, &wi) * (4. * PI)
        }) / n as f64;
        for c in 0..3 {
            assert!((sampled.rgb[c] / uniform.rgb[c] - 1.).abs() < 0.03, "sampled {:?} uniform {:?}", sampled, uniform);
        }
        // The pdf integrates to one.
        let integral = (0..n).map(|_| hair.pdf(&wo, &crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64())) * 4. * PI).sum::<f64>() / n as f64;
        assert!((integral - 1.).abs() < 0.03, "{}", integral);
    }

    #[test]
    fn test_melanin_darkens_and_colours() {
        // More eumelanin, darker; pheomelanin absorbs blue most, so reddens.
        let light = sigma_a_from_melanin(0.3, 0.);
        let dark = sigma_a_from_melanin(8., 0.);
        assert!(dark.rgb.min() > light.rgb.max());
        let red = sigma_a_from_melanin(0., 2.);
        assert!(red.rgb.z > red.rgb.x);
        // A requested colour's σ_a absorbs least where the colour is brightest.
        let s = sigma_a_from_color(Color::new(0.8, 0.5, 0.2), 0.3);
        assert!(s.rgb.x < s.rgb.y && s.rgb.y < s.rgb.z);
    }
}
