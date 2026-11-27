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

/// A surface's parameters at a point, as a rough plastic's: what the kernels need to shade it
/// there without its own code, when it is shaded at vertices rather than at each hit
/// (`MaterialModel::bake`; plan.md section 18). `coat` is the share coated: 0 for a Lambertian
/// surface, 1 for a plastic, between for a blend of the two.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Baked {
    pub albedo: Color,
    pub refractive_index: f64,
    pub roughness: f64,
    pub coat: f64,
}

impl Baked {
    /// Lambertian, of `albedo`.
    pub fn diffuse(albedo: Color) -> Baked {
        Baked { albedo, refractive_index: 1.5, roughness: 1., coat: 0. }
    }

    /// `t` of `other` blended into this: albedo and coat as the shares of the surface they are,
    /// the coat's refractive index and roughness weighted by the coat each brings.
    pub fn mix(&self, other: &Baked, t: f64) -> Baked {
        let t = t.clamp(0., 1.);
        let coat = self.coat * (1. - t) + other.coat * t;
        let (a, b) = (self.coat * (1. - t), other.coat * t);
        let by_coat = |x: f64, y: f64| if a + b > 0. { (x * a + y * b) / (a + b) } else { x * (1. - t) + y * t };
        Baked {
            albedo: self.albedo * (1. - t) + other.albedo * t,
            refractive_index: by_coat(self.refractive_index, other.refractive_index),
            roughness: by_coat(self.roughness, other.roughness),
            coat,
        }
    }
}

/// What covers a surface at a vertex, for shading it there (plan.md section 18): the shares of
/// snow, glacier ice and water over it, the snow's age (0 settled, 1 firn) and the ice's
/// blueness (0 white, 1 blue), and the rest bare, `bare`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Covered {
    pub bare: Baked,
    pub snow: f64,
    pub snow_age: f64,
    pub ice: f64,
    pub blue: f64,
    pub water: f64,
}

impl Covered {
    /// Bare all over.
    pub fn bare(bare: Baked) -> Covered {
        Covered { bare, snow: 0., snow_age: 0., ice: 0., blue: 0., water: 0. }
    }

    /// `t` of `other` blended into this.
    pub fn mix(&self, other: &Covered, t: f64) -> Covered {
        let t = t.clamp(0., 1.);
        let f = |a: f64, b: f64| a * (1. - t) + b * t;
        Covered { bare: self.bare.mix(&other.bare, t), snow: f(self.snow, other.snow), snow_age: f(self.snow_age, other.snow_age),
                  ice: f(self.ice, other.ice), blue: f(self.blue, other.blue), water: f(self.water, other.water) }
    }

    /// Three blended by barycentric weights `w` (summing to 1): across a triangle from its
    /// corners'.
    pub fn interpolate(c: [&Covered; 3], w: [f64; 3]) -> Covered {
        let ab = w[0] + w[1];
        let first = if ab > 0. { c[0].mix(c[1], w[1] / ab) } else { *c[0] };
        first.mix(c[2], w[2])
    }
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

    /// Its parameters at the hit, as a plastic's (`Baked`), for shading at vertices; None for a
    /// material that isn't a plastic, a Lambertian surface or a blend of them.
    fn bake(&self, _i: &Intersection, _s: &Scene) -> Option<Baked> { None }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_blend_takes_its_coat_from_what_is_coated() {
        let rough = Baked { albedo: Color::new(0.4, 0.4, 0.4), refractive_index: 1.6, roughness: 0.3, coat: 1. };
        let half = Baked::diffuse(Color::new(0.2, 0.2, 0.2)).mix(&rough, 0.5);
        // The albedo and the coat by share; the coat's index and roughness all the plastic's,
        // the Lambertian surface having none.
        assert!((half.albedo.rgb.x - 0.3).abs() < 1e-12 && (half.coat - 0.5).abs() < 1e-12);
        assert_eq!((half.refractive_index, half.roughness), (1.6, 0.3));
        // Two coats, weighted by how much each brings.
        let other = Baked { albedo: Color::black(), refractive_index: 1.4, roughness: 0.7, coat: 1. };
        let both = rough.mix(&other, 0.25);
        assert!((both.roughness - 0.4).abs() < 1e-12 && (both.refractive_index - 1.55).abs() < 1e-12);
        // And the ends are the materials themselves.
        assert_eq!(rough.mix(&other, 0.), rough);
    }

    #[test]
    fn test_covers_interpolate_across_a_triangle() {
        let corner = |snow: f64, grey: f64| Covered { snow, ..Covered::bare(Baked::diffuse(Color::new(grey, grey, grey))) };
        let (a, b, c) = (corner(0., 0.1), corner(1., 0.4), corner(0.5, 0.7));
        // At each corner, that corner's; at the middle, the mean.
        for (w, at) in [([1., 0., 0.], &a), ([0., 1., 0.], &b), ([0., 0., 1.], &c)] {
            let x = Covered::interpolate([&a, &b, &c], w);
            assert!((x.snow - at.snow).abs() < 1e-12 && (x.bare.albedo.rgb.x - at.bare.albedo.rgb.x).abs() < 1e-12);
        }
        let mid = Covered::interpolate([&a, &b, &c], [1. / 3.; 3]);
        assert!((mid.snow - 0.5).abs() < 1e-12 && (mid.bare.albedo.rgb.x - 0.4).abs() < 1e-12);
    }
}
