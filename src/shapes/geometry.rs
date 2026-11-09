use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::na::{Vector3};


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

    // Whether intersections are random, as for a participating medium, which samples where
    // light collides in it. Such an object must be tested at most once per ray - the nearest
    // of several samples would be biased towards nearer collisions - so the scene graph keeps
    // it out of its octree, which can list an object in several cells.
    fn is_stochastic(&self) -> bool { false }

    // The fraction of light that passes through this object between `min` and `max` along the
    // ray (world distances, like `intersects`' `dist`): 0 or 1 for solid objects - by default,
    // whether it is hit there - and in between for participating media. A medium may return a
    // random estimate, as long as its expectation is the true transmittance.
    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        if self.intersects_beyond(r, min).is_some_and(|x| x.dist >= min && x.dist < max) { 0. } else { 1. }
    }

    // Surface area, for shapes that can be sampled as area lights; 0 for those that can't.
    fn surface_area(&self) -> f64 { 0. }

    // A point uniformly distributed over the surface by area, and the surface normal there,
    // from three uniform random numbers in [0, 1). None for shapes that can't be sampled.
    fn sample_surface(&self, _u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { None }
}

