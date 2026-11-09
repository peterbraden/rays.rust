use crate::camera;
use crate::na::Vector3;
use crate::scenegraph::SceneGraph;
use crate::light::Light;
use crate::color::Color;
use crate::shapes::bbox::BBox;
use crate::participatingmedia::ParticipatingMedium;
use crate::output::OutputOpts;
use crate::sceneobject::SceneObject;
use std::sync::Arc;

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
    /// Seeds the random numbers used while rendering (sampling, scattering); the same seed
    /// gives the same image.
    pub seed: u64,
    /// Whether diffuse surfaces sample area lights directly (next-event estimation). Off, they
    /// are found only when a scattered ray happens to hit them: unbiased, but far noisier.
    pub sample_lights: bool,
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
    pub output: OutputOpts,
    /// Emitting objects whose surface can be sampled, for direct lighting (next-event
    /// estimation). The same `Arc`s as in `objects`, so a hit can be recognised as one of them.
    pub area_lights: Vec<Arc<SceneObject>>,
}

impl Scene {
    /// Whether `object` is one of the area lights that diffuse surfaces sample directly.
    pub fn is_sampled_light(&self, object: &Arc<SceneObject>) -> bool {
        self.render.sample_lights && self.area_lights.iter().any(|l| Arc::ptr_eq(l, object))
    }
}

