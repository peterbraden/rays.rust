//! Rays: a ray tracer. This library holds the renderer; `main.rs` is the command line on top
//! of it, and `web/` drives it from a browser through WebAssembly.
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate nalgebra as na;
#[macro_use]
extern crate serde_derive;

pub mod assets;
pub mod stopwatch;
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
    pub mod leaf;
    pub mod noise;
    pub mod grass;
    pub mod bump;
    pub mod rock;
    pub mod phase;
    pub mod diffraction;
    pub mod ice_halos;
    pub mod rainbow;
}
pub mod intersection;
pub mod sceneobject;
pub mod light;
pub mod shapes {
    pub mod transform;
    pub mod bbox;
    pub mod geometry;
    pub mod geometry_set;
    pub mod csg;
    pub mod sphere;
    pub mod plane;
    pub mod triangle;
    pub mod mesh;
    pub mod ply;
    pub mod wo3;
    pub mod infinite;
    pub mod repeating_mesh;
    pub mod polynomial;
    pub mod cone;
    pub mod disc;
    pub mod algebraic;
    pub mod polygon;
    pub mod revolution;
    pub mod sweep;
    pub mod blob;
    pub mod implicit;
    pub mod julia;
    pub mod tessellated;
    pub mod text;
}
pub mod octree;
pub mod bvh;
pub mod scenegraph;
pub mod scene;
pub mod scenefile;
pub mod nff;
pub mod tungsten;
pub mod scenefile_shapes;
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
pub mod depth;
//mod wireframe;
pub mod geometry;
pub mod procedural {
    pub mod box_terrain;
    pub mod fireworks;
    pub mod tree;
    pub mod forest;
    pub mod heightmap;
    pub mod grass;
    pub mod rock;
    pub mod nubis_clouds;
    pub mod hogan_kew_cirrus;
    pub mod benard_altocumulus;
    pub mod lenticular;
    pub mod terrain;
    pub mod mammatus;
    pub mod precipitation;
    pub mod harris_cumulus;
}
pub mod participatingmedia;
pub mod noise;
pub mod terrain;
pub mod expr;
pub mod volume;
pub mod glare;
pub mod planet;

/// Parse JSON, for callers that don't depend on `serde_json` themselves. Panics if invalid.
pub fn serde_json_from_str(s: &str) -> serde_json::Value {
    serde_json::from_str(s).unwrap_or_else(|e| panic!("Invalid JSON: {}", e))
}
