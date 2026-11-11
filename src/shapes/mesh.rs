/// Meshes
///
/// Triangle meshes, loaded from OBJ or PLY files. Polygons are split into triangles on loading.
///
/// A mesh is stored as files store it: one array of vertex positions (and, for smooth shading,
/// normals), and each triangle as three indices into it. Most vertices are shared by about six
/// triangles, so this holds each position once rather than six times, and a triangle costs 12
/// bytes of indices instead of a separately allocated, reference-counted `Triangle` of four
/// vectors (issue #26: the Stanford dragon took about 475 bytes per triangle that way). The
/// triangle's vertices are fetched when a ray is tested against it.
///
/// The triangles are ordered as the leaves of a bounding volume hierarchy (`crate::bvh::Tree`),
/// so the triangles in each leaf are contiguous in memory.
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
use crate::shapes::triangle::{intersects_dist, sample_triangle, SmoothTriangle};
use crate::bvh::Tree;
use std::vec::Vec;

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

/// Triangles as indices into shared vertex arrays, ordered as the leaves of their hierarchy.
struct Indexed {
    positions: Vec<Vector3<f64>>,
    /// A normal per vertex for smooth shading; empty for flat. A zero normal (a vertex whose
    /// faces cancel) falls back to the face's own.
    normals: Vec<Vector3<f64>>,
    triangles: Vec<[u32; 3]>,
    tree: Tree,
    /// The position of each file triangle in `triangles`, in file order.
    file_order: Vec<u32>,
}

impl Indexed {
    /// Load an OBJ or PLY file, scaled by `scale`; with `smooth`, keep or compute vertex normals.
    fn load(pth: &str, scale: Vector3<f64>, smooth: bool) -> Indexed {
        let mut positions: Vec<Vector3<f64>> = Vec::new();
        let mut normals = Vec::new();
        let mut file_triangles: Vec<[u32; 3]> = Vec::new();
        for m in load(pth) {
            let offset = positions.len();
            let scaled: Vec<Vector3<f64>> = m.positions.iter().map(|p| p.component_mul(&scale)).collect();
            if smooth {
                if m.normals.is_empty() {
                    // Computed from the scaled positions, so they need no further transform.
                    normals.extend(vertex_normals(&scaled, &m.triangles));
                } else {
                    // Normals transform by the inverse of the scale.
                    normals.extend(m.normals.iter().map(|n| n.component_div(&scale).normalize()));
                }
            }
            positions.extend(scaled);
            file_triangles.extend(m.triangles.iter().map(|t| t.map(|i| (i + offset) as u32)));
        }

        let corners = |t: &[u32; 3]| t.map(|i| positions[i as usize]);
        let bounds: Vec<BBox> = file_triangles.iter().map(|t| {
            let [a, b, c] = corners(t);
            BBox::empty().union_point(&a).union_point(&b).union_point(&c)
        }).collect();
        let (tree, order) = Tree::build(&bounds);
        let triangles: Vec<[u32; 3]> = order.iter().map(|&i| file_triangles[i]).collect();
        let mut file_order = vec![0u32; order.len()];
        for (position, &i) in order.iter().enumerate() {
            file_order[i] = position as u32;
        }
        Indexed { positions, normals, triangles, tree, file_order }
    }

    fn corners(&self, i: usize) -> [Vector3<f64>; 3] {
        self.triangles[i].map(|v| self.positions[v as usize])
    }

    /// The face normal, as `Triangle::new` computes it.
    fn face_normal(&self, i: usize) -> Vector3<f64> {
        let [a, b, c] = self.corners(i);
        (b - a).cross(&(c - a)).normalize()
    }

    fn hit(&self, i: usize, r: &Ray) -> Option<RawIntersection> {
        let [a, b, c] = self.corners(i);
        if self.normals.is_empty() {
            intersects_dist(a, b, c, r).map(|x| RawIntersection { dist: x.dist, point: x.point, normal: self.face_normal(i) })
        } else {
            let n = self.triangles[i].map(|v| self.normals[v as usize]);
            let face = || self.face_normal(i);
            let or_face = |n: Vector3<f64>| if n == Vector3::zeros() { face() } else { n };
            SmoothTriangle::new(a, b, c, or_face(n[0]), or_face(n[1]), or_face(n[2])).intersects(r)
        }
    }

    fn nearest(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.tree.nearest(r, min, f64::INFINITY, |i| self.hit(i, r)).map(|(_, x)| x)
    }
}

pub struct Mesh {
    mesh: Indexed,
    /// For sampling points on the surface: the running total of triangle area, in file order.
    areas: Vec<f64>,
}

impl Mesh {
    /// Load an OBJ or PLY file, scaling it by `scale`.
    pub fn from_file(pth: String, scale: Vector3<f64>) -> Mesh {
        let mesh = Indexed::load(&pth, scale, false);
        let mut total = 0.;
        let areas = mesh.file_order.iter().map(|&i| {
            let [a, b, c] = mesh.corners(i as usize);
            total += 0.5 * (b - a).cross(&(c - a)).norm();
            total
        }).collect();
        Mesh { mesh, areas }
    }
}

impl Geometry for Mesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.mesh.nearest(r, 0.)
    }

    /// The nearest triangle at least `min` along the ray, found in one traversal.
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.mesh.nearest(r, min)
    }

    fn surface_area(&self) -> f64 {
        self.areas.last().copied().unwrap_or(0.)
    }

    /// Choose a triangle with probability proportional to its area, by binary search on the
    /// running totals, then a uniform point on it (Osada et al. 2002, section 4.2; see
    /// `sample_triangle`).
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let target = u[0] * self.surface_area();
        let f = self.areas.partition_point(|total| *total <= target).min(self.areas.len().checked_sub(1)?);
        let i = self.mesh.file_order[f] as usize;
        let [a, b, c] = self.mesh.corners(i);
        Some((sample_triangle(a, b, c, u[1], u[2]), self.mesh.face_normal(i)))
    }

    fn bounds(&self) -> BBox {
        self.mesh.tree.bounds()
    }

    fn primitives(&self) -> u64 {
        self.mesh.triangles.len() as u64
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
    mesh: Indexed,
}

impl SmoothMesh {
    /// Load an OBJ or PLY file, scaling it by `scale`. Meshes without a normal for every vertex
    /// get them from `vertex_normals`.
    pub fn from_file(pth: String, scale: Vector3<f64>) -> SmoothMesh {
        SmoothMesh { mesh: Indexed::load(&pth, scale, true) }
    }
}

impl Geometry for SmoothMesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.mesh.nearest(r, 0.)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.mesh.nearest(r, min)
    }

    fn bounds(&self) -> BBox {
        self.mesh.tree.bounds()
    }

    fn primitives(&self) -> u64 {
        self.mesh.triangles.len() as u64
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
