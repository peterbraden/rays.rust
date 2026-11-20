//! The wavefront path tracer (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th
//! ed., chapter 15; cited below as PBR).
//!
//! The recursive tracer (`trace`) follows one path at a time to its end, branching on what it
//! hits through trait objects. A wavefront renderer instead keeps every path of a pass (one
//! sample per pixel) in a queue and advances them all together, one stage at a time, each
//! stage a single loop over many paths doing one kind of work (PBR section 15.2):
//!
//! 1. Generate camera rays.
//! 2. Intersect: the nearest hit of every path's ray.
//! 3. Escaped rays and emitters: add the background or the light hit (weighted by MIS against
//!    the light sampling of the vertex before), and retire those paths.
//! 4. Shade: sorted by material kind, so neighbouring work runs the same code (pbrt keeps a
//!    queue per material); sample the lights (next-event estimation), queueing a shadow ray
//!    per light sample, then sample the BSDF for the path's next ray, or retire it. Paths on
//!    a material that isn't data sort last, and the scene's own material answers each
//!    (`host_bounce`; `flat::FlatMaterial::Host`).
//! 5. Trace the shadow rays, adding what each carries if nothing blocks it.
//!
//! and back to 2 until no path is left. Each path carries its own state (throughput, the last
//! BSDF density for MIS, its Sobol' sample), as pbrt's do, since no stage may rely on the
//! thread-local state a recursive path keeps (`geometry::rand`'s sampler, the wavelengths).
//! The structure is what a GPU renderer needs, with the scene as data (`flat`). On the CPU,
//! the pass's pixels are split into batches of a few thousand paths, run in parallel, each
//! through all the stages: a whole image's queue streamed through memory at every stage made
//! the stages memory-bound, twice as slow as the recursive tracer.
//!
//! Its estimates match the recursive tracer's: the same BSDFs (`material::bsdf`, `ggx`,
//! `plastic`), the same Fresnel choices, light sampling by power with the power heuristic, and
//! the same depth limit and Russian roulette, sphere lights sampled by their cone, the sun's
//! disc for lobes narrow enough to resolve it, the sky by its brightness, area lights by power
//! or the light BVH, glossy lobes regularised after the first non-delta bounce
//! (`ggx::regularised`), and each path's random numbers from its own Sobol' sample. Differences:
//! only the Sobol' sampler; point lights are always all sampled (the light BVH chooses among
//! area lights only).

use crate::intersection::Intersection;
use crate::material::model::LightSampling;
use crate::scheduler::{Stage, Stats};
use crate::color::Color;
use crate::shapes::geometry::Geometry;
use crate::material::bsdf::{power_heuristic, LambertianBSDF, BSDF};
use crate::material::functions::{fresnel_reflectance, reflect, refract};
use crate::material::ggx::{Fresnel, GGX, MIN_ALPHA};
use crate::material::plastic::PlasticBSDF;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::rendercontext::RenderContext;
use crate::scene::{PathTermination, Scene};
use crate::wavefront::flat::{FlatMaterial, FlatScene, Hit};
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
}

impl Path {
    fn u(&mut self) -> f64 {
        self.sampler.next()
    }
}

/// A shadow ray from shading: the path it belongs to (its place in the queue), and what it
/// adds if unblocked.
struct Shadow {
    path: usize,
    ray: Ray,
    max: f64,
    contribution: Color,
}

/// How far to start a ray off a surface at `p`, to clear rounding in the hit.
fn offset(p: &Vector3<f64>, s: &Scene) -> f64 {
    s.render.shadow_bias.max(1e-7 * (1. + p.amax())) * 2.
}

/// A material's BSDF at a hit, where it has one (non-delta).
enum Local {
    Lambert(LambertianBSDF),
    Plastic(PlasticBSDF),
    Ggx(GGX),
}

impl Local {
    fn bsdf(&self) -> &dyn BSDF {
        match self {
            Local::Lambert(b) => b,
            Local::Plastic(b) => b,
            Local::Ggx(b) => b,
        }
    }
}

/// Paths per batch: a queue small enough to stay in cache through every stage, and enough
/// batches to keep every thread busy.
const BATCH: usize = 4096;

/// Render sample `k` of every pixel: each pixel's radiance (times its weight), and the number
/// of rays cast.
pub fn render_pass(s: &Scene, flat: &FlatScene, k: usize) -> (Vec<Color>, u64) {
    let (image, rays, _) = render_pass_with_stats(s, flat, k);
    (image, rays)
}

/// `render_pass`, with what its stages did (`scheduler::Stats`).
pub fn render_pass_with_stats(s: &Scene, flat: &FlatScene, k: usize) -> (Vec<Color>, u64, Stats) {
    let (w, h) = (s.image.width, s.image.height);
    let mut image = vec![Color::black(); w * h];
    // Batches of pixels run in parallel, each through all the stages: a queue that streams
    // through memory once per stage would make the stages memory-bound on a CPU (on a GPU,
    // the whole image's queue is the batch).
    let batches: Vec<_> = (0..(w * h).div_ceil(BATCH)).into_par_iter()
        .map(|b| render_batch(s, flat, k, b * BATCH..((b + 1) * BATCH).min(w * h)))
        .collect();
    let mut rays = 0;
    let mut stats = Stats::default();
    stats.passes = 1;
    for (done, n, batch_stats) in batches {
        rays += n;
        stats.merge(&batch_stats);
        for (pixel, c) in done {
            image[pixel as usize] = image[pixel as usize] + c;
        }
    }
    (image, rays, stats)
}

/// Sample `k` of the pixels `range`, stage by stage: each finished path's pixel and radiance,
/// and the number of rays cast.
fn render_batch(s: &Scene, flat: &FlatScene, k: usize, range: std::ops::Range<usize>) -> (Vec<(u32, Color)>, u64, Stats) {
    let (w, h) = (s.image.width, s.image.height);
    let pixels = (w * h) as u64;
    // Each stage's batch is counted and timed (`scheduler::Stats`): two clock readings a
    // stage a round, against thousands of paths.
    let mut stats = Stats::default();
    let mut clock = web_time::Instant::now();
    let mut lap = |stats: &mut Stats, stage: Stage, items: usize| {
        let now = web_time::Instant::now();
        stats.record(stage, items as u64, (now - clock).as_secs_f64());
        clock = now;
    };
    // 1. Camera rays.
    let mut paths: Vec<Path> = range.filter_map(|index| {
        let (x, y) = (index % w, index / w);
        let stream = k as u64 * pixels + index as u64;
        // The camera's own random numbers (a lens) come from the thread's generator.
        crate::geometry::seed_rand(s.render.seed ^ 0x5bd1_e995, stream);
        // Sample k of the pixel's Sobol' sequence, as the recursive tracer's.
        let mut sampler = crate::sobol::SampleState::new(crate::sobol::hash(s.render.seed, index as u64), k as u32);
        let ((fx, wx), (fy, wy)) = (s.render.filter.sample_weighted(sampler.next()), s.render.filter.sample_weighted(sampler.next()));
        let (ray, weight) = s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64,
            (0.5 + fx) / w as f64, (0.5 + fy) / h as f64)?;
        let ray = Ray { ro: ray.ro, rd: ray.rd.normalize() };
        Some(Path { pixel: index as u32, ray, beta: Color::white(), l: Color::black(), weight: weight * wx * wy,
                    depth: 0, specular: true, prev_pdf: 0., sampler, scattered: false, lights_counted: false, directional: false, hit: None, done: false })
    }).collect();
    lap(&mut stats, Stage::Generate, paths.len());
    let mut done = Vec::with_capacity(paths.len());
    let mut shadows: Vec<Shadow> = Vec::new();
    let mut rays = 0u64;
    let mut round = 0;
    while !paths.is_empty() {
        stats.round(round, paths.len() as u64);
        round += 1;
        // 2. Intersect.
        rays += paths.len() as u64;
        for p in paths.iter_mut() {
            p.hit = flat.intersect(&p.ray, 0.);
        }
        lap(&mut stats, Stage::Intersect, paths.len());
        // 3. Escaped rays and emitters.
        for p in paths.iter_mut() {
            gather_emission(p, s, flat);
        }
        lap(&mut stats, Stage::Emission, paths.len());
        retire(&mut paths, &mut done);
        // 4. Shade, sorted by material so that each kind's code runs together.
        paths.sort_unstable_by_key(|p| flat.hit_material(p.hit.unwrap().prim).kind());
        shadows.clear();
        // Host materials sort last: the stage's own kinds, then the paths the scene's
        // materials answer, counted apart.
        let ported = paths.partition_point(|p| !matches!(flat.hit_material(p.hit.unwrap().prim), FlatMaterial::Host { .. }));
        for (i, p) in paths[..ported].iter_mut().enumerate() {
            shade(i, p, s, flat, &mut shadows);
        }
        lap(&mut stats, Stage::Shade, ported);
        if ported < paths.len() {
            for (i, p) in paths[ported..].iter_mut().enumerate() {
                shade(ported + i, p, s, flat, &mut shadows);
            }
            lap(&mut stats, Stage::Host, paths.len() - ported);
        }
        // 5. Shadow rays.
        rays += shadows.len() as u64;
        for sh in &shadows {
            if !flat.occluded(&sh.ray, 0., sh.max) {
                paths[sh.path].l = paths[sh.path].l + sh.contribution;
            }
        }
        lap(&mut stats, Stage::Shadow, shadows.len());
        retire(&mut paths, &mut done);
    }
    lap(&mut stats, Stage::Accumulate, done.len());
    (done, rays, stats)
}

/// Move finished paths' radiance to `done`, and drop them from the queue.
fn retire(paths: &mut Vec<Path>, done: &mut Vec<(u32, Color)>) {
    for p in paths.iter().filter(|p| p.done) {
        done.push((p.pixel, (p.l * p.weight).ignore_nan()));
    }
    paths.retain(|p| !p.done);
}

/// A point on an emitting primitive seen from `from`, its normal, and the density per solid
/// angle: a sphere by the cone it subtends, as the recursive tracer's (`Geometry::sample_from`),
/// a triangle by area.
fn sample_light(prim: &crate::wavefront::flat::Prim, from: &Vector3<f64>, u: [f64; 3]) -> (Vector3<f64>, Vector3<f64>, f64) {
    use crate::wavefront::flat::Prim;
    if let Prim::Sphere { center, radius } = prim {
        return crate::shapes::sphere::Sphere::new(*center, *radius).sample_from(from, u).unwrap_or((*center, Vector3::y(), 0.));
    }
    let (y, n) = prim.sample([u[0], u[1]]);
    (y, n, light_pdf(prim, from, &y, &n))
}

/// The density per solid angle with which `sample_light` from `from` picks `y`, normal `n`.
fn light_pdf(prim: &crate::wavefront::flat::Prim, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
    use crate::wavefront::flat::Prim;
    if let Prim::Sphere { center, radius } = prim {
        return crate::shapes::sphere::Sphere::new(*center, *radius).pdf_from(from, y, n);
    }
    let w = y - from;
    let d2 = w.norm_squared();
    let cos = n.dot(&w).abs() / d2.sqrt();
    if cos <= 0. { 0. } else { d2 / (cos * prim.area()) }
}

/// As `trace`: segments shorter than this cross too little air to matter.
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

/// Stage 3 for one path: the background if its ray escaped, or the light it hit.
fn gather_emission(p: &mut Path, s: &Scene, flat: &FlatScene) {
    let Some(h) = p.hit else {
        // Weighted against sampling the sky, where the vertex before did.
        let w = match &s.environment {
            Some(env) if !p.specular && !p.lights_counted => power_heuristic(p.prev_pdf, env.pdf(&p.ray.rd)),
            _ => 1.,
        };
        p.l = p.l + p.beta * sky_radiance(&p.ray, s, flat) * w;
        // The sun's disc, seen by the camera or through a mirror (other rays weigh it nearly
        // nothing against sampling it as a point; `trace`).
        if let Some(sun) = &s.sun {
            // In full after a delta bounce; otherwise against sampling the disc, as `trace`.
            let w = if p.specular { 1. } else { power_heuristic(p.prev_pdf, sun.disc_pdf(&p.ray.rd)) };
            if w > 0. {
                p.l = p.l + p.beta * sun.disc_radiance(&p.ray.ro, &p.ray.rd) * w;
            }
        }
        p.done = true;
        return;
    };
    // Aerial perspective, as `trace`: the air between attenuates what lies beyond the hit,
    // and adds its own light.
    if let Some(sky) = &s.atmosphere
        && h.dist > AERIAL_PERSPECTIVE_FROM {
        let (transmittance, inscattered) = sky.segment(&p.ray.ro, &p.ray.rd, h.dist);
        p.l = p.l + p.beta * Color { rgb: inscattered };
        p.beta = p.beta * Color { rgb: transmittance };
    }
    let m = flat.hit_material(h.prim).at(&h.point);
    if let FlatMaterial::Emitter { .. } = m {
        // The side it emits from by the normal it shades with, as `trace` (for a smooth mesh,
        // the interpolated normal, not the winding).
        let le = m.emitted(&h.shading, &-p.ray.rd);
        if le != Color::black() {
            // Weighted against light sampling, if the vertex before sampled this light.
            let sampled = flat.emitter_index.get(h.prim as usize).copied().flatten();
            let w = match sampled {
                Some(e) if !p.specular && !p.directional && s.render.sample_lights => {
                    let prim = &flat.prims[h.prim as usize];
                    let light_pdf = flat.emitter_pmf(&p.ray.ro, e as usize) * light_pdf(prim, &p.ray.ro, &h.point, &h.geometric);
                    power_heuristic(p.prev_pdf, light_pdf)
                },
                _ => 1.,
            };
            p.l = p.l + p.beta * le * w;
        }
        // Emitters don't scatter.
        p.done = true;
    }
}

/// Stage 4 for one path: next-event estimation and the next ray.
fn shade(i: usize, p: &mut Path, s: &Scene, flat: &FlatScene, shadows: &mut Vec<Shadow>) {
    let h = p.hit.unwrap();
    let wo = -p.ray.rd;
    // Shade the side the ray arrived on.
    let ng = if h.geometric.dot(&wo) >= 0. { h.geometric } else { -h.geometric };
    let ns = if h.shading.dot(&wo) >= 0. { h.shading } else { -h.shading };
    let eps = offset(&h.point, s);
    let above = h.point + ng * eps;
    // As `ggx::regularised`: a roughness floor once the path has scattered.
    let floor = if p.scattered { s.render.regularise } else { 0. };
    let local = match flat.hit_material(h.prim).at(&h.point) {
        FlatMaterial::Diffuse { albedo } => Local::Lambert(LambertianBSDF { albedo: *albedo, normal: ns }),
        FlatMaterial::Plastic { albedo, refractive_index, roughness } => {
            let alpha = (roughness * roughness).max(floor);
            if alpha >= MIN_ALPHA {
                Local::Plastic(PlasticBSDF {
                    coat: GGX::new(alpha, Fresnel::Dielectric(*refractive_index), ns),
                    base: LambertianBSDF { albedo: *albedo, normal: ns },
                    refractive_index: *refractive_index,
                })
            } else {
                // A smooth coat: a mirror with Fresnel's probability, else the diffuse base.
                let coat = fresnel_reflectance(wo.dot(&ns).abs(), 1. / refractive_index, *refractive_index);
                if p.u() < coat {
                    return specular_bounce(p, s, above, reflect(p.ray.rd, ns), Color::white());
                }
                Local::Lambert(LambertianBSDF { albedo: *albedo, normal: ns })
            }
        },
        FlatMaterial::Conductor { optics, roughness } => {
            let (eta, k) = optics.current();
            let fresnel = Fresnel::Conductor { eta, k };
            let alpha = (roughness * roughness).max(floor);
            if alpha >= MIN_ALPHA {
                Local::Ggx(GGX::new(alpha, fresnel, ns))
            } else {
                let tint = fresnel.eval(wo.dot(&ns).abs());
                return specular_bounce(p, s, above, reflect(p.ray.rd, ns), tint);
            }
        },
        FlatMaterial::Specular { albedo, roughness } => {
            let fresnel = Fresnel::Schlick(*albedo);
            let alpha = (roughness * roughness).max(floor);
            if alpha >= MIN_ALPHA {
                Local::Ggx(GGX::new(alpha, fresnel, ns))
            } else {
                let tint = fresnel.eval(wo.dot(&ns).abs());
                return specular_bounce(p, s, above, reflect(p.ray.rd, ns), tint);
            }
        },
        FlatMaterial::Dielectric { refractive_index, tint } => {
            // As `functions::scatter_dielectric`: the outward normal says whether it enters.
            let n = h.shading;
            let exiting = p.ray.rd.dot(&n) > 0.;
            let outward = if exiting { -n } else { n };
            let ni_over_nt = if exiting { *refractive_index } else { 1. / refractive_index };
            let r = fresnel_reflectance(p.ray.rd.dot(&n).abs(), ni_over_nt, *refractive_index);
            if let Some(t) = refract(p.ray.rd, outward, ni_over_nt)
                && p.u() >= r {
                let below = h.point - outward * eps;
                return specular_bounce(p, s, below, t.normalize(), *tint);
            }
            return specular_bounce(p, s, h.point + outward * eps, reflect(p.ray.rd, n), Color::white());
        },
        FlatMaterial::Host { object } => return host_bounce(p, s, flat, *object, &h),
        FlatMaterial::Emitter { .. } | FlatMaterial::Checker { .. } => unreachable!("emitters are retired before shading; checkerboards resolve to a square's material"),
    };
    let bsdf = local.bsdf();
    p.scattered = true;
    // Next-event estimation: every point light, the sun (as a point), and one area light.
    for light in &s.lights {
        let to = light.position - above;
        let d2 = to.norm_squared();
        let wi = to / d2.sqrt();
        let f = bsdf.eval(&wo, &wi);
        if f != Color::black() {
            let c = p.beta * f * light.radiant_intensity(&-wi) / d2;
            shadows.push(Shadow { path: i, ray: Ray { ro: above, rd: wi }, max: d2.sqrt(), contribution: c });
        }
    }
    // The sun, as `functions::sun_light_mis`: a point for lobes that can't resolve its disc,
    // and otherwise a direction on the disc weighted against the BSDF's density.
    if let Some(sun) = &s.sun {
        if bsdf.pdf(&wo, &sun.direction) * sun.solid_angle() < crate::material::functions::POINT_SUN {
            let f = bsdf.eval(&wo, &sun.direction);
            if f != Color::black() {
                shadows.push(Shadow { path: i, ray: Ray { ro: above, rd: sun.direction }, max: crate::light::SUN_SHADOW_DISTANCE,
                                      contribution: p.beta * f * sun.irradiance_at(&above) });
            }
        } else {
            let (wi, radiance, pdf) = sun.sample_disc_at(&above, [p.u(), p.u()]);
            let f = bsdf.eval(&wo, &wi);
            if f != Color::black() && pdf > 0. {
                let w = power_heuristic(pdf, bsdf.pdf(&wo, &wi));
                shadows.push(Shadow { path: i, ray: Ray { ro: above, rd: wi }, max: crate::light::SUN_SHADOW_DISTANCE,
                                      contribution: p.beta * f * radiance * (w / pdf) });
            }
        }
    }
    let u_light = p.u();
    if let Some((e, pmf)) = (s.render.sample_lights && !flat.emitters.is_empty()).then(|| flat.choose_emitter(&above, u_light)).flatten() {
        let prim_index = flat.emitters[e] as usize;
        let prim = &flat.prims[prim_index];
        let (y, n_y, pdf) = sample_light(prim, &above, [p.u(), p.u(), p.u()]);
        let to = y - above;
        let d = to.norm();
        let wi = to / d;
        let le = flat.material(prim_index as u32).emitted(&n_y, &-wi);
        if le != Color::black() && pdf > 0. {
            let light_pdf = pmf * pdf;
            let f = bsdf.eval(&wo, &wi);
            if f != Color::black() {
                let w = power_heuristic(light_pdf, bsdf.pdf(&wo, &wi));
                shadows.push(Shadow { path: i, ray: Ray { ro: above, rd: wi }, max: d * (1. - 1e-6) - eps,
                                      contribution: p.beta * f * le * (w / light_pdf) });
            }
        }
    }
    if let Some(env) = &s.environment {
        let (wi, pdf) = env.sample([p.u(), p.u()]);
        if pdf > 0. {
            let f = bsdf.eval(&wo, &wi);
            if f != Color::black() {
                let ray = Ray { ro: above, rd: wi };
                let w = power_heuristic(pdf, bsdf.pdf(&wo, &wi));
                shadows.push(Shadow { path: i, contribution: p.beta * f * env.radiance(&ray, s) * (w / pdf), ray, max: crate::light::SUN_SHADOW_DISTANCE });
            }
        }
    }
    // The next ray.
    if p.depth >= s.render.max_depth as u32 {
        p.done = true;
        return;
    }
    match bsdf.sample(&wo, [p.u(), p.u(), p.u()]) {
        Some(sample) => {
            if !continue_with(p, s, &sample.weight) {
                return;
            }
            p.beta = p.beta * sample.weight;
            p.prev_pdf = sample.pdf;
            p.specular = false;
            // Off the side the new ray leaves by.
            let side = if sample.wi.dot(&ng) >= 0. { ng } else { -ng };
            p.ray = Ray { ro: h.point + side * eps, rd: sample.wi.normalize() };
            p.depth += 1;
        },
        None => p.done = true,
    }
}

/// A delta bounce: weight `attenuate`, from `from` along `rd`.
fn specular_bounce(p: &mut Path, s: &Scene, from: Vector3<f64>, rd: Vector3<f64>, attenuate: Color) {
    if p.depth >= s.render.max_depth as u32 || !continue_with(p, s, &attenuate) {
        p.done = true;
        return;
    }
    p.beta = p.beta * attenuate;
    p.specular = true;
    p.ray = Ray { ro: from, rd: rd.normalize() };
    p.depth += 1;
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
fn host_bounce(p: &mut Path, s: &Scene, flat: &FlatScene, object: u32, h: &Hit) {
    let o = &flat.hosts[object as usize];
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
    pub done: bool,
}

/// `host_bounce` for a path kept elsewhere, at `hit`, drawing its numbers from `sampler`.
/// Nothing if the hit's material is one the stages shade.
pub fn host_vertex(s: &Scene, flat: &FlatScene, v: &mut HostVertex, hit: &Hit, sampler: crate::sobol::SampleState) {
    let FlatMaterial::Host { object } = flat.hit_material(hit.prim) else { return };
    let mut p = Path {
        pixel: 0, ray: v.ray, beta: v.beta, l: v.l, weight: 1., depth: v.depth, specular: v.specular, prev_pdf: v.prev_pdf, sampler,
        scattered: v.scattered, lights_counted: v.lights_counted, directional: v.directional, hit: Some(*hit), done: false,
    };
    host_bounce(&mut p, s, flat, *object, hit);
    *v = HostVertex {
        ray: p.ray, beta: p.beta, l: p.l, depth: p.depth, specular: p.specular, lights_counted: p.lights_counted, directional: p.directional, prev_pdf: p.prev_pdf,
        scattered: p.scattered, done: p.done,
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

/// Render up to the scene's samples per pixel, a pass of one sample per pixel at a time, from
/// where `rc` has got to; `after_pass` sees the context after each.
pub fn render(s: &Scene, flat: &FlatScene, rc: &Mutex<RenderContext>, mut after_pass: impl FnMut(&RenderContext)) {
    let first = rc.lock().unwrap().min_samples();
    let (w, h) = (s.image.width, s.image.height);
    for k in first..s.render.samples_per_pixel() {
        let (image, rays, stats) = render_pass_with_stats(s, flat, k);
        let mut rc = rc.lock().unwrap();
        rc.schedule.merge(&stats);
        for y in 0..h {
            for x in 0..w {
                // A pixel a resumed state has already taken past k (another integrator's
                // render) keeps what it has: sample k would repeat one of its samples.
                if rc.samples_at(x, y) <= k {
                    let c = image[y * w + x];
                    rc.set_pixel(x, y, c, Color { rgb: c.rgb.component_mul(&c.rgb) }, 1);
                }
            }
        }
        rc.rays_cast += rays;
        after_pass(&rc);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    /// The mean of a pass's pixels over `passes` passes.
    pub fn mean_image(s: &Scene, passes: usize) -> Vec<Color> {
        let flat = FlatScene::build(s).unwrap();
        let mut sum = vec![Color::black(); s.image.width * s.image.height];
        for k in 0..passes {
            let (img, _) = render_pass(s, &flat, k);
            for (a, b) in sum.iter_mut().zip(img) {
                *a = *a + b;
            }
        }
        sum.into_iter().map(|c| c / passes as f64).collect()
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
        for c in image {
            assert!((c.rgb.x - 1.).abs() < 0.15, "{:?}", c.rgb);
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
        let (_, _, stats) = render_pass_with_stats(&s, &flat, 0);
        assert!(stats.stage(Stage::Host).items > 0 && stats.stage(Stage::Shade).items > 0);
        let (w, h) = (s.image.width, s.image.height);
        let (a, b) = (tiles(&mean_image(&s, n), w, h, 3, 2), tiles(&path_image(&s, n), w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: wavefront {} vs path {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
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
