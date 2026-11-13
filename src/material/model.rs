use crate::color::Color;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;

/// See https://google.github.io/filament//Materials.md.html#materialmodels/litmodel
///
/// In Google Filament they refer to
/// - The Lit model (standard material model)
/// - Subsurface model
/// - Cloth model
///
/// In PBRT they refer to 
/// - a "bidirectional reflectance distribution function (BRDF)"
/// - a "bidirectional transmission distribution function (BTDF)
/// - a "bidirectional scattering distribution function (BSDF)
/// - a "bidirectional sub-surface scattering distribution function  BSSRDF"
///
/// In Raytracing in a weekend they call it:
/// - a "hittable"
///
/// This terminology is all horrible. As what all of the above are describing is the way that a ray of
/// light interacts with an object based on the material of that object, we will call this a
/// "Material Model"
///
///  PBRT uses:
///  - eta - the difference in refractive index of the interaction, default 1
///
pub trait MaterialModel: Sync{
    /// Scatter an intersection ray.
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay;

    /// Whether this material emits light, and so can be sampled as an area light.
    fn is_emitter(&self) -> bool { false }

    /// Radiance emitted in the direction `towards` (a unit vector leaving the surface) from a
    /// point with surface normal `normal`.
    fn emitted(&self, _normal: &Vector3<f64>, _towards: &Vector3<f64>) -> Color { Color::black() }
}

/// The outgoing ray, and the weight to assign the color of the traced ray.
/// The resulting colour is `emit + attenuate * incoming`, where incoming is:
/// - Option<Ray>:
///     Some: the colour traced along the ray
///     None: white, ie. attenuate is the final colour
/// - Emit: light leaving the surface independent of the outgoing ray (eg. direct lighting)
///
#[derive(Clone, Debug, PartialEq)]
pub struct ScatteredRay {
    pub ray: Option<Ray>,
    pub attenuate: Color,
    pub emit: Color,
    /// Whether `emit` already includes light sampled directly from the scene's area lights and
    /// sun, and so how much of their emission the continuation ray counts if it hits one.
    pub light_sampling: LightSampling,
}

/// How a scattered ray's vertex sampled the lights (area lights and the sun), which decides
/// what the ray counts of their emission if it hits one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightSampling {
    /// The lights weren't sampled: their emission counts in full.
    None,
    /// The lights were sampled alone, so their emission is already counted and the ray ignores
    /// it (as for a point-like light, which only light sampling can find).
    Exclusive,
    /// The lights were sampled with multiple importance sampling (`material::bsdf`), and the
    /// ray's direction was sampled from the BSDF with density `bsdf_pdf` per steradian: their
    /// emission counts weighted by the power heuristic against the lights' own density.
    MIS { bsdf_pdf: f64 },
}

/// Some material models use a direct ray to the light sources to calculate a scatter.
pub trait DirectMaterialModel: MaterialModel{
    fn scatter_light(&self, inbound: &Ray, light: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay;
}

/*
pub trait BSDFToRename{

    //fn compute_interactions(&self, r: &Ray, intersection: &Intersection, s: &Scene) ->
    //ListOf<(weight, ray)>

}
*/
