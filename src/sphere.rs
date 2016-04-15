use sceneobject::SceneObject;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;
use material::Material;

#[derive(PartialEq)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64
}

impl Sphere{
    pub fn new(center:Vec3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}


impl SceneObject for Sphere {
    fn intersects(&self, r: &Ray) -> Option<Intersection> {
        let dst = r.ro - self.center;
        let b = dst.dot(&r.rd.normalize());
        let c = dst.dot(&dst) - self.radius * self.radius;

        if c.is_sign_positive() && b.is_sign_positive() {
            // Exit if r’s origin outside s (c > 0) and r pointing away from s (b > 0) 
            return None;
        }

        let d = b * b - c;

        if d.is_sign_negative() {
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

    fn get_material(&self, _: Vec3<f64>) -> Material {
        Material::demo()
    }
}
