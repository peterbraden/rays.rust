use crate::camera;
use crate::na::Vector3;
use crate::scenegraph::SceneGraph;
use crate::light::{Light, Sun};
use crate::color::Color;
use crate::shapes::bbox::BBox;
use crate::participatingmedia::ParticipatingMedium;
use crate::output::OutputOpts;
use crate::sceneobject::SceneObject;
use std::sync::Arc;
use crate::skysphere::SkyMaterial;

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
    pub background: Background,
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
    pub filter: Filter,
}

/// What a ray that leaves the scene sees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    Solid(Color),
    /// A blend from `bottom` (looking straight down) to `top` (straight up) by elevation:
    /// a = (y + 1) / 2 for the unit direction's y, then (1 - a) bottom + a top. From Peter
    /// Shirley, Trevor David Black and Steve Hollasch, "Ray Tracing in One Weekend", v4.0.2, 2025,
    /// section "Sending Rays Into the Scene"
    /// (<https://raytracing.github.io/books/RayTracingInOneWeekend.html>).
    Gradient { bottom: Color, top: Color },
}

impl Background {
    /// Radiance arriving from direction `rd`.
    pub fn radiance(&self, rd: &Vector3<f64>) -> Color {
        match self {
            Background::Solid(c) => *c,
            Background::Gradient { bottom, top } => {
                let a = 0.5 * (rd.normalize().y + 1.);
                *bottom * (1. - a) + *top * a
            },
        }
    }
}

/// How samples within a pixel are combined into its value: the reconstruction filter. Each
/// pixel's sample positions are drawn from the filter's own distribution, so every sample has
/// the same weight; see Manfred Ernst, Marc Stamminger and Gunther Greiner, "Filter Importance
/// Sampling", IEEE Symposium on Interactive Ray Tracing 2006, pp. 125-132, and Pharr, Jakob and
/// Humphreys, "Physically Based Rendering", 4th ed., section 8.8.2 "FilterSampler"
/// (<https://pbr-book.org/4ed/Sampling_and_Reconstruction/Image_Reconstruction.html>).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    /// Samples spread evenly over the pixel: an average of its own area.
    Box,
    /// A tent (triangle) of radius one pixel, falling linearly to zero at the neighbouring pixel
    /// centres (PBR section 8.8.4): samples reach into neighbouring pixels, weighted towards the
    /// centre. Smoother edges than the box, and the filter Bitterli's references use.
    Tent,
}

impl Filter {
    /// Map a uniform `u` in [0, 1) to an offset from the pixel's centre, in pixels, distributed
    /// as the filter.
    pub fn sample(&self, u: f64) -> f64 {
        match self {
            Filter::Box => u - 0.5,
            // Inverting the tent's cumulative distribution on [-1, 1].
            Filter::Tent => if u < 0.5 { (2. * u).sqrt() - 1. } else { 1. - (2. - 2. * u).sqrt() },
        }
    }
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
    /// A directional light, sampled directly by diffuse surfaces and media.
    pub sun: Option<Sun>,
    /// The skysphere's atmosphere, if any, for aerial perspective: light from distant objects
    /// is attenuated and hazed by the air it crosses.
    pub atmosphere: Option<Arc<SkyMaterial>>,
}

impl Scene {
    /// Whether `object` is one of the area lights that diffuse surfaces sample directly.
    pub fn is_sampled_light(&self, object: &SceneObject) -> bool {
        self.render.sample_lights && self.area_lights.iter().any(|l| std::ptr::eq(&**l, object))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_filter_is_the_previous_jitter() {
        for u in [0., 0.25, 0.999] {
            assert!((0.5 + Filter::Box.sample(u) - u).abs() < 1e-12);
        }
    }

    #[test]
    fn test_tent_filter_distribution() {
        // A tent on [-1, 1]: mean 0, variance 1/6, three quarters within half a pixel.
        let n = 100_000;
        let t: Vec<f64> = (0..n).map(|i| Filter::Tent.sample((i as f64 + 0.5) / n as f64)).collect();
        let mean = t.iter().sum::<f64>() / n as f64;
        let var = t.iter().map(|x| x * x).sum::<f64>() / n as f64;
        let inner = t.iter().filter(|x| x.abs() < 0.5).count() as f64 / n as f64;
        assert!(mean.abs() < 1e-6 && (var - 1. / 6.).abs() < 1e-4 && (inner - 0.75).abs() < 1e-4,
            "mean {} var {} inner {}", mean, var, inner);
        assert!(t.iter().all(|x| x.abs() <= 1.));
    }
}
