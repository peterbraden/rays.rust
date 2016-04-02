//extern crate image;
extern crate nalgebra as na;

mod ray;
mod color;
mod material;
mod intersection;
mod sceneobject;
mod light;
mod sphere;
mod scenegraph;
mod scene;
mod camera;
mod trace;
mod rendercontext;
mod paint;

use trace::trace;
use rendercontext::RenderContext;


fn poor_mans_paint(ctx: &RenderContext) {
    for y in 0 .. 20 {
        for x in 0 .. 20 {
            let c = ctx.get_pixel(((x as f32/20.0) * ctx.width as f32) as u32 , ((y as f32 / 20.0) * ctx.height as f32) as u32);
            if c.rgb[0] < 0.1 { // Just use red channel ...
                print!("_");
            } else if c.rgb[0] < 0.3 {
                print!(".");
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
    paint::to_png(&rc);
}
