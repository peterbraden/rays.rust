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
//!    per light sample, then sample the BSDF for the path's next ray, or retire it.
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

/// Render sample `k` of every pixel: each pixel's radiance (times its weight), and the number
/// of rays cast.
/// Paths per batch: a queue small enough to stay in cache through every stage, and enough
/// batches to keep every thread busy.
const BATCH: usize = 4096;

pub fn render_pass(s: &Scene, flat: &FlatScene, k: usize) -> (Vec<Color>, u64) {
    let (w, h) = (s.image.width, s.image.height);
    let mut image = vec![Color::black(); w * h];
    // Batches of pixels run in parallel, each through all the stages: a queue that streams
    // through memory once per stage would make the stages memory-bound on a CPU (on a GPU,
    // the whole image's queue is the batch).
    let batches: Vec<(Vec<(u32, Color)>, u64)> = (0..(w * h).div_ceil(BATCH)).into_par_iter()
        .map(|b| render_batch(s, flat, k, b * BATCH..((b + 1) * BATCH).min(w * h)))
        .collect();
    let mut rays = 0;
    for (done, n) in batches {
        rays += n;
        for (pixel, c) in done {
            image[pixel as usize] = image[pixel as usize] + c;
        }
    }
    (image, rays)
}

/// Sample `k` of the pixels `range`, stage by stage: each finished path's pixel and radiance,
/// and the number of rays cast.
fn render_batch(s: &Scene, flat: &FlatScene, k: usize, range: std::ops::Range<usize>) -> (Vec<(u32, Color)>, u64) {
    let (w, h) = (s.image.width, s.image.height);
    let pixels = (w * h) as u64;
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
                    depth: 0, specular: true, prev_pdf: 0., sampler, scattered: false, hit: None, done: false })
    }).collect();
    let mut done = Vec::with_capacity(paths.len());
    let mut shadows: Vec<Shadow> = Vec::new();
    let mut rays = 0u64;
    while !paths.is_empty() {
        // 2. Intersect.
        rays += paths.len() as u64;
        for p in paths.iter_mut() {
            p.hit = flat.intersect(&p.ray, 0.);
        }
        // 3. Escaped rays and emitters.
        for p in paths.iter_mut() {
            gather_emission(p, s, flat);
        }
        retire(&mut paths, &mut done);
        // 4. Shade, sorted by material so that each kind's code runs together.
        paths.sort_unstable_by_key(|p| flat.hit_material(p.hit.unwrap().prim).kind());
        shadows.clear();
        for (i, p) in paths.iter_mut().enumerate() {
            shade(i, p, s, flat, &mut shadows);
        }
        // 5. Shadow rays.
        rays += shadows.len() as u64;
        for sh in &shadows {
            if !flat.occluded(&sh.ray, 0., sh.max) {
                paths[sh.path].l = paths[sh.path].l + sh.contribution;
            }
        }
        retire(&mut paths, &mut done);
    }
    (done, rays)
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
            Some(env) if !p.specular => power_heuristic(p.prev_pdf, env.pdf(&p.ray.rd)),
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
        let le = m.emitted(&h.geometric, &-p.ray.rd);
        if le != Color::black() {
            // Weighted against light sampling, if the vertex before sampled this light.
            let sampled = flat.emitter_index.get(h.prim as usize).copied().flatten();
            let w = match sampled {
                Some(e) if !p.specular && s.render.sample_lights => {
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
        let (image, rays) = render_pass(s, flat, k);
        let mut rc = rc.lock().unwrap();
        for y in 0..h {
            for x in 0..w {
                rc.set_pixel(x, y, image[y * w + x], 1);
            }
        }
        rc.rays_cast += rays;
        after_pass(&rc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    /// The mean of a pass's pixels over `passes` passes.
    fn mean_image(s: &Scene, passes: usize) -> Vec<Color> {
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
