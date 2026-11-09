use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;

pub struct Plane {
    pub y: f64
}

impl Geometry for Plane {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rdn = r.rd.normalize();
        let mut norm = Vector3::new(0., 1., 0.);
        let denom = norm.dot(&rdn);

        if denom.abs() > 0. {
            let dist = -(norm.dot(&r.ro) - self.y) / denom;
            if dist > 0. {
                if denom > 0. {
                    norm = -norm
                }

                return Some(
                    RawIntersection { part: 0,
                        dist, 
                        point: r.ro + (rdn * dist),
                        normal: norm,
                    })
            }
        }

        None
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, self.y, f64::MIN),
            Vector3::new(f64::MAX, self.y, f64::MAX),
          )
    }

    /// Below the plane, as for a half-space with normal +y.
    fn inside(&self, p: &Vector3<f64>) -> bool {
        p.y < self.y
    }
}

/// POV-Ray's `plane`: the surface n.p = d, for unit normal n, filling the half of space it faces
/// away from (n.p < d). Unlike `Plane`, its normal always points out of that half, so it can be
/// used in CSG, eg. to cut a shape in two.
pub struct HalfSpace {
    normal: Vector3<f64>,
    distance: f64,
}

impl HalfSpace {
    pub fn new(normal: Vector3<f64>, distance: f64) -> HalfSpace {
        assert!(normal.norm() > 0., "plane normal must not be zero");
        HalfSpace { normal: normal.normalize(), distance }
    }
}

impl Geometry for HalfSpace {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rd = r.rd.normalize();
        let denom = self.normal.dot(&rd);
        if denom == 0. {
            return None;
        }
        let dist = (self.distance - self.normal.dot(&r.ro)) / denom;
        if dist <= 0. {
            return None;
        }
        Some(RawIntersection { part: 0, dist, point: r.ro + rd * dist, normal: self.normal })
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.intersects(r).into_iter().collect()
    }

    /// Everywhere, unless the normal is along an axis, when the half-space is bounded on that side.
    fn bounds(&self) -> BBox {
        let (mut min, mut max) = (Vector3::repeat(f64::MIN), Vector3::repeat(f64::MAX));
        for a in 0..3 {
            if self.normal[a].abs() == 1. {
                if self.normal[a] > 0. { max[a] = self.distance; } else { min[a] = -self.distance; }
            }
        }
        BBox::new(min, max)
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.normal.dot(p) < self.distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_space() {
        let h = HalfSpace::new(Vector3::new(0., 2., 0.), 1.);
        assert!(h.inside(&Vector3::new(5., 0.5, 3.)) && !h.inside(&Vector3::new(0., 1.5, 0.)));
        // From below, the normal still points up, out of the half-space.
        let i = h.intersects(&Ray { ro: Vector3::new(0., -3., 0.), rd: Vector3::new(0., 1., 0.) }).unwrap();
        assert!((i.dist - 4.).abs() < 1e-12);
        assert_eq!(i.normal, Vector3::new(0., 1., 0.));
        assert_eq!(h.bounds().max.y, 1.);
        let down = HalfSpace::new(Vector3::new(0., -1., 0.), 1.);
        assert_eq!(down.bounds().min.y, -1.);
        assert!(down.inside(&Vector3::new(0., 0., 0.)) && !down.inside(&Vector3::new(0., -2., 0.)));
    }

    #[test]
    fn test_bounds_contain_plane() {
        let p = Plane { y: 5. };
        assert!(p.bounds().contains_point(&Vector3::new(3., 5., -2.)));
    }
}
