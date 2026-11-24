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
#[derive(Clone)]
pub struct Transform {
    pub item: Arc<dyn Geometry + Sync + Send>,
    pub transform: Affine3<f64>,
    inverse: Affine3<f64>,
    normal_matrix: Matrix3<f64>,
    /// The scale factor if the transformation scales equally in every direction (a rotation,
    /// reflection and translation besides), which keeps area uniform, so the transformed surface
    /// can still be sampled as an area light.
    uniform_scale: Option<f64>,
    /// Otherwise the transformed surface's area, estimated when first asked for (`stretched_area`).
    area: std::sync::OnceLock<f64>,
    /// `inverse` without its scaling (`rigid`): where patterns are read.
    pattern_frame: Affine3<f64>,
}

/// An affine map's rotation and translation without its scaling: the rotation nearest its
/// linear part (the polar decomposition's, U Vᵀ of its singular value decomposition), about
/// the point it takes to the origin. So patterns read through it turn and move with their
/// object but keep their size in the scene's units.
fn rigid(m: &Affine3<f64>) -> Affine3<f64> {
    let linear: Matrix3<f64> = m.matrix().fixed_view::<3, 3>(0, 0).into();
    let svd = linear.svd(true, true);
    let rotation = match (svd.u, svd.v_t) {
        (Some(u), Some(v_t)) => u * v_t,
        _ => Matrix3::identity(),
    };
    let origin = m.try_inverse().map_or(Vector3::zeros(), |i| i.transform_point(&Point3::origin()).coords);
    let mut h = Matrix4::identity();
    h.fixed_view_mut::<3, 3>(0, 0).copy_from(&rotation);
    h.fixed_view_mut::<3, 1>(0, 3).copy_from(&(-(rotation * origin)));
    Affine3::from_matrix_unchecked(h)
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
        let pattern_frame = rigid(&inverse);
        Transform { item: item.into(), transform, inverse, normal_matrix, uniform_scale, area: std::sync::OnceLock::new(), pattern_frame }
    }

    /// How much the transformation stretches the item's area at a point with normal `n` (unit,
    /// in the item's space): |det L| |L^-T n| (PBR 4th ed., section 3.10.3, for normals; Nanson's
    /// formula for areas).
    fn stretch(&self, n: &Vector3<f64>) -> f64 {
        self.linear().determinant().abs() * (self.normal_matrix * n).norm()
    }

    /// The most `stretch` can be, for any normal: |det L| / σ_min(L), the largest singular value
    /// of L^-T.
    fn most_stretch(&self) -> f64 {
        let singular = self.normal_matrix.singular_values();
        self.linear().determinant().abs() * singular.max()
    }

    fn linear(&self) -> Matrix3<f64> {
        self.transform.matrix().fixed_view::<3, 3>(0, 0).into()
    }

    /// The area of the item stretched by a transformation that doesn't scale evenly: its own area
    /// times its mean stretch, over 64 × 64 points spread evenly by area (stratified, so the
    /// estimate of a smooth surface's is good to well under a part in a thousand).
    fn stretched_area(&self) -> f64 {
        *self.area.get_or_init(|| {
            let area = self.item.surface_area();
            if area <= 0. {
                return 0.;
            }
            let n = 64;
            let mut total = 0.;
            let mut count = 0;
            for i in 0..n {
                for j in 0..n {
                    let u = [(i as f64 + 0.5) / n as f64, (j as f64 + 0.5) / n as f64, 0.5];
                    if let Some((_, normal)) = self.item.sample_surface(u) {
                        total += self.stretch(&normal);
                        count += 1;
                    }
                }
            }
            if count == 0 { 0. } else { area * total / count as f64 }
        })
    }

    /// A density per solid angle at `local_from` of choosing the point `p` (normal `n`) of the
    /// item, both in the item's space, as one per solid angle at the transformed point: through
    /// densities by area, which the transformation divides by its stretch there.
    fn transformed_pdf(&self, local_from: &Vector3<f64>, p: &Vector3<f64>, n: &Vector3<f64>, pdf: f64) -> f64 {
        let to = p - local_from;
        let d2 = to.norm_squared();
        let cos = n.dot(&to).abs() / d2.sqrt();
        if !(cos > 0. && d2 > 0.) {
            return 0.;
        }
        let by_area = pdf * cos / d2 / self.stretch(n);
        let from = self.transform.transform_point(&Point3::from(*local_from)).coords;
        let y = self.transform.transform_point(&Point3::from(*p)).coords;
        let normal = (self.normal_matrix * n).normalize();
        crate::shapes::geometry::area_pdf(&from, &y, &normal, 1. / by_area)
    }

    pub fn rotate(
        item: impl Into<Arc<dyn Geometry + Sync + Send>>,
        roll: f64, pitch: f64, yaw: f64
    ) -> Transform {
        Transform::from_parts(item, Vector3::zeros(), Rotation3::from_euler_angles(roll, pitch, yaw), Vector3::new(1., 1., 1.))
    }

    /// Scale by `scale` (per axis), then rotate, then translate by `translate`. `item` may be a
    /// `Box`, for a geometry used once, or an `Arc` shared with other instances.
    pub fn from_parts(
        item: impl Into<Arc<dyn Geometry + Sync + Send>>,
        translate: Vector3<f64>,
        rotation: Rotation3<f64>,
        scale: Vector3<f64>,
    ) -> Transform {
        assert!(scale.x != 0. && scale.y != 0. && scale.z != 0., "transform scale must be non-zero on every axis, got {}", scale);
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

    fn to_world(&self, r: &Ray, i: RawIntersection) -> RawIntersection {
        let point = self.transform.transform_point(&Point3::from(i.point)).coords;
        RawIntersection {
            face: i.face,
            part: i.part,
            dist: (point - r.ro).norm(),
            point,
            normal: (self.normal_matrix * i.normal).normalize(),
        }
    }
}

/// Coordinates beyond this are taken to mean the bounds are unlimited (eg. a plane's).
const UNBOUNDED: f64 = 1e300;

impl Geometry for Transform {
    fn kind(&self) -> &'static str {
        self.item.kind()
    }

    fn faces_are_primitives(&self) -> bool {
        self.item.faces_are_primitives()
    }

    fn flatten(&self) -> Option<Vec<crate::wavefront::flat::Prim>> {
        self.item.flatten()?.iter().map(|p| p.transformed(&self.transform)).collect()
    }

    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // Intersect in object space, then map the hit back into world space.
        self.item.intersects(&self.to_object(r)).map(|i| self.to_world(r, i))
    }

    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        self.item.crossings(&self.to_object(r)).into_iter().map(|i| self.to_world(r, i)).collect()
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

    fn bounds(&self) -> BBox {
        self.transformed_bounds(&Affine3::identity())
    }

    /// Unlimited if the item's bounds are, since turning or shearing an unlimited box spreads it
    /// along other axes too (and the arithmetic overflows).
    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        let b = self.item.bounds();
        if b.min.coords.iter().chain(b.max.coords.iter()).any(|c| c.abs() >= UNBOUNDED) {
            return BBox::new(Vector3::repeat(f64::MIN), Vector3::repeat(f64::MAX));
        }
        self.item.transformed_bounds(&(t * self.transform))
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.item.inside(&self.inverse.transform_point(&Point3::from(*p)).coords)
    }

    fn uv(&self, p: &Vector3<f64>, face: u32) -> Option<crate::na::Vector2<f64>> {
        self.item.uv(&self.inverse.transform_point(&Point3::from(*p)).coords, face)
    }

    fn undisplaced(&self, p: &Vector3<f64>, face: u32) -> Option<crate::shapes::geometry::Undisplaced> {
        let u = self.item.undisplaced(&self.inverse.transform_point(&Point3::from(*p)).coords, face)?;
        Some(crate::shapes::geometry::Undisplaced {
            point: self.transform.transform_point(&Point3::from(u.point)).coords,
            normal: (self.normal_matrix * u.normal).normalize(),
            filter: u.filter * self.uniform_scale.unwrap_or(1.),
        })
    }

    fn pattern_frame(&self, p: &Vector3<f64>, face: u32) -> Option<Affine3<f64>> {
        let local = self.inverse.transform_point(&Point3::from(*p)).coords;
        Some(match self.item.pattern_frame(&local, face) {
            None => self.pattern_frame,
            Some(f) => rigid(&(f * self.inverse)),
        })
    }

    fn tangent(&self, p: &Vector3<f64>, face: u32) -> Option<Vector3<f64>> {
        let t = self.item.tangent(&self.inverse.transform_point(&Point3::from(*p)).coords, face)?;
        Some(self.transform.transform_vector(&t).normalize())
    }

    /// The item's tolerance, stretched by at most the transform's largest scale.
    fn tolerance(&self) -> f64 {
        let t = self.item.tolerance();
        if t == 0. { return 0.; }
        let linear: Matrix3<f64> = self.transform.matrix().fixed_view::<3, 3>(0, 0).into();
        t * linear.norm()
    }

    fn is_stochastic(&self) -> bool {
        self.item.is_stochastic()
    }

    fn surface_area(&self) -> f64 {
        match self.uniform_scale {
            Some(s) => self.item.surface_area() * s * s,
            None => self.stretched_area(),
        }
    }

    /// A similarity (uniform scale, rotation, translation) keeps angles, so the item's own
    /// sampling from the point in its space has the same density per solid angle. Other
    /// transformations keep which points `from` sees (lines stay lines) but not angles or areas:
    /// the item's sampling from the point in its space, its density converted
    /// (`transformed_pdf`).
    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let local = self.inverse.transform_point(&Point3::from(*from)).coords;
        let (p, n, pdf) = self.item.sample_from(&local, u)?;
        let pdf = match self.uniform_scale {
            Some(_) => pdf,
            None => self.transformed_pdf(&local, &p, &n, pdf),
        };
        (pdf > 0. && pdf.is_finite()).then(|| (self.transform.transform_point(&Point3::from(p)).coords, (self.normal_matrix * n).normalize(), pdf))
    }

    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        let local = |v: &Vector3<f64>| self.inverse.transform_point(&Point3::from(*v)).coords;
        // Normals map back by the transpose of the linear part (the inverse of `normal_matrix`).
        let ln = (self.linear().transpose() * n).normalize();
        let (from, y) = (local(from), local(y));
        let pdf = self.item.pdf_from(&from, &y, &ln);
        match self.uniform_scale {
            Some(_) => pdf,
            None if pdf > 0. => self.transformed_pdf(&from, &y, &ln, pdf),
            None => 0.,
        }
    }

    /// Uniform scaling keeps samples uniform by area. Other transformations stretch some parts
    /// more than others: a point uniform over the item, kept with the chance its stretch is of
    /// the most there can be, or another drawn (with the renderer's own numbers past the first;
    /// none after 64, as unlikely as the shape is stretched).
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let map = |(p, n): (Vector3<f64>, Vector3<f64>)| (self.transform.transform_point(&Point3::from(p)).coords, (self.normal_matrix * n).normalize());
        if self.uniform_scale.is_some() {
            return self.item.sample_surface(u).map(map);
        }
        let most = self.most_stretch();
        let mut u = u;
        for _ in 0..64 {
            let (p, n) = self.item.sample_surface(u)?;
            if crate::geometry::rand() * most <= self.stretch(&n) {
                return Some(map((p, n)));
            }
            u = [crate::geometry::rand(), crate::geometry::rand(), crate::geometry::rand()];
        }
        None
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
    fn test_primitives_are_the_items() {
        // A stand-in for a mesh: many primitives inside one geometry.
        struct Many;
        impl Geometry for Many {
            fn intersects(&self, _r: &Ray) -> Option<crate::intersection::RawIntersection> { None }
            fn bounds(&self) -> BBox { BBox::new(Vector3::zeros(), Vector3::new(1., 1., 1.)) }
            fn primitives(&self) -> u64 { 871414 }
        }
        let t = Transform::from_parts(std::sync::Arc::new(Many) as std::sync::Arc<dyn Geometry + Sync + Send>, Vector3::zeros(), Rotation3::identity(), Vector3::new(2., 2., 2.));
        assert_eq!(t.primitives(), 871414);
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
    }

    /// The ellipsoid's area, x²/a² + y²/b² + z²/c² = 1, by Knud Thomsen's formula (good to about
    /// a percent).
    fn ellipsoid_area(a: f64, b: f64, c: f64) -> f64 {
        let p = 1.6075;
        4. * std::f64::consts::PI * (((a * b).powf(p) + (a * c).powf(p) + (b * c).powf(p)) / 3.).powf(1. / p)
    }

    #[test]
    fn test_stretched_emitters_are_sampled() {
        // A sphere stretched into an ellipsoid, turned and moved.
        crate::geometry::seed_rand(3, 0);
        let (a, b, c) = (0.5, 1.3, 0.8);
        let t = Transform::from_parts(unit_sphere(), Vector3::new(1., 2., 3.), Rotation3::from_euler_angles(0.3, 0.2, 0.1), Vector3::new(a, b, c));
        let area = t.surface_area();
        assert!((area / ellipsoid_area(a, b, c) - 1.).abs() < 0.012, "{} vs {}", area, ellipsoid_area(a, b, c));
        // Samples lie on it, and agree with pdf_from.
        let from = Vector3::new(4., 0., -1.);
        let inside = |p: &Vector3<f64>| t.inverse.transform_point(&Point3::from(*p)).coords.norm();
        for k in 0..200 {
            let u = [(k as f64 * 0.618) % 1., (k as f64 * 0.414) % 1., (k as f64 * 0.732) % 1.];
            let (y, n, pdf) = t.sample_from(&from, u).unwrap();
            assert!((inside(&y) - 1.).abs() < 1e-9);
            assert!((t.pdf_from(&from, &y, &n) / pdf - 1.).abs() < 1e-9);
            let (s, _) = t.sample_surface(u).unwrap();
            assert!((inside(&s) - 1.).abs() < 1e-9);
        }
        // The density integrates to one over the solid angle the ellipsoid subtends: the
        // estimate of that solid angle, 1/pdf on average, matches the fraction of directions
        // from `from` that hit it.
        let n = 200_000;
        let by_pdf: f64 = (0..n).map(|_| {
            let (_, _, pdf) = t.sample_from(&from, [crate::geometry::rand(), crate::geometry::rand(), crate::geometry::rand()]).unwrap();
            1. / pdf
        }).sum::<f64>() / n as f64;
        let hits = (0..n).filter(|_| {
            let z = 1. - 2. * crate::geometry::rand();
            let phi = 2. * std::f64::consts::PI * crate::geometry::rand();
            let r = (1. - z * z).sqrt();
            t.intersects(&Ray { ro: from, rd: Vector3::new(r * phi.cos(), r * phi.sin(), z) }).is_some()
        }).count();
        let by_hits = 4. * std::f64::consts::PI * hits as f64 / n as f64;
        assert!((by_pdf / by_hits - 1.).abs() < 0.02, "{} vs {}", by_pdf, by_hits);
        // Points by sample_surface are uniform by area: the share in the slab |y'| < 0.3 of
        // the ellipsoid's own height (in its space) matches its share of the area, found by
        // the stretch of a stratified grid.
        let slab = |p: &Vector3<f64>| t.inverse.transform_point(&Point3::from(*p)).coords.y.abs() < 0.3;
        let sampled = (0..n).filter(|_| slab(&t.sample_surface([crate::geometry::rand(), crate::geometry::rand(), crate::geometry::rand()]).unwrap().0)).count() as f64 / n as f64;
        let m = 400;
        let (mut inside_slab, mut total) = (0., 0.);
        for i in 0..m {
            for j in 0..m {
                let (p, normal) = t.item.sample_surface([(i as f64 + 0.5) / m as f64, (j as f64 + 0.5) / m as f64, 0.5]).unwrap();
                let w = t.stretch(&normal);
                total += w;
                if p.y.abs() < 0.3 { inside_slab += w; }
            }
        }
        assert!((sampled - inside_slab / total).abs() < 0.006, "{} vs {}", sampled, inside_slab / total);
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
        // A shear is not uniform: its area is estimated (`stretched_area`), and more than the
        // sphere's, as the shear stretches it (a shear keeps volume).
        let shear = Matrix4::new(1., 0.5, 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
        let t = Transform::new(unit_sphere(), Affine3::from_matrix_unchecked(shear));
        assert!(t.uniform_scale.is_none() && t.surface_area() > 4. * std::f64::consts::PI);
    }

    #[test]
    fn test_sheared_normals_stay_perpendicular() {
        // x += 0.5 y turns the unit cube into a slanted box; its slanted face's normal is
        // perpendicular to the slant.
        let shear = Matrix4::new(1., 0.5, 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
        let b: std::sync::Arc<dyn Geometry + Sync + Send> = std::sync::Arc::new(BBox::new(Vector3::zeros(), Vector3::repeat(1.)));
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
        Transform::new(unit_sphere(), Affine3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(1., 0., 1.))));
    }

    #[test]
    fn test_unbounded_items_stay_unbounded() {
        let plane: std::sync::Arc<dyn Geometry + Sync + Send> = std::sync::Arc::new(crate::shapes::plane::HalfSpace::new(Vector3::new(0., 1., 0.), 0.));
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
    #[should_panic(expected = "non-zero")]
    fn test_zero_scale_is_rejected() {
        Transform::from_parts(unit_sphere(), Vector3::zeros(), Rotation3::identity(), Vector3::new(1., 0., 1.));
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
