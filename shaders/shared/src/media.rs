//! Tracking light through a medium whose density varies: where it first collides (delta
//! tracking), and how much gets through (ratio tracking). The renderer's `volume::Volume` walks
//! with these, and has the account of the methods and their sources; a kernel walks the same.

use crate::{Random, Real};

/// A medium's extinction: its density at a point, and a bound on it from a point along a unit
/// direction with how far the bound holds (0 where it is known to be empty there).
pub trait Medium<F: Real> {
    fn density(&self, x: F, y: F, z: F) -> F;
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
        let (majorant, length) = medium.majorant_from(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t, d.0, d.1, d.2);
        // Always advance, if only by a hair.
        let end = (t + length.max(t.abs().max(F::of(1)) * F::hair())).min(exit);
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
        let (majorant, length) = medium.majorant_from(o.0 + d.0 * t, o.1 + d.1 * t, o.2 + d.2 * t, d.0, d.1, d.2);
        let end = (t + length.max(t.abs().max(F::of(1)) * F::hair())).min(stop);
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
