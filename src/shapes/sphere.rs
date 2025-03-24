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
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let dst = r.ro - self.center;
        let a = r.rd.dot(&r.rd);
        let b = dst.dot(&r.rd.normalize());
        let c = dst.dot(&dst) - self.radius * self.radius;

        /*
        if c > 0. && b > 0. {
            // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0) 
            return None;
        }
        */

        let d = b * b - a*c;

        if d < 0. {
            return None
        }

        let mut dist = (-b - d.sqrt()) / a;

        if dist.is_sign_negative() {
            // If dist is negative, ray started inside sphere so find other root 
            dist = (-b + d.sqrt()) / a;
        }
	
		if dist < 0. { return None; }

        let point = r.ro + (r.rd.normalize() * dist);

        return Some(
            RawIntersection {
                dist, 
                point,
                normal: (point - self.center).normalize()
            })
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(&self.center.x - &self.radius, 
                      &self.center.y - &self.radius, 
                      &self.center.z - &self.radius
                      ),
            Vector3::new(&self.center.x + &self.radius, 
                      &self.center.y + &self.radius, 
                      &self.center.z + &self.radius
                      ),
          )
    }
}
