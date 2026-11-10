use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use std::f64;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;
use crate::na::geometry::{Affine3, Point3, Rotation3, Translation3};
use crate::na::{Matrix3, Matrix4, Vector3};

/// An object placed in the scene by an affine transformation: any combination of scaling,
/// rotation, shearing and translation. See Pharr, Jakob and Humphreys, "Physically Based
/// Rendering", 4th ed., section 3.9 "Transformations" and section 3.10 "Applying Transformations"
/// (<https://pbr-book.org/4ed/Geometry_and_Transformations/Applying_Transformations.html>).
///
/// Rather than transforming the object, rays are transformed into its space by the inverse,
/// intersected there, and the hit mapped back: points by the transformation, and normals by the
/// inverse transpose of its linear part, which keeps them perpendicular to the surface under
/// non-uniform scaling (section 3.10.3).
///
/// # Instancing
///
/// The item is held by `Arc`, so many transforms can share one geometry: an *instance* (PBR 4th
/// ed., section 7.1.4 "Object Instancing"). Each instance costs a transform, its cached inverse
/// and a pointer, however large the shared geometry - a mesh's triangles and its acceleration
/// structure exist once. Because the item is only ever seen through the inverse-transformed ray,
/// it needs no knowledge of where, or how many times, it appears.
///
/// The inverse and the normal matrix are computed once here rather than per ray; inverting a 4x4
/// matrix for every ray tested against every instance would dominate the cost.
pub struct Transform {
    pub item: Arc<dyn Geometry + Sync + Send>,
    pub transform: Affine3<f64>,
    inverse: Affine3<f64>,
    /// The inverse transpose of the linear part, which maps object space normals to world space.
    normal_matrix: Matrix3<f64>,
    /// The scale factor if the transformation scales equally in every direction (a rotation,
    /// reflection and translation besides), which keeps area uniform, so the transformed surface
    /// can still be sampled as an area light.
    uniform_scale: Option<f64>,
}

impl Transform {
    /// `item` transformed by `transform`, which must be invertible.
    pub fn new(item: impl Into<Arc<dyn Geometry + Sync + Send>>, transform: Affine3<f64>) -> Transform {
        let inverse = transform.try_inverse()
            .unwrap_or_else(|| panic!("transformation can't be undone (it flattens the object): {}", transform.matrix()));
        let linear: Matrix3<f64> = transform.matrix().fixed_view::<3, 3>(0, 0).into();
        let normal_matrix = inverse.matrix().fixed_view::<3, 3>(0, 0).transpose();
        // Scaling equally in every direction: the columns are perpendicular and of equal length,
        // ie. L^T L = s^2 I.
        let gram = linear.transpose() * linear;
        let s2 = gram.trace() / 3.;
        let uniform_scale = ((gram - Matrix3::identity() * s2).norm() <= 1e-9 * s2).then(|| s2.sqrt());
        Transform { item: item.into(), transform, inverse, normal_matrix, uniform_scale }
    }

    pub fn rotate(
        item: impl Into<Arc<dyn Geometry + Sync + Send>>,
        roll: f64, pitch: f64, yaw: f64
    ) -> Transform {
        Transform::from_parts(item, Vector3::zeros(), Rotation3::from_euler_angles(roll, pitch, yaw), Vector3::new(1., 1., 1.))
    }

    /// Scale by `scale` (per axis), then rotate, then translate by `translate`.
    pub fn from_parts(
        item: impl Into<Arc<dyn Geometry + Sync + Send>>,
        translate: Vector3<f64>,
        rotation: Rotation3<f64>,
        scale: Vector3<f64>,
    ) -> Transform {
        let m: Matrix4<f64> = Translation3::from(translate).to_homogeneous()
            * rotation.to_homogeneous()
            * Matrix4::new_nonuniform_scaling(&scale);
        Transform::new(item, Affine3::from_matrix_unchecked(m))
    }

    /// A world-space ray in the object's space.
    fn to_object(&self, r: &Ray) -> Ray {
        Ray {
            ro: self.inverse.transform_point(&Point3::from(r.ro)).coords,
            rd: self.inverse.transform_vector(&r.rd),
        }
    }

    /// The ray in object space, and the factor that converts distances along it from world
    /// space to object space. An affine map takes a line to a line, scaling every length along
    /// it by the same factor: how much it stretches the direction.
    fn to_object_scaled(&self, r: &Ray) -> (Ray, f64) {
        let o = self.to_object(r);
        let k = o.rd.norm() / r.rd.norm();
        (o, k)
    }

    /// An object-space hit in world space, for the world ray `r`.
    fn to_world(&self, r: &Ray, i: RawIntersection) -> RawIntersection {
        let point = self.transform.transform_point(&Point3::from(i.point)).coords;
        RawIntersection {
            dist: (point - r.ro).norm(),
            point,
            normal: (self.normal_matrix * i.normal).normalize(),
            ..i
        }
    }
}

impl Geometry for Transform {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Intersect in object space, then map the hit back into world space.
        self.item.intersects(&self.to_object(r)).map(|i| self.to_world(r, i))
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.item.crossings(&self.to_object(r)).into_iter().map(|i| self.to_world(r, i)).collect()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.item.inside(&self.inverse.transform_point(&Point3::from(*p)).coords)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        let (o, k) = self.to_object_scaled(r);
        self.item.intersects_range(&o, min * k, max * k).map(|i| self.to_world(r, i))
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        let (o, k) = self.to_object_scaled(r);
        self.item.occludes(&o, min * k, max * k)
    }

    fn fast_intersects(&self, r: &Ray) -> bool {
        self.item.fast_intersects(&self.to_object(r))
    }

    /// The item's bounds under the transformation (unlimited if they are).
    fn bounds(&self) -> BBox {
        self.item.transformed_bounds(&self.transform)
    }

    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        self.item.transformed_bounds(&(t * self.transform))
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }

    fn surface_area(&self) -> f64 {
        self.uniform_scale.map_or(0., |s| self.item.surface_area() * s * s)
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        // Uniform scaling keeps samples uniform by area; otherwise the item isn't sampled.
        self.uniform_scale?;
        let (p, n) = self.item.sample_surface(u)?;
        let point = self.transform.transform_point(&Point3::from(p)).coords;
        let normal = (self.normal_matrix * n).normalize();
        Some((point, normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use std::f64::consts::FRAC_PI_2;

    use crate::shapes::sphere::Sphere;

    fn unit_sphere() -> Box<dyn Geometry + Sync + Send> {
        Box::new(Sphere::new(Vector3::zeros(), 1.))
    }

    #[test]
    fn test_scaled_and_translated() {
        // A unit sphere scaled by 2 and moved to z = 5: its near side is at z = 3.
        let t = Transform::from_parts(unit_sphere(), Vector3::new(0., 0., 5.), Rotation3::identity(), Vector3::new(2., 2., 2.));
        let i = t.intersects(&Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 1.) }).unwrap();
        assert!((i.dist - 3.).abs() < 1e-9, "{}", i.dist);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-9);
        assert!((t.surface_area() - 16. * std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn test_non_uniform_scale_makes_an_ellipsoid() {
        let t = Transform::from_parts(unit_sphere(), Vector3::zeros(), Rotation3::identity(), Vector3::new(2., 1., 1.));
        let along_x = t.intersects(&Ray { ro: Vector3::new(10., 0., 0.), rd: Vector3::new(-1., 0., 0.) }).unwrap();
        let along_y = t.intersects(&Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!((along_x.point.x - 2.).abs() < 1e-9 && (along_y.point.y - 1.).abs() < 1e-9);
        // On the 45-degree point of the ellipse x^2/4 + y^2 = 1, the normal is (x/4, y) normalised,
        // not the scaled sphere normal: the inverse transpose at work.
        let p = Vector3::new(2f64.sqrt(), 0.5f64.sqrt(), 0.);
        let i = t.intersects(&Ray { ro: p * 2., rd: -p }).unwrap();
        let expected = Vector3::new(p.x / 4., p.y, 0.).normalize();
        assert!((i.normal - expected).norm() < 1e-9, "{} vs {}", i.normal, expected);
        // Area isn't uniform any more, so it can't be sampled as a light.
        assert_eq!(t.surface_area(), 0.);
        assert!(t.sample_surface([0.1, 0.2, 0.3]).is_none());
    }

    #[test]
    fn test_scale_rotate_translate_order() {
        // A box 2 long in x, 1 elsewhere, at the origin: scaled by 2, then rotated 90 degrees
        // about y (x becomes -z), then moved up 10. Its far end is at z = -4, y within [10, 12].
        let b: Box<dyn Geometry + Sync + Send> = Box::new(BBox::new(Vector3::zeros(), Vector3::new(2., 1., 1.)));
        let t = Transform::from_parts(b, Vector3::new(0., 10., 0.), Rotation3::from_euler_angles(0., FRAC_PI_2, 0.), Vector3::new(2., 2., 2.));
        let bounds = t.bounds();
        assert!((bounds.min.z + 4.).abs() < 1e-9, "{}", bounds);
        assert!((bounds.min.y - 10.).abs() < 1e-9 && (bounds.max.y - 12.).abs() < 1e-9, "{}", bounds);
    }

    #[test]
    fn test_samples_lie_on_transformed_surface() {
        let t = Transform::from_parts(unit_sphere(), Vector3::new(1., 2., 3.), Rotation3::from_euler_angles(0.3, 0.2, 0.1), Vector3::new(3., 3., 3.));
        for i in 0..20 {
            let u = i as f64 / 20.;
            let (p, n) = t.sample_surface([u, (u * 7.) % 1., 0.]).unwrap();
            assert!(((p - Vector3::new(1., 2., 3.)).norm() - 3.).abs() < 1e-9);
            assert!((n - (p - Vector3::new(1., 2., 3.)) / 3.).norm() < 1e-9);
        }
    }

    #[test]
    fn test_crossings_and_inside_follow_the_transform() {
        let t = Transform::from_parts(unit_sphere(), Vector3::new(0., 0., 5.), Rotation3::identity(), Vector3::new(2., 1., 1.));
        let c = t.crossings(&Ray { ro: Vector3::new(-10., 0., 5.), rd: Vector3::new(1., 0., 0.) });
        assert_eq!(c.len(), 2);
        assert!((c[0].dist - 8.).abs() < 1e-9 && (c[1].dist - 12.).abs() < 1e-9);
        assert!((c[1].normal - Vector3::new(1., 0., 0.)).norm() < 1e-9);
        assert!(t.inside(&Vector3::new(1.5, 0., 5.)) && !t.inside(&Vector3::new(1.5, 0., 0.)));
    }

    #[test]
    fn test_uniform_scale_survives_rotation() {
        let t = Transform::from_parts(unit_sphere(), Vector3::new(1., 2., 3.), Rotation3::from_euler_angles(0.3, 0.2, 0.1), Vector3::repeat(3.));
        assert!((t.surface_area() - 36. * std::f64::consts::PI).abs() < 1e-9);
        // A shear is not uniform.
        let shear = Matrix4::new(1., 0.5, 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
        let t = Transform::new(unit_sphere(), Affine3::from_matrix_unchecked(shear));
        assert_eq!(t.surface_area(), 0.);
    }

    #[test]
    fn test_sheared_normals_stay_perpendicular() {
        // x += 0.5 y turns the unit cube into a slanted box; its slanted face's normal is
        // perpendicular to the slant.
        let shear = Matrix4::new(1., 0.5, 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
        let b: Box<dyn Geometry + Sync + Send> = Box::new(BBox::new(Vector3::zeros(), Vector3::repeat(1.)));
        let t = Transform::new(b, Affine3::from_matrix_unchecked(shear));
        let i = t.intersects(&Ray { ro: Vector3::new(5., 0.5, 0.5), rd: Vector3::new(-1., 0., 0.) }).unwrap();
        assert!((i.point.x - 1.25).abs() < 1e-9, "{}", i.point);
        let slant = Vector3::new(0.5, 1., 0.);
        assert!(i.normal.dot(&slant).abs() < 1e-9 && i.normal.x > 0., "{}", i.normal);
        assert!(t.inside(&Vector3::new(1.2, 0.9, 0.5)) && !t.inside(&Vector3::new(1.2, 0.1, 0.5)));
    }

    #[test]
    #[should_panic(expected = "can't be undone")]
    fn test_flattening_is_rejected() {
        Transform::from_parts(unit_sphere(), Vector3::zeros(), Rotation3::identity(), Vector3::new(1., 0., 1.));
    }

    #[test]
    fn test_unbounded_items_stay_unbounded() {
        let plane: Box<dyn Geometry + Sync + Send> = Box::new(crate::shapes::plane::HalfSpace::new(Vector3::new(0., 1., 0.), 0.));
        let t = Transform::rotate(plane, 0.5, 0., 0.);
        let b = t.bounds();
        assert!(b.min.coords.iter().all(|c| *c == f64::MIN) && b.max.coords.iter().all(|c| *c == f64::MAX));
        // And it's still hit.
        assert!(t.intersects(&Ray { ro: Vector3::new(0., 5., 0.), rd: Vector3::new(0., -1., 0.) }).is_some());
    }

    #[test]
    fn test_rotated_hit_is_in_world_space() {
        // A slab that is thin in x; rotating 90 degrees about y makes it thin in z.
        let slab = BBox::new(Vector3::new(-0.1, -1., -2.), Vector3::new(0.1, 1., 2.));
        let t = Transform::rotate(Box::new(slab) as Box<dyn Geometry + Sync + Send>, 0., FRAC_PI_2, 0.);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        let i = t.intersects(&r).unwrap();
        assert!((i.dist - 4.9).abs() < 1e-9, "dist {}", i.dist);
        assert!((i.point - Vector3::new(0., 0., -0.1)).norm() < 1e-9, "point {}", i.point);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-9, "normal {}", i.normal);
    }

    #[test]
    fn test_instances_share_one_item() {
        let shared: Arc<dyn Geometry + Sync + Send> = Arc::new(Sphere::new(Vector3::zeros(), 1.));
        let left = Transform::from_parts(shared.clone(), Vector3::new(-3., 0., 0.), Rotation3::identity(), Vector3::new(1., 1., 1.));
        let right = Transform::from_parts(shared.clone(), Vector3::new(3., 0., 0.), Rotation3::identity(), Vector3::new(2., 2., 2.));
        assert!(Arc::ptr_eq(&left.item, &right.item));
        assert_eq!(Arc::strong_count(&shared), 3);
        let down = |x: f64| Ray { ro: Vector3::new(x, 10., 0.), rd: Vector3::new(0., -1., 0.) };
        assert!((left.intersects(&down(-3.)).unwrap().point.y - 1.).abs() < 1e-9);
        assert!((right.intersects(&down(3.)).unwrap().point.y - 2.).abs() < 1e-9);
        assert!(left.intersects(&down(0.)).is_none() && !left.fast_intersects(&down(0.)));
        assert!(right.fast_intersects(&down(4.5)));
    }

    #[test]
    fn test_inside_is_tested_in_object_space() {
        let b: Box<dyn Geometry + Sync + Send> = Box::new(BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.)));
        let t = Transform::from_parts(b, Vector3::new(10., 0., 0.), Rotation3::identity(), Vector3::new(3., 1., 1.));
        assert!(t.inside(&Vector3::new(12.5, 0., 0.)));
        assert!(!t.inside(&Vector3::new(0., 0., 0.)));
    }

    #[test]
    fn test_ranges_are_in_world_distances() {
        // A unit sphere scaled by 3 at z = 10: hits at 7 and 13 from the origin. Object space
        // distances along the ray are a third of those, so the range must be converted.
        let t = Transform::from_parts(unit_sphere(), Vector3::new(0., 0., 10.), Rotation3::identity(), Vector3::new(3., 3., 3.));
        let r = Ray { ro: Vector3::zeros(), rd: Vector3::new(0., 0., 2.) };
        assert!((t.intersects_range(&r, 0., 100.).unwrap().dist - 7.).abs() < 1e-9);
        assert!((t.intersects_range(&r, 8., 100.).unwrap().dist - 13.).abs() < 1e-9);
        assert!((t.intersects_beyond(&r, 8.).unwrap().dist - 13.).abs() < 1e-9);
        assert!(t.intersects_range(&r, 0., 6.9).is_none());
        assert!(t.intersects_range(&r, 13.1, 100.).is_none());
        assert!(t.occludes(&r, 0., 7.1) && !t.occludes(&r, 0., 6.9) && t.occludes(&r, 8., 13.1));
    }
}
