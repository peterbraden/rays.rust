use crate::shapes::bbox::BBox;
use crate::na::{Vector3};
use crate::sceneobject::SceneObject;
use std::sync::Arc;
use std::fmt;
use crate::ray::Ray;
use crate::intersection::{RawIntersection, Intersection};
use ordered_float::OrderedFloat;
use crate::shapes::geometry::Geometry;

#[derive(Clone)]
pub struct Octree<T: Geometry>{
    root: OctreeNode,
    items: Vec<Arc<T>>,
}

#[derive(Clone)]
pub struct OctreeNode {
    depth: usize,
    bounds: BBox,
    // Octree structure:
    children: [Option<Box<OctreeNode>>; 8],
    items: Vec<usize>
}

fn vec3_invert(rd: Vector3<f64>) -> Vector3<f64> {
  Vector3::new(1.0/rd.x, 1.0/rd.y, 1.0/rd.z) 
}

type OctreeIntersections = Option<Vec<usize>>;

impl OctreeNode {
    pub fn is_leaf(&self) -> bool {
        for i in 0..8 {
            if self.children[i as usize].is_some() { return false }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() > 0
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Perform a breadth first search of the tree, and then sort results by distance.
    pub fn naive_intersection(&self, r: &Ray, _max:f64, _min:f64) -> OctreeIntersections {
        let invrd = vec3_invert(r.rd);
        if !self.bounds.fast_intersects(&r.ro, &invrd) {
            None
        } else if self.is_leaf() {
            Some(self.items.clone())
        } else {
            let intersections = self.children
                                .iter()
                                .filter(|i| i.is_some())
                                .filter_map(|c| c.as_ref().unwrap().naive_intersection(r, _max, _min))
                                .flatten()
                                .collect::<Vec<usize>>();

            if intersections.is_empty() {
                None
            } else {
                Some(intersections)
            }
        }
    }

}

impl<T: Geometry> Octree<T> {

    //
    // Create a new node, and subdivide into further nodes up until max_depth
    // or until number of children objects is 0.
    //
    pub fn new(max_depth: usize, b: BBox, items: &[Arc<T>]) -> Octree<T> {
        let items: Vec<Arc<T>> = items.to_vec();
        let indices: Vec<usize> = (0..items.len()).collect();
        Octree {
            root: Octree::create_node(0, max_depth, b, indices, &items),
            items,
        }
    }

    fn create_node(depth: usize, max_depth: usize, b: BBox, items: Vec<usize>, geometries: &Vec<Arc<T>>) -> OctreeNode{
        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<OctreeNode>>; 8] = Default::default();

        for i in 0..8 {
            // Does child node have any objects in?
            if depth < max_depth && items.len() > 1 {
                // Equal subdivision. Enhancement: Use different split.
                let cbox = BBox::for_octant(i, &b);
                let inside = items.clone()
                                .into_iter() 
                                .filter( |x| { cbox.intersects_bbox( &geometries[*x].bounds() ) } )
                                .collect::<Vec<usize>>();

                if !inside.is_empty() {
                    let node = Octree::create_node(depth + 1, max_depth, cbox, inside, geometries);
                    children[i as usize] = Some(Box::new(node));
                }
            }
        }


        OctreeNode {
            depth,
            bounds: b,
            children, 
            items,
        }

    }

    pub fn raw_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<RawIntersection> {
        self.closest_intersection(r, max, min).map(|tupl| tupl.1)
    }


    pub fn intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(Arc<T>, RawIntersection)> {
        self.closest_intersection(r, max, min).map(|tupl| (self.items[tupl.0].clone(), tupl.1))
    }

    fn closest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(usize, RawIntersection)> {
        match self.root.naive_intersection(r, max, min) {
            Some(opts) => self.items_intersection(r,max, min, opts),
            None => None
        }
    }

    // Iterate through potential items. Should only do on leaf if multiple items.
    fn items_intersection(&self, r: &Ray, max:f64, min:f64, items: Vec<usize>) -> Option<(usize, RawIntersection)> {
        let mut cdist = max;
        let mut closest = None;
        for i in items {
            if let Some(x) = self.items[i].intersects(r) {
                if x.dist < cdist && x.dist >= min {
                    cdist = x.dist;
                    closest = Some((i, x));
                }
            }
        }
        closest
    }

    pub fn bounds(&self) -> BBox {
       self.root.bounds 
    }

/*
    pub fn new_node(&self, txm:f64, x:u8, tym:f64, y:u8, tzm:f64, z:u8) -> u8{
        if txm < tym {
            if txm < tzm {return x;}  // YZ plane
        }
        else{
            if tym < tzm {return y;} // XZ plane
        }
       return z; // XY plane;
    }

    pub fn first_node_index(&self, tx0:f64, ty0:f64, tz0:f64, txm:f64, tym:f64, tzm:f64) -> u8 {
        let mut answer:u8 = 0;

        if tx0 > ty0 {
            if tx0 > tz0 { // PLANE YZ
                if tym < tx0 { answer|=2; }   // set bit at position 1
                if tzm < tx0 { answer|=1; } // set bit at position 0
                return answer;
            }
        } else {
            if ty0 > tz0 { // PLANE XZ
                if txm < ty0 { answer|=4; }  // set bit at position 2
                if tzm < ty0 { answer|=1; }  // set bit at position 0
            }
        }

        // PLANE XY
        if txm < tz0 { answer|=4; } // set bit at position 2
        if tym < tz0 { answer|=2; }    // set bit at position 1
        return answer;
    }

    /// Based on: 
    /// An Efficient Parametric Algorithm for Octree Traversal (2000)
    /// by J. Revelles , C. Ureña , M. Lastra
    ///
    /// Status: Broken
    /// Note: Missing bbox intersection?
    pub fn revelles_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        let mut a = 0;

        let rd = r.rd.normalize();
        let mut rd0 = rd[0];
        let mut rd1 = rd[1];
        let mut rd2 = rd[2];
        let mut ro0 = r.ro[0];
        let mut ro1 = r.ro[1];
        let mut ro2 = r.ro[2];

        // fixes for rays with negative direction
        if rd0 < 0. {
            ro0 = self.bounds.size()[0] - ro0;
            rd0  = - rd0;
            a |= 4 ; //bitwise OR (latest bits are XYZ)
        }
        if rd1 < 0. {
            ro1 = self.bounds.size()[1] - ro1;
            rd1 = - rd1;
            a |= 2 ; 
        }
        if rd2 < 0. {
            ro2 = self.bounds.size()[2] - ro2;
            rd2 = - rd2;
            a |= 1 ; 
        }

        let rn = Ray {
            ro: Vector3::new(ro0, ro1, ro2),
            rd: Vector3::new(rd0, rd1, rd2),
        };

        let divx:f64 = 1.0 / rn.rd[0];
        let divy:f64 = 1.0 / rn.rd[1];
        let divz:f64 = 1.0 / rn.rd[2];

        let tx0:f64 = (self.bounds.min[0] - rn.ro[0]) * divx;
        let tx1:f64 = (self.bounds.max[0] - rn.ro[0]) * divx;
        let ty0:f64 = (self.bounds.min[1] - rn.ro[1]) * divy;
        let ty1:f64 = (self.bounds.max[1] - rn.ro[1]) * divy;
        let tz0:f64 = (self.bounds.min[2] - rn.ro[2]) * divz;
        let tz1:f64 = (self.bounds.max[2] - rn.ro[2]) * divz;   


        if tz0.max(tx0.max(ty0)) < tz1.min(tx1.min(ty1)) {
            return self.revelles_proc_subtree(r, &rn, max, min, tx0, ty0, tz0, tx1, ty1, tz1, a);
        }

        //println!("- magnitude miss {} {} {}", r, tz0.max(tx0.max(ty0)), tz1.min(tx1.min(ty1)));
        return None;
    }
    
    pub fn revelles_proc_child(&self, ro: &Ray, r:&Ray, max:f64, min:f64, tx0:f64, ty0:f64, tz0:f64, tx1:f64, ty1:f64, tz1:f64, a:u8, i: u8)-> Option<Intersection> {
        return match self.children[i as usize] {
            Some (ref x) => {
                return x.revelles_proc_subtree(ro, r, max, min, tx0, ty0, tz0, tx1, ty1, tz1, a)
            }
            None =>  {
                return None
            }
        }
    }

    pub fn revelles_proc_subtree(&self, ro:&Ray, r:&Ray, max:f64, min:f64, tx0:f64, ty0:f64, tz0:f64, tx1:f64, ty1:f64, tz1:f64, a:u8) -> Option<Intersection> {
        //println!("- node hit");
        if tx1 < 0. || ty1 < 0. || tz1 < 0. {
            return None;
        }

        if self.is_leaf(){
            return self.items_intersection(ro, max, min);
        }

        let txm = 0.5 * (tx0 + tx1);
        let tym = 0.5 * (ty0 + ty1);
        let tzm = 0.5 * (tz0 + tz1);

        let mut curr_node = self.first_node_index(tx0,ty0,tz0,txm,tym,tzm);

        while curr_node < 8 { // TODO do-while?
            let mut intersection: Option<Intersection> = None;

            match curr_node {
                0 => {
                    intersection = self.revelles_proc_child(ro, r, max, min, tx0,ty0,tz0,txm,tym,tzm, a, 0);
                    curr_node = self.new_node(txm,4,tym,2,tzm,1);
                    },
                1 => { 
                    intersection = self.revelles_proc_child(ro, r, max, min, tx0,ty0,tzm,txm,tym,tz1, a, 1^a);
                    curr_node = self.new_node(txm,5,tym,3,tz1,8);
                    }
                2 => { 
                    intersection =self.revelles_proc_child(ro, r, max, min, tx0,tym,tz0,txm,ty1,tzm, a, 2^a);
                    curr_node = self.new_node(txm,6,ty1,8,tzm,3);
                    }
                3 => { 
                    intersection =self.revelles_proc_child(ro, r, max, min, tx0,tym,tzm,txm,ty1,tz1, a, 3^a);
                    curr_node = self.new_node(txm,7,ty1,8,tz1,8);
                    }
                4 => { 
                    intersection = self.revelles_proc_child(ro, r, max, min, txm,ty0,tz0,tx1,tym,tzm, a, 4^a);
                    curr_node = self.new_node(tx1,8,tym,6,tzm,5);
                    }
                5 => { 
                    intersection = self.revelles_proc_child(ro, r, max, min, txm,ty0,tzm,tx1,tym,tz1, a, 5^a);
                    curr_node = self.new_node(tx1,8,tym,7,tz1,8);
                    }
                6 => { 
                    intersection = self.revelles_proc_child(ro, r, max, min, txm,tym,tz0,tx1,ty1,tzm, a, 6^a);
                    curr_node = self.new_node(tx1,8,ty1,8,tzm,7);
                    }
                7 => { 
                    intersection = self.revelles_proc_child(ro, r, max, min, txm,tym,tzm,tx1,ty1,tz1, a, 7^a);
                    curr_node = 8;
                    }
                _ => {}
            }

            match intersection {
                Some(_) => { return intersection; },
                None => {}
            }
        }
        return None;
    }
    */
}

impl fmt::Display for OctreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut c = "".to_string();
        let mut p = "".to_string();

        for _ in -1 .. self.depth as i64{
            p += "  ";
        }
        p += "|-";

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
