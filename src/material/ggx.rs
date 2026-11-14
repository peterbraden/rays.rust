//! # Rough reflection: the GGX microfacet model
//!
//! A rough surface is modelled as a field of tiny mirrors, microfacets, whose normals m are
//! spread about the surface's normal n by a distribution D(m). Light from ω_i reaches the
//! viewer at ω_o only off facets whose normal is the half vector h = (ω_i + ω_o) / |ω_i + ω_o|,
//! and only if neither direction is blocked by other facets, the masking-shadowing G. With a
//! Fresnel reflectance F per facet, the BRDF is
//!
//! ```text
//! f(ω_o, ω_i) = F(ω_o·h) G₂(ω_o, ω_i) D(h) / (4 |ω_i·n| |ω_o·n|)
//! ```
//!
//! Bruce Walter, Stephen R. Marschner, Hongsong Li and Kenneth E. Torrance, "Microfacet Models
//! for Refraction through Rough Surfaces", Eurographics Symposium on Rendering 2007, pp.
//! 195-206, doi:10.2312/EGWR/EGSR07/195-206
//! (<https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf>), equation 20. The factor
//! 1/4 comes from the Jacobian of the half vector, |∂ω_h/∂ω_i| = 1 / (4 |ω_i·h|) (their
//! equation 14), which converts densities over facet normals to densities over directions.
//!
//! ## The distribution
//!
//! Walter et al.'s GGX (their equation 33), the same distribution Trowbridge and Reitz derived
//! for ellipsoidal facets ("Average irregularity representation of a rough surface for ray
//! reflection", JOSA 65(5), 1975, doi:10.1364/JOSA.65.000531), with roughness α:
//!
//! ```text
//! D(m) = α² / (π cos⁴θ_m (α² + tan²θ_m)²) = α² / (π ((α² - 1) cos²θ_m + 1)²)
//! ```
//!
//! Its long tail keeps a soft glow around highlights, which measured materials show and the
//! older Beckmann distribution lacks.
//!
//! ## Masking and shadowing
//!
//! Smith's model takes facet heights as uncorrelated with slopes, giving the masking of one
//! direction as G₁(ω) = 1 / (1 + Λ(ω)), with, for GGX,
//!
//! ```text
//! Λ(ω) = (-1 + sqrt(1 + α² tan²θ)) / 2
//! ```
//!
//! Eric Heitz, "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs", JCGT
//! 3(2), 2014 (<https://jcgt.org/published/0003/02/03/>), section 5.3, equation 72. For both
//! directions together, Heitz recommends the height-correlated form, which accounts for a facet
//! that is high enough to be visible from one direction being likely visible from the other
//! (section 6, equation 99):
//!
//! ```text
//! G₂(ω_o, ω_i) = 1 / (1 + Λ(ω_o) + Λ(ω_i))
//! ```
//!
//! ## Sampling visible normals
//!
//! Sampling facet normals from D alone wastes samples on facets facing away from the viewer,
//! and gives weights that blow up at grazing angles. Instead, sample only the normals the viewer
//! can see, in proportion to their visible area, the distribution of visible normals
//!
//! ```text
//! D_ω_o(m) = G₁(ω_o) max(0, ω_o·m) D(m) / (ω_o·n)
//! ```
//!
//! then reflect ω_o about m. Eric Heitz, "Sampling the GGX Distribution of Visible Normals",
//! JCGT 7(4), 2018 (<https://jcgt.org/published/0007/04/01/>): equation 3, and the routine of
//! listing 1, which stretches the view direction to the configuration α = 1 (a hemisphere of
//! normals), samples the projected hemisphere there as a disc warped towards the visible half,
//! and unstretches. The density of ω_i is D_ω_o(m) / (4 ω_o·m) (Heitz's appendix B, equation
//! 17), which simplifies to
//!
//! ```text
//! p(ω_i) = G₁(ω_o) D(m) / (4 ω_o·n)
//! ```
//!
//! and the sample's weight f |cos θ_i| / p is F G₂ / G₁(ω_o) (his equation 19): close to F,
//! never large. That bounded weight is what removes the fireflies of the old fuzzed mirror.
//!
//! ## Departures and open questions
//!
//! - Materials' `roughness` r is "perceptual": α = r², which spreads the useful range of the
//!   slider (as Burley's Disney BRDF does). It also matches the fuzzed mirror this replaced,
//!   whose roughness was a fuzz radius: with α = r², scenes written for it look as they did,
//!   and are as noisy; with α = r, their reflections were blurrier and 40% noisier per pixel.
//! - Light scattered between facets more than once is lost, so rough surfaces come out darker
//!   than they should. Seen from the normal, a white surface reflects 99.7% at α = 0.05, 97%
//!   at 0.15, 69% at 0.5, and 1 - ln 2 = 31% at α = 1 (`test_white_furnace_...`). The fuzzed
//!   mirror this replaces lost only the rays fuzzed into the surface, so rough metals are now
//!   darker than they were. Multiple-scattering microfacet models correct this, and are a
//!   candidate for later.
//! - Below a roughness of `MIN_ALPHA` the lobe is narrower than double precision handles well,
//!   and the surface is treated as a perfect mirror.
//!
//! ## Path regularisation
//!
//! Sunlight reflected by a polished surface onto a diffuse one is a caustic. A path tracer
//! finds it from the diffuse end: a bounce off the diffuse surface lands on the polished one,
//! which samples the sun (with MIS). The sharper the polished lobe, the more rarely the bounce
//! lands where it reflects the sun, and the brighter it is when it does: sparkle that takes
//! many samples to settle, and from a mirror, light no sample finds at all.
//!
//! Anton S. Kaplanyan and Carsten Dachsbacher, "Path Space Regularization for Holistic and
//! Robust Light Transport", Computer Graphics Forum 32(2), 2013, pp. 63-72,
//! doi:10.1111/cgf.12026
//! (<https://cg.ivd.kit.edu/publications/p2013/PSR_Kaplanyan_2013/PSR_Kaplanyan_2013.pdf>),
//! regularise only the interactions that unbiased sampling can't handle, blurring them a
//! little, and derive conditions under which shrinking the blur as samples accumulate keeps
//! the estimate consistent. Johannes Jendersie and Thorsten Grosch, "Microfacet Model
//! Regularization for Robust Light Transport", Computer Graphics Forum 2019,
//! doi:10.1111/cgf.13768, do it for microfacet models by raising the roughness before
//! evaluating them.
//!
//! The same holds when the first surface is itself glossy: a brushed sphere showing a polished
//! neighbour's highlight of the sun finds it only when its own sampled reflection lands on
//! that small, bright spot.
//!
//! Here: a path's first scattering vertex is exact, and every GGX vertex after it takes
//! α ≥ α₀, the scene's `"regularise"` (default `DEFAULT_REGULARISE`; 0 turns it off).
//! Surfaces seen directly, and their own reflections of the lights, are untouched. What is
//! blurred is light reaching a polished surface after another scattering: caustics come out
//! softer, and reflections seen in reflections frosted to at least α₀, a bias traded for most
//! of their noise. On glossy-sun, the mean relMSE at 64 samples falls from 5.7 unregularised
//! to 0.0035 flooring only vertices after a diffuse one, and to 0.0014 flooring every vertex
//! after the first. The floor is fixed, not shrunk with the sample count, so this is biased,
//! not consistent.
use crate::color::Color;
use crate::na::Vector3;
use crate::material::bsdf::{BSDF, BSDFSample};
use crate::material::phase::orthonormal_basis;
use std::f64::consts::PI;

/// Below this roughness, a surface is a perfect mirror.
pub const MIN_ALPHA: f64 = 1e-3;

/// The default roughness floor for glossy vertices after a diffuse one (`regularised`).
pub const DEFAULT_REGULARISE: f64 = 0.2;

thread_local! {
    /// Whether the current path has scattered yet (`scattered`), since `begin_path`.
    static PATH_SCATTERED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Start a path: no vertices yet.
pub fn begin_path() {
    PATH_SCATTERED.with(|r| r.set(false));
}

/// The path has scattered (off a surface, or in a medium).
pub fn scattered() {
    PATH_SCATTERED.with(|r| r.set(true));
}

/// α for a GGX vertex of roughness `alpha` on the current path, regularised: unchanged at the
/// path's first scattering, and at least the scene's floor after it (see "Path
/// regularisation").
pub fn regularised(alpha: f64, s: &crate::scene::Scene) -> f64 {
    if PATH_SCATTERED.with(|r| r.get()) { alpha.max(s.render.regularise) } else { alpha }
}

/// How much a facet reflects, by the cosine of the angle between the light and its normal.
#[derive(Clone, Copy, Debug)]
pub enum Fresnel {
    /// Schlick's approximation for a conductor, with the reflectance at normal incidence per
    /// channel: F = F₀ + (1 - F₀) (1 - cos θ)⁵. Christophe Schlick, "An Inexpensive BRDF Model
    /// for Physically-based Rendering", Computer Graphics Forum 13(3), 1994,
    /// doi:10.1111/1467-8659.1330233, section 4.2, equation 15.
    Schlick(Color),
    /// A dielectric of this refractive index, seen from outside (`functions::fresnel_reflectance`).
    Dielectric(f64),
}

impl Fresnel {
    pub fn eval(&self, cos: f64) -> Color {
        match self {
            Fresnel::Schlick(f0) => { let k = (1. - cos).max(0.).powi(5); *f0 * (1. - k) + Color::white() * k },
            Fresnel::Dielectric(eta) => {
                let f = crate::material::functions::fresnel_reflectance(cos, 1. / eta, *eta);
                Color::new(f, f, f)
            },
        }
    }
}

/// GGX reflection about `normal`, roughness `alpha`.
pub struct GGX {
    pub alpha: f64,
    pub fresnel: Fresnel,
    /// Unit normal, on the side the light arrives from.
    pub normal: Vector3<f64>,
}

/// A direction in the frame (t, b, n) of a unit normal: z along n.
struct Frame {
    t: Vector3<f64>,
    b: Vector3<f64>,
    n: Vector3<f64>,
}

impl Frame {
    fn new(n: &Vector3<f64>) -> Frame {
        let (t, b) = orthonormal_basis(n);
        Frame { t, b, n: *n }
    }

    fn to_local(&self, v: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(v.dot(&self.t), v.dot(&self.b), v.dot(&self.n))
    }

    fn to_world(&self, v: &Vector3<f64>) -> Vector3<f64> {
        self.t * v.x + self.b * v.y + self.n * v.z
    }
}

impl GGX {
    /// D(m) for a unit facet normal in the local frame.
    pub fn d(&self, m: &Vector3<f64>) -> f64 {
        if m.z <= 0. {
            return 0.;
        }
        let a2 = self.alpha * self.alpha;
        let k = (a2 - 1.) * m.z * m.z + 1.;
        a2 / (PI * k * k)
    }

    /// Smith's Λ for a unit direction in the local frame.
    pub fn lambda(&self, w: &Vector3<f64>) -> f64 {
        let cos2 = w.z * w.z;
        if cos2 >= 1. {
            return 0.;
        }
        let tan2 = (1. - cos2) / cos2;
        ((1. + self.alpha * self.alpha * tan2).sqrt() - 1.) / 2.
    }

    pub fn g1(&self, w: &Vector3<f64>) -> f64 {
        1. / (1. + self.lambda(w))
    }

    /// Heitz's routine (listing 1) for a visible normal, in the local frame, from uniform `u`.
    fn sample_visible_normal(&self, wo: &Vector3<f64>, u: [f64; 2]) -> Vector3<f64> {
        // Stretch the view direction to the hemisphere configuration (α = 1).
        let vh = Vector3::new(self.alpha * wo.x, self.alpha * wo.y, wo.z).normalize();
        // An orthonormal basis about it (with vh as its z).
        let lensq = vh.x * vh.x + vh.y * vh.y;
        let t1 = if lensq > 0. { Vector3::new(-vh.y, vh.x, 0.) / lensq.sqrt() } else { Vector3::new(1., 0., 0.) };
        let t2 = vh.cross(&t1);
        // A point on the disc, its lower half squashed towards the half visible from vh.
        let r = u[0].sqrt();
        let phi = 2. * PI * u[1];
        let p1 = r * phi.cos();
        let s = 0.5 * (1. + vh.z);
        let p2 = (1. - s) * (1. - p1 * p1).max(0.).sqrt() + s * r * phi.sin();
        // Project it onto the hemisphere, then unstretch.
        let nh = t1 * p1 + t2 * p2 + vh * (1. - p1 * p1 - p2 * p2).max(0.).sqrt();
        Vector3::new(self.alpha * nh.x, self.alpha * nh.y, nh.z.max(0.)).normalize()
    }
}

impl BSDF for GGX {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let frame = Frame::new(&self.normal);
        let (o, i) = (frame.to_local(wo), frame.to_local(wi));
        if o.z <= 0. || i.z <= 0. {
            return Color::black();
        }
        let h = (o + i).normalize();
        let g2 = 1. / (1. + self.lambda(&o) + self.lambda(&i));
        // f cos θ_i: the cosine cancels the BRDF's |ω_i·n|.
        self.fresnel.eval(o.dot(&h)) * (g2 * self.d(&h) / (4. * o.z))
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let frame = Frame::new(&self.normal);
        let (o, i) = (frame.to_local(wo), frame.to_local(wi));
        if o.z <= 0. || i.z <= 0. {
            return 0.;
        }
        let h = (o + i).normalize();
        self.g1(&o) * self.d(&h) / (4. * o.z)
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        let frame = Frame::new(&self.normal);
        let o = frame.to_local(wo);
        if o.z <= 0. {
            return None;
        }
        let m = self.sample_visible_normal(&o, [u[0], u[1]]);
        let i = m * (2. * o.dot(&m)) - o;
        if i.z <= 0. {
            // Reflected below the surface by a facet on a rough slope: shadowed.
            return None;
        }
        let wi = frame.to_world(&i);
        let pdf = self.pdf(wo, &wi);
        if pdf <= 0. {
            return None;
        }
        Some(BSDFSample { wi, weight: self.eval(wo, &wi) / pdf, pdf })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::bsdf::tests::check_bsdf;
    use crate::geometry::{Pcg32, point_on_unit_sphere};

    fn ggx(alpha: f64) -> GGX {
        GGX { alpha, fresnel: Fresnel::Schlick(Color::white()), normal: Vector3::new(0., 0.6, 0.8) }
    }

    #[test]
    fn test_distribution_is_normalised() {
        // The projected area of the facets is that of the surface: ∫ D(m) cos θ_m dm = 1.
        let mut rng = Pcg32::new(2, 1);
        for alpha in [0.05, 0.3, 1.] {
            let g = ggx(alpha);
            let n = 400_000;
            // Cosine-weighted directions about z, density cos θ / π: the estimate is D π.
            let mean = (0..n).map(|_| {
                let p = Vector3::new(0., 0., 1.) + point_on_unit_sphere(rng.next_f64(), rng.next_f64());
                g.d(&p.normalize()) * PI
            }).sum::<f64>() / n as f64;
            // Narrow lobes are hard to hit by chance, so the tolerance widens.
            assert!((mean - 1.).abs() < if alpha < 0.1 { 0.1 } else { 0.01 }, "alpha {}: {}", alpha, mean);
        }
    }

    #[test]
    fn test_visible_normals_project_to_the_view() {
        // Heitz 2014 equation 15: ∫ G₁(ω_o) max(0, ω_o·m) D(m) dm = ω_o·n, so D_ω_o integrates
        // to one. Uniformly over the sphere, density 1 / 4π.
        let mut rng = Pcg32::new(4, 1);
        let g = ggx(0.4);
        let o = Vector3::new(0.6, 0., 0.8);
        let n = 400_000;
        let mean = (0..n).map(|_| {
            let m = point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            g.g1(&o) * o.dot(&m).max(0.) * g.d(&m) * 4. * PI
        }).sum::<f64>() / n as f64;
        assert!((mean - o.z).abs() < 0.01, "{} vs {}", mean, o.z);
    }

    #[test]
    fn test_ggx_sampling_matches_eval_and_pdf() {
        // From near normal and from grazing; a rough lobe, and one narrow enough that uniform
        // sampling finds it only roughly.
        for (alpha, tolerance) in [(0.5, 0.01), (0.15, 0.03)] {
            let g = ggx(alpha);
            for wo in [Vector3::new(0., 0.6, 0.8), Vector3::new(0.95, 0.3, 0.).normalize()] {
                check_bsdf(&g, &wo, tolerance);
            }
        }
    }

    #[test]
    fn test_white_furnace_loses_only_to_multiple_scattering() {
        // With F = 1, the directional albedo is the fraction of light that leaves after one
        // bounce: near one when smooth, much lower when rough (light scattered between facets
        // is lost). At α = 1, seen from the normal, D = 1/π, Λ(ω_o) = 0 and Λ(ω_i) = (1/μ - 1)/2
        // for μ = cos θ_i, so the albedo ∫ G₂ D / (4 cos θ_o) dω_i is
        // (1/4π) ∫ 2π / (1 + (1/μ - 1)/2) dμ = ∫₀¹ μ / (μ + 1) dμ = 1 - ln 2: 69% is lost.
        let mut rng = Pcg32::new(5, 1);
        let wo = Vector3::new(0., 0.6, 0.8);
        let albedo = |alpha: f64, rng: &mut Pcg32| {
            let g = ggx(alpha);
            let n = 100_000;
            (0..n).map(|_| g.sample(&wo, [rng.next_f64(), rng.next_f64(), rng.next_f64()])
                .map_or(0., |s| s.weight.rgb.x)).sum::<f64>() / n as f64
        };
        let smooth = albedo(0.05, &mut rng);
        let rough = albedo(1., &mut rng);
        assert!(smooth > 0.99 && smooth <= 1.0001, "{}", smooth);
        assert!((rough - (1. - 2f64.ln())).abs() < 0.005, "{} vs {}", rough, 1. - 2f64.ln());
    }

    #[test]
    fn test_reciprocity() {
        let g = GGX { alpha: 0.3, fresnel: Fresnel::Dielectric(1.5), normal: Vector3::new(0., 0., 1.) };
        let a = Vector3::new(0.3, 0.1, 0.9).normalize();
        let b = Vector3::new(-0.5, 0.2, 0.7).normalize();
        // f(a, b) = f(b, a); eval includes the cosine of its second argument.
        let fab = g.eval(&a, &b).rgb.x / b.z;
        let fba = g.eval(&b, &a).rgb.x / a.z;
        assert!((fab - fba).abs() < 1e-12 * fab, "{} vs {}", fab, fba);
    }

    #[test]
    fn test_path_roughness_floor() {
        let s = crate::scenefile::SceneFile::from_string(r#"{
            "regularise": 0.25,
            "camera": { "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }
        }"#.to_string());
        begin_path();
        // The first vertex: unchanged.
        assert_eq!(regularised(0.01, &s), 0.01);
        // After it: the scene's floor, but never smoother than the surface's own.
        scattered();
        assert_eq!(regularised(0.01, &s), 0.25);
        assert_eq!(regularised(0.6, &s), 0.6);
        // A new path starts smooth.
        begin_path();
        assert_eq!(regularised(0.01, &s), 0.01);
    }

    #[test]
    fn test_regularisation_tames_caustics() {
        // A diffuse floor beside a polished sphere, with the sun placed so that the sphere's
        // leftmost point, (0.5, 1, 0), reflects it onto the origin: a caustic, which the floor
        // there finds only when a bounce lands near that point. α = 0.09 keeps the exact
        // estimate's spikes common enough to measure; regularised to 0.2, they spread and
        // shrink. Both runs draw the same random numbers, so with regularisation doing nothing
        // they would agree exactly.
        let scene = |regularise: f64| crate::scenefile::SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 2, "shadow_bias": 0.0001, "regularise": {},
            "sun": {{ "direction": [-1, 2, 0], "intensity": 1 }},
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{
                "F": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }},
                "M": {{ "type": "metal", "reflective": [0.9, 0.9, 0.9], "roughness": 0.3 }}
            }},
            "objects": [
                {{ "type": "plane", "y": 0, "material": "F" }},
                {{ "type": "sphere", "location": [1.5, 1, 0], "radius": 1, "material": "M" }}
            ]
        }}"#, regularise));
        let r = crate::ray::Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        let stats = |s: &crate::scene::Scene| {
            crate::geometry::seed_rand(31, 1);
            let n = 100_000;
            let v: Vec<f64> = (0..n).map(|_| crate::trace::trace(&r, 0, s).1.rgb.x).collect();
            let mean = v.iter().sum::<f64>() / n as f64;
            (mean, (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64).sqrt())
        };
        let (exact, exact_sd) = stats(&scene(0.));
        let (regularised, regularised_sd) = stats(&scene(0.2));
        assert!(regularised_sd < 0.7 * exact_sd, "sd {} vs {}", regularised_sd, exact_sd);
        // Blurring the caustic moves little light: the mean changes by a few percent at most.
        assert!((regularised - exact).abs() < 0.1 * exact, "{} vs {}", regularised, exact);
    }
}
