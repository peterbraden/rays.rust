use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use std::f64;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;
use crate::na::geometry::{Affine3, Rotation3};

pub struct Transform {
    pub item: Box<dyn Geometry + Sync + Send>,
    pub transform: Affine3<f64>,
} 

// Affine transformation (invertible)
impl Transform {
    pub fn new(item: Box<dyn Geometry + Sync + Send>) -> Transform {
        Transform {
            item,
            transform: Affine3::identity()
        }
    }

    pub fn rotate(
        item:  Box<dyn Geometry + Sync + Send>,
        roll: f64, pitch: f64, yaw: f64
    ) -> Transform {
        Transform {
            item,
            transform: na::convert(Rotation3::from_euler_angles(roll, pitch, yaw))
        }
    } 
}

impl Geometry for Transform {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(&r.inverse_transform(&self.transform))
    }
    fn bounds(&self) -> BBox {
        self.item.bounds().transform(&na::convert(self.transform))
    }
}
