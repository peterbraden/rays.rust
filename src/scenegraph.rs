use crate::ray::Ray;
use crate::na::{Vector3};
use crate::intersection::{Intersection, RawIntersection};
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use std::sync::Arc;
use std::fmt;
use crate::shapes::geometry::Geometry;

pub struct SceneGraph {
    pub items: Vec<Arc<SceneObject>>,
    pub infinite_items: Vec<Arc<SceneObject>>,
    tree: Octree<SceneObject>,
    scene_bounds: BBox,
}

impl SceneGraph {

    pub fn new(max_depth: usize, objects: Vec<Arc<SceneObject>>, max_bounding: BBox) -> SceneGraph {

        let mut items = vec![];
        let mut infinite_items = vec![];
        let mut scene_bounds = BBox::new( Vector3::new(0f64,0f64,0f64), Vector3::new(0f64,0f64,0f64) ); 
        for x in objects {
            if max_bounding.contains(&x.geometry.bounds()) {
                scene_bounds = scene_bounds.union( &x.geometry.bounds() );
                items.push(x);
            } else {
                // This object is excessively large (probably an infinite plane or skysphere
                // Instead of trying to contain it in our scene BVH, we fall back to a naive
                // iteration intersection.
                // These are of course more expensive.
                infinite_items.push(x)
            }
        }

        let tree = Octree::new(
            max_depth, 
            scene_bounds.clone(),
            &items,
        );

        SceneGraph { items, tree, scene_bounds, infinite_items }
    }

    pub fn items(&self) -> &Vec<Arc<SceneObject>>{
        &self.items
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        return match self.nearest_raw_intersection(r, max, min) {
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
        };
    }


    pub fn nearest_raw_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(Arc<SceneObject>, RawIntersection)> { 
        let tree_results =self.tree.intersection(r, max, min);
        let infinites = self.naive_intersection_infinites(r, max, min);

        if infinites.is_none() {
            return tree_results;
        }
        if tree_results.is_none() {
            return infinites;
        }
        let inf = infinites.unwrap();
        let tree = tree_results.unwrap();
        if inf.1.dist < tree.1.dist {
            return Some(inf);
        }
        return Some(tree);
    }

    pub fn naive_intersection_infinites(&self, r: &Ray, max:f64, min:f64) -> Option<(Arc<SceneObject>, RawIntersection)> {
        let mut cdist = max;
        let mut closest = None;
        
        for o in &self.infinite_items {
            /*
            match exclude {
                Some(x) => {
                    if &*x as *const _  == &**o {
                        continue;
                    }
                }
                None => (),
            }
            */
            match o.intersects(r) {
                Some(x) => {
                    if x.dist < cdist && x.dist >= min {
                        cdist = x.dist;
                        closest = Some((o.clone(), x));
                    }
                },
                None => (),
            }
        }
        return closest;
    }
}

impl fmt::Display for SceneGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| SceneGraph<octree> \n| objects: {} \n| bounded: {} \n| infinite objs. {}", //\n{}\n",
                &self.items.len(), 
                &self.scene_bounds,
                &self.infinite_items.len(), 
 //               &self.root.as_ref().unwrap()
            )
    }
}
