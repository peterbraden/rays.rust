use ray::Ray;
use intersection::Intersection;
use material::Material;

pub trait SceneObject {
    fn intersects(&self, r: &Ray) -> Option<Intersection>;
    fn get_material(&self) -> Material;
}
