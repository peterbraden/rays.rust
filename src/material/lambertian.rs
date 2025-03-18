use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::scatter_lambertian;

pub struct Lambertian {
    pub albedo: Color,
}
impl MaterialModel for Lambertian {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return scatter_lambertian(self.albedo, intersection);
    }
}

