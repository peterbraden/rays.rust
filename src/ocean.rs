//! # Ocean waves
//!
//! An implementation of Jerry Tessendorf, "Simulating Ocean Water", SIGGRAPH course notes,
//! 1999-2004, section 4 ("Practical Ocean Wave Algorithms"):
//! <https://jtessen.people.clemson.edu/reports/papers_files/coursenotes2004.pdf>. Equation numbers
//! below refer to the 2004 notes. For a gentler walk through the same ideas, see "Simulating
//! Ocean Waves", <https://peterbraden.co.uk/article/simulation-ocean-waves/>.
//!
//! The sea is modelled statistically: the height of the surface is a sum of a great many sine
//! waves, with random amplitudes whose statistics follow oceanographic measurements. The sum is
//! evaluated with an inverse FFT over a square patch, triangulated, and tiled to infinity with a
//! `RepeatingMesh`.
//!
//! ## The wavevector (4.1)
//!
//! "the vector k, called the wavevector, is a horizontal vector that points in the direction of
//! travel of the wave, and has magnitude k related to the length of the wave (λ) by
//! k = 2π/λ" (eq. 28).
//!
//! Each point on the 2D Fourier grid is one wavevector, ie. one wave: its direction of travel and
//! its wavelength. The amplitude stored at that point is how much of that wave is present.
//!
//! ## The height field (4.3)
//!
//! ```text
//! h(x, t) = Σ_k h̃(k, t) exp(i k·x)                                        (eq. 36)
//! k = (2πn/Lx, 2πm/Lz),  -N/2 <= n < N/2,  -M/2 <= m < M/2
//! ```
//!
//! In the code N = M = `fourier_size`, and Lx = Lz is the patch size in metres (the scene key is,
//! confusingly, `resolution`). `gen_k` builds k from n and m. The surface is periodic over the
//! patch, which is what lets it tile seamlessly; the notes warn that the repetition can show
//! across a wide field of view.
//!
//! The grid stores n and m from -N/2, but the FFT expects frequency 0 at index 0. By the FFT's
//! shift theorem, offsetting every frequency index by N/2 multiplies the output by
//! exp(iπ(x + z)) = (-1)^(x+z), so each vertex's height is corrected by that sign. (The code
//! uses -(-1)^(x+z), which also flips the whole surface; statistically that makes no difference.)
//!
//! ## The Phillips spectrum (4.3)
//!
//! Measured ocean wave amplitudes are close to independent Gaussian variables with a spatial
//! spectrum P_h(k). For wind-driven waves in a fully developed sea:
//!
//! ```text
//! P_h(k) = A exp(-1/(kL)²) / k⁴ |k̂·ŵ|²,   L = V²/g                          (eq. 40)
//! ```
//!
//! L is the largest wave a continuous wind of speed V can raise, and ŵ the wind direction. "A is a
//! numeric constant": the scene's `amplitude`. Heights scale with √A; the notes give it no units.
//!
//! The factor |k̂·ŵ|² "eliminates waves that move perpendicular to the wind direction". Being
//! squared, it does not suppress waves travelling against the wind; they are as strong as those
//! travelling with it.
//!
//! Not implemented: the notes multiply the spectrum by exp(-k²l²), with l a small length, to
//! suppress very small waves, where the Phillips spectrum converges poorly (eq. 41).
//!
//! ## A random realisation (4.4)
//!
//! ```text
//! h̃0(k) = 1/√2 (ξr + i ξi) √P_h(k)                                           (eq. 42)
//! h̃(k, t) = h̃0(k) exp(iω(k)t) + h̃0*(-k) exp(-iω(k)t)                          (eq. 43)
//! ```
//!
//! ξr and ξi are independent standard Gaussian draws (`randn`). Eq. 43 propagates each wave both
//! ways and "preserves the complex conjugation property h̃*(k, t) = h̃(-k, t)", which is exactly
//! the condition for the inverse transform, the height, to be real. Any time t can be computed
//! directly, without stepping through earlier times.
//!
//! ## Dispersion (4.2)
//!
//! In deep water a wave's frequency depends only on its wavenumber: ω²(k) = gk (eq. 31). To make
//! an animation loop after a time T, every frequency must be a multiple of ω0 = 2π/T (eq. 34),
//! so the notes quantise the dispersion relation: ω̄(k) = [[ω(k)/ω0]] ω0, taking the integer
//! part (eq. 35). See `dispersion`.
//!
//! ## Slopes and choppy waves (4.3, 4.6)
//!
//! The exact slope is another FFT: ε(x, t) = Σ_k i k h̃(k, t) exp(i k·x) (eq. 37). Choppy waves
//! displace each point horizontally to x + λD(x, t), with D(x, t) = Σ_k -i (k/k) h̃(k, t)
//! exp(i k·x) (eq. 44), sharpening crests and broadening troughs. Both are computed in
//! `create_tile`. See "Choppy crests and breaking" below for how the sea uses them.
//!
//! ## Scene options
//!
//! `{"type": "ocean", "wind": [7, 2], "planet_radius": 6360000, "detail_center": [x, z]}`: the
//! sea (`Ocean`), lit physically (`PhysicalOcean`). By default it is the planet's surface, a
//! sphere of the Earth's radius; `"planet_radius": 0` makes it the flat plane y = 0. The radius
//! changes only its shape. The waves are for a wind of `wind` m/s ([x, z]), scaled to the
//! significant wave height that wind raises (`significant_wave_height`: Pierson and
//! Moskowitz's fully developed sea, or with `fetch`, m, the younger sea of a limited stretch of
//! open water upwind), or to `wave_height` (m) if given; or, the older calibration, so their
//! slopes carry `resolved` of Cox and Munk's; or by `amplitude` (Phillips's constant). Crests
//! are sharpened by `choppiness` (default 1; 0 for none). At `time` s with `seed`; displaced within
//! `detail_distance` (default 300 m) of `detail_center` (put it under the camera). `water` is
//! the colour the water body sends back, as an albedo, and `roughness` the mean square slope of
//! the ripples (by default the rest of Cox and Munk's). `"debug": true` shades by normal.
//!
//! ## Choppy crests and breaking
//!
//! Linear waves are sinusoids, as round on top as below. Real wind waves have sharp crests and
//! broad, flat troughs: their water moves in orbits, forwards at the crest and back in the
//! trough, so it crowds together at the crests. The choppy displacement is that motion to first
//! order (the Lagrangian or Gerstner surface, for λ = 1): the point of the sea that was at p
//! is at p + λD(p). The height field at a place x is then the height at the p with
//! p + λD(p) = x, found by fixed-point iteration (`WaveField::source`), and the slopes there
//! are the grid's through the inverse of the map's Jacobian, steeper where it squeezes the
//! surface together. Where the Jacobian J = det(I + λ∇D) reaches zero the surface folds over:
//! the crest is too steep to stand and breaks. (Tessendorf's notes, section 4.6, use the same
//! sign of J to find "where the waves are about to break".)
//!
//! ## Whitecaps
//!
//! Waves break once the wind is more than a breeze, and their foam covers a fraction of the sea
//! that grows steeply with the wind: W = 3.84·10⁻⁶ U^3.41 at a wind of U m/s (E. C. Monahan and
//! I. O'Muircheartaigh, "Optimal power-law description of oceanic whitecap coverage dependence on
//! wind speed", J. Phys. Oceanogr. 10, 1980): a few thousandths in a moderate breeze (7 m/s),
//! 2% at 12 m/s, 5% at 14 m/s, a fifth of the sea in a gale. A crest breaks where the choppy
//! surface is squeezed towards folding, its Jacobian below a threshold (`Foam`): there the foam
//! is fresh and dense. Foam stays with the water it formed in, which the crest moves on from,
//! and fades over a few seconds (whitecaps thin from dense stage A foam to a residual patch,
//! stage B, decaying with an e-folding time of a few seconds: Monahan and Lu 1990, Callaghan et
//! al. 2012). So the foam at a point is the strongest breaking it saw over the last six
//! seconds, faded by how long ago: each breaking crest trails a patch that thins into lace
//! (noise ranked so a patch of strength r covers a fraction r) and, as the wind rises towards a
//! gale, is drawn out in streaks along it. The Jacobians are found on each displaced tile's
//! grid at a dozen past times, and the threshold is set so foam covers W of the sea. Fresh foam
//! is a dense froth scattering diffusely (`foam_albedo`, default 0.6); residual foam is
//! thinner, at 70% of that; and between the patches, the bubble clouds the breaking drove
//! down linger, paling the water and turning it greener. The coverage can be given as
//! `whitecaps` (a fraction, 0 for none) in place of Monahan's.
//!
//! `"transparent": true` lets you see into the water: what lies beneath the surface (an
//! iceberg's foot, the underside of a floe, a reef) shows through it, dimmed by the water's
//! `absorption` (per metre, per channel; by default `[0.38, 0.065, 0.045]`, clear coastal sea:
//! pure water's absorption, about 0.35 m⁻¹ in the red and a few hundredths in the green and
//! blue, R. M. Pope and E. S. Fry, "Absorption spectrum (380-700 nm) of pure water. II",
//! Applied Optics 36(33), 1997, doi:10.1364/AO.36.008710, with a little more in the blue from
//! dissolved organic matter, as in Jerlov's coastal water types). So white ice a few metres down
//! glows turquoise, and deeper it fades into the sea's own colour. A ray refracted into the
//! water is traced to whatever it meets within the depth at which the light would be all but
//! gone, and the water in front of it adds its own colour as in the simplest model of shallow
//! water in remote sensing: the deep water's reflectance weighted by 1 - exp(-a ℓ), the
//! bottom's by exp(-a ℓ) (eg. Z. Lee et al., "Hyperspectral remote sensing for shallow waters",
//! Applied Optics 37(27), 1998, doi:10.1364/AO.37.006329). Sunlight reaches what is under water
//! through the surface (Fresnel's transmission, and the water's absorption along the refracted
//! path down; the shadow ray itself isn't bent), and light leaving it comes back out by
//! refraction.
//!
//! `"transparent": true` lets you see into the water: what lies beneath the surface (an
//! iceberg's foot, the underside of a floe, a reef) shows through it, dimmed by the water's
//! `absorption` (per metre, per channel; by default `[0.38, 0.065, 0.045]`, clear coastal sea
//! water: pure water's absorption, about 0.35 m⁻¹ in the red and a few hundredths in the green
//! and blue, R. M. Pope and E. S. Fry, "Absorption spectrum (380-700 nm) of pure water. II",
//! Applied Optics 36(33), 1997, doi:10.1364/AO.36.008710, with a little more in the blue from
//! dissolved organic matter, as in Jerlov's coastal water types). So white ice a few metres down
//! glows turquoise, and deeper it fades to the sea's own colour. A ray refracted into the water
//! is traced to whatever it meets within the depth at which the light would be all but gone;
//! the water between adds its own colour by the remote-sensing model of shallow water, the
//! deep water's reflectance R∞ weighted by 1 - exp(-a ℓ), the bottom's by exp(-a ℓ) (eg. Z. Lee
//! et al., "Hyperspectral remote sensing for shallow waters", Applied Optics 37(27), 1998,
//! doi:10.1364/AO.37.006329, in its simplest form). Light from the sun reaches what is under
//! water through the surface (Fresnel's transmission, and the water's absorption along the
//! refracted path down), and light leaving it comes back out by refraction; the sun's direction
//! isn't bent for the shadow ray, only its path length in the water is.

extern crate image;
extern crate rustfft;

use crate::material::model::LightSampling;
use crate::scene::Scene;
use crate::intersection::Intersection;
use std::f64;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use serde_json::{Value, Map};
use rustfft::num_complex::Complex;
use crate::na::{Vector3, Vector2};
use rand_distr::{Distribution, StandardNormal};
use std::path::Path;
use crate::color::Color;
use rustfft::algorithm::Radix4;
use rustfft::{Fft, FftDirection};
use crate::shapes::bbox::BBox;
use crate::shapes::triangle::Triangle;
use crate::shapes::repeating_mesh::RepeatingMesh;
use std::vec::Vec;
use std::sync::Arc;
use crate::shapes::mesh::Mesh;
use crate::shapes::geometry::Geometry;
use crate::sceneobject::SceneObject;
use crate::material::legacy::Whitted;
use crate::material::normal::NormalShade;
use crate::material::texture::{Solid, Medium};
use crate::scenefile::SceneFile;
use crate::material::functions::{scatter_dielectric, refract, reflect, schlick};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::geometry;
use rand::{SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::*;

/// A standard Gaussian draw: ξr or ξi in eq. 42.
fn randn(rng: &mut StdRng) -> f64 {
    StandardNormal.sample(rng)
}

/// The Phillips spectrum P_h(k) (eq. 40). `scale` is A; `wind` is the wind velocity, giving both
/// V and ŵ.
fn phillips(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> f64 {
    let ksq = k.x * k.x + k.y * k.y;
    if ksq < f64::MIN_POSITIVE { return 0. };
    // No wind, no waves (and wind.normalize() would be NaN).
    if wind.norm() < f64::MIN_POSITIVE { return 0. };
    let wind_dir = wind.normalize();
    let wk = k.normalize().dot(&wind_dir);
    let wind_speed = wind.norm();
    // The largest wave the wind can raise.
    let l = (wind_speed * wind_speed) / gravity;
    scale * (-1.0 / (ksq * l * l)).exp() / (ksq * ksq) * wk * wk
}

/// A random Fourier amplitude h̃0(k) (eq. 42).
fn amplitude(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64, rng: &mut StdRng) -> Complex<f64> {
    1f64/(2f64.sqrt()) *
        Complex::new(randn(rng), randn(rng)) *
        phillips(k, wind, scale, gravity).sqrt()
}

/// Repeat time T of the animation, in seconds (eq. 34).
const REPEAT_TIME: f64 = 200.;

/// The frequency ω of a wave: the deep-water dispersion relation ω² = gk (eq. 31), quantised to
/// multiples of ω0 = 2π/T so that the surface repeats after T (eqs. 34, 35). A longer T keeps
/// ω̄ closer to ω.
fn dispersion(k: Vector2<f64>, gravity: f64) -> f64 {
    let w = (k.norm() * gravity).sqrt();
    let w_0 = 2f64 * std::f64::consts::PI / REPEAT_TIME;
    (w / w_0).floor() * w_0
}

/// h̃0(k) for every k on the N x N grid, row-major with rows along z.
fn create_amplitude_tile(
        wind: Vector2<f64>,
        scale: f64,
        gravity: f64,
        lx: f64,
        lz: f64,
        fourier_grid_size: usize,
        rng: &mut StdRng,
        )-> Vec<Complex<f64>> {
    let mut h0 = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];

    for j in 0 .. fourier_grid_size {
        for i in 0 .. fourier_grid_size {
            let ind = j * fourier_grid_size + i;

            // Column i is n, along x; row j is m, along z. Both run from -N/2.
            let n = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let m = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let k = gen_k(n, m, lx, lz);
            h0[ind] = amplitude(k, wind, scale, gravity, rng);
        }
    }
    h0
}

/// The wavevector k = (2πn/Lx, 2πm/Lz) for grid indices n, m (below eq. 36). The vector's y
/// component is the z axis.
fn gen_k(n: f64, m: f64, lx: f64, lz: f64) -> Vector2<f64> {
    Vector2::new(
        2f64 * std::f64::consts::PI * n /  lx,
        2f64 * std::f64::consts::PI * m /  lz,
    )
}

fn mn_to_i(m: i32, n: i32, size: i32) -> usize {
    ((n + size/2) * size + (m + size/2)) as usize
}

fn transpose(matr: &[Complex<f64>], size: usize) -> Vec<Complex<f64>> {
    let mut out = matr.to_vec();
    for x in 0..size {
        for y in 0..size {
            out[x*size + y] = matr[y*size + x];
        }
    }
    out
}

/// 2D Fast Fourier Transform (Used to test IFFT2)
fn fft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let fft = Radix4::new(size, FftDirection::Forward);
    let mut rows = tile;
    fft.process(&mut rows);
    let mut cols = transpose(&rows, size);
    fft.process(&mut cols);
    transpose(&cols, size)
}

/// Inverse 2D Fast Fourier Transform
pub(crate) fn ifft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let ifft = Radix4::new(size, FftDirection::Inverse);
    let mut rows = tile;
    ifft.process(&mut rows);
    let mut cols = transpose(&rows, size);
    ifft.process(&mut cols);
    // Normalised by 1/size per axis.
    let scale = 1. / (size * size) as f64;
    transpose(&cols, size).iter().map(|x| x.scale(scale)).collect()
}

/// A field over the N x N Fourier grid.
type Field = Vec<Complex<f64>>;

/// A tile of the surface at time t, on the N x N grid.
pub struct OceanTile {
    /// Heights h(x, t) (eqs. 43, 36).
    pub heights: Field,
    /// Choppy displacements D(x, t) in x and z (eq. 44), if requested.
    pub displacement: Option<(Field, Field)>,
    /// Slopes ∇h(x, t), in x and z (eq. 37), if requested.
    pub slopes: Option<(Field, Field)>,
}

/// The surface at time t, from the amplitudes h̃0. Each field is an inverse FFT, the dominant
/// cost of building the ocean, so the choppy displacements are only computed when `choppy`, and
/// the slopes (eq. 37) when `slopes` (the flat ocean's normals come from its triangles).
pub fn create_tile(
        fourier_grid_size:usize,
        lx: f64,
        lz: f64,
        gravity: f64,
        time: f64,
        choppy: bool,
        slopes: bool,
        h0: Vec<Complex<f64>>) 
        -> OceanTile {

    // Tile of amplitudes
    let mut ht =          vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dx = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dz = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_x =  vec![Complex::new(0., 0.); if slopes { fourier_grid_size * fourier_grid_size } else { 0 }];
    let mut ht_slope_z =  vec![Complex::new(0., 0.); if slopes { fourier_grid_size * fourier_grid_size } else { 0 }];

    for j in 0 .. fourier_grid_size {
        for i in 0 .. fourier_grid_size {
            let ind = j * fourier_grid_size + i;
            // Column i is n, along x; row j is m, along z (see create_amplitude_tile).
            let n = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let m = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let k = gen_k(n, m, lx, lz);
            let wt = dispersion(k, gravity) * time;
            let c0 = Complex::new(wt.cos(), wt.sin());
            let c1 = Complex::new(wt.cos(), -wt.sin());

            // h̃(k, t) = h̃0(k) e^{iωt} + h̃0*(-k) e^{-iωt} (eq. 43). The index of -k wraps n and
            // m round, so -(-N/2) maps back to -N/2.
            let neg_k = ((fourier_grid_size - j) % fourier_grid_size) * fourier_grid_size
                + (fourier_grid_size - i) % fourier_grid_size;
            ht[ind] =
                h0[ind] * c0 +
                h0[neg_k].conj() * c1;

            // i k h̃ (eq. 37)
            if slopes {
                ht_slope_x[ind] = ht[ind] * Complex::new(0., k.x);
                ht_slope_z[ind] = ht[ind] * Complex::new(0., k.y);
            }

            // -i (k/k) h̃ (eq. 44)
            let len = k.norm();
            if choppy && len > 0.00001 {
                ht_slope_dx[ind] = ht[ind] * Complex::new(0., -k.x / len); 
                ht_slope_dz[ind] = ht[ind] * Complex::new(0., -k.y / len); 
            } 
        }
    }

    // Eq. 36 is a plain sum over the waves; ifft2 normalises by 1/N², which would make wave
    // height depend on the grid size.
    let sum = |t: Vec<Complex<f64>>| -> Vec<Complex<f64>> {
        let n2 = (fourier_grid_size * fourier_grid_size) as f64;
        ifft2(t, fourier_grid_size).into_iter().map(|x| x * n2).collect()
    };
    OceanTile {
        heights: sum(ht),
        displacement: if choppy { Some((sum(ht_slope_dx), sum(ht_slope_dz))) } else { None },
        slopes: if slopes { Some((sum(ht_slope_x), sum(ht_slope_z))) } else { None },
    }
}


#[cfg(test)]
mod tests {
	use super::{fft2, ifft2, transpose, create_amplitude_tile, create_tile, phillips};
	use rustfft::num_complex::Complex;
	use crate::na::Vector2;
	use rand::{SeedableRng, rngs::StdRng};

	const N: usize = 32;
	const L: f64 = 100.;

	fn heights(wind: Vector2<f64>, time: f64) -> Vec<Complex<f64>> {
		heights_n(wind, time, N)
	}

	fn heights_n(wind: Vector2<f64>, time: f64, n: usize) -> Vec<Complex<f64>> {
		let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
		let h0 = create_amplitude_tile(wind, 1., 9.81, L, L, n, &mut rng);
		create_tile(n, L, L, 9.81, time, false, false, h0).heights
	}

	#[test]
	fn test_wave_height_independent_of_grid_size() {
		let rms = |n: usize| {
			let h = heights_n(Vector2::new(20., 5.), 0., n);
			(h.iter().map(|c| c.re * c.re).sum::<f64>() / h.len() as f64).sqrt()
		};
		let (small, large) = (rms(32), rms(256));
		let ratio = large / small;
		assert!(ratio > 0.5 && ratio < 2., "rms {} at N=32, {} at N=256", small, large);
	}

	#[test]
	fn test_height_field_is_real() {
		// h(k) + conj(h(-k)) is Hermitian, so its inverse transform must be real.
		for t in [0., 1.5] {
			let h = heights(Vector2::new(20., 5.), t);
			let max_re = h.iter().map(|c| c.re.abs()).fold(0., f64::max);
			let max_im = h.iter().map(|c| c.im.abs()).fold(0., f64::max);
			assert!(max_re > 0.);
			assert!(max_im < max_re * 1e-9, "im {} re {}", max_im, max_re);
		}
	}

	#[test]
	fn test_time_moves_waves() {
		let a = heights(Vector2::new(20., 5.), 0.);
		let b = heights(Vector2::new(20., 5.), 3.);
		assert!(a.iter().zip(&b).any(|(a, b)| (a.re - b.re).abs() > 1e-6));
	}

	#[test]
	fn test_wind_along_x_makes_waves_along_x() {
		// Rows are z, columns are x. Waves travelling along x vary most between columns.
		let h = heights(Vector2::new(20., 0.), 0.);
		let (mut dx, mut dz) = (0., 0.);
		for z in 0..N - 1 {
			for x in 0..N - 1 {
				// Undo the (-1)^(x+z) shift from the centred spectrum.
				let at = |z: usize, x: usize| h[z * N + x].re * if (x + z) % 2 == 0 { -1. } else { 1. };
				dx += (at(z, x + 1) - at(z, x)).powi(2);
				dz += (at(z + 1, x) - at(z, x)).powi(2);
			}
		}
		assert!(dx > 2. * dz, "dx {} dz {}", dx, dz);
	}

	#[test]
	fn test_zero_wind_is_calm() {
		assert_eq!(phillips(Vector2::new(1., 1.), Vector2::new(0., 0.), 1., 9.81), 0.);
	}

	fn assert_eq_vecs(a: &[Complex<f64>], b: &[Complex<f64>]) {
		for (a, b) in a.iter().zip(b) {
			assert!((a - b).norm() < 0.1f64);
		}
	}

	#[test]
	fn test_transpose() {

		let input: Vec<Complex<f64>> = (1u32 .. 5u32).map(|x| Complex::new(x as f64, 0.)).collect();
		let transposed = transpose(&input, 2);
		let output = transpose(&transposed, 2);
		let expected = [Complex::new(1.,0.), Complex::new(3.,0.), 
						Complex::new(2.,0.) , Complex::new(4., 0.)];
		assert_eq_vecs(&input, &output);
		assert_eq_vecs(&expected, &transposed);
	}

	#[test]
	fn test_inverse_fft2() {
		let input: Vec<Complex<f64>> = (1u32 .. 17u32).map(|x| Complex::new(x as f64, 0.)).collect();
		let output = fft2(input, 4);
		let o2 = output.clone();
		let output2 = ifft2(o2, 4);
		println!("! {:?} \n\n {:?} ", output, output2);
		let expected: Vec<Complex<f64>> = (1u32 .. 17u32).map(|x| Complex::new(x as f64, 0.)).collect();
		assert_eq_vecs(&expected, &output2);
	}
}

// ## The sea
//
// The sea is the planet's surface (or a plane), raised and lowered by the waves: `Ocean`. Up
// close the surface is displaced, so waves stand up against the horizon and
// hide the troughs behind them; rays find it by marching through the height field. Further
// out a wave is smaller than a pixel, and its slopes matter more than its heights, so the
// displacement fades out and the waves remain only in the normals.

/// The sea's heights and slopes (∂h/∂x, ∂h/∂z) over a periodic tile `size` metres across, and
/// for a tile that displaces the surface, its choppy displacement and the displacement's
/// gradient (see "Choppy crests and breaking" in the module docs).
struct WaveTile {
    n: usize,
    size: f64,
    texels: Vec<[f32; 3]>,
    /// D_x, D_z, ∂D_x/∂x, ∂D_z/∂z, ∂D_x/∂z (eq. 44 and its derivatives), at the same scale as
    /// the heights.
    chop: Option<Vec<[f32; 5]>>,
    /// The amplitudes h̃0, the time, and the scale applied since, to evaluate the tile at other
    /// times (for the foam's history).
    h0: Vec<Complex<f64>>,
    time: f64,
    gain: f64,
}

/// `texels` on an `n` x `n` periodic grid `size` metres across, at (x, z), interpolated
/// bilinearly.
fn bilinear<const N: usize>(n: usize, size: f64, texels: &[[f32; N]], x: f64, z: f64) -> [f64; N] {
    let (u, v) = (x / size * n as f64, z / size * n as f64);
    let n = n as i64;
    let (u0, v0) = (u.floor(), v.floor());
    let (fu, fv) = (u - u0, v - v0);
    let mut out = [0.; N];
    // (A tile is a power of two across: its indices wrap by a mask, which is the remainder
    // without the eight divisions a lookup it took, in the middle of every march's step.)
    debug_assert!(n & (n - 1) == 0);
    for (du, dv, w) in [(0, 0, (1. - fu) * (1. - fv)), (1, 0, fu * (1. - fv)), (0, 1, (1. - fu) * fv), (1, 1, fu * fv)] {
        let (i, j) = ((u0 as i64 + du) & (n - 1), (v0 as i64 + dv) & (n - 1));
        let t = &texels[(j * n + i) as usize];
        for a in 0..N {
            out[a] += t[a] as f64 * w;
        }
    }
    out
}

/// The inverse transforms of h̃(k, t) times each of `multipliers` (functions of k), on the tile's
/// grid with the centred spectrum's sign undone.
fn tile_fields(h0: &[Complex<f64>], n: usize, size: f64, time: f64, multipliers: &[&dyn Fn(Vector2<f64>) -> Complex<f64>]) -> Vec<Vec<f64>> {
    let mut ht = vec![Complex::new(0., 0.); n * n];
    let mut ks = vec![Vector2::zeros(); n * n];
    for j in 0..n {
        for i in 0..n {
            let ind = j * n + i;
            let k = gen_k(i as f64 - n as f64 / 2., j as f64 - n as f64 / 2., size, size);
            let wt = dispersion(k, 9.81) * time;
            let neg_k = ((n - j) % n) * n + (n - i) % n;
            ht[ind] = h0[ind] * Complex::new(wt.cos(), wt.sin()) + h0[neg_k].conj() * Complex::new(wt.cos(), -wt.sin());
            ks[ind] = k;
        }
    }
    let n2 = (n * n) as f64;
    multipliers.iter().map(|m| {
        let f: Vec<Complex<f64>> = ht.iter().zip(&ks).map(|(h, k)| h * m(*k)).collect();
        ifft2(f, n).into_iter().enumerate().map(|(ind, x)| {
            let sign = if (ind % n + ind / n).is_multiple_of(2) { -1. } else { 1. };
            x.re * n2 * sign
        }).collect()
    }).collect()
}

/// The choppy displacement's multipliers: -i k/|k| (eq. 44), and its gradient, k_a k_b / |k|.
fn unit(k: Vector2<f64>) -> Vector2<f64> {
    let len = k.norm();
    if len > 1e-9 { k / len } else { Vector2::zeros() }
}

impl WaveTile {
    /// The waves of the Phillips spectrum for `wind` with wavenumbers from `k_min` up to
    /// `k_max`, on an `n` x `n` grid, at `time`; with the choppy displacement if `choppy`.
    fn new(wind: Vector2<f64>, size: f64, n: usize, k_min: f64, k_max: f64, time: f64, seed: u64, choppy: bool) -> WaveTile {
        let mut s = [0u8; 32];
        s[..8].copy_from_slice(&seed.to_le_bytes());
        let mut rng: StdRng = SeedableRng::from_seed(s);
        let mut h0 = create_amplitude_tile(wind, 1., 9.81, size, size, n, &mut rng);
        for j in 0..n {
            for i in 0..n {
                let k = gen_k(i as f64 - n as f64 / 2., j as f64 - n as f64 / 2., size, size).norm();
                if k < k_min || k >= k_max {
                    h0[j * n + i] = Complex::new(0., 0.);
                }
            }
        }
        let one = |_: Vector2<f64>| Complex::new(1., 0.);
        let sx = |k: Vector2<f64>| Complex::new(0., k.x);
        let sz = |k: Vector2<f64>| Complex::new(0., k.y);
        let dx = |k: Vector2<f64>| Complex::new(0., -unit(k).x);
        let dz = |k: Vector2<f64>| Complex::new(0., -unit(k).y);
        let jxx = |k: Vector2<f64>| Complex::new(unit(k).x * k.x, 0.);
        let jzz = |k: Vector2<f64>| Complex::new(unit(k).y * k.y, 0.);
        let jxz = |k: Vector2<f64>| Complex::new(unit(k).x * k.y, 0.);
        let mut wanted: Vec<&dyn Fn(Vector2<f64>) -> Complex<f64>> = vec![&one, &sx, &sz];
        if choppy {
            wanted.extend_from_slice(&[&dx, &dz, &jxx, &jzz, &jxz]);
        }
        let f = tile_fields(&h0, n, size, time, &wanted);
        let texels = (0..n * n).map(|i| [f[0][i] as f32, f[1][i] as f32, f[2][i] as f32]).collect();
        let chop = choppy.then(|| (0..n * n).map(|i| [f[3][i] as f32, f[4][i] as f32, f[5][i] as f32, f[6][i] as f32, f[7][i] as f32]).collect());
        WaveTile { n, size, texels, chop, h0, time, gain: 1. }
    }

    fn scale(&mut self, k: f64) {
        for t in self.texels.iter_mut() {
            for v in t.iter_mut() {
                *v *= k as f32;
            }
        }
        if let Some(c) = self.chop.as_mut() {
            for t in c.iter_mut() {
                for v in t.iter_mut() {
                    *v *= k as f32;
                }
            }
        }
        self.gain *= k;
    }

    /// `texels` on the tile's grid at (x, z), interpolated bilinearly.
    fn lerp<const N: usize>(&self, texels: &[[f32; N]], x: f64, z: f64) -> [f64; N] {
        bilinear(self.n, self.size, texels, x, z)
    }

    /// Height and slopes at (x, z) of the undisplaced grid.
    fn at(&self, x: f64, z: f64) -> Vector3<f64> {
        let [h, sx, sz] = self.lerp(&self.texels, x, z);
        Vector3::new(h, sx, sz)
    }

    /// The choppy displacement and its gradient at (x, z) of the undisplaced grid (zero if the
    /// tile has none).
    fn chop_at(&self, x: f64, z: f64) -> [f64; 5] {
        match &self.chop {
            Some(c) => self.lerp(c, x, z),
            None => [0.; 5],
        }
    }

    /// The Jacobian of the choppy map x + λD at each texel, `dt` seconds from the tile's time:
    /// below 1 where the surface is squeezed together, at 0 where it folds over.
    fn jacobian(&self, lambda: f64, dt: f64) -> Vec<f64> {
        let jxx = |k: Vector2<f64>| Complex::new(unit(k).x * k.x, 0.);
        let jzz = |k: Vector2<f64>| Complex::new(unit(k).y * k.y, 0.);
        let jxz = |k: Vector2<f64>| Complex::new(unit(k).x * k.y, 0.);
        let f = tile_fields(&self.h0, self.n, self.size, self.time + dt, &[&jxx, &jzz, &jxz]);
        let l = lambda * self.gain;
        (0..self.n * self.n).map(|i| (1. + l * f[0][i]) * (1. + l * f[1][i]) - (l * f[2][i]).powi(2)).collect()
    }

    /// The largest height and slope in the tile.
    fn maxima(&self) -> (f64, f64) {
        self.texels.iter().fold((0., 0.), |(h, s), t| {
            (h.max((t[0] as f64).abs()), s.max(((t[1] as f64).powi(2) + (t[2] as f64).powi(2)).sqrt()))
        })
    }
}

/// Cox and Munk's mean square slope of the sea for a wind of `speed` m/s (C. Cox and W. Munk,
/// "Measurement of the roughness of the sea surface from photographs of the sun's glitter",
/// J. Opt. Soc. Am. 44, 1954; the clean-surface fit).
pub fn cox_munk_slope(speed: f64) -> f64 {
    0.003 + 0.00512 * speed.max(0.)
}

/// Significant wave height (m) of the sea a wind of `speed` m/s raises: the fully developed sea
/// of Pierson and Moskowitz, H_s = 0.22 U²/g (W. J. Pierson and L. Moskowitz, J. Geophys. Res.
/// 69(24), 1964, for the wind at 10 m), or with a `fetch` (m) of open water upwind the younger
/// sea it has had room to raise, H_s = 0.0016 U √(F/g) (JONSWAP: K. Hasselmann et al., 1973),
/// whichever is lower.
pub fn significant_wave_height(speed: f64, fetch: Option<f64>) -> f64 {
    let g = 9.81;
    let developed = 0.22 * speed * speed / g;
    match fetch {
        Some(f) => developed.min(0.0016 * speed * (f.max(0.) / g).sqrt()),
        None => developed,
    }
}

/// The peak wavelength (m) of the waves a wind of `speed` m/s raises over `fetch` m of water:
/// JONSWAP's peak frequency f_p = 3.5 (g/U) (g F / U²)^-0.33 (K. Hasselmann et al., 1973),
/// no lower than the fully developed sea's, ω_p = 0.877 g/U (Pierson and Moskowitz); as deep
/// water waves, λ = g / (2π f²).
pub fn jonswap_peak(speed: f64, fetch: f64) -> f64 {
    let g = 9.81;
    let u = speed.max(0.1);
    let young = 3.5 * g / u * (g * fetch.max(1.) / (u * u)).powf(-0.33);
    let f = young.max(0.877 * g / (u * 2. * std::f64::consts::PI));
    g / (2. * std::f64::consts::PI * f * f)
}

/// How the sea's waves are scaled and shaped.
#[derive(Clone, Copy, Debug)]
pub struct SeaState {
    /// Phillips's constant, if given; overrides the rest of the scaling.
    pub amplitude: Option<f64>,
    /// Scale the waves so their slopes are this fraction of Cox and Munk's (the older
    /// calibration), rather than to a wave height.
    pub resolved: Option<f64>,
    /// The significant wave height (m), if given; otherwise from the wind and `fetch`.
    pub wave_height: Option<f64>,
    pub fetch: Option<f64>,
    /// The choppy displacement's strength, λ: 1 for the first-order Lagrangian (Gerstner)
    /// surface, 0 for none.
    pub choppiness: f64,
    /// The spectrum's peak wavelength (m), if not the fully developed sea's for the wind: a
    /// young sea's, short of fetch (`jonswap_peak`).
    pub peak: Option<f64>,
    /// The wave tiles' sizes as a fraction of the sea's (487, 61.3 and 7.9 m): smaller for the
    /// short waves of a lake.
    pub scale: f64,
}

impl Default for SeaState {
    fn default() -> SeaState {
        SeaState { amplitude: None, resolved: None, wave_height: None, fetch: None, choppiness: 1., peak: None, scale: 1. }
    }
}

impl SeaState {
    /// From an ocean's scene object.
    pub fn parse(opts: &Value) -> SeaState {
        SeaState {
            amplitude: opts["amplitude"].as_f64(),
            resolved: opts["resolved"].as_f64().map(|r| r.clamp(0., 1.)),
            wave_height: opts["wave_height"].as_f64().map(|h| h.max(0.)),
            fetch: opts["fetch"].as_f64(),
            choppiness: SceneFile::parse_number(&opts["choppiness"], 1.).max(0.),
            ..SeaState::default()
        }
    }
}

/// The sea's surface as the sum of three tiles of incommensurate sizes, so that the pattern
/// repeats only far away: swell and wind waves on one, on the next the shorter waves it has no
/// room for, down to a metre, and on the last wavelets down to 12 cm, the chop that gives water
/// its fine texture. The wavelets are too low to matter to the surface's shape, and only tilt
/// its normals.
/// How many of the wave tiles displace the surface; the rest only tilt its normals.
const DISPLACED: usize = 2;

/// The smallest Jacobian the slopes are sharpened by, short of the fold.
const MIN_JACOBIAN: f64 = 0.2;

pub struct WaveField {
    tiles: Vec<WaveTile>,
    /// Bounds on the height and the slope of the displaced surface.
    pub max_height: f64,
    pub max_slope: f64,
    /// The mean square slope of the waves (as interpolated).
    pub mean_square_slope: f64,
    /// The choppy map is x + `lambda` D: negative, so points crowd towards the crests (a Gerstner
    /// wave's particles orbit forwards at its crest), sharpening them.
    lambda: f64,
    /// The wind, m/s.
    pub wind: Vector2<f64>,
}

impl WaveField {
    /// The waves for `wind` (m/s) at `time`, without choppiness. The Phillips spectrum's constant
    /// is arbitrary; with `amplitude` it is that, and otherwise the waves are scaled so their
    /// mean square slope is `resolved` of Cox and Munk's for the wind.
    pub fn new(wind: Vector2<f64>, time: f64, seed: u64, amplitude: Option<f64>, resolved: f64) -> WaveField {
        WaveField::sea(wind, time, seed, SeaState { amplitude, resolved: Some(resolved), choppiness: 0., ..SeaState::default() })
    }

    /// The waves for `wind` (m/s) at `time`, scaled and shaped by `state`: by default to the
    /// wave height the wind raises (`significant_wave_height`), with choppy crests.
    pub fn sea(wind: Vector2<f64>, time: f64, seed: u64, state: SeaState) -> WaveField {
        // The largest tile holds the wind sea's peak, λ_p = 2π g / ω_p², ω_p = 0.877 g/U
        // (Pierson-Moskowitz), with room to spare.
        let developed = 2. * std::f64::consts::PI * wind.norm().powi(2) / (0.877f64.powi(2) * 9.81);
        let peak = state.peak.unwrap_or(developed);
        // A younger sea's spectrum: Phillips's, peaking at k = 1 / (√2 L) for L = U² / g, for
        // the wind that would peak at `peak` (its height is scaled below to the real wind's).
        let spectral = match state.peak {
            Some(p) if wind.norm() > 0. => wind.normalize() * (9.81 * p / (2. * std::f64::consts::PI * 2f64.sqrt())).sqrt(),
            _ => wind,
        };
        let s = state.scale;
        let (big, small, tiny, n) = ((487. * s).max(1.2 * peak), 61.3 * s, 7.9 * s, 256);
        let choppy = state.choppiness > 0.;
        // Each tile takes over just short of the previous one's shortest waves.
        let k_split = std::f64::consts::PI * n as f64 / big * 0.9;
        let k_tiny = std::f64::consts::PI * (n / 2) as f64 / small * 0.9;
        let mut tiles = vec![
            WaveTile::new(spectral, big, n, 0., k_split, time, seed, choppy),
            WaveTile::new(spectral, small, n / 2, k_split, k_tiny, time, seed.wrapping_add(1), choppy),
            WaveTile::new(spectral, tiny, n / 2, k_tiny, f64::INFINITY, time, seed.wrapping_add(2), false),
        ];
        // Each wave's amplitude should be √P(k) Δk, but `create_amplitude_tile` leaves out the
        // grid's spacing Δk = 2π/L, which is larger on a smaller tile: put it back, relative to
        // the largest, or the shorter waves are far too weak.
        for t in tiles.iter_mut().skip(1) {
            t.scale(big / t.size);
        }
        // Measured unchopped (the displacement scales with the waves, so it would be wrong until
        // they are), then chopped.
        let mut field = WaveField { tiles: vec![], max_height: 0., max_slope: 0., mean_square_slope: 0.,
            lambda: 0., wind };
        field.tiles = std::mem::take(&mut tiles);
        let k = match (state.amplitude, state.resolved) {
            (Some(a), _) => a.sqrt(),
            (None, Some(resolved)) => {
                let mss = field.measure_slope();
                if mss > 0. { (resolved * cox_munk_slope(wind.norm()) / mss).sqrt() } else { 0. }
            },
            (None, None) => {
                let target = state.wave_height.unwrap_or_else(|| significant_wave_height(wind.norm(), state.fetch));
                let (_, sd) = field.height_exceeded(0.5);
                if sd > 0. { target / (4. * sd) } else { 0. }
            },
        };
        for t in field.tiles.iter_mut() {
            t.scale(k);
        }
        field.lambda = -state.choppiness;
        let (h, _) = field.tiles[..DISPLACED].iter().map(|t| t.maxima()).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        field.max_height = h;
        field.mean_square_slope = field.measure_slope();
        field.max_slope = field.measure_max_slope();
        field
    }

    /// The waves a wind raises on a lake: `fetch` metres of water upwind, so a young sea of
    /// short waves (`jonswap_peak`, `significant_wave_height`), on tiles scaled to them, without
    /// choppiness (they are centimetres high: they only tilt the surface).
    pub fn lake(wind: Vector2<f64>, fetch: f64, time: f64, seed: u64) -> WaveField {
        let peak = jonswap_peak(wind.norm(), fetch);
        let scale = (25. * peak / 487.).min(1.);
        let mut field = WaveField::sea(wind, time, seed, SeaState { fetch: Some(fetch), peak: Some(peak), scale, choppiness: 0., ..SeaState::default() });
        // JONSWAP's fits were made over kilometres of fetch; over a lake's few hundred metres
        // the height they give, on waves that short, is steeper than the open sea in the same
        // wind. Lakes are no rougher than the sea: their slopes at most Cox and Munk's.
        let most = cox_munk_slope(wind.norm());
        if field.mean_square_slope > most {
            let k = (most / field.mean_square_slope).sqrt();
            for t in field.tiles.iter_mut() { t.scale(k); }
            field.max_height *= k;
            field.mean_square_slope = field.measure_slope();
            field.max_slope = field.measure_max_slope();
        }
        field
    }

    /// Where on the undisplaced grid the point now at (x, z) came from: the choppy map
    /// p + λD(p) inverted by fixed-point iteration (it contracts wherever the surface hasn't
    /// folded).
    pub fn source(&self, x: f64, z: f64) -> (f64, f64) {
        if self.lambda == 0. {
            return (x, z);
        }
        let mut p = (x, z);
        for _ in 0..3 {
            let (mut dx, mut dz) = (0., 0.);
            for t in &self.tiles[..DISPLACED] {
                let c = t.chop_at(p.0, p.1);
                dx += c[0];
                dz += c[1];
            }
            p = (x - self.lambda * dx, z - self.lambda * dz);
        }
        p
    }

    /// The Jacobian of the choppy map at a point of the grid: how much the surface there is
    /// stretched (above 1) or squeezed together (below), and the matrix's entries.
    fn squeeze(&self, p: (f64, f64)) -> (f64, f64, f64, f64) {
        let (mut jxx, mut jzz, mut jxz) = (0., 0., 0.);
        for t in &self.tiles[..DISPLACED] {
            let c = t.chop_at(p.0, p.1);
            jxx += c[2];
            jzz += c[3];
            jxz += c[4];
        }
        let (a, d, b) = (1. + self.lambda * jxx, 1. + self.lambda * jzz, self.lambda * jxz);
        (a * d - b * b, a, d, b)
    }

    /// Height (of the displaced tiles) and slopes (of all) at horizontal position (x, z): the
    /// choppy surface's, whose slopes are the grid's through the inverse of the map's Jacobian.
    pub fn at(&self, x: f64, z: f64) -> Vector3<f64> {
        let p = self.source(x, z);
        let mut w = Vector3::zeros();
        for t in &self.tiles[..DISPLACED] {
            w += t.at(p.0, p.1);
        }
        if self.lambda != 0. {
            // ∇_x h = M⁻¹ ∇_p h for the symmetric M = I + λ∇D.
            let (det, a, d, b) = self.squeeze(p);
            let det = det.max(MIN_JACOBIAN);
            let (sx, sz) = (w.y, w.z);
            w.y = (d * sx - b * sz) / det;
            w.z = (a * sz - b * sx) / det;
        }
        for t in &self.tiles[DISPLACED..] {
            let v = t.at(x, z);
            w += Vector3::new(0., v.y, v.z);
        }
        w
    }

    /// The height of all the waves at (x, z), the shortest included.
    pub fn full_height(&self, x: f64, z: f64) -> f64 {
        let p = self.source(x, z);
        self.tiles[..DISPLACED].iter().map(|t| t.at(p.0, p.1).x).sum::<f64>()
            + self.tiles[DISPLACED..].iter().map(|t| t.at(x, z).x).sum::<f64>()
    }

    /// The height the waves (`full_height`) exceed over `fraction` of the sea, and the spread
    /// of the heights (their standard deviation), sampled across the largest tile.
    pub fn height_exceeded(&self, fraction: f64) -> (f64, f64) {
        let (m, size) = (256, self.tiles[0].size);
        let mut rng = geometry::Pcg32::new(17, 3);
        let mut heights: Vec<f64> = (0..m * m).map(|i| {
            let (x, z) = (((i % m) as f64 + rng.next_f64()) * size / m as f64, ((i / m) as f64 + rng.next_f64()) * size / m as f64);
            self.full_height(x, z)
        }).collect();
        let mean = heights.iter().sum::<f64>() / heights.len() as f64;
        let sd = (heights.iter().map(|h| (h - mean).powi(2)).sum::<f64>() / heights.len() as f64).sqrt();
        heights.sort_by(|a, b| a.total_cmp(b));
        let k = ((1. - fraction.clamp(0., 1.)) * (heights.len() - 1) as f64).round() as usize;
        (heights[k], sd)
    }

    /// The mean square slope as sampled: between texels, interpolation smooths the shortest
    /// waves, so it is measured on a grid finer than the tiles'.
    fn measure_slope(&self) -> f64 {
        let (m, size) = (1024, self.tiles[0].size);
        let step = size / m as f64;
        let sum: f64 = (0..m).into_par_iter().map(|j| {
            (0..m).map(|i| {
                let s = self.at((i as f64 + 0.37) * step, (j as f64 + 0.61) * step);
                s.y * s.y + s.z * s.z
            }).sum::<f64>()
        }).sum();
        sum / (m * m) as f64
    }

    /// The steepest the displaced surface gets (of its displaced tiles), sharpened crests
    /// included, sampled finely across the largest tile.
    fn measure_max_slope(&self) -> f64 {
        let (m, size) = (1024, self.tiles[0].size);
        let step = size / m as f64;
        let plain: f64 = self.tiles[..DISPLACED].iter().map(|t| t.maxima().1).sum();
        if self.lambda == 0. {
            return plain;
        }
        let sharpened = (0..m).into_par_iter().map(|j| {
            (0..m).map(|i| {
                let (x, z) = ((i as f64 + 0.37) * step, (j as f64 + 0.61) * step);
                let p = self.source(x, z);
                let det = self.squeeze(p).0.max(MIN_JACOBIAN);
                let s: Vector3<f64> = self.tiles[..DISPLACED].iter().map(|t| t.at(p.0, p.1)).sum();
                (s.y * s.y + s.z * s.z).sqrt() / det.min(1.)
            }).fold(0., f64::max)
        }).reduce(|| 0., f64::max);
        plain.max(sharpened)
    }
}

/// The sea: the planet's surface (a sphere of `planet_radius`) or, with none, the plane y = 0,
/// displaced by `waves` within `detail_distance` of `detail_center` ([x, z]: put it under the
/// camera), and beyond that smooth, with normals that still carry the waves' slopes. The shape
/// is all the radius changes: the waves and the material are the same.
pub struct Ocean {
    pub planet_radius: Option<f64>,
    pub waves: WaveField,
    pub detail_center: Vector2<f64>,
    pub detail_distance: f64,
    /// For transparent water, its absorption per metre: shadow rays from under water pass up
    /// through the surface.
    pub absorption: Option<Color>,
}

impl Ocean {
    fn center(&self) -> Option<Vector3<f64>> {
        self.planet_radius.map(|r| Vector3::new(0., -r, 0.))
    }

    /// Height above the mean surface.
    fn altitude(&self, p: &Vector3<f64>) -> f64 {
        match self.planet_radius {
            Some(r) => (p - Vector3::new(0., -r, 0.)).norm() - r,
            None => p.y,
        }
    }

    /// The local vertical.
    fn up(&self, p: &Vector3<f64>) -> Vector3<f64> {
        self.center().map_or(Vector3::y(), |c| (p - c).normalize())
    }

    /// How much of the waves' height is displaced at (x, z): all of it within half the detail
    /// distance, fading to none at it.
    fn displaced(&self, x: f64, z: f64) -> f64 {
        let d = (Vector2::new(x, z) - self.detail_center).norm() / self.detail_distance;
        1. - crate::procedural::nubis_clouds::smoothstep(0.5, 1., d)
    }

    /// Height of `p` above the displaced surface.
    fn above(&self, p: &Vector3<f64>) -> f64 {
        let altitude = self.altitude(p);
        let a = self.displaced(p.x, p.z);
        if a <= 0. {
            return altitude;
        }
        altitude - a * self.waves.at(p.x, p.z).x
    }

    /// The local vertical and horizontal axes (x and z at the origin).
    fn frame(&self, p: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let up = self.up(p);
        let east = (Vector3::x() - up * up.x).normalize();
        (up, east, east.cross(&up))
    }

    /// The surface's normal at `p`: the waves' slopes, as displaced, and in full beyond.
    pub fn normal_at(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let (up, x, z) = self.frame(p);
        let w = self.waves.at(p.x, p.z);
        (up - x * w.y - z * w.z).normalize()
    }

    /// The stretch of the ray (by distance along it, maybe unbounded) below altitude `level`:
    /// inside a sphere, or below a plane.
    fn below(&self, r: &Ray, level: f64) -> Option<(f64, f64)> {
        match self.planet_radius {
            Some(radius) => {
                let o = r.ro - Vector3::new(0., -radius, 0.);
                let b = o.dot(&r.rd);
                let disc = b * b - (o.norm_squared() - (radius + level).powi(2));
                if disc < 0. {
                    return None;
                }
                Some((-b - disc.sqrt(), -b + disc.sqrt()))
            },
            None => {
                let t = (level - r.ro.y) / r.rd.y;
                if r.rd.y < 0. {
                    Some((t, f64::INFINITY))
                } else if r.rd.y > 0. {
                    Some((f64::NEG_INFINITY, t))
                } else if r.ro.y < level {
                    Some((f64::NEG_INFINITY, f64::INFINITY))
                } else {
                    None
                }
            },
        }
    }

    /// Where the ray is within `detail_distance` of the detail centre, horizontally.
    fn detail_span(&self, r: &Ray) -> Option<(f64, f64)> {
        let o = Vector2::new(r.ro.x, r.ro.z) - self.detail_center;
        let d = Vector2::new(r.rd.x, r.rd.z);
        let a = d.norm_squared();
        if a < 1e-18 {
            return if o.norm() < self.detail_distance { Some((f64::NEG_INFINITY, f64::INFINITY)) } else { None };
        }
        let b = o.dot(&d);
        let disc = b * b - a * (o.norm_squared() - self.detail_distance * self.detail_distance);
        if disc < 0. {
            return None;
        }
        Some(((-b - disc.sqrt()) / a, (-b + disc.sqrt()) / a))
    }

    /// The first crossing of the displaced surface from `t0` to `t1`, from above or below (a
    /// ray refracted into the water comes back up through it), by stepping no further than the
    /// surface could move to meet the ray, then bisecting.
    fn march(&self, r: &Ray, t0: f64, t1: f64) -> Option<f64> {
        let fade_rate = 1.5 / (0.5 * self.detail_distance) * self.waves.max_height;
        // A ray leaving the surface starts on it: step off it first.
        let mut t = t0 + 1e-4;
        let start = self.above(&(r.ro + r.rd * t));
        let side = if start < 0. { -1. } else { 1. };
        let mut f = start.abs();
        while t < t1 {
            let p = r.ro + r.rd * t;
            let up = self.up(&p);
            // Towards the surface: down from above, up from below.
            let descent = (-side * r.rd.dot(&up)).max(0.);
            let horizontal = (1. - r.rd.dot(&up).powi(2)).max(0.).sqrt();
            let rate = descent + horizontal * (self.waves.max_slope + fade_rate) + 1e-6;
            let step = (f / rate).max(0.01).min(t1 - t + 0.01);
            let next = t + step;
            let g = side * self.above(&(r.ro + r.rd * next));
            if g <= 0. {
                // Crossed between t and next. Placed by false position, the end that stays put
                // weighing half as much each time (the Illinois rule, so neither end stalls):
                // the surface is nearly straight across one step, and each asking is the
                // choppy map inverted, so six of them, to a hundredth of a micrometre, where
                // twenty-four halvings were a fifth of a render of the sea.
                let (mut lo, mut flo, mut hi, mut fhi) = (t, f, next, g);
                let between = |lo: f64, flo: f64, hi: f64, fhi: f64| if flo - fhi > 0. { (lo + (hi - lo) * flo / (flo - fhi)).clamp(lo, hi) } else { hi };
                for _ in 0..6 {
                    let x = between(lo, flo, hi, fhi);
                    let v = side * self.above(&(r.ro + r.rd * x));
                    if v > 0. {
                        (lo, flo, fhi) = (x, v, fhi * 0.5);
                    } else {
                        (hi, fhi, flo) = (x, v, flo * 0.5);
                    }
                }
                return Some(between(lo, flo, hi, fhi));
            }
            t = next;
            f = g;
        }
        None
    }
}

impl Ocean {
    /// The sea's shape as data, for the GPU's kernel of it (`pathtrace::ocean_hit`; the layout
    /// is `pathtrace::OCEAN_*`): the planet's radius (0 for a plane), where the waves displace
    /// it, their greatest height and slope, how many tiles of waves there are and how many
    /// displace, the choppy map's λ, and places for where its foam's record is and for the
    /// planet in the render's frame (which whoever lays the record out fills in:
    /// `kernels::Buffers`); then for each tile its size, its texels a side, and where
    /// its texels and its choppy displacement's (0 if it has none) begin from the record's
    /// start.
    pub fn export(&self) -> Vec<f32> {
        let tiles = &self.waves.tiles;
        // (The kernel wraps a tile's indices by a mask.)
        assert!(tiles.iter().all(|t| t.n.is_power_of_two()), "a wave tile's size must be a power of two");
        let header = 14 + 4 * tiles.len();
        let mut out = vec![
            self.planet_radius.unwrap_or(0.) as f32, self.detail_center.x as f32, self.detail_center.y as f32, self.detail_distance as f32,
            self.waves.max_height as f32, self.waves.max_slope as f32, tiles.len() as f32, DISPLACED.min(tiles.len()) as f32,
            self.waves.lambda as f32, 0.,
            // (The planet's centre in the render's frame, and its `excess`: laid out with it.)
            0., 0., 0., 0.,
        ];
        let mut at = header;
        for t in tiles {
            let texels = at;
            at += 3 * t.texels.len();
            let chop = t.chop.as_ref().map_or(0, |c| {
                at += 5 * c.len();
                at - 5 * c.len()
            });
            out.extend([t.size as f32, t.n as f32, f32::from_bits(texels as u32), f32::from_bits(chop as u32)]);
        }
        for t in tiles {
            out.extend(t.texels.iter().flatten());
            if let Some(c) = &t.chop {
                out.extend(c.iter().flatten());
            }
        }
        out
    }
}

impl Geometry for Ocean {
    fn ocean(&self) -> Option<&Ocean> {
        Some(self)
    }

    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let r = Ray { ro: r.ro, rd: r.rd.normalize() };
        let h = self.waves.max_height;
        let hit = |t: f64| {
            let point = r.ro + r.rd * t;
            Some(RawIntersection { face: 0, part: 0, dist: t, point, normal: self.normal_at(&point) })
        };
        // The smooth surface, where it is beyond the displaced region.
        let detail = self.detail_span(&r);
        let outside = |t: f64| detail.is_none_or(|(a, b)| t < a || t > b);
        let smooth = self.below(&r, 0.)
            .map(|(a, b)| [a, b])
            .and_then(|ts| ts.into_iter().find(|t| t.is_finite() && *t > 0. && outside(*t)));
        // The displaced region: below the waves' crests and above their troughs, near the detail
        // centre.
        let displaced = match (detail, self.below(&r, h)) {
            (Some((da, db)), Some((oa, ob))) => {
                let inner = self.below(&r, -h).map(|(a, _)| a).filter(|a| a.is_finite() && *a > 0.);
                let start = da.max(oa).max(0.);
                let end = db.min(ob).min(inner.unwrap_or(f64::INFINITY)).min(start + 1e7);
                if start < end { self.march(&r, start, end) } else { None }
            },
            _ => None,
        };
        match (displaced, smooth) {
            (Some(a), Some(b)) => hit(a.min(b)),
            (Some(a), None) => hit(a),
            (None, Some(b)) => hit(b),
            (None, None) => None,
        }
    }

    /// Light rising through the surface from under transparent water passes (as much as the
    /// surface transmits and the water doesn't absorb); otherwise the sea blocks it.
    fn transmittance_color(&self, r: &Ray, min: f64, max: f64) -> Color {
        let hit = match self.intersects_range(r, min, max) {
            Some(x) => x,
            None => return Color::white(),
        };
        let absorption = match &self.absorption {
            Some(a) => a,
            None => return Color::black(),
        };
        let rd = r.rd.normalize();
        let up = self.up(&hit.point);
        let start = r.ro + rd * min;
        let depth = -self.above(&start);
        if rd.dot(&up) <= 0. || depth < 0. {
            return Color::black();
        }
        // The path in the water, along the direction the light refracts to under the surface.
        let cos_air = rd.dot(&hit.normal).clamp(0., 1.);
        let cos_water = (1. - (1. - cos_air * cos_air) / (WATER_INDEX * WATER_INDEX)).max(1e-3).sqrt();
        let length = depth / cos_water.max(rd.dot(&up));
        let a = absorption.as_unbounded();
        Color { rgb: a.rgb.map(|k| (-k * length).exp()) } * (1. - fresnel(cos_air))
    }

    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        crate::color::luminance(&self.transmittance_color(r, min, max))
    }

    fn transmits(&self) -> bool { self.absorption.is_some() }

    fn bounds(&self) -> BBox {
        let h = self.waves.max_height;
        match self.center() {
            Some(c) => {
                let r = Vector3::repeat(self.planet_radius.unwrap() + h);
                BBox::new(c - r, c + r)
            },
            None => BBox::new(Vector3::new(-1e12, -h, -1e12), Vector3::new(1e12, h, 1e12)),
        }
    }
}

/// The physical sea: Fresnel reflection from ripples on the waves, the sun's glitter, and the
/// faint light the water body sends back up, lit as the scene is.
///
/// The waves' normal comes from the geometry. Ripples too small for it carry the rest of the
/// slope: a Beckmann distribution of microfacet normals, of mean square slope `roughness`.
/// Reflections are sampled from it, and the sun's glitter is evaluated through it directly, so
/// the glitter path is smooth rather than a scatter of rare hits on the sun's disc. The water
/// reflects by Fresnel's equations, 2% straight down rising to all at grazing angles. What it
/// lets in is mostly absorbed: deep, clear water sends back a percent or two of blue and almost
/// no red (A. Morel, "Optical properties of pure water and pure sea water", 1974), lit by the
/// sun and lights as they are, so the sea darkens and reddens at sunset.
pub struct PhysicalOcean {
    pub water: Color,
    pub roughness: f64,
    /// The planet's centre, for the local vertical; None for a flat sea.
    pub center: Option<Vector3<f64>>,
    /// For transparent water, its absorption per metre (see the module's notes).
    pub absorption: Option<Color>,
    /// Whitecaps, if the wind raises any.
    pub foam: Option<Foam>,
    /// Where the sea is over ground near the shore, the waves breaking there (`Surf`).
    pub surf: Option<Surf>,
}

/// McCowan's breaking index: a wave breaks when its height reaches this fraction of the depth
/// (J. McCowan, "On the highest wave of permanent type", Phil. Mag. 38, 1894; the value used by
/// Battjes and Janssen for their maximum height, H_m = γ d).
pub const BREAKING_INDEX: f64 = 0.78;

/// The fraction of waves breaking where waves of root-mean-square height `hrms` (m) reach water
/// `depth` metres deep, by Battjes and Janssen's model of random waves in the surf zone: the
/// heights are Rayleigh distributed but truncated at H_m = γ d, those at it breaking, so the
/// fraction Q_b satisfies (1 - Q_b) / -ln Q_b = (H_rms / H_m)² (J. A. Battjes and J. P. F. M.
/// Janssen, "Energy loss and set-up due to breaking of random waves", Proc. 16th Int. Conf.
/// Coastal Engineering, 1978). None break in deep water; all where the waves are as high as the
/// water allows, and over land.
pub fn breaking_fraction(hrms: f64, depth: f64) -> f64 {
    if depth <= 0. { return 1.; }
    let b2 = (hrms / (BREAKING_INDEX * depth)).powi(2);
    if b2 >= 1. { return 1.; }
    if b2 < 0.05 { return 0.; }
    // f(Q) = (1 - Q) + b² ln Q rises from -∞ at 0 through its root to 0 at 1 (its maximum
    // there is past the root for b² < 1): bisect on (0, 1).
    let f = |q: f64| (1. - q) + b2 * q.ln();
    let (mut lo, mut hi) = (1e-12, 1. - 1e-12);
    if f(hi) < 0. { return 1.; }
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if f(mid) < 0. { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

/// The wavenumber (rad/m) of waves of angular frequency `omega` (rad/s) in water `depth`
/// metres deep, by linear theory's dispersion relation ω² = g k tanh(k h).
pub fn wavenumber(omega: f64, depth: f64) -> f64 {
    let g = 9.81;
    let deep = omega * omega / g;
    if depth <= 0. { return f64::INFINITY; }
    // g k tanh(k h) rises with k. Both the deep-water k (tanh < 1) and the shallow-water
    // ω / √(g h) (tanh(kh) < kh) are below the root: bisect from the larger, doubling for a
    // bound above it.
    let lo0 = deep.max(omega / (g * depth).sqrt());
    let mut hi = 2. * lo0;
    while g * hi * (hi * depth).tanh() < omega * omega { hi *= 2.; }
    let mut lo = lo0;
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if g * mid * (mid * depth).tanh() < omega * omega { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

/// How much taller waves of angular frequency `omega` are in water `depth` metres deep than in
/// deep water, coming straight in, by conservation of their energy flux: K_s = √(c_g0 / c_g),
/// the group velocity c_g = n c, n = ½ (1 + 2kh / sinh 2kh), in deep water half the phase
/// speed g / ω (linear theory; e.g. Dean and Dalrymple, "Water Wave Mechanics for Engineers
/// and Scientists", 1991). A little below 1 at intermediate depths, rising without bound in
/// the shallows (where the waves break first).
pub fn shoaling(omega: f64, depth: f64) -> f64 {
    let g = 9.81;
    let k = wavenumber(omega, depth);
    if !k.is_finite() { return f64::INFINITY; }
    let kh = k * depth;
    if kh > 20. { return 1.; }
    let n = 0.5 * (1. + 2. * kh / (2. * kh).sinh());
    let cg = n * omega / k;
    let cg0 = 0.5 * g / omega;
    (cg0 / cg).sqrt()
}

/// The water's depth under a point of the sea's surface, metres, if known.
pub type Depth = std::sync::Arc<dyn Fn(&Vector3<f64>) -> Option<f64> + Send + Sync>;

/// The surf: the water's depth under the sea's surface near the land (a function of a point
/// on the surface, None where it doesn't know, as far out at sea), the waves' root-mean-square
/// height out at sea, and their angular frequency (the spectrum's peak), by which they grow as
/// they come into the shallows (`shoaling`).
#[derive(Clone)]
pub struct Surf {
    pub depth: Depth,
    pub hrms: f64,
    pub omega: f64,
    /// The way the waves travel out at sea (unit, scene x and z): the wind's.
    pub heading: Vector2<f64>,
    /// The shoreface's Dean parameter, m^(1/3) (`terrain::scenery::dean_a`), by which the
    /// distance from the shore follows from the depth.
    pub dean_a: f64,
    /// Each wave's height about the root-mean-square, varying along its crest and from wave to
    /// wave (wave groups).
    pub noise: crate::noise::PerlinNoise,
}

impl Surf {
    /// The fraction of waves breaking at `p`: the waves there as tall as shoaling makes them.
    pub fn breaking(&self, p: &Vector3<f64>) -> f64 {
        (self.depth)(p).map_or(0., |d| breaking_fraction(self.hrms * shoaling(self.omega, d), d))
    }

    /// The white water at `p`, 0 to 1, and the fraction of waves breaking there. In the surf
    /// zone the waves are refracted: their crests swing round towards the depth contours, as the
    /// part in deeper water outruns the part in shallower (Snell's law, sin θ / c the same
    /// all the way in, over contours taken as straight and parallel), and their heights change
    /// by K_r = √(cos θ₀ / cos θ) as the crests spread or bunch, as well as by shoaling. The
    /// crests are laid out by their phase: across the shore, on Dean's profile (y = (d / A)^1.5
    /// from the shore) in shallow water, S = ∫ k dy = 1.5 ω y^(2/3) / √(g A); along it, k₀ sin
    /// θ₀ (Snell's constant). A wave, its height varying about the root-mean-square from wave
    /// to wave and along its crest, breaks where that height reaches 0.78 of the depth, and is
    /// white on its front and behind it with the foam it leaves, fading as it ages. Waves travelling away from the shore (an offshore wind's) never reach it:
    /// none break.
    pub fn white(&self, p: &Vector3<f64>) -> (f64, f64) {
        let g = 9.81;
        let Some(d) = (self.depth)(p) else { return (0., 0.) };
        if d <= 0. { return (1., 1.); }
        // Out to sea where the depth increases, from its slope across a couple of metres.
        let e = 2.;
        let at = |dx: f64, dz: f64| (self.depth)(&(p + Vector3::new(dx, 0., dz)));
        let (Some(xp), Some(xm), Some(zp), Some(zm)) = (at(e, 0.), at(-e, 0.), at(0., e), at(0., -e)) else { return (0., 0.) };
        let gradient = Vector2::new(xp - xm, zp - zm) / (2. * e);
        if gradient.norm() < 1e-6 { return (0., 0.); }
        let offshore = gradient.normalize();
        let along = Vector2::new(-offshore.y, offshore.x);
        // The angle the waves come in at, out at sea, from the shore's normal.
        let cos0 = self.heading.dot(&-offshore);
        if cos0 <= 0.05 { return (0., 0.); }
        let sin0 = self.heading.dot(&along);
        let k0 = self.omega * self.omega / g;
        let k = wavenumber(self.omega, d);
        let sin = (sin0 * k0 / k).clamp(-1., 1.);
        let refraction = (cos0 / (1. - sin * sin).sqrt().max(1e-3)).sqrt();
        let grow = shoaling(self.omega, d) * refraction;
        let breaking = breaking_fraction(self.hrms * grow, d);
        if breaking <= 0. { return (0., 0.); }
        // The phase of the crests here.
        let y = (d / self.dean_a).powf(1.5);
        let t = p.x * along.x + p.z * along.y;
        let phase = 1.5 * self.omega * y.powf(2. / 3.) / (g * self.dean_a).sqrt() + k0 * sin0 * t;
        let wave = (phase / std::f64::consts::TAU).floor();
        // This wave's height here, about the root-mean-square.
        let n = 0.5 + 0.5 * self.noise.noise(t / 40., wave * 0.37, 0.5);
        let height = self.hrms * (0.4 + 1.2 * n) * grow;
        // A broken wave's foam: the roller on its front, and behind it (seaward: the phase grows
        // out to sea) the foam it left as it passed, as old as the phase since its crest over
        // ω, fading as the whitecaps' does (`FOAM_DECAY`); so the inner surf zone, where every
        // wave breaks, stays about half white between them.
        let age = (phase / std::f64::consts::TAU).rem_euclid(1.) * std::f64::consts::TAU / self.omega;
        let foam = (-age / FOAM_DECAY).exp();
        let white = crate::math::smoothstep(0.95, 1.05, height / (BREAKING_INDEX * d)) * foam;
        (white, breaking)
    }
}

/// Residual foam's albedo, as a fraction of fresh foam's: a thinner layer of larger bubbles.
const RESIDUAL_FOAM: f64 = 0.7;

/// The light that bubbles under the surface send back, as an albedo added to the water's: a
/// dense bubble cloud scatters most light it meets, but what comes back up has crossed some
/// water, which takes the red (as the turquoise under a breaking wave).
const BUBBLES: Color = Color { rgb: Vector3::new(0.01, 0.05, 0.06) };

/// Whitecap coverage by Monahan and O'Muircheartaigh's fit, for a wind of `speed` m/s (at 10 m).
pub fn whitecap_coverage(speed: f64) -> f64 {
    (3.84e-6 * speed.max(0.).powf(3.41)).min(1.)
}

/// How far back (s) the foam's history goes, in steps of `FOAM_STEP`; and how quickly foam left
/// by a breaking crest fades, its e-folding time (s): a whitecap's dense foam thins into a
/// residual patch within seconds (stage A to stage B), which itself lasts some seconds more
/// (Monahan and Lu, 1990; A. H. Callaghan et al., Geophys. Res. Lett. 39, 2012).
const FOAM_HISTORY: usize = 12;
const FOAM_STEP: f64 = 0.5;
const FOAM_DECAY: f64 = 3.5;
/// The Jacobian's range over which a crest goes from whole to breaking.
const FOAM_EDGE: f64 = 0.12;

/// Foam from breaking waves (see "Whitecaps" in the module docs). A wave breaks where the choppy
/// surface is squeezed towards folding over, its Jacobian below `threshold`: there the whitecap
/// is fresh, dense foam. Foam stays with the water it formed in, which the crest leaves behind,
/// and fades over seconds: so each crest that broke trails a residual patch, broken into lace as
/// it thins, and in a gale drawn out in streaks along the wind. The threshold is set so foam
/// covers a fraction `coverage` of the sea.
pub struct Foam {
    pub waves: WaveField,
    pub threshold: f64,
    /// Fresh foam's albedo; residual foam is thinner, and darker.
    pub albedo: Color,
    /// Per displaced tile, fresh and residual foam at each texel.
    tiles: Vec<(usize, f64, Vec<[f32; 2]>)>,
    wind: Vector2<f64>,
    speed: f64,
    noise: crate::noise::PerlinNoise,
    /// The lace noise's spread, to turn it into a uniform rank.
    lace_sd: f64,
}

/// A breaking crest's foam for Jacobian `j` against `threshold`: 1 well below it, 0 above.
fn breaking(j: f64, threshold: f64) -> f64 {
    let t = ((threshold + FOAM_EDGE - j) / (2. * FOAM_EDGE)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Fresh and residual foam at a texel with Jacobians `history` (now, then earlier).
fn foam_from(history: impl Iterator<Item = f64>, threshold: f64) -> (f64, f64) {
    let (mut fresh, mut residual) = (0., 0f64);
    for (k, j) in history.enumerate() {
        let b = breaking(j, threshold);
        if k == 0 {
            fresh = b;
        } else {
            residual = residual.max(b * (-(k as f64) * FOAM_STEP / FOAM_DECAY).exp());
        }
    }
    (fresh, residual)
}

impl Foam {
    /// A fraction `coverage` of the sea, on the crests of `waves` raised by `wind`.
    pub fn new(waves: WaveField, wind: Vector2<f64>, coverage: f64, albedo: Color) -> Foam {
        // A sea without choppy crests still breaks where they would be.
        let lambda = if waves.lambda != 0. { waves.lambda } else { -1. };
        let history: Vec<Vec<Vec<f64>>> = waves.tiles[..DISPLACED].iter().map(|t| {
            (0..=FOAM_HISTORY).into_par_iter().map(|k| t.jacobian(lambda, -(k as f64) * FOAM_STEP)).collect()
        }).collect();
        let texels = |tile: usize| history[tile][0].len();
        let foam_at = |tile: usize, i: usize, threshold: f64| foam_from(history[tile].iter().map(|h| h[i]), threshold);
        // The share of the sea foam covers, the tiles' foam taken as independent.
        let covered = |threshold: f64| {
            1. - (0..history.len()).map(|tile| {
                let m = (0..texels(tile)).into_par_iter().map(|i| {
                    let (f, r) = foam_at(tile, i, threshold);
                    f.max(r)
                }).sum::<f64>() / texels(tile) as f64;
                1. - m
            }).product::<f64>()
        };
        let target = coverage.clamp(0., 1.);
        let (mut lo, mut hi) = (-1.5, 1.);
        let threshold = if target <= 0. {
            -10.
        } else if covered(hi) < target {
            hi
        } else {
            for _ in 0..30 {
                let mid = 0.5 * (lo + hi);
                if covered(mid) < target { lo = mid } else { hi = mid }
            }
            0.5 * (lo + hi)
        };
        let tiles = waves.tiles[..DISPLACED].iter().enumerate().map(|(k, t)| {
            let foam = (0..texels(k)).map(|i| {
                let (f, r) = foam_at(k, i, threshold);
                [f as f32, r as f32]
            }).collect();
            (t.n, t.size, foam)
        }).collect();
        let noise = crate::noise::PerlinNoise::shuffled(0xf0a);
        let lace_sd = {
            let mut rng = geometry::Pcg32::new(29, 1);
            let v: Vec<f64> = (0..4096).map(|_| {
                let (x, z) = (rng.next_f64() * 300., rng.next_f64() * 300.);
                0.65 * noise.fbm(x / 1.6, 0.3, z / 1.6, 3, 0.55, 2.2) + 0.35 * noise.fbm(x / 6., 4.1, z / 6., 2, 0.5, 2.)
            }).collect();
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt().max(1e-6)
        };
        Foam {
            speed: wind.norm(),
            wind: if wind.norm() > 0. { wind.normalize() } else { Vector2::new(1., 0.) },
            waves, threshold, albedo, tiles, noise, lace_sd,
        }
    }

    /// Fresh and residual foam at (x, z), and the point of the undisplaced sea it rides on.
    pub fn sample(&self, x: f64, z: f64) -> (f64, f64, (f64, f64)) {
        let p = self.waves.source(x, z);
        let (mut clear_f, mut clear_r) = (1., 1.);
        for (n, size, foam) in &self.tiles {
            let [f, r] = bilinear(*n, *size, foam, p.0, p.1);
            clear_f *= 1. - f;
            clear_r *= 1. - r;
        }
        (1. - clear_f, 1. - clear_r, p)
    }

    /// What breaks the foam up, as the shared code takes it (`shared::sea::FoamPattern`, which
    /// the GPU's shader compiles too).
    fn pattern(&self) -> shared::sea::FoamPattern<f64> {
        shared::sea::FoamPattern { wind: (self.wind.x, self.wind.y), speed: self.speed, lace_sd: self.lace_sd, perm: 0 }
    }

    /// Residual foam of strength `r` at `p` broken into lace: covering about a fraction r of
    /// the surface, in patches that shrink and part as it thins, and in a gale drawn out along
    /// the wind.
    fn lace(&self, r: f64, p: (f64, f64)) -> f64 {
        self.pattern().lace(&self.noise, r, p)
    }

    /// A breaking crest's fresh foam of strength `f` at `p`: solid at its core, its edges torn
    /// into tongues and filaments drawn downwind, as the crest spills.
    fn ragged(&self, f: f64, p: (f64, f64)) -> f64 {
        self.pattern().ragged(&self.noise, f, p)
    }

    /// The foam as data, for the GPU's kernel of the water (`pathtrace::FOAM_*` has the
    /// layout): its albedo, the wind's direction and speed, the lace noise's spread and how
    /// many tiles; for each tile its texels a side, its size and where its texels (fresh and
    /// residual foam) begin from the record's start; the noise's permutation; the texels.
    pub fn export(&self) -> Vec<f32> {
        let albedo = self.albedo.as_reflectance().rgb;
        let mut out = vec![
            albedo.x as f32, albedo.y as f32, albedo.z as f32, self.wind.x as f32, self.wind.y as f32,
            self.speed as f32, self.lace_sd as f32, self.tiles.len() as f32,
        ];
        let mut at = out.len() + 3 * self.tiles.len() + 512;
        for (n, size, foam) in &self.tiles {
            out.extend([*n as f32, *size as f32, f32::from_bits(at as u32)]);
            at += 2 * foam.len();
        }
        out.extend(self.noise.permutation().iter().map(|&p| p as f32));
        for (_, _, foam) in &self.tiles {
            out.extend(foam.iter().flatten());
        }
        out
    }

    /// How much of the surface at (x, z) is foam, 0 to 1, and how much of that is fresh.
    pub fn cover(&self, x: f64, z: f64) -> (f64, f64) {
        let (fresh, residual, p) = self.sample(x, z);
        let (fresh, lace) = (self.ragged(fresh, p), self.lace(residual, p));
        let covered = fresh.max(lace);
        (covered, if covered > 0. { fresh / covered } else { 0. })
    }

    /// How much of the surface at (x, z) is foam, 0 to 1.
    pub fn at(&self, x: f64, z: f64) -> f64 {
        self.cover(x, z).0
    }
}

/// Water's refractive index.
pub const WATER_INDEX: f64 = 1.333;

/// Fresnel reflectance of unpolarised light from air onto water, at incidence with cosine `cos_i`.
pub fn fresnel(cos_i: f64) -> f64 {
    let cos_i = cos_i.clamp(0., 1.);
    let sin_t = (1. - cos_i * cos_i).sqrt() / WATER_INDEX;
    let cos_t = (1. - sin_t * sin_t).max(0.).sqrt();
    let rs = (cos_i - WATER_INDEX * cos_t) / (cos_i + WATER_INDEX * cos_t);
    let rp = (WATER_INDEX * cos_i - cos_t) / (WATER_INDEX * cos_i + cos_t);
    (rs * rs + rp * rp) / 2.
}

impl PhysicalOcean {
    /// A microfacet normal about `n`, from the Beckmann distribution of the ripples' slopes.
    fn sample_ripple(&self, n: &Vector3<f64>) -> Vector3<f64> {
        // Slopes are Gaussian, each component with variance roughness / 2.
        let r = (-self.roughness * (1. - geometry::rand()).ln()).sqrt();
        let phi = 2. * std::f64::consts::PI * geometry::rand();
        let t = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::z() };
        let a = (t - n * n.dot(&t)).normalize();
        let b = n.cross(&a);
        (n - a * (r * phi.cos()) - b * (r * phi.sin())).normalize()
    }

    /// Beckmann's distribution of microfacet normals `m` about `n`, per steradian of `m`.
    fn ripple_density(&self, n: &Vector3<f64>, m: &Vector3<f64>) -> f64 {
        let c = n.dot(m);
        if c <= 0. {
            return 0.;
        }
        let tan2 = (1. - c * c) / (c * c);
        (-tan2 / self.roughness).exp() / (std::f64::consts::PI * self.roughness * c.powi(4))
    }
}

impl PhysicalOcean {
    /// The local vertical at `point`, and the normal to shade with seen along `rd`.
    fn shading_frame(&self, rd: &Vector3<f64>, intersection: &Intersection) -> (Vector3<f64>, Vector3<f64>) {
        let up = self.center.map_or(Vector3::y(), |c| (intersection.point - c).normalize());
        // Seen from beyond a wave's slope: use the mean surface.
        let n = if intersection.normal.dot(rd) >= 0. { up } else { intersection.normal };
        (up, n)
    }

    /// The glitter of `sun` (the sun, or moonlight) through the ripples seen along `rd`:
    /// F D / (4 cos θv) per unit irradiance, Smith's shadowing left out (it matters only at
    /// grazing angles).
    fn glint(&self, rd: &Vector3<f64>, intersection: &Intersection, s: &Scene, sun: &crate::light::Sun) -> Color {
        let (up, n) = self.shading_frame(rd, intersection);
        let (v, l) = (-rd.normalize(), sun.direction);
        if l.dot(&up) <= 0. || n.dot(&l) <= 0. {
            return Color::black();
        }
        let h = (l + v).normalize();
        let glint = fresnel(h.dot(&v)) * self.ripple_density(&n, &h) / (4. * n.dot(&v));
        if glint <= 1e-6 {
            return Color::black();
        }
        sun.irradiance_at(&intersection.point) * sun.transmittance_from(&intersection.point, s) * glint
    }
}

/// The sea as media see it, for glitter probes (`VolumeMaterial::glitter_probe`): the sun's
/// glitter lights the clouds above (their bases most), but a path scattering in a cloud finds
/// the glitter only by chance, rarely and brightly. These sample directions towards it: to the
/// points of a flat sea where facets tilted as Cox and Munk's slopes would reflect the sun.
#[derive(Clone, Copy, Debug)]
pub struct Sea {
    /// The mean square slope of the waves and ripples together.
    pub roughness: f64,
    /// The planet's centre, for the local vertical; None for a flat sea.
    pub center: Option<Vector3<f64>>,
}

impl Sea {
    /// From an ocean's scene object.
    pub fn parse(opts: &Value) -> Sea {
        let wind = SceneFile::parse_vec2_def(opts, "wind", Vector2::new(7., 2.));
        let radius = SceneFile::parse_number(&opts["planet_radius"], crate::planet::EARTH_RADIUS);
        Sea {
            roughness: cox_munk_slope(wind.norm()).max(opts["roughness"].as_f64().unwrap_or(0.)),
            center: (radius > 0.).then(|| Vector3::new(0., -radius, 0.)),
        }
    }

    fn up(&self, point: &Vector3<f64>) -> Vector3<f64> {
        self.center.map_or(Vector3::y(), |c| (point - c).normalize())
    }

    /// Beckmann's density of facet normals `m` about `up`, per steradian of `m` (projected).
    fn facets(&self, up: &Vector3<f64>, m: &Vector3<f64>) -> f64 {
        let c = up.dot(m);
        if c <= 0. {
            return 0.;
        }
        let tan2 = (1. - c * c) / (c * c);
        (-tan2 / self.roughness).exp() / (std::f64::consts::PI * self.roughness * c.powi(4))
    }

    /// A direction from `point` towards the sea's glitter of the sun along `sun`: a facet normal
    /// from the slopes' distribution, the direction that facet reflects into the sun's. None if
    /// it would not reach the sea.
    pub fn sample_glitter(&self, point: &Vector3<f64>, sun: &Vector3<f64>, u: (f64, f64)) -> Option<Vector3<f64>> {
        let up = self.up(point);
        let r = (-self.roughness * (1. - u.0).ln()).sqrt();
        let phi = 2. * std::f64::consts::PI * u.1;
        let t = if up.x.abs() < 0.9 { Vector3::x() } else { Vector3::z() };
        let a = (t - up * up.dot(&t)).normalize();
        let b = up.cross(&a);
        let m = (up + a * (r * phi.cos()) + b * (r * phi.sin())).normalize();
        let lm = sun.dot(&m);
        if lm <= 0. {
            return None;
        }
        let d = sun - m * (2. * lm);
        (d.dot(&up) < 0.).then_some(d)
    }

    /// The density of `sample_glitter`'s direction `d` from `point`, per steradian.
    pub fn glitter_pdf(&self, point: &Vector3<f64>, sun: &Vector3<f64>, d: &Vector3<f64>) -> f64 {
        let up = self.up(point);
        if d.dot(&up) >= 0. {
            return 0.;
        }
        let h = sun - d;
        let norm = h.norm();
        if norm < 1e-12 {
            return 0.;
        }
        let m = h / norm;
        let lm = sun.dot(&m);
        if lm <= 0. {
            return 0.;
        }
        self.facets(&up, &m) * up.dot(&m) / (4. * lm)
    }
}

impl PhysicalOcean {
    /// A ray from under transparent water meeting the surface: it leaves by refraction, as
    /// much as the surface transmits, dimmed by the water it crossed. (What the surface
    /// reflects back down, a few percent, or all of it beyond the critical angle, is dropped.)
    fn leave_water(&self, r: &Ray, intersection: &Intersection, absorption: &Color) -> ScatteredRay {
        let d = r.rd.normalize();
        let n = intersection.normal;
        let black = ScatteredRay { emit: Color::black(), light_sampling: LightSampling::Exclusive, attenuate: Color::black(), ray: None };
        let out = match crate::material::functions::refract(d, -n, WATER_INDEX) {
            Some(t) => t.normalize(),
            None => return black,
        };
        let a = absorption.as_unbounded();
        let through = Color { rgb: a.rgb.map(|k| (-k * intersection.dist).exp()) } * (1. - fresnel(out.dot(&n)));
        ScatteredRay {
            emit: Color::black(),
            light_sampling: LightSampling::Exclusive,
            attenuate: through,
            ray: Some(Ray { ro: intersection.point + n * 1e-3, rd: out }),
        }
    }
}

impl MaterialModel for PhysicalOcean {
    fn water(&self) -> Option<crate::material::model::Water> {
        // (Transparent water goes down to what is beneath, which the kernel doesn't; nor does
        // it know the surf.)
        (self.absorption.is_none() && self.surf.is_none()).then(|| crate::material::model::Water {
            body: self.water.as_reflectance(), bubbles: BUBBLES.as_reflectance(), roughness: self.roughness,
            radius: self.center.map_or(0., |c| -c.y), foam: self.foam.as_ref().map(|f| f.export()),
        })
    }

    fn sun_glint(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> Option<Color> {
        Some(s.sun.as_ref().map_or(Color::black(), |sun| self.glint(&r.rd, intersection, s, sun)))
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let v = -r.rd.normalize();
        // A ray from under transparent water, leaving it.
        if let Some(a) = &self.absorption {
            if r.rd.dot(&intersection.normal) > 0. {
                return self.leave_water(r, intersection, a);
            }
        }
        let (up, n) = self.shading_frame(&r.rd, intersection);
        // Whitecaps: foam over that fraction of the point, chosen at random, scattering
        // diffusely about the mean surface (foam rides the wave, smoothing its ripples). Fresh
        // foam is a dense froth; residual foam a thin skin of bubbles over the water, darker.
        let mut bubbles = 0.;
        // The surf: where waves break over the shallows, white water over that fraction of the
        // sea, as fresh foam.
        if let Some(surf) = &self.surf {
            let (white, breaking) = surf.white(&intersection.point);
            if white > 0. && geometry::rand() < white {
                let albedo = self.foam.as_ref().map_or(Color::new(0.6, 0.6, 0.6), |f| f.albedo).as_reflectance();
                let normal = if up.dot(&v) > 0. { up } else { -up };
                return crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal }, r, &intersection.point, s);
            }
            bubbles = breaking;
        }
        if let Some(foam) = &self.foam {
            let (fresh, residual, p) = foam.sample(intersection.point.x, intersection.point.z);
            let (solid, lace) = (foam.ragged(fresh, p), foam.lace(residual, p));
            let covered = solid.max(lace);
            if covered > 0. && geometry::rand() < covered {
                let thin = if solid >= lace { 1. } else { RESIDUAL_FOAM };
                let albedo = foam.albedo.as_reflectance() * thin;
                let normal = if up.dot(&v) > 0. { up } else { -up };
                return crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal }, r, &intersection.point, s);
            }
            // Between the patches, the clouds of bubbles a breaking crest drove down linger
            // under the surface, scattering light back up through the water: paler, greener sea.
            bubbles = bubbles.max(fresh.max(0.5 * residual));
        }
        let into = Intersection { normal: n, ..*intersection };
        // The water body, lit through the surface, as much as gets in.
        let water = self.water.as_reflectance() + BUBBLES.as_reflectance() * bubbles;
        let body = crate::material::functions::direct_lighting(water * (1. - fresnel(n.dot(&v))), &into, s);
        // The glitter of the sun and the moon. The sun's is shared with a glitter probe that
        // might have found it from the medium the ray left (`VolumeMaterial::glitter_probe`).
        let mut glitter = Color::black();
        for (k, light) in s.directional_lights().enumerate() {
            let weight = if k == 0 && s.sun.is_some() { crate::volume::glint_weight_for(r) } else { 1. };
            glitter = glitter + self.glint(&r.rd, intersection, s, light) * weight;
        }
        // Everything else reflected from a sampled ripple, weighted by Fresnel's equations.
        // Only ripples facing the viewer are seen: at grazing angles half the slopes drawn face
        // away, and counting them as black darkens calm water seen from its shore. So draw
        // again until one faces the viewer (sampling the visible slopes, as Heitz's visible-
        // normal sampling does exactly), falling back to the mean surface. Transparent water
        // only; the GPU's kernel matches the opaque sea as it was.
        let m = if self.absorption.is_some() {
            (0..16).map(|_| self.sample_ripple(&n)).find(|m| m.dot(&v) > 0.).unwrap_or(n)
        } else {
            self.sample_ripple(&n)
        };
        let d = r.rd.normalize();
        let reflected = d - m * 2. * m.dot(&d);
        // Seen at a grazing angle, a ripple can send its reflection down towards the water,
        // which reflects it again, upwards (Fresnel's reflectance there is near 1): take that
        // second bounce off the mean surface rather than losing the light, which greys out
        // a river's reflections seen from its bank. For transparent water only, which the GPU
        // doesn't take (its kernel matches the opaque sea's shading as it was).
        let reflected = if self.absorption.is_some() && reflected.dot(&up) <= 0. { reflected - up * 2. * reflected.dot(&up) } else { reflected };
        let reflects = reflected.dot(&up) > 0. && m.dot(&v) > 0.;
        let f = fresnel(m.dot(&v).max(0.));
        let mirror = Ray { ro: intersection.point + up * 1e-3, rd: reflected };
        // Through transparent water, what lies beneath within reach of the light: the ray
        // refracted through the ripple, and the fraction of light the water lets through.
        let beneath = self.absorption.as_ref().and_then(|a| {
            let t = crate::material::functions::refract(d, m, 1. / WATER_INDEX)?.normalize();
            let a = a.as_unbounded();
            let reach = 7. / a.rgb.min().max(1e-3);
            // Under the surface: as far below it as the trace lifted the hit above it (which far
            // from the scene's origin is more than a millimetre).
            let down = Ray { ro: intersection.point - up * crate::material::functions::surface_offset(intersection, s).max(1e-3), rd: t };
            let (object, hit) = s.objects.nearest_raw_intersection(&down, reach, 1e-4)?;
            if std::ptr::eq(object, intersection.object) { return None; }
            Some((down, Color { rgb: a.rgb.map(|k| (-k * hit.dist).exp()) }))
        });
        let (emit, attenuate, ray) = match beneath {
            // Deep water: its own colour, and the reflection.
            None => (glitter + body, Color::white() * if reflects { f } else { 0. }, reflects.then_some(mirror)),
            // The deep water's colour only from the water in front of what is seen; then
            // reflect, or go down to it, by Fresnel's odds.
            Some((down, through)) => {
                let emit = glitter + body * (Color::white() - through);
                if reflects && geometry::rand() < f {
                    (emit, Color::white(), Some(mirror))
                } else {
                    (emit, through * if reflects { 1. } else { 1. - f }, Some(down))
                }
            }
        };
        // The glitter of the sun and moon was sampled above; area lights (lightning) only the
        // water body sampled, so their reflections in the ripples count when this ray hits them.
        ScatteredRay { emit, light_sampling: LightSampling::Directional, attenuate, ray }
    }
}

/// The physical material for an ocean, from its scene object: `water` (what the water body
/// sends back, as an albedo) and the ripples' share of the slope, the rest of Cox and Munk's
/// after the waves'.
fn physical_material(opts: &Value, waves_slope: f64, wind: Vector2<f64>, center: Option<Vector3<f64>>) -> PhysicalOcean {
    PhysicalOcean {
        water: SceneFile::parse_color_def(opts, "water", Color::new(0.002, 0.012, 0.03)),
        roughness: SceneFile::parse_number(&opts["roughness"], (cox_munk_slope(wind.norm()) - waves_slope).max(1e-3)),
        center,
        foam: None,
        surf: None,
        absorption: absorption(opts),
    }
}

/// For `"transparent": true`, the water's absorption per metre (see the module's notes).
fn absorption(opts: &Value) -> Option<Color> {
    (opts["transparent"].as_bool() == Some(true))
        .then(|| SceneFile::parse_color_def(opts, "absorption", Color::new(0.38, 0.065, 0.045)))
}

/// The waves an ocean's scene object describes (deterministic, so built alike wherever needed).
fn waves_of(opts: &Value) -> WaveField {
    WaveField::sea(
        SceneFile::parse_vec2_def(opts, "wind", Vector2::new(7., 2.)),
        SceneFile::parse_number(&opts["time"], 0.),
        SceneFile::parse_int(&opts["seed"], 0) as u64,
        SeaState::parse(opts),
    )
}

/// An ocean from its scene object (see the module docs' scene options).
pub fn create_ocean(opts: &Value) -> SceneObject {
    create_ocean_over(opts, None)
}

/// As `create_ocean`, over `ground` (a landscape before it in the scene): its waves break in the
/// shallows (`Surf`), the depth under each point found by a ray cast straight down from above
/// the sea onto the ground.
pub fn create_ocean_over(opts: &Value, ground: Option<std::sync::Arc<crate::sceneobject::SceneObject>>) -> SceneObject {
    let wind = SceneFile::parse_vec2_def(opts, "wind", Vector2::new(7., 2.));
    let waves = waves_of(opts);
    let radius = SceneFile::parse_number(&opts["planet_radius"], crate::planet::EARTH_RADIUS);
    let planet_radius = (radius > 0.).then_some(radius);
    let material: Box<dyn MaterialModel + Sync + Send> = if opts["debug"].as_bool() == Some(true) {
        Box::new(NormalShade {})
    } else {
        let mut m = physical_material(opts, waves.mean_square_slope, wind, planet_radius.map(|r| Vector3::new(0., -r, 0.)));
        // The foam's own copy of the same waves (the geometry keeps the other).
        let coverage = SceneFile::parse_number(&opts["whitecaps"], whitecap_coverage(wind.norm())).clamp(0., 1.);
        if coverage > 0. {
            m.foam = Some(Foam::new(waves_of(opts), wind, coverage, SceneFile::parse_color_def(opts, "foam_albedo", Color::new(0.6, 0.6, 0.6))));
        }
        if let Some(ground) = ground {
            // The waves' height as they arrive: the open sea's significant height, four times
            // its surface's standard deviation; H_rms = H_s / √2 for Rayleigh-distributed waves.
            let (_, sd) = waves.height_exceeded(0.5);
            let hrms = 4. * sd / 2f64.sqrt();
            // Their period: the spectrum's peak, the fully developed sea's or a younger one's
            // over the fetch (`jonswap_peak`), as deep-water waves: ω² = 2π g / λ.
            let fetch = SceneFile::parse_number(&opts["fetch"], f64::INFINITY);
            let omega = (2. * std::f64::consts::PI * 9.81 / jonswap_peak(wind.norm(), fetch.min(1e12))).sqrt();
            let centre = planet_radius.map(|r| Vector3::new(0., -r, 0.));
            let (above, reach) = (200., 2000.);
            m.surf = Some(Surf { hrms, omega, heading: wind.normalize(), dean_a: crate::terrain::scenery::dean_a(crate::terrain::scenery::BEACH_SAND),
                noise: crate::noise::PerlinNoise::shuffled(0x5afe), depth: std::sync::Arc::new(move |p: &Vector3<f64>| {
                // Sea level under `p`, and the ground below it.
                let (up, level) = match centre {
                    Some(c) => { let u = (p - c).normalize(); (u, c + u * (-c.y)) },
                    None => (Vector3::y(), Vector3::new(p.x, 0., p.z)),
                };
                let ray = Ray { ro: level + up * above, rd: -up };
                let hit = ground.geometry.intersects(&ray)?;
                (hit.dist <= above + reach).then_some(hit.dist - above)
            }) });
        }
        Box::new(m)
    };
    let ocean = Ocean {
        planet_radius,
        waves,
        detail_center: SceneFile::parse_vec2_def(opts, "detail_center", Vector2::zeros()),
        detail_distance: SceneFile::parse_number(&opts["detail_distance"], 300.).max(1.),
        absorption: absorption(opts),
    };
    SceneObject { geometry: Box::new(ocean), medium: Box::new(Solid { m: material }) }
}

/// The surface of the ocean a scene object describes, for floating things on its waves: the
/// same waves as `create_ocean` builds (they are deterministic), shared between the objects
/// that ask for the same ocean.
pub fn surface(opts: &Value) -> std::sync::Arc<Ocean> {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<Ocean>>>> = OnceLock::new();
    let key = opts.to_string();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(o) = cache.lock().unwrap().get(&key) { return o.clone(); }
    let radius = SceneFile::parse_number(&opts["planet_radius"], crate::planet::EARTH_RADIUS);
    let ocean = Arc::new(Ocean {
        planet_radius: (radius > 0.).then_some(radius),
        waves: waves_of(opts),
        detail_center: SceneFile::parse_vec2_def(opts, "detail_center", Vector2::zeros()),
        detail_distance: SceneFile::parse_number(&opts["detail_distance"], 300.).max(1.),
        absorption: absorption(opts),
    });
    cache.lock().unwrap().insert(key, ocean.clone());
    ocean
}

impl Ocean {
    /// The displaced surface's height above the mean at (x, z) (a flat ocean's, or near the
    /// origin a curved one's), and its normal there.
    pub fn surface_at(&self, x: f64, z: f64) -> (f64, Vector3<f64>) {
        let h = self.displaced(x, z) * self.waves.at(x, z).x;
        (h, self.normal_at(&Vector3::new(x, h, z)))
    }
}

#[cfg(test)]
mod curved_tests {
    use super::*;

    fn ocean_with(planet_radius: Option<f64>) -> Ocean {
        Ocean {
            planet_radius,
            waves: WaveField::new(Vector2::new(7., 2.), 0., 1, None, 0.5),
            detail_center: Vector2::zeros(),
            detail_distance: 300.,
            absorption: None,
        }
    }

    /// Transparent water lets light up through it from below, red least; light from above
    /// doesn't pass down through it, and opaque water blocks both.
    #[test]
    fn test_transparent_water_transmits_from_below() {
        let mut ocean = ocean_with(None);
        let up = Ray { ro: Vector3::new(30., -3., 40.), rd: Vector3::new(0.2, 1., 0.1).normalize() };
        let down = Ray { ro: Vector3::new(30., 3., 40.), rd: Vector3::new(0.2, -1., 0.1).normalize() };
        assert_eq!(ocean.transmittance_color(&up, 1e-6, 1e9), Color::black());
        ocean.absorption = Some(Color::new(0.38, 0.065, 0.045));
        let t = ocean.transmittance_color(&up, 1e-6, 1e9);
        assert!(t.rgb.x < 0.5 && t.rgb.y > 0.6 && t.rgb.z > t.rgb.y, "{:?}", t);
        assert_eq!(ocean.transmittance_color(&down, 1e-6, 1e9), Color::black());
        // Deeper, darker.
        let deep = Ray { ro: Vector3::new(30., -20., 40.), ..up };
        assert!(ocean.transmittance_color(&deep, 1e-6, 1e9).rgb.y < t.rgb.y);
    }

    #[test]
    fn test_glitter_density_matches_its_sampling() {
        use rand::{RngExt, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(5);
        let point = Vector3::new(0., 1000., 0.);
        for (roughness, sun) in [(0.04, Vector3::new(0.8, 0.6, 0.)), (0.01, Vector3::new(-0.3, 0.9, 0.3).normalize()), (0.1, Vector3::new(0.97, 0.24, 0.).normalize())] {
            let sea = Sea { roughness, center: None };
            let n = 400000;
            // The share of samples that reach the sea is the density's integral over the sphere.
            let valid = (0..n).filter(|_| sea.sample_glitter(&point, &sun, (rng.random(), rng.random())).is_some()).count() as f64 / n as f64;
            let integral = (0..n).map(|_| {
                let z: f64 = 2. * rng.random::<f64>() - 1.;
                let phi = 2. * f64::consts::PI * rng.random::<f64>();
                let r = (1. - z * z).sqrt();
                sea.glitter_pdf(&point, &sun, &Vector3::new(r * phi.cos(), z, r * phi.sin()))
            }).sum::<f64>() * 4. * f64::consts::PI / n as f64;
            assert!((integral - valid).abs() < 0.03 * valid + 0.01, "roughness {}: integral {} vs {} reaching the sea", roughness, integral, valid);
            // And each sample's direction has the density the pdf gives: E[f / pdf] for f = 1 on a
            // cap about the mirrored sun is the cap's solid angle.
            let mirror = Vector3::new(sun.x, -sun.y, sun.z);
            let cap = 0.97;
            let est = (0..n).filter_map(|_| sea.sample_glitter(&point, &sun, (rng.random(), rng.random())))
                .map(|d| if d.dot(&mirror) > cap { 1. / sea.glitter_pdf(&point, &sun, &d) } else { 0. }).sum::<f64>() / n as f64;
            let solid = 2. * f64::consts::PI * (1. - cap);
            assert!((est - solid).abs() < 0.03 * solid, "roughness {}: cap {} vs {}", roughness, est, solid);
        }
    }

    /// Area lights, such as lightning, are seen reflected in the sea: its glitter samples only
    /// the sun and moon, so the reflected ray must count the lights it hits.
    #[test]
    fn test_sea_reflects_area_lights() {
        let s = crate::scenefile::SceneFile::from_string(r#"{
            "width": 10, "height": 10, "max_depth": 4,
            "camera": { "location": [0, 5, 0], "lookat": [0, -50, 100], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "LIGHT": { "type": "diffuse-light", "intensity": 1, "two_sided": true } },
            "objects": [
                { "type": "ocean", "planet_radius": 0, "wind": [1, 0], "whitecaps": 0, "water": [0, 0, 0] },
                { "type": "sphere", "location": [0, 50, 100], "radius": 15, "material": "LIGHT" }
            ]
        }"#.to_string());
        // Towards the light's mirror image, 61° from the vertical: water reflects some 6%.
        let to_image = Vector3::new(0., -55., 100.).normalize();
        let n = 400;
        let mean = (0..n).map(|_| crate::trace::trace(&Ray { ro: Vector3::new(0., 5., 0.), rd: to_image }, 0, &s).1.rgb.y).sum::<f64>() / n as f64;
        assert!(mean > 0.02 && mean < 0.2, "{}", mean);
    }

    #[test]
    fn test_whitecaps_grow_with_the_wind() {
        // Monahan and O'Muircheartaigh: about 2% at 12 m/s, none in a calm.
        assert!((whitecap_coverage(12.) - 0.0186).abs() < 0.001, "{}", whitecap_coverage(12.));
        assert!(whitecap_coverage(3.) < 1e-3 && whitecap_coverage(0.) == 0.);
        // The foam covers about that fraction of the sea, on the crests.
        let wind = Vector2::new(14., 3.);
        let foam = Foam::new(WaveField::new(wind, 0., 1, None, 0.8), wind, whitecap_coverage(wind.norm()), Color::white());
        let n = 200;
        let covered = (0..n * n).map(|i| foam.at((i % n) as f64 * 2.31, (i / n) as f64 * 2.17)).sum::<f64>() / (n * n) as f64;
        let w = whitecap_coverage(wind.norm());
        assert!((covered - w).abs() < 0.4 * w, "{} vs {}", covered, w);
    }

    #[test]
    fn test_waves_grow_as_they_come_into_the_shallows() {
        let omega = 2. * std::f64::consts::PI / 5.3;
        // The dispersion relation holds; in deep water k = ω² / g, and no change of height.
        let k = wavenumber(omega, 3.);
        assert!((9.81 * k * (k * 3.).tanh() - omega * omega).abs() < 1e-9);
        assert!((wavenumber(omega, 500.) - omega * omega / 9.81).abs() < 1e-9);
        assert!((shoaling(omega, 500.) - 1.).abs() < 1e-6);
        // At intermediate depths a little lower, least about 0.913 (at kh near 1.2).
        let least = (1..400).map(|i| shoaling(omega, 0.1 * i as f64)).fold(f64::MAX, f64::min);
        assert!((least - 0.913).abs() < 0.002, "{}", least);
        // In shallow water, Green's law: c_g → √(g h), so K_s → √(g / (2ω) / √(g h)).
        let h: f64 = 0.05;
        let green = (9.81 / (2. * omega) / (9.81 * h).sqrt()).sqrt();
        assert!((shoaling(omega, h) / green - 1.).abs() < 0.02, "{} vs {}", shoaling(omega, h), green);
    }

    #[test]
    fn test_breakers_line_up_with_the_shore() {
        // A straight beach of Dean's profile, the sea out along +x, waves of 0.84 m rms and
        // 5.3 s coming in obliquely.
        let a = crate::terrain::scenery::dean_a(0.3);
        let surf = |heading: Vector2<f64>| Surf {
            depth: std::sync::Arc::new(move |p: &Vector3<f64>| Some(a * p.x.max(0.).powf(2. / 3.))),
            hrms: 0.84, omega: 2. * std::f64::consts::PI / 5.3, heading: heading.normalize(), dean_a: a,
            noise: crate::noise::PerlinNoise::shuffled(1),
        };
        let s = surf(Vector2::new(-1., 0.4));
        let white = |x: f64, z: f64| s.white(&Vector3::new(x, 0., z)).0;
        // Out where it is deep, none break.
        assert!((0..50).all(|z| white(400., z as f64) == 0.));
        // In the surf zone the white water comes in lines along the shore: alike a metre apart
        // along it, crest and trough across it.
        let mut along = 0.;
        let mut across: f64 = 0.;
        for i in 0..200 {
            let (x, z) = (5. + 0.2 * i as f64, 3. * i as f64);
            along += (white(x, z) - white(x, z + 1.)).abs();
            across = across.max((white(x, z) - white(x + 3., z)).abs());
        }
        assert!(along / 200. < 0.1, "{}", along / 200.);
        assert!(across > 0.5, "{}", across);
        // Where every wave breaks, the foam each leaves fades over FOAM_DECAY before the next
        // comes: white on average (τ / T)(1 - e^(-T / τ)) of the time, about half.
        let (t, tau) = (5.3, FOAM_DECAY);
        let expect = tau / t * (1. - (-t / tau).exp());
        let inner = (0..2000).map(|i| white(1. + 0.003 * i as f64, 7.3 * i as f64)).sum::<f64>() / 2000.;
        assert!((inner - expect).abs() < 0.1, "{} vs {}", inner, expect);
        // Waves heading out to sea (an offshore wind's) don't reach the shore.
        let off = surf(Vector2::new(1., 0.2));
        assert!((0..100).all(|i| off.white(&Vector3::new(2. + 0.3 * i as f64, 0., i as f64)).0 == 0.));
    }

    #[test]
    fn test_waves_break_in_the_shallows() {
        let hrms = 0.78;
        // Out at sea none break; where the waves are as high as the water allows (γ d), all do,
        // and over land.
        assert_eq!(breaking_fraction(hrms, 100.), 0.);
        assert_eq!(breaking_fraction(hrms, hrms / BREAKING_INDEX), 1.);
        assert_eq!(breaking_fraction(hrms, -1.), 1.);
        // More break the shallower it gets.
        let q: Vec<f64> = [3., 2., 1.5, 1.2, 1.05].iter().map(|&d| breaking_fraction(hrms, d)).collect();
        assert!(q.windows(2).all(|w| w[0] < w[1]), "{:?}", q);
        // The fraction satisfies Battjes and Janssen's (1 - Q) / -ln Q = (H_rms / γ d)².
        let (d, q) = (1.5, breaking_fraction(hrms, 1.5));
        assert!(((1. - q) / -q.ln() - (hrms / (BREAKING_INDEX * d)).powi(2)).abs() < 1e-9, "{}", q);
        // Half break where (H_rms / H_m)² = 0.5 / ln 2: 1.178 m deep for these waves.
        let half = hrms / (BREAKING_INDEX * (0.5 / 2f64.ln()).sqrt());
        assert!((breaking_fraction(hrms, half) - 0.5).abs() < 1e-6, "{} at {}", breaking_fraction(hrms, half), half);
    }

    #[test]
    fn test_a_short_fetch_raises_short_waves() {
        // JONSWAP's peak for 10 m/s over 10 km: f_p = 3.5 g/U (gF/U²)^-0.33 = 0.354 Hz, so
        // λ = g / (2π f²) = 12.5 m; over the open ocean it is Pierson-Moskowitz's, 2π U² / (0.877² g).
        assert!((jonswap_peak(10., 10_000.) - 12.5).abs() < 0.2, "{}", jonswap_peak(10., 10_000.));
        let developed = 2. * std::f64::consts::PI * 100. / (0.877f64.powi(2) * 9.81);
        assert!((jonswap_peak(10., 1e9) / developed - 1.).abs() < 1e-9);
        // A lake's waves, 500 m of fetch in a 7 m/s wind: under a metre long and a few
        // centimetres high, as JONSWAP has them, or lower where that would be steeper than the
        // open sea in the same wind (Cox and Munk).
        let wind = Vector2::new(7., 0.);
        let w = WaveField::lake(wind, 500., 0., 1);
        let peak = jonswap_peak(7., 500.);
        assert!(peak > 0.5 && peak < 1.5, "{}", peak);
        let (_, sd) = w.height_exceeded(0.5);
        let hs = significant_wave_height(7., Some(500.));
        assert!(4. * sd < 1.1 * hs && 4. * sd > 0.2 * hs, "{} vs {}", 4. * sd, hs);
        assert!(w.mean_square_slope <= 1.01 * cox_munk_slope(7.), "{}", w.mean_square_slope);
        assert!(w.mean_square_slope > 0.5 * cox_munk_slope(7.), "{}", w.mean_square_slope);
    }

    #[test]
    fn test_wave_height_follows_the_wind() {
        for (u, fetch) in [(7., None), (15., None), (15., Some(20_000.))] {
            let w = WaveField::sea(Vector2::new(u, 2.), 0., 1, SeaState { fetch, ..SeaState::default() });
            let (_, sd) = w.height_exceeded(0.5);
            let hs = significant_wave_height(Vector2::new(u, 2.).norm(), fetch);
            assert!((4. * sd / hs - 1.).abs() < 0.1, "{} m/s: {} vs {}", u, 4. * sd, hs);
        }
        // A short fetch raises a lower sea than the open ocean.
        assert!(significant_wave_height(15., Some(20_000.)) < 0.6 * significant_wave_height(15., None));
    }

    #[test]
    fn test_choppy_crests_are_sharp_and_troughs_broad() {
        // Points crowd towards the crests, so less of the surface stands above the mean, and its
        // steepest slopes are steeper.
        let wind = Vector2::new(14., 3.);
        let above = |w: &WaveField| {
            let n = 300;
            (0..n * n).filter(|i| w.full_height((i % n) as f64 * 1.63, (i / n) as f64 * 1.71) > 0.).count() as f64 / (n * n) as f64
        };
        let smooth = WaveField::sea(wind, 0., 1, SeaState { choppiness: 0., ..SeaState::default() });
        let choppy = WaveField::sea(wind, 0., 1, SeaState::default());
        assert!(above(&choppy) < above(&smooth) - 0.02, "{} vs {}", above(&choppy), above(&smooth));
        let steepest = |w: &WaveField| {
            let n = 300;
            let mut s: Vec<f64> = (0..n * n).map(|i| {
                let a = w.at((i % n) as f64 * 1.63, (i / n) as f64 * 1.71);
                (a.y * a.y + a.z * a.z).sqrt()
            }).collect();
            s.sort_by(|a, b| a.total_cmp(b));
            s[s.len() * 999 / 1000]
        };
        assert!(steepest(&choppy) > steepest(&smooth), "{} vs {}", steepest(&choppy), steepest(&smooth));
        // Some crests are squeezed well together, towards breaking.
        let squeezed = (0..200 * 200).map(|i| choppy.squeeze(choppy.source((i % 200) as f64 * 2.4, (i / 200) as f64 * 2.4)).0).fold(1., f64::min);
        assert!(squeezed < 0.7, "{}", squeezed);
        assert!(choppy.max_slope >= steepest(&choppy));
        // The map is inverted: the point found maps back to where it was asked for.
        let (x, z) = (37.2, 81.9);
        let p = choppy.source(x, z);
        let (mut dx, mut dz) = (0., 0.);
        for t in &choppy.tiles[..DISPLACED] {
            let c = t.chop_at(p.0, p.1);
            dx += c[0];
            dz += c[1];
        }
        let back = (p.0 + choppy.lambda * dx, p.1 + choppy.lambda * dz);
        assert!((back.0 - x).abs() < 0.05 && (back.1 - z).abs() < 0.05, "{:?}", back);
    }

    #[test]
    fn test_foam_breaks_on_crests_and_trails_behind() {
        let wind = Vector2::new(16., 3.);
        let waves = WaveField::sea(wind, 0., 1, SeaState::default());
        let w = whitecap_coverage(wind.norm());
        let foam = Foam::new(WaveField::sea(wind, 0., 1, SeaState::default()), wind, w, Color::white());
        let n = 300;
        let (mut fresh, mut residual, mut crest, mut count) = (0., 0., 0., 0.);
        let (_, sd) = waves.height_exceeded(0.5);
        for i in 0..n * n {
            let (x, z) = ((i % n) as f64 * 1.63, (i / n) as f64 * 1.71);
            let (f, r, _) = foam.sample(x, z);
            fresh += f;
            residual += r;
            if f > 0.5 {
                crest += waves.full_height(x, z) / sd;
                count += 1.;
            }
        }
        // Breaking crests stand above the mean; foam they left lingers over more of the sea than
        // is breaking now.
        assert!(count > 0. && crest / count > 0.3, "{}", crest / count.max(1.));
        assert!(residual > 1.5 * fresh, "{} vs {}", residual, fresh);
    }

    #[test]
    fn test_fresnel() {
        assert!((fresnel(1.) - 0.0204).abs() < 1e-3);
        assert!(fresnel(0.01) > 0.9);
    }

    #[test]
    fn test_waves_match_cox_munk() {
        let w = WaveField::new(Vector2::new(7., 2.), 0., 1, None, 0.5);
        let total = cox_munk_slope(Vector2::new(7., 2.).norm());
        assert!((w.mean_square_slope - 0.5 * total).abs() < 0.02 * total, "{} vs {}", w.mean_square_slope, total);
        // A moderate breeze's waves: heights of a metre or so at most.
        assert!(w.max_height > 0.2 && w.max_height < 5., "{}", w.max_height);
    }

    #[test]
    fn test_rays_hit_the_waves_near_and_the_smooth_surface_far() {
        for radius in [Some(crate::planet::EARTH_RADIUS), None] {
            rays_hit_the_waves(&ocean_with(radius));
        }
    }

    #[test]
    fn test_radius_changes_only_the_shape() {
        // Near the origin, curved and flat agree: the same waves, the same normals.
        let (curved, flat) = (ocean_with(Some(crate::planet::EARTH_RADIUS)), ocean_with(None));
        for (x, z) in [(3., 4.), (-20., 51.)] {
            let r = Ray { ro: Vector3::new(x, 20., z), rd: Vector3::new(0., -1., 0.) };
            let (a, b) = (curved.intersects(&r).unwrap(), flat.intersects(&r).unwrap());
            assert!((a.point - b.point).norm() < 1e-3, "{} vs {}", a.point, b.point);
            assert!((a.normal - b.normal).norm() < 1e-4);
        }
        // Far off, the sphere drops away and the plane doesn't.
        let r = Ray { ro: Vector3::new(100_000., 1000., 0.), rd: Vector3::new(0., -1., 0.) };
        let drop = 100_000f64.powi(2) / (2. * crate::planet::EARTH_RADIUS);
        assert!((curved.intersects(&r).unwrap().dist - (1000. + drop)).abs() < 1.);
        assert!((flat.intersects(&r).unwrap().dist - 1000.).abs() < 1e-6);
    }

    fn rays_hit_the_waves(o: &Ocean) {
        // Straight down near the centre: on the displaced surface.
        for (x, z) in [(3., 4.), (-20., 51.), (100., -7.)] {
            let r = Ray { ro: Vector3::new(x, 20., z), rd: Vector3::new(0., -1., 0.) };
            let hit = o.intersects(&r).unwrap();
            assert!(o.above(&hit.point).abs() < 1e-3, "{}", o.above(&hit.point));
            let expected = o.waves.at(x, z).x;
            assert!((hit.point.y - expected).abs() < 0.01, "{} vs {}", hit.point.y, expected);
        }
        // Grazing from a camera 2 m up: hits the surface, not before it rises to meet the ray.
        let ro = Vector3::new(0., 2., 0.);
        for k in 0..50 {
            let d = Vector3::new((k as f64 * 0.37).sin() * 0.3, -0.004 - 0.002 * k as f64, 1.).normalize();
            let r = Ray { ro, rd: d };
            let hit = o.intersects(&r).unwrap();
            assert!(o.above(&hit.point).abs() < 1e-3);
            // Nothing above water short of the hit.
            for i in 1..200 {
                let p = ro + d * (hit.dist * i as f64 / 200.);
                assert!(o.above(&p) > -1e-6, "{} at {}", o.above(&p), i);
            }
        }
        // Far beyond the detail: the smooth surface.
        let r = Ray { ro: Vector3::new(5000., 50., 0.), rd: Vector3::new(0., -1., 0.) };
        let hit = o.intersects(&r).unwrap();
        assert!(o.altitude(&hit.point).abs() < 1e-3);
        // From under the water, straight up and slanting: through the surface from below.
        for (x, z) in [(3., 4.), (-20., 51.)] {
            for d in [Vector3::new(0., 1., 0.), Vector3::new(0.3, 1., -0.2).normalize()] {
                let r = Ray { ro: Vector3::new(x, -10., z), rd: d };
                let hit = o.intersects(&r).unwrap();
                assert!(o.above(&hit.point).abs() < 1e-3, "{}", o.above(&hit.point));
                for i in 1..100 {
                    assert!(o.above(&(r.ro + d * (hit.dist * i as f64 / 100.))) < 1e-6);
                }
            }
        }
        // Up into the sky: nothing.
        assert!(o.intersects(&Ray { ro, rd: Vector3::new(0., 1., 1.).normalize() }).is_none());
    }
}

#[cfg(test)]
mod probe {
    #[test]
    #[ignore]
    fn probe_sea_state() {
        for u in [3., 7., 12., 18., 25.] {
            let w = super::WaveField::sea(crate::na::Vector2::new(u, 0.), 0., 1, super::SeaState::default());
            let (_, sd) = w.height_exceeded(0.5);
            println!("U {:4} Hs {:6.2} m (PM {:6.2})  mss {:.4} (CM {:.4}) max h {:.2}", u, 4. * sd, 0.21 * u * u / 9.81, w.mean_square_slope, super::cox_munk_slope(u), w.max_height);
        }
    }
}
