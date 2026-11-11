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
use crate::noise::{worley_hash, PerlinNoise};
use crate::procedural::nubis_clouds::{lerp, smoothstep};
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
            seed: seed as u32,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(5).wrapping_add(41)),
        }
    }

    pub fn bottom(&self) -> f64 {
        self.base - self.base_variation - self.sag - self.pouch_depth * 1.2 - self.roughness
    }

    /// The height of the anvil's base at (x, z), `inside` its edge.
    fn local_base(&self, x: f64, z: f64, inside: f64) -> f64 {
        let mut b = self.base;
        if self.base_variation > 0. {
            b += (self.noise.fbm(x / self.base_size, 0.13, z / self.base_size, 3, 0.5, 2.) - 0.5) * 2. * self.base_variation;
        }
        if self.sag > 0. {
            b -= self.sag * smoothstep(0., self.sag_distance, inside);
        }
        b.min(self.top() - 1.)
    }

    pub fn top(&self) -> f64 {
        self.base + self.thickness
    }

    /// How far inside the anvil's ragged edge (x, z) lies.
    fn inside_edge(&self, x: f64, z: f64) -> f64 {
        let s = x * self.edge_direction.x + z * self.edge_direction.y;
        let across = -x * self.edge_direction.y + z * self.edge_direction.x;
        let ragged = (self.noise.fbm(across / 9000., 0.31, s / 9000., 4, 0.5, 2.) - 0.5) * 2. * self.edge_raggedness;
        self.edge_distance + ragged - s
    }

    /// For the pouch cell around (x, z): how far out from its middle (0 to 1, where 1 is the
    /// crease with the next), and its relative depth (0 for none).
    fn pouch_at(&self, x: f64, z: f64) -> (f64, f64) {
        let (u, v) = (x / self.pouch_size, z / self.pouch_size);
        let warp = |o: f64| (self.noise.fbm(u * 0.3, o, v * 0.3, 2, 0.5, 2.) - 0.5) * 0.7;
        let (u, v) = (u + warp(0.4), v + warp(0.8));
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
                    (f2, f1, nearest) = (f1, d, (i, j));
                } else if d < f2 {
                    f2 = d;
                }
            }
        }
        let to_edge = (f2.sqrt() - f1.sqrt()) / 2.;
        let edge = (1. - 2. * to_edge).clamp(0., 1.);
        let (i, j) = nearest;
        let h = |k: u32| worley_hash(i as u32, j as u32, 0, self.seed.wrapping_add(k));
        let depth = if h(2) < self.coverage { 0.55 + 0.45 * h(3) } else { 0. };
        // Round rather than polygonal, still meeting the neighbours at the crease.
        (lerp(2. * f1.sqrt(), edge, 0.5).min(1.), depth)
    }

    /// Fraction of `anvil_density` (above the base) or `pouch_density` (below), in [0, 1].
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        let inside = self.inside_edge(p.x, p.z);
        if inside <= 0. {
            return 0.;
        }
        let taper = smoothstep(0., self.edge_taper, inside);
        let d = p / self.detail_size;
        let ripple = (self.noise.fbm(d.x, d.y, d.z, 3, 0.5, 2.) - 0.5) * 2.;
        let base = self.local_base(p.x, p.z, inside);
        if p.y >= base {
            // The anvil: thinning towards its edge, from the top down, and towards its top, and
            // thickening over its soft underside.
            let depth = self.top() - base;
            let h = (p.y - base) / depth;
            let top = taper * (0.75 + 0.25 * ripple);
            let underside = if self.base_softness > 0. { smoothstep(0., self.base_softness / depth, h + 0.3 * ripple * self.base_softness / depth) } else { 1. };
            return (1. - smoothstep(top - 0.15, top, h)) * (0.85 + 0.15 * ripple) * underside;
        }
        let band = smoothstep(0., self.pouch_band, inside) * (1. - smoothstep(self.pouch_band, 3. * self.pouch_band, inside)).max(0.3);
        let (t, depth) = self.pouch_at(p.x, p.z);
        let clusters = self.noise.fbm(p.x / self.cluster_size, 0.77, p.z / self.cluster_size, 3, 0.5, 2.);
        let depth = depth * lerp(1., smoothstep(0.3, 0.65, clusters), self.clustering);
        let hang = self.pouch_depth * depth * band * taper * (1. - t * t).max(0.).sqrt();
        // Height above the pouch's lower surface.
        let above = p.y - (base - hang) + ripple * self.roughness;
        smoothstep(0., self.edge_sharpness, above)
    }
}

impl DensityField for Mammatus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let f = self.fraction(p);
        if f <= 0. {
            return 0.;
        }
        let base = self.local_base(p.x, p.z, self.inside_edge(p.x, p.z));
        if p.y >= base { self.anvil_density * f } else { self.pouch_density * f }
    }

    fn majorant(&self) -> f64 {
        self.anvil_density.max(self.pouch_density)
    }

    /// Two slabs, the pouches' and the anvil's, and nothing beyond the anvil's ragged edge.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        let positive = |t: f64| if t > 0. { t } else { f64::INFINITY };
        // Beyond the furthest the edge reaches: empty until the ray comes back inside it.
        let s = ahead.x * self.edge_direction.x + ahead.z * self.edge_direction.y;
        let ds = d.x * self.edge_direction.x + d.z * self.edge_direction.y;
        let limit = self.edge_distance + self.edge_raggedness;
        if s > limit {
            let back = if ds < 0. { (s - limit) / -ds } else { f64::INFINITY };
            return (0., back + 0.01);
        }
        let leave = if ds > 0. { (limit - s) / ds } else { f64::INFINITY };
        let (lo, hi) = (self.bottom(), self.top());
        // With a varying base, the pouches and the anvil share one slab; otherwise each has its
        // own, either side of the base.
        let flat = self.base_variation <= 0. && self.sag <= 0.;
        let (m, slab) = if ahead.y < lo {
            (0., if d.y > 0. { to_y(lo) } else { f64::INFINITY })
        } else if ahead.y > hi {
            (0., if d.y < 0. { to_y(hi) } else { f64::INFINITY })
        } else if !flat {
            (self.majorant(), positive(to_y(lo)).min(positive(to_y(hi))))
        } else if ahead.y < self.base {
            (self.pouch_density, positive(to_y(lo)).min(positive(to_y(self.base))))
        } else {
            (self.anvil_density, positive(to_y(self.base)).min(positive(to_y(hi))))
        };
        (m, slab.min(leave) + 0.01)
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
        (m.base_variation, m.sag, m.base_softness) = (500., 1200., 300.);
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
