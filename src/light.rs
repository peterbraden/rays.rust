use crate::na::Vector3;
use crate::color::Color;

pub struct Light {
    pub position: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}

/// Light from a source so distant that it arrives along a single direction: the sun. It is
/// small and bright enough that surfaces and media sample it directly (next-event estimation),
/// with a shadow ray, as if it were a point at infinity; rays that didn't (a camera's, or a
/// mirror's) see its disc instead (`disc_radiance`).
#[derive(Debug, Clone, PartialEq)]
pub struct Sun {
    /// Unit vector pointing towards the sun.
    pub direction: Vector3<f64>,
    /// Irradiance on a surface facing the sun (per colour channel), before any atmosphere.
    pub irradiance: Color,
    /// Angular radius of the disc, radians.
    pub angular_radius: f64,
    /// With a skysphere: the transmittance of its atmosphere towards the sun from each
    /// altitude, every `ALTITUDE_STEP` from the ground up, and the planet altitude is measured
    /// above (rather than scene y).
    pub atmosphere: Option<Vec<Color>>,
    pub planet: Option<crate::planet::Planet>,
}

/// The sun's mean angular radius, 0.2666°.
pub const SUN_ANGULAR_RADIUS: f64 = 0.004_653;
/// Altitude between entries of `Sun::atmosphere`, and how many there are (to 40 km, above
/// which the atmosphere barely attenuates).
pub const ALTITUDE_STEP: f64 = 100.;
pub const ALTITUDE_STEPS: usize = 401;
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
        Sun { direction: direction.normalize(), irradiance, angular_radius: SUN_ANGULAR_RADIUS, atmosphere: None, planet: None }
    }

    /// Irradiance at `point`: through the atmosphere from its altitude (scene y), if any.
    pub fn irradiance_at(&self, point: &Vector3<f64>) -> Color {
        match &self.atmosphere {
            None => self.irradiance,
            Some(t) => {
                let altitude = self.planet.map_or(point.y, |p| p.altitude(point));
                let x = (altitude / ALTITUDE_STEP).clamp(0., (t.len() - 1) as f64);
                let i = (x as usize).min(t.len() - 2);
                let f = x - i as f64;
                self.irradiance * (t[i] * (1. - f) + t[i + 1] * f)
            },
        }
    }

    /// Radiance seen from `from` looking along unit `d`: the limb-darkened disc,
    /// I(μ) ∝ 1 - u (1 - μ) for μ the cosine of the angle from the disc's centre on the
    /// sun's surface, scaled so the whole disc gives `irradiance_at`.
    pub fn disc_radiance(&self, from: &Vector3<f64>, d: &Vector3<f64>) -> Color {
        let cos = d.dot(&self.direction);
        let cos_r = self.angular_radius.cos();
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
        let shadow_ray = crate::ray::Ray { ro: *point, rd: self.direction };
        s.objects.transmittance(&shadow_ray, SUN_SHADOW_DISTANCE, f64::MIN_POSITIVE)
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
}
