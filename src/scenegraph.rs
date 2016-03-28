use ray::Ray;
use intersection::Intersection;

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
        Intersection { dist: max } 
    }
}
