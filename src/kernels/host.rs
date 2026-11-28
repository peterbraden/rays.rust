//! The host's side of the CPU queue: vertices the kernels have no code for, shaded here.
//!
//! The kernels cover the materials that are data (`wavefront::flat::FlatMaterial`). A scene
//! may have others (textured, layered, procedural: `FlatMaterial::Host`), and rather than
//! refuse the scene, or wait until every material is ported, a path that hits one stops
//! there (`pathtrace::PATH_HOST`) and waits in a queue of its own (`pathtrace::HOST_QUEUE`).
//! When a pass's rounds are done the host reads that queue and its paths back, shades each
//! vertex with the scene's own material, exactly as the CPU's wavefront integrator does
//! (`integrator::host_vertex`, so `MaterialModel::scatter`, with its own light sampling and
//! shadow rays through the scene's own acceleration structure), writes the paths back with
//! their next rays, and runs the rounds again over them, until none waits
//! (`GpuRenderer::block_with_host`). A path can wait once a vertex, so that is at most
//! `max_depth` + 1 times round.
//!
//! This is the arrangement Budge and others describe for path tracing on "hybrid resources"
//! (see `scheduler`'s reading): queues of work per kind, each drained by whichever processor
//! can, the CPU taking what the GPU can't. Theirs schedules both for throughput; this one, so
//! far, has a fixed policy, which the scheduler's measurements exist to improve on:
//!
//! - The host is served after all of a pass's rounds, not between them: one read a wait,
//!   not one a round, since a read is a synchronisation with the device (and in a browser a
//!   frame's delay). A scene whose camera sees a host material directly so pays one read a
//!   pass at least.
//! - Only the waiting paths are read and written: a kernel gathers them side by side for
//!   the read (`pathtrace::gather_host`) and another scatters the shaded ones back to their
//!   slots and queues those that go on (`scatter_host`), each dispatched over the host's
//!   queue. Their number isn't known until the read, so a guess of them is read with the
//!   queue's header (a quarter more than last time) and the rest, if there are more, with a
//!   second read. Reading the whole wavefront instead, as this first did, cost more than a
//!   wider wavefront saved (`scheduler::lanes`).
//! - A pass can't end until its waiting paths have, as a pixel has one path a pass. Holding
//!   them over to a later pass, so the device never idles on the host, needs a path to carry
//!   its own sample index (plan.md section 13, step 4).
//!
//! The picture doesn't depend on any of that: the numbers a host vertex draws are a Sobol'
//! point of its own, keyed by the pixel, the sample and the depth, so the same whenever and
//! wherever it is shaded.
//!
//! What a vertex costs here against on the device is `scheduler::Stage::Host` in the render's
//! report: its paths and its time are what porting a material would save.

use crate::color::Color;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::wavefront::flat::{FlatScene, Hit};
use crate::wavefront::integrator::{host_vertex, HostVertex};
use pathtrace::glam::{Vec3, Vec4};
use pathtrace::{PathState, PATH_ACTIVE, PATH_DIRECTIONAL, PATH_HOST, PATH_LIGHTS_COUNTED, PATH_SCATTERED, PATH_SPECULAR};

/// A waiting path (`PATH_HOST`) of `pixel`'s sample `sample`, in the frame about `origin`
/// (`kernels::origin`), after its vertex: with its next
/// ray and queued to go on, or ended. `from` is the vertex its ray left, if the host shaded
/// that one too (`leaving`): finding the hit again must see the scene's shapes as the kernels'
/// round did. With `defer`, the vertex's sun's and sky's shadow rays are left to the next
/// round to trace, where its material allows (`pathtrace::PATH_DEFERRED`): for the device,
/// whose rounds cost the host nothing. (Not where the host intersects shapes: their rays are
/// the host's anyway, and `hit` carries the vertex then.)
#[allow(clippy::too_many_arguments)]
pub fn shade(s: &Scene, flat: &FlatScene, origin: &Vector3<f64>, p: &PathState, pixel: u32, sample: u32,
             from: Option<crate::shapes::leaving::Leaving>, defer: bool) -> PathState {
    let mut next = *p;
    let flags = p.state.y & !(PATH_HOST | PATH_ACTIVE);
    next.state.y = flags;
    next.state.z = 0;
    next.hit = Vec4::ZERO;
    let f64s = |v: Vec3| Vector3::new(v.x as f64, v.y as f64, v.z as f64);
    // (The kernels' frame is about `origin`; the scene's own code is in the world's.)
    let ray = Ray { ro: f64s(p.o.truncate()) + origin, rd: f64s(p.d.truncate()).normalize() };
    // The hit again, on the primitive the kernel found but in f64: a point on the surface to
    // the precision the renderer's materials offset their rays by. (A ray that grazed the
    // primitive in f32 and misses it in f64 ends here: its vertex was an edge's.)
    let prim = p.state.z;
    let Some(hit) = crate::shapes::leaving::with(from, || flat.refine(prim, &ray, p.hit.x as f64)) else { return next };
    let depth = p.beta.w as u32;
    let mut v = HostVertex {
        ray,
        beta: Color::new(p.beta.x as f64, p.beta.y as f64, p.beta.z as f64),
        l: Color::black(),
        depth,
        specular: p.state.y & PATH_SPECULAR != 0,
        lights_counted: p.state.y & PATH_LIGHTS_COUNTED != 0,
        directional: p.state.y & PATH_DIRECTIONAL != 0,
        prev_pdf: p.d.w as f64,
        scattered: p.state.y & PATH_SCATTERED != 0,
        volume: volume_state(flat, p.state.y),
        done: false,
        defer: defer && !host_shapes(flat),
        deferred: Vec::new(),
    };
    let stream = crate::sobol::hash(s.render.seed ^ 0x686f_7374, ((pixel as u64) << 16) | depth as u64);
    // Its shadow rays, traced here by the scene's own code, leave this vertex
    // (`shapes::leaving`), as does its next ray, traced in the next round (`HOST_VERTEX`).
    // (Its rays start off it by the scene's shadow bias, or its rounding error if larger, as
    // the path tracer's do: `wavefront::integrator::host_bounce`.)
    let vertex = crate::shapes::leaving::Leaving {
        point: hit.point,
        tolerance: 1e-7 * (1. + hit.point.amax()),
        reach: 10. * s.render.shadow_bias + 1e-5 * (1. + hit.point.amax() + hit.dist),
    };
    crate::shapes::leaving::with(Some(vertex), || host_vertex(s, flat, &mut v, &hit, crate::sobol::SampleState::new(stream, sample)));
    let f32s = |v: &Vector3<f64>| Vec3::new(v.x as f32, v.y as f32, v.z as f32);
    // The kept rays as the next round reads them; one whose light can't be packed traced here.
    let mut kept = [(0u32, 0u32); crate::material::functions::DEFERRED_MOST];
    for (k, d) in v.deferred.iter().enumerate() {
        let light = f32s(&d.light.rgb);
        match pathtrace::pack_light(light) {
            Some(bits) => {
                // The path carries the light as packed, so what is taken back cancels it.
                let packed = pathtrace::unpack_light(bits);
                next.l += (packed - light).extend(0.);
                kept[k] = (pathtrace::pack_direction(f32s(&d.ray.rd)), bits | if d.sun { pathtrace::DEFERRED_SUN } else { 0 });
            }
            None => {
                let through = s.objects.transmittance(&d.ray, d.distance, f64::MIN_POSITIVE);
                v.l = v.l - d.light * (Color::white() - through);
            }
        }
    }
    next.l += f32s(&v.l.rgb).extend(0.);
    let deferring = kept.iter().any(|&(_, light)| light != 0);
    if deferring {
        let f = pathtrace::bits_as_float;
        next.hit = Vec4::new(f(kept[0].0), f(kept[0].1), f(kept[1].0), f(kept[1].1));
        let reach = |k: usize| v.deferred.get(k).map_or(pathtrace::DISTANCE_UNBOUNDED, |d| pathtrace::pack_distance(d.distance as f32));
        next.state.z = reach(0) | (reach(1) << 16);
    }
    if v.done {
        if deferring {
            // A round for its shadow rays alone, from off the surface on the side its ray came
            // from, as the host's own start (`integrator::host_bounce`).
            let side = if ray.rd.dot(&hit.geometric) < 0. { hit.geometric } else { -hit.geometric };
            let from = hit.point - origin;
            next.o = f32s(&(from + side * (4e-6 * (1. + from.amax())))).extend(p.o.w);
            next.state.y = (p.state.y & (pathtrace::PATH_MEDIA | pathtrace::PATH_PROVEN)) | PATH_ACTIVE | pathtrace::PATH_DEFERRED | pathtrace::PATH_SHADOWS_ONLY;
        }
        return next;
    }
    // The next ray leaves from where the scene's own code put it, off the surface by what
    // f64 needs; the kernels' f32 rounds a point by up to 6e-8 of its distance from the
    // render's origin, which far off is more than that (64 m at Saturn's 1.5 million km: the ray would
    // start on either side of its rings). So further off, on the side it leaves by.
    let side = if v.ray.rd.dot(&hit.geometric) >= 0. { hit.geometric } else { -hit.geometric };
    let from = v.ray.ro - origin;
    let from = from + side * (4e-6 * (1. + from.amax()));
    next.o = f32s(&from).extend(p.o.w);
    // The vertex, for the next round to name to the scene's shapes its ray meets
    // (`wavefront::integrator`'s batch reads it before `pathtrace::find_all`, which clears it).
    // (Only where the host intersects shapes, which is on the CPU: the device's kernels read
    // a path as they left it.)
    if host_shapes(flat) {
        next.hit = f32s(&(hit.point - origin)).extend(HOST_VERTEX);
    }
    next.d = f32s(&v.ray.rd).extend(v.prev_pdf as f32);
    next.beta = f32s(&v.beta.rgb).extend(v.depth as f32);
    let flag = |on: bool, bit: u32| if on { bit } else { 0 };
    // (A surface leaves what the path has done in media as it was.)
    next.state.y = (p.state.y & (pathtrace::PATH_MEDIA | pathtrace::PATH_PROVEN)) | PATH_ACTIVE | flag(deferring, pathtrace::PATH_DEFERRED) | flag(v.specular, PATH_SPECULAR) | flag(v.scattered, PATH_SCATTERED) | flag(v.lights_counted, PATH_LIGHTS_COUNTED) | flag(v.directional, PATH_DIRECTIONAL);
    next
}

/// How many shadow rays a path `shade` returned leaves to its next round (`PATH_DEFERRED`).
pub fn deferred_rays(p: &PathState) -> u64 {
    if p.state.y & pathtrace::PATH_DEFERRED == 0 {
        return 0;
    }
    [p.hit.y, p.hit.w].iter().filter(|&&light| pathtrace::float_as_bits(light) & !pathtrace::DEFERRED_SUN != 0).count() as u64
}

/// The object the host answers for at a hit on primitive `prim`, as a key for its work
/// (`scheduler::ObjectStats`): an object whose material isn't data by its place in
/// `FlatScene::hosts`; a shape the host intersects after them, by its place in `shapes`.
pub fn object_key(flat: &FlatScene, prim: u32) -> Option<usize> {
    let listed = flat.prims.len() + flat.planes.len();
    if prim as usize >= listed {
        return Some(shape_key(flat, prim as usize - listed));
    }
    match flat.hit_material(prim) {
        crate::wavefront::flat::FlatMaterial::Host { object } => Some(*object as usize),
        _ => None,
    }
}

/// The key of shape `k` of `FlatScene::shapes` (`object_key`).
pub fn shape_key(flat: &FlatScene, k: usize) -> usize {
    flat.hosts.len() + k
}

/// What each key names (`object_key`): the object by its place in the scene file and its kind
/// of shape, and whether the host shades its material or intersects its shape.
pub fn object_names(s: &Scene, flat: &FlatScene) -> Vec<(String, &'static str)> {
    let name = |o: &crate::sceneobject::SceneObject| match s.source_index(o) {
        Some(i) => format!("objects[{}] ({})", i, o.geometry.kind()),
        None => o.geometry.kind().to_string(),
    };
    flat.hosts.iter().map(|o| (name(o), "material"))
        .chain(flat.shapes.iter().map(|(o, _)| (name(o), "shape")))
        .collect()
}

/// `PathState::hit.w` of a path whose ray leaves a vertex the host shaded, its point (in the
/// frame about the origin) in `hit.xyz`: until its next ray is intersected, a path's `hit` is
/// otherwise unread (`shade` clears what would read it, and `pathtrace::find_all` clears it).
pub const HOST_VERTEX: f32 = -2.0;

/// Whether the host intersects some of the scene's shapes, as `Buffers::build_at` gives it
/// those the kernels have no code for: all but one sea.
fn host_shapes(flat: &FlatScene) -> bool {
    let seas = flat.shapes.iter().filter(|(o, _)| o.geometry.ocean().is_some()).count().min(1);
    flat.shapes.len() > seas
}

/// The vertex a path's ray leaves (`shapes::leaving`), if the host shaded it, from its state
/// in the frame about `origin`: as exact as single precision there, and the ray as far off it
/// as `shade` started it.
pub fn leaving(p: &PathState, origin: &Vector3<f64>) -> Option<crate::shapes::leaving::Leaving> {
    (p.hit.w == HOST_VERTEX).then(|| {
        let at = Vector3::new(p.hit.x as f64, p.hit.y as f64, p.hit.z as f64);
        let far = 1. + at.amax();
        crate::shapes::leaving::Leaving { point: at + origin, tolerance: 2e-7 * far, reach: 0.01 + 3e-5 * far }
    })
}

/// What a path with these flags has done in media, as the scene's own code keeps it
/// (`volume::resume_path`): how many times it has scattered there, and the factor on every
/// medium's density that the one it last scattered in set for its segments, as the kernels
/// read it (`pathtrace::segment_scale`).
fn volume_state(flat: &FlatScene, flags: u32) -> (u32, f64) {
    let last = (flags >> pathtrace::PATH_MEDIUM_SHIFT) & 0xff;
    if last == 0 {
        return (0, 1.);
    }
    let scatterings = (flags >> pathtrace::PATH_SCATTERINGS_SHIFT) & 0xff;
    let medium = &flat.media[last as usize - 1];
    let scale = match medium.reduced {
        Some((from, reduced)) if scatterings >= from => reduced,
        _ => medium.scale,
    };
    (scatterings.max(1), scale)
}

#[cfg(test)]
mod deferred_tests {
    use pathtrace::glam::Vec3;

    #[test]
    fn test_any_thirty_bits_are_a_normal_float_and_back() {
        let mut rng = crate::geometry::Pcg32::new(7, 1);
        let mut patterns: Vec<u32> = (0..100_000).map(|_| rng.next_u32() & 0x3fff_ffff).collect();
        patterns.extend([0, 1, 0x3fff_ffff, 1 << 29, (1 << 29) - 1, 0x1fff_ffff]);
        for bits in patterns {
            let f = pathtrace::bits_as_float(bits);
            assert!(f.is_normal(), "{:#x} is {:e}", bits, f);
            assert_eq!(pathtrace::float_as_bits(f), bits);
        }
    }

    #[test]
    fn test_light_packs_to_a_part_in_256_of_its_brightest_channel() {
        let mut rng = crate::geometry::Pcg32::new(9, 1);
        for _ in 0..100_000 {
            let scale = 2f32.powf(rng.next_f64() as f32 * 31. - 20.);
            let c = Vec3::new(rng.next_f64() as f32, rng.next_f64() as f32, rng.next_f64() as f32) * scale;
            let most = c.max_element();
            if !(pathtrace::LIGHT_LEAST..pathtrace::LIGHT_MOST).contains(&most) {
                continue;
            }
            let bits = pathtrace::pack_light(c).expect("within bounds");
            assert!(bits < 1 << 29, "{:#x} leaves no room for the sun's bit", bits);
            let back = pathtrace::unpack_light(bits | pathtrace::DEFERRED_SUN);
            assert!((back - c).abs().max_element() <= most / 256., "{:?} back as {:?}", c, back);
        }
        assert_eq!(pathtrace::pack_light(Vec3::ZERO), Some(0));
        assert_eq!(pathtrace::pack_light(Vec3::splat(pathtrace::LIGHT_MOST)), None);
        assert_eq!(pathtrace::pack_light(Vec3::new(0., pathtrace::LIGHT_LEAST / 2., 0.)), None);
    }

    #[test]
    fn test_a_direction_packs_to_within_two_ten_thousandths_of_a_radian() {
        let mut rng = crate::geometry::Pcg32::new(11, 1);
        for _ in 0..100_000 {
            let v = Vec3::new(rng.next_f64() as f32 - 0.5, rng.next_f64() as f32 - 0.5, rng.next_f64() as f32 - 0.5);
            if v.length() < 1e-3 {
                continue;
            }
            let v = v.normalize();
            let back = pathtrace::unpack_direction(pathtrace::pack_direction(v));
            assert!(back.cross(v).length() < 2e-4 && back.dot(v) > 0., "{:?} back as {:?}", v, back);
        }
    }

    #[test]
    fn test_a_distance_packs_short_by_at_most_a_part_in_1024() {
        let mut rng = crate::geometry::Pcg32::new(13, 1);
        for _ in 0..100_000 {
            let d = 2f32.powf(rng.next_f64() as f32 * 62. - 31.);
            let back = pathtrace::unpack_distance(pathtrace::pack_distance(d));
            assert!(back <= d && back >= d * (1. - 1. / 1024.), "{} back as {}", d, back);
        }
        assert_eq!(pathtrace::pack_distance(1e30), pathtrace::DISTANCE_UNBOUNDED);
        assert!(pathtrace::unpack_distance(pathtrace::DISTANCE_UNBOUNDED) > 1e38);
    }
}
