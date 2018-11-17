use camera;
//use na::Vec3;
use scenegraph::SceneGraph;
use light::Light;
use color::Color;


pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Box<camera::Camera>,
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
}

