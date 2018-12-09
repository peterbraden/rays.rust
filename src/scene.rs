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

    pub ambient: f64,
    pub max_depth: u64,

    pub reflection: bool,
    pub specular: bool,
    pub ambient_diffuse: u64,
    pub diffuse: bool,
    pub shadow_bias: f64,
    pub supersamples: u32,

    pub max_bounding: BBox,
    pub black_threshold: Color,
}

