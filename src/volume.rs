//! Bounded participating media whose density varies through space, such as smoke or cloud.
//!
//! Light travelling through a medium of extinction coefficient σ(x) collides within a short
//! step dt with probability σ(x) dt, so the chance of reaching distance t without colliding is
//! the transmittance exp(-∫₀ᵗ σ). When σ is constant, a collision distance can be drawn directly
//! (see `participatingmedia::HomogenousFog`). When it varies, the integral has no closed form,
//! and `Volume` instead uses delta tracking (E. R. Woodcock et al., "Techniques used in the GEM
//! code for Monte Carlo neutronics calculations in reactors and other systems of complex
//! geometry", Argonne National Laboratory, ANL-7050, 1965; see also Novák et al., "Monte Carlo
//! Methods for Volumetric Light Transport Simulation", Computer Graphics Forum 37(2), 2018,
//! section 4.2):
//!
//! 1. Fill the medium up to a constant majorant σ̄ ≥ σ(x) everywhere with "null" particles that
//!    don't change the light's direction.
//! 2. Sample collisions in that homogeneous medium: steps of -ln(1 - u) / σ̄.
//! 3. At each, the collision is real with probability σ(x) / σ̄; otherwise it was a null
//!    collision, and the walk continues.
//!
//! The first real collision is distributed exactly as in the heterogeneous medium, without any
//! step size to choose. Its cost is the number of null collisions, so the majorant should be
//! tight: mostly-empty volumes with a few dense spots are slow.
//!
//! As geometry, a volume reports the first real collision as its "intersection". Nearer
//! surfaces hide it, and a shadow ray is blocked by a volume exactly when it collides in it,
//! which estimates the transmittance unbiasedly, if noisily, as either 0 or 1.

use crate::material::model::LightSampling;
use crate::color::Color;
use std::cell::Cell;
use crate::geometry::rand;
use crate::intersection::{Intersection, RawIntersection};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::phase::PhaseFunction;
use crate::material::diffraction::DiffractionTable;
use crate::material::ice_halos::{Crystals, HaloTable};
use crate::material::rainbow::DropletPhase;
use std::sync::OnceLock;
use std::sync::Arc;
use crate::material::texture::Solid;
use crate::na::Vector3;
use crate::noise::PerlinNoise;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

pub mod chromatic;
pub mod emission;
pub mod grid;

thread_local! {
    /// The state of the path being traced on this thread, for the multiple-scattering
    /// approximations (`ScatteringApproximation`): how many times it has scattered in a
    /// volume, and the factor all volumes' densities are scaled by for its next segments,
    /// shadow rays included. Geometry is intersected without knowing which path it serves,
    /// hence a thread-local; each thread traces one path at a time, from `begin_path`.
    static PATH: Cell<(u32, f64)> = const { Cell::new((0, 1.)) };

    /// How many vertices the path has scattered at diffusely or glossily (not a mirror or
    /// glass), counting the one it is at: once it has left one, its segments cross the media
    /// truncated (`Volume::truncated`), even before it scatters in one.
    static VERTICES: Cell<u32> = const { Cell::new(0) };
    /// Whether the path has gathered the sunlight diffused through a medium with the diffusion
    /// approximation (`ScatteringApproximation::diffusion`), which stands for all its later
    /// scatterings' sunlight.
    static DIFFUSED: Cell<bool> = const { Cell::new(false) };
}

/// Start a new path: no volume scatterings yet, and densities as given.
pub fn begin_path() {
    PATH.with(|p| p.set((0, 1.)));
    VERTICES.with(|v| v.set(0));
    DIFFUSED.with(|d| d.set(false));
    chromatic::begin_path();
}

/// Take up a path kept elsewhere (the wavefront's, whose vertex the host shades:
/// `wavefront::integrator::host_bounce`): it has scattered `scatterings` times in media, which
/// left their densities scaled by `scale` for its segments, and has or hasn't `left` a vertex
/// that isn't a mirror's or glass's.
pub fn resume_path(scatterings: u32, scale: f64, left: bool) {
    begin_path();
    PATH.with(|p| p.set((scatterings, scale)));
    VERTICES.with(|v| v.set(left as u32));
}

/// The path is at a vertex, about to scatter: its shadow rays and its continuation leave it.
/// A specular vertex is then taken back (`specular_vertex`), so that what the camera sees in a
/// mirror or through glass is seen as directly.
pub fn at_vertex() {
    VERTICES.with(|v| v.set(v.get() + 1));
}

/// The vertex just scattered specularly: it doesn't count.
pub fn specular_vertex() {
    VERTICES.with(|v| v.set(v.get().saturating_sub(1)));
}

/// Whether the path's segments now leave a vertex.
fn left_vertex() -> bool {
    VERTICES.with(|v| v.get()) > 0
}

/// Whether the path left a vertex before the one it is at.
fn left_earlier_vertex() -> bool {
    VERTICES.with(|v| v.get()) > 1
}

fn path_density_scale() -> f64 {
    PATH.with(|p| p.get().1)
}

impl Volume {
    /// The factor this volume's density is scaled by for the current segment: as the path's
    /// last volume scattering set it, or before any, this volume's own truncation once the path
    /// has left a vertex, and as given on the camera's own rays.
    fn segment_scale(&self) -> f64 {
        match PATH.with(|p| p.get()) {
            (0, _) if left_vertex() => self.truncated,
            (0, _) => 1.,
            (_, scale) => scale,
        }
    }
}

fn path_scatterings() -> u32 {
    PATH.with(|p| p.get().0)
}

fn set_path(scatterings: u32, scale: f64) {
    PATH.with(|p| p.set((scatterings, scale)));
}

/// Below this estimated transmittance, ratio tracking plays Russian roulette. Survivors carry
/// this much, so it is also the size of the speckles in a deep shadow, whose mean is far less:
/// small, so that sunlight through a cloud's edge, or under it, isn't a scatter of bright
/// grains of 0 or this (at 0.3, the ground under a cloud speckled like gravel).
const RATIO_TRACKING_ROULETTE: f64 = 0.02;

/// Ratio tracking samples collisions against this multiple of the residual majorant. Against
/// the majorant itself, a collision where the density reaches it (a cloud's core) multiplies
/// the estimate by 0, and ratio tracking is no better than delta tracking's 0 or 1; against
/// twice it, each factor is at least a half, at the cost of twice the steps (Novák, Selle and
/// Jarosz 2014, section 5: looser bounds trade steps for variance, towards exp(-∫σ) exactly).
const RATIO_TRACKING_SLACK: f64 = 2.;

/// A density field as the shared walks take one (`shared::media::Medium`).
pub(crate) struct Field<'a>(pub(crate) &'a dyn DensityField);

impl shared::media::Medium<f64> for Field<'_> {
    #[inline]
    fn density(&self, x: f64, y: f64, z: f64) -> f64 {
        self.0.density(&Vector3::new(x, y, z))
    }

    #[inline]
    fn bound(&self) -> f64 {
        self.0.majorant()
    }

    #[inline]
    fn majorant_from(&self, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> (f64, f64) {
        self.0.majorant_from(&Vector3::new(x, y, z), &Vector3::new(dx, dy, dz))
    }
}

/// The path's random numbers (`geometry::rand`), as the shared walks draw them.
struct PathRandom;

impl shared::Random<f64> for PathRandom {
    #[inline]
    fn next(&mut self) -> f64 {
        rand()
    }
}

/// The extinction coefficient through a volume: the chance of a collision per unit distance.
pub trait DensityField: Sync + Send {
    fn density(&self, p: &Vector3<f64>) -> f64;

    /// An upper bound on `density` everywhere in the volume. Delta tracking is only correct if
    /// it really is a bound, and is fastest if it is close.
    fn majorant(&self) -> f64;

    /// A local bound: an upper bound on `density` from `p` along the unit direction `d`, and how
    /// far along it holds. Where a field knows it is empty or thin, tracking takes larger steps
    /// there, or skips it altogether. By default, `majorant()` everywhere.
    fn majorant_from(&self, _p: &Vector3<f64>, _d: &Vector3<f64>) -> (f64, f64) {
        (self.majorant(), f64::INFINITY)
    }

    /// The optical depths of the vertical column through `p` from height `lo` to `hi`, below
    /// `p` and above it, from `steps` jittered steps (for `ScatteringApproximation::diffusion`).
    /// Fields whose density is largely the same all the way up a column can work it out once.
    fn column_depths(&self, p: &Vector3<f64>, lo: f64, hi: f64, steps: u32) -> (f64, f64) {
        let step = (hi - lo) / steps as f64;
        let (mut below, mut above) = (0., 0.);
        for k in 0..steps {
            let y = lo + (k as f64 + rand()) * step;
            let tau = self.density(&Vector3::new(p.x, y, p.z)) * step;
            if y < p.y { below += tau } else { above += tau }
        }
        (below, above)
    }

    /// A lower bound on `density` everywhere in the volume (by default 0). Transmittance
    /// through this much is computed exactly, and only the rest estimated.
    fn minorant(&self) -> f64 {
        0.
    }

    /// The field as data, for the kernels that have its code (`FieldExport`); None for one that
    /// only this code can evaluate.
    fn export(&self) -> Option<FieldExport> {
        None
    }

    /// The mean radius (micrometres) of the particles at `p`, for media whose particle size
    /// varies; None to use the material's.
    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        None
    }
}

/// The same density throughout.
pub struct ConstantDensity(pub f64);

impl DensityField for ConstantDensity {
    fn export(&self) -> Option<FieldExport> { Some(FieldExport::Constant(self.0)) }
    fn density(&self, _p: &Vector3<f64>) -> f64 { self.0 }
    fn majorant(&self) -> f64 { self.0 }
    fn minorant(&self) -> f64 { self.0 }
}

/// Density from a function graph (`graph.rs`), its value the extinction per unit distance,
/// at least 0; bounded above by the graph's range, as delta tracking needs. With `fields` (a
/// landscape's), its `field` nodes read the simulation, and `altitude` and `slope` go by the
/// planet's up: fog that lies in the valleys, or thins with height above the ground.
pub struct GraphDensity {
    pub graph: crate::graph::Graph,
    pub fields: Option<std::sync::Arc<dyn crate::graph::Fields>>,
}

impl GraphDensity {
    pub fn new(graph: crate::graph::Graph, fields: Option<std::sync::Arc<dyn crate::graph::Fields>>) -> Result<GraphDensity, String> {
        let hi = graph.range().1;
        if !hi.is_finite() {
            return Err("a volume's density graph must be bounded above: give an expression a \"range\", a field \"from\" and \"to\"".into());
        }
        Ok(GraphDensity { graph, fields })
    }
}

impl DensityField for GraphDensity {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let c = crate::graph::Context::new(*p, Vector3::y(), 0.).with_fields(self.fields.as_deref());
        self.graph.eval(&c).max(0.)
    }
    fn majorant(&self) -> f64 { self.graph.range().1.max(0.) }
    fn minorant(&self) -> f64 { self.graph.range().0.max(0.) }
}

/// Density from fractal Perlin noise: `max_density` where the noise is highest, falling to
/// nothing where it is below `threshold`, so the volume breaks up into separate puffs.
pub struct NoiseDensity {
    pub perlin: PerlinNoise,
    pub max_density: f64,
    /// Noise features are about 1 / scale across.
    pub scale: f64,
    pub octaves: u32,
    /// Noise values (in [0, 1]) below this are empty space.
    pub threshold: f64,
}

impl DensityField for NoiseDensity {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let q = p * self.scale;
        let n = self.perlin.fbm(q.x, q.y, q.z, self.octaves, 0.5, 2.0);
        self.max_density * ((n - self.threshold) / (1. - self.threshold)).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { self.max_density }
}

/// Several density fields filling one volume, their densities added: media lit together, as
/// one column for the diffusion approximation (an overcast deck and the hill cloud under it).
pub struct Sum {
    pub fields: Vec<Arc<dyn DensityField>>,
}

impl DensityField for Sum {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.fields.iter().map(|f| f.density(p)).sum()
    }
    fn majorant(&self) -> f64 {
        self.fields.iter().map(|f| f.majorant()).sum()
    }
    /// The sum of the parts' bounds, as far as all of them hold.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        self.fields.iter().map(|f| f.majorant_from(p, d)).fold((0., f64::INFINITY), |(m, l), (fm, fl)| (m + fm, l.min(fl)))
    }
    fn minorant(&self) -> f64 {
        self.fields.iter().map(|f| f.minorant()).sum()
    }
    fn column_depths(&self, p: &Vector3<f64>, lo: f64, hi: f64, steps: u32) -> (f64, f64) {
        self.fields.iter().map(|f| f.column_depths(p, lo, hi, steps)).fold((0., 0.), |(b, a), (fb, fa)| (b + fb, a + fa))
    }
    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> {
        self.fields.iter().find_map(|f| f.particle_radius(p))
    }
}

/// A density field times a constant.
pub struct Scaled {
    pub field: Arc<dyn DensityField>,
    pub by: f64,
}

impl DensityField for Scaled {
    fn export(&self) -> Option<FieldExport> { self.field.export().and_then(|f| f.scaled(self.by)) }
    fn density(&self, p: &Vector3<f64>) -> f64 { self.by * self.field.density(p) }
    fn majorant(&self) -> f64 { self.by * self.field.majorant() }
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let (m, l) = self.field.majorant_from(p, d);
        (self.by * m, l)
    }
    fn minorant(&self) -> f64 { self.by * self.field.minorant() }
    fn column_depths(&self, p: &Vector3<f64>, lo: f64, hi: f64, steps: u32) -> (f64, f64) {
        let (below, above) = self.field.column_depths(p, lo, hi, steps);
        (self.by * below, self.by * above)
    }
    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> { self.field.particle_radius(p) }
}

/// A participating medium filling a box, or a closed shape within it.
pub struct Volume {
    pub bounds: BBox,
    pub density: Arc<dyn DensityField>,
    /// The closed shape the medium fills, within `bounds`; None to fill the box. This is pbrt's
    /// medium interface for a shape's interior (Pharr, Jakob and Humphreys, "Physically Based
    /// Rendering", 4th ed., section 11.4, `MediumInterface`): a glass of coloured liquid is a
    /// dielectric surface and a volume in the same shape, the surface refracting light in and
    /// the medium absorbing it on its way through.
    pub shape: Option<Box<dyn Geometry + Sync + Send>>,
    /// For a chromatic medium, the extinction per unit density in each channel (RGB, upsampled
    /// at the sample's wavelengths in spectral mode); None when it is the same in all, the
    /// density alone. Tracked by spectral MIS (`chromatic`).
    pub chromatic: Option<Color>,
    /// The density scale for segments that leave a vertex before the path has scattered in a
    /// medium (a surface's shadow rays, and the light it scatters), which the medium's
    /// material sets from its truncations (`VolumeMaterial::density_scale`, from its first
    /// truncated generation): light reaching a surface past a cloud's edge has been scattered
    /// by small angles as often as not, as it has on its way to a cloud's vertex. 1 by default.
    pub truncated: f64,
}

/// The stretches of a ray inside a volume: one for a box, any number for a shape.
enum Spans {
    One(Option<(f64, f64)>),
    Many(std::vec::IntoIter<(f64, f64)>),
}

impl Iterator for Spans {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<(f64, f64)> {
        match self {
            Spans::One(s) => s.take(),
            Spans::Many(i) => i.next(),
        }
    }
}

impl Volume {
    pub fn new(bounds: BBox, density: Arc<dyn DensityField>) -> Volume {
        Volume { bounds, density, shape: None, chromatic: None, truncated: 1. }
    }

    /// A medium filling the inside of a closed shape.
    pub fn in_shape(shape: Box<dyn Geometry + Sync + Send>, density: Arc<dyn DensityField>) -> Volume {
        Volume { bounds: shape.bounds(), density, shape: Some(shape), chromatic: None, truncated: 1. }
    }

    /// Where the ray from `ro` along the unit direction `d` is in the medium: (entry, exit)
    /// distances, nearest first. Entry may be behind the origin, if it starts inside. In a shape,
    /// from where it crosses the surface inwards to where it crosses outwards, the surface's
    /// outward normals telling which is which; a ray whose first crossing is outwards started
    /// inside.
    fn spans(&self, ro: &Vector3<f64>, d: &Vector3<f64>) -> Spans {
        let invrd = Vector3::new(1. / d.x, 1. / d.y, 1. / d.z);
        let (entry, exit) = match self.bounds.entry_exit(ro, &invrd) {
            Some(x) => x,
            None => return Spans::One(None),
        };
        let shape = match &self.shape {
            None => return Spans::One(Some((entry, exit))),
            Some(s) => s,
        };
        let mut spans = Vec::new();
        let mut start = None;
        for (k, c) in shape.crossings(&Ray { ro: *ro, rd: *d }).iter().enumerate() {
            let entering = c.normal.dot(d) < 0.;
            match start {
                None if k == 0 && !entering => spans.push((0., c.dist)),
                None if entering => start = Some(c.dist),
                Some(s) if !entering => {
                    spans.push((s, c.dist));
                    start = None;
                },
                _ => (),
            }
        }
        Spans::Many(spans.into_iter())
    }
}

impl Geometry for Volume {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_beyond(r, 0.)
    }

    /// The first real collision at least `min` along the ray, by delta tracking. Light that
    /// hasn't collided by `min` continues as if it had just started there, as the chance of
    /// colliding further on doesn't depend on how far it has already come.
    ///
    /// The majorant may vary along the ray (`DensityField::majorant_from`): within each stretch
    /// it is constant, and a step that would leave the stretch is abandoned at its end and
    /// resampled with the next one's majorant, which by the same memorylessness is exact (the
    /// piecewise-constant majorants of Novák et al. 2018). Stretches with no majorant are
    /// skipped without sampling at all.
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        if let Some(k) = self.chromatic {
            return self.intersects_chromatic(r, min, &k.as_unbounded().rgb);
        }
        let scale = self.segment_scale();
        if self.density.majorant() * scale <= 0. {
            return None;
        }
        let d = r.rd.normalize();
        // The walk is `shared::media::delta_track`, which a kernel of the GPU's compiles too.
        let (medium, o, dir) = (Field(&*self.density), (r.ro.x, r.ro.y, r.ro.z), (d.x, d.y, d.z));
        for (entry, exit) in self.spans(&r.ro, &d) {
            if let Some(t) = shared::media::delta_track(&medium, o, dir, entry.max(min), exit, scale, &mut PathRandom) {
                // Media have no surface; facing back along the ray keeps the shadow bias from
                // moving the point sideways.
                return Some(RawIntersection { face: 0, part: 0, dist: t, point: r.ro + d * t, normal: -d });
            }
        }
        None
    }

    /// Residual ratio tracking (Jan Novák, Andrew Selle and Wojciech Jarosz, "Residual Ratio
    /// Tracking for Estimating Attenuation in Participating Media", ACM Transactions on
    /// Graphics 33(6), SIGGRAPH Asia 2014).
    ///
    /// Ratio tracking takes the same tentative collisions as delta tracking, but rather than
    /// stopping at the first real one, with probability σ(x) / σ̄, it carries on and multiplies
    /// the estimate by the chance of the collision being null, 1 - σ(x) / σ̄. The expectation is
    /// still the transmittance exp(-∫σ), but each estimate is a fraction rather than 0 or 1, so
    /// shadows through media are far less noisy. Where the density reaches the majorant the
    /// factor is 0, though, and through a homogeneous medium it is no better than 0 or 1.
    ///
    /// So, "residual": a constant lower bound σc (`DensityField::minorant`) is attenuated
    /// exactly, exp(-σc L), and only the residual σ(x) - σc above it is tracked, against the
    /// majorant σ̄ - σc. Homogeneous media need no tracking at all.
    ///
    /// The walk must cross the whole stretch rather than stopping at a collision; once the
    /// estimate is small, Russian roulette ends most walks early (and doubles the survivors),
    /// which stays unbiased.
    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        let scale = self.segment_scale();
        if self.density.majorant() * scale <= 0. {
            return 1.;
        }
        let d = r.rd.normalize();
        let control = self.density.minorant();
        let mut transmitted = 1.;
        // The walk is `shared::media::ratio_track`.
        let (medium, o, dir) = (Field(&*self.density), (r.ro.x, r.ro.y, r.ro.z), (d.x, d.y, d.z));
        for (entry, exit) in self.spans(&r.ro, &d) {
            let stop = exit.min(max);
            let t = entry.max(min);
            if t >= stop {
                continue;
            }
            match shared::media::ratio_track(&medium, o, dir, t, stop, control, scale, transmitted, &mut PathRandom) {
                Some(left) => transmitted = left,
                None => return 0.,
            }
        }
        transmitted
    }

    /// Per channel: for a chromatic medium, ratio tracking against the majorant of the channel
    /// with the most extinction, which estimates every channel's transmittance at once.
    fn transmittance_color(&self, r: &Ray, min: f64, max: f64) -> Color {
        let k = match self.chromatic {
            None => return Color::white() * self.transmittance(r, min, max),
            Some(k) => k.as_unbounded().rgb,
        };
        let scale = self.segment_scale();
        let kmax = k.max();
        if self.density.majorant() * scale * kmax <= 0. {
            return Color::white();
        }
        let d = r.rd.normalize();
        let control = self.density.minorant();
        let mut transmitted = Vector3::new(1., 1., 1.);
        for (entry, exit) in self.spans(&r.ro, &d) {
            let stop = exit.min(max);
            let mut t = entry.max(min);
            if t >= stop {
                continue;
            }
            transmitted = transmitted.component_mul(&k.map(|k| (-control * k * scale * (stop - t)).exp()));
            while t < stop {
                let (majorant, length) = self.density.majorant_from(&(r.ro + d * t), &d);
                let end = (t + length.max(t.abs().max(1.) * 1e-12)).min(stop);
                let residual = (majorant - control) * RATIO_TRACKING_SLACK;
                if residual > 0. {
                    let step = -(1. - rand()).ln() / (residual * kmax * scale);
                    if t + step < end {
                        t += step;
                        let excess = (self.density.density(&(r.ro + d * t)) - control) / (residual * kmax);
                        transmitted = transmitted.component_mul(&k.map(|k| 1. - excess * k));
                        let largest = transmitted.max();
                        if largest < RATIO_TRACKING_ROULETTE {
                            if rand() * RATIO_TRACKING_ROULETTE >= largest {
                                return Color::black();
                            }
                            transmitted *= RATIO_TRACKING_ROULETTE / largest;
                        }
                        continue;
                    }
                }
                t = end;
            }
        }
        Color { rgb: transmitted }
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn is_stochastic(&self) -> bool { true }

    fn medium(&self) -> Option<&Volume> { Some(self) }
}

/// A density field as data: the kinds a kernel of the GPU's can evaluate, with the shared
/// crate's code (`pathtrace::KernelMedium`).
pub enum FieldExport {
    /// The same extinction everywhere.
    Constant(f64),
    /// A Nubis cloud layer's record (`NubisCloudLayer::export`), flat, or curved round a planet
    /// of `radius` (`planet::PlanetMapping`) where that isn't 0.
    Nubis { record: Vec<f32>, radius: f64 },
    /// A layer of altocumulus's record (`BenardAltocumulus::export`), flat or round a planet
    /// likewise.
    Benard { record: Vec<f32>, radius: f64 },
    /// A layer of cirrus's record (`HoganKewCirrus::export`), likewise.
    Cirrus { record: Vec<f32>, radius: f64 },
    /// Precipitation's record (`Precipitation::export`), likewise.
    Rain { record: Vec<f32>, radius: f64 },
    /// Mammatus's record (`Mammatus::export`), likewise.
    Mammatus { record: Vec<f32>, radius: f64 },
}

impl FieldExport {
    /// With every density multiplied by `by`, where the record has a number that does that.
    fn scaled(self, by: f64) -> Option<FieldExport> {
        Some(match self {
            FieldExport::Constant(d) => FieldExport::Constant(d * by),
            FieldExport::Nubis { mut record, radius } => {
                // Its densest extinction, which the rest is a fraction of.
                record[7] *= by as f32;
                FieldExport::Nubis { record, radius }
            },
            FieldExport::Benard { mut record, radius } => {
                record[2] *= by as f32;
                FieldExport::Benard { record, radius }
            },
            FieldExport::Cirrus { mut record, radius } => {
                record[2] *= by as f32;
                FieldExport::Cirrus { record, radius }
            },
            // (Its extinction isn't a multiple of anything in its record.)
            FieldExport::Rain { .. } => return None,
            FieldExport::Mammatus { mut record, radius } => {
                // The anvil's density and the pouches'.
                record[8] *= by as f32;
                record[9] *= by as f32;
                FieldExport::Mammatus { record, radius }
            },
        })
    }
}

/// A medium as data, for the wavefront's flat scene (`wavefront::flat::FlatScene::media`): what
/// the GPU's kernels track paths through and scatter them by (`pathtrace::media_collision`,
/// its `shade`).
pub struct FlatMedium {
    pub bounds: BBox,
    pub field: FieldExport,
    /// A lower bound on the extinction (`DensityField::minorant`).
    pub lower: f64,
    /// `Volume::truncated`: the factor on densities for segments of a path that has scattered
    /// here, and on this medium's for one that has left a surface.
    pub scale: f64,
    pub phase: PhaseFunction,
    pub albedo: Color,
    /// The fraction of scattering in the forward peak that truncated vertices leave out
    /// (`VolumeMaterial::forward_peak`).
    pub peak: f64,
    /// The similarity relations (`ScatteringApproximation::similarity_after`): how many
    /// scatterings they begin at, and the factor on densities from then on.
    pub reduced: Option<(u32, f64)>,
    /// The mean cosine of the scattering angle, whole and truncated.
    pub mean_cosine: (f64, f64),
    /// The Oz approximation of the sunlight scattered many times, if the medium takes it.
    pub oz: Option<Oz>,
}

impl Volume {
    /// The medium as data, scattering as `material` says; or why a kernel couldn't render it:
    /// they have the plain path tracing of a grey medium filling its box, with delta-M
    /// truncation, the similarity relations and the Oz approximation, lit by the sun, the lights and the sky. Sun and glitter probes they leave
    /// out: those change a render's noise, not its mean.
    pub fn flat(&self, material: &VolumeMaterial) -> Result<FlatMedium, String> {
        let a = &material.approximation;
        let unported = [
            (self.shape.is_some(), "it fills a shape"),
            (self.chromatic.is_some() || material.extinction.is_some(), "it is chromatic"),
            (material.diffraction.is_some(), "diffraction"),
            (material.halos.is_some(), "halos"),
            (material.rainbow.is_some(), "rainbows"),
            (material.emission.is_some(), "it emits"),
            (a.diffusion.is_some(), "the diffusion approximation"),
        ];
        if let Some((_, why)) = unported.iter().find(|(is, _)| *is) {
            return Err(why.to_string());
        }
        let field = self.density.export().ok_or("its density")?;
        Ok(FlatMedium {
            bounds: self.bounds, field, lower: self.density.minorant(), scale: self.truncated,
            phase: material.phase, albedo: material.albedo.as_reflectance(), peak: material.forward_peak(1),
            reduced: a.similarity_after.map(|n| (n, material.density_scale(n.max(1)))),
            mean_cosine: (material.mean_cosine(0), material.mean_cosine(1)),
            oz: a.oz,
        })
    }
}

impl Volume {
    /// Delta tracking with the path's hero channel through a medium of extinction `k` per unit
    /// density per channel, recording each tentative collision's probability ratios for the
    /// others (`chromatic`).
    fn intersects_chromatic(&self, r: &Ray, min: f64, k: &Vector3<f64>) -> Option<RawIntersection> {
        let scale = self.segment_scale();
        let kmax = k.max();
        if self.density.majorant() * scale * kmax <= 0. {
            return None;
        }
        let hero = chromatic::hero();
        let d = r.rd.normalize();
        for (entry, exit) in self.spans(&r.ro, &d) {
            let mut t = entry.max(min);
            while t < exit {
                let (majorant, length) = self.density.majorant_from(&(r.ro + d * t), &d);
                let end = (t + length.max(t.abs().max(1.) * 1e-12)).min(exit);
                if majorant > 0. {
                    let step = -(1. - rand()).ln() / (majorant * kmax * scale);
                    if t + step < end {
                        t += step;
                        let point = r.ro + d * t;
                        // Each channel's chance of the collision being real.
                        let p = k * (self.density.density(&point) / (majorant * kmax));
                        if rand() < p[hero] {
                            chromatic::record(t, [p.x / p[hero], p.y / p[hero], p.z / p[hero]], true);
                            return Some(RawIntersection { face: 0, part: 0, dist: t, point, normal: -d });
                        }
                        let n = 1. - p[hero];
                        chromatic::record(t, [(1. - p.x) / n, (1. - p.y) / n, (1. - p.z) / n], false);
                        continue;
                    }
                }
                t = end;
            }
        }
        None
    }
}

/// Approximations that trade exactness for speed and noise in thick, bright media like
/// clouds, where light scatters hundreds of times. All off by default: plain path tracing.
/// (Scene files turn `delta_m` on unless told otherwise.)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScatteringApproximation {
    /// Similarity relations from this many volume scatterings on. After a few scatterings,
    /// light in a forward-scattering medium is nearly isotropic anyway, and its spread is
    /// governed by the "reduced" coefficients: a medium of extinction σ, albedo α and mean
    /// cosine g behaves, at depth, like one scattering isotropically with extinction
    /// σ (1 - α g) and albedo α (1 - g) / (1 - α g) (D. R. Wyman, M. S. Patterson and
    /// B. C. Wilson, "Similarity relations for anisotropic scattering in Monte Carlo simulations
    /// of deeply penetrating neutral particles", Journal of Computational Physics 81, 1989; see
    /// Zhao, Ramamoorthi and Bala, "High-Order Similarity Relations in Radiative Transfer",
    /// SIGGRAPH 2014). For g = 0.85 that is about seven times fewer collisions, and no spiky
    /// phase function at the deep vertices that dominate the noise.
    pub similarity_after: Option<u32>,
    /// Magnus Wrenninge, Chris Kulla and Viktor Lundqvist, "Oz: The Great and Volumetric",
    /// SIGGRAPH 2013 Talks: approximate the sunlight scattered many times by a sum of single
    /// scatterings, octave i weighted b^i, with extinction scaled a^i (light penetrates
    /// further) and the phase function's anisotropy c^i (it spreads out). Applied at a
    /// path's first volume scattering; later ones then skip the sun, which would otherwise be
    /// counted twice, but still gather light from the sky and surroundings.
    pub oz: Option<Oz>,
    /// Delta-M truncation of a Henyey-Greenstein phase function wherever the camera doesn't see
    /// the medium directly (`VolumeMaterial::forward_peak`, `Volume::truncated`): the sharpest part of the forward peak,
    /// a fraction g² of scattering, is taken as no scattering at all, and the rest scatters by
    /// Henyey-Greenstein of mean cosine g / (1 + g). Light scattered through so small an angle
    /// carries on almost as if unscattered, so the light that reaches the camera hardly
    /// changes, but sunlight gathered at deep vertices no longer swings between almost none and
    /// eighty times the mean (for g = 0.85), the grain that dominates a cloud's noise. The
    /// first scattering, whose silver lining the camera sees, keeps the whole phase function.
    pub delta_m: bool,
    /// The diffusion approximation for thick decks of cloud, where sunlight reaches the
    /// underside only after hundreds of scatterings: a random walk rarely gets there, and when
    /// it does it is a firefly. Instead, at the first vertex of a path in the medium, the
    /// sunlight scattered many times is the diffusion solution for a plane-parallel slab
    /// (the "independent column approximation" of cloud radiation, e.g. R. F. Cahalan et
    /// al., "The albedo of fractal stratocumulus clouds", J. Atmos. Sci. 51, 1994) for the
    /// column of medium through the vertex, measured straight up and down. For reduced
    /// optical depths τ* = (1 - α g) τ of the whole column and of the part below the vertex,
    /// and the sun at cosine μ₀ from the zenith, the diffuse flux through a conservative
    /// slab is F = μ₀ K(μ₀) E / (1 + ¾ τ*_column), K(μ₀) = 3/7 (1 + 2 μ₀) its injection
    /// function, and the mean radiance at the vertex 3F/4π (τ*_below + ⅔): F/2π at the base,
    /// rising linearly into the cloud (A. Kokhanovsky, "Cloud Optics", 2006, ch. 3;
    /// G. L. Stephens, "Remote Sensing of the Lower Atmosphere", ch. 6). Light under a deck
    /// is as bright as the deck over it is thin, with no noise. The sunlight is attenuated by
    /// whatever lies above the column's top. Later vertices of the path in such a medium skip
    /// the sun, which the first has counted; they still gather the sky, and light off the
    /// ground. Columns are measured with `diffusion` jittered steps; the medium's box sets
    /// how far. Point and area lights (lightning in a thunderstorm) are diffused the same way,
    /// along the line to each light sample rather than a column (`diffused_gain`).
    pub diffusion: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oz {
    pub octaves: u32,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Default for Oz {
    /// Values in the range Wrenninge et al. and Hillaire ("Physically Based Sky, Atmosphere and
    /// Cloud Rendering in Frostbite", SIGGRAPH 2016 course) suggest; a ≤ b keeps energy bounded.
    fn default() -> Oz {
        Oz { octaves: 4, a: 0.5, b: 0.5, c: 0.5 }
    }
}

/// What happens at a collision in a volume: the light is scattered with probability `albedo`
/// (per colour channel), in a direction from the phase function, and otherwise absorbed.
/// Clouds scatter nearly everything (albedo ≈ 1) and mostly forwards (mean cosine ≈ 0.85);
/// smoke absorbs more.
pub struct VolumeMaterial {
    pub albedo: Color,
    /// The phase function; with diffraction, that of the light not diffracted.
    pub phase: PhaseFunction,
    pub approximation: ScatteringApproximation,
    pub diffraction: Option<Diffraction>,
    pub halos: Option<Halos>,
    /// Rainbows from raindrops (see `material::rainbow`), or small droplets' Mie scattering
    /// (`material::mie`: the glory and cloudbow): like halos, the drops' phase function for
    /// direct sunlight at a path's first scattering.
    pub rainbow: Option<Arc<dyn DropletPhase>>,
    /// The volume's density field, for the particle size where it has one.
    pub field: Arc<dyn DensityField>,
    /// Light the medium emits (`emission`).
    pub emission: Option<emission::VolumeEmission>,
    /// A chromatic medium's absorption and scattering coefficients per unit density (RGB), from
    /// which its albedo at the sample's wavelengths follows; `albedo` holds their RGB ratio.
    pub extinction: Option<(Color, Color)>,
    /// The lowest and highest the medium reaches (its box's), for the diffusion approximation's
    /// columns.
    pub column: (f64, f64),
    /// Whether its vertices send sun probes (`sun_probe`).
    pub sun_probes: bool,
}

/// Whether volumes send sun probes by default.
pub const SUN_PROBES: bool = true;

/// A continuation ray (origin and direction) and the weight of the sunlight at its next vertex.
type SunWeight = (Vector3<f64>, Vector3<f64>, f64);

thread_local! {
    /// The weight of the sunlight gathered at the next vertex along a path's continuation, from
    /// the vertex that sent it and a sun probe (`sun_probe`): the continuation ray (origin and
    /// direction, exactly) and the weight. A vertex reached by any other ray, after a surface
    /// say, which no probe could have reached, takes all of it.
    static NEXT_SUN_WEIGHT: Cell<Option<SunWeight>> = const { Cell::new(None) };
}

thread_local! {
    /// As `NEXT_SUN_WEIGHT`, for the sea's glitter at the continuation's next vertex, against a
    /// glitter probe's (`VolumeMaterial::glitter_probe`).
    static NEXT_GLINT_WEIGHT: Cell<Option<SunWeight>> = const { Cell::new(None) };
}

/// The weight of the sun's glitter seen on a sea reached by `r` (`ocean::PhysicalOcean`): all of
/// it, unless `r` left a medium that sent a glitter probe.
pub fn glint_weight_for(r: &Ray) -> f64 {
    NEXT_GLINT_WEIGHT.with(|w| match w.get() {
        Some((ro, rd, weight)) if ro == r.ro && rd == r.rd => weight,
        _ => 1.,
    })
}

/// The weight of the sunlight gathered at a volume vertex reached by `r`.
fn sun_weight_for(r: &Ray) -> f64 {
    NEXT_SUN_WEIGHT.with(|w| match w.get() {
        Some((ro, rd, weight)) if ro == r.ro && rd == r.rd => weight,
        _ => 1.,
    })
}

/// Halos from ice crystals (see `material::ice_halos`): sun dogs, the 22° and 46° halos,
/// tangent arcs. Their scattering depends on the direction the light comes from, so it is
/// traced for the scene's sun, `rays` rays a colour channel, when first needed, and used for
/// direct sunlight at a path's first scattering in the medium, where halos are seen: there it
/// replaces the base phase function (the part of scattering not diffracted). Elsewhere the
/// base phase function stands in for it.
pub struct Halos {
    pub crystals: Vec<Crystals>,
    pub rays: usize,
    pub seed: u64,
    pub table: OnceLock<HaloTable>,
}

/// Diffraction by the medium's particles (see `material::diffraction`): a `fraction` of the
/// scattered light (a half, for particles much larger than the wavelength) goes into the
/// wavelength-dependent diffraction lobe, whose width depends on the particles' mean radius
/// (micrometres): the density field's `particle_radius`, or else `radius`.
pub struct Diffraction {
    pub table: DiffractionTable,
    pub fraction: f64,
    pub radius: f64,
}

/// The wavelength (µm) whose diffraction lobe directions are sampled from.
const SAMPLED_WAVELENGTH: f64 = 0.55;

/// How light scatters at one vertex: a phase function, and diffraction by particles of a
/// radius. With diffraction, the phase function differs per colour channel.
struct VertexPhase<'a> {
    base: PhaseFunction,
    diffraction: Option<(&'a Diffraction, f64)>,
}

impl VertexPhase<'_> {
    fn eval(&self, cos: f64) -> Color {
        match self.diffraction {
            None => Color::white() * self.base.eval(cos),
            Some((d, radius)) => Color::white() * ((1. - d.fraction) * self.base.eval(cos))
                + d.table.eval_color(cos, radius) * d.fraction,
        }
    }

    /// The density of `sample`'s directions.
    fn pdf(&self, cos: f64) -> f64 {
        match self.diffraction {
            None => self.base.eval(cos),
            Some((d, radius)) => (1. - d.fraction) * self.base.eval(cos)
                + d.fraction * d.table.eval(cos, SAMPLED_WAVELENGTH, radius),
        }
    }

    /// A scattered direction, and the weight per channel, phase / pdf: white unless the phase
    /// differs between channels. The lobe is chosen in proportion to its share, then the
    /// direction weighted by the mixture's density, which is never zero as the base phase
    /// function covers the sphere.
    fn sample(&self, incoming: &Vector3<f64>) -> (Vector3<f64>, Color) {
        match self.diffraction {
            None => (self.base.sample(incoming), Color::white()),
            Some((d, radius)) => {
                let dir = if rand() < d.fraction {
                    d.table.sample(incoming, SAMPLED_WAVELENGTH, radius, rand(), rand())
                } else {
                    self.base.sample(incoming)
                };
                let cos = incoming.dot(&dir);
                (dir, self.eval(cos) / self.pdf(cos))
            },
        }
    }
}

impl VolumeMaterial {
    /// Whether a vertex after `scatterings` earlier volume scatterings treats diffraction as no
    /// scattering at all: delta-M truncation (W. J. Wiscombe, "The Delta-M Method: Rapid Yet
    /// Accurate Radiative Flux Calculations for Strongly Asymmetric Phase Functions", Journal of
    /// the Atmospheric Sciences 34, 1977). The diffraction lobe is a few degrees wide: after
    /// the first scattering, whose colours the camera sees, light diffracted carries on almost
    /// as if unscattered, and sampling its sharp peak from a point-like sun only makes
    /// fireflies. So from the second scattering on, the diffracted fraction f of scattering is
    /// removed from the extinction, σ (1 - f α), and the albedo, α (1 - f) / (1 - f α), and the
    /// rest scatters by the base phase function.
    fn truncated(&self, scatterings: u32) -> bool {
        self.diffraction.is_some() && scatterings >= 1
    }

    /// The fraction f of the base phase function's scattering that a vertex after `scatterings`
    /// earlier volume scatterings treats as no scattering: delta-M truncation
    /// (`ScatteringApproximation::delta_m`; Wiscombe 1977, as for diffraction above). For
    /// Henyey-Greenstein of mean cosine g, f = g² keeps the phase function's first two moments
    /// (Wiscombe's equations 7-8 with its Legendre moments g^l), and the rest is
    /// Henyey-Greenstein again with g' = (g - f) / (1 - f) = g / (1 + g). Like diffraction, it
    /// leaves the extinction σ (1 - f α) and the albedo α (1 - f) / (1 - f α).
    fn forward_peak(&self, scatterings: u32) -> f64 {
        match self.phase {
            PhaseFunction::HenyeyGreenstein { g } if g > 0. && self.approximation.delta_m && scatterings >= 1 => shared::phase::forward_peak(g),
            _ => 0.,
        }
    }

    /// Whether a path that hasn't scattered in a medium yet arrives at one of its vertices
    /// truncated, having left a surface (`Volume::segment_scale`), when any truncation applies.
    fn reached_truncated(&self) -> bool {
        left_earlier_vertex() && self.density_scale(1) != 1.
    }

    /// The base phase function after `scatterings`: without its forward peak, when truncated.
    fn base_phase(&self, scatterings: u32) -> PhaseFunction {
        let f = self.forward_peak(scatterings);
        match self.phase {
            PhaseFunction::HenyeyGreenstein { g } if f > 0. => PhaseFunction::HenyeyGreenstein { g: shared::phase::without_peak(g, f) },
            p => p,
        }
    }

    /// How a vertex at `point`, after `scatterings` volume scatterings, scatters: isotropically
    /// when reduced, without diffraction when truncated, and otherwise in full.
    fn phase_at(&self, point: &Vector3<f64>, scatterings: u32) -> VertexPhase<'_> {
        if self.reduced(scatterings) {
            return VertexPhase { base: PhaseFunction::isotropic(), diffraction: None };
        }
        VertexPhase {
            base: self.base_phase(scatterings),
            diffraction: self.diffraction.as_ref().filter(|_| !self.truncated(scatterings))
                .map(|d| (d, self.field.particle_radius(point).unwrap_or(d.radius))),
        }
    }

    /// The mean cosine of the scattering angle after `scatterings`, before any similarity
    /// reduction: diffraction scatters almost straight on, unless truncated away.
    fn mean_cosine(&self, scatterings: u32) -> f64 {
        match &self.diffraction {
            Some(d) if !self.truncated(scatterings) => (1. - d.fraction) * self.phase.mean_cosine() + d.fraction,
            _ => self.base_phase(scatterings).mean_cosine(),
        }
    }

    /// Whether a vertex after `scatterings` earlier volume scatterings uses the similarity
    /// relations.
    fn reduced(&self, scatterings: u32) -> bool {
        self.approximation.similarity_after.is_some_and(|n| scatterings >= n)
    }

    /// The single-scattering albedo, mean over channels, which sets the reduced extinction.
    fn mean_albedo(&self) -> f64 {
        self.albedo.rgb.mean()
    }

    /// The density scale for the path's segments after `scatterings` volume scatterings:
    /// truncation's, then similarity's on what remains.
    fn density_scale(&self, scatterings: u32) -> f64 {
        let mut scale = 1.;
        let mut albedo = self.mean_albedo();
        if let Some(d) = self.diffraction.as_ref().filter(|_| self.truncated(scatterings)) {
            scale *= shared::phase::truncated_extinction(albedo, d.fraction);
            albedo = shared::phase::truncated_albedo(albedo, d.fraction);
        }
        // Diffraction is truncated wherever the forward peak is, so then the base phase function
        // is all the scattering left.
        let f = self.forward_peak(scatterings);
        if f > 0. {
            scale *= shared::phase::truncated_extinction(albedo, f);
            albedo = shared::phase::truncated_albedo(albedo, f);
        }
        if self.reduced(scatterings) {
            scale *= 1. - albedo * self.mean_cosine(scatterings);
        }
        scale
    }

    /// The albedo at a vertex after `scatterings`, truncated and reduced as they apply.
    fn albedo_at(&self, scatterings: u32) -> Color {
        self.adjusted_albedo(self.albedo, scatterings)
    }

    /// The albedo `a` after `scatterings`, truncated and reduced as they apply.
    fn adjusted_albedo(&self, a: Color, scatterings: u32) -> Color {
        let mut a = a.rgb;
        if let Some(d) = self.diffraction.as_ref().filter(|_| self.truncated(scatterings)) {
            a = a.map(|a| shared::phase::truncated_albedo(a, d.fraction));
        }
        let f = self.forward_peak(scatterings);
        if f > 0. {
            a = a.map(|a| shared::phase::truncated_albedo(a, f));
        }
        if self.reduced(scatterings) {
            let g = self.mean_cosine(scatterings);
            a = a.map(|a| a * (1. - g) / (1. - a * g));
        }
        Color::new(a.x, a.y, a.z)
    }

    /// Light from the scene's lights scattered at `point` towards the viewer, for light that
    /// was travelling along `incoming` (a unit vector): each light's radiance times the phase
    /// function at the angle between `incoming` and the direction to it, before the albedo.
    /// Point lights and the sun are delta lights, which a scattered ray can never hit, so this
    /// is the only way they reach a medium; area lights are sampled once, as at a surface.
    #[allow(clippy::too_many_arguments)]
    fn direct_lighting(&self, incoming: &Vector3<f64>, point: &Vector3<f64>, s: &Scene,
                       phase: &VertexPhase, scatterings: u32, first: u32, sun_weight: f64) -> Color {
        let p = |w: &Vector3<f64>| phase.eval(incoming.dot(w)).as_unbounded();
        // With diffusion, whether this is the path's first vertex in such a medium, where the
        // light diffused through it is counted (and only there).
        let diffusing = self.approximation.diffusion.map(|_| !DIFFUSED.with(|d| d.replace(true)));
        // Through the scene to a light, along a unit ray, as the phase function scatters it;
        // with diffusion, plus what the medium in between diffuses (`diffused_gain`),
        // scattered evenly.
        // The surface the diffused light comes out through, at this point: where the cloud
        // thins, so that turrets are lit where they face a flash inside.
        let inward = if diffusing == Some(true) { self.inward(point, incoming) } else { *incoming };
        let through = |r: &Ray, d: f64| -> Color {
            let transmitted = s.objects.transmittance(r, d, f64::MIN_POSITIVE) * p(&r.rd);
            match diffusing {
                Some(true) => transmitted + Color::white() * (self.diffused_gain(&r.ro, &r.rd, d, &inward) / (4. * std::f64::consts::PI)),
                _ => transmitted,
            }
        };
        let mut direct = Color::black();
        if diffusing != Some(false) {
            // With a light BVH, point lights are sampled with the area lights below.
            for light in s.lights.iter().filter(|_| s.sums_point_lights()) {
                let to_light = light.position - point;
                let d = to_light.norm();
                let w = to_light / d;
                direct = direct + light.radiant_intensity(&-w) * through(&Ray { ro: *point, rd: w }, d) / (d * d);
            }
            direct = direct + crate::material::functions::sample_lights_through(point, s, |_, _| Color::white(), &through);
        }
        if let Some(sun) = &s.sun {
            // Halos, rainbows and diffraction colour the phase function in RGB.
            let sun_phase = self.sun_phase(phase, incoming, s, scatterings).as_unbounded();
            direct = direct + match self.approximation.oz {
                _ if self.approximation.diffusion.is_some() => {
                    if diffusing != Some(true) {
                        // Counted at the path's first vertex in such a medium.
                        Color::black()
                    } else {
                        sun.irradiance_at(point) * sun_phase * sun.transmittance_from(point, s)
                            + self.diffused_sunlight(point, s)
                    }
                },
                None if sun_weight <= 0. => Color::black(),
                None => sun.irradiance_at(point) * sun_phase * sun.transmittance_from(point, s) * sun_weight,
                // At the path's first volume scattering, even one reached off a surface.
                Some(oz) if first == 0 => sun.irradiance_at(point) * self.oz_sunlight(&oz, sun_phase, incoming, point, s),
                // Already counted, octaves and all, at the path's first scattering.
                Some(_) => Color::black(),
            };
        }
        // Moonlight, by the plain phase function (no halos: their tables are the sun's).
        if let Some(moon) = &s.moonlight {
            let m = &moon.light;
            direct = direct + m.irradiance_at(point) * p(&m.direction) * m.transmittance_from(point, s);
        }
        direct
    }

    /// The direction into the medium at `point`, near its surface: up its density's gradient
    /// (by central differences over 25 m), or `incoming`, the way the path came in, where the
    /// density hardly varies.
    fn inward(&self, point: &Vector3<f64>, incoming: &Vector3<f64>) -> Vector3<f64> {
        let h = 25.;
        let at = |dx: f64, dy: f64, dz: f64| self.field.density(&(point + Vector3::new(dx, dy, dz)));
        let g = Vector3::new(at(h, 0., 0.) - at(-h, 0., 0.), at(0., h, 0.) - at(0., -h, 0.), at(0., 0., h) - at(0., 0., -h));
        let mean = at(0., 0., 0.).max(1e-12);
        // A gradient of less than a hundredth of the density per metre isn't a surface.
        if g.norm() / (2. * h) < 0.01 * mean / 25. {
            return *incoming;
        }
        g.normalize()
    }

    /// With diffusion, the light that reaches `point` diffused through this medium from a light
    /// a distance `length` away along the unit direction `d`, as a multiple of what would reach
    /// it through empty space (the part not transmitted unscattered, which the shadow ray
    /// counts, left out). `inward` is the direction into the medium there (`inward`), so the
    /// point lies just inside a surface facing back along it.
    ///
    /// A cloud absorbs next to nothing, so light from a source inside it is trapped, scattered
    /// to and fro until it leaks out, and near the surface over the source it is brighter than
    /// the source alone would make it there. The diffusion solution for a point source at depth
    /// z under the surface of a half-space is a dipole (Farrell, Patterson and Wilson, Med.
    /// Phys. 19, 1992; as Jensen et al.'s BSSRDF, SIGGRAPH 2001): the fluence at the surface is
    /// P / (4π D) (e^(-σ_tr r₁) / r₁ - e^(-σ_tr r₂) / r₂), r₁ the distance to the source and r₂
    /// to its negative image mirrored above the extrapolated boundary 2D over the surface, with
    /// D = 1 / 3σ'_t and σ_tr = √(3 σ_a σ'_t) from the reduced extinction σ'_t = σ_t (1 - α g)
    /// and absorption σ_a = σ_t (1 - α). Against the free-space P / (4π d²), straight over a
    /// deep source and without absorption, that is four times as bright. The coefficients
    /// are the mean along the line to the light. A cloud isn't a half-space: light leaves by
    /// its far side too, so the dipole is scaled by the share diffusion through a slab sends
    /// out of the near side, (τ'b + ⅔) / (τ'f + τ'b + 4⁄3) for reduced depths τ'f in front of
    /// the source and τ'b behind it (Kokhanovsky, "Cloud Optics", ch. 3), which is one for a
    /// deep cloud behind it.
    fn diffused_gain(&self, point: &Vector3<f64>, d: &Vector3<f64>, length: f64, inward: &Vector3<f64>) -> f64 {
        let steps = self.approximation.diffusion.unwrap_or(32).max(1);
        let albedo = self.mean_albedo();
        let reduce = 1. - albedo * self.phase.mean_cosine();
        let jitter = crate::geometry::rand();
        let depth = |from: Vector3<f64>, len: f64| -> f64 {
            let step = len / steps as f64;
            (0..steps).map(|k| self.field.density(&(from + d * ((k as f64 + jitter) * step)))).sum::<f64>() * step * reduce
        };
        let front = depth(*point, length);
        if front <= 0. || length <= 0. {
            return 0.;
        }
        // Behind the light, as far again or to the medium's height range, whichever is less.
        let (lo, hi) = self.column;
        let to_edge = if d.y > 0. { (hi - (point.y + d.y * length)) / d.y } else if d.y < 0. { (lo - (point.y + d.y * length)) / d.y } else { f64::INFINITY };
        let back = depth(point + d * length, length.min(to_edge.max(0.)).max(1.));
        let escaping = (back + 2. / 3.) / (front + back + 4. / 3.) * 2.;
        // The mean coefficients along the line.
        let sigma_r = front / length;
        let diffusion = 1. / (3. * sigma_r);
        let sigma_a = sigma_r / reduce.max(1e-9) * (1. - albedo);
        let sigma_tr = (3. * sigma_a * sigma_r).sqrt();
        // The source's depth under the surface, and its image above it.
        let z = (length * d.dot(inward)).max(0.);
        let rho2 = (length * length - z * z).max(0.);
        let (r1, r2) = (length, (rho2 + (z + 4. * diffusion).powi(2)).sqrt());
        let dipole = length * length / diffusion * ((-sigma_tr * r1).exp() / r1 - (-sigma_tr * r2).exp() / r2);
        // The part the shadow ray already carried, unscattered, is left out.
        dipole.max(0.) * escaping.min(1.) * (1. - (-front / reduce.max(1e-9)).exp())
    }

    /// The phase function for direct sunlight at a vertex seen along `incoming`: with halos,
    /// at a path's first scattering, the traced crystals' in place of the base phase function.
    fn sun_phase(&self, phase: &VertexPhase, incoming: &Vector3<f64>, s: &Scene, scatterings: u32) -> Color {
        let sun = s.sun.as_ref().unwrap();
        let cos = incoming.dot(&sun.direction);
        let first = scatterings == 0 && !self.reduced(scatterings);
        let halos = match (&self.halos, &self.rainbow) {
            (Some(h), _) if first => h,
            (None, Some(rainbow)) if first => return rainbow.eval(cos),
            _ => return phase.eval(cos),
        };
        let table = halos.table.get_or_init(|| crate::geometry::build_apart(|| HaloTable::new(&sun.direction, &halos.crystals, halos.rays, halos.seed)));
        let crystals = table.eval(incoming);
        match phase.diffraction {
            None => crystals,
            Some((d, radius)) => crystals * (1. - d.fraction) + d.table.eval_color(cos, radius) * d.fraction,
        }
    }

    /// The diffusion approximation's sunlight scattered many times, as radiance arriving at
    /// `point` from all around (`ScatteringApproximation::diffusion`).
    fn diffused_sunlight(&self, point: &Vector3<f64>, s: &Scene) -> Color {
        let sun = s.sun.as_ref().unwrap();
        let mu = sun.direction.normalize().y;
        let steps = self.approximation.diffusion.unwrap_or(32).max(1);
        if mu <= 0. {
            return Color::black();
        }
        // The optical depths of the column below the point and above it.
        let (lo, hi) = self.column;
        let (below, above) = self.field.column_depths(point, lo, hi, steps);
        let reduce = 1. - self.mean_albedo() * self.phase.mean_cosine();
        let radiance = diffused_radiance(mu, (below + above) * reduce, below * reduce);
        // Sunlight arriving at the top of the column, past anything above it.
        let top = Vector3::new(point.x, hi, point.z) + sun.direction.normalize() * 1e-3;
        sun.irradiance_at(&top) * sun.transmittance_from(&top, s) * radiance
    }

    /// Wrenninge et al.'s sum over octaves, per unit sun irradiance: Σ b^i p_i(θ) T(a^i σ),
    /// p_i the phase function with its mean cosine scaled c^i (as Henyey-Greenstein beyond the
    /// first octave).
    fn oz_sunlight(&self, oz: &Oz, first: Color, incoming: &Vector3<f64>, point: &Vector3<f64>, s: &Scene) -> Color {
        let sun = s.sun.as_ref().unwrap();
        let cos = incoming.dot(&sun.direction);
        let saved = (path_scatterings(), path_density_scale());
        let mut total = Color::black();
        for i in 0..oz.octaves as i32 {
            let phase = if i == 0 {
                first
            } else {
                Color::white() * PhaseFunction::HenyeyGreenstein { g: self.mean_cosine(saved.0) * oz.c.powi(i) }.eval(cos)
            };
            set_path(saved.0, saved.1 * oz.a.powi(i));
            total = total + phase * sun.transmittance_from(point, s) * oz.b.powi(i);
        }
        set_path(saved.0, saved.1);
        total
    }
}

impl VolumeMaterial {
    /// # Sun probes
    ///
    /// The sunlight a path gathers at its next vertex in a cloud is the sun's irradiance times
    /// the phase function at the angle between the path's direction and the sun's. Where the
    /// phase function has a strong forward peak (g = 0.85 gives about 80 times its mean) the
    /// rare paths heading nearly straight for the sun carry those peaks, and make the fireflies
    /// of sunlit clouds: in a measured cumulus scene they were nearly all of its noise (none
    /// without the sun, a sixth of it at g = 0.5).
    ///
    /// So that light is estimated by two techniques, combined by multiple importance sampling
    /// (Veach and Guibas, "Optimally Combining Sampling Techniques for Monte Carlo Rendering",
    /// SIGGRAPH 1995) with the balance heuristic: the path's own continuation, its direction
    /// drawn from this vertex's phase function, and a probe from here whose direction is drawn
    /// from the same phase function about the sun, which meets the next vertex's peak. Both
    /// find the next vertex by the same free flight, whose density cancels from the weights,
    /// which are each technique's direction density over their sum (`NEXT_SUN_WEIGHT`, applied
    /// where the continuation scatters next). The path itself is sampled as before and keeps
    /// its weight; sampling its direction from a mixture instead (a defensive mixture) gave
    /// every unguided bounce a weight above one, which compounded over the dozens of bounces in
    /// a thick cloud into far worse fireflies. The probe costs a free flight and a shadow ray.
    ///
    /// Returns the probe's estimate (before this vertex's albedo) of the sunlight scattered at
    /// the vertex it finds and back along it to here, then here towards `incoming`.
    fn sun_probe(&self, incoming: &Vector3<f64>, point: &Vector3<f64>, sun: &Vector3<f64>, phase: &VertexPhase, s: &Scene) -> Color {
        let d = phase.base.sample(sun);
        let probe = Ray { ro: *point, rd: d };
        let found = s.objects.nearest_intersection(&probe, f64::INFINITY, 0.);
        // Chromatic media crossed on the way weight it as they would the continuation, without
        // changing the path's own state.
        let mis = chromatic::side_weight(found.as_ref().map(|h| h.dist));
        let Some(hit) = found else { return Color::black() };
        if hit.dist >= crate::light::SUN_SHADOW_DISTANCE {
            return Color::black();
        }
        let Some(light) = hit.object.medium.material_at(hit.point).volume_sunlight(&d, &hit.point, s) else { return Color::black() };
        // Through the air on the way, as the continuation's would be (`trace`).
        let air = match &s.atmosphere {
            Some(sky) if hit.dist > crate::trace::AERIAL_PERSPECTIVE_FROM => Color { rgb: sky.segment(point, &d, hit.dist).0 },
            _ => Color::white(),
        };
        let cos = incoming.dot(&d);
        let density = phase.pdf(cos) + phase.base.eval(sun.dot(&d));
        phase.eval(cos) * light * air * mis / density
    }
}

/// The local vertical over `sea` at `point`.
fn sea_up(sea: &crate::ocean::Sea, point: &Vector3<f64>) -> Vector3<f64> {
    sea.center.map_or(Vector3::y(), |c| (point - c).normalize())
}

impl VolumeMaterial {
    /// # Glitter probes
    ///
    /// Over a sea, a cloud's base is lit by the sun's glitter as well as the sky: a patch of
    /// the sea below, as bright as the sun's reflection in water, a degree or two of slope wide
    /// (Cox and Munk). A path scattering in the cloud finds that patch only by chance, and then
    /// carries all of it: measured on a lone cumulus over the sea, it made most of the speckle
    /// (relative error 0.052 against 0.020 without the sea). As with the sun probes, that light
    /// is estimated by two techniques combined by the balance heuristic: the continuation, and a
    /// probe whose direction is drawn towards the glitter (`ocean::Sea::sample_glitter`: a facet
    /// from the sea's slopes, the direction it would reflect into the sun). It counts the
    /// glitter where the probe meets the sea first (`MaterialModel::sun_glint`), weighted by its
    /// density over the two's sum; the continuation's sea takes the rest
    /// (`NEXT_GLINT_WEIGHT`). Only the glitter, not the sky the sea reflects, which is dim.
    #[allow(clippy::too_many_arguments)]
    fn glitter_probe(&self, incoming: &Vector3<f64>, point: &Vector3<f64>, sea: &crate::ocean::Sea, sun: &Vector3<f64>,
                     phase: &VertexPhase, s: &Scene) -> Color {
        let Some(d) = sea.sample_glitter(point, sun, (rand(), rand())) else { return Color::black() };
        let probe = Ray { ro: *point, rd: d };
        let found = s.objects.nearest_intersection(&probe, f64::INFINITY, 0.);
        let mis = chromatic::side_weight(found.as_ref().map(|h| h.dist));
        let Some(hit) = found else { return Color::black() };
        let Some(glint) = hit.object.medium.material_at(hit.point).sun_glint(&probe, &hit, s) else { return Color::black() };
        let air = match &s.atmosphere {
            Some(sky) if hit.dist > crate::trace::AERIAL_PERSPECTIVE_FROM => Color { rgb: sky.segment(point, &d, hit.dist).0 },
            _ => Color::white(),
        };
        let cos = incoming.dot(&d);
        let density = phase.pdf(cos) + sea.glitter_pdf(point, sun, &d);
        phase.eval(cos) * glint * air * mis / density
    }
}

/// The mean radiance of sunlight diffused through a conservative slab of reduced optical
/// thickness `column`, at a reduced depth `below` above its base, per unit irradiance from a
/// sun at cosine `mu` from the zenith (`ScatteringApproximation::diffusion`).
fn diffused_radiance(mu: f64, column: f64, below: f64) -> f64 {
    let injection = 3. / 7. * (1. + 2. * mu);
    let flux = mu * injection / (1. + 0.75 * column);
    3. * flux / (4. * std::f64::consts::PI) * (below + 2. / 3.)
}

impl MaterialModel for VolumeMaterial {
    fn medium(&self) -> Option<&VolumeMaterial> { Some(self) }

    fn volume_sunlight(&self, incoming: &Vector3<f64>, point: &Vector3<f64>, s: &Scene) -> Option<Color> {
        if self.extinction.is_some() {
            return None;
        }
        // In a medium lit by diffusion, the sun is counted once per path, at its first vertex
        // there, by whichever ray reached it (`direct_lighting`, which gives that vertex all of
        // it, whatever its `sun_weight`): a probe finds none.
        if self.approximation.diffusion.is_some() {
            return Some(Color::black());
        }
        let sun = s.sun.as_ref()?;
        // As `scatter` reckons this vertex's generation (a probe always follows a scattering, so
        // `first` is at least 1, but the two must agree).
        let first = path_scatterings();
        let scatterings = if first == 0 && self.reached_truncated() { 1 } else { first };
        if self.approximation.oz.is_some() && first > 0 {
            return Some(Color::black());
        }
        let phase = self.phase_at(point, scatterings);
        let sun_phase = self.sun_phase(&phase, incoming, s, scatterings).as_unbounded();
        // The sunlight reaches the vertex as it would if the path scattered here (`scatter`):
        // through media scaled for a path that has scattered once more. (Without this the
        // probe's sunlight came through the densities of the vertex that sent it, which differ
        // where the similarity relations begin: against the GPU's kernels, which send no
        // probes, a cumulus layer with `similarity_after` 4 was 2 to 3% too dark.)
        let saved = (path_scatterings(), path_density_scale());
        set_path(scatterings + 1, self.density_scale(scatterings + 1));
        let through = sun.transmittance_from(point, s);
        set_path(saved.0, saved.1);
        Some(self.albedo_at(scatterings).as_reflectance() * sun.irradiance_at(point) * sun_phase * through)
    }

    fn scatter(&self, r: &Ray, i: &Intersection, s: &Scene) -> ScatteredRay {
        // The path scatters here (`ggx::regularised`).
        crate::material::ggx::scattered();
        let incoming = r.rd.normalize();
        let first = path_scatterings();
        // A first volume scattering after the path has left a surface arrived truncated
        // (`Volume::truncated`): it scatters as the first truncated generation. Only one the
        // camera sees directly scatters in full.
        let scatterings = if first == 0 && self.reached_truncated() { 1 } else { first };
        let phase = self.phase_at(&i.point, scatterings);
        let albedo = self.albedo_at(scatterings);
        // Light leaving this vertex, towards the lights and onwards, has scattered once more.
        set_path(scatterings + 1, self.density_scale(scatterings + 1));
        let (rd, weight) = phase.sample(&incoming);
        // At the sample's wavelengths: a chromatic medium's from its coefficients there.
        let albedo = match &self.extinction {
            None => albedo.as_reflectance(),
            Some((sigma_a, sigma_s)) => {
                let (a, s) = (sigma_a.as_unbounded().rgb, sigma_s.as_unbounded().rgb);
                let single = s.zip_map(&(a + s), |s, t| if t > 0. { s / t } else { 0. });
                self.adjusted_albedo(Color { rgb: single }, scatterings)
            },
        };
        // Its share of the sunlight here, against a sun probe's that might have found this vertex
        // (none can, in a chromatic medium: `volume_sunlight`).
        let sun_weight = if self.extinction.is_none() { sun_weight_for(r) } else { 1. };
        let mut emit = albedo * self.direct_lighting(&incoming, &i.point, s, &phase, scatterings, first, sun_weight);
        // The next vertex's sunlight, by two techniques combined: along the continuation, and
        // by a probe towards the sun.
        let probe = s.sun.as_ref().map(|sun| sun.direction)
            .filter(|_| self.sun_probes && self.extinction.is_none() && self.approximation.diffusion.is_none() && phase.base.mean_cosine() > 0.2);
        NEXT_SUN_WEIGHT.with(|w| w.set(probe.map(|sun| {
            let p = phase.pdf(incoming.dot(&rd));
            (i.point, rd, p / (p + phase.base.eval(sun.dot(&rd))))
        })));
        if let Some(sun) = probe {
            emit = emit + albedo * self.sun_probe(&incoming, &i.point, &sun, &phase, s);
        }
        // And the sea's glitter of the sun, by the continuation and a probe towards it.
        let glitter = match (&s.sea, &s.sun) {
            (Some(sea), Some(sun)) if self.sun_probes && self.extinction.is_none() && sun.direction.dot(&sea_up(sea, &i.point)) > 0. => Some((*sea, sun.direction)),
            _ => None,
        };
        NEXT_GLINT_WEIGHT.with(|w| w.set(glitter.map(|(sea, sun)| {
            let p = phase.pdf(incoming.dot(&rd));
            (i.point, rd, p / (p + sea.glitter_pdf(&i.point, &sun, &rd)))
        })));
        if let Some((sea, sun)) = glitter {
            emit = emit + albedo * self.glitter_probe(&incoming, &i.point, &sea, &sun, &phase, s);
        }
        if let Some(e) = &self.emission {
            // σ_a / σ_t of the emitted radiance (`emission`).
            emit = emit + (Color::white() - albedo) * e.radiance(&i.point);
        }
        ScatteredRay {
            ray: Some(Ray { ro: i.point, rd }),
            // Phase / pdf: 1, unless the phase function differs between channels.
            attenuate: albedo * weight.as_unbounded(),
            emit,
            // Area lights were sampled above (when enabled); the trace skips them if the
            // scattered ray hits one, so they aren't counted twice.
            light_sampling: LightSampling::Exclusive,
        }
    }
}

/// How a volume's medium scatters, besides its density.
pub struct Scattering {
    pub albedo: Color,
    pub phase: PhaseFunction,
    pub approximation: ScatteringApproximation,
    pub diffraction: Option<Diffraction>,
    pub halos: Option<Halos>,
    pub rainbow: Option<Arc<dyn DropletPhase>>,
    pub emission: Option<emission::VolumeEmission>,
    /// Absorption and scattering coefficients per unit density (RGB), for a chromatic medium;
    /// `albedo` should then be their ratio.
    pub extinction: Option<(Color, Color)>,
    /// Whether its vertices send sun probes (`VolumeMaterial::sun_probe`).
    pub sun_probes: bool,
}

pub fn create_volume(bounds: BBox, density: Arc<dyn DensityField>, s: Scattering) -> SceneObject {
    create_medium(Volume::new(bounds, density), s)
}

/// A volume as an object, scattering as `s` says.
pub fn create_medium(mut volume: Volume, s: Scattering) -> SceneObject {
    // A medium whose extinction is the same in every channel is grey: its density, scaled.
    let (density, extinction) = match s.extinction {
        Some((a, sc)) if !(a + sc).rgb.iter().all(|k| *k == (a + sc).rgb.x) => (volume.density.clone(), Some((a, sc))),
        Some((a, sc)) => {
            let k = (a + sc).rgb.x;
            let scaled: Arc<dyn DensityField> = Arc::new(Scaled { field: volume.density.clone(), by: k });
            (scaled, None)
        },
        None => (volume.density.clone(), None),
    };
    volume.density = density;
    volume.chromatic = extinction.map(|(a, sc)| a + sc);
    let material = VolumeMaterial {
        albedo: s.albedo, phase: s.phase, approximation: s.approximation, diffraction: s.diffraction,
        halos: s.halos, rainbow: s.rainbow, field: volume.density.clone(), emission: s.emission,
        extinction, sun_probes: s.sun_probes, column: (volume.bounds.min.y, volume.bounds.max.y),
    };
    volume.truncated = material.density_scale(1);
    SceneObject {
        geometry: Box::new(volume),
        medium: Box::new(Solid { m: Box::new(material) }),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_graph_density_follows_its_graph() {
        // Fog two per metre at the ground, thinning to nothing a metre up.
        let g = crate::graph::Graph::parse(&serde_json::json!({"node": "multiply", "inputs": [2, {"node": "altitude", "from": 1, "to": 0}]})).unwrap();
        let fog = GraphDensity::new(g, None).unwrap();
        assert_eq!(fog.density(&Vector3::new(3., -1., 2.)), 2.);
        assert_eq!(fog.density(&Vector3::new(3., 0.5, 2.)), 1.);
        assert_eq!(fog.density(&Vector3::new(3., 4., 2.)), 0.);
        assert_eq!(fog.majorant(), 2.);
        // Never negative, and bounded above.
        let below = crate::graph::Graph::parse(&serde_json::json!({"node": "expr", "expr": "y", "range": [-5, 5]})).unwrap();
        assert_eq!(GraphDensity::new(below, None).unwrap().density(&Vector3::new(0., -3., 0.)), 0.);
        let unbounded = crate::graph::Graph::parse(&serde_json::json!("y")).unwrap();
        assert!(GraphDensity::new(unbounded, None).is_err());
    }

    use super::*;
    use crate::geometry::seed_rand;

    fn unit_box() -> BBox {
        BBox::new(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.))
    }

    /// The fraction of `n` rays along +x through the volume at height (y, z) that pass through.
    fn transmitted(v: &Volume, y: f64, z: f64, n: usize) -> f64 {
        let r = Ray { ro: Vector3::new(-1., y, z), rd: Vector3::new(1., 0., 0.) };
        (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64
    }

    #[test]
    fn test_constant_volume_follows_beer_lambert() {
        seed_rand(1, 0);
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(1.5)));
        let t = transmitted(&v, 0.5, 0.5, 40_000);
        assert!((t - (-1.5f64).exp()).abs() < 0.01, "{}", t);
    }

    #[test]
    fn test_collisions_lie_within_the_box() {
        seed_rand(2, 0);
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(3.)));
        // Unnormalised direction: distances are still world-space.
        let r = Ray { ro: Vector3::new(-2., 0.5, 0.5), rd: Vector3::new(4., 0., 0.) };
        for _ in 0..1000 {
            if let Some(i) = v.intersects(&r) {
                assert!(i.dist >= 2. && i.dist <= 3., "{}", i.dist);
                assert!((i.point.x - (i.dist - 2.)).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn test_ray_starting_inside_or_beyond_min() {
        seed_rand(3, 0);
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(1.)));
        let r = Ray { ro: Vector3::new(0.5, 0.5, 0.5), rd: Vector3::new(1., 0., 0.) };
        let n = 40_000;
        // From the centre, half a unit of medium remains; from min = 0.3 along, a fifth.
        let inside = (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64;
        let beyond = (0..n).filter(|_| v.intersects_beyond(&r, 0.3).is_none()).count() as f64 / n as f64;
        assert!((inside - (-0.5f64).exp()).abs() < 0.01, "{}", inside);
        assert!((beyond - (-0.2f64).exp()).abs() < 0.01, "{}", beyond);
        assert!(v.intersects_beyond(&r, 0.3).map_or(true, |i| i.dist >= 0.3));
    }

    #[test]
    fn test_volume_in_a_sphere() {
        use crate::shapes::sphere::Sphere;
        seed_rand(10, 0);
        let v = Volume::in_shape(Box::new(Sphere::new(Vector3::new(0., 0., 0.), 1.)), Arc::new(ConstantDensity(1.)));
        let n = 40_000;
        // Through the middle: 2 units of medium. Off-centre at 0.8: a chord of 1.2.
        for (y, chord) in [(0., 2.), (0.8, 1.2)] {
            let r = Ray { ro: Vector3::new(-3., y, 0.), rd: Vector3::new(1., 0., 0.) };
            let t = (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64;
            assert!((t - (-chord as f64).exp()).abs() < 0.01, "{}: {}", y, t);
            assert!((v.transmittance(&r, 0., f64::INFINITY) - (-chord as f64).exp()).abs() < 1e-9);
            // Collisions are inside.
            for _ in 0..200 {
                if let Some(i) = v.intersects(&r) {
                    assert!(i.point.norm() <= 1. + 1e-9, "{:?}", i.point);
                }
            }
        }
        // Outside the sphere but within its bounds: clear.
        let corner = Ray { ro: Vector3::new(-3., 0.9, 0.9), rd: Vector3::new(1., 0., 0.) };
        assert!((0..100).all(|_| v.intersects(&corner).is_none()));
        // From inside, only what lies ahead: a unit radius.
        let inside = Ray { ro: Vector3::new(0., 0., 0.), rd: Vector3::new(0., 1., 0.) };
        assert!((v.transmittance(&inside, 0., f64::INFINITY) - (-1f64).exp()).abs() < 1e-9);
    }

    #[test]
    fn test_empty_volume_is_clear() {
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(0.)));
        assert_eq!(transmitted(&v, 0.5, 0.5, 100), 1.);
    }

    fn noise_volume() -> (Volume, f64) {
        let density = NoiseDensity {
            perlin: PerlinNoise::shuffled(9), max_density: 4., scale: 3., octaves: 4, threshold: 0.45,
        };
        let steps = 20_000;
        let optical_depth: f64 = (0..steps).map(|i| {
            density.density(&Vector3::new((i as f64 + 0.5) / steps as f64, 0.37, 0.61))
        }).sum::<f64>() / steps as f64;
        (Volume::new(unit_box(), Arc::new(density)), optical_depth)
    }

    #[test]
    fn test_ratio_tracking_matches_optical_depth_with_less_noise() {
        seed_rand(7, 0);
        let (v, optical_depth) = noise_volume();
        let expected = (-optical_depth).exp();
        let r = Ray { ro: Vector3::new(-1., 0.37, 0.61), rd: Vector3::new(1., 0., 0.) };
        let n = 40_000;
        let ratio: Vec<f64> = (0..n).map(|_| v.transmittance(&r, 0., f64::INFINITY)).collect();
        let mean = ratio.iter().sum::<f64>() / n as f64;
        let var = ratio.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / n as f64;
        assert!((mean - expected).abs() < 0.005, "{} vs {}", mean, expected);
        // A 0-or-1 estimate of the same mean has variance T(1 - T).
        assert!(var < 0.5 * expected * (1. - expected), "variance {} vs {}", var, expected * (1. - expected));
        assert!(ratio.iter().all(|t| (0. ..=1.).contains(t)));
    }

    #[test]
    fn test_ratio_tracking_respects_min_and_max() {
        seed_rand(8, 0);
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(2.)));
        let r = Ray { ro: Vector3::new(-1., 0.5, 0.5), rd: Vector3::new(1., 0., 0.) };
        let n = 40_000;
        // From 1.2 to 1.7 along the ray: half a unit of medium.
        let mean = (0..n).map(|_| v.transmittance(&r, 1.2, 1.7)).sum::<f64>() / n as f64;
        assert!((mean - (-1f64).exp()).abs() < 0.005, "{}", mean);
        assert_eq!(v.transmittance(&r, 0., 0.9), 1.);
    }

    #[test]
    fn test_diffraction_is_truncated_after_the_first_scattering() {
        let m = VolumeMaterial {
            albedo: Color::new(0.9, 0.9, 0.9),
            phase: PhaseFunction::HenyeyGreenstein { g: 0.5 },
            approximation: ScatteringApproximation::default(),
            diffraction: Some(Diffraction { table: DiffractionTable::new(0.1), fraction: 0.5, radius: 5. }),
            halos: None,
            rainbow: None,
            field: Arc::new(ConstantDensity(1.)),
            emission: None,
            extinction: None,
            sun_probes: false,
            column: (0., 1.),
        };
        // First scattering in full: the extinction as given, diffraction in the phase.
        assert_eq!(m.density_scale(0), 1.);
        assert_eq!(m.albedo_at(0), m.albedo);
        assert!(m.phase_at(&Vector3::zeros(), 0).diffraction.is_some());
        // After it, the diffracted half of scattering, 0.45 of extinction, is no scattering.
        assert!((m.density_scale(1) - 0.55).abs() < 1e-12);
        assert!((m.albedo_at(1).rgb.x - 0.45 / 0.55).abs() < 1e-12);
        assert!(m.phase_at(&Vector3::zeros(), 1).diffraction.is_none());
        // Scattered light is conserved: σ_s' = σ α' = 0.55 × 0.818 = 0.45 = σ α (1 - f).
        assert!((m.density_scale(1) * m.albedo_at(1).rgb.x - 0.9 * 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_delta_m_truncates_all_but_the_cameras_segments() {
        // Henyey-Greenstein g = 0.8, albedo 0.9: delta-M takes f = 0.64 of scattering as none,
        // so rays that leave a vertex see the extinction scaled by 1 - 0.9 × 0.64; the camera's
        // own rays see the medium as it is.
        let scattering = Scattering {
            albedo: Color::new(0.9, 0.9, 0.9),
            phase: PhaseFunction::HenyeyGreenstein { g: 0.8 },
            approximation: ScatteringApproximation { delta_m: true, ..ScatteringApproximation::default() },
            diffraction: None, halos: None, rainbow: None, emission: None, extinction: None, sun_probes: false,
        };
        let o = create_volume(unit_box(), Arc::new(ConstantDensity(2.)), scattering);
        let r = Ray { ro: Vector3::new(-1., 0.5, 0.5), rd: Vector3::new(1., 0., 0.) };
        begin_path();
        assert!((o.transmittance(&r, 0., f64::INFINITY) - (-2f64).exp()).abs() < 1e-12);
        at_vertex();
        let truncated = (-2. * (1. - 0.9 * 0.64f64)).exp();
        assert!((o.transmittance(&r, 0., f64::INFINITY) - truncated).abs() < 1e-12);
        // Not after a mirror.
        specular_vertex();
        assert!((o.transmittance(&r, 0., f64::INFINITY) - (-2f64).exp()).abs() < 1e-12);
        begin_path();
    }

    #[test]
    fn test_diffusion_through_a_slab() {
        use std::f64::consts::PI;
        // At the base, the radiance below a thick slab: its diffuse transmittance, spread over
        // the hemisphere (F = π L), and the mean over the sphere half that.
        let (mu, tau) = (0.6, 8.);
        let transmitted = mu * 3. / 7. * (1. + 2. * mu) / (1. + 0.75 * tau);
        assert!((diffused_radiance(mu, tau, 0.) - transmitted / (2. * PI)).abs() < 1e-12);
        // Brighter up through the cloud, linearly, towards the reflected light at the top.
        let at = |below: f64| diffused_radiance(mu, tau, below);
        assert!(at(4.) > at(2.) && at(2.) > at(0.));
        assert!(((at(4.) - at(2.)) - (at(2.) - at(0.))).abs() < 1e-12);
        // Darker under a thicker deck: the light under it goes as 1 / (1 + ¾ τ*).
        assert!(diffused_radiance(mu, 16., 0.) < 0.55 * diffused_radiance(mu, 8., 0.));
    }

    #[test]
    fn test_inward_follows_the_clouds_surface() {
        // A cloud whose density rises across y = 0 over a hundred metres.
        struct Edge;
        impl DensityField for Edge {
            fn density(&self, p: &Vector3<f64>) -> f64 { 0.05 * ((p.y + 50.) / 100.).clamp(0., 1.) }
            fn majorant(&self) -> f64 { 0.05 }
        }
        let m = |field: Arc<dyn DensityField>| VolumeMaterial {
            albedo: Color::white(), phase: PhaseFunction::HenyeyGreenstein { g: 0.85 },
            approximation: ScatteringApproximation { diffusion: Some(32), ..ScatteringApproximation::default() },
            diffraction: None, halos: None, rainbow: None, field, emission: None, extinction: None, sun_probes: false, column: (-100., 1e4),
        };
        // Whatever way the path came in, into the cloud is up.
        let slant = Vector3::new(0.9, 0.3, 0.3).normalize();
        let n = m(Arc::new(Edge)).inward(&Vector3::new(0., 0., 0.), &slant);
        assert!((n - Vector3::new(0., 1., 0.)).norm() < 1e-6, "{}", n);
        // Inside an even cloud there is no surface: the way the path came in.
        assert_eq!(m(Arc::new(ConstantDensity(0.05))).inward(&Vector3::zeros(), &slant), slant);
    }

    #[test]
    fn test_light_inside_a_cloud_glows_through_its_surface() {
        // A deep cloud above y = 0 of extinction 0.05, g = 0.85: reduced extinction 0.0075 per
        // metre, D = 44 m. A path entering it from below, straight up.
        let cloud = |albedo: f64| VolumeMaterial {
            albedo: Color::white() * albedo,
            phase: PhaseFunction::HenyeyGreenstein { g: 0.85 },
            approximation: ScatteringApproximation { diffusion: Some(400), ..ScatteringApproximation::default() },
            diffraction: None, halos: None, rainbow: None,
            field: Arc::new(ConstantDensity(0.05)), emission: None, extinction: None, sun_probes: false, column: (0., 1e6),
        };
        let m = cloud(1.);
        let (at, up) = (Vector3::new(0., 1e-6, 0.), Vector3::new(0., 1., 0.));
        // Straight over a source 500 m deep: the dipole, D²/D (1/r₁ - 1/r₂) with r₂ = 500 + 4D.
        let dd = 1. / (3. * 0.0075);
        let expected = 500f64.powi(2) / dd * (1. / 500. - 1. / (500. + 4. * dd));
        let over = m.diffused_gain(&at, &up, 500., &up);
        assert!((over - expected).abs() < 0.02 * expected, "{} vs {}", over, expected);
        // Several times brighter than the source would be in empty space.
        assert!(over > 2.5, "{}", over);
        // 400 m to the side of a source 300 m deep, the light (gain over distance squared) is a
        // third of that straight over it: the glow gathers over the channel.
        let slant = Vector3::new(0.8, 0.6, 0.);
        let (side, straight) = (m.diffused_gain(&at, &slant, 500., &up) / 500f64.powi(2), m.diffused_gain(&at, &up, 300., &up) / 300f64.powi(2));
        assert!(side < 0.4 * straight && side > 0.2 * straight, "{} {}", side, straight);
        // Absorption dims it: with an albedo of 0.99 the light dies out within some 400 m.
        assert!(cloud(0.99).diffused_gain(&at, &up, 500., &up) < 0.5 * over);
        // No medium in between: nothing diffused (the shadow ray carries it all).
        let clear = VolumeMaterial { field: Arc::new(ConstantDensity(0.)), ..cloud(1.) };
        assert_eq!(clear.diffused_gain(&at, &up, 500., &up), 0.);
    }

    #[test]
    fn test_sum_adds_densities_bounds_and_columns() {
        let s = Sum { fields: vec![Arc::new(ConstantDensity(0.01)), Arc::new(ConstantDensity(0.03))] };
        let p = Vector3::new(1., 50., 2.);
        assert!((s.density(&p) - 0.04).abs() < 1e-15);
        assert!((s.majorant() - 0.04).abs() < 1e-15);
        assert_eq!(s.majorant_from(&p, &Vector3::y()), (s.majorant(), f64::INFINITY));
        let (below, above) = s.column_depths(&p, 0., 100., 16);
        assert!((below + above - 4.).abs() < 1e-9, "{}", below + above);
    }

    #[test]
    fn test_column_depths_of_a_constant_medium() {
        let field = ConstantDensity(0.02);
        let (below, above) = field.column_depths(&Vector3::new(5., 300., -2.), 0., 1000., 64);
        assert!((below + above - 20.).abs() < 1e-9, "{}", below + above);
        assert!((below - 6.).abs() < 0.4 && (above - 14.).abs() < 0.4, "{} {}", below, above);
    }

    #[test]
    fn test_delta_m_conserves_scattering() {
        // From the second scattering, g' = g / (1 + g), and the scattering coefficient that is
        // left is what wasn't truncated: σ' α' = σ α (1 - f).
        let m = VolumeMaterial {
            albedo: Color::new(0.95, 0.95, 0.95),
            phase: PhaseFunction::HenyeyGreenstein { g: 0.85 },
            approximation: ScatteringApproximation { delta_m: true, ..ScatteringApproximation::default() },
            diffraction: None, halos: None, rainbow: None,
            field: Arc::new(ConstantDensity(1.)), emission: None, extinction: None, sun_probes: false, column: (0., 1.),
        };
        assert_eq!(m.density_scale(0), 1.);
        assert_eq!(m.mean_cosine(0), 0.85);
        assert!((m.mean_cosine(1) - 0.85 / 1.85).abs() < 1e-12);
        let f = 0.85f64.powi(2);
        assert!((m.density_scale(1) * m.albedo_at(1).rgb.x - 0.95 * (1. - f)).abs() < 1e-12);
        assert!((m.density_scale(1) - (1. - 0.95 * f)).abs() < 1e-12);
    }

    #[test]
    fn test_homogeneous_transmittance_is_exact() {
        // The constant density is its own minorant: nothing left to estimate.
        let v = Volume::new(unit_box(), Arc::new(ConstantDensity(2.)));
        let r = Ray { ro: Vector3::new(-1., 0.5, 0.5), rd: Vector3::new(1., 0., 0.) };
        for _ in 0..10 {
            assert!((v.transmittance(&r, 0., f64::INFINITY) - (-2f64).exp()).abs() < 1e-12);
        }
    }

    #[test]
    fn test_delta_tracking_matches_optical_depth_of_noise() {
        // Transmittance through varying density is exp(-∫σ); integrate σ finely along the ray
        // and compare with the fraction of tracked rays that get through.
        seed_rand(4, 0);
        let (v, optical_depth) = noise_volume();
        assert!(optical_depth > 0.2 && optical_depth < 3., "uninformative test: {}", optical_depth);
        let t = transmitted(&v, 0.37, 0.61, 40_000);
        assert!((t - (-optical_depth).exp()).abs() < 0.01, "{} vs {}", t, (-optical_depth).exp());
    }
}
