use sceneobject::SceneObject;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;
use material::Material;

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
        let d = b * b - c;

        if d > 0f64 {
            let dist = -b - d.sqrt();
            let point = r.ro + (r.rd.normalize() * dist);

            return Some(
                Intersection {
                    dist: dist, 
                    point: point,
                    normal: (point - self.center).normalize(),
                    object: self
                })
        }

        return None;
    }

    fn get_material(&self) -> Material {
        Material::demo()
    }
}
