//! # Motion blur
//!
//! A real camera's shutter is open for a while, and whatever moves meanwhile smears across the
//! image. Each camera sample is given a time, uniformly within the shutter interval (`Shutter`),
//! and the scene is seen as it is at that time: the camera's pose and moving objects' transforms
//! are interpolated to it. Averaging over samples integrates over the shutter's opening (Pharr,
//! Jakob and Humphreys, "Physically Based Rendering", 4th ed., section 5.1.1 "Time"; cited below
//! as PBR).
//!
//! In pbrt every ray carries its time, inherited by the rays spawned from it. Here a sample's
//! time is per-thread state for the duration of the sample (`begin_sample` to `end_sample`), as
//! its wavelengths are (`spectrum`): every ray of a path shares it either way, and this spares a
//! field on each of the renderer's many `Ray`s.
//!
//! ## Interpolating transformations (pbrt's `AnimatedTransform`; PBR 3rd ed., section 2.9,
//! "Animating Transformations")
//!
//! Interpolating two matrices entry by entry doesn't interpolate a rotation: half way between a
//! turn of 0° and of 180° it collapses the object to a line. Instead each affine map is
//! decomposed into a translation T, a rotation R and a stretch S (symmetric), M = T R S, and
//! these are interpolated separately: T and S linearly, R by spherical linear interpolation of
//! its quaternion. The rotation is found by polar decomposition, averaging the linear part with
//! its inverse transpose until it converges to an orthogonal matrix (Ken Shoemake and Tom Duff,
//! "Matrix Animation and Polar Decomposition", Graphics Interface 1992, pp. 258-264).
//!
//! The moving object's bounds must hold it at every time. They are the union of its bounds at
//! many times through the motion, widened by how far its corners can stray from the chords
//! between them on the arcs a rotation sweeps (pbrt instead bounds the motion analytically).
use crate::intersection::RawIntersection;
use crate::na::{Affine3, Matrix3, Matrix4, Rotation3, Translation3, UnitQuaternion, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use crate::shapes::transform::Transform;
use std::cell::Cell;
use std::sync::Arc;

thread_local! {
    /// The time of the sample this thread is tracing.
    static TIME: Cell<f64> = const { Cell::new(0.) };
}

/// Start tracing a sample at time `t`.
pub fn begin_sample(t: f64) {
    TIME.with(|c| c.set(t));
}

/// Back to time zero, the scene's resting pose.
pub fn end_sample() {
    TIME.with(|c| c.set(0.));
}

/// The time of the sample being traced: 0 at the start of motion, 1 at its end.
pub fn time() -> f64 {
    TIME.with(|c| c.get())
}

/// When the shutter is open, in the time of motion: things move from their start (time 0) to
/// their end (time 1). The default opens it across the whole motion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shutter {
    pub open: f64,
    pub close: f64,
}

impl Default for Shutter {
    fn default() -> Shutter {
        Shutter { open: 0., close: 1. }
    }
}

impl Shutter {
    /// The time for a uniform `u`.
    pub fn time(&self, u: f64) -> f64 {
        self.open + (self.close - self.open) * u
    }
}

/// An affine map as translation, rotation and stretch (see the module's notes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decomposed {
    pub translation: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
    pub stretch: Matrix3<f64>,
}

impl Decomposed {
    /// Decompose `m`, whose linear part must be invertible.
    pub fn new(m: &Affine3<f64>) -> Decomposed {
        let mat = m.matrix();
        let translation = Vector3::new(mat[(0, 3)], mat[(1, 3)], mat[(2, 3)]);
        let linear: Matrix3<f64> = mat.fixed_view::<3, 3>(0, 0).into();
        // Polar decomposition: R = lim (R + R^-T) / 2.
        let mut r = linear;
        for _ in 0..100 {
            let inv_t = r.try_inverse().expect("a transform's linear part must be invertible").transpose();
            let next = (r + inv_t) * 0.5;
            let change = (next - r).abs().max();
            r = next;
            if change < 1e-12 {
                break;
            }
        }
        // A reflection: keep R a rotation, and leave the reflection in the stretch.
        if r.determinant() < 0. {
            r = -r;
        }
        let stretch = r.transpose() * linear;
        let rotation = UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix_unchecked(r));
        Decomposed { translation, rotation, stretch }
    }

    /// The map itself again.
    pub fn compose(&self) -> Affine3<f64> {
        let mut linear = Matrix4::identity();
        linear.fixed_view_mut::<3, 3>(0, 0).copy_from(&(self.rotation.to_rotation_matrix().matrix() * self.stretch));
        Affine3::from_matrix_unchecked(Translation3::from(self.translation).to_homogeneous() * linear)
    }

    /// Part way, `t` in [0, 1], from this to `end`.
    pub fn interpolate(&self, end: &Decomposed, t: f64) -> Decomposed {
        let rotation = self.rotation.try_slerp(&end.rotation, t, 1e-12).unwrap_or(if t < 0.5 { self.rotation } else { end.rotation });
        Decomposed {
            translation: self.translation.lerp(&end.translation, t),
            rotation,
            stretch: self.stretch + (end.stretch - self.stretch) * t,
        }
    }
}

/// An object moving from one transform (at time 0) to another (at time 1).
pub struct AnimatedTransform {
    item: Arc<dyn Geometry + Sync + Send>,
    start: Decomposed,
    end: Decomposed,
    bounds: BBox,
}

/// Times at which the motion's bounds are taken (see the module's notes).
const BOUND_STEPS: usize = 64;

impl AnimatedTransform {
    pub fn new(item: impl Into<Arc<dyn Geometry + Sync + Send>>, start: Affine3<f64>, end: Affine3<f64>) -> AnimatedTransform {
        let item = item.into();
        let (start, end) = (Decomposed::new(&start), Decomposed::new(&end));
        let mut bounds: Option<BBox> = None;
        for i in 0..=BOUND_STEPS {
            let b = Transform::new(item.clone(), start.interpolate(&end, i as f64 / BOUND_STEPS as f64).compose()).bounds();
            bounds = Some(match bounds { None => b, Some(a) => a.union(&b) });
        }
        let mut bounds = bounds.unwrap();
        // A point at distance d from the axis, turning by θ per step, strays from the chord by
        // at most d (1 - cos(θ/2)); d is at most the radius of the bounds about their centre.
        let angle = start.rotation.angle_to(&end.rotation) / BOUND_STEPS as f64;
        let radius = (bounds.max - bounds.min).norm() / 2.;
        let pad = radius * (1. - (angle / 2.).cos()) + 1e-9 * radius;
        if pad.is_finite() && radius.is_finite() {
            bounds = BBox::new(bounds.min.coords - Vector3::repeat(pad), bounds.max.coords + Vector3::repeat(pad));
        }
        AnimatedTransform { item, start, end, bounds }
    }

    /// The object as it is at the current sample's time.
    fn now(&self) -> Transform {
        Transform::new(self.item.clone(), self.start.interpolate(&self.end, time()).compose())
    }
}

impl Geometry for AnimatedTransform {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.now().intersects(r)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.now().intersects_beyond(r, min)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.now().intersects_range(r, min, max)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.now().occludes(r, min, max)
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.now().crossings(r)
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.now().inside(p)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }

    fn is_stochastic(&self) -> bool {
        self.item.is_stochastic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;

    #[test]
    fn test_decomposition_recomposes() {
        let m = Affine3::from_matrix_unchecked(
            Translation3::new(1., -2., 3.).to_homogeneous()
            * Rotation3::from_euler_angles(0.3, -1.1, 2.).to_homogeneous()
            * Matrix4::new_nonuniform_scaling(&Vector3::new(2., 0.5, 3.)));
        let d = Decomposed::new(&m);
        assert!((d.compose().matrix() - m.matrix()).abs().max() < 1e-9);
        assert!((d.stretch - d.stretch.transpose()).abs().max() < 1e-9, "stretch not symmetric: {}", d.stretch);
    }

    #[test]
    fn test_rotation_is_interpolated_on_the_sphere() {
        // Half way through a quarter turn about y, a point on x is at 45°, still at unit distance.
        let start = Decomposed::new(&Affine3::identity());
        let end = Decomposed::new(&Affine3::from_matrix_unchecked(Rotation3::from_euler_angles(0., std::f64::consts::FRAC_PI_2, 0.).to_homogeneous()));
        let half = start.interpolate(&end, 0.5).compose();
        let p = half.transform_point(&crate::na::Point3::new(1., 0., 0.));
        assert!((p.coords.norm() - 1.).abs() < 1e-9 && (p.x - p.z.abs()).abs() < 1e-9, "{}", p);
    }

    #[test]
    fn test_moving_sphere_is_where_the_time_puts_it() {
        let sphere: Arc<dyn Geometry + Sync + Send> = Arc::new(Sphere::new(Vector3::zeros(), 1.));
        let moving = AnimatedTransform::new(sphere, Affine3::identity(),
            Affine3::from_matrix_unchecked(Translation3::new(10., 0., 0.).to_homogeneous()));
        // The bounds cover the whole path.
        let b = moving.bounds();
        assert!(b.min.x <= -1. && b.max.x >= 11., "{:?}", b);
        let down = |x: f64| Ray { ro: Vector3::new(x, 5., 0.), rd: Vector3::new(0., -1., 0.) };
        begin_sample(0.);
        assert!(moving.intersects(&down(0.)).is_some() && moving.intersects(&down(10.)).is_none());
        begin_sample(1.);
        assert!(moving.intersects(&down(0.)).is_none() && moving.intersects(&down(10.)).is_some());
        begin_sample(0.5);
        assert!(moving.intersects(&down(5.)).is_some());
        end_sample();
    }

    #[test]
    fn test_turning_bounds_hold_the_object() {
        // A long box turning half way round: every intermediate pose is inside the bounds.
        let bar: Arc<dyn Geometry + Sync + Send> = Arc::new(crate::shapes::bbox::BBox::new(Vector3::new(-5., -0.1, -0.1), Vector3::new(5., 0.1, 0.1)));
        let end = Affine3::from_matrix_unchecked(Rotation3::from_euler_angles(0., 0., 3.).to_homogeneous());
        let moving = AnimatedTransform::new(bar.clone(), Affine3::identity(), end);
        let b = moving.bounds();
        for i in 0..=100 {
            let t = i as f64 / 100.;
            let pose = Transform::new(bar.clone(), moving.start.interpolate(&moving.end, t).compose()).bounds();
            assert!(pose.min.x >= b.min.x - 1e-9 && pose.max.x <= b.max.x + 1e-9 && pose.min.y >= b.min.y - 1e-9 && pose.max.y <= b.max.y + 1e-9,
                "at {}: {:?} outside {:?}", t, pose, b);
        }
    }
}
