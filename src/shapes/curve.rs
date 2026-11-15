//! # Curves
//!
//! Hair, fur and grass are far too thin and numerous to model with triangles. Each strand is a
//! cubic Bézier curve with a width that varies linearly along it (Pharr, Jakob and Humphreys,
//! "Physically Based Rendering", 4th ed., section 6.7, "Curves"; cited below as PBR, which this
//! follows):
//!
//! - `Flat`: a ribbon that always turns to face the ray: cheap, for strands too thin to see
//!   round.
//! - `Cylinder`: the same ribbon, but shaded as if round: its normal turns across the width.
//! - `Ribbon`: a strip oriented by a normal at each end, interpolated spherically between them
//!   (eg. blades of grass), which narrows as it turns edge-on.
//!
//! ## Intersection (PBR section 6.7.2)
//!
//! The curve's control points are taken into a frame in which the ray starts at the origin and
//! runs along +z. There, the curve is hit if some point of it lies within half its width of the
//! z axis. The curve is split in two, recursively, discarding halves whose control points'
//! bounds (widened by half the width) miss the axis, until each piece is nearly straight: the
//! depth needed follows from how far the control polygon bends (its second differences) and a
//! tolerance of a twentieth of the width. A piece is then treated as a segment: the ray's
//! closest approach to it gives u along the curve, and the hit if it is within half the width
//! there, in front of the ray's origin. v runs across the width, from 0 to 1.
//!
//! Long curves are split into segments, each its sub-curve by blossoming, and the segments
//! of all the curves held in a BVH.
use crate::bvh::Bvh;
use crate::intersection::RawIntersection;
use crate::material::phase::orthonormal_basis;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurveKind {
    Flat,
    Cylinder,
    /// With the normals at the start and end.
    Ribbon(Vector3<f64>, Vector3<f64>),
}

/// One strand: a cubic Bézier curve and its width at each end.
#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    pub points: [Vector3<f64>; 4],
    pub width: [f64; 2],
    pub kind: CurveKind,
}

fn lerp(t: f64, a: Vector3<f64>, b: Vector3<f64>) -> Vector3<f64> {
    a * (1. - t) + b * t
}

/// The point of the cubic Bézier `cp` at `u`, and its derivative there (PBR's
/// `EvaluateCubicBezier`).
pub fn evaluate(cp: &[Vector3<f64>; 4], u: f64) -> (Vector3<f64>, Vector3<f64>) {
    let a = [lerp(u, cp[0], cp[1]), lerp(u, cp[1], cp[2]), lerp(u, cp[2], cp[3])];
    let b = [lerp(u, a[0], a[1]), lerp(u, a[1], a[2])];
    let d = b[1] - b[0];
    let deriv = if d.norm_squared() > 0. { d * 3. } else { cp[3] - cp[0] };
    (lerp(u, b[0], b[1]), deriv)
}

/// The blossom of `cp` at (u0, u1, u2) (PBR's `BlossomCubicBezier`): with three equal arguments
/// a point of the curve, and in general a control point of a sub-curve.
fn blossom(cp: &[Vector3<f64>; 4], u0: f64, u1: f64, u2: f64) -> Vector3<f64> {
    let a = [lerp(u0, cp[0], cp[1]), lerp(u0, cp[1], cp[2]), lerp(u0, cp[2], cp[3])];
    let b = [lerp(u1, a[0], a[1]), lerp(u1, a[1], a[2])];
    lerp(u2, b[0], b[1])
}

/// The control points of the part of `cp` from `u0` to `u1`.
pub fn sub_curve(cp: &[Vector3<f64>; 4], u0: f64, u1: f64) -> [Vector3<f64>; 4] {
    [blossom(cp, u0, u0, u0), blossom(cp, u0, u0, u1), blossom(cp, u0, u1, u1), blossom(cp, u1, u1, u1)]
}

/// The two halves of `cp`, sharing the middle point (PBR's `SubdivideCubicBezier`).
fn subdivide(cp: &[Vector3<f64>; 4]) -> [Vector3<f64>; 7] {
    [
        cp[0],
        (cp[0] + cp[1]) / 2.,
        (cp[0] + cp[1] * 2. + cp[2]) / 4.,
        (cp[0] + cp[1] * 3. + cp[2] * 3. + cp[3]) / 8.,
        (cp[1] + cp[2] * 2. + cp[3]) / 4.,
        (cp[2] + cp[3]) / 2.,
        cp[3],
    ]
}

/// A part of a curve, `u0` to `u1`, for the BVH.
#[derive(Debug, Clone, Copy)]
struct Segment {
    curve: u32,
    u0: f64,
    u1: f64,
}

/// A hit: the distance along the (unit) ray, u along the curve and v across it.
struct Hit {
    t: f64,
    u: f64,
    v: f64,
}

/// Many curves, split into segments in a BVH.
pub struct Curves {
    curves: Vec<Curve>,
    segments: Vec<Segment>,
    bvh: Bvh,
}

impl Curves {
    /// `curves`, each split into `2^split` segments.
    pub fn new(curves: Vec<Curve>, split: u32) -> Curves {
        let n = 1usize << split;
        let segments: Vec<Segment> = curves.iter().enumerate().flat_map(|(c, _)| {
            (0..n).map(move |i| Segment { curve: c as u32, u0: i as f64 / n as f64, u1: (i + 1) as f64 / n as f64 })
        }).collect();
        let (bvh, segments) = Bvh::build(segments, |s| Curves::segment_bounds(&curves[s.curve as usize], s));
        Curves { curves, segments, bvh }
    }

    fn width_at(c: &Curve, u: f64) -> f64 {
        c.width[0] * (1. - u) + c.width[1] * u
    }

    fn segment_bounds(c: &Curve, s: &Segment) -> BBox {
        let cp = sub_curve(&c.points, s.u0, s.u1);
        let w = Curves::width_at(c, s.u0).max(Curves::width_at(c, s.u1)) / 2.;
        let lo = cp.iter().fold(Vector3::repeat(f64::MAX), |a, p| a.inf(p)) - Vector3::repeat(w);
        let hi = cp.iter().fold(Vector3::repeat(f64::MIN), |a, p| a.sup(p)) + Vector3::repeat(w);
        BBox::new(lo, hi)
    }

    /// The ray's frame: origin, and unit axes with z along the ray.
    fn frame(r: &Ray, cp: &[Vector3<f64>; 4]) -> (Vector3<f64>, [Vector3<f64>; 3]) {
        let z = r.rd.normalize();
        // An x axis across the curve keeps its bounds in the frame tight (as pbrt's LookAt).
        let x = z.cross(&(cp[3] - cp[0]));
        let (x, y) = if x.norm_squared() > 1e-300 {
            let x = x.normalize();
            (x, z.cross(&x))
        } else {
            orthonormal_basis(&z)
        };
        (r.ro, [x, y, z])
    }

    /// Test segment `s` against the ray, between `min` and `max` along it.
    fn hit_segment(&self, r: &Ray, s: &Segment, min: f64, max: f64) -> Option<Hit> {
        let c = &self.curves[s.curve as usize];
        let cp = sub_curve(&c.points, s.u0, s.u1);
        let (o, [x, y, z]) = Curves::frame(r, &cp);
        let to_ray = |p: &Vector3<f64>| { let d = p - o; Vector3::new(d.dot(&x), d.dot(&y), d.dot(&z)) };
        let cpr = cp.map(|p| to_ray(&p));
        // How far the control polygon bends sets how often to split (PBR 6.7.2).
        let l0 = (0..2).map(|i| (cpr[i] - cpr[i + 1] * 2. + cpr[i + 2]).abs().max()).fold(0., f64::max);
        let eps = c.width[0].max(c.width[1]) * 0.05;
        let r0 = if l0 > 0. && eps > 0. { ((std::f64::consts::SQRT_2 * 6. * l0 / (8. * eps)).log2().floor() as i64) / 2 } else { 0 };
        let depth = r0.clamp(0, 10) as u32;
        let mut best: Option<Hit> = None;
        self.recurse(c, &cpr, &z, s.u0, s.u1, depth, min, max, &mut best);
        best
    }

    #[allow(clippy::too_many_arguments)]
    fn recurse(&self, c: &Curve, cp: &[Vector3<f64>; 4], ray: &Vector3<f64>, u0: f64, u1: f64, depth: u32, min: f64, max: f64, best: &mut Option<Hit>) {
        let max_width = Curves::width_at(c, u0).max(Curves::width_at(c, u1));
        let half = 0.5 * max_width;
        let limit = best.as_ref().map_or(max, |b| b.t.min(max));
        let lo = cp.iter().fold(Vector3::repeat(f64::MAX), |a, p| a.inf(p));
        let hi = cp.iter().fold(Vector3::repeat(f64::MIN), |a, p| a.sup(p));
        if hi.x + half < 0. || lo.x - half > 0. || hi.y + half < 0. || lo.y - half > 0. || hi.z + half < min || lo.z - half > limit {
            return;
        }
        if depth > 0 {
            let split = subdivide(cp);
            let um = 0.5 * (u0 + u1);
            self.recurse(c, &[split[0], split[1], split[2], split[3]], ray, u0, um, depth - 1, min, max, best);
            self.recurse(c, &[split[3], split[4], split[5], split[6]], ray, um, u1, depth - 1, min, max, best);
            return;
        }
        // The ray must pass between the planes perpendicular to the segment at its ends.
        let edge = (cp[1].y - cp[0].y) * -cp[0].y + cp[0].x * (cp[0].x - cp[1].x);
        if edge < 0. {
            return;
        }
        let edge = (cp[2].y - cp[3].y) * -cp[3].y + cp[3].x * (cp[3].x - cp[2].x);
        if edge < 0. {
            return;
        }
        // The closest approach to the axis along the chord, as a fraction w of it.
        let dir = Vector2::new(cp[3].x - cp[0].x, cp[3].y - cp[0].y);
        let denom = dir.norm_squared();
        if denom == 0. {
            return;
        }
        let w = (-Vector2::new(cp[0].x, cp[0].y)).dot(&dir) / denom;
        let u = (u0 + (u1 - u0) * w).clamp(u0, u1);
        let mut width = Curves::width_at(c, u);
        if let CurveKind::Ribbon(n0, n1) = c.kind {
            // Narrower as the ribbon turns edge-on to the ray.
            width *= slerp(n0, n1, u).dot(ray).abs();
        }
        let (pc, deriv) = evaluate(cp, w.clamp(0., 1.));
        let d2 = pc.x * pc.x + pc.y * pc.y;
        if d2 > width * width * 0.25 || pc.z < min || pc.z > limit {
            return;
        }
        let dist = d2.sqrt();
        let side = deriv.x * -pc.y + pc.x * deriv.y;
        let v = if side > 0. { 0.5 + dist / width.max(1e-300) } else { 0.5 - dist / width.max(1e-300) };
        *best = Some(Hit { t: pc.z, u, v });
    }

    /// The normal at a hit, in world space, facing back along the ray.
    fn normal(&self, r: &Ray, curve: u32, hit: &Hit) -> Vector3<f64> {
        let c = &self.curves[curve as usize];
        let (_, tangent) = evaluate(&c.points, hit.u);
        let tangent = tangent.try_normalize(1e-300).unwrap_or_else(Vector3::x);
        let toward = -r.rd.normalize();
        // Facing the ray, across the tangent.
        let facing = (toward - tangent * toward.dot(&tangent)).try_normalize(1e-300).unwrap_or(toward);
        match c.kind {
            CurveKind::Flat => facing,
            CurveKind::Cylinder => {
                // Across the width, round as a cylinder: from the side at the edges to facing the
                // ray in the middle (PBR rotates dpdv by an angle from -90° to 90° across v).
                let side = tangent.cross(&facing);
                let o = (2. * hit.v - 1.).clamp(-1., 1.);
                (facing * (1. - o * o).max(0.).sqrt() + side * o).normalize()
            },
            CurveKind::Ribbon(n0, n1) => {
                let n = slerp(n0, n1, hit.u);
                if n.dot(&toward) < 0. { -n } else { n }
            },
        }
    }
}

/// Spherical interpolation between unit vectors.
fn slerp(a: Vector3<f64>, b: Vector3<f64>, t: f64) -> Vector3<f64> {
    let cos = a.dot(&b).clamp(-1., 1.);
    let angle = cos.acos();
    if angle < 1e-9 {
        return a;
    }
    ((a * ((1. - t) * angle).sin() + b * (t * angle).sin()) / angle.sin()).normalize()
}

impl Geometry for Curves {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        let scale = r.rd.norm();
        let unit = Ray { ro: r.ro, rd: r.rd / scale };
        let mut found: Option<(u32, Hit)> = None;
        self.bvh.nearest(&unit, min, max, |i, _| {
            let s = &self.segments[i];
            let limit = found.as_ref().map_or(max, |(_, h)| h.t);
            let h = self.hit_segment(&unit, s, min, limit)?;
            let point = unit.ro + unit.rd * h.t;
            let out = RawIntersection { face: s.curve, part: 0, dist: h.t, point, normal: Vector3::zeros() };
            found = Some((s.curve, h));
            Some(out)
        })?;
        let (curve, h) = found?;
        Some(RawIntersection { face: curve, part: 0, dist: h.t, point: unit.ro + unit.rd * h.t, normal: self.normal(&unit, curve, &h) })
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        let unit = Ray { ro: r.ro, rd: r.rd.normalize() };
        self.bvh.any_hit(&unit, min, max, |i| self.hit_segment(&unit, &self.segments[i], min, max).is_some())
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.segments.len() as u64
    }

    /// u along the curve, found by projecting the point onto it; v, without the ray, is the
    /// middle of the width.
    fn uv(&self, p: &Vector3<f64>, face: u32) -> Option<Vector2<f64>> {
        let c = self.curves.get(face as usize)?;
        let mut best = (f64::MAX, 0.);
        for i in 0..=32 {
            let u = i as f64 / 32.;
            let d = (evaluate(&c.points, u).0 - p).norm_squared();
            if d < best.0 { best = (d, u); }
        }
        let mut u = best.1;
        for _ in 0..8 {
            let (q, d) = evaluate(&c.points, u);
            let step = (q - p).dot(&d) / d.norm_squared().max(1e-300);
            u = (u - step).clamp(0., 1.);
        }
        Some(Vector2::new(u, 0.5))
    }
}

/// A ball of fur: `count` strands growing from a sphere at `center` of `radius`, each `length`
/// long, from `width[0]` at the root to `width[1]` at the tip, leaning out from the surface and
/// drooping under gravity (`droop`, the tip's fall as a fraction of the length), with `curl`
/// (random sideways bend, a fraction of the length). Roots are spread evenly over the sphere (a
/// Fibonacci spiral), and the rest varied by `seed`.
#[allow(clippy::too_many_arguments)]
pub fn fur(center: Vector3<f64>, radius: f64, count: usize, length: f64, width: [f64; 2], droop: f64, curl: f64, kind: CurveKind, seed: u64) -> Vec<Curve> {
    let mut rng = crate::geometry::Pcg32::new(seed, 0x6675_72);
    let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
    (0..count).map(|i| {
        let yy = 1. - 2. * (i as f64 + 0.5) / count as f64;
        let rr = (1. - yy * yy).sqrt();
        let n = Vector3::new(rr * (golden * i as f64).cos(), yy, rr * (golden * i as f64).sin());
        let root = center + n * radius;
        let l = length * (0.8 + 0.4 * rng.next_f64());
        let random = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5) * (2. * curl * l);
        let down = Vector3::new(0., -droop * l, 0.);
        let p1 = root + n * (l / 3.);
        let p2 = root + n * (2. * l / 3.) + random * 0.5 + down * 0.4;
        let p3 = root + n * l + random + down;
        Curve { points: [root, p1, p2, p3], width, kind }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight(kind: CurveKind) -> Curves {
        // Along x from 0 to 3, 0.2 wide.
        let c = Curve { points: [Vector3::zeros(), Vector3::x(), Vector3::x() * 2., Vector3::x() * 3.], width: [0.2, 0.2], kind };
        Curves::new(vec![c], 2)
    }

    #[test]
    fn test_bezier_pieces_agree() {
        let cp = [Vector3::zeros(), Vector3::new(1., 2., 0.), Vector3::new(3., -1., 1.), Vector3::new(4., 0., 2.)];
        // A sub-curve's ends are the curve's points there, and its halves meet at the middle.
        let sub = sub_curve(&cp, 0.25, 0.75);
        assert!((sub[0] - evaluate(&cp, 0.25).0).norm() < 1e-12 && (sub[3] - evaluate(&cp, 0.75).0).norm() < 1e-12);
        let halves = subdivide(&cp);
        assert!((halves[3] - evaluate(&cp, 0.5).0).norm() < 1e-12);
        assert!((evaluate(&[halves[0], halves[1], halves[2], halves[3]], 0.5).0 - evaluate(&cp, 0.25).0).norm() < 1e-12);
    }

    #[test]
    fn test_straight_curve_hits_within_its_width() {
        let c = straight(CurveKind::Flat);
        let down = |x: f64, z: f64| Ray { ro: Vector3::new(x, 5., z), rd: Vector3::new(0., -1., 0.) };
        let hit = c.intersects(&down(1.5, 0.05)).unwrap();
        assert!((hit.dist - 5.).abs() < 1e-9);
        assert!((hit.normal - Vector3::y()).norm() < 1e-9, "{}", hit.normal);
        assert!(c.intersects(&down(1.5, 0.15)).is_none());
        assert!(c.intersects(&down(3.5, 0.)).is_none());
        let uv = c.uv(&hit.point, hit.face).unwrap();
        assert!((uv.x - 0.5).abs() < 1e-6, "{}", uv);
        assert!(c.occludes(&down(1.5, 0.), 0., 10.) && !c.occludes(&down(1.5, 0.), 0., 4.));
    }

    #[test]
    fn test_cylinder_normal_turns_across_the_width() {
        let c = straight(CurveKind::Cylinder);
        let down = |z: f64| Ray { ro: Vector3::new(1.5, 5., z), rd: Vector3::new(0., -1., 0.) };
        let middle = c.intersects(&down(0.)).unwrap().normal;
        let edge = c.intersects(&down(0.095)).unwrap().normal;
        assert!(middle.y > 0.99, "{}", middle);
        assert!(edge.z.abs() > 0.9, "{}", edge);
    }

    #[test]
    fn test_curved_strand_is_found_by_its_points() {
        // A bent strand: rays aimed at points along it, from the side, hit it there.
        let cp = [Vector3::zeros(), Vector3::new(1., 1., 0.), Vector3::new(2., -1., 0.), Vector3::new(3., 0.5, 0.)];
        let c = Curves::new(vec![Curve { points: cp, width: [0.05, 0.01], kind: CurveKind::Cylinder }], 3);
        for i in 1..20 {
            let u = i as f64 / 20.;
            let p = evaluate(&cp, u).0;
            let hit = c.intersects(&Ray { ro: p + Vector3::new(0., 0., 4.), rd: Vector3::new(0., 0., -1.) }).unwrap();
            assert!((hit.dist - 4.).abs() < 1e-3, "u {}: {}", u, hit.dist);
        }
    }

    #[test]
    fn test_fur_grows_outwards() {
        let strands = fur(Vector3::zeros(), 1., 100, 0.3, [0.01, 0.002], 0.2, 0.1, CurveKind::Cylinder, 1);
        assert_eq!(strands.len(), 100);
        for s in &strands {
            assert!((s.points[0].norm() - 1.).abs() < 1e-9);
            assert!(s.points[3].norm() > 1.1);
        }
    }
}
