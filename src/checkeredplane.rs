use sceneobject::SceneObject;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;
use material::Material;

pub struct CheckeredPlane {
    pub y: f64
}

impl SceneObject for CheckeredPlane {

    fn intersects(&self, r: &Ray) -> Option<Intersection> {
        let rdn = r.rd.normalize();
        let norm = Vec3::new(0., 1., 0.);
        let pos = Vec3::new(0., self.y, 0.);
        let dist = norm.dot(&(pos - r.ro)) / norm.dot(&rdn);
        if dist > 0. {
            return Some(
                Intersection {
                    dist: dist, 
                    point: r.ro + (r.rd * dist),
                    normal: norm,
                    object: self
                })
        }

        None
    }

    fn get_material(&self, point: Vec3<f64>) -> Material {
        Material::demo()
    }
}


