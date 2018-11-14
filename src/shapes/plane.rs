use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::RawIntersection;
use bbox::BBox;
use shapes::geometry::Geometry;

pub struct Plane {
    pub y: f64
}

impl Geometry for Plane {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let rdn = r.rd.normalize();
        let mut norm = Vec3::new(0., 1., 0.);
        let denom = norm.dot(&rdn);

        if denom.abs() > 0. {
            let dist = -(norm.dot(&r.ro) - self.y) / denom;
            if dist > 0. {
                if denom > 0. {
                    norm = -norm
                }

                return Some(
                    RawIntersection {
                        dist: dist, 
                        point: r.ro + (rdn * dist),
                        normal: norm,
                    })
            }
        }

        None
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vec3::new(0.,0.,0.),
            Vec3::new(0.,0.,0.),
          )
    }
}
