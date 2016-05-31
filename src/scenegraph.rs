use ray::Ray;
use na::{Vec3};
use intersection::Intersection;
use sceneobject::SceneObject;
use bbox::BBox;

pub struct SceneGraphNode {
    depth: i64,
    bounds: BBox,
    mid: Vec3<f64>,
    // Octree structure:
    children: [Option<Box<SceneGraphNode>>; 8],
    items: Vec<Box<SceneObject>>,
}

impl SceneGraphNode {

    //
    // Create a new node, and subdivide into further nodes up until max_depth
    // or until number of children objects is 0.
    //
    pub fn new(depth: i64, max_depth: i64, b: BBox, children: Vec<Box<SceneObject>>) -> SceneGraphNode {

        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<SceneGraphNode>>; 8] = Default::default();
        
        for i in 0..8 {
            if (depth < max_depth) {
            
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


pub struct SceneGraph {
    items: Vec<Box<SceneObject>>,
    root: SceneGraphNode,
}

impl SceneGraph {
    pub fn new() -> SceneGraph {
        let root = SceneGraphNode::new(0, 1, 
                    BBox::new(
                        Vec3::new(0f64,0f64,0f64),
                        Vec3::new(0f64,0f64,0f64),
                        ),
                    vec![],
                    );

        SceneGraph {
            items: vec![],
            root: root,
        }
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

    pub fn push(&mut self, s: Box<SceneObject>) {
        &self.items.push(s);
    }
}
