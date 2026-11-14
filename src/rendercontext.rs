use std::time::{Instant};

use crate::scene::Scene;
use crate::color::Color;
use crate::trace::trace;
use crate::geometry::{rand, seed_rand, begin_sample, end_sample};
use crate::scene::Sampler;
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
    /// Samples carried over from a saved state (see `load_state`).
    pub resumed_samples: u64,
}

/// A render's saved state begins with this, then the key of what it is a render of, its width
/// and height, then each pixel's summed colour (three f64s) and number of samples (a u64), all
/// little-endian. The sums are kept exactly, so a resumed render is the same, bit for bit, as
/// one that ran straight through.
const STATE_MAGIC: &[u8; 8] = b"RAYSACC2";

/// What a render's samples depend on, besides how many there are: a hash of the scene file's
/// contents (all but its `supersamples`, so more can be asked for later) and of the overrides
/// of its size and seed. A saved state is resumed only for the same key, so editing the scene
/// starts the render afresh. Files the scene refers to (meshes, cached simulations) and the
/// renderer itself aren't covered: after changing those, render with --fresh.
pub fn render_key(scene: &str, overrides: (Option<usize>, Option<usize>, Option<u64>)) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    match serde_json::from_str::<serde_json::Value>(scene) {
        Ok(mut v) => {
            if let Some(o) = v.as_object_mut() {
                o.remove("supersamples");
            }
            v.to_string().hash(&mut h);
        },
        Err(_) => scene.hash(&mut h),
    }
    overrides.hash(&mut h);
    h.finish()
}

/// Where a render's state is saved: alongside its image, as `<image>.accum`.
pub fn state_path_for(output: &str) -> String {
    format!("{}.accum", output)
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
    /// The samples each pixel takes: `samples` of them, from number `first_sample`.
    pub first_sample: usize,
    pub samples: usize,
    /// Which region of the image this is, and which of that region's passes.
    pub region: usize,
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
            resumed_samples: 0,
        }
    }

    /// The fewest samples any pixel has.
    pub fn min_samples(&self) -> usize {
        self.samples.iter().copied().min().unwrap_or(0)
    }

    /// The accumulated samples, encoded as a state file for a render identified by `key`.
    pub fn encode_state(&self, key: u64) -> Vec<u8> {
        let mut out = Vec::with_capacity(32 + self.image.len() * 32);
        out.extend_from_slice(STATE_MAGIC);
        for v in [key, self.width as u64, self.height as u64] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for (c, n) in self.image.iter().zip(&self.samples) {
            for v in [c.rgb.x, c.rgb.y, c.rgb.z] {
                out.extend_from_slice(&v.to_le_bytes());
            }
            out.extend_from_slice(&(*n as u64).to_le_bytes());
        }
        out
    }

    /// Carry on from a saved state, if it is of this render (`key`, and the same size).
    pub fn load_state(&mut self, path: &str, key: u64) -> Result<(), String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {}", path, e))?;
        let pixels = self.width * self.height;
        if bytes.len() != 32 + pixels * 32 || &bytes[..8] != STATE_MAGIC {
            return Err(format!("{} is not a saved render of this size", path));
        }
        let u64_at = |i: usize| u64::from_le_bytes(bytes[i..i + 8].try_into().unwrap());
        let f64_at = |i: usize| f64::from_le_bytes(bytes[i..i + 8].try_into().unwrap());
        if u64_at(8) != key || u64_at(16) != self.width as u64 || u64_at(24) != self.height as u64 {
            return Err(format!("{} is of a different scene or settings", path));
        }
        for i in 0..pixels {
            let o = 32 + i * 32;
            self.image[i] = Color::new(f64_at(o), f64_at(o + 8), f64_at(o + 16));
            self.samples[i] = u64_at(o + 24) as usize;
        }
        self.resumed_samples = self.samples.iter().map(|n| *n as u64).sum();
        Ok(())
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
        ((self.resumed_samples + self.samples_rendered) as f64 / (self.width * self.height * s.render.samples_per_pixel()) as f64) * 100.
    }

    /// The chunks still to render, pass by pass: each region of the image takes the samples
    /// from those it already has up to the scene's `supersamples`, `samples_per_layer` at a
    /// time.
    pub fn iter(&self, s: &Scene) -> impl Iterator<Item=RenderableChunk> + '_ {
        let regions: Vec<RenderableChunk> = RenderIterator {
            i: 0, layer: 0, width: self.width, height: self.height, chunk_size: s.render.chunk_size, samples: 0,
        }.collect();
        let target = s.render.samples_per_pixel();
        let per_layer = s.render.samples_per_layer();
        // Every pixel in a region has the same number of samples.
        let have: Vec<usize> = regions.iter().map(|c| self.samples[c.ymin * self.width + c.xmin]).collect();
        let layers = (0..regions.len()).map(|r| target.saturating_sub(have[r]).div_ceil(per_layer)).max().unwrap_or(0);
        (0..layers).flat_map(move |layer| {
            let regions = regions.clone();
            let have = have.clone();
            (0..regions.len()).filter_map(move |r| {
                let first = have[r] + layer * per_layer;
                (first < target).then(|| RenderableChunk {
                    first_sample: first,
                    samples: per_layer.min(target - first),
                    region: r,
                    layer,
                    ..regions[r]
                })
            })
        }).collect::<Vec<_>>().into_iter()
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
    // Each region's chunks, in order: a region's passes must be added up in turn.
    let regions = chunks.iter().map(|c| c.region).max().unwrap() + 1;
    let mut by_region = vec![vec![]; regions];
    for (i, c) in chunks.iter().enumerate() {
        by_region[c.region].push(i);
    }
    let next_chunk = AtomicUsize::new(0);
    // The next pass due for each region, and finished chunks waiting for their turn.
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
            let region = chunks[i].region;
            let mut applied = false;
            while let Some(&due) = by_region[region].get(next_layer[region]) {
                let Some(r) = waiting.remove(&due) else { break };
                ctx.apply_chunk(&chunks[due], &r);
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
            .map(|i| render_pixel(self.xmin + i % w, self.ymin + i / w, self.first_sample, self.samples, s))
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
                    first_sample: 0,
                    samples: self.samples,
                    region: 0,
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
                    first_sample: 0,
                    samples: self.samples,
                    region: 0,
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
                first_sample: 0,
                samples: self.samples,
                region: 0,
                layer: self.layer,
            })
        } else {
            self.i = (self.i - x) + self.chunk_size * self.width;
            Some(RenderableChunk {
                xmin: x ,
                xmax: self.width,
                ymin: y,
                ymax: self.height,
                first_sample: 0,
                samples: self.samples,
                region: 0,
                layer: self.layer,
            })
        }
    }
}

/// The R2 sequence's steps (M. Roberts, "The unreasonable effectiveness of quasirandom
/// sequences", 2018): sample k of a pixel is jittered to (k a1, k a2) mod 1, offset by a random
/// amount per pixel. Any number of samples is well spread over the pixel, and more can be added
/// later, which a grid fixed in advance can't do.
const R2: (f64, f64) = (0.754_877_666_246_692_7, 0.569_840_290_998_053_2);

/// Samples `first` to `first + count` of pixel (x, y). Every sample has its own random stream,
/// so a render depends only on the seed and how many samples it takes, not on which thread
/// renders which chunk, nor on how the samples were split between passes or runs.
fn render_pixel(x: usize, y: usize, first: usize, count: usize, s: &Scene) -> (u64, usize, Color) {
    let pixels = (s.image.width * s.image.height) as u64;
    let index = (y * s.image.width + x) as u64;
    // This pixel's offset of the sequence.
    seed_rand(s.render.seed ^ 0x9e37_79b9_7f4a_7c15, index);
    let offset = (rand(), rand());

    let mut pixel = Color::black();
    let mut cast = 0;
    // With the Sobol' sampler, sample k of the pixel is point k of the pixel's own sequence.
    let pixel_seed = crate::sobol::hash(s.render.seed, index);
    for k in first..first + count {
        seed_rand(s.render.seed, k as u64 * pixels + index);
        // Where in the pixel, as a uniform in [0, 1)²: the Sobol' point's first two dimensions,
        // or the R2 sequence from the pixel's offset.
        let (u, v) = match s.render.sampler {
            Sampler::Sobol => {
                begin_sample(pixel_seed, k as u32);
                (rand(), rand())
            },
            Sampler::Independent => ((offset.0 + k as f64 * R2.0).fract(), (offset.1 + k as f64 * R2.1).fract()),
        };
        // In spectral mode, the sample's wavelengths (`spectrum`).
        if s.render.spectral {
            crate::spectrum::begin_sample(crate::spectrum::Wavelengths::sample_visible(rand()));
        }
        let (rays_cast, c) = trace(
                &s.camera.get_ray(
                    x as f64 / (s.image.width as f64),
                    y as f64 / (s.image.height as f64),
                    // Jitter warped to the reconstruction filter, measured from the pixel's
                    // corner.
                    (0.5 + s.render.filter.sample(u)) / (s.image.width as f64),
                    (0.5 + s.render.filter.sample(v)) / (s.image.height as f64))
                , 0, s);
        cast += rays_cast;
        // A spectral sample's radiance becomes a colour (and stray NaNs, as in RGB, pass on).
        let c = match crate::spectrum::end_sample() {
            Some(w) => w.to_srgb(&c),
            None => c,
        };
        pixel = pixel + c;
    }
    end_sample();
    (cast, count, pixel)
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
        render_from(seed, 32, None).snapshot().pixels
    }

    /// As `render_parallel`, to `supersamples`, carrying on from a saved state.
    fn render_from(seed: u64, supersamples: usize, state: Option<&[u8]>) -> RenderContext {
        let s = SceneFile::from_string(format!(r#"{{
            "width": 24, "height": 16, "chunk_size": 2, "supersamples": {}, "samples_per_chunk": 1,
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
        }}"#, supersamples, seed));
        let mut ctx = RenderContext::new(s.image.width, s.image.height, false, "test.json");
        if let Some(bytes) = state {
            let path = std::env::temp_dir().join(format!("rays-test-state-{}-{}.accum", seed, std::process::id()));
            std::fs::write(&path, bytes).unwrap();
            ctx.load_state(path.to_str().unwrap(), 7).unwrap();
            std::fs::remove_file(&path).unwrap();
        }
        let rc = Mutex::new(ctx);
        super::render(&s, &rc, |_| None::<()>, |_| ());
        rc.into_inner().unwrap()
    }

    #[test]
    fn test_resumed_render_is_identical() {
        let straight = render_from(3, 32, None);
        // Stopped at 12 samples, saved, and carried on to 32: the same image, bit for bit.
        let part = render_from(3, 12, None);
        assert_eq!(part.min_samples(), 12);
        let state = part.encode_state(7);
        let resumed = render_from(3, 32, Some(&state));
        assert_eq!(resumed.resumed_samples, 12 * 24 * 16);
        assert_eq!(resumed.snapshot().pixels, straight.snapshot().pixels);
        assert_eq!(resumed.samples, straight.samples);
        // Stopped at an odd count, part way through a pass: the same again.
        let odd = render_from(3, 5, None).encode_state(7);
        assert_eq!(render_from(3, 32, Some(&odd)).snapshot().pixels, straight.snapshot().pixels);
        // Already there: nothing more to do.
        let done = render_from(3, 12, Some(&state));
        assert_eq!(done.samples_rendered, 0);
        assert_eq!(done.snapshot().pixels, part.snapshot().pixels);
    }

    #[test]
    fn test_render_key_follows_the_scene_but_not_its_samples() {
        let a = r#"{"width": 10, "supersamples": 16, "camera": {"angle": 0.5}}"#;
        let more = r#"{"width": 10, "supersamples": 64, "camera": {"angle": 0.5}}"#;
        let edited = r#"{"width": 10, "supersamples": 16, "camera": {"angle": 0.6}}"#;
        let none = (None, None, None);
        assert_eq!(render_key(a, none), render_key(more, none));
        assert_ne!(render_key(a, none), render_key(edited, none));
        assert_ne!(render_key(a, none), render_key(a, (Some(20), None, None)));
        assert_ne!(render_key(a, none), render_key(a, (None, None, Some(1))));
    }

    #[test]
    fn test_state_of_another_render_is_refused() {
        let part = render_from(3, 4, None);
        let path = std::env::temp_dir().join(format!("rays-test-refuse-{}.accum", std::process::id()));
        std::fs::write(&path, part.encode_state(7)).unwrap();
        let mut other = RenderContext::new(24, 16, false, "test.json");
        assert!(other.load_state(path.to_str().unwrap(), 8).is_err());
        let mut smaller = RenderContext::new(12, 16, false, "test.json");
        assert!(smaller.load_state(path.to_str().unwrap(), 7).is_err());
        std::fs::remove_file(&path).unwrap();
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
            assert_eq!(expected, supersamples.max(1));
            for i in 0 .. s.image.width * s.image.height {
                assert_eq!(rc.samples[i], expected, "pixel {} for {:?}", i, (supersamples, per_chunk));
            }
            assert!((rc.progress_percentage(&s) - 100.).abs() < 1e-9);
        }
    }
}
