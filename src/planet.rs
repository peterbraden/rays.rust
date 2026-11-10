//! The planet the scene stands on: a sphere, with the scene's origin on its surface and y up.
//!
//! Over a few kilometres the ground is flat, but cloud layers stretch for hundreds: a layer at
//! constant height over a flat plane runs out to the horizon and beyond, where on a planet it
//! curves away below it, its distant parts dropping (d² / 2R: 800 m at 100 km) and finally
//! hidden. So density fields written in flat coordinates, (x, height, z), are evaluated on the
//! planet through `PlanetMapping`: each point in the scene is mapped to planet-local
//! coordinates, its altitude above the sphere and its position along the surface, by the
//! azimuthal equidistant projection about the point beneath the scene's origin. Distances
//! along the surface from there are kept exactly, and in any direction distortion is under 1%
//! out to 1500 km, so over a scene the fields keep their shapes while curving with the planet.

use crate::na::{Vector2, Vector3};
use crate::shapes::geometry::Geometry;
use crate::volume::DensityField;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Planet {
    pub radius: f64,
}

/// The Earth's mean radius, as the skysphere's.
pub const EARTH_RADIUS: f64 = 6_360_000.;

impl Planet {
    /// The centre: straight below the scene's origin, which is on the surface.
    pub fn center(&self) -> Vector3<f64> {
        Vector3::new(0., -self.radius, 0.)
    }

    pub fn altitude(&self, p: &Vector3<f64>) -> f64 {
        (p - self.center()).norm() - self.radius
    }

    /// Planet-local coordinates of `p`: (distance along the surface eastwards, altitude,
    /// northwards), the azimuthal equidistant projection about the origin's foot. Near the
    /// origin this is (x, y, z).
    pub fn local_coordinates(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let v = p - self.center();
        let r = v.norm();
        let horizontal = Vector2::new(v.x, v.z);
        let sideways = horizontal.norm();
        // The angle from the pole, by atan2 so it is exact near the origin.
        let theta = sideways.atan2(v.y);
        let along = if sideways > 0. { horizontal * (self.radius * theta / sideways) } else { Vector2::zeros() };
        Vector3::new(along.x, r - self.radius, along.y)
    }

    /// The inverse of `local_coordinates`.
    pub fn scene_point(&self, q: &Vector3<f64>) -> Vector3<f64> {
        let along = Vector2::new(q.x, q.z);
        let arc = along.norm();
        let theta = arc / self.radius;
        let r = self.radius + q.y;
        let dir = if arc > 0. { along / arc } else { Vector2::zeros() };
        self.center() + Vector3::new(dir.x * theta.sin() * r, theta.cos() * r, dir.y * theta.sin() * r)
    }

    /// World bounds of the planet-local box `local`: the layer it describes curves down away
    /// from the origin, and bulges slightly outwards.
    pub fn bounds_of(&self, local: &crate::shapes::bbox::BBox) -> crate::shapes::bbox::BBox {
        let reach = [local.min.x.abs(), local.max.x.abs()].iter().cloned().fold(0., f64::max)
            .hypot([local.min.z.abs(), local.max.z.abs()].iter().cloned().fold(0., f64::max));
        let theta = (reach / self.radius).min(std::f64::consts::PI);
        let low = (self.radius + local.min.y) * theta.cos() - self.radius;
        let pad = (local.max.y.max(0.) * theta).max(1.);
        crate::shapes::bbox::BBox::new(
            Vector3::new(local.min.x - pad, low.min(local.min.y) - 1., local.min.z - pad),
            Vector3::new(local.max.x + pad, local.max.y + 1., local.max.z + pad),
        )
    }
}

/// How far a local majorant through `PlanetMapping` is trusted along a straight ray: over
/// this, the ray's altitude departs from the straight line in planet-local coordinates by
/// L² / 2R, 8 cm, which the fields' bounds aren't padded for.
const CURVED_STEP: f64 = 1000.;

/// A density field written in flat coordinates, evaluated on a planet (see the module docs).
pub struct PlanetMapping {
    pub planet: Planet,
    pub field: Arc<dyn DensityField>,
}

impl DensityField for PlanetMapping {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.field.density(&self.planet.local_coordinates(p))
    }

    fn majorant(&self) -> f64 {
        self.field.majorant()
    }

    fn minorant(&self) -> f64 {
        self.field.minorant()
    }

    /// The field's local majorant along the ray's direction in planet-local coordinates, its
    /// length converted back to the scene's, and trusted only `CURVED_STEP` at a time.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let q = self.planet.local_coordinates(p);
        let eps = 1.;
        let dq = (self.planet.local_coordinates(&(p + d * eps)) - q) / eps;
        let speed = dq.norm();
        if speed <= 0. {
            return (self.majorant(), CURVED_STEP);
        }
        let (majorant, length) = self.field.majorant_from(&q, &(dq / speed));
        (majorant, (length / speed).min(CURVED_STEP))
    }

    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> {
        self.field.particle_radius(&self.planet.local_coordinates(p))
    }
}

/// The planet's surface, as scene geometry: a sphere.
pub fn ground(planet: &Planet) -> Box<dyn Geometry + Sync + Send> {
    Box::new(crate::shapes::sphere::Sphere::new(planet.center(), planet.radius))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::volume::ConstantDensity;

    #[test]
    fn test_local_coordinates() {
        let planet = Planet { radius: EARTH_RADIUS };
        // At the origin, and nearby, local coordinates are scene coordinates.
        for p in [Vector3::zeros(), Vector3::new(30., 2., -40.), Vector3::new(0., 9000., 0.)] {
            assert!((planet.local_coordinates(&p) - p).norm() < 1e-3, "{}", p);
        }
        // The ground 100 km out on a flat plane is 786 m above the planet.
        let q = planet.local_coordinates(&Vector3::new(100_000., 0., 0.));
        assert!((q.y - 100_000f64.powi(2) / (2. * EARTH_RADIUS)).abs() < 1., "{}", q);
        // Round trips, near and far.
        for q in [Vector3::new(5000., 9000., -3000.), Vector3::new(-400_000., 2500., 700_000.)] {
            assert!((planet.local_coordinates(&planet.scene_point(&q)) - q).norm() < 1e-4, "{}", q);
        }
        // Along the surface, distances from the origin are kept exactly.
        let far = planet.scene_point(&Vector3::new(0., 0., 1_000_000.));
        let angle = (far - planet.center()).normalize().dot(&Vector3::new(0., 1., 0.)).acos();
        assert!((angle * EARTH_RADIUS - 1_000_000.).abs() < 1e-3);
    }

    #[test]
    fn test_distant_clouds_are_hidden_by_the_planet() {
        // From 2 m, a cloud 1 km up is in sight out to sqrt(2R x 1000) + sqrt(2R x 2) = 118 km;
        // further away the planet's bulge is in the way.
        let planet = Planet { radius: EARTH_RADIUS };
        let eye = Vector3::new(0., 2., 0.);
        let hidden = |distance: f64| {
            let cloud = planet.scene_point(&Vector3::new(0., 1000., distance));
            let r = crate::ray::Ray { ro: eye, rd: (cloud - eye).normalize() };
            ground(&planet).intersects(&r).is_some_and(|hit| hit.dist < (cloud - eye).norm())
        };
        assert!(!hidden(80_000.) && !hidden(115_000.));
        assert!(hidden(125_000.) && hidden(300_000.));
        // The layer, mapped, is where it should be along the way: a ray to the cloud at 80 km
        // meets 1 km of altitude there.
        let field = PlanetMapping { planet, field: Arc::new(LayerAt { low: 999., high: 1001. }) };
        let cloud = planet.scene_point(&Vector3::new(0., 1000., 80_000.));
        assert!(field.density(&cloud) > 0.);
    }

    struct LayerAt { low: f64, high: f64 }
    impl DensityField for LayerAt {
        fn density(&self, p: &Vector3<f64>) -> f64 { if p.y >= self.low && p.y <= self.high { 1. } else { 0. } }
        fn majorant(&self) -> f64 { 1. }
    }

    #[test]
    fn test_bounds_contain_the_curved_layer() {
        let planet = Planet { radius: EARTH_RADIUS };
        let local = crate::shapes::bbox::BBox::new(Vector3::new(-100_000., 8000., -100_000.), Vector3::new(100_000., 10_000., 100_000.));
        let world = planet.bounds_of(&local);
        for x in [-100_000., 0., 100_000.] {
            for z in [-100_000., 0., 100_000.] {
                for y in [8000., 10_000.] {
                    assert!(world.contains_point(&planet.scene_point(&Vector3::new(x, y, z))), "{} {} {}", x, y, z);
                }
            }
        }
        let _ = ConstantDensity(1.);
    }
}
