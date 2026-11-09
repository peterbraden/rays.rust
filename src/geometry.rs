use crate::na::{Vector3, Vector2};
extern crate rand as _rand;
use crate::geometry::_rand::Rng;
use std::f64;

pub fn rand() -> f64 {
    _rand::thread_rng().gen_range(0.,1.)
}

pub fn random_point_on_unit_sphere() -> Vector3<f64>{
    point_on_unit_sphere(rand(), rand())
}

/// Map (u, v) in [0, 1]^2 uniformly onto the surface of the unit sphere.
pub fn point_on_unit_sphere(u: f64, v: f64) -> Vector3<f64>{
    let theta = u * 2.0 * f64::consts::PI;
    let phi = (2.0 * v - 1.0).acos();
    let sin_phi = phi.sin();
    Vector3::new(sin_phi * theta.cos(), sin_phi * theta.sin(), phi.cos())
}

pub fn random_point_in_unit_ball() -> Vector3<f64>{
    point_in_unit_ball(rand(), rand())
}

/// A point uniformly distributed within the unit ball, in the direction given by (u, v).
pub fn point_in_unit_ball(u: f64, v: f64) -> Vector3<f64>{
    point_on_unit_sphere(u, v) * rand().cbrt()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_and_ball_samplers() {
        for _ in 0..1000 {
            assert!((random_point_on_unit_sphere().norm() - 1.).abs() < 1e-9);
            assert!(random_point_in_unit_ball().norm() <= 1.);
        }
    }
}
