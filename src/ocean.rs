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

/// Using Tessendorf algorithm, generate a mesh of triangles based on an
/// inverse-fast-fourier transform of a set of waves based on the Phillips
/// Spectrum.
///
///
/// == K ==
/// 
/// " the vector k, called the wavevector, is a horizontal vector that points in the direction of
/// travel of the wave, and has magnitude k related to the wavelength of the wave."
///
/// K = 2Pi / wavelength
///
/// The k-vector is simply the point on the 2D spectrum that the amplitude
/// applies to. Because we dot product the wind and k, waves _backwards_
/// from the wind location are weighted to zero.
///
/// ===
///
/// I still haven't worked out what units everything is in, or what the
/// scale factor 'A' should represent.
/// 
/// Normal distribution rand (gaussian)
fn randn(rng: &mut StdRng) -> f64 {
    let normal = Normal::new(0.0, 1.0);
    normal.sample(rng)
}

/// Phillips Spectrum
fn phillips(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> f64 {
    let ksq = k.x * k.x + k.y * k.y;
    if ksq < f64::MIN_POSITIVE { return 0. };
    let wind_dir = wind.normalize();
    let wk = k.normalize().dot(&wind_dir);
    let wind_speed = wind.norm();
    let l = (wind_speed * wind_speed) / gravity;
    scale * (-1.0 / (ksq * l * l)).exp() / (ksq * ksq) * wk * wk
}

fn amplitude(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64, rng: &mut StdRng) -> Complex<f64> {
    1f64/(2f64.sqrt()) *
        Complex::new(randn(rng), randn(rng)) *
        phillips(k, wind, scale, gravity).sqrt()
}

fn dispersion(k: Vector2<f64>, gravity: f64) -> f64 {
    let w = (k.norm() * gravity).sqrt(); // Deep water frequency relationship to magnitude
    let w_0 = 2f64 * std::f64::consts::PI / 200f64; // No idea? Rounding factor. Comes from keithlantz impl.
    (w / w_0).floor() * w_0
}

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

            // n and m are indices into mesh space - (-N/2 .. N/2)
            let n = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let m = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;

            let k = gen_k(n, m , lx, lz);
            h0[ind] = amplitude(k, wind, scale, gravity, rng);
        }
    }
    h0
}

fn gen_k(n: f64, m: f64, lx: f64, lz: f64) -> Vector2<f64> {
    // <2 pi n / Lx, 2 pi m / Lz>
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
fn ifft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
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

pub fn create_tile(
        fourier_grid_size:usize,
        lx: f64,
        lz: f64,
        gravity: f64,
        h0: Vec<Complex<f64>>) 
        -> OceanComplexData {

    let h0trans = transpose(&h0, fourier_grid_size);
    // Tile of amplitudes
    let mut ht =          vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_x =  vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_z =  vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dx = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];
    let mut ht_slope_dz = vec![Complex::new(0., 0.); fourier_grid_size * fourier_grid_size];

    for j in 0 .. fourier_grid_size {
        for i in 0 .. fourier_grid_size {
            let ind = j * fourier_grid_size + i;
            let n = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let m = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
            let k = gen_k(n, m , lx, lz);
            let w = dispersion(k, gravity);
            let c0 = Complex::new(w.cos(), w.sin());
            let c1 = Complex::new(w.cos(), -w.sin());

            ht[ind] =
                h0[ind] * c0 +
                h0trans[ind].conj() * c1;

            ht_slope_x[ind] = ht[ind] * Complex::new(0., k.x); 
            ht_slope_z[ind] = ht[ind] * Complex::new(0., k.y); 

            let len = k.norm();
            if len > 0.00001 {
                ht_slope_dx[ind] = ht[ind] * Complex::new(0., -k.x / len); 
                ht_slope_dz[ind] = ht[ind] * Complex::new(0., -k.y / len); 
            } 
        }
    }

    ht = ifft2(ht, fourier_grid_size);
    ht_slope_x = ifft2(ht_slope_x, fourier_grid_size);
    ht_slope_z = ifft2(ht_slope_z, fourier_grid_size);
    ht_slope_dx = ifft2(ht_slope_dx, fourier_grid_size);
    ht_slope_dz = ifft2(ht_slope_dz, fourier_grid_size);

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

        // Mesh size
        let lx = SceneFile::parse_number(&o["resolution"], 100.);
        let lz = lx;
        let choppyness_shift = false;

        let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

        // Size of the amplitude grid - between 16 and 2048, powers of 2
        // Titanic and Waterworld used 2048 - equivalent to 3cm resolution
        // Above 2048 numerics break down
        // In paper this is 'N'
        let fourier_grid_size = SceneFile::parse_number(&o["fourier_size"], 128.) as usize;

        let scale = SceneFile::parse_number(&o["amplitude"], 1.) * (fourier_grid_size * fourier_grid_size) as f64; // (A)

        let h0 = create_amplitude_tile(wind, scale, gravity, lx, lz, fourier_grid_size, &mut rng);
        let(
            ht,
            _ht_slope_x,
            _ht_slope_z,
            ht_slope_dx,
            ht_slope_dz,
        ) = create_tile(fourier_grid_size, lx, lz, gravity, h0); 

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
                let mut sign = 1.;
                if (x + z) % 2 == 0 {
                    // Sign correct ? Don't really understand this part
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
        let mut bb = BBox::min();

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

					return ScatteredRay{
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
		ScatteredRay {
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
	use super::{fft2, ifft2, transpose};
	use rustfft::num_complex::Complex;

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
