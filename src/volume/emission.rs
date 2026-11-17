//! Media that emit light, as flames and hot gas do (Pharr, Jakob and Humphreys, "Physically
//! Based Rendering", 4th ed., section 11.1.4, and its `GridMedium` with `Le` and the
//! temperature grid of `NanoVDBMedium`).
//!
//! A medium emits radiance L_e per unit absorption: along a ray it adds σ_a(x) L_e(x) per unit
//! length, as much as it absorbs of light of radiance L_e. Delta tracking picks collisions with
//! density σ_t T, so at a collision the emission estimate is σ_a / σ_t L_e = (1 - α) L_e, α the
//! single-scattering albedo: added at each collision, as surfaces add theirs at a hit. A medium
//! that only scatters (α = 1) emits nothing, however hot.
//!
//! The emission is a colour, or blackbody radiation (Planck's law) at a temperature, constant
//! or from a field such as a fire's temperature grid. Unlike pbrt, which normalises each
//! temperature's spectrum to a peak of one, the radiance follows Planck's law in brightness as
//! well as colour, so a flame's 1800 K core outshines its 1200 K edges about 800 times, as a
//! real one does: `scale` sets the luminance at the hottest temperature (the field's majorant).

use crate::color::Color;
use crate::na::Vector3;
use crate::spectrum::{current, planck, spectrum_to_xyz, xyz_to_srgb};
use crate::volume::DensityField;
use std::sync::Arc;

/// Temperatures in the RGB table.
const TABLE_SIZE: usize = 512;

pub enum Source {
    /// A colour, as an illuminant.
    Color(Color),
    /// Blackbody radiation at the field's temperature (kelvin), zero below `cutoff`.
    Temperature { field: Arc<dyn DensityField>, cutoff: f64 },
}

pub struct VolumeEmission {
    source: Source,
    scale: f64,
    /// For temperatures: one over the luminance of a blackbody at the hottest, so that it has
    /// luminance `scale`.
    norm: f64,
    hottest: f64,
    /// Linear sRGB of a blackbody (times `norm`) at temperatures from 0 to `hottest`, for RGB
    /// mode.
    table: Vec<Color>,
}

impl VolumeEmission {
    pub fn color(c: Color, scale: f64) -> VolumeEmission {
        VolumeEmission { source: Source::Color(c), scale, norm: 1., hottest: 0., table: vec![] }
    }

    /// Blackbody emission at the field's temperature, with luminance `scale` at its hottest.
    pub fn temperature(field: Arc<dyn DensityField>, cutoff: f64, scale: f64) -> VolumeEmission {
        let hottest = field.majorant();
        assert!(hottest > 0., "An emitting medium's temperatures must be above zero");
        let norm = 1. / spectrum_to_xyz(|l| planck(l, hottest)).y;
        let table = (0..TABLE_SIZE).map(|i| {
            let t = hottest * i as f64 / (TABLE_SIZE - 1) as f64;
            xyz_to_srgb(&(spectrum_to_xyz(|l| planck(l, t)) * norm))
        }).collect();
        VolumeEmission { source: Source::Temperature { field, cutoff }, scale, norm, hottest, table }
    }

    /// The emitted radiance at `p`, at the current sample's wavelengths in spectral mode.
    pub fn radiance(&self, p: &Vector3<f64>) -> Color {
        match &self.source {
            Source::Color(c) => c.as_illuminant() * self.scale,
            Source::Temperature { field, cutoff } => {
                let t = field.density(p);
                if t <= *cutoff || t <= 0. {
                    return Color::black();
                }
                match current() {
                    Some(w) => w.map(|l| planck(l, t) * self.norm * self.scale),
                    None => {
                        let x = (t / self.hottest).min(1.) * (TABLE_SIZE - 1) as f64;
                        let i = (x as usize).min(TABLE_SIZE - 2);
                        let f = x - i as f64;
                        (self.table[i] * (1. - f) + self.table[i + 1] * f) * self.scale
                    },
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::luminance;
    use crate::volume::ConstantDensity;

    #[test]
    fn test_hottest_has_the_given_luminance() {
        let e = VolumeEmission::temperature(Arc::new(ConstantDensity(1800.)), 0., 3.);
        let c = e.radiance(&Vector3::zeros());
        assert!((luminance(&c) - 3.).abs() < 0.01, "{}", c);
        // Orange-red: far more red than blue.
        assert!(c.rgb.x > 5. * c.rgb.z, "{}", c);
    }

    #[test]
    fn test_cooler_is_dimmer_by_plancks_law() {
        struct Ramp;
        impl DensityField for Ramp {
            fn density(&self, p: &Vector3<f64>) -> f64 { p.x }
            fn majorant(&self) -> f64 { 1800. }
        }
        let e = VolumeEmission::temperature(Arc::new(Ramp), 500., 1.);
        let hot = luminance(&e.radiance(&Vector3::new(1800., 0., 0.)));
        let cool = luminance(&e.radiance(&Vector3::new(1200., 0., 0.)));
        // Visible emission falls steeply with temperature: about 800 times from 1800 to 1200 K
        // (Wien's approximation at 600 nm: exp(14388 µm K / 0.6 µm × (1/1200 - 1/1800) K⁻¹)).
        assert!(hot / cool > 600. && hot / cool < 1100., "{}", hot / cool);
        assert_eq!(e.radiance(&Vector3::new(400., 0., 0.)), Color::black());
    }
}
