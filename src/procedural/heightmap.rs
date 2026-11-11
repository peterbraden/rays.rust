//! # Heightmaps and terrain
//!
//! A heightmap gives the ground's height at every (x, z). Sources are anything that can answer
//! that question - a flat plane, fractal noise, a greyscale image - and are sampled onto a
//! regular grid, a `HeightGrid`. The grid, split into triangles, *is* the terrain: everything
//! placed on the ground (eg. grass) asks the grid for its height, so it sits exactly on the
//! surface that is rendered, however coarse the grid.
//!
//! In a scene file a heightmap is an object:
//!
//! ```json
//! { "type": "noise", "min": [-20, -20], "max": [20, 20], "resolution": 200,
//!   "amplitude": 2, "scale": 15, "octaves": 5, "seed": 3 }
//! ```
//!
//! - `min`, `max`: the [x, z] corners of the area covered (default [-10, -10] to [10, 10])
//! - `resolution`: grid cells along the longer side (default 128), or `cell_size` in metres
//! - `base`: height added everywhere (default 0)
//!
//! and by `type`:
//! - `flat`: nothing more
//! - `noise`: fractal Brownian motion. `amplitude` (metres either side of `base`, default 1),
//!   `scale` (size of the largest features in metres, default 10), `octaves` (default 5),
//!   `persistence` (default 0.5), `lacunarity` (default 2), `ridged` (default false: sharp
//!   crests, as of eroded hills), `seed`
//! - `image`: a greyscale image stretched over the area, black at `base` and white at `base +
//!   height`. `src` (path), `height` (default 1)
//!
//! To share one heightmap between a terrain and the grass on it, define it once in the scene's
//! `variables` and refer to it as `"$name"`.
//!
//! The `terrain` geometry (`{"type": "terrain", "heightmap": {...}, "material": ...}`)
//! renders the grid. It is intersected by walking the ray across the grid cells in order
//! (`walk_grid`), testing only the two triangles of each cell the ray passes over, and only
//! when the ray is within the cell's height range.

use crate::na::{Vector3};
use crate::noise::PerlinNoise;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use crate::scenefile::SceneFile;
use serde_json::Value;
use std::sync::Arc;

/// Anything that can give the height of the ground at (x, z).
pub trait HeightSource: Sync + Send {
    fn height(&self, x: f64, z: f64) -> f64;
}

pub struct Flat {
    pub base: f64,
}

impl HeightSource for Flat {
    fn height(&self, _x: f64, _z: f64) -> f64 { self.base }
}

/// Fractal Brownian motion: octaves of Perlin noise, each at a higher frequency and lower
/// amplitude than the last (Ken Musgrave in Ebert et al., "Texturing and Modeling: A Procedural
/// Approach", 3rd ed. 2003, ch. 14). Ridged noise folds each octave about zero, `1 - |n|`,
/// which gives sharp crests where the noise crosses zero (ibid., "ridged multifractal").
pub struct NoiseHeight {
    pub noise: PerlinNoise,
    pub base: f64,
    pub amplitude: f64,
    pub scale: f64,
    pub octaves: u32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub ridged: bool,
    /// Offset into noise space, so different seeds give different terrain.
    pub offset: (f64, f64),
}

impl HeightSource for NoiseHeight {
    fn height(&self, x: f64, z: f64) -> f64 {
        let (nx, nz) = (x / self.scale + self.offset.0, z / self.scale + self.offset.1);
        let n = if self.ridged {
            let (mut sum, mut amp, mut freq, mut total) = (0., 1., 1., 0.);
            for _ in 0 .. self.octaves {
                let r = 1. - self.noise.noise(nx * freq, 0.5, nz * freq).abs();
                sum += r * r * amp;
                total += amp;
                amp *= self.persistence;
                freq *= self.lacunarity;
            }
            if total > 0. { sum / total } else { 0.5 }
        } else {
            self.noise.fbm(nx, 0.5, nz, self.octaves, self.persistence, self.lacunarity)
        };
        self.base + self.amplitude * (2. * n - 1.)
    }
}

/// A greyscale image stretched over [min, max], sampled bilinearly. Row 0 of the image is at
/// min z, column 0 at min x.
pub struct ImageHeight {
    pub width: usize,
    pub height: usize,
    /// Brightness in [0, 1], row by row.
    pub values: Vec<f64>,
    pub min: (f64, f64),
    pub max: (f64, f64),
    pub base: f64,
    pub scale: f64,
}

impl ImageHeight {
    pub fn from_file(path: &str, min: (f64, f64), max: (f64, f64), base: f64, scale: f64) -> ImageHeight {
        let img = image::open(path)
            .unwrap_or_else(|e| panic!("Can't read heightmap image {}: {}", path, e))
            .to_luma16();
        let (w, h) = (img.width() as usize, img.height() as usize);
        let values = img.pixels().map(|p| p.0[0] as f64 / 65535.).collect();
        ImageHeight { width: w, height: h, values, min, max, base, scale }
    }

    fn texel(&self, x: usize, y: usize) -> f64 {
        self.values[y.min(self.height - 1) * self.width + x.min(self.width - 1)]
    }
}

impl HeightSource for ImageHeight {
    fn height(&self, x: f64, z: f64) -> f64 {
        let u = ((x - self.min.0) / (self.max.0 - self.min.0)).clamp(0., 1.) * (self.width - 1) as f64;
        let v = ((z - self.min.1) / (self.max.1 - self.min.1)).clamp(0., 1.) * (self.height - 1) as f64;
        let (x0, y0) = (u.floor() as usize, v.floor() as usize);
        let (fx, fy) = (u - x0 as f64, v - y0 as f64);
        let top = self.texel(x0, y0) * (1. - fx) + self.texel(x0 + 1, y0) * fx;
        let bottom = self.texel(x0, y0 + 1) * (1. - fx) + self.texel(x0 + 1, y0 + 1) * fx;
        self.base + self.scale * (top * (1. - fy) + bottom * fy)
    }
}

/// Heights sampled on a regular grid over [min, max] in x and z, with `nx` by `nz` cells.
///
/// The surface is the grid split into triangles: each cell, with corners a (low x, low z), b
/// (high x), c (high z) and d, is split along the diagonal from b to c. `height` interpolates
/// on those triangles, so it agrees exactly with the rendered terrain.
pub struct HeightGrid {
    pub min: (f64, f64),
    pub cell: f64,
    pub nx: usize,
    pub nz: usize,
    /// (nx + 1) * (nz + 1) heights, row by row along x.
    heights: Vec<f64>,
    /// Lowest and highest corner of each cell, row by row.
    ranges: Vec<(f64, f64)>,
    /// Lowest and highest point of the whole surface.
    pub y_range: (f64, f64),
}

impl HeightGrid {
    /// Sample `source` over [min, max] with square cells of size about `cell`.
    pub fn sample(source: &dyn HeightSource, min: (f64, f64), max: (f64, f64), cell: f64) -> HeightGrid {
        assert!(max.0 > min.0 && max.1 > min.1, "heightmap max must exceed min");
        let nx = ((max.0 - min.0) / cell).ceil().max(1.) as usize;
        let nz = ((max.1 - min.1) / cell).ceil().max(1.) as usize;
        // Cells exactly fill the area in x; z rounds up to whole cells.
        let cell = (max.0 - min.0) / nx as f64;
        let nz = nz.max(((max.1 - min.1) / cell).round().max(1.) as usize);
        let mut heights = Vec::with_capacity((nx + 1) * (nz + 1));
        for iz in 0 ..= nz {
            for ix in 0 ..= nx {
                heights.push(source.height(min.0 + ix as f64 * cell, min.1 + iz as f64 * cell));
            }
        }
        HeightGrid::from_heights(min, cell, nx, nz, heights)
    }

    pub fn from_heights(min: (f64, f64), cell: f64, nx: usize, nz: usize, heights: Vec<f64>) -> HeightGrid {
        assert_eq!(heights.len(), (nx + 1) * (nz + 1));
        let mut ranges = Vec::with_capacity(nx * nz);
        let mut y_range = (f64::INFINITY, f64::NEG_INFINITY);
        for iz in 0 .. nz {
            for ix in 0 .. nx {
                let i = iz * (nx + 1) + ix;
                let c = [heights[i], heights[i + 1], heights[i + nx + 1], heights[i + nx + 2]];
                let lo = c.iter().cloned().fold(f64::INFINITY, f64::min);
                let hi = c.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                ranges.push((lo, hi));
                y_range = (y_range.0.min(lo), y_range.1.max(hi));
            }
        }
        HeightGrid { min, cell, nx, nz, heights, ranges, y_range }
    }

    /// Parse a heightmap object (see the module documentation) and sample it.
    pub fn from_json(o: &Value) -> HeightGrid {
        let corner = |k: &str, def: f64| -> (f64, f64) {
            match o.get(k) {
                Some(v) => (v[0].as_f64().unwrap(), v[1].as_f64().unwrap()),
                None => (def, def),
            }
        };
        let (min, max) = (corner("min", -10.), corner("max", 10.));
        let longest = (max.0 - min.0).max(max.1 - min.1);
        let cell = match o.get("cell_size").and_then(Value::as_f64) {
            Some(c) => c,
            None => longest / SceneFile::parse_int(&o["resolution"], 128) as f64,
        };
        let base = SceneFile::parse_number(&o["base"], 0.);
        let seed = SceneFile::parse_int(&o["seed"], 0) as f64;
        let source: Box<dyn HeightSource> = match o["type"].as_str().unwrap_or("flat") {
            "flat" => Box::new(Flat { base }),
            "noise" => Box::new(NoiseHeight {
                noise: PerlinNoise::new(),
                base,
                amplitude: SceneFile::parse_number(&o["amplitude"], 1.),
                scale: SceneFile::parse_number(&o["scale"], 10.),
                octaves: SceneFile::parse_int(&o["octaves"], 5) as u32,
                persistence: SceneFile::parse_number(&o["persistence"], 0.5),
                lacunarity: SceneFile::parse_number(&o["lacunarity"], 2.),
                ridged: o["ridged"].as_bool().unwrap_or(false),
                offset: (seed * 31.17 + 0.37, seed * 17.73 + 0.71),
            }),
            "image" => Box::new(ImageHeight::from_file(
                o["src"].as_str().expect("image heightmap needs a \"src\""),
                min, max, base, SceneFile::parse_number(&o["height"], 1.))),
            t => panic!("Unknown heightmap type {}", t),
        };
        HeightGrid::sample(&*source, min, max, cell)
    }

    pub fn max(&self) -> (f64, f64) {
        (self.min.0 + self.nx as f64 * self.cell, self.min.1 + self.nz as f64 * self.cell)
    }

    pub fn contains(&self, x: f64, z: f64) -> bool {
        let max = self.max();
        x >= self.min.0 && x <= max.0 && z >= self.min.1 && z <= max.1
    }

    fn corner(&self, ix: usize, iz: usize) -> f64 {
        self.heights[iz * (self.nx + 1) + ix]
    }

    /// Cell containing (x, z), clamped to the grid, and the position within it in [0, 1].
    fn locate(&self, x: f64, z: f64) -> (usize, usize, f64, f64) {
        let u = ((x - self.min.0) / self.cell).clamp(0., self.nx as f64);
        let v = ((z - self.min.1) / self.cell).clamp(0., self.nz as f64);
        let ix = (u.floor() as usize).min(self.nx - 1);
        let iz = (v.floor() as usize).min(self.nz - 1);
        (ix, iz, u - ix as f64, v - iz as f64)
    }

    /// The four corners of a cell, a (low x, low z), b (high x), c (high z), d.
    fn cell_corners(&self, ix: usize, iz: usize) -> [Vector3<f64>; 4] {
        let p = |i: usize, j: usize| Vector3::new(
            self.min.0 + i as f64 * self.cell, self.corner(i, j), self.min.1 + j as f64 * self.cell);
        [p(ix, iz), p(ix + 1, iz), p(ix, iz + 1), p(ix + 1, iz + 1)]
    }

    /// Height of the triangulated surface at (x, z). Outside the grid, the height at the
    /// nearest edge.
    pub fn height(&self, x: f64, z: f64) -> f64 {
        let (ix, iz, fx, fz) = self.locate(x, z);
        let (a, b, c, d) = (self.corner(ix, iz), self.corner(ix + 1, iz), self.corner(ix, iz + 1), self.corner(ix + 1, iz + 1));
        if fx + fz <= 1. {
            a + fx * (b - a) + fz * (c - a)
        } else {
            d + (1. - fx) * (c - d) + (1. - fz) * (b - d)
        }
    }

    /// The (unsmoothed) upward normal of the triangle under (x, z).
    pub fn normal(&self, x: f64, z: f64) -> Vector3<f64> {
        let (ix, iz, fx, fz) = self.locate(x, z);
        let [a, b, c, d] = self.cell_corners(ix, iz);
        let n = if fx + fz <= 1. { (c - a).cross(&(b - a)) } else { (b - d).cross(&(c - d)) };
        n.normalize()
    }

    /// Normal at a grid vertex, from central differences of its neighbours.
    fn vertex_normal(&self, ix: usize, iz: usize) -> Vector3<f64> {
        let h = |i: isize, j: isize| self.corner(
            i.clamp(0, self.nx as isize) as usize, j.clamp(0, self.nz as isize) as usize);
        let (i, j) = (ix as isize, iz as isize);
        let dx = (h(i + 1, j) - h(i - 1, j)) / (((i + 1).min(self.nx as isize) - (i - 1).max(0)) as f64 * self.cell);
        let dz = (h(i, j + 1) - h(i, j - 1)) / (((j + 1).min(self.nz as isize) - (j - 1).max(0)) as f64 * self.cell);
        Vector3::new(-dx, 1., -dz).normalize()
    }

    pub fn bounds(&self) -> BBox {
        let max = self.max();
        BBox::new(
            Vector3::new(self.min.0, self.y_range.0, self.min.1),
            Vector3::new(max.0, self.y_range.1, max.1))
    }

    /// Nearest intersection of the ray with the surface within the cell (ix, iz), between t0 and
    /// t1 (multiples of the ray direction): t, and the normal interpolated from the vertices'.
    fn intersect_cell(&self, r: &Ray, ix: usize, iz: usize, t0: f64, t1: f64) -> Option<(f64, Vector3<f64>)> {
        let [a, b, c, d] = self.cell_corners(ix, iz);
        let [na, nb, nc, nd] = [self.vertex_normal(ix, iz), self.vertex_normal(ix + 1, iz),
            self.vertex_normal(ix, iz + 1), self.vertex_normal(ix + 1, iz + 1)];
        let mut best: Option<(f64, Vector3<f64>)> = None;
        for (p, n) in [((a, b, c), (na, nb, nc)), ((d, c, b), (nd, nc, nb))] {
            if let Some((t, u, v)) = intersect_triangle(r, p.0, p.1, p.2)
                && t >= t0 && t <= t1 && best.is_none_or(|(bt, _)| t < bt) {
                best = Some((t, (n.0 * (1. - u - v) + n.1 * u + n.2 * v).normalize()));
            }
        }
        best
    }
}

/// Möller-Trumbore ray-triangle intersection, two-sided: (t, u, v) where the hit is at
/// `ro + t rd` = (1 - u - v) a + u b + v c. Tomas Möller and Ben Trumbore, "Fast, Minimum
/// Storage Ray-Triangle Intersection", Journal of Graphics Tools 2(1), 1997,
/// doi:10.1080/10867651.1997.10487468.
#[inline]
pub fn intersect_triangle(r: &Ray, a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>) -> Option<(f64, f64, f64)> {
    let e1 = b - a;
    let e2 = c - a;
    let p = r.rd.cross(&e2);
    let det = e1.dot(&p);
    if det.abs() < 1e-14 { return None }
    let inv = 1. / det;
    let s = r.ro - a;
    let u = s.dot(&p) * inv;
    if !(0. ..= 1.).contains(&u) { return None }
    let q = s.cross(&e1);
    let v = r.rd.dot(&q) * inv;
    if v < 0. || u + v > 1. { return None }
    let t = e2.dot(&q) * inv;
    if t > 1e-9 { Some((t, u, v)) } else { None }
}

/// A regular grid of square cells in the xz plane, for walking rays across.
#[derive(Clone, Copy, Debug)]
pub struct Grid2 {
    pub min: (f64, f64),
    pub cell: f64,
    pub nx: usize,
    pub nz: usize,
}

/// Visit the cells of `grid` that the ray passes over, in order, within the vertical slab
/// [y_lo, y_hi]: `visit(ix, iz, t0, t1)` gets the range of t (multiples of the ray direction)
/// over which the ray is above the cell, and returns true to stop. A 2D version of John
/// Amanatides and Andrew Woo, "A Fast Voxel Traversal Algorithm for Ray Tracing",
/// Eurographics 1987.
pub fn walk_grid(grid: &Grid2, y_lo: f64, y_hi: f64, r: &Ray, mut visit: impl FnMut(usize, usize, f64, f64) -> bool) {
    let max = (grid.min.0 + grid.nx as f64 * grid.cell, grid.min.1 + grid.nz as f64 * grid.cell);
    let bounds = BBox::new(Vector3::new(grid.min.0, y_lo, grid.min.1), Vector3::new(max.0, y_hi, max.1));
    let invrd = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
    let (tmin, tmax) = match bounds.entry_exit(&r.ro, &invrd) {
        Some(x) => x,
        None => return,
    };
    let mut t = tmin.max(0.);
    let p = r.ro + r.rd * t;
    let cell_of = |v: f64, lo: f64, n: usize| (((v - lo) / grid.cell).floor().max(0.) as usize).min(n - 1);
    let (mut ix, mut iz) = (cell_of(p.x, grid.min.0, grid.nx), cell_of(p.z, grid.min.1, grid.nz));

    // Per axis: the t of the next cell boundary, the t between boundaries, and the step.
    let axis = |d: f64, o: f64, lo: f64, i: usize| -> (f64, f64, isize) {
        if d > 0. {
            ((lo + (i + 1) as f64 * grid.cell - o) / d, grid.cell / d, 1)
        } else if d < 0. {
            ((lo + i as f64 * grid.cell - o) / d, -grid.cell / d, -1)
        } else {
            (f64::INFINITY, f64::INFINITY, 0)
        }
    };
    let (mut next_x, dt_x, step_x) = axis(r.rd.x, r.ro.x, grid.min.0, ix);
    let (mut next_z, dt_z, step_z) = axis(r.rd.z, r.ro.z, grid.min.1, iz);

    loop {
        let exit = next_x.min(next_z).min(tmax);
        if visit(ix, iz, t, exit) || exit >= tmax {
            return;
        }
        if next_x < next_z {
            let n = ix as isize + step_x;
            if n < 0 || n >= grid.nx as isize { return }
            ix = n as usize;
            next_x += dt_x;
        } else {
            let n = iz as isize + step_z;
            if n < 0 || n >= grid.nz as isize { return }
            iz = n as usize;
            next_z += dt_z;
        }
        t = exit;
    }
}

/// Whether the ray, between t0 and t1, comes within the heights [lo, hi].
#[inline]
pub fn ray_spans(r: &Ray, t0: f64, t1: f64, lo: f64, hi: f64) -> bool {
    let (y0, y1) = (r.ro.y + r.rd.y * t0, r.ro.y + r.rd.y * t1);
    y0.min(y1) <= hi && y0.max(y1) >= lo
}

/// The rendered surface of a `HeightGrid`.
pub struct Terrain {
    pub grid: Arc<HeightGrid>,
}

impl Geometry for Terrain {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let g = &self.grid;
        let grid = Grid2 { min: g.min, cell: g.cell, nx: g.nx, nz: g.nz };
        let mut hit = None;
        // Slightly loose bounds: a ray along the flat top of the terrain is still inside.
        walk_grid(&grid, g.y_range.0 - 1e-6, g.y_range.1 + 1e-6, r, |ix, iz, t0, t1| {
            let (lo, hi) = g.ranges[iz * g.nx + ix];
            let eps = 1e-9 * (1. + t1.abs());
            if ray_spans(r, t0, t1, lo, hi) {
                hit = g.intersect_cell(r, ix, iz, t0 - eps, t1 + eps);
            }
            hit.is_some()
        });
        hit.map(|(t, normal)| RawIntersection { part: 0,
            face: 0,
            dist: t * r.rd.norm(),
            point: r.ro + r.rd * t,
            normal,
        })
    }

    fn bounds(&self) -> BBox {
        self.grid.bounds()
    }

    fn primitives(&self) -> u64 {
        (self.grid.nx * self.grid.nz * 2) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Slope;
    impl HeightSource for Slope {
        fn height(&self, x: f64, z: f64) -> f64 { 0.5 * x + 0.25 * z }
    }

    #[test]
    fn test_grid_reproduces_planar_source() {
        let g = HeightGrid::sample(&Slope, (-4., -4.), (4., 4.), 1.);
        for &(x, z) in &[(0.3, 0.2), (-3.9, 2.7), (1.5, -0.5), (3.99, 3.99)] {
            assert!((g.height(x, z) - Slope.height(x, z)).abs() < 1e-9, "{} {}", x, z);
        }
        let n = g.normal(0.3, 0.7);
        let expected = Vector3::new(-0.5, 1., -0.25).normalize();
        assert!((n - expected).norm() < 1e-9, "{}", n);
    }

    #[test]
    fn test_terrain_hit_lies_on_grid_surface() {
        let source = NoiseHeight {
            noise: PerlinNoise::new(), base: 0., amplitude: 2., scale: 5., octaves: 4,
            persistence: 0.5, lacunarity: 2., ridged: false, offset: (0.1, 0.2),
        };
        let grid = Arc::new(HeightGrid::sample(&source, (-10., -10.), (10., 10.), 0.25));
        let t = Terrain { grid: grid.clone() };
        for i in 0 .. 50 {
            let a = i as f64 * 0.37;
            let r = Ray { ro: Vector3::new(a.sin() * 3., 6., a.cos() * 3.), rd: Vector3::new(a.cos(), -1.5, 0.3 * a.sin()) };
            let hit = t.intersects(&r).expect("downward ray inside the terrain should hit");
            assert!((hit.point.y - grid.height(hit.point.x, hit.point.z)).abs() < 1e-6, "{}", hit.point);
            assert!((hit.dist - (hit.point - r.ro).norm()).abs() < 1e-9);
            assert!(hit.normal.y > 0.);
        }
    }

    #[test]
    fn test_grazing_ray_finds_nearest_hit() {
        // A ray skimming flat ground from outside must hit the first cell it reaches.
        let grid = Arc::new(HeightGrid::sample(&Slope, (0., 0.), (10., 10.), 0.5));
        let t = Terrain { grid };
        let r = Ray { ro: Vector3::new(-5., 10., 5.), rd: Vector3::new(1., -0.5, 0.) };
        let hit = t.intersects(&r).unwrap();
        // Along z = 5 the ground is 0.5x + 1.25; the ray is 10 - 0.5(x + 5).
        assert!((hit.point.x - 6.25).abs() < 1e-9, "{}", hit.point);
        let up = Ray { ro: Vector3::new(5., 20., 5.), rd: Vector3::new(0., 1., 0.) };
        assert!(t.intersects(&up).is_none());
    }

    #[test]
    fn test_image_heightmap_interpolates() {
        // 2 x 2: black, white on the first row (low z); white, white on the second.
        let path = std::env::temp_dir().join(format!("rays-heightmap-test-{}.png", std::process::id()));
        image::GrayImage::from_raw(2, 2, vec![0, 255, 255, 255]).unwrap().save(&path).unwrap();
        let h = ImageHeight::from_file(path.to_str().unwrap(), (0., 0.), (2., 4.), 1., 3.);
        std::fs::remove_file(&path).unwrap();
        assert!((h.height(0., 0.) - 1.).abs() < 1e-9);
        assert!((h.height(2., 0.) - 4.).abs() < 1e-9);
        assert!((h.height(1., 0.) - 2.5).abs() < 1e-9);
        assert!((h.height(0., 4.) - 4.).abs() < 1e-9);
        // Outside the image: the nearest edge.
        assert!((h.height(-5., -5.) - 1.).abs() < 1e-9);
    }

    #[test]
    fn test_walk_visits_cells_in_order() {
        let grid = Grid2 { min: (0., 0.), cell: 1., nx: 4, nz: 4 };
        let r = Ray { ro: Vector3::new(0.5, 0., 0.5), rd: Vector3::new(1., 0., 0.55) };
        let mut cells = vec![];
        let mut last = 0.;
        walk_grid(&grid, -1., 1., &r, |ix, iz, t0, t1| {
            assert!(t0 >= last - 1e-12 && t1 >= t0);
            last = t1;
            cells.push((ix, iz));
            false
        });
        assert_eq!(cells, vec![(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 2)]);
    }
}
