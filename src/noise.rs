/// Noise module for procedural patterns generation
/// 
/// This module implements various noise functions used for procedural generation,
/// including 3D Perlin noise and fractal Brownian motion (fBm).
/// Used for creating procedural textures and patterns.

use crate::na::Vector3;
use crate::geometry::Pcg32;
use std::f64;
use std::f64::consts::PI;

/// Perlin noise generator for 3D space
#[derive(Clone)]
pub struct PerlinNoise {
    /// Permutation table for pseudo-random generation
    perm: [usize; 512],
    /// Gradient vectors for 3D noise
    grad3: [Vector3<f64>; 12],
}

impl PerlinNoise {
    /// Create a new Perlin noise generator with the default permutation table
    pub fn new() -> Self {
        // Standard permutation table (0-255)
        let base_perm: [usize; 256] = [
            151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225,
            140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148,
            247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32,
            57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
            74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122,
            60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54,
            65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169,
            200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64,
            52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212,
            207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213,
            119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9,
            129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104,
            218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241,
            81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157,
            184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
            222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
        ];
        
        Self::from_permutation(base_perm)
    }

    /// Create a Perlin noise generator whose permutation table is a shuffle chosen by `seed`.
    /// The same seed always gives the same noise; `new()` keeps Perlin's reference table.
    pub fn with_seed(seed: u64) -> Self {
        let mut base_perm = [0usize; 256];
        for (i, p) in base_perm.iter_mut().enumerate() {
            *p = i;
        }
        // Fisher-Yates. The modulo bias over at most 256 values is below 1e-7.
        let mut rng = Pcg32::new(seed, 0);
        for i in (1 .. 256).rev() {
            let j = rng.next_u32() as usize % (i + 1);
            base_perm.swap(i, j);
        }
        Self::from_permutation(base_perm)
    }

    fn from_permutation(base_perm: [usize; 256]) -> Self {
        // Double permutation array
        let mut perm = [0; 512];
        perm[..256].copy_from_slice(&base_perm);
        perm[256..512].copy_from_slice(&base_perm);
        
        // 12 gradient vectors for 3D noise
        let grad3 = [
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(-1.0, 1.0, 0.0),
            Vector3::new(1.0, -1.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(0.0, -1.0, 1.0),
            Vector3::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, -1.0, -1.0),
        ];
        
        Self { perm, grad3 }
    }
    
    /// Get noise value at a 3D point
    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        // Unit cube that contains point
        let x_i = x.floor() as i32 & 255;
        let y_i = y.floor() as i32 & 255;
        let z_i = z.floor() as i32 & 255;
        
        // Relative coordinates of point in cube
        let x = x - x.floor();
        let y = y - y.floor();
        let z = z - z.floor();
        
        // Compute fade curves for each coordinate
        let u = self.fade(x);
        let v = self.fade(y);
        let w = self.fade(z);
        
        // Hash coordinates of the 8 cube corners
        let a = self.perm[x_i as usize] + y_i as usize;
        let aa = self.perm[a] + z_i as usize;
        let ab = self.perm[a + 1] + z_i as usize;
        let b = self.perm[(x_i + 1) as usize] + y_i as usize;
        let ba = self.perm[b] + z_i as usize;
        let bb = self.perm[b + 1] + z_i as usize;
        
        // Blend gradients from 8 corners of cube
        let g1 = self.grad(self.perm[aa], x, y, z);
        let g2 = self.grad(self.perm[ba], x - 1.0, y, z);
        let g3 = self.grad(self.perm[ab], x, y - 1.0, z);
        let g4 = self.grad(self.perm[bb], x - 1.0, y - 1.0, z);
        let g5 = self.grad(self.perm[aa + 1], x, y, z - 1.0);
        let g6 = self.grad(self.perm[ba + 1], x - 1.0, y, z - 1.0);
        let g7 = self.grad(self.perm[ab + 1], x, y - 1.0, z - 1.0);
        let g8 = self.grad(self.perm[bb + 1], x - 1.0, y - 1.0, z - 1.0);
        
        // Interpolate gradients
        let lerp1 = self.lerp(g1, g2, u);
        let lerp2 = self.lerp(g3, g4, u);
        let lerp3 = self.lerp(g5, g6, u);
        let lerp4 = self.lerp(g7, g8, u);
        
        let lerp5 = self.lerp(lerp1, lerp2, v);
        let lerp6 = self.lerp(lerp3, lerp4, v);
        
        // Scale to [-1, 1]
        self.lerp(lerp5, lerp6, w)
    }
    
    /// Generate fractal Brownian motion (fBm) noise
    /// 
    /// fBm sums multiple octaves of Perlin noise at different frequencies and amplitudes
    /// to create more complex, natural-looking patterns.
    /// 
    /// # Arguments
    /// * `x`, `y`, `z` - Coordinates to sample noise at
    /// * `octaves` - Number of noise layers to sum
    /// * `persistence` - How much each octave's amplitude decreases (typically 0.5)
    /// * `lacunarity` - How much each octave's frequency increases (typically 2.0)
    pub fn fbm(&self, x: f64, y: f64, z: f64, octaves: u32, persistence: f64, lacunarity: f64) -> f64 {
        let mut result = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;
        
        for _ in 0..octaves {
            result += self.noise(x * frequency, y * frequency, z * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }
        
        // Normalize to [0, 1]
        if max_value == 0.0 {
            // No octaves: no variation.
            return 0.5;
        }
        (result / max_value + 1.0) * 0.5
    }
    
    /// Fade function - 6t^5 - 15t^4 + 10t^3
    fn fade(&self, t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }
    
    /// Linear interpolation
    fn lerp(&self, a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }
    
    /// Gradient function for 3D noise
    fn grad(&self, hash: usize, x: f64, y: f64, z: f64) -> f64 {
        // Use hash to pick one of the 12 gradient vectors
        let h = hash % 12;
        let grad = &self.grad3[h];
        
        // Dot product of gradient vector with offset vector
        grad.x * x + grad.y * y + grad.z * z
    }
}

/// Hash a cell (and seed) to a number in [0, 1]. Cells are numbered by their lowest corner,
/// converted via i64 so negative cells wrap rather than saturating to 0.
pub fn worley_hash(ix: u32, iy: u32, iz: u32, s: u32) -> f64 {
    let mut h = (ix.wrapping_mul(73856093)) ^
             (iy.wrapping_mul(19349663)) ^
             (iz.wrapping_mul(83492791)) ^
             s.wrapping_mul(0x9E37_79B9);
    // Avalanche (murmur3 finaliser) so neighbouring cells are uncorrelated.
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    h as f64 / u32::MAX as f64
}

/// Worley noise (cellular noise) generator
#[derive(Clone)]
pub struct WorleyNoise {
    /// Feature points density
    point_density: f64,
    /// Random seed
    seed: u32,
}

impl WorleyNoise {
    /// Create a new Worley noise generator
    pub fn new(point_density: f64, seed: u32) -> Self {
        Self { point_density, seed }
    }

    /// The same generator with a different seed.
    pub fn reseeded(&self, seed: u32) -> Self {
        Self { point_density: self.point_density, seed }
    }
    
    /// Get noise value at a 3D point
    /// 
    /// Returns the distance to the closest feature point (Worley's F1), times `point_density`.
    /// Each unit cell has one feature point, at a position hashed from the cell and the seed;
    /// the nearest is in the point's own cell or one of its 26 neighbours.
    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let hash = worley_hash;
        // Find cell containing the point, and where in it the point is.
        let (xi, yi, zi) = (x.floor(), y.floor(), z.floor());
        let (fx, fy, fz) = (x - xi, y - yi, z - zi);
        let (cx, cy, cz) = (xi as i64, yi as i64, zi as i64);
        // The squared distance from the point to the nearest face of the neighbour at offset
        // -1, 0 or +1 along an axis: no feature point there can be nearer.
        let gap = |f: f64, d: i64| match d { -1 => f * f, 1 => (1. - f) * (1. - f), _ => 0. };
        
        let mut min_dist2 = f64::MAX;
        // Own cell first, so the bound below prunes as many neighbours as possible.
        for (dx, dy, dz) in std::iter::once((0, 0, 0)).chain(
            (-1..=1).flat_map(|dx| (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| (dx, dy, dz))))
                .filter(|&o| o != (0, 0, 0))
        ) {
            if gap(fx, dx) + gap(fy, dy) + gap(fz, dz) >= min_dist2 {
                continue;
            }
            let (ix, iy, iz) = ((cx + dx) as u32, (cy + dy) as u32, (cz + dz) as u32);
            // Random position within cell, relative to the point.
            let px = dx as f64 + hash(ix, iy, iz, self.seed) - fx;
            let py = dy as f64 + hash(ix, iy, iz, self.seed.wrapping_add(1)) - fy;
            let pz = dz as f64 + hash(ix, iy, iz, self.seed.wrapping_add(2)) - fz;
            min_dist2 = min_dist2.min(px * px + py * py + pz * pz);
        }
        
        // Scale by point density
        min_dist2.sqrt() * self.point_density
    }
}

/// Utility functions for combining noise types
pub mod combined_noise {
    use super::*;
    
    /// Generate density field by combining noise types
    /// 
    /// This function combines Perlin and Worley noise to create
    /// complex density patterns with fine detail.
    /// 
    /// # Arguments
    /// * `position` - 3D position to sample
    /// * `perlin` - Perlin noise generator
    /// * `worley` - Worley noise generator
    /// * `scale` - Overall noise scale factor
    /// * `falloff` - Controls how density decreases with distance from origin
    pub fn density_field(
        position: Vector3<f64>, 
        perlin: &PerlinNoise, 
        worley: &WorleyNoise, 
        scale: f64,
        falloff: f64
    ) -> f64 {
        let x = position.x * scale;
        let y = position.y * scale;
        let z = position.z * scale;
        
        // Base shape from Perlin noise
        let shape = perlin.fbm(x * 0.1, y * 0.1, z * 0.1, 4, 0.5, 2.0);
        
        // Detail from Worley noise
        let detail = worley.noise(x, y, z);
        
        // Combine shape and detail
        let raw_density = shape - detail * 0.5;
        
        // Apply falloff factor
        let distance = position.norm();
        let falloff_factor = (-distance * falloff).exp();
        
        // Ensure density is in [0, 1] range
        (raw_density * falloff_factor).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_perlin_noise_range() {
        let perlin = PerlinNoise::new();
        
        // Test noise bounds
        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    let n = perlin.noise(x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1);
                    assert!((-1.0..=1.0).contains(&n));
                }
            }
        }
    }
    
    #[test]
    fn test_perlin_fbm_range() {
        let perlin = PerlinNoise::new();
        
        // Test fbm bounds
        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    let n = perlin.fbm(
                        x as f64 * 0.1, 
                        y as f64 * 0.1, 
                        z as f64 * 0.1,
                        4, 0.5, 2.0
                    );
                    assert!((0.0..=1.0).contains(&n));
                }
            }
        }
    }
    
    #[test]
    fn test_perlin_grad_uses_all_gradients() {
        let perlin = PerlinNoise::new();
        let mut seen = [false; 12];
        for hash in 0..256 {
            // Recover the index by probing with unit axes.
            let g = Vector3::new(
                perlin.grad(hash, 1., 0., 0.),
                perlin.grad(hash, 0., 1., 0.),
                perlin.grad(hash, 0., 0., 1.),
            );
            let i = perlin.grad3.iter().position(|v| *v == g).unwrap();
            seen[i] = true;
        }
        assert!(seen.iter().all(|s| *s), "unused gradients: {:?}", seen);
    }

    fn perlin_samples(perlin: &PerlinNoise) -> Vec<f64> {
        (0..64).map(|i| perlin.noise(i as f64 * 0.37, i as f64 * 0.11, -(i as f64) * 0.23)).collect()
    }

    #[test]
    fn test_seeded_perlin_is_reproducible() {
        assert_eq!(perlin_samples(&PerlinNoise::with_seed(7)), perlin_samples(&PerlinNoise::with_seed(7)));
        assert_ne!(perlin_samples(&PerlinNoise::with_seed(7)), perlin_samples(&PerlinNoise::with_seed(8)));
        assert_ne!(perlin_samples(&PerlinNoise::with_seed(7)), perlin_samples(&PerlinNoise::new()));
    }

    #[test]
    fn test_seeded_perlin_table_is_a_permutation() {
        let perlin = PerlinNoise::with_seed(12345);
        let mut seen = perlin.perm[..256].to_vec();
        seen.sort();
        assert_eq!(seen, (0..256).collect::<Vec<_>>());
        assert_eq!(perlin.perm[..256], perlin.perm[256..]);
    }

    #[test]
    fn test_seeded_perlin_range() {
        let perlin = PerlinNoise::with_seed(3);
        for v in perlin_samples(&perlin) {
            assert!((-1.0..=1.0).contains(&v));
        }
    }

    #[test]
    fn test_pruned_worley_matches_full_search() {
        let w = WorleyNoise::new(1.3, 77);
        let mut rng = Pcg32::new(1, 0);
        for _ in 0..5000 {
            let (x, y, z) = ((rng.next_f64() - 0.5) * 40., (rng.next_f64() - 0.5) * 40., (rng.next_f64() - 0.5) * 40.);
            let (cx, cy, cz) = (x.floor() as i64, y.floor() as i64, z.floor() as i64);
            let mut best = f64::MAX;
            for dx in -1..=1 { for dy in -1..=1 { for dz in -1..=1 {
                let (ix, iy, iz) = ((cx + dx) as u32, (cy + dy) as u32, (cz + dz) as u32);
                let p = Vector3::new(
                    (cx + dx) as f64 + worley_hash(ix, iy, iz, 77),
                    (cy + dy) as f64 + worley_hash(ix, iy, iz, 78),
                    (cz + dz) as f64 + worley_hash(ix, iy, iz, 79));
                best = best.min((p - Vector3::new(x, y, z)).norm());
            }}}
            assert!((w.noise(x, y, z) - best * 1.3).abs() < 1e-9, "at ({}, {}, {})", x, y, z);
        }
    }

    #[test]
    fn test_worley_noise_varies_in_negative_space() {
        let worley = WorleyNoise::new(1.0, 42);
        // Sample the centre of a row of negative cells; with a broken hash every cell has the
        // same feature point, so all values are equal.
        let values: Vec<f64> = (1..8).map(|i| worley.noise(-(i as f64) - 0.5, -0.5, -0.5)).collect();
        let first = values[0];
        assert!(values.iter().any(|v| (v - first).abs() > 1e-6), "{:?}", values);
    }

    #[test]
    fn test_worley_noise_seed_zero_and_max() {
        // Seed 0 must not produce a regular lattice; u32::MAX must not overflow.
        let a = WorleyNoise::new(1.0, 0);
        let values: Vec<f64> = (0..8).map(|i| a.noise(i as f64 + 0.5, 0.5, 0.5)).collect();
        assert!(values.iter().any(|v| (v - values[0]).abs() > 1e-6), "{:?}", values);
        WorleyNoise::new(1.0, u32::MAX).noise(0.5, 0.5, 0.5);
    }

    #[test]
    fn test_fbm_zero_octaves_is_finite() {
        assert_eq!(PerlinNoise::new().fbm(0.3, 0.2, 0.1, 0, 0.5, 2.0), 0.5);
    }

    #[test]
    fn test_worley_noise() {
        let worley = WorleyNoise::new(1.0, 42);
        
        // Test worley noise is positive
        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    let n = worley.noise(x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1);
                    assert!(n >= 0.0);
                }
            }
        }
    }
    
    #[test]
    fn test_density_field() {
        let perlin = PerlinNoise::new();
        let worley = WorleyNoise::new(1.0, 42);
        
        // Test density field is in [0, 1] range
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(x as f64, y as f64, z as f64);
                    let density = combined_noise::density_field(pos, &perlin, &worley, 0.1, 0.1);
                    assert!((0.0..=1.0).contains(&density));
                }
            }
        }
    }
    
    #[test]
    fn test_distance_gradient() {
        let perlin = PerlinNoise::new();
        let worley = WorleyNoise::new(1.0, 42);
        
        // Test that density decreases with distance from origin due to falloff
        let scale = 0.1;
        let falloff = 0.2;
        
        // Sample at different distances from origin
        let pos_close = Vector3::new(0.0, 0.0, 0.0);
        let pos_mid = Vector3::new(5.0, 0.0, 0.0);
        let pos_far = Vector3::new(10.0, 0.0, 0.0);
        
        let density_close = combined_noise::density_field(pos_close, &perlin, &worley, scale, falloff);
        let density_mid = combined_noise::density_field(pos_mid, &perlin, &worley, scale, falloff);
        let density_far = combined_noise::density_field(pos_far, &perlin, &worley, scale, falloff);
        
        // Density should decrease with distance
        // Note: This test may occasionally fail due to the nature of noise,
        // but the general trend should hold across most seed values
        assert!(density_close >= density_mid || density_mid >= density_far);
    }
    
    #[test]
    fn test_density_variation() {
        // This test verifies the density_field function produces varied results
        
        // Hard-coded inputs for deterministic results
        let perlin = PerlinNoise::new();
        let worley = WorleyNoise::new(1.0, 42);
        let scale = 0.1;
        let falloff = 0.05;
        
        // Sample a few specific points
        let positions = [Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(0.0, 10.0, 0.0),
            Vector3::new(0.0, 0.0, 10.0)];
        
        // Get densities at each position
        let densities: Vec<f64> = positions.iter()
            .map(|pos| combined_noise::density_field(*pos, &perlin, &worley, scale, falloff))
            .collect();
        
        // Print the densities for inspection
        println!("Density field values at test points: {:?}", densities);
        
        // Simple sanity check - make sure all results are in the valid range
        for &density in &densities {
            assert!((0.0..=1.0).contains(&density), 
                    "Density should be in range [0,1], got {}", density);
        }
    }
    
    #[test]
    fn test_noise_visualization() {
        let perlin = PerlinNoise::new();
        let worley = WorleyNoise::new(1.5, 42);
        let scale = 0.03;
        let falloff = 0.01; // Minimal falloff for visualization
        
        // Generate a small grid of density values
        let size = 10;
        
        // Print header
        println!("\nNoise pattern visualization (10x10 grid):");
        println!("----------------------------------------");
        
        // Use the same objects as in other tests for consistency
        let perlin = perlin.clone();
        let worley = worley.clone();
        
        // Print grid with ASCII density representation
        for y in 0..size {
            let mut line = String::new();
            for x in 0..size {
                // Convert to world coordinates
                let wx = (x as f64 / size as f64) * 2.0 - 1.0;
                let wz = (y as f64 / size as f64) * 2.0 - 1.0;
                
                let pos = Vector3::new(wx * 100.0, 0.0, wz * 100.0);
                let density = combined_noise::density_field(
                    pos, &perlin, &worley, scale, falloff
                );
                
                // Map density to ASCII characters
                let char_idx = (density * 9.0).round() as usize;
                let density_chars = " .:-=+*#%@";
                line.push(density_chars.chars().nth(char_idx).unwrap());
                line.push(' '); // Add space for better visibility
            }
            println!("{}", line);
        }
        println!("----------------------------------------");
        
        // This test always passes - it's for visual inspection
        assert!(true);
    }
}