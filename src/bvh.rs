//! # Bounding volume hierarchy
//!
//! A tree of nested bounding boxes over a set of items, for finding the nearest item along a ray
//! without testing them all. Bounding volumes were introduced to ray tracing by Steven M. Rubin
//! and Turner Whitted, "A 3-Dimensional Representation for Fast Rendering of Complex Scenes",
//! SIGGRAPH 1980, doi:10.1145/965105.807479, and the hierarchy of axis-aligned boxes by Timothy
//! L. Kay and James T. Kajiya, "Ray Tracing Complex Scenes", SIGGRAPH 1986,
//! doi:10.1145/15886.15916. Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed.,
//! section 7.3 (<https://pbr-book.org/4ed/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies>)
//! describe the construction used here.
//!
//! ## Why not the octree
//!
//! `Octree` subdivides *space*, so an item is stored in every cell its box touches. For a mesh
//! of long, thin or large triangles that is many cells each: the 6,320-triangle Utah teapot at
//! depth 8 makes 745,766 nodes holding 2,033,792 item references, about 120 MB, for 1 MB of
//! triangles. A BVH subdivides the *items*: each is in exactly one leaf, and there are at most
//! 2n - 1 nodes, so it needs memory linear in the item count. The price is that sibling boxes
//! may overlap, so a ray can't stop at the first leaf with a hit; it can still skip any node
//! whose box begins beyond the best hit so far.
//!
//! ## Construction: the surface area heuristic
//!
//! Each node's items are split in two along one axis. Where to split comes from the surface area
//! heuristic of J. David MacDonald and Kellogg S. Booth, "Heuristics for Ray Tracing Using Space
//! Subdivision", The Visual Computer 6(3), 153-166, 1990, doi:10.1007/BF01911006, after Jeffrey
//! Goldsmith and John Salmon, "Automatic Creation of Object Hierarchies for Ray Tracing", IEEE
//! CG&A 7(5), 1987, doi:10.1109/MCG.1987.276983. For a random ray that hits a box, the chance it
//! also hits a box inside it is the ratio of their surface areas, so the expected cost of a split
//! into A and B is
//!
//! ```text
//!     C = C_trav + (S_A / S) N_A C_isect + (S_B / S) N_B C_isect
//! ```
//!
//! with S the surface areas and N the item counts. Making a leaf instead costs N C_isect. Here
//! C_isect = 1 and C_trav = `TRAVERSAL_COST`.
//!
//! Evaluating C at every possible split is O(n^2) per level; instead items are put into
//! `BINS` buckets by their centroids along the axis where centroids spread furthest, and only
//! the boundaries between buckets are tried (Ingo Wald, "On Fast Construction of SAH-based
//! Bounding Volume Hierarchies", IEEE Symposium on Interactive Ray Tracing 2007,
//! doi:10.1109/RT.2007.4342588). The tree is then a little worse but the build is O(n log n).
//!
//! ## Layout
//!
//! Nodes are a flat `Vec` in depth-first order: an interior node's first child is the next node,
//! and it records only where its second child is. A leaf records a run of items, which are
//! reordered at build time so every leaf's items are contiguous (PBR section 7.3.4, "Compact BVH
//! for Traversal"). Items are addressed by index, so the same tree serves triangles, scene
//! objects, or triangles that are only indices into a shared vertex buffer.
//!
//! ## Traversal
//!
//! Depth first with an explicit stack, visiting the nearer child first and skipping any node
//! that begins beyond the best hit so far. Shadow rays use `any_hit`, which stops at the first
//! hit in range.

use crate::intersection::RawIntersection;
use crate::na::Vector3;
use crate::na::geometry::{Affine3, Transform3};
use crate::ray::Ray;
use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use std::mem::MaybeUninit;
use std::ops::Range;

/// Buckets tried along the split axis.
const BINS: usize = 16;
/// The cost of visiting a node, relative to intersecting an item.
const TRAVERSAL_COST: f64 = 0.125;
/// Leaves never hold more than this many items (unless they can't be split at all, eg. all
/// centroids coincide), whatever the heuristic says.
const MAX_LEAF: usize = 8;
/// How deep `transformed_bounds` looks: at most 2^8 = 256 boxes. For a rotated, finely
/// tessellated sphere that is within 20% of the true extent; transforming the root box's corners
/// gives 70% too wide.
const TRANSFORMED_BOUNDS_DEPTH: usize = 8;
/// Beyond this depth nodes are halved by count instead of by the heuristic, which can split off
/// one item at a time (eg. from items spaced exponentially): the tree is then at most this plus
/// 32 deep, for up to 2^32 items, which bounds the traversal stack.
const MAX_HEURISTIC_DEPTH: usize = 64;
/// Traversal pushes at most one node per level (and the root): deep enough for any tree `build`
/// makes.
const STACK_SIZE: usize = MAX_HEURISTIC_DEPTH + 34;

#[derive(Clone, Copy, Debug)]
struct Node {
    bounds: BBox,
    /// Leaf: the index of its first item. Interior: the index of its second child.
    offset: u32,
    /// Leaf: how many items. Interior: 0.
    count: u32,
}

pub struct Bvh {
    nodes: Vec<Node>,
}

/// A ray prepared for repeated box tests.
struct TraversalRay {
    ro: Vector3<f64>,
    invrd: Vector3<f64>,
    /// Box tests give distances in multiples of the ray direction; this converts them to
    /// world-space distances, which items report.
    len: f64,
}

impl TraversalRay {
    fn new(r: &Ray) -> TraversalRay {
        TraversalRay {
            ro: r.ro,
            invrd: Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z),
            len: r.rd.norm(),
        }
    }

    /// Where the ray enters the box, if it passes through some part of it in `[min, max]`.
    ///
    /// `BBox::entry_exit`, with plain comparisons in place of `f64::min` and `max`: NaN (a ray
    /// parallel to a slab, starting on its face) is handled first, so their care with NaN was
    /// wasted, and the box tests are most of traversal.
    #[inline]
    fn enters(&self, b: &BBox, min: f64, max: f64) -> Option<f64> {
        let (mut entry, mut exit) = (f64::NEG_INFINITY, f64::INFINITY);
        for a in 0..3 {
            let t1 = (b.min[a] - self.ro[a]) * self.invrd[a];
            let t2 = (b.max[a] - self.ro[a]) * self.invrd[a];
            // Parallel to the slab and starting on one of its faces (0 * inf): inside the
            // closed slab, so it limits nothing.
            if t1.is_nan() || t2.is_nan() { continue }
            let (near, far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
            if near > entry { entry = near }
            if far < exit { exit = far }
        }
        if exit < 0. || entry > exit { return None }
        let entry = if entry > 0. { entry } else { 0. } * self.len;
        if entry <= max && exit * self.len >= min { Some(entry) } else { None }
    }
}

fn surface_area(b: &BBox) -> f64 {
    let d = b.size();
    if d.x < 0. { 0. } else { 2. * (d.x * d.y + d.y * d.z + d.z * d.x) }
}

impl Bvh {
    /// Build a tree over `items`, returning it with the items reordered into leaf order: the
    /// indices the tree passes to callers index the returned `Vec`.
    pub fn build<T>(items: Vec<T>, bounds_of: impl Fn(&T) -> BBox) -> (Bvh, Vec<T>) {
        let bounds: Vec<BBox> = items.iter().map(&bounds_of).collect();
        let centroids: Vec<Vector3<f64>> = bounds.iter().map(|b| b.mid()).collect();
        let mut order: Vec<usize> = (0 .. items.len()).collect();
        let mut nodes = Vec::with_capacity(2 * items.len());
        if !items.is_empty() {
            Bvh::build_node(&mut nodes, &bounds, &centroids, &mut order, 0, 0);
        }

        // Permute the items into leaf order.
        let mut slots: Vec<Option<T>> = items.into_iter().map(Some).collect();
        let items = order.iter().map(|&i| slots[i].take().unwrap()).collect();
        nodes.shrink_to_fit();
        (Bvh { nodes }, items)
    }

    /// Build the node for `order[..]` (which starts at `first` in the full order), and its
    /// subtree, appending them to `nodes`.
    fn build_node(nodes: &mut Vec<Node>, bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize], first: usize, depth: usize) {
        let node_bounds = order.iter().fold(BBox::empty(), |b, &i| b.union(&bounds[i]));
        let index = nodes.len();
        nodes.push(Node { bounds: node_bounds, offset: first as u32, count: order.len() as u32 });
        if order.len() <= 1 {
            return;
        }

        let centroid_bounds = order.iter().fold(BBox::empty(), |b, &i| b.union_point(&centroids[i]));
        let extent = centroid_bounds.size();
        let axis = if extent.x >= extent.y && extent.x >= extent.z { 0 } else if extent.y >= extent.z { 1 } else { 2 };
        let (lo, width) = (centroid_bounds.min[axis], extent[axis]);
        if width.is_nan() || width <= 0. {
            // Every centroid is in the same place: no split separates anything.
            return;
        }
        if depth >= MAX_HEURISTIC_DEPTH {
            if order.len() > MAX_LEAF {
                let mid = order.len() / 2;
                order.select_nth_unstable_by(mid, |&a, &b| centroids[a][axis].total_cmp(&centroids[b][axis]));
                Bvh::split_at(nodes, bounds, centroids, order, first, depth, index, mid);
            }
            return;
        }
        let bin_of = |i: usize| (((centroids[i][axis] - lo) / width * BINS as f64) as usize).min(BINS - 1);

        let mut counts = [0usize; BINS];
        let mut bin_bounds = [BBox::empty(); BINS];
        for &i in order.iter() {
            let b = bin_of(i);
            counts[b] += 1;
            bin_bounds[b] = bin_bounds[b].union(&bounds[i]);
        }

        // Sweep from each side to get the cost of splitting after each bin.
        let mut cost = [0.; BINS - 1];
        let (mut acc, mut n) = (BBox::empty(), 0);
        for s in 0 .. BINS - 1 {
            acc = acc.union(&bin_bounds[s]);
            n += counts[s];
            cost[s] = n as f64 * surface_area(&acc);
        }
        let (mut acc, mut n) = (BBox::empty(), 0);
        for s in (0 .. BINS - 1).rev() {
            acc = acc.union(&bin_bounds[s + 1]);
            n += counts[s + 1];
            cost[s] += n as f64 * surface_area(&acc);
        }
        let (split, best) = cost.iter().copied().enumerate()
            .min_by(|a, b| a.1.total_cmp(&b.1)).unwrap();
        let area = surface_area(&node_bounds);
        let split_cost = TRAVERSAL_COST + if area > 0. { best / area } else { 0. };
        if order.len() <= MAX_LEAF && split_cost >= order.len() as f64 {
            return;
        }

        // Partition: bins up to `split` first. Both sides are non-empty, as the lowest and highest
        // centroids fall in the first and last bins.
        let mut mid = 0;
        for j in 0 .. order.len() {
            if bin_of(order[j]) <= split {
                order.swap(j, mid);
                mid += 1;
            }
        }
        Bvh::split_at(nodes, bounds, centroids, order, first, depth, index, mid);
    }

    /// Make node `index` an interior node, its children holding `order[..mid]` and `order[mid..]`.
    #[allow(clippy::too_many_arguments)]
    fn split_at(nodes: &mut Vec<Node>, bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize],
                first: usize, depth: usize, index: usize, mid: usize) {
        let (left, right) = order.split_at_mut(mid);
        Bvh::build_node(nodes, bounds, centroids, left, first, depth + 1);
        let second = nodes.len();
        Bvh::build_node(nodes, bounds, centroids, right, first + mid, depth + 1);
        nodes[index].offset = second as u32;
        nodes[index].count = 0;
    }

    pub fn bounds(&self) -> BBox {
        self.nodes.first().map_or(BBox::empty(), |n| n.bounds)
    }

    /// Bounds of the tree's contents after transforming them by `t`: the union of its boxes
    /// `TRANSFORMED_BOUNDS_DEPTH` levels down (or leaves above that), each transformed. Much
    /// tighter than transforming the root box when `t` rotates, and the cost is bounded
    /// whatever the size of the tree.
    pub fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        let t: Transform3<f64> = na::convert(*t);
        let mut out = BBox::empty();
        let mut stack = vec![(0usize, 0usize)];
        while let Some((index, depth)) = stack.pop() {
            let Some(node) = self.nodes.get(index) else { break };
            if node.count > 0 || depth == TRANSFORMED_BOUNDS_DEPTH {
                out = out.union(&node.bounds.transform(&t));
            } else {
                stack.push((index + 1, depth + 1));
                stack.push((node.offset as usize, depth + 1));
            }
        }
        out
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The nearest hit at least `min` and less than `max` along the ray, with the index of the
    /// item hit. `hit(i, best)` intersects the ray with item `i`; `best` is the distance of the
    /// nearest hit so far, which an item may use to give up early. Hits outside `[min, best)` are
    /// ignored.
    pub fn nearest(&self, r: &Ray, min: f64, max: f64, mut hit: impl FnMut(usize, f64) -> Option<RawIntersection>) -> Option<(usize, RawIntersection)> {
        self.nearest_in_leaves(r, min, max, |items, best_dist| {
            let mut best = None;
            let mut best_dist = best_dist;
            for i in items {
                if let Some(x) = hit(i, best_dist).filter(|x| x.dist >= min && x.dist < best_dist) {
                    best_dist = x.dist;
                    best = Some((i, x));
                }
            }
            best
        })
    }

    /// As `nearest`, but `leaf(items, best)` searches a whole leaf's items at once (as
    /// `triangle::may_hit` does, several at a time), returning the nearest hit among them in
    /// `[min, best)`.
    pub fn nearest_in_leaves(&self, r: &Ray, min: f64, max: f64,
                             mut leaf: impl FnMut(Range<usize>, f64) -> Option<(usize, RawIntersection)>) -> Option<(usize, RawIntersection)> {
        let ray = TraversalRay::new(r);
        let mut best: Option<(usize, RawIntersection)> = None;
        let mut best_dist = max;
        // Uninitialised: zeroing it on every ray was a measurable cost. Only entries below `top`,
        // all written, are read.
        let mut stack = [MaybeUninit::<(u32, f64)>::uninit(); STACK_SIZE];
        stack[0].write((0, self.nodes.first().and_then(|n| ray.enters(&n.bounds, min, best_dist))?));
        let mut top = 1;

        while top > 0 {
            top -= 1;
            // SAFETY: stack[top] was written when top last rose past it.
            let (index, entry) = unsafe { stack[top].assume_init() };
            if entry > best_dist {
                continue; // a hit was found nearer than this node, since it was pushed
            }
            let node = &self.nodes[index as usize];
            if node.count > 0 {
                if let Some((i, x)) = leaf(node.offset as usize .. (node.offset + node.count) as usize, best_dist) {
                    best_dist = x.dist;
                    best = Some((i, x));
                }
                continue;
            }
            let (a, b) = (index + 1, node.offset);
            let ea = ray.enters(&self.nodes[a as usize].bounds, min, best_dist);
            let eb = ray.enters(&self.nodes[b as usize].bounds, min, best_dist);
            // Push the farther child first, so the nearer is visited next.
            match (ea, eb) {
                (Some(ea), Some(eb)) => {
                    let (near, far) = if ea <= eb { ((a, ea), (b, eb)) } else { ((b, eb), (a, ea)) };
                    stack[top].write(far);
                    stack[top + 1].write(near);
                    top += 2;
                },
                (Some(ea), None) => { stack[top].write((a, ea)); top += 1; },
                (None, Some(eb)) => { stack[top].write((b, eb)); top += 1; },
                (None, None) => {},
            }
        }
        best
    }

    /// Whether any item is hit in `[min, max)` along the ray, stopping at the first found: for
    /// shadow rays. `hit(i)` says whether item `i` is hit in range.
    pub fn any_hit(&self, r: &Ray, min: f64, max: f64, mut hit: impl FnMut(usize) -> bool) -> bool {
        self.any_hit_in_leaves(r, min, max, |mut items| items.any(&mut hit))
    }

    /// As `any_hit`, but `leaf(items)` tests a whole leaf's items at once.
    pub fn any_hit_in_leaves(&self, r: &Ray, min: f64, max: f64, mut leaf: impl FnMut(Range<usize>) -> bool) -> bool {
        let ray = TraversalRay::new(r);
        let mut stack = [MaybeUninit::<u32>::uninit(); STACK_SIZE];
        if self.nodes.first().and_then(|n| ray.enters(&n.bounds, min, max)).is_none() {
            return false;
        }
        stack[0].write(0);
        let mut top = 1;
        while top > 0 {
            top -= 1;
            // SAFETY: stack[top] was written when top last rose past it.
            let index = unsafe { stack[top].assume_init() };
            let node = &self.nodes[index as usize];
            if node.count > 0 {
                if leaf(node.offset as usize .. (node.offset + node.count) as usize) {
                    return true;
                }
                continue;
            }
            for child in [index + 1, node.offset] {
                if ray.enters(&self.nodes[child as usize].bounds, min, max).is_some() {
                    stack[top].write(child);
                    top += 1;
                }
            }
        }
        false
    }
}

/// A BVH over items of one kind of geometry, which it owns: for containers of shapes (CSG
/// unions, box terrain, fireworks) whose items need nothing beyond `Geometry`.
pub struct GeometryBvh<T> {
    bvh: Bvh,
    items: Vec<T>,
}

impl<T: Geometry> GeometryBvh<T> {
    pub fn new(items: Vec<T>) -> GeometryBvh<T> {
        let (bvh, items) = Bvh::build(items, |x| x.bounds());
        GeometryBvh { bvh, items }
    }

    /// The nearest item hit in `[min, max)`, and where.
    pub fn nearest(&self, r: &Ray, min: f64, max: f64) -> Option<(&T, RawIntersection)> {
        self.bvh.nearest(r, min, max, |i, _| self.items[i].intersects_beyond(r, min))
            .map(|(i, x)| (&self.items[i], x))
    }

    pub fn bounds(&self) -> BBox {
        self.bvh.bounds()
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

    fn spheres(n: usize, seed: u64) -> Vec<Sphere> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0 .. n).map(|_| Sphere::new(
            Vector3::new(rng.gen_range(-10., 10.), rng.gen_range(-10., 10.), rng.gen_range(-10., 10.)),
            rng.gen_range(0.05, 1.5),
        )).collect()
    }

    fn random_ray(rng: &mut StdRng) -> Ray {
        Ray {
            ro: Vector3::new(rng.gen_range(-15., 15.), rng.gen_range(-15., 15.), rng.gen_range(-15., 15.)),
            rd: Vector3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.)) * 3.,
        }
    }

    fn brute_force(items: &[Sphere], r: &Ray, min: f64, max: f64) -> Option<f64> {
        items.iter().filter_map(|s| s.intersects_beyond(r, min))
            .map(|x| x.dist).filter(|&d| d >= min && d < max)
            .min_by(|a, b| a.total_cmp(b))
    }

    #[test]
    fn test_matches_brute_force() {
        let original = spheres(500, 1);
        let (bvh, items) = Bvh::build(spheres(500, 1), |s| s.bounds());
        assert_eq!(items.len(), original.len());
        assert!(bvh.node_count() < 2 * items.len());
        let mut rng = StdRng::seed_from_u64(2);
        for _ in 0 .. 2000 {
            let r = random_ray(&mut rng);
            let (min, max) = (rng.gen_range(0., 3.), rng.gen_range(3., 60.));
            let expected = brute_force(&original, &r, min, max);
            let found = bvh.nearest(&r, min, max, |i, _| items[i].intersects_beyond(&r, min)).map(|(_, x)| x.dist);
            assert_eq!(expected, found, "{}", r);
            let any = bvh.any_hit(&r, min, max, |i| items[i].intersects_beyond(&r, min).is_some_and(|x| x.dist >= min && x.dist < max));
            assert_eq!(any, expected.is_some(), "{}", r);
        }
    }

    #[test]
    fn test_index_is_of_the_item_hit() {
        let (bvh, items) = Bvh::build(spheres(100, 3), |s| s.bounds());
        let mut rng = StdRng::seed_from_u64(4);
        for _ in 0 .. 500 {
            let r = random_ray(&mut rng);
            if let Some((i, x)) = bvh.nearest(&r, 0., f64::INFINITY, |i, _| items[i].intersects(&r)) {
                assert_eq!(items[i].intersects(&r).unwrap().dist, x.dist);
            }
        }
    }

    #[test]
    fn test_coincident_items_make_a_leaf() {
        // No split separates identical boxes; the build must stop rather than recurse forever.
        let (bvh, items) = Bvh::build(vec![Sphere::new(Vector3::zeros(), 1.); 100], |s| s.bounds());
        assert_eq!(bvh.node_count(), 1);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert!(bvh.nearest(&r, 0., f64::INFINITY, |i, _| items[i].intersects(&r)).is_some());
    }

    fn depth(bvh: &Bvh, index: usize) -> usize {
        let n = &bvh.nodes[index];
        if n.count > 0 { 1 } else { 1 + depth(bvh, index + 1).max(depth(bvh, n.offset as usize)) }
    }

    #[test]
    fn test_depth_is_bounded() {
        // Spheres spaced exponentially along x: each split by the heuristic separates the
        // farthest from the rest, which would make a tree as deep as there are items, and
        // overflow the traversal stack.
        let original: Vec<Sphere> = (-450 .. 450).map(|i| {
            let x = 2f64.powi(i);
            Sphere::new(Vector3::new(x, 0., 0.), x / 4.)
        }).collect();
        let (bvh, items) = Bvh::build(original.clone(), |s| s.bounds());
        assert!(depth(&bvh, 0) <= MAX_HEURISTIC_DEPTH + 34, "{}", depth(&bvh, 0));
        assert!(depth(&bvh, 0) > MAX_HEURISTIC_DEPTH, "the case should reach the limit: {}", depth(&bvh, 0));
        // Rays along the row and across it, from both ends and from within.
        for (ro, rd) in [(-1., 1.), (1e140, -1.), (3e-50, 1.), (3e-50, -1.)] {
            let r = Ray { ro: Vector3::new(ro, 0., 0.), rd: Vector3::new(rd, 0., 0.) };
            let expected = brute_force(&original, &r, 0., f64::INFINITY);
            let found = bvh.nearest(&r, 0., f64::INFINITY, |i, _| items[i].intersects_beyond(&r, 0.)).map(|(_, x)| x.dist);
            assert_eq!(expected, found, "{}", r);
            assert_eq!(bvh.any_hit(&r, 0., f64::INFINITY, |i| items[i].intersects_beyond(&r, 0.).is_some()), expected.is_some());
        }
    }

    #[test]
    fn test_empty() {
        let (bvh, _) = Bvh::build(Vec::<Sphere>::new(), |s| s.bounds());
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 1.) };
        assert!(bvh.nearest(&r, 0., f64::INFINITY, |_, _| unreachable!()).is_none());
        assert!(!bvh.any_hit(&r, 0., f64::INFINITY, |_| unreachable!()));
    }
}
