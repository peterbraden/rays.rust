use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use std::f64;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;


struct IntersectionRange(RawIntersection, Option<RawIntersection>);
type CSGOp = fn(&IntersectionRange, &IntersectionRange) -> Option<RawIntersection>;

const EPSILON:f64 = 0.0000001; // f64::EPSILON is too big...

fn _reflect_normal(i: RawIntersection) -> RawIntersection {
    let mut j = i.clone();
    j.normal = i.normal * -1.;
    return j
}

// A U B
fn union(a: &IntersectionRange, b: &IntersectionRange) -> Option<RawIntersection> {
    if a.0.dist < b.0.dist {
        return Some(a.0)
    } else {
        return Some(b.0)
    }
}

// A - B
fn difference(a: &IntersectionRange, b: &IntersectionRange) -> Option<RawIntersection> {
    if a.0.dist < b.0.dist {
        return Some(a.0)
    }

    match b.1 {
        Some(bo) => {
            if bo.dist < a.0.dist {
                match a.1 {
                    Some(ao) => {
                        if ao.dist < bo.dist {
                            // contained in b
                            return None
                        } else {
                            return Some(_reflect_normal(bo)) 
                        }
                    },
                    None => {
                        // Infinite A
                        return Some(_reflect_normal(bo)) 
                    }
                }
            } else {
                // B is before A
                return Some(a.0)
            }
        },
        None => {
            // B is infinite
            return None;
        }
    }
}


// Primitive is an extended geometry to allow 'sets' of all intersections. 
pub struct Primitive {
    pub item: Box<dyn Geometry + Sync + Send>
}

impl Primitive {

    // We can simply move the ray each intersection to build a set
    // PBRT store both r0 and hitmin in ray, maybe that allows less mutation of mem,
    // but for now let's do it the simpler way.
    pub fn next_intersection(&self, r: &Ray, dist: f64) -> Option<RawIntersection> {
        let along = (dist + EPSILON) * r.rd.normalize(); 
        let r2 = Ray { ro: r.ro + along, rd: r.rd}; // TODO scale better (avoids rep) 
        match self.item.intersects(&r2) {
        
            Some(mut x) => {
                x.dist = (x.point - r.ro).norm(); 
                return Some(x)
            }, 
            None => { return None; }
        }
    }

    fn next_intersection_range(&self, r: &Ray, dist: f64) -> Option<IntersectionRange> {
        let in_a = self.next_intersection(r, dist);
        return match in_a {
            Some (x) => Some(IntersectionRange(x, self.next_intersection(r, x.dist))),
            None => None
        };
    }
    

    fn apply_csg(&self, r: &Ray, op: CSGOp, other: &Primitive) -> Option<RawIntersection> {
        let mut adist = 0.;
        let mut bdist = 0.;

        loop {
            let arange = match self.next_intersection_range(r, adist) {
                Some(arange) => arange,
                None => return None
            };
            adist += match arange.1 {
                Some(x) => x.dist,
                None => arange.0.dist + 1. // Bigger than entry to infinite obj
            }; 

            loop {
                let brange = match other.next_intersection_range(r, bdist) {
                    Some(brange) => brange,
                    None => break
                };
                bdist += match brange.1 {
                    Some(x) => x.dist,
                    None => brange.0.dist + 1. // Bigger than entry to infinite obj
                }; 

                match op(&arange, &brange) {
                    Some(x) => {return Some(x)}, // TODO BUG - what if b2 | a1 intersects different
                    None => {} // continue
                }
            }
        }
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

fn _find_intersect_iter(a: &Primitive, b: &Primitive, r: &Ray, adist: f64, bdist: f64) -> Option<RawIntersection> {
    match a.next_intersection_range(r, adist) {
        Some(ia) => {
            match b.next_intersection_range(r, bdist) {
                Some(ib) => {
                    if ia.0.dist < ib.0.dist {
                        // Enter A first
                        return Some(ia.0);
                    } else {
                        return _find_intersect_inside_b_iter(a, b, &ia, &ib, r);

                    }
                }
                None => {
                    // No B intersection
                    return Some(ia.0);
                }
            }
        },
        None => {
            // Would never have hit A
            return None
        }
    }
}

fn _find_intersect_inside_b_iter(
        a: &Primitive,
        b: &Primitive,
        arange: &IntersectionRange, 
        brange: &IntersectionRange,
        r: &Ray
    ) -> Option<RawIntersection> {

    // We are inside B
    // last intersections are inda, indb
    
    match brange.1 {
        None => {
            // That was the last B intersection, B stretches forever.
            return None
        },
        Some(nb) => {
            if nb.dist < arange.0.dist {
                // We exit b before hitting a => try again
                return _find_intersect_iter(a, b, r, arange.0.dist - EPSILON, nb.dist); // TODO
            }

            match arange.1 {
                Some (exit_a) => {
                    // We enter A inside B:
                    if exit_a.dist < nb.dist {
                        // If we exit A before exiting B
                        match a.next_intersection_range(r, exit_a.dist + EPSILON){
                            Some(narange) => {
                                // There is another A intersection.
                                print!("!!");
                                return _find_intersect_inside_b_iter(a, b, &narange, brange, r);
                            },
                            None => { return None }
                        }
                    } else {
                        // We're inside a inside b, the b exit is smaller than a exit
                        // Therefore as soon as we leave B we are still in A.
                        // Therefore B exit is intersection
                        return Some(_reflect_normal(nb));
                    }
                },
                None => {
                    // last A intersection, A is infinite, intersect at exit of B
                    return Some(_reflect_normal(nb));
                }
            }
        }
    }
}


impl Geometry for Difference {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return _find_intersect_iter(&self.a, &self.b, r, 0., 0.);
        //return self.a.apply_csg(r, difference, &self.b);
    }

    fn bounds(&self) -> BBox {
        return self.a.bounds(); // Can never be bigger than A
    }
}
