//! A layer of procedural clouds: a density field for `volume::Volume`, after Andrew Schneider
//! and Nathan Vos, "The Real-time Volumetric Cloudscapes of Horizon: Zero Dawn", Advances in
//! Real-Time Rendering, SIGGRAPH 2015, and Schneider's "Nubis" follow-ups (Advances 2017,
//! 2022, 2023, <https://www.guerrilla-games.com/read/nubis-realtime-volumetric-cloudscapes-in-a-nutshell>).
//!
//! The density at a point is built up from cheap, large-scale terms to expensive, small-scale
//! ones, stopping as soon as it is zero, which is most of the time:
//!
//! 1. **Height profile.** Clouds of each type occupy a characteristic part of the layer:
//!    stratus a thin band near its base, cumulus most of it, with a flat base and rounded top.
//!    `cloud_type` blends between them.
//! 2. **Coverage.** A low-frequency 2D noise map over the ground (the "weather map") says where
//!    there is cloud and how strongly, `coverage` setting how much of it counts, so clouds
//!    gather with clear sky between. Heaped clouds grow taller where it is stronger.
//! 3. **Base shape.** Low-frequency "Perlin-Worley" noise: fractal Perlin noise remapped by
//!    inverted Worley noise, which gives Perlin's connected shapes the billowing, rounded
//!    lobes of cellular noise. The coverage then sets how much of it is cloud.
//! 4. **Erosion.** Higher-frequency Worley noise eats into the edges (where the base shape is
//!    thin) but barely touches the cores: billows near the top, wisps near the base.
//!
//! The thresholds are remaps, (v - lo) / (hi - lo) clamped to [0, 1], rather than cut-offs,
//! so density rises smoothly from the edge of a cloud, as it does in real ones.
//!
//! Most of a cloud layer is clear sky, where delta tracking with a single majorant still has to
//! stop and evaluate the noise every mean free path. So, as in Nubis, the weather map is a
//! texture: noise sampled on a grid and interpolated bilinearly. Bilinear interpolation never
//! exceeds the four corners of its texel, so the strongest weather anywhere over a texel is
//! known exactly, and with it whether that column of sky can hold any cloud, and how high (see
//! `majorant_from`). Tracking skips clear columns, and the clear air above low clouds, entirely.
//! Inside the clouds, every step evaluates the noise; the Worley noise is a precomputed, tiling
//! 3D texture (`WorleyTexture`) for the same reason.

use crate::na::{Vector2, Vector3};
use std::sync::OnceLock;
use crate::noise::{PerlinNoise, worley_hash};
use crate::volume::DensityField;

/// (v - lo) / (hi - lo), clamped to [0, 1]; a step at `hi` if the range is empty, with `hi`
/// itself below it, so a threshold of 1 on a value in [0, 1] (no coverage) passes nothing.
fn remap(v: f64, lo: f64, hi: f64) -> f64 {
    if hi <= lo {
        return if v > hi { 1. } else { 0. };
    }
    ((v - lo) / (hi - lo)).clamp(0., 1.)
}

fn smoothstep(lo: f64, hi: f64, v: f64) -> f64 {
    let t = remap(v, lo, hi);
    t * t * (3. - 2. * t)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Where in the layer (0 at its base, 1 at its top) clouds of a type rise and fall: density
/// grows from `rise.0` to `rise.1` and dies away from `fall.0` to `fall.1`.
#[derive(Clone, Copy)]
struct Profile {
    rise: (f64, f64),
    fall: (f64, f64),
}

const STRATUS: Profile = Profile { rise: (0., 0.08), fall: (0.15, 0.3) };
const STRATOCUMULUS: Profile = Profile { rise: (0., 0.12), fall: (0.35, 0.65) };
const CUMULUS: Profile = Profile { rise: (0., 0.08), fall: (0.6, 1.) };

/// The profile of `cloud_type`, from 0 (stratus) through 0.5 (stratocumulus) to 1 (cumulus).
fn profile_of(cloud_type: f64) -> Profile {
    let (a, b, t) = if cloud_type < 0.5 {
        (&STRATUS, &STRATOCUMULUS, cloud_type * 2.)
    } else {
        (&STRATOCUMULUS, &CUMULUS, cloud_type * 2. - 1.)
    };
    let mix = |x: (f64, f64), y: (f64, f64)| (lerp(x.0, y.0, t), lerp(x.1, y.1, t));
    Profile { rise: mix(a.rise, b.rise), fall: mix(a.fall, b.fall) }
}

/// The fraction of cloud density allowed at height `h` through the layer; zero from
/// `profile_of(cloud_type).fall.1` up.
fn height_profile(h: f64, cloud_type: f64) -> f64 {
    let Profile { rise, fall } = profile_of(cloud_type);
    smoothstep(rise.0, rise.1, h) * (1. - smoothstep(fall.0, fall.1, h))
}

/// The weather map: noise sampled at the corners of a grid of square texels over the layer's
/// region, and interpolated bilinearly between them. Zero outside the region.
struct WeatherMap {
    /// The (x, z) of the first corner, and the texel size.
    origin: Vector2<f64>,
    texel: f64,
    /// Texels along each side; there is one more corner.
    n: usize,
    corners: Vec<f64>,
    /// The largest corner of each texel: the most the map reaches anywhere over it.
    texel_max: Vec<f64>,
}

impl WeatherMap {
    fn new(origin: Vector2<f64>, size: f64, texel: f64, sample: impl Fn(f64, f64) -> f64) -> WeatherMap {
        let n = ((size / texel).ceil() as usize).clamp(1, 4096);
        let texel = size / n as f64;
        let corner = |i: usize, j: usize| sample(origin.x + i as f64 * texel, origin.y + j as f64 * texel);
        let corners: Vec<f64> = (0..=n).flat_map(|j| (0..=n).map(move |i| (i, j))).map(|(i, j)| corner(i, j)).collect();
        let at = |i: usize, j: usize| corners[j * (n + 1) + i];
        let texel_max = (0..n).flat_map(|j| (0..n).map(move |i| (i, j)))
            .map(|(i, j)| at(i, j).max(at(i + 1, j)).max(at(i, j + 1)).max(at(i + 1, j + 1)))
            .collect();
        WeatherMap { origin, texel, n, corners, texel_max }
    }

    /// The texel holding (x, z), and the position within it in [0, 1]^2; None outside the map.
    fn locate(&self, x: f64, z: f64) -> Option<(usize, usize, f64, f64)> {
        let u = (x - self.origin.x) / self.texel;
        let v = (z - self.origin.y) / self.texel;
        if !(u >= 0. && v >= 0. && u <= self.n as f64 && v <= self.n as f64) {
            return None;
        }
        let i = (u.floor() as usize).min(self.n - 1);
        let j = (v.floor() as usize).min(self.n - 1);
        Some((i, j, u - i as f64, v - j as f64))
    }

    fn at(&self, x: f64, z: f64) -> f64 {
        match self.locate(x, z) {
            None => 0.,
            Some((i, j, fu, fv)) => {
                let c = |i: usize, j: usize| self.corners[j * (self.n + 1) + i];
                lerp(lerp(c(i, j), c(i + 1, j), fu), lerp(c(i, j + 1), c(i + 1, j + 1), fu), fv)
            },
        }
    }
}

/// Fractal Perlin noise is in [0, 1] but rarely strays far from 0.5; stretch it to use the range.
fn stretched(n: f64) -> f64 {
    ((n - 0.5) * 2.2 + 0.5).clamp(0., 1.)
}

/// Worley noise (distance to the nearest feature point, one point per unit cell) precomputed
/// over a tile of `PERIOD` cells a side, which repeats, and interpolated trilinearly between
/// texels: Nubis's noise textures. Searching the 27 neighbouring cells took two thirds of the
/// render time when done for every sample; a lookup reads eight texels.
struct WorleyTexture {
    texels: Vec<f32>,
}

impl WorleyTexture {
    const PERIOD: usize = 16;
    const PER_CELL: usize = 4;
    const N: usize = Self::PERIOD * Self::PER_CELL;

    fn new(seed: u32) -> WorleyTexture {
        let p = Self::PERIOD as i64;
        // Feature points per cell, with cell indices wrapped so the tile repeats seamlessly.
        let feature = |cx: i64, cy: i64, cz: i64| {
            let (ix, iy, iz) = (cx.rem_euclid(p) as u32, cy.rem_euclid(p) as u32, cz.rem_euclid(p) as u32);
            Vector3::new(
                cx as f64 + worley_hash(ix, iy, iz, seed),
                cy as f64 + worley_hash(ix, iy, iz, seed.wrapping_add(1)),
                cz as f64 + worley_hash(ix, iy, iz, seed.wrapping_add(2)),
            )
        };
        let n = Self::N;
        let texels = (0..n * n * n).map(|i| {
            let (x, y, z) = (i % n, (i / n) % n, i / (n * n));
            let q = Vector3::new(x as f64, y as f64, z as f64) / Self::PER_CELL as f64;
            let c = q.map(|v| v.floor() as i64);
            let mut best = f64::MAX;
            for dx in -1..=1 { for dy in -1..=1 { for dz in -1..=1 {
                best = best.min((feature(c.x + dx, c.y + dy, c.z + dz) - q).norm_squared());
            }}}
            best.sqrt() as f32
        }).collect();
        WorleyTexture { texels }
    }

    /// The distance to the nearest feature point from `p`, in cells, interpolated.
    fn at(&self, p: &Vector3<f64>) -> f64 {
        let n = Self::N as i64;
        let q = p * Self::PER_CELL as f64;
        let (x0, y0, z0) = (q.x.floor(), q.y.floor(), q.z.floor());
        let (fx, fy, fz) = (q.x - x0, q.y - y0, q.z - z0);
        let (x0, y0, z0) = (x0 as i64, y0 as i64, z0 as i64);
        let t = |dx: i64, dy: i64, dz: i64| {
            let (x, y, z) = ((x0 + dx).rem_euclid(n), (y0 + dy).rem_euclid(n), (z0 + dz).rem_euclid(n));
            self.texels[(z * n * n + y * n + x) as usize] as f64
        };
        let along_x = |dy, dz| lerp(t(0, dy, dz), t(1, dy, dz), fx);
        lerp(lerp(along_x(0, 0), along_x(1, 0), fy), lerp(along_x(0, 1), along_x(1, 1), fy), fz)
    }
}

/// Inverted Worley noise (high at feature points, falling away between them), in three octaves:
/// round, billowing cells.
fn worley_fbm(w: &WorleyTexture, p: &Vector3<f64>) -> f64 {
    let cell = |q: Vector3<f64>| (1. - w.at(&q)).clamp(0., 1.);
    0.625 * cell(*p) + 0.25 * cell(p * 2.) + 0.125 * cell(p * 4.)
}

pub struct CloudLayer {
    /// Heights of the bottom and top of the layer.
    pub base: f64,
    pub top: f64,
    /// Extinction coefficient in the densest cloud: collisions per unit distance.
    pub max_density: f64,
    /// Average fraction of the sky covered, 0 to 1.
    pub coverage: f64,
    /// 0 stratus, 0.5 stratocumulus, 1 cumulus.
    pub cloud_type: f64,
    /// Typical sizes, in scene units, of cloud fields (the weather map), of individual clouds
    /// (the base shape), and of the eroded detail.
    pub weather_size: f64,
    pub shape_size: f64,
    pub detail_size: f64,
    /// How strongly detail erodes the edges, 0 to 1.
    pub erosion: f64,
    /// The square, `extent` either side of `center` in x and z, that the clouds fill.
    pub center: Vector2<f64>,
    pub extent: f64,
    /// Built on first use, from `weather_size`, `center` and `extent`.
    weather_map: OnceLock<WeatherMap>,
    weather: PerlinNoise,
    shape: PerlinNoise,
    seed: u64,
    /// Built on first use.
    cells: OnceLock<WorleyTexture>,
}

impl CloudLayer {
    pub fn new(base: f64, top: f64, center: Vector2<f64>, extent: f64, seed: u64) -> CloudLayer {
        assert!(top > base, "The cloud layer's top ({}) must be above its base ({})", top, base);
        CloudLayer {
            base,
            top,
            max_density: 0.05,
            coverage: 0.5,
            cloud_type: 1.,
            weather_size: (top - base) * 2.5,
            shape_size: (top - base) * 0.8,
            detail_size: (top - base) * 0.15,
            erosion: 0.4,
            center,
            extent,
            weather_map: OnceLock::new(),
            // Separate streams, so the weather map and the shapes aren't correlated.
            weather: PerlinNoise::with_seed(seed.wrapping_mul(2).wrapping_add(1)),
            shape: PerlinNoise::with_seed(seed.wrapping_mul(2)),
            seed,
            cells: OnceLock::new(),
        }
    }

    fn weather_map(&self) -> &WeatherMap {
        self.weather_map.get_or_init(|| {
            let origin = self.center - Vector2::new(self.extent, self.extent);
            // Fine enough to keep all three octaves.
            WeatherMap::new(origin, 2. * self.extent, self.weather_size / 24., |x, z| {
                stretched(self.weather.fbm(x / self.weather_size, 0.5, z / self.weather_size, 3, 0.5, 2.0))
            })
        })
    }

    fn cells(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed as u32))
    }

    /// The column strength (0 clear, to 1) for a weather map value.
    fn cell(&self, weather: f64) -> f64 {
        remap(weather, 1. - self.coverage, 1.)
    }

    /// How far up the layer (0 to 1) clouds in a column of `cell` strength may reach: stronger
    /// cells grow taller for heaped (cumulus) clouds; layered (stratus) ones stay level.
    fn reach(&self, cell: f64) -> f64 {
        lerp(1., lerp(0.3, 1., cell.sqrt()), self.cloud_type)
    }

    /// The highest any cloud can be over weather of at most `weather`, or None if none can be.
    /// The cell strength and reach only grow with the weather, and the profile is zero from
    /// `fall.1` of the reach up.
    fn column_top(&self, weather: f64) -> Option<f64> {
        let cell = self.cell(weather);
        if cell <= 0. {
            return None;
        }
        let h = (self.reach(cell) * profile_of(self.cloud_type).fall.1).min(1.);
        Some(self.base + h * (self.top - self.base))
    }

    /// Cloud density as a fraction of `max_density`, in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        // 1. Height in the layer.
        let h = (p.y - self.base) / (self.top - self.base);
        if !(0. ..=1.).contains(&h) {
            return 0.;
        }
        // 2. The weather map: how strongly this column of sky is cloud, from 0 (clear) to 1.
        let cell = self.cell(self.weather_map().at(p.x, p.z));
        if cell <= 0. {
            return 0.;
        }
        let profile = height_profile(h / self.reach(cell), self.cloud_type);
        if profile <= 0. {
            return 0.;
        }

        // 3. The base shape, cut to the cell's strength.
        let q = p / self.shape_size;
        let perlin = stretched(self.shape.fbm(q.x, q.y, q.z, 4, 0.5, 2.0));
        let perlin_worley = remap(perlin, worley_fbm(self.cells(), &q) - 1., 1.);
        let shape = remap(perlin_worley * profile, 0.7 * (1. - cell), 1.);
        if shape <= 0. {
            return 0.;
        }

        // 4. Erode the edges: billowing detail high up, its inverse (wisps) near the base.
        let d = worley_fbm(self.cells(), &(p / self.detail_size));
        let detail = lerp(1. - d, d, (h * 5.).min(1.));
        remap(shape, detail * self.erosion, 1.)
    }
}

impl DensityField for CloudLayer {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.max_density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }

    /// Within one texel of the weather map, clouds lie only below `column_top` of its strongest
    /// corner: there the bound is `max_density`, elsewhere zero. Holds to the texel's edge, or
    /// to that height if the ray crosses it first.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let map = self.weather_map();
        // Classify by a point just along the ray, so one on a texel's edge counts as in the
        // texel it is entering.
        let ahead = p + d * (map.texel * 1e-9);
        let (i, j) = match map.locate(ahead.x, ahead.z) {
            Some((i, j, _, _)) => (i, j),
            None => return (self.max_density, f64::INFINITY),
        };
        let (x0, z0) = (map.origin.x + i as f64 * map.texel, map.origin.y + j as f64 * map.texel);
        let exit = |p: f64, d: f64, lo: f64| {
            if d > 0. { (lo + map.texel - p) / d } else if d < 0. { (lo - p) / d } else { f64::INFINITY }
        };
        let edge = exit(p.x, d.x, x0).min(exit(p.z, d.z, z0)).max(0.);
        match self.column_top(map.texel_max[j * map.n + i]) {
            None => (0., edge),
            Some(top) => {
                let to_top = if d.y != 0. { (top - p.y) / d.y } else { f64::INFINITY };
                if ahead.y > top {
                    // Above the clouds: clear until the ray comes down to them.
                    (0., if to_top > 0. { edge.min(to_top) } else { edge })
                } else {
                    (self.max_density, if to_top > 0. { edge.min(to_top) } else { edge })
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(seed: u64) -> CloudLayer {
        CloudLayer::new(1000., 2000., Vector2::new(12000., 12000.), 12500., seed)
    }

    /// Fraction of a grid of points through the layer, at `y`, that are cloud.
    fn cover_at(l: &CloudLayer, y: f64) -> f64 {
        let n = 60;
        let mut cloudy = 0;
        for i in 0..n {
            for j in 0..n {
                let p = Vector3::new(i as f64 * 400., y, j as f64 * 400.);
                cloudy += (l.fraction(&p) > 0.) as usize;
            }
        }
        cloudy as f64 / (n * n) as f64
    }

    #[test]
    fn test_no_cloud_outside_the_layer() {
        let l = layer(1);
        for y in [999., 2001., -5., 1e6] {
            assert_eq!(cover_at(&l, y), 0.);
        }
    }

    #[test]
    fn test_density_is_bounded_by_the_majorant() {
        let l = layer(2);
        for i in 0..20_000 {
            let p = Vector3::new((i % 137) as f64 * 97.3, 1000. + (i % 101) as f64 * 9.9, (i / 137) as f64 * 113.);
            let d = l.density(&p);
            assert!((0. ..=l.majorant()).contains(&d), "{} at {}", d, p);
        }
    }

    #[test]
    fn test_coverage_controls_how_much_is_cloud() {
        let mut thin = layer(3);
        thin.coverage = 0.2;
        let mut thick = layer(3);
        thick.coverage = 0.8;
        let mut none = layer(3);
        none.coverage = 0.;
        let y = 1300.;
        assert_eq!(cover_at(&none, y), 0.);
        let (a, b) = (cover_at(&thin, y), cover_at(&thick, y));
        assert!(a > 0. && b > a * 1.5, "coverage 0.2: {}, 0.8: {}", a, b);
    }

    #[test]
    fn test_stratus_stays_low_and_cumulus_rises() {
        let mut stratus = layer(4);
        stratus.cloud_type = 0.;
        stratus.coverage = 0.8;
        let mut cumulus = layer(4);
        cumulus.coverage = 0.8;
        assert!(cover_at(&stratus, 1100.) > 0.);
        assert_eq!(cover_at(&stratus, 1500.), 0.);
        assert!(cover_at(&cumulus, 1500.) > 0.);
    }

    #[test]
    fn test_clouds_depend_on_seed() {
        let row = |l: CloudLayer| -> Vec<f64> {
            (0..400).map(|i| l.fraction(&Vector3::new(i as f64 * 37., 1300., 0.))).collect()
        };
        let p = row(layer(5));
        assert_ne!(p, row(layer(6)));
        assert_eq!(p, row(layer(5)));
    }

    /// A small, thick-coverage patch of cloud, quick to build.
    fn patch() -> CloudLayer {
        let mut l = CloudLayer::new(1000., 2000., Vector2::new(500., -300.), 4000., 8);
        l.coverage = 0.7;
        l.max_density = 0.002;
        l
    }

    #[test]
    fn test_local_majorant_bounds_density() {
        let l = patch();
        let mut rng = crate::geometry::Pcg32::new(11, 0);
        let mut bounded_some_empty = false;
        for _ in 0..3000 {
            let p = Vector3::new(500. + (rng.next_f64() - 0.5) * 8000., 900. + rng.next_f64() * 1200.,
                                 -300. + (rng.next_f64() - 0.5) * 8000.);
            let d = crate::geometry::random_point_on_unit_sphere();
            let (m, len) = l.majorant_from(&p, &d);
            assert!(len >= 0. && m <= l.majorant());
            bounded_some_empty |= m == 0. && len > 0.;
            for k in 0..20 {
                let q = p + d * (len.min(5000.) * k as f64 / 20.);
                assert!(l.density(&q) <= m, "{} > {} at {} (from {} along {})", l.density(&q), m, q, p, d);
            }
        }
        assert!(bounded_some_empty, "the local majorant never skipped anything");
    }

    #[test]
    fn test_tracking_through_clouds_matches_optical_depth() {
        // As volume's test for noise, but with the clouds' local majorants: rays crossing
        // clear columns and passing over low clouds must still see exactly exp(-∫σ).
        use crate::geometry::seed_rand;
        use crate::ray::Ray;
        use crate::shapes::bbox::BBox;
        use crate::shapes::geometry::Geometry;
        use crate::volume::Volume;
        seed_rand(9, 0);
        let l = patch();
        let (from, to) = (Vector3::new(-3000., 1050., -2000.), Vector3::new(4000., 1900., 1500.));
        let steps = 40_000;
        let optical_depth: f64 = (0..steps).map(|i| {
            l.density(&(from + (to - from) * ((i as f64 + 0.5) / steps as f64)))
        }).sum::<f64>() * (to - from).norm() / steps as f64;
        assert!(optical_depth > 0.2 && optical_depth < 3., "uninformative test: {}", optical_depth);
        let v = Volume {
            bounds: BBox::new(Vector3::new(-3500., 1000., -4300.), Vector3::new(4500., 2000., 3700.)),
            density: Box::new(l),
        };
        // Stop at `to`: tracking beyond it would also count cloud past the end.
        let r = Ray { ro: from, rd: to - from };
        let n = 20_000;
        let through = (0..n).filter(|_| v.intersects(&r).map_or(true, |i| i.dist >= (to - from).norm())).count() as f64 / n as f64;
        assert!((through - (-optical_depth).exp()).abs() < 0.012, "{} vs {}", through, (-optical_depth).exp());
    }

    #[test]
    fn test_worley_texture_matches_worley_noise_and_tiles() {
        // At texel corners, before interpolation, it is ordinary Worley noise (within its first
        // tile, where the wrapped cell indices are the cells themselves).
        let w = WorleyTexture::new(3);
        let reference = crate::noise::WorleyNoise::new(1., 3);
        for i in 0..2000 {
            let (x, y, z) = ((i * 7) % 60, (i * 13) % 60, (i * 29) % 60);
            let p = Vector3::new(x as f64, y as f64, z as f64) / 4.;
            // Away from the tile's edges, where the neighbours wrap.
            if p.iter().all(|v| *v >= 1. && *v < 14.) {
                assert!((w.at(&p) - reference.noise(p.x, p.y, p.z)).abs() < 1e-6, "at {}", p);
            }
        }
        let p = Vector3::new(1.37, -2.71, 5.05);
        let period = WorleyTexture::PERIOD as f64;
        assert!((w.at(&p) - w.at(&(p + Vector3::new(period, -2. * period, period)))).abs() < 1e-9);
    }

    #[test]
    fn test_height_profiles() {
        // Zero at the base, and above each type's top.
        for t in [0., 0.25, 0.5, 1.] {
            assert_eq!(height_profile(0., t), 0.);
        }
        assert_eq!(height_profile(0.5, 0.), 0.);
        assert!(height_profile(0.5, 1.) > 0.9);
        assert!(height_profile(0.1, 0.) > 0.5);
    }
}

