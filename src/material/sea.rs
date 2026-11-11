//! The open sea, as a material for the planet's surface: waves in its normals, Fresnel
//! reflection of the sky, clouds and sun, and the faint light scattered back up out of the
//! water.
//!
//! Seen from a few metres up and out to the horizon, the sea's look is its slopes, not its
//! heights: they decide what each point reflects. Their statistics are known from C. Cox and
//! W. Munk's photographs of sun glitter ("Measurement of the roughness of the sea surface from
//! photographs of the sun's glitter", J. Opt. Soc. Am. 44, 1954): slopes are close to Gaussian,
//! with a mean square slope of 0.003 + 0.00512 U for a wind of U m/s, about 0.04 in a moderate
//! breeze. Here they are split in two:
//!
//! - **Resolved waves**, from the Phillips spectrum by Tessendorf's FFT (see `ocean`): two
//!   tiles of incommensurate sizes, one for swell and wind waves, one for waves shorter than
//!   the first can hold, so the pattern doesn't repeat visibly. They bend the surface's normal
//!   and carry `resolved` of the mean square slope.
//! - **Unresolved ripples**, below half a metre: a Beckmann distribution of microfacet normals
//!   about the resolved normal, carrying the rest. Reflections are sampled from it, and the
//!   sun's glitter is evaluated through it directly, so the glitter path is smooth rather than a
//!   scatter of rare hits on the sun's disc.
//!
//! The water reflects by Fresnel's equations, 2% straight down rising to all at grazing angles.
//! What it lets in is mostly absorbed: deep, clear water sends back a percent or two of blue
//! and almost no red (A. Morel, "Optical properties of pure water and pure sea water", 1974).
//! That light is lit by the sun and sky as they are, so the sea darkens and reddens at sunset.

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::Intersection;
use crate::material::functions::direct_lighting;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::{Vector2, Vector3};
use crate::ocean::{create_amplitude_tile, create_tile};
use crate::ray::Ray;
use crate::scene::Scene;
use rand::{rngs::StdRng, SeedableRng};
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

/// Water's refractive index.
const WATER: f64 = 1.333;

/// Fresnel reflectance of unpolarised light from air onto water, at incidence with cosine `cos_i`.
pub fn fresnel(cos_i: f64) -> f64 {
    let cos_i = cos_i.clamp(0., 1.);
    let sin_t = (1. - cos_i * cos_i).sqrt() / WATER;
    let cos_t = (1. - sin_t * sin_t).max(0.).sqrt();
    let rs = (cos_i - WATER * cos_t) / (cos_i + WATER * cos_t);
    let rp = (WATER * cos_i - cos_t) / (WATER * cos_i + cos_t);
    (rs * rs + rp * rp) / 2.
}

/// Cox and Munk's mean square slope for a wind of `speed` m/s (the clean-surface fit).
pub fn cox_munk_slope(speed: f64) -> f64 {
    0.003 + 0.00512 * speed.max(0.)
}

/// A periodic tile of surface slopes (∂h/∂x, ∂h/∂z), `size` metres across, sampled bilinearly.
struct SlopeTile {
    n: usize,
    size: f64,
    slopes: Vec<[f32; 2]>,
}

impl SlopeTile {
    /// The waves of the Phillips spectrum for `wind` with wavenumbers from `k_min` to `k_max`,
    /// on an `n` x `n` grid `size` metres across, at `time`.
    fn new(wind: Vector2<f64>, size: f64, n: usize, k_min: f64, k_max: f64, time: f64, seed: u64) -> SlopeTile {
        let mut s = [0u8; 32];
        s[..8].copy_from_slice(&seed.to_le_bytes());
        let mut rng: StdRng = SeedableRng::from_seed(s);
        let mut h0 = create_amplitude_tile(wind, 1., 9.81, size, size, n, &mut rng);
        for j in 0..n {
            for i in 0..n {
                let k = Vector2::new(i as f64 - n as f64 / 2., j as f64 - n as f64 / 2.) * (2. * PI / size);
                if k.norm() < k_min || k.norm() >= k_max {
                    h0[j * n + i] = Complex::new(0., 0.);
                }
            }
        }
        let (_, sx, sz, _, _) = create_tile(n, size, size, 9.81, time, h0);
        // Undo the centred spectrum's (-1)^(x+z) (see `ocean`).
        let slopes = (0..n * n).map(|ind| {
            let sign = if (ind % n + ind / n) % 2 == 0 { -1. } else { 1. };
            [(sx[ind].re * sign) as f32, (sz[ind].re * sign) as f32]
        }).collect();
        SlopeTile { n, size, slopes }
    }

    /// The mean square slope of the surface as sampled: between texels, interpolation smooths
    /// the shortest waves, so this is measured on a grid four times finer.
    fn mean_square(&self) -> f64 {
        let m = 4 * self.n;
        let step = self.size / m as f64;
        let mut sum = 0.;
        for j in 0..m {
            for i in 0..m {
                let s = self.at((i as f64 + 0.37) * step, (j as f64 + 0.61) * step);
                sum += s.norm_squared();
            }
        }
        sum / (m * m) as f64
    }

    fn scale(&mut self, k: f64) {
        for s in self.slopes.iter_mut() {
            s[0] *= k as f32;
            s[1] *= k as f32;
        }
    }

    fn at(&self, x: f64, z: f64) -> Vector2<f64> {
        let n = self.n as i64;
        let (u, v) = (x / self.size * self.n as f64, z / self.size * self.n as f64);
        let (u0, v0) = (u.floor(), v.floor());
        let (fu, fv) = (u - u0, v - v0);
        let t = |du: i64, dv: i64| {
            let (i, j) = ((u0 as i64 + du).rem_euclid(n), (v0 as i64 + dv).rem_euclid(n));
            let s = self.slopes[(j * n + i) as usize];
            Vector2::new(s[0] as f64, s[1] as f64)
        };
        (t(0, 0) * (1. - fu) + t(1, 0) * fu) * (1. - fv) + (t(0, 1) * (1. - fu) + t(1, 1) * fu) * fv
    }
}

pub struct Sea {
    /// Light the water body sends back up, per channel, as a Lambertian albedo.
    pub water: Color,
    /// Mean square slope of the unresolved ripples.
    roughness: f64,
    /// Planet's centre, for the local vertical; None for a flat sea (vertical y).
    center: Option<Vector3<f64>>,
    swell: SlopeTile,
    chop: SlopeTile,
}

impl Sea {
    /// A sea under a wind of `wind` ([x, z], m/s), with `resolved` of the mean square slope in
    /// waves and the rest in ripples, at `time` seconds.
    pub fn new(wind: Vector2<f64>, resolved: f64, time: f64, seed: u64, center: Option<Vector3<f64>>) -> Sea {
        let total = cox_munk_slope(wind.norm());
        // Swell and wind waves on a 487 m tile, and shorter waves on a 61.3 m one, down to half a
        // metre: incommensurate, so the two repeat together only far away.
        let (big, small, n) = (487., 61.3, 256);
        let k_split = PI * n as f64 / big * 0.9;
        let mut swell = SlopeTile::new(wind, big, n, 0., k_split, time, seed);
        let mut chop = SlopeTile::new(wind, small, n / 2, k_split, f64::INFINITY, time, seed.wrapping_add(1));
        // Phillips's constant is arbitrary: scale the two so their slopes carry the resolved
        // part, in the proportion the spectrum gives them.
        let (a, b) = (swell.mean_square(), chop.mean_square());
        let target = resolved.clamp(0., 1.) * total;
        if a + b > 0. {
            let k = (target / (a + b)).sqrt();
            swell.scale(k);
            chop.scale(k);
        }
        let resolved = swell.mean_square() + chop.mean_square();
        Sea {
            water: Color::new(0.002, 0.012, 0.03),
            roughness: (total - resolved).max(1e-4),
            center,
            swell,
            chop,
        }
    }

    /// The local vertical and two horizontal axes at `p`.
    fn frame(&self, p: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let up = self.center.map_or(Vector3::y(), |c| (p - c).normalize());
        let east = (Vector3::x() - up * up.x).normalize();
        let north = up.cross(&east);
        // east × up is -z at the origin; keep x and z as the scene's.
        (up, east, -north)
    }

    /// The resolved surface's normal at `p`.
    pub fn wave_normal(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let (up, x, z) = self.frame(p);
        let (u, v) = (p.dot(&x), p.dot(&z));
        let s = self.swell.at(u, v) + self.chop.at(u, v);
        (up - x * s.x - z * s.y).normalize()
    }

    /// A microfacet normal about `n`, from the Beckmann distribution of the ripples' slopes.
    fn sample_ripple(&self, n: &Vector3<f64>) -> Vector3<f64> {
        // Slopes are Gaussian, each component with variance roughness / 2.
        let r = (-self.roughness * (1. - rand()).ln()).sqrt();
        let phi = 2. * PI * rand();
        let (sx, sz) = (r * phi.cos(), r * phi.sin());
        let t = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::z() };
        let a = (t - n * n.dot(&t)).normalize();
        let b = n.cross(&a);
        (n - a * sx - b * sz).normalize()
    }

    /// Beckmann's distribution of microfacet normals `m` about `n`, per steradian of `m`.
    fn ripple_density(&self, n: &Vector3<f64>, m: &Vector3<f64>) -> f64 {
        let c = n.dot(m);
        if c <= 0. {
            return 0.;
        }
        let tan2 = (1. - c * c) / (c * c);
        (-tan2 / self.roughness).exp() / (PI * self.roughness * c.powi(4))
    }
}

impl MaterialModel for Sea {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let v = -r.rd.normalize();
        let geometric = intersection.facing(r).normal;
        let mut n = self.wave_normal(&intersection.point);
        if n.dot(&geometric) < 0. {
            n = geometric;
        }
        // Seen from below a wave's slope: use the mean surface.
        if n.dot(&v) <= 0. {
            n = geometric;
        }
        let into = Intersection { normal: n, ..intersection.clone() };
        // The water body, lit by the sun and lights through the surface, as much as gets in.
        let entering = 1. - fresnel(n.dot(&v));
        let mut emit = direct_lighting(self.water * entering, &into, s);
        // The sun's glitter, through the ripples: F D / (4 cos θv) per unit irradiance, with
        // Smith's shadowing left out (it matters only at grazing angles).
        if let Some(sun) = &s.sun {
            let l = sun.direction;
            if l.dot(&geometric) > 0. && n.dot(&l) > 0. {
                let h = (l + v).normalize();
                let glint = fresnel(h.dot(&v)) * self.ripple_density(&n, &h) / (4. * n.dot(&v));
                if glint > 1e-6 {
                    let lit = sun.transmittance_from(&intersection.point, s);
                    emit = emit + sun.irradiance_at(&intersection.point) * (glint * lit);
                }
            }
        }
        // Reflection of everything else from a sampled ripple, weighted by Fresnel's equations.
        let m = self.sample_ripple(&n);
        let reflected = r.rd.normalize() - m * 2. * m.dot(&r.rd.normalize());
        let ray = if reflected.dot(&geometric) > 0. && m.dot(&v) > 0. {
            Some(Ray { ro: intersection.point + geometric * 1e-3, rd: reflected })
        } else {
            None
        };
        let attenuate = if ray.is_some() { Color::white() * fresnel(m.dot(&v)) } else { Color::black() };
        ScatteredRay { emit, sampled_lights: true, attenuate, ray }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresnel() {
        assert!((fresnel(1.) - 0.0204).abs() < 1e-3);
        assert!(fresnel(0.01) > 0.9);
        assert!(fresnel(0.5) < fresnel(0.2));
    }

    #[test]
    fn test_slopes_match_cox_munk() {
        let sea = Sea::new(Vector2::new(7., 2.), 0.5, 0., 1, None);
        let total = cox_munk_slope(Vector2::new(7., 2.).norm());
        let resolved = sea.swell.mean_square() + sea.chop.mean_square();
        assert!((resolved - 0.5 * total).abs() < 1e-3 * total, "{} vs {}", resolved, total);
        assert!((resolved + sea.roughness - total).abs() < 1e-9);
        // The normals tilt about as much as the slopes say.
        let mut sum = 0.;
        let n = 20_000;
        for k in 0..n {
            let p = Vector3::new((k % 141) as f64 * 3.7, 0., (k / 141) as f64 * 3.3);
            let w = sea.wave_normal(&p);
            sum += (w.x * w.x + w.z * w.z) / (w.y * w.y);
        }
        assert!((sum / n as f64 / resolved - 1.).abs() < 0.2, "{} vs {}", sum / n as f64, resolved);
    }

    #[test]
    fn test_ripples_are_beckmann() {
        let sea = Sea::new(Vector2::new(7., 0.), 0.5, 0., 1, None);
        let n = Vector3::y();
        // Sampled microfacet normals' mean square slope is the ripples'.
        let k = 50_000;
        let mss = (0..k).map(|_| {
            let m = sea.sample_ripple(&n);
            (m.x * m.x + m.z * m.z) / (m.y * m.y)
        }).sum::<f64>() / k as f64;
        assert!((mss / sea.roughness - 1.).abs() < 0.05, "{} vs {}", mss, sea.roughness);
        // And the density, projected, integrates to one over the hemisphere.
        let steps = 400;
        let mut total = 0.;
        for i in 0..steps {
            let theta = (i as f64 + 0.5) / steps as f64 * PI / 2.;
            let m = Vector3::new(theta.sin(), theta.cos(), 0.);
            total += sea.ripple_density(&n, &m) * theta.cos() * 2. * PI * theta.sin() * (PI / 2. / steps as f64);
        }
        assert!((total - 1.).abs() < 0.01, "{}", total);
    }
}
