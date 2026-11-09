use crate::color::Color;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::scene::{Scene, PathTermination};
use crate::geometry::rand;
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

/// Whether to continue a path, and the factor to scale what it gathers by.
fn continuation_weight(attenuate: &Color, depth: u64, s: &Scene) -> Option<f64> {
    if depth >= s.render.max_depth as u64 || attenuate.as_vec().norm() <= s.black_threshold {
        return None;
    }
    match s.render.path_termination {
        PathTermination::RussianRoulette { min_depth } if depth >= min_depth as u64 => {
            let p = attenuate.as_vec().max().min(1.);
            if rand() < p { Some(1. / p) } else { None }
        },
        _ => Some(1.),
    }
}

fn trace_sample(r: &Ray, intersection: &Intersection, depth: u64, s: &Scene) -> (u64, Color){
    let mut cast = 1;
    let material = intersection.object.medium.material_at(intersection.point);
    let interaction = material.scatter(r, intersection, s);

    let incoming = match interaction.ray {
        // No continuation: attenuate is the final colour.
        None => Color::white(),
        Some(ray) => match continuation_weight(&interaction.attenuate, depth, s) {
            Some(w) => {
                let (c, col) = trace(&ray, depth + 1, s);
                cast += c;
                col * w
            },
            // Path stopped: contributes nothing beyond its emission.
            None => Color::black(),
        },
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
        plane_scene_with(material, background, 2, "")
    }

    fn plane_scene_with(material: &str, background: f64, max_depth: usize, extra: &str) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.0001, "background": [{b}, {b}, {b}], "max_depth": {max_depth} {extra},
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {material} }}, "media": {{}}, "variables": {{}},
            "lights": [ {{ "location": [0, 10, 0], "intensity": 1 }} ],
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, b = background, material = material, max_depth = max_depth, extra = extra))
    }

    fn trace_down(s: &Scene) -> Color {
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        trace(&r, 0, s).1
    }

    const HALF_MIRROR: &str = r#"{ "type": "metal", "reflective": [0.5, 0.5, 0.5] }"#;

    #[test]
    fn test_path_stopped_at_max_depth_is_black() {
        // The reflection would escape to a white background, but the depth limit stops it.
        let s = plane_scene_with(HALF_MIRROR, 1., 0, "");
        assert_eq!(trace_down(&s), Color::black());
        let s = plane_scene_with(HALF_MIRROR, 1., 1, "");
        assert!((trace_down(&s).rgb[0] - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_russian_roulette_is_unbiased() {
        let s = plane_scene_with(HALF_MIRROR, 1., 5,
            r#", "path_termination": "russian_roulette", "russian_roulette_depth": 0"#);
        let n = 4000;
        let samples: Vec<f64> = (0..n).map(|_| trace_down(&s).rgb[0]).collect();
        let mean = samples.iter().sum::<f64>() / n as f64;
        // Survivors carry 0.5 / 0.5 = 1, the rest 0; the expectation is unchanged.
        assert!(samples.iter().any(|x| *x == 0.) && samples.iter().any(|x| (*x - 1.).abs() < 1e-9));
        assert!((mean - 0.5).abs() < 0.03, "mean {}", mean);
    }

    #[test]
    #[should_panic(expected = "Unknown path_termination")]
    fn test_unknown_path_termination() {
        plane_scene_with(HALF_MIRROR, 1., 1, r#", "path_termination": "roulette""#);
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
