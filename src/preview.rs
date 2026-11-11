//! An interactive preview: the same path tracer, driven a few rows at a time by a caller that
//! wants to stay responsive and to restart when the scene is edited.
//!
//! `rendercontext::render` runs a whole image to completion on every thread, which suits the
//! command line. An editor needs something else: render a little, show it, and start again the
//! moment the scene changes. So a `Preview` holds a built scene and renders any rows of any
//! layer on request. A layer is one pass over the image at `samples_per_chunk`^2 samples per
//! pixel; the caller adds layers up and divides by their count.
//!
//! Each (layer, pixel) has its own random stream (see `rendercontext::render_pixel`), so
//! layers can be rendered in any order, by any number of independent `Preview`s of the same
//! scene, and add up to the image the command line gives. `web/` uses this for parallelism:
//! each web worker has its own copy of the scene and renders different layers.
use crate::camera::Camera;
use crate::geometry::seed_rand;
use crate::output::OutputOpts;
use crate::rendercontext::render_pixel;
use crate::scene::Scene;
use crate::scenefile::SceneFile;
use serde_json::Value;

pub struct Preview {
    pub scene: Scene,
}

impl Preview {
    /// Build a scene from scene-file JSON. Panics, as the scene file parser does, if the
    /// description is invalid.
    pub fn new(json: &str) -> Preview {
        Preview { scene: SceneFile::from_string(json.to_string()) }
    }

    pub fn width(&self) -> usize { self.scene.image.width }
    pub fn height(&self) -> usize { self.scene.image.height }

    /// Samples each layer adds to every pixel.
    pub fn samples_per_layer(&self) -> usize { self.scene.render.samples_per_layer() }

    /// Replace the camera, from the scene file's `camera` object, keeping everything else.
    /// Moving the view is the most frequent edit, and this avoids rebuilding the scene for it.
    pub fn set_camera(&mut self, camera_json: &str) {
        let c: Value = serde_json::from_str(camera_json).unwrap_or_else(|e| panic!("camera: {}", e));
        let (w, h) = (self.width() as u32, self.height() as u32);
        self.scene.camera = Box::new(SceneFile::parse_camera(c, w, h));
    }

    /// Render image rows `y0 .. y1` (row 0 is the top) of one layer, writing the linear sum of
    /// each pixel's samples as three `f32`s. `out` holds `(y1 - y0) * width * 3` values.
    /// Returns the number of rays cast.
    pub fn render_rows(&self, layer: usize, y0: usize, y1: usize, out: &mut [f32]) -> u64 {
        let (w, h) = (self.width(), self.height());
        let spc = self.scene.render.samples_per_chunk.max(1);
        let mut cast = 0;
        for (row, y) in (y0 .. y1.min(h)).enumerate() {
            for x in 0 .. w {
                // The renderer's rows run from the bottom; images' from the top.
                let (c, _, sum) = render_pixel(x, h - 1 - y, layer, spc, &self.scene);
                let sum = sum.ignore_nan();
                let i = (row * w + x) * 3;
                out[i] = sum.rgb.x as f32;
                out[i + 1] = sum.rgb.y as f32;
                out[i + 2] = sum.rgb.z as f32;
                cast += c;
            }
        }
        cast
    }

    /// Render rows `y0 .. y1` of a draft image, `cols` by `rows` pixels whatever the scene's
    /// own size, with `trace::draft`: one unbounced ray per pixel. `out` takes three `f32`s
    /// per pixel. Fast enough to follow the camera while it is being dragged.
    pub fn render_draft(&self, cols: usize, rows: usize, y0: usize, y1: usize, out: &mut [f32]) {
        let (w, h) = (cols as f64, rows as f64);
        for (row, y) in (y0 .. y1.min(rows)).enumerate() {
            for x in 0 .. cols {
                // The same numbers for a pixel every time, so a still camera gives a still image.
                seed_rand(0, (y * cols + x) as u64);
                let r = self.scene.camera.get_ray(x as f64 / w, (rows - 1 - y) as f64 / h, 0.5 / w, 0.5 / h);
                let c = crate::trace::draft(&r, &self.scene).ignore_nan();
                let i = (row * cols + x) * 3;
                out[i] = c.rgb.x as f32;
                out[i + 1] = c.rgb.y as f32;
                out[i + 2] = c.rgb.z as f32;
            }
        }
    }

    /// The object seen at image position (x, y), in pixels from the top left: its index in the
    /// scene file's `objects` list. None for the background, or an object with no entry of its
    /// own.
    pub fn pick(&self, x: f64, y: f64) -> Option<usize> {
        let (w, h) = (self.width() as f64, self.height() as f64);
        // The lens jitters ray origins; a fixed seed makes the same click pick the same thing.
        seed_rand(0, 0);
        let r = self.scene.camera.get_ray(x.floor() / w, (h - 1. - y.floor()) / h, 0.5 / w, 0.5 / h);
        let hit = self.scene.objects.nearest_intersection(&r, f64::INFINITY, 0.)?;
        self.scene.source_index(hit.object)
    }
}

/// Turn summed linear samples into 8-bit RGBA for display, as the PNG writer would encode them.
/// `sums` holds three values per pixel, `out` four.
pub fn encode_rgba(sums: &[f32], samples: f64, opts: &OutputOpts, out: &mut [u8]) {
    let scale = 1. / samples.max(1.);
    for (px, o) in sums.chunks_exact(3).zip(out.chunks_exact_mut(4)) {
        let c = crate::color::Color::new(px[0] as f64 * scale, px[1] as f64 * scale, px[2] as f64 * scale);
        let [r, g, b] = opts.quantise(c);
        o.copy_from_slice(&[r, g, b, 255]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendercontext::{RenderContext, render};
    use std::sync::Mutex;

    const SCENE: &str = r#"{
        "width": 12, "height": 8, "chunk_size": 4, "supersamples": 3, "samples_per_chunk": 1,
        "seed": 7, "background": [0.3, 0.4, 0.5], "max_depth": 4,
        "camera": { "location": [0, 1, -5], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.5, "aperture": 0.1 },
        "materials": {
            "D": { "type": "lambertian", "albedo": [0.8, 0.8, 0.8] },
            "M": { "type": "metal", "reflective": [0.9, 0.9, 0.9], "roughness": 0.3 }
        },
        "lights": [ { "location": [0, 5, -2], "intensity": 100 } ],
        "objects": [
            { "type": "skysphere" },
            { "type": "plane", "y": 0, "material": "D" },
            { "type": "sphere", "location": [0, 1, 0], "radius": 1, "material": "M" }
        ]
    }"#;

    /// Layers rendered through the preview, in rows and out of order, add up to exactly the
    /// image the command line's renderer gives.
    #[test]
    fn test_preview_layers_match_the_full_render() {
        let p = Preview::new(SCENE);
        let (w, h) = (p.width(), p.height());
        let mut layers = vec![vec![0f32; w * h * 3]; 3];
        for layer in [2, 0, 1] {
            for y in (0 .. h).step_by(3) {
                let y1 = (y + 3).min(h);
                p.render_rows(layer, y, y1, &mut layers[layer][y * w * 3 .. y1 * w * 3]);
            }
        }
        let rc = Mutex::new(RenderContext::new(w, h, false, "t.json"));
        render(&p.scene, &rc, |_| None::<()>, |_| ());
        let img = rc.lock().unwrap().snapshot();
        for (i, px) in img.pixels.iter().enumerate() {
            for ch in 0 .. 3 {
                let sum: f64 = (0 .. 3).map(|l| layers[l][i * 3 + ch] as f64).sum();
                assert!((sum / 3. - px.rgb[ch]).abs() < 1e-5, "pixel {} channel {}: {} vs {}", i, ch, sum / 3., px.rgb[ch]);
            }
        }
    }

    #[test]
    fn test_pick_returns_the_scene_file_index() {
        let mut p = Preview::new(SCENE);
        // The sky is built as an object of its own kind; the sphere is entry 2 whatever came
        // before it.
        assert_eq!(p.pick(6., 3.), Some(2));
        assert_eq!(p.pick(6., 7.), Some(1));
        p.set_camera(r#"{ "location": [0, 1, 5], "lookat": [0, 20, 5.1], "up": [0, 1, 0], "angle": 0.5 }"#);
        assert_eq!(p.pick(6., 3.), Some(0));
    }

    #[test]
    fn test_draft_shows_the_scene_at_any_size() {
        let p = Preview::new(SCENE);
        let (cols, rows) = (6, 4);
        let mut out = vec![0f32; cols * rows * 3];
        p.render_draft(cols, rows, 0, 2, &mut out[.. cols * 2 * 3]);
        p.render_draft(cols, rows, 2, 4, &mut out[cols * 2 * 3 ..]);
        let again = { let mut o = vec![0f32; cols * rows * 3]; p.render_draft(cols, rows, 0, rows, &mut o); o };
        assert_eq!(out, again);
        let px = |x: usize, y: usize| &out[(y * cols + x) * 3 .. (y * cols + x) * 3 + 3];
        // Sky at the top, lit ground at the bottom, and they differ.
        assert!(px(0, 0).iter().all(|v| *v > 0.) && px(0, 3).iter().all(|v| *v > 0.));
        assert_ne!(px(0, 0), px(0, 3));
    }

    #[test]
    fn test_encode_averages_and_encodes() {
        let mut out = [0u8; 8];
        encode_rgba(&[2., 0., 0.5, 0., 4., 0.], 2., &OutputOpts::default(), &mut out);
        assert_eq!(out[0], 255);
        assert_eq!(out[1], 0);
        assert_eq!(out[3], 255);
        assert_eq!(out[5], 255);
        assert_eq!(out[2], OutputOpts::default().quantise(crate::color::Color::new(0.25, 0., 0.))[0]);
    }
}
