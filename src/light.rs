use na::Vector3;
use color::Color;

pub struct Light {
    pub position: Vector3<f64>,
    pub color: Color,
    pub intensity: f64,
}
