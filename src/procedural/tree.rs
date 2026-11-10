/// Procedural trees, after Jason Weber and Joseph Penn, "Creation and Rendering of Realistic
/// Trees", SIGGRAPH '95 Proceedings, 119-128, 1995, doi:10.1145/218380.218427.
///
/// Weber and Penn describe a tree by a few dozen parameters that a botanist or artist can read
/// off a real tree - how many branches leave the trunk, at what angle, how long they are
/// relative to their parent, how they curve - rather than by rewriting rules, as L-systems do
/// (Przemyslaw Prusinkiewicz and Aristid Lindenmayer, "The Algorithmic Beauty of Plants",
/// Springer, 1990), or by competition for space, as in Adam Runions, Brendan Lane and
/// Przemyslaw Prusinkiewicz, "Modeling Trees with a Space Colonization Algorithm", Eurographics
/// Workshop on Natural Phenomena, 2007. That makes it the natural choice for generating many
/// species: a species is a parameter set (see `species`), and a seed picks one individual of it.
///
/// A tree is a hierarchy of stems: the trunk is level 0, its branches level 1 and so on, up to
/// `levels - 1`; leaves grow on the last level. Each stem is a chain of `curve_res` straight
/// segments, each turned from the last about the stem's local x axis by the curve angles. Every
/// parameter "x" with a companion "x_v" is varied uniformly in [x - x_v, x + x_v] per stem.
/// The numbered sections below refer to the paper.
///
/// Departures from the paper:
/// - The paper is z-up; here the world and each stem's local frame are y-up, the stem growing
///   along its local +y.
/// - Pruning (`PruneRatio`, shape 8, and `ZScale`) is not implemented, nor are helical stems
///   (negative `CurveV`, whose magnitude is used) or fanned leaves (negative `Leaves`).
/// - Attraction up (4.7) rotates each segment towards the vertical by
///   `attraction_up * declination / curve_res`, clamped so it doesn't pass the vertical; the
///   paper weights the same rotation about the stem's local x axis by the cosine of its
///   orientation.
/// - When a stem splits (4.3), each clone's radius is divided by sqrt(clones), so the clones'
///   cross-sections add up to the parent's: "Leonardo's rule", see Christophe Eloy,
///   "Leonardo's Rule, Self-Similarity, and Wind-Induced Stresses in Trees", Physical Review
///   Letters 107, 258101, 2011, doi:10.1103/PhysRevLett.107.258101. The paper keeps the radius.
/// - Negative `rotate` alternates children between two opposite sides (distichous, as in an
///   elm's twigs or a palm's leaflets), each varied by `rotate_v`; its magnitude is unused.
/// - Leaves are bent towards the sky by `leaf_bend` (0 to 1), after Wolfram Diestel's Arbaro,
///   a Java implementation of the paper (<https://sourceforge.net/projects/arbaro/>); the paper
///   (4.8) mentions orienting leaves towards the light without giving a rule.
use crate::na::{Vector3, UnitQuaternion, Unit};
use crate::geometry::Pcg32;
use crate::shapes::triangle::{Triangle, SmoothTriangle};
use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use crate::octree::Octree;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use serde_json::Value;
use std::f64::consts::PI;
use std::sync::Arc;

/// The parameters of one level of stems (the paper's `nDownAngle`, `nRotate` etc. for level n).
/// Angles are in degrees.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct StemLevel {
    /// Angle between a child stem and its parent (4.4). With a negative `down_angle_v` it
    /// instead varies with position along the parent, from `down_angle + down_angle_v` at the
    /// base to `down_angle - down_angle_v` at the tip.
    pub down_angle: f64,
    pub down_angle_v: f64,
    /// Angle about the parent between one child and the next (4.4). Any negative value
    /// alternates children between opposite sides instead: each turns 180 degrees from the
    /// last, varied by `rotate_v`.
    pub rotate: f64,
    pub rotate_v: f64,
    /// Maximum number of child stems of this level on each parent (4.4). For leaves, unused.
    pub branches: f64,
    /// Length relative to the parent (for the trunk, to the tree's scale).
    pub length: f64,
    pub length_v: f64,
    /// 0 to 1: from a cylinder to a cone; 1 to 2: towards a rounded tip; 2 to 3: periodic,
    /// like a string of beads (4.2).
    pub taper: f64,
    /// Number of clones each segment splits into, on average (4.3).
    pub seg_splits: f64,
    pub split_angle: f64,
    pub split_angle_v: f64,
    /// Number of segments.
    pub curve_res: usize,
    /// Total curvature of the stem, or of its first half when `curve_back` is non-zero (4.2).
    pub curve: f64,
    /// Curvature of the second half, for S-shaped stems.
    pub curve_back: f64,
    pub curve_v: f64,
}

impl Default for StemLevel {
    fn default() -> StemLevel {
        StemLevel {
            down_angle: 45., down_angle_v: 10., rotate: 140., rotate_v: 0.,
            branches: 0., length: 0.5, length_v: 0., taper: 1.,
            seg_splits: 0., split_angle: 0., split_angle_v: 0.,
            curve_res: 3, curve: 0., curve_back: 0., curve_v: 0.,
        }
    }
}

/// The whole-tree parameters of the paper's Table 1, named in snake case. `stems[n]` holds the
/// paper's per-level parameters "nX"; the level after the last stems' gives the leaves' angles.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct TreeParams {
    /// Envelope of first-level branch lengths along the trunk, see `shape_ratio`.
    pub shape: u32,
    /// Fraction of the trunk, from the ground, without branches.
    pub base_size: f64,
    /// Height of the tree, in scene units.
    pub scale: f64,
    pub scale_v: f64,
    /// Levels of stems, including the trunk: 1 to 4.
    pub levels: usize,
    /// Trunk radius over trunk length.
    pub ratio: f64,
    /// How quickly radii fall with level: child radius = parent radius * (child length /
    /// parent length) ^ ratio_power.
    pub ratio_power: f64,
    /// Number of lobes around the trunk (as in an oak's buttressed trunk) and their depth.
    pub lobes: u32,
    pub lobe_depth: f64,
    /// Exponential widening of the trunk's base.
    pub flare: f64,
    /// Scale of the trunk's radius (the paper's `0Scale`).
    pub trunk_scale: f64,
    pub trunk_scale_v: f64,
    /// Number of clones the trunk splits into at its base (`0BaseSplits`).
    pub base_splits: usize,
    /// Leaves per stem of the last level.
    pub leaves: f64,
    /// Leaf length, in scene units, and its width as a fraction of that.
    pub leaf_scale: f64,
    pub leaf_scale_x: f64,
    /// Multiplies the number of leaves; the leaves shrink to keep their total area.
    pub leaf_quality: f64,
    /// 0 to 1: how far leaves turn to face the sky.
    pub leaf_bend: f64,
    /// Tendency of stems of level 2 and above to turn upwards (or, negative, to droop).
    pub attraction_up: f64,
    pub stems: Vec<StemLevel>,
}

impl Default for TreeParams {
    fn default() -> TreeParams {
        TreeParams {
            shape: 0, base_size: 0.3, scale: 10., scale_v: 0., levels: 3,
            ratio: 0.015, ratio_power: 1.2, lobes: 0, lobe_depth: 0., flare: 0.5,
            trunk_scale: 1., trunk_scale_v: 0., base_splits: 0,
            leaves: 25., leaf_scale: 0.15, leaf_scale_x: 0.6, leaf_quality: 1., leaf_bend: 0.3,
            attraction_up: 0.5,
            stems: vec![StemLevel::default(); 4],
        }
    }
}

impl TreeParams {
    /// The parameters for stems (or leaves) of `level`; levels past the last listed reuse it.
    fn level(&self, level: usize) -> &StemLevel {
        &self.stems[level.min(self.stems.len() - 1)]
    }
}

/// The paper's ShapeRatio (4.4): the length of a first-level branch, relative to the longest,
/// at `ratio` = (distance from the trunk's tip) / (length of the branched part of the trunk), so
/// 0 at the top of the tree and 1 at the lowest branch. The shapes are 0 conical, 1 spherical,
/// 2 hemispherical, 3 cylindrical, 4 tapered cylindrical, 5 flame, 6 inverse conical and 7 tend
/// flame. 8, the pruning envelope, isn't implemented and is treated as cylindrical.
pub fn shape_ratio(shape: u32, ratio: f64) -> f64 {
    let ratio = ratio.clamp(0., 1.);
    match shape {
        0 => 0.2 + 0.8 * ratio,
        1 => 0.2 + 0.8 * (PI * ratio).sin(),
        2 => 0.2 + 0.8 * (0.5 * PI * ratio).sin(),
        4 => 0.5 + 0.5 * ratio,
        5 => if ratio <= 0.7 { ratio / 0.7 } else { (1. - ratio) / 0.3 },
        6 => 1. - 0.8 * ratio,
        7 => if ratio <= 0.7 { 0.5 + 0.5 * ratio / 0.7 } else { 0.5 + 0.5 * (1. - ratio) / 0.3 },
        _ => 1.,
    }
}

/// The radius of a stem of base radius `radius`, length `length` and the given taper at
/// fraction `z` of the way along it (4.2).
pub fn taper_radius(taper: f64, radius: f64, length: f64, z: f64) -> f64 {
    let unit_taper = if taper <= 1. { taper } else if taper <= 2. { 2. - taper } else { 0. };
    let taper_z = radius * (1. - unit_taper * z);
    if taper < 1. {
        return taper_z;
    }
    if taper_z <= 0. {
        return 0.;
    }
    let z2 = (1. - z) * length;
    let depth = if taper < 2. || z2 < taper_z { 1. } else { taper - 2. };
    let z3 = if taper < 2. { z2 } else { (z2 - 2. * taper_z * (z2 / (2. * taper_z) + 0.5).floor()).abs() };
    if taper < 2. && z3 >= taper_z {
        taper_z
    } else {
        (1. - depth) * taper_z + depth * (taper_z * taper_z - (z3 - taper_z).powi(2)).max(0.).sqrt()
    }
}

/// The trunk's flare (4.2): its radius at fraction `z` up it is multiplied by this.
fn flare(flare: f64, z: f64) -> f64 {
    let y = (1. - 8. * z).max(0.);
    flare * (100f64.powf(y) - 1.) / 100. + 1.
}

fn rot_x(degrees: f64) -> UnitQuaternion<f64> {
    UnitQuaternion::from_axis_angle(&Vector3::x_axis(), degrees.to_radians())
}

fn rot_y(degrees: f64) -> UnitQuaternion<f64> {
    UnitQuaternion::from_axis_angle(&Vector3::y_axis(), degrees.to_radians())
}

/// Angle in degrees between the direction a frame grows along and the vertical.
fn declination(frame: &UnitQuaternion<f64>) -> f64 {
    (frame * Vector3::y()).y.clamp(-1., 1.).acos().to_degrees()
}

/// Turn `frame` by `degrees` towards the vertical (or away, if negative), about the horizontal
/// axis perpendicular to its direction, stopping at the vertical.
fn towards_vertical(frame: UnitQuaternion<f64>, degrees: f64) -> UnitQuaternion<f64> {
    let dir = frame * Vector3::y();
    let decl = declination(&frame);
    let degrees = degrees.clamp(-(180. - decl), decl);
    match Unit::try_new(dir.cross(&Vector3::y()), 1e-9) {
        Some(axis) => UnitQuaternion::from_axis_angle(&axis, degrees.to_radians()) * frame,
        None => frame,
    }
}

/// The triangles of a generated tree.
pub struct TreeMesh {
    pub bark: Vec<Arc<SmoothTriangle>>,
    pub leaves: Vec<Arc<Triangle>>,
}

/// One stem being grown: what its children and its mesh need to know.
struct Stem {
    level: usize,
    length: f64,
    radius: f64,
    /// Length of the trunk without branches; 0 for other stems.
    base_length: f64,
    /// This stem's maximum length relative to its parent, as drawn for it.
    length_max: f64,
    /// Number of child stems or leaves along the whole stem (before splits share them out).
    children: f64,
    leaves: bool,
}

/// Where the next split, child or leaf falls: fractional parts carried along a stem, as error
/// diffusion (4.3) rather than rounding each segment's count independently.
#[derive(Clone, Copy)]
struct Carry {
    splits: f64,
    children: f64,
    rotate: f64,
}

struct Generator<'a> {
    p: &'a TreeParams,
    rng: Pcg32,
    mesh: TreeMesh,
}

impl Generator<'_> {
    fn uniform(&mut self) -> f64 {
        self.rng.next_f64()
    }

    /// `x` varied uniformly by up to `v` either way.
    fn vary(&mut self, x: f64, v: f64) -> f64 {
        x + (self.uniform() * 2. - 1.) * v
    }

    /// Grow a stem of `level` from `origin` along `frame`'s +y, and everything on it.
    /// `parent` is the parent's length and this stem's offset along it, None for the trunk.
    #[allow(clippy::too_many_arguments)]
    fn stem(&mut self, level: usize, origin: Vector3<f64>, frame: UnitQuaternion<f64>,
            length: f64, radius: f64, length_max: f64, parent: Option<(f64, f64)>) {
        let p = self.p;
        let leaves = level + 1 >= p.levels;
        let children = if leaves {
            // 4.8: more leaves on branches that leave their parent further out.
            let r = parent.map_or(1., |(parent_length, offset)| offset / parent_length);
            p.leaves.abs() * shape_ratio(4, r) * p.leaf_quality
        } else if level == 0 {
            p.level(1).branches
        } else {
            // 4.4: longer stems carry more children.
            let parent_length = parent.unwrap().0;
            p.level(level + 1).branches * (0.2 + 0.8 * (length / parent_length) / length_max)
        };
        let s = Stem {
            level, length, radius, length_max, children, leaves,
            base_length: if level == 0 { p.base_size * length } else { 0. },
        };
        let carry = Carry { splits: 0., children: self.uniform(), rotate: self.uniform() * 360. };
        self.segments(&s, 0, origin, frame, 1., carry, 0., false);
    }

    /// Grow `s` from segment `first` on, as one tube: splitting off clones (which grow their own
    /// tubes) and adding children as it goes. `share` is the fraction of the stem's children
    /// and cross-section this clone carries.
    #[allow(clippy::too_many_arguments)]
    fn segments(&mut self, s: &Stem, first: usize, origin: Vector3<f64>, mut frame: UnitQuaternion<f64>,
                mut share: f64, mut carry: Carry, mut curve_correction: f64, mut split_done: bool) {
        let p = self.p;
        let lp = p.level(s.level);
        let n = lp.curve_res.max(1);
        let seg_length = s.length / n as f64;
        // Each segment's start, frame and share; then the end point.
        let mut segs: Vec<(Vector3<f64>, UnitQuaternion<f64>, f64)> = vec![];
        let mut pos = origin;

        for i in first .. n {
            // 4.3: splits, at the base of the trunk or along any stem.
            let clones = if split_done {
                split_done = false;
                0
            } else if i == 0 && s.level == 0 {
                p.base_splits
            } else if i > 0 {
                let want = lp.seg_splits + carry.splits;
                let k = want.round().max(0.);
                carry.splits = want - k;
                k as usize
            } else {
                0
            };
            if clones > 0 {
                let angle = (self.vary(lp.split_angle, lp.split_angle_v) - declination(&frame)).max(0.);
                let spread = 360. / (clones + 1) as f64;
                let jitter = self.uniform() * 360.;
                share /= (clones + 1) as f64;
                // The remaining segments turn back by the split angle, so the clones diverge but
                // keep the stem's overall direction, as the paper's trunk splits do.
                let back = angle / (n - i) as f64;
                for c in 1 ..= clones {
                    let f = frame * rot_y(jitter + spread * c as f64) * rot_x(angle);
                    let mut clone_carry = carry;
                    clone_carry.rotate = self.uniform() * 360.;
                    self.segments(s, i, pos, f, share, clone_carry, curve_correction - back, true);
                }
                frame = frame * rot_y(jitter) * rot_x(angle);
                curve_correction -= back;
            }

            let z0 = i as f64 / n as f64;
            let dir = frame * Vector3::y();
            self.children(s, &frame, pos, dir, z0, share, &mut carry);
            segs.push((pos, frame, share));
            pos += dir * seg_length;

            // 4.2: curvature for the next segment.
            let curve = if lp.curve_back == 0. {
                lp.curve / n as f64
            } else if i < n / 2 {
                lp.curve / (n as f64 / 2.)
            } else {
                lp.curve_back / (n as f64 / 2.)
            };
            let curve = self.vary(curve, lp.curve_v.abs() / n as f64) + curve_correction;
            frame *= rot_x(curve);
            if s.level >= 2 {
                // 4.7: attraction up.
                frame = towards_vertical(frame, p.attraction_up * declination(&frame) / n as f64);
            }
        }
        segs.push((pos, frame, share));
        self.tube(s, first, &segs);
    }

    /// Add the child stems or leaves growing from the segment of `s` starting at `pos`, `z0` of
    /// the way along it.
    #[allow(clippy::too_many_arguments)]
    fn children(&mut self, s: &Stem, frame: &UnitQuaternion<f64>, pos: Vector3<f64>, dir: Vector3<f64>,
                z0: f64, share: f64, carry: &mut Carry) {
        let p = self.p;
        if s.children <= 0. || (s.leaves && p.leaf_scale <= 0.) {
            return;
        }
        let n = p.level(s.level).curve_res.max(1) as f64;
        let z1 = z0 + 1. / n;
        // Children grow above the trunk's base, anywhere on other stems.
        let zmin = s.base_length / s.length;
        let (a, b) = (z0.max(zmin), z1);
        if b <= a {
            return;
        }
        let want = s.children * share * (b - a) / (1. - zmin) + carry.children;
        let count = want.floor();
        carry.children = want - count;
        let count = count as usize;
        let cp = p.level(s.level + 1);
        for k in 0 .. count {
            let z = a + (b - a) * (k as f64 + self.uniform()) / count as f64;
            let offset = z * s.length;
            let at = pos + dir * (z - z0) * s.length;

            // 4.4: angles.
            let down = if cp.down_angle_v >= 0. {
                self.vary(cp.down_angle, cp.down_angle_v)
            } else {
                let r = (s.length - offset) / (s.length - s.base_length);
                cp.down_angle + cp.down_angle_v * (1. - 2. * shape_ratio(0, r))
            };
            carry.rotate += if cp.rotate >= 0. {
                self.vary(cp.rotate, cp.rotate_v)
            } else {
                self.vary(180., cp.rotate_v)
            };
            let f = frame * rot_y(carry.rotate) * rot_x(down);

            if s.leaves {
                self.leaf(at, f);
                continue;
            }

            // 4.4: lengths and radii.
            let length_max = self.vary(cp.length, cp.length_v);
            let length = if s.level == 0 {
                let r = (s.length - offset) / (s.length - s.base_length);
                s.length * length_max * shape_ratio(p.shape, r)
            } else {
                length_max * (s.length - 0.6 * offset)
            };
            if length <= 0. {
                continue;
            }
            let parent_radius = taper_radius(p.level(s.level).taper, s.radius, s.length, z) * share.sqrt();
            let radius = (s.radius * (length / s.length).powf(p.ratio_power)).min(parent_radius);
            if radius <= 0. {
                continue;
            }
            self.stem(s.level + 1, at, f, length, radius, length_max, Some((s.length, offset)));
        }
    }

    /// A leaf with its stalk at `at`, in the plane of `frame`'s x and y axes, growing along y.
    fn leaf(&mut self, at: Vector3<f64>, frame: UnitQuaternion<f64>) {
        let p = self.p;
        // Face the sky: first flip a leaf facing down over (leaves have no back), then turn it
        // `leaf_bend` of the way to face straight up.
        let frame = if (frame * Vector3::z()).y < 0. { frame * rot_y(180.) } else { frame };
        let frame = match UnitQuaternion::rotation_between(&(frame * Vector3::z()), &Vector3::y()) {
            Some(r) => UnitQuaternion::identity().slerp(&r, p.leaf_bend.clamp(0., 1.)) * frame,
            None => frame,
        };
        let length = p.leaf_scale / p.leaf_quality.max(1e-6).sqrt();
        let half_width = 0.5 * length * p.leaf_scale_x;
        // A pointed oval: stalk, two pairs of edge points, and tip.
        let pt = |x: f64, y: f64| at + frame * Vector3::new(x * half_width, y * length, 0.);
        let base = pt(0., 0.);
        let (l1, r1) = (pt(-0.9, 0.3), pt(0.9, 0.3));
        let (l2, r2) = (pt(-0.75, 0.65), pt(0.75, 0.65));
        let tip = pt(0., 1.);
        for [a, b, c] in [[base, r1, l1], [l1, r1, r2], [l1, r2, l2], [l2, r2, tip]] {
            self.mesh.leaves.push(Arc::new(Triangle::new(a, b, c)));
        }
    }

    /// A tube along the segments of `s` from segment `first`: each a start point, frame and the
    /// share of the stem's cross-section this clone has, with the end point last.
    fn tube(&mut self, s: &Stem, first: usize, segs: &[(Vector3<f64>, UnitQuaternion<f64>, f64)]) {
        let p = self.p;
        let lp = p.level(s.level);
        let n = lp.curve_res.max(1);
        let sides = [16, 10, 6, 4][s.level.min(3)];
        // Rings: the trunk's flared base and stems with rounded tips need more of them.
        let mut rings: Vec<(Vector3<f64>, UnitQuaternion<f64>, f64, f64)> = vec![];
        for (j, w) in segs.windows(2).enumerate() {
            let ((a, fa, share), (b, _, _)) = (w[0], w[1]);
            let z0 = (first + j) as f64 / n as f64;
            // At a joint, the ring lies halfway between the two segments' directions.
            let joint = if j == 0 { fa } else { segs[j - 1].1.slerp(&fa, 0.5) };
            let sub = if s.level == 0 && z0 < 0.125 { 8 } else if lp.taper > 1. { 3 } else { 1 };
            for k in 0 .. sub {
                let t = k as f64 / sub as f64;
                rings.push((a + (b - a) * t, if k == 0 { joint } else { fa }, z0 + t / n as f64, share));
            }
        }
        let (end, end_frame, end_share) = segs[segs.len() - 1];
        rings.push((end, end_frame, 1., end_share));

        let ring_points: Vec<Vec<(Vector3<f64>, Vector3<f64>)>> = rings.iter().map(|(c, f, z, share)| {
            let mut r = taper_radius(lp.taper, s.radius * share.sqrt(), s.length, *z);
            if s.level == 0 {
                r *= flare(p.flare, *z);
            }
            (0 .. sides).map(|i| {
                let theta = 2. * PI * i as f64 / sides as f64;
                let out = f * Vector3::new(theta.cos(), 0., theta.sin());
                let lobed = if s.level == 0 { 1. + p.lobe_depth * (p.lobes as f64 * theta).sin() } else { 1. };
                (c + out * r * lobed, out)
            }).collect()
        }).collect();

        for w in ring_points.windows(2) {
            for i in 0 .. sides {
                let j = (i + 1) % sides;
                let (a, b, c, d) = (w[0][i], w[0][j], w[1][j], w[1][i]);
                for [x, y, z] in [[a, b, c], [a, c, d]] {
                    if (y.0 - x.0).cross(&(z.0 - x.0)).norm() > 1e-14 {
                        self.mesh.bark.push(Arc::new(SmoothTriangle::new(x.0, y.0, z.0, x.1, y.1, z.1)));
                    }
                }
            }
        }
    }
}

/// Generate the tree `p` describes, chosen by `seed`, with its base at `origin`.
pub fn generate(p: &TreeParams, seed: u64, origin: Vector3<f64>) -> TreeMesh {
    let mut g = Generator { p, rng: Pcg32::new(seed, 0x7ee), mesh: TreeMesh { bark: vec![], leaves: vec![] } };
    let scale = g.vary(p.scale, p.scale_v).max(0.);
    let trunk = p.level(0);
    let length_max = g.vary(trunk.length, trunk.length_v);
    let length = length_max * scale;
    let radius = length * p.ratio * g.vary(p.trunk_scale, p.trunk_scale_v);
    let frame = rot_y(g.uniform() * 360.);
    if length > 0. && radius > 0. {
        g.stem(0, origin, frame, length, radius, length_max, None);
    }
    g.mesh
}

/// Merge `overrides` into `base`: objects key by key, arrays element by element, anything else
/// replaced.
fn merge(base: &mut Value, overrides: &Value) {
    match (base, overrides) {
        (Value::Object(b), Value::Object(o)) => {
            for (k, v) in o {
                merge(b.entry(k.clone()).or_insert(Value::Null), v);
            }
        },
        (Value::Array(b), Value::Array(o)) => {
            for (i, v) in o.iter().enumerate() {
                if i < b.len() { merge(&mut b[i], v) } else { b.push(v.clone()) }
            }
        },
        (b, o) => *b = o.clone(),
    }
}

/// The parameters for a tree: a species' (see `species`, default "quaking-aspen") with
/// `overrides` merged in, as a JSON object of `TreeParams` fields (`stems` an array of per-level
/// objects, merged by position).
pub fn params(species_name: &str, overrides: &Value) -> TreeParams {
    let base = species(species_name)
        .unwrap_or_else(|| panic!("Unknown tree species {:?}; known species are {}", species_name, SPECIES.join(", ")));
    let mut v = serde_json::to_value(base).unwrap();
    if !overrides.is_null() {
        merge(&mut v, overrides);
    }
    let p: TreeParams = serde_json::from_value(v)
        .unwrap_or_else(|e| panic!("Bad tree parameters {}: {}", overrides, e));
    assert!(!p.stems.is_empty(), "A tree needs parameters for at least one level of stems");
    p
}

pub const SPECIES: [&str; 8] = [
    "quaking-aspen", "black-tupelo", "california-black-oak", "weeping-willow",
    "lombardy-poplar", "balsam-fir", "palm", "shrub",
];

/// A `StemLevel` without splits, its arguments in the order of the paper's Table 1.
#[allow(clippy::too_many_arguments)]
fn level(down_angle: f64, down_angle_v: f64, rotate: f64, rotate_v: f64, branches: f64,
         length: f64, length_v: f64, taper: f64, curve_res: usize, curve: f64, curve_back: f64,
         curve_v: f64) -> StemLevel {
    StemLevel {
        down_angle, down_angle_v, rotate, rotate_v, branches, length, length_v, taper,
        curve_res, curve, curve_back, curve_v, ..StemLevel::default()
    }
}

fn split(l: StemLevel, seg_splits: f64, split_angle: f64, split_angle_v: f64) -> StemLevel {
    StemLevel { seg_splits, split_angle, split_angle_v, ..l }
}

/// Parameters for a named species.
///
/// The first four are the species of the paper's figures, starting from its parameter tables
/// and tuned by eye in this renderer, partly for this implementation's departures (see the
/// module documentation): the willow carries its leaves on its hanging whips, with three levels
/// of stems rather than four, and the oak has fewer twigs. The others are written for this
/// renderer in
/// the same terms, from the trees' published descriptions: the Lombardy poplar's narrow
/// columnar crown of steep, upturned branches; the balsam fir's conical whorls of near-level
/// branches with flat sprays of short needles; a palm's single unbranched trunk crowned by
/// arching fronds with leaflets in two opposite rows, spiralling at the golden angle of
/// 137.5 degrees (Prusinkiewicz and Lindenmayer, ch. 4); and a generic multi-stemmed shrub.
pub fn species(name: &str) -> Option<TreeParams> {
    let d = TreeParams::default();
    let p = match name {
        "quaking-aspen" => TreeParams {
            shape: 7, base_size: 0.4, scale: 13., scale_v: 3., levels: 3,
            ratio: 0.015, ratio_power: 1.2, lobes: 5, lobe_depth: 0.07, flare: 0.6,
            leaves: 25., leaf_scale: 0.17, leaf_scale_x: 1., attraction_up: 0.5,
            stems: vec![
                level(0., 0., 0., 0., 0., 1., 0., 1., 3, 0., 0., 20.),
                level(60., -50., 140., 0., 50., 0.3, 0., 1., 5, -40., 0., 50.),
                level(45., 10., 140., 0., 30., 0.6, 0., 1., 3, -40., 0., 75.),
                level(45., 10., 77., 0., 10., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "black-tupelo" => TreeParams {
            shape: 4, base_size: 0.2, scale: 23., scale_v: 5., levels: 4,
            ratio: 0.015, ratio_power: 1.3, lobes: 3, lobe_depth: 0.1, flare: 1.,
            leaves: 6., leaf_scale: 0.3, leaf_scale_x: 0.5, attraction_up: 0.5,
            stems: vec![
                level(0., 0., 0., 0., 0., 1., 0., 1.1, 10, 0., 0., 40.),
                level(60., -40., 140., 0., 50., 0.3, 0.05, 1., 10, 0., 0., 90.),
                level(30., 10., 140., 0., 25., 0.6, 0.1, 1., 10, -10., 0., 150.),
                level(45., 10., 140., 0., 12., 0.4, 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "california-black-oak" => TreeParams {
            shape: 2, base_size: 0.05, scale: 13., scale_v: 3., levels: 3,
            ratio: 0.018, ratio_power: 1.3, lobes: 5, lobe_depth: 0.1, flare: 1.2,
            base_splits: 2, leaves: 40., leaf_scale: 0.14, leaf_scale_x: 0.66, attraction_up: 0.8,
            stems: vec![
                split(level(0., 0., 0., 0., 0., 1., 0., 0.95, 8, 0., 0., 90.), 0.4, 35., 10.),
                split(level(30., -30., 80., 0., 40., 0.8, 0.1, 1., 10, 40., -70., 150.), 0.2, 10., 10.),
                split(level(45., 10., 140., 0., 50., 0.25, 0.05, 1., 3, 0., 0., 30.), 0.1, 10., 10.),
                level(45., 10., 140., 0., 0., 0.4, 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "weeping-willow" => TreeParams {
            shape: 3, base_size: 0.05, scale: 15., scale_v: 5., levels: 3,
            ratio: 0.03, ratio_power: 2., lobes: 9, lobe_depth: 0.03, flare: 0.75,
            leaves: 250., leaf_scale: 0.25, leaf_scale_x: 0.3, leaf_bend: 0., attraction_up: -3.,
            stems: vec![
                split(level(0., 0., 0., 0., 0., 0.8, 0., 1., 8, 0., 0., 20.), 0.1, 3., 0.),
                split(level(20., 10., -180., 30., 25., 0.5, 0.1, 1., 16, 40., 0., 90.), 0.2, 30., 10.),
                level(30., 10., -180., 30., 14., 1.1, 0.2, 1., 12, 0., 0., 0.),
                level(25., 10., 140., 20., 0., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "lombardy-poplar" => TreeParams {
            shape: 5, base_size: 0.08, scale: 22., scale_v: 3., levels: 3,
            ratio: 0.015, ratio_power: 1.2, lobes: 4, lobe_depth: 0.05, flare: 0.7,
            leaves: 35., leaf_scale: 0.1, leaf_scale_x: 1., attraction_up: 1.2,
            stems: vec![
                level(0., 0., 0., 0., 0., 1., 0., 1., 6, 0., 0., 8.),
                level(22., 8., 140., 0., 70., 0.25, 0.05, 1., 5, -25., 0., 20.),
                level(35., 10., 140., 0., 20., 0.45, 0.05, 1., 3, -10., 0., 40.),
                level(40., 10., 140., 0., 0., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "balsam-fir" => TreeParams {
            shape: 0, base_size: 0.12, scale: 14., scale_v: 2., levels: 3,
            ratio: 0.02, ratio_power: 1.5, lobes: 0, lobe_depth: 0., flare: 0.4,
            leaves: 50., leaf_scale: 0.2, leaf_scale_x: 0.35, leaf_bend: 0., attraction_up: 0.,
            stems: vec![
                level(0., 0., 0., 0., 0., 1., 0., 1., 6, 0., 0., 4.),
                level(80., 10., 137.5, 10., 90., 0.3, 0.03, 1., 4, 15., 0., 10.),
                level(60., 10., -180., 15., 20., 0.35, 0.1, 1., 2, 0., 0., 10.),
                level(55., 10., 137.5, 0., 0., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "palm" => TreeParams {
            shape: 4, base_size: 0.95, scale: 10., scale_v: 2., levels: 2,
            ratio: 0.025, ratio_power: 2., lobes: 0, lobe_depth: 0., flare: 0.3,
            leaves: 70., leaf_scale: 0.6, leaf_scale_x: 0.12, leaf_bend: 0.1, attraction_up: 0.,
            stems: vec![
                level(0., 0., 0., 0., 0., 1., 0., 0.5, 10, 0., 0., 15.),
                level(60., -40., 137.5, 5., 24., 0.4, 0.05, 1., 10, 70., 0., 20.),
                level(70., 10., -180., 10., 0., 0., 0., 1., 1, 0., 0., 0.),
                level(70., 10., -180., 10., 0., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        "shrub" => TreeParams {
            shape: 1, base_size: 0., scale: 2., scale_v: 0.5, levels: 3,
            ratio: 0.02, ratio_power: 1.3, lobes: 0, lobe_depth: 0., flare: 0.,
            base_splits: 4, leaves: 30., leaf_scale: 0.07, leaf_scale_x: 0.6, attraction_up: 0.6,
            stems: vec![
                split(level(0., 0., 0., 0., 0., 1., 0.1, 1., 5, 0., 0., 30.), 0.3, 25., 10.),
                level(50., 10., 140., 20., 25., 0.5, 0.1, 1., 4, -10., 0., 40.),
                level(45., 10., 140., 20., 12., 0.5, 0.1, 1., 2, 0., 0., 30.),
                level(45., 10., 140., 0., 0., 0., 0., 1., 1, 0., 0., 0.),
            ],
            ..d
        },
        _ => return None,
    };
    Some(p)
}

/// Triangles to trace, behind an octree.
pub struct TriangleSet<T: Geometry> {
    tree: Octree<T>,
    bounds: BBox,
    count: usize,
}

impl<T: Geometry> TriangleSet<T> {
    pub fn new(items: &[Arc<T>]) -> TriangleSet<T> {
        let mut bounds = BBox::empty();
        for t in items {
            bounds = bounds.union(&t.bounds());
        }
        TriangleSet { tree: Octree::new(8, bounds, items), bounds, count: items.len() }
    }
}

impl<T: Geometry + Sync + Send> Geometry for TriangleSet<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.tree.raw_intersection(r, f64::INFINITY, 0f64)
    }

    fn bounds(&self) -> BBox {
        self.bounds
    }

    fn primitives(&self) -> u64 {
        self.count as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_species_grows() {
        for name in SPECIES {
            let m = generate(&params(name, &Value::Null), 1, Vector3::zeros());
            assert!(m.bark.len() > 100, "{} has {} bark triangles", name, m.bark.len());
            assert!(!m.leaves.is_empty(), "{} has no leaves", name);
            for t in &m.bark {
                assert!(t.v0.iter().chain(t.v1.iter()).chain(t.v2.iter()).all(|x| x.is_finite()), "{}", name);
            }
        }
    }

    #[test]
    fn test_seed_chooses_the_tree() {
        let p = params("quaking-aspen", &Value::Null);
        let (a, b, c) = (generate(&p, 3, Vector3::zeros()), generate(&p, 3, Vector3::zeros()), generate(&p, 4, Vector3::zeros()));
        assert_eq!(a.bark.len(), b.bark.len());
        assert_eq!(a.leaves[10].v0, b.leaves[10].v0);
        assert!(a.bark.len() != c.bark.len() || a.leaves[10].v0 != c.leaves[10].v0);
    }

    #[test]
    fn test_overrides_merge_into_species() {
        let p = params("quaking-aspen", &serde_json::json!({ "levels": 2, "stems": [{}, { "branches": 7 }] }));
        assert_eq!(p.levels, 2);
        assert_eq!(p.stems[1].branches, 7.);
        // Untouched fields keep the species' values.
        assert_eq!(p.stems[1].down_angle, 60.);
        assert_eq!(p.shape, 7);
    }

    #[test]
    fn test_trunk_only_tree_is_a_tapered_tube() {
        let mut p = params("quaking-aspen", &Value::Null);
        p.levels = 1;
        p.leaves = 0.;
        p.scale_v = 0.;
        p.flare = 0.;
        p.lobe_depth = 0.;
        p.stems[0].curve_v = 0.;
        let m = generate(&p, 0, Vector3::zeros());
        assert!(m.leaves.is_empty());
        let top = m.bark.iter().map(|t| t.v0.y.max(t.v1.y).max(t.v2.y)).fold(0., f64::max);
        assert!((top - 13.).abs() < 1e-9, "{}", top);
        let widest = m.bark.iter().map(|t| t.v0.xz().norm()).fold(0., f64::max);
        assert!((widest - 13. * 0.015).abs() < 1e-9, "{}", widest);
    }

    #[test]
    fn test_shape_ratios() {
        assert_eq!(shape_ratio(0, 0.), 0.2);
        assert_eq!(shape_ratio(0, 1.), 1.);
        assert_eq!(shape_ratio(3, 0.3), 1.);
        assert!((shape_ratio(5, 0.7) - 1.).abs() < 1e-12);
        assert!((shape_ratio(1, 0.5) - 1.).abs() < 1e-12);
    }

    #[test]
    fn test_taper() {
        assert_eq!(taper_radius(0., 1., 10., 0.5), 1.);
        assert_eq!(taper_radius(1., 1., 10., 0.5), 0.5);
        assert_eq!(taper_radius(1., 1., 10., 1.), 0.);
        // Between 1 and 2 the tip is rounded: full radius until near the end.
        assert_eq!(taper_radius(2., 1., 10., 0.5), 1.);
        assert!(taper_radius(2., 1., 10., 0.99) < 1.);
    }

    #[test]
    fn test_drooping_stems_turn_down() {
        let f = rot_x(60.);
        assert!((declination(&towards_vertical(f, 20.)) - 40.).abs() < 1e-9);
        assert!((declination(&towards_vertical(f, -20.)) - 80.).abs() < 1e-9);
        assert!(declination(&towards_vertical(f, 90.)).abs() < 1e-9);
    }
}
