use ray::Ray;
use intersection::Intersection;
use bbox::BBox;
use material::Material;
use na::Vec3;
use shape::geometry::Geometry;

pub struct SceneObject {
    geometry: Geometry,
    material: Material
}
