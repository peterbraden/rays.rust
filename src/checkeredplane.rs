use sceneobject::SceneObject;
use na::{Vector3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;
use material::Material;
use bbox::BBox;

pub struct CheckeredPlane {
    pub y: f64
}

impl SceneObject for CheckeredPlane {

    fn intersects(&self, r: &Ray) -> Option<Intersection> {
        let rdn = r.rd.normalize();
        let mut norm = Vector3::new(0., 1., 0.);
        let denom = norm.dot(&rdn);

        if denom.abs() > 0. {
            let dist = -(norm.dot(&r.ro) - self.y) / denom;
            if dist > 0. {
                if denom > 0. {
                    norm = -norm
                }

                return Some(
                    Intersection {
                        dist: dist, 
                        point: r.ro + (rdn * dist),
                        normal: norm,
                        object: self
                    })
            }
        }

        None
    }

    fn get_material(&self, pt: Vector3<f64>) -> Material {
        Material::checker_demo(pt, 2., 2.)
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(0.,0.,0.),
            Vector3::new(0.,0.,0.),
          )
    }
}


