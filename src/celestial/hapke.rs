//! Hapke's photometric model of a particulate surface: how bright regolith is for any angles of
//! incidence, emission and phase.
//!
//! B. Hapke, "Bidirectional reflectance spectroscopy 1. Theory", JGR 86(B4), 1981, and
//! "Theory of Reflectance and Emittance Spectroscopy", 2nd ed., Cambridge 2012, chapters 9-10:
//!
//! r(i, e, g) = w / 4π · μ₀ / (μ₀ + μ) · [(1 + B(g)) P(g) + H(μ₀) H(μ) - 1]
//!
//! for single-scattering albedo w, μ₀ and μ the cosines of incidence and emission, and g the
//! phase angle. r is the radiance per unit irradiance on a surface facing the light, so a
//! Lambertian surface of albedo a has r = a μ₀ / π. The μ₀ / (μ₀ + μ) factor is the
//! Lommel-Seeliger law of single scattering, which is why the full moon is as bright at its limb
//! as at its centre, a flat disc rather than a shaded ball; `P` is the particles' phase function
//! (one Henyey-Greenstein lobe, scattering mostly backwards, as dark rough grains do), `B` the
//! opposition effect (shadow hiding: near zero phase each grain hides its own shadow, and the
//! full moon brightens sharply), and `H` Chandrasekhar's function for multiple scattering, in
//! Hapke's approximation H(x) = (1 + 2x) / (1 + 2γx), γ = √(1 - w).
//!
//! The Moon's parameters (`Hapke::MOON`) were fitted to its disc-integrated phase curve, the
//! magnitude at phase angle α being V(α) = V(0) + 0.026|α| + 4·10⁻⁹ α⁴ (Allen's Astrophysical
//! Quantities, 4th ed., section 12.4, α in degrees), and its geometric albedo, 0.12: they give
//! the curve within 13% out to 150°, and that albedo for w = 0.19.

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hapke {
    /// The width of the particles' phase lobe, as a Henyey-Greenstein |g|: 0 isotropic.
    pub b: f64,
    /// The share of that lobe scattering backwards (1) rather than forwards (-1): the
    /// double Henyey-Greenstein weighting of Hapke's 2012 book, eq. 6.7b.
    pub c: f64,
    /// The amplitude of the opposition effect at zero phase.
    pub b0: f64,
    /// Its angular width (as tan(g / 2)).
    pub h: f64,
}

impl Hapke {
    /// Fitted to the Moon's phase curve (see the module docs).
    pub const MOON: Hapke = Hapke { b: 0.27, c: 1., b0: 1.07, h: 0.18 };

    /// The particles' phase function at phase angle `g`: a mix of a backward and a forward
    /// Henyey-Greenstein lobe, normalised to average 1 over the sphere.
    pub fn phase(&self, g: f64) -> f64 {
        let (b, cg) = (self.b, g.cos());
        let back = (1. - b * b) / (1. - 2. * b * cg + b * b).powf(1.5);
        let forward = (1. - b * b) / (1. + 2. * b * cg + b * b).powf(1.5);
        (1. + self.c) / 2. * back + (1. - self.c) / 2. * forward
    }

    /// The shadow-hiding opposition effect at phase angle `g`.
    pub fn opposition(&self, g: f64) -> f64 {
        self.b0 / (1. + (g / 2.).tan() / self.h)
    }

    /// Hapke's approximation to Chandrasekhar's H function, for single-scattering albedo `w`.
    pub fn h_function(w: f64, x: f64) -> f64 {
        let gamma = (1. - w).max(0.).sqrt();
        (1. + 2. * x) / (1. + 2. * gamma * x)
    }

    /// The bidirectional reflectance r: radiance per unit irradiance on a surface facing the
    /// light, for single-scattering albedo `w`, cosines of incidence `mu0` and emission `mu`,
    /// and phase angle `g` (radians). Zero unless both cosines are positive.
    pub fn reflectance(&self, w: f64, mu0: f64, mu: f64, g: f64) -> f64 {
        if mu0 <= 0. || mu <= 0. {
            return 0.;
        }
        let g = g.clamp(0., PI - 1e-6);
        w / (4. * PI) * mu0 / (mu0 + mu)
            * ((1. + self.opposition(g)) * self.phase(g) + Hapke::h_function(w, mu0) * Hapke::h_function(w, mu) - 1.)
    }

    /// The normal albedo of a surface of single-scattering albedo `w`: its brightness seen and
    /// lit face on, against a white Lambertian surface's, π r(0, 0, 0).
    pub fn normal_albedo(&self, w: f64) -> f64 {
        PI * self.reflectance(w, 1., 1., 0.)
    }

    /// The table inverting `normal_albedo`, for surfaces described by the albedo one would
    /// measure.
    pub fn albedo_table(&self) -> AlbedoTable {
        let max = self.normal_albedo(1. - 1e-9);
        let w = (0..ALBEDO_TABLE).map(|i| {
            let a = max * i as f64 / (ALBEDO_TABLE - 1) as f64;
            // normal_albedo increases with w: bisect.
            let (mut lo, mut hi) = (0., 1.);
            for _ in 0..50 {
                let mid = (lo + hi) / 2.;
                if self.normal_albedo(mid) < a { lo = mid } else { hi = mid }
            }
            (lo + hi) / 2.
        }).collect();
        AlbedoTable { max, w }
    }
}

const ALBEDO_TABLE: usize = 512;

/// Single-scattering albedos by normal albedo (`Hapke::albedo_table`).
#[derive(Debug, Clone)]
pub struct AlbedoTable {
    max: f64,
    w: Vec<f64>,
}

impl AlbedoTable {
    /// The single-scattering albedo giving normal albedo `a` (clamped to what is possible).
    pub fn w(&self, a: f64) -> f64 {
        let x = (a / self.max).clamp(0., 1.) * (ALBEDO_TABLE - 1) as f64;
        let i = (x as usize).min(ALBEDO_TABLE - 2);
        let f = x - i as f64;
        self.w[i] * (1. - f) + self.w[i + 1] * f
    }
}

/// A sphere's brightness against a white Lambertian disc of the same size facing the light
/// and the observer: its geometric albedo times its phase function, p Φ(g), at phase angle
/// `g`, for a surface everywhere of reflectance `r(mu0, mu, g)`. By quadrature over the disc.
pub fn disc_brightness(g: f64, r: impl Fn(f64, f64, f64) -> f64) -> f64 {
    // The observer is along +z, the light in the xz plane; the disc is sampled on a grid of
    // its projection, where each sample's area is the surface's projected area.
    let n = 200;
    let light = (g.sin(), g.cos());
    let mut sum = 0.;
    for i in 0..n {
        for j in 0..n {
            let x = -1. + 2. * (i as f64 + 0.5) / n as f64;
            let y = -1. + 2. * (j as f64 + 0.5) / n as f64;
            let rho2 = x * x + y * y;
            if rho2 >= 1. {
                continue;
            }
            let z = (1. - rho2).sqrt();
            sum += r(x * light.0 + z * light.1, z, g);
        }
    }
    // A white Lambertian disc, r = 1 / π over an area π, sums to 1.
    sum * (2. / n as f64).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_phase_curve() {
        // The Moon's whole-disc brightness through its phases, against Allen's magnitudes.
        let m = Hapke::MOON;
        let at = |deg: f64| disc_brightness(deg.to_radians(), |mu0, mu, g| m.reflectance(0.19, mu0, mu, g));
        let full = at(2.);
        for &(deg, tolerance) in &[(10f64, 0.05), (30., 0.05), (60., 0.1), (90., 0.15), (120., 0.1), (150., 0.25)] {
            let allen = 10f64.powf(-0.4 * (0.026 * (deg - 2.) + 4e-9 * (deg.powi(4) - 16.)));
            let v = at(deg) / full;
            assert!((v / allen - 1.).abs() < tolerance, "{}°: {} vs Allen's {}", deg, v, allen);
        }
        // The geometric albedo, near zero phase.
        assert!((full - 0.12).abs() < 0.01, "{}", full);
    }

    #[test]
    fn test_full_moon_is_a_flat_disc() {
        // Lommel-Seeliger: at zero phase, the limb is as bright as the centre (a Lambertian
        // ball would be dark at its edge).
        let m = Hapke::MOON;
        let centre = m.reflectance(0.19, 1., 1., 0.);
        let limb = m.reflectance(0.19, 0.2, 0.2, 0.);
        assert!((limb / centre - 1.).abs() < 0.15, "{} {}", limb, centre);
    }

    #[test]
    fn test_albedo_table_inverts_normal_albedo() {
        let m = Hapke::MOON;
        let t = m.albedo_table();
        for &w in &[0.05, 0.19, 0.5, 0.9] {
            let a = m.normal_albedo(w);
            // What matters is the albedo it gives back.
            assert!((m.normal_albedo(t.w(a)) / a - 1.).abs() < 1e-3, "{} -> {} -> {}", w, a, t.w(a));
        }
        assert!(t.w(0.) < 1e-9);
    }

    #[test]
    fn test_reflectance_is_reciprocal() {
        // r / μ₀, the BRDF, is symmetric in its two directions.
        let m = Hapke::MOON;
        let f = |mu0: f64, mu: f64| m.reflectance(0.6, mu0, mu, 0.7) / mu0;
        assert!((f(0.3, 0.8) - f(0.8, 0.3)).abs() < 1e-12);
        assert_eq!(m.reflectance(0.6, -0.1, 0.5, 0.3), 0.);
    }
}
