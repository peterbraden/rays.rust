use std::path::Path;
use tobj;
use std::f64;
use shapes::geometry::Geometry;
use na::{Vector3};
use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use shapes::triangle::Triangle;
use std::vec::Vec;
use std::sync::Arc;


#[derive(Clone)]
pub struct Mesh {

    // TODO use BVH
    triangles: Vec<Triangle>,

    bounds: BBox,
}

impl Mesh {
    pub fn from_obj(pth: String, scale: Vector3<f64>) -> Mesh {
        let obj = tobj::load_obj(&Path::new(&pth));
        assert!(obj.is_ok());
        let (models, _materials) = obj.unwrap();
        //println!("# of models: {}", models.len());
        //println!("# of materials: {}", materials.len());

        let mut triangles = Vec::new();
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let positions: Arc<Vec<Vector3<f64>>> = Arc::new(
                mesh.positions
                    .chunks(3)
                    .map(|i| Vector3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .map(|i| i.component_mul(&scale))
                    .collect()
            );
            let mut tris: Vec<Triangle> = mesh.indices.chunks(3).map(|i| {
                Triangle::new(
                    positions[i[0] as usize],
                    positions[i[1] as usize],
                    positions[i[2] as usize])
            }).collect();
            triangles.append(&mut tris);
        }

        //println!("# of triangles: {}", triangles.len());

        let bounds = Mesh::bounds_of(&triangles);

        return Mesh {
            triangles: triangles,
            bounds: bounds,
        };
    }

    fn bounds_of(triangles: &Vec<Triangle>) -> BBox {
        let mut bb = BBox::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.)
        );

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        return bb;
    }

    pub fn naive_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<RawIntersection> {
        let mut cdist = max;
        let mut closest = None;
        
        for o in &self.triangles {
            match o.intersects(r) {
                Some(x) => {
                    if x.dist < cdist && x.dist >= min {
                        cdist = x.dist;
                        closest = Some(x);
                    }
                },
                None => (),
            }
        }
        return closest;
    }
}

impl Geometry for Mesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.naive_intersection(r, f64::INFINITY, 0f64);
    }

    fn bounds(&self) -> BBox {
        return self.bounds;
    }

    fn primitives(&self) -> u64 {
        return self.triangles.len() as u64;
    }
}
