//! # Measuring a render's noise
//!
//! A path-traced pixel is a Monte Carlo estimate: the mean of n samples of a random variable
//! whose expectation is the pixel's true value. Its error is noise, the variance of that
//! estimate. Comparing sampling techniques needs that variance as a number, measured the same
//! way each time, rather than by eye.
//!
//! The variance of one render can't be found from the render alone, but it can from several
//! renders that differ only in their seed: each pixel then has `k` independent estimates
//! x_1 .. x_k, whose sample mean and unbiased sample variance are
//!
//! ```text
//! m = (1/k) Σ x_i         v = (1/(k-1)) Σ (x_i - m)²
//! ```
//!
//! `v` estimates the variance of a single render at the scene's sample count. No reference
//! image is needed, so any scene can be measured; the cost is k renders. With few renders `v` is
//! itself noisy, so summaries average it over every pixel.
//!
//! Absolute variance favours dark images, so each pixel's is divided by its squared value, the
//! relative mean squared error. Near black that division explodes, so a small constant ε is
//! added to the denominator (here 0.01, in linear units where 1 is a white display value). The
//! value divided by is the median of the x_i, not their mean:
//!
//! ```text
//! relMSE = v / (median(x)² + ε)
//! ```
//!
//! Dividing by the mean caps a firefly's share. A pixel whose estimates are all near a,
//! except one that is f ≫ a, has v ≈ f²/k and m ≈ f/k, so v / m² ≈ k however large f is: a
//! technique with rare, huge samples (the worst noise there is) measured as less noisy than
//! one with moderate noise everywhere. Measured so, BSDF sampling alone on `glossy-sun` came out
//! half as noisy as MIS, against an error from a converged reference 90 times larger. The
//! median ignores the one firefly, so v / median² grows as f² does.
//!
//! Halving the noise by taking more samples costs time, so a technique that halves the variance
//! but doubles the render time gains nothing. The efficiency of an estimator F is the reciprocal
//! of its variance times its running time, ε[F] = 1 / (V[F] T[F]): Eric Veach, "Robust Monte
//! Carlo Methods for Light Transport Simulation", PhD thesis, Stanford 1997, section 2.4.3,
//! equation 2.19 (<https://graphics.stanford.edu/papers/veach_thesis/>); and Pharr, Jakob and
//! Humphreys, "Physically Based Rendering", 4th ed., section 2.1.4
//! (<https://pbr-book.org/4ed/Monte_Carlo_Integration/Monte_Carlo_Basics>). Here V is the
//! relMSE. Its absolute value means nothing; ratios between techniques on the same scene do.
//!
//! Each channel of each pixel counts separately. The mean over pixels is sensitive to
//! fireflies, rare samples many times brighter than the rest, which is what it should be: they
//! are the most visible noise. The median shows the noise of a typical pixel.
use crate::color::Color;
use crate::output::LinearImage;

/// The constant added to each pixel's squared value before dividing by it, so near-black
/// pixels don't dominate the relative error.
pub const EPSILON: f64 = 0.01;

/// How noisy a render of the scene is, from several renders differing only in their seed.
pub struct Variance {
    /// Mean of the renders: a less noisy image than any one of them.
    pub mean: LinearImage,
    /// Per pixel, the square root of the mean over channels of relMSE: the standard deviation
    /// of a single render relative to the pixel's value.
    pub relative_sd: Vec<f64>,
    /// Mean relMSE over every channel of every pixel.
    pub rel_mse: f64,
    /// Median relMSE over every channel of every pixel.
    pub median_rel_mse: f64,
}

impl Variance {
    /// Measure from `renders`, which must number at least two and share a size.
    pub fn of(renders: &[LinearImage]) -> Variance {
        assert!(renders.len() >= 2, "Measuring variance needs at least two renders");
        let (width, height) = (renders[0].width, renders[0].height);
        assert!(renders.iter().all(|r| r.width == width && r.height == height), "Renders differ in size");
        let k = renders.len() as f64;
        let n = width * height;
        let mut mean = Vec::with_capacity(n);
        let mut relative_sd = Vec::with_capacity(n);
        let mut rel_mse = Vec::with_capacity(n * 3);
        for i in 0..n {
            let m = renders.iter().fold(Color::black(), |a, r| a + r.pixels[i]) / k;
            let mut pixel = 0.;
            for c in 0..3 {
                let v = renders.iter().map(|r| (r.pixels[i].rgb[c] - m.rgb[c]).powi(2)).sum::<f64>() / (k - 1.);
                let e = v / (median(renders.iter().map(|r| r.pixels[i].rgb[c]).collect()).powi(2) + EPSILON);
                rel_mse.push(e);
                pixel += e / 3.;
            }
            mean.push(m);
            relative_sd.push(pixel.sqrt());
        }
        let total = rel_mse.iter().sum::<f64>() / rel_mse.len() as f64;
        rel_mse.sort_by(|a, b| a.total_cmp(b));
        Variance {
            mean: LinearImage { width, height, pixels: mean },
            relative_sd,
            rel_mse: total,
            median_rel_mse: rel_mse[rel_mse.len() / 2],
        }
    }

    /// The relative standard deviation per pixel as an image to view: black is noiseless, white
    /// a standard deviation as large as the pixel's value or more.
    pub fn map(&self) -> LinearImage {
        LinearImage {
            width: self.mean.width,
            height: self.mean.height,
            pixels: self.relative_sd.iter().map(|&s| Color::new(s.min(1.), s.min(1.), s.min(1.))).collect(),
        }
    }
}

/// The median of `values`: the middle one, or the mean of the middle two.
fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    if n % 2 == 1 { values[n / 2] } else { (values[n / 2 - 1] + values[n / 2]) / 2. }
}

/// Efficiency, `1 / (relMSE t)` for a render taking `seconds`: higher is better.
pub fn efficiency(rel_mse: f64, seconds: f64) -> f64 {
    1. / (rel_mse * seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(values: &[f64]) -> LinearImage {
        LinearImage { width: values.len(), height: 1, pixels: values.iter().map(|&v| Color::new(v, v, v)).collect() }
    }

    #[test]
    fn test_identical_renders_have_no_variance() {
        let v = Variance::of(&[image(&[0.5, 1.]), image(&[0.5, 1.])]);
        assert_eq!(v.rel_mse, 0.);
        assert_eq!(v.relative_sd, vec![0., 0.]);
    }

    #[test]
    fn test_variance_is_unbiased_sample_variance() {
        // Values 1 and 3: mean and median 2, sample variance ((1)² + (1)²) / (2 - 1) = 2.
        let v = Variance::of(&[image(&[1.]), image(&[3.])]);
        assert_eq!(v.mean.pixels[0], Color::new(2., 2., 2.));
        assert!((v.rel_mse - 2. / (4. + EPSILON)).abs() < 1e-12);
    }

    #[test]
    fn test_rare_fireflies_count_as_worse_than_moderate_noise() {
        // Moderate: every pixel 0 or 2, in turn (mean 1, relMSE about 1.3). Fireflies: every
        // pixel 0.9, but a tenth of them 100 in one render of the four. The fireflies are the
        // far larger error (a variance of about 2500 there), and must measure as such; divided
        // by the mean, each firefly's share was capped near k = 4, and they measured as less
        // noisy.
        let moderate: Vec<LinearImage> = (0..4).map(|i| image(&vec![if i % 2 == 0 { 0. } else { 2. }; 10])).collect();
        let fireflies: Vec<LinearImage> = (0..4).map(|i| {
            let mut v = vec![0.9; 10];
            if i == 0 { v[0] = 100.; }
            image(&v)
        }).collect();
        let (moderate, fireflies) = (Variance::of(&moderate), Variance::of(&fireflies));
        assert!(fireflies.rel_mse > 10. * moderate.rel_mse, "{} vs {}", fireflies.rel_mse, moderate.rel_mse);
    }

    #[test]
    fn test_relative_error_ignores_brightness() {
        // The same relative spread at two brightnesses gives (nearly) the same relMSE.
        let dim = Variance::of(&[image(&[1.]), image(&[3.])]);
        let bright = Variance::of(&[image(&[10.]), image(&[30.])]);
        assert!((dim.rel_mse - bright.rel_mse).abs() < 0.01 * bright.rel_mse);
    }

    #[test]
    fn test_median_ignores_a_firefly() {
        let renders: Vec<LinearImage> = (0..4).map(|i| {
            let mut v = vec![1.; 9];
            if i == 0 { v[0] = 1000.; }
            image(&v)
        }).collect();
        let v = Variance::of(&renders);
        assert_eq!(v.median_rel_mse, 0.);
        assert!(v.rel_mse > 0.1);
    }
}
