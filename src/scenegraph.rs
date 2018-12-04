use ray::Ray;
use na::{Vector3};
use intersection::Intersection;
use sceneobject::SceneObject;
use shapes::bbox::BBox;
use octree::Octree;
use std::sync::Arc;
use std::fmt;

pub struct SceneGraph {
    pub items: Vec<Arc<SceneObject>>,
    tree: Octree<SceneObject>,
    scene_bounds: BBox,
}

impl SceneGraph {

    pub fn new(max_depth: usize, objects: Vec<Arc<SceneObject>>) -> SceneGraph {

        let mut items = vec![];
        let mut scene_bounds = BBox::new( Vector3::new(0f64,0f64,0f64), Vector3::new(0f64,0f64,0f64) ); 
        for x in objects {
            scene_bounds = scene_bounds.union( &x.geometry.bounds() );
            items.push(x);
        }

        let tree = Octree::new(
            max_depth, 
            scene_bounds.clone(),
            &items,
        );

        SceneGraph { items, tree, scene_bounds }
    }

    pub fn items(&self) -> &Vec<Arc<SceneObject>>{
        &self.items
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        return match self.tree.intersection(r, max, min){
            Some(tupl) =>{
                let scene_obj = tupl.0.clone();
                return Some(
                            Intersection {
                              dist: tupl.1.dist, 
                              point: tupl.1.point,
                              normal: tupl.1.normal,
                              object: scene_obj,
                           })
            },
            None => None
        }
    }
    /*

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
*/
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
