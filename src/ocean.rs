extern crate rand as _rand;
extern crate image;
extern crate rustfft;

use scene::Scene;
use intersection::Intersection;
use std::f64;
use ray::Ray;
use intersection::RawIntersection;
use serde_json::{Value, Map};
use rustfft::num_complex::Complex;
use na::{Vector3, Vector2};
use rand::distributions::{Normal, Distribution};
use ocean::_rand::Rng;
use std::path::Path;
use color::Color;
use rustfft::algorithm::Radix4;
use rustfft::FFT;
use shapes::bbox::BBox;
use shapes::triangle::Triangle;
use std::vec::Vec;
use std::sync::Arc;
use octree::Octree;
use shapes::geometry::Geometry;
use sceneobject::SceneObject;
use material::legacy::Whitted;
use material::normal::NormalShade;
use material::texture::{Solid, Medium};
use scenefile::SceneFile;
use material::functions::{scatter_dielectric, refract, reflect, schlick};
use material::model::{MaterialModel, ScatteredRay};
use geometry;



/// Using Tessendorf algorithm, generate a mesh of triangles based on an
/// inverse-fast-fourier transform of a set of waves based on the Phillips
/// Spectrum.
///
/// From what I've worked out from reading far too much about this, is that
/// although the basic idea, of using the IFFT to sum amplitudes over time,
/// is simple, there are a ton of non-obvious mathematical tricks used to
/// make this faster.
///
/// The tricky part, not mentioned in _any_ of the papers, is that we use
/// Euler's idea of rotation being a complex number, in order to store the
/// vertical and horizontal components of the wave - the real part of the
/// complex number ends up being the height component.
///
/// Another part that could be more obvious is _what_ the k vector is.
/// Phillips generates a weight for amplitudes based on a wind vector -
/// The k-vector is simply the point on the 2D spectrum that the amplitude
/// applies to. Because we dot product the wind and k, waves _backwards_
/// from the wind location are weighted to zero.
///
/// I still haven't worked out what units everything is in, or what the
/// scale factor 'A' should represent.
///
/// The Tessendorf paper is everything I hate about maths papers. Barely
/// enough details to implement, scattered symbols with no explanation,
/// and far more mathematical notation than is necessary. I don't understand
/// why mathematicians feel the need to code-golf their papers in this
/// way.
///
/// All the code on the internet is based on shaders - there's very little
/// CPU based code. I think I've managed to get this mostly right, but there
/// may be some lingering bugs.
///
///

fn randn() -> f64 {
    let normal = Normal::new(0.0, 1.0);
    return normal.sample(&mut rand::thread_rng());
}

/// Phillips Spectrum
fn phillips(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> f64 {
    let ksq = k.x * k.x + k.y * k.y;
    if ksq == 0. { return 0. };
    let wind_dir = wind.normalize();
    let wk = k.normalize().dot(&wind_dir);
    if wk < 0f64 { return 0. };  // modulate waves moving against the wind
    let wind_speed = wind.norm();
    let l = (wind_speed * wind_speed) / gravity;
    return scale / ksq * ksq * (-1.0 / (ksq * l * l)).exp() * wk * wk ;
}

fn amplitude(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> Complex<f64> {
    return
        1f64/(2f64.sqrt()) *
        Complex::new(randn(), randn()) *
        phillips(k, wind, scale, gravity).sqrt()
    ;
}

fn gen_k(n: f64, m: f64, lx: f64, lz: f64) -> Vector2<f64> {
    // <2 pi n / Lx, 2 pi m / Lz>
    return Vector2::new(
        2f64 * std::f64::consts::PI * n /  lx,
        2f64 * std::f64::consts::PI * m /  lz,
    );
}

fn mn_to_i(m: i32, n: i32, size: i32) -> usize {
    return ((n + size/2) * size + (m + size/2)) as usize;
}

fn transpose(matr: &Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let mut out = matr.clone();
    for x in 0..size {
        for y in 0..size {
            out[x*size + y] = matr[y*size + x];
        }
    }
    return out;
}

fn fft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let ifft = Radix4::new((size) as usize, false);
    let mut tile_clone = tile.clone();
    let mut fft = vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut tile_clone[..], &mut fft[..]);
    let mut conj = transpose(&fft, size);
    let mut out =  vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut conj, &mut out[..]);
    return transpose(&out, size);
}

fn ifft2 (tile: Vec<Complex<f64>>, size: usize) -> Vec<Complex<f64>> {
    let ifft = Radix4::new((size) as usize, true);
    let mut tile_clone = tile.clone();
    let mut fft = vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut tile_clone[..], &mut fft[..]);
	fft = fft.iter().map(|x| x.unscale(size as f64)).collect();
    let mut conj = transpose(&fft, size);
    let mut out =  vec![Complex::new(0., 0.); size * size];
    ifft.process_multi(&mut conj[..], &mut out[..]);
	out = out.iter().map(|x| x.unscale(size as f64)).collect();
    return transpose(&out, size);
}

fn to_real(x: usize, y: f64, z: usize, size: usize, lx: f64, lz: f64) -> Vector3<f64>{
	// TODO - scale
	return Vector3::new(
		x as f64 / size as f64 * lx - (lx/2.),
	    y,
		z as f64 / size as f64 * lz - (lz/2.));
}

fn get_y(x: usize, z: usize, mesh: &Vec<Complex<f64>>, fourier_grid_size: usize) -> f64 {
	return mesh[
			((x % fourier_grid_size) * fourier_grid_size as usize +
			 (z % fourier_grid_size))
		   as usize].re;
}


pub struct Ocean {
    triangles: Octree<Triangle>,
    bounds: BBox,
    triangle_count: usize,
}

impl Ocean {
    pub fn new(o: &Value) -> Ocean {
        let scale = SceneFile::parse_number(&o["amplitude"], 1.1e2f64); // (A)
        let gravity = SceneFile::parse_number(&o["gravity"], 9.81f64);
        let wind = SceneFile::parse_vec2_def(&o, "wind", Vector2::new(40., 30.));
        let time = 4f64;

        // Mesh size
        let lx = SceneFile::parse_number(&o["resolution"], 100.);
        let lz = lx;

        // Size of the amplitude grid - between 16 and 2048, powers of 2
        // Titanic and Waterworld used 2048 - equivalent to 3cm resolution
        // Above 2048 numerics break down
        let fourier_grid_size = SceneFile::parse_number(&o["fourier_size"], 128.) as usize; //2048i32;

        // Tile of amplitudes
        let mut h0 = vec![Complex::new(0., 0.); (fourier_grid_size * fourier_grid_size) as usize];
        let mut ht = vec![Complex::new(0., 0.); (fourier_grid_size * fourier_grid_size) as usize];


        for j in 0 .. fourier_grid_size {
            for i in 0 .. fourier_grid_size {
                let ind = j * fourier_grid_size + i;
                let n = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
                let m = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
                let k = gen_k(n, m , lx as f64, lz as f64);
                h0[ind] = amplitude(k, wind, scale, gravity);
            }
        }
        let h0trans = transpose(&h0, fourier_grid_size);

        for j in 0 .. fourier_grid_size {
            for i in 0 .. fourier_grid_size {
                let ind = j * fourier_grid_size + i;
                let n = (j as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
                let m = (i as f64 / fourier_grid_size as f64 - 0.5) * fourier_grid_size as f64;
                let k = gen_k(n, m , lx as f64, lz as f64);

                let w = (k.norm() * gravity).sqrt(); // Deep water frequency relationship to magnitude
                let wt = Complex::new(1., w * time).exp();

                ht[ind] =
                    h0[ind] * wt +
                    h0trans[ind].conj() * wt.conj();

                //println!(" - {} {} {} {} {}", m, n, mn_to_i(m, n, fourier_grid_size), k, wt);

            }
        }

        let mut mesh_complex = ifft2(ht, fourier_grid_size);
        //print!("OCEAN {:?}", mesh_complex);

        // Sign correct ?
        for x in 0 .. fourier_grid_size {
            for y in 0 .. fourier_grid_size {
                if x + y % 2 == 0 {
                    mesh_complex[x* fourier_grid_size + y] = mesh_complex[x* fourier_grid_size + y] * -1.
                }
            }
        }

		//DEBUG IMAGE
        let img = image::ImageBuffer::from_fn(fourier_grid_size as u32, fourier_grid_size as u32, |x, y| {
            let val = (mesh_complex[(x * fourier_grid_size as u32 + y) as usize].re * 0.5 + 0.5).max(0.);
            //let val = (h0[(x * fourier_grid_size as u32 + y) as usize].re * 50. + 0.5).max(0.);
            //println!("- {}", val);
            let (r,g,b) = Color::new(val, val, val).to_u8();
            image::Rgb([r,g,b])
        });
        let _ = img.save(&Path::new("debug-ocean.png"));

		// Generate Mesh
        let mut triangles = Vec::new();
        for x in 1 .. fourier_grid_size {
            for z in 1 .. fourier_grid_size {
				triangles.push(
					Arc::new(
                    Triangle::new(
						to_real(x,   get_y(x,   z,   &mesh_complex, fourier_grid_size),   z, fourier_grid_size, lx, lz),
						to_real(x,   get_y(x, z-1,   &mesh_complex, fourier_grid_size),   z-1, fourier_grid_size, lx, lz),
						to_real(x-1, get_y(x -1, z,  &mesh_complex, fourier_grid_size),   z, fourier_grid_size, lx, lz),
						)
					)
				);
				triangles.push(
					Arc::new(
                    Triangle::new(
						to_real(x-1, get_y(x-1, z,   &mesh_complex, fourier_grid_size),   z, fourier_grid_size, lx, lz),
						to_real(x,   get_y(x,   z-1, &mesh_complex, fourier_grid_size), z-1, fourier_grid_size, lx, lz),
						to_real(x-1, get_y(x-1, z-1, &mesh_complex, fourier_grid_size), z-1, fourier_grid_size, lx, lz),
						)
					)
				);
			}
		}

        let bounds = Ocean::bounds_of(&triangles);
        let tree = Octree::new(8, bounds, &triangles); 

        return Ocean {
            triangles: tree,
            bounds: bounds,
            triangle_count: triangles.len()
		};
    }

    fn bounds_of(triangles: &Vec<Arc<Triangle>>) -> BBox {
        let mut bb = BBox::min();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        return bb;
    }
}

struct OceanMaterial {}

/// Simplified dielectric with no refraction
impl MaterialModel for OceanMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
		let  ni_over_nt = 1. / 1.31;
		let drn = r.rd.dot(&intersection.normal);
		let cosine = - drn / r.rd.norm();

		match refract(r.rd, intersection.normal, ni_over_nt) {
			Some(_refracted) => {
				// refracted ray exists
				// Schlick approximation of fresnel amount
				let reflect_prob = schlick(cosine, 1.31);
				if geometry::rand() >= reflect_prob {
					return ScatteredRay{
						attenuate: Color::new(0., 0.2, 0.3),
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
		return ScatteredRay {
			attenuate: Color::white(),
			ray: Some(Ray {
				ro: intersection.point,
				rd: reflected
			}) 
		};
	}
}


impl Geometry for Ocean {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.triangles.raw_intersection(r, f64::INFINITY, 0f64);
    }

    fn bounds(&self) -> BBox {
        return self.bounds;
    }

    fn primitives(&self) -> u64 {
        return self.triangle_count as u64;
    }
}

pub fn create_ocean(opts: &Value) -> SceneObject {
	let o = Ocean::new(opts);
	return SceneObject {
		geometry: Box::new(o),
		medium: Box::new(Solid { m:Box::new(OceanMaterial {})}),
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
