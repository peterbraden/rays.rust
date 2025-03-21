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
    /// # Arguments
    /// * `height` - Height above the earth's surface in meters
    /// * `thickness` - Scale height parameter (rayleigh_thickness or mie_thickness)
    /// * `segment_length` - Length of the ray segment being calculated
    fn optical_depth(&self, height: f64, thickness: f64, segment_length: f64) -> f64 {
        (-height / thickness).exp() * segment_length
    }
    
    /// Calculate the scattering for a single sample point along the view ray
    ///
    /// # Arguments
    /// * `sample_position` - Position of the sample in world space
    /// * `segment_length` - Length of the ray segment
    /// * `sun_direction` - Direction to the sun
    /// * `beta_r` - Rayleigh scattering coefficients for RGB wavelengths
    /// * `beta_m` - Mie scattering coefficients for RGB wavelengths
    /// * `num_samples_light` - Number of samples for the light ray
    fn calculate_sample_scattering(
        &self,
        sample_position: Vector3<f64>,
        segment_length: f64,
        beta_r: Vector3<f64>,
        beta_m: Vector3<f64>,
        num_samples_light: usize,
    ) -> (f64, f64, Vector3<f64>, Vector3<f64>) {
        // Calculate height above the earth's surface
        let height = (sample_position - self.atmosphere.center).norm() - self.earth.radius;
        
        // Calculate Rayleigh and Mie optical depths for the view ray segment
        let rayleigh_depth = self.optical_depth(height, self.rayleigh_thickness, segment_length);
        let mie_depth = self.optical_depth(height, self.mie_thickness, segment_length);
        
        // Check if we can see the sun from this position
        let light_ray = Ray { ro: sample_position, rd: self.sun_direction };
        let atmosphere_intersection = self.atmosphere.intersects(&light_ray);
        
        if atmosphere_intersection.is_none() {
            // No intersection with atmosphere, we're outside the atmospheric bounds
            return (rayleigh_depth, mie_depth, Vector3::zeros(), Vector3::zeros());
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
        let tau = beta_r * (rayleigh_depth + optical_depth_light_r) + 
                  beta_m * 1.1 * (mie_depth + optical_depth_light_m);
        
        // Calculate attenuation using Beer's Law
        let attenuation = Vector3::new((-tau.x).exp(), (-tau.y).exp(), (-tau.z).exp());
        
        // Return Rayleigh and Mie contributions
        (
            rayleigh_depth,
            mie_depth,
            attenuation.component_mul(&Vector3::new(rayleigh_depth, rayleigh_depth, rayleigh_depth)),
            attenuation.component_mul(&Vector3::new(mie_depth, mie_depth, mie_depth))
        )
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
        // Scattering coefficients for Rayleigh (RGB-dependent) and Mie (wavelength-independent)
        let beta_r: Vector3<f64> = Vector3::new(3.8e-6_f64, 13.5e-6_f64, 33.1e-6_f64); 
        let beta_m: Vector3<f64> = Vector3::new(21e-6_f64, 21e-6_f64, 21e-6_f64);
        
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

        // These variables track accumulated optical depth but we're not using them directly
        // as we calculate contributions in the calculate_sample_scattering function
        let mut _optical_depth_r = 0.0;
        let mut _optical_depth_m = 0.0;

        // Calculate the angular term (mu) between view and sun directions
        let mu = r.rd.dot(&self.sun_direction);
        
        // Calculate phase functions for Rayleigh and Mie
        let phase_r = Self::rayleigh_phase(mu);
        let g = 0.76; // Mie anisotropy factor (controls the size of the glow around the sun)
        let phase_m = Self::mie_phase(mu, g);
        
        // Integrate along the view ray through the atmosphere
        for i in 0..num_samples {
            let sample_position = r.ro + (i as f64 * segment_length) * r.rd;
            
            // Calculate scattering for this sample
            let (r_depth, m_depth, r_contrib, m_contrib) = self.calculate_sample_scattering(
                sample_position,
                segment_length,
                beta_r,
                beta_m,
                num_samples_light
            );
            
            _optical_depth_r += r_depth;
            _optical_depth_m += m_depth;
            rayleigh_sum = rayleigh_sum + r_contrib;
            mie_sum = mie_sum + m_contrib;
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

/// Creates a sky sphere object from JSON configuration
///
/// # Arguments
/// * `o` - JSON value containing the sky configuration
///
/// # Returns
/// A SceneObject representing the sky sphere
pub fn create_sky_sphere(o: &Value) -> SceneObject {
    let earth_location = SceneFile::parse_vec3_def(o, "earth_location", Vector3::new(0., -6360e3, 0.));
    
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
            rayleigh_thickness: SceneFile::parse_number(&o["rayleigh_thickness"], 7994.),
            mie_thickness: SceneFile::parse_number(&o["mie_thickness"], 1200.),
            sun_direction: SceneFile::parse_vec3_def(o, "sun_direction", Vector3::new(0., 0.5, 2.)).normalize(),
            brightness: SceneFile::parse_number(&o["brightness"], 20.)
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
}