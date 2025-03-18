use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::na::Vector3;
use crate::light::Light;
use crate::material::functions::{reflect, diffuse, phong};

pub struct Whitted {
    pub pigment: Color,
    pub reflection: f64,
    pub phong: f64,
}

impl MaterialModel for Whitted {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        let mut out = Color::black();
        for light in &s.lights {
            let light_vec = light.position - intersection.point;
            let shadow_ray = Ray {ro: intersection.point, rd: light_vec.normalize()};
            let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), 0.001); 

            match shadow_intersection {
                Some(_) => (),// Point in shadow...
                None => out = diffuse(self.pigment, &intersection, &light_vec, &light) + phong(self.phong, &r, &intersection, &light_vec),
            }
        }

        if self.reflection > 0. {
            let refl = Ray {
                ro: intersection.point,
                rd: reflect(r.rd, intersection.normal)
            };
            return ScatteredRay{ attenuate: out * self.reflection, ray: Some(refl) };
        }
        return ScatteredRay{ attenuate: out, ray: None };
    }
}

pub struct FlatColor {
    pub pigment: Color,
}

impl MaterialModel for FlatColor {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return ScatteredRay{ attenuate: self.pigment, ray: None };
    }
}
