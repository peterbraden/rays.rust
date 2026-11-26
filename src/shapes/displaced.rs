//! # Displaced terrain
//!
//! A terrain whose surface is moved by a function graph (`graph.rs`) at render time, finely
//! enough near the camera that rock, scree and ledges have real silhouettes and cast real
//! shadows, below the heightmap's resolution. As in Terragen and in Reyes renderers, the surface
//! is diced into micropolygons about a pixel across where the camera sees it (Robert Cook, Loren
//! Carpenter and Edwin Catmull, "The Reyes image rendering architecture", SIGGRAPH 1987,
//! doi:10.1145/37402.37414); here, for a ray tracer, lazily and per cell of the heightmap, as
//! in Pharr and Hanrahan's geometry caching ("Geometry caching for ray-tracing displacement
//! maps", Eurographics Workshop on Rendering 1996).
//!
//! In a scene file, a `terrain` with a `displacement` (a graph) is displaced:
//!
//! ```json
//! { "type": "terrain", "heightmap": {...},
//!   "displacement": {"node": "fractal", "style": "ridged", "scale": 4, "smallest": 0.02, "amplitude": 0.5},
//!   "displacement_detail": {"pixels": 1, "max_level": 8, "offscreen": 4} }
//! ```
//!
//! - The base surface is the heightmap made smooth (`HeightGrid::smooth`: Catmull-Rom splines
//!   through its vertices), so it shows no facets of the grid however close the camera is.
//! - The graph is evaluated at points of the base surface, knowing its normal (for `slope`
//!   masks) and the spacing of the vertices there (to filter out finer detail), and moves them
//!   along the base's normal, so cliffs gain relief as well as level ground; or straight up,
//!   with `"displacement_direction": "vertical"`.
//! - `"vertical_displacement"`, a second graph, moves them straight up as well: for things
//!   that stand up from the ground whatever its slope, like the fallen blocks of a `talus`
//!   (`talus.rs`), whose tops then lie above where they stand, so a material's mask finds them.
//! - Each cell is diced into 2^k x 2^k quads, `k` its level, chosen once from the camera so
//!   that a quad is about `pixels` pixels across where the cell is nearest it, up to
//!   `max_level` (default 8), and no finer than half the graph's finest detail (or
//!   `min_spacing`, metres). Without a camera, every cell is diced at `level` (default 4).
//!   Since the level depends on the cell alone, every ray (from the camera, a light or a bounce)
//!   sees the same surface, and shadows match what is seen.
//! - Cells are diced when a ray first comes near them, and kept. Each keeps a quadtree of
//!   bounding boxes over its quads, which rays descend nearest first. Before dicing, a cell's
//!   bounds allow sideways displacement only as far as its base's slopes can tilt the normal,
//!   so rays grazing gentle ground dice few cells besides those they hit.
//! - Cells wholly outside a still perspective camera's view are diced `offscreen` times
//!   coarser (default 4): they're seen only through their shadows and the light they reflect.
//! - Neighbouring cells at different levels meet without cracks: a cell's edge shared with a
//!   coarser one follows the coarser cell's edge, and the points on edges and corners are
//!   evaluated as filtered for the coarsest cell they belong to, so both sides compute exactly
//!   the same points.
//!
//! The grass, rocks or trees placed on a heightmap stand on its grid, not on the displaced
//! surface: keep displacement small where they are, or mask it off.
use crate::bvh::Bvh;
use crate::graph::{Context, Graph};
use crate::intersection::RawIntersection;
use crate::na::{Affine3, Matrix3, Vector3};
use crate::procedural::heightmap::{intersect_triangle, HeightGrid};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, OnceLock};

/// Threads for dicing large cells asked for from outside a thread pool, shared by every
/// displaced terrain (one pool each would start a thread per core for each).
static DICING: OnceLock<Option<rayon::ThreadPool>> = OnceLock::new();

thread_local! {
    static ROUGHLY: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with displaced terrains seen roughly, as their heightmaps' flat cells, without
/// dicing any: for guesses that needn't be exact, such as which of the sky the camera sees
/// (`environment`). A ray grazing the ground from the camera passes close to many cells, and
/// dicing them all for that guess cost more than the render.
pub fn roughly<T>(f: impl FnOnce() -> T) -> T {
    // Restored when dropped, so a panic in `f` (caught further up) can't leave this thread
    // seeing terrain roughly for the rest of the render.
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) { ROUGHLY.with(|r| r.set(self.0)); }
    }
    let _restore = Restore(ROUGHLY.with(|r| r.replace(true)));
    f()
}

/// While nonzero, displaced terrains are diced no finer than this level less one, on the fly,
/// keeping no tiles (`coarsely`).
static COARSELY: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The tiles diced coarsely while `coarsely` runs, by terrain and cell; dropped after.
static COARSE_TILES: LazyLock<std::sync::Mutex<HashMap<(usize, usize), Arc<Tile>>>> = LazyLock::new(Default::default);

/// Run `f` with displaced terrains diced no finer than `level` (cells split 2^level ways),
/// each ray's cells diced afresh and not kept: for placing things on the ground while a scene
/// is built (a landscape's own boulders), where the camera's detail isn't needed, and keeping
/// the tile under every one of thousands of candidates, diced as finely as the camera wants
/// the cells near it, took gigabytes. For every thread, so only while nothing renders.
pub fn coarsely<T>(level: u32, f: impl FnOnce() -> T) -> T {
    use std::sync::atomic::Ordering;
    struct Restore(u32);
    impl Drop for Restore {
        fn drop(&mut self) {
            COARSELY.store(self.0, Ordering::Relaxed);
            if self.0 == 0 { COARSE_TILES.lock().unwrap().clear(); }
        }
    }
    let _restore = Restore(COARSELY.swap(level + 1, Ordering::Relaxed));
    f()
}

/// How finely to dice: from the camera, or everywhere alike.
#[derive(Clone, Debug)]
pub struct Detail {
    /// The camera's position, and the angle between neighbouring pixels (radians).
    pub eye: Option<(Vector3<f64>, f64)>,
    /// Quads about this many pixels across.
    pub pixels: f64,
    pub max_level: u32,
    /// The smallest spacing of vertices, metres (by default half the graph's finest detail).
    pub min_spacing: Option<f64>,
    /// The level of every cell, without a camera.
    pub level: u32,
    /// What the camera sees, if known: cells wholly outside it are diced `offscreen` times
    /// coarser.
    pub view: Option<View>,
    pub offscreen: f64,
    /// Where copies of the terrain are put in the scene (transforms around it, or instances of
    /// a named geometry), when the camera above is the scene's: each cell is diced as finely as
    /// the nearest copy of it needs, and is out of view only if every copy is. Empty for a
    /// terrain where it stands.
    pub instances: Vec<Affine3<f64>>,
}

impl Default for Detail {
    fn default() -> Detail {
        Detail { eye: None, pixels: 1., max_level: 8, min_spacing: None, level: 4, view: None, offscreen: 1., instances: vec![] }
    }
}

/// A perspective camera's field of view: its axes (unit vectors), the tangents of half its
/// angles across and up, and how far rays may start from its position (the lens's radius).
#[derive(Clone, Debug)]
pub struct View {
    pub eye: Vector3<f64>,
    pub right: Vector3<f64>,
    pub up: Vector3<f64>,
    pub forward: Vector3<f64>,
    pub tan_x: f64,
    pub tan_y: f64,
    pub margin: f64,
}

impl View {
    /// This view seen from a frame `to_frame` maps the scene into: None unless that is a
    /// rotation, translation and uniform scale, which keep the view a frustum.
    pub fn through(&self, to_frame: &Affine3<f64>) -> Option<View> {
        let linear: Matrix3<f64> = to_frame.matrix().fixed_view::<3, 3>(0, 0).into();
        let gram = linear.transpose() * linear;
        let s2 = gram[(0, 0)];
        if !s2.is_finite() || s2 <= 0. || (gram - Matrix3::identity() * s2).abs().max() > 1e-9 * s2 {
            return None;
        }
        let s = s2.sqrt();
        Some(View {
            eye: to_frame.transform_point(&crate::na::Point3::from(self.eye)).coords,
            right: linear * self.right / s, up: linear * self.up / s, forward: linear * self.forward / s,
            tan_x: self.tan_x, tan_y: self.tan_y, margin: self.margin * s,
        })
    }

    /// Whether the box lies wholly outside the view: all its corners beyond one of the planes
    /// bounding it (the four sides, and the plane of the lens behind).
    pub fn excludes(&self, b: &BBox) -> bool {
        let planes = [
            (self.right - self.forward * self.tan_x).normalize(),
            (-self.right - self.forward * self.tan_x).normalize(),
            (self.up - self.forward * self.tan_y).normalize(),
            (-self.up - self.forward * self.tan_y).normalize(),
            -self.forward,
        ];
        planes.iter().any(|n| (0 .. 8).all(|c| {
            let p = Vector3::new(
                if c & 1 == 0 { b.min.x } else { b.max.x },
                if c & 2 == 0 { b.min.y } else { b.max.y },
                if c & 4 == 0 { b.min.z } else { b.max.z });
            (p - self.eye).dot(n) > self.margin
        }))
    }
}

/// A diced cell: its (2^k + 1)^2 vertices and their normals, row by row along x, and the
/// bounds of each block of 2^j x 2^j quads for j from 1 to k.
struct Tile {
    n: usize,
    pos: Vec<Vector3<f64>>,
    nrm: Vec<[f32; 3]>,
    boxes: Vec<Vec<BBox>>,
}

/// A cell's edge: its first vertex (i, j), the step along it, and the offset to the cell across it.
type Edge = ((usize, usize), (usize, usize), (isize, isize));

pub struct DisplacedTerrain {
    grid: Arc<HeightGrid>,
    graph: Graph,
    vertical: bool,
    /// A second displacement, straight up, added to the first: things that stand up from the
    /// ground, like fallen blocks (`talus`), on slopes as on level ground.
    lift: Option<Graph>,
    /// Displacement in directions of its own, added last.
    sideways: Sideways,
    /// Metres over which the displacement fades to nothing at the grid's edges, so it meets
    /// an undisplaced grid around it (a coarser terrain with a `hole`) without a seam.
    falloff: f64,
    /// The displacement's range.
    range: (f64, f64),
    /// Each cell's level, and conservative bounds.
    levels: Vec<u8>,
    cell_bounds: Vec<BBox>,
    tiles: Vec<OnceLock<Tile>>,
    /// Over the cells, by index into `cells`.
    bvh: Bvh,
    cells: Vec<u32>,
    bounds: BBox,
    /// Where the terrain is put in the scene, when it is placed by a transform around it: its
    /// origin and axes there, so its graphs are evaluated at the scene's points, as they are
    /// when shading (`graph::Context::at_hit`), and nearby placements displace alike.
    place: Option<(Vector3<f64>, Matrix3<f64>)>,
    /// What its graphs' `field` nodes read (a landscape's simulation).
    fields: Option<Arc<dyn crate::graph::Fields>>,
    /// Cells left out for a finer terrain nested inside (`Nest`): [x0, x1) x [z0, z1).
    hole: Option<(usize, usize, usize, usize)>,
    /// The coarser terrain this one is nested in, whose edge it meets.
    outer: Option<Nest>,
}

/// A displacement as a scene asks for it (`SceneFile::parse_displacement`).
pub struct Displacement {
    pub graph: Graph,
    pub vertical: bool,
    pub lift: Option<Graph>,
    pub sideways: Sideways,
    pub falloff: f64,
    pub detail: Detail,
}

/// Displacement in directions of its own (`"vector_displacement"`), added after the rest:
/// along the terrain's own x, y and z, and `outward`, horizontally away from the slope (along
/// the base's normal, flattened), which on a cliff cuts beds back under the ones above or
/// pushes them out over those below: overhangs.
#[derive(Default)]
pub struct Sideways {
    pub x: Option<Graph>,
    pub y: Option<Graph>,
    pub z: Option<Graph>,
    pub outward: Option<Graph>,
}

impl Sideways {
    fn graphs(&self) -> impl Iterator<Item = &Graph> {
        [&self.x, &self.y, &self.z, &self.outward].into_iter().flatten()
    }

    /// How far it moves points along x, y and z (each range toward 0 if `toward_zero`).
    fn ranges(&self, toward_zero: &dyn Fn((f64, f64)) -> (f64, f64)) -> [(f64, f64); 3] {
        let r = |g: &Option<Graph>| toward_zero(g.as_ref().map_or((0., 0.), |g| g.range()));
        let o = r(&self.outward);
        let out = o.0.abs().max(o.1.abs());
        let (x, y, z) = (r(&self.x), r(&self.y), r(&self.z));
        [(x.0 - out, x.1 + out), y, (z.0 - out, z.1 + out)]
    }

    /// The move at a point whose base has `normal`, seen as `c`, kept by `fade`.
    fn at(&self, c: &Context, normal: &Vector3<f64>, fade: f64) -> Vector3<f64> {
        let e = |g: &Option<Graph>| g.as_ref().map_or(0., |g| g.eval(c));
        let mut v = Vector3::new(e(&self.x), e(&self.y), e(&self.z));
        if let Some(g) = &self.outward {
            let flat = Vector3::new(normal.x, 0., normal.z);
            let n = flat.norm();
            if n > 1e-9 {
                v += flat / n * g.eval(c);
            }
        }
        v * fade
    }
}

/// Where a terrain sits in the coarser one around it (`DisplacedTerrain::nested`): the outer
/// terrain, of exactly twice its cell, and its own first cell's position in the outer's grid in
/// its own cells (even, so its edges lie on the outer's grid lines). Along its edges it takes
/// its points from the outer terrain, at the level the outer's cells there are diced to, so the
/// two meet without a crack.
pub struct Nest {
    pub outer: Arc<DisplacedTerrain>,
    pub offset: (i64, i64),
}

/// Where the ray enters and leaves the box, in multiples of its direction, within [t0, t1].
#[inline]
fn slab(b: &BBox, ro: &Vector3<f64>, inv: &Vector3<f64>, t0: f64, t1: f64) -> Option<f64> {
    let (mut near, mut far) = (t0, t1);
    for a in 0 .. 3 {
        let u = (b.min[a] - ro[a]) * inv[a];
        let v = (b.max[a] - ro[a]) * inv[a];
        if u.is_nan() || v.is_nan() { continue }
        let (lo, hi) = if u < v { (u, v) } else { (v, u) };
        if lo > near { near = lo }
        if hi < far { far = hi }
        if near > far { return None }
    }
    Some(near)
}

/// Conservative bounds of cell (`ix`, `iz`) of `grid` displaced by `range` (along the base's
/// normal, or straight up if `vertical`), lifted by `up` and moved by `aside` along x, y, z.
fn cell_bound(grid: &HeightGrid, ix: usize, iz: usize, range: (f64, f64), up: (f64, f64), aside: &[(f64, f64); 3], vertical: bool) -> BBox {
    let cell = grid.cell;
    let reach = range.0.abs().max(range.1.abs());
    let (lo, hi) = grid.smooth_range(ix, iz);
    let (x0, z0) = (grid.min.0 + ix as f64 * cell, grid.min.1 + iz as f64 * cell);
    let (min, max) = if vertical {
        (Vector3::new(x0, lo + range.0, z0), Vector3::new(x0 + cell, hi + range.1, z0 + cell))
    } else {
        // Moved along the base's normal, (-hx, 1, -hz) normalised: sideways by at most reach
        // times |hx| / sqrt(1 + hx^2) (and so in z), and up or down by the displacement times
        // a y component between 1 and 1 / sqrt(1 + hx^2 + hz^2). On gentle ground that keeps
        // the bounds to a little more than the cell, so rays dice only cells they come near.
        let (sx, sz) = grid.smooth_slope_bounds(ix, iz);
        let side = |s: f64| reach * (s / (1. + s * s).sqrt()).min(1.);
        let (rx, rz) = (side(sx), side(sz));
        let ny = 1. / (1. + sx * sx + sz * sz).sqrt();
        let down = if range.0 < 0. { range.0 } else { range.0 * ny };
        let rise = if range.1 > 0. { range.1 } else { range.1 * ny };
        (Vector3::new(x0 - rx, lo + down, z0 - rz), Vector3::new(x0 + cell + rx, hi + rise, z0 + cell + rz))
    };
    let (min, max) = (min + Vector3::y() * up.0.min(0.), max + Vector3::y() * up.1.max(0.));
    let (min, max) = (min + Vector3::new(aside[0].0.min(0.), aside[1].0.min(0.), aside[2].0.min(0.)),
        max + Vector3::new(aside[0].1.max(0.), aside[1].1.max(0.), aside[2].1.max(0.)));
    // Room for rounding.
    let pad = Vector3::repeat(1e-9 * (1. + min.abs().max().max(max.abs().max())));
    BBox::new(min - pad, max + pad)
}

impl DisplacedTerrain {
    pub fn new(grid: Arc<HeightGrid>, graph: Graph, vertical: bool, detail: &Detail) -> Result<DisplacedTerrain, String> {
        DisplacedTerrain::with_lift(grid, graph, vertical, None, 0., detail)
    }

    /// With `lift`, a displacement straight up added to the first; with `falloff` (metres),
    /// both fade to nothing at the grid's edges.
    pub fn with_lift(grid: Arc<HeightGrid>, graph: Graph, vertical: bool, lift: Option<Graph>, falloff: f64, detail: &Detail) -> Result<DisplacedTerrain, String> {
        DisplacedTerrain::nested(grid, graph, vertical, lift, falloff, detail, None, None)
    }

    /// The terrain a scene's displacement `d` asks for, leaving out `hole`, nested in `outer`.
    pub fn of(grid: Arc<HeightGrid>, d: Displacement, hole: Option<(usize, usize, usize, usize)>, outer: Option<Nest>) -> Result<DisplacedTerrain, String> {
        DisplacedTerrain::build(grid, d.graph, d.vertical, d.lift, d.sideways, d.falloff, &d.detail, hole, outer)
    }

    /// As `with_lift`, leaving out the cells in `hole` ([x0, x1) x [z0, z1)) for a finer
    /// terrain to fill, and nested in `outer`.
    #[allow(clippy::too_many_arguments)]
    pub fn nested(grid: Arc<HeightGrid>, graph: Graph, vertical: bool, lift: Option<Graph>, falloff: f64, detail: &Detail,
        hole: Option<(usize, usize, usize, usize)>, outer: Option<Nest>) -> Result<DisplacedTerrain, String>
    {
        DisplacedTerrain::build(grid, graph, vertical, lift, Sideways::default(), falloff, detail, hole, outer)
    }

    #[allow(clippy::too_many_arguments)]
    fn build(grid: Arc<HeightGrid>, graph: Graph, vertical: bool, lift: Option<Graph>, sideways: Sideways, falloff: f64, detail: &Detail,
        hole: Option<(usize, usize, usize, usize)>, outer: Option<Nest>) -> Result<DisplacedTerrain, String>
    {
        use rayon::prelude::*;
        if let Some(n) = &outer {
            let o = &n.outer.grid;
            let aligned = (o.cell - 2. * grid.cell).abs() <= 1e-9 * o.cell && n.offset.0 % 2 == 0 && n.offset.1 % 2 == 0
                && grid.nx.is_multiple_of(2) && grid.nz.is_multiple_of(2)
                && (o.min.0 + n.offset.0 as f64 * grid.cell - grid.min.0).abs() <= 1e-6 * grid.cell
                && (o.min.1 + n.offset.1 as f64 * grid.cell - grid.min.1).abs() <= 1e-6 * grid.cell;
            if !aligned {
                return Err("a nested terrain must have half the outer's cell, an even number of them, and lie on its grid".into());
            }
        }
        let in_hole = |ix: usize, iz: usize| hole.is_some_and(|(x0, x1, z0, z1)| ix >= x0 && ix < x1 && iz >= z0 && iz < z1);
        // Faded towards the edges, the displacement takes values between its range and 0.
        let toward_zero = |r: (f64, f64)| if falloff > 0. { (r.0.min(0.), r.1.max(0.)) } else { r };
        let range = toward_zero(graph.range());
        let up = toward_zero(lift.as_ref().map_or((0., 0.), |g| g.range()));
        let aside = sideways.ranges(&toward_zero);
        if !(range.0.is_finite() && range.1.is_finite() && up.0.is_finite() && up.1.is_finite()
            && aside.iter().all(|r| r.0.is_finite() && r.1.is_finite())) {
            return Err("a terrain's displacement must be bounded: give an expression a \"range\"".into());
        }
        let reach = range.0.abs().max(range.1.abs());
        let (nx, nz, cell) = (grid.nx, grid.nz, grid.cell);
        let cell_bounds: Vec<BBox> = (0 .. nx * nz).into_par_iter()
            .map(|k| cell_bound(&grid, k % nx, k / nx, range, up, &aside, vertical)).collect();
        // Dice no finer than half the finest detail, or an eighth of a cell, so the smooth base
        // itself looks smooth.
        // A steep cell's surface is longer than the cell is wide: on a cliff, a 1.5 m cell may
        // span 20 m of rock. Its level is chosen for the longer of the two, so quads up a cliff
        // are no taller than a pixel rather than smearing the relief into vertical streaks.
        let finest = graph.finest().min(lift.as_ref().map_or(f64::INFINITY, |g| g.finest()))
            .min(sideways.graphs().map(|g| g.finest()).fold(f64::INFINITY, f64::min));
        let min_spacing = detail.min_spacing.unwrap_or_else(|| (0.5 * finest).min(cell / 8.)).max(1e-6);
        // The camera as each copy of the terrain sees it, in the terrain's own frame (the
        // angle between pixels is the same there, under a uniform scale).
        let pixel_angle = detail.eye.map_or(0., |(_, a)| a);
        let eyes: Vec<(Vector3<f64>, Option<View>)> = match detail.eye {
            None => vec![],
            Some((eye, _)) if detail.instances.is_empty() => vec![(eye, detail.view.clone())],
            Some((eye, _)) => detail.instances.iter().filter_map(|t| {
                let to_frame = t.try_inverse()?;
                Some((to_frame.transform_point(&crate::na::Point3::from(eye)).coords,
                    detail.view.as_ref().and_then(|v| v.through(&to_frame))))
            }).collect(),
        };
        let levels: Vec<u8> = cell_bounds.par_iter().enumerate().map(|(k, b)| {
            let (lo, hi) = grid.smooth_range(k % nx, k / nx);
            let extent = cell.max(hi - lo);
            let finest = ((extent / min_spacing).log2().floor().max(0.) as u32).min(detail.max_level);
            let level = if eyes.is_empty() { detail.level } else {
                // As finely as the copy that needs it most.
                eyes.iter().map(|(eye, view)| {
                    let nearest = Vector3::new(eye.x.clamp(b.min.x, b.max.x), eye.y.clamp(b.min.y, b.max.y), eye.z.clamp(b.min.z, b.max.z));
                    // Cells out of view are seen only by the light they shade or reflect.
                    let unseen = view.as_ref().is_some_and(|v| v.excludes(b));
                    let footprint = (nearest - eye).norm().max(1e-6) * pixel_angle * detail.pixels
                        * if unseen { detail.offscreen.max(1.) } else { 1. };
                    (extent / footprint).log2().ceil().max(0.) as u32
                }).max().unwrap_or(detail.level)
            };
            level.min(finest).min(15) as u8
        }).collect();
        let mut levels = levels;
        let mut cell_bounds = cell_bounds;
        // Beside a hole, at least level 1: a nested terrain's cells are half as wide, and their
        // corners halfway along this cell's edge must be vertices of it.
        if let Some((x0, x1, z0, z1)) = hole {
            for iz in z0.saturating_sub(1) .. (z1 + 1).min(nz) {
                for ix in x0.saturating_sub(1) .. (x1 + 1).min(nx) {
                    if !in_hole(ix, iz) { levels[iz * nx + ix] = levels[iz * nx + ix].max(1); }
                }
            }
        }
        // Along a nested terrain's edge, at least as fine as the outer's cells across it (so
        // its edges follow theirs), and bounds allowing for the outer's points there.
        if let Some(n) = &outer {
            let o = &n.outer;
            let across = |ix: i64, iz: i64| o.level((ix + n.offset.0).div_euclid(2) as isize, (iz + n.offset.1).div_euclid(2) as isize)
                .map_or(0, |l| l.saturating_sub(1)) as u8;
            // How far the two bases part along the edge, sampled at quarter cells.
            let mut parted: f64 = 0.;
            let (x1, z1) = (grid.min.0 + nx as f64 * cell, grid.min.1 + nz as f64 * cell);
            for k in 0 ..= 4 * nx.max(nz) {
                let t = k as f64 * cell / 4.;
                for (x, z) in [(grid.min.0 + t, grid.min.1), (grid.min.0 + t, z1), (grid.min.0, grid.min.1 + t), (x1, grid.min.1 + t)] {
                    if x <= x1 + 1e-9 && z <= z1 + 1e-9 {
                        parted = parted.max((o.grid.smooth(x, z).0 - grid.smooth(x, z).0).abs());
                    }
                }
            }
            let pad = Vector3::repeat(parted + 0.1 * reach + 1e-3 * cell);
            for iz in 0 .. nz as i64 {
                for ix in 0 .. nx as i64 {
                    if ix > 0 && iz > 0 && ix < nx as i64 - 1 && iz < nz as i64 - 1 { continue; }
                    let k = (iz as usize) * nx + ix as usize;
                    let outside = [(ix - 1, iz), (ix + 1, iz), (ix, iz - 1), (ix, iz + 1), (ix - 1, iz - 1), (ix + 1, iz - 1), (ix - 1, iz + 1), (ix + 1, iz + 1)];
                    for (i, j) in outside {
                        if i < 0 || j < 0 || i >= nx as i64 || j >= nz as i64 {
                            levels[k] = levels[k].max(across(i, j));
                        }
                    }
                    cell_bounds[k] = BBox::new(cell_bounds[k].min.coords - pad, cell_bounds[k].max.coords + pad);
                }
            }
        }
        let (bvh, cells) = Bvh::build((0 .. (nx * nz) as u32).filter(|&k| !in_hole(k as usize % nx, k as usize / nx)).collect(),
            |&k| cell_bounds[k as usize]);
        let bounds = bvh.bounds();
        let vertices: f64 = levels.iter().map(|&l| ((1u64 << l) as f64 + 1.).powi(2)).sum();
        println!("Displaced terrain: {} x {} cells diced to levels {} to {}, up to {:.1} million vertices if all are seen",
            nx, nz, levels.iter().min().unwrap_or(&0), levels.iter().max().unwrap_or(&0), vertices / 1e6);
        let tiles = (0 .. nx * nz).map(|_| OnceLock::new()).collect();
        Ok(DisplacedTerrain { grid, graph, vertical, lift, sideways, falloff: falloff.max(0.), range, levels, cell_bounds, tiles,
            bvh, cells, bounds, place: None, fields: None, hole, outer })
    }

    /// Evaluate the graphs where the terrain will be in the scene: at `origin + axes * p` for
    /// its own point p (the transform it will be put through, which must be a rotation and a
    /// translation).
    pub fn placed(mut self, origin: Vector3<f64>, axes: Matrix3<f64>) -> DisplacedTerrain {
        self.place = Some((origin, axes));
        self
    }

    /// Let its graphs read `fields` (as `field` nodes, and the planet's up for `slope` and
    /// `altitude`).
    pub fn with_fields(mut self, fields: Arc<dyn crate::graph::Fields>) -> DisplacedTerrain {
        self.fields = Some(fields);
        self
    }

    /// Bound each cell by what its displacement can be there (`Graph::range_near`, at the
    /// cell's centre with the fields and placement it has now, for its steepest base), rather
    /// than by the displacement's whole range, and rebuild the hierarchy over them. Only where
    /// the displacement is a graph that knows its local bounds (the landscape's own detail):
    /// sand varies by centimetres where granite does by metres, and a ray grazing the ground
    /// dices only the cells it comes near. Cells along a nested terrain's edge keep their
    /// bounds, which allow for the outer's points.
    pub fn tighten(mut self) -> DisplacedTerrain {
        use rayon::prelude::*;
        if self.lift.is_some() || self.sideways.graphs().next().is_some() { return self; }
        let g = self.grid.clone();
        let (nx, nz) = (g.nx, g.nz);
        let toward_zero = |r: (f64, f64)| if self.falloff > 0. { (r.0.min(0.), r.1.max(0.)) } else { r };
        let aside = [(0., 0.); 3];
        let tightened: Vec<BBox> = (0 .. nx * nz).into_par_iter().map(|k| {
            let (ix, iz) = (k % nx, k / nx);
            if self.outer.is_some() && (ix == 0 || iz == 0 || ix == nx - 1 || iz == nz - 1) { return self.cell_bounds[k]; }
            let (x, z) = (g.min.0 + (ix as f64 + 0.5) * g.cell, g.min.1 + (iz as f64 + 0.5) * g.cell);
            let (y, hx, hz) = g.smooth(x, z);
            let base = Vector3::new(x, y, z);
            let normal = Vector3::new(-hx, 1., -hz).normalize();
            let c = match &self.place {
                Some((origin, axes)) => Context::new(origin + axes * base, axes * normal, g.cell),
                None => Context::new(base, normal, g.cell),
            }.with_fields(self.fields.as_deref());
            let (sx, sz) = g.smooth_slope_bounds(ix, iz);
            let steepest = sx.hypot(sz).atan().to_degrees() + 5.;
            let (lo, hi) = g.smooth_range(ix, iz);
            let r = self.graph.range_near(&c, steepest, 0.5 * g.cell, (lo - y, hi - y));
            let r = (r.0.max(self.range.0), r.1.min(self.range.1));
            cell_bound(&g, ix, iz, toward_zero(r), (0., 0.), &aside, self.vertical)
        }).collect();
        let hole = self.hole;
        let in_hole = |ix: usize, iz: usize| hole.is_some_and(|(x0, x1, z0, z1)| ix >= x0 && ix < x1 && iz >= z0 && iz < z1);
        let (bvh, cells) = Bvh::build((0 .. (nx * nz) as u32).filter(|&k| !in_hole(k as usize % nx, k as usize / nx)).collect(),
            |&k| tightened[k as usize]);
        self.bounds = bvh.bounds();
        self.bvh = bvh;
        self.cells = cells;
        self.cell_bounds = tightened;
        self
    }

    /// The displacement's range.
    pub fn displacement_range(&self) -> (f64, f64) {
        self.range
    }

    /// Cell (ix, iz)'s level; for one across a nested terrain's edge, the level in its own cells
    /// that the outer's cell there is diced to; None for none (off the grid, or in its hole).
    fn level(&self, ix: isize, iz: isize) -> Option<u32> {
        let (nx, nz) = (self.grid.nx as isize, self.grid.nz as isize);
        if ix >= 0 && iz >= 0 && ix < nx && iz < nz {
            let in_hole = self.hole.is_some_and(|(x0, x1, z0, z1)|
                ix as usize >= x0 && (ix as usize) < x1 && iz as usize >= z0 && (iz as usize) < z1);
            return (!in_hole).then(|| self.levels[(iz * nx + ix) as usize] as u32);
        }
        let n = self.outer.as_ref()?;
        let l = n.outer.level((ix as i64 + n.offset.0).div_euclid(2) as isize, (iz as i64 + n.offset.1).div_euclid(2) as isize)?;
        Some(l.saturating_sub(1))
    }

    /// The displaced surface at grid vertex (gx, gz) of level `level` (2^level vertices to a
    /// cell), filtered for that level: computed from the integers alone, so every cell that
    /// shares the point computes it identically.
    fn point(&self, gx: i64, gz: i64, level: u32) -> Vector3<f64> {
        let g = &self.grid;
        // On a nested terrain's edge, the outer's point, as the outer computes it: half as many
        // of its cells, so the same spacing at one level finer.
        if let Some(n) = &self.outer
            && (gx == 0 || gz == 0 || gx == (g.nx as i64) << level || gz == (g.nz as i64) << level) {
            return n.outer.point(gx + (n.offset.0 << level), gz + (n.offset.1 << level), level + 1);
        }
        let scale = 0.5f64.powi(level as i32);
        let (x, z) = (g.min.0 + (gx as f64 * g.cell) * scale, g.min.1 + (gz as f64 * g.cell) * scale);
        let (y, hx, hz) = g.smooth(x, z);
        let base = Vector3::new(x, y, z);
        let normal = Vector3::new(-hx, 1., -hz).normalize();
        let c = match &self.place {
            Some((origin, axes)) => Context::new(origin + axes * base, axes * normal, g.cell * scale),
            None => Context::new(base, normal, g.cell * scale),
        }.with_fields(self.fields.as_deref());
        let fade = self.fade(x, z);
        if fade <= 0. {
            return base;
        }
        let d = fade * self.graph.eval(&c);
        let up = fade * self.lift.as_ref().map_or(0., |l| l.eval(&c));
        let aside = self.sideways.at(&c, &normal, fade);
        base + if self.vertical { Vector3::y() } else { normal } * d + Vector3::y() * up + aside
    }

    /// How much of the displacement is kept at (x, z): 1, falling smoothly to 0 within
    /// `falloff` of the grid's edges.
    fn fade(&self, x: f64, z: f64) -> f64 {
        if self.falloff <= 0. {
            return 1.;
        }
        let g = &self.grid;
        let (x1, z1) = (g.min.0 + g.nx as f64 * g.cell, g.min.1 + g.nz as f64 * g.cell);
        let inside = (x - g.min.0).min(x1 - x).min(z - g.min.1).min(z1 - z);
        let t = (inside / self.falloff).clamp(0., 1.);
        t * t * (3. - 2. * t)
    }

    /// The level whose filter the grid corner (cx, cz) is evaluated at: the coarsest of the
    /// cells that share it.
    fn corner_level(&self, cx: i64, cz: i64) -> u32 {
        [(cx - 1, cz - 1), (cx, cz - 1), (cx - 1, cz), (cx, cz)].iter()
            .filter_map(|&(i, j)| self.level(i as isize, j as isize)).min().unwrap_or(0)
    }

    fn dice(&self, k: usize, parallel: bool) -> Tile {
        self.dice_at(k, self.levels[k] as u32, parallel)
    }

    /// Cell k diced to `level`.
    fn dice_at(&self, k: usize, level: u32, parallel: bool) -> Tile {
        let (ix, iz) = ((k % self.grid.nx) as i64, (k / self.grid.nx) as i64);
        let n = 1usize << level;
        let ni = n as i64;
        // With a ring of vertices outside the cell, for normals.
        let w = n + 3;
        let ring_point = |q: usize| {
            let (i, j) = ((q % w) as i64 - 1, (q / w) as i64 - 1);
            self.point(ix * ni + i, iz * ni + j, level)
        };
        let ring: Vec<Vector3<f64>> = if parallel {
            use rayon::prelude::*;
            (0 .. w * w).into_par_iter().with_min_len(256).map(ring_point).collect()
        } else {
            (0 .. w * w).map(ring_point).collect()
        };
        let at = |i: usize, j: usize| ring[(j + 1) * w + i + 1];
        let mut pos = Vec::with_capacity((n + 1) * (n + 1));
        let mut nrm = Vec::with_capacity((n + 1) * (n + 1));
        for j in 0 ..= n {
            for i in 0 ..= n {
                pos.push(at(i, j));
                let tu = ring[(j + 1) * w + i + 2] - ring[(j + 1) * w + i];
                let tv = ring[(j + 2) * w + i + 1] - ring[j * w + i + 1];
                let m = tv.cross(&tu);
                let m = if m.norm() > 0. { m.normalize() } else { Vector3::y() };
                nrm.push([m.x as f32, m.y as f32, m.z as f32]);
            }
        }
        // Edges: where the neighbour across is coarser, follow its edge; and evaluate the
        // points on the edge filtered for the coarser of the two.
        // (edge's first vertex as (i, j), step along it, the neighbour's offset)
        let edges: [Edge; 4] =
            [((0, 0), (1, 0), (0, -1)), ((0, n), (1, 0), (0, 1)), ((0, 0), (0, 1), (-1, 0)), ((n, 0), (0, 1), (1, 0))];
        for &((i0, j0), (di, dj), (ox, oz)) in &edges {
            let theirs = self.level(ix as isize + ox, iz as isize + oz).unwrap_or(level);
            let shared = theirs.min(level);
            let m = 1usize << (level - shared);
            let coarse = |c: usize| -> Vector3<f64> {
                // The c-th vertex along this edge at the shared level; at its ends, the corner.
                let (vi, vj) = (i0 + di * c * m, j0 + dj * c * m);
                let (gx, gz) = (ix * ni + vi as i64, iz * ni + vj as i64);
                if (vi == 0 || vi == n) && (vj == 0 || vj == n) {
                    let (cx, cz) = (gx / ni, gz / ni);
                    let lc = self.corner_level(cx, cz);
                    self.point(cx << lc, cz << lc, lc)
                } else {
                    self.point(gx >> (level - shared), gz >> (level - shared), shared)
                }
            };
            let span = n / m;
            let mut prev = coarse(0);
            for c in 0 .. span {
                let next = coarse(c + 1);
                for s in 0 .. m {
                    let along = c * m + s;
                    let p = if s == 0 { prev } else { prev + (next - prev) * (s as f64 / m as f64) };
                    pos[(j0 + dj * along) * (n + 1) + i0 + di * along] = p;
                }
                prev = next;
            }
            pos[(j0 + dj * n) * (n + 1) + i0 + di * n] = prev;
        }
        // Bounds of blocks of quads, from 2 x 2 up to the whole cell.
        let mut boxes: Vec<Vec<BBox>> = vec![];
        let mut size = 2;
        while size <= n {
            let blocks = n / size;
            let level_boxes = (0 .. blocks * blocks).map(|b| {
                let (bi, bj) = ((b % blocks) * size, (b / blocks) * size);
                let mut lo = Vector3::repeat(f64::INFINITY);
                let mut hi = Vector3::repeat(f64::NEG_INFINITY);
                for j in bj ..= bj + size {
                    for i in bi ..= bi + size {
                        let p = pos[j * (n + 1) + i];
                        lo = lo.inf(&p);
                        hi = hi.sup(&p);
                    }
                }
                BBox::new(lo, hi)
            }).collect();
            boxes.push(level_boxes);
            size *= 2;
        }
        Tile { n, pos, nrm, boxes }
    }

    fn tile(&self, k: usize) -> &Tile {
        self.tiles[k].get_or_init(|| {
            // Asked from outside a thread pool (placing a forest, say), a large cell is diced
            // on a pool shared by every displaced terrain. Not from a render's threads: one
            // waiting there takes up other pixels meanwhile, which may want this same cell (see
            // `geometry::build_apart`), and the render keeps every core busy anyway.
            if self.levels[k] >= 5 && rayon::current_thread_index().is_none()
                && let Some(pool) = DICING.get_or_init(|| rayon::ThreadPoolBuilder::new().build().ok()) {
                return pool.install(|| self.dice(k, true));
            }
            self.dice(k, false)
        })
    }

    /// The nearest hit with cell k, between t0 and t1 (multiples of the ray's direction): t, and
    /// the interpolated normal.
    fn intersect_cell(&self, r: &Ray, k: usize, t0: f64, t1: f64, any: bool) -> Option<(f64, Vector3<f64>)> {
        if ROUGHLY.with(Cell::get) {
            return self.intersect_base(r, k, t0, t1);
        }
        let coarse = COARSELY.load(std::sync::atomic::Ordering::Relaxed);
        let fresh: Arc<Tile>;
        let tile = if coarse > 0 && (self.levels[k] as u32) >= coarse && self.tiles[k].get().is_none() {
            let key = (self as *const DisplacedTerrain as usize, k);
            let kept = COARSE_TILES.lock().unwrap().get(&key).cloned();
            fresh = kept.unwrap_or_else(|| {
                let t = Arc::new(self.dice_at(k, coarse - 1, false));
                COARSE_TILES.lock().unwrap().insert(key, t.clone());
                t
            });
            &*fresh
        } else {
            self.tile(k)
        };
        let n = tile.n;
        let inv = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        let mut best: Option<(f64, Vector3<f64>)> = None;
        let mut far = t1;
        let quad = |i: usize, j: usize, far: f64| -> Option<(f64, Vector3<f64>)> {
            let v = |a: usize, b: usize| b * (n + 1) + a;
            let (a, b, c, d) = (v(i, j), v(i + 1, j), v(i, j + 1), v(i + 1, j + 1));
            let mut hit: Option<(f64, Vector3<f64>)> = None;
            for [p, q, s] in [[a, b, c], [d, c, b]] {
                if let Some((t, u, w)) = intersect_triangle(r, tile.pos[p], tile.pos[q], tile.pos[s])
                    && t >= t0 && t <= far && hit.is_none_or(|(ht, _)| t < ht) {
                    let nv = |x: usize| Vector3::new(tile.nrm[x][0] as f64, tile.nrm[x][1] as f64, tile.nrm[x][2] as f64);
                    let m = nv(p) * (1. - u - w) + nv(q) * u + nv(s) * w;
                    hit = Some((t, if m.norm() > 0. { m.normalize() } else { Vector3::y() }));
                }
            }
            hit
        };
        if tile.boxes.is_empty() {
            return quad(0, 0, far);
        }
        // Descend the quadtree, nearest block first: (depth in `boxes`, block x, block z).
        let top = tile.boxes.len() - 1;
        let mut stack: Vec<(usize, usize, usize, f64)> = Vec::with_capacity(4 * (top + 1));
        stack.push((top, 0, 0, t0));
        while let Some((depth, bi, bj, entry)) = stack.pop() {
            if entry > far { continue }
            let blocks = n >> (depth + 1);
            if slab(&tile.boxes[depth][bj * blocks + bi], &r.ro, &inv, t0, far).is_none() { continue }
            if depth == 0 {
                for (i, j) in [(2 * bi, 2 * bj), (2 * bi + 1, 2 * bj), (2 * bi, 2 * bj + 1), (2 * bi + 1, 2 * bj + 1)] {
                    if let Some(h) = quad(i, j, far) {
                        far = h.0;
                        best = Some(h);
                        if any { return best }
                    }
                }
                continue;
            }
            let child_blocks = blocks * 2;
            let mut children: [(usize, usize, f64); 4] = [(0, 0, 0.); 4];
            let mut count = 0;
            for (ci, cj) in [(2 * bi, 2 * bj), (2 * bi + 1, 2 * bj), (2 * bi, 2 * bj + 1), (2 * bi + 1, 2 * bj + 1)] {
                if let Some(e) = slab(&tile.boxes[depth - 1][cj * child_blocks + ci], &r.ro, &inv, t0, far) {
                    children[count] = (ci, cj, e);
                    count += 1;
                }
            }
            let children = &mut children[.. count];
            // Farthest pushed first, so the nearest is visited next.
            children.sort_by(|a, b| b.2.total_cmp(&a.2));
            for &(ci, cj, e) in children.iter() {
                stack.push((depth - 1, ci, cj, e));
            }
        }
        best
    }

    /// The nearest hit with cell k's two flat triangles through its heightmap's corners.
    fn intersect_base(&self, r: &Ray, k: usize, t0: f64, t1: f64) -> Option<(f64, Vector3<f64>)> {
        let g = &self.grid;
        let (ix, iz) = ((k % g.nx) as isize, (k / g.nx) as isize);
        let corner = |a: isize, b: isize| Vector3::new(
            g.min.0 + (ix + a) as f64 * g.cell, g.corner_clamped(ix + a, iz + b), g.min.1 + (iz + b) as f64 * g.cell);
        let (a, b, c, d) = (corner(0, 0), corner(1, 0), corner(0, 1), corner(1, 1));
        let mut best: Option<(f64, Vector3<f64>)> = None;
        for [p, q, s] in [[a, b, c], [d, c, b]] {
            if let Some((t, _, _)) = intersect_triangle(r, p, q, s)
                && t >= t0 && t <= t1 && best.is_none_or(|(bt, _)| t < bt) {
                let n = (s - p).cross(&(q - p));
                best = Some((t, if n.y < 0. { -n } else { n }.normalize()));
            }
        }
        best
    }

    /// Where the point `p`, on cell k's diced surface, was displaced from (see
    /// `Geometry::undisplaced`): found on the quad of the cell's tile nearest it, as the grid
    /// position its corners were displaced from, interpolated there.
    fn undisplaced_in(&self, p: &Vector3<f64>, k: usize) -> Option<crate::shapes::geometry::Undisplaced> {
        let g = &self.grid;
        let tile = self.tiles.get(k)?.get()?;
        let n = tile.n;
        let (ix, iz) = ((k % g.nx) as f64, (k / g.nx) as f64);
        let tol = 1e-6 * (1. + p.abs().max());
        let contains = |b: &BBox| (0 .. 3).all(|a| p[a] >= b.min[a] - tol && p[a] <= b.max[a] + tol);
        // Quads whose blocks hold p, down the quadtree.
        let mut quads: Vec<(usize, usize)> = vec![];
        if tile.boxes.is_empty() {
            quads.push((0, 0));
        } else {
            let mut stack = vec![(tile.boxes.len() - 1, 0usize, 0usize)];
            while let Some((depth, bi, bj)) = stack.pop() {
                let blocks = n >> (depth + 1);
                if !contains(&tile.boxes[depth][bj * blocks + bi]) { continue }
                for (ci, cj) in [(2 * bi, 2 * bj), (2 * bi + 1, 2 * bj), (2 * bi, 2 * bj + 1), (2 * bi + 1, 2 * bj + 1)] {
                    if depth == 0 { quads.push((ci, cj)) } else { stack.push((depth - 1, ci, cj)) }
                }
            }
        }
        // The nearest point on their triangles to p, and its grid coordinates in the cell.
        let mut best: Option<(f64, (f64, f64))> = None;
        for (i, j) in quads {
            for tri in [[(i, j), (i + 1, j), (i, j + 1)], [(i + 1, j + 1), (i, j + 1), (i + 1, j)]] {
                let at = |(a, b): (usize, usize)| tile.pos[b * (n + 1) + a];
                let (a, e1, e2) = (at(tri[0]), at(tri[1]) - at(tri[0]), at(tri[2]) - at(tri[0]));
                let (d11, d12, d22) = (e1.dot(&e1), e1.dot(&e2), e2.dot(&e2));
                let det = d11 * d22 - d12 * d12;
                if det <= 0. { continue }
                let v = p - a;
                let (v1, v2) = (v.dot(&e1), v.dot(&e2));
                let u = ((d22 * v1 - d12 * v2) / det).clamp(0., 1.);
                let w = ((d11 * v2 - d12 * v1) / det).clamp(0., 1. - u);
                let dist = (a + e1 * u + e2 * w - p).norm();
                if best.is_none_or(|b| dist < b.0) {
                    let grid_at = |(a, b): (usize, usize)| (a as f64, b as f64);
                    let (a, b, c) = (grid_at(tri[0]), grid_at(tri[1]), grid_at(tri[2]));
                    best = Some((dist, (a.0 + (b.0 - a.0) * u + (c.0 - a.0) * w, a.1 + (b.1 - a.1) * u + (c.1 - a.1) * w)));
                }
            }
        }
        let (dist, (fi, fj)) = best?;
        // Not on this cell (a `face` some wrapper set): no answer rather than a wrong one.
        if dist > 1e-3 * g.cell / n as f64 + tol {
            return None;
        }
        let (x, z) = (g.min.0 + (ix + fi / n as f64) * g.cell, g.min.1 + (iz + fj / n as f64) * g.cell);
        let (y, hx, hz) = g.smooth(x, z);
        Some(crate::shapes::geometry::Undisplaced {
            point: Vector3::new(x, y, z),
            normal: Vector3::new(-hx, 1., -hz).normalize(),
            filter: g.cell / n as f64,
        })
    }

    /// The nearest hit in [min, max) (world distances).
    fn nearest(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        let len = r.rd.norm();
        self.bvh.nearest(r, min, max, |i, best| {
            let k = self.cells[i] as usize;
            let inv = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
            let (t0, t1) = (min / len, best / len);
            slab(&self.cell_bounds[k], &r.ro, &inv, t0, t1)?;
            self.intersect_cell(r, k, t0, t1, false).map(|(t, normal)| RawIntersection {
                dist: t * len, point: r.ro + r.rd * t, normal, face: k as u32, part: 0,
            })
        }).map(|(_, x)| x)
    }
}

impl Geometry for DisplacedTerrain {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.nearest(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.nearest(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.nearest(r, min, max)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        let len = r.rd.norm();
        let inv = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        let (t0, t1) = (min / len, max / len);
        self.bvh.any_hit(r, min, max, |i| {
            let k = self.cells[i] as usize;
            slab(&self.cell_bounds[k], &r.ro, &inv, t0, t1).is_some()
                && self.intersect_cell(r, k, t0, t1, true).is_some_and(|(t, _)| t < t1)
        })
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn undisplaced(&self, p: &Vector3<f64>, face: u32) -> Option<crate::shapes::geometry::Undisplaced> {
        self.undisplaced_in(p, face as usize)
    }

    fn primitives(&self) -> u64 {
        (self.cells.len() * 2) as u64
    }

    fn flatten(&self) -> Option<Vec<crate::wavefront::flat::Prim>> {
        Some(self.flatten_faces()?.into_iter().map(|(p, _)| p).collect())
    }

    /// Every cell diced, as it would be once a ray met it (to its level for the camera, coarser
    /// out of its view), as triangles with the dicing's normals: for the kernels, which can't
    /// dice as they go. Each triangle's face is its cell, as a hit's is, so `undisplaced` finds
    /// its tile, which stays diced. Only within `FLATTEN_TRIANGLES`: a terrain the camera sees
    /// up close, diced finely where it looks, can come to hundreds of millions of triangles
    /// over all its cells (`displaced-terrain.json`: 200 million), of which rays meet a few.
    fn flatten_faces(&self) -> Option<Vec<(crate::wavefront::flat::Prim, u32)>> {
        self.flatten_within(FLATTEN_TRIANGLES)
    }
}

/// The most triangles a displaced terrain flattens to (`flatten_faces`); beyond them it stays
/// its own code's to intersect, diced as rays meet it. A landscape's patches about a camera
/// near the ground come to some 3 million (`terrain-alpine`: 5.3 million primitives in all, at
/// a peak of 6.9 GB on the GPU).
pub const FLATTEN_TRIANGLES: u64 = 6_000_000;

impl DisplacedTerrain {
    /// The triangles it would dice to, over every cell.
    pub fn diced_triangles(&self) -> u64 {
        self.cells.iter().map(|&k| 2u64 << (2 * self.levels[k as usize] as u64)).sum()
    }

    /// `flatten_faces`, if within `budget` triangles.
    fn flatten_within(&self, budget: u64) -> Option<Vec<(crate::wavefront::flat::Prim, u32)>> {
        use rayon::prelude::*;
        if self.diced_triangles() > budget {
            return None;
        }
        let triangles = |k: usize| -> Vec<(crate::wavefront::flat::Prim, u32)> {
            let tile = self.tile(k);
            let n = tile.n;
            let at = |i: usize, j: usize| j * (n + 1) + i;
            let normal = |x: usize| Vector3::new(tile.nrm[x][0] as f64, tile.nrm[x][1] as f64, tile.nrm[x][2] as f64);
            let mut out = Vec::with_capacity(2 * n * n);
            for j in 0 .. n {
                for i in 0 .. n {
                    let (a, b, c, d) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
                    // Wound so the geometric normal points up the base's normal, as the
                    // dicing's do: (c - a) x (b - a) with x and z increasing along b and c.
                    for [p, q, s] in [[a, c, b], [d, b, c]] {
                        out.push((crate::wavefront::flat::Prim::Triangle {
                            v: [tile.pos[p], tile.pos[q], tile.pos[s]],
                            n: Some([normal(p), normal(q), normal(s)]),
                        }, k as u32));
                    }
                }
            }
            out
        };
        Some(self.cells.par_iter().map(|&k| triangles(k as usize)).flatten().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn grid(n: usize, size: f64) -> Arc<HeightGrid> {
        let heights = (0 .. (n + 1) * (n + 1)).map(|k| {
            let (i, j) = ((k % (n + 1)) as f64, (k / (n + 1)) as f64);
            0.3 * (i * 0.7).sin() + 0.2 * (j * 0.5).cos()
        }).collect();
        Arc::new(HeightGrid::from_heights((-size / 2., -size / 2.), size / n as f64, n, n, heights))
    }

    fn terrain(g: Value, detail: Detail) -> DisplacedTerrain {
        DisplacedTerrain::new(grid(8, 8.), Graph::parse(&g).unwrap(), false, &detail).unwrap()
    }

    /// With a falloff, the edges lie on the undisplaced base (to meet a grid around them), and
    /// the displaced surface stays within its cells' bounds, whose range now reaches 0.
    #[test]
    fn test_placed_graphs_read_the_scene_and_fields() {
        struct East;
        impl crate::graph::Fields for East {
            fn field(&self, _: crate::graph::Field, p: &Vector3<f64>) -> Option<f64> { Some(p.x) }
        }
        let rise = |g: &DisplacedTerrain, x: f64, z: f64| g.intersects(&Ray { ro: Vector3::new(x, 50., z), rd: -Vector3::y() }).unwrap().point.y;
        let flat = || Arc::new(HeightGrid::from_heights((-4., -4.), 1., 8, 8, vec![0.; 81]));
        let detail = Detail { level: 2, ..Detail::default() };
        // Displaced by the scene's x, a hundred metres east of where the terrain thinks it is.
        let graph = || Graph::parse(&serde_json::json!({"node": "expr", "expr": "0.01 * x", "range": [-10, 10]})).unwrap();
        let here = DisplacedTerrain::new(flat(), graph(), true, &detail).unwrap();
        let there = DisplacedTerrain::new(flat(), graph(), true, &detail).unwrap()
            .placed(Vector3::new(100., 0., 0.), Matrix3::identity());
        assert!((rise(&here, 1.5, 0.5) - 0.015).abs() < 1e-9);
        assert!((rise(&there, 1.5, 0.5) - 1.015).abs() < 1e-9);
        // A field, read at the scene's points too.
        let sediment = || Graph::parse(&serde_json::json!({"node": "field", "field": "sediment", "from": 0, "to": 200})).unwrap();
        let unread = DisplacedTerrain::new(flat(), sediment(), true, &detail).unwrap();
        assert!(rise(&unread, 1.5, 0.5).abs() < 1e-9);
        let read = DisplacedTerrain::new(flat(), sediment(), true, &detail).unwrap()
            .placed(Vector3::new(100., 0., 0.), Matrix3::identity())
            .with_fields(Arc::new(East));
        assert!((rise(&read, 1.5, 0.5) - crate::procedural::nubis_clouds::smoothstep(0., 200., 101.5)).abs() < 1e-9);
    }

    #[test]
    fn test_roughly_is_undone_by_a_panic() {
        let seen = std::panic::catch_unwind(|| roughly(|| -> () { panic!("in a guess") }));
        assert!(seen.is_err());
        assert!(!ROUGHLY.with(|r| r.get()));
        assert!(roughly(|| ROUGHLY.with(|r| r.get())));
    }

    #[test]
    fn test_falloff_meets_the_base_at_the_edges() {
        // Displaced only upwards, so the bounds must widen to reach 0.
        let g = json!({"node": "fractal", "style": "ridged", "scale": 2, "smallest": 0.1, "amplitude": 0.4});
        let t = DisplacedTerrain::with_lift(grid(8, 8.), Graph::parse(&g).unwrap(), false, None, 1.5,
            &Detail { level: 3, ..Default::default() }).unwrap();
        assert!(t.range.0 <= 0.);
        let level = 3;
        let n = 8 << level;
        for q in 0 ..= n {
            for (gx, gz) in [(q, 0), (q, n), (0, q), (n, q)] {
                let p = t.point(gx, gz, level as u32);
                let (y, _, _) = t.grid.smooth(p.x, p.z);
                assert!((p.y - y).abs() < 1e-12, "edge vertex ({}, {}) displaced", gx, gz);
            }
        }
        // Inside, more than the falloff from the edges, it is displaced as without one.
        let plain = terrain(g, Detail { level: 3, ..Default::default() });
        assert_eq!(t.point(n / 2, n / 2, 3), plain.point(n / 2, n / 2, 3));
        for k in 0 .. 64 {
            let b = t.cell_bounds[k];
            for p in &t.dice(k, false).pos {
                assert!((0 .. 3).all(|a| p[a] >= b.min[a] && p[a] <= b.max[a]), "cell {}: {:?} outside {:?}", k, p, b);
            }
        }
    }

    /// A hit's undisplaced point is where the surface there was displaced from: straight
    /// below it for vertical displacement; along the base's normal otherwise, displacing it
    /// again lands on the hit (to within the quads' flatness).
    #[test]
    fn test_undisplaced_finds_where_hits_came_from() {
        let g = json!({"node": "fractal", "scale": 3, "smallest": 0.5, "amplitude": 0.3});
        for vertical in [true, false] {
            let t = DisplacedTerrain::new(grid(8, 8.), Graph::parse(&g).unwrap(), vertical, &Detail { level: 6, min_spacing: Some(0.01), ..Default::default() }).unwrap();
            let mut rng = crate::geometry::Pcg32::new(3, 9);
            let mut found = 0;
            for _ in 0 .. 500 {
                let ro = Vector3::new(rng.next_f64() * 7. - 3.5, 5., rng.next_f64() * 7. - 3.5);
                let rd = Vector3::new(rng.next_f64() - 0.5, -2., rng.next_f64() - 0.5);
                let Some(hit) = t.intersects(&Ray { ro, rd }) else { continue };
                let u = t.undisplaced(&hit.point, hit.face).expect("a hit's cell knows where it came from");
                let d = t.graph.eval(&Context::new(u.point, u.normal, 0.));
                let back = u.point + if vertical { Vector3::y() } else { u.normal } * d;
                assert!((back - hit.point).norm() < 2e-3, "{:?} back at {:?}, hit {:?}", vertical, back, hit.point);
                if vertical {
                    assert!((u.point.x - hit.point.x).abs() < 1e-9 && (u.point.z - hit.point.z).abs() < 1e-9);
                }
                found += 1;
            }
            assert!(found > 400);
            // A point on no cell's surface has no answer.
            assert!(t.undisplaced(&Vector3::new(0., 50., 0.), 0).is_none());
        }
    }

    #[test]
    fn test_flattened_cells_are_the_diced_surface() {
        // The triangles a terrain flattens to (for the kernels) are hit where the terrain is,
        // facing up, each numbered by its cell, whose tile `undisplaced` then reads.
        let g = json!({"node": "fractal", "scale": 3, "smallest": 0.5, "amplitude": 0.3});
        let t = DisplacedTerrain::new(grid(8, 8.), Graph::parse(&g).unwrap(), false, &Detail { level: 4, min_spacing: Some(0.05), ..Default::default() }).unwrap();
        // Counted before dicing, so a terrain too fine to hold whole (`FLATTEN_TRIANGLES`) isn't
        // diced to find out.
        assert!(t.flatten_within(t.diced_triangles() - 1).is_none());
        assert!(t.tiles.iter().all(|tile| tile.get().is_none()));
        let prims = t.flatten_faces().unwrap();
        let diced: usize = t.cells.iter().map(|&k| 2 * t.tile(k as usize).n.pow(2)).sum();
        assert_eq!((prims.len(), prims.len() as u64), (diced, t.diced_triangles()));
        let mut rng = crate::geometry::Pcg32::new(5, 2);
        let mut tested = 0;
        for _ in 0 .. 300 {
            let ro = Vector3::new(rng.next_f64() * 7. - 3.5, 5., rng.next_f64() * 7. - 3.5);
            let r = Ray { ro, rd: Vector3::new(rng.next_f64() - 0.5, -2., rng.next_f64() - 0.5) };
            let Some(hit) = t.intersects(&r) else { continue };
            // The nearest of the triangles, by brute force.
            let nearest = prims.iter().filter_map(|(p, face)| match p {
                crate::wavefront::flat::Prim::Triangle { v, .. } => intersect_triangle(&r, v[0], v[1], v[2]).map(|(d, _, _)| (d, *face, (v[1] - v[0]).cross(&(v[2] - v[0])).y)),
                _ => None,
            }).filter(|x| x.0 > 0.).min_by(|a, b| a.0.total_cmp(&b.0)).expect("the triangles are hit where the terrain is");
            assert!((nearest.0 * r.rd.norm() - hit.dist).abs() < 1e-9, "{} against {}", nearest.0 * r.rd.norm(), hit.dist);
            assert_eq!(nearest.1, hit.face);
            assert!(nearest.2 > 0., "wound to face up: the geometric normal, (v1 - v0) x (v2 - v0), along +y");
            assert!(t.undisplaced(&hit.point, nearest.1).is_some());
            tested += 1;
        }
        assert!(tested > 200, "{} rays hit", tested);
    }

    /// With no displacement, rays straight down land on the smooth base surface.
    #[test]
    fn test_zero_displacement_follows_the_base() {
        let t = terrain(json!(0), Detail { level: 5, ..Default::default() });
        let mut rng = crate::geometry::Pcg32::new(3, 1);
        for _ in 0 .. 200 {
            let (x, z) = (rng.next_f64() * 7. - 3.5, rng.next_f64() * 7. - 3.5);
            let hit = t.intersects(&Ray { ro: Vector3::new(x, 10., z), rd: -Vector3::y() }).expect("hit");
            let (h, _, _) = t.grid.smooth(x, z);
            assert!((hit.point.y - h).abs() < 2e-3, "({}, {}): {} vs {}", x, z, hit.point.y, h);
            assert!(hit.normal.y > 0.5);
        }
    }

    /// The surface is watertight where cells of different levels meet: rays aimed at the seams
    /// from all around never slip through.
    #[test]
    fn test_no_cracks_between_levels() {
        let g = json!({"node": "fractal", "scale": 1.5, "smallest": 0.01, "amplitude": 0.3});
        // A camera close to one corner: levels fall away from it.
        let detail = Detail { eye: Some((Vector3::new(-4., 1., -4.), 0.004)), max_level: 7, ..Default::default() };
        let t = terrain(g, detail);
        let (lo, hi) = (*t.levels.iter().min().unwrap(), *t.levels.iter().max().unwrap());
        assert!(hi > lo + 1, "levels {} to {}", lo, hi);
        let mut rng = crate::geometry::Pcg32::new(9, 4);
        let mut misses = 0;
        for _ in 0 .. 20000 {
            // Aim at a point on a cell boundary, from above at a random slant.
            let along = rng.next_f64() * 5. - 2.5;
            let line = -2. + (rng.next_f64() * 5.).floor();
            let (x, z) = if rng.next_f64() < 0.5 { (line, along) } else { (along, line) };
            // Through the seam to a point just below the whole surface, steeply enough to
            // cross the surface within the grid: it must hit.
            let dir = Vector3::new(0.6 * (rng.next_f64() - 0.5), -1., 0.6 * (rng.next_f64() - 0.5)).normalize();
            let r = Ray { ro: Vector3::new(x, -1.2, z) - dir * 20., rd: dir };
            if t.intersects(&r).is_none() { misses += 1 }
        }
        assert_eq!(misses, 0);
    }


    /// Shadow rays agree with the nearest hit.
    #[test]
    fn test_occlusion_agrees_with_intersection() {
        let g = json!({"node": "fractal", "style": "ridged", "scale": 2, "smallest": 0.05, "amplitude": 0.4});
        let t = terrain(g, Detail { level: 4, ..Default::default() });
        let mut rng = crate::geometry::Pcg32::new(1, 7);
        for _ in 0 .. 3000 {
            let ro = Vector3::new(rng.next_f64() * 8. - 4., rng.next_f64() * 2. - 0.5, rng.next_f64() * 8. - 4.);
            let rd = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
            let r = Ray { ro, rd };
            let max = 3.;
            let hit = t.intersects_range(&r, 1e-6, max);
            assert_eq!(hit.is_some(), t.occludes(&r, 1e-6, max), "{:?}", r);
        }
    }

    /// Every diced vertex lies within its cell's bounds, which are found before dicing, also
    /// on steep ground where the normal tilts the displacement sideways.
    #[test]
    fn test_diced_cells_stay_within_their_bounds() {
        let n = 8;
        let heights = (0 .. (n + 1) * (n + 1)).map(|k| {
            let (i, j) = ((k % (n + 1)) as f64, (k / (n + 1)) as f64);
            // A cliff across the middle, and rolling ground.
            6. / (1. + (-(i - 4.) * 3.).exp()) + 0.3 * (j * 0.9).sin()
        }).collect();
        let grid = Arc::new(HeightGrid::from_heights((-4., -4.), 1., n, n, heights));
        let g = json!({"node": "add", "inputs": [
            {"node": "fractal", "style": "ridged", "scale": 2, "smallest": 0.05, "amplitude": 0.8},
            {"node": "fractal", "scale": 1, "smallest": 0.05, "amplitude": -0.5, "seed": 3}]});
        let t = DisplacedTerrain::new(grid, Graph::parse(&g).unwrap(), false, &Detail { level: 5, ..Default::default() }).unwrap();
        let mut sideways = 0f64;
        for k in 0 .. n * n {
            let b = t.cell_bounds[k];
            let x0 = t.grid.min.0 + (k % n) as f64;
            sideways = sideways.max(x0 - b.min.x);
            for p in &t.dice(k, false).pos {
                assert!((0 .. 3).all(|a| p[a] >= b.min[a] && p[a] <= b.max[a]), "cell {}: {:?} outside {:?}", k, p, b);
            }
        }
        // On the cliff the bounds reach sideways; on the flat they'd be as wide as the
        // displacement's reach without the slopes.
        assert!(sideways > 0.3, "{}", sideways);
        let flat = t.cell_bounds[0];
        assert!(flat.max.x - flat.min.x < 1. + 0.8, "{:?}", flat);
    }

    /// Sand on the west, bare granite on the east, rising into a cliff between.
    struct SandAndGranite;

    impl crate::graph::Fields for SandAndGranite {
        fn field(&self, f: crate::graph::Field, p: &Vector3<f64>) -> Option<f64> {
            use crate::graph::Field;
            use crate::terrain::geology::Rock;
            let sandy = p.x < 0.;
            Some(match f {
                Field::Sediment => if sandy { 10. } else { 0. },
                Field::Sand => if sandy { 10. } else { 0. },
                Field::Rock => if sandy { 0. } else { Rock::Granite.index() as f64 },
                Field::Hardness => if sandy { 0. } else { -Rock::Granite.scores().0 },
                _ => 0.,
            })
        }
    }

    /// The landscape's own detail bounds each cell by the ground there: thinly on sand, more
    /// on rock, and every diced vertex stays within its cell's bounds.
    #[test]
    fn test_tightened_cells_hold_the_ground_detail() {
        let n = 8;
        let heights = (0 .. (n + 1) * (n + 1)).map(|k| {
            let i = (k % (n + 1)) as f64;
            6. / (1. + (-(i - 6.) * 2.).exp())
        }).collect();
        let grid = Arc::new(HeightGrid::from_heights((-4., -4.), 1., n, n, heights));
        let g = Graph::parse(&json!({"node": "ground", "seed": 2})).unwrap();
        let t = DisplacedTerrain::new(grid, g, false, &Detail { level: 5, ..Default::default() }).unwrap()
            .with_fields(Arc::new(SandAndGranite)).tighten();
        let height = |k: usize| t.cell_bounds[k].max.y - t.cell_bounds[k].min.y;
        // A sand cell's bounds are a fraction of the ±4 m reach; a granite cell's aren't.
        assert!(height(2 * n + 1) < 1.5, "sand {}", height(2 * n + 1));
        assert!(height(2 * n + 6) > 3., "granite {}", height(2 * n + 6));
        for k in 0 .. n * n {
            let b = t.cell_bounds[k];
            for p in &t.dice(k, false).pos {
                assert!((0 .. 3).all(|a| p[a] >= b.min[a] && p[a] <= b.max[a]), "cell {}: {:?} outside {:?}", k, p, b);
            }
        }
    }

    /// Cells outside the camera's view are diced coarser than those in it, at the same distance.
    #[test]
    fn test_cells_out_of_view_are_diced_coarser() {
        let g = json!({"node": "fractal", "scale": 1, "smallest": 0.001, "amplitude": 0.1});
        let eye = Vector3::new(0., 1., 0.);
        // Looking along +x, 30 degrees each way.
        let view = View { eye, right: Vector3::new(0., 0., 1.), up: Vector3::y(), forward: Vector3::x(),
            tan_x: 0.58, tan_y: 0.58, margin: 0. };
        assert!(view.excludes(&BBox::new(Vector3::new(-3., -1., -1.), Vector3::new(-2., 1., 1.))));
        assert!(!view.excludes(&BBox::new(Vector3::new(2., -1., -1.), Vector3::new(3., 1., 1.))));
        assert!(view.excludes(&BBox::new(Vector3::new(1., -1., 3.), Vector3::new(2., 1., 4.))));
        let detail = |view: Option<View>| Detail { eye: Some((eye, 0.002)), max_level: 10, view, offscreen: 4., ..Default::default() };
        let seen = terrain(g.clone(), detail(None));
        let culled = terrain(g, detail(Some(view)));
        // Cell (6, 4) lies ahead (+x), cell (1, 4) behind.
        let (ahead, behind) = (4 * 8 + 6, 4 * 8 + 1);
        assert_eq!(seen.levels[ahead], culled.levels[ahead]);
        assert_eq!(seen.levels[behind], culled.levels[behind] + 2);
    }

    /// Cells near the camera are diced finer than far ones.
    #[test]
    fn test_levels_fall_with_distance() {
        let g = json!({"node": "fractal", "scale": 1, "smallest": 0.001, "amplitude": 0.1});
        let t = terrain(g, Detail { eye: Some((Vector3::new(-3.5, 0.5, -3.5), 0.002)), max_level: 10, ..Default::default() });
        let near = t.levels[0];
        let far = t.levels[t.levels.len() - 1];
        assert!(near > far, "near {} far {}", near, far);
    }

    /// A terrain put elsewhere in the scene is diced for the camera as its copies see it: as
    /// finely as the nearest copy needs, and out of view only where every copy is.
    #[test]
    fn test_copies_are_diced_for_the_camera_as_they_see_it() {
        let g = json!({"node": "fractal", "scale": 1, "smallest": 0.001, "amplitude": 0.1});
        let shift = |x: f64| Affine3::from_matrix_unchecked(crate::na::Translation3::new(x, 0., 0.).to_homogeneous());
        let local_eye = Vector3::new(-3.5, 0.5, -3.5);
        let here = terrain(g.clone(), Detail { eye: Some((local_eye, 0.002)), max_level: 10, ..Default::default() });
        // The same view of a copy a kilometre east, and of that and another far off.
        let eye = local_eye + Vector3::new(1000., 0., 0.);
        let one = terrain(g.clone(), Detail { eye: Some((eye, 0.002)), max_level: 10, instances: vec![shift(1000.)], ..Default::default() });
        let two = terrain(g.clone(), Detail { eye: Some((eye, 0.002)), max_level: 10, instances: vec![shift(1000.), shift(-5000.)], ..Default::default() });
        assert_eq!(here.levels, one.levels);
        assert_eq!(here.levels, two.levels);
        // Taken where it stands, it would be diced as if a kilometre off.
        let unplaced = terrain(g.clone(), Detail { eye: Some((eye, 0.002)), max_level: 10, ..Default::default() });
        assert!(unplaced.levels.iter().max() < here.levels.iter().max());
        // A view through the copy's placement culls as it would where it stands.
        let view = View { eye: Vector3::new(0., 1., 0.), right: Vector3::new(0., 0., 1.), up: Vector3::y(), forward: Vector3::x(),
            tan_x: 0.58, tan_y: 0.58, margin: 0. };
        let moved = View { eye: view.eye + Vector3::new(1000., 0., 0.), ..view.clone() };
        let detail = |eye: Vector3<f64>, view: View, instances: Vec<Affine3<f64>>|
            Detail { eye: Some((eye, 0.002)), max_level: 10, view: Some(view), offscreen: 4., instances, ..Default::default() };
        let stands = terrain(g.clone(), detail(view.eye, view.clone(), vec![]));
        let copied = terrain(g, detail(moved.eye, moved.clone(), vec![shift(1000.)]));
        assert_eq!(stands.levels, copied.levels);
        // A copy scaled unevenly isn't culled at all (its view is no longer a frustum).
        let squash = Affine3::from_matrix_unchecked(crate::na::Matrix4::new_nonuniform_scaling(&Vector3::new(1., 2., 1.)));
        assert!(view.through(&squash).is_none());
        assert!(view.through(&shift(5.)).is_some());
    }

    /// A smooth surface sampled into grids of any cell.
    struct Hills;
    impl crate::procedural::heightmap::HeightSource for Hills {
        fn height(&self, x: f64, z: f64) -> f64 { 0.6 * (x * 0.37).sin() + 0.4 * (z * 0.23).cos() + 0.1 * (x * z * 0.05).sin() }
    }

    /// A terrain nested in one of twice its cell meets it without cracks, whatever the levels
    /// either side of the join.
    #[test]
    fn test_nested_terrains_meet_without_cracks() {
        let g = || Graph::parse(&json!({"node": "fractal", "scale": 1.5, "smallest": 0.01, "amplitude": 0.3})).unwrap();
        // A camera near one side of the inner terrain: the levels fall across the join.
        let detail = Detail { eye: Some((Vector3::new(-3., 1., 0.5), 0.004)), max_level: 7, ..Default::default() };
        let outer_grid = Arc::new(HeightGrid::sample(&Hills, (-16., -16.), (16., 16.), 2.));
        // The inner covers [-8, 8]^2: the outer's cells 4 to 12 each way.
        let outer = Arc::new(DisplacedTerrain::nested(outer_grid, g(), false, None, 0., &detail, Some((4, 12, 4, 12)), None).unwrap());
        let inner_grid = Arc::new(HeightGrid::sample(&Hills, (-8., -8.), (8., 8.), 1.));
        let inner = DisplacedTerrain::nested(inner_grid, g(), false, None, 0., &detail, None,
            Some(Nest { outer: outer.clone(), offset: (8, 8) })).unwrap();
        let both = crate::shapes::group::Group::new(vec![Box::new(outer.clone()), Box::new(inner)]);
        let mut rng = crate::geometry::Pcg32::new(5, 7);
        let mut misses = 0;
        for _ in 0 .. 20000 {
            // At a point on the join, from above at a slant, to just below the whole surface.
            let along = rng.next_f64() * 16. - 8.;
            let side = if rng.next_f64() < 0.5 { -8. } else { 8. };
            let (x, z) = if rng.next_f64() < 0.5 { (side, along) } else { (along, side) };
            let dir = Vector3::new(0.6 * (rng.next_f64() - 0.5), -1., 0.6 * (rng.next_f64() - 0.5)).normalize();
            let r = Ray { ro: Vector3::new(x, -1.8, z) - dir * 30., rd: dir };
            if both.intersects(&r).is_none() { misses += 1 }
        }
        assert_eq!(misses, 0);
        // The hole is left to the inner: straight down its middle, only the inner is hit.
        assert!(outer.intersects(&Ray { ro: Vector3::new(0.3, 20., 0.2), rd: -Vector3::y() }).is_none());
        assert!(both.intersects(&Ray { ro: Vector3::new(0.3, 20., 0.2), rd: -Vector3::y() }).is_some());
    }

    #[test]
    fn test_nesting_must_line_up() {
        let g = || Graph::parse(&json!(0)).unwrap();
        let outer = Arc::new(DisplacedTerrain::nested(Arc::new(HeightGrid::sample(&Hills, (-16., -16.), (16., 16.), 2.)), g(), false,
            None, 0., &Detail::default(), Some((4, 12, 4, 12)), None).unwrap());
        let misplaced = Arc::new(HeightGrid::sample(&Hills, (-7., -8.), (9., 8.), 1.));
        assert!(DisplacedTerrain::nested(misplaced, g(), false, None, 0., &Detail::default(), None,
            Some(Nest { outer, offset: (8, 8) })).is_err());
    }

    fn sideways(base: impl Fn(f64) -> f64, sideways: Sideways) -> DisplacedTerrain {
        let n = 32;
        let heights = (0 .. (n + 1) * (n + 1)).map(|k| base(-4. + (k % (n + 1)) as f64 * 0.25)).collect();
        let grid = Arc::new(HeightGrid::from_heights((-4., -4.), 0.25, n, n, heights));
        let d = Displacement { graph: Graph::parse(&json!(0)).unwrap(), vertical: false, lift: None, sideways, falloff: 0.,
            detail: Detail { level: 4, min_spacing: Some(0.01), ..Default::default() } };
        DisplacedTerrain::of(grid, d, None, None).unwrap()
    }

    /// Moved along x by a constant, a slope y = x / 2 is the same slope a metre east.
    #[test]
    fn test_vector_displacement_moves_points_its_own_way() {
        let t = sideways(|x| 0.5 * x, Sideways { x: Some(Graph::parse(&json!(1)).unwrap()), ..Default::default() });
        for x in [-1.5, 0., 1.3] {
            let hit = t.intersects(&Ray { ro: Vector3::new(x, 10., 0.3), rd: -Vector3::y() }).unwrap();
            assert!((hit.point.y - 0.5 * (x - 1.)).abs() < 1e-3, "{} at {}", hit.point.y, x);
        }
    }

    /// A step pushed outward above a metre overhangs the ground in front of it: straight up
    /// from there, a ray meets its underside, which it doesn't without.
    #[test]
    fn test_outward_displacement_makes_overhangs() {
        let step = |x: f64| 3. * ((x / 0.5).clamp(0., 1.));
        let out = || Some(Graph::parse(&json!({"node": "multiply", "inputs": [1.5, {"node": "altitude", "from": 1, "to": 1.5}]})).unwrap());
        let overhang = sideways(step, Sideways { outward: out(), ..Default::default() });
        let plain = sideways(step, Sideways::default());
        let up = Ray { ro: Vector3::new(-0.8, 0.5, 0.2), rd: Vector3::y() };
        let under = overhang.intersects(&up).expect("no overhang");
        assert!(under.point.y > 1. && under.point.y < 3., "{}", under.point.y);
        assert!(under.normal.y < 0., "an underside faces down: {:?}", under.normal);
        assert!(plain.intersects(&up).is_none());
        // And rays from above all still meet the ground.
        for k in 0..200 {
            let x = -3. + 6. * k as f64 / 200.;
            assert!(overhang.intersects(&Ray { ro: Vector3::new(x, 10., 0.1), rd: -Vector3::y() }).is_some(), "hole at {}", x);
        }
    }
}
