use ray::Ray;
use intersection::Intersection;
use na::Vec3;
use sceneobject::SceneObject;

pub struct SceneGraph {
     items: Vec<Box<SceneObject>>
}
impl SceneGraph {
    pub fn new() -> SceneGraph {
        SceneGraph {
            items: vec![]
        }
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        // Naive approach first
        let mut cdist = max;
        let mut closest = None;

        
        for o in &self.items {
            match o.intersects(r) {
                Some(x) => {
                    if x.dist < cdist {
                        cdist = x.dist;
                        closest = Some(x);
                    }
                },
                None => (),
            }
        }
        return closest;
    }

    pub fn push(&mut self, s: Box<SceneObject>) {
        &self.items.push(s);
    }
}
