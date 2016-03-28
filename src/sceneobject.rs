use ray::Ray;
use intersection::Intersection;

pub trait SceneObject {
    fn intersects(&self, r: &Ray) -> Option<Intersection>;
}
