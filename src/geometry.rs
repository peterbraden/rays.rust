use na::{Vec3};
extern crate rand as _rand;
use geometry::_rand::Rng;
use std::f64;

pub fn rand() -> f64 {
    return _rand::thread_rng().gen_range(0.,1.);
}



pub fn random_point_on_unit_sphere() -> Vec3<f64>{
    let u = rand();
    let v = rand();
    let theta = u * 2.0 * f64::consts::PI;
    let phi = (2.0 * v - 1.0).acos();
    let r = rand().cbrt();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let x = r * sin_phi * cos_theta;
    let y = r * sin_phi * sin_theta;
    let z = r * cos_phi;
    return Vec3::new(x, y, z);
}

