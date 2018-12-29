use shapes::geometry::Geometry;
use na::{Vector3};
use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub normal: Vector3<f64>,
}

fn panic_if_nan(v: Vector3<f64>) {
    if v.y.is_nan() {
        panic!("NaN in vector");
    }
}

impl Triangle {

    pub fn new(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>) -> Triangle{
        let v0v1 = v1 - v0; 
        let v0v2 = v2 - v0; 

        //let area2 = normal.length(); // Before norm
        let normal = v0v1.cross(&v0v2).normalize();  
        //panic_if_nan(normal);
        return Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            normal: normal,
        }
    }
    pub fn new_with_normal(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, normal: Vector3<f64>) -> Triangle{
        return Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            normal: normal,
        }
    }

    pub fn translate_vec3(&self, v: Vector3<f64>) -> Triangle {
        return Triangle {
            v0: self.v0 - v,
            v1: self.v1 - v,
            v2: self.v2 - v,
            normal: self.normal,
        }
    }
}

const SMALL: f64 = 0.0000001;

impl Geometry for Triangle {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {

        let v0v1 = self.v1 - self.v0; 
        let v0v2 = self.v2 - self.v0; 
        let pvec = r.rd.cross(&v0v2); 
        panic_if_nan(pvec);
        let det = v0v1.dot(&pvec); 
                     
        // ray and triangle are parallel if det is close to 0
        if det.abs() < SMALL { return None }; 

        let inv_det = 1. / det; 
        let tvec = r.ro - self.v0; 
        let u = tvec.dot(&pvec) * inv_det; 

        if u < 0. || u > 1. { return None }; 

        let qvec = tvec.cross(&v0v1); 
        panic_if_nan(qvec);
        let v = r.rd.dot(&qvec) * inv_det; 

        if v < 0. || u + v > 1. { return None }; 
                
        let dist = v0v2.dot(&qvec) * inv_det; 
                     
        if dist > 0. {
            let point = r.ro + (r.rd.normalize() * dist);
            return Some(RawIntersection {
                dist: dist, 
                point: point,
                normal: self.normal 
            })
        }
        return None;
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(
                self.v0.x.min(self.v1.x).min(self.v2.x),
                self.v0.y.min(self.v1.y).min(self.v2.y),
                self.v0.z.min(self.v1.z).min(self.v2.z),
            ),
            Vector3::new(
                self.v0.x.max(self.v1.x).max(self.v2.x),
                self.v0.y.max(self.v1.y).max(self.v2.y),
                self.v0.z.max(self.v1.z).max(self.v2.z),
            )
        )
    }
}
