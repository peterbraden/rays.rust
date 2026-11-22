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
                leaf_hits += geometry.intersects(&ray).is_some() as u8 as f64;
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
            println!("    response of the leaf hit, against the a|n·d| choice: unshadowed {:+.2}%, shadowed by its cluster {:+.2}%; lit fraction from leaves {:.3}",
                100. * (t.cast.1 / t.cast.0 - 1.), 100. * (t.cast.2 / t.cast.0 - 1.), t.lit.0 / t.lit.1);
            row += &format!(",{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}\n", t.hits.0 / nv, t.hits.1 / nv, t.cast.0, t.cast.1, t.cast.2, t.lit.0 / t.lit.1);
            csv += &row;
        }
    }
    std::fs::write(&out, csv).unwrap();
    println!("written to {out}");
}
