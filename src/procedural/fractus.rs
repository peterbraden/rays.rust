//! Fractus: ragged shreds of low cloud, torn and carried by the wind. A density field for
//! `volume::Volume`.
//!
//! Fractus forms when a strong wind near the ground mixes damp air up past its condensation
//! level in eddies, or tears pieces from the base of a cloud, especially under rain (pannus) and
//! as a cumulus forms or dies. The shreds are small, a few hundred metres across, with ragged,
//! ever-changing edges; they hang below other cloud or in clear sky, drift fast with the wind,
//! and are drawn out along it (WMO International Cloud Atlas: stratus fractus and cumulus
//! fractus).
//!
//! The field:
//!
//! 1. **Shreds.** On a jittered lattice `spacing` apart, each cell holds a shred with chance
//!    `coverage`, at a height of its own between `base` and `base + depth`, `size` across,
//!    `stretch` times as long along the wind as across it, and leaning downwind (torn from the
//!    air above, its top runs ahead of its bottom).
//! 2. **Tearing.** Each shred's outline is fractal noise, thresholded: so it falls apart into
//!    lumps and wisps with gaps between, and its edges are wispy (`raggedness`).
//! 3. **Density.** Thin, at most `density`, and thinner towards its edges: fractus is
//!    translucent grey or white.

use crate::na::{Vector2, Vector3};
use crate::noise::{worley_hash, PerlinNoise};
use crate::procedural::nubis_clouds::{lerp, remap, smoothstep};
use crate::volume::DensityField;

pub struct Fractus {
    /// The lowest shreds' height, and the depth of the layer they drift in (m).
    pub base: f64,
    pub depth: f64,
    /// Lattice spacing between shreds, and the share of cells holding one.
    pub spacing: f64,
    pub coverage: f64,
    /// A shred's size across the wind (m), and how much longer it is along it.
    pub size: f64,
    pub stretch: f64,
    pub wind_direction: Vector2<f64>,
    /// How far its outline is torn (0 to 1).
    pub raggedness: f64,
    pub density: f64,
    /// The layer's extent about `center` ([x, z]).
    pub center: Vector2<f64>,
    pub extent: f64,
    seed: u32,
    noise: PerlinNoise,
}

impl Fractus {
    pub fn new(seed: u64) -> Fractus {
        Fractus {
            base: 500.,
            depth: 600.,
            spacing: 1500.,
            coverage: 0.6,
            size: 450.,
            stretch: 2.5,
            wind_direction: Vector2::new(1., 0.),
            raggedness: 0.6,
            density: 0.03,
            center: Vector2::zeros(),
            extent: 40_000.,
            seed: seed as u32,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(17).wrapping_add(3)),
        }
    }

    pub fn bottom(&self) -> f64 {
        self.base - 0.5 * self.size
    }

    pub fn top(&self) -> f64 {
        self.base + self.depth + 0.5 * self.size
    }

    /// The fraction of `density` at `p`, 0 to 1.
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        let rel = Vector2::new(p.x, p.z) - self.center;
        if rel.x.abs() > self.extent || rel.y.abs() > self.extent {
            return 0.;
        }
        let w = self.wind_direction;
        let (along, across) = (p.x * w.x + p.z * w.y, -p.x * w.y + p.z * w.x);
        // The lattice in the wind's frame: cells stretched along it like the shreds.
        let (u, v) = (along / (self.spacing * self.stretch.sqrt()), across / self.spacing);
        let (i0, j0) = (u.floor() as i64, v.floor() as i64);
        let mut best: f64 = 0.;
        for i in i0 - 1..=i0 + 1 {
            for j in j0 - 1..=j0 + 1 {
                let h = |k: u32| worley_hash(i as u32, j as u32, 0, self.seed.wrapping_add(k));
                if h(0) >= self.coverage {
                    continue;
                }
                let centre_u = (i as f64 + 0.15 + 0.7 * h(1)) * self.spacing * self.stretch.sqrt();
                let centre_v = (j as f64 + 0.15 + 0.7 * h(2)) * self.spacing;
                let height = self.base + self.depth * h(3);
                let size = self.size * (0.6 + 0.8 * h(4));
                // Leaning downwind: higher parts further along.
                let lean = 0.8 * (p.y - height);
                let du = (along - centre_u - lean) / (size * self.stretch);
                let dv = (across - centre_v) / size;
                let dy = (p.y - height) / (0.45 * size);
                let r = (du * du + dv * dv + dy * dy).sqrt();
                if r >= 1.3 {
                    continue;
                }
                // Torn: it falls apart into a few lumps and many wisps, with sky between. Coarse
                // noise, a third of the shred across, cuts it into pieces; finer noise wears
                // their edges; and the density inside follows the noise rather than filling the
                // outline, so each shred is uneven, thin in places.
                let q = Vector3::new(along / (0.33 * size * self.stretch), p.y / (0.33 * size), across / (0.33 * size));
                // (Stretched: Perlin noise's values hug its middle.)
                let coarse = (0.5 + 1.8 * (self.noise.fbm(q.x + 31. * h(5), q.y, q.z + 17. * h(6), 2, 0.5, 2.) - 0.5)).clamp(0., 1.);
                let fine = self.noise.fbm(q.x * 4. + 7.1, q.y * 4., q.z * 4. + 2.3, 3, 0.55, 2.1);
                let solid = 1. - smoothstep(0.3, 1.2, r);
                let pieces = solid * (1. + 2.4 * self.raggedness * (coarse - 0.5) * 2.) + 0.8 * self.raggedness * (fine - 0.5);
                let torn = remap(pieces, 0.45, 0.85).powf(1.5);
                best = best.max(torn * lerp(0.6, 1., h(7)));
            }
        }
        best
    }
}

impl DensityField for Fractus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.density
    }

    /// A slab: `density` within it, nothing above or below.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let (lo, hi) = (self.bottom(), self.top());
        let to = |y: f64| if d.y != 0. { (y - p.y) / d.y } else { f64::INFINITY };
        if p.y < lo {
            (0., if d.y > 0. { to(lo) } else { f64::INFINITY })
        } else if p.y > hi {
            (0., if d.y < 0. { to(hi) } else { f64::INFINITY })
        } else {
            (self.density, if d.y > 0. { to(hi) } else if d.y < 0. { to(lo) } else { f64::INFINITY })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    #[test]
    fn test_shreds_are_scattered_ragged_and_bounded() {
        let f = Fractus::new(2);
        let mut rng = Pcg32::new(1, 0);
        let (mut cloud, n) = (0, 40_000);
        for _ in 0..n {
            let p = Vector3::new(rng.next_f64() * 20000. - 10000., f.bottom() + rng.next_f64() * (f.top() - f.bottom()), rng.next_f64() * 20000. - 10000.);
            let d = f.density(&p);
            assert!((0. ..=f.density).contains(&d));
            if d > 0. { cloud += 1; }
        }
        // Mostly clear air, with shreds in it.
        let share = cloud as f64 / n as f64;
        assert!(share > 0.002 && share < 0.2, "{}", share);
        // Nothing outside the layer.
        assert_eq!(f.density(&Vector3::new(0., f.top() + 1., 0.)), 0.);
        assert_eq!(f.density(&Vector3::new(0., f.bottom() - 1., 0.)), 0.);
    }

    #[test]
    fn test_majorant_bounds_density() {
        let f = Fractus::new(4);
        let mut rng = Pcg32::new(3, 0);
        for _ in 0..500 {
            let p = Vector3::new(rng.next_f64() * 8000. - 4000., rng.next_f64() * 2000., rng.next_f64() * 8000. - 4000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = f.majorant_from(&p, &d);
            for k in 0..30 {
                let q = p + d * (len.min(5000.) * k as f64 / 30.);
                assert!(f.density(&q) <= m + 1e-12);
            }
        }
    }
}
