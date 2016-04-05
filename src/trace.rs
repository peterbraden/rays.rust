use color::Color;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use scene::Scene;
use std::f64;
use intersection::Intersection;
use sceneobject::SceneObject;
use light::Light;

pub fn trace (r: &Ray, depth: i32, s: &Scene) -> Color {
    
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64, None);

    match closest {
        Some(x) => return trace_intersection(r, x, depth, s),
        None => return Color::black(),
    }
}

fn trace_intersection(r: &Ray, intersection: Intersection, depth: i32, s: &Scene) -> Color {

    let mut out = ambient(&intersection, s);

    for light in &s.lights { 
        let light_vec = light.position - intersection.point;
        let shadow_ray = Ray {ro: intersection.point, rd: light_vec};
        let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), 0.1, None); 

        match shadow_intersection {
            Some(_) => (
                    // Point in shadow...
                ),
            None => (
                out = out + trace_for_light(&r, &light_vec, &light, &intersection, &s)
                ),
        }
    }

    if s.reflection && depth < s.max_depth {
        out = out + reflection(r, &intersection, depth + 1, s);
    }

    return out;
}

fn trace_for_light(r: &Ray, light_vec: &Vec3<f64>, l: &Light, intersection: &Intersection, s: &Scene) -> Color {
    return diffuse(&intersection, &light_vec, &l, s) + specular(r, intersection, light_vec, s);
}


fn ambient(intersection: &Intersection, s: &Scene) -> Color {
    return intersection.object.get_material(intersection.point).pigment * s.ambient;
}

fn specular (r: &Ray, intersection: &Intersection, light_vec: &Vec3<f64>, s: &Scene) -> Color {
    if !s.specular {
        return Color::black();
    }
    let ln = light_vec.normalize();
    let refl = ln - (intersection.normal * (2.0 * intersection.normal.dot(&ln) ) ); 
    let dp = refl.dot(&r.rd);

    if dp > 0f64 {
        let spec_scale = dp.powf(intersection.object.get_material(intersection.point).phong);
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
        return i.object.get_material(i.point).pigment * diffuse_scale;
    }
    return Color::black()
}



fn reflection(r: &Ray, intersection: &Intersection, depth: i32, s: &Scene) -> Color {

    let refl = Ray {
        ro: intersection.point,
        rd: r.rd - (intersection.normal * 2.0 * (intersection.normal * r.rd)),
    };

    return trace(&refl, depth + 1, s) * intersection.object.get_material(intersection.point).reflection; 
}
