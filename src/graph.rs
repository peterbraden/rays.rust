//! # Function graphs
//!
//! A small graph of functions over the ground, written in a scene file as nested JSON: noise,
//! warped, folded into ridges, masked by slope or altitude, blended, terraced. A graph gives a
//! number at each point of a surface, and is what drives a terrain's displacement
//! (`shapes::displaced`); it is meant to drive surface layers' masks and media densities in the
//! same way. After Terragen's node networks, in which any fractal can be warped, masked and fed
//! into anything (Planetside, "Power Fractal Shader v3", "Surface Layer", "Warp Input Shader").
//!
//! A node is a number, a string (an expression, see below), or an object naming its kind with
//! `"node"`. Reuse a node by defining it once in the scene's `variables` and writing `"$name"`.
//!
//! Sources:
//! - `fractal`: Perlin noise summed over octaves, from `scale` (the largest features, metres,
//!   default 10) down to `smallest` (default `scale / 256`), each `lacunarity` (default 2.03)
//!   times finer and `roughness` (default 0.5) times weaker. `style`: `"perlin"` (in
//!   [-amplitude, amplitude]), `"ridged"` (sharp crests, Musgrave's ridged multifractal, in
//!   [0, amplitude]) or `"billow"` (rounded heaps, |noise|, in [0, amplitude]). `amplitude`
//!   (default 1), `seed`, and `plan` (true: vary only across x and z, as for strata seen from
//!   above; by default in 3D, so cliffs get relief too).
//! - `cells`: Worley's cellular noise, cells `scale` across (default 1), `seed`, `amplitude`,
//!   `plan`, and `style`: `"f1"` (distance to the nearest feature point, in cells), `"edges"`
//!   (F2 - F1: zero along the cracks between cells), or `"stones"` (a rounded stone in each
//!   cell, each its own size and height, as Terragen's Fake Stones), each in [0, amplitude].
//! - `talus`: fallen blocks and scree, angular and fractured, many small to few large, sorted
//!   down the slope (`talus.rs`). Heights in metres; meant for a terrain's
//!   `"vertical_displacement"`, so blocks stand upright on a slope and a material can find them.
//! - `slope`: the surface's slope, 0 below `from` degrees and 1 above `to`, smoothly (falling
//!   instead where `to` is below `from`, as for every node with `from` and `to`);
//!   `breakup` (a node, in degrees) added to the slope first, so the edge of the zone is
//!   ragged rather than a contour.
//! - `altitude`: as `slope`, by height (y, metres) from `from` to `to`, with `breakup` in metres.
//! - `aspect`: how squarely the surface faces `direction` (a vector, default `[0, 0, -1]`, which
//!   is north on a landscape): the normal's component along it, 0 on level ground and 1 on a
//!   wall facing it, as shaded north faces keep their snow; smoothly from `from` (default 0) to
//!   `to` (default 1), with `breakup`.
//! - `field`: what the surface knows about itself, given by the object hit (`Fields`): for a
//!   landscape, what its simulation left there (see `Field`: `sediment`, `ice`, `lava`, `flow`,
//!   `wetness`, `convexity`, `elevation`, `snowline`, `treeline`, `temperature`,
//!   `precipitation`), `default` (0) on surfaces without it. With `from` and `to`, smoothly from
//!   0 to 1 between them; without, the value itself; either way with `breakup` added first.
//! - `strata`: ledges: beds `step` metres thick (default 1), each eroded back from the one
//!   above it and ending in a riser `sharpness` of the way to vertical (default 0.8), as the
//!   difference between the terraced height and the height, in [-amplitude, amplitude]
//!   (default `step` / 2); `breakup` (a node, metres) added to the height first, so the beds
//!   waver and pinch out. Displaced along a slope's normal, it cuts a slope into ledges.
//! - `cross_beds`: the festoon cross-bedding of a `rock` material (`material::rock::cross_beds`)
//!   as relief, so the laminae coloured on slickrock stand out as ribs where they are hard and
//!   are worn back where soft: give the rock's `seed`, `scale` and `cross_set` (`set`), and the
//!   relief between the softest and hardest laminae (`amplitude`, metres, default 0.05). In
//!   [-amplitude / 2, amplitude / 2] about a mean of 0; filtered out where the laminae are
//!   finer than the vertices.
//! - a number, or `{"node": "constant", "value": v}`.
//! - an expression, `"0.1 * sin(x)"`, or `{"node": "expr", "expr": ..., "inputs": {"a": node,
//!   ...}, "range": [lo, hi]}` (`expr.rs`'s syntax), over `x`, `y`, `z`, `slope` (degrees) and
//!   the named inputs. Its value is clamped to `range`, which a displacement needs to bound the
//!   surface; a bare string has no range.
//!
//! Combinations (`input`, `a`, `b` and `mask` are nodes; `inputs` a list of them):
//! - `add`, `multiply`, `min`, `max` over `inputs`; `subtract`: `a` - `b`.
//! - `blend`: `a` where `mask` is 0, `b` where it is 1, mixed between.
//! - `clamp` (`input`, `min`, `max`), `abs`, `power` (`input`, `exponent`: of |input|, signed),
//!   `smoothstep` (`input` from `from` to `to`, to [0, 1]), `remap` (`input` from `from` [lo,
//!   hi] onto `to` [lo, hi], linearly).
//! - `terrace`: `input` stepped in `step`s, each step's riser `sharpness` (0: smooth, 1:
//!   vertical, default 0.8) of the way to a cliff, as strata weather into ledges.
//! - `warp`: `input` read at a point moved by `amount` (metres, default 1) times a vector of
//!   noise `scale` across (default 10; `seed`, `octaves`, default 3), so its features flow and
//!   bend (domain distortion, Inigo Quilez, "Domain warping",
//!   <https://iquilezles.org/articles/warp/>); or by the nodes `x`, `y` and `z` given.
//!
//! Every node knows its range (by interval arithmetic) and clamps to it, so a displacement can
//! bound its surface before evaluating it. Fractals and cells are filtered: given the size of
//! the smallest detail that will be seen at a point (`Context::filter`, for a displaced surface
//! the spacing of its vertices), octaves finer than twice that are left out (their mean in
//! their place, the last fading out), so a surface tessellated coarsely far from the camera
//! doesn't alias.
use crate::expr::Expr;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use serde_json::Value;
use std::sync::LazyLock;

/// What a surface can say about itself at a point, for `field` nodes: a landscape knows what its
/// simulation left there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    /// Loose sediment over the rock (alluvium, colluvium, lake fill), metres thick.
    Sediment,
    /// Glacier ice, metres thick.
    Ice,
    /// Volcanic rock under the sediment, metres thick.
    Lava,
    /// How much water flows past: log10 of the area draining through the point, m².
    Flow,
    /// The topographic wetness index, ln(a / tan β), a the area drained per unit width of
    /// contour (m) and β the slope (Beven and Kirkby, "A physically based, variable
    /// contributing area model of basin hydrology", Hydrological Sciences Bulletin 24, 1979):
    /// high in hollows and valley floors where water gathers, low on ridges and steep slopes.
    Wetness,
    /// How far the ground stands above the mean of its surroundings, metres: positive on ridges
    /// and spurs, negative in hollows and gullies.
    Convexity,
    /// Height above sea level, metres (unlike `altitude`, the planet's curvature and a
    /// landscape's scale and exaggeration taken out).
    Elevation,
    /// Height above the snowline, metres: where the summer stays below freezing.
    Snowline,
    /// Height above the treeline, metres: where the warmest month reaches 10 °C.
    Treeline,
    /// Mean annual temperature, °C (from the climate model).
    Temperature,
    /// Precipitation, mm a year (from the climate model).
    Precipitation,
}

impl Field {
    pub fn parse(s: &str) -> Result<Field, String> {
        Ok(match s {
            "sediment" => Field::Sediment,
            "ice" => Field::Ice,
            "lava" => Field::Lava,
            "flow" => Field::Flow,
            "wetness" => Field::Wetness,
            "convexity" => Field::Convexity,
            "elevation" => Field::Elevation,
            "snowline" => Field::Snowline,
            "treeline" => Field::Treeline,
            "temperature" => Field::Temperature,
            "precipitation" => Field::Precipitation,
            _ => return Err(format!("unknown field \"{}\": sediment, ice, lava, flow, wetness, convexity, elevation, snowline, treeline, temperature or precipitation", s)),
        })
    }
}

/// A surface's fields (`Field`) at scene points on it; None for one it doesn't have.
pub trait Fields: Sync + Send {
    fn field(&self, f: Field, p: &Vector3<f64>) -> Option<f64>;

    /// Where the scene's y isn't up at `p` (a landscape curved round a planet): the local up
    /// (a unit vector) and the height above the planet's surface, in scene units, for `slope`,
    /// `altitude` and `aspect`. None where y is up.
    fn vertical(&self, _p: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> { None }

    /// The climate's biome at `p` (`biome` nodes); None where there is no climate.
    fn biome(&self, _p: &Vector3<f64>) -> Option<crate::terrain::climate::Biome> { None }
}

/// Where a graph is evaluated: the point on the surface (before any displacement), the
/// surface's unit normal there, the size of the smallest detail worth keeping (metres; 0
/// for all of it), and what the surface knows about itself (`Fields`), if anything.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub filter: f64,
    /// The point is on the surface the graph displaced (a shading point), not the surface
    /// before displacement: `talus` then finds its blocks' footing below it.
    pub lifted: bool,
    pub fields: Option<&'a dyn Fields>,
}

impl<'a> Context<'a> {
    pub fn new(p: Vector3<f64>, normal: Vector3<f64>, filter: f64) -> Context<'a> {
        Context { p, normal, filter, lifted: false, fields: None }
    }

    /// At a shading point on the displaced surface, unfiltered (see `lifted`).
    pub fn lifted(p: Vector3<f64>, normal: Vector3<f64>) -> Context<'a> {
        Context { p, normal, filter: 0., lifted: true, fields: None }
    }

    /// At a shading point: where a displaced surface's point was displaced from, with the
    /// filter its displacement had there, so the graph gives what the geometry got; on other
    /// surfaces at the hit itself, `lifted`. With the surface's fields.
    pub fn at_hit(i: &'a crate::intersection::Intersection) -> Context<'a> {
        let c = match i.object.geometry.undisplaced(&i.point, i.face) {
            Some(u) => Context::new(u.point, u.normal, u.filter),
            None => Context::lifted(i.point, i.normal),
        };
        c.with_fields(i.object.medium.fields())
    }

    /// As `at_hit`, its detail filtered to the pixel's footprint there (`textures::footprint`)
    /// where the surface wasn't displaced (a displaced surface's filter is its vertices'
    /// spacing): what textures and bumps read, so detail finer than a pixel fades to its mean
    /// rather than aliasing.
    pub fn at_hit_filtered(i: &'a crate::intersection::Intersection, s: &crate::scene::Scene) -> Context<'a> {
        let c = Context::at_hit(i);
        if c.filter > 0. {
            return c;
        }
        match crate::textures::footprint(i, s) {
            Some((dx, dy)) => Context { filter: dx.norm().max(dy.norm()), ..c },
            None => c,
        }
    }

    /// The context seen from a pattern's frame (`Geometry::pattern_frame`): its point mapped
    /// there, its normal turned (by the inverse transpose) and its filter scaled.
    pub fn in_frame(self, frame: &crate::na::Affine3<f64>) -> Context<'a> {
        let linear: crate::na::Matrix3<f64> = frame.matrix().fixed_view::<3, 3>(0, 0).into();
        let normals = linear.try_inverse().map_or(linear, |m| m.transpose());
        Context {
            p: frame.transform_point(&crate::na::Point3::from(self.p)).coords,
            normal: (normals * self.normal).try_normalize(1e-12).unwrap_or(self.normal),
            filter: self.filter * linear.determinant().abs().cbrt(),
            ..self
        }
    }

    /// As `at_hit_filtered`, in the hit object's own space (`Geometry::pattern_frame`), where
    /// solid textures are read so that they go with their object.
    pub fn at_hit_in_object(i: &'a crate::intersection::Intersection, s: &crate::scene::Scene) -> Context<'a> {
        let c = Context::at_hit_filtered(i, s);
        match i.object.geometry.pattern_frame(&i.point, i.face) {
            Some(f) => c.in_frame(&f),
            None => c,
        }
    }

    /// The same, on a surface with these fields.
    pub fn with_fields(self, fields: Option<&'a dyn Fields>) -> Context<'a> {
        Context { fields, ..self }
    }

    /// The local up and height (`Fields::vertical`), where y isn't up.
    fn vertical(&self) -> Option<(Vector3<f64>, f64)> {
        self.fields.and_then(|f| f.vertical(&self.p))
    }

    /// The surface's slope, degrees from level.
    pub fn slope(&self) -> f64 {
        let up = self.vertical().map_or(self.normal.y, |(up, _)| self.normal.dot(&up));
        up.clamp(-1., 1.).acos().to_degrees()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FractalStyle { Perlin, Ridged, Billow }

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellStyle { F1, F2, Edges, Stones, Id }

/// How distance is measured between a point and cells' feature points (Worley, "A cellular
/// texture basis function", SIGGRAPH 1996, section 4): Euclidean cells are rounded, Manhattan
/// ones diamond-faceted, Chebyshev ones square, like crystals or paving.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Metric { Euclidean, Manhattan, Chebyshev }

#[derive(Clone)]
enum Op {
    Const(f64),
    Fractal { noise: Box<SeededNoise>, style: FractalStyle, scale: f64, smallest: f64, roughness: f64, lacunarity: f64, amplitude: f64, plan: bool },
    Cells { seed: u32, style: CellStyle, scale: f64, amplitude: f64, plan: bool, metric: Metric, jitter: f64 },
    /// Sparse Gabor or phasor noise (`patterns::Gabor`), in [-amplitude, amplitude].
    Gabor { noise: Box<crate::patterns::Gabor>, amplitude: f64 },
    /// The distance along `direction` (a unit vector) over `scale`: POV-Ray's gradient.
    Gradient { direction: Vector3<f64>, scale: f64 },
    /// The distance from the line through `center` along `axis` (a unit vector) over `scale`:
    /// rings about it, as a tree's growth rings.
    Rings { center: Vector3<f64>, axis: Vector3<f64>, scale: f64 },
    /// The distance from `center` over `scale`: shells, as an onion's.
    Spherical { center: Vector3<f64>, scale: f64 },
    /// `input` turned into repeating bands, `frequency` per unit of it, box filtered.
    Wave { input: Box<Node>, shape: crate::patterns::Wave, frequency: f64, phase: f64, exponent: f64 },
    Checker { size: f64, plan: bool },
    Brick { size: Vector3<f64>, mortar: f64, style: crate::patterns::BrickStyle, seed: u64 },
    /// `input` read at `inverse` (p - translate): moved, turned and stretched.
    Transform { input: Box<Node>, inverse: crate::na::Matrix3<f64>, normals: crate::na::Matrix3<f64>, translate: Vector3<f64>, shrink: f64 },
    Slope { from: f64, to: f64, breakup: Option<Box<Node>> },
    Altitude { from: f64, to: f64, breakup: Option<Box<Node>> },
    Aspect { direction: Vector3<f64>, from: f64, to: f64, breakup: Option<Box<Node>> },
    /// A surface's field; smoothstepped from `from` to `to` if `band` is given.
    Field { field: Field, default: f64, band: Option<(f64, f64)>, breakup: Option<Box<Node>> },
    /// 1 where the climate's biome is one of these, else 0 (`default` without a climate).
    Biome { of: Vec<crate::terrain::climate::Biome>, default: f64 },
    Strata { step: f64, sharpness: f64, amplitude: f64, breakup: Option<Box<Node>> },
    CrossBeds { noise: Box<SeededNoise>, scale: f64, set: f64, amplitude: f64 },
    Expr { expr: Expr, inputs: Vec<Node>, range: (f64, f64) },
    Add(Vec<Node>),
    Multiply(Vec<Node>),
    Min(Vec<Node>),
    Max(Vec<Node>),
    Subtract(Box<Node>, Box<Node>),
    Blend { a: Box<Node>, b: Box<Node>, mask: Box<Node> },
    Clamp { input: Box<Node>, lo: f64, hi: f64 },
    Abs(Box<Node>),
    Power { input: Box<Node>, exponent: f64 },
    Smoothstep { input: Box<Node>, from: f64, to: f64 },
    Remap { input: Box<Node>, from: (f64, f64), to: (f64, f64) },
    Terrace { input: Box<Node>, step: f64, sharpness: f64 },
    Warp { input: Box<Node>, by: [Box<Node>; 3] },
    Talus(Box<crate::talus::Talus<Node>>),
}

/// A node: its operation, and the bounds on its value, found once when it is parsed (every
/// evaluation clamps to them).
#[derive(Clone)]
struct Node {
    op: Op,
    range: (f64, f64),
}

/// A parsed graph: its output node.
#[derive(Clone)]
pub struct Graph {
    root: Node,
    range: (f64, f64),
}

/// The mean hardness of cross-bedded laminae, about which `cross_beds` relief is centred.
const CROSS_BED_MEAN: f64 = 0.6;

/// Octaves are rotated, which hides the lattice's axes (as `SeededNoise::fbm` does).
static ROTATION: LazyLock<crate::na::Rotation3<f64>> = LazyLock::new(|| crate::na::Rotation3::from_euler_angles(0.61, 0.93, 1.27));

/// The mean of a ridged and of a billowed octave, put in place of the octaves filtered out, so
/// that filtering removes detail without raising or lowering the surface.
static OCTAVE_MEANS: LazyLock<(f64, f64)> = LazyLock::new(|| {
    let noise = SeededNoise::new(7);
    let mut rng = crate::geometry::Pcg32::new(11, 3);
    let (mut ridged, mut billow, n) = (0., 0., 20000);
    for _ in 0 .. n {
        let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 64.;
        let v = noise.noise(p).clamp(-1., 1.);
        ridged += (1. - v.abs()).powi(2);
        billow += v.abs();
    }
    (ridged / n as f64, billow / n as f64)
});

/// 0 below `lo`, 1 above `hi`, smoothly between; falling instead where `hi` is below `lo`.
fn smoothstep(lo: f64, hi: f64, x: f64) -> f64 {
    if hi < lo {
        return 1. - smoothstep(hi, lo, x);
    }
    if hi == lo {
        return if x >= hi { 1. } else { 0. };
    }
    let t = ((x - lo) / (hi - lo)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Hash integers to [0, 1).
fn hash01(a: i64, b: i64, c: i64, seed: u32) -> f64 {
    let mut h = (a as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (b as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ (c as u64).wrapping_mul(0x1656_67b1_9e37_79f9)
        ^ (seed as u64).wrapping_mul(0x27d4_eb2f_1656_67c5);
    h ^= h >> 31;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 29;
    (h >> 11) as f64 / (1u64 << 53) as f64
}

fn mul_range(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    let c = [a.0 * b.0, a.0 * b.1, a.1 * b.0, a.1 * b.1].map(|v| if v.is_nan() { 0. } else { v });
    (c.iter().cloned().fold(f64::INFINITY, f64::min), c.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
}

impl Node {
    fn new(op: Op) -> Node {
        // Interval arithmetic on infinite bounds (an unbanded `field`) can give NaN, eg.
        // inf * 0 in a `remap` to a single value; f64::clamp panics on NaN bounds.
        let (lo, hi) = op.range();
        let range = (if lo.is_nan() { f64::NEG_INFINITY } else { lo }, if hi.is_nan() { f64::INFINITY } else { hi });
        Node { op, range }
    }

    fn eval(&self, c: &Context) -> f64 {
        let v = match &self.op {
            Op::Const(v) => *v,
            Op::Fractal { noise, style, scale, smallest, roughness, lacunarity, amplitude, plan } => {
                fractal(noise, *style, *scale, *smallest, *roughness, *lacunarity, *plan, c) * amplitude
            },
            Op::Cells { seed, style, scale, amplitude, plan, metric, jitter } => cells(*seed, *style, *scale, *plan, *metric, *jitter, c) * amplitude,
            Op::Gabor { noise, amplitude } => {
                let p = if noise.plan { Vector3::new(c.p.x, 0., c.p.z) } else { c.p };
                noise.eval(p, c.filter) * amplitude
            },
            Op::Gradient { direction, scale } => c.p.dot(direction) / scale,
            Op::Rings { center, axis, scale } => {
                let d = c.p - center;
                (d - axis * d.dot(axis)).norm() / scale
            },
            Op::Spherical { center, scale } => (c.p - center).norm() / scale,
            Op::Wave { input, shape, frequency, phase, exponent } => {
                let t = input.eval(c) * frequency + phase;
                // The cycles the footprint spans: the input's change across it, along the
                // surface (two more evaluations, only where filtering).
                let w = if c.filter > 0. {
                    let (u, v) = tangents(&c.normal);
                    let h = c.filter;
                    let base = input.eval(c);
                    let du = input.eval(&Context { p: c.p + u * h, ..*c }) - base;
                    let dv = input.eval(&Context { p: c.p + v * h, ..*c }) - base;
                    (du * du + dv * dv).sqrt() * frequency.abs()
                } else { 0. };
                let v = shape.filtered(t, w);
                if *exponent == 1. { v } else { v.powf(*exponent) }
            },
            Op::Checker { size, plan } => crate::patterns::checker(c.p, *size, c.filter, *plan),
            Op::Brick { size, mortar, style, seed } => crate::patterns::brick(c.p, *size, *mortar, c.filter, *style, *seed),
            Op::Transform { input, inverse, normals, translate, shrink } => {
                let n = (normals * c.normal).try_normalize(1e-12).unwrap_or(c.normal);
                input.eval(&Context { p: inverse * (c.p - translate), normal: n, filter: c.filter * shrink, ..*c })
            },
            Op::Slope { from, to, breakup } => {
                let s = c.slope() + breakup.as_ref().map_or(0., |b| b.eval(c));
                smoothstep(*from, *to, s)
            },
            Op::Altitude { from, to, breakup } => {
                let h = c.vertical().map_or(c.p.y, |(_, h)| h) + breakup.as_ref().map_or(0., |b| b.eval(c));
                smoothstep(*from, *to, h)
            },
            Op::Aspect { direction, from, to, breakup } => {
                // Round a planet, the direction laid level where the surface is.
                let facing = match c.vertical() {
                    Some((up, _)) => (direction - up * direction.dot(&up)).try_normalize(1e-12).unwrap_or(*direction),
                    None => *direction,
                };
                let a = c.normal.dot(&facing) + breakup.as_ref().map_or(0., |b| b.eval(c));
                smoothstep(*from, *to, a)
            },
            Op::Biome { of, default } => match c.fields.and_then(|f| f.biome(&c.p)) {
                Some(b) => if of.contains(&b) { 1. } else { 0. },
                None => *default,
            },
            Op::Field { field, default, band, breakup } => {
                let v = c.fields.and_then(|f| f.field(*field, &c.p)).unwrap_or(*default)
                    + breakup.as_ref().map_or(0., |b| b.eval(c));
                match band {
                    Some((from, to)) => smoothstep(*from, *to, v),
                    None => v,
                }
            },
            Op::Strata { step, sharpness, amplitude, breakup } => {
                let h = c.p.y + breakup.as_ref().map_or(0., |b| b.eval(c));
                (terrace(h, *step, *sharpness) - h) / step * amplitude
            },
            Op::CrossBeds { noise, scale, set, amplitude } => {
                // Fade the laminae out to their mean where the vertices are too far apart to
                // show them.
                let spacing = crate::material::rock::LAMINA_FINEST * set * scale;
                let keep = if c.filter > 0. { smoothstep(2. * c.filter, 4. * c.filter, spacing) } else { 1. };
                if keep <= 0. {
                    0.
                } else {
                    let l = crate::material::rock::cross_beds(noise, c.p / *scale, *set);
                    keep * amplitude * (l.hardness - CROSS_BED_MEAN)
                }
            },
            Op::Expr { expr, inputs, .. } => {
                // On the stack, unless there are many inputs.
                let mut vars = [0.; 16];
                if inputs.len() + 4 > vars.len() {
                    let mut vars = vec![c.p.x, c.p.y, c.p.z, c.slope()];
                    vars.extend(inputs.iter().map(|n| n.eval(c)));
                    return expr.eval(&vars).clamp(self.range.0, self.range.1);
                }
                vars[.. 4].copy_from_slice(&[c.p.x, c.p.y, c.p.z, c.slope()]);
                for (v, n) in vars[4 ..].iter_mut().zip(inputs) {
                    *v = n.eval(c);
                }
                expr.eval(&vars[.. 4 + inputs.len()])
            },
            Op::Add(v) => v.iter().map(|n| n.eval(c)).sum(),
            Op::Multiply(v) => {
                // Inputs after a zero are left unevaluated: often a mask, or stones where there
                // are none.
                let mut product = 1.;
                for n in v {
                    product *= n.eval(c);
                    if product == 0. { break }
                }
                product
            },
            Op::Min(v) => v.iter().map(|n| n.eval(c)).fold(f64::INFINITY, f64::min),
            Op::Max(v) => v.iter().map(|n| n.eval(c)).fold(f64::NEG_INFINITY, f64::max),
            Op::Subtract(a, b) => a.eval(c) - b.eval(c),
            Op::Blend { a, b, mask } => {
                let m = mask.eval(c).clamp(0., 1.);
                // Only what is seen: either side alone where the mask is 0 or 1.
                if m <= 0. { a.eval(c) } else if m >= 1. { b.eval(c) } else { a.eval(c) * (1. - m) + b.eval(c) * m }
            },
            Op::Clamp { input, lo, hi } => input.eval(c).clamp(*lo, *hi),
            Op::Abs(a) => a.eval(c).abs(),
            Op::Power { input, exponent } => {
                let v = input.eval(c);
                v.signum() * v.abs().powf(*exponent)
            },
            Op::Smoothstep { input, from, to } => smoothstep(*from, *to, input.eval(c)),
            Op::Remap { input, from, to } => {
                let t = if from.1 != from.0 { (input.eval(c) - from.0) / (from.1 - from.0) } else { 0. };
                to.0 + t * (to.1 - to.0)
            },
            Op::Terrace { input, step, sharpness } => terrace(input.eval(c), *step, *sharpness),
            Op::Warp { input, by } => {
                let d = Vector3::new(by[0].eval(c), by[1].eval(c), by[2].eval(c));
                input.eval(&Context { p: c.p + d, ..*c })
            },
            Op::Talus(t) => t.eval(c, |n, c| n.eval(c)),
        };
        v.clamp(self.range.0, self.range.1)
    }

    fn range(&self) -> (f64, f64) {
        self.range
    }

    fn finest(&self) -> f64 {
        self.op.finest()
    }
}

impl Op {
    /// Bounds on the node's value, by interval arithmetic.
    fn range(&self) -> (f64, f64) {
        match self {
            Op::Const(v) => (*v, *v),
            Op::Fractal { style, amplitude, .. } => {
                let a = amplitude.abs();
                match style { FractalStyle::Perlin => (-a, a), _ => (amplitude.min(0.), amplitude.max(0.)) }
            },
            Op::Cells { amplitude, .. } => (amplitude.min(0.), amplitude.max(0.)),
            Op::Gabor { amplitude, .. } => (-amplitude.abs(), amplitude.abs()),
            Op::Gradient { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Op::Rings { .. } | Op::Spherical { .. } => (0., f64::INFINITY),
            Op::Wave { exponent, .. } => if *exponent >= 0. { (0., 1.) } else { (0., f64::INFINITY) },
            Op::Checker { .. } | Op::Brick { .. } => (0., 1.),
            Op::Transform { input, .. } => input.range(),
            Op::Strata { amplitude, .. } => (-amplitude.abs(), amplitude.abs()),
            // Hardness lies in [0.175, 1].
            Op::CrossBeds { amplitude, .. } => {
                let a = amplitude.abs();
                (a * (0.175 - CROSS_BED_MEAN), a * (1. - CROSS_BED_MEAN))
            },
            Op::Slope { .. } | Op::Altitude { .. } | Op::Aspect { .. } | Op::Smoothstep { .. } => (0., 1.),
            Op::Field { band: Some(_), .. } => (0., 1.),
            Op::Biome { default, .. } => (default.min(0.), default.max(1.)),
            Op::Field { band: None, .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Op::Expr { range, .. } => *range,
            Op::Add(v) => v.iter().map(Node::range).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1)),
            Op::Multiply(v) => v.iter().map(Node::range).fold((1., 1.), mul_range),
            Op::Min(v) => v.iter().map(Node::range).fold((f64::INFINITY, f64::INFINITY), |a, b| (a.0.min(b.0), a.1.min(b.1))),
            Op::Max(v) => v.iter().map(Node::range).fold((f64::NEG_INFINITY, f64::NEG_INFINITY), |a, b| (a.0.max(b.0), a.1.max(b.1))),
            Op::Subtract(a, b) => {
                let (a, b) = (a.range(), b.range());
                (a.0 - b.1, a.1 - b.0)
            },
            Op::Blend { a, b, .. } => {
                let (a, b) = (a.range(), b.range());
                (a.0.min(b.0), a.1.max(b.1))
            },
            Op::Clamp { input, lo, hi } => {
                let r = input.range();
                (r.0.clamp(*lo, *hi), r.1.clamp(*lo, *hi))
            },
            Op::Abs(a) => {
                let (lo, hi) = a.range();
                if lo >= 0. { (lo, hi) } else if hi <= 0. { (-hi, -lo) } else { (0., hi.max(-lo)) }
            },
            Op::Power { input, exponent } => {
                let (lo, hi) = input.range();
                let m = lo.abs().max(hi.abs()).powf(*exponent);
                (if lo < 0. { -m } else { lo.powf(*exponent).min(m) }, if hi > 0. { m } else { -(hi.abs().powf(*exponent)) })
            },
            Op::Remap { input, from, to } => {
                let r = input.range();
                let f = |v: f64| if from.1 != from.0 { to.0 + (v - from.0) / (from.1 - from.0) * (to.1 - to.0) } else { to.0 };
                let (a, b) = (f(r.0), f(r.1));
                (a.min(b), a.max(b))
            },
            // Terracing keeps values within a step of where they were.
            Op::Terrace { input, step, .. } => {
                let r = input.range();
                (r.0 - step.abs(), r.1 + step.abs())
            },
            Op::Warp { input, .. } => input.range(),
            Op::Talus(t) => t.range(),
        }
    }

    /// The size of the finest detail the node can make (metres): fractals' `smallest`, cells'
    /// scale; infinite for a node with none.
    fn finest(&self) -> f64 {
        let children: Vec<&Node> = match self {
            Op::Fractal { smallest, .. } => return *smallest,
            Op::Cells { scale, .. } => return 0.1 * scale,
            Op::Gabor { noise, .. } => return noise.finest(),
            Op::Gradient { .. } | Op::Rings { .. } | Op::Spherical { .. } => return f64::INFINITY,
            Op::Checker { size, .. } => return *size,
            Op::Brick { mortar, .. } => return *mortar,
            Op::Wave { input, frequency, .. } => {
                // A cycle is 1 / frequency of the input; how far that is depends on the input.
                return input.finest().min(1. / frequency.abs().max(1e-9));
            },
            Op::Transform { input, shrink, .. } => return input.finest() / shrink.max(1e-12),
            Op::Talus(t) => return t.smallest,
            Op::Const(_) | Op::Biome { .. } => return f64::INFINITY,
            Op::CrossBeds { scale, set, .. } => return crate::material::rock::LAMINA_FINEST * set * scale,
            Op::Strata { step, breakup, .. } => return breakup.as_ref().map_or(f64::INFINITY, |b| b.finest()).min(0.25 * step),
            Op::Slope { breakup, .. } | Op::Altitude { breakup, .. } | Op::Aspect { breakup, .. }
                | Op::Field { breakup, .. } => breakup.iter().map(|b| &**b).collect(),
            Op::Expr { inputs, .. } => inputs.iter().collect(),
            Op::Add(v) | Op::Multiply(v) | Op::Min(v) | Op::Max(v) => v.iter().collect(),
            Op::Subtract(a, b) => vec![a, b],
            Op::Blend { a, b, mask } => vec![a, b, mask],
            Op::Clamp { input, .. } | Op::Abs(input) | Op::Power { input, .. } | Op::Smoothstep { input, .. }
                | Op::Remap { input, .. } | Op::Terrace { input, .. } => vec![input],
            Op::Warp { input, by } => vec![input, &by[0], &by[1], &by[2]],
        };
        children.into_iter().map(Node::finest).fold(f64::INFINITY, f64::min)
    }
}

/// A fractal's value at the context's point, filtered (see the module documentation): in
/// [-1, 1] for Perlin, [0, 1] for ridged and billowed.
#[allow(clippy::too_many_arguments)]
fn fractal(noise: &SeededNoise, style: FractalStyle, scale: f64, smallest: f64, roughness: f64, lacunarity: f64, plan: bool, c: &Context) -> f64 {
    let p = if plan { Vector3::new(c.p.x, 0., c.p.z) } else { c.p } / scale;
    let (ridged_mean, billow_mean) = *OCTAVE_MEANS;
    let mean = match style { FractalStyle::Perlin => 0., FractalStyle::Ridged => ridged_mean, FractalStyle::Billow => billow_mean };
    let (mut sum, mut total, mut amplitude, mut weight, mut size, mut q) = (0., 0., 1., 1., scale, p);
    // Octaves down to `smallest`: the last whose features are at least `smallest` across.
    let octaves = ((scale / smallest.max(1e-9)).ln() / lacunarity.ln()).floor().max(0.) as usize + 1;
    for _ in 0 .. octaves.min(40) {
        // Keep the octave in full while its features are at least four times the filter, fade it
        // out to its mean by twice the filter.
        let keep = if c.filter > 0. { smoothstep(2. * c.filter, 4. * c.filter, size) } else { 1. };
        let v = if keep > 0. {
            let n = noise.noise(q).clamp(-1., 1.);
            match style {
                FractalStyle::Perlin => n,
                FractalStyle::Ridged => (1. - n.abs()).powi(2),
                FractalStyle::Billow => n.abs(),
            }
        } else {
            mean
        };
        let v = mean + (v - mean) * keep;
        sum += v * amplitude * weight;
        total += amplitude;
        if style == FractalStyle::Ridged {
            // Detail gathers on the crests: each octave weighted by the one before.
            weight = (v * 2.).clamp(0., 1.);
        }
        amplitude *= roughness;
        size /= lacunarity;
        q = *ROTATION * q * lacunarity;
    }
    if total <= 0. {
        return mean;
    }
    match style {
        FractalStyle::Perlin => (sum / total).clamp(-1., 1.),
        _ => (sum / total).clamp(0., 1.),
    }
}

/// Cellular noise at the context's point: one feature point jittered in each cell of a lattice
/// `scale` across.
fn cells(seed: u32, style: CellStyle, scale: f64, plan: bool, metric: Metric, jitter: f64, c: &Context) -> f64 {
    let p = c.p / scale;
    let (cx, cy, cz) = (p.x.floor() as i64, if plan { 0 } else { p.y.floor() as i64 }, p.z.floor() as i64);
    let ys: &[i64] = if plan { &[0] } else { &[-1, 0, 1] };
    let (mut f1, mut f2, mut nearest) = (f64::INFINITY, f64::INFINITY, (0, 0, 0));
    for dx in -1 ..= 1 {
        for &dy in ys {
            for dz in -1 ..= 1 {
                let (i, j, k) = (cx + dx, cy + dy, cz + dz);
                let mut f = Vector3::new(
                    i as f64 + hash01(i, j, k, seed),
                    if plan { p.y } else { j as f64 + hash01(i, j, k, seed ^ 0x51ed) },
                    k as f64 + hash01(i, j, k, seed ^ 0xa2c3));
                if jitter != 1. {
                    // Pulled towards the cell's centre: 0 a regular lattice.
                    let centre = Vector3::new(i as f64 + 0.5, if plan { p.y } else { j as f64 + 0.5 }, k as f64 + 0.5);
                    f = centre + (f - centre) * jitter;
                }
                let e = f - p;
                let d = match metric {
                    Metric::Euclidean => e.norm(),
                    Metric::Manhattan => e.x.abs() + e.y.abs() + e.z.abs(),
                    Metric::Chebyshev => e.x.abs().max(e.y.abs()).max(e.z.abs()),
                };
                if d < f1 {
                    f2 = f1;
                    f1 = d;
                    nearest = (i, j, k);
                } else if d < f2 {
                    f2 = d;
                }
            }
        }
    }
    let v = match style {
        CellStyle::F1 => f1,
        CellStyle::F2 => f2,
        CellStyle::Id => {
            let (i, j, k) = nearest;
            hash01(i, j, k, seed ^ 0x1d)
        },
        CellStyle::Edges => f2 - f1,
        CellStyle::Stones => {
            // A stone round each feature point, at most halfway to the next, flattened.
            let (i, j, k) = nearest;
            let radius = (0.5 * (f1 + f2)).min(0.35 + 0.3 * hash01(i, j, k, seed ^ 0x77));
            let height = 0.5 + 0.5 * hash01(i, j, k, seed ^ 0x99);
            let t = (1. - (f1 / radius).powi(2)).max(0.);
            height * t.sqrt()
        },
    };
    v.clamp(0., 1.)
}

/// `v` stepped in `step`s, each riser `sharpness` of the way to vertical.
fn terrace(v: f64, step: f64, sharpness: f64) -> f64 {
    if step <= 0. {
        return v;
    }
    let s = v / step;
    let (k, f) = (s.floor(), s.fract());
    // Each step's rise is squeezed into a band `1 - sharpness` of a step wide at its end.
    let band = (1. - sharpness.clamp(0., 1.)).max(1e-3);
    let rise = smoothstep(1. - band, 1., f);
    (k + rise) * step
}

/// Parsing.
impl Graph {
    pub fn parse(v: &Value) -> Result<Graph, String> {
        let root = parse_node(v)?;
        let range = root.range();
        Ok(Graph { root, range })
    }

    pub fn eval(&self, c: &Context) -> f64 {
        self.root.eval(c)
    }

    /// Bounds on the graph's value.
    pub fn range(&self) -> (f64, f64) {
        self.range
    }

    /// The size of the finest detail the graph makes, metres (infinite for none).
    pub fn finest(&self) -> f64 {
        self.root.finest()
    }
}

fn num(o: &Value, k: &str, def: f64) -> Result<f64, String> {
    match &o[k] {
        Value::Null => Ok(def),
        v => v.as_f64().ok_or_else(|| format!("graph node's \"{}\" must be a number, got {}", k, v)),
    }
}

fn pair(o: &Value, k: &str, def: (f64, f64)) -> Result<(f64, f64), String> {
    match &o[k] {
        Value::Null => Ok(def),
        Value::Array(a) if a.len() == 2 && a.iter().all(Value::is_number) => Ok((a[0].as_f64().unwrap(), a[1].as_f64().unwrap())),
        v => Err(format!("graph node's \"{}\" must be [lo, hi], got {}", k, v)),
    }
}

fn vec3(o: &Value, k: &str, def: Vector3<f64>) -> Result<Vector3<f64>, String> {
    match &o[k] {
        Value::Null => Ok(def),
        Value::Array(a) if a.len() == 3 && a.iter().all(Value::is_number) =>
            Ok(Vector3::new(a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap())),
        v => Err(format!("graph node's \"{}\" must be [x, y, z], got {}", k, v)),
    }
}

fn positive(o: &Value, k: &str, def: f64) -> Result<f64, String> {
    let v = num(o, k, def)?;
    if v > 0. { Ok(v) } else { Err(format!("graph node {}'s \"{}\" must be positive", o["node"], k)) }
}

/// Two unit vectors across a normal.
fn tangents(n: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let u = n.cross(&a).try_normalize(1e-12).unwrap_or(Vector3::z());
    (u, n.cross(&u))
}

fn child(o: &Value, k: &str) -> Result<Box<Node>, String> {
    match o.get(k) {
        Some(v) => Ok(Box::new(parse_node(v)?)),
        None => Err(format!("graph node {} needs \"{}\"", o["node"], k)),
    }
}

fn children(o: &Value) -> Result<Vec<Node>, String> {
    match &o["inputs"] {
        Value::Array(a) if !a.is_empty() => a.iter().map(parse_node).collect(),
        _ => Err(format!("graph node {} needs a list of \"inputs\"", o["node"])),
    }
}

fn parse_node(v: &Value) -> Result<Node, String> {
    match v {
        Value::Number(n) => return Ok(Node::new(Op::Const(n.as_f64().unwrap()))),
        Value::String(s) => return parse_expr(s, &Value::Null, (f64::NEG_INFINITY, f64::INFINITY)),
        Value::Object(_) => {},
        _ => return Err(format!("a graph node is a number, an expression or an object, not {}", v)),
    }
    let o = v;
    let kind = o["node"].as_str().or_else(|| o["type"].as_str()).ok_or_else(|| format!("graph node needs a \"node\": {}", o))?;
    let seed = || -> Result<u64, String> {
        match &o["seed"] {
            Value::Null => Ok(0),
            s => s.as_u64().or_else(|| s.as_f64().filter(|f| *f >= 0. && f.fract() == 0. && *f < 2f64.powi(64)).map(|f| f as u64))
                .ok_or_else(|| format!("a seed is a whole number, at least 0, not {}", s)),
        }
    };
    let plan = o["plan"].as_bool().unwrap_or(false);
    Ok(Node::new(match kind {
        "constant" => Op::Const(num(o, "value", 0.)?),
        "fractal" => {
            let scale = num(o, "scale", 10.)?;
            let style = match o["style"].as_str().unwrap_or("perlin") {
                "perlin" => FractalStyle::Perlin,
                "ridged" => FractalStyle::Ridged,
                "billow" => FractalStyle::Billow,
                s => return Err(format!("unknown fractal style \"{}\": perlin, ridged or billow", s)),
            };
            let lacunarity = num(o, "lacunarity", 2.03)?;
            if lacunarity <= 1. || scale <= 0. {
                return Err("a fractal's scale must be positive and its lacunarity above 1".into());
            }
            Op::Fractal { noise: Box::new(SeededNoise::new(seed()?)), style, scale, smallest: num(o, "smallest", scale / 256.)?.max(1e-6),
                roughness: num(o, "roughness", 0.5)?, lacunarity, amplitude: num(o, "amplitude", 1.)?, plan }
        },
        "cells" => {
            let style = match o["style"].as_str().unwrap_or("f1") {
                "f1" => CellStyle::F1,
                "edges" => CellStyle::Edges,
                "stones" => CellStyle::Stones,
                "f2" => CellStyle::F2,
                "id" => CellStyle::Id,
                s => return Err(format!("unknown cells style \"{}\": f1, f2, edges, stones or id", s)),
            };
            let metric = match o["metric"].as_str().unwrap_or("euclidean") {
                "euclidean" => Metric::Euclidean,
                "manhattan" => Metric::Manhattan,
                "chebyshev" => Metric::Chebyshev,
                s => return Err(format!("unknown cells metric \"{}\": euclidean, manhattan or chebyshev", s)),
            };
            let scale = num(o, "scale", 1.)?;
            if scale <= 0. {
                return Err("cells' scale must be positive".into());
            }
            Op::Cells { seed: seed()? as u32, style, scale, amplitude: num(o, "amplitude", 1.)?, plan, metric,
                jitter: num(o, "jitter", 1.)?.clamp(0., 1.) }
        },
        "slope" | "altitude" => {
            let (from, to) = (num(o, "from", 0.)?, num(o, "to", if kind == "slope" { 90. } else { 1. })?);
            let breakup = o.get("breakup").map(parse_node).transpose()?.map(Box::new);
            if kind == "slope" { Op::Slope { from, to, breakup } } else { Op::Altitude { from, to, breakup } }
        },
        "aspect" => {
            let direction = match &o["direction"] {
                Value::Null => -Vector3::z(),
                Value::Array(a) if a.len() == 3 && a.iter().all(Value::is_number) =>
                    Vector3::new(a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()),
                v => return Err(format!("aspect's \"direction\" must be [x, y, z], got {}", v)),
            };
            if direction.norm() == 0. {
                return Err("aspect's direction must not be zero".into());
            }
            Op::Aspect { direction: direction.normalize(), from: num(o, "from", 0.)?, to: num(o, "to", 1.)?,
                breakup: o.get("breakup").map(parse_node).transpose()?.map(Box::new) }
        },
        "biome" => {
            let names: Vec<&str> = match &o["is"] {
                Value::String(s) => vec![s.as_str()],
                Value::Array(a) => a.iter().filter_map(Value::as_str).collect(),
                _ => return Err("a biome node needs \"is\": a biome's name, or a list of them".into()),
            };
            let of = names.iter().map(|n| crate::terrain::climate::Biome::parse(n)).collect::<Result<Vec<_>, _>>()?;
            Op::Biome { of, default: num(o, "default", 0.)? }
        },
        "field" => {
            let field = Field::parse(o["field"].as_str().ok_or("a field node needs a \"field\" name")?)?;
            let band = match (o.get("from"), o.get("to")) {
                (None, None) => None,
                _ => Some((num(o, "from", 0.)?, num(o, "to", 1.)?)),
            };
            Op::Field { field, default: num(o, "default", 0.)?, band, breakup: o.get("breakup").map(parse_node).transpose()?.map(Box::new) }
        },
        "strata" => {
            let step = num(o, "step", 1.)?;
            if step <= 0. {
                return Err("strata's step must be positive".into());
            }
            Op::Strata { step, sharpness: num(o, "sharpness", 0.8)?, amplitude: num(o, "amplitude", 0.5 * step)?,
                breakup: o.get("breakup").map(parse_node).transpose()?.map(Box::new) }
        },
        "cross_beds" => {
            let (scale, set) = (num(o, "scale", 1.)?, num(o, "set", 0.4)?);
            if scale <= 0. || set <= 0. {
                return Err("cross_beds' scale and set must be positive".into());
            }
            Op::CrossBeds { noise: Box::new(SeededNoise::new(seed()?)), scale, set, amplitude: num(o, "amplitude", 0.05)? }
        },
        "gabor" | "phasor" => {
            use crate::patterns::{Gabor, Profile};
            let profile = if kind == "gabor" { Profile::Gabor } else {
                match o["profile"].as_str().unwrap_or("sine") {
                    "sine" => Profile::Sine,
                    "saw" => Profile::Saw,
                    "square" => Profile::Square,
                    s => return Err(format!("unknown phasor profile \"{}\": sine, saw or square", s)),
                }
            };
            let frequency = match o.get("wavelength") {
                Some(_) => 1. / num(o, "wavelength", 1.)?,
                None => num(o, "frequency", 1.)?,
            };
            let bandwidth = num(o, "bandwidth", 0.5 * frequency)?;
            if !(frequency > 0. && frequency.is_finite()) || bandwidth <= 0. {
                return Err(format!("{}'s frequency (or wavelength) and bandwidth must be positive", kind));
            }
            let direction = match o.get("direction") {
                None => None,
                Some(_) => Some(vec3(o, "direction", Vector3::x())?.try_normalize(1e-12).ok_or("a direction must not be zero")?),
            };
            let direction = direction.map(|d| if plan { Vector3::new(d.x, 0., d.z).try_normalize(1e-12).unwrap_or(Vector3::x()) } else { d });
            Op::Gabor { noise: Box::new(Gabor { seed: seed()?, frequency, bandwidth, direction, spread: num(o, "spread", 0.)?.clamp(0., 1.),
                impulses: num(o, "impulses", 16.)?.clamp(1., 256.), plan, profile }), amplitude: num(o, "amplitude", 1.)? }
        },
        "gradient" => {
            let direction = vec3(o, "direction", Vector3::y())?.try_normalize(1e-12).ok_or("gradient's direction must not be zero")?;
            Op::Gradient { direction, scale: positive(o, "scale", 1.)? }
        },
        "rings" => Op::Rings { center: vec3(o, "center", Vector3::zeros())?,
            axis: vec3(o, "axis", Vector3::y())?.try_normalize(1e-12).ok_or("rings' axis must not be zero")?, scale: positive(o, "scale", 1.)? },
        "spherical" => Op::Spherical { center: vec3(o, "center", Vector3::zeros())?, scale: positive(o, "scale", 1.)? },
        "wave" => Op::Wave {
            input: child(o, "input")?,
            shape: crate::patterns::Wave::parse(o["shape"].as_str().unwrap_or("sine"))?,
            frequency: num(o, "frequency", 1.)?, phase: num(o, "phase", 0.)?, exponent: num(o, "exponent", 1.)?,
        },
        "checker" => Op::Checker { size: positive(o, "size", 1.)?, plan },
        "brick" => {
            let size = vec3(o, "size", Vector3::new(0.215, 0.065, 0.1025))?;
            if size.iter().any(|s| *s <= 0.) {
                return Err("a brick's size must be positive".into());
            }
            let style = match o["style"].as_str().unwrap_or("mortar") {
                "mortar" => crate::patterns::BrickStyle::Mortar,
                "id" => crate::patterns::BrickStyle::Id,
                s => return Err(format!("unknown brick style \"{}\": mortar or id", s)),
            };
            Op::Brick { size, mortar: num(o, "mortar", 0.01)?.max(0.), style, seed: seed()? }
        },
        "transform" => {
            let scale = match &o["scale"] {
                Value::Number(n) => Vector3::repeat(n.as_f64().unwrap()),
                _ => vec3(o, "scale", Vector3::repeat(1.))?,
            };
            if scale.iter().any(|s| *s == 0.) {
                return Err("a transform's scale must not be zero".into());
            }
            let r = vec3(o, "rotate", Vector3::zeros())?.map(f64::to_radians);
            // Rotated about x, then y, then z, as POV-Ray's rotate <x, y, z>.
            let rotation = crate::na::Rotation3::from_axis_angle(&Vector3::z_axis(), r.z)
                * crate::na::Rotation3::from_axis_angle(&Vector3::y_axis(), r.y)
                * crate::na::Rotation3::from_axis_angle(&Vector3::x_axis(), r.x);
            // Scaled, then rotated, then moved: p = R S q + t, so q = S⁻¹ Rᵀ (p - t).
            let inverse = crate::na::Matrix3::from_diagonal(&scale.map(|s| 1. / s)) * rotation.matrix().transpose();
            let shrink = 1. / scale.iter().fold(f64::INFINITY, |m, s| m.min(s.abs()));
            // Normals go by the inverse transpose.
            let normals = inverse.transpose().try_inverse().unwrap_or(inverse);
            Op::Transform { input: child(o, "input")?, inverse, normals, translate: vec3(o, "translate", Vector3::zeros())?, shrink }
        },
        "expr" => {
            let text = o["expr"].as_str().ok_or("an expr node needs an \"expr\" string")?;
            return parse_expr(text, &o["inputs"], pair(o, "range", (f64::NEG_INFINITY, f64::INFINITY))?)
        },
        "add" => Op::Add(children(o)?),
        "multiply" => Op::Multiply(children(o)?),
        "min" => Op::Min(children(o)?),
        "max" => Op::Max(children(o)?),
        "subtract" => Op::Subtract(child(o, "a")?, child(o, "b")?),
        "blend" => Op::Blend { a: child(o, "a")?, b: child(o, "b")?, mask: child(o, "mask")? },
        "clamp" => Op::Clamp { input: child(o, "input")?, lo: num(o, "min", f64::NEG_INFINITY)?, hi: num(o, "max", f64::INFINITY)? },
        "abs" => Op::Abs(child(o, "input")?),
        "power" => Op::Power { input: child(o, "input")?, exponent: num(o, "exponent", 1.)? },
        "smoothstep" => Op::Smoothstep { input: child(o, "input")?, from: num(o, "from", 0.)?, to: num(o, "to", 1.)? },
        "remap" => Op::Remap { input: child(o, "input")?, from: pair(o, "from", (0., 1.))?, to: pair(o, "to", (0., 1.))? },
        "terrace" => Op::Terrace { input: child(o, "input")?, step: num(o, "step", 1.)?, sharpness: num(o, "sharpness", 0.8)? },
        "warp" => {
            let by = if o.get("x").is_some() || o.get("y").is_some() || o.get("z").is_some() {
                let axis = |k: &str| o.get(k).map_or(Ok(Box::new(Node::new(Op::Const(0.)))), |v| parse_node(v).map(Box::new));
                [axis("x")?, axis("y")?, axis("z")?]
            } else {
                // A vector of noise: three fractals of the same scale, seeded apart.
                let (amount, scale, octaves) = (num(o, "amount", 1.)?, num(o, "scale", 10.)?, num(o, "octaves", 3.)?.max(1.));
                let lacunarity: f64 = 2.03;
                let smallest = scale / lacunarity.powf(octaves - 1.) * 0.999;
                let base = seed()?;
                let axis = |k: u64| Box::new(Node::new(Op::Fractal { noise: Box::new(SeededNoise::new(base.wrapping_mul(3).wrapping_add(k + 101))),
                    style: FractalStyle::Perlin, scale, smallest, roughness: 0.5, lacunarity, amplitude: amount, plan: false }));
                [axis(0), axis(1), axis(2)]
            };
            Op::Warp { input: child(o, "input")?, by }
        },
        "talus" => Op::Talus(Box::new(crate::talus::Talus::parse(o, seed()?, |k| o.get(k).map(parse_node).transpose())?)),
        k => return Err(format!("unknown graph node \"{}\"", k)),
    }))
}

fn parse_expr(text: &str, inputs: &Value, range: (f64, f64)) -> Result<Node, String> {
    let mut names = vec!["x", "y", "z", "slope"];
    let mut nodes = vec![];
    if let Value::Object(m) = inputs {
        for (k, v) in m {
            names.push(k);
            nodes.push(parse_node(v)?);
        }
    }
    let expr = Expr::parse(text, &names).map_err(|e| format!("in expression \"{}\": {}", text, e))?;
    Ok(Node::new(Op::Expr { expr, inputs: nodes, range }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn at(x: f64, y: f64, z: f64) -> Context<'static> {
        Context::new(Vector3::new(x, y, z), Vector3::y(), 0.)
    }

    fn graph(v: Value) -> Graph {
        Graph::parse(&v).unwrap()
    }

    #[test]
    fn test_pattern_nodes() {
        // Bands along y, turned to run along x by a transform.
        let bands = json!({"node": "wave", "shape": "square", "input": {"node": "gradient", "direction": [0, 1, 0]}});
        let g = graph(bands.clone());
        assert_eq!(g.range(), (0., 1.));
        assert_eq!(g.eval(&at(0., 0.25, 0.)), 1.);
        assert_eq!(g.eval(&at(0., 0.75, 0.)), 0.);
        let turned = graph(json!({"node": "transform", "rotate": [0, 0, -90], "input": bands}));
        assert_eq!(turned.eval(&at(0.25, 0., 0.)), 1.);
        assert_eq!(turned.eval(&at(0.75, 0., 0.)), 0.);
        let doubled = graph(json!({"node": "transform", "scale": 2, "input": {"node": "gradient"}}));
        assert!((doubled.eval(&at(0., 3., 0.)) - 1.5).abs() < 1e-12);
        let moved = graph(json!({"node": "transform", "translate": [0, 1, 0], "input": {"node": "gradient"}}));
        assert!((moved.eval(&at(0., 3., 0.)) - 2.).abs() < 1e-12);
        // Rings about the y axis, and shells about the origin.
        assert!((graph(json!({"node": "rings"})).eval(&at(3., 7., 4.)) - 5.).abs() < 1e-12);
        assert!((graph(json!({"node": "spherical", "scale": 2})).eval(&at(0., 3., 4.)) - 2.5).abs() < 1e-12);
        // Seen from far off, fine bands are their mean.
        let fine = graph(json!({"node": "wave", "frequency": 100, "input": {"node": "gradient", "direction": [1, 0, 0]}}));
        let far = Context::new(Vector3::new(0.123, 0., 0.), Vector3::y(), 1.);
        assert!((fine.eval(&far) - 0.5).abs() < 0.01, "{}", fine.eval(&far));
        let checker = graph(json!({"node": "checker", "size": 0.5}));
        assert_eq!(checker.eval(&at(0.25, 0.25, 0.25)), 0.);
        assert_eq!(checker.eval(&at(0.75, 0.25, 0.25)), 1.);
        assert!(Graph::parse(&json!({"node": "wave", "shape": "zigzag", "input": 0})).is_err());
    }

    #[test]
    fn test_cell_styles_and_metrics() {
        let id = graph(json!({"node": "cells", "style": "id", "scale": 1, "jitter": 0}));
        // With no jitter, the cells are a lattice: points near one centre share its value.
        assert_eq!(id.eval(&at(0.4, 0.5, 0.6)), id.eval(&at(0.6, 0.45, 0.5)));
        assert_ne!(id.eval(&at(0.5, 0.5, 0.5)), id.eval(&at(1.5, 0.5, 0.5)));
        for metric in ["euclidean", "manhattan", "chebyshev"] {
            let f1 = graph(json!({"node": "cells", "style": "f1", "metric": metric}));
            let f2 = graph(json!({"node": "cells", "style": "f2", "metric": metric}));
            for k in 0 .. 50 {
                let c = at(k as f64 * 0.37, k as f64 * 0.11, k as f64 * 0.23);
                assert!(f1.eval(&c) <= f2.eval(&c), "{}", metric);
            }
        }
        let regular = graph(json!({"node": "cells", "style": "f1", "jitter": 0}));
        assert!(regular.eval(&at(0.5, 0.5, 0.5)) < 1e-12);
        assert!(Graph::parse(&json!({"node": "cells", "metric": "taxicab"})).is_err());
    }

    #[test]
    fn test_gabor_nodes() {
        let g = graph(json!({"node": "gabor", "wavelength": 0.1, "direction": [1, 0, 0], "amplitude": 2}));
        assert_eq!(g.range(), (-2., 2.));
        assert!((g.finest() - 0.1).abs() < 1e-12);
        let p = graph(json!({"node": "phasor", "frequency": 10, "profile": "saw", "plan": true}));
        let vs: Vec<f64> = (0 .. 200).map(|k| p.eval(&at(k as f64 * 0.0137, 0., k as f64 * 0.0071))).collect();
        assert!(vs.iter().all(|v| (-1. ..= 1.).contains(v)));
        assert!(vs.iter().any(|v| *v > 0.5) && vs.iter().any(|v| *v < -0.5));
        // In plan, the same at any height.
        assert_eq!(p.eval(&at(0.3, 0., 0.2)), p.eval(&at(0.3, 5., 0.2)));
        assert!(Graph::parse(&json!({"node": "gabor", "frequency": 0})).is_err());
    }

    #[test]
    fn test_values_stay_in_range() {
        let nodes = [
            json!({"node": "fractal", "scale": 3, "amplitude": 2}),
            json!({"node": "fractal", "style": "ridged", "scale": 3, "amplitude": 0.5}),
            json!({"node": "fractal", "style": "billow", "scale": 3}),
            json!({"node": "cells", "style": "stones", "scale": 0.5, "amplitude": 0.2}),
            json!({"node": "cells", "style": "edges", "scale": 0.5}),
            json!({"node": "warp", "amount": 2, "input": {"node": "fractal", "scale": 1}}),
            json!({"node": "terrace", "step": 0.5, "input": {"node": "fractal", "scale": 4, "amplitude": 3}}),
            json!({"node": "multiply", "inputs": [{"node": "slope", "from": 10, "to": 40}, {"node": "fractal", "amplitude": -1}]}),
            json!({"node": "power", "exponent": 2, "input": {"node": "fractal"}}),
            json!({"node": "strata", "step": 0.3, "breakup": {"node": "fractal", "scale": 2}}),
        ];
        for n in nodes {
            let g = graph(n.clone());
            let (lo, hi) = g.range();
            assert!(lo.is_finite() && hi.is_finite() && lo <= hi, "{}: {:?}", n, g.range());
            let mut rng = crate::geometry::Pcg32::new(1, 1);
            for _ in 0 .. 2000 {
                let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 20.;
                let v = g.eval(&at(p.x, p.y, p.z));
                assert!(v >= lo && v <= hi, "{}: {} outside {:?}", n, v, g.range());
            }
        }
    }

    /// Round a planet, slope, altitude and aspect are judged by the local up, not y.
    #[test]
    fn test_masks_follow_a_planet_round() {
        struct Ball;
        impl Fields for Ball {
            fn field(&self, _: Field, _: &Vector3<f64>) -> Option<f64> { None }
            fn vertical(&self, p: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> { Some((p.normalize(), p.norm() - 100.)) }
        }
        // On the ball's "equator", level ground faces +x, 5 units up.
        let c = Context::new(Vector3::new(105., 0., 0.), Vector3::x(), 0.).with_fields(Some(&Ball));
        assert!(c.slope() < 1e-6);
        assert_eq!(graph(json!({"node": "altitude", "from": 4, "to": 6})).eval(&c), 0.5);
        // Level ground faces no direction; a wall facing north (-z) does.
        let north = graph(json!({"node": "aspect"}));
        assert_eq!(north.eval(&c), 0.);
        let wall = Context::new(Vector3::new(105., 0., 0.), -Vector3::z(), 0.).with_fields(Some(&Ball));
        assert!((wall.slope() - 90.).abs() < 1e-9);
        assert_eq!(north.eval(&wall), 1.);
        // Without a vertical, y is up as before.
        assert!((at(105., 0., 0.).with_fields(None).slope()).abs() < 1e-9);
    }

    /// A remap of an unbounded field to one value has a NaN range by interval arithmetic
    /// (inf * 0); evaluating it must not panic.
    #[test]
    fn test_unbounded_remap_does_not_panic() {
        let g = Graph::parse(&json!({"node": "remap", "input": {"node": "field", "field": "wetness"},
            "from": [0, 10], "to": [1, 1]})).unwrap();
        let c = Context::new(Vector3::zeros(), Vector3::y(), 0.);
        assert_eq!(g.eval(&c), 1.);
    }

    #[test]
    fn test_arithmetic_and_expressions() {
        let g = graph(json!({"node": "add", "inputs": [1, {"node": "multiply", "inputs": [2, 3]},
            {"node": "subtract", "a": 1, "b": 4}]}));
        assert_eq!(g.eval(&at(0., 0., 0.)), 4.);
        assert_eq!(g.range(), (4., 4.));
        let e = graph(json!({"node": "expr", "expr": "a * x + y", "inputs": {"a": 2}, "range": [-100, 100]}));
        assert_eq!(e.eval(&at(3., 1., 0.)), 7.);
        assert_eq!(e.eval(&at(300., 1., 0.)), 100.);
        assert_eq!(graph(json!("x + z")).eval(&at(1., 0., 2.)), 3.);
        assert!(Graph::parse(&json!({"node": "frob"})).is_err());
        assert!(Graph::parse(&json!({"node": "blend", "a": 1})).err().unwrap().contains("\"b\""));
    }

    #[test]
    fn test_seeds_are_whole_numbers() {
        let fractal = |seed: Value| Graph::parse(&json!({"node": "fractal", "seed": seed}));
        assert!(fractal(json!(3)).is_ok());
        assert!(fractal(json!(3.0)).is_ok());
        assert!(fractal(json!(-1)).is_err());
        assert!(fractal(json!(2.5)).is_err());
        assert!(fractal(json!("7")).is_err());
        assert!(Graph::parse(&json!({"node": "cross_beds", "seed": -4})).is_err());
    }

    #[test]
    fn test_biome_nodes_pick_out_climates() {
        use crate::terrain::climate::Biome;
        // Taiga to the north (z < 0), grassland to the south.
        struct Zoned;
        impl Fields for Zoned {
            fn field(&self, _: Field, _: &Vector3<f64>) -> Option<f64> { None }
            fn biome(&self, p: &Vector3<f64>) -> Option<Biome> { Some(if p.z < 0. { Biome::Taiga } else { Biome::Grassland }) }
        }
        let forest = graph(json!({"node": "biome", "is": ["taiga", "temperate_forest"]}));
        assert_eq!(forest.eval(&at(0., 0., -5.).with_fields(Some(&Zoned))), 1.);
        assert_eq!(forest.eval(&at(0., 0., 5.).with_fields(Some(&Zoned))), 0.);
        // Without a climate, the default.
        assert_eq!(graph(json!({"node": "biome", "is": "taiga", "default": 1})).eval(&at(0., 0., 5.)), 1.);
        assert!(Graph::parse(&json!({"node": "biome", "is": "jungle"})).err().unwrap().contains("rainforest"));
    }

    struct Deep;
    impl Fields for Deep {
        fn field(&self, f: Field, p: &Vector3<f64>) -> Option<f64> {
            (f == Field::Sediment).then_some(p.x)
        }
    }

    #[test]
    fn test_fields_and_aspect() {
        let g = graph(json!({"node": "field", "field": "sediment", "from": 1, "to": 3}));
        assert_eq!(g.range(), (0., 1.));
        assert_eq!(g.eval(&at(5., 0., 0.).with_fields(Some(&Deep))), 1.);
        assert_eq!(g.eval(&at(2., 0., 0.).with_fields(Some(&Deep))), 0.5);
        // Off a surface with fields, the default.
        assert_eq!(g.eval(&at(5., 0., 0.)), 0.);
        let raw = graph(json!({"node": "field", "field": "ice", "default": 7}));
        assert_eq!(raw.eval(&at(5., 0., 0.).with_fields(Some(&Deep))), 7.);
        assert!(Graph::parse(&json!({"node": "field", "field": "mud"})).is_err());
        // A breakup moves a raw field as it does a banded one.
        let broken = graph(json!({"node": "field", "field": "sediment", "breakup": 0.5}));
        assert_eq!(broken.eval(&at(2., 0., 0.).with_fields(Some(&Deep))), 2.5);
        // A wall facing north (-z) faces north fully; level ground and a south wall don't.
        let north = graph(json!({"node": "aspect", "from": 0, "to": 1}));
        let facing = |n: Vector3<f64>| north.eval(&Context::new(Vector3::zeros(), n, 0.));
        assert_eq!(facing(-Vector3::z()), 1.);
        assert_eq!(facing(Vector3::y()), 0.);
        assert_eq!(facing(Vector3::z()), 0.);
        // Reversed edges fall.
        let gentle = graph(json!({"node": "slope", "from": 40, "to": 30}));
        assert_eq!(gentle.eval(&at(0., 0., 0.)), 1.);
    }

    #[test]
    fn test_masks() {
        let slope = graph(json!({"node": "slope", "from": 20, "to": 40}));
        let tilt = |deg: f64| Context::new(Vector3::zeros(), Vector3::new(deg.to_radians().sin(), deg.to_radians().cos(), 0.), 0.);
        assert_eq!(slope.eval(&tilt(10.)), 0.);
        assert_eq!(slope.eval(&tilt(60.)), 1.);
        assert!((slope.eval(&tilt(30.)) - 0.5).abs() < 1e-9);
        let alt = graph(json!({"node": "blend", "a": 1, "b": 3, "mask": {"node": "altitude", "from": 0, "to": 10}}));
        assert_eq!(alt.eval(&at(0., -5., 0.)), 1.);
        assert_eq!(alt.eval(&at(0., 15., 0.)), 3.);
        assert_eq!(alt.eval(&at(0., 5., 0.)), 2.);
    }

    #[test]
    fn test_terrace_steps() {
        let g = graph(json!({"node": "terrace", "step": 1, "sharpness": 1, "input": "x"}));
        assert!((g.eval(&at(2.3, 0., 0.)) - 2.).abs() < 1e-9);
        assert!((g.eval(&at(2.7, 0., 0.)) - 2.).abs() < 1e-9);
    }

    /// Filtering drops fine octaves without moving the surface: the filtered fractal follows
    /// the full one, and far coarser filters average to the same mean.
    #[test]
    fn test_filtering_keeps_the_mean() {
        for style in ["perlin", "ridged", "billow"] {
            let g = graph(json!({"node": "fractal", "style": style, "scale": 8, "smallest": 0.01}));
            let mut rng = crate::geometry::Pcg32::new(5, 2);
            let (mut full, mut coarse, mut diff, n) = (0., 0., 0f64, 4000);
            for _ in 0 .. n {
                let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 100.;
                let f = g.eval(&Context::new(p, Vector3::y(), 0.));
                let c = g.eval(&Context::new(p, Vector3::y(), 0.5));
                full += f;
                coarse += c;
                diff = diff.max((f - c).abs());
            }
            assert!(((full - coarse) / n as f64).abs() < 0.03, "{}: means {} and {}", style, full / n as f64, coarse / n as f64);
            assert!(diff < 0.5, "{}: filtering moved the surface by {}", style, diff);
        }
    }

    #[test]
    fn test_finest_detail() {
        let g = graph(json!({"node": "add", "inputs": [{"node": "fractal", "scale": 10, "smallest": 0.05}, {"node": "cells", "scale": 2}]}));
        assert!((g.finest() - 0.05).abs() < 1e-12);
        assert_eq!(graph(json!(3)).finest(), f64::INFINITY);
    }

    #[test]
    fn test_cross_beds_follow_the_rocks_laminae() {
        let g = graph(json!({"node": "cross_beds", "seed": 8, "scale": 30, "set": 0.12, "amplitude": 0.1}));
        let (lo, hi) = g.range();
        let noise = SeededNoise::new(8);
        let (mut sum, mut min, mut max, n) = (0., f64::INFINITY, f64::NEG_INFINITY, 4000);
        for i in 0 .. n {
            let p = Vector3::new(0.37 * i as f64, 0.011 * i as f64, 0.21 * i as f64);
            let v = g.eval(&at(p.x, p.y, p.z));
            // The rock material's laminae: ribs where they are hard.
            let l = crate::material::rock::cross_beds(&noise, p / 30., 0.12);
            assert!((v - 0.1 * (l.hardness - CROSS_BED_MEAN)).abs() < 1e-12);
            assert!(v >= lo && v <= hi);
            sum += v;
            min = min.min(v);
            max = max.max(v);
        }
        assert!((sum / n as f64).abs() < 0.015, "mean {}", sum / n as f64);
        assert!(max - min > 0.05);
        // Too fine for vertices a metre apart: nothing.
        assert_eq!(g.eval(&Context::new(Vector3::new(1., 2., 3.), Vector3::y(), 1.)), 0.);
        assert!((g.finest() - crate::material::rock::LAMINA_FINEST * 0.12 * 30.).abs() < 1e-12);
    }
}
