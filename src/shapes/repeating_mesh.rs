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

const MAX_DEPTH: usize = 50;

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


    fn intersect_tile(&self, r: &Ray, transform: &Vector3<f64>, depth: usize) -> Option<RawIntersection> {
        if depth > MAX_DEPTH {
            return None;
        }

        let transformed_ray = Ray {
            ro: r.ro - transform,
            rd: r.rd,
        };
        //println!(" 2. ... {:?} {}", self.tile.bounds().intersects(&transformed_ray), transformed_ray);

        let intersects = self.tile.raw_intersection(&transformed_ray, f64::INFINITY, 0f64);
        //println!(">>> {:?} {}", intersects, transformed_ray);
        
        match intersects {
            Some(i) => {
                let mut intersection = i;
                // reverse the transform on the intersection
                intersection.point += transform;
                Some(intersection)
            },
            None => {
                match self.find_next_tile(&transformed_ray, transform) {
                    Some(t) => {
                        self.intersect_tile(r, &t, depth + 1)
                    },
                    None => {
                        None
                    }
                }
            }
        }
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
    
        if r.ro.y > self.tile_bounds.max.y {
            // Looking down on plane
            // Assuming the ray is above the plane, the max y face will be the first intersected -
            // preceding rays will intersect the previous tile
            let (transform, dist) = self.find_tile_transform(r, denom, self.tile_bounds.max.y);
            if dist < 0. { return None }
            return self.intersect_tile(r, &transform, 1);
        }

        if r.ro.y < self.tile_bounds.min.y {
            // Looking up at plane
            let (transform, dist) = self.find_tile_transform(r, denom, self.tile_bounds.min.y);
            if dist < 0. { return None }
            return self.intersect_tile(r, &transform, 1);
        }


        // We are in the plane bounds. We need to start with the current positioned bounds.
        // - Find out what tile is at that point
        let ix = (r.ro.x / self.tile_size.x).floor();
        let iz = (r.ro.z / self.tile_size.z).floor();
        let transform = self.transform_for(ix, iz, &Vector3::new(0., 0., 0.));

        self.intersect_tile(r, &transform, 1)
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, self.tile_bounds.min.y, f64::MIN),
            Vector3::new(f64::MAX, self.tile_bounds.max.y, f64::MAX),
          )
    }
}
