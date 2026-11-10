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
//! children the ray enters are visited nearest first, and the search stops at the first child
//! that begins beyond the best hit so far. Because an item's reported hit may lie in a later leaf
//! than the one being visited, the stopping test compares node entry distances with the best
//! hit, rather than stopping at the first leaf with a hit.
//!
//! Children are found as in Juan Revelles, Carlos Ureña and Miguel Lastra, "An Efficient Parametric
//! Algorithm for Octree Traversal", WSCG 2000, pp. 212-219
//! (<https://www.ugr.es/~curena/publ/2000-wscg>): a child's slabs are halves of its parent's, so
//! its entry and exit follow from the parent's and where the ray crosses the parent's three
//! mid-planes, and the children come out in the order the ray enters them, with no box test or
//! sort (`OctreeNode::children_along`). This replaced a slab test of each of the eight children
//! and a sort, which were the largest cost of rendering mesh scenes: cornell-box and ocean-boat
//! render about 30% and 20% faster. The paper mirrors the ray so that its direction is positive
//! and visits children with a state table; here the mirroring is an XOR of the child index, and
//! the next child is found by comparing the current one's exit planes. (The authors note an
//! erratum in their Table 1: the bits for X and Z are swapped.)
//!
//! ## Triangle leaves
//!
//! When every item is a triangle (a mesh, or the ocean), the tree keeps a contiguous copy of the
//! triangles, and a leaf tests them four at a time with vector instructions
//! (`triangle::may_hit`) before testing the few that pass exactly. The copy is per triangle, not
//! per leaf: leaves share triangles (the ocean's hold each about 30 times), so a copy per leaf
//! would have been thirty times the size. On ocean-boat, with leaves of up to 16 or more triangles,
//! this cut rendering by 13%; scenes with small leaves are unchanged.
//!
//! Not done: an item spanning several leaves is tested once per leaf the ray visits. Recording,
//! per ray, which items have been tested would avoid the repeats.

use crate::shapes::bbox::BBox;
use crate::shapes::triangle::{EdgeTriangle, LANES, may_hit};
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
    /// If every item is a triangle, the triangles in one contiguous array, in the order of
    /// `items`, for leaves to test several at once (`triangle::may_hit`).
    triangles: Option<Vec<EdgeTriangle>>,
}

/// An octree's items, as its leaves test them.
struct Items<'a, T> {
    items: &'a [Arc<T>],
    triangles: Option<&'a [EdgeTriangle]>,
}

impl<T: Geometry> Items<'_, T> {
    /// Call `test` with each of `indices`, in order, until it returns true, skipping (when the
    /// items are triangles) those the ray cannot hit nearer than `limit(state)`. True if `test`
    /// did. The limit is read before each group of triangles, so `test` may lower it.
    fn find<S>(&self, indices: &[usize], r: &Ray, len: f64, state: &mut S,
               limit: impl Fn(&S) -> f64, mut test: impl FnMut(&mut S, usize) -> bool) -> bool {
        match self.triangles {
            Some(triangles) => indices.chunks(LANES).any(|group| {
                let mut hits = may_hit(triangles, group, r, len, limit(state));
                while hits != 0 {
                    let lane = hits.trailing_zeros() as usize;
                    hits &= hits - 1;
                    if test(state, group[lane]) {
                        return true;
                    }
                }
                false
            }),
            None => indices.iter().any(|&i| test(state, i)),
        }
    }
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

/// The most copies of a node's items its octants may hold between them for the node to be split
/// (see `Octree::create_node`). At 1200x1200 with 3 samples on 4 cores, ocean-boat rendered in
/// about 11 s with a limit of 2, 6.5 s with 2.5 to 3.5, and 9 s with 4; the cornell box, demo and
/// spheres-on-water scenes changed little between 2 and 4. Without the limit it took 13 s, and
/// the cornell box three times as long.
const MAX_SPLIT_COPIES: usize = 3;

/// A ray prepared for repeated box tests.
struct TraversalRay {
    ro: [f64; 3],
    invrd: [f64; 3],
    /// Box tests give distances in multiples of the ray direction; this converts them to
    /// world-space distances, which items report.
    len: f64,
    /// Octant bits (x = 1, y = 2, z = 4) of the axes the ray travels down. Along those axes the
    /// upper half of a node is the near one.
    mirror: u8,
}

impl TraversalRay {
    fn new(r: &Ray) -> TraversalRay {
        let invrd = vec3_invert(r.rd);
        let mirror = (0..3).filter(|&a| invrd[a].is_sign_negative()).fold(0, |m, a| m | 1 << a);
        TraversalRay { ro: [r.ro.x, r.ro.y, r.ro.z], invrd: [invrd.x, invrd.y, invrd.z], len: r.rd.norm(), mirror }
    }

    /// Where the ray crosses each axis' slab of a box: where it meets the near plane and the far
    /// plane, in multiples of the ray direction.
    fn span(&self, b: &BBox) -> Span {
        let mut s = Span { near: [0.; 3], far: [0.; 3] };
        for a in 0..3 {
            let t1 = (b.min[a] - self.ro[a]) * self.invrd[a];
            let t2 = (b.max[a] - self.ro[a]) * self.invrd[a];
            (s.near[a], s.far[a]) = if t1.is_nan() || t2.is_nan() {
                // Parallel to the slab and starting on one of its faces (0 * inf): inside the
                // closed slab, as `BBox::entry_exit` treats it.
                (f64::NEG_INFINITY, f64::INFINITY)
            } else {
                (t1.min(t2), t1.max(t2))
            };
        }
        s
    }
}

/// The ray's parameter where it enters and leaves a node's slab along each axis, nearest plane
/// first. The node's own entry and exit are the latest entry and the earliest exit.
#[derive(Clone, Copy)]
struct Span {
    near: [f64; 3],
    far: [f64; 3],
}

impl Span {
    fn entry(&self) -> f64 { self.near[0].max(self.near[1]).max(self.near[2]) }
    fn exit(&self) -> f64 { self.far[0].min(self.far[1]).min(self.far[2]) }
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

    /// Call `visit` with each child the ray passes through, in the order it enters them, with
    /// the child's span, until `visit` returns false. Following Revelles, Urena and Lastra (see
    /// the module docs): a child's slabs are halves of its parent's, so its span needs only where
    /// the ray crosses the parent's three mid-planes, not a fresh box test. The ray starts in the
    /// child on the near side of each mid-plane it has not crossed by the node's entry, and moves
    /// to the next by crossing whichever of the current child's exit planes comes first: a
    /// mid-plane leads to the neighbouring child, an outer plane leaves the node. It visits at
    /// most four children.
    fn children_along<F: FnMut(&OctreeNode, &Span) -> bool>(&self, ray: &TraversalRay, span: &Span, mut visit: F) {
        let mut mid = [0.; 3];
        for (a, m) in mid.iter_mut().enumerate() {
            // The same split as `BBox::for_octant`.
            let plane = self.bounds.min[a] + (self.bounds.max[a] - self.bounds.min[a]) * 0.5;
            let t = (plane - ray.ro[a]) * ray.invrd[a];
            // NaN: parallel to the mid-plane and lying in it, so in both halves. Stay in the
            // near one (an infinite crossing is never reached).
            *m = if t.is_nan() { f64::INFINITY } else { t };
        }

        // Which side of each mid-plane the ray is on: 0 near, 1 far.
        let entry = span.entry();
        let mut side = [0u8; 3];
        for a in 0..3 {
            if mid[a] < entry { side[a] = 1 }
        }

        loop {
            let mut child_span = *span;
            for a in 0..3 {
                if side[a] == 0 { child_span.far[a] = mid[a] } else { child_span.near[a] = mid[a] }
            }
            let octant = (side[0] | side[1] << 1 | side[2] << 2) ^ ray.mirror;
            if let Some(child) = &self.children[octant as usize]
                && child_span.entry() <= child_span.exit() && child_span.exit() >= 0.
                && !visit(child, &child_span) {
                return;
            }

            // Cross the nearest exit plane of this child.
            let mut axis = 0;
            for a in 1..3 {
                if child_span.far[a] < child_span.far[axis] { axis = a }
            }
            if side[axis] == 1 || child_span.far[axis] == f64::INFINITY {
                return;
            }
            side[axis] = 1;
        }
    }

    /// Whether any item in this subtree is hit between `min` and `max` along the ray. Unlike
    /// `nearest`, it needn't find the nearest hit: it stops at the first.
    fn any_hit<T: Geometry>(&self, items: &Items<T>, r: &Ray, ray: &TraversalRay, span: &Span, min: f64, max: f64) -> bool {
        if self.is_leaf() {
            return items.find(&self.items, r, ray.len, &mut (), |_| max, |_, i| {
                items.items[i].intersects_beyond(r, min).is_some_and(|x| x.dist >= min && x.dist < max)
            });
        }
        let mut found = false;
        self.children_along(ray, span, |child, child_span| {
            if child_span.entry().max(0.) * ray.len >= max {
                // Children are visited in order, so the rest begin beyond `max` too.
                return false;
            }
            found = child_span.exit() * ray.len >= min && child.any_hit(items, r, ray, child_span, min, max);
            !found
        });
        found
    }

    /// Find the nearest item hit in this subtree, at least `min` along the ray and nearer than
    /// `best`. Children are visited in the order the ray enters them, and the search stops at the
    /// first child that begins beyond the best hit so far: nothing in it, or in any later child,
    /// can be nearer.
    fn nearest<T: Geometry>(&self, items: &Items<T>, r: &Ray, ray: &TraversalRay, span: &Span, min: f64, best: &mut Nearest) {
        if self.is_leaf() {
            items.find(&self.items, r, ray.len, best, |best| best.dist, |best, i| {
                if let Some(x) = items.items[i].intersects_beyond(r, min)
                    && x.dist >= min && x.dist < best.dist {
                    best.dist = x.dist;
                    best.hit = Some((i, x));
                }
                false
            });
            return;
        }

        self.children_along(ray, span, |child, child_span| {
            if child_span.entry().max(0.) * ray.len > best.dist {
                return false;
            }
            if child_span.exit() * ray.len >= min {
                child.nearest(items, r, ray, child_span, min, best);
            }
            true
        });
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
        let triangles = items.iter()
            .map(|x| x.vertices().map(|v| EdgeTriangle::new(&v)))
            .collect::<Option<Vec<EdgeTriangle>>>()
            .filter(|t| !t.is_empty());
        Octree {
            root: Octree::<T>::create_node(0, max_depth, b, indices, &bounds),
            items,
            triangles,
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

            // Splitting stops when it would copy items more than it separates them:
            // - If no octant separates any items (eg. they all overlap the whole node), splitting
            //   only duplicates them; recursing would create 8^depth identical nodes.
            // - If the octants between them hold more than `MAX_SPLIT_COPIES` times the node's
            //   items, most items straddle the split. Along a shared edge, or across a surface
            //   flatter than the node (the ocean, split in y as well as x and z), every level
            //   multiplied the items and the nodes a ray passes through, while barely shrinking
            //   any leaf: a 12-triangle box became 400000 leaves, and the ocean's 131072
            //   triangles 41 million leaf entries.
            let copies: usize = octants.iter().map(|(_, inside)| inside.len()).sum();
            let separates = octants.iter().any(|(_, inside)| inside.len() < items.len())
                && copies <= MAX_SPLIT_COPIES * items.len();

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
        let ray = TraversalRay::new(r);
        let span = ray.span(&self.root.bounds);
        let (entry, exit) = (span.entry(), span.exit());
        if exit < 0. || entry > exit || entry.max(0.) * ray.len >= max || exit * ray.len < min {
            return false;
        }
        self.root.any_hit(&self.leaf_items(), r, &ray, &span, min, max)
    }

    fn closest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(usize, RawIntersection)> {
        let ray = TraversalRay::new(r);
        let span = ray.span(&self.root.bounds);
        let (entry, exit) = (span.entry(), span.exit());
        if exit < 0. || entry > exit || entry.max(0.) * ray.len > max || exit * ray.len < min {
            return None;
        }
        let mut best = Nearest { dist: max, hit: None };
        self.root.nearest(&self.leaf_items(), r, &ray, &span, min, &mut best);
        best.hit
    }

    fn leaf_items(&self) -> Items<'_, T> {
        Items { items: &self.items, triangles: self.triangles.as_deref() }
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
    fn test_axis_parallel_rays_on_mid_planes_match_brute_force() {
        // Rays with zero direction components, starting on node mid-planes and running along
        // them and along item faces: the cases where the traversal's crossings are infinite or
        // NaN. Everything lies on a grid of whole numbers, which every mid-plane is part of.
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng: StdRng = SeedableRng::from_seed([5; 32]);
        let mut int = |a: i32, b: i32| rng.gen_range(a, b + 1) as f64;
        let items: Vec<Arc<BBox>> = (0..80).map(|_| {
            let (x, y, z) = (int(-16, 14), int(-16, 14), int(-16, 14));
            Arc::new(BBox::new(Vector3::new(x, y, z), Vector3::new(x + int(0, 2), y + int(0, 2), z + int(0, 2))))
        }).collect();
        let tree = Octree::new(5, BBox::new(Vector3::new(-16., -16., -16.), Vector3::new(16., 16., 16.)), &items);
        assert!(count_nodes(&tree.root) > 1);

        for _ in 0..20000 {
            let ro = Vector3::new(int(-20, 20), int(-20, 20), int(-20, 20));
            // Components of -1, -0, 0 and 1: -0 gives an inverse of -inf, which mirrors the axis.
            let mut component = || [-1., -0., 0., 1.][int(0, 3) as usize];
            let mut rd = Vector3::new(component(), component(), component());
            if rd.iter().all(|c| *c == 0.) { rd.x = 1. }
            let r = Ray { ro, rd };
            let expected = items.iter()
                .filter_map(|b| b.intersects_beyond(&r, 0.).map(|x| x.dist))
                .filter(|d| *d >= 0.)
                .min_by(f64::total_cmp);
            let got = tree.closest_intersection(&r, f64::INFINITY, 0.).map(|(_, x)| x.dist);
            match (expected, got) {
                (None, None) => (),
                (Some(e), Some(g)) => assert!((e - g).abs() < 1e-9, "ray {}: {} vs {}", r, e, g),
                _ => panic!("ray {}: brute force {:?}, octree {:?}", r, expected, got),
            }
            assert_eq!(tree.occluded(&r, f64::INFINITY, 0.), expected.is_some(), "ray {}", r);
        }
    }

    #[test]
    fn test_triangle_leaves_match_brute_force() {
        // Triangles are tested in groups by `may_hit` before the exact test; the result must be
        // exactly the exact test's. A rough surface of small triangles, as the ocean is, so that
        // leaves hold several, and rays from above it and grazing along it.
        use crate::shapes::triangle::Triangle;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng: StdRng = SeedableRng::from_seed([6; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        let n = 24;
        let heights: Vec<f64> = (0..(n + 1) * (n + 1)).map(|_| u(-0.2, 0.2)).collect();
        let p = |x: usize, z: usize| Vector3::new(x as f64, heights[z * (n + 1) + x], z as f64);
        let mut items = vec![];
        for x in 0..n {
            for z in 0..n {
                items.push(Arc::new(Triangle::new(p(x, z), p(x + 1, z), p(x, z + 1))));
                items.push(Arc::new(Triangle::new(p(x + 1, z), p(x + 1, z + 1), p(x, z + 1))));
            }
        }
        let bounds = items.iter().fold(BBox::empty(), |b, t| b.union(&t.bounds()));
        let tree = Octree::new(8, bounds, &items);
        assert!(tree.triangles.is_some());
        fn largest_leaf(n: &OctreeNode) -> usize {
            n.children.iter().flatten().map(|c| largest_leaf(c)).max().unwrap_or(n.items.len())
        }
        assert!(largest_leaf(&tree.root) > crate::shapes::triangle::LANES, "{}", largest_leaf(&tree.root));

        let mut hits = 0;
        for i in 0..20000 {
            let ro = Vector3::new(u(-5., 29.), u(-0.3, 4.), u(-5., 29.));
            let rd = Vector3::new(u(-1., 1.), if i % 2 == 0 { u(-1., 0.2) } else { u(-0.02, 0.02) }, u(-1., 1.)) * u(0.5, 2.);
            let r = Ray { ro, rd };
            let (max, min) = match i % 3 { 0 => (f64::INFINITY, 0.), 1 => (u(1., 30.), 0.), _ => (f64::INFINITY, u(0., 5.)) };
            let expected = items.iter().enumerate()
                .filter_map(|(i, t)| t.intersects_beyond(&r, min).map(|x| (i, x.dist)))
                .filter(|(_, d)| *d >= min && *d < max)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            let got = tree.closest_intersection(&r, max, min).map(|(i, x)| (i, x.dist));
            assert_eq!(got.map(|x| x.1), expected.map(|x| x.1), "ray {} max {} min {}", r, max, min);
            assert_eq!(tree.occluded(&r, max, min), expected.is_some(), "ray {}", r);
            if expected.is_some() { hits += 1 }
        }
        assert!(hits > 2000, "{}", hits);
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
