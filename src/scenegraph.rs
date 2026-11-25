use crate::color::Color;
use crate::ray::Ray;
use crate::na::{Vector3};
use crate::intersection::{Intersection, RawIntersection};
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::bvh::Bvh;
use std::sync::Arc;
use std::fmt;
use crate::shapes::geometry::Geometry;

/// The scene's objects, for finding what a ray hits. Bounded objects are in a bounding volume
/// hierarchy (see `crate::bvh`); unbounded ones, such as planes and the sky, can't be, and are
/// tested against every ray.
///
/// With instancing this is the top level of a two-level hierarchy: the scene's BVH holds
/// instances, each of which transforms the ray and searches its own geometry's BVH (PBR 4th ed.,
/// section 7.1.4).
pub struct SceneGraph {
    /// The bounded objects, in the BVH's leaf order.
    pub items: Vec<Arc<SceneObject>>,
    pub infinite_items: Vec<Arc<SceneObject>>,
    bvh: Bvh,
}

/// Objects any larger than this along an axis are treated as unbounded: their boxes would
/// overflow the surface area heuristic.
const UNBOUNDED: f64 = 1e30;

fn is_unbounded(b: &BBox) -> bool {
    !b.size().iter().all(|d| d.abs() < UNBOUNDED)
}

impl SceneGraph {

    pub fn new(objects: Vec<Arc<SceneObject>>) -> SceneGraph {
        // Unbounded objects can't go in the BVH. Nor can participating media: they must be
        // tested once per ray (see `Geometry::is_stochastic`). Nor objects that let light
        // through (`Geometry::transmits`), which the BVH's shadow test would take as blocking.
        let (infinite_items, bounded): (Vec<_>, Vec<_>) = objects.into_iter()
            .partition(|x| x.geometry.is_stochastic() || x.geometry.transmits() || is_unbounded(&x.geometry.bounds()));
        let (bvh, items) = Bvh::build(bounded, |x| x.geometry.bounds());
        SceneGraph { items, infinite_items, bvh }
    }

    pub fn items(&self) -> &Vec<Arc<SceneObject>>{
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn primitives_len(&self) -> u64 {
        self.items
            .iter()
            .map(|x| x.geometry.primitives())
            .sum::<u64>()
    }

    pub fn nearest_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<Intersection<'_>> {
        self.nearest_raw_intersection(r, max, min).map(|(object, hit)| Intersection {
            dist: hit.dist,
            point: hit.point,
            normal: hit.normal,
            face: hit.face,
            object,
            part: hit.part,
        })
    }

    /// Whether anything is hit between `min` and `max` along the ray, stopping at the first hit
    /// found: for shadow rays.
    pub fn occluded(&self, r: &Ray, max: f64, min: f64) -> bool {
        self.bvh.any_hit(r, min, max, |i| self.items[i].occludes(r, min, max))
            || self.infinite_items.iter().any(|o| o.occludes(r, min, max))
    }

    /// The fraction of light that gets along the ray between `min` and `max` (world distances):
    /// 0 if anything solid is in the way, otherwise the product of the transmittances of the
    /// participating media it crosses (`Geometry::transmittance`), which may be a random
    /// estimate. For shadow rays, in place of `occluded` wherever media should cast soft,
    /// partial shadows rather than all-or-nothing ones.
    ///
    /// Per colour channel (or wavelength), for chromatic media (`volume::chromatic`).
    pub fn transmittance(&self, r: &Ray, max: f64, min: f64) -> Color {
        if self.bvh.any_hit(r, min, max, |i| self.items[i].occludes(r, min, max)) {
            return Color::black();
        }
        // Solid objects first: if one blocks the ray, the media needn't be tracked. (Most block
        // it or don't; transparent water lets some through, by colour.)
        // (Two passes over the few unbounded items rather than partitioning them into vectors,
        // which allocated on every shadow ray.)
        let mut t = Color::white();
        for o in self.infinite_items.iter().filter(|o| !o.is_stochastic()) {
            t = t * o.transmittance_color(r, min, max);
            if t.rgb.max() <= 0. {
                return Color::black();
            }
        }
        for o in self.infinite_items.iter().filter(|o| o.is_stochastic()) {
            t = t * o.transmittance_color(r, min, max);
            if t.rgb.max() <= 0. {
                break;
            }
        }
        t
    }

    /// The transmittance of the participating media alone along the ray between `min` and
    /// `max`, where it is known that no surface is in the way.
    pub fn media_transmittance(&self, r: &Ray, max: f64, min: f64) -> Color {
        let mut t = Color::white();
        for o in self.infinite_items.iter().filter(|o| o.is_stochastic()) {
            t = t * o.transmittance_color(r, min, max);
            if t.rgb.max() <= 0. {
                break;
            }
        }
        t
    }

    /// The distance along the ray to the nearest surface beyond `min`, passing through
    /// participating media (infinite where there is none).
    pub fn nearest_surface(&self, r: &Ray, min: f64) -> f64 {
        let mut best = f64::INFINITY;
        for o in self.infinite_items.iter().filter(|o| !o.is_stochastic()) {
            if let Some(x) = o.intersects_beyond(r, min).filter(|x| x.dist >= min && x.dist < best) {
                best = x.dist;
            }
        }
        match self.bvh.nearest(r, min, best, |i, b| self.items[i].intersects_range(r, min, b)) {
            Some((_, x)) => best.min(x.dist),
            None => best,
        }
    }

    pub fn nearest_raw_intersection(&self, r: &Ray, max:f64, min:f64) -> Option<(&SceneObject, RawIntersection)> {
        crate::volume::chromatic::begin_query();
        let infinites = self.naive_intersection_infinites(r, max, min);
        // Anything in the BVH must be no farther than the nearest unbounded object to matter
        // (on a tie, the bounded object wins).
        let max = infinites.as_ref().map_or(max, |(_, x)| x.dist.next_up());
        let bounded = self.bvh.nearest(r, min, max, |i, best| self.items[i].intersects_range(r, min, best))
            .map(|(i, x)| (&*self.items[i], x));
        bounded.or(infinites)
    }

    pub fn naive_intersection_infinites(&self, r: &Ray, max:f64, min:f64) -> Option<(&SceneObject, RawIntersection)> {
        let mut cdist = max;
        let mut closest = None;
        
        for o in &self.infinite_items {
            /*
            match exclude {
                Some(x) => {
                    if &*x as *const _  == &**o {
                        continue;
                    }
                }
                None => (),
            }
            */
            if let Some(x) = o.intersects_beyond(r, min) {
                if x.dist < cdist && x.dist >= min {
                    cdist = x.dist;
                    closest = Some((&**o, x));
                }
            }
        }
        closest
    }
}

impl fmt::Display for SceneGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| SceneGraph<bvh> \n| objects: {} \n| bounded: {} \n| infinite objs. {}", //\n{}\n",
                &self.items.len(), 
                &self.bvh.bounds(),
                &self.infinite_items.len(), 
 //               &self.root.as_ref().unwrap()
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::lambertian::Lambertian;
    use crate::material::texture::Solid;
    use crate::shapes::plane::Plane;
    use crate::shapes::sphere::Sphere;
    use crate::color::Color;
    use rand::{RngExt, SeedableRng};
    use rand::rngs::StdRng;

    fn object(g: Box<dyn Geometry + Sync + Send>) -> Arc<SceneObject> {
        Arc::new(SceneObject { geometry: g, medium: Box::new(Solid { m: Box::new(Lambertian { albedo: Color::white().into() }) }) })
    }

    #[test]
    fn test_matches_brute_force_with_unbounded_objects() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut objects: Vec<Arc<SceneObject>> = (0 .. 200).map(|_| object(Box::new(Sphere::new(
            Vector3::new(rng.random_range(-20.0..20.), rng.random_range(-1.0..6.), rng.random_range(-20.0..20.)),
            rng.random_range(0.1..2.),
        )))).collect();
        objects.push(object(Box::new(Plane { y: 0. })));
        let all = objects.clone();
        let g = SceneGraph::new(objects);
        assert_eq!((g.items.len(), g.infinite_items.len()), (200, 1));

        for _ in 0 .. 3000 {
            let r = Ray {
                ro: Vector3::new(rng.random_range(-25.0..25.), rng.random_range(-3.0..10.), rng.random_range(-25.0..25.)),
                rd: Vector3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.)),
            };
            let (min, max) = (rng.random_range(0.0..1.), rng.random_range(1.0..80.));
            let expected = all.iter().filter_map(|o| o.intersects_beyond(&r, min))
                .map(|x| x.dist).filter(|&d| d >= min && d < max)
                .min_by(|a, b| a.total_cmp(b));
            assert_eq!(g.nearest_raw_intersection(&r, max, min).map(|(_, x)| x.dist), expected, "{}", r);
            assert_eq!(g.occluded(&r, max, min), expected.is_some(), "{}", r);
        }
    }
}
