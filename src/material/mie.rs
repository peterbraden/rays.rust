//! Mie scattering by water droplets: the exact solution of Maxwell's equations for a sphere,
//! for cloud and fog droplets too small for geometric optics (and Airy's theory of the bow,
//! `material::rainbow`) to describe. It gives the glory, the coloured rings round the
//! antisolar point on a cloud below (seen round an aircraft's shadow, or a climber's: the
//! Brocken spectre), the white cloudbow near 140°, and the forward diffraction peak.
//!
//! The glory has no explanation in geometric optics: no ray through a drop returns straight
//! back. It comes from surface waves, light tunnelling along the drop's surface at grazing
//! incidence and leaving from the far side, and from the edge rays' interference; only the full
//! Mie series has it (H. M. Nussenzveig, "Light tunneling in clouds", Applied Optics 42(9),
//! 2003; P. Laven, "Simulation of rainbows, coronas and glories by use of Mie theory",
//! Applied Optics 42(3), 2003).
//!
//! The series is computed as in C. F. Bohren and D. R. Huffman, "Absorption and Scattering of
//! Light by Small Particles", Wiley 1983, appendix A (BHMIE): the logarithmic derivative
//! D_n(mx) by downward recurrence, Riccati-Bessel functions ψ_n, χ_n upwards, the coefficients
//! a_n, b_n, and the amplitudes
//!
//! ```text
//! S₁(θ) = Σ (2n+1)/(n(n+1)) (a_n π_n + b_n τ_n),   S₂(θ) = Σ (2n+1)/(n(n+1)) (a_n τ_n + b_n π_n)
//! ```
//!
//! over n up to x + 4x^⅓ + 2 (Wiscombe's criterion). Unpolarised light's differential
//! scattering cross-section is (|S₁|² + |S₂|²) / 2k². A cloud's droplets vary in size, which
//! washes out all but the strongest rings: the table averages over a gamma distribution of
//! radii (Hansen and Travis, "Light scattering in planetary atmospheres", Space Science Reviews
//! 16, 1974) with effective radius r_eff and effective variance v_eff, and over three
//! wavelengths across each colour channel (`material::diffraction::WAVELENGTHS`).

use crate::color::Color;
use rayon::prelude::*;
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

/// Bins of the scattering angle, 0 to 180°: an eighth of a degree, fine enough for the glory's
/// rings and a diffraction peak for droplets up to about 20 µm.
const BINS: usize = 1440;

/// Angles evaluated across each bin, averaged.
const SUBSAMPLES: usize = 3;

/// Water's refractive index at `wavelength` µm (visible; Hale and Querry 1973, fitted).
pub fn water_index(wavelength: f64) -> f64 {
    1.3247 + 3.3e3 / (wavelength * 1e3).powi(2) - 3.2e7 / (wavelength * 1e3).powi(4)
}

/// The Mie series for a sphere of size parameter `x` = 2πr/λ and real relative index `m`: the
/// amplitudes S₁ and S₂ at each of `mus` (cosines of the scattering angle), and the scattering
/// efficiency Q_sca.
pub fn mie(x: f64, m: f64, mus: &[f64]) -> (Vec<Complex<f64>>, Vec<Complex<f64>>, f64) {
    let nstop = (x + 4. * x.cbrt() + 2.).ceil() as usize;
    let y = m * x;
    let nmx = (nstop as f64).max(y.abs()).ceil() as usize + 15;
    // D_n(y) = ψ_n'(y)/ψ_n(y), downwards from zero far above nstop.
    let mut d = vec![0.; nmx + 1];
    for n in (1..=nmx).rev() {
        let nf = n as f64;
        d[n - 1] = nf / y - 1. / (d[n] + nf / y);
    }
    let i = Complex::new(0., 1.);
    let (mut psi0, mut psi1) = (x.cos(), x.sin());
    let (mut chi0, mut chi1) = (-x.sin(), x.cos());
    let mut xi1 = psi1 - i * chi1;
    let mut s1 = vec![Complex::new(0., 0.); mus.len()];
    let mut s2 = vec![Complex::new(0., 0.); mus.len()];
    let mut pi0 = vec![0.; mus.len()];
    let mut pi1 = vec![1.; mus.len()];
    let mut qsca = 0.;
    for n in 1..=nstop {
        let nf = n as f64;
        let psi = (2. * nf - 1.) / x * psi1 - psi0;
        let chi = (2. * nf - 1.) / x * chi1 - chi0;
        let xi = psi - i * chi;
        let da = d[n] / m + nf / x;
        let db = m * d[n] + nf / x;
        let an = (da * psi - psi1) / (da * xi - xi1);
        let bn = (db * psi - psi1) / (db * xi - xi1);
        qsca += (2. * nf + 1.) * (an.norm_sqr() + bn.norm_sqr());
        let f = (2. * nf + 1.) / (nf * (nf + 1.));
        for (j, &mu) in mus.iter().enumerate() {
            // π_n and π_{n-1}; τ_n = n μ π_n - (n+1) π_{n-1}.
            let (p, prev) = (pi1[j], pi0[j]);
            let tau = nf * mu * p - (nf + 1.) * prev;
            s1[j] += (an * p + bn * tau) * f;
            s2[j] += (an * tau + bn * p) * f;
            pi0[j] = p;
            pi1[j] = ((2. * nf + 1.) * mu * p - (nf + 1.) * prev) / nf;
        }
        psi0 = psi1;
        psi1 = psi;
        chi0 = chi1;
        chi1 = chi;
        xi1 = psi1 - i * chi1;
    }
    (s1, s2, qsca * 2. / (x * x))
}

/// Mie scattering by a cloud of water droplets, tabulated per colour channel.
pub struct MieTable {
    /// Per bin, the phase function (per steradian) for each channel.
    bins: Vec<[f64; 3]>,
}

impl MieTable {
    /// For droplets of effective radius `radius` µm, their sizes spread with effective variance
    /// `variance` (0.1 is typical of cloud; small values give sharper rings).
    pub fn new(radius: f64, variance: f64) -> MieTable {
        let (a, v) = (radius.max(0.1), variance.clamp(0.01, 0.5));
        // Cosines at the sub-angles of each bin.
        let mus: Vec<f64> = (0..BINS * SUBSAMPLES)
            .map(|k| ((k as f64 + 0.5) / (BINS * SUBSAMPLES) as f64 * PI).cos())
            .collect();
        // Radii across the gamma distribution n(r) ∝ r^((1-3v)/v) e^(-r/(av)), weighted by it.
        let radii = 48;
        let (lo, hi) = (a * (1. - 3. * v.sqrt()).max(0.15), a * (1. + 4. * v.sqrt()));
        let sizes: Vec<(f64, f64)> = (0..radii).map(|k| {
            let r = lo + (hi - lo) * (k as f64 + 0.5) / radii as f64;
            let w = ((1. - 3. * v) / v * r.ln() - r / (a * v)).exp();
            (r, w)
        }).collect();
        let peak = sizes.iter().map(|s| s.1).fold(0., f64::max);
        let sizes: Vec<(f64, f64)> = sizes.into_iter().map(|(r, w)| (r, w / peak)).collect();
        let waves = crate::material::diffraction::WAVELENGTHS;
        let channels: Vec<Vec<f64>> = (0..3).map(|c| {
            let mut sum = vec![0.; BINS];
            for &lambda in &waves[c] {
                let k = 2. * PI / lambda;
                let m = water_index(lambda);
                let per_size: Vec<(Vec<f64>, f64)> = sizes.par_iter().map(|&(r, w)| {
                    let (s1, s2, q) = mie(k * r, m, &mus);
                    let dc: Vec<f64> = s1.iter().zip(&s2).map(|(a, b)| w * (a.norm_sqr() + b.norm_sqr()) / (2. * k * k)).collect();
                    (dc, w * q * PI * r * r)
                }).collect();
                let csca: f64 = per_size.iter().map(|p| p.1).sum();
                let mut phase = vec![0.; BINS];
                for (dc, _) in &per_size {
                    for (j, v) in dc.iter().enumerate() {
                        phase[j / SUBSAMPLES] += v / (csca * SUBSAMPLES as f64);
                    }
                }
                // Renormalised over the bins, so the table conserves energy exactly.
                let total: f64 = phase.iter().enumerate().map(|(b, p)| p * bin_solid_angle(b)).sum();
                for (s, p) in sum.iter_mut().zip(&phase) {
                    *s += p / total / waves[c].len() as f64;
                }
            }
            sum
        }).collect();
        MieTable { bins: (0..BINS).map(|b| [channels[0][b], channels[1][b], channels[2][b]]).collect() }
    }

    /// The phase function (per steradian) for each channel at the scattering angle whose cosine
    /// is `cos_theta`.
    pub fn eval(&self, cos_theta: f64) -> Color {
        let theta = cos_theta.clamp(-1., 1.).acos();
        let x = theta / PI * BINS as f64 - 0.5;
        let k = (x.floor().max(0.) as usize).min(BINS - 1);
        let f = (x - k as f64).clamp(0., 1.);
        let next = (k + 1).min(BINS - 1);
        let v = [0, 1, 2].map(|c| self.bins[k][c] * (1. - f) + self.bins[next][c] * f);
        Color::new(v[0], v[1], v[2])
    }
}

/// The solid angle of bin `b`.
fn bin_solid_angle(b: usize) -> f64 {
    let t0 = b as f64 / BINS as f64 * PI;
    let t1 = (b + 1) as f64 / BINS as f64 * PI;
    2. * PI * (t0.cos() - t1.cos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rayleigh_limit() {
        // For x ≪ 1, Q_sca = 8/3 x⁴ ((m² - 1)/(m² + 2))².
        let (x, m) = (0.05, 1.33);
        let (_, _, q) = mie(x, m, &[1.]);
        let l = (m * m - 1.) / (m * m + 2.);
        let expected = 8. / 3. * x.powi(4) * l * l;
        assert!((q / expected - 1.).abs() < 0.01, "{} vs {}", q, expected);
    }

    #[test]
    fn test_known_values() {
        // Bohren and Huffman's test case: x = 3, m = 1.33 (real), Q_sca ≈ 1.7 to 2; and for large
        // spheres the extinction paradox, Q → 2.
        let (_, _, q) = mie(3., 1.33, &[1.]);
        assert!(q > 1.0 && q < 4.5, "{}", q);
        let (_, _, q) = mie(400., 1.33, &[1.]);
        assert!((q - 2.).abs() < 0.05, "{}", q);
        // Forward amplitude: the optical theorem, Q_ext = 4/x² Re S(0); with no absorption
        // Q_ext = Q_sca.
        let x = 20.;
        let (s1, _, q) = mie(x, 1.33, &[1.]);
        assert!((4. / (x * x) * s1[0].re / q - 1.).abs() < 1e-6, "{}", 4. / (x * x) * s1[0].re / q);
    }

    #[test]
    fn test_table_has_glory_and_cloudbow() {
        let t = MieTable::new(5., 0.05);
        let at = |deg: f64| t.eval(deg.to_radians().cos());
        // Normalised.
        let total: f64 = (0..BINS).map(|b| t.bins[b][1] * bin_solid_angle(b)).sum();
        assert!((total - 1.).abs() < 1e-9);
        // Strongly forward; then the glory: backscatter rising to a peak at 180°.
        assert!(at(0.5).rgb.y > 100. * at(90.).rgb.y);
        assert!(at(179.9).rgb.y > 2. * at(172.).rgb.y, "{} vs {}", at(179.9).rgb.y, at(172.).rgb.y);
        // The cloudbow: a maximum between 130° and 150° above the side-scatter at 120°.
        let bow = (130..150).map(|d| at(d as f64).rgb.y).fold(0., f64::max);
        assert!(bow > 1.5 * at(120.).rgb.y, "{} vs {}", bow, at(120.).rgb.y);
    }
}
