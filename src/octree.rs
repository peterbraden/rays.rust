use bbox::BBox;
use na::{Vec3, Norm};
use sceneobject::SceneObject;
use std::rc::Rc;
use std::fmt;
use ray::Ray;
use intersection::Intersection;

pub struct OctreeNode {
    depth: i64,
    bounds: BBox,
    mid: Vec3<f64>,
    // Octree structure:
    children: [Option<Box<OctreeNode>>; 8],
    items: Vec<Rc<SceneObject>>,
}

impl OctreeNode {

    //
    // Create a new node, and subdivide into further nodes up until max_depth
    // or until number of children objects is 0.
    //
    pub fn new(depth: i64, max_depth: i64, b: BBox, items: &Vec<Rc<SceneObject>>) -> OctreeNode {

        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<OctreeNode>>; 8] = Default::default();

        for i in 0..8 {
            // Does child node have any objects in?
            if depth < max_depth && items.len() > 1 {
                // Equal subdivision. Enhancement: Use different split.
                let cbox = BBox::for_octant(i, &b);
                let item_iter = items.into_iter();
                let inside = item_iter
                                    .cloned()
                                    //.filter( |x| { cbox.intersects_bbox( &x.bounds() ) } )
                                    .collect::<Vec<Rc<SceneObject>>>();

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


    pub fn new_node(&self, txm:f64, x:u8, tym:f64, y:u8, tzm:f64, z:u8) -> u8{
        if txm < tym {
            if txm < tzm {return x;}  // YZ plane
        }
        else{
            if tym < tzm {return y;} // XZ plane
        }
       return z; // XY plane;
    }

    pub fn intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {

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
            ro: Vec3::new(ro0, ro1, ro2),
            rd: Vec3::new(rd0, rd1, rd2),
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
            return self.proc_subtree(r, &rn, max, min, tx0, ty0, tz0, tx1, ty1, tz1, a);
        }

        //println!("- magnitude miss {} {} {}", r, tz0.max(tx0.max(ty0)), tz1.min(tx1.min(ty1)));
        return None;
    }
    
    pub fn proc_child(&self, ro: &Ray, r:&Ray, max:f64, min:f64, tx0:f64, ty0:f64, tz0:f64, tx1:f64, ty1:f64, tz1:f64, a:u8, i: u8)-> Option<Intersection> {
        return match self.children[i as usize] {
            Some (ref x) => {
                return x.proc_subtree(ro, r, max, min, tx0, ty0, tz0, tx1, ty1, tz1, a)
            }
            None =>  {
                return None
            }
        }
    }

    pub fn proc_subtree(&self, ro:&Ray, r:&Ray, max:f64, min:f64, tx0:f64, ty0:f64, tz0:f64, tx1:f64, ty1:f64, tz1:f64, a:u8) -> Option<Intersection> {
        //println!("- node hit");
        if tx1 < 0. || ty1 < 0. || tz1 < 0. {
            return None;
        }

        if self.is_leaf(){
            println!("- leaf hit: {} {} {} {}", self.items.len(), ro, max, min);
            match self.items_intersection(ro, max, min) {
                Some(_) => println!("- - intersects "),
                None => println!("- - none"),
            
            }
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
                    intersection = self.proc_child(ro, r, max, min, tx0,ty0,tz0,txm,tym,tzm, a, 0);
                    curr_node = self.new_node(txm,4,tym,2,tzm,1);
                    },
                1 => { 
                    intersection = self.proc_child(ro, r, max, min, tx0,ty0,tzm,txm,tym,tz1, a, 1^a);
                    curr_node = self.new_node(txm,5,tym,3,tz1,8);
                    }
                2 => { 
                    intersection =self.proc_child(ro, r, max, min, tx0,tym,tz0,txm,ty1,tzm, a, 2^a);
                    curr_node = self.new_node(txm,6,ty1,8,tzm,3);
                    }
                3 => { 
                    intersection =self.proc_child(ro, r, max, min, tx0,tym,tzm,txm,ty1,tz1, a, 3^a);
                    curr_node = self.new_node(txm,7,ty1,8,tz1,8);
                    }
                4 => { 
                    intersection = self.proc_child(ro, r, max, min, txm,ty0,tz0,tx1,tym,tzm, a, 4^a);
                    curr_node = self.new_node(tx1,8,tym,6,tzm,5);
                    }
                5 => { 
                    intersection = self.proc_child(ro, r, max, min, txm,ty0,tzm,tx1,tym,tz1, a, 5^a);
                    curr_node = self.new_node(tx1,8,tym,7,tz1,8);
                    }
                6 => { 
                    intersection = self.proc_child(ro, r, max, min, txm,tym,tz0,tx1,ty1,tzm, a, 6^a);
                    curr_node = self.new_node(tx1,8,ty1,8,tzm,7);
                    }
                7 => { 
                    intersection = self.proc_child(ro, r, max, min, txm,tym,tzm,tx1,ty1,tz1, a, 7^a);
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



    // Iterate through items. Should only do on leaf if multiple items.
    pub fn items_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection> {
        let mut cdist = max;
        let mut closest = None;
        for o in &self.items {
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

