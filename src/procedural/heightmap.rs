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
//!   `floor_ledges` (the height of the low steps the floor is terraced in, metres), `domes`
//!   (the size of the slickrock domes the tops weather into, `relief` high), `domain` ([min
//!   x, min z, max x, max z]: the area the mesas are laid out over, so that a fine grid of the
//!   near country and a coarse one of the far can share them) and `hole` (a rectangle sunk
//!   out of sight, for the coarse grid where the fine one is), `uplift` ([x, z, radius,
//!   height]: a mesa there, about three quarters of the radius across, that much taller),
//!   `joints` ([spacing, depth]: vertical joints weathered into slots, splitting cliffs into
//!   columns and fins), `taper` (how much the cliffs' steepness varies), `shoulder` (rounded
//!   rims, a fraction of `height`), `valley` ([x, z, inner, outer]: few mesas near (x, z),
//!   tablelands past `outer`), `washes` (depth of dry washes across the floor) and `buttes`
//!   ([[x, z, radius], ...]: buttes placed by hand), `laminae` (`{"seed", "scale", "set",
//!   "relief"}`: a rock material's cross-bedding as relief on the tops, for a fine grid seen
//!   close)
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
use std::f64::consts::PI;

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
    /// The size of the sandstone domes the tops are weathered into (metres; 0 for low rounded
    /// hills instead), `relief` of `height` high.
    pub domes: f64,
    /// A rectangle [min x, min z, max x, max z] where the surface is sunk out of sight, so a
    /// coarse grid of the far country can leave the near country to a finer one.
    pub hole: Option<[f64; 4]>,
    /// [x, z, radius, height]: a mesa stands there, about three quarters of the radius across,
    /// that much taller than the others, as a higher remnant of the plateau does (Hunts Mesa
    /// above Monument Valley).
    pub uplift: Option<[f64; 4]>,
    /// Vertical joints: [spacing, depth] in metres. Two sets at right angles, each joint
    /// weathered into a slot up to `depth` into the cliff (twice that where two cross), splitting it into columns and fins,
    /// as the Mittens' thumbs were split off.
    pub joints: Option<[f64; 2]>,
    /// How much the cliff band's width varies along the outline (0: the same everywhere), so
    /// some faces stand sheer and others taper.
    pub taper: f64,
    /// Rounding of the rims, as a fraction of `height`: the tops fall away in a convex
    /// shoulder to the cliff rather than meeting it at a sharp edge.
    pub shoulder: f64,
    /// [x, z, inner, outer]: few mesas within `inner` of (x, z), more beyond, and continuous
    /// tablelands past `outer`, as a valley ringed by the plateau it was cut from.
    pub valley: Option<[f64; 4]>,
    /// The depth of the dry washes winding across the floor, metres (0 for none).
    pub washes: f64,
    /// The cross-bedded laminae of the slickrock as real relief, matching a `rock` material's
    /// (`material::rock::cross_beds`): its noise (from its seed), `scale` and `cross_set`, and
    /// the relief in metres between the softest and hardest laminae. Only on the tops.
    pub laminae: Option<(crate::noise::SeededNoise, f64, f64, f64)>,
    /// [x, z, radius] each: buttes placed by hand, about three quarters of their radius
    /// across, as landmarks.
    pub buttes: Vec<[f64; 3]>,
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
            relief: 0., height_variation: 0., roughness: 0., floor_ledges: 0., domes: 0., hole: None, uplift: None, joints: None, taper: 0., shoulder: 0., valley: None, washes: 0., buttes: vec![], laminae: None,
            towers: vec![], tower_cell: 1., tower_grid: Default::default(),
            sdf: vec![], sdf_min: min, sdf_cell: 1., sdf_n: (1, 1),
        };
        m.build_distances(min, max, coverage);
        m
    }

    /// As `new`, with `uplift` (see the field), the outlines cut again to take it in.
    pub fn with_uplift(self, uplift: [f64; 4], coverage: f64) -> MesaHeight {
        self.reshaped(Some(uplift), None, vec![], coverage)
    }

    /// As `new`, with `uplift` and `valley` (see the field), the outlines cut again to take
    /// them in.
    pub fn reshaped(mut self, uplift: Option<[f64; 4]>, valley: Option<[f64; 4]>, buttes: Vec<[f64; 3]>, coverage: f64) -> MesaHeight {
        self.uplift = uplift;
        self.valley = valley;
        self.buttes = buttes;
        let (nx, nz) = self.sdf_n;
        let min = self.sdf_min;
        let max = (min.0 + (nx - 1) as f64 * self.sdf_cell, min.1 + (nz - 1) as f64 * self.sdf_cell);
        self.build_distances(min, max, coverage);
        self
    }

    /// Cut the outlines from the field over [min, max] so they cover `coverage` of it, and find
    /// the signed distance from them.
    fn build_distances(&mut self, min: (f64, f64), max: (f64, f64), coverage: f64) {
        let cliff = self.cliff;
        // Fine enough for the cliffs, at most `SDF_CELLS` a side.
        let extent = (max.0 - min.0).max(max.1 - min.1);
        let cell = (0.5 * cliff).max(extent / SDF_CELLS).max(1e-6);
        let (nx, nz) = (((max.0 - min.0) / cell).ceil() as usize + 1, ((max.1 - min.1) / cell).ceil() as usize + 1);
        let field: Vec<f64> = {
            use rayon::prelude::*;
            let m = &*self;
            (0 .. nx * nz).into_par_iter().map(|k| m.field(min.0 + (k % nx) as f64 * cell, min.1 + (k / nx) as f64 * cell)).collect()
        };
        let mut sorted = field.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let threshold = sorted[(((1. - coverage.clamp(0., 1.)) * sorted.len() as f64) as usize).min(sorted.len() - 1)];
        let inside: Vec<bool> = field.iter().map(|f| *f > threshold).collect();
        let to_outside = distance_transform(&inside, nx, nz, true);
        let to_inside = distance_transform(&inside, nx, nz, false);
        // Half a cell either side of the edge, so it falls between the cells.
        self.sdf = (0 .. nx * nz).map(|k| {
            if inside[k] { (to_outside[k].sqrt() - 0.5) * cell } else { -(to_inside[k].sqrt() - 0.5) * cell }
        } as f32).collect();
        self.sdf_min = min;
        self.sdf_cell = cell;
        self.sdf_n = (nx, nz);
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
        if let Some([vx, vz, inner, outer]) = self.valley {
            // Beyond the valley, the outlines come from noise four times broader, raised: a
            // few long tablelands rather than a crowd of small mesas.
            let d = ((x - vx).powi(2) + (z - vz).powi(2)).sqrt();
            let t = smoothstep(inner, outer, d);
            if t > 0. {
                let (bx, bz) = (x / (4. * self.scale) + 3.7, z / (4. * self.scale) - 5.3);
                let broad = self.noise.fbm(bx, 2.9, bz, 4, 0.45, 2.);
                f = f * (1. - t) + (broad + 0.1) * t;
            }
        }
        for (k, &[bx, bz, r]) in self.buttes.iter().enumerate() {
            // Elongated along its own axis, and warped, so its outline is irregular.
            let (angle, aspect) = (hash01(k as i64, 7, 1) * PI, 1. + 1.2 * hash01(k as i64, 7, 2));
            let w = |o: f64| 0.35 * r * (2. * self.noise.fbm(x / r + o, 7.3 + k as f64, z / r - o, 3, 0.5, 2.) - 1.);
            let (dx, dz) = (x - bx + w(3.1), z - bz + w(8.7));
            let (u, v) = (dx * angle.cos() + dz * angle.sin(), -dx * angle.sin() + dz * angle.cos());
            let d = ((u / aspect.sqrt()).powi(2) + (v * aspect.sqrt()).powi(2)).sqrt();
            f += 0.6 * (1. - smoothstep(0.5 * r, r, d));
        }
        if let Some([ux, uz, r, _]) = self.uplift {
            // A mesa stands there, its edge about three quarters of the way out.
            let d = ((x - ux).powi(2) + (z - uz).powi(2)).sqrt();
            f += 0.6 * (1. - smoothstep(0.5 * r, r, d));
        }
        f
    }

    /// Signed distance (metres, positive inside) from the outlines, interpolated, notched.
    pub fn distance(&self, x: f64, z: f64) -> f64 {
        let d = self.base_distance(x, z);
        // Embayments a few cliff-widths across, and smaller notches.
        let n = |s: f64, o: f64| 2. * self.noise.fbm(x / s + o, 3.3, z / s - o, 3, 0.5, 2.) - 1.;
        let mut d = d + 2.5 * self.cliff * n(6. * self.cliff, 17.7) + 0.6 * self.cliff * n(1.5 * self.cliff, 41.1);
        if let Some([spacing, depth]) = self.joints {
            // Two sets of joints at right angles, turned by the seed, wavering a little; each
            // open (weathered into a slot) only in places.
            // One dominant set, open along much of its length, and a cross set, open only here
            // and there and shallower: buttes are split into slabs and fins, not a grid of
            // blocks.
            for (k, (u, salt)) in self.joint_coords(x, z, spacing).into_iter().zip([5.3, 9.1]).enumerate() {
                let line = (u - u.round()).abs() * spacing;
                let (bias, deep) = if k == 0 { (-0.1, 1.) } else { (0.35, 0.5) };
                let open = smoothstep(bias, bias + 0.4, n(3. * spacing, salt + 11. * k as f64));
                let width = 0.08 * spacing;
                d -= depth * deep * open * (1. - smoothstep(0., width, line));
            }
        }
        d
    }

    /// Where (x, z) lies across the two sets of joints `spacing` apart: the joints are at whole
    /// numbers, turned by the seed and wavering a little.
    fn joint_coords(&self, x: f64, z: f64, spacing: f64) -> [f64; 2] {
        let n = |s: f64, o: f64| 2. * self.noise.fbm(x / s + o, 3.3, z / s - o, 3, 0.5, 2.) - 1.;
        let angle = self.offset.0 * 2.7;
        let (c, s) = (angle.cos(), angle.sin());
        // Irregularly spaced: the coordinates warped by noise a joint's spacing across.
        [(x * c + z * s) / spacing + 0.35 * n(2. * spacing, 5.3), (-x * s + z * c) / (1.7 * spacing) + 0.35 * n(3. * spacing, 9.1)]
    }

    /// How far the column of rock between joints at (x, z) has been worn down, as a fraction
    /// of the mesa's height: columns at the rim stand at their own heights, some worn to
    /// stubs, so a cliff breaks into stepped spires; inland they keep the top's height.
    fn column_drop(&self, x: f64, z: f64, d: f64, cliff: f64) -> f64 {
        let Some([spacing, _]) = self.joints else { return 0. };
        let [u, v] = self.joint_coords(x, z, spacing);
        let (i, j) = (u.floor() as i64, v.floor() as i64);
        let h = hash01(i, j, 77);
        // Most columns whole or slightly lowered; some well down.
        let drop = 0.1 * h.powi(3) + 0.02 * h;
        // Only at the rim, fading inland over a couple of columns.
        drop * (1. - smoothstep(cliff, cliff + 2. * spacing, d))
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
            // Banded shale under the cliff: alternating ledges of siltstone and slopes of shale.
            let n = 7.;
            let ledges = ((u * n).floor() + smoothstep(0., 0.3, (u * n).fract())) / n;
            let u = u + 0.7 * (ledges - u) * smoothstep(0.1, 0.35, u);
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

    /// Sandstone domes, 0 to 1: rounded, swollen knobs on a jittered grid of `domes` cells, each
    /// its own size and height, meeting in the sharp-floored joints between them, as massive
    /// cross-bedded sandstone weathers along its joints (slickrock).
    fn dome_field(&self, x: f64, z: f64) -> f64 {
        dome_value(&self.noise, x, z, self.domes)
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

/// Slickrock domes, 0 to 1: rounded, swollen knobs on a jittered grid of cells `size` across,
/// each its own size and height, meeting in the sharp-floored joints between them, as massive
/// cross-bedded sandstone weathers along its joints. Their flanks step down in thin ledges
/// where harder beds stand out, and their crowns hold potholes (weathering pits, "tinajas")
/// where rain collects.
pub fn dome_value(noise: &PerlinNoise, x: f64, z: f64, size: f64) -> f64 {
    let warp = 0.3 * size * noise.fbm(x / (2. * size), 9.1, z / (2. * size), 2, 0.5, 2.);
    let (u, v) = ((x + warp) / size, (z - warp) / size);
    let (ci, cj) = (u.floor() as i64, v.floor() as i64);
    let mut best = 0f64;
    for i in ci - 1 ..= ci + 1 {
        for j in cj - 1 ..= cj + 1 {
            let (cx, cz) = (i as f64 + 0.2 + 0.6 * hash01(i, j, 1), j as f64 + 0.2 + 0.6 * hash01(i, j, 2));
            let radius = 0.6 + 0.5 * hash01(i, j, 3);
            let height = 0.35 + 0.65 * hash01(i, j, 4);
            let r = ((u - cx).powi(2) + (v - cz).powi(2)).sqrt() / radius;
            // A rounded dome, swelling smoothly from the joints to its crown.
            let t = (1. - r * r).max(0.);
            best = best.max(height * t * t * (3. - 2. * t));
        }
    }
    // Ledges: the dome terraced in steps of a bed's thickness, half way.
    let step = 1. / 9.;
    let f = best / step;
    let terraced = step * (f.floor() + smoothstep(0., 0.35, f.fract()));
    let mut value = 0.5 * (best + terraced);
    // Potholes, on the flatter tops: round pits on a finer jittered grid.
    let (pu, pv) = (u * 5., v * 5.);
    let (pi, pj) = (pu.floor() as i64, pv.floor() as i64);
    for i in pi - 1 ..= pi + 1 {
        for j in pj - 1 ..= pj + 1 {
            if hash01(i, j, 11) > 0.25 { continue }
            let (cx, cz) = (i as f64 + 0.5 + 0.3 * (hash01(i, j, 12) - 0.5), j as f64 + 0.5 + 0.3 * (hash01(i, j, 13) - 0.5));
            let r = ((pu - cx).powi(2) + (pv - cz).powi(2)).sqrt() / (0.25 + 0.2 * hash01(i, j, 14));
            if r < 1. {
                value -= 0.04 * (1. - r * r) * smoothstep(0.3, 0.6, best);
            }
        }
    }
    value
}

/// How much (x, z) lies in a dry wash, 0 to 1, for a floor whose undulations are `floor_scale`
/// across: the washes wind along the zero lines of broad noise (`MesaHeight::washes`).
pub fn wash_value(noise: &PerlinNoise, x: f64, z: f64, floor_scale: f64) -> f64 {
    let (s, o) = (2.5 * floor_scale, 44.4);
    let w = (2. * noise.fbm(x / s + o, 0.37 * o, z / s - o, 3, 0.5, 2.) - 1.).abs();
    1. - smoothstep(0., 0.05, w)
}

/// How far (x, z) lies in a hollow of the floor's undulations, 0 to 1: where sand collects.
pub fn hollow_value(noise: &PerlinNoise, x: f64, z: f64, floor_scale: f64) -> f64 {
    let (s, o) = (floor_scale, 71.3);
    let f = 2. * noise.fbm(x / s + o, 0.37 * o, z / s - o, 4, 0.5, 2.) - 1.;
    smoothstep(0., -0.35, f)
}

/// The most cells along a side of a mesa field's distance grid.
const SDF_CELLS: f64 = 3000.;

/// A hash of integers to [0, 1).
fn hash01(i: i64, j: i64, salt: u64) -> f64 {
    let mut h = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (j as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f) ^ salt.wrapping_mul(0x1656_67b1_9e37_79f9);
    h ^= h >> 31;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 29;
    (h >> 11) as f64 / (1u64 << 53) as f64
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
            // Terraced: each step a short, steep riser and a gently rolling tread; only on the
            // floor, not on the tops.
            let step = self.floor_ledges;
            let f = floor / step + 0.15 * n(0.2 * self.floor_scale, 3.7, 3);
            let ledged = step * (f.floor() + smoothstep(0., 0.3, f.fract()));
            floor += (ledged - floor) * (1. - smoothstep(-self.cliff, 0., d));
        }
        if self.washes > 0. {
            // Dry washes: sinuous channels along the zero lines of broad noise, on the floor.
            let wash = wash_value(&self.noise, x, z, self.floor_scale);
            floor -= self.washes * wash * (1. - smoothstep(-3. * self.cliff, -self.cliff, d));
        }
        // Each mesa's top at its own height, as erosion has stripped more from some.
        let mut h = self.height * (1. + self.height_variation * n(3. * self.scale, 23.9, 2));
        if let Some([ux, uz, r, extra]) = self.uplift {
            h += extra * (1. - smoothstep(r, 1.5 * r, ((x - ux).powi(2) + (z - uz).powi(2)).sqrt()));
        }
        let width = self.talus_width * h / self.height;
        let cliff = self.cliff * (1. + self.taper * (0.5 + 0.5 * n(8. * self.cliff, 31.3, 2)).max(0.) * 2.);
        let (mut profile, mut slope) = self.shape(d, h, self.talus, width, cliff);
        if d > 0. && self.joints.is_some() {
            // Columns at the rim worn to their own heights; the cliff below them stays.
            let floor_of_cliff = h * self.talus;
            // Not on the uplifted mesa: it is the one looked out from.
            let uplifted = self.uplift.map_or(0., |[ux, uz, r, _]| 1. - smoothstep(r, 1.5 * r, ((x - ux).powi(2) + (z - uz).powi(2)).sqrt()));
            let drop = self.column_drop(x, z, d, cliff) * self.height * (1. - uplifted);
            profile = (profile - drop).max(floor_of_cliff.min(profile));
        }
        if self.shoulder > 0. && d > 0. {
            // A convex shoulder: the top falls away towards the rim.
            let t = (1. - (d - cliff) / (2. * cliff)).clamp(0., 1.);
            profile -= self.shoulder * h * t * t * smoothstep(0., cliff, d);
        }
        // The tops: low rounded hills of the softer beds above the caprock, set back from the
        // rim, on the rubbly caprock.
        // Buttes keep thin, level caprock; domes and hills only on the uplifted mesa when there
        // is one (its top is the slickrock seen close).
        let on_uplift = self.uplift.map_or(1., |[ux, uz, r, _]| 1. - smoothstep(r, 1.5 * r, ((x - ux).powi(2) + (z - uz).powi(2)).sqrt()));
        let inside = smoothstep(self.cliff, 3. * self.cliff, d) * (0.1 + 0.9 * on_uplift);
        let hills = if self.domes > 0. { self.dome_field(x, z) } else {
            (n(6. * self.cliff, 5.1, 4) + 0.15).max(0.).powf(1.5) * 1.6
        };
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
        let sunk = match self.hole {
            Some([x0, z0, x1, z1]) if x > x0 && x < x1 && z > z0 && z < z1 => 2. * self.height,
            _ => 0.,
        };
        let mut y = self.base + floor + profile - sunk;
        if let Some((noise, scale, set, relief)) = &self.laminae {
            let tops = smoothstep(0.5 * self.cliff, 2. * self.cliff, d);
            if tops > 0. {
                let l = crate::material::rock::cross_beds(noise, Vector3::new(x, y, z) / *scale, *set);
                y += tops * relief * (l.hardness - 0.6);
            }
        }
        y
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
                // The field the mesas are laid out over: by default the heightmap's own area, but
                // grids of the same country at different resolutions must share one.
                let (dmin, dmax) = match o.get("domain") {
                    Some(d) => ((d[0].as_f64().unwrap(), d[1].as_f64().unwrap()), (d[2].as_f64().unwrap(), d[3].as_f64().unwrap())),
                    None => (min, max),
                };
                let m = MesaHeight::new(dmin, dmax, SceneFile::parse_number(&o["coverage"], 0.15), base, height,
                    SceneFile::parse_number(&o["scale"], 800.),
                    SceneFile::parse_number(&o["cliff"], 30.),
                    SceneFile::parse_int(&o["benches"], 2) as u32,
                    SceneFile::parse_number(&o["talus"], 0.35).clamp(0., 1.),
                    SceneFile::parse_number(&o["talus_width"], height),
                    SceneFile::parse_number(&o["floor_amplitude"], 2.),
                    SceneFile::parse_number(&o["floor_scale"], 150.),
                    clearing, seed);
                let four = |k: &str| o.get(k).map(|u| [0, 1, 2, 3].map(|i| u[i].as_f64().unwrap()));
                let buttes: Vec<[f64; 3]> = o["buttes"].as_array().map_or(vec![], |list| list.iter()
                    .map(|b| [0, 1, 2].map(|i| b[i].as_f64().unwrap())).collect());
                let m = if four("uplift").is_some() || four("valley").is_some() || !buttes.is_empty() {
                    m.reshaped(four("uplift"), four("valley"), buttes, SceneFile::parse_number(&o["coverage"], 0.15))
                } else { m };
                let mut m = m.with_towers(SceneFile::parse_number(&o["towers"], 0.), dmin, dmax, seed as u64);
                m.domes = SceneFile::parse_number(&o["domes"], 0.);
                m.joints = o.get("joints").map(|j| [j[0].as_f64().unwrap(), j[1].as_f64().unwrap()]);
                m.taper = SceneFile::parse_number(&o["taper"], 0.);
                m.shoulder = SceneFile::parse_number(&o["shoulder"], 0.);
                m.washes = SceneFile::parse_number(&o["washes"], 0.);
                // "laminae": {"seed": the rock's, "scale": its scale, "set": its cross_set, "relief": 0.15}
                if let Some(l) = o.get("laminae") {
                    m.laminae = Some((crate::noise::SeededNoise::new(SceneFile::parse_int(&l["seed"], 0) as u64),
                        SceneFile::parse_number(&l["scale"], 1.), SceneFile::parse_number(&l["set"], 1.),
                        SceneFile::parse_number(&l["relief"], 0.1)));
                }
                m.hole = o.get("hole").map(|h| [0, 1, 2, 3].map(|k| h[k].as_f64().unwrap()));
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
    fn test_uplift_stands_a_taller_mesa_there() {
        let m = mesas(0.1).with_uplift([1500., 1500., 500., 150.], 0.1);
        // A mesa at the uplift's centre, full height plus the uplift, level to near its edge.
        let centre = m.height(1500., 1500.);
        assert!(centre > 0.95 * (m.height + 150.), "{}", centre);
        assert!(m.height(1500., 1500. + 0.5 * 500.) > 0.95 * (m.height + 150.));
    }

    #[test]
    fn test_domes_are_rounded_and_jointed() {
        let mut m = mesas(0.2);
        m.domes = 50.;
        // Each dome is 0 to 1, peaks within a cell, and falls to joints between them.
        let samples: Vec<f64> = (0 .. 400).map(|k| m.dome_field(k as f64 * 0.5, 13.)).collect();
        let (lo, hi) = samples.iter().fold((1f64, 0f64), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
        assert!(lo >= 0. && hi <= 1. && hi > 0.5 && lo < 0.5 * hi, "{} to {}", lo, hi);
        // Smooth: no step between neighbouring samples greater than a dome's rise allows.
        assert!(samples.windows(2).all(|w| (w[1] - w[0]).abs() < 0.1));
    }

    #[test]
    fn test_placed_buttes_stand_where_asked() {
        let m = mesas(0.02).reshaped(None, None, vec![[1500., -1500., 300.]], 0.02);
        assert!(m.height(1500., -1500.) > 0.9 * m.height, "{}", m.height(1500., -1500.));
    }

    #[test]
    fn test_valley_has_fewer_mesas_than_its_rim() {
        let m = mesas(0.15).reshaped(None, Some([0., 0., 1200., 2500.]), vec![], 0.15);
        let share = |r0: f64, r1: f64| {
            let (mut tops, mut all) = (0, 0);
            for k in 0 .. 4000 {
                let (a, t) = (k as f64 * 2.399, (k as f64 + 0.5) / 4000.);
                let r = r0 + (r1 - r0) * t;
                all += 1;
                if m.distance(r * a.cos(), r * a.sin()) > 0. { tops += 1; }
            }
            tops as f64 / all as f64
        };
        assert!(share(400., 1200.) < 0.5 * share(2600., 2990.), "{} {}", share(400., 1200.), share(2600., 2990.));
    }

    #[test]
    fn test_joints_cut_slots_into_the_cliffs() {
        let mut m = mesas(0.2);
        let before: Vec<f64> = (0 .. 2000).map(|k| m.distance(k as f64 * 2.9 - 2900., 1234.)).collect();
        m.joints = Some([40., 15.]);
        let after: Vec<f64> = (0 .. 2000).map(|k| m.distance(k as f64 * 2.9 - 2900., 1234.)).collect();
        // Joints only ever eat into the mesas, by up to their depth (twice where two cross),
        // and somewhere they do.
        assert!(before.iter().zip(&after).all(|(b, a)| a <= b && b - a <= 30. + 1e-9));
        assert!(before.iter().zip(&after).any(|(b, a)| b - a > 10.));
    }

    #[test]
    fn test_terrain_texture_follows_the_washes() {
        let mut m = mesas(0.02);
        m.washes = 4.;
        let t = crate::textures::TerrainTexture {
            feature: crate::textures::TerrainFeature::Washes, scale: m.floor_scale, spread: 1., noise: PerlinNoise::new(),
        };
        // Where the texture says wash, the floor is cut down; elsewhere not.
        let mut cut = 0;
        for k in 0 .. 3000 {
            let (x, z) = (k as f64 * 1.9 - 2850., 1800.);
            let a = t.amount(&Vector3::new(x, 0., z));
            let mut flat = mesas(0.02);
            flat.washes = 0.;
            let depth = flat.height(x, z) - m.height(x, z);
            assert!((depth - 4. * a).abs() < 1e-6 || m.distance(x, z) > -3. * m.cliff, "{} {}", depth, a);
            if a > 0.9 { cut += 1; }
        }
        assert!(cut > 0);
    }

    #[test]
    fn test_domes_have_ledges_and_potholes() {
        // The terraced dome steps: along a line, some runs are much flatter than others.
        let n = PerlinNoise::new();
        let v: Vec<f64> = (0 .. 2000).map(|k| dome_value(&n, k as f64 * 0.05, 7., 50.)).collect();
        let steps: Vec<f64> = v.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
        let flat = steps.iter().filter(|d| **d < 1e-4).count();
        assert!(flat > 50, "{}", flat);
    }

    #[test]
    fn test_jointed_rims_break_into_columns_of_their_own_heights() {
        let mut m = mesas(0.2);
        m.joints = Some([40., 10.]);
        // Along a line, the rim's columns differ in height, while deep inside the tops are level.
        let (mut rim, mut inland) = (vec![], vec![]);
        for k in 0 .. 6000 {
            let (x, z) = (k as f64 * 1.0 - 3000., 777.);
            let d = m.distance(x, z);
            if d > 0. && d < m.cliff { rim.push(m.column_drop(x, z, d, m.cliff)); }
            if d > m.cliff + 100. { inland.push(m.column_drop(x, z, d, m.cliff)); }
        }
        assert!(rim.iter().cloned().fold(0., f64::max) > 0.1, "{:?}", rim.len());
        assert!(inland.iter().all(|v| *v == 0.));
    }

    #[test]
    fn test_laminae_raise_ribs_on_the_tops_only() {
        let mut m = mesas(0.2);
        let plain: Vec<(f64, f64, f64)> = (0 .. 4000).map(|k| {
            let (x, z) = ((k % 63) as f64 * 90. - 2800. + (k / 63) as f64 * 0.37, (k / 63) as f64 * 90. - 2800.);
            (m.distance(x, z), x, z)
        }).collect();
        assert!(plain.iter().any(|(d, _, _)| *d > 2. * m.cliff));
        let before: Vec<f64> = plain.iter().map(|(_, x, z)| m.height(*x, *z)).collect();
        m.laminae = Some((crate::noise::SeededNoise::new(3), 10., 0.2, 0.1));
        let mut ribs = 0;
        for ((d, x, z), b) in plain.iter().zip(&before) {
            let change = m.height(*x, *z) - b;
            if *d < 0. { assert!(change.abs() < 1e-12) } else { assert!(change.abs() <= 0.1 + 1e-9) }
            if change.abs() > 0.02 { ribs += 1; }
        }
        assert!(ribs > 0);
    }

    #[test]
    fn test_washes_cut_into_the_floor() {
        let mut m = mesas(0.02);
        let before: Vec<f64> = (0 .. 1000).map(|k| m.height(k as f64 * 3.7 - 1900., 2400.)).collect();
        m.washes = 4.;
        let after: Vec<f64> = (0 .. 1000).map(|k| m.height(k as f64 * 3.7 - 1900., 2400.)).collect();
        assert!(before.iter().zip(&after).all(|(b, a)| a <= &(b + 1e-9)));
        assert!(before.iter().zip(&after).any(|(b, a)| b - a > 2.));
    }

    #[test]
    fn test_hole_sinks_the_surface() {
        let mut m = mesas(0.2);
        let outside = m.height(-2000., -2000.);
        m.hole = Some([-2100., -2100., -1900., -1900.]);
        assert!(m.height(-2000., -2000.) < outside - m.height);
        assert_eq!(m.height(-2200., -2000.), { m.hole = None; m.height(-2200., -2000.) });
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
