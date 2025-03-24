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

// Constants for atmospheric scattering
const DEFAULT_NUM_SAMPLES: usize = 16;
const DEFAULT_NUM_SAMPLES_LIGHT: usize = 8;

// Default scattering coefficient values (Earth-like)
const DEFAULT_RAYLEIGH_R: f64 = 3.8e-6;
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
        rayleigh_r: 3.8e-6,
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
        rayleigh_r: 3.8e-6,
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
        rayleigh_r: 3.8e-6,
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
    
    /// Calculate the Mie phase function (Henyey-Greenstein approximation)
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
    
    /// Apply a tone mapping function to convert HDR values to displayable values
    ///
    /// # Arguments
    /// * `value` - HDR color value that may be outside the [0,1] range
    fn tone_map(value: f64) -> f64 {
        let threshold = 1.413f64;
        let gamma = 1.0 / 2.2; // Standard gamma correction value
        
        if value < threshold {
            (value * 0.38317f64).powf(gamma)
        } else {
            1.0f64 - (-value).exp()
        }
    }
}

impl MaterialModel for SkyMaterial {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        // Use the provided scattering coefficients
        let beta_r: Vector3<f64> = self.rayleigh_coefficients;
        let mie_coef = self.mie_coefficient;
        let beta_m: Vector3<f64> = Vector3::new(mie_coef, mie_coef, mie_coef);
        
        // Check if the ray intersects the atmosphere
        let atmos_intersection = self.atmosphere.intersects(r);
        if atmos_intersection.is_none() {
            return ScatteredRay { attenuate: Color::black(), ray: None };
        }
        
        // Maximum distance through the atmosphere
        let mut ray_max = atmos_intersection.unwrap().dist; 

        // Check if the ray intersects the earth (if we're above the surface)
        let earth_intersection = self.earth.intersects(r);
        if r.ro.y > self.earth.radius && earth_intersection.is_some() {
            ray_max = earth_intersection.unwrap().dist; 
        }
        
        // Setup for numerical integration
        let num_samples = DEFAULT_NUM_SAMPLES; 
        let num_samples_light = DEFAULT_NUM_SAMPLES_LIGHT; 
        let segment_length = ray_max / num_samples as f64; 

        let mut rayleigh_sum: Vector3<f64> = Vector3::zeros();
        let mut mie_sum: Vector3<f64> = Vector3::zeros();

        // Track accumulated optical depth for more physically accurate extinction calculation
        // This is important for proper sunset colors (see implementation note at top of file)
        let mut optical_depth_r = 0.0;
        let mut optical_depth_m = 0.0;

        // Calculate the angular term (mu) between view and sun directions
        let mu = r.rd.dot(&self.sun_direction);
        
        // Calculate phase functions for Rayleigh and Mie
        let phase_r = Self::rayleigh_phase(mu);
        let phase_m = Self::mie_phase(mu, self.mie_anisotropy);
        
        // Integrate along the view ray through the atmosphere
        for i in 0..num_samples {
            let sample_position = r.ro + (i as f64 * segment_length) * r.rd;
            
            // Calculate height above the earth's surface
            let height = (sample_position - self.atmosphere.center).norm() - self.earth.radius;
            
            // Calculate Rayleigh and Mie optical depths for this segment
            let rayleigh_depth = self.optical_depth(height, self.rayleigh_thickness, segment_length);
            let mie_depth = self.optical_depth(height, self.mie_thickness, segment_length);
            
            // Accumulate optical depth along the view ray
            optical_depth_r += rayleigh_depth;
            optical_depth_m += mie_depth;
            
            // Check if we can see the sun from this position
            let light_ray = Ray { ro: sample_position, rd: self.sun_direction };
            let atmosphere_intersection = self.atmosphere.intersects(&light_ray);
            
            if atmosphere_intersection.is_none() {
                // No intersection with atmosphere, we're outside the atmospheric bounds
                continue;
            }
            
            // Calculate light ray path through atmosphere
            let light_len = atmosphere_intersection.unwrap().dist;
            let segment_length_light = light_len / num_samples_light as f64;
            let mut optical_depth_light_r = 0.0;
            let mut optical_depth_light_m = 0.0;
            
            // Integrate along light ray to the sun
            for j in 0..num_samples_light {
                let sample_position_light = sample_position + (j as f64 * segment_length_light) * self.sun_direction;
                let height_light = (sample_position_light - self.atmosphere.center).norm() - self.earth.radius;
                
                optical_depth_light_r += self.optical_depth(height_light, self.rayleigh_thickness, segment_length_light);
                optical_depth_light_m += self.optical_depth(height_light, self.mie_thickness, segment_length_light);
            }
            
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
        
        // Calculate final attenuate value combining Rayleigh and Mie scattering
        let attenuate_vec = (rayleigh_sum.component_mul(&beta_r) * phase_r + 
                          mie_sum.component_mul(&beta_m) * phase_m) * self.brightness; 
        
        // Apply tone mapping function to each channel
        let attenuate = Color::new(
            Self::tone_map(attenuate_vec.x),
            Self::tone_map(attenuate_vec.y),
            Self::tone_map(attenuate_vec.z),
        );

        ScatteredRay { attenuate, ray: None }
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
    
    SceneObject {
        geometry: Box::new(Infinite {}),
        medium: Box::new(Solid { m: Box::new(SkyMaterial {
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
        }) })
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
    fn test_tone_map() {
        // Test values within the threshold
        assert_relative_eq!(SkyMaterial::tone_map(0.0), 0.0, epsilon = 1e-10);
        assert_relative_eq!(SkyMaterial::tone_map(1.0), (0.38317f64).powf(1.0/2.2), epsilon = 1e-5);
        
        // Test values above the threshold
        assert_relative_eq!(SkyMaterial::tone_map(2.0), 1.0 - (-2.0f64).exp(), epsilon = 1e-10);
        assert_relative_eq!(SkyMaterial::tone_map(5.0), 1.0 - (-5.0f64).exp(), epsilon = 1e-10);
        
        // Check monotonicity - higher values should map to higher results
        assert!(SkyMaterial::tone_map(0.5) < SkyMaterial::tone_map(1.0));
        assert!(SkyMaterial::tone_map(1.0) < SkyMaterial::tone_map(2.0));
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