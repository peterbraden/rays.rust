//! The `sphere_sweep`: the volume a sphere of varying radius sweeps out along a path (POV-Ray 3.7
//! reference, "Sphere Sweep").
//!
//! Between two spheres the sweep is bounded by the cone that touches both, so a linear sweep is
//! exactly the union of its spheres and those cones. For spheres with centres d apart and radii
//! r1, r2, the cone's side makes an angle a with the axis where sin a = (r1 - r2) / d (its slant
//! line is tangent to both circles in any plane through the axis), so it touches each sphere of
//! radius r on a circle offset r sin a along the axis from the centre, of radius r cos a. Curved
//! sweeps (`cubic` through the points, or `b_spline` near them) are sampled into many short
//! linear pieces, where POV-Ray solves them directly.
//!
//! The visible surface of a union is where a component's surface is not inside any other.

use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::cone::Cone;
use crate::shapes::sphere::Sphere;

/// How a sweep's path runs through its points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SweepSpline {
    /// Straight between consecutive points.
    Linear,
    /// A Catmull-Rom curve through every point but the first and last, which only shape the ends.
    Cubic,
    /// A uniform cubic B-spline, which passes near rather than through the points.
    BSpline,
}

/// One sphere or touching cone of a sweep, as a solid.
enum Part {
    Ball(Sphere),
    Frustum { cone: Cone, base: Vector3<f64>, axis: Vector3<f64>, height: f64, r0: f64, r1: f64 },
}

impl Part {
    fn geometry(&self) -> &dyn Geometry {
        match self {
            Part::Ball(s) => s,
            Part::Frustum { cone, .. } => cone,
        }
    }

    /// Strictly inside the solid (the cone's ends count as closed: they lie inside the spheres).
    fn contains(&self, p: &Vector3<f64>, eps: f64) -> bool {
        match self {
            Part::Ball(s) => (p - s.center).norm() < s.radius - eps,
            Part::Frustum { base, axis, height, r0, r1, .. } => {
                let s = (p - base).dot(axis);
                let rho = ((p - base) - axis * s).norm();
                s > eps && s < height - eps && rho < r0 + (r1 - r0) * s / height - eps
            },
        }
    }
}

pub struct SphereSweep {
    parts: Vec<(Part, BBox)>,
    bounds: BBox,
}

impl SphereSweep {
    /// `spheres` are (centre, radius). Curved splines are sampled `steps` times per segment.
    pub fn new(spheres: &[(Vector3<f64>, f64)], spline: SweepSpline, steps: usize) -> SphereSweep {
        let path = match spline {
            SweepSpline::Linear => {
                assert!(spheres.len() >= 2, "a linear sphere sweep needs at least two spheres");
                spheres.to_vec()
            },
            SweepSpline::Cubic | SweepSpline::BSpline => {
                assert!(spheres.len() >= 4, "a curved sphere sweep needs at least four spheres");
                let m = if spline == SweepSpline::Cubic {
                    [[0., 1., 0., 0.], [-0.5, 0., 0.5, 0.], [1., -2.5, 2., -0.5], [-0.5, 1.5, -1.5, 0.5]]
                } else {
                    let k = 1. / 6.;
                    [[k, 4. * k, k, 0.], [-0.5, 0., 0.5, 0.], [0.5, -1., 0.5, 0.], [-k, 0.5, -0.5, k]]
                };
                let mut path = Vec::new();
                for w in spheres.windows(4) {
                    let first = if path.is_empty() { 0 } else { 1 };
                    for i in first..=steps {
                        let s = i as f64 / steps as f64;
                        let basis = [1., s, s * s, s * s * s];
                        let weight = |j: usize| (0..4).map(|k| m[k][j] * basis[k]).sum::<f64>();
                        let c = (0..4).fold(Vector3::zeros(), |acc, j| acc + w[j].0 * weight(j));
                        let r = (0..4).map(|j| w[j].1 * weight(j)).sum::<f64>();
                        path.push((c, r.max(0.)));
                    }
                }
                path
            },
        };

        let mut parts = Vec::new();
        for &(c, r) in &path {
            if r > 0. {
                parts.push(Part::Ball(Sphere::new(c, r)));
            }
        }
        for w in path.windows(2) {
            let ((c0, r0), (c1, r1)) = (w[0], w[1]);
            let d = (c1 - c0).norm();
            if d <= (r0 - r1).abs() {
                continue; // One sphere holds the other.
            }
            let axis = (c1 - c0) / d;
            let sin = (r0 - r1) / d;
            let cos = (1. - sin * sin).sqrt();
            let (base, cap) = (c0 + axis * (r0 * sin), c1 + axis * (r1 * sin));
            let (b0, b1) = (r0 * cos, r1 * cos);
            let height = (cap - base).dot(&axis);
            if height <= 0. {
                continue;
            }
            parts.push(Part::Frustum { cone: Cone::new(base, b0, cap, b1, true), base, axis, height, r0: b0, r1: b1 });
        }
        let parts: Vec<(Part, BBox)> = parts.into_iter().map(|p| { let b = p.geometry().bounds(); (p, b) }).collect();
        let bounds = parts.iter().fold(BBox::empty(), |b, (_, pb)| b.union(pb));
        SphereSweep { parts, bounds }
    }
}

impl Geometry for SphereSweep {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let eps = 1e-9 * (1. + self.bounds.size().norm());
        let mut best: Option<RawIntersection> = None;
        for (i, (part, pb)) in self.parts.iter().enumerate() {
            if pb.entry_exit(&r.ro, &inv).is_none() {
                continue;
            }
            // A sphere or open cone is crossed at most twice.
            let mut from = 0.;
            for _ in 0..2 {
                let Some(hit) = part.geometry().intersects_beyond(&Ray { ro: r.ro, rd }, from) else { break };
                if best.is_some_and(|b| hit.dist >= b.dist) {
                    break;
                }
                let hidden = self.parts.iter().enumerate()
                    .any(|(j, (other, _))| j != i && other.contains(&hit.point, eps));
                if !hidden {
                    best = Some(hit);
                    break;
                }
                from = hit.dist + eps;
            }
        }
        best
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.parts.len() as u64
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.parts.iter().any(|(part, _)| part.contains(p, 0.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    #[test]
    fn test_equal_radii_is_a_capsule() {
        let s = SphereSweep::new(&[(v(0., 0., 0.), 1.), (v(4., 0., 0.), 1.)], SweepSpline::Linear, 1);
        // The middle: a cylinder.
        let mid = s.intersects(&Ray { ro: v(2., 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((mid.dist - 4.).abs() < 1e-9);
        assert!((mid.normal - v(0., 1., 0.)).norm() < 1e-9);
        // The rounded end.
        let end = s.intersects(&Ray { ro: v(-5., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((end.dist - 4.).abs() < 1e-9);
        // Along the axis from inside, out through the far end - not stopping at internal surfaces.
        let out = s.intersects(&Ray { ro: v(0., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((out.dist - 5.).abs() < 1e-9, "{}", out.dist);
        assert!(s.inside(&v(2., 0.5, 0.)) && !s.inside(&v(2., 1.5, 0.)));
    }

    #[test]
    fn test_tapered_sweep_is_tangent() {
        // From radius 2 to radius 1: the surface between is the touching cone, whose normal is
        // continuous with the spheres' where they meet.
        let s = SphereSweep::new(&[(v(0., 0., 0.), 2.), (v(5., 0., 0.), 1.)], SweepSpline::Linear, 1);
        let sin: f64 = (2. - 1.) / 5.;
        let cos = (1. - sin * sin).sqrt();
        // The touching circle on the first sphere: at x = 2 sin, radius 2 cos.
        let i = s.intersects(&Ray { ro: v(2. * sin, 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((i.point.y - 2. * cos).abs() < 1e-6, "{}", i.point);
        assert!((i.normal - v(sin, cos, 0.)).norm() < 1e-6, "{}", i.normal);
    }

    #[test]
    fn test_cubic_sweep_passes_through_inner_points() {
        let pts = [(v(-1., 0., 0.), 0.5), (v(0., 0., 0.), 0.5), (v(2., 1., 0.), 0.5), (v(3., 1., 0.), 0.5)];
        let s = SphereSweep::new(&pts, SweepSpline::Cubic, 8);
        // Straight up onto the second point's sphere, from below, where the path curves away.
        let i = s.intersects(&Ray { ro: v(0., -5., 0.), rd: v(0., 1., 0.) }).unwrap();
        assert!((i.point.y + 0.5).abs() < 1e-6, "{}", i.point);
        assert!(s.inside(&v(2., 1., 0.)));
        assert!(!s.inside(&v(-1., 0., 0.))); // The first point only shapes the start.
    }
}
