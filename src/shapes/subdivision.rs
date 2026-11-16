//! # Loop subdivision surfaces
//!
//! A coarse triangle mesh describes a smooth surface: the limit of refining it again and again,
//! splitting every triangle into four and moving every vertex towards a weighted average of its
//! neighbours. Charles Loop, "Smooth Subdivision Surfaces Based on Triangles", MS thesis,
//! University of Utah, 1987; Pharr, Jakob and Humphreys, "Physically Based Rendering", 3rd ed.,
//! section 3.8 "Subdivision Surfaces" (an online chapter in the 4th; cited below as PBR), which
//! this follows.
//!
//! Each level:
//!
//! - every edge gets a new ("odd") vertex: 3/8 of each end plus 1/8 of each of the two
//!   vertices opposite it, or the edge's midpoint on the boundary;
//! - every old ("even") vertex moves to (1 - kβ) of itself plus β of each of its k neighbours,
//!   with β = 3/16 for k = 3 and 3/(8k) otherwise (Warren's simplification of Loop's weights);
//!   on the boundary, 3/4 of itself and 1/8 of each neighbour along it, so a boundary refines as
//!   a curve of its own;
//! - every triangle becomes four.
//!
//! After the last level, every vertex is moved to the point the refinement converges to, its
//! limit position: (1 - kγ) of itself plus γ of each neighbour, γ = 1 / (k + 3 / (8β)), or on
//! the boundary 3/5 of itself and 1/5 of each neighbour along it (PBR's `loopGamma` and
//! `weightBoundary`). Normals are interpolated from the refined mesh's faces (`mesh::vertex_normals`)
//! rather than from the limit surface's tangents, as PBR computes them: a departure that matters
//! little after a few levels. Edges shared by more than two triangles are treated as boundaries.
use crate::na::Vector3;
use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
use std::collections::HashMap;

fn edge(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}

/// The vertices opposite each edge, by the triangles that share it.
fn opposites(triangles: &[[usize; 3]]) -> HashMap<(usize, usize), Vec<usize>> {
    let mut m: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for t in triangles {
        for k in 0..3 {
            m.entry(edge(t[k], t[(k + 1) % 3])).or_default().push(t[(k + 2) % 3]);
        }
    }
    m
}

/// Each vertex's neighbours, and its neighbours along the boundary (empty if it is inside).
fn neighbours(n: usize, edges: &HashMap<(usize, usize), Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut ring = vec![Vec::new(); n];
    let mut boundary = vec![Vec::new(); n];
    for (&(a, b), opp) in edges {
        ring[a].push(b);
        ring[b].push(a);
        if opp.len() != 2 {
            boundary[a].push(b);
            boundary[b].push(a);
        }
    }
    (ring, boundary)
}

fn beta(k: usize) -> f64 {
    if k == 3 { 3. / 16. } else { 3. / (8. * k as f64) }
}

/// One level of Loop subdivision.
pub fn refine(positions: &[Vector3<f64>], triangles: &[[usize; 3]]) -> (Vec<Vector3<f64>>, Vec<[usize; 3]>) {
    let edges = opposites(triangles);
    let (ring, boundary) = neighbours(positions.len(), &edges);
    // Even vertices.
    let mut out: Vec<Vector3<f64>> = (0..positions.len()).map(|v| {
        let p = positions[v];
        if boundary[v].len() == 2 {
            p * 0.75 + (positions[boundary[v][0]] + positions[boundary[v][1]]) * 0.125
        } else if !boundary[v].is_empty() {
            // A corner, or a vertex on a non-manifold edge: keep it.
            p
        } else {
            let k = ring[v].len();
            if k == 0 { return p; }
            let b = beta(k);
            p * (1. - k as f64 * b) + ring[v].iter().map(|&n| positions[n]).sum::<Vector3<f64>>() * b
        }
    }).collect();
    // Odd vertices.
    let mut odd: HashMap<(usize, usize), usize> = HashMap::new();
    let mut keys: Vec<&(usize, usize)> = edges.keys().collect();
    keys.sort();
    for &(a, b) in keys {
        let opp = &edges[&(a, b)];
        let p = if opp.len() == 2 {
            (positions[a] + positions[b]) * 0.375 + (positions[opp[0]] + positions[opp[1]]) * 0.125
        } else {
            (positions[a] + positions[b]) * 0.5
        };
        odd.insert((a, b), out.len());
        out.push(p);
    }
    let tris = triangles.iter().flat_map(|&[a, b, c]| {
        let (ab, bc, ca) = (odd[&edge(a, b)], odd[&edge(b, c)], odd[&edge(c, a)]);
        [[a, ab, ca], [b, bc, ab], [c, ca, bc], [ab, bc, ca]]
    }).collect();
    (out, tris)
}

/// Each vertex moved to its limit position.
pub fn limit(positions: &[Vector3<f64>], triangles: &[[usize; 3]]) -> Vec<Vector3<f64>> {
    let edges = opposites(triangles);
    let (ring, boundary) = neighbours(positions.len(), &edges);
    (0..positions.len()).map(|v| {
        let p = positions[v];
        if boundary[v].len() == 2 {
            p * 0.6 + (positions[boundary[v][0]] + positions[boundary[v][1]]) * 0.2
        } else if !boundary[v].is_empty() {
            p
        } else {
            let k = ring[v].len();
            if k == 0 { return p; }
            let gamma = 1. / (k as f64 + 3. / (8. * beta(k)));
            p * (1. - k as f64 * gamma) + ring[v].iter().map(|&n| positions[n]).sum::<Vector3<f64>>() * gamma
        }
    }).collect()
}

/// The mesh refined `levels` times and pushed to its limit, as a smooth mesh.
pub fn subdivide(positions: Vec<Vector3<f64>>, triangles: Vec<[usize; 3]>, levels: u32) -> SmoothMesh {
    let (mut p, mut t) = (positions, triangles);
    for _ in 0..levels {
        let (np, nt) = refine(&p, &t);
        p = np;
        t = nt;
    }
    let p = limit(&p, &t);
    SmoothMesh::from_indexed(&[IndexedMesh { positions: p, normals: vec![], triangles: t, texcoords: vec![] }])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn octahedron() -> (Vec<Vector3<f64>>, Vec<[usize; 3]>) {
        let p = vec![Vector3::x(), -Vector3::x(), Vector3::y(), -Vector3::y(), Vector3::z(), -Vector3::z()];
        let t = vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
        (p, t)
    }

    #[test]
    fn test_refinement_counts() {
        let (p, t) = octahedron();
        let (p1, t1) = refine(&p, &t);
        // V + E vertices, four times the faces.
        assert_eq!(p1.len(), 6 + 12);
        assert_eq!(t1.len(), 32);
    }

    /// The largest angle (degrees) between the normals of triangles sharing an edge.
    fn sharpest(p: &[Vector3<f64>], t: &[[usize; 3]]) -> f64 {
        let normal = |f: &[usize; 3]| (p[f[1]] - p[f[0]]).cross(&(p[f[2]] - p[f[0]])).normalize();
        let mut by_edge: HashMap<(usize, usize), Vec<Vector3<f64>>> = HashMap::new();
        for f in t {
            for k in 0..3 {
                by_edge.entry(edge(f[k], f[(k + 1) % 3])).or_default().push(normal(f));
            }
        }
        by_edge.values().filter(|n| n.len() == 2).map(|n| n[0].dot(&n[1]).clamp(-1., 1.).acos().to_degrees()).fold(0., f64::max)
    }

    /// On a mesh whose every vertex has valence 6 (a triangulated torus), Loop's surface is a box
    /// spline, and the angle between neighbouring faces halves with each level, as the faces do.
    #[test]
    fn test_regular_mesh_smooths_by_halves() {
        let (nu, nv) = (16, 8);
        let mut p = vec![];
        for i in 0..nu {
            for j in 0..nv {
                let (a, b) = (2. * std::f64::consts::PI * i as f64 / nu as f64, 2. * std::f64::consts::PI * j as f64 / nv as f64);
                p.push(Vector3::new((2. + b.cos()) * a.cos(), b.sin(), (2. + b.cos()) * a.sin()));
            }
        }
        let id = |i: usize, j: usize| (i % nu) * nv + (j % nv);
        let mut t = vec![];
        for i in 0..nu {
            for j in 0..nv {
                t.push([id(i, j), id(i, j + 1), id(i + 1, j)]);
                t.push([id(i + 1, j), id(i, j + 1), id(i + 1, j + 1)]);
            }
        }
        let mut last = sharpest(&p, &t);
        for _ in 0..3 {
            let (np, nt) = refine(&p, &t);
            p = np;
            t = nt;
            let now = sharpest(&p, &t);
            assert!((now / last - 0.5).abs() < 0.03, "{} then {}", last, now);
            last = now;
        }
    }

    /// The octahedron's creases smooth away too, more slowly at its corners, where four edges
    /// meet rather than six (extraordinary vertices), and its limit keeps its symmetry.
    #[test]
    fn test_octahedron_smooths() {
        let (mut p, mut t) = octahedron();
        let mut last = sharpest(&p, &t);
        assert!((last - 70.53).abs() < 0.1);
        for _ in 0..4 {
            let (np, nt) = refine(&p, &t);
            p = np;
            t = nt;
            let now = sharpest(&p, &t);
            assert!(now < 0.85 * last, "{} then {}", last, now);
            last = now;
        }
        let lim = limit(&p, &t);
        let along = |axis: Vector3<f64>| lim.iter().map(|v| v.dot(&axis)).fold(f64::MIN, f64::max);
        assert!((along(Vector3::x()) - along(Vector3::y())).abs() < 1e-9 && (along(Vector3::y()) - along(Vector3::z())).abs() < 1e-9);
    }

    /// A flat open patch stays flat, and its boundary stays on its edges' lines.
    #[test]
    fn test_flat_patch_stays_flat() {
        let p = vec![Vector3::new(0., 0., 0.), Vector3::new(1., 0., 0.), Vector3::new(1., 1., 0.), Vector3::new(0., 1., 0.)];
        let t = vec![[0, 1, 2], [0, 2, 3]];
        let (mut p, mut t) = (p, t);
        for _ in 0..3 {
            let (np, nt) = refine(&p, &t);
            p = np;
            t = nt;
        }
        let lim = limit(&p, &t);
        assert!(lim.iter().all(|v| v.z.abs() < 1e-12));
        assert!(lim.iter().all(|v| v.x >= -1e-12 && v.x <= 1. + 1e-12 && v.y >= -1e-12 && v.y <= 1. + 1e-12));
    }
}
