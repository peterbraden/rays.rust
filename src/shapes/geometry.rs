use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;
use crate::na::{Vector2, Vector3};
use crate::na::geometry::Affine3;


/// Most crossings `crossings` gathers, in case of numerical trouble.
pub const MAX_CROSSINGS: usize = 64;

/// How far past each hit, relative to its distance, the default `crossings` looks for the next.
pub const CROSSING_STEP: f64 = 1e-9;

// 
// - Roughly equivalent to the Shape class in PBRT
// - We don't include transforms in geometry, preferring to compose as:
//      Translate(X) is easier than adding x.translate to all geometries.
// - The simpler we can keep this interface, the easier it is to add new shapes.
// - CSG (see csg.rs) combines shapes using `crossings` and `inside`.
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

    /// The nearest hit in `[min, max)` along the ray. A composite can use the range to skip
    /// whatever lies outside it, eg. a mesh's BVH stops at nodes beyond `max`, the nearest hit
    /// found elsewhere in the scene so far.
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.intersects_beyond(r, min).filter(|x| x.dist >= min && x.dist < max)
    }

    /// Whether anything is hit in `[min, max)` along the ray: for shadow rays, which don't
    /// need the nearest hit, so a composite can stop at the first. PBR calls this
    /// `IntersectP`.
    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.intersects_range(r, min, max).is_some()
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

    /// Every point where the ray crosses the surface ahead of its origin, nearest first, with
    /// outward normals: what CSG needs to combine shapes. By default found by asking for the
    /// next hit beyond each one in turn; shapes that find all their hits at once should override
    /// this, which is faster and doesn't depend on stepping past each hit.
    fn crossings(&self, r: &Ray) -> Vec<RawIntersection> {
        let mut out: Vec<RawIntersection> = Vec::new();
        let mut from = 0.;
        while out.len() < MAX_CROSSINGS {
            match self.intersects_beyond(r, from) {
                Some(x) if out.last().is_none_or(|l| x.dist > l.dist) => {
                    from = x.dist + CROSSING_STEP * (1. + x.dist);
                    out.push(x);
                },
                _ => break,
            }
        }
        out
    }

    // Is the point inside the object? False for objects with no 'inside'
    // NB. We ignore 'ON' for points that are on the surface, as floating comparison
    // is problematic. 
    fn inside(&self, _: &Vector3<f64>) -> bool { false }

    // A triangle's corners, in the order its intersection test takes them; None for other
    // shapes. Containers of triangles use them to test several at once (see `triangle::may_hit`).
    fn vertices(&self) -> Option<[Vector3<f64>; 3]> { None }

    // Whether intersections are random, as for a participating medium, which samples where
    // light collides in it. Such an object must be tested at most once per ray - the nearest
    // of several samples would be biased towards nearer collisions - so the scene graph keeps
    // it out of its octree, which can list an object in several cells.
    fn is_stochastic(&self) -> bool { false }

    // Whether light can pass through this solid object (in part, by `transmittance_color`), as
    // through transparent water; the scene graph then asks it, rather than taking any hit as a
    // shadow.
    fn transmits(&self) -> bool { false }

    // The fraction of light that passes through this object between `min` and `max` along the
    // ray (world distances, like `intersects`' `dist`): 0 or 1 for solid objects - by default,
    // whether it is hit there - and in between for participating media. A medium may return a
    // random estimate, as long as its expectation is the true transmittance.
    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        if self.intersects_beyond(r, min).is_some_and(|x| x.dist >= min && x.dist < max) { 0. } else { 1. }
    }

    // The shape as flat primitives (triangles, spheres, planes), for the wavefront integrator
    // (`wavefront::flat`); None if it can't be.
    fn flatten(&self) -> Option<Vec<crate::wavefront::flat::Prim>> { None }

    // Whether a hit's `face` is which of `flatten`'s primitives was hit, as a mesh's is (its
    // triangle, for `uv`); otherwise the flat scene's hits report face 0, as the shape's do.
    fn faces_are_primitives(&self) -> bool { false }

    // The sea, if that is what this is: the GPU has a kernel for its shape (`ocean::Ocean::export`).
    fn ocean(&self) -> Option<&crate::ocean::Ocean> { None }

    // What kind of shape this is, for telling someone which of a scene's objects is in the way
    // of something (the wavefront integrator and the GPU, which need `flatten`): the type's
    // name, or for a wrapper its contents'.
    fn kind(&self) -> &'static str {
        // (Without its path, or what a generic one is of: `Octree<Triangle>` is an `Octree`.)
        let name = std::any::type_name::<Self>();
        let name = name.split('<').next().unwrap_or(name);
        name.rsplit("::").next().unwrap_or(name)
    }

    // The transmittance per colour channel (or wavelength, in spectral mode): the same in all
    // but for chromatic media (`volume::chromatic`).
    fn transmittance_color(&self, r: &Ray, min: f64, max: f64) -> crate::color::Color {
        crate::color::Color::white() * self.transmittance(r, min, max)
    }

    // Surface area, for shapes that can be sampled as area lights; 0 for those that can't.
    fn surface_area(&self) -> f64 { 0. }

    // A point uniformly distributed over the surface by area, and the surface normal there,
    // from three uniform random numbers in [0, 1). None for shapes that can't be sampled.
    fn sample_surface(&self, _u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { None }

    /// Texture coordinates (u, v) of the point `p` on the surface, hit as part `face` (see
    /// `RawIntersection::face`): the surface's own parameterisation, as pbrt's shapes give
    /// theirs (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., chapter 6),
    /// for textures (ch. 10). Computed on demand, when a material asks, rather than for every
    /// hit. None for shapes without one.
    fn uv(&self, _p: &Vector3<f64>, _face: u32) -> Option<Vector2<f64>> { None }

    /// The accuracy of this shape's hits, in its own units, beyond floating-point rounding: the
    /// tolerance of an iterative solver (an isosurface's `accuracy`), or 0 for shapes solved in
    /// closed form. See `Intersection::spawn_offset`.
    fn tolerance(&self) -> f64 { 0. }

    /// The direction the surface runs at `p` (hit as part `face`), for materials that depend on
    /// it: a curve's direction, for hair. None for shapes without one.
    fn tangent(&self, _p: &Vector3<f64>, _face: u32) -> Option<Vector3<f64>> { None }

    /// A point on the surface for light sampling from `from`, its normal, and the density with
    /// which it was chosen per unit solid angle at `from` (PBR section 6.1.4, `Shape::Sample`
    /// with a reference point). By default a point uniform by area (`sample_surface`), whose
    /// density converts to solid angle by d² / (|cos θ| A); shapes that can do better sample
    /// only what `from` can see (a sphere, the cone it subtends).
    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let (y, n) = self.sample_surface(u)?;
        let pdf = area_pdf(from, &y, &n, self.surface_area());
        (pdf > 0. && pdf.is_finite()).then_some((y, n, pdf))
    }

    /// The density per unit solid angle with which `sample_from(from, ..)` chooses the point `y`
    /// with normal `n`.
    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        area_pdf(from, y, n, self.surface_area())
    }
}

/// The density per solid angle at `from` of choosing `y` (normal `n`) uniformly by area on a
/// surface of area `area`: d² / (|cos θ_y| A).
pub fn area_pdf(from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>, area: f64) -> f64 {
    let w = y - from;
    let d2 = w.norm_squared();
    let cos = n.dot(&w).abs() / d2.sqrt();
    if cos <= 0. || area <= 0. { 0. } else { d2 / (cos * area) }
}


/// A shared geometry is a geometry: an object can hold a geometry that other objects, or
/// instancing `Transform`s, also use, without copying it.
impl<G: Geometry + Send + ?Sized> Geometry for std::sync::Arc<G> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> { (**self).intersects(r) }
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> { (**self).intersects_beyond(r, min) }
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> { (**self).intersects_range(r, min, max) }
    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool { (**self).occludes(r, min, max) }
    fn bounds(&self) -> BBox { (**self).bounds() }
    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox { (**self).transformed_bounds(t) }
    fn fast_intersects(&self, r: &Ray) -> bool { (**self).fast_intersects(r) }
    fn primitives(&self) -> u64 { (**self).primitives() }
    fn inside(&self, p: &Vector3<f64>) -> bool { (**self).inside(p) }
    fn surface_area(&self) -> f64 { (**self).surface_area() }
    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> { (**self).sample_surface(u) }
    fn uv(&self, p: &Vector3<f64>, face: u32) -> Option<Vector2<f64>> { (**self).uv(p, face) }
    fn tolerance(&self) -> f64 { (**self).tolerance() }
    fn tangent(&self, p: &Vector3<f64>, face: u32) -> Option<Vector3<f64>> { (**self).tangent(p, face) }
    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> { (**self).sample_from(from, u) }
    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 { (**self).pdf_from(from, y, n) }
    fn is_stochastic(&self) -> bool { (**self).is_stochastic() }
    fn transmits(&self) -> bool { (**self).transmits() }
    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 { (**self).transmittance(r, min, max) }
    fn transmittance_color(&self, r: &Ray, min: f64, max: f64) -> crate::color::Color { (**self).transmittance_color(r, min, max) }
    fn flatten(&self) -> Option<Vec<crate::wavefront::flat::Prim>> { (**self).flatten() }
    fn kind(&self) -> &'static str { (**self).kind() }
    fn ocean(&self) -> Option<&crate::ocean::Ocean> { (**self).ocean() }
    fn faces_are_primitives(&self) -> bool { (**self).faces_are_primitives() }
}

/// A geometry whose hits report `face`, so a composite object made of many of them (eg. the
/// triangles of a tree's leaves) can say which part was hit; see `RawIntersection::face`.
pub struct Tagged<T: Geometry> {
    pub item: T,
    pub face: u32,
}

impl<T: Geometry> Geometry for Tagged<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(r).map(|i| RawIntersection { face: self.face, ..i })
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.item.intersects_beyond(r, min).map(|i| RawIntersection { face: self.face, ..i })
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.item.intersects_range(r, min, max).map(|i| RawIntersection { face: self.face, ..i })
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.item.occludes(r, min, max)
    }

    fn transformed_bounds(&self, t: &Affine3<f64>) -> BBox {
        self.item.transformed_bounds(t)
    }

    fn bounds(&self) -> BBox {
        self.item.bounds()
    }

    fn surface_area(&self) -> f64 {
        self.item.surface_area()
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        self.item.sample_surface(u)
    }

    fn uv(&self, p: &Vector3<f64>, _face: u32) -> Option<Vector2<f64>> {
        self.item.uv(p, 0)
    }

    fn tolerance(&self) -> f64 {
        self.item.tolerance()
    }

    fn tangent(&self, p: &Vector3<f64>, _face: u32) -> Option<Vector3<f64>> {
        self.item.tangent(p, 0)
    }

    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        self.item.sample_from(from, u)
    }

    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        self.item.pdf_from(from, y, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::triangle::Triangle;
    use crate::shapes::geometry_set::GeometrySet;
    use std::sync::Arc;

    #[test]
    fn test_tagged_hits_report_their_face() {
        // Two parallel triangles, the nearer tagged 7 and the farther 3, in a set.
        let at = |z: f64, face: u32| Arc::new(Tagged {
            item: Triangle::new(Vector3::new(-1., -1., z), Vector3::new(1., -1., z), Vector3::new(0., 1., z)),
            face,
        });
        let items = vec![at(2., 3), at(1., 7)];
        let tree = GeometrySet::new(&items);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 1.) };
        assert_eq!(tree.intersects(&r).unwrap().face, 7);
        let back = Ray { ro: Vector3::new(0., 0., 5.), rd: Vector3::new(0., 0., -1.) };
        assert_eq!(tree.intersects(&back).unwrap().face, 3);
    }
}
