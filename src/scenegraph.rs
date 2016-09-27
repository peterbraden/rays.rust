use ray::Ray;
use na::{Vec3};
use intersection::Intersection;
use sceneobject::SceneObject;
use bbox::BBox;
use octree::OctreeNode;
use std::sync::Arc;
use std::fmt;



pub struct SceneGraph {
    items: Vec<Arc<SceneObject>>,
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

        return self.tree_nearest_intersection(r,max,min);
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

    pub fn push(&mut self, s: Vec<Arc<SceneObject>>) {
        for x in s {
            self.scene_bounds = self.scene_bounds.union( &x.geometry.bounds() );
            &self.items.push(x);
        }
<<<<<<< HEAD
        self.partition(8);
=======
        self.partition(2);
        //print!("{}", self);

>>>>>>> 1d1ab25... Octree algorithm
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
