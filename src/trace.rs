use color::Color;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use scene::Scene;
use intersection::Intersection;
use std::f64;
use light::Light;
use geometry::random_point_on_unit_sphere;
use material::{MaterialModel, AmbientLambertian, Ambient};

// Returns num rays cast, Color
pub fn trace (r: &Ray, depth: u64, s: &Scene) -> (u64, Color) {
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64, None);

    match closest {
        Some(x) => return trace_intersection(r, x, depth, s),
        None => return (1, s.background),
    }
}

fn trace_intersection(r: &Ray, intersection: Intersection, depth: u64, s: &Scene) -> (u64, Color) {
    // Shadow bias -> Move the origin of the intersection point along the normal, in case a
    // floating point error puts it slightly below the surface which would cause a sign flip
    // leading to shadow acne.
    let mut biased_intersection = intersection.clone();
    biased_intersection.point = intersection.point + (intersection.normal * s.shadow_bias);

    let mut cast = 1;
    let mut out = ambient(r, &biased_intersection, s);
    let (cad, ad) = ambient_diffuse(r, &biased_intersection, s, depth);
    out = out + ad;
    cast = cast + cad;

    for light in &s.lights {
        let light_vec = light.position - biased_intersection.point;
        let shadow_ray = Ray {ro: biased_intersection.point, rd: light_vec};
        let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), 0.1, Some(intersection.object)); 
        cast = cast + 1;

        match shadow_intersection {
            Some(_) => (
                    // Point in shadow...
                ),
            None => (
                out = out + trace_for_light(&r, &light_vec, &light, &biased_intersection, &s)
                ),
        }
    }

    if s.reflection && depth < s.max_depth {
        let (c, refl) = reflection(r, out, &biased_intersection, depth + 1, s);
        out = refl;
        cast = cast + c;
    }

    return (cast, out);
}

fn trace_for_light(r: &Ray, light_vec: &Vec3<f64>, l: &Light, intersection: &Intersection, s: &Scene) -> Color {
    return diffuse(&intersection, &light_vec, &l, s) + specular(r, intersection, light_vec, s);
}


fn ambient(r: &Ray, intersection: &Intersection, s: &Scene) -> Color {
    let ambient = Ambient { pigment: intersection.object.medium.material_at(intersection.point).pigment };
    let (_c, col, _r) = ambient.scatter(r, intersection, s);
    return col;
}

// Diffuse light due to roughness and ambient light
// - Lambertian with randomised unit vector
fn ambient_diffuse(r: &Ray, intersection: &Intersection, s: &Scene, depth: u64) -> (u64, Color) {

    let m = intersection.object.medium.material_at(intersection.point);

    if s.ambient_diffuse == 0 || depth > s.ambient_diffuse || m.albedo < 0.01 {
        return (0, Color::black());
    }

    let model = AmbientLambertian { albedo: m.pigment * m.albedo };
    let (_c, attenuate, refl) = model.scatter(r, intersection, s);
    let (c, col) = trace(&refl.unwrap(), depth + 1, s);
    return (c, col * attenuate)
}

fn specular (r: &Ray, intersection: &Intersection, light_vec: &Vec3<f64>, s: &Scene) -> Color {
    let phong = intersection.object.medium.material_at(intersection.point).phong;
    if !s.specular || phong == 0. {
        return Color::black();
    }
    let ln = light_vec.normalize();
    let refl = ln - (intersection.normal * (2.0 * intersection.normal.dot(&ln) ) ); 
    let dp = refl.dot(&r.rd);

    if dp > 0f64 {
        let spec_scale = dp.powf(phong);
        return Color::white() * spec_scale;
    }

    return Color::black();
}

// Lambertian
fn diffuse (i: &Intersection, light_vec: &Vec3<f64>, light: &Light, s: &Scene) -> Color {
    if !s.diffuse {
        return Color::black();
    }

    let diffuse_scale = light_vec.normalize().dot(&i.normal) * light.intensity;
    if diffuse_scale.is_sign_positive() {
        return light.color * i.object.medium.material_at(i.point).pigment * diffuse_scale;
    }
    return Color::black()
}


fn reflection(r: &Ray, out: Color, intersection: &Intersection, depth: u64, s: &Scene) -> (u64, Color) {
    let m = intersection.object.medium.material_at(intersection.point);
    let scale = m.reflection;
    if scale < 0.0001 {
        return (depth, out)
    }

    let fuzz =random_point_on_unit_sphere() * m.roughness;

    let refl = Ray {
        ro: intersection.point,
        rd: r.rd - (intersection.normal * 2.0 * intersection.normal.dot(&r.rd) + fuzz),
    };

    let (c, col) = trace(&refl, depth + 1, s);

    return (c, (out * (1. - scale)) + (col * scale) );
}
