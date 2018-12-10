extern crate rand as _rand;
extern crate image;

use num_complex::Complex;
use na::{Vector3, Vector2};
use rand::distributions::{Normal, Distribution};
use ocean::_rand::Rng;
use std::path::Path;
use color::Color;

fn randn() -> f64 {
    let normal = Normal::new(0.0, 1.0);
    return normal.sample(&mut rand::thread_rng());
}

/// Phillips Spectrum
/// L = largest possible waves
///
fn phillips(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> f64 {
    let ksq = k.x * k.x + k.y * k.y;
    if ksq == 0. { return 0. };
    let wind_dir = wind.normalize();
    let wk = k.normalize().dot(&wind_dir);
    let wind_speed = wind.norm();
    let l = (wind_speed * wind_speed) / gravity;
    //if wk < std::f64::MIN_POSITIVE { return 0. }; // TODO!!
    return scale / ksq * ksq * (-1.0 / (ksq * l * l)).exp() * wk * wk ;
}


fn amplitudes(k: Vector2<f64>, wind: Vector2<f64>, scale: f64, gravity: f64) -> Complex<f64> {
    let irt2 = 1f64/(2f64.sqrt());
    let rd = Complex::new(randn(), randn());
    let rtphlps = phillips(k, wind, scale, gravity);
    return irt2 * rd * rtphlps.sqrt();
}

fn gen_k(n: f64, m: f64, lx: f64, lz: f64) -> Vector2<f64> {
    // <2 pi n / Lx, 2 pi m / Ly>
    return Vector2::new(
        2f64 * std::f64::consts::PI * n /  lx,
        2f64 * std::f64::consts::PI * m /  lz,
    );
}
    
/*
fn calc_wave(initial_height: f64, W, time: f64, Grid_Sign ) -> {
    let wt = exp(1i .* W .* time ) ;
    let Ht = initial_height .* wt + conj(rot90(H0,2)) .* conj(wt) ;  
    return real( ifft2(Ht) .* Grid_Sign ) ;
}
*/

fn mn_to_i(m: i32, n: i32, size: i32) -> usize {
    return ((n + size/2) * size + (m + size/2)) as usize;
}


pub struct Ocean {
} 


impl Ocean {
    
    pub fn new() -> Ocean {
        let scale = 1e-7f64; // (A)
        let gravity = 9.81f64;
        let wind = Vector2::new(50., 20.);
        let time = 0f64;
        let fourier_grid_size = 256; //2048i32; // Same as titanic - 3cm;
        let mut height = vec![Complex::new(0., 0.); (fourier_grid_size * fourier_grid_size) as usize];
        let lx = 350.;
        let ly = 350.;

        for n in -1 * fourier_grid_size/2 .. fourier_grid_size / 2 {
            for m in -1 * fourier_grid_size/2 .. fourier_grid_size / 2 {
                let k = gen_k(n as f64, m as f64, lx, ly);
                let amplitude = amplitudes(k, wind, scale, gravity);
                height[mn_to_i(m, n, fourier_grid_size)] = amplitude;
                println!(" - {} {} {} {} {}", m, n, mn_to_i(m, n, fourier_grid_size), k, amplitude);
            } 
        } 
        //print!("OCEAN {:?}", height); 

        let img = image::ImageBuffer::from_fn(fourier_grid_size as u32, fourier_grid_size as u32, |x, y| {
            let val = height[(x * fourier_grid_size as u32 + y) as usize].im;
            let (r,g,b) = Color::new(val, val, val).to_u8();
            image::Rgb([r,g,b])
        });

        let _ = img.save(&Path::new("debug-ocean.png"));
        //let tile = vec![0., n*m];
        return Ocean {};
    }

}
