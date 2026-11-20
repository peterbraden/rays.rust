//! The Nubis clouds' density (the renderer's `procedural::nubis_clouds`, which has the account
//! of the method and its sources): a height profile by cloud type, a base shape of Perlin
//! noise carved by Worley cells and cut to the weather's strength, and edges eroded by finer
//! cells. The weather map is the caller's: a flat layer's and a planet's differ.

use crate::media::Medium;
use crate::noise::fbm;
use crate::{lerp, remap, smoothstep, Real, Tables};

/// The Worley texture's size: a tile of `WORLEY_PERIOD` cells a side, `WORLEY_PER_CELL` texels
/// a cell, which repeats.
pub const WORLEY_PERIOD: u32 = 16;
pub const WORLEY_PER_CELL: u32 = 4;
pub const WORLEY_N: u32 = WORLEY_PERIOD * WORLEY_PER_CELL;

/// A cloud layer's recipe, and where its tables are: the shape noise's permutation (512
/// entries) and the Worley texture (`WORLEY_N` cubed texels).
#[derive(Clone, Copy)]
pub struct Nubis<F: Real> {
    pub base: F,
    pub top: F,
    /// Average fraction of the sky covered.
    pub coverage: F,
    /// 0 stratus, 0.5 stratocumulus, 1 cumulus.
    pub cloud_type: F,
    pub shape_size: F,
    pub detail_size: F,
    pub erosion: F,
    pub perm: u32,
    pub worley: u32,
}

/// Fractal Perlin noise is in [0, 1] but rarely strays far from 0.5; stretch it to use the range.
#[inline]
pub fn stretched<F: Real>(n: F) -> F {
    ((n - F::ratio(1, 2)) * F::ratio(22, 10) + F::ratio(1, 2)).clamp(F::of(0), F::of(1))
}

/// Where in the layer (0 at its base, 1 at its top) clouds of a type rise and fall: density
/// grows over `rise` and dies away over `fall`. (rise.0, rise.1, fall.0, fall.1), from stratus
/// (type 0) through stratocumulus (0.5) to cumulus (1).
pub fn profile_of<F: Real>(cloud_type: F) -> (F, F, F, F) {
    let stratus = (F::of(0), F::ratio(8, 100), F::ratio(15, 100), F::ratio(3, 10));
    let stratocumulus = (F::of(0), F::ratio(12, 100), F::ratio(35, 100), F::ratio(65, 100));
    let cumulus = (F::of(0), F::ratio(8, 100), F::ratio(6, 10), F::of(1));
    let low = cloud_type < F::ratio(1, 2);
    let (a, b) = if low { (stratus, stratocumulus) } else { (stratocumulus, cumulus) };
    let t = if low { cloud_type * F::of(2) } else { cloud_type * F::of(2) - F::of(1) };
    (lerp(a.0, b.0, t), lerp(a.1, b.1, t), lerp(a.2, b.2, t), lerp(a.3, b.3, t))
}

/// The fraction of cloud density allowed at height `h` through the layer.
pub fn height_profile<F: Real>(h: F, cloud_type: F) -> F {
    let (rise0, rise1, fall0, fall1) = profile_of(cloud_type);
    smoothstep(rise0, rise1, h) * (F::of(1) - smoothstep(fall0, fall1, h))
}

/// The Worley texture at `p` (in cells): the distance to the nearest feature point,
/// interpolated between texels, the tile repeating.
pub fn worley<F: Real, T: Tables<F>>(tables: &T, texture: u32, x: F, y: F, z: F) -> F {
    let n = WORLEY_N as i32;
    let per = F::of(WORLEY_PER_CELL as i32);
    let (qx, qy, qz) = (x * per, y * per, z * per);
    let (x0, y0, z0) = (qx.floor(), qy.floor(), qz.floor());
    let (fx, fy, fz) = (qx - x0, qy - y0, qz - z0);
    let (x0, y0, z0) = (x0.to_i32(), y0.to_i32(), z0.to_i32());
    // (n is a power of two: masking is the remainder that is never negative.)
    let t = |dx: i32, dy: i32, dz: i32| {
        let (x, y, z) = ((x0 + dx) & (n - 1), (y0 + dy) & (n - 1), (z0 + dz) & (n - 1));
        tables.value(texture + (z * n * n + y * n + x) as u32)
    };
    let near = lerp(lerp(t(0, 0, 0), t(1, 0, 0), fx), lerp(t(0, 1, 0), t(1, 1, 0), fx), fy);
    let far = lerp(lerp(t(0, 0, 1), t(1, 0, 1), fx), lerp(t(0, 1, 1), t(1, 1, 1), fx), fy);
    lerp(near, far, fz)
}

/// Inverted Worley noise (high at feature points, falling away between them), in three
/// octaves: round, billowing cells.
pub fn worley_fbm<F: Real, T: Tables<F>>(tables: &T, texture: u32, x: F, y: F, z: F) -> F {
    let (zero, one, two, four) = (F::of(0), F::of(1), F::of(2), F::of(4));
    let a = (one - worley(tables, texture, x, y, z)).clamp(zero, one);
    let b = (one - worley(tables, texture, x * two, y * two, z * two)).clamp(zero, one);
    let c = (one - worley(tables, texture, x * four, y * four, z * four)).clamp(zero, one);
    F::ratio(625, 1000) * a + F::ratio(1, 4) * b + F::ratio(1, 8) * c
}

impl<F: Real> Nubis<F> {
    /// How far up the layer `altitude` is, if it is in it: 0 at the base, 1 at the top.
    #[inline]
    pub fn height(&self, altitude: F) -> Option<F> {
        let h = (altitude - self.base) / (self.top - self.base);
        if h >= F::of(0) && h <= F::of(1) { Some(h) } else { None }
    }

    /// The column strength (0 clear, to 1) for a weather map value.
    #[inline]
    pub fn cell(&self, weather: F) -> F {
        remap(weather, F::of(1) - self.coverage, F::of(1))
    }

    /// How far up the layer clouds in a column of `cell` strength may reach: stronger cells
    /// grow taller for heaped (cumulus) clouds; layered (stratus) ones stay level.
    #[inline]
    pub fn reach(&self, cell: F) -> F {
        lerp(F::of(1), lerp(F::ratio(3, 10), F::of(1), cell.sqrt()), self.cloud_type)
    }

    /// Cloud density as a fraction of the densest, in [0, 1]: at height `h` in the layer
    /// (`height`), under weather `weather`, with the shape and detail noise taken at `p`.
    pub fn fraction<T: Tables<F>>(&self, tables: &T, h: F, weather: F, x: F, y: F, z: F) -> F {
        let (zero, one) = (F::of(0), F::of(1));
        // The weather map: how strongly this column of sky is cloud.
        let cell = self.cell(weather);
        if cell <= zero {
            return zero;
        }
        let profile = height_profile(h / self.reach(cell), self.cloud_type);
        if profile <= zero {
            return zero;
        }
        // The base shape, cut to the cell's strength.
        let (qx, qy, qz) = (x / self.shape_size, y / self.shape_size, z / self.shape_size);
        let perlin = stretched(fbm(tables, self.perm, qx, qy, qz, 4, F::ratio(1, 2), F::of(2)));
        let perlin_worley = remap(perlin, worley_fbm(tables, self.worley, qx, qy, qz) - one, one);
        let shape = remap(perlin_worley * profile, F::ratio(7, 10) * (one - cell), one);
        if shape <= zero {
            return zero;
        }
        // Erode the edges: billowing detail high up, its inverse (wisps) near the base.
        let d = worley_fbm(tables, self.worley, x / self.detail_size, y / self.detail_size, z / self.detail_size);
        let detail = lerp(one - d, d, (h * F::of(5)).min(one));
        remap(shape, detail * self.erosion, one)
    }
}

/// A weather map: noise sampled at the corners of a grid of `n` by `n` square texels from
/// (`x`, `z`), interpolated between them, and zero outside. In its tables, the (n + 1)^2
/// corners at `corners`, and at `maxima` the largest corner of each texel: the most the map
/// reaches anywhere over it.
#[derive(Clone, Copy)]
pub struct WeatherGrid<F: Real> {
    pub x: F,
    pub z: F,
    pub texel: F,
    pub n: u32,
    pub corners: u32,
    pub maxima: u32,
}

impl<F: Real> WeatherGrid<F> {
    /// The texel holding (x, z), and the position within it in [0, 1]^2; None outside the map.
    pub fn locate(&self, x: F, z: F) -> Option<(u32, u32, F, F)> {
        let u = (x - self.x) / self.texel;
        let v = (z - self.z) / self.texel;
        let n = F::of(self.n as i32);
        if !(u >= F::of(0) && v >= F::of(0) && u <= n && v <= n) {
            return None;
        }
        let i = (u.floor().to_i32() as u32).min(self.n - 1);
        let j = (v.floor().to_i32() as u32).min(self.n - 1);
        Some((i, j, u - F::of(i as i32), v - F::of(j as i32)))
    }

    pub fn at<T: Tables<F>>(&self, tables: &T, x: F, z: F) -> F {
        match self.locate(x, z) {
            None => F::of(0),
            Some((i, j, fu, fv)) => {
                let row = self.n + 1;
                let c = |i: u32, j: u32| tables.value(self.corners + j * row + i);
                lerp(lerp(c(i, j), c(i + 1, j), fu), lerp(c(i, j + 1), c(i + 1, j + 1), fu), fv)
            },
        }
    }
}

/// How far along a ray the coordinate `p`, moving at `d`, leaves the cell from `lo` to
/// `lo + size`.
#[inline]
fn leaves<F: Real>(p: F, d: F, lo: F, size: F) -> F {
    if d > F::of(0) { (lo + size - p) / d } else if d < F::of(0) { (lo - p) / d } else { F::far() }
}

/// A flat layer of Nubis cloud as a medium: its recipe, its densest extinction, and its
/// weather map.
#[derive(Clone, Copy)]
pub struct NubisField<F: Real> {
    pub recipe: Nubis<F>,
    pub max_density: F,
    pub weather: WeatherGrid<F>,
}

impl<F: Real> NubisField<F> {
    /// The highest any cloud can be over weather of at most `weather`, or None if none can be.
    /// The cell strength and reach only grow with the weather, and the profile is zero from
    /// its fall's end of the reach up.
    pub fn column_top(&self, weather: F) -> Option<F> {
        let cell = self.recipe.cell(weather);
        if cell <= F::of(0) {
            return None;
        }
        let h = (self.recipe.reach(cell) * profile_of(self.recipe.cloud_type).3).min(F::of(1));
        Some(self.recipe.base + h * (self.recipe.top - self.recipe.base))
    }

    /// The extinction at a point.
    pub fn density<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        match self.recipe.height(y) {
            Some(h) => self.max_density * self.recipe.fraction(tables, h, self.weather.at(tables, x, z), x, y, z),
            None => F::of(0),
        }
    }

    /// Within one texel of the weather map, clouds lie only below `column_top` of its strongest
    /// corner: there the bound is the densest extinction, elsewhere zero. Holds to the texel's
    /// edge, or to that height if the ray crosses it first.
    pub fn majorant_from<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        let map = &self.weather;
        // Classify by a point a centimetre along the ray, so one on a texel's edge (or the
        // clouds' top) counts as where it is entering.
        let near = F::ratio(1, 100);
        let (ax, ay, az) = (x + dx * near, y + dy * near, z + dz * near);
        let (i, j) = match map.locate(ax, az) {
            Some((i, j, _, _)) => (i, j),
            None => return (self.max_density, F::far()),
        };
        let (x0, z0) = (map.x + F::of(i as i32) * map.texel, map.z + F::of(j as i32) * map.texel);
        let edge = leaves(x, dx, x0, map.texel).min(leaves(z, dz, z0, map.texel)).max(F::of(0));
        match self.column_top(tables.value(map.maxima + j * map.n + i)) {
            None => (F::of(0), edge),
            Some(top) => {
                let to_top = if dy != F::of(0) { (top - y) / dy } else { F::far() };
                let length = if to_top > F::of(0) { edge.min(to_top) } else { edge };
                // Above the clouds: clear until the ray comes down to them.
                if ay > top { (F::of(0), length) } else { (self.max_density, length) }
            },
        }
    }
}

/// A layer with its tables: what a walk through it takes (`media::Medium`).
pub struct NubisMedium<'a, F: Real, T: Tables<F>>(pub &'a NubisField<F>, pub &'a T);

impl<F: Real, T: Tables<F>> Medium<F> for NubisMedium<'_, F, T> {
    #[inline]
    fn density(&self, x: F, y: F, z: F) -> F {
        self.0.density(self.1, x, y, z)
    }

    #[inline]
    fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        self.0.majorant_from(self.1, x, y, z, dx, dy, dz)
    }
}
