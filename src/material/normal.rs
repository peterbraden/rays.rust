use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;
use na::{Vector3};

// Shade color based on normal

pub struct NormalShade {
}

impl MaterialModel for NormalShade {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        //let angle = (intersection.normal.dot(&Vector3::new(0., 1., 0.)));
        let c = Color::white() * intersection.normaL; // * angle; 
        return ScatteredRay{ attenuate: c, ray: None };
    }
}



