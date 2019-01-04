use std::path::Path;
use tobj;
use std::f64;
use shapes::geometry::Geometry;
use na::{Vector3, Rotation3};
use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use shapes::triangle::Triangle;
use std::vec::Vec;
use std::sync::Arc;
use octree::Octree;
use std::iter::Iterator;


pub struct Mesh {
    triangles: Octree<Triangle>,
    bounds: BBox,
    triangle_count: usize,
}

impl Mesh {
    pub fn from_obj(pth: String, scale: Vector3<f64>, translate: Vector3<f64>, rotate: Vector3<f64>) -> Mesh {
        let obj = tobj::load_obj(&Path::new(&pth));
        assert!(obj.is_ok());
        let (models, _materials) = obj.unwrap();
        //println!("# of models: {}", models.len());
        //println!("# of materials: {}", materials.len());
        let rotation = Rotation3::new(rotate);

        let mut triangles = Vec::new();
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let positions: Arc<Vec<Vector3<f64>>> = Arc::new(
                mesh.positions
                    .chunks(3)
                    .map(|i| Vector3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .map(|i| i.component_mul(&scale)) // scale
                    .map(|i| rotate * i) // rotate
                    .map(|i| i + translate) // translate)
                    .collect()
            );
            let mut tris: Vec<Arc<Triangle>> = mesh.indices.chunks(3).map(|i| {
                Arc::new(
                    Triangle::new(
                        positions[i[0] as usize],
                        positions[i[1] as usize],
                        positions[i[2] as usize])
                    )
            }).collect();
            triangles.append(&mut tris);
        }

        //println!("# of triangles: {}", triangles.len());

        let bounds = Mesh::bounds_of(&triangles);
        let tree = Octree::new(8, bounds, &triangles); 
        return Mesh {
            triangles: tree,
            bounds: bounds,
            triangle_count: triangles.len()
        }
    }

    fn bounds_of(triangles: &Vec<Arc<Triangle>>) -> BBox {
        let mut bb = BBox::min();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        return bb;
    }
}

impl Geometry for Mesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.triangles.raw_intersection(r, f64::INFINITY, 0f64);
    }

    fn bounds(&self) -> BBox {
        return self.bounds;
    }

    fn primitives(&self) -> u64 {
        return self.triangle_count as u64;
    }
}


/// Mesh

type TriangleInd = (usize, usize, usize);

pub struct Mesh2 {
    vertices: Vec<Vector3<f64>>,
    vertice_normals: Vec<Vector3<f64>>,
    edges: Vec<TriangleInd>, // Indices into vertices
    bounds: BBox,
    triangle_count: usize,
}

/*
impl Mesh2 {
    
    pub fn triangles(&self) -> Iterator<Triangle> {
    
    }
}
*/
