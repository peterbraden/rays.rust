use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use octree::Octree;
use std::f64;
use shapes::geometry::Geometry;
use std::sync::Arc;
use na::geometry::{Affine3, Rotation3};

pub struct Transform {
    pub item: Box<dyn Geometry + Sync + Send>,
    pub transform: Affine3<f64>,
} 

// Affine transformation (invertible)
impl Transform {
    pub fn new(item: Box<dyn Geometry + Sync + Send>) -> Transform {
        return Transform {
            item: item,
            transform: Affine3::identity()
        };
    }

    pub fn rotate(
        item:  Box<dyn Geometry + Sync + Send>,
        roll: f64, pitch: f64, yaw: f64
    ) -> Transform {
        return Transform {
            item: item,
            transform: na::convert(Rotation3::from_euler_angles(roll, pitch, yaw))
        }
    } 
}

impl Geometry for Transform {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.item.intersects(&r.inverse_transform(&self.transform));
    }
    fn bounds(&self) -> BBox {
        return self.item.bounds().transform(&na::convert(self.transform));
    }
}
