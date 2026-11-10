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
//! `create_tile`, but neither is used yet: normals come from the triangles, and the displacement
//! is switched off (`choppyness_shift`), with λ fixed at 1.

extern crate rand as _rand;
extern crate image;
extern crate rustfft;

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
use crate::octree::Octree;
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

fn vertex_at(z: usize, x: usize, vertices: &[Vector3<f64>], fourier_grid_size: usize) -> Vector3<f64>{
    vertices[(z % fourier_grid_size) * fourier_grid_size + (x % fourier_grid_size)]
}

fn make_square_for(x: usize, z: usize, fourier_grid_size: usize, vertices: &[Vector3<f64>]) -> (Triangle, Triangle) {
    (
        Triangle::new(
            vertex_at(z + 1, x + 1, vertices, fourier_grid_size),
            vertex_at(z, x + 1, vertices, fourier_grid_size),
            vertex_at(z + 1, x, vertices, fourier_grid_size),
        ),
        Triangle::new(
            vertex_at(z + 1, x, vertices, fourier_grid_size),
            vertex_at(z, x + 1, vertices, fourier_grid_size),
            vertex_at(z, x, vertices, fourier_grid_size),
        )
    )
}

// Type definition for complex height field data
type OceanComplexData = (
    Vec<Complex<f64>>, // ht
    Vec<Complex<f64>>, // ht_slope_x
    Vec<Complex<f64>>, // ht_slope_z
    Vec<Complex<f64>>, // ht_slope_dx
    Vec<Complex<f64>>, // ht_slope_dz
);

/// The surface at time t, from the amplitudes h̃0: heights h(x, t) (eqs. 43, 36), slopes ε(x, t)
/// (eq. 37) and choppy displacements D(x, t) (eq. 44), each on the N x N grid.
pub fn create_tile(
        fourier_grid_size:usize,
        lx: f64,
        lz: f64,
        gravity: f64,
        time: f64,
        h0: Vec<Complex<f64>>) 
        -> OceanComplexData {

    // Tile of amplitudes
    let mut ht =          vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_x =  vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_z =  vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dx = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dz = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];

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
            ht_slope_x[ind] = ht[ind] * Complex::new(0., k.x); 
            ht_slope_z[ind] = ht[ind] * Complex::new(0., k.y); 

            // -i (k/k) h̃ (eq. 44)
            let len = k.norm();
            if len > 0.00001 {
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
    ht = sum(ht);
    ht_slope_x = sum(ht_slope_x);
    ht_slope_z = sum(ht_slope_z);
    ht_slope_dx = sum(ht_slope_dx);
    ht_slope_dz = sum(ht_slope_dz);

    (
        ht,
        ht_slope_x,
        ht_slope_z,
        ht_slope_dx,
        ht_slope_dz,
    )
}


/// Generate a mesh from a list of vertices
fn generate_mesh(vertices: Vec<Vector3<f64>>, size: usize, lx: f64, lz: f64) -> Vec<Arc<Triangle>> {
    // Generate Mesh
    let mut triangles = Vec::new();
    for x in 0 .. size - 1 {
        for z in 0 .. size - 1 {
            let (t0, t1) = make_square_for(x, z, size, &vertices);
            triangles.push(Arc::new(t0));
            triangles.push(Arc::new(t1));

            if x == size - 2 {
                // Need to add the tile from end -> end + 1
                let (mut t0, mut t1) = make_square_for(x+1, z, size, &vertices);
                t0.v0.x += lx;
                t0.v1.x += lx;
                t1.v1.x += lx;
                triangles.push(Arc::new(Triangle::new(t0.v0, t0.v1, t0.v2))); // Recalc normal
                triangles.push(Arc::new(Triangle::new(t1.v0, t1.v1, t1.v2)));
            }
            if z == size - 2 {
                // Need to add the tile from end -> end + 1
                let (mut t0, mut t1) = make_square_for(x, z+1, size, &vertices);
                t0.v0.z += lz;
                t0.v2.z += lz;
                t1.v0.z += lz;
                triangles.push(Arc::new(Triangle::new(t0.v0, t0.v1, t0.v2))); // Recalc normal
                triangles.push(Arc::new(Triangle::new(t1.v0, t1.v1, t1.v2)));
            }

            if z == size - 2 && x == size - 2 {
                let (mut t0, mut t1) = make_square_for(x+1, z+1, size, &vertices);
                t0.v0.x += lx;
                t0.v1.x += lx;
                t1.v1.x += lx;
                t0.v0.z += lz;
                t0.v2.z += lz;
                t1.v0.z += lz;
                triangles.push(Arc::new(Triangle::new(t0.v0, t0.v1, t0.v2))); // Recalc normal
                triangles.push(Arc::new(Triangle::new(t1.v0, t1.v1, t1.v2)));
            }
        }
    }
    triangles
}


pub struct OceanGeometry {
    mesh: RepeatingMesh,
}

impl OceanGeometry {
    pub fn new(o: &Value) -> OceanGeometry {
        let gravity = SceneFile::parse_number(&o["gravity"], 9.81f64);
        let wind = SceneFile::parse_vec2_def(o, "wind", Vector2::new(40., 30.));
        let time = SceneFile::parse_number(&o["time"],4f64);

        // Patch size in metres, Lx = Lz (eq. 36)
        let lx = SceneFile::parse_number(&o["resolution"], 100.);
        let lz = lx;
        let choppyness_shift = false;

        // The random realisation of the spectrum (4.4). Seed 0, the default, is the realisation
        // every ocean used before seeds were configurable.
        let mut seed = [0u8; 32];
        seed[..8].copy_from_slice(&(SceneFile::parse_int(&o["seed"], 0) as u64).to_le_bytes());
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // N = M, the size of the Fourier grid (4.4): a power of two from 16 to 2048. Titanic and
        // Waterworld used 2048, about 3 cm between grid points; above 2048 floating point
        // accuracy becomes noticeable.
        let fourier_grid_size = SceneFile::parse_number(&o["fourier_size"], 128.) as usize;

        // The Phillips constant A (eq. 40). Heights are independent of fourier_size; the default
        // matches the look of the former default (amplitude 1 at fourier_size 128).
        let scale = SceneFile::parse_number(&o["amplitude"], 1. / (128. * 128.)); // (A)

        let h0 = create_amplitude_tile(wind, scale, gravity, lx, lz, fourier_grid_size, &mut rng);
        let(
            ht,
            _ht_slope_x,
            _ht_slope_z,
            ht_slope_dx,
            ht_slope_dz,
        ) = create_tile(fourier_grid_size, lx, lz, gravity, time, h0); 

        /*
		// DEBUG IMAGE
        let img = image::ImageBuffer::from_fn(fourier_grid_size as u32, fourier_grid_size as u32, |x, y| {
            let yscale = fourier_grid_size as f64 / lx; 
            let val = (ht[(x * fourier_grid_size as u32 + y) as usize].re * yscale * 0.5 + 0.5).max(0.);
            let (r,g,b) = Color::new(val, val, val).to_u8();
            image::Rgb([r,g,b])
        });
        let _ = img.save(&Path::new("debug-ocean.png"));
        */
        let mut vertices = vec![Vector3::new(0., 0., 0.); fourier_grid_size * fourier_grid_size];

        for x in 0 .. fourier_grid_size {
            for z in 0 .. fourier_grid_size {
                let ind = z * fourier_grid_size + x;
                // The grid's frequencies start at -N/2, not 0; by the shift theorem the transform
                // comes out multiplied by (-1)^(x+z), undone here (see the module docs).
                let mut sign = 1.;
                if (x + z) % 2 == 0 {
                    sign =  -1.
                }

                let x0 = x as f64 / fourier_grid_size as f64 * lx;
                let z0 = z as f64 / fourier_grid_size as f64 * lz;
                if  x > 0 &&
                    x < fourier_grid_size - 1 &&
                    z > 0 &&
                    z < fourier_grid_size - 1
                    && choppyness_shift 
                    {
                    vertices[ind] = Vector3::new(
                        x0 + ht_slope_dx[ind].re * sign,
                        ht[ind].re * sign,
                        z0 + ht_slope_dz[ind].re * sign,
                    );
                } else { // Dont shift edge vertices
                    vertices[ind] = Vector3::new(
                        x0,
                        ht[ind].re * sign,
                        z0,
                );
                
                }
            }
        }

        let triangles = generate_mesh(vertices, fourier_grid_size, lx, lz);
        let bounds = OceanGeometry::bounds_of(&triangles);
        let tree = Octree::new(8, bounds, &triangles); 

        println!(" - OCEAN [A={}, g={}, W={}, t={}, N={}, {}]", scale, gravity, wind, time, lx, fourier_grid_size);
        println!("  bounded:{}, triangles:{} ", bounds, triangles.len());

        OceanGeometry {
            mesh: RepeatingMesh {
                tile: tree,
                tile_size: Vector3::new(lx, 0., lz),
                tile_bounds: bounds,
                triangle_count: triangles.len(),
            }
		}
    }

    fn bounds_of(triangles: &Vec<Arc<Triangle>>) -> BBox {
        let mut bb = BBox::empty();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        bb
    }
}

impl Geometry for OceanGeometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.mesh.intersects(r)
    }

    fn bounds(&self) -> BBox {
        self.mesh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.mesh.triangle_count as u64
    }
}

struct OceanMaterial {
    deep_color: Color,

}


impl OceanMaterial {
    pub fn new(o: &Value) -> OceanMaterial {
    
        let deep = SceneFile::parse_color_def(o, "color", Color::new(0., 0.2, 0.3));
        OceanMaterial {
            deep_color: deep
        }
    }

}

/// Simplified dielectric with no refraction
impl MaterialModel for OceanMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
		let ni_over_nt = 1. / 1.31;
		let drn = r.rd.dot(&intersection.normal);
		let cosine = - drn / r.rd.norm();

		match refract(r.rd, intersection.normal, ni_over_nt) {
			Some(refracted) => {
				// refracted ray exists
				// Schlick approximation of fresnel amount
				let reflect_prob = schlick(cosine, 1.31);
				if geometry::rand() >= reflect_prob {
                    // Rather than refract, shade darker depending whether the ray is deeper than
                    // 90degrees down
                    let deep_angle = 1. / (refracted.dot(&Vector3::new(0., 1., 0.)).acos() * 0.8);

					return ScatteredRay{ sampled_lights: false, emit: Color::black(),
						attenuate: self.deep_color * deep_angle,
						ray: None, // Don't try and refract
					};
				}
			},
			None => {
				// refracted ray does not exist
				//  - total internal reflection
			}
		}

		let reflected = reflect(r.rd, intersection.normal);
		ScatteredRay { sampled_lights: false, emit: Color::black(),
			attenuate: Color::white(),
			ray: Some(Ray {
				ro: intersection.point,
				rd: reflected
			}) 
		}
	}
}

pub fn create_ocean(opts: &Value) -> SceneObject {
	let o = OceanGeometry::new(opts);
    let mut m: Box<dyn MaterialModel + Sync + Send> = Box::new(OceanMaterial::new(opts));
    if opts["debug"].as_bool().unwrap_or(false) {
        m = Box::new(NormalShade {});
    }
	SceneObject {
		geometry: Box::new(o),
		medium: Box::new(Solid { m }),
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
		create_tile(n, L, L, 9.81, time, h0).0
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
