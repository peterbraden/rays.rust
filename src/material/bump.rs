//! Bump mapping: surfaces that look rough, rippled or dented without changing their shape, by
//! tilting the normal that materials shade with (James F. Blinn, "Simulation of wrinkled
//! surfaces", SIGGRAPH 1978). The patterns are modelled on POV-Ray's `normal` patterns of the
//! same names (POV-Ray 3.7 reference: "Normal"), though not identical to them.
//!
//! Each pattern is a height field h over space. The surface normal n is tilted against the part of
//! h's gradient that lies along the surface: n' = normalize(n - amount (g - (g.n) n)), as if the
//! surface were displaced outwards by amount x h. The gradient is taken in the pattern's own
//! space (before scaling), so `amount` gives the same strength of bumps at any scale.
//!
//! Or the height is any function graph (`graph.rs`), in scene units, times `amount`: the same
//! graph a pattern texture colours by (`textures::PatternTexture`), so the relief and the
//! colours line up, as they do in real materials, where colour and relief come from the same
//! structure (veins, grains, pores, cracks). Its gradient is taken across the surface by
//! central differences a pixel's footprint apart, of the graph filtered to that footprint, so
//! relief finer than a pixel fades rather than glittering. A material can carry its own bump
//! (`"normal"` in its definition: `BumpedMaterial`), so it keeps it wherever it is used.

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

#[derive(Clone)]
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
    /// A function graph's value, in scene units.
    Graph(std::sync::Arc<crate::graph::Graph>),
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Pattern::Bumps => write!(f, "Bumps"),
            Pattern::Dents => write!(f, "Dents"),
            Pattern::Wrinkles { octaves } => write!(f, "Wrinkles {{ octaves: {} }}", octaves),
            Pattern::Ripples { frequency, phase } => write!(f, "Ripples {{ {}, {} }}", frequency, phase),
            Pattern::Waves { frequency, phase } => write!(f, "Waves {{ {}, {} }}", frequency, phase),
            Pattern::Graph(_) => write!(f, "Graph"),
        }
    }
}

/// A pattern of bumps, scaled and applied with some strength.
#[derive(Debug, Clone)]
pub struct Bump {
    pub pattern: Pattern,
    pub amount: f64,
    /// Size of the pattern along each axis.
    pub scale: Vector3<f64>,
}

impl Bump {
    /// The height field, at a point in the pattern's space.
    pub fn height(&self, q: &Vector3<f64>) -> f64 {
        match &self.pattern {
            Pattern::Bumps => PERLIN.noise(q.x, q.y, q.z),
            Pattern::Dents => PERLIN.noise(q.x, q.y, q.z).powi(3),
            Pattern::Wrinkles { octaves } => {
                let (mut sum, mut f) = (0., 1.);
                for _ in 0..(*octaves).max(1) {
                    sum += PERLIN.noise(q.x * f, q.y * f, q.z * f).abs() / f;
                    f *= 2.;
                }
                sum
            },
            Pattern::Ripples { frequency, phase } => {
                SOURCES.iter().map(|c| (2. * PI * ((q - c).norm() * frequency + phase)).cos()).sum::<f64>()
                    / SOURCES.len() as f64
            },
            Pattern::Graph(g) => g.eval(&crate::graph::Context::lifted(*q, Vector3::y())),
            Pattern::Waves { frequency, phase } => {
                SOURCES.iter().enumerate().map(|(i, c)| {
                    let f = frequency * (1. + i as f64 * 0.25);
                    (2. * PI * ((q - c).norm() * f + phase)).cos() / f
                }).sum::<f64>() * frequency / SOURCES.len() as f64
            },
        }
    }

    /// The normal at a hit, tilted from `n` (the hit's normal, perhaps already tilted) by the
    /// pattern, filtered to the pixel's footprint there.
    pub fn perturb_at(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene, n: &Vector3<f64>) -> Vector3<f64> {
        match &self.pattern {
            Pattern::Graph(g) => {
                let c = crate::graph::Context::at_hit_filtered(i, s);
                // Where a displaced surface was displaced from; the bump tilts the hit's normal.
                // The relief is read in the object's own space, as its colours are.
                let frame = i.object.geometry.pattern_frame(&i.point, i.face);
                self.perturb_graph(g, &crate::graph::Context { normal: *n, ..c }, n, frame.as_ref())
            },
            _ => self.perturb(&i.point, n),
        }
    }

    /// A graph's relief: n tilted against the graph's gradient along the surface, by central
    /// differences across the footprint.
    fn perturb_graph(&self, g: &crate::graph::Graph, c: &crate::graph::Context, n: &Vector3<f64>, frame: Option<&crate::na::Affine3<f64>>) -> Vector3<f64> {
        let a = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
        let u = n.cross(&a).normalize();
        let v = n.cross(&u);
        // Half the footprint, so relief finer than a pixel averages out of the differences
        // rather than tilting the normal at random; where there is no footprint, a tenth of the
        // graph's finest detail.
        let e = if c.filter > 0. { 0.5 * c.filter } else { (0.1 * g.finest()).min(1e-3) }.max(1e-6);
        // Heights in the object's units, scaled with it to the scene's.
        let size = frame.map_or(1., |f| 1. / f.matrix().fixed_view::<3, 3>(0, 0).determinant().abs().cbrt().max(1e-300));
        let h = |p: Vector3<f64>| {
            let at = crate::graph::Context { p, ..*c };
            g.eval(&frame.map_or(at, |f| at.in_frame(f))) * size
        };
        let du = (h(c.p + u * e) - h(c.p - u * e)) / (2. * e);
        let dv = (h(c.p + v * e) - h(c.p - v * e)) / (2. * e);
        let tilted = n - (u * du + v * dv) * self.amount;
        tilted.try_normalize(1e-12).unwrap_or(*n)
    }

    /// The normal at world point `p`, tilted from the surface normal `n` by the pattern.
    pub fn perturb(&self, p: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
        if let Pattern::Graph(g) = &self.pattern {
            return self.perturb_graph(g, &crate::graph::Context::lifted(*p, *n), n, None);
        }
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

    fn normal_at_hit(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Vector3<f64> {
        self.bump.perturb_at(i, s, &self.inner.normal_at_hit(i, s))
    }

    fn part(&self, k: u32) -> Option<&(dyn Medium + Sync + Send)> {
        self.inner.part(k)
    }

    fn fields(&self) -> Option<&dyn crate::graph::Fields> { self.inner.fields() }
}

/// A material with its own bump map, applied over whatever its object's medium does.
pub struct BumpedMaterial {
    pub inner: Box<dyn MaterialModel + Sync + Send>,
    pub bump: Bump,
}

impl MaterialModel for BumpedMaterial {
    fn direct_emit(&self) -> bool { self.inner.direct_emit() }

    /// The material under the bump: at a vertex's scale its normal is the surface's.
    fn bake(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Option<crate::material::model::Baked> {
        self.inner.bake(i, s)
    }

    fn scatter(&self, r: &crate::ray::Ray, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> crate::material::model::ScatteredRay {
        self.inner.scatter(r, i, s)
    }

    fn shading_normal(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Option<Vector3<f64>> {
        let n = self.inner.shading_normal(i, s).unwrap_or(i.normal);
        Some(self.bump.perturb_at(i, s, &n))
    }

    fn is_emitter(&self) -> bool { self.inner.is_emitter() }

    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> crate::color::Color { self.inner.emitted(normal, towards) }

    fn emitted_at(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> crate::color::Color {
        self.inner.emitted_at(normal, towards, uv)
    }

    fn emitted_in(&self, c: &crate::graph::Context, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> crate::color::Color {
        self.inner.emitted_in(c, towards, uv)
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
            let b = bump(pattern.clone(), 0.3);
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
            let b = bump(pattern.clone(), 1.);
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
    fn test_graph_relief_tilts_by_its_slope() {
        // A ramp rising 0.5 per unit along x tilts a level surface's normal back by atan(0.5).
        let g = crate::graph::Graph::parse(&serde_json::json!({"node": "expr", "expr": "0.5 * x"})).unwrap();
        let b = Bump { pattern: Pattern::Graph(std::sync::Arc::new(g)), amount: 1., scale: Vector3::repeat(1.) };
        let m = b.perturb(&Vector3::new(0.3, 0., 0.2), &Vector3::y());
        let expected = Vector3::new(-0.5, 1., 0.).normalize();
        assert!((m - expected).norm() < 1e-6, "{}", m);
        // Twice the amount, twice the slope.
        let b2 = Bump { amount: 2., ..b };
        assert!((b2.perturb(&Vector3::zeros(), &Vector3::y()) - Vector3::new(-1., 1., 0.).normalize()).norm() < 1e-6);
    }

    #[test]
    fn test_relief_finer_than_the_footprint_fades() {
        // Stones a centimetre across: seen from close, they tilt the normal a lot; with a
        // footprint of a metre, hardly at all, rather than at random.
        let g = crate::graph::Graph::parse(&serde_json::json!({"node": "cells", "style": "stones", "scale": 0.01, "amplitude": 0.005})).unwrap();
        let b = Bump { pattern: Pattern::Graph(std::sync::Arc::new(g.clone())), amount: 1., scale: Vector3::repeat(1.) };
        let tilt = |filter: f64| (0 .. 200).map(|k| {
            let c = crate::graph::Context::new(Vector3::new(k as f64 * 0.0137, 0., k as f64 * 0.0071), Vector3::y(), filter);
            1. - b.perturb_graph(&g, &c, &Vector3::y(), None).y
        }).sum::<f64>() / 200.;
        assert!(tilt(0.0005) > 0.05, "{}", tilt(0.0005));
        assert!(tilt(1.) < 1e-4, "{}", tilt(1.));
    }

    #[test]
    fn test_phase_is_in_cycles() {
        let r = |phase| Bump { pattern: Pattern::Ripples { frequency: 2., phase }, amount: 1., scale: Vector3::repeat(1.) };
        let p = Vector3::new(0.3, 0.4, 0.5);
        assert!((r(0.).height(&p) - r(1.).height(&p)).abs() < 1e-12);
        assert!((r(0.).height(&p) - r(0.5).height(&p)).abs() > 1e-6);
    }
}
