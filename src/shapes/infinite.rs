use na::{Vector3};
use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use shapes::geometry::Geometry;

pub struct Infinite {}

impl Geometry for Infinite{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return Some(RawIntersection {
            dist: std::f64::MAX,
            point: r.ro + r.rd * std::f64::MAX,
            normal: r.rd * -1.
        })
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN),
            Vector3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX),
          )
    }
}
