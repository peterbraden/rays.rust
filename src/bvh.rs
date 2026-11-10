//! # Bounding volume hierarchy
//!
//! A binary tree of bounding boxes over the scene's items, for finding the nearest item along a
//! ray without testing every one. Each node's box bounds the items beneath it; a ray that misses
//! the box misses them all. Unlike the octree it replaced, a node divides its items rather than
//! its space: every item is in exactly one leaf, and boxes are fitted to their items rather than
//! fixed halves of their parent. Following Matt Pharr, Wenzel Jakob and Greg Humphreys,
//! "Physically Based Rendering", 4th ed., section 7.3 (<https://pbr-book.org/4ed/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies>).
//!
//! The octree split every node at its centre on all three axes, copying items that straddled
//! the split into each side. On a surface flatter than its node (the ocean) or along a shared
//! edge, most items straddle, so each level multiplied the items: the ocean's 131072 triangles
//! were held 4 million times. Here none are copied.
//!
//! ## Construction
//!
//! Where to divide a node's items is chosen by the surface area heuristic (Goldsmith and Salmon,
//! "Automatic Creation of Object Hierarchies for Ray Tracing", IEEE CG&A 7(5), 1987; MacDonald
//! and Booth, "Heuristics for Ray Tracing Using Space Subdivision", The Visual Computer 6(3),
//! 1990): the chance a ray that hits a box also hits a box inside it is the ratio of their surface
//! areas, so the expected cost of a division is
//!
//! C = C_trav + (A_L N_L + A_R N_R) / A,
//!
//! for children of areas A_L, A_R holding N_L, N_R items, in units of an item's test. The node
//! is divided where C is least, if that is below N, the cost of testing every item as a leaf.
//! Candidate divisions are planes between `BINS` equal bins of item centroids along each axis
//! (PBR 4th ed. section 7.3.2; Ingo Wald, "On fast Construction of SAH-based Bounding Volume
//! Hierarchies", IEEE Symposium on Interactive Ray Tracing, 2007).
//!
//! ## Traversal
//!
//! Depth first, nearer child first (PBR 4th ed. section 7.3.5): the child on the side of the
//! dividing axis the ray comes from is visited first, the other kept on a stack. A node whose box
//! the ray enters beyond the nearest hit so far is skipped, so once a near hit is found most of
//! the tree is.
//!
//! ## Triangle leaves
//!
//! When every item is a triangle (a mesh, or the ocean), the tree keeps the triangles in leaf
//! order in one array, and a leaf tests them four at a time with vector instructions
//! (`triangle::may_hit`) before testing exactly the few that pass.

use crate::shapes::bbox::BBox;
use crate::shapes::triangle::{EdgeTriangle, LANES, may_hit};
use crate::na::Vector3;
use std::sync::Arc;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::geometry::Geometry;

/// Centroid bins per axis when choosing where to divide a node.
const BINS: usize = 16;
/// The cost of visiting a node, relative to testing an item. With 2 or 4 the bench scenes
/// rendered within a few percent of their times with 1, except the cornell box, 10% slower.
/// Costing triangle leaves by the groups of `LANES` that `may_hit` tests at once was no better.
const TRAVERSAL_COST: f64 = 1.;
/// Nodes with more items are divided even where the heuristic would make a leaf of them.
const MAX_LEAF: usize = 16;
/// Nodes with more items are built in parallel.
const PARALLEL_BUILD: usize = 4096;
/// Beyond this depth, nodes are halved by count rather than by the heuristic, which can divide
/// one item from the rest at every level. The tree is then at most this plus log2(items)
/// deep, which bounds the traversal stack.
const MAX_HEURISTIC_DEPTH: usize = 64;
/// The traversal stack: deep enough for any tree `build` makes of up to 2^64 items.
const STACK: usize = MAX_HEURISTIC_DEPTH + 64;

#[derive(Clone)]
pub struct Bvh<T: Geometry> {
    nodes: Vec<Node>,
    items: Vec<Arc<T>>,
    /// The items' indices in leaf order: a leaf holds `order[start .. start + count]`.
    order: Vec<usize>,
    /// If every item is a triangle, the triangles in leaf order (as `order`).
    triangles: Option<Vec<EdgeTriangle>>,
    bounds: BBox,
}

/// A node of the tree, stored depth first: an interior node's first child follows it.
#[derive(Clone, Copy)]
struct Node {
    min: [f64; 3],
    max: [f64; 3],
    /// Leaf: the first of its items in `order`. Interior: the index of its second child.
    offset: u32,
    /// Leaf: how many items it holds (at least one). Interior: zero.
    count: u32,
    /// Interior: the axis its children are divided along.
    axis: u8,
}

/// A node during construction.
enum Build {
    Leaf { bounds: BBox, items: Vec<usize> },
    Interior { bounds: BBox, axis: u8, children: Box<(Build, Build)> },
}

fn surface_area(b: &BBox) -> f64 {
    let d = b.max - b.min;
    if d.x < 0. || d.y < 0. || d.z < 0. { return 0. }
    2. * (d.x * d.y + d.y * d.z + d.z * d.x)
}

fn centroid(b: &BBox) -> Vector3<f64> {
    let c = (b.min.coords + b.max.coords) * 0.5;
    // Unbounded items (an infinite plane) have no centre; any finite point will do to place them.
    c.map(|x| if x.is_finite() { x } else { 0. })
}

fn build(items: Vec<usize>, depth: usize, bounds_of: &[BBox], centroids: &[Vector3<f64>]) -> Build {
    let bounds = items.iter().fold(BBox::empty(), |b, &i| b.union(&bounds_of[i]));
    let n = items.len();
    if n == 1 {
        return Build::Leaf { bounds, items };
    }

    let (cmin, cmax) = items.iter().fold(
        (Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY)),
        |(lo, hi), &i| (lo.inf(&centroids[i]), hi.sup(&centroids[i])));

    // Halve by count along the axis the centroids spread furthest: when too deep, or when all
    // centroids coincide and no plane separates them.
    let halve = |mut items: Vec<usize>| {
        let axis = (cmax - cmin).imax();
        items.select_nth_unstable_by(n / 2, |&a, &b| centroids[a][axis].total_cmp(&centroids[b][axis]));
        let far = items.split_off(n / 2);
        let children = join(n, || build(items, depth + 1, bounds_of, centroids), || build(far, depth + 1, bounds_of, centroids));
        Build::Interior { bounds, axis: axis as u8, children: Box::new(children) }
    };
    if depth >= MAX_HEURISTIC_DEPTH {
        return if n <= MAX_LEAF { Build::Leaf { bounds, items } } else { halve(items) };
    }

    // The cheapest division: (cost, axis, bins on the near side).
    let area = surface_area(&bounds);
    let mut best: Option<(f64, usize, usize)> = None;
    for axis in 0..3 {
        let extent = cmax[axis] - cmin[axis];
        if extent <= 0. { continue }
        let bin_of = |i: usize| (((centroids[i][axis] - cmin[axis]) / extent * BINS as f64) as usize).min(BINS - 1);
        let mut bins = [(0usize, BBox::empty()); BINS];
        for &i in &items {
            let b = &mut bins[bin_of(i)];
            b.0 += 1;
            b.1 = b.1.union(&bounds_of[i]);
        }
        // Items and bounds left of each plane, sweeping up; then right of it, sweeping down.
        let mut left = [(0usize, 0.); BINS - 1];
        let (mut count, mut b) = (0, BBox::empty());
        for s in 0 .. BINS - 1 {
            count += bins[s].0;
            b = b.union(&bins[s].1);
            left[s] = (count, surface_area(&b));
        }
        let (mut count, mut b) = (0, BBox::empty());
        for s in (0 .. BINS - 1).rev() {
            count += bins[s + 1].0;
            b = b.union(&bins[s + 1].1);
            let (nl, al) = left[s];
            if nl == 0 || count == 0 { continue }
            let cost = TRAVERSAL_COST + (al * nl as f64 + surface_area(&b) * count as f64) / area;
            if best.is_none_or(|(c, _, _)| cost < c) {
                best = Some((cost, axis, s + 1));
            }
        }
    }

    let (axis, split) = match best {
        // Not worth dividing, unless the leaf would be too large; a flat or empty-area node
        // (area 0, cost NaN) is divided by count.
        Some((cost, axis, split)) if cost < n as f64 || n > MAX_LEAF || area == 0. => (axis, split),
        _ if n <= MAX_LEAF => return Build::Leaf { bounds, items },
        _ => return halve(items),
    };
    let extent = cmax[axis] - cmin[axis];
    let (near, far): (Vec<usize>, Vec<usize>) = items.into_iter().partition(|&i| {
        (((centroids[i][axis] - cmin[axis]) / extent * BINS as f64) as usize).min(BINS - 1) < split
    });
    let children = join(n, || build(near, depth + 1, bounds_of, centroids), || build(far, depth + 1, bounds_of, centroids));
    Build::Interior { bounds, axis: axis as u8, children: Box::new(children) }
}

fn join<A: Send, B: Send>(n: usize, a: impl FnOnce() -> A + Send, b: impl FnOnce() -> B + Send) -> (A, B) {
    if n > PARALLEL_BUILD { rayon::join(a, b) } else { (a(), b()) }
}

fn flatten(b: Build, nodes: &mut Vec<Node>, order: &mut Vec<usize>) {
    let at = nodes.len();
    let corners = |b: &BBox| ([b.min.x, b.min.y, b.min.z], [b.max.x, b.max.y, b.max.z]);
    match b {
        Build::Leaf { bounds, items } => {
            let (min, max) = corners(&bounds);
            nodes.push(Node { min, max, offset: order.len() as u32, count: items.len() as u32, axis: 0 });
            order.extend(items);
        },
        Build::Interior { bounds, axis, children } => {
            let (min, max) = corners(&bounds);
            nodes.push(Node { min, max, offset: 0, count: 0, axis });
            let (first, second) = *children;
            flatten(first, nodes, order);
            nodes[at].offset = nodes.len() as u32;
            flatten(second, nodes, order);
        },
    }
}

/// A ray prepared for repeated box tests.
struct TraversalRay {
    ro: [f64; 3],
    invrd: [f64; 3],
    /// Box tests give distances in multiples of the ray direction; this converts them to
    /// world-space distances, which items report.
    len: f64,
}

/// Rounding in a box test can put the exit a little before a hit on the box's face (as on an
/// axis-aligned wall, whose box is flat); widening it by this factor keeps such hits (PBR 4th
/// ed. section 6.8.2 widens by 1 + 2 gamma(3), about 1 + 7e-16).
const EXIT_SLACK: f64 = 1. + 1e-12;

impl TraversalRay {
    fn new(r: &Ray) -> TraversalRay {
        TraversalRay {
            ro: [r.ro.x, r.ro.y, r.ro.z],
            invrd: [1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z],
            len: r.rd.norm(),
        }
    }

    /// Where the ray enters and leaves a node's box, as world-space distances; None if it
    /// misses, or the box lies outside `min` to `max`.
    fn enters(&self, n: &Node, min: f64, max: f64) -> Option<f64> {
        let (mut entry, mut exit) = (f64::NEG_INFINITY, f64::INFINITY);
        for a in 0..3 {
            let t1 = (n.min[a] - self.ro[a]) * self.invrd[a];
            let t2 = (n.max[a] - self.ro[a]) * self.invrd[a];
            // NaN: parallel to the slab and starting on one of its faces (0 * inf), so inside
            // the closed slab; it limits nothing.
            if t1.is_nan() || t2.is_nan() { continue }
            // Plain comparisons: NaN is excluded, so f64::min and max's NaN handling is wasted.
            let (near, far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
            if near > entry { entry = near }
            if far < exit { exit = far }
        }
        let (entry, exit) = (entry.max(0.) * self.len, exit * EXIT_SLACK * self.len);
        (entry <= exit && entry <= max && exit >= min).then_some(entry)
    }
}

impl<T: Geometry> Bvh<T> {
    /// A tree over `items`. `bounds` is what `bounds` reports: the space the items are
    /// arranged in, which may be larger than they fill (a repeating tile).
    pub fn new(bounds: BBox, items: &[Arc<T>]) -> Bvh<T> {
        let items: Vec<Arc<T>> = items.to_vec();
        let bounds_of: Vec<BBox> = items.iter().map(|x| x.bounds()).collect();
        let centroids: Vec<Vector3<f64>> = bounds_of.iter().map(centroid).collect();
        let (mut nodes, mut order) = (vec![], vec![]);
        if !items.is_empty() {
            flatten(build((0..items.len()).collect(), 0, &bounds_of, &centroids), &mut nodes, &mut order);
        }
        let triangles = order.iter()
            .map(|&i| items[i].vertices().map(|v| EdgeTriangle::new(&v)))
            .collect::<Option<Vec<EdgeTriangle>>>()
            .filter(|t| !t.is_empty());
        Bvh { nodes, items, order, triangles, bounds }
    }

    pub fn bounds(&self) -> BBox {
        self.bounds
    }

    pub fn raw_intersection(&self, r: &Ray, max: f64, min: f64) -> Option<RawIntersection> {
        self.closest_intersection(r, max, min).map(|x| x.1)
    }

    pub fn intersection(&self, r: &Ray, max: f64, min: f64) -> Option<(&T, RawIntersection)> {
        self.closest_intersection(r, max, min).map(|(i, hit)| (&*self.items[i], hit))
    }

    /// Whether anything is hit between `min` and `max` along the ray: the predicate form of
    /// `intersection`, for shadow rays (PBR 4th ed. section 6.1.3, `IntersectP`). It stops at
    /// the first hit found.
    pub fn occluded(&self, r: &Ray, max: f64, min: f64) -> bool {
        let mut found = false;
        self.traverse(r, min, &mut found, |_| max, |found, i| {
            *found = self.items[i].intersects_beyond(r, min).is_some_and(|x| x.dist >= min && x.dist < max);
            *found
        });
        found
    }

    fn closest_intersection(&self, r: &Ray, max: f64, min: f64) -> Option<(usize, RawIntersection)> {
        let mut best: (f64, Option<(usize, RawIntersection)>) = (max, None);
        self.traverse(r, min, &mut best, |best| best.0, |best, i| {
            if let Some(x) = self.items[i].intersects_beyond(r, min)
                && x.dist >= min && x.dist < best.0 {
                *best = (x.dist, Some((i, x)));
            }
            false
        });
        best.1
    }

    /// Visit the leaves whose boxes the ray passes through between `min` and `limit(state)`,
    /// nearer child first, calling `test` with each item there (skipping, when the items are
    /// triangles, those the ray cannot hit before the limit), until it returns true. `test` may
    /// lower the limit.
    fn traverse<S>(&self, r: &Ray, min: f64, state: &mut S,
                   limit: impl Fn(&S) -> f64, mut test: impl FnMut(&mut S, usize) -> bool) {
        if self.nodes.is_empty() { return }
        let ray = TraversalRay::new(r);
        let toward_lower = [ray.invrd[0] < 0., ray.invrd[1] < 0., ray.invrd[2] < 0.];
        // Uninitialised: zeroing it was 7% of rendering the cornell box. Only entries below
        // `depth`, which have been written, are read.
        let mut stack = [std::mem::MaybeUninit::<u32>::uninit(); STACK];
        let mut depth = 0;
        let mut at = 0usize;
        loop {
            let node = &self.nodes[at];
            if ray.enters(node, min, limit(state)).is_some() {
                if node.count > 0 {
                    let (start, end) = (node.offset as usize, (node.offset + node.count) as usize);
                    if self.test_leaf(r, ray.len, (start, end), state, &limit, &mut test) {
                        return;
                    }
                } else {
                    // The first child holds the lower side of the dividing plane.
                    let (near, far) = if toward_lower[node.axis as usize] {
                        (node.offset, at as u32 + 1)
                    } else {
                        (at as u32 + 1, node.offset)
                    };
                    stack[depth].write(far);
                    depth += 1;
                    at = near as usize;
                    continue;
                }
            }
            if depth == 0 { return }
            depth -= 1;
            // SAFETY: stack[depth] was written when depth was last incremented past it.
            at = unsafe { stack[depth].assume_init() } as usize;
        }
    }

    fn test_leaf<S>(&self, r: &Ray, len: f64, (start, end): (usize, usize), state: &mut S,
                    limit: &impl Fn(&S) -> f64, test: &mut impl FnMut(&mut S, usize) -> bool) -> bool {
        match &self.triangles {
            Some(triangles) => {
                const LANE: [usize; LANES] = [0, 1, 2, 3];
                (start .. end).step_by(LANES).any(|group| {
                    let n = LANES.min(end - group);
                    let mut hits = may_hit(&triangles[group .. group + n], &LANE[..n], r, len, limit(state));
                    while hits != 0 {
                        let lane = hits.trailing_zeros() as usize;
                        hits &= hits - 1;
                        if test(state, self.order[group + lane]) {
                            return true;
                        }
                    }
                    false
                })
            },
            None => self.order[start .. end].iter().any(|&i| test(state, i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;
    use crate::shapes::triangle::Triangle;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    fn depth(nodes: &[Node], at: usize) -> usize {
        let n = &nodes[at];
        if n.count > 0 { 1 } else { 1 + depth(nodes, at + 1).max(depth(nodes, n.offset as usize)) }
    }

    /// The nearest hit by testing every item, which the tree must agree with exactly.
    fn brute_force<T: Geometry>(items: &[Arc<T>], r: &Ray, max: f64, min: f64) -> Option<f64> {
        items.iter()
            .filter_map(|x| x.intersects_beyond(r, min).map(|x| x.dist))
            .filter(|d| *d >= min && *d < max)
            .min_by(f64::total_cmp)
    }

    fn check<T: Geometry>(tree: &Bvh<T>, items: &[Arc<T>], r: &Ray, max: f64, min: f64) -> bool {
        let expected = brute_force(items, r, max, min);
        let got = tree.closest_intersection(r, max, min).map(|x| x.1.dist);
        assert_eq!(got, expected, "ray {} max {} min {}", r, max, min);
        assert_eq!(tree.occluded(r, max, min), expected.is_some(), "ray {} max {} min {}", r, max, min);
        expected.is_some()
    }

    #[test]
    fn test_spheres_match_brute_force() {
        let mut rng: StdRng = SeedableRng::from_seed([3; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        // Overlapping spheres of mixed sizes.
        let items: Vec<Arc<Sphere>> = (0..300).map(|_| Arc::new(Sphere::new(
            Vector3::new(u(-10., 10.), u(-10., 10.), u(-10., 10.)), u(0.2, 3.)))).collect();
        let bounds = items.iter().fold(BBox::empty(), |b, s| b.union(&s.bounds()));
        let tree = Bvh::new(bounds, &items);
        assert!(tree.nodes.len() > 20);
        let mut hits = 0;
        for i in 0..5000 {
            // Origins inside and outside the bounds; some limited by max, some by min.
            let r = Ray {
                ro: Vector3::new(u(-15., 15.), u(-15., 15.), u(-15., 15.)),
                rd: Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)) * u(0.5, 2.),
            };
            let (max, min) = match i % 3 { 0 => (f64::INFINITY, 0.), 1 => (u(1., 20.), 0.), _ => (f64::INFINITY, u(0., 5.)) };
            if check(&tree, &items, &r, max, min) { hits += 1 }
        }
        assert!(hits > 1000 && hits < 4900, "{}", hits);
    }

    #[test]
    fn test_triangle_surface_matches_brute_force() {
        // A rough surface of small triangles, as the ocean is, and rays from above it and
        // grazing along it. Leaves test their triangles in groups, before the exact test.
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
        let tree = Bvh::new(bounds, &items);
        assert!(tree.triangles.is_some());
        let mut hits = 0;
        for i in 0..20000 {
            let ro = Vector3::new(u(-5., 29.), u(-0.3, 4.), u(-5., 29.));
            let rd = Vector3::new(u(-1., 1.), if i % 2 == 0 { u(-1., 0.2) } else { u(-0.02, 0.02) }, u(-1., 1.)) * u(0.5, 2.);
            let (max, min) = match i % 3 { 0 => (f64::INFINITY, 0.), 1 => (u(1., 30.), 0.), _ => (f64::INFINITY, u(0., 5.)) };
            if check(&tree, &items, &Ray { ro, rd }, max, min) { hits += 1 }
        }
        assert!(hits > 2000, "{}", hits);
    }

    #[test]
    fn test_axis_aligned_boxes_and_rays_match_brute_force() {
        // Flat and solid boxes on a grid of whole numbers, and rays with zero (and -0)
        // direction components starting on the grid, running along faces: where box tests
        // give infinities, NaN, and entries equal to exits.
        let mut rng: StdRng = SeedableRng::from_seed([5; 32]);
        let mut int = |a: i32, b: i32| rng.gen_range(a, b + 1) as f64;
        let items: Vec<Arc<BBox>> = (0..120).map(|_| {
            let (x, y, z) = (int(-16, 14), int(-16, 14), int(-16, 14));
            Arc::new(BBox::new(Vector3::new(x, y, z), Vector3::new(x + int(0, 2), y + int(0, 2), z + int(0, 2))))
        }).collect();
        let tree = Bvh::new(BBox::new(Vector3::repeat(-16.), Vector3::repeat(16.)), &items);
        for _ in 0..20000 {
            let ro = Vector3::new(int(-20, 20), int(-20, 20), int(-20, 20));
            let mut component = || [-1., -0., 0., 1.][int(0, 3) as usize];
            let mut rd = Vector3::new(component(), component(), component());
            if rd.iter().all(|c| *c == 0.) { rd.x = 1. }
            check(&tree, &items, &Ray { ro, rd }, f64::INFINITY, 0.);
        }
    }

    #[test]
    fn test_identical_items_make_a_balanced_tree() {
        // No plane separates items with the same centre; they are halved by count instead.
        let items: Vec<Arc<BBox>> = (0..1000).map(|_| Arc::new(BBox::new(Vector3::repeat(-1.), Vector3::repeat(1.)))).collect();
        let tree = Bvh::new(BBox::new(Vector3::repeat(-1.), Vector3::repeat(1.)), &items);
        assert!(depth(&tree.nodes, 0) < 12, "{}", depth(&tree.nodes, 0));
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert!((tree.raw_intersection(&r, f64::INFINITY, 0.).unwrap().dist - 4.).abs() < 1e-12);
    }

    #[test]
    fn test_min_distance_skips_only_the_near_surface() {
        // One composite item with two parallel triangles. A shadow ray leaving the first (as
        // after a shadow bias, slightly behind it) must still find the second.
        use crate::shapes::csg::Union;
        let tri = |z: f64| -> Box<dyn Geometry + Sync + Send> {
            Box::new(Triangle::new(Vector3::new(-1., -1., z), Vector3::new(1., -1., z), Vector3::new(0., 1., z)))
        };
        let items = vec![Arc::new(Union::new(vec![tri(0.), tri(1.)]))];
        let tree = Bvh::new(BBox::new(Vector3::repeat(-2.), Vector3::repeat(2.)), &items);
        let r = Ray { ro: Vector3::new(0.1, 0.1, -1e-12), rd: Vector3::new(0., 0., 1.) };
        let hit = tree.raw_intersection(&r, 10., 0.001).expect("second triangle should occlude");
        assert!((hit.dist - 1.).abs() < 1e-6, "{}", hit.dist);
        assert!(tree.occluded(&r, 10., 0.001));
    }

    #[test]
    fn test_empty_tree_hits_nothing() {
        let tree: Bvh<Sphere> = Bvh::new(BBox::empty(), &[]);
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 1.) };
        assert!(tree.raw_intersection(&r, f64::INFINITY, 0.).is_none());
        assert!(!tree.occluded(&r, f64::INFINITY, 0.));
    }
}
