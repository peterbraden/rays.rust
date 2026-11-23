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
//! - **Bedrock.** A folded sedimentary cover over crystalline basement, with granite plutons
//!   (`structure`, and `geology` for the model): the lithology that erosion exposes as it cuts
//!   down, each rock resisting rivers and glaciers differently. In the mountain belts the cover
//!   is folded along the belt's length, and the folds are part of the relief (`fold_relief`).
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
use crate::terrain::geology::{self, Column, GeologyParams, Rock, Structure, Trend};
use crate::terrain::tectonics::{Crust, TectonicsParams};
use std::sync::{Arc, OnceLock};

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
    /// The bedrock: its rocks, layers and folds.
    pub geology: GeologyParams,
    /// Uplift of the plateaus, broad regions of continent away from the mountain belts raised
    /// without folding (the Colorado Plateau, the Ozarks), metres per year (`plateau`).
    pub plateau_uplift: f64,
    /// How far the plateaus stand above the lowlands at time zero, metres.
    pub plateau_height: f64,
    /// Plate tectonics, to place the continents, ocean basins, mountain belts and volcanoes;
    /// None for noise fields alone.
    pub tectonics: Option<TectonicsParams>,
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
            geology: GeologyParams::default(),
            plateau_uplift: 2e-4,
            plateau_height: 800.,
            tectonics: None,
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
    plutons: Octaves,
    plunges: Octaves,
    plateaus: Octaves,
    /// The crust from plate tectonics, if simulated.
    pub crust: Option<Arc<Crust>>,
    /// The sedimentary column.
    pub column: Column,
    /// Which way the mountain belts are shortened, found when first asked.
    trend: OnceLock<Trend>,
}

/// Lattice units per planet radius of the continent field.
const CONTINENT_FREQUENCY: f64 = 1.2;
/// ... of the coast's detail below the tectonic grid ...
const COAST_FREQUENCY: f64 = 40.;
/// ... of the hills ...
const RELIEF_FREQUENCY: f64 = 12.;
/// ... and of the mountain ridges: a base wavelength of about 12 km on an Earth-sized planet,
/// roughly the spacing of major valleys in real ranges.
const RIDGE_FREQUENCY: f64 = 530.;
/// ... of plateaus: regions several hundred kilometres across ...
const PLATEAU_FREQUENCY: f64 = 8.;
/// ... and of granite plutons: bodies some tens of kilometres across.
const PLUTON_FREQUENCY: f64 = 160.;

impl Planet {
    pub fn new(params: PlanetParams) -> Planet {
        let seed = params.seed;
        let crust = params.tectonics.as_ref().map(|t| crate::terrain::tectonics::crust(t, seed, params.radius));
        let column = Column::new(&params.geology, seed);
        Planet {
            params, continents: Octaves::new(seed, 1), belts: Octaves::new(seed, 2), relief: Octaves::new(seed, 3),
            folds: Octaves::new(seed, 4), provinces: Octaves::new(seed, 5), hotspots: Octaves::new(seed, 6),
            plutons: Octaves::new(seed, 7), plunges: Octaves::new(seed, 8), plateaus: Octaves::new(seed, 9),
            column,
            crust, trend: OnceLock::new(),
        }
    }

    /// Octaves needed, starting at `frequency` lattice units per radius, to reach detail of
    /// wavelength `finest` metres.
    fn octaves(&self, frequency: f64, finest: f64) -> usize {
        let base_wavelength = self.params.radius / frequency;
        ((base_wavelength / finest).log2().ceil().max(1.)) as usize
    }

    /// The large-scale elevation from the crust, with the coast broken up below the tectonic
    /// grid's resolution, metres.
    fn crust_elevation(&self, crust: &Crust, d: &Vector3<f64>) -> f64 {
        let e = crust.elevation(d);
        // Bays, peninsulas and islands: relief at wavelengths from about the grid's cells down
        // to tens of kilometres, strongest near sea level.
        let coast = self.continents.fbm(d, COAST_FREQUENCY, 5);
        let m = crust.margin(d);
        // Strongest where continent meets ocean; island arcs and hotspots are raised enough by
        // themselves.
        e + 1400. * coast * (4. * m * (1. - m)).min(1.)
    }

    /// Elevation at the scale of regions (hundreds of kilometres), metres: the crust's, or the
    /// continents and mountain belts without their ridges. For the climate, whose winds rise
    /// over ranges, not over every ridge.
    pub fn regional_elevation(&self, d: &Vector3<f64>) -> f64 {
        if let Some(c) = &self.crust { return self.crust_elevation(c, d); }
        let c = self.continent(d);
        let base = if c >= 0. { 1500. * c } else { -4500. * (-6. * c).tanh() };
        base + self.params.mountain_height * self.orogeny(d) * 0.5
    }

    /// Positive on land, negative at sea; roughly in [-1, 1].
    pub fn continent(&self, d: &Vector3<f64>) -> f64 {
        if let Some(c) = &self.crust {
            return (self.crust_elevation(c, d) / 1000.).tanh();
        }
        self.continents.fbm(d, CONTINENT_FREQUENCY, 7) + self.params.land_bias
    }

    /// Strength of mountain building, in [0, 1].
    pub fn orogeny(&self, d: &Vector3<f64>) -> f64 {
        if let Some(c) = &self.crust {
            return c.orogeny(d).clamp(0., 1.) * smoothstep(-0.1, 0.1, self.continent(d));
        }
        let belt = self.belts.ridged(d, self.params.belt_frequency, 3);
        smoothstep(0.55, 0.85, belt) * smoothstep(0., 0.12, self.continent(d))
    }

    /// Which way the mountain belts are shortened (`geology::Trend`), found when first asked.
    pub fn trend(&self) -> &Trend {
        self.trend.get_or_init(|| Trend::new(self.params.radius, |d| self.orogeny(d)))
    }

    /// The folds of the mountain belts at `d`, where the belts' strength is `orogeny` and detail
    /// of wavelength `finest` is resolved: their shape (`Structure::crest`), how far they raise
    /// the strata, metres (`fold`), how far thrusts offset the strata, metres (`thrust`), and
    /// how fractured the rock is (`fracture`).
    fn belt_folds(&self, d: &Vector3<f64>, orogeny: f64, finest: f64) -> geology::Fold {
        let g = &self.params.geology;
        let none = geology::Fold { shape: 0., thrust: 0., fracture: 0. };
        let fade = geology::fade(finest, g.fold_wavelength);
        if orogeny <= 1e-3 || fade <= 0. { return none; }
        let trend = self.trend();
        let across = trend.shortening(d);
        // Leaning towards the foreland: upright at the belt's centre, where it is fanned.
        let lean = g.fold_vergence * across.dot(&trend.foreland(d)).tanh();
        let f = geology::fold(self.params.seed, &(d * self.params.radius), &across, g.fold_wavelength, lean);
        // Folds rise and fall along their axes over a few wavelengths: periclines.
        let plunge = smoothstep(-0.35, 0.35, self.plunges.fbm(d, self.params.radius / (5. * g.fold_wavelength), 2));
        geology::Fold {
            shape: orogeny * fade * (0.25 + 0.75 * plunge) * f.shape,
            thrust: g.thrust_throw * orogeny * fade * f.thrust,
            fracture: orogeny.min(1.) * fade * f.fracture,
        }
    }

    /// The bedrock's structure at `d` (`geology::Structure`), with folds down to wavelength
    /// `finest` metres.
    pub fn structure(&self, d: &Vector3<f64>, finest: f64) -> Structure {
        let g = &self.params.geology;
        let o = self.orogeny(d);
        let folds = self.belt_folds(d, o, finest);
        let (crest, fold, thrust) = (folds.shape, g.fold_amplitude * folds.shape, folds.thrust);
        let warp = g.warp_amplitude * self.folds.fbm(d, RELIEF_FREQUENCY, 4);
        let cover = g.cover_thickness * (0.4 * self.provinces.fbm(d, RELIEF_FREQUENCY, 3)).exp();
        let basement = -cover + g.exhumation * o * o;
        // Granite rises as domes, mostly where there was magma: under belts and arcs.
        let magma = o.max(self.volcanism(d));
        let strength = self.plutons.fbm(d, PLUTON_FREQUENCY, 3) + 0.35 * magma - 0.55;
        let pluton = if g.plutons > 0. { basement - 1000. + 10000. * g.plutons * strength } else { -1e5 };
        Structure {
            fold: fold + thrust + warp, crest, fracture: folds.fracture,
            oceanic: match &self.crust {
                Some(c) => 1. - c.margin(d),
                None => 1. - smoothstep(-0.03, 0.03, self.continent(d)),
            }, basement, pluton, grade: o,
            beds: (0.3 * self.provinces.octave(2, d, 23.)).exp(),
            shift: 3000. * self.provinces.octave(3, d, 9.),
            schist: self.provinces.octave(1, d, 37.),
        }
    }

    /// The bedrock at direction `d` and elevation `z` (metres) at time zero, with folds down to
    /// wavelength `finest`.
    pub fn rock(&self, d: &Vector3<f64>, z: f64, finest: f64) -> Rock {
        let s = self.structure(d, finest);
        self.column.rock(&s, s.coordinate(z, 0.))
    }

    /// Where volcanoes occur, from 0 (never) to 1: along the flanks of mountain belts (as arcs
    /// stand beside the ranges at subduction zones), and over scattered hotspots.
    pub fn volcanism(&self, d: &Vector3<f64>) -> f64 {
        if let Some(c) = &self.crust { return c.volcanism(d).clamp(0., 1.); }
        let belt = self.belts.ridged(d, self.params.belt_frequency, 3);
        let arc = smoothstep(0.4, 0.55, belt) * (1. - smoothstep(0.6, 0.75, belt));
        let hotspot = smoothstep(0.55, 0.7, self.hotspots.fbm(d, 8., 2));
        arc.max(hotspot)
    }

    /// How much of a plateau `d` is on, 0 to 1: broad regions of continent away from the
    /// mountain belts, raised as a whole without being folded, so their strata lie flat. Rivers
    /// cut down through them into canyons, and their hard beds hold up the mesas and buttes
    /// that the retreating scarps leave behind. (Unfolded continent elsewhere has no uplift, so
    /// its flat beds are barely cut into.)
    pub fn plateau(&self, d: &Vector3<f64>) -> f64 {
        let p = &self.params;
        if p.plateau_uplift == 0. && p.plateau_height == 0. { return 0.; }
        let field = smoothstep(0.05, 0.3, self.plateaus.fbm(d, PLATEAU_FREQUENCY, 3));
        field * smoothstep(0.05, 0.25, self.continent(d)) * (1. - smoothstep(0.05, 0.3, self.orogeny(d)))
    }

    /// Tectonic uplift rate, metres per year: the mountain belts', and the plateaus'.
    pub fn uplift(&self, d: &Vector3<f64>) -> f64 {
        let plateau = self.params.plateau_uplift * self.plateau(d);
        if let Some(c) = &self.crust {
            // Only where the crust is being thickened now.
            return self.params.uplift_rate * c.activity(d).clamp(0., 1.) * smoothstep(-0.1, 0.1, self.continent(d)) + plateau;
        }
        self.params.uplift_rate * self.orogeny(d) + plateau
    }

    /// As `most_mountainous`, but for a plateau of flat strata in the subtropics, where
    /// deserts are, capped by a hard bed (sandstone, limestone, quartzite) over soft: mesa
    /// country. (Soft rock on top, the canyons strip it back to benches on the hard bed below
    /// rather than leaving mesas.)
    pub fn most_plateau(&self, samples: usize) -> (f64, f64) {
        self.best_site_in(samples, 15. ..35., |d| {
            let plateau = self.plateau(d);
            if plateau <= 0. { return -10.; }
            let top = self.elevation(d, 5000.);
            let hard = |z: f64| matches!(self.rock(d, z, 5000.), Rock::Sandstone | Rock::Limestone | Rock::Quartzite);
            // A hard bed at the surface, over soft rock within a few hundred metres.
            let cap = hard(top) && (1..=6).any(|k| !hard(top - 60. * k as f64));
            2. * plateau + 0.1 * self.continent(d) + if cap { 1. } else { 0. }
        })
    }

    /// Elevation above sea level at time zero, metres, with detail down to wavelength `finest`.
    pub fn elevation(&self, d: &Vector3<f64>, finest: f64) -> f64 {
        if let Some(crust) = &self.crust {
            let base = self.crust_elevation(crust, d);
            let land = smoothstep(-100., 200., base);
            let hills = 150. * self.relief.fbm(d, RELIEF_FREQUENCY, self.octaves(RELIEF_FREQUENCY, finest)) * land;
            // The crust's thickening already raised the belt as a whole: the ridges are relief
            // about that.
            let ridges = self.relief.ridged(d, RIDGE_FREQUENCY, self.octaves(RIDGE_FREQUENCY, finest).max(3));
            let o = self.orogeny(d);
            let crest = self.belt_folds(d, o, finest).shape;
            let fold = self.params.geology.fold_amplitude * crest;
            let mountains = self.params.mountain_height * o * (ridges * ridges - 0.2) * self.fold_emphasis(crest);
            return base + hills + mountains + self.params.geology.fold_relief * fold + self.params.plateau_height * self.plateau(d);
        }
        let c = self.continent(d);
        let base = if c >= 0. { 1500. * c } else { -4500. * (-6. * c).tanh() };
        let land = smoothstep(-0.02, 0.05, c);
        let hills = 150. * self.relief.fbm(d, RELIEF_FREQUENCY, self.octaves(RELIEF_FREQUENCY, finest)) * land;
        // Ridges over a raised belt; squaring the ridged noise leaves broad lowlands between
        // sharp crests.
        let ridges = self.relief.ridged(d, RIDGE_FREQUENCY, self.octaves(RIDGE_FREQUENCY, finest).max(3));
        let o = self.orogeny(d);
        let crest = self.belt_folds(d, o, finest).shape;
        let fold = self.params.geology.fold_amplitude * crest;
        let mountains = self.params.mountain_height * o * (0.2 + ridges * ridges) * self.fold_emphasis(crest);
        base + hills + mountains + self.params.geology.fold_relief * fold + self.params.plateau_height * self.plateau(d)
    }

    /// How much the ridges are raised where the folds' shape is `crest`: the ranges stand on
    /// the anticlines and the valleys follow the synclines, so the peaks line up along the
    /// folds' axes.
    fn fold_emphasis(&self, crest: f64) -> f64 {
        (1. + self.params.geology.fold_relief * crest).max(0.)
    }

    /// The (latitude, longitude) in degrees of the most mountainous land at middle latitudes
    /// (30 to 55 degrees, where mountains carry valley glaciers rather than ice sheets),
    /// sampling `samples` points spread evenly over the planet (a Fibonacci lattice). Lets a
    /// scene ask for "some mountains" without knowing where this seed put them.
    pub fn most_mountainous(&self, samples: usize) -> (f64, f64) {
        self.best_site(samples, |d| self.orogeny(d) + self.crust.as_ref().map_or(0., |c| 0.5 * c.activity(d)) + 0.1 * self.continent(d))
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
        self.best_site_in(samples, 30. ..55., score)
    }

    /// The best site by `score`, preferring latitudes (degrees, north or south) in `band`.
    fn best_site_in(&self, samples: usize, band: std::ops::Range<f64>, score: impl Fn(&Vector3<f64>) -> f64) -> (f64, f64) {
        let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
        let mut best = (f64::NEG_INFINITY, 0., 0.);
        for i in 0..samples {
            let y = 1. - 2. * (i as f64 + 0.5) / samples as f64;
            let lon = (golden * i as f64).rem_euclid(std::f64::consts::TAU);
            let lat = y.asin();
            let d = Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin());
            let mid = band.contains(&lat.abs().to_degrees());
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
        // On land outside the mountain belts, in the sedimentary cover (the sea floor is basalt
        // all the way down, and the belts' cores crystalline).
        let d = samples().into_iter().find(|d| p.continent(d) > 0.3 && p.orogeny(d) < 0.05 && p.structure(d, 100.).basement < -1500.).unwrap();
        assert!(p.structure(&d, 100.).oceanic < 0.5);
        let k: Vec<f64> = (0..20).map(|i| p.rock(&d, i as f64 * 50., 100.).erodibility(geology::Process::Rivers, 5.)).collect();
        let (lo, hi) = (k.iter().cloned().fold(f64::MAX, f64::min), k.iter().cloned().fold(0., f64::max));
        // Cutting down 1 km passes through hard and soft layers.
        assert!(hi / lo > 2., "{:?}", k);
        // At sea, basalt.
        let sea = samples().into_iter().find(|d| p.continent(d) < -0.3).unwrap();
        assert_eq!(p.rock(&sea, -4000., 100.), geology::Rock::Basalt);
        let any = samples().iter().any(|d| p.volcanism(d) > 0.5);
        assert!(any);
    }

    #[test]
    fn test_belts_have_no_seams() {
        // Walking across and along a mountain belt in small steps, the ground never jumps: no
        // seams where the trend's direction or the folds' phase changes.
        let p = planet(0);
        let (lat, lon) = p.most_mountainous(4000);
        let (up, east, north) = crate::terrain::domain::Domain::frame(lat, lon);
        let step = 25.;
        let mut worst: f64 = 0.;
        for k in 0..8 {
            let a = k as f64 * std::f64::consts::PI / 8.;
            let dir = east * a.cos() + north * a.sin();
            let at = |i: i64| (up + dir * (i as f64 * step / p.params.radius)).normalize();
            let mut last = p.elevation(&at(-8000), 50.);
            for i in -7999..8000 {
                let e = p.elevation(&at(i), 50.);
                worst = worst.max((e - last).abs() / step);
                last = e;
            }
        }
        // Ridge flanks reach slopes of about 4; a seam would be a single step of up to the
        // folds' full relief, slope 60.
        assert!(worst < 6., "a step of slope {}", worst);
    }

    #[test]
    fn test_finer_detail_refines_rather_than_replaces() {
        // The same planet sampled for a coarser grid differs only by the omitted fine octaves.
        let p = planet(1);
        let pts = samples();
        let diff = pts.iter().map(|d| (p.elevation(d, 1000.) - p.elevation(d, 50000.)).abs()).fold(0., f64::max);
        // The ridges' fine octaves, and the folds, which are too narrow for the coarse grid: their
        // relief, and the ridges' emphasis on the anticlines (up to the mountains' height,
        // 1.2 H at most, times the share).
        let g = &p.params.geology;
        assert!(diff < 600. + g.fold_relief * (g.fold_amplitude + 1.2 * p.params.mountain_height), "{}", diff);
        // But the folds average out: the coarse grid isn't higher or lower overall.
        let bias = pts.iter().map(|d| p.elevation(d, 1000.) - p.elevation(d, 50000.)).sum::<f64>() / pts.len() as f64;
        assert!(bias.abs() < 50., "{}", bias);
    }
}
