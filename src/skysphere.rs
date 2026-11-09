// The implementation of a physically-based sky using Rayleigh and Mie scattering models.
//
// This module simulates the scattering of light through the atmosphere using two primary
// physical models:
//
// 1. Rayleigh scattering: Scattering of light by particles much smaller than the wavelength
//    of the light. This is what makes the sky appear blue, as shorter (blue) wavelengths
//    scatter more than longer (red) wavelengths according to a 1/λ⁴ relationship.
//
// 2. Mie scattering: Scattering by particles comparable to or larger than the wavelength
//    of light, such as dust, pollen, and water droplets. This creates the white-ish haze 
//    around the sun and is less wavelength-dependent.
//
// The implementation uses numerical integration to accumulate the scattering effects along
// both the view ray and light rays to the sun.
//
// IMPLEMENTATION NOTE ON OPTICAL DEPTH ACCUMULATION:
// ==================================================
// Optical depth must be accumulated along the entire light path to correctly model
// atmospheric scattering according to the Beer-Lambert law:
//
// - We accumulate optical depth along the entire path from the camera through the atmosphere
// - Each sample's tau calculation uses the total optical depth from camera to sample point
// - This correctly produces the wavelength-dependent effects that make sunsets appear redder
//   as the light travels through more atmosphere near the horizon
// - This follows directly from the physics of light attenuation, where extinction is exponential
//   with respect to the total optical depth along the entire path
//
// For this implementation, we track the accumulated optical depth during numerical integration
// to ensure the tau calculation properly accounts for all atmospheric material the light has
// traveled through. This is critical for realistic rendering of sunset/sunrise conditions.

use crate::shapes::infinite::Infinite;
use crate::shapes::sphere::Sphere;
use crate::na::{Vector3};
use crate::sceneobject::SceneObject;
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::shapes::geometry::Geometry;
use serde_json::{Value};
use crate::scenefile::SceneFile;
use rayon::prelude::*;

// Constants for atmospheric scattering
/// Scales the integrated radiance into the renderer's working range. Under the default sRGB
/// output this keeps the sky looking as it did with its former built-in tone curve,
/// (0.383 v)^(1/2.2).
const SKY_CALIBRATION: f64 = 0.38317;

const DEFAULT_NUM_SAMPLES: usize = 16;
const DEFAULT_NUM_SAMPLES_LIGHT: usize = 8;

// Default scattering coefficient values (Earth-like)
// Sea-level Rayleigh scattering coefficients (per metre) at 680, 550 and 440 nm. These scale as
// 1/wavelength^4; the red value was previously 3.8e-6, a typo that made the sky scatter too
// little red light.
const DEFAULT_RAYLEIGH_R: f64 = 5.8e-6;
const DEFAULT_RAYLEIGH_G: f64 = 13.5e-6;
const DEFAULT_RAYLEIGH_B: f64 = 33.1e-6;
const DEFAULT_MIE_COEFFICIENT: f64 = 21.0e-6;
const DEFAULT_MIE_ANISOTROPY: f64 = 0.76;

// Preset scattering coefficients for different atmospheres
pub struct SkyPreset {
    pub name: &'static str,
    pub rayleigh_r: f64,
    pub rayleigh_g: f64,
    pub rayleigh_b: f64,
    pub mie_coefficient: f64,
    pub rayleigh_thickness: f64,
    pub mie_thickness: f64,
    pub mie_anisotropy: f64,
}

// Presets for different atmospheric conditions
pub const SKY_PRESETS: [SkyPreset; 3] = [
    // Earth-like (default)
    SkyPreset {
        name: "earth",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 21.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.76,
    },
    // Earth sunset (warmer colors)
    SkyPreset {
        name: "sunset",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 25.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.85,
    },
    // Hazy (dusty/smoky atmosphere)
    SkyPreset {
        name: "hazy",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 45.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.9,
    },
];

/// SkyMaterial simulates atmospheric scattering with Rayleigh and Mie models
pub struct SkyMaterial {
    /// Sphere representing the atmospheric boundary
    atmosphere: Sphere,
    
    /// Sphere representing the earth/planet surface
    earth: Sphere,
    
    /// Direction to the sun (normalized)
    sun_direction: Vector3<f64>,
    
    /// Scale height for Rayleigh scattering in meters
    rayleigh_thickness: f64,
    
    /// Scale height for Mie scattering in meters
    mie_thickness: f64,
    
    /// Overall brightness/intensity of the sky
    brightness: f64,
    
    /// Rayleigh scattering coefficients for RGB channels
    rayleigh_coefficients: Vector3<f64>,
    
    /// Mie scattering coefficient (wavelength-independent)
    mie_coefficient: f64,
    
    /// Mie anisotropy factor (controls the size of the glow around the sun)
    mie_anisotropy: f64,
}

impl SkyMaterial {
    /// Calculate the Rayleigh phase function
    /// 
    /// The Rayleigh phase function describes the angular distribution of 
    /// light scattered by particles much smaller than the wavelength.
    ///
    /// # Arguments
    /// * `mu` - Cosine of the angle between the view direction and sun direction
    fn rayleigh_phase(mu: f64) -> f64 {
        3.0 / (16.0 * std::f64::consts::PI) * (1.0 + mu * mu)
    }
    
    /// Calculate the Mie phase function. This is not Henyey-Greenstein, as previously described,
    /// but a related form with an extra (1 + mu^2) factor:
    /// 3(1 - g^2) (1 + mu^2) / (8 pi (2 + g^2) (1 + g^2 - 2 g mu)^(3/2)).
    /// 
    /// The Mie phase function describes the angular distribution of light
    /// scattered by particles comparable to or larger than the wavelength.
    ///
    /// # Arguments
    /// * `mu` - Cosine of the angle between the view direction and sun direction
    /// * `g` - Anisotropy factor (-1 to 1), controls the distribution of scattered light
    fn mie_phase(mu: f64, g: f64) -> f64 {
        let g2 = g * g;
        3.0 / (8.0 * std::f64::consts::PI) * 
            ((1.0 - g2) * (1.0 + mu * mu)) / 
            ((2.0 + g2) * (1.0 + g2 - 2.0 * g * mu).powf(1.5))
    }
    
    /// Calculate optical depth for a segment through the atmosphere
    /// 
    /// The optical depth represents how much a ray of light is attenuated by the medium.
    /// For atmospheric scattering, it's calculated using the barometric formula, where 
    /// density decreases exponentially with height.
    ///
    /// # Arguments
    /// * `height` - Height above the earth's surface in meters
    /// * `thickness` - Scale height parameter (rayleigh_thickness or mie_thickness)
    /// * `segment_length` - Length of the ray segment being calculated
    fn optical_depth(&self, height: f64, thickness: f64, segment_length: f64) -> f64 {
        (-height / thickness).exp() * segment_length
    }
    

    /// Distance the view ray travels through the atmosphere before leaving it, or hitting the
    /// ground if it starts above the surface. None if it misses the atmosphere entirely.
    fn view_ray_length(&self, r: &Ray) -> Option<f64> {
        let mut ray_max = self.atmosphere.intersects(r)?.dist;

        let above_surface = (r.ro - self.earth.center).norm() > self.earth.radius;
        if above_surface {
            if let Some(ground) = self.earth.intersects(r) {
                ray_max = ray_max.min(ground.dist);
            }
        }
        Some(ray_max)
    }
}

impl SkyMaterial {
    /// Rayleigh and Mie optical depths from `p` to the top of the atmosphere towards the sun, or
    /// None if the earth is in the way.
    fn optical_depth_to_sun(&self, p: Vector3<f64>) -> Option<(f64, f64)> {
        let light_ray = Ray { ro: p, rd: self.sun_direction };
        if self.earth.intersects(&light_ray).is_some() {
            return None;
        }
        let light_len = self.atmosphere.intersects(&light_ray)?.dist;
        let segment_length_light = light_len / DEFAULT_NUM_SAMPLES_LIGHT as f64;
        let mut optical_depth_r = 0.0;
        let mut optical_depth_m = 0.0;

        // Integrate along light ray to the sun, sampling each segment's midpoint
        for j in 0..DEFAULT_NUM_SAMPLES_LIGHT {
            let sample = p + ((j as f64 + 0.5) * segment_length_light) * self.sun_direction;
            let height = (sample - self.atmosphere.center).norm() - self.earth.radius;
            optical_depth_r += self.optical_depth(height, self.rayleigh_thickness, segment_length_light);
            optical_depth_m += self.optical_depth(height, self.mie_thickness, segment_length_light);
        }
        Some((optical_depth_r, optical_depth_m))
    }

    /// Linear radiance arriving along the view ray.
    pub fn radiance(&self, r: &Ray) -> Vector3<f64> {
        let (rayleigh, mie) = self.scattering(r);
        self.apply_phase(&r.rd.normalize(), rayleigh, mie)
    }

    /// Combine in-scattered Rayleigh and Mie light (see `scattering`) for the view direction
    /// `d`: weight each by its phase function at the angle between `d` and the sun.
    fn apply_phase(&self, d: &Vector3<f64>, rayleigh: Vector3<f64>, mie: Vector3<f64>) -> Vector3<f64> {
        let mu = d.dot(&self.sun_direction);
        (rayleigh * Self::rayleigh_phase(mu) + mie * Self::mie_phase(mu, self.mie_anisotropy)) * self.brightness
    }

    /// Light scattered towards the viewer along the view ray, by Rayleigh and by Mie scattering,
    /// before the phase functions and brightness are applied. Unlike the phase functions, which
    /// peak sharply around the sun, these vary smoothly with direction.
    pub fn scattering(&self, r: &Ray) -> Scattering {
        // Use the provided scattering coefficients
        let beta_r: Vector3<f64> = self.rayleigh_coefficients;
        let mie_coef = self.mie_coefficient;
        let beta_m: Vector3<f64> = Vector3::new(mie_coef, mie_coef, mie_coef);
        
        let ray_max = match self.view_ray_length(r) {
            Some(d) => d,
            None => return (Vector3::zeros(), Vector3::zeros()),
        };
        
        // Setup for numerical integration
        let num_samples = DEFAULT_NUM_SAMPLES; 
        let segment_length = ray_max / num_samples as f64; 

        let mut rayleigh_sum: Vector3<f64> = Vector3::zeros();
        let mut mie_sum: Vector3<f64> = Vector3::zeros();

        // Track accumulated optical depth for more physically accurate extinction calculation
        // This is important for proper sunset colors (see implementation note at top of file)
        let mut optical_depth_r = 0.0;
        let mut optical_depth_m = 0.0;

        
        
        // Integrate along the view ray through the atmosphere
        for i in 0..num_samples {
            // The midpoint of each segment.
            let sample_position = r.ro + ((i as f64 + 0.5) * segment_length) * r.rd;
            
            // Calculate height above the earth's surface
            let height = (sample_position - self.atmosphere.center).norm() - self.earth.radius;
            
            // Calculate Rayleigh and Mie optical depths for this segment
            let rayleigh_depth = self.optical_depth(height, self.rayleigh_thickness, segment_length);
            let mie_depth = self.optical_depth(height, self.mie_thickness, segment_length);
            
            // Accumulate optical depth along the view ray
            optical_depth_r += rayleigh_depth;
            optical_depth_m += mie_depth;
            
            // Sunlight reaching this sample: none in the earth's shadow.
            let (optical_depth_light_r, optical_depth_light_m) = match self.optical_depth_to_sun(sample_position) {
                Some(d) => d,
                None => continue,
            };
            
            // Calculate the total extinction for both Rayleigh and Mie
            // Use the total accumulated optical depth from both camera path and light path
            // This is essential for correct application of the Beer-Lambert law
            let tau = beta_r * (optical_depth_r + optical_depth_light_r) + 
                    beta_m * 1.1 * (optical_depth_m + optical_depth_light_m);
            
            // Calculate attenuation using Beer's Law
            let attenuation = Vector3::new((-tau.x).exp(), (-tau.y).exp(), (-tau.z).exp());
            
            // Accumulate contributions for Rayleigh and Mie
            rayleigh_sum += attenuation.component_mul(&Vector3::new(rayleigh_depth, rayleigh_depth, rayleigh_depth));
            mie_sum += attenuation.component_mul(&Vector3::new(mie_depth, mie_depth, mie_depth));
        } 
        
        // Combine Rayleigh and Mie scattering
        (rayleigh_sum.component_mul(&beta_r), mie_sum.component_mul(&beta_m))
    }
}

impl MaterialModel for SkyMaterial {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        // Linear light; display encoding happens at output.
        let v = self.radiance(r) * SKY_CALIBRATION;
        ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::new(v.x, v.y, v.z), ray: None }
    }
}

/// Rayleigh and Mie in-scattering, before their phase functions (`SkyMaterial::scattering`).
type Scattering = (Vector3<f64>, Vector3<f64>);

/// # Sky-view lookup table
///
/// Integrating the atmosphere for every ray that escapes the scene was the dominant cost of
/// sky scenes (`exp` alone was 63% of samples). The sky changes slowly with direction, so it
/// can be precomputed into a table and interpolated, following the Sky-View LUT of Sebastien
/// Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique",
/// Computer Graphics Forum 39(4), 2020, section 5.3, doi:10.1111/cgf.14050.
///
/// The table stores the Rayleigh and Mie in-scattering separately, before their phase
/// functions are applied, indexed by latitude l (elevation above the horizon) and longitude.
/// The phase functions depend only on the angle to the sun and are applied exactly at lookup:
/// the Mie phase function peaks too sharply around the sun to interpolate (tabulating finished
/// radiance put a few percent of error there). Colour changes
/// fastest near the horizon, so rows are spaced non-linearly to put more of them there, with
/// Hillaire's mapping v = 0.5 + 0.5 sign(l) sqrt(|l| / (pi/2)); this table covers only the
/// upper hemisphere, so v = sqrt(l / (pi/2)).
///
/// Differences from the paper:
///
/// - Hillaire rebuilds the table for each camera position. Here rays start anywhere in the
///   scene, and near the horizon the sky depends on altitude (a table for 1 m was 3-5% out at
///   30 m). So there is a table for each of several altitudes, `LUT_ALTITUDES`, interpolated
///   linearly, which is the altitude dimension of Eric Bruneton and Fabrice Neyret's
///   "Precomputed Atmospheric Scattering", Computer Graphics Forum 27(4), 2008,
///   doi:10.1111/j.1467-8659.2008.01245.x. Above the highest, rays are integrated exactly.
/// - Rays below the horizon are integrated exactly. Their radiance depends strongly on
///   altitude (how soon they reach the ground), and they are rarely seen, being mostly hidden by
///   geometry.
/// - The sun is fixed for the scene, so it needs no dimension of its own.
pub struct SkyViewLut {
    width: usize,
    height: usize,
    /// One table per altitude in `LUT_ALTITUDES`, of Rayleigh and Mie scattering before the
    /// phase functions (`SkyMaterial::scattering`).
    slices: Vec<Vec<Scattering>>,
}

/// Heights above the ground, in metres, at which the sky is tabulated: closely spaced, since
/// interpolating between distant altitudes is poor near the horizon (2-4% between 300 m and
/// 1 km). Scenes keep within tens of metres of the ground; above the top, rays are integrated
/// exactly.
const LUT_ALTITUDES: [f64; 8] = [1., 5., 15., 30., 60., 100., 150., 200.];
/// Longitude and latitude resolution of the table.
const LUT_WIDTH: usize = 256;
const LUT_HEIGHT: usize = 128;

impl SkyViewLut {
    pub fn build(sky: &SkyMaterial, width: usize, height: usize) -> SkyViewLut {
        let slices = LUT_ALTITUDES.iter().map(|altitude| {
            let origin = sky.earth.center + Vector3::new(0., sky.earth.radius + altitude, 0.);
            (0 .. width * height).into_par_iter().map(|i| {
                let (x, y) = (i % width, i / width);
                let rd = SkyViewLut::direction(x as f64 / width as f64, y as f64 / (height - 1) as f64);
                sky.scattering(&Ray { ro: origin, rd })
            }).collect()
        }).collect();
        SkyViewLut { width, height, slices }
    }

    /// The direction for texture coordinates u (longitude) and v (latitude), in [0, 1].
    fn direction(u: f64, v: f64) -> Vector3<f64> {
        let l = v * v * std::f64::consts::FRAC_PI_2;
        let phi = u * 2. * std::f64::consts::PI;
        Vector3::new(l.cos() * phi.cos(), l.sin(), l.cos() * phi.sin())
    }

    /// Rayleigh and Mie scattering for a unit direction at or above the horizon, from
    /// `altitude` metres up: bilinear within the two nearest altitude tables, linear between
    /// them. None above the highest table.
    pub fn lookup(&self, d: &Vector3<f64>, altitude: f64) -> Option<Scattering> {
        let top = LUT_ALTITUDES.len() - 1;
        if altitude > LUT_ALTITUDES[top] {
            return None;
        }
        let a = altitude.max(LUT_ALTITUDES[0]);
        let i = LUT_ALTITUDES.partition_point(|h| *h <= a).clamp(1, top);
        let f = (a - LUT_ALTITUDES[i - 1]) / (LUT_ALTITUDES[i] - LUT_ALTITUDES[i - 1]);
        let (r0, m0) = self.lookup_slice(i - 1, d);
        let (r1, m1) = self.lookup_slice(i, d);
        Some((r0 * (1. - f) + r1 * f, m0 * (1. - f) + m1 * f))
    }

    fn lookup_slice(&self, slice: usize, d: &Vector3<f64>) -> Scattering {
        let l = d.y.clamp(0., 1.).asin();
        let v = (l / std::f64::consts::FRAC_PI_2).sqrt() * (self.height - 1) as f64;
        let u = (d.z.atan2(d.x) / (2. * std::f64::consts::PI)).rem_euclid(1.) * self.width as f64;

        let (x0, y0) = (u.floor() as usize % self.width, (v.floor() as usize).min(self.height - 2));
        let (x1, y1) = ((x0 + 1) % self.width, y0 + 1);
        let (fx, fy) = (u - u.floor(), v - y0 as f64);
        let texels = &self.slices[slice];
        let mix = |pick: fn(&Scattering) -> Vector3<f64>| {
            let t = |x: usize, y: usize| pick(&texels[y * self.width + x]);
            (t(x0, y0) * (1. - fx) + t(x1, y0) * fx) * (1. - fy) + (t(x0, y1) * (1. - fx) + t(x1, y1) * fx) * fy
        };
        (mix(|p| p.0), mix(|p| p.1))
    }
}

/// The sky, looked up in a `SkyViewLut` above the horizon and integrated exactly below it. The
/// table is built when the sky is first needed.
pub struct LutSky {
    sky: SkyMaterial,
    lut: std::sync::OnceLock<SkyViewLut>,
}

impl MaterialModel for LutSky {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        let d = r.rd.normalize();
        let altitude = (r.ro - self.sky.earth.center).norm() - self.sky.earth.radius;
        let looked_up = if d.y >= 0. {
            self.lut.get_or_init(|| SkyViewLut::build(&self.sky, LUT_WIDTH, LUT_HEIGHT)).lookup(&d, altitude)
        } else {
            None
        };
        // The phase functions, which peak sharply around the sun, are applied exactly.
        let v = match looked_up {
            Some((rayleigh, mie)) => self.sky.apply_phase(&d, rayleigh, mie),
            None => self.sky.radiance(r),
        } * SKY_CALIBRATION;
        ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::new(v.x, v.y, v.z), ray: None }
    }
}

/// Get a SkyPreset by name
/// 
/// # Arguments
/// * `name` - Name of the preset
///
/// # Returns
/// The preset if found, or the default Earth preset
fn get_preset_by_name(name: &str) -> &SkyPreset {
    for preset in &SKY_PRESETS {
        if preset.name == name {
            return preset;
        }
    }
    // Default to first preset (Earth)
    &SKY_PRESETS[0]
}

/// Creates a sky sphere object from JSON configuration
///
/// # Arguments
/// * `o` - JSON value containing the sky configuration
///
/// # Returns
/// A SceneObject representing the sky sphere
pub fn create_sky_sphere(o: &Value) -> SceneObject {
    let earth_location = SceneFile::parse_vec3_def(o, "earth_location", Vector3::new(0., -6360e3, 0.));
    
    // Handle preset or custom scattering parameters
    let preset_name = match o.get("preset") {
        Some(v) => v.as_str().unwrap_or("earth"),
        None => "earth"
    };
    
    let preset = get_preset_by_name(preset_name);
    
    // Get rayleigh coefficients (custom or from preset)
    let rayleigh_r = SceneFile::parse_number(&o["rayleigh_r"], preset.rayleigh_r);
    let rayleigh_g = SceneFile::parse_number(&o["rayleigh_g"], preset.rayleigh_g);
    let rayleigh_b = SceneFile::parse_number(&o["rayleigh_b"], preset.rayleigh_b);
    let rayleigh_coefficients = Vector3::new(rayleigh_r, rayleigh_g, rayleigh_b);
    
    // Get mie coefficient and anisotropy (custom or from preset)
    let mie_coefficient = SceneFile::parse_number(&o["mie_coefficient"], preset.mie_coefficient);
    let mie_anisotropy = SceneFile::parse_number(&o["mie_anisotropy"], preset.mie_anisotropy);
    
    // Get thickness values (either custom or from preset)
    let rayleigh_thickness = SceneFile::parse_number(&o["rayleigh_thickness"], preset.rayleigh_thickness);
    let mie_thickness = SceneFile::parse_number(&o["mie_thickness"], preset.mie_thickness);
    
    let sky = SkyMaterial {
        earth: Sphere::new(
            earth_location,
            SceneFile::parse_number(&o["earth_radius"], 6360e3),
        ),
        atmosphere: Sphere::new(
            earth_location,
            SceneFile::parse_number(&o["atmosphere_radius"], 6420e3)
        ),
        rayleigh_thickness,
        mie_thickness,
        sun_direction: SceneFile::parse_vec3_def(o, "sun_direction", Vector3::new(0., 0.5, 2.)).normalize(),
        brightness: SceneFile::parse_number(&o["brightness"], 20.),
        rayleigh_coefficients,
        mie_coefficient,
        mie_anisotropy,
    };
    // The lookup table is on by default; "lut": false integrates every ray exactly.
    let m: Box<dyn MaterialModel + Sync + Send> = if o["lut"].as_bool().unwrap_or(true) {
        Box::new(LutSky { sky, lut: std::sync::OnceLock::new() })
    } else {
        Box::new(sky)
    };
    SceneObject {
        geometry: Box::new(Infinite {}),
        medium: Box::new(Solid { m }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rayleigh_phase() {
        // Test with mu = 0 (perpendicular direction)
        let phase_perpendicular = SkyMaterial::rayleigh_phase(0.0);
        assert_relative_eq!(phase_perpendicular, 3.0 / (16.0 * std::f64::consts::PI), epsilon = 1e-10);
        
        // Test with mu = 1 (same direction)
        let phase_same = SkyMaterial::rayleigh_phase(1.0);
        assert_relative_eq!(phase_same, 3.0 / (8.0 * std::f64::consts::PI), epsilon = 1e-10);
        
        // Test with mu = -1 (opposite direction)
        let phase_opposite = SkyMaterial::rayleigh_phase(-1.0);
        assert_relative_eq!(phase_opposite, 3.0 / (8.0 * std::f64::consts::PI), epsilon = 1e-10);
    }
    
    #[test]
    fn test_mie_phase() {
        let g = 0.76; // Standard anisotropy factor used in implementation
        
        // Test with mu = 0 (perpendicular)
        let phase_perpendicular = SkyMaterial::mie_phase(0.0, g);
        assert!(phase_perpendicular > 0.0);
        
        // Test with mu = 1 (same direction) - should be maximum
        let phase_same = SkyMaterial::mie_phase(1.0, g);
        assert!(phase_same > SkyMaterial::mie_phase(0.0, g));
        
        // Test with mu = -1 (opposite direction) - should be minimum
        let phase_opposite = SkyMaterial::mie_phase(-1.0, g);
        assert!(phase_opposite < SkyMaterial::mie_phase(0.0, g));
    }
    
    #[test]
    fn test_rayleigh_coefficients_scale_with_inverse_fourth_power() {
        // Red, green and blue at 680, 550 and 440 nm.
        let scale = |nm: f64| (440. / nm).powi(4) * DEFAULT_RAYLEIGH_B;
        assert!((DEFAULT_RAYLEIGH_R / scale(680.) - 1.).abs() < 0.01, "{}", DEFAULT_RAYLEIGH_R);
        assert!((DEFAULT_RAYLEIGH_G / scale(550.) - 1.).abs() < 0.01, "{}", DEFAULT_RAYLEIGH_G);
    }

    #[test]
    fn test_no_sunlight_in_earths_shadow() {
        let sky = |sun: Vector3<f64>| SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: sun.normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
        };
        // Just after sunset, the sun 0.02 radians below the horizon. From 100 m up, the sun ray
        // passes only slightly beneath the ground, so integrating through it would let a little
        // light through; the ground must block it entirely.
        let after_sunset = sky(Vector3::new(0., -0.02, 1.));
        assert_eq!(after_sunset.optical_depth_to_sun(Vector3::new(0., 100., 0.)), None);
        // From 5 km up the sun is still above the horizon (the dip of the horizon there is about
        // 0.04 radians).
        assert!(after_sunset.optical_depth_to_sun(Vector3::new(0., 5000., 0.)).is_some());
    }

    fn test_sky(sun: Vector3<f64>) -> SkyMaterial {
        SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: sun.normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
        }
    }

    /// The largest error of the lookup table relative to the exact integral, over random
    /// directions above the horizon, for rays from each of `altitudes` metres up.
    fn lut_errors(sun: Vector3<f64>, altitudes: &[f64]) -> Vec<f64> {
        let sky = test_sky(sun);
        let lut = SkyViewLut::build(&sky, LUT_WIDTH, LUT_HEIGHT);
        altitudes.iter().map(|altitude| {
            crate::geometry::seed_rand(11, 0);
            (0..300).map(|i| {
                // Half the directions within about 10 degrees of the sun, where the sky changes
                // fastest; half anywhere above the horizon.
                let d = crate::geometry::random_point_on_unit_sphere();
                let d = if i % 2 == 0 { (sky.sun_direction + d * 0.17).normalize() } else { d };
                let d = Vector3::new(d.x, d.y.abs(), d.z);
                let exact = sky.radiance(&Ray { ro: Vector3::new(0., *altitude, 0.), rd: d });
                let (rayleigh, mie) = lut.lookup(&d, *altitude).unwrap();
                (sky.apply_phase(&d, rayleigh, mie) - exact).norm() / exact.norm()
            }).fold(0., f64::max)
        }).collect()
    }

    #[test]
    fn test_lut_matches_exact_sky() {
        // Between the tabulated altitudes, with the sun at the horizon: the hardest case, since
        // the sky changes fastest there. (Measured worst errors: under 0.5% in either case.)
        let altitudes = [3., 45., 175.];
        for sun in [Vector3::new(0., 0.02, 1.)] {
            for (altitude, e) in altitudes.iter().zip(lut_errors(sun, &altitudes)) {
                assert!(e < 0.01, "sun {}, altitude {} m: relative error {}", sun, altitude, e);
            }
        }
    }

    #[test]
    fn test_lut_defers_to_exact_above_its_range() {
        let sky = test_sky(Vector3::new(0., 0.5, 1.));
        let lut = SkyViewLut::build(&sky, 8, 4);
        assert!(lut.lookup(&Vector3::new(0., 1., 0.), 500.).is_none());
    }

    #[test]
    fn test_sky_radiance_is_linear() {
        let sky = |brightness: f64| SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: Vector3::new(0., 0.5, 1.).normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
        };
        let r = Ray { ro: Vector3::new(0., 1., 0.), rd: Vector3::new(0., 0.3, 1.).normalize() };
        let (a, b) = (sky(20.).radiance(&r), sky(40.).radiance(&r));
        assert!(a.x > 0.);
        // No display encoding: doubling the light doubles the value.
        assert!((b - a * 2.).norm() < 1e-9 * b.norm());
    }

    #[test]
    fn test_view_ray_stops_at_ground() {
        let sky = SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: Vector3::new(0., 1., 0.),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
        };
        // Camera 10m above the surface, looking straight down.
        let down = Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., -1., 0.) };
        assert!((sky.view_ray_length(&down).unwrap() - 10.).abs() < 1e-3);
        // Looking up, the ray runs to the top of the atmosphere.
        let up = Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., 1., 0.) };
        assert!((sky.view_ray_length(&up).unwrap() - (60e3 - 10.)).abs() < 1e-3);
    }

    #[test]
    fn test_optical_depth() {
        // Create a simple sky material for testing
        let sky = SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., 0., 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., 0., 0.), 6360e3),
            sun_direction: Vector3::new(0., 1., 0.),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
        };
        
        // Test optical depth at different heights
        let segment_length = 1000.0; // 1000m segment
        
        // At height = 0 (sea level)
        let depth_sea_level_r = sky.optical_depth(0.0, sky.rayleigh_thickness, segment_length);
        let depth_sea_level_m = sky.optical_depth(0.0, sky.mie_thickness, segment_length);
        assert_relative_eq!(depth_sea_level_r, segment_length, epsilon = 1e-10);
        assert_relative_eq!(depth_sea_level_m, segment_length, epsilon = 1e-10);
        
        // At one scale height, optical depth should be reduced by factor of e^-1
        let depth_one_scale_r = sky.optical_depth(sky.rayleigh_thickness, sky.rayleigh_thickness, segment_length);
        assert_relative_eq!(depth_one_scale_r, segment_length / std::f64::consts::E, epsilon = 1e-10);
    }
    
    #[test]
    fn test_create_sky_sphere() {
        let json_str = r#"
        {
            "sun_direction": [0, 0.5, 1],
            "brightness": 30.0,
            "rayleigh_thickness": 8000.0,
            "mie_thickness": 1200.0
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj = create_sky_sphere(&value);
        
        // Just verify that a sky sphere was created successfully
        // We test for "infinite" bounds as a characteristic of Infinite geometry
        let bounds = sky_obj.geometry.bounds();
        assert!(bounds.min.x == std::f64::MIN);
        assert!(bounds.max.x == std::f64::MAX);
        
        // Test basic ray intersection - Infinite should always return Some intersection
        let test_ray = Ray {
            ro: Vector3::new(0.0, 0.0, 0.0),
            rd: Vector3::new(0.0, 0.0, 1.0)
        };
        assert!(sky_obj.geometry.intersects(&test_ray).is_some());
    }
    
    #[test]
    fn test_sky_presets() {
        // Test with default preset
        let json_str = r#"
        {
            "preset": "earth",
            "sun_direction": [0, 0.5, 1],
            "brightness": 30.0
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj = create_sky_sphere(&value);
        
        // Test with custom parameters that override preset
        let json_str = r#"
        {
            "preset": "sunset",
            "rayleigh_r": 5.0e-6,
            "mie_coefficient": 30.0e-6,
            "sun_direction": [0, 0.5, 1]
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj_custom = create_sky_sphere(&value);
        
        // Both should create valid objects
        assert!(sky_obj.geometry.intersects(&Ray { ro: Vector3::new(0.0, 0.0, 0.0), rd: Vector3::new(0.0, 0.0, 1.0) }).is_some());
        assert!(sky_obj_custom.geometry.intersects(&Ray { ro: Vector3::new(0.0, 0.0, 0.0), rd: Vector3::new(0.0, 0.0, 1.0) }).is_some());
    }
    
    #[test]
    fn test_get_preset_by_name() {
        // Test getting existing preset
        let preset = get_preset_by_name("sunset");
        assert_eq!(preset.name, "sunset");
        
        // Test getting invalid preset (should return default Earth preset)
        let preset = get_preset_by_name("invalid");
        assert_eq!(preset.name, "earth");
    }
}