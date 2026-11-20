//! Small functions everything uses: interpolation and remapping. They are the `shared` crate's
//! (`shaders/shared`, which the GPU's kernels compile too), under the renderer's f64 names.
//! They used to live in `procedural::nubis_clouds`, which still re-exports them.

/// (v - lo) / (hi - lo), clamped to [0, 1]; a step at `hi` if the range is empty, with `hi`
/// itself below it, so a threshold of 1 on a value in [0, 1] (no coverage) passes nothing.
pub fn remap(v: f64, lo: f64, hi: f64) -> f64 {
    shared::remap(v, lo, hi)
}

/// 0 below `lo`, 1 above `hi`, and Hermite's smooth step between.
pub fn smoothstep(lo: f64, hi: f64, v: f64) -> f64 {
    shared::smoothstep(lo, hi, v)
}

/// `a` at t = 0, `b` at t = 1, linearly between and beyond.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    shared::lerp(a, b, t)
}

/// Fractal Perlin noise is in [0, 1] but rarely strays far from 0.5; stretch it to use the range.
pub fn stretched(n: f64) -> f64 {
    shared::clouds::stretched(n)
}
