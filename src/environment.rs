//! # Sampling the sky in proportion to its brightness
//!
//! A surface lit by the sky gathers light from every direction above it. Sampling those
//! directions by the BSDF alone (for a diffuse surface, by the cosine) is blind to where the sky
//! is bright: under a sunset, the glow near the horizon is found only by the few rays that
//! happen to go that way. Light sampling for the sky picks directions in proportion to the sky's
//! radiance instead, and multiple importance sampling (`material::bsdf`) combines the two, so
//! each counts most where it does best.
//!
//! The sky's radiance is a function of direction (and, slowly, of where it is seen from). It is
//! tabulated once, before rendering, from the camera's position, over an equirectangular grid:
//! u = φ / 2π across, v = θ / π down from the zenith. Sampling a cell in proportion to its
//! tabulated value, then a point uniformly within it, gives a density p(u, v) over the unit
//! square that is piecewise constant. Pharr, Jakob and Humphreys, "Physically Based Rendering",
//! 4th ed., section A.5.5, "Piecewise-Constant 2D Distributions"
//! (<https://pbr-book.org/4ed/Sampling_Algorithms/Sampling_Multidimensional_Functions>):
//! sample v from the marginal density of the rows, then u from the chosen row's conditional
//! density, each by inverting its cumulative distribution (section A.4.7).
//!
//! The grid's cells shrink towards the poles: a cell at angle θ from the zenith covers a solid
//! angle sin θ times as large as one at the horizon, for the same area of (u, v). So each cell's
//! value is its radiance times sin θ, and densities convert as
//!
//! ```text
//! p(ω) = p(u, v) / (2π² sin θ)
//! ```
//!
//! (the Jacobian |∂(u, v)/∂ω| = 1 / (2π² sin θ); PBR 3rd ed., section 14.2.4,
//! <https://pbr-book.org/3ed-2018/Light_Transport_I_Surface_Reflection/Sampling_Light_Sources>;
//! the 4th edition uses an equal-area mapping, which needs no such factor).
//!
//! Much of the sky is hidden from most of a scene: below the horizon, by the ground. Samples
//! spent there are wasted, so each cell is also weighted by whether it can be seen from the
//! camera (the transmittance of a ray towards it), a guess at what the surfaces in view can
//! see. Where it guesses wrong, sampling is merely less efficient.
//!
//! The table only shapes the sampling. Radiance is always evaluated exactly, from the shading
//! point, by the sky itself (`radiance`): the table needn't be accurate, only roughly
//! proportional, and the estimate stays unbiased wherever it's off. Where the table is zero (a
//! black sky below the horizon, say) light sampling can't reach, and BSDF sampling, weighted
//! one there by the power heuristic, covers it alone.
//!
//! The sun's disc is a separate light (`light::Sun`): the sky here is what an escaping ray sees
//! besides it.
use crate::color::Color;
use crate::intersection::Intersection;
use crate::light::SUN_SHADOW_DISTANCE;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sceneobject::SceneObject;
use crate::shapes::geometry::Geometry;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::sync::Arc;

/// Columns (azimuth) and rows (angle from the zenith) of the table.
const WIDTH: usize = 128;
const HEIGHT: usize = 64;

/// A piecewise-constant density on [0, 1) of n equal pieces, in proportion to `func`.
/// PBR 4th ed., section A.4.7.
pub struct Distribution1D {
    func: Vec<f64>,
    /// cdf[i] = ∫ of the density up to i / n; cdf[0] = 0, cdf[n] = 1.
    cdf: Vec<f64>,
    /// ∫ func over [0, 1): the mean of func.
    integral: f64,
}

impl Distribution1D {
    pub fn new(func: Vec<f64>) -> Distribution1D {
        let n = func.len();
        let mut cdf = vec![0.; n + 1];
        for i in 0..n {
            cdf[i + 1] = cdf[i] + func[i] / n as f64;
        }
        let integral = cdf[n];
        for c in cdf.iter_mut() {
            *c = if integral > 0. { *c / integral } else { 0. };
        }
        Distribution1D { func, cdf, integral }
    }

    /// x in [0, 1) from uniform `u`, its density, and the piece it lies in.
    pub fn sample(&self, u: f64) -> (f64, f64, usize) {
        let n = self.func.len();
        // The last i with cdf[i] <= u.
        let i = self.cdf.partition_point(|&c| c <= u).saturating_sub(1).min(n - 1);
        let width = self.cdf[i + 1] - self.cdf[i];
        let du = if width > 0. { (u - self.cdf[i]) / width } else { 0. };
        ((i as f64 + du) / n as f64, self.func[i] / self.integral, i)
    }

    /// The density at piece `i`.
    pub fn pdf(&self, i: usize) -> f64 {
        if self.integral > 0. { self.func[i] / self.integral } else { 0. }
    }
}

/// A piecewise-constant density on [0, 1)², rows of `width` values: the marginal density of
/// the rows, and each row's conditional density across it. PBR 4th ed., section A.5.5.
pub struct Distribution2D {
    rows: Vec<Distribution1D>,
    marginal: Distribution1D,
    width: usize,
}

impl Distribution2D {
    pub fn new(func: &[f64], width: usize) -> Distribution2D {
        let rows: Vec<Distribution1D> = func.chunks(width).map(|r| Distribution1D::new(r.to_vec())).collect();
        let marginal = Distribution1D::new(rows.iter().map(|r| r.integral).collect());
        Distribution2D { rows, marginal, width }
    }

    /// (u, v) from uniform `u`, and its density.
    pub fn sample(&self, u: [f64; 2]) -> ((f64, f64), f64) {
        let (v, pv, row) = self.marginal.sample(u[1]);
        let (x, px, _) = self.rows[row].sample(u[0]);
        ((x, v), pv * px)
    }

    /// The density at (u, v).
    pub fn pdf(&self, u: f64, v: f64) -> f64 {
        let row = ((v * self.rows.len() as f64) as usize).min(self.rows.len() - 1);
        let col = ((u * self.width as f64) as usize).min(self.width - 1);
        self.marginal.pdf(row) * self.rows[row].pdf(col)
    }

    /// The integral of the function over [0, 1)²: zero for one that is zero everywhere.
    pub fn integral(&self) -> f64 {
        self.marginal.integral
    }
}

/// Relative luminance of linear sRGB (ITU-R BT.709 primaries).
fn luminance(c: &Color) -> f64 {
    0.2126 * c.rgb.x + 0.7152 * c.rgb.y + 0.0722 * c.rgb.z
}

/// The direction for (u, v): azimuth 2π u about the y axis (the zenith), angle π v from it.
fn direction(u: f64, v: f64) -> Vector3<f64> {
    let (theta, phi) = (PI * v, 2. * PI * u);
    Vector3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin())
}

/// (u, v) for unit direction `d`, and sin θ.
fn coordinates(d: &Vector3<f64>) -> (f64, f64, f64) {
    let theta = d.y.clamp(-1., 1.).acos();
    let phi = d.z.atan2(d.x);
    let u = if phi < 0. { phi / (2. * PI) + 1. } else { phi / (2. * PI) };
    (u.min(1. - 1e-12), (theta / PI).min(1. - 1e-12), theta.sin())
}

/// The sky as a light: what a ray that escapes the scene sees (besides the sun's disc), and a
/// distribution over directions roughly in proportion to its brightness.
pub struct Environment {
    /// The skysphere, if any; otherwise the scene's background.
    sky: Option<Arc<SceneObject>>,
    distribution: Distribution2D,
}

impl Environment {
    /// Tabulate the scene's sky as seen from the camera, or None if none of it is visible.
    pub fn build(s: &Scene) -> Option<Environment> {
        let origin = s.camera.get_ray(0.5, 0.5, 0., 0.).ro;
        let up = Ray { ro: origin, rd: Vector3::new(0., 1., 0.) };
        // The skysphere reports every ray hitting it, infinitely far off.
        let sky = s.objects.infinite_items.iter()
            .find(|o| o.intersects(&up).is_some_and(|i| i.dist >= SUN_SHADOW_DISTANCE))
            .cloned();
        let mut env = Environment { sky, distribution: Distribution2D::new(&[0.], 1) };
        let values: Vec<f64> = (0 .. WIDTH * HEIGHT).into_par_iter().map(|k| {
            let (u, v) = ((k % WIDTH) as f64 + 0.5, (k / WIDTH) as f64 + 0.5);
            let (u, v) = (u / WIDTH as f64, v / HEIGHT as f64);
            let ray = Ray { ro: origin, rd: direction(u, v) };
            let visible = s.objects.transmittance(&ray, SUN_SHADOW_DISTANCE, f64::MIN_POSITIVE);
            luminance(&env.radiance(&ray, s)).max(0.) * visible * (PI * v).sin()
        }).collect();
        env.distribution = Distribution2D::new(&values, WIDTH);
        if env.distribution.integral() > 0. { Some(env) } else { None }
    }

    /// The radiance a ray sees on escaping the scene: the skysphere's, or the background's.
    pub fn radiance(&self, r: &Ray, s: &Scene) -> Color {
        match &self.sky {
            None => s.render.background.radiance(&r.rd),
            Some(sky) => match sky.intersects(r) {
                None => Color::black(),
                Some(raw) => {
                    let i = Intersection { dist: raw.dist, point: raw.point, normal: raw.normal, face: raw.face, part: raw.part, object: sky };
                    let scattered = sky.medium.material_at(i.point).scatter(r, &i, s);
                    // Skies end the path: their light is all in emit and attenuate.
                    if scattered.ray.is_some() { Color::black() } else { scattered.emit + scattered.attenuate }
                },
            },
        }
    }

    /// A direction from uniform `u`, and its density per steradian.
    pub fn sample(&self, u: [f64; 2]) -> (Vector3<f64>, f64) {
        let ((x, v), pdf) = self.distribution.sample(u);
        let sin = (PI * v).sin();
        let d = direction(x, v);
        if sin <= 0. { (d, 0.) } else { (d, pdf / (2. * PI * PI * sin)) }
    }

    /// The density per steradian with which `sample` picks unit direction `d`.
    pub fn pdf(&self, d: &Vector3<f64>) -> f64 {
        let (u, v, sin) = coordinates(d);
        if sin <= 0. { 0. } else { self.distribution.pdf(u, v) / (2. * PI * PI * sin) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Pcg32, point_on_unit_sphere};
    use crate::scenefile::SceneFile;

    #[test]
    fn test_distribution_samples_match_pdf() {
        let f: Vec<f64> = (0..24).map(|i| ((i * 7) % 5) as f64 + if i == 3 { 0. } else { 0.5 }).collect();
        let d = Distribution2D::new(&f, 6);
        let mut rng = Pcg32::new(1, 1);
        let mut counts = vec![0usize; 24];
        let n = 200_000;
        for _ in 0..n {
            let ((u, v), pdf) = d.sample([rng.next_f64(), rng.next_f64()]);
            assert!((pdf - d.pdf(u, v)).abs() < 1e-9, "{} vs {}", pdf, d.pdf(u, v));
            counts[(v * 4.) as usize * 6 + (u * 6.) as usize] += 1;
        }
        // Each cell is sampled in proportion to its value.
        let total: f64 = f.iter().sum();
        for (c, v) in counts.iter().zip(&f) {
            assert!((*c as f64 / n as f64 - v / total).abs() < 0.005, "{} vs {}", *c as f64 / n as f64, v / total);
        }
    }

    /// A Lambertian floor of albedo 0.5 under a sky graded from `bottom` to `top` (no
    /// skysphere: the background), looked at from above.
    fn graded(bottom: f64, top: f64, sample_sky: bool) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": {{ "type": "gradient", "bottom": [{b}, {b}, {b}], "top": [{t}, {t}, {t}] }},
            "max_depth": 1, "shadow_bias": 0.0001, "sample_sky": {},
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, sample_sky, b = bottom, t = top))
    }

    #[test]
    fn test_environment_pdf_integrates_to_one() {
        let s = graded(3., 0.2, true);
        let env = s.environment.as_ref().unwrap();
        let mut rng = Pcg32::new(2, 1);
        let n = 400_000;
        let total = (0..n).map(|_| env.pdf(&point_on_unit_sphere(rng.next_f64(), rng.next_f64())) * 4. * PI).sum::<f64>() / n as f64;
        assert!((total - 1.).abs() < 0.01, "{}", total);
        for _ in 0..1000 {
            let (d, pdf) = env.sample([rng.next_f64(), rng.next_f64()]);
            assert!((pdf - env.pdf(&d)).abs() < 1e-6 * pdf, "{} vs {}", pdf, env.pdf(&d));
        }
    }

    #[test]
    fn test_black_sky_is_not_a_light() {
        assert!(graded(0., 0., true).environment.is_none());
        assert!(graded(1., 1., false).environment.is_none());
    }

    /// Mean and standard deviation of n samples of the floor's radiance.
    fn floor(s: &Scene, n: usize) -> (f64, f64) {
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        let v: Vec<f64> = (0..n).map(|_| crate::trace::trace(&r, 0, s).1.rgb.x).collect();
        let mean = v.iter().sum::<f64>() / n as f64;
        (mean, (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64).sqrt())
    }

    #[test]
    fn test_sky_sampling_is_unbiased_and_less_noisy() {
        // Radiance b + (t - b)(y + 1)/2 over the upper hemisphere gives irradiance
        // ∫ L cos dω = 2π ∫₀¹ (b + (t - b)(y + 1)/2) y dy = π (b + 5 (t - b) / 6), and the floor
        // reflects albedo / π of that. A bright horizon and a dark zenith, which cosine-weighted
        // sampling, favouring the zenith, finds poorly.
        crate::geometry::seed_rand(3, 1);
        let (b, t) = (4., 0.1);
        let expected = 0.5 * (b + 5. * (t - b) / 6.);
        let (sampled, sampled_sd) = floor(&graded(b, t, true), 40_000);
        let (unsampled, unsampled_sd) = floor(&graded(b, t, false), 40_000);
        assert!((sampled - expected).abs() < 0.01 * expected, "sky sampling: {} vs {}", sampled, expected);
        assert!((unsampled - expected).abs() < 0.01 * expected, "without: {} vs {}", unsampled, expected);
        assert!(sampled_sd < 0.92 * unsampled_sd, "sd {} vs {}", sampled_sd, unsampled_sd);
    }
}
