//! A planet's shape: an oblate spheroid, flattened by its spin, and optionally a ring plane
//! about its equator, as one geometry (`part` 0 the planet, `RING_PART` the rings).
//!
//! A fluid planet spinning fast bulges at its equator: Saturn's polar radius is 9.8% shorter
//! than its equatorial one, Jupiter's 6.5%, plainly visible in a photograph. The spheroid is
//! solved as a sphere in coordinates where the polar axis is stretched to the equatorial
//! radius; its normal is the gradient of x²/a² + y²/c² + z²/a².
//!
//! At the distances these are seen from (10⁹ m, against radii of 10⁷ m) the quadratic is
//! solved by the perpendicular distance from the centre to the ray (Hearn and Baker's form, as
//! in Ray Tracing Gems chapter 7), not b² - c, which would lose most of its digits.

use crate::intersection::RawIntersection;
use crate::na::{Matrix3, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

/// The `part` of hits on the rings.
pub const RING_PART: u32 = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct Spheroid {
    pub centre: Vector3<f64>,
    /// Columns x, y, z: y is the pole.
    pub frame: Matrix3<f64>,
    /// Equatorial radius, metres.
    pub a: f64,
    /// Polar radius, metres.
    pub c: f64,
}

impl Spheroid {
    /// From scene to the body's frame, with the pole stretched so the spheroid is a sphere of
    /// radius `a`.
    fn to_sphere(&self, v: &Vector3<f64>) -> Vector3<f64> {
        let l = self.frame.transpose() * v;
        Vector3::new(l.x, l.y * self.a / self.c, l.z)
    }

    /// The nearest crossing beyond 0 along the ray (unit direction `d`), with the outward
    /// normal; and whether it is entering.
    pub fn hit(&self, ro: &Vector3<f64>, d: &Vector3<f64>) -> Option<(f64, Vector3<f64>)> {
        let o = self.to_sphere(&(ro - self.centre));
        let v = self.to_sphere(d);
        let vv = v.norm_squared();
        let b = o.dot(&v) / vv;
        // The closest approach to the centre, in the stretched space.
        let perp = o - v * b;
        let disc = (self.a * self.a - perp.norm_squared()) / vv;
        if disc < 0. {
            return None;
        }
        let s = disc.sqrt();
        let t = if -b - s > 0. { -b - s } else { -b + s };
        if t <= 0. {
            return None;
        }
        let p = ro + d * t;
        Some((t, self.normal(&p)))
    }

    /// The outward normal at `p` on the surface.
    pub fn normal(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let l = self.frame.transpose() * (p - self.centre);
        let g = Vector3::new(l.x / (self.a * self.a), l.y / (self.c * self.c), l.z / (self.a * self.a));
        (self.frame * g).normalize()
    }

    /// The point's unit direction in the body's frame, on the sphere the spheroid is stretched
    /// to (so its latitude is the planetocentric one of that sphere): what textures are indexed
    /// by.
    pub fn local(&self, p: &Vector3<f64>) -> Vector3<f64> {
        self.to_sphere(&(p - self.centre)).normalize()
    }

    /// Whether the ray from `p` towards unit `d` is blocked by the spheroid, softened over the
    /// sun's disc of angular radius `sun_radius`: the share of that disc left uncovered.
    pub fn uncovered(&self, p: &Vector3<f64>, d: &Vector3<f64>, sun_radius: f64) -> f64 {
        // In the stretched space the spheroid is a sphere; directions there are near enough
        // for the angles of a penumbra.
        let o = self.to_sphere(&(self.centre - p));
        let v = self.to_sphere(d).normalize();
        let dist = o.norm();
        if dist <= self.a {
            return 0.;
        }
        let r = (self.a / dist).asin();
        crate::celestial::uncovered(sun_radius, r, (o / dist).dot(&v).clamp(-1., 1.).acos())
    }
}

/// A spheroid and, optionally, a ring plane about its equator between `inner` and `outer`
/// (metres from the centre).
#[derive(Debug, Clone)]
pub struct Ringed {
    pub body: Spheroid,
    pub rings: Option<(f64, f64)>,
    /// The rings' optical depths: where they are empty, rays pass; and how much light gets
    /// through them, for shadows cast on other objects.
    pub profile: Option<std::sync::Arc<crate::celestial::rings::Rings>>,
}

impl Ringed {
    /// Where the ray from `ro` along unit `d` crosses the ring plane, if within the rings:
    /// the distance and the radius there.
    pub fn ring_hit(&self, ro: &Vector3<f64>, d: &Vector3<f64>) -> Option<(f64, f64)> {
        let (inner, outer) = self.rings?;
        let pole = self.body.frame.column(1).into_owned();
        let o = ro - self.body.centre;
        let dn = d.dot(&pole);
        if dn.abs() < 1e-15 {
            return None;
        }
        let t = -o.dot(&pole) / dn;
        if t <= 0. {
            return None;
        }
        let r = (o + d * t).norm();
        if r < inner || r > outer || self.profile.as_ref().is_some_and(|p| p.tau(r, 0.) <= 0.) {
            return None;
        }
        Some((t, r))
    }
}

impl Geometry for Ringed {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let d = r.rd.normalize();
        let planet = self.body.hit(&r.ro, &d);
        let ring = self.ring_hit(&r.ro, &d);
        match (planet, ring) {
            (Some((tp, _)), Some((tr, _))) if tr < tp => self.ring_intersection(r, &d, tr),
            (None, Some((tr, _))) => self.ring_intersection(r, &d, tr),
            (Some((tp, n)), _) => Some(RawIntersection { dist: tp, point: r.ro + d * tp, normal: n, face: 0, part: 0 }),
            _ => None,
        }
    }

    fn bounds(&self) -> BBox {
        let r = self.rings.map_or(self.body.a, |(_, outer)| outer.max(self.body.a));
        let e = Vector3::new(r, r, r);
        BBox::new(self.body.centre - e, self.body.centre + e)
    }

    /// Light gets through the rings as exp(-τ/μ); none through the planet.
    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        let d = r.rd.normalize();
        let ro = r.ro + d * min;
        if self.body.hit(&ro, &d).is_some_and(|(t, _)| t < max - min) {
            return 0.;
        }
        match (self.ring_hit(&ro, &d), &self.profile) {
            (Some((t, radius)), Some(p)) if t < max - min => {
                let mu = d.dot(&self.body.frame.column(1)).abs().max(1e-6);
                (-p.tau(radius, 0.) / mu).exp()
            },
            (Some((t, _)), None) if t < max - min => 0.,
            _ => 1.,
        }
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.body.to_sphere(&(p - self.body.centre)).norm() < self.body.a
    }
}

impl Ringed {
    fn ring_intersection(&self, r: &Ray, d: &Vector3<f64>, t: f64) -> Option<RawIntersection> {
        let pole = self.body.frame.column(1).into_owned();
        let normal = if d.dot(&pole) < 0. { pole } else { -pole };
        // On the plane exactly: the point along the ray can be metres off at 10⁹ m.
        let p = r.ro + d * t;
        let p = p - pole * (p - self.body.centre).dot(&pole);
        Some(RawIntersection { dist: t, point: p, normal, face: 0, part: RING_PART })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saturn() -> Ringed {
        Ringed {
            body: Spheroid { centre: Vector3::new(0., 0., 1.2e9), frame: Matrix3::identity(), a: 60_268e3, c: 54_364e3 },
            rings: Some((74_500e3, 136_800e3)),
            profile: None,
        }
    }

    #[test]
    fn test_spheroid_is_flattened() {
        let s = saturn();
        // Through the centre along z: the equatorial radius in front.
        let (t, n) = s.body.hit(&Vector3::zeros(), &Vector3::z()).unwrap();
        assert!((t - (1.2e9 - 60_268e3)).abs() < 1e-3, "{}", t);
        assert!((n + Vector3::z()).norm() < 1e-9);
        // A ray grazing at 57,000 km above the equator's plane misses (the pole is lower); one
        // at 57,000 km beside it, in the plane, hits.
        let aim = |off: Vector3<f64>| (s.body.centre + off).normalize();
        assert!(s.body.hit(&Vector3::zeros(), &aim(Vector3::new(0., 57_000e3, 0.))).is_none());
        assert!(s.body.hit(&Vector3::zeros(), &aim(Vector3::new(57_000e3, 0., 0.))).is_some());
    }

    #[test]
    fn test_rings_in_front_and_behind() {
        let mut s = saturn();
        // Tilt the rings towards the viewer, about x.
        s.body.frame = *crate::na::Rotation3::from_axis_angle(&Vector3::x_axis(), 0.4).matrix();
        let r = |off: Vector3<f64>| Ray { ro: Vector3::zeros(), rd: (s.body.centre + off).normalize() };
        // Beside the planet, at 100,000 km: the rings.
        let x = s.intersects(&r(Vector3::new(100_000e3, 0., 0.))).unwrap();
        assert_eq!(x.part, RING_PART);
        assert!(((x.point - s.body.centre).norm() - 100_000e3).abs() < 1e3);
        // Through the centre: the planet, unless the rings cross in front of it.
        let c = s.intersects(&r(Vector3::zeros())).unwrap();
        assert_eq!(c.part, 0);
        // The pole leans away, so the near half of the ring crosses in front of the disc
        // above its centre...
        let above = s.intersects(&r(Vector3::new(0., 45_000e3, 0.))).unwrap();
        assert_eq!(above.part, RING_PART, "{}", above.dist);
        // ...and the far half is hidden behind it below.
        assert_eq!(s.intersects(&r(Vector3::new(0., -45_000e3, 0.))).unwrap().part, 0);
    }

    #[test]
    fn test_uncovered_by_the_planet() {
        let s = saturn();
        // From behind the planet, looking at the sun through it.
        let p = s.body.centre + Vector3::new(0., 0., 100_000e3);
        assert_eq!(s.body.uncovered(&p, &-Vector3::z(), 1e-4), 0.);
        assert_eq!(s.body.uncovered(&p, &Vector3::x(), 1e-4), 1.);
    }
}
