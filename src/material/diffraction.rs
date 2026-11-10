//! Diffraction by cloud particles: the source of cloud iridescence and coronae.
//!
//! A particle much larger than the wavelength removes twice the light its cross-section
//! intercepts (the "extinction paradox", van de Hulst, "Light Scattering by Small Particles",
//! 1957, section 8.22): half is refracted and reflected through the particle, scattering
//! broadly, and half is diffracted around it into a narrow forward lobe. For a disc of radius
//! a, or a sphere seen as one, the lobe is the Fraunhofer (Airy) pattern,
//!
//! ```text
//! A(v) = (2 J₁(v) / v)²,   v = k a sin θ,   k = 2π / λ,
//! ```
//!
//! whose width, about λ / 2a, depends on the wavelength. Near the sun, thin cloud of small,
//! similar particles therefore splits sunlight into colours: rings when the size is uniform over
//! a patch of sky (a corona), and pastel bands following the cloud's edges when it varies
//! (iridescence, typically in young, thin cloud whose particles are newly formed and small).
//! Larger particles diffract into a narrower lobe, close to the sun, so the colour seen at an
//! angle maps out the particle size (C. F. Bohren and E. E. Clothiaux, "Fundamentals of
//! Atmospheric Radiation", Wiley 2006, section 3.5; Cowley, Laven and Vollmer, "Rings around
//! the sun and moon: coronae and diffraction", Physics Education 40(1), 2005).
//!
//! The renderer has three colour channels; each is the mean of three wavelengths across its
//! band, which keeps the colours from being too pure. Real particles vary in size: the pattern
//! is averaged over radii a(1 + s), s Gaussian with a relative spread, weighted by
//! cross-section, so all but the first rings wash out unless the size is very uniform.
//!
//! Everything scales with v: averaged over sizes, the lobe for mean radius a is
//! (ka)² B(ka sin θ) for one universal function B(v) = E[(1+s)⁴ A((1+s) v)] / E[(1+s)²],
//! tabulated once with its cumulative ∫B(v) v dv for sampling. The lobe used here is
//!
//! ```text
//! p(θ) = (ka)² B(v) cos θ / (2π C(v_max)),   C(v) = ∫₀ᵛ B(u) u du,  v_max = min(ka, V),
//! ```
//!
//! in the forward hemisphere: the small-angle Airy lobe with an obliquity factor cos θ that
//! makes it normalise exactly over the hemisphere, and sample exactly (draw v from B(v) v,
//! then sin θ = v / ka). Beyond the first few degrees it is negligible anyway.

use crate::color::Color;
use crate::na::Vector3;
use std::f64::consts::PI;

/// Wavelengths (micrometres) averaged for each of the red, green and blue channels.
pub const WAVELENGTHS: [[f64; 3]; 3] = [[0.58, 0.61, 0.65], [0.51, 0.55, 0.57], [0.43, 0.465, 0.49]];

/// Largest v tabulated; beyond it the lobe carries well under 1% of the light.
const V_MAX: f64 = 200.;
const V_STEPS: usize = 20_000;

/// The Bessel function of the first kind of order one: the power series for small x, and
/// Hankel's asymptotic expansion for large (Abramowitz and Stegun, "Handbook of Mathematical
/// Functions", 9.1.10 and 9.2.5-9.2.10), each well within 1e-9 where it is used.
pub fn bessel_j1(x: f64) -> f64 {
    let ax = x.abs();
    let j = if ax < 20. {
        // Σ (-1)^m (x/2)^(2m+1) / (m! (m+1)!)
        let h = ax / 2.;
        let mut term = h;
        let mut sum = term;
        let mut m = 0.;
        while term.abs() > 1e-17 * sum.abs().max(1e-300) || m < 2. {
            m += 1.;
            term *= -h * h / (m * (m + 1.));
            sum += term;
            if m > 200. {
                break;
            }
        }
        sum
    } else {
        // J1(x) = sqrt(2 / πx) (P cos χ - Q sin χ), χ = x - 3π/4, with μ = 4ν² = 4.
        let mu = 4.;
        let z = 8. * ax;
        let (mut p, mut q) = (1., 0.);
        let mut term = 1.;
        // Terms (μ - 1²)(μ - 3²)...(μ - (2k-1)²) / (k! z^k), alternating between P and Q.
        for k in 1..12 {
            let odd = (2 * k - 1) as f64;
            term *= (mu - odd * odd) / (k as f64 * z);
            match k % 4 {
                1 => q += term,
                2 => p -= term,
                3 => q -= term,
                _ => p += term,
            }
        }
        let chi = ax - 0.75 * PI;
        (2. / (PI * ax)).sqrt() * (p * chi.cos() - q * chi.sin())
    };
    if x < 0. { -j } else { j }
}

/// The Airy pattern, (2 J₁(v) / v)², 1 at v = 0.
pub fn airy(v: f64) -> f64 {
    if v.abs() < 1e-8 {
        return 1.;
    }
    let a = 2. * bessel_j1(v) / v;
    a * a
}

/// B(v) and its cumulative ∫₀ᵛ B(u) u du, tabulated, for one relative spread of sizes.
#[derive(Debug)]
pub struct DiffractionTable {
    pub spread: f64,
    b: Vec<f64>,
    cumulative: Vec<f64>,
}

impl DiffractionTable {
    pub fn new(spread: f64) -> DiffractionTable {
        // Gaussian quadrature over s by sampling ±3 spread; a single size with no spread.
        let sizes: Vec<(f64, f64)> = if spread <= 0. {
            vec![(1., 1.)]
        } else {
            let n = 48;
            (0..n).map(|i| {
                let t = -3. + 6. * (i as f64 + 0.5) / n as f64;
                ((1. + spread * t).max(0.05), (-0.5 * t * t).exp())
            }).collect()
        };
        let norm: f64 = sizes.iter().map(|(s, w)| w * s * s).sum();
        let dv = V_MAX / V_STEPS as f64;
        let b: Vec<f64> = (0..=V_STEPS).map(|i| {
            let v = i as f64 * dv;
            sizes.iter().map(|(s, w)| w * s.powi(4) * airy(s * v)).sum::<f64>() / norm
        }).collect();
        let mut cumulative = vec![0.; V_STEPS + 1];
        for i in 1..=V_STEPS {
            let (v0, v1) = ((i - 1) as f64 * dv, i as f64 * dv);
            cumulative[i] = cumulative[i - 1] + 0.5 * (b[i - 1] * v0 + b[i] * v1) * dv;
        }
        DiffractionTable { spread, b, cumulative }
    }

    fn lookup(table: &[f64], v: f64) -> f64 {
        let x = (v / V_MAX * V_STEPS as f64).clamp(0., V_STEPS as f64);
        let i = (x.floor() as usize).min(V_STEPS - 1);
        let f = x - i as f64;
        table[i] * (1. - f) + table[i + 1] * f
    }

    fn b(&self, v: f64) -> f64 {
        Self::lookup(&self.b, v)
    }

    fn cumulative(&self, v: f64) -> f64 {
        Self::lookup(&self.cumulative, v)
    }

    /// v at which the cumulative reaches `c`, by bisection over the table.
    fn invert(&self, c: f64) -> f64 {
        let i = self.cumulative.partition_point(|x| *x < c).clamp(1, V_STEPS);
        let (c0, c1) = (self.cumulative[i - 1], self.cumulative[i]);
        let f = if c1 > c0 { (c - c0) / (c1 - c0) } else { 0. };
        ((i - 1) as f64 + f) * V_MAX / V_STEPS as f64
    }

    /// The diffraction lobe for one wavelength (µm) and mean particle radius (µm), at the
    /// cosine of the scattering angle.
    pub fn eval(&self, cos_theta: f64, wavelength: f64, radius: f64) -> f64 {
        if cos_theta <= 0. {
            return 0.;
        }
        let ka = 2. * PI * radius / wavelength;
        let sin = (1. - cos_theta * cos_theta).max(0.).sqrt();
        let v = ka * sin;
        let v_max = ka.min(V_MAX);
        if v >= v_max {
            return 0.;
        }
        ka * ka * self.b(v) * cos_theta / (2. * PI * self.cumulative(v_max))
    }

    /// The lobe per colour channel: each the mean over its wavelengths.
    pub fn eval_color(&self, cos_theta: f64, radius: f64) -> Color {
        let channel = |c: usize| WAVELENGTHS[c].iter().map(|l| self.eval(cos_theta, *l, radius)).sum::<f64>() / 3.;
        Color::new(channel(0), channel(1), channel(2))
    }

    /// A direction around `incoming` distributed exactly as `eval` for `wavelength` and
    /// `radius`, from two uniform numbers.
    pub fn sample(&self, incoming: &Vector3<f64>, wavelength: f64, radius: f64, u1: f64, u2: f64) -> Vector3<f64> {
        let ka = 2. * PI * radius / wavelength;
        let v_max = ka.min(V_MAX);
        let v = self.invert(u1 * self.cumulative(v_max)).min(v_max);
        let sin = (v / ka).min(1.);
        let cos = (1. - sin * sin).sqrt();
        let phi = 2. * PI * u2;
        let (t, b) = crate::material::phase::orthonormal_basis(incoming);
        (t * (sin * phi.cos()) + b * (sin * phi.sin()) + incoming * cos).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_j1() {
        // Reference values (Abramowitz and Stegun, table 9.1), and the first zero, 3.8317.
        for (x, j) in [(1., 0.4400505857), (2., 0.5767248078), (5., -0.3275791376), (10., 0.0434727462)] {
            assert!((bessel_j1(x) - j).abs() < 1e-9, "J1({}) = {} not {}", x, bessel_j1(x), j);
        }
        assert!(bessel_j1(3.831705970).abs() < 1e-8);
        assert!((bessel_j1(-2.) + bessel_j1(2.)).abs() < 1e-15);
        // The series and the asymptotic expansion meet at 20.
        assert!((bessel_j1(20. - 1e-9) - bessel_j1(20. + 1e-9)).abs() < 1e-8);
        // ∫₀^∞ J₁(v)² / v dv = 1/2; the tail beyond X is about 1/(2πX).
        let (n, x_max) = (400_000, 2000.);
        let sum: f64 = (0..n).map(|i| {
            let v = (i as f64 + 0.5) * x_max / n as f64;
            bessel_j1(v).powi(2) / v
        }).sum::<f64>() * x_max / n as f64;
        assert!((sum + 1. / (2. * PI * x_max) - 0.5).abs() < 1e-4, "{}", sum);
    }

    /// Integral over the hemisphere and the fraction within `within` radians, by quadrature in θ.
    fn integrate(t: &DiffractionTable, wavelength: f64, radius: f64, within: f64) -> (f64, f64) {
        let n = 200_000;
        let (mut total, mut inner) = (0., 0.);
        for i in 0..n {
            let theta = (i as f64 + 0.5) / n as f64 * PI / 2.;
            let w = t.eval(theta.cos(), wavelength, radius) * 2. * PI * theta.sin() * (PI / 2.) / n as f64;
            total += w;
            if theta < within {
                inner += w;
            }
        }
        (total, inner)
    }

    #[test]
    fn test_lobe_is_normalised_and_narrows_with_size() {
        let t = DiffractionTable::new(0.1);
        for radius in [3., 8., 20.] {
            let (total, _) = integrate(&t, 0.55, radius, 0.);
            assert!((total - 1.).abs() < 2e-3, "radius {}: {}", radius, total);
        }
        // Most of the light within a few times λ / 2a of the sun.
        let (_, near_small) = integrate(&t, 0.55, 5., 0.05);
        let (_, near_large) = integrate(&t, 0.55, 20., 0.05);
        assert!(near_small > 0.7 && near_large > near_small, "{} {}", near_small, near_large);
    }

    #[test]
    fn test_first_dark_ring_of_uniform_particles() {
        // With one size, the pattern is dark at v = 3.8317: sin θ = 3.8317 λ / 2πa, further out
        // for red than blue.
        let t = DiffractionTable::new(0.);
        let a = 6.;
        for l in [0.45, 0.65] {
            let sin = 3.831706 / (2. * PI * a / l);
            let cos = (1. - sin * sin).sqrt();
            let peak = t.eval(1., l, a);
            assert!(t.eval(cos, l, a) < 1e-4 * peak, "{}: {}", l, t.eval(cos, l, a) / peak);
        }
        // So at the blue dark ring it is still red: colour.
        let sin = 3.831706 / (2. * PI * a / 0.465);
        let c = t.eval_color((1. - sin * sin).sqrt(), a);
        assert!(c.rgb.x > 3. * c.rgb.z, "{:?}", c);
    }

    #[test]
    fn test_spread_washes_out_the_rings() {
        // The second bright ring, near v = 5.14, against the first dark one at 3.83.
        let ratio = |spread: f64| {
            let t = DiffractionTable::new(spread);
            t.b(5.14) / t.b(3.83)
        };
        assert!(ratio(0.) > 100.);
        assert!(ratio(0.25) < 3., "{}", ratio(0.25));
    }

    #[test]
    fn test_sampling_follows_eval() {
        crate::geometry::seed_rand(4, 0);
        let t = DiffractionTable::new(0.1);
        let (l, a) = (0.55, 6.);
        let incoming = Vector3::new(0.2, -0.3, 0.9).normalize();
        let n = 200_000;
        let bands = [(0., 0.02), (0.02, 0.05), (0.05, 0.12)];
        let mut counts = [0usize; 3];
        for _ in 0..n {
            let d = t.sample(&incoming, l, a, crate::geometry::rand(), crate::geometry::rand());
            assert!((d.norm() - 1.).abs() < 1e-9);
            let theta = d.dot(&incoming).clamp(-1., 1.).acos();
            for (k, (lo, hi)) in bands.iter().enumerate() {
                counts[k] += (theta >= *lo && theta < *hi) as usize;
            }
        }
        for (k, (lo, hi)) in bands.iter().enumerate() {
            let steps = 20_000;
            let expected: f64 = (0..steps).map(|i| {
                let th = lo + (hi - lo) * (i as f64 + 0.5) / steps as f64;
                t.eval(th.cos(), l, a) * 2. * PI * th.sin()
            }).sum::<f64>() * (hi - lo) / steps as f64;
            let got = counts[k] as f64 / n as f64;
            assert!((got - expected).abs() < 0.004, "band {:?}: {} vs {}", bands[k], got, expected);
        }
    }
}
