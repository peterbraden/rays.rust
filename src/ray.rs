use na::{Vector3, Point3};
use na::geometry::{Transform3, Affine3};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub ro: Vector3<f64>,
    pub rd: Vector3<f64>,
}

// Expanded Ray definition.
#[derive(Debug)]
pub struct RayX {
    pub ro: Vector3<f64>,
    pub rd: Vector3<f64>,

    pub time: f64,
    pub depth: i64,
}

pub struct RayDifferential {
    pub rx: Ray,
    pub ry: Ray,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Ray {}->{})", self.ro, self.rd)
    }
}

impl Ray {
    pub fn transform(&self, t: &Transform3<f64>) -> Ray {
        return Ray {
            ro: (t * Point3::from(self.ro)).coords,
            rd: t * self.rd
        }; 
    }

    pub fn inverse_transform(&self, t: &Affine3<f64>) -> Ray {
        return Ray {
            ro: t.inverse_transform_point(&Point3::from(self.ro)).coords,
            rd: t.inverse_transform_vector(&self.rd)
        }; 
    }
}
