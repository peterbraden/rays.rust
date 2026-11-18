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
            let margin = 1.6 * t.radius + 0.4 * self.radius + self.billow();
            add(t.foot, margin);
            add(t.foot + self.wind_direction * reach, margin);
        }
        let top = self.towers[0].foot + self.wind_direction * reach;
        let far = top + self.wind_direction * self.anvil_length();
        let back = top - self.wind_direction * self.anvil_back();
        let width = self.anvil_half_width(self.anvil_length()) * 1.3;
        add(far, width);
        add(back, self.anvil_half_width(0.) * 1.3);
        (lo, hi)
    }

    /// Where a tower's axis is at height y.
    fn axis(&self, t: &Tower, y: f64) -> Vector2<f64> {
        t.foot + self.wind_direction * (self.lean * (y - self.base).max(0.))
    }

    /// How far inside tower `t` the point is (m, negative outside), before billows.
    fn tower_depth(&self, t: &Tower, p: &Vector3<f64>) -> f64 {
        let rho = (Vector2::new(p.x, p.z) - self.axis(t, p.y)).norm();
        let h = ((p.y - self.base) / (t.top - self.base)).clamp(0., 1.);
        // The main tower narrower at the base, where the updraft is gathered, swelling upwards
        // into the anvil; the younger towers domed, narrowing upwards.
        let main = t.top > self.tropopause;
        let mut r = t.radius * if main { lerp(0.7, 1.15, h) } else { lerp(1., 0.6, h) };
        // A rounded top: an ellipsoidal cap.
        let cap = (0.8 * t.radius).min(0.5 * (t.top - self.base));
        let from_cap = p.y - (t.top - cap);
        if from_cap > 0. {
            let q = (from_cap / cap).min(1.);
            r *= (1. - q * q).max(0.).sqrt();
        }
        (r - rho).min(p.y - self.base + 1.).min(t.top - p.y)
    }

    /// The anvil's half-width `s` metres downwind of the tower's top.
    fn anvil_half_width(&self, s: f64) -> f64 {
        let spread = self.outflow_speed / self.upper_wind.max(1.);
        1.1 * self.radius + spread * s.max(0.)
    }

    /// How far inside the anvil the point is (m, negative outside), before its texture.
    fn anvil_depth(&self, p: &Vector3<f64>) -> f64 {
        let w = self.wind_direction;
        let top_axis = self.axis(&self.towers[0], self.tropopause);
        let d = Vector2::new(p.x, p.z) - top_axis;
        let s = d.dot(&w);
        let n = d.x * -w.y + d.y * w.x;
        let length = self.anvil_length();
        let back = self.anvil_back();
        if s < -back - self.radius || s > length + self.radius {
            return -1e9;
        }
        // Ragged ends and edges.
        let ragged = (self.noise.fbm(s / 9000., 0.3, n / 9000., 3, 0.5, 2.) - 0.5) * 2.;
        let half = self.anvil_half_width(s) * (1. + 0.25 * ragged);
        let along = if s >= 0. { s / length.max(1.) } else { -s / back.max(1.) };
        let along = (along + 0.15 * ragged).max(0.);
        // A wedge: thickest at the tower, thinning towards its far end (most of the ice has
        // spread and settled there), its edges cut off steeply across and at the end.
        let across = (n / half).abs();
        let edge = (1. - across.powi(4)).max(0.).powf(0.35) * (1. - along.powi(6)).max(0.).powf(0.35);
        let thickness = self.anvil_thickness * (1. - 0.85 * along.min(1.)).powf(1.5) * edge;
        // A flat top, rising and falling a little.
        let top = self.tropopause + 150. * (self.noise.fbm(p.x / 15000., 0.7, p.z / 15000., 2, 0.5, 2.) - 0.5);
        (top - p.y).min(p.y - (top - thickness))
    }

    /// Cauliflower billows: how far the surface is pushed out at p (m).
    fn billows(&self, p: &Vector3<f64>, scale: f64) -> f64 {
        let cells = self.cells();
        let cell = |q: Vector3<f64>| (1. - cells.at(&q)).clamp(0., 1.);
        let q = p / scale;
        let warp = Vector3::new(
            self.noise.noise(q.x * 0.5, q.y * 0.5, q.z * 0.5 + 3.1),
            self.noise.noise(q.x * 0.5 + 7.7, q.y * 0.5, q.z * 0.5),
            self.noise.noise(q.x * 0.5, q.y * 0.5 + 5.3, q.z * 0.5),
        ) * 0.35;
        let q = q + warp;
        scale * (0.55 * (cell(q) - 0.45) + 0.25 * (cell(q * 2.6 + Vector3::new(0.3, 0.1, 0.7)) - 0.45)
            + 0.12 * (cell(q * 6.5 + Vector3::new(0.9, 0.4, 0.2)) - 0.45))
    }

    /// Fibrous ice: streaks along the wind (m).
    fn fibres(&self, p: &Vector3<f64>, scale: f64) -> f64 {
        let w = self.wind_direction;
        let along = (p.x * w.x + p.z * w.y) / (scale * 6.);
        let across = (-p.x * w.y + p.z * w.x) / (scale * 0.7);
        scale * 0.6 * (self.noise.fbm(along, p.y / (scale * 0.8), across, 4, 0.55, 2.1) - 0.5)
    }

    /// How icy the cloud is at height y: 0 liquid, 1 ice.
    fn ice(&self, y: f64) -> f64 {
        smoothstep(self.glaciation - 1000., self.glaciation + 1000., y)
    }

    /// The cloud's density as a fraction of the local extinction, and that extinction.
    pub fn fraction(&self, p: &Vector3<f64>) -> (f64, f64) {
        if p.y < self.bottom() || p.y > self.top() {
            return (0., 0.);
        }
        let b = self.billow();
        let margin = b * 0.8;
        let mut depth = f64::MIN;
        let margin_lobes = margin + 0.4 * self.radius;
        for t in &self.towers {
            if p.y > t.top + margin_lobes {
                continue;
            }
            let rho = (Vector2::new(p.x, p.z) - self.axis(t, p.y)).norm();
            if rho > 1.1 * t.radius + margin_lobes {
                continue;
            }
            depth = depth.max(self.tower_depth(t, p));
        }
        let anvil = if p.y > self.tropopause - self.anvil_thickness - margin { self.anvil_depth(p) } else { -1e9 };
        let near_surface = depth > -margin_lobes || anvil > -margin;
        if !near_surface {
            return (0., 0.);
        }
        let ice = self.ice(p.y);
        // Hard cauliflower turrets in the liquid cloud, soft streaks in the ice.
        let surface = if ice < 1. { self.billows(p, b) * self.billowing } else { 0. };
        let streaks = if ice > 0. { self.fibres(p, b * 0.6) } else { 0. };
        // Turrets: the largest lobes, rising thermals a tower's width across, which the billows
        // then cover.
        let lobes = if depth > -margin - 0.4 * self.radius {
            let q = p / (0.8 * self.radius);
            0.3 * self.radius * ((self.noise.fbm(q.x, q.y * 0.7, q.z, 2, 0.5, 2.) - 0.4) * 2.).max(-0.4) * lerp(1., 0.4, ice)
        } else {
            0.
        };
        let push = lerp(surface, streaks, ice) + lobes;
        let softness = lerp(self.softness, 6. * self.softness, ice);
        // The anvil is ice throughout, with streaks.
        let anvil = anvil + self.fibres(p, b * 0.5);
        let inside = (depth + push).max(anvil);
        let f = smoothstep(0., softness, inside);
        if f <= 0. {
            return (0., 0.);
        }
        // A ragged base, as drops evaporate below it.
        let under = smoothstep(self.base - self.softness, self.base + self.softness, p.y + 0.2 * self.billows(p, b * 0.3));
        (f * under, lerp(self.density, self.ice_density, ice.max(if anvil > depth + push { 1. } else { 0. })))
    }

    fn icy_above(&self) -> f64 {
        self.glaciation + 1000.
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
        let ahead = p + d * 0.01;
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        let positive = |t: f64| if t > 0. { t } else { f64::INFINITY };
        let (lo, hi, icy) = (self.bottom(), self.top(), self.icy_above());
        let (m, len) = if ahead.y < lo {
            (0., if d.y > 0. { to_y(lo) } else { f64::INFINITY })
        } else if ahead.y > hi {
            (0., if d.y < 0. { to_y(hi) } else { f64::INFINITY })
        } else if ahead.y > icy {
            (self.ice_density, positive(to_y(icy)).min(positive(to_y(hi))))
        } else {
            (self.majorant(), positive(to_y(lo)).min(positive(to_y(icy))))
        };
        (m, len + 0.01)
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
    fn test_flanking_towers_step_down() {
        let c = Cumulonimbus::new(4);
        assert_eq!(c.towers.len(), 1 + c.flanking);
        assert!(c.towers[1..].iter().all(|t| t.top < c.glaciation + 1000.));
        assert!(c.towers[1].top > c.towers[c.flanking].top);
    }
}

