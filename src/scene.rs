use camera;
//use na::Vec3;
use scenegraph::SceneGraph;
use light::Light;


pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: camera::Camera,
    pub objects: SceneGraph,
    pub lights: Vec<Light>,

    pub ambient: f64,
    pub max_depth: u64,

    pub reflection: bool,
    pub specular: bool,
    pub diffuse: bool,
    pub shadow_bias: f64,
    pub supersamples: u32,
}

