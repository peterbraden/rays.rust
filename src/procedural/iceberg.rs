//! # Icebergs and ice floes
//!
//! ## Icebergs
//!
//! An iceberg is a piece of glacier calved into the sea. Ice of density ρ_i floats in sea water
//! of density ρ_w ≈ 1025 kg m⁻³ with the fraction 1 - ρ_i / ρ_w of its volume above the water:
//! about a tenth for solid ice (917 kg m⁻³), more for a berg capped with lighter firn, which is
//! why the default density here is 900 (an eighth). The waterline is found from the shape: the
//! volume above a plane y = h is, by the divergence theorem, the sum over the parts of the
//! mesh's triangles above it of the signed volumes of the tetrahedra they make with a point on
//! the plane (`Iceberg::volume_above`), and h is found by bisection so that the fraction above
//! is the freeboard's.
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
//! - cleavage: the faces broken along planes into a few large slabs, each Voronoi cell of the
//!   face cut back a little to a plane of its own, nearly in line with it, the steps between
//!   neighbours worn round over a couple of metres, the cells taller than wide
//!   (`Cleavage`); the mesh's normals are smoothed only across sharp edges, so the remaining
//!   corners stay crisp
//!   (`creased`); and the mesh is refined adaptively where the surface bends (`refine`), so
//!   the facets' edges are straight rather than serrated by the triangles they cross;
//! - for tabular bergs, an outline broken into bays, a gently rolling top, and the ice
//!   shelf's layers of firn and ice showing on the cliffs as horizontal grooves and as bands
//!   of whiter and bluer ice (in place of veins);
//! - erosion: a little lumpiness, fine vertical flutes on the walls where meltwater runs
//!   down, and melt scallops, dimples a metre or so across and a few centimetres deep;
//! - a wave-cut notch at the waterline, where the sea melts it fastest (the overhang that
//!   eventually breaks off as a growler), and under it the ram, a shelf of ice jutting out
//!   below the water, which glows turquoise through it; and a slight tilt, but for tabular
//!   bergs.
//!
//! The berg's surface (`IcebergMaterial`) is `material::snow`'s: the white, bubble-rich ice of
//! a glacier, bluer bubbly ice low on its walls (ice from deep in the glacier, or rolled up from
//! under water, not yet weathered white), snow on the gentler upward surfaces, veins of blue ice where meltwater
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
//! are white ice. Or the pieces are brash ice and bergy bits, broken from bergs: rounded lumps
//! of glacier ice, each floating at its own waterline, notched there and spreading in a
//! skirt just below, as the bergs are. With a scene's ocean, every piece rides its waves:
//! raised or lowered with the surface where it floats, and tipped with its slope.
use crate::geometry::{rand, Pcg32};
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::snow::{Ice, IceStyle, Snow, SnowStyle};
use crate::material::subsurface::{Coefficients, Subsurface};
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
        Iceberg { kind, size, density: 900., detail: 7, seed }
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
            Kind::Tabular => (Vector3::new(0.5, j(0.3), j(0.4)), 16.),
            Kind::Blocky => (Vector3::new(0.5, j(1.1), j(0.42)), 14.),
            Kind::Dome => (Vector3::new(0.5, j(0.7), j(0.42)), 2.2),
            Kind::Pinnacle => (Vector3::new(0.5, j(0.6), j(0.42)), 3.),
            Kind::Wedge => (Vector3::new(0.5, j(0.9), j(0.38)), 12.),
            Kind::Drydock => (Vector3::new(0.5, j(1.1), j(0.36)), 12.),
        }
    }

    /// The berg's surface in the direction `d` from its centre, in its own frame, in metres,
    /// and the starting directions and triangles to mesh it from: a geodesic sphere stretched
    /// to the body's proportions, so that its vertices spread evenly over the surface rather
    /// than evenly by angle (which would mesh a tall berg's walls coarsely).
    #[allow(clippy::type_complexity)]
    fn shape(&self, detail: u32) -> (impl Fn(&Vector3<f64>) -> (Vector3<f64>, f64, f64) + Sync + '_, Vec<Vector3<f64>>, Vec<[usize; 3]>) {
        let mut rng = Pcg32::new(self.seed, 23);
        let noise = SeededNoise::new(self.seed.wrapping_add(5));
        let (axes, p) = self.body(&mut rng);
        let rounded = self.kind == Kind::Dome;
        // Fracture planes: the faces where it broke away.
        let (fractures, steep) = match self.kind {
            Kind::Dome => (0, 0.),
            // A shelf's edge breaks back in bays and inlets.
            Kind::Tabular => (10, 0.05),
            Kind::Pinnacle => (7, 0.5),
            Kind::Blocky => (7, 0.4),
            _ => (7, 1.4),
        };
        let planes: Vec<(Vector3<f64>, f64)> = (0..fractures).map(|_| {
            // Near-vertical faces, and (but on tabular bergs) oblique ones cutting the top's
            // edges; none from above on a pinnacle berg, which would cut its spires.
            let a = rng.next_f64() * TAU;
            let n = Vector3::new(a.cos(), steep * (rng.next_f64() - 0.35), a.sin()).normalize();
            let support = axes.component_mul(&n).norm();
            let deepest = if self.kind == Kind::Tabular { 0.4 } else { 0.3 };
            (n, support * (1. - deepest * rng.next_f64()))
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

        // Conchoidal fracture at three scales (metres): calving scars a fifth of the berg
        // across, smaller breaks within them, and melt scallops a metre or so across.
        let big = (0.18 * self.size).clamp(5., 16.);
        let scoops = if rounded {
            // A worn piece: broad shallow hollows, and scallops.
            vec![
                Scoops { cell: Vector3::repeat(0.3 * self.size), depth: 0.07 * self.size, seed: self.seed.wrapping_add(101) },
                Scoops { cell: Vector3::new(1.3, 2., 1.3), depth: 0.1, seed: self.seed.wrapping_add(303) },
            ]
        } else {
            vec![
                Scoops { cell: Vector3::new(big, 1.8 * big, big), depth: 0.3 * big, seed: self.seed.wrapping_add(101) },
                Scoops { cell: Vector3::new(big, 1.7 * big, big) / 3.2, depth: 0.14 * big, seed: self.seed.wrapping_add(202) },
                Scoops { cell: Vector3::new(1.3, 2., 1.3), depth: 0.14, seed: self.seed.wrapping_add(303) },
            ]
        };
        // Crevasses: a few narrow, deep cracks, each in a vertical plane through the berg (its
        // horizontal normal, its offset from the centre, half-width and depth, in metres).
        let crevasses: Vec<(Vector3<f64>, f64, f64, f64)> = if rounded { vec![] } else {
            (0..3 + (rng.next_f64() * 3.) as usize).map(|_| {
                let a = rng.next_f64() * TAU;
                (Vector3::new(a.cos(), 0., a.sin()), (rng.next_f64() - 0.5) * 0.7 * self.size,
                    0.25 + 0.35 * rng.next_f64(), (0.4 + 0.6 * rng.next_f64()) * (0.5 * big).min(0.08 * self.size))
            }).collect()
        };
        // Roughly where the waterline will be (exactly, for a box), in the length: the detail
        // fades out a few metres under it, where it can't be seen.
        let waterline = axes.y * (1. - 2. * self.freeboard_fraction());

        let (directions, triangles) = icosphere(detail);
        let directions: Vec<Vector3<f64>> = directions.iter().map(|d| d.component_mul(&axes).normalize()).collect();
        let kind = self.kind;
        let size = self.size;
        let outline = std::sync::Arc::new(SeededNoise::new(self.seed.wrapping_add(9)));
        let crown_noise = outline.clone();
        // A shelf's top rolls, and it is crowned: snow lies deeper away from the edge, where it
        // calves. The factor its height is scaled by over (x, z).
        let crown = std::sync::Arc::new(move |q: &Vector3<f64>| -> f64 {
            let inland = 1. - ((q.x / axes.x).powi(2) + (q.z / axes.z).powi(2)).min(1.);
            1. + 0.1 * crown_noise.fbm(Vector3::new(q.x, 5., q.z) * 3., 3) + 0.14 * inland
        });
        let body_crown = crown.clone();
        // The body, cut and shaped, in the direction `d`.
        let body = move |d: &Vector3<f64>| -> Vector3<f64> {
            let r0 = (d.x.abs() / axes.x).powf(p) + (d.y.abs() / axes.y).powf(p) + (d.z.abs() / axes.z).powf(p);
            let mut q = d * r0.powf(-1. / p);
            if kind == Kind::Tabular {
                // An irregular outline, broken from the shelf's front, and a top that rolls
                // gently, as the shelf's snow surface did.
                let h = Vector3::new(d.x, 0., d.z);
                let wobble = 1. + 0.15 * outline.fbm(h * 2., 3) + 0.05 * outline.noise(h * 9. + Vector3::new(0., 3., 0.));
                q.x *= wobble;
                q.z *= wobble;
                if q.y > 0. {
                    q.y *= body_crown(&q);
                }
            }
            let mut s: f64 = 1.;
            if kind == Kind::Dome {
                // Lumpy, not a smooth dome: melted unevenly.
                s *= 1. + 0.3 * outline.fbm(d * 1.4, 3);
            }
            match kind {
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
                if along > 0. { s = smin(s, offset / along, 0.012); }
            }
            q * s.max(0.05)
        };
        let surface = move |d: &Vector3<f64>| -> (Vector3<f64>, f64, f64) {
            let q = body(d);
            // The face's normal, from the body's surface to either side: far enough (a few
            // metres) that it turns smoothly round the corners between fracture faces, so the
            // cuts made along it don't tear there.
            let (t1, t2) = crate::material::phase::orthonormal_basis(d);
            let e = (3. / size).min(0.03);
            let side = |t: Vector3<f64>| body(&(d + t * e).normalize()) - body(&(d - t * e).normalize());
            let normal = side(t1).cross(&side(t2))
                .try_normalize(1e-18)
                .map(|n| if n.dot(d) < 0. { -n } else { n })
                .unwrap_or(*d);
            // In metres from here. The top is snow: smooth, with gentle drifts. The walls are
            // broken ice.
            let m = q * size;
            let top = smoothstep(0.55, 0.85, normal.y);
            let wall = 1. - 0.9 * top;
            let seen = smoothstep(waterline - 12. / size, waterline - 4. / size, q.y);
            // More broken in some stretches of the wall than others.
            let broken = 0.35 + 1.3 * smoothstep(-0.3, 0.4, noise.fbm(m / (3. * big), 2));
            let cuts: Vec<f64> = scoops.iter().map(|c| c.cut(&m)).collect();
            let cut: f64 = broken * scoops.iter().zip(&cuts).map(|(c, f)| c.depth * f).sum::<f64>();
            let crevasse = crevasses.iter()
                .map(|&(n, o, w, depth)| depth * (-((m.dot(&n) - o) / w).powi(2)).exp())
                .fold(0., f64::max);
            // Fine vertical streaks where meltwater runs down, and drifts on the snow.
            let streaks = 0.05 * noise.noise(Vector3::new(m.x / 0.7, m.y / 7., m.z / 0.7))
                + 0.22 * noise.noise(Vector3::new(m.x / 1.6, m.y / 14., m.z / 1.6) + Vector3::new(9., 0., 4.));
            // Crumpled: the cliff folds in upright creases and buttresses a few metres across.
            let fold = |w: f64, o: f64| noise.ridged(Vector3::new(m.x / w, m.y / (4. * w), m.z / w) + Vector3::repeat(o), 3) - 0.5;
            let folds = 0.06 * big * fold(0.4 * big, 11.) + 0.02 * big * fold(0.13 * big, 23.);
            let streaks = if rounded { streaks } else { streaks + folds };
            let drifts = 0.5 * noise.fbm(m / 14., 3) * top;
            // How freshly broken the ice here is, 0 to 1: the deeper it was cut back, the less
            // weathered (and the bluer: `IcebergMaterial`).
            // Not at the rim, where the cut is into the snow.
            let rim = 1. - smoothstep(0.15, 0.45, normal.y);
            let scars = if cuts.len() > 1 { 0.55 * cuts[0] + 0.55 * cuts[1] } else { 0. };
            let fresh = (seen * rim * (broken * scars + crevasse)).clamp(0., 1.);
            // The firn: a shelf's top few metres are last years' snow, white down the cliff.
            let firn = if kind == Kind::Tabular {
                let top_y = axes.y * crown(&q);
                smoothstep(top_y - 5. / size, top_y - 2. / size, q.y)
            } else { 0. };
            (m + normal * (drifts - seen * wall * (cut + crevasse + streaks)), fresh, firn)
        };
        (surface, directions, triangles)
    }

    /// The volume of a closed, outward-wound mesh above the plane y = `h`: by the divergence
    /// theorem, the sum over the parts of its triangles above the plane of the signed volumes
    /// of the tetrahedra they make with a point on the plane (the plane's own cross-section
    /// adds nothing, lying in it).
    pub fn volume_above(positions: &[Vector3<f64>], triangles: &[[usize; 3]], h: f64) -> f64 {
        let o = Vector3::new(0., h, 0.);
        let tet = |a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>| (a - o).dot(&(b - o).cross(&(c - o))) / 6.;
        triangles.iter().map(|t| {
            let v = t.map(|i| positions[i]);
            // The triangle clipped to y >= h (Sutherland-Hodgman against one plane).
            let mut poly: Vec<Vector3<f64>> = Vec::with_capacity(4);
            for k in 0..3 {
                let (a, b) = (v[k], v[(k + 1) % 3]);
                let (da, db) = (a.y - h, b.y - h);
                if da >= 0. { poly.push(a); }
                if (da >= 0.) != (db >= 0.) { poly.push(a + (b - a) * (da / (da - db))); }
            }
            (1..poly.len().saturating_sub(1)).map(|k| tet(&poly[0], &poly[k], &poly[k + 1])).sum::<f64>()
        }).sum()
    }

    /// The height of the waterline of a closed mesh: where the fraction of its volume above
    /// it is `fraction`.
    pub fn waterline(positions: &[Vector3<f64>], triangles: &[[usize; 3]], fraction: f64) -> f64 {
        let (lo, hi) = positions.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.y), hi.max(p.y)));
        let total = Iceberg::volume_above(positions, triangles, lo - 1.);
        let (mut lo, mut hi) = (lo, hi);
        for _ in 0..50 {
            let mid = 0.5 * (lo + hi);
            if Iceberg::volume_above(positions, triangles, mid) / total > fraction { lo = mid } else { hi = mid }
        }
        0.5 * (lo + hi)
    }

    /// The mesh floating with its waterline at `sea_level`, centred over `(x, z)`, turned by
    /// `yaw` (radians).
    pub fn mesh(&self, x: f64, z: f64, sea_level: f64, yaw: f64) -> IndexedMesh {
        let base = self.detail.saturating_sub(1);
        let (shape, directions, triangles) = self.shape(base);
        let mut rng = Pcg32::new(self.seed, 29);
        // Bergs other than tabular ones float tilted, having rolled as they melted.
        let tilt = if self.kind == Kind::Tabular { 0.01 } else { 0.12 };
        let rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), yaw)
            * Rotation3::from_euler_angles(tilt * (rng.next_f64() - 0.5), 0., tilt * (rng.next_f64() - 0.5));
        let turned = |d: &Vector3<f64>| rotation * shape(d).0;
        // The waterline, roughly, from the coarse mesh.
        let coarse: Vec<Vector3<f64>> = directions.iter().map(&turned).collect();
        let h = Iceberg::waterline(&coarse, &triangles, self.freeboard_fraction());
        // The wave-cut notch: the sea melts a groove a metre or two high at the waterline.
        let notch_depth = (0.015 * self.size).min(2.5);
        let notch_height = (0.012 * self.size).clamp(0.8, 2.);
        // The ram: under the notch the ice the waves haven't reached juts out as a submerged
        // shelf, broadest on one side.
        let width = (0.15 * self.size).min(20.) * smoothstep(10., 60., self.size).max(0.25);
        let reach = 8. + 0.12 * self.size;
        let side = rng.next_f64() * TAU;
        let surface = |d: &Vector3<f64>| -> Vector3<f64> {
            let mut p = turned(d);
            let across = (p.x * p.x + p.z * p.z).sqrt();
            let notch = notch_depth * (-((p.y - h - 0.3 * notch_height) / notch_height).powi(2)).exp();
            let below = h - p.y;
            let g = smoothstep(0.8, 4., below) * (1. - smoothstep(0.5 * reach, reach, below));
            let facing = 0.5 + 0.5 * (0.5 + 0.5 * (p.z.atan2(p.x) - side).cos()).powi(2);
            let out = width * g * facing - notch;
            if across > 1e-6 && across + out > 0.05 * across {
                let s = 1. + out / across;
                p.x *= s;
                p.z *= s;
            }
            p
        };
        // Refined where the surface bends more than a centimetre or so between vertices: along
        // the facets' edges and steps, the notch, and the scallops and flutes on the faces.
        let tolerance = (0.0001 * self.size).clamp(0.005, 0.03);
        // What shows above the water is meshed to 40 cm at least, to carry the scallops.
        let coarsest = |p: &Vector3<f64>| if p.y > h - 4. { 0.4 } else { f64::INFINITY };
        let (directions, positions, triangles) = refine(&directions, &triangles, &surface, 8, tolerance, &coarsest);
        println!("Iceberg: {} triangles", triangles.len());
        // How fresh the ice is at each vertex, and how much it is firn, as texture coordinates.
        let texcoords = {
            use rayon::prelude::*;
            directions.par_iter().map(|d| { let s = shape(d); crate::na::Vector2::new(s.1, s.2) }).collect()
        };
        let mut mesh = IndexedMesh { positions, normals: vec![], triangles, texcoords };
        // Floated at its waterline, ram and all.
        let h = Iceberg::waterline(&mesh.positions, &mesh.triangles, self.freeboard_fraction());
        let lift = Vector3::new(x, sea_level - h, z);
        for p in mesh.positions.iter_mut() { *p += lift; }
        if self.kind == Kind::Dome { mesh } else { creased(&mesh, 35f64.to_radians().cos()) }
    }
}

/// Mesh `surface` (a function of direction, from the unit `directions`) adaptively: each pass
/// splits every edge whose midpoint on the surface lies more than `tolerance` from the
/// straight edge, and splits the triangles accordingly (into two, three or four), up to
/// `levels` times; and every edge longer than `coarsest` allows at its midpoint. An edge's
/// split is decided once for both its triangles, so the mesh stays closed. Returns the
/// positions and triangles.
pub fn refine(directions: &[Vector3<f64>], triangles: &[[usize; 3]], surface: &(dyn Fn(&Vector3<f64>) -> Vector3<f64> + Sync), levels: u32, tolerance: f64, coarsest: &(dyn Fn(&Vector3<f64>) -> f64 + Sync)) -> (Vec<Vector3<f64>>, Vec<Vector3<f64>>, Vec<[usize; 3]>) {
    use rayon::prelude::*;
    use std::collections::HashMap;
    let mut dirs: Vec<Vector3<f64>> = directions.to_vec();
    let mut points: Vec<Vector3<f64>> = dirs.par_iter().map(surface).collect();
    let mut tris: Vec<[usize; 3]> = triangles.to_vec();
    for _ in 0..levels {
        // Each edge once, with its midpoint's direction.
        let mut edges: Vec<(usize, usize)> = tris.iter().flat_map(|&[a, b, c]| [(a, b), (b, c), (c, a)])
            .map(|(a, b)| (a.min(b), a.max(b))).collect();
        edges.sort_unstable();
        edges.dedup();
        let mids: Vec<Option<(Vector3<f64>, Vector3<f64>)>> = edges.par_iter().map(|&(a, b)| {
            let d = (dirs[a] + dirs[b]).normalize();
            let p = surface(&d);
            // (A midpoint can sit on the straight edge by chance while the surface between is
            // rough, so edges are also split down to the length `coarsest` allows there.)
            let long = (points[a] - points[b]).norm() > coarsest(&p);
            (long || (p - (points[a] + points[b]) / 2.).norm() > tolerance).then_some((d, p))
        }).collect();
        let mut split: HashMap<(usize, usize), usize> = HashMap::new();
        for (e, m) in edges.iter().zip(mids) {
            if let Some((d, p)) = m {
                split.insert(*e, dirs.len());
                dirs.push(d);
                points.push(p);
            }
        }
        if split.is_empty() { break; }
        let mid = |a: usize, b: usize| split.get(&(a.min(b), a.max(b))).copied();
        let mut next = Vec::with_capacity(tris.len() * 2);
        for &t in &tris {
            // Turned so that the split edges come first: ab, then bc, then ca.
            let ms = [mid(t[0], t[1]), mid(t[1], t[2]), mid(t[2], t[0])];
            match ms.iter().filter(|m| m.is_some()).count() {
                0 => next.push(t),
                3 => {
                    let (ab, bc, ca) = (ms[0].unwrap(), ms[1].unwrap(), ms[2].unwrap());
                    next.extend([[t[0], ab, ca], [ab, t[1], bc], [ca, bc, t[2]], [ab, bc, ca]]);
                }
                1 => {
                    let k = ms.iter().position(|m| m.is_some()).unwrap();
                    let (a, b, c) = (t[k], t[(k + 1) % 3], t[(k + 2) % 3]);
                    let m = ms[k].unwrap();
                    next.extend([[a, m, c], [m, b, c]]);
                }
                _ => {
                    // The unsplit edge is k: c to a, after turning.
                    let k = ms.iter().position(|m| m.is_none()).unwrap();
                    let (a, b, c) = (t[(k + 1) % 3], t[(k + 2) % 3], t[k]);
                    let (ab, bc) = (ms[(k + 1) % 3].unwrap(), ms[(k + 2) % 3].unwrap());
                    next.extend([[a, ab, c], [ab, bc, c], [ab, b, bc]]);
                }
            }
        }
        tris = next;
    }
    (dirs, points, tris)
}

/// Conchoidal fracture: ice breaks like glass, leaving concave scoops that meet at sharp
/// ridges (calving scars; at a small scale, the scallops melt leaves are the same shape).
/// Each jittered site of a grid of `cell` carves a paraboloid bowl, and the cut at a point is
/// the deepest of them there: the upper envelope of a_i - |x - s_i|² (in cells, a_i random),
/// which is continuous, concave within each bowl and creased where two meet, and zero where
/// no bowl reaches, leaving the face as it was.
struct Scoops {
    /// The grid's cell (taller than wide, so the scoops stand upright) and the deepest cut.
    cell: Vector3<f64>,
    depth: f64,
    seed: u64,
}

impl Scoops {
    fn unit(h: u64, k: u64) -> f64 {
        (crate::sobol::hash(h, k) >> 11) as f64 / (1u64 << 53) as f64
    }

    /// How far to cut back at `m`, as a fraction of `depth`.
    fn cut(&self, m: &Vector3<f64>) -> f64 {
        let g = m.component_div(&self.cell);
        let base = g.map(f64::floor);
        let mut best: f64 = 0.;
        // A site farther than one cell away can't reach (a_i is at most 1).
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let c = base + Vector3::new(dx as f64, dy as f64, dz as f64);
                    let h = [c.x, c.y, c.z].iter().fold(self.seed, |h, v| crate::sobol::hash(h, *v as i64 as u64));
                    let site = c + Vector3::new(Scoops::unit(h, 1), Scoops::unit(h, 2), Scoops::unit(h, 3));
                    best = best.max(0.45 + 0.55 * Scoops::unit(h, 4) - (g - site).norm_squared());
                }
            }
        }
        best
    }
}

/// The mesh with its normals smoothed only across edges where the faces meet at less than the
/// angle whose cosine is `crease`: sharper edges stay sharp. A vertex is split only where its
/// corners' normals differ (along a crease), so a smooth mesh keeps its vertices.
pub fn creased(mesh: &IndexedMesh, crease: f64) -> IndexedMesh {
    use rayon::prelude::*;
    let p = &mesh.positions;
    let face: Vec<Vector3<f64>> = mesh.triangles.par_iter().map(|&[a, b, c]| (p[b] - p[a]).cross(&(p[c] - p[a]))).collect();
    let unit: Vec<Vector3<f64>> = face.par_iter().map(|f| f.try_normalize(1e-300).unwrap_or_else(Vector3::zeros)).collect();
    let mut around: Vec<Vec<u32>> = vec![vec![]; p.len()];
    for (t, tri) in mesh.triangles.iter().enumerate() {
        for &v in tri { around[v].push(t as u32); }
    }
    // Each corner's normal: the faces round its vertex within the crease angle of its own.
    let corners: Vec<[Vector3<f64>; 3]> = mesh.triangles.par_iter().enumerate().map(|(t, tri)| {
        tri.map(|v| {
            around[v].iter()
                .filter(|&&u| unit[u as usize].dot(&unit[t]) >= crease)
                .fold(Vector3::zeros(), |a, &u| a + face[u as usize])
                .try_normalize(1e-300).unwrap_or(unit[t])
        })
    }).collect();
    // A vertex per distinct normal at each original vertex.
    let mut positions: Vec<Vector3<f64>> = Vec::with_capacity(p.len());
    let mut normals: Vec<Vector3<f64>> = Vec::with_capacity(p.len());
    let mut texcoords = Vec::with_capacity(mesh.texcoords.len());
    let mut made: Vec<Vec<u32>> = vec![vec![]; p.len()];
    let mut triangles = Vec::with_capacity(mesh.triangles.len());
    for (t, tri) in mesh.triangles.iter().enumerate() {
        let mut out = [0usize; 3];
        for k in 0..3 {
            let (v, n) = (tri[k], corners[t][k]);
            let found = made[v].iter().copied().find(|&i| normals[i as usize] == n || (normals[i as usize] - n).norm_squared() < 1e-10);
            out[k] = match found {
                Some(i) => i as usize,
                None => {
                    positions.push(p[v]);
                    normals.push(n);
                    if !mesh.texcoords.is_empty() { texcoords.push(mesh.texcoords[v]); }
                    made[v].push((positions.len() - 1) as u32);
                    positions.len() - 1
                }
            };
        }
        triangles.push(out);
    }
    IndexedMesh { positions, normals, triangles, texcoords }
}

/// An iceberg's ice: snow on the top and on ledges, and beneath it ice that is whiter the
/// longer it has weathered. A glacier's ice is blue within: dense, its bubbles few. Where a
/// face has stood in the sun and air for a season, melt opens the crystal boundaries and it
/// turns to a white, bubbly, cracked crust; where a piece has just broken away, in the
/// scoops of calving scars and down crevasses, the blue ice shows. The generator records how
/// far each point was cut back (`fresh`, a texture coordinate), and that picks between
/// `snow::Ice`'s white, bubbly and blue ice, all the same physics with different bubbles.
///
/// Small pieces (`translucent`) are instead lit through: light wanders inside them
/// (`material::subsurface`, with ice's own absorption and its bubbles' scattering), so thin
/// edges glow and thick parts are bluer.
pub struct IcebergMaterial {
    pub sea_level: f64,
    body: Option<Subsurface>,
    white: Ice,
    glacier: Ice,
    blue: Ice,
    /// Wet ice melted smooth: glossy, and greyer than the white pieces.
    clear: Ice,
    snow: Snow,
    /// The berg's length, m.
    size: f64,
    noise: SeededNoise,
}

/// The reduced scattering coefficient of a small piece's ice, per metre: light travels a
/// quarter of a metre between changes of direction.
pub const ICE_SCATTERING: f64 = 4.;

impl IcebergMaterial {
    pub fn new(sea_level: f64, size: f64, seed: u64) -> IcebergMaterial {
        let clean = |preset: &str, roughness: f64| Ice::new(IceStyle { impurities: Default::default(), roughness, ..IceStyle::preset(preset) });
        IcebergMaterial {
            sea_level,
            body: None,
            white: clean("white", 0.3),
            glacier: clean("glacier", 0.15),
            // Dense ice with few bubbles, as deep in a glacier: a saturated blue.
            blue: Ice::new(IceStyle { bubble_radius: 0.7, bubbles: 1.2, impurities: Default::default(), roughness: 0.08 }),
            clear: Ice::new(IceStyle { bubble_radius: 0.6, bubbles: 9., impurities: Default::default(), roughness: 0.04 }),
            snow: Snow::new(SnowStyle { sparkle: 0., ..SnowStyle::preset("settled") }, 1., seed),
            size,
            noise: SeededNoise::new(seed.wrapping_add(17)),
        }
    }

    /// Lit through, for small pieces.
    pub fn translucent(self) -> IcebergMaterial {
        IcebergMaterial {
            body: Some(Subsurface {
                coefficients: Coefficients::Ice { scattering: ICE_SCATTERING, impurities: Default::default() },
                g: 0.,
                refractive_index: crate::material::snow::ICE_REFRACTIVE_INDEX,
            }),
            ..self
        }
    }

    /// The shares at `p`, with outward normal `n` and freshness `fresh`, of snow, white crust
    /// and blue ice (the rest is bubbly glacier ice).
    pub fn shares(&self, p: &Vector3<f64>, n: &Vector3<f64>, fresh: f64, firn: f64) -> (f64, f64, f64) {
        let height = p.y - self.sea_level;
        // Above the reach of the waves (a band that grows with the berg).
        let dry = smoothstep(0.3, 1., height / (0.03 * self.size).clamp(0.05, 2.5));
        // Snow lies on the top and on ledges.
        let snow = (dry * smoothstep(0.3, 0.6, n.y + 0.15 * self.noise.fbm(p / 6., 3))).max(firn);
        // Weathering varies in patches, and faces that lean back catch more of it.
        let patches = self.noise.fbm(Vector3::new(p.x / 9., p.y / 5., p.z / 9.), 4);
        let fresh = (fresh - 0.15 * patches - 0.2 * n.y).clamp(0., 1.);
        let white = dry * (1. - smoothstep(0.1, 0.4, fresh));
        let blue = smoothstep(0.45, 0.8, fresh);
        (snow, white, blue)
    }
}

impl MaterialModel for IcebergMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let (fresh, firn) = intersection.uv().map_or((0., 0.), |uv| (uv.x, uv.y));
        let (snow, white, blue) = self.shares(&intersection.point, &intersection.normal, fresh, firn);
        let outside = r.rd.dot(&intersection.normal) < 0.;
        let i = &intersection.facing(r);
        let f = crate::material::functions::scatter_bsdf;
        let (snow, fresh) = if self.body.is_some() { (if fresh > 0.5 { 0. } else { snow }, fresh) } else { (snow, fresh) };
        if outside && rand() < snow {
            return f(&self.snow.bsdf(&i.point, &i.normal, Some(s)), r, &i.point, s);
        }
        if let Some(body) = &self.body {
            // Brash: for it the first texture coordinate says a piece is clear ice.
            if fresh > 0.5 {
                return f(&self.clear.bsdf(&i.normal, Some(s)), r, &i.point, s);
            }
            return if outside && rand() < 0.6 * white {
                f(&self.white.bsdf(&i.normal, Some(s)), r, &i.point, s)
            } else {
                body.scatter(r, intersection, s)
            };
        }
        let u = rand();
        let ice = if u < white { &self.white } else if u < white + blue { &self.blue } else { &self.glacier };
        f(&ice.bsdf(&i.normal, Some(s)), r, &i.point, s)
    }
}

/// A field of ice floes on the sea.
#[derive(Clone)]
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
    pub style: FloeStyle,
    /// How much the floes gather in clumps, 0 (evenly) to 1.
    pub clumping: f64,
    /// The ocean whose waves they ride, if any: each floats at the surface's height where it
    /// is, tipped with its slope (fully for small pieces, less for those as large as the
    /// waves).
    pub waves: Option<std::sync::Arc<crate::ocean::Ocean>>,
    pub seed: u64,
}

/// What the pieces are.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FloeStyle {
    /// Slabs of sea ice, snow on top.
    Slab,
    /// Brash ice and bergy bits: lumps of glacier ice broken from bergs, rounded by melting,
    /// floating by buoyancy as a berg does (a tenth above water), about half as high as wide.
    Brash,
}

/// One floe: its centre (x, z) and radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Floe { pub x: f64, pub z: f64, pub radius: f64 }

impl FloeField {
    /// Floes placed largest first, none touching, until the concentration is reached: sizes
    /// from a power law (many more small than large), and the smaller ones, half the time,
    /// placed beside a larger one, as fragments gather round the floes and bergy bits they
    /// broke from.
    pub fn floes(&self) -> Vec<Floe> {
        let mut rng = Pcg32::new(self.seed, 37);
        let area = self.extent.0 * self.extent.1;
        let target = self.concentration.clamp(0., 0.95) * area;
        // Diameters with number density ∝ d^-α between the smallest and largest: by inverting
        // the cumulative distribution, d = d_min (1 - u (1 - (d_min/d_max)^(α-1)))^(-1/(α-1)).
        // Brash ice is steeper than the floes of pack ice: mostly small pieces.
        let alpha = match self.style { FloeStyle::Brash => 2.3, FloeStyle::Slab => 2.2 };
        let k = 1. - (self.min_size / self.max_size).powf(alpha - 1.);
        let mut radii = vec![];
        let mut sum = 0.;
        while sum < target && radii.len() < 40_000 {
            let d = self.min_size * (1. - rng.next_f64() * k).powf(-1. / (alpha - 1.));
            let r = d.min(self.max_size) / 2.;
            // A floe's outline covers about 0.7 of its circle.
            sum += 0.7 * PI * r * r;
            radii.push(r);
        }
        radii.sort_by(|a, b| b.total_cmp(a));
        // A grid of cells for the overlap test; a floe is listed in every cell its circle
        // touches.
        let cell = (4. * self.min_size).max(0.5 * self.max_size);
        let nx = (self.extent.0 / cell).ceil().max(1.) as usize;
        let nz = (self.extent.1 / cell).ceil().max(1.) as usize;
        let mut grid: Vec<Vec<usize>> = vec![vec![]; nx * nz];
        let mut placed: Vec<Floe> = vec![];
        let clumps = SeededNoise::new(self.seed.wrapping_add(53));
        let clump_size = 0.3 * self.extent.0.min(self.extent.1).max(4. * self.max_size);
        let (x0, z0) = (self.center.0 - self.extent.0 / 2., self.center.1 - self.extent.1 / 2.);
        let cells = |x: f64, z: f64, r: f64| {
            let i0 = ((x - r - x0) / cell).floor().max(0.) as usize;
            let i1 = (((x + r - x0) / cell).floor().max(0.) as usize).min(nx - 1);
            let j0 = ((z - r - z0) / cell).floor().max(0.) as usize;
            let j1 = (((z + r - z0) / cell).floor().max(0.) as usize).min(nz - 1);
            (i0.min(nx - 1), i1, j0.min(nz - 1), j1)
        };
        for r in radii {
            // Packed close: brash jostles edge to edge.
            let gap = match self.style { FloeStyle::Brash => 0.02 * r + 0.02, FloeStyle::Slab => 0.04 * r + 0.05 };
            for _ in 0..40 {
                // Beside a larger floe, or anywhere (where the clumps are).
                let near = if !placed.is_empty() && rng.next_f64() < 0.5 {
                    let f = placed[(rng.next_f64() * placed.len() as f64) as usize];
                    (f.radius > 2. * r).then(|| {
                        let a = rng.next_f64() * TAU;
                        let dist = f.radius + r + gap * (1. + 3. * rng.next_f64());
                        (f.x + a.cos() * dist, f.z + a.sin() * dist)
                    })
                } else { None };
                let (x, z) = match near {
                    Some(p) => p,
                    None => {
                        let (x, z) = (x0 + rng.next_f64() * self.extent.0, z0 + rng.next_f64() * self.extent.1);
                        // Pack ice gathers in clumps and streaks, with open water between.
                        let clump = smoothstep(-0.35, 0.35, clumps.fbm(Vector3::new(x, 0., z) / clump_size, 3));
                        if rng.next_f64() > 1. - self.clumping * (1. - clump) { continue; }
                        (x, z)
                    }
                };
                if x < x0 || x > x0 + self.extent.0 || z < z0 || z > z0 + self.extent.1 { continue; }
                // Thinning out towards the field's edges, along a ragged line.
                let edge = (x - x0).min(x0 + self.extent.0 - x).min(z - z0).min(z0 + self.extent.1 - z);
                let margin = 0.25 * self.extent.0.min(self.extent.1);
                let thin = smoothstep(0., 1., edge / margin + 0.4 * clumps.fbm(Vector3::new(x, 7., z) / (0.5 * margin), 3));
                if rng.next_f64() > thin { continue; }
                let (i0, i1, j0, j1) = cells(x, z, r + gap);
                let clear = (i0..=i1).all(|i| (j0..=j1).all(|j| grid[j * nx + i].iter().all(|&k| {
                    let f = &placed[k];
                    (f.x - x).hypot(f.z - z) > f.radius + r + gap
                })));
                if clear {
                    let (i0, i1, j0, j1) = cells(x, z, r);
                    for i in i0..=i1 { for j in j0..=j1 { grid[j * nx + i].push(placed.len()); } }
                    placed.push(Floe { x, z, radius: r });
                    break;
                }
            }
        }
        placed
    }

    /// The floes as one mesh.
    pub fn mesh(&self) -> IndexedMesh {
        match self.style {
            FloeStyle::Slab => self.slabs(),
            FloeStyle::Brash => self.brash(),
        }
    }

    /// Brash ice: mostly broken plates (flat, angular, with straight fractured edges, some
    /// pushed up and tilted on their neighbours), and among the larger pieces rounded lumps of
    /// glacier ice; each floating at its own waterline.
    fn brash(&self) -> IndexedMesh {
        let floes = self.floes();
        let noise = SeededNoise::new(self.seed.wrapping_add(7));
        let mut rng = Pcg32::new(self.seed, 47);
        // Geodesic spheres of each detail, so that whatever a piece's size its vertices are
        // about fifteen centimetres apart.
        let spheres: Vec<(Vec<Vector3<f64>>, Vec<[usize; 3]>)> = (0..=6).map(icosphere).collect();
        let mut positions = vec![];
        let mut triangles = vec![];
        let mut texcoords = vec![];
        let fraction = 1. - 917. / SEA_WATER_DENSITY;
        for f in &floes {
            let r = f.radius * 0.95;
            let lump = rng.next_f64() < 0.1 + 0.5 * smoothstep(1.5, 6., r);
            let offset = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 100.;
            // A plate's thickness, a fifth to nearly half its width (but at least a few
            // centimetres); a lump about half as high as wide.
            let axes = if lump {
                Vector3::new(r, r * (0.3 + 0.3 * rng.next_f64()), r * (0.5 + 0.5 * rng.next_f64()))
            } else {
                let t = (r * (0.2 + 0.25 * rng.next_f64())).max(0.04);
                Vector3::new(r, t, r * (0.55 + 0.45 * rng.next_f64()))
            };
            let detail = ((1.1 * r / 0.15).log2().ceil().max(2.) as usize).min(5);
            let (directions, sphere) = &spheres[detail];
            // Spread over the surface rather than by angle, as for the bergs.
            let directions: Vec<Vector3<f64>> = directions.iter().map(|d| d.component_mul(&axes).normalize()).collect();
            // Tilt: most lie nearly flat; some plates are rafted, pushed up on their
            // neighbours.
            // Small pieces are jostled about more: half of them tipped, some steeply.
            let jumbled = 0.3 + 0.3 / (1. + 2. * r);
            let tilt = if !lump && rng.next_f64() < jumbled { 0.6 * rng.next_f64() } else { 0.1 * rng.next_f64() };
            let lean = rng.next_f64() * TAU;
            let rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), rng.next_f64() * TAU)
                * Rotation3::from_axis_angle(&crate::na::Unit::new_normalize(Vector3::new(lean.cos(), 0., lean.sin())), tilt);
            // Fracture planes: the edges it broke along, near-vertical for a plate (straight
            // edges round a polygon), any way for a lump.
            let count = if lump { 3 + (rng.next_f64() * 3.) as usize } else { 4 + (rng.next_f64() * 4.) as usize };
            let planes: Vec<(Vector3<f64>, f64)> = (0..count).map(|_| {
                let a = rng.next_f64() * TAU;
                let n = if lump {
                    Vector3::new(a.cos(), 1.2 * (rng.next_f64() - 0.4), a.sin()).normalize()
                } else {
                    Vector3::new(a.cos(), 0.15 * (rng.next_f64() - 0.5), a.sin()).normalize()
                };
                (n, axes.component_mul(&n).norm() * (0.55 + 0.4 * rng.next_f64()))
            }).collect();
            let (p, rounding, lumpiness) = if lump { (2.6, 0.08 + 0.1 * rng.next_f64(), 0.12) } else { (6., 0.015, 0.0) };
            let mut piece: Vec<Vector3<f64>> = directions.iter().map(|d| {
                let r0 = (d.x.abs() / axes.x).powf(p) + (d.y.abs() / axes.y).powf(p) + (d.z.abs() / axes.z).powf(p);
                let q = d * r0.powf(-1. / p);
                let mut s: f64 = 1.;
                for (n, o) in &planes {
                    let along = q.dot(n);
                    if along > 0. { s = smin(s, o / along, rounding * r); }
                }
                let q = q * s.max(0.05);
                // Lumps are knobbly; a plate is lumpy too, at its own size and finer.
                let bumps = if lump {
                    lumpiness * r * noise.fbm(d * 1.6 + offset, 4)
                } else {
                    0.07 * r * noise.fbm(q / (0.5 * r) + offset, 3) + 0.01 * noise.fbm(q / 0.12 + offset, 2)
                };
                rotation * (q + d * bumps)
            }).collect();
            if lump {
                // Worn at the waterline, as the bergs are: a notch, and a small skirt below.
                let h0 = Iceberg::waterline(&piece, sphere, fraction);
                let height = 2. * axes.y;
                let skirt = r * (0.04 + 0.06 * rng.next_f64());
                for p in piece.iter_mut() {
                    let below = (h0 - p.y) / height;
                    let g = smoothstep(0.02, 0.15, below) * (1. - smoothstep(0.3, 0.75, below));
                    let notch = 0.02 * r * (-((p.y - h0) / (0.06 * height)).powi(2)).exp();
                    let across = (p.x * p.x + p.z * p.z).sqrt();
                    let out = skirt * g - notch;
                    if across > 1e-9 && across + out > 0.1 * across {
                        let k = 1. + out / across;
                        p.x *= k;
                        p.z *= k;
                    }
                }
            }
            let h = Iceberg::waterline(&piece, sphere, fraction);
            // Riding the waves: raised or lowered with the surface, and tipped with it.
            let (rise, ride) = self.ride(f);
            let pivot = Vector3::new(0., h, 0.);
            let start = positions.len();
            for p in piece.iter_mut() {
                *p = ride * (*p - pivot) + Vector3::new(f.x, self.sea_level + rise, f.z);
            }
            // Small pieces are often clear ice: melted smooth, washed by the waves, glassy and
            // dark against the white ones. (Carried as a texture coordinate.)
            let clear = !lump && rng.next_f64() < 0.15 + 0.5 * (1. - smoothstep(0.3, 1.2, r));
            texcoords.extend(std::iter::repeat_n(crate::na::Vector2::new(clear as u8 as f64, 0.), piece.len()));
            positions.extend(piece);
            triangles.extend(sphere.iter().map(|t| t.map(|i| i + start)));
        }
        println!("Brash: {} pieces, {} triangles", floes.len(), triangles.len());
        creased(&IndexedMesh { positions, normals: vec![], triangles, texcoords }, 50f64.to_radians().cos())
    }

    /// How the waves carry floe `f`: the surface's height at its centre, and the turn that
    /// tips it with the surface's slope there (less, the larger it is beside the waves).
    fn ride(&self, f: &Floe) -> (f64, Rotation3<f64>) {
        match &self.waves {
            None => (0., Rotation3::identity()),
            Some(w) => {
                let (h, n) = w.surface_at(f.x, f.z);
                let follow = 1. / (1. + (f.radius / 3.).powi(2));
                let up = (Vector3::y() * (1. - follow) + n * follow).normalize();
                (h, Rotation3::rotation_between(&Vector3::y(), &up).unwrap_or_else(Rotation3::identity))
            }
        }
    }

    /// Slabs of sea ice.
    fn slabs(&self) -> IndexedMesh {
        let floes = self.floes();
        let mut positions = vec![];
        let mut triangles = vec![];
        let noise = SeededNoise::new(self.seed.wrapping_add(3));
        let mut rng = Pcg32::new(self.seed, 41);
        let fraction = 0.1;
        for f in &floes {
            let first = positions.len();
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
            // Riding the waves.
            let (rise, ride) = self.ride(f);
            let pivot = Vector3::new(f.x, self.sea_level, f.z);
            for p in positions[first..].iter_mut() {
                *p = ride * (*p - pivot) + pivot + Vector3::new(0., rise, 0.);
            }
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
        let (dirs, tris) = icosphere(5);
        let h = Iceberg::waterline(&dirs, &tris, 0.5);
        assert!(h.abs() < 0.01, "{}", h);
        // With less above, the waterline rises through it.
        assert!(Iceberg::waterline(&dirs, &tris, 0.1) > 0.4);
    }

    #[test]
    fn test_floes_cover_the_concentration_without_overlapping() {
        let field = FloeField { center: (0., 0.), extent: (400., 300.), concentration: 0.6, min_size: 2., max_size: 60., thickness: 1., sea_level: 0., style: FloeStyle::Slab, clumping: 0., waves: None, seed: 2 };
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
