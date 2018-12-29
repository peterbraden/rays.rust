use na::Vector3;
use sceneobject::SceneObject;
use std::cmp;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RawIntersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
}

#[derive(Clone)]
pub struct Intersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub object: Arc<SceneObject>,
}

/*
pub struct MediumIntersection {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub inside: Box<Medium + Sync + Send>,
    pub outside: Box<Medium + Sync + Send>,
}
*/

impl cmp::PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        &self.point == &other.point
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Intersection p:{} d:{} n:{})", self.point, self.dist, self.normal)
    }
}
