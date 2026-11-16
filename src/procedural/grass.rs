//! # Procedural grass
//!
//! A field of individual grass blades growing on a heightmap (see `heightmap`), so it can cover
//! any terrain. A field mixes one or more *species*, which say what the blades look like, and is
//! shaped by *conditions*: wind, drought, frost, rain, mowing, trampling and where grass can
//! grow at all.
//!
//! ```json
//! { "type": "grass", "heightmap": "$ground", "density": 800, "seed": 1,
//!   "min": [-10, -10], "max": [10, 10],
//!   "species": ["meadow", { "preset": "tall", "weight": 0.3, "height": 1.2 }],
//!   "conditions": { "preset": "summer", "wind": { "direction": [1, 0.3], "strength": 0.6 } } }
//! ```
//!
//! - `heightmap`: as for `terrain`; use the same one so the grass sits on the ground
//! - `density`: blades per square metre (default 600). Short grass needs many to cover the
//!   ground: a lawn looks like turf from about 5000
//! - `min`, `max`: [x, z] corners of the area to grow in (default: the whole heightmap)
//! - `seed`: the same seed gives the same field
//! - `material`: render every blade with this scene material instead (eg. for debugging)
//!
//! ## Species
//!
//! Each entry of `species` is a preset name, or an object with a `preset` (default `meadow`)
//! and fields overriding it. Presets: `lawn`, `meadow`, `fescue`, `tall`, `tussock`,
//! `savanna`, `reed`. Fields:
//!
//! - `weight`: relative abundance (default 1)
//! - `height` (m), `height_variation` (fraction either way)
//! - `width` (m, at the base); `tip`: how sharply the width tapers to the tip - 1 gives a
//!   triangle, larger numbers keep the sides parallel for longer
//! - `stiffness` (0 to 1): how well the blade resists drooping and wind
//! - `lean`: largest random lean from vertical, degrees; `twist`: largest twist along the blade,
//!   degrees
//! - `clump_size`: blades per tuft; `clump_radius` (m); `clump_spread` (0 to 1): how much the
//!   blades of a tuft fan outward
//! - `base_color`, `tip_color`: the living blade, from base to tip; `dry_color`: dead
//! - `dryness` (0 to 1): how dry this species is regardless of conditions
//! - `translucency` (0 to 1): how much light passes through a blade; `sheen` (0 to 1): waxy
//!   highlights
//! - `segments`: straight pieces each blade is built from (1 to 8)
//! - `normal_alignment` (0 to 1): 0 grows straight up, 1 perpendicular to the slope
//! - `color_variation` (0 to 2.2): how much blades differ in colour (default 1)
//!
//! ### Wild grass, wildflowers and clover
//!
//! A field can mix wild plants in with its grass, and the grasses can flower. Presets:
//!
//! - `rough`: uncut grass, olive and yellowing, uneven, with a few flowering stems
//! - `timothy`, `wild_oat`, `bent`: grasses with seed heads - a dense spike, big dangling
//!   spikelets on a nodding panicle, and an airy reddish haze of tiny ones
//! - `daisy`, `oxeye` (the tall meadow daisy), `dandelion` (with seed clocks), `buttercup`
//! - `clover` (white), `red_clover`
//! - `heather`: low, wiry shrubs with purple flower spikes; `bracken`: a fern
//!
//! A grass flowers with a `seed_head` (null for none):
//!
//! - `style`: `spike` (spikelets packed round the stem, like timothy) or `panicle` (spikelets
//!   hanging from branches)
//! - `fraction`: of the blades of a tuft, how many are flowering stems; `stem_height` (times
//!   the blade height) and `stem_width` (m)
//! - `length` (m) of the head, and for a spike its `width` (m)
//! - `spikelets`: how many; `spikelet_length`, `spikelet_width` (m)
//! - for a panicle, `branch_length` (m) and `droop` (0 to 1): how much the spikelets hang
//! - `color`, `tip_color`, `dry_color`: spikelets ripen to `dry_color` as the grass dries
//!
//! Flowering stems that a mower would cut are left as stalks.
//!
//! `"kind": "flower"` makes a flowering plant: a rosette of leaves, and stems with a flower or
//! a seed clock on each. The blade fields describe its stems (`height`, `width`, `stiffness`,
//! `lean`, `base_color` and `tip_color`; `clump_radius` spreads them), and under a mower they
//! flower short, like lawn daisies. Its `flower` (defaults: a daisy):
//!
//! - `count`: stems per plant, on average
//! - `petals`, in `layers`, each layer in tilted up by `layer_cup` (degrees) more than the
//!   outer one, which is tilted `cup` (degrees) from the flower's face
//! - `petal_length`, `petal_width` (m); `petal_shape`: where along it a petal is widest (0 to
//!   1); `curl`: how far the tips turn up (or, negative, droop), as a fraction of the length
//! - `petal_color` at the base, `petal_tip_color`, `sheen`
//! - `centre_radius` (m), `dome` (its height, as a fraction of the radius), `centre_color`
//! - `clock_fraction`: of the stems, how many have gone to seed, with a clock of
//!   `clock_radius` (m) made of `pappus` seeds, in `clock_color`, on a stem `clock_height`
//!   times as tall
//!
//! and its `leaves`: `count`, `length`, `width` (m), `shape` (as `petal_shape`), `angle`
//! (degrees above the ground), `color` and `tip_color`.
//!
//! `"kind": "clover"` makes a clover plant: leaves of three leaflets on stalks spreading over
//! `clump_radius`, the blade fields describing the stalks. Its `leaves`: `count` per plant,
//! leaflet `length`, `width` and `shape` (m, and as `petal_shape`), `angle`, `color`,
//! `tip_color` and the `band_color` of the pale chevron across them. Its `flower`:
//! `fraction` of the stalks that bear a round head of `florets`, of `radius` (m), on stalks
//! `height` times as tall, in `color` and `tip_color`; the lowest `wilted` fraction of the
//! head has withered to `old_color`.
//!
//! `"kind": "fern"` makes a fern: fronds on stems that rise and arch over, holding the leafy
//! part nearly level. Of the blade fields, `height`, `height_variation`, `width` and the
//! colours describe the stems, and `clump_radius` spreads them. Its `frond` (defaults: bracken): `count` per plant,
//! on average; `blade_start`, how far along the stem the leafy part starts (0 to 1); `pinnae`,
//! the pairs of leaflets along it, the first `pinna_length` and `pinna_width` (m), shortening
//! to the tip; their `sweep` towards the tip (degrees) and `droop` (a fraction of their
//! length). Fern pinnae dry to the species' `dry_color`, rust for bracken.
//!
//! In `density`, a flowering plant, clover or fern counts as one blade, and `weight` sets how many of
//! the field's tufts are of the species.
//!
//! ## Conditions
//!
//! `conditions` is a preset name, or an object with a `preset` (default `summer`) and fields
//! overriding it. Presets: `lush`, `summer`, `drought`, `autumn`, `after_rain`, `frost`,
//! `windy`, `manicured`, `trampled`, `wild` (left to grow, with dead stems among the new, and
//! plants in drifts). Fields:
//!
//! - `wind`: `{ "direction": [x, z], "strength": 0 to 1, "gustiness": 0 to 1, "scale": m }`.
//!   Blades bend downwind, more where gusts (noise of the given scale) are stronger
//! - `dryness` (0 to 1): turns blades from green toward straw; `dead_fraction`: blades that
//!   are dead outright, which are straw coloured and droop
//! - `wetness` (0 to 1): darker blades with a glossy coat
//! - `frost` (0 to 1): pale, icy blade tips
//! - `patchiness` (0 to 1): bare and thin patches, of size `patch_scale` (m); grass at their
//!   edges is shorter and drier
//! - `mow_height` (m): blades are cut flat at this height. `mow_stripes`: `{ "width": m,
//!   "direction": [x, z] }` - alternate stripes lean opposite ways, as after a mower
//! - `trampled` (0 to 1): blades pressed over and shortened
//! - `max_slope` (degrees): no grass on slopes steeper than this
//! - `min_altitude`, `max_altitude` (m), `altitude_fade` (m): grow only between these heights,
//!   eg. above a water line or below a tree line
//! - `species_zones` (m): with several species, group them into patches of about this size
//!   rather than mixing them evenly; 0 to mix evenly
//! - `litter`: dead blades lying on the ground, per square metre - the thatch that covers the
//!   soil under long grass (default 0; `wild` has some). It thins, but doesn't vanish, in bare
//!   patches
//!
//! ## Geometry
//!
//! A blade is a ribbon along a quadratic Bézier curve, its control points found as Klemens
//! Jahrmann and Michael Wimmer do in "Responsive Real-Time Grass Rendering for General 3D
//! Scenes", I3D 2017, doi:10.1145/3023368.3023380: the tip is pushed by lean, droop and wind,
//! kept above the ground, and the middle control point is then placed so the curve keeps the
//! blade's length. The ribbon is cut into a few straight segments, each two triangles, and
//! tapers to the tip.
//!
//! Blades are generated once, when the scene is loaded, deterministically from the seed and
//! position (so a field is the same however many threads build it), and bucketed into a grid of
//! cells in the xz plane by the cells their bounding boxes overlap. A ray walks the cells in
//! order (`heightmap::walk_grid`), skipping cells it passes above, and testing the blades in the
//! others; the first cell with a hit inside it has the nearest hit.
//!
//! Flowers, clover and seed heads are built of the same ribbons, with other outlines: petals
//! and leaflets narrow at the base and rounded at the tip, the wedges of a flower's disc, and
//! the flared stalks of a dandelion clock's seeds. Petals, spikelets and seeds are spread
//! around their heads by the golden angle, or over a sphere by the Fibonacci lattice.
//!
//! Each kind of part of each species - blade, stem, petal, leaflet - has its own *look*: its
//! outline and colours. A part's colour depends on its look, how dry it is and how far along
//! it the hit is. The medium finds which part a point is on, and returns a material from a
//! palette built for the field.

use crate::color::Color;
use crate::geometry::Pcg32;
use crate::intersection::RawIntersection;
use crate::material::grass::GrassMaterial;
use crate::material::model::MaterialModel;
use crate::material::texture::Medium;
use crate::na::Vector3;
use crate::noise::PerlinNoise;
use crate::procedural::heightmap::{HeightGrid, Grid2, walk_grid, ray_spans, intersect_triangle};
use crate::ray::Ray;
use crate::scenefile::SceneFile;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use rayon::prelude::*;
use serde_json::{json, Value, Map};
use std::f64::consts::PI;
use std::sync::Arc;

const MAX_SEGMENTS: usize = 8;
/// Palette levels: of dryness, of random per-blade variation, and along the blade.
const DRY_LEVELS: usize = 8;
const JITTER_LEVELS: usize = 4;
const HEIGHT_LEVELS: usize = 8;
/// Size of the tiles blades are generated in, each with its own random sequence.
const TILE: f64 = 1.;
/// Blades per traversal cell to aim for.
const BLADES_PER_CELL: f64 = 16.;
/// The golden angle, to spread petals, spikelets and seeds evenly around a head.
const GOLDEN_ANGLE: f64 = 2.399_963_229_728_653;

/// The outline of a ribbon: its width along it, as a fraction of its widest.
#[derive(Clone, Copy, Debug)]
enum Outline {
    /// A grass blade, tapering to its tip: the larger the number, the longer its sides stay
    /// parallel.
    Blade(f64),
    /// A stem, its sides almost parallel.
    Stem,
    /// A petal or leaflet: narrow at its base, widest this fraction of the way along it, and
    /// rounded at the tip.
    Petal(f64),
    /// From a point to its full width: a sector of a disc.
    Wedge,
    /// A thin stalk flaring at its end, like a dandelion seed's parachute.
    Plume,
}

impl Outline {
    fn width(&self, s: f64) -> f64 {
        match *self {
            Outline::Blade(tip) => 1. - s.powf(tip),
            Outline::Stem => 1. - 0.4 * s,
            Outline::Petal(widest) => {
                let x = if s < widest { 0.5 * s / widest } else { 0.5 + 0.5 * (s - widest) / (1. - widest) };
                0.8 * (PI * x).sin().max(0.).powf(0.6) + 0.2 * (1. - s)
            },
            Outline::Wedge => s,
            Outline::Plume => 0.08 + 0.92 * smoothstep(0.6, 1., s),
        }
    }

    /// Width once mown: grass blades cut square, anything else is unchanged.
    fn cut_width(&self, s: f64) -> f64 {
        match self {
            Outline::Blade(_) => 1. - 0.25 * s,
            _ => self.width(s),
        }
    }
}

/// What one kind of part of a plant - a blade, a stem, a petal - looks like: its outline and
/// its colours, from which the medium builds its palette.
#[derive(Clone, Debug)]
pub struct Look {
    segments: usize,
    /// Width at the end of each segment, for uncut and cut parts.
    tapers: [[f64; MAX_SEGMENTS + 1]; 2],
    base_color: Color,
    tip_color: Color,
    dry_color: Color,
    /// A band of another colour across the middle, like the pale chevron on a clover leaflet.
    band: Option<Color>,
    translucency: f64,
    sheen: f64,
    /// How much dryness browns it: 1 for grass, less for petals, 0 for what is already dead.
    dry_response: f64,
    /// Scale for the random variation from part to part.
    color_variation: f64,
    /// Whether it's a green leaf, whose chlorophyll yellows the light through it.
    leafy: bool,
}

impl Look {
    fn new(outline: Outline, segments: usize, colors: [Color; 3], translucency: f64, sheen: f64, dry_response: f64) -> Look {
        let segments = segments.clamp(1, MAX_SEGMENTS);
        let mut tapers = [[0.; MAX_SEGMENTS + 1]; 2];
        let [uncut, cut] = &mut tapers;
        for (i, (u, c)) in uncut.iter_mut().zip(cut.iter_mut()).enumerate().take(segments + 1) {
            let s = i as f64 / segments as f64;
            (*u, *c) = (outline.width(s), outline.cut_width(s));
        }
        Look {
            segments, tapers,
            base_color: colors[0], tip_color: colors[1], dry_color: colors[2],
            band: None, translucency, sheen, dry_response, color_variation: 1., leafy: true,
        }
    }
}

/// Flowering stems on a grass, with a seed head at the top.
#[derive(Clone, Debug)]
pub struct SeedHead {
    /// A panicle of branches with spikelets hanging from them (oats, bents), rather than a
    /// dense spike of spikelets around the stem (timothy).
    pub panicle: bool,
    /// Of the blades of a tuft, the fraction that are flowering stems.
    pub fraction: f64,
    /// The stem, grown like a blade.
    pub stem: Species,
    pub length: f64,
    pub width: f64,
    pub spikelets: usize,
    pub spikelet_length: f64,
    pub spikelet_width: f64,
    pub branch_length: f64,
    pub droop: f64,
}

/// A flowering plant: a rosette of leaves, and stems with a flower, or a seed clock, on top.
#[derive(Clone, Debug)]
pub struct Flower {
    /// Flowering stems per plant, on average.
    pub count: f64,
    pub stem: Species,
    /// The stem of a seed clock, which grows taller than a flower's.
    pub clock_stem: Species,
    pub petals: usize,
    pub layers: usize,
    pub petal_length: f64,
    pub petal_width: f64,
    /// Angle of the outer petals above the flower's face, and how much steeper each layer in.
    pub cup: f64,
    pub layer_cup: f64,
    /// How far the petal tips curl up (positive) or droop, as a fraction of their length.
    pub curl: f64,
    pub centre_radius: f64,
    pub dome: f64,
    pub clock_fraction: f64,
    pub clock_radius: f64,
    pub pappus: usize,
    pub leaves: usize,
    pub leaf_length: f64,
    pub leaf_width: f64,
    pub leaf_angle: f64,
}

/// Clover: leaves of three leaflets on stalks spreading from a plant, and round flower heads.
#[derive(Clone, Debug)]
pub struct Clover {
    pub leaves: usize,
    pub stem: Species,
    pub leaflet_length: f64,
    pub leaflet_width: f64,
    pub leaf_angle: f64,
    pub flower_fraction: f64,
    pub flower_stem: Species,
    pub flower_radius: f64,
    pub florets: usize,
    /// The fraction of the head, from below, whose florets have withered and hang down.
    pub wilted: f64,
}

/// A fern: fronds on arching stems, each with pairs of pinnae along it, longest at the base.
#[derive(Clone, Debug)]
pub struct Fern {
    /// Fronds per plant, on average.
    pub fronds: f64,
    /// Where along the stem the leafy blade starts.
    pub blade_start: f64,
    pub pinnae: usize,
    pub pinna_length: f64,
    pub pinna_width: f64,
    /// How far the pinnae sweep towards the frond's tip, and how much they droop.
    pub sweep: f64,
    pub droop: f64,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Grass(Option<Box<SeedHead>>),
    Flower(Box<Flower>),
    Clover(Box<Clover>),
    Fern(Box<Fern>),
}

/// Parts of each kind of plant: an offset into the species' looks.
const BLADE: usize = 0;
const STEM: usize = 1;
const SPIKELET: usize = 2;
const FLOWER_STEM: usize = 0;
const LEAF: usize = 1;
const PETAL: usize = 2;
const DISC: usize = 3;
const PAPPUS: usize = 4;
const PETIOLE: usize = 0;
const LEAFLET: usize = 1;
const FLORET: usize = 2;
const WILTED: usize = 3;
const FROND_STEM: usize = 0;
const PINNA: usize = 1;

#[derive(Clone, Debug)]
pub struct Species {
    pub weight: f64,
    pub height: f64,
    pub height_variation: f64,
    pub width: f64,
    pub tip: f64,
    pub stiffness: f64,
    pub lean: f64,
    pub twist: f64,
    pub segments: usize,
    pub clump_size: usize,
    pub clump_radius: f64,
    pub clump_spread: f64,
    pub base_color: Color,
    pub tip_color: Color,
    pub dry_color: Color,
    pub dryness: f64,
    pub translucency: f64,
    pub sheen: f64,
    pub normal_alignment: f64,
    pub color_variation: f64,
    pub kind: Kind,
    looks: Vec<Look>,
}

#[derive(Clone, Debug)]
pub struct Conditions {
    pub wind: Vector3<f64>,
    pub gustiness: f64,
    pub gust_scale: f64,
    pub dryness: f64,
    pub dead_fraction: f64,
    pub wetness: f64,
    pub frost: f64,
    pub patchiness: f64,
    pub patch_scale: f64,
    pub mow_height: Option<f64>,
    pub stripe_width: f64,
    pub stripe_direction: Vector3<f64>,
    pub trampled: f64,
    pub max_slope: f64,
    pub min_altitude: f64,
    pub max_altitude: f64,
    pub altitude_fade: f64,
    pub species_zones: f64,
    pub litter: f64,
}

fn species_preset(name: &str) -> Value {
    let lawn = json!({
        "weight": 1, "height": 0.07, "height_variation": 0.25, "width": 0.004, "tip": 4,
        "stiffness": 0.8, "lean": 15, "twist": 20, "segments": 3,
        "clump_size": 1, "clump_radius": 0.0, "clump_spread": 0.0,
        "base_color": [0.035, 0.12, 0.02], "tip_color": [0.09, 0.26, 0.04], "dry_color": [0.42, 0.34, 0.17],
        "dryness": 0, "translucency": 0.45, "sheen": 0.3, "normal_alignment": 0.2, "color_variation": 1
    });
    let overrides = match name {
        "lawn" => json!({}),
        "meadow" => json!({
            "height": 0.35, "height_variation": 0.4, "width": 0.005, "tip": 3, "stiffness": 0.5,
            "lean": 25, "twist": 45, "segments": 5, "clump_size": 3, "clump_radius": 0.03, "clump_spread": 0.4,
            "base_color": [0.045, 0.13, 0.025], "tip_color": [0.15, 0.3, 0.06]
        }),
        "fescue" => json!({
            "height": 0.2, "height_variation": 0.3, "width": 0.0015, "tip": 2, "stiffness": 0.4,
            "lean": 20, "twist": 60, "segments": 4, "clump_size": 12, "clump_radius": 0.025, "clump_spread": 0.7,
            "base_color": [0.05, 0.12, 0.05], "tip_color": [0.13, 0.24, 0.1]
        }),
        "tall" => json!({
            "height": 0.9, "height_variation": 0.3, "width": 0.008, "tip": 2.5, "stiffness": 0.35,
            "lean": 20, "twist": 60, "segments": 6, "clump_size": 5, "clump_radius": 0.05, "clump_spread": 0.5,
            "base_color": [0.06, 0.14, 0.03], "tip_color": [0.24, 0.32, 0.1]
        }),
        "tussock" => json!({
            "height": 0.45, "height_variation": 0.25, "width": 0.004, "tip": 2, "stiffness": 0.3,
            "lean": 10, "twist": 90, "segments": 6, "clump_size": 30, "clump_radius": 0.06, "clump_spread": 0.9,
            "base_color": [0.1, 0.15, 0.04], "tip_color": [0.33, 0.31, 0.15], "dryness": 0.3
        }),
        "savanna" => json!({
            "height": 0.6, "height_variation": 0.3, "width": 0.004, "tip": 2, "stiffness": 0.55,
            "lean": 20, "twist": 120, "segments": 5, "clump_size": 8, "clump_radius": 0.04, "clump_spread": 0.6,
            "base_color": [0.25, 0.22, 0.1], "tip_color": [0.5, 0.42, 0.22], "dry_color": [0.55, 0.45, 0.25],
            "dryness": 0.6, "translucency": 0.3, "sheen": 0.15
        }),
        "reed" => json!({
            "height": 1.6, "height_variation": 0.2, "width": 0.015, "tip": 2, "stiffness": 0.85,
            "lean": 6, "twist": 30, "segments": 6, "clump_size": 4, "clump_radius": 0.05, "clump_spread": 0.3,
            "base_color": [0.07, 0.15, 0.05], "tip_color": [0.2, 0.28, 0.09], "sheen": 0.5
        }),
        // Rough, uncut grass: olive and yellowing, uneven, and a few flowering stems.
        "rough" => json!({
            "height": 0.5, "height_variation": 0.45, "width": 0.004, "tip": 2.5, "stiffness": 0.4,
            "lean": 30, "twist": 70, "segments": 5, "clump_size": 6, "clump_radius": 0.05, "clump_spread": 0.6,
            "base_color": [0.06, 0.12, 0.03], "tip_color": [0.24, 0.29, 0.1], "dryness": 0.12,
            "color_variation": 2.2,
            "seed_head": {
                "style": "panicle", "fraction": 0.08, "length": 0.12, "spikelets": 18, "branch_length": 0.04,
                "spikelet_length": 0.006, "spikelet_width": 0.0025, "droop": 0.35,
                "color": [0.24, 0.2, 0.14], "tip_color": [0.36, 0.3, 0.2]
            }
        }),
        // Timothy (Phleum pratense): a dense cylindrical spike on a tall stem.
        "timothy" => json!({
            "height": 0.45, "height_variation": 0.35, "width": 0.006, "tip": 2.5, "stiffness": 0.45,
            "lean": 25, "twist": 50, "segments": 5, "clump_size": 6, "clump_radius": 0.04, "clump_spread": 0.5,
            "base_color": [0.05, 0.13, 0.03], "tip_color": [0.18, 0.28, 0.08], "color_variation": 1.5,
            "seed_head": { "style": "spike", "fraction": 0.3 }
        }),
        // Wild oat (Avena fatua): big spikelets dangling from a loose, nodding panicle.
        "wild_oat" => json!({
            "height": 0.5, "height_variation": 0.3, "width": 0.008, "tip": 2.5, "stiffness": 0.4,
            "lean": 25, "twist": 60, "segments": 6, "clump_size": 5, "clump_radius": 0.05, "clump_spread": 0.5,
            "base_color": [0.06, 0.14, 0.04], "tip_color": [0.22, 0.3, 0.12], "color_variation": 1.5,
            "seed_head": {
                "style": "panicle", "fraction": 0.4, "stem_height": 1.8, "stem_width": 0.003, "length": 0.22,
                "spikelets": 14, "branch_length": 0.06, "spikelet_length": 0.022, "spikelet_width": 0.006,
                "droop": 0.85, "color": [0.36, 0.4, 0.2], "tip_color": [0.6, 0.56, 0.36]
            }
        }),
        // Bent (Agrostis): fine leaves, and airy panicles of tiny reddish spikelets that make a
        // haze over a meadow.
        "bent" => json!({
            "height": 0.25, "height_variation": 0.35, "width": 0.003, "tip": 2, "stiffness": 0.45,
            "lean": 25, "twist": 60, "segments": 4, "clump_size": 8, "clump_radius": 0.04, "clump_spread": 0.6,
            "base_color": [0.05, 0.13, 0.03], "tip_color": [0.15, 0.26, 0.07],
            "seed_head": {
                "style": "panicle", "fraction": 0.5, "stem_height": 2, "stem_width": 0.0012, "length": 0.1,
                "spikelets": 45, "branch_length": 0.035, "spikelet_length": 0.003, "spikelet_width": 0.0013,
                "droop": 0.15, "color": [0.32, 0.17, 0.16], "tip_color": [0.45, 0.28, 0.24]
            }
        }),
        // Daisy (Bellis perennis): a low rosette, and short stems with white, pink-tipped rays
        // round a yellow disc. The defaults of a flower.
        "daisy" => json!({
            "kind": "flower", "height": 0.07, "height_variation": 0.3, "width": 0.0015, "stiffness": 0.7,
            "lean": 15, "twist": 0, "segments": 4, "clump_radius": 0.02,
            "base_color": [0.06, 0.14, 0.04], "tip_color": [0.1, 0.2, 0.06], "translucency": 0.3, "sheen": 0.2,
            "normal_alignment": 0.3
        }),
        // Oxeye daisy (Leucanthemum vulgare): the tall daisy of meadows.
        "oxeye" => json!({
            "kind": "flower", "height": 0.45, "height_variation": 0.3, "width": 0.0025, "stiffness": 0.65,
            "lean": 15, "twist": 0, "segments": 6, "clump_radius": 0.04,
            "base_color": [0.06, 0.14, 0.04], "tip_color": [0.1, 0.2, 0.06], "translucency": 0.3, "sheen": 0.2,
            "flower": {
                "count": 2.5, "petals": 24, "petal_length": 0.018, "petal_width": 0.0045,
                "petal_tip_color": [0.86, 0.86, 0.83], "centre_radius": 0.008, "dome": 0.3
            },
            "leaves": { "count": 6, "length": 0.07, "width": 0.015 }
        }),
        // Dandelion (Taraxacum officinale): layers of yellow rays, seed clocks on taller stems,
        // and a rosette of long leaves.
        "dandelion" => json!({
            "kind": "flower", "height": 0.2, "height_variation": 0.35, "width": 0.003, "stiffness": 0.55,
            "lean": 20, "twist": 0, "segments": 5, "clump_radius": 0.03,
            "base_color": [0.12, 0.18, 0.08], "tip_color": [0.2, 0.22, 0.1], "translucency": 0.3, "sheen": 0.3,
            "flower": {
                "count": 1.5, "petals": 110, "layers": 5, "petal_length": 0.016, "petal_width": 0.0018,
                "petal_shape": 0.75, "cup": 8, "layer_cup": 16, "curl": 0.05, "centre_radius": 0.004, "dome": 0.4,
                "petal_color": [0.82, 0.45, 0.01], "petal_tip_color": [0.92, 0.7, 0.03],
                "centre_color": [0.8, 0.45, 0.01], "sheen": 0.15, "clock_fraction": 0.35
            },
            "leaves": { "count": 10, "length": 0.14, "width": 0.022, "shape": 0.65, "angle": 30 }
        }),
        // Meadow buttercup (Ranunculus acris): five glossy yellow petals in a cup, on tall,
        // branching stems.
        "buttercup" => json!({
            "kind": "flower", "height": 0.45, "height_variation": 0.3, "width": 0.0018, "stiffness": 0.6,
            "lean": 20, "twist": 0, "segments": 6, "clump_radius": 0.06,
            "base_color": [0.06, 0.14, 0.04], "tip_color": [0.1, 0.2, 0.06], "translucency": 0.3, "sheen": 0.2,
            "flower": {
                "count": 3, "petals": 5, "petal_length": 0.009, "petal_width": 0.009, "petal_shape": 0.6,
                "cup": 40, "curl": 0.1, "centre_radius": 0.0025, "dome": 0.6,
                "petal_color": [0.9, 0.62, 0.02], "petal_tip_color": [0.95, 0.75, 0.05],
                "centre_color": [0.6, 0.6, 0.1], "sheen": 0.9
            },
            "leaves": { "count": 5, "length": 0.06, "width": 0.03, "shape": 0.5, "angle": 30 }
        }),
        // White clover (Trifolium repens): creeping, with round white heads. The defaults of
        // clover.
        "clover" => json!({
            "kind": "clover", "height": 0.08, "height_variation": 0.35, "width": 0.0012, "stiffness": 0.6,
            "lean": 25, "twist": 0, "segments": 3, "clump_radius": 0.08,
            "base_color": [0.12, 0.2, 0.08], "tip_color": [0.12, 0.2, 0.08], "translucency": 0.35, "sheen": 0.3
        }),
        // Red clover (Trifolium pratense): taller, with bigger, oval leaflets and pink heads.
        "red_clover" => json!({
            "kind": "clover", "height": 0.25, "height_variation": 0.3, "width": 0.002, "stiffness": 0.55,
            "lean": 20, "twist": 0, "segments": 5, "clump_radius": 0.06,
            "base_color": [0.1, 0.18, 0.07], "tip_color": [0.12, 0.2, 0.08], "translucency": 0.35, "sheen": 0.25,
            "leaves": { "count": 6, "length": 0.03, "width": 0.015, "shape": 0.55 },
            "flower": {
                "fraction": 0.3, "height": 1.2, "radius": 0.013, "florets": 60,
                "color": [0.55, 0.15, 0.35], "tip_color": [0.7, 0.3, 0.5], "old_color": [0.35, 0.2, 0.15]
            }
        }),
        // Heather (Calluna vulgaris): a low, dense shrub of wiry dark shoots, with spikes of tiny
        // purple flowers that brown as they dry.
        "heather" => json!({
            "height": 0.25, "height_variation": 0.35, "width": 0.003, "tip": 1.5, "stiffness": 0.8,
            "lean": 35, "twist": 90, "segments": 4, "clump_size": 40, "clump_radius": 0.15, "clump_spread": 0.8,
            "base_color": [0.05, 0.06, 0.03], "tip_color": [0.07, 0.1, 0.04], "dry_color": [0.2, 0.15, 0.1],
            "translucency": 0.15, "sheen": 0.1, "color_variation": 1.5,
            "seed_head": {
                "style": "spike", "fraction": 0.2, "stem_height": 1.15, "stem_width": 0.002, "length": 0.07,
                "width": 0.012, "spikelets": 40, "spikelet_length": 0.006, "spikelet_width": 0.004,
                "color": [0.26, 0.08, 0.2], "tip_color": [0.38, 0.16, 0.32], "dry_color": [0.24, 0.16, 0.13]
            }
        }),
        // Bracken (Pteridium aquilinum): fronds on tall stems, their broad triangular blades
        // held level, turning rust brown as they die back. The defaults of a fern.
        "bracken" => json!({
            "kind": "fern", "height": 0.9, "height_variation": 0.35, "width": 0.005, "stiffness": 0.55,
            "lean": 20, "twist": 0, "segments": 6, "clump_radius": 0.3, "normal_alignment": 0.2,
            "base_color": [0.08, 0.13, 0.04], "tip_color": [0.1, 0.18, 0.05], "dry_color": [0.42, 0.2, 0.07],
            "translucency": 0.4, "sheen": 0.2
        }),
        // Dead blades lying on the ground: not for fields' species lists, but grown under them
        // by their conditions' `litter`.
        "litter" => json!({
            // Leans are drawn up to this and capped at 81 degrees, so most lie flat.
            "height": 0.16, "height_variation": 0.5, "width": 0.004, "tip": 2, "stiffness": 0,
            "lean": 160, "twist": 120, "segments": 3, "dryness": 1, "normal_alignment": 1,
            "base_color": [0.26, 0.22, 0.15], "tip_color": [0.34, 0.29, 0.2], "dry_color": [0.3, 0.26, 0.19],
            "translucency": 0.15, "sheen": 0.05, "color_variation": 2
        }),
        _ => panic!("Unknown grass species preset {}", name),
    };
    merge(lawn, &overrides)
}

/// Defaults for a grass's `seed_head`, by style.
fn seed_head_defaults(style: &str) -> Value {
    match style {
        "spike" => json!({
            "fraction": 0.25, "stem_height": 1.9, "stem_width": 0.0022, "length": 0.09, "width": 0.008,
            "spikelets": 90, "spikelet_length": 0.0065, "spikelet_width": 0.003,
            "branch_length": 0, "droop": 0,
            "color": [0.34, 0.37, 0.25], "tip_color": [0.48, 0.45, 0.38], "dry_color": [0.55, 0.47, 0.3]
        }),
        "panicle" => json!({
            "fraction": 0.25, "stem_height": 1.6, "stem_width": 0.0015, "length": 0.15, "width": 0,
            "spikelets": 16, "spikelet_length": 0.012, "spikelet_width": 0.004,
            "branch_length": 0.05, "droop": 0.5,
            "color": [0.34, 0.36, 0.2], "tip_color": [0.5, 0.46, 0.3], "dry_color": [0.58, 0.5, 0.32]
        }),
        _ => panic!("Unknown seed head style {} (expected spike or panicle)", style),
    }
}

/// Defaults for a flower's `flower` and `leaves`: a daisy.
fn flower_defaults() -> Value {
    json!({
        "flower": {
            "count": 1.5, "petals": 36, "layers": 1, "petal_length": 0.008, "petal_width": 0.0018,
            "petal_shape": 0.6, "cup": 5, "layer_cup": 20, "curl": -0.08, "centre_radius": 0.0035, "dome": 0.5,
            "petal_color": [0.86, 0.86, 0.83], "petal_tip_color": [0.85, 0.7, 0.74],
            "centre_color": [0.8, 0.55, 0.03], "sheen": 0.1,
            "clock_fraction": 0, "clock_radius": 0.018, "clock_height": 1.5, "pappus": 110,
            "clock_color": [0.85, 0.85, 0.83]
        },
        "leaves": {
            "count": 8, "length": 0.04, "width": 0.012, "shape": 0.75, "angle": 12,
            "color": [0.05, 0.14, 0.03], "tip_color": [0.07, 0.18, 0.04]
        }
    })
}

/// Defaults for a fern's `frond`: bracken.
fn fern_defaults() -> Value {
    json!({
        "frond": {
            "count": 2.5, "blade_start": 0.4, "pinnae": 14, "pinna_length": 0.22, "pinna_width": 0.05,
            "sweep": 25, "droop": 0.15
        }
    })
}

/// Defaults for clover's `leaves` and `flower`: white clover.
fn clover_defaults() -> Value {
    json!({
        "leaves": {
            "count": 8, "length": 0.012, "width": 0.011, "shape": 0.7, "angle": 10,
            "color": [0.04, 0.13, 0.03], "tip_color": [0.05, 0.15, 0.035], "band_color": [0.2, 0.3, 0.15]
        },
        "flower": {
            "fraction": 0.12, "height": 1.4, "radius": 0.01, "florets": 45, "wilted": 0.3,
            "color": [0.85, 0.85, 0.8], "tip_color": [0.88, 0.84, 0.84], "old_color": [0.4, 0.3, 0.2]
        }
    })
}

fn conditions_preset(name: &str) -> Value {
    let summer = json!({
        "wind": { "direction": [1, 0], "strength": 0.15, "gustiness": 0.5, "scale": 6 },
        "dryness": 0.1, "dead_fraction": 0.03, "wetness": 0, "frost": 0,
        "patchiness": 0.1, "patch_scale": 3, "mow_height": null,
        "mow_stripes": { "width": 0, "direction": [1, 0] },
        "trampled": 0, "max_slope": 50, "min_altitude": -1e9, "max_altitude": 1e9,
        "altitude_fade": 0.5, "species_zones": 0, "litter": 0
    });
    let overrides = match name {
        "summer" => json!({}),
        "lush" => json!({ "dryness": 0, "dead_fraction": 0.01, "wetness": 0.15, "patchiness": 0 }),
        "drought" => json!({ "dryness": 0.75, "dead_fraction": 0.3, "patchiness": 0.45, "patch_scale": 2 }),
        "autumn" => json!({ "dryness": 0.4, "dead_fraction": 0.12, "wetness": 0.3 }),
        "after_rain" => json!({ "dryness": 0.05, "wetness": 0.9, "wind": { "strength": 0.05 } }),
        "frost" => json!({ "frost": 0.8, "dryness": 0.25, "dead_fraction": 0.08, "wind": { "strength": 0 } }),
        "windy" => json!({ "wind": { "strength": 0.8, "gustiness": 0.7, "scale": 4 } }),
        "manicured" => json!({
            "mow_height": 0.04, "mow_stripes": { "width": 1, "direction": [1, 0] }, "dryness": 0.02,
            "dead_fraction": 0.005, "patchiness": 0, "wind": { "strength": 0 }
        }),
        "trampled" => json!({ "trampled": 0.7, "dryness": 0.2, "dead_fraction": 0.1, "patchiness": 0.3 }),
        // Left to grow: last year's dead stems among the new, uneven growth, and plants in
        // drifts rather than evenly mixed.
        "wild" => json!({
            "dryness": 0.15, "dead_fraction": 0.12, "patchiness": 0.25, "patch_scale": 2.5,
            "species_zones": 2.5, "wind": { "strength": 0.25, "gustiness": 0.6, "scale": 5 }, "litter": 600
        }),
        _ => panic!("Unknown grass conditions preset {}", name),
    };
    merge(summer, &overrides)
}

/// `base` with the fields of `over` replacing its own, merging objects field by field.
fn merge(mut base: Value, over: &Value) -> Value {
    if let (Some(b), Some(o)) = (base.as_object_mut(), over.as_object()) {
        for (k, v) in o {
            let merged = match b.remove(k) {
                Some(existing) if existing.is_object() && v.is_object() => merge(existing, v),
                _ => v.clone(),
            };
            b.insert(k.clone(), merged);
        }
        return base;
    }
    over.clone()
}

/// A preset name, or an object with an optional "preset" and fields overriding it.
fn with_preset(v: &Value, default: &str, preset: fn(&str) -> Value) -> Value {
    match v {
        Value::String(name) => preset(name),
        Value::Object(fields) => {
            let mut fields: Map<String, Value> = fields.clone();
            let name = fields.remove("preset").and_then(|p| p.as_str().map(String::from)).unwrap_or(default.to_string());
            merge(preset(&name), &Value::Object(fields))
        },
        Value::Null => preset(default),
        _ => panic!("Expected a preset name or an object, got {}", v),
    }
}

fn number(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("grass: \"{}\" should be a number, got {}", k, v[k]))
}

fn count(v: &Value, k: &str) -> usize {
    number(v, k).round().max(0.) as usize
}

fn color(v: &Value, k: &str) -> Color {
    SceneFile::parse_color(&v[k])
}

/// A horizontal unit vector from an [x, z] array.
fn direction(v: &Value) -> Vector3<f64> {
    let d = Vector3::new(v[0].as_f64().unwrap_or(1.), 0., v[1].as_f64().unwrap_or(0.));
    if d.norm() < 1e-12 { Vector3::new(1., 0., 0.) } else { d.normalize() }
}

impl Species {
    pub fn from_json(v: &Value) -> Species {
        let v = with_preset(v, "meadow", species_preset);
        let mut sp = Species {
            weight: number(&v, "weight").max(0.),
            height: number(&v, "height").max(1e-4),
            height_variation: number(&v, "height_variation").clamp(0., 0.95),
            width: number(&v, "width").max(1e-5),
            tip: number(&v, "tip").max(0.1),
            stiffness: number(&v, "stiffness").clamp(0., 1.),
            lean: number(&v, "lean").to_radians(),
            twist: number(&v, "twist").to_radians(),
            segments: SceneFile::parse_int(&v["segments"], 4).clamp(1, MAX_SEGMENTS),
            clump_size: SceneFile::parse_int(&v["clump_size"], 1).max(1),
            clump_radius: number(&v, "clump_radius").max(0.),
            clump_spread: number(&v, "clump_spread").clamp(0., 1.),
            base_color: color(&v, "base_color"),
            tip_color: color(&v, "tip_color"),
            dry_color: color(&v, "dry_color"),
            dryness: number(&v, "dryness").clamp(0., 1.),
            translucency: number(&v, "translucency").clamp(0., 1.),
            sheen: number(&v, "sheen").clamp(0., 1.),
            normal_alignment: number(&v, "normal_alignment").clamp(0., 1.),
            color_variation: v["color_variation"].as_f64().unwrap_or(1.).clamp(0., 2.2),
            kind: Kind::Grass(None),
            looks: vec![],
        };
        let kind = v["kind"].as_str().unwrap_or("grass");
        sp.kind = match kind {
            "grass" => Kind::Grass(match &v["seed_head"] {
                Value::Null => None,
                h => Some(Box::new(sp.seed_head(h))),
            }),
            "flower" => Kind::Flower(Box::new(sp.flower(&merge(flower_defaults(), &v)))),
            "clover" => Kind::Clover(Box::new(sp.clover(&merge(clover_defaults(), &v)))),
            "fern" => Kind::Fern(Box::new(sp.fern(&merge(fern_defaults(), &v)))),
            _ => panic!("Unknown grass species kind {} (expected grass, flower, clover or fern)", kind),
        };
        sp.looks = sp.make_looks(&v);
        sp
    }

    /// This species' blade shape for a stem `height` times as tall, of the given width.
    fn stalk(&self, height: f64, width: f64) -> Species {
        Species {
            height: self.height * height,
            width,
            twist: 0.,
            kind: Kind::Grass(None),
            looks: vec![],
            ..self.clone()
        }
    }

    fn seed_head(&self, h: &Value) -> SeedHead {
        let style = h["style"].as_str().unwrap_or("spike");
        let h = merge(seed_head_defaults(style), h);
        let mut stem = self.stalk(number(&h, "stem_height").max(0.1), number(&h, "stem_width").max(1e-5));
        // Flowering stems are stiffer and straighter than leaves, and don't fan out.
        stem.stiffness = 0.5 + 0.5 * self.stiffness;
        stem.lean *= 0.6;
        stem.clump_spread *= 0.5;
        SeedHead {
            panicle: style == "panicle",
            fraction: number(&h, "fraction").clamp(0., 1.),
            stem,
            length: number(&h, "length").max(1e-4),
            width: number(&h, "width").max(0.),
            spikelets: count(&h, "spikelets"),
            spikelet_length: number(&h, "spikelet_length").max(1e-5),
            spikelet_width: number(&h, "spikelet_width").max(1e-5),
            branch_length: number(&h, "branch_length").max(0.),
            droop: number(&h, "droop").clamp(0., 1.),
        }
    }

    fn flower(&self, v: &Value) -> Flower {
        let (f, l) = (&v["flower"], &v["leaves"]);
        Flower {
            count: number(f, "count").max(0.),
            stem: self.stalk(1., self.width),
            clock_stem: self.stalk(number(f, "clock_height").max(0.1), self.width),
            petals: count(f, "petals"),
            layers: count(f, "layers").max(1),
            petal_length: number(f, "petal_length").max(1e-5),
            petal_width: number(f, "petal_width").max(1e-5),
            cup: number(f, "cup").to_radians(),
            layer_cup: number(f, "layer_cup").to_radians(),
            curl: number(f, "curl"),
            centre_radius: number(f, "centre_radius").max(0.),
            dome: number(f, "dome"),
            clock_fraction: number(f, "clock_fraction").clamp(0., 1.),
            clock_radius: number(f, "clock_radius").max(1e-4),
            pappus: count(f, "pappus"),
            leaves: count(l, "count"),
            leaf_length: number(l, "length").max(1e-5),
            leaf_width: number(l, "width").max(1e-5),
            leaf_angle: number(l, "angle").to_radians(),
        }
    }

    fn fern(&self, v: &Value) -> Fern {
        let f = &v["frond"];
        Fern {
            fronds: number(f, "count").max(0.),
            blade_start: number(f, "blade_start").clamp(0., 0.95),
            pinnae: count(f, "pinnae"),
            pinna_length: number(f, "pinna_length").max(1e-4),
            pinna_width: number(f, "pinna_width").max(1e-5),
            sweep: number(f, "sweep").to_radians(),
            droop: number(f, "droop"),
        }
    }

    fn clover(&self, v: &Value) -> Clover {
        let (l, f) = (&v["leaves"], &v["flower"]);
        Clover {
            leaves: count(l, "count"),
            stem: self.stalk(1., self.width),
            leaflet_length: number(l, "length").max(1e-5),
            leaflet_width: number(l, "width").max(1e-5),
            leaf_angle: number(l, "angle").to_radians(),
            flower_fraction: number(f, "fraction").clamp(0., 1.),
            flower_stem: self.stalk(number(f, "height").max(0.1), self.width * 1.2),
            flower_radius: number(f, "radius").max(1e-4),
            florets: count(f, "florets"),
            wilted: number(f, "wilted").clamp(0., 1.),
        }
    }

    /// The looks of this species' parts, in the order of the part constants for its kind.
    fn make_looks(&self, v: &Value) -> Vec<Look> {
        let leafy = [self.base_color, self.tip_color, self.dry_color];
        let mut blade = Look::new(Outline::Blade(self.tip), self.segments, leafy, self.translucency, self.sheen, 1.);
        blade.color_variation = self.color_variation;
        let stem = Look::new(Outline::Stem, self.segments.max(3), leafy, self.translucency * 0.6, self.sheen, 1.);
        let withered = Color::new(0.36, 0.3, 0.2);
        match &self.kind {
            Kind::Grass(None) => vec![blade],
            Kind::Grass(Some(h)) => {
                let style = v["seed_head"]["style"].as_str().unwrap_or("spike");
                let hv = merge(seed_head_defaults(style), &v["seed_head"]);
                let colors = [color(&hv, "color"), color(&hv, "tip_color"), color(&hv, "dry_color")];
                let mut spikelet = Look::new(Outline::Petal(0.45), if h.panicle { 3 } else { 2 }, colors, 0.35, 0.2, 1.);
                spikelet.color_variation = 1.5;
                spikelet.leafy = false;
                vec![blade, stem, spikelet]
            },
            Kind::Flower(_) => {
                let d = merge(flower_defaults(), v);
                let (f, l) = (&d["flower"], &d["leaves"]);
                let leaf = Look::new(Outline::Petal(number(l, "shape").clamp(0.05, 0.95)), 4,
                    [color(l, "color"), color(l, "tip_color"), self.dry_color], self.translucency, self.sheen, 0.7);
                let mut petal = Look::new(Outline::Petal(number(f, "petal_shape").clamp(0.05, 0.95)), 3,
                    [color(f, "petal_color"), color(f, "petal_tip_color"), withered], 0.4, number(f, "sheen").clamp(0., 1.), 0.25);
                let centre = color(f, "centre_color");
                let mut disc = Look::new(Outline::Wedge, 1, [centre, centre, withered], 0.1, 0.1, 0.2);
                let mut pappus = Look::new(Outline::Plume, 3,
                    [Color::new(0.4, 0.35, 0.28), color(f, "clock_color"), Color::white()], 0.6, 0., 0.);
                (petal.leafy, disc.leafy, pappus.leafy) = (false, false, false);
                vec![stem, leaf, petal, disc, pappus]
            },
            Kind::Fern(_) => {
                // Pinnae are themselves cut into many small lobes; at a distance a long,
                // narrow leaf with a pointed tip stands in for them.
                let mut pinna = Look::new(Outline::Petal(0.3), 4, leafy, self.translucency, self.sheen, 1.);
                pinna.color_variation = self.color_variation;
                vec![stem, pinna]
            },
            Kind::Clover(_) => {
                let d = merge(clover_defaults(), v);
                let (l, f) = (&d["leaves"], &d["flower"]);
                let mut leaflet = Look::new(Outline::Petal(number(l, "shape").clamp(0.05, 0.95)), 4,
                    [color(l, "color"), color(l, "tip_color"), self.dry_color], self.translucency, self.sheen, 0.6);
                leaflet.band = Some(color(l, "band_color"));
                let mut floret = Look::new(Outline::Petal(0.7), 2,
                    [color(f, "color"), color(f, "tip_color"), withered], 0.35, 0.1, 0.3);
                let old = color(f, "old_color");
                let mut wilted = Look::new(Outline::Petal(0.7), 2, [old, old, old], 0.2, 0.05, 0.);
                (floret.leafy, wilted.leafy) = (false, false);
                vec![stem, leaflet, floret, wilted]
            },
        }
    }

    /// Plants for each tuft, as far as `density` (blades per square metre) counts them: a
    /// grass tuft's blades, or one flowering plant or clover.
    fn per_tuft(&self) -> f64 {
        match self.kind {
            Kind::Grass(_) => self.clump_size as f64,
            _ => 1.,
        }
    }
}

impl Conditions {
    pub fn from_json(v: &Value) -> Conditions {
        let v = with_preset(v, "summer", conditions_preset);
        let wind = &v["wind"];
        let stripes = &v["mow_stripes"];
        Conditions {
            wind: direction(&wind["direction"]) * number(wind, "strength").max(0.),
            gustiness: number(wind, "gustiness").clamp(0., 1.),
            gust_scale: number(wind, "scale").max(1e-3),
            dryness: number(&v, "dryness").clamp(0., 1.),
            dead_fraction: number(&v, "dead_fraction").clamp(0., 1.),
            wetness: number(&v, "wetness").clamp(0., 1.),
            frost: number(&v, "frost").clamp(0., 1.),
            patchiness: number(&v, "patchiness").clamp(0., 1.),
            patch_scale: number(&v, "patch_scale").max(1e-3),
            mow_height: v["mow_height"].as_f64(),
            stripe_width: number(stripes, "width").max(0.),
            stripe_direction: direction(&stripes["direction"]),
            trampled: number(&v, "trampled").clamp(0., 1.),
            max_slope: number(&v, "max_slope").to_radians(),
            min_altitude: number(&v, "min_altitude"),
            max_altitude: number(&v, "max_altitude"),
            altitude_fade: number(&v, "altitude_fade").max(1e-6),
            species_zones: number(&v, "species_zones").max(0.),
            litter: number(&v, "litter").max(0.),
        }
    }
}

/// A blade, stored compactly: a field can hold millions.
#[derive(Clone, Debug)]
struct Blade {
    /// Bézier control points: root, middle, tip.
    v: [[f32; 3]; 3],
    /// Unit vectors across the blade's face, and the way it faces (and bends).
    side: [f32; 3],
    facing: [f32; 3],
    width: f32,
    twist: f32,
    lo: [f32; 3],
    hi: [f32; 3],
    /// Which of the field's looks it has.
    look: u8,
    /// Palette entry, dryness level * JITTER_LEVELS + jitter.
    shade: u8,
    cut: bool,
}

fn v3(a: [f32; 3]) -> Vector3<f64> { Vector3::new(a[0] as f64, a[1] as f64, a[2] as f64) }
fn f3(a: Vector3<f64>) -> [f32; 3] { [a.x as f32, a.y as f32, a.z as f32] }

/// The edges of a blade's ribbon: left and right points and `s` along the blade, at each end
/// of each segment.
struct Ribbon {
    left: [Vector3<f64>; MAX_SEGMENTS + 1],
    right: [Vector3<f64>; MAX_SEGMENTS + 1],
    n: usize,
}

impl Blade {
    fn point(&self, s: f64) -> Vector3<f64> {
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        a * ((1. - s) * (1. - s)) + b * (2. * s * (1. - s)) + c * (s * s)
    }

    fn tangent(&self, s: f64) -> Vector3<f64> {
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        (b - a) * (2. * (1. - s)) + (c - b) * (2. * s)
    }

    fn side_at(&self, s: f64) -> Vector3<f64> {
        let a = self.twist as f64 * s;
        v3(self.side) * a.cos() + v3(self.facing) * a.sin()
    }

    fn normal_at(&self, s: f64) -> Vector3<f64> {
        let n = self.tangent(s).cross(&self.side_at(s));
        if n.norm() < 1e-12 { v3(self.facing) } else { n.normalize() }
    }

    fn ribbon(&self, sp: &Look) -> Ribbon {
        let n = sp.segments;
        let mut r = Ribbon { left: [Vector3::zeros(); MAX_SEGMENTS + 1], right: [Vector3::zeros(); MAX_SEGMENTS + 1], n };
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        let (side, facing) = (v3(self.side), v3(self.facing));
        // The twist turns the side vector by the same angle each segment: rotate it
        // incrementally rather than taking a sine and cosine per point.
        let (sin_step, cos_step) = (self.twist as f64 / n as f64).sin_cos();
        let (mut sin_a, mut cos_a) = (0., 1.);
        let taper = &sp.tapers[usize::from(self.cut)];
        for (i, width) in taper.iter().enumerate().take(n + 1) {
            let s = i as f64 / n as f64;
            let p = a * ((1. - s) * (1. - s)) + b * (2. * s * (1. - s)) + c * (s * s);
            let half = (side * cos_a + facing * sin_a) * (0.5 * self.width as f64 * width);
            r.left[i] = p - half;
            r.right[i] = p + half;
            (sin_a, cos_a) = (sin_a * cos_step + cos_a * sin_step, cos_a * cos_step - sin_a * sin_step);
        }
        r
    }

    /// Nearest hit between t0 and t1: t, and s along the blade.
    fn intersect(&self, sp: &Look, r: &Ray, t0: f64, t1: f64) -> Option<(f64, f64)> {
        let rb = self.ribbon(sp);
        let ds = 1. / rb.n as f64;
        let mut best: Option<(f64, f64)> = None;
        for i in 0 .. rb.n {
            let s0 = i as f64 * ds;
            let tris = [
                (rb.left[i], rb.right[i], rb.left[i + 1], false),
                (rb.right[i], rb.right[i + 1], rb.left[i + 1], true),
            ];
            for (a, b, c, second) in tris {
                if let Some((t, u, v)) = intersect_triangle(r, a, b, c)
                    && t >= t0 && t <= t1 && best.is_none_or(|(bt, _)| t < bt) {
                    let s = s0 + ds * if second { u + v } else { v };
                    best = Some((t, s));
                }
            }
        }
        best
    }

    /// If `p` is on the blade (within `tolerance`), its distance from it and s along it.
    fn locate(&self, sp: &Look, p: Vector3<f64>, tolerance: f64) -> Option<(f64, f64)> {
        let rb = self.ribbon(sp);
        let ds = 1. / rb.n as f64;
        let mut best: Option<(f64, f64)> = None;
        for i in 0 .. rb.n {
            let s0 = i as f64 * ds;
            let tris = [
                (rb.left[i], rb.right[i], rb.left[i + 1], false),
                (rb.right[i], rb.right[i + 1], rb.left[i + 1], true),
            ];
            for (a, b, c, second) in tris {
                if let Some((d, u, v)) = point_on_triangle(p, a, b, c, tolerance)
                    && best.is_none_or(|(bd, _)| d < bd) {
                    let s = s0 + ds * if second { u + v } else { v };
                    best = Some((d, s.clamp(0., 1.)));
                }
            }
        }
        best
    }

    /// Arc length, from chords.
    fn length(&self) -> f64 {
        (0 .. 8).map(|i| (self.point((i + 1) as f64 / 8.) - self.point(i as f64 / 8.)).norm()).sum()
    }

    fn bounds_hit(&self, r: &Ray, invrd: &Vector3<f64>, t0: f64, t1: f64) -> bool {
        let b = BBox::new(v3(self.lo), v3(self.hi));
        b.entry_exit(&r.ro, invrd).is_some_and(|(a, z)| z >= t0 && a <= t1)
    }
}

/// Distance of `p` from the triangle's plane and its barycentric (u, v), if it projects inside
/// the triangle (allowing `tolerance`, in barycentric units) and is within `tolerance` of it.
fn point_on_triangle(p: Vector3<f64>, a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>, tolerance: f64) -> Option<(f64, f64, f64)> {
    let (e1, e2) = (b - a, c - a);
    let n = e1.cross(&e2);
    let area2 = n.norm();
    if area2 < 1e-16 { return None }
    let n = n / area2;
    let w = p - a;
    let d = w.dot(&n);
    if d.abs() > tolerance { return None }
    let q = w - n * d;
    // Barycentric coordinates from sub-areas.
    let u = q.cross(&e2).dot(&n) / area2;
    let v = e1.cross(&q).dot(&n) / area2;
    let slack = 0.05;
    if u < -slack || v < -slack || u + v > 1. + slack { return None }
    Some((d.abs(), u, v))
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Fractal noise in [0, 1], stretched so values spread across the range rather than bunching
/// around a half.
fn field_noise(noise: &PerlinNoise, x: f64, z: f64, scale: f64, layer: f64) -> f64 {
    let n = noise.fbm(x / scale + layer * 13.7, layer * 3.1 + 0.5, z / scale - layer * 7.3, 4, 0.5, 2.);
    ((n - 0.5) * 3. + 0.5).clamp(0., 1.)
}

/// SplitMix64, to give each generation tile its own seed.
fn hash(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// What is needed to generate blades.
struct Generator<'a> {
    grid: &'a HeightGrid,
    species: &'a [Species],
    conditions: &'a Conditions,
    noise: PerlinNoise,
    seed: u64,
    min: (f64, f64),
    max: (f64, f64),
    /// Tufts per square metre where nothing thins them.
    clump_density: f64,
    /// Each species' first look in the field's list.
    first_look: &'a [usize],
    /// The dead blades of litter, and their look.
    litter: &'a Species,
    litter_look: usize,
}

impl Generator<'_> {
    /// How well grass grows at (x, z), in [0, 1], from patches, slope and altitude.
    fn vigour(&self, x: f64, z: f64, y: f64, normal: &Vector3<f64>) -> (f64, f64) {
        let c = self.conditions;
        let patch = if c.patchiness > 0. {
            let threshold = c.patchiness * 1.1 - 0.1;
            smoothstep(threshold - 0.1, threshold + 0.15, field_noise(&self.noise, x, z, c.patch_scale, self.seed as f64 * 0.1 + 1.))
        } else { 1. };
        let slope = normal.y.clamp(-1., 1.).acos();
        let slope_ok = 1. - smoothstep(c.max_slope - 10f64.to_radians(), c.max_slope, slope);
        let altitude = smoothstep(c.min_altitude, c.min_altitude + c.altitude_fade, y)
            * (1. - smoothstep(c.max_altitude - c.altitude_fade, c.max_altitude, y));
        (patch, slope_ok * altitude)
    }

    fn pick_species(&self, x: f64, z: f64, rng: &mut Pcg32) -> usize {
        let total: f64 = self.species.iter().map(|s| s.weight).sum();
        let random = rng.next_f64();
        let u = if self.conditions.species_zones > 0. && self.species.len() > 1 {
            let zone = field_noise(&self.noise, x, z, self.conditions.species_zones, self.seed as f64 * 0.1 + 2.);
            (0.8 * zone + 0.2 * random).clamp(0., 1. - 1e-12)
        } else { random };
        let mut acc = 0.;
        for (i, s) in self.species.iter().enumerate() {
            acc += s.weight / total;
            if u < acc { return i }
        }
        self.species.len() - 1
    }

    fn tile(&self, tx: usize, tz: usize) -> Vec<Blade> {
        let x0 = self.min.0 + tx as f64 * TILE;
        let z0 = self.min.1 + tz as f64 * TILE;
        let (w, d) = ((self.max.0 - x0).min(TILE), (self.max.1 - z0).min(TILE));
        let mut rng = Pcg32::new(hash(self.seed ^ hash(((tx as u64) << 32) | tz as u64)), 0x5eed);
        let expected = self.clump_density * w * d;
        let count = expected.floor() as usize + usize::from(rng.next_f64() < expected.fract());
        let mut blades = vec![];
        for _ in 0 .. count {
            let (x, z) = (x0 + rng.next_f64() * w, z0 + rng.next_f64() * d);
            let y = self.grid.height(x, z);
            let normal = self.grid.normal(x, z);
            let (patch, grows) = self.vigour(x, z, y, &normal);
            let keep = patch * grows;
            if rng.next_f64() >= keep { continue }
            let si = self.pick_species(x, z, &mut rng);
            let sp = &self.species[si];
            let centre = Vector3::new(x, y, z);
            let look = self.first_look[si];
            match &sp.kind {
                Kind::Grass(head) => for _ in 0 .. sp.clump_size {
                    // Uniform in the tuft's disc.
                    let (rr, a) = (rng.next_f64().sqrt() * sp.clump_radius, rng.next_f64() * 2. * PI);
                    let (bx, bz) = (x + rr * a.cos(), z + rr * a.sin());
                    if !self.grid.contains(bx, bz) { continue }
                    match head {
                        Some(h) if rng.next_f64() < h.fraction => self.culm(h, look, centre, bx, bz, patch, &mut rng, &mut blades),
                        _ => blades.push(self.blade(sp, look + BLADE, centre, bx, bz, patch, &mut rng)),
                    }
                },
                Kind::Flower(f) => self.flower(sp, f, look, centre, patch, &mut rng, &mut blades),
                Kind::Clover(cl) => self.clover(sp, cl, look, centre, patch, &mut rng, &mut blades),
                Kind::Fern(f) => self.fern(sp, f, look, centre, patch, &mut rng, &mut blades),
            }
        }
        if self.conditions.litter > 0. {
            // From a random sequence of its own, so litter doesn't change the living plants.
            let mut rng = Pcg32::new(hash(self.seed ^ hash(((tx as u64) << 32) | tz as u64)), 0x1177e7);
            let expected = self.conditions.litter * w * d;
            let count = expected.floor() as usize + usize::from(rng.next_f64() < expected.fract());
            for _ in 0 .. count {
                let (x, z) = (x0 + rng.next_f64() * w, z0 + rng.next_f64() * d);
                let y = self.grid.height(x, z);
                let (patch, grows) = self.vigour(x, z, y, &self.grid.normal(x, z));
                if rng.next_f64() >= grows * (0.3 + 0.7 * patch) { continue }
                blades.push(self.blade(self.litter, self.litter_look, Vector3::new(x, y, z), x, z, patch, &mut rng));
            }
        }
        blades
    }

    /// The way up for plants of this species at (x, z).
    fn up(&self, sp: &Species, x: f64, z: f64) -> Vector3<f64> {
        (Vector3::new(0., 1., 0.) * (1. - sp.normal_alignment) + self.grid.normal(x, z) * sp.normal_alignment).normalize()
    }

    /// A point uniformly in the disc of `radius` about `centre`, if it's on the heightmap.
    fn in_disc(&self, centre: Vector3<f64>, radius: f64, rng: &mut Pcg32) -> Option<(f64, f64)> {
        let (rr, a) = (rng.next_f64().sqrt() * radius, rng.next_f64() * 2. * PI);
        let (x, z) = (centre.x + rr * a.cos(), centre.z + rr * a.sin());
        if self.grid.contains(x, z) { Some((x, z)) } else { None }
    }

    /// A flowering stem of grass, with its seed head.
    #[allow(clippy::too_many_arguments)]
    fn culm(&self, h: &SeedHead, look: usize, centre: Vector3<f64>, x: f64, z: f64, patch: f64, rng: &mut Pcg32, out: &mut Vec<Blade>) {
        let stem = self.blade(&h.stem, look + STEM, centre, x, z, patch, rng);
        if stem.cut {
            out.push(stem);
            return;
        }
        let shade = stem.shade;
        let length = stem.length();
        let s0 = (1. - h.length / length).clamp(0.3, 0.97);
        let n = h.spikelets.max(1);
        if !h.panicle {
            // Spikelets packed round the top of the stem, angled out so that together they make
            // a cylinder of the head's width.
            let angle = (0.5 * h.width / h.spikelet_length).min(0.95).asin();
            for k in 0 .. n {
                let u = (k as f64 + 0.5) / n as f64;
                let s = s0 + (1. - s0) * u;
                let (p, t) = (stem.point(s), stem.tangent(s).normalize());
                let (e1, e2) = frame(t);
                let a = k as f64 * GOLDEN_ANGLE + 0.5 * rng.next_f64();
                let r = e1 * a.cos() + e2 * a.sin();
                // Shorter at the ends, so the head is rounded.
                let l = h.spikelet_length * (0.5 + 0.5 * (PI * u).sin().sqrt()) * (0.85 + 0.3 * rng.next_f64());
                let d = t * angle.cos() + r * angle.sin();
                let v0 = p + r * (0.5 * h.stem.width) - t * (0.2 * l);
                let v2 = v0 + d * l;
                let v1 = (v0 + v2) * 0.5 + r * (0.05 * l);
                out.push(part(look + SPIKELET, shade, v0, v1, v2, t.cross(&r), h.spikelet_width * (0.8 + 0.4 * rng.next_f64())));
            }
        } else {
            // Branches from along the top of the stem, longest at the bottom, each with a
            // spikelet hanging from its end. Wind blows them downwind.
            let down = Vector3::new(0., -1., 0.) + self.conditions.wind * 0.6;
            for k in 0 .. n {
                let u = (k as f64 + rng.next_f64()) / n as f64;
                let s = s0 + (1. - s0) * u;
                let (p, t) = (stem.point(s), stem.tangent(s).normalize());
                let (e1, e2) = frame(t);
                let a = k as f64 * GOLDEN_ANGLE + rng.next_f64();
                let r = e1 * a.cos() + e2 * a.sin();
                let spread = 0.5 + 0.5 * rng.next_f64();
                let d0 = t * spread.cos() + r * spread.sin();
                let bl = h.branch_length * (1. - 0.7 * u) * (0.6 + 0.8 * rng.next_f64());
                let end = if bl > 1e-6 {
                    let v1 = p + d0 * (0.55 * bl);
                    let end = p + (d0 + down * (0.8 * h.droop)).normalize() * bl;
                    out.push(part(look + STEM, shade, p, v1, end, t.cross(&r), h.stem.width * 0.5));
                    end
                } else { p };
                let along = if bl > 1e-6 { (d0 + down * (0.8 * h.droop)).normalize() } else { d0 };
                let hang = (along * (1. - h.droop) + down * h.droop).normalize();
                let l = h.spikelet_length * (0.8 + 0.4 * rng.next_f64());
                let v2 = end + hang * l;
                let side = hang.cross(&Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5));
                out.push(part(look + SPIKELET, shade, end, (end + v2) * 0.5, v2, side, h.spikelet_width * (0.8 + 0.4 * rng.next_f64())));
            }
        }
        out.push(stem);
    }

    /// A flowering plant: its rosette of leaves, and its stems with flowers or seed clocks.
    #[allow(clippy::too_many_arguments)]
    fn flower(&self, sp: &Species, f: &Flower, look: usize, centre: Vector3<f64>, patch: f64, rng: &mut Pcg32, out: &mut Vec<Blade>) {
        let c = self.conditions;
        let up = self.up(sp, centre.x, centre.z);
        let shade = self.shade(self.dryness(sp, patch, rng).1, rng);
        let lushness = (0.55 + 0.45 * patch) * (1. - 0.35 * c.trampled);
        let v0 = centre - up * 0.002;
        for i in 0 .. f.leaves {
            let a = i as f64 * GOLDEN_ANGLE + 0.6 * rng.next_f64();
            let dir = Vector3::new(a.cos(), 0., a.sin());
            let l = f.leaf_length * (0.7 + 0.6 * rng.next_f64()) * lushness;
            let rise = (f.leaf_angle * (0.5 + rng.next_f64())).min(PI * 0.45);
            let mut v1 = v0 + dir * (0.5 * l) + up * (0.75 * l * rise.sin());
            let mut v2 = v0 + dir * (0.95 * l * rise.cos()) + up * (0.5 * l * rise.sin());
            v1.y = v1.y.max(self.grid.height(v1.x, v1.z) + 0.002);
            v2.y = v2.y.max(self.grid.height(v2.x, v2.z) + 0.002);
            out.push(part(look + LEAF, shade, v0, v1, v2, up.cross(&dir), f.leaf_width * (0.8 + 0.4 * rng.next_f64())));
        }
        let stems = (f.count * (0.5 + rng.next_f64())).round() as usize;
        for _ in 0 .. stems {
            let Some((x, z)) = self.in_disc(centre, sp.clump_radius, rng) else { continue };
            let clock = rng.next_f64() < f.clock_fraction;
            let stem = self.blade(if clock { &f.clock_stem } else { &f.stem }, look + FLOWER_STEM, centre, x, z, patch, rng);
            let (p, shade) = (stem.point(1.), stem.shade);
            // Heads turn up, towards the light.
            let n = (stem.tangent(1.).normalize() + up).normalize();
            let (e1, e2) = frame(n);
            let turn = rng.next_f64() * 2. * PI;
            if clock {
                // Seeds on a sphere, on stalks ending in parachutes, but none where the stem is.
                let radius = f.clock_radius * (0.85 + 0.3 * rng.next_f64());
                for k in 0 .. f.pappus {
                    let y = 1. - 2. * (k as f64 + 0.5) / f.pappus as f64;
                    if y < -0.55 { continue }
                    let a = k as f64 * GOLDEN_ANGLE + turn;
                    let jitter = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
                    let d = (n * y + (e1 * a.cos() + e2 * a.sin()) * (1. - y * y).sqrt() + jitter * 0.15).normalize();
                    let (a0, a2) = (p + d * (0.12 * radius), p + d * (radius * (0.9 + 0.2 * rng.next_f64())));
                    out.push(part(look + PAPPUS, shade, a0, (a0 + a2) * 0.5, a2, d.cross(&jitter), radius * 0.35));
                }
            } else {
                // The disc: a shallow cone of wedges.
                let r = f.centre_radius;
                if r > 0. {
                    let m = 10;
                    let apex = p + n * (f.dome * r);
                    for k in 0 .. m {
                        let a = turn + k as f64 * 2. * PI / m as f64;
                        let out_dir = e1 * a.cos() + e2 * a.sin();
                        let rim = p + out_dir * r;
                        out.push(part(look + DISC, shade, apex, (apex + rim) * 0.5, rim, n.cross(&out_dir), 2.5 * r * (PI / m as f64).tan()));
                    }
                }
                // Rings of petals, the inner ones shorter and more upright.
                let per = (f.petals / f.layers).max(1);
                for l in 0 .. f.layers {
                    let lf = l as f64 / f.layers as f64;
                    let cup = (f.cup + f.layer_cup * l as f64).min(PI * 0.48);
                    let length = f.petal_length * (1. - 0.5 * lf);
                    let r0 = r * (0.85 - 0.6 * lf).max(0.1);
                    let offset = turn + l as f64 * GOLDEN_ANGLE;
                    for j in 0 .. per {
                        let a = offset + (j as f64 + 0.3 * (rng.next_f64() - 0.5)) * 2. * PI / per as f64;
                        let radial = e1 * a.cos() + e2 * a.sin();
                        let d = radial * cup.cos() + n * cup.sin();
                        let len = length * (0.85 + 0.3 * rng.next_f64());
                        let a0 = p + radial * r0 + n * (0.0004 * l as f64);
                        let a2 = a0 + d * len + n * (f.curl * len);
                        out.push(part(look + PETAL, shade, a0, a0 + d * (0.55 * len), a2, n.cross(&radial), f.petal_width * (0.8 + 0.4 * rng.next_f64())));
                    }
                }
            }
            out.push(stem);
        }
    }

    /// A fern: fronds on arching stems, with pairs of pinnae along the leafy part.
    #[allow(clippy::too_many_arguments)]
    fn fern(&self, sp: &Species, f: &Fern, look: usize, centre: Vector3<f64>, patch: f64, rng: &mut Pcg32, out: &mut Vec<Blade>) {
        let c = self.conditions;
        let fronds = (f.fronds * (0.5 + rng.next_f64())).round() as usize;
        for _ in 0 .. fronds {
            let Some((x, z)) = self.in_disc(centre, sp.clump_radius, rng) else { continue };
            // The stem rises and arches over, so the leafy blade is held nearly level, facing
            // the light: a curve from the root, through a point above it, to a tip out to the
            // side and lower. Wind pushes the tip.
            let up = self.up(sp, x, z);
            let shade = self.shade(self.dryness(sp, patch, rng).1, rng);
            let mut height = sp.height * (1. + sp.height_variation * (2. * rng.next_f64() - 1.))
                * (0.55 + 0.45 * patch) * (1. - 0.35 * c.trampled);
            if let Some(m) = c.mow_height { height = height.min(m) }
            let a = rng.next_f64() * 2. * PI;
            let away = Vector3::new(x - centre.x, 0., z - centre.z);
            let mut dir = Vector3::new(a.cos(), 0., a.sin());
            if away.norm() > 1e-9 { dir = (dir + away.normalize()).normalize() }
            let v0 = Vector3::new(x, self.grid.height(x, z), z) - up * 0.01;
            let v1 = v0 + up * (0.95 * height);
            let mut v2 = v0 + up * (height * (0.5 + 0.2 * rng.next_f64())) + dir * (height * (0.55 + 0.2 * rng.next_f64()))
                + c.wind * (0.3 * height);
            v2.y = v2.y.max(self.grid.height(v2.x, v2.z) + 0.05 * height);
            let stem = part(look + FROND_STEM, shade, v0, v1, v2, dir.cross(&up), sp.width);
            let size = height / sp.height;
            let n = f.pinnae.max(1);
            for k in 0 .. n {
                let u = (k as f64 + 0.5) / n as f64;
                let s = f.blade_start + (1. - f.blade_start) * u;
                let (p, t) = (stem.point(s), stem.tangent(s).normalize());
                // Across the frond, level, and the frond's face.
                let across = t.cross(&Vector3::new(0., 1., 0.));
                let across = if across.norm() < 1e-6 { frame(t).0 } else { across.normalize() };
                let up = across.cross(&t);
                let length = f.pinna_length * size * (1. - u).powf(0.8).max(0.08) * (0.85 + 0.3 * rng.next_f64());
                let width = f.pinna_width * size * (1. - 0.6 * u);
                for side in [-1., 1.] {
                    let d = across * (side * f.sweep.cos()) + t * f.sweep.sin();
                    let v2 = p + d * length - up * (f.droop * length);
                    let v1 = p + d * (0.5 * length) + up * (0.05 * length);
                    out.push(part(look + PINNA, shade, p, v1, v2, d.cross(&up), width));
                }
            }
            out.push(stem);
        }
    }

    /// A clover plant: leaves of three leaflets on stalks, spread around it, and flower heads.
    #[allow(clippy::too_many_arguments)]
    fn clover(&self, sp: &Species, cl: &Clover, look: usize, centre: Vector3<f64>, patch: f64, rng: &mut Pcg32, out: &mut Vec<Blade>) {
        for _ in 0 .. cl.leaves {
            let Some((x, z)) = self.in_disc(centre, sp.clump_radius, rng) else { continue };
            let up = self.up(sp, x, z);
            let flower = rng.next_f64() < cl.flower_fraction;
            let stem = self.blade(if flower { &cl.flower_stem } else { &cl.stem }, look + PETIOLE, centre, x, z, patch, rng);
            let (p, t, shade) = (stem.point(1.), stem.tangent(1.).normalize(), stem.shade);
            if flower {
                // A ball of florets; the lowest have withered and hang down.
                let n = (t + up * 0.5).normalize();
                let (e1, e2) = frame(n);
                let radius = cl.flower_radius * (0.85 + 0.3 * rng.next_f64());
                let hub = p + n * (0.6 * radius);
                let turn = rng.next_f64() * 2. * PI;
                for k in 0 .. cl.florets {
                    let y = 1. - 2. * (k as f64 + 0.5) / cl.florets as f64;
                    if y < -0.85 { continue }
                    let a = k as f64 * GOLDEN_ANGLE + turn;
                    let mut d = n * y + (e1 * a.cos() + e2 * a.sin()) * (1. - y * y).sqrt();
                    let wilted = (1. - y) * 0.5 > 1. - cl.wilted;
                    if wilted { d = (d - n * 0.6).normalize() }
                    let (a0, a2) = (hub + d * (0.2 * radius), hub + d * radius);
                    let side = d.cross(&Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5));
                    out.push(part(look + if wilted { WILTED } else { FLORET }, shade, a0, (a0 + a2) * 0.5 + n * (0.05 * radius), a2, side, radius * 0.4));
                }
            } else {
                // Three leaflets (and rarely four), spread nearly flat to the light.
                let n = (t * 0.3 + up).normalize();
                let (e1, e2) = frame(n);
                let turn = rng.next_f64() * 2. * PI;
                let leaflets = if rng.next_f64() < 0.002 { 4 } else { 3 };
                for k in 0 .. leaflets {
                    let a = turn + k as f64 * 2. * PI / leaflets as f64;
                    let radial = e1 * a.cos() + e2 * a.sin();
                    let rise = cl.leaf_angle * (0.5 + rng.next_f64());
                    let d = radial * rise.cos() + n * rise.sin();
                    let l = cl.leaflet_length * (0.85 + 0.3 * rng.next_f64()) * (0.6 + 0.4 * patch);
                    let v1 = p + d * (0.5 * l) + n * (0.08 * l);
                    let v2 = p + d * l - n * (0.08 * l);
                    out.push(part(look + LEAFLET, shade, p, v1, v2, n.cross(&radial), cl.leaflet_width * (l / cl.leaflet_length)));
                }
            }
            out.push(stem);
        }
    }

    /// Whether a plant is dead, and how dry it is.
    fn dryness(&self, sp: &Species, patch: f64, rng: &mut Pcg32) -> (bool, f64) {
        let c = self.conditions;
        let dead = rng.next_f64() < c.dead_fraction;
        let dryness = if dead { 1. } else {
            (sp.dryness + c.dryness + (1. - patch) * 0.5 * c.patchiness + (rng.next_f64() - 0.5) * 0.25).clamp(0., 1.)
        };
        (dead, dryness)
    }

    /// The palette entry for this dryness, and a random variation.
    fn shade(&self, dryness: f64, rng: &mut Pcg32) -> u8 {
        let level = ((dryness * (DRY_LEVELS - 1) as f64).round() as usize).min(DRY_LEVELS - 1);
        let jitter = ((rng.next_f64() * JITTER_LEVELS as f64) as usize).min(JITTER_LEVELS - 1);
        (level * JITTER_LEVELS + jitter) as u8
    }

    #[allow(clippy::too_many_arguments)]
    fn blade(&self, sp: &Species, look: usize, centre: Vector3<f64>, x: f64, z: f64, patch: f64, rng: &mut Pcg32) -> Blade {
        let c = self.conditions;
        let ground = self.grid.height(x, z);
        let up = self.up(sp, x, z);
        let (dead, dryness) = self.dryness(sp, patch, rng);

        let mut height = sp.height * (1. + sp.height_variation * (2. * rng.next_f64() - 1.))
            * (0.55 + 0.45 * patch) * (1. - 0.35 * c.trampled);
        let mut cut = false;
        if let Some(m) = c.mow_height && height > m {
            height = m * (1. + 0.1 * (rng.next_f64() - 0.5));
            cut = true;
        }

        // Which way the blade leans: random, fanned out from the tuft's centre, and along mowing
        // stripes.
        let a = rng.next_f64() * 2. * PI;
        let mut facing = Vector3::new(a.cos(), 0., a.sin());
        let out = Vector3::new(x - centre.x, 0., z - centre.z);
        let out_frac = if sp.clump_radius > 0. { (out.norm() / sp.clump_radius).min(1.) } else { 0. };
        if out.norm() > 1e-9 {
            facing = facing * (1. - sp.clump_spread) + out.normalize() * sp.clump_spread;
        }
        let mut lean = rng.next_f64() * sp.lean + sp.clump_spread * 0.6 * out_frac + c.trampled * rng.next_f64() * 1.2;
        if c.stripe_width > 0. {
            let along = c.stripe_direction;
            let across = Vector3::new(-along.z, 0., along.x);
            let band = ((x * across.x + z * across.z) / c.stripe_width).floor() as i64;
            let sign = if band.rem_euclid(2) == 0 { 1. } else { -1. };
            facing = facing * 0.2 + along * sign;
            lean += 0.6;
        }
        if facing.norm() < 1e-9 { facing = Vector3::new(1., 0., 0.) }
        let facing = facing.normalize();
        let lean = lean.min(PI * 0.45);

        // Droop under its own weight; wind, varying with gusts.
        let droop = (1. - sp.stiffness) * 0.5 * if dead { 1.6 } else { 1. } * (1. - 0.5 * f64::from(cut));
        let gust = if c.wind.norm() > 0. {
            1. + c.gustiness * (2. * field_noise(&self.noise, x, z, c.gust_scale, self.seed as f64 * 0.1 + 3.) - 1.)
        } else { 0. };
        let wind = c.wind * (gust * (1. - 0.6 * sp.stiffness) * 1.2);

        // Tip pushed by lean, droop and wind, then the length restored (Jahrmann and Wimmer).
        // The pushes add as vectors, so wind against the lean straightens a blade; their sum
        // gives the angle of the tip from the vertical.
        let v0 = Vector3::new(x, ground, z) - up * (0.01 * height + 0.002);
        let push = facing * (lean + droop) + wind;
        let angle = push.norm().min(1.75);
        let towards = if push.norm() > 1e-9 { push.normalize() } else { facing };
        let mut v2 = v0 + (up * angle.cos() + towards * angle.sin()) * height;
        let above = (v2 - v0).dot(&up);
        if above < 0.02 * height { v2 += up * (0.02 * height - above) }
        let proj = ((v2 - v0) - up * (v2 - v0).dot(&up)).norm() / height;
        let mut v1 = v0 + up * height * (1. - proj).max(0.05 * proj.max(1.));
        let l0 = (v2 - v0).norm();
        let l1 = (v1 - v0).norm() + (v2 - v1).norm();
        let ratio = height / ((2. * l0 + l1) / 3.);
        v1 = v0 + (v1 - v0) * ratio;
        v2 = v1 + (v2 - v1) * ratio;
        // On a slope, a tip bent uphill could still end up below the ground beneath it.
        let tip_ground = self.grid.height(v2.x, v2.z) + 0.01 * height;
        if v2.y < tip_ground { v2.y = tip_ground }

        let side = up.cross(&facing);
        let side = if side.norm() < 1e-9 { Vector3::new(0., 0., 1.) } else { side.normalize() };
        let width = sp.width * (0.85 + 0.3 * rng.next_f64()) * if dead { 0.8 } else { 1. };
        let twist = sp.twist * (2. * rng.next_f64() - 1.);

        let pad = Vector3::repeat(width * 0.5 + 1e-5);
        let lo = v0.inf(&v1).inf(&v2) - pad;
        let hi = v0.sup(&v1).sup(&v2) + pad;
        Blade {
            v: [f3(v0), f3(v1), f3(v2)],
            side: f3(side),
            facing: f3(facing),
            width: width as f32,
            twist: twist as f32,
            lo: f3(lo), hi: f3(hi),
            look: look as u8,
            shade: self.shade(dryness, rng),
            cut,
        }
    }
}

/// Two unit vectors perpendicular to `n` and each other.
fn frame(n: Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let other = if n.x.abs() < 0.9 { Vector3::new(1., 0., 0.) } else { Vector3::new(0., 0., 1.) };
    let e1 = n.cross(&other).normalize();
    (e1, n.cross(&e1))
}

/// A straight-sided part of a plant - a petal, a leaflet, a spikelet - as a ribbon along the
/// Bézier curve v0, v1, v2, its face turned as near as it can be across `side`.
fn part(look: usize, shade: u8, v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, side: Vector3<f64>, width: f64) -> Blade {
    let chord = v2 - v0;
    let t = if chord.norm() > 1e-12 { chord.normalize() } else { Vector3::new(0., 1., 0.) };
    let mut side = side - t * t.dot(&side);
    if side.norm() < 1e-9 { side = frame(t).0 }
    let side = side.normalize();
    let facing = t.cross(&side);
    let pad = Vector3::repeat(width * 0.5 + 1e-5);
    Blade {
        v: [f3(v0), f3(v1), f3(v2)],
        side: f3(side),
        facing: f3(facing),
        width: width as f32,
        twist: 0.,
        lo: f3(v0.inf(&v1).inf(&v2) - pad),
        hi: f3(v0.sup(&v1).sup(&v2) + pad),
        look: look as u8,
        shade,
        cut: false,
    }
}

/// The blades of a field, bucketed by cell.
pub struct GrassField {
    blades: Vec<Blade>,
    species: Vec<Species>,
    /// The looks of every species' parts, in order.
    looks: Vec<Look>,
    cells: Grid2,
    /// Blades overlapping each cell are `items[start[i] .. start[i + 1]]`.
    start: Vec<u32>,
    items: Vec<u32>,
    /// The heights each cell's blades span.
    ranges: Vec<(f32, f32)>,
    bounds: BBox,
}

impl GrassField {
    /// Grow a field of `density` blades per square metre over [min, max] of the heightmap.
    pub fn new(grid: &HeightGrid, species: Vec<Species>, conditions: &Conditions, density: f64, seed: u64, min: (f64, f64), max: (f64, f64)) -> GrassField {
        assert!(!species.is_empty(), "grass needs at least one species");
        assert!(species.len() <= 256, "grass can have at most 256 species");
        let total: f64 = species.iter().map(|s| s.weight).sum();
        assert!(total > 0., "grass species weights must not all be 0");
        let per_clump: f64 = species.iter().map(|s| s.weight * s.per_tuft()).sum::<f64>() / total;
        let mut first_look = vec![];
        let mut looks = vec![];
        for sp in &species {
            first_look.push(looks.len());
            looks.extend(sp.looks.iter().cloned());
        }
        let litter = Species::from_json(&json!("litter"));
        let litter_look = looks.len();
        looks.extend(litter.looks.iter().cloned());
        assert!(looks.len() <= 256, "grass can have at most 256 looks (about 50 species of flowers)");
        let gmax = grid.max();
        let min = (min.0.max(grid.min.0), min.1.max(grid.min.1));
        let max = (max.0.min(gmax.0), max.1.min(gmax.1));
        let generator = Generator {
            grid, species: &species, conditions, noise: PerlinNoise::new(), seed,
            min, max, clump_density: density.max(0.) / per_clump, first_look: &first_look,
            litter: &litter, litter_look,
        };
        let tiles_x = ((max.0 - min.0) / TILE).ceil().max(0.) as usize;
        let tiles_z = ((max.1 - min.1) / TILE).ceil().max(0.) as usize;
        let blades: Vec<Blade> = (0 .. tiles_x * tiles_z).into_par_iter()
            .map(|i| generator.tile(i % tiles_x, i / tiles_x))
            .collect::<Vec<_>>()
            .into_iter().flatten().collect();
        GrassField::bucket(blades, species, looks, density)
    }

    fn bucket(blades: Vec<Blade>, species: Vec<Species>, looks: Vec<Look>, density: f64) -> GrassField {
        let mut bounds = BBox::empty();
        let mut reach = 0.;
        for b in &blades {
            bounds = bounds.union(&BBox::new(v3(b.lo), v3(b.hi)));
            reach += ((b.hi[0] - b.lo[0]) + (b.hi[2] - b.lo[2])) as f64 * 0.5;
        }
        if blades.is_empty() {
            let cells = Grid2 { min: (0., 0.), cell: 1., nx: 1, nz: 1 };
            return GrassField { blades, species, looks, cells, start: vec![0, 0], items: vec![], ranges: vec![(0., 0.)], bounds };
        }
        reach /= blades.len() as f64;
        let size = bounds.size();
        // Cells small enough to hold a few blades, but not so small that every blade spans many.
        // Flowers and seed heads add parts beyond the blades `density` counts.
        let density = density.max(blades.len() as f64 / (size.x * size.z).max(1e-9));
        let mut cell = (BLADES_PER_CELL / density.max(1e-9)).sqrt().max(reach * 0.75);
        // At most about 4 million cells.
        cell = cell.max((size.x * size.z / 4e6).sqrt()).max(1e-4);
        let nx = ((size.x / cell).ceil() as usize).max(1);
        let nz = ((size.z / cell).ceil() as usize).max(1);
        let cells = Grid2 { min: (bounds.min.x, bounds.min.z), cell, nx, nz };

        let span = |b: &Blade| {
            let f = |v: f32, lo: f64, n: usize| (((v as f64 - lo) / cell).floor().max(0.) as usize).min(n - 1);
            (f(b.lo[0], cells.min.0, nx), f(b.hi[0], cells.min.0, nx), f(b.lo[2], cells.min.1, nz), f(b.hi[2], cells.min.1, nz))
        };
        let mut counts = vec![0u32; nx * nz];
        for b in &blades {
            let (x0, x1, z0, z1) = span(b);
            for iz in z0 ..= z1 { for ix in x0 ..= x1 { counts[iz * nx + ix] += 1 } }
        }
        let mut start = Vec::with_capacity(nx * nz + 1);
        let mut acc = 0u32;
        start.push(0);
        for c in &counts { acc += c; start.push(acc) }
        let mut fill: Vec<u32> = start[.. nx * nz].to_vec();
        let mut items = vec![0u32; acc as usize];
        let mut ranges = vec![(f32::INFINITY, f32::NEG_INFINITY); nx * nz];
        for (i, b) in blades.iter().enumerate() {
            let (x0, x1, z0, z1) = span(b);
            for iz in z0 ..= z1 {
                for ix in x0 ..= x1 {
                    let c = iz * nx + ix;
                    items[fill[c] as usize] = i as u32;
                    fill[c] += 1;
                    ranges[c] = (ranges[c].0.min(b.lo[1]), ranges[c].1.max(b.hi[1]));
                }
            }
        }
        GrassField { blades, species, looks, cells, start, items, ranges, bounds }
    }

    pub fn len(&self) -> usize {
        self.blades.len()
    }

    fn cell_items(&self, c: usize) -> &[u32] {
        &self.items[self.start[c] as usize .. self.start[c + 1] as usize]
    }

    /// Nearest hit: t (multiples of the ray direction), the blade, and s along it.
    fn nearest(&self, r: &Ray) -> Option<(f64, usize, f64)> {
        if self.blades.is_empty() { return None }
        let invrd = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        let mut best: Option<(f64, usize, f64)> = None;
        walk_grid(&self.cells, self.bounds.min.y, self.bounds.max.y, r, |ix, iz, t0, t1| {
            let c = iz * self.cells.nx + ix;
            let (lo, hi) = self.ranges[c];
            if !ray_spans(r, t0, t1, lo as f64, hi as f64) {
                return false;
            }
            // Allow for rounding at cell boundaries: a hit just outside this cell is also in
            // the next, so accepting it here can't skip a nearer one.
            let eps = 1e-9 * (1. + t1.abs());
            for &i in self.cell_items(c) {
                let b = &self.blades[i as usize];
                let limit = best.map_or(t1 + eps, |x| x.0);
                if !b.bounds_hit(r, &invrd, t0 - eps, limit) { continue }
                if let Some((t, s)) = b.intersect(&self.looks[b.look as usize], r, t0 - eps, limit) {
                    best = Some((t, i as usize, s));
                }
            }
            best.is_some()
        });
        best
    }

    /// The blade `p` is on, and how far along it.
    fn locate(&self, p: Vector3<f64>) -> Option<(usize, f64)> {
        let g = &self.cells;
        let ix = (p.x - g.min.0) / g.cell;
        let iz = (p.z - g.min.1) / g.cell;
        if ix < 0. || iz < 0. || ix >= g.nx as f64 || iz >= g.nz as f64 { return None }
        let c = iz as usize * g.nx + ix as usize;
        let tolerance = 1e-3;
        let mut best: Option<(f64, usize, f64)> = None;
        for &i in self.cell_items(c) {
            let b = &self.blades[i as usize];
            let inside = (0 .. 3).all(|k| p[k] >= b.lo[k] as f64 - tolerance && p[k] <= b.hi[k] as f64 + tolerance);
            if !inside { continue }
            if let Some((d, s)) = b.locate(&self.looks[b.look as usize], p, tolerance)
                && best.is_none_or(|x| d < x.0) {
                best = Some((d, i as usize, s));
            }
        }
        best.map(|(_, i, s)| (i, s))
    }
}

pub struct GrassGeometry {
    pub field: Arc<GrassField>,
}

impl Geometry for GrassGeometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let (t, i, s) = self.field.nearest(r)?;
        Some(RawIntersection { part: 0,
            face: 0,
            dist: t * r.rd.norm(),
            point: r.ro + r.rd * t,
            normal: self.field.blades[i].normal_at(s),
        })
    }

    fn bounds(&self) -> BBox {
        self.field.bounds
    }

    fn primitives(&self) -> u64 {
        self.field.len() as u64
    }
}

/// Materials for every look, dryness, per-blade variation and position along the blade.
pub struct GrassMedium {
    pub field: Arc<GrassField>,
    materials: Vec<GrassMaterial>,
}

fn palette_index(look: usize, shade: usize, s: f64) -> usize {
    let h = ((s * HEIGHT_LEVELS as f64) as usize).min(HEIGHT_LEVELS - 1);
    (look * DRY_LEVELS * JITTER_LEVELS + shade) * HEIGHT_LEVELS + h
}

fn mix(a: Color, b: Color, t: f64) -> Color {
    Color { rgb: a.rgb * (1. - t) + b.rgb * t }
}

impl GrassMedium {
    pub fn new(field: Arc<GrassField>, conditions: &Conditions) -> GrassMedium {
        let mut materials = Vec::with_capacity(field.looks.len() * DRY_LEVELS * JITTER_LEVELS * HEIGHT_LEVELS);
        let ice = Color::new(0.75, 0.8, 0.85);
        for sp in &field.looks {
            for level in 0 .. DRY_LEVELS {
                let dry = level as f64 / (DRY_LEVELS - 1) as f64 * sp.dry_response;
                for j in 0 .. JITTER_LEVELS {
                    let jitter = (j as f64 / (JITTER_LEVELS - 1) as f64 - 0.5) * sp.color_variation;
                    for h in 0 .. HEIGHT_LEVELS {
                        let s = (h as f64 + 0.5) / HEIGHT_LEVELS as f64;
                        let mut green = mix(sp.base_color, sp.tip_color, s.powf(0.8));
                        if let Some(band) = sp.band {
                            green = mix(green, band, (-((s - 0.5) / 0.15).powi(2)).exp());
                        }
                        let straw = sp.dry_color * (0.8 + 0.35 * s);
                        let mut c = mix(green, straw, dry);
                        // Per-blade variation: brightness, and a slight shift toward yellow.
                        c = Color::new(c.rgb.x * (1. + 0.45 * jitter), c.rgb.y * (1. + 0.3 * jitter), c.rgb.z * (1. + 0.1 * jitter));
                        c = mix(c, ice, conditions.frost * s.powf(1.5));
                        c = c * (1. - 0.25 * conditions.wetness);
                        let translucency = sp.translucency * (1. - 0.6 * dry) * (1. - 0.7 * conditions.frost * s);
                        let reflect = c * (1. - 0.35 * translucency);
                        // Light through a leaf is more saturated, and yellower.
                        let transmit = if sp.leafy { Color::new(c.rgb.x * 1.1, c.rgb.y * 1.15, c.rgb.z * 0.6) } else { c } * translucency;
                        materials.push(GrassMaterial {
                            reflect,
                            transmit,
                            sheen: (sp.sheen * (1. - 0.5 * dry) * (1. - conditions.frost) + 0.7 * conditions.wetness).min(1.),
                            roughness: 0.3 - 0.2 * conditions.wetness,
                        });
                    }
                }
            }
        }
        GrassMedium { field, materials }
    }
}

impl Medium for GrassMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        let i = match self.field.locate(pt) {
            Some((b, s)) => {
                let blade = &self.field.blades[b];
                palette_index(blade.look as usize, blade.shade as usize, s)
            },
            // Shouldn't happen: a mid-height, mid-dryness blade of the first look.
            None => palette_index(0, (DRY_LEVELS / 4) * JITTER_LEVELS + 1, 0.5),
        };
        &self.materials[i]
    }
}

/// Parse a grass object (see the module documentation) into its geometry, and the conditions
/// for its medium.
pub fn create_grass(o: &Value) -> (GrassGeometry, Conditions) {
    let grid = HeightGrid::from_json(&o["heightmap"]);
    let species: Vec<Species> = match &o["species"] {
        Value::Array(list) => list.iter().map(Species::from_json).collect(),
        Value::Null => vec![Species::from_json(&Value::Null)],
        v => vec![Species::from_json(v)],
    };
    let conditions = Conditions::from_json(&o["conditions"]);
    let corner = |k: &str, def: (f64, f64)| match o.get(k) {
        Some(v) => (v[0].as_f64().unwrap(), v[1].as_f64().unwrap()),
        None => def,
    };
    let field = GrassField::new(
        &grid, species, &conditions,
        SceneFile::parse_number(&o["density"], 600.),
        SceneFile::parse_int(&o["seed"], 0) as u64,
        corner("min", grid.min), corner("max", grid.max()));
    (GrassGeometry { field: Arc::new(field) }, conditions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedural::heightmap::{Flat, HeightSource};

    struct Hill;
    impl HeightSource for Hill {
        fn height(&self, x: f64, z: f64) -> f64 { 2. - 0.02 * (x * x + z * z) }
    }

    fn field(conditions: Value, species: Value, density: f64) -> (HeightGrid, GrassField) {
        let grid = HeightGrid::sample(&Hill, (-5., -5.), (5., 5.), 0.25);
        let species: Vec<Species> = species.as_array().unwrap().iter().map(Species::from_json).collect();
        let c = Conditions::from_json(&conditions);
        let f = GrassField::new(&grid, species, &c, density, 7, (-4., -4.), (4., 4.));
        (grid, f)
    }

    #[test]
    fn test_field_is_reproducible_and_has_expected_density() {
        let (_, a) = field(json!("lush"), json!(["lawn"]), 200.);
        let (_, b) = field(json!("lush"), json!(["lawn"]), 200.);
        assert_eq!(a.len(), b.len());
        assert_eq!(a.blades[123].v, b.blades[123].v);
        // 64 m^2 at 200 per m^2, with lush conditions thinning nothing.
        let expected = 64. * 200.;
        assert!((a.len() as f64 - expected).abs() < expected * 0.05, "{} blades", a.len());
    }

    #[test]
    fn test_blades_keep_their_length_and_grow_from_the_ground() {
        let (grid, f) = field(json!({ "preset": "windy" }), json!(["meadow", "tall"]), 50.);
        for b in f.blades.iter().step_by(37) {
            // Grass without seed heads has one look per species.
            let sp = &f.species[b.look as usize];
            let root = v3(b.v[0]);
            assert!((root.y - grid.height(root.x, root.z)) < 0.0, "root above ground: {}", root);
            // Arc length of the Bézier, by summing chords.
            let len: f64 = (0 .. 64).map(|i| (b.point((i + 1) as f64 / 64.) - b.point(i as f64 / 64.)).norm()).sum();
            let max_height = sp.height * (1. + sp.height_variation);
            assert!(len > 0. && len < max_height * 1.05, "length {} for species height {}", len, sp.height);
            // The tip stays above the ground.
            let tip = b.point(1.);
            assert!(tip.y > grid.height(tip.x, tip.z) - 0.01 * len - 0.003, "tip {} ground {} root {} mid {}", tip, grid.height(tip.x, tip.z), root, v3(b.v[1]));
        }
    }

    #[test]
    fn test_wind_bends_blades_downwind() {
        let calm = field(json!({ "wind": { "strength": 0 } }), json!([{ "preset": "meadow", "lean": 0, "clump_size": 1 }]), 30.).1;
        let windy = field(json!({ "wind": { "strength": 1, "direction": [1, 0], "gustiness": 0 } }),
            json!([{ "preset": "meadow", "lean": 0, "clump_size": 1 }]), 30.).1;
        let mean_dx = |f: &GrassField| f.blades.iter().map(|b| (b.v[2][0] - b.v[0][0]) as f64).sum::<f64>() / f.len() as f64;
        assert!(mean_dx(&calm).abs() < 0.02, "{}", mean_dx(&calm));
        assert!(mean_dx(&windy) > 0.1, "{}", mean_dx(&windy));
    }

    #[test]
    fn test_mowing_cuts_blades() {
        let (_, f) = field(json!({ "mow_height": 0.05 }), json!(["meadow"]), 30.);
        assert!(f.blades.iter().all(|b| b.cut));
        assert!(f.blades.iter().all(|b| b.point(1.).y - b.point(0.).y < 0.06));
    }

    #[test]
    fn test_altitude_and_slope_limits() {
        let (grid, f) = field(json!({ "max_altitude": 1.5, "altitude_fade": 0.01 }), json!(["lawn"]), 50.);
        assert!(f.len() > 0);
        for b in &f.blades {
            let r = v3(b.v[0]);
            assert!(grid.height(r.x, r.z) < 1.52);
        }
        let (_, none) = field(json!({ "max_slope": 0 }), json!(["lawn"]), 50.);
        assert_eq!(none.len(), 0);
    }

    #[test]
    fn test_ray_hits_blade_and_medium_finds_it() {
        let grid = HeightGrid::sample(&Flat { base: 0. }, (-2., -2.), (2., 2.), 0.5);
        let sp = vec![Species::from_json(&json!({ "preset": "tall", "clump_size": 1 }))];
        let c = Conditions::from_json(&json!("lush"));
        let f = Arc::new(GrassField::new(&grid, sp, &c, 400., 1, (-1., -1.), (1., 1.)));
        let geometry = GrassGeometry { field: f.clone() };
        let medium = GrassMedium::new(f.clone(), &c);
        let mut hits = 0;
        for i in 0 .. 200 {
            let a = i as f64 * 0.1;
            let r = Ray { ro: Vector3::new(-3., 0.4, 0.3 * a.sin()), rd: Vector3::new(1., -0.05, 0.1 * a.cos()) };
            if let Some(h) = geometry.intersects(&r) {
                hits += 1;
                assert!((h.dist - (h.point - r.ro).norm()).abs() < 1e-9);
                let (b, s) = f.locate(h.point).expect("hit point should be on a blade");
                assert!((0. ..= 1.).contains(&s));
                // No blade is hit nearer than the reported one.
                let nearer = f.blades.iter().enumerate().filter(|(j, _)| *j != b).any(|(_, other)| {
                    other.intersect(&f.looks[other.look as usize], &r, 0., h.dist / r.rd.norm() - 1e-9).is_some()
                });
                assert!(!nearer, "missed a nearer blade on ray {}", i);
                let _ = medium.material_at(h.point);
            }
        }
        assert!(hits > 150, "only {} of 200 rays through dense grass hit", hits);
    }

    #[test]
    fn test_presets_parse() {
        for p in ["lawn", "meadow", "fescue", "tall", "tussock", "savanna", "reed", "rough", "timothy", "wild_oat", "bent", "heather", "bracken",
            "daisy", "oxeye", "dandelion", "buttercup", "clover", "red_clover"] {
            Species::from_json(&json!(p));
        }
        for p in ["lush", "summer", "drought", "autumn", "after_rain", "frost", "windy", "manicured", "trampled", "wild"] {
            Conditions::from_json(&json!(p));
        }
        let s = Species::from_json(&json!({ "preset": "reed", "height": 3 }));
        assert_eq!(s.height, 3.);
        let c = Conditions::from_json(&json!({ "preset": "windy", "wind": { "direction": [0, 1] } }));
        assert!((c.wind.z - 0.8).abs() < 1e-12, "{}", c.wind);
    }

    #[test]
    fn test_overrides_merge_into_flowers_and_seed_heads() {
        let s = Species::from_json(&json!({ "preset": "dandelion", "flower": { "petals": 12 } }));
        match &s.kind {
            // The other fields come from the dandelion, and what it doesn't set from the daisy.
            Kind::Flower(f) => assert!(f.petals == 12 && f.layers == 5 && f.pappus == 110),
            _ => panic!("a dandelion is a flower"),
        }
        let s = Species::from_json(&json!({ "preset": "meadow", "seed_head": { "style": "panicle", "fraction": 0.5 } }));
        match &s.kind {
            Kind::Grass(Some(h)) => assert!(h.panicle && h.fraction == 0.5 && h.spikelets == 16),
            _ => panic!("expected a seed head"),
        }
        assert_eq!(s.looks.len(), 3);
        assert!(matches!(Species::from_json(&json!("meadow")).kind, Kind::Grass(None)));
    }

    #[test]
    fn test_wild_plants_grow_their_parts() {
        let species = json!(["timothy", "wild_oat", "daisy", { "preset": "dandelion", "flower": { "clock_fraction": 0.5 } }, "red_clover"]);
        let (grid, f) = field(json!("lush"), species, 40.);
        // And one for litter, which lush grass hasn't.
        assert_eq!(f.looks.len(), 3 + 3 + 5 + 5 + 4 + 1);
        // Every look is used, but for the seeds of the daisy, which doesn't go to seed: each kind
        // of part has grown somewhere.
        for look in (0 .. f.looks.len() - 1).filter(|&l| l != 6 + PAPPUS) {
            assert!(f.blades.iter().any(|b| b.look as usize == look), "no parts with look {}", look);
        }
        for b in &f.blades {
            assert!((b.look as usize) < f.looks.len());
            // Everything is above the ground, apart from the roots of stems and blades.
            for k in 1 .. 3 {
                let p = v3(b.v[k]);
                assert!(p.y > grid.height(p.x, p.z) - 0.01, "{:?} below the ground", b);
            }
            for k in 0 .. 3 {
                assert!(b.lo[k] <= b.v[0][k].min(b.v[1][k]).min(b.v[2][k]) && b.hi[k] >= b.v[0][k].max(b.v[1][k]).max(b.v[2][k]));
            }
        }
    }

    #[test]
    fn test_litter_lies_on_the_ground_under_unchanged_grass() {
        let (_, bare) = field(json!("lush"), json!(["meadow"]), 100.);
        let (grid, f) = field(json!({ "preset": "lush", "litter": 200 }), json!(["meadow"]), 100.);
        let litter_look = f.looks.len() - 1;
        let (litter, living): (Vec<&Blade>, Vec<&Blade>) = f.blades.iter().partition(|b| b.look as usize == litter_look);
        assert_eq!(living.len(), bare.len());
        assert_eq!(living[100].v, bare.blades[100].v);
        // 64 m^2 at 200 per m^2.
        assert!((litter.len() as f64 - 64. * 200.).abs() < 64. * 200. * 0.05, "{} bits of litter", litter.len());
        // Most of it lies flat, its tip near the ground.
        let flat = litter.iter().filter(|b| {
            let tip = b.point(1.);
            tip.y - grid.height(tip.x, tip.z) < 0.05
        }).count();
        assert!(flat as f64 > 0.7 * litter.len() as f64, "{} of {} flat", flat, litter.len());
    }

    #[test]
    fn test_bracken_fronds_arch_over_and_hold_their_pinnae_level() {
        let (_, f) = field(json!({ "preset": "lush", "wind": { "strength": 0 } }), json!(["bracken"]), 3.);
        let stems: Vec<&Blade> = f.blades.iter().filter(|b| b.look as usize == FROND_STEM).collect();
        let pinnae: Vec<&Blade> = f.blades.iter().filter(|b| b.look as usize == PINNA).collect();
        assert!(stems.len() > 20);
        assert_eq!(pinnae.len(), stems.len() * 2 * 14);
        // The stem's tip is below the highest point of its curve.
        assert!(stems.iter().all(|b| b.point(1.).y < b.point(0.7).y));
        let steep = pinnae.iter().filter(|b| {
            let chord = v3(b.v[2]) - v3(b.v[0]);
            chord.y.abs() > 0.5 * chord.norm()
        }).count();
        assert!(steep * 10 < pinnae.len(), "{} of {} pinnae steep", steep, pinnae.len());
    }

    #[test]
    fn test_mown_grass_has_no_seed_heads_but_flowers_flower_short() {
        let (_, f) = field(json!({ "mow_height": 0.05 }), json!([{ "preset": "timothy", "seed_head": { "fraction": 1 } }, "oxeye"]), 30.);
        // Timothy's looks are 0 to 2, the oxeye's from 3: its petals are 3 + PETAL.
        assert!(!f.blades.iter().any(|b| b.look as usize == SPIKELET));
        let petals: Vec<&Blade> = f.blades.iter().filter(|b| b.look as usize == 3 + PETAL).collect();
        assert!(!petals.is_empty());
        assert!(petals.iter().all(|b| b.v[0][1] < 0.05 + 2.1), "flowers above the mower");
    }

    #[test]
    fn test_rays_find_flowers() {
        let grid = HeightGrid::sample(&Flat { base: 0. }, (-2., -2.), (2., 2.), 0.5);
        let sp = vec![Species::from_json(&json!({ "preset": "oxeye", "flower": { "count": 4 } }))];
        let c = Conditions::from_json(&json!({ "preset": "lush", "wind": { "strength": 0 } }));
        let f = Arc::new(GrassField::new(&grid, sp, &c, 30., 1, (-1., -1.), (1., 1.)));
        let geometry = GrassGeometry { field: f.clone() };
        let medium = GrassMedium::new(f.clone(), &c);
        // Look straight down onto the discs of the flowers.
        let mut heads = 0;
        for b in f.blades.iter().filter(|b| b.look as usize == DISC).step_by(10) {
            let top = v3(b.v[0]) * 0.6 + v3(b.v[2]) * 0.4;
            let r = Ray { ro: top + Vector3::new(0., 1., 0.), rd: Vector3::new(0., -1., 0.) };
            let h = geometry.intersects(&r).expect("should hit the flower");
            let own = b.intersect(&f.looks[DISC], &r, 0., 10.).expect("should hit the disc").0;
            assert!(h.dist <= own + 1e-9);
            // Unless another stem is in the way, the hit is on the head, and the medium finds it.
            if (h.dist - own).abs() > 1e-6 { continue }
            let (i, _) = f.locate(h.point).expect("hit point should be on a part");
            assert!([DISC, PETAL].contains(&(f.blades[i].look as usize)), "hit look {}", f.blades[i].look);
            let _ = medium.material_at(h.point);
            heads += 1;
        }
        assert!(heads > 5);
    }
}
