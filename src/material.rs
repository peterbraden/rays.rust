use color::Color;
use na::Vec3;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use geometry::random_point_on_unit_sphere;

///
/// See https://google.github.io/filament//Materials.md.html#materialmodels/litmodel

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
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
    /// Returns:
    /// - u64: the count of subsequent rays cast (used to calculate total rays cast in recursive
    ///         scenes)
    /// - Color: the scaling of the subsequent reflections/refractions 
    /// - Option<Ray>: 
    ///     Some: Another ray to cast into the image, multiply by Color
    ///     None: Return Color
    ///
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> (u64, Color, Option<Ray>);
}


pub struct Ambient {
    pub pigment: Color,
}
impl MaterialModel for Ambient {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> (u64, Color, Option<Ray>){
        return (0, self.pigment, None);
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
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> (u64, Color, Option<Ray>){
        let refl = Ray {
            ro: intersection.point,
            rd: intersection.normal + random_point_on_unit_sphere(),
        };
        return (1, self.albedo, Some(refl));
    }
}



// ----------------------------------------------------------------------------------------------

// TODO - rename texture
pub trait Medium {
    fn box_clone(&self) -> Box<dyn Medium>;
    fn material_at(&self, pt: Vec3<f64>) -> Material; 
}

#[derive(Clone)]
pub struct Solid {
    pub m: Material
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vec3<f64>) -> Material {
        self.m.clone()
    }

    fn box_clone(&self) -> Box<Medium>{
        return Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct CheckeredYPlane {
    pub m1: Material,
    pub m2: Material,
    pub xsize: f64,
    pub zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: Material, m2: Material, xsize: f64, zsize: f64) -> CheckeredYPlane {
        CheckeredYPlane { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(&self, pt: Vec3<f64>) -> Material {
        let zig = if (pt[0].abs() / self.xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / self.zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        return if !zig != !zag { self.m1.clone() } else { self.m2.clone() };
    }

    fn box_clone(&self) -> Box<Medium>{
        return Box::new(self.clone())
    }
}

