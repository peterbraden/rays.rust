use color::Color;
use na::{Vector3};
use ray::Ray;
use scene::Scene;
use intersection::Intersection;
use std::f64;

// Returns num rays cast, Color
pub fn trace (r: &Ray, depth: u64, s: &Scene) -> (u64, Color) {
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64);

    match closest {
        Some(x) => return trace_intersection(r, x, depth, s),
        None => return (1, s.render.background),
    }
}

fn trace_sample(r: &Ray, intersection: &Intersection, depth: u64, s: &Scene) -> (u64, Color){
    let mut cast = 1;
    let material = intersection.object.medium.material_at(intersection.point);
    let interaction = material.scatter(r, &intersection, s);

    if depth < s.render.max_depth as u64 && interaction.attenuate > s.black_threshold {
        if let Some(ray) = interaction.ray {
            let (c, col) = trace(&ray, depth + 1, s);
            cast += c;
            return (cast, interaction.attenuate * col.clamp(2.)); // TODO - use emission
        } else {
			return (cast, interaction.attenuate)
		}
    }
	
	// Too many bounce, fallback to color
    return (cast, (interaction.attenuate * s.render.background));
}

fn trace_intersection(r: &Ray, intersection: Intersection, depth: u64, scene: &Scene) -> (u64, Color) {
    // Shadow bias -> Move the origin of the intersection point along the normal, in case a
    // floating point error puts it slightly below the surface which would cause a sign flip
    // leading to shadow acne.
    let mut biased_intersection = intersection.clone();
    biased_intersection.point = intersection.point + (intersection.normal * scene.render.shadow_bias);

    let mut cast = 1;
    let (c, o) = trace_sample(r, &biased_intersection, depth, scene);
    cast = cast + c;
    return (cast, o) 
}


