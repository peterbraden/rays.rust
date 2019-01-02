use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use octree::Octree;
use std::f64;
use shapes::geometry::Geometry;
use std::sync::Arc;

pub struct Primitive {
    item: Box<dyn Geometry + Sync + Send>
}

impl Geometry for Primitive {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.item.intersects(r);
    }
    fn bounds(&self) -> BBox {
        return self.item.bounds();
    }
}

pub struct Union {
    items: Octree<Primitive>,
    primitives: u64,
}

impl Union {
    pub fn new (geometries: Vec<Box<dyn Geometry + Sync + Send>>) -> Union {
        let mut count = 0;
        let mut bounds = BBox::min() ;
        let primitives = geometries.into_iter().map(|g| {
            count += 1;
            bounds = bounds.union(&g.bounds());
            Arc::new(Primitive { item: g })
        }).collect();
        let tree = Octree::new(8, bounds, &primitives); 
    
        return Union {
            items: tree,
            primitives: count,
        }
    }
}

impl Geometry for Union {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.items.raw_intersection(r, f64::INFINITY, 0f64);
    }
    fn bounds(&self) -> BBox {
        return self.items.bounds();
    }

    fn primitives(&self) -> u64 { 
        return self.primitives;
    }
}
