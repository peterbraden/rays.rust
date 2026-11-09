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
    pub mod ply;
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
mod depth;
mod checkpoint;
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

    ///Write the image here instead of alongside the scene file. The extension selects the
    ///format: .png (8-bit, encoded for display) or .exr (linear floating point)
    #[arg(short, long)]
    output: Option<String>,

    ///Compare the render with this reference image (eg. a linear EXR from another renderer):
    ///print the exposure fit, RMSE and FLIP error, and write a FLIP error map alongside the
    ///output
    #[arg(long)]
    compare: Option<String>,

    ///Also write a depth image: the distance to the nearest surface through each pixel's centre.
    ///.exr keeps distances (misses are infinite); .png scales them for viewing, near white
    #[arg(long)]
    depth: Option<String>,

    ///With --compare, fail (exit status 2) if the mean FLIP error exceeds this
    #[arg(long)]
    max_flip: Option<f64>,

    ///Seed for the render's random numbers; overrides the scenefile if specified. The same
    ///seed gives the same image
    #[arg(long)]
    seed: Option<u64>,

    ///Set the samples per pixel; overrides the scenefile if specified. With --checkpoint, raise
    ///it to add samples to a finished render
    #[arg(long)]
    supersamples: Option<usize>,

    ///Save the render's progress to this file, and resume from it if it exists. A checkpoint
    ///from a different scene, size or seed is refused. The number of samples may differ: a
    ///finished render can be continued with more
    #[arg(long)]
    checkpoint: Option<String>,

    ///With --checkpoint, the seconds between saves; it is also saved when the render finishes
    #[arg(long, default_value_t = 60)]
    checkpoint_interval: u64,
}

/// The process's peak resident memory so far, in bytes: large meshes and their acceleration
/// structures are the main use. `getrusage` reports `ru_maxrss` in bytes on macOS and in
/// kilobytes on Linux and the BSDs.
#[cfg(unix)]
fn peak_memory() -> Option<u64> {
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return None;
    }
    let units = if cfg!(target_os = "macos") { 1 } else { 1024 };
    Some(usage.ru_maxrss as u64 * units)
}

#[cfg(not(unix))]
fn peak_memory() -> Option<u64> {
    None
}

fn main() {
    let args = Args::parse();

    println!("{}",
        style("# 1. - Parsing scene, building models").bold().cyan()
    );

    let build_start = Instant::now();
    let fail = |e: String| -> ! { eprintln!("{}", e); std::process::exit(1) };
    let contents = std::fs::read_to_string(&args.scene)
        .unwrap_or_else(|e| fail(format!("Failed to read {}: {}", args.scene, e)));
    let s = scenefile::SceneFile::from_string_overridden(contents.clone(), &scenefile::Overrides {
        width: args.width,
        height: args.height,
        seed: args.seed,
        supersamples: args.supersamples,
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
    let peak = peak_memory().map_or(String::new(), |b| format!(", peak memory {:.0} MiB", b as f64 / (1024. * 1024.)));
    println!("- Built in {:.2} s{}", build_start.elapsed().as_secs_f64(), peak);

    // Fail before rendering if the output can't be written in this format.
    let writer = output::writer_for(&rc.output_filename, s.output).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if let Some(path) = &args.depth {
        let raw = path.to_ascii_lowercase().ends_with(".exr");
        let opts = output::OutputOpts { transfer: output::Transfer::Linear, ..output::OutputOpts::default() };
        let img = depth::depth_image(&depth::depth(&s), s.image.width, s.image.height, raw);
        if let Err(e) = output::writer_for(path, opts).and_then(|w| w.write(&img, path)) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        println!("- Depth -> {}", path);
    }

    let checkpoint = args.checkpoint.as_ref().map(|path| {
        let header = checkpoint::CheckpointHeader {
            width: s.image.width,
            height: s.image.height,
            seed: s.render.seed,
            samples_per_layer: s.render.samples_per_layer(),
            scene: scenefile::SceneFile::fingerprint(&contents).unwrap_or_else(|e| fail(e)),
        };
        resume(&mut rc, &s, path, &header);
        (path.clone(), header)
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
    // Checkpoints likewise; the samples are copied under the lock, but saved outside it. Saves
    // are serialised, so two never share the temporary file.
    let checkpoint_interval = Duration::from_secs(args.checkpoint_interval);
    let last_checkpoint = Mutex::new(Instant::now());
    let saving = Mutex::new(());
    let save = |c: checkpoint::Checkpoint, path: &str| {
        let _saving = saving.lock().unwrap();
        if let Err(e) = c.save(path) {
            eprintln!("{}", e);
        }
    };

    rendercontext::render(&s, &rcmtx,
        |rc| {
            pb.set_position(rc.progress_percentage(&s) as u64);
            pb.set_message(rc.progress(&s));
            pb.tick();

            let mut last = last_progressive.lock().unwrap();
            let image = if rc.progressive_render && last.elapsed() >= progressive_interval {
                *last = Instant::now();
                Some((rc.snapshot(), rc.output_filename.clone()))
            } else {
                None
            };
            let mut last = last_checkpoint.lock().unwrap();
            let saved = match &checkpoint {
                Some((path, header)) if last.elapsed() >= checkpoint_interval => {
                    *last = Instant::now();
                    Some((rc.checkpoint(header.clone()), path.clone()))
                },
                _ => None,
            };
            if image.is_none() && saved.is_none() { None } else { Some((image, saved)) }
        },
        |(image, saved)| {
            if let Some((img, path)) = image {
                if let Err(e) = writer.write(&img, &path) {
                    eprintln!("{}", e);
                }
            }
            if let Some((c, path)) = saved {
                save(c, &path);
            }
        });
    pb.finish();

    let rc = rcmtx.lock().unwrap();
    if let Some((path, header)) = &checkpoint {
        save(rc.checkpoint(header.clone()), path);
        println!("- Checkpoint -> {}", path);
    }
    //wireframe::wireframe(&s, &mut rc);
    let img = rc.snapshot();
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

/// Continue from the checkpoint at `path`, if there is one. One that doesn't belong to this
/// render is an error rather than overwritten, as it may hold hours of work.
fn resume(rc: &mut RenderContext, s: &scene::Scene, path: &str, header: &checkpoint::CheckpointHeader) {
    let fail = |e: String| -> ! { eprintln!("{}", e); std::process::exit(1) };
    match checkpoint::Checkpoint::load(path).unwrap_or_else(|e| fail(e)) {
        None => println!("- Checkpoint: {} (new)", path),
        Some(c) => {
            if let Err(e) = c.check(header) {
                fail(format!("Cannot resume from {}: {}. Remove it, or choose another --checkpoint, to start afresh.", path, e));
            }
            rc.resume(c);
            let done = rc.progress_percentage(s);
            println!("- Checkpoint: resuming from {}, {:.0}% done", path, done.min(100.));
            if done > 100. {
                println!("- The checkpoint has more samples than --supersamples asks for; it is used as it is");
            }
        },
    }
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
