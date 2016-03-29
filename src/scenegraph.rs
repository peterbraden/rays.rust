use ray::Ray;
use intersection::Intersection;
use na::Vec3;

pub struct SceneGraph {
    items: Vec<i32>
}
impl SceneGraph {
    pub fn new() -> SceneGraph {
        SceneGraph {
            items: vec![]
        }
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        return None
    }
}
