//! The quaternion Julia set: points q whose orbit under q <- q^2 + c stays bounded, sliced to 3D
//! by fixing the fourth component (POV-Ray 3.7 reference: "Julia Fractal", with
//! its default `quaternion` algebra and `sqr` function).
//!
//! The set is found by ray marching with a distance estimate. If the orbit escapes, the distance
//! to the set is at least about |q| ln|q| / (2 |q'|), with q' the orbit's derivative, which obeys
//! q' <- 2 q q' (John C. Hart, Daniel J. Sandin and Louis H. Kauffman, "Ray tracing deterministic
//! 3-D fractals", SIGGRAPH 1989; Inigo Quilez, "3D Julia sets",
//! <https://iquilezles.org/articles/juliasets3d/>). Points that do not escape within the
//! iteration limit count as in the set, so more iterations give finer detail.

use crate::shapes::geometry::Geometry;
use crate::na::{Vector3, Vector4};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polynomial::quadratic_roots;

pub struct Julia {
    /// The constant c, as (real, i, j, k).
    c: Vector4<f64>,
    /// The fourth (k) component of every point of the 3D slice.
    slice: f64,
    max_iteration: usize,
    /// How near the distance estimate must come to count as a hit.
    accuracy: f64,
    /// A sphere about the origin that holds the whole set.
    radius: f64,
}

/// The quaternion product.
fn qmul(a: &Vector4<f64>, b: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(
        a.x * b.x - a.y * b.y - a.z * b.z - a.w * b.w,
        a.x * b.y + a.y * b.x + a.z * b.w - a.w * b.z,
        a.x * b.z - a.y * b.w + a.z * b.x + a.w * b.y,
        a.x * b.w + a.y * b.z - a.z * b.y + a.w * b.x)
}

const ESCAPE: f64 = 1e4;

impl Julia {
    pub fn new(c: Vector4<f64>, slice: f64, max_iteration: usize, accuracy: f64) -> Julia {
        assert!(max_iteration > 0 && accuracy > 0., "julia_fractal needs positive max_iteration and accuracy");
        // Beyond |q| = R with R^2 - R >= |c|, |q^2 + c| >= |q|^2 - |c| > |q|: the orbit escapes.
        // With a margin, so that the set doesn't touch the bounding sphere.
        let radius = 0.5 * (1. + (1. + 4. * c.norm()).sqrt()) * 1.05;
        Julia { c, slice, max_iteration, accuracy, radius }
    }

    /// A lower bound on the distance from p to the set (0 within it).
    pub fn distance(&self, p: &Vector3<f64>) -> f64 {
        let mut q = Vector4::new(p.x, p.y, p.z, self.slice);
        let mut dq = Vector4::new(1., 0., 0., 0.);
        for _ in 0..self.max_iteration {
            dq = qmul(&q, &dq) * 2.;
            q = qmul(&q, &q) + self.c;
            if q.norm_squared() > ESCAPE {
                break;
            }
        }
        let (m, dm) = (q.norm(), dq.norm());
        if m * m <= ESCAPE || dm == 0. {
            return 0.;
        }
        0.5 * m * m.ln() / dm
    }
}

impl Geometry for Julia {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let (a, b) = match quadratic_roots(r.ro.norm_squared() - self.radius * self.radius, 2. * r.ro.dot(&rd), 1.)[..] {
            [a, b] if b > 0. => (a, b),
            _ => return None,
        };
        let mut t = a.max(0.);
        // Starting inside the set, march out of it first.
        while a <= 0. && t < b && self.distance(&(r.ro + rd * t)) < self.accuracy {
            t += self.accuracy;
        }
        let mut steps = 0;
        while t < b && steps < 10_000 {
            let p = r.ro + rd * t;
            let d = self.distance(&p);
            if d < self.accuracy {
                let h = self.accuracy;
                let n = Vector3::new(
                    self.distance(&(p + Vector3::x() * h)) - self.distance(&(p - Vector3::x() * h)),
                    self.distance(&(p + Vector3::y() * h)) - self.distance(&(p - Vector3::y() * h)),
                    self.distance(&(p + Vector3::z() * h)) - self.distance(&(p - Vector3::z() * h)));
                let normal = if n.norm() > 0. { n.normalize() } else { -rd };
                return Some(RawIntersection { face: 0, dist: t, point: p, normal });
            }
            t += d;
            steps += 1;
        }
        None
    }

    fn bounds(&self) -> BBox {
        let r = Vector3::new(self.radius, self.radius, self.radius);
        BBox::new(-r, r)
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.distance(p) == 0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_zero_is_the_unit_ball() {
        // With c = 0 the orbit q^(2^n) stays bounded exactly when |q| <= 1.
        let j = Julia::new(Vector4::zeros(), 0., 30, 1e-5);
        let i = j.intersects(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-3, "{}", i.dist);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-2, "{}", i.normal);
        assert!(j.inside(&Vector3::new(0.5, 0., 0.)) && !j.inside(&Vector3::new(1.5, 0., 0.)));
    }

    #[test]
    fn test_distance_is_a_lower_bound() {
        let j = Julia::new(Vector4::new(-0.4, 0.6, 0.2, 0.), 0., 20, 1e-4);
        // Wherever the estimate says the set is at least d away, it isn't within d along a line.
        for k in 0..20 {
            let p = Vector3::new(1.5 - k as f64 * 0.05, 0.3, -0.2);
            let d = j.distance(&p);
            if d > 0. {
                assert!(j.distance(&(p + Vector3::new(-1., 0., 0.) * d * 0.99)) >= 0.);
                assert!(!j.inside(&(p + Vector3::new(-1., 0., 0.) * d * 0.5)), "at {}", p);
            }
        }
        assert!(j.intersects(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) }).is_some());
    }

    #[test]
    fn test_qmul() {
        let (i, jj) = (Vector4::new(0., 1., 0., 0.), Vector4::new(0., 0., 1., 0.));
        assert_eq!(qmul(&i, &jj), Vector4::new(0., 0., 0., 1.)); // ij = k
        assert_eq!(qmul(&jj, &i), Vector4::new(0., 0., 0., -1.));
        assert_eq!(qmul(&i, &i), Vector4::new(-1., 0., 0., 0.));
    }
}
