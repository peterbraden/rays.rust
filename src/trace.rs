use color::Color;
use ray::Ray;
use scene::Scene;
use intersection::Intersection;
use std::f64;

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

    let material = intersection.object.medium.material_at(intersection.point);

    let mut cast = 1;
    let mut out = Color::black();
    
    let interaction = material.scatter(r, &biased_intersection, s);

    if depth < s.max_depth {
        if let Some(ray) = interaction.ray {
            let (c, col) = trace(&ray, depth + 1, s);
            out = out + interaction.attenuate * col;
            cast += c;
        } else {
            // Wish I could if && if let
            out = out + interaction.attenuate;
        }
    } else {
        out = out + interaction.attenuate;
    }

    return (cast, out);
}


