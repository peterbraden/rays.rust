//! Cumulonimbus: a thunderstorm's towers and its anvil. The density of the renderer's
//! `procedural::cumulonimbus`, which has the account of the method and its sources: towers
//! shaped by parcel theory with cauliflower billows, glaciating into fibrous ice, and an anvil
//! carried downwind from the main tower's top.

use crate::clouds::worley;
use crate::noise::{fbm, perlin};
use crate::{lerp, smoothstep, Real, Tables};

/// Numbers a tower: where its axis meets the base (x, z), its radius, and its top.
pub const TOWER_SIZE: u32 = 4;

/// The recipe (the renderer's `Cumulonimbus`: its fields, and what it derives from them once),
/// and where its tables are: the permutation of its noise (512 entries), the Worley texture of
/// its billows, and its `towers`, the main one first.
#[derive(Clone, Copy)]
pub struct Cumulonimbus<F: Real> {
    pub base: F,
    pub tropopause: F,
    pub radius: F,
    pub lean: F,
    pub wind: (F, F),
    pub upper_wind: F,
    pub outflow_speed: F,
    pub anvil_thickness: F,
    pub glaciation: F,
    pub density: F,
    pub ice_density: F,
    pub billowing: F,
    pub softness: F,
    /// The largest billows; how far the anvil reaches downwind of the tower and upwind of it;
    /// and the heights between which there is cloud.
    pub billow: F,
    pub anvil_length: F,
    pub anvil_back: F,
    pub bottom: F,
    pub top: F,
    pub towers: u32,
    pub tower_count: u32,
    pub perm: u32,
    pub worley: u32,
}

/// A tower's numbers.
#[derive(Clone, Copy)]
struct Tower<F: Real> {
    foot: (F, F),
    radius: F,
    top: F,
}

impl<F: Real> Cumulonimbus<F> {
    fn tower<T: Tables<F>>(&self, tables: &T, i: u32) -> Tower<F> {
        let at = self.towers + i * TOWER_SIZE;
        Tower { foot: (tables.value(at), tables.value(at + 1)), radius: tables.value(at + 2), top: tables.value(at + 3) }
    }

    /// Where a tower's axis is at height y.
    fn axis(&self, t: &Tower<F>, y: F) -> (F, F) {
        let along = self.lean * (y - self.base).max(F::of(0));
        (t.foot.0 + self.wind.0 * along, t.foot.1 + self.wind.1 * along)
    }

    /// How far a point is from a tower's axis.
    fn from_axis(&self, t: &Tower<F>, x: F, y: F, z: F) -> F {
        let a = self.axis(t, y);
        let (dx, dz) = (x - a.0, z - a.1);
        (dx * dx + dz * dz).sqrt()
    }

    /// How far inside tower `t` the point is (m, negative outside), before billows.
    fn tower_depth(&self, t: &Tower<F>, x: F, y: F, z: F) -> F {
        let (zero, one) = (F::of(0), F::of(1));
        let rho = self.from_axis(t, x, y, z);
        let h = ((y - self.base) / (t.top - self.base)).clamp(zero, one);
        // The main tower narrower at the base, swelling upwards, flaring through its upper
        // third into the anvil's base, then narrowing to the overshooting dome; the younger
        // towers domed, narrowing upwards.
        let main = t.top > self.tropopause;
        let flare = smoothstep(F::ratio(1, 2), F::ratio(8, 10), h) * (one - smoothstep(F::ratio(85, 100), F::ratio(95, 100), h));
        let mut r = t.radius * if main { lerp(F::ratio(7, 10), F::ratio(11, 10), h) + F::ratio(1, 2) * flare } else { lerp(one, F::ratio(6, 10), h) };
        // A rounded top: an ellipsoidal cap.
        let cap = (F::ratio(8, 10) * t.radius).min(F::ratio(1, 2) * (t.top - self.base));
        let from_cap = y - (t.top - cap);
        if from_cap > zero {
            let q = (from_cap / cap).min(one);
            r = r * (one - q * q).max(zero).sqrt();
        }
        (r - rho).min(y - self.base + one).min(t.top - y)
    }

    /// The anvil's half-width `s` metres downwind of the tower's top.
    fn anvil_half_width(&self, s: F) -> F {
        let spread = self.outflow_speed / self.upper_wind.max(F::of(1));
        F::ratio(11, 10) * self.radius + spread * s.max(F::of(0))
    }

    /// How far inside the anvil the point is (m, negative outside), before its texture.
    fn anvil_depth<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (zero, half, one, two) = (F::of(0), F::ratio(1, 2), F::of(1), F::of(2));
        let w = self.wind;
        let top_axis = self.axis(&self.tower(tables, 0), self.tropopause);
        let d = (x - top_axis.0, z - top_axis.1);
        let s = d.0 * w.0 + d.1 * w.1;
        let n = d.0 * -w.1 + d.1 * w.0;
        let (length, back) = (self.anvil_length, self.anvil_back);
        if s < -back - self.radius || s > length + self.radius {
            return F::of(-1_000_000_000);
        }
        // Ragged ends and edges, scalloped at every scale down to a few hundred metres.
        let ragged = (fbm(tables, self.perm, s / F::of(7000), F::ratio(3, 10), n / F::of(7000), 5, F::ratio(6, 10), F::ratio(22, 10)) - half) * two;
        let half_width = self.anvil_half_width(s) * (one + F::ratio(35, 100) * ragged);
        let along = if s >= zero { s / length.max(one) } else { -s / back.max(one) };
        let along = (along + F::ratio(15, 100) * ragged).max(zero);
        // A wedge: thickest at the tower, thinning towards its far end, its edges cut off
        // steeply across and at the end; upwind, where the outflow pushes against the wind,
        // the end is rounded.
        let across = (n / half_width).abs();
        let end = if s >= zero {
            let a2 = along * along;
            (one - a2 * (a2 * a2)).max(zero).powf(F::ratio(35, 100))
        } else {
            (one - along * along).max(zero).sqrt()
        };
        let c2 = across * across;
        let edge = (one - c2 * c2).max(zero).powf(F::ratio(35, 100)) * end;
        // Its underside hangs in soft lobes as the ice sinks out of it.
        let lobes = fbm(tables, self.perm, x / F::of(2500), F::ratio(9, 10), z / F::of(2500), 3, half, two);
        // And it sags towards the tower, flaring down to meet it rather than sitting on it.
        let out = (d.0 * d.0 + d.1 * d.1).sqrt() / (F::ratio(3, 2) * self.radius);
        let sag = F::ratio(6, 10) * self.anvil_thickness * (-(out * out)).exp() * (one - smoothstep(back, back + self.radius, -s));
        let thickness = self.anvil_thickness * (one - F::ratio(85, 100) * along.min(one)).powf(F::ratio(3, 2)) * edge * (F::ratio(75, 100) + half * lobes) + sag;
        // A flat top, rising and falling a little.
        let top = self.tropopause + F::of(150) * (fbm(tables, self.perm, x / F::of(15000), F::ratio(7, 10), z / F::of(15000), 2, half, two) - half);
        (top - y).min(y - (top - thickness))
    }

    /// The Worley texture, turned over: 1 at a cell's feature point.
    #[inline]
    fn cell<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        (F::of(1) - worley(tables, self.worley, x, y, z)).clamp(F::of(0), F::of(1))
    }

    /// Cauliflower billows: how far the surface is pushed out at a point (m).
    fn billows<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, scale: F) -> F {
        let half = F::ratio(1, 2);
        let (qx, qy, qz) = (x / scale, y / scale, z / scale);
        let by = F::ratio(35, 100);
        let wx = perlin(tables, self.perm, qx * half, qy * half, qz * half + F::ratio(31, 10)) * by;
        let wy = perlin(tables, self.perm, qx * half + F::ratio(77, 10), qy * half, qz * half) * by;
        let wz = perlin(tables, self.perm, qx * half, qy * half + F::ratio(53, 10), qz * half) * by;
        let (qx, qy, qz) = (qx + wx, qy + wy, qz + wz);
        let off = F::ratio(45, 100);
        let (m, f) = (F::ratio(26, 10), F::ratio(65, 10));
        scale * (F::ratio(55, 100) * (self.cell(tables, qx, qy, qz) - off)
            + F::ratio(25, 100) * (self.cell(tables, qx * m + F::ratio(3, 10), qy * m + F::ratio(1, 10), qz * m + F::ratio(7, 10)) - off)
            + F::ratio(12, 100) * (self.cell(tables, qx * f + F::ratio(9, 10), qy * f + F::ratio(4, 10), qz * f + F::ratio(2, 10)) - off))
    }

    /// Fibrous ice: streaks along the wind (m).
    fn fibres<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, scale: F) -> F {
        let w = self.wind;
        let along = (x * w.0 + z * w.1) / (scale * F::of(6));
        let across = (-x * w.1 + z * w.0) / (scale * F::ratio(7, 10));
        scale * F::ratio(6, 10) * (fbm(tables, self.perm, along, y / (scale * F::ratio(8, 10)), across, 4, F::ratio(55, 100), F::ratio(21, 10)) - F::ratio(1, 2))
    }

    /// The height above which the cloud is all ice.
    pub fn icy_above(&self) -> F {
        self.glaciation + F::of(1000)
    }

    /// The greatest density anywhere.
    pub fn bound(&self) -> F {
        self.density.max(self.ice_density)
    }

    /// The cloud's density as a fraction of the local extinction, and that extinction.
    pub fn fraction<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> (F, F) {
        let (zero, one) = (F::of(0), F::of(1));
        if y < self.bottom || y > self.top {
            return (zero, zero);
        }
        let b = self.billow;
        let margin = b * F::ratio(8, 10);
        let mut depth = -F::far();
        let margin_lobes = margin + F::ratio(4, 10) * self.radius;
        let mut i = 0;
        while i < self.tower_count {
            let t = self.tower(tables, i);
            i += 1;
            if y > t.top + margin_lobes {
                continue;
            }
            if self.from_axis(&t, x, y, z) > F::ratio(165, 100) * t.radius + margin_lobes {
                continue;
            }
            depth = depth.max(self.tower_depth(&t, x, y, z));
        }
        let anvil = if y > self.tropopause - F::ratio(16, 10) * self.anvil_thickness - margin { self.anvil_depth(tables, x, y, z) } else { F::of(-1_000_000_000) };
        if !(depth > -margin_lobes || anvil > -margin) {
            return (zero, zero);
        }
        let ice = smoothstep(self.glaciation - F::of(1000), self.glaciation + F::of(1000), y);
        // Hard cauliflower turrets in the liquid cloud, soft streaks in the ice.
        let surface = if ice < one { self.billows(tables, x, y, z, b) * self.billowing } else { zero };
        let streaks = if ice > zero { self.fibres(tables, x, y, z, b * F::ratio(6, 10)) } else { zero };
        // Turrets: the largest lobes, rising thermals a tower's width across, which the billows
        // then cover.
        let lobes = if depth > -margin - F::ratio(4, 10) * self.radius {
            let s = F::ratio(8, 10) * self.radius;
            let n = fbm(tables, self.perm, x / s, y / s * F::ratio(7, 10), z / s, 2, F::ratio(1, 2), F::of(2));
            F::ratio(3, 10) * self.radius * ((n - F::ratio(4, 10)) * F::of(2)).max(F::ratio(-4, 10)) * lerp(one, F::ratio(4, 10), ice)
        } else {
            zero
        };
        let push = lerp(surface, streaks, ice) + lobes;
        let softness = lerp(self.softness, F::of(6) * self.softness, ice);
        // The anvil is ice throughout, with streaks.
        let anvil = anvil + self.fibres(tables, x, y, z, b * F::ratio(1, 2));
        let inside = (depth + push).max(anvil);
        let f = smoothstep(zero, softness, inside);
        if f <= zero {
            return (zero, zero);
        }
        // A ragged base, as drops evaporate below it.
        let under = smoothstep(self.base - self.softness, self.base + self.softness, y + F::ratio(2, 10) * self.billows(tables, x, y, z, b * F::ratio(3, 10)));
        (f * under, lerp(self.density, self.ice_density, ice.max(if anvil > depth + push { one } else { zero })))
    }

    /// The extinction at a point.
    pub fn extinction<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (f, sigma) = self.fraction(tables, x, y, z);
        f * sigma
    }

    /// A bound on the density from a height along a direction, and how far it holds: clear
    /// below the base and above the top; only ice above the glaciation layer.
    pub fn majorant_from(&self, y: F, dy: F) -> (F, F) {
        let (zero, step) = (F::of(0), F::ratio(1, 100));
        let ahead = y + dy * step;
        let (lo, hi, icy) = (self.bottom, self.top, self.icy_above());
        let (m, len) = if ahead < lo {
            (zero, if dy > zero { (lo - ahead) / dy } else { F::far() })
        } else if ahead > hi {
            (zero, if dy < zero { (hi - ahead) / dy } else { F::far() })
        } else if ahead > icy {
            (self.ice_density, self.to(icy, ahead, dy).min(self.to(hi, ahead, dy)))
        } else {
            (self.bound(), self.to(lo, ahead, dy).min(self.to(icy, ahead, dy)))
        };
        (m, len + step)
    }

    /// How far along a ray at height `from`, climbing at `dy`, the height `y` is ahead; never
    /// if it isn't.
    #[inline]
    fn to(&self, y: F, from: F, dy: F) -> F {
        let t = if dy != F::of(0) { (y - from) / dy } else { F::far() };
        if t > F::of(0) { t } else { F::far() }
    }
}
