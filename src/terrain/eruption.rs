//! # An eruption in progress
//!
//! A landscape with `"eruption": true` shows its active volcanoes erupting: each hotspot shield
//! still building (`hotspot::activity`) holds a lava lake in its summit crater, and lava runs
//! down from the summit and from a rift zone in open channels, found on the landscape's own
//! surface by the way down that is steepest (perturbed, as `volcano::erupt`'s flows are).
//!
//! Molten basalt is at 1,100-1,200 °C, a blackbody's dull orange: its luminance is about
//! 1,000 cd/m² at 1,130 °C (a filament at 2,800 K is about 10⁷; the visible part of Planck's
//! law falls as exp(-25,900 K / T)), so by day it glows faintly against sunlit rock, and at
//! night it lights its surroundings. Little of it is open: a skin of black crust forms within
//! seconds and is cracked by the moving lava, the cracks hotter than the crust. In a channel's
//! core the lava runs open in places, hotter towards the vent; on a lake the crust is plates
//! with incandescent seams between them (Kīlauea's Halemaʻumaʻu, Nyiragongo).
//!
//! The lava's material (`ActiveLava`) is the landscape's own ground cover, dark crust, which
//! also emits: by `Eruption::heat` (0 off the lava, 1 at a channel's core or on a lake), with
//! a pattern of cracks; it isn't sampled as a light (hits on it count).

use crate::color::Color;
use crate::geometry::Pcg32;
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::procedural::nubis_clouds::smoothstep;
use crate::ray::Ray;
use crate::scene::Scene;
use std::collections::HashMap;

/// Luminance of a blackbody at `kelvin`, cd/m², in Wien's approximation over the visible:
/// scaled from a filament's 10⁷ cd/m² at 2,800 K.
pub fn luminance(kelvin: f64) -> f64 {
    1e7 * (25_900. / 2_800. - 25_900. / kelvin.max(300.)).exp()
}

/// The scene's radiance per cd/m² under a sun of `sun_intensity`, on the scale the scene's
/// other physically bright things share (`procedural::lightning::SUN_ILLUMINANCE`: a sun of
/// intensity 1 gives 1.27·10⁵ lux; `SceneFile::lightning_follows_sun` passes the scene's sun's
/// intensity to the landscape).
pub fn radiance_per_nit(sun_intensity: f64) -> f64 {
    sun_intensity / crate::procedural::lightning::SUN_ILLUMINANCE
}

/// A lava channel: its course (scene points, a few tens of metres apart, from the vent down)
/// and its width at each, metres.
#[derive(Clone, Debug)]
pub struct Channel {
    pub points: Vec<Vector3<f64>>,
    pub widths: Vec<f64>,
}

/// The eruption's lava: lakes (a centre and a radius) and channels, indexed by where they are.
pub struct Eruption {
    pub lakes: Vec<(Vector3<f64>, f64)>,
    pub channels: Vec<Channel>,
    /// The surface wind its plumes are carried by: towards (x, z) in the scene's plan, and its
    /// speed, m/s.
    pub wind: (crate::na::Vector2<f64>, f64),
    /// Plan cells (`CELL` metres in scene x and z) and the channel segments crossing each.
    cells: HashMap<(i64, i64), Vec<(usize, usize)>>,
}

const CELL: f64 = 100.;

/// `Eruption::heat`'s place along a channel for a lake.
pub const LAKE: f64 = -1.;

fn key(x: f64, z: f64) -> (i64, i64) { ((x / CELL).floor() as i64, (z / CELL).floor() as i64) }

impl Eruption {
    pub fn new(lakes: Vec<(Vector3<f64>, f64)>, channels: Vec<Channel>) -> Eruption {
        let mut cells: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new();
        for (c, ch) in channels.iter().enumerate() {
            for k in 0..ch.points.len().saturating_sub(1) {
                let (a, b) = (ch.points[k], ch.points[k + 1]);
                let w = ch.widths[k].max(ch.widths[k + 1]);
                let (lo, hi) = (key(a.x.min(b.x) - w, a.z.min(b.z) - w), key(a.x.max(b.x) + w, a.z.max(b.z) + w));
                for i in lo.0..=hi.0 {
                    for j in lo.1..=hi.1 { cells.entry((i, j)).or_default().push((c, k)); }
                }
            }
        }
        Eruption { lakes, channels, wind: (crate::na::Vector2::new(1., 0.), 7.), cells }
    }

    /// Its plumes carried by a wind towards `towards` (the scene's plan) at `speed` m/s.
    pub fn with_wind(mut self, towards: crate::na::Vector2<f64>, speed: f64) -> Eruption {
        self.wind = (towards, speed);
        self
    }

    /// The gas plume over each lava lake (`procedural::plume`): a lake loses about 15 kW per
    /// square metre of its surface (crust and open lava; Halemaʻumaʻu's lake, about a gigawatt),
    /// its plume thick with steam and sulphate haze (extinction 0.01 per metre).
    pub fn plumes(&self, seed: u64) -> Vec<crate::procedural::plume::Plume> {
        self.lakes.iter().enumerate().map(|(k, &(c, r))| {
            let heat = 15e3 * std::f64::consts::PI * r * r;
            crate::procedural::plume::Plume::new(c, r, heat, self.wind.0, self.wind.1, 0.01, 15_000., seed.wrapping_add(k as u64))
        }).collect()
    }

    /// How hot the lava at scene point `p` is, 0 (none) to 1 (a channel's open core, or a
    /// lake), and how far down its channel it is, 0 to 1 (`LAKE` on a lake).
    pub fn heat(&self, p: &Vector3<f64>) -> Option<(f64, f64)> {
        for &(c, r) in &self.lakes {
            let d = (p.x - c.x).hypot(p.z - c.z);
            if d < r { return Some((1. - smoothstep(0.85 * r, r, d), LAKE)); }
        }
        let mut best: Option<(f64, f64)> = None;
        for &(c, k) in self.cells.get(&key(p.x, p.z)).into_iter().flatten() {
            let ch = &self.channels[c];
            let (a, b) = (ch.points[k], ch.points[k + 1]);
            let ab = b.xz() - a.xz();
            let t = ((p.xz() - a.xz()).dot(&ab) / ab.norm_squared().max(1e-9)).clamp(0., 1.);
            let d = (p.xz() - (a.xz() + ab * t)).norm();
            let half = 0.5 * (ch.widths[k] * (1. - t) + ch.widths[k + 1] * t);
            if d < half {
                let heat = 1. - smoothstep(0.3 * half, half, d);
                let along = (k as f64 + t) / (ch.points.len() - 1).max(1) as f64;
                if best.is_none_or(|(h, _)| heat > h) { best = Some((heat, along)); }
            }
        }
        best
    }
}

/// A channel from scene point `start` down the steepest way over the landscape's surface
/// (`height` above the sea at scene x, z; the channel's points keep it as their y), in steps of `step` metres perturbed by up to `roughness`, for up to
/// `length` metres or to the sea, from `seed`; `width` metres wide at the vent, spreading to
/// three times that in a fan at its front. Lava ponds in a hollow and spills over its lowest
/// side, so a channel crosses pits up to `SPILL` metres deep; it never turns back on itself.
pub fn channel(start: Vector3<f64>, height: &dyn Fn(f64, f64) -> f64, length: f64, width: f64, seed: u64) -> Channel {
    let step = 40.;
    let mut rng = Pcg32::new(seed, 11);
    let mut p = Vector3::new(start.x, height(start.x, start.z), start.z);
    let mut points = vec![p];
    let mut run = 0.;
    let mut heading: Option<Vector3<f64>> = None;
    while run < length {
        let mut next: Option<(Vector3<f64>, f64)> = None;
        for k in 0..12 {
            let a = k as f64 * std::f64::consts::TAU / 12.;
            let (x, z) = (p.x + step * a.cos(), p.z + step * a.sin());
            if heading.is_some_and(|d| d.x * a.cos() + d.z * a.sin() < 0.) { continue; }
            let h = height(x, z);
            let drop = p.y - h + 3. * (rng.next_f64() - 0.5);
            if drop > -SPILL && next.is_none_or(|(_, d)| drop > d) { next = Some((Vector3::new(x, h, z), drop)); }
        }
        let Some((q, _)) = next else { break };
        heading = Some((q - p).normalize());
        p = q;
        points.push(p);
        run += step;
        if p.y < 0. { break; }
    }
    let n = points.len();
    let widths = (0..n).map(|k| {
        let along = k as f64 / (n - 1).max(1) as f64;
        width * (1. + 0.5 * along + 1.5 * smoothstep(0.85, 1., along))
    }).collect();
    Channel { points, widths }
}

/// The deepest hollow a channel spills across, metres.
const SPILL: f64 = 15.;

/// The lava's material: the ground beneath (`crust`, the landscape's own ground cover) that
/// also glows where `eruption` has lava.
pub struct ActiveLava {
    pub eruption: std::sync::Arc<Eruption>,
    pub crust: Box<dyn MaterialModel + Sync + Send>,
    pub noise: crate::noise::PerlinNoise,
    /// The scene's radiance per cd/m² (`radiance_per_nit`).
    pub per_nit: f64,
}

impl ActiveLava {
    /// The lava's radiance at `p`: crust with glowing cracks, and open lava in a channel's
    /// core and between a lake's plates, hottest by the vent.
    pub fn glow(&self, p: &Vector3<f64>) -> Color {
        let Some((heat, along)) = self.eruption.heat(p) else { return Color::black() };
        let n = |s: f64, o: f64| self.noise.fbm(p.x / s + o * 13., p.y / s, p.z / s, 3, 0.5, 2.);
        if along == LAKE {
            // A lake's crust: plates metres to tens of metres across, carried on the
            // convecting lava and pulled apart at seams that glow at its full heat; open
            // lava where the crust founders against the crater's wall.
            let core = 1_430.;
            let seams = 1. - smoothstep(0., 0.025, (n(18., 0.53) - 0.5).abs());
            let fine = 1. - smoothstep(0., 0.03, (n(3., 0.31) - 0.5).abs());
            let rim = 1. - heat;
            let open = smoothstep(0.3, 0.9, rim + 0.4 * (n(25., 0.77) - 0.5));
            let glow = open * luminance(core) + (1. - open) * (seams * luminance(core - 40.) + 0.3 * fine * luminance(core - 180.));
            return crate::spectrum::Emission::blackbody(core).radiance() * (glow * heat.max(0.3) * self.per_nit);
        }
        // Cracks in the crust: the noise's narrow valleys, metres apart, a decimetre wide.
        let cracks = 1. - smoothstep(0., 0.04, (n(2.5, 0.31) - 0.5).abs());
        // Open lava where the crust is torn: most at the core, in patches.
        let open = heat * smoothstep(0.45, 0.6, n(12., 0.77) + 0.3 * heat - 0.15);
        let core = 1_420. - 120. * along;
        let glow = open * luminance(core) + (1. - open) * heat.max(0.2) * cracks * 0.6 * luminance(core - 120.);
        let emission = crate::spectrum::Emission::blackbody(core);
        emission.radiance() * (glow * self.per_nit)
    }
}

impl MaterialModel for ActiveLava {
    fn scatter(&self, r: &Ray, i: &Intersection, s: &Scene) -> ScatteredRay {
        let mut sr = self.crust.scatter(r, i, s);
        sr.emit = sr.emit + self.glow(&i.point);
        sr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lava_glows_by_its_temperature() {
        // 1,130 °C: about a thousand cd/m², a hundredth of what the sun lights white rock to.
        let l = luminance(1_403.);
        assert!(l > 500. && l < 2_000., "{} cd/m²", l);
        assert!(luminance(1_300.) < 0.5 * l);
    }

    #[test]
    fn test_a_channel_runs_downhill_and_is_hot_along_it() {
        // A cone: the channel runs away from its summit.
        let height = |x: f64, z: f64| 2000. - 0.2 * x.hypot(z);
        let c = channel(Vector3::new(10., 0., 0.), &height, 3000., 30., 1);
        assert!(c.points.len() > 50);
        assert!(c.points.windows(2).all(|w| w[1].y < w[0].y));
        let e = Eruption::new(vec![(Vector3::new(0., 2000., 0.), 100.)], vec![c.clone()]);
        assert_eq!(e.heat(&Vector3::new(0., 2000., 0.)).unwrap().0, 1.);
        let mid = c.points[c.points.len() / 2];
        let (h, along) = e.heat(&mid).unwrap();
        assert!(h > 0.9 && (along - 0.5).abs() < 0.05, "{} {}", h, along);
        assert!(e.heat(&(mid + Vector3::new(500., 0., 500.))).is_none());
        assert_eq!(e.heat(&Vector3::new(0., 2000., 0.)).unwrap().1, LAKE);
    }

    #[test]
    fn test_a_channel_spills_across_a_hollow() {
        // A slope down x with a 10 m pit across it: the lava fills it and carries on.
        let height = |x: f64, _z: f64| 1000. - 0.1 * x - 10. * (1. - ((x - 1000.) / 100.).powi(2)).max(0.);
        let c = channel(Vector3::new(0., 0., 0.), &height, 3000., 30., 3);
        assert!(c.points.last().unwrap().x > 2000., "stopped at {}", c.points.last().unwrap().x);
    }
}
