use na::Vector3;
use std::fmt;

#[derive(Debug)]
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
