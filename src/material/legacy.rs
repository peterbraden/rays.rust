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
                None => out = out + diffuse(self.pigment, intersection, &light_vec, light) + phong(self.phong, r, intersection, &light_vec),
            }
        }

        if self.reflection > 0. {
            let refl = Ray {
                ro: intersection.point,
                rd: reflect(r.rd, intersection.normal)
            };
            return ScatteredRay{ attenuate: out * self.reflection, ray: Some(refl) };
        }
        ScatteredRay{ attenuate: out, ray: None }
    }
}

pub struct FlatColor {
    pub pigment: Color,
}

impl MaterialModel for FlatColor {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        ScatteredRay{ attenuate: self.pigment, ray: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use crate::sceneobject::SceneObject;
    use crate::shapes::plane::Plane;
    use crate::material::texture::Solid;
    use std::sync::Arc;

    #[test]
    fn test_whitted_sums_all_lights() {
        let s = SceneFile::from_string(r#"{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.0001, "background": [0, 0, 0], "max_depth": 1,
            "camera": { "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": {}, "media": {}, "objects": [], "variables": {},
            "lights": [
                { "location": [0, 10, 0], "intensity": 1 },
                { "location": [0, 10, 0], "intensity": 1 }
            ]
        }"#.to_string());
        let m = Whitted { pigment: Color::white(), reflection: 0., phong: 0. };
        let i = Intersection {
            dist: 1.,
            point: Vector3::new(0., 0., 0.),
            normal: Vector3::new(0., 1., 0.),
            object: Arc::new(SceneObject {
                geometry: Box::new(Plane { y: 0. }),
                medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
            }),
        };
        let r = Ray { ro: Vector3::new(0., 1., -1.), rd: Vector3::new(0., -1., 1.).normalize() };
        let out = m.scatter(&r, &i, &s).attenuate;
        assert!((out.rgb[0] - 2.).abs() < 1e-9, "{:?}", out);
    }
}
