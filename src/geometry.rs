use crate::na::{Vector3, Vector2};
use std::cell::RefCell;
use std::f64;

/// PCG32, the "XSH RR" member of Melissa O'Neill's PCG family: Melissa E. O'Neill, "PCG: A
/// Family of Simple Fast Space-Efficient Statistically Good Algorithms for Random Number
/// Generation", Harvey Mudd College, HMC-CS-2014-0905, 2014
/// (<https://pcg-random.org/pdf/hmc-cs-2014-0905.pdf>).
///
/// The state advances as a 64-bit linear congruential generator, x' = a x + c (mod 2^64), and
/// each output is a permutation of the old state: an xorshift of the high bits, then a rotation
/// chosen by the top five bits. The LCG supplies a long period cheaply; the permutation hides
/// the LCG's well-known weaknesses in its low bits.
///
/// The increment c selects a stream. O'Neill (4.3.2) shows every odd c gives a full-period
/// sequence that shares no pair of successive outputs with any other c, which is what lets each
/// pixel and sample layer have its own independent, reproducible sequence (see
/// `rendercontext::render_pixel`).
#[derive(Clone, Debug)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    const MULTIPLIER: u64 = 6_364_136_223_846_793_005;

    /// Seed the generator `initstate` on stream `stream`, as `pcg32_srandom_r` in O'Neill's
    /// reference implementation.
    pub fn new(initstate: u64, stream: u64) -> Pcg32 {
        let mut rng = Pcg32 { state: 0, inc: (stream << 1) | 1 };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(initstate);
        rng.next_u32();
        rng
    }

    pub fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(Pcg32::MULTIPLIER).wrapping_add(self.inc);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Uniform in [0, 1), using 53 random bits (two outputs) to fill an f64's mantissa.
    pub fn next_f64(&mut self) -> f64 {
        let bits = ((self.next_u32() as u64) << 32) | self.next_u32() as u64;
        (bits >> 11) as f64 * (1. / (1u64 << 53) as f64)
    }
}

thread_local! {
    /// The generator behind `rand()`. Rendering re-seeds it for every pixel and layer, so the
    /// numbers a sample sees don't depend on which thread renders it or in what order.
    static RNG: RefCell<Pcg32> = RefCell::new(Pcg32::new(0, 0));
}

/// Re-seed this thread's generator.
pub fn seed_rand(seed: u64, stream: u64) {
    RNG.with(|r| *r.borrow_mut() = Pcg32::new(seed, stream));
}

/// Uniform in [0, 1), from this thread's generator.
pub fn rand() -> f64 {
    RNG.with(|r| r.borrow_mut().next_f64())
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
    fn test_pcg32_matches_reference_implementation() {
        // pcg32-demo's first outputs for seed 42, stream 54 (pcg-random.org).
        let mut rng = Pcg32::new(42, 54);
        let out: Vec<u32> = (0..6).map(|_| rng.next_u32()).collect();
        assert_eq!(out, [0xa15c02b7, 0x7b47f409, 0xba1d3330, 0x83d2f293, 0xbfa4784b, 0xcbed606e]);
    }

    #[test]
    fn test_seeded_rand_is_reproducible() {
        seed_rand(7, 3);
        let a: Vec<f64> = (0..10).map(|_| rand()).collect();
        seed_rand(7, 3);
        let b: Vec<f64> = (0..10).map(|_| rand()).collect();
        seed_rand(7, 4);
        let c: Vec<f64> = (0..10).map(|_| rand()).collect();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.iter().all(|x| (0. ..1.).contains(x)));
    }

    #[test]
    fn test_sphere_and_ball_samplers() {
        for _ in 0..1000 {
            assert!((random_point_on_unit_sphere().norm() - 1.).abs() < 1e-9);
            assert!(random_point_in_unit_ball().norm() <= 1.);
        }
    }
}
