//! A path tracer on the GPU, with wgpu (Vulkan, Metal, DirectX 12): the flattened scene of the
//! wavefront integrator (`wavefront::flat`) uploaded as buffers, and a compute shader
//! (`pathtrace.wgsl`) that traces one path per pixel per pass, porting the wavefront
//! integrator's shading to f32 (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th
//! ed., chapter 15, whose wavefront renderer is pbrt-v4's GPU renderer).
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
use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;
use std::sync::Mutex;
use wgpu::util::DeviceExt;

/// The baked sky's resolution.
const ENV_WIDTH: usize = 1024;
const ENV_HEIGHT: usize = 512;
/// Passes between reading the accumulated image back.
const PASSES_PER_READ: usize = 8;
/// The aerial perspective table: azimuth from the sun, elevation and distance.
const AP_AZIMUTH: usize = 32;
const AP_ELEVATION: usize = 64;
const AP_DISTANCE: usize = 32;
/// Its distances, logarithmically from `AERIAL_FROM` to `AERIAL_TO` (beyond, the last).
const AERIAL_FROM: f64 = 100.;
const AERIAL_TO: f64 = 2e5;
const WORKGROUP: u32 = 64;
const MAX_GROUPS: u32 = 65535;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Params {
    dims: [u32; 4],
    counts: [u32; 4],
    offs: [u32; 4],
    offs2: [u32; 4],
    misc: [f32; 4],
    sun_dir: [f32; 4],
    sun_irr: [f32; 4],
    seed: [u32; 4],
    /// The sun's azimuth frame for the aerial perspective table (x along the sun, z across).
    ap_x: [f32; 4],
    ap_z: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuPrim {
    v0: [f32; 4],
    v1: [f32; 4],
    v2: [f32; 4],
    n0: [f32; 4],
    n1: [f32; 4],
    n2: [f32; 4],
    meta: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuNode {
    bmin: [f32; 3],
    offset: u32,
    bmax: [f32; 3],
    count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuMaterial {
    kind: u32,
    a: u32,
    b: u32,
    pad: u32,
    c0: [f32; 4],
    c1: [f32; 4],
    p: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuLight {
    pos: [f32; 4],
    intensity: [f32; 4],
    axis: [f32; 4],
    extra: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuRay {
    o: [f32; 4],
    d: [f32; 4],
}

fn v4(v: &Vector3<f64>, w: f32) -> [f32; 4] {
    [v.x as f32, v.y as f32, v.z as f32, w]
}

fn c4(c: &Color) -> [f32; 4] {
    [c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32, 0.]
}

/// Flatten `m` into `out` (checkerboards' squares after it), returning its index.
fn push_material(m: &FlatMaterial, out: &mut Vec<GpuMaterial>) -> u32 {
    let index = out.len() as u32;
    let blank = GpuMaterial { kind: 0, a: 0, b: 0, pad: 0, c0: [0.; 4], c1: [0.; 4], p: [0.; 4] };
    out.push(blank);
    let g = match m {
        FlatMaterial::Diffuse { albedo } => GpuMaterial { kind: 0, c0: c4(albedo), ..blank },
        FlatMaterial::Plastic { albedo, refractive_index, roughness } =>
            GpuMaterial { kind: 1, c0: c4(albedo), p: [*refractive_index as f32, *roughness as f32, 0., 0.], ..blank },
        FlatMaterial::Dielectric { refractive_index, tint } =>
            GpuMaterial { kind: 2, c0: c4(tint), p: [*refractive_index as f32, 0., 0., 0.], ..blank },
        FlatMaterial::Conductor { optics, roughness } => {
            let (eta, k) = optics.current();
            GpuMaterial { kind: 3, c0: c4(&eta), c1: c4(&k), p: [0., *roughness as f32, 0., 0.], ..blank }
        },
        FlatMaterial::Emitter { radiance, two_sided } => GpuMaterial { kind: 4, a: *two_sided as u32, c0: c4(radiance), ..blank },
        FlatMaterial::Specular { albedo, roughness } => GpuMaterial { kind: 5, c0: c4(albedo), p: [0., *roughness as f32, 0., 0.], ..blank },
        FlatMaterial::Checker { m1, m2, xsize, zsize } => {
            let a = push_material(m1, out);
            let b = push_material(m2, out);
            GpuMaterial { kind: 6, a, b, p: [0., 0., *xsize as f32, *zsize as f32], ..blank }
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

fn entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: buffer.as_entire_binding() }
}

impl GpuRenderer {
    /// Upload the flattened scene, or say why the GPU can't render it.
    pub fn new(s: &Scene, flat: &FlatScene) -> Result<GpuRenderer, String> {
        let mut lights = Vec::new();
        for l in &s.lights {
            let i = l.emission.srgb() * l.intensity;
            let (kind, axis, start, end) = match &l.profile {
                Profile::Isotropic => (0., Vector3::z(), 1., 1.),
                Profile::Spot { axis, cos_falloff_start, cos_total } => (1., *axis, *cos_falloff_start, *cos_total),
                _ => return Err("goniometric and projection lights".into()),
            };
            lights.push(GpuLight { pos: v4(&l.position, kind), intensity: c4(&i), axis: v4(&axis, start as f32), extra: [end as f32, 0., 0., 0.] });
        }
        if lights.is_empty() {
            lights.push(GpuLight::zeroed());
        }

        // Primitives: the bounded ones in the BVH's order, then the planes.
        let mut materials = Vec::new();
        let material_index: Vec<u32> = flat.materials.iter().map(|m| push_material(m, &mut materials)).collect();
        let to_gpu = |p: &Prim, m: u32, emitter: u32| -> GpuPrim {
            let mut g = GpuPrim::zeroed();
            g.meta = [0, material_index[m as usize], emitter, 0];
            match p {
                Prim::Triangle { v, n } => {
                    g.v0 = v4(&v[0], 0.); g.v1 = v4(&v[1], 0.); g.v2 = v4(&v[2], 0.);
                    if let Some(n) = n {
                        g.n0 = v4(&n[0], 0.); g.n1 = v4(&n[1], 0.); g.n2 = v4(&n[2], 0.);
                        g.meta[3] = 1;
                    }
                },
                Prim::Sphere { center, radius } => { g.v0 = v4(center, *radius as f32); g.meta[0] = 1; },
                Prim::Plane { y } => { g.v0 = [*y as f32, 0., 0., 0.]; g.meta[0] = 2; },
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
        let mut nodes: Vec<GpuNode> = flat.bvh.export().iter().map(|(b, offset, count)| GpuNode {
            bmin: [b.min.x as f32, b.min.y as f32, b.min.z as f32], offset: *offset,
            bmax: [b.max.x as f32, b.max.y as f32, b.max.z as f32], count: *count,
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
        let texels: Vec<Color> = (0..ENV_WIDTH * ENV_HEIGHT).into_par_iter().map(|i| {
            let d = env_direction(i % ENV_WIDTH, i / ENV_WIDTH);
            crate::wavefront::integrator::sky_radiance(&Ray { ro: Vector3::zeros(), rd: d }, s, flat)
        }).collect();
        for t in &texels {
            tables.extend([t.rgb.x as f32, t.rgb.y as f32, t.rgb.z as f32]);
        }
        // Rows by brightness times sin θ (a row's share of the sphere), then within each row.
        let weight = |x: usize, y: usize| {
            let theta = (y as f64 + 0.5) / ENV_HEIGHT as f64 * std::f64::consts::PI;
            crate::color::luminance(&texels[y * ENV_WIDTH + x]).max(0.) * theta.sin() + 1e-9
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
        if s.atmosphere.is_some() { flags |= 8; }
        if s.render.sample_lights { flags |= 1; }
        if s.environment.is_some() { flags |= 2; }
        let (sun_dir, sun_irr) = match &s.sun {
            Some(sun) => {
                flags |= 4;
                let e = sun.irradiance_at(&Vector3::zeros());
                (v4(&sun.direction, sun.angular_radius.cos() as f32), [e.rgb.x as f32, e.rgb.y as f32, e.rgb.z as f32, sun.angular_radius as f32])
            },
            None => ([0., 1., 0., 1.], [0.; 4]),
        };
        let (rr, rr_min) = match s.render.path_termination {
            PathTermination::RussianRoulette { min_depth } => (1, min_depth as f32),
            _ => (0, 0.),
        };
        let params = Params {
            dims: [s.image.width as u32, s.image.height as u32, 0, s.render.max_depth as u32],
            counts: [flat.prims.len() as u32, flat.planes.len() as u32, flat.emitters.len() as u32, s.lights.len() as u32],
            offs: [emitter_off, cdf_off, env_off, marginal_off],
            offs2: [conditional_off, ENV_WIDTH as u32, ENV_HEIGHT as u32, flags],
            misc: [s.render.regularise as f32, s.black_threshold as f32, s.render.shadow_bias as f32, rr_min],
            sun_dir,
            sun_irr,
            ap_x: v4(&ap_frame.0, 0.),
            ap_z: v4(&ap_frame.1, 0.),
            seed: [s.render.seed as u32, rr, aerial_off, 0],
        };

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })).map_err(|e| format!("no GPU adapter ({})", e))?;
        let adapter_name = format!("{} ({:?})", adapter.get_info().name, adapter.get_info().backend);
        let mut limits = wgpu::Limits::default();
        let available = adapter.limits();
        limits.max_storage_buffer_binding_size = available.max_storage_buffer_binding_size;
        limits.max_buffer_size = available.max_buffer_size;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("rays"),
            required_limits: limits,
            ..Default::default()
        })).map_err(|e| format!("can't open the GPU ({})", e))?;

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("pathtrace"),
            source: wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into()),
        });
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
        let pixels = (s.image.width * s.image.height) as u64;
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
    fn camera_rays(&self, s: &Scene, k: usize) -> Vec<GpuRay> {
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

    /// Render passes `first..last`, accumulating on the GPU; returns the sums per pixel.
    fn run(&mut self, s: &Scene, passes: std::ops::Range<usize>) -> Vec<[f32; 4]> {
        let pixels = (s.image.width * s.image.height) as u32;
        let groups = pixels.div_ceil(WORKGROUP);
        let (gx, gy) = (groups.min(MAX_GROUPS), groups.div_ceil(MAX_GROUPS));
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.clear_buffer(&self.accum_buffer, 0, None);
        self.queue.submit(Some(encoder.finish()));
        for k in passes {
            self.params.dims[2] = k as u32;
            self.queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&self.params));
            self.queue.write_buffer(&self.rays_buffer, 0, bytemuck::cast_slice(&self.camera_rays(s, k)));
            let mut encoder = self.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.dispatch_workgroups(gx, gy, 1);
            }
            self.queue.submit(Some(encoder.finish()));
        }
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.accum_buffer, 0, &self.readback, 0, None);
        self.queue.submit(Some(encoder.finish()));
        self.readback.map_async(wgpu::MapMode::Read, .., |r| r.expect("couldn't read the GPU's image back"));
        self.device.poll(wgpu::PollType::wait_indefinitely()).expect("GPU device lost");
        let out = bytemuck::cast_slice::<u8, [f32; 4]>(&self.readback.get_mapped_range(..).unwrap()).to_vec();
        self.readback.unmap();
        out
    }

    /// Render up to the scene's samples per pixel from where `rc` has got to, reading the
    /// image back every `PASSES_PER_READ` passes; `after` sees the context each time.
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
                    let v = sums[y * w + x];
                    rc.set_pixel(x, y, Color::new(v[0] as f64, v[1] as f64, v[2] as f64), n);
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

    /// The GPU renders what the CPU wavefront integrator does (skipped without a GPU adapter,
    /// such as Mesa's lavapipe).
    #[test]
    fn test_agrees_with_the_wavefront_integrator() {
        let s = SceneFile::from_string(r#"{
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
        }"#.to_string());
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
        let gpu_image = rc.lock().unwrap().snapshot();
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        crate::wavefront::integrator::render(&s, &flat, &rc, |_| ());
        let cpu_image = rc.lock().unwrap().snapshot();
        let total = |img: &crate::output::LinearImage| img.pixels.iter().fold(Vector3::zeros(), |a, c| a + c.rgb);
        let (g, c) = (total(&gpu_image), total(&cpu_image));
        for ch in 0..3 {
            assert!((g[ch] - c[ch]).abs() < 0.03 * c[ch], "channel {}: GPU {} vs CPU {}", ch, g[ch], c[ch]);
        }
    }
}
