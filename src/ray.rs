use na::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub ro: Vec3<f64>,
    pub rd: Vec3<f64>,
}



// Expanded Ray definition.
#[derive(Debug)]
pub struct RayX {
    pub ro: Vec3<f64>,
    pub rd: Vec3<f64>,

    pub time: f64,
    pub depth: i64,
}



pub struct RayDifferential {
    pub rx: Ray,
    pub ry: Ray,
}
