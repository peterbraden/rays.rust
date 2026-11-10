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
}

impl Default for PlanetParams {
    fn default() -> PlanetParams {
        PlanetParams {
            seed: 0,
            radius: 6.371e6,
            land_bias: 0.05,
            mountain_height: 2500.,
            uplift_rate: 1e-3,
            belt_frequency: 3.,
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
}

/// Lattice units per planet radius of the continent field.
const CONTINENT_FREQUENCY: f64 = 1.2;
/// ... of the hills ...
const RELIEF_FREQUENCY: f64 = 12.;
/// ... and of the mountain ridges: a base wavelength of about 40 km on an Earth-sized planet,
/// the spacing of major valleys in real ranges.
const RIDGE_FREQUENCY: f64 = 160.;

impl Planet {
    pub fn new(params: PlanetParams) -> Planet {
        let seed = params.seed;
        Planet { params, continents: Octaves::new(seed, 1), belts: Octaves::new(seed, 2), relief: Octaves::new(seed, 3) }
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
        let mountains = self.params.mountain_height * self.orogeny(d) * (0.25 + 0.75 * ridges * ridges);
        base + hills + mountains
    }

    /// The (latitude, longitude) in degrees of the most mountainous land at middle latitudes
    /// (30 to 55 degrees, where mountains carry valley glaciers rather than ice sheets),
    /// sampling `samples` points spread evenly over the planet (a Fibonacci lattice). Lets a
    /// scene ask for "some mountains" without knowing where this seed put them.
    pub fn most_mountainous(&self, samples: usize) -> (f64, f64) {
        let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
        let mut best = (f64::NEG_INFINITY, 0., 0.);
        for i in 0..samples {
            let y = 1. - 2. * (i as f64 + 0.5) / samples as f64;
            let lon = (golden * i as f64).rem_euclid(std::f64::consts::TAU);
            let lat = y.asin();
            let d = Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin());
            let mid = (30f64.to_radians() .. 55f64.to_radians()).contains(&lat.abs());
            let score = self.orogeny(&d) + 0.1 * self.continent(&d) + if mid { 1. } else { 0. };
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
    fn test_finer_detail_refines_rather_than_replaces() {
        // The same planet sampled for a coarser grid differs only by the omitted fine octaves.
        let p = planet(1);
        let pts = samples();
        let diff = pts.iter().map(|d| (p.elevation(d, 1000.) - p.elevation(d, 50000.)).abs()).fold(0., f64::max);
        assert!(diff < 600., "{}", diff);
    }
}
