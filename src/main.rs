#![allow(dead_code)]
//extern crate image;
extern crate nalgebra as na;
extern crate rand;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rayon;

use clap::{Arg, App};
use rayon::prelude::*;

mod ray;
mod bbox;
mod color;
mod material {
    pub mod model;
    pub mod dielectric;
    pub mod specular;
    pub mod ambient;
    pub mod texture;
    pub mod lambertian;
    pub mod diffuse_light;
    pub mod normal;
    pub mod functions;
}
mod intersection;
mod sceneobject;
mod light;
mod shapes {
    pub mod geometry;
    pub mod sphere;
    pub mod plane;
}
mod octree;
mod scenegraph;
mod scene;
mod scenefile;
mod camera;
mod trace;
mod rendercontext;
mod paint;
mod geometry;

use trace::trace;
use rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use rand::Rng;

fn render_row(y: u32, s: &scene::Scene, rcmtx: Arc<Mutex<rendercontext::RenderContext>>){
    for x in 0..s.width {
        let mut pixel = color::Color::black();
        let mut cast = 0;

        // Monte-Carlo method: We sample many times and average.
        for sx in 0..s.supersamples {
            for sy in 0..s.supersamples {
                let (rays_cast, c) = trace(
                        &s.camera.get_ray(
                            x as f64 / (s.width as f64),
                            y as f64 / (s.height as f64),
                            sx as f64 / (s.supersamples as f64) * 1. / (s.width as f64),
                            sy as f64 / (s.supersamples as f64) * 1. / (s.height as f64))
                        , 0, &s);
                cast = cast + rays_cast;
                pixel = pixel + c;
            }
        }
        let mut rc = rcmtx.lock().unwrap();
        rc.rays_cast += cast;
        rc.set_pixel(x, y, pixel / ((s.supersamples * s.supersamples) as f64));
    }
}

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
    let rcmtx = Arc::new(Mutex::new(rc));
    let mut rows: Vec<u32> = (0 .. s.height).collect();
    rand::thread_rng().shuffle(&mut rows);

    rows.into_par_iter().for_each(|y| {
        // Progressive render out:
        render_row(y, &s, rcmtx.clone());

        let rc = rcmtx.lock().unwrap();
        paint::to_png(&rc);
        if &rc.rays_cast % 10000 == 0 {
            &rc.print_progress(x, y);
        }
    });
    
    let rc = rcmtx.lock().unwrap();

    paint::to_png(&rc);
    paint::poor_mans(&rc);
    rc.print_stats();

}
