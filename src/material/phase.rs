//! Phase functions: how a participating medium redistributes the light it scatters, by the
//! angle between the direction light was travelling and the direction it leaves in. They play
//! the part a BSDF plays at a surface, normalised over the whole sphere rather than a hemisphere.

use crate::na::Vector3;
use std::f64::consts::PI;

/// The Henyey-Greenstein phase function, from L. G. Henyey and J. L. Greenstein, "Diffuse
/// radiation in the galaxy", Astrophysical Journal 93, 1941:
///
/// ```text
/// p(cos θ) = (1 - g²) / (4π (1 + g² - 2g cos θ)^(3/2))
/// ```
///
/// `cos_theta` is the cosine of the angle between the direction of travel before and after
/// scattering, so g > 0 scatters forwards, g < 0 backwards and g = 0 equally in all directions.
/// The mean of cos θ over the distribution is g, which is what makes a single parameter enough
/// for a first approximation of cloud droplets (g ≈ 0.85; see Jendersie and d'Eon, "An
/// Approximate Mie Scattering Function for Fog and Cloud Rendering", SIGGRAPH 2023 Talks, for
/// a closer fit).
pub fn henyey_greenstein(cos_theta: f64, g: f64) -> f64 {
    let denom = 1. + g * g - 2. * g * cos_theta;
    (1. - g * g) / (4. * PI * denom * denom.sqrt())
}

/// Sample a direction from the Henyey-Greenstein distribution around `incoming`, the unit
/// direction of travel, from two uniform numbers in [0, 1). The density of the result is
/// exactly `henyey_greenstein`, so a medium scattering this way weights the new ray by its
/// albedo alone.
///
/// Inverting the CDF of cos θ gives, for g ≠ 0,
///
/// ```text
/// cos θ = (1 + g² - ((1 - g²) / (1 - g + 2g u))²) / 2g
/// ```
///
/// (Pharr, Jakob and Humphreys, "Physically Based Rendering", 3rd ed., section 15.2.3, writes
/// it for a direction pointing back along the incoming ray, so with the sign flipped.)
pub fn sample_henyey_greenstein(incoming: &Vector3<f64>, g: f64, u1: f64, u2: f64) -> Vector3<f64> {
    let cos_theta = if g.abs() < 1e-3 {
        1. - 2. * u1
    } else {
        let s = (1. - g * g) / (1. - g + 2. * g * u1);
        (1. + g * g - s * s) / (2. * g)
    }.clamp(-1., 1.);
    let sin_theta = (1. - cos_theta * cos_theta).max(0.).sqrt();
    let phi = 2. * PI * u2;
    let (t, b) = orthonormal_basis(incoming);
    (t * (sin_theta * phi.cos()) + b * (sin_theta * phi.sin()) + incoming * cos_theta).normalize()
}

/// Two unit vectors perpendicular to the unit vector `n` and to each other, after Duff et al.,
/// "Building an Orthonormal Basis, Revisited", Journal of Computer Graphics Techniques 6(1), 2017.
fn orthonormal_basis(n: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let sign = 1f64.copysign(n.z);
    let a = -1. / (sign + n.z);
    let b = n.x * n.y * a;
    (
        Vector3::new(1. + sign * n.x * n.x * a, sign * b, -sign * n.x),
        Vector3::new(b, sign + n.y * n.y * a, -n.y),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Pcg32};

    #[test]
    fn test_henyey_greenstein_integrates_to_one() {
        for g in [-0.7, 0., 0.3, 0.85] {
            // Over the sphere, the integral of p is 2π ∫ p(μ) dμ over μ in [-1, 1].
            let n = 200_000;
            let sum: f64 = (0..n).map(|i| {
                let mu = -1. + 2. * (i as f64 + 0.5) / n as f64;
                henyey_greenstein(mu, g)
            }).sum();
            let integral = 2. * PI * sum * 2. / n as f64;
            assert!((integral - 1.).abs() < 1e-3, "g {}: {}", g, integral);
        }
    }

    #[test]
    fn test_henyey_greenstein_isotropic_at_zero() {
        assert!((henyey_greenstein(0.3, 0.) - 1. / (4. * PI)).abs() < 1e-12);
    }

    #[test]
    fn test_sampled_mean_cosine_is_g() {
        let mut rng = Pcg32::new(3, 0);
        let incoming = Vector3::new(1., 2., -2.).normalize();
        for g in [-0.5, 0., 0.85] {
            let n = 100_000;
            let mean = (0..n).map(|_| {
                let d = sample_henyey_greenstein(&incoming, g, rng.next_f64(), rng.next_f64());
                assert!((d.norm() - 1.).abs() < 1e-9);
                d.dot(&incoming)
            }).sum::<f64>() / n as f64;
            assert!((mean - g).abs() < 0.01, "g {}: mean cosine {}", g, mean);
        }
    }

    #[test]
    fn test_sampled_cosines_follow_the_density() {
        // The fraction of samples in a band of cos θ matches the integral of p over it.
        let mut rng = Pcg32::new(5, 0);
        let incoming = Vector3::new(0., 0., -1.);
        let g = 0.6;
        let n = 200_000;
        let (lo, hi) = (0.2, 0.6);
        let within = (0..n).filter(|_| {
            let mu = sample_henyey_greenstein(&incoming, g, rng.next_f64(), rng.next_f64()).dot(&incoming);
            mu >= lo && mu < hi
        }).count() as f64 / n as f64;
        let steps = 10_000;
        let expected: f64 = (0..steps).map(|i| {
            henyey_greenstein(lo + (hi - lo) * (i as f64 + 0.5) / steps as f64, g)
        }).sum::<f64>() * (hi - lo) / steps as f64 * 2. * PI;
        assert!((within - expected).abs() < 0.005, "{} vs {}", within, expected);
    }

    #[test]
    fn test_basis_is_orthonormal() {
        for n in [Vector3::new(0., 0., 1.), Vector3::new(0., 0., -1.), Vector3::new(1., 2., 3.).normalize()] {
            let (t, b) = orthonormal_basis(&n);
            for (x, y) in [(t, b), (t, n), (b, n)] {
                assert!(x.dot(&y).abs() < 1e-12);
            }
            assert!((t.norm() - 1.).abs() < 1e-12 && (b.norm() - 1.).abs() < 1e-12);
        }
    }
}
