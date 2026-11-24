//! # Patterns for procedural materials
//!
//! Sources of detail beyond Perlin's and Worley's noise (`noise.rs`), for function graphs
//! (`graph.rs`) to colour, bump and roughen surfaces with, and the filtering that lets them be
//! fine without aliasing.
//!
//! ## Sparse convolution noise
//!
//! Gabor noise (Lagae, Lefebvre, Drettakis and Dutré, "Procedural noise using sparse Gabor
//! convolution", SIGGRAPH 2009) sums Gabor kernels, a Gaussian times a sinusoid,
//!
//!   g(x) = exp(-π a² |x|²) cos(2π F ω·x),
//!
//! placed at random points (a Poisson process, λ kernels per unit volume) with random weights
//! (Lewis, "Algorithms for solid noise synthesis", SIGGRAPH 1989). Its spectrum is the
//! kernel's: a Gaussian blob of width a about the frequency F in the direction ω, so unlike
//! Perlin noise it can be made anisotropic (all ω alike: brushed metal, grain, fibres), and
//! its band is known, so it can be filtered exactly. The impulses are found by dividing space
//! into cells as wide as the kernel's reach and seeding each cell's random number generator
//! from its coordinates, so only the 27 cells about a point (9 in plan) are visited. The
//! variance of the sum is λ E[w²] ∫g², by Campbell's theorem, which normalises it.
//!
//! Phasor noise (Tricard, Efremov, Zanni, Neyret, Martínez and Lefebvre, "Procedural phasor
//! noise", SIGGRAPH 2019) sums the same kernels as complex exponentials and keeps only the
//! phase of the sum, φ = arg Σ w exp(-π a² |x|²) exp(i 2π F ω·x). A profile of φ (a sine, a
//! sawtooth, a square) then gives oscillations of constant contrast that Gabor noise, whose
//! amplitude wanders, can't: the regular stripes of wood fibre, fingerprints, sand ripples.
//!
//! Filtering: convolving a kernel with a Gaussian of standard deviation σ multiplies its
//! spectrum by exp(-2π² σ² |f|²), which for a kernel narrow in frequency is close to
//! attenuating it by exp(-2π² σ² F²) (Lagae et al., section 4). The pixel's footprint gives σ.
//!
//! ## Regular patterns, box filtered
//!
//! POV-Ray's patterns (POV-Ray 3.7 reference, "Pattern"): `gradient`, `rings` (cylindrical, as
//! wood), `spherical` (onion), `checker`, `brick`, and its wave types turning any of them into
//! repeating bands. A periodic function aliases into moiré once its period nears a pixel; the
//! remedy in RenderMan's shading language is to return the average of the function over the
//! filter's width, from its integral (Apodaca and Gritz, "Advanced RenderMan", 2000, chapter
//! 11, "filteredpulsetrain"): F(x + w/2) - F(x - w/2) over w, exact for a box filter. The waves
//! here are filtered so, given how many cycles the footprint spans.

use crate::na::Vector3;
use std::f64::consts::PI;

/// A small, fast generator seeded per cell (SplitMix64; Steele, Lea and Flood, "Fast
/// splittable pseudorandom number generators", OOPSLA 2014).
struct CellRng(u64);

impl CellRng {
    fn new(i: i64, j: i64, k: i64, seed: u64) -> CellRng {
        let h = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ (j as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
            ^ (k as u64).wrapping_mul(0x1656_67b1_9e37_79f9)
            ^ seed.wrapping_mul(0x27d4_eb2f_1656_67c5);
        let mut r = CellRng(h);
        r.next();
        r
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    fn uniform(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Poisson distributed, by Knuth's product of uniforms (means here are a few).
    fn poisson(&mut self, mean: f64) -> u32 {
        let limit = (-mean).exp();
        let (mut k, mut p) = (0, self.uniform());
        while p > limit && k < 64 {
            k += 1;
            p *= self.uniform();
        }
        k
    }
}

/// Where the Gaussian has fallen to 5%: the kernel's reach, a multiple of 1 / a.
const REACH: f64 = 0.977_205;

/// What a phasor's phase is turned into.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Profile {
    /// Gabor noise itself: the real part of the sum, normalised, in [-1, 1].
    Gabor,
    /// sin φ, in [-1, 1].
    Sine,
    /// φ / π, a sawtooth in [-1, 1].
    Saw,
    /// ±1 by the sign of sin φ, softened over the footprint.
    Square,
}

/// Sparse Gabor or phasor noise (see the module documentation).
#[derive(Clone, Debug)]
pub struct Gabor {
    pub seed: u64,
    /// Cycles per unit length, F.
    pub frequency: f64,
    /// The kernel's Gaussian width a, per unit length: the band's width in frequency. Smaller
    /// is a narrower band, so more regular, and a wider kernel, so slower to evaluate.
    pub bandwidth: f64,
    /// The common direction of the kernels' oscillation, a unit vector; None for every
    /// direction (isotropic).
    pub direction: Option<Vector3<f64>>,
    /// How far each kernel's direction may stray from `direction`: 0 none, 1 any.
    pub spread: f64,
    /// Kernels overlapping a point, on average.
    pub impulses: f64,
    /// Only in x and z, as on the ground.
    pub plan: bool,
    pub profile: Profile,
}

impl Gabor {
    fn radius(&self) -> f64 {
        REACH / self.bandwidth
    }

    /// Kernels per unit volume (or area, in plan).
    fn density(&self) -> f64 {
        let r = self.radius();
        if self.plan { self.impulses / (PI * r * r) } else { self.impulses / (4. / 3. * PI * r * r * r) }
    }

    /// The standard deviation of the real part of the sum (Campbell's theorem: λ E[w²] ∫g²,
    /// with weights uniform in [-1, 1] and ∫ exp(-2π a² |x|²) cos² ≈ half the Gaussian's
    /// integral, (2a²)^(-d/2)).
    fn deviation(&self) -> f64 {
        let a2 = 2. * self.bandwidth * self.bandwidth;
        let gaussian = if self.plan { 1. / a2 } else { a2.powf(-1.5) };
        (self.density() / 3. * 0.5 * gaussian).sqrt()
    }

    /// A kernel's direction, drawn about `direction` (or anywhere).
    fn orientation(&self, rng: &mut CellRng) -> Vector3<f64> {
        let random = |rng: &mut CellRng| -> Vector3<f64> {
            if self.plan {
                let t = 2. * PI * rng.uniform();
                Vector3::new(t.cos(), 0., t.sin())
            } else {
                let z = 2. * rng.uniform() - 1.;
                let t = 2. * PI * rng.uniform();
                let s = (1. - z * z).max(0.).sqrt();
                Vector3::new(s * t.cos(), s * t.sin(), z)
            }
        };
        match self.direction {
            None => random(rng),
            Some(d) => {
                let r = random(rng);
                if self.spread <= 0. {
                    d
                } else {
                    // Lean towards a random direction by up to `spread` of a right angle,
                    // keeping to the same half (a direction and its opposite are one stripe).
                    let r = if r.dot(&d) < 0. { -r } else { r };
                    let lean = self.spread.min(1.) * rng.uniform();
                    (d * (1. - lean) + r * lean).try_normalize(1e-12).unwrap_or(d)
                }
            },
        }
    }

    /// The complex sum at `p`, its contrast attenuated for a footprint `filter` wide; and the
    /// attenuation.
    fn sum(&self, p: Vector3<f64>, filter: f64) -> (f64, f64, f64) {
        // A box `filter` wide is about a Gaussian of half that deviation.
        let sigma = 0.5 * filter;
        let attenuation = (-2. * PI * PI * sigma * sigma * self.frequency * self.frequency).exp();
        if attenuation < 1e-3 {
            return (0., 0., 0.);
        }
        let r = self.radius();
        let per_cell = self.density() * if self.plan { r * r } else { r * r * r };
        let q = p / r;
        let (ci, cj, ck) = (q.x.floor() as i64, if self.plan { 0 } else { q.y.floor() as i64 }, q.z.floor() as i64);
        let js: &[i64] = if self.plan { &[0] } else { &[-1, 0, 1] };
        let (a2, f) = (self.bandwidth * self.bandwidth, 2. * PI * self.frequency);
        let (mut re, mut im) = (0., 0.);
        for di in -1 ..= 1 {
            for &dj in js {
                for dk in -1 ..= 1 {
                    let (i, j, k) = (ci + di, cj + dj, ck + dk);
                    let mut rng = CellRng::new(i, j, k, self.seed);
                    for _ in 0 .. rng.poisson(per_cell) {
                        let at = Vector3::new(
                            (i as f64 + rng.uniform()) * r,
                            if self.plan { p.y } else { (j as f64 + rng.uniform()) * r },
                            (k as f64 + rng.uniform()) * r);
                        let w = 2. * rng.uniform() - 1.;
                        let omega = self.orientation(&mut rng);
                        let x = p - at;
                        let d2 = x.norm_squared();
                        if d2 >= r * r {
                            continue;
                        }
                        let g = w * (-PI * a2 * d2).exp();
                        let (s, c) = (f * omega.dot(&x)).sin_cos();
                        re += g * c;
                        im += g * s;
                    }
                }
            }
        }
        (re, im, attenuation)
    }

    /// The noise at `p`, in [-1, 1], filtered over a footprint `filter` wide (0: none).
    pub fn eval(&self, p: Vector3<f64>, filter: f64) -> f64 {
        let (re, im, attenuation) = self.sum(p, filter);
        if attenuation == 0. {
            return 0.;
        }
        let v = match self.profile {
            Profile::Gabor => re / (3. * self.deviation()),
            Profile::Sine | Profile::Saw | Profile::Square => {
                if re == 0. && im == 0. {
                    return 0.;
                }
                let phi = im.atan2(re);
                match self.profile {
                    Profile::Sine => phi.sin(),
                    Profile::Saw => phi / PI,
                    _ => {
                        // Softened where the footprint spans much of a cycle.
                        let soft = (filter * self.frequency * 4.).max(0.05);
                        (phi.sin() / soft).clamp(-1., 1.)
                    },
                }
            },
        };
        (v * attenuation).clamp(-1., 1.)
    }

    /// The wavelength, the finest detail.
    pub fn finest(&self) -> f64 {
        1. / self.frequency
    }
}

/// The shapes a `wave` gives a pattern, after POV-Ray's wave types, each with period 1 and
/// values in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Wave {
    /// Rising from 0 to 1, then back to 0 at once: a sawtooth.
    Ramp,
    /// Up and down linearly.
    Triangle,
    /// 0.5 + 0.5 sin 2πt.
    Sine,
    /// |sin πt|: rounded humps with sharp troughs.
    Scallop,
    /// 1 for the first half of each cycle, 0 for the second.
    Square,
}

impl Wave {
    pub fn parse(s: &str) -> Result<Wave, String> {
        Ok(match s {
            "ramp" => Wave::Ramp,
            "triangle" => Wave::Triangle,
            "sine" => Wave::Sine,
            "scallop" => Wave::Scallop,
            "square" => Wave::Square,
            _ => return Err(format!("unknown wave \"{}\": ramp, triangle, sine, scallop or square", s)),
        })
    }

    pub fn at(self, t: f64) -> f64 {
        let u = t - t.floor();
        match self {
            Wave::Ramp => u,
            Wave::Triangle => 1. - (2. * u - 1.).abs(),
            Wave::Sine => 0.5 + 0.5 * (2. * PI * t).sin(),
            Wave::Scallop => (PI * u).sin(),
            Wave::Square => if u < 0.5 { 1. } else { 0. },
        }
    }

    /// The integral of the wave from 0 to t.
    fn integral(self, t: f64) -> f64 {
        let (n, u) = (t.floor(), t - t.floor());
        match self {
            Wave::Ramp => 0.5 * n + 0.5 * u * u,
            Wave::Triangle => 0.5 * n + if u < 0.5 { u * u } else { 2. * u - u * u - 0.5 },
            Wave::Sine => 0.5 * t + (1. - (2. * PI * t).cos()) / (4. * PI),
            Wave::Scallop => (2. / PI) * n + (1. - (PI * u).cos()) / PI,
            Wave::Square => 0.5 * n + u.min(0.5),
        }
    }

    /// The mean over a cycle.
    pub fn mean(self) -> f64 {
        match self {
            Wave::Scallop => 2. / PI,
            _ => 0.5,
        }
    }

    /// The wave's average over [t - w/2, t + w/2]: box filtered (`w` in cycles).
    pub fn filtered(self, t: f64, w: f64) -> f64 {
        if w < 1e-4 {
            return self.at(t);
        }
        if w > 64. {
            return self.mean();
        }
        // Integrals taken from the start of the cycle, so they stay small.
        let base = (t - 0.5 * w).floor();
        let (a, b) = (t - 0.5 * w - base, t + 0.5 * w - base);
        ((self.integral(b) - self.integral(a)) / w).clamp(0., 1.)
    }
}

/// A square wave of ±1, period 2 (+1 on [0, 1)), box filtered over `w`.
fn filtered_square(x: f64, w: f64) -> f64 {
    // Its integral: a triangle wave, rising on [0, 1), falling on [1, 2).
    let integral = |x: f64| {
        let u = x.rem_euclid(2.);
        if u < 1. { u } else { 2. - u }
    };
    if w < 1e-9 {
        return if x.rem_euclid(2.) < 1. { 1. } else { -1. };
    }
    ((integral(x + 0.5 * w) - integral(x - 0.5 * w)) / w).clamp(-1., 1.)
}

/// POV-Ray's checker in 3D: 0 and 1 in alternate cubes `size` across, box filtered over a
/// footprint `filter` wide (separable: the product of filtered square waves along each axis).
pub fn checker(p: Vector3<f64>, size: f64, filter: f64, plan: bool) -> f64 {
    let w = filter / size;
    let q = p / size;
    let product = filtered_square(q.x, w) * filtered_square(q.z, w) * if plan { 1. } else { filtered_square(q.y, w) };
    0.5 - 0.5 * product
}

/// What `brick` gives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrickStyle {
    /// 1 in the mortar, 0 on the bricks.
    Mortar,
    /// A random value in [0, 1) for each brick (and the mortar the brick it borders), for
    /// varying the bricks' colours.
    Id,
}

/// Bricks `size` (x, y, z) laid in courses along y, each course moved half a brick along x and
/// z from the last (a running bond on any face), with `mortar` between them: POV-Ray's brick
/// pattern. The mortar's edges are smoothed over the footprint `filter`.
pub fn brick(p: Vector3<f64>, size: Vector3<f64>, mortar: f64, filter: f64, style: BrickStyle, seed: u64) -> f64 {
    let course = (p.y / size.y).floor();
    let shift = if (course as i64).rem_euclid(2) == 1 { 0.5 } else { 0. };
    let (u, w) = (p.x / size.x + shift, p.z / size.z + shift);
    match style {
        BrickStyle::Id => {
            let mut rng = CellRng::new(u.floor() as i64, course as i64, w.floor() as i64, seed);
            rng.uniform()
        },
        BrickStyle::Mortar => {
            // Distance to the nearest joint along each axis, in scene units.
            let joint = |t: f64, s: f64| (t - t.round()).abs() * s;
            let d = joint(u, size.x).min(joint(p.y / size.y, size.y)).min(joint(w, size.z));
            let half = 0.5 * mortar;
            let soft = 0.5 * filter;
            if soft <= 0. {
                if d < half { 1. } else { 0. }
            } else {
                1. - smoothstep(half - soft, half + soft, d)
            }
        },
    }
}

fn smoothstep(lo: f64, hi: f64, x: f64) -> f64 {
    let t = ((x - lo) / (hi - lo)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gabor(direction: Option<Vector3<f64>>, profile: Profile) -> Gabor {
        Gabor { seed: 3, frequency: 4., bandwidth: 1.5, direction, spread: 0., impulses: 16., plan: false, profile }
    }

    fn samples(n: usize) -> impl Iterator<Item = Vector3<f64>> {
        let mut rng = CellRng::new(1, 2, 3, 4);
        (0 .. n).map(move |_| Vector3::new(rng.uniform(), rng.uniform(), rng.uniform()) * 10. - Vector3::repeat(5.))
    }

    #[test]
    fn test_gabor_is_normalised_and_centred() {
        let g = gabor(None, Profile::Gabor);
        let v: Vec<f64> = samples(4000).map(|p| g.eval(p, 0.)).collect();
        let mean = v.iter().sum::<f64>() / v.len() as f64;
        let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
        assert!(mean.abs() < 0.03, "mean {}", mean);
        // Normalised to a third of its deviation, give or take the truncated kernel.
        assert!((0.2 .. 0.45).contains(&sd), "deviation {}", sd);
    }

    #[test]
    fn test_anisotropic_gabor_varies_along_its_direction() {
        // Along x it oscillates; across it (y) it changes only slowly.
        let g = gabor(Some(Vector3::x()), Profile::Gabor);
        let (mut along, mut across) = (0., 0.);
        for p in samples(2000) {
            let h = 0.02;
            along += (g.eval(p + Vector3::x() * h, 0.) - g.eval(p, 0.)).abs();
            across += (g.eval(p + Vector3::y() * h, 0.) - g.eval(p, 0.)).abs();
        }
        assert!(along > 3. * across, "{} vs {}", along, across);
    }

    #[test]
    fn test_phasor_has_constant_contrast() {
        // Most of a phasor sine's values are near its extremes, as a sine's are; Gabor noise's
        // gather about 0.
        let ph = gabor(Some(Vector3::x()), Profile::Sine);
        let gb = gabor(Some(Vector3::x()), Profile::Gabor);
        let strong = |g: &Gabor| samples(2000).filter(|p| g.eval(*p, 0.).abs() > 0.7).count();
        assert!(strong(&ph) > 800, "{}", strong(&ph));
        assert!(strong(&gb) < strong(&ph) / 2);
    }

    #[test]
    fn test_noise_fades_when_finer_than_the_footprint() {
        let g = gabor(None, Profile::Gabor);
        let p = Vector3::new(0.3, 0.2, 0.1);
        assert!((g.eval(p, 0.) - g.eval(p, 1e-9)).abs() < 1e-9);
        assert_eq!(g.eval(p, 2.), 0.);
    }

    #[test]
    fn test_filtered_waves_are_their_averages() {
        for wave in [Wave::Ramp, Wave::Triangle, Wave::Sine, Wave::Scallop, Wave::Square] {
            for k in 0 .. 50 {
                let (t, w) = (k as f64 * 0.173 - 3., 0.05 + k as f64 * 0.031);
                let n = 4000;
                let mean = (0 .. n).map(|i| wave.at(t - 0.5 * w + w * (i as f64 + 0.5) / n as f64)).sum::<f64>() / n as f64;
                assert!((wave.filtered(t, w) - mean).abs() < 2e-3, "{:?} at {} over {}: {} vs {}", wave, t, w, wave.filtered(t, w), mean);
            }
            // A wide footprint gives the mean.
            assert!((wave.filtered(0.37, 40.) - wave.mean()).abs() < 0.02);
        }
    }

    #[test]
    fn test_checker_alternates_and_filters_to_grey() {
        assert_eq!(checker(Vector3::new(0.5, 0.5, 0.5), 1., 0., false), 0.);
        assert_eq!(checker(Vector3::new(1.5, 0.5, 0.5), 1., 0., false), 1.);
        assert_eq!(checker(Vector3::new(1.5, 1.5, 0.5), 1., 0., false), 0.);
        assert!((checker(Vector3::new(0.3, 0.7, 0.2), 1., 20., false) - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_bricks_have_mortar_between_them() {
        let size = Vector3::new(0.2, 0.06, 0.1);
        let m = |x: f64, y: f64| brick(Vector3::new(x, y, 0.03), size, 0.01, 0., BrickStyle::Mortar, 0);
        assert_eq!(m(0.1, 0.03), 0.);
        assert_eq!(m(0.2, 0.03), 1.);
        assert_eq!(m(0.1, 0.06), 1.);
        // The next course is moved half a brick: its joint is at 0.1.
        assert_eq!(m(0.1, 0.09), 1.);
        assert_eq!(m(0.2, 0.09), 0.);
        let id = |x: f64, y: f64| brick(Vector3::new(x, y, 0.05), size, 0.01, 0., BrickStyle::Id, 0);
        assert_eq!(id(0.05, 0.03), id(0.15, 0.02));
        assert_ne!(id(0.05, 0.03), id(0.25, 0.03));
    }
}
