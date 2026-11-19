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
use pathtrace::{PathState, PATH_ACTIVE, PATH_HOST, PATH_LIGHTS_COUNTED, PATH_SCATTERED, PATH_SPECULAR};

/// A waiting path (`PATH_HOST`) of `pixel`'s sample `sample`, after its vertex: with its next
/// ray and queued to go on, or ended.
pub fn shade(s: &Scene, flat: &FlatScene, p: &PathState, pixel: u32, sample: u32) -> PathState {
    let mut next = *p;
    let flags = p.state.y & !(PATH_HOST | PATH_ACTIVE);
    next.state.y = flags;
    next.state.z = 0;
    next.hit = Vec4::ZERO;
    let f64s = |v: Vec3| Vector3::new(v.x as f64, v.y as f64, v.z as f64);
    let ray = Ray { ro: f64s(p.o.truncate()), rd: f64s(p.d.truncate()).normalize() };
    // The hit again, on the primitive the kernel found but in f64: a point on the surface to
    // the precision the renderer's materials offset their rays by. (A ray that grazed the
    // primitive in f32 and misses it in f64 ends here: its vertex was an edge's.)
    let prim = p.state.z;
    let Some(hit) = flat.refine(prim, &ray) else { return next };
    let depth = p.beta.w as u32;
    let mut v = HostVertex {
        ray,
        beta: Color::new(p.beta.x as f64, p.beta.y as f64, p.beta.z as f64),
        l: Color::black(),
        depth,
        specular: p.state.y & PATH_SPECULAR != 0,
        lights_counted: p.state.y & PATH_LIGHTS_COUNTED != 0,
        prev_pdf: p.d.w as f64,
        scattered: p.state.y & PATH_SCATTERED != 0,
        done: false,
    };
    let stream = crate::sobol::hash(s.render.seed ^ 0x686f_7374, ((pixel as u64) << 16) | depth as u64);
    host_vertex(s, flat, &mut v, &hit, crate::sobol::SampleState::new(stream, sample));
    let f32s = |v: &Vector3<f64>| Vec3::new(v.x as f32, v.y as f32, v.z as f32);
    next.l += f32s(&v.l.rgb).extend(0.);
    if v.done {
        return next;
    }
    next.o = f32s(&v.ray.ro).extend(p.o.w);
    next.d = f32s(&v.ray.rd).extend(v.prev_pdf as f32);
    next.beta = f32s(&v.beta.rgb).extend(v.depth as f32);
    let flag = |on: bool, bit: u32| if on { bit } else { 0 };
    next.state.y = PATH_ACTIVE | flag(v.specular, PATH_SPECULAR) | flag(v.scattered, PATH_SCATTERED) | flag(v.lights_counted, PATH_LIGHTS_COUNTED);
    next
}
