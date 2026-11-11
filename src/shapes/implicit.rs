//! Surfaces found by searching along the ray for where a function changes sign: the `isosurface`,
//! for any function, and the `superellipsoid` (POV-Ray 3.7 reference:
//! "Isosurface" and "Superellipsoid").
//!
//! The isosurface steps along the ray by the function's distance from the threshold divided by
//! its largest gradient (`max_gradient`): it cannot cross the surface in less than that, so no
//! crossing is missed if the bound is right (John C. Hart, "Sphere tracing: a geometric method for
//! the antialiased ray tracing of implicit surfaces", The Visual Computer 12(10), 1996). A sign
//! change between steps is then narrowed down by bisection. An underestimated gradient shows as
//! holes, as in POV-Ray.

use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polynomial::quadratic_roots;
use crate::expr::Expr;

/// The region an implicit surface is searched for in. Where the surface would extend beyond it,
/// the container's own surface closes the solid (unless the shape is open).
#[derive(Debug, Clone, Copy)]
pub enum Container {
    Box(BBox),
    Sphere { center: Vector3<f64>, radius: f64 },
}

impl Container {
    /// Where the line o + t d (unit d) is within the container, as an interval of t.
    fn span(&self, o: &Vector3<f64>, d: &Vector3<f64>) -> Option<(f64, f64)> {
        match self {
            Container::Box(b) => b.entry_exit(o, &Vector3::new(1. / d.x, 1. / d.y, 1. / d.z)),
            Container::Sphere { center, radius } => {
                let oc = o - center;
                match quadratic_roots(oc.norm_squared() - radius * radius, 2. * oc.dot(d), 1.)[..] {
                    [a, b] if b >= 0. => Some((a, b)),
                    _ => None,
                }
            },
        }
    }

    /// Outward normal of the container's surface at p.
    fn normal(&self, p: &Vector3<f64>) -> Vector3<f64> {
        match self {
            Container::Sphere { center, .. } => (p - center).normalize(),
            Container::Box(b) => {
                let (mut nearest, mut n) = (f64::INFINITY, Vector3::zeros());
                for a in 0..3 {
                    for (face, s) in [(b.min[a], -1.), (b.max[a], 1.)] {
                        if (p[a] - face).abs() < nearest {
                            nearest = (p[a] - face).abs();
                            n = Vector3::zeros();
                            n[a] = s;
                        }
                    }
                }
                n
            },
        }
    }

    fn bounds(&self) -> BBox {
        match self {
            Container::Box(b) => *b,
            Container::Sphere { center, radius } => {
                let r = Vector3::new(*radius, *radius, *radius);
                BBox::new(center - r, center + r)
            },
        }
    }

    fn contains(&self, p: &Vector3<f64>) -> bool {
        match self {
            Container::Box(b) => b.contains_point(p),
            Container::Sphere { center, radius } => (p - center).norm() < *radius,
        }
    }
}

/// The first t in [t0, t1] where g changes sign from its sign at t0, stepping by `step(g)` and
/// then bisecting to within `accuracy`.
fn march<G, S>(g: G, t0: f64, t1: f64, accuracy: f64, step: S) -> Option<f64>
where
    G: Fn(f64) -> f64,
    S: Fn(f64) -> f64,
{
    let mut t = t0;
    let mut gt = g(t);
    let outside = gt >= 0.;
    while t < t1 {
        let next = (t + step(gt).max(accuracy)).min(t1);
        let gn = g(next);
        if (gn >= 0.) != outside {
            let (mut lo, mut hi) = (t, next);
            while hi - lo > accuracy * 0.01 {
                let mid = 0.5 * (lo + hi);
                if (g(mid) >= 0.) == outside { lo = mid; } else { hi = mid; }
            }
            return Some(0.5 * (lo + hi));
        }
        if next >= t1 {
            break;
        }
        t = next;
        gt = gn;
    }
    None
}

/// Find the surface of a solid that is inside where `g` < 0, within `container`: the first sign
/// change along the ray; or the container's surface where the solid is cut by it, unless `open`.
fn implicit_hit<G>(r: &Ray, container: &Container, open: bool, accuracy: f64, g: G, step: impl Fn(f64) -> f64) -> Option<RawIntersection>
where
    G: Fn(&Vector3<f64>) -> f64,
{
    let rd = r.rd.normalize();
    let (entry, exit) = container.span(&r.ro, &rd)?;
    let t0 = entry.max(0.);
    if exit <= t0 {
        return None;
    }
    let at = |t: f64| r.ro + rd * t;
    let eps = accuracy * 0.5;
    let hit = |dist: f64, normal: Vector3<f64>| Some(RawIntersection { face: 0, part: 0, dist, point: at(dist), normal });

    // Entering the container where the solid fills it: the container's surface.
    if !open && entry > eps && g(&at(entry)) < 0. {
        return hit(entry, container.normal(&at(entry)));
    }
    let start = if entry > 0. { entry } else { eps };
    let starts_inside = g(&at(start)) < 0.;
    match march(|t| g(&at(t)), start, exit, accuracy, step) {
        Some(t) => {
            let p = at(t);
            let h = accuracy * 0.1;
            let grad = Vector3::new(
                g(&(p + Vector3::x() * h)) - g(&(p - Vector3::x() * h)),
                g(&(p + Vector3::y() * h)) - g(&(p - Vector3::y() * h)),
                g(&(p + Vector3::z() * h)) - g(&(p - Vector3::z() * h)));
            hit(t, if grad.norm() > 0. { grad.normalize() } else { -rd })
        },
        // Leaving the container from within the solid.
        None if starts_inside && !open => hit(exit, container.normal(&at(exit))),
        None => None,
    }
}

/// The surface where `function` of (x, y, z) equals `threshold`; inside where it is less.
pub struct Isosurface {
    function: Expr,
    threshold: f64,
    container: Container,
    max_gradient: f64,
    accuracy: f64,
    open: bool,
}

impl Isosurface {
    pub fn new(function: Expr, threshold: f64, container: Container, max_gradient: f64, accuracy: f64, open: bool) -> Isosurface {
        assert!(max_gradient > 0., "isosurface max_gradient must be positive");
        assert!(accuracy > 0., "isosurface accuracy must be positive");
        Isosurface { function, threshold, container, max_gradient, accuracy, open }
    }

    fn g(&self, p: &Vector3<f64>) -> f64 {
        self.function.eval(&[p.x, p.y, p.z]) - self.threshold
    }
}

impl Geometry for Isosurface {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        implicit_hit(r, &self.container, self.open, self.accuracy, |p| self.g(p), |g| g.abs() / self.max_gradient)
    }

    fn bounds(&self) -> BBox {
        self.container.bounds()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.container.contains(p) && self.g(p) < 0.
    }
}

/// POV-Ray's superellipsoid within the cube from -1 to 1:
/// (|x|^(2/e) + |y|^(2/e))^(e/n) + |z|^(2/n) = 1. `e` shapes the cross-section in xy and `n`
/// the profile along z: 1 is round, towards 0 square, 2 straight-sided like an octahedron, and
/// above 2 pinched.
pub struct Superellipsoid {
    e: f64,
    n: f64,
}

/// Steps across the superellipsoid's cube: it has no gradient bound, so it is sampled.
const SUPERELLIPSOID_STEPS: f64 = 200.;

impl Superellipsoid {
    pub fn new(e: f64, n: f64) -> Superellipsoid {
        assert!(e > 0. && n > 0., "superellipsoid exponents must be positive");
        Superellipsoid { e, n }
    }

    fn g(&self, p: &Vector3<f64>) -> f64 {
        let (e, n) = (self.e, self.n);
        (p.x.abs().powf(2. / e) + p.y.abs().powf(2. / e)).powf(e / n) + p.z.abs().powf(2. / n) - 1.
    }
}

impl Geometry for Superellipsoid {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let cube = Container::Box(self.bounds());
        implicit_hit(r, &cube, true, 1e-6, |p| self.g(p), |_| 2. * 3f64.sqrt() / SUPERELLIPSOID_STEPS)
    }

    fn bounds(&self) -> BBox {
        BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.))
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.g(p) < 0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    fn sphere_iso(open: bool, container: Container) -> Isosurface {
        Isosurface::new(Expr::parse("sqrt(x*x + y*y + z*z) - 1", &["x", "y", "z"]).unwrap(), 0., container, 1., 1e-6, open)
    }

    fn big_box() -> Container {
        Container::Box(BBox::new(v(-2., -2., -2.), v(2., 2., 2.)))
    }

    #[test]
    fn test_isosurface_sphere() {
        let s = sphere_iso(false, big_box());
        let i = s.intersects(&Ray { ro: v(0., 0., -5.), rd: v(0., 0., 1.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-5, "{}", i.dist);
        assert!((i.normal - v(0., 0., -1.)).norm() < 1e-4);
        let out = s.intersects(&Ray { ro: v(0., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((out.dist - 1.).abs() < 1e-5);
        assert!(s.intersects(&Ray { ro: v(0., 1.5, -5.), rd: v(0., 0., 1.) }).is_none());
    }

    #[test]
    fn test_container_cuts_the_solid() {
        // A sphere of radius 1 in a box reaching only to z = 0.5: its flat face is the box's.
        let c = Container::Box(BBox::new(v(-2., -2., -2.), v(2., 2., 0.5)));
        let closed = sphere_iso(false, c);
        let i = closed.intersects(&Ray { ro: v(0., 0., 5.), rd: v(0., 0., -1.) }).unwrap();
        assert!((i.dist - 4.5).abs() < 1e-9);
        assert_eq!(i.normal, v(0., 0., 1.));
        // Open, the ray passes in to the far side of the sphere.
        let open = sphere_iso(true, c);
        let j = open.intersects(&Ray { ro: v(0., 0., 5.), rd: v(0., 0., -1.) }).unwrap();
        assert!((j.dist - 6.).abs() < 1e-5, "{}", j.dist);
    }

    #[test]
    fn test_sphere_container() {
        let s = sphere_iso(false, Container::Sphere { center: v(0., 0., 0.), radius: 3. });
        let i = s.intersects(&Ray { ro: v(0., 0., -5.), rd: v(0., 0., 1.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-5);
    }

    #[test]
    fn test_superellipsoid() {
        // e = n = 1 is the unit sphere.
        let s = Superellipsoid::new(1., 1.);
        let i = s.intersects(&Ray { ro: v(0.3, 0.2, -5.), rd: v(0., 0., 1.) }).unwrap();
        let expected = 5. - (1f64 - 0.09 - 0.04).sqrt();
        assert!((i.dist - expected).abs() < 1e-5);
        assert!((i.normal - i.point.normalize()).norm() < 1e-3);
        // Small exponents approach the cube.
        let cube = Superellipsoid::new(0.1, 0.1);
        let j = cube.intersects(&Ray { ro: v(0.8, 0.8, -5.), rd: v(0., 0., 1.) }).unwrap();
        assert!((j.dist - 4.).abs() < 0.05, "{}", j.dist);
        assert!((j.normal - v(0., 0., -1.)).norm() < 0.05);
        assert!(cube.inside(&v(0.9, 0.9, 0.)) && !s.inside(&v(0.9, 0.9, 0.)));
    }
}
