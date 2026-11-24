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
/// The sea's water as data, for the GPU's kernel of it (`pathtrace::MAT_WATER`): what the water
/// body sends back, and what bubbles under a breaking crest add to that; the ripples' mean
/// square slope; the planet's radius (0 for a flat sea); and its foam's record
/// (`ocean::Foam::export`), if the wind raises any.
pub struct Water {
    pub body: Color,
    pub bubbles: Color,
    pub roughness: f64,
    pub radius: f64,
    pub foam: Option<Vec<f32>>,
}

pub trait MaterialModel: Sync{
    /// Scatter an intersection ray.
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay;

    /// Whether this material emits light, and so can be sampled as an area light.
    fn is_emitter(&self) -> bool { false }

    /// Radiance emitted in the direction `towards` (a unit vector leaving the surface) from a
    /// point with surface normal `normal`.
    fn emitted(&self, _normal: &Vector3<f64>, _towards: &Vector3<f64>) -> Color { Color::black() }

    /// As `emitted`, at a point with texture coordinates `uv` (if the shape has them), for
    /// textured emitters; `emitted` is then their mean.
    fn emitted_at(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, _uv: Option<crate::na::Vector2<f64>>) -> Color {
        self.emitted(normal, towards)
    }

    /// As `emitted_at`, at the point of the surface `c` describes (where it is, its normal and
    /// its object's fields), for materials whose emission varies over it as a graph says
    /// (`surface_layers`); sampled lights are evaluated here.
    fn emitted_in(&self, c: &crate::graph::Context, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> Color {
        self.emitted_at(&c.normal, towards, uv)
    }

    /// This material as data, for the wavefront integrator (`wavefront::flat`); None if it
    /// can't be.
    fn flat(&self) -> Option<crate::wavefront::flat::FlatMaterial> { None }

    /// The normal to shade with at a hit (whose normal is the surface's, already bumped by
    /// its medium, if that is), if the material tilts it: a material's own bump map
    /// (`bump::BumpedMaterial`), so its relief stays aligned with its pattern wherever it's
    /// used. None to shade with the hit's normal.
    fn shading_normal(&self, _i: &Intersection, _s: &crate::scene::Scene) -> Option<Vector3<f64>> { None }

    /// The sea's water, if this is its material and the GPU's kernel for it does all this one
    /// does (opaque water). See `ocean::PhysicalOcean`.
    fn water(&self) -> Option<Water> { None }
    /// A medium's scattering, if that is what this is (`volume::Volume::flat`).
    fn medium(&self) -> Option<&crate::volume::VolumeMaterial> { None }

    /// Whether this is the sky's material, which the wavefront integrator evaluates for rays
    /// that escape the scene.
    fn is_sky(&self) -> bool { false }

    /// A sky's air, for the GPU to evaluate as the CPU does (`skysphere::AirExport`).
    fn air(&self) -> Option<crate::skysphere::AirExport> { None }
    /// For a scattering medium: the sunlight a scattering at `point` sends back along
    /// `incoming` (a unit vector, the way the light arriving there travelled), exactly as its
    /// own `scatter` gathers it (albedo, phase function, sun, transmittance), for the path's
    /// current state; None if it can't say (a surface, or a chromatic medium). The other half
    /// of the sun probes' multiple importance sampling (`volume::VertexPhase::sun_probe`).
    fn volume_sunlight(&self, _incoming: &Vector3<f64>, _point: &Vector3<f64>, _s: &crate::scene::Scene) -> Option<Color> { None }
    /// For a glitter probe (`VolumeMaterial::glitter_probe`): the sun's glitter on this surface
    /// seen along `r` at `intersection`, if it is a sea.
    fn sun_glint(&self, _r: &crate::ray::Ray, _intersection: &crate::intersection::Intersection, _s: &crate::scene::Scene) -> Option<Color> { None }
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
    /// Only the distant lights, the sun and moonlight, were sampled (as for the glitter of the
    /// sea): the ray ignores their discs, but area lights it hits count in full.
    Directional,
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
