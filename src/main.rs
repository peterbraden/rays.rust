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
extern crate ordered_float;
extern crate num_complex;
extern crate rustfft;


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
    pub mod legacy;
    pub mod plastic;
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
    pub mod infinite;
    pub mod repeating_mesh;
}
mod octree;
mod scenegraph;
mod scene;
mod scenefile;
mod skysphere;
mod ocean;
mod camera;
mod trace;
mod rendercontext;
mod paint;
//mod wireframe;
mod geometry;
mod procedural {
    pub mod box_terrain;
}

use trace::trace;
use rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use rand::thread_rng;
use rand::seq::SliceRandom;

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

    let start_time = time::precise_time_s();
    println!("- Building models");
    let matches = app.get_matches();
    let s = scenefile::SceneFile::from_file(
                matches.value_of("scene").unwrap()
            );
    let rc = RenderContext::new(
            start_time,
            s.image.width,
            s.image.height,
            matches.is_present("progressive_render"),
            matches.value_of("scene").unwrap(),
            );
    rc.print_scene_stats(&s);
    let chunks: Vec<rendercontext::RenderableChunk> = rc.iter(&s).collect();
    let rcmtx = Arc::new(Mutex::new(rc));

    //let mut rng = thread_rng();
    //chunks.shuffle(&mut rng);

    println!("- Starting Render");
    chunks
        .into_par_iter()
        //.iter()
        .for_each(|c| {
            let p = c.render(&s);
            let rcmtx = rcmtx.clone();
            let mut rc = rcmtx.lock().unwrap();
            rc.apply_chunk(&c, &p);
            // Progressive render out:
            if rc.progressive_render {
                paint::to_png(&rc);
            }
            &rc.print_progress(&s);
        });

    let rc = rcmtx.lock().unwrap();
    //wireframe::wireframe(&s, &mut rc);
    paint::to_png(&rc);
    paint::poor_mans(&rc);
    rc.print_stats();
}
