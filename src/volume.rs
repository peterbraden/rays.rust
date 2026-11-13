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
use crate::material::functions::sample_area_light_with;
use crate::material::phase::PhaseFunction;
use crate::material::diffraction::DiffractionTable;
use crate::material::ice_halos::{Crystals, HaloTable};
use crate::material::rainbow::RainbowTable;
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

thread_local! {
    /// The state of the path being traced on this thread, for the multiple-scattering
    /// approximations (`ScatteringApproximation`): how many times it has scattered in a
    /// volume, and the factor all volumes' densities are scaled by for its next segments,
    /// shadow rays included. Geometry is intersected without knowing which path it serves,
    /// hence a thread-local; each thread traces one path at a time, from `begin_path`.
    static PATH: Cell<(u32, f64)> = const { Cell::new((0, 1.)) };
}

/// Start a new path: no volume scatterings yet, and densities as given.
pub fn begin_path() {
    PATH.with(|p| p.set((0, 1.)));
}

fn path_density_scale() -> f64 {
    PATH.with(|p| p.get().1)
}

fn path_scatterings() -> u32 {
    PATH.with(|p| p.get().0)
}

fn set_path(scatterings: u32, scale: f64) {
    PATH.with(|p| p.set((scatterings, scale)));
}

/// Below this estimated transmittance, ratio tracking plays Russian roulette.
const RATIO_TRACKING_ROULETTE: f64 = 0.3;

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

    /// A lower bound on `density` everywhere in the volume (by default 0). Transmittance
    /// through this much is computed exactly, and only the rest estimated.
    fn minorant(&self) -> f64 {
        0.
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
    fn density(&self, _p: &Vector3<f64>) -> f64 { self.0 }
    fn majorant(&self) -> f64 { self.0 }
    fn minorant(&self) -> f64 { self.0 }
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

/// A participating medium filling a box.
pub struct Volume {
    pub bounds: BBox,
    pub density: Arc<dyn DensityField>,
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
        let scale = path_density_scale();
        if self.density.majorant() * scale <= 0. {
            return None;
        }
        let d = r.rd.normalize();
        let invrd = Vector3::new(1. / d.x, 1. / d.y, 1. / d.z);
        let (entry, exit) = self.bounds.entry_exit(&r.ro, &invrd)?;
        let mut t = entry.max(min);
        while t < exit {
            let (majorant, length) = self.density.majorant_from(&(r.ro + d * t), &d);
            // Always advance, if only by a hair, so a stretch reported as empty at a boundary
            // can't stall the walk.
            let end = (t + length.max(t.abs().max(1.) * 1e-12)).min(exit);
            if majorant > 0. {
                // Scaling the density scales the majorant alike: steps shrink, and the chance
                // of a collision being real is unchanged.
                let step = -(1. - rand()).ln() / (majorant * scale);
                if t + step < end {
                    t += step;
                    let point = r.ro + d * t;
                    if rand() * majorant < self.density.density(&point) {
                        // Media have no surface; facing back along the ray keeps the shadow
                        // bias from moving the point sideways.
                        return Some(RawIntersection { face: 0, part: 0, dist: t, point, normal: -d });
                    }
                    continue;
                }
            }
            t = end;
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
        let scale = path_density_scale();
        if self.density.majorant() * scale <= 0. {
            return 1.;
        }
        let d = r.rd.normalize();
        let invrd = Vector3::new(1. / d.x, 1. / d.y, 1. / d.z);
        let (entry, exit) = match self.bounds.entry_exit(&r.ro, &invrd) {
            Some(x) => x,
            None => return 1.,
        };
        let stop = exit.min(max);
        let mut t = entry.max(min);
        if t >= stop {
            return 1.;
        }
        let control = self.density.minorant();
        let mut transmitted = (-control * scale * (stop - t)).exp();
        while t < stop {
            let (majorant, length) = self.density.majorant_from(&(r.ro + d * t), &d);
            let end = (t + length.max(t.abs().max(1.) * 1e-12)).min(stop);
            let residual = majorant - control;
            if residual > 0. {
                let step = -(1. - rand()).ln() / (residual * scale);
                if t + step < end {
                    t += step;
                    transmitted *= 1. - (self.density.density(&(r.ro + d * t)) - control) / residual;
                    if transmitted < RATIO_TRACKING_ROULETTE {
                        // Survive with probability T / threshold, carrying the threshold.
                        if rand() * RATIO_TRACKING_ROULETTE >= transmitted {
                            return 0.;
                        }
                        transmitted = RATIO_TRACKING_ROULETTE;
                    }
                    continue;
                }
            }
            t = end;
        }
        transmitted
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn is_stochastic(&self) -> bool { true }
}

/// Approximations that trade exactness for speed and noise in thick, bright media like
/// clouds, where light scatters hundreds of times. Both off by default: plain path tracing.
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
    /// Rainbows from raindrops (see `material::rainbow`): like halos, the traced drops' phase
    /// function for direct sunlight at a path's first scattering.
    pub rainbow: Option<Arc<RainbowTable>>,
    /// The volume's density field, for the particle size where it has one.
    pub field: Arc<dyn DensityField>,
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

    /// How a vertex at `point`, after `scatterings` volume scatterings, scatters: isotropically
    /// when reduced, without diffraction when truncated, and otherwise in full.
    fn phase_at(&self, point: &Vector3<f64>, scatterings: u32) -> VertexPhase<'_> {
        if self.reduced(scatterings) {
            return VertexPhase { base: PhaseFunction::isotropic(), diffraction: None };
        }
        VertexPhase {
            base: self.phase,
            diffraction: self.diffraction.as_ref().filter(|_| !self.truncated(scatterings))
                .map(|d| (d, self.field.particle_radius(point).unwrap_or(d.radius))),
        }
    }

    /// The mean cosine of the scattering angle after `scatterings`, before any similarity
    /// reduction: diffraction scatters almost straight on, unless truncated away.
    fn mean_cosine(&self, scatterings: u32) -> f64 {
        match &self.diffraction {
            Some(d) if !self.truncated(scatterings) => (1. - d.fraction) * self.phase.mean_cosine() + d.fraction,
            _ => self.phase.mean_cosine(),
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
            scale *= 1. - d.fraction * albedo;
            albedo = albedo * (1. - d.fraction) / (1. - d.fraction * albedo);
        }
        if self.reduced(scatterings) {
            scale *= 1. - albedo * self.mean_cosine(scatterings);
        }
        scale
    }

    /// The albedo at a vertex after `scatterings`, truncated and reduced as they apply.
    fn albedo_at(&self, scatterings: u32) -> Color {
        let mut a = self.albedo.rgb;
        if let Some(d) = self.diffraction.as_ref().filter(|_| self.truncated(scatterings)) {
            a = a.map(|a| a * (1. - d.fraction) / (1. - d.fraction * a));
        }
        if self.reduced(scatterings) {
            let g = self.mean_cosine(scatterings);
            a = a.map(|a| a * (1. - g) / (1. - a * g));
        }
        Color::new(a.x, a.y, a.z)
    }

    /// Light from the scene's lights scattered at `point` towards the viewer, for light that
    /// was travelling along `incoming` (a unit vector): each light's radiance times the phase
    /// function at the angle between `incoming` and the direction to it, times the albedo.
    /// Point lights and the sun are delta lights, which a scattered ray can never hit, so this
    /// is the only way they reach a medium; area lights are sampled once, as at a surface.
    fn direct_lighting(&self, incoming: &Vector3<f64>, point: &Vector3<f64>, s: &Scene,
                       phase: &VertexPhase, albedo: Color, scatterings: u32) -> Color {
        let p = |w: &Vector3<f64>| phase.eval(incoming.dot(w));
        let mut direct = Color::black();
        for light in &s.lights {
            let to_light = light.position - point;
            let d = to_light.norm();
            let w = to_light / d;
            let shadow_ray = Ray { ro: *point, rd: w };
            let transmitted = s.objects.transmittance(&shadow_ray, d, f64::MIN_POSITIVE);
            direct = direct + light.color * p(&w) * (light.intensity / (d * d) * transmitted);
        }
        if let Some(sun) = &s.sun {
            let sun_phase = self.sun_phase(phase, incoming, s, scatterings);
            direct = direct + match self.approximation.oz {
                None => sun.irradiance_at(point) * sun_phase * sun.transmittance_from(point, s),
                Some(oz) if scatterings == 0 => sun.irradiance_at(point) * self.oz_sunlight(&oz, sun_phase, incoming, point, s),
                // Already counted, octaves and all, at the path's first scattering.
                Some(_) => Color::black(),
            };
        }
        if s.render.sample_lights {
            direct = direct + sample_area_light_with(point, s, |w, _| p(w));
        }
        albedo * direct
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
        let table = halos.table.get_or_init(|| HaloTable::new(&sun.direction, &halos.crystals, halos.rays, halos.seed));
        let crystals = table.eval(incoming);
        match phase.diffraction {
            None => crystals,
            Some((d, radius)) => crystals * (1. - d.fraction) + d.table.eval_color(cos, radius) * d.fraction,
        }
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
            total = total + phase * (oz.b.powi(i) * sun.transmittance_from(point, s));
        }
        set_path(saved.0, saved.1);
        total
    }
}

impl MaterialModel for VolumeMaterial {
    fn scatter(&self, r: &Ray, i: &Intersection, s: &Scene) -> ScatteredRay {
        let incoming = r.rd.normalize();
        let scatterings = path_scatterings();
        let phase = self.phase_at(&i.point, scatterings);
        let albedo = self.albedo_at(scatterings);
        // Light leaving this vertex, towards the lights and onwards, has scattered once more.
        set_path(scatterings + 1, self.density_scale(scatterings + 1));
        let (rd, weight) = phase.sample(&incoming);
        ScatteredRay {
            ray: Some(Ray { ro: i.point, rd }),
            // Phase / pdf: 1, unless the phase function differs between channels.
            attenuate: albedo * weight,
            emit: self.direct_lighting(&incoming, &i.point, s, &phase, albedo, scatterings),
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
    pub rainbow: Option<Arc<RainbowTable>>,
}

pub fn create_volume(bounds: BBox, density: Arc<dyn DensityField>, s: Scattering) -> SceneObject {
    let material = VolumeMaterial {
        albedo: s.albedo, phase: s.phase, approximation: s.approximation, diffraction: s.diffraction,
        halos: s.halos, rainbow: s.rainbow, field: density.clone(),
    };
    SceneObject {
        geometry: Box::new(Volume { bounds, density }),
        medium: Box::new(Solid { m: Box::new(material) }),
    }
}

#[cfg(test)]
mod tests {
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
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(1.5)) };
        let t = transmitted(&v, 0.5, 0.5, 40_000);
        assert!((t - (-1.5f64).exp()).abs() < 0.01, "{}", t);
    }

    #[test]
    fn test_collisions_lie_within_the_box() {
        seed_rand(2, 0);
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(3.)) };
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
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(1.)) };
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
    fn test_empty_volume_is_clear() {
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(0.)) };
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
        (Volume { bounds: unit_box(), density: Arc::new(density) }, optical_depth)
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
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(2.)) };
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
    fn test_homogeneous_transmittance_is_exact() {
        // The constant density is its own minorant: nothing left to estimate.
        let v = Volume { bounds: unit_box(), density: Arc::new(ConstantDensity(2.)) };
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
