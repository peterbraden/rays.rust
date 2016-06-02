use ray::Ray;
use na::{Vec3};
use intersection::Intersection;
use sceneobject::SceneObject;
use bbox::BBox;
use std::rc::Rc;
use std::fmt;

pub struct SceneGraphNode {
    depth: i64,
    bounds: BBox,
    mid: Vec3<f64>,
    // Octree structure:
    children: [Option<Box<SceneGraphNode>>; 8],
    items: Vec<Rc<SceneObject>>,
}

impl SceneGraphNode {

    //
    // Create a new node, and subdivide into further nodes up until max_depth
    // or until number of children objects is 0.
    //
    pub fn new(depth: i64, max_depth: i64, b: BBox, items: &Vec<Rc<SceneObject>>) -> SceneGraphNode {

        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<SceneGraphNode>>; 8] = Default::default();
        
        for i in 0..8 {
            if depth < max_depth {
                // Does child node have any objects in?
                let cbox = BBox::for_octant(i, &b);
                let item_iter = items.into_iter();
                let inside = item_iter
                                    .cloned()
                                    .filter( |x| { cbox.intersects_bbox( &x.bounds() ) } )
                                    .collect::<Vec<Rc<SceneObject>>>();

                if inside.len() > 0 {
                    let node = SceneGraphNode::new( depth + 1, max_depth, cbox, &inside);
                    children[i as usize] = Some(Box::new(node));
                }
            }
        }


        SceneGraphNode {
            depth: depth,
            mid: (&b).mid(), 
            bounds: b,
            children: children, 
            items: vec![],
        }
    }
}

impl fmt::Display for SceneGraphNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n {} Node -[]", &self.depth)
    }
}

pub struct SceneGraph {
    items: Vec<Rc<SceneObject>>,
    root: Option<SceneGraphNode>,
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

    pub fn partition(&mut self) {
        self.root = Some(
                        SceneGraphNode::new(
                            0,
                            1, 
                            (&self.scene_bounds).clone(),
                            &self.items,
                            )
                        );
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64, exclude: Option<&SceneObject>) -> Option<Intersection> {
        // Naive approach first
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

    pub fn push(&mut self, s: Vec<Rc<SceneObject>>) {
        for x in s {
            self.scene_bounds = self.scene_bounds.loosen( &x.bounds() );
            &self.items.push(x);
        }
        self.partition();
    }
}

impl fmt::Display for SceneGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SceneGraph \n objects: {} \n bounded: {}->\n", &self.items.len(), &self.scene_bounds)
    }
}
