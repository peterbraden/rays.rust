use color::Color;
use na::Vec3;

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
            phong: 80f64,
        }
    }


    pub fn checker_demo(pt: Vec3<f64>, xsize: f64, zsize: f64) -> Material {
        
        let zig = if (pt[0].abs() / xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
         // zig XOR zag
        let col = if !zig != !zag { Color::black() } else { Color::white() };
    
        return Material {
            pigment: col,
            reflection: 0.2,
            phong: 0.,
        }
    }
}
