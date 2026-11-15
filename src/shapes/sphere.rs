use crate::shapes::geometry::Geometry;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;

#[derive(PartialEq, Clone)]
pub struct Sphere {
	pub center: Vector3<f64>,
	pub radius: f64,
}

impl Sphere{
    pub fn new(center:Vector3<f64>, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}


impl Geometry for Sphere {
    fn surface_area(&self) -> f64 {
        4. * std::f64::consts::PI * self.radius * self.radius
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let n = crate::geometry::point_on_unit_sphere(u[0], u[1]);
        Some((self.center + n * self.radius, n))
    }

    /// Sample the cone of directions the sphere subtends from `from` (PBR section 6.2.4): every
    /// sample is on the visible cap, with density 1 / (2π (1 - cos θ_max)) per solid angle, where
    /// a uniform point on the whole sphere is hidden behind it half the time or more. From
    /// inside, by area. For small cones, cos θ comes from a Taylor series in sin² θ, as in pbrt,
    /// which keeps precision for distant lights.
    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let to_center = self.center - from;
        let d2 = to_center.norm_squared();
        let r2 = self.radius * self.radius;
        if d2 <= r2 * (1. + 1e-9) {
            let (y, n) = self.sample_surface(u)?;
            let pdf = crate::shapes::geometry::area_pdf(from, &y, &n, self.surface_area());
            return (pdf > 0. && pdf.is_finite()).then_some((y, n, pdf));
        }
        let sin2_max = r2 / d2;
        let sin_max = sin2_max.sqrt();
        let mut one_minus_cos_max = 1. - (1. - sin2_max).max(0.).sqrt();
        let mut cos_theta = (1. - one_minus_cos_max - 1.) * u[0] + 1.;
        let mut sin2_theta = 1. - cos_theta * cos_theta;
        if sin2_max < 0.00068523 {
            // sin² 1.5°
            sin2_theta = sin2_max * u[0];
            cos_theta = (1. - sin2_theta).sqrt();
            one_minus_cos_max = sin2_max / 2.;
        }
        // The angle at the centre between the direction back to `from` and the sampled point.
        let cos_alpha = sin2_theta / sin_max + cos_theta * (1. - sin2_theta / sin2_max).max(0.).sqrt();
        let sin_alpha = (1. - cos_alpha * cos_alpha).max(0.).sqrt();
        let phi = 2. * std::f64::consts::PI * u[1];
        let z = to_center / d2.sqrt();
        let (a, b) = crate::material::phase::orthonormal_basis(&z);
        // The normal points back towards `from`, at angle alpha from it.
        let n = -(a * (sin_alpha * phi.cos()) + b * (sin_alpha * phi.sin()) + z * cos_alpha);
        Some((self.center + n * self.radius, n, 1. / (2. * std::f64::consts::PI * one_minus_cos_max)))
    }

    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        let d2 = (self.center - from).norm_squared();
        let r2 = self.radius * self.radius;
        if d2 <= r2 * (1. + 1e-9) {
            return crate::shapes::geometry::area_pdf(from, y, n, self.surface_area());
        }
        let sin2_max = r2 / d2;
        let one_minus_cos_max = if sin2_max < 0.00068523 { sin2_max / 2. } else { 1. - (1. - sin2_max).sqrt() };
        1. / (2. * std::f64::consts::PI * one_minus_cos_max)
    }

    /// Longitude and latitude, y up (PBR section 6.2, with pbrt's z up turned to y): u = φ / 2π
    /// round from +x towards +z, v from 0 at the bottom to 1 at the top.
    fn uv(&self, p: &Vector3<f64>, _face: u32) -> Option<crate::na::Vector2<f64>> {
        let d = (p - self.center) / self.radius;
        let phi = d.z.atan2(d.x).rem_euclid(2. * std::f64::consts::PI);
        Some(crate::na::Vector2::new(phi / (2. * std::f64::consts::PI), 1. - d.y.clamp(-1., 1.).acos() / std::f64::consts::PI))
    }

    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Solve with a unit direction so that `dist` is a world-space distance even when the
        // caller's ray direction is not normalised.
        let rd = r.rd.normalize();
        let dst = r.ro - self.center;
        let b = dst.dot(&rd);
        let c = dst.dot(&dst) - self.radius * self.radius;

        /*
        if c > 0. && b > 0. {
            // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0) 
            return None;
        }
        */

        let d = b * b - c;

        if d < 0. {
            return None
        }

        let mut dist = -b - d.sqrt();

        if dist.is_sign_negative() {
            // If dist is negative, ray started inside sphere so find other root 
            dist = -b + d.sqrt();
        }
	
		if dist < 0. { return None; }

        // Reprojected onto the sphere, which removes the error of computing it along the ray
        // (PBR section 6.8.5).
        let normal = (r.ro + rd * dist - self.center).normalize();
        let point = self.center + normal * self.radius;

        Some(
            RawIntersection { face: 0, part: 0,
                dist,
                point,
                normal,
            })
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        let rd = r.rd.normalize();
        let dst = r.ro - self.center;
        let b = dst.dot(&rd);
        let d = b * b - (dst.dot(&dst) - self.radius * self.radius);
        if d < 0. {
            return vec![];
        }
        [-b - d.sqrt(), -b + d.sqrt()].into_iter()
            .filter(|t| *t >= 0.)
            .map(|dist| {
                let point = r.ro + rd * dist;
                RawIntersection { face: 0, part: 0, dist, point, normal: (point - self.center).normalize() }
            })
            .collect()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        (p - self.center).norm_squared() < self.radius * self.radius
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(self.center.x - self.radius, 
                      self.center.y - self.radius, 
                      self.center.z - self.radius
                      ),
            Vector3::new(self.center.x + self.radius, 
                      self.center.y + self.radius, 
                      self.center.z + self.radius
                      ),
          )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_hit(rd: Vector3<f64>) {
        let s = Sphere::new(Vector3::new(0., 0., 0.), 1.);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd };
        let i = s.intersects(&r).expect("ray should hit sphere");
        assert!((i.dist - 4.).abs() < 1e-9, "dist {} for rd {}", i.dist, rd);
        assert!((i.point - Vector3::new(0., 0., -1.)).norm() < 1e-9);
    }

    #[test]
    fn test_intersects_unit_direction() {
        assert_hit(Vector3::new(0., 0., 1.));
    }

    #[test]
    fn test_intersects_non_unit_direction() {
        // Scattered rays (eg. Lambertian) are not normalised.
        assert_hit(Vector3::new(0., 0., 0.5));
        assert_hit(Vector3::new(0., 0., 10.));
    }

    #[test]
    fn test_crossings_and_inside() {
        let s = Sphere::new(Vector3::new(0., 0., 0.), 1.);
        let c = s.crossings(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 2.) });
        assert_eq!(c.len(), 2);
        assert!((c[0].dist - 4.).abs() < 1e-9 && (c[1].dist - 6.).abs() < 1e-9);
        assert_eq!(c[1].normal, Vector3::new(0., 0., 1.));
        assert_eq!(s.crossings(&Ray { ro: Vector3::zeros(), rd: Vector3::new(1., 0., 0.) }).len(), 1);
        assert!(s.inside(&Vector3::new(0.5, 0., 0.)) && !s.inside(&Vector3::new(1.5, 0., 0.)));
    }

    #[test]
    fn test_intersects_from_inside() {
        let s = Sphere::new(Vector3::new(0., 0., 0.), 1.);
        let r = Ray { ro: Vector3::new(0., 0., 0.), rd: Vector3::new(0., 0., 2.) };
        let i = s.intersects(&r).unwrap();
        assert!((i.dist - 1.).abs() < 1e-9);
    }
}

#[cfg(test)]
mod sampling_tests {
    use super::*;
    use crate::geometry::Pcg32;

    /// Every cone sample is on the sphere and visible from outside it, its density is the
    /// cone's, and weighted samples integrate the solid angle the sphere subtends.
    #[test]
    fn test_sphere_cone_sampling() {
        let s = Sphere::new(Vector3::new(0., 0., 5.), 1.);
        let mut rng = Pcg32::new(2, 9);
        for from in [Vector3::zeros(), Vector3::new(0.3, -0.2, 3.5)] {
            let d = (s.center - from).norm();
            let solid = 2. * std::f64::consts::PI * (1. - (1. - 1. / (d * d)).sqrt());
            let n = 20_000;
            let mut sum = 0.;
            for _ in 0..n {
                let (y, normal, pdf) = s.sample_from(&from, [rng.next_f64(), rng.next_f64(), rng.next_f64()]).unwrap();
                assert!(((y - s.center).norm() - 1.).abs() < 1e-9);
                assert!(normal.dot(&(from - y)) >= -1e-9, "sample on the hidden side");
                assert!((pdf - s.pdf_from(&from, &y, &normal)).abs() < 1e-9 * pdf);
                sum += 1. / pdf;
            }
            assert!((sum / n as f64 - solid).abs() < 1e-9 * solid, "{} vs {}", sum / n as f64, solid);
        }
        // A distant sphere, in the Taylor series' range: the density is still the cone's.
        let far = Sphere::new(Vector3::new(0., 0., 1e4), 1.);
        let (_, _, pdf) = far.sample_from(&Vector3::zeros(), [0.3, 0.6, 0.]).unwrap();
        let expected = 1. / (std::f64::consts::PI * 1e-8);
        assert!((pdf / expected - 1.).abs() < 1e-6, "{} vs {}", pdf, expected);
    }
}
