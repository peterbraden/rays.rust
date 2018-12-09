use shapes::infinite::Infinite;
use shapes::sphere::Sphere;
use sceneobject::SceneObject;
use material::texture::{Solid, Medium};
use color::Color;
use intersection::Intersection;
use scene::Scene;
use geometry::{random_point_on_unit_sphere, rand};
use material::model::{MaterialModel, ScatteredRay};
use material::functions::{scatter_lambertian, scatter_dielectric, diffuse};
use shapes::geometry::Geometry;
use material::lambertian::Lambertian;
use octree::Octree;
use shapes::bbox::BBox;
use intersection::RawIntersection;
use std::f64;
use na::{Vector3};
use ray::Ray;
use std::sync::Arc;


pub struct BoxTerrain {
    boxes: Octree<BBox>,
    boxes_count: usize,
    bounds: BBox,
} 

impl Geometry for BoxTerrain {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        return self.boxes.raw_intersection(r, f64::INFINITY, 0f64);
    }

    fn bounds(&self) -> BBox {
        return self.bounds;
    }

    fn primitives(&self) -> u64 {
        return self.boxes_count as u64;
    }
}

pub fn create_box_terrain() -> SceneObject {
    let mut boxes_vec = vec![];
    let mut bounds = BBox::min();
    let cube_size = 3.;
    for x in -20 .. 20 {
        for z in -20 .. 20 {
            let y = rand() * rand();
            let b = BBox { 
                min: Vector3::new(
                    x as f64 * cube_size, 0., z as f64 *cube_size),
                max: Vector3::new(
                    x as f64 *cube_size + cube_size, y * cube_size,z as f64 * cube_size + cube_size)

            };
            bounds = bounds.union(&b);
            boxes_vec.push(Arc::new(b));
        }
    }

    let boxes = Octree::new(8, bounds, &boxes_vec);
    let terrain = BoxTerrain { boxes, boxes_count: boxes_vec.len(), bounds};

    return SceneObject {
        geometry: Box::new(terrain),
        medium: Box::new(Solid { m: Box::new(Lambertian {
            albedo: Color::new(0.75, 0.75, 0.75)
        }) })
    }
}
