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
                    RawIntersection { face: 0,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_contain_plane() {
        let p = Plane { y: 5. };
        assert!(p.bounds().contains_point(&Vector3::new(3., 5., -2.)));
    }
}
