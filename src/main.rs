//extern crate image;
extern crate nalgebra as na;

mod ray;
mod color;
mod scene;
mod camera;
mod trace;
mod rendercontext;

use na::Vec3;
use trace::trace;


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
          let c = trace( s.camera.get_ray(x, y), 0, &s);  
        }
    }
    poorMansPaint(s);
}
