use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_on_unit_sphere, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, scatter_dielectric, diffuse};


/// A plastic model.
/// Specular attenuation
///
/// Roughness causes lambertian,
/// Opacity causes dielectric,
/// - Reflection
///
pub struct Plastic {
    pub albedo: Color,
    pub refractive_index: f64,
    pub roughness: f64,
    pub opacity: f64,
}
impl MaterialModel for Plastic {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        let diffuse_probability = rand();
        if diffuse_probability > self.opacity {
            // Direct lighting is emitted; the bounce carries indirect light, weighted by albedo.
            let mut direct = Color::black();

            for light in &s.lights {
                let light_vec = light.position - intersection.point;
                let shadow_ray = Ray {ro: intersection.point, rd: light_vec.normalize()};
                let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), f64::MIN_POSITIVE); 
                match shadow_intersection {
                    Some(_) => (),// Point in shadow...
                    None => {
                        direct = direct + diffuse(self.albedo, intersection, &light_vec, light);
                    },
                }
            }
            
            ScatteredRay { emit: direct, ..scatter_lambertian(self.albedo, intersection) }
        } else {
            scatter_dielectric(self.refractive_index, self.albedo, r, intersection)
        }
    }
}

