//! Media whose density is given on a voxel grid (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 4th ed., section 11.4.3, `GridMedium`): smoke, fire and clouds from simulations
//! or modelling tools, rather than from noise.
//!
//! The grid's samples sit at the centres of `nx × ny × nz` cells spanning the volume's box, and
//! the density between them is trilinearly interpolated (pbrt's `SampledGrid`); beyond the outer
//! samples it falls towards zero, reaching half the outer value at the box's faces, and is zero
//! outside the box.
//!
//! Delta tracking needs a bound on the density, and a single bound for the whole grid wastes
//! most of its tentative collisions where a plume is thin or the box empty. As in pbrt, a coarse
//! grid of local bounds (`MajorantGrid`, 16³ cells) gives each stretch of a ray its own
//! majorant, walked cell by cell with a 3D DDA (Amanatides and Woo, "A Fast Voxel Traversal
//! Algorithm for Ray Tracing", Eurographics 1987): empty cells are skipped without sampling,
//! and thin ones are crossed in long steps (`DensityField::majorant_from`).
//!
//! Grids come inline (`Grid::new`) or from Mitsuba's `.vol` files (`Grid::load`), the simplest
//! format that simulation tools and pbrt-style scenes export to (Wenzel Jakob, "Mitsuba
//! Renderer documentation", version 0.5, section 8.7.2, "Grid-based volume data source").

use crate::na::Vector3;
use crate::shapes::bbox::BBox;
use crate::volume::DensityField;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Values on a regular lattice over the unit cube, x fastest, then y, then z.
#[derive(Debug, Clone)]
pub struct Grid {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub values: Vec<f32>,
    /// Where a file placed the grid in its own coordinates, if it did.
    pub bounds: Option<BBox>,
}

impl Grid {
    pub fn new(nx: usize, ny: usize, nz: usize, values: Vec<f32>) -> Grid {
        assert!(nx > 0 && ny > 0 && nz > 0, "A grid needs at least one sample along each axis");
        assert_eq!(values.len(), nx * ny * nz, "A {}×{}×{} grid needs {} values, got {}", nx, ny, nz, nx * ny * nz, values.len());
        assert!(values.iter().all(|v| v.is_finite() && *v >= 0.), "Grid values must be finite and non-negative");
        Grid { nx, ny, nz, values, bounds: None }
    }

    fn at(&self, x: isize, y: isize, z: isize) -> f64 {
        if x < 0 || y < 0 || z < 0 || x >= self.nx as isize || y >= self.ny as isize || z >= self.nz as isize {
            return 0.;
        }
        self.values[(z as usize * self.ny + y as usize) * self.nx + x as usize] as f64
    }

    /// The trilinearly interpolated value at `u` in the unit cube; zero outside it.
    pub fn lookup(&self, u: &Vector3<f64>) -> f64 {
        if u.iter().any(|v| !(0. ..=1.).contains(v)) {
            return 0.;
        }
        // Sample coordinates: sample i sits at (i + ½) / n.
        let s = Vector3::new(u.x * self.nx as f64 - 0.5, u.y * self.ny as f64 - 0.5, u.z * self.nz as f64 - 0.5);
        let i = s.map(|v| v.floor());
        let f = s - i;
        let (x, y, z) = (i.x as isize, i.y as isize, i.z as isize);
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        let c00 = lerp(self.at(x, y, z), self.at(x + 1, y, z), f.x);
        let c10 = lerp(self.at(x, y + 1, z), self.at(x + 1, y + 1, z), f.x);
        let c01 = lerp(self.at(x, y, z + 1), self.at(x + 1, y, z + 1), f.x);
        let c11 = lerp(self.at(x, y + 1, z + 1), self.at(x + 1, y + 1, z + 1), f.x);
        lerp(lerp(c00, c10, f.y), lerp(c01, c11, f.y), f.z)
    }

    /// The largest value interpolation can give anywhere in the box from `lo` to `hi` (in the
    /// unit cube): the largest of the samples whose influence reaches it (pbrt's
    /// `SampledGrid::MaxValue`).
    pub fn max_value(&self, lo: &Vector3<f64>, hi: &Vector3<f64>) -> f64 {
        let range = |l: f64, h: f64, n: usize| {
            let a = ((l * n as f64 - 0.5).floor() as isize).max(0);
            let b = ((h * n as f64 - 0.5).floor() as isize + 1).min(n as isize - 1);
            (a, b)
        };
        let (x0, x1) = range(lo.x, hi.x, self.nx);
        let (y0, y1) = range(lo.y, hi.y, self.ny);
        let (z0, z1) = range(lo.z, hi.z, self.nz);
        let mut m: f64 = 0.;
        for z in z0..=z1 {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    m = m.max(self.at(x, y, z));
                }
            }
        }
        m
    }

    pub fn max(&self) -> f64 {
        self.values.iter().fold(0f32, |a, &b| a.max(b)) as f64
    }

    /// Parse a Mitsuba `.vol` file: "VOL", version 3, then little-endian: encoding (i32: 1 for
    /// float32, 3 for uint8, read as /255), resolution (3 × i32), channels (i32; several are
    /// averaged), bounds (6 × f32: min, then max), and the values, x fastest.
    pub fn parse_vol(bytes: &[u8]) -> Result<Grid, String> {
        if bytes.len() < 48 || &bytes[0..3] != b"VOL" {
            return Err("not a .vol file (no VOL header)".into());
        }
        if bytes[3] != 3 {
            return Err(format!("unsupported .vol version {}", bytes[3]));
        }
        let i32_at = |o: usize| i32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let f32_at = |o: usize| f32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let encoding = i32_at(4);
        let (nx, ny, nz, channels) = (i32_at(8), i32_at(12), i32_at(16), i32_at(20));
        if nx <= 0 || ny <= 0 || nz <= 0 || channels <= 0 {
            return Err(format!("bad .vol resolution {}×{}×{}×{}", nx, ny, nz, channels));
        }
        let (nx, ny, nz, channels) = (nx as usize, ny as usize, nz as usize, channels as usize);
        let lo = Vector3::new(f32_at(24), f32_at(28), f32_at(32)).cast::<f64>();
        let hi = Vector3::new(f32_at(36), f32_at(40), f32_at(44)).cast::<f64>();
        let count = nx * ny * nz * channels;
        let size = match encoding { 1 => 4, 3 => 1, e => return Err(format!("unsupported .vol encoding {} (only 1, float32, and 3, uint8)", e)) };
        let data = &bytes[48..];
        if data.len() < count * size {
            return Err(format!("truncated .vol file: {} bytes of values, expected {}", data.len(), count * size));
        }
        let value = |i: usize| -> f32 {
            if size == 4 { f32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap()) } else { data[i] as f32 / 255. }
        };
        let values: Vec<f32> = (0..nx * ny * nz).map(|v| {
            (0..channels).map(|c| value(v * channels + c)).sum::<f32>() / channels as f32
        }).map(|v| if v.is_finite() { v.max(0.) } else { 0. }).collect();
        let mut g = Grid::new(nx, ny, nz, values);
        if hi.iter().zip(lo.iter()).all(|(h, l)| h > l) {
            g.bounds = Some(BBox::new(lo, hi));
        }
        Ok(g)
    }

    /// Write a single-channel float32 `.vol` file's bytes over `bounds`.
    pub fn to_vol(&self, bounds: &BBox) -> Vec<u8> {
        let mut out = b"VOL\x03".to_vec();
        for v in [1, self.nx as i32, self.ny as i32, self.nz as i32, 1] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in bounds.min.iter().chain(bounds.max.iter()) {
            out.extend_from_slice(&(*v as f32).to_le_bytes());
        }
        for v in &self.values {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    /// A `.vol` file, read once however many objects use it.
    pub fn load(path: &str) -> Arc<Grid> {
        type Cache = Mutex<HashMap<String, Arc<Grid>>>;
        static CACHE: OnceLock<Cache> = OnceLock::new();
        let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        if let Some(g) = cache.lock().unwrap().get(path) {
            return g.clone();
        }
        let bytes = crate::assets::read(path).unwrap_or_else(|e| panic!("Couldn't read volume grid: {}", e));
        let g = Arc::new(Grid::parse_vol(&bytes).unwrap_or_else(|e| panic!("Couldn't read volume grid {}: {}", path, e)));
        cache.lock().unwrap().insert(path.to_string(), g.clone());
        g
    }
}

/// Cells along each axis of the majorant grid, as pbrt's.
pub const MAJORANT_RESOLUTION: usize = 16;

/// Bounds on a grid's values over coarse cells, for tracking.
#[derive(Debug, Clone)]
pub struct MajorantGrid {
    pub n: usize,
    pub values: Vec<f64>,
}

impl MajorantGrid {
    pub fn new(grid: &Grid, n: usize) -> MajorantGrid {
        let mut values = Vec::with_capacity(n * n * n);
        for z in 0..n {
            for y in 0..n {
                for x in 0..n {
                    let lo = Vector3::new(x as f64, y as f64, z as f64) / n as f64;
                    let hi = Vector3::new(x as f64 + 1., y as f64 + 1., z as f64 + 1.) / n as f64;
                    values.push(grid.max_value(&lo, &hi));
                }
            }
        }
        MajorantGrid { n, values }
    }

    /// The bound in the cell holding `u` (in the unit cube), and the distance along `d` (a
    /// direction in unit-cube coordinates, any length: the distance is in multiples of it) to
    /// where the ray leaves that cell.
    pub fn from(&self, u: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let n = self.n as f64;
        let mut cell = [0usize; 3];
        let mut exit = f64::INFINITY;
        for a in 0..3 {
            let s = u[a] * n;
            // On a boundary, the cell the ray is heading into.
            let mut c = s.floor();
            if s == c && d[a] < 0. {
                c -= 1.;
            }
            let c = c.clamp(0., n - 1.);
            cell[a] = c as usize;
            if d[a] != 0. {
                let edge = if d[a] > 0. { c + 1. } else { c };
                exit = exit.min(((edge - s) / (d[a] * n)).max(0.));
            }
        }
        (self.values[(cell[2] * self.n + cell[1]) * self.n + cell[0]], exit)
    }
}

/// A density field from a grid over a box, times `scale`.
pub struct GridDensity {
    pub bounds: BBox,
    pub grid: Arc<Grid>,
    pub scale: f64,
    majorants: MajorantGrid,
    max: f64,
}

impl GridDensity {
    pub fn new(bounds: BBox, grid: Arc<Grid>, scale: f64) -> GridDensity {
        assert!(scale >= 0., "A grid's density scale must be non-negative, got {}", scale);
        let majorants = MajorantGrid::new(&grid, MAJORANT_RESOLUTION);
        let max = grid.max();
        GridDensity { bounds, grid, scale, majorants, max }
    }

    fn unit(&self, p: &Vector3<f64>) -> Vector3<f64> {
        (p - self.bounds.min.coords).component_div(&(self.bounds.max - self.bounds.min))
    }
}

impl DensityField for GridDensity {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.scale * self.grid.lookup(&self.unit(p))
    }

    fn majorant(&self) -> f64 {
        self.scale * self.max
    }

    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let u = self.unit(p);
        if u.iter().any(|v| !(0. ..=1.).contains(v)) {
            return (self.majorant(), f64::INFINITY);
        }
        // A world step along d moves d / size in the unit cube.
        let du = d.component_div(&(self.bounds.max - self.bounds.min));
        let (m, length) = self.majorants.from(&u, &du);
        (self.scale * m, length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{rand, seed_rand};

    fn random_grid(n: usize, seed: u64) -> Grid {
        seed_rand(seed, 0);
        // Sparse: most cells empty, as a plume's are.
        Grid::new(n, n + 1, n + 2, (0..n * (n + 1) * (n + 2)).map(|_| if rand() < 0.2 { rand() as f32 * 3. } else { 0. }).collect())
    }

    #[test]
    fn test_lookup_interpolates_cell_centres() {
        let g = Grid::new(2, 1, 1, vec![1., 3.]);
        // Sample centres at x = ¼ and ¾.
        assert!((g.lookup(&Vector3::new(0.25, 0.5, 0.5)) - 1.).abs() < 1e-12);
        assert!((g.lookup(&Vector3::new(0.75, 0.5, 0.5)) - 3.).abs() < 1e-12);
        assert!((g.lookup(&Vector3::new(0.5, 0.5, 0.5)) - 2.).abs() < 1e-12);
        // Falling to zero half a cell beyond: half way at the box's face.
        assert!((g.lookup(&Vector3::new(0., 0.5, 0.5)) - 0.5).abs() < 1e-12);
        assert_eq!(g.lookup(&Vector3::new(-0.01, 0.5, 0.5)), 0.);
        assert_eq!(g.lookup(&Vector3::new(2., 0.5, 0.5)), 0.);
    }

    #[test]
    fn test_majorants_bound_the_density() {
        let g = random_grid(7, 1);
        let m = MajorantGrid::new(&g, 4);
        seed_rand(2, 0);
        for _ in 0..20_000 {
            let u = Vector3::new(rand(), rand(), rand());
            let d = Vector3::new(rand() - 0.5, rand() - 0.5, rand() - 0.5);
            let (bound, length) = m.from(&u, &d);
            assert!(length >= 0.);
            // Everywhere the ray goes within the cell, the density stays under the bound.
            for k in 0..=8 {
                let p = u + d * (length * k as f64 / 8.);
                assert!(g.lookup(&p) <= bound + 1e-9, "{} > {} at {:?}", g.lookup(&p), bound, p);
            }
        }
    }

    #[test]
    fn test_dda_steps_reach_the_next_cell() {
        let g = random_grid(5, 3);
        let m = MajorantGrid::new(&g, 4);
        let d = Vector3::new(0.3, -0.2, 0.7);
        let mut u = Vector3::new(0.1, 0.9, 0.05);
        let mut steps = 0;
        while u.iter().all(|v| (0. ..=1.).contains(v)) {
            let (_, length) = m.from(&u, &d);
            u += d * (length + 1e-12);
            steps += 1;
            assert!(steps < 20, "stuck at {:?}", u);
        }
        // Crossing 4³ cells corner to corner touches at most 3 × 4 cells.
        assert!(steps >= 4 && steps <= 12, "{}", steps);
    }

    #[test]
    fn test_vol_round_trip() {
        let g = random_grid(3, 4);
        let b = BBox::new(Vector3::new(-1., 0., 2.), Vector3::new(1., 3., 4.));
        let read = Grid::parse_vol(&g.to_vol(&b)).unwrap();
        assert_eq!(read.values, g.values);
        assert_eq!((read.nx, read.ny, read.nz), (g.nx, g.ny, g.nz));
        let rb = read.bounds.unwrap();
        assert_eq!((rb.min, rb.max), (b.min, b.max));
        assert!(Grid::parse_vol(b"VOX\x03").is_err());
    }

    #[test]
    fn test_uint8_vol() {
        let mut bytes = b"VOL\x03".to_vec();
        for v in [3i32, 2, 1, 1, 1] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        for v in [0f32, 0., 0., 1., 1., 1.] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        bytes.extend_from_slice(&[0, 255]);
        let g = Grid::parse_vol(&bytes).unwrap();
        assert_eq!(g.values, vec![0., 1.]);
    }

    #[test]
    fn test_grid_volume_follows_optical_depth() {
        use crate::ray::Ray;
        use crate::shapes::geometry::Geometry;
        use crate::volume::Volume;
        let g = Arc::new(random_grid(6, 5));
        let b = BBox::new(Vector3::new(0., 0., 0.), Vector3::new(2., 1., 1.));
        let field = GridDensity::new(b, g, 1.5);
        let (from, to) = (Vector3::new(-0.5, 0.31, 0.62), Vector3::new(2.5, 0.73, 0.4));
        let steps = 40_000;
        let optical_depth: f64 = (0..steps).map(|i| field.density(&(from + (to - from) * ((i as f64 + 0.5) / steps as f64))))
            .sum::<f64>() * (to - from).norm() / steps as f64;
        assert!(optical_depth > 0.2 && optical_depth < 3., "uninformative test: {}", optical_depth);
        let v = Volume::new(b, Arc::new(field));
        let r = Ray { ro: from, rd: to - from };
        seed_rand(6, 0);
        let n = 40_000;
        let through = (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64;
        assert!((through - (-optical_depth).exp()).abs() < 0.01, "{} vs {}", through, (-optical_depth).exp());
        let ratio = (0..n).map(|_| v.transmittance(&r, 0., f64::INFINITY)).sum::<f64>() / n as f64;
        assert!((ratio - (-optical_depth).exp()).abs() < 0.005, "{} vs {}", ratio, (-optical_depth).exp());
    }
}
