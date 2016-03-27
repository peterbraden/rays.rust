use na::Vec3;
use std::fmt;

#[derive(Debug)]
pub struct Color {
    pub rgb: Vec3<i32>
}

impl Color {
    pub fn new(r:i32, g:i32, b:i32) -> Color {
        return Color {
            rgb: Vec3::new(r,g,b)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.rgb.x, self.rgb.y, self.rgb.z)
    }
}
