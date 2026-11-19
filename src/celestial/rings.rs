//! Planetary rings: a thin sheet of icy or dusty particles about a planet's equator, described
//! by its normal optical depth τ(r) and its particles' albedo and phase function.
//!
//! **Brightness.** A ring is a slab of particles a few metres to tens of metres thick, many
//! particles deep. In the classical treatment (Chandrasekhar, "Radiative Transfer", 1960; Cuzzi
//! et al., "Saturn's rings: properties and processes", in Gehrels and Matthews, "Saturn", 1984)
//! its single-scattered light, as I/F (radiance against that of a white Lambertian surface
//! square to the sun, πF the solar flux), is, on the lit face,
//!
//!   I/F = ϖ P(α) / 4 · μ₀ / (μ + μ₀) · [1 - exp(-τ (1/μ + 1/μ₀))]
//!
//! and, seen from the unlit face, the light scattered through it,
//!
//!   I/F = ϖ P(α) / 4 · μ₀ / (μ - μ₀) · [exp(-τ/μ) - exp(-τ/μ₀)]
//!
//! for particle albedo ϖ, phase function P (averaging 1 over the sphere), phase angle α and
//! μ, μ₀ the cosines of the view and the sun to the ring plane. So an optically thick ring
//! (Saturn's B ring, τ ≈ 2-5) is bright on its lit face and dark from below, a thin one (the C
//! ring, the Cassini Division, τ ≈ 0.1) faint on both but glowing in forward scattering when
//! its particles are dust; and a ring seen nearly edge-on (μ small) brightens, as the path
//! through it lengthens, until it saturates. Light passes straight through as exp(-τ/μ), so
//! the planet and the stars show through the thinner rings, and the rings' shadow on the
//! planet is exp(-τ/μ₀) of the sunlight. On the lit face, multiple scattering between
//! particles adds Hapke's H-function term (scaled by the same factor for the ring's
//! thickness), which matters for the brightest, thickest rings.
//!
//! The particles of Saturn's rings are rough chunks of water ice, cm to m across, slightly
//! reddened by organics and silicates: albedo ϖ ≈ 0.5-0.6 in the A and B rings and darker
//! (0.2-0.3) where the rings are thin, polluted more by meteoroid dust (Cuzzi and Estrada,
//! Icarus 132, 1998). They scatter strongly backwards, with a sharp opposition surge at zero
//! phase. Uranus' rings are narrow and nearly black (ϖ ≈ 0.02-0.05).
//!
//! **Structure.** Saturn's preset follows the measured radial profile (Cassini UVIS and radio
//! occultations: Colwell et al., in Dougherty et al., "Saturn from Cassini-Huygens", 2009):
//! the D, C, B and A rings, the Cassini Division, the Huygens, Encke and Keeler gaps and the
//! narrow F ring, with ringlets as fine noise. Generated rings are built the way rings are:
//! within the planet's Roche limit, a few broad regions of different τ, gaps cleared at the
//! inner Lindblad resonances of moons outside (the Cassini Division is at Mimas' 2:1), and
//! narrower gaps swept by moonlets embedded in them (Encke's Pan).
//!
//! **Filtering.** τ is tabulated finely (a few km) and kept with its running sum, so the mean τ
//! over any width, a pixel's footprint or the sun's, is two lookups: the rings never alias.

use crate::celestial::hapke::Hapke;
use crate::color::Color;
use crate::noise::SeededNoise;
use crate::procedural::space::{mix, smoothstep};
use crate::na::Vector3;
use std::f64::consts::PI;

const TABLE: usize = 32_768;

#[derive(Debug, Clone)]
pub struct Rings {
    /// Inner and outer radii, metres.
    pub inner: f64,
    pub outer: f64,
    /// Running sums of τ over the table's cells (TABLE + 1 entries).
    sums: Vec<f64>,
    /// The particles' albedo where the rings are thick, and where they are thin.
    pub bright: Color,
    pub dark: Color,
    /// The particles' phase function and opposition surge.
    pub photometry: Hapke,
}

/// A ring region: from and to radius (km), optical depth, and how much it varies in ringlets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Band {
    pub from: f64,
    pub to: f64,
    pub tau: f64,
    pub ringlets: f64,
}

fn band(from: f64, to: f64, tau: f64, ringlets: f64) -> Band {
    Band { from, to, tau, ringlets }
}

/// Saturn's ring particles: backward-scattering rough ice, and a strong opposition surge from
/// the shadows the particles of a packed layer hide at zero phase.
pub const ICY_PARTICLES: Hapke = Hapke { b: 0.3, c: 0.7, b0: 1.5, h: 0.1 };
/// Fine dust (Neptune's and Jupiter's faint rings, Saturn's D and E): forward scattering.
pub const DUSTY_PARTICLES: Hapke = Hapke { b: 0.6, c: -0.8, b0: 0., h: 1. };

fn unit(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

impl Rings {
    /// Rings from their bands (later ones override earlier where they overlap; radii in km),
    /// with ringlets from `seed`. Each band's edges are softened over `edge` km.
    pub fn from_bands(bands: &[Band], seed: u64, edge: f64, bright: Color, dark: Color, photometry: Hapke) -> Rings {
        let inner = bands.iter().map(|b| b.from).fold(f64::INFINITY, f64::min);
        let outer = bands.iter().map(|b| b.to).fold(0., f64::max);
        let noise = SeededNoise::new(mix(seed ^ 0x7269_6e67));
        let step = (outer - inner) / TABLE as f64;
        let mut sums = Vec::with_capacity(TABLE + 1);
        sums.push(0.);
        let mut total = 0.;
        for i in 0..TABLE {
            let r = inner + (i as f64 + 0.5) * step;
            let mut tau = 0.;
            let mut ringlets = 0.;
            for b in bands {
                let e = edge.min((b.to - b.from) / 4.);
                let inside = smoothstep(b.from - e, b.from + e, r) * (1. - smoothstep(b.to - e, b.to + e, r));
                if inside > 0. {
                    tau = tau * (1. - inside) + b.tau * inside;
                    ringlets = ringlets * (1. - inside) + b.ringlets * inside;
                }
            }
            // Ringlets: structure on every scale from hundreds of km to the table's own.
            if ringlets > 0. {
                let n = noise.fbm(Vector3::new(r / 900., 0.3, 0.7), 8) * 1.4 + noise.noise(Vector3::new(r / 23., 5.1, 2.2)) * 0.6;
                tau *= (ringlets * n * 2.).exp();
            }
            total += tau * step;
            sums.push(total);
        }
        Rings { inner: inner * 1e3, outer: outer * 1e3, sums, bright, dark, photometry }
    }

    /// The mean normal optical depth over radii `r ± width / 2` (metres).
    pub fn tau(&self, r: f64, width: f64) -> f64 {
        let span = self.outer - self.inner;
        let cell = |x: f64| ((x - self.inner) / span * TABLE as f64).clamp(0., TABLE as f64);
        let w = width.max(span / TABLE as f64);
        let (a, b) = (cell(r - w / 2.), cell(r + w / 2.));
        if b <= a {
            return 0.;
        }
        let sum = |x: f64| {
            let i = (x as usize).min(TABLE - 1);
            let f = x - i as f64;
            self.sums[i] * (1. - f) + self.sums[i + 1] * f
        };
        // Sums are in km of τ: the mean over the cells is their difference over the width.
        (sum(b) - sum(a)) / ((b - a) * span / 1e3 / TABLE as f64)
    }

    /// The particles' albedo for a ring of optical depth `tau`: darker where thin.
    pub fn albedo(&self, tau: f64) -> Color {
        let t = smoothstep(0.05, 1.2, tau);
        self.dark * (1. - t) + self.bright * t
    }

    /// I/F of the ring (normal optical depth `tau`) for the cosines `mu0` (sun) and `mu`
    /// (view) to the ring plane, both positive, phase angle `g`, and whether the sun lights the
    /// face that is seen; per channel.
    pub fn i_over_f(&self, tau: f64, mu0: f64, mu: f64, g: f64, lit_face: bool) -> Color {
        if tau <= 0. || mu0 <= 0. || mu <= 0. {
            return Color::black();
        }
        let (mu0, mu) = (mu0.max(1e-4), mu.max(1e-4));
        let ph = &self.photometry;
        let p = (1. + ph.opposition(g.clamp(0., PI - 1e-6))) * ph.phase(g.clamp(0., PI - 1e-6));
        let w = self.albedo(tau).as_reflectance().rgb;
        let c = if lit_face {
            let depth = 1. - (-tau * (1. / mu + 1. / mu0)).exp();
            let geometry = mu0 / (mu + mu0) * depth / 4.;
            w.map(|w| w * geometry * (p + Hapke::h_function(w, mu0) * Hapke::h_function(w, mu) - 1.))
        } else {
            let through = if (mu - mu0).abs() < 1e-6 {
                tau / mu0 * (-tau / mu0).exp()
            } else {
                mu0 / (mu - mu0) * ((-tau / mu).exp() - (-tau / mu0).exp())
            };
            w.map(|w| w * p / 4. * through.max(0.))
        };
        Color { rgb: c }
    }

    /// Saturn's rings (see the module docs), radii in km.
    pub fn saturn(seed: u64) -> Rings {
        Rings::from_bands(&[
            band(66_900., 74_490., 0.002, 0.3),           // D
            band(74_490., 91_980., 0.1, 0.35),            // C, with its plateaus
            band(84_750., 85_300., 0.4, 0.1),
            band(86_400., 86_700., 0.35, 0.1),
            band(88_400., 89_000., 0.4, 0.1),
            band(91_980., 99_000., 1.2, 0.3),             // B1
            band(99_000., 104_500., 2.5, 0.35),           // B2
            band(104_500., 110_000., 4.5, 0.25),          // B3, the core
            band(110_000., 117_507., 2.0, 0.35),          // B4
            band(117_507., 122_340., 0.12, 0.4),          // Cassini Division
            band(117_680., 117_965., 0.0, 0.),            // Huygens gap
            band(122_340., 133_420., 0.6, 0.2),           // A
            band(126_000., 131_000., 0.75, 0.25),
            band(133_420., 133_745., 0.002, 0.),          // Encke gap
            band(133_745., 136_485., 0.45, 0.15),
            band(136_485., 136_525., 0.0, 0.),            // Keeler gap
            band(136_525., 136_775., 0.4, 0.1),
            band(136_775., 140_160., 0.0, 0.),            // Roche Division
            band(140_160., 140_210., 0.5, 0.),            // F
        ], seed, 15., Color::new(0.72, 0.62, 0.50), Color::new(0.30, 0.27, 0.23), ICY_PARTICLES)
    }

    /// Uranus' narrow, nearly black rings (Elliot and Nicholson, "The rings of Uranus", in
    /// Greenberg and Brahic, "Planetary Rings", 1984), widths exaggerated a little.
    pub fn uranus(seed: u64) -> Rings {
        let ring = |r: f64, w: f64, tau: f64| band(r - w / 2., r + w / 2., tau, 0.);
        Rings::from_bands(&[
            band(41_000., 51_250., 0.0, 0.),
            ring(41_837., 3., 0.3), ring(42_234., 3., 0.5), ring(42_571., 3., 0.3),
            ring(44_718., 8., 0.4), ring(45_661., 9., 0.25), ring(47_176., 2., 0.4),
            ring(47_627., 4., 1.2), ring(48_300., 7., 0.5), ring(50_023., 2., 0.1),
            ring(51_149., 60., 1.0),
        ], seed, 0.5, Color::new(0.05, 0.05, 0.05), Color::new(0.03, 0.03, 0.03), ICY_PARTICLES)
    }

    /// Neptune's faint dusty rings: Galle, Le Verrier, Lassell and Adams.
    pub fn neptune(seed: u64) -> Rings {
        Rings::from_bands(&[
            band(40_900., 42_900., 1e-4, 0.), band(53_150., 53_260., 0.02, 0.),
            band(53_200., 57_200., 1e-4, 0.), band(62_910., 62_950., 0.05, 0.),
        ], seed, 5., Color::new(0.06, 0.06, 0.06), Color::new(0.04, 0.04, 0.04), DUSTY_PARTICLES)
    }

    /// Rings of their own from `seed`, for a planet of equatorial radius `radius` (m): see the
    /// module docs.
    pub fn generated(seed: u64, radius: f64) -> Rings {
        let r = |k: u64| unit(mix(mix(seed ^ 0x7269_6e67) ^ mix(k)));
        let a = radius / 1e3;
        // Inside the Roche limit for ice, about 2.4 planetary radii for a giant's density.
        let inner = a * (1.1 + 0.25 * r(1));
        let outer = a * (1.8 + 0.5 * r(2));
        let mut bands = vec![];
        let regions = 2 + (r(3) * 5.) as u64;
        let mut edges: Vec<f64> = (0..regions - 1).map(|i| inner + (outer - inner) * r(10 + i)).collect();
        edges.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let mut from = inner;
        for (i, to) in edges.iter().copied().chain(std::iter::once(outer)).enumerate() {
            // Optical depths log-uniform from 0.01 to 4.
            let tau = 0.01 * 400f64.powf(r(20 + i as u64));
            bands.push(band(from, to, tau, 0.15 + 0.3 * r(30 + i as u64)));
            from = to;
        }
        // Moons outside clear gaps at their inner 2:1 and 3:2 resonances.
        for m in 0..(r(40) * 3.) as u64 {
            let moon = outer * (1.05 + 0.8 * r(41 + m));
            for (k, ratio) in [(0u64, 0.5f64), (1, 2. / 3.)] {
                let at = moon * ratio.powf(2. / 3.);
                if at > inner && at < outer {
                    let width = (outer - inner) * (0.005 + 0.04 * r(50 + m * 2 + k));
                    bands.push(band(at - width / 2., at + width / 2., 0.03 * r(60 + m), 0.4));
                }
            }
        }
        // Moonlets embedded in the rings open narrow gaps.
        for g in 0..(r(70) * 3.) as u64 {
            let at = inner + (outer - inner) * r(71 + g);
            let width = (outer - inner) * (0.001 + 0.004 * r(80 + g));
            bands.push(band(at - width / 2., at + width / 2., 0., 0.));
        }
        // A narrow shepherded ring outside, sometimes.
        if r(90) < 0.5 {
            let at = outer * (1.02 + 0.05 * r(91));
            bands.push(band(at, at + 20. + 200. * r(92), 0.2 + r(93), 0.));
        }
        // Icy and bright, or dark and dusty.
        let (bright, dark, photometry) = if r(94) < 0.7 {
            let red = 0.1 * r(95);
            (Color::new(0.62, 0.55 - red, 0.46 - 2. * red) * (0.8 + 0.3 * r(96)), Color::new(0.27, 0.24, 0.21), ICY_PARTICLES)
        } else {
            (Color::new(0.07, 0.065, 0.06), Color::new(0.04, 0.04, 0.04), ICY_PARTICLES)
        };
        Rings::from_bands(&bands, seed, (outer - inner) * 0.002, bright, dark, photometry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturn_profile() {
        let s = Rings::saturn(1);
        let t = |km: f64| s.tau(km * 1e3, 200e3);
        // The B ring is opaque, the C ring and Cassini Division thin, the A ring between.
        assert!(t(107_000.) > 2., "B {}", t(107_000.));
        assert!(t(80_000.) < 0.3, "C {}", t(80_000.));
        assert!(t(120_000.) < 0.3, "CD {}", t(120_000.));
        let a = t(128_000.);
        assert!(a > 0.3 && a < 1.5, "A {}", a);
        // The Encke gap is nearly empty.
        assert!(s.tau(133_580e3, 100e3) < 0.05, "Encke {}", s.tau(133_580e3, 100e3));
        // Outside the rings, nothing.
        assert_eq!(s.tau(150_000e3, 1e3), 0.);
    }

    #[test]
    fn test_tau_is_filtered() {
        // The mean over a wide footprint is the mean of the fine values.
        let s = Rings::saturn(2);
        let wide = s.tau(105_000e3, 2_000e3);
        let fine: f64 = (0..2000).map(|i| s.tau(104_000e3 + (i as f64 + 0.5) * 1e3, 1e3)).sum::<f64>() / 2000.;
        assert!((wide / fine - 1.).abs() < 0.02, "{} {}", wide, fine);
    }

    #[test]
    fn test_b_ring_brightness() {
        // Saturn's B ring seen near opposition with the sun 20° above the ring plane: I/F
        // about 0.5 in the visible (Cuzzi et al., Cassini; Dones et al., Icarus 105, 1993).
        let s = Rings::saturn(1);
        let mu = 20f64.to_radians().sin();
        let i = crate::color::luminance(&s.i_over_f(s.tau(107_000e3, 500e3), mu, mu, 6f64.to_radians(), true));
        assert!(i > 0.3 && i < 0.8, "{}", i);
        // From the unlit side the B ring is far darker than the A ring, which lets more through.
        let below = |km: f64| crate::color::luminance(&s.i_over_f(s.tau(km * 1e3, 500e3), mu, 0.5, 2., false));
        assert!(below(107_000.) < 0.3 * below(128_000.), "{} {}", below(107_000.), below(128_000.));
    }

    #[test]
    fn test_unlit_face_limit_is_continuous() {
        let s = Rings::saturn(1);
        let a = s.i_over_f(0.5, 0.3, 0.3, 2., false);
        let b = s.i_over_f(0.5, 0.3, 0.3 + 1e-4, 2., false);
        assert!((a.rgb - b.rgb).norm() < 1e-3 * a.rgb.norm(), "{:?} {:?}", a.rgb, b.rgb);
    }

    #[test]
    fn test_generated_rings_lie_within_the_roche_limit() {
        for seed in 0..20 {
            let r = Rings::generated(seed, 60_000e3);
            assert!(r.inner > 60_000e3 && r.outer < 60_000e3 * 2.6, "{} {}", r.inner, r.outer);
            assert!(r.tau((r.inner + r.outer) / 2., (r.outer - r.inner) / 2.) > 0.);
        }
    }
}
