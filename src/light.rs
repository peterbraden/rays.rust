use crate::na::Vector3;
use crate::color::Color;

pub struct Light {
    pub position: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}

/// Light from a source so distant that it arrives along a single direction with the same
/// irradiance everywhere: the sun. Like a point light it can't be hit by a scattered ray, so
/// surfaces and media sample it directly (next-event estimation), with a shadow ray.
#[derive(Debug, Clone, PartialEq)]
pub struct Sun {
    /// Unit vector pointing towards the sun.
    pub direction: Vector3<f64>,
    /// Irradiance on a surface facing the sun (per colour channel).
    pub irradiance: Color,
}

/// How far shadow rays towards the sun look for occluders. Short of `f64::MAX`, the distance at
/// which `shapes::infinite::Infinite` (the skysphere) reports every ray hitting it, so the sky
/// doesn't shade its own sun.
pub const SUN_SHADOW_DISTANCE: f64 = 1e30;

impl Sun {
    /// Whether nothing blocks the sun from `point`. Through a participating medium this is
    /// random, and true with probability equal to the transmittance.
    pub fn visible_from(&self, point: &Vector3<f64>, s: &crate::scene::Scene) -> bool {
        let shadow_ray = crate::ray::Ray { ro: *point, rd: self.direction };
        !s.objects.occluded(&shadow_ray, SUN_SHADOW_DISTANCE, f64::MIN_POSITIVE)
    }
}
