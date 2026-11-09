//! Surfaces given implicitly by a polynomial in x, y and z, f(x, y, z) = 0: the torus, and
//! general algebraic surfaces like POV-Ray's `quadric`, `cubic`, `quartic` and `poly` (POV-Ray
//! 3.7 reference, sections 2.4.1.12 "Torus", 2.4.3.4 "Poly, Cubic and Quartic" and 2.4.3.5
//! "Quadric").
//!
//! Along a ray o + t d each coordinate is linear in t, so f becomes a polynomial in t whose
//! smallest positive root is the hit. The surface normal is the gradient of f, which points out of
//! the solid where f < 0 (POV-Ray's convention for inside).
//!
//! To keep the polynomial well conditioned, the ray origin is first moved to where the ray enters
//! the shape's bounding box: a far-off origin makes the coefficients large and the roots
//! imprecise (see Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., section
//! 6.8.2 on the related problem for quadrics).

use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polynomial as poly;

/// Furthest along a ray that an unbounded surface is searched for.
const FAR: f64 = 1e7;

/// The nearest hit within `bounds`: the smallest positive root of the polynomial in t that
/// `along(origin, d)` gives for the ray from `origin` in unit direction `d`, with the normal from
/// `gradient`.
fn nearest_root<P, G>(r: &Ray, bounds: &BBox, along: P, gradient: G) -> Option<RawIntersection>
where
    P: Fn(&Vector3<f64>, &Vector3<f64>) -> Vec<f64>,
    G: Fn(&Vector3<f64>) -> Vector3<f64>,
{
    let rd = r.rd.normalize();
    let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
    let (entry, exit) = bounds.entry_exit(&r.ro, &inv)?;
    let start = bounds.solve_start(entry).min(FAR);
    let end = exit.min(FAR);
    if end <= start {
        return None;
    }
    let origin = r.ro + rd * start;
    let c = along(&origin, &rd);
    let eps = 1e-9 * (1. + start);
    poly::roots_in(&c, 0., end - start)
        .into_iter()
        .map(|t| t + start)
        .find(|t| *t > eps)
        .map(|dist| {
            let point = r.ro + rd * dist;
            let g = gradient(&point);
            let normal = if g.norm() > 0. { g.normalize() } else { -rd };
            RawIntersection { dist, point, normal }
        })
}

/// A torus about the y axis, centred on the origin: the circle of radius `minor` swept round the
/// circle of radius `major` in the xz plane. Implicitly,
/// (x^2 + y^2 + z^2 + R^2 - r^2)^2 - 4 R^2 (x^2 + z^2) = 0, a quartic.
pub struct Torus {
    major: f64,
    minor: f64,
}

impl Torus {
    pub fn new(major: f64, minor: f64) -> Torus {
        assert!(major > 0. && minor > 0., "torus radii must be positive");
        Torus { major, minor }
    }

    fn eval(&self, p: &Vector3<f64>) -> f64 {
        let r2 = self.major * self.major;
        let s = p.norm_squared() + r2 - self.minor * self.minor;
        s * s - 4. * r2 * (p.x * p.x + p.z * p.z)
    }
}

impl Geometry for Torus {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let r2 = self.major * self.major;
        let k = r2 - self.minor * self.minor;
        nearest_root(r, &self.bounds(), |o, d| {
            // |o + t d|^2 + k, and x^2 + z^2, as polynomials in t.
            let s = [o.norm_squared() + k, 2. * o.dot(d), d.norm_squared()];
            let xz = [o.x * o.x + o.z * o.z, 2. * (o.x * d.x + o.z * d.z), d.x * d.x + d.z * d.z];
            poly::add(&poly::mul(&s, &s), &poly::scale(&xz, -4. * r2))
        }, |p| {
            let s = p.norm_squared() + k;
            p * (4. * s) - Vector3::new(p.x, 0., p.z) * (8. * r2)
        })
    }

    fn bounds(&self) -> BBox {
        let (a, b) = (self.major + self.minor, self.minor);
        BBox::new(Vector3::new(-a, -b, -a), Vector3::new(a, b, a))
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.eval(p) < 0.
    }

    fn surface_area(&self) -> f64 {
        4. * std::f64::consts::PI * std::f64::consts::PI * self.major * self.minor
    }
}

/// One term c x^i y^j z^k of a polynomial in three variables.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term {
    pub coefficient: f64,
    pub powers: [usize; 3],
}

/// The surface f(x, y, z) = 0 of any polynomial f, clipped to a box (unbounded by default).
pub struct Algebraic {
    terms: Vec<Term>,
    bounds: BBox,
}

impl Algebraic {
    pub fn new(terms: Vec<Term>, bounds: Option<BBox>) -> Algebraic {
        let terms: Vec<Term> = terms.into_iter().filter(|t| t.coefficient != 0.).collect();
        assert!(!terms.is_empty(), "polynomial surface has no non-zero terms");
        let bounds = bounds.unwrap_or_else(|| BBox::new(
            Vector3::new(f64::MIN, f64::MIN, f64::MIN), Vector3::new(f64::MAX, f64::MAX, f64::MAX)));
        Algebraic { terms, bounds }
    }

    /// The terms of a polynomial of the given order from coefficients listed in POV-Ray's order:
    /// descending powers of x, then of y within each, then of z within those, every term of total
    /// degree up to `order` - so for order 2: x^2, xy, xz, x, y^2, yz, y, z^2, z, 1.
    pub fn pov_terms(order: usize, coefficients: &[f64]) -> Vec<Term> {
        let mut powers = Vec::new();
        for i in (0..=order).rev() {
            for j in (0..=order - i).rev() {
                for k in (0..=order - i - j).rev() {
                    powers.push([i, j, k]);
                }
            }
        }
        assert_eq!(coefficients.len(), powers.len(),
            "a polynomial of order {} needs {} coefficients, got {}", order, powers.len(), coefficients.len());
        powers.into_iter().zip(coefficients).map(|(powers, &coefficient)| Term { coefficient, powers }).collect()
    }

    /// POV-Ray's `quadric`: A x^2 + B y^2 + C z^2 + D xy + E xz + F yz + G x + H y + I z + J.
    pub fn quadric_terms(squares: Vector3<f64>, mixed: Vector3<f64>, linear: Vector3<f64>, constant: f64) -> Vec<Term> {
        let t = |coefficient, powers| Term { coefficient, powers };
        vec![
            t(squares.x, [2, 0, 0]), t(squares.y, [0, 2, 0]), t(squares.z, [0, 0, 2]),
            t(mixed.x, [1, 1, 0]), t(mixed.y, [1, 0, 1]), t(mixed.z, [0, 1, 1]),
            t(linear.x, [1, 0, 0]), t(linear.y, [0, 1, 0]), t(linear.z, [0, 0, 1]),
            t(constant, [0, 0, 0]),
        ]
    }

    fn order(&self) -> usize {
        self.terms.iter().map(|t| t.powers.iter().sum::<usize>()).max().unwrap_or(0)
    }

    pub fn eval(&self, p: &Vector3<f64>) -> f64 {
        self.terms.iter().map(|t| {
            t.coefficient * p.x.powi(t.powers[0] as i32) * p.y.powi(t.powers[1] as i32) * p.z.powi(t.powers[2] as i32)
        }).sum()
    }

    pub fn gradient(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let mut g = Vector3::zeros();
        for t in &self.terms {
            for axis in 0..3 {
                let n = t.powers[axis];
                if n == 0 { continue; }
                let mut v = t.coefficient * n as f64;
                for (a, &m) in t.powers.iter().enumerate() {
                    let e = if a == axis { m - 1 } else { m };
                    v *= p[a].powi(e as i32);
                }
                g[axis] += v;
            }
        }
        g
    }
}

impl Geometry for Algebraic {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let n = self.order();
        nearest_root(r, &self.bounds, |o, d| {
            // Powers of each coordinate as polynomials in t.
            let pows: Vec<Vec<Vec<f64>>> = (0..3).map(|a| {
                let mut v = vec![vec![1.]];
                for i in 1..=n {
                    let next = poly::mul(&v[i - 1], &[o[a], d[a]]);
                    v.push(next);
                }
                v
            }).collect();
            self.terms.iter().fold(vec![], |acc, t| {
                let m = poly::mul(&poly::mul(&pows[0][t.powers[0]], &pows[1][t.powers[1]]), &pows[2][t.powers[2]]);
                poly::add(&acc, &poly::scale(&m, t.coefficient))
            })
        }, |p| self.gradient(p))
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.eval(p) < 0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    #[test]
    fn test_torus_hits_outer_and_hole() {
        let t = Torus::new(2., 0.5);
        let i = t.intersects(&Ray { ro: v(-10., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.dist - 7.5).abs() < 1e-7, "{}", i.dist);
        assert!((i.normal - v(-1., 0., 0.)).norm() < 1e-7);
        // Down through the hole.
        assert!(t.intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., -1., 0.) }).is_none());
        // Down onto the tube.
        let top = t.intersects(&Ray { ro: v(0., 5., 2.), rd: v(0., -1., 0.) }).unwrap();
        assert!((top.dist - 4.5).abs() < 1e-7);
        assert!((top.normal - v(0., 1., 0.)).norm() < 1e-7);
        // From inside the tube.
        let inner = t.intersects(&Ray { ro: v(2., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((inner.dist - 0.5).abs() < 1e-7);
        assert!(t.inside(&v(2., 0., 0.)) && !t.inside(&v(0., 0., 0.)));
    }

    #[test]
    fn test_pov_term_order() {
        let terms = Algebraic::pov_terms(2, &[1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        let powers: Vec<[usize; 3]> = terms.iter().map(|t| t.powers).collect();
        assert_eq!(powers, vec![[2, 0, 0], [1, 1, 0], [1, 0, 1], [1, 0, 0], [0, 2, 0], [0, 1, 1],
            [0, 1, 0], [0, 0, 2], [0, 0, 1], [0, 0, 0]]);
        assert_eq!(Algebraic::pov_terms(4, &[0.; 35]).len(), 35);
    }

    #[test]
    fn test_quadric_sphere_matches_sphere() {
        // x^2 + y^2 + z^2 - 1
        let s = Algebraic::new(Algebraic::quadric_terms(v(1., 1., 1.), v(0., 0., 0.), v(0., 0., 0.), -1.), None);
        let i = s.intersects(&Ray { ro: v(0.3, 0.2, -5.), rd: v(0., 0., 1.) }).unwrap();
        let expected = 5. - (1f64 - 0.09 - 0.04).sqrt();
        assert!((i.dist - expected).abs() < 1e-7);
        assert!((i.normal - i.point.normalize()).norm() < 1e-7);
        assert!(s.inside(&v(0., 0., 0.)));
    }

    #[test]
    fn test_unbounded_and_clipped_cylinder() {
        // x^2 + z^2 - 1: an infinite cylinder along y.
        let terms = Algebraic::quadric_terms(v(1., 0., 1.), v(0., 0., 0.), v(0., 0., 0.), -1.);
        let infinite = Algebraic::new(terms.clone(), None);
        assert!(infinite.intersects(&Ray { ro: v(-5., 1000., 0.), rd: v(1., 0., 0.) }).is_some());
        let clipped = Algebraic::new(terms, Some(BBox::new(v(-2., -1., -2.), v(2., 1., 2.))));
        assert!(clipped.intersects(&Ray { ro: v(-5., 1000., 0.), rd: v(1., 0., 0.) }).is_none());
        let i = clipped.intersects(&Ray { ro: v(-5., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-7);
    }

    #[test]
    fn test_gradient_matches_finite_difference() {
        let a = Algebraic::new(Algebraic::pov_terms(3, &(0..20).map(|i| (i as f64 * 0.37).sin()).collect::<Vec<_>>()), None);
        let p = v(0.3, -0.7, 1.1);
        let h = 1e-6;
        let g = a.gradient(&p);
        for axis in 0..3 {
            let mut e = Vector3::zeros();
            e[axis] = h;
            let fd = (a.eval(&(p + e)) - a.eval(&(p - e))) / (2. * h);
            assert!((g[axis] - fd).abs() < 1e-6);
        }
    }
}
