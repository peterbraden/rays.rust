use ray::Ray;
use intersection::RawIntersection;
use bbox::BBox;

pub trait Geometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;

    // World space bounding box
    fn bounds(&self) -> BBox;
}
