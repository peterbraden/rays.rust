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
    let ambient = ambient(intersection, s); 

    return ambient
    //return Color::new(255f64,255f64,255f64);
}


fn ambient(intersection: Intersection, s: &Scene) -> Color {
    return intersection.object.get_material().pigment * s.ambient;
}
