use crate::shapes::infinite::Infinite;
use crate::shapes::sphere::Sphere;
use crate::sceneobject::SceneObject;
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_on_unit_sphere, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, scatter_dielectric, diffuse};
use crate::shapes::geometry::Geometry;
use crate::material::lambertian::Lambertian;
use crate::octree::Octree;
use crate::shapes::bbox::BBox;
use crate::intersection::RawIntersection;
use std::f64;
use crate::na::{Vector3};
use crate::ray::Ray;
use std::sync::Arc;


pub struct BoxTerrain {
    boxes: Octree<BBox>,
    boxes_count: usize,
    bounds: BBox,
} 

impl Geometry for BoxTerrain {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.boxes.raw_intersection(r, f64::INFINITY, 0f64)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.boxes_count as u64
    }
}

pub fn create_box_terrain() -> SceneObject {
    let mut boxes_vec = vec![];
    let mut bounds = BBox::min();
    let cube_size = 3.;
    for x in -20 .. 20 {
        for z in -20 .. 20 {
            let y = rand() * rand();
            let b = BBox::new( 
                Vector3::new(
                    x as f64 * cube_size, 0., z as f64 *cube_size),
                Vector3::new(
                    x as f64 *cube_size + cube_size, y * cube_size,z as f64 * cube_size + cube_size)

            );
            bounds = bounds.union(&b);
            boxes_vec.push(Arc::new(b));
        }
    }

    let boxes = Octree::new(8, bounds, &boxes_vec);
    let terrain = BoxTerrain { boxes, boxes_count: boxes_vec.len(), bounds};

    SceneObject {
        geometry: Box::new(terrain),
        medium: Box::new(Solid { m: Box::new(Lambertian {
            albedo: Color::new(0.75, 0.75, 0.75)
        }) })
    }
}
