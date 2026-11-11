//! Shapes built as triangle meshes on a grid: the `height_field` (heights from an image or a
//! function), the `parametric` surface (a point for each (u, v), given by functions) and the
//! `bicubic_patch` (a Bezier surface of 16 control points) (POV-Ray 3.7 reference: "Height Field",
//! "Parametric" and "Bicubic Patch").
//!
//! POV-Ray intersects the parametric surface and patch directly; here they are sampled on a grid
//! and smoothed with normals from the surface at each grid point, so fine detail needs a finer
//! grid.

use crate::na::Vector3;
use crate::shapes::mesh::{IndexedMesh, Mesh, SmoothMesh};
use crate::expr::Expr;

/// A grid of points with their normals, `nu` + 1 by `nv` + 1, row by row (u fastest).
struct Grid {
    nu: usize,
    nv: usize,
    points: Vec<Vector3<f64>>,
    normals: Vec<Vector3<f64>>,
}

impl Grid {
    fn sample(nu: usize, nv: usize, f: impl Fn(f64, f64) -> (Vector3<f64>, Vector3<f64>)) -> Grid {
        assert!(nu >= 1 && nv >= 1, "a grid needs at least one step each way");
        let (mut points, mut normals) = (Vec::new(), Vec::new());
        for j in 0..=nv {
            for i in 0..=nu {
                let (p, n) = f(i as f64 / nu as f64, j as f64 / nv as f64);
                points.push(p);
                normals.push(n);
            }
        }
        Grid { nu, nv, points, normals }
    }

    /// The two triangles of each cell, as indices, skipping those `keep` rejects and those of
    /// no area (where the surface pinches to a point).
    fn cells(&self, keep: impl Fn(&[usize; 3]) -> bool) -> Vec<[usize; 3]> {
        let idx = |i: usize, j: usize| j * (self.nu + 1) + i;
        let mut tris = Vec::new();
        for j in 0..self.nv {
            for i in 0..self.nu {
                let (a, b, c, d) = (idx(i, j), idx(i + 1, j), idx(i + 1, j + 1), idx(i, j + 1));
                for t in [[a, b, c], [a, c, d]] {
                    let area = (self.points[t[1]] - self.points[t[0]]).cross(&(self.points[t[2]] - self.points[t[0]])).norm();
                    if area > 0. && keep(&t) {
                        tris.push(t);
                    }
                }
            }
        }
        assert!(!tris.is_empty(), "the surface has no triangles");
        tris
    }

    fn smooth(&self, keep: impl Fn(&[usize; 3]) -> bool) -> SmoothMesh {
        // Where the surface has no normal (a pole), leave it zero: the mesh then uses the face's.
        let normals = self.normals.iter().map(|&n| {
            if n.norm() > 0. && n.iter().all(|x| x.is_finite()) { n.normalize() } else { Vector3::zeros() }
        }).collect();
        SmoothMesh::from_indexed(&[IndexedMesh { positions: self.points.clone(), normals, triangles: self.cells(keep) }])
    }

    fn flat(&self, keep: impl Fn(&[usize; 3]) -> bool) -> Mesh {
        Mesh::new(self.points.clone(), self.cells(keep))
    }
}

/// Heights in [0, 1] on a grid of `width` by `depth` samples.
pub struct Heights {
    pub width: usize,
    pub depth: usize,
    pub values: Vec<f64>,
}

impl Heights {
    /// From an image's brightness: columns along x, rows along z, the first row at z = 1 so that
    /// the image reads the right way up looking down the y axis with z up the page.
    pub fn from_image(path: &str) -> Heights {
        let bytes = crate::assets::read(path).unwrap_or_else(|e| panic!("Failed to load height field image: {}", e));
        let img = image::load_from_memory(&bytes)
            .unwrap_or_else(|e| panic!("Failed to load height field image {}: {}", path, e))
            .to_luma16();
        let (w, h) = (img.width() as usize, img.height() as usize);
        assert!(w >= 2 && h >= 2, "a height field image must be at least 2 by 2 pixels");
        let mut values = Vec::with_capacity(w * h);
        for j in 0..h {
            for i in 0..w {
                values.push(img.get_pixel(i as u32, (h - 1 - j) as u32).0[0] as f64 / 65535.);
            }
        }
        Heights { width: w, depth: h, values }
    }

    /// From a function of x and z, each from 0 to 1.
    pub fn from_function(f: &Expr, width: usize, depth: usize) -> Heights {
        assert!(width >= 2 && depth >= 2, "a height field needs at least 2 by 2 samples");
        let mut values = Vec::with_capacity(width * depth);
        for j in 0..depth {
            for i in 0..width {
                values.push(f.eval(&[i as f64 / (width - 1) as f64, j as f64 / (depth - 1) as f64]));
            }
        }
        Heights { width, depth, values }
    }

    fn at(&self, i: usize, j: usize) -> f64 {
        self.values[j * self.width + i]
    }
}

/// A height field over the unit square in x and z, y from the heights. Smooth, normals come from
/// the slope at each sample. Cells entirely below `water_level` are left out.
pub fn height_field(h: &Heights, smooth: bool, water_level: f64) -> Box<dyn crate::shapes::geometry::Geometry + Sync + Send> {
    let (nu, nv) = (h.width - 1, h.depth - 1);
    let grid = Grid::sample(nu, nv, |u, v| {
        let (i, j) = ((u * nu as f64).round() as usize, (v * nv as f64).round() as usize);
        let p = Vector3::new(u, h.at(i, j), v);
        // Central differences (one-sided at the edges) of height against x and z.
        let (i0, i1) = (i.saturating_sub(1), (i + 1).min(nu));
        let (j0, j1) = (j.saturating_sub(1), (j + 1).min(nv));
        let dx = (h.at(i1, j) - h.at(i0, j)) / ((i1 - i0) as f64 / nu as f64);
        let dz = (h.at(i, j1) - h.at(i, j0)) / ((j1 - j0) as f64 / nv as f64);
        (p, Vector3::new(-dx, 1., -dz))
    });
    let keep = |t: &[usize; 3]| t.iter().any(|&k| grid.points[k].y >= water_level);
    if smooth { Box::new(grid.smooth(keep)) } else { Box::new(grid.flat(keep)) }
}

/// The surface (x(u, v), y(u, v), z(u, v)) for u and v over the given ranges, sampled on a grid
/// of `nu` by `nv` cells, with normals from the cross product of the partial derivatives.
pub fn parametric(f: [&Expr; 3], u_range: (f64, f64), v_range: (f64, f64), nu: usize, nv: usize) -> SmoothMesh {
    let at = |u: f64, v: f64| Vector3::new(f[0].eval(&[u, v]), f[1].eval(&[u, v]), f[2].eval(&[u, v]));
    let (du, dv) = ((u_range.1 - u_range.0) * 1e-6, (v_range.1 - v_range.0) * 1e-6);
    let grid = Grid::sample(nu, nv, |s, t| {
        let u = u_range.0 + s * (u_range.1 - u_range.0);
        let v = v_range.0 + t * (v_range.1 - v_range.0);
        let pu = (at(u + du, v) - at(u - du, v)) / (2. * du);
        let pv = (at(u, v + dv) - at(u, v - dv)) / (2. * dv);
        (at(u, v), pu.cross(&pv))
    });
    grid.smooth(|_| true)
}

/// The cubic Bernstein polynomials and their derivatives at t.
fn bernstein(t: f64) -> ([f64; 4], [f64; 4]) {
    let s = 1. - t;
    ([s * s * s, 3. * s * s * t, 3. * s * t * t, t * t * t],
     [-3. * s * s, 3. * s * s - 6. * s * t, 6. * s * t - 3. * t * t, 3. * t * t])
}

/// A bicubic Bezier patch of 4 rows of 4 control points (u along a row, v down the rows),
/// sampled on a grid of `steps` by `steps` cells.
pub fn bicubic_patch(points: &[Vector3<f64>], steps: usize) -> SmoothMesh {
    assert_eq!(points.len(), 16, "a bicubic patch has 16 control points");
    let grid = Grid::sample(steps, steps, |u, v| {
        let ((bu, du), (bv, dv)) = (bernstein(u), bernstein(v));
        let (mut p, mut pu, mut pv) = (Vector3::zeros(), Vector3::zeros(), Vector3::zeros());
        for j in 0..4 {
            for i in 0..4 {
                let c = points[j * 4 + i];
                p += c * (bu[i] * bv[j]);
                pu += c * (du[i] * bv[j]);
                pv += c * (bu[i] * dv[j]);
            }
        }
        (p, pu.cross(&pv))
    });
    grid.smooth(|_| true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::shapes::geometry::Geometry;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    fn down(g: &dyn Geometry, x: f64, z: f64) -> Option<Vector3<f64>> {
        g.intersects(&Ray { ro: v(x, 5., z), rd: v(0., -1., 0.) }).map(|i| i.point)
    }

    #[test]
    fn test_height_field_from_function() {
        let f = Expr::parse("0.5 * x", &["x", "z"]).unwrap();
        let h = Heights::from_function(&f, 11, 11);
        let g = height_field(&h, true, 0.);
        let p = down(g.as_ref(), 0.43, 0.47).unwrap();
        assert!((p.y - 0.215).abs() < 1e-9, "{}", p);
        let n = g.intersects(&Ray { ro: v(0.43, 5., 0.47), rd: v(0., -1., 0.) }).unwrap().normal;
        assert!((n - v(-0.5, 1., 0.).normalize()).norm() < 1e-9, "{}", n);
        assert!(down(g.as_ref(), 1.5, 0.5).is_none());
        let b = g.bounds();
        assert!((b.max.x - 1.).abs() < 1e-12 && (b.max.y - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_height_field_image_orientation() {
        // A 2 x 2 image, white only at the top left: that corner is x = 0, z = 1.
        let mut img = image::GrayImage::new(2, 2);
        img.put_pixel(0, 0, image::Luma([255]));
        let dir = std::env::temp_dir().join(format!("rays-hf-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("hf.png");
        img.save(&path).unwrap();
        let h = Heights::from_image(path.to_str().unwrap());
        assert_eq!((h.width, h.depth), (2, 2));
        assert_eq!(h.at(0, 1), 1.);
        assert_eq!(h.at(0, 0) + h.at(1, 0) + h.at(1, 1), 0.);
    }

    #[test]
    fn test_water_level_drops_low_cells() {
        let f = Expr::parse("x", &["x", "z"]).unwrap();
        let g = height_field(&Heights::from_function(&f, 11, 3), false, 0.55);
        assert!(down(g.as_ref(), 0.2, 0.5).is_none());
        assert!(down(g.as_ref(), 0.8, 0.5).is_some());
    }

    #[test]
    fn test_parametric_sphere() {
        let vars = ["u", "v"];
        let x = Expr::parse("cos(u) * sin(v)", &vars).unwrap();
        let y = Expr::parse("cos(v)", &vars).unwrap();
        let z = Expr::parse("sin(u) * sin(v)", &vars).unwrap();
        let s = parametric([&x, &y, &z], (0., 2. * std::f64::consts::PI), (0., std::f64::consts::PI), 64, 32);
        let i = s.intersects(&Ray { ro: v(0.1, 0.2, -5.), rd: v(0., 0., 1.) }).unwrap();
        assert!((i.point.norm() - 1.).abs() < 0.01, "{}", i.point);
        // Normals at the grid points are the sphere's (either way out, depending on orientation).
        assert!(i.normal.dot(&i.point.normalize()).abs() > 0.99);
    }

    #[test]
    fn test_flat_bicubic_patch() {
        // Control points evenly spaced on the unit square in xz: the patch is that square.
        let pts: Vec<Vector3<f64>> = (0..16).map(|k| v((k % 4) as f64 / 3., 0., (k / 4) as f64 / 3.)).collect();
        let p = bicubic_patch(&pts, 4);
        let hit = down(&p, 0.3, 0.7).unwrap();
        assert!(hit.y.abs() < 1e-12);
        assert!(down(&p, 1.3, 0.7).is_none());
        // Raising the inner controls makes a bump.
        let mut bumped = pts.clone();
        for k in [5, 6, 9, 10] { bumped[k].y = 1.; }
        let b = bicubic_patch(&bumped, 16);
        let top = down(&b, 0.5, 0.5).unwrap();
        // At the centre, each inner weight is (3/8)^2 and there are four of them.
        assert!((top.y - 4. * (0.375f64 * 0.375)).abs() < 1e-3, "{}", top);
    }
}
