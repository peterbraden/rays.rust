//! A path tracer on the GPU, with wgpu (Vulkan, Metal, DirectX 12): the flattened scene of the
//! wavefront integrator (`wavefront::flat`) uploaded as buffers, and a compute shader that
//! traces one path per pixel per pass, porting the wavefront integrator's shading to f32
//! (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., chapter 15, whose
//! wavefront renderer is pbrt-v4's GPU renderer).
//!
//! The shader is Rust: the crate `shaders/pathtrace`, compiled to SPIR-V by rust-gpu
//! (`scripts/build-shaders.sh`, run by `build.rs`, on rust-gpu's pinned nightly; the renderer
//! stays on stable and includes the `pathtrace.spv` it writes), which wgpu translates for the device (naga), for
//! Metal too. The same crate is also built for the CPU: its structs are these buffers'
//! layouts (so the two sides can't drift), and the tests run its functions, and its whole
//! `radiance`, on the CPU against the renderer's own and the GPU's.
//!
//! What stays on the CPU: camera rays (generated each pass and uploaded, so every camera,
//! realistic lenses included, works unchanged), and a map of the sky (equirectangular), which
//! the shader samples directions from in proportion to its brightness and, for a background
//! other than a skysphere's or space, looks up (`ENV_WIDTH` × `ENV_HEIGHT` then; the CPU's
//! sampling resolution, `SAMPLING_WIDTH`, where it only chooses directions). A skysphere's air is
//! the GPU's own: its parameters and tables (`skysphere::AirExport`, as `air_record`s) let the
//! shader evaluate it per ray as the CPU does, for escaping rays, aerial perspective from each
//! ray's origin, and sunlight at each point. Each pass's radiance accumulates on the GPU, and
//! is read back into the render's context every `PASSES_PER_READ` passes.
//!
//! Differences from the CPU integrators: f32 arithmetic (ray offsets scaled accordingly); area
//! lights chosen by power (not the light BVH); independent random numbers (a hash per pixel,
//! pass and dimension), not Sobol'; point lights isotropic or spot.

#[cfg(not(target_arch = "wasm32"))]
mod rt;

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

/// Passes between reading the accumulated image back.
const PASSES_PER_READ: usize = 8;

/// The most memory for the wavefront: the paths, their sky queries and their places in the
/// queues, for every lane (`pathtrace::slots`). It sets how many samples of each pixel a pass
/// may carry at once (`GpuRenderer::lanes`): 8 of a small image, one of a large.
const WAVEFRONT_BUDGET: u64 = 256 << 20;

/// Runs of rounds between reads whose rounds are counted (`GpuRenderer::rounds_buffer`): a
/// pass's, and each run after the host has shaded the paths that waited for it (`host`). Runs
/// beyond these go uncounted, not unrendered.
const COUNTED_PASSES: usize = 256;

/// What the host is keeping about the passes queued since the image was last read
/// (`scheduler::Stats`, and what completes it at the read).
#[derive(Default)]
struct Block {
    stats: crate::scheduler::Stats,
    /// Runs of rounds queued whose rounds are being counted, and which of them followed the
    /// host's shading: those count towards the stages' work, but not the paths alive by
    /// round, which is of a pass's first run.
    counted: usize,
    after_host: Vec<usize>,
    /// When the first of them was queued, and the host's own time in them since.
    started: Option<web_time::Instant>,
    host_seconds: f64,
}

// The buffers' layouts are the shader crate's own (`shaders/pathtrace`), so the two can't
// drift; the scene laid out in them is `kernels`'.
pub use crate::kernels::{host, Buffers};
#[allow(unused_imports)]
use crate::kernels::{air_record, bvh_depth, c4, cdf, env_direction, gpu_nodes, media_records, space_record, v4};

use pathtrace::{CameraRay as GpuRay, Light as GpuLight, Material as GpuMaterial, Node as GpuNode, Params, PathState, Prim as GpuPrim, SkyQuery};
use pathtrace::glam::{UVec4, Vec4};

/// The most kernels timed between reads of the image (`Timers`); the rest run untimed.
const TIMED_KERNELS: usize = 1024;

/// The device's own clock around each kernel, where it has one (wgpu's timestamp queries:
/// Vulkan and Metal natively): a compute pass a kernel, a timestamp at each end, resolved and
/// read with the image. With them the report gives the time of each stage on the device
/// (`scheduler::Stats`), which the host otherwise sees only whole; they are what says whether
/// a render is traversal, shading or the sky.
struct Timers {
    set: wgpu::QuerySet,
    resolved: wgpu::Buffer,
    readback: wgpu::Buffer,
    /// Nanoseconds a tick.
    period: f64,
    /// The stage of each kernel timed since the last read, in the order of their queries.
    timed: Mutex<Vec<crate::scheduler::Stage>>,
}

/// A block's camera rays (`GpuRenderer::make_rays`): how many lanes, how many rays live, and
/// the time making them took.
struct BlockRays {
    lanes: usize,
    live: u64,
    began: web_time::Instant,
    seconds: f64,
    /// Made while the block before ran on the GPU, so not part of this block's time.
    ahead: bool,
    rays: Vec<GpuRay>,
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
    /// The paths started from their camera rays, and the first round's queue
    /// (`pathtrace::start_paths`), in a bind group of their own.
    start_pipeline: wgpu::ComputePipeline,
    rays_buffer: wgpu::Buffer,
    add_group: wgpu::BindGroup,
    params: Params,
    /// The block's halves (`Half`): two where the scene has materials the host shades, so
    /// that the host shades one's paths while the device runs the other's rounds; else one,
    /// the whole block.
    halves: Vec<Half>,
    /// The kernels that gather the paths waiting for the host and scatter them back
    /// (`pathtrace::gather_host`, `scatter_host`), where it shades any.
    host_kernels: Option<(wgpu::ComputePipeline, wgpu::ComputePipeline)>,
    /// `empty_header` resets a side of a queue.
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
    /// The most samples of each pixel a pass may carry at once (`pathtrace::slots`): what the
    /// wavefront's buffers have room for, within `WAVEFRONT_BUDGET`.
    pub lanes: usize,
    block: Mutex<Block>,
    pub adapter_name: String,
    /// Whether the kernels trace by the device's ray queries (`rt`), and what they trace through.
    #[cfg(not(target_arch = "wasm32"))]
    acceleration: Option<rt::Acceleration>,
    timers: Option<Timers>,
    /// What setting up took, part by part, in seconds: the render's start, which a short
    /// render is mostly made of (plan.md section 12).
    pub setup: Vec<(&'static str, f64)>,
    /// The buffers it made, by name, and their sizes in bytes (not the acceleration structures,
    /// whose sizes wgpu doesn't give).
    pub buffers: Vec<(&'static str, u64)>,
    /// The render's origin, which the scene's buffers are laid out about (`kernels::origin`):
    /// the camera's, when the scene was uploaded. Rays from a camera since moved are still
    /// given in this frame.
    pub origin: Vector3<f64>,
}

/// A block's half (plan.md section 22): its rows (`pathtrace::FLAG_ROWS_EVEN`, `FLAG_ROWS_ODD`,
/// or 0 for every row), and what of the wavefront is its own: its parameters, its queues and
/// what reads them. The paths, the sky queries, the camera rays and the image are shared, the
/// halves' slots being apart, so the halves together cost little more memory than one.
struct Half {
    rows: u32,
    /// Its parameters for rounds reading the queue's side 0 and its side 1 (`Params::seed.z`).
    params_buffers: [wgpu::Buffer; 2],
    /// The two queues of paths and the host's (`pathtrace::queue`), and the buffer each
    /// round's dispatch is read from: the header of the queue it reads, copied there, as a
    /// buffer bound for writing can't also be a dispatch's arguments.
    queue_buffer: wgpu::Buffer,
    dispatch_buffer: wgpu::Buffer,
    /// The scene and the paths, for rounds reading side 0 and side 1.
    round_groups: [wgpu::BindGroup; 2],
    start_group: wgpu::BindGroup,
    /// Where the scene has materials the host shades (`host`): what serves its queue.
    host: Option<HostQueue>,
}

/// What serves a half's host queue (`host`; `GpuRenderer::request_host`, `serve_host`): the
/// buffer the waiting paths are gathered in side by side and scattered back from once shaded,
/// the dispatch over the host's queue (a copy of its header), and what is read: that header,
/// then the gathered paths.
/// The turns of a block whose host reads are guessed apart (`HostQueue::guess`); later ones
/// share the last's.
const GUESSED_TURNS: usize = 16;

struct HostQueue {
    /// The paths, the queues and `gathered`; the second with the parameters under which a
    /// kernel's appends go to side 0 of the queue, where the next rounds begin.
    groups: [wgpu::BindGroup; 2],
    gathered: wgpu::Buffer,
    dispatch: wgpu::Buffer,
    readback: wgpu::Buffer,
    /// How many paths to read with the header, before their number is known: somewhat more
    /// than waited at the same turn of the block before (`GUESSED_TURNS`), since the turns of
    /// a block differ a hundredfold (a pass's first vertices, then the few paths that bounce
    /// onto host materials again) but repeat from block to block. (More waiting than that
    /// takes a second read, queued behind the other half's rounds.)
    guess: [std::sync::atomic::AtomicUsize; GUESSED_TURNS],
}

fn entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: buffer.as_entire_binding() }
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

    /// Whether rays are traced by the hardware's ray queries (`rt`), rather than the BVH.
    pub fn ray_queries(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        return self.acceleration.is_some();
        #[cfg(target_arch = "wasm32")]
        false
    }

    /// Whether there is a GPU worth choosing unasked, or why not: none at all, or a software
    /// rasteriser (lavapipe, SwiftShader), for which the kernels are quicker compiled for the
    /// CPU itself.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn hardware() -> Result<(), String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })).map_err(|e| format!("no GPU adapter ({})", e))?;
        let info = adapter.get_info();
        if info.device_type == wgpu::DeviceType::Cpu {
            return Err(format!("{} is a software rasteriser", info.name));
        }
        Ok(())
    }

    /// Upload the flattened scene, or say why the GPU can't render it. Async, for a browser,
    /// whose WebGPU can't be waited on.
    pub async fn create(s: &Scene, flat: &FlatScene) -> Result<GpuRenderer, String> {
        // (`RAYS_GPU_HALVES=1` keeps a block whole, to compare.)
        let halves = if std::env::var("RAYS_GPU_HALVES").is_ok_and(|v| v == "1") { 1 } else { 2 };
        Self::create_with(s, flat, halves).await
    }

    /// `create`, with a block in `halves` (1 or 2) where the scene has materials the host
    /// shades (`Half`); otherwise it is whole.
    pub async fn create_with(s: &Scene, flat: &FlatScene, halves: usize) -> Result<GpuRenderer, String> {
        let mut setup = Vec::new();
        let mut clock = web_time::Instant::now();
        let mut lap = |setup: &mut Vec<(&'static str, f64)>, what: &'static str| {
            let now = web_time::Instant::now();
            setup.push((what, (now - clock).as_secs_f64()));
            // And for the render's report, with the memory then.
            crate::report::phase(format!("gpu set-up: {}", what), (now - clock).as_secs_f64());
            clock = now;
        };
        #[allow(unused_mut)]
        let Buffers { mut params, prims, prim_records, nodes, materials, lights, mut tables, hosts: any_host, unported, origin, .. } = Buffers::build(s, flat)?;
        // A kernel can't stop for the host in the midst of a shadow ray.
        if !unported.is_empty() {
            return Err(format!("shapes the kernels can't intersect yet: {}{}", unported.iter().take(4).cloned().collect::<Vec<_>>().join(", "), if unported.len() > 4 { " ..." } else { "" }));
        }
        // A browser's passes are queued without waiting on the device, which serving the
        // host's queue must (`host`).
        if cfg!(target_arch = "wasm32") && any_host {
            return Err("objects whose materials the renderer's own code shades, which the GPU's host can't yet do in a browser".into());
        }
        lap(&mut setup, "the scene's buffers");
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
        // The wavefront's width (`pathtrace::slots`): as many lanes as the budget and the
        // device's largest buffer have room for.
        let slot_bytes = std::mem::size_of::<PathState>() as u64 + 2 * std::mem::size_of::<SkyQuery>() as u64 + std::mem::size_of::<GpuRay>() as u64 + 12;
        let lanes = (WAVEFRONT_BUDGET / (pixels * slot_bytes).max(1))
            .min(largest / (2 * pixels * std::mem::size_of::<SkyQuery>() as u64).max(1))
            .clamp(1, PASSES_PER_READ as u64);
        let slots = lanes * pixels;
        limits.max_storage_buffer_binding_size = available.max_storage_buffer_binding_size;
        limits.max_buffer_size = available.max_buffer_size;
        let timed = adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
        // With ray queries, the hardware traces the bounded primitives (`rt`), found through a
        // table in `tables`.
        #[cfg(not(target_arch = "wasm32"))]
        let ray_query = rt::wanted(&adapter) && params.counts.x > 0;
        #[cfg(target_arch = "wasm32")]
        let ray_query = false;
        #[cfg(not(target_arch = "wasm32"))]
        if ray_query {
            params.origin.w = rt::map_table(&prim_records[..params.counts.x as usize], &mut tables);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = &prim_records;
        let mut required_features = if timed { wgpu::Features::TIMESTAMP_QUERY } else { wgpu::Features::empty() };
        if ray_query {
            required_features |= wgpu::Features::EXPERIMENTAL_RAY_QUERY;
            limits.max_acceleration_structures_per_shader_stage = available.max_acceleration_structures_per_shader_stage;
            limits.max_tlas_instance_count = available.max_tlas_instance_count;
            limits.max_blas_geometry_count = available.max_blas_geometry_count;
            limits.max_blas_primitive_count = available.max_blas_primitive_count;
        }
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("rays"),
            required_features,
            required_limits: limits,
            // (Ray queries are wgpu's experimental feature.)
            experimental_features: if ray_query { unsafe { wgpu::ExperimentalFeatures::enabled() } } else { Default::default() },
            ..Default::default()
        }).await.map_err(|e| format!("can't open the GPU ({})", e))?;
        let adapter_name = if ray_query { format!("{}, tracing by ray queries", adapter_name) } else { adapter_name };

        // The shader, written in Rust (`shaders/pathtrace`) and compiled to SPIR-V by rust-gpu
        // (`scripts/build-shaders.sh`); a browser takes the WGSL translation made with it.
        // Vulkan takes the SPIR-V; other backends the WGSL, whose loops the builder has
        // rewritten for naga's MSL and HLSL writers (`fold_continuing_blocks` in
        // shaders/builder: from SPIR-V, Metal's loops left one iteration early). RAYS_GPU_WGSL=1 runs the WGSL
        // on Vulkan too, to test the translation without a browser.
        // The kernels that trace by ray queries are WGSL on every backend: wgpu reads SPIR-V with
        // its own naga, which can't read ray queries (`shaders/builder`).
        #[cfg(not(target_arch = "wasm32"))]
        let source = if ray_query {
            wgpu::ShaderSource::Wgsl(include_str!("pathtrace-rq.wgsl").into())
        } else if info.backend != wgpu::Backend::Vulkan || std::env::var_os("RAYS_GPU_WGSL").is_some() {
            wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into())
        } else {
            wgpu::util::make_spirv(include_bytes!("pathtrace.spv"))
        };
        #[cfg(target_arch = "wasm32")]
        let source = wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into());
        let timers = timed.then(|| {
            let size = 16 * TIMED_KERNELS as u64;
            Timers {
                set: device.create_query_set(&wgpu::QuerySetDescriptor { label: Some("kernels"), ty: wgpu::QueryType::Timestamp, count: 2 * TIMED_KERNELS as u32 }),
                resolved: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("timestamps"), size, usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
                }),
                readback: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("timestamps read"), size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
                }),
                period: queue.get_timestamp_period() as f64,
                timed: Mutex::new(Vec::new()),
            }
        });
        lap(&mut setup, "the device");
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("pathtrace"), source });
        lap(&mut setup, "the shader");
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
        let mut scene_entries = vec![binding(0, wgpu::BufferBindingType::Uniform), binding(1, read), binding(2, read), binding(3, read),
                                     binding(4, read), binding(5, read), binding(6, write), binding(7, write), binding(8, write)];
        // And with ray queries, the acceleration structure (which isn't a storage buffer).
        #[cfg(not(target_arch = "wasm32"))]
        if ray_query {
            scene_entries.push(wgpu::BindGroupLayoutEntry {
                binding: pathtrace::ray_query::BINDING, visibility: wgpu::ShaderStages::COMPUTE, count: None,
                ty: wgpu::BindingType::AccelerationStructure { vertex_return: false },
            });
        }
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("scene"), entries: &scene_entries });
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
        let start_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("start"),
            entries: &[binding(0, wgpu::BufferBindingType::Uniform), binding(1, read), binding(2, write), binding(3, write)],
        });
        // Each kernel is compiled by the driver when its pipeline is made, the large ones
        // (shading, the sky) for a good part of a second: all at once, on threads, where there
        // are threads.
        // A scene with media takes the kernels that have their code (`pathtrace::Media`).
        let (intersect, bounce) = if params.media.y > 0 { ("intersect_paths_media", "bounce_paths_media") } else { ("intersect_paths", "bounce_paths") };
        let wanted = [(intersect, &layout), (bounce, &layout), ("answer_sky", &layout), ("add_paths", &add_layout), ("start_paths", &start_layout)];
        #[cfg(not(target_arch = "wasm32"))]
        let made: Vec<wgpu::ComputePipeline> = std::thread::scope(|scope| {
            let threads: Vec<_> = wanted.iter().map(|&(name, layout)| { let kernel = &kernel; scope.spawn(move || kernel(name, layout)) }).collect();
            threads.into_iter().map(|t| t.join().expect("a kernel's pipeline")).collect()
        });
        #[cfg(target_arch = "wasm32")]
        let made: Vec<wgpu::ComputePipeline> = wanted.iter().map(|&(name, layout)| kernel(name, layout)).collect();
        let Ok([intersect_pipeline, bounce_pipeline, sky_pipeline, add_pipeline, start_pipeline]) = <[wgpu::ComputePipeline; 5]>::try_from(made) else {
            unreachable!("five kernels were asked for")
        };
        lap(&mut setup, "the kernels");
        let storage = |label: &str, bytes: &[u8]| device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label), contents: bytes, usage: wgpu::BufferUsages::STORAGE,
        });
        let prims_buffer = storage("prims", bytemuck::cast_slice(&prims));
        // The kernels built for ray queries don't read the BVH: one node fills its binding.
        let nodes_buffer = storage("nodes", bytemuck::cast_slice(if ray_query { &nodes[..1] } else { &nodes }));
        drop((prims, nodes));
        let materials_buffer = storage("materials", bytemuck::cast_slice(&materials));
        let lights_buffer = storage("lights", bytemuck::cast_slice(&lights));
        let tables_buffer = storage("tables", bytemuck::cast_slice(&tables));
        let empty_header = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("empty queue"), contents: bytemuck::cast_slice(&pathtrace::EMPTY_QUEUE), usage: wgpu::BufferUsages::COPY_SRC,
        });
        let paths_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("paths"), size: slots * std::mem::size_of::<PathState>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
        });
        let rays_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera rays"), size: slots * std::mem::size_of::<GpuRay>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
        });
        let queries_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sky queries"), size: 2 * slots * std::mem::size_of::<SkyQuery>() as u64,
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
        #[cfg(not(target_arch = "wasm32"))]
        let acceleration = ray_query.then(|| rt::build(&device, &queue, &prim_records[..params.counts.x as usize]));
        if ray_query {
            lap(&mut setup, "the acceleration structures");
        }
        let host_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("host"),
            entries: &[binding(0, wgpu::BufferBindingType::Uniform), binding(1, write), binding(2, write), binding(3, write)],
        });
        let host_kernels = any_host.then(|| (kernel("gather_host", &host_layout), kernel("scatter_host", &host_layout)));
        // Two halves where the host shades: by row, even and odd.
        let halves: Vec<u32> = if any_host && halves > 1 {
            vec![pathtrace::FLAG_ROWS_EVEN, pathtrace::FLAG_ROWS_ODD]
        } else {
            vec![0]
        };
        // A half's most slots: its rows' (the even ones, if they are more).
        let half_slots = if halves.len() > 1 { s.image.height.div_ceil(2) as u64 * s.image.width as u64 * lanes as u64 } else { slots };
        let halves: Vec<Half> = halves.into_iter().map(|rows| {
            let mut own = params;
            own.offs2.w |= rows;
            let params_buffers = [0, 1].map(|_| device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("params"), contents: bytemuck::bytes_of(&own),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }));
            let queue_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("queues"), size: (pathtrace::QUEUE_HEADER as u64 + 3 * slots) * 4,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
            });
            let dispatch_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("dispatch"), size: 16,
                usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
            });
            let round_groups = [0, 1].map(|side| {
                #[allow(unused_mut)]
                let mut entries = vec![entry(0, &params_buffers[side]), entry(1, &prims_buffer), entry(2, &nodes_buffer), entry(3, &materials_buffer),
                                       entry(4, &lights_buffer), entry(5, &tables_buffer), entry(6, &paths_buffer), entry(7, &queue_buffer),
                                       entry(8, &queries_buffer)];
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(a) = &acceleration {
                    entries.push(wgpu::BindGroupEntry { binding: pathtrace::ray_query::BINDING, resource: wgpu::BindingResource::AccelerationStructure(&a.tlas) });
                }
                device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("scene"), layout: &layout, entries: &entries })
            });
            // Side 1's parameters: `start_paths` appends to side 0, which the first round reads.
            let start_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("start"),
                layout: &start_layout,
                entries: &[entry(0, &params_buffers[1]), entry(1, &rays_buffer), entry(2, &paths_buffer), entry(3, &queue_buffer)],
            });
            let host = any_host.then(|| {
            let size = half_slots * std::mem::size_of::<PathState>() as u64;
            let gathered = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gathered"), size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
            });
            let dispatch = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("host dispatch"), size: 16,
                usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
            });
            let readback = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("host readback"), size: 16 + size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
            });
            let groups = [0, 1].map(|side| device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("host"),
                layout: &host_layout,
                entries: &[entry(0, &params_buffers[side]), entry(1, &paths_buffer), entry(2, &queue_buffer), entry(3, &gathered)],
            }));
            HostQueue { groups, gathered, dispatch, readback, guess: std::array::from_fn(|_| std::sync::atomic::AtomicUsize::new(pixels as usize)) }
            });
            Half { rows, params_buffers, queue_buffer, dispatch_buffer, round_groups, start_group, host }
        }).collect();
        let add_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image"),
            layout: &add_layout,
            entries: &[entry(0, &halves[0].params_buffers[0]), entry(1, &paths_buffer), entry(2, &accum_buffer)],
        });
        let mut buffers: Vec<(&'static str, u64)> = vec![("prims", prims_buffer.size()), ("nodes", nodes_buffer.size()),
            ("materials", materials_buffer.size()), ("lights", lights_buffer.size()), ("tables", tables_buffer.size()),
            ("params", 2 * halves.len() as u64 * halves[0].params_buffers[0].size()), ("queues", halves.len() as u64 * halves[0].queue_buffer.size()), ("paths", paths_buffer.size()),
            ("camera rays", rays_buffer.size()), ("sky queries", queries_buffer.size()), ("accum", accum_buffer.size()),
            ("rounds", rounds_buffer.size()), ("readback", readback.size())];
        if let Some(h) = &halves[0].host {
            let n = halves.len() as u64;
            buffers.extend([("host gathered", n * h.gathered.size()), ("host readback", n * h.readback.size())]);
        }
        Ok(GpuRenderer { device, queue, buffers, intersect_pipeline, bounce_pipeline, sky_pipeline, add_pipeline, start_pipeline, rays_buffer,
            add_group, params, halves, host_kernels,
            empty_header, paths_buffer, accum_buffer, rounds_buffer, readback, lanes: lanes as usize, block: Default::default(), adapter_name,
            #[cfg(not(target_arch = "wasm32"))]
            acceleration,
            timers, setup, origin })
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
    pub fn dispatch_samples(&mut self, s: &Scene, samples: &[Option<u32>]) {
        let first: Vec<usize> = samples.iter().map(|k| k.unwrap_or(0) as usize).collect();
        let count: Vec<usize> = samples.iter().map(|k| k.is_some() as usize).collect();
        self.dispatch_block(s, &first, &count);
    }

    /// Queue a pass in which pixel i takes its samples `first[i]..first[i] + count[i]` (at
    /// most `lanes` of them) at once, each in a lane of the wavefront (`pathtrace::slots`):
    /// the rounds of one sample, for all of them. Returns at once.
    pub fn dispatch_block(&mut self, s: &Scene, first: &[usize], count: &[usize]) {
        let rays = Self::make_rays(s, first, count, &self.origin);
        self.dispatch_rays(rays);
    }

    /// `dispatch_block` from rays made already (`make_rays`).
    fn dispatch_rays(&mut self, rays: BlockRays) {
        assert!(self.host_kernels.is_none(), "a scene with materials the host shades takes its passes through block_with_host");
        let slot = self.begin_pass_with(rays);
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_start(&mut encoder, 0);
        self.encode_rounds(&mut encoder, 0, slot);
        self.encode_add(&mut encoder);
        self.queue.submit(Some(encoder.finish()));
    }

    /// As `dispatch_block`, for a scene with materials the host shades (`host`): after the
    /// rounds, the paths waiting for the host are read back, shaded and queued again, until
    /// none waits; then the pass is added to the image. This waits on the device.
    pub async fn block_with_host(&mut self, s: &Scene, flat: &FlatScene, first: &[usize], count: &[usize]) -> Result<(), String> {
        let rays = Self::make_rays(s, first, count, &self.origin);
        self.block_with_host_rays(s, flat, first, rays).await
    }

    /// `block_with_host` from rays made already (`make_rays`). With two halves (`Half`), each
    /// half's read of its host queue is queued after its own rounds and before the other's
    /// next ones, so that the host shades one half's paths while the device runs the other's.
    async fn block_with_host_rays(&mut self, s: &Scene, flat: &FlatScene, first: &[usize], rays: BlockRays) -> Result<(), String> {
        let slot = self.begin_pass_with(rays);
        let n = self.halves.len();
        let mut reads = Vec::with_capacity(n);
        for h in 0..n {
            let mut encoder = self.device.create_command_encoder(&Default::default());
            self.encode_start(&mut encoder, h);
            // (The other half's first rounds counted as the pass's too.)
            let slot = if h == 0 { slot } else { self.run_slot(false) };
            self.encode_rounds(&mut encoder, h, slot);
            self.queue.submit(Some(encoder.finish()));
            reads.push(Some(self.request_host(h, 0)));
        }
        while reads.iter().any(Option::is_some) {
            for h in 0..n {
                let Some(read) = reads[h].take() else { continue };
                let turn = read.1;
                let Some(rounds) = self.serve_host(s, flat, first, h, read).await? else { continue };
                let slot = self.run_slot(true);
                let mut encoder = self.device.create_command_encoder(&Default::default());
                if let Some(slot) = slot {
                    // How many paths these rounds start with: those the host sent on.
                    encoder.copy_buffer_to_buffer(&self.halves[h].queue_buffer, 12, &self.rounds_buffer, 4 * slot as u64, 4);
                }
                self.encode_rounds_up_to(&mut encoder, h, slot, rounds);
                self.queue.submit(Some(encoder.finish()));
                reads[h] = Some(self.request_host(h, turn + 1));
            }
        }
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_add(&mut encoder);
        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// A run of rounds' slot for the scheduler's counts, if any are left before the next read;
    /// `after` for a run that follows the host's turn, which counts towards the stages' work
    /// but not the paths alive by round.
    fn run_slot(&self, after: bool) -> Option<usize> {
        let rounds = self.rounds();
        let mut block = self.block.lock().unwrap();
        (block.counted < COUNTED_PASSES).then(|| {
            block.counted += 1;
            let run = block.counted - 1;
            if after {
                block.after_host.push(run);
            }
            run * rounds
        })
    }

    /// Half `h`'s paths waiting for the host gathered (`pathtrace::gather_host`), and their
    /// count and as many as are guessed to wait copied to be read: queued, not waited for.
    /// Returns the guess and the submission to wait for. `turn` is which of the block's turns
    /// this is for the half.
    fn request_host(&self, h: usize, turn: usize) -> (u64, usize, wgpu::SubmissionIndex) {
        let half = &self.halves[h];
        let host = half.host.as_ref().expect("a scene with host materials");
        let (gather, _) = self.host_kernels.as_ref().expect("a scene with host materials");
        let size = std::mem::size_of::<PathState>() as u64;
        let header = 16 * pathtrace::HOST_QUEUE as u64;
        let guess = (host.guess[turn.min(GUESSED_TURNS - 1)].load(std::sync::atomic::Ordering::Relaxed) as u64).min(host.gathered.size() / size);
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&half.queue_buffer, header, &host.dispatch, 0, 16);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(gather);
            pass.set_bind_group(0, &host.groups[0], &[]);
            pass.dispatch_workgroups_indirect(&host.dispatch, 0);
        }
        encoder.copy_buffer_to_buffer(&half.queue_buffer, header, &host.readback, 0, 16);
        encoder.copy_buffer_to_buffer(&host.gathered, 0, &host.readback, 16, guess * size);
        (guess, turn, self.queue.submit(Some(encoder.finish())))
    }

    /// The host's turn for half `h`, once its read (`request_host`) is done: the paths
    /// waiting shaded (`host::shade`), and scattered back to their slots, those that go on
    /// queued for the next rounds (`scatter_host`). None if none waited; else how many rounds
    /// those that go on can take at most: a path whose next vertex is its D-th has at most
    /// `max_depth` + 1 - D left, so the rounds after the host's turn needn't be a whole
    /// pass's, most of which, late in a pass, would be empty dispatches the host waits out.
    async fn serve_host(&self, s: &Scene, flat: &FlatScene, first: &[usize], h: usize, (guess, turn, read): (u64, usize, wgpu::SubmissionIndex)) -> Result<Option<u32>, String> {
        use std::sync::atomic::Ordering;
        let half = &self.halves[h];
        let host = half.host.as_ref().expect("a scene with host materials");
        let (_, scatter) = self.host_kernels.as_ref().expect("a scene with host materials");
        let image = s.image.width * s.image.height;
        let size = std::mem::size_of::<PathState>() as u64;
        let header = 16 * pathtrace::HOST_QUEUE as u64;
        self.map_after(&host.readback, 0..16 + guess * size, Some(read)).await?;
        let (count, mut paths) = {
            let data = host.readback.get_mapped_range(..16 + guess * size).unwrap();
            let count = bytemuck::pod_read_unaligned::<u32>(&data[12..16]) as u64;
            let paths: Vec<PathState> = bytemuck::pod_collect_to_vec(&data[16..16 + (count.min(guess) * size) as usize]);
            (count, paths)
        };
        host.readback.unmap();
        if count == 0 {
            return Ok(None);
        }
        if count > guess {
            // More waited than was read with the header: the rest.
            let rest = 16 + guess * size..16 + count * size;
            let mut encoder = self.device.create_command_encoder(&Default::default());
            encoder.copy_buffer_to_buffer(&host.gathered, guess * size, &host.readback, rest.start, rest.end - rest.start);
            self.queue.submit(Some(encoder.finish()));
            self.map(&host.readback, rest.clone()).await?;
            paths.extend(bytemuck::pod_collect_to_vec::<u8, PathState>(&host.readback.get_mapped_range(rest).unwrap()));
            host.readback.unmap();
        }
        host.guess[turn.min(GUESSED_TURNS - 1)].store((count + count / 4 + 64) as usize, Ordering::Relaxed);
        let began = web_time::Instant::now();
        let origin = self.origin;
        // (`RAYS_HOST_SHADOWS=1` traces the host's vertices' shadow rays on the host, to compare.)
        let defer = std::env::var_os("RAYS_HOST_SHADOWS").is_none();
        let shaded: Vec<(PathState, Option<usize>, f64)> = paths.par_iter().map(|p| {
            // Its slot (`gather_host`) is its pixel's path in a lane, so that sample of the
            // pixel's.
            let (pixel, lane) = (p.state.w as usize % image, p.state.w as usize / image);
            let (began, key) = (web_time::Instant::now(), host::object_key(flat, p.state.z));
            let next = host::shade(s, flat, &origin, p, pixel as u32, (first[pixel] + lane) as u32, None, defer);
            (next, key, began.elapsed().as_secs_f64())
        }).collect();
        // Each object's vertices, for the report.
        let mut objects = crate::scheduler::Stats::default();
        for (next, key, seconds) in shaded.iter().map(|(p, k, t)| (p, *k, *t)) {
            if let Some(key) = key {
                objects.object(key).vertices += 1;
                objects.object(key).shade_seconds += seconds;
                objects.object(key).deferred += host::deferred_rays(next);
            }
        }
        let shaded: Vec<PathState> = shaded.into_iter().map(|(p, _, _)| p).collect();
        let shallowest = shaded.iter().filter(|p| p.state.y & pathtrace::PATH_ACTIVE != 0).map(|p| p.beta.w as u32).min();
        let rounds = shallowest.map_or(0, |d| (self.params.dims.w + 1).saturating_sub(d).max(1));
        self.queue.write_buffer(&host.gathered, 0, bytemuck::cast_slice(&shaded));
        let seconds = began.elapsed().as_secs_f64();
        // The next rounds begin at side 0 of the queue, which the scattered paths that go on
        // are appended to; then the host's queue is empty again.
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.empty_header, 0, &half.queue_buffer, 0, 16);
        encoder.copy_buffer_to_buffer(&self.empty_header, 0, &half.queue_buffer, 16, 16);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(scatter);
            pass.set_bind_group(0, &host.groups[1], &[]);
            pass.dispatch_workgroups_indirect(&host.dispatch, 0);
        }
        encoder.copy_buffer_to_buffer(&half.queue_buffer, 0, &half.dispatch_buffer, 0, 16);
        encoder.copy_buffer_to_buffer(&self.empty_header, 0, &half.queue_buffer, header, 16);
        self.queue.submit(Some(encoder.finish()));
        let mut block = self.block.lock().unwrap();
        block.host_seconds += seconds;
        block.stats.record(crate::scheduler::Stage::Host, count, seconds);
        block.stats.merge_objects(&objects.objects);
        Ok(Some(rounds))
    }

    /// A pass's paths, from their camera rays, and its first queue, uploaded: pixel i's
    /// samples `first[i]..first[i] + count[i]`, one a lane. Returns the pass's slot for its
    /// rounds' counts (the scheduler's measurements), if there is one left before the next
    /// read.
    fn begin_pass_with(&mut self, rays: BlockRays) -> Option<usize> {
        let began = web_time::Instant::now();
        let lanes = rays.lanes;
        assert!(lanes <= self.lanes, "a pass of {} samples a pixel, with lanes for {}", lanes, self.lanes);
        let pixels = self.params.dims.x * self.params.dims.y;
        self.params.seed.w = lanes as u32;
        // Every queue starts empty: the kernel fills side 0, and the first round's dispatch is
        // copied from it.
        let empty: Vec<u32> = (0..3).flat_map(|_| pathtrace::EMPTY_QUEUE).collect();
        for half in &self.halves {
            let mut own = self.params;
            own.offs2.w |= half.rows;
            for side in 0..2 {
                own.seed.z = side as u32;
                self.queue.write_buffer(&half.params_buffers[side], 0, bytemuck::bytes_of(&own));
            }
            self.queue.write_buffer(&half.queue_buffer, 0, bytemuck::cast_slice(&empty));
        }
        self.queue.write_buffer(&self.rays_buffer, 0, bytemuck::cast_slice(&rays.rays));
        let live = rays.live;
        // The host's part ends here: the rays generated and uploaded. Encoding and submitting
        // the kernels is counted as the device's, since a driver may run them as they arrive.
        // Rays made while the block before ran are none of this block's time.
        let (began, seconds) = if rays.ahead {
            (began, began.elapsed().as_secs_f64())
        } else {
            (rays.began, rays.seconds + began.elapsed().as_secs_f64())
        };
        let rounds = self.rounds();
        // What the host knows of the pass already: the paths it generated (which are the first
        // round's), and that every pixel's path is added to the film. The later rounds are
        // counted at the read.
        use crate::scheduler::Stage;
        let slot = {
            let block = self.block.get_mut().unwrap();
            block.started.get_or_insert(began);
            block.host_seconds += seconds;
            block.stats.passes += lanes as u64;
            block.stats.record(Stage::Generate, live, seconds);
            block.stats.record(Stage::Accumulate, pixels as u64 * lanes as u64, 0.);
            (block.counted < COUNTED_PASSES).then(|| {
                block.counted += 1;
                block.stats.round(0, live);
                (block.counted - 1) * rounds
            })
        };
        if slot.is_some() {
            self.record_round(live);
        }
        slot
    }

    /// A block's camera rays: pixel i's samples `first[i]..first[i] + count[i]`, one a lane,
    /// every lane's at once, in parallel (`pathtrace::start_paths` starts the paths from them,
    /// so 32 bytes a path cross to the GPU, not 96). A pixel that takes no sample in a lane has
    /// a dead ray there, so a path that adds nothing. Only the CPU's work: `render` makes the
    /// next block's while the GPU runs this one.
    fn make_rays(s: &Scene, first: &[usize], count: &[usize], origin: &Vector3<f64>) -> BlockRays {
        let began = web_time::Instant::now();
        let image = s.image.width * s.image.height;
        let lanes = count.iter().copied().max().unwrap_or(0).max(1);
        let rays: Vec<GpuRay> = (0..image * lanes).into_par_iter().map(|j| {
            let (lane, i) = (j / image, j % image);
            if lane < count[i] { crate::kernels::camera_ray(s, i, first[i] + lane, origin) } else { GpuRay::zeroed() }
        }).collect();
        let live = rays.par_iter().filter(|r| r.d.w >= 0.5).count() as u64;
        BlockRays { lanes, live, began, seconds: began.elapsed().as_secs_f64(), ahead: false, rays }
    }

    /// A path has at most `max_depth` + 1 vertices: that many rounds, each over the paths the
    /// last left alive, starting from side 0 of the queue. With a `slot`, each round's count
    /// is kept for the scheduler's measurements.
    fn encode_rounds(&self, encoder: &mut wgpu::CommandEncoder, h: usize, slot: Option<usize>) {
        self.encode_rounds_up_to(encoder, h, slot, self.params.dims.w + 1);
    }

    /// `encode_rounds`, `rounds` of them (at most a pass's).
    fn encode_rounds_up_to(&self, encoder: &mut wgpu::CommandEncoder, h: usize, slot: Option<usize>, rounds: u32) {
        let half = &self.halves[h];
        for round in 0..rounds.min(self.params.dims.w + 1) {
            let side = (round % 2) as usize;
            // A compute pass a kernel, so that each can be timed (`Timers`).
            use crate::scheduler::Stage;
            for (stage, pipeline) in [(Stage::Intersect, &self.intersect_pipeline), (Stage::Shade, &self.bounce_pipeline), (Stage::Sky, &self.sky_pipeline)] {
                let mut pass = self.timed_pass(encoder, stage);
                pass.set_bind_group(0, &half.round_groups[side], &[]);
                pass.set_pipeline(pipeline);
                pass.dispatch_workgroups_indirect(&half.dispatch_buffer, 0);
            }
            // The next round reads the side this one filled, and fills the side it read.
            encoder.copy_buffer_to_buffer(&half.queue_buffer, 16 * (1 - side as u64), &half.dispatch_buffer, 0, 16);
            if let Some(slot) = slot.filter(|_| round < self.params.dims.w) {
                // How many paths the next round starts with (the header's count).
                encoder.copy_buffer_to_buffer(&half.queue_buffer, 16 * (1 - side as u64) + 12, &self.rounds_buffer, 4 * (slot + round as usize + 1) as u64, 4);
            }
            encoder.copy_buffer_to_buffer(&self.empty_header, 0, &half.queue_buffer, 16 * side as u64, 16);
        }
    }

    /// The pass's paths started from their camera rays, and side 0 filled with the live ones
    /// (`pathtrace::start_paths`); the first round's dispatch is the count they leave.
    fn encode_start(&self, encoder: &mut wgpu::CommandEncoder, h: usize) {
        let half = &self.halves[h];
        let slots = self.params.dims.x * self.params.dims.y * self.params.seed.w;
        let groups = slots.div_ceil(pathtrace::WORKGROUP);
        {
            let mut pass = self.timed_pass(encoder, crate::scheduler::Stage::Generate);
            pass.set_bind_group(0, &half.start_group, &[]);
            pass.set_pipeline(&self.start_pipeline);
            pass.dispatch_workgroups(groups.min(pathtrace::MAX_GROUPS), groups.div_ceil(pathtrace::MAX_GROUPS), 1);
        }
        encoder.copy_buffer_to_buffer(&half.queue_buffer, 0, &half.dispatch_buffer, 0, 16);
    }

    /// A compute pass for one kernel of `stage`, with the device's clock read at each end where
    /// it has one and there are queries left before the next read (`Timers`).
    fn timed_pass<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, stage: crate::scheduler::Stage) -> wgpu::ComputePass<'a> {
        let timestamp_writes = self.timers.as_ref().and_then(|t| {
            let mut timed = t.timed.lock().unwrap();
            (timed.len() < TIMED_KERNELS).then(|| {
                timed.push(stage);
                let at = 2 * (timed.len() as u32 - 1);
                wgpu::ComputePassTimestampWrites { query_set: &t.set, beginning_of_pass_write_index: Some(at), end_of_pass_write_index: Some(at + 1) }
            })
        });
        encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes })
    }

    /// Every pixel's path, added to the image: the end of a pass.
    fn encode_add(&self, encoder: &mut wgpu::CommandEncoder) {
        let pixels = self.params.dims.x * self.params.dims.y;
        let groups = pixels.div_ceil(pathtrace::WORKGROUP);
        let mut pass = self.timed_pass(encoder, crate::scheduler::Stage::Accumulate);
        pass.set_bind_group(0, &self.add_group, &[]);
        pass.set_pipeline(&self.add_pipeline);
        pass.dispatch_workgroups(groups.min(pathtrace::MAX_GROUPS), groups.div_ceil(pathtrace::MAX_GROUPS), 1);
    }

    /// Map `buffer` for reading. Natively this waits for the GPU; in a browser it yields until
    /// the buffer can be read.
    async fn map(&self, buffer: &wgpu::Buffer, range: std::ops::Range<u64>) -> Result<(), String> {
        self.map_after(buffer, range, None).await
    }

    /// `map`, natively waiting only for submission `after` (and those before it), if given,
    /// not for everything queued since: the other half's rounds run on meanwhile.
    async fn map_after(&self, buffer: &wgpu::Buffer, range: std::ops::Range<u64>, after: Option<wgpu::SubmissionIndex>) -> Result<(), String> {
        let mapped = Mapped::default();
        let slot = mapped.0.clone();
        buffer.map_async(wgpu::MapMode::Read, range, move |r| {
            let mut slot = slot.lock().unwrap();
            slot.0 = Some(r.map_err(|e| e.to_string()));
            if let Some(waker) = slot.1.take() {
                waker.wake();
            }
        });
        #[cfg(not(target_arch = "wasm32"))]
        self.device.poll(wgpu::PollType::Wait { submission_index: after, timeout: None }).map_err(|e| format!("GPU device lost ({})", e))?;
        mapped.await.map_err(|e| format!("couldn't read from the GPU ({})", e))
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
        let (counted, after_host) = {
            let mut block = self.block.lock().unwrap();
            (std::mem::take(&mut block.counted), std::mem::take(&mut block.after_host))
        };
        if counted > 0 {
            encoder.copy_buffer_to_buffer(&self.rounds_buffer, 0, &self.readback, image, (counted * self.rounds() * 4) as u64);
        }
        // The kernels' timestamps since the last read, with the image.
        let timed = self.timers.as_ref().map(|t| std::mem::take(&mut *t.timed.lock().unwrap())).unwrap_or_default();
        if let Some(t) = self.timers.as_ref().filter(|_| !timed.is_empty()) {
            encoder.resolve_query_set(&t.set, 0..2 * timed.len() as u32, &t.resolved, 0);
            encoder.copy_buffer_to_buffer(&t.resolved, 0, &t.readback, 0, 16 * timed.len() as u64);
        }
        self.queue.submit(Some(encoder.finish()));
        if let Some(t) = self.timers.as_ref().filter(|_| !timed.is_empty()) {
            let size = 16 * timed.len() as u64;
            self.map(&t.readback, 0..size).await?;
            let ticks: Vec<u64> = bytemuck::pod_collect_to_vec(&t.readback.get_mapped_range(..size).unwrap());
            t.readback.unmap();
            let mut block = self.block.lock().unwrap();
            for (stage, pair) in timed.iter().zip(ticks.chunks(2)) {
                block.stats.add_seconds(*stage, pair[1].saturating_sub(pair[0]) as f64 * t.period * 1e-9);
            }
        }
        self.map(&self.readback, 0..self.readback.size()).await?;
        let both = bytemuck::cast_slice::<u8, [f32; 4]>(&self.readback.get_mapped_range(..image).unwrap()).to_vec();
        let rounds = bytemuck::cast_slice::<u8, u32>(&self.readback.get_mapped_range(image..image + (counted * self.rounds() * 4) as u64).unwrap()).to_vec();
        self.readback.unmap();
        for (run, pass) in rounds.chunks(self.rounds()).enumerate() {
            // (A pass's first round was counted when it was queued; a run after the host's
            // turn has its first here.)
            let after = after_host.contains(&run);
            for (round, &paths) in pass.iter().enumerate().skip(if after { 0 } else { 1 }) {
                if !after {
                    self.block.lock().unwrap().stats.round(round, paths as u64);
                }
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

    /// Render up to the scene's samples per pixel from where `rc` has got to, a pass of up to
    /// `lanes` samples a pixel at a time, the image read back after each; `after` sees the
    /// context each time.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(&mut self, s: &Scene, flat: &FlatScene, rc: &Mutex<RenderContext>, mut after: impl FnMut(&RenderContext)) {
        let (w, h) = (s.image.width, s.image.height);
        // The next block's rays, made while the GPU runs this one: for the block that carries
        // every pixel on from where this one leaves it, which is the next one unless the
        // scheduler decides otherwise (adaptive sampling's pixels stopping, or the end); kept
        // only if it doesn't, so a guess costs time, never the picture.
        let mut ahead: Option<(Vec<usize>, Vec<usize>, BlockRays)> = None;
        let target = s.render.samples_per_pixel();
        loop {
            // The scheduler's part (`scheduler::plan`): which pixels take samples in the next
            // few passes, and how many each. Decided a region at a time, as the recursive
            // tracer's passes are, from the film alone, so a render resumed from a saved film
            // goes on as an uninterrupted one would.
            let (mut first, mut count) = (vec![0usize; w * h], vec![0usize; w * h]);
            {
                let rc = rc.lock().unwrap();
                for b in rc.regions(s) {
                    let (f, c) = rc.plan(s, &b, crate::scheduler::lanes(self.lanes));
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
            let rays = match ahead.take() {
                Some((f, c, rays)) if f == first && c == count => BlockRays { ahead: true, ..rays },
                _ => Self::make_rays(s, &first, &count, &self.origin),
            };
            self.clear();
            if self.host_kernels.is_some() {
                pollster::block_on(self.block_with_host_rays(s, flat, &first, rays)).expect("GPU readback");
            } else {
                self.dispatch_rays(rays);
                // The GPU is busy: guess the next block, and make its rays meanwhile.
                let lanes = crate::scheduler::lanes(self.lanes);
                let next_first: Vec<usize> = first.iter().zip(&count).map(|(f, c)| f + c).collect();
                let next_count: Vec<usize> = next_first.iter().zip(&count)
                    .map(|(f, c)| if *c == 0 { 0 } else { target.saturating_sub(*f).min(lanes) }).collect();
                if next_count.iter().any(|c| *c > 0) {
                    let rays = Self::make_rays(s, &next_first, &next_count, &self.origin);
                    ahead = Some((next_first, next_count, rays));
                }
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
            let b = Buffers::build_at(&s, &flat, &Vector3::zeros()).unwrap(); // (Probed at the world's own positions.)
            let shader = b.scene(None);
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
            let b = Buffers::build_at(&s, &flat, &Vector3::zeros()).unwrap(); // (Probed at the world's own positions.)
            let shader = b.scene(None);
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
        let shader = b.scene(None);
        let passes = 16;
        let mut on_shader = Vector3::zeros();
        for k in 0..passes {
            for (index, ray) in crate::kernels::camera_rays(&s, k, &b.origin).iter().enumerate() {
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
            let shader = b.scene(None);
            let passes = 32;
            let mut cpu = Vector3::zeros();
            let mut cpu_rays = 0u64;
            for k in 0..passes {
                for (index, ray) in crate::kernels::camera_rays(&s, k, &b.origin).iter().enumerate() {
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
            let b = Buffers::build_at(&s, &flat, &Vector3::zeros()).unwrap(); // (Probed at the world's own positions.)
            let shader = b.scene(None);
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
                    let single = pathtrace::Scene { params, prims: prim, nodes, materials: &b.materials, lights: &b.lights, tables: &b.tables, host: None };
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
        for name in ["demo/demo.json", "demo/benchmarks/teapot-sky.json", "demo/benchmarks/glossy-sun.json", "demo/benchmarks/spheres.json", "demo/benchmarks/lamps.json", "demo/scenes/night-orbit.json"] {
            let s = SceneFile::from_file(name, &crate::scenefile::Overrides { width: Some(160), height: Some(90), seed: None });
            let flat = match FlatScene::build(&s) {
                Ok(f) => f,
                Err(e) => { println!("{:<16} not flattened: {}", name, e); continue; },
            };
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = b.scene(None);
            pathtrace::stats::take();
            let mut total_rays = 0u64;
            for k in 0..2 {
                for (index, ray) in crate::kernels::camera_rays(&s, k, &b.origin).iter().enumerate() {
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
            for (index, ray) in crate::kernels::camera_rays(&s, 0, &b.origin).iter().enumerate() {
                let mut p = PathState::start(index as u32, 0, b.params.seed.x, ray);
                for _ in 0..=b.params.dims.w {
                    slots += 1;
                    if p.state.y & pathtrace::PATH_ACTIVE != 0 {
                        live += 1;
                        p = pathtrace::bounce(&shader, &p, index as u32).0;
                    }
                }
            }
            pathtrace::stats::take();
            let live_share = live as f64 / slots as f64;
            // How the work divides: the share of the megakernel's time (on this CPU, one
            // thread) that is traversal, closest hits and shadow rays together.
            pathtrace::stats::time_traversal(true);
            let start = std::time::Instant::now();
            for (index, ray) in crate::kernels::camera_rays(&s, 0, &b.origin).iter().enumerate() {
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
        gpu.render(&s, &flat, &rc, |_| ());
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

    /// The clouds' density is one definition (`shared::clouds`), which the renderer computes at
    /// f64 and the shader at f32. At points through a layer, under every strength of weather:
    /// the shader's functions run on the CPU agree with the renderer's to f32's precision, and
    /// the kernel that calls them on the device (`probe_clouds`) agrees with those.
    #[test]
    fn test_clouds_are_the_renderers_on_the_device() {
        use crate::procedural::nubis_clouds::NubisCloudLayer;
        let layer = NubisCloudLayer::new(1500., 4000., crate::na::Vector2::new(0., 0.), 20000., 7);
        let record = layer.export();
        let mut rng = crate::geometry::Pcg32::new(3, 0);
        let mut next = move || rng.next_f64();
        let n = 4096;
        let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
            (next() * 16000. - 8000.) as f32, (1400. + next() * 2700.) as f32, (next() * 16000. - 8000.) as f32, next() as f32), Vec4::ZERO]).collect();
        // The renderer's, at f64, from the same f32 points; and the shader's functions here.
        let exact: Vec<f64> = points.chunks(2).map(|q| {
            layer.fraction_for_test(q[0].y as f64, q[0].w as f64, &Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64))
        }).collect();
        let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_DENSITY)).collect();
        let cloudy = exact.iter().filter(|e| **e > 0.).count();
        let worst = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).fold(0., f64::max);
        println!("{} of {} points in cloud; f32 within {:.1e} of the renderer's f64", cloudy, n, worst);
        assert!(cloudy > n / 20, "{} of {} points in cloud", cloudy, n);
        assert!(worst < 2e-3, "the shader's density differs from the renderer's by {}", worst);

        // On the device.
        let Some(on_device) = probe(&record, &points, pathtrace::PROBE_DENSITY) else { return };
        let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
        println!("on the device: within {:.1e} of the shader's functions on the CPU", worst);
        assert!(worst < 1e-4, "the device's density differs from the shader's on the CPU by {}", worst);
    }

    /// Altocumulus likewise (`shared::altocumulus`): through a layer with waves and turrets,
    /// the shader's density is the renderer's to f32's precision, here and on the device.
    #[test]
    fn test_altocumulus_is_the_renderers_on_the_device() {
        use crate::procedural::benard_altocumulus::BenardAltocumulus;
        let mut layer = BenardAltocumulus::new(4000., 250., 5);
        (layer.undulation, layer.castellanus, layer.turret_height, layer.anisotropy, layer.patchiness) = (0.3, 0.4, 3., 1.6, 0.);
        layer.wind_direction = crate::na::Vector2::new(1., 0.3).normalize();
        let record = layer.export();
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let mut next = move || rng.next_f64();
        let n = 4096;
        let (lo, hi) = (layer.bottom() - 50., layer.top() + 50.);
        let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
            (next() * 16000. - 8000.) as f32, (lo + next() * (hi - lo)) as f32, (next() * 16000. - 8000.) as f32, 0.), Vec4::ZERO]).collect();
        let exact: Vec<f64> = points.chunks(2).map(|q| layer.fraction(&Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64))).collect();
        let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_ALTOCUMULUS)).collect();
        let cloudy = exact.iter().filter(|e| **e > 0.).count();
        // (Edges are steep: a point's density moves by what f32 moves the point, so the mean
        // difference is the measure, with a looser bound on the worst.)
        let differences: Vec<f64> = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).collect();
        let (worst, mean) = (differences.iter().cloned().fold(0., f64::max), differences.iter().sum::<f64>() / n as f64);
        println!("{} of {} points in cloud; f32 within {:.1e} of the renderer's f64 (mean {:.1e})", cloudy, n, worst, mean);
        assert!(cloudy > n / 40, "{} of {} points in cloud", cloudy, n);
        assert!(worst < 2e-2 && mean < 1e-4, "the shader's density differs from the renderer's by {} (mean {})", worst, mean);

        let Some(on_device) = probe(&record, &points, pathtrace::PROBE_ALTOCUMULUS) else { return };
        let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
        println!("on the device: within {:.1e} of the shader's functions on the CPU", worst);
        assert!(worst < 1e-3, "the device's density differs from the shader's on the CPU by {}", worst);
    }

    /// Cirrus likewise (`shared::cirrus`): the shader's density at f32 is the renderer's at
    /// f64 but for the rounding, and the device computes what the shader's functions do here.
    #[test]
    fn test_cirrus_is_the_renderers_on_the_device() {
        use crate::procedural::hogan_kew_cirrus::{CirrusField, CirrusGenus, HoganKewCirrus};
        for genus in [CirrusGenus::Cirrus, CirrusGenus::Cirrocumulus] {
            let mut layer = HoganKewCirrus::new(CirrusField { seed: 5, ..CirrusField::new(genus) });
            (layer.time, layer.evolution_period) = (300., 4000.);
            let record = layer.export();
            let mut rng = crate::geometry::Pcg32::new(5, 0);
            let mut next = move || rng.next_f64();
            let n = 4096;
            let (lo, hi) = (layer.base - 50., layer.top + 50.);
            let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
                (next() * 40000. - 20000.) as f32, (lo + next() * (hi - lo)) as f32, (next() * 40000. - 20000.) as f32, 0.), Vec4::ZERO]).collect();
            let exact: Vec<f64> = points.chunks(2).map(|q| layer.fraction(&Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64))).collect();
            let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_CIRRUS)).collect();
            let cloudy = exact.iter().filter(|e| **e > 0.).count();
            let differences: Vec<f64> = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).collect();
            let (worst, mean) = (differences.iter().cloned().fold(0., f64::max), differences.iter().sum::<f64>() / n as f64);
            println!("{} of {} points in cloud; f32 within {:.1e} of the renderer's f64 (mean {:.1e})", cloudy, n, worst, mean);
            assert!(cloudy > n / 40, "{} of {} points in cloud", cloudy, n);
            assert!(worst < 2e-2 && mean < 1e-4, "the shader's density differs from the renderer's by {} (mean {})", worst, mean);

            let Some(on_device) = probe(&record, &points, pathtrace::PROBE_CIRRUS) else { return };
            // (The flow's curl is a difference of noise a thousandth of a cell apart: what the
            // device rounds otherwise there moves the ice's source by decimetres, and the
            // density at a fibre's edge with it. So the mean, as against f64.)
            let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
            let mean = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).sum::<f32>() / n as f32;
            println!("on the device: within {:.1e} of the shader's functions on the CPU (mean {:.1e})", worst, mean);
            assert!(worst < 2e-2 && mean < 1e-4, "the device's density differs from the shader's on the CPU by {} (mean {})", worst, mean);
        }
    }

    /// Precipitation likewise (`shared::rain`): rain, and snow melting to rain as it
    /// evaporates.
    #[test]
    fn test_precipitation_is_the_renderers_on_the_device() {
        use crate::procedural::precipitation::{Kind, Precipitation};
        use crate::volume::DensityField;
        for snow in [false, true] {
            let mut layer = Precipitation::new(2000., 5);
            if snow {
                (layer.kind, layer.melting_level, layer.fall_depth, layer.base_variation, layer.shear) = (Kind::Snow, Some(900.), 2500., 200., 1.5);
            }
            let record = layer.export();
            let peak = layer.peak_extinction();
            let mut rng = crate::geometry::Pcg32::new(5, 0);
            let mut next = move || rng.next_f64();
            let n = 4096;
            let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
                (next() * 40000. - 20000.) as f32, (next() * 2100. - 50.) as f32, (next() * 40000. - 20000.) as f32, 0.), Vec4::ZERO]).collect();
            let exact: Vec<f64> = points.chunks(2).map(|q| layer.density(&Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64)) / peak).collect();
            let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_RAIN)).collect();
            let wet = exact.iter().filter(|e| **e > 0.).count();
            let differences: Vec<f64> = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).collect();
            let (worst, mean) = (differences.iter().cloned().fold(0., f64::max), differences.iter().sum::<f64>() / n as f64);
            println!("{} of {} points in it; f32 within {:.1e} of the renderer's f64 (mean {:.1e})", wet, n, worst, mean);
            assert!(wet > n / 40, "{} of {} points in precipitation", wet, n);
            assert!(worst < 2e-2 && mean < 1e-4, "the shader's density differs from the renderer's by {} (mean {})", worst, mean);

            let Some(on_device) = probe(&record, &points, pathtrace::PROBE_RAIN) else { return };
            let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
            let mean = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).sum::<f32>() / n as f32;
            println!("on the device: within {:.1e} of the shader's functions on the CPU (mean {:.1e})", worst, mean);
            assert!(worst < 2e-2 && mean < 1e-4, "the device's density differs from the shader's on the CPU by {} (mean {})", worst, mean);
        }
    }

    /// A cumulonimbus likewise (`shared::cumulonimbus`).
    #[test]
    fn test_cumulonimbus_is_the_renderers_on_the_device() {
        use crate::procedural::cumulonimbus::Cumulonimbus;
        let layer = Cumulonimbus::new(5);
        let record = layer.export();
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let mut next = move || rng.next_f64();
        let n = 8192;
        // About the main tower and the anvil's near end.
        let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
            (next() * 16000. - 5000.) as f32, (layer.bottom() + next() * (layer.top() - layer.bottom())) as f32, (next() * 10000. - 5000.) as f32, 0.), Vec4::ZERO]).collect();
        let exact: Vec<f64> = points.chunks(2).map(|q| layer.fraction(&Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64)).0).collect();
        let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_CUMULONIMBUS)).collect();
        let cloudy = exact.iter().filter(|e| **e > 0.).count();
        let differences: Vec<f64> = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).collect();
        let (worst, mean) = (differences.iter().cloned().fold(0., f64::max), differences.iter().sum::<f64>() / n as f64);
        println!("{} of {} points in cloud; f32 within {:.1e} of the renderer's f64 (mean {:.1e})", cloudy, n, worst, mean);
        assert!(cloudy > n / 40, "{} of {} points in cloud", cloudy, n);
        assert!(worst < 5e-2 && mean < 2e-4, "the shader's density differs from the renderer's by {} (mean {})", worst, mean);

        let Some(on_device) = probe(&record, &points, pathtrace::PROBE_CUMULONIMBUS) else { return };
        let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
        let mean = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).sum::<f32>() / n as f32;
        println!("on the device: within {:.1e} of the shader's functions on the CPU (mean {:.1e})", worst, mean);
        assert!(worst < 5e-2 && mean < 2e-4, "the device's density differs from the shader's on the CPU by {} (mean {})", worst, mean);
    }

    /// Mammatus likewise (`shared::mammatus`).
    #[test]
    fn test_mammatus_is_the_renderers_on_the_device() {
        use crate::procedural::mammatus::Mammatus;
        let mut layer = Mammatus::new(5);
        (layer.base_variation, layer.sag, layer.base_softness, layer.top_variation) = (300., 400., 200., 150.);
        let record = layer.export();
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let mut next = move || rng.next_f64();
        let n = 4096;
        let (lo, hi) = (layer.bottom() - 50., layer.base + 600.);
        let points: Vec<Vec4> = (0..n).flat_map(|_| [Vec4::new(
            (next() * 40000. - 35000.) as f32, (lo + next() * (hi - lo)) as f32, (next() * 40000. - 20000.) as f32, 0.), Vec4::ZERO]).collect();
        let exact: Vec<f64> = points.chunks(2).map(|q| layer.fraction(&Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64))).collect();
        let here: Vec<f32> = points.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_MAMMATUS)).collect();
        let cloudy = exact.iter().filter(|e| **e > 0.).count();
        let differences: Vec<f64> = here.iter().zip(&exact).map(|(f, e)| (*f as f64 - e).abs()).collect();
        let (worst, mean) = (differences.iter().cloned().fold(0., f64::max), differences.iter().sum::<f64>() / n as f64);
        println!("{} of {} points in cloud; f32 within {:.1e} of the renderer's f64 (mean {:.1e})", cloudy, n, worst, mean);
        assert!(cloudy > n / 40, "{} of {} points in cloud", cloudy, n);
        assert!(worst < 5e-2 && mean < 2e-4, "the shader's density differs from the renderer's by {} (mean {})", worst, mean);

        let Some(on_device) = probe(&record, &points, pathtrace::PROBE_MAMMATUS) else { return };
        let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
        let mean = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).sum::<f32>() / n as f32;
        println!("on the device: within {:.1e} of the shader's functions on the CPU (mean {:.1e})", worst, mean);
        assert!(worst < 5e-2 && mean < 2e-4, "the device's density differs from the shader's on the CPU by {} (mean {})", worst, mean);
    }

    /// Light through a cloud is one walk (`shared::media::ratio_track` over the shared
    /// density and its bounds), which the renderer's `Volume::transmittance` takes at f64 and a
    /// kernel at f32. Along rays through a layer, up, down and aslant: the shader's estimate,
    /// averaged, is the renderer's (they draw different random numbers, so to within their
    /// noise), and the kernel on the device computes what the shader's function does here.
    #[test]
    fn test_light_through_clouds_is_the_renderers_on_the_device() {
        use crate::procedural::nubis_clouds::NubisCloudLayer;
        use crate::shapes::geometry::Geometry;
        let (centre, extent) = (crate::na::Vector2::new(0., 0.), 20000.);
        let layer = std::sync::Arc::new(NubisCloudLayer::new(1500., 4000., centre, extent, 7));
        let record = layer.export();
        let bounds = crate::shapes::bbox::BBox::new(Vector3::new(-extent, 1500., -extent), Vector3::new(extent, 4000., extent));
        let volume = crate::volume::Volume::new(bounds, layer.clone());
        let mut rng = crate::geometry::Pcg32::new(9, 0);
        let mut next = move || rng.next_f64();
        let n = 256;
        // From the ground up through the layer, and from above it down, far enough to cross.
        let rays: Vec<Vec4> = (0..n).flat_map(|i| {
            let up = i % 2 == 0;
            let o = Vec4::new((next() * 12000. - 6000.) as f32, if up { 10. } else { 6000. }, (next() * 12000. - 6000.) as f32, 20000.);
            let d = Vector3::new(next() - 0.5, if up { 0.3 + next() } else { -0.3 - next() }, next() - 0.5).normalize();
            [o, Vec4::new(d.x as f32, d.y as f32, d.z as f32, 0.)]
        }).collect();
        let here: Vec<f32> = rays.chunks(2).enumerate().map(|(i, q)| pathtrace::probe(&record, 0, i as u32, q[0], q[1], pathtrace::PROBE_RAYS)).collect();
        // The renderer's, with as many estimates and more: its mean, to compare means with.
        let samples = 16 * pathtrace::PROBE_ESTIMATES as usize;
        crate::volume::begin_path();
        let exact: Vec<f64> = rays.chunks(2).enumerate().map(|(i, q)| {
            let unit = Vector3::new(q[1].x as f64, q[1].y as f64, q[1].z as f64).normalize();
            let ray = Ray { ro: Vector3::new(q[0].x as f64, q[0].y as f64, q[0].z as f64), rd: unit };
            crate::geometry::seed_rand(77, i as u64);
            (0..samples).map(|_| volume.transmittance(&ray, 0., q[0].w as f64)).sum::<f64>() / samples as f64
        }).collect();
        let (mut shadowed, mut sum, mut sum_exact, mut worst) = (0, 0., 0., 0f64);
        for (h, e) in here.iter().zip(&exact) {
            shadowed += (*e < 0.9) as usize;
            sum += *h as f64;
            sum_exact += e;
            worst = worst.max((*h as f64 - e).abs());
        }
        println!("{} of {} rays shadowed; mean through {:.4} against the renderer's {:.4}; worst ray differs by {:.3}", shadowed, n, sum / n as f64, sum_exact / n as f64, worst);
        assert!(shadowed > n / 8, "{} of {} rays cross cloud", shadowed, n);
        // The means over all the rays agree closely; a ray's own 64 estimates are noisier.
        assert!((sum - sum_exact).abs() / (n as f64) < 0.01, "mean {} against {}", sum / n as f64, sum_exact / n as f64);
        assert!(worst < 0.2, "a ray's transmittance differs by {}", worst);

        let Some(on_device) = probe(&record, &rays, pathtrace::PROBE_RAYS) else { return };
        let worst = here.iter().zip(&on_device).map(|(f, d)| (f - d).abs()).fold(0f32, f32::max);
        println!("on the device: within {:.1e} of the shader's function on the CPU", worst);
        assert!(worst < 2e-2, "the device's transmittance differs from the shader's on the CPU by {}", worst);
    }

    /// `probe_clouds` run on the device over `points` (two vectors a probe: `pathtrace::probe`),
    /// for the cloud whose record is `record`; None without a device.
    fn probe(record: &[f32], points: &[Vec4], what: u32) -> Option<Vec<f32>> {
        pollster::block_on(async {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
            let adapter = match instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("skipped: {}", e);
                    return None;
                },
            };
            let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.ok()?;
            let source = if adapter.get_info().backend != wgpu::Backend::Vulkan || std::env::var_os("RAYS_GPU_WGSL").is_some() {
                wgpu::ShaderSource::Wgsl(include_str!("pathtrace.wgsl").into())
            } else {
                wgpu::util::make_spirv(include_bytes!("pathtrace.spv"))
            };
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("pathtrace"), source });
            let mut params = Params::zeroed();
            params.counts.x = points.len() as u32 / 2;
            params.counts.y = what;
            let buffer = |contents: &[u8], usage| device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents, usage });
            let uniform = buffer(bytemuck::bytes_of(&params), wgpu::BufferUsages::UNIFORM);
            let tables = buffer(bytemuck::cast_slice(record), wgpu::BufferUsages::STORAGE);
            let out = buffer(bytemuck::cast_slice(points), wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
            let read = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: out.size(), usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let entry = |binding: u32, ty| wgpu::BindGroupLayoutEntry {
                binding, visibility: wgpu::ShaderStages::COMPUTE, count: None,
                ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            };
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[
                entry(0, wgpu::BufferBindingType::Uniform),
                entry(1, wgpu::BufferBindingType::Storage { read_only: true }),
                entry(2, wgpu::BufferBindingType::Storage { read_only: false }),
            ] });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[Some(&layout)], immediate_size: 0 });
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None, layout: Some(&pipeline_layout), module: &module, entry_point: Some("probe_clouds"),
                compilation_options: Default::default(), cache: None,
            });
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: tables.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: out.as_entire_binding() },
            ] });
            let mut encoder = device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups((points.len() as u32 / 2).div_ceil(pathtrace::WORKGROUP), 1, 1);
            }
            encoder.copy_buffer_to_buffer(&out, 0, &read, 0, None);
            queue.submit(Some(encoder.finish()));
            read.map_async(wgpu::MapMode::Read, .., |r| r.expect("mapping the probe's points"));
            device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
            let values: Vec<[f32; 4]> = bytemuck::pod_collect_to_vec(&read.get_mapped_range(..).unwrap());
            Some(values.chunks(2).map(|v| v[1][3]).collect())
        })
    }

    /// The kernel's sea against the renderer's, ray by ray: where a ray meets it (the wave
    /// march and the planet's sphere, in f32) and the normal there (the tiles' slopes).
    #[test]
    fn test_the_sea_matches_the_renderer() {
        use crate::shapes::geometry::Geometry;
        use pathtrace::glam::Vec3;
        // A plane and the planet, flat-crested and with choppy crests (a brisk wind's, so that
        // they are sharp).
        for (radius, choppiness) in [("0", 0), ("6360000", 0), ("0", 1), ("6360000", 1)] {
            let s = SceneFile::from_string(format!(r#"{{
                "width": 4, "height": 4, "background": [0, 0, 0],
                "camera": {{ "location": [0, 3, -12], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.7 }},
                "objects": [ {{ "type": "ocean", "wind": [7, 2], "choppiness": {}, "whitecaps": 0, "planet_radius": {}, "detail_center": [0, -6], "detail_distance": 30 }} ]
            }}"#, choppiness, radius));
            let flat = FlatScene::build(&s).unwrap();
            let b = Buffers::build(&s, &flat).unwrap();
            let shader = b.scene(None);
            let sea = &flat.shapes[0].0;
            let mut rng = crate::geometry::Pcg32::new(5, 0);
            let mut next = move || rng.next_f64();
            let (mut both, mut one, mut worst_t, mut worst_n, mut sum_t) = (0, 0, 0f64, 0f64, 0f64);
            for _ in 0..4000 {
                // From above the waves, down at the sea near and far.
                let ro = Vector3::new(next() * 40. - 20., 0.6 + next() * 6., next() * 40. - 26.);
                let rd = Vector3::new(next() - 0.5, -next() * 0.6 - 0.002, next() - 0.5).normalize();
                // The kernel's ray is in the render's frame, about the camera (`kernels::origin`);
                // the renderer's, the same ray in the world's.
                let local = ro - b.origin;
                let (ro32, rd32) = (Vec3::new(local.x as f32, local.y as f32, local.z as f32), Vec3::new(rd.x as f32, rd.y as f32, rd.z as f32).normalize());
                let ray = Ray { ro: Vector3::new(ro32.x as f64, ro32.y as f64, ro32.z as f64) + b.origin, rd: Vector3::new(rd32.x as f64, rd32.y as f64, rd32.z as f64).normalize() };
                let cpu = sea.geometry.intersects(&ray);
                let (t, prim) = pathtrace::nearest(&shader, ro32, rd32, 0., f32::MAX, false);
                match (cpu, prim != u32::MAX) {
                    (Some(x), true) => {
                        both += 1;
                        let dt = (t as f64 - x.dist).abs();
                        sum_t += dt;
                        worst_t = worst_t.max(dt / (1. + x.dist));
                        let n = pathtrace::sea_normal(&shader, ro32 + rd32 * x.dist as f32);
                        let dn = (Vector3::new(n.x as f64, n.y as f64, n.z as f64) - x.normal).norm();
                        worst_n = worst_n.max(dn);
                    },
                    (None, false) => {},
                    _ => one += 1,
                }
            }
            println!("radius {}, choppiness {}: {} hits, {} disagree on hitting, mean |dt| {:.2e}, worst relative dt {:.2e}, worst normal {:.2e}", radius, choppiness, both, one, sum_t / both as f64, worst_t, worst_n);
            assert!(both > 3000 && one * 200 < both, "{} of {} rays disagree on hitting the sea", one, both);
            assert!(worst_n < 2e-3, "normals differ by {}", worst_n);
        }
    }

    /// The sea is a shape the kernels intersect themselves (`pathtrace::ocean_hit`: the
    /// planet's sphere or a plane, marched over its waves near the camera), and its water a
    /// material they shade (`MAT_WATER`). As a plane and as the planet, with a sphere standing in it to shadow it and be
    /// reflected, the picture is the wavefront integrator's, whose sea is the renderer's own.
    #[test]
    fn test_intersects_the_sea() {
        use crate::wavefront::integrator::tests::{mean_image, tiles};
        // Flat-crested without foam, then with choppy crests, and with more foam than this
        // wind raises (whitecaps over a third of it), so that the foam counts.
        for (radius, sea) in [("0", r#""choppiness": 0, "whitecaps": 0"#), ("6360000", r#""choppiness": 0, "whitecaps": 0"#),
                              ("0", r#""whitecaps": 0.35"#), ("6360000", r#""choppiness": 1"#)] {
            let mut s = SceneFile::from_string(format!(r#"{{
                "width": 24, "height": 16, "background": [0.3, 0.4, 0.6], "max_depth": 4, "path_termination": "russian_roulette",
                "sun": {{ "direction": [-1, 1, 1], "intensity": 3 }},
                "camera": {{ "location": [0, 3, -12], "lookat": [0, 0.5, 0], "up": [0, 1, 0], "angle": 0.7 }},
                "materials": {{ "R": {{ "type": "lambertian", "albedo": [0.7, 0.2, 0.1] }} }},
                "objects": [
                    {{ "type": "ocean", "wind": [7, 2], {}, "planet_radius": {}, "detail_center": [0, -6], "detail_distance": 30 }},
                    {{ "type": "sphere", "location": [0, 0.6, 0], "radius": 1.5, "material": "R" }}
                ]
            }}"#, sea, radius));
            s.render.supersamples = 128;
            let flat = FlatScene::build(&s).unwrap();
            assert_eq!(flat.shapes.len(), 1);
            let mut gpu = match GpuRenderer::new(&s, &flat) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("skipped: {}", e);
                    return;
                },
            };
            let (w, h) = (s.image.width, s.image.height);
            let rc = Mutex::new(RenderContext::new(w, h, false, "test.png"));
            gpu.render(&s, &flat, &rc, |_| ());
            let rc = rc.into_inner().unwrap();
            // Its water too is the kernels' (`pathtrace::MAT_WATER`), foam and all: nothing
            // waits for the host.
            assert_eq!(rc.schedule.stage(crate::scheduler::Stage::Host).items, 0);
            let on_gpu: Vec<Color> = (0..w * h).map(|i| rc.get_pixel(i % w, i / w)).collect();
            let cpu = mean_image(&s, 128);
            let (a, b) = (tiles(&on_gpu, w, h, 3, 2), tiles(&cpu, w, h, 3, 2));
            for t in 0..6 {
                for ch in 0..3 {
                    assert!((a[t][ch] - b[t][ch]).abs() < 0.06 * b[t][ch] + 2e-3, "radius {}, {}: tile {} channel {}: GPU {} vs CPU {}", radius, sea, t, ch, a[t][ch], b[t][ch]);
                }
            }
        }
    }

    /// The CPU queue (`host`): paths that hit a material the kernels have no code for wait
    /// for the host, which shades them with the scene's own and sends them on. The picture is
    /// the wavefront integrator's, which shades the same vertices the same way, and the
    /// report counts them.
    #[test]
    fn test_halves_render_the_whole_blocks_picture() {
        // A block in two halves (by row, so the host shades one's paths while the device runs
        // the other's rounds) traces the same paths, in other queues: the same picture, bit for
        // bit, the image's sums added once after both.
        use crate::wavefront::integrator::tests::host_scene;
        let mut s = host_scene();
        s.render.supersamples = 16;
        let flat = FlatScene::build(&s).unwrap();
        let render = |halves: usize| -> Option<Vec<Color>> {
            let mut gpu = match pollster::block_on(GpuRenderer::create_with(&s, &flat, halves)) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("skipped: {}", e);
                    return None;
                },
            };
            assert_eq!(gpu.halves.len(), halves);
            let (w, h) = (s.image.width, s.image.height);
            let rc = Mutex::new(RenderContext::new(w, h, false, "test.png"));
            gpu.render(&s, &flat, &rc, |_| ());
            let rc = rc.into_inner().unwrap();
            Some((0..w * h).map(|i| rc.get_pixel(i % w, i / w)).collect())
        };
        let Some(whole) = render(1) else { return };
        let halves = render(2).unwrap();
        assert!(whole.iter().any(|c| c.rgb.max() > 0.), "a picture");
        for (k, (a, b)) in whole.iter().zip(&halves).enumerate() {
            assert_eq!(a.rgb, b.rgb, "pixel {}", k);
        }
    }

    #[test]
    fn test_host_shades_what_the_kernels_cant() {
        use crate::scheduler::Stage;
        use crate::wavefront::integrator::tests::{host_scene, mean_image, tiles};
        let mut s = host_scene();
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
        gpu.render(&s, &flat, &rc, |_| ());
        let rc = rc.into_inner().unwrap();
        let host = rc.schedule.stage(Stage::Host);
        // The camera sees host materials, and paths reach them again after a bounce: more
        // than one wait a pass.
        assert!(host.items > 0 && host.batches > 0, "{:?} in {} passes", host, rc.schedule.passes);
        let on_gpu: Vec<Color> = (0..w * h).map(|i| rc.get_pixel(i % w, i / w)).collect();
        let cpu = mean_image(&s, 256);
        let (a, b) = (tiles(&on_gpu, w, h, 3, 2), tiles(&cpu, w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.05 * b[t][ch] + 1e-3, "tile {} channel {}: GPU {} vs CPU {}", t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// A scene with fog in a box over a floor, in sun, a lamp, an area light and a sky: seen
    /// directly and in a mirror (all its scattering), and by light off the floor and by its own
    /// (truncated), shadowing the floor and shadowed by a ball.
    fn foggy_scene(density: &str) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "width": 24, "height": 16, "background": [0.2, 0.25, 0.3], "max_depth": 24, "path_termination": "russian_roulette",
            "sun": {{ "direction": [-1, 2, -1], "intensity": 2 }},
            "lights": [ {{ "location": [1, 3, -1], "intensity": 4 }} ],
            "camera": {{ "location": [0, 1.5, -5], "lookat": [0, 0.6, 0], "up": [0, 1, 0], "angle": 0.6 }},
            "materials": {{
                "F": {{ "type": "lambertian", "albedo": [0.6, 0.6, 0.6] }},
                "M": {{ "type": "metal", "reflective": [0.9, 0.9, 0.9] }},
                "E": {{ "type": "diffuse-light", "color": [1, 0.9, 0.8], "intensity": 8 }}
            }},
            "objects": [
                {{ "type": "quad", "points": [[-6, 0, -6], [-6, 0, 6], [6, 0, -6], [6, 0, 6]], "material": "F" }},
                {{ "type": "sphere", "location": [1.5, 0.5, 0], "radius": 0.5, "material": "M" }},
                {{ "type": "sphere", "location": [-0.3, 1.6, -0.2], "radius": 0.25, "material": "F" }},
                {{ "type": "quad", "points": [[-0.5, 2.5, 0.5], [0.5, 2.5, 0.5], [-0.5, 2.5, 1.5], [0.5, 2.5, 1.5]], "material": "E" }},
                {{ "type": "volume", "min": [-1.2, 0.2, -0.8], "max": [0.8, 1.3, 0.8], {density}, "albedo": [0.9, 0.8, 0.7], "g": 0.8 }}
            ]
        }}"#, density = density))
    }

    /// Participating media in the kernels (`pathtrace::media_collision`, the medium's vertex
    /// in `shade`, `media_transmittance`): the picture is the path tracer's, tile by tile,
    /// with delta-M truncation, with the similarity relations and with the Oz approximation's
    /// sunlight too.
    #[test]
    fn test_media_are_the_path_tracers() {
        for density in [r#""density": 1.5"#, r#""density": 6, "similarity_after": 2"#, r#""density": 6, "oz": true"#] {
            media_agree(density);
        }
    }

    fn media_agree(density: &str) {
        use crate::wavefront::integrator::tests::{path_image, tiles};
        let mut s = foggy_scene(density);
        s.render.supersamples = 512;
        let flat = FlatScene::build(&s).unwrap();
        assert_eq!(flat.media.len(), 1);
        assert!(flat.media[0].peak > 0. && flat.media[0].scale < 1.);
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let (w, h) = (s.image.width, s.image.height);
        let rc = Mutex::new(RenderContext::new(w, h, false, "test.png"));
        gpu.render(&s, &flat, &rc, |_| ());
        let rc = rc.into_inner().unwrap();
        let on_gpu: Vec<Color> = (0..w * h).map(|i| rc.get_pixel(i % w, i / w)).collect();
        let cpu = path_image(&s, 512);
        let (a, b) = (tiles(&on_gpu, w, h, 3, 2), tiles(&cpu, w, h, 3, 2));
        for t in 0..6 {
            for ch in 0..3 {
                assert!((a[t][ch] - b[t][ch]).abs() < 0.04 * b[t][ch] + 1e-3, "{}: tile {} channel {}: GPU {} vs CPU {}", density, t, ch, a[t][ch], b[t][ch]);
            }
        }
    }

    /// The device against the shader run on the CPU, sample for sample, for variations of a
    /// small scene: which part of the kernels a device computes differently. Written to find
    /// why lavapipe on one kind of CI runner (AMD Zen 5, Mesa 26, LLVM 21) rendered a few
    /// percent brighter than on others; run with `--ignored --nocapture`.
    #[test]
    #[ignore]
    fn measure_device_against_shader() {
        // The two scenes of the tests that fail there (`scene` and `hard_scene`), whole and
        // with one thing taken out at a time: (what is left out, its text in the scene).
        let materials = r#"{
            "width": 24, "height": 16, "supersamples": 32, "background": [0.2, 0.25, 0.3], "max_depth": 6,
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
        }"#;
        let hard = r#"{
            "width": 16, "height": 16, "supersamples": 32, "background": [0, 0, 0], "max_depth": 6, "path_termination": "russian_roulette",
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
                { "type": "transform", "item": { "type": "quad", "points": [[-1, 3, -1], [-1, 3, 1], [1, 3, -1], [1, 3, 1]] }, "scale": [-1, 1, 1], "material": "QUAD" },
                { "type": "sphere", "location": [0, 0.6, 0], "radius": 0.6, "material": "G" },
                { "type": "sphere", "location": [-1.8, 0.5, 0.5], "radius": 0.5, "material": "F" },
                { "type": "sphere", "location": [-40, 7, -30], "radius": 0.01, "material": "STAR" }
            ]
        }"#;
        let cases: [(&str, &str, &[(&str, &str, &str)]); 2] = [
            ("materials", materials, &[
                ("nothing", "", ""),
                ("the sun", r#""sun": { "direction": [-1, 2, -1], "intensity": 2 },"#, ""),
                ("the lamp", r#"{ "location": [1, 3, -1], "intensity": 4 }"#, ""),
                ("the sphere light", r#",
                { "type": "sphere", "location": [0, 2.5, 1], "radius": 0.3, "material": "E" }"#, ""),
                ("the plastic", r#""material": "P""#, r#""material": "F""#),
                ("the glass", r#""material": "G""#, r#""material": "F""#),
                ("the gold", r#""material": "C""#, r#""material": "F""#),
                ("the bounces", r#""max_depth": 6"#, r#""max_depth": 0"#),
                ("roulette", r#""path_termination": "russian_roulette","#, ""),
            ]),
            ("hard", hard, &[
                ("nothing", "", ""),
                ("the star", r#",
                { "type": "sphere", "location": [-40, 7, -30], "radius": 0.01, "material": "STAR" }"#, ""),
                ("the quad light", r#"{ "type": "transform", "item": { "type": "quad", "points": [[-1, 3, -1], [-1, 3, 1], [1, 3, -1], [1, 3, 1]] }, "scale": [-1, 1, 1], "material": "QUAD" },"#, ""),
                ("the ceiling light", r#"{ "type": "plane", "y": 8, "material": "SKY" },"#, ""),
                ("the glass", r#""material": "G""#, r#""material": "F""#),
                ("the bounces", r#""max_depth": 6"#, r#""max_depth": 0"#),
            ]),
        ];
        for (name, scene, variants) in cases {
            for (what, out, instead) in variants {
                assert!(out.is_empty() || scene.contains(out), "{}: no {:?} to take out", name, out);
                let s = SceneFile::from_string(scene.replacen(out, instead, 1));
                let flat = FlatScene::build(&s).unwrap();
                let mut gpu = GpuRenderer::new(&s, &flat).unwrap();
                let b = Buffers::build(&s, &flat).unwrap();
                let shader = b.scene(None);
                let (w, passes) = (s.image.width, 32);
                let mut cpu = vec![0f64; w * s.image.height];
                for k in 0..passes {
                    for (index, ray) in crate::kernels::camera_rays(&s, k, &b.origin).iter().enumerate() {
                        if ray.d.w < 0.5 {
                            continue;
                        }
                        let mut rng = pathtrace::path_seed(index as u32, k as u32, b.params.seed.x);
                        let mut rays = 0;
                        let c = pathtrace::radiance(&shader, ray.o.truncate(), ray.d.truncate(), &mut rng, &mut rays) * ray.o.w;
                        cpu[index] += c.x as f64;
                    }
                }
                let sums = gpu.run(&s, 0..passes);
                let off: Vec<usize> = (0..cpu.len()).filter(|&i| (sums[i][0] as f64 - cpu[i]).abs() > 1e-3 * cpu[i].abs() + 1e-4).collect();
                let (g, c): (f64, f64) = (sums.iter().map(|v| v[0] as f64).sum(), cpu.iter().sum());
                let brighter = off.iter().filter(|&&i| sums[i][0] as f64 > cpu[i]).count();
                // The largest differences: (x, y) device against shader, over the 32 samples.
                let mut worst = off.clone();
                worst.sort_by(|a, b| (sums[*b][0] as f64 - cpu[*b]).abs().total_cmp(&(sums[*a][0] as f64 - cpu[*a]).abs()));
                let some: Vec<String> = worst.iter().take(4).map(|&i| format!("({}, {}) {:.3} vs {:.3}", i % w, i / w, sums[i][0], cpu[i])).collect();
                eprintln!("{} without {}: device / shader {:.4}, {} of {} pixels differ, {} brighter; worst {}", name, what, g / c, off.len(), cpu.len(), brighter, some.join("; "));
            }
        }
    }

    /// A cloud scene on the device against the path tracer, in bands of rows from the bottom
    /// of the picture, in linear light: `RAYS_SCENE` the scene file (a Nubis layer over the
    /// sea by default), `RAYS_EDIT` a JSON object of fields to set first, an object's as
    /// `"2.similarity_after"`. How the clouds' port was checked, and how the path tracer's sun
    /// probes were found to be dark where the similarity relations begin. (The band at the
    /// horizon differs by some 5% without any cloud: the sea's, still to be found.)
    #[test]
    #[ignore]
    fn measure_clouds() {
        use crate::wavefront::integrator::tests::path_image;
        let file = std::env::var("RAYS_SCENE").unwrap_or("demo/scenes/nubis-clouds.json".into());
        let mut v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        v["width"] = 32.into();
        v["height"] = 18.into();
        if let Ok(edit) = std::env::var("RAYS_EDIT") {
            let e: serde_json::Value = serde_json::from_str(&edit).unwrap();
            for (k, x) in e.as_object().unwrap() {
                if let Some((o, f)) = k.split_once('.') {
                    v["objects"][o.parse::<usize>().unwrap()][f] = x.clone();
                } else {
                    v[k] = x.clone();
                }
            }
        }
        let n = 512;
        let mut s = SceneFile::from_string(v.to_string());
        s.render.supersamples = n;
        let flat = FlatScene::build(&s).unwrap();
        let mut gpu = GpuRenderer::new(&s, &flat).unwrap();
        let (w, h) = (s.image.width, s.image.height);
        let rc = Mutex::new(RenderContext::new(w, h, false, "test.png"));
        gpu.render(&s, &flat, &rc, |_| ());
        let rc = rc.into_inner().unwrap();
        let img = path_image(&s, n);
        for band in 0..6 {
            let rows = band * 3..band * 3 + 3;
            let g: f64 = rows.clone().flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| rc.get_pixel(x, y).rgb.mean()).sum();
            let c: f64 = rows.flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| img[y * w + x].rgb.mean()).sum();
            eprintln!("rows from {}: GPU {:.4} CPU {:.4} ratio {:.4}", band * 3, g / (3 * w) as f64, c / (3 * w) as f64, g / c);
        }
    }

    /// How far the kernels on the device are from the same kernels on the CPU, sample for
    /// sample: each pixel's first sample, by both, and how many differ in their bits and by
    /// how much. `RAYS_SCENE` names a scene; `--ignored --nocapture`.
    #[test]
    #[ignore]
    fn measure_device_against_cpu() {
        let scenes: Vec<String> = match std::env::var("RAYS_SCENE") {
            Ok(f) => f.split(',').map(String::from).collect(),
            Err(_) => vec!["demo/demo.json".into()],
        };
        for file in scenes {
            let s = SceneFile::from_file(&file, &crate::scenefile::Overrides { width: Some(96), height: Some(60), seed: None });
            let flat = FlatScene::build(&s).unwrap();
            let kernels = crate::wavefront::integrator::Kernels::new(&s, &flat).unwrap();
            let mut gpu = match GpuRenderer::new(&s, &flat) {
                Ok(g) => g,
                Err(e) => { println!("{}: not on the device ({})", file, e); continue; },
            };
            let (cpu, _, _) = kernels.pass(0);
            let sums = gpu.run(&s, 0..1);
            let n = cpu.len();
            let (mut same, mut ulp, mut near, mut close, mut far, mut worst) = (0, 0, 0, 0, 0, 0f64);
            for i in 0..n {
                let c = [cpu[i].rgb.x as f32, cpu[i].rgb.y as f32, cpu[i].rgb.z as f32];
                let g = [sums[i][0], sums[i][1], sums[i][2]];
                let bits = (0..3).map(|k| (c[k].to_bits() as i64 - g[k].to_bits() as i64).abs()).max().unwrap();
                let rel = (0..3).map(|k| ((c[k] - g[k]).abs() / c[k].abs().max(g[k].abs()).max(1e-6)) as f64).fold(0., f64::max);
                worst = worst.max(rel);
                if bits == 0 { same += 1 } else if bits <= 4 { ulp += 1 } else if rel < 1e-5 { near += 1 } else if rel < 1e-3 { close += 1 } else { far += 1 }
            }
            println!("{}: of {} pixels, {} the same bits, {} within 4 steps of a float, {} within 1e-5, {} within 1e-3, {} further (a different path); worst {:.1e}",
                file, n, same, ulp, near, close, far, worst);
        }
    }

    /// Pixel completion (`scheduler::plan`): with adaptive sampling the GPU stops sampling the
    /// pixels that are quiet (a flat background: every sample the same) and carries on with
    /// the noisy ones, each taking its own next samples; the picture is the one it gets from
    /// sampling every pixel in full; and a render stopped part-way and resumed from its film
    /// ends where an uninterrupted one does.
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
            gpu.render(s, &flat, &rc, |_| ());
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
    fn test_rock_on_the_device_is_the_cpus() {
        // The procedural rock in the kernels (`pathtrace::MAT_ROCK`, `shared::rock`) on the
        // device: the CPU's kernels' picture, the same code in other arithmetic.
        let s = SceneFile::from_string(r#"{
            "width": 24, "height": 16, "supersamples": 64, "background": [0.2, 0.25, 0.3], "max_depth": 6,
            "sun": { "direction": [-1, 2, -1], "intensity": 2 },
            "lights": [ { "location": [1, 2, -1], "intensity": 3 } ],
            "camera": { "location": [0, 1.5, -4], "lookat": [0, 0.4, 0], "up": [0, 1, 0], "angle": 0.6 },
            "materials": {
                "R": { "type": "rock", "preset": "sandstone", "scale": 0.3, "bump": 0.6, "seed": 4, "pits": 0.5, "fractal": 0.4,
                       "layers": [[0, [0.6, 0.4, 0.3]], [0.8, [0.5, 0.5, 0.5]]], "varnish": 0.5 },
                "G": { "type": "rock", "preset": "granite", "scale": 0.5, "seed": 2, "lichen": 0.3 }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "R" },
                { "type": "sphere", "location": [-0.5, 0.5, 0], "radius": 0.5, "material": "G" }
            ]
        }"#.to_string());
        let flat = FlatScene::build(&s).unwrap();
        assert!(flat.hosts.is_empty(), "the rock is the kernels'");
        let mut gpu = match GpuRenderer::new(&s, &flat) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("skipped: {}", e);
                return;
            },
        };
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        gpu.render(&s, &flat, &rc, |_| ());
        let gpu_image = rc.lock().unwrap().snapshot();
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        crate::rendercontext::render(&s, &rc, Some(&crate::wavefront::integrator::Kernels::new(&s, &flat).unwrap()), |_| None::<()>, |_| ());
        let cpu_image = rc.lock().unwrap().snapshot();
        let total = |img: &crate::output::LinearImage| img.pixels.iter().fold(Vector3::zeros(), |a, c| a + c.rgb);
        let (g, c) = (total(&gpu_image), total(&cpu_image));
        for ch in 0..3 {
            assert!((g[ch] - c[ch]).abs() < 0.02 * c[ch], "channel {}: GPU {} vs CPU {}", ch, g[ch], c[ch]);
        }
    }

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
        gpu.render(&s, &flat, &rc, |_| ());
        let (gpu_image, gpu_rays) = { let rc = rc.lock().unwrap(); (rc.snapshot(), rc.rays_cast) };
        let rc = Mutex::new(RenderContext::new(24, 16, false, "test.png"));
        crate::rendercontext::render(&s, &rc, Some(&crate::wavefront::integrator::Kernels::new(&s, &flat).unwrap()), |_| None::<()>, |_| ());
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
