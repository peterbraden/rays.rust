use crate::na::{Vector3};
use crate::material::model::MaterialModel;

pub trait Medium : Sync{
    fn material_at(&self, pt: Vector3<f64>) -> &Box<dyn MaterialModel + Sync + Send>; 
}

pub struct Solid {
    pub m: Box<dyn MaterialModel + Sync + Send> 
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vector3<f64>) -> &Box<dyn MaterialModel + Sync + Send> {
        &self.m
    }
}

pub struct CheckeredYPlane {
    pub m1: Box<dyn MaterialModel + Sync + Send>,
    pub m2: Box<dyn MaterialModel + Sync + Send>,
    pub xsize: f64,
    pub zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: Box<dyn MaterialModel + Sync + Send>, m2: Box<dyn MaterialModel + Sync + Send>, xsize: f64, zsize: f64) -> CheckeredYPlane {
        CheckeredYPlane { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(&self, pt: Vector3<f64>) -> &Box<dyn MaterialModel + Sync + Send> {
        let zig = if (pt[0].abs() / self.xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / self.zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        return if !zig != !zag { &self.m1 } else { &self.m2 };
    }
}

