use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::na::{Vector3};
use crate::na::geometry::Affine3;


// 
// - Roughly equivalent to the Shape class in PBRT
// - We don't include transforms in geometry, preferring to compose as:
//      Translate(X) is easier than adding x.translate to all geometries.
// - The simpler we can keep this interface, the easier it is to add new shapes.
// - For extensions to allow CSG, see 'Primitive' which is a wrapper object.
//
// - All shapes must allow intersection (PBRT has a weird canIntersect thing)
pub trait Geometry: Sync{
    fn intersects(&self, r: &Ray) -> Option<RawIntersection>;

    // Nearest intersection at least `min` along the ray. A composite (eg. a mesh) reports only
    // its nearest hit, so if that is nearer than `min` - typically the surface a shadow ray
    // leaves from - search again from `min` rather than discarding the whole object.
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        match self.intersects(r) {
            Some(x) if x.dist < min => {
                let advanced = Ray { ro: r.ro + r.rd.normalize() * min, rd: r.rd };
                self.intersects(&advanced).map(|mut x| { x.dist += min; x })
            },
            x => x,
        }
    }

    // World space bounding box
    fn bounds(&self) -> BBox;

    /// The bounds of this geometry after transforming it by `t`, as when it is instanced. The
    /// default transforms the corners of `bounds()`, which under rotation can be much larger
    /// than the geometry itself (up to sqrt(3) times as wide); composites can do better.
    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        self.bounds().transform(&na::convert(*t))
    }


    // === Optional methods ===
    // - We provide defaults to keep implementation burden low.
    
    // Equivalent of intersectsP in PBRT. If there is a fast way
    // to calculate intersection, override, otherwise fall back to intersects() call.
    // - NB. Callers should be aware that calling fast_intersects(), then intersects() is slower
    // than just calling intersects. This is only really for cases where you want to avoid
    // allocation of a RawIntersection object.
    fn fast_intersects(&self, r: &Ray) -> bool {
        self.intersects(r).is_some()
    }

    // Number of primitives inside geometry (ie triangles in mesh, circles etc.)
    // Used for statistics on composite geometries.
    fn primitives(&self) -> u64 { 1 }

    // Is the point inside the object? False for objects with no 'inside'
    // NB. We ignore 'ON' for points that are on the surface, as floating comparison
    // is problematic. 
    fn inside(&self, _: &Vector3<f64>) -> bool { false }

    // Surface area, for shapes that can be sampled as area lights; 0 for those that can't.
    fn surface_area(&self) -> f64 { 0. }

    // A point uniformly distributed over the surface by area, and the surface normal there,
    // from three uniform random numbers in [0, 1). None for shapes that can't be sampled.
    fn sample_surface(&self, _u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { None }
}


/// A shared geometry is a geometry: an object can hold a geometry that other objects, or
/// instancing `Transform`s, also use, without copying it.
impl<G: Geometry + Send + ?Sized> Geometry for std::sync::Arc<G> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> { (**self).intersects(r) }
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> { (**self).intersects_beyond(r, min) }
    fn bounds(&self) -> BBox { (**self).bounds() }
    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox { (**self).transformed_bounds(t) }
    fn fast_intersects(&self, r: &Ray) -> bool { (**self).fast_intersects(r) }
    fn primitives(&self) -> u64 { (**self).primitives() }
    fn inside(&self, p: &Vector3<f64>) -> bool { (**self).inside(p) }
    fn surface_area(&self) -> f64 { (**self).surface_area() }
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { (**self).sample_surface(u) }
}
