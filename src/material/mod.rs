use color::Color;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;

///
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
pub trait MaterialModel {
    /// Scatter an intersection ray.
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay;
}

/// The outgoing ray, and the weight to assign the color of the traced ray.
/// - Color: the scaling of the subsequent reflections/refractions
/// - Option<Ray>:
///     Some: Another ray to cast into the image, multiply by Color
///     None: Return Color
///
#[derive(Clone, Debug, PartialEq)]
pub struct ScatteredRay {
    pub ray: Option<Ray>,
    pub attenuate: Color
}

/*
pub trait BSDFToRename{

    //fn compute_interactions(&self, r: &Ray, intersection: &Intersection, s: &Scene) ->
    //ListOf<(weight, ray)>
}
*/
