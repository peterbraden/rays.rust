//! What the renderer and its GPU shader both compute, written once.
//!
//! The shader is Rust, compiled to SPIR-V by rust-gpu, but it can't take the renderer's code as
//! it stands: a GPU has no f64 (WebGPU and Metal have none at all), no heap, no pointers to
//! functions, no standard library. So the kernels were ports, a second copy of each formula in
//! f32, to be kept in step with the first by tests. This crate is the other way: a function
//! here is generic over the float it computes in (`Real`), reads its tables through a trait
//! (`Tables`), and uses nothing a kernel can't. The renderer calls it at f64 with its own
//! tables, the shader at f32 with a buffer, and there is one definition.
//!
//! What that asks of the code here:
//! - Constants are ratios of small integers (`Real::ratio`), not float literals: 7/10 divided
//!   in f64 is the double nearest 0.7, as the literal `0.7` is, so moving a formula here
//!   doesn't change the renderer's results by a bit; and no f64 reaches the GPU's build.
//! - Tables are read by index from whatever holds them, floats and integers alike.
//! - No recursion, no slices of local arrays, no closures over buffers: what rust-gpu and
//!   naga take (the renderer's CLAUDE.md lists what they don't).
//!
//! So far: Perlin's noise, and the Nubis clouds' density (`clouds`).
#![cfg_attr(target_arch = "spirv", no_std)]

pub mod clouds;
pub mod media;
pub mod noise;
pub mod phase;

use core::ops::{Add, Div, Mul, Neg, Sub};

/// The float a shared function computes in: f64 for the renderer, f32 for the shader.
pub trait Real: Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self> {
    /// An integer, exactly.
    fn of(n: i32) -> Self;
    fn floor(self) -> Self;
    fn sqrt(self) -> Self;
    /// Towards zero, as `as i32`.
    fn to_i32(self) -> i32;
    fn ln(self) -> Self;
    fn exp(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn pi() -> Self;
    /// 1 with this number's sign (a negative zero's too, where the float tells them apart).
    fn sign(self) -> Self;
    /// The most tries a rejection sampler takes before it keeps what it has: without limit for
    /// the renderer, as ever; a kernel's loops must end.
    fn tries() -> u32;
    /// Further than anything: infinity where there is one (the GPU's shaders have no constant
    /// for it).
    fn far() -> Self;
    /// The least a walk along a ray advances, relative to how far along it is: so that a
    /// stretch reported as empty at a boundary can't stall it.
    fn hair() -> Self;

    #[inline]
    fn abs(self) -> Self {
        if self < Self::of(0) { -self } else { self }
    }

    /// n / d: the nearest value to that ratio, as a decimal literal for it would be.
    #[inline]
    fn ratio(n: i32, d: i32) -> Self {
        Self::of(n) / Self::of(d)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        if other < self { other } else { self }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        if other > self { other } else { self }
    }

    #[inline]
    fn clamp(self, lo: Self, hi: Self) -> Self {
        if self < lo { lo } else if self > hi { hi } else { self }
    }
}

impl Real for f32 {
    #[inline]
    fn of(n: i32) -> f32 { n as f32 }
    #[inline]
    fn floor(self) -> f32 {
        // (core has no floor or sqrt: on the GPU they are spirv-std's.)
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::floor(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::floor(self)
    }
    #[inline]
    fn sqrt(self) -> f32 {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::sqrt(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::sqrt(self)
    }
    #[inline]
    fn to_i32(self) -> i32 { self as i32 }
    #[inline]
    fn ln(self) -> f32 {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::ln(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::ln(self)
    }
    #[inline]
    fn exp(self) -> f32 {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::exp(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::exp(self)
    }
    #[inline]
    fn sin(self) -> f32 {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::sin(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::sin(self)
    }
    #[inline]
    fn cos(self) -> f32 {
        #[cfg(target_arch = "spirv")]
        return spirv_std::num_traits::Float::cos(self);
        #[cfg(not(target_arch = "spirv"))]
        f32::cos(self)
    }
    #[inline]
    fn pi() -> f32 { core::f32::consts::PI }
    #[inline]
    fn sign(self) -> f32 { if self < 0.0 { -1.0 } else { 1.0 } }
    #[inline]
    fn tries() -> u32 { 64 }
    #[inline]
    fn far() -> f32 { 1e30 }
    #[inline]
    fn hair() -> f32 { 1e-6 }
}

// (The GPU has no f64: its build leaves this out.)
#[cfg(not(target_arch = "spirv"))]
impl Real for f64 {
    #[inline]
    fn of(n: i32) -> f64 { n as f64 }
    #[inline]
    fn floor(self) -> f64 {
        // `self.floor()`, exactly, without calling libm's `floor` (which it is without
        // SSE4.1): noise takes three a point.
        if self.abs() < 4.5e15 {
            let t = self as i64 as f64;
            if t > self { t - 1. } else { t }
        } else {
            f64::floor(self)
        }
    }
    #[inline]
    fn sqrt(self) -> f64 { f64::sqrt(self) }
    #[inline]
    fn to_i32(self) -> i32 { self as i32 }
    #[inline]
    fn ln(self) -> f64 { f64::ln(self) }
    #[inline]
    fn exp(self) -> f64 { f64::exp(self) }
    #[inline]
    fn sin(self) -> f64 { f64::sin(self) }
    #[inline]
    fn cos(self) -> f64 { f64::cos(self) }
    #[inline]
    fn pi() -> f64 { core::f64::consts::PI }
    #[inline]
    fn sign(self) -> f64 { 1f64.copysign(self) }
    #[inline]
    fn tries() -> u32 { u32::MAX }
    #[inline]
    fn far() -> f64 { f64::INFINITY }
    #[inline]
    fn hair() -> f64 { 1e-12 }
}

/// Uniform random numbers in [0, 1), from wherever a caller's come: the renderer's sampler,
/// or a kernel's hash.
pub trait Random<F: Real> {
    fn next(&mut self) -> F;
}

/// Where a shared function's tables are: numbers by index, as floats (a texture's texels) or
/// as integers (a permutation). The renderer's are its own vectors; the shader's, a buffer.
pub trait Tables<F: Real> {
    fn value(&self, i: u32) -> F;
    fn index(&self, i: u32) -> u32;
}

/// a + (b - a) t.
#[inline]
pub fn lerp<F: Real>(a: F, b: F, t: F) -> F {
    a + (b - a) * t
}

/// (v - lo) / (hi - lo), clamped to [0, 1]; a step at `hi` if the range is empty, with `hi`
/// itself below it, so a threshold of 1 on a value in [0, 1] passes nothing.
#[inline]
pub fn remap<F: Real>(v: F, lo: F, hi: F) -> F {
    if hi <= lo {
        return if v > hi { F::of(1) } else { F::of(0) };
    }
    ((v - lo) / (hi - lo)).clamp(F::of(0), F::of(1))
}

#[inline]
pub fn smoothstep<F: Real>(lo: F, hi: F, v: F) -> F {
    let t = remap(v, lo, hi);
    t * t * (F::of(3) - F::of(2) * t)
}
