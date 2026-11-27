use crate::na::{Vector3};
use crate::material::model::MaterialModel;
use crate::noise::{PerlinNoise, WorleyNoise, combined_noise};

pub trait Medium : Sync{
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send); 

    /// The normal to shade with at `pt`, given the surface's: the same, unless the surface is
    /// bump mapped (see `bump::Bumped`).
    fn normal_at(&self, _pt: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> { *normal }
    /// The normal to shade with at a hit: `normal_at`, unless the medium needs more of the hit
    /// than its point, such as a normal map its texture coordinates (`NormalMapped`).
    fn normal_at_hit(&self, i: &crate::intersection::Intersection, _s: &crate::scene::Scene) -> Vector3<f64> {
        self.normal_at(&i.point, &i.normal)
    }
    /// The medium of part `k` of a combined shape (see `csg::Tagged`), if it has its own.
    fn part(&self, _k: u32) -> Option<&(dyn Medium + Sync + Send)> { None }
    /// The material as data, if the medium is one material throughout and it can be
    /// flattened (`wavefront::flat`).
    fn flat_material(&self) -> Option<crate::wavefront::flat::FlatMaterial> { None }
    /// The one material the medium is throughout, if it is one (as `Solid` is): what may be
    /// judged for the whole object. Not what `material_at` gives at some point, which for a
    /// medium that varies (a landscape: water in its rivers, ground elsewhere) is only that
    /// point's.
    fn uniform(&self) -> Option<&(dyn MaterialModel + Sync + Send)> { None }
    /// What covers the surface at the hit `i`, for shading it at vertices (`Covered`): by
    /// default, its material there, baked (`MaterialModel::bake`); None if that can't be.
    fn bake_at(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Option<crate::material::model::Covered> {
        self.material_at(i.point).bake(i, s).map(crate::material::model::Covered::bare)
    }
    /// What the surface knows about itself at its points, for function graphs' `field` nodes
    /// (`graph::Fields`): a landscape's simulation.
    fn fields(&self) -> Option<&dyn crate::graph::Fields> { None }
}

/// The media of an object whose parts have their own: `base` for the object itself (and parts
/// without one), and `parts[k - 1]` for part k.
pub struct PartMedia {
    pub base: Box<dyn Medium + Sync + Send>,
    pub parts: Vec<Box<dyn Medium + Sync + Send>>,
}

impl Medium for PartMedia {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        self.base.material_at(pt)
    }

    fn part(&self, k: u32) -> Option<&(dyn Medium + Sync + Send)> {
        if k == 0 { None } else { self.parts.get(k as usize - 1).map(|m| &**m) }
    }

    fn normal_at(&self, pt: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        self.base.normal_at(pt, normal)
    }

    fn normal_at_hit(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Vector3<f64> {
        self.base.normal_at_hit(i, s)
    }

    fn fields(&self) -> Option<&dyn crate::graph::Fields> { self.base.fields() }
}

/// A medium whose surface is normal mapped (`textures::NormalMap`).
pub struct NormalMapped {
    pub inner: Box<dyn Medium + Sync + Send>,
    pub map: crate::textures::NormalMap,
}

impl Medium for NormalMapped {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        self.inner.material_at(pt)
    }

    fn normal_at_hit(&self, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Vector3<f64> {
        let n = self.inner.normal_at_hit(i, s);
        self.map.perturb(&crate::intersection::Intersection { normal: n, ..*i }, s)
    }

    fn part(&self, k: u32) -> Option<&(dyn Medium + Sync + Send)> {
        self.inner.part(k)
    }

    fn fields(&self) -> Option<&dyn crate::graph::Fields> { self.inner.fields() }
}

pub struct Solid {
    pub m: Box<dyn MaterialModel + Sync + Send> 
}
impl Medium for Solid {
    fn material_at(&self, _pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        &*self.m
    }

    fn flat_material(&self) -> Option<crate::wavefront::flat::FlatMaterial> {
        self.m.flat()
    }

    fn uniform(&self) -> Option<&(dyn MaterialModel + Sync + Send)> {
        Some(&*self.m)
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
    fn flat_material(&self) -> Option<crate::wavefront::flat::FlatMaterial> {
        Some(crate::wavefront::flat::FlatMaterial::Checker {
            m1: Box::new(self.m1.flat()?), m2: Box::new(self.m2.flat()?), xsize: self.xsize, zsize: self.zsize,
        })
    }

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

    /// Re-seed the noise generators, so the same scene can give different patterns. Worley
    /// noise takes the seed's low 32 bits.
    pub fn seeded(mut self, seed: u64) -> Self {
        self.perlin = PerlinNoise::shuffled(seed);
        self.worley = self.worley.map(|w| w.reseeded(seed as u32));
        if let NoiseType::Worley { seed: s, .. } = &mut self.noise_type {
            *s = seed as u32;
        }
        self
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

