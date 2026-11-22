//! Rain, snow and hail falling from a cloud's base: the density of the renderer's
//! `procedural::precipitation`, which has the account of the method and its sources. Shafts
//! under the cells of a cloud, leaning with the wind's shear, streaked, evaporating as they
//! fall, with the extinction of the drop, flake or stone sizes at each rate.

use crate::noise::fbm;
use crate::{lerp, smoothstep, Real, Tables};

/// What falls (`Rain::kind`).
pub const RAIN: u32 = 0;
pub const SNOW: u32 = 1;
pub const HAIL: u32 = 2;

/// `x` cubed.
#[inline]
fn cube<F: Real>(x: F) -> F {
    x * x * x
}

/// Extinction (per metre) of rain falling at `rate` mm/h, for the Marshall-Palmer distribution
/// of drop sizes.
pub fn rain_extinction<F: Real>(rate: F) -> F {
    if rate <= F::of(0) {
        return F::of(0);
    }
    let n0 = F::of(8000) * F::ratio(1, 1_000_000);
    let lambda = F::ratio(41, 10) * rate.powf(F::ratio(-21, 100));
    F::pi() * n0 / cube(lambda)
}

/// Extinction (per metre) of snow falling at `rate` mm/h of water, for Gunn and Marshall's
/// distribution of melted diameters and flakes three times as wide.
pub fn snow_extinction<F: Real>(rate: F) -> F {
    if rate <= F::of(0) {
        return F::of(0);
    }
    let n0 = F::of(3800) * rate.powf(F::ratio(-87, 100)) * F::ratio(1, 1_000_000);
    let lambda = F::ratio(255, 100) * rate.powf(F::ratio(-48, 100));
    F::of(9) * F::pi() * n0 / cube(lambda)
}

/// Extinction (per metre) of hail of mean diameter `size` mm falling at `rate` mm/h of water
/// (as ice), exponentially distributed in size and falling at 3.62 √D m/s.
pub fn hail_extinction<F: Real>(rate: F, size: F) -> F {
    if rate <= F::of(0) {
        return F::of(0);
    }
    let lambda = F::of(1000) / size.max(F::ratio(1, 10));
    let flux = rate / F::of(3_600_000);
    let n0 = flux / (F::pi() / F::of(6) * F::ratio(362, 100) * F::of(1000).sqrt() * F::ratio(11_631_728, 1_000_000) / lambda.powf(F::ratio(9, 2)));
    F::pi() * n0 / cube(lambda)
}

/// Precipitation's recipe (the renderer's `Precipitation`, whose fields these are), and where
/// the permutations of its cells' noise and its streaks' are (512 entries each).
#[derive(Clone, Copy)]
pub struct Rain<F: Real> {
    pub base: F,
    pub ground: F,
    pub rain_rate: F,
    pub cell_size: F,
    pub coverage: F,
    /// The distance over which it falls to 1/e by evaporating, if it does (`evaporates`).
    pub fall_depth: F,
    pub shear: F,
    pub wind: (F, F),
    pub streak_size: F,
    pub streakiness: F,
    pub base_variation: F,
    pub center: (F, F),
    pub radius: F,
    pub fade: F,
    /// `RAIN`, `SNOW` or `HAIL`; hail's mean diameter (mm); and the height below which snow or
    /// hail has melted to rain, if there is one (`melts`).
    pub kind: u32,
    pub hail_size: F,
    pub melting_level: F,
    pub evaporates: bool,
    pub melts: bool,
    pub cells: u32,
    pub streaks: u32,
}

impl<F: Real> Rain<F> {
    /// Extinction per metre of what falls at height `y`, at `rate` mm/h of water.
    fn extinction_of(&self, kind: u32, rate: F) -> F {
        if kind == SNOW {
            snow_extinction(rate)
        } else if kind == HAIL {
            hail_extinction(rate, self.hail_size)
        } else {
            rain_extinction(rate)
        }
    }

    /// The most extinction anywhere: the full rate, with the streaks' strengthening, in
    /// whichever form stops more light.
    pub fn peak_extinction(&self) -> F {
        let rate = self.rain_rate * (F::of(1) + F::ratio(1, 2) * self.streakiness);
        let melted = if self.melts { rain_extinction(rate) } else { F::of(0) };
        self.extinction_of(self.kind, rate).max(melted)
    }

    /// The rain rate at a point, relative to `rain_rate` (0 to about 1.5 in the streaks).
    pub fn intensity<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (zero, one) = (F::of(0), F::of(1));
        if y < self.ground || y > self.base {
            return zero;
        }
        let fallen = self.base - y;
        // Where this rain fell from, at the base.
        let (qx, qz) = (x - self.wind.0 * self.shear * fallen, z - self.wind.1 * self.shear * fallen);
        let (cx, cz) = (qx - self.center.0, qz - self.center.1);
        let from_centre = (cx * cx + cz * cz).sqrt() / self.radius;
        let region = smoothstep(zero, self.fade.max(F::ratio(1, 1_000_000_000)), one - from_centre);
        if region <= zero {
            return zero;
        }
        let c = fbm(tables, self.cells, qx / self.cell_size, F::ratio(1, 2), qz / self.cell_size, 4, F::ratio(1, 2), F::of(2));
        let shafts = smoothstep(one - self.coverage, one - self.coverage + F::ratio(2, 10), c) * region;
        if shafts <= zero {
            return zero;
        }
        // Streaks, drawn out down the fall.
        let s = fbm(tables, self.streaks, qx / self.streak_size, y / (self.streak_size * F::of(15)), qz / self.streak_size, 3, F::ratio(1, 2), F::of(2));
        let streaks = lerp(one, F::ratio(2, 10) + F::ratio(13, 10) * s, self.streakiness);
        // Evaporation, fraying the streaks: the weaker ones end first.
        let (left, frayed) = if self.evaporates {
            let left = (-fallen / self.fall_depth).exp();
            (left, smoothstep(zero, F::ratio(3, 10), left - (one - s) * F::ratio(4, 10)))
        } else {
            (one, one)
        };
        // Drops leave the cloud over its last hundred metres or so.
        let below = if self.base_variation > zero {
            fbm(tables, self.streaks, qx / self.cell_size, F::ratio(21, 100), qz / self.cell_size, 3, F::ratio(1, 2), F::of(2)) * self.base_variation
        } else {
            zero
        };
        let emerging = smoothstep(below, below + F::of(150), fallen);
        (shafts * streaks * left * frayed * emerging).clamp(zero, one + F::ratio(1, 2) * self.streakiness)
    }

    /// The extinction at a point.
    pub fn density<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let i = self.intensity(tables, x, y, z);
        if i <= F::of(0) {
            return F::of(0);
        }
        let kind = if self.melts && y < self.melting_level { RAIN } else { self.kind };
        self.extinction_of(kind, self.rain_rate * i)
    }

    /// A bound on the density from a height along a direction, and how far it holds: nothing
    /// above the base or below the ground.
    pub fn majorant_from(&self, y: F, dy: F) -> (F, F) {
        let step = F::ratio(1, 100);
        let ahead = y + dy * step;
        let (lo, hi) = (self.ground, self.base);
        if ahead < lo {
            return (F::of(0), if dy > F::of(0) { (lo - ahead) / dy + step } else { F::far() });
        }
        if ahead > hi {
            return (F::of(0), if dy < F::of(0) { (hi - ahead) / dy + step } else { F::far() });
        }
        let exit = if dy > F::of(0) { (hi - ahead) / dy } else if dy < F::of(0) { (lo - ahead) / dy } else { F::far() };
        (self.peak_extinction(), exit + step)
    }
}
