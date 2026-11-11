//! Shapes made by spinning a 2D profile about the y axis: the `lathe`, whose profile is a spline
//! in (radius, height), and the `sor` (surface of revolution), whose squared radius is a cubic in
//! height (POV-Ray 3.7 reference: "Lathe" and "Surface of Revolution").
//!
//! For a lathe segment with radius R(s) and height Y(s), a ray o + t d meets the surface where
//! y(t) = Y(s) and |xz(t)| = R(s). Eliminating t via t = (Y(s) - o.y) / d.y and clearing the
//! division gives, in s,
//!     (o.x d.y + (Y(s) - o.y) d.x)^2 + (o.z d.y + (Y(s) - o.y) d.z)^2 - d.y^2 R(s)^2 = 0,
//! a polynomial of twice the spline's degree. For the sor, x^2 + z^2 = f(y) is directly a cubic
//! in t.

use crate::shapes::geometry::Geometry;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polygon::Outline;
use crate::shapes::polynomial as poly;

type P2 = Vector2<f64>;

/// How a lathe's points make its profile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spline {
    /// Straight lines between consecutive points.
    Linear,
    /// Catmull-Rom curves through every point but the first and last, which only shape the ends.
    Cubic,
    /// Cubic Bezier curves, four points per segment: start, two controls, end.
    Bezier,
}

/// One piece of the profile: radius and height as polynomials in s from 0 to 1.
#[derive(Debug, Clone)]
struct Segment {
    r: Vec<f64>,
    y: Vec<f64>,
}

impl Segment {
    fn at(&self, s: f64) -> P2 {
        P2::new(poly::eval(&self.r, s), poly::eval(&self.y, s))
    }
}

fn split(points: [P2; 4], m: [[f64; 4]; 4]) -> Segment {
    // Power-basis coefficients, lowest first: row k of m weights the points for s^k.
    let coeffs = |f: &dyn Fn(&P2) -> f64| -> Vec<f64> {
        (0..4).map(|k| (0..4).map(|i| m[k][i] * f(&points[i])).sum()).collect()
    };
    Segment { r: coeffs(&|p| p.x), y: coeffs(&|p| p.y) }
}

const CATMULL_ROM: [[f64; 4]; 4] = [
    [0., 1., 0., 0.],
    [-0.5, 0., 0.5, 0.],
    [1., -2.5, 2., -0.5],
    [-0.5, 1.5, -1.5, 0.5],
];

const BEZIER: [[f64; 4]; 4] = [
    [1., 0., 0., 0.],
    [-3., 3., 0., 0.],
    [3., -6., 3., 0.],
    [-1., 3., -3., 1.],
];

/// A profile of (radius, height) points spun about the y axis.
pub struct Lathe {
    segments: Vec<Segment>,
    /// The profile flattened into straight lines, for inside tests and to tell which way is out.
    outline: Outline,
    /// +1 if the profile runs anticlockwise in the (radius, height) plane, so that outwards is to
    /// the right of its direction; -1 if clockwise.
    orientation: f64,
    bounds: BBox,
}

impl Lathe {
    pub fn new(points: &[P2], spline: Spline) -> Lathe {
        let segments: Vec<Segment> = match spline {
            Spline::Linear => {
                assert!(points.len() >= 2, "a linear lathe needs at least two points");
                points.windows(2).map(|w| Segment {
                    r: vec![w[0].x, w[1].x - w[0].x],
                    y: vec![w[0].y, w[1].y - w[0].y],
                }).collect()
            },
            Spline::Cubic => {
                assert!(points.len() >= 4, "a cubic lathe needs at least four points");
                points.windows(4).map(|w| split([w[0], w[1], w[2], w[3]], CATMULL_ROM)).collect()
            },
            Spline::Bezier => {
                assert!(points.len() >= 4 && points.len().is_multiple_of(4),
                    "a bezier lathe needs four points per segment, got {}", points.len());
                points.chunks(4).map(|w| split([w[0], w[1], w[2], w[3]], BEZIER)).collect()
            },
        };

        let samples = if spline == Spline::Linear { 1 } else { 32 };
        let mut flat: Vec<P2> = Vec::new();
        for seg in &segments {
            for i in 0..samples {
                flat.push(seg.at(i as f64 / samples as f64));
            }
        }
        flat.push(segments.last().unwrap().at(1.));
        let area: f64 = (0..flat.len()).map(|i| {
            let (a, b) = (flat[i], flat[(i + 1) % flat.len()]);
            a.x * b.y - b.x * a.y
        }).sum();

        let (mut rmax, mut ymin, mut ymax) = (0f64, f64::INFINITY, f64::NEG_INFINITY);
        for seg in &segments {
            let mut ss = vec![0., 1.];
            ss.extend(poly::roots_in(&poly::derivative(&seg.r), 0., 1.));
            ss.extend(poly::roots_in(&poly::derivative(&seg.y), 0., 1.));
            for s in ss {
                let p = seg.at(s);
                rmax = rmax.max(p.x.abs());
                ymin = ymin.min(p.y);
                ymax = ymax.max(p.y);
            }
        }
        let bounds = BBox::new(Vector3::new(-rmax, ymin, -rmax), Vector3::new(rmax, ymax, rmax));
        Lathe { segments, outline: Outline::new(vec![flat]), orientation: if area >= 0. { 1. } else { -1. }, bounds }
    }

    fn normal(&self, seg: &Segment, s: f64, p: &Vector3<f64>, rd: &Vector3<f64>) -> Vector3<f64> {
        let dr = poly::eval(&poly::derivative(&seg.r), s);
        let dy = poly::eval(&poly::derivative(&seg.y), s);
        let rho = (p.x * p.x + p.z * p.z).sqrt();
        let radial = if rho > 0. { Vector3::new(p.x / rho, 0., p.z / rho) } else { Vector3::zeros() };
        // A negative radius is the same circle, reached the other way round.
        let radial = radial * poly::eval(&seg.r, s).signum();
        let n = (radial * dy - Vector3::y() * dr) * self.orientation;
        if n.norm() > 0. { n.normalize() } else { -rd }
    }
}

impl Geometry for Lathe {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let (entry, _) = self.bounds.entry_exit(&r.ro, &inv)?;
        let start = self.bounds.solve_start(entry);
        let o = r.ro + rd * start;
        let dxz2 = rd.x * rd.x + rd.z * rd.z;

        let mut best: Option<(f64, &Segment, f64)> = None;
        for seg in &self.segments {
            let ss = if rd.y.abs() > 1e-9 {
                let yo = poly::add(&seg.y, &[-o.y]);
                let a = poly::add(&[o.x * rd.y], &poly::scale(&yo, rd.x));
                let b = poly::add(&[o.z * rd.y], &poly::scale(&yo, rd.z));
                let g = poly::add(&poly::add(&poly::mul(&a, &a), &poly::mul(&b, &b)),
                    &poly::scale(&poly::mul(&seg.r, &seg.r), -rd.y * rd.y));
                poly::roots_in(&g, 0., 1.)
            } else {
                // A level ray: the heights where the profile is at the ray's level.
                poly::roots_in(&poly::add(&seg.y, &[-o.y]), 0., 1.)
            };
            for s in ss {
                let p = seg.at(s);
                // The ray meets the circle of radius p.x in the xz plane: of the (up to two)
                // crossings, the one at the profile's height.
                let ts = poly::quadratic_roots(o.x * o.x + o.z * o.z - p.x * p.x, 2. * (o.x * rd.x + o.z * rd.z), dxz2);
                let t = if rd.y.abs() > 1e-9 {
                    let ty = (p.y - o.y) / rd.y;
                    if ts.is_empty() { Some(ty) } else {
                        ts.into_iter().min_by(|a, b| (a - ty).abs().total_cmp(&(b - ty).abs()))
                    }
                } else {
                    ts.into_iter().find(|t| t + start > 1e-9)
                };
                if let Some(t) = t {
                    let dist = t + start;
                    if dist > 1e-9 && best.is_none_or(|(b, _, _)| dist < b) {
                        best = Some((dist, seg, s));
                    }
                }
            }
        }
        best.map(|(dist, seg, s)| {
            let point = r.ro + rd * dist;
            RawIntersection { face: 0, dist, point, normal: self.normal(seg, s, &point, &rd) }
        })
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.outline.contains(&P2::new((p.x * p.x + p.z * p.z).sqrt(), p.y))
    }
}

/// A surface of revolution: x^2 + z^2 = f(y), with f a cubic between each pair of points that
/// matches their squared radii, and slopes taken from the points either side (so, as in POV-Ray,
/// the first and last points only shape the ends). Heights must increase. Closed by flat caps at
/// the second and second-last points unless `open`.
pub struct Sor {
    /// Each piece: its range of heights, and f on it.
    pieces: Vec<(f64, f64, Vec<f64>)>,
    open: bool,
    bounds: BBox,
}

impl Sor {
    pub fn new(points: &[P2], open: bool) -> Sor {
        assert!(points.len() >= 4, "sor needs at least four points");
        assert!(points.windows(2).all(|w| w[1].y > w[0].y), "sor point heights must increase");
        let r2 = |p: &P2| p.x * p.x;
        let mut pieces = Vec::new();
        for w in points.windows(4) {
            let (y0, y1) = (w[1].y, w[2].y);
            let (f0, f1) = (r2(&w[1]), r2(&w[2]));
            let m0 = (r2(&w[2]) - r2(&w[0])) / (w[2].y - w[0].y);
            let m1 = (r2(&w[3]) - r2(&w[1])) / (w[3].y - w[1].y);
            // Cubic Hermite on [y0, y1] in u = (y - y0) / h, then in y.
            let h = y1 - y0;
            let in_u = [f0, m0 * h, -3. * f0 - 2. * m0 * h + 3. * f1 - m1 * h, 2. * f0 + m0 * h - 2. * f1 + m1 * h];
            let f = poly::compose(&in_u, &[-y0 / h, 1. / h]);
            pieces.push((y0, y1, f));
        }
        let mut rmax2 = 0f64;
        for (y0, y1, f) in &pieces {
            let mut ys = vec![*y0, *y1];
            ys.extend(poly::roots_in(&poly::derivative(f), *y0, *y1));
            for y in ys {
                rmax2 = rmax2.max(poly::eval(f, y));
            }
        }
        let rmax = rmax2.sqrt();
        let (ylo, yhi) = (pieces[0].0, pieces.last().unwrap().1);
        let bounds = BBox::new(Vector3::new(-rmax, ylo, -rmax), Vector3::new(rmax, yhi, rmax));
        Sor { pieces, open, bounds }
    }

    fn f_at(&self, y: f64) -> Option<&Vec<f64>> {
        self.pieces.iter().find(|(y0, y1, _)| y >= *y0 && y <= *y1).map(|(_, _, f)| f)
    }
}

impl Geometry for Sor {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let (entry, exit) = self.bounds.entry_exit(&r.ro, &inv)?;
        let start = self.bounds.solve_start(entry);
        let o = r.ro + rd * start;
        let xz = [o.x * o.x + o.z * o.z, 2. * (o.x * rd.x + o.z * rd.z), rd.x * rd.x + rd.z * rd.z];
        let len = exit - start;

        let mut best: Option<(f64, Vector3<f64>)> = None;
        let mut consider = |t: f64, n: Vector3<f64>| {
            if t + start > 1e-9 && best.is_none_or(|(b, _)| t < b) {
                best = Some((t, n));
            }
        };
        for (y0, y1, f) in &self.pieces {
            // The part of the ray within this piece's heights.
            let (mut lo, mut hi) = (0., len);
            if rd.y != 0. {
                let (ta, tb) = ((y0 - o.y) / rd.y, (y1 - o.y) / rd.y);
                lo = f64::max(lo, ta.min(tb));
                hi = f64::min(hi, ta.max(tb));
            } else if o.y < *y0 || o.y > *y1 {
                continue;
            }
            let g = poly::add(&xz, &poly::scale(&poly::compose(f, &[o.y, rd.y]), -1.));
            for t in poly::roots_in(&g, lo, hi) {
                let p = o + rd * t;
                let n = Vector3::new(2. * p.x, -poly::eval(&poly::derivative(f), p.y), 2. * p.z);
                consider(t, if n.norm() > 0. { n.normalize() } else { -rd });
            }
        }
        if !self.open && rd.y != 0. {
            let (ylo, yhi) = (self.pieces[0].0, self.pieces.last().unwrap().1);
            for (y, ny) in [(ylo, -1.), (yhi, 1.)] {
                let t = (y - o.y) / rd.y;
                let p = o + rd * t;
                if p.x * p.x + p.z * p.z <= poly::eval(self.f_at(y).unwrap(), y) {
                    consider(t, Vector3::new(0., ny, 0.));
                }
            }
        }
        best.map(|(t, normal)| {
            let dist = t + start;
            RawIntersection { face: 0, dist, point: r.ro + rd * dist, normal }
        })
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        !self.open && self.f_at(p.y).is_some_and(|f| p.x * p.x + p.z * p.z < poly::eval(f, p.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }
    fn p(x: f64, y: f64) -> P2 { P2::new(x, y) }

    /// A closed square profile from r = 1 to 2, y = 0 to 1: a thick-walled tube.
    fn tube(reverse: bool) -> Lathe {
        let mut pts = vec![p(1., 0.), p(2., 0.), p(2., 1.), p(1., 1.), p(1., 0.)];
        if reverse { pts.reverse(); }
        Lathe::new(&pts, Spline::Linear)
    }

    #[test]
    fn test_linear_lathe_tube() {
        for reverse in [false, true] {
            let l = tube(reverse);
            let side = l.intersects(&Ray { ro: v(-5., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
            assert!((side.dist - 3.).abs() < 1e-9);
            assert!((side.normal - v(-1., 0., 0.)).norm() < 1e-9, "{}", side.normal);
            // Inner wall from the axis faces inwards.
            let inner = l.intersects(&Ray { ro: v(0., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
            assert!((inner.dist - 1.).abs() < 1e-9);
            assert!((inner.normal - v(-1., 0., 0.)).norm() < 1e-9);
            // Top, down.
            let top = l.intersects(&Ray { ro: v(1.5, 5., 0.), rd: v(0., -1., 0.) }).unwrap();
            assert!((top.dist - 4.).abs() < 1e-9);
            assert!((top.normal - v(0., 1., 0.)).norm() < 1e-9);
            // Down the middle: nothing.
            assert!(l.intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., -1., 0.) }).is_none());
            assert!(l.inside(&v(0., 0.5, 1.5)) && !l.inside(&v(0., 0.5, 0.5)));
        }
    }

    #[test]
    fn test_oblique_ray_on_lathe_cylinder() {
        let l = tube(false);
        let rd = v(1., 0.3, 0.2).normalize();
        let ro = v(-5., -1., -0.5);
        let i = l.intersects(&Ray { ro, rd }).unwrap();
        let rho = (i.point.x * i.point.x + i.point.z * i.point.z).sqrt();
        assert!((rho - 2.).abs() < 1e-7 || (i.point.y.abs() < 1e-7), "{}", i.point);
        assert!(((i.point - ro).norm() - i.dist).abs() < 1e-9);
    }

    #[test]
    fn test_bezier_lathe_sphere_like() {
        // A Bezier half circle of radius 1 from the bottom of the axis to the top.
        let k = 0.5523;
        let l = Lathe::new(&[p(0., -1.), p(k, -1.), p(1., -k), p(1., 0.), p(1., 0.), p(1., k), p(k, 1.), p(0., 1.)], Spline::Bezier);
        let i = l.intersects(&Ray { ro: v(0., 0., -5.), rd: v(0., 0., 1.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-9);
        assert!((i.normal - v(0., 0., -1.)).norm() < 1e-6);
        let top = l.intersects(&Ray { ro: v(0.01, 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((top.dist - 4.).abs() < 1e-3);
        assert!(l.bounds().max.y >= 1. - 1e-9 && l.bounds().max.x >= 1. - 1e-9);
    }

    #[test]
    fn test_sor_cylinder() {
        // Constant radius 1 from y = 0 to 2: a cylinder.
        let s = Sor::new(&[p(1., -1.), p(1., 0.), p(1., 2.), p(1., 3.)], false);
        let side = s.intersects(&Ray { ro: v(-5., 1., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((side.dist - 4.).abs() < 1e-9);
        assert!((side.normal - v(-1., 0., 0.)).norm() < 1e-9);
        let cap = s.intersects(&Ray { ro: v(0.5, 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((cap.dist - 3.).abs() < 1e-9);
        assert_eq!(cap.normal, v(0., 1., 0.));
        let open = Sor::new(&[p(1., -1.), p(1., 0.), p(1., 2.), p(1., 3.)], true);
        let inside_wall = open.intersects(&Ray { ro: v(0.5, 5., 0.), rd: v(0., -1., 0.) });
        assert!(inside_wall.is_none());
        assert!(s.inside(&v(0., 1., 0.)) && !s.inside(&v(0., 2.5, 0.)));
    }

    #[test]
    fn test_sor_passes_through_points() {
        let pts = [p(0.5, -1.), p(1., 0.), p(2., 1.), p(1.5, 2.), p(1., 3.), p(1., 4.)];
        let s = Sor::new(&pts, false);
        for q in &pts[1..5] {
            let f = s.f_at(q.y).unwrap();
            assert!((poly::eval(f, q.y) - q.x * q.x).abs() < 1e-9);
        }
        // A ray at the height of a point meets the surface at that point's radius.
        let i = s.intersects(&Ray { ro: v(-5., 1., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.point.x + 2.).abs() < 1e-9);
    }
}
