//! # Rough and thin dielectrics
//!
//! ## Rough glass
//!
//! Frosted glass and rough ice both reflect and transmit off microfacets: each facet a smooth
//! interface with Fresnel's reflectance, its normal drawn from the GGX distribution (`ggx`).
//! Bruce Walter, Stephen R. Marschner, Hongsong Li and Kenneth E. Torrance, "Microfacet Models
//! for Refraction through Rough Surfaces", EGSR 2007, doi:10.2312/EGWR/EGSR07/195-206; as pbrt's
//! `DielectricBxDF` (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., section
//! 9.7; cited below as PBR). For light passing from index n_o to n_i (η' = n_i / n_o, or 1 for
//! reflection), the generalised half vector is ω_m ∝ η' ω_i + ω_o, and
//!
//! ```text
//! reflection:   f = D(ω_m) G(ω_o, ω_i) F / (4 |cos θ_o cos θ_i|)
//! transmission: f = D(ω_m) G(ω_o, ω_i) (1 - F) |ω_i·ω_m| |ω_o·ω_m| / (|cos θ_o cos θ_i| (ω_i·ω_m + ω_o·ω_m / η')²)
//! ```
//!
//! (Walter et al., equations 20 and 21), the transmitted radiance divided by η'² as it is
//! squeezed into a narrower cone (PBR section 9.5.2). A facet normal is sampled from the visible
//! normals, then reflection or refraction chosen in proportion to F and 1 - F. F is Fresnel's
//! exact reflectance for unpolarised light, not Schlick's approximation. Rays that refract start
//! on the far side of the surface, as the smooth dielectric's do.
//!
//! Light a facet sends back through the surface (into another facet, in reality) is lost: the
//! model scatters once. Nearly smooth, all the light is accounted for; at α = 0.3, 98% seen from
//! outside but 82% from inside, where facets beyond the critical angle reflect totally (the
//! test's measurements). Multiple-scattering microfacet models would recover it.
//!
//! ## Thin glass
//!
//! A window pane is two parallel interfaces, too close together to matter: light either passes
//! straight through or reflects, and the reflections inside it add up as a geometric series,
//! R' = R + T² R / (1 - R²) with T = 1 - R (PBR section 9.5.1, `ThinDielectricBxDF`). Modelled as
//! a single surface, a pane needs no thickness.
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::bsdf::{BSDF, BSDFSample};
use crate::material::functions::{direct_lighting_mis, reflect, surface_offset};
use crate::material::ggx::{regularised, Fresnel, GGX, MIN_ALPHA};
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::spectrum::RefractiveIndex;

/// Fresnel's reflectance for unpolarised light at an interface of relative index `eta` (inside
/// over outside), for a direction at cosine `cos` from the outward normal: negative from inside
/// (PBR's `FrDielectric`).
pub fn fresnel_dielectric(cos: f64, eta: f64) -> f64 {
    let (mut cos_i, mut eta) = (cos.clamp(-1., 1.), eta);
    if cos_i < 0. {
        eta = 1. / eta;
        cos_i = -cos_i;
    }
    let sin2_t = (1. - cos_i * cos_i) / (eta * eta);
    if sin2_t >= 1. {
        return 1.;
    }
    let cos_t = (1. - sin2_t).sqrt();
    let r_parl = (eta * cos_i - cos_t) / (eta * cos_i + cos_t);
    let r_perp = (cos_i - eta * cos_t) / (cos_i + eta * cos_t);
    (r_parl * r_parl + r_perp * r_perp) / 2.
}

/// The rough dielectric's BSDF about the outward normal `normal`, with relative index `eta`.
pub struct RoughDielectricBSDF {
    pub ggx: GGX,
    pub eta: f64,
    pub tint: Color,
}

impl RoughDielectricBSDF {
    fn frame(&self) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let n = self.ggx.normal;
        let (t, b) = crate::material::phase::orthonormal_basis(&n);
        (t, b, n)
    }

    fn local(&self, v: &Vector3<f64>) -> Vector3<f64> {
        let (t, b, n) = self.frame();
        Vector3::new(v.dot(&t), v.dot(&b), v.dot(&n))
    }

    fn world(&self, v: &Vector3<f64>) -> Vector3<f64> {
        let (t, b, n) = self.frame();
        t * v.x + b * v.y + n * v.z
    }

    /// The generalised half vector (facing +z) and η' for local `o` and `i`, or None if a
    /// microfacet can't connect them.
    fn half(&self, o: &Vector3<f64>, i: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> {
        if o.z == 0. || i.z == 0. {
            return None;
        }
        let reflect = o.z * i.z > 0.;
        let etap = if reflect { 1. } else if o.z > 0. { self.eta } else { 1. / self.eta };
        let m = i * etap + o;
        if m.norm_squared() == 0. {
            return None;
        }
        let mut m = m.normalize();
        if m.z < 0. {
            m = -m;
        }
        // Light can't reach the far side of a facet.
        if m.dot(i) * i.z < 0. || m.dot(o) * o.z < 0. {
            return None;
        }
        Some((m, etap))
    }

    /// The density of visible normal `m` seen from `o` (either side).
    fn visible(&self, o: &Vector3<f64>, m: &Vector3<f64>) -> f64 {
        let up = Vector3::new(o.x, o.y, o.z.abs());
        self.ggx.g1(&up) / up.z * self.ggx.d(m) * m.dot(o).abs()
    }
}

impl BSDF for RoughDielectricBSDF {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let (o, i) = (self.local(wo), self.local(wi));
        let Some((m, etap)) = self.half(&o, &i) else { return Color::black() };
        let f = fresnel_dielectric(o.dot(&m), self.eta);
        let g = 1. / (1. + self.ggx.lambda(&o) + self.ggx.lambda(&i));
        let d = self.ggx.d(&m);
        if o.z * i.z > 0. {
            // f |cos θ_i|.
            Color::white() * (d * g * f / (4. * o.z.abs()))
        } else {
            let denom = (i.dot(&m) + o.dot(&m) / etap).powi(2);
            let ft = d * (1. - f) * g * (i.dot(&m) * o.dot(&m) / (o.z * denom)).abs() / (etap * etap);
            self.tint * ft
        }
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let (o, i) = (self.local(wo), self.local(wi));
        let Some((m, etap)) = self.half(&o, &i) else { return 0. };
        let r = fresnel_dielectric(o.dot(&m), self.eta);
        if o.z * i.z > 0. {
            self.visible(&o, &m) / (4. * o.dot(&m).abs()) * r
        } else {
            let denom = (i.dot(&m) + o.dot(&m) / etap).powi(2);
            self.visible(&o, &m) * i.dot(&m).abs() / denom * (1. - r)
        }
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        let o = self.local(wo);
        if o.z == 0. {
            return None;
        }
        // A visible normal, sampled as from above, mirrored if the view is from below (D is
        // symmetric).
        let up = Vector3::new(o.x, o.y, o.z.abs());
        let m = self.ggx.sample_visible_normal(&up, [u[0], u[1]]);
        let r = fresnel_dielectric(o.dot(&m), self.eta);
        let i = if u[2] < r {
            m * (2. * o.dot(&m)) - o
        } else {
            // Refract o about m.
            let cos = o.dot(&m);
            let eta = if cos > 0. { self.eta } else { 1. / self.eta };
            let mf = if cos > 0. { m } else { -m };
            let wi = crate::lens::refract(&o, &mf, eta)?;
            if wi.z * o.z >= 0. { return None; }
            wi
        };
        if u[2] < r && i.z * o.z <= 0. {
            return None;
        }
        let wi = self.world(&i);
        let pdf = self.pdf(wo, &wi);
        if pdf <= 0. || !pdf.is_finite() {
            return None;
        }
        Some(BSDFSample { wi, weight: self.eval(wo, &wi) / pdf, pdf })
    }
}

/// Frosted glass (see the module's notes): `roughness` perceptual (α = r²); below `MIN_ALPHA`,
/// the smooth dielectric's behaviour.
pub struct RoughDielectric {
    pub refractive_index: RefractiveIndex,
    pub roughness: f64,
    pub attenuate: Color,
}

impl MaterialModel for RoughDielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let eta = self.refractive_index.current();
        let alpha = regularised(self.roughness * self.roughness, s);
        let offset = surface_offset(intersection, s);
        if alpha < MIN_ALPHA {
            return crate::material::functions::scatter_dielectric(eta, self.attenuate.as_reflectance(), r, intersection, offset);
        }
        let bsdf = RoughDielectricBSDF {
            ggx: GGX::new(alpha, Fresnel::Dielectric(eta), intersection.normal),
            eta,
            tint: self.attenuate.as_reflectance(),
        };
        let wo = -r.rd.normalize();
        // Lights sampled directly (reflected off the surface; through it, the glass shadows
        // them anyway).
        let emit = direct_lighting_mis(&bsdf, &intersection.point, &wo, s);
        crate::material::ggx::scattered();
        match bsdf.sample(&wo, [crate::geometry::rand(), crate::geometry::rand(), crate::geometry::rand()]) {
            Some(sample) => {
                // Through the surface: start on the far side.
                let n = intersection.normal;
                let side = if sample.wi.dot(&n) * wo.dot(&n) > 0. { 0. } else { -wo.dot(&n).signum() };
                ScatteredRay {
                    emit,
                    attenuate: sample.weight,
                    ray: Some(Ray { ro: intersection.point + n * (side * offset), rd: sample.wi }),
                    light_sampling: LightSampling::MIS { bsdf_pdf: sample.pdf },
                }
            },
            None => ScatteredRay { emit, attenuate: Color::black(), ray: None, light_sampling: LightSampling::MIS { bsdf_pdf: 0. } },
        }
    }
}

/// A pane of glass of no thickness (see the module's notes).
pub struct ThinDielectric {
    pub refractive_index: RefractiveIndex,
}

impl ThinDielectric {
    /// What the pane reflects at cosine `cos`, counting the reflections inside it.
    pub fn reflectance(cos: f64, eta: f64) -> f64 {
        let r = fresnel_dielectric(cos.abs(), eta);
        if r >= 1. {
            return 1.;
        }
        let t = 1. - r;
        r + t * t * r / (1. - r * r)
    }
}

impl MaterialModel for ThinDielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let eta = self.refractive_index.current();
        let n = intersection.facing_normal(r);
        let cos = (r.rd.dot(&n) / r.rd.norm()).abs();
        let reflect_prob = ThinDielectric::reflectance(cos, eta);
        let ray = if crate::geometry::rand() < reflect_prob {
            Ray { ro: intersection.point, rd: reflect(r.rd, n) }
        } else {
            // Straight through, from just beyond the pane.
            Ray { ro: intersection.point - n * surface_offset(intersection, s), rd: r.rd }
        };
        ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(), ray: Some(ray) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    #[test]
    fn test_fresnel_dielectric() {
        assert!((fresnel_dielectric(1., 1.5) - 0.04).abs() < 1e-12);
        // From inside, beyond the critical angle: total internal reflection.
        assert_eq!(fresnel_dielectric(-0.5, 1.5), 1.);
        // Symmetric in the light's direction at normal incidence.
        assert!((fresnel_dielectric(-1., 1.5) - 0.04).abs() < 1e-12);
    }

    #[test]
    fn test_thin_pane_reflects_more_than_one_surface() {
        let one = fresnel_dielectric(1., 1.5);
        let pane = ThinDielectric::reflectance(1., 1.5);
        // Two surfaces' worth, less what's lost between them: 2R / (1 + R).
        assert!((pane - 2. * one / (1. + one)).abs() < 1e-12, "{} {}", pane, one);
    }

    /// Sampled weights agree with eval / pdf, densities integrate to at most one, and the light
    /// reflected and transmitted is all of it when nearly smooth, and most of it when rough: the
    /// single-scattering model loses what a facet sends back through the surface (most from
    /// inside, where total internal reflection is common), which multiple-scattering models
    /// would recover.
    #[test]
    fn test_rough_dielectric_is_consistent() {
        for (alpha, wo, least) in [
            (0.05, Vector3::new(0.3, 0.1, 0.9), 0.98), (0.05, Vector3::new(0.2, 0.4, -0.8), 0.97),
            (0.3, Vector3::new(0.3, 0.1, 0.9), 0.95), (0.3, Vector3::new(0.2, 0.4, -0.8), 0.78), (0.6, Vector3::new(0.7, 0., 0.3), 0.8),
        ] {
            let wo = wo.normalize();
            let bsdf = RoughDielectricBSDF { ggx: GGX::new(alpha, Fresnel::Dielectric(1.5), Vector3::z()), eta: 1.5, tint: Color::white() };
            let mut rng = Pcg32::new(3, 5);
            let n = 200_000;
            let mut weight_sum = 0.;
            let mut found = 0;
            for _ in 0..n {
                if let Some(s) = bsdf.sample(&wo, [rng.next_f64(), rng.next_f64(), rng.next_f64()]) {
                    let pdf = bsdf.pdf(&wo, &s.wi);
                    assert!((pdf - s.pdf).abs() <= 1e-6 * pdf.max(1.), "pdf {} vs {}", s.pdf, pdf);
                    let w = bsdf.eval(&wo, &s.wi) / pdf;
                    assert!((w.rgb - s.weight.rgb).norm() < 1e-6 * w.rgb.norm().max(1.));
                    // Radiance across the boundary scales by 1/η'², so count throughput as
                    // energy: transmitted weights times η'².
                    let etap = if s.wi.z * wo.z > 0. { 1. } else if wo.z > 0. { 1.5 } else { 1. / 1.5 };
                    weight_sum += s.weight.rgb.x * if s.wi.z * wo.z > 0. { 1. } else { etap * etap };
                    found += 1;
                }
            }
            let energy = weight_sum / n as f64;
            assert!(found as f64 > 0.8 * n as f64, "only {} of {} sampled", found, n);
            // Never more than comes in.
            assert!(energy > least && energy < 1.02, "alpha {} wo {}: energy {}", alpha, wo, energy);
            // The pdf integrates to at most one over the sphere.
            let m = 200_000;
            let integral: f64 = (0..m).map(|_| {
                let wi = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
                bsdf.pdf(&wo, &wi) * 4. * std::f64::consts::PI
            }).sum::<f64>() / m as f64;
            assert!(integral < 1.03, "pdf integrates to {}", integral);
        }
    }
}
