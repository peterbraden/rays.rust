//! Tracing by the device's ray tracing hardware: where the adapter has ray queries (wgpu's
//! `EXPERIMENTAL_RAY_QUERY`: Metal on Apple GPUs from macOS 15, Vulkan with
//! `VK_KHR_ray_query`), the kernels are the shader built with its `ray_query` feature
//! (`src/gpu/pathtrace-rq.wgsl`, `pathtrace::ray_query`), which find what a ray hits through
//! acceleration structures the device builds from the scene's bounded primitives, rather than
//! through the BVH (`kernels::gpu_nodes`), which they don't read.
//!
//! Two bottom-level structures, each an instance of the top-level one (`pathtrace::ray_query`'s
//! `TRIANGLES` and `BOXES`, its custom index): the triangles, which the hardware intersects,
//! and the spheres as their boxes, which the shader intersects with its own test. Positions are
//! the buffers' own (about `kernels::origin`, in f32), so the hardware tests the same triangles
//! the BVH's traversal would. A table in `tables` maps each structure's primitives back to
//! `prims` (`map_table`).
//!
//! `RAYS_GPU_BVH=1` keeps to the BVH where ray queries are available, to compare them.

use pathtrace::glam::Vec4;
use crate::kernels::GpuPrim;
use wgpu::util::DeviceExt;

/// What has to be kept for as long as the kernels trace through it: the top-level structure
/// and the compacted bottom-level ones it holds. Not the primitives they were built from,
/// which the device has no more use for.
pub(super) struct Acceleration {
    pub tlas: wgpu::Tlas,
    _blas: Vec<wgpu::Blas>,
}

/// Whether to trace by ray queries on this adapter.
pub(super) fn wanted(adapter: &wgpu::Adapter) -> bool {
    adapter.features().contains(wgpu::Features::EXPERIMENTAL_RAY_QUERY) && std::env::var_os("RAYS_GPU_BVH").is_none()
}

/// The bounded primitives' indices in `prims`: the triangles, then the rest (spheres).
fn split(prims: &[GpuPrim]) -> (Vec<u32>, Vec<u32>) {
    (0..prims.len() as u32).partition(|&i| prims[i as usize].info.x == pathtrace::PRIM_TRIANGLE)
}

/// Append the map from the structures' primitives to `prims` (the bounded ones, `prims`) to
/// `tables`, laid out as `pathtrace::ray_query` reads it, and return what `Params::origin.w`
/// holds to say where: its offset plus one, as the bits of an f32.
pub(super) fn map_table(prims: &[GpuPrim], tables: &mut Vec<f32>) -> f32 {
    let (triangles, boxes) = split(prims);
    let at = tables.len() as u32;
    tables.push(f32::from_bits(triangles.len() as u32));
    tables.extend(triangles.iter().chain(&boxes).map(|&i| f32::from_bits(i)));
    f32::from_bits(at + 1)
}

/// A sphere's box, rounded outwards to f32.
fn sphere_box(p: &GpuPrim) -> [f32; 6] {
    let (c, r) = (p.v0.truncate(), p.v0.w);
    let lo = (c - r).to_array().map(f32::next_down);
    let hi = (c + r).to_array().map(f32::next_up);
    [lo[0], lo[1], lo[2], hi[0], hi[1], hi[2]]
}

/// Build the structures over the bounded primitives `prims`, on the device, now. The
/// bottom-level structures are built, compacted, and only then gathered into the top-level
/// one: a scene of millions of primitives otherwise holds gigabytes more than the BVH, of
/// what may be a machine's shared memory.
pub(super) fn build(device: &wgpu::Device, queue: &wgpu::Queue, prims: &[GpuPrim]) -> Acceleration {
    let (triangles, boxes) = split(prims);
    let mut buffers = Vec::new();
    let flags = wgpu::AccelerationStructureFlags::PREFER_FAST_TRACE;
    let blas_flags = flags | wgpu::AccelerationStructureFlags::ALLOW_COMPACTION;
    let mut tlas = device.create_tlas(&wgpu::CreateTlasDescriptor {
        label: Some("scene"), flags, update_mode: wgpu::AccelerationStructureUpdateMode::Build, max_instances: 2,
    });
    // The instances keep the buffers' frame: no transform.
    let identity = [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.];

    let triangle_size = wgpu::BlasTriangleGeometrySizeDescriptor {
        vertex_format: wgpu::VertexFormat::Float32x3,
        vertex_count: 3 * triangles.len() as u32,
        index_format: None,
        index_count: None,
        flags: wgpu::AccelerationStructureGeometryFlags::OPAQUE,
    };
    let box_size = wgpu::BlasAABBGeometrySizeDescriptor {
        primitive_count: boxes.len() as u32,
        flags: wgpu::AccelerationStructureGeometryFlags::OPAQUE,
    };
    let triangle_blas = (!triangles.is_empty()).then(|| {
        let vertices: Vec<[f32; 3]> = triangles.iter().flat_map(|&i| {
            let p = &prims[i as usize];
            [p.v0, p.v1, p.v2].map(|v: Vec4| v.truncate().to_array())
        }).collect();
        buffers.push(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangles"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::BLAS_INPUT,
        }));
        device.create_blas(&wgpu::CreateBlasDescriptor { label: Some("triangles"), flags: blas_flags, update_mode: wgpu::AccelerationStructureUpdateMode::Build },
            wgpu::BlasGeometrySizeDescriptors::Triangles { descriptors: vec![triangle_size.clone()] })
    });
    let triangle_buffer = triangle_blas.as_ref().map(|_| buffers.len() - 1);
    let box_blas = (!boxes.is_empty()).then(|| {
        let aabbs: Vec<[f32; 6]> = boxes.iter().map(|&i| sphere_box(&prims[i as usize])).collect();
        buffers.push(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("boxes"), contents: bytemuck::cast_slice(&aabbs), usage: wgpu::BufferUsages::BLAS_INPUT,
        }));
        device.create_blas(&wgpu::CreateBlasDescriptor { label: Some("boxes"), flags: blas_flags, update_mode: wgpu::AccelerationStructureUpdateMode::Build },
            wgpu::BlasGeometrySizeDescriptors::AABBs { descriptors: vec![box_size.clone()] })
    });
    let box_buffer = box_blas.as_ref().map(|_| buffers.len() - 1);

    let mut entries = Vec::new();
    if let (Some(b), Some(i)) = (&triangle_blas, triangle_buffer) {
        entries.push(wgpu::BlasBuildEntry {
            blas: b,
            geometry: wgpu::BlasGeometries::TriangleGeometries(vec![wgpu::BlasTriangleGeometry {
                size: &triangle_size, vertex_buffer: &buffers[i], first_vertex: 0, vertex_stride: 12,
                index_buffer: None, first_index: None, transform_buffer: None, transform_buffer_offset: None,
            }]),
        });
    }
    if let (Some(b), Some(i)) = (&box_blas, box_buffer) {
        entries.push(wgpu::BlasBuildEntry {
            blas: b,
            geometry: wgpu::BlasGeometries::AabbGeometries(vec![wgpu::BlasAabbGeometry {
                size: &box_size, stride: 24, aabb_buffer: &buffers[i], primitive_offset: 0,
            }]),
        });
    }
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("bottom-level structures") });
    encoder.build_acceleration_structures(entries.iter(), std::iter::empty());
    queue.submit(Some(encoder.finish()));
    drop(entries);
    // The inputs are wgpu's to free once the build is done, as is its scratch space.
    drop(buffers);

    // Each structure copied to the size it turned out to need; the originals freed.
    let built: Vec<(wgpu::Blas, u32)> = [(triangle_blas, pathtrace::ray_query::TRIANGLES), (box_blas, pathtrace::ray_query::BOXES)]
        .into_iter().filter_map(|(b, kind)| Some((b?, kind))).collect();
    for (b, _) in &built {
        b.prepare_compaction_async(|_| {});
    }
    device.poll(wgpu::PollType::wait_indefinitely()).expect("the bottom-level structures built");
    let blas: Vec<(wgpu::Blas, u32)> = built.into_iter()
        .map(|(b, kind)| (if b.ready_for_compaction() { queue.compact_blas(&b) } else { b }, kind))
        .collect();

    for (b, kind) in &blas {
        tlas[*kind as usize] = Some(wgpu::TlasInstance::new(b, identity, *kind, 0xff));
    }
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("top-level structure") });
    encoder.build_acceleration_structures(std::iter::empty(), std::iter::once(&tlas));
    queue.submit(Some(encoder.finish()));
    Acceleration { tlas, _blas: blas.into_iter().map(|(b, _)| b).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    #[test]
    fn test_the_map_lists_triangles_then_spheres() {
        let mut prims = vec![GpuPrim::zeroed(); 5];
        for i in [1, 3] {
            prims[i].info.x = pathtrace::PRIM_SPHERE;
        }
        let mut tables = vec![7.; 3];
        let w = map_table(&prims, &mut tables);
        assert_eq!(w.to_bits(), 4);
        let read: Vec<u32> = tables[3..].iter().map(|f| f.to_bits()).collect();
        assert_eq!(read, [3, 0, 2, 4, 1, 3]);
    }

    #[test]
    fn test_a_spheres_box_holds_it() {
        let mut p = GpuPrim::zeroed();
        p.v0 = Vec4::new(1e3 + 0.1, -2.5, 0.3, 0.7);
        let b = sphere_box(&p);
        assert!(b[0] <= 1e3 + 0.1 - 0.7 && b[3] >= 1e3 + 0.1 + 0.7);
        assert!(b[1] < -3.2 && b[4] > -1.8);
    }
}
