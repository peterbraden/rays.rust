//! Procedural rock surfaces.
//!
//! Rock is shaded as a solid texture (Darwyn Peachey, "Solid Texturing of Complex Surfaces",
//! SIGGRAPH 1985; Ken Perlin, "An Image Synthesizer", SIGGRAPH 1985): its colour is a function
//! of position in space rather than of surface coordinates, so it needs no parameterisation and
//! carries on through any shape cut from it, as the grain of real stone does. It is built from
//! the features that identify a stone at a glance:
//!
//! - mottling between two body colours, as mineral content varies through the rock;
//! - grains: specks of dark and light mineral, eg. the biotite and quartz in granite;
//! - strata: bands of sediment, warped by noise, in sedimentary rock;
//! - cracks: dark, recessed seams;
//! - weathering: broad variation in tone;
//! - lichen or moss on the surfaces facing the sky;
//! - wetness, which darkens the rock (the water fills its pores, so less light escapes; John
//!   Lekner and Michael C. Dorf, "Why some things are darker when wet", Applied Optics 27(7),
//!   1988) and adds a glossy film reflecting with water's Fresnel factor.
//!
//! Relief finer than the geometry is bump mapped (James F. Blinn, "Simulation of Wrinkled
//! Surfaces", SIGGRAPH 1978): the shading normal is tilted against the gradient of a height
//! field, estimated by central differences.
//!
//! Reflectances are typical of the stones each preset is named for, in linear RGB: from about
//! 0.06 for basalt to 0.6 for limestone (eg. the USGS spectral library, Kokaly et al. 2017).
//! Rock is close to Lambertian, so the base is diffuse.

use crate::material::model::LightSampling;
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::geometry::{rand, random_point_on_unit_sphere, random_point_in_unit_ball};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{direct_lighting, fresnel_reflectance, reflect};

/// The look of a kind of stone. Lengths are in units of the material's `scale`.
#[derive(Clone, Debug, PartialEq)]
pub struct RockStyle {
    /// The body colour, and the colour it is mottled with.
    pub color: Color,
    pub secondary: Color,
    /// Dark and light mineral grains, and the fraction of the surface each covers.
    pub dark_grain: Color,
    pub dark_grain_amount: f64,
    pub light_grain: Color,
    pub light_grain_amount: f64,
    pub grain_size: f64,
    /// Sedimentary layers: their spacing, colour, and strength (0 for none).
    pub strata_spacing: f64,
    pub strata_color: Color,
    pub strata_amount: f64,
    /// Depth of the bump-mapped relief, and the size of its features.
    pub bump: f64,
    pub bump_size: f64,
    /// Strength of cracks, 0 for none.
    pub cracks: f64,
}

impl RockStyle {
    pub fn preset(name: &str) -> RockStyle {
        let grey = |v: f64| Color::new(v, v, v);
        let plain = RockStyle {
            color: grey(0.3), secondary: grey(0.36),
            dark_grain: grey(0.05), dark_grain_amount: 0.,
            light_grain: grey(0.7), light_grain_amount: 0.,
            grain_size: 0.01,
            strata_spacing: 0.1, strata_color: grey(0.2), strata_amount: 0.,
            bump: 0.4, bump_size: 0.15, cracks: 0.3,
        };
        match name {
            // Coarse-grained and speckled: feldspar body, black biotite, glassy quartz.
            "granite" => RockStyle {
                color: Color::new(0.42, 0.37, 0.34), secondary: Color::new(0.52, 0.45, 0.41),
                dark_grain: Color::new(0.04, 0.04, 0.04), dark_grain_amount: 0.18,
                light_grain: Color::new(0.72, 0.70, 0.67), light_grain_amount: 0.12,
                grain_size: 0.012,
                bump: 0.35, cracks: 0.35,
                ..plain
            },
            // Dark, fine-grained volcanic rock, pitted with gas bubbles.
            "basalt" => RockStyle {
                color: Color::new(0.07, 0.07, 0.07), secondary: Color::new(0.11, 0.10, 0.095),
                dark_grain: Color::new(0.02, 0.02, 0.02), dark_grain_amount: 0.08,
                light_grain: Color::new(0.2, 0.19, 0.17), light_grain_amount: 0.03,
                grain_size: 0.008,
                bump: 0.55, bump_size: 0.1, cracks: 0.25,
                ..plain
            },
            // Layered sand, coloured by iron oxides.
            "sandstone" => RockStyle {
                color: Color::new(0.52, 0.34, 0.20), secondary: Color::new(0.62, 0.46, 0.30),
                dark_grain: Color::new(0.30, 0.18, 0.10), dark_grain_amount: 0.06,
                light_grain: Color::new(0.75, 0.62, 0.45), light_grain_amount: 0.06,
                grain_size: 0.004,
                strata_spacing: 0.08, strata_color: Color::new(0.40, 0.22, 0.12), strata_amount: 0.55,
                bump: 0.3, bump_size: 0.2, cracks: 0.15,
            },
            // Pale and chalky, with darker fossil fragments.
            "limestone" => RockStyle {
                color: Color::new(0.60, 0.57, 0.50), secondary: Color::new(0.50, 0.48, 0.42),
                dark_grain: Color::new(0.32, 0.30, 0.26), dark_grain_amount: 0.05,
                light_grain: Color::new(0.75, 0.73, 0.68), light_grain_amount: 0.08,
                grain_size: 0.02,
                strata_spacing: 0.3, strata_color: Color::new(0.45, 0.43, 0.38), strata_amount: 0.25,
                bump: 0.45, bump_size: 0.25, cracks: 0.4,
            },
            // Blue-grey, in thin sheets.
            "slate" => RockStyle {
                color: Color::new(0.13, 0.14, 0.16), secondary: Color::new(0.18, 0.18, 0.20),
                light_grain: Color::new(0.3, 0.3, 0.32), light_grain_amount: 0.02,
                grain_size: 0.005,
                strata_spacing: 0.02, strata_color: Color::new(0.10, 0.11, 0.12), strata_amount: 0.4,
                bump: 0.3, bump_size: 0.06, cracks: 0.2,
                ..plain
            },
            // Hard, pale, and nearly featureless: water-worn quartzite pebbles.
            "quartzite" => RockStyle {
                color: Color::new(0.68, 0.62, 0.56), secondary: Color::new(0.74, 0.70, 0.66),
                dark_grain: Color::new(0.45, 0.38, 0.32), dark_grain_amount: 0.03,
                light_grain: Color::new(0.85, 0.83, 0.80), light_grain_amount: 0.05,
                grain_size: 0.006,
                bump: 0.15, bump_size: 0.2, cracks: 0.05,
                ..plain
            },
            _ => panic!("Unknown rock preset '{}': expected granite, basalt, sandstone, limestone, slate or quartzite", name),
        }
    }
}

pub struct Rock {
    pub style: RockStyle,
    /// The size of the texture's features, in scene units: 1 for metre-sized features when the
    /// scene is in metres.
    pub scale: f64,
    /// Broad variation in tone, 0 for none.
    pub weathering: f64,
    /// Coverage of lichen on upward surfaces, 0 to 1, and its colour.
    pub lichen: f64,
    pub lichen_color: Color,
    /// 0 for dry, 1 for soaked.
    pub wetness: f64,
    /// Formations by elevation: (y in scene units, colour), sorted by y, the body colour
    /// blending between them; their boundaries warped up and down by up to `layer_warp`.
    pub layers: Vec<(f64, Color)>,
    pub layer_warp: f64,
    /// Colours for individual beds of the strata, each bed taking one; empty for
    /// `strata_color` alone.
    pub beds: Vec<Color>,
    /// Broad patches of another colour, as iron oxides stain and bleach sandstone: their
    /// colour and how much of the surface they take, 0 to 1.
    pub variety: f64,
    pub variety_color: Color,
    /// Desert varnish: dark streaks of manganese and iron oxide running down steep faces where
    /// water runs, 0 to 1, and its colour.
    pub varnish: f64,
    pub varnish_color: Color,
    /// Depth of vertical flutes worn into steep faces, bump mapped.
    pub flutes: f64,
    /// Cross-bedding, 0 to 1: laminae inclined within sets `cross_set` thick (in units of
    /// `scale`), each set's dipping its own way, as the foresets of the dunes that became
    /// aeolian sandstone (Navajo, De Chelly) do; drawn in place of level strata where strong.
    pub cross_bedding: f64,
    pub cross_set: f64,
    /// Pitting, 0 to 1: the small cavities of honeycomb weathering (tafoni) on steep faces,
    /// `pit_size` across (in units of `scale`), dark and recessed.
    pub pits: f64,
    pub pit_size: f64,
    /// Fractal relief for close-up detail, as POV-Ray's turbulent normals: octaves of
    /// turbulence (|noise|, creased like weathered stone) from `fractal_size` down to the size
    /// of a pixel where the surface is seen, so it holds detail however near the camera comes
    /// and doesn't alias far away. `fractal` scales its slopes; each octave's amplitude falls
    /// by 2^-`fractal_h` (1: equally steep at every scale; less: rougher in the small).
    pub fractal: f64,
    pub fractal_size: f64,
    pub fractal_h: f64,
    /// Fractures: the cell walls of cellular (Worley) noise `crackle_size` across, as
    /// POV-Ray's crackle pattern, recessed `crackle` deep and dark with grime.
    pub crackle: f64,
    pub crackle_size: f64,
    noise: SeededNoise,
    /// The fractal relief's rotations, an octave's 3 x 3 rows each (`shared::rock`'s tables).
    rotations: [f64; 144],
    /// Noise levels above which the given fraction of the surface is grain or lichen.
    dark_threshold: f64,
    light_threshold: f64,
    lichen_threshold: f64,
}

/// The fractal relief's octaves' rotations (`Rock::fractal_relief`), row by row: each octave
/// turned, so their lattices don't line up.
fn fractal_rotations() -> [f64; 144] {
    let mut out = [0.; 144];
    for k in 0 .. 16 {
        let r = crate::na::Rotation3::from_euler_angles(0.61 * k as f64, 0.93 * k as f64, 1.27 * k as f64);
        let m = r.matrix();
        for i in 0 .. 3 {
            for j in 0 .. 3 {
                out[9 * k + 3 * i + j] = m[(i, j)];
            }
        }
    }
    out
}

/// A rock's tables as `shared::rock::Rock` reads them: the permutation by index; by value the
/// fractal rotations (144), then the beds (3 each), then the layers (4 each).
struct RockTables<'a> {
    perm: &'a [usize; 512],
    rotations: &'a [f64; 144],
    beds: &'a [Color],
    layers: &'a [(f64, Color)],
}

impl shared::Tables<f64> for RockTables<'_> {
    fn value(&self, i: u32) -> f64 {
        let i = i as usize;
        if i < 144 {
            return self.rotations[i];
        }
        let j = i - 144;
        if j < 3 * self.beds.len() {
            return self.beds[j / 3].rgb[j % 3];
        }
        let l = j - 3 * self.beds.len();
        let (y, c) = &self.layers[l / 4];
        if l.is_multiple_of(4) { *y } else { c.rgb[l % 4 - 1] }
    }

    fn index(&self, i: u32) -> u32 {
        self.perm[i as usize] as u32
    }
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Where a point lies in cross-bedded sandstone (`cross_beds`).
#[derive(Clone, Copy, Debug)]
pub struct Lamina {
    /// Which lamina (whole part) and where across it (fraction), counted within its set.
    pub phase: f64,
    /// A number for the lamina, unique within the rock, for its colour and hardness.
    pub id: i64,
    /// 1 on a set's bounding surface, falling to 0 within a few percent of the set's thickness.
    pub boundary: f64,
    /// How hard the lamina is, 0 to 1: coarse, well-cemented laminae weather out as ribs, fine
    /// ones are worn back, so this is its relief on a weathered face.
    pub hardness: f64,
}

/// The width, in sets, that nine laminae in ten are wider than, crossed along any line through
/// the rock (`test_laminae_are_mostly_wider_than_the_finest`): the detail to resolve them.
pub const LAMINA_FINEST: f64 = 0.025;

/// Cross-bedded sandstone at `q` (in the rock's own units), in sets about `set` thick: the
/// foresets of the dunes that became aeolian sandstone (Navajo, De Chelly). After the
/// sedimentology (eg. Rubin, "Cross-bedding, bedforms, and paleocurrents", SEPM 1987):
/// - Sets are wedges between bounding surfaces that are near level but tilted and wavy, each
///   set truncating the tops of the laminae of the one below.
/// - Within a set, laminae dip steeply (up to the angle of repose) near its top and curve down
///   to meet its base tangentially, as a dune's avalanche face does at its toe.
/// - Each set dips its own way, as the wind and dunes did; laminae are of varying thickness,
///   with crisp edges, and differ from each other in colour and hardness.
pub fn cross_beds(noise: &SeededNoise, q: Vector3<f64>, set: f64) -> Lamina {
    let set = set.max(1e-9);
    // Bounding surfaces: level on average, tilted and wavy, so the sets are wedges.
    let tilt = 0.08 * (q.x + 0.6 * q.z) / set;
    let y = q.y / set + tilt * 0.5 + 0.35 * noise.fbm(q / (5. * set) + Vector3::new(5., 0., 0.), 2);
    let k = y.floor();
    let within = y - k;
    let hash = |salt: f64| ((k * 12.9898 + salt * 78.233).sin() * 43_758.545_3).fract().abs();
    let azimuth = hash(1.) * std::f64::consts::TAU;
    let (along, across) = ((q.x * azimuth.cos() + q.z * azimuth.sin()) / set, (-q.x * azimuth.sin() + q.z * azimuth.cos()) / set);
    // Tangential foresets: a lamina is a curve along which `along` grows as the square root of
    // the height in the set, so it is steep above and flattens towards the base; troughs bend
    // the laminae across the set.
    let steepness = 0.8 + 0.8 * hash(2.);
    // Troughs: the laminae bend into festoons, arcs a few set-thicknesses across, so a level
    // surface cutting them shows the swirls of slickrock rather than straight stripes.
    let width = 1.2 + 1.2 * hash(3.);
    let t = (across / width).rem_euclid(1.) - 0.5;
    let trough = 2.5 * width * t * t + 0.4 * noise.fbm(Vector3::new(across * 0.3, k * 3.3, along * 0.3), 2);
    let coordinate = along + trough - steepness * within.max(0.).sqrt();
    // Laminae of varying thickness: the coordinate stretched by noise before it is counted.
    let spacing = 0.07 + 0.05 * hash(4.);
    let wobble = 0.5 * noise.fbm(Vector3::new(coordinate * 3., k * 7.1, across * 0.5), 2);
    let phase = coordinate / spacing + wobble;
    let n = phase.floor();
    let id = (n as i64).wrapping_mul(7919).wrapping_add((k as i64).wrapping_mul(104_729));
    let h = ((id as f64 * 0.618_033_988_7).fract() + 1.).fract();
    // Each lamina's hardness from its number; a thin, softer parting at its edges.
    let f = phase - n;
    let parting = smoothstep(0.2, 0., f.min(1. - f));
    let hardness = (0.35 + 0.65 * h) * (1. - 0.5 * parting);
    let boundary = smoothstep(0.05, 0., within.min(1. - within));
    Lamina { phase, id, boundary, hardness }
}

/// Refractive index of water.
const WATER: f64 = 1.33;

impl Rock {
    pub fn new(style: RockStyle, scale: f64, seed: u64) -> Rock {
        let noise = SeededNoise::new(seed);
        let quantile = |fraction: f64, sample: &dyn Fn(Vector3<f64>) -> f64| {
            // Noise levels aren't uniformly distributed, so find the level exceeded by the
            // given fraction of points from a sample.
            let mut rng = crate::geometry::Pcg32::new(seed, 7);
            let mut v: Vec<f64> = (0 .. 4096).map(|_| {
                sample(Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 50.)
            }).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if fraction <= 0. { return f64::INFINITY }
            v[((1. - fraction.min(1.)) * (v.len() - 1) as f64) as usize]
        };
        let dark_threshold = quantile(style.dark_grain_amount, &|p| noise.noise(p));
        let light_threshold = quantile(style.light_grain_amount, &|p| noise.noise(p + Vector3::new(31.4, 0., 0.)));
        Rock {
            style, scale, weathering: 0.3, lichen: 0., lichen_color: Color::new(0.38, 0.40, 0.26), wetness: 0.,
            layers: vec![], layer_warp: 0., beds: vec![], variety: 0., variety_color: Color::new(0.7, 0.55, 0.4),
            varnish: 0., varnish_color: Color::new(0.12, 0.07, 0.05), flutes: 0.,
            cross_bedding: 0., cross_set: 1., pits: 0., pit_size: 0.05,
            fractal: 0., fractal_size: 1., fractal_h: 1., crackle: 0., crackle_size: 1.,
            noise, dark_threshold, light_threshold, lichen_threshold: 0., rotations: fractal_rotations(),
        }
    }

    /// The recipe, as the shared code takes it (`shared::rock::Rock`, where the relief and the
    /// colour are: the GPU's kernels compile the same functions at f32), with its tables laid
    /// out as `RockTables` gives them.
    fn recipe(&self) -> shared::rock::Rock<f64> {
        let rgb = |c: &Color| (c.rgb.x, c.rgb.y, c.rgb.z);
        let st = &self.style;
        let r = self.noise.rotation_rows();
        let o = self.noise.offset();
        shared::rock::Rock {
            color: rgb(&st.color), secondary: rgb(&st.secondary), dark_grain: rgb(&st.dark_grain), light_grain: rgb(&st.light_grain),
            grain_size: st.grain_size, strata_spacing: st.strata_spacing, strata_color: rgb(&st.strata_color), strata_amount: st.strata_amount,
            bump: st.bump, bump_size: st.bump_size, cracks: st.cracks, scale: self.scale, weathering: self.weathering,
            lichen: self.lichen, lichen_color: rgb(&self.lichen_color), wetness: self.wetness, layer_warp: self.layer_warp,
            variety: self.variety, variety_color: rgb(&self.variety_color), varnish: self.varnish, varnish_color: rgb(&self.varnish_color),
            flutes: self.flutes, cross_bedding: self.cross_bedding, pits: self.pits, pit_size: self.pit_size,
            fractal: self.fractal, fractal_size: self.fractal_size, fractal_h: self.fractal_h, crackle: self.crackle, crackle_size: self.crackle_size,
            offset: (o.x, o.y, o.z), rotation: r,
            dark_threshold: self.dark_threshold, light_threshold: self.light_threshold, lichen_threshold: self.lichen_threshold,
            dark_grains: self.dark_threshold.is_finite(), light_grains: self.light_threshold.is_finite(),
            perm: 0, rotations: 0, beds: 144, bed_count: self.beds.len() as u32,
            layers: 144 + 3 * self.beds.len() as u32, layer_count: self.layers.len() as u32,
        }
    }

    /// The rock as data for the kernels (`pathtrace::load_rock`, which has the layout): its
    /// recipe's numbers, then its tables (the permutation, the fractal rotations, the beds and
    /// the layers).
    pub fn export(&self) -> Vec<f32> {
        let r = self.recipe();
        let mut out: Vec<f32> = Vec::new();
        for c in [r.color, r.secondary, r.dark_grain, r.light_grain, r.strata_color, r.lichen_color, r.variety_color, r.varnish_color] {
            out.extend([c.0 as f32, c.1 as f32, c.2 as f32]);
        }
        out.extend([r.grain_size, r.strata_spacing, r.strata_amount, r.bump, r.bump_size, r.cracks, r.scale, r.weathering, r.lichen,
            r.wetness, r.layer_warp, r.variety, r.varnish, r.flutes, r.cross_bedding, r.pits, r.pit_size, r.fractal, r.fractal_size,
            r.fractal_h, r.crackle, r.crackle_size, r.offset.0, r.offset.1, r.offset.2].iter().map(|&v| v as f32));
        out.extend(r.rotation.iter().map(|&v| v as f32));
        let finite = |v: f64| if v.is_finite() { v as f32 } else { 0. };
        out.extend([finite(r.dark_threshold), finite(r.light_threshold), finite(r.lichen_threshold),
            r.dark_grains as u32 as f32, r.light_grains as u32 as f32, r.bed_count as f32, r.layer_count as f32]);
        out.extend(self.noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.rotations.iter().map(|&v| v as f32));
        for c in &self.beds {
            out.extend([c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32]);
        }
        for (y, c) in &self.layers {
            out.extend([*y as f32, c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32]);
        }
        out
    }

    fn tables(&self) -> RockTables<'_> {
        RockTables { perm: self.noise.permutation(), rotations: &self.rotations, beds: &self.beds, layers: &self.layers }
    }

    pub fn with_lichen(mut self, lichen: f64, color: Color) -> Rock {
        self.lichen = lichen.clamp(0., 1.);
        self.lichen_color = color;
        let noise = &self.noise;
        let mut rng = crate::geometry::Pcg32::new(1, 9);
        let mut v: Vec<f64> = (0 .. 4096).map(|_| {
            noise.fbm(Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 50., 4)
        }).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.lichen_threshold = v[((1. - self.lichen) * (v.len() - 1) as f64) as usize];
        self
    }

    /// The cross-bedded laminae at `q` (`cross_beds`): the lamina's phase and how close the
    /// point is to a set's bounding surface (1 on it).
    fn cross_beds(&self, q: Vector3<f64>) -> (f64, f64) {
        let l = cross_beds(&self.noise, q, self.cross_set);
        (l.phase, l.boundary)
    }

    /// The shading normal: the surface normal tilted by the relief.
    pub fn bumped_normal(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        self.bumped_normal_filtered(p, normal, 0.)
    }

    /// As `bumped_normal`, with fractal detail no finer than `width` (scene units: about a
    /// pixel's size where the surface is seen; 0 for the coarse relief alone).
    pub fn bumped_normal_filtered(&self, p: Vector3<f64>, normal: Vector3<f64>, width: f64) -> Vector3<f64> {
        let n = self.recipe().bumped_normal(&self.tables(), (p.x, p.y, p.z), (normal.x, normal.y, normal.z), width);
        Vector3::new(n.0, n.1, n.2)
    }

    /// The diffuse reflectance at a point, given its shading normal.
    pub fn albedo(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Color {
        let (recipe, tables) = (self.recipe(), self.tables());
        let c = recipe.albedo_before(&tables, (p.x, p.y, p.z));
        let mut c = Color::new(c.0, c.1, c.2);
        if self.cross_bedding > 0. {
            let q = p / self.scale;
            let st = &self.style;
            let l = cross_beds(&self.noise, q, self.cross_set);
            // Each lamina its own colour; harder ones (the ribs) paler, the partings between
            // them darker; a set's bounding surface a thin darker line.
            let bed = if self.beds.is_empty() { st.strata_color } else { self.beds[l.id.rem_euclid(self.beds.len() as i64) as usize] };
            c = c.blend(&bed, self.cross_bedding * 0.65);
            c = c * (1. + self.cross_bedding * 0.25 * (l.hardness - 0.6));
            c = c * (1. - 0.3 * self.cross_bedding * l.boundary);
        }
        let c = recipe.albedo_after(&tables, (p.x, p.y, p.z), (normal.x, normal.y, normal.z), (c.rgb.x, c.rgb.y, c.rgb.z));
        Color::new(c.0, c.1, c.2)
    }

}

impl MaterialModel for Rock {
    fn direct_emit(&self) -> bool { true }

    /// For the kernels (`pathtrace::MAT_ROCK`), but not a wet rock (its film is a mirror the
    /// kernels don't draw), one cross-bedded (its laminae are the renderer's alone) or one with
    /// crackle (hashed by the sine of large arguments, which f32 can't follow).
    fn flat(&self) -> Option<crate::wavefront::flat::FlatMaterial> {
        (self.wetness <= 0. && self.cross_bedding <= 0. && self.crackle <= 0.)
            .then(|| crate::wavefront::flat::FlatMaterial::Rock { record: self.export() })
    }

    /// Lambertian of its albedo where the hit's point came from, for shading at vertices; its
    /// bump, which at a vertex's spacing is the surface's own normal, goes. Not a wet rock,
    /// whose film is a mirror.
    fn bake(&self, i: &Intersection, _s: &Scene) -> Option<crate::material::model::Baked> {
        (self.wetness <= 0.).then(|| crate::material::model::Baked::diffuse(self.albedo(i.texture_point().point, i.normal)))
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let geometric = intersection.facing_normal(r);
        // About a pixel's width where the surface is seen, so the fractal detail stops there.
        let width = s.footprint.as_ref().map_or(0., |f| {
            let dist = (intersection.point - f.location).norm();
            let cos = (r.rd.normalize().dot(&geometric)).abs().max(0.2);
            dist * f.spread * (s.render.samples_per_pixel() as f64).sqrt() / cos
        });
        let normal = self.bumped_normal_filtered(intersection.point, geometric, width);

        if self.wetness > 0. {
            let cos_i = (r.rd.dot(&normal) / r.rd.norm()).abs();
            let film = self.wetness * fresnel_reflectance(cos_i, 1. / WATER, WATER);
            if rand() < film {
                let rd = reflect(r.rd.normalize(), normal) + random_point_in_unit_ball() * 0.05;
                if rd.dot(&geometric) <= 0. {
                    return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::black(), ray: None };
                }
                return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(),
                    ray: Some(Ray { ro: intersection.point, rd }) };
            }
        }

        // Coloured where a displaced surface's point came from, as its relief was made (the
        // ribs of a `cross_beds` displacement under the laminae shown here).
        let albedo = self.albedo(intersection.texture_point().point, normal).as_reflectance();
        let shaded = Intersection { normal, ..*intersection };
        // Cosine-weighted about the shading normal; a bounce the bump tilts into the surface is
        // mirrored back out of it.
        let mut rd = normal + random_point_on_unit_sphere();
        if rd.norm() < 1e-9 { rd = normal }
        let into = rd.dot(&geometric);
        if into < 0. { rd -= geometric * (2. * into) }
        ScatteredRay {
            emit: direct_lighting(albedo, &shaded, s),
            light_sampling: LightSampling::Exclusive,
            attenuate: albedo,
            ray: Some(Ray { ro: intersection.point, rd: rd.normalize() }),
        }
    }
}

#[cfg(test)]
mod tests {
    /// Laminae seen along any line through the rock are mostly wider than `LAMINA_FINEST` sets,
    /// though their spacing is 0.07 to 0.12 of a set along the foresets: troughs bend them and
    /// the foresets steepen towards a set's base.
    #[test]
    fn test_laminae_are_mostly_wider_than_the_finest() {
        let noise = SeededNoise::new(3);
        let mut widths = vec![];
        for line in 0..200 {
            let a = line as f64 * 2.399;
            let dir = Vector3::new(a.cos() * (line as f64 * 0.7).sin(), (line as f64 * 1.3).cos() * 0.6, a.sin()).normalize();
            let start = Vector3::new(line as f64 * 1.7, line as f64 * 0.31, -(line as f64) * 0.9);
            let step = 0.001;
            let (mut last, mut since) = (cross_beds(&noise, start, 1.).id, 0.);
            for k in 1..10000 {
                let l = cross_beds(&noise, start + dir * (k as f64 * step), 1.);
                since += step;
                if l.id != last { widths.push(since); since = 0.; last = l.id; }
            }
        }
        let narrower = widths.iter().filter(|&&w| w < LAMINA_FINEST).count() as f64 / widths.len() as f64;
        assert!(narrower < 0.1, "{:.0}% of laminae are narrower than {}", 100. * narrower, LAMINA_FINEST);
    }

    use super::*;

    const PRESETS: [&str; 6] = ["granite", "basalt", "sandstone", "limestone", "slate", "quartzite"];

    #[test]
    fn test_albedo_is_plausible_and_varies() {
        for name in PRESETS {
            let rock = Rock::new(RockStyle::preset(name), 1., 3).with_lichen(0.2, Color::new(0.3, 0.3, 0.2));
            let mut rng = crate::geometry::Pcg32::new(0, 0);
            let mut seen = Vec::new();
            for _ in 0 .. 500 {
                let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 10.;
                let c = rock.albedo(p, Vector3::y());
                for v in c.rgb.iter() {
                    assert!(*v >= 0. && *v < 0.95, "{} albedo {}", name, c.rgb);
                }
                seen.push(c.rgb.x);
            }
            let (lo, hi) = seen.iter().fold((1f64, 0f64), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
            assert!(hi - lo > 0.02, "{} is flat: {} to {}", name, lo, hi);
        }
    }

    fn mean_albedo(rock: &Rock, y: f64, normal: Vector3<f64>) -> Color {
        let mut rng = crate::geometry::Pcg32::new(1, 1);
        let mut sum = Color::black();
        for _ in 0 .. 400 {
            let p = Vector3::new(rng.next_f64() * 50., y, rng.next_f64() * 50.);
            sum = sum + rock.albedo(p, normal);
        }
        sum * (1. / 400.)
    }

    #[test]
    fn test_layers_colour_the_rock_by_elevation() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        rock.layers = vec![(0., Color::new(0.6, 0.1, 0.1)), (100., Color::new(0.1, 0.1, 0.6))];
        let (low, high) = (mean_albedo(&rock, -10., Vector3::y()), mean_albedo(&rock, 110., Vector3::y()));
        assert!(low.rgb.x > 2. * low.rgb.z && high.rgb.z > 2. * high.rgb.x, "{} {}", low.rgb, high.rgb);
    }

    #[test]
    fn test_varnish_darkens_steep_faces_only() {
        let plain = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        let mut varnished = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        varnished.varnish = 1.;
        let wall = Vector3::x();
        let lum = |c: Color| crate::color::luminance(&c);
        assert!(lum(mean_albedo(&varnished, 5., wall)) < 0.85 * lum(mean_albedo(&plain, 5., wall)));
        assert!((lum(mean_albedo(&varnished, 5., Vector3::y())) - lum(mean_albedo(&plain, 5., Vector3::y()))).abs() < 1e-9);
    }

    #[test]
    fn test_cross_beds_dip_within_level_sets() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        rock.cross_set = 1.;
        // Within a set, the laminae's phase changes along the ground (they dip); every set has
        // level bounding surfaces at whole multiples of its thickness, give or take its warp.
        let (a, _) = rock.cross_beds(Vector3::new(0., 0.5, 0.));
        let (b, _) = rock.cross_beds(Vector3::new(0.5, 0.5, 0.5));
        assert!((a - b).abs() > 0.5, "{} {}", a, b);
        let mid = rock.cross_beds(Vector3::new(3., 10.5, 3.)).1;
        assert!(mid < 0.5, "{}", mid);
    }

    #[test]
    fn test_pits_darken_and_recess() {
        let plain = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        let mut pitted = Rock::new(RockStyle::preset("sandstone"), 1., 2);
        pitted.pits = 1.;
        let lum = |c: Color| crate::color::luminance(&c);
        let wall = Vector3::x();
        assert!(lum(mean_albedo(&pitted, 5., wall)) < 0.97 * lum(mean_albedo(&plain, 5., wall)));
    }

    #[test]
    fn test_fractal_detail_follows_the_pixel_size() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 10., 8);
        rock.fractal = 0.5;
        rock.fractal_size = 0.1;
        let n = Vector3::new(0., 0., -1.);
        // The mean tilt of the shading normal along a line, at a given pixel width.
        let tilt = |width: f64| (0 .. 200).map(|k| {
            let p = Vector3::new(k as f64 * 0.0137, 1.3, -3.);
            rock.bumped_normal_filtered(p, n, width).dot(&n).acos()
        }).sum::<f64>() / 200.;
        // Seen closer (smaller pixels), finer octaves change the normal; seen from afar, the
        // fractal fades out rather than aliasing.
        let change = (0 .. 200).map(|k| {
            let p = Vector3::new(k as f64 * 0.0137, 1.3, -3.);
            rock.bumped_normal_filtered(p, n, 0.001).dot(&rock.bumped_normal_filtered(p, n, 0.05)).min(1.).acos()
        }).sum::<f64>() / 200.;
        assert!(change > 0.05, "{}", change);
        assert!(tilt(100.) < tilt(0.001) * 0.6, "{} {}", tilt(100.), tilt(0.001));
    }

    #[test]
    fn test_crackle_draws_thin_cracks() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 1., 8);
        rock.crackle = 1.;
        rock.crackle_size = 1.;
        let on: usize = (0 .. 2000).filter(|k| rock.recipe().crackle_at(&rock.tables(), (*k as f64 * 0.0377, 0.31, 0.77)) > 0.5).count();
        // Cracks cover a small part of the surface.
        assert!(on > 10 && on < 400, "{}", on);
    }

    #[test]
    fn test_grain_fraction_follows_amount() {
        let rock = Rock::new(RockStyle::preset("granite"), 1., 0);
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let n = 4000;
        let dark = (0 .. n).filter(|_| {
            let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 100.;
            rock.noise.noise(p) > rock.dark_threshold
        }).count() as f64 / n as f64;
        assert!((dark - 0.18).abs() < 0.04, "{}", dark);
    }

    #[test]
    fn test_bumped_normal_stays_on_the_surface_side() {
        let rock = Rock::new(RockStyle::preset("basalt"), 1., 1);
        let mut rng = crate::geometry::Pcg32::new(2, 0);
        let mut tilted = 0;
        for _ in 0 .. 500 {
            let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 5.;
            let n = rock.bumped_normal(p, Vector3::y());
            assert!((n.norm() - 1.).abs() < 1e-9);
            assert!(n.y > 0.2, "{}", n);
            if n.y < 0.999 { tilted += 1 }
        }
        assert!(tilted > 400);
    }

    #[test]
    fn test_wet_rock_is_darker() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 1., 0);
        let p = Vector3::new(0.3, 0.2, 0.1);
        let dry = rock.albedo(p, Vector3::y());
        rock.wetness = 1.;
        let wet = rock.albedo(p, Vector3::y());
        assert!(wet.rgb.x < dry.rgb.x * 0.6);
    }

    #[test]
    #[should_panic(expected = "Unknown rock preset")]
    fn test_unknown_preset() {
        RockStyle::preset("cheese");
    }
}

#[cfg(test)]
mod kernel_tests {
    use super::*;

    /// The kernels' rock (`pathtrace::load_rock` of `Rock::export`, through `shared::rock` at
    /// f32) is the renderer's: every preset, with each optional feature and all of them, at
    /// points within 2 m. A field out of its place in the record would differ by far more than
    /// f32 does; f32 rounds the relief's central differences, at coordinates of hundreds of the
    /// texture's units, by a percent of the gradient, so the normals' tolerance is wider.
    #[test]
    fn test_the_kernels_rock_is_the_renderers() {
        let mut rng = crate::geometry::Pcg32::new(17, 5);
        let mut u = || rng.next_f64();
        let features: [&dyn Fn(&mut Rock); 9] = [
            &|_| {},
            &|r| { r.layers = vec![(-0.5, Color::new(0.5, 0.3, 0.2)), (0.5, Color::new(0.7, 0.6, 0.4))]; r.layer_warp = 0.2; },
            &|r| { r.beds = vec![Color::new(0.6, 0.4, 0.3), Color::new(0.5, 0.5, 0.45)]; r.style.strata_amount = 0.6; },
            &|r| { r.variety = 0.6; r.varnish = 0.8; },
            &|r| { r.flutes = 0.5; r.pits = 0.7; r.pit_size = 0.08; },
            &|r| { r.fractal = 0.8; r.fractal_size = 0.5; r.fractal_h = 0.8; },
            &|r| { *r = std::mem::replace(r, Rock::new(RockStyle::preset("granite"), 1., 0)).with_lichen(0.4, Color::new(0.4, 0.42, 0.25)); },
            &|r| { r.wetness = 0.4; r.weathering = 0.9; r.style.cracks = 0.6; },
            &|r| { r.layers = vec![(0., Color::new(0.5, 0.3, 0.2))]; r.beds = vec![Color::new(0.6, 0.4, 0.3)]; r.variety = 0.5; r.varnish = 0.5;
                   r.flutes = 0.3; r.pits = 0.5; r.fractal = 0.5; r.style.strata_amount = 0.4; r.style.cracks = 0.5; },
        ];
        let (mut albedo_err, mut angles) = (0f64, Vec::new());
        for (i, preset) in ["granite", "basalt", "sandstone", "limestone", "slate", "quartzite"].iter().enumerate() {
            for (j, feature) in features.iter().enumerate() {
                let mut rock = Rock::new(RockStyle::preset(preset), 0.25 + u(), (i * 13 + j) as u64);
                feature(&mut rock);
                let mut tables = vec![0f32; pathtrace::ROCK_FOOTPRINT as usize];
                tables.extend(rock.export());
                let (kernel, _) = pathtrace::load_rock(&tables, 0);
                let t = pathtrace::FloatTables(&tables);
                for _ in 0 .. 100 {
                    let p = Vector3::new(u() - 0.5, u() - 0.5, u() - 0.5) * 4.;
                    let n = Vector3::new(u() - 0.5, u() - 0.5, u() - 0.5).normalize();
                    let width = if u() < 0.3 { 0. } else { u() * 0.02 };
                    let (p32, n32) = ((p.x as f32, p.y as f32, p.z as f32), (n.x as f32, n.y as f32, n.z as f32));
                    let c = kernel.albedo_after(&t, p32, n32, kernel.albedo_before(&t, p32));
                    let want = rock.albedo(p, n);
                    albedo_err = albedo_err.max((0 .. 3).map(|k| ([c.0, c.1, c.2][k] as f64 - want.rgb[k]).abs()).fold(0., f64::max));
                    let b = kernel.bumped_normal(&t, p32, n32, width as f32);
                    let want = rock.bumped_normal_filtered(p, n, width);
                    angles.push(Vector3::new(b.0 as f64, b.1 as f64, b.2 as f64).normalize().cross(&want).norm().asin());
                }
            }
        }
        angles.sort_by(f64::total_cmp);
        let median = angles[angles.len() / 2];
        assert!(albedo_err < 0.01, "albedo differs by {}", albedo_err);
        assert!(median < 0.01, "median normal {} radians apart", median);
    }
}

#[cfg(test)]
mod bake_tests {
    use super::*;

    #[test]
    fn test_a_rock_bakes_to_its_albedo_unless_wet() {
        let s = crate::scenefile::SceneFile::from_string(r#"{ "camera": { "location": [0, 1, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "m": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } },
            "objects": [ { "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "m" } ] }"#.to_string());
        let object = &s.objects.items()[0];
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 0.05, 9);
        for k in 0..20 {
            let p = Vector3::new(k as f64 * 0.37, 0.1 * k as f64, -0.2 * k as f64);
            let i = Intersection { dist: 1., point: p, normal: Vector3::y(), face: 0, object, part: 0 };
            let baked = rock.bake(&i, &s).expect("a dry rock bakes");
            assert_eq!((baked.albedo, baked.coat), (rock.albedo(p, Vector3::y()), 0.));
        }
        rock.wetness = 0.5;
        let i = Intersection { dist: 1., point: Vector3::zeros(), normal: Vector3::y(), face: 0, object, part: 0 };
        assert!(rock.bake(&i, &s).is_none(), "a wet rock's film is a mirror");
    }
}
