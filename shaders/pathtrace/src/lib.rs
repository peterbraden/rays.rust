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
    /// Seed, Russian roulette on.
    pub seed: UVec4,
    /// Records of the air in `tables` (`AIR_*`), or `NO_AIR`: the sky's (for rays that escape),
    /// the aerial perspective's, the sun's (its reddening); and space's (`SPACE_*`).
    pub air: UVec4,
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

// Constants the host shares (`gpu/mod.rs` uses these, so the two sides agree).

/// `Params::offs2.w`'s flags.
pub const FLAG_SAMPLE_LIGHTS: u32 = 1;
pub const FLAG_SAMPLE_ENV: u32 = 2;
pub const FLAG_SUN: u32 = 4;
pub const FLAG_AERIAL: u32 = 8;
/// `Prim::info.x`.
pub const PRIM_TRIANGLE: u32 = 0;
pub const PRIM_SPHERE: u32 = 1;
pub const PRIM_PLANE: u32 = 2;
/// `Material::kind`.
pub const MAT_DIFFUSE: u32 = 0;
pub const MAT_PLASTIC: u32 = 1;
pub const MAT_DIELECTRIC: u32 = 2;
pub const MAT_CONDUCTOR: u32 = 3;
pub const MAT_EMITTER: u32 = 4;
pub const MAT_METAL: u32 = 5;
pub const MAT_CHECKER: u32 = 6;
/// Hits nearer than this are not hazed (as the renderer's `AERIAL_PERSPECTIVE_FROM`).
pub const AERIAL_FROM: f32 = 100.0;
/// No record (`Params::air`), table (`AIR_TABLES`) or moon.
pub const NO_AIR: u32 = 0xffff_ffff;
/// A record of the air in `tables` (`gpu::air_record`, after `skysphere::AirExport`): these
/// offsets within it. The earth's centre; its radius and the atmosphere's; |c|² - r² for each,
/// found in f64 (`excess`); the sun's direction; the Rayleigh and Mie scale heights; the
/// brightness; the Rayleigh coefficients; Mie's coefficient and anisotropy; ozone's; the
/// altitudes of the sky-view tables; then the tables' offsets (as bits), `NO_AIR` if absent.
pub const AIR_CENTRE: u32 = 0;
pub const AIR_RADII: u32 = 3;
pub const AIR_EXCESS: u32 = 5;
pub const AIR_SUN: u32 = 7;
pub const AIR_HEIGHTS: u32 = 10;
pub const AIR_BRIGHTNESS: u32 = 12;
pub const AIR_RAYLEIGH: u32 = 13;
pub const AIR_MIE: u32 = 16;
pub const AIR_OZONE: u32 = 18;
pub const AIR_LUT_ALTITUDES: u32 = 21;
/// The sun's depth table, multiple scattering's, the sky-view tables, the moon's record.
pub const AIR_TABLES: u32 = 29;
pub const AIR_HEADER: u32 = 33;
/// As `skysphere`'s: the steps of a view ray and of a ray to the sun, the tables' sizes, and the
/// sky's calibration.
pub const SKY_STEPS: u32 = 32;
pub const SUN_STEPS: u32 = 16;
pub const SUN_TABLE_ALTITUDES: u32 = 64;
pub const SUN_TABLE_ANGLES: u32 = 256;
pub const MS_ALTITUDES: u32 = 32;
pub const MS_SUN_ANGLES: u32 = 32;
pub const LUT_SLICES: u32 = 8;
pub const LUT_WIDTH: u32 = 256;
pub const LUT_HEIGHT: u32 = 128;
pub const SKY_CALIBRATION: f32 = 0.38317;
/// A record of space in `tables` (`gpu::space_record`, after `procedural::space::Space`):
/// the galactic frame (centre, pole, east), brightness, threshold, the noise's rotation between
/// octaves (rows), then offsets and counts (as bits): the noise's permutation table, the
/// stars', the Milky Way's, the nebulae's and how many, the galaxies' and how many; `NO_AIR`
/// for those absent.
pub const SPACE_FRAME: u32 = 0;
pub const SPACE_BRIGHTNESS: u32 = 9;
pub const SPACE_THRESHOLD: u32 = 10;
pub const SPACE_ROTATION: u32 = 11;
pub const SPACE_TABLES: u32 = 20;
pub const SPACE_HEADER: u32 = 27;
/// The stars' record: σ, cells across a face, then offsets (as bits) of where each cell's stars
/// start (6 n² + 1 of them), the stars (direction, flux times colour) and each cell's total flux.
pub const STARS_HEADER: u32 = 5;
/// `MilkyWay::export`'s order: brightness, dust, disc colour, bulge colour, noise offsets.
pub const MILKY_SIZE: u32 = 14;
/// `Nebula::export`'s and `Galaxy::export`'s sizes.
pub const NEBULA_SIZE: u32 = 18;
pub const GALAXY_SIZE: u32 = 29;
/// As the renderer's `ggx::MIN_ALPHA`, `functions::POINT_SUN` and `light`'s limb darkening.
pub const MIN_ALPHA: f32 = 1e-3;
pub const POINT_SUN: f32 = 1e-3;
pub const LIMB_DARKENING: f32 = 0.6;
/// The BVH traversal's stack: the host refuses trees deeper than this (`Buffers::build`).
pub const STACK: usize = 100;
const PI: f32 = core::f32::consts::PI;
const INF: f32 = 3.0e38;
const NONE: u32 = 0xffff_ffff;

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
    if kind == PRIM_TRIANGLE {
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
    } else if kind == PRIM_SPHERE {
        let oc = ro - p.v0.xyz();
        let b = oc.dot(rd);
        // A ray from outside going away from the centre misses, decided without the roots: a
        // shadow ray leaving a sphere otherwise re-hits it where sqrt is a few ULP out
        // (WebGPU allows that; SwiftShader's is), darkening every sphere.
        if b > 0.0 && oc.dot(oc) > p.v0.w * p.v0.w {
            return Vec4::ZERO;
        }
        // pbrt's discriminant, r² - |oc - b rd|², keeps a small, distant sphere's precision.
        let perp = oc - rd * b;
        let disc = p.v0.w * p.v0.w - perp.dot(perp);
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
    if kind == PRIM_TRIANGLE {
        let g = (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).normalize();
        let mut n = g;
        if p.info.w == 1 {
            let i = p.n0.xyz() * (1.0 - h.y - h.z) + p.n1.xyz() * h.y + p.n2.xyz() * h.z;
            if i.dot(i) > 0.0 {
                n = i.normalize();
            }
        }
        (g, n)
    } else if kind == PRIM_SPHERE {
        let n = (ro + rd * h.x - p.v0.xyz()) / p.v0.w;
        (n, n)
    } else {
        // A plane faces each ray, as the renderer's `Plane`.
        let n = if rd.y > 0.0 { -Vec3::Y } else { Vec3::Y };
        (n, n)
    }
}

/// The interval (near, far) narrowed by one axis's slab, entered and left at p and q (in
/// either order); unchanged if either is NaN.
fn slab(p: f32, q: f32, near: f32, far: f32) -> (f32, f32) {
    #[allow(clippy::eq_op)]
    if p == p && q == q { (near.max(p.min(q)), far.min(p.max(q))) } else { (near, far) }
}

fn box_hit(bmin: Vec3, bmax: Vec3, ro: Vec3, inv: Vec3, tmax: f32) -> bool {
    let t1 = (bmin - ro) * inv;
    let t2 = (bmax - ro) * inv;
    // Per axis; a ray parallel to a slab and starting on its face makes 0 × ∞ = NaN, and
    // limits nothing (as `bvh::TraversalRay::enters`). Unrolled: a component indexed by a
    // variable is a pointer chosen by branch, which logical SPIR-V can't have.
    let (n0, f0) = slab(t1.x, t2.x, -INF, INF);
    let (n1, f1) = slab(t1.y, t2.y, n0, f0);
    let (near, far) = slab(t1.z, t2.z, n1, f1);
    !(far < 0.0 || near > far || near > tmax)
}

/// The nearest hit beyond `tmin`, or with `any_hit`, whether anything lies before `tmax`.
fn trace(s: &Scene, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, any_hit: bool) -> Hit {
    let mut best = Hit { t: tmax, prim: NONE, ng: Vec3::ZERO, ns: Vec3::ZERO };
    let mut best_h = Vec4::ZERO;
    let np = s.params.counts.x;
    // One loop, and so one copy of `intersect_prim` once rust-gpu inlines everything: a leaf's
    // primitives j..end are tested first (to begin with, the planes, which follow the bounded
    // primitives), then the next node is taken from the stack.
    let mut j = np;
    let mut end = np + s.params.counts.y;
    let inv = Vec3::ONE / rd;
    let mut stack = [0u32; STACK];
    let mut sp = if np > 0 { 1usize } else { 0 };
    loop {
        if j < end {
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
            continue;
        }
        if sp == 0 {
            break;
        }
        sp -= 1;
        let ni = stack[sp];
        let n = at(s.nodes, ni);
        if !box_hit(n.bmin, n.bmax, ro, inv, best.t) {
            continue;
        }
        if n.count > 0 {
            j = n.offset;
            end = n.offset + n.count;
        } else if sp + 2 <= STACK {
            stack[sp] = n.offset;
            stack[sp + 1] = ni + 1;
            sp += 2;
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
    // A texel by the distributions, then a point uniformly within it (from what is left of
    // the uniform numbers), so the directions cover the sky, not only texel centres.
    let marg = s.params.offs.w;
    let y = search(s, marg, h, u1);
    let cond = s.params.offs2.x + y * (w + 1);
    let x = search(s, cond, w, u2);
    let within = |off: u32, i: u32, u: f32| {
        let (lo, hi) = (at(s.tables, off + i), at(s.tables, off + i + 1));
        if hi > lo { ((u - lo) / (hi - lo)).clamp(0.0, 0.99999) } else { 0.5 }
    };
    let u = (x as f32 + within(cond, x, u2)) / w as f32;
    let v = (y as f32 + within(marg, y, u1)) / h as f32;
    let phi = (u - 0.5) * 2.0 * PI;
    let theta = v * PI;
    let d = Vec3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());
    d.extend(env_pdf(s, d))
}

// ---- The atmosphere, per ray, as the renderer's `skysphere` (in f32). ----

/// A record of the air (`AIR_*`), read once.
#[derive(Copy, Clone)]
struct Air {
    centre: Vec3,
    earth: f32,
    top: f32,
    /// |c|² - r² for the earth and the atmosphere (`excess`).
    earth_k: f32,
    top_k: f32,
    sun: Vec3,
    rayleigh_height: f32,
    mie_height: f32,
    brightness: f32,
    rayleigh: Vec3,
    mie: f32,
    g: f32,
    ozone: Vec3,
    base: u32,
    sun_table: u32,
    multiple: u32,
    lut: u32,
    moon: u32,
}

fn tv3(s: &Scene, i: u32) -> Vec3 {
    Vec3::new(at(s.tables, i), at(s.tables, i + 1), at(s.tables, i + 2))
}

// (Closures that capture the scene's buffers are avoided here: naga rejects the struct of
// pointers rust-gpu can leave for them.)
fn load_air(s: &Scene, base: u32) -> Air {
    Air {
        centre: tv3(s, base + AIR_CENTRE), earth: at(s.tables, base + AIR_RADII), top: at(s.tables, base + AIR_RADII + 1),
        earth_k: at(s.tables, base + AIR_EXCESS), top_k: at(s.tables, base + AIR_EXCESS + 1), sun: tv3(s, base + AIR_SUN),
        rayleigh_height: at(s.tables, base + AIR_HEIGHTS), mie_height: at(s.tables, base + AIR_HEIGHTS + 1),
        brightness: at(s.tables, base + AIR_BRIGHTNESS),
        rayleigh: tv3(s, base + AIR_RAYLEIGH), mie: at(s.tables, base + AIR_MIE), g: at(s.tables, base + AIR_MIE + 1),
        ozone: tv3(s, base + AIR_OZONE),
        base,
        sun_table: at(s.tables, base + AIR_TABLES).to_bits(), multiple: at(s.tables, base + AIR_TABLES + 1).to_bits(),
        lut: at(s.tables, base + AIR_TABLES + 2).to_bits(), moon: at(s.tables, base + AIR_TABLES + 3).to_bits(),
    }
}

/// |p - c|² - r², for k = |c|² - r²: p·p - 2 p·c + k, which keeps its precision near the ground
/// at a planet's scale, where |p - c|² and r² are 4e13 and agree to six figures.
fn excess(p: Vec3, c: Vec3, k: f32) -> f32 {
    p.dot(p) - 2.0 * p.dot(c) + k
}

/// Height above the ground: (|p - c|² - r²) / (|p - c| + r).
fn altitude(a: &Air, p: Vec3) -> f32 {
    excess(p, a.centre, a.earth_k) / ((p - a.centre).length() + a.earth)
}

/// The roots of |o + t d|² = r² along unit d (nearer first), or None: in the stable form.
fn sphere_roots(o: Vec3, d: Vec3, c: Vec3, k: f32) -> Option<(f32, f32)> {
    let b = (o - c).dot(d);
    let cc = excess(o, c, k);
    let disc = b * b - cc;
    if disc < 0.0 {
        return None;
    }
    let q = -(b + if b < 0.0 { -disc.sqrt() } else { disc.sqrt() });
    if q == 0.0 {
        return Some((0.0, 0.0));
    }
    let (t0, t1) = (q, cc / q);
    Some(if t0 < t1 { (t0, t1) } else { (t1, t0) })
}

/// The distance to a sphere along unit d, as the renderer's `Sphere::intersects`: the nearer
/// root if not behind, else the farther; negative if none.
fn sphere_hit(o: Vec3, d: Vec3, c: Vec3, k: f32) -> f32 {
    match sphere_roots(o, d, c, k) {
        None => -1.0,
        Some((t0, t1)) => if t0 >= 0.0 { t0 } else { t1 },
    }
}

/// `SkyMaterial::view_ray_span`: where the ray is in the atmosphere, up to the ground.
fn view_ray_span(a: &Air, o: Vec3, d: Vec3) -> Option<(f32, f32)> {
    let (enter, leave) = sphere_roots(o, d, a.centre, a.top_k)?;
    if leave <= 0.0 {
        return None;
    }
    let start = enter.max(0.0);
    let mut end = leave;
    if excess(o, a.centre, a.earth_k) > 0.0 {
        let g = sphere_hit(o, d, a.centre, a.earth_k);
        if g >= 0.0 {
            end = end.min(g);
        }
    }
    if end <= start { None } else { Some((start, end)) }
}

/// Sample `k` of `path_samples(length, n)`: (distance, step).
fn path_sample(length: f32, n: u32, k: u32) -> (f32, f32) {
    let (lo, hi) = (k as f32 / n as f32, (k + 1) as f32 / n as f32);
    let u = (lo + hi) / 2.0;
    (length * u * u, length * (hi * hi - lo * lo))
}

/// Sample `k` of `SkyMaterial::span_samples(r, start, end, n)`: crowded about the ray's
/// closest approach to the planet.
fn span_sample(a: &Air, o: Vec3, d: Vec3, start: f32, end: f32, n: u32, k: u32) -> (f32, f32) {
    let focus = (-(o - a.centre).dot(d)).clamp(start, end);
    let (before, after) = (focus - start, end - focus);
    // Crowded towards b over [a, b], in order of distance; or away from a.
    let towards = |lo: f32, hi: f32, n: u32, k: u32| { let (t, step) = path_sample(hi - lo, n, n - 1 - k); (hi - t, step) };
    let away = |lo: f32, hi: f32, n: u32, k: u32| { let (t, step) = path_sample(hi - lo, n, k); (lo + t, step) };
    if before <= 1e-9 * (end - start) {
        away(start, end, n, k)
    } else if after <= 1e-9 * (end - start) {
        towards(start, end, n, k)
    } else {
        let n1 = ((n as f32 * before / (before + after)).round() as u32).clamp(1, n - 1);
        if k < n1 { towards(start, focus, n1, k) } else { away(focus, end, n - n1, k - n1) }
    }
}

/// Optical depths (Rayleigh, Mie, ozone) of a step at `height` (`SkyMaterial::depths`).
fn depths(a: &Air, height: f32, step: f32) -> Vec3 {
    let ozone = (1.0 - (height - 25_000.0).abs() / 15_000.0).max(0.0);
    Vec3::new((-height / a.rayleigh_height).exp() * step, (-height / a.mie_height).exp() * step, ozone * step)
}

fn transmittance(a: &Air, d: Vec3) -> Vec3 {
    let tau = a.rayleigh * d.x + Vec3::splat(a.mie * 1.1 * d.y) + a.ozone * d.z;
    Vec3::new((-tau.x).exp(), (-tau.y).exp(), (-tau.z).exp())
}

fn scattered(a: &Air, d: Vec3) -> Vec3 {
    a.rayleigh * d.x + Vec3::splat(a.mie * d.y)
}

/// `SkyMaterial::optical_depth_towards` the sun: None if the earth is in the way.
fn depths_to_sun(a: &Air, p: Vec3) -> Option<Vec3> {
    if sphere_hit(p, a.sun, a.centre, a.earth_k) >= 0.0 {
        return None;
    }
    let length = sphere_hit(p, a.sun, a.centre, a.top_k);
    if length < 0.0 {
        return None;
    }
    let mut total = Vec3::ZERO;
    let mut k = 0;
    while k < SUN_STEPS {
        let (t, step) = path_sample(length, SUN_STEPS, k);
        total += depths(a, altitude(a, p + a.sun * t), step);
        k += 1;
    }
    Some(total)
}

/// Bilinear in a table of `cols`-wide rows of 3 at `off`, at fractional row u and column v.
fn bilinear3(s: &Scene, off: u32, rows: u32, cols: u32, u: f32, v: f32) -> Vec3 {
    let (i, j) = ((u as u32).min(rows - 2), (v as u32).min(cols - 2));
    let (fu, fv) = (u - i as f32, v - j as f32);
    let (r0, r1) = (off + 3 * (i * cols + j), off + 3 * ((i + 1) * cols + j));
    (tv3(s, r0) * (1.0 - fv) + tv3(s, r0 + 3) * fv) * (1.0 - fu) + (tv3(s, r1) * (1.0 - fv) + tv3(s, r1 + 3) * fv) * fu
}

/// `MultipleScattering::lookup`'s Ψ at `p`, or none if it is off.
fn multiple_at(s: &Scene, a: &Air, p: Vec3) -> Vec3 {
    if a.multiple == NO_AIR {
        return Vec3::ZERO;
    }
    let mu = (p - a.centre).normalize().dot(a.sun);
    let u = (altitude(a, p).max(0.0) / (a.top - a.earth)).sqrt().min(1.0) * (MS_ALTITUDES - 1) as f32;
    let v = (mu.clamp(-1.0, 1.0) + 1.0) / 2.0 * (MS_SUN_ANGLES - 1) as f32;
    bilinear3(s, a.multiple, MS_ALTITUDES, MS_SUN_ANGLES, u, v)
}

/// `SkyMaterial::sun_depths_at`: from the table, at the point where the line to the sun enters
/// the air if `p` is above it (none if it never does).
fn sun_depths_at(s: &Scene, a: &Air, p_in: Vec3) -> Vec3 {
    let mut p = p_in;
    if excess(p, a.centre, a.top_k) > 0.0 {
        let o = p - a.centre;
        let b = o.dot(a.sun);
        let disc = b * b - excess(p, a.centre, a.top_k);
        if b >= 0.0 || disc <= 0.0 {
            return Vec3::ZERO;
        }
        p += a.sun * (-b - disc.sqrt());
    }
    let up = (p - a.centre).normalize();
    let u = (altitude(a, p).max(0.0) / (a.top - a.earth)).sqrt().min(1.0) * (SUN_TABLE_ALTITUDES - 1) as f32;
    let v = (up.dot(a.sun).clamp(-1.0, 1.0) + 1.0) / 2.0 * (SUN_TABLE_ANGLES - 1) as f32;
    bilinear3(s, a.sun_table, SUN_TABLE_ALTITUDES, SUN_TABLE_ANGLES, u, v)
}

fn rayleigh_phase(mu: f32) -> f32 {
    3.0 / (16.0 * PI) * (1.0 + mu * mu)
}

fn mie_phase(mu: f32, g: f32) -> f32 {
    let g2 = g * g;
    3.0 / (8.0 * PI) * ((1.0 - g2) * (1.0 + mu * mu)) / ((2.0 + g2) * (1.0 + g2 - 2.0 * g * mu).powf(1.5))
}

/// `SkyMaterial::apply_phase`.
fn apply_phase(a: &Air, d: Vec3, rayleigh: Vec3, mie: Vec3, multiple: Vec3) -> Vec3 {
    let mu = d.dot(a.sun);
    (rayleigh * rayleigh_phase(mu) + mie * mie_phase(mu, a.g) + multiple) * a.brightness
}

/// `Air::scattering`: Rayleigh, Mie and multiple scattering along the view ray, before the
/// phase functions.
fn scattering(s: &Scene, a: &Air, o: Vec3, d: Vec3) -> (Vec3, Vec3, Vec3) {
    let (start, end) = match view_ray_span(a, o, d) {
        Some(span) => span,
        None => return (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
    };
    let (mut rayleigh, mut mie, mut multiple) = (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
    let mut depth = Vec3::ZERO;
    let mut k = 0;
    while k < SKY_STEPS {
        let (t, step) = span_sample(a, o, d, start, end, SKY_STEPS, k);
        let p = o + d * t;
        let here = depths(a, altitude(a, p), step);
        depth += here * 0.5;
        multiple += transmittance(a, depth) * scattered(a, here) * multiple_at(s, a, p);
        if let Some(to_sun) = depths_to_sun(a, p) {
            let lit = transmittance(a, depth + to_sun);
            rayleigh += lit * here.x;
            mie += lit * here.y;
        }
        depth += here * 0.5;
        k += 1;
    }
    (rayleigh * a.rayleigh, mie * a.mie, multiple)
}

/// One of `SkyViewLUT`'s slices, bilinear (`lookup_slice`).
fn lut_slice(s: &Scene, a: &Air, slice: u32, d: Vec3) -> (Vec3, Vec3, Vec3) {
    let l = d.y.clamp(0.0, 1.0).asin();
    let v = (l / (PI / 2.0)).sqrt() * (LUT_HEIGHT - 1) as f32;
    let u = fract(d.z.atan2(d.x) / (2.0 * PI)) * LUT_WIDTH as f32;
    let x0 = (u.floor() as u32) % LUT_WIDTH;
    let y0 = (v.floor() as u32).min(LUT_HEIGHT - 2);
    let (x1, y1) = ((x0 + 1) % LUT_WIDTH, y0 + 1);
    let (fx, fy) = (u - u.floor(), v - y0 as f32);
    let base = a.lut + slice * LUT_WIDTH * LUT_HEIGHT * 9;
    let t = UVec4::new(base + 9 * (y0 * LUT_WIDTH + x0), base + 9 * (y0 * LUT_WIDTH + x1), base + 9 * (y1 * LUT_WIDTH + x0), base + 9 * (y1 * LUT_WIDTH + x1));
    let w = Vec4::new((1.0 - fx) * (1.0 - fy), fx * (1.0 - fy), (1.0 - fx) * fy, fx * fy);
    // Rayleigh, Mie and multiple scattering, each bilinear.
    (lut_texels(s, t, w, 0), lut_texels(s, t, w, 3), lut_texels(s, t, w, 6))
}

/// The four texels `t` (offsets) weighted by `w`, at `c` within each.
fn lut_texels(s: &Scene, t: UVec4, w: Vec4, c: u32) -> Vec3 {
    tv3(s, t.x + c) * w.x + tv3(s, t.y + c) * w.y + tv3(s, t.z + c) * w.z + tv3(s, t.w + c) * w.w
}

/// The altitude of sky-view table `i`.
fn lut_altitude(s: &Scene, a: &Air, i: u32) -> f32 {
    at(s.tables, a.base + AIR_LUT_ALTITUDES + i)
}

/// `SkyViewLUT::lookup`: linear between the two tables about `altitude`; None above the top.
fn lut_lookup(s: &Scene, a: &Air, d: Vec3, altitude: f32) -> Option<(Vec3, Vec3, Vec3)> {
    if altitude > lut_altitude(s, a, LUT_SLICES - 1) {
        return None;
    }
    let aa = altitude.max(lut_altitude(s, a, 0));
    // The first table above `aa` (`partition_point`), at least the second, at most the last.
    let mut i = 0;
    while i < LUT_SLICES && lut_altitude(s, a, i) <= aa {
        i += 1;
    }
    let i = i.clamp(1, LUT_SLICES - 1);
    let (h0, h1) = (lut_altitude(s, a, i - 1), lut_altitude(s, a, i));
    let f = (aa - h0) / (h1 - h0);
    let (r0, m0, s0) = lut_slice(s, a, i - 1, d);
    let (r1, m1, s1) = lut_slice(s, a, i, d);
    Some((r0 * (1.0 - f) + r1 * f, m0 * (1.0 - f) + m1 * f, s0 * (1.0 - f) + s1 * f))
}

/// The light the air sends along the ray from `o` towards unit `d`, as the renderer's
/// `LUTSky::air_radiance` (or `SkyMaterial`'s, without tables): looked up above the horizon
/// below the highest table, integrated otherwise, and the same for its moonlight.
pub fn air_radiance(s: &Scene, record: u32, o: Vec3, d: Vec3) -> Vec3 {
    let mut out = Vec3::ZERO;
    let mut r = record;
    // The sky, then its moonlit copy: one call site of the integration.
    let mut n = 0;
    while r != NO_AIR && n < 2 {
        let a = load_air(s, r);
        let looked_up = if d.y < 0.0 || a.lut == NO_AIR { None } else { lut_lookup(s, &a, d, altitude(&a, o)) };
        let (rayleigh, mie, multiple) = match looked_up {
            Some(v) => v,
            None => scattering(s, &a, o, d),
        };
        out += apply_phase(&a, d, rayleigh, mie, multiple) * SKY_CALIBRATION;
        r = a.moon;
        n += 1;
    }
    out
}

/// `Air::segment`: for light reaching `o` along unit `d` from `distance` away, the fraction
/// the air between lets through, and the light it scatters in on the way.
fn segment(s: &Scene, record: u32, o: Vec3, d: Vec3, distance: f32) -> (Vec3, Vec3) {
    let a = load_air(s, record);
    let (start, end) = match view_ray_span(&a, o, d) {
        Some((start, end)) if start < distance => (start, end.min(distance)),
        _ => return (Vec3::ONE, Vec3::ZERO),
    };
    let mut depth = Vec3::ZERO;
    let (mut rayleigh, mut mie, mut multiple) = (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
    let mut k = 0;
    while k < SKY_STEPS {
        let (t, step) = span_sample(&a, o, d, start, end, SKY_STEPS, k);
        let p = o + d * t;
        let here = depths(&a, altitude(&a, p), step);
        depth += here * 0.5;
        multiple += transmittance(&a, depth) * scattered(&a, here) * multiple_at(s, &a, p);
        let lit = transmittance(&a, depth + sun_depths_at(s, &a, p));
        rayleigh += lit * here.x;
        mie += lit * here.y;
        depth += here * 0.5;
        k += 1;
    }
    let inscattered = apply_phase(&a, d, rayleigh * a.rayleigh, mie * a.mie, multiple) * SKY_CALIBRATION;
    (transmittance(&a, depth), inscattered)
}

/// The fraction of sunlight reaching `p` through the sun's air (`SkyMaterial::sun_transmittance`);
/// all of it without one.
pub fn sun_transmittance(s: &Scene, p: Vec3) -> Vec3 {
    let record = s.params.air.z;
    if record == NO_AIR {
        return Vec3::ONE;
    }
    let a = load_air(s, record);
    transmittance(&a, sun_depths_at(s, &a, p))
}

/// `SkyMaterial::transmittance_to_space`: the fraction of light from beyond the atmosphere
/// that crosses it to `o` along unit `d`; none through the ground.
fn transmittance_to_space(s: &Scene, record: u32, o: Vec3, d: Vec3) -> Vec3 {
    let a = load_air(s, record);
    if excess(o, a.centre, a.earth_k) > 0.0 && sphere_hit(o, d, a.centre, a.earth_k) >= 0.0 {
        return Vec3::ZERO;
    }
    let (start, end) = match view_ray_span(&a, o, d) {
        Some(span) => span,
        None => return Vec3::ONE,
    };
    let mut depth = Vec3::ZERO;
    let mut k = 0;
    while k < SKY_STEPS {
        let (t, step) = span_sample(&a, o, d, start, end, SKY_STEPS, k);
        depth += depths(&a, altitude(&a, o + d * t), step);
        k += 1;
    }
    transmittance(&a, depth)
}

/// What reaches `o` along unit `d` from beyond the scene: the sky's air, with space showing
/// through it where it could add more than its threshold (`SkyMaterial::space_seen`); space
/// alone; or the background's map.
pub fn background(s: &Scene, o: Vec3, d: Vec3) -> Vec3 {
    let (sky, space) = (s.params.air.x, s.params.air.w);
    if sky == NO_AIR && space == NO_AIR {
        return env_lookup(s, d);
    }
    let mut v = Vec3::ZERO;
    let mut through = Vec3::ONE;
    let mut with_space = space != NO_AIR;
    if sky != NO_AIR {
        v = air_radiance(s, sky, o, d);
        if with_space {
            let threshold = at(s.tables, space + SPACE_THRESHOLD);
            if space_bound(s, space, d) <= threshold * luminance(v) {
                with_space = false;
            } else {
                through = transmittance_to_space(s, sky, o, d);
                with_space = through.max_element() > 0.0;
            }
        }
    }
    if with_space {
        v += space_radiance(s, space, d) * through;
    }
    v
}

// ---- Space, as `procedural::space` (in f32). ----

/// Where space's parts are (`SPACE_TABLES`): the stars', the Milky Way's, the nebulae's
/// and how many, the galaxies' and how many.
fn space_tables(s: &Scene, space: u32) -> (u32, u32, u32, u32, u32, u32) {
    let t = space + SPACE_TABLES;
    (at(s.tables, t + 1).to_bits(), at(s.tables, t + 2).to_bits(), at(s.tables, t + 3).to_bits(),
     at(s.tables, t + 4).to_bits(), at(s.tables, t + 5).to_bits(), at(s.tables, t + 6).to_bits())
}

fn luminance(c: Vec3) -> f32 {
    0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Galactic longitude and latitude of unit `d` (`GalacticFrame::coordinates`).
fn galactic(s: &Scene, space: u32, d: Vec3) -> (f32, f32) {
    let (centre, pole, east) = (tv3(s, space + SPACE_FRAME), tv3(s, space + SPACE_FRAME + 3), tv3(s, space + SPACE_FRAME + 6));
    let b = d.dot(pole).clamp(-1.0, 1.0).asin();
    (d.dot(east).atan2(d.dot(centre)), b)
}

/// `PerlinNoise::noise`, with the permutation table at `perm`.
fn perlin(s: &Scene, perm: u32, p: Vec3) -> f32 {
    let f = Vec3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let xi = ((f.x as i32) & 255) as u32;
    let yi = ((f.y as i32) & 255) as u32;
    let zi = ((f.z as i32) & 255) as u32;
    let (x, y, z) = (p.x - f.x, p.y - f.y, p.z - f.z);
    let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    let (u, v, w) = (fade(x), fade(y), fade(z));
    let a = perm_at(s, perm, xi) + yi;
    let aa = perm_at(s, perm, a) + zi;
    let ab = perm_at(s, perm, a + 1) + zi;
    let b = perm_at(s, perm, xi + 1) + yi;
    let ba = perm_at(s, perm, b) + zi;
    let bb = perm_at(s, perm, b + 1) + zi;
    let g1 = grad(perm_at(s, perm, aa), x, y, z);
    let g2 = grad(perm_at(s, perm, ba), x - 1.0, y, z);
    let g3 = grad(perm_at(s, perm, ab), x, y - 1.0, z);
    let g4 = grad(perm_at(s, perm, bb), x - 1.0, y - 1.0, z);
    let g5 = grad(perm_at(s, perm, aa + 1), x, y, z - 1.0);
    let g6 = grad(perm_at(s, perm, ba + 1), x - 1.0, y, z - 1.0);
    let g7 = grad(perm_at(s, perm, ab + 1), x, y - 1.0, z - 1.0);
    let g8 = grad(perm_at(s, perm, bb + 1), x - 1.0, y - 1.0, z - 1.0);
    let lerp = |a: f32, b: f32, t: f32| a + t * (b - a);
    let (l1, l2, l3, l4) = (lerp(g1, g2, u), lerp(g3, g4, u), lerp(g5, g6, u), lerp(g7, g8, u));
    lerp(lerp(l1, l2, v), lerp(l3, l4, v), w)
}

fn perm_at(s: &Scene, perm: u32, i: u32) -> u32 {
    at(s.tables, perm + i) as u32
}

/// `PerlinNoise::grad`: one of the 12 edge directions of a cube, by `hash` % 12, dotted with
/// (x, y, z).
fn grad(hash: u32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash % 12;
    let first = if h & 1 == 0 { 1.0 } else { -1.0 };
    let second = if h & 2 == 0 { 1.0 } else { -1.0 };
    if h < 4 {
        first * x + second * y
    } else if h < 8 {
        first * x + second * z
    } else {
        first * y + second * z
    }
}

/// `SeededNoise::fbm`: octaves of noise displaced by `offset`, each rotated, at twice the
/// frequency and half the amplitude.
fn fbm(s: &Scene, space: u32, offset: Vec3, p: Vec3, octaves: u32) -> f32 {
    let perm = at(s.tables, space + SPACE_TABLES).to_bits();
    let (r0, r1, r2) = (tv3(s, space + SPACE_ROTATION), tv3(s, space + SPACE_ROTATION + 3), tv3(s, space + SPACE_ROTATION + 6));
    let (mut sum, mut amplitude, mut total, mut q) = (0.0, 1.0, 0.0, p);
    let mut k = 0;
    while k < octaves {
        sum += perlin(s, perm, q + offset) * amplitude;
        total += amplitude;
        amplitude *= 0.5;
        q = Vec3::new(r0.dot(q), r1.dot(q), r2.dot(q)) * 2.03;
        k += 1;
    }
    if total > 0.0 { sum / total } else { 0.0 }
}

/// Where `warped_fbm` samples the three fields that displace its point: the `g`th.
fn warp_offset(g: u32) -> Vec3 {
    if g == 0 { Vec3::new(1.7, 9.2, 3.1) } else if g == 1 { Vec3::new(8.3, 2.8, 5.4) } else { Vec3::new(4.6, 7.1, 0.9) }
}

/// Coordinate `i` of `d` (no indexing by a variable, which logical SPIR-V can't do).
fn component(d: Vec3, i: u32) -> f32 {
    if i == 0 { d.x } else if i == 1 { d.y } else { d.z }
}

/// The cells of the stars that may show in unit `d` (`Stars::neighbourhood`), each passed to
/// `cell` as its index; `total` adds what it returns.
fn star_cells(s: &Scene, stars: u32, d: Vec3, want_bound: bool) -> Vec3 {
    let sigma = at(s.tables, stars);
    let n = at(s.tables, stars + 1).to_bits();
    let (starts, data, flux) = (at(s.tables, stars + 2).to_bits(), at(s.tables, stars + 3).to_bits(), at(s.tables, stars + 4).to_bits());
    let reach = 4.0 * sigma;
    let norm = 1.0 / (2.0 * PI * sigma * sigma);
    let mut sum = Vec3::ZERO;
    let mut face = 0;
    while face < 6 {
        let (axis, sign) = (face / 2, if face % 2 == 0 { 1.0 } else { -1.0 });
        let along = component(d, axis) * sign;
        let margin = 3.0 * reach;
        if along > 0.0 {
            let (u, v) = (component(d, (axis + 1) % 3) / along, component(d, (axis + 2) % 3) / along);
            if u.abs() <= 1.0 + margin && v.abs() <= 1.0 + margin {
                let ci = (((u + 1.0) / 2.0 * n as f32).floor() as i32).clamp(0, n as i32 - 1);
                let cj = (((v + 1.0) / 2.0 * n as f32).floor() as i32).clamp(0, n as i32 - 1);
                let mut i = (ci - 1).max(0);
                while i <= (ci + 1).min(n as i32 - 1) {
                    let mut j = (cj - 1).max(0);
                    while j <= (cj + 1).min(n as i32 - 1) {
                        let cell = (face * n + i as u32) * n + j as u32;
                        if want_bound {
                            sum.x += at(s.tables, flux + cell);
                        } else {
                            let (first, last) = (at(s.tables, starts + cell).to_bits(), at(s.tables, starts + cell + 1).to_bits());
                            let mut k = first;
                            while k < last {
                                let (dir, c) = (tv3(s, data + 6 * k), tv3(s, data + 6 * k + 3));
                                let r = d - dir;
                                let r2 = r.dot(r);
                                if r2 < reach * reach {
                                    sum += c * (norm * (-r2 / (2.0 * sigma * sigma)).exp());
                                }
                                k += 1;
                            }
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
        }
        face += 1;
    }
    if want_bound { Vec3::splat(sum.x * norm) } else { sum }
}

/// `milky_way::profile`: the disc's and the bulge's glow at (l, b).
fn milky_profile(l: f32, b: f32) -> (f32, f32) {
    let scale = 0.06 + 0.05 * l.abs() / PI;
    let disc = (-b.abs() / scale).exp() * (0.3 + 0.7 * (-(l / 1.4) * (l / 1.4)).exp());
    let bulge = 1.3 * (-(l * l + (b / 0.75) * (b / 0.75)) / (2.0 * 0.2 * 0.2)).exp();
    (disc, bulge)
}

/// A bound on space's luminance in unit `d` (`Space::bound`).
fn space_bound(s: &Scene, space: u32, d: Vec3) -> f32 {
    let (stars, milky, nebulae, n_nebulae, galaxies, n_galaxies) = space_tables(s, space);
    let (l, b) = galactic(s, space, d);
    let mut total = 0.0;
    if stars != NO_AIR {
        total += star_cells(s, stars, d, true).x;
    }
    if milky != NO_AIR {
        let (disc, bulge) = milky_profile(l, b);
        total += at(s.tables, milky) * (disc + bulge) * 4.73;
    }
    let mut k = 0;
    while k < n_nebulae {
        let r = nebulae + k * NEBULA_SIZE;
        if d.dot(tv3(s, r + 1)) >= at(s.tables, r + 5) {
            let kind = at(s.tables, r) as u32;
            let scale = if kind == 0 { 1.8 } else if kind == 1 { 1.0 } else if kind == 2 { 4.7 } else { 0.0 };
            total += at(s.tables, r + 6) * scale;
        }
        k += 1;
    }
    let mut k = 0;
    while k < n_galaxies {
        let r = galaxies + k * GALAXY_SIZE;
        if d.dot(tv3(s, r + 1)) >= at(s.tables, r + 5) {
            let spiral = at(s.tables, r) < 0.5;
            total += at(s.tables, r + 6) * if spiral { 1.0 + 4.0 * 0.12 * (1.0 + 1.8 * 2.2) } else { 1.0 };
        }
        k += 1;
    }
    total * at(s.tables, space + SPACE_BRIGHTNESS)
}

/// The radiance from space in unit `d` (`Space::radiance`): stars, the Milky Way's glow and
/// galaxies behind the dust, reddened by it, and the nebulae's light in front.
fn space_radiance(s: &Scene, space: u32, d: Vec3) -> Vec3 {
    let (stars, milky, nebulae, n_nebulae, galaxies, n_galaxies) = space_tables(s, space);
    let (l, b) = galactic(s, space, d);
    let mut behind = Vec3::ZERO;
    let mut tau = 0.0;
    if stars != NO_AIR {
        behind += star_cells(s, stars, d, false);
    }
    if milky != NO_AIR {
        let (brightness, dust) = (at(s.tables, milky), at(s.tables, milky + 1));
        let (disc_colour, bulge_colour) = (tv3(s, milky + 2), tv3(s, milky + 5));
        let (noise, dust_noise) = (tv3(s, milky + 8), tv3(s, milky + 11));
        // Its noise, in one call of fbm (rust-gpu inlines each call): the glow's two fields of
        // clouds (passes 0-1), the dust lane's offset (2), and the dust's two warped fields
        // (3-6 and 7-10: three fields that displace it, then the field itself; `warped_fbm`).
        let glows = brightness > 0.0;
        let dusty = dust > 0.0 && b.abs() <= 0.8;
        let (mut clouds, mut offset, mut fields) = (1.0, 0.0, Vec3::ZERO);
        let (mut qa, mut qb) = (Vec3::ZERO, Vec3::ZERO);
        let mut k = if glows { 0 } else { 2 };
        let end = if dusty { 11 } else { 2 };
        while k < end {
            let (field, p, octaves) = if k < 2 {
                (noise, d * if k == 0 { 4.0 } else { 23.0 }, 4)
            } else if k == 2 {
                (dust_noise, d * 1.5 + Vec3::splat(5.0), 3)
            } else {
                let second = k >= 7;
                let g = if second { k - 7 } else { k - 3 };
                let base = if second { d * 11.0 + Vec3::new(-3.0, 1.0, 7.0) } else { d * 7.0 };
                if g < 3 {
                    (dust_noise, base + warp_offset(g), 4)
                } else if second {
                    (dust_noise, base + qb, 5)
                } else {
                    (dust_noise, base + qa * 1.1, 7)
                }
            };
            let v = fbm(s, space, field, p, octaves);
            if k == 0 { clouds += 1.5 * v; } else if k == 1 { clouds += 0.8 * v; } else if k == 2 { offset = v; }
            else if k == 3 { qa.x = v; } else if k == 4 { qa.y = v; } else if k == 5 { qa.z = v; } else if k == 6 { fields.x = v; }
            else if k == 7 { qb.x = v; } else if k == 8 { qb.y = v; } else if k == 9 { qb.z = v; } else { fields.y = v; }
            k += 1;
        }
        // The glow (`MilkyWay::glow`).
        if glows {
            let (disc, bulge) = milky_profile(l, b);
            behind += (disc_colour * disc + bulge_colour * bulge) * (clouds.max(0.1).powf(1.3) * brightness);
        }
        // The dust (`MilkyWay::dust`).
        if dusty {
            let offset = 0.025 * offset;
            let width = 0.045 + 0.03 * l.abs() / PI;
            let lane = (-((b - offset) / width) * ((b - offset) / width)).exp();
            let filaments = smoothstep(0.0, 0.3, fields.x);
            let centre = 0.5 + 0.9 * (-(l / 1.5) * (l / 1.5)).exp();
            let clouds = (-(b / 0.3) * (b / 0.3)).exp() * smoothstep(0.25, 0.45, fields.y);
            tau += dust * (lane * (0.15 + 3.0 * filaments) * centre + 1.2 * clouds);
        }
    }
    let mut k = 0;
    while k < n_galaxies {
        behind += galaxy_radiance(s, space, galaxies + k * GALAXY_SIZE, d);
        k += 1;
    }
    let mut front = Vec3::ZERO;
    let mut k = 0;
    while k < n_nebulae {
        let (light, depth) = nebula_sample(s, space, nebulae + k * NEBULA_SIZE, d);
        front += light;
        tau += depth;
        k += 1;
    }
    let reddened = if tau <= 0.0 { Vec3::ONE } else { Vec3::new((-tau * 0.81).exp(), (-tau).exp(), (-tau * 1.25).exp()) };
    (behind * reddened + front) * at(s.tables, space + SPACE_BRIGHTNESS)
}

/// `nebula::colour`: a colour of unit luminance.
fn unit_colour(c: Vec3) -> Vec3 {
    c / luminance(c)
}

/// `Nebula::sample`: its light in unit `d`, and the optical depth of its dust. Each kind's
/// fields come from one call of warped fbm and one of fbm, with the kind's parameters.
fn nebula_sample(s: &Scene, space: u32, r: u32, d: Vec3) -> (Vec3, f32) {
    let kind = at(s.tables, r) as u32;
    let centre = tv3(s, r + 1);
    let (radius, cos_reach, brightness, dust) = (at(s.tables, r + 4), at(s.tables, r + 5), at(s.tables, r + 6), at(s.tables, r + 7));
    let (axis0, axis1) = (tv3(s, r + 8), tv3(s, r + 11));
    let (elongation, noise) = (at(s.tables, r + 14), tv3(s, r + 15));
    let c = d.dot(centre);
    if c < cos_reach {
        return (Vec3::ZERO, 0.0);
    }
    let w = d / c - centre;
    let (x, y) = (w.dot(axis0) / (radius * elongation), w.dot(axis1) / radius);
    let rr = (x * x + y * y).sqrt();
    let p = Vec3::new(x, y, 0.0) * 2.2;
    // Its noise, in one call of fbm: the ragged edge (pass 0); the gas of emission, reflection
    // and dark nebulae, warped (1-4, `warped_fbm`); a second field, emission's lanes or a
    // planetary nebula's wobble (5).
    let (gp, strength, gas_octaves) = if kind == 0 { (p, 1.2, 6) } else if kind == 1 { (p * 0.7, 0.8, 5) } else { (p, 1.0, 6) };
    let (sp, second_octaves) = if kind == 0 { (p * 1.7 + Vec3::new(-7.0, 2.0, 5.0), 5) } else { (p * 1.5, 4) };
    let (mut edge_field, mut gas_field, mut second) = (0.0, 0.0, 0.0);
    let mut q = Vec3::ZERO;
    let mut k = 0;
    while k < 6 {
        let wanted = k == 0 || (k <= 4 && kind != 2) || (k == 5 && (kind == 0 || kind == 2));
        if wanted {
            let (at_p, octaves) = if k == 0 {
                (p * 0.8 + Vec3::splat(3.0), 3)
            } else if k < 4 {
                (gp + warp_offset(k - 1), 4)
            } else if k == 4 {
                (gp + q * strength, gas_octaves)
            } else {
                (sp, second_octaves)
            };
            let v = fbm(s, space, noise, at_p, octaves);
            if k == 0 { edge_field = v; } else if k == 1 { q.x = v; } else if k == 2 { q.y = v; } else if k == 3 { q.z = v; }
            else if k == 4 { gas_field = v; } else { second = v; }
        }
        k += 1;
    }
    let edge = rr * (1.0 + 0.45 * edge_field);
    let h_alpha = unit_colour(Vec3::new(1.0, 0.12, 0.22));
    let o_iii = unit_colour(Vec3::new(0.1, 0.85, 0.75));
    if kind == 0 {
        let gas = smoothstep(-0.25, 0.45, gas_field);
        let envelope = smoothstep(1.05, 0.2, edge);
        let density = gas * envelope;
        let core = smoothstep(0.65, 0.0, edge) * gas;
        let lanes = smoothstep(0.1, 0.45, second) * envelope;
        let light = h_alpha * density + o_iii * (0.8 * core * core);
        (light * (brightness * (-2.0 * lanes).exp()), dust * lanes)
    } else if kind == 1 {
        let gas = smoothstep(-0.3, 0.5, gas_field);
        let density = gas * smoothstep(1.0, 0.0, edge).powf(1.5);
        (unit_colour(Vec3::new(0.35, 0.55, 1.0)) * (brightness * density), dust * density)
    } else if kind == 2 {
        let wobbled = rr * (1.0 + 0.15 * second);
        let shell = (-((wobbled - 0.65) / 0.18) * ((wobbled - 0.65) / 0.18)).exp();
        let inside = smoothstep(0.75, 0.2, wobbled);
        let star = (-(rr / 0.04) * (rr / 0.04)).exp() * 3.0;
        ((h_alpha * shell + o_iii * (0.7 * inside) + Vec3::splat(star)) * brightness, 0.0)
    } else {
        let thick = smoothstep(-0.2, 0.4, gas_field) * smoothstep(1.0, 0.3, edge);
        (Vec3::ZERO, dust * thick)
    }
}

/// `Galaxy::radiance`: its light in unit `d`.
fn galaxy_radiance(s: &Scene, space: u32, r: u32, d: Vec3) -> Vec3 {
    let elliptical = at(s.tables, r) > 0.5;
    let centre = tv3(s, r + 1);
    let (radius, cos_reach, brightness) = (at(s.tables, r + 4), at(s.tables, r + 5), at(s.tables, r + 6));
    let c = d.dot(centre);
    if c < cos_reach {
        return Vec3::ZERO;
    }
    let (normal, disc0, disc1, sky0, sky1) = (tv3(s, r + 7), tv3(s, r + 10), tv3(s, r + 13), tv3(s, r + 16), tv3(s, r + 19));
    let (axis_ratio, arms, pitch, phase, noise) = (at(s.tables, r + 22), at(s.tables, r + 23), at(s.tables, r + 24), at(s.tables, r + 25), tv3(s, r + 26));
    let w = (d / c - centre) / radius;
    let old = unit_colour(Vec3::new(1.0, 0.84, 0.64));
    let young = unit_colour(Vec3::new(0.7, 0.82, 1.0));
    if elliptical {
        let (x, y) = (w.dot(sky0), w.dot(sky1) / axis_ratio);
        let rr = (x * x + y * y).sqrt();
        return old * (brightness * (-2.5 * (rr / 0.3).powf(0.7)).exp());
    }
    let facing0 = normal.dot(centre);
    let facing = if facing0.abs() < 0.12 { if facing0 < 0.0 { -0.12 } else { 0.12 } } else { facing0 };
    let p = w - centre * (normal.dot(w) / facing);
    let (x, y) = (p.dot(disc0), p.dot(disc1));
    let rho = (x * x + y * y).sqrt();
    let theta = y.atan2(x);
    let wind = arms * (theta - (rho + 0.03).ln() / pitch) + phase;
    let arm = (0.5 + 0.5 * wind.cos()).powi(3) * facing.abs().sqrt();
    let knots = (1.0 + 1.2 * fbm(s, space, noise, Vec3::new(x, y, 0.0) * 9.0, 4)).max(0.0);
    let disc = (-rho / 0.28).exp() * smoothstep(1.15, 0.8, rho) * (1.0 / facing.abs()).min(4.0) * 0.12;
    let r_sky = w.length();
    let bulge = (-r_sky / 0.06).exp();
    let across = w.dot(sky1);
    let lane = 1.0 - 0.75 * (1.0 - facing.abs()) * (1.0 - facing.abs()) * (-(across / 0.035) * (across / 0.035)).exp() * (-r_sky / 0.7).exp();
    (old * (disc * (0.6 + 0.4 * (1.0 - arm)) + bulge) + young * (disc * arm * 1.8 * knots)) * (brightness * lane)
}

pub fn space_radiance_at(s: &Scene, d: Vec3) -> Vec3 {
    space_radiance(s, s.params.air.w, d)
}

pub fn space_bound_at(s: &Scene, d: Vec3) -> f32 {
    space_bound(s, s.params.air.w, d)
}

pub fn aerial_segment(s: &Scene, o: Vec3, d: Vec3, distance: f32) -> (Vec3, Vec3) {
    segment(s, s.params.air.y, o, d, distance)
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

/// Complex numbers, three at a time (one per colour channel): (real, imaginary).
type C3 = (Vec3, Vec3);

fn cmul(a: C3, b: C3) -> C3 {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn norm2(a: C3) -> Vec3 {
    a.0 * a.0 + a.1 * a.1
}

fn sqrt3(v: Vec3) -> Vec3 {
    Vec3::new(v.x.max(0.0).sqrt(), v.y.max(0.0).sqrt(), v.z.max(0.0).sqrt())
}

/// `ggx::conductor_reflectance` for each channel of a conductor with complex index of
/// refraction `eta` + i `k`, complex arithmetic spelled out. All three channels at once, so the
/// kernel has one copy of this rather than three.
pub fn conductor_reflectance3(cos_in: f32, eta: Vec3, k: Vec3) -> Vec3 {
    let cos_i = Vec3::splat(cos_in.clamp(0.0, 1.0));
    let sin2_i = Vec3::ONE - cos_i * cos_i;
    let n2 = cmul((eta, k), (eta, k));
    let d = norm2(n2);
    // sin²t = sin²i / n², cos t = √(1 - sin²t), principal root.
    let z = (Vec3::ONE - sin2_i * n2.0 / d, sin2_i * n2.1 / d);
    let r = sqrt3(norm2(z));
    let mut ct = (sqrt3((r + z.0) / 2.0), sqrt3((r - z.0) / 2.0));
    ct.1 = Vec3::new(
        if z.1.x < 0.0 { -ct.1.x } else { ct.1.x },
        if z.1.y < 0.0 { -ct.1.y } else { ct.1.y },
        if z.1.z < 0.0 { -ct.1.z } else { ct.1.z },
    );
    let a = (eta * cos_i, k * cos_i);
    let nct = cmul((eta, k), ct);
    let tiny = Vec3::splat(1e-30);
    let rp = norm2((a.0 - ct.0, a.1 - ct.1)) / norm2((a.0 + ct.0, a.1 + ct.1)).max(tiny);
    let rs = norm2((cos_i - nct.0, -nct.1)) / norm2((cos_i + nct.0, nct.1)).max(tiny);
    (rp + rs) / 2.0
}

/// One channel of `conductor_reflectance3`.
pub fn conductor_reflectance(cos_in: f32, eta: f32, k: f32) -> f32 {
    conductor_reflectance3(cos_in, Vec3::splat(eta), Vec3::splat(k)).x
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
            conductor_reflectance3(cos, self.albedo, self.k)
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

    /// The microfacet lobe's f |cos θ_i| and density, sharing their terms.
    fn ggx_eval_pdf(&self, wo: Vec3, wi: Vec3) -> (Vec3, f32) {
        let (o, i) = (self.local(wo), self.local(wi));
        if o.z <= 0.0 || i.z <= 0.0 {
            return (Vec3::ZERO, 0.0);
        }
        let h = (o + i).normalize();
        let d = self.ggx_d(h);
        let lo = self.ggx_lambda(o);
        let g2 = 1.0 / (1.0 + lo + self.ggx_lambda(i));
        (self.fresnel(o.dot(h)) * (g2 * d / (4.0 * o.z)), d / (1.0 + lo) / (4.0 * o.z))
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

    /// f |cos θ_i| and the density of sampling `wi`, together: the kernel evaluates a lobe
    /// at as few places as it can, since each is inlined.
    pub fn eval_pdf(&self, wo: Vec3, wi: Vec3) -> (Vec3, f32) {
        let c = wi.dot(self.n);
        let cos_pdf = c.max(0.0) / PI;
        if self.kind == 0 {
            return (self.lambert_eval(self.albedo, wi), cos_pdf);
        }
        let (f, pdf) = self.ggx_eval_pdf(wo, wi);
        if self.kind == 1 {
            return (f, pdf);
        }
        // The coat, plus the base dimmed by what the coat reflects; each sampled in proportion.
        let share = self.coat_share(wo);
        (f + self.lambert_eval(self.base, wi) * (1.0 - share), share * pdf + (1.0 - share) * cos_pdf)
    }

    /// f |cos θ_i|.
    pub fn eval(&self, wo: Vec3, wi: Vec3) -> Vec3 {
        self.eval_pdf(wo, wi).0
    }

    pub fn pdf(&self, wo: Vec3, wi: Vec3) -> f32 {
        self.eval_pdf(wo, wi).1
    }

    /// A direction, or zero if none.
    fn sample(&self, wo: Vec3, u1: f32, u2: f32, u3: f32) -> Vec3 {
        if self.kind == 0 || (self.kind == 2 && u3 >= self.coat_share(wo)) {
            cosine_sample(self.n, u1, u2)
        } else {
            self.ggx_sample(wo, u1, u2)
        }
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
        if mat.kind != MAT_CHECKER {
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
    if mat.kind == MAT_EMITTER && (mat.a == 1 || towards.dot(n) > 0.0) { mat.c0.xyz() } else { Vec3::ZERO }
}

fn prim_area(s: &Scene, i: u32) -> f32 {
    let p = at(s.prims, i);
    if p.info.x == PRIM_TRIANGLE {
        (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).length() / 2.0
    } else {
        4.0 * PI * p.v0.w * p.v0.w
    }
}

/// 1 - cos θ_max for a sphere of radius² `r2` at distance² `d2`, without the cancellation of
/// 1 - √(1 - x) in f32 for small, distant spheres (x = r²/d²).
fn one_minus_cos_max(r2: f32, d2: f32) -> f32 {
    let x = (r2 / d2).min(1.0);
    x / (1.0 + (1.0 - x).sqrt())
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
    1.0 / (2.0 * PI * one_minus_cos_max(r2, d2))
}

/// A point on emitting primitive i seen from `org`, its normal, and the density per solid
/// angle: a sphere by its cone, a triangle by area.
fn sample_light(s: &Scene, i: u32, org: Vec3, u1: f32, u2: f32) -> (Vec3, Vec3, f32) {
    let p = at(s.prims, i);
    if p.info.x == PRIM_SPHERE {
        let (c, r) = (p.v0.xyz(), p.v0.w);
        let to = c - org;
        let d2 = to.dot(to);
        if d2 <= r * r * 1.000001 {
            let n = uniform_sphere(u1, u2);
            let y = c + n * r;
            return (y, n, sphere_pdf(c, r, org, y, n));
        }
        // pbrt's cone sampling: the point is placed by its angle at the centre rather than by
        // intersecting the sampled direction with the sphere, which in f32 loses a small,
        // distant sphere entirely (d² - r² rounds to d²).
        let sin2_max = (r * r / d2).min(1.0);
        let sin_max = sin2_max.sqrt();
        let one_minus = one_minus_cos_max(r * r, d2);
        let sin2_t = if sin2_max < 0.00068523 {
            sin2_max * u1
        } else {
            let cos_t = 1.0 - u1 * one_minus;
            (1.0 - cos_t * cos_t).max(0.0)
        };
        let cos_t = (1.0 - sin2_t).max(0.0).sqrt();
        let cos_a = sin2_t / sin_max + cos_t * (1.0 - sin2_t / sin2_max).max(0.0).sqrt();
        let sin_a = (1.0 - cos_a * cos_a).max(0.0).sqrt();
        let phi = 2.0 * PI * u2;
        let w = to / d2.sqrt();
        let (t, b) = basis(w);
        let n = -(t * (sin_a * phi.cos()) + b * (sin_a * phi.sin()) + w * cos_a);
        return (c + n * r, n, 1.0 / (2.0 * PI * one_minus));
    }
    let su = u1.sqrt();
    let (b1, b2) = (1.0 - su, u2 * su);
    let y = p.v0.xyz() * b1 + p.v1.xyz() * b2 + p.v2.xyz() * (1.0 - b1 - b2);
    let n = (p.v1.xyz() - p.v0.xyz()).cross(p.v2.xyz() - p.v0.xyz()).normalize();
    (y, n, light_pdf(s, i, org, y, n))
}

fn light_pdf(s: &Scene, i: u32, org: Vec3, y: Vec3, n: Vec3) -> f32 {
    let p = at(s.prims, i);
    if p.info.x == PRIM_SPHERE {
        return sphere_pdf(p.v0.xyz(), p.v0.w, org, y, n);
    }
    let w = y - org;
    let d2 = w.dot(w);
    let cos = n.dot(w).abs() / d2.sqrt();
    if cos <= 0.0 { 0.0 } else { d2 / (cos * prim_area(s, i)) }
}

/// The sun's disc seen from `o` along `d` (`Sun::disc_radiance`): reddened by the air to `o`.
/// The angle between unit `d` and the sun's centre, from their chord: near the sun, cos θ is
/// within f32's rounding of 1 (the sun's 1 - cos r is 1.1e-5, f32's steps near 1 are 6e-8), so
/// the disc's edge, found from the cosine, was ragged.
fn sun_angle(s: &Scene, d: Vec3) -> f32 {
    2.0 * ((d - s.params.sun_dir.xyz()).length() / 2.0).min(1.0).asin()
}

/// The disc's solid angle, 2π (1 - cos r) = 4π sin²(r / 2), without the cancellation.
fn sun_solid_angle(s: &Scene) -> f32 {
    let h = (s.params.sun_irr.w / 2.0).sin();
    4.0 * PI * h * h
}

fn sun_disc(s: &Scene, o: Vec3, d: Vec3) -> Vec3 {
    let theta = sun_angle(s, d);
    if theta >= s.params.sun_irr.w {
        return Vec3::ZERO;
    }
    let mu = (1.0 - (theta / s.params.sun_irr.w).powi(2)).max(0.0).sqrt();
    s.params.sun_irr.xyz() * sun_transmittance(s, o) * ((1.0 - LIMB_DARKENING * (1.0 - mu)) / (1.0 - LIMB_DARKENING / 3.0) / sun_solid_angle(s))
}

fn sun_disc_pdf(s: &Scene, d: Vec3) -> f32 {
    if sun_angle(s, d) >= s.params.sun_irr.w { 0.0 } else { 1.0 / sun_solid_angle(s) }
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

/// A path between bounces, one per pixel in the `paths` buffer: `bounce` advances it a vertex
/// at a time, and `sky_light` adds the light its vertex asked of the sky (`SkyQuery`). The host
/// starts it from a camera ray (`PathState::start`).
#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct PathState {
    /// The ray's origin, and the camera sample's weight.
    pub o: Vec4,
    /// Its direction, and the density it was sampled with (for MIS).
    pub d: Vec4,
    /// The throughput, and the depth.
    pub beta: Vec4,
    /// The light gathered so far, and the rays traced.
    pub l: Vec4,
    /// The random numbers' state, and `PATH_*` flags.
    pub state: UVec4,
}

/// `PathState::state.y`: still tracing; its last bounce was specular; it has scattered off a
/// rough surface (so regularisation applies); its light has been added to the image.
pub const PATH_ACTIVE: u32 = 1;
pub const PATH_SPECULAR: u32 = 2;
pub const PATH_SCATTERED: u32 = 4;
pub const PATH_ADDED: u32 = 8;

impl PathState {
    /// Pixel `index`'s path in pass `pass` from its camera ray.
    pub fn start(index: u32, pass: u32, seed: u32, ray: &CameraRay) -> PathState {
        let live = ray.d.w >= 0.5;
        PathState {
            o: ray.o, d: ray.d.truncate().normalize_or_zero().extend(0.0), beta: Vec4::new(1.0, 1.0, 1.0, 0.0), l: Vec4::ZERO,
            state: UVec4::new(path_seed(index, pass, seed), if live { PATH_ACTIVE | PATH_SPECULAR } else { PATH_ADDED }, 0, 0),
        }
    }
}

/// Light a vertex asks of the sky, which `sky_light` finds: the sky beyond (`background`) along
/// a ray from `o` (`o.w` < 0), or the air's in-scattering towards `o` from `o.w` away (aerial
/// perspective, `segment`), times `weight`; none if `weight` is zero. Two per pixel: the
/// escaping ray's or the hit's haze, and the sky's light sample.
#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct SkyQuery {
    pub o: Vec4,
    pub d: Vec4,
    pub weight: Vec4,
}

impl SkyQuery {
    pub const NONE: SkyQuery = SkyQuery { o: Vec4::ZERO, d: Vec4::ZERO, weight: Vec4::ZERO };

    fn sky(o: Vec3, d: Vec3, weight: Vec3) -> SkyQuery {
        SkyQuery { o: o.extend(-1.0), d: d.extend(0.0), weight: weight.extend(0.0) }
    }

    fn haze(o: Vec3, d: Vec3, distance: f32, weight: Vec3) -> SkyQuery {
        SkyQuery { o: o.extend(distance), d: d.extend(0.0), weight: weight.extend(0.0) }
    }
}

/// The light a query finds, times its weight.
pub fn sky_light(s: &Scene, q: &SkyQuery) -> Vec3 {
    let w = q.weight.xyz();
    if w.dot(w) == 0.0 {
        return Vec3::ZERO;
    }
    if q.o.w < 0.0 {
        w * background(s, q.o.xyz(), q.d.xyz())
    } else {
        w * segment(s, s.params.air.y, q.o.xyz(), q.d.xyz(), q.o.w).1
    }
}

/// `segment`'s transmittance alone: the depths along the ray, which the path needs at once
/// (its in-scattering is the sky kernel's).
fn aerial_transmittance(s: &Scene, o: Vec3, d: Vec3, distance: f32) -> Vec3 {
    let a = load_air(s, s.params.air.y);
    let (start, end) = match view_ray_span(&a, o, d) {
        Some((start, end)) if start < distance => (start, end.min(distance)),
        _ => return Vec3::ONE,
    };
    let mut depth = Vec3::ZERO;
    let mut k = 0;
    while k < SKY_STEPS {
        let (t, step) = span_sample(&a, o, d, start, end, SKY_STEPS, k);
        depth += depths(&a, altitude(&a, o + d * t), step);
        k += 1;
    }
    transmittance(&a, depth)
}

/// The path with a new ray, still tracing.
#[allow(clippy::too_many_arguments)]
fn carry_on(p: &PathState, o: Vec3, d: Vec3, beta: Vec3, l: Vec3, rays: u32, rng: u32, flags: u32, prev_pdf: f32, depth: u32, q0: SkyQuery, q1: SkyQuery) -> (PathState, SkyQuery, SkyQuery) {
    (PathState { o: o.extend(p.o.w), d: d.extend(prev_pdf), beta: beta.extend(depth as f32), l: l.extend(rays as f32), state: UVec4::new(rng, flags, 0, 0) }, q0, q1)
}

/// The path, ended at this vertex (its sky queries still to be answered).
#[allow(clippy::too_many_arguments)]
fn finish(p: &PathState, o: Vec3, d: Vec3, beta: Vec3, l: Vec3, rays: u32, rng: u32, flags: u32, prev_pdf: f32, depth: u32, q0: SkyQuery, q1: SkyQuery) -> (PathState, SkyQuery, SkyQuery) {
    carry_on(p, o, d, beta, l, rays, rng, flags & !PATH_ACTIVE, prev_pdf, depth, q0, q1)
}

/// One vertex of a path (`PathState`): trace its ray; at what it hits, haze, emission, direct
/// light (asking the sky for its light sample), and the next ray, or the end. Returns the path
/// and its two sky queries.
pub fn bounce(s: &Scene, p: &PathState) -> (PathState, SkyQuery, SkyQuery) {
    let (ro, rd, prev_pdf) = (p.o.xyz(), p.d.xyz(), p.d.w);
    let mut beta = p.beta.xyz();
    let depth = p.beta.w as u32;
    let mut l = p.l.xyz();
    let mut rays = p.l.w as u32;
    let mut rng = p.state.x;
    let flags_p = p.state.y;
    let specular = flags_p & PATH_SPECULAR != 0;
    let (mut q0, mut q1) = (SkyQuery::NONE, SkyQuery::NONE);
    let max_depth = s.params.dims.w;
    let flags = s.params.offs2.w;
    let n_emitters = s.params.counts.z;

    let hit = trace(s, ro, rd, 0.0, INF, false);
    rays += 1;
    if hit.prim == NONE {
        // The sky, weighted against sampling it if the last vertex did, and the sun's disc.
        let mut w = 1.0;
        if !specular && flags & FLAG_SAMPLE_ENV != 0 {
            w = power_heuristic(prev_pdf, env_pdf(s, rd));
        }
        q0 = SkyQuery::sky(ro, rd, beta * w);
        if flags & FLAG_SUN != 0 {
            let ws = if specular { 1.0 } else { power_heuristic(prev_pdf, sun_disc_pdf(s, rd)) };
            l += beta * sun_disc(s, ro, rd) * ws;
        }
        return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
    }
    // Aerial perspective: the air before the hit attenuates what lies beyond it, and adds its
    // own light (found by the sky kernel, weighted by the throughput before).
    if flags & FLAG_AERIAL != 0 && hit.t > AERIAL_FROM {
        q0 = SkyQuery::haze(ro, rd, hit.t, beta);
        beta *= aerial_transmittance(s, ro, rd, hit.t);
    }
    let point = ro + rd * hit.t;
    let prim = at(s.prims, hit.prim);
    let m = resolve_material(s, prim.info.y, point);
    let mat = at(s.materials, m);
    if mat.kind == MAT_EMITTER {
        let le = emitted(s, m, hit.ns, -rd);
        let mut w = 1.0;
        if !specular && flags & FLAG_SAMPLE_LIGHTS != 0 && prim.info.z > 0 {
            let lp = emitter_probability(s, prim.info.z - 1) * light_pdf(s, hit.prim, ro, point, hit.ng);
            w = power_heuristic(prev_pdf, lp);
        }
        l += beta * le * w;
        return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
    }
    let wo = -rd;
    let ng = if hit.ng.dot(wo) < 0.0 { -hit.ng } else { hit.ng };
    let ns = if hit.ns.dot(wo) < 0.0 { -hit.ns } else { hit.ns };
    let eps = offset_for(s, point);
    let above = point + ng * eps;
    let floor_alpha = if flags_p & PATH_SCATTERED != 0 { s.params.misc.x } else { 0.0 };

    // Delta bounces: mirrors, glass, smooth coats.
    let mut delta = false;
    let mut next_dir = Vec3::ZERO;
    let mut next_from = above;
    let mut attenuate = Vec3::ONE;
    let mut lobe = Lobe::new(0, ns);
    if mat.kind == MAT_DIFFUSE {
        lobe.albedo = mat.c0.xyz();
    } else if mat.kind == MAT_PLASTIC {
        let alpha = (mat.p.y * mat.p.y).max(floor_alpha);
        if alpha >= MIN_ALPHA {
            lobe.kind = 2;
            lobe.fresnel = 1;
            lobe.alpha = alpha;
            lobe.ior = mat.p.x;
            lobe.base = mat.c0.xyz();
        } else if rand(&mut rng) < fresnel_reflectance(wo.dot(ns).abs(), 1.0 / mat.p.x, mat.p.x) {
            delta = true;
            next_dir = reflect_about(rd, ns);
        } else {
            lobe.albedo = mat.c0.xyz();
        }
    } else if mat.kind == MAT_CONDUCTOR || mat.kind == MAT_METAL {
        let alpha = (mat.p.y * mat.p.y).max(floor_alpha);
        lobe.kind = 1;
        lobe.alpha = alpha;
        lobe.albedo = mat.c0.xyz();
        lobe.k = mat.c1.xyz();
        lobe.fresnel = if mat.kind == MAT_METAL { 0 } else { 2 };
        if alpha < MIN_ALPHA {
            delta = true;
            next_dir = reflect_about(rd, ns);
            attenuate = lobe.fresnel(wo.dot(ns).abs());
        }
    } else if mat.kind == MAT_DIELECTRIC {
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
        if disc > 0.0 && rand(&mut rng) >= r {
            next_dir = ((rd - outward * dt) * ni_over_nt - outward * disc.sqrt()).normalize();
            next_from = point - outward * eps;
            attenuate = mat.c0.xyz();
        } else {
            next_dir = reflect_about(rd, n);
            next_from = point + outward * eps;
        }
    }

    if delta {
        if depth >= max_depth || !survives(s, depth, attenuate, &mut beta, &mut rng) {
            return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
        }
        beta *= attenuate;
        return carry_on(p, next_from, next_dir.normalize(), beta, l, rays, rng, flags_p | PATH_SPECULAR, prev_pdf, depth + 1, q0, q1);
    }

    let flags_p = (flags_p | PATH_SCATTERED) & !PATH_SPECULAR;
    // Direct light, one shadow ray at a time, with one evaluation of the lobe and one call
    // of `occluded` (so one copy of each once rust-gpu inlines everything): every point
    // light, then the sun, one area light and the sky. Each gives a direction, a distance,
    // the light arriving if nothing is in the way, and its density (0 for a delta light, so
    // no MIS weight).
    let n_lights = s.params.counts.w;
    let mut k = 0;
    while k < n_lights + 3 {
        let mut wi = Vec3::ZERO;
        let mut dist = 1e30;
        let mut li = Vec3::ZERO;
        let mut pdf = 0.0;
        if k < n_lights {
            let light = at(s.lights, k);
            let to = light.pos.xyz() - above;
            let d2 = to.dot(to);
            wi = to / d2.sqrt();
            dist = d2.sqrt();
            li = light.intensity.xyz() * spot(&light, -wi) / d2;
        } else if k == n_lights {
            // The sun: a point for lobes that can't resolve its disc, else its disc with MIS.
            if flags & FLAG_SUN != 0 {
                let sd = s.params.sun_dir.xyz();
                let solid_angle = sun_solid_angle(s);
                if lobe.pdf(wo, sd) * solid_angle < POINT_SUN {
                    wi = sd;
                    li = s.params.sun_irr.xyz() * sun_transmittance(s, above);
                } else {
                    // Uniformly in the cone, by 1 - cos θ (as `sun_angle`, without the
                    // rounding of cos θ near 1).
                    let one_minus = rand(&mut rng) * solid_angle / (2.0 * PI);
                    let cos_t = 1.0 - one_minus;
                    let sin_t = (one_minus * (2.0 - one_minus)).max(0.0).sqrt();
                    let phi = 2.0 * PI * rand(&mut rng);
                    let (t, b) = basis(sd);
                    wi = t * (sin_t * phi.cos()) + b * (sin_t * phi.sin()) + sd * cos_t;
                    pdf = sun_disc_pdf(s, wi);
                    if pdf > 0.0 {
                        li = sun_disc(s, above, wi);
                    }
                }
            }
        } else if k == n_lights + 1 {
            // One area light, by power.
            if flags & FLAG_SAMPLE_LIGHTS != 0 && n_emitters > 0 {
                let e = search(s, s.params.offs.y, n_emitters, rand(&mut rng));
                let pmf = emitter_probability(s, e);
                let pi = at(s.tables, s.params.offs.x + e).to_bits();
                let (y, ny, light_pdf) = sample_light(s, pi, above, rand(&mut rng), rand(&mut rng));
                let to = y - above;
                let d = to.length();
                wi = to / d;
                dist = d * (1.0 - 1e-4) - eps;
                if light_pdf > 0.0 {
                    pdf = pmf * light_pdf;
                    li = emitted(s, at(s.prims, pi).info.y, ny, -wi);
                }
            }
        } else if flags & FLAG_SAMPLE_ENV != 0 {
            // The sky, by its brightness: its light found by the `sky` kernel (`q1`).
            let es = env_sample(s, rand(&mut rng), rand(&mut rng));
            if es.w > 0.0 {
                wi = es.xyz();
                pdf = es.w;
                li = Vec3::ONE;
            }
        }
        if li.dot(li) > 0.0 {
            let (f, lobe_pdf) = lobe.eval_pdf(wo, wi);
            let w = if pdf > 0.0 { power_heuristic(pdf, lobe_pdf) / pdf } else { 1.0 };
            if f.dot(f) > 0.0 {
                rays += 1;
                if !occluded(s, above, wi, dist) {
                    if k == n_lights + 2 {
                        q1 = SkyQuery::sky(above, wi, beta * f * w);
                    } else {
                        l += beta * f * li * w;
                    }
                }
            }
        }
        k += 1;
    }
    // The next ray.
    if depth >= max_depth {
        return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
    }
    let wi = lobe.sample(wo, rand(&mut rng), rand(&mut rng), rand(&mut rng));
    let (f, pdf) = lobe.eval_pdf(wo, wi);
    if wi.dot(wi) == 0.0 || pdf <= 0.0 {
        return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
    }
    let weight = f / pdf;
    if !survives(s, depth, weight, &mut beta, &mut rng) {
        return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
    }
    beta *= weight;
    let side = if wi.dot(ng) < 0.0 { -ng } else { ng };
    carry_on(p, point + side * eps, wi, beta, l, rays, rng, flags_p, pdf, depth + 1, q0, q1)
}

/// The radiance along one camera ray, a vertex at a time as on the GPU (`bounce`, then its sky
/// queries): for the tests, on the CPU. `rays` counts the rays traced.
#[cfg(not(target_arch = "spirv"))]
pub fn radiance(s: &Scene, ro: Vec3, rd: Vec3, rng: &mut u32, rays: &mut u32) -> Vec3 {
    let ray = CameraRay { o: ro.extend(1.0), d: rd.extend(1.0) };
    let mut p = PathState::start(0, 0, 0, &ray);
    p.state.x = *rng;
    loop {
        let (next, q0, q1) = bounce(s, &p);
        p = next;
        p.l += (sky_light(s, &q0) + sky_light(s, &q1)).extend(0.0);
        if p.state.y & PATH_ACTIVE == 0 {
            break;
        }
    }
    *rng = p.state.x;
    *rays = p.l.w as u32;
    p.l.xyz()
}

/// The pixel this invocation serves (dispatched as rows of at most 65535 workgroups), if any.
fn pixel(gid: UVec3, params: &Params) -> Option<usize> {
    let index = gid.x + gid.y * (65535 * 64);
    if index >= params.dims.x * params.dims.y { None } else { Some(index as usize) }
}

/// One vertex of every pixel's path (`bounce`), its sky queries in two slots a pixel; a path
/// found ended (its queries answered by `answer_sky` since) adds its light to the image, once.
/// A pass dispatches this `max_depth` + 2 times, with `answer_sky` after each but the last.
#[spirv(compute(threads(64)))]
pub fn bounce_paths(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Prim],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] accum: &mut [Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] queries: &mut [SkyQuery],
) {
    let Some(index) = pixel(gid, params) else { return };
    let p = paths[index];
    // Each buffer written once, after the branches (writes in each would join pointers chosen
    // by branch, which needs SPIR-V's variable pointers).
    let (mut next, mut q0, mut q1) = (p, SkyQuery::NONE, SkyQuery::NONE);
    let mut add = Vec4::ZERO;
    if p.state.y & PATH_ACTIVE != 0 {
        let s = Scene { params, prims, nodes, materials, lights, tables };
        (next, q0, q1) = bounce(&s, &p);
    } else if p.state.y & PATH_ADDED == 0 {
        // A NaN (from some degenerate case) is dropped, as the CPU renderers drop theirs; its
        // rays were still cast. (Not `is_finite`: its infinite constant is invalid for naga,
        // which wgpu translates through.) w counts the rays.
        let c = p.l.xyz() * p.o.w;
        #[allow(clippy::eq_op)]
        let not_nan = c.x == c.x && c.y == c.y && c.z == c.z;
        add = if not_nan { c } else { Vec3::ZERO }.extend(p.l.w);
        next.state.y = p.state.y | PATH_ADDED;
    }
    paths[index] = next;
    queries[2 * index] = q0;
    queries[2 * index + 1] = q1;
    if add.w > 0.0 {
        accum[index] += add;
    }
}

/// The light every pixel's sky queries find (`sky_light`), added to its path's: the sky, space
/// and the air's haze, in a kernel of their own, as they are most of the shader's code.
#[spirv(compute(threads(64)))]
pub fn answer_sky(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Prim],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] queries: &mut [SkyQuery],
) {
    let Some(index) = pixel(gid, params) else { return };
    let s = Scene { params, prims, nodes, materials, lights, tables };
    let mut light = Vec3::ZERO;
    // Both slots, at one call site of `sky_light`.
    let mut k = 0;
    while k < 2 {
        // A copy: a reference into the buffer chosen by the loop would be a pointer chosen by
        // branch, which needs SPIR-V's variable pointers.
        let q = queries[2 * index + k];
        light += sky_light(&s, &q);
        k += 1;
    }
    if light.dot(light) > 0.0 {
        paths[index].l += light.extend(0.0);
    }
}
