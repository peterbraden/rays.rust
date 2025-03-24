use crate::na::{Vector3, Vector2};
extern crate rand as _rand;
use crate::geometry::_rand::Rng;
use std::f64;

pub fn rand() -> f64 {
    _rand::thread_rng().gen_range(0.,1.)
}

pub fn random_point_on_unit_sphere() -> Vector3<f64>{
    let u = rand();
    let v = rand();
    point_on_unit_sphere(u, v)
}

pub fn point_on_unit_sphere(u: f64, v: f64) -> Vector3<f64>{
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
    Vector3::new(x, y, z)
}

pub fn random_point_on_disc(radius: f64) -> Vector2<f64>{
    let r = radius * rand().sqrt();
    let theta = rand() * 2.0 * f64::consts::PI;
    Vector2::new(r * theta.cos(), r * theta.sin())
}

pub fn uniform_sample_hemisphere(r1: f64, r2: f64) -> Vector3<f64>{
    let sin_theta = (1. - r1 * r1).sqrt(); 
    let  phi = 2. * f64::consts::PI * r2; 
    let x = sin_theta * phi.cos(); 
    let z = sin_theta * phi.sin(); 
    Vector3::new(x, r1, z)
}

// Transform into the world of vec
//pub fn uniform_sample_hemisphere_around(r1: f64, r2:f64, vec: Vector3<f64>) -> Vector3<f64> {
//    let sample = uniform_sample_hemisphere();
//}
