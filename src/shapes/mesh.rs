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

/// A mesh as stored in a file: vertex positions, optionally a normal per vertex, and triangles
/// indexing them.
#[derive(Debug)]
pub struct IndexedMesh {
    pub positions: Vec<Vector3<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub triangles: Vec<[usize; 3]>,
}

/// Load the models in an OBJ file as triangles, with positions and normals sharing one index.
/// Lines and points are dropped. A missing or broken material library is ignored, as the
/// materials are unused.
fn load_obj(pth: &str) -> Vec<IndexedMesh> {
    let options = tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_points: true,
        ignore_lines: true,
    };
    let (models, _materials) = tobj::load_obj(Path::new(pth), &options)
        .unwrap_or_else(|e| panic!("Failed to load mesh {}: {}", pth, e));
    models.iter().map(|m| {
        let positions = to_vectors(&m.mesh.positions);
        let mut normals = to_vectors(&m.mesh.normals);
        if normals.len() != positions.len() {
            if !normals.is_empty() {
                println!("!! [ Warning in OBJ parse ] {} lacks a normal for every vertex - computing them", m.name);
            }
            normals.clear();
        }
        let triangles = m.mesh.indices.chunks(3).map(|i| [i[0] as usize, i[1] as usize, i[2] as usize]).collect();
        IndexedMesh { positions, normals, triangles }
    }).collect()
}

/// Load a mesh file: OBJ (`.obj`) or PLY (`.ply`, see `crate::shapes::ply`).
fn load(pth: &str) -> Vec<IndexedMesh> {
    if pth.to_lowercase().ends_with(".ply") {
        vec![crate::shapes::ply::read(pth).unwrap_or_else(|e| panic!("Failed to load mesh {}", e))]
    } else {
        load_obj(pth)
    }
}

fn to_vectors(v: &[f32]) -> Vec<Vector3<f64>> {
    v.chunks(3).map(|i| Vector3::new(i[0] as f64, i[1] as f64, i[2] as f64)).collect()
}

pub struct Mesh {
    triangles: Octree<Triangle>,
    bounds: BBox,
    triangle_count: usize,
    /// For sampling points on the surface: each triangle with the running total of area up to
    /// and including it.
    areas: Vec<(Arc<Triangle>, f64)>,
}

// A simple triangle collection mesh. 
impl Mesh {
    /// Load an OBJ or PLY file, scaling it by `scale`.
    pub fn from_file(pth: String, scale: Vector3<f64>) -> Mesh {
        let mut triangles = Vec::new();
        for m in load(&pth) {
            let positions: Vec<Vector3<f64>> = m.positions.iter().map(|p| p.component_mul(&scale)).collect();
            triangles.extend(m.triangles.iter().map(|&[a, b, c]| {
                Arc::new(Triangle::new(positions[a], positions[b], positions[c]))
            }));
        }

        //println!("# of triangles: {}", triangles.len());

        let bounds = Mesh::bounds_of(&triangles);
        let tree = Octree::new(8, bounds, &triangles); 
        let mut total = 0.;
        let areas = triangles.iter().map(|t| {
            total += t.surface_area();
            (t.clone(), total)
        }).collect();
        Mesh {
            triangles: tree,
            bounds,
            triangle_count: triangles.len(),
            areas,
        }
    }

    fn bounds_of(triangles: &Vec<Arc<Triangle>>) -> BBox {
        let mut bb = BBox::empty();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        bb
    }
}

impl Geometry for Mesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.triangles.raw_intersection(r, f64::INFINITY, 0f64)
    }

    fn surface_area(&self) -> f64 {
        self.areas.last().map_or(0., |(_, total)| *total)
    }

    /// Choose a triangle with probability proportional to its area, by binary search on the
    /// running totals, then a uniform point on it (Osada et al. 2002, section 4.2; see
    /// `sample_triangle`).
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let target = u[0] * self.surface_area();
        let i = self.areas.partition_point(|(_, total)| *total <= target).min(self.areas.len().checked_sub(1)?);
        self.areas[i].0.sample_surface([u[1], u[2], 0.])
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.triangle_count as u64
    }
}


/// Normals for each vertex of a mesh that doesn't provide them, as the area-weighted average of
/// the normals of the triangles sharing the vertex.
///
/// Averaging facet normals at the vertices, then interpolating across each face, is Henri
/// Gouraud's, from "Continuous Shading of Curved Surfaces", IEEE Transactions on Computers
/// C-20(6), 623-629, 1971, doi:10.1109/T-C.1971.223313; Phong interpolates the normals rather
/// than the shading, as `SmoothTriangle` does. Weighting by area is the usual choice and the
/// cheapest: the cross product E1 x E2 of a triangle's edges is its normal scaled by twice its
/// area, so summing unnormalised cross products weights each face by its area for free. Small
/// slivers then barely tilt the result, which matters for scanned meshes with uneven
/// triangulation. Other weights are compared by Grit Thürrner and Charles A. Wüthrich, "Computing
/// Vertex Normals from Polygonal Facets", Journal of Graphics Tools 3(1), 43-46, 1998 (by the
/// angle each face makes at the vertex), and Nelson Max, "Weights for Computing Vertex Normals
/// from Facet Normals", Journal of Graphics Tools 4(2), 1-6, 1999,
/// doi:10.1080/10867651.1999.10487501, whose weights give exact normals for a polyhedron
/// inscribed in a sphere.
///
/// A vertex whose faces cancel out (or belong only to degenerate triangles) gets a zero vector;
/// callers fall back to the face normal there.
pub fn vertex_normals(positions: &[Vector3<f64>], triangles: &[[usize; 3]]) -> Vec<Vector3<f64>> {
    let mut sums = vec![Vector3::zeros(); positions.len()];
    for &[a, b, c] in triangles {
        let n = (positions[b] - positions[a]).cross(&(positions[c] - positions[a]));
        sums[a] += n;
        sums[b] += n;
        sums[c] += n;
    }
    sums.into_iter().map(|n| n.try_normalize(1e-300).unwrap_or_else(Vector3::zeros)).collect()
}

pub struct SmoothMesh {
    triangles: Octree<SmoothTriangle>,
    bounds: BBox,
    triangle_count: usize,
}

impl SmoothMesh {
    /// Load an OBJ or PLY file, scaling it by `scale`. Meshes without a normal for every vertex
    /// get them from `vertex_normals`.
    pub fn from_file(pth: String, scale: Vector3<f64>) -> SmoothMesh {
        let mut triangles = Vec::new();
        for m in load(&pth) {
            let positions: Vec<Vector3<f64>> = m.positions.iter().map(|p| p.component_mul(&scale)).collect();
            let normals: Vec<Vector3<f64>> = if m.normals.is_empty() {
                // Computed from the scaled positions, so they need no further transform.
                vertex_normals(&positions, &m.triangles)
            } else {
                // Normals transform by the inverse of the scale.
                m.normals.iter().map(|n| n.component_div(&scale).normalize()).collect()
            };
            triangles.extend(m.triangles.iter().map(|&[a, b, c]| {
                let face = Triangle::new(positions[a], positions[b], positions[c]).normal;
                let or_face = |n: Vector3<f64>| if n == Vector3::zeros() { face } else { n };
                let (na, nb, nc) = (or_face(normals[a]), or_face(normals[b]), or_face(normals[c]));
                Arc::new(SmoothTriangle::new(positions[a], positions[b], positions[c], na, nb, nc))
            }));
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
        let mut bb = BBox::empty();

        for t in triangles {
            bb = bb.union(&t.bounds());
        }

        bb
    }
}

impl Geometry for SmoothMesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.triangles.raw_intersection(r, f64::INFINITY, 0f64)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.triangle_count as u64
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_obj(name: &str, contents: &str) -> String {
        let dir = std::env::temp_dir().join(format!("rays-mesh-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join(name);
        fs::write(&p, contents).unwrap();
        p.to_str().unwrap().to_string()
    }

    const QUAD: &str = "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 1 1 0\n";
    const ONE: Vector3<f64> = Vector3::new(1., 1., 1.);

    #[test]
    fn test_line_elements_are_ignored() {
        let p = write_obj("lines.obj", &format!("{}f 1 2 3\nl 1 4\nf 2 4 3\n", QUAD));
        assert_eq!(Mesh::from_file(p, ONE).primitives(), 2);
    }

    #[test]
    fn test_missing_material_library_is_ignored() {
        let p = write_obj("nomtl.obj", &format!("mtllib missing.mtl\n{}f 1 2 3\n", QUAD));
        assert_eq!(Mesh::from_file(p, ONE).primitives(), 1);
    }

    #[test]
    fn test_ply_files_load_by_extension() {
        let p = write_obj("quad.PLY", "ply\nformat ascii 1.0\nelement vertex 4\nproperty float x\n\
            property float y\nproperty float z\nelement face 1\nproperty list uchar int vertex_indices\n\
            end_header\n0 0 0\n1 0 0\n1 1 0\n0 1 0\n4 0 1 2 3\n");
        assert_eq!(Mesh::from_file(p.clone(), ONE).primitives(), 2);
        assert_eq!(SmoothMesh::from_file(p, ONE).primitives(), 2);
    }

    #[test]
    fn test_vertex_normals_are_area_weighted() {
        // An octahedron: by symmetry each vertex normal points away from the centre.
        let p: Vec<Vector3<f64>> = [[1., 0., 0.], [-1., 0., 0.], [0., 1., 0.], [0., -1., 0.], [0., 0., 1.], [0., 0., -1.]]
            .iter().map(|v| Vector3::new(v[0], v[1], v[2])).collect();
        let faces = [[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
        for (n, v) in vertex_normals(&p, &faces).iter().zip(&p) {
            assert!((n - v).norm() < 1e-12, "{} at {}", n, v);
        }

        // A large triangle facing +z and a small one facing +x share vertex 0: the large one
        // dominates, in proportion to area (4 against 0.5).
        let p = vec![Vector3::zeros(), Vector3::new(2., 0., 0.), Vector3::new(0., 4., 0.), Vector3::new(0., 1., 0.), Vector3::new(0., 0., 1.)];
        let n = vertex_normals(&p, &[[0, 1, 2], [0, 3, 4]]);
        assert!((n[0] - Vector3::new(0.5, 0., 4.).normalize()).norm() < 1e-12, "{}", n[0]);
        // A vertex of only the large triangle takes its normal exactly.
        assert_eq!(n[1], Vector3::new(0., 0., 1.));
        // A vertex used by no triangle gets zero, for the face-normal fallback.
        assert_eq!(vertex_normals(&p, &[])[0], Vector3::zeros());
    }

    #[test]
    fn test_smooth_mesh_without_normals_is_smooth() {
        // Two faces folded 90 degrees along the x axis; a hit beside the shared edge gets a
        // normal between the two face normals, not either face's own.
        let p = write_obj("fold.obj", "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\nf 1 2 3\nf 1 4 2\n");
        let m = SmoothMesh::from_file(p, ONE);
        let r = Ray { ro: Vector3::new(0.3, 0.01, 5.), rd: Vector3::new(0., 0., -1.) };
        let n = m.intersects(&r).expect("hit").normal;
        assert!(n.y > 0.1 && n.z > 0.1, "{}", n);
    }

    #[test]
    fn test_smooth_mesh_with_partial_normals() {
        let p = write_obj("partial.obj", &format!("{}vn 0 0 1\nf 1//1 2//1 3//1\nf 2 4 3\n", QUAD));
        assert_eq!(SmoothMesh::from_file(p, ONE).primitives(), 2);
    }

    #[test]
    fn test_smooth_normals_follow_scale() {
        // A plane tilted 45 degrees about z, stretched 3x in y: its true normal is
        // (1, 1/3, 0) normalised.
        let p = write_obj("tilted.obj", "v 0 0 -1\nv 0 0 1\nv -1 1 0\nvn 1 1 0\nf 1//1 2//1 3//1\n");
        let m = SmoothMesh::from_file(p, Vector3::new(1., 3., 1.));
        let r = Ray { ro: Vector3::new(5., 1., 0.), rd: Vector3::new(-1., 0., 0.) };
        let n = m.intersects(&r).expect("hit").normal;
        assert!((n - Vector3::new(1., 1. / 3., 0.).normalize()).norm() < 1e-6, "{}", n);
    }
}
