//extern crate image;
extern crate nalgebra as na;

mod ray;
mod scene;
mod camera;


use na::Vec3;

fn main() {
    println!("Hello, world!");
    let s = scene::Scene::demo();

    for y in 0..s.height {
        for x in 0..s.width {
          println!("{:?}", s.camera.get_ray(x, y));
        }
    }
}
