//! Several shapes as one, for an object whose parts share a material: a landscape's mesh and
//! the displaced patch about its camera (`terrain::scenery`).
//!
//! Each hit's `face` carries which part was hit in its top bits, so what a part numbers by face
//! (texture coordinates, where a displaced point came from) still reaches it.

use crate::intersection::RawIntersection;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::{Geometry, Undisplaced};

/// Bits of `face` left to each part: parts may number up to 2^28 faces.
const FACE_BITS: u32 = 28;
const MAX_PARTS: usize = 1 << (32 - FACE_BITS);

pub struct Group {
    parts: Vec<Box<dyn Geometry + Send + Sync>>,
    bounds: BBox,
}

impl Group {
    pub fn new(parts: Vec<Box<dyn Geometry + Send + Sync>>) -> Group {
        assert!(!parts.is_empty() && parts.len() <= MAX_PARTS, "a group has 1 to {} parts", MAX_PARTS);
        let bounds = parts.iter().fold(BBox::empty(), |b, p| b.union(&p.bounds()));
        Group { parts, bounds }
    }

    /// The part a face belongs to, and its face there.
    fn part(&self, face: u32) -> Option<(&(dyn Geometry + Send + Sync), u32)> {
        let k = (face >> FACE_BITS) as usize;
        self.parts.get(k).map(|p| (&**p, face & ((1 << FACE_BITS) - 1)))
    }

    fn nearest(&self, f: impl Fn(&dyn Geometry) -> Option<RawIntersection>) -> Option<RawIntersection> {
        let mut best: Option<RawIntersection> = None;
        for (k, p) in self.parts.iter().enumerate() {
            if let Some(mut x) = f(&**p) && best.as_ref().is_none_or(|b| x.dist < b.dist) {
                debug_assert!(x.face >> FACE_BITS == 0, "a group's part numbers more faces than it can carry");
                x.face = (x.face & ((1 << FACE_BITS) - 1)) | ((k as u32) << FACE_BITS);
                best = Some(x);
            }
        }
        best
    }
}

impl Geometry for Group {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.nearest(|p| p.intersects(r))
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.nearest(|p| p.intersects_beyond(r, min))
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.nearest(|p| p.intersects_range(r, min, max))
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.parts.iter().any(|p| p.occludes(r, min, max))
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.parts.iter().map(|p| p.primitives()).sum()
    }

    fn uv(&self, p: &Vector3<f64>, face: u32) -> Option<Vector2<f64>> {
        self.part(face).and_then(|(g, f)| g.uv(p, f))
    }

    fn undisplaced(&self, p: &Vector3<f64>, face: u32) -> Option<Undisplaced> {
        self.part(face).and_then(|(g, f)| g.undisplaced(p, f))
    }

    fn tangent(&self, p: &Vector3<f64>, face: u32) -> Option<Vector3<f64>> {
        self.part(face).and_then(|(g, f)| g.tangent(p, f))
    }

    fn tolerance(&self) -> f64 {
        self.parts.iter().map(|p| p.tolerance()).fold(0., f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;

    #[test]
    fn test_nearest_part_and_its_face() {
        let g = Group::new(vec![
            Box::new(Sphere::new(Vector3::new(0., 0., 10.), 1.)),
            Box::new(Sphere::new(Vector3::new(0., 0., 5.), 1.)),
        ]);
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::z() };
        let hit = g.intersects(&r).unwrap();
        assert!((hit.dist - 4.).abs() < 1e-9);
        assert_eq!(hit.face >> FACE_BITS, 1);
        assert!(g.occludes(&r, 0., 4.5));
        assert!(!g.occludes(&r, 0., 3.5));
        let far = g.intersects_beyond(&r, 7.).unwrap();
        assert!((far.dist - 9.).abs() < 1e-9);
        assert_eq!(far.face >> FACE_BITS, 0);
    }
}
