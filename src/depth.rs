//! # Depth
//!
//! A depth buffer, or z-buffer, records for each pixel the distance to the nearest surface. In
//! a rasteriser it is how visibility is resolved: Catmull's 1974 thesis, "A Subdivision
//! Algorithm for Computer Display of Curved Surfaces" (University of Utah), keeps the nearest
//! depth seen at each pixel. A ray tracer resolves visibility by finding the nearest hit along
//! each ray, so here depth is a by-product, written out for compositing (fog, depth of field in
//! post), for checking a scene's layout, and for debugging geometry.
//!
//! Each pixel gets one ray, through its centre, and records the distance along that ray to the
//! nearest hit: the range, as Mitsuba's depth output does, rather than the distance along the
//! camera's view axis. Rays that hit nothing record no depth. With a non-zero aperture the
//! camera jitters each ray's origin, so depth is noisy at defocused edges.
use crate::color::Color;
use crate::geometry::seed_rand;
use crate::output::LinearImage;
use crate::scene::Scene;
use rayon::prelude::*;

/// The distance to the nearest hit through each pixel's centre, row by row from the top.
pub fn depth(s: &Scene) -> Vec<Option<f64>> {
    let (w, h) = (s.image.width, s.image.height);
    (0 .. w * h).into_par_iter().map(|i| {
        // Image rows run from the top; the camera's y runs from the bottom.
        let (x, y) = (i % w, h - 1 - i / w);
        // Only the aperture draws random numbers; seed per pixel so depth is reproducible.
        seed_rand(s.render.seed, i as u64);
        let r = s.camera.get_ray(x as f64 / w as f64, y as f64 / h as f64, 0.5 / w as f64, 0.5 / h as f64);
        s.objects.nearest_intersection(&r, f64::INFINITY, 0.).map(|hit| hit.dist)
    }).collect()
}

/// Depth as an image. `raw` keeps distances as they are, with misses at infinity, for a
/// floating-point format. Otherwise depth is scaled for viewing: the nearest hit white, the
/// farthest black, misses black.
pub fn depth_image(d: &[Option<f64>], width: usize, height: usize, raw: bool) -> LinearImage {
    let hits = d.iter().flatten();
    let near = hits.clone().cloned().fold(f64::INFINITY, f64::min);
    let far = hits.cloned().fold(0., f64::max);
    let pixels = d.iter().map(|v| {
        let g = match (*v, raw) {
            (Some(z), true) => z,
            (None, true) => f64::INFINITY,
            (Some(z), false) if far > near => 1. - (z - near) / (far - near),
            (Some(_), false) => 1.,
            (None, false) => 0.,
        };
        Color::new(g, g, g)
    }).collect();
    LinearImage { width, height, pixels }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    /// A 5x5 pinhole view of a unit sphere at `centre`, 5 in front of the camera.
    fn sphere_at(centre: [f64; 3]) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 5, "height": 5, "chunk_size": 5, "supersamples": 1, "samples_per_chunk": 1,
            "background": [0, 0, 0], "max_depth": 1,
            "camera": {{ "location": [0, 0, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 1.0, "aperture": 0 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
            "media": {{}}, "lights": [], "variables": {{}},
            "objects": [{{ "type": "sphere", "location": {:?}, "radius": 1, "material": "M" }}]
        }}"#, centre))
    }

    #[test]
    fn test_depth_of_a_sphere() {
        // The centre pixel sees the sphere's near side at 4; the corners see past it.
        let d = depth(&sphere_at([0., 0., 0.]));
        assert!((d[2 * 5 + 2].unwrap() - 4.).abs() < 1e-6, "{:?}", d[12]);
        assert_eq!(d[0], None);

        let raw = depth_image(&d, 5, 5, true);
        assert_eq!(raw.get(2, 2).rgb.x, d[12].unwrap());
        assert_eq!(raw.get(0, 0).rgb.x, f64::INFINITY);
        let view = depth_image(&d, 5, 5, false);
        assert_eq!(view.get(2, 2).rgb.x, 1.);
        assert_eq!(view.get(0, 0).rgb.x, 0.);
    }

    #[test]
    fn test_depth_rows_run_from_the_top() {
        // A sphere below the view's centre appears in the bottom row of the image, as in the
        // render, not the top.
        let d = depth(&sphere_at([0., -2., 0.]));
        assert!(d[4 * 5 + 2].is_some() && d[2].is_none(), "{:?}", d);
    }
}
