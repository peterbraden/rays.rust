//! Altocumulus (and the cellular forms of stratocumulus) as Rayleigh-Bénard convection cells:
//! a density field for `volume::Volume`.
//!
//! A thin layer of cloud a few kilometres up, cooled at its top by radiating to space and
//! warmed below, overturns in cells: air rises in their middles, where it condenses into a
//! rounded cloudlet, and sinks around their edges, where it dries and leaves clear gaps. Like
//! the convection cells Bénard saw in a heated layer of fluid, they tile the layer in rough
//! hexagons a few times as wide as it is deep (B. W. Atkinson and J. W. Zhang, "Mesoscale
//! shallow convection in the atmosphere", Reviews of Geophysics 34, 1996). Seen from the ground
//! the cloudlets of altocumulus are 1° to 5° across (WMO International Cloud Atlas), so a
//! few hundred metres at 3-6 km: smaller ones are cirrocumulus, larger stratocumulus.
//!
//! The field here is built from that picture:
//!
//! 1. **Cells.** A hexagonal lattice of cell centres, each jittered by `1 - regularity`, its
//!    coordinates warped a little by Perlin noise so the cells aren't ruled lines, and
//!    stretched along the wind by `anisotropy`: in shear, cells draw out into rows. The
//!    distance to the nearest cell boundary, half the difference of the distances to the
//!    nearest two centres (Worley's F2 - F1), places each point between its cell's centre and
//!    edge.
//! 2. **Cloudlets.** Each cell holds a cloudlet reaching a fraction of the way to its edge,
//!    larger with `coverage` (past about 1 they merge into a sheet with holes at the cells'
//!    corners: perlucidus), and varying a little from cell to cell. In height it is a dome:
//!    flat-bottomed, sagging slightly towards its edge, and highest in the middle where the air
//!    rises, with turrets (Worley noise) on top.
//! 3. **Waves.** With `undulation`, gravity waves across the wind, `wavelength` apart, raise and
//!    grow the cloudlets along their crests and thin them in the troughs: undulatus, the
//!    rippled sky.
//! 4. **Patches.** Large-scale noise thins the layer in places (`patchiness`), into the patches
//!    and holes of a real sheet.
//! 5. **Detail.** As in Nubis, Worley noise erodes each cloudlet's edges.

use crate::na::{Vector2, Vector3};
use std::sync::OnceLock;
use crate::noise::{PerlinNoise, worley_hash};
use crate::procedural::nubis_clouds::{lerp, remap, smoothstep, stretched, worley_fbm, WorleyTexture};
use crate::volume::DensityField;

/// Rows of the hexagonal lattice are this far apart, for centres a unit apart.
const ROW: f64 = 0.866_025_403_784_438_6;

pub struct BenardAltocumulus {
    /// Height of the cloudlets' bases, and how deep they are in the middle.
    pub base: f64,
    pub thickness: f64,
    /// Extinction coefficient in the densest cloud: by default 0.15 per metre, an optical
    /// depth of about 30 through a cloudlet's middle (thick altocumulus), so that seen from
    /// below the middles are grey and the edges bright.
    pub max_density: f64,
    /// Distance between neighbouring cells' centres (across the wind).
    pub cell_size: f64,
    /// How far each cloudlet reaches towards its cell's edge: around 0.5 broken, 1 joined.
    pub coverage: f64,
    /// 1 for a perfect hexagonal lattice, 0 for cells placed at random.
    pub regularity: f64,
    /// How much longer the cells are along the wind than across it.
    pub anisotropy: f64,
    /// Unit [x, z] direction of the wind (and of the stretch).
    pub wind_direction: Vector2<f64>,
    /// How strongly waves across the wind modulate the cloudlets (0 to 1), and their spacing.
    pub undulation: f64,
    pub wavelength: f64,
    /// How much the layer breaks into patches (0 to 1), and their size.
    pub patchiness: f64,
    pub patch_size: f64,
    /// How far (in cells) noise displaces the lattice, and how lumpy the cloudlets' outlines
    /// are (0 round, to 1).
    pub warp: f64,
    pub lumpiness: f64,
    /// Size of the eroded detail, and how strongly it erodes (0 to 1).
    pub detail_size: f64,
    pub erosion: f64,
    /// The square, `extent` either side of `center` in x and z, that the layer fills, fading
    /// out over its outer `fade` (fraction of `extent`).
    pub center: Vector2<f64>,
    pub extent: f64,
    pub fade: f64,
    /// Mean droplet radius (micrometres), for diffraction: altocumulus droplets are small and
    /// even, which is why it shows coronae and iridescence.
    pub particle_radius: Option<f64>,
    seed: u32,
    warp_noise: PerlinNoise,
    patch_noise: PerlinNoise,
    cells: OnceLock<WorleyTexture>,
}

/// Where a point lies among the cells: `edge` from 0 at its cell's centre to 1 at its edge,
/// `radial` its distance from the centre in half-spacings, and two numbers in [0, 1]
/// particular to the cell.
struct CellPoint {
    edge: f64,
    radial: f64,
    hash: (f64, f64),
}

impl BenardAltocumulus {
    pub fn new(base: f64, thickness: f64, seed: u64) -> BenardAltocumulus {
        BenardAltocumulus {
            base,
            thickness,
            max_density: 0.15,
            cell_size: 2.5 * thickness,
            coverage: 1.,
            regularity: 0.3,
            anisotropy: 1.,
            wind_direction: Vector2::new(1., 0.),
            undulation: 0.,
            wavelength: 12. * thickness,
            patchiness: 0.5,
            patch_size: 60. * thickness,
            warp: 0.35,
            lumpiness: 0.3,
            detail_size: thickness * 0.4,
            erosion: 0.35,
            center: Vector2::zeros(),
            extent: 50_000.,
            fade: 0.3,
            particle_radius: None,
            seed: seed as u32,
            warp_noise: PerlinNoise::with_seed(seed.wrapping_mul(3).wrapping_add(1)),
            patch_noise: PerlinNoise::with_seed(seed.wrapping_mul(3).wrapping_add(2)),
            cells: OnceLock::new(),
        }
    }

    /// The heights between which there can be cloud: bases are raised or lowered by up to a
    /// tenth of the thickness per cell, and by a third of it on the waves.
    pub fn bottom(&self) -> f64 {
        self.base - self.thickness * (0.1 + 0.3 * self.undulation)
    }

    pub fn top(&self) -> f64 {
        self.base + self.thickness * (1.5 + 0.3 * self.undulation)
    }

    fn detail(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed.wrapping_add(7)))
    }

    /// Along and across the wind, in metres.
    fn wind_frame(&self, x: f64, z: f64) -> (f64, f64) {
        let w = self.wind_direction;
        (x * w.x + z * w.y, -x * w.y + z * w.x)
    }

    /// The cell around (`u`, `v`), in lattice units: centres a unit apart.
    fn cell_at(&self, u: f64, v: f64) -> CellPoint {
        let jitter = 0.5 * (1. - self.regularity.clamp(0., 1.));
        let j0 = (v / ROW).round() as i64;
        let (mut f1, mut f2, mut nearest) = (f64::MAX, f64::MAX, (0i64, 0i64));
        for j in j0 - 2..=j0 + 2 {
            let shift = if j.rem_euclid(2) == 1 { 0.5 } else { 0. };
            let i0 = (u - shift).round() as i64;
            for i in i0 - 2..=i0 + 2 {
                let h = |k: u32| worley_hash(i as u32, j as u32, 0, self.seed.wrapping_add(k)) - 0.5;
                let c = Vector2::new(i as f64 + shift + 2. * jitter * h(0), j as f64 * ROW + 2. * jitter * h(1));
                let d = (c - Vector2::new(u, v)).norm_squared();
                if d < f1 {
                    f2 = f1;
                    f1 = d;
                    nearest = (i, j);
                } else if d < f2 {
                    f2 = d;
                }
            }
        }
        // Half the gap between the nearest two centres' distances is (nearly) the distance to
        // the boundary between them: 0.5 at the middle of a regular cell.
        let to_edge = (f2.sqrt() - f1.sqrt()) / 2.;
        let (i, j) = nearest;
        let h = |k: u32| worley_hash(i as u32, j as u32, 0, self.seed.wrapping_add(k));
        CellPoint { edge: (1. - 2. * to_edge).clamp(0., 1.), radial: 2. * f1.sqrt(), hash: (h(2), h(3)) }
    }

    /// Cloud density as a fraction of `max_density`, in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        let from_centre = (Vector2::new(p.x, p.z) - self.center).abs().max() / self.extent;
        let edge_fade = smoothstep(0., self.fade.max(1e-9), 1. - from_centre);
        if edge_fade <= 0. {
            return 0.;
        }
        // 4. Patches.
        let patches = {
            let n = stretched(self.patch_noise.fbm(p.x / self.patch_size, 0.5, p.z / self.patch_size, 3, 0.5, 2.));
            lerp(1., smoothstep(0.3, 0.6, n), self.patchiness)
        };
        let strength = edge_fade * patches;
        if strength <= 0. {
            return 0.;
        }
        // 1. The cells, in the wind's frame, warped.
        let (along, across) = self.wind_frame(p.x, p.z);
        let (mut u, mut v) = (along / (self.cell_size * self.anisotropy), across / self.cell_size);
        let warp = |s: f64| (self.warp_noise.fbm(u * 0.35, s, v * 0.35, 2, 0.5, 2.) - 0.5) * 2. * self.warp;
        (u, v) = (u + warp(0.25), v + warp(0.75));
        // 3. Waves across the wind, their crests wandering a little.
        let wave = if self.undulation > 0. {
            let wander = (self.warp_noise.fbm(along / self.wavelength * 0.3, 0.1, across / self.wavelength * 0.3, 2, 0.5, 2.) - 0.5) * 4.;
            0.5 + 0.5 * (std::f64::consts::TAU * along / self.wavelength + wander).cos()
        } else {
            1.
        };
        let cell = self.cell_at(u, v);
        // 2. This cell's cloudlet: its reach towards the edge, and how far out we are in it.
        let reach = self.coverage * strength * lerp(1., wave, self.undulation) * (0.8 + 0.4 * cell.hash.0);
        if reach <= 0. {
            return 0.;
        }
        // Round, not the cell's polygon (which only keeps neighbours apart), with a lumpy
        // outline.
        let lumps = 1. + (self.warp_noise.fbm(u * 2.5, 0.6, v * 2.5, 2, 0.5, 2.) - 0.5) * 2. * self.lumpiness;
        let t = lerp(cell.radial, cell.edge, 0.35) * lumps / reach;
        if t >= 1. {
            return 0.;
        }
        let offset = (cell.hash.1 - 0.5) * 0.2 + (wave - 0.5) * 0.6 * self.undulation;
        let h = (p.y - self.base) / self.thickness - offset;
        // A dome, smaller for smaller cloudlets and with turrets on top, over a base that sags
        // towards the edge.
        let turrets = worley_fbm(self.detail(), &(Vector3::new(p.x, self.base, p.z) / (2. * self.detail_size)));
        let top = (1. - t * t).sqrt() * reach.min(1.).sqrt() * (0.6 + 0.8 * turrets);
        let bottom = 0.15 * t * t;
        let inside = (h - bottom).min(top - h);
        if inside <= 0. {
            return 0.;
        }
        // Neighbours stay apart: the sinking air at the boundary is clear.
        let body = smoothstep(0., 0.3, inside) * smoothstep(0., 0.4, 1. - t) * smoothstep(0., 0.2, 1. - cell.edge);
        // 5. Erode the edges.
        let d = worley_fbm(self.detail(), &(p / self.detail_size));
        remap(body, (1. - d) * self.erosion, 1.)
    }
}

impl DensityField for BenardAltocumulus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.max_density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }

    /// The layer is a slab: `max_density` within it, nothing above or below.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let (lo, hi) = (self.bottom(), self.top());
        let to = |y: f64| if d.y != 0. { (y - p.y) / d.y } else { f64::INFINITY };
        if p.y < lo {
            (0., if d.y > 0. { to(lo) } else { f64::INFINITY })
        } else if p.y > hi {
            (0., if d.y < 0. { to(hi) } else { f64::INFINITY })
        } else {
            (self.max_density, if d.y > 0. { to(hi) } else if d.y < 0. { to(lo) } else { f64::INFINITY })
        }
    }

    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        self.particle_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn uniform(rng: &mut Pcg32, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * rng.next_f64()
    }

    fn layer() -> BenardAltocumulus {
        let mut c = BenardAltocumulus::new(4000., 200., 3);
        c.patchiness = 0.;
        c
    }

    #[test]
    fn test_density_is_bounded_and_in_the_slab() {
        let c = layer();
        let mut rng = Pcg32::new(1, 0);
        for _ in 0..20_000 {
            let p = Vector3::new(uniform(&mut rng, -5000., 5000.), uniform(&mut rng, 3500., 4500.), uniform(&mut rng, -5000., 5000.));
            let f = c.fraction(&p);
            assert!((0. ..=1.).contains(&f), "{}", f);
            if p.y < c.bottom() || p.y > c.top() {
                assert_eq!(f, 0.);
            }
        }
    }

    #[test]
    fn test_majorant_bounds_density() {
        let c = layer();
        let mut rng = Pcg32::new(2, 0);
        for _ in 0..500 {
            let p = Vector3::new(uniform(&mut rng, -3000., 3000.), uniform(&mut rng, 3000., 5000.), uniform(&mut rng, -3000., 3000.));
            let d = Vector3::new(uniform(&mut rng, -1., 1.), uniform(&mut rng, -1., 1.), uniform(&mut rng, -1., 1.)).normalize();
            let (m, len) = c.majorant_from(&p, &d);
            for k in 0..50 {
                let t = len.min(5000.) * (k as f64 + 0.5) / 50.;
                assert!(c.density(&(p + d * t)) <= m, "{} along {} at {}", p, d, t);
            }
        }
    }

    #[test]
    fn test_regular_cells_are_cloud_in_the_middle_and_clear_at_the_edges() {
        let mut c = layer();
        (c.regularity, c.warp, c.erosion, c.coverage) = (1., 0., 0., 0.6);
        let mid = c.base + 0.3 * c.thickness;
        // Centres are `cell_size` apart along x (the wind); the boundary is half way.
        for i in -3..3 {
            let x = i as f64 * c.cell_size;
            assert!(c.fraction(&Vector3::new(x, mid, 0.)) > 0.5, "{}", x);
            assert_eq!(c.fraction(&Vector3::new(x + c.cell_size / 2., mid, 0.)), 0.);
        }
        // The next row is offset by half a cell.
        let row = Vector3::new(c.cell_size / 2., mid, c.cell_size * ROW);
        assert!(c.fraction(&row) > 0.5);
        // Domes: thicker in the middle than near the edge.
        let depth = |x: f64| (0..100).filter(|k| c.fraction(&Vector3::new(x, c.bottom() + (c.top() - c.bottom()) * *k as f64 / 100., 0.)) > 0.).count();
        assert!(depth(0.) > depth(0.2 * c.cell_size));
    }

    #[test]
    fn test_coverage_and_waves() {
        let mut rng = Pcg32::new(3, 0);
        let points: Vec<Vector3<f64>> = (0..20_000).map(|_| Vector3::new(uniform(&mut rng, -8000., 8000.), 4060., uniform(&mut rng, -8000., 8000.))).collect();
        let cloudy = |c: &BenardAltocumulus, f: &dyn Fn(&Vector3<f64>) -> bool| {
            points.iter().filter(|p| f(p)).filter(|p| c.fraction(p) > 0.).count() as f64
                / points.iter().filter(|p| f(p)).count() as f64
        };
        let mut c = layer();
        c.coverage = 0.4;
        let sparse = cloudy(&c, &|_| true);
        c.coverage = 0.9;
        let dense = cloudy(&c, &|_| true);
        assert!(dense > sparse * 1.5, "{} vs {}", dense, sparse);
        // Undulatus: cloud along the crests (x a multiple of the wavelength), little in the troughs.
        c.undulation = 1.;
        let points: Vec<Vector3<f64>> = points.iter().map(|p| Vector3::new(p.x, 4100., p.z)).collect();
        let cloudy = |c: &BenardAltocumulus, f: &dyn Fn(&Vector3<f64>) -> bool| {
            points.iter().filter(|p| f(p)).filter(|p| c.fraction(p) > 0.).count() as f64
                / points.iter().filter(|p| f(p)).count() as f64
        };
        let phase = |p: &Vector3<f64>| (p.x / c.wavelength).rem_euclid(1.);
        let crests = cloudy(&c, &|p| !(0.15..0.85).contains(&phase(p)));
        let troughs = cloudy(&c, &|p| (0.35..0.65).contains(&phase(p)));
        assert!(crests > 2. * troughs, "{} vs {}", crests, troughs);
    }
}
