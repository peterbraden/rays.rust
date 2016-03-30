//extern crate image;
extern crate nalgebra as na;

mod ray;
mod color;
mod material;
mod intersection;
mod sceneobject;
mod sphere;
mod scenegraph;
mod scene;
mod camera;
mod trace;
mod rendercontext;

use trace::trace;
use rendercontext::RenderContext;


fn poor_mans_paint(ctx: &RenderContext) {
    for y in 0 .. ctx.height {
        for x in 0 .. ctx.width {
            let c = ctx.get_pixel(x, y);
            if (c.rgb[0] < 0.1) { // Just use red channel ...
                print!("_");
            } else {
                print!("X");
            } 
        }
        print!("\n");
    }
}

fn main() {
    let s = scene::Scene::demo();
    let mut rc = RenderContext::new(s.width, s.height);

    for y in 0..s.height {
        for x in 0..s.width {
            let c = trace( &s.camera.get_ray(x as f64 / (s.width as f64), y as f64 / (s.height as f64)), 0, &s);  
            rc.set_pixel(x, y, c);
        }
    }
    poor_mans_paint(&rc);
}
