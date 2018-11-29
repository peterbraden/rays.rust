use material::texture::Medium;
use shapes::geometry::Geometry;
use intersection::Intersection;
use bbox::BBox;
use na::Vector3;
use ray::Ray;

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
