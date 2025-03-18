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
        return self.intersects(r).is_some();
    }

    // Number of primitives inside geometry (ie triangles in mesh, circles etc.)
    // Used for statistics on composite geometries.
    fn primitives(&self) -> u64 { 1 }

    // Is the point inside the object? False for objects with no 'inside'
    // NB. We ignore 'ON' for points that are on the surface, as floating comparison
    // is problematic. 
    fn inside(&self, _: &Vector3<f64>) -> bool { false }
}

