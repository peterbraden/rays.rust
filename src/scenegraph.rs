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

    pub fn nearestIntersection(&self, r: Ray, max:f64, min:f64) -> Intersection {
        let nil = Vec3::new(0f64,0f64,0f64);
        Intersection { dist: max, point: nil, normal: nil } 
    }
}
