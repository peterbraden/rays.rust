//! Mammatus: rounded pouches hanging from the underside of an anvil (or another cloud's base).
//! A density field for `volume::Volume`.
//!
//! Air laden with ice or droplets sinking from the base of a cloud cools as its particles
//! evaporate, and where that cooling outpaces the warming of its descent it keeps sinking, in
//! lobes, into the drier air below. They hang in clusters, each a kilometre or two across and a
//! few hundred metres deep, with sharp, smooth boundaries where the cloud evaporates (D. M.
//! Schultz et al., "The mysteries of mammatus clouds: observations and formation mechanisms",
//! J. Atmos. Sci. 63, 2006). They are most striking at sunset, when sunlight comes in under the
//! edge of the anvil and lights them from below.
//!
//! The field is:
//!
//! 1. **The anvil.** A thick sheet of ice, from `base` up `thickness`, covering the sky on one
//!    side of a ragged edge (`edge_direction`, `edge_distance`), thinning towards the edge.
//! 2. **Pouches.** Cells of a jittered hexagonal lattice under the base, `pouch_size` apart. Each
//!    cell that holds one (`coverage`) hangs a half-ellipsoid of its own depth (up to
//!    `pouch_depth`), touching its neighbours with narrow creases between. They hang deepest a
//!    few kilometres in from the anvil's edge, and gather into clusters (`clustering`).
//! 3. **Their surface.** Smooth: the density rises over `edge_sharpness` metres inside the
//!    boundary, which noise ripples by `roughness` metres.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::volume::DensityField;

/// Rows of the hexagonal lattice are this far apart, for centres a unit apart.
const ROW: f64 = 0.866_025_403_784_438_6;

pub struct Mammatus {
    /// Height of the anvil's base, and how thick it is: its top is flat, at base + thickness,
    /// where it spreads against the tropopause.
    pub base: f64,
    pub thickness: f64,
    /// How far the base undulates up and down (m), over what distance; how far it sags below
    /// `base` towards the storm's core, reached `sag_distance` in from the edge; and over how
    /// many metres its underside thickens to full density.
    pub base_variation: f64,
    pub base_size: f64,
    pub sag: f64,
    pub sag_distance: f64,
    pub base_softness: f64,
    /// How far the flat top rises and falls (m), over `base_size`: anvils spread against the
    /// tropopause, but not along a ruled line.
    pub top_variation: f64,
    /// Extinction in the anvil, and in the pouches.
    pub anvil_density: f64,
    pub pouch_density: f64,
    /// Distance between neighbouring pouches, the deepest they hang, and the fraction of cells
    /// that hold one.
    pub pouch_size: f64,
    pub pouch_depth: f64,
    pub coverage: f64,
    /// 1 for a perfect lattice, 0 for pouches placed at random.
    pub regularity: f64,
    /// How much the pouches gather into clusters (0 to 1), and how far apart the clusters are.
    pub clustering: f64,
    pub cluster_size: f64,
    /// How far inside its boundary a pouch reaches full density, and how much its surface is
    /// rippled (both m).
    pub edge_sharpness: f64,
    pub roughness: f64,
    pub detail_size: f64,
    /// The anvil covers the sky where the distance along `edge_direction` ([x, z], unit) is
    /// less than `edge_distance`, give or take `edge_raggedness` metres; it tapers to nothing over
    /// `edge_taper` inside the edge.
    pub edge_direction: Vector2<f64>,
    pub edge_distance: f64,
    pub edge_raggedness: f64,
    pub edge_taper: f64,
    /// The sheet's extent: within `extent` of `center` ([x, z]), fading out over its outer
    /// `fade` (a fraction of `extent`; 0 for no fade, as for a sheet larger than the view).
    pub center: Vector2<f64>,
    pub extent: f64,
    pub fade: f64,
    /// The pouches are deepest around `pouch_band` metres in from the edge, fading over the
    /// same distance either side.
    pub pouch_band: f64,
    seed: u32,
    noise: PerlinNoise,
}

impl Mammatus {
    pub fn new(seed: u64) -> Mammatus {
        Mammatus {
            base: 5000.,
            thickness: 2500.,
            base_variation: 0.,
            base_size: 20_000.,
            sag: 0.,
            sag_distance: 15_000.,
            base_softness: 0.,
            top_variation: 0.,
            anvil_density: 0.015,
            pouch_density: 0.02,
            pouch_size: 1400.,
            pouch_depth: 600.,
            coverage: 0.85,
            regularity: 0.15,
            clustering: 0.5,
            cluster_size: 7000.,
            edge_sharpness: 40.,
            roughness: 25.,
            detail_size: 300.,
            edge_direction: Vector2::new(1., 0.),
            edge_distance: 0.,
            edge_raggedness: 3000.,
            edge_taper: 2500.,
            pouch_band: 12_000.,
            center: Vector2::zeros(),
            extent: 60_000.,
            fade: 0.,
            seed: seed as u32,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(5).wrapping_add(41)),
        }
    }

    pub fn bottom(&self) -> f64 {
        self.recipe().bottom()
    }

    pub fn top(&self) -> f64 {
        self.recipe().top()
    }

    /// Fraction of `anvil_density` (above the base) or `pouch_density` (below), in [0, 1]: the
    /// anvil thinning towards its ragged edge and its top over a soft underside, and below
    /// its base the pouches, a warped lattice of cells each hanging as a rounded bag, deepest
    /// in a band in from the edge and gathered into clusters
    /// (`shared::mammatus::Mammatus::fraction`).
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        self.recipe().fraction(&self.noise, p.x, p.y, p.z)
    }
}

impl Mammatus {
    /// The recipe, as the shared code takes it (`shared::mammatus::Mammatus`, where the
    /// density is: the GPU's shader compiles the same functions at f32).
    fn recipe(&self) -> shared::mammatus::Mammatus<f64> {
        shared::mammatus::Mammatus {
            base: self.base, thickness: self.thickness, base_variation: self.base_variation, base_size: self.base_size,
            sag: self.sag, sag_distance: self.sag_distance, base_softness: self.base_softness, top_variation: self.top_variation,
            anvil_density: self.anvil_density, pouch_density: self.pouch_density, pouch_size: self.pouch_size,
            pouch_depth: self.pouch_depth, coverage: self.coverage, regularity: self.regularity, clustering: self.clustering,
            cluster_size: self.cluster_size, edge_sharpness: self.edge_sharpness, roughness: self.roughness,
            detail_size: self.detail_size, edge_direction: (self.edge_direction.x, self.edge_direction.y),
            edge_distance: self.edge_distance, edge_raggedness: self.edge_raggedness, edge_taper: self.edge_taper,
            center: (self.center.x, self.center.y), extent: self.extent, fade: self.fade, pouch_band: self.pouch_band,
            seed: self.seed, perm: 0,
        }
    }

    /// The layer as data, for the GPU's shader (`pathtrace::load_mammatus`, which has the
    /// layout): the recipe's numbers in their fields' order, the seed's bits, then the
    /// permutation.
    pub fn export(&self) -> Vec<f32> {
        let r = self.recipe();
        let mut out: Vec<f32> = [
            r.base, r.thickness, r.base_variation, r.base_size, r.sag, r.sag_distance, r.base_softness, r.top_variation,
            r.anvil_density, r.pouch_density, r.pouch_size, r.pouch_depth, r.coverage, r.regularity, r.clustering,
            r.cluster_size, r.edge_sharpness, r.roughness, r.detail_size, r.edge_direction.0, r.edge_direction.1,
            r.edge_distance, r.edge_raggedness, r.edge_taper, r.center.0, r.center.1, r.extent, r.fade, r.pouch_band,
        ].iter().map(|&v| v as f32).collect();
        out.push(f32::from_bits(self.seed));
        out.extend(self.noise.permutation().iter().map(|&p| p as f32));
        out
    }
}

impl DensityField for Mammatus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.recipe().density(&self.noise, p.x, p.y, p.z)
    }

    fn majorant(&self) -> f64 {
        self.anvil_density.max(self.pouch_density)
    }

    /// Two slabs, the pouches' and the anvil's, and nothing beyond the anvil's ragged edge.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        self.recipe().majorant_from(p.x, p.y, p.z, d.x, d.y, d.z)
    }

    fn export(&self) -> Option<crate::volume::FieldExport> {
        Some(crate::volume::FieldExport::Mammatus { record: Mammatus::export(self), radius: 0. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn uniform(rng: &mut Pcg32, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * rng.next_f64()
    }

    #[test]
    fn test_majorant_bounds_density() {
        let mut m = Mammatus::new(2);
        m.edge_direction = Vector2::new(0.6, -0.8);
        test_majorant_for(&m);
        (m.base_variation, m.sag, m.base_softness, m.top_variation) = (500., 1200., 300., 400.);
        test_majorant_for(&m);
    }

    fn test_majorant_for(m: &Mammatus) {
        let mut rng = Pcg32::new(7, 0);
        for _ in 0..1000 {
            let p = Vector3::new(uniform(&mut rng, -30000., 30000.), uniform(&mut rng, 3000., 8500.), uniform(&mut rng, -30000., 30000.));
            let d = Vector3::new(uniform(&mut rng, -1., 1.), uniform(&mut rng, -1., 1.), uniform(&mut rng, -1., 1.)).normalize();
            let (bound, len) = m.majorant_from(&p, &d);
            assert!(len > 0.);
            for k in 0..40 {
                let t = len.min(20000.) * (k as f64 + 0.5) / 40.;
                assert!(m.density(&(p + d * t)) <= bound + 1e-12, "{} along {} at {}", p, d, t);
            }
        }
    }

    #[test]
    fn test_pouches_hang_below_the_base_inside_the_anvil() {
        let mut m = Mammatus::new(3);
        (m.roughness, m.edge_raggedness, m.coverage) = (0., 0., 1.);
        // Sample a line under the anvil, a band's width in from its edge.
        let x = -m.pouch_band;
        let below = |y: f64| (0..400).filter(|k| m.fraction(&Vector3::new(x, y, *k as f64 * 37.)) > 0.5).count() as f64 / 400.;
        assert!(below(m.base - 50.) > 0.5, "{}", below(m.base - 50.));
        assert!(below(m.base - 0.4 * m.pouch_depth) > 0.05);
        assert!(below(m.base - 0.4 * m.pouch_depth) < below(m.base - 50.));
        assert_eq!(below(m.bottom()), 0.);
        // The anvil is solid above the base, and absent beyond its edge.
        assert!(m.fraction(&Vector3::new(x, m.base + 500., 0.)) > 0.8);
        assert_eq!(m.fraction(&Vector3::new(1000., m.base + 500., 0.)), 0.);
        assert_eq!(m.fraction(&Vector3::new(1000., m.base - 100., 0.)), 0.);
    }
}
