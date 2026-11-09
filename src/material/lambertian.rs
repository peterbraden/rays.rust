use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, direct_lighting};

pub struct Lambertian {
    pub albedo: Color,
}
impl MaterialModel for Lambertian {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        // Direct light from point lights is emitted; the bounce gathers indirect light.
        ScatteredRay {
            emit: direct_lighting(self.albedo, intersection, s),
            sampled_lights: true,
            ..scatter_lambertian(self.albedo, intersection)
        }
    }
}

