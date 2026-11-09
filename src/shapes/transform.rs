use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use std::f64;
use crate::shapes::geometry::Geometry;
use std::sync::Arc;
use crate::na::geometry::{Affine3, Point3, Rotation3, Translation3};
use crate::na::{Matrix4, Vector3};

/// An object placed in the scene by an affine transformation: scaled, then rotated, then
/// translated. See Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., section
/// 3.9 "Transformations" and section 3.10 "Applying Transformations"
/// (<https://pbr-book.org/4ed/Geometry_and_Transformations/Applying_Transformations.html>).
///
/// Rather than transforming the object, rays are transformed into its space by the inverse,
/// intersected there, and the hit mapped back: points by the transformation, and normals by the
/// inverse transpose of its linear part, which keeps them perpendicular to the surface under
/// non-uniform scaling (section 3.10.3).
pub struct Transform {
    pub item: Box<dyn Geometry + Sync + Send>,
    pub transform: Affine3<f64>,
    /// The scale factor if the scaling is the same along every axis, which keeps area uniform,
    /// so the transformed surface can still be sampled as an area light.
    uniform_scale: Option<f64>,
} 

// Affine transformation (invertible)
impl Transform {
    pub fn new(item: Box<dyn Geometry + Sync + Send>) -> Transform {
        Transform {
            item,
            transform: Affine3::identity(),
            uniform_scale: Some(1.),
        }
    }

    pub fn rotate(
        item:  Box<dyn Geometry + Sync + Send>,
        roll: f64, pitch: f64, yaw: f64
    ) -> Transform {
        Transform::from_parts(item, Vector3::zeros(), Rotation3::from_euler_angles(roll, pitch, yaw), Vector3::new(1., 1., 1.))
    }

    /// Scale by `scale` (per axis), then rotate, then translate by `translate`.
    pub fn from_parts(
        item: Box<dyn Geometry + Sync + Send>,
        translate: Vector3<f64>,
        rotation: Rotation3<f64>,
        scale: Vector3<f64>,
    ) -> Transform {
        let m: Matrix4<f64> = Translation3::from(translate).to_homogeneous()
            * rotation.to_homogeneous()
            * Matrix4::new_nonuniform_scaling(&scale);
        let uniform_scale = if scale.x == scale.y && scale.y == scale.z { Some(scale.x.abs()) } else { None };
        Transform { item, transform: Affine3::from_matrix_unchecked(m), uniform_scale }
    }
}

impl Geometry for Transform {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Intersect in object space, then map the hit back into world space.
        self.item.intersects(&r.inverse_transform(&self.transform)).map(|i| {
            let point = self.transform.transform_point(&Point3::from(i.point)).coords;
            // Normals transform by the inverse transpose of the linear part.
            let normal_matrix = self.transform.inverse().matrix().fixed_view::<3, 3>(0, 0).transpose();
            RawIntersection {
                dist: (point - r.ro).norm(),
                point,
                normal: (normal_matrix * i.normal).normalize(),
            }
        })
    }
    fn bounds(&self) -> BBox {
        self.item.bounds().transform(&na::convert(self.transform))
    }

    fn surface_area(&self) -> f64 {
        self.uniform_scale.map_or(0., |s| self.item.surface_area() * s * s)
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        // Uniform scaling keeps samples uniform by area; otherwise the item isn't sampled.
        self.uniform_scale?;
        let (p, n) = self.item.sample_surface(u)?;
        let point = self.transform.transform_point(&Point3::from(p)).coords;
        let normal = (self.transform.transform_vector(&n)).normalize();
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
        let b = Box::new(BBox::new(Vector3::zeros(), Vector3::new(2., 1., 1.)));
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
    fn test_rotated_hit_is_in_world_space() {
        // A slab that is thin in x; rotating 90 degrees about y makes it thin in z.
        let slab = BBox::new(Vector3::new(-0.1, -1., -2.), Vector3::new(0.1, 1., 2.));
        let t = Transform::rotate(Box::new(slab), 0., FRAC_PI_2, 0.);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        let i = t.intersects(&r).unwrap();
        assert!((i.dist - 4.9).abs() < 1e-9, "dist {}", i.dist);
        assert!((i.point - Vector3::new(0., 0., -0.1)).norm() < 1e-9, "point {}", i.point);
        assert!((i.normal - Vector3::new(0., 0., -1.)).norm() < 1e-9, "normal {}", i.normal);
    }
}
