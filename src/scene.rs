use camera;
use na::Vector3;
use scenegraph::SceneGraph;
use light::Light;
use color::Color;
use shapes::bbox::BBox;

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub camera: Box<camera::Camera + Sync>,
    pub objects: SceneGraph,
    pub lights: Vec<Light>,

    pub background: Color,

    pub max_depth: usize,
    pub shadow_bias: f64,

    pub supersamples: usize,
    pub chunk_size: usize,
    pub samples_per_chunk: usize,

    pub max_bounding: BBox,
    pub black_threshold: Color,
}

