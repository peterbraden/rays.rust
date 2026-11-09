use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_in_unit_ball, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, direct_lighting, fresnel_reflectance, reflect};


/// A plastic model: an opaque diffuse base under a clear, glossy coat.
///
/// - The coat reflects with the Fresnel probability for its refractive index, so reflections
///   strengthen at grazing angles. Roughness fuzzes the coat's reflection.
/// - Otherwise light reaches the base and scatters diffusely, tinted by albedo.
///
pub struct Plastic {
    pub albedo: Color,
    pub refractive_index: f64,
    pub roughness: f64,
}

impl MaterialModel for Plastic {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        let cos_i = (r.rd.dot(&intersection.normal) / r.rd.norm()).abs();
        let coat = fresnel_reflectance(cos_i, 1. / self.refractive_index, self.refractive_index);

        if rand() < coat {
            // Clear coat: reflect, untinted.
            let refl = Ray {
                ro: intersection.point,
                rd: reflect(r.rd, intersection.normal) + random_point_in_unit_ball() * self.roughness,
            };
            if refl.rd.dot(&intersection.normal) <= 0. {
                // Fuzz has pushed the reflection into the surface: absorb it.
                return ScatteredRay { emit: Color::black(), attenuate: Color::black(), ray: None };
            }
            return ScatteredRay { emit: Color::black(), attenuate: Color::white(), ray: Some(refl) };
        }

        // Base: direct lighting is emitted; the bounce carries indirect light, weighted by albedo.
        ScatteredRay {
            emit: direct_lighting(self.albedo, intersection, s),
            ..scatter_lambertian(self.albedo, intersection)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use crate::scenefile::SceneFile;
    use crate::trace::trace;

    /// A plastic sphere above a white emissive floor, against a black background.
    fn sphere_over_light() -> Scene {
        SceneFile::from_string(r#"{
            "background": [0, 0, 0], "max_depth": 6,
            "camera": { "location": [0, 10, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 },
            "materials": {
                "P": { "type": "plastic", "albedo": [0.9, 0.1, 0.1], "refractive_index": 1.575 },
                "L": { "type": "diffuse-light", "color": [1, 1, 1], "intensity": 1 }
            },
            "objects": [
                { "type": "sphere", "location": [0, 5, 0], "radius": 1, "material": "P" },
                { "type": "plane", "y": 0, "material": "L" }
            ]
        }"#.to_string())
    }

    #[test]
    fn test_plastic_is_opaque() {
        // Looking straight down at the sphere's top: an opaque sphere hides the light below,
        // and with a black sky its top can only show what it reflects, ie. nothing.
        let s = sphere_over_light();
        let r = Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., -1., 0.) };
        for _ in 0..2000 {
            assert_eq!(trace(&r, 0, &s).1, Color::black());
        }
    }

    #[test]
    fn test_coat_reflects_more_at_grazing_angles() {
        let normal = fresnel_reflectance(1., 1. / 1.5, 1.5);
        let grazing = fresnel_reflectance(0.05, 1. / 1.5, 1.5);
        assert!((normal - 0.04).abs() < 1e-9);
        assert!(grazing > 0.5, "{}", grazing);
    }
}
