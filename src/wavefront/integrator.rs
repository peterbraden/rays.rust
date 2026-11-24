//! The wavefront path tracer on the CPU (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 4th ed., chapter 15; cited below as PBR).
//!
//! The recursive tracer (`trace`) follows one path at a time to its end, branching on what it
//! hits through trait objects. A wavefront renderer instead keeps every path of a pass (one
//! sample per pixel) in a queue and advances them all together, one stage at a time, each
//! stage a single loop over many paths doing one kind of work (PBR section 15.2).
//!
//! The stages are the kernels' (`shaders/pathtrace`: the one copy of the wavefront's
//! intersection and shading, in f32, over the scene laid out as its records, `kernels`),
//! which the GPU runs as a shader (`gpu`) and this runs as the same crate compiled for the
//! CPU. A round, over the paths still alive:
//!
//! 1. Intersect: the surface each path's ray hits (`pathtrace::intersect_surface`); then the
//!    shapes the kernels have no test for, by their own code (`FlatScene::shapes`,
//!    `pathtrace::PRIM_HOST`); then the walk through the media as far as that
//!    (`intersect_media`).
//! 2. Shade (`pathtrace::shade`): the sky or a light the ray found, haze, direct light with
//!    its shadow rays, and the next ray; with what it asks of the sky (`SkyQuery`).
//! 3. The sky's answers (`pathtrace::sky_light`).
//! 4. The host's vertices: paths stopped at a material or a shape the kernels have no code
//!    for (`pathtrace::PATH_HOST`) are shaded by the scene's own `MaterialModel::scatter`
//!    (`kernels::host::shade`, `host_bounce`), and go on.
//!
//! and round again until no path is left. On the GPU the host's vertices wait in a queue that
//! is read back; here they are in memory, and the host is asked at once, shadow rays included
//! (`pathtrace::HostShapes`), which is why a shape only the host can intersect renders here
//! and not there. Nothing else differs: the same code, the same random numbers (a hash a
//! path; a Sobol' point a host vertex), so the same picture but for the two machines'
//! arithmetic.
//!
//! The pass's pixels are split into batches of a few thousand paths, run in parallel, each
//! through all the stages: a whole image's queue streamed through memory at every stage made
//! the stages memory-bound, twice as slow as the recursive tracer.
//!
//! This replaced stages of its own in f64, a second copy of the kernels' shading that had to
//! be kept in step with them and with `trace` (plan.md section 15). What it gave up with
//! them, until the kernels have it: Sobol' points for the paths' own vertices, and the light
//! BVH (area lights are chosen by power).

use crate::intersection::Intersection;
use crate::kernels::Buffers;
use crate::material::model::LightSampling;
use crate::scheduler::{Stage, Stats};
use crate::color::Color;
use crate::material::bsdf::power_heuristic;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::rendercontext::RenderContext;
use crate::scene::{PathTermination, Scene};
use crate::wavefront::flat::{FlatMaterial, FlatScene, Hit};
use pathtrace::glam::{Vec3, Vec4};
use pathtrace::{PathState, SkyQuery, PATH_ACTIVE, PATH_HOST};
use rayon::prelude::*;
use std::sync::Mutex;

/// One path's state between stages.
struct Path {
    pixel: u32,
    ray: Ray,
    /// Throughput: the product of the BSDF weights so far.
    beta: Color,
    /// Radiance gathered so far.
    l: Color,
    /// The camera's and filter's weight for the sample.
    weight: f64,
    depth: u32,
    /// Whether the last bounce was a delta (a mirror or glass), or there was none: then a
    /// light the ray hits counts in full.
    specular: bool,
    /// The density the last bounce sampled its direction with, for MIS.
    prev_pdf: f64,
    /// The path's own Sobol' sample (as the recursive tracer's `begin_sample`, but carried
    /// with the path rather than in the thread).
    sampler: crate::sobol::SampleState,
    /// Whether the path has scattered off a non-delta BSDF, after which glossy lobes are
    /// regularised (`ggx::regularised`).
    scattered: bool,
    /// Whether the last bounce's light sampling counted the lights in full
    /// (`LightSampling::Exclusive`, which only a host material answers): then a light the ray
    /// hits adds nothing, though a sky does.
    lights_counted: bool,
    /// Of those, only the distant ones (`LightSampling::Directional`): an area light the ray
    /// finds counts in full.
    directional: bool,
    hit: Option<Hit>,
    done: bool,
    /// Its scatterings in media and the scale they left on densities (`HostVertex::volume`).
    volume: (u32, f64),
}

impl Path {
    fn u(&mut self) -> f64 {
        self.sampler.next()
    }
}


/// Paths per batch: a queue small enough to stay in cache through every stage, and enough
/// batches to keep every thread busy.
const BATCH: usize = 4096;

/// What `Kernels::samples` rendered, for each pixel asked for: the sum of its samples and of
/// their squares, the rays they cast and the seconds they took; and what the stages did.
pub struct Samples {
    pub sums: Vec<Color>,
    pub squares: Vec<Color>,
    pub rays: Vec<u64>,
    pub seconds: Vec<f64>,
    pub stats: Stats,
}

/// A flat scene laid out for the kernels, with the scene whose own code answers for what they
/// lack: what a pass on the CPU runs over.
pub struct Kernels<'a> {
    s: &'a Scene,
    flat: &'a FlatScene,
    pub buffers: Buffers,
}

/// A kernel's ray (in the frame about `origin`), as the scene's own code takes one.
fn host_ray(ro: Vec3, rd: Vec3, origin: &Vector3<f64>) -> Ray {
    Ray { ro: Vector3::new(ro.x as f64, ro.y as f64, ro.z as f64) + origin, rd: Vector3::new(rd.x as f64, rd.y as f64, rd.z as f64).normalize() }
}

/// Shadow rays past the shapes the kernels can't test.
impl pathtrace::HostShapes for Kernels<'_> {
    fn occluded(&self, ro: Vec3, rd: Vec3, tmax: f32) -> bool {
        let r = host_ray(ro, rd, &self.buffers.origin);
        self.buffers.host_shapes.iter().any(|&k| self.flat.shapes[k].0.geometry.occludes(&r, crate::wavefront::flat::MIN_SHAPE, tmax as f64))
    }
}

/// The CPU's finder: the kernels' surfaces, the host's shapes by their own code, then the
/// media as far as what those found.
struct WithHost<'a, M: pathtrace::Media> {
    kernels: &'a Kernels<'a>,
    host_shapes: &'a [usize],
    listed: usize,
    media: std::marker::PhantomData<M>,
}

impl<M: pathtrace::Media> pathtrace::Finder for WithHost<'_, M> {
    const MEDIA: bool = M::ON;

    fn find(&self, scene: &pathtrace::Scene, p: &PathState) -> (PathState, f32) {
        let mut found = pathtrace::intersect_surface(scene, p);
        if !self.host_shapes.is_empty() {
            let r = host_ray(p.o.truncate(), p.d.truncate(), &self.kernels.buffers.origin);
            let mut reach = if found.state.z == u32::MAX { f64::INFINITY } else { found.hit.x as f64 };
            for &shape in self.host_shapes {
                if let Some(x) = self.kernels.flat.shapes[shape].0.geometry.intersects_range(&r, crate::wavefront::flat::MIN_SHAPE, reach) {
                    reach = x.dist;
                    found.state.z = (self.listed + shape) as u32;
                    found.hit = Vec4::new(x.dist as f32, 0., 0., 1.);
                }
            }
        }
        pathtrace::intersect_media::<M>(scene, &found)
    }
}

impl<'a> Kernels<'a> {
    /// The scene as the kernels read it, or why they can't render it.
    pub fn new(s: &'a Scene, flat: &'a FlatScene) -> Result<Kernels<'a>, String> {
        Ok(Kernels::with(s, flat, Buffers::build(s, flat)?))
    }

    /// As `new`, with the scene's buffers already laid out (`Buffers::build`).
    pub fn with(s: &'a Scene, flat: &'a FlatScene, buffers: Buffers) -> Kernels<'a> {
        Kernels { s, flat, buffers }
    }

    /// Render sample `k` of every pixel: each pixel's radiance (times its weight), the number
    /// of rays cast, and what the stages did (`scheduler::Stats`).
    pub fn pass(&self, k: usize) -> (Vec<Color>, u64, Stats) {
        let pixels: Vec<usize> = (0..self.s.image.width * self.s.image.height).collect();
        let done = self.samples(&pixels, &vec![k; pixels.len()], &vec![1; pixels.len()]);
        (done.sums, done.rays.iter().sum(), done.stats)
    }

    /// Render `count[i]` samples of pixel `pixels[i]` (its index in the image) from its sample
    /// `first[i]`, as the scheduler planned (`scheduler::plan`): a region's pass
    /// (`rendercontext::RenderableChunk`). A sample is the same whichever pass it is in, and a
    /// pixel's are added in their order, so the sums are those of as many passes of one.
    pub fn samples(&self, pixels: &[usize], first: &[usize], count: &[usize]) -> Samples {
        let n = pixels.len();
        let passes = count.iter().copied().max().unwrap_or(0);
        let media = !self.flat.media.is_empty();
        // Batches of paths run in parallel, each through all the stages: a queue that streams
        // through memory once per stage would make the stages memory-bound on a CPU (on a GPU,
        // the whole image's queue is the batch).
        let batches: Vec<_> = (0..n.div_ceil(BATCH) * passes).into_par_iter()
            .map(|j| {
                // (Pass by pass: so a pixel's samples are collected in their order.)
                let (lane, b) = (j / n.div_ceil(BATCH), j % n.div_ceil(BATCH));
                let paths: Vec<(u32, u32, u32)> = (b * BATCH..((b + 1) * BATCH).min(n)).filter(|&i| lane < count[i])
                    .map(|i| (i as u32, pixels[i] as u32, (first[i] + lane) as u32)).collect();
                // (A scene without media runs the stages compiled without them, as on the GPU.)
                if media { self.batch::<pathtrace::WithMedia>(&paths) } else { self.batch::<pathtrace::NoMedia>(&paths) }
            })
            .collect();
        let mut out = Samples { sums: vec![Color::black(); n], squares: vec![Color::black(); n], rays: vec![0; n], seconds: vec![0.; n], stats: Stats::default() };
        out.stats.passes = passes as u64;
        for (done, seconds, stats) in batches {
            out.stats.merge(&stats);
            // A batch's time, shared among its paths by the rays each traced.
            let traced: u64 = done.iter().map(|d| d.2).sum();
            for (i, c, rays) in done {
                let i = i as usize;
                out.sums[i] = out.sums[i] + c;
                out.squares[i] = out.squares[i] + Color { rgb: c.rgb.component_mul(&c.rgb) };
                out.rays[i] += rays;
                out.seconds[i] += seconds * rays as f64 / traced.max(1) as f64;
            }
        }
        out
    }

    /// The paths `paths` (each its place in the caller's list, its pixel and its sample's
    /// number), stage by stage: each finished path's place, radiance and rays; the seconds
    /// the batch took; and what its stages did.
    // (Never inlined: a profiler collects inside it, `--toggle-collect=*Kernels*batch*`.)
    #[inline(never)]
    fn batch<M: pathtrace::Media>(&self, wanted: &[(u32, u32, u32)]) -> (Vec<(u32, Color, u64)>, f64, Stats) {
        let began = crate::time_map::thread_time();
        let (s, flat) = (self.s, self.flat);
        let host_shapes = &self.buffers.host_shapes;
        let scene = self.buffers.scene(if host_shapes.is_empty() { None } else { Some(self) });
        let listed = flat.prims.len() + flat.planes.len();
        // Each stage's batch is counted and timed (`scheduler::Stats`): two clock readings a
        // stage a round, against thousands of paths.
        let mut stats = Stats::default();
        let mut clock = web_time::Instant::now();
        let mut lap = |stats: &mut Stats, stage: Stage, items: usize| {
            let now = web_time::Instant::now();
            stats.record(stage, items as u64, (now - clock).as_secs_f64());
            clock = now;
        };
        // Camera rays, and the paths they start (`pathtrace::start_paths` on the GPU).
        let mut paths: Vec<((u32, u32, u32), PathState)> = wanted.iter().filter_map(|&(place, pixel, k)| {
            let ray = crate::kernels::camera_ray(s, pixel as usize, k as usize, &self.buffers.origin);
            (ray.d.w >= 0.5).then(|| ((place, pixel, k), PathState::start(pixel, k, self.buffers.params.seed.x, &ray)))
        }).collect();
        lap(&mut stats, Stage::Generate, paths.len());
        let mut done = Vec::with_capacity(paths.len());
        let mut queries: Vec<(usize, SkyQuery, SkyQuery)> = Vec::new();
        let mut round = 0;
        while !paths.is_empty() {
            stats.round(round, paths.len() as u64);
            round += 1;
            // 1. Intersect: the kernels' surfaces, the host's shapes, then the media; a
            // path's own ray and those traced for it (`pathtrace::find_all`).
            let finder = WithHost::<M> { kernels: self, host_shapes, listed, media: std::marker::PhantomData };
            for (_, p) in paths.iter_mut() {
                *p = pathtrace::find_all(&scene, p, &finder);
            }
            lap(&mut stats, Stage::Intersect, paths.len());
            // 2. Shade.
            queries.clear();
            for (i, ((_, pixel, _), p)) in paths.iter_mut().enumerate() {
                let (next, q0, q1) = pathtrace::shade::<M>(&scene, p, *pixel);
                *p = next;
                if q0.weight.truncate().length_squared() > 0. || q1.weight.truncate().length_squared() > 0. {
                    queries.push((i, q0, q1));
                }
            }
            lap(&mut stats, Stage::Shade, paths.len());
            // 3. The sky's answers: a vertex's two summed, then added, as `answer_sky` adds
            // them on the GPU (in any other order the sums differ in their last bits).
            for (i, q0, q1) in &queries {
                let light = Vec3::ZERO + pathtrace::sky_light(&scene, q0) + pathtrace::sky_light(&scene, q1);
                if light.dot(light) > 0. {
                    paths[*i].1.l += light.extend(0.);
                }
            }
            lap(&mut stats, Stage::Sky, queries.len());
            // 4. The host's vertices.
            let mut waiting = 0;
            for ((_, pixel, k), p) in paths.iter_mut().filter(|(_, p)| p.state.y & PATH_HOST != 0) {
                *p = crate::kernels::host::shade(s, flat, &self.buffers.origin, p, *pixel, *k);
                waiting += 1;
            }
            if waiting > 0 {
                lap(&mut stats, Stage::Host, waiting);
            }
            // Finished paths leave the queue.
            paths.retain(|((place, _, _), p)| {
                if p.state.y & PATH_ACTIVE != 0 {
                    return true;
                }
                let sum = pathtrace::path_sum(p);
                done.push((*place, Color::new(sum.x as f64, sum.y as f64, sum.z as f64), sum.w as u64));
                false
            });
        }
        lap(&mut stats, Stage::Accumulate, done.len());
        (done, crate::time_map::thread_time() - began, stats)
    }
}

/// As `trace`: segments shorter than this cross too little air to matter.
#[allow(dead_code)]
const AERIAL_PERSPECTIVE_FROM: f64 = 100.;

/// What a ray that escaped the flattened scene sees: the skysphere, by its material, or the
/// background.
pub fn sky_radiance(r: &Ray, s: &Scene, flat: &FlatScene) -> Color {
    use crate::shapes::geometry::Geometry;
    if let Some(sky) = &flat.sky
        && let Some(x) = sky.geometry.intersects(r) {
        let i = crate::intersection::Intersection { dist: x.dist, point: x.point, normal: x.normal, face: x.face, part: x.part, object: sky };
        let m = sky.medium.material_at(x.point);
        let out = m.scatter(r, &i, s);
        return out.emit + out.attenuate;
    }
    s.render.background.radiance(&r.rd)
}

/// A vertex on a material that isn't data (`FlatMaterial::Host`): the scene's own material
/// answers, as it does the recursive tracer (`trace::trace_sample`, which this follows). It
/// gathers its direct light itself, with its own shadow rays through the scene's own
/// acceleration structure, and returns the next ray; the path's sampler and its regularisation
/// state are lent to the thread for the call, since that code draws from `geometry::rand()`.
///
/// This is the CPU's side of the scheduler's CPU queue (`scheduler`): work the stages have no
/// code for, done one path at a time by code that has. It is why such paths sort last in the
/// shading stage and are counted apart (`Stage::Host`): what they cost is what porting their
/// material would save.
fn host_bounce(p: &mut Path, s: &Scene, o: &std::sync::Arc<crate::sceneobject::SceneObject>, h: &Hit) {
    let found = Intersection { dist: h.dist, point: h.point, normal: h.shading, face: h.face, object: o, part: h.part };
    // Off the surface, on the side the ray arrived from (`trace::trace_intersection`).
    let bias = s.render.shadow_bias.max(found.spawn_offset());
    let hit = Intersection { point: found.point + found.facing_normal(&p.ray) * bias, ..found };
    let shaded = crate::trace::shading(&p.ray, &*o.medium, &hit, s);
    // A moon is sampled as a light (`Scene::moonlight`): a ray that finds it counts it as one
    // that finds the sun's disc (`trace::trace_sample`).
    if let Some(moon) = s.moonlight.as_ref().filter(|m| m.is_on(&hit.point)) {
        let w = match (p.specular, p.lights_counted) {
            (true, _) => 1.,
            (_, true) => 0.,
            _ => power_heuristic(p.prev_pdf, moon.light.disc_pdf(&p.ray.rd)),
        };
        if w == 0. {
            p.done = true;
            return;
        }
        p.beta = p.beta * w;
    }
    crate::material::ggx::set_scattered(p.scattered);
    // Its shadow rays cross the media as the path's segments do (`Volume::segment_scale`), and
    // leave a vertex (`trace::trace_sample`).
    crate::volume::resume_path(p.volume.0, p.volume.1, p.scattered);
    crate::volume::at_vertex();
    crate::geometry::begin_sampler(crate::sampler::SampleState::Sobol(p.sampler.clone()));
    let interaction = o.medium.material_at(hit.point).scatter(&p.ray, &shaded, s);
    if let Some(crate::sampler::SampleState::Sobol(sampler)) = crate::geometry::take_sampler() {
        p.sampler = sampler;
    }
    p.scattered = crate::material::ggx::has_scattered();
    p.l = p.l + p.beta * interaction.emit;
    let Some(ray) = interaction.ray else {
        // No ray: the weight is the vertex's own colour.
        p.l = p.l + p.beta * interaction.attenuate;
        p.done = true;
        return;
    };
    if p.depth >= s.render.max_depth as u32 || !continue_with(p, s, &interaction.attenuate) {
        p.done = true;
        return;
    }
    p.beta = p.beta * interaction.attenuate;
    // What the next vertex counts of a light it finds (`trace::light_weight`).
    (p.specular, p.lights_counted, p.prev_pdf) = match interaction.light_sampling {
        LightSampling::None => (true, false, 0.),
        LightSampling::Exclusive | LightSampling::Directional => (false, true, 0.),
        LightSampling::MIS { bsdf_pdf } => (false, false, bsdf_pdf),
    };
    p.directional = interaction.light_sampling == LightSampling::Directional;
    p.ray = Ray { ro: ray.ro, rd: ray.rd.normalize() };
    p.depth += 1;
}

/// A vertex of a path kept elsewhere (the GPU's: `gpu::host`), for `host_vertex`: the ray that
/// arrived, the throughput, and what the vertex before did; after it, the next ray, or `done`,
/// and in `l` the light the vertex added.
pub struct HostVertex {
    pub ray: Ray,
    pub beta: Color,
    pub l: Color,
    pub depth: u32,
    pub specular: bool,
    pub lights_counted: bool,
    pub directional: bool,
    pub prev_pdf: f64,
    pub scattered: bool,
    /// How many times the path has scattered in media, and the factor that left on their
    /// densities for its segments (`volume::resume_path`); (0, 1) before any.
    pub volume: (u32, f64),
    pub done: bool,
}

/// `host_bounce` for a path kept elsewhere, at `hit`, drawing its numbers from `sampler`.
/// Nothing if the hit's material is one the stages shade.
pub fn host_vertex(s: &Scene, flat: &FlatScene, v: &mut HostVertex, hit: &Hit, sampler: crate::sobol::SampleState) {
    // A shape the host intersects is the host's to shade whatever its material; else a
    // material that isn't data.
    let listed = flat.prims.len() + flat.planes.len();
    let o = if hit.prim as usize >= listed {
        &flat.shapes[hit.prim as usize - listed].0
    } else if let FlatMaterial::Host { object } = flat.hit_material(hit.prim) {
        &flat.hosts[*object as usize]
    } else {
        return;
    };
    let mut p = Path {
        pixel: 0, ray: v.ray, beta: v.beta, l: v.l, weight: 1., depth: v.depth, specular: v.specular, prev_pdf: v.prev_pdf, sampler,
        scattered: v.scattered, lights_counted: v.lights_counted, directional: v.directional, hit: Some(*hit), done: false, volume: v.volume,
    };
    host_bounce(&mut p, s, o, hit);
    *v = HostVertex {
        ray: p.ray, beta: p.beta, l: p.l, depth: p.depth, specular: p.specular, lights_counted: p.lights_counted, directional: p.directional, prev_pdf: p.prev_pdf,
        scattered: p.scattered, done: p.done, volume: p.volume,
    };
}

/// Whether the path goes on after a bounce of weight `attenuate`, by the scene's Russian
/// roulette (as `trace::continuation_weight`); a survivor's throughput is scaled to match.
fn continue_with(p: &mut Path, s: &Scene, attenuate: &Color) -> bool {
    if attenuate.as_vec().norm() <= s.black_threshold {
        p.done = true;
        return false;
    }
    if let PathTermination::RussianRoulette { min_depth } = s.render.path_termination
        && p.depth >= min_depth as u32 {
        let q = attenuate.as_vec().max().min(1.);
        if p.u() >= q {
            p.done = true;
            return false;
        }
        p.beta = p.beta / q;
    }
    true
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    /// The mean of a pass's pixels over `passes` passes.
    pub fn mean_image(s: &Scene, passes: usize) -> Vec<Color> {
        let flat = FlatScene::build(s).unwrap();
        let kernels = Kernels::new(s, &flat).unwrap();
        let mut sum = vec![Color::black(); s.image.width * s.image.height];
        for k in 0..passes {
            let (img, _, _) = kernels.pass(k);
            for (a, b) in sum.iter_mut().zip(img) {
                *a = *a + b;
            }
        }
        sum.into_iter().map(|c| c / passes as f64).collect()
    }

    /// Pixel completion on the CPU (`scheduler::plan`, as the GPU's
    /// `test_samples_adaptively`): with adaptive sampling the quiet pixels stop and the noisy
    /// ones carry on, each taking its own next samples; the picture is the one from sampling
    /// every pixel in full; and a render stopped part-way and resumed from its film ends
    /// where an uninterrupted one does, pixel for pixel.
    #[test]
    fn test_samples_adaptively() {
        let mut s = SceneFile::from_string(r#"{
            "width": 24, "height": 16, "supersamples": 64, "background": [0.2, 0.25, 0.3], "max_depth": 6,
            "path_termination": "russian_roulette",
            "sun": { "direction": [-1, 2, -1], "intensity": 2 },
            "camera": { "location": [0, 1.5, -5], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {
                "F": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] },
                "P": { "type": "plastic", "albedo": [0.7, 0.2, 0.1], "roughness": 0.3 },
                "E": { "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "F" },
                { "type": "sphere", "location": [-1.2, 0.5, 0], "radius": 0.5, "material": "P" },
                { "type": "torus", "center": [1, 0.4, 0], "major": 0.6, "minor": 0.2, "material": "F" },
                { "type": "sphere", "location": [0, 2.5, 1], "radius": 0.3, "material": "E" }
            ]
        }"#.to_string());
        let (w, h) = (s.image.width, s.image.height);
        let film = |s: &Scene, rc: RenderContext| {
            let flat = FlatScene::build(s).unwrap();
            let rc = Mutex::new(rc);
            crate::rendercontext::render(s, &rc, Some(&Kernels::new(s, &flat).unwrap()), |_| None::<()>, |_| ());
            rc.into_inner().unwrap()
        };
        // A scene set up twice is laid out the same, to the bit.
        {
            let flat = FlatScene::build(&s).unwrap();
            let (a, b) = (Kernels::new(&s, &flat).unwrap(), Kernels::new(&s, &flat).unwrap());
            assert!(a.buffers.tables.iter().zip(&b.buffers.tables).all(|(x, y)| x.to_bits() == y.to_bits()), "the kernels' tables differ between two builds");
        }
        let full = film(&s, RenderContext::new(w, h, false, "test.png"));
        assert!((0..w * h).all(|i| full.samples_at(i % w, i / w) == 64));
        // A block's sums are its passes': sample k of a pixel is the same wherever it is taken.
        let passes = mean_image(&s, 64);
        for i in 0..w * h {
            let (a, b) = (full.get_pixel(i % w, i / w).rgb, passes[i].rgb);
            assert!((a - b).norm() < 1e-9 * (1. + b.norm()), "pixel {}: in blocks {:?}, in passes {:?}", i, a, b);
        }

        s.render.adaptive = Some(crate::scene::Adaptive { threshold: 0.05, min_samples: 8 });
        let adaptive = film(&s, RenderContext::new(w, h, false, "test.png"));
        let counts: Vec<usize> = (0..w * h).map(|i| adaptive.samples_at(i % w, i / w)).collect();
        let (least, most) = (*counts.iter().min().unwrap(), *counts.iter().max().unwrap());
        assert!(least >= 8 && most <= 64 && least < most, "samples from {} to {}", least, most);
        assert!(counts.iter().sum::<usize>() < w * h * 64 * 9 / 10, "{} samples of {}", counts.iter().sum::<usize>(), w * h * 64);
        let mean = |rc: &RenderContext| (0..w * h).fold(Vector3::zeros(), |a, i| a + rc.get_pixel(i % w, i / w).rgb) / (w * h) as f64;
        let (a, f) = (mean(&adaptive), mean(&full));
        for ch in 0..3 {
            assert!((a[ch] - f[ch]).abs() < 0.03 * f[ch], "channel {}: adaptive {} vs full {}", ch, a[ch], f[ch]);
        }

        s.render.supersamples = 16;
        let part = film(&s, RenderContext::new(w, h, false, "test.png"));
        s.render.supersamples = 64;
        let resumed = film(&s, part);
        for i in 0..w * h {
            let (x, y) = (i % w, i / w);
            assert_eq!(resumed.samples_at(x, y), adaptive.samples_at(x, y), "pixel {}'s samples", i);
            assert_eq!(resumed.get_pixel(x, y), adaptive.get_pixel(x, y), "pixel {}", i);
        }
    }

    /// The render's frame is about the camera (`kernels::origin`), so where in the world a
    /// scene stands doesn't matter to the kernels' f32: the same scene 7,000 km from the
    /// world's origin, where a float's steps are half a metre, is the same picture pixel for
    /// pixel; primitives, a plane, lights, an emitter, plastic and glass. (Laid out about the
    /// world's origin instead, its primitives are on a float's grid there.)
    ///
    /// Only the kernels' part of a scene: what the scene's own code does, it does in the
    /// world's frame, in f64, and not all of it the same everywhere. Found with this test: a
    /// torus (the host's to intersect: a quartic in the world's coordinates) and rough glass
    /// (the host's to shade: its rays start off the surface by a fraction of the point's
    /// distance from the world's origin, `Intersection::spawn_offset`) each change a tenth of
    /// this picture's pixels out there. Moving a shape or a material into the kernels is what
    /// mends that.
    #[test]
    fn test_a_scene_far_from_the_origin_is_the_scene_at_it() {
        let scene = |d: [f64; 3]| {
            let at = |p: [f64; 3]| format!("[{}, {}, {}]", p[0] + d[0], p[1] + d[1], p[2] + d[2]);
            SceneFile::from_string(format!(r#"{{
                "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 5, "path_termination": "russian_roulette",
                "sun": {{ "direction": [-1, 2, -1], "intensity": 2 }},
                "lights": [ {{ "location": {light}, "intensity": 4 }} ],
                "camera": {{ "location": {eye}, "lookat": {look}, "up": [0, 1, 0], "angle": 0.6, "aperture": 0 }},
                "materials": {{
                    "F": {{ "type": "lambertian", "albedo": [0.6, 0.6, 0.6] }},
                    "P": {{ "type": "plastic", "albedo": [0.7, 0.2, 0.1], "roughness": 0.3 }},
                    "R": {{ "type": "dielectric", "refractive_index": 1.5 }},
                    "E": {{ "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }}
                }},
                "objects": [
                    {{ "type": "plane", "y": {floor}, "material": "F" }},
                    {{ "type": "sphere", "location": {ball}, "radius": 0.5, "material": "P" }},
                    {{ "type": "sphere", "location": {glass}, "radius": 0.4, "material": "R" }},
                    {{ "type": "box", "min": {lo}, "max": {hi}, "material": "F" }},
                    {{ "type": "sphere", "location": {lamp}, "radius": 0.3, "material": "E" }}
                ]
            }}"#, light = at([1., 3., -1.]), eye = at([0., 1.5, -5.]), look = at([0., 0.5, 0.]), floor = d[1], ball = at([-1.2, 0.5, 0.]),
                glass = at([0., 0.4, -0.8]), lo = at([-0.4, 0., 0.6]), hi = at([0.4, 0.8, 1.4]), lamp = at([0., 2.5, 1.])))
        };
        let (near, far) = (scene([0., 0., 0.]), scene([3_000_000., 200_000., -7_000_000.]));
        let (a, b) = (mean_image(&near, 32), mean_image(&far, 32));
        let (w, h) = (near.image.width, near.image.height);
        let differing = (0..w * h).filter(|&i| (a[i].rgb - b[i].rgb).norm() > 1e-3 * (0.05 + a[i].rgb.norm())).count();
        assert_eq!(differing, 0, "{} of {} pixels differ", differing, w * h);
        let (ta, tb) = (tiles(&a, w, h, 3, 2), tiles(&b, w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((ta[t][ch] - tb[t][ch]).abs() < 0.01 * ta[t][ch] + 1e-4, "tile {} channel {}: at the origin {} vs far from it {}", t, ch, ta[t][ch], tb[t][ch]);
            }
        }
        // And about the world's origin, the far scene's primitives are on a float's grid there.
        let flat = FlatScene::build(&far).unwrap();
        let world = crate::kernels::Buffers::build_at(&far, &flat, &Vector3::zeros()).unwrap();
        let local = crate::kernels::Buffers::build(&far, &flat).unwrap();
        let fine = |b: &crate::kernels::Buffers| b.prim_records.iter().filter(|p| p.info.x == pathtrace::PRIM_TRIANGLE)
            .any(|p| (p.v0.x.fract() != 0. && (p.v0.x * 2.).fract() != 0.) || (p.v1.x * 2.).fract() != 0. || (p.v2.x * 2.).fract() != 0.);
        assert!(fine(&local) && !fine(&world));
    }

    #[test]
    fn test_white_furnace() {
        // White diffuse objects under a white sky: every pixel white.
        let s = SceneFile::from_string(r#"{
            "width": 16, "height": 16, "background": [1, 1, 1], "max_depth": 1000, "path_termination": "russian_roulette",
            "camera": { "location": [0, 1, -4], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": { "W": { "type": "lambertian", "albedo": [1, 1, 1] } },
            "objects": [ { "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "W" },
                         { "type": "box", "min": [1.2, -1, -0.5], "max": [2, 0, 0.5], "material": "W" } ]
        }"#.to_string());
        // Sky sampling and BSDF sampling, combined by MIS, make each estimate noisy, though
        // their mean is exactly one.
        let image = mean_image(&s, 64);
        let mean = image.iter().map(|c| c.rgb.x).sum::<f64>() / image.len() as f64;
        assert!((mean - 1.).abs() < 0.01, "{}", mean);
        // (A pixel's own noise is that of the kernels' hashed random numbers, wider than the
        // Sobol' points the stages here once drew.)
        for c in image {
            assert!((c.rgb.x - 1.).abs() < 0.3, "{:?}", c.rgb);
        }
    }

    /// The image's mean radiance in each of `nx` × `ny` tiles.
    pub fn tiles(image: &[Color], w: usize, h: usize, nx: usize, ny: usize) -> Vec<Vector3<f64>> {
        let mut out = vec![Vector3::zeros(); nx * ny];
        for (i, c) in image.iter().enumerate() {
            let (x, y) = (i % w, i / w);
            out[(y * ny / h) * nx + x * nx / w] += c.rgb / ((w / nx) * (h / ny)) as f64;
        }
        out
    }

    /// What the review of the flat scene found missing from the tests: a two-sided plane light,
    /// a one-sided quad light under a mirroring transform, a sphere light small and far enough
    /// to trouble f32 (out of the camera's view: a camera ray that hit it would dominate its
    /// tile), and glass.
    pub fn hard_scene() -> Scene {
        SceneFile::from_string(r#"{
            "width": 16, "height": 16, "background": [0, 0, 0], "max_depth": 6, "path_termination": "russian_roulette",
            "camera": { "location": [0, 1.5, -5], "lookat": [0, 1, 0], "up": [0, 1, 0], "angle": 0.7 },
            "materials": {
                "F": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] },
                "G": { "type": "dielectric", "refractive_index": 1.5 },
                "SKY": { "type": "diffuse-light", "color": [0.2, 0.25, 0.3], "intensity": 1 },
                "QUAD": { "type": "diffuse-light", "color": [1, 0.8, 0.6], "intensity": 3 },
                "STAR": { "type": "diffuse-light", "color": [1, 1, 1], "intensity": 4e8 }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "F" },
                { "type": "plane", "y": 8, "material": "SKY" },
                { "type": "transform", "item": { "type": "quad", "points": [[-1, 3, -1], [-1, 3, 1], [1, 3, -1], [1, 3, 1]] },
                  "scale": [-1, 1, 1], "material": "QUAD" },
                { "type": "sphere", "location": [0, 0.6, 0], "radius": 0.6, "material": "G" },
                { "type": "sphere", "location": [-1.8, 0.5, 0.5], "radius": 0.5, "material": "F" },
                { "type": "sphere", "location": [-40, 7, -30], "radius": 0.01, "material": "STAR" }
            ]
        }"#.to_string())
    }

    #[test]
    fn test_agrees_with_the_path_tracer_on_hard_cases() {
        let s = hard_scene();
        let n = 256;
        let wavefront = mean_image(&s, n);
        let (w, h) = (s.image.width, s.image.height);
        let mut path = vec![Color::black(); w * h];
        for (index, p) in path.iter_mut().enumerate() {
            let (x, y) = (index % w, index / w);
            for k in 0..n {
                crate::geometry::seed_rand(9, (k * w * h + index) as u64);
                let (u, v) = (crate::geometry::rand(), crate::geometry::rand());
                let (ray, weight) = s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, u / w as f64, v / h as f64).unwrap();
                *p = *p + crate::trace::trace(&ray, 0, &s).1 * (weight / n as f64);
            }
        }
        let (a, b) = (tiles(&wavefront, w, h, 2, 2), tiles(&path, w, h, 2, 2));
        for t in 0..4 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: wavefront {} vs path {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// The recursive tracer's mean image over `n` samples a pixel.
    pub fn path_image(s: &Scene, n: usize) -> Vec<Color> {
        let (w, h) = (s.image.width, s.image.height);
        let mut path = vec![Color::black(); w * h];
        for (index, p) in path.iter_mut().enumerate() {
            let (x, y) = (index % w, index / w);
            for k in 0..n {
                crate::geometry::seed_rand(9, (k * w * h + index) as u64);
                let (u, v) = (crate::geometry::rand(), crate::geometry::rand());
                let (ray, weight) = s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, u / w as f64, v / h as f64).unwrap();
                *p = *p + crate::trace::trace(&ray, 0, s).1 * (weight / n as f64);
            }
        }
        path
    }

    /// Materials that aren't data (layered, rough glass, a texture) no longer stop a scene
    /// being flattened: they are host materials, shaded by the scene's own code, among
    /// materials the stages shade, under every kind of light (a sun, a point light, an area
    /// light, the sky), whose weights the two kinds of vertex must agree on.
    #[test]
    fn test_host_materials_agree_with_the_path_tracer() {
        let s = host_scene();
        let flat = FlatScene::build(&s).unwrap();
        assert_eq!(flat.hosts.len(), 3);
        let n = 256;
        let (_, _, stats) = Kernels::new(&s, &flat).unwrap().pass(0);
        assert!(stats.stage(Stage::Host).items > 0 && stats.stage(Stage::Shade).items > 0);
        let (w, h) = (s.image.width, s.image.height);
        let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: wavefront {} vs path {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// Media beside vertices the host shades (a layered floor, a torus of its own code): the
    /// host's shadow rays cross the medium by the scene's own code with the path's state
    /// there (`kernels::host::volume_state`: truncated once the path has left a vertex, and as
    /// its last scattering in the medium left it), so the picture is the path tracer's, with
    /// delta-M truncation alone, with the similarity relations and with the Oz approximation.
    #[test]
    fn test_media_beside_host_vertices_agree_with_the_path_tracer() {
        for density in [r#""density": 1.5"#, r#""density": 6, "similarity_after": 2"#, r#""density": 6, "oz": true"#] {
            let s = SceneFile::from_string(format!(r#"{{
                "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 24, "path_termination": "russian_roulette",
                "sun": {{ "direction": [-1, 2, -1], "intensity": 2 }},
                "lights": [ {{ "location": [1, 3, -1], "intensity": 4 }} ],
                "camera": {{ "location": [0, 1.5, -5], "lookat": [0, 0.6, 0], "up": [0, 1, 0], "angle": 0.6 }},
                "materials": {{
                    "F": {{ "type": "coated_diffuse", "albedo": [0.6, 0.6, 0.6], "roughness": 0.3 }},
                    "M": {{ "type": "lambertian", "albedo": [0.7, 0.5, 0.3] }},
                    "E": {{ "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }}
                }},
                "objects": [
                    {{ "type": "quad", "points": [[-6, 0, -6], [-6, 0, 6], [6, 0, -6], [6, 0, 6]], "material": "F" }},
                    {{ "type": "torus", "location": [1.5, 0.3, 0], "major": 0.5, "minor": 0.2, "material": "M" }},
                    {{ "type": "sphere", "location": [-0.3, 1.6, -0.2], "radius": 0.25, "material": "M" }},
                    {{ "type": "quad", "points": [[-0.5, 2.5, 0.5], [0.5, 2.5, 0.5], [-0.5, 2.5, 1.5], [0.5, 2.5, 1.5]], "material": "E" }},
                    {{ "type": "volume", "min": [-1.2, 0.2, -0.8], "max": [0.8, 1.3, 0.8], {density}, "albedo": [0.9, 0.8, 0.7], "g": 0.8 }}
                ]
            }}"#, density = density));
            let flat = FlatScene::build(&s).unwrap();
            assert!(flat.media.len() == 1 && !flat.hosts.is_empty() && !flat.shapes.is_empty());
            let n = 512;
            let (_, _, stats) = Kernels::new(&s, &flat).unwrap().pass(0);
            assert!(stats.stage(Stage::Host).items > 0);
            let (w, h) = (s.image.width, s.image.height);
            let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
            for t in 0..6 {
                for ch in 0..3 {
                    assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "{}: tile {} channel {}: wavefront {} vs path {}", density, t, ch, a[t][ch], b[t][ch]);
                }
            }
        }
    }

    /// A cloud over a sea, lit from above by the sun and from below by its glitter: the
    /// kernels' vertices in the cloud send sun probes and glitter probes
    /// (`pathtrace::send_probe`, `send_glitter_probe`) as the path tracer's do, and share the
    /// light with them, so the picture is the path tracer's, tile by tile; with the phase
    /// function whole, and with the Oz approximation, under which only the glitter is probed.
    #[test]
    fn test_probes_agree_with_the_path_tracer() {
        for scattering in [r#""density": 0.6"#, r#""density": 0.6, "oz": true"#] {
            let s = SceneFile::from_string(format!(r#"{{
                "width": 24, "height": 16, "background": [0.3, 0.4, 0.6], "max_depth": 24, "path_termination": "russian_roulette",
                "sun": {{ "direction": [-0.4, 1, 0.6], "intensity": 3 }},
                "camera": {{ "location": [0, 1.2, -14], "lookat": [0, 2.6, 0], "up": [0, 1, 0], "angle": 0.8 }},
                "objects": [
                    {{ "type": "ocean", "wind": [7, 2], "choppiness": 0, "whitecaps": 0, "planet_radius": 0, "detail_center": [0, -6], "detail_distance": 30 }},
                    {{ "type": "volume", "min": [-9, 3, -6], "max": [9, 5, 8], {scattering}, "albedo": [0.95, 0.95, 0.95], "g": 0.85 }}
                ]
            }}"#, scattering = scattering));
            let flat = FlatScene::build(&s).unwrap();
            assert!(flat.media.len() == 1 && flat.media[0].sun_probes && s.sea.is_some());
            let buffers = crate::kernels::Buffers::build(&s, &flat).unwrap();
            assert!(f32::from_bits(buffers.params.media.z) > 0., "the kernels' water, for glitter probes to find");
            let n = 768;
            let (w, h) = (s.image.width, s.image.height);
            let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
            for t in 0..6 {
                for ch in 0..3 {
                    assert!((a[t][ch] - b[t][ch]).abs() < 0.04 * b[t][ch] + 1e-3, "{}: tile {} channel {}: wavefront {} vs path {}", scattering, t, ch, a[t][ch], b[t][ch]);
                }
            }
        }
    }

    /// Crepuscular rays (`pathtrace::crepuscular_samples`): under a slab that shadows the air, with
    /// a gap the sun shines through, the sky and the haze before the ground lose what the
    /// path tracer takes from them (`SkyMaterial::shadowed_loss`), tile by tile; and they lose
    /// something, or this tests nothing.
    #[test]
    fn test_crepuscular_rays_agree_with_the_path_tracer() {
        let scene = |rays: bool| SceneFile::from_string(format!(r#"{{
            "width": 24, "height": 16, "max_depth": 3, "sun": {{ "intensity": 3 }},
            "camera": {{ "location": [0, 300, -6000], "lookat": [0, 1500, 0], "up": [0, 1, 0], "angle": 0.9, "aperture": 0 }},
            "materials": {{ "F": {{ "type": "lambertian", "albedo": [0.3, 0.3, 0.3] }} }},
            "objects": [
                {{ "type": "skysphere", "sun_direction": [0.3, 0.5, 1], "crepuscular_rays": {rays} }},
                {{ "type": "quad", "points": [[-40000, 0, -40000], [-40000, 0, 40000], [40000, 0, -40000], [40000, 0, 40000]], "material": "F" }},
                {{ "type": "box", "min": [-30000, 4000, -30000], "max": [-600, 4200, 30000], "material": "F" }},
                {{ "type": "box", "min": [600, 4000, -30000], "max": [30000, 4200, 30000], "material": "F" }}
            ]
        }}"#, rays = rays));
        let (s, plain) = (scene(true), scene(false));
        assert!(s.sun.is_some() && s.atmosphere.as_ref().unwrap().crepuscular_rays);
        let (w, h) = (s.image.width, s.image.height);
        let n = 256;
        let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
        let without = tiles(&mean_image(&plain, n), w, h, 3, 2);
        let mut lost = 0.;
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.04 * b[t][ch] + 1e-4, "tile {} channel {}: wavefront {} vs path {}", t, ch, a[t][ch], b[t][ch]);
            }
            lost += (without[t][2] - a[t][2]) / without[t][2] / 6.;
        }
        assert!(lost > 0.03, "the shadowed air takes {} of the picture's blue", lost);
    }

    /// Shapes that aren't primitives (a torus, a cone) are kept whole and intersected by their
    /// own code, among primitives, whether their materials are the stages' or their own; they
    /// shadow and are shadowed as the rest.
    #[test]
    fn test_shapes_their_own_code_intersects_agree_with_the_path_tracer() {
        let s = SceneFile::from_string(r#"{
            "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 6, "path_termination": "russian_roulette",
            "sun": { "direction": [-1, 2, -1], "intensity": 2 },
            "lights": [ { "location": [1, 3, -1], "intensity": 4 } ],
            "camera": { "location": [0, 2, -5], "lookat": [0, 0.4, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {
                "F": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] },
                "P": { "type": "coated_diffuse", "albedo": [0.7, 0.2, 0.1], "roughness": 0.2 },
                "C": { "type": "conductor", "metal": "Au", "roughness": 0.2 }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "F" },
                { "type": "torus", "center": [-0.9, 0.3, 0], "major": 0.7, "minor": 0.3, "material": "C" },
                { "type": "cone", "base": [1, 0, 0], "cap": [1, 1.2, 0], "base_radius": 0.6, "cap_radius": 0, "material": "P" },
                { "type": "sphere", "location": [0, 0.3, -1.2], "radius": 0.3, "material": "F" }
            ]
        }"#.to_string());
        let flat = FlatScene::build(&s).unwrap();
        assert_eq!((flat.shapes.len(), flat.hosts.len()), (2, 1));
        let n = 256;
        let (w, h) = (s.image.width, s.image.height);
        let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: wavefront {} vs path {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// Three host materials (a texture, a layered one, rough glass) among two the stages
    /// shade and an area light, under a sun, a point light and a sky.
    pub fn host_scene() -> Scene {
        SceneFile::from_string(r#"{
            "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 6, "path_termination": "russian_roulette",
            "sun": { "direction": [-1, 2, -1], "intensity": 2 },
            "lights": [ { "location": [1, 3, -1], "intensity": 4 } ],
            "camera": { "location": [0, 1.5, -5], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {
                "F": { "type": "checker", "colors": [[0.7, 0.7, 0.7], [0.3, 0.3, 0.4]], "scale": [4, 4] },
                "P": { "type": "coated_diffuse", "albedo": [0.7, 0.2, 0.1], "roughness": 0.2 },
                "G": { "type": "rough_dielectric", "refractive_index": 1.5, "roughness": 0.3 },
                "C": { "type": "conductor", "metal": "Au", "roughness": 0.2 },
                "D": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] },
                "E": { "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }
            },
            "objects": [
                { "type": "quad", "points": [[-6, 0, -6], [-6, 0, 6], [6, 0, -6], [6, 0, 6]], "material": "F" },
                { "type": "sphere", "location": [-1.2, 0.5, 0], "radius": 0.5, "material": "P" },
                { "type": "sphere", "location": [0, 0.5, 0], "radius": 0.5, "material": "G" },
                { "type": "sphere", "location": [1.2, 0.5, 0], "radius": 0.5, "material": "C" },
                { "type": "sphere", "location": [0, 0.3, -1.2], "radius": 0.3, "material": "D" },
                { "type": "quad", "points": [[-0.5, 2.5, 0.5], [0.5, 2.5, 0.5], [-0.5, 2.5, 1.5], [0.5, 2.5, 1.5]], "material": "E" }
            ]
        }"#.to_string())
    }

    /// The kernels' Sobol' points are the renderer's (`sobol`), computed without its tables;
    /// and a pixel's samples of a decision are stratified: of 64, one in each 64th of each of
    /// the first two coordinates, as a (0, 2)-sequence's are, however scrambled.
    #[test]
    fn test_the_kernels_sobol_points_are_the_renderers() {
        let mut rng = crate::geometry::Pcg32::new(3, 5);
        let directions: Vec<f32> = pathtrace::sobol_directions().iter().flatten().map(|&v| f32::from_bits(v)).collect();
        for i in 0..20_000u32 {
            let index = if i < 4096 { i } else { rng.next_u32() };
            let x = pathtrace::sobol4(&directions, 0, index);
            for (d, x) in [x.x, x.y, x.z, x.w].into_iter().enumerate() {
                assert_eq!(x, crate::sobol::sobol(index, d), "point {} dimension {}", index, d);
            }
            let seed = rng.next_u32();
            assert_eq!(pathtrace::owen_scramble(index, seed), crate::sobol::owen_scramble(index, seed));
        }
        let s = SceneFile::from_string(r#"{ "width": 4, "height": 4, "background": [0, 0, 0],
            "camera": { "location": [0, 0, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": { "F": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] } },
            "objects": [ { "type": "sphere", "location": [0, 0, 5], "radius": 1, "material": "F" } ] }"#.to_string());
        let flat = FlatScene::build(&s).unwrap();
        let buffers = crate::kernels::Buffers::build(&s, &flat).unwrap();
        let scene = buffers.scene(None);
        for (pixel, depth, which) in [(0, 0, pathtrace::SOBOL_SCATTER), (7, 2, pathtrace::SOBOL_LIGHT), (11, 5, pathtrace::SOBOL_SKY)] {
            let (mut xs, mut ys) = ([false; 64], [false; 64]);
            for sample in 0..64 {
                let u = pathtrace::decide(&scene, pixel, sample, depth, which, &mut 0);
                assert!((0. ..1.).contains(&u.x) && (0. ..1.).contains(&u.y) && (0. ..1.).contains(&u.z) && (0. ..1.).contains(&u.w));
                let (x, y) = ((u.x * 64.) as usize, (u.y * 64.) as usize);
                assert!(!xs[x] && !ys[y], "pixel {} depth {}: two samples in a stratum", pixel, depth);
                (xs[x], ys[y]) = (true, true);
            }
        }
    }

    /// A checkerboard's squares are where the world has them, not about the camera (whose
    /// frame the kernels compute in): from a camera an odd number of squares from the origin,
    /// each tile of the picture is the path tracer's.
    #[test]
    fn test_a_checkerboard_is_laid_out_in_the_world() {
        let s = SceneFile::from_string(r#"{
            "width": 24, "height": 16, "background": [0.5, 0.5, 0.5], "max_depth": 3,
            "camera": { "location": [7, 9, -15], "lookat": [7, 0, -14.9], "up": [0, 0, 1], "angle": 0.5, "aperture": 0 },
            "materials": {
                "A": { "type": "lambertian", "albedo": [0.9, 0.9, 0.9] },
                "B": { "type": "lambertian", "albedo": [0.05, 0.05, 0.05] }
            },
            "media": { "CHECKS": { "type": "checkered-y-plane", "m1": "A", "m2": "B", "xsize": 5, "zsize": 5 } },
            "objects": [ { "type": "checkeredplane", "y": 0, "medium": "CHECKS" } ]
        }"#.to_string());
        let n = 256;
        let kernels = mean_image(&s, n);
        let (w, h) = (s.image.width, s.image.height);
        let path: Vec<Color> = (0..w * h).map(|index| {
            let (x, y) = (index % w, index / w);
            let mut sum = Color::black();
            for k in 0..n {
                crate::geometry::seed_rand(7, (k * w * h + index) as u64);
                let (u, v) = (crate::geometry::rand(), crate::geometry::rand());
                let (ray, weight) = s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, u / w as f64, v / h as f64).unwrap();
                sum = sum + crate::trace::trace(&ray, 0, &s).1 * weight;
            }
            sum / n as f64
        }).collect();
        let (a, b) = (tiles(&kernels, w, h, 6, 4), tiles(&path, w, h, 6, 4));
        // (The picture has both kinds of square, or it tests nothing.)
        let brightness: Vec<f64> = b.iter().map(|t| t[0]).collect();
        assert!(brightness.iter().cloned().fold(0., f64::max) > 3. * brightness.iter().cloned().fold(f64::INFINITY, f64::min), "{:?}", brightness);
        for t in 0..a.len() {
            assert!((a[t][0] - b[t][0]).abs() < 0.1 * b[t][0] + 1e-3, "tile {}: the kernels {} vs the path tracer {}", t, a[t][0], b[t][0]);
        }
    }

    /// The wavefront integrator and the recursive tracer estimate the same image.
    #[test]
    fn test_agrees_with_the_path_tracer() {
        let s = SceneFile::from_string(r#"{
            "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 6, "path_termination": "russian_roulette",
            "sun": { "direction": [-1, 2, -1], "intensity": 2 },
            "lights": [ { "location": [1, 3, -1], "intensity": 4 } ],
            "camera": { "location": [0, 1.5, -5], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {
                "F": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] },
                "P": { "type": "plastic", "albedo": [0.7, 0.2, 0.1], "roughness": 0.3 },
                "G": { "type": "dielectric", "refractive_index": 1.5 },
                "C": { "type": "conductor", "metal": "Au", "roughness": 0.2 },
                "E": { "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "F" },
                { "type": "sphere", "location": [-1.2, 0.5, 0], "radius": 0.5, "material": "P" },
                { "type": "sphere", "location": [0, 0.5, 0], "radius": 0.5, "material": "G" },
                { "type": "sphere", "location": [1.2, 0.5, 0], "radius": 0.5, "material": "C" },
                { "type": "sphere", "location": [0, 2.5, 1], "radius": 0.3, "material": "E" }
            ]
        }"#.to_string());
        let n = 256;
        let wavefront = mean_image(&s, n);
        // The recursive tracer, through the same camera rays' pixels.
        let (w, h) = (s.image.width, s.image.height);
        let mut total = (Vector3::zeros(), Vector3::zeros());
        for (index, c) in wavefront.iter().enumerate() {
            let (x, y) = (index % w, index / w);
            let mut sum = Color::black();
            for k in 0..n {
                crate::geometry::seed_rand(7, (k * w * h + index) as u64);
                let (u, v) = (crate::geometry::rand(), crate::geometry::rand());
                let (ray, weight) = s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, u / w as f64, v / h as f64).unwrap();
                sum = sum + crate::trace::trace(&ray, 0, &s).1 * weight;
            }
            total.0 += c.rgb;
            total.1 += (sum / n as f64).rgb;
        }
        for ch in 0..3 {
            assert!((total.0[ch] - total.1[ch]).abs() < 0.03 * total.1[ch], "channel {}: wavefront {} vs path {}", ch, total.0[ch], total.1[ch]);
        }
    }
}
