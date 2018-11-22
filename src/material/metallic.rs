use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;
use geometry::{random_point_on_unit_sphere, reflect};

pub struct Metallic {
    pub reflective: Color,
    pub roughness: f64,
}

impl MaterialModel for Metallic {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let fuzz = random_point_on_unit_sphere() * self.roughness;

        let refl = Ray {
            ro: intersection.point,
            rd: reflect(r.rd, intersection.normal) + fuzz
        };

        return ScatteredRay{ attenuate:self.reflective, ray: Some(refl) };
    }
}


