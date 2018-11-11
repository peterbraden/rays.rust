use color::Color;
use na::Vec3;
use rand;
///
/// See https://google.github.io/filament//Materials.md.html#materialmodels/litmodel

pub struct Material {
    pub pigment: Color, // Attenuation due to albedo
    pub metallic: f64, // Dielectric 0 to Metallic 1
    pub roughness: f64, // Glossiness
    pub reflection: f64,
    pub phong: f64,
    pub normal_peturbation: Vec3<f64>,
}

pub trait Medium {
    fn material_at(pt: Vec3<f64>) -> Material; 
}

pub struct CheckeredYPlane {
    m1: Material,
    m2: Material,
    xsize: f64,
    zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: Material, m2: Material, xsize: f64, zsize: f64) -> Medium {
        Medium { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(pt: Vec3<f64>) -> Material {
        let zig = if (pt[0].abs() / xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        return (if !zig != !zag { m1 } else { m2 });
    }
}

const POLISHED_COPPER: Material = Material{
    pigment: Color::new(0.97, 0.74, 0.62),
    metallic: 1.,
    roughness: 0.01,
    reflection: 0.7,
    phong: 0.01,
    normal_peturbation: Vec3::new( 0., 0., 0.)
};

const WHITE_MARBLE: Material = Material{
    pigment: Color::new(0.9, 0.9, 0.9),
    metallic: 0.1,
    roughness: 0.01,
    reflection: 0.7,
    phong: 0.01,
    normal_peturbation: Vec3::new( 0., 0., 0.)
};
const BLACK_MARBLE: Material = Material{
    pigment: Color::new(0.1, 0.1, 0.1),
    metallic: 0.1,
    roughness: 0.01,
    reflection: 0.7,
    phong: 0.01,
    normal_peturbation: Vec3::new( 0., 0., 0.)
};

const CHECKERED_MARBLE: Medium = CheckeredYPlane::new(WHITE_MARBLE, BLACK_MARBLE, 3. 3.); 
