use ray::Ray;
use intersection::Intersection;
use material::Material;
use na::Vec3;

pub trait SceneObject {
    fn intersects(&self, r: &Ray) -> Option<Intersection>;
    fn get_material(&self, point: Vec3<f64>) -> Material;
}
