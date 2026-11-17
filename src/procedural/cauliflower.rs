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
    /// carries turrets of 225, 100, 45 and 20 m.
    pub fn cumulus() -> Cauliflower {
        Cauliflower::new(4, 0.45, 8, 0.35, 0.9)
    }

    /// The farthest any sphere of a subtree reaches from its root's centre, over the root's
    /// radius, for a root at each level: a child of radius at most r (1 + spread) ratio sits
    /// with its centre r - embed × its radius from its parent's, and reaches its own reach
    /// beyond that.
    fn reaches(&self) -> Vec<f64> {
        let mut reach = vec![1.; self.levels + 1];
        let largest = self.ratio * (1. + self.spread);
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
    fn child(&self, s: &Sphere, k: usize, up: &Vector3<f64>) -> Sphere {
        let id = mix(s.id.wrapping_mul(31).wrapping_add(k as u64 + 1));
        // A direction uniform over the sphere, then leant upwards; turned back up if it would
        // still point downwards (turrets don't hang beneath a cloud).
        let z = 2. * unit(id, 0) - 1.;
        let a = std::f64::consts::TAU * unit(id, 1);
        let r = (1. - z * z).max(0.).sqrt();
        let mut d = Vector3::new(r * a.cos(), z, r * a.sin());
        // In the frame where y is `up`.
        let (t1, t2) = basis(up);
        d = t1 * d.x + up * d.y + t2 * d.z;
        d = (d + up * self.rise).normalize();
        let below = d.dot(up);
        if below < -0.05 {
            d -= up * (2. * below);
        }
        let radius = s.radius * self.ratio * (1. + self.spread * (2. * unit(id, 2) - 1.));
        Sphere { centre: s.centre + d * (s.radius - self.embed * radius), radius, id }
    }

    /// How deep `p` is inside the union of the trees grown from `roots` (metres; negative
    /// outside), along `up`; any depth over `enough` is reported as `enough`, which is all
    /// that's needed and cuts the search short.
    pub fn depth(&self, roots: &[Sphere], up: &Vector3<f64>, p: &Vector3<f64>, enough: f64) -> f64 {
        let mut best = f64::NEG_INFINITY;
        for root in roots {
            self.visit(root, 0, up, p, enough, &mut best);
            if best >= enough {
                return enough;
            }
        }
        best
    }

    fn visit(&self, s: &Sphere, level: usize, up: &Vector3<f64>, p: &Vector3<f64>, enough: f64, best: &mut f64) {
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
            let c = self.child(s, k, up);
            self.visit(&c, level + 1, up, p, enough, best);
            if *best >= enough {
                return;
            }
        }
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
        out.push((*s, level));
        if level < c.levels {
            for k in 0..c.child_count(level) {
                all_spheres(c, &c.child(s, k, up), level + 1, up, out);
            }
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
        assert!(finest <= 500. * (c.ratio * (1. + c.spread)).powi(c.levels as i32) + 1e-9 && finest < 60., "{}", finest);
    }
}
