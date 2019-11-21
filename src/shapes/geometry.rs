use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;


// 
// - Roughly equivalent to the Shape class in PBRT
// - We don't include transforms in geometry, preferring to compose as:
//      Translate(X) is easier than adding x.translate to all geometries.
// - The simpler we can keep this interface, the easier it is to add new shapes.
// - For extensions to allow CSG, see 'Primitive' which is a wrapper object.
//
// - All shapes must allow intersection (PBRT has a weird canIntersect thing)
// - For now we don't have a bool intersects (PBRT has intersectsP) TODO Add 
pub trait Geometry: Sync{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;

    // World space bounding box
    fn bounds(&self) -> BBox;

    // Number of primitives (ie triangles in mesh
    fn primitives(&self) -> u64 { 1 }
}

