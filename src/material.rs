use color::Color;
use na::Vec3;
///
/// See https://google.github.io/filament//Materials.md.html#materialmodels/litmodel

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub pigment: Color, // Attenuation due to albedo
    pub metallic: f64, // Dielectric 0 to Metallic 1
    pub roughness: f64, // Glossiness
    pub reflection: f64,
    pub phong: f64,
    pub normal_peturbation: Vec3<f64>,
}

pub trait Medium {
    fn material_at(&self, pt: Vec3<f64>) -> Material; 
}

pub struct Solid {
    pub m: Material
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vec3<f64>) -> Material {
        self.m.clone()
    }
}

pub struct CheckeredYPlane {
    m1: Material,
    m2: Material,
    xsize: f64,
    zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: Material, m2: Material, xsize: f64, zsize: f64) -> CheckeredYPlane {
        CheckeredYPlane { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(&self, pt: Vec3<f64>) -> Material {
        let zig = if (pt[0].abs() / self.xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / self.zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        return if !zig != !zag { self.m1.clone() } else { self.m2.clone() };
    }
}

