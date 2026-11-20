//! A layer of procedural clouds by the "Nubis" method: a density field for `volume::Volume`,
//! after Andrew Schneider and Nathan Vos, "The Real-time Volumetric Cloudscapes of Horizon: Zero
//! Dawn", Advances in Real-Time Rendering, SIGGRAPH 2015, and Schneider's "Nubis" follow-ups (Advances 2017,
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

// The recipe itself (the height profiles, the noise's remapping, the shape and its erosion) is
// in `shared::clouds`, generic over the float it computes in, so that the GPU's shader compiles
// the same functions at f32 (see that crate). Here are the renderer's f64 names for them.

/// (v - lo) / (hi - lo), clamped to [0, 1]; a step at `hi` if the range is empty, with `hi`
/// itself below it, so a threshold of 1 on a value in [0, 1] (no coverage) passes nothing.
pub(crate) fn remap(v: f64, lo: f64, hi: f64) -> f64 {
    shared::remap(v, lo, hi)
}

pub(crate) fn smoothstep(lo: f64, hi: f64, v: f64) -> f64 {
    shared::smoothstep(lo, hi, v)
}

pub(crate) fn lerp(a: f64, b: f64, t: f64) -> f64 {
    shared::lerp(a, b, t)
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

    /// The map as the shared code reads it (`shared::clouds::WeatherGrid`), its corners and
    /// maxima in tables at `corners` and `maxima`.
    fn grid(&self, corners: u32, maxima: u32) -> shared::clouds::WeatherGrid<f64> {
        shared::clouds::WeatherGrid { x: self.origin.x, z: self.origin.y, texel: self.texel, n: self.n as u32, corners, maxima }
    }

    fn at(&self, x: f64, z: f64) -> f64 {
        self.grid(0, 0).at(self, x, z)
    }
}

/// The corners, alone, from 0.
impl shared::Tables<f64> for WeatherMap {
    fn value(&self, i: u32) -> f64 {
        self.corners[i as usize]
    }

    fn index(&self, _i: u32) -> u32 {
        0
    }
}

/// Fractal Perlin noise is in [0, 1] but rarely strays far from 0.5; stretch it to use the range.
pub(crate) fn stretched(n: f64) -> f64 {
    shared::clouds::stretched(n)
}

/// Worley noise (distance to the nearest feature point, one point per unit cell) precomputed
/// over a tile of `PERIOD` cells a side, which repeats, and interpolated trilinearly between
/// texels: Nubis's noise textures. Searching the 27 neighbouring cells took two thirds of the
/// render time when done for every sample; a lookup reads eight texels.
pub(crate) struct WorleyTexture {
    texels: Vec<f32>,
}

impl WorleyTexture {
    const PERIOD: usize = shared::clouds::WORLEY_PERIOD as usize;
    const PER_CELL: usize = shared::clouds::WORLEY_PER_CELL as usize;
    const N: usize = shared::clouds::WORLEY_N as usize;

    pub(crate) fn new(seed: u32) -> WorleyTexture {
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
    pub(crate) fn at(&self, p: &Vector3<f64>) -> f64 {
        shared::clouds::worley(self, 0, p.x, p.y, p.z)
    }

    /// The texels, for whoever lays them out elsewhere (the GPU).
    pub(crate) fn texels(&self) -> &[f32] {
        &self.texels
    }
}

impl shared::Tables<f64> for WorleyTexture {
    fn value(&self, i: u32) -> f64 {
        self.texels[i as usize] as f64
    }

    fn index(&self, _i: u32) -> u32 {
        0
    }
}

/// Inverted Worley noise (high at feature points, falling away between them), in three octaves:
/// round, billowing cells.
pub(crate) fn worley_fbm(w: &WorleyTexture, p: &Vector3<f64>) -> f64 {
    shared::clouds::worley_fbm(w, 0, p.x, p.y, p.z)
}

/// A layer's tables as its shared recipe reads them: the shape noise's permutation, then the
/// Worley texture, then (for a flat layer) the weather map's corners and its texels' maxima.
/// The weather map is built when first read, as ever.
struct LayerTables<'a> {
    layer: &'a NubisCloudLayer,
    cells: &'a WorleyTexture,
}

/// Where the Worley texture begins, after the permutation's 512 entries, and the weather map's
/// corners, after its texels.
const WORLEY_AT: u32 = 512;
const CORNERS_AT: u32 = WORLEY_AT + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N;

impl shared::Tables<f64> for LayerTables<'_> {
    fn value(&self, i: u32) -> f64 {
        if i < CORNERS_AT {
            return self.cells.texels[(i - WORLEY_AT) as usize] as f64;
        }
        let map = self.layer.weather_map();
        let k = (i - CORNERS_AT) as usize;
        if k < map.corners.len() { map.corners[k] } else { map.texel_max[k - map.corners.len()] }
    }

    fn index(&self, i: u32) -> u32 {
        self.layer.shape.permutation()[i as usize] as u32
    }
}

pub struct NubisCloudLayer {
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

impl NubisCloudLayer {
    pub fn new(base: f64, top: f64, center: Vector2<f64>, extent: f64, seed: u64) -> NubisCloudLayer {
        assert!(top > base, "The cloud layer's top ({}) must be above its base ({})", top, base);
        NubisCloudLayer {
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
            weather: PerlinNoise::shuffled(seed.wrapping_mul(2).wrapping_add(1)),
            shape: PerlinNoise::shuffled(seed.wrapping_mul(2)),
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

    /// The layer as data, for the GPU's shader, which computes its density and walks through
    /// it with the same functions (`pathtrace::load_nubis`, which has the layout).
    pub fn export(&self) -> Vec<f32> {
        let map = self.weather_map();
        let mut out = vec![
            self.base as f32, self.top as f32, self.coverage as f32, self.cloud_type as f32,
            self.shape_size as f32, self.detail_size as f32, self.erosion as f32, self.max_density as f32,
            map.origin.x as f32, map.origin.y as f32, map.texel as f32, map.n as f32,
            (self.center.x - self.extent) as f32, (self.center.y - self.extent) as f32,
            (self.center.x + self.extent) as f32, (self.center.y + self.extent) as f32,
        ];
        out.extend(self.shape.permutation().iter().map(|&p| p as f32));
        out.extend(self.cells().texels());
        out.extend(map.corners.iter().map(|&c| c as f32));
        out.extend(map.texel_max.iter().map(|&c| c as f32));
        out
    }

    /// `fraction_at` with the weather given, for tests of the shader's copy of it.
    #[cfg(test)]
    pub(crate) fn fraction_for_test(&self, altitude: f64, weather: f64, p: &Vector3<f64>) -> f64 {
        self.fraction_at(altitude, || weather, p)
    }

    /// The layer's recipe, as the shared code takes it (`shared::clouds::Nubis`).
    fn recipe(&self) -> shared::clouds::Nubis<f64> {
        shared::clouds::Nubis {
            base: self.base, top: self.top, coverage: self.coverage, cloud_type: self.cloud_type,
            shape_size: self.shape_size, detail_size: self.detail_size, erosion: self.erosion,
            perm: 0, worley: WORLEY_AT,
        }
    }

    /// The layer as a medium (`shared::clouds::NubisField`), flat, with its weather map.
    fn field(&self) -> shared::clouds::NubisField<f64> {
        let map = self.weather_map();
        shared::clouds::NubisField {
            recipe: self.recipe(), max_density: self.max_density,
            weather: map.grid(CORNERS_AT, CORNERS_AT + map.corners.len() as u32),
        }
    }

    /// The highest any cloud can be over weather of at most `weather`, or None if none can be
    /// (`shared::clouds::NubisField::column_top`, which needs no weather map).
    fn column_top(&self, weather: f64) -> Option<f64> {
        let unmapped = shared::clouds::WeatherGrid { x: 0., z: 0., texel: 1., n: 1, corners: 0, maxima: 0 };
        shared::clouds::NubisField { recipe: self.recipe(), max_density: self.max_density, weather: unmapped }.column_top(weather)
    }

    /// Cloud density as a fraction of `max_density`, in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        self.fraction_at(p.y, || self.weather_map().at(p.x, p.z), p)
    }

    /// The cloud recipe (`shared::clouds::Nubis::fraction`), given where its inputs come from:
    /// the height `altitude`, the weather map's value over the point (asked for only if
    /// needed), and the position `p` at which to evaluate the shape and detail noise. Flat
    /// layers take these from flat coordinates, global ones from the planet (`GlobalNubis`).
    fn fraction_at(&self, altitude: f64, weather: impl FnOnce() -> f64, p: &Vector3<f64>) -> f64 {
        let recipe = self.recipe();
        let Some(h) = recipe.height(altitude) else { return 0. };
        let tables = LayerTables { layer: self, cells: self.cells() };
        recipe.fraction(&tables, h, weather(), p.x, p.y, p.z)
    }
}

impl DensityField for NubisCloudLayer {
    fn export(&self) -> Option<crate::volume::FieldExport> {
        Some(crate::volume::FieldExport::Nubis { record: NubisCloudLayer::export(self), radius: 0. })
    }

    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.field().density(&LayerTables { layer: self, cells: self.cells() }, p.x, p.y, p.z)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }

    /// Within one texel of the weather map, clouds lie only below `column_top` of its strongest
    /// corner (`shared::clouds::NubisField::majorant_from`).
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        self.field().majorant_from(&LayerTables { layer: self, cells: self.cells() }, p.x, p.y, p.z, d.x, d.y, d.z)
    }
}

/// # Nubis clouds over a whole planet
///
/// The same clouds (`NubisCloudLayer`'s profiles, Perlin-Worley shapes and erosion), with
/// heights as altitudes above the planet and no flat coordinates anywhere: the weather map is a
/// `CubeMap` over the globe, sampled from solid noise at each texel corner's direction, so it
/// has no poles or seams; the shapes and detail are solid noise at the point's position
/// relative to the planet's centre. Each texel's largest corner bounds the clouds above it, as
/// in the flat layer: up to the sphere of their highest possible top, and nothing over them
/// beyond it, with exits from the column (four planes) and to the sphere exact.
///
/// With `climate`, coverage follows the planet's broad bands of weather by latitude (about the
/// `pole`): cloudy along the equator (the intertropical convergence zone), clear over the
/// subtropical highs near 25°, cloudy again along the storm tracks of 40-65°.
pub struct GlobalNubis {
    pub layer: NubisCloudLayer,
    pub map_size: usize,
    pub climate: bool,
    /// Unit vector of the planet's axis (north).
    pub pole: Vector3<f64>,
    pub planet_radius: f64,
    map: OnceLock<crate::planet::CubeMap>,
}

impl GlobalNubis {
    pub fn new(layer: NubisCloudLayer, planet_radius: f64) -> GlobalNubis {
        GlobalNubis { layer, map_size: 1024, climate: true, pole: Vector3::z(), planet_radius, map: OnceLock::new() }
    }

    /// How much more or less cloud there is at latitude `lat` (radians), by its bands.
    fn climate_factor(&self, lat: f64) -> f64 {
        let deg = lat.to_degrees().abs();
        let band = |centre: f64, width: f64| (-((deg - centre) / width).powi(2)).exp();
        0.85 + 0.45 * band(0., 8.) - 0.4 * band(25., 9.) + 0.35 * band(52., 12.)
    }

    fn map(&self) -> &crate::planet::CubeMap {
        self.map.get_or_init(|| crate::geometry::build_apart(|| {
            let scale = self.planet_radius / self.layer.weather_size;
            crate::planet::CubeMap::new(self.map_size, |d| {
                let q = d * scale;
                let w = stretched(self.layer.weather.fbm(q.x, q.y, q.z, 6, 0.5, 2.0));
                if self.climate {
                    (w * self.climate_factor(d.dot(&self.pole).clamp(-1., 1.).asin())).clamp(0., 1.)
                } else {
                    w
                }
            })
        }))
    }
}

impl crate::planet::PlanetField for GlobalNubis {
    fn density(&self, at: &crate::planet::SurfacePoint) -> f64 {
        self.layer.max_density * self.layer.fraction_at(at.altitude, || self.map().at(&at.up), &at.position)
    }

    fn majorant(&self) -> f64 {
        self.layer.max_density
    }

    fn majorant_from(&self, at: &crate::planet::SurfacePoint, _p: &Vector3<f64>, d: &Vector3<f64>,
                     planet: &crate::planet::Planet) -> (f64, f64) {
        let map = self.map();
        // Classify by a point a centimetre along the ray, so one on a texel's edge (or the
        // clouds' top) counts as where it is going; otherwise it finds the edge it is on, no
        // distance away, again and again.
        let ahead = at.position + d * 0.01;
        let (face, i, j) = map.texel(&ahead.normalize());
        let edge = map.cube.exit(&at.position, d, face, i, j);
        match self.layer.column_top(map.texel_max(face, i, j)) {
            None => (0., edge),
            Some(top) => {
                let to_top = crate::planet::to_sphere(&at.position, d, planet.radius + top);
                let majorant = if ahead.norm() - planet.radius > top { 0. } else { self.layer.max_density };
                (majorant, edge.min(to_top))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(seed: u64) -> NubisCloudLayer {
        NubisCloudLayer::new(1000., 2000., Vector2::new(12000., 12000.), 12500., seed)
    }

    /// Fraction of a grid of points through the layer, at `y`, that are cloud.
    fn cover_at(l: &NubisCloudLayer, y: f64) -> f64 {
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
        let row = |l: NubisCloudLayer| -> Vec<f64> {
            (0..400).map(|i| l.fraction(&Vector3::new(i as f64 * 37., 1300., 0.))).collect()
        };
        let p = row(layer(5));
        assert_ne!(p, row(layer(6)));
        assert_eq!(p, row(layer(5)));
    }

    /// A small, thick-coverage patch of cloud, quick to build.
    fn patch() -> NubisCloudLayer {
        let mut l = NubisCloudLayer::new(1000., 2000., Vector2::new(500., -300.), 4000., 8);
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
        let v = Volume::new(
            BBox::new(Vector3::new(-3500., 1000., -4300.), Vector3::new(4500., 2000., 3700.)),
            std::sync::Arc::new(l),
        );
        // Stop at `to`: tracking beyond it would also count cloud past the end.
        let r = Ray { ro: from, rd: to - from };
        let n = 20_000;
        let through = (0..n).filter(|_| v.intersects(&r).map_or(true, |i| i.dist >= (to - from).norm())).count() as f64 / n as f64;
        assert!((through - (-optical_depth).exp()).abs() < 0.012, "{} vs {}", through, (-optical_depth).exp());
        // And ratio tracking, for shadow rays.
        let ratio = (0..n).map(|_| v.transmittance(&r, 0., (to - from).norm())).sum::<f64>() / n as f64;
        assert!((ratio - (-optical_depth).exp()).abs() < 0.006, "{} vs {}", ratio, (-optical_depth).exp());
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
    fn test_global_clouds_bounded_and_banded() {
        use crate::planet::{Planet, PlanetField, GlobalMapping};
        use crate::volume::DensityField;
        let mut layer = NubisCloudLayer::new(1500., 3000., Vector2::zeros(), 1., 3);
        layer.weather_size = 400_000.;
        layer.coverage = 0.6;
        let mut g = GlobalNubis::new(layer, 6_360_000.);
        g.map_size = 128;
        let planet = Planet { radius: 6_360_000. };
        let field = GlobalMapping { planet, field: std::sync::Arc::new(g) };
        // The local majorant bounds the density along random rays through the layer.
        let mut rng = crate::geometry::Pcg32::new(4, 0);
        for _ in 0..1500 {
            let up = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let p = planet.center() + up * (planet.radius + 1000. + rng.next_f64() * 2500.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = field.majorant_from(&p, &d);
            for k in 0..20 {
                let q = p + d * (len.min(50_000.) * k as f64 / 20.);
                assert!(field.density(&q) <= m + 1e-12, "{} > {}", field.density(&q), m);
            }
        }
        // On a texel's edge, a ray across it gets a step to the far side of the next texel, not
        // none (which stalled tracking).
        let map_field = &field.field;
        let corner = crate::planet::CubeSphere { size: 128 }.direction(0, 64., 70.);
        let on_edge = planet.center() + corner * (planet.radius + 2000.);
        for towards in [62., 66.] {
            // Into the texel on either side of the edge.
            let across = crate::planet::CubeSphere { size: 128 }.direction(0, towards, 70.) - corner;
            let d = (across - corner * corner.dot(&across)).normalize();
            let (m, len) = map_field.majorant_from(&planet.surface_point(&on_edge), &on_edge, &d, &planet);
            assert!(len > 1000., "{}", len);
            for k in 0..50 {
                let q = on_edge + d * (len.min(100_000.) * k as f64 / 50.);
                assert!(field.density(&q) <= m + 1e-12, "{} > {} towards {}", field.density(&q), m, towards);
            }
        }
        // Climate: more cloud along the equator and the storm tracks than over the subtropics.
        let g = match &field.field { f => f };
        let cover = |lat: f64| {
            let n = 4000;
            (0..n).filter(|k| {
                let lon = *k as f64 / n as f64 * std::f64::consts::TAU;
                let (l, o) = (lat.to_radians(), lon);
                let up = Vector3::new(l.cos() * o.cos(), l.cos() * o.sin(), l.sin());
                let p = planet.center() + up * (planet.radius + 1800.);
                g.density(&planet.surface_point(&p)) > 0.
            }).count() as f64 / n as f64
        };
        let (equator, subtropics, storms) = (cover(0.), cover(25.), cover(52.));
        assert!(equator > subtropics && storms > subtropics, "{} {} {}", equator, subtropics, storms);
    }

    #[test]
    fn test_height_profiles() {
        // Zero at the base, and above each type's top.
        for t in [0., 0.25, 0.5, 1.] {
            assert_eq!(shared::clouds::height_profile(0., t), 0.);
        }
        assert_eq!(shared::clouds::height_profile(0.5, 0.), 0.);
        assert!(shared::clouds::height_profile(0.5, 1.) > 0.9);
        assert!(shared::clouds::height_profile(0.1, 0.) > 0.5);
    }
}

