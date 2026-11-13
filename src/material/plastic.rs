use crate::material::model::LightSampling;
use crate::color::Color;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::rand;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::bsdf::{BSDF, BSDFSample, LambertianBSDF};
use crate::material::ggx::{GGX, Fresnel, MIN_ALPHA};
use crate::material::functions::{scatter_lambertian, scatter_bsdf, direct_lighting, fresnel_reflectance, reflect};


/// A plastic model: an opaque diffuse base under a clear, glossy coat.
///
/// - The coat reflects by the Fresnel reflectance for its refractive index, so reflections
///   strengthen at grazing angles. A rough coat is a GGX microfacet surface (`ggx`) of
///   roughness α = `roughness`; a smooth one (roughness 0) a mirror.
/// - What the coat lets through reaches the base and scatters diffusely, tinted by albedo:
///   (1 - F(cos θ_o)) albedo / π, with F the coat's reflectance seen from the viewer. This
///   ignores the coat's effect on light leaving the base (its own Fresnel loss, and light
///   reflected back down inside it), as the model always has.
///
pub struct Plastic {
    pub albedo: Color,
    pub refractive_index: f64,
    pub roughness: f64,
}

/// The rough plastic's BSDF: the coat's GGX lobe plus the base's diffuse one, sampled by
/// choosing the coat with probability F(cos θ_o), the share of the light it reflects, and the
/// base otherwise. The density of a direction is then the mixture of the two lobes' densities,
/// which MIS needs whichever lobe produced it.
pub struct PlasticBSDF {
    pub coat: GGX,
    pub base: LambertianBSDF,
    pub refractive_index: f64,
}

impl PlasticBSDF {
    /// The coat's reflectance seen from `wo`, at the surface's normal.
    fn coat_share(&self, wo: &Vector3<f64>) -> f64 {
        let cos = wo.dot(&self.base.normal).max(0.);
        fresnel_reflectance(cos, 1. / self.refractive_index, self.refractive_index)
    }
}

impl BSDF for PlasticBSDF {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        self.coat.eval(wo, wi) + self.base.eval(wo, wi) * (1. - self.coat_share(wo))
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let f = self.coat_share(wo);
        f * self.coat.pdf(wo, wi) + (1. - f) * self.base.pdf(wo, wi)
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        let f = self.coat_share(wo);
        let lobe = if u[2] < f { self.coat.sample(wo, u) } else { self.base.sample(wo, u) };
        let wi = lobe?.wi;
        let pdf = self.pdf(wo, &wi);
        if pdf <= 0. {
            return None;
        }
        Some(BSDFSample { wi, weight: self.eval(wo, &wi) / pdf, pdf })
    }
}

impl MaterialModel for Plastic {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        // Shade whichever side the ray hit.
        let intersection = &intersection.facing(r);
        if self.roughness >= MIN_ALPHA {
            let bsdf = PlasticBSDF {
                coat: GGX { alpha: self.roughness, fresnel: Fresnel::Dielectric(self.refractive_index), normal: intersection.normal },
                base: LambertianBSDF { albedo: self.albedo, normal: intersection.normal },
                refractive_index: self.refractive_index,
            };
            return scatter_bsdf(&bsdf, r, &intersection.point, s);
        }

        // A smooth coat: a mirror, which only its own reflected ray can see the lights by.
        let cos_i = (r.rd.dot(&intersection.normal) / r.rd.norm()).abs();
        let coat = fresnel_reflectance(cos_i, 1. / self.refractive_index, self.refractive_index);
        if rand() < coat {
            let refl = Ray { ro: intersection.point, rd: reflect(r.rd, intersection.normal) };
            return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(), ray: Some(refl) };
        }

        // Base: direct lighting is emitted; the bounce carries indirect light, weighted by albedo.
        ScatteredRay {
            emit: direct_lighting(self.albedo, intersection, s),
            light_sampling: LightSampling::Exclusive,
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
    fn test_rough_plastic_bsdf() {
        let n = Vector3::new(0., 0.6, 0.8);
        for alpha in [0.1, 0.4] {
            let b = PlasticBSDF {
                coat: GGX { alpha, fresnel: Fresnel::Dielectric(1.5), normal: n },
                base: LambertianBSDF { albedo: Color::new(0.8, 0.3, 0.1), normal: n },
                refractive_index: 1.5,
            };
            for wo in [n, Vector3::new(0.9, 0.4, 0.1).normalize()] {
                crate::material::bsdf::tests::check_bsdf(&b, &wo, 0.02);
            }
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
