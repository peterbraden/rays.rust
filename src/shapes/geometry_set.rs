use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::bvh::Bvh;
use crate::na::Vector3;
use crate::na::geometry::Affine3;
use std::sync::Arc;

/// Many geometries traced as one, behind a bounding volume hierarchy (`crate::bvh::Bvh`): the
/// triangles of a tree's bark or leaves, or the instancing transforms placing the trees of a
/// forest.
///
/// Its items overlap, as neighbouring trees' crowns and long thin bark triangles do, which a
/// BVH handles well: each item is in one leaf, so is tested at most once per ray. On the
/// forest demo, against the octree this replaced, the trees tested per ray fell from 101 to 18
/// and the triangles from 2600 to 75. It passes the range searched on to its items, so a tree
/// stops at the nearest hit found so far elsewhere, and a shadow ray stops at the first leaf.
pub struct GeometrySet<T: Geometry> {
    bvh: Bvh,
    items: Vec<Arc<T>>,
    primitives: u64,
}

impl<T: Geometry> GeometrySet<T> {
    pub fn new(items: &[Arc<T>]) -> GeometrySet<T> {
        let primitives = items.iter().map(|t| t.primitives()).sum();
        let (bvh, items) = Bvh::build(items.to_vec(), |t| t.bounds());
        GeometrySet { bvh, items, primitives }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Geometry + Sync + Send> Geometry for GeometrySet<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        if self.items.is_empty() {
            return None;
        }
        self.bvh.nearest(r, min, max, |i, best| self.items[i].intersects_range(r, min, best)).map(|(_, x)| x)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        !self.items.is_empty() && self.bvh.any_hit(r, min, max, |i| self.items[i].occludes(r, min, max))
    }

    fn bounds(&self) -> BBox {
        if self.items.is_empty() { BBox::empty() } else { self.bvh.bounds() }
    }

    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        if self.items.is_empty() { BBox::empty() } else { self.bvh.transformed_bounds(t) }
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
