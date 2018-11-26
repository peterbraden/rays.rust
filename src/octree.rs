use bbox::BBox;
use na::{Vec3};
use sceneobject::SceneObject;
use std::sync::Arc;
use std::fmt;
use ray::Ray;
use intersection::Intersection;

pub struct OctreeNode {
    depth: i64,
    bounds: BBox,
    mid: Vec3<f64>,
    // Octree structure:
    children: [Option<Box<OctreeNode>>; 8],
    items: Vec<Arc<SceneObject>>,
}

impl OctreeNode {

    //
    // Create a new node, and subdivide into further nodes up until max_depth
    // or until number of children objects is 0.
    //
    pub fn new(depth: i64, max_depth: i64, b: BBox, items: &Vec<Arc<SceneObject>>) -> OctreeNode {

        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<OctreeNode>>; 8] = Default::default();
/*
        let contained: Vec<Rc<SceneObject>> =
                items
                    .into_iter()
                    .cloned()
                    .filter( |x| {b.contains( &x.bounds() )})
                    .collect();
  */     

        for i in 0..8 {
            // Does child node have any objects in?
            if depth < max_depth && items.len() > 1 {
                // Equal subdivision. Enhancement: Use different split.
                let cbox = BBox::for_octant(i, &b);
                let item_iter = items.into_iter();
                let inside = item_iter
                                    .cloned()
                                    .filter( |x| { cbox.intersects_bbox( &x.geometry.bounds() ) } )
                                    .collect::<Vec<Arc<SceneObject>>>();

                if inside.len() > 0 {
                    let node = OctreeNode::new( depth + 1, max_depth, cbox, &inside);
                    children[i as usize] = Some(Box::new(node));
                }
            }
        }


        OctreeNode {
            depth: depth,
            mid: (&b).mid(), 
            bounds: b,
            children: children, 
            items: items.into_iter().cloned().collect(),
        }
    }


    pub fn is_leaf(&self) -> bool {
        for i in 0..8 {
            match self.children[i as usize] {
                Some(_) => { return false },
                None => {}
            }
        }
        return true;
    }

    pub fn is_empty(&self) -> bool {
        return self.items.len() > 0;
    }

    pub fn len(&self) -> usize {
        return self.items.len()
    }

    


    // Iterate through items. Should only do on leaf if multiple items.
    pub fn items_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        let mut cdist = max;
        let mut closest = None;
        for o in &self.items {
            match o.intersects(r) {
                Some(x) => {
                    if x.dist < cdist && x.dist > min {
                        cdist = x.dist;
                        closest = Some(x);
                    }
                },
                None => (),
            }
        }
        return closest;
    }

}



impl fmt::Display for OctreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut c = "".to_string();
        let mut p = "".to_string();

        for _ in -1.. self.depth{
            p = p + "  ";
        }
        p = p + "|-";

        for i in 0..8 {
            match self.children[i as usize].as_ref() {
                Some (ref r) => {
                    c = c + "\n" + &p + " " + &r.to_string();
                    
                },
                None => {
                    //c = c + "\n" + &p + " None";
                },
            }
        }
        write!(f, "OctreeNode -[{}]{}", self.items.len(), c)
    }
}

