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
    fn child(&self, s: &Sphere, k: usize, n: usize, up: &Vector3<f64>) -> Sphere {
        let id = mix(s.id.wrapping_mul(31).wrapping_add(k as u64 + 1));
        let n = n.max(1) as f64;
        let low = -0.25 * self.rise.clamp(0., 4.);
        let jitter = |j: u64| (unit(id, j) - 0.5) / n;
        let z = (1. - (1. - low) * ((k as f64 + 0.5) / n + jitter(0))).clamp(low, 1.);
        let a = std::f64::consts::TAU * (0.381_966 * k as f64 + unit(s.id, 7) + jitter(1));
        let r = (1. - z * z).max(0.).sqrt();
        let (t1, t2) = basis(up);
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
    fn visit(&self, s: &Sphere, level: usize, up: &Vector3<f64>, p: &Vector3<f64>, enough: f64, best: &mut f64, roots: &[Sphere], from: usize) {
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
    roots: Vec<Sphere>,
    r0: f64,
    lift: f64,
    dome: f64,
    reach: f64,
    centre: Vector3<f64>,
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
        Cumulus { tree, surface, max_density: 0.05, base, edge: 15., lean, wind, roots, r0, lift, dome, reach, centre: at }
    }

    pub fn top(&self) -> f64 {
        let largest = self.r0 * (1. + self.dome);
        self.base + self.lift * largest + self.tree.extent(largest)
    }

    /// The box holding all of it: its spheres' reach, its lean, and its surface's margin.
    pub fn bounds(&self) -> crate::shapes::bbox::BBox {
        let m = self.surface.margin();
        let side = self.reach + self.lean * (self.top() - self.base) + m;
        crate::shapes::bbox::BBox::new(
            Vector3::new(self.centre.x - side, self.base - (1. - self.lift) * self.r0 * (1. + self.dome) - m, self.centre.z - side),
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
        let depth = self.depth_at(&(q - lean));
        let m = self.surface.margin();
        if depth <= -m {
            return 0.;
        }
        let billows = if depth < self.edge { self.surface.billowing(&q) } else { 0. };
        smoothstep(0., self.edge, depth + billows) * smoothstep(self.base, self.base + 0.3 * self.edge, p.y)
    }

    fn depth_at(&self, p: &Vector3<f64>) -> f64 {
        self.tree.depth(&self.roots, &Vector3::y(), p, self.edge)
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
                let child = c.child(s, k, c.child_count(level), up);
                if !Cauliflower::buried(&child, roots, from) {
                    all_spheres_of(c, &child, level + 1, up, out, roots, from);
                }
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
