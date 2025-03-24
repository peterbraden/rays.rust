use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::na::{Vector3};

// Shade color based on normal

pub struct NormalShade {
}

impl MaterialModel for NormalShade {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let angle = intersection.normal.dot(&Vector3::new(0., 1., 0.));
        let c = Color::white() * intersection.normal.abs() * angle * angle * angle; 
        ScatteredRay{ attenuate: c, ray: None }
    }
}



