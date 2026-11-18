//! # Cauliflower cumulus
//!
//! A cumulus is a heap of rising thermals, each a turret carrying smaller turrets: domes on
//! domes over several scales, the "cauliflower" look. Noise thresholded to a coverage (Nubis)
//! makes soft, cotton-wool shapes instead, without that structure. Here, after Antoine Bouthors
//! and Fabrice Neyret, "Modeling clouds shape", Eurographics 2004 Short Papers (and Webanck et
//! al., "Procedural Cloudscapes", Computer Graphics Forum 37(2), 2018), a cloud is a tree of
//! spheres:
//!
//! - A few root spheres make the cloud's body, side by side over its base.
//! - Each sphere carries `children` smaller spheres, `ratio` its radius, set into its surface
//!   (their centres `embed` of their radius inside it) in directions biased upwards (`rise`):
//!   turrets rise from the top and sides, never hang from the bottom.
//! - So on down for `levels` levels, a few tens of metres at the finest.
//!
//! The cloud is the union of the spheres, and its density rises over a thin `edge` inwards
//! from the union's surface: crisp outlines, as real cumulus have, with their base cut flat at
//! the condensation level by whoever uses it.
//!
//! Nothing is stored: each sphere's children follow from a hash of its own id, so any number
//! of clouds cost nothing until looked at. Evaluating a point visits only spheres whose whole
//! subtree could reach it (each level's reach is known in advance), and stops as soon as the
//! point is deeper than the edge inside any sphere, as most of a cloud's interior is.

use crate::na::Vector3;

/// One sphere of a tree: its centre and radius, and the id its children are hashed from.
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector3<f64>,
    pub radius: f64,
    pub id: u64,
}

#[derive(Clone, Debug)]
pub struct Cauliflower {
    /// Levels of turrets below the roots.
    pub levels: usize,
    /// A child's radius, as a fraction of its parent's.
    pub ratio: f64,
    /// Children per sphere at the first level; one fewer at each level after, but at least 3.
    pub children: usize,
    /// How far a child's centre is set inside its parent's surface, as a fraction of the
    /// child's radius.
    pub embed: f64,
    /// How strongly turrets point upwards (0 any direction, larger more upright).
    pub rise: f64,
    /// Largest and smallest child, relative to `ratio`: turrets vary in size.
    pub spread: f64,
    /// Each level's reach beyond its own radius, as a multiple of its radius (from `reach`).
    reach: Vec<f64>,
}

/// The most a turret's size is scaled on top of a sphere (`Cauliflower::top_factor`).
const TOP_BOOST: f64 = 1.25;

/// SplitMix64: a good 64-bit mix, for ids and their random numbers.
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// The `k`-th random number in [0, 1) of `id`.
pub fn unit(id: u64, k: u64) -> f64 {
    (mix(id ^ mix(k.wrapping_add(0x51ED_270B))) >> 11) as f64 / (1u64 << 53) as f64
}

impl Cauliflower {
    pub fn new(levels: usize, ratio: f64, children: usize, embed: f64, rise: f64) -> Cauliflower {
        let mut c = Cauliflower { levels, ratio, children, embed, rise, spread: 0.3, reach: vec![] };
        c.reach = c.reaches();
        c
    }

    /// The default: four levels of turrets, each 0.45 of the one it grows on, so a 500 m body
    /// carries turrets of 225, 100, 45 and 20 m, eight on each body sphere.
    pub fn cumulus() -> Cauliflower {
        Cauliflower::new(4, 0.45, 8, 0.35, 0.9)
    }

    /// The farthest any sphere of a subtree reaches from its root's centre, over the root's
    /// radius, for a root at each level: a child of radius at most r (1 + spread) ratio sits
    /// with its centre r - embed × its radius from its parent's, and reaches its own reach
    /// beyond that.
    fn reaches(&self) -> Vec<f64> {
        let mut reach = vec![1.; self.levels + 1];
        let largest = self.ratio * (1. + self.spread) * TOP_BOOST;
        for l in (0..self.levels).rev() {
            reach[l] = (1. - self.embed * largest + largest * reach[l + 1]).max(1.);
        }
        reach
    }

    /// How far a tree rooted at a sphere of `radius` can reach from its centre.
    pub fn extent(&self, radius: f64) -> f64 {
        radius * self.reach[0]
    }

    fn child_count(&self, level: usize) -> usize {
        self.children.saturating_sub(level).max(3)
    }

    /// The `k`-th child of `s` (at `level`, the parent's), growing up along `up` (a unit vector).
    ///
    /// Packed over the parent's upper surface, as Bouthors and Neyret place their particles on
    /// the cloud's surface: the `n` turrets of a sphere at the points of a Fibonacci spiral over
    /// the cap from straight up to `rise` below the equator (cos θ from 1 to -`rise` × 0.25),
    /// turned by the parent's hash and jittered a little. Evenly spread, each takes its own share
    /// of the surface: neighbours meet in a crease rather than piling into one smooth blob, as
    /// randomly placed turrets did (they cluster).
    fn child(&self, s: &Sphere, k: usize, n: usize, up: &Frame) -> Sphere {
        let (t1, t2, up) = (&up.t1, &up.t2, &up.up);
        let id = mix(s.id.wrapping_mul(31).wrapping_add(k as u64 + 1));
        let n = n.max(1) as f64;
        let low = -0.25 * self.rise.clamp(0., 4.);
        let jitter = |j: u64| (unit(id, j) - 0.5) / n;
        let z = (1. - (1. - low) * ((k as f64 + 0.5) / n + jitter(0))).clamp(low, 1.);
        let a = std::f64::consts::TAU * (0.381_966 * k as f64 + unit(s.id, 7) + jitter(1));
        let r = (1. - z * z).max(0.).sqrt();
        let d = t1 * (r * a.cos()) + up * z + t2 * (r * a.sin());
        // Larger on top, smaller on the sides: the head of a cumulus is its biggest domes.
        let upness = d.dot(up).max(0.);
        let radius = s.radius * self.ratio * (1. + self.spread * (2. * unit(id, 2) - 1.)) * Self::top_factor(upness);
        Sphere { centre: s.centre + d * (s.radius - self.embed * radius), radius, id }
    }

    /// How a turret's size varies with how upright it is (0 sideways, 1 straight up): from 0.75
    /// to `TOP_BOOST` times.
    fn top_factor(upness: f64) -> f64 {
        0.75 + (TOP_BOOST - 0.75) * upness
    }

    /// Whether `c` (a turret of the tree grown from `roots[from]`) is buried in another root:
    /// its centre deeper inside it than half its own radius. As Bouthors and Neyret place their
    /// particles on the cloud's surface, where they are seen, a buried turret and everything on
    /// it is left out (removing spheres only shrinks the union, so bounds still hold).
    fn buried(c: &Sphere, roots: &[Sphere], from: usize) -> bool {
        roots.iter().enumerate().any(|(j, r)| j != from && (c.centre - r.centre).norm() < r.radius - 0.5 * c.radius)
    }

    /// How deep `p` is inside the union of the trees grown from `roots` (metres; negative
    /// outside), along `up`; any depth over `enough` is reported as `enough`, which is all
    /// that's needed and cuts the search short.
    pub fn depth(&self, roots: &[Sphere], up: &Vector3<f64>, p: &Vector3<f64>, enough: f64) -> f64 {
        let up = &Frame::new(up);
        let mut best = f64::NEG_INFINITY;
        for (i, root) in roots.iter().enumerate() {
            self.visit(root, 0, up, p, enough, &mut best, roots, i);
            if best >= enough {
                return enough;
            }
        }
        best
    }

    #[allow(clippy::too_many_arguments)]
    fn visit(&self, s: &Sphere, level: usize, up: &Frame, p: &Vector3<f64>, enough: f64, best: &mut f64, roots: &[Sphere], from: usize) {
        let dist = (p - s.centre).norm();
        // Nothing in this subtree reaches the point any deeper than already found.
        if s.radius * self.reach[level] - dist <= *best {
            return;
        }
        *best = best.max(s.radius - dist);
        if *best >= enough || level == self.levels {
            return;
        }
        for k in 0..self.child_count(level) {
            let c = self.child(s, k, self.child_count(level), up);
            if Cauliflower::buried(&c, roots, from) {
                continue;
            }
            self.visit(&c, level + 1, up, p, enough, best, roots, from);
            if *best >= enough {
                return;
            }
        }
    }
}

/// A tree grown once and kept (`Cauliflower::grow`): every sphere not buried, with its level and
/// children, so a cloud looked up millions of times needn't regenerate its turrets (their
/// hashes, directions and burial tests) at each lookup.
pub struct Grown {
    nodes: Vec<Node>,
    roots: usize,
}

struct Node {
    centre: Vector3<f64>,
    radius: f64,
    level: u8,
    first: u32,
    count: u8,
}

impl Cauliflower {
    /// The whole tree grown from `roots` along `up`, as `depth` would visit it.
    pub fn grow(&self, roots: &[Sphere], up: &Vector3<f64>) -> Grown {
        self.grow_capped(roots, up, |_| self.levels)
    }

    /// As `grow`, but a sphere centred at `c` carries turrets only down to level `cap(c)`: a
    /// glaciated top's smooth domes (`Cumulus::glaciation`). Fewer levels only shrink the
    /// union, so the levels' reach still bounds it.
    pub fn grow_capped(&self, roots: &[Sphere], up: &Vector3<f64>, cap: impl Fn(&Vector3<f64>) -> usize) -> Grown {
        let frame = Frame::new(up);
        let mut nodes: Vec<Node> = roots.iter().map(|r| Node { centre: r.centre, radius: r.radius, level: 0, first: 0, count: 0 }).collect();
        let mut spheres: Vec<(Sphere, usize)> = roots.iter().enumerate().map(|(i, r)| (*r, i)).collect();
        // Breadth first, so each node's children are contiguous.
        let mut i = 0;
        while i < nodes.len() {
            let level = nodes[i].level as usize;
            if level < self.levels && level < cap(&nodes[i].centre) {
                let (s, from) = spheres[i];
                let first = nodes.len();
                for k in 0..self.child_count(level) {
                    let c = self.child(&s, k, self.child_count(level), &frame);
                    if Cauliflower::buried(&c, roots, from) {
                        continue;
                    }
                    nodes.push(Node { centre: c.centre, radius: c.radius, level: level as u8 + 1, first: 0, count: 0 });
                    spheres.push((c, from));
                }
                nodes[i].first = first as u32;
                nodes[i].count = (nodes.len() - first) as u8;
            }
            i += 1;
        }
        Grown { nodes, roots: roots.len() }
    }
}

impl Grown {
    /// As `Cauliflower::depth`, from the kept tree (`tree` for the levels' reach).
    pub fn depth(&self, tree: &Cauliflower, p: &Vector3<f64>, enough: f64) -> f64 {
        let mut best = f64::NEG_INFINITY;
        for i in 0..self.roots {
            self.visit(tree, i, p, enough, &mut best);
            if best >= enough {
                return enough;
            }
        }
        best
    }

    fn visit(&self, tree: &Cauliflower, i: usize, p: &Vector3<f64>, enough: f64, best: &mut f64) {
        let n = &self.nodes[i];
        let dist = (p - n.centre).norm();
        if n.radius * tree.reach[n.level as usize] - dist <= *best {
            return;
        }
        *best = best.max(n.radius - dist);
        if *best >= enough {
            return;
        }
        for c in n.first as usize..n.first as usize + n.count as usize {
            self.visit(tree, c, p, enough, best);
            if *best >= enough {
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

/// The body of one cumulus: root spheres over a flat disc of radius `width` about
/// `base_centre` (on its base), standing `lift` of their radius above it so the base cuts them
/// broad and flat, more of them the wider it is, bunched towards the core the taller the cloud
/// (`tall`, 0 to 1), the outer ones smaller: the middle ones larger by a factor 1 + `dome`,
/// falling off as 1 - ρ² to the rim, so the body is already a heap, a convex dome, before any
/// turrets grow on it (spheres all alike made a flat-topped, boxy cloud), while every sphere,
/// its centre `lift` of its radius above the base, is still cut flat by it. The largest root's
/// radius is r0 (1 + dome).
/// `phase` turns the pattern; `id` seeds it.
#[allow(clippy::too_many_arguments)]
pub fn body(base_centre: &Vector3<f64>, up: &Vector3<f64>, r0: f64, width: f64, lift: f64, dome: f64, tall: f64, id: u64, phase: f64) -> Vec<Sphere> {
    let (t1, t2) = basis(up);
    let n = ((1.5 * (width / r0).powi(2)).round() as usize).clamp(1, 12);
    let disc = (width - r0).max(0.);
    (0..n).map(|k| {
        let id = id.wrapping_mul(7).wrapping_add(k as u64);
        let u = |j: u64| unit(id, j);
        let (rho, a) = if k == 0 { (0., 0.) } else { (u(0).powf(0.5 + tall), std::f64::consts::TAU * (u(1) + phase)) };
        let r = r0 * (1. - 0.5 * rho) * (0.8 + 0.2 * u(2)) * (1. + dome * (1. - rho * rho));
        let centre = base_centre + up * (lift * r) + (t1 * a.cos() + t2 * a.sin()) * (disc * rho);
        Sphere { centre, radius: r, id }
    }).collect()
}

/// The body of a towering cumulus (congestus, or the column of a cumulonimbus), `width` in
/// radius and `height` tall over `base_centre`.
///
/// Deep convection rises as a succession of buoyant bubbles, each overtaking the wake of the
/// last (R. S. Scorer and F. H. Ludlam, "Bubble theory of penetrative convection", Q. J. R.
/// Meteorol. Soc. 79, 1953): a tower is a column of rounded heads, the newest on top, its sides
/// keeping the lumps of those below. And a congestus is not one column but a cluster of them
/// at different stages: the tallest near the middle, younger and older ones stepping down
/// around it, all rising from a broad heaped base; seen from afar, a mountain of turrets rather
/// than a pillar. So the roots are columns of stacked thermals (`column`): a main one, and
/// around it others lower the farther out they stand, over a base of short ones (`reach` says
/// how far out it goes, turrets and all).
pub fn tower(base_centre: &Vector3<f64>, up: &Vector3<f64>, width: f64, height: f64, lift: f64, tree: &Cauliflower, id: u64, phase: f64) -> Vec<Sphere> {
    let (t1, t2) = basis(up);
    let mut roots = Vec::new();
    // The main column.
    column(&mut roots, base_centre, up, 0.55 * width, height, lift, tree, mix(id), phase);
    // Flanking columns, lower the farther out; then the broad base of short ones.
    let (inner, outer) = (4, 6);
    for k in 0..inner + outer {
        let cid = mix(id.wrapping_add(17 + k as u64));
        let u = |j: u64| unit(cid, j);
        let ring = k >= inner;
        let (m, n) = if ring { (k - inner, outer) } else { (k, inner) };
        let rho = if ring { 0.6 + 0.08 * u(0) } else { 0.28 + 0.2 * u(0) };
        let a = std::f64::consts::TAU * ((m as f64 + 0.6 * u(1)) / n as f64 + phase + if ring { 0.13 } else { 0. });
        let cw = if ring { (0.3 + 0.08 * u(2)) * width } else { (0.38 + 0.1 * u(2)) * width };
        let h = if ring { height * (0.12 + 0.15 * u(3)) } else { height * (0.35 + 0.45 * u(3)) * (1. - 0.8 * (rho - 0.28)) };
        let at = base_centre + (t1 * a.cos() + t2 * a.sin()) * (rho * width);
        column(&mut roots, &at, up, cw, h, lift, tree, cid, u(4));
    }
    roots
}

/// How far from the axis through `base_centre` along `up` the trees of `tree` grown from
/// `roots` can reach.
pub fn reach(roots: &[Sphere], base_centre: &Vector3<f64>, up: &Vector3<f64>, tree: &Cauliflower) -> f64 {
    roots.iter().map(|r| {
        let d = r.centre - base_centre;
        (d - up * up.dot(&d)).norm() + tree.extent(r.radius)
    }).fold(0., f64::max)
}

/// One column of thermals, `width` in radius and `height` tall over `base_centre`: spheres
/// 0.55 of its width stacked up its axis, each a little off it, the top one the largest (its
/// head) and standing so that its turrets reach `height`.
#[allow(clippy::too_many_arguments)]
fn column(roots: &mut Vec<Sphere>, base_centre: &Vector3<f64>, up: &Vector3<f64>, width: f64, height: f64, lift: f64, tree: &Cauliflower, id: u64, phase: f64) {
    let (t1, t2) = basis(up);
    let ext = tree.extent(1.);
    let r = 0.55 * width;
    // A short, broad column: one sphere.
    let head = (1.1 * r).min(height / (lift + ext));
    let top = (height - ext * head).max(lift * head);
    let bottom = lift * r.min(head);
    let step = 0.75 * r;
    let levels = (((top - bottom) / step).ceil() as usize).clamp(0, 24) + 1;
    for k in 0..levels {
        let id = mix(id.wrapping_mul(13).wrapping_add(k as u64));
        let u = |j: u64| unit(id, j);
        let t = if levels == 1 { 1. } else { k as f64 / (levels - 1) as f64 };
        let z = bottom + (top - bottom) * t;
        let radius = if k + 1 == levels { head } else { r * (0.85 + 0.2 * u(0)) };
        // Off the axis, so the column isn't a stack of beads: at most 0.1 of the width.
        let a = std::f64::consts::TAU * (u(1) + phase);
        let off = 0.1 * width * u(2).sqrt();
        roots.push(Sphere { centre: base_centre + up * z + (t1 * a.cos() + t2 * a.sin()) * off, radius, id });
    }
}

/// A cumulus's surface finer than its spheres: pyroclastic billows (after Tessendorf's Cumulo:
/// generations of |noise|, `billows` as (size, amplitude) m, pushing the surface outwards,
/// puffs on puffs with creases between) and wisps (the point looked up carried two steps
/// through vector noise `wisp_size` across, by up to `wisps` m, fraying the edges).
pub struct Surface {
    pub billows: Vec<(f64, f64)>,
    pub wisps: f64,
    pub wisp_size: f64,
    noise: crate::noise::PerlinNoise,
}

impl Surface {
    pub fn new(seed: u64) -> Surface {
        Surface { billows: vec![(120., 30.), (60., 15.), (30., 8.)], wisps: 40., wisp_size: 150., noise: crate::noise::PerlinNoise::shuffled(seed) }
    }

    /// How far beyond the spheres any density can reach: the billows' outward push, and the
    /// wisps' shift of the point looked up (√3 times their amplitude: a noise vector).
    pub fn margin(&self) -> f64 {
        self.billows.iter().map(|b| b.1).sum::<f64>() + 3f64.sqrt() * self.wisps
    }

    /// The point `p` looked up, carried two steps through vector noise (the wisps).
    pub fn frayed(&self, p: &Vector3<f64>) -> Vector3<f64> {
        if self.wisps <= 0. {
            return *p;
        }
        let mut q = *p;
        for step in 0..2 {
            let x = q / self.wisp_size + Vector3::repeat(41.3 * step as f64);
            let v = Vector3::new(self.noise.noise(x.x, x.y, x.z), self.noise.noise(x.x + 17.1, x.y - 5.3, x.z + 9.7),
                                 self.noise.noise(x.x - 23.9, x.y + 13.3, x.z - 3.1));
            q += v * (0.5 * self.wisps);
        }
        q
    }

    /// How much of the densest cloud is at a point `depth` metres inside the spheres (billows
    /// included), `q` (frayed), of a cloud with crisp edges `edge` deep, as `maturity` (0 to 1)
    /// of the way to a grown cloud.
    ///
    /// A cumulus forms as wisps: rising thermals reach their condensation level a patch at a
    /// time, and the first cloud is thin and ragged, translucent, the sky seen through it,
    /// before it fills out into a heap with sharp edges; a dying one frays and thins the same way
    /// as it evaporates into the drier air around it (entrainment at its edges, the edges first).
    /// So the less mature the cloud, the less water it holds (density down to a sixth), the
    /// softer its edge (to 25 times as deep), and the more of it is gaps: condensed only where a
    /// noise (patches about 250 m, the size of the first thermals) exceeds what it lacks.
    pub fn condensed(&self, depth: f64, edge: f64, maturity: f64, q: &Vector3<f64>) -> f64 {
        let m = maturity.clamp(0., 1.);
        let soft = 1. - m;
        let edge = edge * (1. + 24. * soft * soft);
        let f = smoothstep(0., edge, depth);
        if f <= 0. || soft <= 0. {
            return f;
        }
        let x = q / 250. + Vector3::repeat(3.1);
        let n = 0.5 + 0.5 * (0.65 * self.noise.noise(x.x, x.y, x.z) + 0.35 * self.noise.noise(2.1 * x.x, 2.1 * x.y + 5.3, 2.1 * x.z));
        f * smoothstep(soft - 0.15, soft + 0.15, n) * (0.15 + 0.85 * m * m)
    }

    /// The billows' outward push at `p` (m): |noise| per generation.
    pub fn billowing(&self, p: &Vector3<f64>) -> f64 {
        self.billows.iter().enumerate().map(|(g, (size, amplitude))| {
            let x = p / *size + Vector3::repeat(7.7 * g as f64);
            amplitude * self.noise.noise(x.x, x.y, x.z).abs().min(1.)
        }).sum()
    }
}

/// # One cumulus
///
/// A single cauliflower cumulus in the scene's own (flat) coordinates, for a cloud placed by
/// hand: a hero cloud, or a test of the shape. Its body is `body`'s spheres, `width` in radius,
/// growing turrets up to `depth` above its flat `base`, leaning `lean` per metre of height along
/// `wind`, and finished by its `surface`.
pub struct Cumulus {
    pub tree: Cauliflower,
    pub surface: Surface,
    pub max_density: f64,
    pub base: f64,
    pub edge: f64,
    pub lean: f64,
    pub wind: Vector3<f64>,
    grown: Grown,
    r0: f64,
    lift: f64,
    dome: f64,
    reach: f64,
    centre: Vector3<f64>,
    /// How far it has formed (0 to 1; `Surface::condensed`).
    pub maturity: f64,
    /// For a cumulonimbus: the altitude above which its top has frozen (`glaciated`).
    pub glaciation: Option<f64>,
    /// Its highest top, for a tower (`Cumulus::tower`).
    tower_top: Option<f64>,
}

/// How glaciated a cloud is at `altitude`, its tops freezing from `glaciation` (0 to 1, over
/// a kilometre). When a towering cumulus's top freezes it becomes a cumulonimbus: the sharp
/// cauliflower of liquid droplets gives way to the smooth, then fibrous, outline of ice (calvus,
/// then capillatus; the WMO's International Cloud Atlas). Ice crystals are fewer and larger
/// than droplets, so the ice is thinner and its edges soft: here half the density, an edge
/// ten times as deep, and turrets only one level below the thermals (`Cauliflower::grow_capped`).
pub fn glaciated(glaciation: Option<f64>, altitude: f64) -> f64 {
    glaciation.map_or(0., |g| smoothstep(g - 500., g + 500., altitude))
}

/// The turret levels a sphere centred at `altitude` keeps under `glaciation`.
pub fn glaciated_levels(tree: &Cauliflower, glaciation: Option<f64>, altitude: f64) -> usize {
    if glaciated(glaciation, altitude) > 0.5 { 1 } else { tree.levels }
}

impl Cumulus {
    /// A cloud over (`x`, `z`) = `centre`, its base at `base`, `width` in radius and `depth` deep.
    #[allow(clippy::too_many_arguments)]
    pub fn new(tree: Cauliflower, surface: Surface, centre: crate::na::Vector2<f64>, base: f64, width: f64, depth: f64,
               lift: f64, dome: f64, lean: f64, wind: crate::na::Vector2<f64>, seed: u64) -> Cumulus {
        let r0 = (depth / ((1. + dome) * (lift + tree.extent(1.)))).max(10.);
        let width = width.max(r0);
        let tall = (depth / (2. * width)).clamp(0., 1.);
        let at = Vector3::new(centre.x, base, centre.y);
        let roots = body(&at, &Vector3::y(), r0, width, lift, dome, tall, mix(seed), unit(seed, 99));
        let wind = Vector3::new(wind.x, 0., wind.y);
        let wind = if wind.norm() > 0. { wind.normalize() } else { Vector3::zeros() };
        let reach = width + r0 * (1. + dome) * tree.extent(1.);
        let grown = tree.grow(&roots, &Vector3::y());
        Cumulus { tree, surface, max_density: 0.05, base, edge: 15., lean, wind, grown, r0, lift, dome, reach, centre: at,
                  maturity: 1., glaciation: None, tower_top: None }
    }

    /// A towering cumulus (`tower`'s thermals), `width` in radius and `height` tall, freezing
    /// above `glaciation` (a cumulonimbus) if given.
    #[allow(clippy::too_many_arguments)]
    pub fn tower(tree: Cauliflower, surface: Surface, centre: crate::na::Vector2<f64>, base: f64, width: f64, height: f64,
                 lift: f64, lean: f64, wind: crate::na::Vector2<f64>, glaciation: Option<f64>, seed: u64) -> Cumulus {
        let at = Vector3::new(centre.x, base, centre.y);
        let roots = tower(&at, &Vector3::y(), width, height, lift, &tree, mix(seed), unit(seed, 99));
        let grown = tree.grow_capped(&roots, &Vector3::y(), |c| glaciated_levels(&tree, glaciation, c.y));
        let wind = Vector3::new(wind.x, 0., wind.y);
        let wind = if wind.norm() > 0. { wind.normalize() } else { Vector3::zeros() };
        let r0 = roots.iter().map(|r| r.radius).fold(0., f64::max);
        let reach = reach(&roots, &at, &Vector3::y(), &tree);
        Cumulus { surface, max_density: 0.05, base, edge: 15., lean, wind, grown, r0, lift, dome: 0., reach, centre: at,
                  maturity: 1., glaciation, tower_top: Some(base + height), tree }
    }

    pub fn top(&self) -> f64 {
        if let Some(t) = self.tower_top {
            return t;
        }
        let largest = self.r0 * (1. + self.dome);
        self.base + self.lift * largest + self.tree.extent(largest)
    }

    /// The box holding all of it: its spheres' reach, its lean, and its surface's margin.
    pub fn bounds(&self) -> crate::shapes::bbox::BBox {
        let m = self.surface.margin();
        let side = self.reach + self.lean * (self.top() - self.base) + m;
        crate::shapes::bbox::BBox::new(
            Vector3::new(self.centre.x - side, self.base - m, self.centre.z - side),
            Vector3::new(self.centre.x + side, self.top() + m, self.centre.z + side),
        )
    }

    /// The fraction (0 to 1) of the densest cloud at `p`.
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y <= self.base {
            return 0.;
        }
        let q = self.surface.frayed(p);
        let lean = self.wind * (self.lean * (p.y - self.base));
        let ice = glaciated(self.glaciation, p.y);
        let soft = 1. - self.maturity.clamp(0., 1.);
        let edge = self.edge * (1. + 9. * ice);
        // Deep enough for the softest edge.
        let enough = edge * (1. + 24. * soft * soft);
        let depth = self.grown.depth(&self.tree, &(q - lean), enough);
        let m = self.surface.margin();
        if depth <= -m {
            return 0.;
        }
        let billows = if depth < enough { self.surface.billowing(&q) * (1. - ice) } else { 0. };
        self.surface.condensed(depth + billows, edge, self.maturity, &q) * (1. - 0.5 * ice)
            * smoothstep(self.base, self.base + 0.3 * self.edge, p.y)
    }
}

fn smoothstep(lo: f64, hi: f64, v: f64) -> f64 {
    let t = ((v - lo) / (hi - lo)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

impl crate::volume::DensityField for Cumulus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.max_density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.max_density
    }
}

/// The direction turrets grow (`up`), and two unit vectors across it: computed once for a whole
/// tree rather than for every turret.
pub struct Frame {
    pub up: Vector3<f64>,
    pub t1: Vector3<f64>,
    pub t2: Vector3<f64>,
}

impl Frame {
    pub fn new(up: &Vector3<f64>) -> Frame {
        let (t1, t2) = basis(up);
        Frame { up: *up, t1, t2 }
    }
}

/// Two unit vectors perpendicular to the unit `up` and each other.
fn basis(up: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if up.x.abs() < 0.9 { Vector3::x() } else { Vector3::z() };
    let t1 = up.cross(&a).normalize();
    (t1, up.cross(&t1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_spheres(c: &Cauliflower, s: &Sphere, level: usize, up: &Vector3<f64>, out: &mut Vec<(Sphere, usize)>) {
        all_spheres_of(c, s, level, up, out, &[*s], 0);
    }

    fn all_spheres_of(c: &Cauliflower, s: &Sphere, level: usize, up: &Vector3<f64>, out: &mut Vec<(Sphere, usize)>, roots: &[Sphere], from: usize) {
        out.push((*s, level));
        if level < c.levels {
            for k in 0..c.child_count(level) {
                let child = c.child(s, k, c.child_count(level), &Frame::new(up));
                if !Cauliflower::buried(&child, roots, from) {
                    all_spheres_of(c, &child, level + 1, up, out, roots, from);
                }
            }
        }
    }

    #[test]
    fn test_grown_tree_matches_generated() {
        let c = Cauliflower::cumulus();
        let up = Vector3::y();
        let roots = body(&Vector3::zeros(), &up, 400., 900., 0.35, 0.8, 0.5, 13, 0.7);
        let grown = c.grow(&roots, &up);
        let mut rng = crate::geometry::Pcg32::new(3, 0);
        for _ in 0..3000 {
            let p = Vector3::new(rng.next_f64() - 0.5, rng.next_f64(), rng.next_f64() - 0.5) * 3000.;
            for enough in [15., f64::INFINITY] {
                let (a, b) = (c.depth(&roots, &up, &p, enough), grown.depth(&c, &p, enough));
                assert!((a - b).abs() < 1e-9, "{} vs {}", a, b);
            }
        }
    }

    #[test]
    fn test_buried_turrets_are_left_out_exactly() {
        // A body of several roots: the search agrees with every sphere not buried, and some are.
        let c = Cauliflower::cumulus();
        let up = Vector3::y();
        let roots = body(&Vector3::zeros(), &up, 400., 900., 0.35, 0.8, 0.5, 11, 0.3);
        assert!(roots.len() > 3);
        let mut spheres = vec![];
        let mut all = 0;
        for (i, r) in roots.iter().enumerate() {
            all_spheres_of(&c, r, 0, &up, &mut spheres, &roots, i);
            let mut every = vec![];
            all_spheres_of(&c, r, 0, &up, &mut every, &[*r], 0);
            all += every.len();
        }
        assert!(spheres.len() < all, "nothing was buried: {} of {}", spheres.len(), all);
        let mut rng = crate::geometry::Pcg32::new(2, 0);
        for _ in 0..2000 {
            let p = Vector3::new(rng.next_f64() - 0.5, rng.next_f64(), rng.next_f64() - 0.5) * 3000.;
            let brute = spheres.iter().map(|(s, _)| s.radius - (p - s.centre).norm()).fold(f64::NEG_INFINITY, f64::max);
            let got = c.depth(&roots, &up, &p, f64::INFINITY);
            assert!((brute - got).abs() < 1e-9, "{} vs {}", got, brute);
        }
    }

    #[test]
    fn test_depth_is_the_unions_and_the_search_exact() {
        // The pruned search gives the same depth as checking every sphere.
        let c = Cauliflower::cumulus();
        let up = Vector3::y();
        let root = Sphere { centre: Vector3::new(0., 500., 0.), radius: 500., id: 7 };
        let mut spheres = vec![];
        all_spheres(&c, &root, 0, &up, &mut spheres);
        let mut rng = crate::geometry::Pcg32::new(1, 0);
        for _ in 0..3000 {
            let p = Vector3::new(rng.next_f64() - 0.5, rng.next_f64(), rng.next_f64() - 0.5) * 2400. + Vector3::new(0., -100., 0.);
            let brute = spheres.iter().map(|(s, _)| s.radius - (p - s.centre).norm()).fold(f64::NEG_INFINITY, f64::max);
            let got = c.depth(&[root], &up, &p, f64::INFINITY);
            assert!((brute - got).abs() < 1e-9, "{} vs {} at {}", got, brute, p);
            // Capped at `enough`, but exact below it.
            let capped = c.depth(&[root], &up, &p, 30.);
            assert!((capped - brute.min(30.)).abs() < 1e-9 || (brute >= 30. && capped == 30.));
        }
        // Every sphere lies within the reach computed for its level's subtree.
        for (s, level) in &spheres {
            assert!((s.centre - root.centre).norm() + s.radius <= c.extent(root.radius) + 1e-9, "level {}", level);
        }
    }

    #[test]
    fn test_turrets_grow_upwards() {
        // Most of the spheres' volume is above the root's centre, and the finest are small.
        let c = Cauliflower::cumulus();
        let up = Vector3::y();
        let root = Sphere { centre: Vector3::zeros(), radius: 500., id: 3 };
        let mut spheres = vec![];
        all_spheres(&c, &root, 0, &up, &mut spheres);
        let (above, below) = spheres.iter().skip(1).fold((0., 0.), |(a, b), (s, _)| {
            let v = s.radius.powi(3);
            if s.centre.y > 0. { (a + v, b) } else { (a, b + v) }
        });
        assert!(above > 3. * below, "{} {}", above, below);
        let finest = spheres.iter().filter(|(_, l)| *l == c.levels).map(|(s, _)| s.radius).fold(0., f64::max);
        assert!(finest <= 500. * (c.ratio * (1. + c.spread) * TOP_BOOST).powi(c.levels as i32) + 1e-9 && finest < 0.3 * 500., "{}", finest);
    }
}
