//! Cumulonimbus: the thunderstorm cloud. A density field for `volume::Volume`.
//!
//! A cumulonimbus is a cumulus that has grown through the whole troposphere. Air warmer than its
//! surroundings rises from the cloud base, condensing as it goes, and keeps rising while it is
//! buoyant: up to the *equilibrium level*, where it is no warmer than the air around it, usually
//! near the tropopause. There it spreads out sideways into the anvil (incus), carried downwind by
//! the strong winds aloft. The strongest updrafts carry on past the equilibrium level on their
//! momentum, as a dome (the overshooting top) above the flat anvil, before sinking back into
//! the stable stratosphere. Above about -38 °C every droplet freezes, so the upper tower and the
//! anvil are ice: their outlines go soft and fibrous (capillatus), unlike the hard cauliflower
//! turrets of the liquid tower beneath. New towers grow along a flanking line on the upwind
//! side, each lower than the one before.
//!
//! The field follows that physics in its parameters, not in a simulation:
//!
//! 1. **The updraft.** Parcel theory gives the strongest updraft from the convective available
//!    potential energy, w = sqrt(2 CAPE), less by `efficiency` for entrainment and the weight of
//!    the water (typically about half). It overshoots the equilibrium level (`tropopause`) by
//!    about w / N, N being the stratosphere's buoyancy frequency (`stability`, about 0.02 s⁻¹):
//!    with CAPE 2500 J/kg, about 1.8 km.
//! 2. **The towers.** The main tower, `radius` across at mid-levels, from `base` to the
//!    tropopause, leaning downwind (`lean`) as the wind strengthens with height; and `flanking`
//!    towers stepping down away from it along `flank_angle`. Each is a column, rounded at its
//!    top, whose surface is pushed out by billows (inverted Worley cells, as in
//!    `nubis_clouds`) of three sizes, from `billow_size` down: billows of billows.
//! 3. **Glaciation.** Over a kilometre either side of `glaciation`, the cloud turns to ice: the
//!    billows smooth out into streaks drawn out along the wind, and the extinction falls from
//!    `density` (liquid) to `ice_density`.
//! 4. **The anvil.** A sheet under the tropopause, `anvil_thickness` deep at the tower, carried
//!    `upper_wind` × `age` downwind and spreading sideways at `outflow_speed` as it goes (so its
//!    edges fan out), thinning towards its far end and edges; it reaches a little upwind of the
//!    tower too (the back-sheared anvil). Its top is flat, its edges ragged and fibrous.
//!
//! Rain falling from the core is a separate `precipitation` volume under it.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::{lerp, smoothstep, WorleyTexture};
use crate::volume::DensityField;
use std::sync::OnceLock;

/// One tower: where its axis meets the base, its radius, and its top.
#[derive(Clone, Copy, Debug)]
struct Tower {
    foot: Vector2<f64>,
    radius: f64,
    top: f64,
}

pub struct Cumulonimbus {
    /// Where the main updraft stands ([x, z]), the cloud base and the tropopause (m).
    pub center: Vector2<f64>,
    pub base: f64,
    pub tropopause: f64,
    /// Convective available potential energy (J/kg), the fraction of the parcel's updraft that
    /// is realised, and the stratosphere's buoyancy frequency (s⁻¹).
    pub cape: f64,
    pub efficiency: f64,
    pub stability: f64,
    /// The main tower's radius at mid-levels (m), and how far it leans downwind per metre up.
    pub radius: f64,
    pub lean: f64,
    /// The wind aloft: its direction ([x, z], unit) and speed (m/s); how fast the anvil spreads
    /// sideways (m/s); and how long the storm has been spreading it (s).
    pub wind_direction: Vector2<f64>,
    pub upper_wind: f64,
    pub outflow_speed: f64,
    pub age: f64,
    /// How deep the anvil is where it leaves the tower (m).
    pub anvil_thickness: f64,
    /// How many towers grow along the flanking line, the angle of the line from upwind
    /// (degrees, clockwise seen from above), and how far apart they stand (m).
    pub flanking: usize,
    pub flank_angle: f64,
    pub flank_spacing: f64,
    /// The height around which the cloud turns to ice (m).
    pub glaciation: f64,
    /// Extinction (per m) in the liquid cloud and in the ice.
    pub density: f64,
    pub ice_density: f64,
    /// The largest billows (m) and how much they push the surface out (0 to 1).
    pub billow_size: f64,
    pub billowing: f64,
    /// Over how many metres the density rises inside the surface.
    pub softness: f64,
    towers: Vec<Tower>,
    seed: u64,
    noise: PerlinNoise,
    cells: OnceLock<WorleyTexture>,
}

impl Cumulonimbus {
    pub fn new(seed: u64) -> Cumulonimbus {
        let mut c = Cumulonimbus {
            center: Vector2::zeros(),
            base: 1200.,
            tropopause: 11_500.,
            cape: 2500.,
            efficiency: 0.5,
            stability: 0.02,
            radius: 3500.,
            lean: 0.12,
            wind_direction: Vector2::new(1., 0.),
            upper_wind: 25.,
            outflow_speed: 9.,
            age: 1800.,
            anvil_thickness: 2200.,
            flanking: 4,
            flank_angle: 110.,
            flank_spacing: 0.,
            glaciation: 8000.,
            density: 0.04,
            ice_density: 0.006,
            billow_size: 0.,
            billowing: 1.,
            softness: 40.,
            towers: vec![],
            seed,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(13).wrapping_add(7)),
            cells: OnceLock::new(),
        };
        c.build();
        c
    }

    fn cells(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed as u32 ^ 0xcb))
    }

    /// The strongest updraft (m/s): parcel theory's sqrt(2 CAPE), times `efficiency`.
    pub fn updraft(&self) -> f64 {
        self.efficiency * (2. * self.cape.max(0.)).sqrt()
    }

    /// How far the updraft carries the top above the tropopause: w / N.
    pub fn overshoot(&self) -> f64 {
        self.updraft() / self.stability.max(1e-4)
    }

    /// The largest billows, by default an eighth of the tower's width.
    fn billow(&self) -> f64 {
        if self.billow_size > 0. { self.billow_size } else { self.radius / 4. }
    }

    /// How far the anvil reaches downwind of the tower, and upwind of it.
    pub fn anvil_length(&self) -> f64 {
        self.upper_wind * self.age
    }

    fn anvil_back(&self) -> f64 {
        self.radius * 0.8 + (self.outflow_speed - self.upper_wind).max(0.) * self.age
    }

    /// Places the towers. Call after changing their parameters.
    pub fn build(&mut self) {
        let mut towers = vec![Tower { foot: self.center, radius: self.radius, top: self.tropopause + self.overshoot() }];
        let spacing = if self.flank_spacing > 0. { self.flank_spacing } else { 1.7 * self.radius };
        let a = self.flank_angle.to_radians();
        let up = -self.wind_direction;
        let dir = Vector2::new(up.x * a.cos() - up.y * a.sin(), up.x * a.sin() + up.y * a.cos());
        let mut rng = crate::geometry::Pcg32::new(self.seed, 0x7a);
        let mut along = 0.;
        for i in 1..=self.flanking {
            // Each tower younger and lower than the last, a little off the line.
            let f = i as f64 / (self.flanking + 1) as f64;
            let radius = self.radius * lerp(0.75, 0.45, f) * (0.85 + 0.3 * rng.next_f64());
            along += spacing * lerp(0.75, 0.5, f) * (0.8 + 0.4 * rng.next_f64());
            let off = (rng.next_f64() - 0.5) * 0.6 * radius;
            let foot = self.center + dir * along + Vector2::new(-dir.y, dir.x) * off;
            let depth = (self.glaciation - self.base) * (1. - f).powf(1.3) * (0.8 + 0.3 * rng.next_f64());
            towers.push(Tower { foot, radius, top: self.base + depth.max(1.5 * radius) });
        }
        self.towers = towers;
    }

    pub fn top(&self) -> f64 {
        self.tropopause + self.overshoot() + self.billow() + 0.4 * self.radius
    }

    pub fn bottom(&self) -> f64 {
        self.base - self.softness - 0.3 * self.billow()
    }

    /// The horizontal extent of the cloud: (min, max) corners [x, z].
    pub fn extent(&self) -> (Vector2<f64>, Vector2<f64>) {
        let mut lo = Vector2::new(f64::MAX, f64::MAX);
        let mut hi = -lo;
        let mut add = |p: Vector2<f64>, r: f64| {
            lo = lo.inf(&(p - Vector2::new(r, r)));
            hi = hi.sup(&(p + Vector2::new(r, r)));
        };
        let reach = self.lean * (self.top() - self.base);
        for t in &self.towers {
            let margin = 2. * t.radius + 0.4 * self.radius + self.billow();
            add(t.foot, margin);
            add(t.foot + self.wind_direction * reach, margin);
        }
        let top = self.towers[0].foot + self.wind_direction * reach;
        let far = top + self.wind_direction * self.anvil_length();
        let back = top - self.wind_direction * self.anvil_back();
        // The anvil's ragged edges reach a third beyond its half-width, its ends a tower's
        // radius beyond its length, and it sags and flares around the tower.
        let reach = |s: f64| self.anvil_half_width(s) * 1.4 + self.radius;
        add(far + self.wind_direction * self.radius, reach(self.anvil_length()));
        add(back - self.wind_direction * self.radius, reach(0.));
        add(top, 3.5 * self.radius);
        (lo, hi)
    }

    /// Where a tower's axis is at height y.
    fn axis(&self, t: &Tower, y: f64) -> Vector2<f64> {
        t.foot + self.wind_direction * (self.lean * (y - self.base).max(0.))
    }

    /// The anvil's half-width `s` metres downwind of the tower's top.
    fn anvil_half_width(&self, s: f64) -> f64 {
        let spread = self.outflow_speed / self.upper_wind.max(1.);
        1.1 * self.radius + spread * s.max(0.)
    }

    /// The cloud's density as a fraction of the local extinction, and that extinction: the
    /// towers (the main one swelling into the anvil's base under an overshooting dome, the
    /// flanking ones domed), pushed out in cauliflower billows and turrets where liquid and
    /// streaked where ice; the anvil, a wedge with ragged edges, hanging lobes and a sag to
    /// meet its tower; over a ragged base (`shared::cumulonimbus::Cumulonimbus::fraction`).
    pub fn fraction(&self, p: &Vector3<f64>) -> (f64, f64) {
        self.recipe().fraction(&self.tables(), p.x, p.y, p.z)
    }
}

/// The cloud's tables as its shared recipe reads them: the permutation of its noise; the
/// Worley texture of its billows; then its towers.
struct CloudTables<'a> {
    cloud: &'a Cumulonimbus,
    cells: &'a WorleyTexture,
}

const WORLEY_AT: u32 = 512;
const TOWERS_AT: u32 = WORLEY_AT + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N;

impl shared::Tables<f64> for CloudTables<'_> {
    fn value(&self, i: u32) -> f64 {
        if i >= TOWERS_AT {
            let (k, part) = (((i - TOWERS_AT) / shared::cumulonimbus::TOWER_SIZE) as usize, (i - TOWERS_AT) % shared::cumulonimbus::TOWER_SIZE);
            let t = &self.cloud.towers[k];
            return [t.foot.x, t.foot.y, t.radius, t.top][part as usize];
        }
        self.cells.texels()[(i - WORLEY_AT) as usize] as f64
    }

    fn index(&self, i: u32) -> u32 {
        self.cloud.noise.permutation()[i as usize] as u32
    }
}

impl Cumulonimbus {
    /// The recipe, as the shared code takes it (`shared::cumulonimbus::Cumulonimbus`, where
    /// the density is: the GPU's shader compiles the same functions at f32).
    fn recipe(&self) -> shared::cumulonimbus::Cumulonimbus<f64> {
        shared::cumulonimbus::Cumulonimbus {
            base: self.base, tropopause: self.tropopause, radius: self.radius, lean: self.lean,
            wind: (self.wind_direction.x, self.wind_direction.y), upper_wind: self.upper_wind, outflow_speed: self.outflow_speed,
            anvil_thickness: self.anvil_thickness, glaciation: self.glaciation, density: self.density, ice_density: self.ice_density,
            billowing: self.billowing, softness: self.softness, billow: self.billow(), anvil_length: self.anvil_length(),
            anvil_back: self.anvil_back(), bottom: self.bottom(), top: self.top(), towers: TOWERS_AT,
            tower_count: self.towers.len() as u32, perm: 0, worley: WORLEY_AT,
        }
    }

    fn tables(&self) -> CloudTables<'_> {
        CloudTables { cloud: self, cells: self.cells() }
    }

    /// The cloud as data, for the GPU's shader (`pathtrace::load_cumulonimbus`, which has the
    /// layout): the recipe's numbers in their fields' order and how many towers; then the
    /// permutation, the Worley texture and the towers.
    pub fn export(&self) -> Vec<f32> {
        let r = self.recipe();
        let mut out: Vec<f32> = [
            r.base, r.tropopause, r.radius, r.lean, r.wind.0, r.wind.1, r.upper_wind, r.outflow_speed, r.anvil_thickness,
            r.glaciation, r.density, r.ice_density, r.billowing, r.softness, r.billow, r.anvil_length, r.anvil_back, r.bottom,
            r.top, r.tower_count as f64,
        ].iter().map(|&v| v as f32).collect();
        out.extend(self.noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.cells().texels());
        for t in &self.towers {
            out.extend([t.foot.x as f32, t.foot.y as f32, t.radius as f32, t.top as f32]);
        }
        out
    }
}

impl DensityField for Cumulonimbus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let (f, sigma) = self.fraction(p);
        f * sigma
    }

    fn majorant(&self) -> f64 {
        self.density.max(self.ice_density)
    }

    /// Clear below the base and above the top; only ice above the glaciation layer.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        self.recipe().majorant_from(p.y, d.y)
    }

    fn export(&self) -> Option<crate::volume::FieldExport> {
        Some(crate::volume::FieldExport::Cumulonimbus { record: Cumulonimbus::export(self), radius: 0. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    #[test]
    fn test_overshoot_follows_parcel_theory() {
        let mut c = Cumulonimbus::new(1);
        c.cape = 2500.;
        c.efficiency = 0.5;
        c.stability = 0.02;
        // w = 0.5 sqrt(5000) ≈ 35 m/s, overshooting by w / N ≈ 1.8 km.
        assert!((c.updraft() - 35.36).abs() < 0.01);
        assert!((c.overshoot() - 1767.8).abs() < 1.);
    }

    #[test]
    fn test_majorant_bounds_density() {
        let c = Cumulonimbus::new(3);
        let (lo, hi) = c.extent();
        let mut rng = Pcg32::new(5, 0);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.next_f64();
        for _ in 0..600 {
            let p = Vector3::new(u(lo.x, hi.x), u(c.bottom() - 200., c.top() + 200.), u(lo.y, hi.y));
            let d = Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)).normalize();
            let (bound, len) = c.majorant_from(&p, &d);
            assert!(len > 0.);
            for k in 0..30 {
                let t = len.min(20000.) * (k as f64 + 0.5) / 30.;
                assert!(c.density(&(p + d * t)) <= bound + 1e-12);
            }
        }
    }

    #[test]
    fn test_tower_anvil_and_clear_air() {
        let c = Cumulonimbus::new(2);
        // The core of the tower, at mid-levels, is solid liquid cloud.
        let mid = c.base + 0.4 * (c.tropopause - c.base);
        let axis = c.axis(&c.towers[0], mid);
        let (f, sigma) = c.fraction(&Vector3::new(axis.x, mid, axis.y));
        assert!(f > 0.99 && sigma == c.density, "{} {}", f, sigma);
        // The anvil, downwind at the tropopause, is ice; under it, clear air.
        let top = c.axis(&c.towers[0], c.tropopause) + c.wind_direction * 0.3 * c.anvil_length();
        let (f, sigma) = c.fraction(&Vector3::new(top.x, c.tropopause - 300., top.y));
        assert!(f > 0.5 && (sigma - c.ice_density).abs() < 1e-12, "{} {}", f, sigma);
        assert_eq!(c.density(&Vector3::new(top.x, c.base + 1000., top.y)), 0.);
        // Nothing below the base, or far upwind.
        assert_eq!(c.density(&Vector3::new(axis.x, c.base - 500., axis.y)), 0.);
        let far = c.center - c.wind_direction * 40_000.;
        assert_eq!(c.density(&Vector3::new(far.x, c.tropopause - 300., far.y)), 0.);
    }

    #[test]
    fn test_anvil_ends_smoothly() {
        // Upwind of the tower and out to the sides through the anvil, the density changes
        // gradually: nothing slices it off along a line.
        for (seed, age, thickness) in [(0, 1800., 2200.), (3, 900., 1600.), (5, 1200., 2200.)] {
            let mut c = Cumulonimbus::new(seed);
            (c.age, c.anvil_thickness, c.radius) = (age, thickness, 3800.);
            c.wind_direction = Vector2::new(1., 0.15).normalize();
            c.build();
            let top = c.axis(&c.towers[0], c.tropopause);
            let w = c.wind_direction;
            for y in [c.tropopause - 300., c.tropopause - 1500.] {
                for dir in [-w, Vector2::new(-w.y, w.x), Vector2::new(w.y, -w.x)] {
                    let mut last = c.density(&Vector3::new(top.x, y, top.y));
                    for k in 1..800 {
                        let q = top + dir * (k as f64 * 25.);
                        let here = c.density(&Vector3::new(q.x, y, q.y));
                        assert!((here - last).abs() < 0.6 * c.ice_density, "seed {} at {} {}: {} after {}", seed, q, y, here, last);
                        last = here;
                    }
                }
            }
        }
    }

    #[test]
    fn test_flanking_towers_step_down() {
        let c = Cumulonimbus::new(4);
        assert_eq!(c.towers.len(), 1 + c.flanking);
        assert!(c.towers[1..].iter().all(|t| t.top < c.glaciation + 1000.));
        assert!(c.towers[1].top > c.towers[c.flanking].top);
    }
}

