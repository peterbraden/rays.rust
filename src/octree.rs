//! # Octree
//!
//! A spatial subdivision for finding the nearest object along a ray without testing every
//! object. Following Andrew S. Glassner, "Space Subdivision for Fast Ray Tracing", IEEE Computer
//! Graphics and Applications 4(10), October 1984, doi:10.1109/MCG.1984.6429331
//! (<https://glassner.com/computer-graphics/graphics-research/space-subdivision-for-fast-ray-tracing/>).
//!
//! ## Construction
//!
//! The scene's bounding box is split into eight equal octants, recursively, until a node holds a
//! single item or reaches the maximum depth. An item is stored in every octant its bounding box
//! touches, so large items appear in many leaves. A node is not split if no octant would hold
//! fewer items than the node itself, since splitting would only copy the same items downwards.
//!
//! ## Traversal
//!
//! Glassner's insight is that a ray only needs testing against the items in the nodes it passes
//! through, taken in the order it passes through them: once a hit is found, nodes further along
//! cannot contain a nearer one. He steps from leaf to leaf, finding the next node the ray enters
//! after it misses everything in the current one.
//!
//! Here the same ordering is achieved recursively (`OctreeNode::nearest`): at each node, the
//! children the ray enters are sorted by entry distance and visited nearest first, and the search
//! stops at the first child that begins beyond the best hit so far. Because an item's reported
//! hit may lie in a later leaf than the one being visited, the stopping test compares node entry
//! distances with the best hit, rather than stopping at the first leaf with a hit.
//!
//! Each child's entry and exit come from a fresh slab test against its box. Juan Revelles,
//! Carlos Ureña and Miguel Lastra, "An Efficient Parametric Algorithm for Octree Traversal",
//! WSCG 2000, pp. 212-219 (<https://www.ugr.es/~curena/publ/2000-wscg>), avoid those tests: a
//! child's entry and exit parameters follow from its parent's and their midpoints, and a small
//! table gives the order in which the ray visits the children. (The authors note an erratum in
//! their Table 1: the bits for X and Z are swapped.) That is the natural next step if traversal
//! cost matters again.
//!
//! Not done: an item spanning several leaves is tested once per leaf the ray visits. Recording,
//! per ray, which items have been tested would avoid the repeats.

use crate::shapes::bbox::BBox;
use crate::na::{Vector3};
use std::sync::Arc;
use std::fmt;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::geometry::Geometry;
use rayon::prelude::*;

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

/// A ray prepared for repeated box tests.
struct TraversalRay {
    invrd: Vector3<f64>,
    /// Box tests give distances in multiples of the ray direction; this converts them to
    /// world-space distances, which items report.
    len: f64,
}

/// The nearest hit found so far in a traversal: the item's index and its intersection.
struct Nearest {
    dist: f64,
    hit: Option<(usize, RawIntersection)>,
}

impl OctreeNode {
    pub fn is_leaf(&self) -> bool {
        for i in 0..8 {
            if self.children[i as usize].is_some() { return false }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether any item in this subtree is hit between `min` and `max` along the ray. Unlike
    /// `nearest`, it needn't visit children in order or find the nearest hit: it stops at the
    /// first.
    fn any_hit<T: Geometry>(&self, items: &[Arc<T>], r: &Ray, ray: &TraversalRay, min: f64, max: f64) -> bool {
        if self.is_leaf() {
            return self.items.iter().any(|&i| {
                items[i].intersects_beyond(r, min).is_some_and(|x| x.dist >= min && x.dist < max)
            });
        }
        self.children.iter().flatten().any(|child| {
            child.bounds.entry_exit(&r.ro, &ray.invrd).is_some_and(|(entry, exit)| {
                entry.max(0.) * ray.len < max && exit * ray.len >= min
            }) && child.any_hit(items, r, ray, min, max)
        })
    }

    /// Find the nearest item hit in this subtree, at least `min` along the ray and nearer than
    /// `best`. Children are visited in the order the ray enters them, and the search stops at the
    /// first child that begins beyond the best hit so far: nothing in it, or in any later child,
    /// can be nearer.
    fn nearest<T: Geometry>(&self, items: &[Arc<T>], r: &Ray, ray: &TraversalRay, min: f64, best: &mut Nearest) {
        if self.is_leaf() {
            for &i in &self.items {
                if let Some(x) = items[i].intersects_beyond(r, min) {
                    if x.dist >= min && x.dist < best.dist {
                        best.dist = x.dist;
                        best.hit = Some((i, x));
                    }
                }
            }
            return;
        }

        // The children the ray passes through (beyond `min`), with their entry distances.
        let mut order: [(f64, &OctreeNode); 8] = [(0., self); 8];
        let mut count = 0;
        for child in self.children.iter().flatten() {
            if let Some((entry, exit)) = child.bounds.entry_exit(&r.ro, &ray.invrd) {
                if exit * ray.len >= min {
                    order[count] = (entry.max(0.) * ray.len, child);
                    count += 1;
                }
            }
        }
        order[..count].sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

        for &(entry, child) in &order[..count] {
            if entry > best.dist {
                break;
            }
            child.nearest(items, r, ray, min, best);
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
        // Each item's bounds are tested against every octant at every level; computing them
        // once (a triangle's takes 12 comparisons) was most of the cost of building a large mesh.
        let bounds: Vec<BBox> = items.iter().map(|x| x.bounds()).collect();
        Octree {
            root: Octree::<T>::create_node(0, max_depth, b, indices, &bounds),
            items,
        }
    }

    fn create_node(depth: usize, max_depth: usize, b: BBox, items: Vec<usize>, item_bounds: &[BBox]) -> OctreeNode{
        // Rust arrays suck - this defaults them to 'None'
        let mut children: [Option<Box<OctreeNode>>; 8] = Default::default();

        if depth < max_depth && items.len() > 1 {
            // Equal subdivision. Enhancement: Use different split.
            // Octants are filtered, and subtrees built, in parallel: building the ocean's tree
            // of 131072 triangles took 2 s on one thread.
            let octants: Vec<(BBox, Vec<usize>)> = (0..8u8).into_par_iter().map(|i| {
                let cbox = BBox::for_octant(i, &b);
                let inside = items.iter()
                                .copied()
                                .filter( |x| { cbox.intersects_bbox( &item_bounds[*x] ) } )
                                .collect::<Vec<usize>>();
                (cbox, inside)
            }).collect();

            // If no octant separates any items (eg. they all overlap the whole node), splitting
            // only duplicates them; recursing would create 8^depth identical nodes.
            let separates = octants.iter().any(|(_, inside)| inside.len() < items.len());

            if separates {
                let nodes: Vec<Option<Box<OctreeNode>>> = octants.into_par_iter().map(|(cbox, inside)| {
                    // Does child node have any objects in?
                    (!inside.is_empty()).then(|| {
                        Box::new(Octree::<T>::create_node(depth + 1, max_depth, cbox, inside, item_bounds))
                    })
                }).collect();
                for (child, node) in children.iter_mut().zip(nodes) {
                    *child = node;
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


    pub fn intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(&T, RawIntersection)> {
        self.closest_intersection(r, max, min).map(|(i, hit)| (&*self.items[i], hit))
    }

    /// Whether anything is hit between `min` and `max` along the ray: the predicate form of
    /// `intersection`, for shadow rays. Pharr, Jakob and Humphreys, "Physically Based
    /// Rendering", 4th ed., section 6.1.3, call this `IntersectP`: "often more efficient than a
    /// full intersection test", since the search can stop at the first hit.
    pub fn occluded(&self, r: &Ray, max: f64, min: f64) -> bool {
        let ray = TraversalRay { invrd: vec3_invert(r.rd), len: r.rd.norm() };
        match self.root.bounds.entry_exit(&r.ro, &ray.invrd) {
            Some((entry, exit)) if entry.max(0.) * ray.len < max && exit * ray.len >= min =>
                self.root.any_hit(&self.items, r, &ray, min, max),
            _ => false,
        }
    }

    fn closest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(usize, RawIntersection)> {
        let ray = TraversalRay { invrd: vec3_invert(r.rd), len: r.rd.norm() };
        let (entry, exit) = self.root.bounds.entry_exit(&r.ro, &ray.invrd)?;
        if entry.max(0.) * ray.len > max || exit * ray.len < min {
            return None;
        }
        let mut best = Nearest { dist: max, hit: None };
        self.root.nearest(&self.items, r, &ray, min, &mut best);
        best.hit
    }

    pub fn bounds(&self) -> BBox {
       self.root.bounds 
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn count_nodes(n: &OctreeNode) -> usize {
        1 + n.children.iter().flatten().map(|c| count_nodes(c)).sum::<usize>()
    }

    fn cube(min: f64, max: f64) -> Arc<BBox> {
        Arc::new(BBox::new(Vector3::new(min, min, min), Vector3::new(max, max, max)))
    }

    #[test]
    fn test_min_distance_skips_only_the_near_surface() {
        // One composite item with two parallel triangles. A shadow ray leaving the first (as
        // after a shadow bias, slightly behind it) must still find the second.
        use crate::shapes::csg::Union;
        use crate::shapes::triangle::Triangle;
        let tri = |z: f64| -> Box<dyn Geometry + Sync + Send> {
            Box::new(Triangle::new(Vector3::new(-1., -1., z), Vector3::new(1., -1., z), Vector3::new(0., 1., z)))
        };
        let items = vec![Arc::new(Union::new(vec![tri(0.), tri(1.)]))];
        let tree = Octree::new(2, BBox::new(Vector3::new(-2., -2., -2.), Vector3::new(2., 2., 2.)), &items);
        let r = Ray { ro: Vector3::new(0.1, 0.1, -1e-12), rd: Vector3::new(0., 0., 1.) };
        let hit = tree.raw_intersection(&r, 10., 0.001).expect("second triangle should occlude");
        assert!((hit.dist - 1.).abs() < 1e-6, "{}", hit.dist);
    }

    #[test]
    fn test_occluded_agrees_with_nearest() {
        use crate::shapes::sphere::Sphere;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng: StdRng = SeedableRng::from_seed([4; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        let items: Vec<Arc<Sphere>> = (0..60).map(|_| Arc::new(Sphere::new(
            Vector3::new(u(-10., 10.), u(-10., 10.), u(-10., 10.)), u(0.2, 3.)))).collect();
        let bounds = items.iter().fold(BBox::empty(), |b, s| b.union(&s.bounds()));
        let tree = Octree::new(6, bounds, &items);
        let mut blocked = 0;
        for _ in 0..5000 {
            let r = Ray {
                ro: Vector3::new(u(-15., 15.), u(-15., 15.), u(-15., 15.)),
                rd: Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)) * u(0.5, 2.),
            };
            let (max, min) = (u(0.5, 30.), u(0., 2.));
            let nearest = tree.closest_intersection(&r, max, min).is_some();
            assert_eq!(tree.occluded(&r, max, min), nearest, "ray {} max {} min {}", r, max, min);
            if nearest { blocked += 1; }
        }
        // A meaningful mix of blocked and clear rays.
        assert!(blocked > 1000 && blocked < 4000, "{}", blocked);
    }

    #[test]
    fn test_overlapping_items_do_not_subdivide_forever() {
        let items = vec![cube(-10., 10.), cube(-10., 10.)];
        let tree = Octree::new(5, BBox::new(Vector3::new(-10., -10., -10.), Vector3::new(10., 10., 10.)), &items);
        assert_eq!(count_nodes(&tree.root), 1);
    }

    #[test]
    fn test_separable_items_still_subdivide() {
        let items = vec![cube(-10., -9.), cube(9., 10.)];
        let tree = Octree::new(5, BBox::new(Vector3::new(-10., -10., -10.), Vector3::new(10., 10., 10.)), &items);
        assert!(count_nodes(&tree.root) > 1);
        let r = Ray { ro: Vector3::new(9.5, 9.5, -20.), rd: Vector3::new(0., 0., 1.) };
        let hit = tree.raw_intersection(&r, f64::INFINITY, 0.).unwrap();
        assert!((hit.dist - 29.).abs() < 1e-9, "{}", hit.dist);
    }

    /// Nearest hit by testing every item: what the octree must agree with.
    fn brute_force(items: &[Arc<crate::shapes::sphere::Sphere>], r: &Ray, max: f64, min: f64) -> Option<(usize, f64)> {
        items.iter().enumerate()
            .filter_map(|(i, s)| s.intersects_beyond(r, min).map(|x| (i, x.dist)))
            .filter(|(_, d)| *d >= min && *d < max)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }

    #[test]
    fn test_traversal_matches_brute_force() {
        use crate::shapes::sphere::Sphere;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng: StdRng = SeedableRng::from_seed([3; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);

        // Overlapping spheres of mixed sizes, so items span several leaves.
        let items: Vec<Arc<Sphere>> = (0..60).map(|_| Arc::new(Sphere::new(
            Vector3::new(u(-10., 10.), u(-10., 10.), u(-10., 10.)), u(0.2, 3.)))).collect();
        let bounds = items.iter().fold(BBox::empty(), |b, s| b.union(&s.bounds()));
        let tree = Octree::new(6, bounds, &items);
        assert!(count_nodes(&tree.root) > 1);

        for i in 0..5000 {
            // Origins inside and outside the bounds; some limited by max, some by min.
            let ro = Vector3::new(u(-15., 15.), u(-15., 15.), u(-15., 15.));
            let rd = Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)) * u(0.5, 2.);
            let r = Ray { ro, rd };
            let (max, min) = match i % 3 { 0 => (f64::INFINITY, 0.), 1 => (u(1., 20.), 0.), _ => (f64::INFINITY, u(0., 5.)) };
            let expected = brute_force(&items, &r, max, min);
            let got = tree.closest_intersection(&r, max, min).map(|(i, x)| (i, x.dist));
            match (expected, got) {
                (None, None) => (),
                (Some((_, e)), Some((_, g))) => assert!((e - g).abs() < 1e-9, "ray {}: {} vs {}", r, e, g),
                _ => panic!("ray {} max {} min {}: brute force {:?}, octree {:?}", r, max, min, expected, got),
            }
        }
    }
}
