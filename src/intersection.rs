use na::Vec3;
use sceneobject::SceneObject;
use std::cmp;
use std::fmt;

pub struct Intersection<'a> {
    pub dist: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub object: &'a SceneObject
}

impl <'a> Intersection<'a> {
    /*
    pub fn new(dist: f64, point: Vec3<f64>, normal: Vec3<f64>, object: &'a SceneObject) -> Intersection<'a> {
        return Intersection {
            dist: dist,
            point: point,
            normal: normal,
            object: object,
        }
    }
    */
}

impl<'a> cmp::PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection) -> bool {
        &self.point == &other.point
    }
}

impl<'a> fmt::Display for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Intersection p:{} d:{})", self.point, self.dist)
    }
}
