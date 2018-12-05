use na::Vector3;
use std::fmt;
use std::ops::{Mul, Add, Div};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub rgb: Vector3<f64>
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Color {
        return Color {
            rgb: Vector3::new(r,g,b)
        }
    }

    pub fn black() -> Color {
        Color::new(0f64,0f64,0f64)
    }

    pub fn white() -> Color {
        return Color::new(1f64,1f64,1f64);
    }
    pub fn red() -> Color {
        return Color::new(1f64,0f64,0f64);
    }
    pub fn blue() -> Color {
        return Color::new(0f64,0f64,01f64);
    }
    pub fn green() -> Color {
        return Color::new(0f64,1f64,0f64);
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        return ((self.rgb[0] * 255f64).min(255f64) as u8, (self.rgb[1] * 255f64).min(255f64) as u8, (self.rgb[2] * 255f64).min(255f64) as u8);
    }

    pub fn to_vec(&self) -> Vector3<f64> {
        return self.rgb.clone();
    }

    pub fn min() -> Color {
        return Color::new(1./255.,1./255.,1./255.);
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", (self.rgb.x * 255f64) as i32, (self.rgb.y * 255f64) as i32, (self.rgb.z * 255f64) as i32)
    }
}

impl Mul<Vector3<f64>> for Color {
    type Output = Color;

    fn mul(self, _rhs: Vector3<f64>) -> Color {
        Color {
            rgb: _rhs.component_mul(&self.rgb)
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {rgb: (_rhs * self.to_vec()).to_vec() }
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

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, _rhs: f64) -> Color {
        Color {rgb: self.rgb / _rhs }
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        if self.rgb[0] < other.rgb[0] &&
           self.rgb[1] < other.rgb[1] &&
           self.rgb[2] < other.rgb[2]{
            return Some(Ordering::Less);
        }
        if self.rgb[0] > other.rgb[0] &&
           self.rgb[1] > other.rgb[1] &&
           self.rgb[2] > other.rgb[2] {
            return Some(Ordering::Greater);
        }
        return None;
    }
}
