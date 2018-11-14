use ray::Ray;
use na::{Vec3};
use intersection::Intersection;
use sceneobject::SceneObject;
use bbox::BBox;
use octree::OctreeNode;
use std::rc::Rc;
use std::fmt;



pub struct SceneGraph {
    items: Vec<Rc<SceneObject>>,
    root: Option<OctreeNode>,
    scene_bounds: BBox,
}

impl SceneGraph {

    pub fn new() -> SceneGraph {
        SceneGraph {
            items: vec![],
            root: None, 
            scene_bounds: BBox::new( Vec3::new(0f64,0f64,0f64), Vec3::new(0f64,0f64,0f64) ),
        }
    }

    pub fn partition(&mut self, max_depth: i64) {
        self.root = Some(
                        OctreeNode::new(
                            0,
                            max_depth, 
                            (&self.scene_bounds).clone(),
                            &self.items,
                            )
                        );
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64, exclude: Option<&SceneObject>) -> Option<Intersection> {
        //match self.root {
        //    Some (ref root) => { return root.items_intersection(r, max, min); }
        //    None => { return None; }
        //}
        
        return self.naive_intersection(r,max,min,exclude);
    }


    pub fn naive_intersection(&self, r: &Ray, max:f64, min:f64, exclude: Option<&SceneObject>) -> Option<Intersection> {
        let mut cdist = max;
        let mut closest = None;
        
        for o in &self.items {
            match exclude {
                Some(x) => {
                    if &*x as *const _  == &**o {
                        continue;
                    }
                }
                None => (),
            }
            match o.intersects(r) {
                Some(x) => {
                    if x.dist < cdist && x.dist >= min {
                        cdist = x.dist;
                        closest = Some(x);
                    }
                },
                None => (),
            }
        }
        return closest;
    }

    pub fn push(&mut self, s: Vec<Rc<SceneObject>>) {
        for x in s {
            self.scene_bounds = self.scene_bounds.union( &x.geometry.bounds() );
            &self.items.push(x);
        }
        self.partition(8);
    }
}

impl fmt::Display for SceneGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SceneGraph \n objects: {} \n bounded: {}\n{}\n",
                &self.items.len(), 
                &self.scene_bounds,
                &self.root.as_ref().unwrap()
            )
    }
}
