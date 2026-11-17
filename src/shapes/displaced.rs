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
use crate::na::Vector3;
use crate::procedural::heightmap::{intersect_triangle, HeightGrid};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use std::cell::Cell;
use std::sync::{Arc, OnceLock};

thread_local! {
    static ROUGHLY: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with displaced terrains seen roughly, as their heightmaps' flat cells, without
/// dicing any: for guesses that needn't be exact, such as which of the sky the camera sees
/// (`environment`). A ray grazing the ground from the camera passes close to many cells, and
/// dicing them all for that guess cost more than the render.
pub fn roughly<T>(f: impl FnOnce() -> T) -> T {
    let before = ROUGHLY.with(|r| r.replace(true));
    let v = f();
    ROUGHLY.with(|r| r.set(before));
    v
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
}

impl Default for Detail {
    fn default() -> Detail {
        Detail { eye: None, pixels: 1., max_level: 8, min_spacing: None, level: 4, view: None, offscreen: 1. }
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
    /// The displacement's range.
    range: (f64, f64),
    /// Each cell's level, and conservative bounds.
    levels: Vec<u8>,
    cell_bounds: Vec<BBox>,
    tiles: Vec<OnceLock<Tile>>,
    /// Threads for dicing large cells asked for from outside a thread pool.
    pool: OnceLock<Option<rayon::ThreadPool>>,
    /// Over the cells, by index into `cells`.
    bvh: Bvh,
    cells: Vec<u32>,
    bounds: BBox,
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

impl DisplacedTerrain {
    pub fn new(grid: Arc<HeightGrid>, graph: Graph, vertical: bool, detail: &Detail) -> Result<DisplacedTerrain, String> {
        use rayon::prelude::*;
        let range = graph.range();
        if !(range.0.is_finite() && range.1.is_finite()) {
            return Err("a terrain's displacement must be bounded: give an expression a \"range\"".into());
        }
        let reach = range.0.abs().max(range.1.abs());
        let (nx, nz, cell) = (grid.nx, grid.nz, grid.cell);
        let cell_bounds: Vec<BBox> = (0 .. nx * nz).into_par_iter().map(|k| {
            let (ix, iz) = (k % nx, k / nx);
            let (lo, hi) = grid.smooth_range(ix, iz);
            let (x0, z0) = (grid.min.0 + ix as f64 * cell, grid.min.1 + iz as f64 * cell);
            let (min, max) = if vertical {
                (Vector3::new(x0, lo + range.0, z0), Vector3::new(x0 + cell, hi + range.1, z0 + cell))
            } else {
                // Moved along the base's normal, (-hx, 1, -hz) normalised: sideways by at most
                // reach times |hx| / sqrt(1 + hx^2) (and so in z), and up or down by the
                // displacement times a y component between 1 and 1 / sqrt(1 + hx^2 + hz^2). On
                // gentle ground that keeps the bounds to a little more than the cell, so rays
                // dice only cells they come near.
                let (sx, sz) = grid.smooth_slope_bounds(ix, iz);
                let side = |s: f64| reach * (s / (1. + s * s).sqrt()).min(1.);
                let (rx, rz) = (side(sx), side(sz));
                let ny = 1. / (1. + sx * sx + sz * sz).sqrt();
                let down = if range.0 < 0. { range.0 } else { range.0 * ny };
                let up = if range.1 > 0. { range.1 } else { range.1 * ny };
                (Vector3::new(x0 - rx, lo + down, z0 - rz), Vector3::new(x0 + cell + rx, hi + up, z0 + cell + rz))
            };
            // Room for rounding.
            let pad = Vector3::repeat(1e-9 * (1. + min.abs().max().max(max.abs().max())));
            BBox::new(min - pad, max + pad)
        }).collect();
        // Dice no finer than half the finest detail, or an eighth of a cell, so the smooth base
        // itself looks smooth.
        // A steep cell's surface is longer than the cell is wide: on a cliff, a 1.5 m cell may
        // span 20 m of rock. Its level is chosen for the longer of the two, so quads up a cliff
        // are no taller than a pixel rather than smearing the relief into vertical streaks.
        let min_spacing = detail.min_spacing.unwrap_or_else(|| (0.5 * graph.finest()).min(cell / 8.)).max(1e-6);
        let levels: Vec<u8> = cell_bounds.par_iter().enumerate().map(|(k, b)| {
            let (lo, hi) = grid.smooth_range(k % nx, k / nx);
            let extent = cell.max(hi - lo);
            let finest = ((extent / min_spacing).log2().floor().max(0.) as u32).min(detail.max_level);
            let level = match detail.eye {
                None => detail.level,
                Some((eye, pixel_angle)) => {
                    let nearest = Vector3::new(eye.x.clamp(b.min.x, b.max.x), eye.y.clamp(b.min.y, b.max.y), eye.z.clamp(b.min.z, b.max.z));
                    // Cells out of view are seen only by the light they shade or reflect.
                    let unseen = detail.view.as_ref().is_some_and(|v| v.excludes(b));
                    let footprint = (nearest - eye).norm().max(1e-6) * pixel_angle * detail.pixels
                        * if unseen { detail.offscreen.max(1.) } else { 1. };
                    (extent / footprint).log2().ceil().max(0.) as u32
                },
            };
            level.min(finest).min(15) as u8
        }).collect();
        let (bvh, cells) = Bvh::build((0 .. (nx * nz) as u32).collect(), |&k| cell_bounds[k as usize]);
        let bounds = bvh.bounds();
        let vertices: f64 = levels.iter().map(|&l| ((1u64 << l) as f64 + 1.).powi(2)).sum();
        println!("Displaced terrain: {} x {} cells diced to levels {} to {}, up to {:.1} million vertices if all are seen",
            nx, nz, levels.iter().min().unwrap_or(&0), levels.iter().max().unwrap_or(&0), vertices / 1e6);
        let tiles = (0 .. nx * nz).map(|_| OnceLock::new()).collect();
        Ok(DisplacedTerrain { grid, graph, vertical, range, levels, cell_bounds, tiles, pool: OnceLock::new(), bvh, cells, bounds })
    }

    /// The displacement's range.
    pub fn displacement_range(&self) -> (f64, f64) {
        self.range
    }

    fn level(&self, ix: isize, iz: isize) -> Option<u32> {
        let (nx, nz) = (self.grid.nx as isize, self.grid.nz as isize);
        (ix >= 0 && iz >= 0 && ix < nx && iz < nz).then(|| self.levels[(iz * nx + ix) as usize] as u32)
    }

    /// The displaced surface at grid vertex (gx, gz) of level `level` (2^level vertices to a
    /// cell), filtered for that level: computed from the integers alone, so every cell that
    /// shares the point computes it identically.
    fn point(&self, gx: i64, gz: i64, level: u32) -> Vector3<f64> {
        let g = &self.grid;
        let scale = 0.5f64.powi(level as i32);
        let (x, z) = (g.min.0 + (gx as f64 * g.cell) * scale, g.min.1 + (gz as f64 * g.cell) * scale);
        let (y, hx, hz) = g.smooth(x, z);
        let base = Vector3::new(x, y, z);
        let normal = Vector3::new(-hx, 1., -hz).normalize();
        let d = self.graph.eval(&Context::new(base, normal, g.cell * scale));
        base + if self.vertical { Vector3::y() } else { normal } * d
    }

    /// The level whose filter the grid corner (cx, cz) is evaluated at: the coarsest of the
    /// cells that share it.
    fn corner_level(&self, cx: i64, cz: i64) -> u32 {
        [(cx - 1, cz - 1), (cx, cz - 1), (cx - 1, cz), (cx, cz)].iter()
            .filter_map(|&(i, j)| self.level(i as isize, j as isize)).min().unwrap_or(0)
    }

    fn dice(&self, k: usize, parallel: bool) -> Tile {
        let (ix, iz) = ((k % self.grid.nx) as i64, (k / self.grid.nx) as i64);
        let level = self.levels[k] as u32;
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
            // on a pool of the terrain's own. Not from a render's threads: one waiting there
            // takes up other pixels meanwhile, which may want this same cell (see
            // `geometry::build_apart`), and the render keeps every core busy anyway.
            if self.levels[k] >= 5 && rayon::current_thread_index().is_none()
                && let Some(pool) = self.pool.get_or_init(|| rayon::ThreadPoolBuilder::new().build().ok()) {
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
        let tile = self.tile(k);
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

    /// The nearest hit in [min, max) (world distances).
    fn nearest(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        let len = r.rd.norm();
        self.bvh.nearest(r, min, max, |i, best| {
            let k = self.cells[i] as usize;
            let inv = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
            let (t0, t1) = (min / len, best / len);
            slab(&self.cell_bounds[k], &r.ro, &inv, t0, t1)?;
            self.intersect_cell(r, k, t0, t1, false).map(|(t, normal)| RawIntersection {
                dist: t * len, point: r.ro + r.rd * t, normal, face: 0, part: 0,
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

    fn primitives(&self) -> u64 {
        (self.grid.nx * self.grid.nz * 2) as u64
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
}
