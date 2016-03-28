use na::Vec3;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub rgb: Vec3<i32>
}

impl Color {
    pub fn new(r:i32, g:i32, b:i32) -> Color {
        return Color {
            rgb: Vec3::new(r,g,b)
        }
    }

    pub fn black() -> Color {
        Color::new(0,0,0)
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.rgb.x, self.rgb.y, self.rgb.z)
    }
}
