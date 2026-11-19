//! # Subsurface scattering by random walk
//!
//! Skin, wax, marble and milk are lit from within: light enters, scatters many times, and
//! leaves somewhere else, softening shadows and glowing through thin parts. pbrt models this
//! with a tabulated BSSRDF and probe rays (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 3rd ed., chapter 15; 4th ed., which leaves the details to its online material).
//! Production renderers have since moved to brute force: follow the light through the object
//! as through a participating medium, bounded by the object's own surface (Wrenninge, Villemin
//! and Hery, "Path Traced Subsurface Scattering using Anisotropic Phase Functions and
//! Non-Exponential Free Flights", Pixar technical memo 17-07, 2017; Chiang, Kutz and Burley,
//! "Practical and Controllable Subsurface Scattering for Production Path Tracing", SIGGRAPH
//! 2016 Talks; Cycles' "random walk" method). That is what this material does:
//!
//! 1. At the surface, a smooth dielectric boundary reflects with Fresnel's probability (a
//!    mirror-like sheen); otherwise light goes in, in a cosine-distributed direction (diffuse
//!    transmission), and leaves wherever it reaches the surface again: the boundary is
//!    otherwise index-matched, as Chiang et al.'s fit below assumes. (Refracting in and
//!    reflecting back in at the exit by Fresnel traps light and made an albedo of 0.3 look
//!    0.16.)
//! 2. Inside, a random walk through a medium of extinction σ_t and single-scattering albedo α
//!    per channel, with a Henyey-Greenstein phase function, until it reaches the surface (the
//!    object's geometry alone is tested). The channel that samples each free flight is chosen
//!    in proportion to the path's throughput in it, and the step weighted by the balance
//!    heuristic over the channels (as Cycles does), so a medium that is red-translucent and
//!    blue-opaque stays noise-free in both.
//! 3. Where it leaves, the point is shaded as an ideal diffuse surface facing out: next-event
//!    estimation towards the lights from there (so the sun reaches the inside, which a medium
//!    behind a dielectric can't see), and a cosine-distributed ray to carry on.
//!
//! Artists can't pick σ_a and σ_s, so it takes the colour the surface should have, `albedo` A
//! (its multiple-scattering albedo), and how far light travels in it, `radius` d (scene
//! units, per channel: skin carries red further than blue). Chiang, Kutz and Burley's fits
//! (their equations 2 and 4, from fitting random walks in a half-space) give
//!
//! ```text
//! α = 1 - exp(-5.09406 A + 2.61188 A² - 4.31805 A³)
//! σ_t = 1 / (d s),   s = 1.9 - A + 3.5 (A - 0.8)²
//! ```
//!
//! for isotropic scattering; with asymmetry g, the scattering coefficient is raised to keep
//! the reduced one, σ_s' = σ_s (1 - g) (the similarity relation), so the look barely changes
//! while the phase function shapes the near-surface glow. `sigma_a` and `sigma_s` can be given
//! directly instead.

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::Intersection;
use crate::material::bsdf::LambertianBSDF;
use crate::material::functions::{direct_lighting_mis, reflect, surface_offset};
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::material::phase::sample_henyey_greenstein;
use crate::material::rough_dielectric::fresnel_dielectric;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;

/// Steps after which the walk gives up (the light is taken as absorbed).
const MAX_STEPS: u32 = 1024;

/// How the medium inside is given.
pub enum Coefficients {
    /// Chiang et al.'s parameters: the surface's colour and how far light travels.
    Albedo { albedo: Color, radius: Color },
    /// Absorption and scattering coefficients (per scene unit).
    Sigma { sigma_a: Color, sigma_s: Color },
    /// Glacier ice, in metres: ice's own absorption at each wavelength
    /// (`snow::ice_absorption`; in RGB mode at 630, 532 and 465 nm) with its impurities', and
    /// its bubbles' scattering coefficient `scattering` (per metre, the same at all
    /// wavelengths: bubbles are far larger than light's).
    Ice { scattering: f64, impurities: crate::material::snow::Impurities },
}

pub struct Subsurface {
    pub coefficients: Coefficients,
    pub g: f64,
    pub refractive_index: f64,
}

/// Chiang, Kutz and Burley's single-scattering albedo and extinction for surface albedo `a`
/// and distance `d`, isotropic.
pub fn chiang(a: f64, d: f64) -> (f64, f64) {
    let a = a.clamp(0., 0.999);
    let alpha = 1. - (-5.09406 * a + 2.61188 * a * a - 4.31805 * a * a * a).exp();
    let s = 1.9 - a + 3.5 * (a - 0.8).powi(2);
    (alpha, 1. / (d * s).max(1e-12))
}

impl Subsurface {
    /// σ_t and α per channel, at the sample's wavelengths.
    pub fn medium(&self) -> (Vector3<f64>, Vector3<f64>) {
        match &self.coefficients {
            Coefficients::Albedo { albedo, radius } => {
                let (a, d) = (albedo.as_reflectance().rgb, radius.as_unbounded().rgb);
                let mut sigma_t = Vector3::zeros();
                let mut alpha = Vector3::zeros();
                for c in 0..3 {
                    let (al, st) = chiang(a[c], d[c]);
                    // Keep σ_s' = σ_s (1 - g) and σ_a as fitted.
                    let (sa, ss) = (st * (1. - al), st * al / (1. - self.g));
                    sigma_t[c] = sa + ss;
                    alpha[c] = if sigma_t[c] > 0. { ss / sigma_t[c] } else { 0. };
                }
                (sigma_t, alpha)
            },
            Coefficients::Ice { scattering, impurities } => {
                let lambda = crate::spectrum::current().map_or(crate::material::conductor::RGB_WAVELENGTHS, |w| w.lambda);
                let a = Vector3::from(lambda.map(|l| crate::material::snow::ice_absorption(l) + impurities.absorption(l)));
                let t = a.add_scalar(*scattering);
                (t, t.map(|t| scattering / t))
            },
            Coefficients::Sigma { sigma_a, sigma_s } => {
                let (a, s) = (sigma_a.as_unbounded().rgb, sigma_s.as_unbounded().rgb);
                let t = a + s;
                (t, s.zip_map(&t, |s, t| if t > 0. { s / t } else { 0. }))
            },
        }
    }

    /// Walk from `p` along `w` inside the object, until the light leaves (the exit point,
    /// its outward normal, and the throughput) or is lost.
    fn walk(&self, i: &Intersection, mut p: Vector3<f64>, mut w: Vector3<f64>) -> Option<(Vector3<f64>, Vector3<f64>, Color)> {
        let (sigma_t, alpha) = self.medium();
        let geometry = &i.object.geometry;
        let mut throughput = Vector3::new(1., 1., 1.);
        for step in 0..MAX_STEPS {
            let total = throughput.sum();
            if total <= 0. {
                return None;
            }
            // Russian roulette once the light has faded.
            if step > 16 && throughput.max() < 0.1 {
                let q = throughput.max() / 0.1;
                if rand() >= q {
                    return None;
                }
                throughput /= q;
            }
            // A channel for this flight, in proportion to the throughput.
            let u = rand() * total;
            let c = if u < throughput.x { 0 } else if u < throughput.x + throughput.y { 1 } else { 2 };
            let t = if sigma_t[c] > 0. { -(1. - rand()).ln() / sigma_t[c] } else { f64::INFINITY };
            let probability = throughput / total;
            let hit = geometry.intersects(&Ray { ro: p, rd: w }).filter(|h| h.dist > 0.);
            match hit {
                Some(h) if h.dist <= t => {
                    // Reached the surface: the chance of flying at least this far, per channel,
                    // over its balance-heuristic mixture.
                    let tr = sigma_t.map(|st| (-st * h.dist).exp());
                    let pdf = probability.dot(&tr);
                    if pdf <= 0. {
                        return None;
                    }
                    throughput = throughput.component_mul(&tr) / pdf;
                    let n_out = if h.normal.dot(&w) > 0. { h.normal } else { -h.normal };
                    // Index-matched from inside: the light leaves (as Chiang et al.'s fit
                    // assumes; the boundary's reflection is the sheen at entry).
                    return Some((h.point, n_out, Color { rgb: throughput }));
                },
                _ => {
                    if !t.is_finite() {
                        // Nothing to scatter off and no surface ahead: an open shape.
                        return None;
                    }
                    // A collision: scattered with probability α.
                    let tr = sigma_t.map(|st| (-st * t).exp());
                    let pdf = probability.dot(&sigma_t.component_mul(&tr));
                    if pdf <= 0. {
                        return None;
                    }
                    throughput = throughput.component_mul(&sigma_t.component_mul(&alpha).component_mul(&tr)) / pdf;
                    p += w * t;
                    w = sample_henyey_greenstein(&w, self.g, rand(), rand());
                },
            }
        }
        None
    }
}

impl MaterialModel for Subsurface {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = intersection.facing(r);
        let d = r.rd.normalize();
        let n = i.normal;
        let cos = -d.dot(&n);
        let offset = surface_offset(&i, s);
        // The boundary: a mirror reflection with Fresnel's probability.
        if rand() < fresnel_dielectric(cos, self.refractive_index) {
            return ScatteredRay {
                emit: Color::black(), attenuate: Color::white(),
                ray: Some(Ray { ro: i.point, rd: reflect(d, n) }),
                light_sampling: LightSampling::None,
            };
        }
        let none = ScatteredRay { emit: Color::black(), attenuate: Color::black(), ray: None, light_sampling: LightSampling::None };
        // In, in a cosine-distributed direction (diffuse transmission, as Chiang et al. and
        // Cycles), from just inside: the trace moved the hit towards the ray's side.
        let inward = LambertianBSDF { albedo: Color::white(), normal: -n };
        let Some(entry) = crate::material::bsdf::BSDF::sample(&inward, &-n, [rand(), rand(), rand()]) else { return none };
        let p = i.point - n * (2. * offset);
        let Some((exit, n_out, throughput)) = self.walk(&i, p, entry.wi) else { return none };
        // Leave as from a diffuse surface facing out.
        let exit = exit + n_out * offset;
        let lambert = LambertianBSDF { albedo: Color::white(), normal: n_out };
        let emit = direct_lighting_mis(&lambert, &exit, &n_out, s) * throughput;
        crate::material::ggx::scattered();
        match crate::material::bsdf::BSDF::sample(&lambert, &n_out, [rand(), rand(), rand()]) {
            Some(sample) => ScatteredRay {
                emit,
                attenuate: throughput * sample.weight,
                ray: Some(Ray { ro: exit, rd: sample.wi }),
                light_sampling: LightSampling::MIS { bsdf_pdf: sample.pdf },
            },
            None => ScatteredRay { emit, ..none },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chiang_fit_is_monotonic_and_bounded() {
        let mut last = 0.;
        for k in 1..100 {
            let a = k as f64 / 100.;
            let (alpha, sigma_t) = chiang(a, 1.);
            assert!(alpha > last && alpha < 1., "{}: {}", a, alpha);
            assert!(sigma_t > 0.);
            last = alpha;
        }
        // Light-coloured materials scatter nearly everything.
        assert!(chiang(0.9, 1.).0 > 0.99);
    }

    #[test]
    fn test_similarity_keeps_reduced_scattering() {
        let iso = Subsurface { coefficients: Coefficients::Albedo { albedo: Color::white() * 0.7, radius: Color::white() }, g: 0., refractive_index: 1.4 };
        let fwd = Subsurface { coefficients: Coefficients::Albedo { albedo: Color::white() * 0.7, radius: Color::white() }, g: 0.8, refractive_index: 1.4 };
        let ((ti, ai), (tf, af)) = (iso.medium(), fwd.medium());
        // σ_a the same, σ_s (1 - g) the same.
        assert!((ti.x * (1. - ai.x) - tf.x * (1. - af.x)).abs() < 1e-9);
        assert!((ti.x * ai.x - tf.x * af.x * 0.2).abs() < 1e-9);
    }
}
