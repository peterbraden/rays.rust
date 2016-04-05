use color::Color;

pub struct Material {
    pub pigment: Color,
    pub reflection: f64,
    pub phong: f64,
}

impl Material {

    pub fn demo() -> Material {
        Material {
            pigment: Color::new(0.5f64, 0.5f64, 0.5f64),
            reflection: 0.2,
            phong: 40f64,
        }
    }
}
