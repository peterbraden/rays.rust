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
//! behind their cells in a curve, steepening with depth. The wind usually turns with height
//! too (`veer`), and then the trail hooks round sideways as it falls: the commas and "mares'
//! tails" of cirrus uncinus. For a wind slowing linearly downwards and turning at a constant
//! rate the drift has a closed form (`wind_integral`). Below the generating level the air is
//! drier, and the crystals sublimate as they fall.
//!
//! Hogan and Kew fit the statistics of the generating field to radar: a power-law spectrum
//! of fluctuations, as for turbulence, cut off at an outer scale, and stretched along the
//! wind. Here, as in their model, the field is synthesised by an inverse FFT of random
//! amplitudes with that spectrum, then thresholded to the requested coverage. Each point's
//! density is the generating field at its source, traced back along the fall:
//!
//! ```text
//! density(p) = σ × profile(Z) × G(warp(x - u_gen t - Δ(Z))) × fibres(...)
//! ```
//!
//! with the fall speed varying from point to point (crystals of different sizes fall at
//! different speeds, which frays each streak), and three things that make cirrus wispy and
//! curved rather than streaky and solid:
//!
//! - Fibres. Within a cell, the ice generates in small elements a couple of hundred metres
//!   across, and each trails into a strand along its own fall. The elements are fine,
//!   direction-less noise at the generating level, looked up at each point's source, so the
//!   strands follow their streaks' curves by construction, with clear air between them.
//! - Large-scale flow: a divergence-free warp of the whole pattern, kilometres across, which
//!   bends cells, streaks and fibres together into arcs.
//! - Turbulence: curl noise (Bridson, Hourihan and Nordenstam, "Curl-Noise for Procedural
//!   Fluid Flow", SIGGRAPH 2007) growing with depth, for the eddies the crystals fall through.
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
    /// How far (radians) the wind turns between the top of the layer and its base. With speed
    /// shear alone, ice trails straight behind its cell; when the wind's direction changes with
    /// height too, the trail hooks round: the commas and hooks of cirrus uncinus.
    pub veer: f64,
    /// Width of the soft edge of the cells, in standard deviations of the field.
    pub edge: f64,
    /// The large-scale flow that bends cirrus into arcs: a divergence-free warp of the
    /// horizontal, curl noise `flow_size` across, displacing by up to about `flow`.
    pub flow_size: f64,
    pub flow: f64,
    /// Fibres: within a cell, ice generates in small elements about `fibre_width` across, each
    /// of which trails into a strand of that width along its own fall; `fibres` (0 to 1) is
    /// how completely the ice is gathered into them, from none (smooth cells) to all of it
    /// (wisps with clear air between).
    pub fibre_width: f64,
    pub fibres: f64,
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
    /// Mean radius (µm) of the ice crystals at the generating level; how much larger, as a
    /// fraction, they have grown by the base; and their relative size at the cells' edges,
    /// where the ice is newest. Only diffraction depends on them: small, uniform, young
    /// crystals near the edges give iridescence.
    pub particle_radius: f64,
    pub particle_growth: f64,
    pub young_size: f64,
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
            // The streaks come from the fibres and the fall: the cells themselves only a little
            // elongated.
            CirrusGenus::Cirrus => (0.25, 2500., 5. / 3., 1.5),
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
        let (max_density, generating_depth, fall_depth, sublimation, shear, fall_speed, turbulence, patchiness)
            = match f.genus {
            // 4 m/s per km of shear, crystals falling 1 m/s: streaks trailing several kilometres.
            // Dense, as the fibres leave most of the volume clear.
            CirrusGenus::Cirrus => (6e-3, 0.12, 1., 1.5, 0.004, 1., 0.15 * depth, 0.3),
            CirrusGenus::Cirrostratus => (2e-4, 0.4, 1., 1., 0.003, 0.7, 0.1 * depth, 0.3),
            // Barely falling, and soon gone: compact cells in a thin layer near the top.
            CirrusGenus::Cirrocumulus => (1.5e-3, 0.12, 0.25, 2., 0.002, 0.2, 0.02 * depth, 0.8),
        };
        let (veer, flow, fibres, fibre_width) = match f.genus {
            // The wind turning 70° down through the layer: hooked trails.
            CirrusGenus::Cirrus => (1.2, 6000., 0.95, 250.),
            CirrusGenus::Cirrostratus => (0.2, 3000., 0.5, 300.),
            CirrusGenus::Cirrocumulus => (0., 600., 0.2, 100.),
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
            fall_speed_spread: 0.3,
            veer,
            edge: 0.6,
            flow_size: f.cell_size * 5.,
            flow,
            fibre_width,
            fibres,
            turbulence_size: f.cell_size,
            turbulence,
            time: 0.,
            evolution_period: f64::INFINITY,
            patchiness,
            patch_size: f.cell_size * 12.,
            particle_radius: match f.genus {
                CirrusGenus::Cirrus => 12.,
                CirrusGenus::Cirrostratus => 20.,
                CirrusGenus::Cirrocumulus => 6.,
            },
            particle_growth: 1.,
            young_size: 0.5,
            center: Vector2::zeros(),
            extent: f64::INFINITY,
            fade: 0.3,
            field,
            threshold,
            detail_noise: PerlinNoise::shuffled(f.seed.wrapping_mul(3).wrapping_add(1)),
            curl_noise: PerlinNoise::shuffled(f.seed.wrapping_mul(3).wrapping_add(2)),
        }
    }

    /// ∫ (u(z) - u_gen) dz down to depth `fallen` below the top, in the (along the wind,
    /// across it) frame: the drift, times the fall speed, of ice relative to the cell it left.
    /// The wind slows downwards at the shear s from a at the top, and turns at θ = veer / depth:
    /// u(z) = (a - s z) (cos θz, sin θz), whose integral has a closed form. With θ → 0 it is
    /// (-s Z² / 2, 0), straight back along the wind.
    fn wind_integral(&self, fallen: f64) -> Vector2<f64> {
        let (a, s, z) = (self.generating_wind, self.shear, fallen);
        let theta = self.veer / (self.top - self.base);
        let phi = theta * z;
        let (along, across) = if phi.abs() < 1e-6 {
            (a * z - s * z * z / 2., theta * (a * z * z / 2. - s * z * z * z / 3.))
        } else {
            ((a - s * z) * phi.sin() / theta + s * (1. - phi.cos()) / (theta * theta),
             a / theta - (a - s * z) * phi.cos() / theta - s * phi.sin() / (theta * theta))
        };
        Vector2::new(along - a * z, across)
    }

    /// Where ice at depth `fallen`, now at `x` in the cells' frame, left its cell, for fall
    /// speed `v`.
    fn source(&self, x: &Vector2<f64>, fallen: f64, v: f64) -> Vector2<f64> {
        let i = self.wind_integral(fallen) / v;
        let across = Vector2::new(-self.wind.y, self.wind.x);
        x - self.wind * i.x - across * i.y
    }

    /// The large-scale flow's warp of horizontal position (x, z), so that what is straight
    /// along the wind in the generating field is curved in the sky.
    fn warp(&self, x: &Vector2<f64>) -> Vector2<f64> {
        if self.flow <= 0. {
            return *x;
        }
        let q = x / self.flow_size;
        let h = 1e-3;
        let psi = |dx: f64, dz: f64| self.curl_noise.noise(q.x + dx, 3.7, q.y + dz);
        let dpsi_dx = (psi(h, 0.) - psi(-h, 0.)) / (2. * h);
        let dpsi_dz = (psi(0., h) - psi(0., -h)) / (2. * h);
        x + Vector2::new(dpsi_dz, -dpsi_dx) * self.flow
    }

    /// The fall speed of the ice at `p` (in the cells' frame, so the pattern travels with
    /// them): the mean, varied by up to `fall_speed_spread` either way by noise a few fibres
    /// across.
    fn fall_speed_at(&self, p: &Vector3<f64>) -> f64 {
        if self.fall_speed_spread <= 0. {
            return self.fall_speed;
        }
        let q = p / (self.fibre_width * 3.);
        self.fall_speed * (1. + self.fall_speed_spread * self.curl_noise.noise(q.x, q.y, q.z).clamp(-1., 1.))
    }

    /// The generating elements at source position `q`, in [0, 1]: small, separate blobs of
    /// noise, with no direction of their own and the same at every height, since each is the
    /// head of a strand. Looked up at the source of the ice, each draws out into a strand along
    /// its own fall, curving as the fall does.
    fn fibre(&self, q: &Vector2<f64>) -> f64 {
        let q = q / self.fibre_width;
        let (mut sum, mut weight, mut amp, mut freq) = (0., 0., 1., 1.);
        for _ in 0..3 {
            let n = self.detail_noise.noise(q.x * freq, 0.37, q.y * freq);
            sum += amp * n.max(0.).powi(2) * 4.;
            weight += amp;
            amp *= 0.5;
            freq *= 2.2;
        }
        (sum / weight).min(1.)
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

    /// `p` in the cells' frame, in which the pattern of fibres and eddies travels too, and its
    /// (x, z) there after the eddies have displaced it, at fractional depth `d`; the source of
    /// its ice is then back along the fall from there.
    fn frame(&self, p: &Vector3<f64>, d: f64) -> (Vector3<f64>, Vector2<f64>) {
        let mut x = Vector2::new(p.x, p.z) - self.wind * (self.generating_wind * self.time);
        let framed = Vector3::new(x.x, p.y, x.y);
        if self.turbulence > 0. {
            x += self.curl(&framed) * (self.turbulence * d);
        }
        (framed, x)
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
        let (framed, x) = self.frame(p, d);
        let fall_speed = self.fall_speed_at(&framed);
        let edge_fade = {
            let from_centre = (Vector2::new(p.x, p.z) - self.center).abs().max() / self.extent;
            let t = ((1. - from_centre) / self.fade.max(1e-9)).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        if edge_fade <= 0. {
            return 0.;
        }
        // Where this ice came from: back along its fall, at a fall speed that varies from
        // point to point (crystals of different sizes fall at different speeds), so that
        // streaks fray rather than show copies of their cells. The fibres belong to the ice,
        // so they are looked up there too, and curve with the streaks and with the flow.
        let source = self.warp(&self.source(&x, fallen, fall_speed));
        let cells = self.cells(source.x, source.y);
        if cells <= 0. {
            return 0.;
        }
        let detail = 1. - self.fibres * (1. - self.fibre(&source));
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

    /// Crystals grow as they fall, and are smallest where the ice is newest: at the edges of
    /// the cells, traced back along the (mean) fall.
    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> {
        let fallen = (self.top - p.y).clamp(0., self.top - self.base);
        let d = fallen / (self.top - self.base);
        let (framed, x) = self.frame(p, d);
        let source = self.warp(&self.source(&x, fallen, self.fall_speed_at(&framed)));
        let maturity = self.cells(source.x, source.y);
        let young = self.young_size + (1. - self.young_size) * maturity;
        Some(self.particle_radius * (1. + self.particle_growth * d) * young)
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

/// A periodic 3D Gaussian random field with a power-law spectrum, in two independent
/// realisations: `GeneratingField` in three dimensions, for cirrus over a whole planet, sampled
/// on the sphere of its generating level. In three dimensions a 1D spectral slope of -β is a
/// spectral density ∝ k^-(β + 2).
struct GeneratingField3 {
    texel: f64,
    n: usize,
    real: Vec<f32>,
    imaginary: Vec<f32>,
}

/// In-place 3D FFT of `data`, n on a side (a power of two), x fastest.
fn fft3d(data: &mut [rustfft::num_complex::Complex<f64>], n: usize, inverse: bool) {
    use rustfft::{Fft, FftDirection};
    let fft = rustfft::algorithm::Radix4::new(n, if inverse { FftDirection::Inverse } else { FftDirection::Forward });
    let mut line = vec![Complex::new(0., 0.); n];
    // Along each axis in turn: gather a line, transform it, scatter it back.
    for stride in [1, n, n * n] {
        for base in 0..n * n {
            // The line's start: every index whose coordinate along this axis is zero.
            let start = match stride {
                1 => base * n,
                s if s == n => (base / n) * n * n + base % n,
                _ => base,
            };
            for k in 0..n {
                line[k] = data[start + k * stride];
            }
            fft.process(&mut line);
            for k in 0..n {
                data[start + k * stride] = line[k];
            }
        }
    }
}

impl GeneratingField3 {
    const N: usize = 128;

    fn new(seed: u64, texel: f64, outer_scale: f64, slope: f64) -> GeneratingField3 {
        let n = Self::N;
        let mut rng = Pcg32::new(seed, 9);
        let mut gaussian = || {
            let (u, v) = (1. - rng.next_f64(), rng.next_f64());
            (-2. * u.ln()).sqrt() * (2. * PI * v).cos()
        };
        let k0 = 2. * PI / outer_scale;
        let dk = 2. * PI / (n as f64 * texel);
        let freq = |i: usize| if i <= n / 2 { i as f64 } else { i as f64 - n as f64 };
        let mut spectrum = vec![Complex::new(0., 0.); n * n * n];
        for z in 0..n {
            for y in 0..n {
                for x in 0..n {
                    let k2 = (freq(x).powi(2) + freq(y).powi(2) + freq(z).powi(2)) * dk * dk;
                    let amplitude = if x + y + z == 0 { 0. } else { (k2 + k0 * k0).powf(-(slope + 2.) / 2.).sqrt() };
                    spectrum[(z * n + y) * n + x] = Complex::new(gaussian(), gaussian()) * amplitude;
                }
            }
        }
        fft3d(&mut spectrum, n, true);
        let normalise = |v: Vec<f64>| {
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
            v.into_iter().map(|x| ((x - mean) / sd) as f32).collect::<Vec<f32>>()
        };
        GeneratingField3 {
            texel,
            n,
            real: normalise(spectrum.iter().map(|c| c.re).collect()),
            imaginary: normalise(spectrum.iter().map(|c| c.im).collect()),
        }
    }

    /// Trilinearly interpolated and periodic; `phase` rotates between the realisations.
    fn at(&self, p: &Vector3<f64>, phase: (f64, f64)) -> f64 {
        let n = self.n as i64;
        let q = p / self.texel;
        let (x0, y0, z0) = (q.x.floor(), q.y.floor(), q.z.floor());
        let (fx, fy, fz) = (q.x - x0, q.y - y0, q.z - z0);
        let (x0, y0, z0) = (x0 as i64, y0 as i64, z0 as i64);
        let t = |dx: i64, dy: i64, dz: i64| {
            let idx = (((z0 + dz).rem_euclid(n) * n + (y0 + dy).rem_euclid(n)) * n + (x0 + dx).rem_euclid(n)) as usize;
            phase.0 * self.real[idx] as f64 + phase.1 * self.imaginary[idx] as f64
        };
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        let plane = |dz: i64| lerp(lerp(t(0, 0, dz), t(1, 0, dz), fx), lerp(t(0, 1, dz), t(1, 1, dz), fx), fy);
        lerp(plane(0), plane(1), fz)
    }

    fn quantile_above(&self, coverage: f64) -> f64 {
        let mut sorted = self.real.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let i = (((1. - coverage) * sorted.len() as f64) as usize).min(sorted.len() - 1);
        sorted[i] as f64
    }
}

/// # Cirrus over a whole planet
///
/// Hogan and Kew's cirrus (`HoganKewCirrus`'s parameters and physics: the profile, the drift of
/// falling ice under a wind that slows and turns with depth, its fall speeds and fibres) on a
/// sphere, with nothing flat in it:
///
/// - The generating cells are a 3D Gaussian random field (`GeneratingField3`) sampled on the
///   sphere of the generating level: no poles, seams or stretching.
/// - The wind is the planet's (`GlobalWind`): the drift is taken in each point's local
///   horizontal, along and across its own wind, so streaks follow the jets and curl round the
///   eddies, and the fibres, looked up at the ice's source as before, follow them.
/// - Solid noise elsewhere (fall speeds, fibres, patchiness, turbulence) at the position from
///   the planet's centre.
pub struct GlobalCirrus {
    pub params: HoganKewCirrus,
    pub wind: crate::planet::GlobalWind,
    field: GeneratingField3,
    threshold: f64,
    radius: f64,
}

impl GlobalCirrus {
    /// From the flat layer's parameters and the generating cells' size, spectral slope,
    /// coverage and seed (as in `CirrusField`).
    pub fn new(params: HoganKewCirrus, cell_size: f64, slope: f64, coverage: f64, seed: u64,
               radius: f64, pole: Vector3<f64>) -> GlobalCirrus {
        let field = GeneratingField3::new(seed, cell_size / TEXELS_PER_CELL, cell_size, slope);
        let threshold = field.quantile_above(coverage.clamp(1e-6, 1.));
        GlobalCirrus { params, wind: crate::planet::GlobalWind::new(pole, radius, seed.wrapping_add(17)), field, threshold, radius }
    }

    /// Where the ice at `at` left its cell, on the sphere of the generating level, and its
    /// depth as a fraction of the layer: back along its fall in the local horizontal, along and
    /// across the local wind, then the cells' own drift with the wind at the generating level.
    fn source(&self, at: &crate::planet::SurfacePoint) -> (Vector3<f64>, f64) {
        let c = &self.params;
        let fallen = (c.top - at.altitude).clamp(0., c.top - c.base);
        let d = fallen / (c.top - c.base);
        let along = self.wind.direction(&at.up);
        let across = at.up.cross(&along);
        let i = c.wind_integral(fallen) / c.fall_speed_at(&at.position);
        let mut shift = -(along * (i.x + c.generating_wind * c.time) + across * i.y);
        if c.turbulence > 0. {
            // Eddies the ice falls through: the local curl of solid noise.
            let q = at.position / c.turbulence_size;
            let h = 1e-3;
            let psi = |e: Vector3<f64>| c.curl_noise.noise(q.x + e.x, q.y + e.y, q.z + e.z);
            let grad = Vector3::new(psi(Vector3::x() * h) - psi(-Vector3::x() * h),
                                    psi(Vector3::y() * h) - psi(-Vector3::y() * h),
                                    psi(Vector3::z() * h) - psi(-Vector3::z() * h)) / (2. * h);
            shift += at.up.cross(&grad) * (c.turbulence * d);
        }
        let generating = self.radius + c.top;
        (((at.up * generating) + shift).normalize() * generating, d)
    }

    fn cells(&self, source: &Vector3<f64>) -> f64 {
        let c = &self.params;
        let w = if c.evolution_period.is_finite() { 2. * PI * c.time / c.evolution_period } else { 0. };
        ((self.field.at(source, (w.cos(), w.sin())) - self.threshold) / c.edge).clamp(0., 1.)
    }

    /// The generating elements (see `HoganKewCirrus::fibre`), in solid noise.
    fn fibre(&self, source: &Vector3<f64>) -> f64 {
        let q = source / self.params.fibre_width;
        let (mut sum, mut weight, mut amp, mut freq) = (0., 0., 1., 1.);
        for _ in 0..3 {
            let n = self.params.detail_noise.noise(q.x * freq, q.y * freq, q.z * freq);
            sum += amp * n.max(0.).powi(2) * 4.;
            weight += amp;
            amp *= 0.5;
            freq *= 2.2;
        }
        (sum / weight).min(1.)
    }
}

impl crate::planet::PlanetField for GlobalCirrus {
    fn density(&self, at: &crate::planet::SurfacePoint) -> f64 {
        let c = &self.params;
        if at.altitude < c.base || at.altitude > c.top {
            return 0.;
        }
        let profile = c.profile((c.top - at.altitude) / (c.top - c.base));
        if profile <= 0. {
            return 0.;
        }
        let (source, _) = self.source(at);
        let cells = self.cells(&source);
        if cells <= 0. {
            return 0.;
        }
        let detail = 1. - c.fibres * (1. - self.fibre(&source));
        let patches = if c.patchiness > 0. {
            let q = source / c.patch_size;
            let n = c.detail_noise.fbm(q.x, q.y, q.z, 3, 0.5, 2.);
            1. - c.patchiness * (1. - ((n - 0.4) * 5.).clamp(0., 1.))
        } else {
            1.
        };
        c.max_density * (profile * cells * detail * patches).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 {
        self.params.max_density
    }

    /// Nothing below the lowest ice or above the layer's top: zero there until the ray reaches
    /// the layer's spheres; inside, the majorant until it leaves.
    fn majorant_from(&self, at: &crate::planet::SurfacePoint, _p: &Vector3<f64>, d: &Vector3<f64>,
                     planet: &crate::planet::Planet) -> (f64, f64) {
        let (low, high) = (self.params.lowest(), self.params.top);
        let ahead = (at.position + d * 0.01).norm() - planet.radius;
        let to_low = crate::planet::to_sphere(&at.position, d, planet.radius + low);
        let to_high = crate::planet::to_sphere(&at.position, d, planet.radius + high);
        if ahead < low || ahead > high {
            (0., to_low.min(to_high))
        } else {
            (self.params.max_density, to_low.min(to_high))
        }
    }

    fn particle_radius(&self, at: &crate::planet::SurfacePoint) -> Option<f64> {
        let c = &self.params;
        let (source, d) = self.source(at);
        let young = c.young_size + (1. - c.young_size) * self.cells(&source);
        Some(c.particle_radius * (1. + c.particle_growth * d) * young)
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
            c.fibres = 0.;
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
        c.fibres = 0.;
        c.flow = 0.;
        c.veer = 0.;
        c.fall_speed_spread = 0.;
        c.patchiness = 0.;
        c.shear = 0.01;
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
        assert!(along > 1.5 * across, "along {} across {}", along, across);
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
        assert!(inner > 10. && outer < 0.3 * inner, "outer {} inner {}", outer, inner);
    }

    #[test]
    fn test_crystals_grow_and_are_young_at_edges() {
        let mut c = cirrus(CirrusGenus::Cirrus, 11);
        c.turbulence = 0.;
        let r = |p: Vector3<f64>| c.particle_radius(&p).unwrap();
        // At the generating level, between young_size and 1 times the radius; larger lower down.
        let mut sizes = vec![];
        for i in 0..2000 {
            let p = Vector3::new(i as f64 * 97., 9990., (i % 31) as f64 * 311.);
            let s = r(p) / c.particle_radius;
            assert!((c.young_size - 1e-9..=1.01).contains(&s), "{}", s);
            sizes.push(s);
        }
        assert!(sizes.iter().any(|s| *s < 0.6) && sizes.iter().any(|s| *s > 0.9), "no range of sizes");
        let low = (0..200).map(|i| r(Vector3::new(i as f64 * 97., 8100., 0.))).sum::<f64>();
        let high = (0..200).map(|i| r(Vector3::new(i as f64 * 97., 9990., 0.))).sum::<f64>();
        assert!(low > 1.5 * high, "{} {}", low, high);
    }

    #[test]
    fn test_veer_hooks_the_trail() {
        // The drift's integral: with no veer, straight back along the wind as s Z² / 2; with
        // it, turned aside, and agreeing with a numerical integral of the turning wind.
        let mut c = cirrus(CirrusGenus::Cirrus, 12);
        c.veer = 0.;
        let straight = c.wind_integral(1500.);
        assert!((straight.x + c.shear * 1500. * 1500. / 2.).abs() < 1e-6 && straight.y.abs() < 1e-9);
        c.veer = 1.2;
        let hooked = c.wind_integral(1500.);
        let theta = c.veer / 2000.;
        let n = 100_000;
        let numeric = (0..n).fold(Vector2::zeros(), |acc, i| {
            let z = (i as f64 + 0.5) / n as f64 * 1500.;
            let speed = c.generating_wind - c.shear * z;
            acc + (Vector2::new((theta * z).cos(), (theta * z).sin()) * speed - Vector2::new(c.generating_wind, 0.)) * (1500. / n as f64)
        });
        assert!((hooked - numeric).norm() < 1e-3 * numeric.norm(), "{} vs {}", hooked, numeric);
        assert!(hooked.y.abs() > 0.3 * hooked.norm(), "barely turned: {}", hooked);
    }

    #[test]
    fn test_fibres_leave_clear_air_between() {
        // At the generating level, within the cells, most points are much thinner than the
        // densest: wisps, not a fill.
        let c = cirrus(CirrusGenus::Cirrus, 13);
        let v: Vec<f64> = (0..20000).map(|i| c.fraction(&Vector3::new((i % 200) as f64 * 37., 9850., (i / 200) as f64 * 41.)))
            .filter(|f| *f > 0.).collect();
        let max = v.iter().cloned().fold(0., f64::max);
        let thin = v.iter().filter(|f| **f < 0.2 * max).count() as f64 / v.len() as f64;
        assert!(v.len() > 100 && thin > 0.5, "{} of {} thin", thin, v.len());
    }

    #[test]
    fn test_global_cirrus() {
        use crate::planet::{Planet, PlanetField, GlobalMapping};
        use crate::volume::DensityField;
        let mut f = CirrusField::new(CirrusGenus::Cirrus);
        f.seed = 21;
        let (cell, slope, coverage) = (f.cell_size, f.spectral_slope, f.coverage);
        let params = HoganKewCirrus::new(f);
        let planet = Planet { radius: 6_360_000. };
        let global = GlobalCirrus::new(params, cell, slope, coverage, 21, planet.radius, Vector3::z());
        // The 3D field: unit variance, and the threshold leaves the coverage above it.
        let above = global.field.real.iter().filter(|v| **v as f64 > global.threshold).count() as f64 / global.field.real.len() as f64;
        assert!((above - coverage).abs() < 0.01, "{}", above);
        let field = GlobalMapping { planet, field: std::sync::Arc::new(global) };
        let mut rng = crate::geometry::Pcg32::new(6, 0);
        let mut cloudy = 0;
        for _ in 0..3000 {
            let up = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let p = planet.center() + up * (planet.radius + 7000. + rng.next_f64() * 4000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let density = field.density(&p);
            assert!((0. ..=field.majorant()).contains(&density));
            cloudy += (density > 0.) as usize;
            // The local majorant bounds it along the ray.
            let (m, len) = field.majorant_from(&p, &d);
            for k in 0..10 {
                let q = p + d * (len.min(20_000.) * k as f64 / 10.);
                assert!(field.density(&q) <= m + 1e-12);
            }
        }
        // Some cirrus, at every longitude band and near the poles alike (no seams to miss).
        assert!(cloudy > 100, "{}", cloudy);
    }

    #[test]
    fn test_fft3d_round_trip() {
        let n = 8;
        let original: Vec<Complex<f64>> = (0..n * n * n).map(|i| Complex::new((i as f64 * 0.37).sin(), (i as f64 * 0.11).cos())).collect();
        let mut data = original.clone();
        fft3d(&mut data, n, false);
        fft3d(&mut data, n, true);
        for (a, b) in original.iter().zip(&data) {
            assert!((a - b / (n * n * n) as f64).norm() < 1e-9);
        }
    }

    #[test]
    fn test_seeded() {
        let row = |c: HoganKewCirrus| (0..300).map(|i| c.fraction(&Vector3::new(i as f64 * 211., 9600., 0.))).collect::<Vec<f64>>();
        assert_eq!(row(cirrus(CirrusGenus::Cirrus, 7)), row(cirrus(CirrusGenus::Cirrus, 7)));
        assert_ne!(row(cirrus(CirrusGenus::Cirrus, 7)), row(cirrus(CirrusGenus::Cirrus, 8)));
    }
}

