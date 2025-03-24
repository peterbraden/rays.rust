use crate::material::texture::Medium;
use crate::shapes::geometry::Geometry;
use crate::intersection::{RawIntersection, Intersection};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use std::sync::Arc;

pub struct SceneObject {
    pub geometry: Box<dyn Geometry + Sync + Send>,
    pub medium: Box<dyn Medium + Sync + Send>,
}
/*
impl SceneObject {
   pub fn intersects(&self, r: &Ray) -> Option<Intersection> { 
       match self.geometry.intersects(r) {
           Some(i) => {
               return Some(Intersection {
                  dist: i.dist, 
                  point: i.point,
                  normal: i.normal,
                  object: Arc::new(self),
               })
           },
           None => return None
       }
   }
}*/

impl Geometry for SceneObject {
   fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
    self.geometry.intersects(r)
   }

    fn bounds(&self) -> BBox {
        self.geometry.bounds()
    }
}
