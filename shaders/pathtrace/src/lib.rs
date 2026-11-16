//! The GPU path tracer, in Rust, compiled to SPIR-V by rust-gpu: one invocation per pixel per
//! pass over the flattened scene (see the renderer's `gpu/mod.rs` for the buffers' layouts,
//! which these structs match). It ports the CPU wavefront integrator's shading
//! (`wavefront::integrator`): the same BSDFs, light sampling with the power heuristic, Russian
//! roulette and regularisation, in f32.
#![cfg_attr(target_arch = "spirv", no_std)]
#![allow(clippy::too_many_arguments)]

use spirv_std::glam::{UVec3, UVec4, Vec3, Vec4, Vec4Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

/// glam, as the shader sees it, for building its buffers on the CPU.
pub use spirv_std::glam;

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Params {
    /// Width, height, pass, max depth.
    pub dims: UVec4,
    /// Bounded primitives, planes, emitters, lights.
    pub counts: UVec4,
    /// In `tables`: emitter primitives, emitter cdf, sky radiance, sky marginal cdf.
    pub offs: UVec4,
    /// Sky conditional cdfs, sky width, sky height, flags.
    pub offs2: UVec4,
    /// Regularisation floor, black threshold, shadow bias, Russian roulette's first depth.
    pub misc: Vec4,
    /// The sun's direction and cos of its angular radius.
    pub sun_dir: Vec4,
    /// The sun's irradiance and angular radius.
    pub sun_irr: Vec4,
    /// Seed, Russian roulette on, aerial perspective table.
    pub seed: UVec4,
    /// The sun's azimuth frame for the aerial perspective table.
    pub ap_x: Vec4,
    pub ap_z: Vec4,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Prim {
    /// A triangle's vertices, or a sphere's centre and radius, or a plane's y in v0.x.
    pub v0: Vec4,
    pub v1: Vec4,
    pub v2: Vec4,
    /// Vertex normals, for smooth triangles.
    pub n0: Vec4,
    pub n1: Vec4,
    pub n2: Vec4,
    /// Kind (0 triangle, 1 sphere, 2 plane), material, emitter + 1 (0 none), smooth.
    pub info: UVec4,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Node {
    pub bmin: Vec3,
    /// A leaf's first item, or an interior node's second child.
    pub offset: u32,
    pub bmax: Vec3,
    /// A leaf's items; 0 for an interior node.
    pub count: u32,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Material {
    /// 0 diffuse, 1 plastic, 2 dielectric, 3 conductor, 4 emitter, 5 metal, 6 checker.
    pub kind: u32,
    /// A checkerboard's first material; an emitter's two-sidedness.
    pub a: u32,
    /// A checkerboard's second material.
    pub b: u32,
    pub pad: u32,
    /// Albedo, tint, radiance, or a conductor's eta.
    pub c0: Vec4,
    /// A conductor's k.
    pub c1: Vec4,
    /// Refractive index, roughness, checker x size, z size.
    pub p: Vec4,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Light {
    /// Position, and kind (0 point, 1 spot).
    pub pos: Vec4,
    pub intensity: Vec4,
    /// A spot's axis, and the cos at the start of its falloff.
    pub axis: Vec4,
    /// The cos at its edge.
    pub extra: Vec4,
}

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct CameraRay {
    /// Origin, and the sample's weight.
    pub o: Vec4,
    /// Direction, and 1 if there is a ray.
    pub d: Vec4,
}

const FLAG_SAMPLE_LIGHTS: u32 = 1;
const FLAG_SAMPLE_ENV: u32 = 2;
const FLAG_SUN: u32 = 4;
const FLAG_AERIAL: u32 = 8;
const AP_AZIMUTH: u32 = 32;
const AP_ELEVATION: u32 = 64;
const AP_DISTANCE: u32 = 32;
const AERIAL_FROM: f32 = 100.0;
const AERIAL_TO: f32 = 2e5;
const PI: f32 = core::f32::consts::PI;
const MIN_ALPHA: f32 = 1e-3;
const INF: f32 = 3.0e38;
const LIMB_DARKENING: f32 = 0.6;
const NONE: u32 = 0xffff_ffff;
const STACK: usize = 100;

/// The scene's buffers.
pub struct Scene<'a> {
    pub params: &'a Params,
    pub prims: &'a [Prim],
    pub nodes: &'a [Node],
    pub materials: &'a [Material],
    pub lights: &'a [Light],
    pub tables: &'a [f32],
}

/// `slice[i]` without the bounds check: the host sizes every buffer for the indices it writes
/// into the others (a checked index costs a branch to a panic each time, a third of the
/// shader's time on lavapipe).
#[inline(always)]
fn at<T: Copy>(slice: &[T], i: u32) -> T {
    unsafe { *slice.get_unchecked(i as usize) }
}

// ---- Random numbers: a counter-based hash (PCG), per pixel, pass and dimension. ----

pub fn pcg(v: u32) -> u32 {
    let state = v.wrapping_mul(747796405).wrapping_add(2891336453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);
    (word >> 22) ^ word
}

/// The random number state for pixel `index` in pass `pass`.
pub fn path_seed(index: u32, pass: u32, seed: u32) -> u32 {
    pcg(index ^ pcg(pass.wrapping_mul(0x9e3779b9) ^ seed))
}

fn rand(state: &mut u32) -> f32 {
    *state = pcg(*state);
    (*state >> 8) as f32 * (1.0 / 16777216.0)
}

fn fract(x: f32) -> f32 {
    x - x.floor()
}

// ---- Geometry. ----

#[derive(Copy, Clone)]
struct Hit {
    pub t: f32,
    prim: u32,
    ng: Vec3,
    ns: Vec3,
}

/// (t, b1, b2, 1) for a hit within (tmin, tmax), else zero.
fn intersect_prim(s: &Scene, i: u32, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32) -> Vec4 {
    let p = at(s.prims, i);
    let kind = p.info.x;
    if kind == 0 {
        let e1 = p.v1.xyz() - p.v0.xyz();
        let e2 = p.v2.xyz() - p.v0.xyz();
        let pv = rd.cross(e2);
        let det = e1.dot(pv);
        if det.abs() < 1e-12 {
            return Vec4::ZERO;
        }
        let inv = 1.0 / det;
        let sv = ro - p.v0.xyz();
        let b1 = sv.dot(pv) * inv;
        if b1 < 0.0 || b1 > 1.0 {
            return Vec4::ZERO;
        }
        let q = sv.cross(e1);
        let b2 = rd.dot(q) * inv;
        if b2 < 0.0 || b1 + b2 > 1.0 {
            return Vec4::ZERO;
        }
        let t = e2.dot(q) * inv;
        if t <= tmin || t >= tmax {
            return Vec4::ZERO;
        }
        Vec4::new(t, b1, b2, 1.0)
    } else if kind == 1 {
        let oc = ro - p.v0.xyz();
        let b = oc.dot(rd);
        let c = oc.dot(oc) - p.v0.w * p.v0.w;
        let disc = b * b - c;
        if disc < 0.0 {
            return Vec4::ZERO;
        }
        let sq = disc.sqrt();
        let mut t = -b - sq;
        if t <= tmin || t >= tmax {
            t = -b + sq;
        }
        if t <= tmin || t >= tmax {
            return Vec4::ZERO;
        }
        Vec4::new(t, 0.0, 0.0, 1.0)
    } else {
        if rd.y.abs() < 1e-12 {
            return Vec4::ZERO;
        }
        let t = (p.v0.x - ro.y) / rd.y;
        if t <= tmin || t >= tmax {
            return Vec4::ZERO;
        }
        Vec4::new(t, 0.0, 0.0, 1.0)
    }
}

/// The geometric and shading normals of a hit.
fn normals(s: &Scene, i: u32, ro: Vec3, rd: Vec3, h: Vec4) -> (Vec3, Vec3) {
    let p = at(s.prims, i);
    let kind = p.info.x;
    if kind == 0 {
        let g = (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).normalize();
        let mut n = g;
        if p.info.w == 1 {
            let i = p.n0.xyz() * (1.0 - h.y - h.z) + p.n1.xyz() * h.y + p.n2.xyz() * h.z;
            if i.dot(i) > 0.0 {
                n = i.normalize();
            }
        }
        (g, n)
    } else if kind == 1 {
        let n = (ro + rd * h.x - p.v0.xyz()) / p.v0.w;
        (n, n)
    } else {
        (Vec3::Y, Vec3::Y)
    }
}

fn box_hit(bmin: Vec3, bmax: Vec3, ro: Vec3, inv: Vec3, tmax: f32) -> bool {
    let t1 = (bmin - ro) * inv;
    let t2 = (bmax - ro) * inv;
    let near = t1.min(t2).max_element();
    let far = t1.max(t2).min_element();
    !(far < 0.0 || near > far || near > tmax)
}

/// The nearest hit beyond `tmin`, or with `any_hit`, whether anything lies before `tmax`.
fn trace(s: &Scene, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, any_hit: bool) -> Hit {
    let mut best = Hit { t: tmax, prim: NONE, ng: Vec3::ZERO, ns: Vec3::ZERO };
    let mut best_h = Vec4::ZERO;
    let np = s.params.counts.x;
    // Planes, after the bounded primitives.
    let mut k = 0;
    while k < s.params.counts.y {
        let h = intersect_prim(s, np + k, ro, rd, tmin, best.t);
        if h.w > 0.0 {
            best.t = h.x;
            best.prim = np + k;
            best_h = h;
            if any_hit {
                return best;
            }
        }
        k += 1;
    }
    if np > 0 {
        let inv = Vec3::ONE / rd;
        let mut stack = [0u32; STACK];
        let mut sp = 1usize;
        while sp > 0 {
            sp -= 1;
            let ni = stack[sp];
            let n = at(s.nodes, ni);
            if !box_hit(n.bmin, n.bmax, ro, inv, best.t) {
                continue;
            }
            if n.count > 0 {
                let mut j = n.offset;
                while j < n.offset + n.count {
                    let h = intersect_prim(s, j, ro, rd, tmin, best.t);
                    if h.w > 0.0 {
                        best.t = h.x;
                        best.prim = j;
                        best_h = h;
                        if any_hit {
                            return best;
                        }
                    }
                    j += 1;
                }
            } else if sp + 2 <= STACK {
                stack[sp] = n.offset;
                stack[sp + 1] = ni + 1;
                sp += 2;
            }
        }
    }
    if best.prim != NONE {
        let (g, n) = normals(s, best.prim, ro, rd, best_h);
        best.ng = g;
        best.ns = n;
    }
    best
}

fn occluded(s: &Scene, ro: Vec3, rd: Vec3, tmax: f32) -> bool {
    trace(s, ro, rd, 0.0, tmax, true).prim != NONE
}

// ---- Tables: emitters, the sky and aerial perspective. ----

/// The piece i with cdf[i] <= u < cdf[i + 1], of n starting at `off`.
fn search(s: &Scene, off: u32, n: u32, u: f32) -> u32 {
    let mut lo = 0;
    let mut hi = n;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if at(s.tables, off + mid) <= u {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn emitter_probability(s: &Scene, e: u32) -> f32 {
    let off = s.params.offs.y;
    at(s.tables, off + e + 1) - at(s.tables, off + e)
}

fn env_texel(s: &Scene, x: u32, y: u32) -> Vec3 {
    let w = s.params.offs2.y;
    let i = (s.params.offs.z + 3 * (y * w + x % w)) as usize;
    Vec3::new(at(s.tables, i as u32), at(s.tables, (i + 1) as u32), at(s.tables, (i + 2) as u32))
}

fn lerp3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

/// The baked sky in direction d, bilinear (around in azimuth, clamped at the poles).
fn env_lookup(s: &Scene, d: Vec3) -> Vec3 {
    let (w, h) = (s.params.offs2.y, s.params.offs2.z);
    let u = fract(d.z.atan2(d.x) / (2.0 * PI) + 0.5) * w as f32 - 0.5;
    let v = (d.y.clamp(-1.0, 1.0).acos() / PI * h as f32 - 0.5).clamp(0.0, (h - 1) as f32);
    let x0 = u.floor();
    let y0 = v.floor();
    let (fx, fy) = (u - x0, v - y0);
    let xi = ((x0 as i32 + w as i32) as u32) % w;
    let yi = y0 as u32;
    let y1 = (yi + 1).min(h - 1);
    let top = lerp3(env_texel(s, xi, yi), env_texel(s, xi + 1, yi), fx);
    let bottom = lerp3(env_texel(s, xi, y1), env_texel(s, xi + 1, y1), fx);
    lerp3(top, bottom, fy)
}

fn env_pdf(s: &Scene, d: Vec3) -> f32 {
    let (w, h) = (s.params.offs2.y, s.params.offs2.z);
    let u = fract(d.z.atan2(d.x) / (2.0 * PI) + 0.5);
    let v = d.y.clamp(-1.0, 1.0).acos() / PI;
    let x = ((u * w as f32) as u32).min(w - 1);
    let y = ((v * h as f32) as u32).min(h - 1);
    let sin_theta = (v * PI).sin();
    if sin_theta <= 0.0 {
        return 0.0;
    }
    let marg = s.params.offs.w;
    let cond = s.params.offs2.x + y * (w + 1);
    let p_row = (at(s.tables, marg + y + 1) - at(s.tables, marg + y)) * h as f32;
    let p_col = (at(s.tables, cond + x + 1) - at(s.tables, cond + x)) * w as f32;
    p_row * p_col / (2.0 * PI * PI * sin_theta)
}

/// A direction from the sky, in proportion to its brightness, and its density.
fn env_sample(s: &Scene, u1: f32, u2: f32) -> Vec4 {
    let (w, h) = (s.params.offs2.y, s.params.offs2.z);
    let y = search(s, s.params.offs.w, h, u1);
    let x = search(s, s.params.offs2.x + y * (w + 1), w, u2);
    let u = (x as f32 + 0.5) / w as f32;
    let v = (y as f32 + 0.5) / h as f32;
    let phi = (u - 0.5) * 2.0 * PI;
    let theta = v * PI;
    let d = Vec3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());
    d.extend(env_pdf(s, d))
}

fn ap_entry(s: &Scene, a: u32, e: u32, k: u32) -> (Vec3, Vec3) {
    let i = (s.params.seed.z + 6 * ((a * AP_ELEVATION + e) * AP_DISTANCE + k)) as usize;
    (Vec3::new(at(s.tables, i as u32), at(s.tables, (i + 1) as u32), at(s.tables, (i + 2) as u32)), Vec3::new(at(s.tables, (i + 3) as u32), at(s.tables, (i + 4) as u32), at(s.tables, (i + 5) as u32)))
}

/// Transmittance and inscattered light along d to distance t (trilinear in the table).
fn aerial(s: &Scene, d: Vec3, t: f32) -> (Vec3, Vec3) {
    let x = d.dot(s.params.ap_x.xyz());
    let z = d.dot(s.params.ap_z.xyz());
    let fa = (z.atan2(x).abs() / PI).clamp(0.0, 1.0) * (AP_AZIMUTH - 1) as f32;
    let fe = ((d.y + 1.0) / 2.0).clamp(0.0, 1.0) * (AP_ELEVATION - 1) as f32;
    let fk = ((t / AERIAL_FROM).ln() / (AERIAL_TO / AERIAL_FROM).ln()).clamp(0.0, 1.0) * (AP_DISTANCE - 1) as f32;
    let a0 = (fa as u32).min(AP_AZIMUTH - 2);
    let e0 = (fe as u32).min(AP_ELEVATION - 2);
    let k0 = (fk as u32).min(AP_DISTANCE - 2);
    let (wa, we, wk) = (fa - a0 as f32, fe - e0 as f32, fk - k0 as f32);
    let mut tr = Vec3::ZERO;
    let mut ins = Vec3::ZERO;
    let mut c = 0u32;
    while c < 8 {
        let (da, de, dk) = (c & 1, (c >> 1) & 1, (c >> 2) & 1);
        let w = if da == 1 { wa } else { 1.0 - wa } * if de == 1 { we } else { 1.0 - we } * if dk == 1 { wk } else { 1.0 - wk };
        let (vt, vi) = ap_entry(s, a0 + da, e0 + de, k0 + dk);
        tr += vt * w;
        ins += vi * w;
        c += 1;
    }
    (tr, ins)
}

// ---- Scattering. ----

fn reflect_about(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * n.dot(v)
}

fn schlick(cosine: f32, ior: f32) -> f32 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

/// `functions::fresnel_reflectance`.
pub fn fresnel_reflectance(cos_i: f32, ni_over_nt: f32, ior: f32) -> f32 {
    let sin2_t = ni_over_nt * ni_over_nt * (1.0 - cos_i * cos_i);
    if sin2_t >= 1.0 {
        return 1.0;
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    schlick(cos_i.min(cos_t), ior)
}

fn cmul(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn norm2(a: (f32, f32)) -> f32 {
    a.0 * a.0 + a.1 * a.1
}

/// `ggx::conductor_reflectance`, complex arithmetic spelled out.
pub fn conductor_reflectance(cos_in: f32, eta: f32, k: f32) -> f32 {
    let cos_i = cos_in.clamp(0.0, 1.0);
    let sin2_i = 1.0 - cos_i * cos_i;
    let n2 = cmul((eta, k), (eta, k));
    let d = norm2(n2);
    // sin²t = sin²i / n², cos t = √(1 - sin²t), principal root.
    let z = (1.0 - sin2_i * n2.0 / d, sin2_i * n2.1 / d);
    let r = norm2(z).sqrt();
    let mut ct = (((r + z.0) / 2.0).max(0.0).sqrt(), ((r - z.0) / 2.0).max(0.0).sqrt());
    if z.1 < 0.0 {
        ct.1 = -ct.1;
    }
    let a = (eta * cos_i, k * cos_i);
    let nct = cmul((eta, k), ct);
    let rp = norm2((a.0 - ct.0, a.1 - ct.1)) / norm2((a.0 + ct.0, a.1 + ct.1)).max(1e-30);
    let rs = norm2((cos_i - nct.0, -nct.1)) / norm2((cos_i + nct.0, nct.1)).max(1e-30);
    (rp + rs) / 2.0
}

/// A BSDF at a hit: kind 0 Lambert, 1 GGX, 2 plastic (GGX coat over Lambert); Fresnel 0
/// Schlick (`albedo` is F0), 1 dielectric (`ior`), 2 conductor (`albedo` eta, `k`).
#[derive(Copy, Clone)]
pub struct Lobe {
    pub kind: u32,
    pub fresnel: u32,
    pub alpha: f32,
    pub ior: f32,
    pub albedo: Vec3,
    pub k: Vec3,
    pub base: Vec3,
    pub n: Vec3,
    pub t: Vec3,
    pub b: Vec3,
}

fn basis(n: Vec3) -> (Vec3, Vec3) {
    // Duff et al.'s orthonormal basis.
    let s = if n.z >= 0.0 { 1.0 } else { -1.0 };
    let a = -1.0 / (s + n.z);
    let b = n.x * n.y * a;
    (Vec3::new(1.0 + s * n.x * n.x * a, s * b, -s * n.x), Vec3::new(b, s + n.y * n.y * a, -n.y))
}

impl Lobe {
    pub fn new(kind: u32, n: Vec3) -> Lobe {
        let (t, b) = basis(n);
        Lobe { kind, fresnel: 0, alpha: 1.0, ior: 1.5, albedo: Vec3::ZERO, k: Vec3::ZERO, base: Vec3::ZERO, n, t, b }
    }

    fn local(&self, v: Vec3) -> Vec3 {
        Vec3::new(v.dot(self.t), v.dot(self.b), v.dot(self.n))
    }

    fn world(&self, v: Vec3) -> Vec3 {
        self.t * v.x + self.b * v.y + self.n * v.z
    }

    fn fresnel(&self, cos: f32) -> Vec3 {
        if self.fresnel == 0 {
            let k = (1.0 - cos).max(0.0).powi(5);
            self.albedo * (1.0 - k) + Vec3::splat(k)
        } else if self.fresnel == 1 {
            Vec3::splat(fresnel_reflectance(cos, 1.0 / self.ior, self.ior))
        } else {
            Vec3::new(conductor_reflectance(cos, self.albedo.x, self.k.x), conductor_reflectance(cos, self.albedo.y, self.k.y),
                      conductor_reflectance(cos, self.albedo.z, self.k.z))
        }
    }

    fn ggx_d(&self, m: Vec3) -> f32 {
        if m.z <= 0.0 {
            return 0.0;
        }
        let a2 = self.alpha * self.alpha;
        let k = (a2 - 1.0) * m.z * m.z + 1.0;
        a2 / (PI * k * k)
    }

    fn ggx_lambda(&self, w: Vec3) -> f32 {
        let cos2 = w.z * w.z;
        if cos2 >= 1.0 {
            return 0.0;
        }
        let a2tan2 = self.alpha * self.alpha * (w.x * w.x + w.y * w.y) / cos2;
        ((1.0 + a2tan2).sqrt() - 1.0) / 2.0
    }

    fn ggx_eval(&self, wo: Vec3, wi: Vec3) -> Vec3 {
        let (o, i) = (self.local(wo), self.local(wi));
        if o.z <= 0.0 || i.z <= 0.0 {
            return Vec3::ZERO;
        }
        let h = (o + i).normalize();
        let g2 = 1.0 / (1.0 + self.ggx_lambda(o) + self.ggx_lambda(i));
        self.fresnel(o.dot(h)) * (g2 * self.ggx_d(h) / (4.0 * o.z))
    }

    fn ggx_pdf(&self, wo: Vec3, wi: Vec3) -> f32 {
        let (o, i) = (self.local(wo), self.local(wi));
        if o.z <= 0.0 || i.z <= 0.0 {
            return 0.0;
        }
        let h = (o + i).normalize();
        self.ggx_d(h) / (1.0 + self.ggx_lambda(o)) / (4.0 * o.z)
    }

    /// Heitz's sampling of visible normals (`GGX::sample_visible_normal`); zero if none.
    fn ggx_sample(&self, wo: Vec3, u1: f32, u2: f32) -> Vec3 {
        let o = self.local(wo);
        if o.z <= 0.0 {
            return Vec3::ZERO;
        }
        let vh = Vec3::new(self.alpha * o.x, self.alpha * o.y, o.z).normalize();
        let lensq = vh.x * vh.x + vh.y * vh.y;
        let t1 = if lensq > 0.0 { Vec3::new(-vh.y, vh.x, 0.0) / lensq.sqrt() } else { Vec3::X };
        let t2 = vh.cross(t1);
        let r = u1.sqrt();
        let phi = 2.0 * PI * u2;
        let p1 = r * phi.cos();
        let sv = 0.5 * (1.0 + vh.z);
        let p2 = (1.0 - sv) * (1.0 - p1 * p1).max(0.0).sqrt() + sv * r * phi.sin();
        let nh = t1 * p1 + t2 * p2 + vh * (1.0 - p1 * p1 - p2 * p2).max(0.0).sqrt();
        let m = Vec3::new(self.alpha * nh.x, self.alpha * nh.y, nh.z.max(0.0)).normalize();
        let i = m * (2.0 * o.dot(m)) - o;
        if i.z <= 0.0 {
            return Vec3::ZERO;
        }
        self.world(i)
    }

    fn lambert_eval(&self, albedo: Vec3, wi: Vec3) -> Vec3 {
        let c = wi.dot(self.n);
        if c <= 0.0 { Vec3::ZERO } else { albedo * (c / PI) }
    }

    fn coat_share(&self, wo: Vec3) -> f32 {
        fresnel_reflectance(wo.dot(self.n).max(0.0), 1.0 / self.ior, self.ior)
    }

    /// f |cos θ_i|.
    pub fn eval(&self, wo: Vec3, wi: Vec3) -> Vec3 {
        if self.kind == 0 {
            self.lambert_eval(self.albedo, wi)
        } else if self.kind == 1 {
            self.ggx_eval(wo, wi)
        } else {
            // The coat, plus the base dimmed by what the coat reflects.
            self.ggx_eval(wo, wi) + self.lambert_eval(self.base, wi) * (1.0 - self.coat_share(wo))
        }
    }

    pub fn pdf(&self, wo: Vec3, wi: Vec3) -> f32 {
        let cos_pdf = wi.dot(self.n).max(0.0) / PI;
        if self.kind == 0 {
            cos_pdf
        } else if self.kind == 1 {
            self.ggx_pdf(wo, wi)
        } else {
            let f = self.coat_share(wo);
            f * self.ggx_pdf(wo, wi) + (1.0 - f) * cos_pdf
        }
    }

    /// A direction and its density (w = 0 if none).
    fn sample(&self, wo: Vec3, u1: f32, u2: f32, u3: f32) -> Vec4 {
        let wi = if self.kind == 0 || (self.kind == 2 && u3 >= self.coat_share(wo)) {
            cosine_sample(self.n, u1, u2)
        } else {
            self.ggx_sample(wo, u1, u2)
        };
        if wi.dot(wi) == 0.0 {
            return Vec4::ZERO;
        }
        wi.extend(self.pdf(wo, wi))
    }
}

fn uniform_sphere(u: f32, v: f32) -> Vec3 {
    let theta = u * 2.0 * PI;
    let z = 2.0 * v - 1.0;
    let sn = (1.0 - z * z).max(0.0).sqrt();
    Vec3::new(sn * theta.cos(), sn * theta.sin(), z)
}

fn cosine_sample(n: Vec3, u: f32, v: f32) -> Vec3 {
    let d = n + uniform_sphere(u, v);
    if d.length() < 1e-6 { n } else { d.normalize() }
}

pub fn power_heuristic(p: f32, q: f32) -> f32 {
    if p >= INF {
        return 1.0;
    }
    let (p2, q2) = (p * p, q * q);
    if p2 + q2 == 0.0 { 0.0 } else { p2 / (p2 + q2) }
}

fn resolve_material(s: &Scene, m_in: u32, p: Vec3) -> u32 {
    let mut m = m_in;
    let mut k = 0;
    while k < 4 {
        let mat = at(s.materials, m);
        if mat.kind != 6 {
            break;
        }
        let even_x = ((p.x.abs() / mat.p.z) as u32) % 2 == 0;
        let even_z = ((p.z.abs() / mat.p.w) as u32) % 2 == 0;
        let zig = if even_x { p.x > 0.0 } else { p.x <= 0.0 };
        let zag = if even_z { p.z > 0.0 } else { p.z <= 0.0 };
        m = if zig != zag { mat.a } else { mat.b };
        k += 1;
    }
    m
}

fn emitted(s: &Scene, m: u32, n: Vec3, towards: Vec3) -> Vec3 {
    let mat = at(s.materials, m);
    if mat.kind == 4 && (mat.a == 1 || towards.dot(n) > 0.0) { mat.c0.xyz() } else { Vec3::ZERO }
}

fn prim_area(s: &Scene, i: u32) -> f32 {
    let p = at(s.prims, i);
    if p.info.x == 0 {
        (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).length() / 2.0
    } else {
        4.0 * PI * p.v0.w * p.v0.w
    }
}

pub fn sphere_pdf(c: Vec3, r: f32, org: Vec3, y: Vec3, n: Vec3) -> f32 {
    let d2 = (c - org).dot(c - org);
    let r2 = r * r;
    if d2 <= r2 * 1.000001 {
        let w = y - org;
        let dist2 = w.dot(w);
        let cos = n.dot(w).abs() / dist2.sqrt();
        return dist2 / (cos * 4.0 * PI * r2);
    }
    let cos_max = (1.0 - r2 / d2).max(0.0).sqrt();
    1.0 / (2.0 * PI * (1.0 - cos_max))
}

/// A point on emitting primitive i seen from `org`, its normal, and the density per solid
/// angle: a sphere by its cone, a triangle by area.
fn sample_light(s: &Scene, i: u32, org: Vec3, u1: f32, u2: f32) -> (Vec3, Vec3, f32) {
    let p = at(s.prims, i);
    if p.info.x == 1 {
        let (c, r) = (p.v0.xyz(), p.v0.w);
        let to = c - org;
        let d2 = to.dot(to);
        if d2 <= r * r * 1.000001 {
            let n = uniform_sphere(u1, u2);
            let y = c + n * r;
            return (y, n, sphere_pdf(c, r, org, y, n));
        }
        let w = to / d2.sqrt();
        let cos_max = (1.0 - r * r / d2).max(0.0).sqrt();
        let cos_t = 1.0 - u1 * (1.0 - cos_max);
        let sin_t = (1.0 - cos_t * cos_t).max(0.0).sqrt();
        let phi = 2.0 * PI * u2;
        let (t, b) = basis(w);
        let dir = t * (sin_t * phi.cos()) + b * (sin_t * phi.sin()) + w * cos_t;
        // Where that direction meets the sphere, nearest first.
        let bb = (org - c).dot(dir);
        let disc = (bb * bb - (d2 - r * r)).max(0.0);
        let y = org + dir * (-bb - disc.sqrt());
        return (y, (y - c).normalize(), 1.0 / (2.0 * PI * (1.0 - cos_max)));
    }
    let su = u1.sqrt();
    let (b1, b2) = (1.0 - su, u2 * su);
    let y = p.v0.xyz() * b1 + p.v1.xyz() * b2 + p.v2.xyz() * (1.0 - b1 - b2);
    let n = (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).normalize();
    (y, n, light_pdf(s, i, org, y, n))
}

fn light_pdf(s: &Scene, i: u32, org: Vec3, y: Vec3, n: Vec3) -> f32 {
    let p = at(s.prims, i);
    if p.info.x == 1 {
        return sphere_pdf(p.v0.xyz(), p.v0.w, org, y, n);
    }
    let w = y - org;
    let d2 = w.dot(w);
    let cos = n.dot(w).abs() / d2.sqrt();
    if cos <= 0.0 { 0.0 } else { d2 / (cos * prim_area(s, i)) }
}

fn sun_disc(s: &Scene, d: Vec3) -> Vec3 {
    let cos = d.dot(s.params.sun_dir.xyz());
    let cos_r = s.params.sun_dir.w;
    if cos <= cos_r {
        return Vec3::ZERO;
    }
    let theta = cos.min(1.0).acos();
    let mu = (1.0 - (theta / s.params.sun_irr.w).powi(2)).max(0.0).sqrt();
    let solid_angle = 2.0 * PI * (1.0 - cos_r);
    s.params.sun_irr.xyz() * ((1.0 - LIMB_DARKENING * (1.0 - mu)) / (1.0 - LIMB_DARKENING / 3.0) / solid_angle)
}

fn sun_disc_pdf(s: &Scene, d: Vec3) -> f32 {
    if d.dot(s.params.sun_dir.xyz()) <= s.params.sun_dir.w { 0.0 } else { 1.0 / (2.0 * PI * (1.0 - s.params.sun_dir.w)) }
}

/// A spot light's falloff towards `towards` (pbrt's smoothstep); one for a point light.
fn spot(light: &Light, towards: Vec3) -> f32 {
    if light.pos.w < 0.5 {
        return 1.0;
    }
    let (a, b) = (light.extra.x, light.axis.w);
    let x = light.axis.xyz().dot(towards);
    if a == b {
        return if x < a { 0.0 } else { 1.0 };
    }
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn offset_for(s: &Scene, p: Vec3) -> f32 {
    s.params.misc.z.max(2e-5 * (1.0 + p.abs().max_element())) * 2.0
}

/// Whether a path survives a bounce of weight `attenuate` (Russian roulette, as
/// `trace::continuation_weight`); a survivor's throughput is divided by its chance.
fn survives(s: &Scene, depth: u32, attenuate: Vec3, beta: &mut Vec3, rng: &mut u32) -> bool {
    if attenuate.length() <= s.params.misc.y {
        return false;
    }
    if s.params.seed.y == 1 && depth as f32 >= s.params.misc.w {
        let q = attenuate.max_element().min(1.0);
        if rand(rng) >= q {
            return false;
        }
        *beta /= q;
    }
    true
}

/// The radiance along one camera ray.
pub fn radiance(s: &Scene, ro_in: Vec3, rd_in: Vec3, rng: &mut u32) -> Vec3 {
    let mut ro = ro_in;
    let mut rd = rd_in.normalize();
    let mut beta = Vec3::ONE;
    let mut l = Vec3::ZERO;
    let mut specular = true;
    let mut prev_pdf = 0.0;
    let mut scattered = false;
    let max_depth = s.params.dims.w;
    let flags = s.params.offs2.w;
    let n_emitters = s.params.counts.z;
    let mut depth = 0u32;
    loop {
        let hit = trace(s, ro, rd, 0.0, INF, false);
        if hit.prim == NONE {
            // The sky, weighted against sampling it if the last vertex did.
            let mut w = 1.0;
            if !specular && flags & FLAG_SAMPLE_ENV != 0 {
                w = power_heuristic(prev_pdf, env_pdf(s, rd));
            }
            l += beta * env_lookup(s, rd) * w;
            if flags & FLAG_SUN != 0 {
                let ws = if specular { 1.0 } else { power_heuristic(prev_pdf, sun_disc_pdf(s, rd)) };
                l += beta * sun_disc(s, rd) * ws;
            }
            break;
        }
        // Aerial perspective: the air before the hit attenuates what lies beyond it, and adds
        // its own light.
        if flags & FLAG_AERIAL != 0 && hit.t > AERIAL_FROM {
            let (tr, ins) = aerial(s, rd, hit.t);
            l += beta * ins;
            beta *= tr;
        }
        let point = ro + rd * hit.t;
        let prim = at(s.prims, hit.prim);
        let m = resolve_material(s, prim.info.y, point);
        let mat = at(s.materials, m);
        if mat.kind == 4 {
            let le = emitted(s, m, hit.ng, -rd);
            let mut w = 1.0;
            if !specular && flags & FLAG_SAMPLE_LIGHTS != 0 && prim.info.z > 0 {
                let lp = emitter_probability(s, prim.info.z - 1) * light_pdf(s, hit.prim, ro, point, hit.ng);
                w = power_heuristic(prev_pdf, lp);
            }
            l += beta * le * w;
            break;
        }
        let wo = -rd;
        let ng = if hit.ng.dot(wo) < 0.0 { -hit.ng } else { hit.ng };
        let ns = if hit.ns.dot(wo) < 0.0 { -hit.ns } else { hit.ns };
        let eps = offset_for(s, point);
        let above = point + ng * eps;
        let floor_alpha = if scattered { s.params.misc.x } else { 0.0 };

        // Delta bounces: mirrors, glass, smooth coats.
        let mut delta = false;
        let mut next_dir = Vec3::ZERO;
        let mut next_from = above;
        let mut attenuate = Vec3::ONE;
        let mut lobe = Lobe::new(0, ns);
        if mat.kind == 0 {
            lobe.albedo = mat.c0.xyz();
        } else if mat.kind == 1 {
            let alpha = (mat.p.y * mat.p.y).max(floor_alpha);
            if alpha >= MIN_ALPHA {
                lobe.kind = 2;
                lobe.fresnel = 1;
                lobe.alpha = alpha;
                lobe.ior = mat.p.x;
                lobe.base = mat.c0.xyz();
            } else if rand(rng) < fresnel_reflectance(wo.dot(ns).abs(), 1.0 / mat.p.x, mat.p.x) {
                delta = true;
                next_dir = reflect_about(rd, ns);
            } else {
                lobe.albedo = mat.c0.xyz();
            }
        } else if mat.kind == 3 || mat.kind == 5 {
            let alpha = (mat.p.y * mat.p.y).max(floor_alpha);
            lobe.kind = 1;
            lobe.alpha = alpha;
            lobe.albedo = mat.c0.xyz();
            lobe.k = mat.c1.xyz();
            lobe.fresnel = if mat.kind == 5 { 0 } else { 2 };
            if alpha < MIN_ALPHA {
                delta = true;
                next_dir = reflect_about(rd, ns);
                attenuate = lobe.fresnel(wo.dot(ns).abs());
            }
        } else if mat.kind == 2 {
            // As `functions::scatter_dielectric`.
            let n = hit.ns;
            let exiting = rd.dot(n) > 0.0;
            let outward = if exiting { -n } else { n };
            let ior = mat.p.x;
            let ni_over_nt = if exiting { ior } else { 1.0 / ior };
            let r = fresnel_reflectance(rd.dot(n).abs(), ni_over_nt, ior);
            let dt = rd.dot(outward);
            let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
            delta = true;
            if disc > 0.0 && rand(rng) >= r {
                next_dir = ((rd - outward * dt) * ni_over_nt - outward * disc.sqrt()).normalize();
                next_from = point - outward * eps;
                attenuate = mat.c0.xyz();
            } else {
                next_dir = reflect_about(rd, n);
                next_from = point + outward * eps;
            }
        }

        if delta {
            if depth >= max_depth || !survives(s, depth, attenuate, &mut beta, rng) {
                break;
            }
            beta *= attenuate;
            specular = true;
            ro = next_from;
            rd = next_dir.normalize();
            depth += 1;
            continue;
        }

        scattered = true;
        // Every point light.
        let mut li = 0;
        while li < s.params.counts.w {
            let light = at(s.lights, li);
            let to = light.pos.xyz() - above;
            let d2 = to.dot(to);
            let wi = to / d2.sqrt();
            let f = lobe.eval(wo, wi);
            if f.dot(f) > 0.0 {
                let i = light.intensity.xyz() * spot(&light, -wi);
                if i.dot(i) > 0.0 && !occluded(s, above, wi, d2.sqrt()) {
                    l += beta * f * i / d2;
                }
            }
            li += 1;
        }
        // The sun: a point for lobes that can't resolve its disc, else its disc with MIS.
        if flags & FLAG_SUN != 0 {
            let sd = s.params.sun_dir.xyz();
            let solid_angle = 2.0 * PI * (1.0 - s.params.sun_dir.w);
            if lobe.pdf(wo, sd) * solid_angle < 1e-3 {
                let f = lobe.eval(wo, sd);
                if f.dot(f) > 0.0 && !occluded(s, above, sd, 1e30) {
                    l += beta * f * s.params.sun_irr.xyz();
                }
            } else {
                let cos_t = 1.0 - rand(rng) * (1.0 - s.params.sun_dir.w);
                let sin_t = (1.0 - cos_t * cos_t).max(0.0).sqrt();
                let phi = 2.0 * PI * rand(rng);
                let (t, b) = basis(sd);
                let wi = t * (sin_t * phi.cos()) + b * (sin_t * phi.sin()) + sd * cos_t;
                let pdf = sun_disc_pdf(s, wi);
                let f = lobe.eval(wo, wi);
                if pdf > 0.0 && f.dot(f) > 0.0 && !occluded(s, above, wi, 1e30) {
                    l += beta * f * sun_disc(s, wi) * (power_heuristic(pdf, lobe.pdf(wo, wi)) / pdf);
                }
            }
        }
        // One area light, by power.
        if flags & FLAG_SAMPLE_LIGHTS != 0 && n_emitters > 0 {
            let e = search(s, s.params.offs.y, n_emitters, rand(rng));
            let pmf = emitter_probability(s, e);
            let pi = at(s.tables, s.params.offs.x + e).to_bits();
            let (y, ny, pdf) = sample_light(s, pi, above, rand(rng), rand(rng));
            let to = y - above;
            let d = to.length();
            let wi = to / d;
            let le = emitted(s, at(s.prims, pi).info.y, ny, -wi);
            if pdf > 0.0 && le.dot(le) > 0.0 {
                let lp = pmf * pdf;
                let f = lobe.eval(wo, wi);
                if f.dot(f) > 0.0 && !occluded(s, above, wi, d * (1.0 - 1e-4) - eps) {
                    l += beta * f * le * (power_heuristic(lp, lobe.pdf(wo, wi)) / lp);
                }
            }
        }
        // The sky, by its brightness.
        if flags & FLAG_SAMPLE_ENV != 0 {
            let es = env_sample(s, rand(rng), rand(rng));
            if es.w > 0.0 {
                let d = es.xyz();
                let f = lobe.eval(wo, d);
                if f.dot(f) > 0.0 && !occluded(s, above, d, 1e30) {
                    l += beta * f * env_lookup(s, d) * (power_heuristic(es.w, lobe.pdf(wo, d)) / es.w);
                }
            }
        }
        // The next ray.
        if depth >= max_depth {
            break;
        }
        let smp = lobe.sample(wo, rand(rng), rand(rng), rand(rng));
        if smp.w <= 0.0 {
            break;
        }
        let wi = smp.xyz();
        let weight = lobe.eval(wo, wi) / smp.w;
        if !survives(s, depth, weight, &mut beta, rng) {
            break;
        }
        beta *= weight;
        prev_pdf = smp.w;
        specular = false;
        let side = if wi.dot(ng) < 0.0 { -ng } else { ng };
        ro = point + side * eps;
        rd = wi;
        depth += 1;
    }
    l
}

#[spirv(compute(threads(64)))]
pub fn main(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Prim],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] rays: &[CameraRay],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] accum: &mut [Vec4],
) {
    // Dispatched as rows of at most 65535 workgroups.
    let index = gid.x + gid.y * (65535 * 64);
    if index >= params.dims.x * params.dims.y {
        return;
    }
    let cam = rays[index as usize];
    if cam.d.w < 0.5 {
        return;
    }
    let s = Scene { params, prims, nodes, materials, lights, tables };
    let mut rng = path_seed(index, params.dims.z, params.seed.x);
    let c = radiance(&s, cam.o.xyz(), cam.d.xyz(), &mut rng) * cam.o.w;
    // A NaN (from some degenerate case) is dropped, as the CPU renderers drop theirs. (Not
    // `is_finite`: its infinite constant is invalid for naga, which wgpu translates through.)
    #[allow(clippy::eq_op)]
    let not_nan = c.x == c.x && c.y == c.y && c.z == c.z;
    if not_nan {
        accum[index as usize] += c.extend(1.0);
    }
}
