use crate::color::Color;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::scene::{Scene, PathTermination};
use crate::geometry::rand;
use crate::intersection::Intersection;
use std::f64;

// Returns num rays cast, Color
pub fn trace (r: &Ray, depth: u64, s: &Scene) -> (u64, Color) {
    crate::volume::begin_path();
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

    /// A volume filling the box from -1 to 1 under a uniform white sky, with small spheres off
    /// to the side so the octree subdivides (and would list the volume in several cells).
    fn volume_scene(volume: &str) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [1, 1, 1], "max_depth": 10000,
            "path_termination": "russian_roulette",
            "camera": {{ "location": [0, 0, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
            "objects": [
                {{ "type": "volume", "min": [-1, -1, -1], "max": [1, 1, 1], {} }},
                {{ "type": "sphere", "location": [20, 20, 20], "radius": 0.1, "material": "M" }},
                {{ "type": "sphere", "location": [-20, 20, 20], "radius": 0.1, "material": "M" }},
                {{ "type": "sphere", "location": [20, -20, -20], "radius": 0.1, "material": "M" }}
            ]
        }}"#, volume))
    }

    #[test]
    fn test_volumes_are_kept_out_of_the_octree() {
        let s = volume_scene(r#""density": 1"#);
        assert_eq!(s.objects.items.len(), 3);
        assert_eq!(s.objects.infinite_items.len(), 1);
    }

    #[test]
    fn test_volume_furnace() {
        // A medium that only scatters, lit evenly from every direction, looks exactly as bright
        // as its surroundings however it scatters: every path ends in the sky with weight 1.
        for volume in [r#""density": 2, "g": 0"#, r#""density": 5, "g": 0.85"#,
                       r#""density": 8, "g": -0.5, "noise": { "scale": 2, "seed": 3 }"#] {
            let s = volume_scene(volume);
            for d in [Vector3::new(0., 0., 1.), Vector3::new(0.1, -0.2, 1.)] {
                for _ in 0..200 {
                    let c = trace(&Ray { ro: Vector3::new(0., 0., -5.), rd: d.normalize() }, 0, &s).1;
                    assert!((c.rgb.x - 1.).abs() < 1e-9, "{}: {}", volume, c.rgb.x);
                }
            }
        }
    }

    #[test]
    fn test_volume_furnace_with_similarity() {
        // With albedo 1 the reduced albedo is 1 too: still no light lost.
        let s = volume_scene(r#""density": 6, "g": 0.85, "similarity_after": 2"#);
        for _ in 0..200 {
            let c = trace(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0.1, 0., 1.).normalize() }, 0, &s).1;
            assert!((c.rgb.x - 1.).abs() < 1e-9, "{}", c.rgb.x);
        }
    }

    /// Mean radiance, rays cast and standard deviation looking up through a 2-unit-thick slab
    /// of forward-scattering medium, from below, under a white sky above (and black below).
    fn through_slab(extra: &str, n: usize) -> (f64, u64, f64) {
        let s = SceneFile::from_string(format!(r#"{{
            "background": {{ "type": "gradient", "bottom": [0, 0, 0], "top": [2, 2, 2] }},
            "max_depth": 100000, "path_termination": "russian_roulette",
            "camera": {{ "location": [0, -1, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 }},
            "objects": [ {{ "type": "volume", "min": [-1000, 0, -1000], "max": [1000, 2, 1000],
                           "density": 4, "albedo": [0.98, 0.98, 0.98], "g": 0.85 {} }} ]
        }}"#, extra));
        let r = Ray { ro: Vector3::new(0., -1., 0.), rd: Vector3::new(0., 1., 0.) };
        let mut cast = 0;
        let v: Vec<f64> = (0..n).map(|_| { let (c, col) = trace(&r, 0, &s); cast += c; col.rgb.x }).collect();
        let mean = v.iter().sum::<f64>() / n as f64;
        let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64).sqrt();
        (mean, cast, sd)
    }

    #[test]
    fn test_similarity_approximates_deep_scattering_cheaply() {
        // An optical depth of 8: nearly all light arriving has scattered many times.
        crate::geometry::seed_rand(13, 1);
        let n = 20_000;
        let (exact, exact_cast, exact_sd) = through_slab("", n);
        let (similar, similar_cast, similar_sd) = through_slab(r#", "similarity_after": 4"#, n);
        let se = ((exact_sd.powi(2) + similar_sd.powi(2)) / n as f64).sqrt();
        assert!((similar - exact).abs() < 0.05 * exact + 3. * se, "similarity {} vs exact {} (se {})", similar, exact, se);
        assert!((similar_cast as f64) < 0.5 * exact_cast as f64, "rays {} vs {}", similar_cast, exact_cast);
    }

    #[test]
    fn test_oz_single_octave_is_single_scattering() {
        // As test_single_scattering_of_sunlight_in_a_slab, but a long path rather than depth 0:
        // with Oz the sun is only seen from the first scattering, and with one octave exactly.
        // The rest of the path sees a black sky, so adds nothing.
        crate::geometry::seed_rand(5, 2);
        let slab = |oz: &str| SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1000, "shadow_bias": 0.0001,
            "sun": {{ "direction": [0, 1, 0], "intensity": 2 }},
            "camera": {{ "location": [0, 5, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 }},
            "objects": [ {{ "type": "volume", "min": [-1000, 0, -1000], "max": [1000, 1, 1000],
                           "density": 1.3, "albedo": [0.8, 0.8, 0.8], "oz": {} }} ]
        }}"#, oz));
        let r = Ray { ro: Vector3::new(0., 5., 0.), rd: Vector3::new(0., -1., 0.) };
        let n = 100_000;
        let pi = std::f64::consts::PI;
        let (sigma, a, e): (f64, f64, f64) = (1.3, 0.8, 2.);
        let one = slab(r#"{ "octaves": 1 }"#);
        let mean = (0..n).map(|_| trace(&r, 0, &one).1.rgb.x).sum::<f64>() / n as f64;
        let expected = a * e * (1. - (-2. * sigma).exp()) / (8. * pi);
        assert!((mean - expected).abs() < expected * 0.02, "{} vs {}", mean, expected);
        // Octave i: weight b^i, sun's extinction a^i: a E b^i / 4π × (1 - e^-(1+a^i)σ) / (1 + a^i).
        let three = slab(r#"{ "octaves": 3, "a": 0.5, "b": 0.6 }"#);
        let mean = (0..n).map(|_| trace(&r, 0, &three).1.rgb.x).sum::<f64>() / n as f64;
        let expected: f64 = (0..3).map(|i| {
            let ai = 0.5f64.powi(i);
            a * e * 0.6f64.powi(i) / (4. * pi) * (1. - (-(1. + ai) * sigma).exp()) / (1. + ai)
        }).sum();
        assert!((mean - expected).abs() < expected * 0.02, "{} vs {}", mean, expected);
    }

    #[test]
    fn test_volume_furnace_with_diffraction() {
        // Per-channel weights vary path by path, but average to 1 in every channel.
        crate::geometry::seed_rand(14, 1);
        let s = volume_scene(r#""density": 3, "g": 0.5, "diffraction": { "radius": 4, "spread": 0.05 }"#);
        let n = 20_000;
        let mut sum = Color::black();
        for _ in 0..n {
            sum = sum + trace(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0.1, 0.05, 1.).normalize() }, 0, &s).1;
        }
        let mean = sum / n as f64;
        for c in [mean.rgb.x, mean.rgb.y, mean.rgb.z] {
            assert!((c - 1.).abs() < 0.02, "{:?}", mean);
        }
    }

    #[test]
    fn test_diffraction_colours_single_scattering() {
        // Looking through a thin slab 4° from the sun: single scattering, whose colour is the
        // phase function's at 4°.
        crate::geometry::seed_rand(15, 1);
        let s = SceneFile::from_string(r#"{
            "background": [0, 0, 0], "max_depth": 0, "shadow_bias": 0.0001,
            "sun": { "direction": [0, 0, 1], "intensity": 10 },
            "camera": { "location": [0, 0, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "objects": [ { "type": "volume", "min": [-1000, -1000, 0], "max": [1000, 1000, 0.05],
                           "density": 1, "g": 0.5, "diffraction": { "radius": 5, "spread": 0.05 } } ]
        }"#.to_string());
        let theta: f64 = 4f64.to_radians();
        // The sun is behind the slab (towards +z) and the camera in front, looking 4° from it.
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., theta.sin(), theta.cos()) };
        let n = 20_000;
        let mut sum = Color::black();
        for _ in 0..n {
            sum = sum + trace(&r, 0, &s).1;
        }
        let table = crate::material::diffraction::DiffractionTable::new(0.05);
        let cos = theta.cos();
        let hg = crate::material::phase::henyey_greenstein(cos, 0.5);
        let expected = Color::white() * (0.5 * hg) + table.eval_color(cos, 5.) * 0.5;
        let (got, want) = (sum.rgb.x / sum.rgb.z, expected.rgb.x / expected.rgb.z);
        assert!(sum.rgb.z > 0., "no light");
        assert!((got / want - 1.).abs() < 0.03, "red/blue {} vs {}", got, want);
        assert!((want - 1.).abs() > 0.2, "uninformative angle: red/blue {}", want);
    }

    #[test]
    fn test_absorbing_volume_follows_beer_lambert() {
        // Albedo 0: only light that crosses the 2 units of medium uncollided arrives.
        crate::geometry::seed_rand(1, 2);
        let s = volume_scene(r#""density": 0.6, "albedo": [0, 0, 0]"#);
        let n = 40_000;
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        let mean = (0..n).map(|_| trace(&r, 0, &s).1.rgb.x).sum::<f64>() / n as f64;
        assert!((mean - (-1.2f64).exp()).abs() < 0.01, "{}", mean);
    }

    /// A floor of albedo 0.5, `objects` besides it, and the sun `sun`; black background,
    /// direct light only.
    fn sunlit(sun: &str, objects: &str) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 0, "shadow_bias": 0.0001, "sun": {},
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} {} ]
        }}"#, sun, objects))
    }

    #[test]
    fn test_sun_lights_lambertian_by_cosine() {
        // Irradiance 3 at 60 degrees from the normal: 0.5 / π × 3 × cos 60°.
        let s = sunlit(r#"{ "direction": [0, 1, 1.7320508075688772], "intensity": 3 }"#, "");
        let c = trace_down(&s);
        assert!((c.rgb.x - 0.5 / std::f64::consts::PI * 3. * 0.5).abs() < 1e-9, "{:?}", c);
        // From below the horizon it lights nothing.
        let below = sunlit(r#"{ "direction": [0, -1, 0], "intensity": 3 }"#, "");
        assert_eq!(trace_down(&below).rgb.x, 0.);
    }

    #[test]
    fn test_sun_casts_shadows_but_not_from_the_sky() {
        let ball = r#", { "type": "sphere", "location": [0, 5, 0], "radius": 1, "material": "M" }"#;
        assert_eq!(trace_down(&sunlit(r#"{ "direction": [0, 1, 0] }"#, ball)).rgb.x, 0.);
        // The skysphere reports every ray hitting it, infinitely far away; it mustn't block its
        // own sun.
        let sky = r#", { "type": "skysphere", "sun_direction": [0, 1, 0] }"#;
        assert!(trace_down(&sunlit(r#"{ "intensity": 1 }"#, sky)).rgb.x > 0.1);
    }

    #[test]
    fn test_media_cast_soft_shadows() {
        // An absorbing slab, 0.5 thick and of density 2, between the floor and the sun overhead
        // lets through e^-1 of the light: on average, and in a fraction of each sample rather
        // than all or nothing.
        crate::geometry::seed_rand(4, 4);
        let slab = r#", { "type": "volume", "min": [-50, 3, -50], "max": [50, 3.5, 50], "density": 2, "albedo": [0, 0, 0] }"#;
        let s = sunlit(r#"{ "direction": [0, 1, 0], "intensity": 1 }"#, slab);
        let n = 20_000;
        let v: Vec<f64> = (0..n).map(|_| trace_down(&s).rgb.x).collect();
        let full = 0.5 / std::f64::consts::PI * 0.01f64.cos();
        let mean = v.iter().sum::<f64>() / n as f64;
        assert!((mean / full - (-1f64).exp()).abs() < 0.01, "{}", mean / full);
        assert!(v.iter().any(|x| *x > 0.05 * full && *x < 0.95 * full), "only all-or-nothing shadows");
    }

    #[test]
    fn test_sun_follows_the_sky() {
        let at = |dir: &str| SceneFile::parse_sun(
            &serde_json::json!({ "intensity": 1 }),
            &[serde_json::from_str(&format!(r#"{{ "type": "skysphere", "sun_direction": {} }}"#, dir)).unwrap()],
        ).unwrap();
        let high = at("[0, 1, 0]");
        let low = at("[0, 0.05, 1]");
        assert!((high.direction - Vector3::new(0., 1., 0.)).norm() < 1e-12);
        assert!((low.direction - Vector3::new(0., 0.05, 1.).normalize()).norm() < 1e-12);
        // The atmosphere dims the sun, and dims a low sun more, and its blue most of all.
        let [hr, hg, hb] = [high.irradiance.rgb.x, high.irradiance.rgb.y, high.irradiance.rgb.z];
        let [lr, _, lb] = [low.irradiance.rgb.x, low.irradiance.rgb.y, low.irradiance.rgb.z];
        assert!(hr < 1. && hb < hg && hg < hr, "{:?}", high);
        assert!(lr < hr && lb / lr < hb / hr, "high {:?} low {:?}", high, low);
        assert_eq!(at("[0, -1, 0]").irradiance, Color::black());
    }

    #[test]
    #[should_panic(expected = "follows the skysphere")]
    fn test_sun_direction_conflicts_with_sky() {
        SceneFile::parse_sun(&serde_json::json!({ "direction": [0, 1, 0] }),
            &[serde_json::json!({ "type": "skysphere" })]);
    }

    #[test]
    fn test_single_scattering_of_sunlight_in_a_slab() {
        // Looking straight down into a slab of depth 1, with the sun straight overhead. Light
        // scattered at depth z has crossed z of medium from the sun and crosses z again to the
        // eye; isotropic scattering sends 1/4π of it per steradian back up:
        // L = a E / 4π ∫ σ e^(-2σz) dz = a E (1 - e^(-2σ)) / 8π.
        crate::geometry::seed_rand(5, 1);
        let (sigma, a, e): (f64, f64, f64) = (1.3, 0.8, 2.);
        let s = SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 0, "shadow_bias": 0.0001,
            "sun": {{ "direction": [0, 1, 0], "intensity": {} }},
            "camera": {{ "location": [0, 5, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 }},
            "objects": [ {{ "type": "volume", "min": [-1000, 0, -1000], "max": [1000, 1, 1000],
                           "density": {}, "albedo": [{a}, {a}, {a}] }} ]
        }}"#, e, sigma, a = a));
        let r = Ray { ro: Vector3::new(0., 5., 0.), rd: Vector3::new(0., -1., 0.) };
        let n = 100_000;
        let mean = (0..n).map(|_| trace(&r, 0, &s).1.rgb.x).sum::<f64>() / n as f64;
        let expected = a * e * (1. - (-2. * sigma).exp()) / (8. * std::f64::consts::PI);
        assert!((mean - expected).abs() < expected * 0.02, "{} vs {}", mean, expected);
    }

    /// Mean and standard deviation of n samples through a forward-scattering cube of medium
    /// two units below a 1 x 1 square light, facing down. Paths end only by leaving the cube: at a
    /// depth limit, sampling lights directly counts one more scattering than finding them by
    /// chance does, so the two would differ.
    fn medium_under_light(sample_lights: bool, n: usize) -> (f64, f64) {
        let s = SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 100000, "sample_lights": {},
            "camera": {{ "location": [0, 0.5, -3], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "L": {{ "type": "diffuse-light", "color": [1, 1, 1], "intensity": 1 }} }},
            "objects": [
                {{ "type": "volume", "min": [-0.5, 0, -0.5], "max": [0.5, 0.9, 0.5], "density": 2, "g": 0.5 }},
                {{ "type": "triangle", "v0": [-0.5, 3, -0.5], "v1": [0.5, 3, -0.5], "v2": [0.5, 3, 0.5], "material": "L" }},
                {{ "type": "triangle", "v0": [-0.5, 3, -0.5], "v1": [0.5, 3, 0.5], "v2": [-0.5, 3, 0.5], "material": "L" }}
            ]
        }}"#, sample_lights));
        let r = Ray { ro: Vector3::new(0., 0.4, -3.), rd: Vector3::new(0., 0., 1.) };
        let v: Vec<f64> = (0..n).map(|_| trace(&r, 0, &s).1.rgb.x).collect();
        let mean = v.iter().sum::<f64>() / n as f64;
        let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        (mean, var.sqrt())
    }

    #[test]
    fn test_area_light_sampling_in_media_is_unbiased() {
        crate::geometry::seed_rand(6, 1);
        let n = 60_000;
        let (sampled, sampled_sd) = medium_under_light(true, n);
        let (unsampled, unsampled_sd) = medium_under_light(false, n);
        let se = (sampled_sd.powi(2) / n as f64 + unsampled_sd.powi(2) / n as f64).sqrt();
        assert!((sampled - unsampled).abs() < 4. * se, "{} vs {} (se {})", sampled, unsampled, se);
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
