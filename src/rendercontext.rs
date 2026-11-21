use web_time::Instant;

use crate::scene::Scene;
use crate::color::Color;
use crate::trace::trace;
use crate::geometry::{rand, seed_rand, begin_sample, begin_sampler, end_sample};
use crate::scene::Sampler;
use std::sync::Mutex;
use std::collections::BTreeMap;
use rayon::prelude::*;
use crate::output::LinearImage;


// The render context is the data structure
// that holds state about the current render.
// 
// It needs to be thread safe.
pub struct RenderContext {
    image: Vec<Color>,
    /// Each pixel's sum of its samples' squares, channel by channel, for their variance
    /// (`adaptive`).
    squares: Vec<Color>,
    samples: Vec<usize>,
    /// Each pixel's time spent rendering its samples this run, in seconds (`time_map`).
    times: Vec<f64>,
    pub width: usize,
    pub height: usize,
    pub rays_cast: u64,
    /// What the stages of a wavefront render did (`scheduler::Stats`); empty for the recursive
    /// tracer, which has none.
    pub schedule: crate::scheduler::Stats,
    pub start_time: Instant,
    pub progressive_render: bool,
    pub samples_rendered: u64,
    pub output_filename: String,
    /// Samples carried over from a saved state (see `load_state`).
    pub resumed_samples: u64,
}

/// A render's saved state begins with this, then the key of what it is a render of, its width
/// and height, then each pixel's summed colour and summed squares (three f64s each) and number
/// of samples (a u64), all little-endian. The sums are kept exactly, so a resumed render is the
/// same, bit for bit, as one that ran straight through.
const STATE_MAGIC: &[u8; 8] = b"RAYSACC3";

/// The bytes of a saved state's header, and of each pixel.
const STATE_HEADER: usize = 32;
const STATE_PIXEL: usize = 56;

/// What a render's samples depend on, besides how many there are: a hash of the scene file's
/// contents (all but its `supersamples` and `adaptive`, so more can be asked for later, or
/// fewer pixels given them; sample k of a pixel is the same either way) and of the overrides
/// of its size and seed. A saved state is resumed only for the same key, so editing the scene
/// starts the render afresh. Files the scene refers to (meshes, cached simulations) and the
/// renderer itself aren't covered: after changing those, render with --fresh.
pub fn render_key(scene: &str, overrides: (Option<usize>, Option<usize>, Option<u64>)) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    match serde_json::from_str::<serde_json::Value>(scene) {
        Ok(mut v) => {
            // Neither changes the samples: the target count, and what's done to the finished
            // image (exposure, film, glare, flare), so a render can be refinished, or carried
            // on, after editing them.
            if let Some(o) = v.as_object_mut() {
                o.remove("supersamples");
                o.remove("adaptive");
                o.remove("output");
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

/// A rectangle of the image, `chunk_size` square but at its right and top edges: the unit
/// whose pixels are rendered together, and judged together for adaptive sampling.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounds {
    pub xmin: usize,
    pub xmax: usize,
    pub ymin: usize,
    pub ymax: usize,
}

impl Bounds {
    pub fn width(&self) -> usize {
        self.xmax - self.xmin
    }

    pub fn len(&self) -> usize {
        self.width() * (self.ymax - self.ymin)
    }

    /// The image's regions, row by row.
    pub fn tiles(width: usize, height: usize, size: usize) -> Vec<Bounds> {
        let size = size.max(1);
        (0..height).step_by(size).flat_map(|y| (0..width).step_by(size).map(move |x| Bounds {
            xmin: x, xmax: (x + size).min(width), ymin: y, ymax: (y + size).min(height),
        })).collect()
    }
}

/// One pass over a region: the samples each of its pixels takes.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderableChunk {
    pub bounds: Bounds,
    /// Pixel by pixel, row by row, its first sample this pass and how many it takes (none for
    /// a pixel that is done).
    pub first: Vec<usize>,
    pub count: Vec<usize>,
    /// Which region of the image this is, and which of that region's passes.
    pub region: usize,
    pub layer: usize,
}

pub struct RenderedChunk {
    pixels: Vec<Color>,
    squares: Vec<Color>,
    samples: Vec<usize>,
    /// Each pixel's time rendering them, in seconds.
    times: Vec<f64>,
    rays_cast: u64,
    /// What the wavefront's stages did, where the kernels rendered it.
    stats: Option<crate::scheduler::Stats>,
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
            squares: vec![Color::black(); width*height],
            samples: vec![0; width*height],
            times: vec![0.; width*height],
            width,
            height,
            rays_cast: 0,
            schedule: Default::default(),
            start_time,
            progressive_render,
            samples_rendered: 0,
            output_filename,
            resumed_samples: 0,
        }
    }

    /// The fewest samples any pixel has.
    /// How many samples pixel (x, y) has.
    pub fn samples_at(&self, x: usize, y: usize) -> usize {
        self.samples[y * self.width + x]
    }

    pub fn min_samples(&self) -> usize {
        self.samples.iter().copied().min().unwrap_or(0)
    }

    /// The most samples any pixel has.
    pub fn max_samples(&self) -> usize {
        self.samples.iter().copied().max().unwrap_or(0)
    }

    /// The mean samples per pixel.
    pub fn mean_samples(&self) -> f64 {
        self.samples.iter().sum::<usize>() as f64 / self.samples.len().max(1) as f64
    }

    /// Each pixel's samples, as a fraction of the most any has: where adaptive sampling spent
    /// them. Top row first.
    pub fn sample_map(&self) -> LinearImage {
        let most = self.max_samples().max(1) as f64;
        let pixels = (0..self.height).rev().flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .map(|(x, y)| {
                let v = self.samples[y * self.width + x] as f64 / most;
                Color::new(v, v, v)
            }).collect();
        LinearImage { width: self.width, height: self.height, pixels }
    }

    /// Each pixel's time spent rendering this run, in seconds, row by row from the bottom.
    pub fn times(&self) -> &[f64] {
        &self.times
    }

    /// The accumulated samples, encoded as a state file for a render identified by `key`.
    pub fn encode_state(&self, key: u64) -> Vec<u8> {
        let mut out = Vec::with_capacity(STATE_HEADER + self.image.len() * STATE_PIXEL);
        out.extend_from_slice(STATE_MAGIC);
        for v in [key, self.width as u64, self.height as u64] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for ((c, q), n) in self.image.iter().zip(&self.squares).zip(&self.samples) {
            for v in [c.rgb.x, c.rgb.y, c.rgb.z, q.rgb.x, q.rgb.y, q.rgb.z] {
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
        if bytes.len() < 8 || &bytes[..8] != STATE_MAGIC {
            return Err(format!("{} is not a saved render, or is from an older version", path));
        }
        if bytes.len() != STATE_HEADER + pixels * STATE_PIXEL {
            return Err(format!("{} is not a saved render of this size", path));
        }
        let u64_at = |i: usize| u64::from_le_bytes(bytes[i..i + 8].try_into().unwrap());
        let f64_at = |i: usize| f64::from_le_bytes(bytes[i..i + 8].try_into().unwrap());
        if u64_at(8) != key || u64_at(16) != self.width as u64 || u64_at(24) != self.height as u64 {
            return Err(format!("{} is of a different scene or settings", path));
        }
        for i in 0..pixels {
            let o = STATE_HEADER + i * STATE_PIXEL;
            self.image[i] = Color::new(f64_at(o), f64_at(o + 8), f64_at(o + 16));
            self.squares[i] = Color::new(f64_at(o + 24), f64_at(o + 32), f64_at(o + 40));
            self.samples[i] = u64_at(o + 48) as usize;
        }
        self.resumed_samples = self.samples.iter().map(|n| *n as u64).sum();
        Ok(())
    }

    /// Add `samples` samples to pixel (x, y): their sum `c`, and the sum of their squares.
    pub fn set_pixel(&mut self, x: usize, y: usize, c:Color, squares: Color, samples: usize) {
        if x >= self.width || y.saturating_mul(self.width).saturating_add(x) >= self.width * self.height {
            return;
        }

        let i:usize = y*self.width + x;
        self.image[i] = self.image[i] + c.ignore_nan();
        self.squares[i] = self.squares[i] + squares.ignore_nan();
        self.samples[i] += samples;
        self.samples_rendered += samples as u64;
    }

    pub fn apply_chunk(&mut self, c: &RenderableChunk, p: &RenderedChunk){
        let b = &c.bounds;
        let mut i = 0;
        for y in b.ymin .. b.ymax {
            for x in b.xmin .. b.xmax {
                if p.samples[i] > 0 {
                    self.set_pixel(x, y, p.pixels[i], p.squares[i], p.samples[i]);
                    self.times[y * self.width + x] += p.times[i];
                }
                i += 1;
            }
        }
        self.rays_cast += p.rays_cast;
        if let Some(stats) = &p.stats {
            self.schedule.merge(stats);
        }
    }

    /// The image's regions, each rendered and judged as one.
    pub fn regions(&self, s: &Scene) -> Vec<Bounds> {
        Bounds::tiles(self.width, self.height, s.render.chunk_size)
    }

    /// The next pass over region `region` (`b`), its `layer`th from here: `samples_per_layer`
    /// more samples for each pixel short of `supersamples`, or with `adaptive`, for each
    /// pixel still too noisy (`adaptive::active`). None once every pixel is done. It depends
    /// only on the region's own pixels, so it is the same however the render is scheduled.
    pub fn next_pass(&self, s: &Scene, region: usize, b: &Bounds, layer: usize) -> Option<RenderableChunk> {
        let (first, count) = self.plan(s, b, s.render.samples_per_layer());
        count.iter().any(|&n| n > 0).then_some(RenderableChunk { bounds: *b, first, count, region, layer })
    }

    /// For each pixel of region `b`, row by row: its next sample's index (its count so far),
    /// and how many it should take in its next pass, at most `most` (`scheduler::plan`: none
    /// for a pixel that is complete).
    pub fn plan(&self, s: &Scene, b: &Bounds, most: usize) -> (Vec<usize>, Vec<usize>) {
        let target = s.render.samples_per_pixel();
        let index: Vec<usize> = (b.ymin..b.ymax).flat_map(|y| (b.xmin..b.xmax).map(move |x| y * self.width + x)).collect();
        let first: Vec<usize> = index.iter().map(|&i| self.samples[i]).collect();
        // The sums are read only by adaptive sampling.
        let (sum, squares): (Vec<Color>, Vec<Color>) = match &s.render.adaptive {
            None => (Vec::new(), Vec::new()),
            Some(_) => (index.iter().map(|&i| self.image[i]).collect(), index.iter().map(|&i| self.squares[i]).collect()),
        };
        let film = crate::adaptive::Region { width: b.width(), sum: &sum, squares: &squares, samples: &first };
        let count = crate::scheduler::plan(&film, target, s.render.adaptive.as_ref(), crate::adaptive::display_gain(s), most);
        (first, count)
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
        if !self.schedule.is_empty() {
            println!("{}", self.schedule.report());
        }
        println!("==========================================");

    }

    
    pub fn progress(&self, s: &Scene) -> String {
        let elapsed = Instant::now() - self.start_time;
        format!("{:.2}s {:.1} spp, {} rays cast ({} RPS), {} Rays per sample, {}%, {} threads",
                 elapsed.as_secs_f64(),
                 self.mean_samples(),
                 format_f64(self.rays_cast as f64),
                 format_f64(self.rays_cast as f64 / elapsed.as_secs_f64()),
                 format_f64(self.rays_cast as f64 / self.samples_rendered as f64),
                 format_f64(self.progress_percentage(s)),
                 rayon::current_num_threads())
    }

    pub fn progress_percentage(&self, s: &Scene) -> f64 {
        ((self.resumed_samples + self.samples_rendered) as f64 / (self.width * self.height * s.render.samples_per_pixel()) as f64) * 100.
    }
}

/// Render every chunk of the image, all threads working at once, with output that depends only
/// on the scene and its seed.
///
/// Each pixel's samples are already independent of scheduling (see `render_pixel`), but the
/// order in which a pixel's passes are added up still matters: floating-point addition isn't
/// associative. And with adaptive sampling, which pixels a pass samples depends on the passes
/// before it. So a region's next pass is decided (`RenderContext::next_pass`) only once its
/// last has been applied, from the region's own pixels. Passes waiting to be rendered are
/// taken lowest layer first, so the image fills in evenly.
///
/// After each chunk is applied, `after_chunk` runs with the context locked; anything it returns
/// is passed to `outside_lock`, eg. to write a progressive image without holding up the render.
///
/// A chunk is rendered by the wavefront's kernels where the scene is laid out for them
/// (`kernels`: `wavefront::integrator::Kernels`), and otherwise a path at a time by the scene's
/// own code (`render_pixel`, `trace`): one driver either way, so the same passes, adaptive
/// sampling, saved states and progress.
pub fn render<T, F, G>(s: &Scene, rc: &Mutex<RenderContext>, kernels: Option<&crate::wavefront::integrator::Kernels>, after_chunk: F, outside_lock: G)
where
    F: Fn(&RenderContext) -> Option<T> + Sync,
    G: Fn(T) + Sync,
{
    let (regions, first) = {
        let ctx = rc.lock().unwrap();
        let regions = ctx.regions(s);
        let first: Vec<RenderableChunk> = regions.iter().enumerate()
            .filter_map(|(r, b)| ctx.next_pass(s, r, b, 0)).collect();
        (regions, first)
    };
    let jobs = Jobs {
        s, rc, kernels, after_chunk, outside_lock, regions,
        queue: Mutex::new(first.into_iter().map(|c| ((c.layer, c.region), c)).collect()),
    };
    let n = jobs.queue.lock().unwrap().len();
    rayon::scope(|scope| {
        for _ in 0..n {
            scope.spawn(|scope| jobs.run(scope));
        }
    });
}

/// The passes waiting to be rendered, by layer and region, and what to do with each.
struct Jobs<'a, F, G> {
    s: &'a Scene,
    rc: &'a Mutex<RenderContext>,
    kernels: Option<&'a crate::wavefront::integrator::Kernels<'a>>,
    after_chunk: F,
    outside_lock: G,
    regions: Vec<Bounds>,
    queue: Mutex<BTreeMap<(usize, usize), RenderableChunk>>,
}

impl<T, F, G> Jobs<'_, F, G>
where
    F: Fn(&RenderContext) -> Option<T> + Sync,
    G: Fn(T) + Sync,
{
    /// Render the first pass waiting, apply it, and queue the region's next (with a task to
    /// run it: there is one task for each pass queued, though not necessarily the same one).
    fn run<'s>(&'s self, scope: &rayon::Scope<'s>) {
        let Some((_, chunk)) = self.queue.lock().unwrap().pop_first() else { return };
        let rendered = chunk.render(self.s, self.kernels);
        let pending = {
            let mut ctx = self.rc.lock().unwrap();
            ctx.apply_chunk(&chunk, &rendered);
            if let Some(next) = ctx.next_pass(self.s, chunk.region, &self.regions[chunk.region], chunk.layer + 1) {
                self.queue.lock().unwrap().insert((next.layer, next.region), next);
                scope.spawn(|scope| self.run(scope));
            }
            (self.after_chunk)(&ctx)
        };
        if let Some(t) = pending {
            (self.outside_lock)(t);
        }
    }
}

impl RenderableChunk {
    pub fn render(&self, s: &Scene, kernels: Option<&crate::wavefront::integrator::Kernels>) -> RenderedChunk {
        let b = &self.bounds;
        let size = b.len();
        if let Some(kernels) = kernels {
            // The wavefront's stages over the chunk's paths.
            let w = b.width();
            let pixels: Vec<usize> = (0..size).map(|i| (b.ymin + i / w) * s.image.width + b.xmin + i % w).collect();
            let done = kernels.samples(&pixels, &self.first, &self.count);
            return RenderedChunk {
                pixels: done.sums, squares: done.squares, samples: self.count.clone(), times: done.seconds,
                rays_cast: done.rays.iter().sum(), stats: Some(done.stats),
            };
        }
        let mut chunk = RenderedChunk {
            pixels: Vec::with_capacity(size),
            squares: Vec::with_capacity(size),
            samples: Vec::with_capacity(size),
            times: Vec::with_capacity(size),
            rays_cast: 0,
            stats: None,
        };
        // Pixels are rendered in parallel too, so a layer with few chunks of uneven cost (eg. sky
        // and a horizon of grazing rays) still keeps every thread busy. Results are collected in
        // order, so they are the same however the work is scheduled.
        let w = b.width();
        // Each pixel is timed on the thread that renders it, for `--time-map`.
        let results: Vec<((u64, usize, Color, Color), f64)> = (0 .. size).into_par_iter()
            .map(|i| match self.count[i] {
                0 => ((0, 0, Color::black(), Color::black()), 0.),
                n => {
                    let start = crate::time_map::thread_time();
                    let r = render_pixel(b.xmin + i % w, b.ymin + i / w, self.first[i], n, s);
                    (r, crate::time_map::thread_time() - start)
                },
            })
            .collect();
        for ((cast, samples, pixel, squares), time) in results {
            chunk.pixels.push(pixel);
            chunk.squares.push(squares);
            chunk.samples.push(samples);
            chunk.times.push(time);
            chunk.rays_cast += cast;
        }
        chunk
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
/// Returns the rays cast, the samples taken, and their sum and sum of squares.
pub(crate) fn render_pixel(x: usize, y: usize, first: usize, count: usize, s: &Scene) -> (u64, usize, Color, Color) {
    let pixels = (s.image.width * s.image.height) as u64;
    let index = (y * s.image.width + x) as u64;
    // This pixel's offset of the sequence.
    seed_rand(s.render.seed ^ 0x9e37_79b9_7f4a_7c15, index);
    let offset = (rand(), rand());

    let mut pixel = Color::black();
    let mut squares = Color::black();
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
            Sampler::Halton => {
                begin_sampler(crate::sampler::SampleState::Halton(crate::sampler::Halton::new(pixel_seed, k as u64)));
                (rand(), rand())
            },
            Sampler::Stratified => {
                begin_sampler(crate::sampler::SampleState::Stratified(crate::sampler::Stratified::new(pixel_seed, k as u32, s.render.supersamples as u32)));
                (rand(), rand())
            },
            Sampler::ZSobol => {
                let planned = s.render.supersamples.max(1).next_power_of_two();
                if k < planned {
                    let resolution = s.image.width.max(s.image.height) as u32;
                    begin_sampler(crate::sampler::SampleState::ZSobol(crate::sampler::ZSobol::new(
                        s.render.seed, x as u32, y as u32, k as u32, planned as u32, resolution)));
                } else {
                    // Beyond the planned samples, the pixel's own Sobol' sequence.
                    begin_sample(pixel_seed, k as u32);
                }
                (rand(), rand())
            },
            Sampler::Independent => ((offset.0 + k as f64 * R2.0).fract(), (offset.1 + k as f64 * R2.1).fract()),
        };
        // With motion blur, the sample's time (`motion`).
        if s.render.motion {
            crate::motion::begin_sample(s.render.shutter.time(rand()));
        }
        // In spectral mode, the sample's wavelengths (`spectrum`).
        if s.render.spectral {
            crate::spectrum::begin_sample(crate::spectrum::Wavelengths::sample_visible(rand()));
        }
        // Jitter warped to the reconstruction filter, and the sample's weight (one, but for
        // filters with negative lobes).
        let ((fx, wx), (fy, wy)) = (s.render.filter.sample_weighted(u), s.render.filter.sample_weighted(v));
        let camera_ray = s.camera.get_ray_weighted(
            x as f64 / (s.image.width as f64),
            y as f64 / (s.image.height as f64),
            // Measured from the pixel's corner.
            (0.5 + fx) / (s.image.width as f64),
            (0.5 + fy) / (s.image.height as f64));
        // A ray the lens stops is a black sample.
        let (rays_cast, c) = match camera_ray {
            Some((ray, weight)) => {
                let (n, c) = trace(&ray, 0, s);
                (n, if weight == 1. { c } else { c * weight })
            },
            None => (0, Color::black()),
        };
        cast += rays_cast;
        // A spectral sample's radiance becomes a colour (and stray NaNs, as in RGB, pass on).
        let c = match crate::spectrum::end_sample() {
            Some(w) => w.to_srgb(&c),
            None => c,
        };
        let c = if wx * wy == 1. { c } else { c * (wx * wy) };
        crate::motion::end_sample();
        pixel = pixel + c;
        squares = squares + Color { rgb: c.rgb.component_mul(&c.rgb) };
    }
    end_sample();
    (cast, count, pixel, squares)
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
        render_layers(s, usize::MAX)
    }

    /// Render one region at a time, stopping after `layers` passes.
    fn render_layers(s: &Scene, layers: usize) -> RenderContext {
        let mut rc = RenderContext::new(s.image.width, s.image.height, false, "test.json");
        let regions = rc.regions(s);
        for layer in 0..layers {
            let mut any = false;
            for (r, b) in regions.iter().enumerate() {
                if let Some(c) = rc.next_pass(s, r, b, layer) {
                    let p = c.render(s, None);
                    rc.apply_chunk(&c, &p);
                    any = true;
                }
            }
            if !any {
                break;
            }
        }
        rc
    }

    /// Render on all threads through `render`, as main does.
    fn render_parallel(seed: u64) -> Vec<Color> {
        render_from(seed, 32, None).snapshot().pixels
    }

    /// As `render_parallel`, to `supersamples`, carrying on from a saved state.
    fn render_from(seed: u64, supersamples: usize, state: Option<&[u8]>) -> RenderContext {
        render_adaptive(seed, supersamples, state, "false", 2)
    }

    /// As `render_from`, with `"adaptive": adaptive` and regions `chunk_size` square.
    fn render_adaptive(seed: u64, supersamples: usize, state: Option<&[u8]>, adaptive: &str, chunk_size: usize) -> RenderContext {
        let s = test_scene(seed, supersamples, adaptive, chunk_size);
        let mut ctx = RenderContext::new(s.image.width, s.image.height, false, "test.json");
        if let Some(bytes) = state {
            let path = std::env::temp_dir().join(format!("rays-test-state-{}-{}.accum", seed, std::process::id()));
            std::fs::write(&path, bytes).unwrap();
            ctx.load_state(path.to_str().unwrap(), 7).unwrap();
            std::fs::remove_file(&path).unwrap();
        }
        let rc = Mutex::new(ctx);
        super::render(&s, &rc, None, |_| None::<()>, |_| ());
        rc.into_inner().unwrap()
    }

    /// A small scene of diffuse, metal and glass spheres in fog, for `render_adaptive`.
    fn test_scene(seed: u64, supersamples: usize, adaptive: &str, chunk_size: usize) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 24, "height": 16, "chunk_size": {}, "supersamples": {}, "samples_per_chunk": 1, "adaptive": {},
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
        }}"#, chunk_size, supersamples, adaptive, seed))
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
    fn test_adaptive_render_is_deterministic_and_resumable() {
        let adaptive = r#"{"threshold": 0.05, "min_samples": 4}"#;
        let straight = render_adaptive(5, 64, None, adaptive, 8);
        // Smooth background pixels stop early; noisy ones carry on.
        assert_eq!(straight.min_samples(), 4);
        assert_eq!(straight.max_samples(), 64);
        assert!(straight.mean_samples() < 48., "{}", straight.mean_samples());
        // The same again, however the threads ran.
        let again = render_adaptive(5, 64, None, adaptive, 8);
        assert_eq!(again.samples, straight.samples);
        assert_eq!(again.snapshot().pixels, straight.snapshot().pixels);
        // Or one region at a time.
        let s = test_scene(5, 64, adaptive, 8);
        assert_eq!(render(&s).snapshot().pixels, straight.snapshot().pixels);
        // Interrupted, saved and carried on: the same image.
        for layers in [3, 9, 20] {
            let part = render_layers(&s, layers).encode_state(7);
            let resumed = render_adaptive(5, 64, Some(&part), adaptive, 8);
            assert_eq!(resumed.samples, straight.samples, "after {} layers", layers);
            assert_eq!(resumed.snapshot().pixels, straight.snapshot().pixels);
        }
        // A uniform render carried on adaptively, and vice versa.
        let uniform = render_adaptive(5, 16, None, "false", 8).encode_state(7);
        assert!(render_adaptive(5, 64, Some(&uniform), adaptive, 8).mean_samples() >= 16.);
        let more = render_adaptive(5, 64, Some(&straight.encode_state(7)), "false", 8);
        assert!(more.samples.iter().all(|&n| n == 64));
    }

    #[test]
    fn test_adaptive_sampling_spends_samples_where_the_noise_is() {
        // At equal mean samples, a lower error where it's highest.
        let reference = render_adaptive(11, 1024, None, "false", 8).snapshot().pixels;
        let error = |rc: &RenderContext| {
            let p = rc.snapshot().pixels;
            let mut e: Vec<f64> = p.iter().zip(&reference).map(|(a, b)| (a.rgb - b.rgb).norm_squared()).collect();
            e.sort_by(|a, b| b.partial_cmp(a).unwrap());
            e[..e.len() / 10].iter().sum::<f64>()
        };
        let adaptive = render_adaptive(1, 128, None, r#"{"threshold": 0.03, "min_samples": 8}"#, 8);
        let spp = adaptive.mean_samples().ceil() as usize;
        assert!(spp < 96, "{}", spp);
        let uniform = render_adaptive(1, spp, None, "false", 8);
        assert!(error(&adaptive) < error(&uniform), "adaptive {} vs uniform {} at {} spp", error(&adaptive), error(&uniform), spp);
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
        // The finished image's treatment doesn't change the samples either.
        let refinished = r#"{"width": 10, "supersamples": 16, "camera": {"angle": 0.5}, "output": {"exposure": 2}}"#;
        assert_eq!(render_key(a, none), render_key(refinished, none));
        // Nor does which pixels take them.
        let adaptive = r#"{"width": 10, "supersamples": 16, "camera": {"angle": 0.5}, "adaptive": 0.02}"#;
        assert_eq!(render_key(a, none), render_key(adaptive, none));
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
