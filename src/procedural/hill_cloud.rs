//! Hill cloud: low cloud forced up over high ground, capping the summits and hiding the hills'
//! tops. A density field for `volume::Volume`, over a `procedural::terrain` heightfield.
//!
//! On a raw, damp day the air near the ground is close to saturation: a little lift, a few
//! hundred metres, cools it to its dew point. Over the plains nothing lifts it, but wind blowing
//! onto hills is forced up and over them, and where the hills are high enough it condenses. The
//! hills disappear into cloud from a certain height up; the cloud hugs the summits and windward
//! slopes, and trails off downwind, evaporating as the air sinks again in the lee (hill fog, or
//! orographic stratus; a cap cloud over a single summit).
//!
//! The field takes the streamlines of the air to follow the ground, the displacement dying away
//! with height above it: air at `y` over ground raised `g` above the plains has been lifted by
//! η = g e^(-(y - g)/decay). It is cloud where that lift exceeds `deficit`, the lift that
//! saturates it, which varies a little from place to place (`humidity_variation`). So the cloud
//! sits on hills taller than the deficit, `decay` ln(g / deficit) deep over a summit of height g,
//! and none forms over lower ground. Air that crossed a summit a little upwind keeps some of its
//! lift (`trail`), so the cloud streams off downwind of the crests, at their height. The cloud
//! water, and so the density, grows with the excess lift; Worley detail wears the edges ragged.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::{remap, smoothstep, stretched, worley_fbm, WorleyTexture};
use crate::procedural::terrain::Terrain;
use crate::volume::DensityField;

/// The ground's height on a grid, interpolated bilinearly: the terrain's relief is costly.
struct HeightGrid {
    origin: Vector2<f64>,
    texel: f64,
    n: usize,
    heights: Vec<f64>,
    max: f64,
}

impl HeightGrid {
    fn new(t: &Terrain, n: usize) -> HeightGrid {
        let n = n.max(2);
        let origin = t.center - Vector2::new(t.extent, t.extent);
        let texel = 2. * t.extent / (n - 1) as f64;
        let heights: Vec<f64> = (0..n * n).map(|k| t.relief_at(origin.x + (k % n) as f64 * texel, origin.y + (k / n) as f64 * texel)).collect();
        let max = heights.iter().cloned().fold(f64::MIN, f64::max);
        HeightGrid { origin, texel, n, heights, max }
    }

    fn at(&self, x: f64, z: f64) -> f64 {
        let u = ((x - self.origin.x) / self.texel).clamp(0., (self.n - 1) as f64);
        let v = ((z - self.origin.y) / self.texel).clamp(0., (self.n - 1) as f64);
        let (i, j) = ((u.floor() as usize).min(self.n - 2), (v.floor() as usize).min(self.n - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let h = |i: usize, j: usize| self.heights[j * self.n + i];
        let a = h(i, j) + (h(i + 1, j) - h(i, j)) * fu;
        let b = h(i, j + 1) + (h(i + 1, j + 1) - h(i, j + 1)) * fu;
        a + (b - a) * fv
    }
}

pub struct HillCloud {
    /// The lift (m) that saturates the air, and how much it varies (0 to 1) over `humidity_size`.
    pub deficit: f64,
    pub humidity_variation: f64,
    pub humidity_size: f64,
    /// Over what height (m) the ground's lift of the air dies away above it.
    pub decay: f64,
    /// Extinction where the air is lifted twice the deficit, and more.
    pub max_density: f64,
    /// The wind ([x, z], unit), and how far downwind of a crest (m) the cloud streams.
    pub wind_direction: Vector2<f64>,
    pub trail: f64,
    /// The size of the ragged detail, and how much it wears the edges (0 to 1).
    pub detail_size: f64,
    pub raggedness: f64,
    ground: HeightGrid,
    /// The terrain's region: [x, z] middle and half-width.
    center: Vector2<f64>,
    extent: f64,
    seed: u64,
    noise: PerlinNoise,
    cells: std::sync::OnceLock<WorleyTexture>,
}

/// Samples of the air's path upwind: how far back (as a fraction of `trail`), and how much of
/// the lift it keeps.
const UPWIND: [(f64, f64); 4] = [(0., 1.), (0.33, 0.9), (0.67, 0.75), (1., 0.55)];

impl HillCloud {
    /// Over `terrain`, its heights sampled on a grid `resolution` a side.
    pub fn new(terrain: &Terrain, resolution: usize, seed: u64) -> HillCloud {
        HillCloud {
            deficit: 350.,
            humidity_variation: 0.25,
            humidity_size: 6000.,
            decay: 250.,
            max_density: 0.02,
            wind_direction: Vector2::new(1., 0.),
            trail: 1500.,
            detail_size: 150.,
            raggedness: 0.5,
            ground: HeightGrid::new(terrain, resolution),
            center: terrain.center,
            extent: terrain.extent,
            seed,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(13).wrapping_add(7)),
            cells: std::sync::OnceLock::new(),
        }
    }

    fn cells(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed as u32 ^ 0x4111))
    }

    /// The least lift that ever saturates the air.
    fn least_deficit(&self) -> f64 {
        self.deficit * (1. - self.humidity_variation)
    }

    /// The lowest cloud can be: on the lowest ground tall enough to raise it.
    pub fn bottom(&self) -> f64 {
        self.least_deficit()
    }

    /// The highest: over the highest summit, where the deficit is least.
    pub fn top(&self) -> f64 {
        let g = self.ground.max;
        if g <= self.least_deficit() {
            return self.bottom();
        }
        g + self.decay * (g / self.least_deficit()).ln() * 1.1 + self.raggedness * self.detail_size
    }

    /// How far the air at `p` has been lifted, by the ground under it or a little upwind.
    fn lift(&self, p: &Vector3<f64>) -> f64 {
        let ground = self.ground.at(p.x, p.z);
        if p.y < ground {
            return 0.;
        }
        UPWIND.iter().map(|(back, keep)| {
            let q = Vector2::new(p.x, p.z) - self.wind_direction * (back * self.trail);
            let g = self.ground.at(q.x, q.y).max(0.);
            g * keep * (-(p.y - g).max(0.) / self.decay).exp()
        }).fold(0., f64::max)
    }

    fn extinction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        if (Vector2::new(p.x, p.z) - self.center).abs().max() > self.extent {
            return 0.;
        }
        let lift = self.lift(p);
        if lift < self.least_deficit() * (1. - 0.5 * self.raggedness) {
            return 0.;
        }
        let humid = stretched(self.noise.fbm(p.x / self.humidity_size, 0.29, p.z / self.humidity_size, 3, 0.5, 2.));
        let deficit = self.deficit * (1. + self.humidity_variation * (1. - 2. * humid));
        // Ragged edges: the detail lowers or raises the lift needed, by up to half the deficit.
        let w = Vector3::new(p.x, p.y * 1.5, p.z) / self.detail_size;
        let d = worley_fbm(self.cells(), &w);
        let needed = deficit * (1. + self.raggedness * (0.5 - d));
        let excess = (lift - needed) / deficit;
        if excess <= 0. {
            return 0.;
        }
        self.max_density * smoothstep(0., 1., remap(excess, 0., 0.6))
    }
}

impl DensityField for HillCloud {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.extinction(p)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }

    /// Nothing below the bottom or above the top.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let (lo, hi) = (self.bottom(), self.top());
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        if ahead.y < lo {
            return (0., if d.y > 0. { to_y(lo) + 0.01 } else { f64::INFINITY });
        }
        if ahead.y > hi {
            return (0., if d.y < 0. { to_y(hi) + 0.01 } else { f64::INFINITY });
        }
        let exit = if d.y > 0. { to_y(hi) } else if d.y < 0. { to_y(lo) } else { f64::INFINITY };
        (self.max_density, exit.max(0.) + 0.01)
    }

    /// Up the column only where there can be cloud.
    fn column_depths(&self, p: &Vector3<f64>, lo: f64, hi: f64, steps: u32) -> (f64, f64) {
        let (lo, hi) = (lo.max(self.bottom()), hi.min(self.top()));
        if hi <= lo {
            return (0., 0.);
        }
        let step = (hi - lo) / steps as f64;
        let (mut below, mut above) = (0., 0.);
        for k in 0..steps {
            let y = lo + (k as f64 + crate::geometry::rand()) * step;
            let tau = self.extinction(&Vector3::new(p.x, y, p.z)) * step;
            if y < p.y { below += tau } else { above += tau }
        }
        (below, above)
    }

    /// Cloud droplets, about 10 µm across.
    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(5.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn hills() -> Terrain {
        let mut t = Terrain::new(5);
        t.center = Vector2::new(0., 10000.);
        t.extent = 8000.;
        t.peak = Vector2::new(0., 10000.);
        t.peak_height = 650.;
        t.peak_radius = 5000.;
        t.relief = 150.;
        t.planet_radius = None;
        t
    }

    #[test]
    fn test_cloud_caps_high_ground_only() {
        let t = hills();
        let c = HillCloud::new(&t, 256, 1);
        // Just above the summit area: cloud. Just above low ground: none.
        let mut rng = Pcg32::new(3, 0);
        let (mut high, mut high_cloudy, mut low_cloudy) = (0, 0, 0);
        for _ in 0..4000 {
            let (x, z) = (rng.next_f64() * 16000. - 8000., 2000. + rng.next_f64() * 16000.);
            let g = t.relief_at(x, z);
            let cloudy = c.density(&Vector3::new(x, g + 20., z)) > 0.;
            if g > 1.6 * c.deficit {
                high += 1;
                high_cloudy += cloudy as usize;
            } else if g < 0.5 * c.deficit {
                low_cloudy += cloudy as usize;
            }
        }
        assert!(high > 50 && high_cloudy as f64 > 0.7 * high as f64, "{} of {}", high_cloudy, high);
        assert_eq!(low_cloudy, 0);
        // And it dies away with height above the summit.
        let peak = Vector3::new(0., t.relief_at(0., 10000.), 10000.);
        assert!(c.density(&(peak + Vector3::new(0., 30., 0.))) > 0.);
        assert_eq!(c.density(&(peak + Vector3::new(0., 1500., 0.))), 0.);
    }

    #[test]
    fn test_majorant_bounds_density() {
        let c = HillCloud::new(&hills(), 128, 2);
        let mut rng = Pcg32::new(4, 0);
        for _ in 0..5000 {
            let p = Vector3::new(rng.next_f64() * 20000. - 10000., rng.next_f64() * 1500., rng.next_f64() * 20000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = c.majorant_from(&p, &d);
            for k in 0..10 {
                let q = p + d * (len.min(5000.) * k as f64 / 10.);
                assert!(c.density(&q) <= m + 1e-12, "{} > {} at {}", c.density(&q), m, q);
            }
        }
    }

    #[test]
    fn test_cloud_streams_downwind() {
        let t = hills();
        let cloud = |trail: f64| {
            let mut c = HillCloud::new(&t, 256, 3);
            (c.wind_direction, c.trail, c.raggedness, c.humidity_variation) = (Vector2::new(1., 0.), trail, 0., 0.);
            c
        };
        // Cloudy points across the massif: trailing adds cloud, and only downwind of the ground
        // that raised it.
        let count = |c: &HillCloud| (0..20_000).filter(|k| {
            let (x, z) = ((k % 141) as f64 * 100. - 7000., 3000. + (k / 141) as f64 * 100.);
            c.density(&Vector3::new(x, t.relief_at(x, z).max(0.) + 150., z)) > 0.
        }).count();
        let (none, streaming) = (count(&cloud(0.)), count(&cloud(1500.)));
        assert!(none > 0 && streaming as f64 > 1.15 * none as f64, "{} vs {}", streaming, none);
    }
}
