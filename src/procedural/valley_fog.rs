//! Valley fog: cold, saturated air pooled in the low ground, as on a clear, still night and
//! the morning after. A density field for `volume::Volume`, over whatever ground the scene has.
//!
//! On a clear night the ground radiates its heat to the sky and chills the air touching it.
//! Cold air is dense: on slopes it drains downhill, as a slow katabatic flow, and collects in
//! the valley bottoms and basins, as water would, under a temperature inversion. Where it
//! cools past its dew point it condenses into radiation fog. So the fog lies in the hollows,
//! with a surface nearly level across each valley (a lake of cold air), filling it to some
//! depth, typically a hundred metres or two, and the ridges and hills stand out above it; from
//! above, a "sea of fog" with islands. Its top is crisp, where the fog radiates and cools
//! most, rounded into billows a few tens of metres across, and laps raggedly against the
//! slopes; above it, the air is clearer but still a little hazy (C. D. Whiteman, "Mountain
//! Meteorology", Oxford University Press, 2000, chapters 8 and 13; Gultepe et al., "Fog
//! research: a review of past achievements and future perspectives", Pure and Applied
//! Geophysics 164, 2007).
//!
//! The ground is sampled on a grid. Each point's *floor* is the low ground the cold air pools
//! over: the lowest ground within `pool_size` (a minimum filter, as wide as the valleys), then
//! smoothed over the same width, so the floor is level across a valley and descends gently
//! along it, and over a ridge it is the floor of the valleys beside it. The fog fills `depth`
//! metres above the floor, up to `top` (an inversion's altitude, if any, the same everywhere:
//! a sea of fog over a whole region). Both `density` and `depth` are function graphs
//! (`crate::graph`), so they can vary from place to place.
//!
//! Its extinction is `density` in the pool, fading out over `softness` metres at the top; the
//! top heaves into billows `top_variation` high over `billow_size`, and `raggedness` metres of
//! smaller 3D detail fray it. `variation` makes the fog patchier within. Above it, `haze`
//! (extinction) thins with height over `haze_height`.

use crate::graph::{Context, Graph};
use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::volume::DensityField;
use rayon::prelude::*;
use std::sync::OnceLock;

/// Values on a square grid over the fog's region, interpolated bilinearly.
#[derive(Clone)]
struct Grid {
    origin: Vector2<f64>,
    texel: f64,
    n: usize,
    values: Vec<f64>,
}

impl Grid {
    fn at(&self, x: f64, z: f64) -> f64 {
        let u = ((x - self.origin.x) / self.texel).clamp(0., (self.n - 1) as f64);
        let v = ((z - self.origin.y) / self.texel).clamp(0., (self.n - 1) as f64);
        let (i, j) = ((u.floor() as usize).min(self.n - 2), (v.floor() as usize).min(self.n - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let h = |i: usize, j: usize| self.values[j * self.n + i];
        let a = h(i, j) + (h(i + 1, j) - h(i, j)) * fu;
        let b = h(i, j + 1) + (h(i + 1, j + 1) - h(i, j + 1)) * fu;
        a + (b - a) * fv
    }

    /// The grid filtered along its rows, then its columns, by `f` over the `2 r + 1` values
    /// around each (clamped at the edges).
    fn separable(&self, r: usize, f: impl Fn(&[f64]) -> f64 + Sync) -> Grid {
        let n = self.n;
        let pass = |src: &Vec<f64>, rows: bool| -> Vec<f64> {
            (0..n * n).into_par_iter().map(|k| {
                let (i, j) = (k % n, k / n);
                let c = if rows { i } else { j };
                let window: Vec<f64> = (c.saturating_sub(r)..=(c + r).min(n - 1))
                    .map(|m| if rows { src[j * n + m] } else { src[m * n + i] })
                    .collect();
                f(&window)
            }).collect()
        };
        let rows = pass(&self.values, true);
        Grid { values: pass(&rows, false), ..self.clone() }
    }
}

fn smoothstep(x: f64) -> f64 {
    let x = x.clamp(0., 1.);
    x * x * (3. - 2. * x)
}

/// How many grid texels a side the tiles bounding the fog's top are.
const TILE: usize = 8;

/// Haze is cut off this many `haze_height`s above the fog.
const HAZE_SCALES: f64 = 4.;

pub struct ValleyFog {
    /// Extinction (per metre) in the pool: a graph, evaluated at each point.
    pub density: Graph,
    /// How deep the fog is above the floor (m): a graph, evaluated on the floor.
    pub depth: Graph,
    /// The altitude above which there is no fog (an inversion), if any.
    pub top: f64,
    /// Over how many metres the fog fades out at its top.
    pub softness: f64,
    /// How high the billows of its top rise and fall (m), and how far apart they are (m).
    pub top_variation: f64,
    pub billow_size: f64,
    /// Smaller, 3D detail fraying the top (m), and its size (m).
    pub raggedness: f64,
    pub detail_size: f64,
    /// How patchy the fog is within (0 to 1).
    pub variation: f64,
    /// Haze above the fog: extinction at the top, and the height over which it thins (m).
    pub haze: f64,
    pub haze_height: f64,
    ground: Grid,
    floor: Grid,
    center: Vector2<f64>,
    extent: f64,
    noise: PerlinNoise,
    /// The highest the fog (with its haze) reaches over each tile of `TILE` texels a side.
    ceilings: OnceLock<Vec<f64>>,
}

impl ValleyFog {
    /// Over the square `extent` either side of `center` ([x, z]), the ground's height sampled
    /// on a grid `resolution` a side by `ground` (None where there is none: the lowest ground
    /// found stands in), the cold air pooling over hollows `pool_size` metres across.
    pub fn new(center: Vector2<f64>, extent: f64, resolution: usize, pool_size: f64, seed: u64,
               ground: impl Fn(f64, f64) -> Option<f64> + Sync) -> ValleyFog {
        let n = resolution.max(2);
        let origin = center - Vector2::new(extent, extent);
        let texel = 2. * extent / (n - 1) as f64;
        let heights: Vec<Option<f64>> = (0..n * n).into_par_iter()
            .map(|k| ground(origin.x + (k % n) as f64 * texel, origin.y + (k / n) as f64 * texel))
            .collect();
        let lowest = heights.iter().flatten().cloned().fold(f64::INFINITY, f64::min);
        let lowest = if lowest.is_finite() { lowest } else { 0. };
        let ground = Grid { origin, texel, n, values: heights.into_iter().map(|h| h.unwrap_or(lowest)).collect() };
        // The pooled floor: the lowest ground within half a pool either way, then smoothed
        // twice over the same width (nearly Gaussian).
        let r = ((0.5 * pool_size / texel).round() as usize).max(1);
        let min = |w: &[f64]| w.iter().cloned().fold(f64::INFINITY, f64::min);
        let mean = |w: &[f64]| w.iter().sum::<f64>() / w.len() as f64;
        let floor = ground.separable(r, min).separable(r, mean).separable(r, mean);
        ValleyFog {
            density: Graph::parse(&serde_json::json!(0.02)).unwrap(),
            depth: Graph::parse(&serde_json::json!(150.)).unwrap(),
            top: f64::INFINITY,
            softness: 15.,
            top_variation: 12.,
            billow_size: 120.,
            raggedness: 6.,
            detail_size: 25.,
            variation: 0.3,
            haze: 0.,
            haze_height: 150.,
            ground,
            floor,
            center,
            extent,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(29).wrapping_add(11)),
            ceilings: OnceLock::new(),
        }
    }

    /// The ground's height, as sampled.
    pub fn ground_at(&self, x: f64, z: f64) -> f64 {
        self.ground.at(x, z)
    }

    /// The floor the cold air pools over.
    pub fn floor_at(&self, x: f64, z: f64) -> f64 {
        self.floor.at(x, z)
    }

    /// The fog's mean top (m) over (x, z), before its billows and detail.
    pub fn top_at(&self, x: f64, z: f64) -> f64 {
        let f = self.floor.at(x, z);
        let p = Vector3::new(x, f, z);
        (f + self.depth.eval(&Context::new(p, Vector3::y(), 0.)).max(0.)).min(self.top)
    }

    /// The lowest the fog can be: the lowest ground.
    pub fn bottom(&self) -> f64 {
        self.ground.values.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// How far above its mean top the fog can reach.
    fn reach(&self) -> f64 {
        let fog = self.top_variation + self.raggedness;
        if self.haze > 0. { fog + HAZE_SCALES * self.haze_height } else { fog }
    }

    /// The highest the fog (or its haze) can be.
    pub fn ceiling(&self) -> f64 {
        let highest = self.floor.values.iter().cloned().fold(f64::MIN, f64::max);
        (highest + self.depth.range().1.max(0.)).min(self.top) + self.reach()
    }

    /// The square region the fog fills: [x, z] middle and half-width.
    pub fn region(&self) -> (Vector2<f64>, f64) {
        (self.center, self.extent)
    }

    fn tiles(&self) -> usize {
        (self.floor.n - 1).div_ceil(TILE)
    }

    fn ceilings(&self) -> &Vec<f64> {
        self.ceilings.get_or_init(|| {
            let (n, t) = (self.floor.n, self.tiles());
            let deepest = self.depth.range().1.max(0.);
            (0..t * t).map(|k| {
                let (ti, tj) = (k % t, k / t);
                let mut highest = f64::MIN;
                for j in tj * TILE..=((tj + 1) * TILE).min(n - 1) {
                    for i in ti * TILE..=((ti + 1) * TILE).min(n - 1) {
                        highest = highest.max(self.floor.values[j * n + i]);
                    }
                }
                (highest + deepest).min(self.top) + self.reach()
            }).collect()
        })
    }

    fn majorant_in_fog(&self) -> f64 {
        self.density.range().1.max(0.) + self.haze
    }

    fn extinction(&self, p: &Vector3<f64>) -> f64 {
        if (Vector2::new(p.x, p.z) - self.center).abs().max() > self.extent {
            return 0.;
        }
        let top = self.top_at(p.x, p.z);
        if p.y > top + self.reach() {
            return 0.;
        }
        let haze = if self.haze > 0. {
            let above = (p.y - top).max(0.) / self.haze_height;
            let cut = (-HAZE_SCALES).exp();
            self.haze * (((-above).exp() - cut) / (1. - cut)).max(0.)
        } else {
            0.
        };
        if p.y > top + self.top_variation + self.raggedness {
            return haze;
        }
        // The billowed top: rounded heaps (|noise|) in plan, frayed by finer 3D detail.
        let b = self.billow_size;
        let heave = 1. - (2. * self.noise.fbm(p.x / b, 0.37, p.z / b, 3, 0.5, 2.) - 1.).abs() * 2.;
        let s = self.detail_size;
        let fray = 2. * self.noise.fbm(p.x / s, p.y / s, p.z / s + 71.3, 3, 0.5, 2.) - 1.;
        let surface = top + self.top_variation * heave.clamp(-1., 1.) + self.raggedness * fray.clamp(-1., 1.);
        let fade = smoothstep((surface - p.y) / self.softness.max(1e-6));
        if fade <= 0. {
            return haze;
        }
        let density = self.density.eval(&Context::new(*p, Vector3::y(), 0.)).max(0.);
        let patch = if self.variation > 0. {
            let q = p / (4. * s);
            let n = self.noise.fbm(q.x + 13.1, q.y, q.z, 3, 0.5, 2.);
            1. - self.variation * smoothstep(2. * n - 0.5)
        } else {
            1.
        };
        density * patch * fade + haze
    }
}

impl DensityField for ValleyFog {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.extinction(p)
    }

    fn majorant(&self) -> f64 {
        self.majorant_in_fog()
    }

    /// Over each tile of the grid, nothing above the highest the fog reaches there: rays above
    /// the fog, and over the ridges standing out of it, skip it.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let t = self.tiles();
        let size = self.floor.texel * TILE as f64;
        let u = ((ahead.x - self.floor.origin.x) / size).clamp(0., t as f64 - 1e-9);
        let v = ((ahead.z - self.floor.origin.y) / size).clamp(0., t as f64 - 1e-9);
        let (ti, tj) = (u.floor() as usize, v.floor() as usize);
        let ceiling = self.ceilings()[tj * t + ti];
        // How far along d the ray stays in this tile (in plan), and above or below its ceiling.
        let exit = |c: f64, lo: f64, dc: f64| -> f64 {
            if dc > 0. { (lo + size - c) / dc } else if dc < 0. { (lo - c) / dc } else { f64::INFINITY }
        };
        let (x0, z0) = (self.floor.origin.x + ti as f64 * size, self.floor.origin.y + tj as f64 * size);
        // Tiles at the region's edge extend outwards.
        let in_tile = {
            let ex = if (d.x < 0. && ti == 0) || (d.x > 0. && ti == t - 1) { f64::INFINITY } else { exit(ahead.x, x0, d.x) };
            let ez = if (d.z < 0. && tj == 0) || (d.z > 0. && tj == t - 1) { f64::INFINITY } else { exit(ahead.z, z0, d.z) };
            ex.min(ez).max(0.)
        };
        let to_ceiling = if d.y != 0. { (ceiling - ahead.y) / d.y } else { f64::INFINITY };
        let length = if to_ceiling > 0. { in_tile.min(to_ceiling) } else { in_tile };
        let majorant = if ahead.y >= ceiling { 0. } else { self.majorant_in_fog() };
        (majorant, length + 0.01)
    }

    /// Fog droplets, a few micrometres across.
    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(4.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    /// Parallel ridges running along z, 400 m high and 2 km apart, the valleys between them
    /// descending gently northwards.
    fn ridges(x: f64, z: f64) -> Option<f64> {
        Some(200. * (1. - (x * std::f64::consts::PI / 1000.).cos()) - 0.02 * z)
    }

    fn fog() -> ValleyFog {
        ValleyFog::new(Vector2::new(0., 0.), 4000., 257, 1500., 1, ridges)
    }

    #[test]
    fn test_fog_fills_valleys_not_ridges() {
        let f = fog();
        // Valley bottoms (x = 0, ±2000) are foggy just above the ground; crests (±1000) aren't.
        for z in [-2000., 0., 2000.] {
            for x in [-2000., 0., 2000.] {
                let g = ridges(x, z).unwrap();
                assert!(f.density(&Vector3::new(x, g + 30., z)) > 0.01, "valley at {}, {}", x, z);
                assert_eq!(f.density(&Vector3::new(x, g + 600., z)), 0.);
            }
            for x in [-1000., 1000.] {
                let g = ridges(x, z).unwrap();
                assert_eq!(f.density(&Vector3::new(x, g + 5., z)), 0., "crest at {}, {}", x, z);
            }
        }
    }

    #[test]
    fn test_top_is_level_across_a_valley() {
        let f = fog();
        let mid = f.top_at(0., 0.);
        for x in [-300., -150., 150., 300.] {
            assert!((f.top_at(x, 0.) - mid).abs() < 25., "{} vs {} at {}", f.top_at(x, 0.), mid, x);
        }
        // Depth about as asked above the valley bottom.
        let depth = mid - ridges(0., 0.).unwrap();
        assert!(depth > 50. && depth < 200., "{}", depth);
    }

    #[test]
    fn test_inversion_caps_the_fog() {
        let mut f = fog();
        f.depth = Graph::parse(&serde_json::json!(10000.)).unwrap();
        f.top = 100.;
        assert!(f.density(&Vector3::new(1000., 80., 0.)) > 0. || ridges(1000., 0.).unwrap() > 80.);
        assert_eq!(f.density(&Vector3::new(0., 100. + f.reach() + 1., 0.)), 0.);
        assert!(f.ceiling() <= 100. + f.reach() + 1e-9);
    }

    #[test]
    fn test_majorant_bounds_density() {
        let mut f = fog();
        f.haze = 0.002;
        let mut rng = Pcg32::new(4, 0);
        for _ in 0..5000 {
            let p = Vector3::new(rng.next_f64() * 8000. - 4000., rng.next_f64() * 900. - 200., rng.next_f64() * 8000. - 4000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = f.majorant_from(&p, &d);
            assert!(len > 0.);
            for k in 0..10 {
                let q = p + d * (len.min(5000.) * k as f64 / 10.);
                assert!(f.density(&q) <= m + 1e-12, "{} > {} at {}", f.density(&q), m, q);
            }
        }
    }

    #[test]
    fn test_graph_varies_depth() {
        let mut f = fog();
        f.depth = Graph::parse(&serde_json::json!({"node": "expr", "expr": "select(x, 50, 300)", "range": [50, 300]})).unwrap();
        assert!(f.top_at(2000., 0.) - f.floor_at(2000., 0.) > f.top_at(-2000., 0.) - f.floor_at(-2000., 0.) + 100.);
    }
}
