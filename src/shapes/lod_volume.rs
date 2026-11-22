//! A research prototype for docs/research/learned-lod (stage 6): the coarsest clusters of an
//! aggregate as volumes whose free paths follow transmittances measured on their own leaves,
//! in place of cards. Off unless `LOD_VOLUME_TRIANGLES` is set: clusters of at least that many
//! triangles become volumes.
//!
//! Stage 5 found that foliage's transmittance for paths leaving a leaf (`T_c`) can't be had
//! from its transmittance for paths entering from outside (`T_u`), as the reciprocal relation
//! of non-exponential transport would have it, so both are measured, per cluster, as tables:
//! the chance of a free path longer than each of 16 distances, by Kaplan-Meier over rays cast
//! against the cluster's leaves at build time. A ray from outside the cluster's box has its
//! free path from the box drawn from the first; a ray from within it, from the second. Each
//! draw inverts the table with a hash of the ray, so a ray always meets the same volume, as a
//! card's constituent is chosen. A hit is shaded as one of the cluster's constituents.
use crate::na::{Matrix3, Vector3};
use crate::ray::Ray;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::{Geometry, Tagged};
use crate::shapes::geometry_set::GeometrySet;
use crate::shapes::triangle::Triangle;
use std::sync::Arc;

/// The distances a table holds survival at: `KNOTS` of them over the box's diagonal, closer near
/// the start, where foliage's `T_c` falls fastest.
const KNOTS: usize = 16;

/// How many triangles make a cluster a volume, if prototyping (`LOD_VOLUME_TRIANGLES`).
pub fn threshold() -> Option<usize> {
    static T: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var("LOD_VOLUME_TRIANGLES").ok().and_then(|s| s.parse().ok()))
}

/// A cluster as a volume: its box (centred at `centre`, `u` and `v` across, `thick` deep along
/// their normal), the survival of free paths entering it and leaving one of its leaves at
/// each knot (255ths), and its cluster's number.
#[derive(Clone, Copy, Debug)]
pub struct Volume {
    centre: [f32; 3],
    u: [f32; 3],
    v: [f32; 3],
    thick: f32,
    enter: [u8; KNOTS],
    leave: [u8; KNOTS],
    pub cluster: u32,
}

fn vector(a: &[f32; 3]) -> Vector3<f64> {
    Vector3::new(a[0] as f64, a[1] as f64, a[2] as f64)
}

fn single(v: &Vector3<f64>) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}

impl Volume {
    fn frame(&self) -> (Vector3<f64>, [Vector3<f64>; 3], [f64; 3]) {
        let (u, v) = (vector(&self.u), vector(&self.v));
        let n = u.cross(&v).normalize();
        (vector(&self.centre), [u.normalize(), v.normalize(), n], [u.norm() * 0.5, v.norm() * 0.5, self.thick as f64 * 0.5])
    }

    fn diagonal(&self) -> f64 {
        let (_, _, h) = self.frame();
        2. * (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt()
    }

    /// The multiples of `r`'s direction at which it enters and leaves the box, if it meets it.
    fn slab(&self, r: &Ray) -> Option<(f64, f64)> {
        let (c, axes, half) = self.frame();
        let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
        for k in 0 .. 3 {
            let (o, d) = ((r.ro - c).dot(&axes[k]), r.rd.dot(&axes[k]));
            if d.abs() < 1e-300 {
                if o.abs() > half[k] {
                    return None;
                }
                continue;
            }
            let (a, b) = ((-half[k] - o) / d, (half[k] - o) / d);
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
        (near <= far && far > 0.).then_some((near, far))
    }

    /// The distance at which a free path ends, given `xi` uniform in [0, 1), by `table`: none
    /// if it outlasts the table.
    fn draw(&self, table: &[u8; KNOTS], xi: f64) -> Option<f64> {
        let at = |k: usize| self.diagonal() * (k as f64 / (KNOTS - 1) as f64).powi(2);
        // Survival is 1 at 0 and falls through the knots; find where it passes xi.
        let (mut s0, mut d0) = (1., 0.);
        for (k, &t) in table.iter().enumerate() {
            let (s1, d1) = (t as f64 / 255., at(k));
            if s1 <= xi {
                return Some(if s0 > s1 { d0 + (d1 - d0) * (s0 - xi) / (s0 - s1) } else { d1 });
            }
            (s0, d0) = (s1, d1);
        }
        None
    }

    /// Where `r` is stopped in it, as a multiple of its direction, given `xi`: drawn from the
    /// leaving table if `r` starts within the box, the entering one from where it enters if not.
    pub fn hit(&self, r: &Ray, xi: f64) -> Option<f64> {
        let (t0, t1) = self.slab(r)?;
        let speed = r.rd.norm();
        let (from, table) = if t0 <= 0. { (0., &self.leave) } else { (t0, &self.enter) };
        let s = self.draw(table, xi)?;
        let t = from + s / speed;
        (t < t1 && t > 0.).then_some(t)
    }

    pub fn bounds(&self) -> BBox {
        let (c, axes, half) = self.frame();
        let mut b = BBox::new(Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY));
        for i in 0 .. 8 {
            let p = c + axes[0] * half[0] * if i & 1 == 0 { -1. } else { 1. }
                + axes[1] * half[1] * if i & 2 == 0 { -1. } else { 1. }
                + axes[2] * half[2] * if i & 4 == 0 { -1. } else { 1. };
            b = b.union_point(&p);
        }
        b
    }
}

fn area(t: &Triangle) -> f64 {
    0.5 * (t.v1 - t.v0).cross(&(t.v2 - t.v0)).norm()
}

/// The Kaplan-Meier estimate of survival at `at` from free paths `(length, ended)`.
fn survival(mut paths: Vec<(f64, bool)>, at: &[f64]) -> Vec<f64> {
    paths.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut at_risk = paths.len() as f64;
    let (mut s, mut k) = (1., 0);
    at.iter().map(|&t| {
        while k < paths.len() && paths[k].0 <= t {
            if paths[k].1 {
                s *= 1. - 1. / at_risk;
            }
            at_risk -= 1.;
            k += 1;
        }
        s
    }).collect()
}

/// The volume for the cluster `members`, of cluster number `cluster`: its box from the plane its
/// leaves cover most of and their spread, its tables measured by `rays` rays of each kind.
pub fn volume(members: &[&Tagged<Triangle>], cluster: u32, rays: usize) -> Option<Volume> {
    let mut moment = Matrix3::zeros();
    let (mut centre, mut total) = (Vector3::zeros(), 0.);
    for m in members {
        let a = area(&m.item);
        moment += m.item.normal * m.item.normal.transpose() * a;
        centre += (m.item.v0 + m.item.v1 + m.item.v2) / 3. * a;
        total += a;
    }
    if total <= 0. {
        return None;
    }
    centre /= total;
    let eigen = moment.symmetric_eigen();
    let n: Vector3<f64> = eigen.eigenvectors.column(eigen.eigenvalues.imax()).into_owned();
    let any = if n.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let e1 = n.cross(&any).normalize();
    let e2 = n.cross(&e1);
    let (mut lo, mut hi) = (Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY));
    for m in members {
        for p in [m.item.v0, m.item.v1, m.item.v2] {
            let q = Vector3::new((p - centre).dot(&e1), (p - centre).dot(&e2), (p - centre).dot(&n));
            lo = lo.inf(&q);
            hi = hi.sup(&q);
        }
    }
    let size = hi - lo;
    if size.x <= 0. || size.y <= 0. {
        return None;
    }
    let mid = centre + e1 * (0.5 * (lo.x + hi.x)) + e2 * (0.5 * (lo.y + hi.y)) + n * (0.5 * (lo.z + hi.z));
    let mut volume = Volume {
        centre: single(&mid), u: single(&(e1 * size.x)), v: single(&(e2 * size.y)), thick: size.z.max(1e-6) as f32,
        enter: [0; KNOTS], leave: [0; KNOTS], cluster,
    };
    let owned: Vec<Arc<Tagged<Triangle>>> = members.iter().map(|m| Arc::new(Tagged { item: m.item.clone(), face: m.face })).collect();
    let leaves = GeometrySet::new(&owned);
    let diagonal = volume.diagonal();
    let knots: Vec<f64> = (0 .. KNOTS).map(|k| diagonal * (k as f64 / (KNOTS - 1) as f64).powi(2)).collect();
    let mut rng = crate::geometry::Pcg32::new(cluster as u64, 0x7015);
    let unit = |rng: &mut crate::geometry::Pcg32| {
        let z = 2. * rng.next_f64() - 1.;
        let phi = 2. * std::f64::consts::PI * rng.next_f64();
        let r = (1. - z * z).sqrt();
        Vector3::new(r * phi.cos(), z, r * phi.sin())
    };
    let (c, axes, half) = volume.frame();
    // Entering: aimed at points uniform in the box, from outside, every way.
    let mut entering = vec![];
    // The points rays from outside meet first: where hits are, and whence rays leave.
    let mut seen = vec![];
    for _ in 0 .. rays {
        let d = unit(&mut rng);
        let q = c + axes[0] * half[0] * (2. * rng.next_f64() - 1.) + axes[1] * half[1] * (2. * rng.next_f64() - 1.) + axes[2] * half[2] * (2. * rng.next_f64() - 1.);
        let r = Ray { ro: q - d * (2. * diagonal), rd: d };
        let Some((t0, t1)) = volume.slab(&r) else { continue };
        match leaves.intersects(&r) {
            Some(h) if h.dist < t1 => {
                entering.push((h.dist - t0, true));
                seen.push(h.point);
            },
            _ => entering.push((t1 - t0, false)),
        }
    }
    // Leaving: from the points rays from outside meet first, every way. Those are the leaves a
    // hit is on, and they're lit better than leaves within (the correlation of what's seen with
    // what's lit, as in a canopy's hotspot); drawn from all the leaves by area, rays leaving
    // would be shadowed as interior leaves are.
    let mut leaving = vec![];
    for k in 0 .. rays {
        let Some(&p) = seen.get(k % seen.len().max(1)) else { break };
        let d = unit(&mut rng);
        let r = Ray { ro: p + d * (1e-6 * diagonal), rd: d };
        let exit = volume.slab(&r).map_or(0., |(_, t1)| t1);
        match leaves.intersects(&r) {
            Some(h) if h.dist < exit => leaving.push((h.dist, true)),
            _ => leaving.push((exit, false)),
        }
    }
    let quantise = |s: Vec<f64>| -> [u8; KNOTS] {
        let mut out = [0u8; KNOTS];
        for (o, v) in out.iter_mut().zip(s) {
            *o = (v.clamp(0., 1.) * 255.).round() as u8;
        }
        out
    };
    volume.enter = quantise(survival(entering, &knots));
    volume.leave = quantise(survival(leaving, &knots));
    Some(volume)
}
