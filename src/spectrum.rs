//! # Spectral rendering
//!
//! Light is a spectrum: radiance is a function of wavelength λ, and what a surface or a medium
//! does to it can differ from one wavelength to the next. Rendering in RGB treats the spectrum
//! as three numbers and multiplies them channel by channel, which is exact only when every
//! spectrum involved is a combination of the same three basis functions. It isn't, in general:
//! the product of two spectra need not have the RGB of the product of their RGBs. A yellow
//! (red plus green) light on a surface that reflects only a narrow band of yellow lights it in
//! reality and not at all in RGB, and the brightness of a colour after several bounces between
//! coloured walls drifts. Wavelength-dependent effects, dispersion above all, cannot be
//! rendered in RGB at all.
//!
//! This module follows Pharr, Jakob and Humphreys, "Physically Based Rendering: From Theory to
//! Implementation", 4th ed., MIT Press 2023, chapter 4, "Radiometry, Spectra, and Color"
//! (<https://pbr-book.org/4ed/Radiometry,_Spectra,_and_Color>), cited below as PBR.
//!
//! ## Point-sampled spectra (PBR section 4.5.4)
//!
//! A path carries radiance at a handful of wavelengths sampled per camera ray, and the film
//! turns that estimate of the spectrum into a colour by Monte Carlo integration against the
//! CIE matching functions (below). pbrt-v4 samples four wavelengths; here a path carries three,
//! in the three slots of [`Color`], so that spectral mode reuses every piece of code written
//! for RGB. While a sample is spectral (`begin_sample` to `end_sample`), a `Color` on the path
//! holds radiance, or a factor scaling radiance, at the sample's wavelengths
//! ([`Wavelengths::lambda`]) instead of red, green and blue.
//!
//! Quantities the scene gives as RGB (an albedo, a light's colour) turn into spectra where they
//! enter light transport: [`Color::as_reflectance`], [`Color::as_illuminant`] and
//! [`Color::as_unbounded`] give their values at the sample's wavelengths, and outside spectral
//! sampling return the RGB unchanged, so a renderer in RGB mode is untouched. The conversion
//! is the job of `rgb2spec` (PBR section 4.6.6).
//!
//! The three wavelengths are not independent: the first, the hero, is sampled, and the others
//! are rotations of it by a third of the range in the sampling's primary space, so the three
//! cover the visible spectrum evenly (stratification). Alexander Wilkie, Sehera Nawaz, Marc
//! Droske, Andrea Weidlich and Johannes Hanika, "Hero Wavelength Spectral Sampling", Computer
//! Graphics Forum 33(4) (EGSR 2014), pp. 123-131, doi:10.1111/cgf.12419; PBR section 4.6.5
//! ("SampledWavelengths") and 5.4.2 (`SampleVisibleWavelengths`).
//!
//! When the light's path depends on wavelength (refraction by a dispersive glass, which bends
//! each wavelength its own way), one path can't serve all three: it follows the hero's, and the
//! others are dropped (`terminate_secondary`), their share of the estimate going to the hero.
//!
//! ## Colour (PBR section 4.6)
//!
//! The CIE 1931 standard observer's matching functions x̄, ȳ, z̄ give a spectrum's tristimulus
//! values X = ∫ S(λ) x̄(λ) dλ / ∫ ȳ(λ) dλ (and likewise Y, Z), normalised so that a constant
//! spectrum of 1 has Y = 1. Linear sRGB is a fixed matrix from XYZ (`cie::XYZ_TO_SRGB`), with
//! white at illuminant D65. From a path's three wavelengths, sampled with densities p(λ_i), the
//! estimate (PBR section 4.6.5, `SampledSpectrum::ToXYZ`) is
//!
//! ```text
//! X ≈ (1/3) Σ_i S(λ_i) x̄(λ_i) / p(λ_i) / ∫ ȳ(λ) dλ
//! ```
//!
//! An emitter whose RGB is (1, 1, 1) emits D65 itself, normalised to Y = 1, which the sRGB
//! matrix takes to white: so a white light on a white surface is white, as in RGB mode, and
//! the image needs no white balancing. Reflectances are upsampled to spectra whose colour
//! under D65 is the RGB given, so a scene lit by white light renders, up to noise, as it does
//! in RGB, until coloured light meets coloured surfaces.
//!
//! ## Blackbody emission (PBR section 4.4.1)
//!
//! Planck's law gives the spectral radiance of a blackbody at temperature T. An emitter given a
//! temperature (`Emission::Blackbody`) emits that spectrum, normalised to a luminance of one.
//!
//! ## Dispersion (PBR section 9.3, and the glass data of section 4.5)
//!
//! The refractive index of glass falls with wavelength, which is why a prism splits white light
//! into colours. [`RefractiveIndex`] gives it by the Sellmeier or Cauchy equations, with a few
//! glasses named. In RGB mode a glass takes its index at the sodium d line (587.56 nm), the
//! wavelength at which glass catalogues quote it.
//!
//! ## Where this departs from pbrt-v4
//!
//! - Three wavelengths per path, not four, to fit [`Color`].
//! - Spectra are tabulated every 5 nm and interpolated linearly, not every 1 nm.
//! - The sky (`skysphere`) is evaluated at the sample's wavelengths, but its tables of multiple
//!   scattering and of the sky's view are built at 15 wavelengths and interpolated. The halo,
//!   rainbow and diffraction code keeps its own wavelengths, averaged per RGB channel, and its
//!   colours are upsampled.
//! - Only sRGB, with D65 white; pbrt also offers DCI-P3, Rec. 2020 and ACES2065-1.

pub mod cie;
pub mod rgb2spec;
pub mod atmosphere;

use crate::color::Color;
use crate::na::{Matrix3, Vector3};
use std::cell::Cell;
use std::sync::OnceLock;

/// The range of wavelengths sampled, in nanometres (PBR's `Lambda_min` and `Lambda_max`):
/// that of the CIE tables, beyond which the eye sees nothing.
pub const LAMBDA_MIN: f64 = 360.;
pub const LAMBDA_MAX: f64 = 830.;

/// Spacing of the CIE tables, nm.
const STEP: f64 = 5.;

/// The wavelength glass catalogues quote refractive indices at: the sodium d line (Fraunhofer's
/// D, strictly the helium d line, 587.56 nm), n_d.
pub const D_LINE: f64 = 587.56;

/// Wavelengths carried by each path.
pub const N: usize = 3;

/// A tabulated function of wavelength at `LAMBDA_MIN + STEP i`, interpolated linearly; zero
/// outside the table.
fn interpolate(table: &[f64; cie::SAMPLES], lambda: f64) -> f64 {
    let x = (lambda - LAMBDA_MIN) / STEP;
    if !(0. ..=(cie::SAMPLES - 1) as f64).contains(&x) {
        return 0.;
    }
    let i = (x as usize).min(cie::SAMPLES - 2);
    let f = x - i as f64;
    table[i] * (1. - f) + table[i + 1] * f
}

/// The CIE 1931 matching functions (x̄, ȳ, z̄) at `lambda` nm.
pub fn xyz_matching(lambda: f64) -> Vector3<f64> {
    Vector3::new(interpolate(&cie::CIE_X, lambda), interpolate(&cie::CIE_Y, lambda), interpolate(&cie::CIE_Z, lambda))
}

/// Integrate `f(λ)` over the visible range by the trapezoid rule at 1 nm: exact for the
/// product of the linearly interpolated tables with any function that is linear between
/// nanometres, and close for anything smooth.
pub fn integrate(f: impl Fn(f64) -> f64) -> f64 {
    let n = (LAMBDA_MAX - LAMBDA_MIN) as usize;
    (0..=n).map(|i| {
        let w = if i == 0 || i == n { 0.5 } else { 1. };
        w * f(LAMBDA_MIN + i as f64)
    }).sum()
}

struct Constants {
    /// ∫ ȳ(λ) dλ (PBR's `CIE_Y_integral`).
    y_integral: f64,
    /// Scales the tabulated D65 to Y = 1.
    d65_scale: f64,
}

fn constants() -> &'static Constants {
    static C: OnceLock<Constants> = OnceLock::new();
    C.get_or_init(|| {
        let y_integral = integrate(|l| interpolate(&cie::CIE_Y, l));
        let d65_y = integrate(|l| interpolate(&cie::D65, l) * interpolate(&cie::CIE_Y, l)) / y_integral;
        Constants { y_integral, d65_scale: 1. / d65_y }
    })
}

/// Illuminant D65 at `lambda`, scaled so its luminance Y is 1 (PBR normalises the standard
/// illuminants alike): the spectrum of an emitter of RGB (1, 1, 1).
pub fn d65(lambda: f64) -> f64 {
    interpolate(&cie::D65, lambda) * constants().d65_scale
}

/// The XYZ tristimulus values of the spectrum `f`, normalised so that a constant 1 has Y = 1.
pub fn spectrum_to_xyz(f: impl Fn(f64) -> f64) -> Vector3<f64> {
    let y_integral = constants().y_integral;
    Vector3::new(
        integrate(|l| f(l) * interpolate(&cie::CIE_X, l)),
        integrate(|l| f(l) * interpolate(&cie::CIE_Y, l)),
        integrate(|l| f(l) * interpolate(&cie::CIE_Z, l)),
    ) / y_integral
}

fn matrix(m: &[[f64; 3]; 3]) -> Matrix3<f64> {
    Matrix3::new(m[0][0], m[0][1], m[0][2], m[1][0], m[1][1], m[1][2], m[2][0], m[2][1], m[2][2])
}

/// Linear sRGB from CIE XYZ.
pub fn xyz_to_srgb(xyz: &Vector3<f64>) -> Color {
    Color { rgb: matrix(&cie::XYZ_TO_SRGB) * xyz }
}

/// CIE XYZ from linear sRGB.
pub fn srgb_to_xyz(c: &Color) -> Vector3<f64> {
    matrix(&cie::SRGB_TO_XYZ) * c.rgb
}

/// A wavelength in nm from a uniform `u`, distributed in proportion to `visible_wavelength_pdf`:
/// pbrt-v4's `SampleVisibleWavelengths` (PBR section 5.4.2), which inverts the cumulative
/// distribution of a sech² fitted to the eye's sensitivity, so that wavelengths it hardly sees
/// are rarely sampled.
pub fn sample_visible_wavelength(u: f64) -> f64 {
    (538. - 138.888889 * (0.85691062 - 1.82750197 * u).atanh()).clamp(LAMBDA_MIN, LAMBDA_MAX)
}

/// The density, per nm, of `sample_visible_wavelength`: p(λ) = 0.0039398042 / cosh²(0.0072 (λ -
/// 538)) over [360, 830], which integrates to one.
pub fn visible_wavelength_pdf(lambda: f64) -> f64 {
    if !(LAMBDA_MIN..=LAMBDA_MAX).contains(&lambda) {
        return 0.;
    }
    0.0039398042 / (0.0072 * (lambda - 538.)).cosh().powi(2)
}

/// The wavelengths a spectral sample carries, and their densities (PBR's
/// `SampledWavelengths`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wavelengths {
    /// In nm; the first is the hero.
    pub lambda: [f64; N],
    pub pdf: [f64; N],
    /// Whether only the hero counts (`terminate_secondary`).
    pub secondary_terminated: bool,
}

impl Wavelengths {
    /// The hero wavelength from `u`, and the others at `u + i / N` (mod 1), so the three are
    /// stratified over the visible distribution.
    pub fn sample_visible(u: f64) -> Wavelengths {
        let mut lambda = [0.; N];
        let mut pdf = [0.; N];
        for i in 0..N {
            let ui = (u + i as f64 / N as f64).fract();
            lambda[i] = sample_visible_wavelength(ui);
            pdf[i] = visible_wavelength_pdf(lambda[i]);
        }
        Wavelengths { lambda, pdf, secondary_terminated: false }
    }

    /// The XYZ tristimulus values estimated from radiance `l` at these wavelengths (see the
    /// module's "Colour").
    pub fn to_xyz(self, l: &Color) -> Vector3<f64> {
        let y_integral = constants().y_integral;
        // Terminated, the hero alone stands for all three: its weight is N times as much.
        let (count, weight) = if self.secondary_terminated { (1, 1.) } else { (N, 1. / N as f64) };
        let mut xyz = Vector3::zeros();
        for i in 0..count {
            if self.pdf[i] > 0. {
                xyz += xyz_matching(self.lambda[i]) * (l.rgb[i] / self.pdf[i]);
            }
        }
        xyz * (weight / y_integral)
    }

    /// Linear sRGB estimated from radiance `l` at these wavelengths.
    pub fn to_srgb(self, l: &Color) -> Color {
        xyz_to_srgb(&self.to_xyz(l))
    }

    /// Values of `f` at these wavelengths.
    pub fn map(&self, f: impl Fn(f64) -> f64) -> Color {
        Color::new(f(self.lambda[0]), f(self.lambda[1]), f(self.lambda[2]))
    }
}

thread_local! {
    /// The wavelengths of the spectral sample this thread is tracing, if any.
    static CURRENT: Cell<Option<Wavelengths>> = const { Cell::new(None) };
}

/// Start tracing a spectral sample at `w`: until `end_sample`, colours on the path are values
/// at these wavelengths.
pub fn begin_sample(w: Wavelengths) {
    CURRENT.with(|c| c.set(Some(w)));
}

/// Finish the spectral sample, returning its wavelengths (with any termination).
pub fn end_sample() -> Option<Wavelengths> {
    CURRENT.with(|c| c.replace(None))
}

/// The wavelengths of the sample being traced, or None in RGB mode.
pub fn current() -> Option<Wavelengths> {
    CURRENT.with(|c| c.get())
}

/// Follow only the hero wavelength from here: the path's direction has come to depend on
/// wavelength (PBR's `TerminateSecondary`).
pub fn terminate_secondary() {
    CURRENT.with(|c| if let Some(mut w) = c.get() {
        w.secondary_terminated = true;
        c.set(Some(w));
    });
}

/// A reflectance given as RGB in [0, 1], at the current wavelengths (PBR's
/// `RGBAlbedoSpectrum`): a smooth spectrum with values in [0, 1] whose colour under D65 is the
/// RGB (`rgb2spec`). Components are clamped to [0, 1].
pub fn reflectance(c: Color) -> Color {
    match current() {
        None => c,
        Some(w) => {
            let s = rgb2spec::table().lookup(c.rgb.map(|v| v.clamp(0., 1.)).into());
            w.map(|l| s.eval(l))
        },
    }
}

/// The spectrum of a non-negative RGB that may exceed one, eg. a scattering coefficient or a
/// path weight (PBR's `RGBUnboundedSpectrum`): a reflectance spectrum for the RGB divided by
/// twice its largest component, scaled back up.
pub fn unbounded(c: Color) -> Color {
    match current() {
        None => c,
        Some(w) => {
            let (s, scale) = unbounded_sigmoid(c);
            w.map(|l| scale * s.eval(l))
        },
    }
}

fn unbounded_sigmoid(c: Color) -> (rgb2spec::Sigmoid, f64) {
    let rgb = c.rgb.map(|v| v.max(0.));
    let m = rgb.max();
    let scale = 2. * m;
    let s = if scale > 0. { rgb2spec::table().lookup((rgb / scale).into()) } else { rgb2spec::Sigmoid::constant(0.) };
    (s, scale)
}

/// The spectrum of light given as RGB (PBR's `RGBIlluminantSpectrum`): the unbounded spectrum
/// of the RGB times illuminant D65, so that RGB (1, 1, 1) is D65 itself, which is white in sRGB.
pub fn illuminant(c: Color) -> Color {
    match current() {
        None => c,
        Some(w) => {
            let (s, scale) = unbounded_sigmoid(c);
            w.map(|l| scale * s.eval(l) * d65(l))
        },
    }
}

impl Color {
    /// This RGB reflectance at the current sample's wavelengths ([`reflectance`]), or itself in
    /// RGB mode.
    pub fn as_reflectance(&self) -> Color { reflectance(*self) }

    /// This RGB emission at the current sample's wavelengths ([`illuminant`]), or itself in RGB
    /// mode.
    pub fn as_illuminant(&self) -> Color { illuminant(*self) }

    /// This non-negative RGB, possibly over one, at the current sample's wavelengths
    /// ([`unbounded`]), or itself in RGB mode.
    pub fn as_unbounded(&self) -> Color { unbounded(*self) }
}

/// Planck's law: the spectral radiance of a blackbody at `kelvin`, at `lambda` nm, in W / (m² sr
/// m) (PBR section 4.4.1, `Blackbody`).
pub fn planck(lambda: f64, kelvin: f64) -> f64 {
    if kelvin <= 0. {
        return 0.;
    }
    const C: f64 = 299_792_458.;
    const H: f64 = 6.626_070_15e-34;
    const KB: f64 = 1.380_649e-23;
    let l = lambda * 1e-9;
    (2. * H * C * C) / (l.powi(5) * ((H * C / (l * KB * kelvin)).exp() - 1.))
}

/// What an emitter emits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Emission {
    /// A colour, upsampled as an illuminant.
    Rgb(Color),
    /// A blackbody at `kelvin`, scaled to a luminance (Y) of one: `scale` is one over the
    /// unscaled luminance, and `srgb` its colour, for RGB mode.
    Blackbody { kelvin: f64, scale: f64, srgb: Color },
}

impl Emission {
    pub fn blackbody(kelvin: f64) -> Emission {
        assert!(kelvin > 0., "A blackbody's temperature must be positive, got {}", kelvin);
        let xyz = spectrum_to_xyz(|l| planck(l, kelvin));
        let scale = 1. / xyz.y;
        Emission::Blackbody { kelvin, scale, srgb: xyz_to_srgb(&(xyz * scale)) }
    }

    /// The emission's colour in linear sRGB.
    pub fn srgb(&self) -> Color {
        match self {
            Emission::Rgb(c) => *c,
            Emission::Blackbody { srgb, .. } => *srgb,
        }
    }

    /// The emission at the current wavelengths, or in RGB mode its colour.
    pub fn radiance(&self) -> Color {
        match (self, current()) {
            (_, None) => self.srgb(),
            (Emission::Rgb(c), Some(_)) => illuminant(*c),
            (Emission::Blackbody { kelvin, scale, .. }, Some(w)) => w.map(|l| planck(l, *kelvin) * scale),
        }
    }
}

/// A refractive index that may vary with wavelength.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefractiveIndex {
    Constant(f64),
    /// Cauchy's equation, n(λ) = a + b / λ², λ in micrometres: a fit for glass across the
    /// visible (Augustin-Louis Cauchy, 1836).
    Cauchy { a: f64, b: f64 },
    /// The Sellmeier equation, n²(λ) = 1 + Σ b_i λ² / (λ² - c_i), λ in micrometres, c_i in µm²
    /// (Wilhelm Sellmeier, 1871): the form glass catalogues give their fits in.
    Sellmeier { b: [f64; 3], c: [f64; 3] },
}

impl RefractiveIndex {
    /// Sellmeier coefficients of a named glass, with n_d in brackets:
    /// - `BK7`: Schott N-BK7 borosilicate crown, the commonest optical glass (1.5168); Schott
    ///   optical glass data sheet.
    /// - `fused silica` (1.4585): I. H. Malitson, "Interspecimen Comparison of the Refractive
    ///   Index of Fused Silica", JOSA 55(10), 1965, pp. 1205-1209, doi:10.1364/JOSA.55.001205.
    /// - `SF11`: Schott SF11 dense flint, strongly dispersive (1.7847); Schott data sheet.
    /// - `diamond` (2.4175): H. Peter, "Über Brechungsindizes und Absorptionskonstanten des
    ///   Diamanten zwischen 644 und 226 mμ", Zeitschrift für Physik 15, 1923, pp. 358-368.
    pub fn glass(name: &str) -> Option<RefractiveIndex> {
        let (b, c) = match name {
            "BK7" => ([1.03961212, 0.231792344, 1.01046945], [0.00600069867, 0.0200179144, 103.560653]),
            "fused silica" => ([0.6961663, 0.4079426, 0.8974794], [0.0684043f64.powi(2), 0.1162414f64.powi(2), 9.896161f64.powi(2)]),
            "SF11" => ([1.73759695, 0.313747346, 1.89878101], [0.013188707, 0.0623068142, 155.23629]),
            "diamond" => ([0.3306, 4.3356, 0.], [0.175f64.powi(2), 0.106f64.powi(2), 0.]),
            _ => return None,
        };
        Some(RefractiveIndex::Sellmeier { b, c })
    }

    /// The index at `lambda` nm.
    pub fn at(&self, lambda: f64) -> f64 {
        let um = lambda * 1e-3;
        let l2 = um * um;
        match self {
            RefractiveIndex::Constant(n) => *n,
            RefractiveIndex::Cauchy { a, b } => a + b / l2,
            RefractiveIndex::Sellmeier { b, c } => {
                (1. + (0..3).map(|i| b[i] * l2 / (l2 - c[i])).sum::<f64>()).sqrt()
            },
        }
    }

    pub fn is_dispersive(&self) -> bool {
        !matches!(self, RefractiveIndex::Constant(_))
    }

    /// The index for the sample being traced: at the hero wavelength of a spectral sample (and,
    /// if it varies with wavelength, following the hero alone from here: `terminate_secondary`),
    /// and otherwise at the d line.
    pub fn current(&self) -> f64 {
        match (self, current()) {
            (RefractiveIndex::Constant(n), _) => *n,
            (_, None) => self.at(D_LINE),
            (_, Some(w)) => {
                terminate_secondary();
                self.at(w.lambda[0])
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    #[test]
    fn test_visible_wavelength_pdf_integrates_to_one() {
        let total = integrate(visible_wavelength_pdf);
        assert!((total - 1.).abs() < 1e-3, "{}", total);
    }

    #[test]
    fn test_visible_wavelengths_follow_their_pdf() {
        // The fraction sampled below 500 nm matches the density's integral there.
        let n = 200_000;
        let below = (0..n).filter(|i| sample_visible_wavelength((*i as f64 + 0.5) / n as f64) < 500.).count();
        let expected = integrate(|l| if l < 500. { visible_wavelength_pdf(l) } else { 0. });
        assert!((below as f64 / n as f64 - expected).abs() < 2e-3, "{} vs {}", below as f64 / n as f64, expected);
    }

    #[test]
    fn test_d65_is_white() {
        let xyz = spectrum_to_xyz(d65);
        assert!((xyz.y - 1.).abs() < 1e-9);
        let rgb = xyz_to_srgb(&xyz);
        for c in 0..3 {
            assert!((rgb.rgb[c] - 1.).abs() < 2e-3, "{:?}", rgb);
        }
    }

    /// The film's estimate of a spectrum's XYZ, averaged over many samples, is its XYZ.
    #[test]
    fn test_sampled_xyz_is_unbiased() {
        let f = |l: f64| 1. + ((l - 400.) / 60.).sin();
        let exact = spectrum_to_xyz(f);
        let mut rng = Pcg32::new(3, 7);
        let n = 200_000;
        let mut sum = Vector3::zeros();
        for _ in 0..n {
            let w = Wavelengths::sample_visible(rng.next_f64());
            sum += w.to_xyz(&w.map(f));
        }
        let mean = sum / n as f64;
        assert!((mean - exact).norm() < 0.01 * exact.norm(), "{} vs {}", mean, exact);
    }

    /// Terminating the secondary wavelengths leaves the estimate unbiased.
    #[test]
    fn test_terminated_xyz_is_unbiased() {
        let f = |l: f64| (l - 300.) / 500.;
        let exact = spectrum_to_xyz(f);
        let mut rng = Pcg32::new(5, 1);
        let n = 400_000;
        let mut sum = Vector3::zeros();
        for _ in 0..n {
            let mut w = Wavelengths::sample_visible(rng.next_f64());
            w.secondary_terminated = true;
            sum += w.to_xyz(&w.map(f));
        }
        let mean = sum / n as f64;
        assert!((mean - exact).norm() < 0.01 * exact.norm(), "{} vs {}", mean, exact);
    }

    #[test]
    fn test_rgb_mode_is_unchanged() {
        let c = Color::new(0.2, 0.7, 3.);
        assert_eq!(c.as_reflectance(), c);
        assert_eq!(c.as_illuminant(), c);
        assert_eq!(c.as_unbounded(), c);
    }

    /// An emitter's RGB and a reflectance's RGB survive the round trip through spectra: white
    /// light of colour E on a surface of reflectance ρ looks like E ρ when both are grey, and
    /// like ρ under white light.
    #[test]
    fn test_upsampled_spectra_have_their_colours() {
        for rgb in [Color::new(0.8, 0.2, 0.1), Color::new(0.1, 0.5, 0.9), Color::new(0.3, 0.3, 0.3), Color::new(0.05, 0.9, 0.2)] {
            let s = rgb2spec::table().lookup(rgb.rgb.into());
            let seen = xyz_to_srgb(&spectrum_to_xyz(|l| s.eval(l) * d65(l)));
            assert!((seen.rgb - rgb.rgb).norm() < 0.01, "{:?} came back as {:?}", rgb, seen);
        }
        // Lights, which may exceed one.
        let light = Color::new(17., 12., 4.);
        let mut rng = Pcg32::new(1, 2);
        let n = 100_000;
        let mut sum = Color::black();
        for _ in 0..n {
            let w = Wavelengths::sample_visible(rng.next_f64());
            begin_sample(w);
            let l = light.as_illuminant();
            end_sample();
            sum = sum + w.to_srgb(&l);
        }
        let mean = sum / n as f64;
        assert!((mean.rgb - light.rgb).norm() < 0.02 * light.rgb.norm(), "{:?}", mean);
    }

    #[test]
    fn test_blackbody() {
        // Wien's displacement law: the peak is at b / T, b = 2.898e-3 m K.
        let t = 5000.;
        let peak = (300..1500).map(|l| l as f64).max_by(|a, b| planck(*a, t).total_cmp(&planck(*b, t))).unwrap();
        assert!((peak - 2.897771955e6 / t).abs() < 1., "{}", peak);
        // Normalised to unit luminance; warm at low temperatures, cool at high.
        let warm = Emission::blackbody(2700.).srgb();
        let cool = Emission::blackbody(10000.).srgb();
        assert!(warm.rgb.x > warm.rgb.z && cool.rgb.z > cool.rgb.x, "{:?} {:?}", warm, cool);
        let y = srgb_to_xyz(&warm).y;
        assert!((y - 1.).abs() < 1e-3, "{}", y);
        // About 6500 K is near-white in sRGB, whose white is D65 (correlated colour
        // temperature 6504 K).
        let d = Emission::blackbody(6504.).srgb();
        assert!((d.rgb.x / d.rgb.z - 1.).abs() < 0.15, "{:?}", d);
    }

    #[test]
    fn test_glass_indices() {
        for (name, nd) in [("BK7", 1.5168), ("fused silica", 1.4585), ("SF11", 1.7847), ("diamond", 2.4175)] {
            let n = RefractiveIndex::glass(name).unwrap();
            assert!((n.at(D_LINE) - nd).abs() < 1e-3, "{} n_d = {}", name, n.at(D_LINE));
            // Normal dispersion: blue bends more than red.
            assert!(n.at(450.) > n.at(650.));
        }
        assert!(RefractiveIndex::glass("unobtainium").is_none());
        assert!(!RefractiveIndex::Constant(1.5).is_dispersive());
    }

    #[test]
    fn test_dispersive_index_terminates_secondary_wavelengths() {
        let glass = RefractiveIndex::glass("BK7").unwrap();
        assert!((glass.current() - glass.at(D_LINE)).abs() < 1e-12);
        let w = Wavelengths::sample_visible(0.3);
        begin_sample(w);
        assert_eq!(RefractiveIndex::Constant(1.5).current(), 1.5);
        assert!(!current().unwrap().secondary_terminated);
        let n = glass.current();
        let after = end_sample().unwrap();
        assert!(after.secondary_terminated);
        assert_eq!(n, glass.at(w.lambda[0]));
    }
}
