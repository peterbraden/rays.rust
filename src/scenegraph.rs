use ray::Ray;
use na::{Vector3};
use intersection::Intersection;
use sceneobject::SceneObject;
use shapes::bbox::BBox;
use octree::OctreeNode;
use std::sync::Arc;
use std::fmt;

pub struct SceneGraph {
    pub items: Vec<Arc<SceneObject>>,
    root: Option<OctreeNode<SceneObject>>,
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

    pub fn items(&self) -> &Vec<Arc<SceneObject>>{
        &self.items
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64, exclude: Option<&SceneObject>) -> Option<Intersection> {
        let tree = self.tree_nearest_intersection(r,max,min);
		/*
        let naive = self.naive_intersection(r,max,min,exclude);
        if naive != tree {
            println!("Intersection doesn't match for {} ({} {})", r, max, min);
            match naive{
                Some(_) => (println!("- naive: {}", &naive.clone().unwrap())),
                None => (println!("- naive: none")),
            }
            match tree{
                Some(_) => (println!("- tree: {}", &tree.clone().unwrap())),
                None => (println!("- tree: none")),
            }
        }
		*/
        return tree;
    }

    pub fn tree_nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        if self.root.is_some() { 
             return match self.root.unwrap().intersection(r, max, min) {
                Some(i) => Some(Intersection {
                      dist: i.0.dist, 
                      point: i.0.point,
                      normal: i.0.normal,
                      object: &i.1,
                   }),
                None => None
            };
        }
        return None
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
        self.partition(2);
    }
}

impl fmt::Display for SceneGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SceneGraph \n objects: {} \n bounded: {}", //\n{}\n",
                &self.items.len(), 
                &self.scene_bounds,
 //               &self.root.as_ref().unwrap()
            )
    }
}
