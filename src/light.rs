use crate::na::Vector3;
use crate::color::Color;

pub struct Light {
    pub position: Vector3<f64>,
    pub emission: crate::spectrum::Emission,
    pub intensity: f64,
}

impl Light {
    /// Radiant intensity, per steradian, at the current sample's wavelengths (or in RGB).
    pub fn radiant_intensity(&self) -> Color {
        self.emission.radiance() * self.intensity
    }
}

/// Light from a source so distant that it arrives along a single direction: the sun. It is
/// small and bright enough that surfaces and media sample it directly (next-event estimation),
/// with a shadow ray, as if it were a point at infinity; rays that didn't (a camera's, or a
/// mirror's) see its disc instead (`disc_radiance`).
#[derive(Clone)]
pub struct Sun {
    /// Unit vector pointing towards the sun.
    pub direction: Vector3<f64>,
    /// Irradiance on a surface facing the sun (per colour channel), before any atmosphere.
    pub irradiance: Color,
    /// Angular radius of the disc, radians (set with `with_angular_radius`).
    pub angular_radius: f64,
    /// Its cosine, which sampling needs often.
    pub(crate) cos_radius: f64,
    /// With a skysphere: its atmosphere, which the sunlight crosses on its way to each point.
    pub atmosphere: Option<std::sync::Arc<crate::skysphere::SkyMaterial>>,
}

impl std::fmt::Debug for Sun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sun {{ direction: {:?}, irradiance: {:?}, angular_radius: {}, atmosphere: {} }}",
            self.direction, self.irradiance, self.angular_radius, self.atmosphere.is_some())
    }
}

/// The sun's mean angular radius, 0.2666°.
pub const SUN_ANGULAR_RADIUS: f64 = 0.004_653;
/// The sun's linear limb-darkening coefficient in the visible: the disc's edge is about 40%
/// as bright as its centre (D. F. Gray, "The Observation and Analysis of Stellar
/// Photospheres", Cambridge 2005, chapter 17).
const LIMB_DARKENING: f64 = 0.6;

/// How far shadow rays towards the sun look for occluders. Short of `f64::MAX`, the distance at
/// which `shapes::infinite::Infinite` (the skysphere) reports every ray hitting it, so the sky
/// doesn't shade its own sun.
pub const SUN_SHADOW_DISTANCE: f64 = 1e30;

impl Sun {
    /// A sun with no atmosphere and the mean angular radius.
    pub fn new(direction: Vector3<f64>, irradiance: Color) -> Sun {
        Sun { direction: direction.normalize(), irradiance, angular_radius: SUN_ANGULAR_RADIUS,
              cos_radius: SUN_ANGULAR_RADIUS.cos(), atmosphere: None }
    }

    /// The same sun with a disc of angular radius `r`, radians.
    pub fn with_angular_radius(self, r: f64) -> Sun {
        Sun { angular_radius: r, cos_radius: r.cos(), ..self }
    }

    /// Irradiance at `point`: through the atmosphere, if any, by its altitude and the sun's
    /// height in its sky, so anywhere on the planet; none on the night side.
    pub fn irradiance_at(&self, point: &Vector3<f64>) -> Color {
        // In spectral mode, at the sample's wavelengths: the irradiance as an illuminant, and the
        // air's transmittance, computed in RGB, as a reflectance.
        match &self.atmosphere {
            None => self.irradiance.as_illuminant(),
            Some(sky) => {
                let t = sky.sun_transmittance(point);
                self.irradiance.as_illuminant() * Color::new(t.x, t.y, t.z).as_reflectance()
            },
        }
    }

    /// Radiance seen from `from` looking along unit `d`: the limb-darkened disc,
    /// I(μ) ∝ 1 - u (1 - μ) for μ the cosine of the angle from the disc's centre on the
    /// sun's surface, scaled so the whole disc gives `irradiance_at`.
    pub fn disc_radiance(&self, from: &Vector3<f64>, d: &Vector3<f64>) -> Color {
        let cos = d.dot(&self.direction);
        let cos_r = self.cos_radius;
        if cos <= cos_r {
            return Color::black();
        }
        let theta = cos.min(1.).acos();
        let mu = (1. - (theta / self.angular_radius).powi(2)).max(0.).sqrt();
        let solid_angle = 2. * std::f64::consts::PI * (1. - cos_r);
        self.irradiance_at(from) * ((1. - LIMB_DARKENING * (1. - mu)) / (1. - LIMB_DARKENING / 3.) / solid_angle)
    }

    /// The fraction of sunlight reaching `point`: 0 behind anything solid, and partly through
    /// participating media (an estimate; see `SceneGraph::transmittance`).
    pub fn transmittance_from(&self, point: &Vector3<f64>, s: &crate::scene::Scene) -> f64 {
        self.transmittance_along(point, &self.direction, s)
    }

    /// As `transmittance_from`, towards the point of the disc in unit direction `d`.
    pub fn transmittance_along(&self, point: &Vector3<f64>, d: &Vector3<f64>, s: &crate::scene::Scene) -> f64 {
        let shadow_ray = crate::ray::Ray { ro: *point, rd: *d };
        s.objects.transmittance(&shadow_ray, SUN_SHADOW_DISTANCE, f64::MIN_POSITIVE)
    }

    /// The disc's solid angle, 2π (1 - cos r) for angular radius r.
    pub fn solid_angle(&self) -> f64 {
        2. * std::f64::consts::PI * (1. - self.cos_radius)
    }

    /// A direction to a point on the disc, from uniform `u`, distributed nearly in proportion to
    /// the disc's limb-darkened radiance, so that radiance / density is nearly the same
    /// everywhere on it: sampling the sun this way adds almost no noise to the light it
    /// delivers. (Uniformly over the disc, the estimate of a diffuse surface's light would swing
    /// between 0.5 and 1.25 times its mean with the limb darkening alone.)
    ///
    /// Let ρ = θ / r be the angle from the centre as a fraction of the disc's radius, and
    /// s = ρ². The radiance is proportional to g(s) = (1 - u (1 - √(1 - s))) / (1 - u/3), for
    /// limb darkening u, and the disc's area element to ρ dρ, i.e. ds / 2, so s has density
    /// g(s) on [0, 1], whose cumulative distribution is
    ///
    /// ```text
    /// G(s) = ((1 - u) s + (2u/3) (1 - (1 - s)^(3/2))) / (1 - u/3)
    /// ```
    ///
    /// Inverting G(s) = u₀ by Newton's method gives s, and so θ; the azimuth is 2π u₁. The exact density over solid angle is in `disc_pdf`. This is
    /// the inversion method for a 1D distribution, as in Pharr, Jakob and Humphreys, "Physically
    /// Based Rendering", 4th ed., section A.5.4, "Sampling Within a Cone", applied to the
    /// limb-darkened profile rather than a uniform one
    /// (<https://pbr-book.org/4ed/Sampling_Algorithms/Sampling_Multidimensional_Functions>).
    pub fn sample_disc(&self, u: [f64; 2]) -> Vector3<f64> {
        self.sample_disc_at(&Vector3::zeros(), u).0
    }

    /// As `sample_disc`, with the radiance arriving at `from` from that direction
    /// (`disc_radiance`) and the direction's density (`disc_pdf`), found from the sampled angle
    /// rather than recomputed from the direction.
    pub fn sample_disc_at(&self, from: &Vector3<f64>, u: [f64; 2]) -> (Vector3<f64>, Color, f64) {
        // In terms of μ = √(1 - s), 1 - G is a cubic, (b μ² + a μ³) / (1 - k/3) with
        // b = 1 - k and a = 2k/3: smooth up to the limb, where G's derivative in s isn't, so
        // Newton's method converges in two or three steps from μ for a uniform disc.
        let k = LIMB_DARKENING;
        let (a, b) = (2. * k / 3., 1. - k);
        let target = (1. - u[0]) * (1. - k / 3.);
        let mut mu = (1. - u[0]).sqrt();
        for _ in 0..8 {
            let f = (b + a * mu) * mu * mu - target;
            let df = (2. * b + 3. * a * mu) * mu;
            if df <= 0. {
                break;
            }
            let step = f / df;
            mu = (mu - step).clamp(0., 1.);
            if step.abs() < 1e-10 {
                break;
            }
        }
        let s = 1. - mu * mu;
        let r = self.angular_radius;
        let rho = s.sqrt();
        let theta = rho * r;
        let phi = 2. * std::f64::consts::PI * u[1];
        let (a, b) = crate::material::phase::orthonormal_basis(&self.direction);
        let (sin, cos) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let d = (self.direction * cos + (a * cos_phi + b * sin_phi) * sin).normalize();
        let g = Sun::profile(s);
        let radiance = self.irradiance_at(from) * (g / self.solid_angle());
        (d, radiance, Sun::pdf_at(g, rho, r, theta, sin))
    }

    /// The density of `sample_disc` at angle θ = r ρ from the centre, of sine `sin`, where the
    /// profile is g.
    fn pdf_at(g: f64, rho: f64, r: f64, theta: f64, sin: f64) -> f64 {
        let sinc = if theta < 1e-8 { r * r } else { r * sin / rho };
        g / (std::f64::consts::PI * sinc)
    }

    /// g(s): the radiance at s = (θ / r)² relative to the disc's mean, by the linear limb
    /// darkening law (`disc_radiance`), with μ = √(1 - s).
    fn profile(s: f64) -> f64 {
        (1. - LIMB_DARKENING * (1. - (1. - s).max(0.).sqrt())) / (1. - LIMB_DARKENING / 3.)
    }

    /// The density per steradian with which `sample_disc` picks unit direction `d`. With s
    /// distributed as g(s) ds and the azimuth uniform, ρ = √s has density 2ρ g(ρ²), θ = r ρ has
    /// density 2ρ g / r, and a solid angle element is sin θ dθ dφ, so
    ///
    /// ```text
    /// p(ω) = 2ρ g(ρ²) / (r · 2π sin θ) = g(ρ²) ρ / (π r sin θ)
    /// ```
    ///
    /// which tends to g(0) / (π r²) at the centre.
    pub fn disc_pdf(&self, d: &Vector3<f64>) -> f64 {
        let cos = d.dot(&self.direction);
        if cos <= self.cos_radius {
            return 0.;
        }
        let r = self.angular_radius;
        let theta = cos.min(1.).acos();
        let rho = theta / r;
        Sun::pdf_at(Sun::profile(rho * rho), rho, r, theta, theta.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disc_integrates_to_the_irradiance() {
        let sun = Sun::new(Vector3::new(0.3, 0.8, -0.2), Color::new(1., 2., 3.));
        let r = sun.angular_radius;
        // Over the cone around the sun, in rings by angle from its centre.
        let n = 20_000;
        let total: f64 = (0..n).map(|i| {
            let theta = (i as f64 + 0.5) / n as f64 * r * 1.01;
            let side = sun.direction.cross(&Vector3::new(0., 0., 1.)).normalize();
            let d = (sun.direction * theta.cos() + side * theta.sin()).normalize();
            sun.disc_radiance(&Vector3::zeros(), &d).rgb.y * 2. * std::f64::consts::PI * theta.sin() * (r * 1.01 / n as f64)
        }).sum();
        assert!((total - 2.).abs() < 2e-3, "{}", total);
    }

    #[test]
    fn test_disc_samples_lie_on_the_disc_and_integrate_to_the_irradiance() {
        let sun = Sun::new(Vector3::new(0.3, 0.8, -0.2), Color::new(1., 2., 3.));
        let mut rng = crate::geometry::Pcg32::new(3, 1);
        let n = 20_000;
        let mut total = 0.;
        for _ in 0..n {
            let (d, radiance, sampled_pdf) = sun.sample_disc_at(&Vector3::zeros(), [rng.next_f64(), rng.next_f64()]);
            assert!((sampled_pdf / sun.disc_pdf(&d) - 1.).abs() < 1e-6, "{} vs {}", sampled_pdf, sun.disc_pdf(&d));
            let looked_up = sun.disc_radiance(&Vector3::zeros(), &d);
            assert!((radiance.rgb.y / looked_up.rgb.y - 1.).abs() < 1e-6, "{:?} vs {:?}", radiance, looked_up);
            assert!((d.norm() - 1.).abs() < 1e-12);
            let pdf = sun.disc_pdf(&d);
            assert!(pdf > 0., "sample off the disc");
            let estimate = sun.disc_radiance(&Vector3::zeros(), &d).rgb.y / pdf;
            // Sampled in proportion to the radiance: every sample gives the irradiance.
            assert!((estimate - 2.).abs() < 1e-4, "{}", estimate);
            total += estimate;
        }
        assert!((total / n as f64 - 2.).abs() < 1e-4, "{}", total / n as f64);
    }

    #[test]
    fn test_disc_pdf_integrates_to_one() {
        // In rings by angle from the centre, as test_disc_integrates_to_the_irradiance.
        let sun = Sun::new(Vector3::new(0., 1., 0.), Color::white());
        let r = sun.angular_radius;
        let n = 20_000;
        let total: f64 = (0..n).map(|i| {
            let theta = (i as f64 + 0.5) / n as f64 * r * 1.01;
            let d = Vector3::new(theta.sin(), theta.cos(), 0.);
            sun.disc_pdf(&d) * 2. * std::f64::consts::PI * theta.sin() * (r * 1.01 / n as f64)
        }).sum();
        assert!((total - 1.).abs() < 1e-3, "{}", total);
    }
}
