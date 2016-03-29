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
        return None
        //return self.items[0].intersects(r);
        // Naive approach first
        /*
        for o in &self.items {
            return o.intersects(r);
        }
        */
            //closest = match &o.intersects(r) {
            //    Some(i) => i,
            //    None => closest,
           //}
    }

    //pub fn push(&self, s: SceneObject) {
        //self.items.push(Box::new(s));
    //}
}
