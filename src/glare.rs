//! Glare: the light a camera's lens, or an eye, scatters away from where it should land.
//!
//! No optical system images a point to a point. Diffraction at the aperture, scattering in the
//! glass (or the eye's cornea, lens and fluid) and reflections between surfaces spread a small
//! fraction of every bright point's light into a halo whose brightness falls off roughly as the
//! inverse cube of the angle (G. Spencer, P. Shirley, K. Zimmerman and D. P. Greenberg,
//! "Physically-Based Glare Effects for Digital Images", SIGGRAPH 1995). For ordinary scenes the
//! halo is far too faint to see. For the sun, some hundred thousand times brighter than the sky,
//! it is a glow several degrees across, which is how the sun looks in photographs and to the
//! eye: much larger than its disc, half a degree across.
//!
//! The point-spread function used here is a fraction `strength` of the light spread as
//!
//! ```text
//! K(θ) ∝ (1 + (θ/θ₀)²)^(-3/2),
//! ```
//!
//! with core θ₀ (`core`), out to `radius`: the θ⁻³ tail, normalised over the image plane. Only
//! pixels brighter than `threshold` (as displayed, after exposure) spread their light, since
//! only they make a visible halo; the convolution is done by FFT.

use crate::color::Color;
use crate::output::LinearImage;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::FFT;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glare {
    /// Fraction of a bright pixel's light spread into the halo.
    pub strength: f64,
    /// Angular size (radians) of the halo's core, θ₀, and how far it extends.
    pub core: f64,
    pub radius: f64,
    /// Pixels whose value, times the exposure, exceeds this spread their light.
    pub threshold: f64,
}

impl Default for Glare {
    fn default() -> Glare {
        Glare { strength: 0.05, core: 0.3f64.to_radians(), radius: 25f64.to_radians(), threshold: 1. }
    }
}

/// In-place 2D FFT of `data`, `width` x `height` (both powers of two), rows first.
fn fft2d(data: &mut [Complex<f64>], width: usize, height: usize, inverse: bool) {
    let rows = Radix4::new(width, inverse);
    let mut out = vec![Complex::new(0., 0.); width * height];
    rows.process_multi(data, &mut out);
    // Transpose, transform the columns as rows, transpose back.
    let mut t = vec![Complex::new(0., 0.); width * height];
    for y in 0..height {
        for x in 0..width {
            t[x * height + y] = out[y * width + x];
        }
    }
    let cols = Radix4::new(height, inverse);
    let mut out2 = vec![Complex::new(0., 0.); width * height];
    cols.process_multi(&mut t, &mut out2);
    for x in 0..width {
        for y in 0..height {
            data[y * width + x] = out2[x * height + y];
        }
    }
}

impl Glare {
    /// The kernel's weight `r` pixels from its centre, for pixels `pixel_angle` radians across,
    /// unnormalised.
    fn weight(&self, r: f64, pixel_angle: f64) -> f64 {
        let theta = r * pixel_angle;
        if theta > self.radius {
            return 0.;
        }
        (1. + (theta / self.core).powi(2)).powf(-1.5)
    }

    /// `img` with glare, for pixels `pixel_angle` radians across and the given exposure.
    pub fn apply(&self, img: &LinearImage, pixel_angle: f64, exposure: f64) -> LinearImage {
        let (w, h) = (img.width, img.height);
        let threshold = self.threshold / exposure.max(1e-300);
        let bright = |c: &Color| c.rgb.max() > threshold;
        if self.strength <= 0. || !img.pixels.iter().any(bright) {
            return LinearImage { width: w, height: h, pixels: img.pixels.clone() };
        }
        // The kernel's reach in pixels, at most the image's size: light from further off would
        // land outside it anyway.
        let reach = ((self.radius / pixel_angle).ceil() as usize).min(w.max(h));
        let (pw, ph) = ((w + reach).next_power_of_two(), (h + reach).next_power_of_two());
        // The kernel, centred at the origin and wrapped around, normalised to 1.
        let mut kernel = vec![Complex::new(0., 0.); pw * ph];
        let mut total = 0.;
        for dy in -(reach as i64)..=(reach as i64) {
            for dx in -(reach as i64)..=(reach as i64) {
                let k = self.weight(((dx * dx + dy * dy) as f64).sqrt(), pixel_angle);
                if k > 0. {
                    let (x, y) = (dx.rem_euclid(pw as i64) as usize, dy.rem_euclid(ph as i64) as usize);
                    kernel[y * pw + x].re += k;
                    total += k;
                }
            }
        }
        for k in kernel.iter_mut() {
            k.re /= total;
        }
        fft2d(&mut kernel, pw, ph, false);
        let mut pixels = img.pixels.clone();
        for c in 0..3 {
            // What the bright pixels spread, in the padded grid.
            let mut source = vec![Complex::new(0., 0.); pw * ph];
            for y in 0..h {
                for x in 0..w {
                    let p = img.pixels[y * w + x];
                    if bright(&p) {
                        source[y * pw + x].re = p.rgb[c] * self.strength;
                    }
                }
            }
            fft2d(&mut source, pw, ph, false);
            for (s, k) in source.iter_mut().zip(&kernel) {
                *s *= k;
            }
            fft2d(&mut source, pw, ph, true);
            let n = (pw * ph) as f64;
            for y in 0..h {
                for x in 0..w {
                    let i = y * w + x;
                    let spread = source[y * pw + x].re / n;
                    let kept = if bright(&img.pixels[i]) { 1. - self.strength } else { 1. };
                    pixels[i].rgb[c] = img.pixels[i].rgb[c] * kept + spread.max(0.);
                }
            }
        }
        LinearImage { width: w, height: h, pixels }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(w: usize, h: usize, bright: &[(usize, usize, f64)]) -> LinearImage {
        let mut pixels = vec![Color::new(0.1, 0.2, 0.3); w * h];
        for (x, y, v) in bright {
            pixels[y * w + x] = Color::new(*v, *v, *v);
        }
        LinearImage { width: w, height: h, pixels }
    }

    #[test]
    fn test_dim_images_are_untouched() {
        let img = image(40, 30, &[]);
        let out = Glare::default().apply(&img, 0.01, 1.);
        assert_eq!(out.pixels, img.pixels);
    }

    #[test]
    fn test_glare_conserves_light_and_falls_off() {
        // A bright point in the middle of a big image: the halo's light is what the point lost,
        // and it falls as the kernel does.
        let (w, h) = (200, 160);
        let img = image(w, h, &[(100, 80, 1e5)]);
        let g = Glare { strength: 0.05, core: 0.01, radius: 0.3, threshold: 1. };
        let pixel = 0.005;
        let out = g.apply(&img, pixel, 1.);
        let sum = |i: &LinearImage| i.pixels.iter().map(|p| p.rgb.x).sum::<f64>();
        // The kernel reaches 60 pixels, all within the image.
        assert!((sum(&out) - sum(&img)).abs() / sum(&img) < 0.01, "{} vs {}", sum(&out), sum(&img));
        assert!((out.get(100, 80).rgb.x - 0.95e5).abs() < 0.02e5);
        // Two and eight core radii out: (1 + 4)^-1.5 / (1 + 64)^-1.5.
        let halo = |r: usize| out.get(100 + r, 80).rgb.x - 0.1;
        let ratio = halo(4) / halo(16);
        let expected = (65f64 / 5.).powf(1.5);
        assert!((ratio / expected - 1.).abs() < 0.05, "{} vs {}", ratio, expected);
        // No light wraps around from the far side.
        assert!((out.get(0, 0).rgb.x - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_threshold_follows_exposure() {
        // At exposure 0.1, a pixel of 5 shows as 0.5: below white, no glare.
        let img = image(32, 32, &[(16, 16, 5.)]);
        let g = Glare::default();
        assert_eq!(g.apply(&img, 0.01, 0.1).pixels, img.pixels);
        assert_ne!(g.apply(&img, 0.01, 1.).pixels, img.pixels);
    }
}
