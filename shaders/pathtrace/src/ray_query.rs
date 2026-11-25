//! Traversal by the device's ray tracing hardware, for the shader built with the `ray_query`
//! feature (`shaders/builder`; native GPUs only: WebGPU has no ray queries): what `traverse`
//! does with the BVH, done by a ray query (`SPV_KHR_ray_query`) through acceleration structures
//! the host builds from the same primitives (`gpu::rt`).
//!
//! The host builds two bottom-level structures, each its own instance in the top-level one:
//! the triangles, which the hardware intersects itself, and every other bounded primitive as
//! its box (an AABB), which the hardware only finds the ray may hit, and this code tests with
//! `intersect_prim`, as `traverse` does, committing the hit with
//! `OpRayQueryGenerateIntersectionKHR`. The instance's custom index says which it is, and a
//! table maps the primitive's index within its structure to its index in `prims`.
//!
//! The unbounded primitives (planes, the sea) are tested first, as by `traverse`, and the
//! query then searches only nearer than any hit among them.
//!
//! On the CPU this module holds the layout of the table alone, which the host writes.

/// The binding of the top-level acceleration structure in the kernels that trace.
pub const BINDING: u32 = 9;

/// The instance custom index of the triangles' structure, and of the boxes'.
pub const TRIANGLES: u32 = 0;
pub const BOXES: u32 = 1;

/// Where the map from the hardware's primitives to `prims` is in `tables`: `Params::origin.w`
/// holds its offset plus one, as bits (0: none, and the shader traces by the BVH). The table is
/// the number of triangles, then the index in `prims` of each triangle in the triangles'
/// structure, then of each box in the boxes'.
#[cfg(all(target_arch = "spirv", feature = "ray_query"))]
fn map(s: &super::Scene, instance: u32, primitive: u32) -> u32 {
    let base = s.params.origin.w.to_bits() - 1;
    let skip = if instance == BOXES { super::at(s.tables, base).to_bits() } else { 0 };
    super::at(s.tables, base + 1 + skip + primitive).to_bits()
}

/// `OpRayQueryGenerateIntersectionKHR`, which spirv-std 0.11 lacks: commit a hit at `t` with
/// the current candidate, a box.
#[cfg(all(target_arch = "spirv", feature = "ray_query"))]
unsafe fn generate_intersection(q: &spirv_std::ray_tracing::RayQuery, t: f32) {
    unsafe { core::arch::asm!("OpRayQueryGenerateIntersectionKHR {q} {t}", q = in(reg) q, t = in(reg) t) }
}

/// `traverse`, by a ray query: the nearest hit in (`tmin`, `tmax`), or with `any_hit`, the
/// first found.
#[cfg(all(target_arch = "spirv", feature = "ray_query"))]
#[inline(always)]
pub(super) fn traverse(s: &super::Scene, ro: super::Vec3, rd: super::Vec3, tmin: f32, tmax: f32, any_hit: bool) -> (u32, super::Vec4) {
    use spirv_std::ray_tracing::{CandidateIntersection, CommittedIntersection, RayFlags, RayQuery};
    use super::{intersect_prim, NONE, Vec4};

    let mut best_t = tmax;
    let mut best_prim = NONE;
    let mut best_h = Vec4::ZERO;
    // The unbounded primitives, as `traverse` tests them before the tree.
    let np = s.params.counts.x;
    let mut j = np;
    let end = np + s.params.counts.y;
    spirv_std::ray_query!(let mut q);
    let mut querying = false;
    // One loop, and one call of `intersect_prim` (inlined, the sea and all), as in `traverse`:
    // the unbounded primitives first, then the boxes the query finds.
    loop {
        let prim;
        let from_query;
        if j < end {
            prim = j;
            j += 1;
            from_query = false;
        } else {
            if !querying {
                if np == 0 {
                    break;
                }
                // Opaque: triangles are committed by the hardware, never handed back.
                let flags = if any_hit { RayFlags::OPAQUE | RayFlags::TERMINATE_ON_FIRST_HIT } else { RayFlags::OPAQUE };
                unsafe { q.initialize(s.acs, flags, 0xff, ro, tmin, rd, best_t) };
                querying = true;
            }
            if !unsafe { q.proceed() } {
                break;
            }
            if unsafe { q.get_candidate_intersection_type() } != CandidateIntersection::AABB {
                continue;
            }
            let instance: u32 = unsafe { q.get_candidate_intersection_instance_custom_index() };
            let primitive: u32 = unsafe { q.get_candidate_intersection_primitive_index() };
            prim = map(s, instance, primitive);
            from_query = true;
        }
        // Nearer than what the query has committed, a triangle or a box.
        let mut limit = best_t;
        if from_query && unsafe { q.get_committed_intersection_type() } != CommittedIntersection::None {
            let t: f32 = unsafe { q.get_committed_intersection_t() };
            if t < limit {
                limit = t;
            }
        }
        let h = intersect_prim(s, prim, ro, rd, tmin, limit);
        if h.w > 0.0 {
            if from_query {
                unsafe { generate_intersection(&q, h.x) };
            }
            best_t = h.x;
            best_prim = prim;
            best_h = h;
            if any_hit {
                if from_query {
                    unsafe { q.terminate() };
                }
                return (best_prim, best_h);
            }
        }
    }
    // A triangle the hardware committed is nearer than every hit kept above, which bounded the
    // query; a box it committed is the last of those.
    if querying && unsafe { q.get_committed_intersection_type() } == CommittedIntersection::Triangle {
        let instance: u32 = unsafe { q.get_committed_intersection_instance_custom_index() };
        let primitive: u32 = unsafe { q.get_committed_intersection_primitive_index() };
        let t: f32 = unsafe { q.get_committed_intersection_t() };
        let b: super::Vec2 = unsafe { q.get_committed_intersection_barycentrics() };
        return (map(s, instance, primitive), Vec4::new(t, b.x, b.y, 1.0));
    }
    (best_prim, best_h)
}
