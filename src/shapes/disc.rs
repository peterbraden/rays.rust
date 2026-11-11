use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::cone::disc_bounds;

/// A flat disc about `center`, perpendicular to `normal`, optionally with a central hole: an
/// annulus between `hole_radius` and `radius`. The POV-Ray `disc` (POV-Ray 3.7 reference:
/// "Disc"). It has no inside; the reported normal is `normal`, whichever side is hit.
pub struct Disc {
    center: Vector3<f64>,
    normal: Vector3<f64>,
    radius: f64,
    hole_radius: f64,
}

impl Disc {
    pub fn new(center: Vector3<f64>, normal: Vector3<f64>, radius: f64, hole_radius: f64) -> Disc {
        assert!(normal.norm() > 0., "disc normal must not be zero");
        assert!(hole_radius >= 0. && hole_radius < radius, "disc needs 0 <= hole radius < radius");
        Disc { center, normal: normal.normalize(), radius, hole_radius }
    }

    /// Two unit vectors in the plane of the disc, perpendicular to each other.
    fn basis(&self) -> (Vector3<f64>, Vector3<f64>) {
        let n = self.normal;
        let a = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
        let u = n.cross(&a).normalize();
        (u, n.cross(&u))
    }
}

impl Geometry for Disc {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let denom = rd.dot(&self.normal);
        if denom == 0. {
            return None;
        }
        let dist = (self.center - r.ro).dot(&self.normal) / denom;
        if dist <= 0. {
            return None;
        }
        let point = r.ro + rd * dist;
        let d2 = (point - self.center).norm_squared();
        if d2 > self.radius * self.radius || d2 < self.hole_radius * self.hole_radius {
            return None;
        }
        Some(RawIntersection { face: 0, dist, point, normal: self.normal })
    }

    fn bounds(&self) -> BBox {
        disc_bounds(self.center, self.normal, self.radius)
    }

    fn surface_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius - self.hole_radius * self.hole_radius)
    }

    /// Uniform by area over the annulus: the squared radius is uniform between its limits.
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let (h2, r2) = (self.hole_radius * self.hole_radius, self.radius * self.radius);
        let rho = (h2 + u[0] * (r2 - h2)).sqrt();
        let phi = 2. * std::f64::consts::PI * u[1];
        let (a, b) = self.basis();
        Some((self.center + (a * phi.cos() + b * phi.sin()) * rho, self.normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    #[test]
    fn test_annulus() {
        let d = Disc::new(v(0., 0., 0.), v(0., 1., 0.), 2., 1.);
        let down = |x: f64| d.intersects(&Ray { ro: v(x, 3., 0.), rd: v(0., -1., 0.) });
        assert!(down(0.5).is_none());
        assert!((down(1.5).unwrap().dist - 3.).abs() < 1e-9);
        assert!(down(2.5).is_none());
        // From below, the same normal.
        let up = d.intersects(&Ray { ro: v(1.5, -1., 0.), rd: v(0., 1., 0.) }).unwrap();
        assert_eq!(up.normal, v(0., 1., 0.));
    }

    #[test]
    fn test_samples_lie_on_the_annulus() {
        let d = Disc::new(v(1., 2., 3.), v(1., 1., 0.), 2., 1.);
        for i in 0..50 {
            let u = [i as f64 / 50., (i * 7 % 50) as f64 / 50., 0.];
            let (p, _) = d.sample_surface(u).unwrap();
            let rel = p - v(1., 2., 3.);
            assert!(rel.dot(&d.normal).abs() < 1e-9);
            assert!(rel.norm() >= 1. - 1e-9 && rel.norm() <= 2. + 1e-9);
        }
    }
}
