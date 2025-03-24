use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;

pub struct DiffuseLight {
    pub color: Color,
    pub intensity: f64,
}

impl MaterialModel for DiffuseLight {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        ScatteredRay{ 
            attenuate:self.color * self.intensity,
            ray: None }
    }
}
