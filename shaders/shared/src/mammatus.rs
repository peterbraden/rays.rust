//! Mammatus: pouches hanging from the underside of a thunderstorm's anvil. The density of the
//! renderer's `procedural::mammatus`, which has the account of the method and its sources: an
//! anvil with a ragged edge, a base that undulates and sags, and a lattice of pouches below it.

use crate::noise::{cell_hash, fbm};
use crate::{lerp, smoothstep, Real, Tables};

/// The recipe (the renderer's `Mammatus`, whose fields these are), and where the permutation
/// of its noise is (512 entries).
#[derive(Clone, Copy)]
pub struct Mammatus<F: Real> {
    pub base: F,
    pub thickness: F,
    pub base_variation: F,
    pub base_size: F,
    pub sag: F,
    pub sag_distance: F,
    pub base_softness: F,
    pub top_variation: F,
    pub anvil_density: F,
    pub pouch_density: F,
    pub pouch_size: F,
    pub pouch_depth: F,
    pub coverage: F,
    pub regularity: F,
    pub clustering: F,
    pub cluster_size: F,
    pub edge_sharpness: F,
    pub roughness: F,
    pub detail_size: F,
    pub edge_direction: (F, F),
    pub edge_distance: F,
    pub edge_raggedness: F,
    pub edge_taper: F,
    pub center: (F, F),
    pub extent: F,
    pub fade: F,
    pub pouch_band: F,
    pub seed: u32,
    pub perm: u32,
}

impl<F: Real> Mammatus<F> {
    /// The noise, in [0, 1].
    #[inline]
    fn noise<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, octaves: u32) -> F {
        fbm(tables, self.perm, x, y, z, octaves, F::ratio(1, 2), F::of(2))
    }

    pub fn bottom(&self) -> F {
        self.base - self.base_variation - self.sag - self.pouch_depth * F::ratio(12, 10) - self.roughness
    }

    pub fn top(&self) -> F {
        self.base + self.thickness + self.top_variation
    }

    /// The greatest density anywhere.
    pub fn bound(&self) -> F {
        self.anvil_density.max(self.pouch_density)
    }

    /// The height of the anvil's base at (x, z), `inside` its edge.
    fn local_base<T: Tables<F>>(&self, tables: &T, x: F, z: F, inside: F) -> F {
        let mut b = self.base;
        if self.base_variation > F::of(0) {
            b = b + (self.noise(tables, x / self.base_size, F::ratio(13, 100), z / self.base_size, 3) - F::ratio(1, 2)) * F::of(2) * self.base_variation;
        }
        if self.sag > F::of(0) {
            b = b - self.sag * smoothstep(F::of(0), self.sag_distance, inside);
        }
        b.min(self.top() - F::of(1))
    }

    /// The height of the anvil's top at (x, z).
    fn local_top<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        let t = self.base + self.thickness;
        if self.top_variation <= F::of(0) {
            return t;
        }
        t + (self.noise(tables, x / self.base_size, F::ratio(57, 100), z / self.base_size, 3) - F::ratio(1, 2)) * F::of(2) * self.top_variation
    }

    /// How far inside the anvil's ragged edge (x, z) lies.
    fn inside_edge<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        let s = x * self.edge_direction.0 + z * self.edge_direction.1;
        let across = -x * self.edge_direction.1 + z * self.edge_direction.0;
        let ragged = (self.noise(tables, across / F::of(9000), F::ratio(31, 100), s / F::of(9000), 4) - F::ratio(1, 2)) * F::of(2) * self.edge_raggedness;
        self.edge_distance + ragged - s
    }

    /// For the pouch cell around (x, z): how far out from its middle (0 to 1, where 1 is the
    /// crease with the next), and its relative depth (0 for none).
    fn pouch_at<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> (F, F) {
        let (half, one, two) = (F::ratio(1, 2), F::of(1), F::of(2));
        let row = F::of(3).sqrt() / two;
        let (u, v) = (x / self.pouch_size, z / self.pouch_size);
        let third = F::ratio(3, 10);
        let wu = (fbm(tables, self.perm, u * third, F::ratio(4, 10), v * third, 2, half, two) - half) * F::ratio(7, 10);
        let wv = (fbm(tables, self.perm, u * third, F::ratio(8, 10), v * third, 2, half, two) - half) * F::ratio(7, 10);
        let (u, v) = (u + wu, v + wv);
        let jitter = half * (one - self.regularity.clamp(F::of(0), one));
        let j0 = (v / row).round().to_i32();
        let (mut f1, mut f2, mut nearest) = (F::far(), F::far(), (0i32, 0i32));
        let mut j = j0 - 2;
        while j <= j0 + 2 {
            let shift = if j & 1 == 1 { half } else { F::of(0) };
            let i0 = (u - shift).round().to_i32();
            let mut i = i0 - 2;
            while i <= i0 + 2 {
                let hx: F = cell_hash::<F>(i as u32, j as u32, 0, self.seed) - half;
                let hy: F = cell_hash::<F>(i as u32, j as u32, 0, self.seed.wrapping_add(1)) - half;
                let (cx, cy) = (F::of(i) + shift + two * jitter * hx, F::of(j) * row + two * jitter * hy);
                let d = (cx - u) * (cx - u) + (cy - v) * (cy - v);
                if d < f1 {
                    f2 = f1;
                    f1 = d;
                    nearest = (i, j);
                } else if d < f2 {
                    f2 = d;
                }
                i += 1;
            }
            j += 1;
        }
        let to_edge = (f2.sqrt() - f1.sqrt()) / two;
        let edge = (one - two * to_edge).clamp(F::of(0), one);
        let (i, j) = nearest;
        let holds: F = cell_hash::<F>(i as u32, j as u32, 0, self.seed.wrapping_add(2));
        let deep: F = cell_hash::<F>(i as u32, j as u32, 0, self.seed.wrapping_add(3));
        let depth = if holds < self.coverage { F::ratio(55, 100) + F::ratio(45, 100) * deep } else { F::of(0) };
        // Round rather than polygonal, still meeting the neighbours at the crease.
        (lerp(two * f1.sqrt(), edge, half).min(one), depth)
    }

    /// The fraction of `anvil_density` (above the base) or `pouch_density` (below), in
    /// [0, 1], and which: whether the point is in the anvil.
    fn part<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> (F, bool) {
        let (zero, half, one, two) = (F::of(0), F::ratio(1, 2), F::of(1), F::of(2));
        if y < self.bottom() || y > self.top() {
            return (zero, false);
        }
        let inside = self.inside_edge(tables, x, z);
        if inside <= zero {
            return (zero, false);
        }
        // Rounded and ragged, so that no edge of the region is a ruled line.
        let region = if self.fade > zero {
            let (cx, cz) = (x - self.center.0, z - self.center.1);
            let from_centre = (cx * cx + cz * cz).sqrt() / self.extent;
            let ragged = (self.noise(tables, x / F::of(9000), F::ratio(91, 100), z / F::of(9000), 3) - half) * self.fade;
            one - smoothstep(one - self.fade, one, from_centre + ragged)
        } else {
            one
        };
        if region <= zero {
            return (zero, false);
        }
        let taper = smoothstep(zero, self.edge_taper, inside) * region;
        let ripple = (self.noise(tables, x / self.detail_size, y / self.detail_size, z / self.detail_size, 3) - half) * two;
        let top = self.local_top(tables, x, z);
        let base = self.local_base(tables, x, z, inside).min(top - one);
        if y >= base {
            // The anvil: thinning towards its edge, from the top down, and towards its top, and
            // thickening over its soft underside.
            let depth = top - base;
            let h = (y - base) / depth;
            let top = taper * (F::ratio(75, 100) + F::ratio(25, 100) * ripple);
            if h > one {
                return (zero, true);
            }
            let underside = if self.base_softness > zero {
                smoothstep(zero, self.base_softness / depth, h + F::ratio(3, 10) * ripple * self.base_softness / depth)
            } else {
                one
            };
            return ((one - smoothstep(top - F::ratio(15, 100), top, h)) * (F::ratio(85, 100) + F::ratio(15, 100) * ripple) * underside, true);
        }
        let band = smoothstep(zero, self.pouch_band, inside) * (one - smoothstep(self.pouch_band, F::of(3) * self.pouch_band, inside)).max(F::ratio(3, 10));
        let (t, depth) = self.pouch_at(tables, x, z);
        let clusters = self.noise(tables, x / self.cluster_size, F::ratio(77, 100), z / self.cluster_size, 3);
        let depth = depth * lerp(one, smoothstep(F::ratio(3, 10), F::ratio(65, 100), clusters), self.clustering);
        let hang = self.pouch_depth * depth * band * taper * (one - t * t).max(zero).sqrt();
        // Height above the pouch's lower surface.
        let above = y - (base - hang) + ripple * self.roughness;
        (smoothstep(zero, self.edge_sharpness, above), false)
    }

    /// The fraction of the anvil's or the pouches' density at a point.
    pub fn fraction<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        self.part(tables, x, y, z).0
    }

    /// The extinction at a point.
    pub fn density<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (f, anvil) = self.part(tables, x, y, z);
        if f <= F::of(0) {
            return F::of(0);
        }
        if anvil { self.anvil_density * f } else { self.pouch_density * f }
    }

    /// A bound on the density from a point along a direction, and how far it holds: two
    /// slabs, the pouches' and the anvil's, and nothing beyond the anvil's ragged edge.
    pub fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        let (zero, step) = (F::of(0), F::ratio(1, 100));
        let (ax, ay, az) = (x + dx * step, y + dy * step, z + dz * step);
        // Beyond the furthest the edge reaches: empty until the ray comes back inside it.
        let s = ax * self.edge_direction.0 + az * self.edge_direction.1;
        let ds = dx * self.edge_direction.0 + dz * self.edge_direction.1;
        let limit = self.edge_distance + self.edge_raggedness;
        if s > limit {
            let back = if ds < zero { (s - limit) / -ds } else { F::far() };
            return (zero, back + step);
        }
        let leave = if ds > zero { (limit - s) / ds } else { F::far() };
        let (lo, hi) = (self.bottom(), self.top());
        // With a varying base, the pouches and the anvil share one slab; otherwise each has its
        // own, either side of the base.
        let flat = self.base_variation <= zero && self.sag <= zero;
        let (m, slab) = if ay < lo {
            (zero, if dy > zero { self.to_y(lo, ay, dy) } else { F::far() })
        } else if ay > hi {
            (zero, if dy < zero { self.to_y(hi, ay, dy) } else { F::far() })
        } else if !flat {
            (self.bound(), self.ahead(lo, ay, dy).min(self.ahead(hi, ay, dy)))
        } else if ay < self.base {
            (self.pouch_density, self.ahead(lo, ay, dy).min(self.ahead(self.base, ay, dy)))
        } else {
            (self.anvil_density, self.ahead(self.base, ay, dy).min(self.ahead(hi, ay, dy)))
        };
        (m, slab.min(leave) + step)
    }

    /// How far along a ray at height `from`, climbing at `dy`, the height `y` is.
    #[inline]
    fn to_y(&self, y: F, from: F, dy: F) -> F {
        if dy != F::of(0) { (y - from) / dy } else { F::far() }
    }

    /// `to_y`, if it is ahead; else never.
    #[inline]
    fn ahead(&self, y: F, from: F, dy: F) -> F {
        let t = self.to_y(y, from, dy);
        if t > F::of(0) { t } else { F::far() }
    }
}
