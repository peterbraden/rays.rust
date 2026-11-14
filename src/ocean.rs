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
//! `create_tile`. The ocean's normals come from the slopes; the choppy displacement isn't used.
//!
//! ## Scene options
//!
//! `{"type": "ocean", "wind": [7, 2], "planet_radius": 6360000, "detail_center": [x, z]}`: the
//! sea (`Ocean`), lit physically (`PhysicalOcean`). By default it is the planet's surface, a
//! sphere of the Earth's radius; `"planet_radius": 0` makes it the flat plane y = 0. The radius
//! changes only its shape. The waves are for a wind of `wind` m/s ([x, z]), scaled so their
//! slopes carry `resolved` (default 0.8) of Cox and Munk's for the wind, unless `amplitude`
//! (Phillips's constant) is given, at `time` s with `seed`; they are displaced within
//! `detail_distance` (default 300 m) of `detail_center` (put it under the camera). `water` is
//! the colour the water body sends back, as an albedo, and `roughness` the mean square slope of
//! the ripples (by default the rest of Cox and Munk's). `"debug": true` shades by normal.

extern crate rand as _rand;
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
use rand::distributions::{Normal, Distribution};
use crate::ocean::_rand::Rng;
use std::path::Path;
use crate::color::Color;
use rustfft::algorithm::Radix4;
use rustfft::FFT;
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

/// A standard Gaussian draw: ξr or ξi in eq. 42.
fn randn(rng: &mut StdRng) -> f64 {
    let normal = Normal::new(0.0, 1.0);
    normal.sample(rng)
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
    let ifft = Radix4::new(size, false);
    let mut tile_clone = tile.clone();
    let mut fft = vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut tile_clone[..], &mut fft[..]);
    let mut conj = transpose(&fft, size);
    let mut out =  vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut conj, &mut out[..]);
    transpose(&out, size)
}

/// Inverse 2D Fast Fourier Transform
pub(crate) fn ifft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let ifft = Radix4::new(size, true);
    let mut tile_clone = tile.clone();
    let mut fft = vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut tile_clone[..], &mut fft[..]);
	fft = fft.iter().map(|x| x.unscale(size as f64)).collect();
    let mut conj = transpose(&fft, size);
    let mut out =  vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut conj[..], &mut out[..]);
	out = out.iter().map(|x| x.unscale(size as f64)).collect();
    transpose(&out, size)
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

/// The sea's heights and slopes (∂h/∂x, ∂h/∂z) over a periodic tile `size` metres across.
struct WaveTile {
    n: usize,
    size: f64,
    texels: Vec<[f32; 3]>,
}

impl WaveTile {
    /// The waves of the Phillips spectrum for `wind` with wavenumbers from `k_min` up to
    /// `k_max`, on an `n` x `n` grid, at `time`.
    fn new(wind: Vector2<f64>, size: f64, n: usize, k_min: f64, k_max: f64, time: f64, seed: u64) -> WaveTile {
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
        let tile = create_tile(n, size, size, 9.81, time, false, true, h0);
        let (h, (sx, sz)) = (tile.heights, tile.slopes.unwrap());
        // Undo the centred spectrum's (-1)^(x+z) (see the module docs).
        let texels = (0..n * n).map(|ind| {
            let sign = if (ind % n + ind / n).is_multiple_of(2) { -1. } else { 1. };
            [(h[ind].re * sign) as f32, (sx[ind].re * sign) as f32, (sz[ind].re * sign) as f32]
        }).collect();
        WaveTile { n, size, texels }
    }

    fn scale(&mut self, k: f64) {
        for t in self.texels.iter_mut() {
            for v in t.iter_mut() {
                *v *= k as f32;
            }
        }
    }

    /// Height and slopes at (x, z), interpolated bilinearly.
    fn at(&self, x: f64, z: f64) -> Vector3<f64> {
        let n = self.n as i64;
        let (u, v) = (x / self.size * self.n as f64, z / self.size * self.n as f64);
        let (u0, v0) = (u.floor(), v.floor());
        let (fu, fv) = (u - u0, v - v0);
        let t = |du: i64, dv: i64| {
            let (i, j) = ((u0 as i64 + du).rem_euclid(n), (v0 as i64 + dv).rem_euclid(n));
            let s = self.texels[(j * n + i) as usize];
            Vector3::new(s[0] as f64, s[1] as f64, s[2] as f64)
        };
        (t(0, 0) * (1. - fu) + t(1, 0) * fu) * (1. - fv) + (t(0, 1) * (1. - fu) + t(1, 1) * fu) * fv
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

/// The sea's surface as the sum of three tiles of incommensurate sizes, so that the pattern
/// repeats only far away: swell and wind waves on one, on the next the shorter waves it has no
/// room for, down to a metre, and on the last wavelets down to 12 cm, the chop that gives water
/// its fine texture. The wavelets are too low to matter to the surface's shape, and only tilt
/// its normals.
/// How many of the wave tiles displace the surface; the rest only tilt its normals.
const DISPLACED: usize = 2;

pub struct WaveField {
    tiles: Vec<WaveTile>,
    /// Bounds on the height and the slope of the displaced surface.
    pub max_height: f64,
    pub max_slope: f64,
    /// The mean square slope of the waves (as interpolated).
    pub mean_square_slope: f64,
}

impl WaveField {
    /// The waves for `wind` (m/s) at `time`. The Phillips spectrum's constant is arbitrary; with
    /// `amplitude` it is that, and otherwise the waves are scaled so their mean square slope is
    /// `resolved` of Cox and Munk's for the wind.
    pub fn new(wind: Vector2<f64>, time: f64, seed: u64, amplitude: Option<f64>, resolved: f64) -> WaveField {
        let (big, small, tiny, n) = (487., 61.3, 7.9, 256);
        // Each tile takes over just short of the previous one's shortest waves.
        let k_split = std::f64::consts::PI * n as f64 / big * 0.9;
        let k_tiny = std::f64::consts::PI * (n / 2) as f64 / small * 0.9;
        let mut tiles = vec![
            WaveTile::new(wind, big, n, 0., k_split, time, seed),
            WaveTile::new(wind, small, n / 2, k_split, k_tiny, time, seed.wrapping_add(1)),
            WaveTile::new(wind, tiny, n / 2, k_tiny, f64::INFINITY, time, seed.wrapping_add(2)),
        ];
        // Each wave's amplitude should be √P(k) Δk, but `create_amplitude_tile` leaves out the
        // grid's spacing Δk = 2π/L, which is larger on a smaller tile: put it back, relative to
        // the largest, or the shorter waves are far too weak.
        for t in tiles.iter_mut().skip(1) {
            t.scale(big / t.size);
        }
        let mut field = WaveField { tiles: vec![], max_height: 0., max_slope: 0., mean_square_slope: 0. };
        let k = match amplitude {
            Some(a) => a.sqrt(),
            None => {
                field.tiles = std::mem::take(&mut tiles);
                let mss = field.measure_slope();
                tiles = std::mem::take(&mut field.tiles);
                if mss > 0. { (resolved * cox_munk_slope(wind.norm()) / mss).sqrt() } else { 0. }
            },
        };
        for t in tiles.iter_mut() {
            t.scale(k);
        }
        let (h, s) = tiles[..DISPLACED].iter().map(|t| t.maxima()).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        field.tiles = tiles;
        field.max_height = h;
        field.max_slope = s;
        field.mean_square_slope = field.measure_slope();
        field
    }

    /// Height (of the displaced tiles) and slopes (of all) at horizontal position (x, z).
    pub fn at(&self, x: f64, z: f64) -> Vector3<f64> {
        let mut w = Vector3::zeros();
        for (i, t) in self.tiles.iter().enumerate() {
            let v = t.at(x, z);
            w += if i < DISPLACED { v } else { Vector3::new(0., v.y, v.z) };
        }
        w
    }

    /// The mean square slope as sampled: between texels, interpolation smooths the shortest
    /// waves, so it is measured on a grid finer than the tiles'.
    fn measure_slope(&self) -> f64 {
        let (m, size) = (1024, 487.);
        let step = size / m as f64;
        let mut sum = 0.;
        for j in 0..m {
            for i in 0..m {
                let s = self.at((i as f64 + 0.37) * step, (j as f64 + 0.61) * step);
                sum += s.y * s.y + s.z * s.z;
            }
        }
        sum / (m * m) as f64
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
                // Crossed: bisect.
                let (mut lo, mut hi) = (t, next);
                for _ in 0..24 {
                    let mid = 0.5 * (lo + hi);
                    if side * self.above(&(r.ro + r.rd * mid)) > 0. { lo = mid } else { hi = mid }
                }
                return Some(hi);
            }
            t = next;
            f = g;
        }
        None
    }
}

impl Geometry for Ocean {
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
}

/// Water's refractive index.
const WATER_INDEX: f64 = 1.333;

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

impl MaterialModel for PhysicalOcean {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let v = -r.rd.normalize();
        let up = self.center.map_or(Vector3::y(), |c| (intersection.point - c).normalize());
        let mut n = intersection.normal;
        // Seen from beyond a wave's slope: use the mean surface.
        if n.dot(&v) <= 0. {
            n = up;
        }
        let into = Intersection { normal: n, ..*intersection };
        // The water body, lit through the surface, as much as gets in.
        let mut emit = crate::material::functions::direct_lighting(self.water.as_reflectance() * (1. - fresnel(n.dot(&v))), &into, s);
        // The sun's glitter through the ripples: F D / (4 cos θv) per unit irradiance, Smith's
        // shadowing left out (it matters only at grazing angles).
        if let Some(sun) = &s.sun {
            let l = sun.direction;
            if l.dot(&up) > 0. && n.dot(&l) > 0. {
                let h = (l + v).normalize();
                let glint = fresnel(h.dot(&v)) * self.ripple_density(&n, &h) / (4. * n.dot(&v));
                if glint > 1e-6 {
                    let lit = sun.transmittance_from(&intersection.point, s);
                    emit = emit + sun.irradiance_at(&intersection.point) * (glint * lit);
                }
            }
        }
        // Everything else reflected from a sampled ripple, weighted by Fresnel's equations.
        let m = self.sample_ripple(&n);
        let d = r.rd.normalize();
        let reflected = d - m * 2. * m.dot(&d);
        let (attenuate, ray) = if reflected.dot(&up) > 0. && m.dot(&v) > 0. {
            (Color::white() * fresnel(m.dot(&v)), Some(Ray { ro: intersection.point + up * 1e-3, rd: reflected }))
        } else {
            (Color::black(), None)
        };
        ScatteredRay { emit, light_sampling: LightSampling::Exclusive, attenuate, ray }
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
    }
}

/// An ocean from its scene object (see the module docs' scene options).
pub fn create_ocean(opts: &Value) -> SceneObject {
    let wind = SceneFile::parse_vec2_def(opts, "wind", Vector2::new(7., 2.));
    let waves = WaveField::new(
        wind,
        SceneFile::parse_number(&opts["time"], 0.),
        SceneFile::parse_int(&opts["seed"], 0) as u64,
        opts["amplitude"].as_f64(),
        SceneFile::parse_number(&opts["resolved"], 0.8).clamp(0., 1.),
    );
    let radius = SceneFile::parse_number(&opts["planet_radius"], crate::planet::EARTH_RADIUS);
    let planet_radius = (radius > 0.).then_some(radius);
    let material: Box<dyn MaterialModel + Sync + Send> = if opts["debug"].as_bool() == Some(true) {
        Box::new(NormalShade {})
    } else {
        Box::new(physical_material(opts, waves.mean_square_slope, wind, planet_radius.map(|r| Vector3::new(0., -r, 0.))))
    };
    let ocean = Ocean {
        planet_radius,
        waves,
        detail_center: SceneFile::parse_vec2_def(opts, "detail_center", Vector2::zeros()),
        detail_distance: SceneFile::parse_number(&opts["detail_distance"], 300.).max(1.),
    };
    SceneObject { geometry: Box::new(ocean), medium: Box::new(Solid { m: material }) }
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
        }
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
