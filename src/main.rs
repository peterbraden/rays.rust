//extern crate image;
extern crate nalgebra as na;

mod ray;
mod color;
mod scene;
mod camera;


use na::Vec3;


fn poorMansPaint(s: scene::Scene) { //, RenderContext ctx){
    for y in 0..s.height {
        for x in 0..s.width {
          let c = color::Color::new(0,0,0);
          print!("{} ", c);
        }
        print!("\n");
    }
}

fn main() {
    let s = scene::Scene::demo();

    for y in 0..s.height {
        for x in 0..s.width {
          //println!("{:?}", s.camera.get_ray(x, y));
        }
    }
    poorMansPaint(s);
}
