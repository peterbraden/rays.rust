use ray::Ray;
use na::{Vector3};
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
            scene_bounds: BBox::new( Vector3::new(0f64,0f64,0f64), Vector3::new(0f64,0f64,0f64) ),
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


    pub fn partitions(&self) -> Vec<BBox> {
        let mut bboxes = Vec::new();
        match self.root {
            Some (ref root) => { 
                bboxes = root.partitions();
            }
            None => {}
        }
        return bboxes;
    }

    pub fn items(&self) -> &Vec<Rc<SceneObject>>{
        &self.items
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64, exclude: Option<&SceneObject>) -> Option<Intersection> {
        /*
        let naive = self.naive_intersection(r,max,min,exclude);
        let tree = self.tree_nearest_intersection(r,max,min);

        if naive != tree {
            println!("Intersection doesn't match for {} ({} {})", r, max, min);
            match naive{
                Some(_) => (println!("- naive: {}", naive.unwrap())),
                None => (println!("- naive: none")),
            }
            match tree{
                Some(_) => (println!("- tree: {}", tree.unwrap())),
                None => (println!("- tree: none")),
            }
        }
        */

        return self.naive_intersection(r,max,min, exclude);
    }

    pub fn tree_nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        match self.root {
            Some (ref root) => { return root.intersection(r, max, min); }
            None => { return None; }
        }
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
            self.scene_bounds = self.scene_bounds.union( &x.bounds() );
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
