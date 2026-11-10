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
    pub mod phase;
    pub mod diffraction;
    pub mod ice_halos;
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
mod output;
mod compare;
//mod wireframe;
mod geometry;
mod procedural {
    pub mod box_terrain;
    pub mod fireworks;
    pub mod nubis_clouds;
    pub mod hogan_kew_cirrus;
    pub mod benard_altocumulus;
}
mod participatingmedia;
mod noise;
mod volume;
mod glare;
mod planet;

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

    ///Write the image here instead of alongside the scene file. The extension selects the
    ///format: .png (8-bit, encoded for display) or .exr (linear floating point)
    #[arg(short, long)]
    output: Option<String>,

    ///Compare the render with this reference image (eg. a linear EXR from another renderer):
    ///print the exposure fit, RMSE and FLIP error, and write a FLIP error map alongside the
    ///output
    #[arg(long)]
    compare: Option<String>,

    ///With --compare, fail (exit status 2) if the mean FLIP error exceeds this
    #[arg(long)]
    max_flip: Option<f64>,

    ///Seed for the render's random numbers; overrides the scenefile if specified. The same
    ///seed gives the same image
    #[arg(long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    println!("{}",
        style("# 1. - Parsing scene, building models").bold().cyan()
    );

    let s = scenefile::SceneFile::from_file(&args.scene, &scenefile::Overrides {
        width: args.width,
        height: args.height,
        seed: args.seed,
    });

    let mut rc = RenderContext::new(
            s.image.width,
            s.image.height,
            args.progressive_render,
            &args.scene,
            );
    if let Some(path) = &args.output {
        rc.output_filename = path.clone();
    }
    println!("- Output: {}x{} @ {} samples -> {}", s.image.width, s.image.height, s.render.samples_per_pixel(), rc.output_filename);
    println!("- Scene Objects: {}, Primitives: {} ", s.objects.len(),  s.objects.primitives_len()); 

    // Fail before rendering if the output can't be written in this format.
    let writer = output::writer_for(&rc.output_filename, s.output).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let rcmtx = Mutex::new(rc);

    println!("{}",
        style("# 2. - Rendering ").bold().cyan()
    );
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {prefix}{msg} [{bar:.cyan/blue}]").unwrap().progress_chars("#>-"));

    // Progressive output is throttled, and encoded outside the lock so it doesn't serialise
    // the render.
    let progressive_interval = Duration::from_secs(1);
    let last_progressive = Mutex::new(Instant::now());

    rendercontext::render(&s, &rcmtx,
        |rc| {
            pb.set_position(rc.progress_percentage(&s) as u64);
            pb.set_message(rc.progress(&s));
            pb.tick();

            let mut last = last_progressive.lock().unwrap();
            if rc.progressive_render && last.elapsed() >= progressive_interval {
                *last = Instant::now();
                Some((rc.snapshot(), rc.output_filename.clone()))
            } else {
                None
            }
        },
        |(img, path)| {
            if let Err(e) = writer.write(&s.output.finish(&img), &path) {
                eprintln!("{}", e);
            }
        });
    pb.finish();

    let rc = rcmtx.lock().unwrap();
    //wireframe::wireframe(&s, &mut rc);
    let img = s.output.finish(&rc.snapshot());
    if let Err(e) = writer.write(&img, &rc.output_filename) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    if let Some(reference) = &args.compare {
        compare_with(&img, reference, &rc.output_filename, &s.output, args.max_flip);
    }
    paint::poor_mans(&img, &s.output);
    rc.print_stats();
}

/// Compare the finished render with a reference image, reporting how close they are.
fn compare_with(img: &output::LinearImage, reference: &str, output_filename: &str, opts: &output::OutputOpts, max_flip: Option<f64>) {
    let fail = |e: String| -> ! { eprintln!("{}", e); std::process::exit(1) };
    let reference_img = output::read_reference(reference, opts).unwrap_or_else(|e| fail(e));
    let c = compare::compare(img, &reference_img, opts).unwrap_or_else(|e| fail(e));
    let map_path = std::path::Path::new(output_filename).with_extension("flip.png");
    if let Err(e) = c.error_map.save(&map_path) {
        eprintln!("Failed to write {}: {}", map_path.display(), e);
    }
    // The reference at our size, encoded exactly as our PNG output is, to view side by side.
    let reference_path = std::path::Path::new(output_filename).with_extension("reference.png");
    let reference_path = reference_path.to_string_lossy();
    if let Err(e) = output::ImageWriter::write(&output::Png { opts: *opts }, &c.reference, &reference_path) {
        eprintln!("{}", e);
    }
    println!("{}", style("# Comparison").bold().cyan());
    println!("- Reference: {}", reference);
    println!("- Exposure fit: {:.4} (1 means the same brightness)", c.exposure);
    println!("- RMSE: {:.5}, after exposure fit: {:.5}", c.rmse, c.rmse_fitted);
    println!("- Mean FLIP error: {:.4} (error map: {})", c.flip_mean, map_path.display());
    println!("- Reference at this size: {}", reference_path);
    if let Some(max) = max_flip {
        if c.flip_mean > max {
            eprintln!("Mean FLIP error {:.4} exceeds {:.4}", c.flip_mean, max);
            std::process::exit(2);
        }
    }
}
