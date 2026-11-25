//! Measurements of the BVH (`crate::bvh`): build time, the tree's cost by the surface area
//! heuristic, and the time and work per ray of its searches, over meshes of a few kinds and rays
//! of the kinds a path tracer casts. Run with
//! `cargo test --release measure_bvh -- --ignored --nocapture`.

use crate::bvh::Bvh;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::triangle::{intersects_dist, triangle_bounds};
use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use std::time::Instant;

pub(crate) struct TestMesh {
    pub name: String,
    /// The OBJ file it was read from, if any.
    pub path: Option<String>,
    pub positions: Vec<Vector3<f64>>,
    pub triangles: Vec<[u32; 3]>,
}

pub(crate) fn from_file(path: &str) -> TestMesh {
    let parts = crate::shapes::mesh::load(path);
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    for m in parts {
        let base = positions.len() as u32;
        positions.extend(m.positions.iter().copied());
        triangles.extend(m.triangles.iter().map(|t| t.map(|i| base + i as u32)));
    }
    TestMesh { name: path.rsplit('/').next().unwrap().into(), path: Some(path.into()), positions, triangles }
}

/// A sphere of `n` by `n` quads in latitude and longitude, its radius rippled by noise: slivers
/// at the poles, small triangles of even size elsewhere, as a scan.
pub(crate) fn bumpy_sphere(n: usize) -> TestMesh {
    let mut positions = Vec::new();
    for i in 0 ..= n {
        let theta = std::f64::consts::PI * i as f64 / n as f64;
        for j in 0 .. n {
            let phi = 2. * std::f64::consts::PI * j as f64 / n as f64;
            let d = Vector3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());
            let r = 1. + 0.05 * (13. * d.x).sin() * (11. * d.y).cos() + 0.02 * (41. * d.z).sin();
            positions.push(d * r);
        }
    }
    let at = |i: usize, j: usize| (i * n + j % n) as u32;
    let mut triangles = Vec::new();
    for i in 0 .. n {
        for j in 0 .. n {
            triangles.push([at(i, j), at(i + 1, j), at(i + 1, j + 1)]);
            triangles.push([at(i, j), at(i + 1, j + 1), at(i, j + 1)]);
        }
    }
    TestMesh { name: format!("sphere {}k", triangles.len() / 1000), path: None, positions, triangles }
}

/// Small triangles of random size and orientation strewn through a cube, as leaves of a tree;
/// a few large ones among them.
pub(crate) fn soup(count: usize) -> TestMesh {
    let mut rng = StdRng::seed_from_u64(11);
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    for k in 0 .. count {
        let c = Vector3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.));
        let size = if k % 200 == 0 { 0.5 } else { rng.random_range(0.005..0.03) };
        let base = positions.len() as u32;
        for _ in 0..3 {
            positions.push(c + Vector3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.)) * size);
        }
        triangles.push([base, base + 1, base + 2]);
    }
    TestMesh { name: format!("soup {}k", count / 1000), path: None, positions, triangles }
}

/// A mesh behind a BVH, searched as `shapes::mesh` searches it.
pub(crate) struct Searched {
    pub bvh: Bvh,
    pub binary: crate::bvh::BinaryBvh,
    pub positions: Vec<Vector3<f64>>,
    pub triangles: Vec<[u32; 3]>,
    pub build_seconds: f64,
}

impl Searched {
    pub fn new(m: &TestMesh) -> Searched {
        let start = Instant::now();
        let positions = m.positions.clone();
        let (binary, triangles) = crate::bvh::BinaryBvh::build(m.triangles.clone(), |&[a, b, c]| {
            triangle_bounds(positions[a as usize], positions[b as usize], positions[c as usize])
        });
        let build_seconds = start.elapsed().as_secs_f64();
        let bvh = Bvh::from_binary_for_tests(&binary);
        Searched { bvh, binary, positions, triangles, build_seconds }
    }

    fn vertices(&self, i: usize) -> [Vector3<f64>; 3] {
        self.triangles[i].map(|v| self.positions[v as usize])
    }

    /// As `shapes::mesh` searches.
    pub fn nearest(&self, r: &Ray, min: f64, max: f64) -> Option<(usize, f64)> {
        let len = r.rd.norm();
        self.bvh.nearest_in_leaves(r, min, max, |items, best| {
            let mut found = None;
            let mut best = best;
            for i in items {
                let [v0, v1, v2] = self.vertices(i);
                if let Some(t) = crate::shapes::triangle::intersects_t(v0, v1, v2, r) {
                    let dist = t * len;
                    if dist >= min && dist < best {
                        best = dist;
                        found = Some(i);
                    }
                }
            }
            found.map(|i| (i, crate::intersection::RawIntersection { part: 0, face: 0, dist: best, point: Vector3::zeros(), normal: Vector3::zeros() }))
        }).map(|(i, x)| (i, x.dist))
    }

    pub fn nearest_binary(&self, r: &Ray, min: f64, max: f64) -> Option<(usize, f64)> {
        self.binary.nearest(r, min, max, |i, _| {
            let [a, b, c] = self.vertices(i);
            intersects_dist(a, b, c, r).map(|x| crate::intersection::RawIntersection {
                part: 0, face: 0, dist: x.dist, point: x.point, normal: Vector3::zeros() })
        }).map(|(i, x)| (i, x.dist))
    }

    pub fn occluded_binary(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.binary.any_hit(r, min, max, |i| {
            let [a, b, c] = self.vertices(i);
            intersects_dist(a, b, c, r).is_some_and(|x| x.dist >= min && x.dist < max)
        })
    }

    pub fn occluded(&self, r: &Ray, min: f64, max: f64) -> bool {
        let len = r.rd.norm();
        self.bvh.any_hit(r, min, max, |i| {
            let [a, b, c] = self.vertices(i);
            crate::shapes::triangle::intersects_t(a, b, c, r).is_some_and(|t| { let d = t * len; d >= min && d < max })
        })
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    /// A point on triangle `i` and its normal.
    fn on(&self, i: usize, rng: &mut StdRng) -> (Vector3<f64>, Vector3<f64>) {
        let [a, b, c] = self.vertices(i);
        let p = crate::shapes::triangle::sample_triangle(a, b, c, rng.random(), rng.random());
        let n = (b - a).cross(&(c - a)).try_normalize(0.).unwrap_or(Vector3::y());
        (p, n)
    }
}

pub(crate) struct Rays {
    /// Rays from a camera outside, at points within the bounds.
    pub primary: Vec<Ray>,
    /// From points on the surface, in directions uniform over the sphere, starting just off it.
    pub diffuse: Vec<Ray>,
    /// From points on the surface to a light outside, and the distance to it.
    pub shadow: Vec<(Ray, f64)>,
}

fn unit(rng: &mut StdRng) -> Vector3<f64> {
    loop {
        let v = Vector3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.));
        if v.norm_squared() <= 1. && v.norm_squared() > 1e-6 { return v.normalize() }
    }
}

pub(crate) fn rays(m: &Searched, n: usize) -> Rays {
    let mut rng = StdRng::seed_from_u64(5);
    let b = m.bounds();
    let (mid, diag) = (b.mid(), b.size().norm());
    let eye = mid + Vector3::new(0.6, 0.5, -1.).normalize() * diag * 1.2;
    let light = mid + Vector3::new(-0.3, 1., 0.4).normalize() * diag * 2.;
    let inside = |rng: &mut StdRng| Vector3::new(rng.random_range(b.min.x..=b.max.x), rng.random_range(b.min.y..=b.max.y), rng.random_range(b.min.z..=b.max.z));
    let primary = (0..n).map(|_| Ray { ro: eye, rd: (inside(&mut rng) - eye).normalize() }).collect();
    let eps = diag * 1e-7;
    let surface: Vec<_> = (0..n).map(|_| { let i = rng.random_range(0..m.triangles.len()); m.on(i, &mut rng) }).collect();
    let diffuse = surface.iter().map(|&(p, nrm)| {
        let mut d = unit(&mut rng);
        if d.dot(&nrm) < 0. && rng.random::<f64>() < 0.5 { d = -d }
        Ray { ro: p + d * eps, rd: d }
    }).collect();
    let shadow = surface.iter().map(|&(p, _)| {
        let to = light - p;
        (Ray { ro: p + to.normalize() * eps, rd: to.normalize() }, to.norm())
    }).collect();
    Rays { primary, diffuse, shadow }
}

/// Nanoseconds per ray of `f` over `rays`, the best of a few runs.
fn time<T>(rays: &[T], mut f: impl FnMut(&T) -> usize) -> (f64, usize) {
    let mut best = f64::INFINITY;
    let mut hits = 0;
    for _ in 0..5 {
        let start = Instant::now();
        hits = rays.iter().map(&mut f).sum::<usize>();
        best = best.min(start.elapsed().as_secs_f64());
    }
    (best * 1e9 / rays.len() as f64, hits)
}

pub(crate) fn meshes() -> Vec<TestMesh> {
    if let Ok(only) = std::env::var("BVH_MEASURE") {
        if only == "teapot" { return vec![from_file("demo/teapot.obj")] }
        if only == "sphere" { return vec![bumpy_sphere(500)] }
    }
    vec![
        from_file("demo/teapot.obj"),
        from_file("demo/boat.obj"),
        bumpy_sphere(500),
        soup(300_000),
    ]
}

impl TestMesh {
    /// An OBJ file of it.
    fn obj(&self) -> String {
        if let Some(p) = &self.path { return p.clone() }
        use std::io::Write;
        std::fs::create_dir_all("target/bvh-measure").unwrap();
        let path = format!("target/bvh-measure/{}.obj", self.name.replace(' ', "-"));
        let mut f = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
        for p in &self.positions { writeln!(f, "v {} {} {}", p.x, p.y, p.z).unwrap() }
        for t in &self.triangles { writeln!(f, "f {} {} {}", t[0] + 1, t[1] + 1, t[2] + 1).unwrap() }
        path
    }
}

/// The same rays through the kernels' traversal (`pathtrace::nearest`, in f32, over the same tree
/// laid out as `kernels::gpu_nodes`), on the CPU: nanoseconds per primary, diffuse and shadow ray.
fn kernels(m: &TestMesh, rays: &Rays) -> (f64, f64, f64) {
    use pathtrace::glam::Vec3;
    let s = crate::scenefile::SceneFile::from_string(format!(r#"{{
        "width": 4, "height": 4, "background": [0, 0, 0],
        "camera": {{ "location": [0, 0, -30], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
        "materials": {{ "M": {{ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }} }},
        "objects": [{{ "type": "mesh", "src": "{}", "material": "M" }}]
    }}"#, m.obj()));
    let flat = crate::wavefront::flat::FlatScene::build(&s).unwrap();
    let b = crate::kernels::Buffers::build_at(&s, &flat, &Vector3::zeros()).unwrap();
    let shader = b.scene(None);
    let v = |v: Vector3<f64>| Vec3::new(v.x as f32, v.y as f32, v.z as f32);
    let (p, _) = time(&rays.primary, |r| (pathtrace::nearest(&shader, v(r.ro), v(r.rd), 0., f32::MAX, false).1 != u32::MAX) as usize);
    let (d, _) = time(&rays.diffuse, |r| (pathtrace::nearest(&shader, v(r.ro), v(r.rd), 0., f32::MAX, false).1 != u32::MAX) as usize);
    let (sh, _) = time(&rays.shadow, |(r, max)| (pathtrace::nearest(&shader, v(r.ro), v(r.rd), 0., *max as f32, true).1 != u32::MAX) as usize);
    (p, d, sh)
}

#[test]
#[ignore]
fn measure_triangle_test() {
    let (a, b, c) = (Vector3::new(0., 0., 0.), Vector3::new(1., 0., 0.), Vector3::new(0., 1., 0.));
    let rays: Vec<Ray> = (0..1000).map(|i| Ray { ro: Vector3::new(0.001 * i as f64, 0.3, -1.), rd: Vector3::new(0., 0., 1.) }).collect();
    let start = Instant::now();
    let mut hits = 0;
    for _ in 0..10000 {
        for r in &rays {
            hits += intersects_dist(std::hint::black_box(a), b, c, r).is_some() as usize;
        }
    }
    println!("{} hits, {:.1}ns a test", hits, start.elapsed().as_secs_f64() * 1e9 / 1e7);
    #[inline(never)]
    fn plain(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, r: &Ray) -> Option<f64> {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let p = r.rd.cross(&e2);
        let det = e1.dot(&p);
        if det == 0. { return None }
        let inv = 1. / det;
        let t = r.ro - v0;
        let u = t.dot(&p) * inv;
        if !(0. ..=1.).contains(&u) { return None }
        let q = t.cross(&e1);
        let v = r.rd.dot(&q) * inv;
        if v < 0. || u + v > 1. { return None }
        let d = e2.dot(&q) * inv;
        (d > 0.).then_some(d)
    }
    let start = Instant::now();
    let mut hits = 0;
    for _ in 0..10000 {
        for r in &rays {
            hits += plain(std::hint::black_box(a), b, c, r).is_some() as usize;
        }
    }
    println!("{} hits, {:.1}ns a plain test", hits, start.elapsed().as_secs_f64() * 1e9 / 1e7);
    let start = Instant::now();
    let mut hits = 0.;
    for _ in 0..10000 {
        for r in &rays {
            hits += std::hint::black_box(r).rd.norm();
        }
    }
    println!("{} {:.1}ns a norm", hits, start.elapsed().as_secs_f64() * 1e9 / 1e7);
    let n = Vec::from_iter((0..10_000_000u64).map(|i| i.wrapping_mul(2654435761) % 1000));
    let start = Instant::now();
    let mut sum = 0;
    for &i in &n { sum += i }
    println!("{} {:.2}ns an add", sum, start.elapsed().as_secs_f64() * 1e9 / 1e7);
}

#[test]
#[ignore]
fn measure_bvh() {
    println!("{:<14} {:>8} {:>8} {:>8} {:>8} {:>8} | {:>9} {:>9} {:>9} | {:>8} {:>8}",
        "mesh", "tris", "nodes", "build ms", "SAH", "leaf avg", "primary", "diffuse", "shadow", "prim/ray", "hits");
    let mut kernel_rows = Vec::new();
    for m in meshes() {
        let s = Searched::new(&m);
        let rays = rays(&s, std::env::var("BVH_RAYS").map_or(200_000, |n| n.parse().unwrap()));
        let (p, hp) = time(&rays.primary, |r| s.nearest(r, 0., f64::INFINITY).is_some() as usize);
        let (d, _) = time(&rays.diffuse, |r| s.nearest(r, 0., f64::INFINITY).is_some() as usize);
        let (sh, _) = time(&rays.shadow, |(r, max)| s.occluded(r, 0., *max) as usize);
        // Triangles tested per diffuse ray.
        let tests = std::cell::Cell::new(0usize);
        for r in &rays.diffuse {
            s.bvh.nearest(r, 0., f64::INFINITY, |i, _| {
                tests.set(tests.get() + 1);
                let [a, b, c] = s.vertices(i);
                intersects_dist(a, b, c, r).map(|x| crate::intersection::RawIntersection { part: 0, face: 0, dist: x.dist, point: x.point, normal: Vector3::zeros() })
            });
        }
        let (mut interior, mut leaf_visits) = (0, 0);
        for r in &rays.diffuse {
            let at = s.nearest(r, 0., f64::INFINITY).map_or(f64::INFINITY, |x| x.1);
            let (i, l) = s.binary.visits(r, 0., f64::INFINITY, at);
            interior += i;
            leaf_visits += l;
        }
        println!("  diffuse: {:.1} interior nodes, {:.1} leaves a ray", interior as f64 / rays.diffuse.len() as f64, leaf_visits as f64 / rays.diffuse.len() as f64);
        let (cost, leaves) = s.binary.sah_cost();
        println!("{:<14} {:>8} {:>8} {:>8.1} {:>8.2} {:>8.2} | {:>7.0}ns {:>7.0}ns {:>7.0}ns | {:>8.1} {:>8}",
            m.name, m.triangles.len(), s.bvh.node_count(), s.build_seconds * 1e3, cost, m.triangles.len() as f64 / leaves as f64,
            p, d, sh, tests.get() as f64 / rays.diffuse.len() as f64, hp);
        let (bp, _) = time(&rays.primary, |r| s.nearest_binary(r, 0., f64::INFINITY).is_some() as usize);
        let (bd, _) = time(&rays.diffuse, |r| s.nearest_binary(r, 0., f64::INFINITY).is_some() as usize);
        let (bs, _) = time(&rays.shadow, |(r, max)| s.occluded_binary(r, 0., *max) as usize);
        println!("{:<14} {:>53} | {:>7.0}ns {:>7.0}ns {:>7.0}ns |  (binary)", "", "", bp, bd, bs);
        // Traversal alone: each ray cut off at its own hit, with leaves found but not searched.
        let cut: Vec<(Ray, f64)> = rays.diffuse.iter().map(|r| (*r, s.nearest(r, 0., f64::INFINITY).map_or(f64::INFINITY, |x| x.1))).collect();
        let (tw, _) = time(&cut, |(r, t)| s.bvh.nearest_in_leaves(r, 0., *t, |_, _| None).is_some() as usize);
        let (tb, _) = time(&cut, |(r, t)| s.binary.nearest_in_leaves(r, 0., *t, |_, _| None).is_some() as usize);
        // And the triangles alone: those a search tests, tested again in a row.
        let mut tested = Vec::new();
        for r in rays.diffuse.iter().take(20000) {
            s.bvh.nearest(r, 0., f64::INFINITY, |i, _| { tested.push((r.clone(), i)); None });
        }
        let (tt, _) = time(&tested, |(r, i)| { let [a, b, c] = s.vertices(*i); intersects_dist(a, b, c, r).is_some() as usize });
        println!("  traversal alone: wide {:.0}ns, binary {:.0}ns; {:.1}ns a triangle", tw, tb, tt);
        // The same hits.
        for r in &rays.diffuse {
            assert_eq!(s.nearest(r, 0., f64::INFINITY), s.nearest_binary(r, 0., f64::INFINITY));
        }
        for (r, max) in &rays.shadow {
            assert_eq!(s.occluded(r, 0., *max), s.occluded_binary(r, 0., *max));
        }
        kernel_rows.push((m.name.clone(), kernels(&m, &rays)));
    }
    println!("The kernels' traversal (f32), on the CPU:");
    for (name, (p, d, sh)) in kernel_rows {
        println!("{:<14} {:>53} | {:>7.0}ns {:>7.0}ns {:>7.0}ns |", name, "", p, d, sh);
    }
}

/// For profiling under cachegrind: the traversal alone of the diffuse rays, cut at their hits,
/// through the tree `BVH_TREE` names (wide, binary or none, for the set-up alone).
#[test]
#[ignore]
fn measure_traversal_alone() {
    let m = meshes().remove(0);
    let s = Searched::new(&m);
    let rays = rays(&s, std::env::var("BVH_RAYS").map_or(20_000, |n| n.parse().unwrap()));
    let cut: Vec<(Ray, f64)> = rays.diffuse.iter().map(|r| (*r, s.nearest(r, 0., f64::INFINITY).map_or(f64::INFINITY, |x| x.1))).collect();
    let tree = std::env::var("BVH_TREE").unwrap_or("wide".into());
    let n: usize = match tree.as_str() {
        "wide" => cut.iter().map(|(r, t)| s.bvh.nearest_in_leaves(r, 0., *t, |_, _| None).is_some() as usize).sum(),
        "binary" => cut.iter().map(|(r, t)| s.binary.nearest_in_leaves(r, 0., *t, |_, _| None).is_some() as usize).sum(),
        "triangles" => cut.iter().map(|(r, _)| s.nearest(r, 0., f64::INFINITY).is_some() as usize).sum(),
        _ => 0,
    };
    println!("{} {}", tree, n);
}
