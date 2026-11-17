use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::bsdf::LambertianBSDF;
use crate::material::functions::scatter_bsdf;

/// Ideal diffuse reflection, of an albedo that may come from a texture (`textures`).
pub struct Lambertian {
    pub albedo: crate::textures::ColorTexture,
}
impl MaterialModel for Lambertian {
    fn flat(&self) -> Option<crate::wavefront::flat::FlatMaterial> {
        match self.albedo {
            crate::textures::ColorTexture::Constant(albedo) => Some(crate::wavefront::flat::FlatMaterial::Diffuse { albedo }),
            _ => None,
        }
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        // Light sampled directly, and a cosine-weighted bounce, combined by MIS.
        let albedo = self.albedo.eval(intersection, s).as_reflectance();
        scatter_bsdf(&LambertianBSDF { albedo, normal: intersection.normal }, r, &intersection.point, s)
    }
}
