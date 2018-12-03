use material::texture::Medium;
use shapes::geometry::Geometry;
use intersection::{RawIntersection, Intersection};
use na::Vector3;
use ray::Ray;
use shapes::bbox::BBox;

pub struct SceneObject {
    pub geometry: Box<Geometry + Sync + Send>,
    pub medium: Box<Medium + Sync + Send>,
}

impl SceneObject {
   pub fn intersects(&self, r: &Ray) -> Option<Intersection> { 
       match self.geometry.intersects(r) {
           Some(i) => {
               return Some(Intersection {
                  dist: i.dist, 
                  point: i.point,
                  normal: i.normal,
                  object: self,
               })
           },
           None => return None
       }
   }
}
impl Geometry for SceneObject {
   fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
    return self.geometry.intersects(r);
   }

    fn bounds(&self) -> BBox {
        return self.geometry.bounds();
    }
}
