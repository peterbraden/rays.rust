/// Meshes
///
/// For simplicity, we'll limit the scope to triangular meshes (rather than including arbitrary
/// polygonal meshes) and limit that scope to meshes that are manifold.
///
/// In order to use a mesh in a ray-tracing context, we will need to use an acceleration structure
/// to performantly intersect with the appropriate triangle. Because of this, at render time, the
/// mesh will need the repetition of vertices necessary to store the triangles in an Octree.
///
/// (POSSIBLE ENHANCEMENT: Is there a way to use the octree with indices to dynamically reconstruct
/// triangles?)
///
/// Most other Mesh datastructures are optimisations to reduce the memory requirements by removing
/// repetition of positional data using indexes of vertices. This makes a lot of sense when you
/// care about a lot of serialisation (for example streaming to a GPU) but in our context of
/// in-memory tracing, doesn't provide a lot of advantages.
///
/// ## References:
/// 1. Fundamentals of Computer Graphics (4th Ed.) - Marschner, Steve


use std::path::Path;
use tobj;
use std::f64;
use crate::shapes::geometry::Geometry;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::triangle::{Triangle, SmoothTriangle};
use std::vec::Vec;
use std::sync::Arc;
use crate::octree::Octree;

pub struct Mesh {
    triangles: Octree<Triangle>,
    bounds: BBox,
    triangle_count: usize,
}

// A simple triangle collection mesh. 
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
        Mesh {
            triangles: tree,
            bounds,
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


pub struct SmoothMesh {
    triangles: Octree<SmoothTriangle>,
    bounds: BBox,
    triangle_count: usize,
}

impl SmoothMesh {
    pub fn from_obj(pth: String, scale: Vector3<f64>) -> SmoothMesh {
        let obj = tobj::load_obj(&Path::new(&pth));
        assert!(obj.is_ok());
        let (models, _materials) = obj.unwrap();
        //println!("# of models: {}", models.len());
        //println!("# of materials: {}", materials.len());

        let mut triangles = Vec::new();
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            if mesh.normals.is_empty() {
                print!("!! [ Warning in STL parse ] Normals are required for a smooth mesh - skipping {}", m.name);
                continue;
            }
            //println!("> model {:?}", mesh.normals.len());

            let positions: Arc<Vec<Vector3<f64>>> = Arc::new(
                mesh.positions
                    .chunks(3)
                    .map(|i| Vector3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .map(|i| i.component_mul(&scale))
                    .collect()
            );
            let normals: Arc<Vec<Vector3<f64>>> = Arc::new(
                mesh.normals
                    .chunks(3)
                    .map(|i| Vector3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .collect()
            );
            let mut tris: Vec<Arc<SmoothTriangle>> = mesh.indices.chunks(3).map(|i| {
                let _n = positions[i[0] as usize];
                Arc::new(
                    SmoothTriangle::new(
                        positions[i[0] as usize],
                        positions[i[1] as usize],
                        positions[i[2] as usize],
                        normals[i[0] as usize],
                        normals[i[1] as usize],
                        normals[i[2] as usize],
                    )
                    )
            }).collect();
            triangles.append(&mut tris);
        }

        println!("# of triangles: {}", triangles.len());

        let bounds = SmoothMesh::bounds_of(&triangles);
        let tree = Octree::new(8, bounds, &triangles); 
        SmoothMesh {
            triangles: tree,
            bounds,
            triangle_count: triangles.len()
        }
    }

    fn bounds_of(triangles: &Vec<Arc<SmoothTriangle>>) -> BBox {
        let mut bb = BBox::min();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        return bb;
    }
}

impl Geometry for SmoothMesh {
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

