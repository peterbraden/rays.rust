use crate::color::Color;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::scene::{Scene, PathTermination};
use crate::geometry::rand;
use crate::intersection::Intersection;
use std::f64;

// Returns num rays cast, Color
pub fn trace (r: &Ray, depth: u64, s: &Scene) -> (u64, Color) {
    trace_from(r, depth, s, false)
}

/// Trace a ray. `after_light_sampling` is set for a ray scattered from a surface that has just
/// sampled the area lights directly; their emission is then already counted, and is ignored if
/// this ray hits one.
fn trace_from(r: &Ray, depth: u64, s: &Scene, after_light_sampling: bool) -> (u64, Color) {
    let closest = s.objects.nearest_intersection(r, f64::INFINITY, 0f64);

    match closest {
        Some(x) => trace_intersection(r, x, depth, s, after_light_sampling),
        None => (1, s.render.background.radiance(&r.rd)),
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

fn trace_sample(r: &Ray, intersection: &Intersection, depth: u64, s: &Scene, after_light_sampling: bool) -> (u64, Color){
    let mut cast = 1;
    if after_light_sampling && s.is_sampled_light(&intersection.object) {
        return (cast, Color::black());
    }
    let material = intersection.object.medium.material_at(intersection.point);
    let interaction = material.scatter(r, intersection, s);

    let incoming = match interaction.ray {
        // No continuation: attenuate is the final colour.
        None => Color::white(),
        Some(ray) => match continuation_weight(&interaction.attenuate, depth, s) {
            Some(w) => {
                let (c, col) = trace_from(&ray, depth + 1, s, interaction.sampled_lights);
                cast += c;
                col * w
            },
            // Path stopped: contributes nothing beyond its emission.
            None => Color::black(),
        },
    };

    (cast, interaction.emit + interaction.attenuate * incoming)
}

fn trace_intersection(r: &Ray, intersection: Intersection, depth: u64, scene: &Scene, after_light_sampling: bool) -> (u64, Color) {
    // Shadow bias -> Move the origin of the intersection point along the normal, in case a
    // floating point error puts it slightly below the surface which would cause a sign flip
    // leading to shadow acne.
    let mut biased_intersection = intersection.clone();
    // Bias towards the side the ray arrived from, which for a ray inside an object is inwards.
    biased_intersection.point = intersection.point + (intersection.facing_normal(r) * scene.render.shadow_bias);

    let mut cast = 1;
    let (c, o) = trace_sample(r, &biased_intersection, depth, scene, after_light_sampling);
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
            "lights": [ {{ "location": [0, 10, 0], "intensity": 314.1592653589793 }} ],
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
    fn test_lambertian_lit_by_point_light() {
        // Black background, so the bounce gathers nothing; the light must still reach it.
        let s = plane_scene(r#"{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }"#, 0.);
        let c = trace_down(&s);
        assert!((c.rgb[0] - 0.5).abs() < 1e-3, "{:?}", c);
    }

    /// The inside of a unit sphere of albedo 0.5, with a point light of intensity π at its
    /// centre, viewed from the centre.
    fn furnace(max_depth: usize) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": {},
            "camera": {{ "location": [0, 0, 0], "lookat": [0, 0, 1], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
            "lights": [ {{ "location": [0, 0, 0], "intensity": 3.141592653589793 }} ],
            "objects": [ {{ "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "M" }} ]
        }}"#, max_depth))
    }

    /// A Lambertian floor of albedo 0.5 under a 2 x 2 square emitter of radiance 1, one unit
    /// above, facing down. Black background, so all light comes from the emitter.
    fn under_square_light(sample_lights: bool) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1, "sample_lights": {},
            "camera": {{ "location": [0, 0.5, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 }},
            "materials": {{
                "F": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }},
                "L": {{ "type": "diffuse-light", "color": [1, 1, 1], "intensity": 1 }}
            }},
            "objects": [
                {{ "type": "plane", "y": 0, "material": "F" }},
                {{ "type": "triangle", "v0": [-1, 1, -1], "v1": [1, 1, -1], "v2": [1, 1, 1], "material": "L" }},
                {{ "type": "triangle", "v0": [-1, 1, -1], "v1": [1, 1, 1], "v2": [-1, 1, 1], "material": "L" }}
            ]
        }}"#, sample_lights))
    }

    /// Mean and standard deviation of n samples of the radiance from the floor below the light.
    fn floor_radiance(s: &Scene, n: usize) -> (f64, f64) {
        let r = Ray { ro: Vector3::new(0., 0.5, 0.), rd: Vector3::new(0., -1., 0.) };
        let v: Vec<f64> = (0..n).map(|_| trace(&r, 0, s).1.rgb.x).collect();
        let mean = v.iter().sum::<f64>() / n as f64;
        let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        (mean, var.sqrt())
    }

    #[test]
    fn test_area_light_sampling_is_unbiased() {
        // The square subtends form factor F = 4 x 0.13853 from the point below its centre (four
        // 1 x 1 rectangles at distance 1, each seen from beneath a corner), so the reflected
        // radiance is albedo x L x F.
        let corner: f64 = 1. / (2. * std::f64::consts::PI) * 2. * (0.5f64.sqrt() * (0.5f64.sqrt()).atan());
        let expected = 0.5 * 4. * corner;
        crate::geometry::seed_rand(1, 1);
        let (sampled, sampled_sd) = floor_radiance(&under_square_light(true), 4000);
        let (unsampled, unsampled_sd) = floor_radiance(&under_square_light(false), 20000);
        assert!((sampled - expected).abs() < 0.005, "light sampling {} vs {}", sampled, expected);
        assert!((unsampled - expected).abs() < 0.01, "without {} vs {}", unsampled, expected);
        // Less noise, though only modestly for a light this large and close, where the geometry
        // term varies a lot across it; the gain is largest for small lights.
        assert!(sampled_sd < unsampled_sd * 0.75, "sd {} vs {}", sampled_sd, unsampled_sd);
    }

    #[test]
    fn test_furnace() {
        for (depth, expected) in [(0, 0.5), (1, 0.75), (10, 1. - 0.5f64.powi(11))] {
            let s = furnace(depth);
            for d in [Vector3::new(0., 0., 1.), Vector3::new(1., 2., -3.), Vector3::new(-1., -1., 0.2)] {
                let c = trace(&Ray { ro: Vector3::zeros(), rd: d.normalize() }, 0, &s).1;
                assert!((c.rgb.x - expected).abs() < 1e-5, "depth {}: {} for {}, expected {}", depth, c.rgb.x, d, expected);
            }
        }
    }

    #[test]
    fn test_point_light_falls_off_with_distance_squared() {
        // Light 10 units above; at 20 units the irradiance is a quarter.
        let at = |y: f64| SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [1, 1, 1] }} }},
            "lights": [ {{ "location": [0, {}, 0], "intensity": 1 }} ],
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, y));
        let near = trace_down(&at(10.)).rgb[0];
        let far = trace_down(&at(20.)).rgb[0];
        // albedo / pi * I / d^2
        assert!((near - 1. / (std::f64::consts::PI * 100.)).abs() < 1e-6, "{}", near);
        assert!((far * 4. - near).abs() < 1e-6, "{} {}", near, far);
    }

    #[test]
    fn test_lambertian_shadowed_from_point_light() {
        // One bounce at most, so only the shadowed direct term can contribute.
        let s = SceneFile::from_string(r#"{
            "background": [0, 0, 0], "max_depth": 1,
            "camera": { "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "M": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } },
            "lights": [ { "location": [0, 10, 0], "intensity": 314.1592653589793 } ],
            "objects": [
                { "type": "plane", "y": 0, "material": "M" },
                { "type": "sphere", "location": [0, 5, 0], "radius": 1, "material": "M" }
            ]
        }"#.to_string());
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        for _ in 0..200 {
            assert_eq!(trace(&r, 0, &s).1, Color::black());
        }
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
