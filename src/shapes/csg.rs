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
    let mut j = i;
    j.normal = i.normal * -1.;
    j
}

// A U B
fn union(a: &IntersectionRange, b: &IntersectionRange) -> Option<RawIntersection> {
    if a.0.dist < b.0.dist {
        Some(a.0)
    } else {
        Some(b.0)
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
                            None
                        } else {
                            Some(_reflect_normal(bo)) 
                        }
                    },
                    None => {
                        // Infinite A
                        Some(_reflect_normal(bo)) 
                    }
                }
            } else {
                // B is before A
                Some(a.0)
            }
        },
        None => {
            // B is infinite
            None
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
                Some(x)
            }, 
            None => { None}
        }
    }

    fn next_intersection_range(&self, r: &Ray, dist: f64) -> Option<IntersectionRange> {
        let in_a = self.next_intersection(r, dist);
        in_a.map(|x| IntersectionRange(x, self.next_intersection(r, x.dist)))
    }
    

    fn apply_csg(&self, r: &Ray, op: CSGOp, other: &Primitive) -> Option<RawIntersection> {
        let mut adist = 0.;
        let mut bdist = 0.;

        loop {
            let arange = self.next_intersection_range(r, adist)?;
            adist += match arange.1 {
                Some(x) => x.dist,
                None => arange.0.dist + 1. // Bigger than entry to infinite obj
            }; 

            while let Some(brange) = other.next_intersection_range(r, bdist) {
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
        self.item.intersects(r)
    }
    fn bounds(&self) -> BBox {
        self.item.bounds()
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
        let mut bounds = BBox::empty();
        let primitives: Vec<Arc<Primitive>> = geometries.into_iter().map(|g| {
            count += 1;
            bounds = bounds.union(&g.bounds());
            Arc::new(Primitive { item: g })
        }).collect();
        let tree = Octree::new(8, bounds, &primitives); 
    
        Union {
            items: tree,
            primitives: count,
        }
    }
}

impl Geometry for Union {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.items.raw_intersection(r, f64::INFINITY, 0f64)
    }
    fn bounds(&self) -> BBox {
        self.items.bounds()
    }

    fn primitives(&self) -> u64 { 
        self.primitives
    }
}


// A - B
pub struct Difference {
    pub a: Primitive,
    pub b: Primitive
}

/// Upper bound on surface crossings gathered per primitive, in case of numerical trouble.
const MAX_CROSSINGS: usize = 64;

/// Every surface crossing along the ray, nearest first, with world-space distances.
fn crossings(p: &Primitive, r: &Ray) -> Vec<RawIntersection> {
    let mut out = Vec::new();
    let mut dist = 0.;
    while out.len() < MAX_CROSSINGS {
        match p.next_intersection(r, dist) {
            Some(x) => {
                dist = x.dist;
                out.push(x);
            },
            None => break,
        }
    }
    out
}

/// Nearest point where the ray crosses the surface of A - B.
///
/// Whether the origin starts inside each (closed) primitive follows from the parity of its
/// crossings, so rays starting inside A or B - eg. secondary rays leaving a carved surface -
/// are handled. Walk the crossings in order and return the first that changes membership of
/// A - B. Surfaces of B face into B, so their normals are reversed.
fn difference_intersection(a: &Primitive, b: &Primitive, r: &Ray) -> Option<RawIntersection> {
    let ca = crossings(a, r);
    let cb = crossings(b, r);
    let mut in_a = ca.len() % 2 == 1;
    let mut in_b = cb.len() % 2 == 1;
    let (mut ia, mut ib) = (0, 0);

    loop {
        let was_inside = in_a && !in_b;
        let take_a = match (ca.get(ia), cb.get(ib)) {
            (None, None) => return None,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(x), Some(y)) => x.dist <= y.dist,
        };
        let hit = if take_a {
            in_a = !in_a;
            ia += 1;
            ca[ia - 1]
        } else {
            in_b = !in_b;
            ib += 1;
            _reflect_normal(cb[ib - 1])
        };
        if (in_a && !in_b) != was_inside {
            return Some(hit);
        }
    }
}


impl Geometry for Difference {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        difference_intersection(&self.a, &self.b, r)
    }

    fn bounds(&self) -> BBox {
        self.a.bounds()// Can never be bigger than A
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use crate::shapes::sphere::Sphere;

    /// Sphere of radius 2 at the origin, with a radius 1 bite taken out at x = 2.
    fn bitten_sphere() -> Difference {
        Difference {
            a: Primitive { item: Box::new(Sphere::new(Vector3::new(0., 0., 0.), 2.)) },
            b: Primitive { item: Box::new(Sphere::new(Vector3::new(2., 0., 0.), 1.)) },
        }
    }

    fn assert_hit(ro: Vector3<f64>, rd: Vector3<f64>, dist: f64, normal: Vector3<f64>) {
        let i = bitten_sphere().intersects(&Ray { ro, rd }).expect("expected a hit");
        assert!((i.dist - dist).abs() < 1e-6, "dist {} != {}", i.dist, dist);
        assert!((i.normal - normal).norm() < 1e-6, "normal {} != {}", i.normal, normal);
    }

    #[test]
    fn test_union_bounds_do_not_include_origin() {
        let far = |c: f64| -> Box<dyn Geometry + Sync + Send> { Box::new(Sphere::new(Vector3::new(c, c, c), 1.)) };
        let u = Union::new(vec![far(500.), far(503.)]);
        assert_eq!(u.bounds().min.coords, Vector3::new(499., 499., 499.));
        assert_eq!(u.bounds().max.coords, Vector3::new(504., 504., 504.));
    }

    #[test]
    fn test_hits_a_outside_b() {
        assert_hit(Vector3::new(0., 5., 0.), Vector3::new(0., -1., 0.), 3., Vector3::new(0., 1., 0.));
    }

    #[test]
    fn test_passes_through_b_into_cavity_wall() {
        // Enters B at x=3, would enter A at x=2 but that is inside B, so hits B's far side at x=1.
        assert_hit(Vector3::new(5., 0., 0.), Vector3::new(-1., 0., 0.), 4., Vector3::new(1., 0., 0.));
    }

    #[test]
    fn test_starting_inside_b() {
        // In the cavity (inside A and B), heading into the material.
        assert_hit(Vector3::new(1.5, 0., 0.), Vector3::new(-1., 0., 0.), 0.5, Vector3::new(1., 0., 0.));
    }

    #[test]
    fn test_starting_inside_material() {
        // Inside A - B, heading out through A.
        assert_hit(Vector3::new(-1., 0., 0.), Vector3::new(-1., 0., 0.), 1., Vector3::new(-1., 0., 0.));
        // Inside A - B, heading out through the cavity wall; the material lies at x < 1.
        assert_hit(Vector3::new(0., 0., 0.), Vector3::new(1., 0., 0.), 1., Vector3::new(1., 0., 0.));
    }

    #[test]
    fn test_miss() {
        assert!(bitten_sphere().intersects(&Ray { ro: Vector3::new(0., 5., 0.), rd: Vector3::new(0., 1., 0.) }).is_none());
    }
}
