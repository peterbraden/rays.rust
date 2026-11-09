//! # Comparing against reference renders
//!
//! A render is compared with a reference image of the same scene from another renderer (see
//! `demo/reference/`). Both are Monte Carlo estimates, so they never agree exactly: the metrics
//! need to separate noise from a genuine difference, such as a wrong brightness.
//!
//! - Exposure: the scale `k` that best fits our image to the reference, minimising
//!   sum((k o - r)^2), so k = sum(o r) / sum(o^2) over every channel of every pixel. If our
//!   radiometry (light units, falloff, BRDF normalisation) matches the other renderer's, k is
//!   close to 1. A k far from 1 is a brightness bug, which a fitted comparison would hide.
//! - RMSE: the root-mean-square difference of linear values, before and after applying k. The
//!   difference between the two separates an overall brightness error from noise and from
//!   errors in particular places.
//! - FLIP: a perceptual difference, from Pontus Andersson, Jim Nilsson, Tomas Akenine-Möller,
//!   Magnus Oskarsson, Kalle Åström and Mark D. Fairchild, "FLIP: A Difference Evaluator for
//!   Alternating Images", Proceedings of the ACM on Computer Graphics and Interactive
//!   Techniques 3(2), 2020, doi:10.1145/3406183
//!   (<https://research.nvidia.com/publication/2020-07_FLIP>). It models what a viewer flipping
//!   between the two images would notice, in colour and in edges and points, at a given viewing
//!   distance. Each pixel's error is in [0, 1]; the paper's authors recommend the mean as the
//!   single summary. This is the low dynamic range form, applied to both images after our sRGB
//!   display encoding. The high dynamic range form, which accounts for a range of exposures, is
//!   in Andersson, Nilsson, Shirley and Akenine-Möller, "Visualizing Errors in Rendered High
//!   Dynamic Range Images", Eurographics 2021 short papers, doi:10.2312/egs.20211015; the Rust
//!   bindings used here (`nv-flip`) only provide the former.
use crate::color::Color;
use crate::output::{LinearImage, OutputOpts};

pub struct Comparison {
    /// The scale that best fits our image to the reference.
    pub exposure: f64,
    /// Root-mean-square difference of linear values.
    pub rmse: f64,
    /// RMSE after scaling our image by `exposure`.
    pub rmse_fitted: f64,
    /// Mean FLIP error, in [0, 1].
    pub flip_mean: f64,
    /// FLIP error per pixel, coloured with the magma colour map.
    pub error_map: image::RgbImage,
    /// The reference at our resolution, for viewing alongside our render.
    pub reference: LinearImage,
}

/// Average `img` down to `width` x `height`, which must divide it exactly.
pub fn downsample(img: &LinearImage, width: usize, height: usize) -> Result<LinearImage, String> {
    if img.width % width != 0 || img.height % height != 0 || img.width / width != img.height / height {
        return Err(format!("Can't scale a {}x{} reference to {}x{}: choose a size dividing it by a whole factor",
            img.width, img.height, width, height));
    }
    let f = img.width / width;
    let pixels = (0 .. width * height).map(|i| {
        let (x, y) = (i % width, i / width);
        let mut sum = Color::black();
        for dy in 0 .. f {
            for dx in 0 .. f {
                sum = sum + img.get(x * f + dx, y * f + dy);
            }
        }
        sum / (f * f) as f64
    }).collect();
    Ok(LinearImage { width, height, pixels })
}

fn rmse(a: &LinearImage, b: &LinearImage, scale: f64) -> f64 {
    let sum: f64 = a.pixels.iter().zip(&b.pixels).map(|(p, q)| (p.rgb * scale - q.rgb).norm_squared()).sum();
    (sum / (3 * a.pixels.len()) as f64).sqrt()
}

fn flip_image(img: &LinearImage, opts: &OutputOpts) -> nv_flip::FlipImageRgb8 {
    let data: Vec<u8> = img.pixels.iter().flat_map(|c| opts.quantise(*c)).collect();
    nv_flip::FlipImageRgb8::with_data(img.width as u32, img.height as u32, &data)
}

/// Compare our render with a reference, scaling the reference down to our size if needed.
pub fn compare(ours: &LinearImage, reference: &LinearImage, opts: &OutputOpts) -> Result<Comparison, String> {
    let reference = if (reference.width, reference.height) == (ours.width, ours.height) {
        LinearImage { width: reference.width, height: reference.height, pixels: reference.pixels.clone() }
    } else {
        downsample(reference, ours.width, ours.height)?
    };

    let dot: f64 = ours.pixels.iter().zip(&reference.pixels).map(|(o, r)| o.rgb.dot(&r.rgb)).sum();
    let norm: f64 = ours.pixels.iter().map(|o| o.rgb.norm_squared()).sum();
    let exposure = if norm > 0. { dot / norm } else { 0. };

    let error = nv_flip::flip(flip_image(&reference, opts), flip_image(ours, opts), nv_flip::DEFAULT_PIXELS_PER_DEGREE);
    let flip_mean = nv_flip::FlipPool::from_image(&error).mean() as f64;
    let map = error.apply_color_lut(&nv_flip::magma_lut());
    let error_map = image::RgbImage::from_raw(map.width(), map.height(), map.to_vec())
        .ok_or("FLIP returned an error map of the wrong size")?;

    Ok(Comparison {
        exposure,
        rmse: rmse(ours, &reference, 1.),
        rmse_fitted: rmse(ours, &reference, exposure),
        flip_mean,
        error_map,
        reference,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(width: usize, height: usize, f: impl Fn(usize, usize) -> f64) -> LinearImage {
        let pixels = (0 .. width * height).map(|i| {
            let v = f(i % width, i / width);
            Color::new(v, v * 0.5, v * 0.25)
        }).collect();
        LinearImage { width, height, pixels }
    }

    #[test]
    fn test_identical_images_agree() {
        let a = image(32, 32, |x, y| (x + y) as f64 / 64.);
        let c = compare(&a, &a, &OutputOpts::default()).unwrap();
        assert!((c.exposure - 1.).abs() < 1e-12);
        assert_eq!(c.rmse, 0.);
        assert!(c.flip_mean < 1e-6, "{}", c.flip_mean);
    }

    #[test]
    fn test_exposure_fit_separates_brightness_from_noise() {
        let reference = image(32, 32, |x, y| 0.1 + (x * y) as f64 / 1024.);
        // Twice as bright: the fit finds k = 0.5, and after it the images agree.
        let bright = image(32, 32, |x, y| 2. * (0.1 + (x * y) as f64 / 1024.));
        let c = compare(&bright, &reference, &OutputOpts::default()).unwrap();
        assert!((c.exposure - 0.5).abs() < 1e-12);
        assert!(c.rmse > 0.05);
        assert!(c.rmse_fitted < 1e-12);
        assert!(c.flip_mean > 0.05, "{}", c.flip_mean);
    }

    #[test]
    fn test_downsample_averages_blocks() {
        let big = image(4, 4, |x, y| (x / 2 + 2 * (y / 2)) as f64 + if (x + y) % 2 == 0 { 0.5 } else { -0.5 });
        let small = downsample(&big, 2, 2).unwrap();
        let reds: Vec<f64> = small.pixels.iter().map(|c| c.rgb.x).collect();
        assert_eq!(reds, [0., 1., 2., 3.]);
        assert!(downsample(&big, 3, 3).is_err());
    }
}
