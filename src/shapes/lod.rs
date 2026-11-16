//! Adaptive meshes: levels of detail for meshes too fine to see, such as distant trees.
//!
//! A tree is a few hundred thousand triangles, most of them leaves a few centimetres across.
//! Seen from a few hundred metres, a pixel covers tens of centimetres, so each pixel sees
//! hundreds of leaves: the detail costs memory and build time, and the renderer can only
//! average it away with samples. A coarser mesh made for the size of the pixels it will cover
//! looks the same, if it keeps what the averaging would have seen. Two kinds of geometry need
//! two ways of doing that:
//!
//! - Surfaces, such as bark, are simplified by vertex clustering, after Jarek Rossignac and Paul
//!   Borrel, "Multi-resolution 3D approximations for rendering complex scenes", Modeling in
//!   Computer Graphics, 455-465, 1993, doi:10.1007/978-3-642-78114-8_29: the vertices in each
//!   cell of a grid as large as the detail to drop are merged into one, at their mean, and the
//!   triangles that collapse are dropped. It keeps the mesh closed, needs no connectivity, and
//!   removes twigs thinner than a cell, which at that distance are thinner than a pixel.
//!
//! - Aggregates of many small separate pieces, such as leaves, can't be simplified as a surface:
//!   each leaf would collapse to nothing, and the crown with it. Instead, nearby leaves are
//!   gathered into clusters as large as the detail to drop, and each cluster is replaced by one
//!   rectangular `Card`, as Robert L. Cook, John Halstead, Maxwell Planck and David Ryu replace
//!   a plant's leaves by fewer, larger ones in "Stochastic Simplification of Aggregate Detail",
//!   SIGGRAPH 2007, doi:10.1145/1275808.1276476, keeping the area the eye sees. The card lies in
//!   the plane that the cluster's leaves cover most of: the major axis of their normals' second
//!   moment, `Σ a n nᵀ`, as of the SGGX microflake distribution that Eric Heitz, Jonathan Dupuy,
//!   Cyril Crassin and Carsten Dachsbacher fit to such aggregates ("The SGGX Microflake
//!   Distribution", EGSR 2015, doi:10.1111/cgf.12667). Its size is the area the leaves cover,
//!   with overlaps counted as for leaves scattered at random over the cluster's extent `R`:
//!   `R (1 - exp(-S / R))` of their projected area `S`, Beer-Lambert's law as in canopy models
//!   (Monsi and Saeki's, 1953).
//!
//!   A flat card shaded with its own normal would look like a flat card: the crown's sheen, its
//!   light and shade, come from leaves facing every way. So a card keeps its leaves' normals
//!   and faces (what each leaf's material needs, such as its autumn colour), and each ray that
//!   hits it is shaded as one of them, chosen in proportion to the area that leaf shows the ray,
//!   `a |n · d|`: the distribution of visible normals (Heitz, "Understanding the
//!   Masking-Shadowing Function in Microfacet-Based BRDFs", JCGT 3(2), 2014). Averaged over the
//!   samples of a pixel, the card reflects and transmits as the leaves it stands for would,
//!   which is the normal perturbation of a consolidated surface: in the limit, a microfacet
//!   model whose facets are the original leaves. As Loubet and Neyret's hybrid mesh-volume
//!   levels of detail (Guillaume Loubet and Fabrice Neyret, "Hybrid mesh-volume LoDs for
//!   all-scale pre-filtering of complex 3D assets", Eurographics 2017, doi:10.1111/cgf.13124),
//!   surfaces stay meshes while sub-pixel aggregates are pre-filtered.
//!
//! The level to use is chosen by the size of a pixel where the mesh is seen (`level_for`):
//! level `k` drops detail up to `base · 2^(k-1)`, `base` being the size of the aggregate's
//! pieces, and level 0 is the mesh itself. Choosing per placed instance, rather than per ray as
//! R-LODs do (Sung-Eui Yoon, Christian Lauterbach and Dinesh Manocha, "R-LODs: fast LOD-based
//! ray tracing of massive models", The Visual Computer 22, 2006, doi:10.1007/s00371-006-0062-y),
//! keeps every ray, from the camera, the lights or a bounce, meeting the same surface, so
//! shadows stay consistent; and it means only the levels in use are built.
use crate::na::{Matrix3, Vector3};
use crate::intersection::RawIntersection;
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::{Geometry, Tagged};
use crate::shapes::geometry_set::GeometrySet;
use crate::shapes::triangle::{SmoothTriangle, Triangle};
use std::collections::HashMap;
use std::sync::Arc;

/// The most constituents a card keeps: a cluster of more keeps an even sample of them.
pub const MAX_CONSTITUENTS: usize = 32;

/// One of the pieces a card stands for: its normal, area and face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constituent {
    pub normal: [f32; 3],
    pub area: f32,
    pub face: u32,
}

impl Constituent {
    fn normal(&self) -> Vector3<f64> {
        Vector3::new(self.normal[0] as f64, self.normal[1] as f64, self.normal[2] as f64)
    }
}

/// The constituents of every card of a level, one run per card.
#[derive(Default, Debug)]
pub struct Clusters {
    starts: Vec<u32>,
    items: Vec<Constituent>,
}

impl Clusters {
    fn push(&mut self, items: &[Constituent]) -> u32 {
        if self.starts.is_empty() {
            self.starts.push(0);
        }
        self.items.extend_from_slice(items);
        self.starts.push(self.items.len() as u32);
        (self.starts.len() - 2) as u32
    }

    pub fn len(&self) -> usize {
        self.starts.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The constituents of card `c`.
    pub fn of(&self, c: u32) -> &[Constituent] {
        let c = c as usize;
        &self.items[self.starts[c] as usize .. self.starts[c + 1] as usize]
    }

    /// The constituent of card `c` a ray along `d` sees, given `u` uniform in [0, 1): chosen in
    /// proportion to the area each shows the ray.
    pub fn choose(&self, c: u32, d: &Vector3<f64>, u: f64) -> &Constituent {
        let items = self.of(c);
        let shown = |k: &Constituent| k.area as f64 * k.normal().dot(d).abs();
        let total: f64 = items.iter().map(shown).sum();
        if total <= 0. {
            return &items[((u * items.len() as f64) as usize).min(items.len() - 1)];
        }
        let mut left = u * total;
        for k in items {
            left -= shown(k);
            if left < 0. {
                return k;
            }
        }
        &items[items.len() - 1]
    }
}

/// A uniform number in [0, 1) from the bits of a ray and where it hit: each sample's rays
/// differ, so each chooses afresh, but it's a function of the ray, so the choice doesn't depend
/// on the order the scene's objects are searched in, and doesn't use up the sampler's dimensions.
fn hash01(r: &Ray, a: f64, b: f64) -> f64 {
    let mut h: u64 = 0x9e37_79b9_7f4a_7c15;
    for x in [r.ro.x, r.ro.y, r.ro.z, r.rd.x, r.rd.y, r.rd.z, a, b] {
        h ^= x.to_bits();
        h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        h ^= h >> 31;
        h = h.wrapping_mul(0x94d0_49bb_1331_11eb);
        h ^= h >> 29;
    }
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// A rectangle standing for a cluster of small pieces (see the module's notes): `corner`, and
/// the edges `u` and `v` from it. A ray hitting it is shaded as one of its constituents, chosen
/// by `Clusters::choose`: the hit reports that one's normal and face. A piece kept as it is is
/// a card of one constituent, cut to the `triangle` of its edges.
pub struct Card {
    corner: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    normal: Vector3<f64>,
    /// Rows of the inverse of the edges' Gram matrix, for a point's coordinates along them.
    du: Vector3<f64>,
    dv: Vector3<f64>,
    triangle: bool,
    cluster: u32,
    clusters: Arc<Clusters>,
}

impl Card {
    fn new(corner: Vector3<f64>, u: Vector3<f64>, v: Vector3<f64>, triangle: bool, cluster: u32, clusters: Arc<Clusters>) -> Card {
        let (uu, uv, vv) = (u.dot(&u), u.dot(&v), v.dot(&v));
        let det = uu * vv - uv * uv;
        Card {
            corner, u, v, normal: u.cross(&v).normalize(),
            du: (u * vv - v * uv) / det, dv: (v * uu - u * uv) / det,
            triangle, cluster, clusters,
        }
    }

    fn hit(&self, r: &Ray) -> Option<(f64, f64, f64)> {
        let denom = r.rd.dot(&self.normal);
        if denom == 0. {
            return None;
        }
        let t = (self.corner - r.ro).dot(&self.normal) / denom;
        if t <= 0. || !t.is_finite() {
            return None;
        }
        let d = r.ro + r.rd * t - self.corner;
        let (a, b) = (d.dot(&self.du), d.dot(&self.dv));
        if a < 0. || b < 0. || if self.triangle { a + b > 1. } else { a > 1. || b > 1. } {
            return None;
        }
        Some((t, a, b))
    }
}

impl Geometry for Card {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let (t, a, b) = self.hit(r)?;
        let k = self.clusters.choose(self.cluster, &r.rd.normalize(), if self.triangle { 0. } else { hash01(r, a, b) });
        Some(RawIntersection { dist: t * r.rd.norm(), point: r.ro + r.rd * t, normal: k.normal(), face: k.face, part: 0 })
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.hit(r).is_some_and(|(t, _, _)| {
            let d = t * r.rd.norm();
            d >= min && d < max
        })
    }

    fn bounds(&self) -> BBox {
        let (a, b, c, d) = (self.corner, self.corner + self.u, self.corner + self.v, self.corner + self.u + self.v);
        BBox::new(a.inf(&b).inf(&c).inf(&d), a.sup(&b).sup(&c).sup(&d))
    }

    fn surface_area(&self) -> f64 {
        self.u.cross(&self.v).norm() * if self.triangle { 0.5 } else { 1. }
    }
}

/// The detail of one part of a model, from which levels are made.
pub enum Detail {
    /// A connected surface, simplified by vertex clustering.
    Surface(Vec<Arc<SmoothTriangle>>),
    /// Small separate pieces, each triangle tagged with its face, consolidated into cards.
    Aggregate(Vec<Arc<Tagged<Triangle>>>),
}

impl Detail {
    /// This part with detail smaller than `size` dropped; `size` 0 for the part as it is.
    pub fn geometry(&self, size: f64) -> Arc<dyn Geometry + Sync + Send> {
        match self {
            Detail::Surface(t) if size <= 0. => Arc::new(GeometrySet::new(t)),
            Detail::Aggregate(t) if size <= 0. => Arc::new(GeometrySet::new(t)),
            Detail::Surface(t) => Arc::new(GeometrySet::new(&simplify(t, size))),
            Detail::Aggregate(t) => {
                let (cards, _) = consolidate(t, size);
                Arc::new(GeometrySet::new(&cards))
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Detail::Surface(t) => t.is_empty(),
            Detail::Aggregate(t) => t.is_empty(),
        }
    }
}

/// Size of the pieces of an aggregate: the median longest edge of its triangles.
pub fn piece_size(triangles: &[Arc<Tagged<Triangle>>]) -> f64 {
    let mut edges: Vec<f64> = triangles.iter().map(|t| {
        let t = &t.item;
        (t.v1 - t.v0).norm().max((t.v2 - t.v1).norm()).max((t.v0 - t.v2).norm())
    }).collect();
    if edges.is_empty() {
        return 0.;
    }
    let mid = edges.len() / 2;
    *edges.select_nth_unstable_by(mid, f64::total_cmp).1
}

/// The level of detail to see a model with pieces `base` across at, where a pixel covers
/// `footprint`, with `pixels` the largest detail dropped, in pixels: the coarsest level `k`
/// whose `base · 2^(k-1)` is no larger than `pixels · footprint`, up to `max`.
pub fn level_for(base: f64, footprint: f64, pixels: f64, max: usize) -> usize {
    if base <= 0. || footprint <= 0. || !footprint.is_finite() {
        return 0;
    }
    let ratio = pixels * footprint / base;
    if ratio < 1. {
        return 0;
    }
    ((ratio.log2().floor() as usize) + 1).min(max)
}

/// The size of detail level `k` drops (see `level_for`).
pub fn level_size(base: f64, k: usize) -> f64 {
    if k == 0 { 0. } else { base * 2f64.powi(k as i32 - 1) }
}

/// The surface `triangles` with its vertices merged within cells of a grid `cell` across (see
/// the module's notes). Merged vertices take the mean of their positions, and of their normals.
pub fn simplify(triangles: &[Arc<SmoothTriangle>], cell: f64) -> Vec<Arc<SmoothTriangle>> {
    let key = |p: &Vector3<f64>| ((p.x / cell).floor() as i64, (p.y / cell).floor() as i64, (p.z / cell).floor() as i64);
    // Per cell: the sums of its vertices' positions and normals, and their count.
    type Sums = (Vector3<f64>, Vector3<f64>, f64);
    let mut cells: HashMap<(i64, i64, i64), Sums> = HashMap::new();
    for t in triangles {
        for (p, n) in [(t.v0, t.normalv0), (t.v1, t.normalv1), (t.v2, t.normalv2)] {
            let c = cells.entry(key(&p)).or_insert((Vector3::zeros(), Vector3::zeros(), 0.));
            c.0 += p;
            c.1 += n;
            c.2 += 1.;
        }
    }
    let mut seen = std::collections::HashSet::new();
    let mut out = vec![];
    for t in triangles {
        let (a, b, c) = (key(&t.v0), key(&t.v1), key(&t.v2));
        if a == b || b == c || c == a {
            continue;
        }
        // Each surviving triangle once, whichever way round it was listed.
        let mut sorted = [a, b, c];
        sorted.sort();
        if !seen.insert(sorted) {
            continue;
        }
        let vertex = |k| {
            let (p, n, count): Sums = cells[&k];
            (p / count, if n.norm() > 0. { n.normalize() } else { n })
        };
        let ((p0, n0), (p1, n1), (p2, n2)) = (vertex(a), vertex(b), vertex(c));
        let face = (p1 - p0).cross(&(p2 - p0));
        if face.norm() <= 1e-14 {
            continue;
        }
        // Where a merged normal has vanished or turned against the face, use the face's.
        let fix = |n: Vector3<f64>| if n.dot(&face) <= 0. { face.normalize() } else { n };
        out.push(Arc::new(SmoothTriangle::new(p0, p1, p2, fix(n0), fix(n1), fix(n2))));
    }
    out
}

/// The aggregate `triangles` gathered into clusters `size` across, each made a `Card` (see the
/// module's notes); a cluster of one or two triangles keeps them. Returns the cards, and their
/// constituents.
pub fn consolidate(triangles: &[Arc<Tagged<Triangle>>], size: f64) -> (Vec<Arc<Card>>, Arc<Clusters>) {
    let mut clusters = Clusters::default();
    // Each card's corner, edges, whether it's a kept triangle, and its cluster.
    type Made = (Vector3<f64>, Vector3<f64>, Vector3<f64>, bool, u32);
    let mut cards: Vec<Made> = vec![];
    let centre = |t: &Triangle| (t.v0 + t.v1 + t.v2) / 3.;
    let mut order: Vec<usize> = (0 .. triangles.len()).collect();
    let mut stack = vec![(0, order.len())];
    while let Some((start, end)) = stack.pop() {
        let group = &mut order[start .. end];
        let (mut lo, mut hi) = (Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY));
        let (mut clo, mut chi) = (lo, hi);
        for &i in group.iter() {
            let t = &triangles[i].item;
            for p in [t.v0, t.v1, t.v2] {
                lo = lo.inf(&p);
                hi = hi.sup(&p);
            }
            let c = centre(t);
            clo = clo.inf(&c);
            chi = chi.sup(&c);
        }
        let spread = chi - clo;
        let axis = spread.imax();
        if (hi - lo).max() > size && group.len() > 2 && spread[axis] > 0. {
            // Split at the median along the axis the centres spread furthest.
            let mid = group.len() / 2;
            group.select_nth_unstable_by(mid, |&a, &b| centre(&triangles[a].item)[axis].total_cmp(&centre(&triangles[b].item)[axis]));
            stack.push((start, start + mid));
            stack.push((start + mid, end));
            continue;
        }
        let members: Vec<&Tagged<Triangle>> = group.iter().map(|&i| &*triangles[i]).collect();
        if members.len() <= 2 {
            // Too few to gain by: keep them, each a card of one constituent, as a triangle.
            for m in members {
                let t = &m.item;
                let c = clusters.push(&[constituent(m)]);
                cards.push((t.v0, t.v1 - t.v0, t.v2 - t.v0, true, c));
            }
            continue;
        }
        if let Some((corner, u, v)) = card(&members) {
            let all: Vec<Constituent> = members.iter().map(|m| constituent(m)).collect();
            let kept: Vec<Constituent> = if all.len() <= MAX_CONSTITUENTS { all } else {
                let step = all.len() as f64 / MAX_CONSTITUENTS as f64;
                (0 .. MAX_CONSTITUENTS).map(|k| all[(k as f64 * step) as usize]).collect()
            };
            let c = clusters.push(&kept);
            cards.push((corner, u, v, false, c));
        }
    }
    let clusters = Arc::new(clusters);
    let cards = cards.into_iter()
        .map(|(corner, u, v, triangle, c)| Arc::new(Card::new(corner, u, v, triangle, c, clusters.clone())))
        .collect();
    (cards, clusters)
}

fn constituent(t: &Tagged<Triangle>) -> Constituent {
    let tri = &t.item;
    let n = tri.normal;
    Constituent {
        normal: [n.x as f32, n.y as f32, n.z as f32],
        area: (0.5 * (tri.v1 - tri.v0).cross(&(tri.v2 - tri.v0)).norm()) as f32,
        face: t.face,
    }
}

/// The card for a cluster: its corner and edges (see the module's notes).
fn card(members: &[&Tagged<Triangle>]) -> Option<(Vector3<f64>, Vector3<f64>, Vector3<f64>)> {
    let mut moment = Matrix3::zeros();
    let (mut centre, mut total) = (Vector3::zeros(), 0.);
    for m in members {
        let t = &m.item;
        let area = 0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm();
        moment += t.normal * t.normal.transpose() * area;
        centre += (t.v0 + t.v1 + t.v2) / 3. * area;
        total += area;
    }
    if total <= 0. {
        return None;
    }
    centre /= total;
    let eigen = moment.symmetric_eigen();
    let mut normal: Vector3<f64> = eigen.eigenvectors.column(eigen.eigenvalues.imax()).into_owned();
    if normal.y < 0. {
        normal = -normal;
    }
    // Projected area: what the leaves cover, looking along the normal.
    let shown: f64 = members.iter().map(|m| {
        let t = &m.item;
        0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm() * t.normal.dot(&normal).abs()
    }).sum();

    // Axes in the plane: the principal axes of the vertices' spread over it.
    let any = if normal.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let e1 = normal.cross(&any).normalize();
    let e2 = normal.cross(&e1);
    let points: Vec<(f64, f64)> = members.iter().flat_map(|m| [m.item.v0, m.item.v1, m.item.v2])
        .map(|p| ((p - centre).dot(&e1), (p - centre).dot(&e2))).collect();
    let n = points.len() as f64;
    let (mx, my) = points.iter().fold((0., 0.), |(x, y), p| (x + p.0 / n, y + p.1 / n));
    let (mut cxx, mut cyy, mut cxy) = (0., 0., 0.);
    for (x, y) in &points {
        cxx += (x - mx) * (x - mx);
        cyy += (y - my) * (y - my);
        cxy += (x - mx) * (y - my);
    }
    let angle = 0.5 * (2. * cxy).atan2(cxx - cyy);
    let (a1, a2) = (e1 * angle.cos() + e2 * angle.sin(), -e1 * angle.sin() + e2 * angle.cos());
    let (mut lo, mut hi) = ((f64::INFINITY, f64::INFINITY), (f64::NEG_INFINITY, f64::NEG_INFINITY));
    for m in members {
        for p in [m.item.v0, m.item.v1, m.item.v2] {
            let (x, y) = ((p - centre).dot(&a1), (p - centre).dot(&a2));
            lo = (lo.0.min(x), lo.1.min(y));
            hi = (hi.0.max(x), hi.1.max(y));
        }
    }
    let (w, h) = (hi.0 - lo.0, hi.1 - lo.1);
    let extent = w * h;
    if extent <= 0. || shown <= 0. {
        return None;
    }
    // The area covered, with overlaps as for pieces scattered at random over the extent.
    let covered = extent * (1. - (-shown / extent).exp());
    let k = (covered / extent).sqrt();
    let mid = centre + a1 * (0.5 * (lo.0 + hi.0)) + a2 * (0.5 * (lo.1 + hi.1));
    let (u, v) = (a1 * (w * k), a2 * (h * k));
    Some((mid - 0.5 * (u + v), u, v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    /// `n` small leaves scattered through a box `extent` across, facing about `up`.
    fn leaves(n: usize, extent: f64, size: f64, seed: u64) -> Vec<Arc<Tagged<Triangle>>> {
        let mut rng = Pcg32::new(seed, 1);
        (0 .. n).map(|k| {
            let at = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * extent;
            let tilt = Vector3::new(rng.next_f64() - 0.5, 0., rng.next_f64() - 0.5) * 0.6;
            let (a, b) = ((Vector3::x() + tilt.y * Vector3::y()).normalize(), (Vector3::z() - tilt.x * Vector3::y() + tilt.z * Vector3::x()).normalize());
            Arc::new(Tagged { item: Triangle::new(at, at + b * size, at + a * size), face: k as u32 })
        }).collect()
    }

    #[test]
    fn test_levels_by_pixel_size() {
        assert_eq!(level_for(0.1, 0.05, 1., 10), 0);
        assert_eq!(level_for(0.1, 0.1, 1., 10), 1);
        assert_eq!(level_for(0.1, 0.19, 1., 10), 1);
        assert_eq!(level_for(0.1, 0.2, 1., 10), 2);
        assert_eq!(level_for(0.1, 0.2, 0.5, 10), 1);
        assert_eq!(level_for(0.1, 1e9, 1., 6), 6);
        assert_eq!(level_size(0.1, 0), 0.);
        assert!((level_size(0.1, 3) - 0.4).abs() < 1e-12);
        // Just past the size a level drops, that level is chosen.
        for k in 1 .. 8 {
            let footprint = level_size(0.1, k) * 1.01;
            assert_eq!(level_for(0.1, footprint, 1., 10), k);
        }
    }

    #[test]
    fn test_cards_stand_for_their_leaves() {
        let all = leaves(2000, 4., 0.1, 1);
        let (cards, clusters) = consolidate(&all, 1.);
        assert!(cards.len() < all.len() / 8, "{} cards", cards.len());
        // Every card is no larger than the clusters, and keeps leaves from within them.
        let mut faces = vec![];
        for c in 0 .. clusters.len() as u32 {
            assert!(!clusters.of(c).is_empty() && clusters.of(c).len() <= MAX_CONSTITUENTS);
            faces.extend(clusters.of(c).iter().map(|k| k.face));
        }
        faces.sort_unstable();
        faces.dedup();
        // Few enough to a cluster that every leaf is kept.
        assert_eq!(faces.len(), all.len());
        for card in &cards {
            let b = card.bounds();
            assert!((b.max - b.min).max() <= 1.5, "{:?}", b);
        }
        // Seen from above, the cards cover what the leaves do, overlaps allowed for: about
        // the leaves' projected area for a sparse cluster, and never more than it.
        let shown: f64 = all.iter().map(|t| t.item.normal.y.abs() * 0.5 * (t.item.v1 - t.item.v0).cross(&(t.item.v2 - t.item.v0)).norm()).sum();
        let covered: f64 = cards.iter().map(|c| c.surface_area() * c.normal.y.abs()).sum();
        assert!(covered <= shown * 1.001 && covered > 0.5 * shown, "{} of {}", covered, shown);
    }

    #[test]
    fn test_card_shows_the_normal_of_a_leaf_it_stands_for() {
        // Half the leaves face up, half, smaller, face along x: from above only those facing up
        // show.
        let mut all = vec![];
        for k in 0 .. 40 {
            let at = Vector3::new((k % 7) as f64 * 0.1, (k % 5) as f64 * 0.05, (k / 7) as f64 * 0.1);
            let up = Triangle::new(at, at + Vector3::z() * 0.05, at + Vector3::x() * 0.05);
            let side = Triangle::new(at, at + Vector3::y() * 0.04, at + Vector3::z() * 0.04);
            all.push(Arc::new(Tagged { item: up, face: 2 * k }));
            all.push(Arc::new(Tagged { item: side, face: 2 * k + 1 }));
        }
        let (cards, _) = consolidate(&all, 10.);
        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        assert!(card.normal.y.abs() > 0.99, "{:?}", card.normal);
        let mut rng = Pcg32::new(3, 3);
        let (mut hits, mut slanted, mut side) = (0, 0, 0);
        for _ in 0 .. 2000 {
            let b = card.bounds();
            let (x, z) = (b.min.x + rng.next_f64() * (b.max.x - b.min.x), b.min.z + rng.next_f64() * (b.max.z - b.min.z));
            let down = Ray { ro: Vector3::new(x, 10., z), rd: -Vector3::y() };
            if let Some(h) = card.intersects(&down) {
                hits += 1;
                assert!(h.normal.y.abs() > 0.99 || h.normal.x.abs() > 0.99);
                assert!(h.normal.y.abs() > 0.99, "a leaf seen edge on was shown: {:?}", h.normal);
                assert_eq!(h.face % 2, 0);
                assert!((h.dist - (10. - h.point.y)).abs() < 1e-9);
            }
            // At 45 degrees, both kinds show, in proportion to their areas.
            let slant = Ray { ro: Vector3::new(x - 10., 10. + card.corner.y, z), rd: Vector3::new(1., -1., 0.) };
            if let Some(h) = card.intersects(&slant) {
                slanted += 1;
                side += h.face % 2;
            }
        }
        assert!(hits > 1000, "{} hits", hits);
        let expected = 0.04 * 0.04 / (0.04 * 0.04 + 0.05 * 0.05);
        assert!((side as f64 / slanted as f64 - expected).abs() < 0.05, "{} of {}", side, slanted);
    }

    #[test]
    fn test_a_few_pieces_are_kept_as_they_are() {
        let all = leaves(2, 4., 0.1, 2);
        let (cards, clusters) = consolidate(&all, 100.);
        assert_eq!((cards.len(), clusters.len()), (2, 2));
        for (card, leaf) in cards.iter().zip(&all) {
            let t = &leaf.item;
            let centre = (t.v0 + t.v1 + t.v2) / 3.;
            let r = Ray { ro: centre + t.normal, rd: -t.normal };
            let h = card.intersects(&r).expect("the kept triangle is hit");
            assert_eq!(h.face, leaf.face);
            assert!((h.point - centre).norm() < 1e-9 && (h.normal - t.normal).norm() < 1e-6);
            // Outside the triangle but inside its parallelogram: a miss.
            let beyond = t.v1 + (t.v2 - t.v0) * 0.6;
            assert!(card.intersects(&Ray { ro: beyond + t.normal, rd: -t.normal }).is_none());
        }
    }

    #[test]
    fn test_simplified_surface_keeps_its_shape() {
        // A sphere of radius 1, finely meshed.
        let (rings, sides) = (60, 120);
        // Shared vertices exactly shared, at the seam and the poles, as a mesh's are.
        let at = |i: usize, j: usize| {
            let (theta, phi) = (std::f64::consts::PI * i as f64 / rings as f64, 2. * std::f64::consts::PI * (j % sides) as f64 / sides as f64);
            let r = if i == 0 || i == rings { 0. } else { theta.sin() };
            Vector3::new(r * phi.cos(), theta.cos(), r * phi.sin())
        };
        let mut mesh = vec![];
        for i in 0 .. rings {
            for j in 0 .. sides {
                let (a, b, c, d) = (at(i, j), at(i, j + 1), at(i + 1, j + 1), at(i + 1, j));
                for [x, y, z] in [[a, b, c], [a, c, d]] {
                    if (y - x).cross(&(z - x)).norm() > 1e-12 {
                        mesh.push(Arc::new(SmoothTriangle::new(x, y, z, x, y, z)));
                    }
                }
            }
        }
        let coarse = simplify(&mesh, 0.2);
        assert!(coarse.len() * 10 < mesh.len() && coarse.len() > 100, "{} of {}", coarse.len(), mesh.len());
        let area = |m: &[Arc<SmoothTriangle>]| m.iter().map(|t| 0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm()).sum::<f64>();
        assert!((area(&coarse) / area(&mesh) - 1.).abs() < 0.1, "{} vs {}", area(&coarse), area(&mesh));
        for t in &coarse {
            for (v, n) in [(t.v0, t.normalv0), (t.v1, t.normalv1), (t.v2, t.normalv2)] {
                assert!((v.norm() - 1.).abs() < 0.1, "{:?}", v);
                assert!(n.dot(&v.normalize()) > 0.9);
            }
        }
        // Rays at the centre still hit it, from every side: it has no holes.
        let set = GeometrySet::new(&coarse);
        let mut rng = Pcg32::new(5, 5);
        let mut misses = 0;
        for _ in 0 .. 2000 {
            let d = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalize();
            match set.intersects(&Ray { ro: d * 5., rd: -d }) {
                Some(h) if (h.dist - 4.).abs() < 0.1 => {},
                _ => misses += 1,
            }
        }
        assert_eq!(misses, 0);
        // Detail smaller than a cell goes: a thin twig collapses.
        let twig: Vec<Arc<SmoothTriangle>> = mesh.iter().map(|t| {
            let s = |v: Vector3<f64>| Vector3::new(v.x * 0.01, v.y, v.z * 0.01);
            Arc::new(SmoothTriangle::new(s(t.v0), s(t.v1), s(t.v2), t.normalv0, t.normalv1, t.normalv2))
        }).collect();
        assert!(simplify(&twig, 0.2).len() * 50 < twig.len());
    }
}
