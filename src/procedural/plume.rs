//! # Volcanic plumes
//!
//! The gas rising from a lava lake or a vent: steam, sulphur dioxide and the sulphate haze it
//! makes (vog), carried up by the lava's heat and bent over by the wind. A density field for
//! `volume::Volume`.
//!
//! A buoyant plume in a wind rises along G. A. Briggs' "two-thirds law" ("Plume rise
//! predictions", Lectures on Air Pollution and Environmental Impact Analyses, AMS, 1975):
//! Δh = 1.6 F^⅓ x^⅔ / U at a distance x downwind, for a buoyancy flux F = g Q / (π ρ c_p T)
//! from a heat output Q, in a wind U; in stable air it levels off at Δh = 2.6 (F / (U s))^⅓,
//! where s = (g / T) dθ/dz. Its radius is about half its rise as it climbs, entraining the air
//! about it, then grows slowly with the wind's turbulence; and it thins downwind as it is
//! diluted, the steam evaporating. A lava lake of Halemaʻumaʻu's size loses about a gigawatt,
//! so its plume rises some 500 m in a trade wind of 7 m/s before drifting off as a level haze.
//!
//! The centreline is kept as a polyline from the source up the bend and along the wind; the
//! density about it falls off as a Gaussian of the plume's radius there, and is broken into
//! billows by noise carried along the plume (so they ride with it), denser and lumpier near
//! the source, smoother and more diffuse as it drifts.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::smoothstep;
use crate::volume::DensityField;

/// A buoyant plume from `source`, bent over by the wind.
pub struct Plume {
    /// Where it leaves the lava (a scene point), and the source's radius (m).
    pub source: Vector3<f64>,
    pub radius: f64,
    /// Its centreline from the source: points, and the distance along it to each (m).
    line: Vec<Vector3<f64>>,
    along: Vec<f64>,
    /// Its half-widths at each point of the centreline (m): across the wind, and up and down
    /// (in stable air a plume fans out sideways far faster than it deepens).
    widths: Vec<f64>,
    depths: Vec<f64>,
    /// The direction the wind carries it, in the scene's plan (x, z).
    pub wind: Vector2<f64>,
    /// Extinction at the source, per metre.
    pub density: f64,
    /// How far downwind it lasts before it is diluted away (m).
    pub length: f64,
    noise: PerlinNoise,
}

/// The plume's centreline is sampled this many times.
const POINTS: usize = 96;

/// The atmosphere's stability, s = (g / T) dθ/dz, per second squared: for a lapse rate of
/// 6 K/km against the dry adiabatic 9.8 (θ rising 3.8 K/km), as in the trade winds' lower air.
const STABILITY: f64 = 9.81 / 290. * 0.0038;

impl Plume {
    /// A plume from `source` (a lava lake of `radius` m losing `heat` W) in a wind of `speed`
    /// m/s blowing towards `wind` (the scene's plan, x and z), lasting `length` m downwind,
    /// with extinction `density` per metre at the source; its billows from `seed`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(source: Vector3<f64>, radius: f64, heat: f64, wind: Vector2<f64>, speed: f64, density: f64, length: f64, seed: u64) -> Plume {
        let wind = if wind.norm() > 0. { wind.normalize() } else { Vector2::new(1., 0.) };
        let u = speed.max(0.5);
        let flux = 9.81 * heat / (std::f64::consts::PI * 1.2 * 1005. * 290.);
        let top = 2.6 * (flux / (u * STABILITY)).cbrt();
        // Its rise at x metres downwind: the two-thirds law, levelling off at its final rise.
        let rise = |x: f64| {
            let climbing = 1.6 * flux.cbrt() * x.max(0.).powf(2. / 3.) / u;
            if climbing <= 0. { return 0.; }
            (climbing.powi(-3) + top.powi(-3)).powf(-1. / 3.)
        };
        // Points along it: dense near the source, where it bends, spreading out downwind.
        let mut line = Vec::with_capacity(POINTS);
        let (mut widths, mut depths) = (Vec::with_capacity(POINTS), Vec::with_capacity(POINTS));
        for k in 0..POINTS {
            let t = k as f64 / (POINTS - 1) as f64;
            let x = length * t * t;
            let h = rise(x);
            line.push(source + Vector3::new(wind.x * x, h, wind.y * x));
            // Half its rise as it climbs, entraining air; then the wind's turbulence spreads it,
            // sideways as Pasquill-Gifford's stable classes do, hardly at all up and down.
            widths.push(radius + 0.4 * h + 0.06 * x);
            depths.push(radius + 0.4 * h + 0.008 * x);
        }
        let mut along = vec![0.];
        for k in 1..POINTS { along.push(along[k - 1] + (line[k] - line[k - 1]).norm()); }
        Plume { source, radius, line, along, widths, depths, wind, density, length, noise: PerlinNoise::shuffled(seed.wrapping_mul(31).wrapping_add(7)) }
    }

    /// Its final rise above the source (m).
    pub fn top(&self) -> f64 {
        self.line.last().map_or(0., |p| p.y - self.source.y)
    }

    /// The box it fills.
    pub fn bounds(&self) -> (Vector3<f64>, Vector3<f64>) {
        let (mut lo, mut hi) = (self.source, self.source);
        for ((p, w), d) in self.line.iter().zip(&self.widths).zip(&self.depths) {
            let r = Vector3::repeat(2. * w.max(*d));
            lo = lo.inf(&(p - r));
            hi = hi.sup(&(p + r));
        }
        // (It doesn't sink below its source.)
        lo.y = lo.y.max(self.source.y - 2. * self.radius);
        (lo, hi)
    }

    /// Where `p` is about the centreline's nearest point: the distance along the plume to it,
    /// the plume's half-width and half-depth there, and `p`'s offset across the wind and
    /// square to the plume besides.
    fn nearest(&self, p: &Vector3<f64>) -> (f64, f64, f64, f64, f64) {
        let mut best = (0, 0., f64::INFINITY);
        for k in 0..POINTS - 1 {
            let (a, b) = (self.line[k], self.line[k + 1]);
            let ab = b - a;
            let t = ((p - a).dot(&ab) / ab.norm_squared().max(1e-12)).clamp(0., 1.);
            let d = (p - (a + ab * t)).norm_squared();
            if d < best.2 { best = (k, t, d); }
        }
        let (k, t, _) = best;
        let (a, b) = (self.line[k], self.line[k + 1]);
        let tangent = (b - a).normalize();
        let off = p - (a + (b - a) * t);
        let side = Vector3::new(-self.wind.y, 0., self.wind.x);
        let across = off.dot(&side);
        let rest = off - side * across - tangent * off.dot(&tangent);
        // (Up or down: below the centreline negative.)
        let square = rest.norm() * if rest.y < 0. { -1. } else { 1. };
        let lerp = |v: &[f64]| v[k] + t * (v[k + 1] - v[k]);
        (lerp(&self.along), lerp(&self.widths), lerp(&self.depths), across, square)
    }

    /// The fraction of the source's density at `p`, 0 to 1.
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        // (Not below its source: it rises, and the ground is there.)
        if p.y < self.source.y - 2. * self.radius { return 0.; }
        let (s, w, h, across, square) = self.nearest(p);
        let r2 = (across / w).powi(2) + (square / h).powi(2);
        if r2 > 4. { return 0.; }
        // Diluted as it spreads and drifts, the steam evaporating.
        let dilution = (self.radius * self.radius / (w * h)).powf(0.6) * (1. - smoothstep(0.3 * self.length, self.length, s));
        // Billows in the plume's own frame, so they ride along it as it goes, each about its
        // depth across and drawn out along it; lumpy near the source, smoothing as it drifts.
        let scale = 0.7 * h;
        let billows = self.noise.fbm(s / (1.6 * scale), square / scale, across / (1.4 * scale), 5, 0.5, 2.);
        let lumpy = 1. - smoothstep(0., 0.5 * self.length, s);
        let core = (-r2).exp();
        let shaped = (core + (0.6 + 0.8 * lumpy) * (billows - 0.5)).clamp(0., 1.);
        dilution * smoothstep(0.08, 0.6, shaped)
    }
}

impl DensityField for Plume {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.density
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lake() -> Plume {
        // Halemaʻumaʻu's: 150 m across, a gigawatt, in a trade wind of 7 m/s to the west.
        Plume::new(Vector3::new(0., 1000., 0.), 150., 1e9, Vector2::new(-1., 0.), 7., 0.02, 15_000., 3)
    }

    #[test]
    fn test_a_lava_lakes_plume_rises_hundreds_of_metres_and_bends_downwind() {
        let p = lake();
        assert!(p.top() > 300. && p.top() < 1000., "{} m", p.top());
        // A kilometre downwind it has risen most of the way, and is denser on its centreline
        // than beside it.
        let x = 1000.;
        let up = p.line.iter().find(|q| q.x <= -x).unwrap().y - 1000.;
        assert!(up > 0.5 * p.top(), "{} m up at {} m", up, x);
        let mean = |c: Vector3<f64>| (0..64).map(|k| p.density(&(c + Vector3::new(0., 0., 5. * k as f64 - 160.)))).sum::<f64>() / 64.;
        let on = mean(Vector3::new(-x, 1000. + up, 0.));
        let off = mean(Vector3::new(-x, 1000. + up, 1500.));
        assert!(on > 0. && on > 10. * off, "{} {}", on, off);
        // Nothing upwind of the source, or far below it.
        assert_eq!(p.density(&Vector3::new(2000., 1300., 0.)), 0.);
        assert_eq!(p.density(&Vector3::new(-x, 500., 0.)), 0.);
    }

    #[test]
    fn test_downwind_it_fans_out_into_a_thinning_haze() {
        // Ten kilometres downwind: wider across the wind than it is deep, and thinner than near
        // its source.
        let p = lake();
        let k = p.along.iter().position(|&a| a > 10_000.).unwrap();
        assert!(p.widths[k] > 2. * p.depths[k], "{} wide, {} deep", p.widths[k], p.depths[k]);
        let column = |c: Vector3<f64>, d: Vector3<f64>, n: usize, step: f64| (0..n).map(|i| p.density(&(c + d * (step * (i as f64 - n as f64 / 2.))))).sum::<f64>() * step;
        let (near, far) = (p.line[POINTS / 6], p.line[k]);
        let across = column(far, Vector3::z(), 400, 10.);
        let up = column(far, Vector3::y(), 400, 10.);
        assert!(across > 0. && up > 0.);
        let mean = |c: Vector3<f64>| (0..200).map(|i| p.density(&(c + Vector3::new(0., 0., i as f64 - 100.)))).sum::<f64>() / 200.;
        assert!(mean(far) < 0.5 * mean(near), "{} far, {} near", mean(far), mean(near));
    }

    #[test]
    fn test_the_plume_stays_in_its_bounds_and_under_its_majorant() {
        let p = lake();
        let (lo, hi) = p.bounds();
        let mut rng = crate::geometry::Pcg32::new(1, 1);
        for _ in 0..20_000 {
            let q = Vector3::new(-16_000. + 20_000. * rng.next_f64(), 500. + 2500. * rng.next_f64(), -4000. + 8000. * rng.next_f64());
            let d = p.density(&q);
            assert!(d <= p.majorant());
            if d > 0. { assert!((0..3).all(|i| q[i] >= lo[i] && q[i] <= hi[i]), "{:?} outside {:?} {:?}", q, lo, hi); }
        }
    }
}
