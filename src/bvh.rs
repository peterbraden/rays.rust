//! # Bounding volume hierarchy
//!
//! A tree of bounding boxes over the primitives: each node's box encloses everything below it,
//! so a ray that misses a box skips its whole subtree. Where the octree this replaced divides
//! *space* into fixed octants, and so stores an item in every octant it touches, a
//! bounding volume hierarchy divides the *primitives* into two groups at each node: every
//! primitive is in exactly one leaf, and the boxes adapt to where the primitives are, not to a
//! fixed grid. Boxes may overlap, which is the price.
//!
//! ## History
//!
//! - Steven M. Rubin and Turner Whitted, "A 3-Dimensional Representation for Fast Rendering of
//!   Complex Scenes", SIGGRAPH 1980, Computer Graphics 14(3), 110-116, doi:10.1145/965105.807479:
//!   the first hierarchy of bounding volumes for ray tracing, built by hand.
//! - Timothy L. Kay and James T. Kajiya, "Ray Tracing Complex Scenes", SIGGRAPH 1986, Computer
//!   Graphics 20(4), 269-278, doi:10.1145/15886.15916: bounding volumes made of slabs (pairs of
//!   parallel planes; an axis-aligned box is three slabs), and the slab test this module uses to
//!   intersect a ray with a box, via `BBox::entry_exit`.
//! - Jeffrey Goldsmith and John Salmon, "Automatic Creation of Object Hierarchies for Ray
//!   Tracing", IEEE Computer Graphics and Applications 7(5), 14-20, 1987,
//!   doi:10.1109/MCG.1987.276983: the first automatic construction, estimating the chance that
//!   a ray hits a bounding volume from its surface area.
//! - J. David MacDonald and Kellogg S. Booth, "Heuristics for Ray Tracing Using Space
//!   Subdivision", The Visual Computer 6(3), 153-166, 1990, doi:10.1007/BF01911006: the
//!   surface area heuristic as a cost model for choosing splits (for kd-trees).
//! - Ingo Wald, "On Fast Construction of SAH-based Bounding Volume Hierarchies", IEEE Symposium
//!   on Interactive Ray Tracing 2007, 33-40, doi:10.1109/RT.2007.4342588: evaluating the SAH
//!   at a few bucket boundaries rather than at every primitive, which this follows.
//! - Matt Pharr, Wenzel Jakob and Greg Humphreys, "Physically Based Rendering", 4th ed.,
//!   section 7.3 (<https://pbr-book.org/4ed/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies>):
//!   the construction, constants and flattened layout used here.
//!
//! ## The surface area heuristic (PBR section 7.3.2)
//!
//! Splitting a node A into children B and C costs, for a ray reaching A (PBR eq. 7.1),
//!
//!   c(B, C) = t_trav + p_B sum_{i in B} t_isect(i) + p_C sum_{i in C} t_isect(i),
//!
//! where t_trav is the cost of visiting a node and t_isect of testing a primitive. For a convex
//! volume B inside a convex volume A, the probability that a random ray through A also passes
//! through B is the ratio of their surface areas, p(B | A) = s_B / s_A. Making a leaf instead
//! costs sum_i t_isect(i). Taking t_isect = 1 and t_trav = 1/2, as PBR does, a split costs
//! 1/2 + (n_B s_B + n_C s_C) / s_A, a leaf n; split when that is cheaper. The heuristic is
//! greedy, minimising each node's cost as though its children were leaves.
//!
//! Candidate splits: the axis along which the primitives' centroids spread furthest, divided
//! into 12 equal buckets by centroid, with a split considered at each of the 11 boundaries.
//!
//! ## Layout and traversal (PBR sections 7.3.4, 7.3.5)
//!
//! The tree is flattened into one array in depth-first order: a node's first child follows it,
//! and it records where its second child is. Leaves record a contiguous range of primitives; the
//! builder returns the order to store them in, so the caller's array lines up with the leaves.
//! Traversal is a loop with a small stack rather than recursion. At each interior node it
//! visits first the child on the near side of the split, judged by the sign of the ray's
//! direction along the split axis, and pushes the other; boxes that begin beyond the nearest
//! hit found so far are skipped.
//!
//! ## Compared with the octree
//!
//! The renderer previously used an octree, after Andrew S. Glassner, "Space Subdivision for Fast
//! Ray Tracing", IEEE Computer Graphics and Applications 4(10), 1984,
//! doi:10.1109/MCG.1984.6429331; it is in the history as `src/octree.rs`, with a discussion of
//! Revelles, Ureña and Lastra's parametric traversal (WSCG 2000). The octree tested a primitive spanning several octants once per octant visited (the
//! "mailboxing" problem Glassner noted), stored such primitives several times, and could not
//! split items whose boxes overlapped the whole node. Here each primitive is stored and tested
//! once per ray at most, and overlapping items are separated by their centroids.
use crate::intersection::RawIntersection;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;

/// Buckets per split axis (PBR: "12 buckets usually work well in practice").
const BUCKETS: usize = 12;
/// Relative cost of visiting a node, against 1 for testing a primitive.
const TRAVERSAL_COST: f64 = 0.5;
/// Leaves never hold more than this; larger groups are split even where the SAH would not.
const MAX_LEAF: usize = 8;
/// Depth at which a node becomes a leaf regardless, so the traversal stack can be fixed.
const MAX_DEPTH: usize = 60;

#[derive(Clone, Copy, Debug)]
struct Node {
    bounds: BBox,
    /// A leaf: the first primitive. An interior node: the index of its second child.
    offset: u32,
    /// Primitives in a leaf; 0 for an interior node.
    count: u32,
    /// The split axis of an interior node.
    axis: u8,
}

/// The hierarchy itself, over primitives identified by their position in the order returned by
/// `Tree::build`.
#[derive(Clone, Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

struct Primitive {
    index: usize,
    bounds: BBox,
    centroid: Vector3<f64>,
}

fn surface_area(b: &BBox) -> f64 {
    let d = b.size();
    2. * (d.x * d.y + d.y * d.z + d.z * d.x)
}

/// A ray prepared for repeated box tests.
struct Traversal {
    invrd: Vector3<f64>,
    /// Box tests give distances in multiples of the ray direction; this converts them to the
    /// world-space distances primitives report.
    len: f64,
    /// Whether the ray runs towards negative x, y, z: the near child is then the second.
    negative: [bool; 3],
}

impl Traversal {
    fn new(r: &Ray) -> Traversal {
        let invrd = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        Traversal { invrd, len: r.rd.norm(), negative: [invrd.x < 0., invrd.y < 0., invrd.z < 0.] }
    }

    /// Whether the ray passes through `b` somewhere in [min, max).
    fn enters(&self, r: &Ray, b: &BBox, min: f64, max: f64) -> bool {
        b.entry_exit(&r.ro, &self.invrd)
            .is_some_and(|(entry, exit)| entry.max(0.) * self.len <= max && exit * self.len >= min)
    }
}

impl Tree {
    /// Build over primitives with these bounds. Returns the tree, and the order to store the
    /// primitives in: position i of the order holds the index (into `bounds`) of the primitive
    /// the tree calls i.
    pub fn build(bounds: &[BBox]) -> (Tree, Vec<usize>) {
        let mut prims: Vec<Primitive> = bounds.iter().enumerate()
            .map(|(index, b)| Primitive { index, bounds: *b, centroid: b.mid() })
            .collect();
        let mut nodes = Vec::with_capacity(2 * bounds.len().max(1));
        let mut order = Vec::with_capacity(bounds.len());
        if prims.is_empty() {
            return (Tree { nodes }, order);
        }
        Tree::build_node(&mut prims, 0, &mut nodes, &mut order);
        (Tree { nodes }, order)
    }

    fn build_node(prims: &mut [Primitive], depth: usize, nodes: &mut Vec<Node>, order: &mut Vec<usize>) {
        let bounds = prims.iter().fold(BBox::empty(), |b, p| b.union(&p.bounds));
        let centroids = prims.iter().fold(BBox::empty(), |b, p| b.union_point(&p.centroid));
        let extent = centroids.size();
        let axis = if extent.x >= extent.y && extent.x >= extent.z { 0 } else if extent.y >= extent.z { 1 } else { 2 };
        let (lo, width) = (centroids.min[axis], extent[axis]);

        let make_leaf = |prims: &[Primitive], nodes: &mut Vec<Node>, order: &mut Vec<usize>| {
            nodes.push(Node { bounds, offset: order.len() as u32, count: prims.len() as u32, axis: 0 });
            order.extend(prims.iter().map(|p| p.index));
        };

        // One primitive, centroids that can't be told apart along any axis, or too deep.
        if prims.len() == 1 || width.is_nan() || width <= 0. || depth >= MAX_DEPTH {
            return make_leaf(prims, nodes, order);
        }

        let bucket = |c: &Vector3<f64>| (((c[axis] - lo) / width * BUCKETS as f64) as usize).min(BUCKETS - 1);
        let mut counts = [0usize; BUCKETS];
        let mut boxes = [BBox::empty(); BUCKETS];
        for p in prims.iter() {
            let b = bucket(&p.centroid);
            counts[b] += 1;
            boxes[b] = boxes[b].union(&p.bounds);
        }

        // Cost of splitting after each bucket, n_B s_B + n_C s_C, by a forward and a backward
        // scan (PBR section 7.3.2). Splits with an empty side aren't splits.
        let mut costs = [f64::INFINITY; BUCKETS - 1];
        let (mut below, mut n_below) = (BBox::empty(), 0);
        for i in 0 .. BUCKETS - 1 {
            below = below.union(&boxes[i]);
            n_below += counts[i];
            costs[i] = if n_below == 0 { f64::INFINITY } else { n_below as f64 * surface_area(&below) };
        }
        let (mut above, mut n_above) = (BBox::empty(), 0);
        for i in (1 .. BUCKETS).rev() {
            above = above.union(&boxes[i]);
            n_above += counts[i];
            costs[i - 1] = if n_above == 0 { f64::INFINITY } else { costs[i - 1] + n_above as f64 * surface_area(&above) };
        }
        let (split, best) = costs.iter().enumerate()
            .min_by(|a, b| a.1.total_cmp(b.1)).map(|(i, c)| (i, *c)).unwrap();

        let area = surface_area(&bounds);
        let split_cost = TRAVERSAL_COST + if area > 0. { best / area } else { 0. };
        let leaf_cost = prims.len() as f64;
        if !best.is_finite() || (prims.len() <= MAX_LEAF && leaf_cost <= split_cost) {
            return make_leaf(prims, nodes, order);
        }

        // Partition in place: buckets up to `split` first.
        let mut mid = 0;
        for i in 0 .. prims.len() {
            if bucket(&prims[i].centroid) <= split {
                prims.swap(i, mid);
                mid += 1;
            }
        }

        let this = nodes.len();
        nodes.push(Node { bounds, offset: 0, count: 0, axis: axis as u8 });
        let (first, second) = prims.split_at_mut(mid);
        Tree::build_node(first, depth + 1, nodes, order);
        nodes[this].offset = nodes.len() as u32;
        Tree::build_node(second, depth + 1, nodes, order);
    }

    pub fn bounds(&self) -> BBox {
        self.nodes.first().map_or(BBox::empty(), |n| n.bounds)
    }

    /// Visit the leaves the ray passes through within [min, max), near side first. `leaf` is
    /// given each leaf's range of primitives and returns the new limit on distance (the nearest
    /// hit so far, or `max`), or None to stop.
    fn traverse(&self, r: &Ray, min: f64, max: f64, mut leaf: impl FnMut(std::ops::Range<usize>) -> Option<f64>) {
        if self.nodes.is_empty() {
            return;
        }
        let t = Traversal::new(r);
        let mut limit = max;
        let mut stack = [0u32; MAX_DEPTH + 2];
        let mut top = 0;
        let mut i = 0;
        loop {
            let node = &self.nodes[i];
            if t.enters(r, &node.bounds, min, limit) {
                if node.count > 0 {
                    let start = node.offset as usize;
                    match leaf(start .. start + node.count as usize) {
                        Some(l) => limit = l,
                        None => return,
                    }
                } else {
                    let (near, far) = if t.negative[node.axis as usize] { (node.offset as usize, i + 1) } else { (i + 1, node.offset as usize) };
                    stack[top] = far as u32;
                    top += 1;
                    i = near;
                    continue;
                }
            }
            if top == 0 {
                return;
            }
            top -= 1;
            i = stack[top] as usize;
        }
    }

    /// The nearest primitive hit at least `min` and less than `max` along the ray. `hit` tests
    /// one primitive, as `Geometry::intersects_beyond` does.
    pub fn nearest(&self, r: &Ray, min: f64, max: f64, hit: impl Fn(usize) -> Option<RawIntersection>) -> Option<(usize, RawIntersection)> {
        let mut best: Option<(usize, RawIntersection)> = None;
        let mut limit = max;
        self.traverse(r, min, max, |range| {
            for p in range {
                if let Some(x) = hit(p) {
                    if x.dist >= min && x.dist < limit {
                        limit = x.dist;
                        best = Some((p, x));
                    }
                }
            }
            Some(limit)
        });
        best
    }

    /// Whether any primitive is hit at least `min` and less than `max` along the ray, stopping
    /// at the first found: PBR's `IntersectP`, for shadow rays.
    pub fn any(&self, r: &Ray, min: f64, max: f64, hit: impl Fn(usize) -> Option<RawIntersection>) -> bool {
        let mut found = false;
        self.traverse(r, min, max, |range| {
            found = range.into_iter().any(|p| hit(p).is_some_and(|x| x.dist >= min && x.dist < max));
            if found { None } else { Some(max) }
        });
        found
    }

    #[cfg(test)]
    fn depth(&self) -> usize {
        fn d(nodes: &[Node], i: usize) -> usize {
            if nodes[i].count > 0 { 1 } else { 1 + d(nodes, i + 1).max(d(nodes, nodes[i].offset as usize)) }
        }
        if self.nodes.is_empty() { 0 } else { d(&self.nodes, 0) }
    }
}

/// A hierarchy over a collection of geometries.
#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)] // Abbreviations stay in capitals in this codebase.
pub struct BVH<T: Geometry> {
    tree: Tree,
    items: Vec<Arc<T>>,
}

impl<T: Geometry> BVH<T> {
    pub fn new(items: &[Arc<T>]) -> BVH<T> {
        let bounds: Vec<BBox> = items.iter().map(|i| i.bounds()).collect();
        let (tree, order) = Tree::build(&bounds);
        BVH { tree, items: order.into_iter().map(|i| items[i].clone()).collect() }
    }

    fn closest(&self, r: &Ray, max: f64, min: f64) -> Option<(usize, RawIntersection)> {
        self.tree.nearest(r, min, max, |i| self.items[i].intersects_beyond(r, min))
    }

    pub fn intersection(&self, r: &Ray, max: f64, min: f64) -> Option<(&T, RawIntersection)> {
        self.closest(r, max, min).map(|(i, hit)| (&*self.items[i], hit))
    }

    pub fn raw_intersection(&self, r: &Ray, max: f64, min: f64) -> Option<RawIntersection> {
        self.closest(r, max, min).map(|(_, hit)| hit)
    }

    /// Whether anything is hit between `min` and `max` along the ray (PBR section 6.1.3,
    /// `IntersectP`): it can stop at the first hit.
    pub fn occluded(&self, r: &Ray, max: f64, min: f64) -> bool {
        self.tree.any(r, min, max, |i| self.items[i].intersects_beyond(r, min))
    }

    pub fn bounds(&self) -> BBox {
        self.tree.bounds()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    fn spheres(seed: u8, n: usize) -> Vec<Arc<Sphere>> {
        let mut rng: StdRng = SeedableRng::from_seed([seed; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        (0..n).map(|_| Arc::new(Sphere::new(
            Vector3::new(u(-10., 10.), u(-10., 10.), u(-10., 10.)), u(0.2, 3.)))).collect()
    }

    /// Nearest hit by testing every item: what the hierarchy must agree with.
    fn brute_force(items: &[Arc<Sphere>], r: &Ray, max: f64, min: f64) -> Option<f64> {
        items.iter()
            .filter_map(|s| s.intersects_beyond(r, min).map(|x| x.dist))
            .filter(|d| *d >= min && *d < max)
            .min_by(|a, b| a.total_cmp(b))
    }

    #[test]
    fn test_traversal_matches_brute_force() {
        // Overlapping spheres of mixed sizes; origins inside and outside the bounds; some rays
        // limited by max, some by min.
        let items = spheres(3, 200);
        let bvh = BVH::new(&items);
        assert!(bvh.tree.nodes.len() > 1);
        let mut rng: StdRng = SeedableRng::from_seed([9; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        let mut hits = 0;
        for i in 0..5000 {
            let r = Ray {
                ro: Vector3::new(u(-15., 15.), u(-15., 15.), u(-15., 15.)),
                rd: Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)) * u(0.5, 2.),
            };
            let (max, min) = match i % 3 { 0 => (f64::INFINITY, 0.), 1 => (u(1., 20.), 0.), _ => (f64::INFINITY, u(0., 5.)) };
            let expected = brute_force(&items, &r, max, min);
            let got = bvh.raw_intersection(&r, max, min).map(|x| x.dist);
            match (expected, got) {
                (None, None) => (),
                (Some(e), Some(g)) => { hits += 1; assert!((e - g).abs() < 1e-9, "ray {}: {} vs {}", r, e, g) },
                _ => panic!("ray {} max {} min {}: brute force {:?}, BVH {:?}", r, max, min, expected, got),
            }
            assert_eq!(bvh.occluded(&r, max, min), expected.is_some(), "ray {} max {} min {}", r, max, min);
        }
        assert!(hits > 1000 && hits < 4900, "{}", hits);
    }

    #[test]
    fn test_every_item_is_stored_once() {
        let items = spheres(5, 500);
        let (tree, order) = Tree::build(&items.iter().map(|s| s.bounds()).collect::<Vec<_>>());
        let mut seen = order.clone();
        seen.sort();
        assert_eq!(seen, (0..500).collect::<Vec<_>>());
        let in_leaves: u32 = tree.nodes.iter().map(|n| n.count).sum();
        assert_eq!(in_leaves, 500);
        // Leaves stay small, and children's boxes lie within their parents'.
        assert!(tree.nodes.iter().all(|n| n.count as usize <= MAX_LEAF));
        for (i, n) in tree.nodes.iter().enumerate() {
            if n.count == 0 {
                for c in [i + 1, n.offset as usize] {
                    assert!(n.bounds.contains(&tree.nodes[c].bounds));
                }
            }
        }
    }

    #[test]
    fn test_coincident_items_make_a_leaf() {
        // Items whose centroids can't be separated (the octree once subdivided these forever).
        let items: Vec<Arc<BBox>> = (0..5).map(|_| Arc::new(BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.)))).collect();
        let bvh = BVH::new(&items);
        assert_eq!(bvh.tree.nodes.len(), 1);
        assert!(bvh.raw_intersection(&Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) }, f64::INFINITY, 0.).is_some());
    }

    #[test]
    fn test_depth_is_bounded() {
        // Spheres along a line with geometrically shrinking spacing: a degenerate input for
        // a centroid split, which still must not exceed the traversal stack.
        let items: Vec<Arc<Sphere>> = (0..2000).map(|i| Arc::new(Sphere::new(Vector3::new(0.5f64.powi(i % 64), 0., 0.), 1e-3))).collect();
        let bvh = BVH::new(&items);
        assert!(bvh.tree.depth() <= MAX_DEPTH + 1, "{}", bvh.tree.depth());
        let r = Ray { ro: Vector3::new(-5., 0., 0.), rd: Vector3::new(1., 0., 0.) };
        assert!(bvh.raw_intersection(&r, f64::INFINITY, 0.).is_some());
    }

    #[test]
    fn test_empty() {
        let bvh: BVH<Sphere> = BVH::new(&[]);
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 1.) };
        assert!(bvh.raw_intersection(&r, f64::INFINITY, 0.).is_none());
        assert!(!bvh.occluded(&r, f64::INFINITY, 0.));
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
        let bvh = BVH::new(&items);
        let r = Ray { ro: Vector3::new(0.1, 0.1, -1e-12), rd: Vector3::new(0., 0., 1.) };
        let hit = bvh.raw_intersection(&r, 10., 0.001).expect("second triangle should occlude");
        assert!((hit.dist - 1.).abs() < 1e-6, "{}", hit.dist);
    }
}
