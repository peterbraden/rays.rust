use crate::na::{Vector3};
use crate::ray::Ray;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::geometry::{random_point_on_unit_sphere};
use crate::intersection::Intersection;
use crate::color::Color;
use crate::geometry::{rand};
use crate::light::Light;

pub fn refract(v: Vector3<f64>, n: Vector3<f64>, ni_over_nt:f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        return Some( (uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    }
    None
}

/// Schlick approximation of Fresnel
pub fn schlick(cosine:f64, ref_idx:f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

pub fn reflect(v: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
    v - normal * 2.0 * normal.dot(&v)
}

/// Implement Lambertian reflection (purely diffuse) for ambient incoming light (light at a random
/// incoming angle.)
/// Practically, we implement random reflection within a unit sphere on the normal.
/// This will be very noisy if we don't subsample a lot.
pub fn scatter_lambertian(albedo: Color, intersection: &Intersection) -> ScatteredRay {
    let refl = Ray {
        ro: intersection.point,
        rd: intersection.normal + random_point_on_unit_sphere(),
    };
    ScatteredRay{ attenuate:albedo, ray: Some(refl) }
}

pub fn scatter_dielectric(
    refractive_index: f64,
    albedo: Color, 
    r: &Ray,
    intersection: &Intersection
) -> ScatteredRay {

    let mut ni_over_nt = refractive_index; // Assumes it comes from air - TODO
    let cosine;
    let drn = r.rd.dot(&intersection.normal);
    let outward_normal;
    if drn > 0.0 {
        // when ray shoot through object back into vacuum,
        // ni_over_nt = ref_idx, surface normal has to be inverted.
        cosine = drn / r.rd.norm(); 
        outward_normal = -intersection.normal
    } else {
        // when ray shoots into object,
        // ni_over_nt = 1 / ref_idx.
        cosine = - drn / r.rd.norm(); 
        ni_over_nt = 1.0 / refractive_index; 
        outward_normal = intersection.normal
    };

    match refract(r.rd, outward_normal, ni_over_nt) {
        Some(refracted) => {
            // refracted ray exists
            // Schlick approximation of fresnel amount
            let reflect_prob = schlick(cosine, refractive_index);
            if rand() >= reflect_prob {
                return ScatteredRay{
                    attenuate: albedo,
                    ray: Some( Ray {
                        ro: intersection.point + (refracted * 0.001),
                        rd: refracted
                    }),
                };
            }
        },
        None => {
            // refracted ray does not exist
            //  - total internal reflection
        }
    }

    let reflected = reflect(r.rd, intersection.normal);
    ScatteredRay {
        attenuate: Color::white(),
        ray: Some(Ray {
            ro: intersection.point,
            rd: reflected
        }) 
    }
}


pub fn diffuse (pigment: Color, i: &Intersection, light_vec: &Vector3<f64>, light: &Light) -> Color {
    let diffuse_scale = light_vec.normalize().dot(&i.normal) * light.intensity;
    if diffuse_scale.is_sign_positive() {
        light.color * pigment * diffuse_scale
    } else {
        Color::black()
    }
}

pub fn phong (phong: f64, r: &Ray, intersection: &Intersection, light_vec: &Vector3<f64>) -> Color {
    if phong < f64::MIN_POSITIVE {
        return Color::black();
    }
    let ln = light_vec.normalize();
    let refl = ln - (intersection.normal * (2.0 * intersection.normal.dot(&ln) ) ); 
    let dp = refl.dot(&r.rd);

    if dp > 0f64 {
        let spec_scale = dp.powf(phong);
        return Color::white() * spec_scale;
    }

    Color::black()
}
