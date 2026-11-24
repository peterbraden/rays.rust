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
    /// The local bound, or, if not `sampled`, one that is proven: a medium whose local bounds
    /// are partly sampled (a majorant grid's cells, which can miss a wisp between the points
    /// they were sampled at) gives the bounds it can prove. The walks change to these where a
    /// density was found over its bound, so that an overshoot weights a walk once.
    fn majorant_along(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F, sampled: bool) -> (F, F) {
        let _ = sampled;
        self.majorant_from(x, y, z, dx, dy, dz)
    }
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
    let (mut weight, mut sampled) = (F::of(1), true);
    delta_track_weighted(medium, o, d, from, exit, scale, random, &mut weight, &mut sampled)
}

/// `delta_track`, correct where a bound isn't one: where the density at a tentative
/// collision exceeds the bound it was drawn against (a bound found by sampling the density,
/// which can miss a wisp: the kernels' majorant grids), the collision is chosen with
/// probability σ / (σ + |μ - σ|) and `weight` multiplied by the event's weight, the null
/// event's negative: weighted delta tracking (Kutz, Habel, Li and Novák, "Spectral and
/// Decomposition Tracking for Rendering Heterogeneous Volumes", SIGGRAPH 2017, section 4),
/// whose estimates are unbiased whatever the bound. Where the density is within its bound,
/// as with every bound the renderer's own fields give, it draws the same numbers as plain
/// delta tracking and the weight stays 1. Bounds are the medium's sampled ones while
/// `sampled`, which an overshoot clears: a path then walks by proven bounds from there on
/// (any bound may be chosen at each event, given what came before), so that a cloud the
/// sampled bounds missed weights the path once, not at every event and bounce inside it.
#[allow(clippy::too_many_arguments)]
pub fn delta_track_weighted<F: Real, M: Medium<F>, R: Random<F>>(medium: &M, o: (F, F, F), d: (F, F, F), from: F, exit: F, scale: F, random: &mut R, weight: &mut F, sampled: &mut bool) -> Option<F> {
    let mut t = from;
    while t < exit {
        let (x, y, z) = (o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
        let (majorant, length) = medium.majorant_along(x, y, z, d.0, d.1, d.2, *sampled);
        // Always advance, if only by a hair.
        let end = (t + length.max(F::scale_at(t, x, y, z) * F::hair())).min(exit);
        if majorant > F::of(0) {
            // Scaling the density scales the majorant alike: steps shrink, and the chance of
            // a collision being real is unchanged.
            let step = -(F::of(1) - random.next()).ln() / (majorant * scale);
            if t + step < end {
                t = t + step;
                let density = medium.density(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
                if density <= majorant {
                    if random.next() * majorant < density {
                        return Some(t);
                    }
                } else {
                    // Over the bound: the real and null events in proportion to σ and
                    // |μ - σ|, each weighted by its coefficient over μ times its probability;
                    // and from here on proven bounds (a walk may change its majorant at any
                    // event), lest a feature a sampled bound missed weight the walk again at
                    // every event in it.
                    *sampled = false;
                    let both = density + (density - majorant);
                    if random.next() * both < density {
                        *weight = *weight * (both / majorant);
                        return Some(t);
                    }
                    *weight = *weight * -(both / majorant);
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
    ratio_track_from(medium, o, d, from, stop, control, scale, transmitted, random, true)
}

/// `ratio_track`, by sampled bounds where `sampled` (until a density exceeds one), else by
/// proven ones (`Medium::majorant_along`).
#[allow(clippy::too_many_arguments)]
pub fn ratio_track_from<F: Real, M: Medium<F>, R: Random<F>>(medium: &M, o: (F, F, F), d: (F, F, F), from: F, stop: F, control: F, scale: F, transmitted: F, random: &mut R, sampled: bool) -> Option<F> {
    let mut t = from;
    let mut transmitted = transmitted * (-control * scale * (stop - t)).exp();
    let mut sampled = sampled;
    while t < stop {
        let (x, y, z) = (o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
        let (majorant, length) = medium.majorant_along(x, y, z, d.0, d.1, d.2, sampled);
        let end = (t + length.max(F::scale_at(t, x, y, z) * F::hair())).min(stop);
        let residual = (majorant - control) * slack();
        if residual > F::of(0) {
            let step = -(F::of(1) - random.next()).ln() / (residual * scale);
            if t + step < end {
                t = t + step;
                let density = medium.density(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t);
                // (Proven bounds from here on, as in `delta_track_weighted`.)
                sampled = sampled && density <= majorant;
                transmitted = transmitted * (F::of(1) - (density - control) / residual);
                // (Negative where a density exceeded its bound by more than the residual's
                // slack: still an unbiased estimate, so roulette goes by its size and keeps
                // its sign.)
                let size = transmitted.abs();
                if size < roulette() {
                    // Survive with probability |T| / threshold, carrying the threshold.
                    if random.next() * roulette() >= size {
                        return None;
                    }
                    transmitted = roulette::<F>() * transmitted.sign();
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
    let theta = sideways.small_angle(up);
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
        let (q, dq, speed) = planet_ray(self.radius, x, y, z, dx, dy, dz);
        if speed <= F::of(0) {
            return (self.medium.bound(), curved_step());
        }
        let (majorant, length) = self.medium.majorant_from(q.0, q.1, q.2, dq.0, dq.1, dq.2);
        (majorant, (length / speed).min(curved_step()))
    }
}

/// A ray from (`x`, `y`, `z`) along (`dx`, `dy`, `dz`) in a planet's local coordinates
/// (`planet_local`): where it is, the unit direction it goes in there, and how fast that
/// coordinate moves along the scene's ray (0 if it can't be found), by which a length there
/// is converted back.
pub fn planet_ray<F: Real>(radius: F, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> ((F, F, F), (F, F, F), F) {
    let q = planet_local(radius, x, y, z);
    // The direction there by a difference, over a stride the float can resolve: a metre
    // for f64; for f32, whose steps 50 km out are 4 mm, a metre's difference had its
    // vertical part wrong by half a degree, so that a level ray's rise or fall, which the
    // bounds turn on, was noise (and the walk through a layer crawled: six times the
    // steps of f64's).
    let h = F::stride();
    let ahead = planet_local(radius, x + dx * h, y + dy * h, z + dz * h);
    let dq = ((ahead.0 - q.0) / h, (ahead.1 - q.1) / h, (ahead.2 - q.2) / h);
    let speed = (dq.0 * dq.0 + dq.1 * dq.1 + dq.2 * dq.2).sqrt();
    if speed <= F::of(0) {
        return (q, dq, F::of(0));
    }
    (q, (dq.0 / speed, dq.1 / speed, dq.2 / speed), speed)
}
