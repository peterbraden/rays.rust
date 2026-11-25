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
//! The octree this replaced (`octree.rs`, since removed: its last user, CSG unions, has a BVH
//! too) subdivided *space*, so an item was stored in every cell its box touched. For a mesh
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
//! `BINS` buckets by their centroids, and only the boundaries between buckets are tried (Ingo
//! Wald, "On Fast Construction of SAH-based Bounding Volume Hierarchies", IEEE Symposium on
//! Interactive Ray Tracing 2007, doi:10.1109/RT.2007.4342588). The tree is then a little worse
//! but the build is O(n log n). The buckets are tried along all three axes, in one pass over the
//! items, as pbrt-v4 does, rather than only along the axis where the centroids spread furthest:
//! trees 3-5% cheaper by the heuristic (`sah_cost`; `bvh_measure`'s meshes), for a build about
//! 1.5 times as long.
//!
//! ## Layout
//!
//! Large subtrees are built in parallel: once a node's items are partitioned, its two children
//! are independent (pbrt-v4's `BVHAggregate` does the same). The top of the tree, built before
//! them, bins its items in parallel.
//!
//! The binary tree (`BinaryBvh`) is a flat `Vec` of nodes in depth-first order: an interior
//! node's first child is the next node, and it records only where its second child is. A leaf
//! records a run of items, which are reordered at build time so every leaf's items are contiguous
//! (PBR section 7.3.4, "Compact BVH for Traversal"). Items are addressed by index, so the same
//! tree serves triangles, scene objects, or triangles that are only indices into a shared vertex
//! buffer. The kernels (`crate::kernels`, the GPU's and the wavefront's) take this tree.
//!
//! The renderer's own searches use `Bvh`, the same tree with each node's children and
//! grandchildren gathered into one node of up to four (`Wide`), their boxes stored coordinate
//! by coordinate. The leaves and their boxes are the binary tree's, so it finds the same hits.
//!
//! ## Traversal
//!
//! Depth first with an explicit stack. At each node the ray is tested against all four
//! children's boxes at once (`WideRay::enters`: branch-free, so each line is one vector
//! instruction); the children it enters are pushed farthest first, so the nearest is visited
//! next, and any that begins beyond the best hit so far by the time it comes off is skipped.
//! Shadow rays use `any_hit`, which stops at the first hit in range and tests a node's leaves
//! before going deeper.
//!
//! Measured over meshes of 6,000 to 500,000 triangles (`bvh_measure`), the wide tree's
//! traversal is 1.2 to 1.9 times as fast as the binary tree's, which it replaced: a binary node
//! visit read two nodes of 56 bytes from wherever they were, often across cache lines, and
//! tested their boxes one after the other, with a branch on each slab.

use crate::intersection::RawIntersection;
use crate::na::Vector3;
use crate::na::geometry::{Affine3, Transform3};
use crate::ray::Ray;
use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use std::mem::MaybeUninit;
use std::ops::Range;
use rayon::prelude::*;

/// Buckets tried along the split axis.
const BINS: usize = 16;
/// The cost of visiting a node, relative to intersecting an item: pbrt-v4's value. At 0.125
/// (pbrt-v3's) splitting nearly always looked cheaper than a leaf, so most leaves held one
/// triangle, and a 518k-triangle mesh had about 960k nodes.
const TRAVERSAL_COST: f64 = 0.5;
/// Leaves never hold more than this many items (unless they can't be split at all, eg. all
/// centroids coincide), whatever the heuristic says.
const MAX_LEAF: usize = 8;
/// How deep `transformed_bounds` looks, in levels of the binary tree: at most 2^8 = 256 boxes. For a rotated, finely
/// tessellated sphere that is within 20% of the true extent; transforming the root box's corners
/// gives 70% too wide.
const TRANSFORMED_BOUNDS_DEPTH: usize = 8;
/// Beyond this depth nodes are halved by count instead of by the heuristic, which can split off
/// one item at a time (eg. from items spaced exponentially): the tree is then at most this plus
/// 32 deep, for up to 2^32 items, which bounds the traversal stack.
const MAX_HEURISTIC_DEPTH: usize = 64;
/// The binary tree's traversal pushes at most one node per level (and the root): deep enough
/// for any tree `build` makes.
const STACK_SIZE: usize = MAX_HEURISTIC_DEPTH + 34;
/// Nodes of at most this many items are split by trying every split of them sorted along each
/// axis, rather than by bins: exact, and for a handful of items much cheaper than filling and
/// sweeping `BINS` bins on each axis, which was most of the time spent on small trees (forests
/// build thousands).
const SWEEP: usize = 16;
/// The parallel build splits the tree into subtrees of fewer than this many items (or a 64th
/// of the items, if that is more).
const PARALLEL_BUILD: usize = 4096;

#[derive(Clone, Copy, Debug)]
struct Node {
    bounds: BBox,
    /// Leaf: the index of its first item. Interior: the index of its second child.
    offset: u32,
    /// Leaf: how many items. Interior: 0.
    count: u32,
}

/// The tree as built: binary, its nodes in depth-first order. The kernels (`crate::kernels`)
/// traverse this shape; the renderer's own searches use the wider `Bvh` made from it.
pub struct BinaryBvh {
    nodes: Vec<Node>,
}

/// A ray prepared for repeated box tests: for the binary tree, whose traversal is kept as the
/// reference the wide tree's is tested against, and the box of a tree that is one leaf.
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

impl BinaryBvh {
    /// The nodes as data, for uploading to a GPU (`gpu`): each node's bounds, then for a leaf
    /// the index of its first item and how many, for an interior node the index of its second
    /// child (the first follows it) and 0.
    pub fn export(&self) -> Vec<(BBox, u32, u32)> {
        self.nodes.iter().map(|n| (n.bounds, n.offset, n.count)).collect()
    }

    /// Build a tree over `items`, returning it with the items reordered into leaf order: the
    /// indices the tree passes to callers index the returned `Vec`.
    pub fn build<T: Sync>(items: Vec<T>, bounds_of: impl Fn(&T) -> BBox + Sync) -> (BinaryBvh, Vec<T>) {
        let bounds: Vec<BBox> = items.par_iter().map(&bounds_of).collect();
        let centroids: Vec<Vector3<f64>> = bounds.iter().map(|b| b.mid()).collect();
        let mut order: Vec<usize> = (0 .. items.len()).collect();
        let mut nodes = if items.is_empty() { Vec::new() } else { BinaryBvh::build_parallel(&bounds, &centroids, &mut order) };
        // Kept for the whole render, and grown by pushing: give back the slack.
        nodes.shrink_to_fit();

        // Permute the items into leaf order.
        let mut slots: Vec<Option<T>> = items.into_iter().map(Some).collect();
        let items = order.iter().map(|&i| slots[i].take().unwrap()).collect();
        (BinaryBvh { nodes }, items)
    }

    /// As `build`, with every subtree of at most `leaf` items made one leaf: for items so cheap
    /// to test that visiting a node costs more, such as the cards of `shapes::lod`, it saves most
    /// of the nodes, which are larger than the items (a card is 40 bytes, a node 56).
    pub fn build_with_leaves<T: Sync>(items: Vec<T>, bounds_of: impl Fn(&T) -> BBox + Sync, leaf: usize) -> (BinaryBvh, Vec<T>) {
        let (bvh, items) = BinaryBvh::build(items, bounds_of);
        (bvh.collapsed(leaf), items)
    }

    /// This tree with every subtree of at most `leaf` items made one leaf. A subtree's items are
    /// a contiguous run, the tree being laid out depth first, as are its nodes.
    fn collapsed(self, leaf: usize) -> BinaryBvh {
        if self.nodes.is_empty() {
            return self;
        }
        // Each node's run of items: children follow their parents, so visit from the end.
        let mut runs = vec![(0u32, 0u32); self.nodes.len()];
        for i in (0 .. self.nodes.len()).rev() {
            let n = &self.nodes[i];
            runs[i] = if n.count > 0 { (n.offset, n.count) } else {
                let (a, b) = (runs[i + 1], runs[n.offset as usize]);
                (a.0.min(b.0), a.1 + b.1)
            };
        }
        let mut nodes = Vec::with_capacity(self.nodes.len());
        // (old node, the new node whose right child it is, if it is one)
        let mut stack = vec![(0usize, None::<usize>)];
        while let Some((i, parent)) = stack.pop() {
            let index = nodes.len();
            if let Some(p) = parent {
                let p: &mut Node = &mut nodes[p];
                p.offset = index as u32;
            }
            let n = self.nodes[i];
            let (first, count) = runs[i];
            if n.count > 0 || count as usize <= leaf {
                nodes.push(Node { bounds: n.bounds, offset: first, count });
            } else {
                nodes.push(Node { bounds: n.bounds, offset: 0, count: 0 });
                // The left child goes next, so it's pushed last.
                stack.push((n.offset as usize, Some(index)));
                stack.push((i + 1, None));
            }
        }
        nodes.shrink_to_fit();
        BinaryBvh { nodes }
    }

    /// Build the tree in three phases, so that each node is copied at most once and the tree is
    /// the same as a sequential build's:
    ///
    /// 1. Partition the top of the tree, sequentially, until every part has fewer than
    ///    `task_size` items: these are independent subtrees, each a contiguous run of `order`.
    /// 2. Build the subtrees in parallel, each into a `Vec` of its own.
    /// 3. Lay out the top nodes depth first, splicing each subtree in where it belongs and
    ///    shifting its interior nodes' offsets.
    fn build_parallel(bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize]) -> Vec<Node> {
        enum Top { Split(BBox, Box<Top>, Box<Top>), Subtree(usize) }
        // (first, len, depth) of each subtree's run of `order`, in order.
        fn partition_top(bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize], first: usize, depth: usize, task_size: usize, runs: &mut Vec<(usize, usize, usize)>) -> Top {
            if order.len() >= task_size
                && let (node_bounds, Some(mid)) = BinaryBvh::partition(bounds, centroids, order, depth) {
                let (left, right) = order.split_at_mut(mid);
                let l = partition_top(bounds, centroids, left, first, depth + 1, task_size, runs);
                let r = partition_top(bounds, centroids, right, first + mid, depth + 1, task_size, runs);
                return Top::Split(node_bounds, Box::new(l), Box::new(r));
            }
            runs.push((first, order.len(), depth));
            Top::Subtree(runs.len() - 1)
        }
        fn lay_out(top: Top, subtrees: &mut [Vec<Node>], nodes: &mut Vec<Node>) {
            match top {
                Top::Split(b, l, r) => {
                    let index = nodes.len();
                    nodes.push(Node { bounds: b, offset: 0, count: 0 });
                    lay_out(*l, subtrees, nodes);
                    nodes[index].offset = nodes.len() as u32;
                    lay_out(*r, subtrees, nodes);
                },
                Top::Subtree(i) => {
                    let base = nodes.len() as u32;
                    nodes.extend(std::mem::take(&mut subtrees[i]).into_iter().map(|mut n| {
                        if n.count == 0 { n.offset += base }
                        n
                    }));
                },
            }
        }

        let task_size = (order.len() / 64).max(PARALLEL_BUILD);
        let mut runs = Vec::new();
        let top = partition_top(bounds, centroids, order, 0, 0, task_size, &mut runs);

        // The runs tile `order`, so it splits into one disjoint slice per subtree.
        let mut slices = Vec::with_capacity(runs.len());
        let mut rest = &mut *order;
        for &(_, len, _) in &runs {
            let (slice, tail) = rest.split_at_mut(len);
            slices.push(slice);
            rest = tail;
        }
        let mut subtrees: Vec<Vec<Node>> = slices.into_par_iter().zip(runs.par_iter()).map(|(slice, &(first, _, depth))| {
            let mut sub = Vec::new();
            BinaryBvh::build_node(&mut sub, bounds, centroids, slice, first, depth);
            sub.shrink_to_fit();
            sub
        }).collect();

        let mut nodes = Vec::with_capacity(subtrees.iter().map(|s| s.len()).sum::<usize>() + runs.len());
        lay_out(top, &mut subtrees, &mut nodes);
        nodes
    }

    /// Build the node for `order[..]` (which starts at `first` in the full order), and its
    /// subtree, appending them to `nodes`.
    fn build_node(nodes: &mut Vec<Node>, bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize], first: usize, depth: usize) {
        let (node_bounds, mid) = BinaryBvh::partition(bounds, centroids, order, depth);
        let index = nodes.len();
        nodes.push(Node { bounds: node_bounds, offset: first as u32, count: order.len() as u32 });
        let Some(mid) = mid else { return };
        let (left, right) = order.split_at_mut(mid);
        BinaryBvh::build_node(nodes, bounds, centroids, left, first, depth + 1);
        let second = nodes.len();
        BinaryBvh::build_node(nodes, bounds, centroids, right, first + mid, depth + 1);
        nodes[index].offset = second as u32;
        nodes[index].count = 0;
    }

    /// The bounds of `order`'s items and, if they should be split, where: `order` is
    /// partitioned so the first `mid` items go to one child and the rest to the other. `depth` is
    /// the node's depth in the tree.
    fn partition(bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize], depth: usize) -> (BBox, Option<usize>) {
        let node_bounds = order.iter().fold(BBox::empty(), |b, &i| b.union(&bounds[i]));
        if order.len() <= 1 {
            return (node_bounds, None);
        }

        let centroid_bounds = order.iter().fold(BBox::empty(), |b, &i| b.union_point(&centroids[i]));
        let extent = centroid_bounds.size();
        let axis = if extent.x >= extent.y && extent.x >= extent.z { 0 } else if extent.y >= extent.z { 1 } else { 2 };
        let width = extent[axis];
        if width.is_nan() || width <= 0. {
            // Every centroid is in the same place: no split separates anything.
            return (node_bounds, None);
        }
        if depth >= MAX_HEURISTIC_DEPTH {
            if order.len() > MAX_LEAF {
                let mid = order.len() / 2;
                order.select_nth_unstable_by(mid, |&a, &b| centroids[a][axis].total_cmp(&centroids[b][axis]));
                return (node_bounds, Some(mid));
            }
            return (node_bounds, None);
        }
        if order.len() <= SWEEP {
            return (node_bounds, BinaryBvh::sweep(bounds, centroids, order, &node_bounds));
        }
        // Bin the items along all three axes in one pass, then find the best split on each axis
        // the centroids spread along: (cost, axis, last bin of the first child).
        let scale = Vector3::from_fn(|a, _| if extent[a] > 0. { BINS as f64 / extent[a] } else { 0. });
        let bin_of = |i: usize, a: usize| (((centroids[i][a] - centroid_bounds.min[a]) * scale[a]) as usize).min(BINS - 1);
        type Bins = ([[usize; BINS]; 3], [[BBox; BINS]; 3]);
        let bin = |items: &[usize]| -> Bins {
            let mut counts = [[0usize; BINS]; 3];
            let mut bin_bounds = [[BBox::empty(); BINS]; 3];
            for &i in items {
                for a in 0..3 {
                    let b = bin_of(i, a);
                    counts[a][b] += 1;
                    bin_bounds[a][b] = bin_bounds[a][b].union(&bounds[i]);
                }
            }
            (counts, bin_bounds)
        };
        // The top of a large tree is built before its subtrees are built in parallel: bin its
        // nodes' items in parallel too. (The sums and unions are the same in any order.)
        let (counts, bin_bounds) = if order.len() >= 4 * PARALLEL_BUILD {
            order.par_chunks(PARALLEL_BUILD).map(bin).reduce(|| ([[0; BINS]; 3], [[BBox::empty(); BINS]; 3]), |(mut c, mut b), (c2, b2)| {
                for a in 0..3 {
                    for k in 0..BINS {
                        c[a][k] += c2[a][k];
                        b[a][k] = b[a][k].union(&b2[a][k]);
                    }
                }
                (c, b)
            })
        } else {
            bin(order)
        };
        let mut best: Option<(f64, usize, usize)> = None;
        for a in 0..3 {
            if extent[a].is_nan() || extent[a] <= 0. { continue }
            // Sweep from each side to get the cost of splitting after each bin.
            let mut cost = [0.; BINS - 1];
            let (mut acc, mut n) = (BBox::empty(), 0);
            for s in 0 .. BINS - 1 {
                acc = acc.union(&bin_bounds[a][s]);
                n += counts[a][s];
                cost[s] = n as f64 * surface_area(&acc);
            }
            let (mut acc, mut n) = (BBox::empty(), 0);
            for s in (0 .. BINS - 1).rev() {
                acc = acc.union(&bin_bounds[a][s + 1]);
                n += counts[a][s + 1];
                cost[s] += n as f64 * surface_area(&acc);
            }
            let (split, c) = cost.iter().copied().enumerate().min_by(|x, y| x.1.total_cmp(&y.1)).unwrap();
            if best.is_none_or(|b| c < b.0) {
                best = Some((c, a, split));
            }
        }
        let (best, axis, split) = best.unwrap();
        let area = surface_area(&node_bounds);
        let split_cost = TRAVERSAL_COST + if area > 0. { best / area } else { 0. };
        if order.len() <= MAX_LEAF && split_cost >= order.len() as f64 {
            return (node_bounds, None);
        }

        // Partition: bins up to `split` first. Both sides are non-empty, as the lowest and highest
        // centroids fall in the first and last bins.
        let mut mid = 0;
        for j in 0 .. order.len() {
            if bin_of(order[j], axis) <= split {
                order.swap(j, mid);
                mid += 1;
            }
        }
        (node_bounds, Some(mid))
    }

    /// Where to split a node of at most `SWEEP` items (`order`), if anywhere: every split of the
    /// items sorted along each axis is tried, and `order` left in the best one's order.
    fn sweep(bounds: &[BBox], centroids: &[Vector3<f64>], order: &mut [usize], node_bounds: &BBox) -> Option<usize> {
        let n = order.len();
        // (cost, items in the first child, the items in order)
        let mut best: Option<(f64, usize, [usize; SWEEP])> = None;
        for a in 0..3 {
            let mut sorted = [0usize; SWEEP];
            sorted[..n].copy_from_slice(order);
            // Stable, so ties keep their order and the tree is the same on every build.
            sorted[..n].sort_by(|&x, &y| centroids[x][a].total_cmp(&centroids[y][a]));
            let mut after = [BBox::empty(); SWEEP];
            let mut acc = BBox::empty();
            for k in (1..n).rev() {
                acc = acc.union(&bounds[sorted[k]]);
                after[k] = acc;
            }
            let mut acc = BBox::empty();
            for k in 1..n {
                acc = acc.union(&bounds[sorted[k - 1]]);
                let cost = k as f64 * surface_area(&acc) + (n - k) as f64 * surface_area(&after[k]);
                if best.is_none_or(|b| cost < b.0) {
                    best = Some((cost, k, sorted));
                }
            }
        }
        let (cost, mid, sorted) = best?;
        let area = surface_area(node_bounds);
        let split_cost = TRAVERSAL_COST + if area > 0. { cost / area } else { 0. };
        if n <= MAX_LEAF && split_cost >= n as f64 {
            return None;
        }
        order.copy_from_slice(&sorted[..n]);
        Some(mid)
    }

    pub fn bounds(&self) -> BBox {
        self.nodes.first().map_or(BBox::empty(), |n| n.bounds)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The tree's expected cost for a random ray through its root, by the surface area
    /// heuristic with this module's costs (each node's area relative to the root's, times
    /// `TRAVERSAL_COST` for an interior node or its count for a leaf), and how many leaves it
    /// has: a measure of its quality whatever the rays.
    pub fn sah_cost(&self) -> (f64, usize) {
        let Some(root) = self.nodes.first() else { return (0., 0) };
        let root = surface_area(&root.bounds);
        let mut leaves = 0;
        let cost = self.nodes.iter().map(|n| {
            let share = if root > 0. { surface_area(&n.bounds) / root } else { 1. };
            if n.count > 0 { leaves += 1; share * n.count as f64 } else { share * TRAVERSAL_COST }
        }).sum();
        (cost, leaves)
    }

    #[cfg(test)]
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

    #[cfg(test)]
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

    /// How many interior nodes and leaves `nearest` visits, given the distance of the nearest hit
    /// (`max` for none): for measuring trees.
    #[cfg(test)]
    pub(crate) fn visits(&self, r: &Ray, min: f64, max: f64, hit_at: f64) -> (usize, usize) {
        let (mut interior, mut leaves) = (0, 0);
        let ray = TraversalRay::new(r);
        let mut stack = vec![(0usize, 0.)];
        let mut best = max;
        if self.nodes.is_empty() || ray.enters(&self.nodes[0].bounds, min, best).is_none() { return (0, 0) }
        while let Some((i, entry)) = stack.pop() {
            if entry > best { continue }
            let n = &self.nodes[i];
            if n.count > 0 {
                leaves += 1;
                // The hit is in whichever leaf's box contains it; near enough for counting.
                let p = r.ro + r.rd.normalize() * hit_at;
                if hit_at < best && (0..3).all(|a| p[a] >= n.bounds.min[a] - 1e-9 && p[a] <= n.bounds.max[a] + 1e-9) { best = hit_at }
                continue;
            }
            interior += 1;
            let (a, b) = (i + 1, n.offset as usize);
            let ea = ray.enters(&self.nodes[a].bounds, min, best);
            let eb = ray.enters(&self.nodes[b].bounds, min, best);
            match (ea, eb) {
                (Some(x), Some(y)) => if x <= y { stack.push((b, y)); stack.push((a, x)) } else { stack.push((a, x)); stack.push((b, y)) },
                (Some(x), None) => stack.push((a, x)),
                (None, Some(y)) => stack.push((b, y)),
                _ => {},
            }
        }
        (interior, leaves)
    }

    #[cfg(test)]
    /// Whether any item is hit in `[min, max)` along the ray, stopping at the first found: for
    /// shadow rays. `hit(i)` says whether item `i` is hit in range.
    pub fn any_hit(&self, r: &Ray, min: f64, max: f64, mut hit: impl FnMut(usize) -> bool) -> bool {
        self.any_hit_in_leaves(r, min, max, |mut items| items.any(&mut hit))
    }

    #[cfg(test)]
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

/// Children of a node of `Bvh`.
const WIDTH: usize = 4;
/// `Wide::count` of a child that isn't there.
const NO_CHILD: u32 = u32::MAX;
/// `Bvh`'s traversal stack: each node visited replaces itself with at most `WIDTH` children,
/// and the wide tree is no deeper than the binary one.
const WIDE_STACK_SIZE: usize = (WIDTH - 1) * STACK_SIZE + 1;

/// A node of `Bvh`: up to four children, each a node or a leaf, with their boxes laid out
/// coordinate by coordinate, so that one visit tests all four with the same instructions on
/// each (four f64 lanes: one AVX register, or two SSE2 or NEON ones).
#[derive(Clone, Copy, Debug)]
#[repr(C, align(32))]
struct Wide {
    /// Each child's box: `lo[axis][child]` to `hi[axis][child]`.
    lo: [[f64; WIDTH]; 3],
    hi: [[f64; WIDTH]; 3],
    /// A node's index, or a leaf's first item.
    child: [u32; WIDTH],
    /// A leaf's items; 0 for a node; `NO_CHILD` for no child.
    count: [u32; WIDTH],
}

/// A bounding volume hierarchy for searching: the binary tree built by the surface area
/// heuristic (`BinaryBvh`), with each node's children and grandchildren gathered into one node
/// of up to four children (Ingo Wald, Carsten Benthin and Solomon Boulos, "Getting Rid of
/// Packets: Efficient SIMD Single-Ray Traversal using Multi-branching BVHs", IEEE Symposium on
/// Interactive Ray Tracing 2008, doi:10.1109/RT.2008.4634620; Manfred Ernst and Günther Greiner,
/// "Multi Bounding Volume Hierarchies", the same, doi:10.1109/RT.2008.4634618). A child is opened
/// to make room for its own children largest first, as Embree does, since a ray is likeliest to
/// enter the largest box.
///
/// The boxes are the binary tree's, unchanged, so a search finds the same hits. What changes is
/// the work: a ray visits half as many nodes, testing four boxes together at each, and a node's
/// boxes are in one place, not spread over four nodes of the binary tree (see the module's
/// documentation for measurements).
pub struct Bvh {
    nodes: Vec<Wide>,
    bounds: BBox,
    /// For a tree that is one leaf, how many items it holds; it then has no nodes (one would be
    /// four times the size of the binary tree's, for a tree that is often one of thousands, of
    /// a forest's clusters of cards).
    lone_leaf: u32,
}

/// The ray of `TraversalRay`, coordinate by coordinate.
struct WideRay {
    ro: [f64; 3],
    invrd: [f64; 3],
    len: f64,
    /// Whether some slab test can give NaN: a direction component of zero (or so small that its
    /// reciprocal overflows), for a ray starting on that slab's face, or an infinite one, for an
    /// infinite box. Box coordinates are never NaN, so with finite, non-zero reciprocals a slab
    /// test is never NaN, and the test needs no care for it.
    parallel: bool,
}

impl WideRay {
    fn new(r: &Ray) -> WideRay {
        let invrd = [1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z];
        WideRay { ro: [r.ro.x, r.ro.y, r.ro.z], invrd, len: r.rd.norm(), parallel: !invrd.iter().all(|i| i.is_finite() && *i != 0.) }
    }

    /// `TraversalRay::enters` for each of a node's children: where the ray enters each box, and
    /// a bit for each it passes through some part of in `[min, max]`.
    ///
    /// Written as selects, lane by lane, without branches, so that each line becomes one vector
    /// instruction over the four children. Entries start at 0 rather than minus infinity: an
    /// entry behind the origin counts as 0 anyway, and a box behind the ray is still rejected,
    /// by its exit.
    #[inline(always)]
    fn enters(&self, n: &Wide, min: f64, max: f64) -> ([f64; WIDTH], u32) {
        let mut entry = [0.; WIDTH];
        let mut exit = [f64::INFINITY; WIDTH];
        if self.parallel {
            for a in 0..3 {
                let (o, inv) = (self.ro[a], self.invrd[a]);
                for l in 0..WIDTH {
                    let t1 = (n.lo[a][l] - o) * inv;
                    let t2 = (n.hi[a][l] - o) * inv;
                    // Parallel to the slab and starting on one of its faces (0 * inf): inside
                    // the closed slab, so it limits nothing.
                    if t1.is_nan() || t2.is_nan() { continue }
                    let (near, far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
                    if near > entry[l] { entry[l] = near }
                    if far < exit[l] { exit[l] = far }
                }
            }
        } else {
            for a in 0..3 {
                let (o, inv) = (self.ro[a], self.invrd[a]);
                for l in 0..WIDTH {
                    let t1 = (n.lo[a][l] - o) * inv;
                    let t2 = (n.hi[a][l] - o) * inv;
                    let near = if t1 < t2 { t1 } else { t2 };
                    let far = if t1 < t2 { t2 } else { t1 };
                    entry[l] = if near > entry[l] { near } else { entry[l] };
                    exit[l] = if far < exit[l] { far } else { exit[l] };
                }
            }
        }
        let mut hit = [false; WIDTH];
        for l in 0..WIDTH {
            let inside = entry[l] <= exit[l];
            entry[l] *= self.len;
            hit[l] = inside & (entry[l] <= max) & (exit[l] * self.len >= min) & (n.count[l] != NO_CHILD);
        }
        let hits = hit.iter().enumerate().fold(0, |m, (l, &h)| m | (h as u32) << l);
        (entry, hits)
    }
}

impl Bvh {
    /// Build a tree over `items`, returning it with the items reordered into leaf order: the
    /// indices the tree passes to callers index the returned `Vec`.
    pub fn build<T: Sync>(items: Vec<T>, bounds_of: impl Fn(&T) -> BBox + Sync) -> (Bvh, Vec<T>) {
        let (binary, items) = BinaryBvh::build(items, bounds_of);
        (Bvh::from_binary(&binary), items)
    }

    /// As `build`, with every subtree of at most `leaf` items made one leaf: for items so cheap
    /// to test that visiting a node costs more, such as the cards of `shapes::lod`, it saves most
    /// of the nodes, which are larger than the items.
    pub fn build_with_leaves<T: Sync>(items: Vec<T>, bounds_of: impl Fn(&T) -> BBox + Sync, leaf: usize) -> (Bvh, Vec<T>) {
        let (binary, items) = BinaryBvh::build_with_leaves(items, bounds_of, leaf);
        (Bvh::from_binary(&binary), items)
    }

    #[cfg(test)]
    pub(crate) fn from_binary_for_tests(binary: &BinaryBvh) -> Bvh {
        Bvh::from_binary(binary)
    }

    /// The binary tree's nodes gathered four children to a node.
    fn from_binary(binary: &BinaryBvh) -> Bvh {
        let b = &binary.nodes;
        let Some(root) = b.first() else { return Bvh { nodes: Vec::new(), bounds: BBox::empty(), lone_leaf: 0 } };
        if root.count > 0 {
            return Bvh { nodes: Vec::new(), bounds: root.bounds, lone_leaf: root.count };
        }
        let mut nodes = Vec::with_capacity(b.len() / 3 + 1);
        Bvh::widen(b, vec![1, root.offset as usize], &mut nodes);
        nodes.shrink_to_fit();
        Bvh { nodes, bounds: root.bounds, lone_leaf: 0 }
    }

    /// Append the node whose children are binary nodes `kids` (and their subtrees, depth first),
    /// first opening the largest children that aren't leaves until there are `WIDTH`; returns its
    /// index.
    fn widen(b: &[Node], mut kids: Vec<usize>, nodes: &mut Vec<Wide>) -> u32 {
        while kids.len() < WIDTH {
            let open = kids.iter().enumerate().filter(|(_, k)| b[**k].count == 0)
                .max_by(|x, y| surface_area(&b[*x.1].bounds).total_cmp(&surface_area(&b[*y.1].bounds)));
            let Some((at, &k)) = open else { break };
            kids[at] = k + 1;
            kids.insert(at + 1, b[k].offset as usize);
        }
        let index = nodes.len();
        let mut node = Wide {
            lo: [[f64::INFINITY; WIDTH]; 3], hi: [[f64::NEG_INFINITY; WIDTH]; 3],
            child: [0; WIDTH], count: [NO_CHILD; WIDTH],
        };
        nodes.push(node);
        for (l, &k) in kids.iter().enumerate() {
            let n = &b[k];
            for a in 0..3 {
                node.lo[a][l] = n.bounds.min[a];
                node.hi[a][l] = n.bounds.max[a];
            }
            if n.count > 0 {
                node.child[l] = n.offset;
                node.count[l] = n.count;
            } else {
                node.child[l] = Bvh::widen(b, vec![k + 1, n.offset as usize], nodes);
                node.count[l] = 0;
            }
        }
        nodes[index] = node;
        index as u32
    }

    pub fn bounds(&self) -> BBox {
        self.bounds
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// For a tree that is one leaf, its items, if the ray passes through its box in
    /// `[min, max]`.
    fn lone_leaf(&self, r: &Ray, min: f64, max: f64) -> Option<Range<usize>> {
        (self.lone_leaf > 0 && TraversalRay::new(r).enters(&self.bounds, min, max).is_some()).then_some(0 .. self.lone_leaf as usize)
    }

    /// Child `l` of node `n`'s box.
    fn child_bounds(n: &Wide, l: usize) -> BBox {
        BBox::new(Vector3::new(n.lo[0][l], n.lo[1][l], n.lo[2][l]), Vector3::new(n.hi[0][l], n.hi[1][l], n.hi[2][l]))
    }

    /// Bounds of the tree's contents after transforming them by `t`: the union of its boxes
    /// `TRANSFORMED_BOUNDS_DEPTH` binary levels down (or leaves above that), each transformed.
    /// Much tighter than transforming the root box when `t` rotates, and the cost is bounded
    /// whatever the size of the tree.
    pub fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        let t: Transform3<f64> = na::convert(*t);
        let mut out = BBox::empty();
        if self.nodes.is_empty() {
            return if self.lone_leaf > 0 { self.bounds.transform(&t) } else { out };
        }
        // A wide level is about two binary ones.
        let mut stack = vec![(0usize, 1usize)];
        while let Some((index, depth)) = stack.pop() {
            let node = &self.nodes[index];
            for l in 0..WIDTH {
                match node.count[l] {
                    NO_CHILD => {},
                    0 if 2 * depth < TRANSFORMED_BOUNDS_DEPTH => stack.push((node.child[l] as usize, depth + 1)),
                    _ => out = out.union(&Bvh::child_bounds(node, l).transform(&t)),
                }
            }
        }
        out
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

    /// As `nearest`, but `leaf(items, best)` searches a whole leaf's items at once, returning the
    /// nearest hit among them in `[min, best)`.
    pub fn nearest_in_leaves(&self, r: &Ray, min: f64, max: f64,
                             mut leaf: impl FnMut(Range<usize>, f64) -> Option<(usize, RawIntersection)>) -> Option<(usize, RawIntersection)> {
        if self.nodes.is_empty() {
            return self.lone_leaf(r, min, max).and_then(|items| leaf(items, max));
        }
        let ray = WideRay::new(r);
        let mut best: Option<(usize, RawIntersection)> = None;
        let mut best_dist = max;
        // (a child's node or first item, its count, where the ray enters it). Uninitialised:
        // only entries below `top`, all written, are read.
        let mut stack = [MaybeUninit::<(u32, u32, f64)>::uninit(); WIDE_STACK_SIZE];
        let mut top = 0;
        // The node to visit next without going through the stack.
        let mut next = Some(0u32);
        loop {
            let index = match next.take() {
                Some(i) => i,
                None => {
                    if top == 0 { break }
                    top -= 1;
                    // SAFETY: stack[top] was written when top last rose past it.
                    let (child, count, entry) = unsafe { stack[top].assume_init() };
                    if entry > best_dist {
                        continue; // a hit was found nearer than this child, since it was pushed
                    }
                    if count > 0 {
                        if let Some((i, x)) = leaf(child as usize .. (child + count) as usize, best_dist) {
                            best_dist = x.dist;
                            best = Some((i, x));
                        }
                        continue;
                    }
                    child
                },
            };
            let node = &self.nodes[index as usize];
            let (entry, mut hits) = ray.enters(node, min, best_dist);
            if hits == 0 {
                continue;
            }
            if hits & (hits - 1) == 0 {
                // One child: visit it now.
                let l = hits.trailing_zeros() as usize;
                if node.count[l] == 0 {
                    next = Some(node.child[l]);
                } else if let Some((i, x)) = leaf(node.child[l] as usize .. (node.child[l] + node.count[l]) as usize, best_dist) {
                    best_dist = x.dist;
                    best = Some((i, x));
                }
                continue;
            }
            // Several: push them farthest first, so the nearest comes off next. Insertion
            // sort of at most four, nearest last.
            let mut order = [0usize; WIDTH];
            let mut n = 0;
            while hits != 0 {
                let l = hits.trailing_zeros() as usize;
                hits &= hits - 1;
                let mut j = n;
                while j > 0 && entry[order[j - 1]] < entry[l] {
                    order[j] = order[j - 1];
                    j -= 1;
                }
                order[j] = l;
                n += 1;
            }
            for &l in &order[..n] {
                stack[top].write((node.child[l], node.count[l], entry[l]));
                top += 1;
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
        if self.nodes.is_empty() {
            return self.lone_leaf(r, min, max).is_some_and(leaf);
        }
        let ray = WideRay::new(r);
        let mut stack = [MaybeUninit::<u32>::uninit(); WIDE_STACK_SIZE];
        stack[0].write(0);
        let mut top = 1;
        while top > 0 {
            top -= 1;
            // SAFETY: stack[top] was written when top last rose past it.
            let node = &self.nodes[unsafe { stack[top].assume_init() } as usize];
            let (_, mut hits) = ray.enters(node, min, max);
            // Leaves first: one may end the search.
            let mut leaves = 0;
            for l in 0..WIDTH {
                leaves |= ((node.count[l] != 0 && node.count[l] != NO_CHILD) as u32) << l;
            }
            let mut found = hits & leaves;
            while found != 0 {
                let l = found.trailing_zeros() as usize;
                found &= found - 1;
                if leaf(node.child[l] as usize .. (node.child[l] + node.count[l]) as usize) {
                    return true;
                }
            }
            hits &= !leaves;
            while hits != 0 {
                let l = hits.trailing_zeros() as usize;
                hits &= hits - 1;
                stack[top].write(node.child[l]);
                top += 1;
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
    use rand::{RngExt, SeedableRng};
    use rand::rngs::StdRng;

    fn spheres(n: usize, seed: u64) -> Vec<Sphere> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0 .. n).map(|_| Sphere::new(
            Vector3::new(rng.random_range(-10.0..10.), rng.random_range(-10.0..10.), rng.random_range(-10.0..10.)),
            rng.random_range(0.05..1.5),
        )).collect()
    }

    fn random_ray(rng: &mut StdRng) -> Ray {
        Ray {
            ro: Vector3::new(rng.random_range(-15.0..15.), rng.random_range(-15.0..15.), rng.random_range(-15.0..15.)),
            rd: Vector3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.)) * 3.,
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
            let (min, max) = (rng.random_range(0.0..3.), rng.random_range(3.0..60.));
            let expected = brute_force(&original, &r, min, max);
            let found = bvh.nearest(&r, min, max, |i, _| items[i].intersects_beyond(&r, min)).map(|(_, x)| x.dist);
            assert_eq!(expected, found, "{}", r);
            let any = bvh.any_hit(&r, min, max, |i| items[i].intersects_beyond(&r, min).is_some_and(|x| x.dist >= min && x.dist < max));
            assert_eq!(any, expected.is_some(), "{}", r);
        }
    }

    #[test]
    fn test_small_trees_match_brute_force() {
        // Trees of one leaf (no nodes), of one node with a few children, and a little deeper: built
        // by the exact sweep of small nodes, and searched by the wide traversal.
        for n in [1, 2, 3, 5, 8, 9, 16, 17, 40] {
            let original = spheres(n, n as u64);
            let (bvh, items) = Bvh::build(original.clone(), |s| s.bounds());
            let mut rng = StdRng::seed_from_u64(9);
            for _ in 0 .. 500 {
                let r = random_ray(&mut rng);
                let (min, max) = (rng.random_range(0.0..3.), rng.random_range(3.0..60.));
                let expected = brute_force(&original, &r, min, max);
                let found = bvh.nearest(&r, min, max, |i, _| items[i].intersects_beyond(&r, min)).map(|(_, x)| x.dist);
                assert_eq!(expected, found, "{} items: {}", n, r);
                let any = bvh.any_hit(&r, min, max, |i| items[i].intersects_beyond(&r, min).is_some_and(|x| x.dist >= min && x.dist < max));
                assert_eq!(any, expected.is_some(), "{} items: {}", n, r);
            }
        }
    }

    #[test]
    fn test_wide_finds_what_binary_finds() {
        let (binary, items) = BinaryBvh::build(spheres(3000, 12), |s| s.bounds());
        let wide = Bvh::from_binary(&binary);
        assert!(wide.node_count() < binary.node_count() / 2, "{} of {}", wide.node_count(), binary.node_count());
        let mut rng = StdRng::seed_from_u64(13);
        for _ in 0 .. 3000 {
            let r = random_ray(&mut rng);
            let (min, max) = (rng.random_range(0.0..3.), rng.random_range(3.0..60.));
            let hit = |i: usize, _| items[i].intersects_beyond(&r, min);
            let a = binary.nearest(&r, min, max, hit).map(|(i, x)| (i, x.dist));
            let b = wide.nearest(&r, min, max, hit).map(|(i, x)| (i, x.dist));
            assert_eq!(a, b, "{}", r);
        }
    }

    #[test]
    fn test_collapsed_leaves_find_the_same_hits() {
        let original = spheres(2000, 7);
        let (full, items) = Bvh::build(spheres(2000, 7), |s| s.bounds());
        for leaf in [2, 4, 16] {
            let (bvh, collapsed) = Bvh::build_with_leaves(spheres(2000, 7), |s| s.bounds(), leaf);
            // The same items in the same order, behind fewer nodes.
            assert!(collapsed.iter().zip(&items).all(|(a, b)| a.bounds().min == b.bounds().min && a.bounds().max == b.bounds().max));
            assert!(bvh.node_count() < full.node_count(), "{} of {}", bvh.node_count(), full.node_count());
            let mut rng = StdRng::seed_from_u64(8);
            for _ in 0 .. 1000 {
                let r = random_ray(&mut rng);
                let (min, max) = (rng.random_range(0.0..3.), rng.random_range(3.0..60.));
                let expected = brute_force(&original, &r, min, max);
                let found = bvh.nearest(&r, min, max, |i, _| collapsed[i].intersects_beyond(&r, min)).map(|(_, x)| x.dist);
                assert_eq!(expected, found, "{}", r);
                let any = bvh.any_hit(&r, min, max, |i| collapsed[i].intersects_beyond(&r, min).is_some_and(|x| x.dist >= min && x.dist < max));
                assert_eq!(any, expected.is_some(), "{}", r);
            }
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
        let (binary, _) = BinaryBvh::build(vec![Sphere::new(Vector3::zeros(), 1.); 100], |s| s.bounds());
        assert_eq!(binary.node_count(), 1);
        let (bvh, items) = Bvh::build(vec![Sphere::new(Vector3::zeros(), 1.); 100], |s| s.bounds());
        assert_eq!(bvh.node_count(), 0, "a lone leaf needs no node");
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert!(bvh.nearest(&r, 0., f64::INFINITY, |i, _| items[i].intersects(&r)).is_some());
    }

    fn depth(bvh: &BinaryBvh, index: usize) -> usize {
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
        let (binary, items) = BinaryBvh::build(original.clone(), |s| s.bounds());
        assert!(depth(&binary, 0) <= MAX_HEURISTIC_DEPTH + 34, "{}", depth(&binary, 0));
        assert!(depth(&binary, 0) > MAX_HEURISTIC_DEPTH, "the case should reach the limit: {}", depth(&binary, 0));
        let bvh = Bvh::from_binary(&binary);
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
    fn test_axis_aligned_boxes_and_rays_match_brute_force() {
        // Flat and solid boxes on a grid of whole numbers, and rays with zero (and -0) direction
        // components starting on the grid, running along faces: where box tests meet infinite
        // and NaN slab distances, and entries equal to exits.
        let mut rng = StdRng::seed_from_u64(5);
        let mut int = |a: i32, b: i32| rng.random_range(a..b + 1) as f64;
        let original: Vec<BBox> = (0..120).map(|_| {
            let (x, y, z) = (int(-16, 14), int(-16, 14), int(-16, 14));
            BBox::new(Vector3::new(x, y, z), Vector3::new(x + int(0, 2), y + int(0, 2), z + int(0, 2)))
        }).collect();
        let (bvh, items) = Bvh::build(original.clone(), |b| *b);
        let mut hits = 0;
        for _ in 0..20000 {
            let ro = Vector3::new(int(-20, 20), int(-20, 20), int(-20, 20));
            let mut component = || [-1., -0., 0., 1.][int(0, 3) as usize];
            let mut rd = Vector3::new(component(), component(), component());
            if rd.iter().all(|c| *c == 0.) { rd.x = 1. }
            let r = Ray { ro, rd };
            let expected = original.iter().filter_map(|b| b.intersects_beyond(&r, 0.))
                .map(|x| x.dist).min_by(f64::total_cmp);
            let found = bvh.nearest(&r, 0., f64::INFINITY, |i, _| items[i].intersects_beyond(&r, 0.)).map(|(_, x)| x.dist);
            assert_eq!(expected, found, "{}", r);
            assert_eq!(bvh.any_hit(&r, 0., f64::INFINITY, |i| items[i].intersects_beyond(&r, 0.).is_some()), expected.is_some(), "{}", r);
            if expected.is_some() { hits += 1 }
        }
        assert!(hits > 2000, "{}", hits);
    }

    #[test]
    fn test_empty() {
        let (bvh, _) = Bvh::build(Vec::<Sphere>::new(), |s| s.bounds());
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 1.) };
        assert!(bvh.nearest(&r, 0., f64::INFINITY, |_, _| unreachable!()).is_none());
        assert!(!bvh.any_hit(&r, 0., f64::INFINITY, |_| unreachable!()));
    }

    #[test]
    fn test_parallel_build_matches_sequential() {
        let items = spheres(20_000, 6);
        assert!(items.len() >= 4 * PARALLEL_BUILD);
        let bounds: Vec<BBox> = items.iter().map(|s| s.bounds()).collect();
        let centroids: Vec<Vector3<f64>> = bounds.iter().map(|b| b.mid()).collect();
        let mut order_p: Vec<usize> = (0 .. items.len()).collect();
        let mut order_s = order_p.clone();
        let parallel = BinaryBvh::build_parallel(&bounds, &centroids, &mut order_p);
        let mut sequential = Vec::new();
        BinaryBvh::build_node(&mut sequential, &bounds, &centroids, &mut order_s, 0, 0);
        assert_eq!(order_p, order_s);
        assert_eq!(format!("{:?}", parallel), format!("{:?}", sequential));
        let many = Bvh::build(items.clone(), |s| s.bounds());
        // And the joined tree is right: its offsets were shifted when the halves were joined.
        let (bvh, built) = many;
        let mut rng = StdRng::seed_from_u64(8);
        for _ in 0 .. 500 {
            let r = random_ray(&mut rng);
            let found = bvh.nearest(&r, 0., f64::INFINITY, |i, _| built[i].intersects(&r)).map(|(_, x)| x.dist);
            assert_eq!(found, brute_force(&items, &r, 0., f64::INFINITY), "{}", r);
        }
    }
}
