use crate::na::Vector3;
use crate::ray::Ray;
use crate::sceneobject::SceneObject;
use std::cmp;
use std::fmt;

// TODO PBRT DifferentialGeometry
#[derive(Debug, Copy, Clone)]
pub struct RawIntersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    /// Which part of a combined shape was hit, for a material of its own: 0 for none (the
    /// object's own material). See `csg::Tagged`.
    pub part: u32,
    // TODO error : rayEpsilon for float errors
}

/// A hit on a scene object. It borrows the object from the scene rather than holding a counted
/// reference: with every thread tracing against the same few objects, counting each hit cost an
/// atomic increment and decrement on a shared cache line, contended across cores.
#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub object: &'a SceneObject,
    /// As for `RawIntersection`.
    pub part: u32,
}

/*
pub struct MediumIntersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub inside: Box<Medium + Sync + Send>,
    pub outside: Box<Medium + Sync + Send>,
}
*/

impl<'a> Intersection<'a> {
    /// The normal on the side of the surface the ray arrived from. Geometry reports outward
    /// normals, which dielectrics need to tell entering from leaving; opaque materials shade
    /// both sides alike, so they use this, eg. for the inside of a sphere.
    pub fn facing_normal(&self, r: &Ray) -> Vector3<f64> {
        if r.rd.dot(&self.normal) > 0. { -self.normal } else { self.normal }
    }

    /// This intersection with its normal facing the incoming ray (see `facing_normal`).
    pub fn facing(&self, r: &Ray) -> Intersection<'a> {
        Intersection { normal: self.facing_normal(r), ..*self }
    }
}

impl cmp::PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.point == other.point
    }
}

impl fmt::Display for Intersection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Intersection p:{} d:{} n:{})", self.point, self.dist, self.normal)
    }
}
