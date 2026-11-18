//! Lightning: a branching channel of hot air, glowing, that lights the storm around it.
//!
//! A cloud-to-ground flash starts with a *stepped leader*: a channel of ionised air that
//! pushes down from the cloud's negative charge region in steps of tens of metres, pausing
//! after each, and branching as it goes. Each step leaves at an angle to the last: photographs
//! of channels show a mean change of direction of about 16° between straight sections 5 to
//! 70 m long (D. Hill, "Analysis of irregular paths of lightning channels", J. Geophys. Res.
//! 73, 1968), and the same irregularity at every scale down to a metre or so. Every branch
//! races towards the ground; when one comes within striking distance (tens of metres, where an
//! upward leader rises to meet it), the circuit is closed and the *return stroke* sends a
//! current of tens of kiloamperes up the channel. That path, from cloud to ground, is the
//! bright main channel. The branches that lost the race carry only the charge stored along
//! them, so they are fainter, and fade towards their tips. Inside the cloud, the flash's
//! other end spreads sideways through the positive charge region in a network of fainter
//! channels, which light the cloud from within. An intracloud flash (sheet lightning) is that
//! network alone.
//!
//! The channel here is grown that way: leader tips step together, `step` metres a step,
//! turning by about `tortuosity` degrees, drawn down by the field (and towards `target`),
//! branching with probability `branching` per step and dying out, until one comes within
//! `striking_distance` of the ground. Each straight step is then broken into a fractal path
//! down to `detail` metres, by midpoint displacement. Each part of the channel glows in
//! proportion to the current through it: the main channel fully, a branch as the length of
//! channel beyond it (the charge that drains through it) against `branch_scale`.
//!
//! **Brightness.** The channel is a surface of radius `radius` emitting diffusely, as bright as
//! a stroke's light averaged over the exposure: `strokes` strokes of `luminous_energy`
//! lumen-seconds per metre each, over `shutter` seconds. Measured return strokes radiate a few
//! hundred joules per metre between 0.4 and 1.1 µm (E. P. Krider, R. C. Noggle and M. A. Uman,
//! J. Appl. Meteorol. 7, 1968; Y. Guo and E. P. Krider, J. Geophys. Res. 87, 1982), about 1.5·10⁴
//! lm s/m in the visible. The light is on the scene's scale, on which a sun of intensity 1 gives
//! 1.27·10⁵ lux (`sun_intensity`, set from the scene's sun). Its colour is a blackbody at
//! `temperature` (the channel's spectrum is mostly lines of nitrogen, oxygen and hydrogen, at
//! some 30 000 K, which look bluish white).
//!
//! As a light, the channel is sampled segment by segment: a light BVH (`light_bvh`) over its
//! segments chooses one by how much light it could send to the point being lit, then a point on
//! the half of its cylinder facing that point. So clouds, rain and ground are lit by it with
//! next-event estimation, through the cloud between.

use crate::color::Color;
use crate::geometry::Pcg32;
use crate::intersection::{Intersection, RawIntersection};
use crate::light_bvh::{LightBounds, LightBvh};
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::material::texture::Solid;
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scenefile::SceneFile;
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use crate::spectrum::Emission;
use serde_json::Value;
use std::f64::consts::PI;

/// Illuminance of a sun of intensity 1 (lux): the solar constant's, as the scene's other
/// physically bright things are scaled (`procedural::space`, `celestial::moon`).
pub const SUN_ILLUMINANCE: f64 = 1.27e5;

/// A straight piece of the channel: a capsule from `a` to `b`.
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub radius: f64,
    /// The light it gives per metre, relative to the main channel's.
    pub current: f64,
}

impl Segment {
    fn length(&self) -> f64 {
        (self.b - self.a).norm()
    }

    fn area(&self) -> f64 {
        2. * PI * self.radius * self.length()
    }

    /// Its radiance relative to the main channel's: its light per metre spread over its own
    /// circumference.
    fn radiance(&self, main_radius: f64) -> f64 {
        self.current * main_radius / self.radius
    }

    fn bounds(&self) -> BBox {
        let r = Vector3::new(self.radius, self.radius, self.radius);
        BBox::new(self.a.inf(&self.b) - r, self.a.sup(&self.b) + r)
    }

    /// The nearest hit on the capsule beyond `min` along the unit direction `rd`.
    fn hit(&self, ro: &Vector3<f64>, rd: &Vector3<f64>, min: f64) -> Option<f64> {
        let ba = self.b - self.a;
        let oa = ro - self.a;
        let baba = ba.dot(&ba);
        let bard = ba.dot(rd);
        let baoa = ba.dot(&oa);
        let rdoa = rd.dot(&oa);
        let oaoa = oa.dot(&oa);
        let r2 = self.radius * self.radius;
        let a = baba - bard * bard;
        let b = baba * rdoa - baoa * bard;
        let c = baba * oaoa - baoa * baoa - r2 * baba;
        // The side, then the caps, taking each sphere's or cylinder's two roots in turn.
        let mut best: Option<f64> = None;
        let mut consider = |t: f64| if t > min && best.is_none_or(|b| t < b) { best = Some(t) };
        if a > 1e-12 {
            let h = b * b - a * c;
            if h >= 0. {
                let s = h.sqrt();
                for t in [(-b - s) / a, (-b + s) / a] {
                    let y = baoa + t * bard;
                    if y > 0. && y < baba {
                        consider(t);
                    }
                }
            }
        }
        for (centre, outside) in [(self.a, -1.), (self.b, 1.)] {
            let oc = ro - centre;
            let b2 = rd.dot(&oc);
            let c2 = oc.dot(&oc) - r2;
            let h = b2 * b2 - c2;
            if h >= 0. {
                let s = h.sqrt();
                for t in [-b2 - s, -b2 + s] {
                    // Only the cap beyond its end of the cylinder.
                    let y = baoa + t * bard;
                    if (outside < 0. && y <= 0.) || (outside > 0. && y >= baba) {
                        consider(t);
                    }
                }
            }
        }
        best
    }

    /// The point on the axis nearest p, and how far along (0 to 1).
    fn nearest_on_axis(&self, p: &Vector3<f64>) -> (Vector3<f64>, f64) {
        let ba = self.b - self.a;
        let h = ((p - self.a).dot(&ba) / ba.norm_squared().max(1e-300)).clamp(0., 1.);
        (self.a + ba * h, h)
    }

    /// An orthonormal pair across the axis, the first towards `from`: the side facing it.
    fn facing(&self, from: &Vector3<f64>) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let t = (self.b - self.a).normalize();
        let off = from - self.a;
        let q = off - t * off.dot(&t);
        let d = q.norm();
        if d <= self.radius {
            return None;
        }
        let q = q / d;
        Some((q, t.cross(&q), d))
    }
}

/// A node of the BVH over the segments: a box, and either two children or a run of segments.
struct Node {
    bounds: BBox,
    /// For a leaf, the first segment and how many; for an interior node, the second child (the
    /// first follows it directly) and 0.
    start: usize,
    count: usize,
}

/// The channel as a shape: its segments, a BVH over them for rays, and a light BVH for sampling.
pub struct Channel {
    pub segments: Vec<Segment>,
    nodes: Vec<Node>,
    lights: LightBvh,
    /// Cumulative areas, for choosing a point by area.
    cumulative: Vec<f64>,
    area: f64,
    /// The main channel's radius, against which the others' radiance is given.
    main_radius: f64,
    /// The mean radiance over the surface, relative to the main channel's.
    mean_radiance: f64,
}

const LEAF: usize = 4;

impl Channel {
    pub fn new(mut segments: Vec<Segment>, main_radius: f64) -> Channel {
        assert!(!segments.is_empty(), "A lightning channel needs at least one segment");
        let mut nodes = Vec::with_capacity(2 * segments.len() / LEAF + 1);
        let n = segments.len();
        Channel::build(&mut segments, 0, n, &mut nodes);
        let mut cumulative = Vec::with_capacity(segments.len());
        let mut area = 0.;
        let mut weighted = 0.;
        for s in &segments {
            area += s.area();
            weighted += s.area() * s.radiance(main_radius);
            cumulative.push(area);
        }
        let bounds: Vec<LightBounds> = segments.iter().map(|s| LightBounds {
            bounds: s.bounds(),
            phi: s.current * s.length(),
            w: Vector3::new(0., 1., 0.),
            cos_theta_o: -1.,
            cos_theta_e: 0.,
            two_sided: false,
        }).collect();
        Channel {
            lights: LightBvh::new(&bounds),
            segments, nodes, cumulative, area, main_radius,
            mean_radiance: weighted / area.max(1e-300),
        }
    }

    /// Builds the BVH over `segments[start..end]`, reordering them, and returns its root's index.
    fn build(segments: &mut [Segment], start: usize, end: usize, nodes: &mut Vec<Node>) -> usize {
        let bounds = segments[start..end].iter().fold(BBox::empty(), |b, s| b.union(&s.bounds()));
        let index = nodes.len();
        nodes.push(Node { bounds, start, count: end - start });
        if end - start <= LEAF {
            return index;
        }
        let extent = bounds.max - bounds.min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z { 0 } else if extent.y >= extent.z { 1 } else { 2 };
        let mid = (start + end) / 2;
        let centre = |s: &Segment| (s.a[axis] + s.b[axis]) / 2.;
        segments[start..end].select_nth_unstable_by(mid - start, |p, q| centre(p).total_cmp(&centre(q)));
        Channel::build(segments, start, mid, nodes);
        let second = Channel::build(segments, mid, end, nodes);
        nodes[index].start = second;
        nodes[index].count = 0;
        index
    }

    /// The nearest hit in [min, max) along the ray, with the segment hit.
    fn nearest(&self, ro: &Vector3<f64>, rd: &Vector3<f64>, min: f64, max: f64) -> Option<(f64, usize)> {
        let inv = Vector3::new(1. / rd.x, 1. / rd.y, 1. / rd.z);
        let mut best: Option<(f64, usize)> = None;
        let mut stack = vec![0usize];
        while let Some(i) = stack.pop() {
            let node = &self.nodes[i];
            let limit = best.map_or(max, |b| b.0);
            let (near, far) = slab(&node.bounds, ro, &inv);
            if near > far || far < min || near > limit {
                continue;
            }
            if node.count > 0 {
                for k in node.start..node.start + node.count {
                    if let Some(t) = self.segments[k].hit(ro, rd, min) {
                        if t < best.map_or(max, |b| b.0) {
                            best = Some((t, k));
                        }
                    }
                }
            } else {
                stack.push(i + 1);
                stack.push(node.start);
            }
        }
        best
    }

    /// The segment whose surface `p` lies on: of those whose capsule nearly contains it, the
    /// one whose surface it is nearest.
    fn segment_at(&self, p: &Vector3<f64>) -> Option<usize> {
        let mut best: Option<(f64, usize)> = None;
        let mut stack = vec![0usize];
        while let Some(i) = stack.pop() {
            let node = &self.nodes[i];
            let slack = 1e-3 + 1e-6 * p.norm();
            if (0..3).any(|k| p[k] < node.bounds.min[k] - slack || p[k] > node.bounds.max[k] + slack) {
                continue;
            }
            if node.count > 0 {
                for k in node.start..node.start + node.count {
                    let s = &self.segments[k];
                    let (c, _) = s.nearest_on_axis(p);
                    let miss = ((p - c).norm() - s.radius).abs();
                    if best.is_none_or(|b| miss < b.0) {
                        best = Some((miss, k));
                    }
                }
            } else {
                stack.push(i + 1);
                stack.push(node.start);
            }
        }
        best.map(|b| b.1)
    }

    /// The radiance of the segment at p, relative to the main channel's.
    pub fn radiance_at(&self, p: &Vector3<f64>) -> f64 {
        self.segment_at(p).map_or(0., |k| self.segments[k].radiance(self.main_radius))
    }
}

/// Where the ray enters and leaves the box.
fn slab(b: &BBox, ro: &Vector3<f64>, inv: &Vector3<f64>) -> (f64, f64) {
    let mut near = f64::NEG_INFINITY;
    let mut far = f64::INFINITY;
    for k in 0..3 {
        let t0 = (b.min[k] - ro[k]) * inv[k];
        let t1 = (b.max[k] - ro[k]) * inv[k];
        let (lo, hi) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
        // NaN (a ray in the plane of a face) compares false and leaves the bounds alone.
        if lo > near { near = lo; }
        if hi < far { far = hi; }
    }
    (near, far)
}

impl Geometry for Channel {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        let len = r.rd.norm();
        let rd = r.rd / len;
        // Distances along the ray are in world units (`rd` unit), as for the other shapes.
        let (t, k) = self.nearest(&r.ro, &rd, min.max(1e-9), max)?;
        let point = r.ro + rd * t;
        let s = &self.segments[k];
        let (c, _) = s.nearest_on_axis(&point);
        let normal = (point - c).normalize();
        Some(RawIntersection { dist: t, point, normal, face: k as u32, part: 0 })
    }

    fn bounds(&self) -> BBox {
        self.nodes[0].bounds
    }

    /// The real channel is centimetres across and shadows nothing; and widened to a pixel
    /// (`parse_lightning`) its segments overlap, so shadow rays towards one would be stopped by
    /// its neighbours.
    fn occludes(&self, _r: &Ray, _min: f64, _max: f64) -> bool { false }

    fn transmittance(&self, _r: &Ray, _min: f64, _max: f64) -> f64 { 1. }

    /// One part: area light sampling then looks up each sampled point's brightness (`uv`).
    fn primitives(&self) -> u64 { 1 }

    fn surface_area(&self) -> f64 {
        self.area
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let target = u[0] * self.area;
        let k = self.cumulative.partition_point(|c| *c < target).min(self.segments.len() - 1);
        let s = &self.segments[k];
        let t = (s.b - s.a).normalize();
        let helper = if t.x.abs() < 0.9 { Vector3::new(1., 0., 0.) } else { Vector3::new(0., 1., 0.) };
        let e1 = t.cross(&helper).normalize();
        let e2 = t.cross(&e1);
        let phi = 2. * PI * u[1];
        let n = e1 * phi.cos() + e2 * phi.sin();
        Some((s.a + (s.b - s.a) * u[2] + n * s.radius, n))
    }

    /// Brightness as texture coordinates: u is the radiance relative to the main channel's.
    fn uv(&self, p: &Vector3<f64>, _face: u32) -> Option<Vector2<f64>> {
        Some(Vector2::new(self.radiance_at(p), 0.))
    }

    /// A segment chosen by the light BVH from `from`, then a point on the half of it facing
    /// `from`, uniformly by area.
    fn sample_from(&self, from: &Vector3<f64>, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let (k, chosen) = self.lights.sample(from, u[0])?;
        let s = &self.segments[k];
        let (q, e, _) = s.facing(from)?;
        let phi = (u[1] - 0.5) * PI;
        let n = q * phi.cos() + e * phi.sin();
        let y = s.a + (s.b - s.a) * u[2] + n * s.radius;
        let w = from - y;
        let d2 = w.norm_squared();
        let cos = n.dot(&w) / d2.sqrt();
        if cos <= 0. {
            return None;
        }
        let pdf = chosen * d2 / (cos * PI * s.radius * s.length());
        (pdf.is_finite() && pdf > 0.).then_some((y, n, pdf))
    }

    fn pdf_from(&self, from: &Vector3<f64>, y: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        let Some(k) = self.segment_at(y) else { return 0. };
        let s = &self.segments[k];
        let Some((q, _, _)) = s.facing(from) else { return 0. };
        let (c, _) = s.nearest_on_axis(y);
        if (y - c).dot(&q) < 0. {
            return 0.;
        }
        let w = from - y;
        let d2 = w.norm_squared();
        let cos = n.dot(&w) / d2.sqrt();
        if cos <= 0. {
            return 0.;
        }
        self.lights.pmf(from, k) * d2 / (cos * PI * s.radius * s.length())
    }
}

/// The channel's glow: diffuse emission, the main channel's `radiance` scaled by each part's
/// relative radiance (given as its texture coordinate u).
pub struct LightningMaterial {
    pub emission: Emission,
    pub radiance: f64,
    pub mean: f64,
}

impl LightningMaterial {
    fn glow(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, relative: f64) -> Color {
        if towards.dot(normal) <= 0. {
            return Color::black();
        }
        self.emission.radiance() * (self.radiance * relative)
    }
}

impl MaterialModel for LightningMaterial {
    fn is_emitter(&self) -> bool { true }

    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        self.glow(normal, towards, self.mean)
    }

    fn emitted_at(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, uv: Option<Vector2<f64>>) -> Color {
        self.glow(normal, towards, uv.map_or(self.mean, |uv| uv.x))
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        let radiance = self.emitted_at(&intersection.normal, &-r.rd.normalize(), intersection.uv());
        ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: radiance, ray: None }
    }
}

/// What kind of flash.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    CloudToGround,
    Intracloud,
}

/// How the channel grows.
#[derive(Clone, Debug)]
pub struct Leader {
    pub kind: Kind,
    /// Where the flash starts, in the cloud's charge region, and the height of the ground.
    pub start: Vector3<f64>,
    pub ground: f64,
    /// Where on the ground it is drawn to ([x, z]), if anywhere.
    pub target: Option<Vector2<f64>>,
    /// Mean step length (m), mean change of direction per step (degrees), and how strongly the
    /// field draws the leader down (0 to 1).
    pub step: f64,
    pub tortuosity: f64,
    pub pull: f64,
    /// Chance per step that a tip forks, and that a branch dies out.
    pub branching: f64,
    pub dying: f64,
    pub striking_distance: f64,
    /// Total length of the channels spreading through the cloud (m), their height range about
    /// the start (m), and their relative brightness.
    pub cloud_length: f64,
    pub cloud_depth: f64,
    pub cloud_current: f64,
    /// A branch's light: `branch_current` of the main channel's for a long branch, falling for
    /// one with less than `branch_scale` metres of channel beyond it.
    pub branch_current: f64,
    pub branch_scale: f64,
    /// The finest detail of the path (m), and how far midpoint displacement moves it, as a
    /// fraction of each piece's length.
    pub detail: f64,
    pub roughness: f64,
    /// The main channel's radius (m); others are narrower as they are fainter.
    pub radius: f64,
    pub seed: u64,
}

impl Default for Leader {
    fn default() -> Leader {
        Leader {
            kind: Kind::CloudToGround,
            start: Vector3::new(0., 6000., 0.),
            ground: 0.,
            target: None,
            step: 45.,
            tortuosity: 16.,
            pull: 0.35,
            branching: 0.07,
            dying: 0.035,
            striking_distance: 60.,
            cloud_length: 6000.,
            cloud_depth: 1500.,
            cloud_current: 0.25,
            branch_current: 0.45,
            branch_scale: 600.,
            detail: 2.,
            roughness: 0.18,
            radius: 0.3,
            seed: 0,
        }
    }
}

/// A point of the coarse channel tree, and the one it grew from.
struct Point {
    p: Vector3<f64>,
    parent: Option<usize>,
    /// Inside the cloud's network rather than the leader to ground.
    cloud: bool,
}

struct Tip {
    at: usize,
    dir: Vector3<f64>,
    cloud: bool,
}

fn gaussian(rng: &mut Pcg32) -> f64 {
    // Box-Muller.
    let u1 = rng.next_f64().max(1e-12);
    let u2 = rng.next_f64();
    (-2. * u1.ln()).sqrt() * (2. * PI * u2).cos()
}

fn random_direction(rng: &mut Pcg32) -> Vector3<f64> {
    Vector3::new(gaussian(rng), gaussian(rng), gaussian(rng)).normalize()
}

/// A step length: log-normal about the mean, as Hill's measured segments (5 to 70 m).
fn step_length(mean: f64, rng: &mut Pcg32) -> f64 {
    mean * (0.45 * gaussian(rng) - 0.1).exp()
}

impl Leader {
    /// Grows the channel and returns its segments.
    pub fn grow(&self) -> Vec<Segment> {
        let mut rng = Pcg32::new(self.seed, 0x11);
        let mut points = vec![Point { p: self.start, parent: None, cloud: false }];
        // The spread of each step's kick: across the direction it is a 2D Gaussian of σ per
        // axis, which turns it by σ √(π/2) ≈ 1.25 σ on average, for small angles.
        let sigma = self.tortuosity.to_radians() / 1.25;
        let mut main_tip: Option<usize> = None;

        // The leader towards the ground.
        if self.kind == Kind::CloudToGround {
            let mut tips = vec![Tip { at: 0, dir: Vector3::new(0.3 * gaussian(&mut rng), -1., 0.3 * gaussian(&mut rng)).normalize(), cloud: false }];
            let max_steps = (20. * (self.start.y - self.ground) / self.step) as usize + 100;
            'race: for _ in 0..max_steps {
                let mut born = vec![];
                let mut k = 0;
                while k < tips.len() {
                    let from = points[tips[k].at].p;
                    let field = self.field_at(&from);
                    let kick = Vector3::new(gaussian(&mut rng), gaussian(&mut rng), gaussian(&mut rng)) * sigma;
                    let dir = (tips[k].dir + field * self.pull * sigma * 2. + kick).normalize();
                    let to = from + dir * step_length(self.step, &mut rng);
                    points.push(Point { p: to, parent: Some(tips[k].at), cloud: false });
                    tips[k].at = points.len() - 1;
                    tips[k].dir = dir;
                    if to.y - self.ground < self.striking_distance {
                        main_tip = Some(tips[k].at);
                        break 'race;
                    }
                    // Forks leave at 20° to 50°; branches die out, the leader can't.
                    if rng.next_f64() < self.branching && tips.len() < 400 {
                        let r = random_direction(&mut rng);
                        let side = (r - dir * dir.dot(&r)).normalize();
                        let angle = (20. + 30. * rng.next_f64()).to_radians();
                        born.push(Tip { at: tips[k].at, dir: (dir * angle.cos() + side * angle.sin()).normalize(), cloud: false });
                    }
                    if k > 0 && rng.next_f64() < self.dying {
                        tips.swap_remove(k);
                    } else {
                        k += 1;
                    }
                }
                // A branch that overtakes the leader may become it: the lowest tip leads.
                tips.extend(born);
                if let Some(lowest) = (0..tips.len()).min_by(|a, b| points[tips[*a].at].p.y.total_cmp(&points[tips[*b].at].p.y)) {
                    tips.swap(0, lowest);
                }
            }
            // The last gap closes with a short upward leader: straight to the ground.
            if let Some(tip) = main_tip {
                let p = points[tip].p;
                let foot = Vector3::new(p.x + 0.3 * (p.y - self.ground) * gaussian(&mut rng), self.ground - 1., p.z + 0.3 * (p.y - self.ground) * gaussian(&mut rng));
                points.push(Point { p: foot, parent: Some(tip), cloud: false });
                main_tip = Some(points.len() - 1);
            }
        }

        // The network through the cloud: horizontal channels spreading out from the start.
        let mut grown = 0.;
        let mut tips: Vec<Tip> = (0..3).map(|i| {
            let a = 2. * PI * (i as f64 + rng.next_f64() * 0.5) / 3.;
            Tip { at: 0, dir: Vector3::new(a.cos(), 0.15 * gaussian(&mut rng), a.sin()).normalize(), cloud: true }
        }).collect();
        while grown < self.cloud_length && !tips.is_empty() {
            let mut born = vec![];
            let mut k = 0;
            while k < tips.len() && grown < self.cloud_length {
                let from = points[tips[k].at].p;
                // Held near the charge layer, spreading outwards.
                let level = (self.start.y - from.y) / self.cloud_depth.max(1.);
                let out = Vector3::new(from.x - self.start.x, 0., from.z - self.start.z);
                let out = if out.norm() > 1. { out.normalize() } else { tips[k].dir };
                let field = Vector3::new(0., level, 0.) * 0.8 + out;
                let kick = Vector3::new(gaussian(&mut rng), gaussian(&mut rng) * 0.6, gaussian(&mut rng)) * sigma * 1.2;
                let dir = (tips[k].dir + field * sigma * 2. + kick).normalize();
                let len = step_length(self.step, &mut rng);
                let to = from + dir * len;
                grown += len;
                points.push(Point { p: to, parent: Some(tips[k].at), cloud: true });
                tips[k].at = points.len() - 1;
                tips[k].dir = dir;
                if rng.next_f64() < self.branching * 0.5 && tips.len() < 16 {
                    let side = Vector3::new(-dir.z, 0.2 * gaussian(&mut rng), dir.x).normalize() * if rng.next_f64() < 0.5 { 1. } else { -1. };
                    let angle = (25. + 35. * rng.next_f64()).to_radians();
                    born.push(Tip { at: tips[k].at, dir: (dir * angle.cos() + side * angle.sin()).normalize(), cloud: true });
                }
                if tips.len() > 1 && rng.next_f64() < self.dying * 0.25 {
                    tips.swap_remove(k);
                } else {
                    k += 1;
                }
            }
            tips.extend(born);
        }

        self.segments(&points, main_tip, &mut rng)
    }

    /// The direction the field draws a leader at p: down, and towards the target.
    fn field_at(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let down = Vector3::new(0., -1., 0.);
        match self.target {
            None => down,
            Some(t) => {
                let to = Vector3::new(t.x - p.x, self.ground - p.y, t.y - p.z);
                (down + to.normalize()).normalize()
            },
        }
    }

    /// The segments of the coarse tree, each broken into a fractal path, glowing as the
    /// current through it.
    fn segments(&self, points: &[Point], main_tip: Option<usize>, rng: &mut Pcg32) -> Vec<Segment> {
        let n = points.len();
        // The main channel: the path from the ground back to the start.
        let mut main = vec![false; n];
        let mut at = main_tip;
        while let Some(i) = at {
            main[i] = true;
            at = points[i].parent;
        }
        // Channel length beyond each point (children come after their parents).
        let mut beyond = vec![0.; n];
        for i in (1..n).rev() {
            let parent = points[i].parent.unwrap();
            beyond[parent] += beyond[i] + (points[i].p - points[parent].p).norm();
        }
        let mut out = vec![];
        for i in 1..n {
            let parent = points[i].parent.unwrap();
            let current = if main[i] {
                1.
            } else {
                let scale = if points[i].cloud { self.cloud_current } else { self.branch_current };
                let carried = beyond[i] + (points[i].p - points[parent].p).norm();
                scale * carried / (carried + self.branch_scale)
            };
            let radius = self.radius * (0.35 + 0.65 * current.sqrt());
            let mut path = vec![points[parent].p, points[i].p];
            self.roughen(&mut path, rng);
            for w in path.windows(2) {
                out.push(Segment { a: w[0], b: w[1], radius, current });
            }
        }
        out
    }

    /// Midpoint displacement of a straight step down to `detail`.
    fn roughen(&self, path: &mut Vec<Vector3<f64>>, rng: &mut Pcg32) {
        let mut pieces = path.clone();
        loop {
            let longest = pieces.windows(2).map(|w| (w[1] - w[0]).norm()).fold(0., f64::max);
            if longest <= self.detail.max(0.1) {
                break;
            }
            let mut next = Vec::with_capacity(2 * pieces.len());
            for w in pieces.windows(2) {
                next.push(w[0]);
                let d = w[1] - w[0];
                let len = d.norm();
                if len > self.detail {
                    let axis = d / len;
                    let r = random_direction(rng);
                    let across = (r - axis * r.dot(&axis)).normalize();
                    next.push((w[0] + w[1]) / 2. + across * (len * self.roughness * gaussian(rng)));
                }
            }
            next.push(*pieces.last().unwrap());
            pieces = next;
        }
        *path = pieces;
    }
}

/// A lightning flash (see the module's documentation):
///
/// `{"type": "lightning", "start": [x, y, z], "ground": 0, "seed": 0}`
///
/// with `kind` ("cloud_to_ground" or "intracloud"), `target` ([x, z]), `step`, `tortuosity`
/// (degrees), `pull`, `branching`, `dying`, `striking_distance`, `cloud_length`,
/// `cloud_depth`, `cloud_current`, `branch_current`, `branch_scale`, `detail`, `roughness`,
/// `radius` (at least `pixel_width` pixels wide, 0.5 by default, from the `camera` at
/// `pixel_angle`, both set from the scene's); its light: `luminous_energy` (lm s per metre per stroke), `strokes`, `shutter`
/// (s), `temperature` (K), and `sun_intensity` (the scene's sun's, set from it).
pub fn parse_lightning(o: &Value) -> SceneObject {
    let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
    let d = Leader::default();
    let leader = Leader {
        kind: match o["kind"].as_str() {
            None | Some("cloud_to_ground") => Kind::CloudToGround,
            Some("intracloud") => Kind::Intracloud,
            Some(k) => panic!("Lightning's \"kind\" is \"cloud_to_ground\" or \"intracloud\", not {:?}", k),
        },
        start: SceneFile::parse_vec3_def(o, "start", d.start),
        ground: num("ground", d.ground),
        target: o.get("target").map(|_| SceneFile::parse_vec2_def(o, "target", Vector2::zeros())),
        step: num("step", d.step).max(1.),
        tortuosity: num("tortuosity", d.tortuosity).clamp(0., 90.),
        pull: num("pull", d.pull).max(0.),
        branching: num("branching", d.branching).clamp(0., 1.),
        dying: num("dying", d.dying).clamp(0., 1.),
        striking_distance: num("striking_distance", d.striking_distance).max(1.),
        cloud_length: num("cloud_length", d.cloud_length).max(0.),
        cloud_depth: num("cloud_depth", d.cloud_depth).max(1.),
        cloud_current: num("cloud_current", d.cloud_current).max(0.),
        branch_current: num("branch_current", d.branch_current).max(0.),
        branch_scale: num("branch_scale", d.branch_scale).max(1.),
        detail: num("detail", d.detail).max(0.1),
        roughness: num("roughness", d.roughness).max(0.),
        radius: num("radius", d.radius).max(1e-3),
        seed: SceneFile::parse_seed(o).unwrap_or(0),
    };
    let mut segments = leader.grow();
    // Seen from afar the channel is far thinner than a pixel, and most samples would miss it.
    // What a camera records of it there is its light per metre, not its width: so widen it to
    // `pixel_width` pixels as seen from the camera, keeping its light per metre (its radiance
    // falls as its radius grows, `Segment::radiance`).
    if let (Some(camera), Some(angle)) = (o.get("camera"), o["pixel_angle"].as_f64()) {
        let camera = SceneFile::parse_vec3(camera);
        let width = num("pixel_width", 0.5).max(0.);
        for s in segments.iter_mut() {
            let d = ((s.a + s.b) / 2. - camera).norm();
            s.radius = s.radius.max(width * angle * d / 2.);
        }
    }
    let channel = Channel::new(segments, leader.radius);
    // The main channel's luminous flux per metre, averaged over the exposure, leaving its
    // surface diffusely: luminance Φ' / (2π r) / π, on the scene's scale.
    let flux = num("luminous_energy", 1.5e4) * num("strokes", 4.) / num("shutter", 1.).max(1e-6);
    let luminance = flux / (2. * PI * leader.radius) / PI;
    let radiance = luminance / SUN_ILLUMINANCE * num("sun_intensity", 1.);
    let material = LightningMaterial {
        emission: Emission::blackbody(num("temperature", 25_000.)),
        radiance,
        mean: channel.mean_radiance,
    };
    SceneObject { geometry: Box::new(channel), medium: Box::new(Solid { m: Box::new(material) }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bolt(seed: u64) -> Vec<Segment> {
        Leader { seed, ..Leader::default() }.grow()
    }

    #[test]
    fn test_main_channel_reaches_the_ground_from_the_start() {
        for seed in 0..4 {
            let s = bolt(seed);
            let main: Vec<&Segment> = s.iter().filter(|s| s.current == 1.).collect();
            assert!(main.iter().any(|s| s.a.y.min(s.b.y) < 0.), "seed {}", seed);
            assert!(main.iter().any(|s| (s.a - Vector3::new(0., 6000., 0.)).norm() < 1e-9));
            // Branches are fainter than the main channel.
            assert!(s.iter().filter(|s| s.current < 1.).all(|s| s.current < 0.5));
            assert!(s.iter().all(|s| s.length() <= Leader::default().detail + 1e-9));
        }
    }

    #[test]
    fn test_tortuosity_is_as_measured() {
        // Coarse steps turn by about 16° on average (Hill 1968).
        let l = Leader { detail: 1e9, branching: 0., cloud_length: 0., ..Leader::default() };
        let s = l.grow();
        let dirs: Vec<Vector3<f64>> = s.iter().filter(|s| s.b.y > 100.).map(|s| (s.b - s.a).normalize()).collect();
        let mean = dirs.windows(2).map(|w| w[0].dot(&w[1]).clamp(-1., 1.).acos().to_degrees()).sum::<f64>() / (dirs.len() - 1) as f64;
        assert!(mean > 10. && mean < 24., "{}", mean);
    }

    #[test]
    fn test_capsule_hits_and_lookup() {
        let s = Segment { a: Vector3::new(0., 0., 0.), b: Vector3::new(0., 10., 0.), radius: 1., current: 1. };
        let rd = Vector3::new(1., 0., 0.);
        assert!((s.hit(&Vector3::new(-5., 5., 0.), &rd, 0.).unwrap() - 4.).abs() < 1e-9);
        // The cap at the top.
        assert!((s.hit(&Vector3::new(0., 20., 0.), &Vector3::new(0., -1., 0.), 0.).unwrap() - 9.).abs() < 1e-9);
        assert!(s.hit(&Vector3::new(-5., 5., 3.), &rd, 0.).is_none());
        let c = Channel::new(bolt(1), 0.3);
        let ray = Ray { ro: Vector3::new(-500., 3000., 0.), rd: Vector3::new(1., 0., 0.) };
        // Every hit found is on the surface of the segment it reports.
        for k in 0..200 {
            let r = Ray { ro: ray.ro + Vector3::new(0., (k as f64 - 100.) * 13., (k % 7) as f64 * 0.05), rd: ray.rd };
            if let Some(h) = c.intersects(&r) {
                let s = &c.segments[h.face as usize];
                let (q, _) = s.nearest_on_axis(&h.point);
                assert!(((h.point - q).norm() - s.radius).abs() < 1e-6);
                assert!(c.segment_at(&h.point).is_some());
            }
        }
    }

    #[test]
    fn test_sampling_density_matches_pdf_from() {
        let c = Channel::new(bolt(2), 0.3);
        let from = Vector3::new(800., 2500., -300.);
        let mut rng = Pcg32::new(9, 1);
        let mut checked = 0;
        for _ in 0..500 {
            let u = [rng.next_f64(), rng.next_f64(), rng.next_f64()];
            if let Some((y, n, pdf)) = c.sample_from(&from, u) {
                let again = c.pdf_from(&from, &y, &n);
                // Joints can be claimed by a neighbouring segment; most points must agree.
                if (again - pdf).abs() < 1e-6 * pdf {
                    checked += 1;
                }
            }
        }
        assert!(checked > 400, "{}", checked);
    }

    /// The estimate of the light's power from the surface, ∫ L cos dA dω = π L A, matches the
    /// flux put into it.
    #[test]
    fn test_luminance_from_flux() {
        let o = serde_json::json!({"type": "lightning", "luminous_energy": 1000., "strokes": 1, "shutter": 1., "radius": 0.5, "temperature": 6500.});
        let obj = parse_lightning(&o);
        let m = obj.medium.material_at(Vector3::zeros());
        let n = Vector3::new(1., 0., 0.);
        let l = crate::color::luminance(&m.emitted_at(&n, &n, Some(Vector2::new(1., 0.))));
        let flux = PI * l * 2. * PI * 0.5 * SUN_ILLUMINANCE;
        assert!((flux - 1000.).abs() < 1., "{}", flux);
    }

    /// In a scene, the flash is an area light, as bright as the sun's intensity says, and
    /// widened to the camera's pixels.
    #[test]
    fn test_in_a_scene_it_is_a_light_on_the_suns_scale() {
        let scene = |intensity: f64| crate::scenefile::SceneFile::from_string(format!(r#"{{
            "width": 100, "height": 100,
            "camera": {{ "location": [0, 10, -20000], "lookat": [0, 3000, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "sun": {{ "intensity": {}, "direction": [0, -1, 0] }},
            "objects": [{{ "type": "lightning", "start": [0, 3000, 0], "seed": 1, "cloud_length": 0 }}]
        }}"#, intensity));
        let (a, b) = (scene(1.), scene(3.));
        assert_eq!(a.area_lights.len(), 1);
        let n = Vector3::new(1., 0., 0.);
        let glow = |s: &Scene| crate::color::luminance(&s.area_lights[0].medium.material_at(Vector3::zeros()).emitted(&n, &n));
        assert!((glow(&b) / glow(&a) - 3.).abs() < 1e-9);
        // From the start to the ground, and some 27 m wide from 20 km away (half a pixel of
        // tan(0.5) / 100 radians), far wider than the channel.
        let bounds = a.area_lights[0].geometry.bounds();
        assert!(bounds.max.y > 3000. && bounds.min.y < 0.);
        assert!(bounds.max.x - bounds.min.x > 50.);
    }
}
