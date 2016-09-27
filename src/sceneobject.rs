use ray::Ray;
use intersection::Intersection;
use bbox::BBox;
use material::Material;
use na::Vector3;

pub trait SceneObject {

    fn intersects(&self, r: &Ray) -> Option<Intersection>;
//  fn intersectsP(&self, r: &Ray) -> bool;

    fn get_material(&self, point: Vector3<f64>) -> Material;

    // World space bounding box
    fn bounds(&self) -> BBox;
}
