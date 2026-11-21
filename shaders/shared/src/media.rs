//! Tracking light through a medium whose density varies: where it first collides (delta
//! tracking), and how much gets through (ratio tracking). The renderer's `volume::Volume` walks
//! with these, and has the account of the methods and their sources; a kernel walks the same.

use crate::{Random, Real};

/// A medium's extinction: its density at a point, and a bound on it from a point along a unit
/// direction with how far the bound holds (0 where it is known to be empty there).
pub trait Medium<F: Real> {
    fn density(&self, x: F, y: F, z: F) -> F;
    /// A bound on the density everywhere.
    fn bound(&self) -> F;
    fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F);
}

/// Below this estimated transmittance, ratio tracking plays Russian roulette; survivors carry
/// this much.
#[inline]
pub fn roulette<F: Real>() -> F {
    F::ratio(2, 100)
}

/// Ratio tracking samples collisions against this multiple of the residual majorant.
#[inline]
pub fn slack<F: Real>() -> F {
    F::of(2)
}

/// The first real collision along the ray from `o` in unit direction `d`, from `from` to
/// `exit` (one stretch of the ray inside the medium), by delta tracking against the medium's
/// local bounds, with every density scaled by `scale`; None if the light gets through.
pub fn delta_track<F: Real, M: Medium<F>, R: Random<F>>(medium: &M, o: (F, F, F), d: (F, F, F), from: F, exit: F, scale: F, random: &mut R) -> Option<F> {
    let mut t = from;
    while t < exit {
        let (x, y, z) = (o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
        let (majorant, length) = medium.majorant_from(x, y, z, d.0, d.1, d.2);
        // Always advance, if only by a hair.
        let end = (t + length.max(F::scale_at(t, x, y, z) * F::hair())).min(exit);
        if majorant > F::of(0) {
            // Scaling the density scales the majorant alike: steps shrink, and the chance of
            // a collision being real is unchanged.
            let step = -(F::of(1) - random.next()).ln() / (majorant * scale);
            if t + step < end {
                t = t + step;
                if random.next() * majorant < medium.density(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t) {
                    return Some(t);
                }
                continue;
            }
        }
        t = end;
    }
    None
}

/// What of `transmitted` is left after the stretch from `from` to `stop` of the ray, by
/// residual ratio tracking: the medium's lower bound `control` attenuated exactly, the rest
/// tracked. None if Russian roulette ends the walk (nothing gets through).
pub fn ratio_track<F: Real, M: Medium<F>, R: Random<F>>(medium: &M, o: (F, F, F), d: (F, F, F), from: F, stop: F, control: F, scale: F, transmitted: F, random: &mut R) -> Option<F> {
    let mut t = from;
    let mut transmitted = transmitted * (-control * scale * (stop - t)).exp();
    while t < stop {
        let (x, y, z) = (o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
        let (majorant, length) = medium.majorant_from(x, y, z, d.0, d.1, d.2);
        let end = (t + length.max(F::scale_at(t, x, y, z) * F::hair())).min(stop);
        let residual = (majorant - control) * slack();
        if residual > F::of(0) {
            let step = -(F::of(1) - random.next()).ln() / (residual * scale);
            if t + step < end {
                t = t + step;
                transmitted = transmitted * (F::of(1) - (medium.density(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t) - control) / residual);
                if transmitted < roulette() {
                    // Survive with probability T / threshold, carrying the threshold.
                    if random.next() * roulette() >= transmitted {
                        return None;
                    }
                    transmitted = roulette();
                }
                continue;
            }
        }
        t = end;
    }
    Some(transmitted)
}

/// How far a local bound through `OnPlanet` is trusted along a straight ray: over this, the
/// ray's altitude departs from the straight line in planet-local coordinates by L² / 2R, 8 cm
/// on the Earth, which the fields' bounds aren't padded for.
#[inline]
pub fn curved_step<F: Real>() -> F {
    F::of(1000)
}

/// Planet-local coordinates of a scene point, for a planet of `radius` whose surface touches
/// the origin from below: (distance along the surface eastwards, altitude, northwards), the
/// azimuthal equidistant projection about the origin's foot. Near the origin this is (x, y, z).
pub fn planet_local<F: Real>(radius: F, x: F, y: F, z: F) -> (F, F, F) {
    let up = y + radius;
    let sideways = (x * x + z * z).sqrt();
    // The angle from the pole, by atan2 so it is exact near the origin.
    let theta = sideways.atan2(up);
    let k = radius * theta / sideways;
    let (ax, az) = if sideways > F::of(0) { (x * k, z * k) } else { (F::of(0), F::of(0)) };
    (ax, F::above_sphere(x, y, z, radius), az)
}

/// A medium written in flat coordinates, curved round a planet (the renderer's
/// `planet::PlanetMapping`): its heights become altitudes, so it drops away with distance.
pub struct OnPlanet<'a, F: Real, M: Medium<F>> {
    pub radius: F,
    pub medium: &'a M,
}

impl<F: Real, M: Medium<F>> Medium<F> for OnPlanet<'_, F, M> {
    #[inline]
    fn density(&self, x: F, y: F, z: F) -> F {
        let q = planet_local(self.radius, x, y, z);
        self.medium.density(q.0, q.1, q.2)
    }

    #[inline]
    fn bound(&self) -> F {
        self.medium.bound()
    }

    /// The medium's local bound along the ray's direction in planet-local coordinates, its
    /// length converted back to the scene's, and trusted only `curved_step` at a time.
    fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        let q = planet_local(self.radius, x, y, z);
        // The direction there by a difference, over a stride the float can resolve: a metre
        // for f64; for f32, whose steps 50 km out are 4 mm, a metre's difference had its
        // vertical part wrong by half a degree, so that a level ray's rise or fall, which the
        // bounds below turn on, was noise (and the walk through a layer crawled: six times the
        // steps of f64's).
        let h = F::stride();
        let ahead = planet_local(self.radius, x + dx * h, y + dy * h, z + dz * h);
        let dq = ((ahead.0 - q.0) / h, (ahead.1 - q.1) / h, (ahead.2 - q.2) / h);
        let speed = (dq.0 * dq.0 + dq.1 * dq.1 + dq.2 * dq.2).sqrt();
        if speed <= F::of(0) {
            return (self.medium.bound(), curved_step());
        }
        let (majorant, length) = self.medium.majorant_from(q.0, q.1, q.2, dq.0 / speed, dq.1 / speed, dq.2 / speed);
        (majorant, (length / speed).min(curved_step()))
    }
}
