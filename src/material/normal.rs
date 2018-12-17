use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;

// Shade color based on normal

pub struct NormalShade {
}

impl MaterialModel for NormalShade {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let c = Color::white() * intersection.normal * 0.5; 
        return ScatteredRay{ attenuate: c, ray: None };
    }
}



