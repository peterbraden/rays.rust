use crate::na::Vector3;
use crate::color::Color;

pub struct Light {
    pub position: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}
