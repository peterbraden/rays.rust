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
    /// Which parts were given up as primitives (`flatten_parts`), for the wavefront's kernels
    /// to intersect: those `intersects_rest` and `occludes_rest` leave out. Set once, when the
    /// scene is flattened.
    flat: std::sync::OnceLock<Vec<bool>>,
}

impl Group {
    pub fn new(parts: Vec<Box<dyn Geometry + Send + Sync>>) -> Group {
        assert!(!parts.is_empty() && parts.len() <= MAX_PARTS, "a group has 1 to {} parts", MAX_PARTS);
        let bounds = parts.iter().fold(BBox::empty(), |b, p| b.union(&p.bounds()));
        Group { parts, bounds, flat: std::sync::OnceLock::new() }
    }

    /// The parts left to the group's own code: all, unless some were flattened.
    fn rest(&self) -> impl Iterator<Item = (usize, &(dyn Geometry + Send + Sync))> {
        let flat = self.flat.get();
        self.parts.iter().enumerate().filter(move |(k, _)| !flat.is_some_and(|f| f[*k])).map(|(k, p)| (k, &**p))
    }

    /// The part a face belongs to, and its face there.
    fn part(&self, face: u32) -> Option<(&(dyn Geometry + Send + Sync), u32)> {
        let k = (face >> FACE_BITS) as usize;
        self.parts.get(k).map(|p| (&**p, face & ((1 << FACE_BITS) - 1)))
    }

    fn nearest(&self, f: impl Fn(&dyn Geometry) -> Option<RawIntersection>) -> Option<RawIntersection> {
        self.nearest_of(self.parts.iter().enumerate().map(|(k, p)| (k, &**p)), f)
    }

    /// As `nearest`, of the parts `parts` (each with its place in the group).
    fn nearest_of<'a>(&self, parts: impl Iterator<Item = (usize, &'a (dyn Geometry + Send + Sync))>,
                      f: impl Fn(&dyn Geometry) -> Option<RawIntersection>) -> Option<RawIntersection> {
        let mut best: Option<RawIntersection> = None;
        for (k, p) in parts {
            if let Some(mut x) = f(p) && best.as_ref().is_none_or(|b| x.dist < b.dist) {
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

    /// The parts that `flatten` whole, as primitives, each hit reporting its part as `nearest`
    /// numbers it (and its primitive, if the part's faces are its primitives); None unless some
    /// parts can and some can't.
    fn flatten_parts(&self) -> Option<Vec<(crate::wavefront::flat::Prim, u32)>> {
        let flattened: Vec<Option<Vec<crate::wavefront::flat::Prim>>> = self.parts.iter().map(|p| p.flatten()).collect();
        let flat: Vec<bool> = flattened.iter().map(Option::is_some).collect();
        if flat.iter().all(|&f| f) || !flat.iter().any(|&f| f) {
            return None;
        }
        let _ = self.flat.set(flat);
        Some(flattened.into_iter().enumerate().flat_map(|(k, prims)| {
            let by_face = self.parts[k].faces_are_primitives();
            prims.into_iter().flatten().enumerate()
                .map(move |(i, p)| (p, (if by_face { i as u32 } else { 0 }) | ((k as u32) << FACE_BITS)))
        }).collect())
    }

    fn intersects_rest(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.nearest_of(self.rest(), |p| p.intersects_range(r, min, max))
    }

    fn occludes_rest(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.rest().any(|(_, p)| p.occludes(r, min, max))
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

    fn pattern_frame(&self, p: &Vector3<f64>, face: u32) -> Option<crate::na::Affine3<f64>> {
        self.part(face).and_then(|(g, f)| g.pattern_frame(p, f))
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

    #[test]
    fn test_the_parts_that_flatten_are_given_up_and_the_rest_kept() {
        // A sphere, which flattens, nearer than a group of one, which doesn't (as a landscape's
        // mesh and its displaced patches).
        let g = Group::new(vec![
            Box::new(Sphere::new(Vector3::new(0., 0., 5.), 1.)),
            Box::new(Group::new(vec![Box::new(Sphere::new(Vector3::new(0., 0., 10.), 1.))])),
        ]);
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::z() };
        // Until flattened, the rest is all of it.
        assert!((g.intersects_rest(&r, 0., f64::INFINITY).unwrap().dist - 4.).abs() < 1e-9);
        let parts = g.flatten_parts().expect("one part flattens");
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].1 >> FACE_BITS, 0);
        // Then the rest is the other part, hit as the group numbers it; the whole is as before.
        let rest = g.intersects_rest(&r, 0., f64::INFINITY).unwrap();
        assert!((rest.dist - 9.).abs() < 1e-9 && rest.face >> FACE_BITS == 1);
        assert!(!g.occludes_rest(&r, 0., 6.) && g.occludes_rest(&r, 0., 9.5));
        assert!((g.intersects(&r).unwrap().dist - 4.).abs() < 1e-9);
        // A group whose parts all flatten, or none, gives none up.
        let all = Group::new(vec![Box::new(Sphere::new(Vector3::new(0., 0., 5.), 1.))]);
        assert!(all.flatten_parts().is_none());
        let none = Group::new(vec![Box::new(Group::new(vec![Box::new(Sphere::new(Vector3::new(0., 0., 5.), 1.))]))]);
        assert!(none.flatten_parts().is_none());
    }
}
