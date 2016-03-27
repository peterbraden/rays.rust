use na::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub ro: Vec3<f64>,
    pub rd: Vec3<f64>,
}
