use na::Vec3;
use sceneobject::SceneObject;

pub struct Intersection {
    pub dist: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
}
