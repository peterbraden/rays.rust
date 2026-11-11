/// Meshes
///
/// For simplicity, we'll limit the scope to triangular meshes (rather than including arbitrary
/// polygonal meshes) and limit that scope to meshes that are manifold.
///
/// In order to use a mesh in a ray-tracing context, we will need to use an acceleration structure
/// to performantly intersect with the appropriate triangle. Because of this, at render time, the
/// mesh will need the repetition of vertices necessary to store the triangles in a bounding
/// volume hierarchy (see `crate::bvh`), which holds each triangle once.
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
use crate::bvh::Bvh;
use crate::na::geometry::Affine3;

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
    bvh: Bvh,
    /// In the BVH's leaf order.
    triangles: Vec<Triangle>,
    /// For sampling points on the surface: the running total of area up to and including each
    /// triangle.
    areas: Vec<f64>,
}

// A simple triangle collection mesh. 
impl Mesh {
    /// Load an OBJ or PLY file, scaling it by `scale`.
    pub fn from_file(pth: String, scale: Vector3<f64>) -> Mesh {
        let mut triangles = Vec::new();
        for m in load(&pth) {
            let positions: Vec<Vector3<f64>> = m.positions.iter().map(|p| p.component_mul(&scale)).collect();
            triangles.extend(m.triangles.iter().map(|&[a, b, c]| {
                Triangle::new(positions[a], positions[b], positions[c])
            }));
        }
        Mesh::new(triangles)
    }

    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        let (bvh, triangles) = Bvh::build(triangles, |t| t.bounds());
        let mut total = 0.;
        let areas = triangles.iter().map(|t| { total += t.surface_area(); total }).collect();
        Mesh { bvh, triangles, areas }
    }
}

impl Geometry for Mesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    // A triangle is hit at most once, so its one hit is the nearest beyond any `min`.
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.bvh.nearest(r, min, max, |i, _| self.triangles[i].intersects(r)).map(|(_, x)| x)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.bvh.any_hit(r, min, max, |i| self.triangles[i].intersects(r).is_some_and(|x| x.dist >= min && x.dist < max))
    }

    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        self.bvh.transformed_bounds(t)
    }

    fn surface_area(&self) -> f64 {
        self.areas.last().copied().unwrap_or(0.)
    }

    /// Choose a triangle with probability proportional to its area, by binary search on the
    /// running totals, then a uniform point on it (Osada et al. 2002, section 4.2; see
    /// `sample_triangle`).
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let target = u[0] * self.surface_area();
        let i = self.areas.partition_point(|total| *total <= target).min(self.areas.len().checked_sub(1)?);
        self.triangles[i].sample_surface([u[1], u[2], 0.])
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.triangles.len() as u64
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
    bvh: Bvh,
    /// In the BVH's leaf order.
    triangles: Vec<SmoothTriangle>,
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
                SmoothTriangle::new(positions[a], positions[b], positions[c], na, nb, nc)
            }));
        }

        println!("# of triangles: {}", triangles.len());

        let (bvh, triangles) = Bvh::build(triangles, |t| t.bounds());
        SmoothMesh { bvh, triangles }
    }
}

impl Geometry for SmoothMesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    // A triangle is hit at most once, so its one hit is the nearest beyond any `min`.
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.bvh.nearest(r, min, max, |i, _| self.triangles[i].intersects(r)).map(|(_, x)| x)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.bvh.any_hit(r, min, max, |i| self.triangles[i].intersects(r).is_some_and(|x| x.dist >= min && x.dist < max))
    }

    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        self.bvh.transformed_bounds(t)
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.triangles.len() as u64
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

    /// A unit sphere as `n` x `2n` latitude-longitude quads.
    fn tessellated_sphere(n: usize) -> (Vec<Vector3<f64>>, Mesh) {
        let p = |i: usize, j: usize| {
            let (th, ph) = (std::f64::consts::PI * i as f64 / n as f64, std::f64::consts::PI * j as f64 / n as f64);
            Vector3::new(th.sin() * ph.cos(), th.cos(), th.sin() * ph.sin())
        };
        let mut tris = vec![];
        let mut verts = vec![];
        for i in 0 .. n {
            for j in 0 .. 2 * n {
                verts.push(p(i, j));
                tris.push(Triangle::new(p(i, j), p(i + 1, j), p(i, j + 1)));
                tris.push(Triangle::new(p(i, j + 1), p(i + 1, j), p(i + 1, j + 1)));
            }
        }
        verts.push(p(n, 0));
        (verts, Mesh::new(tris))
    }

    #[test]
    fn test_transformed_bounds_are_tight_and_conservative() {
        use crate::na::geometry::Rotation3;
        let (verts, mesh) = tessellated_sphere(24);
        let t = Affine3::from_matrix_unchecked(Rotation3::from_euler_angles(0.6, 0.7, 0.8).to_homogeneous());
        let tight = mesh.transformed_bounds(&t);
        let corners = mesh.bounds().transform(&na::convert(t));
        // A rotated unit sphere still fits in [-1, 1]^3; the rotated box of corners doesn't.
        assert!(tight.size().max() < 2.5, "{}", tight);
        assert!(corners.size().max() > 2.8, "{}", corners);
        for v in verts {
            assert!(tight.contains_point(&t.transform_point(&v.into()).coords), "{} outside {}", v, tight);
        }
    }

    #[test]
    fn test_ranges_agree_with_brute_force() {
        use rand::{Rng, SeedableRng};
        let (_, mesh) = tessellated_sphere(12);
        let mut rng = rand::rngs::StdRng::seed_from_u64(5);
        for _ in 0 .. 1000 {
            let r = Ray {
                ro: Vector3::new(rng.gen_range(-3., 3.), rng.gen_range(-3., 3.), rng.gen_range(-3., 3.)),
                rd: Vector3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.)),
            };
            let (min, max) = (rng.gen_range(0., 2.), rng.gen_range(2., 6.));
            let expected = mesh.triangles.iter().filter_map(|t| t.intersects(&r))
                .map(|x| x.dist).filter(|&d| d >= min && d < max)
                .min_by(|a, b| a.total_cmp(b));
            assert_eq!(mesh.intersects_range(&r, min, max).map(|x| x.dist), expected);
            assert_eq!(mesh.occludes(&r, min, max), expected.is_some());
        }
    }
}
