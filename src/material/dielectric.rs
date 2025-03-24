use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::geometry::{rand};
use crate::material::functions::scatter_dielectric;

pub struct Dielectric {
    pub refractive_index: f64,
    pub attenuate: Color,
}

impl MaterialModel for Dielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        scatter_dielectric(self.refractive_index, self.attenuate, r, intersection)
    }
}
