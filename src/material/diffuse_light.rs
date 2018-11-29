use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;

pub struct DiffuseLight {
    pub color: Color,
    pub intensity: f64,
}

impl MaterialModel for DiffuseLight {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return ScatteredRay{ 
            attenuate:self.color * self.intensity,
            ray: None };
    }
}
