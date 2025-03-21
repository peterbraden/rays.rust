#![allow(dead_code)]
#![allow(unused_imports)]

//extern crate image;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rayon;
extern crate tobj;
extern crate ordered_float;
extern crate num_complex;
extern crate rustfft;
extern crate termcolor;


use clap::Parser;
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
    pub mod transform;
    pub mod bbox;
    pub mod geometry;
    pub mod csg;
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
    pub mod fireworks;
}
mod participatingmedia;

use crate::trace::trace;
use crate::rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use rand::thread_rng;
use rand::seq::SliceRandom;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use console::style;


/// Rays
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Set scene file
    #[arg(index=1)]
    scene: String,

    ///Update the output file when a chunk is completed. Good for debugging
    #[arg(short, long)]
    progressive_render: bool,

    ///Set the width of the output image; overrides the scenefile if specified
    #[arg(long)]
    width: Option<usize>,

    ///Set the height of the output image; overrides the scenefile if specified
    #[arg(long)]
    height: Option<usize>,
}

fn main() {
    let args = Args::parse();

    println!("{}",
        style("# 1. - Parsing scene, building models").bold().cyan()
    );

    let s = scenefile::SceneFile::from_file(
       &args.scene 
    );
    let width = args.width.unwrap_or(s.image.width);
    let height = args.height.unwrap_or(s.image.height);
    // TODO: Overriding here isn't picked up in the camera config that happens in the parse.
    
    let rc = RenderContext::new(
            width,
            height,
            args.progressive_render,
            &args.scene,
            );
    println!("- Output: {}x{} @ {} samples -> {}", s.image.width, s.image.height, s.render.supersamples, rc.output_filename);
    println!("- Scene Objects: {}, Primitives: {} ", s.objects.len(),  s.objects.primitives_len()); 

    let chunks: Vec<rendercontext::RenderableChunk> = rc.iter(&s).collect();
    let rcmtx = Arc::new(Mutex::new(rc));

    //let mut rng = thread_rng();
    //chunks.shuffle(&mut rng);

    println!("{}",
        style("# 2. - Rendering ").bold().cyan()
    );
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {prefix}{msg} [{bar:.cyan/blue}]").unwrap().progress_chars("#>-"));

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
            pb.set_position(rc.progress_percentage(&s) as u64);
            pb.set_message(rc.progress(&s));
            pb.tick();
        });
    pb.finish();

    let rc = rcmtx.lock().unwrap();
    //wireframe::wireframe(&s, &mut rc);
    paint::to_png(&rc);
    paint::poor_mans(&rc);
    rc.print_stats();
}
