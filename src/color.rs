use crate::na::Vector3;
use std::fmt;
use std::ops::{Mul, Add, Div};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub rgb: Vector3<f64>
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Color {
        Color {
            rgb: Vector3::new(r,g,b)
        }
    }

    pub fn black() -> Color {
        Color::new(0f64,0f64,0f64)
    }

    pub fn white() -> Color {
        Color::new(1f64,1f64,1f64)
    }
    pub fn red() -> Color {
        Color::new(1f64,0f64,0f64)
    }
    pub fn blue() -> Color {
        Color::new(0f64,0f64,01f64)
    }
    pub fn green() -> Color {
        Color::new(0f64,1f64,0f64)
    }

    pub fn as_u8(&self) -> (u8, u8, u8) {
        ((self.rgb[0] * 255f64).min(255f64) as u8, (self.rgb[1] * 255f64).min(255f64) as u8, (self.rgb[2] * 255f64).min(255f64) as u8)
    }

    pub fn as_vec(&self) -> Vector3<f64> {
        self.rgb
    }

	pub fn clamp(&self, val: f64) -> Color {
        Color::new(self.rgb.x.min(val), self.rgb.y.min(val), self.rgb.z.min(val))
	}

    pub fn min() -> Color {
        Color::new(1./255.,1./255.,1./255.)
    }
    
    pub fn ignore_nan(&self) -> Color {
        Color::new(
            if self.rgb.x.is_nan() { 0. } else { self.rgb.x },
            if self.rgb.y.is_nan() { 0. } else { self.rgb.y },
            if self.rgb.z.is_nan() { 0. } else { self.rgb.z },
        )
    }
    
    /// Blend this color with another color using the given factor
    /// 
    /// # Arguments
    /// * `other` - The color to blend with
    /// * `factor` - The blend factor (0.0 = this color only, 1.0 = other color only)
    /// 
    /// # Returns
    /// A new color that is a blend of this color and the other color
    pub fn blend(&self, other: &Color, factor: f64) -> Color {
        let clamped_factor = factor.clamp(0.0, 1.0);
        let self_factor = 1.0 - clamped_factor;
        
        Color::new(
            self.rgb.x * self_factor + other.rgb.x * clamped_factor,
            self.rgb.y * self_factor + other.rgb.y * clamped_factor,
            self.rgb.z * self_factor + other.rgb.z * clamped_factor,
        )
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", (self.rgb.x * 255f64) as u8, (self.rgb.y * 255f64) as u8, (self.rgb.z * 255f64) as u8)
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

impl Add<Vector3<f64>> for Color {
    type Output = Color;

    fn add(self, _rhs: Vector3<f64>) -> Color {
        Color {
            rgb: _rhs + self.rgb
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, _rhs: f64) -> Color {
        Color {rgb: self.rgb / _rhs }
    }
}
