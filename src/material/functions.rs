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

/// A direction for diffuse reflection: the normal plus a point uniformly distributed on the
/// unit sphere, normalised, which is distributed in proportion to the cosine of the angle from
/// the normal.
fn lambertian_direction(normal: Vector3<f64>, on_sphere: Vector3<f64>) -> Vector3<f64> {
    let d = normal + on_sphere;
    // The sample can (rarely) cancel the normal exactly.
    if d.norm() < 1e-9 { normal } else { d.normalize() }
}

/// Implement Lambertian reflection (purely diffuse) for ambient incoming light (light at a random
/// incoming angle.)
/// Practically, we reflect in a random direction about the normal, chosen by
/// `lambertian_direction`. This will be very noisy if we don't subsample a lot.
pub fn scatter_lambertian(albedo: Color, intersection: &Intersection) -> ScatteredRay {
    let refl = Ray {
        ro: intersection.point,
        // Normal plus a point on the unit sphere gives a cosine-weighted direction.
        rd: lambertian_direction(intersection.normal, random_point_on_unit_sphere()),
    };
    ScatteredRay{ sampled_lights: false, emit: Color::black(), attenuate:albedo, ray: Some(refl) }
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
            return ScatteredRay{ sampled_lights: false, emit: Color::black(),
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
    ScatteredRay { sampled_lights: false, emit: Color::black(),
        attenuate: Color::white(),
        ray: Some(Ray {
            ro: intersection.point + outward_normal * SURFACE_OFFSET,
            rd: reflected
        }) 
    }
}


/// Diffuse light arriving directly from the scene's lights (next-event estimation): every point
/// light that isn't in shadow, and one sample of the area lights. Point lights can't be hit by
/// scattered rays at all, and small area lights rarely are, so diffuse materials sample them
/// explicitly. Materials that call this set `ScatteredRay::sampled_lights`, so the trace ignores
/// emission from area lights hit by their continuation ray, which would count them twice.
pub fn direct_lighting(albedo: Color, intersection: &Intersection, s: &Scene) -> Color {
    let mut direct = Color::black();
    for light in &s.lights {
        let light_vec = light.position - intersection.point;
        let shadow_ray = Ray {ro: intersection.point, rd: light_vec.normalize()};
        if !s.objects.occluded(&shadow_ray, light_vec.norm(), f64::MIN_POSITIVE) {
            direct = direct + diffuse(albedo, intersection, &light_vec, light);
        }
    }
    if s.render.sample_lights {
        direct = direct + sample_area_light(albedo, intersection, s);
    }
    direct
}

/// One sample of the light reflected from the area lights, following Peter Shirley, Changyaw
/// Wang and Kurt Zimmerman, "Monte Carlo Techniques for Direct Lighting Calculations", ACM
/// Transactions on Graphics 15(1), 1996, doi:10.1145/226150.226151.
///
/// Rather than integrating incoming light over directions, integrate over the lights' surfaces.
/// For a point y on a light, with normal n_y, at distance d along the unit direction w from the
/// shaded point x with normal n_x, a patch dA subtends solid angle cos(theta_y) dA / d^2, so
///
/// ```text
/// L_direct(x) = integral over lights of (albedo / pi) L_e(y, -w) cos(theta_x) cos(theta_y) / d^2 V(x, y) dA
/// ```
///
/// where V is 1 if nothing blocks the segment from x to y. Choosing one of n lights uniformly,
/// then a point on it uniformly by area, gives density p(y) = 1 / (n area), and the estimate is
/// the integrand divided by p(y). Lights are chosen uniformly rather than by power, and only
/// light sampling is used: combining it with sampling the BSDF, as Veach and Guibas,
/// "Optimally Combining Sampling Techniques for Monte Carlo Rendering", SIGGRAPH 1995, describe,
/// would help glossy surfaces and large lights.
fn sample_area_light(albedo: Color, intersection: &Intersection, s: &Scene) -> Color {
    let n = s.area_lights.len();
    if n == 0 {
        return Color::black();
    }
    let light = &s.area_lights[((rand() * n as f64) as usize).min(n - 1)];
    let (y, n_y) = match light.geometry.sample_surface([rand(), rand(), rand()]) {
        Some(sample) => sample,
        None => return Color::black(),
    };
    let to_light = y - intersection.point;
    let d2 = to_light.norm_squared();
    let d = d2.sqrt();
    let w = to_light / d;
    let cos_x = w.dot(&intersection.normal);
    if cos_x <= 0. {
        return Color::black();
    }
    let emitted = light.medium.material_at(y).emitted(&n_y, &-w);
    if emitted == Color::black() {
        return Color::black();
    }
    // Stop the shadow ray just short of the light, which would otherwise block itself.
    let shadow_ray = Ray { ro: intersection.point, rd: w };
    if s.objects.occluded(&shadow_ray, d * (1. - 1e-6), f64::MIN_POSITIVE) {
        return Color::black();
    }
    let cos_y = n_y.dot(&-w).abs();
    albedo * emitted * (cos_x * cos_y / d2 * light.geometry.surface_area() * n as f64 / std::f64::consts::PI)
}

/// Light reflected by a Lambertian surface from a point light, pbrt's convention: `intensity` is
/// radiant intensity, so irradiance falls off with the square of distance, and the BRDF is
/// albedo / pi.
pub fn diffuse (pigment: Color, i: &Intersection, light_vec: &Vector3<f64>, light: &Light) -> Color {
    let cos = light_vec.normalize().dot(&i.normal);
    if cos.is_sign_positive() {
        light.color * pigment * (cos * light.intensity / (std::f64::consts::PI * light_vec.norm_squared()))
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
        let i = Intersection { part: 0,
            dist: 1.,
            point: Vector3::new(0., 1.0001, 0.),
            normal: Vector3::new(0., 1., 0.),
            object: &(SceneObject {
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
