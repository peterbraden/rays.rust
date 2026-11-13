//! Mountains: a heightfield of ridged noise around a peak, meshed and lying on the planet, with
//! a material that puts snow, rock and vegetation where they grow.
//!
//! The relief is ridged multifractal noise (F. K. Musgrave, "Texturing and Modeling: A
//! Procedural Approach", ch. 16): one minus the absolute value of Perlin noise, squared, makes
//! sharp ridges and rounded valleys, and weighting each octave by the one before keeps valleys
//! smooth and roughens the ridges. Domain warping bends the ridges. A massif rises around
//! `peak`, and the relief fades to gently rolling plains towards the edges, which follow the
//! planet's curve so the range meets the ground and drops behind the horizon with distance.
//!
//! Snow lies above `snowline` where slopes are gentle enough to hold it, rock shows on steep
//! slopes and above `treeline`, and below are forest and grassland.

use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::functions::{direct_lighting, scatter_lambertian};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::smoothstep;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::mesh::IndexedMesh;

pub struct Terrain {
    /// Middle of the square of terrain ([x, z]), and how far it reaches either side.
    pub center: Vector2<f64>,
    pub extent: f64,
    /// Vertices along each side.
    pub resolution: usize,
    /// The massif: where its summit is, how high it rises (m), and how wide it is.
    pub peak: Vector2<f64>,
    pub peak_height: f64,
    pub peak_radius: f64,
    /// Ridged relief: its height (m) and the size of its largest features.
    pub relief: f64,
    pub feature_size: f64,
    /// Rolling relief of the plains (m).
    pub plains: f64,
    /// Height of the terrain's foot relative to the planet's surface: below zero, its low
    /// ground is under the sea, leaving an island.
    pub base: f64,
    /// The planet's radius, to curve the ground; None for flat.
    pub planet_radius: Option<f64>,
    noise: PerlinNoise,
    warp: PerlinNoise,
}

impl Terrain {
    pub fn new(seed: u64) -> Terrain {
        Terrain {
            center: Vector2::zeros(),
            extent: 30_000.,
            resolution: 768,
            peak: Vector2::zeros(),
            peak_height: 3200.,
            peak_radius: 7000.,
            relief: 900.,
            feature_size: 6000.,
            plains: 40.,
            base: 0.,
            planet_radius: Some(crate::planet::EARTH_RADIUS),
            noise: PerlinNoise::shuffled(seed.wrapping_mul(2).wrapping_add(11)),
            warp: PerlinNoise::shuffled(seed.wrapping_mul(2).wrapping_add(12)),
        }
    }

    /// Ridged multifractal noise in [0, about 1].
    fn ridged(&self, x: f64, z: f64, octaves: u32) -> f64 {
        let (mut sum, mut amplitude, mut frequency, mut weight, mut norm) = (0., 1., 1., 1., 0.);
        for i in 0..octaves {
            let n = self.noise.noise(x * frequency, 0.37 * i as f64, z * frequency);
            let ridge = (1. - n.abs()).powi(2);
            let r = ridge * weight;
            sum += r * amplitude;
            norm += amplitude;
            weight = (r * 2.).clamp(0., 1.);
            amplitude *= 0.5;
            frequency *= 2.03;
        }
        sum / norm
    }

    /// Height above the planet's surface at (`x`, `z`).
    pub fn relief_at(&self, x: f64, z: f64) -> f64 {
        let (u, v) = (x / self.feature_size, z / self.feature_size);
        let w = |o: f64| (self.warp.fbm(u * 0.5, o, v * 0.5, 3, 0.5, 2.) - 0.5) * 1.6;
        let (u, v) = (u + w(0.2), v + w(0.7));
        let r = (Vector2::new(x, z) - self.peak).norm() / self.peak_radius;
        // The massif: a broad, steep-sided dome, with ridges carved into it everywhere it rises.
        let massif = (-r.powf(1.6)).exp();
        let foothills = (-(r / 2.2).powi(2)).exp();
        let ridges = self.ridged(u, v, 8);
        let mountains = self.peak_height * massif * (0.55 + 0.45 * ridges) + self.relief * foothills * (ridges - 0.3);
        let rolling = self.plains * (self.warp.fbm(u * 2., 0.5, v * 2., 4, 0.5, 2.) - 0.5) * 2.;
        // Fade to the plains at the edges, so the terrain meets the ground around it.
        let edge = (Vector2::new(x, z) - self.center).abs().max() / self.extent;
        let fade = 1. - smoothstep(0.8, 1., edge);
        (mountains * fade).max(0.) + rolling * fade + self.base
    }

    /// The surface point at (`x`, `z`): on the curved planet unless flat.
    pub fn point(&self, x: f64, z: f64) -> Vector3<f64> {
        let h = self.relief_at(x, z);
        match self.planet_radius {
            None => Vector3::new(x, h, z),
            Some(radius) => {
                // Straight down from the planet's top, onto the sphere, then up by the relief.
                let up = Vector3::new(x, radius, z).normalize();
                Vector3::new(0., -radius, 0.) + up * (radius + h)
            }
        }
    }

    /// The triangles, with normals from the heightfield's slope at each vertex.
    pub fn mesh(&self) -> IndexedMesh {
        let n = self.resolution.max(2);
        let step = 2. * self.extent / (n - 1) as f64;
        let at = |i: usize, j: usize| (self.center.x - self.extent + i as f64 * step, self.center.y - self.extent + j as f64 * step);
        let points: Vec<Vector3<f64>> = (0..n * n).map(|k| {
            let (x, z) = at(k % n, k / n);
            self.point(x, z)
        }).collect();
        let p = |i: usize, j: usize| points[j * n + i];
        let normals: Vec<Vector3<f64>> = (0..n * n).map(|k| {
            let (i, j) = (k % n, k / n);
            let dx = p((i + 1).min(n - 1), j) - p(i.saturating_sub(1), j);
            let dz = p(i, (j + 1).min(n - 1)) - p(i, j.saturating_sub(1));
            dz.cross(&dx).normalize()
        }).collect();
        let k = |i: usize, j: usize| j * n + i;
        let mut triangles = Vec::with_capacity(2 * (n - 1) * (n - 1));
        for j in 0..n - 1 {
            for i in 0..n - 1 {
                triangles.push([k(i, j), k(i, j + 1), k(i + 1, j)]);
                triangles.push([k(i + 1, j), k(i, j + 1), k(i + 1, j + 1)]);
            }
        }
        IndexedMesh { positions: points, normals, triangles }
    }
}

/// Snow, rock and vegetation by height and slope.
pub struct TerrainMaterial {
    pub snowline: f64,
    pub treeline: f64,
    pub snow: Color,
    pub rock: Color,
    pub forest: Color,
    pub grass: Color,
    pub planet_radius: Option<f64>,
    noise: PerlinNoise,
}

impl TerrainMaterial {
    pub fn new(seed: u64) -> TerrainMaterial {
        TerrainMaterial {
            snowline: 2400.,
            treeline: 1800.,
            snow: Color::new(0.85, 0.87, 0.9),
            rock: Color::new(0.16, 0.14, 0.12),
            forest: Color::new(0.035, 0.05, 0.03),
            grass: Color::new(0.11, 0.12, 0.06),
            planet_radius: Some(crate::planet::EARTH_RADIUS),
            noise: PerlinNoise::shuffled(seed.wrapping_add(31)),
        }
    }

    /// Height above the planet, and the local vertical.
    fn frame(&self, p: &Vector3<f64>) -> (f64, Vector3<f64>) {
        match self.planet_radius {
            None => (p.y, Vector3::y()),
            Some(r) => {
                let c = Vector3::new(0., -r, 0.);
                ((p - c).norm() - r, (p - c).normalize())
            }
        }
    }

    pub fn albedo(&self, p: &Vector3<f64>, normal: &Vector3<f64>) -> Color {
        let (height, up) = self.frame(p);
        let flatness = normal.dot(&up);
        let n = |s: f64, o: f64| self.noise.fbm(p.x / s, o, p.z / s, 4, 0.5, 2.);
        // Snow holds on slopes gentler than about 50°, drifting below the snowline in gullies.
        let line = self.snowline + (n(900., 0.1) - 0.5) * 900.;
        let snow = smoothstep(line - 120., line + 120., height) * smoothstep(0.55, 0.75, flatness);
        // Bare rock on steep slopes, and everywhere above the trees.
        let tree = self.treeline + (n(1500., 0.3) - 0.5) * 500.;
        let rock = (1. - smoothstep(0.6, 0.8, flatness)).max(smoothstep(tree - 100., tree + 150., height));
        let vegetation = self.grass * (1. - smoothstep(0.4, 0.6, n(2500., 0.5))) + self.forest * smoothstep(0.4, 0.6, n(2500., 0.5));
        let below = vegetation * (1. - rock) + self.rock * rock * (0.7 + 0.6 * n(300., 0.7));
        below * (1. - snow) + self.snow * snow
    }
}

impl MaterialModel for TerrainMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let intersection = &intersection.facing(r);
        let albedo = self.albedo(&intersection.point, &intersection.normal);
        ScatteredRay {
            emit: direct_lighting(albedo, intersection, s),
            sampled_lights: true,
            ..scatter_lambertian(albedo, intersection)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_massif_rises_at_the_peak_and_fades_at_the_edges() {
        let t = Terrain::new(1);
        let summit = t.relief_at(0., 0.);
        assert!(summit > 0.5 * t.peak_height, "{}", summit);
        let edge = t.relief_at(t.extent, 0.);
        assert!(edge.abs() < 1e-6, "{}", edge);
        // Far out, only the plains' gentle relief.
        assert!(t.relief_at(0.7 * t.extent, 0.7 * t.extent).abs() < 0.2 * t.peak_height);
    }

    #[test]
    fn test_ground_curves_with_the_planet() {
        let t = Terrain::new(1);
        let p = t.point(0.99 * t.extent, 0.);
        let drop = (0.99 * t.extent).powi(2) / (2. * crate::planet::EARTH_RADIUS);
        assert!((p.y + drop).abs() < 1., "{} vs {}", p.y, -drop);
    }

    #[test]
    fn test_snow_on_high_gentle_slopes_and_not_on_cliffs_or_low() {
        let mut m = TerrainMaterial::new(1);
        m.planet_radius = None;
        let up = Vector3::y();
        let cliff = Vector3::new(1., 0.3, 0.).normalize();
        let high = Vector3::new(0., 3500., 0.);
        assert!((m.albedo(&high, &up).rgb.x - m.snow.rgb.x).abs() < 1e-9);
        assert!(m.albedo(&high, &cliff).rgb.x < 0.5);
        assert!(m.albedo(&Vector3::new(0., 200., 0.), &up).rgb.x < 0.2);
    }
}
