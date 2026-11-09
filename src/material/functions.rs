use crate::na::{Vector3};
use crate::ray::Ray;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::geometry::{random_point_on_unit_sphere};
use crate::intersection::Intersection;
use crate::color::Color;
use crate::geometry::{rand};
use crate::light::Light;
use crate::scene::Scene;

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
fn lambertian_direction(normal: Vector3<f64>, on_sphere: Vector3<f64>) -> Vector3<f64> {
    let d = normal + on_sphere;
    // The sample can (rarely) cancel the normal exactly.
    if d.norm() < 1e-9 { normal } else { d.normalize() }
}

pub fn scatter_lambertian(albedo: Color, intersection: &Intersection) -> ScatteredRay {
    let refl = Ray {
        ro: intersection.point,
        // Normal plus a point on the unit sphere gives a cosine-weighted direction.
        rd: lambertian_direction(intersection.normal, random_point_on_unit_sphere()),
    };
    ScatteredRay{ emit: Color::black(), attenuate:albedo, ray: Some(refl) }
}

/// Distance to move a scattered ray's origin off the surface, onto the side it travels into.
/// Must exceed the scene's shadow bias, which pushes intersection points outward.
const SURFACE_OFFSET: f64 = 0.001;

/// Probability that light is reflected rather than transmitted at a dielectric boundary.
/// Returns 1 for total internal reflection.
pub fn fresnel_reflectance(cos_i: f64, ni_over_nt: f64, refractive_index: f64) -> f64 {
    let sin2_t = ni_over_nt * ni_over_nt * (1.0 - cos_i * cos_i);
    if sin2_t >= 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    // Schlick needs the cosine on the less dense side, ie. the larger angle.
    schlick(cos_i.min(cos_t), refractive_index)
}

pub fn scatter_dielectric(
    refractive_index: f64,
    albedo: Color, 
    r: &Ray,
    intersection: &Intersection
) -> ScatteredRay {

    let drn = r.rd.dot(&intersection.normal);
    let exiting = drn > 0.0;
    
    // Normal on the side the ray arrives from. When the ray shoots through the object back
    // into vacuum, the surface normal has to be inverted.
    let outward_normal = if exiting { -intersection.normal } else { intersection.normal };
    
    let ni_over_nt = if exiting {
        refractive_index // Assumes it comes from air - TODO
    } else {
        1.0 / refractive_index
    };

    let cos_i = drn.abs() / r.rd.norm();
    let reflect_prob = fresnel_reflectance(cos_i, ni_over_nt, refractive_index);

    if let Some(refracted) = refract(r.rd, outward_normal, ni_over_nt) {
        if rand() >= reflect_prob {
            return ScatteredRay{ emit: Color::black(),
                attenuate: albedo,
                ray: Some( Ray {
                    ro: intersection.point - outward_normal * SURFACE_OFFSET,
                    rd: refracted
                }),
            };
        }
    }

    // Fresnel or total internal reflection: the ray stays on the side it arrived from.
    let reflected = reflect(r.rd, intersection.normal);
    ScatteredRay { emit: Color::black(),
        attenuate: Color::white(),
        ray: Some(Ray {
            ro: intersection.point + outward_normal * SURFACE_OFFSET,
            rd: reflected
        }) 
    }
}


/// Diffuse light arriving directly from each point light that isn't in shadow. Point lights
/// can't be hit by scattered rays, so diffuse materials must sample them explicitly.
pub fn direct_lighting(albedo: Color, intersection: &Intersection, s: &Scene) -> Color {
    let mut direct = Color::black();
    for light in &s.lights {
        let light_vec = light.position - intersection.point;
        let shadow_ray = Ray {ro: intersection.point, rd: light_vec.normalize()};
        if s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), f64::MIN_POSITIVE).is_none() {
            direct = direct + diffuse(albedo, intersection, &light_vec, light);
        }
    }
    direct
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sceneobject::SceneObject;
    use crate::shapes::sphere::Sphere;
    use crate::material::texture::Solid;
    use crate::material::legacy::FlatColor;
    use std::sync::Arc;

    #[test]
    fn test_lambertian_is_cosine_weighted() {
        // For a cosine-weighted hemisphere, E[cos theta] = 2/3.
        let n = Vector3::new(0., 1., 0.);
        let samples = 100_000;
        let mean: f64 = (0..samples)
            .map(|_| lambertian_direction(n, random_point_on_unit_sphere()).dot(&n))
            .sum::<f64>() / samples as f64;
        assert!((mean - 2. / 3.).abs() < 0.01, "mean cosine {}", mean);
    }

    #[test]
    fn test_fresnel_continuous_at_critical_angle() {
        // Exiting glass, just inside the critical angle (cos ~0.745), reflectance should be
        // approaching 1 rather than jumping from ~0.04.
        let ri: f64 = 1.5;
        let critical = (1.0 - 1.0 / (ri * ri)).sqrt();
        assert!(fresnel_reflectance(critical + 1e-4, ri, ri) > 0.8);
        assert_eq!(fresnel_reflectance(critical - 1e-4, ri, ri), 1.0);
        // Entering at normal incidence is just r0.
        assert!((fresnel_reflectance(1.0, 1.0 / ri, ri) - 0.04).abs() < 1e-9);
    }

    #[test]
    fn test_internal_reflection_stays_inside() {
        // A ray inside a unit sphere grazing the top: total internal reflection. The hit point
        // has been biased outward along the normal, as trace does.
        let i = Intersection {
            dist: 1.,
            point: Vector3::new(0., 1.0001, 0.),
            normal: Vector3::new(0., 1., 0.),
            object: Arc::new(SceneObject {
                geometry: Box::new(Sphere::new(Vector3::new(0., 0., 0.), 1.)),
                medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
            }),
        };
        let r = Ray { ro: Vector3::new(-1., 0.8, 0.), rd: Vector3::new(1., 0.2, 0.).normalize() };
        let out = scatter_dielectric(1.5, Color::white(), &r, &i).ray.unwrap();
        assert!(out.ro.norm() < 1., "reflected ray starts outside: {}", out.ro);
        assert!(out.rd.y < 0.);
    }
}
