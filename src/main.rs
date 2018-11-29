#![allow(dead_code)]
#![allow(unused_imports)]

//extern crate image;
extern crate nalgebra as na;
extern crate rand;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rayon;
extern crate tobj;

use clap::{Arg, App};
use rayon::prelude::*;

mod ray;
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
    pub mod bbox;
    pub mod geometry;
    pub mod sphere;
    pub mod plane;
    pub mod triangle;
    pub mod mesh;
}
mod octree;
mod scenegraph;
mod scene;
mod scenefile;
mod camera;
mod trace;
mod rendercontext;
mod paint;
//mod wireframe;
mod geometry;

use trace::trace;
use rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use rand::thread_rng;
use rand::seq::SliceRandom;


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
            .index(1))
        .arg(Arg::with_name("progressive_render")
            .short("p")
            .long("progressive-render")
            .help("Update the output file when a chunk is completed. Good for debugging"));

    println!("- Building models");
    let matches = app.get_matches();
    let s = scenefile::SceneFile::from_file(
                matches.value_of("scene").unwrap()
            );
    let rc = RenderContext::new(
            s.width,
            s.height,
            matches.is_present("progressive_render"),
            );
    rc.print_scene_stats(&s);
    let rcmtx = Arc::new(Mutex::new(rc));
    let mut rows: Vec<u32> = (0 .. s.height).collect();

    let mut rng = thread_rng();
    rows.shuffle(&mut rng);
    println!("- Starting Render");

    rows.into_par_iter().for_each(|y| {
        // Progressive render out:
        render_row(y, &s, rcmtx.clone());

        let rc = rcmtx.lock().unwrap();

        if rc.progressive_render {
            paint::to_png(&rc);
        }
        if y % 10 == 0 {
            &rc.print_progress(0, y);
        }
    });

    let rc = rcmtx.lock().unwrap();
    //wireframe::wireframe(&s, &mut rc);
    paint::to_png(&rc);
    paint::poor_mans(&rc);
    rc.print_stats();
}
