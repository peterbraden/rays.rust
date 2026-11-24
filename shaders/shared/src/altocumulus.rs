//! Altocumulus as Rayleigh-Bénard convection cells: the density of the renderer's
//! `procedural::benard_altocumulus`, which has the account of the method and its sources. A
//! lattice of cells, a domed cloudlet in each, waves across the wind, patches, eroded edges and
//! castellanus turrets.

use crate::clouds::{stretched, worley_fbm, WeatherGrid};
use crate::media::Medium;
use crate::noise::{cell_hash, fbm};
use crate::{lerp, remap, smoothstep, Real, Tables};

/// A layer's recipe (the renderer's `BenardAltocumulus`, whose fields these are), and where its
/// tables are: the permutations of the noise that warps the lattice and of the patches' (512
/// entries each), and the Worley texture of the detail.
#[derive(Clone, Copy)]
pub struct Benard<F: Real> {
    pub base: F,
    pub thickness: F,
    pub max_density: F,
    pub cell_size: F,
    pub coverage: F,
    pub regularity: F,
    pub anisotropy: F,
    pub wind: (F, F),
    pub undulation: F,
    pub wavelength: F,
    pub patchiness: F,
    pub patch_size: F,
    pub warp: F,
    pub lumpiness: F,
    pub detail_size: F,
    pub erosion: F,
    pub center: (F, F),
    pub extent: F,
    pub fade: F,
    pub castellanus: F,
    pub turret_height: F,
    pub seed: u32,
    pub warp_perm: u32,
    pub patch_perm: u32,
    pub worley: u32,
    /// A map of how high cloud can be over each of its texels (`majorant_from`): the grid,
    /// with the texels' tops at its `maxima` (below the layer where a texel holds none), or
    /// no texels (`n` 0) for none, and the whole slab is taken as cloud.
    pub tops: WeatherGrid<F>,
}

/// Leave a cell's edge by: how far along a ray the coordinate `p`, moving at `d`, leaves the
/// cell from `lo` to `lo + size`.
#[inline]
fn leaves<F: Real>(p: F, d: F, lo: F, size: F) -> F {
    if d > F::of(0) { (lo + size - p) / d } else if d < F::of(0) { (lo - p) / d } else { F::far() }
}

/// A column of the layer that holds cloud (`Benard::column`): where it is in the warped
/// lattice, its cell, its cloudlet's reach and how far out in it the column is, and how far
/// its cloudlet's base is raised or lowered, in layer thicknesses.
struct Column<F: Real> {
    u: F,
    v: F,
    cell: CellPoint<F>,
    reach: F,
    t: F,
    offset: F,
}

/// Where a point lies among the cells: `edge` from 0 at its cell's centre to 1 at its edge,
/// `radial` its distance from the centre in half-spacings, and two numbers in [0, 1]
/// particular to the cell.
struct CellPoint<F: Real> {
    edge: F,
    radial: F,
    hash: (F, F),
}

impl<F: Real> Benard<F> {
    /// The heights between which there can be cloud: bases are raised or lowered by up to a
    /// tenth of the thickness per cell, and by a third of it on the waves.
    pub fn bottom(&self) -> F {
        self.base - self.thickness * (F::ratio(1, 10) + F::ratio(3, 10) * self.undulation)
    }

    pub fn top(&self) -> F {
        let turrets = if self.castellanus > F::of(0) { self.turret_height * F::ratio(12, 10) } else { F::of(0) };
        self.base + self.thickness * (F::ratio(15, 10) + F::ratio(3, 10) * self.undulation).max(turrets + F::ratio(1, 2))
    }

    /// The cell around (`u`, `v`), in lattice units: centres a unit apart, rows √3/2.
    fn cell_at(&self, u: F, v: F) -> CellPoint<F> {
        let (half, one, two) = (F::ratio(1, 2), F::of(1), F::of(2));
        let row = F::of(3).sqrt() / two;
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
        // Half the gap between the nearest two centres' distances is (nearly) the distance to
        // the boundary between them: 0.5 at the middle of a regular cell.
        let to_edge = (f2.sqrt() - f1.sqrt()) / two;
        let (i, j) = nearest;
        CellPoint {
            edge: (one - two * to_edge).clamp(F::of(0), one),
            radial: two * f1.sqrt(),
            hash: (cell_hash(i as u32, j as u32, 0, self.seed.wrapping_add(2)), cell_hash(i as u32, j as u32, 0, self.seed.wrapping_add(3))),
        }
    }

    /// What a column of the layer holds, by where it is among the cells: nothing in the gaps
    /// between cloudlets or where the layer thins away (None); else its cloudlet's reach
    /// towards its cell's edge and how far out in it the column is (`t`, under 1). With
    /// `least`, a cloudlet's reach is at least that: a cloudlet no smaller than the real one,
    /// for bounding where cloud can be (`column_top`); the density takes none.
    fn column<T: Tables<F>>(&self, tables: &T, x: F, z: F, least: F) -> Option<Column<F>> {
        let (zero, half, one, two) = (F::of(0), F::ratio(1, 2), F::of(1), F::of(2));
        let from_centre = (x - self.center.0).abs().max((z - self.center.1).abs()) / self.extent;
        let edge_fade = smoothstep(zero, self.fade.max(F::ratio(1, 1_000_000_000)), one - from_centre);
        if edge_fade <= zero {
            return None;
        }
        // 4. Patches.
        let n = stretched(fbm(tables, self.patch_perm, x / self.patch_size, half, z / self.patch_size, 3, half, two));
        let patches = lerp(one, smoothstep(F::ratio(3, 10), F::ratio(6, 10), n), self.patchiness);
        let strength = edge_fade * patches;
        if strength <= zero {
            return None;
        }
        // 1. The cells, in the wind's frame, warped.
        let (along, across) = (x * self.wind.0 + z * self.wind.1, -x * self.wind.1 + z * self.wind.0);
        let (u, v) = (along / (self.cell_size * self.anisotropy), across / self.cell_size);
        let s = F::ratio(35, 100);
        let warp_u = (fbm(tables, self.warp_perm, u * s, F::ratio(1, 4), v * s, 2, half, two) - half) * two * self.warp;
        let warp_v = (fbm(tables, self.warp_perm, u * s, F::ratio(3, 4), v * s, 2, half, two) - half) * two * self.warp;
        let (u, v) = (u + warp_u, v + warp_v);
        // 3. Waves across the wind, their crests wandering a little.
        let wave = if self.undulation > zero {
            let k = F::ratio(3, 10);
            let wander = (fbm(tables, self.warp_perm, along / self.wavelength * k, F::ratio(1, 10), across / self.wavelength * k, 2, half, two) - half) * F::of(4);
            half + half * (F::pi() * two * along / self.wavelength + wander).cos()
        } else {
            one
        };
        let cell = self.cell_at(u, v);
        // 2. This cell's cloudlet: its reach towards the edge, and how far out we are in it.
        let reach = self.coverage * strength * lerp(one, wave, self.undulation) * (F::ratio(8, 10) + F::ratio(4, 10) * cell.hash.0);
        let reach = if least > reach { least } else { reach };
        if reach <= zero {
            return None;
        }
        // Round, not the cell's polygon (which only keeps neighbours apart), with a lumpy
        // outline.
        let f = F::ratio(25, 10);
        let lumps = one + (fbm(tables, self.warp_perm, u * f, F::ratio(6, 10), v * f, 2, half, two) - half) * two * self.lumpiness;
        let t = lerp(cell.radial, cell.edge, F::ratio(35, 100)) * lumps / reach;
        if t >= one {
            return None;
        }
        // How far this cell's cloudlet is raised or lowered, and on the waves.
        let offset = (cell.hash.1 - half) * F::ratio(2, 10) + (wave - half) * F::ratio(6, 10) * self.undulation;
        Some(Column { u, v, cell, reach, t, offset })
    }

    /// The turret of a column's cell, if it has one: how tall it grows, in layer thicknesses,
    /// and how far out in it the column is (1.4 is its edge at its broadest).
    fn turret(&self, c: &Column<F>) -> Option<(F, F)> {
        let half = F::ratio(1, 2);
        let chosen = c.cell.hash.0 * F::ratio(731, 100) + c.cell.hash.1 * F::ratio(317, 100);
        let chosen = chosen - chosen.floor();
        if chosen < self.castellanus {
            let rise = self.turret_height * (half + half * (chosen / self.castellanus.max(F::ratio(1, 1_000_000_000))));
            let width = F::ratio(65, 100) * (F::ratio(85, 100) + F::ratio(3, 10) * c.cell.hash.1);
            let r = c.t / width;
            if r < F::ratio(14, 10) {
                return Some((rise, r));
            }
        }
        None
    }

    /// The highest cloud can be in the column at (x, z), or None if it holds none: its
    /// cloudlet's dome, or its turret's top at its tallest (its billows at their most),
    /// for a cloudlet reaching at least `least` (so one no smaller than the real one: what
    /// the map of tops is sampled from, `majorant_from`).
    pub fn column_top<T: Tables<F>>(&self, tables: &T, x: F, z: F, least: F) -> Option<F> {
        let (one, two) = (F::of(1), F::of(2));
        let c = self.column(tables, x, z, least)?;
        // (The dome's own turrets, as `fraction` has them: they vary across the layer, not up it.)
        let ds = two * self.detail_size;
        let turrets = worley_fbm(tables, self.worley, x / ds, self.base / ds, z / ds);
        let dome = (one - c.t * c.t).sqrt() * c.reach.min(one).sqrt() * (F::ratio(6, 10) + F::ratio(8, 10) * turrets);
        let top = match self.turret(&c) {
            Some((rise, _)) => dome.max(rise * F::ratio(115, 100)),
            None => dome,
        };
        Some(self.base + (c.offset + top) * self.thickness)
    }

    /// Cloud density as a fraction of `max_density`, in [0, 1].
    pub fn fraction<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F) -> F {
        let (zero, half, one, two) = (F::of(0), F::ratio(1, 2), F::of(1), F::of(2));
        if y < self.bottom() || y > self.top() {
            return zero;
        }
        let Some(column) = self.column(tables, x, z, zero) else { return zero };
        let (u, v, cell, reach, t) = (column.u, column.v, &column.cell, column.reach, column.t);
        let offset = column.offset;
        let h = (y - self.base) / self.thickness - offset;
        // A dome, smaller for smaller cloudlets and with turrets on top, over a base that sags
        // towards the edge.
        let ds = two * self.detail_size;
        let turrets = worley_fbm(tables, self.worley, x / ds, self.base / ds, z / ds);
        let top = (one - t * t).sqrt() * reach.min(one).sqrt() * (F::ratio(6, 10) + F::ratio(8, 10) * turrets);
        let bottom = F::ratio(15, 100) * t * t;
        let mut inside = (h - bottom).min(top - h);
        // 6. A turret, on some cells: narrower than the cloudlet, rising from it with a round,
        // billowing head.
        if let Some((rise, r)) = self.turret(&column) {
            // A stem rising to a broader, billowing head, with bulges up its flanks: a
            // little cumulus on the layer. Billows a third of the turret across push its
            // outline out and in; its top is rounded over the head.
            let up = (h / rise).clamp(zero, one);
            let bs = F::ratio(16, 10) * self.detail_size;
            let billows = worley_fbm(tables, self.worley, x / bs, y * F::ratio(8, 10) / bs, z / bs);
            let bulges = (fbm(tables, self.warp_perm, u * F::of(3), y / (F::ratio(8, 10) * self.thickness), v * F::of(3), 2, half, two) - half) * two;
            let over = ((up - F::ratio(3, 4)) / F::ratio(1, 4)).max(zero);
            let profile = lerp(F::ratio(55, 100), one, smoothstep(F::ratio(3, 10), F::ratio(3, 4), up)) * (one - over * over).max(zero).sqrt();
            let radius = profile * (one + F::ratio(1, 4) * bulges) + F::ratio(45, 100) * (billows - half);
            let flank = (radius - r) * two;
            inside = inside.max((h - bottom).min(rise * (F::ratio(85, 100) + F::ratio(3, 10) * billows) - h).min(flank));
        }
        if inside <= zero {
            return zero;
        }
        // Neighbours stay apart: the sinking air at the boundary is clear.
        let body = smoothstep(zero, F::ratio(3, 10), inside) * smoothstep(zero, F::ratio(4, 10), one - t) * smoothstep(zero, F::ratio(2, 10), one - cell.edge);
        // 5. Erode the edges.
        let d = worley_fbm(tables, self.worley, x / self.detail_size, y / self.detail_size, z / self.detail_size);
        remap(body, (one - d) * self.erosion, one)
    }

    /// The layer is a slab: `max_density` within it, nothing above or below.
    fn slab(&self, y: F, dy: F) -> (F, F) {
        let (lo, hi) = (self.bottom(), self.top());
        let zero = F::of(0);
        let to_lo = if dy != zero { (lo - y) / dy } else { F::far() };
        let to_hi = if dy != zero { (hi - y) / dy } else { F::far() };
        if y < lo {
            (zero, if dy > zero { to_lo } else { F::far() })
        } else if y > hi {
            (zero, if dy < zero { to_hi } else { F::far() })
        } else {
            (self.max_density, if dy > zero { to_hi } else if dy < zero { to_lo } else { F::far() })
        }
    }

    /// A bound on the density from a point along a unit direction, and how far it holds.
    /// The slab's, and within it the map's (`tops`): over one of its texels cloud lies only
    /// below the texel's top, so above that, and over a texel that holds none, the air is
    /// clear as far as the texel's edge (or, coming down, as far as the top). Most of the
    /// slab is: the cloudlets are domes with gaps between, and the slab is as tall as the
    /// tallest turret. Without this a walk asked for the density, which is a dozen noise
    /// lookups, at every step of the slab's worth, nine in ten of them for nothing.
    pub fn majorant_from<T: Tables<F>>(&self, tables: &T, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        let zero = F::of(0);
        let (bound, length) = self.slab(y, dy);
        let map = &self.tops;
        if bound <= zero || map.n == 0 {
            return (bound, length);
        }
        // (Beyond the map is beyond the layer's extent, where it has faded to nothing.)
        let (i, j) = match map.entering(x, z, dx, dz) {
            Some(cell) => cell,
            None => return (zero, length),
        };
        let (x0, z0) = (map.x + F::of(i as i32) * map.texel, map.z + F::of(j as i32) * map.texel);
        let edge = leaves(x, dx, x0, map.texel).min(leaves(z, dz, z0, map.texel)).max(zero).min(length);
        let top = tables.value(map.maxima + j * map.n + i);
        if top <= self.bottom() {
            return (zero, edge);
        }
        // As a Nubis layer's column (`clouds::NubisField::majorant_from`): a point at the top
        // counts as where the ray is going.
        let ay = y + dy * F::ratio(1, 100);
        let to_top = if dy != zero { (top - y) / dy } else { F::far() };
        let length = if to_top > zero { edge.min(to_top) } else { edge };
        if ay > top { (zero, length) } else { (self.max_density, length) }
    }
}

/// A layer with its tables: what a walk through it takes (`media::Medium`).
pub struct BenardMedium<'a, F: Real, T: Tables<F>>(pub &'a Benard<F>, pub &'a T);

impl<F: Real, T: Tables<F>> Medium<F> for BenardMedium<'_, F, T> {
    #[inline]
    fn density(&self, x: F, y: F, z: F) -> F {
        self.0.max_density * self.0.fraction(self.1, x, y, z)
    }

    #[inline]
    fn bound(&self) -> F {
        self.0.max_density
    }

    #[inline]
    fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
        self.0.majorant_from(self.1, x, y, z, dx, dy, dz)
    }
}
