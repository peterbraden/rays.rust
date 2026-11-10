//! Bump mapping: surfaces that look rough, rippled or dented without changing their shape, by
//! tilting the normal that materials shade with (James F. Blinn, "Simulation of wrinkled
//! surfaces", SIGGRAPH 1978). The patterns are modelled on POV-Ray's `normal` patterns of the
//! same names (POV-Ray 3.7 reference: "Normal"), though not identical to them.
//!
//! Each pattern is a height field h over space. The surface normal n is tilted against the part of
//! h's gradient that lies along the surface: n' = normalize(n - amount (g - (g.n) n)), as if the
//! surface were displaced outwards by amount x h. The gradient is taken in the pattern's own
//! space (before scaling), so `amount` gives the same strength of bumps at any scale.

use crate::na::Vector3;
use crate::material::model::MaterialModel;
use crate::material::texture::Medium;
use crate::noise::PerlinNoise;
use std::f64::consts::PI;
use std::sync::LazyLock;

static PERLIN: LazyLock<PerlinNoise> = LazyLock::new(PerlinNoise::new);

/// Sources for ripples and waves: fixed points scattered through the unit cube, as POV-Ray uses.
static SOURCES: LazyLock<Vec<Vector3<f64>>> = LazyLock::new(|| {
    // A small linear congruential generator, so the sources are the same in every render.
    let mut state: u64 = 0x2545_f491_4f6c_dd1d;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (state >> 11) as f64 / (1u64 << 53) as f64
    };
    (0..10).map(|_| Vector3::new(next(), next(), next())).collect()
});

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    /// Smooth random bumps: Perlin noise.
    Bumps,
    /// Sharp dents: Perlin noise cubed, so most of the surface is flat with occasional pits.
    Dents,
    /// Creased, crumpled wrinkles: turbulence, a sum of the magnitudes of noise at `octaves`
    /// doubling frequencies.
    Wrinkles { octaves: u32 },
    /// Concentric ripples from ten sources, `frequency` rings per unit, shifted by `phase` (in
    /// cycles).
    Ripples { frequency: f64, phase: f64 },
    /// Like ripples, but each source with its own wavelength, longer waves stronger.
    Waves { frequency: f64, phase: f64 },
}

/// A pattern of bumps, scaled and applied with some strength.
#[derive(Debug, Clone, Copy)]
pub struct Bump {
    pub pattern: Pattern,
    pub amount: f64,
    /// Size of the pattern along each axis.
    pub scale: Vector3<f64>,
}

impl Bump {
    /// The height field, at a point in the pattern's space.
    pub fn height(&self, q: &Vector3<f64>) -> f64 {
        match self.pattern {
            Pattern::Bumps => PERLIN.noise(q.x, q.y, q.z),
            Pattern::Dents => PERLIN.noise(q.x, q.y, q.z).powi(3),
            Pattern::Wrinkles { octaves } => {
                let (mut sum, mut f) = (0., 1.);
                for _ in 0..octaves.max(1) {
                    sum += PERLIN.noise(q.x * f, q.y * f, q.z * f).abs() / f;
                    f *= 2.;
                }
                sum
            },
            Pattern::Ripples { frequency, phase } => {
                SOURCES.iter().map(|c| (2. * PI * ((q - c).norm() * frequency + phase)).cos()).sum::<f64>()
                    / SOURCES.len() as f64
            },
            Pattern::Waves { frequency, phase } => {
                SOURCES.iter().enumerate().map(|(i, c)| {
                    let f = frequency * (1. + i as f64 * 0.25);
                    (2. * PI * ((q - c).norm() * f + phase)).cos() / f
                }).sum::<f64>() * frequency / SOURCES.len() as f64
            },
        }
    }

    /// The normal at world point `p`, tilted from the surface normal `n` by the pattern.
    pub fn perturb(&self, p: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
        let q = p.component_div(&self.scale);
        let h = 1e-3;
        let mut g = Vector3::zeros();
        for a in 0..3 {
            let mut e = Vector3::zeros();
            e[a] = h;
            g[a] = (self.height(&(q + e)) - self.height(&(q - e))) / (2. * h);
        }
        // From the pattern's space to the world's, keeping the strength independent of size.
        let mean = (self.scale.x.abs() + self.scale.y.abs() + self.scale.z.abs()) / 3.;
        let g = g.component_div(&self.scale) * mean;
        let tangential = g - n * g.dot(n);
        let tilted = n - tangential * self.amount;
        if tilted.norm() > 0. { tilted.normalize() } else { *n }
    }
}

/// A medium whose surfaces are bump mapped.
pub struct Bumped {
    pub inner: Box<dyn Medium + Sync + Send>,
    pub bump: Bump,
}

impl Medium for Bumped {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        self.inner.material_at(pt)
    }

    fn normal_at(&self, pt: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        self.bump.perturb(pt, &self.inner.normal_at(pt, normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bump(pattern: Pattern, scale: f64) -> Bump {
        Bump { pattern, amount: 0.5, scale: Vector3::repeat(scale) }
    }

    const ALL: [Pattern; 5] = [
        Pattern::Bumps, Pattern::Dents, Pattern::Wrinkles { octaves: 4 },
        Pattern::Ripples { frequency: 3., phase: 0. }, Pattern::Waves { frequency: 3., phase: 0. },
    ];

    #[test]
    fn test_tilted_normals_are_unit_and_on_the_same_side() {
        let n = Vector3::new(0., 1., 0.);
        for pattern in ALL {
            let b = bump(pattern, 0.3);
            let mut tilted = 0;
            for i in 0..200 {
                let p = Vector3::new(i as f64 * 0.137, 0., i as f64 * 0.071);
                let m = b.perturb(&p, &n);
                assert!((m.norm() - 1.).abs() < 1e-12, "{:?}", pattern);
                assert!(m.dot(&n) > 0., "{:?} flipped at {}", pattern, p);
                if (m - n).norm() > 1e-3 { tilted += 1; }
            }
            assert!(tilted > 100, "{:?} barely tilts: {}", pattern, tilted);
        }
    }

    #[test]
    fn test_no_amount_no_tilt() {
        let b = Bump { pattern: Pattern::Bumps, amount: 0., scale: Vector3::repeat(1.) };
        let n = Vector3::new(0., 0., -1.);
        assert_eq!(b.perturb(&Vector3::new(0.3, 0.7, 0.1), &n), n);
    }

    #[test]
    fn test_strength_does_not_depend_on_scale() {
        // The same pattern at ten times the size, sampled at the corresponding points, tilts by
        // the same angles.
        let (small, big) = (bump(Pattern::Bumps, 0.1), bump(Pattern::Bumps, 1.));
        let n = Vector3::new(0., 1., 0.);
        for i in 0..20 {
            let p = Vector3::new(i as f64 * 0.031, 0.05, i as f64 * 0.017);
            let (a, b) = (small.perturb(&p, &n), big.perturb(&(p * 10.), &n));
            assert!((a - b).norm() < 1e-6, "{} vs {}", a, b);
        }
    }

    #[test]
    fn test_normal_leans_downhill() {
        // As for a surface raised by the height field, the tilted normal leans towards where the
        // height falls: stepping along its tangential part lowers the height.
        let n = Vector3::new(0., 1., 0.);
        for pattern in ALL {
            let b = bump(pattern, 1.);
            for i in 0..50 {
                let p = Vector3::new(i as f64 * 0.173, 0.21, i as f64 * 0.059);
                let m = b.perturb(&p, &n);
                let along = m - n * m.dot(&n);
                if along.norm() < 1e-6 { continue; }
                let step = along.normalize() * 1e-4;
                assert!(b.height(&(p + step)) < b.height(&p), "{:?} at {}", pattern, p);
            }
        }
    }

    #[test]
    fn test_phase_is_in_cycles() {
        let r = |phase| Bump { pattern: Pattern::Ripples { frequency: 2., phase }, amount: 1., scale: Vector3::repeat(1.) };
        let p = Vector3::new(0.3, 0.4, 0.5);
        assert!((r(0.).height(&p) - r(1.).height(&p)).abs() < 1e-12);
        assert!((r(0.).height(&p) - r(0.5).height(&p)).abs() > 1e-6);
    }
}
