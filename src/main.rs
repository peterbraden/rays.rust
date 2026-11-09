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
    pub mod noise;
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
mod noise;

use crate::trace::trace;
use crate::rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
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

    let s = scenefile::SceneFile::from_file(&args.scene, args.width, args.height);

    let rc = RenderContext::new(
            s.image.width,
            s.image.height,
            args.progressive_render,
            &args.scene,
            );
    println!("- Output: {}x{} @ {} samples -> {}", s.image.width, s.image.height, s.render.samples_per_pixel(), rc.output_filename);
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

    // Progressive output is throttled, and encoded outside the lock so it doesn't serialise
    // the render.
    let progressive_interval = Duration::from_secs(1);
    let last_progressive = Mutex::new(Instant::now());

    chunks
        .into_par_iter()
        //.iter()
        .for_each(|c| {
            let p = c.render(&s);
            let progressive = {
                let mut rc = rcmtx.lock().unwrap();
                rc.apply_chunk(&c, &p);
                pb.set_position(rc.progress_percentage(&s) as u64);
                pb.set_message(rc.progress(&s));
                pb.tick();

                let mut last = last_progressive.lock().unwrap();
                if rc.progressive_render && last.elapsed() >= progressive_interval {
                    *last = Instant::now();
                    Some((paint::to_image(&rc), rc.output_filename.clone()))
                } else {
                    None
                }
            };
            if let Some((img, path)) = progressive {
                if let Err(e) = paint::save_png(&img, &path) {
                    eprintln!("{}", e);
                }
            }
        });
    pb.finish();

    let rc = rcmtx.lock().unwrap();
    //wireframe::wireframe(&s, &mut rc);
    if let Err(e) = paint::to_png(&rc) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    paint::poor_mans(&rc);
    rc.print_stats();
}
