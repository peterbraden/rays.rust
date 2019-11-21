use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use octree::Octree;
use std::f64;
use shapes::geometry::Geometry;
use std::sync::Arc;


// Primitive is an extended geometry to allow 'sets' of all intersections. 
pub struct Primitive {
    pub item: Box<dyn Geometry + Sync + Send>
}

impl Primitive {
    pub fn intersection_set(&self, r: &Ray) -> Vec<RawIntersection> {
        // We can simply move the ray each intersection to build a set
        // PBRT store both r0 and hitmin in ray, maybe that allows less mutation of mem,
        // but for now let's do it the simpler way.
        // TODO what if we are inside?
        let mut intersections = Vec::new(); 
        let mut closest = self.item.intersects(r);

        loop {
            match closest {
                Some(x) => {
                    let r2 = Ray { ro: x.point * 1.00001, rd: r.rd}; // TODO scale better (avoids rep) 
                    intersections.push(x);
                    closest = self.item.intersects(&r2);
                },
                None => { break; }
            }
        }
        return intersections;
    }
}

impl Geometry for Primitive {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.item.intersects(r);
    }
    fn bounds(&self) -> BBox {
        return self.item.bounds();
    }
}


// Item[0] | Item[1] | Item[n...]
//
// Union builds an Octree of contained items, therefore provides efficient intersection of 
// many items.
pub struct Union {
    items: Octree<Primitive>,
    primitives: u64,
}

impl Union {
    pub fn new (geometries: Vec<Box<dyn Geometry + Sync + Send>>) -> Union {
        let mut count = 0;
        let mut bounds = BBox::min() ;
        let primitives = geometries.into_iter().map(|g| {
            count += 1;
            bounds = bounds.union(&g.bounds());
            Arc::new(Primitive { item: g })
        }).collect();
        let tree = Octree::new(8, bounds, &primitives); 
    
        return Union {
            items: tree,
            primitives: count,
        }
    }
}

impl Geometry for Union {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.items.raw_intersection(r, f64::INFINITY, 0f64);
    }
    fn bounds(&self) -> BBox {
        return self.items.bounds();
    }

    fn primitives(&self) -> u64 { 
        return self.primitives;
    }
}

// A - B
pub struct Difference {
    pub a: Primitive,
    pub b: Primitive
}

fn _find_intersect(ia: &Vec<RawIntersection>, ib: &Vec<RawIntersection>, inda: usize, indb: usize) -> Option<RawIntersection> {
    if inda + 1 >= ia.len() { // Out of range
        return None
    }
    if indb + 1  >= ib.len() {
        return Some(ia[inda]);
    }

    if ia[inda].dist < ib[indb].dist {
        // Enter A first
        return Some(ia[inda]);
    } else {
        return _find_intersect_inside_b(&ia, &ib, inda, indb);
    }
}

fn _find_intersect_inside_b(ia: &Vec<RawIntersection>, ib:& Vec<RawIntersection>, inda: usize, indb: usize) -> Option<RawIntersection> {
    // We are inside B
    // last intersections are inda, indb
    if inda >= ia.len() - 1 {
        return None
    }

    if ib.len() - 1 <= indb + 1 {
        // That was the last B intersection, B stretches forever.
        return None
    } 

    if ib[indb + 1].dist < ia[inda].dist {
        // We exit b before hitting a => try again
        return _find_intersect(&ia, &ib, inda, indb + 2);
    }

    // We enter a inside b.
    if ia.len() - 1 <= inda + 1 {
        // last A intersection, A is infinite, intersect at exit of B
        return Some(ib[indb + 1])
    }

    // There is another A intersection.
    if ia[inda + 1].dist < ib[indb + 1].dist {
        // If we exit A before exiting B
        return _find_intersect_inside_b(&ia, &ib, inda + 2, indb)
    } else {
        // We're inside a inside b, the b exit is smaller than a exit
        // Therefore as soon as we leave B we are still in A.
        // Therefore B exit is intersection
        return Some(ib[indb + 1])
    }
}

impl Geometry for Difference {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let ia = self.a.intersection_set(r);
        let ib = self.b.intersection_set(r);
        return _find_intersect(&ia, &ib, 0, 0);
    }

    fn bounds(&self) -> BBox {
        return self.a.bounds(); // Can never be bigger than A
    }
}
