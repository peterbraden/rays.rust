use color::Color;

pub struct Material {
    pub pigment: Color
}

impl Material {

    pub fn demo() -> Material {
        Material {
            pigment: Color::new(0.5f64, 0.5f64, 0.5f64)
        }
    }
}
