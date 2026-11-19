use crate::na::Vector3;
use crate::ray::Ray;
use crate::sceneobject::SceneObject;
use std::cmp;
use std::fmt;

/// See `Intersection::spawn_offset`.
pub const RELATIVE_ERROR: f64 = 1e-7;

// TODO PBRT DifferentialGeometry
#[derive(Debug, Copy, Clone)]
pub struct RawIntersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    /// Which part of a composite object was hit, for data kept per part, such as each leaf's
    /// colour on a tree: pbrt's `faceIndex`. 0 for objects that don't number their parts.
    pub face: u32,
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
    /// See `RawIntersection::face`.
    pub face: u32,
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

    /// The texture coordinates of the point hit (`Geometry::uv`), if its shape has them.
    pub fn uv(&self) -> Option<crate::na::Vector2<f64>> {
        self.object.geometry.uv(&self.point, self.face)
    }

    /// Where to evaluate textures and masks for this hit, and the normal there: on a surface
    /// displaced at render time, the point it was displaced from (`Geometry::undisplaced`), so
    /// they line up with the relief, which was computed there; elsewhere the hit itself.
    pub fn texture_point(&self) -> crate::shapes::geometry::Undisplaced {
        self.object.geometry.undisplaced(&self.point, self.face)
            .unwrap_or(crate::shapes::geometry::Undisplaced { point: self.point, normal: self.normal, filter: 0. })
    }

    /// How far off the surface to start rays leaving this hit, so they don't hit the surface
    /// again by error in where the hit was found (Pharr, Jakob and Humphreys, "Physically Based
    /// Rendering", 4th ed., section 6.8, "Managing Rounding Error"). pbrt bounds each shape's
    /// error by careful analysis, to a few units in the last place. The shapes here aren't
    /// analysed, and some (quartics, iterative solvers) are far less precise, so the bound is a
    /// deliberately conservative relative one, `RELATIVE_ERROR` of the size of the coordinates
    /// and the distance the hit was found at (both of which an error scales with), or the shape's
    /// own `tolerance` if larger: unlike a fixed offset, it suits a scene a millimetre across and
    /// one the size of a planet alike.
    pub fn spawn_offset(&self) -> f64 {
        (RELATIVE_ERROR * (self.point.abs().max() + self.dist)).max(self.object.geometry.tolerance())
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
