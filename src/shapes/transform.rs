use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use std::f64;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;
use crate::na::geometry::{Affine3, Point3, Rotation3};

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
        // Intersect in object space, then map the hit back into world space.
        self.item.intersects(&r.inverse_transform(&self.transform)).map(|i| {
            let point = self.transform.transform_point(&Point3::from(i.point)).coords;
            // Normals transform by the inverse transpose of the linear part.
            let normal_matrix = self.transform.inverse().matrix().fixed_view::<3, 3>(0, 0).transpose();
            RawIntersection {
                dist: (point - r.ro).norm(),
                point,
                normal: (normal_matrix * i.normal).normalize(),
            }
        })
    }
    fn bounds(&self) -> BBox {
        self.item.bounds().transform(&na::convert(self.transform))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn test_rotated_hit_is_in_world_space() {
        // A slab that is thin in x; rotating 90 degrees about y makes it thin in z.
        let slab = BBox::new(Vector3::new(-0.1, -1., -2.), Vector3::new(0.1, 1., 2.));
        let t = Transform::rotate(Box::new(slab), 0., FRAC_PI_2, 0.);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        let i = t.intersects(&r).unwrap();
        assert!((i.dist - 4.9).abs() < 1e-9, "dist {}", i.dist);
        assert!((i.point - Vector3::new(0., 0., -0.1)).norm() < 1e-9, "point {}", i.point);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-9, "normal {}", i.normal);
    }
}
