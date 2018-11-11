use ray::Ray;
use intersection::Intersection;
use bbox::BBox;

pub trait Geometry {
    fn intersects(&self, r: &Ray) -> Option<Intersection>;

    // World space bounding box
    fn bounds(&self) -> BBox;
}
