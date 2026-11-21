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
//!   Distribution", ACM TOG 34(4), SIGGRAPH 2015, doi:10.1145/2766988). Its size is the area the leaves cover,
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
use crate::bvh::Bvh;
use crate::shapes::triangle::{SmoothTriangle, Triangle};
use std::collections::HashMap;
use std::sync::Arc;

/// The most constituents a card keeps: a cluster of more keeps an even sample of them.
pub const MAX_CONSTITUENTS: usize = 32;

/// One of the pieces a card stands for, in 12 bytes: its normal, octahedrally encoded (Cigolle,
/// Donow, Evangelakos, Mara, McGuire and Meyer, "A Survey of Efficient Representations for
/// Independent Unit Vectors", JCGT 3(2), 2014), to within a hundredth of a degree; its area,
/// relative to the largest of its card's; and its face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constituent {
    normal: [i16; 2],
    area: u16,
    pub face: u32,
}

/// `n`, a unit vector, folded onto the octahedron and unfolded onto a square, in 16-bit steps.
fn encode_normal(n: &Vector3<f64>) -> [i16; 2] {
    let l1 = n.x.abs() + n.y.abs() + n.z.abs();
    let (x, y) = (n.x / l1, n.y / l1);
    let (x, y) = if n.z < 0. { ((1. - y.abs()) * x.signum(), (1. - x.abs()) * y.signum()) } else { (x, y) };
    [(x * 32767.).round() as i16, (y * 32767.).round() as i16]
}

fn decode_normal(e: [i16; 2]) -> Vector3<f64> {
    let (x, y) = (e[0] as f64 / 32767., e[1] as f64 / 32767.);
    let z = 1. - x.abs() - y.abs();
    let (x, y) = if z < 0. { ((1. - y.abs()) * x.signum(), (1. - x.abs()) * y.signum()) } else { (x, y) };
    Vector3::new(x, y, z).normalize()
}

impl Constituent {
    pub fn normal(&self) -> Vector3<f64> {
        decode_normal(self.normal)
    }
}

/// The constituents of every card of a level, one run per card.
#[derive(Default, Debug)]
pub struct Clusters {
    starts: Vec<u32>,
    items: Vec<Constituent>,
}

impl Clusters {
    /// A card's constituents, each a normal, area and face; returns its number.
    fn push(&mut self, items: &[(Vector3<f64>, f64, u32)]) -> u32 {
        if self.starts.is_empty() {
            self.starts.push(0);
        }
        let largest = items.iter().map(|k| k.1).fold(0., f64::max);
        self.items.extend(items.iter().map(|(n, area, face)| Constituent {
            normal: encode_normal(n),
            area: if largest > 0. { (area / largest * 65535.).round().max(1.) as u16 } else { 1 },
            face: *face,
        }));
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

/// The most cards in a leaf of a `CardSet`'s BVH.
const CARDS_PER_LEAF: usize = 4;

/// Marks a card that is a kept triangle, whose `data` is then its face.
const TRIANGLE: u32 = 1 << 31;

/// A rectangle standing for a cluster of small pieces (see the module's notes), in 40 bytes:
/// `corner`, and the edges `u` and `v` from it, in single precision (to within a micrometre on a
/// tree, far below a card's own error). A ray hitting it is shaded as one of its constituents,
/// chosen by `Clusters::choose`: the hit reports that one's normal and face. A piece kept as it
/// is, a cluster of one, is a card cut to the triangle of its edges, with `TRIANGLE` set in its
/// `data` and its face below; otherwise `data` is its cluster's number.
#[derive(Clone, Copy, Debug)]
pub struct Card {
    corner: [f32; 3],
    u: [f32; 3],
    v: [f32; 3],
    data: u32,
}

fn vector(a: &[f32; 3]) -> Vector3<f64> {
    Vector3::new(a[0] as f64, a[1] as f64, a[2] as f64)
}

fn single(v: &Vector3<f64>) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}

impl Card {
    pub fn corner(&self) -> Vector3<f64> { vector(&self.corner) }
    pub fn u(&self) -> Vector3<f64> { vector(&self.u) }
    pub fn v(&self) -> Vector3<f64> { vector(&self.v) }

    pub fn is_triangle(&self) -> bool {
        self.data & TRIANGLE != 0
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.u().cross(&self.v()).normalize()
    }

    /// Where a ray hits it: the multiple of its direction, and the coordinates along the edges.
    fn hit(&self, r: &Ray) -> Option<(f64, f64, f64)> {
        let (corner, u, v) = (self.corner(), self.u(), self.v());
        let n = u.cross(&v);
        let denom = r.rd.dot(&n);
        if denom == 0. {
            return None;
        }
        let t = (corner - r.ro).dot(&n) / denom;
        if t <= 0. || !t.is_finite() {
            return None;
        }
        // The coordinates along the edges, which needn't be at right angles: by the inverse of
        // their Gram matrix.
        let d = r.ro + r.rd * t - corner;
        let (uu, uv, vv, du, dv) = (u.dot(&u), u.dot(&v), v.dot(&v), d.dot(&u), d.dot(&v));
        let det = uu * vv - uv * uv;
        let (a, b) = ((du * vv - dv * uv) / det, (dv * uu - du * uv) / det);
        if !(a >= 0. && b >= 0.) || if self.is_triangle() { a + b > 1. } else { a > 1. || b > 1. } {
            return None;
        }
        Some((t, a, b))
    }

    pub fn bounds(&self) -> BBox {
        let (c, u, v) = (self.corner(), self.u(), self.v());
        let (a, b, d, e) = (c, c + u, c + v, c + u + v);
        BBox::new(a.inf(&b).inf(&d).inf(&e), a.sup(&b).sup(&d).sup(&e))
    }

    pub fn surface_area(&self) -> f64 {
        self.u().cross(&self.v()).norm() * if self.is_triangle() { 0.5 } else { 1. }
    }
}

/// The cards of a level of an aggregate, stored in one array behind a BVH (`crate::bvh::Bvh`),
/// with their constituents.
pub struct CardSet {
    bvh: Bvh,
    cards: Vec<Card>,
    clusters: Clusters,
}

impl CardSet {
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn clusters(&self) -> &Clusters {
        &self.clusters
    }

    /// The hit on card `i` by `r`, shaded as one of its constituents, given where it hits.
    fn shade(&self, i: usize, r: &Ray, (t, a, b): (f64, f64, f64)) -> RawIntersection {
        let card = &self.cards[i];
        let (normal, face) = if card.is_triangle() {
            (card.normal(), card.data & !TRIANGLE)
        } else {
            let k = self.clusters.choose(card.data, &r.rd.normalize(), hash01(r, a, b));
            (k.normal(), k.face)
        };
        RawIntersection { dist: t * r.rd.norm(), point: r.ro + r.rd * t, normal, face, part: 0 }
    }
}

impl Geometry for CardSet {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        if self.cards.is_empty() {
            return None;
        }
        // Search by distance alone, then choose the constituent of the nearest card only: many
        // cards may be hit on the way to it.
        let mut nearest = None;
        self.bvh.nearest(r, min, max, |i, best| {
            let hit = self.cards[i].hit(r)?;
            let dist = hit.0 * r.rd.norm();
            if dist < min || dist >= best {
                return None;
            }
            nearest = Some(hit);
            Some(RawIntersection { dist, point: Vector3::zeros(), normal: Vector3::zeros(), face: 0, part: 0 })
        }).map(|(i, x)| {
            // The last hit accepted is the nearest; find it again if a farther one came after.
            let hit = nearest.filter(|h| h.0 * r.rd.norm() == x.dist).or_else(|| self.cards[i].hit(r)).unwrap();
            self.shade(i, r, hit)
        })
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        !self.cards.is_empty() && self.bvh.any_hit(r, min, max, |i| self.cards[i].hit(r).is_some_and(|(t, _, _)| {
            let d = t * r.rd.norm();
            d >= min && d < max
        }))
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.cards.len() as u64
    }

    fn surface_area(&self) -> f64 {
        self.cards.iter().map(Card::surface_area).sum()
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
            Detail::Aggregate(t) => Arc::new(consolidate(t, size)),
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
/// module's notes); a cluster of one or two triangles keeps them.
pub fn consolidate(triangles: &[Arc<Tagged<Triangle>>], size: f64) -> CardSet {
    let mut clusters = Clusters::default();
    let mut cards: Vec<Card> = vec![];
    for group in groups(triangles, size) {
        let members: Vec<&Tagged<Triangle>> = group.iter().map(|&i| &*triangles[i]).collect();
        if members.len() <= 2 {
            // Too few to gain by: keep them, as triangles.
            for m in members {
                let t = &m.item;
                assert!(m.face & TRIANGLE == 0, "face {} is too large to keep", m.face);
                cards.push(Card { corner: single(&t.v0), u: single(&(t.v1 - t.v0)), v: single(&(t.v2 - t.v0)), data: m.face | TRIANGLE });
            }
            continue;
        }
        if let Some((corner, u, v)) = card(&members) {
            let all: Vec<(Vector3<f64>, f64, u32)> = members.iter().map(|m| {
                let t = &m.item;
                (t.normal, 0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm(), m.face)
            }).collect();
            let kept: Vec<(Vector3<f64>, f64, u32)> = if all.len() <= MAX_CONSTITUENTS { all } else {
                let step = all.len() as f64 / MAX_CONSTITUENTS as f64;
                (0 .. MAX_CONSTITUENTS).map(|k| all[(k as f64 * step) as usize]).collect()
            };
            let c = clusters.push(&kept);
            cards.push(Card { corner: single(&corner), u: single(&u), v: single(&v), data: c });
        }
    }
    // Kept for the whole render, and grown by pushing: give back the slack.
    clusters.starts.shrink_to_fit();
    clusters.items.shrink_to_fit();
    // A card is cheaper to test than a node to visit, and smaller: leaves of a few.
    let (bvh, cards) = Bvh::build_with_leaves(cards, Card::bounds, CARDS_PER_LEAF);
    CardSet { bvh, cards, clusters }
}

/// The clusters `consolidate` gathers the aggregate `triangles` into, as indices into it: split
/// at the median along the axis their centres spread furthest until each is no more than `size`
/// across, or has two triangles or fewer.
fn groups(triangles: &[Arc<Tagged<Triangle>>], size: f64) -> Vec<Vec<usize>> {
    let centre = |t: &Triangle| (t.v0 + t.v1 + t.v2) / 3.;
    let mut order: Vec<usize> = (0 .. triangles.len()).collect();
    let mut stack = vec![(0, order.len())];
    let mut out = vec![];
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
            let mid = group.len() / 2;
            group.select_nth_unstable_by(mid, |&a, &b| centre(&triangles[a].item)[axis].total_cmp(&centre(&triangles[b].item)[axis]));
            stack.push((start, start + mid));
            stack.push((start + mid, end));
            continue;
        }
        out.push(group.to_vec());
    }
    out
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
#[path = "lod_research.rs"]
mod research;

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
        let set = consolidate(&all, 1.);
        let (cards, clusters) = (set.cards(), set.clusters());
        assert!(cards.len() < all.len() / 8, "{} cards", cards.len());
        // Every card is no larger than the clusters, and keeps leaves from within them.
        let mut faces = vec![];
        for c in 0 .. clusters.len() as u32 {
            assert!(!clusters.of(c).is_empty() && clusters.of(c).len() <= MAX_CONSTITUENTS);
            faces.extend(clusters.of(c).iter().map(|k| k.face));
        }
        faces.extend(cards.iter().filter(|c| c.is_triangle()).map(|c| c.data & !TRIANGLE));
        faces.sort_unstable();
        faces.dedup();
        // Few enough to a cluster that every leaf is kept.
        assert_eq!(faces.len(), all.len());
        for card in cards {
            let b = card.bounds();
            assert!((b.max - b.min).max() <= 1.5, "{:?}", b);
        }
        // Seen from above, the cards cover what the leaves do, overlaps allowed for: about
        // the leaves' projected area for a sparse cluster, and never more than it.
        let shown: f64 = all.iter().map(|t| t.item.normal.y.abs() * 0.5 * (t.item.v1 - t.item.v0).cross(&(t.item.v2 - t.item.v0)).norm()).sum();
        let covered: f64 = cards.iter().map(|c| c.surface_area() * c.normal().y.abs()).sum();
        assert!(covered <= shown * 1.001 && covered > 0.5 * shown, "{} of {}", covered, shown);
        // The set is hit where its cards are, and gives a kept leaf's face.
        for card in cards.iter().take(50) {
            let centre = card.corner() + (card.u() + card.v()) * if card.is_triangle() { 1. / 3. } else { 0.5 };
            let h = set.intersects(&Ray { ro: centre + card.normal() * 10., rd: -card.normal() }).expect("a card is hit");
            assert!(faces.binary_search(&h.face).is_ok());
            assert!(h.dist <= 10. + 1e-6);
        }
    }

    #[test]
    fn test_normals_are_encoded_closely() {
        let mut rng = Pcg32::new(9, 9);
        for _ in 0 .. 10000 {
            let n = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalize();
            let back = decode_normal(encode_normal(&n));
            assert!(n.dot(&back) > (0.01f64).to_radians().cos(), "{:?} {:?}", n, back);
        }
        for n in [Vector3::x(), -Vector3::x(), Vector3::y(), -Vector3::y(), Vector3::z(), -Vector3::z()] {
            assert!((decode_normal(encode_normal(&n)) - n).norm() < 1e-6, "{:?}", n);
        }
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
        let set = consolidate(&all, 10.);
        assert_eq!(set.cards().len(), 1);
        let card = set.cards()[0];
        assert!(card.normal().y.abs() > 0.99, "{:?}", card.normal());
        let mut rng = Pcg32::new(3, 3);
        let (mut hits, mut slanted, mut side) = (0, 0, 0);
        for _ in 0 .. 2000 {
            let b = card.bounds();
            let (x, z) = (b.min.x + rng.next_f64() * (b.max.x - b.min.x), b.min.z + rng.next_f64() * (b.max.z - b.min.z));
            let down = Ray { ro: Vector3::new(x, 10., z), rd: -Vector3::y() };
            if let Some(h) = set.intersects(&down) {
                hits += 1;
                assert!(h.normal.y.abs() > 0.99, "a leaf seen edge on was shown: {:?}", h.normal);
                assert_eq!(h.face % 2, 0);
                assert!((h.dist - (10. - h.point.y)).abs() < 1e-6);
            }
            // At 45 degrees, both kinds show, in proportion to their areas.
            let slant = Ray { ro: Vector3::new(x - 10., 10. + card.corner().y, z), rd: Vector3::new(1., -1., 0.) };
            if let Some(h) = set.intersects(&slant) {
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
        let set = consolidate(&all, 100.);
        assert_eq!((set.cards().len(), set.clusters().len()), (2, 0));
        for leaf in &all {
            let t = &leaf.item;
            let centre = (t.v0 + t.v1 + t.v2) / 3.;
            let r = Ray { ro: centre + t.normal, rd: -t.normal };
            let h = set.intersects(&r).expect("the kept triangle is hit");
            assert_eq!(h.face, leaf.face);
            assert!((h.point - centre).norm() < 1e-6 && (h.normal - t.normal).norm() < 1e-6);
            assert!(set.occludes(&r, 0., 2.) && !set.occludes(&r, 0., 0.5));
            // Outside the triangle but inside its parallelogram: a miss.
            let beyond = t.v1 + (t.v2 - t.v0) * 0.6;
            assert!(set.intersects(&Ray { ro: beyond + t.normal, rd: -t.normal }).is_none());
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
