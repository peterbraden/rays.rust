use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::bsdf::LambertianBSDF;
use crate::material::functions::scatter_bsdf;

pub struct Lambertian {
    pub albedo: Color,
}
impl MaterialModel for Lambertian {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        // Light sampled directly, and a cosine-weighted bounce, combined by MIS.
        scatter_bsdf(&LambertianBSDF { albedo: self.albedo.as_reflectance(), normal: intersection.normal }, r, &intersection.point, s)
    }
}
