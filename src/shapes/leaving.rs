//! The surface the rays now being traced leave: the point of the vertex they start from, set
//! by whatever shades it (`with`), so that a shape can tell a ray leaving its own surface from
//! one arriving, whatever the integrator offsets its rays by.
//!
//! A leaf card (`shapes::lod::CardSet`) stands for a cluster of leaves, and a ray leaving one
//! must be treated as leaving the cluster: it passes the cluster's other crossed cards, and
//! is shadowed by the cluster's own leaves with the probability they shadow light
//! (`Clusters::lit`). Telling such a ray by its origin alone failed, since integrators start
//! rays off a surface by different amounts: the path tracer by its shadow bias, a fraction of
//! a millimetre, the kernels by 4 millionths of the distance from the camera, which for
//! trees kilometres away is centimetres (`kernels::host::shade`). So the vertex's own point,
//! which lies on the surface, is kept here instead, and a ray is taken to leave it if it
//! starts within `reach` of it.
//!
//! Per thread: a vertex is shaded by one thread, and its rays traced by that thread before
//! the next vertex is (`trace::trace_intersection`). The kernels keep a path's vertex in its
//! state between rounds and set it again around that path's next ray (`wavefront::integrator`).
//! An instance passes it on in its model's frame (`forest::Placed`).
use crate::na::Vector3;
use crate::ray::Ray;
use std::cell::Cell;

/// A vertex rays leave: its point; how far that may be off the surface it is on, by rounding;
/// and how far from it a ray leaving it may start.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Leaving {
    pub point: Vector3<f64>,
    pub tolerance: f64,
    pub reach: f64,
}

impl Leaving {
    /// Whether `r` leaves this vertex: starts within `reach` of it.
    pub fn of(&self, r: &Ray) -> bool {
        (r.ro - self.point).norm_squared() <= self.reach * self.reach
    }
}

thread_local! {
    static FROM: Cell<Option<Leaving>> = const { Cell::new(None) };
}

/// The vertex the rays now traced on this thread leave, if one was set.
pub fn current() -> Option<Leaving> {
    FROM.with(|f| f.get())
}

/// `f`, with the rays it traces leaving `vertex` (or no vertex, for rays from a camera or a
/// medium); the vertex before is restored after.
pub fn with<R>(vertex: Option<Leaving>, f: impl FnOnce() -> R) -> R {
    let before = FROM.with(|c| c.replace(vertex));
    let out = f();
    FROM.with(|c| c.set(before));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_vertex_is_kept_for_what_it_traces_and_restored() {
        let v = Leaving { point: Vector3::new(1., 2., 3.), tolerance: 1e-6, reach: 0.01 };
        assert_eq!(current(), None);
        with(Some(v), || {
            assert_eq!(current(), Some(v));
            with(None, || assert_eq!(current(), None));
            assert_eq!(current(), Some(v));
            assert!(v.of(&Ray { ro: Vector3::new(1.005, 2., 3.), rd: Vector3::x() }));
            assert!(!v.of(&Ray { ro: Vector3::new(1.02, 2., 3.), rd: Vector3::x() }));
        });
        assert_eq!(current(), None);
    }
}
