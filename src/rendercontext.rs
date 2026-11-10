use std::time::{Instant};

use crate::scene::Scene;
use crate::color::Color;
use crate::trace::trace;
use crate::geometry::{rand, seed_rand};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use rayon::prelude::*;
use crate::output::LinearImage;


// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    samples: Vec<usize>,
    pub width: usize,
    pub height: usize,
    pub rays_cast: u64,
    pub start_time: Instant,
    pub progressive_render: bool,
    pub samples_rendered: u64,
    pub output_filename: String,
}

pub struct RenderIterator {
    i: usize,
    pub layer: usize,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub chunk_size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderableChunk {
    pub xmin: usize,
    pub xmax: usize,
    pub ymin: usize,
    pub ymax: usize,
    pub supersamples: usize,
    /// Which pass over the image this chunk belongs to.
    pub layer: usize,
}

pub struct RenderedChunk {
    pixels: Vec<Color>,
    samples: Vec<usize>,
    rays_cast: u64,
}


fn format_f64(v: f64) -> String {
    if v > 1000000. {
        format!("{:.2}M", v / 1000000.)
    } else if v > 1000. {
        format!("{:.2}K", v / 1000.)
    } else {
        format!("{:.2}", v)
    }
}

/// The image is written next to the scene file, with a .png extension.
pub fn output_filename_for(scene: &str) -> String {
    std::path::Path::new(scene).with_extension("png").to_string_lossy().into_owned()
}

impl RenderContext {
    pub fn new(width:usize, height:usize, progressive_render: bool, filename: &str) -> RenderContext {
        let start_time = Instant::now();
        let output_filename = output_filename_for(filename);
        RenderContext {
            image: vec![Color::black(); width*height],
            samples: vec![0; width*height],
            width,
            height,
            rays_cast: 0,
            start_time,
            progressive_render,
            samples_rendered: 0,
            output_filename,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c:Color, samples: usize) {
        if x >= self.width || y.saturating_mul(self.width).saturating_add(x) >= self.width * self.height {
            return;
        }

        let i:usize = y*self.width + x;
        self.image[i] = self.image[i] + c.ignore_nan();
        self.samples[i] += samples;
        self.samples_rendered += samples as u64;
    }

    pub fn apply_chunk(&mut self, c: &RenderableChunk, p: &RenderedChunk){
        let mut i = 0;
        for y in c.ymin .. c.ymax {
            for x in c.xmin .. c.xmax {
                self.set_pixel(x, y, p.pixels[i], p.samples[i]);
                i += 1;
            }
        }
        self.rays_cast += p.rays_cast;
    }

    /// The current estimate as a linear image, top row first.
    pub fn snapshot(&self) -> LinearImage {
        let mut pixels = Vec::with_capacity(self.width * self.height);
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                pixels.push(self.get_pixel(x, self.height - y - 1));
            }
        }
        LinearImage { width: self.width, height: self.height, pixels }
    }

    pub fn get_pixel(&self, x:usize, y:usize) -> Color {
        let i = y*self.width + x; 
        self.image[i] / self.samples[i].max(1) as f64
    }
/*
    pub fn get_pixel_array(&self) -> Vec<u8> {
        let len = (self.width * self.height) as usize;
        let mut out: Vec<u8> = vec![0; len * 3];

        for i in  0 .. len {
            let (r, g, b) = self.image[i].to_u8();
            out.push(r);
            out.push(g);
            out.push(b);
            print!("{} {} {} {} {} \n", i, self.image[i], r, g, b);
        }

        return out;
    }
    */
    pub fn print_stats(&self) {
        let elapsed = Instant::now() - self.start_time;

        print!("\n==========================================\n");
        println!("| Rays Cast: {}", self.rays_cast);
        println!("| Elapsed Time: {:?}", elapsed);
        println!("| Rays per sec: {:.2}", self.rays_cast as f64 / elapsed.as_secs_f64());
        println!("==========================================");

    }

    
    pub fn progress(&self, s: &Scene) -> String {
        let elapsed = Instant::now() - self.start_time;
        format!("{:.2}s {} rays cast ({} RPS), {} Rays per sample, {}%, {} threads",
                 elapsed.as_secs_f64(),
                 format_f64(self.rays_cast as f64),
                 format_f64(self.rays_cast as f64 / elapsed.as_secs_f64()),
                 format_f64(self.rays_cast as f64 / self.samples_rendered as f64),
                 format_f64(self.progress_percentage(s)),
                 rayon::current_num_threads())
    }

    pub fn progress_percentage(&self, s: &Scene) -> f64 {
        (self.samples_rendered as f64 / (self.width * self.height * s.render.samples_per_pixel()) as f64) * 100.
    }

    pub fn iter(&self, s: &Scene) -> impl Iterator<Item=RenderableChunk> + '_ {
        let width = self.width;
        let height = self.height;
        let chunk_size = s.render.chunk_size;
        let chunk_layers = s.render.layers();
        let samples = s.render.samples_per_chunk.max(1);
        (0 .. chunk_layers)
                    .flat_map(move |layer| 
                        RenderIterator {
                            i: 0,
                            layer,
                            width,
                            height,
                            chunk_size,
                            samples,
                        })
    }
}

/// Render every chunk of the image, all threads working at once, with output that depends only
/// on the scene and its seed.
///
/// Each pixel's samples are already independent of scheduling (see `render_pixel`), but the
/// order in which a pixel's layers are added up still matters: floating-point addition isn't
/// associative. So chunks are handed out in order, layer by layer, and a chunk that finishes
/// before the previous layer of the same region has been applied is held back until it has.
/// Because chunks start in order, only a few (about one per thread) are ever held.
///
/// After each chunk is applied, `after_chunk` runs with the context locked; anything it returns
/// is passed to `outside_lock`, eg. to write a progressive image without holding up the render.
pub fn render<T, F, G>(s: &Scene, rc: &Mutex<RenderContext>, after_chunk: F, outside_lock: G)
where
    F: Fn(&RenderContext) -> Option<T> + Sync,
    G: Fn(T) + Sync,
{
    let chunks: Vec<RenderableChunk> = rc.lock().unwrap().iter(s).collect();
    if chunks.is_empty() {
        return;
    }
    // Every layer covers the image with the same regions, in the same order.
    let regions = chunks.len() / s.render.layers();
    let next_chunk = AtomicUsize::new(0);
    // The next layer due for each region, and finished chunks waiting for their turn.
    let order = Mutex::new((vec![0usize; regions], HashMap::<usize, RenderedChunk>::new()));

    (0 .. rayon::current_num_threads()).into_par_iter().for_each(|_| loop {
        let i = next_chunk.fetch_add(1, Ordering::Relaxed);
        if i >= chunks.len() {
            break;
        }
        let rendered = chunks[i].render(s);

        let pending = {
            let mut ctx = rc.lock().unwrap();
            let mut order = order.lock().unwrap();
            let (next_layer, waiting) = &mut *order;
            waiting.insert(i, rendered);
            let region = i % regions;
            let mut applied = false;
            while let Some(r) = waiting.remove(&(next_layer[region] * regions + region)) {
                ctx.apply_chunk(&chunks[next_layer[region] * regions + region], &r);
                next_layer[region] += 1;
                applied = true;
            }
            if applied { after_chunk(&ctx) } else { None }
        };
        if let Some(t) = pending {
            outside_lock(t);
        }
    });
}

impl RenderableChunk {
    pub fn width(&self) -> usize {
        self.xmax - self.xmin
    }

    pub fn render(&self, s: &Scene) -> RenderedChunk {
        let size = self.width() * (self.ymax - self.ymin);
        let mut pixels: Vec<Color> = Vec::with_capacity(size);
        let mut samples: Vec<usize> = Vec::with_capacity(size);
        let mut rays_cast = 0;
        // Pixels are rendered in parallel too, so a layer with few chunks of uneven cost (eg. sky
        // and a horizon of grazing rays) still keeps every thread busy. Results are collected in
        // order, so they are the same however the work is scheduled.
        let w = self.width();
        let results: Vec<(u64, usize, Color)> = (0 .. size).into_par_iter()
            .map(|i| render_pixel(self.xmin + i % w, self.ymin + i / w, self.layer, self.supersamples, s))
            .collect();
        for (cast, psamples, pixel) in results {
            pixels.push(pixel);
            samples.push(psamples);
            rays_cast += cast;
        }

        RenderedChunk {
            pixels, samples, rays_cast
        }
    }   
}

impl Iterator for RenderIterator {
    type Item = RenderableChunk;

    fn next(&mut self) -> Option<RenderableChunk> {
        if self.i >= self.width * self.height {
            return None
        }

        // From i (pixel index) find current chunk
        let y = self.i / self.width;
        let x = self.i % self.width;

        if self.height - y > self.chunk_size {
            if self.width - x > self.chunk_size {
                self.i += self.chunk_size;
                Some(RenderableChunk {
                    xmin: x, 
                    xmax: x + self.chunk_size,
                    ymin: y,
                    ymax: y + self.chunk_size,
                    supersamples: self.samples,
                layer: self.layer,
                })
            } else {
                // Increment down a row
                self.i = (self.i - x) + (self.width * self.chunk_size);
                // return remainder of x
                Some(RenderableChunk {
                    xmin: x ,
                    xmax: self.width,
                    ymin: y,
                    ymax: y + self.chunk_size,
                    supersamples: self.samples,
                layer: self.layer,
                })
            }
        } else if self.width - x > self.chunk_size {
            self.i += self.chunk_size;
            Some(RenderableChunk {
                xmin: x, 
                xmax: x + self.chunk_size,
                ymin: y,
                ymax: self.height,
                supersamples: self.samples,
                layer: self.layer,
            })
        } else {
            self.i = (self.i - x) + self.chunk_size * self.width;
            Some(RenderableChunk {
                xmin: x ,
                xmax: self.width,
                ymin: y,
                ymax: self.height,
                supersamples: self.samples,
                layer: self.layer,
            })
        }
    }
}

pub(crate) fn render_pixel(x: usize, y: usize, layer: usize, max_samples: usize, s: &Scene) -> (u64, usize, Color) {
    // Every pixel and layer has its own random stream, so a render depends only on the seed,
    // not on which thread renders which chunk.
    let stream = ((layer * s.image.height + y) * s.image.width + x) as u64;
    seed_rand(s.render.seed, stream);

    let mut pixel = Color::black();
    let mut cast = 0;
    let mut samples = 0;

    // Monte-Carlo method: We sample many times and average.
    for sx in 0..max_samples {
        for sy in 0..max_samples {
            let (rays_cast, c) = trace(
                    &s.camera.get_ray(
                        x as f64 / (s.image.width as f64),
                        y as f64 / (s.image.height as f64),
                        // Stratified jitter, so repeated layers sample new positions, warped
                        // to the reconstruction filter, then measured from the pixel's corner.
                        (0.5 + s.render.filter.sample((sx as f64 + rand()) / (max_samples as f64))) / (s.image.width as f64),
                        (0.5 + s.render.filter.sample((sy as f64 + rand()) / (max_samples as f64))) / (s.image.height as f64))
                    , 0, s);
            cast += rays_cast;
            pixel = pixel + c;
            samples += 1;
        }
    }
    (cast, samples, pixel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    fn scene(supersamples: usize, samples_per_chunk: usize) -> Scene {
        scene_chunked(supersamples, samples_per_chunk, 2)
    }

    fn scene_chunked(supersamples: usize, samples_per_chunk: usize, chunk_size: usize) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 5, "height": 3, "chunk_size": {},
            "supersamples": {}, "samples_per_chunk": {},
            "shadow_bias": 0.0001, "background": [0, 0, 0], "max_depth": 1,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{}}, "media": {{}}, "lights": [], "objects": [], "variables": {{}}
        }}"#, chunk_size, supersamples, samples_per_chunk))
    }

    fn render(s: &Scene) -> RenderContext {
        let mut rc = RenderContext::new(s.image.width, s.image.height, false, "test.json");
        let chunks: Vec<RenderableChunk> = rc.iter(s).collect();
        for c in chunks {
            let p = c.render(s);
            rc.apply_chunk(&c, &p);
        }
        rc
    }

    /// Render on all threads through `render`, as main does.
    fn render_parallel(seed: u64) -> Vec<Color> {
        let s = SceneFile::from_string(format!(r#"{{
            "width": 24, "height": 16, "chunk_size": 2, "supersamples": 32, "samples_per_chunk": 1,
            "seed": {}, "background": [0.3, 0.4, 0.5], "max_depth": 6,
            "path_termination": "russian_roulette", "russian_roulette_depth": 1,
            "camera": {{ "location": [0, 1, -5], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.5, "aperture": 0.1 }},
            "materials": {{
                "D": {{ "type": "lambertian", "albedo": [0.8, 0.8, 0.8] }},
                "M": {{ "type": "metal", "reflective": [0.9, 0.9, 0.9], "roughness": 0.3 }},
                "G": {{ "type": "dielectric", "refractive_index": 1.5, "attenuate": [1, 1, 1] }}
            }},
            "lights": [ {{ "location": [0, 5, -2], "intensity": 100 }} ],
            "objects": [
                {{ "type": "plane", "y": 0, "material": "D" }},
                {{ "type": "sphere", "location": [-1, 0.5, 0], "radius": 0.5, "material": "M" }},
                {{ "type": "sphere", "location": [1, 0.5, 0], "radius": 0.5, "material": "G" }},
                {{ "type": "fog", "density": 0.02 }}
            ]
        }}"#, seed));
        let rc = Mutex::new(RenderContext::new(s.image.width, s.image.height, false, "test.json"));
        super::render(&s, &rc, |_| None::<()>, |_| ());
        let img = rc.lock().unwrap().snapshot();
        img.pixels
    }

    #[test]
    fn test_same_seed_renders_identically() {
        let a = render_parallel(1);
        assert_eq!(a, render_parallel(1), "same seed, different image");
        assert_ne!(a, render_parallel(2), "different seeds, same image");
    }

    #[test]
    fn test_zero_chunk_size_terminates() {
        let s = scene_chunked(1, 1, 0);
        let rc = render(&s);
        assert!(rc.samples.iter().all(|n| *n == 1));
    }

    #[test]
    fn test_output_filename() {
        assert_eq!(output_filename_for("demo/demo.json"), "demo/demo.png");
        assert_eq!(output_filename_for("scene"), "scene.png");
        assert_eq!(output_filename_for("scene.JSON"), "scene.png");
        assert_eq!(output_filename_for("a/x.json.d/s.json"), "a/x.json.d/s.png");
    }

    #[test]
    fn test_unwritable_output_is_an_error() {
        let s = scene(1, 1);
        let rc = render(&s);
        let w = crate::output::writer_for("out.png", s.output).unwrap();
        assert!(w.write(&rc.snapshot(), "/nonexistent-dir-for-rays-test/out.png").is_err());
    }

    #[test]
    fn test_samples_per_pixel_meets_supersamples() {
        for (supersamples, per_chunk) in [(35, 2), (1, 2), (8, 1), (9, 3), (4, 0)] {
            let s = scene(supersamples, per_chunk);
            let rc = render(&s);
            let expected = s.render.samples_per_pixel();
            assert!(expected >= supersamples, "{} < {}", expected, supersamples);
            for i in 0 .. s.image.width * s.image.height {
                assert_eq!(rc.samples[i], expected, "pixel {} for {:?}", i, (supersamples, per_chunk));
            }
            assert!((rc.progress_percentage(&s) - 100.).abs() < 1e-9);
        }
    }
}
