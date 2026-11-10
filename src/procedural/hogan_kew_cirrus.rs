//! Cirrus: high, thin clouds of ice crystals, after the fallstreak model of Robin J. Hogan and
//! Sarah F. Kew, "A 3D stochastic cloud model for investigating the radiative properties of
//! inhomogeneous cirrus clouds", Quarterly Journal of the Royal Meteorological Society 131,
//! 2005 (<https://centaur.reading.ac.uk/5165>).
//!
//! Cirrus forms in small "generating cells" near the top of its layer, where ice crystals
//! grow. The crystals then fall out of them, at around a metre a second, through air whose
//! wind changes with height. Falling from a cell moving with the wind at the generating level,
//! a crystal that has fallen a depth Z has drifted, relative to its cell,
//!
//! ```text
//! Δ(Z) = ∫ (u(z) - u_gen) dt = s Z² / (2 v)
//! ```
//!
//! for a wind shear s = du/dz (wind growing with height) and fall speed v. Crystals trail
//! behind their cells in a curve, steepening with depth: the hooked "mares' tails" of cirrus
//! uncinus, or long streaks (fibratus) when the cells are themselves stretched by the wind.
//! Below the generating level the air is drier, and the crystals sublimate as they fall.
//!
//! Hogan and Kew fit the statistics of the generating field to radar: a power-law spectrum
//! of fluctuations, as for turbulence, cut off at an outer scale, and stretched along the
//! wind. Here, as in their model, the field is synthesised by an inverse FFT of random
//! amplitudes with that spectrum, then thresholded to the requested coverage. Each point's
//! density is the generating field at its source, traced back along the fall:
//!
//! ```text
//! density(p) = σ × profile(Z) × G(x - u_gen t + Δ(Z) ŵ)
//! ```
//!
//! averaged over a spread of fall speeds (crystals of different sizes fall at different
//! speeds, which smears each streak along the wind), with fine detail stretched along the
//! wind, and curl noise (Bridson, Hourihan and Nordenstam, "Curl-Noise for Procedural Fluid
//! Flow", SIGGRAPH 2007) growing with depth for the turbulence the crystals fall through.
//!
//! Animation: the cells drift with the wind at the generating level, and evolve. The FFT
//! gives two independent fields with the same spectrum (its real and imaginary parts), and
//! cos(ωt) f₁ + sin(ωt) f₂ has the same statistics at every t while changing continuously, so
//! cells grow and fade without the sky as a whole changing character.

use crate::geometry::Pcg32;
use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::volume::DensityField;
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

/// Points in the generating field's FFT, along each side.
const FIELD_SIZE: usize = 1024;
/// Texels of the generating field per `cell_size`.
const TEXELS_PER_CELL: f64 = 6.;
/// Fall speeds averaged per lookup, spread evenly across ± `fall_speed_spread`.
const FALL_SPEEDS: usize = 4;

/// The kinds of cirrus, as starting points for the parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CirrusGenus {
    /// Cirrus fibratus and uncinus: sparse cells, strong shear, long hooked streaks.
    Cirrus,
    /// A thin, nearly uniform veil (the one that makes halos).
    Cirrostratus,
    /// Small, regular cells in a thin layer, falling little: a fine "mackerel sky".
    Cirrocumulus,
}

/// A periodic 2D Gaussian random field with a power-law spectrum, in two independent
/// realisations, normalised to zero mean and unit variance.
struct GeneratingField {
    texel: f64,
    real: Vec<f64>,
    imaginary: Vec<f64>,
}

impl GeneratingField {
    /// Power spectral density ∝ (k_e² + k₀²)^(-(β + 1)/2) in two dimensions, for a 1D slope
    /// of -β (5/3 for turbulence), outer scale 2π / k₀ = `outer_scale`, with the wavenumber
    /// along `wind` multiplied by `anisotropy` in k_e, so structures are that much longer along
    /// the wind than across it.
    fn new(seed: u64, texel: f64, outer_scale: f64, slope: f64, anisotropy: f64, wind: Vector2<f64>) -> GeneratingField {
        let n = FIELD_SIZE;
        let mut rng = Pcg32::new(seed, 7);
        let mut gaussian = || {
            // Box-Muller.
            let (u, v) = (1. - rng.next_f64(), rng.next_f64());
            (-2. * u.ln()).sqrt() * (2. * PI * v).cos()
        };
        let k0 = 2. * PI / outer_scale;
        let dk = 2. * PI / (n as f64 * texel);
        let mut spectrum = vec![Complex::new(0., 0.); n * n];
        for j in 0..n {
            for i in 0..n {
                // Frequencies from -n/2, with 0 at index 0 as the FFT expects.
                let fi = if i <= n / 2 { i as f64 } else { i as f64 - n as f64 };
                let fj = if j <= n / 2 { j as f64 } else { j as f64 - n as f64 };
                let k = Vector2::new(fi, fj) * dk;
                let along = k.dot(&wind) * anisotropy;
                let across = k.dot(&Vector2::new(-wind.y, wind.x));
                let power = (along * along + across * across + k0 * k0).powf(-(slope + 1.) / 2.);
                let amplitude = if i == 0 && j == 0 { 0. } else { power.sqrt() };
                spectrum[j * n + i] = Complex::new(gaussian(), gaussian()) * amplitude;
            }
        }
        let field = crate::ocean::ifft2(spectrum, n);
        let normalise = |v: Vec<f64>| {
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
            v.into_iter().map(|x| (x - mean) / sd).collect::<Vec<f64>>()
        };
        GeneratingField {
            texel,
            real: normalise(field.iter().map(|c| c.re).collect()),
            imaginary: normalise(field.iter().map(|c| c.im).collect()),
        }
    }

    /// The field at (x, z), bilinearly interpolated and periodic; `phase` rotates between the
    /// two realisations.
    fn at(&self, x: f64, z: f64, phase: (f64, f64)) -> f64 {
        let n = FIELD_SIZE as i64;
        let (u, v) = (x / self.texel, z / self.texel);
        let (i0, j0) = (u.floor(), v.floor());
        let (fu, fv) = (u - i0, v - j0);
        let (i0, j0) = (i0 as i64, j0 as i64);
        let texel = |di: i64, dj: i64| {
            let idx = ((j0 + dj).rem_euclid(n) * n + (i0 + di).rem_euclid(n)) as usize;
            phase.0 * self.real[idx] + phase.1 * self.imaginary[idx]
        };
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        lerp(lerp(texel(0, 0), texel(1, 0), fu), lerp(texel(0, 1), texel(1, 1), fu), fv)
    }

    /// The value exceeded by a fraction `coverage` of the field.
    fn quantile_above(&self, coverage: f64) -> f64 {
        let mut sorted = self.real.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let i = (((1. - coverage) * sorted.len() as f64) as usize).min(sorted.len() - 1);
        sorted[i]
    }
}

pub struct HoganKewCirrus {
    /// Heights of the bottom and top of the layer; crystals generate near the top.
    base: f64,
    top: f64,
    /// Extinction coefficient in the thickest ice (per unit distance).
    pub max_density: f64,
    /// Fraction of the generating level that is cloud, 0 to 1.
    coverage: f64,
    /// Fraction of the layer's depth, from the top, in which crystals generate and grow.
    pub generating_depth: f64,
    /// How far down the layer (as a fraction of its depth, from the top) ice survives before it
    /// has all sublimated, and how the profile falls towards there: as (1 - x)^sublimation,
    /// for x the fraction of the way from the generating depth to `fall_depth`.
    pub fall_depth: f64,
    pub sublimation: f64,
    /// Unit vector (x, z) of the wind; shear du/dz (per second, wind growing with height); the
    /// wind speed at the generating level (scene units per second).
    wind: Vector2<f64>,
    pub shear: f64,
    pub generating_wind: f64,
    /// Mean fall speed of the crystals, and its relative spread across crystal sizes.
    pub fall_speed: f64,
    pub fall_speed_spread: f64,
    /// Width of the soft edge of the cells, in standard deviations of the field.
    pub edge: f64,
    /// Size of the fine detail, and how much it modulates the density (0 to 1).
    pub detail_size: f64,
    pub detail: f64,
    /// Size of the curl-noise eddies, and how far (scene units) they displace the falling ice
    /// by the base of the layer.
    pub turbulence_size: f64,
    pub turbulence: f64,
    /// Time (seconds), and how long cells take to evolve through one cycle (∞ to freeze them).
    pub time: f64,
    pub evolution_period: f64,
    /// How much a larger-scale field breaks the cells into patches (0 none, 1 fully), and its
    /// size: cirrocumulus comes in patches of ripples, not an even grain.
    pub patchiness: f64,
    pub patch_size: f64,
    /// The square, `extent` either side of `center` in x and z, that the layer fills. The
    /// density fades out over its outer `fade` (a fraction of `extent`), so the layer has no
    /// visible edge.
    pub center: Vector2<f64>,
    pub extent: f64,
    pub fade: f64,
    field: GeneratingField,
    threshold: f64,
    detail_noise: PerlinNoise,
    curl_noise: PerlinNoise,
}

/// Parameters fixing the generating field, which is synthesised once.
pub struct CirrusField {
    pub genus: CirrusGenus,
    pub base: f64,
    pub top: f64,
    pub coverage: f64,
    /// Outer scale of the generating field: about the size of the largest cells.
    pub cell_size: f64,
    /// Slope of the 1D spectrum, -β: 5/3 for turbulence; steeper is smoother.
    pub spectral_slope: f64,
    /// How much longer cells are along the wind than across it.
    pub anisotropy: f64,
    /// Direction the wind blows towards, (x, z).
    pub wind_direction: Vector2<f64>,
    pub seed: u64,
}

impl CirrusField {
    /// Typical parameters for `genus`, in metres and seconds, for a layer from 8 to 10 km.
    pub fn new(genus: CirrusGenus) -> CirrusField {
        let (coverage, cell_size, spectral_slope, anisotropy) = match genus {
            CirrusGenus::Cirrus => (0.25, 2500., 5. / 3., 3.),
            CirrusGenus::Cirrostratus => (0.9, 6000., 2.5, 1.5),
            CirrusGenus::Cirrocumulus => (0.5, 400., 5. / 3., 1.),
        };
        CirrusField {
            genus, base: 8000., top: 10000., coverage, cell_size, spectral_slope, anisotropy,
            wind_direction: Vector2::new(1., 0.), seed: 0,
        }
    }
}

impl HoganKewCirrus {
    pub fn new(f: CirrusField) -> HoganKewCirrus {
        assert!(f.top > f.base, "The cirrus layer's top ({}) must be above its base ({})", f.top, f.base);
        assert!(f.wind_direction.norm() > 0., "The wind direction must not be zero");
        let wind = f.wind_direction.normalize();
        let field = GeneratingField::new(f.seed, f.cell_size / TEXELS_PER_CELL, f.cell_size,
            f.spectral_slope, f.anisotropy, wind);
        let threshold = field.quantile_above(f.coverage.clamp(1e-6, 1.));
        let depth = f.top - f.base;
        // Extinction: cirrus is optically thin, an optical depth of about 0.03 to 3 overhead;
        // cirrostratus a veil the sun shines through.
        let (max_density, generating_depth, fall_depth, sublimation, shear, fall_speed, detail, turbulence, patchiness)
            = match f.genus {
            // 10 m/s per km of shear, crystals falling 1 m/s: streaks trailing kilometres.
            CirrusGenus::Cirrus => (1e-3, 0.15, 1., 1.5, 0.01, 1., 0.5, 0.15 * depth, 0.3),
            CirrusGenus::Cirrostratus => (1.5e-4, 0.4, 1., 1., 0.003, 0.7, 0.3, 0.1 * depth, 0.3),
            // Barely falling, and soon gone: compact cells in a thin layer near the top.
            CirrusGenus::Cirrocumulus => (1.5e-3, 0.12, 0.25, 2., 0.002, 0.2, 0.4, 0.02 * depth, 0.8),
        };
        HoganKewCirrus {
            base: f.base,
            top: f.top,
            max_density,
            coverage: f.coverage,
            generating_depth,
            fall_depth,
            sublimation,
            wind,
            shear,
            generating_wind: 20.,
            fall_speed,
            fall_speed_spread: 0.5,
            edge: 0.6,
            detail_size: f.cell_size / 8.,
            detail,
            turbulence_size: f.cell_size,
            turbulence,
            time: 0.,
            evolution_period: f64::INFINITY,
            patchiness,
            patch_size: f.cell_size * 12.,
            center: Vector2::zeros(),
            extent: f64::INFINITY,
            fade: 0.3,
            field,
            threshold,
            detail_noise: PerlinNoise::with_seed(f.seed.wrapping_mul(3).wrapping_add(1)),
            curl_noise: PerlinNoise::with_seed(f.seed.wrapping_mul(3).wrapping_add(2)),
        }
    }

    /// The fraction of the generating level that is cloud.
    pub fn coverage(&self) -> f64 {
        self.coverage
    }

    /// The vertical profile at a depth `d` (0 at the top, 1 at the base, as a fraction): ice
    /// grows through the generating depth, then sublimates as it falls.
    fn profile(&self, d: f64) -> f64 {
        let g = self.generating_depth.min(self.fall_depth);
        if d < g {
            return (d / g).sqrt();
        }
        let x = (d - g) / (self.fall_depth - g).max(1e-9);
        (1. - x).clamp(0., 1.).powf(self.sublimation)
    }

    /// The lowest height with any ice.
    fn lowest(&self) -> f64 {
        self.top - self.fall_depth.min(1.) * (self.top - self.base)
    }

    /// The cells' strength (0 to 1) at the generating level, at (x, z) in the cells' own,
    /// drifting frame.
    fn cells(&self, x: f64, z: f64) -> f64 {
        let w = if self.evolution_period.is_finite() { 2. * PI * self.time / self.evolution_period } else { 0. };
        let f = self.field.at(x, z, (w.cos(), w.sin()));
        ((f - self.threshold) / self.edge).clamp(0., 1.)
    }

    /// The horizontal displacement (x, z) of curl noise: the curl of a scalar potential is
    /// divergence-free, so it swirls the ice without bunching or thinning it.
    fn curl(&self, p: &Vector3<f64>) -> Vector2<f64> {
        let s = self.turbulence_size;
        let q = p / s;
        let h = 1e-3;
        let psi = |dx: f64, dz: f64| self.curl_noise.noise(q.x + dx, q.y, q.z + dz);
        let dpsi_dx = (psi(h, 0.) - psi(-h, 0.)) / (2. * h);
        let dpsi_dz = (psi(0., h) - psi(0., -h)) / (2. * h);
        Vector2::new(dpsi_dz, -dpsi_dx)
    }

    /// Ice density as a fraction of `max_density`, in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.base || p.y > self.top {
            return 0.;
        }
        let fallen = self.top - p.y;
        let d = fallen / (self.top - self.base);
        let profile = self.profile(d);
        if profile <= 0. {
            return 0.;
        }
        // Into the cells' frame, in which the pattern of fibres and eddies travels too, and
        // back along the fall.
        let mut x = Vector2::new(p.x, p.z) - self.wind * (self.generating_wind * self.time);
        let framed = Vector3::new(x.x, p.y, x.y);
        if self.turbulence > 0. {
            x += self.curl(&framed) * (self.turbulence * d);
        }
        let edge_fade = {
            let from_centre = (Vector2::new(p.x, p.z) - self.center).abs().max() / self.extent;
            let t = ((1. - from_centre) / self.fade.max(1e-9)).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        if edge_fade <= 0. {
            return 0.;
        }
        let mut cells = 0.;
        for i in 0..FALL_SPEEDS {
            let spread = (i as f64 + 0.5) / FALL_SPEEDS as f64 * 2. - 1.;
            let v = self.fall_speed * (1. + self.fall_speed_spread * spread);
            let drift = self.shear * fallen * fallen / (2. * v);
            let source = x + self.wind * drift;
            cells += self.cells(source.x, source.y);
        }
        cells /= FALL_SPEEDS as f64;
        if cells <= 0. {
            return 0.;
        }
        // Fine fibres along the wind: noise stretched four times along it.
        let along = Vector2::new(framed.x, framed.z).dot(&self.wind);
        let across = Vector2::new(-self.wind.y, self.wind.x).dot(&Vector2::new(framed.x, framed.z));
        let s = self.detail_size;
        let fibre = self.detail_noise.fbm(along / (4. * s), p.y / s, across / s, 4, 0.5, 2.);
        let detail = 1. - self.detail * (1. - (2. * fibre).min(1.));
        let patches = if self.patchiness > 0. {
            let q = Vector2::new(framed.x, framed.z) / self.patch_size;
            let n = self.detail_noise.fbm(q.x, 17.5, q.y, 3, 0.5, 2.);
            1. - self.patchiness * (1. - ((n - 0.4) * 5.).clamp(0., 1.))
        } else {
            1.
        };
        (edge_fade * profile * cells * detail * patches).clamp(0., 1.)
    }
}

impl DensityField for HoganKewCirrus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.max_density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }

    /// Nothing below `lowest`: zero there until the ray climbs to it.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let lowest = self.lowest();
        let to_lowest = if d.y != 0. { (lowest - p.y) / d.y } else { f64::INFINITY };
        let ahead = p.y + d.y * 1e-9 * (self.top - self.base);
        if ahead < lowest {
            (0., if to_lowest > 0. { to_lowest } else { f64::INFINITY })
        } else {
            (self.max_density, if to_lowest > 0. { to_lowest } else { f64::INFINITY })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cirrus(genus: CirrusGenus, seed: u64) -> HoganKewCirrus {
        let mut f = CirrusField::new(genus);
        f.seed = seed;
        HoganKewCirrus::new(f)
    }

    /// Fraction of a grid of points at height y that have any ice.
    fn cover_at(c: &HoganKewCirrus, y: f64) -> f64 {
        let n = 80;
        let hits = (0..n * n).filter(|i| {
            c.fraction(&Vector3::new((i % n) as f64 * 731., y, (i / n) as f64 * 677.)) > 0.
        }).count();
        hits as f64 / (n * n) as f64
    }

    #[test]
    fn test_no_ice_outside_the_layer_and_bounded() {
        let c = cirrus(CirrusGenus::Cirrus, 1);
        assert_eq!(cover_at(&c, 7999.), 0.);
        assert_eq!(cover_at(&c, 10001.), 0.);
        for i in 0..5000 {
            let p = Vector3::new(i as f64 * 37.7, 8000. + (i % 97) as f64 * 20.6, (i / 7) as f64 * 53.1);
            let d = c.density(&p);
            assert!((0. ..=c.majorant()).contains(&d), "{} at {}", d, p);
        }
    }

    #[test]
    fn test_coverage_at_the_generating_level() {
        // With a hard edge, no detail or turbulence, and one fall speed, the fraction of cloud
        // just below the top is the coverage.
        for coverage in [0.2, 0.6] {
            let mut f = CirrusField::new(CirrusGenus::Cirrus);
            f.coverage = coverage;
            let mut c = HoganKewCirrus::new(f);
            c.edge = 1e-9;
            c.turbulence = 0.;
            c.detail = 0.;
            c.fall_speed_spread = 0.;
            c.patchiness = 0.;
            let got = cover_at(&c, 9999.);
            assert!((got - coverage).abs() < 0.05, "coverage {}: {}", coverage, got);
        }
    }

    #[test]
    fn test_fallstreaks_trail_by_shear_over_fall_speed() {
        // Ice at depth Z is the ice that left the cell upwind by s Z² / 2v: the density there
        // is the generating level's, shifted (and scaled by the profile).
        let mut c = cirrus(CirrusGenus::Cirrus, 2);
        c.turbulence = 0.;
        c.detail = 0.;
        c.fall_speed_spread = 0.;
        c.patchiness = 0.;
        let depth = 1200.;
        let drift = c.shear * depth * depth / (2. * c.fall_speed);
        assert!(drift > 5000., "{}", drift);
        let profile = c.profile(depth / 2000.);
        for i in 0..400 {
            let (x, z) = (i as f64 * 211., (i % 13) as f64 * 389.);
            let below = c.fraction(&Vector3::new(x, 10000. - depth, z));
            let source = c.cells(x + drift, z);
            assert!((below - profile * source).abs() < 1e-9, "{} vs {}", below, profile * source);
        }
    }

    #[test]
    fn test_cells_drift_with_the_wind() {
        let mut c = cirrus(CirrusGenus::Cirrus, 3);
        c.turbulence = 0.;
        let p = |x: f64| Vector3::new(x, 9500., 1234.);
        let before: Vec<f64> = (0..200).map(|i| c.fraction(&p(i as f64 * 97.))).collect();
        c.time = 300.;
        let shift = c.generating_wind * 300.;
        let after: Vec<f64> = (0..200).map(|i| c.fraction(&p(i as f64 * 97. + shift))).collect();
        for (a, b) in before.iter().zip(&after) {
            assert!((a - b).abs() < 1e-9);
        }
    }

    #[test]
    fn test_cells_evolve_but_keep_their_coverage() {
        let mut c = cirrus(CirrusGenus::Cirrus, 4);
        c.evolution_period = 3600.;
        c.generating_wind = 0.;
        let start = cover_at(&c, 9900.);
        let row = |c: &HoganKewCirrus| (0..400).map(|i| c.fraction(&Vector3::new(i as f64 * 113., 9900., 0.))).collect::<Vec<f64>>();
        let a = row(&c);
        c.time = 900.;
        let quarter = cover_at(&c, 9900.);
        assert_ne!(a, row(&c));
        assert!((quarter - start).abs() < 0.08, "{} vs {}", quarter, start);
        c.time = 3600.;
        let again = row(&c);
        for (x, y) in a.iter().zip(&again) {
            assert!((x - y).abs() < 1e-9);
        }
    }

    #[test]
    fn test_cells_are_longer_along_the_wind() {
        // Correlation of the generating field at one cell size along the wind versus across.
        let c = cirrus(CirrusGenus::Cirrus, 5);
        let lag = 1500.;
        let corr = |dx: f64, dz: f64| {
            let (mut sxy, mut n) = (0., 0.);
            for i in 0..4000 {
                let (x, z) = ((i % 63) as f64 * 977., (i / 63) as f64 * 1013.);
                sxy += c.field.at(x, z, (1., 0.)) * c.field.at(x + dx, z + dz, (1., 0.));
                n += 1.;
            }
            sxy / n
        };
        let (along, across) = (corr(lag, 0.), corr(0., lag));
        assert!(along > across + 0.1, "along {} across {}", along, across);
    }

    #[test]
    fn test_genera_differ_in_depth() {
        // Cirrocumulus barely falls: its ice stays near the top, while cirrus trails down.
        let cc = cirrus(CirrusGenus::Cirrocumulus, 6);
        let ci = cirrus(CirrusGenus::Cirrus, 6);
        assert!(cover_at(&cc, 9900.) > 0.2);
        assert!(cover_at(&cc, 8800.) < 0.02, "{}", cover_at(&cc, 8800.));
        assert!(cover_at(&ci, 8800.) > 0.1, "{}", cover_at(&ci, 8800.));
        // Cirrostratus is a near-continuous veil.
        assert!(cover_at(&cirrus(CirrusGenus::Cirrostratus, 6), 9500.) > 0.7);
    }

    #[test]
    fn test_local_majorant_bounds_density() {
        let c = cirrus(CirrusGenus::Cirrocumulus, 9);
        let mut rng = Pcg32::new(2, 0);
        let mut skipped = false;
        for _ in 0..2000 {
            let p = Vector3::new(rng.next_f64() * 20000., 7500. + rng.next_f64() * 3000., rng.next_f64() * 20000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = c.majorant_from(&p, &d);
            skipped |= m == 0. && len > 0.;
            for k in 0..20 {
                let q = p + d * (len.min(4000.) * k as f64 / 20.);
                assert!(c.density(&q) <= m, "{} > {} at {}", c.density(&q), m, q);
            }
        }
        assert!(skipped);
    }

    #[test]
    fn test_fades_out_at_the_edge_of_its_region() {
        let mut c = cirrus(CirrusGenus::Cirrostratus, 10);
        c.extent = 20000.;
        assert_eq!(c.fraction(&Vector3::new(20001., 9500., 0.)), 0.);
        let inner = (0..200).map(|i| c.fraction(&Vector3::new(i as f64 * 50., 9500., 1000.))).sum::<f64>();
        let outer = (0..200).map(|i| c.fraction(&Vector3::new(19000. + i as f64 * 5., 9500., 1000.))).sum::<f64>();
        assert!(inner > 50. && outer < 0.3 * inner, "outer {} inner {}", outer, inner);
    }

    #[test]
    fn test_seeded() {
        let row = |c: HoganKewCirrus| (0..300).map(|i| c.fraction(&Vector3::new(i as f64 * 211., 9600., 0.))).collect::<Vec<f64>>();
        assert_eq!(row(cirrus(CirrusGenus::Cirrus, 7)), row(cirrus(CirrusGenus::Cirrus, 7)));
        assert_ne!(row(cirrus(CirrusGenus::Cirrus, 7)), row(cirrus(CirrusGenus::Cirrus, 8)));
    }
}
