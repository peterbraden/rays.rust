//! Cirrus after Hogan and Kew's fallstreak model: the density of the renderer's
//! `procedural::hogan_kew_cirrus`, which has the account of the method and its sources. Ice
//! generated in cells near the layer's top, traced back along its fall through sheared, veering
//! wind to the cell it left, gathered into fibres, bent by a large-scale flow and by eddies.

use crate::noise::{fbm, perlin};
use crate::{lerp, Real, Tables};

/// Texels along a side of the generating field (a power of two: the field is periodic).
pub const FIELD_SIZE: u32 = 1024;

/// A layer's recipe (the renderer's `HoganKewCirrus`, whose fields these are), and where its
/// tables are: the permutations of the fibres' and patches' noise and of the flow's and
/// eddies' (512 entries each), and the generating field at the layer's time, `FIELD_SIZE`
/// squared texels by rows.
#[derive(Clone, Copy)]
pub struct Cirrus<F: Real> {
    pub base: F,
    pub top: F,
    pub max_density: F,
    pub generating_depth: F,
    pub fall_depth: F,
    pub sublimation: F,
    pub wind: (F, F),
    pub shear: F,
    pub generating_wind: F,
    pub fall_speed: F,
    pub fall_speed_spread: F,
    pub veer: F,
    pub edge: F,
    pub flow_size: F,
    pub flow: F,
    pub fibre_width: F,
    pub fibres: F,
    pub turbulence_size: F,
    pub turbulence: F,
    /// How far the cells have drifted with the wind at the generating level: its speed times
    /// the time.
    pub drift: F,
    pub patchiness: F,
    pub patch_size: F,
    pub center: (F, F),
    pub extent: F,
    pub fade: F,
    /// The generating field's texel, and the value its cells begin at.
    pub texel: F,
    pub threshold: F,
    pub detail_perm: u32,
    pub curl_perm: u32,
    pub field: u32,
}

impl<F: Real> Cirrus<F> {
    /// The drift of ice that has fallen `fallen` below the top, relative to the cell it left,
    /// times its fall speed, (along the wind, across it): the renderer's `wind_integral`.
    pub fn wind_integral(&self, fallen: F) -> (F, F) {
        let (a, s, z) = (self.generating_wind, self.shear, fallen);
        let theta = self.veer / (self.top - self.base);
        let phi = theta * z;
        let (along, across) = if phi.abs() < F::ratio(1, 1_000_000) {
            (a * z - s * z * z / F::of(2), theta * (a * z * z / F::of(2) - s * z * z * z / F::of(3)))
        } else {
            ((a - s * z) * phi.sin() / theta + s * (F::of(1) - phi.cos()) / (theta * theta),
             a / theta - (a - s * z) * phi.cos() / theta - s * phi.sin() / (theta * theta))
        };
        (along - a * z, across)
    }

    /// Where ice at depth `fallen`, now at (`x`, `z`) in the cells' frame, left its cell, for
    /// fall speed `v`.
    fn source(&self, x: F, z: F, fallen: F, v: F) -> (F, F) {
        let i = self.wind_integral(fallen);
        let i = (i.0 / v, i.1 / v);
        let across = (-self.wind.1, self.wind.0);
        (x - self.wind.0 * i.0 - across.0 * i.1, z - self.wind.1 * i.0 - across.1 * i.1)
    }

    /// The curl of the flow's noise about (`x`, `y`, `z`): (∂ψ/∂z, -∂ψ/∂x), which is free of
    /// divergence, so it moves the ice without bunching or thinning it.
    fn curl<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> (F, F) {
        let h = F::ratio(1, 1000);
        let dpsi_dx = (perlin(tables, self.curl_perm, x + h, y, z + F::of(0)) - perlin(tables, self.curl_perm, x + -h, y, z + F::of(0))) / (F::of(2) * h);
        let dpsi_dz = (perlin(tables, self.curl_perm, x + F::of(0), y, z + h) - perlin(tables, self.curl_perm, x + F::of(0), y, z + -h)) / (F::of(2) * h);
        (dpsi_dz, -dpsi_dx)
    }

    /// The large-scale flow's warp of a horizontal position.
    fn warp<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> (F, F) {
        if self.flow <= F::of(0) {
            return (x, z);
        }
        let c = self.curl(tables, x / self.flow_size, F::ratio(37, 10), z / self.flow_size);
        (x + c.0 * self.flow, z + c.1 * self.flow)
    }

    /// The fall speed of the ice at a point of the cells' frame.
    pub fn fall_speed_at<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        if self.fall_speed_spread <= F::of(0) {
            return self.fall_speed;
        }
        let w = self.fibre_width * F::of(3);
        self.fall_speed * (F::of(1) + self.fall_speed_spread * perlin(tables, self.curl_perm, x / w, y / w, z / w).clamp(F::of(-1), F::of(1)))
    }

    /// The generating elements at a source position, in [0, 1].
    fn fibre<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        let (qx, qz) = (x / self.fibre_width, z / self.fibre_width);
        let (mut sum, mut weight, mut amp, mut freq) = (F::of(0), F::of(0), F::of(1), F::of(1));
        let mut k = 0;
        while k < 3 {
            let n = perlin(tables, self.detail_perm, qx * freq, F::ratio(37, 100), qz * freq).max(F::of(0));
            sum = sum + amp * (n * n) * F::of(4);
            weight = weight + amp;
            amp = amp * F::ratio(1, 2);
            freq = freq * F::ratio(22, 10);
            k += 1;
        }
        (sum / weight).min(F::of(1))
    }

    /// The vertical profile at a depth `d` (0 at the top, 1 at the base).
    pub fn profile(&self, d: F) -> F {
        let g = self.generating_depth.min(self.fall_depth);
        if d < g {
            return (d / g).sqrt();
        }
        let x = (d - g) / (self.fall_depth - g).max(F::ratio(1, 1_000_000_000));
        (F::of(1) - x).clamp(F::of(0), F::of(1)).powf(self.sublimation)
    }

    /// The lowest height with any ice.
    pub fn lowest(&self) -> F {
        self.top - self.fall_depth.min(F::of(1)) * (self.top - self.base)
    }

    /// The generating field at (`x`, `z`): bilinear, periodic.
    fn generating<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        let (u, v) = (x / self.texel, z / self.texel);
        let (i0, j0) = (u.floor(), v.floor());
        let (fu, fv) = (u - i0, v - j0);
        let (i0, j0) = (i0.to_i32(), j0.to_i32());
        let mask = FIELD_SIZE as i32 - 1;
        let (i1, j1) = ((i0.wrapping_add(1) & mask) as u32, (j0.wrapping_add(1) & mask) as u32);
        let (i0, j0) = ((i0 & mask) as u32, (j0 & mask) as u32);
        let a = lerp(tables.value(self.field + j0 * FIELD_SIZE + i0), tables.value(self.field + j0 * FIELD_SIZE + i1), fu);
        let b = lerp(tables.value(self.field + j1 * FIELD_SIZE + i0), tables.value(self.field + j1 * FIELD_SIZE + i1), fu);
        lerp(a, b, fv)
    }

    /// The cells' strength (0 to 1) at the generating level, in their own drifting frame.
    pub fn cells<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        ((self.generating(tables, x, z) - self.threshold) / self.edge).clamp(F::of(0), F::of(1))
    }

    /// For a point at fractional depth `d`, `fallen` below the top: itself in the cells' frame
    /// (x, z), and where its ice left its cell, after the eddies, the fall and the flow.
    fn traced<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, fallen: F, d: F) -> ((F, F), (F, F)) {
        let (fx, fz) = (x - self.wind.0 * self.drift, z - self.wind.1 * self.drift);
        let (mut ex, mut ez) = (fx, fz);
        if self.turbulence > F::of(0) {
            let s = self.turbulence_size;
            let c = self.curl(tables, fx / s, y / s, fz / s);
            let by = self.turbulence * d;
            ex = ex + c.0 * by;
            ez = ez + c.1 * by;
        }
        let fall_speed = self.fall_speed_at(tables, fx, y, fz);
        let from = self.source(ex, ez, fallen, fall_speed);
        ((fx, fz), self.warp(tables, from.0, from.1))
    }

    /// How mature the ice at a point is: its cell's strength where it left it (for the size
    /// of its crystals, which the renderer asks).
    pub fn maturity<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let fallen = (self.top - y).clamp(F::of(0), self.top - self.base);
        let d = fallen / (self.top - self.base);
        let (_, source) = self.traced(tables, x, y, z, fallen, d);
        self.cells(tables, source.0, source.1)
    }

    /// Ice density as a fraction of `max_density`, in [0, 1].
    pub fn fraction<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (zero, one) = (F::of(0), F::of(1));
        if y < self.base || y > self.top {
            return zero;
        }
        let fallen = self.top - y;
        let d = fallen / (self.top - self.base);
        let profile = self.profile(d);
        if profile <= zero {
            return zero;
        }
        let edge_fade = {
            let from_centre = (x - self.center.0).abs().max((z - self.center.1).abs()) / self.extent;
            let t = ((one - from_centre) / self.fade.max(F::ratio(1, 1_000_000_000))).clamp(zero, one);
            t * t * (F::of(3) - F::of(2) * t)
        };
        if edge_fade <= zero {
            return zero;
        }
        let (framed, source) = self.traced(tables, x, y, z, fallen, d);
        let cells = self.cells(tables, source.0, source.1);
        if cells <= zero {
            return zero;
        }
        let detail = one - self.fibres * (one - self.fibre(tables, source.0, source.1));
        let patches = if self.patchiness > zero {
            let n = fbm(tables, self.detail_perm, framed.0 / self.patch_size, F::ratio(35, 2), framed.1 / self.patch_size, 3, F::ratio(1, 2), F::of(2));
            one - self.patchiness * (one - ((n - F::ratio(4, 10)) * F::of(5)).clamp(zero, one))
        } else {
            one
        };
        (edge_fade * profile * cells * detail * patches).clamp(zero, one)
    }

    /// A bound on the density from a point along a direction, and how far it holds: nothing
    /// below `lowest`, until the ray climbs to it.
    pub fn majorant_from(&self, y: F, dy: F) -> (F, F) {
        let lowest = self.lowest();
        let to_lowest = if dy != F::of(0) { (lowest - y) / dy } else { F::far() };
        let ahead = y + dy * F::ratio(1, 1_000_000_000) * (self.top - self.base);
        let length = if to_lowest > F::of(0) { to_lowest } else { F::far() };
        if ahead < lowest { (F::of(0), length) } else { (self.max_density, length) }
    }
}
