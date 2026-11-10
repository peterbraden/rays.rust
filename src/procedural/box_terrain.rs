use crate::shapes::infinite::Infinite;
use crate::shapes::sphere::Sphere;
use crate::sceneobject::SceneObject;
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::Pcg32;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, scatter_dielectric, diffuse};
use crate::shapes::geometry::Geometry;
use crate::material::lambertian::Lambertian;
use crate::bvh::GeometryBvh;
use crate::shapes::bbox::BBox;
use crate::intersection::RawIntersection;
use std::f64;
use crate::na::{Vector3};
use crate::ray::Ray;


pub struct BoxTerrain {
    boxes: GeometryBvh<BBox>,
    boxes_count: usize,
    bounds: BBox,
} 

impl Geometry for BoxTerrain {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.boxes.nearest(r, 0., f64::INFINITY).map(|x| x.1)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.boxes_count as u64
    }
}

const CELLS: i32 = 20;

/// Heights for the terrain's cells, row by row, as fractions of a cell's width. The same seed
/// always gives the same heights.
fn heights(seed: u64) -> Vec<f64> {
    let mut rng = Pcg32::new(seed, 0);
    (0 .. (2 * CELLS * 2 * CELLS)).map(|_| rng.next_f64() * rng.next_f64()).collect()
}

/// A 40 x 40 grid of boxes of random height, chosen by `seed`.
pub fn create_box_terrain(seed: u64) -> SceneObject {
    let mut heights = heights(seed).into_iter();
    let mut boxes_vec = vec![];
    let mut bounds = BBox::empty();
    let cube_size = 3.;
    for x in -CELLS .. CELLS {
        for z in -CELLS .. CELLS {
            let y = heights.next().unwrap();
            let b = BBox::new( 
                Vector3::new(
                    x as f64 * cube_size, 0., z as f64 *cube_size),
                Vector3::new(
                    x as f64 *cube_size + cube_size, y * cube_size,z as f64 * cube_size + cube_size)

            );
            bounds = bounds.union(&b);
            boxes_vec.push(b);
        }
    }

    let boxes_count = boxes_vec.len();
    let terrain = BoxTerrain { boxes: GeometryBvh::new(boxes_vec), boxes_count, bounds };

    SceneObject {
        geometry: Box::new(terrain),
        medium: Box::new(Solid { m: Box::new(Lambertian {
            albedo: Color::new(0.75, 0.75, 0.75)
        }) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_is_reproducible() {
        assert_eq!(heights(0), heights(0));
        assert_ne!(heights(0), heights(1));
        assert_eq!(heights(0).len(), 1600);
    }
}
