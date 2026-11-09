//! Bounded participating media whose density varies through space, such as smoke or cloud.
//!
//! Light travelling through a medium of extinction coefficient σ(x) collides within a short
//! step dt with probability σ(x) dt, so the chance of reaching distance t without colliding is
//! the transmittance exp(-∫₀ᵗ σ). When σ is constant, a collision distance can be drawn directly
//! (see `participatingmedia::HomogenousFog`). When it varies, the integral has no closed form,
//! and `Volume` instead uses delta tracking (E. R. Woodcock et al., "Techniques used in the GEM
//! code for Monte Carlo neutronics calculations in reactors and other systems of complex
//! geometry", Argonne National Laboratory, ANL-7050, 1965; see also Novák et al., "Monte Carlo
//! Methods for Volumetric Light Transport Simulation", Computer Graphics Forum 37(2), 2018,
//! section 4.2):
//!
//! 1. Fill the medium up to a constant majorant σ̄ ≥ σ(x) everywhere with "null" particles that
//!    don't change the light's direction.
//! 2. Sample collisions in that homogeneous medium: steps of -ln(1 - u) / σ̄.
//! 3. At each, the collision is real with probability σ(x) / σ̄; otherwise it was a null
//!    collision, and the walk continues.
//!
//! The first real collision is distributed exactly as in the heterogeneous medium, without any
//! step size to choose. Its cost is the number of null collisions, so the majorant should be
//! tight: mostly-empty volumes with a few dense spots are slow.
//!
//! As geometry, a volume reports the first real collision as its "intersection". Nearer
//! surfaces hide it, and a shadow ray is blocked by a volume exactly when it collides in it,
//! which estimates the transmittance unbiasedly, if noisily, as either 0 or 1.

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::{Intersection, RawIntersection};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::phase::sample_henyey_greenstein;
use crate::material::texture::Solid;
use crate::na::Vector3;
use crate::noise::PerlinNoise;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

/// The extinction coefficient through a volume: the chance of a collision per unit distance.
pub trait DensityField: Sync + Send {
    fn density(&self, p: &Vector3<f64>) -> f64;

    /// An upper bound on `density` everywhere in the volume. Delta tracking is only correct if
    /// it really is a bound, and is fastest if it is close.
    fn majorant(&self) -> f64;
}

/// The same density throughout.
pub struct ConstantDensity(pub f64);

impl DensityField for ConstantDensity {
    fn density(&self, _p: &Vector3<f64>) -> f64 { self.0 }
    fn majorant(&self) -> f64 { self.0 }
}

/// Density from fractal Perlin noise: `max_density` where the noise is highest, falling to
/// nothing where it is below `threshold`, so the volume breaks up into separate puffs.
pub struct NoiseDensity {
    pub perlin: PerlinNoise,
    pub max_density: f64,
    /// Noise features are about 1 / scale across.
    pub scale: f64,
    pub octaves: u32,
    /// Noise values (in [0, 1]) below this are empty space.
    pub threshold: f64,
}

impl DensityField for NoiseDensity {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let q = p * self.scale;
        let n = self.perlin.fbm(q.x, q.y, q.z, self.octaves, 0.5, 2.0);
        self.max_density * ((n - self.threshold) / (1. - self.threshold)).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { self.max_density }
}

/// A participating medium filling a box.
pub struct Volume {
    pub bounds: BBox,
    pub density: Box<dyn DensityField>,
}

impl Geometry for Volume {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_beyond(r, 0.)
    }

    /// The first real collision at least `min` along the ray, by delta tracking. Light that
    /// hasn't collided by `min` continues as if it had just started there, as the chance of
    /// colliding further on doesn't depend on how far it has already come.
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        let majorant = self.density.majorant();
        if majorant <= 0. {
            return None;
        }
        let d = r.rd.normalize();
        let invrd = Vector3::new(1. / d.x, 1. / d.y, 1. / d.z);
        let (entry, exit) = self.bounds.entry_exit(&r.ro, &invrd)?;
        let mut t = entry.max(min);
        loop {
            t -= (1. - rand()).ln() / majorant;
            if t >= exit {
                return None;
            }
            let point = r.ro + d * t;
            if rand() * majorant < self.density.density(&point) {
                // Media have no surface; facing back along the ray keeps the shadow bias from
                // moving the point sideways.
                return Some(RawIntersection { dist: t, point, normal: -d });
            }
        }
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn is_stochastic(&self) -> bool { true }
}

/// What happens at a collision in a volume: the light is scattered with probability `albedo`
/// (per colour channel), in a direction from the Henyey-Greenstein phase function with
/// asymmetry `g`, and otherwise absorbed. Clouds scatter nearly everything (albedo ≈ 1) and
/// mostly forwards (g ≈ 0.85); smoke absorbs more.
pub struct VolumeMaterial {
    pub albedo: Color,
    pub g: f64,
}

impl MaterialModel for VolumeMaterial {
    fn scatter(&self, r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
        let rd = sample_henyey_greenstein(&r.rd.normalize(), self.g, rand(), rand());
        ScatteredRay {
            ray: Some(Ray { ro: i.point, rd }),
            // The direction was drawn from the phase function itself, so phase / pdf = 1.
            attenuate: self.albedo,
            emit: Color::black(),
            sampled_lights: false,
        }
    }
}

pub fn create_volume(bounds: BBox, density: Box<dyn DensityField>, albedo: Color, g: f64) -> SceneObject {
    SceneObject {
        geometry: Box::new(Volume { bounds, density }),
        medium: Box::new(Solid { m: Box::new(VolumeMaterial { albedo, g }) }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::seed_rand;

    fn unit_box() -> BBox {
        BBox::new(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.))
    }

    /// The fraction of `n` rays along +x through the volume at height (y, z) that pass through.
    fn transmitted(v: &Volume, y: f64, z: f64, n: usize) -> f64 {
        let r = Ray { ro: Vector3::new(-1., y, z), rd: Vector3::new(1., 0., 0.) };
        (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64
    }

    #[test]
    fn test_constant_volume_follows_beer_lambert() {
        seed_rand(1, 0);
        let v = Volume { bounds: unit_box(), density: Box::new(ConstantDensity(1.5)) };
        let t = transmitted(&v, 0.5, 0.5, 40_000);
        assert!((t - (-1.5f64).exp()).abs() < 0.01, "{}", t);
    }

    #[test]
    fn test_collisions_lie_within_the_box() {
        seed_rand(2, 0);
        let v = Volume { bounds: unit_box(), density: Box::new(ConstantDensity(3.)) };
        // Unnormalised direction: distances are still world-space.
        let r = Ray { ro: Vector3::new(-2., 0.5, 0.5), rd: Vector3::new(4., 0., 0.) };
        for _ in 0..1000 {
            if let Some(i) = v.intersects(&r) {
                assert!(i.dist >= 2. && i.dist <= 3., "{}", i.dist);
                assert!((i.point.x - (i.dist - 2.)).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn test_ray_starting_inside_or_beyond_min() {
        seed_rand(3, 0);
        let v = Volume { bounds: unit_box(), density: Box::new(ConstantDensity(1.)) };
        let r = Ray { ro: Vector3::new(0.5, 0.5, 0.5), rd: Vector3::new(1., 0., 0.) };
        let n = 40_000;
        // From the centre, half a unit of medium remains; from min = 0.3 along, a fifth.
        let inside = (0..n).filter(|_| v.intersects(&r).is_none()).count() as f64 / n as f64;
        let beyond = (0..n).filter(|_| v.intersects_beyond(&r, 0.3).is_none()).count() as f64 / n as f64;
        assert!((inside - (-0.5f64).exp()).abs() < 0.01, "{}", inside);
        assert!((beyond - (-0.2f64).exp()).abs() < 0.01, "{}", beyond);
        assert!(v.intersects_beyond(&r, 0.3).map_or(true, |i| i.dist >= 0.3));
    }

    #[test]
    fn test_empty_volume_is_clear() {
        let v = Volume { bounds: unit_box(), density: Box::new(ConstantDensity(0.)) };
        assert_eq!(transmitted(&v, 0.5, 0.5, 100), 1.);
    }

    #[test]
    fn test_delta_tracking_matches_optical_depth_of_noise() {
        // Transmittance through varying density is exp(-∫σ); integrate σ finely along the ray
        // and compare with the fraction of tracked rays that get through.
        seed_rand(4, 0);
        let density = NoiseDensity {
            perlin: PerlinNoise::with_seed(9), max_density: 4., scale: 3., octaves: 4, threshold: 0.45,
        };
        let steps = 20_000;
        let optical_depth: f64 = (0..steps).map(|i| {
            density.density(&Vector3::new((i as f64 + 0.5) / steps as f64, 0.37, 0.61))
        }).sum::<f64>() / steps as f64;
        assert!(optical_depth > 0.2 && optical_depth < 3., "uninformative test: {}", optical_depth);
        let v = Volume { bounds: unit_box(), density: Box::new(density) };
        let t = transmitted(&v, 0.37, 0.61, 40_000);
        assert!((t - (-optical_depth).exp()).abs() < 0.01, "{} vs {}", t, (-optical_depth).exp());
    }
}
