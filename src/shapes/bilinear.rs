//! # Bilinear patches
//!
//! The surface swept by a line whose ends run along two edges: points
//!
//! ```text
//! p(u, v) = (1 - u)(1 - v) p00 + u (1 - v) p10 + (1 - u) v p01 + u v p11,   u, v in [0, 1]
//! ```
//!
//! for four corners that needn't lie in a plane: a quadrilateral if they do, and otherwise a
//! saddle (a hyperbolic paraboloid), the simplest curved surface, and what pbrt-v4 builds its
//! quads and its meshes of quads from (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 4th ed., section 6.6; cited below as PBR).
//!
//! ## Intersection
//!
//! Alexander Reshetov, "Cool Patches: A Geometric Approach to Ray/Bilinear Patch Intersections",
//! in "Ray Tracing Gems", ed. Haines and Akenine-Möller, Apress 2019, chapter 8,
//! doi:10.1007/978-1-4842-4427-2_8, as PBR's `IntersectBilinearPatch`: for each u, the patch's
//! line from p(u, 0) to p(u, 1) either meets the ray or passes it; requiring the two to be
//! coplanar gives a quadratic in u, and for each root in [0, 1], the closest approach of the
//! ray and that line gives v and the distance.
//!
//! ## Sampling
//!
//! Uniform (u, v) is uniform by area only for a parallelogram; elsewhere the area per unit of
//! (u, v), |∂p/∂u × ∂p/∂v|, varies. So (u, v) are drawn from a small table of that area, by
//! cells, and the density per area at the point is the table's density over the area element
//! there: exact for any patch, and for a parallelogram, uniform.
use crate::environment::Distribution1D;
use crate::intersection::RawIntersection;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

/// Cells along each side of the area table.
const TABLE: usize = 8;

pub struct BilinearPatch {
    pub p00: Vector3<f64>,
    pub p10: Vector3<f64>,
    pub p01: Vector3<f64>,
    pub p11: Vector3<f64>,
    /// The area of each cell of (u, v), row by row, and their sum.
    cells: Distribution1D,
    area: f64,
}

impl BilinearPatch {
    pub fn new(p00: Vector3<f64>, p10: Vector3<f64>, p01: Vector3<f64>, p11: Vector3<f64>) -> BilinearPatch {
        let mut patch = BilinearPatch { p00, p10, p01, p11, cells: Distribution1D::new(vec![1.]), area: 0. };
        // Each cell's area by the midpoint rule; fine enough for choosing cells, and the density
        // at a point uses the exact area element.
        let step = 1. / TABLE as f64;
        let areas: Vec<f64> = (0..TABLE * TABLE).map(|k| {
            let (i, j) = (k % TABLE, k / TABLE);
            patch.area_element((i as f64 + 0.5) * step, (j as f64 + 0.5) * step) * step * step
        }).collect();
        patch.area = areas.iter().sum();
        assert!(patch.area > 0., "A bilinear patch needs some area");
        patch.cells = Distribution1D::new(areas);
        patch
    }

    pub fn point(&self, u: f64, v: f64) -> Vector3<f64> {
        self.p00 * ((1. - u) * (1. - v)) + self.p10 * (u * (1. - v)) + self.p01 * ((1. - u) * v) + self.p11 * (u * v)
    }

    /// ∂p/∂u and ∂p/∂v.
    fn derivatives(&self, u: f64, v: f64) -> (Vector3<f64>, Vector3<f64>) {
        let dpdu = (self.p10 * (1. - v) + self.p11 * v) - (self.p00 * (1. - v) + self.p01 * v);
        let dpdv = (self.p01 * (1. - u) + self.p11 * u) - (self.p00 * (1. - u) + self.p10 * u);
        (dpdu, dpdv)
    }

    /// The surface's normal at (u, v): ∂p/∂u × ∂p/∂v, so for corners listed anticlockwise
    /// (p00, p10, p11, p01) seen from in front, it faces the viewer.
    pub fn normal(&self, u: f64, v: f64) -> Vector3<f64> {
        let (dpdu, dpdv) = self.derivatives(u, v);
        dpdu.cross(&dpdv).normalize()
    }

    fn area_element(&self, u: f64, v: f64) -> f64 {
        let (dpdu, dpdv) = self.derivatives(u, v);
        dpdu.cross(&dpdv).norm()
    }

    /// The nearest hit along the ray beyond `min`: distance (along the unit direction) and
    /// (u, v).
    fn hit(&self, r: &Ray, min: f64) -> Option<(f64, Vector2<f64>)> {
        let d = r.rd.normalize();
        let o = r.ro;
        let qn = (self.p10 - self.p00).cross(&(self.p01 - self.p11));
        let e11 = self.p11 - self.p10;
        let e00 = self.p01 - self.p00;
        let q00 = self.p00 - o;
        let q10 = self.p10 - o;
        let a = q00.cross(&d).dot(&e00);
        let c = qn.dot(&d);
        let mut b = q10.cross(&d).dot(&e11);
        b -= a + c;
        let det = b * b - 4. * a * c;
        if det < 0. {
            return None;
        }
        let det = det.sqrt();
        let (u1, u2) = if c == 0. {
            if b == 0. { return None; }
            (-a / b, -1.)
        } else {
            let u1 = (-b - det.copysign(b)) / 2.;
            let u2 = if u1 != 0. { a / u1 } else { -1. };
            (u1 / c, u2)
        };
        let mut best: Option<(f64, Vector2<f64>)> = None;
        for u in [u1, u2] {
            if !(0. ..=1.).contains(&u) {
                continue;
            }
            let pa = q00 * (1. - u) + q10 * u;
            let pb = e00 * (1. - u) + e11 * u;
            let n = d.cross(&pb);
            let det = n.dot(&n);
            if det == 0. {
                continue;
            }
            let n = n.cross(&pa);
            let t1 = n.dot(&pb);
            let v1 = n.dot(&d);
            if t1 > 0. && (0. ..=det).contains(&v1) {
                let t = t1 / det;
                if t > min && best.is_none_or(|(bt, _)| t < bt) {
                    best = Some((t, Vector2::new(u, v1 / det)));
                }
            }
        }
        best
    }
}

impl Geometry for BilinearPatch {
    /// Two triangles, if the corners are coplanar (a quad); a saddle can't be flattened.
    fn flatten(&self) -> Option<Vec<crate::wavefront::flat::Prim>> {
        let n = (self.p10 - self.p00).cross(&(self.p01 - self.p00));
        let size = (self.p11 - self.p00).norm().max(1e-300);
        if (self.p11 - self.p00).dot(&n.normalize()).abs() > 1e-9 * size {
            return None;
        }
        Some(vec![crate::wavefront::flat::Prim::Triangle { v: [self.p00, self.p10, self.p11], n: None },
                  crate::wavefront::flat::Prim::Triangle { v: [self.p00, self.p11, self.p01], n: None }])
    }

    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_beyond(r, 0.)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        let (t, uv) = self.hit(r, min)?;
        let point = self.point(uv.x, uv.y);
        Some(RawIntersection { face: 0, part: 0, dist: t, point, normal: self.normal(uv.x, uv.y) })
    }

    fn bounds(&self) -> BBox {
        // The patch lies in the convex hull of its corners.
        BBox::new(self.p00.inf(&self.p10).inf(&self.p01).inf(&self.p11), self.p00.sup(&self.p10).sup(&self.p01).sup(&self.p11))
    }

    fn uv(&self, p: &Vector3<f64>, _face: u32) -> Option<Vector2<f64>> {
        // Invert p(u, v) by Newton's method from the centre, in the least-squares sense.
        let mut uv = Vector2::new(0.5, 0.5);
        for _ in 0..20 {
            let (dpdu, dpdv) = self.derivatives(uv.x, uv.y);
            let r = self.point(uv.x, uv.y) - p;
            let (a, b, c) = (dpdu.dot(&dpdu), dpdu.dot(&dpdv), dpdv.dot(&dpdv));
            let (g1, g2) = (dpdu.dot(&r), dpdv.dot(&r));
            let det = a * c - b * b;
            if det.abs() < 1e-300 {
                break;
            }
            let step = Vector2::new((c * g1 - b * g2) / det, (a * g2 - b * g1) / det);
            uv -= step;
            if step.norm() < 1e-12 {
                break;
            }
        }
        Some(uv)
    }

    fn surface_area(&self) -> f64 {
        self.area
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let (uv, _) = self.sample_uv(u);
        Some((self.point(uv.x, uv.y), self.normal(uv.x, uv.y)))
    }

    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let (uv, pdf_uv) = self.sample_uv(u);
        let (y, n) = (self.point(uv.x, uv.y), self.normal(uv.x, uv.y));
        let pdf = crate::shapes::geometry::area_pdf(from, &y, &n, 1.) * pdf_uv / self.area_element(uv.x, uv.y);
        (pdf > 0. && pdf.is_finite()).then_some((y, n, pdf))
    }

    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        let uv = self.uv(y, 0).unwrap();
        let k = ((uv.y * TABLE as f64) as usize).min(TABLE - 1) * TABLE + ((uv.x * TABLE as f64) as usize).min(TABLE - 1);
        let pdf_uv = self.cells.probability(k) * (TABLE * TABLE) as f64;
        crate::shapes::geometry::area_pdf(from, y, n, 1.) * pdf_uv / self.area_element(uv.x, uv.y)
    }
}

impl BilinearPatch {
    /// (u, v) drawn from the area table, and its density over the unit square.
    fn sample_uv(&self, u: [f64; 3]) -> (Vector2<f64>, f64) {
        let (_, _, k) = self.cells.sample(u[0]);
        let (i, j) = (k % TABLE, k / TABLE);
        let step = 1. / TABLE as f64;
        let uv = Vector2::new((i as f64 + u[1]) * step, (j as f64 + u[2]) * step);
        (uv, self.cells.probability(k) * (TABLE * TABLE) as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn saddle() -> BilinearPatch {
        BilinearPatch::new(Vector3::new(0., 0., 0.), Vector3::new(1., 0., 1.), Vector3::new(0., 1., 1.), Vector3::new(1., 1., 0.))
    }

    #[test]
    fn test_flat_patch_is_a_quad() {
        let q = BilinearPatch::new(Vector3::new(0., 0., 0.), Vector3::new(2., 0., 0.), Vector3::new(0., 1., 0.), Vector3::new(2., 1., 0.));
        assert!((q.surface_area() - 2.).abs() < 1e-12);
        let hit = q.intersects(&Ray { ro: Vector3::new(0.5, 0.25, 3.), rd: Vector3::new(0., 0., -1.) }).unwrap();
        assert!((hit.dist - 3.).abs() < 1e-12);
        assert!((hit.normal - Vector3::z()).norm() < 1e-12);
        let uv = q.uv(&hit.point, 0).unwrap();
        assert!((uv - Vector2::new(0.25, 0.25)).norm() < 1e-9, "{}", uv);
        assert!(q.intersects(&Ray { ro: Vector3::new(2.5, 0.5, 3.), rd: Vector3::new(0., 0., -1.) }).is_none());
    }

    /// Rays at random points of the saddle hit it there.
    #[test]
    fn test_saddle_hits_where_aimed() {
        let s = saddle();
        let mut rng = Pcg32::new(4, 4);
        for _ in 0..500 {
            let (u, v) = (rng.next_f64(), rng.next_f64());
            let target = s.point(u, v);
            let ro = target + Vector3::new(0.3, 0.2, 5.);
            let hit = s.intersects(&Ray { ro, rd: target - ro }).unwrap();
            assert!((hit.point - target).norm() < 1e-9, "{} vs {}", hit.point, target);
            let found = s.uv(&hit.point, 0).unwrap();
            assert!((found - Vector2::new(u, v)).norm() < 1e-9);
        }
    }

    /// Light sampling's densities integrate to the solid angle the patch subtends, and agree
    /// with `pdf_from`.
    #[test]
    fn test_saddle_sampling_is_consistent() {
        let s = saddle();
        let from = Vector3::new(0.5, 0.5, 4.);
        let mut rng = Pcg32::new(6, 1);
        let n = 100_000;
        let mut sum = 0.;
        for _ in 0..n {
            let (y, normal, pdf) = s.sample_from(&from, [rng.next_f64(), rng.next_f64(), rng.next_f64()]).unwrap();
            assert!((s.pdf_from(&from, &y, &normal) / pdf - 1.).abs() < 1e-6);
            sum += 1. / pdf;
        }
        // The solid angle by brute force: hits of rays uniform over a cone round the patch.
        let m = 400_000;
        let cos_max = 0.8f64;
        let cone = 2. * std::f64::consts::PI * (1. - cos_max);
        let z = (Vector3::new(0.5, 0.5, 0.5) - from).normalize();
        let (a, b) = crate::material::phase::orthonormal_basis(&z);
        let hits = (0..m).filter(|_| {
            let c = 1. - rng.next_f64() * (1. - cos_max);
            let sn = (1. - c * c).sqrt();
            let phi = 2. * std::f64::consts::PI * rng.next_f64();
            let d = a * (sn * phi.cos()) + b * (sn * phi.sin()) + z * c;
            s.intersects(&Ray { ro: from, rd: d }).is_some()
        }).count();
        let solid = cone * hits as f64 / m as f64;
        assert!((sum / n as f64 / solid - 1.).abs() < 0.02, "{} vs {}", sum / n as f64, solid);
    }
}
