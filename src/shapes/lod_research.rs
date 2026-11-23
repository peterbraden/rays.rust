//! Measurements of the leaf cards for docs/research/learned-lod (stage 0), run by hand:
//!
//!     cargo test --release --lib lod_research -- --ignored --nocapture
//!
//! For trees of several species, at several levels, each cluster `consolidate` makes is measured
//! against the leaves it stands for:
//!
//! - **Constituents**: how many leaves a cluster has, and how many its card keeps.
//! - **Hit probability**: the fraction of parallel rays from a direction that hit the leaves,
//!   and the fraction that hit the card.
//! - **The constituent distribution**: the expected response of a leaf chosen by each model,
//!   reflected `ρ max(0, n'·l)` plus transmitted `τ max(0, -n'·l)` (`n'` facing the viewer),
//!   averaged over view and light directions, against the leaves chosen by `a|n·d|`. The models:
//!   the card's kept constituents; one SGGX lobe fitted as Heitz et al. 2015 (Sec. 4.3) and
//!   sampled by their Algorithm 1; reduced zonotopes of 4 and 8 representative leaves, with
//!   their areas merged, or refitted by non-negative least squares to the projected area over
//!   directions; and the card's own normal alone.
//! - **Self-shadowing**: for rays from a view direction that hit the leaves, the response of the
//!   leaf actually hit, unshadowed and shadowed toward the light by the cluster's own leaves,
//!   against the `a|n·d|` choice, which sees every leaf and shadows none.
//!
//! Results are printed and written as CSV to `$LOD_RESEARCH_OUT` (default
//! `target/lod-research.csv`).
use super::*;
use crate::geometry::Pcg32;
use crate::procedural::tree;

/// The most constituents a card kept before `constituents`: an even sample of a larger cluster.
const OLD_KEPT: usize = 32;

/// Leaf reflectance and transmittance (the forest scenes' summer leaf, green).
const RHO: f64 = 0.42;
const TAU: f64 = 0.36;

fn area(t: &Triangle) -> f64 {
    0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm()
}

/// `n` directions spread evenly over the sphere (a Fibonacci lattice).
fn sphere(n: usize) -> Vec<Vector3<f64>> {
    let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
    (0 .. n).map(|i| {
        let y = 1. - 2. * (i as f64 + 0.5) / n as f64;
        let r = (1. - y * y).sqrt();
        let phi = golden * i as f64;
        Vector3::new(r * phi.cos(), y, r * phi.sin())
    }).collect()
}

/// A leaf's response to light from `l`, seen along `d`: reflected if the light is on the side
/// facing the viewer, transmitted if behind.
fn response(n: &Vector3<f64>, d: &Vector3<f64>, l: &Vector3<f64>) -> f64 {
    let facing = if n.dot(d) < 0. { *n } else { -*n };
    let c = facing.dot(l);
    if c > 0. { RHO * c } else { -TAU * c }
}

/// Constituents: normal and area.
type Set = Vec<(Vector3<f64>, f64)>;

/// Half the projected area of `set` along `d`: the support function of its zonotope.
fn sigma(set: &Set, d: &Vector3<f64>) -> f64 {
    0.5 * set.iter().map(|(n, a)| a * n.dot(d).abs()).sum::<f64>()
}

/// The expected response of a constituent of `set` chosen by `a|n·d|`.
fn chosen(set: &Set, d: &Vector3<f64>, l: &Vector3<f64>) -> f64 {
    let (mut sum, mut weight) = (0., 0.);
    for (n, a) in set {
        let w = a * n.dot(d).abs();
        sum += w * response(n, d, l);
        weight += w;
    }
    if weight > 0. { sum / weight } else { 0. }
}

/// The SGGX matrix fitted to `set` as Heitz et al. do (Sec. 4.3): the eigenvectors of the
/// normals' second moment, with the squared projected areas along them as eigenvalues.
fn sggx(set: &Set) -> Matrix3<f64> {
    let mut moment = Matrix3::zeros();
    for (n, a) in set {
        moment += n * n.transpose() * *a;
    }
    let eigen = moment.symmetric_eigen();
    let mut s = Matrix3::zeros();
    for k in 0 .. 3 {
        let w: Vector3<f64> = eigen.eigenvectors.column(k).into_owned();
        // Projected area of the whole set along w, as SGGX's σ(ω) = ∫ ⟨ω, m⟩ D(m) dm, with a
        // floor so the matrix stays invertible for perfectly flat clusters.
        let s_k = (2. * sigma(set, &w)).max(1e-4 * set.iter().map(|k| k.1).sum::<f64>());
        s += w * w.transpose() * (s_k * s_k);
    }
    s
}

/// An orthonormal frame `(k, j)` completing `i`.
fn frame(i: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let any = if i.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let k = i.cross(&any).normalize();
    (k, i.cross(&k))
}

/// A visible normal of SGGX `s` seen from `wi`, by Heitz et al.'s Algorithm 1.
fn sggx_sample(s: &Matrix3<f64>, wi: &Vector3<f64>, u1: f64, u2: f64) -> Vector3<f64> {
    let (wk, wj) = frame(wi);
    let q = |a: &Vector3<f64>, b: &Vector3<f64>| a.dot(&(s * b));
    let (s_kk, s_jj, s_ii) = (q(&wk, &wk), q(&wj, &wj), q(wi, wi));
    let (s_kj, s_ki, s_ji) = (q(&wk, &wj), q(&wk, wi), q(&wj, wi));
    let det = (s_kk * s_jj * s_ii - s_kj * s_kj * s_ii - s_ki * s_ki * s_jj - s_ji * s_ji * s_kk + 2. * s_kj * s_ki * s_ji).abs();
    let tmp = (s_jj * s_ii - s_ji * s_ji).max(1e-300).sqrt();
    let mk = Vector3::new(det.sqrt() / tmp, 0., 0.);
    let mj = Vector3::new(-(s_ki * s_ji - s_kj * s_ii) / tmp, tmp, 0.) / s_ii.sqrt();
    let mi = Vector3::new(s_ki, s_ji, s_ii) / s_ii.sqrt();
    let r = u1.sqrt();
    let (u, v) = (r * (2. * std::f64::consts::PI * u2).cos(), r * (2. * std::f64::consts::PI * u2).sin());
    let w = (1. - u * u - v * v).max(0.).sqrt();
    let m = (mk * u + mj * v + mi * w).normalize();
    wk * m.x + wj * m.y + wi * m.z
}

/// The expected response of a visible normal of SGGX `s`, by `side`² stratified samples.
fn sggx_chosen(s: &Matrix3<f64>, d: &Vector3<f64>, l: &Vector3<f64>, side: usize) -> f64 {
    let wi = -d;
    let mut sum = 0.;
    for a in 0 .. side {
        for b in 0 .. side {
            let m = sggx_sample(s, &wi, (a as f64 + 0.5) / side as f64, (b as f64 + 0.5) / side as f64);
            sum += response(&m, d, l);
        }
    }
    sum / (side * side) as f64
}

/// `m` representative constituents of `set`: axial k-means on the normals (a leaf's two faces
/// alike), each group represented by its member nearest the group's axis (an original leaf, so
/// its face could be kept) with the group's area. With `refit`, the areas are then refitted by
/// non-negative least squares so the projected area matches the set's over `dirs`.
fn reduced(set: &Set, m: usize, refit: bool, dirs: &[Vector3<f64>]) -> Set {
    if set.len() <= m {
        return set.clone();
    }
    // Farthest-point start, from the largest leaf.
    let mut reps: Vec<Vector3<f64>> = vec![set.iter().max_by(|a, b| a.1.total_cmp(&b.1)).unwrap().0];
    while reps.len() < m {
        let next = set.iter().max_by(|a, b| {
            let f = |k: &(Vector3<f64>, f64)| k.1 * reps.iter().map(|r| 1. - k.0.dot(r).abs()).fold(f64::INFINITY, f64::min);
            f(a).total_cmp(&f(b))
        }).unwrap();
        reps.push(next.0);
    }
    let assign = |reps: &[Vector3<f64>]| -> Vec<usize> {
        set.iter().map(|(n, _)| (0 .. reps.len()).max_by(|&a, &b| n.dot(&reps[a]).abs().total_cmp(&n.dot(&reps[b]).abs())).unwrap()).collect()
    };
    for _ in 0 .. 8 {
        let groups = assign(&reps);
        for (g, rep) in reps.iter_mut().enumerate() {
            let mut moment = Matrix3::zeros();
            for (k, (n, a)) in set.iter().enumerate() {
                if groups[k] == g {
                    moment += n * n.transpose() * *a;
                }
            }
            if moment.norm() > 0. {
                let e = moment.symmetric_eigen();
                *rep = e.eigenvectors.column(e.eigenvalues.imax()).into_owned();
            }
        }
    }
    let groups = assign(&reps);
    let mut out: Set = vec![];
    for (g, rep) in reps.iter().enumerate() {
        let members: Vec<&(Vector3<f64>, f64)> = set.iter().enumerate().filter(|(k, _)| groups[*k] == g).map(|(_, x)| x).collect();
        if members.is_empty() {
            continue;
        }
        let medoid = members.iter().max_by(|a, b| a.0.dot(rep).abs().total_cmp(&b.0.dot(rep).abs())).unwrap().0;
        out.push((medoid, members.iter().map(|k| k.1).sum()));
    }
    if refit {
        // Minimise Σ_d (Σ_j b_j |n_j·d| - Σ_k a_k |n_k·d|)² over b ≥ 0, by cyclic coordinate
        // descent (convex, and only a few unknowns).
        let a: Vec<Vec<f64>> = dirs.iter().map(|d| out.iter().map(|(n, _)| n.dot(d).abs()).collect()).collect();
        let target: Vec<f64> = dirs.iter().map(|d| 2. * sigma(set, d)).collect();
        let mut b: Vec<f64> = out.iter().map(|k| k.1).collect();
        for _ in 0 .. 300 {
            for j in 0 .. b.len() {
                let (mut num, mut den) = (0., 0.);
                for (row, t) in a.iter().zip(&target) {
                    let rest: f64 = row.iter().zip(&b).enumerate().filter(|(i, _)| *i != j).map(|(_, (x, y))| x * y).sum();
                    num += row[j] * (t - rest);
                    den += row[j] * row[j];
                }
                if den > 0. {
                    b[j] = (num / den).max(0.);
                }
            }
        }
        for (k, bj) in out.iter_mut().zip(b) {
            k.1 = bj;
        }
        out.retain(|k| k.1 > 0.);
    }
    out
}

/// The largest relative error of `model`'s projected area against `set`'s over `dirs`.
fn sigma_error(set: &Set, model: &Set, dirs: &[Vector3<f64>]) -> f64 {
    dirs.iter().map(|d| (sigma(model, d) / sigma(set, d) - 1.).abs()).fold(0., f64::max)
}

/// Accumulated measurements of one tree at one level.
#[derive(Default)]
struct Tally {
    clusters: usize,
    cards: usize,
    members: usize,
    kept: usize,
    /// Constituents left if a leaf's triangles with the same normal are merged (≤ 32).
    merged: usize,
    over: usize,
    /// Per model: Σ over sampled clusters and direction pairs of the model's response, and of
    /// its absolute error, weighted by the cluster's projected area.
    model: [(f64, f64); 9],
    /// Constituents per card as `constituents` keeps them.
    new_kept: usize,
    truth: f64,
    weight: f64,
    sigma_err: [f64; 4],
    sampled: usize,
    /// Σ over sampled clusters of hits on the leaves and on the card, per ray.
    hits: (f64, f64),
    /// The same from the side (views within 17° of horizontal).
    side_hits: (f64, f64),
    /// From the side: Σ response of the a|n·d| choice, and of the leaf hit (unshadowed).
    side_cast: (f64, f64),
    /// From ray casting: Σ response of the a|n·d| choice, of the leaf hit, of the leaf hit
    /// shadowed by its own cluster; all weighted by hits.
    cast: (f64, f64, f64),
    /// Σ over leaves (by area) of the fraction unshadowed toward the light by the cluster.
    lit: (f64, f64),
}

const MODELS: [&str; 9] = ["kept", "sggx", "z4", "z4-nnls", "z8", "z8-nnls", "card-normal", "z16-nnls", "new"];

fn measure(leaves: &[Arc<Tagged<Triangle>>], size: f64, views: &[Vector3<f64>], lights: &[Vector3<f64>],
           dirs: &[Vector3<f64>], sample_every: usize, rng: &mut Pcg32) -> Tally {
    let mut t = Tally::default();
    for (gi, group) in groups(leaves, size).into_iter().enumerate() {
        t.clusters += 1;
        if group.len() <= 2 {
            continue;
        }
        let members: Vec<&Tagged<Triangle>> = group.iter().map(|&i| &*leaves[i]).collect();
        let Some((corner, u, v)) = card(&members) else { continue };
        let the_card = Card { corner: single(&corner), u: single(&u), v: single(&v), data: 0 };
        t.cards += 1;
        t.members += members.len();
        t.kept += members.len().min(OLD_KEPT);
        t.over += (members.len() > OLD_KEPT) as usize;
        let mut keys: Vec<(u32, [i16; 2])> = members.iter().map(|m| (m.face, encode_normal(&m.item.normal))).collect();
        keys.sort_unstable();
        keys.dedup();
        t.merged += keys.len().min(OLD_KEPT);
        if gi % sample_every != 0 {
            continue;
        }
        t.sampled += 1;
        let set: Set = members.iter().map(|m| (m.item.normal, area(&m.item))).collect();
        let kept: Set = if set.len() <= OLD_KEPT { set.clone() } else {
            let step = set.len() as f64 / OLD_KEPT as f64;
            (0 .. OLD_KEPT).map(|k| set[(k as f64 * step) as usize]).collect()
        };
        let card_normal = u.cross(&v).normalize();
        let s = sggx(&set);
        let z4 = reduced(&set, 4, false, dirs);
        let z4n = reduced(&set, 4, true, dirs);
        let z8 = reduced(&set, 8, false, dirs);
        let z8n = reduced(&set, 8, true, dirs);
        let z16n = reduced(&set, 16, true, dirs);
        let all: Vec<(Vector3<f64>, f64, u32)> = members.iter().map(|m| (m.item.normal, area(&m.item), m.face)).collect();
        let new: Set = constituents(&all, gi as u64).into_iter().map(|k| (k.0, k.1)).collect();
        t.new_kept += new.len();
        let flat: Set = vec![(card_normal, 1.)];
        for (k, m) in [&z4, &z4n, &z8, &z8n].iter().enumerate() {
            t.sigma_err[k] += sigma_error(&set, m, dirs);
        }
        let w = sigma(&set, &Vector3::y()).max(1e-12);
        for d in views {
            for l in lights {
                let truth = chosen(&set, d, l);
                let r = [
                    chosen(&kept, d, l),
                    sggx_chosen(&s, d, l, 24),
                    chosen(&z4, d, l),
                    chosen(&z4n, d, l),
                    chosen(&z8, d, l),
                    chosen(&z8n, d, l),
                    chosen(&flat, d, l),
                    chosen(&z16n, d, l),
                    chosen(&new, d, l),
                ];
                for (acc, x) in t.model.iter_mut().zip(r) {
                    acc.0 += w * x;
                    acc.1 += w * (x - truth).abs();
                }
                t.truth += w * truth;
                t.weight += w;
            }
        }

        // Ray casting against the cluster's own leaves.
        let tris: Vec<Arc<Tagged<Triangle>>> = group.iter().map(|&i| leaves[i].clone()).collect();
        let geometry = GeometrySet::new(&tris);
        let b = geometry.bounds();
        let (centre, radius) = ((b.min.coords + b.max.coords) * 0.5, (b.max - b.min).norm() * 0.5);
        let hit_cards = |ray: &Ray| the_card.hit(ray).is_some();
        let rays = 96;
        for d in views {
            let (e1, e2) = frame(d);
            let (mut leaf_hits, mut card_hits) = (0., 0.);
            for _ in 0 .. rays {
                let (r1, r2) = (rng.next_f64(), rng.next_f64());
                let (x, y) = (radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).cos(), radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).sin());
                let ray = Ray { ro: centre + e1 * x + e2 * y - d * (3. * radius), rd: *d };
                if hit_cards(&ray) {
                    card_hits += 1.;
                }
                if let Some(h) = geometry.intersects(&ray) {
                    leaf_hits += 1.;
                    let n = members.iter().find(|m| m.face == h.face).map_or(h.normal, |m| m.item.normal);
                    for l in lights {
                        let base = response(&n, d, l);
                        let shadow = Ray { ro: h.point + l * (1e-6 * radius), rd: *l };
                        let lit = !geometry.occludes(&shadow, 1e-6 * radius, f64::INFINITY);
                        t.cast.1 += base;
                        t.cast.2 += if lit { base } else { 0. };
                        t.cast.0 += chosen(&kept, d, l);
                    }
                }
            }
            t.hits.0 += leaf_hits / rays as f64;
            t.hits.1 += card_hits / rays as f64;
        }
        // From the side, where an upward card is seen edge on.
        for k in 0 .. 8 {
            let phi = std::f64::consts::PI * k as f64 / 4.;
            let d = Vector3::new(phi.cos(), -0.15 + 0.3 * ((k % 3) as f64 / 2.), phi.sin()).normalize();
            let (e1, e2) = frame(&d);
            let (mut leaf_hits, mut card_hits) = (0., 0.);
            for _ in 0 .. rays {
                let (r1, r2) = (rng.next_f64(), rng.next_f64());
                let (x, y) = (radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).cos(), radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).sin());
                let ray = Ray { ro: centre + e1 * x + e2 * y - d * (3. * radius), rd: d };
                card_hits += hit_cards(&ray) as u8 as f64;
                if let Some(h) = geometry.intersects(&ray) {
                    leaf_hits += 1.;
                    let n = members.iter().find(|m| m.face == h.face).map_or(h.normal, |m| m.item.normal);
                    for l in lights {
                        t.side_cast.0 += chosen(&kept, &d, l);
                        t.side_cast.1 += response(&n, &d, l);
                    }
                }
            }
            t.side_hits.0 += leaf_hits;
            t.side_hits.1 += card_hits;
        }
        // Unshadowed fraction from random points on the leaves, toward each light.
        for l in lights {
            for _ in 0 .. 32 {
                let m = members[(rng.next_f64() * members.len() as f64) as usize];
                let (mut a, mut c) = (rng.next_f64(), rng.next_f64());
                if a + c > 1. {
                    a = 1. - a;
                    c = 1. - c;
                }
                let p = m.item.v0 + (m.item.v1 - m.item.v0) * a + (m.item.v2 - m.item.v0) * c;
                let shadow = Ray { ro: p + l * (1e-6 * radius), rd: *l };
                t.lit.0 += area(&m.item) * if geometry.occludes(&shadow, 1e-6 * radius, f64::INFINITY) { 0. } else { 1. };
                t.lit.1 += area(&m.item);
            }
        }
    }
    t
}

#[test]
fn test_sggx_sampling_of_an_isotropic_lobe_is_cosine() {
    // For S = I every normal is equally likely, so visible normals are cosine distributed
    // about the view: E[m·wi] = 2/3.
    let wi = Vector3::new(0.3, 0.8, -0.2).normalize();
    let side = 64;
    let mut sum = 0.;
    for a in 0 .. side {
        for b in 0 .. side {
            sum += sggx_sample(&Matrix3::identity(), &wi, (a as f64 + 0.5) / side as f64, (b as f64 + 0.5) / side as f64).dot(&wi);
        }
    }
    assert!((sum / (side * side) as f64 - 2. / 3.).abs() < 1e-3);
    // A flat lobe gives its own normal.
    let n = Vector3::new(0.1, 1., 0.2).normalize();
    let flat = n * n.transpose() + Matrix3::identity() * 1e-6;
    assert!(sggx_sample(&flat, &wi, 0.3, 0.7).dot(&n).abs() > 0.99);
}

#[test]
fn test_refitted_zonotope_matches_projected_area() {
    let mut rng = Pcg32::new(4, 4);
    let set: Set = (0 .. 40).map(|_| (Vector3::new(rng.next_f64() - 0.5, rng.next_f64(), rng.next_f64() - 0.5).normalize(), 0.5 + rng.next_f64())).collect();
    let dirs = sphere(128);
    let merged = reduced(&set, 6, false, &dirs);
    let fitted = reduced(&set, 6, true, &dirs);
    assert!(sigma_error(&set, &fitted, &dirs) <= sigma_error(&set, &merged, &dirs) + 1e-9);
    assert_eq!(reduced(&set, 64, true, &dirs).len(), set.len());
}

#[test]
#[ignore]
fn lod_research() {
    let species = ["quaking-aspen", "california-black-oak", "weeping-willow", "scots-pine", "norway-spruce", "hazel"];
    let levels = [2, 3, 4, 5, 6, 7];
    let dirs = sphere(128);
    // Views from above (the forest seen from the air); lights from the upper hemisphere.
    let views: Vec<Vector3<f64>> = sphere(24).into_iter().filter(|d| d.y < -0.1).collect();
    let lights: Vec<Vector3<f64>> = sphere(16).into_iter().filter(|l| l.y > 0.1).collect();
    let out = std::env::var("LOD_RESEARCH_OUT").unwrap_or_else(|_| "target/lod-research.csv".into());
    let mut csv = String::from("species,level,size,leaves,clusters,cards,members_per_card,kept_per_card,over_32,sampled,truth");
    for m in MODELS {
        csv += &format!(",{m}_bias,{m}_abs");
    }
    csv += ",z4_sigma,z4nnls_sigma,z8_sigma,z8nnls_sigma,leaf_hit,card_hit,cast_choice,cast_visible,cast_shadowed,lit_fraction\n";
    for name in species {
        let p = tree::species(name).unwrap();
        let mesh = tree::generate(&p, 1, Vector3::zeros());
        let base = piece_size(&mesh.leaves);
        for &level in &levels {
            let size = level_size(base, level);
            let mut rng = Pcg32::new(level as u64, 7);
            let clusters = groups(&mesh.leaves, size).len();
            let every = (clusters / 150).max(1);
            let t = measure(&mesh.leaves, size, &views, &lights, &dirs, every, &mut rng);
            let s = t.sampled.max(1) as f64;
            let truth = t.truth / t.weight;
            let mut row = format!("{name},{level},{size:.4},{},{},{},{:.2},{:.2},{:.3},{},{:.4}",
                mesh.leaves.len(), t.clusters, t.cards, t.members as f64 / t.cards as f64, t.kept as f64 / t.cards as f64,
                t.over as f64 / t.cards as f64, t.sampled, truth);
            print!("{name:22} L{level} size {size:.3} m: {} clusters, {} cards, {:.1} leaves/card ({:.1} kept, {:.0}% over 32)",
                t.clusters, t.cards, t.members as f64 / t.cards as f64, t.kept as f64 / t.cards as f64, 100. * t.over as f64 / t.cards as f64);
            println!("    constituents as now kept: {:.1} per sampled card", t.new_kept as f64 / t.sampled.max(1) as f64);
            println!("    merging a leaf's triangles of one normal: {:.1} constituents per card ({} triangles a leaf)", t.merged as f64 / t.cards as f64, mesh.leaf_triangles);
            for (k, m) in MODELS.iter().enumerate() {
                let bias = t.model[k].0 / t.weight / truth - 1.;
                let abs = t.model[k].1 / t.weight / truth;
                row += &format!(",{bias:.4},{abs:.4}");
                println!("    {m:12} bias {:+6.2}%  mean |error| {:5.2}%", 100. * bias, 100. * abs);
            }
            for e in t.sigma_err {
                row += &format!(",{:.4}", e / s);
            }
            println!("    projected area, max rel. error: z4 {:.1}%  z4-nnls {:.1}%  z8 {:.1}%  z8-nnls {:.1}%",
                100. * t.sigma_err[0] / s, 100. * t.sigma_err[1] / s, 100. * t.sigma_err[2] / s, 100. * t.sigma_err[3] / s);
            let nv = (t.sampled * views.len()) as f64;
            println!("    hit probability: leaves {:.3}, card {:.3} (card/leaves {:.3}); from the side card/leaves {:.3}", t.hits.0 / nv, t.hits.1 / nv, t.hits.1 / t.hits.0, t.side_hits.1 / t.side_hits.0);
            println!("    from the side, response of the leaf hit against the a|n·d| choice: {:+.2}%", 100. * (t.side_cast.1 / t.side_cast.0 - 1.));
            println!("    response of the leaf hit, against the a|n·d| choice: unshadowed {:+.2}%, shadowed by its cluster {:+.2}%; lit fraction from leaves {:.3}",
                100. * (t.cast.1 / t.cast.0 - 1.), 100. * (t.cast.2 / t.cast.0 - 1.), t.lit.0 / t.lit.1);
            row += &format!(",{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}\n", t.hits.0 / nv, t.hits.1 / nv, t.cast.0, t.cast.1, t.cast.2, t.lit.0 / t.lit.1);
            csv += &row;
        }
    }
    std::fs::write(&out, csv).unwrap();
    println!("written to {out}");
}

/// The distance along `r` (direction of unit length) at which it enters and leaves the box `b`,
/// if it meets it.
fn slab(b: &BBox, r: &Ray) -> Option<(f64, f64)> {
    let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
    for k in 0 .. 3 {
        let (lo, hi) = ((b.min[k] - r.ro[k]) / r.rd[k], (b.max[k] - r.ro[k]) / r.rd[k]);
        near = near.max(lo.min(hi));
        far = far.min(lo.max(hi));
    }
    (near <= far && far > 0.).then_some((near.max(0.), far))
}

/// The Kaplan-Meier estimate of the chance of a free path longer than each of `at`, from free
/// paths `(length, ended)`: those that left the crown first are censored, not ended.
fn survival(mut paths: Vec<(f64, bool)>, at: &[f64]) -> Vec<f64> {
    paths.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut at_risk = paths.len() as f64;
    let (mut s, mut k) = (1., 0);
    let mut out = vec![];
    for &t in at {
        while k < paths.len() && paths[k].0 <= t {
            if paths[k].1 {
                s *= 1. - 1. / at_risk;
            }
            at_risk -= 1.;
            k += 1;
        }
        out.push(s);
    }
    out
}

/// Transmittance in a tree's crown, for docs/research/learned-lod (stage 5): the chance of a
/// free path longer than `s` for rays entering the crown's box from outside (`T_u`) and for
/// rays leaving a point on a leaf (`T_c`), each by Kaplan-Meier over a few thousand rays. The
/// gamma law of Jarabo et al. 2018, `T_u = (1 + s/λ)^-α`, is fitted to `T_u`, and with it
/// `T_c = T_u'(s)/T_u'(0) = (1 + s/λ)^-(α+1)` predicted (d'Eon 2018; Bitterli et al. 2018);
/// an exponential fit predicts `T_c = T_u`. Run by hand:
///
///     cargo test --release --lib lod_transmittance -- --ignored --nocapture
#[test]
#[ignore]
fn lod_transmittance() {
    let species = ["quaking-aspen", "california-black-oak", "weeping-willow", "scots-pine", "norway-spruce", "hazel"];
    let mut csv = String::from("species,lambda,alpha,mu,rms_gamma_u,rms_exp_u,rms_gamma_c,rms_exp_c,rms_gamma_c_refit\n");
    for name in species {
        let mesh = tree::generate(&tree::species(name).unwrap(), 1, Vector3::zeros());
        let leaves = GeometrySet::new(&mesh.leaves);
        let b = leaves.bounds();
        let size = (b.max - b.min).norm();
        let mut rng = Pcg32::new(17, 1);
        let unit = |rng: &mut Pcg32| {
            let z = 2. * rng.next_f64() - 1.;
            let phi = 2. * std::f64::consts::PI * rng.next_f64();
            let r = (1. - z * z).sqrt();
            Vector3::new(r * phi.cos(), z, r * phi.sin())
        };
        let rays = 20000;
        // Entering: aimed at a point uniform in the box, from far outside.
        let mut entering = vec![];
        for _ in 0 .. rays {
            let d = unit(&mut rng);
            let q = b.min.coords + (b.max - b.min).component_mul(&Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()));
            let r = Ray { ro: q - d * (2. * size), rd: d };
            let Some((t0, t1)) = slab(&b, &r) else { continue };
            match leaves.intersects(&r) {
                Some(h) if h.dist < t1 => entering.push((h.dist - t0, true)),
                _ => entering.push((t1 - t0, false)),
            }
        }
        // From points uncorrelated with the leaves, uniform in the box, every way: the
        // uncorrelated-origin transmittance of the theory (for a medium as uniform as the box).
        let mut within = vec![];
        for _ in 0 .. rays {
            let d = unit(&mut rng);
            let p = b.min.coords + (b.max - b.min).component_mul(&Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()));
            let r = Ray { ro: p, rd: d };
            let exit = slab(&b, &r).map_or(0., |(_, t1)| t1);
            match leaves.intersects(&r) {
                Some(h) if h.dist < exit => within.push((h.dist, true)),
                _ => within.push((exit, false)),
            }
        }
        // Leaving: from a point on a leaf, drawn by area, every way.
        let areas: Vec<f64> = mesh.leaves.iter().map(|t| area(&t.item)).collect();
        let total: f64 = areas.iter().sum();
        let mut leaving = vec![];
        let mut others = vec![];
        for _ in 0 .. rays {
            let mut left = rng.next_f64() * total;
            let k = areas.iter().position(|a| { left -= a; left < 0. }).unwrap_or(areas.len() - 1);
            let t = &mesh.leaves[k].item;
            let (mut u, mut v) = (rng.next_f64(), rng.next_f64());
            if u + v > 1. {
                u = 1. - u;
                v = 1. - v;
            }
            let p = t.v0 + (t.v1 - t.v0) * u + (t.v2 - t.v0) * v;
            let d = unit(&mut rng);
            let r = Ray { ro: p + d * (1e-6 * size), rd: d };
            let exit = slab(&b, &r).map_or(0., |(_, t1)| t1);
            match leaves.intersects(&r) {
                Some(h) if h.dist < exit => leaving.push((h.dist, true)),
                _ => leaving.push((exit, false)),
            }
            // The same, passing through the ray's own leaf (its fold and other triangles).
            let own = mesh.leaves[k].face;
            let mut from = 0.;
            let other = loop {
                match leaves.intersects_beyond(&r, from) {
                    Some(h) if h.dist < exit && h.face == own => from = h.dist + 1e-9 * size,
                    Some(h) if h.dist < exit => break (h.dist, true),
                    _ => break (exit, false),
                }
            };
            others.push(other);
        }
        let grid: Vec<f64> = (1 ..= 60).map(|i| size * 0.5 * i as f64 / 60.).collect();
        let su = survival(entering, &grid);
        let sc = survival(leaving, &grid);
        let sw = survival(within, &grid);
        let so = survival(others, &grid);
        // Fit (1 + s/λ)^-α to T_u: for each λ the best α is linear in log T.
        let used: Vec<usize> = (0 .. grid.len()).filter(|&i| su[i] > 0.02).collect();
        let fit = |s: &[f64], plus: f64| -> (f64, f64, f64) {
            let mut best = (f64::INFINITY, 0., 0.);
            for j in 0 .. 300 {
                let lambda = size * 1e-4 * 10f64.powf(j as f64 / 60.);
                let (mut num, mut den) = (0., 0.);
                for &i in &used {
                    let x = (1. + grid[i] / lambda).ln();
                    num += -s[i].max(1e-9).ln() * x;
                    den += x * x;
                }
                let alpha = (num / den - plus).max(1e-6);
                let err: f64 = used.iter().map(|&i| ((1. + grid[i] / lambda).powf(-(alpha + plus)) - s[i]).powi(2)).sum::<f64>();
                if err < best.0 {
                    best = (err, lambda, alpha);
                }
            }
            best
        };
        let (_, lambda, alpha) = fit(&su, 0.);
        let mu = {
            let (mut num, mut den) = (0., 0.);
            for &i in &used {
                num += -su[i].max(1e-9).ln() * grid[i];
                den += grid[i] * grid[i];
            }
            num / den
        };
        let rms = |pred: &dyn Fn(f64) -> f64, s: &[f64]| (used.iter().map(|&i| (pred(grid[i]) - s[i]).powi(2)).sum::<f64>() / used.len() as f64).sqrt();
        let gamma_u = |x: f64| (1. + x / lambda).powf(-alpha);
        let gamma_c = |x: f64| (1. + x / lambda).powf(-(alpha + 1.));
        let exp = |x: f64| (-mu * x).exp();
        // For comparison, the gamma law fitted to T_c directly (how well the form can do).
        let (_, lambda_c, alpha_c) = fit(&sc, 0.);
        let refit = |x: f64| (1. + x / lambda_c).powf(-alpha_c);
        let r = [rms(&gamma_u, &su), rms(&exp, &su), rms(&gamma_c, &sc), rms(&exp, &sc), rms(&refit, &sc)];
        // The same prediction from the uncorrelated origins within the box.
        let (_, lambda_w, alpha_w) = fit(&sw, 0.);
        let gamma_w = |x: f64| (1. + x / lambda_w).powf(-alpha_w);
        let gamma_wc = |x: f64| (1. + x / lambda_w).powf(-(alpha_w + 1.));
        println!("    from within the box: λ {:.3} m, α {:.2}, fit rms {:.4}; T_c predicted from it rms {:.4}",
            lambda_w, alpha_w, rms(&gamma_w, &sw), rms(&gamma_wc, &sc));
        println!("    T_c fitted directly: λ {:.3} m, α {:.3}", lambda_c, alpha_c);
        let (_, lambda_o, alpha_o) = fit(&so, 0.);
        let near = |x: f64| grid.iter().position(|&g| g >= x).unwrap_or(0);
        println!("    T_c past its own leaf: λ {:.3} m, α {:.3}; at {:.2} m {:.3} (with own leaf {:.3}), {:.2} m {:.3} ({:.3})",
            lambda_o, alpha_o, grid[0], so[0], sc[0], grid[near(2.)], so[near(2.)], sc[near(2.)]);
        println!("{name:22} crown {size:.1} m: λ {:.3} m, α {alpha:.2}, μ {mu:.3}/m", lambda);
        println!("    T_u fit rms: gamma {:.4}, exponential {:.4}", r[0], r[1]);
        println!("    T_c predicted rms: gamma (α+1) {:.4}, exponential {:.4}; gamma fitted to T_c itself {:.4}", r[2], r[3], r[4]);
        for i in [2, 5, 11, 23, 47] {
            println!("    s {:5.2} m: T_u {:.3} (gamma {:.3}), within {:.3}, T_c {:.3} (predicted {:.3}, from within {:.3}, exponential {:.3})",
                grid[i], su[i], gamma_u(grid[i]), sw[i], sc[i], gamma_c(grid[i]), gamma_wc(grid[i]), exp(grid[i]));
        }
        csv += &format!("{name},{lambda:.4},{alpha:.4},{mu:.4},{:.4},{:.4},{:.4},{:.4},{:.4}\n", r[0], r[1], r[2], r[3], r[4]);
    }
    let out = std::env::var("LOD_TRANSMITTANCE_OUT").unwrap_or_else(|_| "target/lod-transmittance.csv".into());
    std::fs::write(&out, csv).unwrap();
    println!("written to {out}");
}

/// Whole trees at each level, against their leaves (docs/research/learned-lod/coarse-bias.md):
/// for parallel rays from views like `forest-aerial`'s, the fraction that hit the tree; of
/// those, the fraction lit by the sun through the rest of the tree; and the mean response of
/// what is hit. Unlike `lod_research`, this counts what clusters do to each other.
#[test]
#[ignore]
fn lod_tree_levels() {
    let species = ["quaking-aspen", "california-black-oak", "weeping-willow", "scots-pine", "norway-spruce", "hazel"];
    let sun = Vector3::new(-0.5, 0.75, 0.6).normalize();
    // Looking down 10°–40°, all round.
    let views: Vec<Vector3<f64>> = (0 .. 12).map(|k| {
        let phi = std::f64::consts::PI * k as f64 / 6.;
        let down = (10. + 10. * (k % 4) as f64).to_radians();
        Vector3::new(phi.cos() * down.cos(), -down.sin(), phi.sin() * down.cos())
    }).collect();
    println!("species level cards hit lit response hit*lit*resp (each / leaves)");
    for name in species {
        let p = tree::species(name).unwrap();
        let mesh = tree::generate(&p, 1, Vector3::zeros());
        let base = piece_size(&mesh.leaves);
        let leaves = consolidate(&mesh.leaves, 0.);
        let b = leaves.bounds();
        let (centre, radius) = ((b.min.coords + b.max.coords) * 0.5, (b.max - b.min).norm() * 0.5);
        let eps = 1e-6 * radius;
        let measure = |g: &CardSet| {
            let mut rng = Pcg32::new(3, 9);
            let (mut rays, mut hits, mut lit, mut resp) = (0., 0., 0., 0.);
            for d in &views {
                let (e1, e2) = frame(d);
                for _ in 0 .. 4000 {
                    let (r1, r2) = (rng.next_f64(), rng.next_f64());
                    let (x, y) = (radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).cos(), radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).sin());
                    let ray = Ray { ro: centre + e1 * x + e2 * y - d * (3. * radius), rd: *d };
                    rays += 1.;
                    if let Some(h) = g.intersects(&ray) {
                        hits += 1.;
                        let shadow = Ray { ro: h.point + sun * eps, rd: sun };
                        let l = !g.occludes(&shadow, eps, f64::INFINITY);
                        let r = response(&h.normal, d, &sun);
                        lit += l as u8 as f64;
                        resp += r;
                    }
                }
            }
            (hits / rays, lit / hits, resp / hits)
        };
        let truth = measure(&leaves);
        println!("{name:22} leaves {:>8}  hit {:.3} lit {:.3} resp {:.3}", leaves.cards().len(), truth.0, truth.1, truth.2);
        for level in 2 ..= 9 {
            let g = consolidate(&mesh.leaves, level_size(base, level));
            let m = measure(&g);
            println!("{name:22} L{level} {:>8}  hit {:.3} lit {:.3} resp {:.3} all {:.3}", g.cards().len(),
                m.0 / truth.0, m.1 / truth.1, m.2 / truth.2, m.0 * m.1 * m.2 / (truth.0 * truth.1 * truth.2));
        }
    }
}

/// As `lod_tree_levels`, but the light each first hit sends back to the viewer, path traced
/// with the summer leaf's reflectance and transmittance, sunlight only, so light scattered
/// between leaves counts: whole trees at each level against their leaves (`LOD_SELF_LIT_FROM`
/// sets which cards shadow themselves).
#[test]
#[ignore]
fn lod_tree_scattering() {
    let species: Vec<String> = std::env::var("LOD_SPECIES").map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_else(|_| ["quaking-aspen", "california-black-oak", "scots-pine", "norway-spruce"].map(String::from).to_vec());
    let sun = Vector3::new(-0.5, 0.75, 0.6).normalize();
    let views: Vec<Vector3<f64>> = (0 .. 12).map(|k| {
        let phi = std::f64::consts::PI * k as f64 / 6.;
        let down = (10. + 10. * (k % 4) as f64).to_radians();
        Vector3::new(phi.cos() * down.cos(), -down.sin(), phi.sin() * down.cos())
    }).collect();
    for name in &species {
        let p = tree::species(name).unwrap();
        let mesh = tree::generate(&p, 1, Vector3::zeros());
        let base = piece_size(&mesh.leaves);
        let leaves = consolidate(&mesh.leaves, 0.);
        let b = leaves.bounds();
        let (centre, radius) = ((b.min.coords + b.max.coords) * 0.5, (b.max - b.min).norm() * 0.5);
        let eps = 1e-6 * radius;
        // Mean radiance back along each primary ray (0 for a miss), and the share from bounces.
        let render = |g: &CardSet| {
            let mut rng = Pcg32::new(5, 11);
            let (mut sum, mut bounced, mut n) = (0., 0., 0.);
            for d in &views {
                let (e1, e2) = frame(d);
                for _ in 0 .. 3000 {
                    let (r1, r2) = (rng.next_f64(), rng.next_f64());
                    let (x, y) = (radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).cos(), radius * r1.sqrt() * (2. * std::f64::consts::PI * r2).sin());
                    let mut ray = Ray { ro: centre + e1 * x + e2 * y - d * (3. * radius), rd: *d };
                    n += 1.;
                    let mut weight = 1.;
                    for bounce in 0 .. 8 {
                        let Some(h) = g.intersects(&ray) else { break };
                        // The leaf's side facing where the ray came from.
                        let nf = if h.normal.dot(&ray.rd) < 0. { h.normal } else { -h.normal };
                        let cos = nf.dot(&sun);
                        let f = if cos > 0. { RHO } else { TAU } / std::f64::consts::PI;
                        let shadow = Ray { ro: h.point + sun * eps, rd: sun };
                        if !g.occludes(&shadow, eps, f64::INFINITY) {
                            let c = weight * f * cos.abs();
                            sum += c;
                            if bounce > 0 { bounced += c; }
                        }
                        // Continue: reflect with probability ρ/(ρ+τ), else transmit; weight ρ+τ.
                        let side = if rng.next_f64() < RHO / (RHO + TAU) { nf } else { -nf };
                        let (u1, u2) = (rng.next_f64(), rng.next_f64());
                        let (t1, t2) = frame(&side);
                        let r = u1.sqrt();
                        let dir = (t1 * (r * (2. * std::f64::consts::PI * u2).cos()) + t2 * (r * (2. * std::f64::consts::PI * u2).sin()) + side * (1. - u1).sqrt()).normalize();
                        weight *= RHO + TAU;
                        ray = Ray { ro: h.point + dir * eps, rd: dir };
                    }
                }
            }
            (sum / n, bounced / sum.max(1e-12))
        };
        let truth = render(&leaves);
        println!("{name:22} leaves: radiance {:.5} ({:.0}% from bounces)", truth.0, 100. * truth.1);
        for level in [3, 4, 5, 6, 7] {
            let mut line = format!("{name:22} L{level}:");
            let g = consolidate(&mesh.leaves, level_size(base, level));
            let m = render(&g);
            line += &format!("  {:.3} ({:.0}% bounced)", m.0 / truth.0, 100. * m.1);
            println!("{line}");
        }
    }
}
