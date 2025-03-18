use crate::camera;
use crate::na::Vector3;
use crate::scenegraph::SceneGraph;
use crate::light::Light;
use crate::color::Color;
use crate::shapes::bbox::BBox;
use crate::participatingmedia::ParticipatingMedium;

// TODO
pub enum PathCulling {
    MaxDepth, // Keep recursing unti "max_depth"
    BlackThreshold, // Stop recursing when weight drops below threshold or "max_depth"
    RussianRoulette, // Stop recursing when < "min_depth" and rand() > luminance or "max_depth"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageOpts {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct RenderOpts {
    pub background: Color,
    pub max_depth: usize,
    pub shadow_bias: f64,
    pub supersamples: usize,
    pub chunk_size: usize,
    pub samples_per_chunk: usize,
}

pub struct Scene {
    pub image: ImageOpts,
    pub render: RenderOpts,
    pub camera: Box<dyn camera::Camera + Sync>,
    pub objects: SceneGraph,
    pub lights: Vec<Light>,
    pub max_bounding: BBox,
    pub black_threshold: f64,
    pub air_medium: Box<dyn ParticipatingMedium>,
}

