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
    fn scatter(&self, _r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Direct light from point lights is emitted; the bounce gathers indirect light.
        ScatteredRay { emit: direct_lighting(self.albedo, intersection, s), ..scatter_lambertian(self.albedo, intersection) }
    }
}

