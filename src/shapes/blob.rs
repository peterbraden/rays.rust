//! Blobs: the surface where a sum of smooth field components reaches a threshold, so that nearby
//! components merge (POV-Ray 3.7 reference, section 2.4.1.2 "Blob").
//!
//! As in POV-Ray, each component of radius r and strength s contributes s (1 - d^2 / r^2)^2 at
//! distance d < r from it, and nothing beyond. A sphere component measures d from its centre; a
//! cylinder component from its axis between the ends, and from the end points beyond them (so it
//! has rounded ends). Negative strengths carve away. Inside is where the total exceeds the
//! threshold.
//!
//! Along a ray, d^2 is a quadratic in the distance t within each component's region, so its
//! field is a quartic there. Splitting the ray where regions begin and end leaves intervals on
//! which the total is a single quartic, solved exactly.

use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::cone::disc_bounds;
use crate::shapes::polynomial as poly;

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Sphere { center: Vector3<f64>, radius: f64, strength: f64 },
    Cylinder { base: Vector3<f64>, cap: Vector3<f64>, radius: f64, strength: f64 },
}

/// A region of the ray in which one component's field is s (1 - q(t) / r^2)^2.
struct Piece {
    from: f64,
    to: f64,
    /// d^2 along the ray, as a quadratic.
    q: [f64; 3],
    radius: f64,
    strength: f64,
}

/// d^2 = |o + t d - c|^2 for unit d, as a quadratic in t.
fn dist2_to_point(o: &Vector3<f64>, d: &Vector3<f64>, c: &Vector3<f64>) -> [f64; 3] {
    let oc = o - c;
    [oc.norm_squared(), 2. * oc.dot(d), 1.]
}

/// Where a quadratic is below `limit`, intersected with [lo, hi].
fn below(q: &[f64; 3], limit: f64, lo: f64, hi: f64) -> Option<(f64, f64)> {
    let r = poly::quadratic_roots(q[0] - limit, q[1], q[2]);
    if r.len() < 2 {
        return None;
    }
    let (a, b) = (r[0].max(lo), r[1].min(hi));
    if a < b { Some((a, b)) } else { None }
}

/// Where a linear function a + b t lies in [lo_v, hi_v], as an interval of t.
fn linear_within(a: f64, b: f64, lo_v: f64, hi_v: f64) -> (f64, f64) {
    if b == 0. {
        return if a >= lo_v && a <= hi_v { (f64::NEG_INFINITY, f64::INFINITY) } else { (1., 0.) };
    }
    let (t0, t1) = ((lo_v - a) / b, (hi_v - a) / b);
    (t0.min(t1), t0.max(t1))
}

impl Component {
    fn radius(&self) -> f64 {
        match self { Component::Sphere { radius, .. } | Component::Cylinder { radius, .. } => *radius }
    }

    fn strength(&self) -> f64 {
        match self { Component::Sphere { strength, .. } | Component::Cylinder { strength, .. } => *strength }
    }

    fn bounds(&self) -> BBox {
        let r = self.radius();
        let rv = Vector3::new(r, r, r);
        match self {
            Component::Sphere { center, .. } => BBox::new(center - rv, center + rv),
            Component::Cylinder { base, cap, .. } => BBox::new(base - rv, base + rv)
                .union(&BBox::new(cap - rv, cap + rv))
                .union(&disc_bounds(*base, (cap - base).normalize(), r)),
        }
    }

    /// The vector from the nearest point of the component's core (centre or axis) to p.
    fn offset(&self, p: &Vector3<f64>) -> Vector3<f64> {
        match self {
            Component::Sphere { center, .. } => p - center,
            Component::Cylinder { base, cap, .. } => {
                let axis = cap - base;
                let s = ((p - base).dot(&axis) / axis.norm_squared()).clamp(0., 1.);
                p - (base + axis * s)
            },
        }
    }

    fn field(&self, p: &Vector3<f64>) -> f64 {
        let r2 = self.radius() * self.radius();
        let d2 = self.offset(p).norm_squared();
        if d2 >= r2 { 0. } else { self.strength() * (1. - d2 / r2).powi(2) }
    }

    /// The gradient of the field.
    fn gradient(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let r2 = self.radius() * self.radius();
        let v = self.offset(p);
        let d2 = v.norm_squared();
        if d2 >= r2 { Vector3::zeros() } else { v * (-4. * self.strength() / r2 * (1. - d2 / r2)) }
    }

    /// The regions of the ray (o + t d, t in [0, len]) where this component has a field.
    fn pieces(&self, o: &Vector3<f64>, d: &Vector3<f64>, len: f64, out: &mut Vec<Piece>) {
        let (radius, strength) = (self.radius(), self.strength());
        let r2 = radius * radius;
        let mut push = |q: [f64; 3], lo: f64, hi: f64| {
            if let Some((from, to)) = below(&q, r2, lo.max(0.), hi.min(len)) {
                out.push(Piece { from, to, q, radius, strength });
            }
        };
        match self {
            Component::Sphere { center, .. } => push(dist2_to_point(o, d, center), 0., len),
            Component::Cylinder { base, cap, .. } => {
                let h = (cap - base).norm();
                let w = (cap - base) / h;
                let (s0, ds) = ((o - base).dot(&w), d.dot(&w));
                // Beside the axis: squared distance from it.
                let (op, dp) = ((o - base) - w * s0, d - w * ds);
                let side = [op.norm_squared(), 2. * op.dot(&dp), dp.norm_squared()];
                let (a, b) = linear_within(s0, ds, 0., h);
                push(side, a, b);
                // Beyond each end: distance from the end point.
                let (a, b) = linear_within(s0, ds, f64::NEG_INFINITY, 0.);
                push(dist2_to_point(o, d, base), a, b);
                let (a, b) = linear_within(s0, ds, h, f64::INFINITY);
                push(dist2_to_point(o, d, cap), a, b);
            },
        }
    }
}

pub struct Blob {
    components: Vec<Component>,
    threshold: f64,
    bounds: BBox,
}

impl Blob {
    pub fn new(components: Vec<Component>, threshold: f64) -> Blob {
        assert!(threshold > 0., "blob threshold must be positive");
        assert!(components.iter().all(|c| c.radius() > 0.), "blob component radii must be positive");
        // Only components that add to the field can raise it to the threshold.
        let bounds = components.iter().filter(|c| c.strength() > 0.)
            .fold(BBox::empty(), |b, c| b.union(&c.bounds()));
        Blob { components, threshold, bounds }
    }

    pub fn density(&self, p: &Vector3<f64>) -> f64 {
        self.components.iter().map(|c| c.field(p)).sum()
    }
}

impl Geometry for Blob {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let (entry, exit) = self.bounds.entry_exit(&r.ro, &inv)?;
        let start = self.bounds.solve_start(entry);
        let o = r.ro + rd * start;
        let len = exit - start;

        let mut pieces = Vec::new();
        for c in &self.components {
            c.pieces(&o, &rd, len, &mut pieces);
        }
        let mut cuts: Vec<f64> = pieces.iter().flat_map(|p| [p.from, p.to]).collect();
        cuts.sort_by(f64::total_cmp);
        cuts.dedup();
        for w in cuts.windows(2) {
            let (a, b) = (w[0], w[1]);
            let mid = 0.5 * (a + b);
            // The total field over this interval, less the threshold.
            let mut f = vec![-self.threshold];
            for p in pieces.iter().filter(|p| p.from <= mid && p.to >= mid) {
                let u = poly::add(&[1.], &poly::scale(&p.q, -1. / (p.radius * p.radius)));
                f = poly::add(&f, &poly::scale(&poly::mul(&u, &u), p.strength));
            }
            if f.len() == 1 {
                continue;
            }
            if let Some(t) = poly::roots_in(&f, a, b).into_iter().find(|t| t + start > 1e-9) {
                let dist = t + start;
                let point = r.ro + rd * dist;
                let g: Vector3<f64> = self.components.iter().map(|c| c.gradient(&point)).sum();
                // The field falls outwards.
                let normal = if g.norm() > 0. { -g.normalize() } else { -rd };
                return Some(RawIntersection { dist, point, normal });
            }
        }
        None
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.components.len() as u64
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.density(p) > self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    #[test]
    fn test_single_component_is_a_sphere() {
        // s (1 - d^2/r^2)^2 = t gives d = r sqrt(1 - sqrt(t / s)).
        let b = Blob::new(vec![Component::Sphere { center: v(0., 0., 0.), radius: 2., strength: 1. }], 0.25);
        let expected = 2. * (1f64 - 0.5).sqrt();
        let i = b.intersects(&Ray { ro: v(0., 0., -10.), rd: v(0., 0., 1.) }).unwrap();
        assert!((i.dist - (10. - expected)).abs() < 1e-7, "{}", i.dist);
        assert!((i.normal - v(0., 0., -1.)).norm() < 1e-7);
        assert!(b.inside(&v(0., 0., 0.)));
        // From inside, out.
        let out = b.intersects(&Ray { ro: v(0., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((out.dist - expected).abs() < 1e-7);
    }

    #[test]
    fn test_components_merge() {
        // Two spheres whose separate surfaces wouldn't meet join in the middle.
        let c = |x| Component::Sphere { center: v(x, 0., 0.), radius: 1.5, strength: 1. };
        let b = Blob::new(vec![c(-1.), c(1.)], 0.6);
        let alone = Blob::new(vec![c(-1.)], 0.6);
        assert!(!alone.inside(&v(0., 0., 0.)));
        assert!(b.inside(&v(0., 0., 0.)));
        let i = b.intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((b.density(&i.point) - 0.6).abs() < 1e-7);
        assert!((i.normal - v(0., 1., 0.)).norm() < 1e-7);
    }

    #[test]
    fn test_negative_component_carves() {
        let b = Blob::new(vec![
            Component::Sphere { center: v(0., 0., 0.), radius: 2., strength: 1. },
            Component::Sphere { center: v(0., 0., -1.5), radius: 1., strength: -2. },
        ], 0.25);
        let i = b.intersects(&Ray { ro: v(0., 0., -10.), rd: v(0., 0., 1.) }).unwrap();
        // Deeper than the uncarved surface at z = -sqrt(2), and on the threshold.
        assert!(i.point.z > -1.2, "{}", i.point);
        assert!((b.density(&i.point) - 0.25).abs() < 1e-7);
    }

    #[test]
    fn test_cylinder_component() {
        let b = Blob::new(vec![Component::Cylinder { base: v(0., 0., 0.), cap: v(4., 0., 0.), radius: 1., strength: 1. }], 0.25);
        let d = (1f64 - 0.5).sqrt();
        // Beside the axis, and past the end.
        let side = b.intersects(&Ray { ro: v(2., 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((side.point.y - d).abs() < 1e-7);
        let end = b.intersects(&Ray { ro: v(10., 0., 0.), rd: v(-1., 0., 0.) }).unwrap();
        assert!((end.point.x - (4. + d)).abs() < 1e-7);
        // Diagonally onto the rounded end.
        let rd = v(-1., -1., 0.).normalize();
        let corner = b.intersects(&Ray { ro: v(10., 6., 0.), rd }).unwrap();
        assert!(((corner.point - v(4., 0., 0.)).norm() - d).abs() < 1e-7, "{}", corner.point);
    }
}
