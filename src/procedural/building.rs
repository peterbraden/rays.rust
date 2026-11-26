//! # Buildings
//!
//! A start on procedural buildings: for now one kind, a log cottage in snow, built from parts
//! a builder of other buildings will share. A building is laid out in its own frame (x along
//! the ridge, z out of the front, y up, the origin on the ground at the middle of its
//! footprint) as **walls** (each a frame: along it, up, and out of it) with **openings** in
//! them (doors and windows, each fitted with a frame, a sash or planks), a **roof** over them,
//! a **chimney**, and a **plinth** of stone under it all; then **weather** settles on it: snow
//! lying on the roof and the sills, and icicles hanging from the eaves. Each part is a mesh in
//! a material of its own (`Part`), so a scene can dress the building as it likes, and each
//! window's light is a patch of its own, so the light BVH and the sampling of area lights
//! find it.
//!
//! The cottage is the Nordic and Alpine log house: walls of horizontal logs, the side walls'
//! courses half a log higher than the front and back walls' so that they cross at the
//! corners, saddle-notched, their ends running past (the Scandinavian *laftverk*; Terje
//! Planke, *Tradisjonelt tømmerhus*, 2013, has the joints). Scribed logs sit a little into
//! each other, so each course is 1.7 radii above the last. The gable ends are logs too, cut
//! shorter course by course to the roof's pitch, and the roof rests on purlins and a ridge
//! beam that run out through the gables and carry the overhang there. Openings are cut
//! through whole courses, so their heads and sills come at the seams between logs, and are
//! lined with planks that hide the logs' cut ends. Stone is laid in courses of rounded
//! blocks, proud of a mortar core.
//!
//! Snow on a roof (`snow_pad`) is a slab lying on its plane, as deep as `snow` give or take
//! a fifth, rounded over its edges as a cushion is, and at the eaves curling over the edge in
//! a cornice that droops as it reaches out (snow creeps down a roof and over its edge before
//! it breaks off: Ronald L. Sack, "Snow loads on sloped roofs", 1988, and the overhangs in
//! the Swiss snow-load surveys). Icicles hang where meltwater from the roof refreezes at the
//! cold eaves: many short, a few long, tapering to a point, rippled where each drip froze.
use crate::geometry::Pcg32;
use crate::na::{Rotation3, Vector3};
use crate::noise::PerlinNoise;
use crate::shapes::mesh::{vertex_normals, IndexedMesh};
use std::f64::consts::{PI, TAU};

type V = Vector3<f64>;

/// The parts of a building, each in a material of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Part {
    /// Logs lying along x (the front and back walls, purlins and ridge), and along z (the
    /// gable walls): apart so that a wood's grain can follow each.
    LogsAlongX,
    LogsAlongZ,
    Stone,
    Mortar,
    Roof,
    /// Boards round the openings, sashes, bargeboards.
    Trim,
    Door,
    Iron,
    Snow,
    Icicles,
    Wreath,
    Ribbon,
}

impl Part {
    pub const ALL: [Part; 12] = [Part::LogsAlongX, Part::LogsAlongZ, Part::Stone, Part::Mortar, Part::Roof,
        Part::Trim, Part::Door, Part::Iron, Part::Snow, Part::Icicles, Part::Wreath, Part::Ribbon];

    /// Its name in a scene's `"materials"`.
    pub fn name(&self) -> &'static str {
        match self {
            Part::LogsAlongX | Part::LogsAlongZ => "logs",
            Part::Stone => "stone",
            Part::Mortar => "mortar",
            Part::Roof => "roof",
            Part::Trim => "trim",
            Part::Door => "door",
            Part::Iron => "iron",
            Part::Snow => "snow",
            Part::Icicles => "icicles",
            Part::Wreath => "wreath",
            Part::Ribbon => "ribbon",
        }
    }
}

/// A frame in the building: a point and three axes, `u` and `v` across a surface and `n` out
/// of it.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub origin: V,
    pub u: V,
    pub v: V,
    pub n: V,
}

impl Frame {
    pub fn at(&self, u: f64, v: f64, n: f64) -> V {
        self.origin + self.u * u + self.v * v + self.n * n
    }
}

/// Meshes gathered by part.
#[derive(Default)]
pub struct Parts {
    pub meshes: std::collections::BTreeMap<Part, Vec<IndexedMesh>>,
}

impl Parts {
    pub fn add(&mut self, part: Part, mesh: IndexedMesh) {
        if !mesh.triangles.is_empty() {
            self.meshes.entry(part).or_default().push(mesh);
        }
    }

    pub fn triangles(&self) -> usize {
        self.meshes.values().flatten().map(|m| m.triangles.len()).sum()
    }
}

fn empty() -> IndexedMesh {
    IndexedMesh { positions: vec![], normals: vec![], triangles: vec![], texcoords: vec![] }
}

/// Add the triangle (a, b, c) of `m`'s vertices, wound so that its normal faces `towards`.
fn facing(m: &mut IndexedMesh, a: usize, b: usize, c: usize, towards: &V) {
    let p = &m.positions;
    let n = (p[b] - p[a]).cross(&(p[c] - p[a]));
    m.triangles.push(if n.dot(towards) >= 0. { [a, b, c] } else { [a, c, b] });
}

/// A flat quadrilateral, its own vertices, facing `towards`.
fn quad(m: &mut IndexedMesh, corners: [V; 4], towards: V) {
    let n = (corners[1] - corners[0]).cross(&(corners[2] - corners[0]));
    let n = if n.dot(&towards) >= 0. { n } else { -n };
    let n = n.try_normalize(1e-15).unwrap_or(towards);
    let k = m.positions.len();
    for c in corners {
        m.positions.push(c);
        m.normals.push(n);
    }
    facing(m, k, k + 1, k + 2, &n);
    facing(m, k, k + 2, k + 3, &n);
}

/// A box about `c` with half-extent vectors `a`, `b`, `e` (at right angles), flat faced.
pub fn cuboid(m: &mut IndexedMesh, c: V, a: V, b: V, e: V) {
    for (x, y, z) in [(a, b, e), (b, e, a), (e, a, b)] {
        for s in [1., -1.] {
            let f = c + x * s;
            quad(m, [f - y - z, f + y - z, f + y + z, f - y + z], x * s);
        }
    }
}

/// An axis-aligned box from `lo` to `hi`, in frame `f` (coordinates along u, v, n).
fn block(m: &mut IndexedMesh, f: &Frame, lo: V, hi: V) {
    let c = (lo + hi) / 2.;
    let h = (hi - lo) / 2.;
    cuboid(m, f.at(c.x, c.y, c.z), f.u * h.x, f.v * h.y, f.n * h.z);
}

/// Unit vectors at right angles to `d` (a unit vector).
fn perpendiculars(d: &V) -> (V, V) {
    let a = if d.x.abs() < 0.9 { V::x() } else { V::y() };
    let p = d.cross(&a).normalize();
    (p, d.cross(&p))
}

/// A tube from `a` to `b`, its radius `radius(s, angle)` at `s` from 0 to 1 along it; closed
/// by flat caps where the radius isn't zero.
fn tube(m: &mut IndexedMesh, a: V, b: V, around: usize, along: usize, radius: impl Fn(f64, f64) -> f64) {
    let d = (b - a).normalize();
    let (p, q) = perpendiculars(&d);
    let k = m.positions.len();
    for j in 0 ..= along {
        let s = j as f64 / along as f64;
        let centre = a + (b - a) * s;
        for i in 0 .. around {
            let t = i as f64 / around as f64 * TAU;
            let r = radius(s, t);
            let out = p * t.cos() + q * t.sin();
            m.positions.push(centre + out * r);
            m.normals.push(out);
        }
    }
    for j in 0 .. along {
        for i in 0 .. around {
            let (i1, r0, r1) = ((i + 1) % around, k + j * around, k + (j + 1) * around);
            let out = m.positions[r0 + i] - (a + (b - a) * (j as f64 / along as f64));
            let out = out - d * out.dot(&d);
            facing(m, r0 + i, r0 + i1, r1 + i1, &out);
            facing(m, r0 + i, r1 + i1, r1 + i, &out);
        }
    }
    for (j, outward) in [(0, -d), (along, d)] {
        if radius(j as f64 / along as f64, 0.) <= 1e-9 { continue; }
        let centre = a + (b - a) * (j as f64 / along as f64);
        let c = m.positions.len();
        m.positions.push(centre);
        m.normals.push(outward);
        let ring = m.positions.len();
        for i in 0 .. around {
            let p = m.positions[k + j * around + i];
            m.positions.push(p);
            m.normals.push(outward);
        }
        for i in 0 .. around {
            facing(m, c, ring + i, ring + (i + 1) % around, &outward);
        }
    }
}

/// A torus about `centre`, in the plane at right angles to `axis`, with a tube of radius
/// `tube_radius(angle round, angle about the tube)`.
fn torus(m: &mut IndexedMesh, centre: V, axis: V, major: f64, around: usize, about: usize, tube_radius: impl Fn(f64, f64) -> f64) {
    let (p, q) = perpendiculars(&axis.normalize());
    let k = m.positions.len();
    for i in 0 .. around {
        let a = i as f64 / around as f64 * TAU;
        let out = p * a.cos() + q * a.sin();
        for j in 0 .. about {
            let b = j as f64 / about as f64 * TAU;
            let dir = out * b.cos() + axis.normalize() * b.sin();
            m.positions.push(centre + out * major + dir * tube_radius(a, b));
            m.normals.push(dir);
        }
    }
    for i in 0 .. around {
        for j in 0 .. about {
            let (i1, j1) = ((i + 1) % around, (j + 1) % about);
            let at = |i: usize, j: usize| k + i * about + j;
            let a = i as f64 / around as f64 * TAU;
            let b = j as f64 / about as f64 * TAU;
            let out = (p * a.cos() + q * a.sin()) * b.cos() + axis.normalize() * b.sin();
            facing(m, at(i, j), at(i1, j), at(i1, j1), &out);
            facing(m, at(i, j), at(i1, j1), at(i, j1), &out);
        }
    }
}

/// A rounded block: a superellipsoid of half-extents `half` (exponent `p`: 2 a sphere, larger
/// boxier) along the frame's axes about `centre`, its surface roughened by `rough` (scene
/// units) of noise.
fn rounded_block(centre: V, axes: [V; 3], half: V, p: f64, rough: f64, noise: &PerlinNoise, detail: u32) -> IndexedMesh {
    let (dirs, triangles) = crate::procedural::rock::icosphere(detail);
    let positions: Vec<V> = dirs.iter().map(|d| {
        let d = d.normalize();
        let s: f64 = (0 .. 3).map(|i| (d[i] / half[i]).abs().powf(p)).sum();
        let local = d * s.powf(-1. / p);
        let world = centre + axes[0] * local.x + axes[1] * local.y + axes[2] * local.z;
        let w = world * 9.;
        let bump = (noise.fbm(w.x, w.y, w.z, 3, 0.5, 2.) - 0.5) * 2. * rough;
        let outward = (axes[0] * (local.x / half.x / half.x) + axes[1] * (local.y / half.y / half.y)
            + axes[2] * (local.z / half.z / half.z)).normalize();
        world + outward * bump
    }).collect();
    let normals = vertex_normals(&positions, &triangles);
    IndexedMesh { positions, normals, triangles, texcoords: vec![] }
}

/// Stones laid in courses about the sides of the box from `lo` to `hi` (in the building's
/// frame), each stone a rounded block standing up to `proud` out of the faces, and a mortar
/// core behind them. Only the sides are faced; the top and bottom are mortar.
fn masonry(parts: &mut Parts, lo: V, hi: V, course: f64, rng: &mut Pcg32, noise: &PerlinNoise) {
    let proud = 0.015;
    let mut core = empty();
    let inset = 0.012;
    cuboid(&mut core, (lo + hi) / 2., V::x() * ((hi.x - lo.x) / 2. - inset), V::y() * (hi.y - lo.y) / 2.,
        V::z() * ((hi.z - lo.z) / 2. - inset));
    parts.add(Part::Mortar, core);
    let mut stones = empty();
    let courses = ((hi.y - lo.y) / course).round().max(1.) as usize;
    let height = (hi.y - lo.y) / courses as f64;
    let depth = 0.14_f64.min((hi.x - lo.x).min(hi.z - lo.z) / 2.);
    // The four faces, as (a corner, along, out, length).
    let faces = [
        (V::new(lo.x, 0., hi.z), V::x(), V::z(), hi.x - lo.x),
        (V::new(hi.x, 0., hi.z), -V::z(), V::x(), hi.z - lo.z),
        (V::new(hi.x, 0., lo.z), -V::x(), -V::z(), hi.x - lo.x),
        (V::new(lo.x, 0., lo.z), V::z(), -V::x(), hi.z - lo.z),
    ];
    for j in 0 .. courses {
        let y = lo.y + (j as f64 + 0.5) * height;
        for (k, &(corner, along, out, length)) in faces.iter().enumerate() {
            // Courses take the corner in turn from one face and the next, as quoins do.
            let mut s = if (j + k) % 2 == 0 { 0. } else { depth * 0.8 };
            let end = length - if (j + k) % 2 == 0 { depth * 0.8 } else { 0. };
            while s < end - 0.05 {
                let mut l = height * (1.2 + rng.next_f64() * 1.6);
                if end - (s + l) < height * 0.6 { l = end - s; }
                let gap = 0.008 + rng.next_f64() * 0.006;
                let h = height / 2. - gap - rng.next_f64() * height * 0.08;
                let centre = corner + along * (s + l / 2.) + V::y() * (y + (rng.next_f64() - 0.5) * height * 0.08)
                    + out * (proud * rng.next_f64() - depth / 2.);
                let half = V::new((l / 2. - gap).max(0.02), h.max(0.02), depth / 2.);
                // A little turned and tipped, as stones are laid by hand.
                let r = Rotation3::from_euler_angles((rng.next_f64() - 0.5) * 0.06, (rng.next_f64() - 0.5) * 0.06, (rng.next_f64() - 0.5) * 0.05);
                let axes = [r * along, r * V::y(), r * out];
                let stone = rounded_block(centre, axes, half, 3.5 + rng.next_f64() * 2., 0.006, noise, 2);
                let base = stones.positions.len();
                stones.positions.extend(stone.positions);
                stones.normals.extend(stone.normals);
                stones.triangles.extend(stone.triangles.iter().map(|t| t.map(|i| i + base)));
                s += l;
            }
        }
    }
    parts.add(Part::Stone, stones);
}

/// Snow lying on a rectangle (`u0` to `u1` by `v0` to `v1` of `frame`, `n` up out of it), as
/// deep as `depth`, give or take a fifth, and rounded over its edges; along the `v1` edge
/// curling `lip` beyond it and drooping, a cornice. With `ridge`, the `v0` edge is a ridge: the
/// snow runs right to it, and its top there rises straight up (along `up`) to meet the snow
/// on the roof's other side.
#[allow(clippy::too_many_arguments)]
pub fn snow_pad(frame: &Frame, (u0, u1): (f64, f64), (v0, v1): (f64, f64), depth: f64, lip: f64,
                ridge: Option<V>, noise: &PerlinNoise, spacing: f64) -> IndexedMesh {
    let nu = ((u1 - u0) / spacing).ceil().max(2.) as usize;
    let length = v1 + lip - v0;
    let nv = (length / spacing).ceil().max(2.) as usize;
    let radius = depth * 1.3;
    let edge = |d: f64| -> f64 {
        if d <= 0. { return 0.; }
        let x = (d / radius).min(1.);
        (1. - (1. - x) * (1. - x)).sqrt()
    };
    let droop = 0.5 * depth;
    let mut m = empty();
    let mut grid = |top: bool| {
        let k = m.positions.len();
        for j in 0 ..= nv {
            let v = v0 + length * j as f64 / nv as f64;
            for i in 0 ..= nu {
                let u = u0 + (u1 - u0) * i as f64 / nu as f64;
                let p = frame.at(u, v, 0.);
                let w = p * 1.1;
                let deep = depth * (1. + 0.4 * (noise.fbm(w.x, w.y, w.z, 3, 0.5, 2.) - 0.5));
                let across = edge(u - u0) * edge(u1 - u);
                let from = if ridge.is_some() { 1. } else { edge(v - v0) };
                let (hi, lo) = if v <= v1 || lip <= 0. {
                    let to = if lip > 0. { 1. } else { edge(v1 - v) };
                    (deep * across * from * to, 0.)
                } else {
                    let t = ((v - v1) / lip).min(1.);
                    let curl = droop * t * t;
                    (deep * across * (1. - t * t).max(0.).sqrt() - curl,
                     -curl - 0.3 * deep * across * (PI * t).sin())
                };
                let h = if top { hi } else { lo };
                m.positions.push(match (ridge, j) {
                    (Some(up), 0) if top => p + up * (h / up.dot(&frame.n)),
                    _ => p + frame.n * h,
                });
            }
        }
        k
    };
    let top = grid(true);
    let bottom = grid(false);
    let at = |base: usize, i: usize, j: usize| base + j * (nu + 1) + i;
    for j in 0 .. nv {
        for i in 0 .. nu {
            for (base, out) in [(top, frame.n), (bottom, -frame.n)] {
                facing(&mut m, at(base, i, j), at(base, i + 1, j), at(base, i + 1, j + 1), &out);
                facing(&mut m, at(base, i, j), at(base, i + 1, j + 1), at(base, i, j + 1), &out);
            }
        }
    }
    m.normals = vertex_normals(&m.positions, &m.triangles);
    // Closed at the ridge, by faces of their own, so as not to bend the top's normals.
    if ridge.is_some() {
        for i in 0 .. nu {
            let corners = [(top, i), (top, i + 1), (bottom, i + 1), (bottom, i)].map(|(base, i)| m.positions[at(base, i, 0)]);
            quad(&mut m, corners, -frame.v);
        }
    }
    m
}

/// An icicle hanging from `top`, `length` long and `radius` thick at the top: tapering to a
/// point, rippled, and bending a little as it grew.
fn icicle(m: &mut IndexedMesh, top: V, length: f64, radius: f64, rng: &mut Pcg32) {
    let phase = rng.next_f64() * TAU;
    let ripples = 4. + rng.next_f64() * 6.;
    let lean = V::new(rng.next_f64() - 0.5, 0., rng.next_f64() - 0.5) * 0.08 * length;
    let along = ((length / 0.02).ceil() as usize).clamp(6, 24);
    let start = m.positions.len();
    tube(m, top, top - V::y() * length, 7, along, |s, _| {
        radius * (1. - s).powf(1.3) * (1. + 0.15 * (TAU * ripples * s + phase).sin())
    });
    // The bend, more towards the tip.
    for k in start .. m.positions.len() {
        let s = ((top.y - m.positions[k].y) / length).clamp(0., 1.);
        m.positions[k] += lean * s * s;
    }
}

/// A door or a window, in a wall's frame: from `u0` to `u1` along it, `y0` to `y1` up.
#[derive(Clone, Copy, Debug)]
pub struct Opening {
    pub kind: OpeningKind,
    pub centre: f64,
    pub width: f64,
    /// Its bottom and top, asked for; cut to the seams between logs.
    pub bottom: f64,
    pub top: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpeningKind { Door, Window }

/// A wall of logs: its frame (`u` along it from the middle, `v` up, `n` out), half its length
/// between the centres of the walls it meets, and whether it is a gable wall (along z, its
/// courses half a log higher, rising to the roof).
struct Wall {
    frame: Frame,
    half: f64,
    gable: bool,
    openings: Vec<Opening>,
}

/// A log cottage, see the module's documentation. Lengths in metres.
#[derive(Clone, Debug)]
pub struct Cottage {
    /// Along the ridge, and across it, between the walls' centres.
    pub width: f64,
    pub depth: f64,
    /// The walls' height above the plinth, to the eaves.
    pub wall_height: f64,
    /// The roof's pitch, in degrees.
    pub pitch: f64,
    /// How far the roof reaches past the walls at the eaves, and at the gables.
    pub overhang: f64,
    pub gable_overhang: f64,
    pub log_radius: f64,
    /// How high the stone plinth stands out of the ground.
    pub plinth: f64,
    /// The depth of snow on the roof (0 for none).
    pub snow: f64,
    /// Icicles along the eaves: 0 none, 1 many.
    pub icicles: f64,
    pub chimney: bool,
    pub wreath: bool,
    pub seed: u64,
}

impl Default for Cottage {
    fn default() -> Cottage {
        Cottage {
            width: 7., depth: 5., wall_height: 2.5, pitch: 38., overhang: 0.6, gable_overhang: 0.5,
            log_radius: 0.13, plinth: 0.4, snow: 0.3, icicles: 0.5, chimney: true, wreath: true, seed: 0,
        }
    }
}

/// What a building is made of: its parts' meshes, and its windows' lights (each a
/// quadrilateral, corners in the order of `BilinearPatch::new`, facing out), in its own frame.
pub struct Built {
    pub parts: Parts,
    pub windows: Vec<[V; 4]>,
}

impl Cottage {
    /// The vertical distance between courses.
    fn course(&self) -> f64 { 1.7 * self.log_radius }

    fn tan(&self) -> f64 { self.pitch.clamp(5., 70.).to_radians().tan() }

    /// The top of the walls' highest logs, where the roof rests.
    pub fn wall_top(&self) -> f64 {
        let r = self.log_radius;
        let n = ((self.wall_height - 2. * r) / self.course()).floor().max(0.);
        self.plinth + 2. * r + n * self.course()
    }

    /// The height of the roof's underside over the ridge: it rests on the top logs of the
    /// front and back walls, touching them along their outer sides.
    pub fn ridge(&self) -> f64 {
        let r = self.log_radius;
        let cos = self.tan().atan().cos();
        self.wall_top() - r + self.depth / 2. * self.tan() + r / cos
    }

    /// The roof's underside at a distance `z` from the ridge.
    fn under_roof(&self, z: f64) -> f64 { self.ridge() - z.abs() * self.tan() }

    fn walls(&self) -> Vec<Wall> {
        let (w, d) = (self.width / 2., self.depth / 2.);
        let wall = |origin: V, u: V, n: V, half: f64, gable: bool| Wall {
            frame: Frame { origin, u, v: V::y(), n }, half, gable, openings: vec![],
        };
        let mut walls = vec![
            wall(V::new(0., 0., d), V::x(), V::z(), w, false),
            wall(V::new(0., 0., -d), -V::x(), -V::z(), w, false),
            wall(V::new(w, 0., 0.), -V::z(), V::x(), d, true),
            wall(V::new(-w, 0., 0.), V::z(), -V::x(), d, true),
        ];
        let base = self.plinth;
        let window = |centre: f64| Opening { kind: OpeningKind::Window, centre, width: 0.85, bottom: base + 0.85, top: base + 1.85 };
        // The front: a door in the middle and a window either side, if there is room.
        walls[0].openings.push(Opening { kind: OpeningKind::Door, centre: 0., width: 0.9, bottom: base, top: base + 1.95 });
        if self.width >= 4.5 {
            let x = (self.width / 4.).max(1.4);
            walls[0].openings.push(window(-x));
            walls[0].openings.push(window(x));
        }
        // A window at the back, and in each gable.
        walls[1].openings.push(window(0.));
        walls[2].openings.push(window(0.));
        walls[3].openings.push(window(0.));
        walls
    }

    pub fn build(&self) -> Built {
        let mut rng = Pcg32::new(self.seed, 0x60_77a6e);
        let noise = PerlinNoise::shuffled(self.seed.wrapping_add(17));
        let mut parts = Parts::default();
        let mut windows = vec![];
        let r = self.log_radius;
        let s = self.course();
        let e = 2.2 * r;
        let top = self.wall_top();
        let tan = self.tan();
        let (cos, sin) = { let t = tan.atan(); (t.cos(), t.sin()) };

        // The walls.
        let mut logs_x = empty();
        let mut logs_z = empty();
        for mut wall in self.walls() {
            let f = wall.frame;
            let mut courses = vec![];
            let mut k = if wall.gable { -1 } else { 0 };
            loop {
                let y = self.plinth + r + (k as f64 + if wall.gable { 0.5 } else { 0. }) * s;
                // How far it reaches: past the corners, but under the roof.
                let under = (self.ridge() - y - r) / tan;
                let half = if wall.gable { (wall.half + e).min(under) } else { wall.half + e };
                if y + r > top + 1e-9 && (!wall.gable || half < 0.3) { break; }
                courses.push((y, half));
                k += 1;
            }
            // Cut the openings to the seams between the courses.
            for o in wall.openings.iter_mut() {
                let inside = |y: f64| y > o.bottom && y < o.top;
                let below = courses.iter().map(|c| c.0).filter(|&y| !inside(y) && y <= o.bottom).fold(f64::NEG_INFINITY, f64::max);
                let above = courses.iter().map(|c| c.0).filter(|&y| !inside(y) && y >= o.top).fold(f64::INFINITY, f64::min);
                if below.is_finite() { o.bottom = below + s / 2.; } else { o.bottom = self.plinth; }
                if above.is_finite() { o.top = above - s / 2.; }
            }
            for &(y, half) in &courses {
                // The log, less the openings it crosses.
                let mut pieces = vec![(-half, half)];
                for o in &wall.openings {
                    if y > o.bottom && y < o.top {
                        let (a, b) = (o.centre - o.width / 2., o.centre + o.width / 2.);
                        pieces = pieces.into_iter().flat_map(|(p, q)| {
                            if b <= p || a >= q { vec![(p, q)] } else { vec![(p, a.max(p)), (b.min(q), q)] }
                        }).filter(|(p, q)| q - p > 0.1).collect();
                    }
                }
                for (p, q) in pieces {
                    let thick = r * (1. + (rng.next_f64() - 0.5) * 0.14);
                    let taper = (rng.next_f64() - 0.5) * 0.1;
                    let oval = rng.next_f64() * TAU;
                    let phase = rng.next_f64() * TAU;
                    let length = q - p;
                    let mesh = if f.u.x.abs() > 0.5 { &mut logs_x } else { &mut logs_z };
                    tube(mesh, f.at(p, y, 0.), f.at(q, y, 0.), 16, ((length / 0.35).ceil() as usize).max(1), |t, a| {
                        thick * (1. + taper * (t - 0.5)) * (1. + 0.035 * (2. * (a - oval)).cos())
                            * (1. + 0.02 * (t * length * 2.3 + phase).sin())
                    });
                }
            }
            for o in &wall.openings {
                self.fit(&mut parts, &mut windows, &f, o, &noise, &mut rng);
            }
        }

        // Purlins and the ridge beam, under the roof, running out through the gables.
        let reach = self.width / 2. + self.gable_overhang - 0.05;
        for z in [-self.depth / 4., 0., self.depth / 4.] {
            let y = self.under_roof(z) - r * 0.9 / cos;
            tube(&mut logs_x, V::new(-reach, y, z), V::new(reach, y, z), 16, 12, |_, _| r * 0.9);
        }
        parts.add(Part::LogsAlongX, logs_x);
        parts.add(Part::LogsAlongZ, logs_z);

        // The plinth.
        masonry(&mut parts, V::new(-self.width / 2. - r * 1.2, -0.3, -self.depth / 2. - r * 1.2),
            V::new(self.width / 2. + r * 1.2, self.plinth, self.depth / 2. + r * 1.2), 0.23, &mut rng, &noise);

        // The roof: a slab each side, mitred at the ridge.
        let thick = 0.12;
        let slope = (self.depth / 2. + self.overhang) / cos;
        let half = self.width / 2. + self.gable_overhang;
        let mut roof = empty();
        let mut trim = empty();
        let mut snow = empty();
        let mut icicles = empty();
        for side in [1., -1.] {
            let down = V::new(0., -sin, cos * side);
            let up = V::new(0., cos, sin * side);
            let ridge = V::new(0., self.ridge(), 0.);
            let crest = ridge + V::y() * (thick / cos);
            let eave = ridge + down * slope;
            let eave_top = eave + up * thick;
            // The slab's cross-section, extruded along the ridge: its top, underside, eave
            // edge, the mitre at the ridge, and its two ends.
            let (a, b, c, d) = (ridge, crest, eave_top, eave);
            let x = |p: V, s: f64| p + V::x() * (half * s);
            quad(&mut roof, [x(b, -1.), x(b, 1.), x(c, 1.), x(c, -1.)], up);
            quad(&mut roof, [x(a, -1.), x(a, 1.), x(d, 1.), x(d, -1.)], -up);
            quad(&mut roof, [x(c, -1.), x(c, 1.), x(d, 1.), x(d, -1.)], down);
            quad(&mut roof, [x(a, -1.), x(a, 1.), x(b, 1.), x(b, -1.)], -down);
            quad(&mut roof, [x(a, 1.), x(b, 1.), x(c, 1.), x(d, 1.)], V::x());
            quad(&mut roof, [x(a, -1.), x(b, -1.), x(c, -1.), x(d, -1.)], -V::x());
            // Bargeboards along the gables' edges.
            for end in [-1., 1.] {
                let centre = ridge + down * (slope / 2.) + up * (thick / 2. - 0.03) + V::x() * (end * (half + 0.015));
                cuboid(&mut trim, centre, V::x() * 0.015, down * (slope / 2. + 0.01), up * (thick / 2. + 0.06));
            }
            if self.snow > 0. {
                let frame = Frame { origin: crest, u: V::x(), v: down, n: up };
                snow.append_mesh(snow_pad(&frame, (-half - 0.04, half + 0.04), (0., slope + thick * tan), self.snow,
                    0.8 * self.snow, Some(V::y()), &noise, 0.07));
            }
            // Icicles along the eave: from under the snow's cornice, where it curls over the
            // edge, or with no snow from under the edge itself.
            if self.icicles > 0. {
                let from = if self.snow > 0. {
                    eave_top + down * (0.5 * 0.8 * self.snow) - up * (0.3 * self.snow)
                } else {
                    eave - down * 0.04 + up * 0.02
                };
                let mut u = -half + 0.1;
                loop {
                    u += (0.03 + rng.next_f64() * 0.25) / self.icicles.clamp(0.05, 1.);
                    if u >= half - 0.1 { break; }
                    let length = 0.05 + 0.7 * rng.next_f64().powf(2.5) * (0.4 + self.icicles);
                    let radius = (length * (0.1 + 0.06 * rng.next_f64())).clamp(0.006, 0.04);
                    icicle(&mut icicles, from + V::x() * u, length, radius, &mut rng);
                }
            }
        }
        parts.add(Part::Roof, roof);

        // The chimney, through the back of the roof near one gable.
        if self.chimney {
            let (cx, cz, w) = (self.width / 2. - 1.1, -0.45, 0.36);
            let height = self.ridge() + thick / cos + 0.8;
            masonry(&mut parts, V::new(cx - w, top - 0.3, cz - w), V::new(cx + w, height, cz + w), 0.2, &mut rng, &noise);
            let cap = V::new(cx, height + 0.04, cz);
            let mut slab = empty();
            cuboid(&mut slab, cap, V::x() * (w + 0.06), V::y() * 0.04, V::z() * (w + 0.06));
            parts.add(Part::Stone, slab);
            if self.snow > 0. {
                let frame = Frame { origin: cap + V::y() * 0.04, u: V::x(), v: V::z(), n: V::y() };
                snow.append_mesh(snow_pad(&frame, (-w - 0.08, w + 0.08), (-w - 0.08, w + 0.08), self.snow * 0.6, 0., None, &noise, 0.04));
            }
        }
        if self.snow > 0. {
            self.drifts(&mut snow, &noise);
        }
        parts.add(Part::Trim, trim);
        parts.add(Part::Snow, snow);
        parts.add(Part::Icicles, icicles);
        Built { parts, windows }
    }

    /// Snow heaped on the ground: in a bank along each eave's drip line, where what slides off
    /// the roof lands, and a little against the foot of every wall; cleared from the doorway.
    fn drifts(&self, snow: &mut IndexedMesh, noise: &PerlinNoise) {
        let r = self.log_radius;
        let (w, d) = (self.width / 2., self.depth / 2.);
        let ground = -0.02;
        let door = 0.75;
        for side in [1., -1.] {
            // Under the eave: as long as the roof, out where its edge drips.
            let frame = Frame { origin: V::new(0., ground, side * (d + self.overhang + r)), u: V::x(), v: V::z() * side, n: V::y() };
            let reach = w + self.gable_overhang;
            let spans: Vec<(f64, f64)> = if side > 0. { vec![(-reach, -door), (door, reach)] } else { vec![(-reach, reach)] };
            for span in spans {
                snow.append_mesh(snow_pad(&frame, span, (-0.55, 0.55), self.snow * 1.3, 0., None, noise, 0.06));
            }
        }
        // At the foot of each wall, over the plinth's lowest course.
        let foot = 0.45;
        for (origin, u, v, half, doorway) in [
            (V::new(0., ground, d + r), V::x(), V::z(), w + r, true),
            (V::new(0., ground, -d - r), V::x(), -V::z(), w + r, false),
            (V::new(w + r, ground, 0.), V::z(), V::x(), d + r + foot, false),
            (V::new(-w - r, ground, 0.), V::z(), -V::x(), d + r + foot, false),
        ] {
            let frame = Frame { origin, u, v, n: V::y() };
            let spans: Vec<(f64, f64)> = if doorway { vec![(-half, -door), (door, half)] } else { vec![(-half, half)] };
            for span in spans {
                snow.append_mesh(snow_pad(&frame, span, (-0.2, foot), self.snow * 0.8, 0., None, noise, 0.06));
            }
        }
    }

    /// Fit an opening: line it with planks over the logs' cut ends; then a sash, its panes lit
    /// from within, and a sill with snow on it; or a door of planks, with a ring for a handle,
    /// and a wreath.
    fn fit(&self, parts: &mut Parts, windows: &mut Vec<[V; 4]>, f: &Frame, o: &Opening, noise: &PerlinNoise, rng: &mut Pcg32) {
        let r = self.log_radius;
        let (u0, u1) = (o.centre - o.width / 2., o.centre + o.width / 2.);
        let board = 0.11;
        let (front, back) = (r + 0.03, -r - 0.01);
        let mut trim = empty();
        let door = o.kind == OpeningKind::Door;
        // The opening, inside the lining.
        let (y0, y1) = (if door { o.bottom + 0.03 } else { o.bottom + 0.04 }, o.top - 0.04);
        let (c0, c1) = (u0 + 0.03, u1 - 0.03);
        // Jambs, head and sill.
        block(&mut trim, f, V::new(u0 - board, y0 - 0.06, back), V::new(c0, y1 + board, front));
        block(&mut trim, f, V::new(c1, y0 - 0.06, back), V::new(u1 + board, y1 + board, front));
        block(&mut trim, f, V::new(u0 - board, y1, back), V::new(u1 + board, y1 + board, front));
        if door {
            block(&mut trim, f, V::new(u0 - board, y0 - 0.06, back), V::new(u1 + board, y0, front + 0.04));
        } else {
            let sill = V::new(u1 + board + 0.05, y0, front + 0.09);
            block(&mut trim, f, V::new(u0 - board - 0.05, y0 - 0.06, back), sill);
            if self.snow > 0. {
                let top = Frame { origin: f.at(o.centre, y0, (back + sill.z) / 2.), u: f.u, v: f.n, n: V::y() };
                let half = (sill.z - back) / 2.;
                parts.add(Part::Snow, snow_pad(&top, (-o.width / 2. - board - 0.05, o.width / 2. + board + 0.05),
                    (-half, half + 0.01), (self.snow * 0.25).min(0.08), 0., None, noise, 0.02));
            }
        }
        if door {
            // Planks, upright, a backing behind their gaps, and battens across.
            let mut planks = empty();
            let n = ((c1 - c0) / 0.15).round().max(3.) as usize;
            let w = (c1 - c0) / n as f64;
            for i in 0 .. n {
                let a = c0 + w * i as f64;
                let proud = 0.01 * rng.next_f64();
                block(&mut planks, f, V::new(a + 0.003, y0, -0.02), V::new(a + w - 0.003, y1, 0.03 + proud));
            }
            block(&mut planks, f, V::new(c0, y0, -0.05), V::new(c1, y1, -0.02));
            parts.add(Part::Door, planks);
            // A ring for a handle.
            let mut iron = empty();
            let at = f.at(c1 - 0.13, y0 + 1.0, 0.045);
            torus(&mut iron, at - f.v * 0.04, f.u, 0.04, 16, 8, |_, _| 0.006);
            block(&mut iron, f, V::new(c1 - 0.16, y0 + 0.98, 0.03), V::new(c1 - 0.10, y0 + 1.02, 0.045));
            parts.add(Part::Iron, iron);
            if self.wreath {
                self.wreath(parts, f, f.at(o.centre, y0 + 1.5, 0.08), rng);
            }
        } else {
            // The sash: a frame and glazing bars dividing it in four, in front of the panes'
            // light.
            let bar = 0.045;
            let (z0, z1) = (-0.03, 0.03);
            block(&mut trim, f, V::new(c0, y0, z0), V::new(c0 + bar, y1, z1));
            block(&mut trim, f, V::new(c1 - bar, y0, z0), V::new(c1, y1, z1));
            block(&mut trim, f, V::new(c0, y0, z0), V::new(c1, y0 + bar, z1));
            block(&mut trim, f, V::new(c0, y1 - bar, z0), V::new(c1, y1, z1));
            let (mu, my) = ((c0 + c1) / 2., (y0 + y1) / 2.);
            block(&mut trim, f, V::new(mu - 0.015, y0, z0 + 0.01), V::new(mu + 0.015, y1, z1 - 0.005));
            block(&mut trim, f, V::new(c0, my - 0.015, z0 + 0.01), V::new(c1, my + 0.015, z1 - 0.005));
            let z = -0.02;
            windows.push([f.at(c0, y0, z), f.at(c1, y0, z), f.at(c0, y1, z), f.at(c1, y1, z)]);
            // Behind the light, so nothing is seen through its back.
            let mut back = empty();
            block(&mut back, f, V::new(c0, y0, z - 0.03), V::new(c1, y1, z - 0.01));
            parts.add(Part::Trim, back);
        }
        parts.add(Part::Trim, trim);
    }

    /// A wreath of fir hung on the door, tied with a bow, with berries.
    fn wreath(&self, parts: &mut Parts, f: &Frame, centre: V, rng: &mut Pcg32) {
        let noise = PerlinNoise::shuffled(self.seed.wrapping_add(91));
        let mut fir = empty();
        torus(&mut fir, centre, f.n, 0.2, 72, 14, |a, b| {
            let p = V::new(a.cos() * 3., a.sin() * 3., b.cos() * 2.) * 4.;
            0.06 * (0.7 + 0.8 * noise.fbm(p.x, p.y, p.z, 3, 0.6, 2.2))
        });
        fir.normals = vertex_normals(&fir.positions, &fir.triangles);
        parts.add(Part::Wreath, fir);
        let mut ribbon = empty();
        let knot = centre - f.v * 0.2 + f.n * 0.05;
        cuboid(&mut ribbon, knot, f.u * 0.035, f.v * 0.03, f.n * 0.02);
        for side in [-1., 1.] {
            // The bow's loops, and tails hanging from the knot.
            let loop_centre = knot + f.u * (side * 0.065) + f.v * 0.01;
            torus(&mut ribbon, loop_centre, f.v, 0.04, 16, 6, |_, b| 0.012 * (1. + 0.3 * b.cos()));
            let tail = knot - f.v * 0.08 + f.u * (side * 0.03);
            let r = Rotation3::from_axis_angle(&crate::na::Unit::new_normalize(f.n), side * 0.25);
            cuboid(&mut ribbon, tail, r * f.u * 0.016, r * f.v * 0.08, f.n * 0.004);
        }
        // Berries, in clusters about the ring.
        for k in 0 .. 9 {
            let a = (k as f64 + rng.next_f64() * 0.5) / 9. * TAU;
            let at = centre + (f.u * a.cos() + f.v * a.sin()) * 0.2 + f.n * 0.07;
            for _ in 0 .. 3 {
                let jitter = (f.u * (rng.next_f64() - 0.5) + f.v * (rng.next_f64() - 0.5)) * 0.04;
                let berry = rounded_block(at + jitter, [f.u, f.v, f.n], V::repeat(0.012), 2., 0., &noise, 1);
                ribbon.append_mesh(berry);
            }
        }
        parts.add(Part::Ribbon, ribbon);
    }
}

/// Appending one mesh to another.
pub trait AppendMesh {
    fn append_mesh(&mut self, other: IndexedMesh);
}

impl AppendMesh for IndexedMesh {
    fn append_mesh(&mut self, other: IndexedMesh) {
        let base = self.positions.len();
        let normals = if other.normals.len() == other.positions.len() { other.normals } else { vertex_normals(&other.positions, &other.triangles) };
        if self.normals.len() != self.positions.len() {
            self.normals = vertex_normals(&self.positions, &self.triangles);
        }
        self.positions.extend(other.positions);
        self.normals.extend(normals);
        self.triangles.extend(other.triangles.iter().map(|t| t.map(|i| i + base)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_cottage_has_every_part_and_lit_windows() {
        let built = Cottage::default().build();
        for part in Part::ALL {
            assert!(built.parts.meshes.contains_key(&part), "no {:?}", part);
        }
        // Two front windows, one at the back and one in each gable.
        assert_eq!(built.windows.len(), 5);
        for m in built.parts.meshes.values().flatten() {
            assert_eq!(m.positions.len(), m.normals.len());
            assert!(m.positions.iter().all(|p| p.iter().all(|x| x.is_finite())));
            assert!(m.triangles.iter().all(|t| t.iter().all(|&i| i < m.positions.len())));
        }
    }

    /// The windows' lights face out of the walls they are in.
    #[test]
    fn test_windows_face_outwards() {
        for w in Cottage::default().build().windows {
            let patch = crate::shapes::bilinear::BilinearPatch::new(w[0], w[1], w[2], w[3]);
            let centre = (w[0] + w[3]) / 2.;
            let out = V::new(centre.x, 0., centre.z);
            assert!(patch.normal(0.5, 0.5).dot(&out) > 0., "{:?}", centre);
        }
    }

    /// No log reaches through the roof: every vertex of the walls is under its underside.
    #[test]
    fn test_logs_stay_under_the_roof() {
        let c = Cottage::default();
        let built = c.build();
        for part in [Part::LogsAlongX, Part::LogsAlongZ] {
            for m in &built.parts.meshes[&part] {
                for p in &m.positions {
                    assert!(p.y <= c.under_roof(p.z) + 0.02, "{:?} above the roof's {}", p, c.under_roof(p.z));
                }
            }
        }
    }

    /// The snow on the roof is as deep as asked, give or take its unevenness, and lies on the
    /// roof's top.
    #[test]
    fn test_roof_snow_lies_on_the_roof() {
        let c = Cottage { chimney: false, ..Cottage::default() };
        let built = c.build();
        let snow = &built.parts.meshes[&Part::Snow];
        let top = snow.iter().flat_map(|m| m.positions.iter()).map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        let crest = c.ridge() + 0.12 / c.tan().atan().cos();
        assert!(top > crest + 0.6 * c.snow && top < crest + 1.5 * c.snow / c.tan().atan().cos(), "{} over {}", top, crest);
    }

    #[test]
    fn test_snow_pad_is_watertight_at_its_edges() {
        let frame = Frame { origin: V::zeros(), u: V::x(), v: V::z(), n: V::y() };
        let pad = snow_pad(&frame, (-1., 1.), (-0.5, 0.5), 0.1, 0., None, &PerlinNoise::shuffled(1), 0.05);
        // Top and bottom grids meet along the boundary: the first row of each is the same.
        let row = pad.positions.len() / 2;
        for i in 0 .. 10 {
            assert!((pad.positions[i] - pad.positions[row + i]).norm() < 1e-12);
        }
        let deepest = pad.positions.iter().map(|p| p.y).fold(0., f64::max);
        assert!(deepest > 0.08 && deepest < 0.13, "{}", deepest);
    }
}

