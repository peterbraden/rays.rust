use crate::color::Color;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::intersection::Intersection;
use std::f64;

// Returns num rays cast, Color
pub fn trace (r: &Ray, depth: u64, s: &Scene) -> (u64, Color) {
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64);

    match closest {
        Some(x) => trace_intersection(r, x, depth, s),
        None => (1, s.render.background),
    }
}

fn trace_sample(r: &Ray, intersection: &Intersection, depth: u64, s: &Scene) -> (u64, Color){
    let mut cast = 1;
    let material = intersection.object.medium.material_at(intersection.point);
    let interaction = material.scatter(r, intersection, s);

    let incoming = match interaction.ray {
        // No continuation: attenuate is the final colour.
        None => Color::white(),
        Some(ray) if depth < s.render.max_depth as u64
            && interaction.attenuate.as_vec().norm() > s.black_threshold => {
            let (c, col) = trace(&ray, depth + 1, s);
            cast += c;
            col
        },
        // Too many bounces, fall back to the background colour
        Some(_) => s.render.background,
    };

    (cast, interaction.emit + interaction.attenuate * incoming)
}

fn trace_intersection(r: &Ray, intersection: Intersection, depth: u64, scene: &Scene) -> (u64, Color) {
    // Shadow bias -> Move the origin of the intersection point along the normal, in case a
    // floating point error puts it slightly below the surface which would cause a sign flip
    // leading to shadow acne.
    let mut biased_intersection = intersection.clone();
    biased_intersection.point = intersection.point + (intersection.normal * scene.render.shadow_bias);

    let mut cast = 1;
    let (c, o) = trace_sample(r, &biased_intersection, depth, scene);
    cast += c;
    (cast, o) 
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    /// A single plane at y=0 with `material`, a light overhead, and a uniform background.
    fn plane_scene(material: &str, background: f64) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.0001, "background": [{b}, {b}, {b}], "max_depth": 2,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {material} }}, "media": {{}}, "variables": {{}},
            "lights": [ {{ "location": [0, 10, 0], "intensity": 1 }} ],
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, b = background, material = material))
    }

    fn trace_down(s: &Scene) -> Color {
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        trace(&r, 0, s).1
    }

    #[test]
    fn test_whitted_reflection_adds_to_local_shading() {
        let s = plane_scene(r#"{ "type": "whitted", "pigment": [0.5, 0.5, 0.5], "reflection": 0.5 }"#, 0.2);
        // local diffuse 0.5 + reflection 0.5 * background 0.2
        let c = trace_down(&s);
        assert!((c.rgb[0] - 0.6).abs() < 1e-3, "{:?}", c);
    }

    #[test]
    fn test_plastic_direct_light_not_scaled_by_bounce() {
        // With a black background the diffuse bounce sees nothing; direct light must remain.
        let s = plane_scene(r#"{ "type": "plastic", "albedo": [0.5, 0.5, 0.5] }"#, 0.);
        let c = trace_down(&s);
        assert!((c.rgb[0] - 0.5).abs() < 1e-3, "{:?}", c);
    }
}
