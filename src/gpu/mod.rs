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
//! realistic lenses included, works unchanged), and the sky, baked into an environment map
//! (equirectangular, `ENV_WIDTH` × `ENV_HEIGHT`) that the shader looks up (bilinearly) for escaping rays and
//! samples in proportion to its brightness. Each pass's radiance accumulates on the GPU, and is
//! read back into the render's context every `PASSES_PER_READ` passes.
//!
//! Differences from the CPU integrators: f32 arithmetic (ray offsets scaled accordingly); the
//! sky at the map's resolution; area lights chosen by power (not the light BVH); independent
//! random numbers (a hash per pixel, pass and dimension), not Sobol'; the sun's irradiance
//! fixed at its value at the origin (it varies with altitude under a sky); aerial perspective
//! from a table baked along rays from the camera's position (`AP_*`; the CPU integrates it
//! from each ray's own origin); point lights isotropic or spot.

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
// The aerial perspective table's size and distances, and the flags and kinds, are the shader's.
const AP_AZIMUTH: usize = pathtrace::AP_AZIMUTH as usize;
const AP_ELEVATION: usize = pathtrace::AP_ELEVATION as usize;
const AP_DISTANCE: usize = pathtrace::AP_DISTANCE as usize;
const AERIAL_FROM: f64 = pathtrace::AERIAL_FROM as f64;
const AERIAL_TO: f64 = pathtrace::AERIAL_TO as f64;
const WORKGROUP: u32 = 64;
const MAX_GROUPS: u32 = 65535;

// The buffers' layouts are the shader crate's own (`shaders/pathtrace`), so the two can't drift.
use pathtrace::{CameraRay as GpuRay, Light as GpuLight, Material as GpuMaterial, Node as GpuNode, Params, Prim as GpuPrim};
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

/// The sum of the squares of `n` samples summing to `c`, as far as the GPU tells: it keeps
/// only sums, so the least they can be (were the samples equal). Only adaptive sampling reads
/// them, and the GPU doesn't sample adaptively (`main` renders such scenes with the path tracer).
pub fn squares_at_least(c: &Color, n: usize) -> Color {
    Color { rgb: c.rgb.component_mul(&c.rgb) / n.max(1) as f64 }
}

/// A scene uploaded to the GPU, with the pipeline that renders it.
pub struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    params: Params,
    params_buffer: wgpu::Buffer,
    rays_buffer: wgpu::Buffer,
    accum_buffer: wgpu::Buffer,
    readback: wgpu::Buffer,
    pub adapter_name: String,
}

/// The depth of an exported BVH (`Bvh::export`): one for a lone leaf.
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

fn entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: buffer.as_entire_binding() }
}

/// The shader's inputs, built on the CPU (`Buffers::build`).
pub struct Buffers {
    pub params: Params,
    pub prims: Vec<GpuPrim>,
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
        // The shader's traversal stack holds at most one node per level, and one more.
        let exported = flat.bvh.export();
        let depth = bvh_depth(&exported);
        if depth + 1 > pathtrace::STACK {
            return Err(format!("a BVH {} levels deep (the shader's stack holds {})", depth, pathtrace::STACK));
        }
        let mut nodes: Vec<GpuNode> = exported.iter().map(|(b, offset, count)| GpuNode {
            bmin: pathtrace::glam::Vec3::new(b.min.x as f32, b.min.y as f32, b.min.z as f32), offset: *offset,
            bmax: pathtrace::glam::Vec3::new(b.max.x as f32, b.max.y as f32, b.max.z as f32), count: *count,
        }).collect();
        if nodes.is_empty() {
            nodes.push(GpuNode::zeroed());
        }

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

        // Aerial perspective, from the camera's position: transmittance and inscattered light
        // along each direction (by azimuth from the sun and elevation) to each distance.
        let aerial_off = tables.len() as u32;
        if let Some(sky) = &s.atmosphere {
            let origin = s.camera.get_ray_weighted(0.5, 0.5, 0., 0.).map_or(Vector3::zeros(), |(r, _)| r.ro);
            let sun = sky.sun_direction();
            // A frame with the sun's azimuth along x.
            let ax = Vector3::new(sun.x, 0., sun.z).try_normalize(1e-9).unwrap_or(Vector3::x());
            let az = Vector3::y().cross(&ax);
            let entries: Vec<[f32; 6]> = (0..AP_AZIMUTH * AP_ELEVATION * AP_DISTANCE).into_par_iter().map(|i| {
                let (a, e, k) = (i / (AP_ELEVATION * AP_DISTANCE), (i / AP_DISTANCE) % AP_ELEVATION, i % AP_DISTANCE);
                let phi = (a as f64 / (AP_AZIMUTH - 1) as f64) * std::f64::consts::PI;
                let cos_e = -1. + 2. * e as f64 / (AP_ELEVATION - 1) as f64;
                let sin_e = (1. - cos_e * cos_e).max(0.).sqrt();
                let d = ax * (sin_e * phi.cos()) + az * (sin_e * phi.sin()) + Vector3::y() * cos_e;
                let dist = AERIAL_FROM * (AERIAL_TO / AERIAL_FROM).powf(k as f64 / (AP_DISTANCE - 1) as f64);
                let (t, inscattered) = sky.segment(&origin, &d.normalize(), dist);
                [t.x as f32, t.y as f32, t.z as f32, inscattered.x as f32, inscattered.y as f32, inscattered.z as f32]
            }).collect();
            for e in entries {
                tables.extend(e);
            }
        }

        // The aerial perspective table's frame: the sky's sun's azimuth along x.
        let ap_frame = {
            let sun = s.atmosphere.as_ref().map_or(Vector3::y(), |sky| sky.sun_direction());
            let ax = Vector3::new(sun.x, 0., sun.z).try_normalize(1e-9).unwrap_or(Vector3::x());
            (ax, Vector3::y().cross(&ax))
        };
        let mut flags = 0u32;
        if s.atmosphere.is_some() { flags |= pathtrace::FLAG_AERIAL; }
        if s.render.sample_lights { flags |= pathtrace::FLAG_SAMPLE_LIGHTS; }
        if s.environment.is_some() { flags |= pathtrace::FLAG_SAMPLE_ENV; }
        let (sun_dir, sun_irr) = match &s.sun {
            Some(sun) => {
                flags |= pathtrace::FLAG_SUN;
                // Reddened by the air between it and the camera, as the sky map is baked.
                let e = sun.irradiance_at(&crate::environment::viewpoint(s));
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
            ap_x: v4(&ap_frame.0, 0.),
            ap_z: v4(&ap_frame.1, 0.),
            seed: UVec4::new(s.render.seed as u32, rr, aerial_off, 0),
        };
        Ok(Buffers { params, prims, nodes, materials, lights, tables })
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
        let Buffers { params, prims, nodes, materials, lights, tables } = Buffers::build(s, flat)?;
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
                              ("camera rays", pixels * std::mem::size_of::<GpuRay>() as u64)] {
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
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("pathtrace"),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        let storage = |label: &str, bytes: &[u8]| device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label), contents: bytes, usage: wgpu::BufferUsages::STORAGE,
        });
        let prims_buffer = storage("prims", bytemuck::cast_slice(&prims));
        let nodes_buffer = storage("nodes", bytemuck::cast_slice(&nodes));
        let materials_buffer = storage("materials", bytemuck::cast_slice(&materials));
        let lights_buffer = storage("lights", bytemuck::cast_slice(&lights));
        let tables_buffer = storage("tables", bytemuck::cast_slice(&tables));
        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("params"), contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let rays_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rays"), size: pixels * std::mem::size_of::<GpuRay>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let accum_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("accum"), size: pixels * 16,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"), size: pixels * 16,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let layout = pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene"),
            layout: &layout,
            entries: &[entry(0, &params_buffer), entry(1, &prims_buffer), entry(2, &nodes_buffer), entry(3, &materials_buffer),
                       entry(4, &lights_buffer), entry(5, &tables_buffer), entry(6, &rays_buffer), entry(7, &accum_buffer)],
        });
        Ok(GpuRenderer { device, queue, pipeline, bind_group, params, params_buffer, rays_buffer, accum_buffer, readback, adapter_name })
    }

    /// Camera rays for sample `k` of every pixel, as the wavefront integrator's.
    pub fn camera_rays(s: &Scene, k: usize) -> Vec<GpuRay> {
        let (w, h) = (s.image.width, s.image.height);
        let pixels = (w * h) as u64;
        (0..w * h).into_par_iter().map(|index| {
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
        let pixels = (s.image.width * s.image.height) as u32;
        let groups = pixels.div_ceil(WORKGROUP);
        let (gx, gy) = (groups.min(MAX_GROUPS), groups.div_ceil(MAX_GROUPS));
        self.params.dims.z = k as u32;
        self.queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&self.params));
        self.queue.write_buffer(&self.rays_buffer, 0, bytemuck::cast_slice(&Self::camera_rays(s, k)));
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(gx, gy, 1);
        }
        self.queue.submit(Some(encoder.finish()));
    }

    /// The image accumulated so far: each pixel's sums (the radiance's, and in w the number of
    /// rays traced). Natively this waits for the GPU; in a browser it yields until the copy
    /// can be read.
    pub async fn read_back(&self) -> Result<Vec<[f32; 4]>, String> {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.accum_buffer, 0, &self.readback, 0, None);
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
        let out = bytemuck::cast_slice::<u8, [f32; 4]>(&self.readback.get_mapped_range(..).unwrap()).to_vec();
        self.readback.unmap();
        Ok(out)
    }

    /// Render passes `first..last`, accumulating on the GPU; returns the sums per pixel.
    #[cfg(not(target_arch = "wasm32"))]
    fn run(&mut self, s: &Scene, passes: std::ops::Range<usize>) -> Vec<[f32; 4]> {
        self.clear();
        for k in passes {
            self.dispatch(s, k);
        }
        pollster::block_on(self.read_back()).expect("GPU readback")
    }

    /// Render up to the scene's samples per pixel from where `rc` has got to, reading the
    /// image back every `PASSES_PER_READ` passes; `after` sees the context each time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(&mut self, s: &Scene, rc: &Mutex<RenderContext>, mut after: impl FnMut(&RenderContext)) {
        let first = rc.lock().unwrap().min_samples();
        let target = s.render.samples_per_pixel();
        let (w, h) = (s.image.width, s.image.height);
        let mut k = first;
        while k < target {
            let n = PASSES_PER_READ.min(target - k);
            let sums = self.run(s, k..k + n);
            let mut rc = rc.lock().unwrap();
            for y in 0..h {
                for x in 0..w {
                    // A pixel a resumed state has already taken past k (another integrator's
                    // render) skips passes until it isn't: they would repeat its samples.
                    // (Some sample indices go unused for it; none is used twice.)
                    if rc.samples_at(x, y) > k {
                        continue;
                    }
                    let v = sums[y * w + x];
                    let c = Color::new(v[0] as f64, v[1] as f64, v[2] as f64);
                    rc.set_pixel(x, y, c, squares_at_least(&c, n), n);
                    rc.rays_cast += v[3] as u64;
                }
            }
            after(&rc);
            k += n;
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
        let s = scene();
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
            assert!((on_gpu[ch] - cpu[ch]).abs() < 0.01 * cpu[ch], "channel {}: GPU {} vs CPU {}", ch, on_gpu[ch], cpu[ch]);
        }
        // And traces as many rays (w).
        let gpu_rays: f64 = sums.iter().map(|v| v[3] as f64).sum();
        assert!((gpu_rays - cpu_rays as f64).abs() < 0.01 * cpu_rays as f64, "rays: GPU {} vs CPU {}", gpu_rays, cpu_rays);
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
