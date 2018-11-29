use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;

pub trait Geometry: Sync{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;

    // World space bounding box
    fn bounds(&self) -> BBox;

    // Number of primitives (ie triangles in mesh
    fn primitives(&self) -> u64 { 1 }
}
