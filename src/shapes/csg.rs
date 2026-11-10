//! Constructive solid geometry: shapes combined as sets of points (POV-Ray 3.7 reference:
//! "Constructive Solid Geometry").
//!
//! Combining works as in POV-Ray. Every surface crossing of every part is a candidate, kept if
//! the point is where the combination's surface should be, judged by which of the other parts
//! it is inside:
//!
//! - union: every crossing, including those inside other parts (which only show through
//!   transparent materials);
//! - merge: crossings not inside any other part, so a union with no inner surfaces;
//! - intersection: crossings inside all the other parts;
//! - difference, the first part less all the others: the first part's crossings outside all the
//!   others, and the others' crossings inside the first and outside the rest, turned inside out.
//!
//! So each part needs `crossings`, with outward normals, and `inside`. A combination is itself
//! a shape with both, so combinations nest. Points exactly on two parts' surfaces at once
//! (coincident faces) can go either way, as in POV-Ray: make cutting shapes overlap a little.

use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::bvh::GeometryBvh;
use crate::shapes::geometry::Geometry;
use crate::na::Vector3;
use std::sync::Arc;

type Part = Box<dyn Geometry + Sync + Send>;

/// The ways of combining shapes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Union,
    Merge,
    Intersection,
    Difference,
}

/// Where two boxes overlap (possibly empty, with min beyond max).
fn overlap(a: &BBox, b: &BBox) -> BBox {
    BBox::new(a.min.coords.sup(&b.min.coords), a.max.coords.inf(&b.max.coords))
}

/// A part wrapped so it can go in a bounding volume hierarchy.
struct Item(Part);

impl Geometry for Item {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.0.intersects(r)
    }

    fn bounds(&self) -> BBox {
        self.0.bounds()
    }
}

/// Parts combined by one of the operations.
pub struct Csg {
    op: Op,
    parts: Vec<(Arc<Item>, BBox)>,
    bounds: BBox,
    /// For unions, which show every part's nearest hit: the parts in a bounding volume hierarchy.
    tree: Option<GeometryBvh<Arc<Item>>>,
}

impl Csg {
    pub fn new(op: Op, parts: Vec<Part>) -> Csg {
        assert!(!parts.is_empty(), "{:?} needs at least one shape", op);
        let parts: Vec<(Arc<Item>, BBox)> = parts.into_iter().map(|p| { let b = p.bounds(); (Arc::new(Item(p)), b) }).collect();
        let union_bounds = || parts.iter().fold(BBox::empty(), |b, (_, pb)| b.union(pb));
        let bounds = match op {
            Op::Union | Op::Merge => union_bounds(),
            Op::Intersection => parts.iter().skip(1).fold(parts[0].1, |b, (_, pb)| overlap(&b, pb)),
            Op::Difference => parts[0].1,
        };
        let tree = (op == Op::Union).then(|| {
            let items: Vec<Arc<Item>> = parts.iter().map(|(p, _)| p.clone()).collect();
            GeometryBvh::new(items)
        });
        Csg { op, parts, bounds, tree }
    }

    fn inside_part(&self, j: usize, p: &Vector3<f64>) -> bool {
        let (part, b) = &self.parts[j];
        b.contains_point(p) && part.0.inside(p)
    }

    /// Whether a crossing of part `i` at `p` is on the combination's surface, and if so whether
    /// it is turned inside out.
    fn keep(&self, i: usize, p: &Vector3<f64>) -> Option<bool> {
        let others = || (0..self.parts.len()).filter(move |j| *j != i);
        let keep = match self.op {
            Op::Union => true,
            Op::Merge => !others().any(|j| self.inside_part(j, p)),
            Op::Intersection => others().all(|j| self.inside_part(j, p)),
            Op::Difference => {
                let cut = others().filter(|j| *j != 0).any(|j| self.inside_part(j, p));
                if i == 0 { !cut } else { self.inside_part(0, p) && !cut }
            },
        };
        keep.then_some(self.op == Op::Difference && i > 0)
    }

    /// Every part's crossings, with which part each is from, nearest first.
    fn candidates(&self, r: &Ray) -> Vec<(usize, RawIntersection)> {
        let rd = r.rd.normalize();
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let mut all: Vec<(usize, RawIntersection)> = Vec::new();
        for (i, (part, b)) in self.parts.iter().enumerate() {
            if b.entry_exit(&r.ro, &inv).is_some() {
                all.extend(part.0.crossings(r).into_iter().map(|x| (i, x)));
            }
        }
        all.sort_by(|a, b| a.1.dist.total_cmp(&b.1.dist));
        all
    }

    fn kept(&self, i: usize, x: RawIntersection) -> Option<RawIntersection> {
        self.keep(i, &x.point).map(|flip| if flip { RawIntersection { normal: -x.normal, ..x } } else { x })
    }
}

impl Geometry for Csg {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        if let Some(tree) = &self.tree {
            return tree.nearest(r, 0., f64::INFINITY).map(|x| x.1);
        }
        self.candidates(r).into_iter().find_map(|(i, x)| self.kept(i, x))
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.candidates(r).into_iter().filter_map(|(i, x)| self.kept(i, x)).collect()
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.parts.iter().map(|(p, _)| p.0.primitives()).sum()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        if !self.bounds.contains_point(p) {
            return false;
        }
        let n = self.parts.len();
        match self.op {
            Op::Union | Op::Merge => (0..n).any(|j| self.inside_part(j, p)),
            Op::Intersection => (0..n).all(|j| self.inside_part(j, p)),
            Op::Difference => self.inside_part(0, p) && !(1..n).any(|j| self.inside_part(j, p)),
        }
    }
}

/// A shape turned inside out: POV-Ray's `inverse`. Its inside is everything outside the shape,
/// and its normals point the other way.
pub struct Inverse(pub Part);

impl Geometry for Inverse {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.0.intersects(r).map(|x| RawIntersection { normal: -x.normal, ..x })
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.0.crossings(r).into_iter().map(|x| RawIntersection { normal: -x.normal, ..x }).collect()
    }

    fn bounds(&self) -> BBox {
        BBox::new(Vector3::repeat(f64::MIN), Vector3::repeat(f64::MAX))
    }

    fn primitives(&self) -> u64 {
        self.0.primitives()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        !self.0.inside(p)
    }
}

/// A shape with the parts of its surface outside another shape cut away, leaving it open:
/// POV-Ray's `clipped_by`. Unlike an intersection, the clipping shape adds no surface.
pub struct Clipped {
    item: Part,
    by: Part,
    bounds: BBox,
}

impl Clipped {
    pub fn new(item: Part, by: Part) -> Clipped {
        let bounds = overlap(&item.bounds(), &by.bounds());
        Clipped { item, by, bounds }
    }
}

impl Geometry for Clipped {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.crossings(r).into_iter().next()
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.item.crossings(r).into_iter().filter(|x| self.by.inside(&x.point)).collect()
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.item.inside(p) && self.by.inside(p)
    }
}

/// A part of a combined shape with a material of its own, numbered `part`: its hits say so,
/// unless a part within it has already claimed them (the innermost material wins, as in POV-Ray).
/// The object's medium then gives the part's (see `texture::PartMedia`).
pub struct Tagged {
    pub item: Part,
    pub part: u32,
}

impl Tagged {
    fn tag(&self, x: RawIntersection) -> RawIntersection {
        if x.part == 0 { RawIntersection { part: self.part, ..x } } else { x }
    }
}

impl Geometry for Tagged {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(r).map(|x| self.tag(x))
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.item.crossings(r).into_iter().map(|x| self.tag(x)).collect()
    }

    fn bounds(&self) -> BBox {
        self.item.bounds()
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.item.inside(p)
    }

    fn surface_area(&self) -> f64 {
        self.item.surface_area()
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        self.item.sample_surface(u)
    }
}

/// Many shapes, each showing its nearest hit: a union of shapes that don't overlap, in a
/// bounding volume hierarchy, for large collections such as particle systems.
pub struct Union {
    items: GeometryBvh<Item>,
    primitives: u64,
}

impl Union {
    pub fn new(geometries: Vec<Part>) -> Union {
        let items: Vec<Item> = geometries.into_iter().map(Item).collect();
        Union { primitives: items.len() as u64, items: GeometryBvh::new(items) }
    }
}

impl Geometry for Union {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.items.nearest(r, 0., f64::INFINITY).map(|x| x.1)
    }

    fn bounds(&self) -> BBox {
        self.items.bounds()
    }

    fn primitives(&self) -> u64 {
        self.primitives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::Sphere;
    use crate::shapes::plane::HalfSpace;

    fn v(x: f64, y: f64, z: f64) -> Vector3<f64> { Vector3::new(x, y, z) }

    fn sphere(x: f64, r: f64) -> Part {
        Box::new(Sphere::new(v(x, 0., 0.), r))
    }

    /// Sphere of radius 2 at the origin, with a radius 1 bite taken out at x = 2.
    fn bitten_sphere() -> Csg {
        Csg::new(Op::Difference, vec![sphere(0., 2.), sphere(2., 1.)])
    }

    fn assert_hit(g: &dyn Geometry, ro: Vector3<f64>, rd: Vector3<f64>, dist: f64, normal: Vector3<f64>) {
        let i = g.intersects(&Ray { ro, rd }).expect("expected a hit");
        assert!((i.dist - dist).abs() < 1e-6, "dist {} != {}", i.dist, dist);
        assert!((i.normal - normal).norm() < 1e-6, "normal {} != {}", i.normal, normal);
    }

    #[test]
    fn test_union_bounds_do_not_include_origin() {
        let far = |c: f64| -> Part { Box::new(Sphere::new(v(c, c, c), 1.)) };
        let u = Union::new(vec![far(500.), far(503.)]);
        assert_eq!(u.bounds().min.coords, v(499., 499., 499.));
        assert_eq!(u.bounds().max.coords, v(504., 504., 504.));
    }

    #[test]
    fn test_difference_hits_a_outside_b() {
        assert_hit(&bitten_sphere(), v(0., 5., 0.), v(0., -1., 0.), 3., v(0., 1., 0.));
    }

    #[test]
    fn test_difference_passes_through_b_into_cavity_wall() {
        // Enters B at x=3, would enter A at x=2 but that is inside B, so hits B's far side at x=1.
        assert_hit(&bitten_sphere(), v(5., 0., 0.), v(-1., 0., 0.), 4., v(1., 0., 0.));
    }

    #[test]
    fn test_difference_starting_inside_b() {
        // In the cavity (inside A and B), heading into the material.
        assert_hit(&bitten_sphere(), v(1.5, 0., 0.), v(-1., 0., 0.), 0.5, v(1., 0., 0.));
    }

    #[test]
    fn test_difference_starting_inside_material() {
        // Inside A - B, heading out through A.
        assert_hit(&bitten_sphere(), v(-1., 0., 0.), v(-1., 0., 0.), 1., v(-1., 0., 0.));
        // Inside A - B, heading out through the cavity wall; the material lies at x < 1.
        assert_hit(&bitten_sphere(), v(0., 0., 0.), v(1., 0., 0.), 1., v(1., 0., 0.));
    }

    #[test]
    fn test_difference_miss() {
        assert!(bitten_sphere().intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., 1., 0.) }).is_none());
    }

    #[test]
    fn test_difference_of_many() {
        // Bites on both sides.
        let d = Csg::new(Op::Difference, vec![sphere(0., 2.), sphere(2., 1.), sphere(-2., 1.)]);
        assert_hit(&d, v(-5., 0., 0.), v(1., 0., 0.), 4., v(-1., 0., 0.));
        assert!(d.inside(&v(0., 0., 0.)) && !d.inside(&v(1.5, 0., 0.)) && !d.inside(&v(-1.5, 0., 0.)));
    }

    #[test]
    fn test_union_keeps_inner_surfaces_merge_does_not() {
        let parts = || vec![sphere(-0.5, 1.), sphere(0.5, 1.)];
        let u = Csg::new(Op::Union, parts());
        let m = Csg::new(Op::Merge, parts());
        let ray = Ray { ro: v(-5., 0., 0.), rd: v(1., 0., 0.) };
        let ud: Vec<f64> = u.crossings(&ray).iter().map(|x| x.dist).collect();
        let md: Vec<f64> = m.crossings(&ray).iter().map(|x| x.dist).collect();
        assert_eq!(ud.len(), 4);
        assert_eq!(md.len(), 2);
        assert!((md[0] - 3.5).abs() < 1e-9 && (md[1] - 6.5).abs() < 1e-9);
        // Both hit the near side first.
        assert_hit(&u, ray.ro, ray.rd, 3.5, v(-1., 0., 0.));
        assert_hit(&m, ray.ro, ray.rd, 3.5, v(-1., 0., 0.));
        // From inside the overlap, the union's first surface is an inner one.
        let from_mid = Ray { ro: v(0., 0., 0.), rd: v(1., 0., 0.) };
        assert!((u.intersects(&from_mid).unwrap().dist - 0.5).abs() < 1e-9);
        assert!((m.intersects(&from_mid).unwrap().dist - 1.5).abs() < 1e-9);
    }

    #[test]
    fn test_intersection_is_a_lens() {
        let i = Csg::new(Op::Intersection, vec![sphere(-0.5, 1.), sphere(0.5, 1.)]);
        assert_hit(&i, v(-5., 0., 0.), v(1., 0., 0.), 4.5, v(-1., 0., 0.));
        assert!(i.inside(&v(0., 0., 0.)) && !i.inside(&v(-1., 0., 0.)));
        let b = i.bounds();
        assert!((b.min.x + 0.5).abs() < 1e-12 && (b.max.x - 0.5).abs() < 1e-12);
        // Outside the lens, past the edge where the spheres meet.
        assert!(i.intersects(&Ray { ro: v(-5., 0.9, 0.), rd: v(1., 0., 0.) }).is_none());
    }

    #[test]
    fn test_half_space_cuts_a_sphere() {
        // The lower half of a sphere, by intersecting with the half-space below y = 0.
        let half = Csg::new(Op::Intersection, vec![sphere(0., 1.), Box::new(HalfSpace::new(v(0., 1., 0.), 0.))]);
        assert_hit(&half, v(0.3, 5., 0.), v(0., -1., 0.), 5., v(0., 1., 0.));
        assert!(half.bounds().max.y <= 0.);
    }

    #[test]
    fn test_nested_difference_of_union() {
        // Two overlapping spheres merged by a union, less a sphere on the far side: the inner
        // surfaces of the union must not confuse the difference.
        let u = Csg::new(Op::Union, vec![sphere(-0.5, 1.), sphere(0.5, 1.)]);
        let d = Csg::new(Op::Difference, vec![Box::new(u), sphere(1.5, 0.5)]);
        let ray = Ray { ro: v(5., 0., 0.), rd: v(-1., 0., 0.) };
        // Through the bite to its inner wall at x = 1.
        assert_hit(&d, ray.ro, ray.rd, 4., v(1., 0., 0.));
        assert!(d.inside(&v(0., 0., 0.)) && !d.inside(&v(1.25, 0., 0.)));
    }

    #[test]
    fn test_innermost_tag_wins() {
        let inner = Tagged { item: sphere(2., 1.), part: 2 };
        let d = Tagged { item: Box::new(Csg::new(Op::Difference, vec![sphere(0., 2.), Box::new(inner)])), part: 1 };
        // The outer surface is the difference's own part; the bite's wall is the inner part's.
        assert_eq!(d.intersects(&Ray { ro: v(0., 5., 0.), rd: v(0., -1., 0.) }).unwrap().part, 1);
        assert_eq!(d.intersects(&Ray { ro: v(5., 0., 0.), rd: v(-1., 0., 0.) }).unwrap().part, 2);
    }

    #[test]
    fn test_inverse_and_clipped() {
        let inv = Inverse(sphere(0., 1.));
        assert!(inv.inside(&v(2., 0., 0.)) && !inv.inside(&v(0., 0., 0.)));
        assert_hit(&inv, v(-5., 0., 0.), v(1., 0., 0.), 4., v(1., 0., 0.));
        // An intersection with an inverse is a difference.
        let d = Csg::new(Op::Intersection, vec![sphere(0., 2.), Box::new(Inverse(sphere(2., 1.)))]);
        assert_hit(&d, v(5., 0., 0.), v(-1., 0., 0.), 4., v(1., 0., 0.));
        // Clipping a sphere by a half-space leaves an open bowl: the ray down the middle passes
        // in through the missing top and hits the inside of the bottom.
        let bowl = Clipped::new(sphere(0., 1.), Box::new(HalfSpace::new(v(0., 1., 0.), 0.)));
        assert_hit(&bowl, v(0., 5., 0.), v(0., -1., 0.), 6., v(0., -1., 0.));
    }
}
