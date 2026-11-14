//! # Scattering functions, for combining light sampling with BSDF sampling
//!
//! A surface reflects radiance arriving from direction ω_i towards ω_o in proportion to its
//! bidirectional scattering distribution function f(ω_o, ω_i), the BSDF. The light leaving
//! towards the viewer is the integral over incoming directions
//!
//! ```text
//! L_o(ω_o) = ∫ f(ω_o, ω_i) L_i(ω_i) |cos θ_i| dω_i
//! ```
//!
//! which a path tracer estimates from one direction at a time. It can pick that direction in
//! two ways, each good where the other is poor:
//!
//! - Light sampling: pick a point on a light, with density p_L(ω_i) over directions. Good for
//!   small, bright lights, which BSDF sampling would rarely hit; poor for a shiny surface,
//!   whose BSDF is non-zero only in a narrow cone that a point on a large light rarely falls in.
//! - BSDF sampling: pick ω_i with density p_f(ω_i) roughly proportional to f |cos θ_i|. Good for
//!   shiny surfaces and large lights; poor for small lights (fireflies: a rare hit on the sun
//!   carries 15,000 times the sun's irradiance).
//!
//! Multiple importance sampling (MIS) takes one sample of each and weights each by how likely
//! that technique was to produce it compared with the others, so each technique counts most
//! where it is best: Eric Veach and Leonidas J. Guibas, "Optimally Combining Sampling Techniques
//! for Monte Carlo Rendering", SIGGRAPH 1995, pp. 419-428, doi:10.1145/218380.218498
//! (<https://graphics.stanford.edu/papers/combine/>). With one sample from each technique, the
//! combined estimate of the integral of g is
//!
//! ```text
//! F = w_L(x) g(x) / p_L(x) + w_f(y) g(y) / p_f(y)       x from p_L, y from p_f
//! ```
//!
//! and it is unbiased provided the weights sum to one wherever g is non-zero, and a technique's
//! weight is zero wherever it can't sample (Eric Veach, "Robust Monte Carlo Methods for Light
//! Transport Simulation", PhD thesis, Stanford 1997, section 9.2.1, conditions W1 and W2,
//! <https://graphics.stanford.edu/papers/veach_thesis/>). Veach's power heuristic with exponent
//! β = 2 (Veach and Guibas, equation 14; thesis equation 9.13), which they found works well,
//!
//! ```text
//! w_L = p_L² / (p_L² + p_f²)
//! ```
//!
//! is `power_heuristic`. Both densities must be over the same measure: here, solid angle at the
//! shading point. A light sampled by area converts with p_ω = p_A d² / |cos θ_y|.
//!
//! The path tracer (`trace.rs`) does the BSDF half: a material's scattered ray carries the
//! density it was sampled with (`LightSampling::MIS`), and emission it hits is weighted by w_f.
//! The light half is `functions::direct_lighting_mis`, which needs the BSDF's value and density
//! for any direction: the `BSDF` trait here. See also Pharr, Jakob and Humphreys, "Physically
//! Based Rendering", 4th ed., section 2.2.3 (MIS) and section 13.4 (a path tracer with MIS),
//! <https://pbr-book.org/4ed/Light_Transport_I_Surface_Reflection/A_Better_Path_Tracer>.
//!
//! Directions here are unit vectors pointing away from the surface: ω_o towards the viewer
//! (the reverse of the arriving ray), ω_i towards where light comes from.
use crate::color::Color;
use crate::na::Vector3;
use crate::geometry::point_on_unit_sphere;
use std::f64::consts::PI;

/// A BSDF at one shading point, in world space.
pub trait BSDF {
    /// f(ω_o, ω_i) |cos θ_i|: the radiance leaving towards ω_o per unit radiance arriving from
    /// ω_i, per steradian.
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color;

    /// The density, per steradian, with which `sample` picks ω_i.
    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64;

    /// Pick ω_i from uniform random numbers `u`: the direction, `eval / pdf` (the factor to
    /// scale the radiance arriving from it by), and `pdf`. None if no light scatters.
    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BSDFSample {
    pub wi: Vector3<f64>,
    pub weight: Color,
    pub pdf: f64,
}

/// Veach's power heuristic with β = 2: the weight of a sample from the technique with density
/// `p`, against one other technique with density `q` for the same direction.
pub fn power_heuristic(p: f64, q: f64) -> f64 {
    if p.is_infinite() {
        return 1.;
    }
    let (p2, q2) = (p * p, q * q);
    if p2 + q2 == 0. { 0. } else { p2 / (p2 + q2) }
}

/// Ideal diffuse reflection: f = albedo / π on the side the normal faces, nothing through it.
pub struct LambertianBSDF {
    pub albedo: Color,
    /// Unit normal, on the side the light arrives from.
    pub normal: Vector3<f64>,
}

impl BSDF for LambertianBSDF {
    fn eval(&self, _wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let cos = wi.dot(&self.normal);
        if cos <= 0. { Color::black() } else { self.albedo * (cos / PI) }
    }

    fn pdf(&self, _wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        wi.dot(&self.normal).max(0.) / PI
    }

    /// The normal plus a point uniformly distributed on the unit sphere, normalised, is
    /// distributed in proportion to the cosine from the normal: the point is uniform on the
    /// sphere tangent to the surface at the normal's tip, which projects to a cosine
    /// distribution (Malley's method in another form). eval / pdf is then the albedo.
    fn sample(&self, _wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        let d = self.normal + point_on_unit_sphere(u[0], u[1]);
        // The sample can (rarely) cancel the normal exactly.
        let wi = if d.norm() < 1e-9 { self.normal } else { d.normalize() };
        let pdf = self.pdf(&Vector3::zeros(), &wi);
        if pdf <= 0. {
            return None;
        }
        Some(BSDFSample { wi, weight: self.albedo, pdf })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    /// Check a BSDF's sampling against its eval and pdf, from `wo`: the mean of sampled
    /// weights estimates the directional albedo ∫ f cos dω_i, as does the mean of eval / p over
    /// uniformly sampled directions; each sample's weight must equal eval / pdf; and the pdf
    /// must integrate to (at most) one.
    pub fn check_bsdf(b: &dyn BSDF, wo: &Vector3<f64>, tolerance: f64) {
        let mut rng = Pcg32::new(7, 3);
        let n = 400_000;
        let mut sampled = Color::black();
        for _ in 0..n {
            let u = [rng.next_f64(), rng.next_f64(), rng.next_f64()];
            if let Some(s) = b.sample(wo, u) {
                let eval = b.eval(wo, &s.wi);
                let pdf = b.pdf(wo, &s.wi);
                assert!((pdf - s.pdf).abs() <= 1e-6 * pdf.max(1.), "pdf {} vs {}", s.pdf, pdf);
                for c in 0..3 {
                    assert!((eval.rgb[c] / pdf - s.weight.rgb[c]).abs() < 1e-6 * s.weight.rgb[c].max(1.),
                        "weight {:?} vs eval / pdf {:?}", s.weight, eval / pdf);
                }
                sampled = sampled + s.weight;
            }
        }
        let sampled = sampled / n as f64;
        // Uniformly over the sphere, density 1 / 4π.
        let mut uniform = Color::black();
        let mut pdf_integral = 0.;
        for _ in 0..n {
            let wi = point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            uniform = uniform + b.eval(wo, &wi) * (4. * PI);
            pdf_integral += b.pdf(wo, &wi) * 4. * PI;
        }
        let uniform = uniform / n as f64;
        let pdf_integral = pdf_integral / n as f64;
        for c in 0..3 {
            assert!((sampled.rgb[c] - uniform.rgb[c]).abs() < tolerance, "sampled {:?} vs uniform {:?}", sampled, uniform);
        }
        assert!(pdf_integral < 1. + tolerance, "pdf integrates to {}", pdf_integral);
    }

    #[test]
    fn test_power_heuristic() {
        assert_eq!(power_heuristic(1., 1.), 0.5);
        assert_eq!(power_heuristic(3., 0.), 1.);
        assert_eq!(power_heuristic(0., 2.), 0.);
        assert_eq!(power_heuristic(0., 0.), 0.);
        assert_eq!(power_heuristic(f64::INFINITY, 2.), 1.);
        // Weights for the same point from two techniques sum to one.
        assert!((power_heuristic(0.3, 7.) + power_heuristic(7., 0.3) - 1.).abs() < 1e-15);
    }

    #[test]
    fn test_lambertian_bsdf() {
        let b = LambertianBSDF { albedo: Color::new(0.2, 0.5, 0.9), normal: Vector3::new(0., 0.6, 0.8) };
        check_bsdf(&b, &Vector3::new(0., 1., 0.), 0.01);
        // Directional albedo is the albedo.
        let mut rng = Pcg32::new(1, 1);
        let n = 100_000;
        let mean = (0..n).fold(Color::black(), |a, _| {
            let wi = point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            a + b.eval(&Vector3::zeros(), &wi) * (4. * PI)
        }) / n as f64;
        assert!((mean.rgb.z - 0.9).abs() < 0.01, "{:?}", mean);
    }
}
