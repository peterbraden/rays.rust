//! Flat outlines and the shapes built from them: the `polygon`, a flat outline in space, and the
//! `prism`, an outline in the xz plane swept along y (POV-Ray 3.7 reference, sections 2.4.2.4
//! "Polygon" and 2.4.1.9 "Prism").
//!
//! An outline is one or more closed loops, and a point is inside if a ray from it crosses the
//! loops an odd number of times (the even-odd rule), so a loop inside another makes a hole.

use crate::shapes::geometry::Geometry;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;

type P2 = Vector2<f64>;

/// Closed loops in a plane.
#[derive(Debug, Clone)]
pub struct Outline {
    /// Every edge of every loop, as (from, to).
    edges: Vec<(P2, P2)>,
    min: P2,
    max: P2,
}

impl Outline {
    pub fn new(loops: Vec<Vec<P2>>) -> Outline {
        let mut edges = Vec::new();
        let (mut min, mut max) = (P2::repeat(f64::INFINITY), P2::repeat(f64::NEG_INFINITY));
        for l in loops.iter().filter(|l| l.len() >= 3) {
            for i in 0..l.len() {
                let (a, b) = (l[i], l[(i + 1) % l.len()]);
                if a != b {
                    edges.push((a, b));
                }
                min = min.inf(&a);
                max = max.sup(&a);
            }
        }
        assert!(!edges.is_empty(), "an outline needs a loop of at least three points");
        Outline { edges, min, max }
    }

    /// Split a list of points into loops the way POV-Ray does: a loop ends where its first point
    /// is repeated. A final loop left open is closed.
    pub fn from_points(points: &[P2]) -> Outline {
        let mut loops = Vec::new();
        let mut current: Vec<P2> = Vec::new();
        for &p in points {
            if current.len() > 1 && p == current[0] {
                loops.push(std::mem::take(&mut current));
            } else {
                current.push(p);
            }
        }
        if !current.is_empty() {
            loops.push(current);
        }
        Outline::new(loops)
    }

    /// Loops from cubic Bezier segments, four points each (start, two controls, end): POV-Ray's
    /// `bezier_spline`. A loop ends when a segment ends at the loop's first point. Each curve is
    /// flattened into `steps` straight edges.
    pub fn from_bezier(points: &[P2], steps: usize) -> Outline {
        assert!(points.len() % 4 == 0, "bezier outlines take four points per segment, got {}", points.len());
        let mut loops = Vec::new();
        let mut current: Vec<P2> = Vec::new();
        for seg in points.chunks(4) {
            if current.is_empty() {
                current.push(seg[0]);
            }
            for i in 1..=steps {
                let t = i as f64 / steps as f64;
                let s = 1. - t;
                let p = seg[0] * (s * s * s) + seg[1] * (3. * s * s * t) + seg[2] * (3. * s * t * t) + seg[3] * (t * t * t);
                current.push(p);
            }
            if seg[3] == current[0] {
                current.pop();
                loops.push(std::mem::take(&mut current));
            }
        }
        if !current.is_empty() {
            loops.push(current);
        }
        Outline::new(loops)
    }

    pub fn contains(&self, p: &P2) -> bool {
        if p.x < self.min.x || p.y < self.min.y || p.x > self.max.x || p.y > self.max.y {
            return false;
        }
        let mut inside = false;
        for (a, b) in &self.edges {
            // Edges crossing the horizontal line through p, to the right of it.
            if (a.y > p.y) != (b.y > p.y) {
                let x = a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x);
                if x > p.x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    /// Unit vector perpendicular to the edge a -> b, pointing out of the outline at the point m
    /// on it.
    fn outward(&self, a: &P2, b: &P2, m: &P2) -> P2 {
        let e = b - a;
        let n = P2::new(e.y, -e.x).normalize();
        let probe = m + n * (1e-7 * (1. + e.norm()));
        if self.contains(&probe) { -n } else { n }
    }
}

/// A flat outline in space: points in 3D that lie in one plane.
pub struct Polygon {
    outline: Outline,
    origin: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    normal: Vector3<f64>,
    bounds: BBox,
}

impl Polygon {
    /// Points in POV-Ray's form: a loop closes when its first point repeats. The plane is that of
    /// the points (by Newell's method, which averages over all of them), and points off it are
    /// projected onto it.
    pub fn new(points: &[Vector3<f64>]) -> Polygon {
        assert!(points.len() >= 3, "polygon needs at least three points");
        let mut n = Vector3::zeros();
        for i in 0..points.len() {
            let (a, b) = (points[i], points[(i + 1) % points.len()]);
            n += Vector3::new((a.y - b.y) * (a.z + b.z), (a.z - b.z) * (a.x + b.x), (a.x - b.x) * (a.y + b.y));
        }
        assert!(n.norm() > 0., "polygon points are all in a line");
        let normal = n.normalize();
        let a = if normal.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
        let u = normal.cross(&a).normalize();
        let v = normal.cross(&u);
        let origin = points[0];
        let flat: Vec<P2> = points.iter().map(|p| P2::new((p - origin).dot(&u), (p - origin).dot(&v))).collect();
        let bounds = points.iter().fold(BBox::empty(), |b, p| b.union_point(&(origin + u * (p - origin).dot(&u) + v * (p - origin).dot(&v))));
        Polygon { outline: Outline::from_points(&flat), origin, u, v, normal, bounds }
    }
}

impl Geometry for Polygon {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let denom = rd.dot(&self.normal);
        if denom == 0. {
            return None;
        }
        let dist = (self.origin - r.ro).dot(&self.normal) / denom;
        if dist <= 0. {
            return None;
        }
        let point = r.ro + rd * dist;
        let rel = point - self.origin;
        if !self.outline.contains(&P2::new(rel.dot(&self.u), rel.dot(&self.v))) {
            return None;
        }
        Some(RawIntersection { dist, point, normal: self.normal })
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }
}

/// How a prism's outline is swept along y.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sweep {
    /// The outline unchanged at every height: a straight-sided prism.
    Linear,
    /// The outline scaled by the height, towards an apex at the origin: a pyramid or cone.
    Conic,
}

/// An outline in the xz plane (points given as [x, z]) swept along y between two heights, closed
/// by flat caps unless `open`.
pub struct Prism {
    outline: Outline,
    low: f64,
    high: f64,
    sweep: Sweep,
    open: bool,
}

impl Prism {
    pub fn new(outline: Outline, height1: f64, height2: f64, sweep: Sweep, open: bool) -> Prism {
        assert!(height1 != height2, "prism heights must differ");
        let (low, high) = (height1.min(height2), height1.max(height2));
        if sweep == Sweep::Conic {
            assert!(low >= 0. || high <= 0., "a conic prism can't span its apex at y = 0");
        }
        Prism { outline, low, high, sweep, open }
    }

    /// The outline's point under (x, y, z) at height y.
    fn flat(&self, p: &Vector3<f64>) -> P2 {
        match self.sweep {
            Sweep::Linear => P2::new(p.x, p.z),
            Sweep::Conic => P2::new(p.x / p.y, p.z / p.y),
        }
    }
}

impl Geometry for Prism {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let o = r.ro;
        let mut best: Option<(f64, Vector3<f64>)> = None;
        let mut consider = |t: f64, n: Vector3<f64>| {
            if t > 0. && best.is_none_or(|(b, _)| t < b) {
                best = Some((t, n));
            }
        };

        if !self.open && rd.y != 0. {
            for (h, ny) in [(self.low, -1.), (self.high, 1.)] {
                let t = (h - o.y) / rd.y;
                if t > 0. && self.outline.contains(&self.flat(&(o + rd * t))) {
                    consider(t, Vector3::new(0., ny, 0.));
                }
            }
        }

        for (a, b) in &self.outline.edges {
            let (t, m) = match self.sweep {
                Sweep::Linear => {
                    // 2D ray against the segment: o + t d = a + u (b - a).
                    let e = b - a;
                    let d2 = P2::new(rd.x, rd.z);
                    let den = d2.x * e.y - d2.y * e.x;
                    if den == 0. { continue; }
                    let w = a - P2::new(o.x, o.z);
                    let t = (w.x * e.y - w.y * e.x) / den;
                    let u = (w.x * d2.y - w.y * d2.x) / den;
                    if !(0. ..=1.).contains(&u) { continue; }
                    (t, a + e * u)
                },
                Sweep::Conic => {
                    // The plane through the apex and the edge lifted to y = 1.
                    let (a3, b3) = (Vector3::new(a.x, 1., a.y), Vector3::new(b.x, 1., b.y));
                    let n = a3.cross(&b3);
                    let den = rd.dot(&n);
                    if den == 0. { continue; }
                    let t = -o.dot(&n) / den;
                    let p = o + rd * t;
                    if p.y == 0. { continue; }
                    let f = self.flat(&p);
                    let e = b - a;
                    let u = (f - a).dot(&e) / e.norm_squared();
                    if !(0. ..=1.).contains(&u) { continue; }
                    (t, f)
                },
            };
            if t <= 0. { continue; }
            let y = o.y + rd.y * t;
            if y < self.low || y > self.high { continue; }
            let n2 = self.outline.outward(a, b, &m);
            let normal = match self.sweep {
                Sweep::Linear => Vector3::new(n2.x, 0., n2.y),
                Sweep::Conic => {
                    // Perpendicular to both the edge and the line to the apex, and outwards: below
                    // the apex the outline is scaled by a negative height, so turned about.
                    let (a3, b3) = (Vector3::new(a.x, 1., a.y), Vector3::new(b.x, 1., b.y));
                    let n = a3.cross(&b3).normalize();
                    let out = Vector3::new(n2.x, 0., n2.y) * y.signum();
                    if n.dot(&out) < 0. { -n } else { n }
                },
            };
            consider(t, normal);
        }

        best.map(|(dist, normal)| RawIntersection { dist, point: o + rd * dist, normal })
    }

    fn bounds(&self) -> BBox {
        let (min, max) = (self.outline.min, self.outline.max);
        match self.sweep {
            Sweep::Linear => BBox::new(Vector3::new(min.x, self.low, min.y), Vector3::new(max.x, self.high, max.y)),
            Sweep::Conic => {
                let mut b = BBox::empty();
                for h in [self.low, self.high] {
                    b = b.union_point(&Vector3::new(min.x * h, h, min.y * h))
                        .union_point(&Vector3::new(max.x * h, h, max.y * h));
                }
                b
            },
        }
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        !self.open && p.y > self.low && p.y < self.high && self.outline.contains(&self.flat(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }
    fn p(x: f64, y: f64) -> P2 { P2::new(x, y) }

    fn square_with_hole() -> Outline {
        Outline::from_points(&[
            p(-2., -2.), p(2., -2.), p(2., 2.), p(-2., 2.), p(-2., -2.),
            p(-1., -1.), p(1., -1.), p(1., 1.), p(-1., 1.), p(-1., -1.),
        ])
    }

    #[test]
    fn test_even_odd_hole() {
        let o = square_with_hole();
        assert!(o.contains(&p(1.5, 0.)));
        assert!(!o.contains(&p(0., 0.)));
        assert!(!o.contains(&p(3., 0.)));
        assert_eq!(o.edges.len(), 8);
    }

    #[test]
    fn test_bezier_circle_like() {
        // Two Bezier halves of a rough circle.
        let k = 0.5523 * 2.;
        let o = Outline::from_bezier(&[
            p(1., 0.), p(1., k), p(-1., k), p(-1., 0.),
            p(-1., 0.), p(-1., -k), p(1., -k), p(1., 0.),
        ], 8);
        assert!(o.contains(&p(0., 0.)));
        assert!(!o.contains(&p(1.1, 0.)));
        assert_eq!(o.edges.len(), 16);
    }

    #[test]
    fn test_polygon_in_any_plane() {
        // A unit square in the plane x = 3.
        let poly = Polygon::new(&[v(3., 0., 0.), v(3., 1., 0.), v(3., 1., 1.), v(3., 0., 1.)]);
        let i = poly.intersects(&Ray { ro: v(0., 0.5, 0.5), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.dist - 3.).abs() < 1e-9);
        assert!((i.normal.x.abs() - 1.).abs() < 1e-9);
        assert!(poly.intersects(&Ray { ro: v(0., 1.5, 0.5), rd: v(1., 0., 0.) }).is_none());
    }

    #[test]
    fn test_linear_prism() {
        let pr = Prism::new(square_with_hole(), 0., 1., Sweep::Linear, false);
        // Through the side.
        let side = pr.intersects(&Ray { ro: v(-5., 0.5, 1.5), rd: v(1., 0., 0.) }).unwrap();
        assert!((side.dist - 3.).abs() < 1e-9);
        assert_eq!(side.normal, v(-1., 0., 0.));
        // Into the hole from the side, hitting its wall facing inwards.
        let hole = pr.intersects(&Ray { ro: v(0., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((hole.dist - 1.).abs() < 1e-9);
        assert_eq!(hole.normal, v(-1., 0., 0.));
        // Down onto the cap, and through the hole.
        let cap = pr.intersects(&Ray { ro: v(1.5, 5., 1.5), rd: v(0., -1., 0.) }).unwrap();
        assert!((cap.dist - 4.).abs() < 1e-9);
        assert_eq!(cap.normal, v(0., 1., 0.));
        assert!(pr.intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., -1., 0.) }).is_none());
        assert!(pr.inside(&v(1.5, 0.5, 0.)) && !pr.inside(&v(0., 0.5, 0.)));
    }

    #[test]
    fn test_conic_prism_is_a_pyramid() {
        let square = Outline::from_points(&[p(-1., -1.), p(1., -1.), p(1., 1.), p(-1., 1.)]);
        let pr = Prism::new(square, 0., 1., Sweep::Conic, false);
        // At height 0.5 the square is half size.
        let i = pr.intersects(&Ray { ro: v(-5., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.dist - 4.5).abs() < 1e-9, "{}", i.dist);
        let expected = v(-1., -1., 0.).normalize();
        assert!((i.normal - expected).norm() < 1e-9, "{}", i.normal);
        let top = pr.intersects(&Ray { ro: v(0.5, 5., 0.5), rd: v(0., -1., 0.) }).unwrap();
        assert!((top.dist - 4.).abs() < 1e-9);
        assert_eq!(top.normal, v(0., 1., 0.));
        assert!(pr.intersects(&Ray { ro: v(-5., 0.5, 0.6), rd: v(1., 0., 0.) }).is_none());
        // Below the apex, mirrored.
        let square = Outline::from_points(&[p(-1., -1.), p(1., -1.), p(1., 1.), p(-1., 1.)]);
        let under = Prism::new(square, -1., 0., Sweep::Conic, false);
        let j = under.intersects(&Ray { ro: v(-5., -0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((j.dist - 4.5).abs() < 1e-9);
        assert!((j.normal - v(-1., 1., 0.).normalize()).norm() < 1e-9, "{}", j.normal);
        let bottom = under.intersects(&Ray { ro: v(0.5, -5., 0.5), rd: v(0., 1., 0.) }).unwrap();
        assert_eq!(bottom.normal, v(0., -1., 0.));
    }
}
