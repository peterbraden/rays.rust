use std::f64;
use shapes::geometry::Geometry;
use na::{Vector3};
use ray::Ray;
use intersection::RawIntersection;
use shapes::bbox::BBox;
use shapes::triangle::Triangle;
use std::vec::Vec;
use std::sync::Arc;
use octree::Octree;

/// Infinite Y-Plane mesh of repeating tiles.

pub struct RepeatingMesh {
    pub tile: Octree<Triangle>,
    pub tile_bounds: BBox,
    pub triangle_count: usize,
}

impl RepeatingMesh {
    fn find_tile_transform(&self, r: &Ray, denom: f64,  y: f64) -> (Vector3<f64>, f64) {
        let rdn = r.rd.normalize();
        let norm = Vector3::new(0., 1., 0.);
        let dist = -(norm.dot(&r.ro) - y) / denom;
        let point = r.ro + (rdn * dist);

        // - Find out what tile is at that point
        // -- (p - min) / size to int
        let sx = self.tile_bounds.max.x - self.tile_bounds.min.x;
        let sz = self.tile_bounds.max.z - self.tile_bounds.min.z;
        let ix = ((point.x - self.tile_bounds.min.x) / sx).floor();
        let iz = ((point.z - self.tile_bounds.min.z) / sz).floor();
        let bbx = ix * sx;
        let bbz = iz * sz;
        let transform = Vector3::new(bbx, 0., bbz);
        return (transform, dist);
    } 
}


impl Geometry for RepeatingMesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Find out whether it hits a tile / which tile it hits
        // - Find out point on ymax plane it intersects
        let rdn = r.rd.normalize();
        let norm = Vector3::new(0., 1., 0.);
        let denom = norm.dot(&rdn);
        if denom.abs() < 0. { return None }

        // Transform the ray to find the intersection
        let (transform, dist) = self.find_tile_transform(r, denom, self.tile_bounds.max.y);
        if dist < 0. { return None }

        let transformed_ray = Ray {
            ro: r.ro - transform,
            rd: r.rd,
        };
        let intersects = self.tile.raw_intersection(&transformed_ray, f64::INFINITY, 0f64);

        match intersects {
            Some(i) => {
                let mut intersection = i.clone();
                // reverse the transform on the intersection
                intersection.point = intersection.point + transform;
                return Some(intersection);
            },
            None => {
                // We've intersected with the plane, but not this tile. 
                // The ray has snuck under the max_y surface.
                // We _could_ iterate back to the next tile, and find out the actual
                // intersection. But we could also find the intersection of the
                // _min_ bbox and try that.
                let (transform_min, _dist_min) = self.find_tile_transform(r, denom, self.tile_bounds.min.y);
                let transformed_ray_min = Ray {
                    ro: r.ro - transform_min,
                    rd: r.rd,
                };
                let intersects_min = self.tile.raw_intersection(&transformed_ray_min,
                                                            f64::INFINITY, 0f64);
                match intersects_min {
                    Some(i) => {
                        let mut intersection = i.clone();
                        // reverse the transform on the intersection
                        intersection.point = intersection.point + transform_min;
                        return Some(intersection);
                    },
                    None => {
                        // In this situation the ray has gone all the way through the 
                        // bbox without hitting the mesh. Need a tiling mesh here.
                    }
                }
            }
        };
        return None
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(std::f64::MIN, self.tile_bounds.min.y, std::f64::MIN),
            Vector3::new(std::f64::MAX, self.tile_bounds.max.y, std::f64::MAX),
          )
    }
}
