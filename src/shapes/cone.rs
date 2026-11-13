use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::polynomial::quadratic_roots;

/// A truncated cone between two points, with a radius at each: `base_radius` at `base` and
/// `cap_radius` at `cap`. With equal radii it is a cylinder. Unless `open`, the ends are closed by
/// flat discs, so the cone is a solid that can be used in CSG and with dielectrics. The POV-Ray
/// `cone` and `cylinder` (POV-Ray 3.7 reference: "Cone" and "Cylinder").
///
/// In a frame with the axis w from the base, a point at height s = (p - base).w and distance
/// rho from the axis is on the side if rho = r(s) = base_radius + k s, with k the change in radius
/// per unit height. Along a ray that is a quadratic in the distance.
pub struct Cone {
    base: Vector3<f64>,
    /// Unit axis from base to cap.
    axis: Vector3<f64>,
    height: f64,
    base_radius: f64,
    cap_radius: f64,
    open: bool,
}

impl Cone {
    pub fn new(base: Vector3<f64>, base_radius: f64, cap: Vector3<f64>, cap_radius: f64, open: bool) -> Cone {
        let height = (cap - base).norm();
        assert!(height > 0., "cone or cylinder ends must differ");
        assert!(base_radius >= 0. && cap_radius >= 0., "cone radii must not be negative");
        Cone { base, axis: (cap - base) / height, height, base_radius, cap_radius, open }
    }

    pub fn cylinder(base: Vector3<f64>, cap: Vector3<f64>, radius: f64, open: bool) -> Cone {
        Cone::new(base, radius, cap, radius, open)
    }

    fn slope(&self) -> f64 {
        (self.cap_radius - self.base_radius) / self.height
    }

    fn radius_at(&self, s: f64) -> f64 {
        self.base_radius + self.slope() * s
    }
}

/// Bounds of a disc of radius `r` about `c` perpendicular to unit `w`: along each axis it reaches
/// r sin(angle between the axis and w).
pub fn disc_bounds(c: Vector3<f64>, w: Vector3<f64>, r: f64) -> BBox {
    let e = Vector3::new(
        (1. - w.x * w.x).max(0.).sqrt(),
        (1. - w.y * w.y).max(0.).sqrt(),
        (1. - w.z * w.z).max(0.).sqrt()) * r;
    BBox::new(c - e, c + e)
}

impl Geometry for Cone {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.crossings(r).into_iter().next()
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        let rd = r.rd.normalize();
        let w = self.axis;
        let k = self.slope();
        let oc = r.ro - self.base;
        let (s0, ds) = (oc.dot(&w), rd.dot(&w));
        let (o_perp, d_perp) = (oc - w * s0, rd - w * ds);
        let r0 = self.base_radius + k * s0;

        let mut hits: Vec<(f64, Vector3<f64>)> = Vec::new();
        let mut consider = |t: f64, n: Vector3<f64>| {
            if t > 0. {
                hits.push((t, n));
            }
        };

        // Side: |o_perp + t d_perp|^2 = (r0 + k ds t)^2
        let a = d_perp.dot(&d_perp) - k * k * ds * ds;
        let b = 2. * (d_perp.dot(&o_perp) - k * ds * r0);
        let c = o_perp.dot(&o_perp) - r0 * r0;
        for t in quadratic_roots(c, b, a) {
            let s = s0 + t * ds;
            if (0. ..=self.height).contains(&s) && self.radius_at(s) >= 0. {
                let p_perp = o_perp + d_perp * t;
                let rho = p_perp.norm();
                if rho > 0. {
                    consider(t, (p_perp / rho - w * k).normalize());
                }
            }
        }

        // End caps.
        if !self.open && ds != 0. {
            for (s, radius, n) in [(0., self.base_radius, -w), (self.height, self.cap_radius, w)] {
                let t = (s - s0) / ds;
                if (o_perp + d_perp * t).norm_squared() <= radius * radius {
                    consider(t, n);
                }
            }
        }

        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        hits.dedup_by(|a, b| (a.0 - b.0).abs() <= 1e-9 * (1. + b.0));
        hits.into_iter().map(|(dist, normal)| RawIntersection { face: 0, part: 0, dist, point: r.ro + rd * dist, normal }).collect()
    }

    fn bounds(&self) -> BBox {
        disc_bounds(self.base, self.axis, self.base_radius)
            .union(&disc_bounds(self.base + self.axis * self.height, self.axis, self.cap_radius))
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        let s = (p - self.base).dot(&self.axis);
        let rho = ((p - self.base) - self.axis * s).norm();
        !self.open && s > 0. && s < self.height && rho < self.radius_at(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    #[test]
    fn test_cylinder_side_and_caps() {
        let c = Cone::cylinder(v(0., -1., 0.), v(0., 1., 0.), 0.5, false);
        let side = c.intersects(&Ray { ro: v(0., 0., -5.), rd: v(0., 0., 2.) }).unwrap();
        assert!((side.dist - 4.5).abs() < 1e-9);
        assert!((side.normal - v(0., 0., -1.)).norm() < 1e-9);
        let cap = c.intersects(&Ray { ro: v(0.1, 5., 0.), rd: v(0., -1., 0.) }).unwrap();
        assert!((cap.dist - 4.).abs() < 1e-9);
        assert_eq!(cap.normal, v(0., 1., 0.));
        // Open, the ray down the axis passes straight through.
        let open = Cone::cylinder(v(0., -1., 0.), v(0., 1., 0.), 0.5, true);
        assert!(open.intersects(&Ray { ro: v(0.1, 5., 0.), rd: v(0., -1., 0.) }).is_none());
        // Missing beside it.
        assert!(c.intersects(&Ray { ro: v(1., 0., -5.), rd: v(0., 0., 1.) }).is_none());
    }

    #[test]
    fn test_from_inside_hits_the_wall() {
        let c = Cone::cylinder(v(0., -1., 0.), v(0., 1., 0.), 0.5, false);
        let i = c.intersects(&Ray { ro: v(0., 0., 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.dist - 0.5).abs() < 1e-9);
        assert!(c.inside(&v(0., 0., 0.)) && !c.inside(&v(0., 2., 0.)));
    }

    #[test]
    fn test_cone_normal_slopes() {
        // Radius 1 at y = 0 narrowing to 0 at y = 1: the side at 45 degrees.
        let c = Cone::new(v(0., 0., 0.), 1., v(0., 1., 0.), 0., false);
        let i = c.intersects(&Ray { ro: v(-5., 0.5, 0.), rd: v(1., 0., 0.) }).unwrap();
        assert!((i.point.x + 0.5).abs() < 1e-9);
        let expected = v(-1., 1., 0.).normalize();
        assert!((i.normal - expected).norm() < 1e-9, "{}", i.normal);
        // Above the apex.
        assert!(c.intersects(&Ray { ro: v(-5., 1.5, 0.), rd: v(1., 0., 0.) }).is_none());
    }

    #[test]
    fn test_bounds_of_tilted_cylinder() {
        let c = Cone::cylinder(v(0., 0., 0.), v(1., 1., 0.), 1., false);
        let b = c.bounds();
        let s = (0.5f64).sqrt();
        assert!((b.min.x + s).abs() < 1e-9 && (b.max.x - 1. - s).abs() < 1e-9);
        assert!((b.min.z + 1.).abs() < 1e-9);
    }
}
