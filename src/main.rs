extern crate image;
extern crate nalgebra as na;

mod camera;
mod ray;

use na::Vec3;

fn main() {
    println!("Hello, world!");
    let WIDTH = 50;
    let HEIGHT = 100;

    let c = camera::Camera::new(
        Vec3::new(0f64,0f64,0f64),
        Vec3::new(0f64,0f64,1f64),
        Vec3::new(0f64,1f64,0f64),
        35.0,
        WIDTH, HEIGHT
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH{
          println!("{:?}", c.get_ray(x, y));
        }
    }
}
