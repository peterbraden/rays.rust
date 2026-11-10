//! # Shores: waves running up a beach
//!
//! The ocean in `crate::ocean` is deep water everywhere: every wave obeys ω² = gk and the surface
//! is one periodic FFT tile. An ocean with a `beach` adds a planar seabed rising to a shoreline,
//! and makes the waves feel it. The approach follows the procedural line of work on surf:
//!
//! - Alain Fournier and William Reeves, "A Simple Model of Ocean Waves", SIGGRAPH 1986, and
//!   Darwyn Peachey, "Modeling Waves and Surf", SIGGRAPH 1986: depth-dependent wavelength and
//!   speed, refraction towards the shore, steepening and breaking.
//! - Robert Dean and Robert Dalrymple, "Water Wave Mechanics for Engineers and Scientists",
//!   1991, for the linear theory of shoaling, refraction and the breaker index.
//!
//! ## Splitting the spectrum
//!
//! Depth changes each wave differently, so the wave field can no longer be a single FFT. The
//! strongest `components` wavevectors of the spectrum (by |h̃0(k)|, which near the peak of the
//! Phillips spectrum are the long swell) are taken out of the FFT and summed directly; the rest
//! remains as a "residual" tile. Each amplitude h̃0(k) contributes exactly
//!
//! ```text
//! -2 Re(h̃0(k) exp(i(k·x + ω t)))
//! ```
//!
//! to the ocean's heights (see `create_tile`, eq. 43, and the sign convention of `grid_height`),
//! a single wave travelling towards -k. So offshore, residual + components is the original ocean
//! exactly, and the ocean's own mesh is used there. Inshore of `deep_depth` the components are
//! shoaled and the residual (short, deep water chop) is just scaled down with the breaking.
//!
//! ## The beach
//!
//! The seabed is a plane: depth h = slope · s, where s is the distance offshore of the shoreline.
//! Since the depth only varies with s, each component can be followed analytically:
//!
//! - Dispersion in water of depth h: ω² = gκ tanh(κh). Waves slow and shorten as they come in.
//! - Refraction (Snell's law): the wavenumber along the shore is conserved, so the component
//!   across the shore is k_n(s) = √(κ(s)² - k_t²), and crests swing round towards the shore.
//!   The phase is the integral of k_n over s, tabulated.
//! - Shoaling: energy flux is conserved, so amplitude grows by Ks = √(cg_deep / cg), Green's
//!   law, A ∝ h^(-1/4), in the shallow limit. Refraction spreads or focuses the crests by
//!   Kr = √(cos θ_deep / cos θ).
//! - Breaking: waves cannot be higher than γh, the breaker index (McCowan's 0.78 by default).
//!   Inside the surf zone the significant wave height is held at γh by scaling every wave down,
//!   a "saturated" surf zone, so the energy is lost steadily all the way up the beach.
//! - Steepening: approaching breaking, each wave's profile leans forward, its front face
//!   steeper than its back (by skewing its phase), the look of a wave about to spill or plunge.
//! - Foam: where waves are breaking, in streaks along the crests.
//!
//! The beach itself is part of the object: sand above the waterline, wet near it, and seen
//! through shallow water, with light absorbed along the way down and back (Beer-Lambert). That
//! absorption, more in red than blue, gives the turquoise of shallow water over sand.
//!
//! Not modelled: overturning (a height field can't curl over), run-up and backwash, reflection
//! from the beach, and wave-driven currents.

use crate::color::Color;
use crate::geometry::{rand, random_point_on_unit_sphere};
use crate::intersection::{Intersection, RawIntersection};
use crate::material::functions::{direct_lighting, lambertian_direction, reflect, refract, scatter_lambertian, schlick};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::texture::Medium;
use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::ocean::{create_tile, dispersion, gen_k, grid_height, OceanGeometry, OceanParams};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scenefile::SceneFile;
use crate::sceneobject::SceneObject;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use rustfft::num_complex::Complex;
use serde_json::Value;
use std::f64::consts::PI;
use std::sync::Arc;

/// Refractive index of water.
const WATER_IOR: f64 = 1.33;

/// Samples in the tables over the shallow band.
const TABLE_SIZE: usize = 2048;

/// Depths are clamped to this, in metres, so wavenumbers stay finite at the waterline.
const MIN_DEPTH: f64 = 0.01;

/// The largest forward lean of a wave about to break (see `Shore::height`). Must be below 1.
const MAX_SKEW: f64 = 0.7;

/// A planar seabed rising to the shore. Positions on it are given by s, the distance offshore of
/// the shoreline, and u, the distance along it.
pub struct Beach {
    /// A point (x, z) on the shoreline, where the still water is 0 deep.
    pub shore: Vector2<f64>,
    /// Unit vector (x, z) towards deep water.
    pub offshore: Vector2<f64>,
    /// Unit vector (x, z) along the shoreline.
    pub along: Vector2<f64>,
    /// Rise over run of the seabed, tan β.
    pub slope: f64,
}

impl Beach {
    pub fn new(shore: Vector2<f64>, offshore: Vector2<f64>, slope: f64) -> Beach {
        let offshore = offshore.normalize();
        Beach { shore, offshore, along: Vector2::new(-offshore.y, offshore.x), slope }
    }

    /// Distance offshore of the shoreline, s.
    pub fn offshore_distance(&self, x: f64, z: f64) -> f64 {
        self.offshore.dot(&(Vector2::new(x, z) - self.shore))
    }

    /// Distance along the shoreline, u.
    pub fn along_distance(&self, x: f64, z: f64) -> f64 {
        self.along.dot(&(Vector2::new(x, z) - self.shore))
    }

    /// Height of the seabed (or beach, above the waterline) at (x, z).
    pub fn seabed_y(&self, x: f64, z: f64) -> f64 {
        -self.slope * self.offshore_distance(x, z)
    }

    /// The seabed's upward normal.
    pub fn normal(&self) -> Vector3<f64> {
        Vector3::new(self.slope * self.offshore.x, 1., self.slope * self.offshore.y).normalize()
    }

    /// Distance along the unit direction `d` from `ro` to the seabed, if it is ahead.
    pub fn intersect(&self, ro: &Vector3<f64>, d: &Vector3<f64>) -> Option<f64> {
        // The seabed is F(p) = p.y + slope s(p) = 0.
        let f0 = ro.y + self.slope * self.offshore_distance(ro.x, ro.z);
        let df = d.y + self.slope * self.offshore.dot(&Vector2::new(d.x, d.z));
        if df.abs() < 1e-12 { return None }
        let t = -f0 / df;
        if t > 0. { Some(t) } else { None }
    }
}

/// The wavenumber κ of a wave of angular frequency ω in water of depth h: the root of the
/// finite-depth dispersion relation ω² = gκ tanh(κh).
pub fn wavenumber(omega: f64, depth: f64, gravity: f64) -> f64 {
    let k_deep = omega * omega / gravity;
    // κ tanh(κh) = k_deep, and tanh(x) <= min(1, x), so κ is at least the deep and the shallow
    // water wavenumbers. Newton's method from there increases monotonically to the root.
    let mut k = k_deep.max(omega / (gravity * depth).sqrt());
    for _ in 0..50 {
        let th = (k * depth).tanh();
        let f = k * th - k_deep;
        let df = th + k * depth * (1. - th * th);
        let step = f / df;
        k -= step;
        if step.abs() < 1e-12 * k { break }
    }
    k
}

/// The group velocity of a wave of angular frequency ω in water of depth h: the speed its energy
/// travels at, (ω/κ) · ½(1 + 2κh / sinh 2κh).
pub fn group_velocity(omega: f64, depth: f64, gravity: f64) -> f64 {
    let k = wavenumber(omega, depth, gravity);
    let kh2 = 2. * k * depth;
    let n = if kh2 > 40. { 0.5 } else { 0.5 * (1. + kh2 / kh2.sinh()) };
    omega / k * n
}

/// One wave of the spectrum, followed in to the shore.
struct Wave {
    /// Twice |h̃0(k)|: its amplitude in deep water.
    amplitude: f64,
    /// Phase at the shore point, at the scene's time.
    phase0: f64,
    /// Wavenumber along the shore, k_t, conserved by refraction.
    k_along: f64,
    /// Wavenumber across the shore in deep water, k_n.
    k_offshore: f64,
    /// ∫ k_n ds, sampled over the shallow band.
    phase: Vec<f64>,
    /// Shoaling and refraction, Ks Kr, sampled over the shallow band.
    gain: Vec<f64>,
}

/// The water surface of an ocean meeting a beach. Offshore of `deep_offshore` it is the open
/// ocean; inshore of it, the shallow band, waves are shoaled, refracted and broken.
pub struct Shore {
    pub beach: Beach,
    /// s at the offshore edge of the shallow band, where the depth is `deep_depth`.
    pub deep_offshore: f64,
    /// Breaker index γ: the greatest wave height for the depth.
    pub breaker_index: f64,
    waves: Vec<Wave>,
    /// Heights of the residual tile at its grid points, row-major with rows along z.
    residual: Vec<f64>,
    fourier_grid_size: usize,
    lx: f64,
    /// Over the shallow band: the factor every wave is scaled by to keep it from exceeding the
    /// breaker index, 1 outside the surf zone.
    saturation: Vec<f64>,
    /// Over the shallow band: significant wave height Hs.
    significant_height: Vec<f64>,
    /// Over the shallow band: forward lean of the waves, 0 to `MAX_SKEW`.
    skew: Vec<f64>,
    /// Over the shallow band: a bound on the slope of the surface, for ray marching.
    slope_bound: Vec<f64>,
    /// No wave in the shallow band is higher, or its trough lower, than this.
    pub height_bound: f64,
}

/// Linearly interpolate `table`, sampled evenly over s in [0, `end`].
fn sample(table: &[f64], s: f64, end: f64) -> f64 {
    let x = (s / end).clamp(0., 1.) * (table.len() - 1) as f64;
    let i = (x.floor() as usize).min(table.len() - 2);
    let f = x - i as f64;
    table[i] * (1. - f) + table[i + 1] * f
}

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

impl Shore {
    /// The shore for an ocean's amplitudes `h0`. `components` waves are shoaled; `deep_depth`,
    /// if given, is the depth at which the waves start to feel the bottom.
    pub fn new(
        params: &OceanParams,
        h0: &[Complex<f64>],
        beach: Beach,
        components: usize,
        deep_depth: Option<f64>,
        breaker_index: f64,
    ) -> Shore {
        let n = params.fourier_grid_size;
        let g = params.gravity;

        // The strongest waves. Row and column 0 are the Nyquist frequency -N/2, whose -k wraps
        // round to itself (see `create_tile`); they are left in the residual.
        let mut candidates: Vec<usize> = (0..n * n).filter(|i| i % n != 0 && i / n != 0).collect();
        candidates.sort_by(|a, b| h0[*b].norm().partial_cmp(&h0[*a].norm()).unwrap());
        candidates.truncate(components);
        candidates.retain(|i| h0[*i].norm() > 0.);

        let mut residual_h0 = h0.to_vec();
        for i in &candidates { residual_h0[*i] = Complex::new(0., 0.); }
        let residual_tile = create_tile(n, params.lx, params.lx, g, params.time, false, residual_h0);
        let mut residual = vec![0.; n * n];
        for z in 0..n {
            for x in 0..n {
                residual[z * n + x] = grid_height(&residual_tile.heights, x, z, n);
            }
        }
        let residual_variance = residual.iter().map(|h| h * h).sum::<f64>() / (n * n) as f64;
        let residual_max = residual.iter().fold(0f64, |m, h| m.max(h.abs()));
        let spacing = params.lx / n as f64;
        let mut residual_slope = 0f64;
        for z in 0..n {
            for x in 0..n {
                let h = residual[z * n + x];
                residual_slope = residual_slope
                    .max((residual[z * n + (x + 1) % n] - h).abs() / spacing)
                    .max((residual[((z + 1) % n) * n + x] - h).abs() / spacing);
            }
        }

        let ks: Vec<Vector2<f64>> = candidates.iter().map(|i| {
            let (col, row) = (i % n, i / n);
            gen_k(col as f64 - (n / 2) as f64, row as f64 - (n / 2) as f64, params.lx, params.lx)
        }).collect();

        // Waves feel the bottom once the depth is less than about half their wavelength, kh < π.
        // The band must also start offshore of where the waves break.
        let k_min = ks.iter().map(|k| k.norm()).fold(f64::INFINITY, f64::min);
        let deep_hs = 4. * (residual_variance
            + candidates.iter().map(|i| (2. * h0[*i].norm()).powi(2) / 2.).sum::<f64>()).sqrt();
        let deep_depth = deep_depth.unwrap_or_else(|| (PI / k_min).min(12.).max(2. * deep_hs / breaker_index));
        let deep_offshore = deep_depth / beach.slope;
        let ds = deep_offshore / (TABLE_SIZE - 1) as f64;
        let depth_at = |i: usize| (beach.slope * i as f64 * ds).max(MIN_DEPTH);

        let waves: Vec<Wave> = candidates.iter().zip(&ks).map(|(i, k)| {
            let k0 = k.norm();
            let omega = (g * k0).sqrt();
            let k_offshore = k.dot(&beach.offshore);
            let k_along = k.dot(&beach.along);
            // Normalise so that the wave is exactly the deep water wave at the edge of the band.
            let k_scale = k0 / wavenumber(omega, deep_depth, g);
            let cg_deep = group_velocity(omega, deep_depth, g);
            let cos_deep = k_offshore.abs() / k0;
            let k_normal = |depth: f64| {
                let kappa = wavenumber(omega, depth, g) * k_scale;
                (kappa * kappa - k_along * k_along).max(0.).sqrt() * k_offshore.signum()
            };

            let gain = (0..TABLE_SIZE).map(|j| {
                let depth = depth_at(j);
                let kappa = wavenumber(omega, depth, g) * k_scale;
                let shoaling = (cg_deep / group_velocity(omega, depth, g)).sqrt();
                let cos = k_normal(depth).abs() / kappa;
                let refraction = if cos > 1e-6 { (cos_deep / cos).sqrt() } else { 1. };
                shoaling * refraction
            }).collect();

            // Integrate k_n inwards from the edge of the band, where the phase is k_n s.
            let mut phase = vec![0.; TABLE_SIZE];
            phase[TABLE_SIZE - 1] = k_offshore * deep_offshore;
            const SUBSTEPS: usize = 8;
            for j in (0..TABLE_SIZE - 1).rev() {
                let integral: f64 = (0..SUBSTEPS).map(|q| {
                    let s = (j as f64 + (q as f64 + 0.5) / SUBSTEPS as f64) * ds;
                    k_normal((beach.slope * s).max(MIN_DEPTH)) * ds / SUBSTEPS as f64
                }).sum();
                phase[j] = phase[j + 1] - integral;
            }

            let shore_point = Vector2::new(beach.shore.x, beach.shore.y);
            Wave {
                amplitude: 2. * h0[*i].norm(),
                phase0: h0[*i].arg() + k.dot(&shore_point) + dispersion(*k, g) * params.time,
                k_along,
                k_offshore,
                phase,
                gain,
            }
        }).collect();

        let mut saturation = vec![0.; TABLE_SIZE];
        let mut significant_height = vec![0.; TABLE_SIZE];
        let mut skew = vec![0.; TABLE_SIZE];
        let mut slope_bound = vec![0.; TABLE_SIZE];
        let mut height_bound = residual_max + waves.iter().map(|w| w.amplitude).sum::<f64>();
        for j in 0..TABLE_SIZE {
            let depth = beach.slope * j as f64 * ds;
            let variance = residual_variance + waves.iter()
                .map(|w| (w.amplitude * w.gain[j]).powi(2) / 2.)
                .sum::<f64>();
            let hs = 4. * variance.sqrt();
            let limit = breaker_index * depth;
            let b = if hs > limit { limit / hs } else { 1. };
            significant_height[j] = hs * b;
            saturation[j] = b;
            // Tapered to nothing at the edge of the band, to meet the open ocean.
            let edge_taper = 1. - smoothstep(0.8, 1., j as f64 / (TABLE_SIZE - 1) as f64);
            skew[j] = MAX_SKEW * smoothstep(0.3, 1., hs / limit.max(1e-9)) * edge_taper;

            let lean = 1. + skew[j];
            slope_bound[j] = b * (residual_slope + waves.iter().map(|w| {
                let kappa = wavenumber((g * w.k_along.hypot(w.k_offshore)).sqrt(), depth.max(MIN_DEPTH), g);
                w.amplitude * w.gain[j] * kappa * lean
            }).sum::<f64>());
            height_bound = height_bound.max(
                b * (residual_max + waves.iter().map(|w| w.amplitude * w.gain[j]).sum::<f64>()) * lean);
        }
        if saturation[TABLE_SIZE - 1] < 1. {
            println!("  WARNING: waves are already breaking at the beach's deep_depth; increase it");
        }

        Shore {
            beach,
            deep_offshore,
            breaker_index,
            waves,
            residual,
            fourier_grid_size: n,
            lx: params.lx,
            saturation,
            significant_height,
            skew,
            slope_bound,
            height_bound,
        }
    }

    /// The residual tile at (x, z), interpolated across the same triangles as the ocean's mesh.
    fn residual_at(&self, x: f64, z: f64) -> f64 {
        let n = self.fourier_grid_size;
        let gx = x / self.lx * n as f64;
        let gz = z / self.lx * n as f64;
        let (fx, fz) = (gx - gx.floor(), gz - gz.floor());
        let ix = (gx.floor() as i64).rem_euclid(n as i64) as usize;
        let iz = (gz.floor() as i64).rem_euclid(n as i64) as usize;
        let at = |dx: usize, dz: usize| self.residual[((iz + dz) % n) * n + (ix + dx) % n];
        let (h00, h10, h01, h11) = (at(0, 0), at(1, 0), at(0, 1), at(1, 1));
        if fx + fz <= 1. {
            h00 + fx * (h10 - h00) + fz * (h01 - h00)
        } else {
            h11 + (1. - fx) * (h01 - h11) + (1. - fz) * (h10 - h11)
        }
    }

    /// Height of the water surface at (x, z). Offshore of the shallow band this is the open
    /// ocean; on land, the still water level, 0.
    pub fn height(&self, x: f64, z: f64) -> f64 {
        let s = self.beach.offshore_distance(x, z);
        if s <= 0. { return 0. }
        let u = self.beach.along_distance(x, z);
        let end = self.deep_offshore;
        let deep = s >= end;
        let (b, q) = if deep { (1., 0.) } else { (sample(&self.saturation, s, end), sample(&self.skew, s, end)) };
        if b <= 0. { return 0. }

        let mut h = self.residual_at(x, z) * b;
        for w in &self.waves {
            let (phase, gain) = if deep {
                (w.k_offshore * s, 1.)
            } else {
                (sample(&w.phase, s, end), sample(&w.gain, s, end))
            };
            let psi = w.phase0 + phase + w.k_along * u;
            // A wave moves towards decreasing phase, so its front face is where ψ runs from 0
            // (the trough) to π (the crest). Skewing the phase by -q cos ψ shortens that face.
            h -= w.amplitude * gain * b * (psi - q * psi.cos()).cos();
        }
        h
    }

    /// The upward normal of the water surface at (x, z).
    pub fn normal(&self, x: f64, z: f64) -> Vector3<f64> {
        let e = 0.01;
        let dx = (self.height(x + e, z) - self.height(x - e, z)) / (2. * e);
        let dz = (self.height(x, z + e) - self.height(x, z - e)) / (2. * e);
        Vector3::new(-dx, 1., -dz).normalize()
    }

    /// How much the waves at s are breaking: 0 offshore of the surf zone, approaching 1 at the
    /// waterline.
    pub fn breaking(&self, s: f64) -> f64 {
        if s <= 0. { return 1. }
        if s >= self.deep_offshore { return 0. }
        1. - sample(&self.saturation, s, self.deep_offshore)
    }

    /// Significant wave height at s.
    pub fn significant_height(&self, s: f64) -> f64 {
        sample(&self.significant_height, s.min(self.deep_offshore), self.deep_offshore)
    }

    /// Distance along the unit direction `d` from `ro` to the water surface in the shallow band,
    /// if less than `t_max`. Marches along the ray in steps no longer than the surface's slope
    /// allows it to rise to meet the ray, then bisects the step that crosses it.
    fn band_hit(&self, ro: &Vector3<f64>, d: &Vector3<f64>, t_max: f64) -> Option<f64> {
        let (mut ta, mut tb) = (0f64, t_max);
        // The band, s in [0, deep_offshore].
        let s0 = self.beach.offshore_distance(ro.x, ro.z);
        let ds = self.beach.offshore.dot(&Vector2::new(d.x, d.z));
        if ds.abs() < 1e-12 {
            if s0 < 0. || s0 > self.deep_offshore { return None }
        } else {
            let (t0, t1) = ((0. - s0) / ds, (self.deep_offshore - s0) / ds);
            ta = ta.max(t0.min(t1));
            tb = tb.min(t0.max(t1));
        }
        // The slab the waves stay within.
        let y = self.height_bound;
        if d.y.abs() < 1e-12 {
            if ro.y.abs() > y { return None }
        } else {
            let (t0, t1) = ((-y - ro.y) / d.y, (y - ro.y) / d.y);
            ta = ta.max(t0.min(t1));
            tb = tb.min(t0.max(t1));
        }
        if ta >= tb { return None }

        let f = |t: f64| { let p = ro + d * t; p.y - self.height(p.x, p.z) };
        let horizontal = d.x.hypot(d.z);
        let mut t = ta;
        let mut ft = f(t);
        let above = ft > 0.;
        for _ in 0..20000 {
            if t >= tb { return None }
            let p = ro + d * t;
            let s = self.beach.offshore_distance(p.x, p.z);
            let slope = sample(&self.slope_bound, s, self.deep_offshore);
            let min_step = 0.005 + 0.0005 * t;
            let step = (ft.abs() / (d.y.abs() + 1.25 * slope * horizontal)).clamp(min_step, 5.);
            let tn = (t + step).min(tb);
            let fnext = f(tn);
            if (fnext > 0.) != above {
                let (mut lo, mut hi) = (t, tn);
                for _ in 0..30 {
                    let mid = 0.5 * (lo + hi);
                    if (f(mid) > 0.) == above { lo = mid } else { hi = mid }
                }
                return Some(0.5 * (lo + hi));
            }
            t = tn;
            ft = fnext;
        }
        None
    }
}

/// The ocean's surface and the beach under it, as one object.
pub struct ShoreGeometry {
    shore: Arc<Shore>,
    deep: OceanGeometry,
}

impl Geometry for ShoreGeometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let len = r.rd.norm();
        if len < 1e-12 { return None }
        let d = r.rd / len;
        let beach = &self.shore.beach;
        let mut best: Option<(f64, Vector3<f64>)> = beach.intersect(&r.ro, &d).map(|t| (t, beach.normal()));
        let best_t = |b: &Option<(f64, Vector3<f64>)>| b.map_or(f64::INFINITY, |b| b.0);

        if let Some(t) = self.shore.band_hit(&r.ro, &d, best_t(&best)) {
            let p = r.ro + d * t;
            best = Some((t, self.shore.normal(p.x, p.z)));
        }

        // The open ocean, offshore of the band. A ray crosses the band's edge at most once.
        let edge = self.shore.deep_offshore;
        let s0 = beach.offshore_distance(r.ro.x, r.ro.z);
        let ds = beach.offshore.dot(&Vector2::new(d.x, d.z));
        let deep_hit = if s0 >= edge {
            self.deep.intersects(&Ray { ro: r.ro, rd: d })
                .filter(|h| beach.offshore_distance(h.point.x, h.point.z) >= edge)
        } else if ds > 0. {
            let tc = (edge - s0) / ds;
            if tc < best_t(&best) {
                self.deep.intersects(&Ray { ro: r.ro + d * tc, rd: d })
            } else { None }
        } else { None };
        if let Some(h) = deep_hit {
            let t = (h.point - r.ro).norm();
            if t < best_t(&best) { best = Some((t, h.normal)) }
        }

        best.map(|(t, normal)| RawIntersection { dist: t, point: r.ro + d * t, normal })
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, f64::MIN, f64::MIN),
            Vector3::new(f64::MAX, f64::MAX, f64::MAX),
        )
    }

    fn primitives(&self) -> u64 {
        self.deep.primitives() + self.shore.waves.len() as u64 + 1
    }
}

fn transmittance(absorption: &Vector3<f64>, distance: f64) -> Color {
    Color::new(
        (-absorption.x * distance).exp(),
        (-absorption.y * distance).exp(),
        (-absorption.z * distance).exp(),
    )
}

/// Water that may be shallow: the ocean's material, seeing through to the seabed where it is
/// shallow enough, with foam where waves break.
pub struct ShoreWater {
    shore: Arc<Shore>,
    /// Colour of light scattered back out of deep water.
    deep_color: Color,
    /// Absorption coefficients per metre, per channel.
    absorption: Vector3<f64>,
    foam: Color,
    sand: Color,
    noise: PerlinNoise,
}

impl ShoreWater {
    /// Fraction of the surface at `p` covered by foam.
    fn foam_coverage(&self, p: &Vector3<f64>) -> f64 {
        let beach = &self.shore.beach;
        let s = beach.offshore_distance(p.x, p.z);
        let breaking = self.shore.breaking(s);
        if breaking < 1e-3 { return 0. }
        let u = beach.along_distance(p.x, p.z);
        // Streaks along the crests.
        let streaks = 0.5 + 0.5 * self.noise.fbm(u * 0.08, 0.5, s * 0.5, 4, 0.5, 2.);
        // On the crests of the breaking waves...
        let hs = self.shore.significant_height(s).max(0.1);
        let crest = smoothstep(0.1 * hs, 0.5 * hs, p.y);
        // ...and a thin line where the last of each wave runs up the beach.
        let swash = 1. - smoothstep(0., 0.06, beach.slope * s);
        ((breaking * (0.05 + 0.95 * crest) + 0.6 * swash) * (0.2 + 1.3 * streaks)).clamp(0., 0.95)
    }

    /// Light refracted down into the water along `t`: absorbed and scattered on the way to the
    /// seabed, if it is near enough to see, reflected from it, and absorbed on the way back.
    fn transmit(&self, intersection: &Intersection, t: Vector3<f64>, s: &Scene) -> ScatteredRay {
        let t = t.normalize();
        // Rather than refract, shade darker depending whether the ray is deeper than 90degrees
        // down (as the open ocean's material does).
        let water = self.deep_color * (1. / (t.y.acos() * 0.8));
        let opaque = ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: water, ray: None };

        let beach = &self.shore.beach;
        let p = intersection.point;
        if beach.offshore_distance(p.x, p.z) >= self.shore.deep_offshore { return opaque }
        let down = match beach.intersect(&p, &t) { Some(d) => d, None => return opaque };
        let through = transmittance(&self.absorption, down);
        if through.rgb.max() < 1e-3 { return opaque }

        let bed = p + t * down;
        let depth = (p.y - bed.y).max(0.);
        let bed_normal = beach.normal();
        let surface_normal = intersection.normal;
        let mut emit = water * (Color::white() + through * -1.);

        // Point lights on the seabed, refracted in through the surface here.
        let mut direct = Color::black();
        for light in &s.lights {
            let to_light = light.position - p;
            let l = to_light.normalize();
            let cos_i = l.dot(&surface_normal);
            if cos_i <= 0. { continue }
            if s.objects.occluded(&Ray { ro: p, rd: l }, to_light.norm(), f64::MIN_POSITIVE) { continue }
            let sin_t = (1. - cos_i * cos_i).sqrt() / WATER_IOR;
            let cos_t = (1. - sin_t * sin_t).sqrt();
            let lit = transmittance(&self.absorption, depth / cos_t) * (1. - schlick(cos_i, WATER_IOR));
            // Refraction bends the beam towards the seabed's normal but doesn't concentrate it:
            // the irradiance is that on the surface, so falls with the angle of incidence there.
            let cos_bed = l.dot(&bed_normal).max(0.);
            let distance2 = (light.position - bed).norm_squared();
            direct = direct + light.color * lit * (cos_bed * light.intensity / (PI * distance2));
        }
        emit = emit + through * self.sand * direct;

        // Light from above reaching the seabed: follow a diffuse bounce back up and out.
        let up = lambertian_direction(bed_normal, random_point_on_unit_sphere());
        let back = transmittance(&self.absorption, depth / up.y.max(0.05));
        match refract(up, -surface_normal, WATER_IOR) {
            Some(out) => {
                let exit = 1. - schlick(up.dot(&surface_normal).abs(), WATER_IOR);
                ScatteredRay {
                    sampled_lights: false,
                    emit,
                    attenuate: through * self.sand * back * exit,
                    ray: Some(Ray { ro: p, rd: out }),
                }
            },
            // Total internal reflection: the light stays in the water and is absorbed.
            None => ScatteredRay { sampled_lights: false, emit, attenuate: Color::black(), ray: None },
        }
    }
}

impl MaterialModel for ShoreWater {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let normal = intersection.normal;
        if r.rd.dot(&normal) > 0. {
            // From below: carry on out.
            return ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::white(),
                ray: Some(Ray { ro: intersection.point + r.rd.normalize() * 0.001, rd: r.rd }) };
        }

        if rand() < self.foam_coverage(&intersection.point) {
            return ScatteredRay {
                emit: direct_lighting(self.foam, intersection, s),
                sampled_lights: true,
                ..scatter_lambertian(self.foam, intersection)
            };
        }

        let cosine = -r.rd.dot(&normal) / r.rd.norm();
        match refract(r.rd, normal, 1. / WATER_IOR) {
            Some(t) if rand() >= schlick(cosine, WATER_IOR) => return self.transmit(intersection, t, s),
            // Fresnel reflection, or total internal reflection.
            _ => {},
        }
        ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::white(),
            ray: Some(Ray { ro: intersection.point, rd: reflect(r.rd, normal) }) }
    }
}

/// The beach above the water, darker where the waves have wet it.
pub struct Sand {
    dry: Color,
    /// Height up the beach the waves wet.
    wet_height: f64,
}

impl Sand {
    fn albedo(&self, y: f64) -> Color {
        let wet = 1. - smoothstep(self.wet_height - 0.1, self.wet_height + 0.1, y);
        self.dry * (1. - 0.45 * wet)
    }
}

impl MaterialModel for Sand {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let intersection = &intersection.facing(r);
        let albedo = self.albedo(intersection.point.y);
        ScatteredRay {
            emit: direct_lighting(albedo, intersection, s),
            sampled_lights: true,
            ..scatter_lambertian(albedo, intersection)
        }
    }
}

/// Water or sand, whichever surface a point is on.
pub struct ShoreMedium {
    shore: Arc<Shore>,
    water: ShoreWater,
    sand: Sand,
}

impl Medium for ShoreMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        let beach = &self.shore.beach;
        if beach.offshore_distance(pt.x, pt.z) >= self.shore.deep_offshore { return &self.water }
        let to_sand = (pt.y - beach.seabed_y(pt.x, pt.z)).abs();
        let to_water = (pt.y - self.shore.height(pt.x, pt.z)).abs();
        if to_sand < to_water { &self.sand } else { &self.water }
    }
}

/// An ocean with a `beach`:
///
/// ```json
/// "beach": {
///   "shore": [0, 0],          // a point (x, z) on the shoreline
///   "offshore": [0, 1],       // direction (x, z) towards deep water
///   "slope": 0.03,            // rise over run of the seabed
///   "breaker_index": 0.78,    // greatest wave height, as a fraction of the depth
///   "components": 32,         // waves taken out of the FFT and shoaled
///   "deep_depth": 12,         // depth at which waves start to feel the bottom (default: from the longest wave, at most 12)
///   "absorption": [0.4, 0.07, 0.05], // of the water, per metre
///   "sand": [0.76, 0.68, 0.5],
///   "foam": [0.9, 0.9, 0.9],
///   "wet_height": 0.3         // height up the beach the waves wet
/// }
/// ```
pub fn create_shore(opts: &Value) -> SceneObject {
    let b = &opts["beach"];
    let params = OceanParams::parse(opts);
    let beach = Beach::new(
        SceneFile::parse_vec2_def(b, "shore", Vector2::new(0., 0.)),
        SceneFile::parse_vec2_def(b, "offshore", Vector2::new(0., 1.)),
        SceneFile::parse_number(&b["slope"], 0.03),
    );
    let h0 = params.amplitudes();
    let shore = Arc::new(Shore::new(
        &params,
        &h0,
        beach,
        SceneFile::parse_int(&b["components"], 32),
        b.get("deep_depth").map(|d| SceneFile::parse_number(d, 12.)),
        SceneFile::parse_number(&b["breaker_index"], 0.78),
    ));
    let tile = create_tile(params.fourier_grid_size, params.lx, params.lx, params.gravity, params.time, false, h0);
    println!(" - OCEAN with beach [A={}, W={}, t={}, slope={}, waves={}, shallow band {:.0}m wide, Hs at the band {:.2}m]",
        params.scale, params.wind, params.time, shore.beach.slope, shore.waves.len(), shore.deep_offshore,
        shore.significant_height(shore.deep_offshore));
    let deep = OceanGeometry::from_tile(&tile, params.fourier_grid_size, params.lx);

    let absorption = SceneFile::parse_color_def(b, "absorption", Color::new(0.4, 0.07, 0.05)).rgb;
    let sand = SceneFile::parse_color_def(b, "sand", Color::new(0.76, 0.68, 0.5));
    SceneObject {
        geometry: Box::new(ShoreGeometry { shore: shore.clone(), deep }),
        medium: Box::new(ShoreMedium {
            shore: shore.clone(),
            water: ShoreWater {
                shore: shore.clone(),
                deep_color: SceneFile::parse_color_def(opts, "color", Color::new(0., 0.2, 0.3)),
                absorption,
                foam: SceneFile::parse_color_def(b, "foam", Color::new(0.9, 0.9, 0.9)),
                sand,
                noise: PerlinNoise::new(),
            },
            sand: Sand { dry: sand, wet_height: SceneFile::parse_number(&b["wet_height"], 0.3) },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector2;

    fn params() -> OceanParams {
        OceanParams { gravity: 9.81, wind: Vector2::new(8., 2.), time: 3., lx: 100., fourier_grid_size: 64, scale: 2e-5, seed: 0 }
    }

    fn shore(slope: f64) -> Shore {
        let p = params();
        let h0 = p.amplitudes();
        let shore = Shore::new(&p, &h0, Beach::new(Vector2::new(0., 0.), Vector2::new(0., 1.), slope), 16, Some(10.), 0.78);
        // Waves big enough to break in the band, but not at its edge.
        let hs = shore.significant_height(shore.deep_offshore);
        assert!(hs > 0.5 && hs < 0.5 * 0.78 * 10., "Hs {}", hs);
        shore
    }

    #[test]
    fn test_wavenumber_limits() {
        let g = 9.81;
        // Deep water: ω² = gk.
        let k = wavenumber(2., 1000., g);
        assert!((k - 4. / g).abs() < 1e-9, "{}", k);
        // Shallow water: speed √(gh).
        let k = wavenumber(0.1, 0.01, g);
        assert!((0.1 / k - (g * 0.01f64).sqrt()).abs() < 1e-4, "{}", k);
        // The root.
        let (w, h) = (1.3, 2.);
        let k = wavenumber(w, h, g);
        assert!((g * k * (k * h).tanh() - w * w).abs() < 1e-9);
    }

    #[test]
    fn test_greens_law() {
        // In shallow water, amplitude ∝ h^(-1/4): group velocity ∝ √h.
        let g = 9.81;
        let ratio = group_velocity(0.2, 0.04, g) / group_velocity(0.2, 0.01, g);
        assert!((ratio - 2.).abs() < 0.01, "{}", ratio);
    }

    #[test]
    fn test_offshore_matches_ocean_mesh() {
        // Offshore of the band, residual + components is the ocean exactly: compare at the grid
        // points, where the mesh's vertices are.
        let p = params();
        let h0 = p.amplitudes();
        let shore = Shore::new(&p, &h0, Beach::new(Vector2::new(0., -1000.), Vector2::new(0., 1.), 0.05), 16, Some(10.), 0.78);
        let tile = create_tile(p.fourier_grid_size, p.lx, p.lx, p.gravity, p.time, false, h0);
        let n = p.fourier_grid_size;
        let mut max = 0f64;
        for (x, z) in [(3, 5), (10, 40), (63, 1), (32, 32), (17, 60)] {
            let mesh = grid_height(&tile.heights, x, z, n);
            let ours = shore.height(x as f64 * p.lx / n as f64, z as f64 * p.lx / n as f64);
            assert!((mesh - ours).abs() < 1e-9, "({}, {}): {} vs {}", x, z, mesh, ours);
            max = max.max(mesh.abs());
        }
        assert!(max > 0.);
    }

    #[test]
    fn test_band_continuous_at_its_edge() {
        let shore = shore(0.05);
        let edge = shore.deep_offshore;
        for x in [0., 13.7, 51.2] {
            let inside = shore.height(x, edge - 1e-6);
            let outside = shore.height(x, edge + 1e-6);
            assert!((inside - outside).abs() < 1e-3, "{} vs {}", inside, outside);
        }
    }

    #[test]
    fn test_waves_break_before_the_shore() {
        let shore = shore(0.05);
        // Inshore, wave height is limited by the depth; at the waterline the water is still.
        for s in [0.5, 2., 5., 20.] {
            let hs = shore.significant_height(s);
            assert!(hs <= 0.78 * 0.05 * s + 1e-9, "Hs {} at s {}", hs, s);
        }
        assert_eq!(shore.height(10., 0.), 0.);
        assert!(shore.breaking(1.) > 0.5);
        assert_eq!(shore.breaking(shore.deep_offshore + 1.), 0.);
        assert_eq!(shore.height(10., -5.), 0.);
    }

    #[test]
    fn test_waves_shorten_in_shallow_water() {
        // A wave's phase changes faster across the shore where it is shallow.
        let shore = shore(0.05);
        let w = shore.waves.iter().find(|w| w.k_offshore.abs() > 0.05).unwrap();
        let end = shore.deep_offshore;
        let rate = |s: f64| (sample(&w.phase, s + 0.5, end) - sample(&w.phase, s - 0.5, end)).abs();
        assert!(rate(end * 0.1) > 1.5 * rate(end * 0.9), "{} {}", rate(end * 0.1), rate(end * 0.9));
    }

    #[test]
    fn test_ray_hits_shallow_water_and_beach() {
        let p = params();
        let h0 = p.amplitudes();
        let shore = Arc::new(Shore::new(&p, &h0, Beach::new(Vector2::new(0., 0.), Vector2::new(0., 1.), 0.05), 16, Some(10.), 0.78));
        let tile = create_tile(p.fourier_grid_size, p.lx, p.lx, p.gravity, p.time, false, h0);
        let g = ShoreGeometry { shore: shore.clone(), deep: OceanGeometry::from_tile(&tile, p.fourier_grid_size, p.lx) };

        // Straight down into the surf zone: hits the water surface.
        let hit = g.intersects(&Ray { ro: Vector3::new(3., 20., 30.), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!((hit.point.y - shore.height(3., 30.)).abs() < 1e-6, "{} vs {}", hit.point.y, shore.height(3., 30.));

        // Down onto the beach: hits the sand.
        let hit = g.intersects(&Ray { ro: Vector3::new(3., 20., -30.), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!((hit.point.y - 1.5).abs() < 1e-6, "{}", hit.point);

        // Out to sea, beyond the band: hits the open ocean.
        let far = shore.deep_offshore + 50.;
        let hit = g.intersects(&Ray { ro: Vector3::new(3., 20., far), rd: Vector3::new(0., -1., 0.) }).unwrap();
        assert!((hit.point.y - shore.height(3., far)).abs() < 0.2, "{} vs {}", hit.point.y, shore.height(3., far));

        // From the beach, looking out to sea at a shallow angle, crossing the band.
        let hit = g.intersects(&Ray { ro: Vector3::new(0., 3., -10.), rd: Vector3::new(0., -0.05, 1.) }).unwrap();
        assert!(hit.point.z > 0., "{}", hit.point);
    }
}
