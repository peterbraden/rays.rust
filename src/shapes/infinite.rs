use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

pub struct Infinite {}

impl Geometry for Infinite{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        Some(RawIntersection {
            dist: f64::MAX,
            point: r.ro + r.rd * f64::MAX,
            normal: r.rd * -1.
        })
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, f64::MIN, f64::MIN),
            Vector3::new(f64::MAX, f64::MAX, f64::MAX),
          )
    }
}
