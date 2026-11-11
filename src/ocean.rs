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
//! `create_tile`. The flat ocean's normals come from its triangles, and the displacement is
//! switched off (`choppyness_shift`), with λ fixed at 1; the curved ocean uses the slopes.
//!
//! ## Scene options
//!
//! `{"type": "ocean", "resolution": 100, "fourier_size": 128, "amplitude": A, "wind": [x, z],
//! "time": 4, "seed": 0, "color": colour}`: a flat, tiled patch of displaced waves, coloured
//! `color` where light enters it; `"physical": true` lights it instead (`PhysicalOcean`).
//!
//! `{"type": "ocean", "curved": true, "wind": [7, 2], "detail_center": [x, z]}`: the sea as the
//! planet's surface (`CurvedOcean`), physically lit. Its waves are scaled so their slopes carry
//! `resolved` (default 0.5) of Cox and Munk's for the wind, unless `amplitude` is given, and
//! are displaced within `detail_distance` (default 300 m) of `detail_center` (put it under the
//! camera); `water` is the colour the water body sends back, as an albedo, and `roughness` the
//! mean square slope of the ripples (by default the rest of Cox and Munk's).

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
        let tile = create_tile(fourier_grid_size, lx, lz, gravity, time, choppyness_shift, false, h0);
        let ht = tile.heights;

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
                    let (dx, dz) = tile.displacement.as_ref().unwrap();
                    vertices[ind] = Vector3::new(
                        x0 + dx[ind].re * sign,
                        ht[ind].re * sign,
                        z0 + dz[ind].re * sign,
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
        // Each triangle with its own corners: the tile's last row and column are copies of its
        // first, moved by a tile's width, so not all corners are shared.
        let tile = Mesh::new(
            triangles.iter().flat_map(|t| [t.v0, t.v1, t.v2]).collect(),
            (0 .. triangles.len()).map(|i| [3 * i, 3 * i + 1, 3 * i + 2]).collect(),
        );

        println!(" - OCEAN [A={}, g={}, W={}, t={}, N={}, {}]", scale, gravity, wind, time, lx, fourier_grid_size);
        println!("  bounded:{}, triangles:{} ", bounds, triangles.len());

        OceanGeometry {
            mesh: RepeatingMesh {
                tile,
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
    if opts["curved"].as_bool() == Some(true) {
        return create_curved_ocean(opts);
    }
	let o = OceanGeometry::new(opts);
    let mut m: Box<dyn MaterialModel + Sync + Send> = Box::new(OceanMaterial::new(opts));
    if opts["physical"].as_bool() == Some(true) {
        let wind = SceneFile::parse_vec2_def(opts, "wind", Vector2::new(40., 30.));
        m = Box::new(physical_material(opts, 0., wind, None));
    }
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

// ## On the planet
//
// For scenes on a planet the sea is the planet's surface, raised and lowered by the waves:
// `CurvedOcean`. Up close the surface is displaced, so waves stand up against the horizon and
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

/// The sea's surface as the sum of two tiles of incommensurate sizes, so that the pattern
/// repeats only far away: swell and wind waves on one, and on the other the shorter waves it
/// has no room for, down to half a metre.
pub struct WaveField {
    tiles: Vec<WaveTile>,
    /// Bounds on the height and the slope.
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
        let (big, small, n) = (487., 61.3, 256);
        let k_split = std::f64::consts::PI * n as f64 / big * 0.9;
        let mut tiles = vec![
            WaveTile::new(wind, big, n, 0., k_split, time, seed),
            WaveTile::new(wind, small, n / 2, k_split, f64::INFINITY, time, seed.wrapping_add(1)),
        ];
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
        let (h, s) = tiles.iter().map(|t| t.maxima()).fold((0., 0.), |a, b| (a.0 + b.0, a.1 + b.1));
        field.tiles = tiles;
        field.max_height = h;
        field.max_slope = s;
        field.mean_square_slope = field.measure_slope();
        field
    }

    /// Height and slopes at horizontal position (x, z).
    pub fn at(&self, x: f64, z: f64) -> Vector3<f64> {
        self.tiles.iter().map(|t| t.at(x, z)).sum()
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

/// The sea as the planet's surface, displaced by `waves` within `detail_distance` of
/// `detail_center` ([x, z]: put it under the camera), and beyond that a smooth sphere whose
/// normals still carry the waves' slopes.
pub struct CurvedOcean {
    pub radius: f64,
    pub waves: WaveField,
    pub detail_center: Vector2<f64>,
    pub detail_distance: f64,
}

impl CurvedOcean {
    fn center(&self) -> Vector3<f64> {
        Vector3::new(0., -self.radius, 0.)
    }

    /// How much of the waves' height is displaced at (x, z): all of it within half the detail
    /// distance, fading to none at it.
    fn displaced(&self, x: f64, z: f64) -> f64 {
        let d = (Vector2::new(x, z) - self.detail_center).norm() / self.detail_distance;
        1. - crate::procedural::nubis_clouds::smoothstep(0.5, 1., d)
    }

    /// Height of `p` above the displaced surface.
    fn above(&self, p: &Vector3<f64>) -> f64 {
        let altitude = (p - self.center()).norm() - self.radius;
        let a = self.displaced(p.x, p.z);
        if a <= 0. {
            return altitude;
        }
        altitude - a * self.waves.at(p.x, p.z).x
    }

    /// The local vertical and horizontal axes (x and z at the origin).
    fn frame(&self, p: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let up = (p - self.center()).normalize();
        let east = (Vector3::x() - up * up.x).normalize();
        (up, east, east.cross(&up))
    }

    /// The surface's normal at `p`: the waves' slopes, as displaced, and in full beyond.
    pub fn normal_at(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let (up, x, z) = self.frame(p);
        let w = self.waves.at(p.x, p.z);
        (up - x * w.y - z * w.z).normalize()
    }

    /// Where along the ray (from `t0` to `t1`) it is between the spheres that bound the waves.
    fn sphere_hits(&self, r: &Ray, radius: f64) -> Option<(f64, f64)> {
        let o = r.ro - self.center();
        let b = o.dot(&r.rd);
        let disc = b * b - (o.norm_squared() - radius * radius);
        if disc < 0. {
            return None;
        }
        Some((-b - disc.sqrt(), -b + disc.sqrt()))
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

    /// The first crossing of the displaced surface from `t0` to `t1`, by stepping no further
    /// than the surface could rise to meet the ray, then bisecting.
    fn march(&self, r: &Ray, t0: f64, t1: f64) -> Option<f64> {
        let fade_rate = 1.5 / (0.5 * self.detail_distance) * self.waves.max_height;
        let mut t = t0;
        let mut f = self.above(&(r.ro + r.rd * t));
        if f <= 0. {
            return Some(t);
        }
        while t < t1 {
            let p = r.ro + r.rd * t;
            let up = (p - self.center()).normalize();
            let descent = (-r.rd.dot(&up)).max(0.);
            let horizontal = (1. - r.rd.dot(&up).powi(2)).max(0.).sqrt();
            let rate = descent + horizontal * (self.waves.max_slope + fade_rate) + 1e-6;
            let step = (f / rate).max(0.01).min(t1 - t + 0.01);
            let next = t + step;
            let g = self.above(&(r.ro + r.rd * next));
            if g <= 0. {
                // Crossed: bisect.
                let (mut lo, mut hi) = (t, next);
                for _ in 0..24 {
                    let mid = 0.5 * (lo + hi);
                    if self.above(&(r.ro + r.rd * mid)) > 0. { lo = mid } else { hi = mid }
                }
                return Some(hi);
            }
            t = next;
            f = g;
        }
        None
    }
}

impl Geometry for CurvedOcean {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let r = Ray { ro: r.ro, rd: r.rd.normalize() };
        let h = self.waves.max_height;
        let hit = |t: f64| {
            let point = r.ro + r.rd * t;
            Some(RawIntersection { face: 0, part: 0, dist: t, point, normal: self.normal_at(&point) })
        };
        // The smooth sphere, where it is beyond the displaced region.
        let detail = self.detail_span(&r);
        let outside = |t: f64| detail.is_none_or(|(a, b)| t < a || t > b);
        let smooth = self.sphere_hits(&r, self.radius)
            .map(|(a, b)| [a, b])
            .and_then(|ts| ts.into_iter().find(|t| *t > 0. && outside(*t)));
        // The displaced region: between the bounding spheres, near the detail centre.
        let displaced = match (detail, self.sphere_hits(&r, self.radius + h)) {
            (Some((da, db)), Some((oa, ob))) => {
                let inner = self.sphere_hits(&r, self.radius - h).map(|(a, _)| a).filter(|a| *a > 0.);
                let start = da.max(oa).max(0.);
                let end = db.min(ob).min(inner.unwrap_or(f64::INFINITY));
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
        let r = Vector3::repeat(self.radius + self.waves.max_height);
        BBox::new(self.center() - r, self.center() + r)
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
        let into = Intersection { normal: n, ..intersection.clone() };
        // The water body, lit through the surface, as much as gets in.
        let mut emit = crate::material::functions::direct_lighting(self.water * (1. - fresnel(n.dot(&v))), &into, s);
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
        ScatteredRay { emit, sampled_lights: true, attenuate, ray }
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

/// An ocean lying on the planet (`"curved": true`).
fn create_curved_ocean(opts: &Value) -> SceneObject {
    let wind = SceneFile::parse_vec2_def(opts, "wind", Vector2::new(7., 2.));
    let waves = WaveField::new(
        wind,
        SceneFile::parse_number(&opts["time"], 0.),
        SceneFile::parse_int(&opts["seed"], 0) as u64,
        opts["amplitude"].as_f64(),
        SceneFile::parse_number(&opts["resolved"], 0.5).clamp(0., 1.),
    );
    let radius = SceneFile::parse_number(&opts["earth_radius"], crate::planet::EARTH_RADIUS);
    let material = physical_material(opts, waves.mean_square_slope, wind, Some(Vector3::new(0., -radius, 0.)));
    let ocean = CurvedOcean {
        radius,
        waves,
        detail_center: SceneFile::parse_vec2_def(opts, "detail_center", Vector2::zeros()),
        detail_distance: SceneFile::parse_number(&opts["detail_distance"], 300.).max(1.),
    };
    SceneObject { geometry: Box::new(ocean), medium: Box::new(Solid { m: Box::new(material) }) }
}

#[cfg(test)]
mod curved_tests {
    use super::*;

    fn ocean() -> CurvedOcean {
        CurvedOcean {
            radius: crate::planet::EARTH_RADIUS,
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
    fn test_rays_hit_the_waves_near_and_the_sphere_far() {
        let o = ocean();
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
        // Far beyond the detail: the smooth sphere.
        let r = Ray { ro: Vector3::new(5000., 50., 0.), rd: Vector3::new(0., -1., 0.) };
        let hit = o.intersects(&r).unwrap();
        assert!(((hit.point - o.center()).norm() - o.radius).abs() < 1e-3);
        // Up into the sky: nothing.
        assert!(o.intersects(&Ray { ro, rd: Vector3::new(0., 1., 1.).normalize() }).is_none());
    }
}
