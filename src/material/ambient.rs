use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;

pub struct Ambient {
    pub pigment: Color,
}

impl MaterialModel for Ambient {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return ScatteredRay{ attenuate:self.pigment, ray: None };
    }
}



