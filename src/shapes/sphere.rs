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

        let point = r.ro + (rd * dist);

        Some(
            RawIntersection { part: 0,
                dist, 
                point,
                normal: (point - self.center).normalize()
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
                RawIntersection { part: 0, dist, point, normal: (point - self.center).normalize() }
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
