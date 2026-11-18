//! A path tracer on the GPU, with wgpu (Vulkan, Metal, DirectX 12): the flattened scene of the
//! wavefront integrator (`wavefront::flat`) uploaded as buffers, and a compute shader that
//! traces one path per pixel per pass, porting the wavefront integrator's shading to f32
//! (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., chapter 15, whose
//! wavefront renderer is pbrt-v4's GPU renderer).
//!
//! The shader is Rust: the crate `shaders/pathtrace`, compiled to SPIR-V by rust-gpu
//! (`scripts/build-shaders.sh`, on rust-gpu's pinned nightly; the renderer stays on stable and
//! includes the committed `pathtrace.spv`), which wgpu translates for the device (naga), for
//! Metal too. The same crate is also built for the CPU: its structs are these buffers'
//! layouts (so the two sides can't drift), and the tests run its functions, and its whole
//! `radiance`, on the CPU against the renderer's own and the GPU's.
//!
//! What stays on the CPU: camera rays (generated each pass and uploaded, so every camera,
//! realistic lenses included, works unchanged), and a map of the sky (equirectangular,
//! `ENV_WIDTH` × `ENV_HEIGHT`), which the shader samples directions from in proportion to its
//! brightness and, for a background other than a skysphere's, looks up. A skysphere's air is
//! the GPU's own: its parameters and tables (`skysphere::AirExport`, as `air_record`s) let the
//! shader evaluate it per ray as the CPU does, for escaping rays, aerial perspective from each
//! ray's origin, and sunlight at each point. Each pass's radiance accumulates on the GPU, and
//! is read back into the render's context every `PASSES_PER_READ` passes.
//!
//! Differences from the CPU integrators: f32 arithmetic (ray offsets scaled accordingly); area
//! lights chosen by power (not the light BVH); independent random numbers (a hash per pixel,
//! pass and dimension), not Sobol'; point lights isotropic or spot.

use crate::color::Color;
use crate::rendercontext::RenderContext;
use crate::scene::{PathTermination, Scene};
use crate::wavefront::flat::{FlatMaterial, FlatScene, Prim};
use crate::light::Profile;
use crate::na::Vector3;
use crate::ray::Ray;
use bytemuck::Zeroable;
use rayon::prelude::*;
use std::sync::Mutex;
use wgpu::util::DeviceExt;

/// The baked sky's resolution.
const ENV_WIDTH: usize = 1024;
const ENV_HEIGHT: usize = 512;
/// Texels per side of a cell of the sky map's visibility (`environment::visibility`).
const VISIBILITY_SCALE: usize = 8;
/// Passes between reading the accumulated image back.
const PASSES_PER_READ: usize = 8;

/// Passes between reads whose rounds are counted (`GpuRenderer::rounds_buffer`); passes beyond
/// these go uncounted, not unrendered.
const COUNTED_PASSES: usize = 64;

/// What the host is keeping about the passes queued since the image was last read
/// (`scheduler::Stats`, and what completes it at the read).
#[derive(Default)]
struct Block {
    stats: crate::scheduler::Stats,
    /// Passes queued whose rounds are being counted.
    counted: usize,
    /// When the first of them was queued, and the host's own time in them since.
    started: Option<web_time::Instant>,
    host_seconds: f64,
}

// The buffers' layouts are the shader crate's own (`shaders/pathtrace`), so the two can't drift.
use pathtrace::{CameraRay as GpuRay, Light as GpuLight, Material as GpuMaterial, Node as GpuNode, Params, PathState, Prim as GpuPrim, SkyQuery};
use pathtrace::glam::{UVec4, Vec4};

fn v4(v: &Vector3<f64>, w: f32) -> Vec4 {
    Vec4::new(v.x as f32, v.y as f32, v.z as f32, w)
}

fn c4(c: &Color) -> Vec4 {
    Vec4::new(c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32, 0.)
}

/// Flatten `m` into `out` (checkerboards' squares after it), returning its index.
fn push_material(m: &FlatMaterial, out: &mut Vec<GpuMaterial>) -> u32 {
    let index = out.len() as u32;
    let blank = GpuMaterial::zeroed();
    out.push(blank);
    let g = match m {
        FlatMaterial::Diffuse { albedo } => GpuMaterial { kind: pathtrace::MAT_DIFFUSE, c0: c4(albedo), ..blank },
        FlatMaterial::Plastic { albedo, refractive_index, roughness } =>
            GpuMaterial { kind: pathtrace::MAT_PLASTIC, c0: c4(albedo), p: Vec4::new(*refractive_index as f32, *roughness as f32, 0., 0.), ..blank },
        FlatMaterial::Dielectric { refractive_index, tint } =>
            GpuMaterial { kind: pathtrace::MAT_DIELECTRIC, c0: c4(tint), p: Vec4::new(*refractive_index as f32, 0., 0., 0.), ..blank },
        FlatMaterial::Conductor { optics, roughness } => {
            let (eta, k) = optics.current();
            GpuMaterial { kind: pathtrace::MAT_CONDUCTOR, c0: c4(&eta), c1: c4(&k), p: Vec4::new(0., *roughness as f32, 0., 0.), ..blank }
        },
        FlatMaterial::Emitter { radiance, two_sided } => GpuMaterial { kind: pathtrace::MAT_EMITTER, a: *two_sided as u32, c0: c4(radiance), ..blank },
        FlatMaterial::Specular { albedo, roughness } => GpuMaterial { kind: pathtrace::MAT_METAL, c0: c4(albedo), p: Vec4::new(0., *roughness as f32, 0., 0.), ..blank },
        FlatMaterial::Checker { m1, m2, xsize, zsize } => {
            let a = push_material(m1, out);
            let b = push_material(m2, out);
            GpuMaterial { kind: pathtrace::MAT_CHECKER, a, b, p: Vec4::new(0., 0., *xsize as f32, *zsize as f32), ..blank }
        },
    };
    out[index as usize] = g;
    index
}

/// A cumulative distribution over `f` (n + 1 values from 0 to 1), uniform if `f` sums to 0.
fn cdf(f: &[f64]) -> Vec<f32> {
    let total: f64 = f.iter().sum();
    let mut out = Vec::with_capacity(f.len() + 1);
    let mut acc = 0.;
    out.push(0.);
    for (i, v) in f.iter().enumerate() {
        acc += if total > 0. { v / total } else { 1. / f.len() as f64 };
        out.push(if i + 1 == f.len() { 1. } else { acc as f32 });
    }
    out
}

/// The direction through texel (x, y) of the baked sky's map, as the shader's `env_sample`.
fn env_direction(x: usize, y: usize) -> Vector3<f64> {
    use std::f64::consts::PI;
    let u = (x as f64 + 0.5) / ENV_WIDTH as f64;
    let v = (y as f64 + 0.5) / ENV_HEIGHT as f64;
    let (phi, theta) = ((u - 0.5) * 2. * PI, v * PI);
    Vector3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin())
}

/// Append `air` (and its moonlit copy) to `tables` as the shader's record (`pathtrace::AIR_*`),
/// returning where it starts.
fn air_record(tables: &mut Vec<f32>, air: &crate::skysphere::AirExport) -> u32 {
    use pathtrace::*;
    let base = tables.len();
    tables.resize(base + AIR_HEADER as usize, 0.);
    let put = |tables: &mut Vec<f32>, i: u32, v: f64| tables[base + i as usize] = v as f32;
    let put3 = |tables: &mut Vec<f32>, i: u32, v: &Vector3<f64>| (0..3).for_each(|k| tables[base + i as usize + k] = v[k] as f32);
    let c = air.earth_centre;
    put3(tables, AIR_CENTRE, &c);
    put(tables, AIR_RADII, air.earth_radius);
    put(tables, AIR_RADII + 1, air.atmosphere_radius);
    // |c|² - r², in f64: the shader's `excess` adds the terms in p, which are small near the
    // ground, to these.
    put(tables, AIR_EXCESS, c.norm_squared() - air.earth_radius * air.earth_radius);
    put(tables, AIR_EXCESS + 1, c.norm_squared() - air.atmosphere_radius * air.atmosphere_radius);
    put3(tables, AIR_SUN, &air.sun_direction);
    put(tables, AIR_HEIGHTS, air.rayleigh_thickness);
    put(tables, AIR_HEIGHTS + 1, air.mie_thickness);
    put(tables, AIR_BRIGHTNESS, air.brightness);
    put3(tables, AIR_RAYLEIGH, &air.rayleigh);
    put(tables, AIR_MIE, air.mie);
    put(tables, AIR_MIE + 1, air.mie_anisotropy);
    put3(tables, AIR_OZONE, &air.ozone);
    for (i, h) in crate::skysphere::LUT_ALTITUDES.iter().enumerate() {
        put(tables, AIR_LUT_ALTITUDES + i as u32, *h);
    }
    let offset = |tables: &mut Vec<f32>, i: u32, values: Option<Vec<f32>>| {
        let at = match values {
            Some(v) => { let at = tables.len() as u32; tables.extend(v); at },
            None => NO_AIR,
        };
        tables[base + (AIR_TABLES + i) as usize] = f32::from_bits(at);
    };
    offset(tables, 0, Some(air.sun_table.iter().flat_map(|d| d.map(|x| x as f32)).collect()));
    offset(tables, 1, air.multiple.as_ref().map(|m| m.iter().flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect()));
    offset(tables, 2, air.lut.as_ref().map(|l| l.iter().flat_map(|(r, m, s)| [r, m, s]).flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect()));
    let moon = air.moon.as_ref().map_or(NO_AIR, |m| air_record(tables, m));
    tables[base + (AIR_TABLES + 3) as usize] = f32::from_bits(moon);
    base as u32
}

/// Append `space` to `tables` as the shader's record (`pathtrace::SPACE_*`), returning where it
/// starts: the frame and settings, the noise's tables, and each part's parameters (the stars
/// as a catalogue, which the shader can't generate).
fn space_record(tables: &mut Vec<f32>, space: &crate::procedural::space::Space) -> u32 {
    use pathtrace::*;
    let base = tables.len();
    tables.resize(base + SPACE_HEADER as usize, 0.);
    let f = &space.frame;
    for (i, v) in [f.centre, f.pole, f.east].iter().flat_map(|v| v.iter()).enumerate() {
        tables[base + SPACE_FRAME as usize + i] = *v as f32;
    }
    tables[base + SPACE_BRIGHTNESS as usize] = space.brightness as f32;
    tables[base + SPACE_THRESHOLD as usize] = space.threshold as f32;
    // Every field's noise is Perlin's, with the same table and rotation; only its offset differs.
    let noise = crate::noise::SeededNoise::new(0);
    for (i, v) in noise.rotation_rows().iter().enumerate() {
        tables[base + SPACE_ROTATION as usize + i] = *v as f32;
    }
    let put = |tables: &mut Vec<f32>, i: u32, v: u32| tables[base + (SPACE_TABLES + i) as usize] = f32::from_bits(v);
    let at = tables.len() as u32;
    tables.extend(noise.permutation().iter().map(|p| *p as f32));
    put(tables, 0, at);
    let stars = match &space.stars {
        Some(stars) => {
            let (sigma, n, starts, list, flux) = stars.export(&space.frame);
            let at = tables.len();
            tables.resize(at + STARS_HEADER as usize, 0.);
            tables[at] = sigma as f32;
            tables[at + 1] = f32::from_bits(n as u32);
            let offset = |tables: &mut Vec<f32>, i: usize, values: Vec<f32>| {
                tables[at + i] = f32::from_bits(tables.len() as u32);
                tables.extend(values);
            };
            offset(tables, 2, starts.iter().map(|s| f32::from_bits(*s)).collect());
            offset(tables, 3, list.iter().flat_map(|s| s.map(|x| x as f32)).collect());
            offset(tables, 4, flux);
            at as u32
        },
        None => NO_AIR,
    };
    put(tables, 1, stars);
    let record = |tables: &mut Vec<f32>, values: Vec<f64>| { let at = tables.len() as u32; tables.extend(values.iter().map(|v| *v as f32)); at };
    let milky = space.milky_way.as_ref().map_or(NO_AIR, |m| record(tables, m.export()));
    put(tables, 2, milky);
    let nebulae = record(tables, space.nebulae.iter().flat_map(|n| n.export()).collect());
    put(tables, 3, nebulae);
    put(tables, 4, space.nebulae.len() as u32);
    let galaxies = record(tables, space.galaxies.iter().flat_map(|g| g.export()).collect());
    put(tables, 5, galaxies);
    put(tables, 6, space.galaxies.len() as u32);
    base as u32
}

/// A scene uploaded to the GPU, with the pipeline that renders it.
pub struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    /// A round's three kernels, each over the queue of paths still alive: what each path's ray
    /// hits, the rest of its vertex there, then the light its sky queries find
    /// (`pathtrace::intersect_paths`, `bounce_paths`, `answer_sky`); and the one that ends a
    /// pass, adding every path to the image (`add_paths`).
    intersect_pipeline: wgpu::ComputePipeline,
    bounce_pipeline: wgpu::ComputePipeline,
    sky_pipeline: wgpu::ComputePipeline,
    add_pipeline: wgpu::ComputePipeline,
    /// The scene and the paths, for rounds reading the queue's side 0 and its side 1: they
    /// differ in their parameters (`Params::seed.z`).
    round_groups: [wgpu::BindGroup; 2],
    add_group: wgpu::BindGroup,
    params: Params,
    params_buffers: [wgpu::Buffer; 2],
    /// The two queues of paths (`pathtrace::queue`), and the buffer each round's dispatch is
    /// read from: the header of the queue it reads, copied there, as a buffer bound for
    /// writing can't also be a dispatch's arguments. `empty_header` resets a side.
    queue_buffer: wgpu::Buffer,
    dispatch_buffer: wgpu::Buffer,
    empty_header: wgpu::Buffer,
    paths_buffer: wgpu::Buffer,
    accum_buffer: wgpu::Buffer,
    /// For the scheduler's measurements (`scheduler::Stats::live`): the paths alive at the
    /// start of each round after the first, of each pass since the last read. The count is in
    /// the header of the queue a round reads, and a copy between rounds keeps it; nothing in
    /// the kernels knows. A word a round, `rounds()` a pass, for `COUNTED_PASSES` passes.
    rounds_buffer: wgpu::Buffer,
    /// The image, then the rounds, read together.
    readback: wgpu::Buffer,
    block: Mutex<Block>,
    pub adapter_name: String,
}

/// The depth of an exported BVH (`Bvh::export`), leaves included: one for a lone leaf. The
/// shader's stack needs an entry for each level of interior nodes, which is one fewer.
fn bvh_depth(nodes: &[(crate::shapes::bbox::BBox, u32, u32)]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let mut deepest = 0;
    let mut stack = vec![(0usize, 1usize)];
    while let Some((i, d)) = stack.pop() {
        deepest = deepest.max(d);
        let (_, offset, count) = nodes[i];
        if count == 0 {
            stack.push((i + 1, d + 1));
            stack.push((offset as usize, d + 1));
        }
    }
    deepest
}

/// The BVH as the shader traverses it: its interior nodes alone, each with its two children's
/// boxes (`pathtrace::Node`), the root first. A tree that is a single leaf gets a root with
/// that leaf as its only child; no tree at all, a root no ray enters.
fn gpu_nodes(exported: &[(crate::shapes::bbox::BBox, u32, u32)]) -> Result<Vec<GpuNode>, String> {
    use pathtrace::glam::Vec3;
    // Children that aren't there.
    let none = pathtrace::NO_CHILD;
    let empty = GpuNode { amin: Vec3::ZERO, a: none, amax: Vec3::ZERO, a_count: 0, bmin: Vec3::ZERO, b: none, bmax: Vec3::ZERO, b_count: 0 };
    let corners = |b: &crate::shapes::bbox::BBox| (Vec3::new(b.min.x as f32, b.min.y as f32, b.min.z as f32), Vec3::new(b.max.x as f32, b.max.y as f32, b.max.z as f32));
    let Some((root_bounds, root_offset, root_count)) = exported.first() else {
        return Ok(vec![empty]);
    };
    if *root_count > 0 {
        let (amin, amax) = corners(root_bounds);
        return Ok(vec![GpuNode { amin, amax, a: *root_offset, a_count: *root_count, ..empty }]);
    }
    // Each interior node's index among the interior nodes, in the same (depth-first) order.
    let mut index = vec![u32::MAX; exported.len()];
    let mut interior = 0;
    for (i, (_, _, count)) in exported.iter().enumerate() {
        if *count == 0 {
            index[i] = interior;
            interior += 1;
        }
    }
    // A child as a node holds it: (first item, count) for a leaf, (index, 0) otherwise.
    let child = |i: usize| -> Result<(Vec3, Vec3, u32, u32), String> {
        let (bounds, offset, count) = &exported[i];
        let (min, max) = corners(bounds);
        if *count > pathtrace::MAX_LEAF {
            return Err(format!("a BVH leaf of {} primitives (the shader's hold {})", count, pathtrace::MAX_LEAF));
        }
        Ok(if *count > 0 { (min, max, *offset, *count) } else { (min, max, index[i], 0) })
    };
    let mut nodes = Vec::with_capacity(interior as usize);
    for (i, (_, second, count)) in exported.iter().enumerate() {
        if *count == 0 {
            // An interior node's first child is the next node; its second, at `offset`.
            let (amin, amax, a, a_count) = child(i + 1)?;
            let (bmin, bmax, b, b_count) = child(*second as usize)?;
            nodes.push(GpuNode { amin, a, amax, a_count, bmin, b, bmax, b_count });
        }
    }
    Ok(nodes)
}

fn entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: buffer.as_entire_binding() }
}

/// The shader's inputs, built on the CPU (`Buffers::build`).
pub struct Buffers {
    pub params: Params,
    /// The primitives as the shader reads them (`pathtrace::pack_prims`), and as records.
    pub prims: Vec<pathtrace::glam::Vec4>,
    pub prim_records: Vec<GpuPrim>,
    pub nodes: Vec<GpuNode>,
    pub materials: Vec<GpuMaterial>,
    pub lights: Vec<GpuLight>,
    pub tables: Vec<f32>,
}

impl Buffers {
    /// The flattened scene as the shader's buffers, or why it can't be.
    pub fn build(s: &Scene, flat: &FlatScene) -> Result<Buffers, String> {
        let mut lights = Vec::new();
        for l in &s.lights {
            let i = l.emission.srgb() * l.intensity;
            let (kind, axis, start, end) = match &l.profile {
                Profile::Isotropic => (0., Vector3::z(), 1., 1.),
                Profile::Spot { axis, cos_falloff_start, cos_total } => (1., *axis, *cos_falloff_start, *cos_total),
                _ => return Err("goniometric and projection lights".into()),
            };
            lights.push(GpuLight { pos: v4(&l.position, kind), intensity: c4(&i), axis: v4(&axis, start as f32), extra: Vec4::new(end as f32, 0., 0., 0.) });
        }
        if lights.is_empty() {
            lights.push(GpuLight::zeroed());
        }

        // Primitives: the bounded ones in the BVH's order, then the planes.
        let mut materials = Vec::new();
        let material_index: Vec<u32> = flat.materials.iter().map(|m| push_material(m, &mut materials)).collect();
        // A buffer may not be empty (wgpu refuses to bind it): a scene of sky alone has none.
        if materials.is_empty() {
            materials.push(GpuMaterial::zeroed());
        }
        let to_gpu = |p: &Prim, m: u32, emitter: u32| -> GpuPrim {
            let mut g = GpuPrim::zeroed();
            g.info = UVec4::new(pathtrace::PRIM_TRIANGLE, material_index[m as usize], emitter, 0);
            match p {
                Prim::Triangle { v, n } => {
                    g.v0 = v4(&v[0], 0.); g.v1 = v4(&v[1], 0.); g.v2 = v4(&v[2], 0.);
                    if let Some(n) = n {
                        g.n0 = v4(&n[0], 0.); g.n1 = v4(&n[1], 0.); g.n2 = v4(&n[2], 0.);
                        g.info.w = 1;
                    }
                },
                Prim::Sphere { center, radius } => { g.v0 = v4(center, *radius as f32); g.info.x = pathtrace::PRIM_SPHERE; },
                Prim::Plane { y } => { g.v0 = Vec4::new(*y as f32, 0., 0., 0.); g.info.x = pathtrace::PRIM_PLANE; },
            }
            g
        };
        let mut prims: Vec<GpuPrim> = flat.prims.iter().enumerate().map(|(i, p)| {
            to_gpu(p, flat.prim_material[i], flat.emitter_index[i].map_or(0, |e| e + 1))
        }).collect();
        prims.extend(flat.planes.iter().map(|(p, m)| to_gpu(p, *m, 0)));
        if prims.is_empty() {
            prims.push(GpuPrim::zeroed());
        }
        // The shader's traversal stack holds at most one child per level of interior nodes.
        let exported = flat.bvh.export();
        let depth = bvh_depth(&exported);
        if depth > pathtrace::STACK {
            return Err(format!("a BVH {} levels deep (the shader's stack holds {})", depth, pathtrace::STACK));
        }
        if prims.len() > pathtrace::MAX_LEAF_ITEMS as usize {
            return Err(format!("{} primitives (the shader's traversal counts to {})", prims.len(), pathtrace::MAX_LEAF_ITEMS));
        }
        // The packed buffer keeps a primitive's material and emitter as floats.
        if prims.len().max(flat.materials.len()) >= pathtrace::MAX_PACKED_INDEX as usize {
            return Err(format!("{} primitives or materials (the shader's buffers count to {})", prims.len().max(flat.materials.len()), pathtrace::MAX_PACKED_INDEX));
        }
        let nodes = gpu_nodes(&exported)?;

        // Tables: emitters' primitives and their cumulative distribution, then the sky.
        let mut tables: Vec<f32> = Vec::new();
        let emitter_off = tables.len() as u32;
        tables.extend(flat.emitters.iter().map(|&p| f32::from_bits(p)));
        let cdf_off = tables.len() as u32;
        let powers: Vec<f64> = (0..flat.emitters.len()).map(|e| flat.emitter_choice.probability(e)).collect();
        tables.extend(cdf(&powers));
        let env_off = tables.len() as u32;
        // The sky as seen from the camera, as the CPU integrators see it from each ray's
        // origin: exact for camera rays, near enough for paths that stay near the camera (from
        // the world's origin, an orbiting camera saw the night side's ground-level sky).
        let origin = crate::environment::viewpoint(s);
        let texels: Vec<Color> = (0..ENV_WIDTH * ENV_HEIGHT).into_par_iter().map(|i| {
            let d = env_direction(i % ENV_WIDTH, i / ENV_WIDTH);
            crate::wavefront::integrator::sky_radiance(&Ray { ro: origin, rd: d }, s, flat)
        }).collect();
        for t in &texels {
            tables.extend([t.rgb.x as f32, t.rgb.y as f32, t.rgb.z as f32]);
        }
        // Rows by brightness times sin θ (a row's share of the sphere) times how much of the
        // sky the scene lets through from the camera, as the CPU's `Environment` (so no shadow
        // rays are spent on the ground), then within each row. The visibility is found on a
        // grid of VISIBILITY_SCALE² texels, the CPU's resolution, for a ray per cell.
        let (vw, vh) = (ENV_WIDTH / VISIBILITY_SCALE, ENV_HEIGHT / VISIBILITY_SCALE);
        let visible: Vec<f64> = (0..vw * vh).into_par_iter().map(|i| {
            let (x, y) = ((i % vw) * VISIBILITY_SCALE + VISIBILITY_SCALE / 2, (i / vw) * VISIBILITY_SCALE + VISIBILITY_SCALE / 2);
            crate::environment::visibility(s, &origin, &env_direction(x, y))
        }).collect();
        let weight = |x: usize, y: usize| {
            let theta = (y as f64 + 0.5) / ENV_HEIGHT as f64 * std::f64::consts::PI;
            let v = visible[(y / VISIBILITY_SCALE) * vw + x / VISIBILITY_SCALE];
            crate::color::luminance(&texels[y * ENV_WIDTH + x]).max(0.) * theta.sin() * v + 1e-9
        };
        let rows: Vec<f64> = (0..ENV_HEIGHT).map(|y| (0..ENV_WIDTH).map(|x| weight(x, y)).sum()).collect();
        let marginal_off = tables.len() as u32;
        tables.extend(cdf(&rows));
        let conditional_off = tables.len() as u32;
        for y in 0..ENV_HEIGHT {
            let row: Vec<f64> = (0..ENV_WIDTH).map(|x| weight(x, y)).collect();
            tables.extend(cdf(&row));
        }

        // The air: the sky's (for escaping rays), the aerial perspective's, the sun's.
        let sky_air = flat.sky.as_ref().and_then(|sky| sky.medium.material_at(Vector3::zeros()).air());
        let sky_off = sky_air.map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a));
        let aerial_off = s.atmosphere.as_ref().map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a.export()));
        let sun_off = s.sun.as_ref().and_then(|sun| sun.atmosphere.as_ref()).map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a.export()));
        // Space, if it is the background.
        let space_off = match &s.render.background {
            crate::scene::Background::Space(space) => space_record(&mut tables, space),
            _ => pathtrace::NO_AIR,
        };

        let mut flags = 0u32;
        if aerial_off != pathtrace::NO_AIR { flags |= pathtrace::FLAG_AERIAL; }
        if s.render.sample_lights { flags |= pathtrace::FLAG_SAMPLE_LIGHTS; }
        if s.environment.is_some() { flags |= pathtrace::FLAG_SAMPLE_ENV; }
        let (sun_dir, sun_irr) = match &s.sun {
            Some(sun) => {
                flags |= pathtrace::FLAG_SUN;
                // Above the air: the shader reddens it at each point (`sun_transmittance`).
                let e = sun.irradiance;
                (v4(&sun.direction, sun.angular_radius.cos() as f32), Vec4::new(e.rgb.x as f32, e.rgb.y as f32, e.rgb.z as f32, sun.angular_radius as f32))
            },
            None => (Vec4::new(0., 1., 0., 1.), Vec4::ZERO),
        };
        let (rr, rr_min) = match s.render.path_termination {
            PathTermination::RussianRoulette { min_depth } => (1, min_depth as f32),
            _ => (0, 0.),
        };
        let params = Params {
            dims: UVec4::new(s.image.width as u32, s.image.height as u32, 0, s.render.max_depth as u32),
            counts: UVec4::new(flat.prims.len() as u32, flat.planes.len() as u32, flat.emitters.len() as u32, s.lights.len() as u32),
            offs: UVec4::new(emitter_off, cdf_off, env_off, marginal_off),
            offs2: UVec4::new(conditional_off, ENV_WIDTH as u32, ENV_HEIGHT as u32, flags),
            misc: Vec4::new(s.render.regularise as f32, s.black_threshold as f32, s.render.shadow_bias as f32, rr_min),
            sun_dir,
            sun_irr,
            seed: UVec4::new(s.render.seed as u32, rr, 0, 0),
            air: UVec4::new(sky_off, aerial_off, sun_off, space_off),
        };
        Ok(Buffers { params, prims: pathtrace::pack_prims(&prims), prim_records: prims, nodes, materials, lights, tables })
    }
}

/// A buffer mapping's completion as a future: wgpu calls back when it's done (natively, during
/// `poll`; in a browser, from the event loop, so the render must yield to it).
#[derive(Default)]
struct Mapped(std::sync::Arc<Mutex<MapSlot>>);

/// The mapping's result once wgpu has called back, and the task waiting for it.
type MapSlot = (Option<Result<(), String>>, Option<std::task::Waker>);

impl std::future::Future for Mapped {
    type Output = Result<(), String>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut slot = self.0.lock().unwrap();
        match slot.0.take() {
            Some(r) => std::task::Poll::Ready(r),
            None => {
                slot.1 = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        }
    }
}

impl GpuRenderer {
    /// Upload the flattened scene, or say why the GPU can't render it (blocking; natively).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(s: &Scene, flat: &FlatScene) -> Result<GpuRenderer, String> {
        pollster::block_on(Self::create(s, flat))
    }

    /// Upload the flattened scene, or say why the GPU can't render it. Async, for a browser,
    /// whose WebGPU can't be waited on.
    pub async fn create(s: &Scene, flat: &FlatScene) -> Result<GpuRenderer, String> {
        let Buffers { params, prims, nodes, materials, lights, tables, .. } = Buffers::build(s, flat)?;
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }).await.map_err(|e| format!("no GPU adapter ({})", e))?;
        let info = adapter.get_info();
        // Browsers keep the GPU's name to themselves.
        let adapter_name = if info.name.is_empty() { format!("{:?}", info.backend) } else { format!("{} ({:?})", info.name, info.backend) };
        let mut limits = wgpu::Limits::default();
        let available = adapter.limits();
        // Refuse (and fall back) rather than let wgpu panic on a buffer the device can't bind.
        let pixels = (s.image.width * s.image.height) as u64;
        let largest = available.max_storage_buffer_binding_size.min(available.max_buffer_size);
        for (what, bytes) in [("primitives", std::mem::size_of_val(prims.as_slice()) as u64),
                              ("BVH nodes", std::mem::size_of_val(nodes.as_slice()) as u64),
                              ("tables", std::mem::size_of_val(tables.as_slice()) as u64),
                              ("paths", pixels * std::mem::size_of::<PathState>() as u64),
                              ("sky queries", 2 * pixels * std::mem::size_of::<SkyQuery>() as u64)] {
            if bytes > largest {
                return Err(format!("its {} need {} MB, more than the GPU's largest buffer ({} MB)", what, bytes >> 20, largest >> 20));
            }
        }
        limits.max_storage_buffer_binding_size = available.max_storage_buffer_binding_size;
        limits.max_buffer_size = available.max_buffer_size;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("rays"),
            required_limits: limits,
            ..Default::default()
        }).await.map_err(|e| format!("can't open the GPU ({})", e))?;

        // The shader, written in Rust (`shaders/pathtrace`) and compiled to SPIR-V by rust-gpu
        // (`scripts/build-shaders.sh`); a browser takes the WGSL translation made with it.
        // Vulkan takes the SPIR-V; other backends the WGSL, whose loops the builder has
        // rewritten for naga's MSL and HLSL writers (`fold_continuing_blocks` in
        // shaders/builder: from SPIR-V, Metal's loops left one iteration early). RAYS_GPU_WGSL=1 runs the WGSL
        // on Vulkan too, to test the translation without a browser.
        #[cfg(not(target_arch = "wasm32"))]
        let source = if info.backend != wgpu::Backend::Vulkan || std::env::var_os("RAYS_GPU_WGSL").is_some() {
            wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into())
        } else {
            wgpu::util::make_spirv(include_bytes!("pathtrace.spv"))
        };
        #[cfg(target_arch = "wasm32")]
        let source = wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into());
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("pathtrace"), source });
        // One layout for both kernels, so they share a bind group: the parameters, the scene's
        // buffers (read only), the paths, the image and the sky queries.
        let binding = |binding: u32, ty: wgpu::BufferBindingType| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::COMPUTE, count: None,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        };
        let read = wgpu::BufferBindingType::Storage { read_only: true };
        let write = wgpu::BufferBindingType::Storage { read_only: false };
        // A round's kernels: the scene (read only), the paths, the queues and the sky queries.
        // WebGPU allows a kernel eight storage buffers, so the image has a layout of its own,
        // with the kernel that adds to it.
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene"),
            entries: &[binding(0, wgpu::BufferBindingType::Uniform), binding(1, read), binding(2, read), binding(3, read),
                       binding(4, read), binding(5, read), binding(6, write), binding(7, write), binding(8, write)],
        });
        let add_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image"),
            entries: &[binding(0, wgpu::BufferBindingType::Uniform), binding(1, read), binding(2, write)],
        });
        let kernel = |entry_point: &str, layout: &wgpu::BindGroupLayout| {
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(entry_point), bind_group_layouts: &[Some(layout)], immediate_size: 0,
            });
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&pipeline_layout),
                module: &module,
                entry_point: Some(entry_point),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let (intersect_pipeline, bounce_pipeline, sky_pipeline, add_pipeline) =
            (kernel("intersect_paths", &layout), kernel("bounce_paths", &layout), kernel("answer_sky", &layout), kernel("add_paths", &add_layout));
        let storage = |label: &str, bytes: &[u8]| device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label), contents: bytes, usage: wgpu::BufferUsages::STORAGE,
        });
        let prims_buffer = storage("prims", bytemuck::cast_slice(&prims));
        let nodes_buffer = storage("nodes", bytemuck::cast_slice(&nodes));
        let materials_buffer = storage("materials", bytemuck::cast_slice(&materials));
        let lights_buffer = storage("lights", bytemuck::cast_slice(&lights));
        let tables_buffer = storage("tables", bytemuck::cast_slice(&tables));
        let params_buffers = [0, 1].map(|_| device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("params"), contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }));
        let queue_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("queues"), size: (pathtrace::QUEUE_HEADER as u64 + 2 * pixels) * 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let dispatch_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("dispatch"), size: 16,
            usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let empty_header = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("empty queue"), contents: bytemuck::cast_slice(&pathtrace::EMPTY_QUEUE), usage: wgpu::BufferUsages::COPY_SRC,
        });
        let paths_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("paths"), size: pixels * std::mem::size_of::<PathState>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let queries_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sky queries"), size: 2 * pixels * std::mem::size_of::<SkyQuery>() as u64,
            usage: wgpu::BufferUsages::STORAGE, mapped_at_creation: false,
        });
        let accum_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("accum"), size: pixels * 32,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let rounds_size = (COUNTED_PASSES * (params.dims.w as usize + 1) * 4) as u64;
        let rounds_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rounds"), size: rounds_size,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"), size: pixels * 32 + rounds_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let round_groups = [0, 1].map(|side| device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene"),
            layout: &layout,
            entries: &[entry(0, &params_buffers[side]), entry(1, &prims_buffer), entry(2, &nodes_buffer), entry(3, &materials_buffer),
                       entry(4, &lights_buffer), entry(5, &tables_buffer), entry(6, &paths_buffer), entry(7, &queue_buffer),
                       entry(8, &queries_buffer)],
        }));
        let add_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image"),
            layout: &add_layout,
            entries: &[entry(0, &params_buffers[0]), entry(1, &paths_buffer), entry(2, &accum_buffer)],
        });
        Ok(GpuRenderer { device, queue, intersect_pipeline, bounce_pipeline, sky_pipeline, add_pipeline, round_groups, add_group, params, params_buffers,
            queue_buffer, dispatch_buffer, empty_header, paths_buffer, accum_buffer, rounds_buffer, readback, block: Default::default(), adapter_name })
    }

    /// Camera rays for sample `k` of every pixel, as the wavefront integrator's.
    pub fn camera_rays(s: &Scene, k: usize) -> Vec<GpuRay> {
        Self::camera_rays_for(s, &vec![Some(k as u32); s.image.width * s.image.height])
    }

    /// The camera ray of each pixel's sample `samples[pixel]`; a dead ray (as one the lens
    /// stops) for a pixel that takes none.
    pub fn camera_rays_for(s: &Scene, samples: &[Option<u32>]) -> Vec<GpuRay> {
        let (w, h) = (s.image.width, s.image.height);
        let pixels = (w * h) as u64;
        (0..w * h).into_par_iter().map(|index| {
            let Some(k) = samples[index] else { return GpuRay::zeroed() };
            let k = k as usize;
            let (x, y) = (index % w, index / w);
            let stream = k as u64 * pixels + index as u64;
            crate::geometry::seed_rand(s.render.seed ^ 0x5bd1_e995, stream);
            let mut sampler = crate::sobol::SampleState::new(crate::sobol::hash(s.render.seed, index as u64), k as u32);
            let ((fx, wx), (fy, wy)) = (s.render.filter.sample_weighted(sampler.next()), s.render.filter.sample_weighted(sampler.next()));
            match s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, (0.5 + fx) / w as f64, (0.5 + fy) / h as f64) {
                Some((ray, weight)) => GpuRay { o: v4(&ray.ro, (weight * wx * wy) as f32), d: v4(&ray.rd.normalize(), 1.) },
                None => GpuRay::zeroed(),
            }
        }).collect()
    }

    /// Zero the image accumulated on the GPU.
    pub fn clear(&self) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.clear_buffer(&self.accum_buffer, 0, None);
        self.queue.submit(Some(encoder.finish()));
    }

    /// Queue pass (sample) `k` of every pixel, adding to the image on the GPU. Returns at once;
    /// a pass is one dispatch, so a browser can give each frame one, within its GPU watchdog.
    pub fn dispatch(&mut self, s: &Scene, k: usize) {
        self.dispatch_samples(s, &vec![Some(k as u32); s.image.width * s.image.height]);
    }

    /// Queue a pass in which each pixel takes its sample `samples[pixel]`, or none: the pixels
    /// the scheduler has found complete (`scheduler::plan`) aren't generated, so cost nothing.
    /// A sample's random numbers are its pixel's and its own index's, whichever pass it is in.
    pub fn dispatch_samples(&mut self, s: &Scene, samples: &[Option<u32>]) {
        let began = web_time::Instant::now();
        let pixels = (s.image.width * s.image.height) as u32;
        for side in 0..2 {
            self.params.seed.z = side as u32;
            self.queue.write_buffer(&self.params_buffers[side], 0, bytemuck::bytes_of(&self.params));
        }
        let seed = self.params.seed.x;
        let rays = Self::camera_rays_for(s, samples);
        // (A pixel that takes no sample has a dead ray, so a path that adds nothing.)
        let paths: Vec<PathState> = rays.iter().enumerate()
            .map(|(i, ray)| PathState::start(i as u32, samples[i].unwrap_or(0), seed, ray)).collect();
        self.queue.write_buffer(&self.paths_buffer, 0, bytemuck::cast_slice(&paths));
        // The first round's queue (side 0): every pixel whose camera ray got through the lens.
        // Side 1 starts empty.
        let alive: Vec<u32> = paths.iter().enumerate().filter(|(_, p)| p.state.y & pathtrace::PATH_ACTIVE != 0).map(|(i, _)| i as u32).collect();
        let first = pathtrace::queue_header(alive.len() as u32);
        let mut head = first.to_vec();
        head.extend_from_slice(&pathtrace::EMPTY_QUEUE);
        head.extend_from_slice(&alive);
        self.queue.write_buffer(&self.queue_buffer, 0, bytemuck::cast_slice(&head));
        self.queue.write_buffer(&self.dispatch_buffer, 0, bytemuck::cast_slice(&first));
        // The host's part ends here: the rays generated and uploaded. Encoding and submitting
        // the kernels is counted as the device's, since a driver may run them as they arrive.
        let seconds = began.elapsed().as_secs_f64();
        // For the scheduler's measurements: this pass's slot for its rounds' counts, if there
        // is one left before the next read.
        let rounds = self.rounds();
        let slot = {
            let block = self.block.get_mut().unwrap();
            block.started.get_or_insert(began);
            (block.counted < COUNTED_PASSES).then(|| {
                block.counted += 1;
                (block.counted - 1) * rounds
            })
        };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // A path has at most `max_depth` + 1 vertices: that many rounds, each over the paths
        // the last left alive.
        for round in 0..=self.params.dims.w {
            let side = (round % 2) as usize;
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_bind_group(0, &self.round_groups[side], &[]);
                pass.set_pipeline(&self.intersect_pipeline);
                pass.dispatch_workgroups_indirect(&self.dispatch_buffer, 0);
                pass.set_pipeline(&self.bounce_pipeline);
                pass.dispatch_workgroups_indirect(&self.dispatch_buffer, 0);
                pass.set_pipeline(&self.sky_pipeline);
                pass.dispatch_workgroups_indirect(&self.dispatch_buffer, 0);
            }
            // The next round reads the side this one filled, and fills the side it read.
            encoder.copy_buffer_to_buffer(&self.queue_buffer, 16 * (1 - side as u64), &self.dispatch_buffer, 0, 16);
            if let Some(slot) = slot.filter(|_| round < self.params.dims.w) {
                // How many paths the next round starts with (the header's count).
                encoder.copy_buffer_to_buffer(&self.queue_buffer, 16 * (1 - side as u64) + 12, &self.rounds_buffer, 4 * (slot + round as usize + 1) as u64, 4);
            }
            encoder.copy_buffer_to_buffer(&self.empty_header, 0, &self.queue_buffer, 16 * side as u64, 16);
        }
        {
            let groups = pixels.div_ceil(pathtrace::WORKGROUP);
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_bind_group(0, &self.add_group, &[]);
            pass.set_pipeline(&self.add_pipeline);
            pass.dispatch_workgroups(groups.min(pathtrace::MAX_GROUPS), groups.div_ceil(pathtrace::MAX_GROUPS), 1);
        }
        self.queue.submit(Some(encoder.finish()));
        // What the host knows of the pass already: the paths it generated (which are the first
        // round's), and that every pixel's path is added to the film. The later rounds are
        // counted at the read.
        use crate::scheduler::Stage;
        let block = self.block.get_mut().unwrap();
        block.host_seconds += seconds;
        block.stats.passes += 1;
        block.stats.record(Stage::Generate, alive.len() as u64, seconds);
        block.stats.record(Stage::Accumulate, pixels as u64, 0.);
        if slot.is_some() {
            block.stats.round(0, alive.len() as u64);
            self.record_round(alive.len() as u64);
        }
    }

    /// Rounds in a pass: a path has at most `max_depth` + 1 vertices.
    fn rounds(&self) -> usize {
        self.params.dims.w as usize + 1
    }

    /// A round of `paths` went through its three kernels. (Their time isn't known apart:
    /// `scheduler`'s module documentation.)
    fn record_round(&self, paths: u64) {
        use crate::scheduler::Stage;
        if paths > 0 {
            let mut block = self.block.lock().unwrap();
            for stage in [Stage::Intersect, Stage::Shade, Stage::Sky] {
                block.stats.record(stage, paths, 0.);
            }
        }
    }

    /// What the stages have done since this was last asked, complete up to the last read.
    pub fn take_stats(&self) -> crate::scheduler::Stats {
        std::mem::take(&mut self.block.lock().unwrap().stats)
    }

    /// The image accumulated so far: each pixel's sums (the radiance's, and in w the number of
    /// rays traced). Natively this waits for the GPU; in a browser it yields until the copy
    /// can be read.
    pub async fn read_back(&self) -> Result<(Vec<[f32; 4]>, Vec<[f32; 4]>), String> {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.accum_buffer, 0, &self.readback, 0, None);
        let image = self.accum_buffer.size();
        let counted = std::mem::take(&mut self.block.lock().unwrap().counted);
        if counted > 0 {
            encoder.copy_buffer_to_buffer(&self.rounds_buffer, 0, &self.readback, image, (counted * self.rounds() * 4) as u64);
        }
        self.queue.submit(Some(encoder.finish()));
        let mapped = Mapped::default();
        let slot = mapped.0.clone();
        self.readback.map_async(wgpu::MapMode::Read, .., move |r| {
            let mut slot = slot.lock().unwrap();
            slot.0 = Some(r.map_err(|e| e.to_string()));
            if let Some(waker) = slot.1.take() {
                waker.wake();
            }
        });
        #[cfg(not(target_arch = "wasm32"))]
        self.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| format!("GPU device lost ({})", e))?;
        mapped.await.map_err(|e| format!("couldn't read the GPU's image back ({})", e))?;
        let both = bytemuck::cast_slice::<u8, [f32; 4]>(&self.readback.get_mapped_range(..image).unwrap()).to_vec();
        let rounds = bytemuck::cast_slice::<u8, u32>(&self.readback.get_mapped_range(image..image + (counted * self.rounds() * 4) as u64).unwrap()).to_vec();
        self.readback.unmap();
        for pass in rounds.chunks(self.rounds()) {
            for (round, &paths) in pass.iter().enumerate().skip(1) {
                self.block.lock().unwrap().stats.round(round, paths as u64);
                self.record_round(paths as u64);
            }
        }
        {
            // The time since the first of these passes was queued that wasn't the host's.
            let mut block = self.block.lock().unwrap();
            if let Some(started) = block.started.take() {
                let device = started.elapsed().as_secs_f64() - std::mem::take(&mut block.host_seconds);
                block.stats.device_seconds += device.max(0.);
            }
        }
        // Interleaved on the GPU (`pathtrace::add_paths`): a pixel's sums, then its squares.
        Ok((both.iter().step_by(2).copied().collect(), both.iter().skip(1).step_by(2).copied().collect()))
    }

    /// Render passes `first..last`, accumulating on the GPU; returns the sums per pixel.
    #[cfg(not(target_arch = "wasm32"))]
    fn run(&mut self, s: &Scene, passes: std::ops::Range<usize>) -> Vec<[f32; 4]> {
        self.clear();
        for k in passes {
            self.dispatch(s, k);
        }
        pollster::block_on(self.read_back()).expect("GPU readback").0
    }

    /// Render up to the scene's samples per pixel from where `rc` has got to, reading the
    /// image back every `PASSES_PER_READ` passes; `after` sees the context each time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(&mut self, s: &Scene, rc: &Mutex<RenderContext>, mut after: impl FnMut(&RenderContext)) {
        let (w, h) = (s.image.width, s.image.height);
        loop {
            // The scheduler's part (`scheduler::plan`): which pixels take samples in the next
            // few passes, and how many each. Decided a region at a time, as the recursive
            // tracer's passes are, from the film alone, so a render resumed from a saved film
            // goes on as an uninterrupted one would.
            let (mut first, mut count) = (vec![0usize; w * h], vec![0usize; w * h]);
            {
                let rc = rc.lock().unwrap();
                for b in rc.regions(s) {
                    let (f, c) = rc.plan(s, &b, PASSES_PER_READ);
                    let mut i = 0;
                    for y in b.ymin..b.ymax {
                        for x in b.xmin..b.xmax {
                            first[y * w + x] = f[i];
                            count[y * w + x] = c[i];
                            i += 1;
                        }
                    }
                }
            }
            let passes = count.iter().copied().max().unwrap_or(0);
            if passes == 0 {
                break;
            }
            self.clear();
            for j in 0..passes {
                let samples: Vec<Option<u32>> = (0..w * h).map(|i| (j < count[i]).then(|| (first[i] + j) as u32)).collect();
                self.dispatch_samples(s, &samples);
            }
            let (sums, squares) = pollster::block_on(self.read_back()).expect("GPU readback");
            let mut rc = rc.lock().unwrap();
            rc.schedule.merge(&self.take_stats());
            for y in 0..h {
                for x in 0..w {
                    let i = y * w + x;
                    if count[i] == 0 {
                        continue;
                    }
                    let colour = |v: [f32; 4]| Color::new(v[0] as f64, v[1] as f64, v[2] as f64);
                    // A sample the lens stopped added nothing, and is a black sample all the same.
                    rc.set_pixel(x, y, colour(sums[i]), colour(squares[i]), count[i]);
                    rc.rays_cast += sums[i][3] as u64;
                }
            }
            after(&rc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use pathtrace::glam::Vec3;

    fn g3(v: &Vector3<f64>) -> Vec3 {
        Vec3::new(v.x as f32, v.y as f32, v.z as f32)
    }

    /// No loop in the WGSL has a continuing block or a `break if`, which naga's MSL and HLSL
    /// writers emit before the loop's body, re-evaluating the body's values there after the
    /// block's stores (`fold_continuing_blocks` in shaders/builder). Metal and DX12 take this
    /// WGSL; this checks it without them.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_loops_have_no_continuing_blocks() {
        use wgpu::naga::{Block, Statement};
        fn check(block: &Block, loops: &mut usize) {
            for s in block.iter() {
                match s {
                    Statement::Loop { body, continuing, break_if } => {
                        assert!(continuing.is_empty() && break_if.is_none(), "a loop with a continuing block or a break-if");
                        *loops += 1;
                        check(body, loops);
                    },
                    Statement::Block(b) => check(b, loops),
                    Statement::If { accept, reject, .. } => {
                        check(accept, loops);
                        check(reject, loops);
                    },
                    Statement::Switch { cases, .. } => cases.iter().for_each(|c| check(&c.body, loops)),
                    _ => {},
                }
            }
        }
        let module = wgpu::naga::front::wgsl::parse_str(include_str!("pathtrace.wgsl")).unwrap();
        let mut loops = 0;
        for f in module.functions.iter().map(|(_, f)| f).chain(module.entry_points.iter().map(|ep| &ep.function)) {
            check(&f.body, &mut loops);
        }
        assert!(loops > 0, "no loops found: is the shader the one expected?");
    }

    /// A scene of sky alone (no objects) makes no empty buffers, which wgpu can't bind; it
    /// rendered with a panic rather than an image.
    #[test]
    fn test_a_scene_without_objects_has_no_empty_buffers() {
        let s = SceneFile::from_string(r#"{ "width": 8, "height": 8, "supersamples": 1, "background": [0.2, 0.25, 0.3],
            "camera": { "location": [0, 1, -5], "lookat": [0, 1, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {}, "objects": [] }"#.to_string());
        let flat = FlatScene::build(&s).unwrap();
        let b = Buffers::build(&s, &flat).unwrap();
        for (what, len) in [("prims", b.prims.len()), ("nodes", b.nodes.len()), ("materials", b.materials.len()),
                            ("lights", b.lights.len()), ("tables", b.tables.len())] {
            assert!(len > 0, "{} is empty", what);
        }
        let Ok(mut gpu) = GpuRenderer::new(&s, &flat) else { return };
        let sums = gpu.run(&s, 0..1);
        assert!(sums.iter().all(|v| (v[0] - 0.2).abs() < 1e-3), "the background, not {:?}", sums[0]);
    }

    /// The shader's air against the renderer's, ray by ray: the sky's radiance (from its
    /// tables and integrated), aerial perspective and sunlight, from the ground to orbit.
    #[test]
    fn test_air_matches_the_renderer() {
        let scene = |sky: &str| SceneFile::from_string(format!(r#"{{
            "width": 8, "height": 8, "supersamples": 1, "background": [0, 0, 0],
            "camera": {{ "location": [0, 2, 0], "lookat": [0, 2, 10], "up": [0, 1, 0], "angle": 0.6 }},
            "sun": {{ "intensity": 6 }},
            "materials": {{ "G": {{ "type": "lambertian", "albedo": [0.3, 0.3, 0.3] }} }},
            "objects": [ {{ "type": "plane", "y": 0, "material": "G" }}, {sky} ]
        }}"#));
        let skies = [
            ("tables", r#"{ "type": "skysphere", "sun_direction": [0.3, 0.4, 1] }"#),
            ("exact", r#"{ "type": "skysphere", "sun_direction": [0.3, 0.4, 1], "lut": false }"#),
            ("sunset", r#"{ "type": "skysphere", "sun_direction": [0, 0.02, 1] }"#),
            ("night", r#"{ "type": "skysphere", "sun_direction": [0, -0.3, 0.95] }"#),
            // The sun far below the horizon: the moon's light is all there is.
            ("moonlit", r#"{ "type": "skysphere", "sun_direction": [0, -0.6, 0.8], "moonlight": { "direction": [0.2, 0.5, -1], "share": 2e-6 } }"#),
        ];
        let altitudes = [1.5, 50., 199., 201., 2000., 30e3, 400e3];
        let mut worst: Vec<(String, f64)> = vec![];
        for (name, sky) in skies {
            let s = scene(sky);
            let flat = FlatScene::build(&s).unwrap();
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
            let (mut sky_err, mut seg_err, mut sun_err) = (0f64, 0f64, 0f64);
            for h in altitudes {
                let o = Vector3::new(30., h, -20.);
                for d in crate::procedural::space::fibonacci_sphere(300) {
                    let cpu = crate::wavefront::integrator::sky_radiance(&Ray { ro: o, rd: d }, &s, &flat).rgb;
                    let gpu = pathtrace::air_radiance(&shader, b.params.air.x, g3(&o), g3(&d));
                    for ch in 0..3 {
                        sky_err = sky_err.max((gpu[ch] as f64 - cpu[ch]).abs() / (cpu.max() + 1e-12));
                    }
                    if let Some(air) = &s.atmosphere {
                        for dist in [500., 20e3, 300e3] {
                            let (t, ins) = air.segment(&o, &d, dist);
                            let (gt, gi) = pathtrace::aerial_segment(&shader, g3(&o), g3(&d), dist as f32);
                            for ch in 0..3 {
                                seg_err = seg_err.max((gt[ch] as f64 - t[ch]).abs());
                                seg_err = seg_err.max((gi[ch] as f64 - ins[ch]).abs() / (ins.max() + 1e-12));
                            }
                        }
                    }
                }
                if let Some(air) = s.sun.as_ref().and_then(|sun| sun.atmosphere.as_ref()) {
                    let t = air.sun_transmittance(&o);
                    let g = pathtrace::sun_transmittance(&shader, g3(&o));
                    for ch in 0..3 {
                        sun_err = sun_err.max((g[ch] as f64 - t[ch]).abs());
                    }
                }
            }
            eprintln!("AIR {:8} sky {:.2e} aerial {:.2e} sun {:.2e}", name, sky_err, seg_err, sun_err);
            worst.push((format!("{} sky", name), sky_err));
            worst.push((format!("{} aerial", name), seg_err));
            worst.push((format!("{} sun", name), sun_err));
        }
        // f32 against f64: about 1e-5 of the brightest channel (3e-5 at worst when written).
        for (what, e) in &worst {
            assert!(*e < 2e-4, "{}: {:.2e}", what, e);
        }
    }

    /// The shader's space against the renderer's, direction by direction: its radiance (stars,
    /// the Milky Way, nebulae, galaxies), its bound, and the sky with space showing through it.
    #[test]
    fn test_space_matches_the_renderer() {
        for name in ["demo/scenes/deep-space.json", "demo/scenes/night-orbit.json"] {
            let s = SceneFile::from_file(name, &crate::scenefile::Overrides { width: Some(64), height: Some(36), seed: None });
            let flat = FlatScene::build(&s).unwrap();
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
            let crate::scene::Background::Space(space) = &s.render.background else { panic!("{} has no space", name) };
            if name.contains("deep-space") {
                assert!(space.stars.is_some() && space.milky_way.is_some() && !space.nebulae.is_empty() && !space.galaxies.is_empty(),
                    "deep-space should exercise every part of space");
            }
            // Directions all round, and onto (and just off) stars, which are a pixel or two wide.
            let mut dirs: Vec<Vector3<f64>> = crate::procedural::space::fibonacci_sphere(4000).collect();
            if let Some(stars) = &space.stars {
                let (sigma, _, _, list, _) = stars.export(&space.frame);
                for st in list.iter().step_by(97).take(400) {
                    let c = Vector3::new(st[0], st[1], st[2]);
                    let (t, _) = crate::procedural::space::tangent_basis(&c);
                    dirs.push(c);
                    dirs.push((c + t * sigma).normalize());
                    dirs.push((c + t * 3. * sigma).normalize());
                }
            }
            let (mut worst, mut worst_bound, mut worst_sky) = (0f64, 0f64, 0f64);
            let o = s.camera.get_ray(0.5, 0.5, 0., 0.).ro;
            for d in &dirs {
                let cpu = space.radiance(d).rgb;
                let gpu = pathtrace::space_radiance_at(&shader, g3(d));
                let scale = crate::color::luminance(&space.radiance(d)).max(space.bound(d) * 1e-3).max(1e-30);
                for ch in 0..3 {
                    worst = worst.max((gpu[ch] as f64 - cpu[ch]).abs() / scale);
                }
                let (bc, bg) = (space.bound(d), pathtrace::space_bound_at(&shader, g3(d)) as f64);
                worst_bound = worst_bound.max((bg - bc).abs() / bc.max(1e-30));
                let cpu_sky = crate::wavefront::integrator::sky_radiance(&Ray { ro: o, rd: *d }, &s, &flat).rgb;
                let gpu_sky = pathtrace::background(&shader, g3(&o), g3(d));
                for ch in 0..3 {
                    worst_sky = worst_sky.max((gpu_sky[ch] as f64 - cpu_sky[ch]).abs() / cpu_sky.max().max(1e-30));
                }
            }
            // f32 against f64 (noise sampled hundreds of units out, stars' Gaussians a pixel
            // wide): 4e-4 of the local brightness at worst when written.
            eprintln!("SPACE {} radiance {:.2e} bound {:.2e} sky {:.2e}", name, worst, worst_bound, worst_sky);
            assert!(worst < 3e-3 && worst_bound < 1e-5 && worst_sky < 3e-3, "{}: radiance {:.2e} bound {:.2e} sky {:.2e}", name, worst, worst_bound, worst_sky);
        }
    }

    /// From orbit the GPU sees the sky and the sun the CPU does: its sky map and the sun's
    /// reddening are found from the camera, not the world's origin (on the night side's
    /// ground, where the sun had set, which rendered `limb-sunset` black).
    /// Checked by the shader's CPU build over the GPU's buffers, so without a GPU too.
    #[test]
    fn test_sees_the_sky_from_orbit() {
        let s = SceneFile::from_string(r#"{
            "width": 32, "height": 16, "supersamples": 16, "max_depth": 4, "background": [0, 0, 0],
            "camera": { "location": [0, 400000, 0], "lookat": [0, 40000, 1000000], "up": [0, 1, 0], "angle": 0.3 },
            "sun": { "intensity": 6 },
            "materials": { "GROUND": { "type": "lambertian", "albedo": [0.06, 0.09, 0.15] } },
            "objects": [ { "type": "planet", "material": "GROUND" }, { "type": "skysphere", "sun_direction": [0, -0.3, 0.95] } ]
        }"#.to_string());
        let flat = FlatScene::build(&s).unwrap();
        let b = Buffers::build(&s, &flat).unwrap();
        let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
        let passes = 16;
        let mut on_shader = Vector3::zeros();
        for k in 0..passes {
            for (index, ray) in GpuRenderer::camera_rays(&s, k).iter().enumerate() {
                if ray.d.w < 0.5 {
                    continue;
                }
                let mut rng = pathtrace::path_seed(index as u32, k as u32, b.params.seed.x);
                let c = pathtrace::radiance(&shader, ray.o.truncate(), ray.d.truncate(), &mut rng, &mut 0) * ray.o.w;
                on_shader += Vector3::new(c.x as f64, c.y as f64, c.z as f64) / passes as f64;
            }
        }
        let cpu = crate::wavefront::integrator::tests::mean_image(&s, passes).iter().fold(Vector3::zeros(), |a, c| a + c.rgb);
        for ch in 0..3 {
            assert!((on_shader[ch] - cpu[ch]).abs() < 0.05 * cpu[ch], "channel {}: shader {} vs CPU {}", ch, on_shader[ch], cpu[ch]);
        }
        let Ok(mut gpu) = GpuRenderer::new(&s, &flat) else { return };
        let on_gpu = gpu.run(&s, 0..passes).iter().fold(Vector3::zeros(), |a, v| a + Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64)) / passes as f64;
        for ch in 0..3 {
            assert!((on_gpu[ch] - cpu[ch]).abs() < 0.05 * cpu[ch], "channel {}: GPU {} vs CPU {}", ch, on_gpu[ch], cpu[ch]);
        }
    }

    /// The shader's functions, run on the CPU, agree with the renderer's own.
    #[test]
    fn test_shader_functions_match_the_renderer() {
        use crate::material::bsdf::BSDF;
        crate::geometry::seed_rand(1, 0);
        let r = crate::geometry::rand;
        for _ in 0..2000 {
            let (cos, ior) = (r(), 1. + r() * 1.5);
            for ni_over_nt in [1. / ior, ior] {
                let cpu = crate::material::functions::fresnel_reflectance(cos, ni_over_nt, ior);
                let gpu = pathtrace::fresnel_reflectance(cos as f32, ni_over_nt as f32, ior as f32) as f64;
                // f32, and near total internal reflection a square root steepens the error.
                assert!((cpu - gpu).abs() < 1e-4, "fresnel {} {}: {} vs {}", cos, ni_over_nt, cpu, gpu);
            }
            let (eta, k) = (0.1 + r() * 2., r() * 5.);
            let cpu = crate::material::ggx::conductor_reflectance(cos, eta, k);
            let gpu = pathtrace::conductor_reflectance(cos as f32, eta as f32, k as f32) as f64;
            assert!((cpu - gpu).abs() < 1e-4, "conductor {} {} {}: {} vs {}", cos, eta, k, cpu, gpu);
        }
        // GGX with Schlick's Fresnel: f cos and the density, about a tilted normal.
        let n = Vector3::new(0.3, 0.8, -0.2).normalize();
        let f0 = Color::new(0.9, 0.6, 0.3);
        for alpha in [0.05, 0.3, 0.8] {
            let cpu = crate::material::ggx::GGX::new(alpha, crate::material::ggx::Fresnel::Schlick(f0), n);
            let mut gpu = pathtrace::Lobe::new(1, g3(&n));
            gpu.alpha = alpha as f32;
            gpu.albedo = Vec3::new(0.9, 0.6, 0.3);
            for _ in 0..2000 {
                let d = |u: f64, v: f64| (n + crate::geometry::point_on_unit_sphere(u, v)).normalize();
                let (wo, wi) = (d(r(), r()), d(r(), r()));
                let (fc, fg) = (cpu.eval(&wo, &wi), gpu.eval(g3(&wo), g3(&wi)));
                let (pc, pg) = (cpu.pdf(&wo, &wi), gpu.pdf(g3(&wo), g3(&wi)) as f64);
                assert!((fc.rgb.x - fg.x as f64).abs() <= 1e-3 * (1. + fc.rgb.x), "alpha {}: f {} vs {}", alpha, fc.rgb.x, fg.x);
                assert!((pc - pg).abs() <= 1e-3 * (1. + pc), "alpha {}: pdf {} vs {}", alpha, pc, pg);
            }
        }
        // A sphere light's density from outside and inside.
        let sphere = crate::shapes::sphere::Sphere::new(Vector3::new(1., 2., 3.), 0.5);
        for from in [Vector3::new(4., 0., 1.), Vector3::new(1.1, 2.2, 3.1)] {
            use crate::shapes::geometry::Geometry;
            let (y, ny, _) = sphere.sample_from(&from, [0.3, 0.6, 0.2]).unwrap();
            let cpu = sphere.pdf_from(&from, &y, &ny);
            let gpu = pathtrace::sphere_pdf(g3(&sphere.center), 0.5, g3(&from), g3(&y), g3(&ny)) as f64;
            assert!((cpu - gpu).abs() < 1e-3 * cpu, "{} vs {}", cpu, gpu);
        }
    }

    /// The whole shader, run on the CPU over the same buffers, renders what it renders on the
    /// GPU: a check on the compilation to SPIR-V and its translation for the device.
    #[test]
    fn test_shader_on_the_cpu_matches_the_gpu() {
        // Every material and light, and a sky of air with space showing through it from orbit.
        let orbit = SceneFile::from_file("demo/scenes/night-orbit.json", &crate::scenefile::Overrides { width: Some(24), height: Some(16), seed: None });
        for (name, s) in [("materials", scene()), ("night-orbit", orbit)] {
            let flat = FlatScene::build(&s).unwrap();
            let mut gpu = match GpuRenderer::new(&s, &flat) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("skipped: {}", e);
                    return;
                },
            };
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
            let passes = 32;
            let mut cpu = Vector3::zeros();
            let mut cpu_rays = 0u64;
            for k in 0..passes {
                for (index, ray) in GpuRenderer::camera_rays(&s, k).iter().enumerate() {
                    if ray.d.w < 0.5 {
                        continue;
                    }
                    let mut rng = pathtrace::path_seed(index as u32, k as u32, b.params.seed.x);
                    let mut rays = 0;
                    let c = pathtrace::radiance(&shader, ray.o.truncate(), ray.d.truncate(), &mut rng, &mut rays) * ray.o.w;
                    cpu_rays += rays as u64;
                    cpu += Vector3::new(c.x as f64, c.y as f64, c.z as f64);
                }
            }
            let sums = gpu.run(&s, 0..passes);
            let on_gpu = sums.iter().fold(Vector3::zeros(), |a, v| a + Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64));
            for ch in 0..3 {
                assert!((on_gpu[ch] - cpu[ch]).abs() < 0.01 * cpu[ch], "{} channel {}: GPU {} vs CPU {}", name, ch, on_gpu[ch], cpu[ch]);
            }
            // And traces as many rays (w).
            let gpu_rays: f64 = sums.iter().map(|v| v[3] as f64).sum();
            assert!((gpu_rays - cpu_rays as f64).abs() < 0.01 * cpu_rays as f64, "{} rays: GPU {} vs CPU {}", name, gpu_rays, cpu_rays);
        }
    }

    /// The shader's traversal finds what a search of every primitive finds: the nearest hit, and
    /// whether anything is in the way, for trees of every shape (none, one leaf, deep), with
    /// planes, and for ranges that end before or begin beyond the nearest surface.
    #[test]
    fn test_traversal_matches_brute_force() {
        use pathtrace::glam::Vec3;
        let mut rng = crate::geometry::Pcg32::new(11, 0);
        let mut next = move || rng.next_f64();
        for (spheres, planes) in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (9, 1), (300, 0), (2000, 2)] {
            let mut objects = Vec::new();
            for _ in 0..spheres {
                objects.push(format!(r#"{{ "type": "sphere", "location": [{}, {}, {}], "radius": {}, "material": "M" }}"#,
                    next() * 20. - 10., next() * 20. - 10., next() * 20. - 10., 0.05 + next() * 0.6));
            }
            for i in 0..planes {
                objects.push(format!(r#"{{ "type": "plane", "y": {}, "material": "M" }}"#, -11. - i as f64));
            }
            let s = SceneFile::from_string(format!(r#"{{
                "width": 4, "height": 4, "background": [0, 0, 0],
                "camera": {{ "location": [0, 0, -30], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
                "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
                "objects": [{}]
            }}"#, objects.join(",")));
            let flat = FlatScene::build(&s).unwrap();
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
            let count = (b.params.counts.x + b.params.counts.y) as usize;
            assert_eq!(count, spheres + planes);
            // Every primitive on its own is a scene the traversal searches in full.
            let alone: Vec<(Params, Vec<pathtrace::glam::Vec4>, Vec<GpuNode>)> = (0..count).map(|i| {
                let plane = i >= b.params.counts.x as usize;
                let mut params = b.params;
                params.counts.x = if plane { 0 } else { 1 };
                params.counts.y = if plane { 1 } else { 0 };
                let nodes = gpu_nodes(&if plane { vec![] } else { vec![(flat.bvh.export()[0].0, 0, 1)] }).unwrap();
                (params, pathtrace::pack_prims(&[b.prim_records[i]]), nodes)
            }).collect();
            for _ in 0..400 {
                let ro = Vec3::new(next() as f32 * 30. - 15., next() as f32 * 30. - 15., next() as f32 * 30. - 15.);
                let rd = Vec3::new(next() as f32 - 0.5, next() as f32 - 0.5, next() as f32 - 0.5).normalize();
                let (tmin, tmax) = match (next() * 3.) as u32 { 0 => (0., f32::MAX), 1 => (0., next() as f32 * 20.), _ => (next() as f32 * 20., f32::MAX) };
                let mut expected = (tmax, u32::MAX);
                for (i, (params, prim, nodes)) in alone.iter().enumerate() {
                    let single = pathtrace::Scene { params, prims: prim, nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
                    let (t, hit) = pathtrace::nearest(&single, ro, rd, tmin, expected.0, false);
                    if hit != u32::MAX { expected = (t, i as u32); }
                }
                let (t, prim) = pathtrace::nearest(&shader, ro, rd, tmin, tmax, false);
                assert_eq!((t, prim != u32::MAX), (expected.0, expected.1 != u32::MAX), "{} spheres, {} planes: ray {:?} {:?} in ({}, {})", spheres, planes, ro, rd, tmin, tmax);
                let (_, any) = pathtrace::nearest(&shader, ro, rd, tmin, tmax, true);
                assert_eq!(any != u32::MAX, expected.1 != u32::MAX, "any hit: {} spheres, {} planes", spheres, planes);
            }
        }
    }

    /// How much work the shader's traversal does on scenes of geometry: box tests and
    /// primitive tests per ray, counted on the shader's CPU build (the GPU runs the same code,
    /// so does the same work). Not a test of anything; run it for the figures:
    /// `cargo test --release --features gpu measure_traversal -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn measure_traversal() {
        println!("{:<16} {:>9} {:>9} {:>10} {:>10} {:>10}", "scene", "prims", "nodes", "rays", "boxes/ray", "prims/ray");
        for name in ["demo/demo.json", "demo/scenes/instances.json", "demo/scenes/glossy-sun.json", "demo/scenes/spheres.json", "demo/scenes/lamps.json", "demo/scenes/night-orbit.json"] {
            let s = SceneFile::from_file(name, &crate::scenefile::Overrides { width: Some(160), height: Some(90), seed: None });
            let flat = match FlatScene::build(&s) {
                Ok(f) => f,
                Err(e) => { println!("{:<16} not flattened: {}", name, e); continue; },
            };
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = pathtrace::Scene { params: &b.params, prims: &b.prims, nodes: &b.nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables };
            pathtrace::stats::take();
            let mut total_rays = 0u64;
            for k in 0..2 {
                for (index, ray) in GpuRenderer::camera_rays(&s, k).iter().enumerate() {
                    if ray.d.w < 0.5 { continue; }
                    let mut rng = pathtrace::path_seed(index as u32, k as u32, b.params.seed.x);
                    let mut rays = 0;
                    pathtrace::radiance(&shader, ray.o.truncate(), ray.d.truncate(), &mut rng, &mut rays);
                    total_rays += rays as u64;
                }
            }
            let (boxes, prims) = pathtrace::stats::take();
            // The queues' saving: of the invocations a pass would make with every pixel
            // dispatched every round, the share that had a live path to advance.
            let (mut live, mut slots) = (0u64, 0u64);
            for (index, ray) in GpuRenderer::camera_rays(&s, 0).iter().enumerate() {
                let mut p = PathState::start(index as u32, 0, b.params.seed.x, ray);
                for _ in 0..=b.params.dims.w {
                    slots += 1;
                    if p.state.y & pathtrace::PATH_ACTIVE != 0 {
                        live += 1;
                        p = pathtrace::bounce(&shader, &p).0;
                    }
                }
            }
            pathtrace::stats::take();
            let live_share = live as f64 / slots as f64;
            // How the work divides: the share of the megakernel's time (on this CPU, one
            // thread) that is traversal, closest hits and shadow rays together.
            pathtrace::stats::time_traversal(true);
            let start = std::time::Instant::now();
            for (index, ray) in GpuRenderer::camera_rays(&s, 0).iter().enumerate() {
                if ray.d.w < 0.5 { continue; }
                let mut rng = pathtrace::path_seed(index as u32, 0, b.params.seed.x);
                let mut rays = 0;
                pathtrace::radiance(&shader, ray.o.truncate(), ray.d.truncate(), &mut rng, &mut rays);
            }
            let traversal_share = pathtrace::stats::trace_seconds() / start.elapsed().as_secs_f64();
            pathtrace::stats::time_traversal(false);
            pathtrace::stats::take();
            let short = name.rsplit('/').next().unwrap().trim_end_matches(".json");
            println!("{:<16} {:>9} {:>9} {:>10} {:>10.1} {:>10.1}   (tree {} levels deep; max depth {}, {:.0}% of path-rounds live, traversal {:.0}% of the time)", short, b.prim_records.len(), b.nodes.len(), total_rays,
                boxes as f64 / total_rays as f64, prims as f64 / total_rays as f64, bvh_depth(&flat.bvh.export()), b.params.dims.w, 100. * live_share, 100. * traversal_share);
        }
    }

    /// Every material and kind of light the GPU path has.
    fn scene() -> Scene {
        SceneFile::from_string(r#"{
            "width": 24, "height": 16, "supersamples": 256, "background": [0.2, 0.25, 0.3], "max_depth": 6,
            "path_termination": "russian_roulette",
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
        }"#.to_string())
    }

    /// The review's hard cases (`wavefront::integrator::tests::hard_scene`: a two-sided plane
    /// light, a mirrored quad light, a tiny distant sphere light, glass), tile by tile.
    #[test]
    fn test_agrees_on_hard_cases() {
        use crate::wavefront::integrator::tests::{hard_scene, mean_image, tiles};
        let mut s = hard_scene();
        s.render.supersamples = 256;
        let flat = FlatScene::build(&s).unwrap();
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let (w, h) = (s.image.width, s.image.height);
        let rc = Mutex::new(RenderContext::new(w, h, false, "test.png"));
        gpu.render(&s, &rc, |_| ());
        // The context's rows run bottom up (`RenderContext::snapshot`), as the passes'.
        let on_gpu: Vec<Color> = { let rc = rc.lock().unwrap(); (0..w * h).map(|i| rc.get_pixel(i % w, i / w)).collect() };
        let cpu = mean_image(&s, 256);
        let (a, b) = (tiles(&on_gpu, w, h, 2, 2), tiles(&cpu, w, h, 2, 2));
        for t in 0..4 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: GPU {} vs CPU {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// Pixel completion (`scheduler::plan`): with adaptive sampling the GPU stops sampling the
    /// pixels that are quiet (a flat background: every sample the same) and carries on with
    /// the noisy ones, each taking its own next samples; the picture is the one it gets from
    /// sampling every pixel in full; and a render stopped part-way and resumed from its film
    /// ends where an uninterrupted one does.
    /// The host counts what the scheduler measures: the paths alive at each round, read from
    /// the queues' headers. Every pixel starts a path (a pinhole stops none), none can have
    /// more vertices than the depth allows, and fewer are alive each round.
    #[test]
    fn test_counts_live_paths_by_round() {
        use crate::scheduler::Stage;
        let s = scene();
        let flat = FlatScene::build(&s).unwrap();
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let passes = 3;
        gpu.run(&s, 0..passes);
        let stats = gpu.take_stats();
        let pixels = (s.image.width * s.image.height) as u64;
        assert_eq!(stats.passes, passes as u64);
        assert_eq!(stats.live.len(), gpu.rounds());
        assert_eq!(stats.live[0], passes as u64 * pixels);
        assert!(stats.live.windows(2).all(|w| w[1] <= w[0]), "{:?}", stats.live);
        assert!(stats.live[1] > 0 && *stats.live.last().unwrap() < stats.live[0], "{:?}", stats.live);
        assert_eq!(stats.stage(Stage::Intersect).items, stats.live.iter().sum::<u64>());
        assert_eq!(stats.stage(Stage::Generate).items, stats.live[0]);
        assert_eq!(stats.stage(Stage::Accumulate).items, passes as u64 * pixels);
        assert!(stats.device_seconds > 0.);
        // Taken, so the next block starts from nothing.
        assert!(gpu.take_stats().is_empty());
    }

    #[test]
    fn test_samples_adaptively() {
        let mut s = scene();
        s.render.supersamples = 64;
        let flat = FlatScene::build(&s).unwrap();
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let (w, h) = (s.image.width, s.image.height);
        let render = |gpu: &mut GpuRenderer, s: &Scene, rc: RenderContext| {
            let rc = Mutex::new(rc);
            gpu.render(s, &rc, |_| ());
            rc.into_inner().unwrap()
        };
        let full = render(&mut gpu, &s, RenderContext::new(w, h, false, "test.png"));
        assert!((0..w * h).all(|i| full.samples_at(i % w, i / w) == 64));

        s.render.adaptive = Some(crate::scene::Adaptive { threshold: 0.05, min_samples: 8 });
        let adaptive = render(&mut gpu, &s, RenderContext::new(w, h, false, "test.png"));
        let counts: Vec<usize> = (0..w * h).map(|i| adaptive.samples_at(i % w, i / w)).collect();
        let (least, most) = (*counts.iter().min().unwrap(), *counts.iter().max().unwrap());
        assert!(least >= 8 && most <= 64, "samples from {} to {}", least, most);
        assert!(least < most, "every pixel took {} samples", least);
        let taken: usize = counts.iter().sum();
        assert!(taken < w * h * 64 * 9 / 10, "{} samples of {}", taken, w * h * 64);
        // The same picture: each pixel's mean within its noise of the fully sampled one's.
        let mean = |rc: &RenderContext| (0..w * h).fold(Vector3::zeros(), |a, i| a + rc.get_pixel(i % w, i / w).rgb) / (w * h) as f64;
        let (a, f) = (mean(&adaptive), mean(&full));
        for ch in 0..3 {
            assert!((a[ch] - f[ch]).abs() < 0.03 * f[ch], "channel {}: adaptive {} vs full {}", ch, a[ch], f[ch]);
        }

        // Stopped at 16 samples and resumed: the same film, pixel for pixel.
        s.render.supersamples = 16;
        let part = render(&mut gpu, &s, RenderContext::new(w, h, false, "test.png"));
        s.render.supersamples = 64;
        let resumed = render(&mut gpu, &s, part);
        for i in 0..w * h {
            let (x, y) = (i % w, i / w);
            assert_eq!(resumed.samples_at(x, y), adaptive.samples_at(x, y), "pixel {}'s samples", i);
            assert_eq!(resumed.get_pixel(x, y), adaptive.get_pixel(x, y), "pixel {}", i);
        }
    }

    /// The GPU renders what the CPU wavefront integrator does, tracing as many rays (so it
    /// samples the sky as the CPU does; skipped without a GPU adapter, such as Mesa's
    /// lavapipe).
    #[test]
    fn test_agrees_with_the_wavefront_integrator() {
        let s = scene();
        let flat = FlatScene::build(&s).unwrap();
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        gpu.render(&s, &rc, |_| ());
        let (gpu_image, gpu_rays) = { let rc = rc.lock().unwrap(); (rc.snapshot(), rc.rays_cast) };
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        crate::wavefront::integrator::render(&s, &flat, &rc, |_| ());
        let (cpu_image, cpu_rays) = { let rc = rc.lock().unwrap(); (rc.snapshot(), rc.rays_cast) };
        // Russian roulette ends paths by different random numbers, so the counts differ a little.
        assert!((gpu_rays as f64 - cpu_rays as f64).abs() < 0.02 * cpu_rays as f64, "rays: GPU {} vs CPU {}", gpu_rays, cpu_rays);
        let total = |img: &crate::output::LinearImage| img.pixels.iter().fold(Vector3::zeros(), |a, c| a + c.rgb);
        let (g, c) = (total(&gpu_image), total(&cpu_image));
        for ch in 0..3 {
            assert!((g[ch] - c[ch]).abs() < 0.03 * c[ch], "channel {}: GPU {} vs CPU {}", ch, g[ch], c[ch]);
        }
    }
}
