//! # Geology: what the ground is made of, and how it is folded
//!
//! Erosion works on rock, and rocks differ. This module describes the bedrock under a planet's
//! surface: which rock lies at each point and depth (`Rock`), and how strongly rivers and
//! glaciers wear each one away. Rivers and glaciers attack rock differently, so the two
//! erodibilities are separate, and the landscape each process leaves depends on the rock it
//! cuts. See docs/geology.md for the model as a whole; the parts:
//!
//! ## Rocks, and the two ways of eroding them
//!
//! **Rivers** cut bedrock mostly by the impacts of the gravel they carry. Sklar and Dietrich
//! ("Sediment and rock strength controls on river incision into bedrock", Geology 29(12), 2001)
//! measured abrasion rates in the laboratory falling as the square of the rock's tensile
//! strength, over two orders of magnitude from weak mudstone to quartzite; Stock and Montgomery
//! ("Geologic constraints on bedrock river incision using the stream power law", JGR 104(B3),
//! 1999) found stream power coefficients K varying by orders of magnitude between lithologies in
//! the field. So for rivers the order is that of strength: shale and schist erode fastest,
//! sandstone and limestone in between, then gneiss, granite and basalt, and quartzite slowest.
//!
//! **Glaciers** erode by abrasion, where debris in the ice scratches the bed, and by quarrying
//! (plucking), where the ice pulls away blocks already loosened along joints. Quarrying is the
//! larger part where the bed is fractured (Hallet, "Glacial quarrying: a simple theoretical
//! model", Annals of Glaciology 22, 1996; Iverson, "A theory of glacial quarrying for landscape
//! evolution models", Geology 40(8), 2012), so the spacing of a rock's joints matters as much as
//! its strength. Krabbendam and Glasser ("Glacial erosion and bedrock properties in NW Scotland:
//! abrasion and plucking, hardness and joint spacing", Geomorphology 130, 2011) found glacial
//! landforms on hard rocks controlled by joint spacing, and Dühnforth, Anderson, Ward and
//! Stock ("Bedrock fracture control of glacial erosion processes and rates", Geology 38(5),
//! 2010) found the Sierra Nevada's glaciers deepening valleys in closely fractured granite while
//! sparsely fractured granite stood as domes. So for glaciers the order changes:
//!
//! - **Massive granite**, with joints metres to tens of metres apart, resists ice best of all:
//!   domes and roches moutonnées, while rivers wear it only moderately slowly.
//! - **Basalt** lava is strong, and resists rivers, but its columnar joints make it easy to
//!   quarry.
//! - **Quartzite** is the hardest rock for rivers, but brittle and closely jointed, so glaciers
//!   pluck it readily.
//! - **Schist**, split along its foliation, is quarried easily; **limestone** is soft (Mohs 3)
//!   and so abraded easily, though strong against rivers' impacts.
//!
//! Folding and faulting fracture rock (`Fold::fracture`: anticline crests, whose outer arcs
//! are stretched, and thrusts' damage zones), and fractured rock is weaker (Molnar, Anderson
//! and Anderson, "Tectonics, fracturing of rock, and erosion", JGR 112, 2007): to glaciers
//! fully, as quarrying works on fractures, and to rivers by a share, as they mostly abrade
//! (`fractured`, `fracturing`).
//!
//! Each rock has a score for each process (`Rock::scores`) between -1 (most resistant) and 1
//! (least), and its erodibility, relative to average bedrock, is `contrast^(score / 2)`: the
//! hardest rock is `contrast` times less erodible than the softest (`GeologyParams::contrast`,
//! 5 by default; Perne et al., Earth Surface Dynamics 5, 2017, find layering matters to
//! landscapes from about threefold). The scores order the rocks as the sources above do, but
//! their values are a model choice, not measurements. Sediment and lava have their own
//! erodibilities (`TerrainParams`).
//!
//! ## The column, folded
//!
//! Above a crystalline basement (gneiss, or schist where the noise `schist` is positive) lies
//! a sedimentary cover a few kilometres thick (`cover_thickness`), of hard beds (sandstone or
//! limestone) alternating with soft (shale), each a few hundred metres thick (`Column`; the
//! pair averaging `strata_thickness`), repeating upwards. In mountain belts long erosion before
//! the simulation began has brought the basement up (`exhumation`, as the belt's strength
//! squared), so the cores of the strongest ranges are crystalline and their flanks sedimentary
//! fold belts, as in the Alps or the Canadian Rockies; and the cover near the basement there is
//! metamorphosed, sandstone to quartzite and shale to schist (`grade`). Granite plutons rise
//! into it as domes, mostly in the belts and volcanic arcs (`pluton`, the height of their roof).
//!
//! Rock moves with the ground: uplift carries the strata up, so erosion keeping pace with it
//! cuts down into older rock (`Terrain::exhumed`). A point's position in the column is its
//! *rock coordinate*, ζ = z - exhumed - fold: its elevation, less the uplift since time zero
//! and the folding.
//!
//! ## Folds that shape the ranges
//!
//! Where crust is shortened it buckles into folds whose axes run across the direction of
//! shortening, so along the length of the belt: the parallel ridges of the Appalachians' Valley
//! and Ridge, the whaleback anticlines of the Zagros, the Jura. Here the direction of
//! shortening is taken to be across the belt, the way its strength changes fastest. The belt's
//! strength (`Planet::orogeny`, from plate tectonics or noise) is sampled on a coarse sphere,
//! and its gradient's structure tensor (Bigün and Granlund, "Optimal orientation detection of
//! linear symmetry", ICCV 1987; Knutsson, "Representing local structure using tensors", 1989),
//! smoothed (`Trend`): unlike the gradient itself, the tensor doesn't vanish or flip at a belt's
//! crest, where the gradients on either side point opposite ways.
//!
//! The folds themselves are phasor noise (Tricard, Efremov, Zanni, Neyret, Martínez and
//! Lefebvre, "Procedural Phasor Noise", ACM TOG 38(4), 2019, doi:10.1145/3306346.3322990):
//! a sum of Gabor kernels whose waves all run along the local trend, of which only the phase is
//! kept, so the result is a sinusoid of constant amplitude whose crests follow the trend,
//! continuous, ending and branching where kernels disagree, as real fold trains do (en échelon
//! folds, plunging terminations). Each kernel's wave vector is the trend at the point evaluated
//! rather than at the kernel, so the field stays continuous as the trend turns. Along their
//! axes the folds' amplitude rises and falls (`plunge` noise a few wavelengths long), making
//! doubly plunging periclines.
//!
//! The folds lean towards the foreland (`fold_vergence`), as those of fold-thrust belts do:
//! the steep limb of each anticline faces away from the belt's centre, which lies up the
//! belt's strength (`Trend::foreland`, the smoothed gradient of that strength, fading to
//! nothing at the centre, where real orogens fan out both ways; see `fold_shape`).
//!
//! Where they lean, their forelimbs break along thrusts that repeat the beds towards the
//! foreland (`thrust_throw`, `fold`).
//!
//! Folding shapes the landscape three ways:
//!
//! 1. **Initial relief**: a share (`fold_relief`) of the structural relief is topography at
//!    time zero, young folds standing as ridges (anticlines) and valleys (synclines).
//! 2. **Uplift**: active folds keep growing, so anticlines rise faster than synclines,
//!    `U (1 + fold_growth × crest)` (Terrain's uplift).
//! 3. **Lithology**: the strata follow the folds, so as erosion cuts down it exposes hard and
//!    soft beds in bands along the belt. Rivers follow the soft beds, leaving the hard ones as
//!    hogbacks and strike ridges; breached anticlines open into valleys over soft cores;
//!    synclines with hard beds in their cores can end up as the ridges (inverted relief).
//!
//! Folds narrower than about the grid's cells are left out (`fade`), like the octaves of the
//! planet's relief, so a coarse simulation sees only the broad structure.

use crate::geometry::Pcg32;
use crate::na::{Matrix3, Vector3};
use crate::terrain::domain::Domain;
use rayon::prelude::*;
use std::f64::consts::TAU;

#[derive(Clone, Debug, PartialEq)]
pub struct GeologyParams {
    /// Mean thickness of a hard and a soft bed of the sedimentary cover together, metres.
    pub strata_thickness: f64,
    /// Ratio of erodibility between the softest and hardest rocks (for each process).
    pub contrast: f64,
    /// Structural amplitude of the folds in mountain belts, metres.
    pub fold_amplitude: f64,
    /// Wavelength of the folds, crest to crest, metres.
    pub fold_wavelength: f64,
    /// Share of the folds' structural relief that is topography at time zero.
    pub fold_relief: f64,
    /// How much faster anticline crests are uplifted than the belt as a whole (and synclines
    /// slower): uplift times 1 + fold_growth at a crest.
    pub fold_growth: f64,
    /// How far folds lean towards the foreland, 0 (upright) to below 1: their steep limbs face
    /// away from the belt's centre, by up to 1 + fold_vergence times the gentle limb's slope.
    pub fold_vergence: f64,
    /// Throw of the thrusts on the leaning folds' forelimbs, metres: how far each sheet's
    /// strata ride up over the next.
    pub thrust_throw: f64,
    /// How much more erodible the most fractured rock (anticline crests, thrust zones) is to
    /// glaciers, which quarry it: 1 doubles it; rivers feel a third of this.
    pub fracturing: f64,
    /// Gentle undulation of the strata everywhere, metres.
    pub warp_amplitude: f64,
    /// Typical thickness of the sedimentary cover over the basement, metres.
    pub cover_thickness: f64,
    /// How far the basement has been brought up in the strongest mountain belts before time
    /// zero, metres.
    pub exhumation: f64,
    /// How many granite plutons there are: 0 for none, 1 for the default.
    pub plutons: f64,
}

impl Default for GeologyParams {
    fn default() -> GeologyParams {
        GeologyParams {
            strata_thickness: 600., contrast: 5., fold_amplitude: 1500., fold_wavelength: 8000.,
            fold_relief: 0.5, fold_growth: 0.5, fold_vergence: 0.6, thrust_throw: 800., fracturing: 1.5, warp_amplitude: 400., cover_thickness: 3000.,
            exhumation: 4000., plutons: 1.,
        }
    }
}

/// A kind of rock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rock {
    /// Loose sediment: alluvium, lake fill, colluvium.
    Sediment,
    Shale,
    Sandstone,
    Limestone,
    /// Metamorphosed sandstone.
    Quartzite,
    /// Metamorphosed shale, and foliated basement.
    Schist,
    /// Basement.
    Gneiss,
    /// Plutons.
    Granite,
    /// Lava.
    Basalt,
}

/// The two ways of eroding rock, which rocks resist differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process {
    Rivers,
    Glaciers,
}

impl Rock {
    pub const ALL: [Rock; 9] = [Rock::Sediment, Rock::Shale, Rock::Sandstone, Rock::Limestone, Rock::Quartzite,
        Rock::Schist, Rock::Gneiss, Rock::Granite, Rock::Basalt];

    /// The rock's number, as the `rock` field gives it (`graph::Field::Rock`).
    pub fn index(self) -> usize { Rock::ALL.iter().position(|&r| r == self).unwrap() }

    pub fn from_index(i: usize) -> Rock { Rock::ALL[i.min(Rock::ALL.len() - 1)] }

    pub fn name(self) -> &'static str {
        match self {
            Rock::Sediment => "sediment", Rock::Shale => "shale", Rock::Sandstone => "sandstone",
            Rock::Limestone => "limestone", Rock::Quartzite => "quartzite", Rock::Schist => "schist",
            Rock::Gneiss => "gneiss", Rock::Granite => "granite", Rock::Basalt => "basalt",
        }
    }

    /// How easily (rivers, glaciers) erode the rock, each from -1 (most resistant) to 1 (least):
    /// see the module docs. Sediment and basalt have erodibilities of their own
    /// (`TerrainParams`); their scores here are for the `hardness` field alone.
    pub fn scores(self) -> (f64, f64) {
        match self {
            Rock::Sediment => (1.4, 1.4),
            Rock::Shale => (1.0, 0.7),
            Rock::Sandstone => (0.0, 0.3),
            Rock::Limestone => (-0.3, 0.2),
            Rock::Quartzite => (-1.0, -0.1),
            Rock::Schist => (0.4, 0.8),
            Rock::Gneiss => (-0.6, -0.5),
            Rock::Granite => (-0.7, -1.0),
            Rock::Basalt => (-1.5, 0.5),
        }
    }

    /// Erodibility by `process`, relative to average bedrock, for a contrast `contrast` between
    /// the softest and hardest.
    pub fn erodibility(self, process: Process, contrast: f64) -> f64 {
        let (r, g) = self.scores();
        contrast.max(1.).powf(0.5 * if process == Process::Rivers { r } else { g })
    }

    /// The rock's colour on a geological map (sRGB), in the customary hues: yellows for
    /// sandstone, blue for limestone, reds and pinks for granite and gneiss.
    pub fn map_colour(self) -> [f64; 3] {
        match self {
            Rock::Sediment => [0.96, 0.94, 0.80],
            Rock::Shale => [0.55, 0.62, 0.52],
            Rock::Sandstone => [0.95, 0.80, 0.35],
            Rock::Limestone => [0.45, 0.65, 0.88],
            Rock::Quartzite => [0.98, 0.95, 0.62],
            Rock::Schist => [0.62, 0.45, 0.70],
            Rock::Gneiss => [0.90, 0.58, 0.45],
            Rock::Granite => [0.90, 0.30, 0.38],
            Rock::Basalt => [0.25, 0.42, 0.25],
        }
    }

    /// The colour of the bare rock (its albedo, linear), weathered as it would be seen from a
    /// distance.
    pub fn colour(self) -> [f64; 3] {
        match self {
            Rock::Sediment => [0.20, 0.17, 0.13],
            Rock::Shale => [0.11, 0.105, 0.10],
            Rock::Sandstone => [0.34, 0.24, 0.15],
            Rock::Limestone => [0.42, 0.40, 0.35],
            Rock::Quartzite => [0.46, 0.44, 0.40],
            Rock::Schist => [0.17, 0.17, 0.16],
            Rock::Gneiss => [0.24, 0.22, 0.21],
            Rock::Granite => [0.36, 0.32, 0.29],
            Rock::Basalt => [0.06, 0.055, 0.05],
        }
    }
}

/// The geology at a point of the surface: smooth fields, from which the rock at any depth
/// follows (`Column::rock`). They interpolate between a simulation's cells, so the rock can be
/// looked up wherever a ray hits the ground.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Structure {
    /// How far folding has raised the strata here, metres (negative in synclines).
    pub fold: f64,
    /// The folds' shape here, from -1 in a syncline's trough to 1 at an anticline's crest,
    /// times their strength (0 outside the mountain belts, or where too narrow to resolve).
    pub crest: f64,
    /// Rock coordinate of the top of the crystalline basement, metres.
    pub basement: f64,
    /// Rock coordinate of the roof of granite, metres (far below where there is none).
    pub pluton: f64,
    /// Metamorphic grade of the cover near the basement, 0 to 1.
    pub grade: f64,
    /// Thickness of the beds relative to the column's.
    pub beds: f64,
    /// The column's offset here, metres: which bed lies at a given depth changes over hundreds
    /// of kilometres, as a basin's facies do.
    pub shift: f64,
    /// Where positive, the basement is schist rather than gneiss.
    pub schist: f64,
    /// How fractured the bedrock is by folding and faulting, 0 to 1 (`Fold::fracture`).
    pub fracture: f64,
}

impl Structure {
    /// The rock coordinate (see the module docs) of rock at elevation `z`, which uplift has
    /// carried `exhumed` metres up since time zero.
    pub fn coordinate(&self, z: f64, exhumed: f64) -> f64 { z - exhumed - self.fold }

    /// The fields as an array, to average or interpolate.
    pub fn to_array(&self) -> [f64; 9] {
        [self.fold, self.crest, self.basement, self.pluton, self.grade, self.beds, self.shift, self.schist, self.fracture]
    }

    pub fn from_array(a: [f64; 9]) -> Structure {
        Structure { fold: a[0], crest: a[1], basement: a[2], pluton: a[3], grade: a[4], beds: a[5], shift: a[6], schist: a[7], fracture: a[8] }
    }
}

/// The cover's metamorphosed within this far above the basement, metres.
const METAMORPHIC_DEPTH: f64 = 2500.;

/// The sedimentary column: beds repeating upwards and downwards, alternately hard and soft.
#[derive(Clone, Debug)]
pub struct Column {
    /// Each bed's rock and the coordinate of its top, from the bottom of one repeat.
    beds: Vec<(Rock, f64)>,
    /// Thickness of one repeat, metres.
    period: f64,
}

impl Column {
    /// Beds in one repeat of the column.
    const BEDS: usize = 10;

    pub fn new(p: &GeologyParams, seed: u64) -> Column {
        let mut rng = Pcg32::new(seed, 0x6e01_0617);
        let mut top = 0.;
        let beds = (0..Column::BEDS).map(|k| {
            let rock = if k % 2 == 1 { Rock::Shale } else if rng.next_f64() < 0.55 { Rock::Sandstone } else { Rock::Limestone };
            top += 0.5 * p.strata_thickness.max(1.) * (0.3 + 1.4 * rng.next_f64());
            (rock, top)
        }).collect();
        Column { beds, period: top }
    }

    /// The bed at rock coordinate `zeta` of the cover, for beds `scale` times their thickness.
    pub fn bed(&self, zeta: f64, scale: f64) -> Rock {
        let u = (zeta / scale.max(1e-3)).rem_euclid(self.period);
        self.beds.iter().find(|b| u < b.1).map_or(self.beds[0].0, |b| b.0)
    }

    /// The rock at rock coordinate `zeta` under a point whose structure is `s`.
    pub fn rock(&self, s: &Structure, zeta: f64) -> Rock {
        if zeta < s.pluton { return Rock::Granite; }
        if zeta < s.basement { return if s.schist > 0. { Rock::Schist } else { Rock::Gneiss }; }
        let rock = self.bed(zeta + s.shift, s.beds);
        let metamorphosed = s.grade * (1. - ((zeta - s.basement) / METAMORPHIC_DEPTH).clamp(0., 1.)) > 0.5;
        match rock {
            Rock::Sandstone if metamorphosed => Rock::Quartzite,
            Rock::Shale if metamorphosed => Rock::Schist,
            r => r,
        }
    }
}

/// Two unit vectors perpendicular to `d` and to each other.
pub fn tangents(d: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if d.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
    let t1 = d.cross(&a).normalize();
    (t1, d.cross(&t1))
}

/// Which way mountain belts are shortened, over the whole planet: the smoothed structure tensor
/// of the belts' strength (see the module docs).
pub struct Trend {
    domain: Domain,
    /// The tensor's components xx, xy, xz, yy, yz, zz at the grid's corners.
    tensor: [Vec<f64>; 6],
    /// The way to the foreland (down the belts' strength) at the corners: the smoothed
    /// gradient, negated, over its 90th percentile.
    foreland: [Vec<f64>; 3],
}

impl Trend {
    /// Cells per cube face edge: about 100 km on the Earth, a fraction of a belt's width.
    pub const CELLS: usize = 96;
    /// Passes of smoothing the tensor over neighbouring cells.
    const SMOOTHING: usize = 3;

    /// The trend of the belts whose strength is `orogeny`, on a planet of `radius`.
    pub fn new(radius: f64, orogeny: impl Fn(&Vector3<f64>) -> f64 + Sync) -> Trend {
        let domain = Domain::sphere(Trend::CELLS, radius);
        let n = domain.len();
        let o: Vec<f64> = domain.dirs.par_iter().map(&orogeny).collect();
        // Per cell: the tensor's six components, then the gradient's three.
        let mut t: Vec<[f64; 9]> = (0..n).into_par_iter().map(|i| {
            let d = domain.dirs[i];
            let links = domain.edges(i);
            let mut g = Vector3::zeros();
            for l in links {
                let to = domain.dirs[l.to] - d;
                let along = (to - d * d.dot(&to)).normalize();
                g += along * (o[l.to] - o[i]) / l.dist;
            }
            if !links.is_empty() { g *= 2. / links.len() as f64; }
            [g.x * g.x, g.x * g.y, g.x * g.z, g.y * g.y, g.y * g.z, g.z * g.z, g.x, g.y, g.z]
        }).collect();
        for _ in 0..Trend::SMOOTHING {
            t = (0..n).into_par_iter().map(|i| {
                let links = domain.links(i);
                let mut s = t[i];
                for l in links { for k in 0..9 { s[k] += t[l.to][k]; } }
                s.map(|x| x / (1 + links.len()) as f64)
            }).collect();
        }
        let mut sizes: Vec<f64> = t.iter().map(|c| (c[6] * c[6] + c[7] * c[7] + c[8] * c[8]).sqrt()).collect();
        sizes.sort_by(|a, b| a.total_cmp(b));
        let reference = sizes[sizes.len() * 9 / 10].max(1e-30);
        let corners = |k: usize, scale: f64| -> Vec<f64> {
            domain.corner_cells.iter().map(|cells| cells.iter().map(|&c| t[c][k]).sum::<f64>() * scale / cells.len() as f64).collect()
        };
        let tensor = std::array::from_fn(|k| corners(k, 1.));
        let foreland = std::array::from_fn(|k| corners(6 + k, -1. / reference));
        Trend { domain, tensor, foreland }
    }

    /// The direction of shortening at `d`: a unit vector tangent to the sphere, across the
    /// belt (its sign means nothing).
    pub fn shortening(&self, d: &Vector3<f64>) -> Vector3<f64> {
        let c: [f64; 6] = std::array::from_fn(|k| crate::terrain::scenery::sample(&self.domain, &self.tensor[k], d));
        let m = Matrix3::new(c[0], c[1], c[2], c[1], c[3], c[4], c[2], c[4], c[5]);
        let (e1, e2) = tangents(d);
        let (a, b, cc) = (e1.dot(&(m * e1)), e1.dot(&(m * e2)), e2.dot(&(m * e2)));
        let theta = 0.5 * (2. * b).atan2(a - cc);
        e1 * theta.cos() + e2 * theta.sin()
    }

    /// The way to the foreland at `d`: tangent to the sphere, down the belts' strength, about
    /// 1 long on a belt's flanks and shrinking to nothing at its centre (and away from belts).
    pub fn foreland(&self, d: &Vector3<f64>) -> Vector3<f64> {
        let v = Vector3::from_fn(|k, _| crate::terrain::scenery::sample(&self.domain, &self.foreland[k], d));
        v - d * d.dot(&v)
    }
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn unit(h: u64) -> f64 { (h >> 11) as f64 / (1u64 << 53) as f64 }

/// Gabor kernels per lattice cell of the phasor noise.
const KERNELS: u64 = 4;
/// The kernels' Gaussian radius (standard deviation), in wavelengths.
const KERNEL_SIGMA: f64 = 1.1;
/// The lattice's spacing, in wavelengths: the kernels are cut off at this distance, so only
/// the neighbouring cells hold any that reach a point.
const LATTICE: f64 = 3. * KERNEL_SIGMA;

/// Phasor noise at point `p` (metres from the planet's centre): waves of `wavelength` running
/// along `along` (a unit vector). Returns their phase, radians, and how well defined it is,
/// from 0 to 1.
///
/// Where the kernels' sum vanishes the phase is undefined, and any wave drawn from it (as
/// `sin(phase)`) jumps at that point: these are where fold trains end or fork. The second value
/// fades the waves out there: it is the sum's magnitude relative to that of as many kernels
/// with random phases (the root of the sum of their squared weights), through a smoothstep
/// from 0 to 0.6, so a wave scaled by it is continuous.
///
/// The direction `along` is a line, not an arrow (the trend's sign means nothing, and flips
/// where its angle wraps), so the waves mustn't depend on its sign. Each kernel has a random
/// real amplitude rather than a random phase: then the sum for -`along` is the complex
/// conjugate of that for `along`, the phase is negated, and an even profile of it (`cos`, as in
/// `fold_shape`) is the same either way.
pub fn phasor(seed: u64, p: &Vector3<f64>, along: &Vector3<f64>, wavelength: f64) -> (f64, f64) {
    let spacing = LATTICE * wavelength;
    let sigma = KERNEL_SIGMA * wavelength;
    let k = along * (TAU / wavelength);
    let cell = (p / spacing).map(|x| x.floor() as i64);
    let (mut re, mut im, mut power) = (0., 0., 0.);
    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let c = cell + Vector3::new(dx, dy, dz);
                let mut h = mix(seed ^ mix((c.x as u64).wrapping_mul(0x8da6_b343) ^ mix((c.y as u64).wrapping_mul(0xd816_3841) ^ mix(c.z as u64))));
                for _ in 0..KERNELS {
                    let mut r = || { h = mix(h); unit(h) };
                    let x = Vector3::new(c.x as f64 + r(), c.y as f64 + r(), c.z as f64 + r()) * spacing;
                    // A random real amplitude, rather than the paper's random phase, so that the
                    // sum for -k is the conjugate of that for k (see the doc comment).
                    let amplitude = (r() * TAU).cos();
                    let off = p - x;
                    let r2 = off.norm_squared();
                    if r2 >= spacing * spacing { continue; }
                    // A Gaussian windowed to reach zero, with zero slope, at the cutoff: cut
                    // off abruptly, the kernels' edges were steps in the folds.
                    let window = 1. - r2 / (spacing * spacing);
                    let w = amplitude * (-0.5 * r2 / (sigma * sigma)).exp() * window * window;
                    let a = k.dot(&off);
                    re += w * a.cos();
                    im += w * a.sin();
                    power += w * w;
                }
            }
        }
    }
    if power <= 0. { return (0., 0.); }
    let m = (re.hypot(im) / power.sqrt() / 0.6).min(1.);
    (im.atan2(re), m * m * (3. - 2. * m))
}

/// The folds' shape at `p`, from -1 in a trough to 1 at a crest, faded where the phase is
/// undefined: phasor noise's waves along `along` (whose sign means nothing), as the cosine of
/// the phase ψ, which doesn't depend on it.
///
/// `lean` (in (-1, 1)) makes them asymmetric: the phase is warped to ψ + lean (1 - cos ψ), so
/// a crest's limb on the side `along` points to is shorter and steeper, by a factor of up to
/// 1 + lean, for positive `lean`. The phase increases along `along` and is negated with it, so
/// with `lean` given the sign of `along`'s component towards the foreland (as `Planet` does),
/// the folds are the same whichever sign `along` has.
pub fn fold_shape(seed: u64, p: &Vector3<f64>, along: &Vector3<f64>, wavelength: f64, lean: f64) -> f64 {
    fold(seed, p, along, wavelength, lean).shape
}

/// The folds at `p` as `fold_shape`, and the offset of the strata by thrusts on their
/// forelimbs, from -1/2 to 1/2 of the thrusts' throw.
///
/// Where folds lean, their steep forelimbs break along thrusts, and each thrust sheet rides up
/// over the next towards the foreland (an imbricate stack, as in the Canadian Rockies' Front
/// Ranges or the Appalachians' Valley and Ridge). Each sheet's strata are raised more towards
/// its leading edge, and drop by the throw at the thrust in the middle of the forelimb (the
/// warped phase π/2 on the leaning side): a sawtooth in the phase, so the same beds crop out
/// again behind each thrust. The throw fades with the lean, so the belts' upright centres
/// have none, and where the phase is undefined, so thrusts die out where folds end. Like the
/// folds, the offset doesn't depend on the sign of `along`.
pub fn fold(seed: u64, p: &Vector3<f64>, along: &Vector3<f64>, wavelength: f64, lean: f64) -> Fold {
    let (phase, defined) = phasor(seed, p, along, wavelength);
    let lean = lean.clamp(-0.95, 0.95);
    let warped = phase + lean * (1. - phase.cos());
    let ahead = lean.signum() * warped - std::f64::consts::FRAC_PI_2;
    let sawtooth = ahead.rem_euclid(TAU) / TAU - 0.5;
    let leaning = (3. * lean.abs()).min(1.);
    // Fractured rock: open joints across an anticline's crest, where its outer arc is
    // stretched, and the damage zone either side of a thrust.
    let crest = warped.cos().max(0.).powi(2);
    let off = ahead.rem_euclid(TAU);
    let zone = leaning * (-(off.min(TAU - off) / 0.35).powi(2)).exp();
    Fold { shape: defined * warped.cos(), thrust: defined * leaning * sawtooth, fracture: defined * crest.max(zone) }
}

/// The folds at a point (`fold`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fold {
    /// From -1 in a trough to 1 at a crest.
    pub shape: f64,
    /// The thrusts' offset of the strata, from -1/2 to 1/2 of their throw.
    pub thrust: f64,
    /// How fractured the rock is by the folding and faulting, 0 to 1: across anticline crests
    /// and in thrusts' damage zones.
    pub fracture: f64,
}

/// How much more erodible bedrock fractured to `fracture` (0 to 1) is to `process`, for a
/// `fracturing` of 1 at the most fractured: glaciers quarry the loosened blocks, so the
/// fractures count fully for them, and rivers pluck them too but mostly abrade, so a share.
pub fn fractured(fracture: f64, process: Process, fracturing: f64) -> f64 {
    let share = if process == Process::Glaciers { 1. } else { RIVERS_SHARE_OF_FRACTURING };
    1. + fracturing.max(0.) * share * fracture.clamp(0., 1.)
}

/// The share of fracturing's effect on glaciers that rivers feel.
const RIVERS_SHARE_OF_FRACTURING: f64 = 0.35;

/// How much of folds of `wavelength` a grid resolving detail of wavelength `finest` keeps: all
/// of them a few cells wide, none narrower than about two cells.
pub fn fade(finest: f64, wavelength: f64) -> f64 {
    let t = ((finest / wavelength - 0.25) / 0.75).clamp(0., 1.);
    1. - t * t * (3. - 2. * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glaciers_and_rivers_rank_rocks_differently() {
        let k = |r: Rock, p: Process| r.erodibility(p, 5.);
        // Rivers: by strength.
        assert!(k(Rock::Shale, Process::Rivers) > k(Rock::Sandstone, Process::Rivers));
        assert!(k(Rock::Sandstone, Process::Rivers) > k(Rock::Granite, Process::Rivers));
        assert!(k(Rock::Granite, Process::Rivers) > k(Rock::Quartzite, Process::Rivers));
        // Glaciers quarry jointed rock: quartzite, which rivers can hardly cut, goes faster than
        // massive granite, which rivers cut faster.
        assert!(k(Rock::Quartzite, Process::Glaciers) > k(Rock::Granite, Process::Glaciers));
        assert!(k(Rock::Granite, Process::Glaciers) < k(Rock::Granite, Process::Rivers));
        assert!(k(Rock::Quartzite, Process::Glaciers) > 1.5 * k(Rock::Quartzite, Process::Rivers));
        // The contrast is that between the softest and hardest bedrock.
        let bedrock = [Rock::Shale, Rock::Sandstone, Rock::Limestone, Rock::Quartzite, Rock::Schist, Rock::Gneiss, Rock::Granite];
        for p in [Process::Rivers, Process::Glaciers] {
            let ks: Vec<f64> = bedrock.iter().map(|&r| k(r, p)).collect();
            let ratio = ks.iter().cloned().fold(0., f64::max) / ks.iter().cloned().fold(f64::MAX, f64::min);
            assert!(ratio <= 5. + 1e-9 && ratio > 3., "{:?}: {}", p, ratio);
        }
        for r in Rock::ALL { assert_eq!(Rock::from_index(r.index()), r); }
    }

    #[test]
    fn test_column_alternates_hard_and_soft_and_repeats() {
        let c = Column::new(&GeologyParams::default(), 3);
        let rocks: Vec<Rock> = (0..200).map(|k| c.bed(k as f64 * 25., 1.)).collect();
        assert!(rocks.contains(&Rock::Shale));
        assert!(rocks.iter().any(|&r| r == Rock::Sandstone || r == Rock::Limestone));
        assert_eq!(c.bed(123., 1.), c.bed(123. + c.period, 1.));
        assert_eq!(c.bed(-77., 1.), c.bed(-77. + 3. * c.period, 1.));
        // Below the basement's top, crystalline rock; granite under a pluton's roof.
        let s = Structure { basement: -2000., pluton: -1e5, beds: 1., schist: -1., ..Structure::default() };
        assert_eq!(c.rock(&s, -2500.), Rock::Gneiss);
        assert_eq!(c.rock(&Structure { pluton: 0., ..s }, -10.), Rock::Granite);
        // Deep in a strong belt the cover is metamorphosed.
        let deep = Structure { grade: 1., ..s };
        let found: Vec<Rock> = (0..40).map(|k| c.rock(&deep, -1990. + k as f64 * 20.)).collect();
        assert!(found.iter().all(|&r| r != Rock::Sandstone && r != Rock::Shale), "{:?}", found);
    }

    #[test]
    fn test_phasor_waves_run_along_the_trend() {
        // Along the waves' direction the phase advances by 2π a wavelength; across them it
        // hardly changes.
        let along = Vector3::new(0., 0., 1.);
        let base = Vector3::new(6.371e6, 0., 0.);
        let wavelength = 8000.;
        let wrapped = |a: f64, b: f64| { let d = (a - b).rem_euclid(TAU); d.min(TAU - d) };
        let (mut across_change, mut along_change) = (0., 0.);
        let steps = 400;
        for i in 0..steps {
            let p = base + Vector3::new(0., i as f64 * 1000., 0.);
            let q = p + along * (wavelength / 8.);
            let r = p + Vector3::new(0., 1., 0.) * (wavelength / 8.);
            let (a, _) = phasor(1, &p, &along, wavelength);
            along_change += wrapped(phasor(1, &q, &along, wavelength).0, a);
            across_change += wrapped(phasor(1, &r, &along, wavelength).0, a);
        }
        let (along_change, across_change) = (along_change / steps as f64, across_change / steps as f64);
        // An eighth of a wavelength is π/4 of phase.
        assert!((along_change - TAU / 8.).abs() < 0.2, "{}", along_change);
        assert!(across_change < 0.4 * along_change, "{} vs {}", across_change, along_change);
    }

    #[test]
    fn test_phasor_waves_are_continuous() {
        // Even through the points where the phase is undefined, the faded wave never jumps:
        // over steps of a thousandth of a wavelength it changes by little more than a smooth
        // sinusoid would.
        let along = Vector3::new(0.6, 0.8, 0.);
        let wavelength = 1000.;
        let wave = |p: &Vector3<f64>| fold_shape(9, p, &along, wavelength, 0.6);
        let step = wavelength / 1000.;
        let mut worst: f64 = 0.;
        for row in 0..40 {
            let mut last = None;
            for i in 0..20000 {
                let p = Vector3::new(6.371e6, row as f64 * 137., 0.) + Vector3::new(0., 0., 1.) * (i as f64 * step);
                let w = wave(&p);
                if let Some(l) = last { worst = worst.max((w - l as f64).abs()); }
                last = Some(w);
            }
        }
        // A sinusoid changes at most 2π/1000 a step; allow for the fading and for crowded
        // phase near the singular points.
        assert!(worst < 0.1, "{}", worst);
    }

    #[test]
    fn test_folds_have_no_detail_finer_than_their_wavelength() {
        // A sinusoid's curvature is at most k² = (2π / wavelength)²; the folds' exceeds a few
        // times that only near the points where their phase is undefined.
        let along = Vector3::new(0., 0.8, 0.6);
        let wavelength = 8000.;
        let wave = |p: &Vector3<f64>| fold_shape(3, p, &along, wavelength, 0.);
        let (h, dir) = (20., Vector3::new(0., 0.6, -0.8));
        let mut curvature: Vec<f64> = (0..60).flat_map(|row| (0..600).map(move |i| (row, i))).map(|(row, i)| {
            let p = Vector3::new(6.371e6, row as f64 * 1371., 0.) + dir * (i as f64 * 61.);
            ((wave(&(p + dir * h)) - 2. * wave(&p) + wave(&(p - dir * h))) / (h * h)).abs()
        }).collect();
        curvature.sort_by(|a, b| a.total_cmp(b));
        let k2 = (TAU / wavelength).powi(2);
        let p99 = curvature[curvature.len() * 99 / 100] / k2;
        assert!(p99 < 3., "{}", p99);
    }

    #[test]
    fn test_folds_dont_depend_on_the_sign_of_their_direction() {
        // The trend is a line, not an arrow: `Trend::shortening` may give either sign, and
        // flips it along lines where its angle wraps. The folds must be the same either way.
        let mut worst: f64 = 0.;
        for i in 0..200 {
            let p = Vector3::new(6.371e6, i as f64 * 517., i as f64 * 211.);
            let along = Vector3::new(0., (i as f64 * 0.3).cos(), (i as f64 * 0.3).sin());
            // Leaning one way, given as the direction's component towards the foreland.
            let lean = 0.5 * (i as f64 * 0.7).sin();
            worst = worst.max((fold_shape(4, &p, &along, 8000., lean) - fold_shape(4, &p, &-along, 8000., -lean)).abs());
        }
        assert!(worst < 1e-9, "{}", worst);
    }

    #[test]
    fn test_leaning_folds_are_steeper_towards_the_foreland() {
        // Along the waves, the shape falls from crest to trough over less of the way than it
        // rises: a fraction of about 0.35 for a lean of 0.6.
        let along = Vector3::new(0., 0., 1.);
        let falling = |lean: f64| {
            let (mut falls, mut total) = (0, 0);
            for row in 0..40 {
                let mut last = None;
                for i in 0..2000 {
                    let p = Vector3::new(6.371e6, row as f64 * 3000., 0.) + along * (i as f64 * 40.);
                    let w = fold_shape(2, &p, &along, 8000., lean);
                    if let Some(l) = last { total += 1; if w < l { falls += 1; } }
                    last = Some(w);
                }
            }
            falls as f64 / total as f64
        };
        let (forward, upright, back) = (falling(0.6), falling(0.), falling(-0.6));
        assert!((upright - 0.5).abs() < 0.05, "{}", upright);
        assert!(forward < 0.42 && back > 0.58, "{} {}", forward, back);
    }

    #[test]
    fn test_thrusts_repeat_the_beds_towards_the_foreland() {
        // Walking towards the foreland along leaning folds, the strata's offset climbs slowly
        // and drops at a thrust about once a wavelength; never the other way.
        let along = Vector3::new(0., 0., 1.);
        let (mut drops, mut rises) = (0, 0);
        for row in 0..20 {
            let mut last: Option<f64> = None;
            for i in 0..4000 {
                let p = Vector3::new(6.371e6, row as f64 * 5000., 0.) + along * (i as f64 * 20.);
                let t = fold(5, &p, &along, 8000., 0.6).thrust;
                if let Some(l) = last {
                    if t - l < -0.3 { drops += 1; }
                    if t - l > 0.3 { rises += 1; }
                }
                last = Some(t);
            }
        }
        // 20 rows of 80 km: about 200 wavelengths, some where the phase is undefined.
        assert!(drops > 100 && rises < drops / 10, "{} drops, {} rises", drops, rises);
        // The same whichever way the direction points, and no thrusts in upright folds.
        let p = Vector3::new(6.371e6, 1234., 5678.);
        assert!((fold(5, &p, &along, 8000., 0.6).thrust - fold(5, &p, &-along, 8000., -0.6).thrust).abs() < 1e-9);
        assert_eq!(fold(5, &p, &along, 8000., 0.).thrust, 0.);
    }

    #[test]
    fn test_crests_and_thrusts_are_fractured() {
        // Along leaning folds, the rock is most fractured at anticline crests and at thrusts,
        // and least in syncline troughs.
        let along = Vector3::new(0., 0., 1.);
        let (mut crests, mut troughs) = (vec![], vec![]);
        for i in 0..20000 {
            let p = Vector3::new(6.371e6, 0., 0.) + along * (i as f64 * 20.);
            let f = fold(7, &p, &along, 8000., 0.6);
            if f.shape > 0.9 { crests.push(f.fracture); }
            if f.shape < -0.9 { troughs.push(f.fracture); }
        }
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        assert!(mean(&crests) > 0.7 && mean(&troughs) < 0.2, "{} {}", mean(&crests), mean(&troughs));
        // Glaciers make more of it than rivers do.
        assert!(fractured(1., Process::Glaciers, 1.5) > 2. && fractured(1., Process::Rivers, 1.5) < 1.6);
        assert_eq!(fractured(0., Process::Glaciers, 1.5), 1.);
    }

    #[test]
    fn test_trend_runs_across_a_belt() {
        // A belt along the equator: shortening is north-south.
        let t = Trend::new(6.371e6, |d| (-(d.y / 0.1).powi(2)).exp());
        for d in [Vector3::new(1., 0.05, 0.), Vector3::new(0., 0., 1.), Vector3::new(-0.7, -0.08, 0.7)] {
            let d = d.normalize();
            let s = t.shortening(&d);
            let north = (Vector3::y() - d * d.y).normalize();
            assert!(s.dot(&north).abs() > 0.95, "{:?} {:?}", d, s);
            assert!(s.dot(&d).abs() < 1e-9);
        }
        // The foreland is away from the belt's centre on either side, and nowhere at it.
        let north = Vector3::new(1., 0.15, 0.).normalize();
        assert!(t.foreland(&north).y > 0.3, "{:?}", t.foreland(&north));
        let south = Vector3::new(0., -0.15, 1.).normalize();
        assert!(t.foreland(&south).y < -0.3, "{:?}", t.foreland(&south));
        assert!(t.foreland(&Vector3::new(1., 0., 0.)).norm() < 0.2);
    }
}

