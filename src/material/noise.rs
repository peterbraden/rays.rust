use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::noise::{PerlinNoise, WorleyNoise};

/// Noise texture material that modifies the color of a base material
/// based on noise functions.
pub struct NoiseTexture {
    /// Base material model to apply noise to
    pub base_material: Box<dyn MaterialModel + Sync + Send>,
    /// Scale factor for noise coordinates
    pub scale: f64,
    /// Color to blend with base material
    pub color: Color,
    /// Blend factor between base material and noise color (0.0 = base material only, 1.0 = noise only)
    pub blend_factor: f64,
    /// Perlin noise generator
    perlin: PerlinNoise,
    /// Worley noise generator
    worley: Option<WorleyNoise>,
    /// Type of noise to use
    pub noise_type: NoiseType,
}

/// Different types of noise patterns that can be applied
pub enum NoiseType {
    /// Basic Perlin noise
    Perlin,
    /// Fractal Brownian Motion based on Perlin noise
    Fbm {
        octaves: u32,
        persistence: f64,
        lacunarity: f64,
    },
    /// Worley (cellular) noise
    Worley {
        point_density: f64,
        seed: u32,
    },
    /// Combined Perlin and Worley noise
    Marble,
    /// Turbulence pattern based on Perlin noise
    Turbulence {
        octaves: u32,
    },
}

impl NoiseTexture {
    /// Create a new noise texture with Perlin noise
    pub fn new_perlin(
        base_material: Box<dyn MaterialModel + Sync + Send>,
        color: Color,
        scale: f64,
        blend_factor: f64,
    ) -> Self {
        Self {
            base_material,
            scale,
            color,
            blend_factor,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Perlin,
        }
    }

    /// Create a new noise texture with FBM noise
    pub fn new_fbm(
        base_material: Box<dyn MaterialModel + Sync + Send>,
        color: Color,
        scale: f64,
        blend_factor: f64,
        octaves: u32,
        persistence: f64,
        lacunarity: f64,
    ) -> Self {
        Self {
            base_material,
            scale,
            color,
            blend_factor,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Fbm {
                octaves,
                persistence,
                lacunarity,
            },
        }
    }

    /// Create a new noise texture with Worley noise
    pub fn new_worley(
        base_material: Box<dyn MaterialModel + Sync + Send>,
        color: Color,
        scale: f64,
        blend_factor: f64,
        point_density: f64,
        seed: u32,
    ) -> Self {
        Self {
            base_material,
            scale,
            color,
            blend_factor,
            perlin: PerlinNoise::new(),
            worley: Some(WorleyNoise::new(point_density, seed)),
            noise_type: NoiseType::Worley {
                point_density,
                seed,
            },
        }
    }

    /// Create a new noise texture with marble pattern
    pub fn new_marble(
        base_material: Box<dyn MaterialModel + Sync + Send>,
        color: Color,
        scale: f64,
        blend_factor: f64,
    ) -> Self {
        Self {
            base_material,
            scale,
            color,
            blend_factor,
            perlin: PerlinNoise::new(),
            worley: Some(WorleyNoise::new(1.0, 42)),
            noise_type: NoiseType::Marble,
        }
    }

    /// Create a new noise texture with turbulence pattern
    pub fn new_turbulence(
        base_material: Box<dyn MaterialModel + Sync + Send>,
        color: Color,
        scale: f64,
        blend_factor: f64,
        octaves: u32,
    ) -> Self {
        Self {
            base_material,
            scale,
            color,
            blend_factor,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Turbulence { octaves },
        }
    }

    /// Generate turbulence value at a point
    fn turbulence(&self, p: Vector3<f64>, octaves: u32) -> f64 {
        let mut value = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..octaves {
            value += weight * self.perlin.noise(temp_p.x, temp_p.y, temp_p.z).abs();
            weight *= 0.5;
            temp_p *= 2.0;
        }

        value
    }

    /// Calculate noise value at a point based on the selected noise type
    fn noise_value(&self, p: Vector3<f64>) -> f64 {
        let scaled_p = p * self.scale;

        match &self.noise_type {
            NoiseType::Perlin => {
                // Map from [-1,1] to [0,1]
                (self.perlin.noise(scaled_p.x, scaled_p.y, scaled_p.z) + 1.0) * 0.5
            }
            NoiseType::Fbm { octaves, persistence, lacunarity } => {
                self.perlin.fbm(scaled_p.x, scaled_p.y, scaled_p.z, *octaves, *persistence, *lacunarity)
            }
            NoiseType::Worley { .. } => {
                if let Some(worley) = &self.worley {
                    let value = worley.noise(scaled_p.x, scaled_p.y, scaled_p.z);
                    // Normalize Worley noise to [0,1] range (approximately)
                    (1.0 - value.min(1.0)).max(0.0)
                } else {
                    0.5 // Fallback if Worley noise is not initialized
                }
            }
            NoiseType::Marble => {
                let pattern = scaled_p.x + 
                    self.perlin.fbm(
                        scaled_p.x, 
                        scaled_p.y, 
                        scaled_p.z, 
                        4, 0.5, 2.0
                    ) * 10.0;
                
                (pattern.sin() * 0.5 + 0.5).abs()
            }
            NoiseType::Turbulence { octaves } => {
                self.turbulence(scaled_p, *octaves)
            }
        }
    }
}

impl MaterialModel for NoiseTexture {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        // Get the base material's scatter result
        let base_scatter = self.base_material.scatter(r, intersection, s);
        
        // Calculate noise value at the intersection point
        let noise_value = self.noise_value(intersection.point);
        
        // Blend the base material's color with the noise color based on the noise value
        let noise_influence = noise_value * self.blend_factor;

        match base_scatter.ray {
            // Terminal: blend the base's final colour.
            None => ScatteredRay { sampled_lights: base_scatter.sampled_lights, emit: Color::black(),
                ray: None,
                attenuate: (base_scatter.emit + base_scatter.attenuate).blend(&self.color, noise_influence),
            },
            // Blend the bounce weight, and tint any emitted (eg. direct) light by the noise colour.
            Some(ray) => ScatteredRay { sampled_lights: base_scatter.sampled_lights,
                emit: base_scatter.emit * Color::white().blend(&self.color, noise_influence),
                ray: Some(ray),
                attenuate: base_scatter.attenuate.blend(&self.color, noise_influence),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::lambertian::Lambertian;
    
    #[test]
    fn test_noise_texture_keeps_base_emission() {
        use crate::material::legacy::Whitted;
        use crate::scenefile::SceneFile;
        use crate::sceneobject::SceneObject;
        use crate::shapes::plane::Plane;
        use crate::material::texture::Solid;
        use crate::material::legacy::FlatColor;
        use std::sync::Arc;

        let s = SceneFile::from_string(r#"{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.0001, "background": [0, 0, 0], "max_depth": 1,
            "camera": { "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": {}, "media": {}, "objects": [], "variables": {},
            "lights": [ { "location": [0, 10, 0], "intensity": 314.1592653589793 } ]
        }"#.to_string());
        let i = Intersection { part: 0,
            dist: 1.,
            point: Vector3::new(0., 0., 0.),
            normal: Vector3::new(0., 1., 0.),
            object: &(SceneObject {
                geometry: Box::new(Plane { y: 0. }),
                medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
            }),
        };
        let r = Ray { ro: Vector3::new(0., 1., -1.), rd: Vector3::new(0., -1., 1.).normalize() };
        let lit = |reflection: f64| {
            let base = Box::new(Whitted { pigment: Color::white(), reflection, phong: 0. });
            // Zero blend: the texture must not change the base's lighting.
            let t = NoiseTexture::new_perlin(base, Color::new(1., 0., 0.), 0.1, 0.);
            let out = t.scatter(&r, &i, &s);
            out.emit + out.attenuate * if out.ray.is_some() { Color::black() } else { Color::white() }
        };
        assert!((lit(0.).rgb.x - 1.).abs() < 1e-9, "{:?}", lit(0.));
        assert!((lit(0.5).rgb.x - 1.).abs() < 1e-9, "{:?}", lit(0.5));
    }

    #[test]
    fn test_noise_texture_perlin() {
        let base_material = Box::new(Lambertian { albedo: Color::white() });
        let noise_texture = NoiseTexture::new_perlin(
            base_material,
            Color::new(1.0, 0.0, 0.0), // Red noise color
            0.1,                        // Scale
            0.5,                        // Blend factor
        );
        
        // Check that noise values are in the expected range [0,1]
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(x as f64, y as f64, z as f64);
                    let value = noise_texture.noise_value(pos);
                    assert!((0.0..=1.0).contains(&value), "Noise value out of range: {}", value);
                }
            }
        }
    }
    
    #[test]
    fn test_noise_texture_fbm() {
        let base_material = Box::new(Lambertian { albedo: Color::white() });
        let noise_texture = NoiseTexture::new_fbm(
            base_material,
            Color::new(0.0, 1.0, 0.0), // Green noise color
            0.1,                        // Scale
            0.5,                        // Blend factor
            4,                          // Octaves
            0.5,                        // Persistence
            2.0,                        // Lacunarity
        );
        
        // Check that fbm values are in the expected range [0,1]
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(x as f64, y as f64, z as f64);
                    let value = noise_texture.noise_value(pos);
                    assert!((0.0..=1.0).contains(&value), "FBM value out of range: {}", value);
                }
            }
        }
    }
    
    #[test]
    fn test_noise_texture_marble() {
        let base_material = Box::new(Lambertian { albedo: Color::white() });
        let noise_texture = NoiseTexture::new_marble(
            base_material,
            Color::new(0.0, 0.0, 1.0), // Blue noise color
            0.1,                        // Scale
            0.5,                        // Blend factor
        );
        
        // Check that marble values are in the expected range [0,1]
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(x as f64, y as f64, z as f64);
                    let value = noise_texture.noise_value(pos);
                    assert!((0.0..=1.0).contains(&value), "Marble value out of range: {}", value);
                }
            }
        }
    }
}