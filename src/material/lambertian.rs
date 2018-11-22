use color::Color;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use geometry::{random_point_on_unit_sphere};
use material::model::{MaterialModel, ScatteredRay};

/// Implement Lambertian reflection (purely diffuse) for ambient incoming light (light at a random
/// incoming angle.)
/// Practically, we implement random reflection within a unit sphere on the normal.
/// This will be very noisy if we don't subsample a lot.
pub struct Lambertian {
    pub albedo: Color,
}
impl MaterialModel for Lambertian {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let refl = Ray {
            ro: intersection.point,
            rd: intersection.normal + random_point_on_unit_sphere(),
        };
        return ScatteredRay{ attenuate:self.albedo, ray: Some(refl) };
    }
}

