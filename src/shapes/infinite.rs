use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

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
