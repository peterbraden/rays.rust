//! Lenticular clouds (altocumulus, stratocumulus and cirrocumulus lenticularis): smooth lenses,
//! often stacked like plates, standing still over the lee waves of a mountain range. A density
//! field for `volume::Volume`.
//!
//! Stable air blowing across a ridge is set oscillating by it, and where the stability and the
//! wind trap the oscillation below some height it continues downstream as a train of standing
//! lee waves, 5-20 km apart, that dies away slowly (R. S. Scorer, "Theory of waves in the lee of
//! mountains", QJRMS 75, 1949; D. R. Durran, "Mountain waves and downslope winds", 1990). Air
//! passing through rises over each crest and sinks in each trough. Where it carries a layer
//! that is nearly saturated, the lift on the crests condenses it, and the cloud evaporates again
//! on the way down, so the cloud stands still while the wind blows through it. Moist layers are
//! often thin and several, which stacks the lenses: the "pile of plates".
//!
//! The field is that picture, in physical units:
//!
//! 1. **Waves.** The air's vertical displacement `η` is `amplitude` times a cosine downstream
//!    of the ridge, `wavelength` long, with an envelope that rises over the ridge, decays
//!    downstream over `decay`, and falls off beyond the ridge's ends, spreading with distance.
//!    The crests bend a little (`bend`), as real ridges aren't straight.
//! 2. **Moist layers.** Each layer is given by its middle, its depth and its *deficit*, the
//!    lift needed to saturate it there; the deficit grows towards the layer's top and bottom, so
//!    each layer's cloud is a lens: thickest under the crest, tapering to sharp ends. The
//!    deficit also varies slowly from place to place (`humidity_variation`), in fine strata
//!    (`strata`: the laminae lenticulars often show), and in small eddies (`roughness`).
//! 3. **Cloud water.** Lifted past saturation, air condenses about `condensation_rate` of water
//!    per kilometre (about 1.5 g/kg per km at 4 km); droplets of `droplet_radius` give an extinction of
//!    3 LWC / (2 ρ_w r). The edges are crisp, since flow through the cloud is laminar, but
//!    smooth: the cloud water grows from nothing at them.
//!
//! The waves are steady, so the clouds don't change; `time` only moves the strata and eddies
//! through them with the wind.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::volume::DensityField;
use std::f64::consts::TAU;

/// Density of liquid water, kg/m³.
const WATER_DENSITY: f64 = 1000.;

/// Air density at height `y` (m): a scale height of 8.5 km.
fn air_density(y: f64) -> f64 {
    1.225 * (-y / 8500.).exp()
}

/// The largest value of cos over [lo, hi].
fn max_cos(lo: f64, hi: f64) -> f64 {
    if hi - lo >= TAU || (hi / TAU).floor() > (lo / TAU).floor() || lo.rem_euclid(TAU) == 0. {
        1.
    } else {
        lo.cos().max(hi.cos())
    }
}

#[derive(Clone, Debug)]
pub struct MoistLayer {
    /// Height of its middle, and its depth (m).
    pub height: f64,
    pub depth: f64,
    /// Lift needed to saturate its middle (m).
    pub deficit: f64,
}

pub struct Lenticular {
    /// Where the ridge is ([x, z]), and how far it runs either side across the wind.
    pub ridge: Vector2<f64>,
    pub ridge_length: f64,
    /// Unit [x, z] direction of the wind.
    pub wind_direction: Vector2<f64>,
    pub wind_speed: f64,
    /// The lee waves: crest-to-crest distance, the largest vertical displacement (m), the
    /// distance downstream over which they fall to 1/e, and how far downstream the first crest
    /// is (a fraction of a wavelength).
    pub wavelength: f64,
    pub amplitude: f64,
    pub decay: f64,
    pub first_crest: f64,
    /// How far beyond the ridge's ends the waves fade, at the ridge and growing downstream by
    /// `spread` metres per metre.
    pub falloff: f64,
    pub spread: f64,
    /// How far the crests bend from straight lines (m), and over what distance across the wind.
    pub bend: f64,
    pub bend_size: f64,
    /// How much (as lift, in m) the layers' humidity varies, over `humidity_size`.
    pub humidity_variation: f64,
    pub humidity_size: f64,
    /// Fine laminae: their strength (m of lift) and spacing.
    pub strata: f64,
    pub strata_spacing: f64,
    /// Small eddies: their strength (m of lift) and size.
    pub roughness: f64,
    pub detail_size: f64,
    pub layers: Vec<MoistLayer>,
    /// Cloud water condensed per kilometre of lift (g/kg), and the droplets' mean radius (µm).
    pub condensation_rate: f64,
    pub droplet_radius: f64,
    /// Seconds since the scene's start, to move the fine structure with the wind.
    pub time: f64,
    humidity_noise: PerlinNoise,
    detail_noise: PerlinNoise,
    bend_noise: PerlinNoise,
}

/// The size of a block for the majorant grid, as a fraction of the wavelength.
const BLOCK: f64 = 1. / 16.;

impl Lenticular {
    pub fn new(layers: Vec<MoistLayer>, seed: u64) -> Lenticular {
        Lenticular {
            ridge: Vector2::zeros(),
            ridge_length: 6000.,
            wind_direction: Vector2::new(1., 0.),
            wind_speed: 15.,
            wavelength: 10_000.,
            amplitude: 400.,
            decay: 25_000.,
            first_crest: 0.6,
            falloff: 3000.,
            spread: 0.15,
            bend: 600.,
            bend_size: 8000.,
            humidity_variation: 50.,
            humidity_size: 6000.,
            strata: 12.,
            strata_spacing: 45.,
            roughness: 8.,
            detail_size: 250.,
            layers,
            condensation_rate: 1.5,
            droplet_radius: 8.,
            time: 0.,
            humidity_noise: PerlinNoise::with_seed(seed.wrapping_mul(3).wrapping_add(1)),
            detail_noise: PerlinNoise::with_seed(seed.wrapping_mul(3).wrapping_add(2)),
            bend_noise: PerlinNoise::with_seed(seed.wrapping_mul(3).wrapping_add(3)),
        }
    }

    /// The default stack: three plates a few hundred metres apart, drier upwards.
    pub fn default_layers() -> Vec<MoistLayer> {
        vec![
            MoistLayer { height: 4000., depth: 320., deficit: 190. },
            MoistLayer { height: 4450., depth: 260., deficit: 230. },
            MoistLayer { height: 4850., depth: 200., deficit: 270. },
        ]
    }

    pub fn bottom(&self) -> f64 {
        self.layers.iter().map(|l| l.height - l.depth / 2.).fold(f64::INFINITY, f64::min)
    }

    pub fn top(&self) -> f64 {
        self.layers.iter().map(|l| l.height + l.depth / 2.).fold(f64::NEG_INFINITY, f64::max)
    }

    /// Extinction per metre of lift past saturation, at height `y`.
    fn extinction_per_lift(&self, y: f64) -> f64 {
        let q = self.condensation_rate * 1e-6; // kg/kg per metre
        1.5 * air_density(y) * q / (WATER_DENSITY * self.droplet_radius * 1e-6)
    }

    /// The most the deficit can fall below a layer's own, from the variations.
    fn variation(&self) -> f64 {
        self.humidity_variation + self.strata + self.roughness
    }

    /// Distance along and across the wind from the ridge.
    fn wind_frame(&self, x: f64, z: f64) -> (f64, f64) {
        let (dx, dz) = (x - self.ridge.x, z - self.ridge.y);
        let w = self.wind_direction;
        (dx * w.x + dz * w.y, -dx * w.y + dz * w.x)
    }

    fn along_envelope(&self, s: f64) -> f64 {
        let rise = crate::procedural::nubis_clouds::smoothstep(-0.3 * self.wavelength, 0.3 * self.wavelength, s);
        rise * (-s.max(0.) / self.decay).exp()
    }

    fn across_envelope(&self, c: f64, s: f64) -> f64 {
        let beyond = (c.abs() - self.ridge_length).max(0.);
        let width = self.falloff + self.spread * s.max(0.);
        (-(beyond / width).powi(2)).exp()
    }

    /// The air's vertical displacement at (`s`, `c`) in the wind frame.
    fn displacement(&self, s: f64, c: f64) -> f64 {
        let bend = (self.bend_noise.fbm(c / self.bend_size, 0.5, 0.5, 2, 0.5, 2.) - 0.5) * 2. * self.bend;
        let phase = TAU * ((s + bend) / self.wavelength - self.first_crest);
        self.amplitude * self.along_envelope(s) * self.across_envelope(c, s) * phase.cos()
    }

    /// Lift needed to saturate the air at `p` (m).
    fn deficit(&self, layer: &MoistLayer, p: &Vector3<f64>, drifted: &Vector3<f64>) -> f64 {
        let u = (p.y - layer.height) / (layer.depth / 2.);
        // Parabolic: the lens tapers to a point where the deficit reaches the wave's crest.
        let profile = layer.deficit + (self.amplitude - layer.deficit).max(layer.deficit) * u * u;
        let humidity = (self.humidity_noise.fbm(drifted.x / self.humidity_size, layer.height / 1000., drifted.z / self.humidity_size, 3, 0.5, 2.) - 0.5) * 2. * self.humidity_variation;
        let d = drifted / self.detail_size;
        let eddies = (self.detail_noise.fbm(d.x, d.y, d.z, 3, 0.5, 2.) - 0.5) * 2.;
        let strata = (TAU * p.y / self.strata_spacing + 2. * eddies).sin() * self.strata;
        profile + humidity + strata + eddies * self.roughness
    }

    /// Extinction coefficient at `p`.
    pub fn extinction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        let (s, c) = self.wind_frame(p.x, p.z);
        let eta = self.displacement(s, c);
        if eta <= 0. {
            return 0.;
        }
        // The fine structure moves with the wind, while the waves stand still.
        let w = self.wind_direction * self.wind_speed * self.time;
        let drifted = p - Vector3::new(w.x, 0., w.y);
        let mut lift: f64 = 0.;
        for l in &self.layers {
            if (p.y - l.height).abs() < l.depth / 2. && eta > l.deficit - self.variation() {
                lift = lift.max(eta - self.deficit(l, p, &drifted));
            }
        }
        lift.max(0.) * self.extinction_per_lift(p.y)
    }

    /// An upper bound on the displacement over the block of the wind frame from `s0` to
    /// `s0 + size` along and `c0` to `c0 + size` across.
    fn displacement_bound(&self, s0: f64, c0: f64, size: f64) -> f64 {
        let s1 = s0 + size;
        // The rise only grows downstream and the decay only shrinks, so the rise at the far
        // end times the decay at the near end bounds their product.
        let rise = crate::procedural::nubis_clouds::smoothstep(-0.3 * self.wavelength, 0.3 * self.wavelength, s1);
        let peak = rise * (-s0.max(0.) / self.decay).exp();
        let nearest_c = if c0 <= 0. && c0 + size >= 0. { 0. } else { c0.abs().min((c0 + size).abs()) };
        let across = self.across_envelope(nearest_c, s1.max(0.));
        let phase = |s: f64| TAU * (s / self.wavelength - self.first_crest);
        let slack = TAU * self.bend / self.wavelength;
        self.amplitude * peak * across * max_cos(phase(s0) - slack, phase(s1) + slack)
    }
}

impl DensityField for Lenticular {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.extinction(p)
    }

    fn majorant(&self) -> f64 {
        self.layers.iter().map(|l| (self.amplitude - l.deficit + self.variation()).max(0.)).fold(0., f64::max)
            * self.extinction_per_lift(self.bottom())
    }

    /// Blocks of the wind frame, a sixteenth of a wavelength square and as deep as the stack:
    /// each bounded by the largest displacement in it.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let (lo, hi) = (self.bottom(), self.top());
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        if ahead.y < lo || ahead.y > hi {
            let entry = if ahead.y < lo && d.y > 0. { to_y(lo) } else if ahead.y > hi && d.y < 0. { to_y(hi) } else { f64::INFINITY };
            return (0., entry + 0.01);
        }
        let exit_y = if d.y > 0. { to_y(hi) } else if d.y < 0. { to_y(lo) } else { f64::INFINITY };
        let size = BLOCK * self.wavelength;
        let (s, c) = self.wind_frame(ahead.x, ahead.z);
        let (s0, c0) = ((s / size).floor() * size, (c / size).floor() * size);
        let w = self.wind_direction;
        let (ds, dc) = (d.x * w.x + d.z * w.y, -d.x * w.y + d.z * w.x);
        let to = |v: f64, dv: f64, b0: f64| {
            if dv > 0. { (b0 + size - v) / dv } else if dv < 0. { (b0 - v) / dv } else { f64::INFINITY }
        };
        let exit = to(s, ds, s0).min(to(c, dc, c0)).min(exit_y);
        let eta = self.displacement_bound(s0, c0, size);
        let m = self.layers.iter()
            .map(|l| (eta - l.deficit + self.variation()).max(0.))
            .fold(0., f64::max);
        let m = if eta <= 0. { 0. } else { m * self.extinction_per_lift(lo) };
        (m, exit.max(0.) + 0.01)
    }

    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(self.droplet_radius)
    }
}

/// Half a wave: the lens's half-length along the wind for a layer of this deficit, as a
/// fraction of a wavelength (for tests and documentation).
pub fn lens_half_length(amplitude: f64, deficit: f64) -> f64 {
    (deficit / amplitude).clamp(-1., 1.).acos() / TAU
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn uniform(rng: &mut Pcg32, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * rng.next_f64()
    }

    fn quiet() -> Lenticular {
        let mut l = Lenticular::new(vec![MoistLayer { height: 4000., depth: 300., deficit: 200. }], 1);
        (l.humidity_variation, l.strata, l.roughness, l.bend) = (0., 0., 0., 0.);
        l
    }

    #[test]
    fn test_max_cos() {
        assert_eq!(max_cos(-0.1, 0.1), 1.);
        assert!((max_cos(1., 2.) - 1f64.cos()).abs() < 1e-12);
        assert!((max_cos(4., 5.) - 5f64.cos()).abs() < 1e-12);
        assert_eq!(max_cos(6., 6.5), 1.);
        assert_eq!(max_cos(0., 0.5), 1.);
    }

    #[test]
    fn test_lenses_sit_on_the_crests() {
        let l = quiet();
        let crest = |n: f64| (l.first_crest + n) * l.wavelength;
        // Cloud on the first two crests, in the layer's middle; none in the troughs or upwind.
        let first = l.extinction(&Vector3::new(crest(0.), 4000., 0.));
        let second = l.extinction(&Vector3::new(crest(1.), 4000., 0.));
        assert!(first > 0.01 && second > 0. && second < first, "{} then {}", first, second);
        for n in 0..2 {
            assert_eq!(l.extinction(&Vector3::new(crest(n as f64 + 0.5), 4000., 0.)), 0.);
        }
        assert_eq!(l.extinction(&Vector3::new(-l.wavelength, 4000., 0.)), 0.);
        // Nothing beyond the ridge's ends, nor above or below the layer.
        assert_eq!(l.extinction(&Vector3::new(crest(0.), 4000., l.ridge_length + 4. * l.falloff)), 0.);
        assert_eq!(l.extinction(&Vector3::new(crest(0.), 4200., 0.)), 0.);
        // A lens: thicker in the middle than towards its ends, and ending before the half-length.
        let depth = |s: f64| (0..200).filter(|k| l.extinction(&Vector3::new(s, 3840. + 320. * *k as f64 / 200., 0.)) > 0.).count();
        let half = lens_half_length(l.amplitude * l.along_envelope(crest(0.)), 200.) * l.wavelength;
        assert!(depth(crest(0.)) > depth(crest(0.) + 0.7 * half));
        assert!(depth(crest(0.) + 0.7 * half) > 0);
        assert_eq!(depth(crest(0.) + 1.05 * half), 0);
    }

    #[test]
    fn test_extinction_is_physical() {
        // The first crest lifts the air about 315 m, against a 200 m deficit at 4 km: 115 m past
        // saturation, so 0.17 g/kg of cloud water; 8 µm droplets give about 0.025 per metre.
        let l = quiet();
        let e = l.extinction(&Vector3::new(l.first_crest * l.wavelength, 4000., 0.));
        let eta = l.displacement(l.first_crest * l.wavelength, 0.);
        let expected = (eta - 200.) * 1.5e-6 * 1.5 * air_density(4000.) / (1000. * 8e-6);
        assert!((e - expected).abs() < 1e-9, "{} vs {}", e, expected);
        assert!(e > 0.01 && e < 0.05, "{}", e);
    }

    #[test]
    fn test_majorant_bounds_extinction() {
        let mut l = Lenticular::new(Lenticular::default_layers(), 4);
        l.wind_direction = Vector2::new(0.6, 0.8);
        l.ridge = Vector2::new(1000., -2000.);
        let mut rng = Pcg32::new(5, 0);
        for _ in 0..1000 {
            let p = Vector3::new(uniform(&mut rng, -20000., 40000.), uniform(&mut rng, 3000., 6000.), uniform(&mut rng, -20000., 40000.));
            let d = Vector3::new(uniform(&mut rng, -1., 1.), uniform(&mut rng, -0.3, 0.3), uniform(&mut rng, -1., 1.)).normalize();
            let (m, len) = l.majorant_from(&p, &d);
            assert!(len > 0.);
            for k in 0..40 {
                let t = len.min(20000.) * (k as f64 + 0.5) / 40.;
                assert!(l.extinction(&(p + d * t)) <= m + 1e-12, "{} along {} at {}", p, d, t);
            }
            assert!(m <= l.majorant() + 1e-12);
        }
    }
}
