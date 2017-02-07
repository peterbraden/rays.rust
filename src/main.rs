#![allow(dead_code)]
//extern crate image;
extern crate nalgebra as na;
extern crate rand;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{Arg, App};

mod ray;
mod bbox;
mod color;
mod material;
mod intersection;
mod sceneobject;
mod light;
mod sphere;
mod checkeredplane;
mod octree;
mod scenegraph;
mod scene;
mod scenefile;
mod camera;
mod trace;
mod rendercontext;
mod paint;

use trace::trace;
use rendercontext::RenderContext;

fn main() {
    let app = App::new("Rays")
        .version("0.1")
        .arg(Arg::with_name("scene")
            .value_name("FILE")
            .help("Set scene file")
            .takes_value(true)
            .required(true)
            .index(1));

    let matches = app.get_matches();
    let s = scenefile::SceneFile::from_file(
                matches.value_of("scene").unwrap()
            );
    let mut rc = RenderContext::new(s.width, s.height);

    for y in 0..s.height {
        for x in 0..s.width {
            let (rays_cast, c) = trace( &s.camera.get_ray(x as f64 / (s.width as f64), y as f64 / (s.height as f64)), 0, &s);
            rc.rays_cast += rays_cast;
            rc.set_pixel(x, y, c);
        }
    }

    paint::to_png(&rc);
    paint::poor_mans(&rc);
    rc.print_stats();

}
