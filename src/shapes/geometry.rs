use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::na::{Vector3};


// 
// - Roughly equivalent to the Shape class in PBRT
// - We don't include transforms in geometry, preferring to compose as:
//      Translate(X) is easier than adding x.translate to all geometries.
// - The simpler we can keep this interface, the easier it is to add new shapes.
// - For extensions to allow CSG, see 'Primitive' which is a wrapper object.
//
// - All shapes must allow intersection (PBRT has a weird canIntersect thing)
pub trait Geometry: Sync{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;

    // Nearest intersection at least `min` along the ray. A composite (eg. a mesh) reports only
    // its nearest hit, so if that is nearer than `min` - typically the surface a shadow ray
    // leaves from - search again from `min` rather than discarding the whole object.
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        match self.intersects(r) {
            Some(x) if x.dist < min => {
                let advanced = Ray { ro: r.ro + r.rd.normalize() * min, rd: r.rd };
                self.intersects(&advanced).map(|mut x| { x.dist += min; x })
            },
            x => x,
        }
    }

    // World space bounding box
    fn bounds(&self) -> BBox;


    // === Optional methods ===
    // - We provide defaults to keep implementation burden low.
    
    // Equivalent of intersectsP in PBRT. If there is a fast way
    // to calculate intersection, override, otherwise fall back to intersects() call.
    // - NB. Callers should be aware that calling fast_intersects(), then intersects() is slower
    // than just calling intersects. This is only really for cases where you want to avoid
    // allocation of a RawIntersection object.
    fn fast_intersects(&self, r: &Ray) -> bool {
        self.intersects(r).is_some()
    }

    // Number of primitives inside geometry (ie triangles in mesh, circles etc.)
    // Used for statistics on composite geometries.
    fn primitives(&self) -> u64 { 1 }

    // Is the point inside the object? False for objects with no 'inside'
    // NB. We ignore 'ON' for points that are on the surface, as floating comparison
    // is problematic. 
    fn inside(&self, _: &Vector3<f64>) -> bool { false }

    // Surface area, for shapes that can be sampled as area lights; 0 for those that can't.
    fn surface_area(&self) -> f64 { 0. }

    // A point uniformly distributed over the surface by area, and the surface normal there,
    // from three uniform random numbers in [0, 1). None for shapes that can't be sampled.
    fn sample_surface(&self, _u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { None }
}


/// A geometry whose hits report `face`, so a composite object made of many of them (eg. the
/// triangles of a tree's leaves) can say which part was hit; see `RawIntersection::face`.
pub struct Tagged<T: Geometry> {
    pub item: T,
    pub face: u32,
}

impl<T: Geometry> Geometry for Tagged<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(r).map(|i| RawIntersection { face: self.face, ..i })
    }

    fn bounds(&self) -> BBox {
        self.item.bounds()
    }

    fn surface_area(&self) -> f64 {
        self.item.surface_area()
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        self.item.sample_surface(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::triangle::Triangle;
    use crate::octree::Octree;
    use std::sync::Arc;

    #[test]
    fn test_tagged_hits_report_their_face() {
        // Two parallel triangles, the nearer tagged 7 and the farther 3, in an octree.
        let at = |z: f64, face: u32| Arc::new(Tagged {
            item: Triangle::new(Vector3::new(-1., -1., z), Vector3::new(1., -1., z), Vector3::new(0., 1., z)),
            face,
        });
        let items = vec![at(2., 3), at(1., 7)];
        let bounds = items.iter().fold(BBox::empty(), |b, t| b.union(&t.bounds()));
        let tree = Octree::new(8, bounds, &items);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert_eq!(tree.raw_intersection(&r, f64::INFINITY, 0.).unwrap().face, 7);
        let back = Ray { ro: Vector3::new(0., 0., 5.), rd: Vector3::new(0., 0., -1.) };
        assert_eq!(tree.raw_intersection(&back, f64::INFINITY, 0.).unwrap().face, 3);
    }
}
