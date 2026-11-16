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
//! - `mesas`: flat-topped mesas and buttes standing on a desert floor (`MesaHeight`).
//!   `height` (of the tops above `base`, default 120), `scale` (size of the mesas, default
//!   800), `coverage` (fraction of the area they cover, default 0.15), `cliff` (width of the
//!   cliff band, metres, default 30), `benches` (ledges in the cliff, default 2), `talus`
//!   (height of the scree apron, as a fraction of `height`, default 0.35), `talus_width`
//!   (metres, default `height`), `floor_amplitude` and `floor_scale` (the floor's undulation,
//!   default 2 and 150), `clearing` ([x, z, radius]: keep the mesas out of a circle, eg. round
//!   the camera), `seed`; and for more weathered country, `towers` (free-standing spires and
//!   fins per square kilometre, each on its own debris cone), `relief` (rounded hills left on
//!   the tops, as a fraction of `height`), `height_variation` (from mesa to mesa, a
//!   fraction), `roughness` (gullies and fallen blocks on the debris slopes, 0 to 1) and
//!   `floor_ledges` (the height of the low steps the floor is terraced in, metres)
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

/// Mesas and buttes: flat-topped remnants of a plateau whose hard caprock protects the softer
/// beds below, standing out of a desert floor, as in Monument Valley or on the Colorado
/// Plateau. Their outlines come from fractal noise, cut at a level that leaves `coverage` of
/// the area standing, warped and notched so the edges are embayed like eroded scarps; small
/// separate patches become buttes, and the smallest, never wide enough to reach full height,
/// spires.
///
/// The profile across an edge follows the classic scarp: the caprock a vertical cliff, the
/// softer beds below it stepping down in `benches` ledges, and a concave apron of talus (scree
/// fallen from the cliff, at about its angle of repose) running out onto the floor (Alan D.
/// Howard and Richard C. Kochel, "Introduction to cuesta landforms and sapping processes on
/// the Colorado Plateau", in "Sapping Features of the Colorado Plateau", NASA SP-491, 1988).
/// So that every cliff is about as wide, the profile is a function of the distance from the
/// edge: the outlines are cut from broad noise on a grid, their exact signed distance found by
/// Pedro Felzenszwalb and Daniel Huttenlocher's linear-time Euclidean distance transform
/// ("Distance Transforms of Sampled Functions", Theory of Computing 8, 2012,
/// doi:10.4086/toc.2012.v008a019), interpolated, and roughened by finer noise in metres, which
/// embays and notches the scarps.
pub struct MesaHeight {
    pub noise: PerlinNoise,
    pub base: f64,
    pub height: f64,
    pub scale: f64,
    pub cliff: f64,
    pub benches: u32,
    pub talus: f64,
    pub talus_width: f64,
    pub floor_amplitude: f64,
    pub floor_scale: f64,
    pub clearing: Option<(f64, f64, f64)>,
    pub offset: (f64, f64),
    /// Height of the rounded remnants of softer beds left on the tops, as a fraction of
    /// `height` (0 for flat tops); and how much the tops' heights vary from mesa to mesa.
    pub relief: f64,
    pub height_variation: f64,
    /// Depth of the gullies and size of the fallen blocks on the debris slopes, 0 to 1.
    pub roughness: f64,
    /// Height of the low ledges the floor steps down in, metres (0 for a smoothly rolling
    /// floor), as thin hard beds in the soft rock beneath it outcrop.
    pub floor_ledges: f64,
    /// Free-standing towers and fins, and a grid of them by cell for lookup.
    towers: Vec<Tower>,
    tower_cell: f64,
    tower_grid: std::collections::HashMap<(i64, i64), Vec<usize>>,
    /// Signed distance from the mesas' outlines, metres, positive inside, on a grid.
    sdf: Vec<f32>,
    sdf_min: (f64, f64),
    sdf_cell: f64,
    sdf_n: (usize, usize),
}

impl MesaHeight {
    /// Mesas over [min, max] (and as at its edges beyond), covering `coverage` of it. See the
    /// module documentation for the other parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(min: (f64, f64), max: (f64, f64), coverage: f64, base: f64, height: f64, scale: f64,
               cliff: f64, benches: u32, talus: f64, talus_width: f64, floor_amplitude: f64,
               floor_scale: f64, clearing: Option<(f64, f64, f64)>, seed: f64) -> MesaHeight {
        let mut m = MesaHeight {
            noise: PerlinNoise::new(), base, height, scale, cliff, benches, talus, talus_width,
            floor_amplitude, floor_scale, clearing, offset: (seed * 31.17 + 0.37, seed * 17.73 + 0.71),
            relief: 0., height_variation: 0., roughness: 0., floor_ledges: 0.,
            towers: vec![], tower_cell: 1., tower_grid: Default::default(),
            sdf: vec![], sdf_min: min, sdf_cell: 1., sdf_n: (1, 1),
        };
        // Fine enough for the cliffs, at most about 1500 cells a side.
        let extent = (max.0 - min.0).max(max.1 - min.1);
        let cell = (0.5 * cliff).max(extent / 1500.).max(1e-6);
        let (nx, nz) = (((max.0 - min.0) / cell).ceil() as usize + 1, ((max.1 - min.1) / cell).ceil() as usize + 1);
        let field: Vec<f64> = {
            use rayon::prelude::*;
            let m = &m;
            (0 .. nx * nz).into_par_iter().map(|k| m.field(min.0 + (k % nx) as f64 * cell, min.1 + (k / nx) as f64 * cell)).collect()
        };
        let mut sorted = field.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let threshold = sorted[(((1. - coverage.clamp(0., 1.)) * sorted.len() as f64) as usize).min(sorted.len() - 1)];
        let inside: Vec<bool> = field.iter().map(|f| *f > threshold).collect();
        let to_outside = distance_transform(&inside, nx, nz, true);
        let to_inside = distance_transform(&inside, nx, nz, false);
        // Half a cell either side of the edge, so it falls between the cells.
        m.sdf = (0 .. nx * nz).map(|k| {
            if inside[k] { (to_outside[k].sqrt() - 0.5) * cell } else { -(to_inside[k].sqrt() - 0.5) * cell }
        } as f32).collect();
        m.sdf_cell = cell;
        m.sdf_n = (nx, nz);
        m
    }

    /// The broad field whose level sets are the mesas' outlines, before the cut: warped noise,
    /// pushed down round the clearing.
    fn field(&self, x: f64, z: f64) -> f64 {
        let (nx, nz) = (x / self.scale + self.offset.0, z / self.scale + self.offset.1);
        let warp = |o: f64| self.noise.fbm(nx * 0.7 + o, 1.7, nz * 0.7 - o, 3, 0.5, 2.) - 0.5;
        let (wx, wz) = (nx + 0.6 * warp(13.1), nz + 0.6 * warp(47.9));
        let mut f = self.noise.fbm(wx, 0.5, wz, 4, 0.5, 2.);
        if let Some((cx, cz, r)) = self.clearing {
            let d = ((x - cx).powi(2) + (z - cz).powi(2)).sqrt();
            f -= 0.6 * (1. - smoothstep(r, 1.6 * r, d));
        }
        f
    }

    /// Signed distance (metres, positive inside) from the outlines, interpolated, notched.
    pub fn distance(&self, x: f64, z: f64) -> f64 {
        let d = self.base_distance(x, z);
        // Embayments a few cliff-widths across, and smaller notches.
        let n = |s: f64, o: f64| 2. * self.noise.fbm(x / s + o, 3.3, z / s - o, 3, 0.5, 2.) - 1.;
        d + 2.5 * self.cliff * n(6. * self.cliff, 17.7) + 0.6 * self.cliff * n(1.5 * self.cliff, 41.1)
    }

    /// The height above the floor at signed distance `d` (metres, positive inside) from a
    /// mesa's edge.
    pub fn profile(&self, d: f64) -> f64 {
        self.shape(d, self.height, self.talus, self.talus_width, self.cliff).0
    }

    /// A scarp's profile: the height at signed distance `d` from the foot of the cliff, for a
    /// top `h` high whose debris slope reaches `talus` of the way up and runs out over about
    /// `width`, with a cliff band `cliff` wide; and how much of a debris slope the point is on
    /// (0 on the floor and the cliff, most halfway up the slope), for its gullies and blocks.
    fn shape(&self, d: f64, h: f64, talus: f64, width: f64, cliff: f64) -> (f64, f64) {
        let apron = h * talus;
        if d <= 0. {
            // A concave slope, steepest (at about scree's angle of repose, 35 degrees, for the
            // default width) where it meets the cliff, stepped by the ledges of harder beds in
            // the soft rock it is cut into.
            let u = (d / width.max(1e-9)).exp();
            let n = 5.;
            let ledges = ((u * n).floor() + smoothstep(0., 0.45, (u * n).fract())) / n;
            let u = u + 0.45 * (ledges - u) * smoothstep(0.1, 0.3, u);
            return (apron * u, (std::f64::consts::PI * u).sin().max(0.));
        }
        // The cliff in steps: each a near-vertical riser and a ledge, the last riser the caprock.
        let steps = (self.benches + 1) as f64;
        let s = (d / cliff.max(1e-9)).min(1.) * steps;
        let riser = |x: f64| smoothstep(0., 0.4, x);
        let f = if s >= steps { 1. } else { (s.floor() + riser(s.fract())) / steps };
        (apron + (h - apron) * f, 0.)
    }

    /// Scatter `density` towers and fins per square kilometre over [min, max]: most standing
    /// off the mesas' edges, as the last remnants of a retreating scarp do, some far out on the
    /// floor; none in the clearing.
    pub fn with_towers(mut self, density: f64, min: (f64, f64), max: (f64, f64), seed: u64) -> MesaHeight {
        let area = (max.0 - min.0) * (max.1 - min.1) / 1e6;
        let want = (density.max(0.) * area).round() as usize;
        let mut rng = crate::geometry::Pcg32::new(seed, 0x70e2);
        let mut towers = vec![];
        let mut tries = 0;
        while towers.len() < want && tries < want * 200 {
            tries += 1;
            let (x, z) = (min.0 + rng.next_f64() * (max.0 - min.0), min.1 + rng.next_f64() * (max.1 - min.1));
            let d = self.base_distance(x, z);
            let near = (-6. * self.cliff ..= -1.5 * self.cliff).contains(&d);
            let far = d < -8. * self.cliff && rng.next_f64() < 0.06;
            if !(near || far) {
                continue;
            }
            if let Some((cx, cz, r)) = self.clearing && ((x - cx).powi(2) + (z - cz).powi(2)).sqrt() < 1.2 * r {
                continue;
            }
            // Radii mostly small (spires), a few broad (buttes); fins drawn out along one axis.
            let radius = 0.25 * self.cliff * (1. + 4. * rng.next_f64().powi(3));
            let aspect = if rng.next_f64() < 0.35 { 1.5 + 2. * rng.next_f64() } else { 1. + 0.3 * rng.next_f64() };
            towers.push(Tower {
                x, z, radius, aspect, angle: rng.next_f64() * std::f64::consts::PI,
                height: self.height * (0.55 + 0.45 * rng.next_f64().sqrt()),
                talus: self.talus * (0.7 + 0.6 * rng.next_f64()),
            });
        }
        let reach = towers.iter().map(|t| t.reach(self.talus_width)).fold(1., f64::max);
        self.tower_cell = reach;
        for (k, t) in towers.iter().enumerate() {
            let key = ((t.x / reach).floor() as i64, (t.z / reach).floor() as i64);
            self.tower_grid.entry(key).or_default().push(k);
        }
        self.towers = towers;
        self
    }

    /// Tower `t`'s height at (x, z), and how much debris slope it is.
    fn tower_height(&self, t: &Tower, x: f64, z: f64) -> (f64, f64) {
        let (dx, dz) = (x - t.x, z - t.z);
        let (c, s) = (t.angle.cos(), t.angle.sin());
        let (a, b) = ((dx * c + dz * s) / t.aspect, -dx * s + dz * c);
        // Fluted: the column's outline wavers with the angle round it.
        let flute = 1. + 0.18 * self.noise.noise(b.atan2(a) * 1.7 + t.x * 0.01, 5.5, t.z * 0.01);
        let d = t.radius * flute - (a * a + b * b).sqrt();
        let width = t.height * t.talus / 35f64.to_radians().tan();
        self.shape(d, t.height, t.talus, width, 0.3 * t.radius)
    }

    /// The signed distance before the notches are added.
    fn base_distance(&self, x: f64, z: f64) -> f64 {
        let (nx, nz) = self.sdf_n;
        let u = ((x - self.sdf_min.0) / self.sdf_cell).clamp(0., (nx - 1) as f64);
        let v = ((z - self.sdf_min.1) / self.sdf_cell).clamp(0., (nz - 1) as f64);
        let (i, j) = ((u as usize).min(nx.saturating_sub(2)), (v as usize).min(nz.saturating_sub(2)));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let at = |a: usize, b: usize| self.sdf[(b.min(nz - 1)) * nx + a.min(nx - 1)] as f64;
        (at(i, j) * (1. - fu) + at(i + 1, j) * fu) * (1. - fv) + (at(i, j + 1) * (1. - fu) + at(i + 1, j + 1) * fu) * fv
    }

    /// The tallest tower's height at (x, z), and how much debris slope it is.
    fn towers_at(&self, x: f64, z: f64) -> (f64, f64) {
        let (ci, cj) = ((x / self.tower_cell).floor() as i64, (z / self.tower_cell).floor() as i64);
        let mut best = (0., 0.);
        for i in ci - 1 ..= ci + 1 {
            for j in cj - 1 ..= cj + 1 {
                for &k in self.tower_grid.get(&(i, j)).map_or(&[][..], |v| &v[..]) {
                    let (h, w) = self.tower_height(&self.towers[k], x, z);
                    if h > best.0 {
                        best = (h, w);
                    }
                }
            }
        }
        best
    }
}

/// A free-standing tower or fin of the scarp's rock, on its own cone of debris.
struct Tower {
    x: f64,
    z: f64,
    radius: f64,
    /// Length over width, for fins; and the direction of the length.
    aspect: f64,
    angle: f64,
    height: f64,
    talus: f64,
}

impl Tower {
    /// How far from its centre its debris cone reaches (where it is under a centimetre or so).
    fn reach(&self, _talus_width: f64) -> f64 {
        let width = self.height * self.talus / 35f64.to_radians().tan();
        self.radius * self.aspect + width * (self.height * self.talus / 0.01).ln()
    }
}

/// Squared Euclidean distance from each cell of an `nx` by `nz` grid to the nearest cell whose
/// `mask` differs from `of` (Felzenszwalb and Huttenlocher's transform, by columns then rows).
fn distance_transform(mask: &[bool], nx: usize, nz: usize, of: bool) -> Vec<f64> {
    const FAR: f64 = 1e20;
    let mut d: Vec<f64> = mask.iter().map(|&m| if m == of { FAR } else { 0. }).collect();
    let mut column = vec![0.; nz];
    for i in 0 .. nx {
        for j in 0 .. nz { column[j] = d[j * nx + i]; }
        let out = transform_1d(&column);
        for j in 0 .. nz { d[j * nx + i] = out[j]; }
    }
    for j in 0 .. nz {
        let out = transform_1d(&d[j * nx .. (j + 1) * nx]);
        d[j * nx .. (j + 1) * nx].copy_from_slice(&out);
    }
    d
}

/// The lower envelope of the parabolas (q - p)^2 + f(p): the 1D squared distance transform.
fn transform_1d(f: &[f64]) -> Vec<f64> {
    let n = f.len();
    let mut out = vec![0.; n];
    let mut v = vec![0usize; n];
    let mut z = vec![0.; n + 1];
    let mut k = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    let meet = |q: usize, p: usize| ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2. * q as f64 - 2. * p as f64);
    for q in 1 .. n {
        let mut s = meet(q, v[k]);
        while s <= z[k] {
            k -= 1;
            s = meet(q, v[k]);
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, o) in out.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let dq = q as f64 - v[k] as f64;
        *o = dq * dq + f[v[k]];
    }
    out
}

impl HeightSource for MesaHeight {
    fn height(&self, x: f64, z: f64) -> f64 {
        let d = self.distance(x, z);
        let n = |s: f64, o: f64, octaves: u32| 2. * self.noise.fbm(x / s + o, 0.37 * o, z / s - o, octaves, 0.5, 2.) - 1.;
        let mut floor = self.floor_amplitude * n(self.floor_scale, 71.3, 4);
        if self.floor_ledges > 0. {
            // Terraced: each step a short, steep riser and a gently rolling tread.
            let step = self.floor_ledges;
            let f = floor / step + 0.15 * n(0.2 * self.floor_scale, 3.7, 3);
            floor = step * (f.floor() + smoothstep(0., 0.3, f.fract()));
        }
        // Each mesa's top at its own height, as erosion has stripped more from some.
        let h = self.height * (1. + self.height_variation * n(3. * self.scale, 23.9, 2));
        let width = self.talus_width * h / self.height;
        let (mut profile, mut slope) = self.shape(d, h, self.talus, width, self.cliff);
        // The tops: low rounded hills of the softer beds above the caprock, set back from the
        // rim, on the rubbly caprock.
        let inside = smoothstep(self.cliff, 3. * self.cliff, d);
        let hills = (n(6. * self.cliff, 5.1, 4) + 0.15).max(0.).powf(1.5) * 1.6;
        profile += inside * (self.relief * self.height * hills + 0.01 * self.height * n(0.8 * self.cliff, 8.8, 3));
        let (tower, tower_slope) = self.towers_at(x, z);
        if tower > profile {
            (profile, slope) = (tower, tower_slope);
        }
        // Gullies down the debris slopes (along the zero lines of noise, which wander as a
        // drainage net does), and blocks fallen from the cliffs.
        if self.roughness > 0. && slope > 0. {
            let g = n(1.2 * self.cliff, 61.7, 3).abs();
            let gully = 1. - smoothstep(0., 0.18, g);
            let blocks = n(0.12 * self.cliff, 13.3, 2).max(0.);
            profile += slope * self.roughness * self.talus * self.height * (0.02 * blocks - 0.06 * gully);
        }
        self.base + floor + profile
    }
}

fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0., 1.);
    t * t * (3. - 2. * t)
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
        // Row by row, in parallel: a fine grid can have tens of millions of points.
        use rayon::prelude::*;
        let heights: Vec<f64> = (0 ..= nz).into_par_iter().flat_map_iter(|iz| {
            (0 ..= nx).map(move |ix| source.height(min.0 + ix as f64 * cell, min.1 + iz as f64 * cell))
        }).collect();
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
            "mesas" => {
                let height = SceneFile::parse_number(&o["height"], 120.);
                let clearing = o.get("clearing").map(|c| (c[0].as_f64().unwrap(), c[1].as_f64().unwrap(), c[2].as_f64().unwrap()));
                let m = MesaHeight::new(min, max, SceneFile::parse_number(&o["coverage"], 0.15), base, height,
                    SceneFile::parse_number(&o["scale"], 800.),
                    SceneFile::parse_number(&o["cliff"], 30.),
                    SceneFile::parse_int(&o["benches"], 2) as u32,
                    SceneFile::parse_number(&o["talus"], 0.35).clamp(0., 1.),
                    SceneFile::parse_number(&o["talus_width"], height),
                    SceneFile::parse_number(&o["floor_amplitude"], 2.),
                    SceneFile::parse_number(&o["floor_scale"], 150.),
                    clearing, seed);
                let mut m = m.with_towers(SceneFile::parse_number(&o["towers"], 0.), min, max, seed as u64);
                m.relief = SceneFile::parse_number(&o["relief"], 0.);
                m.height_variation = SceneFile::parse_number(&o["height_variation"], 0.);
                m.roughness = SceneFile::parse_number(&o["roughness"], 0.);
                m.floor_ledges = SceneFile::parse_number(&o["floor_ledges"], 0.);
                Box::new(m)
            },
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

    fn mesas(coverage: f64) -> MesaHeight {
        MesaHeight::new((-3000., -3000.), (3000., 3000.), coverage, 0., 100., 500., 20., 2, 0.3, 80., 0., 100.,
            Some((0., 0., 300.)), 1.)
    }

    #[test]
    fn test_distance_transform_is_euclidean() {
        // One outside cell in the middle of a 7 x 5 grid.
        let (nx, nz) = (7, 5);
        let mask: Vec<bool> = (0 .. nx * nz).map(|k| k != 2 * nx + 3).collect();
        let d = distance_transform(&mask, nx, nz, true);
        for k in 0 .. nx * nz {
            let (i, j) = ((k % nx) as f64 - 3., (k / nx) as f64 - 2.);
            assert_eq!(d[k], i * i + j * j);
        }
    }

    #[test]
    fn test_mesa_profile_steps_up_to_a_flat_top() {
        let m = mesas(0.2);
        // Far out on the floor, flat; the apron rising to the cliff's foot; the top flat.
        assert!(m.profile(-2000.) < 1e-6);
        assert!((m.profile(0.) - 30.).abs() < 1e-9);
        assert!(m.profile(-40.) < m.profile(-10.));
        assert!((m.profile(20.) - 100.).abs() < 1e-9 && (m.profile(500.) - 100.).abs() < 1e-9);
        // Never falling inwards, and with ledges: some of the cliff band is nearly level.
        let mut level = 0;
        for k in 0 .. 200 {
            let (a, b) = (m.profile(k as f64 * 0.1), m.profile((k + 1) as f64 * 0.1));
            assert!(b >= a - 1e-12);
            if b - a < 0.1 { level += 1; }
        }
        assert!(level > 60, "{}", level);
    }

    #[test]
    fn test_towers_stand_off_the_mesas_on_debris_cones() {
        let m = mesas(0.2).with_towers(2., (-3000., -3000.), (3000., 3000.), 4);
        assert!(m.towers.len() > 30, "{}", m.towers.len());
        let mut tall = 0;
        for t in &m.towers {
            // Most stand off a mesa's edge; none in the clearing.
            assert!(m.base_distance(t.x, t.z) < -1.4 * m.cliff);
            assert!((t.x * t.x + t.z * t.z).sqrt() > 300.);
            // Its top well above its debris cone, which runs out onto the floor.
            let top = m.tower_height(t, t.x, t.z).0;
            if top > 0.5 * m.height { tall += 1; }
            assert!(m.height(t.x, t.z) >= top - 1e-9);
            let (c, s) = (t.angle.cos(), t.angle.sin());
            let off = t.radius * (t.aspect + 1.) + 2. * t.height * t.talus;
            assert!(m.tower_height(t, t.x - s * off, t.z + c * off).0 < 0.5 * t.height * t.talus, "{}", top);
        }
        assert!(tall > m.towers.len() / 2, "{} of {}", tall, m.towers.len());
    }

    #[test]
    fn test_mesas_cover_the_asked_fraction_and_keep_the_clearing() {
        let m = mesas(0.2);
        let n = 120;
        let tops = (0 .. n * n).filter(|k| {
            let (x, z) = (-3000. + 6000. * ((k % n) as f64 + 0.5) / n as f64, -3000. + 6000. * ((k / n) as f64 + 0.5) / n as f64);
            m.height(x, z) > 0.3 * m.height
        }).count() as f64 / (n * n) as f64;
        assert!((0.12 .. 0.3).contains(&tops), "{}", tops);
        // The floor between them is flat: no aprons without cliffs.
        let raised = (0 .. n * n).filter(|k| {
            let (x, z) = (-3000. + 6000. * ((k % n) as f64 + 0.5) / n as f64, -3000. + 6000. * ((k / n) as f64 + 0.5) / n as f64);
            m.distance(x, z) < -300. && m.height(x, z) > 1.
        }).count();
        assert_eq!(raised, 0);
        assert!(m.height(0., 0.) < 1., "{}", m.height(0., 0.));
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
