//! Rays: the renderer as a library, for the command-line tool (`src/main.rs`, the `cli`
//! feature) and the browser (`web/`, built for wasm32 without `cli`).
#![allow(dead_code)]
#![allow(unused_imports)]

//extern crate image;
extern crate nalgebra as na;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rayon;
extern crate tobj;
extern crate ordered_float;
extern crate num_complex;
extern crate rustfft;


pub mod assets;
pub mod ray;
pub mod color;
pub mod spectrum;
pub mod motion;
pub mod sampler;
pub mod textures;
pub mod film;
pub mod lens;
pub mod flare;
pub mod material {
    pub mod model;
    pub mod bsdf;
    pub mod ggx;
    pub mod dielectric;
    pub mod specular;
    pub mod conductor;
    pub mod rough_dielectric;
    pub mod layered;
    pub mod subsurface;
    pub mod mix;
    pub mod hair;
    pub mod ambient;
    pub mod texture;
    pub mod lambertian;
    pub mod diffuse_light;
    pub mod normal;
    pub mod uv;
    pub mod functions;
    pub mod legacy;
    pub mod plastic;
    pub mod leaf;
    pub mod noise;
    pub mod grass;
    pub mod bump;
    pub mod rock;
    pub mod soil;
    pub mod phase;
    pub mod diffraction;
    pub mod ice_halos;
    pub mod rainbow;
}
pub mod intersection;
pub mod sceneobject;
pub mod light;
pub mod light_bvh;
pub mod wavefront;
#[cfg(feature = "gpu")]
pub mod gpu;
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
    pub mod bilinear;
    pub mod curve;
    pub mod subdivision;
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
    pub mod space;
}
pub mod participatingmedia;
pub mod noise;
pub mod terrain;
pub mod expr;
pub mod volume;
pub mod glare;
pub mod variance;
pub mod environment;
pub mod sobol;
pub mod planet;
