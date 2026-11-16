//! Choosing a light to sample from many: pbrt's light BVH (Pharr, Jakob and Humphreys,
//! "Physically Based Rendering", 4th ed., section 12.6.3, after Conty Estevez and Kulla,
//! "Importance Sampling of Many Lights with Adaptive Tree Splitting", HPG 2018).
//!
//! Sampling lights in proportion to their power (`SceneFile::light_powers`) ignores where the
//! shading point is: in a city of ten thousand street lamps, nearly every sample goes to a lamp
//! too far away to matter. The light BVH groups lights by position, and bounds each group's
//! power, extent and the directions it emits in (`LightBounds`). From a point, each node's
//! importance estimates how much light the group could deliver there: its power, over the
//! squared distance, times the cosine of the smallest angle between its emission cone and the
//! direction to the point. Walking down from the root, choosing each child in proportion to
//! its importance, picks one light with a probability that follows its contribution, and that
//! probability (`pmf`) is the product of the choices along the way, for weighting the sample
//! against BSDF sampling.
//!
//! pbrt also uses the shading normal; ours doesn't, so that `pmf` can be evaluated where only
//! the previous vertex's position is known (the trace, when a scattered ray hits a light).

use crate::na::{Rotation3, Unit, Vector3};
use crate::shapes::bbox::BBox;
use std::f64::consts::PI;

/// Bounds on what a group of lights emits: where (`bounds`), how much (`phi`, power), and in
/// which directions: surfaces whose normals lie within the cone of half-angle θ_o about `w`,
/// each emitting within θ_e of its normal (π/2 for a diffuse emitter).
#[derive(Clone, Copy, Debug)]
pub struct LightBounds {
    pub bounds: BBox,
    pub phi: f64,
    pub w: Vector3<f64>,
    pub cos_theta_o: f64,
    pub cos_theta_e: f64,
    pub two_sided: bool,
}

impl LightBounds {
    /// A light at a point emitting in every direction.
    pub fn point(p: Vector3<f64>, phi: f64) -> LightBounds {
        LightBounds { bounds: BBox::new(p, p), phi, w: Vector3::new(0., 0., 1.), cos_theta_o: -1., cos_theta_e: 0., two_sided: false }
    }

    /// How much light these lights could deliver to `p`, up to a constant (pbrt's
    /// `LightBounds::Importance`, without the receiver's normal).
    pub fn importance(&self, p: &Vector3<f64>) -> f64 {
        if self.phi <= 0. {
            return 0.;
        }
        let centre = (self.bounds.min.coords + self.bounds.max.coords) / 2.;
        let radius = (self.bounds.max - self.bounds.min).norm() / 2.;
        let to = p - centre;
        // Never nearer than half the bounds' diagonal: a point among the lights mustn't make
        // the estimate blow up.
        let d2 = to.norm_squared().max(radius * radius / 4.).max(f64::MIN_POSITIVE);
        let dist = to.norm();
        // The angle between the cone's axis and the direction to the point.
        let cos_w = if dist > 0. { self.w.dot(&(to / dist)) } else { 1. };
        let cos_w = if self.two_sided { cos_w.abs() } else { cos_w };
        let theta_w = cos_w.clamp(-1., 1.).acos();
        // The angle the bounds subtend from the point (all directions from inside).
        let theta_b = if dist <= radius { PI } else { (radius / dist).asin() };
        let theta_o = self.cos_theta_o.clamp(-1., 1.).acos();
        let theta = (theta_w - theta_o - theta_b).max(0.);
        let cos = theta.cos();
        if cos <= self.cos_theta_e {
            return 0.;
        }
        self.phi * cos / d2
    }

    /// Bounds on both groups.
    pub fn union(&self, o: &LightBounds) -> LightBounds {
        if self.phi <= 0. {
            return *o;
        }
        if o.phi <= 0. {
            return *self;
        }
        let (w, cos_theta_o) = cone_union((self.w, self.cos_theta_o), (o.w, o.cos_theta_o));
        LightBounds {
            bounds: self.bounds.union(&o.bounds),
            phi: self.phi + o.phi,
            w,
            cos_theta_o,
            cos_theta_e: self.cos_theta_e.min(o.cos_theta_e),
            two_sided: self.two_sided || o.two_sided,
        }
    }
}

/// The smallest cone holding both cones of directions (axis, cosine of half-angle): pbrt's
/// `Union(DirectionCone, DirectionCone)`.
fn cone_union(a: (Vector3<f64>, f64), b: (Vector3<f64>, f64)) -> (Vector3<f64>, f64) {
    let theta_a = a.1.clamp(-1., 1.).acos();
    let theta_b = b.1.clamp(-1., 1.).acos();
    let theta_d = a.0.dot(&b.0).clamp(-1., 1.).acos();
    if (theta_d + theta_b).min(PI) <= theta_a {
        return a;
    }
    if (theta_d + theta_a).min(PI) <= theta_b {
        return b;
    }
    let theta_o = (theta_a + theta_d + theta_b) / 2.;
    if theta_o >= PI {
        return (a.0, -1.);
    }
    let theta_r = theta_o - theta_a;
    let axis = a.0.cross(&b.0);
    if axis.norm_squared() < 1e-20 {
        return (a.0, -1.);
    }
    let w = Rotation3::from_axis_angle(&Unit::new_normalize(axis), theta_r) * a.0;
    // A hair wider, against rounding in the rotation.
    (w, (theta_o + 1e-7).min(PI).cos())
}

#[derive(Clone, Debug)]
struct Node {
    bounds: LightBounds,
    /// A light's index, or the index of the second child (the first follows the node).
    leaf: bool,
    index: usize,
}

/// Lights in a tree by position, for choosing one by importance.
pub struct LightBvh {
    nodes: Vec<Node>,
    /// For each light, its path from the root: bit k set if the k-th choice is the second
    /// child.
    trails: Vec<u64>,
}

impl LightBvh {
    /// A tree over lights with these bounds (lights with no power are left out, and never
    /// chosen).
    pub fn new(lights: &[LightBounds]) -> LightBvh {
        let mut bvh = LightBvh { nodes: Vec::new(), trails: vec![0; lights.len()] };
        let mut items: Vec<usize> = (0..lights.len()).filter(|&i| lights[i].phi > 0.).collect();
        if !items.is_empty() {
            bvh.build(lights, &mut items, 0, 0);
        }
        bvh
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Build the subtree over `items`, reached by `trail` at `depth`; returns its bounds.
    fn build(&mut self, lights: &[LightBounds], items: &mut [usize], trail: u64, depth: u32) -> LightBounds {
        if items.len() == 1 {
            self.nodes.push(Node { bounds: lights[items[0]], leaf: true, index: items[0] });
            self.trails[items[0]] = trail;
            return lights[items[0]];
        }
        assert!(depth < 64, "Light BVH too deep");
        // Split at the median along the axis the lights' centres spread furthest.
        let centre = |i: usize| (lights[i].bounds.min.coords + lights[i].bounds.max.coords) / 2.;
        let (lo, hi) = items.iter().fold((Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY)),
            |(lo, hi), &i| (lo.inf(&centre(i)), hi.sup(&centre(i))));
        let axis = (hi - lo).imax();
        let mid = items.len() / 2;
        items.select_nth_unstable_by(mid, |&a, &b| centre(a)[axis].total_cmp(&centre(b)[axis]));
        let at = self.nodes.len();
        self.nodes.push(Node { bounds: lights[items[0]], leaf: false, index: 0 });
        let (first, second) = items.split_at_mut(mid);
        let a = self.build(lights, first, trail, depth + 1);
        self.nodes[at].index = self.nodes.len();
        let b = self.build(lights, second, trail | (1 << depth), depth + 1);
        let bounds = a.union(&b);
        self.nodes[at].bounds = bounds;
        bounds
    }

    /// One light chosen for the point `p` with the random number `u`, and the probability of
    /// choosing it; None if no light can reach `p`.
    pub fn sample(&self, p: &Vector3<f64>, mut u: f64) -> Option<(usize, f64)> {
        let mut node = 0;
        let mut pmf = 1.;
        loop {
            let n = self.nodes.get(node)?;
            if n.leaf {
                return Some((n.index, pmf));
            }
            let (c0, c1) = (node + 1, n.index);
            let i0 = self.nodes[c0].bounds.importance(p);
            let i1 = self.nodes[c1].bounds.importance(p);
            if i0 + i1 <= 0. {
                return None;
            }
            let p0 = i0 / (i0 + i1);
            if u < p0 {
                node = c0;
                pmf *= p0;
                u = (u / p0).min(1. - f64::EPSILON);
            } else {
                node = c1;
                pmf *= 1. - p0;
                u = ((u - p0) / (1. - p0)).min(1. - f64::EPSILON);
            }
        }
    }

    /// The probability that `sample` from `p` chooses light `light`.
    pub fn pmf(&self, p: &Vector3<f64>, light: usize) -> f64 {
        if self.nodes.is_empty() || light >= self.trails.len() {
            return 0.;
        }
        let trail = self.trails[light];
        let mut node = 0;
        let mut pmf = 1.;
        let mut depth = 0;
        loop {
            let n = &self.nodes[node];
            if n.leaf {
                return if n.index == light { pmf } else { 0. };
            }
            let (c0, c1) = (node + 1, n.index);
            let i0 = self.nodes[c0].bounds.importance(p);
            let i1 = self.nodes[c1].bounds.importance(p);
            if i0 + i1 <= 0. {
                return 0.;
            }
            if trail & (1 << depth) == 0 {
                pmf *= i0 / (i0 + i1);
                node = c0;
            } else {
                pmf *= i1 / (i0 + i1);
                node = c1;
            }
            depth += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{rand, seed_rand};

    fn grid_of_points(n: usize) -> Vec<LightBounds> {
        (0..n * n).map(|i| LightBounds::point(Vector3::new((i % n) as f64, 0., (i / n) as f64), 1. + (i % 3) as f64)).collect()
    }

    #[test]
    fn test_pmf_matches_sampling_and_sums_to_one() {
        let lights = grid_of_points(7);
        let bvh = LightBvh::new(&lights);
        seed_rand(1, 0);
        for _ in 0..20 {
            let p = Vector3::new(rand() * 10. - 2., rand() * 3., rand() * 10. - 2.);
            let total: f64 = (0..lights.len()).map(|i| bvh.pmf(&p, i)).sum();
            assert!((total - 1.).abs() < 1e-9, "{}", total);
            for _ in 0..50 {
                let (i, pmf) = bvh.sample(&p, rand()).unwrap();
                assert!((pmf - bvh.pmf(&p, i)).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn test_nearby_lights_are_chosen_more() {
        let lights = grid_of_points(10);
        let bvh = LightBvh::new(&lights);
        let p = Vector3::new(0., 0.5, 0.);
        // The light beneath the point against the far corner's, of equal power.
        assert!(bvh.pmf(&p, 0) > 20. * bvh.pmf(&p, 99), "{} vs {}", bvh.pmf(&p, 0), bvh.pmf(&p, 99));
    }

    #[test]
    fn test_one_sided_lights_facing_away_are_never_chosen() {
        let facing = |y: f64, w: Vector3<f64>| LightBounds {
            bounds: BBox::new(Vector3::new(-0.5, y, -0.5), Vector3::new(0.5, y, 0.5)),
            phi: 1., w, cos_theta_o: 1., cos_theta_e: 0., two_sided: false,
        };
        // A ceiling light facing down and one facing up, both above the point.
        let lights = [facing(2., Vector3::new(0., -1., 0.)), facing(2., Vector3::new(0., 1., 0.))];
        let bvh = LightBvh::new(&lights);
        let p = Vector3::new(0., 0., 0.);
        assert!((bvh.pmf(&p, 0) - 1.).abs() < 1e-12);
        assert_eq!(bvh.pmf(&p, 1), 0.);
    }

    #[test]
    fn test_cone_union_holds_both() {
        seed_rand(2, 0);
        for _ in 0..1000 {
            let d = || Vector3::new(rand() - 0.5, rand() - 0.5, rand() - 0.5).normalize();
            let (a, b) = ((d(), rand() * 2. - 1.), (d(), rand() * 2. - 1.));
            let (w, cos) = cone_union(a, b);
            let theta = cos.clamp(-1., 1.).acos();
            for c in [a, b] {
                // Each cone's axis plus its half-angle fits within the union's.
                let reach = c.0.dot(&w).clamp(-1., 1.).acos() + c.1.clamp(-1., 1.).acos();
                assert!(reach <= theta + 1e-6 || cos <= -1., "{} > {}", reach, theta);
            }
        }
    }
}
