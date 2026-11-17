//! Spectral data for the sky (`skysphere`), every 10 nm from 360 to 830 nm, from Eric
//! Bruneton's "Precomputed Atmospheric Scattering: a New Implementation" (2017), file
//! `atmosphere/demo/demo.cc` (<https://github.com/ebruneton/precomputed_atmospheric_scattering>,
//! BSD 3-clause licence, copyright (c) 2017 Eric Bruneton), which bins them from:
//!
//! - The extraterrestrial solar irradiance: the ETR column of ASTM G-173, "Standard Tables for
//!   Reference Solar Spectral Irradiances", averaged over each 10 nm bin; W m⁻² nm⁻¹.
//! - Ozone's absorption cross-section at 233 K: A. Serdyuchenko, V. Gorshelev, M. Weber, W.
//!   Chehade and J. P. Burrows, "High spectral resolution ozone absorption cross-sections - Part
//!   2: Temperature dependence", Atmospheric Measurement Techniques 7, 2014, pp. 625-636,
//!   doi:10.5194/amt-7-625-2014, averaged over each bin; m².
use crate::spectrum::{LAMBDA_MIN, spectrum_to_xyz};
use std::sync::OnceLock;

const SAMPLES: usize = 48;
const STEP: f64 = 10.;

const SOLAR_IRRADIANCE: [f64; SAMPLES] = [
    1.11776, 1.14259, 1.01249, 1.14716, 1.72765, 1.73054, 1.6887, 1.61253,
    1.91198, 2.03474, 2.02042, 2.02212, 1.93377, 1.95809, 1.91686, 1.8298,
    1.8685, 1.8931, 1.85149, 1.8504, 1.8341, 1.8345, 1.8147, 1.78158, 1.7533,
    1.6965, 1.68194, 1.64654, 1.6048, 1.52143, 1.55622, 1.5113, 1.474, 1.4482,
    1.41018, 1.36775, 1.34188, 1.31429, 1.28303, 1.26758, 1.2367, 1.2082,
    1.18737, 1.14683, 1.12362, 1.1058, 1.07124, 1.04992,
];

const OZONE_CROSS_SECTION: [f64; SAMPLES] = [
    1.18e-27, 2.182e-28, 2.818e-28, 6.636e-28, 1.527e-27, 2.763e-27, 5.52e-27,
    8.451e-27, 1.582e-26, 2.316e-26, 3.669e-26, 4.924e-26, 7.752e-26, 9.016e-26,
    1.48e-25, 1.602e-25, 2.139e-25, 2.755e-25, 3.091e-25, 3.5e-25, 4.266e-25,
    4.672e-25, 4.398e-25, 4.701e-25, 5.019e-25, 4.305e-25, 3.74e-25, 3.215e-25,
    2.662e-25, 2.238e-25, 1.852e-25, 1.473e-25, 1.209e-25, 9.423e-26, 7.455e-26,
    6.566e-26, 5.105e-26, 4.15e-26, 4.228e-26, 3.237e-26, 2.451e-26, 2.801e-26,
    2.534e-26, 1.624e-26, 1.465e-26, 2.078e-26, 1.383e-26, 7.105e-27,
];

/// Ozone's number density at the peak of its layer, m⁻³: 300 Dobson units (2.687e20 molecules
/// m⁻² each) spread over the tent-shaped layer of `skysphere`, whose integral is 15 km.
pub const OZONE_PEAK_DENSITY: f64 = 300. * 2.687e20 / 15_000.;

/// Linear interpolation in a 10 nm table, held constant beyond its ends.
fn interpolate(table: &[f64; SAMPLES], lambda: f64) -> f64 {
    let x = ((lambda - LAMBDA_MIN) / STEP).clamp(0., (SAMPLES - 1) as f64);
    let i = (x as usize).min(SAMPLES - 2);
    let f = x - i as f64;
    table[i] * (1. - f) + table[i + 1] * f
}

/// Ozone's absorption coefficient at the peak of its layer, per metre, at `lambda` nm.
pub fn ozone_absorption(lambda: f64) -> f64 {
    OZONE_PEAK_DENSITY * interpolate(&OZONE_CROSS_SECTION, lambda)
}

/// The sun's spectrum above the atmosphere at `lambda` nm, scaled to unit luminance (Y = 1), as
/// a white of RGB (1, 1, 1) has: so a sun of a given intensity is as bright in spectral mode
/// as in RGB, but with sunlight's own colour, a little warmer than D65's (D65 is daylight,
/// sun and sky, at the ground).
pub fn solar(lambda: f64) -> f64 {
    static SCALE: OnceLock<f64> = OnceLock::new();
    let scale = SCALE.get_or_init(|| 1. / spectrum_to_xyz(|l| interpolate(&SOLAR_IRRADIANCE, l)).y);
    interpolate(&SOLAR_IRRADIANCE, lambda) * scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::xyz_to_srgb;

    #[test]
    fn test_ozone_matches_the_rgb_coefficients() {
        // The sky's RGB coefficients are these at 680, 550 and 440 nm.
        for (lambda, rgb) in [(680., 0.650e-6), (550., 1.881e-6), (440., 0.085e-6)] {
            assert!((ozone_absorption(lambda) / rgb - 1.).abs() < 2e-3, "{} nm: {}", lambda, ozone_absorption(lambda));
        }
    }

    #[test]
    fn test_sunlight_is_bright_white_and_a_little_warm() {
        let xyz = spectrum_to_xyz(solar);
        assert!((xyz.y - 1.).abs() < 1e-9);
        let rgb = xyz_to_srgb(&xyz);
        assert!(rgb.rgb.x > rgb.rgb.z && rgb.rgb.z > 0.8, "{:?}", rgb);
    }
}
