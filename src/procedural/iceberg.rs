//! # Icebergs and ice floes
//!
//! ## Icebergs
//!
//! An iceberg is a piece of glacier calved into the sea. Ice of density ρ_i floats in sea water
//! of density ρ_w ≈ 1025 kg m⁻³ with the fraction 1 - ρ_i / ρ_w of its volume above the water:
//! about a tenth for solid ice (917 kg m⁻³), more for a berg capped with lighter firn, which is
//! why the default density here is 900 (an eighth). The waterline is found from the shape: the
//! berg is star-shaped about its centre, its surface at distance R(ω) in each direction ω of a
//! geodesic sphere, and the volume above a plane y = h is
//!
//! ```text
//! V(h) = Σ_ω Δω ∫ r² [r ω_y > h] dr      (0 < r < R(ω))
//! ```
//!
//! each integral in closed form; h is found by bisection so that V(h) / V(-∞) is that fraction.
//!
//! Shapes follow the classes of the International Ice Patrol and the Canadian Ice Service
//! (eg. MANICE, the Manual of Standard Procedures for Observing and Reporting Ice Conditions,
//! Environment Canada, 2005, section 4.5): **tabular** (flat-topped, horizontally banded, at
//! least five times as long as high, broken from an ice shelf or floating glacier tongue),
//! **blocky** (flat top, steep vertical sides), **dome** (smooth and rounded), **pinnacle**
//! (one or more spires), **wedge** (one steep face, the top sloping away from it), and
//! **drydock** (eroded into a U, a channel between two pillars). Each is a superellipsoid,
//! (|x/a|^p + |y/b|^p + |z/c|^p)^(1/p) = 1 (p near 2 rounded, larger boxy), cut and shaped:
//!
//! - a few fracture planes, their edges rounded, for the faces where it broke away;
//! - erosion: lumps, and vertical flutes on the walls where meltwater runs down;
//! - a wave-cut notch at the waterline, where the sea melts it fastest (the overhang that
//!   eventually breaks off as a growler); and a slight tilt, but for tabular bergs.
//!
//! The berg's surface (`IcebergMaterial`) is `material::snow`'s: mostly the white, bubble-rich
//! ice of a glacier, snow on the gentler upward surfaces, veins of blue ice where meltwater
//! refroze in crevasses (thin layers, all in one plane through the berg), and a band of glossy,
//! bubble-poor glacier ice where the waves wash it.
//!
//! ## Ice floes
//!
//! Sea ice breaks into floes, from metres to kilometres across, their sizes following a power
//! law: many small floes, few large (Donald K. Perovich and Kathleen F. Jones, "The seasonal
//! evolution of sea ice floe size distribution", JGR Oceans 119, 2014,
//! doi:10.1002/2013JC009333, find exponents near 2 for the number per unit area). Floes are
//! placed largest first, each where it doesn't touch those placed before, until the
//! `concentration` (the fraction of the sea's surface covered, as ice charts report it) is
//! reached. Each is a slab of an irregular outline, its top edge rounded, floating with a tenth
//! of its thickness above the water; snow lies on top, with patches of bare ice, and the sides
//! are white ice.
use crate::geometry::{rand, Pcg32};
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::snow::{Ice, IceStyle, Snow, SnowStyle};
use crate::na::{Rotation3, Vector3};
use crate::noise::SeededNoise;
use crate::procedural::nubis_clouds::smoothstep;
use crate::procedural::rock::icosphere;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::mesh::IndexedMesh;
use std::f64::consts::{PI, TAU};

/// Density of sea water, kg/m³.
pub const SEA_WATER_DENSITY: f64 = 1025.;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind { Tabular, Blocky, Dome, Pinnacle, Wedge, Drydock }

impl Kind {
    pub fn named(name: &str) -> Kind {
        match name {
            "tabular" => Kind::Tabular,
            "blocky" => Kind::Blocky,
            "dome" => Kind::Dome,
            "pinnacle" => Kind::Pinnacle,
            "wedge" => Kind::Wedge,
            "drydock" => Kind::Drydock,
            _ => panic!("Unknown iceberg shape '{}': expected tabular, blocky, dome, pinnacle, wedge or drydock", name),
        }
    }
}

/// An iceberg: its shape, size (m, its length) and makeup.
#[derive(Clone, Debug)]
pub struct Iceberg {
    pub kind: Kind,
    pub size: f64,
    /// Bulk density, kg/m³.
    pub density: f64,
    /// Geodesic subdivisions of the mesh.
    pub detail: u32,
    pub seed: u64,
}

/// Polynomial smooth minimum: min(a, b), the corner rounded over a width `k`.
fn smin(a: f64, b: f64, k: f64) -> f64 {
    if k <= 0. { return a.min(b) }
    let h = (k - (a - b).abs()).max(0.) / k;
    a.min(b) - h * h * k / 4.
}

impl Iceberg {
    pub fn new(kind: Kind, size: f64, seed: u64) -> Iceberg {
        Iceberg { kind, size, density: 900., detail: 6, seed }
    }

    /// The fraction of the volume above water.
    pub fn freeboard_fraction(&self) -> f64 {
        (1. - self.density / SEA_WATER_DENSITY).clamp(0.01, 0.9)
    }

    /// Semi-axes (in units of the length) and the superellipsoid's exponent.
    fn body(&self, rng: &mut Pcg32) -> (Vector3<f64>, f64) {
        let mut j = |a: f64| a * (0.85 + 0.3 * rng.next_f64());
        // Half the depth from top to keel: most of a berg is under water, so for the freeboard
        // (height above water, H) to be the fraction of the length (L) that bergs of each kind
        // show, a prism floating with an eighth of it above water must be about 8 H deep.
        match self.kind {
            Kind::Tabular => (Vector3::new(0.5, j(0.3), j(0.4)), 8.),
            Kind::Blocky => (Vector3::new(0.5, j(1.1), j(0.42)), 10.),
            Kind::Dome => (Vector3::new(0.5, j(0.7), j(0.42)), 2.2),
            Kind::Pinnacle => (Vector3::new(0.5, j(0.6), j(0.42)), 3.),
            Kind::Wedge => (Vector3::new(0.5, j(0.9), j(0.38)), 8.),
            Kind::Drydock => (Vector3::new(0.5, j(1.1), j(0.36)), 8.),
        }
    }

    /// Vertex positions and triangles in the berg's frame, about its centre, in metres.
    fn shape(&self) -> IndexedMesh {
        let mut rng = Pcg32::new(self.seed, 23);
        let noise = SeededNoise::new(self.seed.wrapping_add(5));
        let (axes, p) = self.body(&mut rng);
        let rounded = self.kind == Kind::Dome;
        // Fracture planes: the faces where it broke away.
        let (fractures, steep) = match self.kind {
            Kind::Dome => (0, 0.),
            Kind::Tabular => (3, 0.1),
            Kind::Pinnacle => (7, 0.5),
            _ => (7, 1.4),
        };
        let planes: Vec<(Vector3<f64>, f64)> = (0..fractures).map(|_| {
            // Near-vertical faces, and (but on tabular bergs) oblique ones cutting the top's
            // edges; none from above on a pinnacle berg, which would cut its spires.
            let a = rng.next_f64() * TAU;
            let n = Vector3::new(a.cos(), steep * (rng.next_f64() - 0.35), a.sin()).normalize();
            let support = axes.component_mul(&n).norm();
            (n, support * (1. - 0.3 * rng.next_f64()))
        }).collect();
        // The wedge's slope, falling from its steep face.
        let slope = Vector3::new(0.55, 0.85, 0.1 * (rng.next_f64() - 0.5)).normalize();
        let channel = 0.12 + 0.05 * rng.next_f64();
        // A pinnacle berg's spires: where they rise, how wide and how high (in the length).
        let spires: Vec<(f64, f64, f64, f64)> = (0..2 + (rng.next_f64() * 2.) as usize).map(|k| {
            let a = rng.next_f64() * TAU;
            let r = 0.25 * rng.next_f64();
            (r * a.cos(), r * a.sin(), 0.07 + 0.07 * rng.next_f64(), if k == 0 { 0.45 } else { 0.15 + 0.25 * rng.next_f64() })
        }).collect();

        let (directions, triangles) = icosphere(self.detail);
        let positions = directions.iter().map(|d| {
            let r0 = (d.x.abs() / axes.x).powf(p) + (d.y.abs() / axes.y).powf(p) + (d.z.abs() / axes.z).powf(p);
            let q = d * r0.powf(-1. / p);
            let mut s: f64 = 1.;
            match self.kind {
                Kind::Wedge => {
                    let along = q.dot(&slope);
                    if along > 0. { s = smin(s, 0.5 * axes.component_mul(&slope).norm() / along, 0.05); }
                }
                Kind::Drydock if q.y > 0. => {
                    // A channel between two pillars, its floor a little above the water.
                    let floor = 0.12 * axes.y;
                    let f = 1. - smoothstep(channel * 0.75, channel * 1.25, q.x.abs());
                    s = s * (1. - f) + smin(s, floor / q.y, 0.03) * f;
                }
                Kind::Pinnacle if q.y > 0. => {
                    // Spires rising from the top, pushing the surface up above them.
                    let up = spires.iter().map(|&(x, z, w, h)| {
                        let d2 = (q.x - x).powi(2) + (q.z - z).powi(2);
                        h * (-d2 / (w * w)).exp()
                    }).fold(0., f64::max);
                    s *= 1. + up / q.y.max(0.05 * axes.y) * smoothstep(0.4, 0.8, q.y / axes.y);
                }
                _ => {}
            }
            for (n, offset) in &planes {
                let along = q.dot(n);
                if along > 0. { s = smin(s, offset / along, 0.02); }
            }
            let q = q * s.max(0.05);
            // Erosion: lumps, and vertical flutes on the walls (noise stretched vertically).
            let lumps = noise.fbm(q * 4., 4);
            let flutes = noise.ridged(Vector3::new(q.x * 18., q.y * 2., q.z * 18.), 3) - 0.4;
            let wall = 1. - d.y.abs();
            let h = if rounded { 0.02 * lumps } else { 0.025 * lumps + 0.012 * flutes * wall };
            (q + q.normalize() * h) * self.size
        }).collect();
        IndexedMesh { positions, normals: vec![], triangles, texcoords: vec![] }
    }

    /// The height of the waterline in the berg's frame, for vertices on rays from the origin:
    /// where the fraction above it of the volume they enclose is the freeboard's.
    pub fn waterline(positions: &[Vector3<f64>], fraction: f64) -> f64 {
        // Each vertex stands for an equal solid angle of a geodesic sphere.
        let above = |h: f64| -> f64 {
            positions.iter().map(|v| {
                let r = v.norm();
                if r <= 0. { return 0.; }
                let dy = v.y / r;
                let r3 = r * r * r;
                if h >= 0. {
                    if dy <= 0. { return 0.; }
                    let r_min = h / dy;
                    if r_min >= r { 0. } else { (r3 - r_min.powi(3)) / 3. }
                } else if dy >= 0. {
                    r3 / 3.
                } else {
                    let r_max = h / dy;
                    r.min(r_max).powi(3) / 3.
                }
            }).sum()
        };
        let total: f64 = positions.iter().map(|v| v.norm().powi(3) / 3.).sum();
        let r_max = positions.iter().map(|v| v.norm()).fold(0., f64::max);
        let (mut lo, mut hi) = (-r_max, r_max);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if above(mid) / total > fraction { lo = mid } else { hi = mid }
        }
        0.5 * (lo + hi)
    }

    /// The mesh floating with its waterline at `sea_level`, centred over `(x, z)`, turned by
    /// `yaw` (radians).
    pub fn mesh(&self, x: f64, z: f64, sea_level: f64, yaw: f64) -> IndexedMesh {
        let mut mesh = self.shape();
        let mut rng = Pcg32::new(self.seed, 29);
        // Bergs other than tabular ones float tilted, having rolled as they melted.
        let tilt = if self.kind == Kind::Tabular { 0.01 } else { 0.12 };
        let rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), yaw)
            * Rotation3::from_euler_angles(tilt * (rng.next_f64() - 0.5), 0., tilt * (rng.next_f64() - 0.5));
        for p in mesh.positions.iter_mut() { *p = rotation * *p; }
        let h = Iceberg::waterline(&mesh.positions, self.freeboard_fraction());
        // The wave-cut notch: the sea melts a groove a metre or two high at the waterline.
        let depth = (0.015 * self.size).min(2.5);
        let height = (0.012 * self.size).clamp(0.5, 2.);
        for p in mesh.positions.iter_mut() {
            let k = (-((p.y - h - 0.3 * height) / height).powi(2)).exp();
            let across = (p.x * p.x + p.z * p.z).sqrt();
            if k > 1e-3 && across > depth {
                let s = 1. - depth * k / across;
                p.x *= s;
                p.z *= s;
            }
        }
        let lift = Vector3::new(x, sea_level - h, z);
        for p in mesh.positions.iter_mut() { *p += lift; }
        mesh
    }
}

/// An iceberg's surface: white ice, snow on top, veins of blue ice, glossy where waves wash.
pub struct IcebergMaterial {
    pub sea_level: f64,
    white: Ice,
    wet: Ice,
    blue: Ice,
    snow: Snow,
    /// The veins' plane, through the berg, and their spacing (m).
    vein_normal: Vector3<f64>,
    vein_spacing: f64,
    noise: SeededNoise,
}

impl IcebergMaterial {
    pub fn new(sea_level: f64, size: f64, seed: u64) -> IcebergMaterial {
        let mut rng = Pcg32::new(seed, 31);
        let a = rng.next_f64() * TAU;
        let tilt = 0.3 + 0.9 * rng.next_f64();
        IcebergMaterial {
            sea_level,
            white: Ice::new(IceStyle::preset("white")),
            wet: Ice::new(IceStyle { roughness: 0.02, ..IceStyle::preset("glacier") }),
            blue: Ice::new(IceStyle::preset("blue")),
            snow: Snow::new(SnowStyle::preset("settled"), 1., seed),
            vein_normal: Vector3::new(tilt.sin() * a.cos(), tilt.cos(), tilt.sin() * a.sin()),
            vein_spacing: (0.15 * size).clamp(3., 30.),
            noise: SeededNoise::new(seed.wrapping_add(17)),
        }
    }

    /// The shares at `p`, with outward normal `n`, of snow, blue veins and wave-washed ice.
    pub fn shares(&self, p: &Vector3<f64>, n: &Vector3<f64>) -> (f64, f64, f64) {
        let height = p.y - self.sea_level;
        let snow = smoothstep(0.72, 0.9, n.y + 0.2 * self.noise.fbm(p / 6., 3)) * smoothstep(1., 3., height);
        let v = p.dot(&self.vein_normal) / self.vein_spacing + 0.6 * self.noise.fbm(p / 25., 3);
        let d = (v.rem_euclid(1.) - 0.5).abs();
        let vein = (1. - smoothstep(0.015, 0.05, d)) * smoothstep(-0.1, 0.2, self.noise.noise(p / 40.));
        let wet = 1. - smoothstep(0.5, 3., height + 0.8 * self.noise.noise(p / 4.));
        (snow, vein, wet)
    }
}

impl MaterialModel for IcebergMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = &intersection.facing(r);
        let (snow, vein, wet) = self.shares(&i.point, &intersection.normal);
        let f = crate::material::functions::scatter_bsdf;
        if rand() < snow {
            f(&self.snow.bsdf(&i.point, &i.normal, Some(s)), r, &i.point, s)
        } else if rand() < vein {
            f(&self.blue.bsdf(&i.normal, Some(s)), r, &i.point, s)
        } else if rand() < wet {
            f(&self.wet.bsdf(&i.normal, Some(s)), r, &i.point, s)
        } else {
            f(&self.white.bsdf(&i.normal, Some(s)), r, &i.point, s)
        }
    }
}

/// A field of ice floes on the sea.
#[derive(Clone, Debug)]
pub struct FloeField {
    /// Centre of the field (x, z), and its width (x) and depth (z), m.
    pub center: (f64, f64),
    pub extent: (f64, f64),
    /// The fraction of the sea covered.
    pub concentration: f64,
    /// Smallest and largest floes, m across.
    pub min_size: f64,
    pub max_size: f64,
    /// Ice thickness, m.
    pub thickness: f64,
    pub sea_level: f64,
    pub seed: u64,
}

/// One floe: its centre (x, z) and radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Floe { pub x: f64, pub z: f64, pub radius: f64 }

impl FloeField {
    /// Floes placed largest first, none touching, until the concentration is reached.
    pub fn floes(&self) -> Vec<Floe> {
        let mut rng = Pcg32::new(self.seed, 37);
        let area = self.extent.0 * self.extent.1;
        let target = self.concentration.clamp(0., 0.95) * area;
        // Diameters from a power law, number density ∝ d⁻², above min_size: d = d_min / u.
        let mut radii = vec![];
        let mut sum = 0.;
        while sum < 1.15 * target && radii.len() < 200_000 {
            let d = (self.min_size / (1. - rng.next_f64() * (1. - self.min_size / self.max_size))).min(self.max_size);
            let r = d / 2.;
            // A floe's outline covers about 0.7 of its circle.
            sum += 0.7 * PI * r * r;
            radii.push(r);
        }
        radii.sort_by(|a, b| b.total_cmp(a));
        // A grid of cells as wide as the largest floe, for the overlap test.
        let cell = 2. * radii.first().copied().unwrap_or(1.);
        let nx = (self.extent.0 / cell).ceil().max(1.) as usize;
        let nz = (self.extent.1 / cell).ceil().max(1.) as usize;
        let mut grid: Vec<Vec<usize>> = vec![vec![]; nx * nz];
        let mut placed: Vec<Floe> = vec![];
        let mut covered = 0.;
        let (x0, z0) = (self.center.0 - self.extent.0 / 2., self.center.1 - self.extent.1 / 2.);
        for r in radii {
            if covered >= target { break; }
            for _ in 0..30 {
                let x = x0 + rng.next_f64() * self.extent.0;
                let z = z0 + rng.next_f64() * self.extent.1;
                let (ci, cj) = (((x - x0) / cell) as usize, ((z - z0) / cell) as usize);
                let gap = 0.05 * r + 0.1;
                let clear = (ci.saturating_sub(1)..=(ci + 1).min(nx - 1)).all(|i| {
                    (cj.saturating_sub(1)..=(cj + 1).min(nz - 1)).all(|j| {
                        grid[j * nx + i].iter().all(|&k| {
                            let f = &placed[k];
                            (f.x - x).hypot(f.z - z) > f.radius + r + gap
                        })
                    })
                });
                if clear {
                    grid[cj.min(nz - 1) * nx + ci.min(nx - 1)].push(placed.len());
                    placed.push(Floe { x, z, radius: r });
                    covered += 0.7 * PI * r * r;
                    break;
                }
            }
        }
        placed
    }

    /// The floes as one mesh.
    pub fn mesh(&self) -> IndexedMesh {
        let floes = self.floes();
        let mut positions = vec![];
        let mut triangles = vec![];
        let noise = SeededNoise::new(self.seed.wrapping_add(3));
        let mut rng = Pcg32::new(self.seed, 41);
        let fraction = 0.1;
        for f in &floes {
            let t = self.thickness * (0.7 + 0.6 * rng.next_f64());
            let top = self.sea_level + fraction * t;
            let bottom = top - t;
            let bevel = (0.25 * t).min(0.3 * f.radius);
            let n = ((f.radius * 2.).sqrt() * 6.).clamp(10., 48.) as usize;
            let phase = rng.next_f64() * TAU;
            // The outline: a jagged circle.
            let outline: Vec<(f64, f64)> = (0..n).map(|k| {
                let a = phase + TAU * (k as f64 + 0.4 * (rng.next_f64() - 0.5)) / n as f64;
                let u = Vector3::new(a.cos(), 0., a.sin());
                // Within the floe's circle, which placing it kept clear of the others.
                let w = (0.84 + 0.2 * noise.fbm(u * 1.7 + Vector3::new(f.x, 0., f.z) * 0.05, 3) + 0.05 * (rng.next_f64() - 0.5)).min(1.);
                (a.cos() * f.radius * w, a.sin() * f.radius * w)
            }).collect();
            let ring = |positions: &mut Vec<Vector3<f64>>, inset: f64, y: f64| -> usize {
                let start = positions.len();
                for &(x, z) in &outline {
                    let len = x.hypot(z);
                    let s = (len - inset).max(0.1 * len) / len;
                    positions.push(Vector3::new(f.x + x * s, y, f.z + z * s));
                }
                start
            };
            // Top cap: centre and an inset ring.
            let c = positions.len();
            positions.push(Vector3::new(f.x, top, f.z));
            let cap = ring(&mut positions, bevel, top);
            for k in 0..n { triangles.push([c, cap + (k + 1) % n, cap + k]); }
            // Sides, smoothly shaded: the inset ring again, the rounded edge, the bottom edge.
            let a = ring(&mut positions, bevel, top);
            let b = ring(&mut positions, 0., top - bevel);
            let d = ring(&mut positions, 0., bottom);
            for (u, v) in [(a, b), (b, d)] {
                for k in 0..n {
                    let k1 = (k + 1) % n;
                    triangles.push([u + k, u + k1, v + k1]);
                    triangles.push([u + k, v + k1, v + k]);
                }
            }
            // Bottom cap.
            let e = ring(&mut positions, 0., bottom);
            let cb = positions.len();
            positions.push(Vector3::new(f.x, bottom, f.z));
            for k in 0..n { triangles.push([cb, e + k, e + (k + 1) % n]); }
        }
        IndexedMesh { positions, normals: vec![], triangles, texcoords: vec![] }
    }
}

/// Floes' surfaces: snow on top with patches of bare ice, white ice at the sides.
pub struct FloeMaterial {
    snow: Snow,
    ice: Ice,
    noise: SeededNoise,
}

impl FloeMaterial {
    pub fn new(seed: u64) -> FloeMaterial {
        FloeMaterial {
            snow: Snow::new(SnowStyle::preset("settled"), 1., seed),
            ice: Ice::new(IceStyle { roughness: 0.15, ..IceStyle::preset("white") }),
            noise: SeededNoise::new(seed.wrapping_add(13)),
        }
    }

    /// The share of snow at `p`, with outward normal `n`.
    pub fn snow(&self, p: &Vector3<f64>, n: &Vector3<f64>) -> f64 {
        smoothstep(0.8, 0.95, n.y) * smoothstep(-0.35, -0.15, self.noise.fbm(p / 3., 4))
    }
}

impl MaterialModel for FloeMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = &intersection.facing(r);
        let f = crate::material::functions::scatter_bsdf;
        if rand() < self.snow(&i.point, &intersection.normal) {
            f(&self.snow.bsdf(&i.point, &i.normal, Some(s)), r, &i.point, s)
        } else {
            f(&self.ice.bsdf(&i.normal, Some(s)), r, &i.point, s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The volume above the waterline (by Monte Carlo over the mesh) is the freeboard's share.
    #[test]
    fn test_icebergs_float_with_a_tenth_above_water() {
        for kind in [Kind::Tabular, Kind::Dome, Kind::Pinnacle, Kind::Drydock] {
            let mut berg = Iceberg::new(kind, 100., 3);
            berg.detail = 4;
            let mesh = berg.mesh(0., 0., 0., 0.3);
            use crate::shapes::geometry::Geometry;
            let lo = mesh.positions.iter().fold(Vector3::repeat(f64::INFINITY), |a, p| a.inf(p));
            let hi = mesh.positions.iter().fold(Vector3::repeat(f64::NEG_INFINITY), |a, p| a.sup(p));
            let geometry = crate::shapes::mesh::SmoothMesh::from_indexed(&[mesh]);
            let mut rng = Pcg32::new(1, 1);
            let (mut inside, mut above) = (0, 0);
            for _ in 0..20000 {
                let p = lo + (hi - lo).component_mul(&Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()));
                // Inside if a ray upwards crosses the surface an odd number of times.
                let mut crossings = 0;
                let mut o = p;
                while let Some(hit) = geometry.intersects(&Ray { ro: o, rd: Vector3::y() }) {
                    crossings += 1;
                    o = hit.point + Vector3::y() * 1e-6;
                    if crossings > 50 { break; }
                }
                if crossings % 2 == 1 {
                    inside += 1;
                    if p.y > 0. { above += 1; }
                }
            }
            let f = above as f64 / inside as f64;
            assert!((f - berg.freeboard_fraction()).abs() < 0.03, "{:?}: {} above", kind, f);
            assert!(hi.y > 0. && lo.y < 0.);
        }
    }

    #[test]
    fn test_waterline_of_a_sphere() {
        // A sphere floating with half its volume above water floats at its centre.
        let (dirs, _) = icosphere(5);
        let h = Iceberg::waterline(&dirs, 0.5);
        assert!(h.abs() < 0.01, "{}", h);
        // With less above, the waterline rises through it.
        assert!(Iceberg::waterline(&dirs, 0.1) > 0.4);
    }

    #[test]
    fn test_floes_cover_the_concentration_without_overlapping() {
        let field = FloeField { center: (0., 0.), extent: (400., 300.), concentration: 0.6, min_size: 2., max_size: 60., thickness: 1., sea_level: 0., seed: 2 };
        let floes = field.floes();
        let covered: f64 = floes.iter().map(|f| 0.7 * PI * f.radius * f.radius).sum();
        assert!(covered > 0.45 * 400. * 300., "{}", covered / (400. * 300.));
        for (i, a) in floes.iter().enumerate() {
            for b in &floes[i + 1..] {
                assert!((a.x - b.x).hypot(a.z - b.z) > a.radius + b.radius, "{:?} {:?}", a, b);
            }
        }
        let mesh = field.mesh();
        assert!(mesh.positions.iter().all(|p| p.y <= 0.2 && p.y >= -1.4));
    }
}
