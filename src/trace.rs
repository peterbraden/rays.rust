use color::Color;
use ray::Ray;
use scene::Scene;
use std::f64;
use intersection::Intersection;
use sceneobject::SceneObject;

pub fn trace (r: &Ray, depth: i32, s: &Scene) -> Color {
    
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64);

    match closest {
        Some(x) => return trace_closest(r, x, depth, s),
        None => return Color::black(),
    }
}

fn trace_closest(r: &Ray, intersection: Intersection, depth: i32, s: &Scene) -> Color {

    let closest = intersection.object;
    let mut out = ambient(&intersection, s);


    if (depth < s.maxDepth){
        out = out + reflection(r, intersection, depth, s);
    }

    return out;
}


fn ambient(intersection: &Intersection, s: &Scene) -> Color {
    return intersection.object.get_material().pigment * s.ambient;
}

fn reflection(r: &Ray, intersection: Intersection, depth: i32, s: &Scene) -> Color {

    let refl = Ray {
        ro: intersection.point,
        rd: r.rd - (intersection.normal * 2.0 * (intersection.normal * r.rd)),
    };

    return trace(&refl, depth + 1, s) * intersection.object.get_material().reflection; 
}
