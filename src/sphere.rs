use sceneobject::SceneObject;
use na::{Vector3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;
use material::Material;
use bbox::BBox;

#[derive(PartialEq)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Material
}

impl Sphere{
    pub fn new(center:Vector3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Material::demo()
        }
    }
}


impl SceneObject for Sphere {
    fn intersects(&self, r: &Ray) -> Option<Intersection> {
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
            Intersection {
                dist: dist, 
                point: point,
                normal: (point - self.center).normalize(),
                object: self
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

    fn get_material(&self, _: Vector3<f64>) -> Material {
        return self.material.clone();
    }
}
