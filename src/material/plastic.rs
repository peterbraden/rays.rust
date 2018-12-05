use color::Color;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use geometry::{random_point_on_unit_sphere, rand};
use material::model::{MaterialModel, ScatteredRay};


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
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{

        let diffuse_probability = rand();
        if (diffuse_probability > self.opacity) {
            return scatter_lambertian(self.albedo, intersection);
        } else {
            return scatter_dielectric(self.refractive_index, self.albedo, r, intersection);
        }
    }
}

