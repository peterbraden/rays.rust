use na::Vec3;
use sceneobject::SceneObject;

pub struct RawIntersection {
    pub dist: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
}

#[derive(Clone)]
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
