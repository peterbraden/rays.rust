use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use std::sync::Arc;

/// Many geometries traced as one, behind an octree: the triangles of a tree's bark or leaves, or
/// the instancing transforms placing many copies of one tree in a forest.
pub struct GeometrySet<T: Geometry> {
    tree: Octree<T>,
    bounds: BBox,
    primitives: u64,
}

impl<T: Geometry> GeometrySet<T> {
    pub fn new(items: &[Arc<T>]) -> GeometrySet<T> {
        let bounds = items.iter().fold(BBox::empty(), |b, t| b.union(&t.bounds()));
        GeometrySet {
            tree: Octree::new(8, bounds, items),
            bounds,
            primitives: items.iter().map(|t| t.primitives()).sum(),
        }
    }
}

impl<T: Geometry + Sync + Send> Geometry for GeometrySet<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.tree.raw_intersection(r, f64::INFINITY, 0f64)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    /// The primitives of every item: an instanced tree counts all its triangles each time.
    fn primitives(&self) -> u64 {
        self.primitives
    }
}
