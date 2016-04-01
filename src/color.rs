use na::Vec3;
use std::fmt;
use std::ops::{Mul, Add};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub rgb: Vec3<f64>
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Color {
        return Color {
            rgb: Vec3::new(r,g,b)
        }
    }

    pub fn black() -> Color {
        Color::new(0f64,0f64,0f64)
    }

    pub fn white() -> Color {
        return Color::new(1f64,1f64,1f64);
    }


    pub fn to_u8(&self) -> (u8, u8, u8) {
        return ((self.rgb[0] * 255f64) as u8, (self.rgb[1] * 255f64) as u8, (self.rgb[2] * 255f64) as u8);
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", (self.rgb.x * 255f64) as i32, (self.rgb.y * 255f64) as i32, (self.rgb.z * 255f64) as i32)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color {rgb: self.rgb * _rhs }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {rgb: self.rgb + _rhs.rgb }
    }
}
