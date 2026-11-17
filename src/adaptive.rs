//! # Adaptive sampling
//!
//! With a fixed number of samples per pixel, most of a render's time can go to pixels that
//! were already smooth (a clear sky, a lit wall) while a few noisy ones (caustics, glossy
//! reflections, soft shadows) still need more. With `"adaptive"`, each pixel keeps sampling only
//! until its noise is below a threshold; `supersamples` becomes the most any pixel takes.
//!
//! ## The noise of a pixel
//!
//! A pixel's value is the mean of its n samples x_k. Besides their sum, each pixel keeps the
//! sum of their squares, so the variance of one sample, and the standard error of the mean,
//! can be estimated without keeping the samples:
//!
//! ```text
//! m = Σx / n      s² = (Σx² − n m²) / (n − 1)      SE² = s² / n
//! ```
//!
//! This is the stopping rule of Lee, Redner and Uselton, "Statistically optimized sampling for
//! distributed ray tracing" (SIGGRAPH 1985). The noise is judged relative to the pixel's
//! displayed value (after `exposure`), as `variance` measures it, with the same ε so near-black
//! pixels don't demand endless samples:
//!
//! ```text
//! relMSE = (1/3) Σ_channels SE² / (m² + ε)          converged when √relMSE < threshold
//! ```
//!
//! A threshold of 0.01 asks for a 1% standard error.
//!
//! Dammertz, Hanika, Keller and Lensch, "A hierarchical automatic stopping condition for Monte
//! Carlo global illumination" (WSCG 2009), which Blender's Cycles follows, instead compare the
//! image with one made of every other sample. That doesn't suit our Sobol' points: sample k's
//! first dimension is k's bits reversed, so the even samples all lie in one half of the pixel,
//! and the two images differ wherever the pixel isn't flat, noisy or not. The sum of squares
//! treats the samples as independent, which, for low-discrepancy points, overstates the error
//! of smooth integrands (an edge, say, converges faster than 1/√n): the rule errs towards more
//! samples.
//!
//! ## Neighbours
//!
//! An estimate of the variance from a few dozen samples is itself noisy, and worst where it
//! matters: a pixel that hasn't yet found a rare bright path (a caustic seen through glass)
//! has a low variance and looks converged. Stopping on it biases the image dark, as Kirk and
//! Arvo showed ("Unbiased sampling techniques for image synthesis", SIGGRAPH 1991). Three
//! defences, as in Cycles and the literature since:
//!
//! - Every pixel takes `min_samples` before its noise is judged.
//! - Each pixel's relMSE is averaged with its neighbours' (a 3 x 3 box), so a converged
//!   estimate must be shared by the pixels around it: noise is spatially coherent, and the
//!   estimate of each pixel's is improved by the others'.
//! - Every neighbour of an unconverged pixel keeps sampling too (a dilation of the mask),
//!   so the edges of a noisy area aren't left behind.
//!
//! ## Determinism
//!
//! A render depends only on its seed: sample k of a pixel is the same whenever it is taken,
//! and which pixels take more is decided per region (`chunk_size`), from that region's own
//! pixels, after each pass over it. Neighbours across a region's edge aren't looked at, as
//! they may be any number of passes ahead or behind. So threads, interruption and resuming
//! don't change the image. Changing the threshold or the most samples keeps a saved state, as
//! it holds every pixel's exact sums, and carries on from it; but the result then differs a
//! little from a render made with the new settings from the start, as pixels that stopped
//! under the old ones changed their neighbours' later decisions.
//!
//! The wavefront integrator renders whole passes over the image, so doesn't sample
//! adaptively: with `"adaptive"`, the path tracer renders.
use crate::color::Color;
use crate::scene::{Adaptive, Scene};

/// How far around each pixel its relMSE is averaged: a (2 SMOOTH + 1)² box.
const SMOOTH: usize = 1;

/// What `relMSE` adds to each channel's squared value: the same as `variance::EPSILON`.
pub const EPSILON: f64 = crate::variance::EPSILON;

/// A pixel's relative mean squared error, from the sum and the sum of squares of its `n`
/// samples, judged at `gain` times its value (the display's exposure). Infinite for fewer
/// than two samples, whose variance can't be estimated.
pub fn rel_mse(sum: &Color, squares: &Color, n: usize, gain: f64) -> f64 {
    if n < 2 {
        return f64::INFINITY;
    }
    let n = n as f64;
    let channel = |s: f64, q: f64| {
        let m = s / n;
        let variance = ((q - s * m) / (n - 1.)).max(0.);
        let se2 = variance / n * gain * gain;
        se2 / ((m * gain).powi(2) + EPSILON)
    };
    let e = (channel(sum.rgb.x, squares.rgb.x) + channel(sum.rgb.y, squares.rgb.y) + channel(sum.rgb.z, squares.rgb.z)) / 3.;
    // A NaN (from a NaN sample's square) is noise.
    if e.is_nan() { f64::INFINITY } else { e }
}

/// How bright a linear value of one is displayed: the exposure, and the film's gain.
pub fn display_gain(s: &Scene) -> f64 {
    let white = s.output.film.apply(&Color::white());
    s.output.exposure * (white.rgb.x + white.rgb.y + white.rgb.z) / 3.
}

/// A region of the image's pixels: their sums, sums of squares and sample counts, row by row
/// (`width` wide).
pub struct Region<'a> {
    pub width: usize,
    pub sum: &'a [Color],
    pub squares: &'a [Color],
    pub samples: &'a [usize],
}

/// Which of a region's pixels should take more samples, up to `target`.
pub fn active(r: &Region, target: usize, adaptive: &Adaptive, gain: f64) -> Vec<bool> {
    let w = r.width;
    let h = r.samples.len() / w.max(1);
    let min = adaptive.min_samples.min(target);
    let t2 = adaptive.threshold * adaptive.threshold;
    let e: Vec<f64> = (0..w * h).map(|i| rel_mse(&r.sum[i], &r.squares[i], r.samples[i], gain)).collect();
    // The (2r + 1)² neighbourhood of (x, y), within the region.
    let around = |x: usize, y: usize, r: usize| {
        (y.saturating_sub(r)..(y + r + 1).min(h)).flat_map(move |v| (x.saturating_sub(r)..(x + r + 1).min(w)).map(move |u| v * w + u))
    };
    let wanting: Vec<bool> = (0..w * h).map(|i| {
        let (x, y) = (i % w, i / w);
        let (total, count) = around(x, y, SMOOTH).fold((0., 0), |(t, c), j| (t + e[j], c + 1));
        r.samples[i] < min || total / count as f64 > t2
    }).collect();
    (0..w * h).map(|i| r.samples[i] < target && around(i % w, i / w, 1).any(|j| wanting[j])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sums of n samples, alternating between a and b.
    fn sums(a: f64, b: f64, n: usize) -> (Color, Color) {
        let (mut s, mut q) = (0., 0.);
        for k in 0..n {
            let x = if k % 2 == 0 { a } else { b };
            s += x;
            q += x * x;
        }
        (Color::new(s, s, s), Color::new(q, q, q))
    }

    #[test]
    fn test_rel_mse() {
        // Constant samples: no noise.
        let (s, q) = sums(0.5, 0.5, 16);
        assert!(rel_mse(&s, &q, 16, 1.) < 1e-20);
        // Samples of 0 and 1: variance 16/15 x 0.25, mean 0.5.
        let (s, q) = sums(0., 1., 16);
        let expected = (0.25 * 16. / 15.) / 16. / (0.25 + EPSILON);
        assert!((rel_mse(&s, &q, 16, 1.) - expected).abs() < 1e-12);
        // Four times the samples, a quarter of the error.
        let (s4, q4) = sums(0., 1., 64);
        let ratio = rel_mse(&s, &q, 16, 1.) / rel_mse(&s4, &q4, 64, 1.);
        assert!((ratio - 4. * 63. / 60.).abs() < 1e-9, "{}", ratio);
        // Too few samples to tell.
        assert_eq!(rel_mse(&s, &q, 1, 1.), f64::INFINITY);
        // Darker on display, the same noise matters less.
        assert!(rel_mse(&s, &q, 16, 0.01) < rel_mse(&s, &q, 16, 1.));
    }

    /// A region w x h whose pixels all have n samples alternating a and b, but for a noisy one
    /// at `noisy`.
    fn region(w: usize, h: usize, n: usize, noisy: Option<usize>) -> (Vec<Color>, Vec<Color>, Vec<usize>) {
        let (cs, cq) = sums(0.5, 0.5, n);
        let (ns, nq) = sums(0., 1., n);
        let sum = (0..w * h).map(|i| if Some(i) == noisy { ns } else { cs }).collect();
        let squares = (0..w * h).map(|i| if Some(i) == noisy { nq } else { cq }).collect();
        (sum, squares, vec![n; w * h])
    }

    #[test]
    fn test_active_follows_the_noise_and_its_neighbours() {
        let a = Adaptive { threshold: 0.01, min_samples: 8 };
        let (w, h) = (6, 5);
        // Smooth everywhere: done.
        let (sum, squares, samples) = region(w, h, 16, None);
        let r = Region { width: w, sum: &sum, squares: &squares, samples: &samples };
        assert!(active(&r, 64, &a, 1.).iter().all(|x| !x));
        // But not before the minimum.
        let (sum, squares, samples) = region(w, h, 4, None);
        let r = Region { width: w, sum: &sum, squares: &squares, samples: &samples };
        assert!(active(&r, 64, &a, 1.).iter().all(|x| *x));
        // One noisy pixel at (2, 2): it, the neighbours sharing its average, and theirs.
        let (sum, squares, samples) = region(w, h, 16, Some(2 * w + 2));
        let r = Region { width: w, sum: &sum, squares: &squares, samples: &samples };
        let on = active(&r, 64, &a, 1.);
        for y in 0..h {
            for x in 0..w {
                let d = (x as isize - 2).abs().max((y as isize - 2).abs());
                assert_eq!(on[y * w + x], d <= 2, "({}, {})", x, y);
            }
        }
        // None past the most samples.
        assert!(active(&r, 16, &a, 1.).iter().all(|x| !x));
    }
}
