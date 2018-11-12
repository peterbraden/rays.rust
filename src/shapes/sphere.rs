use shapes::geometry::Geometry;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::RawIntersection;
use bbox::BBox;

#[derive(PartialEq)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

impl Sphere{
    pub fn new(center:Vec3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}


impl Geometry for Sphere {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let dst = r.ro - self.center;
        let b = dst.dot(&r.rd.normalize());
        let c = dst.dot(&dst) - self.radius * self.radius;

        if c > 0. && b > 0. {
            // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0) 
            return None;
        }

        let d = b * b - c;

        if d < 0. {
            return None
        }

        let mut dist = -b - d.sqrt();

        // If dist is negative, ray started inside sphere so clamp t to zero 
        if dist.is_sign_negative() {
             dist = 0f64;
        }

        let point = r.ro + (r.rd.normalize() * dist);

        return Some(
            RawIntersection {
                dist: dist, 
                point: point,
                normal: (point - self.center).normalize()
            })
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vec3::new(&self.center.x - &self.radius, 
                      &self.center.y - &self.radius, 
                      &self.center.z - &self.radius
                      ),
            Vec3::new(&self.center.x + &self.radius, 
                      &self.center.y + &self.radius, 
                      &self.center.z + &self.radius
                      ),
          )
    }
}
