use color::Color;
use na::{Vec3, Dot};
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use geometry::random_point_on_unit_sphere;

///
/// See https://google.github.io/filament//Materials.md.html#materialmodels/litmodel

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialProperties {
    pub pigment: Color, // Attenuation due to albedo
    pub albedo: f64,
    pub metallic: f64, // Dielectric 0 to Metallic 1
    pub roughness: f64, // Glossiness
    pub reflection: f64,
    pub phong: f64,
    pub normal_peturbation: Vec3<f64>,
    pub opacity: f64,
    pub refractive_index: f64,
}


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




pub struct Ambient {
    pub pigment: Color,
}
impl MaterialModel for Ambient {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        return ScatteredRay{ attenuate:self.pigment, ray: None };
    }
}

/// Implement Lambertian reflection (purely diffuse) for ambient incoming light (light at a random
/// incoming angle.)
/// Practically, we implement random reflection within a unit sphere on the normal.
/// This will be very noisy if we don't subsample a lot.
pub struct AmbientLambertian {
    pub albedo: Color,
}
impl MaterialModel for AmbientLambertian {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let refl = Ray {
            ro: intersection.point,
            rd: intersection.normal + random_point_on_unit_sphere(),
        };
        return ScatteredRay{ attenuate:self.albedo, ray: Some(refl) };
    }
}

pub struct Reflection {
    pub reflective: Color,
    pub roughness: f64,
}

impl MaterialModel for Reflection {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let fuzz = random_point_on_unit_sphere() * self.roughness;

        let refl = Ray {
            ro: intersection.point,
            rd: r.rd - (intersection.normal * 2.0 * intersection.normal.dot(&r.rd) + fuzz),
        };

        return ScatteredRay{ attenuate:self.reflective, ray: Some(refl) };
    }
}







// ----------------------------------------------------------------------------------------------

// TODO - rename texture
pub trait Medium {
    fn box_clone(&self) -> Box<dyn Medium>;
    fn material_at(&self, pt: Vec3<f64>) -> MaterialProperties; 
}

#[derive(Clone)]
pub struct Solid {
    pub m: MaterialProperties
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vec3<f64>) -> MaterialProperties {
        self.m.clone()
    }

    fn box_clone(&self) -> Box<Medium>{
        return Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct CheckeredYPlane {
    pub m1: MaterialProperties,
    pub m2: MaterialProperties,
    pub xsize: f64,
    pub zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: MaterialProperties, m2: MaterialProperties, xsize: f64, zsize: f64) -> CheckeredYPlane {
        CheckeredYPlane { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(&self, pt: Vec3<f64>) -> MaterialProperties {
        let zig = if (pt[0].abs() / self.xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / self.zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        return if !zig != !zag { self.m1.clone() } else { self.m2.clone() };
    }

    fn box_clone(&self) -> Box<Medium>{
        return Box::new(self.clone())
    }
}

