use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::geometry::{random_point_on_unit_sphere};
use crate::material::functions::{reflect};

pub struct Specular {
    pub albedo: Color,
    pub roughness: f64,
}

impl MaterialModel for Specular {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let fuzz = random_point_on_unit_sphere() * self.roughness;

        let refl = Ray {
            ro: intersection.point,
            rd: reflect(r.rd, intersection.normal) + fuzz
        };

        ScatteredRay{ attenuate:self.albedo, ray: Some(refl) }
    }
}


