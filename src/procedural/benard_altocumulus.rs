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
    /// Built on first use, from the layer as it then is.
    tops: OnceLock<Tops>,
}

/// How high cloud can be over each texel of a grid across the layer
/// (`shared::altocumulus::Benard::tops`): what lets a walk through the layer skip its clear
/// air, which is most of the slab it fills.
struct Tops {
    /// The grid's corner (x, z), its texel, and its texels a side.
    origin: Vector2<f64>,
    texel: f64,
    n: usize,
    /// A texel's top, or far below the layer where it holds no cloud.
    tops: Vec<f32>,
}

/// A cloudlet is taken to reach at least this far towards its cell's edge when the map of
/// tops is sampled (`Benard::column_top`): no smaller than the real one, and wide enough for
/// the samples to find (a fifth of a cell across or more, the samples a sixteenth apart).
const LEAST_REACH: f64 = 0.25;

/// Samples of the layer's top along a side of each of the map's texels (and one more, on its
/// far edge): a sixteenth of a cell apart.
const SAMPLES: usize = 4;

/// The layer's tables as its shared recipe reads them: the permutation of the noise that
/// warps the lattice, then the patches', then the Worley texture of the detail.
struct LayerTables<'a> {
    layer: &'a BenardAltocumulus,
    cells: &'a WorleyTexture,
    /// The map of tops, after the Worley texture, for a bound (the density reads none).
    tops: Option<&'a Tops>,
}

/// Where the map of tops begins, after the Worley texture.
const TOPS_AT: u32 = WORLEY_AT + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N;

const PATCH_AT: u32 = 512;
const WORLEY_AT: u32 = 1024;

impl shared::Tables<f64> for LayerTables<'_> {
    fn value(&self, i: u32) -> f64 {
        if i >= TOPS_AT {
            return self.tops.map_or(0., |t| t.tops[(i - TOPS_AT) as usize] as f64);
        }
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
            tops: OnceLock::new(),
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
            // (No map: the density doesn't read one.)
            tops: shared::clouds::WeatherGrid { x: 0., z: 0., texel: 1., n: 0, corners: 0, maxima: TOPS_AT },
        }
    }

    /// The recipe with its map of tops, for bounding the density (`majorant_from`).
    fn bounded(&self) -> (shared::altocumulus::Benard<f64>, &Tops) {
        let tops = self.tops();
        let grid = shared::clouds::WeatherGrid { x: tops.origin.x, z: tops.origin.y, texel: tops.texel, n: tops.n as u32, corners: 0, maxima: TOPS_AT };
        (shared::altocumulus::Benard { tops: grid, ..self.recipe() }, tops)
    }

    /// The map of how high cloud can be over each texel, a quarter of a cell across (or
    /// coarser, to keep it to a thousand a side). Sampled, not derived: the layer's highest
    /// (`Benard::column_top`) at 25 points of every texel, its edges included, for cloudlets
    /// no smaller than the real ones and at least `LEAST_REACH` across; a texel's top is the
    /// highest of its own samples and its neighbours', and a little over. So it bounds the
    /// cloud wherever a cloudlet's outline wanders less than a texel between samples a
    /// sixteenth of a cell apart, which the lumps of an outline do (they are a fifth of a cell
    /// across at their finest); and the density falls to nothing at a cloudlet's edge
    /// (as the square of the distance in), so what a sliver missed would hold is nothing too.
    /// `test_the_map_of_tops_bounds_the_cloud` looks for cloud above it.
    fn tops(&self) -> &Tops {
        // (Apart from the render's threads, which wait on it: `geometry::build_apart`.)
        self.tops.get_or_init(|| crate::geometry::build_apart(|| {
            use rayon::prelude::*;
            // Sampled at f32 from the layer's record, as a kernel reads it: the same functions,
            // five times as fast as through this layer's own tables, and there are millions.
            let record = self.record(None);
            let recipe = pathtrace::load_benard(&record, 0);
            let tables = pathtrace::FloatTables(&record);
            let n = ((2. * self.extent / (self.cell_size / 4.)).ceil() as usize).clamp(1, 1024);
            let texel = 2. * self.extent / n as f64;
            // Coarser than half a cell, a texel nearly always holds some cloudlet: no map is
            // worth its three seconds then (cells a hundred metres across over a hundred
            // kilometres: cirrocumulus), and the walk takes the slab.
            if texel > self.cell_size / 2. {
                return Tops { origin: self.center - Vector2::new(self.extent, self.extent), texel, n: 0, tops: vec![] };
            }
            let origin = self.center - Vector2::new(self.extent, self.extent);
            let none = f64::NEG_INFINITY;
            // `SAMPLES` to a texel's side, its edges included.
            let m = SAMPLES * n + 1;
            let step = texel / SAMPLES as f64;
            let samples: Vec<f64> = (0..m * m).into_par_iter().map(|k| {
                let (x, z) = (origin.x + (k % m) as f64 * step, origin.y + (k / m) as f64 * step);
                recipe.column_top(&tables, x as f32, z as f32, LEAST_REACH as f32).map_or(none, |top| top as f64)
            }).collect();
            let side = SAMPLES + 1;
            let own: Vec<f64> = (0..n * n).map(|k| {
                let (i, j) = (k % n, k / n);
                (0..side * side).map(|c| samples[(SAMPLES * j + c / side) * m + SAMPLES * i + c % side]).fold(none, f64::max)
            }).collect();
            let pad = 0.1 * self.thickness;
            let tops = (0..n * n).map(|k| {
                let (i, j) = (k % n, k / n);
                let mut top = none;
                for jj in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                    for ii in i.saturating_sub(1)..=(i + 1).min(n - 1) {
                        top = top.max(own[jj * n + ii]);
                    }
                }
                if top == none { -1e30 } else { (top + pad) as f32 }
            }).collect();
            Tops { origin, texel, n, tops }
        }))
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
        self.recipe().fraction(&LayerTables { layer: self, cells: self.detail(), tops: None }, p.x, p.y, p.z)
    }

    /// The layer as data, for the GPU's shader (`pathtrace::load_benard`, which has the
    /// layout): the recipe's numbers in their fields' order and the seed's bits, then the two
    /// permutations, the Worley texture and the map of tops (whose grid follows the seed).
    pub fn export(&self) -> Vec<f32> {
        self.record(Some(self.tops()))
    }

    /// `export`, with the map of tops given, or without one (which is how the map itself is
    /// sampled).
    fn record(&self, tops: Option<&Tops>) -> Vec<f32> {
        let r = self.recipe();
        let mut out: Vec<f32> = [
            r.base, r.thickness, r.max_density, r.cell_size, r.coverage, r.regularity, r.anisotropy, r.wind.0, r.wind.1,
            r.undulation, r.wavelength, r.patchiness, r.patch_size, r.warp, r.lumpiness, r.detail_size, r.erosion,
            r.center.0, r.center.1, r.extent, r.fade, r.castellanus, r.turret_height,
        ].iter().map(|&v| v as f32).collect();
        out.push(f32::from_bits(self.seed));
        out.extend(tops.map_or([0.; 4], |t| [t.origin.x as f32, t.origin.y as f32, t.texel as f32, t.n as f32]));
        out.extend(self.warp_noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.patch_noise.permutation().iter().map(|&p| p as f32));
        out.extend(self.detail().texels());
        if let Some(t) = tops {
            out.extend(&t.tops);
        }
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
        let (recipe, tops) = self.bounded();
        recipe.majorant_from(&LayerTables { layer: self, cells: self.detail(), tops: Some(tops) }, p.x, p.y, p.z, d.x, d.y, d.z)
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

    /// The map of tops (`BenardAltocumulus::tops`) is sampled, so it is tested: over layers
    /// plain, waved, with turrets, sparse and regular, no point of two million in the slab
    /// has cloud above its texel's top, or over a texel marked clear; and the map is worth
    /// having, most of the slab lying above it.
    #[test]
    fn test_the_map_of_tops_bounds_the_cloud() {
        let mut rng = Pcg32::new(21, 0);
        for case in 0..5u64 {
            let mut c = BenardAltocumulus::new(4000., 250., case + 2);
            c.extent = 12_000.;
            if case >= 1 { (c.undulation, c.anisotropy) = (0.8, 1.7); }
            if case >= 2 { (c.castellanus, c.turret_height, c.wind_direction) = (0.4, 6., Vector2::new(1., 0.3).normalize()); }
            if case == 3 { (c.regularity, c.coverage, c.cell_size) = (0.9, 0.7, 900.); }
            if case == 4 { (c.coverage, c.patchiness, c.lumpiness, c.warp) = (0.45, 0.9, 1., 0.6); }
            let tops = c.tops();
            let (mut cloudy, mut above, mut clear) = (0, 0, 0);
            let n = 2_000_000;
            for _ in 0..n {
                let p = Vector3::new(uniform(&mut rng, -12_000., 12_000.), uniform(&mut rng, c.bottom(), c.top()), uniform(&mut rng, -12_000., 12_000.));
                let (i, j) = ((((p.x - tops.origin.x) / tops.texel) as usize).min(tops.n - 1), (((p.z - tops.origin.y) / tops.texel) as usize).min(tops.n - 1));
                let top = tops.tops[j * tops.n + i] as f64;
                if p.y > top { clear += 1; }
                if c.fraction(&p) > 0. {
                    cloudy += 1;
                    if p.y > top { above += 1; }
                }
            }
            println!("case {}: {} of {} points in cloud, {} above the map; {:.0}% of the slab is above it", case, cloudy, n, above, 100. * clear as f64 / n as f64);
            assert!(cloudy > 500, "{} points in cloud", cloudy);
            assert_eq!(above, 0, "case {}: cloud above the map of tops", case);
            assert!(clear > n / 5, "case {}: only {} of {} points above the map", case, clear, n);
        }
    }

    /// What the map of tops costs to build. `--ignored --nocapture`.
    #[test]
    #[ignore]
    fn measure_the_map_of_tops() {
        let c = BenardAltocumulus::new(4000., 200., 2);
        let record = c.record(None);
        let recipe = pathtrace::load_benard(&record, 0);
        let tables = pathtrace::FloatTables(&record);
        let mut rng = Pcg32::new(1, 0);
        let points: Vec<(f32, f32)> = (0..1_000_000).map(|_| (uniform(&mut rng, -40000., 40000.) as f32, uniform(&mut rng, -40000., 40000.) as f32)).collect();
        let start = std::time::Instant::now();
        let found = points.iter().filter(|p| recipe.column_top(&tables, p.0, p.1, LEAST_REACH as f32).is_some()).count();
        println!("a column's top: {:.0} ns a sample ({} of {} hold cloud)", start.elapsed().as_secs_f64() * 1e9 / points.len() as f64, found, points.len());
        let start = std::time::Instant::now();
        let tops = c.tops();
        println!("the map, {} texels a side of {:.0} m: {:.2} s", tops.n, tops.texel, start.elapsed().as_secs_f64());
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
