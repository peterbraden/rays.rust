//! Rays: a ray tracer. This library holds the renderer; `main.rs` is the command line on top
//! of it, and `web/` drives it from a browser through WebAssembly.
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;

pub mod assets;
pub mod preview;
pub mod ray;
pub mod color;
pub mod material {
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
    pub mod grass;
}
pub mod intersection;
pub mod sceneobject;
pub mod light;
pub mod shapes {
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
pub mod octree;
pub mod bvh;
pub mod scenegraph;
pub mod scene;
pub mod scenefile;
pub mod skysphere;
pub mod ocean;
pub mod camera;
pub mod trace;
pub mod rendercontext;
#[cfg(feature = "cli")]
pub mod paint;
pub mod output;
#[cfg(feature = "cli")]
pub mod compare;
//mod wireframe;
pub mod geometry;
pub mod procedural {
    pub mod box_terrain;
    pub mod fireworks;
    pub mod heightmap;
    pub mod grass;
}
pub mod participatingmedia;
pub mod noise;

/// Parse JSON, for callers that don't depend on `serde_json` themselves. Panics if invalid.
pub fn serde_json_from_str(s: &str) -> serde_json::Value {
    serde_json::from_str(s).unwrap_or_else(|e| panic!("Invalid JSON: {}", e))
}
