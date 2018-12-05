use color::Color;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use material::functions::scatter_lambertian;

pub struct Lambertian {
    pub albedo: Color,
}
impl MaterialModel for Lambertian {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return scatter_lambertian(self.albedo, intersection);
    }
}

