/// Desert plants that don't branch the way Weber and Penn's trees do (`crate::procedural::tree`):
/// columnar and barrel cacti, prickly pears, agaves and ocotillos. Each is a `PlantForm` other
/// than `Branching`, chosen by a species' `form`, and generated here into the same `TreeMesh`
/// a tree is, so `tree` and `forest` objects place them like any other species: the "bark" is
/// the plant's green body, and the "leaves" its spines (for the ocotillo, its small leaves).
///
/// The shapes are built from swept surfaces: a cross-section carried along a centreline, its
/// frame carried along by parallel transport (rotation-minimising, so the surface doesn't
/// twist; Wenping Wang, Bert Jüttler, Dayue Zheng and Yang Liu, "Computation of Rotation
/// Minimizing Frames", ACM Transactions on Graphics 27(1), 2008, doi:10.1145/1330511.1330513).
///
/// - Saguaro (Carnegiea gigantea): a ribbed column up to 12 m or more, with a domed tip, its
///   arms leaving the trunk about level, turning up at an elbow and rising parallel to it;
///   arms only on mature plants, more of them with age. 12 to 30 ribs, crests lined with
///   areoles from which the spines grow (Benson, "The Cacti of the United States and Canada",
///   Stanford University Press, 1982).
/// - Barrel cactus (Ferocactus): a single squat, deeply ribbed body, often leaning towards the
///   sun (south in the northern hemisphere), with long stout spines.
/// - Prickly pear (Opuntia): chains of flattened, obovate pads (cladodes), each growing new pads
///   from its upper rim, in a different plane; areoles scattered over both faces.
/// - Agave: a rosette of thick, stiff, channelled leaves in a spiral at the golden angle, the
///   outer ones spreading and arching, the inner upright; a terminal spine and teeth along the
///   margins (Gentry, "Agaves of Continental North America", University of Arizona Press, 1982).
/// - Ocotillo (Fouquieria splendens): a fan of long, nearly unbranched, whip-like canes from a
///   short base, with small leaves along them after rain.
/// - Mound: the desert's commonest shrubs (sagebrush, rabbitbrush) are dense, rounded masses
///   of small leaves on a few twisted stems, which branching doesn't capture at a reasonable
///   cost: their leaves are scattered through a lumpy ellipsoid, densest towards its surface
///   (where light reaches), with stems twisting up into it; `dead_branches` of the stems are
///   bare and grey, with their part of the canopy gone.
///
/// Spines stand in for many more than a real cactus has, larger, as conifers' needle tufts do,
/// so they read as a pale halo against the light without millions of triangles.
///
/// The species' parameters (`TreeParams`) are reused with these meanings:
///
/// | field | saguaro, barrel | prickly pear | agave | ocotillo |
/// |---|---|---|---|---|
/// | `scale` (`scale_v`) | height | pad length | leaf length | cane length |
/// | `ratio` | radius / height | pad width / length | leaf width / length | cane radius / length |
/// | `lobes`, `lobe_depth` | ribs, their depth (fraction of the radius) | | | |
/// | `stems[1].branches` | arms (saguaro, most on the tallest) | pads | leaves | canes |
/// | `leaves` | areoles per metre along each rib | areoles per pad | teeth per margin | leaves per cane |
/// | `leaf_scale`, `leaf_scale_x` | spine length, width / length | the same | the same | leaf length, width / length |
/// | `lean` (`lean_v`) | lean from the vertical, degrees | | | spread of the canes from the vertical |
///
/// For a mound: `scale` is its height, `ratio` its width over height, `stems[1].branches` its
/// stems, `leaves` its leaves, `leaf_scale` and `leaf_scale_x` their size, and `wander` how
/// much its stems twist.
/// | `wander` | meander of the stems, fraction of their length | | | the same |
/// | `attraction_up` | | how far new pads turn upwards, 0 to 1 | | |
use crate::na::{Vector3, UnitQuaternion, Unit};
use crate::geometry::Pcg32;
use crate::noise::PerlinNoise;
use crate::shapes::mesh::vertex_normals;
use crate::shapes::triangle::{Triangle, SmoothTriangle};
use crate::shapes::geometry::Tagged;
use crate::material::leaf::LeafState;
use crate::procedural::tree::{PlantForm, TreeMesh, TreeParams};
use std::f64::consts::PI;
use std::sync::Arc;

/// Triangles in each spine tuft or leaf, so a plant's leaves are equal chunks.
const PER_TUFT: usize = 4;

/// A ring of a swept surface: where the centreline is, its frame and how far along it is.
#[derive(Clone, Copy)]
struct Ring {
    centre: Vector3<f64>,
    tangent: Vector3<f64>,
    x: Vector3<f64>,
    z: Vector3<f64>,
    t: f64,
}

impl Ring {
    /// The direction out from the centreline at angle `theta` in the section.
    fn out(&self, theta: f64) -> Vector3<f64> {
        self.x * theta.cos() + self.z * theta.sin()
    }
}

fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Any unit vector perpendicular to `v`.
fn perpendicular(v: &Vector3<f64>) -> Vector3<f64> {
    let a = if v.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    v.cross(&a).normalize()
}

/// Rotate `v` by `degrees` about `axis`.
fn rotate(v: Vector3<f64>, axis: Vector3<f64>, degrees: f64) -> Vector3<f64> {
    match Unit::try_new(axis, 1e-12) {
        Some(a) => UnitQuaternion::from_axis_angle(&a, degrees.to_radians()) * v,
        None => v,
    }
}

/// How far a domed tip of length `dome` has narrowed `s` along a stem of `length`: 1 below the
/// dome, falling as a quarter ellipse to 0 at the tip.
fn dome(s: f64, length: f64, dome: f64) -> f64 {
    let u = ((s - (length - dome)) / dome.max(1e-9)).clamp(0., 1.);
    (1. - u * u).max(0.).sqrt()
}

/// A rib's cross-section: 1 on the crests, `1 - depth` in the grooves, rounded on the crests
/// and V-shaped in the grooves, as a cactus's pleats are.
fn ribbed(theta: f64, ribs: u32, depth: f64) -> f64 {
    if ribs == 0 {
        return 1.;
    }
    1. - depth * (1. - (0.5 * ribs as f64 * theta).sin().abs().sqrt())
}

struct Builder<'a> {
    p: &'a TreeParams,
    rng: Pcg32,
    noise: PerlinNoise,
    body: Vec<Arc<SmoothTriangle>>,
    tufts: Vec<Triangle>,
    /// Each tuft's height, for its exposure.
    tuft_heights: Vec<f64>,
}

impl Builder<'_> {
    fn uniform(&mut self) -> f64 {
        self.rng.next_f64()
    }

    fn vary(&mut self, x: f64, v: f64) -> f64 {
        x + (self.uniform() * 2. - 1.) * v
    }

    /// Smooth noise about zero, mostly within +-0.5.
    fn signed_noise(&self, p: Vector3<f64>) -> f64 {
        2. * self.noise.fbm(p.x, p.y, p.z, 2, 0.5, 2.) - 1.
    }

    /// A centreline from `start`, `steps` steps of `ds`, each heading where `heading(u)` says
    /// (`u` the distance along it so far), meandering by `wander` of its length.
    fn centreline(&mut self, start: Vector3<f64>, steps: usize, ds: f64, wander: f64,
                  heading: &dyn Fn(f64) -> Vector3<f64>) -> Vec<Vector3<f64>> {
        let at = Vector3::new(self.uniform(), self.uniform(), self.uniform()) * 1000.;
        let length = steps as f64 * ds;
        let mut points = vec![start];
        let mut p = start;
        for k in 0 .. steps {
            p += heading((k as f64 + 0.5) * ds).normalize() * ds;
            points.push(p);
        }
        if wander > 0. {
            for (k, q) in points.iter_mut().enumerate() {
                let z = k as f64 / steps as f64;
                let n = |o: f64| self.signed_noise(at + Vector3::new(o, 2.5 * z, 0.));
                // Fades in from the base, which stays put.
                *q += Vector3::new(n(0.), 0.3 * n(31.4), n(62.8)) * wander * length * smoothstep(0., 0.3, z);
            }
        }
        points
    }

    /// Rings along `points`, framed by parallel transport from `first_x`.
    fn rings(points: &[Vector3<f64>], first_x: Vector3<f64>) -> Vec<Ring> {
        let n = points.len();
        let mut length = vec![0.; n];
        for i in 1 .. n {
            length[i] = length[i - 1] + (points[i] - points[i - 1]).norm();
        }
        let total = length[n - 1].max(1e-12);
        let tangent = |i: usize| (points[(i + 1).min(n - 1)] - points[i.saturating_sub(1)]).normalize();
        let mut rings = Vec::with_capacity(n);
        let t0 = tangent(0);
        let mut x = (first_x - t0 * t0.dot(&first_x)).try_normalize(1e-9).unwrap_or_else(|| perpendicular(&t0));
        let mut prev = t0;
        for i in 0 .. n {
            let t = tangent(i);
            if let Some(r) = UnitQuaternion::rotation_between(&prev, &t) {
                x = r * x;
            }
            x = (x - t * t.dot(&x)).normalize();
            prev = t;
            rings.push(Ring { centre: points[i], tangent: t, x, z: x.cross(&t), t: length[i] / total });
        }
        rings
    }

    /// Sweep `section` (a ring and an angle to the offset from the centreline) along `rings`,
    /// with `sides` points round each, closing either end to a point if asked.
    fn sweep(&mut self, rings: &[Ring], sides: usize, close: (bool, bool),
             section: &dyn Fn(&Ring, f64) -> Vector3<f64>) {
        let n = rings.len();
        let mut positions = vec![];
        let mut index: Vec<Vec<usize>> = vec![];
        for (k, ring) in rings.iter().enumerate() {
            if (k == 0 && close.0) || (k == n - 1 && close.1) {
                positions.push(ring.centre);
                index.push(vec![positions.len() - 1; sides]);
                continue;
            }
            let first = positions.len();
            for i in 0 .. sides {
                let theta = 2. * PI * i as f64 / sides as f64;
                positions.push(ring.centre + section(ring, theta));
            }
            index.push((first .. first + sides).collect());
        }
        let mut triangles = vec![];
        for k in 0 .. n - 1 {
            for i in 0 .. sides {
                let j = (i + 1) % sides;
                let (a, b, c, d) = (index[k][i], index[k][j], index[k + 1][j], index[k + 1][i]);
                // Wound so the normals face out, as a tree's stems are.
                for [u, v, w] in [[a, c, b], [a, d, c]] {
                    if u != v && v != w && u != w {
                        triangles.push([u, v, w]);
                    }
                }
            }
        }
        let normals = vertex_normals(&positions, &triangles);
        for [a, b, c] in triangles {
            let (x, y, z) = (positions[a], positions[b], positions[c]);
            let face = (y - x).cross(&(z - x));
            if face.norm() > 1e-14 {
                let nm = |k: usize| if normals[k] == Vector3::zeros() { face.normalize() } else { normals[k] };
                self.body.push(Arc::new(SmoothTriangle::new(x, y, z, nm(a), nm(b), nm(c))));
            }
        }
    }

    /// A tuft of `PER_TUFT` spines at `at`, splayed up to `spread` degrees about `out`.
    fn tuft(&mut self, at: Vector3<f64>, out: Vector3<f64>, length: f64, spread: f64) {
        let out = out.normalize();
        let side = perpendicular(&out);
        let turn = self.uniform() * 360.;
        let width = length * self.p.leaf_scale_x;
        for k in 0 .. PER_TUFT {
            let around = rotate(side, out, turn + 360. * k as f64 / PER_TUFT as f64);
            let tilt = spread * (0.4 + 0.6 * self.uniform());
            let dir = rotate(out, out.cross(&around), tilt);
            let across = dir.cross(&around).normalize() * (0.5 * width);
            let l = length * self.vary(1., 0.25);
            let base = at - dir * (0.1 * l);
            self.tufts.push(Triangle::new(base - across, base + across, base + dir * l));
        }
        self.tuft_heights.push(at.y);
    }

    /// A small pointed oval leaf, `PER_TUFT` triangles, from `at` along `dir`, facing `normal`.
    fn leaf(&mut self, at: Vector3<f64>, dir: Vector3<f64>, normal: Vector3<f64>, length: f64) {
        let dir = dir.normalize();
        let across = dir.cross(&normal).try_normalize(1e-9).unwrap_or_else(|| perpendicular(&dir));
        let half = 0.5 * length * self.p.leaf_scale_x;
        let pt = |x: f64, y: f64| at + across * (x * half) + dir * (y * length);
        let (base, tip) = (pt(0., 0.), pt(0., 1.));
        let (l1, r1, l2, r2) = (pt(-0.9, 0.3), pt(0.9, 0.3), pt(-0.75, 0.65), pt(0.75, 0.65));
        for [a, b, c] in [[base, r1, l1], [l1, r1, r2], [l1, r2, l2], [l2, r2, tip]] {
            self.tufts.push(Triangle::new(a, b, c));
        }
        self.tuft_heights.push(at.y);
    }

    /// A frame for the whole plant: turned to a random heading and leaning by `lean`.
    fn lean(&mut self) -> UnitQuaternion<f64> {
        let heading = self.uniform() * 360.;
        let lean = self.vary(self.p.lean, self.p.lean_v);
        UnitQuaternion::from_axis_angle(&Vector3::y_axis(), heading.to_radians())
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), lean.to_radians())
    }

    /// A ribbed column from `start` heading by `heading`, `length` long, of radius `radius(t)`
    /// before its domed tip; with areoles along its ribs' crests. The tube's sides follow the
    /// ribs, and its rings are spaced for the sides' width.
    fn column(&mut self, start: Vector3<f64>, length: f64, radius: f64, profile: &dyn Fn(f64) -> f64,
              heading: &dyn Fn(f64) -> Vector3<f64>, first_x: Vector3<f64>, dome_length: f64) {
        let p = self.p;
        let ribs = p.lobes;
        let sides = if ribs > 0 { (ribs as usize * 6).max(24) } else { 24 };
        let ds = (2. * PI * radius / sides as f64 * 2.5).clamp(length / 400., length / 12.);
        let steps = (length / ds).ceil() as usize;
        let ds = length / steps as f64;
        let points = self.centreline(start, steps, ds, p.wander, heading);
        let rings = Builder::rings(&points, first_x);
        let depth = p.lobe_depth;
        let r = |ring: &Ring| radius * profile(ring.t) * dome(ring.t * length, length, dome_length);
        self.sweep(&rings, sides, (false, true), &|ring, theta| ring.out(theta) * (r(ring) * ribbed(theta, ribs, depth)));

        // Areoles along each crest, staggered between neighbouring ribs.
        if p.leaves <= 0. || p.leaf_scale <= 0. {
            return;
        }
        let spacing = 1. / p.leaves;
        let crests = ribs.max(8);
        for j in 0 .. crests {
            let theta = if ribs > 0 { (2 * j + 1) as f64 * PI / ribs as f64 } else { 2. * PI * j as f64 / crests as f64 };
            let mut next = spacing * (0.5 * (j % 2) as f64 + 0.3 * self.uniform());
            let mut travelled = 0.;
            for w in rings.windows(2) {
                travelled += (w[1].centre - w[0].centre).norm();
                if travelled < next {
                    continue;
                }
                next += spacing * self.vary(1., 0.15);
                let ring = &w[1];
                let rad = r(ring);
                if rad < 0.15 * radius {
                    continue;
                }
                // On the dome, the surface faces up along the stem as well as out.
                let s = ring.t * length;
                let tip = (s - (length - dome_length)).max(0.) / dome_length.max(1e-9);
                let out = (ring.out(theta) * (1. - tip) + ring.tangent * tip).normalize();
                self.tuft(ring.centre + ring.out(theta) * rad, out, p.leaf_scale, 35.);
            }
        }
    }

    fn saguaro(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let height = self.vary(p.scale, p.scale_v).max(0.5);
        let radius = height * p.ratio;
        let frame = self.lean();
        let up = frame * Vector3::y();
        let sink = radius;
        let start = origin - Vector3::y() * sink;
        let length = height + sink;
        // Slightly narrower at the base, as the stem thickens with water above it.
        let trunk_profile = |t: f64| 0.88 + 0.12 * smoothstep(0., 0.12, t);
        let first_x = frame * Vector3::x();
        self.column(start, length, radius, &trunk_profile, &|_| up, first_x, 1.3 * radius);

        // Arms only from about 3 m tall, more on taller plants.
        let maturity = smoothstep(3., 8., height);
        let arms = (p.level(1).branches * maturity * self.vary(1., 0.5)).round().max(0.) as usize;
        let spin = self.uniform() * 360.;
        let mut heights: Vec<f64> = (0 .. arms).map(|_| 0.).collect();
        for h in heights.iter_mut() {
            *h = self.vary(0.55, 0.18) * height;
        }
        heights.sort_by(|a, b| a.total_cmp(b));
        for (k, h) in heights.into_iter().enumerate() {
            // Spread round the trunk, at roughly the golden angle from each other.
            let azimuth = spin + 137.5 * k as f64 + self.vary(0., 25.);
            let out = rotate(perpendicular(&up), up, azimuth);
            let out = (out - up * up.dot(&out)).normalize();
            let arm_radius = radius * self.vary(0.72, 0.1);
            // Leaving the trunk about level, a few drooping before they turn up.
            let rise = if self.uniform() < 0.15 { self.vary(-25., 10.) } else { self.vary(12., 12.) };
            let reach = radius + self.vary(1.6, 0.5) * arm_radius;
            let bend = arm_radius * self.vary(2.6, 0.6);
            let elbow = bend * (90. - rise).to_radians();
            let top = height * self.vary(0.85, 0.08);
            // Climbing through the reach, then the elbow (an arc of radius `bend`).
            let upright = top - h - reach * rise.to_radians().sin() - bend * rise.to_radians().cos();
            if upright < 2. * arm_radius {
                continue;
            }
            let arm_length = reach + elbow + upright;
            let heading = move |u: f64| {
                let a = if u < reach { rise } else { (rise + (u - reach) / bend.max(1e-9) * (180. / PI)).min(90.) };
                out * a.to_radians().cos() + up * a.to_radians().sin()
            };
            let start = start + up * (h + sink);
            let neck = |t: f64| 0.72 + 0.28 * smoothstep(0., 0.12, t);
            self.column(start, arm_length, arm_radius, &neck, &heading, up, 1.3 * arm_radius);
        }
    }

    fn barrel(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let height = self.vary(p.scale, p.scale_v).max(0.1);
        let radius = height * p.ratio;
        let frame = self.lean();
        let up = frame * Vector3::y();
        let sink = 0.3 * radius;
        let length = height + sink;
        // Widest about the middle, narrower at the base.
        let profile = |t: f64| 0.78 + 0.22 * (PI * t.min(1.)).sin().powf(0.7);
        self.column(origin - Vector3::y() * sink, length, radius, &profile, &|_| up, frame * Vector3::x(), 0.55 * radius);
    }

    /// One pad of length `length`, from `base` along `dir`, its faces towards +-`normal`; with
    /// areoles on both faces. Returns the pad's centre.
    fn pad(&mut self, base: Vector3<f64>, dir: Vector3<f64>, normal: Vector3<f64>, length: f64) -> Vector3<f64> {
        let p = self.p;
        let width = length * p.ratio;
        let thick = 0.09 * length;
        let across = dir.cross(&normal).normalize();
        let n = 16;
        let points: Vec<Vector3<f64>> = (0 ..= n).map(|k| {
            // Rings bunched towards the rounded ends.
            let u = 0.5 - 0.5 * (PI * k as f64 / n as f64).cos();
            base + dir * (u * length)
        }).collect();
        let rings = Builder::rings(&points, across);
        // Obovate: widest above the middle, rounded at both ends.
        let half_width = move |t: f64| 0.5 * width * (PI * t).sin().powf(0.55) * (0.78 + 0.44 * t) / 1.1;
        let half_thick = move |t: f64| 0.5 * thick * (PI * t).sin().powf(0.4) * (1.2 - 0.4 * t);
        self.sweep(&rings, 20, (true, true), &|ring, theta| {
            ring.x * (half_width(ring.t) * theta.cos()) + ring.z * (half_thick(ring.t) * theta.sin())
        });
        let count = p.leaves.max(0.).round() as usize;
        if p.leaf_scale > 0. {
            for _ in 0 .. count {
                let t = 0.08 + 0.85 * self.uniform();
                let theta = self.uniform() * 2. * PI;
                let ring = Builder::rings(&[base + dir * (t * length), base + dir * (t * length + 1e-3)], across)[0];
                let (a, b) = (half_width(t), half_thick(t));
                let at = ring.centre + ring.x * (a * theta.cos()) + ring.z * (b * theta.sin());
                // The ellipse's normal.
                let out = (ring.x * (theta.cos() / a.max(1e-9)) + ring.z * (theta.sin() / b.max(1e-9))).normalize();
                self.tuft(at, out, p.leaf_scale, 40.);
            }
        }
        base + dir * (0.5 * length)
    }

    fn prickly_pear(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let budget = p.level(1).branches.max(1.).round() as usize;
        // (base, direction, face normal, length)
        let mut queue = std::collections::VecDeque::new();
        let roots = 1 + (self.uniform() * 2.5) as usize;
        for _ in 0 .. roots {
            let azimuth = self.uniform() * 360.;
            let tilt = self.vary(25., 15.);
            let dir = rotate(Vector3::y(), rotate(Vector3::x(), Vector3::y(), azimuth), tilt);
            let normal = rotate(perpendicular(&dir), dir, self.uniform() * 180.);
            let length = self.vary(p.scale, p.scale_v);
            let offset = rotate(Vector3::x(), Vector3::y(), self.uniform() * 360.) * (0.6 * length * self.uniform());
            queue.push_back((origin + offset - Vector3::y() * (0.12 * length), dir, normal, length));
        }
        let mut made = 0;
        while let Some((base, dir, normal, length)) = queue.pop_front() {
            if made >= budget {
                break;
            }
            made += 1;
            let centre = self.pad(base, dir, normal, length);
            let width = length * p.ratio;
            let across = dir.cross(&normal).normalize();
            let children = [0, 1, 1, 2, 2, 2, 3][(self.uniform() * 7.) as usize];
            for c in 0 .. children {
                // From the upper rim, spread across it, at an angle from the pad's tip.
                let phi = if children == 1 { self.vary(0., 40.) } else {
                    -65. + 130. * (c as f64 + 0.5) / children as f64 + self.vary(0., 12.)
                };
                let rim = centre + (dir * (phi.to_radians().cos() * 0.5 * length) + across * (phi.to_radians().sin() * 0.5 * width)) * 0.85;
                let mut d = rotate(dir, normal, -phi * 0.8);
                // Turn towards the sky a little.
                d = (d + Vector3::y() * p.attraction_up.clamp(0., 1.) * 0.3).normalize();
                d = rotate(d, perpendicular(&d), self.vary(0., 15.));
                let n = rotate(normal - d * d.dot(&normal), d, self.vary(0., 70.)).normalize();
                let l = length * self.vary(0.95, 0.1);
                let child_base = rim - d * (0.12 * l);
                if (child_base + d * l).y < origin.y + 0.1 * l {
                    continue;
                }
                queue.push_back((child_base, d, n, l));
            }
        }
    }

    fn agave(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let count = p.level(1).branches.max(1.).round() as usize;
        let size = self.vary(p.scale, p.scale_v).max(0.05);
        let spin = self.uniform() * 360.;
        for i in 0 .. count {
            // Outer (older) leaves first: spreading and arching; inner ones upright.
            let q = i as f64 / count as f64;
            let azimuth = spin + 137.5 * i as f64 + self.vary(0., 6.);
            let horizontal = rotate(Vector3::x(), Vector3::y(), azimuth);
            let elevation = 12. + 72. * q.powf(0.9) + self.vary(0., 5.);
            let droop = 35. * (1. - q) * self.vary(1., 0.3);
            let length = size * (1. - 0.4 * q) * self.vary(1., 0.08);
            let width = length * p.ratio;
            let steps = 20;
            let ds = length / steps as f64;
            let heading = move |u: f64| {
                let a = (elevation - droop * (u / length).powi(2)).to_radians();
                horizontal * a.cos() + Vector3::y() * a.sin()
            };
            let start = origin + horizontal * (0.04 * size * (1. - q)) + Vector3::y() * (0.03 * size * q - 0.02 * size);
            let points = self.centreline(start, steps, ds, 0., &heading);
            let side = Vector3::y().cross(&horizontal).normalize();
            let rings = Builder::rings(&points, side);
            // The upper face, towards the rosette's centre and the sky.
            let upper = if rings[1].z.y >= 0. { 1. } else { -1. };
            let half_width = move |t: f64| 0.5 * width * if t < 0.2 { 0.55 + 0.45 * smoothstep(0., 0.2, t) } else { (1. - (t - 0.2) / 0.8).max(0.).powf(0.9) };
            let half_thick = move |t: f64| half_width(t) * (0.42 - 0.25 * t);
            self.sweep(&rings, 14, (true, true), &|ring, theta| {
                let (w, h) = (half_width(ring.t), half_thick(ring.t));
                // Channelled: the edges turned up, so the upper face is concave.
                let lift = upper * 0.35 * h * (theta.cos().powi(2) - 0.5);
                ring.x * (w * theta.cos()) + ring.z * (h * theta.sin() + lift)
            });
            if p.leaf_scale <= 0. {
                continue;
            }
            // The terminal spine, and teeth along both margins.
            let tip = rings[rings.len() - 1];
            self.tuft(tip.centre - tip.tangent * (0.2 * p.leaf_scale), tip.tangent, 2. * p.leaf_scale, 4.);
            let teeth = p.leaves.max(0.).round() as usize;
            for k in 0 .. teeth {
                let t = 0.15 + 0.75 * (k as f64 + 0.5) / teeth as f64;
                let ring = rings[((t * steps as f64).round() as usize).min(steps)];
                for s in [-1., 1.] {
                    let w = half_width(ring.t);
                    let lift = upper * 0.35 * half_thick(ring.t) * 0.5;
                    self.tuft(ring.centre + ring.x * (s * w) + ring.z * lift, ring.x * s + ring.tangent * 0.3, 0.4 * p.leaf_scale, 10.);
                }
            }
        }
    }

    fn ocotillo(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let canes = p.level(1).branches.max(1.).round() as usize;
        let size = self.vary(p.scale, p.scale_v).max(0.2);
        for _ in 0 .. canes {
            let azimuth = self.uniform() * 360.;
            let horizontal = rotate(Vector3::x(), Vector3::y(), azimuth);
            // Most canes near upright, a few splayed: the fan.
            let spread = (p.lean.abs() * self.uniform().powf(0.7) + self.vary(0., p.lean_v)).clamp(1., 60.);
            let length = size * self.vary(1., 0.3);
            let radius = length * p.ratio;
            let bow = self.vary(8., 6.);
            let heading = move |u: f64| {
                let a = (90. - spread - bow * u / length).to_radians();
                horizontal * a.cos() + Vector3::y() * a.sin()
            };
            let steps = 24;
            let start = origin + horizontal * (0.05 * size * self.uniform()) - Vector3::y() * (2. * radius);
            let points = self.centreline(start, steps, length / steps as f64, p.wander, &heading);
            let rings = Builder::rings(&points, Vector3::y().cross(&horizontal));
            self.sweep(&rings, 6, (false, true), &|ring, theta| ring.out(theta) * (radius * (1. - 0.7 * ring.t)));
            if p.leaf_scale <= 0. {
                continue;
            }
            let count = p.leaves.max(0.).round() as usize;
            for _ in 0 .. count {
                let t = 0.06 + 0.92 * self.uniform();
                let ring = rings[((t * steps as f64) as usize).min(steps)];
                let theta = self.uniform() * 2. * PI;
                let out = ring.out(theta);
                let r = radius * (1. - 0.7 * ring.t);
                let dir = (out + ring.tangent * self.vary(0.9, 0.4)).normalize();
                let normal = dir.cross(&ring.tangent.cross(&dir)).try_normalize(1e-9).unwrap_or(out);
                let l = p.leaf_scale * self.vary(1., p.leaf_scale_v);
                self.leaf(ring.centre + out * r, dir, normal, l);
            }
        }
    }
}

impl Builder<'_> {
    fn mound(&mut self, origin: Vector3<f64>) {
        let p = self.p;
        let height = self.vary(p.scale, p.scale_v).max(0.1);
        let width = height * p.ratio;
        let centre = origin + Vector3::y() * (0.48 * height);
        let half = Vector3::new(0.5 * width, 0.52 * height, 0.5 * width);
        let lump = Vector3::new(self.uniform(), self.uniform(), self.uniform()) * 100.;
        // Lumpy: the canopy's radius varies with direction, in a few broad clumps.
        let reach = |b: &Builder, d: &Vector3<f64>| 1. + 0.3 * b.signed_noise(lump + d * 1.6);
        let stems = p.level(1).branches.max(1.).round() as usize;
        let mut dead = vec![];
        for _ in 0 .. stems {
            // Each stem leans out from the base towards a point in the canopy.
            let azimuth = self.uniform() * 2. * PI;
            let out = Vector3::new(azimuth.cos(), 0., azimuth.sin());
            let tip = centre + Vector3::new(out.x * half.x, 0., out.z * half.z) * (0.3 + 0.5 * self.uniform())
                + Vector3::y() * (half.y * self.vary(0.3, 0.3));
            let base = origin + out * (0.06 * width * self.uniform()) - Vector3::y() * (0.02 * height);
            let length = (tip - base).norm();
            let steps = 12;
            let start_dir = (Vector3::y() * 2. + out).normalize();
            let to_tip = (tip - base).normalize();
            let heading = move |u: f64| {
                let t = (u / length).clamp(0., 1.);
                start_dir * (1. - t) + to_tip * t
            };
            let points = self.centreline(base, steps, length / steps as f64, p.wander, &heading);
            let rings = Builder::rings(&points, out);
            let radius = 0.022 * height * self.vary(1., 0.3);
            self.sweep(&rings, 5, (false, true), &|ring, theta| ring.out(theta) * (radius * (1. - 0.75 * ring.t)));
            if self.uniform() < p.dead_branches {
                dead.push(tip);
            }
        }
        let count = p.leaves.max(0.).round() as usize;
        for _ in 0 .. count {
            // A direction, mostly above the equator, and a depth, mostly near the surface.
            let mut d = Vector3::new(self.uniform() * 2. - 1., self.uniform() * 1.6 - 0.6, self.uniform() * 2. - 1.);
            if d.norm() < 1e-6 { continue }
            d = d.normalize();
            let depth = 1. - 0.45 * self.uniform().powi(2);
            let at = centre + d.component_mul(&half) * (depth * reach(self, &d));
            if at.y < origin.y + 0.06 * height {
                continue;
            }
            // Bare where a dead stem's part of the canopy would have been.
            if dead.iter().any(|t| (at - t).norm() < 0.35 * width) {
                continue;
            }
            let outward = (d + Vector3::y() * 0.5).normalize();
            let dir = (outward + Vector3::new(self.uniform() - 0.5, self.uniform() - 0.5, self.uniform() - 0.5) * 1.6).normalize();
            let normal = (outward - dir * dir.dot(&outward)).try_normalize(1e-9).unwrap_or_else(|| perpendicular(&dir));
            let l = p.leaf_scale * self.vary(1., p.leaf_scale_v);
            self.leaf(at, dir, normal, l);
        }
    }
}

/// Generate the plant `p` describes, whose form isn't `Branching`, chosen by `seed`, with its
/// base at `origin`.
pub fn generate(p: &TreeParams, seed: u64, origin: Vector3<f64>) -> TreeMesh {
    let mut b = Builder {
        p, rng: Pcg32::new(seed, 0xcac7), noise: PerlinNoise::new(),
        body: vec![], tufts: vec![], tuft_heights: vec![],
    };
    match p.form {
        PlantForm::Saguaro => b.saguaro(origin),
        PlantForm::Barrel => b.barrel(origin),
        PlantForm::PricklyPear => b.prickly_pear(origin),
        PlantForm::Agave => b.agave(origin),
        PlantForm::Ocotillo => b.ocotillo(origin),
        PlantForm::Mound => b.mound(origin),
        PlantForm::Branching => panic!("Branching plants are grown by tree::generate"),
    }
    let (low, high) = b.tuft_heights.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &y| (lo.min(y), hi.max(y)));
    let leaf_states = b.tuft_heights.iter().map(|&y| LeafState {
        age: 0., exposure: if high > low { ((y - low) / (high - low)) as f32 } else { 1. },
    }).collect();
    let leaves = b.tufts.chunks(PER_TUFT).enumerate()
        .flat_map(|(k, ts)| ts.iter().map(move |t| Arc::new(Tagged { item: t.clone(), face: k as u32 })))
        .collect();
    TreeMesh { bark: b.body, dead_bark: vec![], leaves, leaf_triangles: PER_TUFT, leaf_states }
}

#[cfg(test)]
mod tests {
    use crate::procedural::tree::{generate, params, SPECIES};
    use crate::na::Vector3;
    use serde_json::Value;

    const SUCCULENTS: [&str; 7] = ["saguaro", "barrel-cactus", "prickly-pear", "agave", "ocotillo", "sagebrush", "rabbitbrush"];

    #[test]
    fn test_succulents_grow_closed_outward_facing_bodies() {
        for name in SUCCULENTS {
            assert!(SPECIES.contains(&name), "{}", name);
            let m = generate(&params(name, &Value::Null), 2, Vector3::zeros());
            assert!(m.bark.len() > 200, "{} has {} body triangles", name, m.bark.len());
            assert!(!m.leaves.is_empty(), "{} has no spines or leaves", name);
            assert_eq!(m.leaves.len(), m.leaf_states.len() * m.leaf_triangles, "{}", name);
            for t in &m.bark {
                assert!(t.v0.iter().chain(t.v1.iter()).chain(t.v2.iter()).all(|x| x.is_finite()), "{}", name);
                // Shading normals agree with the winding.
                let face = (t.v1 - t.v0).cross(&(t.v2 - t.v0));
                assert!(face.dot(&(t.normalv0 + t.normalv1 + t.normalv2)) > 0., "{} has a triangle facing in", name);
            }
        }
    }

    #[test]
    fn test_tall_saguaros_have_arms_and_young_ones_none() {
        let column = |scale: f64| {
            let p = params("saguaro", &serde_json::json!({ "scale": scale, "scale_v": 0, "wander": 0, "lean_v": 0 }));
            let m = generate(&p, 5, Vector3::zeros());
            // Body far from the trunk's axis is an arm's.
            let radius = scale * p.ratio;
            let far = m.bark.iter().filter(|t| t.v0.xz().norm() > 2.5 * radius).count();
            let top = m.bark.iter().map(|t| t.v0.y).fold(0., f64::max);
            (far, top)
        };
        let (young, young_top) = column(2.);
        assert_eq!(young, 0);
        assert!((young_top - 2.).abs() < 0.05, "{}", young_top);
        let (old, old_top) = column(11.);
        assert!(old > 100, "{}", old);
        assert!((old_top - 11.).abs() < 0.1, "{}", old_top);
    }

    #[test]
    fn test_ribs_make_crests_and_grooves() {
        let p = params("barrel-cactus", &serde_json::json!({ "scale_v": 0, "lean": 0, "lean_v": 0, "wander": 0 }));
        let m = generate(&p, 1, Vector3::zeros());
        let height = p.scale;
        // Round the middle of the body, the radius varies by the ribs' depth.
        let ring: Vec<f64> = m.bark.iter().filter(|t| (t.v0.y - 0.5 * height).abs() < 0.03 * height).map(|t| t.v0.xz().norm()).collect();
        let (lo, hi) = ring.iter().fold((f64::MAX, 0f64), |(lo, hi), &r| (lo.min(r), hi.max(r)));
        assert!(1. - lo / hi > 0.6 * p.lobe_depth, "{} to {}", lo, hi);
    }

    #[test]
    fn test_mounds_are_dense_and_rounded() {
        let p = params("sagebrush", &serde_json::json!({ "scale_v": 0, "dead_branches": 0 }));
        let m = generate(&p, 4, Vector3::zeros());
        let (h, w) = (p.scale, p.scale * p.ratio);
        // Leaves fill a mound about as wide and tall as asked, none below the ground.
        let centres: Vec<Vector3<f64>> = m.leaves.iter().map(|t| t.item.v0).collect();
        assert!(centres.len() > 4 * 1500);
        let top = centres.iter().map(|c| c.y).fold(0., f64::max);
        let reach = centres.iter().map(|c| c.xz().norm()).fold(0., f64::max);
        assert!(top > 0.8 * h && top < 1.4 * h, "{} vs {}", top, h);
        assert!(reach > 0.35 * w && reach < 0.8 * w, "{} vs {}", reach, w);
        assert!(centres.iter().all(|c| c.y > -0.05));
        // Dead stems leave gaps: fewer leaves.
        let dead = generate(&params("sagebrush", &serde_json::json!({ "scale_v": 0, "dead_branches": 1 })), 4, Vector3::zeros());
        assert!(dead.leaves.len() < m.leaves.len() / 2, "{} {}", dead.leaves.len(), m.leaves.len());
    }

    #[test]
    fn test_prickly_pear_pads_stay_above_ground() {
        let m = generate(&params("prickly-pear", &Value::Null), 3, Vector3::zeros());
        let lowest = m.bark.iter().map(|t| t.v0.y).fold(f64::MAX, f64::min);
        assert!(lowest > -0.1, "{}", lowest);
        let highest = m.bark.iter().map(|t| t.v0.y).fold(0., f64::max);
        assert!(highest > 0.4, "{}", highest);
    }
}
