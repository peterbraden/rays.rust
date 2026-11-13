use crate::material::model::LightSampling;
use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::geometry::{random_point_in_unit_ball};
use crate::material::functions::{reflect};

pub struct Specular {
    pub albedo: Color,
    pub roughness: f64,
}

impl MaterialModel for Specular {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        let fuzz = random_point_in_unit_ball() * self.roughness;

        let refl = Ray {
            ro: intersection.point,
            rd: reflect(r.rd, intersection.normal) + fuzz
        };

        if refl.rd.dot(&intersection.normal) <= 0. {
            // Fuzz has pushed the reflection into the surface: absorb it.
            return ScatteredRay{ light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::black(), ray: None };
        }

        ScatteredRay{ light_sampling: LightSampling::None, emit: Color::black(), attenuate:self.albedo, ray: Some(refl) }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use crate::scenefile::SceneFile;
    use crate::sceneobject::SceneObject;
    use crate::shapes::plane::Plane;
    use crate::material::texture::Solid;
    use crate::material::legacy::FlatColor;
    use std::sync::Arc;

    #[test]
    fn test_rough_reflection_never_enters_surface() {
        let s = SceneFile::from_string(r#"{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.0001, "background": [0, 0, 0], "max_depth": 1,
            "camera": { "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": {}, "media": {}, "lights": [], "objects": [], "variables": {}
        }"#.to_string());
        let n = Vector3::new(0., 1., 0.);
        let i = Intersection { face: 0, part: 0,
            dist: 1.,
            point: Vector3::new(0., 0., 0.),
            normal: n,
            object: &(SceneObject {
                geometry: Box::new(Plane { y: 0. }),
                medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
            }),
        };
        let m = Specular { albedo: Color::white(), roughness: 1. };
        // Grazing incidence, so fuzz frequently points below the surface.
        let r = Ray { ro: Vector3::new(-1., 0.1, 0.), rd: Vector3::new(1., -0.1, 0.).normalize() };
        for _ in 0..1000 {
            if let Some(out) = m.scatter(&r, &i, &s).ray {
                assert!(out.rd.dot(&n) > 0., "{}", out.rd);
            }
        }
    }
}
