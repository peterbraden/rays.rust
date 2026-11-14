use crate::material::model::LightSampling;
use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::material::functions::{reflect, scatter_bsdf};
use crate::material::ggx::{GGX, Fresnel, MIN_ALPHA, regularised};

/// A metal: reflection only, tinted by `albedo`, its reflectance at normal incidence, rising
/// towards white at grazing angles (Schlick's approximation; `ggx::Fresnel`). A rough metal is a
/// GGX microfacet surface (`ggx`) of α = `roughness`²; below `ggx::MIN_ALPHA`, a mirror.
pub struct Specular {
    pub albedo: Color,
    pub roughness: f64,
}

impl MaterialModel for Specular {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        let fresnel = Fresnel::Schlick(self.albedo);
        let alpha = regularised(self.roughness * self.roughness, s);
        if alpha >= MIN_ALPHA {
            let bsdf = GGX { alpha, fresnel, normal: intersection.normal };
            return scatter_bsdf(&bsdf, r, &intersection.point, s);
        }
        // A mirror: only its reflected ray can find the lights.
        let cos = (r.rd.dot(&intersection.normal) / r.rd.norm()).abs();
        let refl = Ray { ro: intersection.point, rd: reflect(r.rd, intersection.normal) };
        ScatteredRay{ light_sampling: LightSampling::None, emit: Color::black(), attenuate: fresnel.eval(cos), ray: Some(refl) }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use crate::color::Color;
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

    /// Looking at a metal floor along the reflection of the sun, 45° up, with a black sky: the
    /// mean of `n` samples and the largest.
    fn sun_glint(roughness: f64, n: usize) -> (f64, f64, Scene) {
        let s = SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1, "shadow_bias": 0.0001,
            "sun": {{ "direction": [0, 1, 1], "intensity": 1 }},
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "metal", "reflective": [0.9, 0.9, 0.9], "roughness": {} }} }},
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, roughness));
        let r = Ray { ro: Vector3::new(0., 1., -1.), rd: Vector3::new(0., -1., 1.).normalize() };
        let v: Vec<f64> = (0..n).map(|_| crate::trace::trace(&r, 0, &s).1.rgb.x).collect();
        (v.iter().sum::<f64>() / n as f64, v.iter().cloned().fold(0., f64::max), s)
    }

    /// ∫ f(ω_o, ω) L(ω) cos θ dω over the sun's disc, in rings, for the floor of `sun_glint`.
    fn glint_expected(alpha: f64, s: &Scene) -> f64 {
        let sun = s.sun.as_ref().unwrap();
        let bsdf = GGX { alpha, fresnel: Fresnel::Schlick(Color::new(0.9, 0.9, 0.9)), normal: Vector3::new(0., 1., 0.) };
        let wo = Vector3::new(0., 1., -1.).normalize();
        let (a, b) = crate::material::phase::orthonormal_basis(&sun.direction);
        let (rings, around) = (200, 200);
        let r = sun.angular_radius;
        let mut total = 0.;
        for i in 0..rings {
            let theta = (i as f64 + 0.5) / rings as f64 * r;
            for j in 0..around {
                let phi = (j as f64 + 0.5) / around as f64 * 2. * std::f64::consts::PI;
                let d = sun.direction * theta.cos() + (a * phi.cos() + b * phi.sin()) * theta.sin();
                let d_omega = theta.sin() * (r / rings as f64) * (2. * std::f64::consts::PI / around as f64);
                total += crate::material::bsdf::BSDF::eval(&bsdf, &wo, &d).rgb.x * sun.disc_radiance(&Vector3::zeros(), &d).rgb.x * d_omega;
            }
        }
        total
    }

    #[test]
    fn test_rough_metal_reflects_the_sun_without_fireflies() {
        // A fuzzed mirror found the sun only when a reflected ray happened to hit its disc,
        // 6.8e-5 sr: rarely, and then 15,000 times brighter than the mean. Light sampling with
        // MIS finds it every time. α = 0.2 can't resolve the disc (the sun is a point); 0.02
        // can (it is sampled over the disc, and BSDF sampling shares the work).
        crate::geometry::seed_rand(21, 1);
        for alpha in [0.2f64, 0.02] {
            let roughness = alpha.sqrt();
            let (mean, max, s) = sun_glint(roughness, 4000);
            let expected = glint_expected(alpha, &s);
            assert!((mean - expected).abs() < 0.02 * expected, "alpha {}: {} vs {}", alpha, mean, expected);
            assert!(max < 3. * mean, "alpha {}: a sample of {} against a mean of {}", alpha, max, mean);
        }
    }
}
