use std::f64;
use crate::shapes::geometry::Geometry;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::{BBox, BoxFace};
use crate::shapes::triangle::Triangle;
use std::vec::Vec;
use std::sync::Arc;
use crate::octree::Octree;

/// Tiles to step through before approximating the remaining distance by the tile's mean surface.
/// Beyond this a ray is grazing, and the surface is far enough away that detail is lost anyway.
const MAX_TILES: usize = 200;

/// Infinite Y-Plane mesh of repeating tiles.
///
/// As we don't actually have an infinite mesh, this is achieved by working out which tile the ray 
/// should intersect with, and the related transform necessary, then using that transform to
/// calculate the intersection that would have ocurred.
///
/// We can't transform the mesh around, so instead, we apply the inverse transform to the ray -
/// this has the same affect because of relativity.
///
/// 
/// NB Tile size can be smaller than tile bbox - if the tile is an irregular shape.
/// Tile size is only a 2D vector - Y is always 0, but we keep it this shape for easier
/// multiplication.
pub struct RepeatingMesh {
    pub tile: Octree<Triangle>,
    pub tile_size: Vector3<f64>,
    pub tile_bounds: BBox,
    pub triangle_count: usize,
}


impl RepeatingMesh {

    fn transform_for(&self, stepx: f64, stepz: f64, curr_transform: &Vector3<f64>) -> Vector3<f64> {
        //println!(" :  {}x{}", stepx, stepz);
        curr_transform + self.tile_size.component_mul(&Vector3::new(stepx, 0., stepz))
    }

    fn find_tile_transform(&self, r: &Ray, denom: f64,  y: f64) -> (Vector3<f64>, f64) {
        let rdn = r.rd.normalize();
        let norm = Vector3::new(0., 1., 0.);
        let dist = -(norm.dot(&r.ro) - y) / denom;
        let point = r.ro + (rdn * dist);

        // - Find out what tile is at that point
        let ix = (point.x / self.tile_size.x).floor();
        let iz = (point.z / self.tile_size.z).floor();
        //println!("NEW ---");
        let transform = self.transform_for(ix, iz, &Vector3::new(0., 0., 0.));
        (transform, dist)
    } 

    fn find_next_tile(&self, transformed_ray: &Ray, curr_transform: &Vector3<f64>) -> Option<Vector3<f64>> {
        match self.tile.bounds().exit_face(transformed_ray){
            Some(BoxFace::Top) => None,
            Some(BoxFace::Bottom) => None,
            Some(BoxFace::Left) => Some(self.transform_for(-1., 0., curr_transform)),
            Some(BoxFace::Right) => Some(self.transform_for(1., 0., curr_transform)), 
            Some(BoxFace::Front) => Some(self.transform_for(0., -1., curr_transform)),
            Some(BoxFace::Back) => Some(self.transform_for(0., 1., curr_transform)),
            None => {
                // did not intersect with this bbox? Should never happen!
                //panic!("ERR: No intersection - {} - {} {}", self.tile.bounds(), curr_transform, transformed_ray);
                print!("ERR: No intersection - {} - {} {}", self.tile.bounds(), curr_transform, transformed_ray);
                None
            }
        }
    } 


    fn intersect_tile(&self, r: &Ray, first: &Vector3<f64>) -> Option<RawIntersection> {
        let mut transform = *first;
        for _ in 0 .. MAX_TILES {
            let transformed_ray = Ray {
                ro: r.ro - transform,
                rd: r.rd,
            };

            if let Some(mut intersection) = self.tile.raw_intersection(&transformed_ray, f64::INFINITY, 0f64) {
                // reverse the transform on the intersection
                intersection.point += transform;
                return Some(intersection);
            }
            transform = self.find_next_tile(&transformed_ray, &transform)?;
        }
        self.mean_surface_intersection(r)
    }

    /// Intersection with the plane halfway through the tile's height, facing the ray.
    fn mean_surface_intersection(&self, r: &Ray) -> Option<RawIntersection> {
        let y = (self.tile_bounds.min.y + self.tile_bounds.max.y) * 0.5;
        if r.rd.y.abs() < 1e-12 { return None }
        let t = (y - r.ro.y) / r.rd.y;
        if t <= 0. { return None }
        Some(RawIntersection {
            dist: t * r.rd.norm(),
            point: r.ro + r.rd * t,
            normal: Vector3::new(0., -r.rd.y.signum(), 0.),
        })
    }
}


impl Geometry for RepeatingMesh {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Find out whether it hits a tile / which tile it hits
        // - Find out point on ymax plane it intersects
        let rdn = r.rd.normalize();
        let norm = Vector3::new(0., 1., 0.);
        let denom = norm.dot(&rdn);
        if denom.abs() < 1e-12 { return None }
    
        if r.ro.y > self.tile_bounds.max.y {
            // Looking down on plane
            // Assuming the ray is above the plane, the max y face will be the first intersected -
            // preceding rays will intersect the previous tile
            let (transform, dist) = self.find_tile_transform(r, denom, self.tile_bounds.max.y);
            if dist < 0. { return None }
            return self.intersect_tile(r, &transform);
        }

        if r.ro.y < self.tile_bounds.min.y {
            // Looking up at plane
            let (transform, dist) = self.find_tile_transform(r, denom, self.tile_bounds.min.y);
            if dist < 0. { return None }
            return self.intersect_tile(r, &transform);
        }


        // We are in the plane bounds. We need to start with the current positioned bounds.
        // - Find out what tile is at that point
        let ix = (r.ro.x / self.tile_size.x).floor();
        let iz = (r.ro.z / self.tile_size.z).floor();
        let transform = self.transform_for(ix, iz, &Vector3::new(0., 0., 0.));

        self.intersect_tile(r, &transform)
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, self.tile_bounds.min.y, f64::MIN),
            Vector3::new(f64::MAX, self.tile_bounds.max.y, f64::MAX),
          )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A unit tile of flat ground at y=0, in a slab from y=-1 to y=1.
    fn ground() -> RepeatingMesh {
        let tris = vec![
            Arc::new(Triangle::new(Vector3::new(0., 0., 0.), Vector3::new(1., 0., 0.), Vector3::new(0., 0., 1.))),
            Arc::new(Triangle::new(Vector3::new(1., 0., 0.), Vector3::new(1., 0., 1.), Vector3::new(0., 0., 1.))),
        ];
        let bounds = BBox::new(Vector3::new(0., -1., 0.), Vector3::new(1., 1., 1.));
        RepeatingMesh {
            tile: Octree::new(4, bounds, &tris),
            tile_size: Vector3::new(1., 0., 1.),
            tile_bounds: bounds,
            triangle_count: 2,
        }
    }

    #[test]
    fn test_steep_ray_hits_first_tile() {
        let r = Ray { ro: Vector3::new(3.3, 5., 2.6), rd: Vector3::new(0.1, -1., 0.1) };
        let i = ground().intersects(&r).unwrap();
        assert!(i.point.y.abs() < 1e-9);
        assert!((i.point.x - 3.8).abs() < 1e-9, "{}", i.point);
    }

    #[test]
    fn test_grazing_ray_crossing_many_tiles_hits() {
        // Inside the slab, descending 1 in 500: crosses ~450 tiles before reaching y=0.
        let r = Ray { ro: Vector3::new(0.5, 0.9, 0.5), rd: Vector3::new(1., -0.002, 0.) };
        let i = ground().intersects(&r).expect("grazing ray should reach the surface");
        assert!(i.point.y.abs() < 1e-6, "{}", i.point);
        assert!((i.point.x - 450.5).abs() < 1e-3, "{}", i.point);
        assert_eq!(i.normal, Vector3::new(0., 1., 0.));
    }
}
