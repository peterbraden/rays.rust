use crate::na::{Vector3};
use crate::material::model::MaterialModel;
use crate::noise::{PerlinNoise, WorleyNoise, combined_noise};

pub trait Medium : Sync{
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send); 
}

pub struct Solid {
    pub m: Box<dyn MaterialModel + Sync + Send> 
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        &*self.m
    }
}

pub struct CheckeredYPlane {
    pub m1: Box<dyn MaterialModel + Sync + Send>,
    pub m2: Box<dyn MaterialModel + Sync + Send>,
    pub xsize: f64,
    pub zsize: f64,
}
impl CheckeredYPlane {
    pub fn new(m1: Box<dyn MaterialModel + Sync + Send>, m2: Box<dyn MaterialModel + Sync + Send>, xsize: f64, zsize: f64) -> CheckeredYPlane {
        CheckeredYPlane { m1, m2, xsize, zsize}
    }
}

impl Medium for CheckeredYPlane {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        let zig = if (pt[0].abs() / self.xsize) as i32 % 2 == 0 { pt[0] > 0. } else { pt[0] <= 0. };
        let zag = if (pt[2].abs() / self.zsize) as i32 % 2 == 0 { pt[2] > 0. } else { pt[2] <= 0. };
        // zig XOR zag
        if zig != zag { &*self.m1 } else { &*self.m2 }
    }
}

/// A medium that mixes between two materials based on noise patterns
pub struct NoiseMedium {
    /// First material (used where noise value is low)
    pub m1: Box<dyn MaterialModel + Sync + Send>,
    /// Second material (used where noise value is high)
    pub m2: Box<dyn MaterialModel + Sync + Send>,
    /// Noise type specifies the kind of noise pattern to use
    pub noise_type: NoiseType,
    /// Scale factor for noise coordinates
    pub scale: f64,
    /// Threshold value for determining material selection (0.0-1.0)
    pub threshold: f64,
    /// Perlin noise generator
    perlin: PerlinNoise,
    /// Worley noise generator (optional, used by some noise types)
    worley: Option<WorleyNoise>,
}

/// Different types of noise patterns available for the NoiseMedium
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
    /// Combined Perlin and Worley noise to create marble-like patterns
    Marble,
    /// Turbulence pattern based on Perlin noise
    Turbulence {
        octaves: u32,
    },
    /// Combined noise with distance-based falloff
    Combined {
        falloff: f64,
    },
}

impl NoiseMedium {
    /// Create a new noise medium with Perlin noise
    pub fn new_perlin(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Perlin,
        }
    }

    /// Create a new noise medium with FBM noise
    pub fn new_fbm(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
        octaves: u32,
        persistence: f64,
        lacunarity: f64,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Fbm {
                octaves,
                persistence,
                lacunarity,
            },
        }
    }

    /// Create a new noise medium with Worley noise
    pub fn new_worley(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
        point_density: f64,
        seed: u32,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: Some(WorleyNoise::new(point_density, seed)),
            noise_type: NoiseType::Worley {
                point_density,
                seed,
            },
        }
    }

    /// Create a new noise medium with marble pattern
    pub fn new_marble(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: Some(WorleyNoise::new(1.0, 42)),
            noise_type: NoiseType::Marble,
        }
    }

    /// Create a new noise medium with turbulence pattern
    pub fn new_turbulence(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
        octaves: u32,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: None,
            noise_type: NoiseType::Turbulence { octaves },
        }
    }

    /// Create a new noise medium with combined noise
    pub fn new_combined(
        m1: Box<dyn MaterialModel + Sync + Send>,
        m2: Box<dyn MaterialModel + Sync + Send>,
        scale: f64,
        threshold: f64,
        falloff: f64,
    ) -> Self {
        Self {
            m1,
            m2,
            scale,
            threshold,
            perlin: PerlinNoise::new(),
            worley: Some(WorleyNoise::new(1.0, 42)),
            noise_type: NoiseType::Combined { falloff },
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
            NoiseType::Combined { falloff } => {
                if let Some(worley) = &self.worley {
                    combined_noise::density_field(p, &self.perlin, worley, self.scale, *falloff)
                } else {
                    0.5 // Fallback if Worley noise is not initialized
                }
            }
        }
    }
}

impl Medium for NoiseMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        // Calculate noise value at the point
        let noise_value = self.noise_value(pt);
        
        // Choose material based on noise value and threshold
        if noise_value >= self.threshold {
            &*self.m2
        } else {
            &*self.m1
        }
    }
}

