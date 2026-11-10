use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::na::Vector3;
use std::sync::Arc;

/// Many geometries traced as one, behind a bounding volume hierarchy: the triangles of a tree's
/// bark or leaves, or the instancing transforms placing many copies of the trees of a forest.
///
/// A BVH rather than the octree used elsewhere, because its items overlap: the crowns of
/// neighbouring trees, and the long thin triangles of bark. An octree copies each item into
/// every cell it touches, and a ray tests the item again in each cell it crosses; in a BVH each
/// item is in one leaf and is tested at most once per ray. On the forest demo, against octrees
/// for both the trees and their triangles, that cut the trees tested per ray from 101 to 18,
/// the triangles from 2600 to 75, and the render time 18-fold; single trees render 4 times
/// faster.
///
/// The hierarchy is built top down by the surface area heuristic: each node splits its items
/// into the two groups that minimise the expected cost of tracing a random ray through them,
/// the probability of hitting each group's box (its surface area, relative to the parent's)
/// times the number of items in it. Jeffrey Goldsmith and John Salmon, "Automatic Creation of
/// Object Hierarchies for Ray Tracing", IEEE Computer Graphics and Applications 7(5), 14-20,
/// 1987, doi:10.1109/MCG.1987.276983, and J. David MacDonald and Kellogg S. Booth, "Heuristics
/// for ray tracing using space subdivision", The Visual Computer 6(3), 153-166, 1990,
/// doi:10.1007/BF01911006. Candidate splits are found by binning item centroids along each
/// axis, as Ingo Wald, "On fast Construction of SAH-based Bounding Volume Hierarchies", IEEE
/// Symposium on Interactive Ray Tracing, 33-40, 2007, doi:10.1109/RT.2007.4342588 describes.
/// Traversal visits the nearer child first and skips any node entered beyond the nearest hit.
pub struct GeometrySet<T: Geometry> {
    items: Vec<Arc<T>>,
    nodes: Vec<Node>,
    primitives: u64,
}

/// A node: a leaf holds `count` items from `first` in `items`; an inner node (`count` 0) has
/// its children at `first` and `first + 1` in `nodes`.
struct Node {
    bounds: BBox,
    first: u32,
    count: u32,
}

const BINS: usize = 12;
/// A leaf with this many items or fewer isn't split.
const LEAF: usize = 2;
/// Cost of visiting a node, relative to testing an item.
const TRAVERSAL_COST: f64 = 0.5;

fn surface_area(b: &BBox) -> f64 {
    let d = (b.max - b.min).sup(&Vector3::zeros());
    2. * (d.x * d.y + d.y * d.z + d.z * d.x)
}

fn centroid(b: &BBox) -> Vector3<f64> {
    (b.min.coords + b.max.coords) * 0.5
}

impl<T: Geometry> GeometrySet<T> {
    pub fn new(items: &[Arc<T>]) -> GeometrySet<T> {
        let bounds: Vec<BBox> = items.iter().map(|t| t.bounds()).collect();
        let mut order: Vec<usize> = (0 .. items.len()).collect();
        let mut nodes = vec![];
        if !items.is_empty() {
            nodes.push(Node { bounds: BBox::empty(), first: 0, count: 0 });
            build(&bounds, &mut order, 0, &mut nodes, 0);
        }
        GeometrySet {
            items: order.iter().map(|&i| items[i].clone()).collect(),
            nodes,
            primitives: items.iter().map(|t| t.primitives()).sum(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Fill in `nodes[at]` for the items `order[..]`, which start at `offset` in the final order.
fn build(bounds: &[BBox], order: &mut [usize], offset: usize, nodes: &mut Vec<Node>, at: usize) {
    let all = order.iter().fold(BBox::empty(), |b, &i| b.union(&bounds[i]));
    let leaf = Node { bounds: all, first: offset as u32, count: order.len() as u32 };
    if order.len() <= LEAF {
        nodes[at] = leaf;
        return;
    }
    let centres = order.iter().fold(BBox::empty(), |b, &i| b.union_point(&centroid(&bounds[i])));
    let extent = centres.max - centres.min;

    // The cheapest split over every axis and bin boundary.
    let mut best: Option<(f64, usize, f64)> = None;
    for axis in 0 .. 3 {
        // A flat or NaN extent can't be split along.
        if extent[axis].is_nan() || extent[axis] <= 0. {
            continue;
        }
        let bin_of = |i: usize| (((centroid(&bounds[i])[axis] - centres.min[axis]) / extent[axis] * BINS as f64) as usize).min(BINS - 1);
        let mut boxes = [BBox::empty(); BINS];
        let mut counts = [0usize; BINS];
        for &i in order.iter() {
            let b = bin_of(i);
            boxes[b] = boxes[b].union(&bounds[i]);
            counts[b] += 1;
        }
        // Areas and counts of everything left of each boundary, then right of it.
        let mut left = [(0., 0); BINS];
        let (mut b, mut n) = (BBox::empty(), 0);
        for k in 0 .. BINS - 1 {
            b = b.union(&boxes[k]);
            n += counts[k];
            left[k] = (surface_area(&b), n);
        }
        let (mut b, mut n) = (BBox::empty(), 0);
        for k in (1 .. BINS).rev() {
            b = b.union(&boxes[k]);
            n += counts[k];
            let (left_area, left_n) = left[k - 1];
            if left_n == 0 || n == 0 {
                continue;
            }
            let cost = left_area * left_n as f64 + surface_area(&b) * n as f64;
            if best.is_none_or(|(c, _, _)| cost < c) {
                best = Some((cost, axis, centres.min[axis] + extent[axis] * k as f64 / BINS as f64));
            }
        }
    }
    let parent = surface_area(&all).max(1e-300);
    let split = match best {
        // Split only if it's expected to be cheaper than testing every item.
        Some((cost, axis, at)) if TRAVERSAL_COST + cost / parent < order.len() as f64 => (axis, at),
        _ => {
            nodes[at] = leaf;
            return;
        },
    };
    let mid = partition(order, |&i| centroid(&bounds[i])[split.0] < split.1);
    if mid == 0 || mid == order.len() {
        nodes[at] = leaf;
        return;
    }
    let children = nodes.len();
    nodes.push(Node { bounds: BBox::empty(), first: 0, count: 0 });
    nodes.push(Node { bounds: BBox::empty(), first: 0, count: 0 });
    nodes[at] = Node { bounds: all, first: children as u32, count: 0 };
    let (lo, hi) = order.split_at_mut(mid);
    build(bounds, lo, offset, nodes, children);
    build(bounds, hi, offset + mid, nodes, children + 1);
}

/// Move the items for which `left` holds to the front; return how many there are.
fn partition(order: &mut [usize], left: impl Fn(&usize) -> bool) -> usize {
    let mut mid = 0;
    for k in 0 .. order.len() {
        if left(&order[k]) {
            order.swap(k, mid);
            mid += 1;
        }
    }
    mid
}

impl<T: Geometry + Sync + Send> Geometry for GeometrySet<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        if self.nodes.is_empty() {
            return None;
        }
        let invrd = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        let len = r.rd.norm();
        // Distance along the ray to where it enters a node's box, if it does.
        let entry = |n: &Node| n.bounds.entry_exit(&r.ro, &invrd).map(|(t, _)| t.max(0.) * len);
        let mut nearest: Option<RawIntersection> = None;
        let mut stack: Vec<(usize, f64)> = Vec::with_capacity(64);
        if let Some(d) = entry(&self.nodes[0]) {
            stack.push((0, d));
        }
        while let Some((n, d)) = stack.pop() {
            if nearest.is_some_and(|h| d > h.dist) {
                continue;
            }
            let node = &self.nodes[n];
            if node.count > 0 {
                for item in &self.items[node.first as usize .. (node.first + node.count) as usize] {
                    if let Some(h) = item.intersects(r).filter(|h| nearest.is_none_or(|b| h.dist < b.dist)) {
                        nearest = Some(h);
                    }
                }
                continue;
            }
            let (a, b) = (node.first as usize, node.first as usize + 1);
            match (entry(&self.nodes[a]), entry(&self.nodes[b])) {
                // The nearer goes on top, to be visited first.
                (Some(da), Some(db)) => if da < db { stack.extend([(b, db), (a, da)]) } else { stack.extend([(a, da), (b, db)]) },
                (Some(da), None) => stack.push((a, da)),
                (None, Some(db)) => stack.push((b, db)),
                (None, None) => {},
            }
        }
        nearest
    }

    fn bounds(&self) -> BBox {
        self.nodes.first().map_or(BBox::empty(), |n| n.bounds)
    }

    /// The primitives of every item: an instanced tree counts all its triangles each time.
    fn primitives(&self) -> u64 {
        self.primitives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;
    use crate::geometry::Pcg32;

    /// Random overlapping spheres, and the nearest hit by testing every one.
    #[test]
    fn test_finds_the_nearest_hit_as_testing_everything_does() {
        let mut rng = Pcg32::new(1, 0);
        let mut u = || rng.next_f64();
        let spheres: Vec<Arc<Sphere>> = (0 .. 500).map(|_| {
            Arc::new(Sphere::new(Vector3::new(u() * 20. - 10., u() * 20. - 10., u() * 20. - 10.), 0.2 + u()))
        }).collect();
        let set = GeometrySet::new(&spheres);
        assert_eq!(set.len(), 500);
        for _ in 0 .. 2000 {
            let r = Ray {
                ro: Vector3::new(u() * 40. - 20., u() * 40. - 20., u() * 40. - 20.),
                rd: Vector3::new(u() - 0.5, u() - 0.5, u() - 0.5) * (0.5 + 3. * u()),
            };
            let expected = spheres.iter().filter_map(|s| s.intersects(&r)).map(|h| h.dist).fold(f64::INFINITY, f64::min);
            let got = set.intersects(&r).map_or(f64::INFINITY, |h| h.dist);
            assert!(got == expected, "{} vs {}", got, expected);
        }
        // Its bounds hold every item's.
        let b = set.bounds();
        assert!(spheres.iter().all(|s| b.contains(&s.bounds())));
    }

    #[test]
    fn test_empty_and_coincident_items() {
        let none: GeometrySet<Sphere> = GeometrySet::new(&[]);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert!(none.intersects(&r).is_none());
        // Items whose centres coincide can't be split; they still all get tested.
        let same: Vec<Arc<Sphere>> = (1 .. 10).map(|k| Arc::new(Sphere::new(Vector3::zeros(), k as f64 * 0.1))).collect();
        assert!((GeometrySet::new(&same).intersects(&r).unwrap().dist - 4.1).abs() < 1e-9);
    }
}
