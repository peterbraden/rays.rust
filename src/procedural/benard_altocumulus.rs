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
//! 6. **Castellanus.** When the layer turns unstable, by cooling aloft or moist air lifted into
//!    it, the rising air in some cells keeps rising: a turret grows from the cloudlet, taller
//!    than it is wide, its head billowing, all of them on the layer's common base and lined up
//!    with the cells' rows (the WMO's castellanus; a sign that thunderstorms may follow later in
//!    the day). `castellanus` is the share of cells with a turret, `turret_height` how tall
//!    they grow, in layer thicknesses.

use crate::na::{Vector2, Vector3};
use std::sync::OnceLock;
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::WorleyTexture;
use crate::volume::DensityField;

/// Rows of the hexagonal lattice are this far apart, for centres a unit apart.
#[cfg(test)]
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
    /// The share of cells that grow a turret (0 to 1), and how tall the tallest grow, in layer
    /// thicknesses (castellanus).
    pub castellanus: f64,
    pub turret_height: f64,
    seed: u32,
    warp_noise: PerlinNoise,
    patch_noise: PerlinNoise,
    cells: OnceLock<WorleyTexture>,
}

/// The layer's tables as its shared recipe reads them: the permutation of the noise that
/// warps the lattice, then the patches', then the Worley texture of the detail.
struct LayerTables<'a> {
    layer: &'a BenardAltocumulus,
    cells: &'a WorleyTexture,
}

const PATCH_AT: u32 = 512;
const WORLEY_AT: u32 = 1024;

impl shared::Tables<f64> for LayerTables<'_> {
    fn value(&self, i: u32) -> f64 {
        self.cells.texels()[(i - WORLEY_AT) as usize] as f64
    }

    fn index(&self, i: u32) -> u32 {
        if i < PATCH_AT { self.layer.warp_noise.permutation()[i as usize] as u32 } else { self.layer.patch_noise.permutation()[(i - PATCH_AT) as usize] as u32 }
    }
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
            castellanus: 0.,
            turret_height: 3.,
            seed: seed as u32,
            warp_noise: PerlinNoise::shuffled(seed.wrapping_mul(3).wrapping_add(1)),
            patch_noise: PerlinNoise::shuffled(seed.wrapping_mul(3).wrapping_add(2)),
            cells: OnceLock::new(),
        }
    }

    /// The layer's recipe, as the shared code takes it (`shared::altocumulus::Benard`, where
    /// the density is: the GPU's shader compiles the same functions at f32).
    fn recipe(&self) -> shared::altocumulus::Benard<f64> {
        shared::altocumulus::Benard {
            base: self.base, thickness: self.thickness, max_density: self.max_density, cell_size: self.cell_size,
            coverage: self.coverage, regularity: self.regularity, anisotropy: self.anisotropy,
            wind: (self.wind_direction.x, self.wind_direction.y), undulation: self.undulation, wavelength: self.wavelength,
            patchiness: self.patchiness, patch_size: self.patch_size, warp: self.warp, lumpiness: self.lumpiness,
            detail_size: self.detail_size, erosion: self.erosion, center: (self.center.x, self.center.y),
            extent: self.extent, fade: self.fade, castellanus: self.castellanus, turret_height: self.turret_height,
            seed: self.seed, warp_perm: 0, patch_perm: PATCH_AT, worley: WORLEY_AT,
        }
    }

    /// The heights between which there can be cloud: bases are raised or lowered by up to a
    /// tenth of the thickness per cell, and by a third of it on the waves.
    pub fn bottom(&self) -> f64 {
        self.recipe().bottom()
    }

    pub fn top(&self) -> f64 {
        self.recipe().top()
    }

    fn detail(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed.wrapping_add(7)))
    }

    /// Cloud density as a fraction of `max_density`, in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        self.recipe().fraction(&LayerTables { layer: self, cells: self.detail() }, p.x, p.y, p.z)
    }

    /// The layer as data, for the GPU's shader (`pathtrace::load_benard`, which has the
    /// layout): the recipe's numbers in their fields' order and the seed's bits, then the two
    /// permutations and the Worley texture.
    pub fn export(&self) -> Vec<f32> {
        let r = self.recipe();
        let mut out: Vec<f32> = [
            r.base, r.thickness, r.max_density, r.cell_size, r.coverage, r.regularity, r.anisotropy, r.wind.0, r.wind.1,
            r.undulation, r.wavelength, r.patchiness, r.patch_size, r.warp, r.lumpiness, r.detail_size, r.erosion,
            r.center.0, r.center.1, r.extent, r.fade, r.castellanus, r.turret_height,
        ].iter().map(|&v| v as f32).collect();
        out.push(f32::from_bits(self.seed));
        out.extend(self.warp_noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.patch_noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.detail().texels());
        out
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
        self.recipe().majorant_from(p.y, d.y)
    }

    /// As data, where nothing but the density is asked of it (a droplet size is, for
    /// diffraction).
    fn export(&self) -> Option<crate::volume::FieldExport> {
        Some(crate::volume::FieldExport::Benard { record: BenardAltocumulus::export(self), radius: 0. })
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
    fn test_castellanus_turrets_rise_from_the_layer() {
        let mut c = layer();
        (c.castellanus, c.turret_height) = (0.4, 5.);
        // Cloud far above the layer's domes (over twice its thickness), only with turrets, and
        // all of it within the slab the majorant covers.
        let (mut high, mut rng) = (0, Pcg32::new(4, 1));
        for _ in 0..20000 {
            let p = Vector3::new(uniform(&mut rng, -4000., 4000.), uniform(&mut rng, c.bottom() - 100., c.top() + 100.), uniform(&mut rng, -4000., 4000.));
            let d = c.density(&p);
            if d > 0. {
                assert!(p.y >= c.bottom() && p.y <= c.top(), "{}", p);
                if p.y > c.base + 2.5 * c.thickness { high += 1; }
            }
        }
        assert!(high > 10, "{}", high);
        let plain = layer();
        assert!((0..20000).all(|k| plain.density(&Vector3::new((k % 200) as f64 * 40. - 4000., plain.base + 2.5 * plain.thickness, (k / 200) as f64 * 80. - 4000.)) == 0.));
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
