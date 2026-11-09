use crate::camera;
use crate::na::Vector3;
use crate::scenegraph::SceneGraph;
use crate::light::Light;
use crate::color::Color;
use crate::shapes::bbox::BBox;
use crate::participatingmedia::ParticipatingMedium;

/// What a path contributes when it is cut short. Paths always stop at `max_depth`, or when their
/// weight falls below the black threshold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathTermination {
    /// Stopped paths contribute nothing.
    Black,
    /// From `min_depth`, continue with probability equal to the bounce weight, scaling survivors
    /// to compensate. Unbiased (up to `max_depth`), so `max_depth` can be raised cheaply.
    RussianRoulette { min_depth: usize },
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
    pub path_termination: PathTermination,
}

impl RenderOpts {
    /// Samples per pixel in each pass over the image: an n x n grid, n = samples_per_chunk.
    pub fn samples_per_layer(&self) -> usize {
        self.samples_per_chunk.max(1).pow(2)
    }

    /// Passes over the image needed to take at least `supersamples` samples per pixel.
    pub fn layers(&self) -> usize {
        self.supersamples.max(1).div_ceil(self.samples_per_layer())
    }

    /// Samples per pixel actually taken: `supersamples` rounded up to a whole number of layers.
    pub fn samples_per_pixel(&self) -> usize {
        self.layers() * self.samples_per_layer()
    }
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

