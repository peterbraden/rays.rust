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
    /// Whether surfaces with a BSDF sample the sky directly, in proportion to its brightness
    /// (`environment`), combined with BSDF sampling by MIS. Off, the sky is found only by
    /// scattered rays.
    pub sample_sky: bool,
    pub filter: Filter,
    /// Where the random numbers a sample uses come from.
    pub sampler: Sampler,
    /// The roughness floor for glossy vertices after a rougher one (`ggx::regularised`); 0 for
    /// none.
    pub regularise: f64,
    /// Whether to trace light at sampled wavelengths rather than in RGB (`spectrum`).
    pub spectral: bool,
    /// When the shutter is open, and whether anything moves meanwhile (`motion`): only then
    /// does each sample take a time.
    pub shutter: crate::motion::Shutter,
    pub motion: bool,
}

/// How a pixel's samples choose their random numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sampler {
    /// Independent numbers, with the pixel position jittered on an n x n grid.
    Independent,
    /// Owen-scrambled Sobol' points across every dimension (`sobol`).
    Sobol,
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
    /// Radiance arriving from direction `rd` (at the sample's wavelengths, in spectral mode).
    pub fn radiance(&self, rd: &Vector3<f64>) -> Color {
        self.rgb_radiance(rd).as_illuminant()
    }

    fn rgb_radiance(&self, rd: &Vector3<f64>) -> Color {
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
///
/// Filters are separable, f(x, y) = f(x) f(y), so each axis is sampled on its own. The box and
/// tent are sampled exactly, with equal weights. The others are tabulated (`TabulatedFilter`):
/// an offset is drawn in proportion to |f|, piecewise constant over the table, and the sample
/// weighted by f / (p ∫f), which is exact for f itself, and negative where f is: the
/// Mitchell and Lanczos filters' negative lobes sharpen edges (PBR section 8.8.2, whose
/// `FilterSampler` does the same over a 2D table).
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// Samples spread evenly over the pixel: an average of its own area.
    Box,
    /// A tent (triangle) of radius one pixel, falling linearly to zero at the neighbouring pixel
    /// centres (PBR section 8.8.4): samples reach into neighbouring pixels, weighted towards the
    /// centre. Smoother edges than the box, and the filter Bitterli's references use.
    Tent,
    /// Gaussian, Mitchell-Netravali or Lanczos, sampled from a table.
    Tabulated(std::sync::Arc<TabulatedFilter>),
}

impl Filter {
    /// Map a uniform `u` in [0, 1) to an offset from the pixel's centre, in pixels, distributed
    /// as the filter (as |f| for the tabulated filters, which then weight the sample:
    /// `sample_weighted`).
    pub fn sample(&self, u: f64) -> f64 {
        self.sample_weighted(u).0
    }

    /// An offset from the pixel's centre, in pixels, and the sample's weight along this axis.
    pub fn sample_weighted(&self, u: f64) -> (f64, f64) {
        match self {
            Filter::Box => (u - 0.5, 1.),
            // Inverting the tent's cumulative distribution on [-1, 1].
            Filter::Tent => (if u < 0.5 { (2. * u).sqrt() - 1. } else { 1. - (2. - 2. * u).sqrt() }, 1.),
            Filter::Tabulated(t) => t.sample(u),
        }
    }
}

/// The shape of a tabulated filter, in pixels from the centre, out to `radius`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterShape {
    /// A Gaussian of standard deviation `sigma`, less its value at the radius so that it falls
    /// to zero there (PBR section 8.8.5).
    Gaussian { sigma: f64 },
    /// Don P. Mitchell and Arun N. Netravali, "Reconstruction Filters in Computer Graphics",
    /// SIGGRAPH 1988, pp. 221-228, doi:10.1145/54852.378514: the cubic family of parameters B
    /// and C, scaled to the radius; B = C = 1/3 is the pair they recommend, between blurring and
    /// ringing (PBR section 8.8.6).
    Mitchell { b: f64, c: f64 },
    /// The sinc function, the ideal low-pass filter, windowed by a wider sinc: Lanczos's window,
    /// sinc(x) sinc(x / τ) (PBR section 8.8.7).
    Lanczos { tau: f64 },
}

impl FilterShape {
    /// f(x) for a filter of radius `radius`.
    pub fn eval(&self, x: f64, radius: f64) -> f64 {
        if x.abs() >= radius {
            return 0.;
        }
        match *self {
            FilterShape::Gaussian { sigma } => {
                let g = |x: f64| (-x * x / (2. * sigma * sigma)).exp();
                (g(x) - g(radius)).max(0.)
            },
            FilterShape::Mitchell { b, c } => {
                let x = (2. * x / radius).abs();
                if x <= 1. {
                    ((12. - 9. * b - 6. * c) * x.powi(3) + (-18. + 12. * b + 6. * c) * x * x + (6. - 2. * b)) / 6.
                } else {
                    ((-b - 6. * c) * x.powi(3) + (6. * b + 30. * c) * x * x + (-12. * b - 48. * c) * x + (8. * b + 24. * c)) / 6.
                }
            },
            FilterShape::Lanczos { tau } => {
                let sinc = |x: f64| if x.abs() < 1e-9 { 1. } else { (std::f64::consts::PI * x).sin() / (std::f64::consts::PI * x) };
                sinc(x) * sinc(x / tau)
            },
        }
    }
}

/// A filter sampled from a table of |f| (see `Filter`).
#[derive(Debug, Clone, PartialEq)]
pub struct TabulatedFilter {
    pub shape: FilterShape,
    pub radius: f64,
    /// Cumulative |f| at the edges of `TABLE` equal cells over [-radius, radius], normalised.
    cdf: Vec<f64>,
    /// ∫ |f| and ∫ f over the table, by its cells.
    abs_integral: f64,
    integral: f64,
}

impl TabulatedFilter {
    const TABLE: usize = 1024;

    pub fn new(shape: FilterShape, radius: f64) -> TabulatedFilter {
        assert!(radius > 0., "A filter's radius must be positive, got {}", radius);
        let n = Self::TABLE;
        let width = 2. * radius / n as f64;
        let mut cdf = vec![0.; n + 1];
        let (mut abs_integral, mut integral) = (0., 0.);
        for i in 0..n {
            let f = shape.eval(-radius + (i as f64 + 0.5) * width, radius);
            abs_integral += f.abs() * width;
            integral += f * width;
            cdf[i + 1] = abs_integral;
        }
        assert!(integral > 0., "Filter {:?} of radius {} has no positive area", shape, radius);
        for v in cdf.iter_mut() {
            *v /= abs_integral;
        }
        TabulatedFilter { shape, radius, cdf, abs_integral, integral }
    }

    /// An offset drawn as the table's |f|, and its weight f / (p ∫f).
    fn sample(&self, u: f64) -> (f64, f64) {
        let n = Self::TABLE;
        let i = self.cdf.partition_point(|c| *c <= u).clamp(1, n) - 1;
        let span = self.cdf[i + 1] - self.cdf[i];
        let within = if span > 0. { (u - self.cdf[i]) / span } else { 0.5 };
        let width = 2. * self.radius / n as f64;
        let x = -self.radius + (i as f64 + within) * width;
        // The table's density there, per pixel.
        let pdf = span / width;
        if pdf <= 0. {
            return (x, 0.);
        }
        (x, self.shape.eval(x, self.radius) / (pdf * self.integral))
    }

    /// ∫ |f| / ∫ f: one for a non-negative filter, more for one with negative lobes.
    pub fn negative_lobe_factor(&self) -> f64 {
        self.abs_integral / self.integral
    }
}

impl RenderOpts {
    /// Samples per pixel in each pass over the image: n x n, n = samples_per_chunk. Passes only
    /// group the work; the samples themselves don't depend on them.
    pub fn samples_per_layer(&self) -> usize {
        self.samples_per_chunk.max(1).pow(2)
    }

    /// Passes over the image needed to take `supersamples` samples per pixel.
    pub fn layers(&self) -> usize {
        self.samples_per_pixel().div_ceil(self.samples_per_layer())
    }

    /// Samples per pixel: `supersamples`, at least one.
    pub fn samples_per_pixel(&self) -> usize {
        self.supersamples.max(1)
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
    /// The chance of choosing each area light to sample: in proportion to its power
    /// (`SceneFile::light_powers`).
    pub area_light_choice: crate::environment::Distribution1D,
    /// A directional light, sampled directly by diffuse surfaces and media.
    pub sun: Option<Sun>,
    /// The skysphere's atmosphere, if any, for aerial perspective: light from distant objects
    /// is attenuated and hazed by the air it crosses.
    pub atmosphere: Option<Arc<SkyMaterial>>,
    /// The sky as a light to sample (`environment`), if it isn't black and `sample_sky` is on.
    pub environment: Option<crate::environment::Environment>,
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

    /// Weighted samples of a tabulated filter estimate ∫ g f / ∫ f, for any g: here the filter's
    /// own moments, against numerical integration.
    #[test]
    fn test_tabulated_filters_are_unbiased() {
        for (shape, radius) in [
            (FilterShape::Gaussian { sigma: 0.5 }, 1.5),
            (FilterShape::Mitchell { b: 1. / 3., c: 1. / 3. }, 2.),
            (FilterShape::Lanczos { tau: 3. }, 3.),
        ] {
            let f = Filter::Tabulated(std::sync::Arc::new(TabulatedFilter::new(shape, radius)));
            let g = |x: f64| x * x + 0.3 * (2. * x).cos();
            let m = 200_000;
            let exact: (f64, f64) = (0..m).fold((0., 0.), |(a, b), i| {
                let x = -radius + (i as f64 + 0.5) * 2. * radius / m as f64;
                let w = shape.eval(x, radius);
                (a + g(x) * w, b + w)
            });
            let exact = exact.0 / exact.1;
            let n = 400_000;
            let estimate = (0..n).map(|i| {
                let (x, w) = f.sample_weighted((i as f64 + 0.5) / n as f64);
                assert!(x.abs() <= radius);
                g(x) * w
            }).sum::<f64>() / n as f64;
            assert!((estimate - exact).abs() < 2e-3, "{:?}: {} vs {}", shape, estimate, exact);
        }
    }

    #[test]
    fn test_filter_shapes() {
        // Mitchell-Netravali with B = C = 1/3: 8/9 at the centre, zero at the radius, and a
        // negative lobe; Lanczos: one at the centre, zero at whole pixels.
        let mitchell = FilterShape::Mitchell { b: 1. / 3., c: 1. / 3. };
        assert!((mitchell.eval(0., 2.) - 8. / 9.).abs() < 1e-12);
        assert!(mitchell.eval(1.5, 2.) < 0. && mitchell.eval(2., 2.) == 0.);
        let lanczos = FilterShape::Lanczos { tau: 3. };
        assert!((lanczos.eval(0., 3.) - 1.).abs() < 1e-12 && lanczos.eval(1., 3.).abs() < 1e-12);
        assert!(TabulatedFilter::new(lanczos, 3.).negative_lobe_factor() > 1.);
        assert!((TabulatedFilter::new(FilterShape::Gaussian { sigma: 0.5 }, 1.5).negative_lobe_factor() - 1.).abs() < 1e-12);
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
