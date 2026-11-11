//! Procedural rocks: bedrock, boulders and pebbles.
//!
//! Boulders and pebbles are star-shaped meshes: a subdivided icosahedron (a geodesic sphere,
//! whose vertices are spread evenly, unlike a latitude-longitude sphere's) with each vertex moved
//! along its direction from the centre. Moving vertices only along their own directions keeps
//! the surface from folding through itself, whatever the displacement. The shape is built up as
//! stone is shaped:
//!
//! 1. An ellipsoid with random axes: few rocks are round, and most rest on their flattest side.
//! 2. Fracture planes: a block breaks along joints, leaving flat faces meeting at edges. Each
//!    plane clips the shape (the radius is cut to where the direction meets the plane), and the
//!    clips are combined with a smooth minimum (Inigo Quilez, "Smooth minimum",
//!    <https://iquilezles.org/articles/smin/>), which rounds the edges as weathering does. Worn
//!    pebbles have few planes and very round edges.
//! 3. Noise displacement: broad lumps from fractal noise, and sharper ridges and pits from ridged
//!    noise. Finer relief is left to the material's bump mapping (see `material::rock`).
//!
//! Bedrock is a height field: fractal noise for the mass, ridged noise for crests, and terraces,
//! which give the stepped ledges of layered rock eroding bed by bed. It falls away at its edges,
//! so it can rise out of, or sink into, a ground plane.
//!
//! Every shape depends only on its seed, so a scene renders the same rocks each time.

use crate::na::{Vector3, Rotation3};
use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
use crate::shapes::geometry::Geometry;
use crate::sceneobject::SceneObject;
use crate::material::texture::Medium;
use crate::material::model::MaterialModel;
use crate::noise::SeededNoise;
use crate::geometry::Pcg32;
use std::collections::HashMap;

/// A geodesic sphere: an icosahedron with each triangle split into four `subdivisions` times,
/// and the new vertices pushed out onto the unit sphere. Triangles wind anticlockwise seen from
/// outside, so `vertex_normals` points outward.
pub fn icosphere(subdivisions: u32) -> (Vec<Vector3<f64>>, Vec<[usize; 3]>) {
    let t = (1. + 5f64.sqrt()) / 2.;
    let mut positions: Vec<Vector3<f64>> = [
        (-1., t, 0.), (1., t, 0.), (-1., -t, 0.), (1., -t, 0.),
        (0., -1., t), (0., 1., t), (0., -1., -t), (0., 1., -t),
        (t, 0., -1.), (t, 0., 1.), (-t, 0., -1.), (-t, 0., 1.),
    ].iter().map(|&(x, y, z)| Vector3::new(x, y, z).normalize()).collect();
    let mut triangles: Vec<[usize; 3]> = vec![
        [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
        [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
        [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
        [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
    ];
    for _ in 0 .. subdivisions {
        let mut midpoints: HashMap<(usize, usize), usize> = HashMap::new();
        let mut midpoint = |a: usize, b: usize, positions: &mut Vec<Vector3<f64>>| {
            *midpoints.entry((a.min(b), a.max(b))).or_insert_with(|| {
                positions.push(((positions[a] + positions[b]) / 2.).normalize());
                positions.len() - 1
            })
        };
        triangles = triangles.iter().flat_map(|&[a, b, c]| {
            let ab = midpoint(a, b, &mut positions);
            let bc = midpoint(b, c, &mut positions);
            let ca = midpoint(c, a, &mut positions);
            [[a, ab, ca], [b, bc, ab], [c, ca, bc], [ab, bc, ca]]
        }).collect();
    }
    (positions, triangles)
}

/// Polynomial smooth minimum: min(a, b), with the corner rounded over a width `k`.
fn smin(a: f64, b: f64, k: f64) -> f64 {
    if k <= 0. { return a.min(b) }
    let h = (k - (a - b).abs()).max(0.) / k;
    a.min(b) - h * h * k / 4.
}

fn random_direction(rng: &mut Pcg32) -> Vector3<f64> {
    crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64())
}

/// The shape of a single rock, before it is placed in the scene.
#[derive(Clone, Debug)]
pub struct RockShape {
    /// The rock's semi-axes before fracturing.
    pub axes: Vector3<f64>,
    /// How many fracture planes cut the rock, and how deep: the fraction of the radius they can
    /// remove.
    pub fractures: usize,
    pub fracture_depth: f64,
    /// Rounding of the fractured edges, as a fraction of the radius.
    pub rounding: f64,
    /// Height of the noise displacement, as a fraction of the radius.
    pub roughness: f64,
    /// Geodesic subdivisions: each adds four times the triangles.
    pub detail: u32,
    pub seed: u64,
}

impl RockShape {
    /// A boulder of about `radius`: blocky, broken along joints and weathered.
    pub fn boulder(radius: f64, angularity: f64, seed: u64) -> RockShape {
        let mut rng = Pcg32::new(seed, 11);
        let angularity = angularity.clamp(0., 1.);
        let mut axis = || radius * (0.8 + 0.4 * rng.next_f64());
        let (x, y, z) = (axis(), axis() * 0.8, axis());
        RockShape {
            axes: Vector3::new(x, y, z),
            fractures: 3 + (angularity * 10.).round() as usize,
            fracture_depth: 0.12 + 0.3 * angularity,
            rounding: 0.35 - 0.3 * angularity,
            roughness: 0.1,
            detail: 5,
            seed,
        }
    }

    /// A pebble of about `size` across: flattened and worn smooth by water.
    pub fn pebble(size: f64, seed: u64) -> RockShape {
        let mut rng = Pcg32::new(seed, 13);
        let r = size / 2.;
        RockShape {
            axes: Vector3::new(r, r * (0.3 + 0.3 * rng.next_f64()), r * (0.55 + 0.35 * rng.next_f64())),
            fractures: (rng.next_f64() * 3.) as usize,
            fracture_depth: 0.25,
            rounding: 0.6,
            roughness: 0.025,
            detail: 3,
            seed,
        }
    }

    /// Vertex positions and triangles about the origin.
    pub fn mesh(&self) -> IndexedMesh {
        let mut rng = Pcg32::new(self.seed, 17);
        let noise = SeededNoise::new(self.seed);
        let radius = self.axes.max();
        let planes: Vec<(Vector3<f64>, f64)> = (0 .. self.fractures).map(|_| {
            let n = random_direction(&mut rng);
            // Distance from the centre to the ellipsoid's tangent plane with this normal.
            let support = self.axes.component_mul(&n).norm();
            (n, support * (1. - self.fracture_depth * (0.3 + 0.7 * rng.next_f64())))
        }).collect();

        let (directions, triangles) = icosphere(self.detail);
        let positions = directions.iter().map(|d| {
            let q = d.component_mul(&self.axes);
            // Cut back to the nearest plane, with the cuts' edges rounded.
            let mut s = 1.;
            for (n, offset) in &planes {
                let along = q.dot(n);
                if along > 0. {
                    s = smin(s, offset / along, self.rounding);
                }
            }
            let q = q * s.max(0.05);
            let u = q / radius;
            let lumps = noise.fbm(u * 1.4, 5);
            let ridges = noise.ridged(u * 3.5 + Vector3::new(0., 0., 11.), 4) - 0.35;
            let h = self.roughness * radius * (lumps + 0.5 * ridges);
            q + q.normalize() * h
        }).collect();
        IndexedMesh { positions, normals: vec![], triangles }
    }
}

/// Place a rock on the ground: turned by `rotation`, then moved so that it rests at `at` on
/// ground whose normal is `up`, with `embed` of its height below the ground.
pub fn place(mesh: &mut IndexedMesh, at: Vector3<f64>, rotation: Rotation3<f64>, up: Vector3<f64>, embed: f64) {
    for p in mesh.positions.iter_mut() {
        *p = rotation * *p;
    }
    let (lo, hi) = mesh.positions.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.dot(&up)), hi.max(p.dot(&up))));
    let lift = at - up * (lo + embed * (hi - lo));
    for p in mesh.positions.iter_mut() {
        *p += lift;
    }
}

/// A boulder resting on the ground at `location`.
pub fn boulder_mesh(location: Vector3<f64>, radius: f64, angularity: f64, detail: Option<u32>, seed: u64) -> IndexedMesh {
    let mut shape = RockShape::boulder(radius, angularity, seed);
    if let Some(d) = detail { shape.detail = d }
    let mut mesh = shape.mesh();
    let mut rng = Pcg32::new(seed, 19);
    let yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), rng.next_f64() * std::f64::consts::TAU);
    place(&mut mesh, location, yaw, Vector3::y(), 0.12);
    mesh
}

/// A pebble's centre on the ground, its size, and which of the given materials it is made of.
#[derive(Clone, Debug, PartialEq)]
pub struct Pebble {
    pub center: Vector3<f64>,
    pub size: f64,
    pub material: usize,
}

/// The area pebbles are scattered over, about the field's centre.
#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    Disc(f64),
    /// Width across x, and depth across z.
    Rect(f64, f64),
}

/// A bed of pebbles: sizes from `min_size` to `max_size`, small ones commoner, as in sorted
/// gravel. The ground may slope: `slope` is its rise per unit of x and of z.
#[derive(Clone, Debug)]
pub struct PebbleField {
    pub center: Vector3<f64>,
    pub region: Region,
    pub count: usize,
    pub min_size: f64,
    pub max_size: f64,
    /// How closely pebbles pack: at 1, the circles around their long axes don't overlap; below
    /// 1 they may, and the flatter pebbles overlap and lean on each other, as in shingle.
    pub packing: f64,
    pub slope: (f64, f64),
    pub materials: usize,
    pub seed: u64,
}

impl PebbleField {
    /// The ground's normal.
    pub fn up(&self) -> Vector3<f64> {
        Vector3::new(-self.slope.0, 1., -self.slope.1).normalize()
    }

    /// The ground's height at (x, z).
    fn ground(&self, x: f64, z: f64) -> f64 {
        self.center.y + self.slope.0 * (x - self.center.x) + self.slope.1 * (z - self.center.z)
    }

    /// Lay out the pebbles. Stops at `count`, or when no more fit.
    pub fn scatter(&self) -> Vec<Pebble> {
        let mut rng = Pcg32::new(self.seed, 23);
        let mut pebbles: Vec<Pebble> = Vec::new();
        // Pebbles placed so far, by cell of a grid no finer than the largest pebble, so a new
        // one need only be checked against its neighbours'.
        let cell = self.max_size.max(1e-6);
        let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        let key = |x: f64, z: f64| ((x / cell).floor() as i64, (z / cell).floor() as i64);
        let mut attempts = 0;
        while pebbles.len() < self.count && attempts < self.count * 30 {
            attempts += 1;
            let size = self.min_size + (self.max_size - self.min_size) * rng.next_f64().powi(2);
            let (dx, dz) = match self.region {
                Region::Disc(radius) => {
                    let r = (radius - size / 2.).max(0.) * rng.next_f64().sqrt();
                    let a = rng.next_f64() * std::f64::consts::TAU;
                    (r * a.cos(), r * a.sin())
                },
                Region::Rect(w, d) => ((rng.next_f64() - 0.5) * w, (rng.next_f64() - 0.5) * d),
            };
            let (x, z) = (self.center.x + dx, self.center.z + dz);
            let c = Vector3::new(x, self.ground(x, z), z);
            let material = ((rng.next_f64() * self.materials as f64) as usize).min(self.materials.saturating_sub(1));
            let (kx, kz) = key(x, z);
            let clear = (-1 ..= 1).all(|i| (-1 ..= 1).all(|j| {
                grid.get(&(kx + i, kz + j)).is_none_or(|near| near.iter().all(|&n| {
                    let p: &Pebble = &pebbles[n];
                    (p.center - c).norm() > (p.size + size) * 0.5 * self.packing
                }))
            }));
            if clear {
                grid.entry((kx, kz)).or_default().push(pebbles.len());
                pebbles.push(Pebble { center: c, size, material });
            }
        }
        pebbles
    }

    /// The meshes of laid-out pebbles, each turned at random and tilted a little off the ground.
    pub fn meshes(&self, pebbles: &[Pebble], detail: Option<u32>) -> Vec<IndexedMesh> {
        let up = self.up();
        let to_slope = Rotation3::rotation_between(&Vector3::y(), &up).unwrap_or_else(Rotation3::identity);
        pebbles.iter().enumerate().map(|(i, p)| {
            let pebble_seed = self.seed.wrapping_mul(1_000_003).wrapping_add(i as u64);
            let mut shape = RockShape::pebble(p.size, pebble_seed);
            if let Some(d) = detail { shape.detail = d }
            let mut mesh = shape.mesh();
            let mut rng = Pcg32::new(pebble_seed, 29);
            let yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), rng.next_f64() * std::f64::consts::TAU);
            // Closely packed pebbles lean on each other more.
            let lean = 0.25 + 0.5 * (1. - self.packing).max(0.);
            let tilt = Rotation3::from_euler_angles((rng.next_f64() - 0.5) * lean, 0., (rng.next_f64() - 0.5) * lean);
            place(&mut mesh, p.center, to_slope * tilt * yaw, up, 0.15);
            mesh
        }).collect()
    }
}

/// Settings for a bedrock outcrop.
#[derive(Clone, Debug)]
pub struct Bedrock {
    /// The centre of its base, and its extent across x and z.
    pub center: Vector3<f64>,
    pub size: (f64, f64),
    /// Its greatest height above the base.
    pub height: f64,
    /// The size of its larger features.
    pub feature_size: f64,
    /// The thickness of each terrace, 0 for none; and how sharply stepped they are, 0 to 1.
    pub strata: f64,
    pub strata_strength: f64,
    /// The fraction of each side over which it falls away to the base.
    pub falloff: f64,
    /// Vertices along each side.
    pub resolution: usize,
    pub seed: u64,
}

impl Bedrock {
    /// Height above the base at (x, z), relative to the centre.
    pub fn height_at(&self, noise: &SeededNoise, x: f64, z: f64) -> f64 {
        let q = Vector3::new(x, 0., z) / self.feature_size;
        let mass = 0.5 + 0.5 * noise.fbm(q * 0.6, 5);
        let ridges = noise.ridged(q * 1.3 + Vector3::new(0., 7., 0.), 5);
        let mut h = (0.7 * mass + 0.45 * ridges - 0.15).max(0.);

        // Fall away towards the edges.
        let edge = |v: f64, half: f64| (half - v.abs()) / (half * self.falloff.max(1e-6));
        let e = edge(x, self.size.0 / 2.).min(edge(z, self.size.1 / 2.)).clamp(0., 1.);
        h *= e * e * (3. - 2. * e);
        h *= self.height;

        // Terraces: each bed is flat on top, and steep where it breaks off to the one below.
        if self.strata > 0. && self.strata_strength > 0. {
            let tilted = h + noise.fbm(q * 0.3 + Vector3::new(0., 0., 3.), 2) * self.strata * 0.7;
            let bed = (tilted / self.strata).floor();
            let f = tilted / self.strata - bed;
            let stepped = (bed + f.powi(4)) * self.strata - (tilted - h);
            h += (stepped - h) * self.strata_strength.clamp(0., 1.);
        }
        h.max(0.)
    }

    pub fn mesh(&self) -> IndexedMesh {
        let noise = SeededNoise::new(self.seed);
        let n = self.resolution.max(2);
        let mut positions = Vec::with_capacity(n * n);
        for i in 0 .. n {
            for j in 0 .. n {
                let x = (i as f64 / (n - 1) as f64 - 0.5) * self.size.0;
                let z = (j as f64 / (n - 1) as f64 - 0.5) * self.size.1;
                positions.push(self.center + Vector3::new(x, self.height_at(&noise, x, z), z));
            }
        }
        let mut triangles = Vec::with_capacity(2 * (n - 1) * (n - 1));
        for i in 0 .. n - 1 {
            for j in 0 .. n - 1 {
                let (a, b, c, d) = (i * n + j, (i + 1) * n + j, (i + 1) * n + j + 1, i * n + j + 1);
                // Wound to face up.
                triangles.push([a, c, b]);
                triangles.push([a, d, c]);
            }
        }
        IndexedMesh { positions, normals: vec![], triangles }
    }
}

/// A medium of several materials, one for each pebble: each point takes the material of the
/// pebble whose centre is nearest, relative to its size. Pebbles are found through a grid.
pub struct Assorted {
    pub materials: Vec<Box<dyn MaterialModel + Sync + Send>>,
    pebbles: Vec<Pebble>,
    cell: f64,
    grid: HashMap<(i64, i64), Vec<usize>>,
}

impl Assorted {
    pub fn new(materials: Vec<Box<dyn MaterialModel + Sync + Send>>, pebbles: Vec<Pebble>) -> Assorted {
        assert!(!materials.is_empty(), "Assorted needs at least one material");
        let cell = pebbles.iter().map(|p| p.size).fold(1e-6, f64::max);
        let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (i, p) in pebbles.iter().enumerate() {
            grid.entry(Assorted::key(cell, &p.center)).or_default().push(i);
        }
        Assorted { materials, pebbles, cell, grid }
    }

    fn key(cell: f64, p: &Vector3<f64>) -> (i64, i64) {
        ((p.x / cell).floor() as i64, (p.z / cell).floor() as i64)
    }

    /// Which pebble a point belongs to.
    pub fn pebble_at(&self, pt: &Vector3<f64>) -> Option<&Pebble> {
        let (kx, kz) = Assorted::key(self.cell, pt);
        let mut best: Option<(f64, &Pebble)> = None;
        for dx in -1 ..= 1 {
            for dz in -1 ..= 1 {
                for &i in self.grid.get(&(kx + dx, kz + dz)).map(|v| v.as_slice()).unwrap_or(&[]) {
                    let p = &self.pebbles[i];
                    let d = (p.center - pt).norm() / p.size;
                    if best.is_none_or(|(b, _)| d < b) {
                        best = Some((d, p));
                    }
                }
            }
        }
        best.map(|(_, p)| p)
    }
}

impl Medium for Assorted {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        let i = self.pebble_at(&pt).map_or(0, |p| p.material);
        &*self.materials[i.min(self.materials.len() - 1)]
    }
}

pub fn object(meshes: &[IndexedMesh], medium: Box<dyn Medium + Sync + Send>) -> SceneObject {
    SceneObject { geometry: Box::new(SmoothMesh::from_indexed(meshes)), medium }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::mesh::vertex_normals;
    use crate::ray::Ray;

    /// Every edge is shared by exactly two triangles, traversed in opposite directions: the
    /// surface is closed and consistently wound.
    fn assert_closed(triangles: &[[usize; 3]]) {
        let mut edges: HashMap<(usize, usize), i32> = HashMap::new();
        for &[a, b, c] in triangles {
            for (u, v) in [(a, b), (b, c), (c, a)] {
                *edges.entry((u, v)).or_insert(0) += 1;
            }
        }
        for (&(u, v), &n) in &edges {
            assert_eq!(n, 1, "edge {}-{} repeated", u, v);
            assert_eq!(edges.get(&(v, u)), Some(&1), "edge {}-{} has no twin", u, v);
        }
    }

    #[test]
    fn test_icosphere_is_closed_and_faces_out() {
        let (p, t) = icosphere(2);
        assert_eq!(t.len(), 20 * 16);
        assert_eq!(p.len(), 162);
        assert_closed(&t);
        for (v, n) in p.iter().zip(vertex_normals(&p, &t)) {
            assert!((v.norm() - 1.).abs() < 1e-12);
            assert!(v.dot(&n) > 0.9);
        }
    }

    #[test]
    fn test_rocks_are_reproducible_and_vary_by_seed() {
        let a = RockShape::boulder(1., 0.5, 3).mesh().positions;
        assert_eq!(a, RockShape::boulder(1., 0.5, 3).mesh().positions);
        assert_ne!(a, RockShape::boulder(1., 0.5, 4).mesh().positions);
    }

    #[test]
    fn test_boulder_faces_out_and_has_flat_faces() {
        let shape = RockShape::boulder(2., 1., 7);
        let mesh = shape.mesh();
        assert_closed(&mesh.triangles);
        let normals = vertex_normals(&mesh.positions, &mesh.triangles);
        let outward = mesh.positions.iter().zip(&normals).filter(|(p, n)| p.dot(n) > 0.).count();
        assert!(outward as f64 > 0.97 * mesh.positions.len() as f64, "{}", outward);
        // Fractures make it smaller than its ellipsoid, but not much.
        let r = mesh.positions.iter().map(|p| p.norm()).fold(0., f64::max);
        assert!(r < shape.axes.max() * 1.3 && r > shape.axes.min() * 0.6, "{}", r);
    }

    #[test]
    fn test_boulder_rests_on_the_ground() {
        let at = Vector3::new(3., 1., -2.);
        let mesh = boulder_mesh(at, 1., 0.5, Some(3), 1);
        let (lo, hi) = mesh.positions.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.y), hi.max(p.y)));
        assert!(lo < at.y && hi > at.y + 0.5);
        assert!((at.y - lo) / (hi - lo) - 0.12 < 1e-9);
        let o = object(&[mesh], Box::new(crate::material::texture::Solid {
            m: Box::new(crate::material::lambertian::Lambertian { albedo: crate::color::Color::white() }) }));
        // A ray straight down onto it hits its top.
        let hit = o.intersects(&Ray { ro: at + Vector3::new(0., 10., 0.), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!(hit.point.y > at.y && hit.normal.y > 0.);
    }

    fn field(region: Region, count: usize, min_size: f64, max_size: f64, materials: usize, seed: u64) -> PebbleField {
        PebbleField { center: Vector3::zeros(), region, count, min_size, max_size, packing: 1., slope: (0., 0.), materials, seed }
    }

    #[test]
    fn test_pebbles_follow_a_sloping_rectangle() {
        let f = PebbleField { center: Vector3::new(0., 1., 0.), slope: (0., -0.1), packing: 0.8,
            ..field(Region::Rect(2., 4.), 400, 0.05, 0.1, 1, 3) };
        let pebbles = f.scatter();
        assert!(pebbles.len() > 300, "{}", pebbles.len());
        for p in &pebbles {
            assert!(p.center.x.abs() <= 1. && p.center.z.abs() <= 2.);
            assert!((p.center.y - (1. - 0.1 * p.center.z)).abs() < 1e-12);
        }
        let meshes = f.meshes(&pebbles[.. 5], Some(1));
        for (m, p) in meshes.iter().zip(&pebbles) {
            // Resting on the slope: some of each pebble is below the ground, most above it.
            let up = f.up();
            let below = m.positions.iter().filter(|v| (*v - p.center).dot(&up) < 0.).count();
            assert!(below > 0 && below < m.positions.len() / 2, "{}", below);
        }
    }

    #[test]
    fn test_pebbles_do_not_overlap() {
        let pebbles = field(Region::Disc(2.), 200, 0.05, 0.2, 3, 1).scatter();
        assert!(pebbles.len() > 50, "{}", pebbles.len());
        for (i, a) in pebbles.iter().enumerate() {
            assert!(a.material < 3);
            assert!(a.center.norm() + a.size / 2. <= 2. + 1e-9);
            for b in &pebbles[i + 1 ..] {
                assert!((a.center - b.center).norm() > (a.size + b.size) / 2.);
            }
        }
    }

    #[test]
    fn test_assorted_finds_each_pebble() {
        let pebbles = field(Region::Disc(1.), 40, 0.1, 0.2, 2, 2).scatter();
        let m = Assorted::new(vec![
            Box::new(crate::material::lambertian::Lambertian { albedo: crate::color::Color::white() }),
        ], pebbles.clone());
        for p in &pebbles {
            assert_eq!(m.pebble_at(&(p.center + Vector3::new(0., p.size * 0.1, 0.))), Some(p));
        }
    }

    #[test]
    fn test_bedrock_falls_away_at_edges_and_is_terraced() {
        let b = Bedrock {
            center: Vector3::zeros(), size: (20., 20.), height: 4., feature_size: 5.,
            strata: 0.5, strata_strength: 1., falloff: 0.3, resolution: 41, seed: 1,
        };
        let noise = SeededNoise::new(1);
        assert_eq!(b.height_at(&noise, 10., 0.), 0.);
        assert_eq!(b.height_at(&noise, 3., -10.), 0.);
        let mesh = b.mesh();
        assert_eq!(mesh.positions.len(), 41 * 41);
        let top = mesh.positions.iter().map(|p| p.y).fold(0., f64::max);
        assert!(top > 0.5 && top <= 4.5, "{}", top);
        let level = |b: &Bedrock| {
            let mesh = b.mesh();
            let normals = vertex_normals(&mesh.positions, &mesh.triangles);
            assert!(normals.iter().all(|n| n.y > 0.));
            normals.iter().filter(|n| n.y > 0.98).count()
        };
        // Terraces leave more of the surface level.
        let smooth = Bedrock { strata_strength: 0., ..b.clone() };
        assert!(level(&b) as f64 > 1.2 * level(&smooth) as f64, "{} vs {}", level(&b), level(&smooth));
    }
}
