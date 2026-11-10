//! # The planet: initial topography and tectonic uplift
//!
//! Before any erosion, a planet is a set of noise fields over the sphere, all chosen by one seed:
//!
//! - **Continents.** Low-frequency fractal Brownian motion (fBm): a sum of octaves of Perlin
//!   noise (Perlin, "Improving Noise", SIGGRAPH 2002, doi:10.1145/566654.566636), each at twice
//!   the frequency and half the amplitude of the last (Musgrave's chapters in Ebert et al.,
//!   "Texturing and Modeling: A Procedural Approach", 3rd ed., 2003). Its sign separates land
//!   from sea.
//! - **Orogens** (mountain belts). Long, narrow belts where "ridged" noise, 1 - |noise|, is near
//!   its peak, restricted to land. Here the planet is pushed up at the tectonic uplift rate U
//!   (metres per year), the source term that erosion works against. Cordonnier et al. ("Large
//!   Scale Terrain Generation from Tectonic Uplift and Fluvial Erosion", Computer Graphics Forum
//!   35(2), 2016, doi:10.1111/cgf.12820) drive their terrains with a painted uplift map; here the
//!   map is noise. Uplift is constant in time for now.
//! - **Relief.** Ridged multifractal noise in the orogens and gentle hills elsewhere, so the
//!   landscape at time zero is the "noise-based topography" that erosion then reworks.
//!
//! - **Bedrock.** Layers alternating between hard and soft rock, folded, with regional
//!   variation (`erodibility`): the lithology that erosion exposes as it cuts down. Forte,
//!   Yanites and Whipple ("Complexities of landscape evolution during incision through layered
//!   stratigraphy with contrasts in rock strength", ESPL 41(12), 2016, doi:10.1002/esp.3947)
//!   model rivers cutting through layers of different erodibility K; Perne et al. ("Steady
//!   state, erosional continuity, and the topography of landscapes developed in layered rocks",
//!   Earth Surface Dynamics 5, 2017, doi:10.5194/esurf-5-85-2017) find the layering matters
//!   from about a threefold contrast in K. The default contrast here is 5.
//! - **Volcanism.** Where volcanoes occur (`volcanism`): arcs beside the mountain belts, and
//!   hotspots. See `volcano` for the volcanoes themselves.
//!
//! Each field is a function of the direction from the planet's centre, so it is the same planet
//! at any resolution. The finest octave is chosen from the grid spacing (`finest`): detail below
//! about two cells would only alias, much as a texture's mipmap level is chosen from its
//! footprint.
//!
//! Perlin's lattice repeats every 256 units. At high octaves on a large planet that period is
//! small enough to see, so each octave samples the noise through its own random rotation (and
//! offset), and no two octaves repeat together.

use crate::geometry::Pcg32;
use crate::na::{Matrix3, Rotation3, Unit, Vector3};
use crate::noise::PerlinNoise;

#[derive(Clone, Debug)]
pub struct PlanetParams {
    pub seed: u64,
    /// Metres.
    pub radius: f64,
    /// Added to the continent field before taking its sign: positive makes more land.
    pub land_bias: f64,
    /// Height of the initial mountains above the plains, metres.
    pub mountain_height: f64,
    /// Peak tectonic uplift rate, metres per year.
    pub uplift_rate: f64,
    /// Mountain belts per planet radius (their spacing).
    pub belt_frequency: f64,
    /// Thickness of one hard and one soft bedrock layer together, metres.
    pub strata_thickness: f64,
    /// Ratio of erodibility between the softest and hardest bedrock.
    pub strata_contrast: f64,
    /// How far layers are folded up and down, metres.
    pub fold_amplitude: f64,
}

impl Default for PlanetParams {
    fn default() -> PlanetParams {
        PlanetParams {
            seed: 0,
            radius: 6.371e6,
            land_bias: 0.05,
            mountain_height: 3000.,
            uplift_rate: 1e-3,
            belt_frequency: 3.,
            strata_thickness: 600.,
            strata_contrast: 5.,
            fold_amplitude: 800.,
        }
    }
}

const MAX_OCTAVES: usize = 24;

/// Noise whose octaves are each rotated and offset differently.
struct Octaves {
    noise: PerlinNoise,
    frames: Vec<(Matrix3<f64>, Vector3<f64>)>,
}

impl Octaves {
    fn new(seed: u64, stream: u64) -> Octaves {
        let mut rng = Pcg32::new(seed, stream);
        let frames = (0..MAX_OCTAVES).map(|_| {
            let axis = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let rot = Rotation3::from_axis_angle(&Unit::new_normalize(axis), rng.next_f64() * std::f64::consts::TAU);
            let offset = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 256.;
            (*rot.matrix(), offset)
        }).collect();
        Octaves { noise: PerlinNoise::with_seed(seed.wrapping_mul(31).wrapping_add(stream)), frames }
    }

    fn octave(&self, k: usize, p: &Vector3<f64>, frequency: f64) -> f64 {
        let (m, o) = &self.frames[k];
        let q = m * p * frequency + o;
        self.noise.noise(q.x, q.y, q.z)
    }

    /// fBm in roughly [-1, 1].
    fn fbm(&self, p: &Vector3<f64>, frequency: f64, octaves: usize) -> f64 {
        let (mut sum, mut amp, mut norm) = (0., 1., 0.);
        for k in 0..octaves.min(MAX_OCTAVES) {
            sum += amp * self.octave(k, p, frequency * (1 << k) as f64);
            norm += amp;
            amp *= 0.5;
        }
        if norm > 0. { sum / norm } else { 0. }
    }

    /// Ridged multifractal (after Musgrave) in [0, 1]: sharp crests where the noise crosses zero,
    /// each octave weighted by the one before so detail gathers on the ridges.
    fn ridged(&self, p: &Vector3<f64>, frequency: f64, octaves: usize) -> f64 {
        let (mut sum, mut amp, mut norm, mut weight) = (0., 1., 0., 1.);
        for k in 0..octaves.min(MAX_OCTAVES) {
            let r = 1. - self.octave(k, p, frequency * (1 << k) as f64).abs();
            let r = r * r * weight;
            weight = (r * 2.).clamp(0., 1.);
            sum += amp * r;
            norm += amp;
            amp *= 0.5;
        }
        if norm > 0. { sum / norm } else { 0. }
    }
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

pub struct Planet {
    pub params: PlanetParams,
    continents: Octaves,
    belts: Octaves,
    relief: Octaves,
    folds: Octaves,
    provinces: Octaves,
    hotspots: Octaves,
}

/// Lattice units per planet radius of the continent field.
const CONTINENT_FREQUENCY: f64 = 1.2;
/// ... of the hills ...
const RELIEF_FREQUENCY: f64 = 12.;
/// ... and of the mountain ridges: a base wavelength of about 12 km on an Earth-sized planet,
/// roughly the spacing of major valleys in real ranges.
const RIDGE_FREQUENCY: f64 = 530.;

impl Planet {
    pub fn new(params: PlanetParams) -> Planet {
        let seed = params.seed;
        Planet {
            params, continents: Octaves::new(seed, 1), belts: Octaves::new(seed, 2), relief: Octaves::new(seed, 3),
            folds: Octaves::new(seed, 4), provinces: Octaves::new(seed, 5), hotspots: Octaves::new(seed, 6),
        }
    }

    /// Octaves needed, starting at `frequency` lattice units per radius, to reach detail of
    /// wavelength `finest` metres.
    fn octaves(&self, frequency: f64, finest: f64) -> usize {
        let base_wavelength = self.params.radius / frequency;
        ((base_wavelength / finest).log2().ceil().max(1.)) as usize
    }

    /// Positive on land, negative at sea; roughly in [-1, 1].
    pub fn continent(&self, d: &Vector3<f64>) -> f64 {
        self.continents.fbm(d, CONTINENT_FREQUENCY, 7) + self.params.land_bias
    }

    /// Strength of mountain building, in [0, 1].
    pub fn orogeny(&self, d: &Vector3<f64>) -> f64 {
        let belt = self.belts.ridged(d, self.params.belt_frequency, 3);
        smoothstep(0.55, 0.85, belt) * smoothstep(0., 0.12, self.continent(d))
    }

    /// Relative erodibility of the bedrock at direction `d` and elevation `z` (metres): a
    /// multiplier of the erosion coefficients, between 1/√contrast and √contrast times a
    /// regional factor. The rock is a stack of layers alternating smoothly between hard and
    /// soft, folded by noise, so erosion cutting down through it meets one then the other:
    /// hard layers hold up cliffs and benches, soft ones erode into slopes and valleys.
    /// Larger regions ("provinces") are more or less erodible overall.
    pub fn erodibility(&self, d: &Vector3<f64>, z: f64) -> f64 {
        let p = &self.params;
        let fold = p.fold_amplitude * self.folds.fbm(d, RELIEF_FREQUENCY, 4);
        let phase = (z + fold) / p.strata_thickness * std::f64::consts::TAU;
        let province = (0.5 * self.provinces.fbm(d, RELIEF_FREQUENCY, 3)).exp();
        p.strata_contrast.powf(0.5 * phase.sin()) * province
    }

    /// Where volcanoes occur, from 0 (never) to 1: along the flanks of mountain belts (as arcs
    /// stand beside the ranges at subduction zones), and over scattered hotspots.
    pub fn volcanism(&self, d: &Vector3<f64>) -> f64 {
        let belt = self.belts.ridged(d, self.params.belt_frequency, 3);
        let arc = smoothstep(0.4, 0.55, belt) * (1. - smoothstep(0.6, 0.75, belt));
        let hotspot = smoothstep(0.55, 0.7, self.hotspots.fbm(d, 8., 2));
        arc.max(hotspot)
    }

    /// Tectonic uplift rate, metres per year.
    pub fn uplift(&self, d: &Vector3<f64>) -> f64 {
        self.params.uplift_rate * self.orogeny(d)
    }

    /// Elevation above sea level at time zero, metres, with detail down to wavelength `finest`.
    pub fn elevation(&self, d: &Vector3<f64>, finest: f64) -> f64 {
        let c = self.continent(d);
        let base = if c >= 0. { 1500. * c } else { -4500. * (-6. * c).tanh() };
        let land = smoothstep(-0.02, 0.05, c);
        let hills = 150. * self.relief.fbm(d, RELIEF_FREQUENCY, self.octaves(RELIEF_FREQUENCY, finest)) * land;
        // Ridges over a raised belt; squaring the ridged noise leaves broad lowlands between
        // sharp crests.
        let ridges = self.relief.ridged(d, RIDGE_FREQUENCY, self.octaves(RIDGE_FREQUENCY, finest).max(3));
        let mountains = self.params.mountain_height * self.orogeny(d) * (0.2 + ridges * ridges);
        base + hills + mountains
    }

    /// The (latitude, longitude) in degrees of the most mountainous land at middle latitudes
    /// (30 to 55 degrees, where mountains carry valley glaciers rather than ice sheets),
    /// sampling `samples` points spread evenly over the planet (a Fibonacci lattice). Lets a
    /// scene ask for "some mountains" without knowing where this seed put them.
    pub fn most_mountainous(&self, samples: usize) -> (f64, f64) {
        self.best_site(samples, |d| self.orogeny(d) + 0.1 * self.continent(d))
    }

    /// As `most_mountainous`, but for mountainous land with volcanoes.
    pub fn most_volcanic(&self, samples: usize) -> (f64, f64) {
        self.best_site(samples, |d| self.volcanism(d).min(self.orogeny(d)) + 0.1 * self.continent(d).min(0.2))
    }

    /// As `most_mountainous`, but for a coast with mountains behind it: rivers with deltas.
    /// Candidates are judged by elevations about 50 km around them: sea on one side, mountains
    /// on another, lowland between.
    pub fn most_coastal(&self, samples: usize) -> (f64, f64) {
        self.best_site(samples, |d| {
            let centre = self.elevation(d, 5000.);
            if !(0. ..300.).contains(&centre) { return -10.; }
            let (e1, e2) = {
                let a = if d.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
                let e1 = d.cross(&a).normalize();
                (e1, d.cross(&e1))
            };
            let around: Vec<f64> = (0..8).map(|k| {
                let t = k as f64 * std::f64::consts::FRAC_PI_4;
                self.elevation(&(d + 0.008 * (e1 * t.cos() + e2 * t.sin())).normalize(), 5000.)
            }).collect();
            let low = around.iter().cloned().fold(f64::MAX, f64::min);
            let high = around.iter().cloned().fold(f64::MIN, f64::max);
            (if low < -100. { 1. } else { 0. }) + (high / 2000.).min(1.)
        })
    }

    fn best_site(&self, samples: usize, score: impl Fn(&Vector3<f64>) -> f64) -> (f64, f64) {
        let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
        let mut best = (f64::NEG_INFINITY, 0., 0.);
        for i in 0..samples {
            let y = 1. - 2. * (i as f64 + 0.5) / samples as f64;
            let lon = (golden * i as f64).rem_euclid(std::f64::consts::TAU);
            let lat = y.asin();
            let d = Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin());
            let mid = (30f64.to_radians() .. 55f64.to_radians()).contains(&lat.abs());
            let score = score(&d) + if mid { 1. } else { 0. };
            if score > best.0 { best = (score, lat.to_degrees(), lon.to_degrees()); }
        }
        (best.1, best.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::point_on_unit_sphere;

    fn planet(seed: u64) -> Planet {
        Planet::new(PlanetParams { seed, ..PlanetParams::default() })
    }

    fn samples() -> Vec<Vector3<f64>> {
        let mut rng = Pcg32::new(7, 7);
        (0..2000).map(|_| point_on_unit_sphere(rng.next_f64(), rng.next_f64())).collect()
    }

    #[test]
    fn test_planet_is_reproducible() {
        let (a, b, c) = (planet(3), planet(3), planet(4));
        let pts = samples();
        assert!(pts.iter().all(|d| a.elevation(d, 1000.) == b.elevation(d, 1000.)));
        assert!(pts.iter().any(|d| a.elevation(d, 1000.) != c.elevation(d, 1000.)));
    }

    #[test]
    fn test_planet_has_land_sea_and_mountains() {
        for seed in 0..4 {
            let p = planet(seed);
            let pts = samples();
            let land = pts.iter().filter(|d| p.elevation(d, 10000.) > 0.).count() as f64 / pts.len() as f64;
            assert!(land > 0.15 && land < 0.75, "seed {} land fraction {}", seed, land);
            let uplifting = pts.iter().filter(|d| p.uplift(d) > 0.).count();
            assert!(uplifting > 0, "seed {} has no mountains", seed);
            let (lat, lon) = p.most_mountainous(4000);
            let (up, _, _) = crate::terrain::domain::Domain::frame(lat, lon);
            assert!(p.orogeny(&up) > 0.5);
        }
    }

    #[test]
    fn test_rock_strength_varies_with_depth() {
        let p = planet(2);
        let d = Vector3::new(0.3, 0.5, 0.8).normalize();
        let k: Vec<f64> = (0..20).map(|i| p.erodibility(&d, i as f64 * 50.)).collect();
        let (lo, hi) = (k.iter().cloned().fold(f64::MAX, f64::min), k.iter().cloned().fold(0., f64::max));
        // Cutting down 1 km passes through hard and soft layers.
        assert!(hi / lo > 3., "{:?}", k);
        let any = samples().iter().any(|d| p.volcanism(d) > 0.5);
        assert!(any);
    }

    #[test]
    fn test_finer_detail_refines_rather_than_replaces() {
        // The same planet sampled for a coarser grid differs only by the omitted fine octaves.
        let p = planet(1);
        let pts = samples();
        let diff = pts.iter().map(|d| (p.elevation(d, 1000.) - p.elevation(d, 50000.)).abs()).fold(0., f64::max);
        assert!(diff < 600., "{}", diff);
    }
}
