use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;
use geometry::{rand};
use material::functions::scatter_dielectric;

pub struct Dielectric {
    pub refractive_index: f64,
    pub attenuate: Color,
}

impl MaterialModel for Dielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return scatter_dielectric(self.refractive_index, self.attenuate, r, intersection);
    }
}
