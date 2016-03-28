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
use rendercontext::RenderContext;


fn poorMansPaint(ctx: &RenderContext) {
    for y in 0 .. ctx.height {
        for x in 0 .. ctx.width {
          let c = color::Color::new(0,0,0);
          print!("{} ", c);
        }
        print!("\n");
    }
}

fn main() {
    let s = scene::Scene::demo();
    let mut rc = RenderContext::new(s.width, s.height);

    for y in 0..s.height {
        for x in 0..s.width {
            let c = trace( s.camera.get_ray(x, y), 0, &s);  
            rc.setPixel(x, y, c);
        }
    }
    poorMansPaint(&rc);
}
