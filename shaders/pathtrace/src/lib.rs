//! The GPU path tracer, in Rust, compiled to SPIR-V by rust-gpu: one invocation per pixel per
//! pass over the flattened scene (see the renderer's `gpu/mod.rs` for the buffers' layouts,
//! which these structs match). It ports the CPU wavefront integrator's shading
//! (`wavefront::integrator`): the same BSDFs, light sampling with the power heuristic, Russian
//! roulette and regularisation, in f32.
#![cfg_attr(target_arch = "spirv", no_std)]
#![allow(clippy::too_many_arguments)]

// (Some are used only by the kernels' entry points, which the CPU build leaves out.)
#[allow(unused_imports)]
use spirv_std::glam::{UVec3, UVec4, Vec2, Vec3, Vec4, Vec4Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
#[allow(unused_imports)]
use spirv_std::spirv;

/// glam, as the shader sees it, for building its buffers on the CPU.
pub use spirv_std::glam;

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Params {
    /// Width, height, where the sea's record is in `tables` plus one (0 for none: `PRIM_OCEAN`),
    /// max depth.
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
    /// Seed, Russian roulette on, the side of the queue a round reads, and the lanes: how
    /// many paths each pixel has at once (`slots`).
    pub seed: UVec4,
    /// Records of the air in `tables` (`AIR_*`), or `NO_AIR`: the sky's (for rays that escape),
    /// the aerial perspective's, the sun's (its reddening); and space's (`SPACE_*`).
    pub air: UVec4,
    /// The media (`MEDIUM_*`): where their records begin in `tables`, and how many.
    pub media: UVec4,
    /// The render's origin in the world (the renderer's `kernels::origin`: at the camera).
    /// Every position here is relative to it: rays, primitives, boxes, lights, centres. So
    /// f32 holds what is near the camera finely wherever in the world that is, and what is
    /// far to a constant fraction of its distance, which is a constant fraction of a pixel.
    /// Only what is tabulated by where it is in the world (the waves' tiles, the foam's and
    /// the clouds' noise) adds it back (`world`).
    pub origin: Vec4,
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
/// An interior node of the BVH, holding its two children's boxes: a ray tests both at one
/// visit, goes into the nearer and keeps the farther for later only if it enters it (Aila and
/// Laine, "Understanding the Efficiency of Ray Traversal on GPUs", HPG 2009). Leaves have no
/// node of their own: a child is a leaf when its count isn't zero.
pub struct Node {
    pub amin: Vec3,
    /// Child a: a leaf's first item, or an interior node's index.
    pub a: u32,
    pub amax: Vec3,
    /// A leaf's items; 0 for an interior node. A child that isn't there (the second child of
    /// a tree that is one leaf) is the interior node `NO_CHILD`, which no ray enters.
    pub a_count: u32,
    pub bmin: Vec3,
    pub b: u32,
    pub bmax: Vec3,
    pub b_count: u32,
}

/// The index of a child that isn't there. (No box can say so: every ray "enters" an inverted
/// or infinite one, by the slab test.)
pub const NO_CHILD: u32 = 0xffff_ffff;

/// A child as the traversal's stack holds it: an interior node's index, or for a leaf, this
/// bit, its count less one in the next `LEAF_COUNT_BITS` and its first item in the rest.
pub const LEAF: u32 = 0x8000_0000;
pub const LEAF_COUNT_BITS: u32 = 3;
/// The most items a leaf may hold, and one more than the largest first item.
pub const MAX_LEAF: u32 = 1 << LEAF_COUNT_BITS;
pub const MAX_LEAF_ITEMS: u32 = 1 << (31 - LEAF_COUNT_BITS);

#[inline(always)]
fn child(index: u32, count: u32) -> u32 {
    if count > 0 { LEAF | ((count - 1) << (31 - LEAF_COUNT_BITS)) | index } else { index }
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
    /// Direction, and the sample's number plus one if there is a ray (0 if the lens stopped it,
    /// or the pixel takes no sample in its lane): `start_paths` starts each path from its ray.
    pub d: Vec4,
}

// Constants the host shares (`gpu/mod.rs` uses these, so the two sides agree).

/// `Params::offs2.w`'s flags.
pub const FLAG_SAMPLE_LIGHTS: u32 = 1;
pub const FLAG_SAMPLE_ENV: u32 = 2;
pub const FLAG_SUN: u32 = 4;
pub const FLAG_AERIAL: u32 = 8;
/// The air along a camera ray is tested for shadow (`shadowed_loss`): crepuscular rays.
pub const FLAG_CREPUSCULAR: u32 = 16;
/// `Prim::info.x`.
pub const PRIM_TRIANGLE: u32 = 0;
pub const PRIM_SPHERE: u32 = 1;
pub const PRIM_PLANE: u32 = 2;
/// The sea (the renderer's `ocean::Ocean`; `ocean_hit`): one at most, after the planes, its
/// record in `tables` at `Params::dims.z` - 1.
pub const PRIM_OCEAN: u32 = 3;
/// A shape the kernels have no test for, which keeps its place among the primitives: the
/// host finds what a ray hits of it (the renderer's `wavefront::integrator`, between
/// `intersect_surface` and `intersect_media`, and `HostShapes` for shadow rays), and shades
/// it (`MAT_HOST`). Only where the host can be asked at each ray: on the CPU.
pub const PRIM_HOST: u32 = 4;
/// The sea's record: the planet's radius (0 for the plane y = 0), where its waves displace the
/// surface (the centre's x and z, and how far from it), the waves' greatest height and slope,
/// how many tiles of waves there are and how many of them displace (the rest only tilt the
/// normal), and the choppy map's λ (the renderer's `WaveField::lambda`; 0 for none); then for
/// each tile (`OCEAN_TILE` numbers) its size in metres, its texels a side, where its texels
/// (height, and the slopes along x and z) begin from the record's start, and where its choppy
/// displacement's do (five numbers a texel, the renderer's `WaveTile::chop`; 0 for none).
pub const OCEAN_RADIUS: u32 = 0;
pub const OCEAN_CENTRE: u32 = 1;
pub const OCEAN_DETAIL: u32 = 3;
pub const OCEAN_MAX: u32 = 4;
pub const OCEAN_TILES: u32 = 6;
pub const OCEAN_LAMBDA: u32 = 8;
/// Where the sea's foam's record begins, from the sea's (as bits; 0 for none).
pub const OCEAN_FOAM: u32 = 9;
/// The planet's centre in the render's frame, and |c|² - r² for it, as the air's are
/// (`excess`): filled in by whoever lays the record out (the renderer's `kernels`).
pub const OCEAN_PLANET: u32 = 10;
pub const OCEAN_EXCESS: u32 = 13;
pub const OCEAN_HEADER: u32 = 14;
pub const OCEAN_TILE: u32 = 4;
/// The least Jacobian of the choppy map the slopes are sharpened by (the renderer's
/// `ocean::MIN_JACOBIAN`).
pub const OCEAN_MIN_JACOBIAN: f32 = 0.2;
/// The foam's record (the renderer's `ocean::Foam::export`): its albedo, the wind's direction
/// and speed, the spread of the lace's noise, and how many tiles; then for each tile its
/// texels a side, its size in metres and where its texels (fresh foam, residual foam) begin
/// from the record's start; then the noise's permutation (512 entries); then the texels.
pub const FOAM_WIND: u32 = 3;
pub const FOAM_TILES: u32 = 7;
pub const FOAM_HEADER: u32 = 8;
/// Residual foam's albedo, as a fraction of fresh foam's (the renderer's
/// `ocean::RESIDUAL_FOAM`).
pub const RESIDUAL_FOAM: f32 = 0.7;
/// The most steps a ray is marched over the waves (`ocean_march`); the renderer's march has no
/// limit, and takes far fewer.
pub const OCEAN_STEPS: u32 = 1024;
/// `Material::kind`.
pub const MAT_DIFFUSE: u32 = 0;
pub const MAT_PLASTIC: u32 = 1;
pub const MAT_DIELECTRIC: u32 = 2;
pub const MAT_CONDUCTOR: u32 = 3;
pub const MAT_EMITTER: u32 = 4;
pub const MAT_METAL: u32 = 5;
pub const MAT_CHECKER: u32 = 6;
/// A material the kernels have no code for: the host shades the vertex (`PATH_HOST`).
pub const MAT_HOST: u32 = 7;
/// The sea's material (the renderer's `ocean::PhysicalOcean`, opaque water): what the water
/// body sends back (`c0`), what bubbles under a breaking crest add to that (`c1`), the
/// ripples' mean square slope (`p.y`), and the planet's radius for the local vertical (`p.x`;
/// 0 for a flat sea). Its foam, if any, is with the sea's record (`OCEAN_FOAM`, `sea_foam`).
pub const MAT_WATER: u32 = 8;
/// No surface's: a vertex in a medium (`MEDIUM_HIT`), which scatters by its phase function.
pub const MAT_MEDIUM: u32 = 9;
/// Water's refractive index.
pub const WATER_INDEX: f32 = 1.333;
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
/// The BVH traversal's stack: one entry for each level of interior nodes, at most (the
/// farther child, while the nearer is descended into). The host refuses trees deeper than
/// this (`Buffers::build`); a binary tree built by the surface area heuristic over a million
/// triangles is about 40 levels deep.
pub const STACK: usize = 64;
const PI: f32 = core::f32::consts::PI;
const INF: f32 = 3.0e38;
const NONE: u32 = 0xffff_ffff;

/// The scene's buffers.
pub struct Scene<'a> {
    pub params: &'a Params,
    /// The primitives, packed (`pack_prims`): what a ray is tested against first, three
    /// vectors each, then what shading needs of them, five each.
    pub prims: &'a [Vec4],
    pub nodes: &'a [Node],
    pub materials: &'a [Material],
    pub lights: &'a [Light],
    pub tables: &'a [f32],
    /// On the CPU, the shapes the kernels have no test for (`PRIM_HOST`), for shadow rays.
    #[cfg(not(target_arch = "spirv"))]
    pub host: Option<&'a dyn HostShapes>,
}

/// The host's shapes (`PRIM_HOST`), as a shadow ray asks after them: whether any lies along
/// the ray from `ro` in unit direction `rd` before `tmax`. A call to the host from the midst
/// of a kernel, so the CPU's alone; a device that can't make it refuses such scenes.
#[cfg(not(target_arch = "spirv"))]
pub trait HostShapes: Sync {
    fn occluded(&self, ro: Vec3, rd: Vec3, tmax: f32) -> bool;
}

/// How much work traversal does, counted on the CPU build only (the shader has nowhere to
/// keep it): box tests and primitive tests since `stats::take` was last called on this thread.
/// They are the same on the GPU, which runs this code; `gpu::tests::measure_traversal` reports
/// them per ray.
///
/// Behind the `stats` feature, which the renderer's tests turn on: a render on the CPU runs
/// this code too, and a counter in thread-local storage at every box was a tenth of its
/// traversal.
#[cfg(all(not(target_arch = "spirv"), feature = "stats"))]
pub mod stats {
    use std::cell::Cell;
    thread_local! {
        pub static BOXES: Cell<u64> = const { Cell::new(0) };
        pub static PRIMS: Cell<u64> = const { Cell::new(0) };
        /// Nanoseconds spent in `trace`, while `TIMED` is on (two clock readings a ray).
        pub static TRACE_NS: Cell<u64> = const { Cell::new(0) };
        pub static TIMED: Cell<bool> = const { Cell::new(false) };
    }
    /// Time traversal from here on (or stop), on this thread; `trace_seconds` reads it.
    pub fn time_traversal(on: bool) {
        TIMED.with(|t| t.set(on));
        TRACE_NS.with(|c| c.set(0));
    }
    pub fn trace_seconds() -> f64 {
        TRACE_NS.with(|c| c.get()) as f64 * 1e-9
    }
    /// (box tests, primitive tests) since the last call, on this thread.
    pub fn take() -> (u64, u64) {
        (BOXES.with(|c| c.replace(0)), PRIMS.with(|c| c.replace(0)))
    }
}

#[inline(always)]
fn count_box() {
    #[cfg(all(not(target_arch = "spirv"), feature = "stats"))]
    stats::BOXES.with(|c| c.set(c.get() + 1));
}

#[inline(always)]
fn count_prim() {
    #[cfg(all(not(target_arch = "spirv"), feature = "stats"))]
    stats::PRIMS.with(|c| c.set(c.get() + 1));
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

// ---- Sobol' points for a vertex's sampling decisions (the renderer's `sobol`: its
// documentation has the sources). A path's other numbers stay the hash's. ----

/// The bits of an index.
const SOBOL_BITS: usize = 32;

/// Direction numbers v_1 to v_32 of the first four dimensions, a dimension a component: the
/// first's 2^-k; then, by Joe and Kuo's degree s, polynomial a and initial values m,
/// (1, 0, [1]), (2, 1, [1, 3]) and (3, 1, [1, 3, 1]).
const SOBOL_DIRECTIONS: [[u32; 4]; SOBOL_BITS] = sobol_directions();

const fn sobol_directions() -> [[u32; 4]; SOBOL_BITS] {
    let mut v = [[0u32; 4]; SOBOL_BITS];
    v[0] = [0x8000_0000, 0x8000_0000, 0x8000_0000, 0x8000_0000];
    v[1] = [0x4000_0000, 0xc000_0000, 0xc000_0000, 0xc000_0000];
    v[2] = [0x2000_0000, 0xa000_0000, 0x6000_0000, 0x2000_0000];
    let mut k = 3;
    while k < SOBOL_BITS {
        v[k] = [
            v[k - 1][0] >> 1,
            v[k - 1][1] ^ (v[k - 1][1] >> 1),
            v[k - 2][2] ^ (v[k - 2][2] >> 2) ^ v[k - 1][2],
            v[k - 3][3] ^ (v[k - 3][3] >> 3) ^ v[k - 2][3],
        ];
        k += 1;
    }
    v
}

/// The `index`-th Sobol' point's first four coordinates, unscrambled: the XOR of each
/// dimension's direction numbers for the set bits of `index`, as the renderer's
/// `sobol::sobol` (whose tables are these by the byte).
pub fn sobol4(index: u32) -> UVec4 {
    let mut x = UVec4::ZERO;
    let mut k = 0;
    while k < SOBOL_BITS {
        if (index >> k) & 1 != 0 {
            let v = SOBOL_DIRECTIONS[k];
            x ^= UVec4::new(v[0], v[1], v[2], v[3]);
        }
        k += 1;
    }
    x
}

/// Nested uniform scrambling of `x` by `seed` (the renderer's `sobol::owen_scramble`).
pub fn owen_scramble(x: u32, seed: u32) -> u32 {
    let mut v = x.reverse_bits();
    v ^= v.wrapping_mul(0x3d20adea);
    v = v.wrapping_add(seed);
    v = v.wrapping_mul((seed >> 16) | 1);
    v ^= v.wrapping_mul(0x05526c56);
    v ^= v.wrapping_mul(0x53a22864);
    v.reverse_bits()
}

/// Vertices of a path whose decisions draw Sobol' points; deeper ones the hash's numbers, as
/// the renderer's samples do beyond `sobol::DIMENSIONS`: there stratification gains little.
pub const SOBOL_DEPTH: u32 = 6;
/// A vertex's groups of four dimensions: the next direction (and which lobe), a point on an
/// area light (and which light), and a direction to the sky with a point of the sun's disc.
pub const SOBOL_SCATTER: u32 = 0;
pub const SOBOL_LIGHT: u32 = 1;
pub const SOBOL_SKY: u32 = 2;

/// Four numbers in [0, 1) for a decision (`SOBOL_*`) at a path's vertex `depth`: for sample
/// `sample` of a pixel, point `sample` of a four-dimensional Sobol' sequence, Owen-scrambled
/// and shuffled by a seed of the pixel's and the decision's (Burley's padding, as the
/// renderer's `sobol::sample`), so a pixel's samples are stratified in each decision and
/// decisions are independent of each other. Deep in a path, the hash's numbers.
pub fn decide(s: &Scene, pixel: u32, sample: u32, depth: u32, which: u32, rng: &mut u32) -> Vec4 {
    if depth >= SOBOL_DEPTH {
        return Vec4::new(rand(rng), rand(rng), rand(rng), rand(rng));
    }
    let h = pcg(pcg(pixel ^ s.params.seed.x.wrapping_mul(0x9e3779b9)) ^ (depth * 3 + which + 1).wrapping_mul(0x85eb_ca6b));
    let x = sobol4(owen_scramble(sample, h));
    let scrambled = UVec4::new(owen_scramble(x.x, pcg(h ^ 1)), owen_scramble(x.y, pcg(h ^ 2)), owen_scramble(x.z, pcg(h ^ 3)), owen_scramble(x.w, pcg(h ^ 4)));
    Vec4::new((scrambled.x >> 8) as f32, (scrambled.y >> 8) as f32, (scrambled.z >> 8) as f32, (scrambled.w >> 8) as f32) * (1.0 / 16777216.0)
}

fn fract(x: f32) -> f32 {
    x - x.floor()
}

// ---- Geometry. ----

#[derive(Copy, Clone)]
struct Hit {
    t: f32,
    prim: u32,
}

/// A primitive's first vector, as the tests below name it.
struct TestRecord {
    v0: Vec4,
}

/// Vectors a primitive has in each part of the packed buffer: for the intersection test (a
/// triangle's first vertex and two edges, a sphere's centre and radius, a plane's height;
/// the kind in the second's w), and for shading (the other two vertices, with the material
/// and emitter + 1 in their w; three vertex normals, the first's w saying whether to use them).
pub const PRIM_TEST: u32 = 3;
pub const PRIM_SHADING: u32 = 5;
/// Indices are kept as floats there, exact up to this.
pub const MAX_PACKED_INDEX: u32 = 1 << 24;

/// A primitive whole again, from both parts: for shading, where the test's saving doesn't
/// matter and the original vertices do (an edge added back to a vertex isn't the vertex).
fn whole_prim(s: &Scene, i: u32) -> Prim {
    let count = s.params.counts.x + s.params.counts.y;
    let v0 = at(s.prims, PRIM_TEST * i);
    let kind = at(s.prims, PRIM_TEST * i + 1).w as u32;
    let at_shading = PRIM_TEST * count + PRIM_SHADING * i;
    let v1 = at(s.prims, at_shading);
    let v2 = at(s.prims, at_shading + 1);
    let n0 = at(s.prims, at_shading + 2);
    Prim {
        v0, v1: v1.xyz().extend(0.0), v2: v2.xyz().extend(0.0),
        n0: n0.xyz().extend(0.0), n1: at(s.prims, at_shading + 3), n2: at(s.prims, at_shading + 4),
        info: UVec4::new(kind, v1.w as u32, v2.w as u32, n0.w as u32),
    }
}

/// The buffer the shader reads primitives from: every primitive's test record, then every
/// primitive's shading record (`PRIM_TEST`, `PRIM_SHADING`). Edges are differences of the
/// vertices as the shader's f32, so the test computes what it did from the vertices.
#[cfg(not(target_arch = "spirv"))]
pub fn pack_prims(prims: &[Prim]) -> Vec<Vec4> {
    let mut packed = Vec::with_capacity(prims.len() * (PRIM_TEST + PRIM_SHADING) as usize);
    for p in prims {
        let triangle = p.info.x == PRIM_TRIANGLE;
        let e1 = if triangle { p.v1.xyz() - p.v0.xyz() } else { Vec3::ZERO };
        let e2 = if triangle { p.v2.xyz() - p.v0.xyz() } else { Vec3::ZERO };
        packed.extend([p.v0, e1.extend(p.info.x as f32), e2.extend(0.0)]);
    }
    for p in prims {
        packed.extend([p.v1.xyz().extend(p.info.y as f32), p.v2.xyz().extend(p.info.z as f32), p.n0.xyz().extend(p.info.w as f32), p.n1, p.n2]);
    }
    packed
}

/// (t, b1, b2, 1) for a hit within (tmin, tmax), else zero.
fn intersect_prim(s: &Scene, i: u32, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32) -> Vec4 {
    // The test's own record (`PRIM_TEST` vectors), not the whole primitive.
    let v0 = at(s.prims, PRIM_TEST * i);
    let e1 = at(s.prims, PRIM_TEST * i + 1);
    let kind = e1.w as u32;
    if kind == PRIM_TRIANGLE {
        let e2 = at(s.prims, PRIM_TEST * i + 2).xyz();
        let e1 = e1.xyz();
        let p = TestRecord { v0 };
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
        let p = TestRecord { v0 };
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
    } else if kind == PRIM_PLANE {
        if rd.y.abs() < 1e-12 {
            return Vec4::ZERO;
        }
        let t = (v0.x - ro.y) / rd.y;
        if t <= tmin || t >= tmax {
            return Vec4::ZERO;
        }
        Vec4::new(t, 0.0, 0.0, 1.0)
    } else if kind == PRIM_OCEAN {
        // The sea, from where the ray's range begins (as `Geometry::intersects_beyond`).
        let t = tmin + ocean_hit(s, ro + rd * tmin, rd);
        if t <= tmin || t >= tmax {
            return Vec4::ZERO;
        }
        Vec4::new(t, 0.0, 0.0, 1.0)
    } else {
        // A shape the host intersects (`PRIM_HOST`): never by this.
        Vec4::ZERO
    }
}

// ---- The sea: the renderer's `ocean::Ocean`, its shape. A sphere (the planet) or a plane,
// displaced by waves near a centre and smooth beyond, with the waves' slopes in its normal
// everywhere; the waves' crests sharpened by the choppy displacement, which moves the
// surface sideways: a point's height is its source's on the undisplaced grid (`wave_source`). The waves are tiles of heights and slopes in `tables`; a planet's distances are
// kept in the forms `excess` and `sphere_roots` give, as the air's are. ----

struct Ocean {
    base: u32,
    radius: f32,
    centre: Vec2,
    detail: f32,
    max_height: f32,
    max_slope: f32,
    tiles: u32,
    displaced: u32,
    lambda: f32,
    /// The planet's centre, and |c|² - r² (`excess`'s k for sea level).
    planet: Vec3,
    k: f32,
    /// The render's origin in the world, along x and z: where the waves' tiles are laid.
    shift: Vec2,
}

fn load_ocean(s: &Scene) -> Ocean {
    let base = s.params.dims.z - 1;
    Ocean {
        base,
        radius: at(s.tables, base + OCEAN_RADIUS),
        centre: Vec2::new(at(s.tables, base + OCEAN_CENTRE), at(s.tables, base + OCEAN_CENTRE + 1)),
        detail: at(s.tables, base + OCEAN_DETAIL),
        max_height: at(s.tables, base + OCEAN_MAX),
        max_slope: at(s.tables, base + OCEAN_MAX + 1),
        tiles: at(s.tables, base + OCEAN_TILES) as u32,
        displaced: at(s.tables, base + OCEAN_TILES + 1) as u32,
        lambda: at(s.tables, base + OCEAN_LAMBDA),
        planet: tv3(s, base + OCEAN_PLANET),
        k: at(s.tables, base + OCEAN_EXCESS),
        shift: Vec2::new(s.params.origin.x, s.params.origin.z),
    }
}

/// Where (x, z) lies in a tile of `n` texels a side over `size` metres, which repeats: the
/// four texels around it (at (0, 0), (1, 0), (0, 1) and (1, 1)) and how far it is between them.
fn tile_cell(size: f32, n: f32, x: f32, z: f32) -> (UVec4, Vec2) {
    let (u, v) = (x / size * n, z / size * n);
    let (u0, v0) = (u.floor(), v.floor());
    // (A tile is a power of two across, which the host sees to (`Ocean::export`): wrapped by
    // a mask, where six remainders were six divisions a lookup, and a march makes hundreds.)
    let n = n as i32;
    let i0 = u0 as i32 & (n - 1);
    let j0 = v0 as i32 & (n - 1);
    let i1 = (i0 + 1) & (n - 1);
    let j1 = (j0 + 1) & (n - 1);
    (UVec4::new((j0 * n + i0) as u32, (j0 * n + i1) as u32, (j1 * n + i0) as u32, (j1 * n + i1) as u32), Vec2::new(u - u0, v - v0))
}

/// `WaveTile::at`: a tile's height and slopes at (x, z), bilinear, the tile repeating.
fn wave_tile(s: &Scene, o: &Ocean, k: u32, x: f32, z: f32) -> Vec3 {
    let record = o.base + OCEAN_HEADER + OCEAN_TILE * k;
    let texels = o.base + at(s.tables, record + 2).to_bits();
    let (c, f) = tile_cell(at(s.tables, record), at(s.tables, record + 1), x, z);
    let t00 = tv3(s, texels + 3 * c.x);
    let t10 = tv3(s, texels + 3 * c.y);
    let t01 = tv3(s, texels + 3 * c.z);
    let t11 = tv3(s, texels + 3 * c.w);
    (t00 * (1.0 - f.x) + t10 * f.x) * (1.0 - f.y) + (t01 * (1.0 - f.x) + t11 * f.x) * f.y
}

/// `WaveTile::chop_at`: a tile's choppy displacement (D_x, D_z) at (x, z) of the undisplaced
/// grid, and its gradient (∂D_x/∂x, ∂D_z/∂z, ∂D_x/∂z).
fn chop_tile(s: &Scene, o: &Ocean, k: u32, x: f32, z: f32) -> (Vec2, Vec3) {
    let record = o.base + OCEAN_HEADER + OCEAN_TILE * k;
    let texels = o.base + at(s.tables, record + 3).to_bits();
    let (c, f) = tile_cell(at(s.tables, record), at(s.tables, record + 1), x, z);
    let w = Vec4::new((1.0 - f.x) * (1.0 - f.y), f.x * (1.0 - f.y), (1.0 - f.x) * f.y, f.x * f.y);
    let (mut d, mut j) = (Vec2::ZERO, Vec3::ZERO);
    let mut corner = 0;
    while corner < 4 {
        let (t, weight) = if corner == 0 { (c.x, w.x) } else if corner == 1 { (c.y, w.y) } else if corner == 2 { (c.z, w.z) } else { (c.w, w.w) };
        let i = texels + 5 * t;
        d += Vec2::new(at(s.tables, i), at(s.tables, i + 1)) * weight;
        j += tv3(s, i + 2) * weight;
        corner += 1;
    }
    (d, j)
}

/// `WaveField::source` and `squeeze`: where on the undisplaced grid the point now at (x, z)
/// came from, the choppy map p + λD(p) inverted by three rounds of fixed-point iteration; and
/// with `gradient`, the sum of the displacement's gradient there (a fourth round's reading).
fn wave_source(s: &Scene, o: &Ocean, x: f32, z: f32, gradient: bool) -> (Vec2, Vec3) {
    let here = Vec2::new(x, z);
    let mut p = here;
    let mut jacobian = Vec3::ZERO;
    if o.lambda != 0.0 {
        let rounds = if gradient { 4 } else { 3 };
        let mut round = 0;
        while round < rounds {
            let (mut d, mut j) = (Vec2::ZERO, Vec3::ZERO);
            let mut k = 0;
            while k < o.displaced {
                let (dk, jk) = chop_tile(s, o, k, p.x, p.y);
                d += dk;
                j += jk;
                k += 1;
            }
            if round < 3 {
                p = here - d * o.lambda;
            } else {
                jacobian = j;
            }
            round += 1;
        }
    }
    (p, jacobian)
}

/// `WaveField::at`: the height of the tiles that displace, and the slopes of all; the choppy
/// surface's, whose slopes (asked for with `slopes`; a march needs only heights) are the
/// grid's through the inverse of the map's Jacobian.
fn waves(s: &Scene, o: &Ocean, x: f32, z: f32, slopes: bool) -> Vec3 {
    let (p, j) = wave_source(s, o, x, z, slopes);
    let (mut w, mut ripples) = (Vec3::ZERO, Vec3::ZERO);
    let mut k = 0;
    while k < o.tiles {
        let q = if k < o.displaced { p } else { Vec2::new(x, z) };
        let v = wave_tile(s, o, k, q.x, q.y);
        if k < o.displaced {
            w += v;
        } else {
            ripples += Vec3::new(0.0, v.y, v.z);
        }
        k += 1;
    }
    if slopes && o.lambda != 0.0 {
        // ∇_x h = M⁻¹ ∇_p h for the symmetric M = I + λ∇D.
        let (a, d, b) = (1.0 + o.lambda * j.x, 1.0 + o.lambda * j.y, o.lambda * j.z);
        let det = (a * d - b * b).max(OCEAN_MIN_JACOBIAN);
        w = Vec3::new(w.x, (d * w.y - b * w.z) / det, (a * w.z - b * w.y) / det);
    }
    w + ripples
}

/// Height above the mean surface.
fn ocean_altitude(o: &Ocean, p: Vec3) -> f32 {
    if o.radius > 0.0 {
        excess(p, o.planet, o.k) / ((p - o.planet).length() + o.radius)
    } else {
        // (A flat sea's level is in its centre's y.)
        p.y - o.planet.y
    }
}

fn ocean_up(o: &Ocean, p: Vec3) -> Vec3 {
    if o.radius > 0.0 { (p - o.planet).normalize() } else { Vec3::Y }
}

/// `Ocean::above`: height above the displaced surface.
fn ocean_above(s: &Scene, o: &Ocean, p: Vec3) -> f32 {
    let altitude = ocean_altitude(o, p);
    let d = (Vec2::new(p.x, p.z) - o.centre).length() / o.detail;
    let a = 1.0 - smoothstep(0.5, 1.0, d);
    if a <= 0.0 {
        return altitude;
    }
    altitude - a * waves(s, o, p.x + o.shift.x, p.z + o.shift.y, false).x
}

/// `Ocean::below`: the stretch of the ray below altitude `level` (z is 0 if there is none).
fn ocean_below(o: &Ocean, ro: Vec3, rd: Vec3, level: f32) -> Vec3 {
    if o.radius > 0.0 {
        // |c|² - (r + level)².
        let k = o.k - (2.0 * o.radius * level + level * level);
        return match sphere_roots(ro, rd, o.planet, k) {
            Some((a, b)) => Vec3::new(a, b, 1.0),
            None => Vec3::ZERO,
        };
    }
    let level = level + o.planet.y;
    let t = (level - ro.y) / rd.y;
    if rd.y < 0.0 {
        Vec3::new(t, INF, 1.0)
    } else if rd.y > 0.0 {
        Vec3::new(-INF, t, 1.0)
    } else if ro.y < level {
        Vec3::new(-INF, INF, 1.0)
    } else {
        Vec3::ZERO
    }
}

/// `Ocean::detail_span`: where the ray is within the displaced region, horizontally.
fn ocean_detail_span(o: &Ocean, ro: Vec3, rd: Vec3) -> Vec3 {
    let from = Vec2::new(ro.x, ro.z) - o.centre;
    let d = Vec2::new(rd.x, rd.z);
    let a = d.dot(d);
    if a < 1e-18 {
        return if from.length() < o.detail { Vec3::new(-INF, INF, 1.0) } else { Vec3::ZERO };
    }
    let b = from.dot(d);
    let disc = b * b - a * (from.dot(from) - o.detail * o.detail);
    if disc < 0.0 {
        return Vec3::ZERO;
    }
    Vec3::new((-b - disc.sqrt()) / a, (-b + disc.sqrt()) / a, 1.0)
}

/// `Ocean::march`: the first crossing of the displaced surface from t0 to t1, from above or
/// below, stepping no further than the surface could come to meet the ray, then placing the
/// crossing between the step's ends; negative if there is none.
fn ocean_march(s: &Scene, o: &Ocean, ro: Vec3, rd: Vec3, t0: f32, t1: f32) -> f32 {
    let fade_rate = 1.5 / (0.5 * o.detail) * o.max_height;
    // One loop, and one place the surface is asked for (each is the choppy map inverted and
    // the tiles read, and rust-gpu makes a copy of all that wherever it is called): `x` is
    // where to ask next, first for the side the ray starts on, then a step at a time, and
    // once a step has crossed, between the two sides of the crossing.
    let mut x = t0 + 1e-4;
    let (mut t, mut f, mut side) = (x, 0.0, 1.0);
    // The crossing's bracket, and how many times it has been narrowed (0 while marching).
    let (mut lo, mut flo, mut hi, mut fhi) = (0.0, 0.0, 0.0, 0.0);
    let mut narrowed = 0;
    let mut started = false;
    let mut steps = 0;
    while steps < OCEAN_STEPS {
        let above = ocean_above(s, o, ro + rd * x);
        if !started {
            started = true;
            side = if above < 0.0 { -1.0 } else { 1.0 };
            f = above.abs();
        } else if narrowed == 0 {
            let g = side * above;
            if g <= 0.0 {
                // Crossed between t and x.
                lo = t;
                flo = f;
                hi = x;
                fhi = g;
                narrowed = 1;
            } else {
                t = x;
                f = g;
                steps += 1;
            }
        } else {
            // By false position, the end that stays put weighing half as much each time
            // (the Illinois rule, so neither end can stall): the surface is nearly straight
            // across one step, and four askings place the crossing as twenty halvings did.
            let g = side * above;
            if g > 0.0 {
                lo = x;
                flo = g;
                fhi *= 0.5;
            } else {
                hi = x;
                fhi = g;
                flo *= 0.5;
            }
            narrowed += 1;
        }
        if narrowed == 0 {
            if t >= t1 {
                break;
            }
            let up = ocean_up(o, ro + rd * t);
            let along = rd.dot(up);
            let descent = (-side * along).max(0.0);
            let horizontal = (1.0 - along * along).max(0.0).sqrt();
            let rate = descent + horizontal * (o.max_slope + fade_rate) + 1e-6;
            let step = (f / rate).max(0.01).min(t1 - t + 0.01);
            x = t + step;
        } else {
            let span = flo - fhi;
            x = if span > 0.0 { (lo + (hi - lo) * flo / span).clamp(lo, hi) } else { hi };
            if narrowed > 4 {
                return x;
            }
        }
    }
    -1.0
}

/// `Ocean::intersects`: the distance to the sea along unit rd, the nearer of the displaced
/// surface near the centre and the smooth one beyond; negative if the ray misses it.
fn ocean_hit(s: &Scene, ro: Vec3, rd: Vec3) -> f32 {
    let o = load_ocean(s);
    let h = o.max_height;
    let detail = ocean_detail_span(&o, ro, rd);
    // The smooth surface, where it is beyond the displaced region.
    let level = ocean_below(&o, ro, rd, 0.0);
    let mut smooth = -1.0;
    if level.z > 0.0 {
        let outside_a = detail.z == 0.0 || level.x < detail.x || level.x > detail.y;
        let outside_b = detail.z == 0.0 || level.y < detail.x || level.y > detail.y;
        if level.x > 0.0 && level.x < INF && outside_a {
            smooth = level.x;
        } else if level.y > 0.0 && level.y < INF && outside_b {
            smooth = level.y;
        }
    }
    // The displaced region: below the crests and above the troughs, near the centre.
    let mut displaced = -1.0;
    let crests = ocean_below(&o, ro, rd, h);
    if detail.z > 0.0 && crests.z > 0.0 {
        let troughs = ocean_below(&o, ro, rd, -h);
        let inner = if troughs.z > 0.0 && troughs.x > 0.0 && troughs.x < INF { troughs.x } else { INF };
        let start = detail.x.max(crests.x).max(0.0);
        let end = detail.y.min(crests.y).min(inner).min(start + 1e7);
        if start < end {
            displaced = ocean_march(s, &o, ro, rd, start, end);
        }
    }
    if displaced >= 0.0 && smooth >= 0.0 {
        displaced.min(smooth)
    } else if displaced >= 0.0 {
        displaced
    } else {
        smooth
    }
}

/// The sea's foam at a point of it (the foam of `PhysicalOcean::scatter`): where foam covers
/// the point (by one random number, over the fraction of it that foam covers), its albedo,
/// with w 1; else in x how much the bubbles under a breaking crest pale the water, with w 0.
fn sea_foam(s: &Scene, x: f32, z: f32, rng: &mut u32) -> Vec4 {
    let o = load_ocean(s);
    let (x, z) = (x + o.shift.x, z + o.shift.y);
    let place = at(s.tables, o.base + OCEAN_FOAM).to_bits();
    if place == 0 {
        return Vec4::ZERO;
    }
    let base = o.base + place;
    // `Foam::sample`: foam rides the water it formed in, on the undisplaced sea.
    let (p, _) = wave_source(s, &o, x, z, false);
    let tiles = at(s.tables, base + FOAM_TILES) as u32;
    let (mut clear_fresh, mut clear_residual) = (1.0, 1.0);
    let mut k = 0;
    while k < tiles {
        let record = base + FOAM_HEADER + 3 * k;
        let texels = base + at(s.tables, record + 2).to_bits();
        let (c, f) = tile_cell(at(s.tables, record + 1), at(s.tables, record), p.x, p.y);
        let w = Vec4::new((1.0 - f.x) * (1.0 - f.y), f.x * (1.0 - f.y), (1.0 - f.x) * f.y, f.x * f.y);
        let fresh = at(s.tables, texels + 2 * c.x) * w.x + at(s.tables, texels + 2 * c.y) * w.y + at(s.tables, texels + 2 * c.z) * w.z + at(s.tables, texels + 2 * c.w) * w.w;
        let residual = at(s.tables, texels + 2 * c.x + 1) * w.x + at(s.tables, texels + 2 * c.y + 1) * w.y + at(s.tables, texels + 2 * c.z + 1) * w.z + at(s.tables, texels + 2 * c.w + 1) * w.w;
        clear_fresh *= 1.0 - fresh;
        clear_residual *= 1.0 - residual;
        k += 1;
    }
    let (fresh, residual) = (1.0 - clear_fresh, 1.0 - clear_residual);
    let pattern = shared::sea::FoamPattern {
        wind: (at(s.tables, base + FOAM_WIND), at(s.tables, base + FOAM_WIND + 1)), speed: at(s.tables, base + FOAM_WIND + 2),
        lace_sd: at(s.tables, base + FOAM_WIND + 3), perm: base + FOAM_HEADER + 3 * tiles,
    };
    let tables = FloatTables(s.tables);
    let solid = pattern.ragged(&tables, fresh, (p.x, p.y));
    let lace = pattern.lace(&tables, residual, (p.x, p.y));
    let covered = solid.max(lace);
    if covered > 0.0 && rand(rng) < covered {
        let thin = if solid >= lace { 1.0 } else { RESIDUAL_FOAM };
        return (tv3(s, base) * thin).extend(1.0);
    }
    Vec4::new(fresh.max(0.5 * residual), 0.0, 0.0, 0.0)
}

/// `ocean_normal`, for the renderer's tests.
#[cfg(not(target_arch = "spirv"))]
pub fn sea_normal(s: &Scene, p: Vec3) -> Vec3 {
    ocean_normal(s, p)
}

/// `Ocean::normal_at`: the waves' slopes about the local vertical.
fn ocean_normal(s: &Scene, p: Vec3) -> Vec3 {
    let o = load_ocean(s);
    let up = ocean_up(&o, p);
    let east = (Vec3::X - up * up.x).normalize();
    let north = east.cross(up);
    let w = waves(s, &o, p.x + o.shift.x, p.z + o.shift.y, true);
    (up - east * w.y - north * w.z).normalize()
}

/// The geometric and shading normals of a hit.
fn normals(s: &Scene, i: u32, ro: Vec3, rd: Vec3, h: Vec4) -> (Vec3, Vec3) {
    let p = whole_prim(s, i);
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
    } else if kind == PRIM_OCEAN {
        let n = ocean_normal(s, ro + rd * h.x);
        (n, n)
    } else if kind == PRIM_HOST {
        // The host's to find, with the rest of its vertex.
        (-rd, -rd)
    } else {
        // A plane faces each ray, as the renderer's `Plane`.
        let n = if rd.y > 0.0 { -Vec3::Y } else { Vec3::Y };
        (n, n)
    }
}

/// The interval (near, far) narrowed by one axis's slab, entered and left at p and q (in
/// either order); unchanged if either is NaN.
fn slab(p: f32, q: f32, near: f32, far: f32) -> (f32, f32) {
    // (By comparisons, not `min` and `max`: those allow for a NaN, at several instructions
    // each on a CPU, and here there is none. They were a quarter of a mesh's traversal.)
    #[allow(clippy::eq_op)]
    if p == p && q == q {
        let (lo, hi) = if p < q { (p, q) } else { (q, p) };
        (if lo > near { lo } else { near }, if hi < far { hi } else { far })
    } else {
        (near, far)
    }
}

/// Where the ray enters the box, if it does before `tmax`; otherwise `INF`.
fn box_entry(bmin: Vec3, bmax: Vec3, ro: Vec3, inv: Vec3, tmax: f32) -> f32 {
    count_box();
    let t1 = (bmin - ro) * inv;
    let t2 = (bmax - ro) * inv;
    // Per axis; a ray parallel to a slab and starting on its face makes 0 × ∞ = NaN, and
    // limits nothing (as `bvh::TraversalRay::enters`). Unrolled: a component indexed by a
    // variable is a pointer chosen by branch, which logical SPIR-V can't have.
    let (n0, f0) = slab(t1.x, t2.x, -INF, INF);
    let (n1, f1) = slab(t1.y, t2.y, n0, f0);
    let (near, far) = slab(t1.z, t2.z, n1, f1);
    if far < 0.0 || near > far || near > tmax { INF } else { near }
}

/// The nearest hit beyond `tmin`, or with `any_hit`, whether anything lies before `tmax`: the
/// primitive (`NONE` for none) and (t, b1, b2, 1) as `intersect_prim` gives them. Its normals
/// are `normals`' to find, where it is shaded.
fn trace(s: &Scene, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, any_hit: bool) -> (u32, Vec4) {
    #[cfg(all(not(target_arch = "spirv"), feature = "stats"))]
    if stats::TIMED.with(|t| t.get()) {
        let start = std::time::Instant::now();
        let found = traverse(s, ro, rd, tmin, tmax, any_hit);
        stats::TRACE_NS.with(|c| c.set(c.get() + start.elapsed().as_nanos() as u64));
        return found;
    }
    traverse(s, ro, rd, tmin, tmax, any_hit)
}

#[inline(always)]
fn traverse(s: &Scene, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, any_hit: bool) -> (u32, Vec4) {
    let mut best = Hit { t: tmax, prim: NONE };
    let mut best_h = Vec4::ZERO;
    let np = s.params.counts.x;
    // One loop, and so one copy of `intersect_prim` once rust-gpu inlines everything: a leaf's
    // primitives j..end are tested first (to begin with, the planes, which follow the bounded
    // primitives), then the tree is descended: at each interior node into the nearer child
    // the ray enters, the farther going on the stack with the distance at which the ray
    // enters it, so that it can be dropped unvisited if something nearer has been hit by the
    // time it comes off.
    let mut j = np;
    let mut end = np + s.params.counts.y;
    let inv = Vec3::ONE / rd;
    let mut stack = [0u32; STACK];
    let mut entered = [0f32; STACK];
    let mut sp = 0usize;
    // The interior node to visit next, if there is one; the root, to begin with.
    let mut node = if np > 0 { 0 } else { NONE };
    loop {
        if j < end {
            count_prim();
            let h = intersect_prim(s, j, ro, rd, tmin, best.t);
            if h.w > 0.0 {
                best.t = h.x;
                best.prim = j;
                best_h = h;
                if any_hit {
                    return (best.prim, best_h);
                }
            }
            j += 1;
            continue;
        }
        // The next thing to do: the child chosen at the last node, or the stack's top.
        let mut next = node;
        node = NONE;
        if next == NONE {
            if sp == 0 {
                break;
            }
            sp -= 1;
            if entered[sp] >= best.t {
                continue;
            }
            next = stack[sp];
        }
        if next & LEAF != 0 {
            j = next & (MAX_LEAF_ITEMS - 1);
            end = j + ((next & !LEAF) >> (31 - LEAF_COUNT_BITS)) + 1;
            continue;
        }
        let n = at(s.nodes, next);
        let ta = if n.a == NO_CHILD { INF } else { box_entry(n.amin, n.amax, ro, inv, best.t) };
        let tb = if n.b == NO_CHILD { INF } else { box_entry(n.bmin, n.bmax, ro, inv, best.t) };
        let ca = child(n.a, n.a_count);
        let cb = child(n.b, n.b_count);
        // The nearer first (a, if they tie), the farther kept if the ray enters it at all.
        let a_first = ta <= tb;
        let near = if a_first { ca } else { cb };
        let far = if a_first { cb } else { ca };
        let t_near = if a_first { ta } else { tb };
        let t_far = if a_first { tb } else { ta };
        if t_far < INF && sp < STACK {
            stack[sp] = far;
            entered[sp] = t_far;
            sp += 1;
        }
        if t_near < INF {
            node = near;
        }
    }
    (best.prim, best_h)
}

/// The traversal alone, for the renderer's tests of it: the nearest hit's distance and
/// primitive (`u32::MAX` for none) in (tmin, tmax), or any hit's.
#[cfg(not(target_arch = "spirv"))]
pub fn nearest(s: &Scene, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, any_hit: bool) -> (f32, u32) {
    let (prim, h) = trace(s, ro, rd, tmin, tmax, any_hit);
    (if prim == NONE { tmax } else { h.x }, prim)
}

fn occluded(s: &Scene, ro: Vec3, rd: Vec3, tmax: f32) -> bool {
    #[cfg(not(target_arch = "spirv"))]
    if let Some(host) = s.host {
        if host.occluded(ro, rd, tmax) {
            return true;
        }
    }
    trace(s, ro, rd, 0.0, tmax, true).0 != NONE
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
    // sin θ from the direction itself, not back through the arccosine: a device's inverse
    // trigonometry is its own (Mesa's is a polynomial good to a part in 10^5), and through
    // this density it was in every pixel a sky lights. It still chooses the texel, which only
    // a direction on a texel's edge can tell.
    env_density(s, x, y, (1.0 - d.y * d.y).max(0.0).sqrt())
}

/// The density with which the sky's table gives a direction in its texel (x, y), sin θ from
/// the zenith.
fn env_density(s: &Scene, x: u32, y: u32, sin_theta: f32) -> f32 {
    let (w, h) = (s.params.offs2.y, s.params.offs2.z);
    if sin_theta <= 0.0 {
        return 0.0;
    }
    let marg = s.params.offs.w;
    let cond = s.params.offs2.x + y * (w + 1);
    let p_row = (at(s.tables, marg + y + 1) - at(s.tables, marg + y)) * h as f32;
    let p_col = (at(s.tables, cond + x + 1) - at(s.tables, cond + x)) * w as f32;
    p_row * p_col / (2.0 * PI * PI * sin_theta)
}
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
    // (Its density by the texel it was drawn in, not found again from the direction.)
    d.extend(env_density(s, x, y, (1.0 - d.y * d.y).max(0.0).sqrt()))
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

/// Where a point of the render's frame is in the world's: for what is tabulated there.
fn world(s: &Scene, p: Vec3) -> Vec3 {
    p + s.params.origin.xyz()
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
fn lut_slice(s: &Scene, a: &Air, slice: u32, place: (UVec4, Vec4)) -> (Vec3, Vec3, Vec3) {
    let base = a.lut + slice * LUT_WIDTH * LUT_HEIGHT * 9;
    let (t, w) = (place.0 + UVec4::splat(base), place.1);
    // Rayleigh, Mie and multiple scattering, each bilinear.
    (lut_texels(s, t, w, 0), lut_texels(s, t, w, 3), lut_texels(s, t, w, 6))
}

/// Where a direction is in a slice of the table: its four texels, from the slice's start, and
/// their weights. The same in every slice, so found once for the two a lookup reads (its
/// arcsine and arctangent were a tenth of a simple scene's render when found for each).
fn lut_place(d: Vec3) -> (UVec4, Vec4) {
    let l = d.y.clamp(0.0, 1.0).asin();
    let v = (l / (PI / 2.0)).sqrt() * (LUT_HEIGHT - 1) as f32;
    let u = fract(d.z.atan2(d.x) / (2.0 * PI)) * LUT_WIDTH as f32;
    let x0 = (u.floor() as u32) % LUT_WIDTH;
    let y0 = (v.floor() as u32).min(LUT_HEIGHT - 2);
    let (x1, y1) = ((x0 + 1) % LUT_WIDTH, y0 + 1);
    let (fx, fy) = (u - u.floor(), v - y0 as f32);
    let t = UVec4::new(9 * (y0 * LUT_WIDTH + x0), 9 * (y0 * LUT_WIDTH + x1), 9 * (y1 * LUT_WIDTH + x0), 9 * (y1 * LUT_WIDTH + x1));
    (t, Vec4::new((1.0 - fx) * (1.0 - fy), fx * (1.0 - fy), (1.0 - fx) * fy, fx * fy))
}

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
    let place = lut_place(d);
    let (r0, m0, s0) = lut_slice(s, a, i - 1, place);
    let (r1, m1, s1) = lut_slice(s, a, i, place);
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

/// `ocean::fresnel`: the reflectance of unpolarised light from air onto water.
fn water_fresnel(cos_i: f32) -> f32 {
    let cos_i = cos_i.clamp(0.0, 1.0);
    let sin_t = (1.0 - cos_i * cos_i).sqrt() / WATER_INDEX;
    let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
    let rs = (cos_i - WATER_INDEX * cos_t) / (cos_i + WATER_INDEX * cos_t);
    let rp = (WATER_INDEX * cos_i - cos_t) / (WATER_INDEX * cos_i + cos_t);
    (rs * rs + rp * rp) / 2.0
}

/// `PhysicalOcean::ripple_density`: Beckmann's distribution of the ripples' normals `m` about
/// `n`, of mean square slope `roughness`.
fn ripple_density(n: Vec3, m: Vec3, roughness: f32) -> f32 {
    let c = n.dot(m);
    if c <= 0.0 {
        return 0.0;
    }
    let tan2 = (1.0 - c * c) / (c * c);
    (-tan2 / roughness).exp() / (PI * roughness * c * c * c * c)
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

fn resolve_material(s: &Scene, m_in: u32, at_point: Vec3) -> u32 {
    // The squares are laid out in the world, not about the camera.
    let p = world(s, at_point);
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
    let p = whole_prim(s, i);
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
    let p = whole_prim(s, i);
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
    let p = whole_prim(s, i);
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
    /// The random numbers' state, `PATH_*` flags, the primitive its ray hits (`NONE` for
    /// none), once `intersect` has found it, and which of its pixel's samples the path is
    /// (for `decide`; while it waits in the host's queue, its slot, `gather_host`).
    pub state: UVec4,
    /// That hit: its distance and barycentric coordinates (`intersect_prim`).
    pub hit: Vec4,
}

/// `PathState::state.y`: still tracing; its last bounce was specular; it has scattered off a
/// rough surface (so regularisation applies); its light has been added to the image.
pub const PATH_ACTIVE: u32 = 1;
pub const PATH_SPECULAR: u32 = 2;
pub const PATH_SCATTERED: u32 = 4;
pub const PATH_ADDED: u32 = 8;
/// Waiting for the host: its ray hit a material the kernels have no code for (`MAT_HOST`), so
/// it is stopped at the hit, which it keeps (`hit`, `state.z`), and in the host's queue. The
/// host shades the vertex with the renderer's own material and queues the path again, or ends
/// it (the renderer's `gpu::host`).
pub const PATH_HOST: u32 = 16;
/// The last vertex counted the lights (area lights, the sun) wholly by sampling them, as some
/// of the host's materials do (the renderer's `LightSampling::Exclusive`): one the ray finds
/// adds nothing, though a sky does in full.
pub const PATH_LIGHTS_COUNTED: u32 = 32;
/// With `PATH_LIGHTS_COUNTED`: only the distant lights were (the renderer's
/// `LightSampling::Directional`, as the sea's glitter): an area light the ray finds counts in
/// full.
pub const PATH_DIRECTIONAL: u32 = 64;
/// From this bit of the flags up, eight bits: the medium the path last scattered in, plus one
/// (0 before it has scattered in any), which sets how its segments cross every medium
/// (`segment_scale`).
pub const PATH_MEDIUM_SHIFT: u32 = 8;
/// And eight more: how many times it has scattered in media, as the renderer counts them (a
/// first scattering reached off a surface counts two), up to 255.
pub const PATH_SCATTERINGS_SHIFT: u32 = 16;
/// Both.
pub const PATH_MEDIA: u32 = 0xffff << PATH_MEDIUM_SHIFT;
/// `PathState::state.z` of a path whose ray collides in a medium before it hits anything: this
/// plus the medium's number (and `hit.x` how far along).
pub const MEDIUM_HIT: u32 = 0xffff_ff00;
/// The most media a scene may hold.
pub const MAX_MEDIA: u32 = 255;

impl PathState {
    /// Pixel `index`'s path in pass `pass` from its camera ray.
    pub fn start(index: u32, pass: u32, seed: u32, ray: &CameraRay) -> PathState {
        let live = ray.d.w >= 0.5;
        // The ray's direction is unit already (normalised by the host, in f64): not normalised
        // again, which on the GPU (`start_paths`) would round differently on each backend.
        let d = if live { ray.d.truncate() } else { Vec3::ZERO };
        PathState {
            o: ray.o, d: d.extend(0.0), beta: Vec4::new(1.0, 1.0, 1.0, 0.0), l: Vec4::ZERO,
            state: UVec4::new(path_seed(index, pass, seed), if live { PATH_ACTIVE | PATH_SPECULAR } else { PATH_ADDED }, 0, pass),
            hit: Vec4::ZERO,
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

/// Samples of the air's shadow per camera ray (the renderer's `trace::CREPUSCULAR_SAMPLES`).
const CREPUSCULAR_SAMPLES: u32 = 2;

/// Crepuscular rays (the renderer's `SkyMaterial::shadowed_loss`): the part of the sunlight
/// scattered once towards `o` along the camera's ray `d`, as far as `distance`, that doesn't
/// arrive because the air there is in shadow, behind clouds or below mountains. The sky's
/// radiance and the aerial haze (`segment`) take the air to be lit everywhere; this is taken
/// from them. Steps of the ray are chosen in proportion to the light each scatters, a point
/// in each tested for the sun, and its shadow stands for its step's.
fn shadowed_loss<M: Media>(s: &Scene, o: Vec3, d: Vec3, distance: f32, flags_p: u32, rng: &mut u32, rays: &mut u32) -> Vec3 {
    let a = load_air(s, s.params.air.y);
    let (start, end) = match view_ray_span(&a, o, d) {
        Some((start, end)) if start < distance => (start, end.min(distance)),
        _ => return Vec3::ZERO,
    };
    let mu = d.dot(a.sun);
    let (rayleigh, mie) = (rayleigh_phase(mu), mie_phase(mu, a.g));
    let scale = a.brightness * SKY_CALIBRATION;
    // Each step's light (`Air::single_scattering_steps`), twice over: for the total, then to
    // find the steps the samples fall in.
    let mut total = 0.0;
    let (mut u0, mut u1) = (0.0, 0.0);
    let (mut t0, mut t1) = (Vec4::ZERO, Vec4::ZERO);
    let (mut c0, mut c1) = (Vec4::ZERO, Vec4::ZERO);
    let mut pass = 0;
    while pass < 2 {
        let mut depth = Vec3::ZERO;
        let mut k = 0;
        while k < SKY_STEPS {
            let (t, step) = span_sample(&a, o, d, start, end, SKY_STEPS, k);
            let p = o + d * t;
            let here = depths(&a, altitude(&a, p), step);
            depth += here * 0.5;
            let lit = transmittance(&a, depth + sun_depths_at(s, &a, p));
            let c = lit * (a.rayleigh * (here.x * rayleigh) + Vec3::splat(a.mie * here.y * mie)) * scale;
            depth += here * 0.5;
            let w = c.x + c.y + c.z;
            if pass == 0 {
                total += w;
            } else if w > 0.0 {
                // (The first step that takes a sample's number below zero is its own.)
                if u0 >= 0.0 {
                    u0 -= w;
                    if u0 < 0.0 {
                        t0 = Vec4::new(t, step, w, 0.0);
                        c0 = c.extend(0.0);
                    }
                }
                if u1 >= 0.0 {
                    u1 -= w;
                    if u1 < 0.0 {
                        t1 = Vec4::new(t, step, w, 0.0);
                        c1 = c.extend(0.0);
                    }
                }
            }
            k += 1;
        }
        if total <= 0.0 {
            return Vec3::ZERO;
        }
        u0 = rand(rng) * total;
        u1 = rand(rng) * total;
        pass += 1;
    }
    let mut loss = Vec3::ZERO;
    let sun = s.params.sun_dir.xyz();
    let mut j = 0;
    while j < CREPUSCULAR_SAMPLES {
        let (at_step, c) = if j == 0 { (t0, c0) } else { (t1, c1) };
        if at_step.z > 0.0 {
            let from = o + d * (at_step.x + (rand(rng) - 0.5) * at_step.y);
            *rays += 1;
            let mut lit = 0.0;
            if !occluded(s, from, sun, 1e30) {
                lit = 1.0;
                if M::ON {
                    // (As the camera's own rays cross the media: this is before the path
                    // scatters.)
                    lit = media_transmittance(s, from, sun, 1e30, flags_p, false, 1.0, rng);
                }
            }
            if lit < 1.0 {
                loss += c.xyz() * ((1.0 - lit) * total / at_step.z);
            }
        }
        j += 1;
    }
    loss / CREPUSCULAR_SAMPLES as f32
}

/// The path with a new ray, still tracing.
#[allow(clippy::too_many_arguments)]
fn carry_on(p: &PathState, o: Vec3, d: Vec3, beta: Vec3, l: Vec3, rays: u32, rng: u32, flags: u32, prev_pdf: f32, depth: u32, q0: SkyQuery, q1: SkyQuery) -> (PathState, SkyQuery, SkyQuery) {
    (PathState { o: o.extend(p.o.w), d: d.extend(prev_pdf), beta: beta.extend(depth as f32), l: l.extend(rays as f32), state: UVec4::new(rng, flags, 0, p.state.w), hit: Vec4::ZERO }, q0, q1)
}

/// The path, ended at this vertex (its sky queries still to be answered).
#[allow(clippy::too_many_arguments)]
fn finish(p: &PathState, o: Vec3, d: Vec3, beta: Vec3, l: Vec3, rays: u32, rng: u32, flags: u32, prev_pdf: f32, depth: u32, q0: SkyQuery, q1: SkyQuery) -> (PathState, SkyQuery, SkyQuery) {
    carry_on(p, o, d, beta, l, rays, rng, flags & !PATH_ACTIVE, prev_pdf, depth, q0, q1)
}

/// One vertex of a path (`PathState`): trace its ray; at what it hits, haze, emission, direct
/// light (asking the sky for its light sample), and the next ray, or the end. Returns the path
/// and its two sky queries.
pub fn bounce(s: &Scene, p: &PathState, pixel: u32) -> (PathState, SkyQuery, SkyQuery) {
    shade::<WithMedia>(s, &intersect::<WithMedia>(s, p), pixel)
}

/// Whether a kernel has the media's code. A scene without media runs kernels compiled
/// without it (`NoMedia`): the clouds' density and the walks through them are a large part of
/// a kernel that has them, which every device would compile, and which a browser's software
/// device (SwiftShader, in the tests) doesn't survive compiling beside the rest. A constant of
/// the type, so the compiler drops the code rather than the device skipping it.
pub trait Media {
    const ON: bool;
}

pub struct WithMedia;
pub struct NoMedia;

impl Media for WithMedia {
    const ON: bool = true;
}

impl Media for NoMedia {
    const ON: bool = false;
}

/// The path with what its ray hits found (`PathState::hit`, and the primitive in `state.z`):
/// the first of a bounce's two stages, and all of its closest-hit traversal.
pub fn intersect<M: Media>(s: &Scene, p: &PathState) -> PathState {
    intersect_media::<M>(s, &intersect_surface(s, p))
}

/// The surface the path's ray hits, of those the kernels test.
pub fn intersect_surface(s: &Scene, p: &PathState) -> PathState {
    let (prim, h) = trace(s, p.o.xyz(), p.d.xyz(), 0.0, INF, false);
    let mut found = *p;
    found.state.z = prim;
    found.hit = h;
    found
}

/// The path's walk through the media as far as the surface `found` for it: a collision
/// before the surface is the vertex instead.
pub fn intersect_media<M: Media>(s: &Scene, found: &PathState) -> PathState {
    let mut found = *found;
    if M::ON {
        let mut rng = found.state.x;
        let reach = if found.state.z == NONE { INF } else { found.hit.x };
        let (t, m) = media_collision(s, found.o.xyz(), found.d.xyz(), reach, found.state.y, &mut rng);
        found.state.x = rng;
        if m != NONE {
            found.state.z = MEDIUM_HIT + m;
            found.hit = Vec4::new(t, 0.0, 0.0, 0.0);
        }
    }
    found
}

/// The second stage, at what `intersect` found: haze, emission, direct light (its shadow rays
/// traced here, and the sky asked for its light sample), and the next ray, or the end.
pub fn shade<M: Media>(s: &Scene, p: &PathState, pixel: u32) -> (PathState, SkyQuery, SkyQuery) {
    let (ro, rd, prev_pdf) = (p.o.xyz(), p.d.xyz(), p.d.w);
    let mut beta = p.beta.xyz();
    let depth = p.beta.w as u32;
    let mut l = p.l.xyz();
    let mut rays = p.l.w as u32;
    let mut rng = p.state.x;
    let flags_p = p.state.y;
    let specular = flags_p & PATH_SPECULAR != 0;
    let counted = flags_p & PATH_LIGHTS_COUNTED != 0;
    let (mut q0, mut q1) = (SkyQuery::NONE, SkyQuery::NONE);
    let max_depth = s.params.dims.w;
    let flags = s.params.offs2.w;
    let n_emitters = s.params.counts.z;

    let hit = Hit { t: p.hit.x, prim: p.state.z };
    rays += 1;
    // Crepuscular rays: the camera's ray alone, as far as what it meets. (Taken from the
    // sample whatever the path goes on to find: a sample can be less than nothing.)
    if depth == 0 && flags & FLAG_CREPUSCULAR != 0 {
        let reach = if hit.prim == NONE { INF } else { hit.t };
        if reach > AERIAL_FROM {
            l -= beta * shadowed_loss::<M>(s, ro, rd, reach, flags_p, &mut rng, &mut rays);
        }
    }
    if hit.prim == NONE {
        // The sky, weighted against sampling it if the last vertex did, and the sun's disc.
        let mut w = 1.0;
        if !specular && !counted && flags & FLAG_SAMPLE_ENV != 0 {
            w = power_heuristic(prev_pdf, env_pdf(s, rd));
        }
        q0 = SkyQuery::sky(ro, rd, beta * w);
        if flags & FLAG_SUN != 0 {
            let ws = if specular { 1.0 } else if counted { 0.0 } else { power_heuristic(prev_pdf, sun_disc_pdf(s, rd)) };
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
    // A collision in a medium: a vertex with no surface, which scatters by a phase function.
    let in_medium = M::ON && hit.prim >= MEDIUM_HIT;
    let medium = s.params.media.x + (hit.prim & 0xff) * MEDIUM_SIZE;
    let (mut hit_ng, mut hit_ns) = (-rd, -rd);
    let mut mat = Material { kind: MAT_MEDIUM, a: 0, b: 0, pad: 0, c0: Vec4::ZERO, c1: Vec4::ZERO, p: Vec4::ZERO };
    if !in_medium {
        let (g, n) = normals(s, hit.prim, ro, rd, p.hit);
        hit_ng = g;
        hit_ns = n;
        let prim = whole_prim(s, hit.prim);
        let m = resolve_material(s, prim.info.y, point);
        mat = at(s.materials, m);
        if mat.kind == MAT_EMITTER {
            let le = emitted(s, m, hit_ns, -rd);
            let mut w = 1.0;
            if !specular && flags_p & PATH_DIRECTIONAL == 0 && flags & FLAG_SAMPLE_LIGHTS != 0 && prim.info.z > 0 {
                let lp = emitter_probability(s, prim.info.z - 1) * light_pdf(s, hit.prim, ro, point, hit_ng);
                w = if counted { 0.0 } else { power_heuristic(prev_pdf, lp) };
            }
            l += beta * le * w;
            return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
        }
    }
    if mat.kind == MAT_HOST {
        // Stopped here for the host, with the hit and the haze before it.
        let (mut parked, a, b) = finish(p, ro, rd, beta, l, rays, rng, flags_p | PATH_HOST, prev_pdf, depth, q0, q1);
        parked.state.z = hit.prim;
        parked.hit = p.hit;
        return (parked, a, b);
    }
    let wo = -rd;
    let ng = if hit_ng.dot(wo) < 0.0 { -hit_ng } else { hit_ng };
    let ns = if hit_ns.dot(wo) < 0.0 { -hit_ns } else { hit_ns };
    let eps = offset_for(s, point);
    // (A medium has no surface to stand off.)
    let above = if in_medium { point } else { point + ng * eps };
    // In a medium: all its scattering if the camera sees this vertex directly (through
    // mirrors and glass too), and without its forward peak once the path has scattered in a
    // medium or left a surface (delta-M truncation), its albedo that of what is left.
    // And from as many scatterings as the similarity relations begin at, isotropically, with
    // the albedo they give (`peak` 1 then: none of the phase function's shape is left).
    let mut scatterings = (flags_p >> PATH_SCATTERINGS_SHIFT) & 0xff;
    // The Oz approximation (the renderer's `VolumeMaterial::oz_sunlight`): at the path's first
    // scattering in any medium the sun is a sum of octaves, each through thinner media by a
    // broader phase function; at later ones it has been counted.
    let oz = if M::ON && in_medium { at(s.tables, medium + MEDIUM_OZ) as u32 } else { 0 };
    let sun_counted = oz > 0 && scatterings > 0;
    let oz_by = if oz > 0 {
        Vec4::new(at(s.tables, medium + MEDIUM_OZ + 1), at(s.tables, medium + MEDIUM_OZ + 2), at(s.tables, medium + MEDIUM_OZ + 3), at(s.tables, medium + MEDIUM_MEAN_COSINE + 1))
    } else {
        Vec4::ONE
    };
    let mut peak = 0.0;
    let mut medium_albedo = Vec3::ZERO;
    if M::ON && in_medium {
        if scatterings == 0 && flags_p & PATH_SCATTERED != 0 && at(s.tables, medium + MEDIUM_SCALE) != 1.0 {
            scatterings = 1;
        }
        if scatterings >= 1 {
            peak = at(s.tables, medium + MEDIUM_PEAK);
        }
        let a = medium + MEDIUM_ALBEDO;
        let whole = Vec3::new(at(s.tables, a), at(s.tables, a + 1), at(s.tables, a + 2));
        medium_albedo = whole * (1.0 - peak) / (Vec3::ONE - whole * peak);
        if scatterings as f32 >= at(s.tables, medium + MEDIUM_REDUCED_FROM) {
            let g = at(s.tables, medium + MEDIUM_MEAN_COSINE + scatterings.min(1));
            medium_albedo = medium_albedo * (1.0 - g) / (Vec3::ONE - medium_albedo * g);
            peak = 1.0;
        }
    }
    let floor_alpha = if flags_p & PATH_SCATTERED != 0 { s.params.misc.x } else { 0.0 };

    // Delta bounces: mirrors, glass, smooth coats.
    let mut delta = false;
    let mut next_dir = Vec3::ZERO;
    let mut next_from = above;
    let mut attenuate = Vec3::ONE;
    let mut lobe = Lobe::new(0, ns);
    // The sea (`PhysicalOcean::scatter`): the water body as a diffuse surface under what the
    // surface lets in, the sun's glitter through the ripples, and one reflection from a
    // sampled ripple. Its lights are counted by sampling them alone (no sky sample: the
    // reflection finds the sky).
    let mut water = mat.kind == MAT_WATER;
    let water_up = if water && mat.p.x > 0.0 { ocean_up(&load_ocean(s), point) } else { Vec3::Y };
    // Seen from beyond a wave's slope: the mean surface.
    let water_n = if hit_ns.dot(wo) <= 0.0 { water_up } else { hit_ns };
    // Whitecaps: foam over a fraction of the point, chosen at random, scattering diffusely
    // about the mean surface, as any diffuse surface does; between the patches, bubbles under
    // the surface pale the water.
    let mut foamed = false;
    let mut water_body = mat.c0.xyz();
    if water {
        let foam = sea_foam(s, point.x, point.z, &mut rng);
        if foam.w > 0.0 {
            water = false;
            foamed = true;
            lobe = Lobe::new(0, if water_up.dot(wo) > 0.0 { water_up } else { -water_up });
            lobe.albedo = foam.xyz();
        } else {
            water_body += mat.c1.xyz() * foam.x;
        }
    }
    if water {
        lobe = Lobe::new(0, water_n);
        lobe.albedo = water_body * (1.0 - water_fresnel(water_n.dot(wo)));
    } else if foamed {
        // (Its lobe is set.)
    } else if mat.kind == MAT_DIFFUSE {
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
        let n = hit_ns;
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
        return carry_on(p, next_from, next_dir.normalize(), beta, l, rays, rng, (flags_p | PATH_SPECULAR) & !(PATH_LIGHTS_COUNTED | PATH_DIRECTIONAL), prev_pdf, depth + 1, q0, q1);
    }

    let mut flags_p = (flags_p | PATH_SCATTERED) & !(PATH_SPECULAR | PATH_LIGHTS_COUNTED | PATH_DIRECTIONAL);
    if in_medium {
        // Light leaving this vertex, towards the lights and onwards, has scattered here.
        flags_p = (flags_p & !PATH_MEDIA) | (((hit.prim & 0xff) + 1) << PATH_MEDIUM_SHIFT) | ((scatterings + 1).min(255) << PATH_SCATTERINGS_SHIFT);
    }
    // A medium's lights, like the sea's, are counted by sampling them alone, and the sun as a
    // point; the sky is found by the path's next ray.
    let exclusive = water || in_medium;
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
            if flags & FLAG_SUN != 0 && !sun_counted {
                let sd = s.params.sun_dir.xyz();
                let solid_angle = sun_solid_angle(s);
                if in_medium || lobe.pdf(wo, sd) * solid_angle < POINT_SUN {
                    wi = sd;
                    li = s.params.sun_irr.xyz() * sun_transmittance(s, above);
                } else {
                    // Uniformly in the cone, by 1 - cos θ (as `sun_angle`, without the
                    // rounding of cos θ near 1).
                    let u = decide(s, pixel, p.state.w, depth, SOBOL_SKY, &mut rng);
                    let one_minus = u.z * solid_angle / (2.0 * PI);
                    let cos_t = 1.0 - one_minus;
                    let sin_t = (one_minus * (2.0 - one_minus)).max(0.0).sqrt();
                    let phi = 2.0 * PI * u.w;
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
                let u = decide(s, pixel, p.state.w, depth, SOBOL_LIGHT, &mut rng);
                let e = search(s, s.params.offs.y, n_emitters, u.z);
                let pmf = emitter_probability(s, e);
                let pi = at(s.tables, s.params.offs.x + e).to_bits();
                let (y, ny, light_pdf) = sample_light(s, pi, above, u.x, u.y);
                let to = y - above;
                let d = to.length();
                wi = to / d;
                dist = d * (1.0 - 1e-4) - eps;
                if light_pdf > 0.0 {
                    pdf = pmf * light_pdf;
                    li = emitted(s, whole_prim(s, pi).info.y, ny, -wi);
                }
            }
        } else if !exclusive && flags & FLAG_SAMPLE_ENV != 0 {
            // The sky, by its brightness: its light found by the `sky` kernel (`q1`).
            let u = decide(s, pixel, p.state.w, depth, SOBOL_SKY, &mut rng);
            let es = env_sample(s, u.x, u.y);
            if es.w > 0.0 {
                wi = es.xyz();
                pdf = es.w;
                li = Vec3::ONE;
            }
        }
        if li.dot(li) > 0.0 {
            let (mut f, lobe_pdf) = lobe.eval_pdf(wo, wi);
            let mut w = if pdf > 0.0 { power_heuristic(pdf, lobe_pdf) / pdf } else { 1.0 };
            if exclusive && pdf > 0.0 {
                // Light sampling alone.
                w = 1.0 / pdf;
            }
            if M::ON && in_medium {
                f = medium_albedo * medium_phase(s.tables, medium, peak, rd.dot(wi));
            }
            if water {
                // With the sun, its glitter: F D / (4 cos θv).
                if k == n_lights && wi.dot(water_up) > 0.0 && water_n.dot(wi) > 0.0 {
                    let h = (wi + wo).normalize();
                    let glint = water_fresnel(h.dot(wo)) * ripple_density(water_n, h, mat.p.y) / (4.0 * water_n.dot(wo));
                    if glint > 1e-6 {
                        f += Vec3::splat(glint);
                    }
                }
            }
            if f.dot(f) > 0.0 {
                rays += 1;
                if !occluded(s, above, wi, dist) {
                    // Past the media in the way; the sun, with Oz, an octave at a time.
                    if M::ON {
                        let octaves = if k == n_lights && oz > 0 { oz } else { 1 };
                        let (mut thinned, mut share) = (1.0, 1.0);
                        let mut g = oz_by.w;
                        let mut sum = Vec3::ZERO;
                        let mut octave = 0;
                        while octave < octaves {
                            let through = media_transmittance(s, above, wi, dist, flags_p, true, thinned, &mut rng);
                            let fi = if octave == 0 { f } else { medium_albedo * shared::phase::henyey_greenstein(rd.dot(wi), g) };
                            sum += fi * (share * through);
                            thinned *= oz_by.x;
                            share *= oz_by.y;
                            g *= oz_by.z;
                            octave += 1;
                        }
                        f = sum;
                    }
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
    if M::ON && in_medium {
        // By the phase function, which the direction is drawn in proportion to.
        let wi = medium_scatter(s.tables, medium, peak, rd, &mut rng);
        if !survives(s, depth, medium_albedo, &mut beta, &mut rng) {
            return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
        }
        beta *= medium_albedo;
        return carry_on(p, point, wi, beta, l, rays, rng, flags_p | PATH_LIGHTS_COUNTED, 0.0, depth + 1, q0, q1);
    }
    if water {
        // A ripple's normal about the surface's (`sample_ripple`: Gaussian slopes), and the
        // reflection from it, weighted by Fresnel's equations.
        let r = (-mat.p.y * (1.0 - rand(&mut rng)).ln()).sqrt();
        let phi = 2.0 * PI * rand(&mut rng);
        let t = if water_n.x.abs() < 0.9 { Vec3::X } else { Vec3::Z };
        let a = (t - water_n * water_n.dot(t)).normalize();
        let b = water_n.cross(a);
        let m = (water_n - a * (r * phi.cos()) - b * (r * phi.sin())).normalize();
        let reflected = rd - m * 2.0 * m.dot(rd);
        let weight = Vec3::splat(water_fresnel(m.dot(wo).max(0.0)));
        if !(reflected.dot(water_up) > 0.0 && m.dot(wo) > 0.0) || !survives(s, depth, weight, &mut beta, &mut rng) {
            return finish(p, ro, rd, beta, l, rays, rng, flags_p, prev_pdf, depth, q0, q1);
        }
        beta *= weight;
        // From where the renderer's reflection leaves: the hit, off the surface by its bias,
        // then a millimetre up. Not `above`, whose offset is some ten times that: these rays
        // skim the waves, and how many meet the next one depends on where they start. (Both
        // at least what f32 resolves at this point, for a sea a planet wide.)
        let reach = 2e-6 * (1.0 + point.abs().max_element());
        let from = point + ng * s.params.misc.z.max(reach) + water_up * reach.max(1e-3);
        return carry_on(p, from, reflected.normalize(), beta, l, rays, rng, flags_p | PATH_LIGHTS_COUNTED, 0.0, depth + 1, q0, q1);
    }
    let u = decide(s, pixel, p.state.w, depth, SOBOL_SCATTER, &mut rng);
    let wi = lobe.sample(wo, u.x, u.y, u.z);
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
    let ray = CameraRay { o: ro.extend(1.0), d: rd.normalize().extend(1.0) };
    let mut p = PathState::start(0, 0, 0, &ray);
    p.state.x = *rng;
    // Its decisions' points (`decide`) scrambled by the generator's state, so that calls with
    // one generator after another are independent samples.
    let pixel = pcg(*rng ^ 0x5bd1_e995);
    loop {
        let (next, q0, q1) = bounce(s, &p, pixel);
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
#[cfg(target_arch = "spirv")]
fn pixel(gid: UVec3, params: &Params) -> Option<usize> {
    let index = gid.x + gid.y * (65535 * 64);
    if index >= params.dims.x * params.dims.y { None } else { Some(index as usize) }
}

// ---- Clouds: the renderer's own density functions (the `shared` crate, generic over the
// float they compute in), compiled here at f32 with their tables read from `tables`. ----

/// A buffer of floats as the shared functions read their tables: texels as they are, and
/// integers (a permutation) as the floats that hold them exactly.
pub struct FloatTables<'a>(pub &'a [f32]);

impl shared::Tables<f32> for FloatTables<'_> {
    #[inline]
    fn value(&self, i: u32) -> f32 {
        at(self.0, i)
    }

    #[inline]
    fn index(&self, i: u32) -> u32 {
        at(self.0, i) as u32
    }
}

/// A flat Nubis cloud layer's record in a table (`shared::clouds::NubisField`; the renderer's
/// `NubisCloudLayer::export`): its recipe's seven numbers in their fields' order, its densest
/// extinction, its weather map's corner (x, z), texel and texels a side, and the square it
/// fills (least x and z, greatest x and z); then the shape noise's permutation (512 entries),
/// the Worley texture, the weather map's corners and its texels' maxima.
pub const NUBIS_HEADER: u32 = 16;
pub const NUBIS_PERMUTATION: u32 = 512;

/// The layer whose record begins at `base`.
pub fn load_nubis(tables: &[f32], base: u32) -> shared::clouds::NubisField<f32> {
    let worley = base + NUBIS_HEADER + NUBIS_PERMUTATION;
    let n = at(tables, base + 11) as u32;
    let corners = worley + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N;
    shared::clouds::NubisField {
        recipe: shared::clouds::Nubis {
            base: at(tables, base), top: at(tables, base + 1), coverage: at(tables, base + 2), cloud_type: at(tables, base + 3),
            shape_size: at(tables, base + 4), detail_size: at(tables, base + 5), erosion: at(tables, base + 6),
            perm: base + NUBIS_HEADER, worley,
        },
        max_density: at(tables, base + 7),
        weather: shared::clouds::WeatherGrid {
            x: at(tables, base + 8), z: at(tables, base + 9), texel: at(tables, base + 10), n,
            corners, maxima: corners + (n + 1) * (n + 1),
        },
    }
}

/// The cloud's density, as a fraction of its densest, at `p` under weather `weather`: the
/// renderer's `NubisCloudLayer::fraction_at`, the same functions at f32.
pub fn nubis_fraction(tables: &[f32], base: u32, p: Vec3, weather: f32) -> f32 {
    let layer = load_nubis(tables, base).recipe;
    match layer.height(p.y) {
        Some(h) => layer.fraction(&FloatTables(tables), h, weather, p.x, p.y, p.z),
        None => 0.0,
    }
}

/// A kernel's random numbers (`rand`), as the shared walks draw them.
pub struct Hashed(pub u32);

impl shared::Random<f32> for Hashed {
    #[inline]
    fn next(&mut self) -> f32 {
        rand(&mut self.0)
    }
}

/// How much light gets through the cloud layer along the ray from `tmin` to `tmax`: the
/// renderer's `Volume::transmittance` for a box of it, the same walk
/// (`shared::media::ratio_track`) at f32. One estimate, whose mean is exp(-optical depth).
pub fn nubis_transmittance(tables: &[f32], base: u32, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32, rng: &mut u32) -> f32 {
    let field = load_nubis(tables, base);
    // The stretch of the ray in the layer's box.
    let inv = Vec3::ONE / rd;
    let lo = Vec3::new(at(tables, base + 12), field.recipe.base, at(tables, base + 13));
    let hi = Vec3::new(at(tables, base + 14), field.recipe.top, at(tables, base + 15));
    let (a, b) = ((lo - ro) * inv, (hi - ro) * inv);
    let (near, far) = slab(a.x, b.x, tmin, tmax);
    let (near, far) = slab(a.y, b.y, near, far);
    let (near, far) = slab(a.z, b.z, near, far);
    if near >= far {
        return 1.0;
    }
    let mut random = Hashed(*rng);
    let medium = shared::clouds::NubisMedium(&field, &FloatTables(tables));
    let through = shared::media::ratio_track(&medium, (ro.x, ro.y, ro.z), (rd.x, rd.y, rd.z), near, far, 0.0, 1.0, 1.0, &mut random);
    *rng = random.0;
    match through {
        Some(t) => t,
        None => 0.0,
    }
}

/// How many estimates of the transmittance `probe_clouds` averages for a ray.
pub const PROBE_ESTIMATES: u32 = 64;

/// What `probe` is asked for: a Nubis layer's density, light through it along rays, or a layer
/// of altocumulus's density.
pub const PROBE_DENSITY: u32 = 0;
pub const PROBE_RAYS: u32 = 1;
pub const PROBE_ALTOCUMULUS: u32 = 2;
/// And of cirrus's.
pub const PROBE_CIRRUS: u32 = 3;
/// And of precipitation's, as a fraction of its greatest.
pub const PROBE_RAIN: u32 = 4;
/// And of mammatus's.
pub const PROBE_MAMMATUS: u32 = 5;
/// And of a cumulonimbus's.
pub const PROBE_CUMULONIMBUS: u32 = 6;

/// What `probe_clouds` computes for probe `i`: the density at a point (its w the weather),
/// or, with `PROBE_RAYS`, the mean of `PROBE_ESTIMATES` estimates of the transmittance along the ray
/// from `a` (its w how far) in direction `b`, from the random numbers `pcg(i)` begins.
pub fn probe(tables: &[f32], base: u32, i: u32, a: Vec4, b: Vec4, what: u32) -> f32 {
    if what == PROBE_ALTOCUMULUS {
        return load_benard(tables, base).fraction(&FloatTables(tables), a.x, a.y, a.z);
    }
    if what == PROBE_CIRRUS {
        return load_cirrus(tables, base).fraction(&FloatTables(tables), a.x, a.y, a.z);
    }
    if what == PROBE_CUMULONIMBUS {
        return load_cumulonimbus(tables, base).fraction(&FloatTables(tables), a.x, a.y, a.z).0;
    }
    if what == PROBE_MAMMATUS {
        return load_mammatus(tables, base).fraction(&FloatTables(tables), a.x, a.y, a.z);
    }
    if what == PROBE_RAIN {
        let rain = load_rain(tables, base);
        return rain.density(&FloatTables(tables), a.x, a.y, a.z) / rain.peak_extinction();
    }
    if what != PROBE_RAYS {
        return nubis_fraction(tables, base, a.xyz(), a.w);
    }
    let mut rng = pcg(i);
    let mut sum = 0.0;
    let mut k = 0;
    while k < PROBE_ESTIMATES {
        sum += nubis_transmittance(tables, base, a.xyz(), b.xyz(), 0.0, a.w, &mut rng);
        k += 1;
    }
    sum / PROBE_ESTIMATES as f32
}

/// The clouds on the device, for checking what the shared functions compute there against the
/// renderer's (`probe`): `counts.x` probes of the layer whose record is at `offs.x`, two
/// vectors each in `points`, the answer in the second's w; `counts.y` says what is asked
/// (`PROBE_*`). Not part of a render; the volume kernels will call these functions as this does.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn probe_clouds(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] points: &mut [Vec4],
) {
    if gid.x >= params.counts.x {
        return;
    }
    let i = gid.x as usize;
    let (a, b) = (points[2 * i], points[2 * i + 1]);
    points[2 * i + 1].w = probe(tables, params.offs.x, gid.x, a, b, params.counts.y);
}

// ---- Media in a render: a path's ray is tracked through each medium before the surface it
// hits (`media_collision`, in `intersect`), a collision is a vertex of the path (`shade`), and
// every shadow ray is dimmed by the media it crosses (`media_transmittance`). The walks, the
// densities and the phase functions are the renderer's own, from the `shared` crate; the
// vertex follows the renderer's `volume::VolumeMaterial::scatter`, which has the account of
// what it does and why. ----

/// A medium's record in `tables` (the renderer's `gpu::media_records`), `MEDIUM_SIZE` floats:
/// its kind; its box (least corner, greatest); its extinction (`MEDIUM_CONSTANT`) or the bits
/// of where its cloud layer's record is (`MEDIUM_NUBIS`, `MEDIUM_BENARD`, `MEDIUM_CIRRUS`, `MEDIUM_RAIN`, `MEDIUM_MAMMATUS`, `MEDIUM_CUMULONIMBUS`); the radius of the planet it curves
/// round, or 0 where it is flat; the factor on every medium's density for segments of a path
/// that last scattered here, and on its own for one that has left a surface (delta-M
/// truncation: the renderer's `Volume::truncated`); a lower bound on its extinction; its phase
/// function (`PHASE_*`: Henyey-Greenstein's g, or the droplets' blend's g, Draine's g and α,
/// and Draine's share); its albedo; the fraction of its scattering in the forward peak that
/// truncation takes as none; and for the similarity relations (the renderer's
/// `ScatteringApproximation::similarity_after`), that factor on densities once a path has
/// scattered as often as they begin at, that number, and the mean cosine of its scattering
/// whole and truncated; and the Oz approximation's octaves and factors (`MEDIUM_OZ`).
pub const MEDIUM_SIZE: u32 = 28;
pub const MEDIUM_BOX: u32 = 1;
pub const MEDIUM_DENSITY: u32 = 7;
pub const MEDIUM_RADIUS: u32 = 8;
pub const MEDIUM_SCALE: u32 = 9;
pub const MEDIUM_LOWER: u32 = 10;
pub const MEDIUM_PHASE: u32 = 11;
pub const MEDIUM_ALBEDO: u32 = 16;
pub const MEDIUM_PEAK: u32 = 19;
pub const MEDIUM_REDUCED_SCALE: u32 = 20;
pub const MEDIUM_REDUCED_FROM: u32 = 21;
pub const MEDIUM_MEAN_COSINE: u32 = 22;
/// The Oz approximation's octaves (0 without it), then its factors a octave on the extinction,
/// the weight and the mean cosine (the renderer's `volume::Oz`).
pub const MEDIUM_OZ: u32 = 24;
pub const MEDIUM_CONSTANT: u32 = 0;
pub const MEDIUM_NUBIS: u32 = 1;
pub const MEDIUM_BENARD: u32 = 2;
pub const MEDIUM_CIRRUS: u32 = 3;
pub const MEDIUM_RAIN: u32 = 4;
pub const MEDIUM_MAMMATUS: u32 = 5;
pub const MEDIUM_CUMULONIMBUS: u32 = 6;
pub const PHASE_HENYEY_GREENSTEIN: u32 = 0;
pub const PHASE_DROPLETS: u32 = 1;

/// A layer of altocumulus's record in a table (`shared::altocumulus::Benard`; the renderer's
/// `BenardAltocumulus::export`): its recipe's 23 numbers in their fields' order, and its seed's
/// bits, and its map of column tops (`Benard::tops`: its corner's x and z, its texel and its
/// texels a side, 0 for none); then the permutations of the lattice's warp and of the
/// patches (512 entries each), the Worley texture, and the map's tops.
pub const BENARD_HEADER: u32 = 28;

/// The layer whose record begins at `base`.
pub fn load_benard(tables: &[f32], base: u32) -> shared::altocumulus::Benard<f32> {
    // (No closure over `tables`: one that holds a slice is a struct naga refuses.)
    shared::altocumulus::Benard {
        base: at(tables, base), thickness: at(tables, base + 1), max_density: at(tables, base + 2), cell_size: at(tables, base + 3), coverage: at(tables, base + 4), regularity: at(tables, base + 5), anisotropy: at(tables, base + 6),
        wind: (at(tables, base + 7), at(tables, base + 8)), undulation: at(tables, base + 9), wavelength: at(tables, base + 10), patchiness: at(tables, base + 11), patch_size: at(tables, base + 12), warp: at(tables, base + 13),
        lumpiness: at(tables, base + 14), detail_size: at(tables, base + 15), erosion: at(tables, base + 16), center: (at(tables, base + 17), at(tables, base + 18)), extent: at(tables, base + 19), fade: at(tables, base + 20),
        castellanus: at(tables, base + 21), turret_height: at(tables, base + 22), seed: at(tables, base + 23).to_bits(),
        warp_perm: base + BENARD_HEADER, patch_perm: base + BENARD_HEADER + NUBIS_PERMUTATION,
        worley: base + BENARD_HEADER + 2 * NUBIS_PERMUTATION,
        tops: shared::clouds::WeatherGrid {
            x: at(tables, base + 24), z: at(tables, base + 25), texel: at(tables, base + 26), n: at(tables, base + 27) as u32,
            corners: 0,
            maxima: base + BENARD_HEADER + 2 * NUBIS_PERMUTATION + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N,
        },
    }
}

/// A layer of cirrus's record in a table (`shared::cirrus::Cirrus`; the renderer's
/// `HoganKewCirrus::export`): its recipe's 29 numbers in their fields' order; then the
/// permutations of the fibres' noise and of the flow's (512 entries each), and the generating
/// field.
pub const CIRRUS_HEADER: u32 = 29;

/// The layer whose record begins at `base`.
pub fn load_cirrus(tables: &[f32], base: u32) -> shared::cirrus::Cirrus<f32> {
    shared::cirrus::Cirrus {
        base: at(tables, base), top: at(tables, base + 1), max_density: at(tables, base + 2), generating_depth: at(tables, base + 3),
        fall_depth: at(tables, base + 4), sublimation: at(tables, base + 5), wind: (at(tables, base + 6), at(tables, base + 7)),
        shear: at(tables, base + 8), generating_wind: at(tables, base + 9), fall_speed: at(tables, base + 10),
        fall_speed_spread: at(tables, base + 11), veer: at(tables, base + 12), edge: at(tables, base + 13),
        flow_size: at(tables, base + 14), flow: at(tables, base + 15), fibre_width: at(tables, base + 16), fibres: at(tables, base + 17),
        turbulence_size: at(tables, base + 18), turbulence: at(tables, base + 19), drift: at(tables, base + 20),
        patchiness: at(tables, base + 21), patch_size: at(tables, base + 22), center: (at(tables, base + 23), at(tables, base + 24)),
        extent: at(tables, base + 25), fade: at(tables, base + 26), texel: at(tables, base + 27), threshold: at(tables, base + 28),
        detail_perm: base + CIRRUS_HEADER, curl_perm: base + CIRRUS_HEADER + NUBIS_PERMUTATION,
        field: base + CIRRUS_HEADER + 2 * NUBIS_PERMUTATION,
    }
}

/// Precipitation's record in a table (`shared::rain::Rain`; the renderer's
/// `Precipitation::export`): its recipe's 21 numbers in their fields' order (what falls as a
/// number, and whether it evaporates and melts as 0 or 1); then the permutations of the
/// cells' noise and of the streaks' (512 entries each).
pub const RAIN_HEADER: u32 = 21;

/// The precipitation whose record begins at `base`.
pub fn load_rain(tables: &[f32], base: u32) -> shared::rain::Rain<f32> {
    shared::rain::Rain {
        base: at(tables, base), ground: at(tables, base + 1), rain_rate: at(tables, base + 2), cell_size: at(tables, base + 3),
        coverage: at(tables, base + 4), fall_depth: at(tables, base + 5), shear: at(tables, base + 6),
        wind: (at(tables, base + 7), at(tables, base + 8)), streak_size: at(tables, base + 9), streakiness: at(tables, base + 10),
        base_variation: at(tables, base + 11), center: (at(tables, base + 12), at(tables, base + 13)), radius: at(tables, base + 14),
        fade: at(tables, base + 15), kind: at(tables, base + 16) as u32, hail_size: at(tables, base + 17),
        melting_level: at(tables, base + 18), evaporates: at(tables, base + 19) > 0.5, melts: at(tables, base + 20) > 0.5,
        cells: base + RAIN_HEADER, streaks: base + RAIN_HEADER + NUBIS_PERMUTATION,
    }
}

/// Mammatus's record in a table (`shared::mammatus::Mammatus`; the renderer's
/// `Mammatus::export`): its recipe's 29 numbers in their fields' order and its seed's bits;
/// then the permutation of its noise (512 entries).
pub const MAMMATUS_HEADER: u32 = 30;

/// The mammatus whose record begins at `base`.
pub fn load_mammatus(tables: &[f32], base: u32) -> shared::mammatus::Mammatus<f32> {
    shared::mammatus::Mammatus {
        base: at(tables, base), thickness: at(tables, base + 1), base_variation: at(tables, base + 2), base_size: at(tables, base + 3),
        sag: at(tables, base + 4), sag_distance: at(tables, base + 5), base_softness: at(tables, base + 6),
        top_variation: at(tables, base + 7), anvil_density: at(tables, base + 8), pouch_density: at(tables, base + 9),
        pouch_size: at(tables, base + 10), pouch_depth: at(tables, base + 11), coverage: at(tables, base + 12),
        regularity: at(tables, base + 13), clustering: at(tables, base + 14), cluster_size: at(tables, base + 15),
        edge_sharpness: at(tables, base + 16), roughness: at(tables, base + 17), detail_size: at(tables, base + 18),
        edge_direction: (at(tables, base + 19), at(tables, base + 20)), edge_distance: at(tables, base + 21),
        edge_raggedness: at(tables, base + 22), edge_taper: at(tables, base + 23), center: (at(tables, base + 24), at(tables, base + 25)),
        extent: at(tables, base + 26), fade: at(tables, base + 27), pouch_band: at(tables, base + 28),
        seed: at(tables, base + 29).to_bits(), perm: base + MAMMATUS_HEADER,
    }
}

/// A cumulonimbus's record in a table (`shared::cumulonimbus::Cumulonimbus`; the renderer's
/// `Cumulonimbus::export`): its recipe's 19 numbers in their fields' order and how many
/// towers it has; then the permutation of its noise (512 entries), the Worley texture of its
/// billows, and its towers (`TOWER_SIZE` numbers each).
pub const CUMULONIMBUS_HEADER: u32 = 20;

/// The cumulonimbus whose record begins at `base`.
pub fn load_cumulonimbus(tables: &[f32], base: u32) -> shared::cumulonimbus::Cumulonimbus<f32> {
    let worley = base + CUMULONIMBUS_HEADER + NUBIS_PERMUTATION;
    shared::cumulonimbus::Cumulonimbus {
        base: at(tables, base), tropopause: at(tables, base + 1), radius: at(tables, base + 2), lean: at(tables, base + 3),
        wind: (at(tables, base + 4), at(tables, base + 5)), upper_wind: at(tables, base + 6), outflow_speed: at(tables, base + 7),
        anvil_thickness: at(tables, base + 8), glaciation: at(tables, base + 9), density: at(tables, base + 10),
        ice_density: at(tables, base + 11), billowing: at(tables, base + 12), softness: at(tables, base + 13),
        billow: at(tables, base + 14), anvil_length: at(tables, base + 15), anvil_back: at(tables, base + 16),
        bottom: at(tables, base + 17), top: at(tables, base + 18), tower_count: at(tables, base + 19) as u32,
        perm: base + CUMULONIMBUS_HEADER, worley,
        towers: worley + shared::clouds::WORLEY_N * shared::clouds::WORLEY_N * shared::clouds::WORLEY_N,
    }
}

/// A medium as the shared walks take one.
pub struct KernelMedium<'a> {
    pub tables: FloatTables<'a>,
    pub kind: u32,
    pub sigma: f32,
    pub radius: f32,
    pub nubis: shared::clouds::NubisField<f32>,
    pub benard: shared::altocumulus::Benard<f32>,
    pub cirrus: shared::cirrus::Cirrus<f32>,
    pub rain: shared::rain::Rain<f32>,
    pub mammatus: shared::mammatus::Mammatus<f32>,
    pub cumulonimbus: shared::cumulonimbus::Cumulonimbus<f32>,
}

/// A medium's cloud layer in its own flat coordinates, of whichever kind: so that what is
/// around it (the planet's curve) is compiled once.
struct KernelLayer<'a> {
    tables: &'a FloatTables<'a>,
    kind: u32,
    nubis: &'a shared::clouds::NubisField<f32>,
    benard: &'a shared::altocumulus::Benard<f32>,
    cirrus: &'a shared::cirrus::Cirrus<f32>,
    rain: &'a shared::rain::Rain<f32>,
    mammatus: &'a shared::mammatus::Mammatus<f32>,
    cumulonimbus: &'a shared::cumulonimbus::Cumulonimbus<f32>,
}

impl shared::media::Medium<f32> for KernelLayer<'_> {
    fn density(&self, x: f32, y: f32, z: f32) -> f32 {
        if self.kind == MEDIUM_BENARD {
            self.benard.max_density * self.benard.fraction(self.tables, x, y, z)
        } else if self.kind == MEDIUM_CIRRUS {
            self.cirrus.max_density * self.cirrus.fraction(self.tables, x, y, z)
        } else if self.kind == MEDIUM_RAIN {
            self.rain.density(self.tables, x, y, z)
        } else if self.kind == MEDIUM_MAMMATUS {
            self.mammatus.density(self.tables, x, y, z)
        } else if self.kind == MEDIUM_CUMULONIMBUS {
            self.cumulonimbus.extinction(self.tables, x, y, z)
        } else {
            self.nubis.density(self.tables, x, y, z)
        }
    }

    fn bound(&self) -> f32 {
        if self.kind == MEDIUM_BENARD { self.benard.max_density } else if self.kind == MEDIUM_CIRRUS { self.cirrus.max_density } else if self.kind == MEDIUM_RAIN { self.rain.peak_extinction() } else if self.kind == MEDIUM_MAMMATUS { self.mammatus.bound() } else if self.kind == MEDIUM_CUMULONIMBUS { self.cumulonimbus.bound() } else { self.nubis.max_density }
    }

    fn majorant_from(&self, x: f32, y: f32, z: f32, dx: f32, dy: f32, dz: f32) -> (f32, f32) {
        if self.kind == MEDIUM_BENARD {
            self.benard.majorant_from(self.tables, x, y, z, dx, dy, dz)
        } else if self.kind == MEDIUM_CIRRUS {
            self.cirrus.majorant_from(y, dy)
        } else if self.kind == MEDIUM_RAIN {
            self.rain.majorant_from(y, dy)
        } else if self.kind == MEDIUM_MAMMATUS {
            self.mammatus.majorant_from(x, y, z, dx, dy, dz)
        } else if self.kind == MEDIUM_CUMULONIMBUS {
            self.cumulonimbus.majorant_from(y, dy)
        } else {
            self.nubis.majorant_from(self.tables, x, y, z, dx, dy, dz)
        }
    }
}

impl<'a> KernelMedium<'a> {
    fn layer(&'a self) -> KernelLayer<'a> {
        KernelLayer { tables: &self.tables, kind: self.kind, nubis: &self.nubis, benard: &self.benard, cirrus: &self.cirrus, rain: &self.rain, mammatus: &self.mammatus, cumulonimbus: &self.cumulonimbus }
    }
}

impl shared::media::Medium<f32> for KernelMedium<'_> {
    fn density(&self, x: f32, y: f32, z: f32) -> f32 {
        if self.kind == MEDIUM_CONSTANT {
            return self.sigma;
        }
        let q = if self.radius > 0.0 { shared::media::planet_local(self.radius, x, y, z) } else { (x, y, z) };
        self.layer().density(q.0, q.1, q.2)
    }

    fn bound(&self) -> f32 {
        if self.kind == MEDIUM_CONSTANT { self.sigma } else { self.layer().bound() }
    }

    fn majorant_from(&self, x: f32, y: f32, z: f32, dx: f32, dy: f32, dz: f32) -> (f32, f32) {
        if self.kind == MEDIUM_CONSTANT {
            return (self.sigma, INF);
        }
        let layer = self.layer();
        if self.radius > 0.0 {
            shared::media::OnPlanet { radius: self.radius, medium: &layer }.majorant_from(x, y, z, dx, dy, dz)
        } else {
            layer.majorant_from(x, y, z, dx, dy, dz)
        }
    }
}

/// The medium whose record begins at `base`.
pub fn load_medium(tables: &[f32], base: u32) -> KernelMedium<'_> {
    let kind = at(tables, base) as u32;
    // A medium has one layer's record at most: the other kinds' are any record's numbers,
    // unread.
    let record = at(tables, base + MEDIUM_DENSITY).to_bits();
    KernelMedium {
        tables: FloatTables(tables), kind, sigma: at(tables, base + MEDIUM_DENSITY), radius: at(tables, base + MEDIUM_RADIUS),
        nubis: load_nubis(tables, if kind == MEDIUM_NUBIS { record } else { base }),
        benard: load_benard(tables, if kind == MEDIUM_BENARD { record } else { base }),
        cirrus: load_cirrus(tables, if kind == MEDIUM_CIRRUS { record } else { base }),
        rain: load_rain(tables, if kind == MEDIUM_RAIN { record } else { base }),
        mammatus: load_mammatus(tables, if kind == MEDIUM_MAMMATUS { record } else { base }),
        cumulonimbus: load_cumulonimbus(tables, if kind == MEDIUM_CUMULONIMBUS { record } else { base }),
    }
}

/// The stretch of the ray, within `tmin` to `tmax`, inside the medium's box: empty if the
/// first isn't less than the second.
fn medium_span(tables: &[f32], base: u32, ro: Vec3, rd: Vec3, tmin: f32, tmax: f32) -> (f32, f32) {
    let b = base + MEDIUM_BOX;
    let lo = Vec3::new(at(tables, b), at(tables, b + 1), at(tables, b + 2));
    let hi = Vec3::new(at(tables, b + 3), at(tables, b + 4), at(tables, b + 5));
    let inv = Vec3::ONE / rd;
    let (p, q) = ((lo - ro) * inv, (hi - ro) * inv);
    let (near, far) = slab(p.x, q.x, tmin, tmax);
    let (near, far) = slab(p.y, q.y, near, far);
    slab(p.z, q.z, near, far)
}

/// The factor on the density of the medium at `base` for a segment of a path with these
/// flags (the renderer's `Volume::segment_scale`): as the medium it last scattered in set it;
/// before any, the medium's own truncation once the path has `left` a vertex that isn't a
/// mirror's or glass's; and as given for what the camera sees directly.
fn segment_scale(s: &Scene, flags: u32, left: bool, base: u32) -> f32 {
    let last = (flags >> PATH_MEDIUM_SHIFT) & 0xff;
    if last > 0 {
        let it = s.params.media.x + (last - 1) * MEDIUM_SIZE;
        let reduced = ((flags >> PATH_SCATTERINGS_SHIFT) & 0xff) as f32 >= at(s.tables, it + MEDIUM_REDUCED_FROM);
        at(s.tables, it + if reduced { MEDIUM_REDUCED_SCALE } else { MEDIUM_SCALE })
    } else if left {
        at(s.tables, base + MEDIUM_SCALE)
    } else {
        1.0
    }
}

/// The first collision of the ray in any medium before `reach`: how far, and which medium
/// (`NONE` if it gets through them all), by delta tracking.
pub fn media_collision(s: &Scene, ro: Vec3, rd: Vec3, reach: f32, flags: u32, rng: &mut u32) -> (f32, u32) {
    let mut best = reach;
    let mut which = NONE;
    let mut random = Hashed(*rng);
    let left = flags & PATH_SCATTERED != 0;
    let mut m = 0;
    while m < s.params.media.y {
        let base = s.params.media.x + m * MEDIUM_SIZE;
        let (near, far) = medium_span(s.tables, base, ro, rd, 0.0, best);
        let scale = segment_scale(s, flags, left, base);
        if near < far && scale > 0.0 {
            let medium = load_medium(s.tables, base);
            // (The density is tabulated by where it is in the world.)
            let from = world(s, ro);
            if let Some(t) = shared::media::delta_track(&medium, (from.x, from.y, from.z), (rd.x, rd.y, rd.z), near, far, scale, &mut random) {
                best = t;
                which = m;
            }
        }
        m += 1;
    }
    *rng = random.0;
    (best, which)
}

/// How much light gets through the media along the ray as far as `tmax`: one estimate, by
/// residual ratio tracking, whose mean is exp(-optical depth). With every density times
/// `thinned` besides (an octave of the Oz approximation).
pub fn media_transmittance(s: &Scene, ro: Vec3, rd: Vec3, tmax: f32, flags: u32, left: bool, thinned: f32, rng: &mut u32) -> f32 {
    let mut through = 1.0;
    let mut random = Hashed(*rng);
    let mut m = 0;
    while m < s.params.media.y && through > 0.0 {
        let base = s.params.media.x + m * MEDIUM_SIZE;
        let (near, far) = medium_span(s.tables, base, ro, rd, 0.0, tmax);
        let scale = segment_scale(s, flags, left, base) * thinned;
        if near < far && scale > 0.0 {
            let medium = load_medium(s.tables, base);
            let lower = at(s.tables, base + MEDIUM_LOWER);
            let from = world(s, ro);
            through = match shared::media::ratio_track(&medium, (from.x, from.y, from.z), (rd.x, rd.y, rd.z), near, far, lower, scale, through, &mut random) {
                Some(left) => left,
                None => 0.0,
            };
        }
        m += 1;
    }
    *rng = random.0;
    through
}

/// The phase function of the medium at `base` for the cosine of the angle scattered through,
/// without a fraction `peak` of its scattering (delta-M truncation; 0 for all of it; 1 for
/// none of its shape: isotropic, as the similarity relations have it).
fn medium_phase(tables: &[f32], base: u32, peak: f32, cos: f32) -> f32 {
    let p = base + MEDIUM_PHASE;
    if peak >= 1.0 {
        shared::phase::henyey_greenstein(cos, 0.0)
    } else if at(tables, p) as u32 == PHASE_DROPLETS {
        shared::phase::HgDraine { g_hg: at(tables, p + 1), g_d: at(tables, p + 2), alpha: at(tables, p + 3), w_d: at(tables, p + 4) }.eval(cos)
    } else {
        shared::phase::henyey_greenstein(cos, shared::phase::without_peak(at(tables, p + 1), peak))
    }
}

/// A direction scattered into from `incoming`, in proportion to `medium_phase`.
fn medium_scatter(tables: &[f32], base: u32, peak: f32, incoming: Vec3, rng: &mut u32) -> Vec3 {
    let p = base + MEDIUM_PHASE;
    let v = (incoming.x, incoming.y, incoming.z);
    let d = if peak >= 1.0 {
        let (u1, u2) = (rand(rng), rand(rng));
        shared::phase::sample_henyey_greenstein(v, 0.0, u1, u2)
    } else if at(tables, p) as u32 == PHASE_DROPLETS {
        let mut random = Hashed(*rng);
        let d = shared::phase::HgDraine { g_hg: at(tables, p + 1), g_d: at(tables, p + 2), alpha: at(tables, p + 3), w_d: at(tables, p + 4) }.sample(v, &mut random);
        *rng = random.0;
        d
    } else {
        let (u1, u2) = (rand(rng), rand(rng));
        shared::phase::sample_henyey_greenstein(v, shared::phase::without_peak(at(tables, p + 1), peak), u1, u2)
    };
    Vec3::new(d.0, d.1, d.2)
}

// ---- Queues of paths: a round's dispatch covers the paths still alive, not every pixel. ----
//
// One buffer, `queue`, holds two queues of pixel indices, side 0 and side 1. A round reads the
// paths to advance from one side (`Params::seed.z`) and writes those still alive after it to
// the other; the next round swaps them. Each side has a header of four words: the dispatch that
// covers it (workgroups in x and y, then 1), as `dispatchWorkgroupsIndirect` reads it, and its
// count. Kernels append with atomics, keeping the header's dispatch in step with the count,
// so the host needs no kernel between rounds: it copies the filled side's header to the
// buffer the next dispatch is read from (a buffer bound for writing can't also be a
// dispatch's arguments), and a fresh header over the side just emptied.
//
// A third queue, the host's, takes the paths a round can't advance: those at a material the
// kernels have no code for. They wait there, out of both sides, until the rounds are done and
// the host reads them (`HOST_QUEUE`).
//
// Every access to the buffer is atomic: naga gives a buffer touched by atomic instructions
// an atomic element type, as WGSL and Metal require.

/// The wavefront's width: a path for each pixel in each lane. A pass may carry several samples
/// of every pixel at once, lane k's path for pixel i in slot `k * pixels + i` of the paths and
/// of the sky queries, and it is slots the queues hold. The rounds a pass takes don't grow with
/// its lanes, so neither do its dispatches, nor the times the host must be waited for; its
/// memory does. (The other way to keep a wavefront full, starting a pixel's next sample when
/// its path ends, was measured here and gave nothing: with one path a pixel a pass still waits
/// for its slowest pixel. See the renderer's `scheduler`.)
pub fn slots(params: &Params) -> u32 {
    params.dims.x * params.dims.y * params.seed.w
}

/// Words in the queue buffer before the queues' items: a header each for the two sides and
/// the host's queue.
pub const QUEUE_HEADER: u32 = 12;
/// The host's queue, after the two sides: the paths stopped at a material the kernels have no
/// code for (`PATH_HOST`), for the host to read once the rounds are done (`gather_host`,
/// `scatter_host`).
pub const HOST_QUEUE: u32 = 2;
/// In a side's header, after the dispatch's three words.
pub const QUEUE_COUNT: u32 = 3;
/// The most workgroups a dispatch may have in one dimension (WebGPU's default limit).
pub const MAX_GROUPS: u32 = 65535;
pub const WORKGROUP: u32 = 64;

/// A header for an empty queue: no workgroups (but a depth of one), no items.
pub const EMPTY_QUEUE: [u32; 4] = [0, 0, 1, 0];

/// The header of a queue of `count` items, as a kernel's appends leave it.
pub fn queue_header(count: u32) -> [u32; 4] {
    let groups = count.div_ceil(WORKGROUP);
    [groups.min(MAX_GROUPS), groups.div_ceil(MAX_GROUPS), 1, count]
}

#[cfg(target_arch = "spirv")]
mod queue {
    use super::*;
    use spirv_std::arch::{atomic_i_add, atomic_load, atomic_store};
    use spirv_std::memory::{Scope, Semantics};

    // The widest scope Vulkan's memory model allows without a further capability.
    const SCOPE: u32 = Scope::QueueFamily as u32;
    const SEMANTICS: u32 = Semantics::NONE.bits();

    fn load(queue: &[u32], i: u32) -> u32 {
        unsafe { atomic_load::<u32, SCOPE, SEMANTICS>(queue.get_unchecked(i as usize)) }
    }

    /// The pixel this invocation is to advance: item `gid` of the side being read, if there
    /// is one.
    pub fn take(queue: &[u32], gid: UVec3, params: &Params) -> Option<u32> {
        let side = params.seed.z;
        let i = gid.x + gid.y * (MAX_GROUPS * WORKGROUP);
        if i >= load(queue, side * 4 + QUEUE_COUNT) {
            return None;
        }
        Some(load(queue, QUEUE_HEADER + side * slots(params) + i))
    }

    /// Append `pixel` to the side being written.
    pub fn push(queue: &mut [u32], pixel: u32, params: &Params) {
        append(queue, 1 - params.seed.z, pixel, params);
    }

    /// Append `pixel` to the host's queue.
    pub fn push_host(queue: &mut [u32], pixel: u32, params: &Params) {
        append(queue, HOST_QUEUE, pixel, params);
    }

    /// How many paths wait in the host's queue.
    pub fn host_count(queue: &[u32]) -> u32 {
        load(queue, HOST_QUEUE * 4 + QUEUE_COUNT)
    }

    /// Item `i` of the host's queue.
    pub fn host_item(queue: &[u32], i: u32, params: &Params) -> u32 {
        load(queue, QUEUE_HEADER + HOST_QUEUE * slots(params) + i)
    }

    /// Append `pixel` to queue `which` (a side, or the host's), keeping its header's dispatch
    /// in step with its count.
    fn append(queue: &mut [u32], which: u32, pixel: u32, params: &Params) {
        let header = (which * 4) as usize;
        unsafe {
            let slot = atomic_i_add::<u32, SCOPE, SEMANTICS>(queue.get_unchecked_mut(header + QUEUE_COUNT as usize), 1);
            atomic_store::<u32, SCOPE, SEMANTICS>(
                queue.get_unchecked_mut((QUEUE_HEADER + which * slots(params) + slot) as usize), pixel);
            // The first item of each workgroup extends the dispatch to cover it: across x
            // until that is full, then a row of y for every `MAX_GROUPS` groups.
            if slot % WORKGROUP == 0 {
                let group = slot / WORKGROUP;
                if group < MAX_GROUPS {
                    atomic_i_add::<u32, SCOPE, SEMANTICS>(queue.get_unchecked_mut(header), 1);
                }
                if group % MAX_GROUPS == 0 {
                    atomic_i_add::<u32, SCOPE, SEMANTICS>(queue.get_unchecked_mut(header + 1), 1);
                }
            }
        }
    }
}

/// A pass's paths, each from its camera ray (`PathState::start`), and the live ones queued for
/// the first round: on the GPU, so the host uploads the rays (32 bytes a path), not the paths.
/// Dispatched over every slot, with the parameters whose side (`seed.z`) is 1, so the queue it
/// fills is side 0, the first round's.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn start_paths(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &[CameraRay],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] queue: &mut [u32],
) {
    let j = gid.x + gid.y * (MAX_GROUPS * WORKGROUP);
    if j >= slots(params) {
        return;
    }
    let ray = rays[j as usize];
    let pixel = j % (params.dims.x * params.dims.y);
    let sample = if ray.d.w >= 0.5 { ray.d.w as u32 - 1 } else { 0 };
    let path = PathState::start(pixel, sample, params.seed.x, &ray);
    paths[j as usize] = path;
    if path.state.y & PATH_ACTIVE != 0 {
        queue::push(queue, j, params);
    }
}

/// The paths waiting for the host (`PATH_HOST`), copied side by side into `gathered` in the
/// order of the host's queue, each with its slot in `state.w`: what the host reads is then the
/// size of its work, not of the wavefront. Dispatched over the host's queue.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn gather_host(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] queue: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] gathered: &mut [PathState],
) {
    let i = gid.x + gid.y * (MAX_GROUPS * WORKGROUP);
    if i >= queue::host_count(queue) {
        return;
    }
    let slot = queue::host_item(queue, i, params);
    let mut p = paths[slot as usize];
    p.state.w = slot;
    gathered[i as usize] = p;
}

/// The other way: the paths the host has shaded, from `gathered` back to their slots
/// (`state.w`), those that go on queued for the next round. Dispatched over the host's queue,
/// as `gather_host` was.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn scatter_host(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] queue: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] gathered: &mut [PathState],
) {
    let i = gid.x + gid.y * (MAX_GROUPS * WORKGROUP);
    if i >= queue::host_count(queue) {
        return;
    }
    let mut p = gathered[i as usize];
    let slot = p.state.w;
    // (Its sample's number is still in its slot.)
    p.state.w = paths[slot as usize].state.w;
    paths[slot as usize] = p;
    if p.state.y & PATH_ACTIVE != 0 {
        queue::push(queue, slot, params);
    }
}

/// What each queued path's ray hits (`intersect`): a round's first kernel, small, and all of
/// its closest-hit traversal.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn intersect_paths(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] queue: &mut [u32],
) {
    let Some(pixel) = queue::take(queue, gid, params) else { return };
    let index = pixel as usize;
    let s = Scene { params, prims, nodes, materials, lights, tables };
    paths[index] = intersect::<NoMedia>(&s, &paths[index]);
}

/// The rest of a vertex of each queued path (`shade`, at what `intersect_paths` found), its
/// sky queries in its pixel's two slots; those still alive are queued for the next round. A
/// pass runs `max_depth` + 1 rounds, each over the paths the last left alive: `intersect_paths`,
/// this, `answer_sky`; then `add_paths`.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn bounce_paths(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] queue: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] queries: &mut [SkyQuery],
) {
    let Some(pixel) = queue::take(queue, gid, params) else { return };
    let index = pixel as usize;
    let p = paths[index];
    let s = Scene { params, prims, nodes, materials, lights, tables };
    let (next, q0, q1) = shade::<NoMedia>(&s, &p, pixel % (params.dims.x * params.dims.y));
    paths[index] = next;
    queries[2 * index] = q0;
    queries[2 * index + 1] = q1;
    if next.state.y & PATH_ACTIVE != 0 {
        queue::push(queue, pixel, params);
    }
    if next.state.y & PATH_HOST != 0 {
        queue::push_host(queue, pixel, params);
    }
}

/// `intersect_paths` for a scene with media (`WithMedia`): the walk through them too.
/// What each queued path's ray hits (`intersect`): a round's first kernel, small, and all of
/// its closest-hit traversal.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn intersect_paths_media(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] queue: &mut [u32],
) {
    let Some(pixel) = queue::take(queue, gid, params) else { return };
    let index = pixel as usize;
    let s = Scene { params, prims, nodes, materials, lights, tables };
    paths[index] = intersect::<WithMedia>(&s, &paths[index]);
}

/// `bounce_paths` for a scene with media.
/// The rest of a vertex of each queued path (`shade`, at what `intersect_paths` found), its
/// sky queries in its pixel's two slots; those still alive are queued for the next round. A
/// pass runs `max_depth` + 1 rounds, each over the paths the last left alive: `intersect_paths`,
/// this, `answer_sky`; then `add_paths`.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn bounce_paths_media(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] queue: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] queries: &mut [SkyQuery],
) {
    let Some(pixel) = queue::take(queue, gid, params) else { return };
    let index = pixel as usize;
    let p = paths[index];
    let s = Scene { params, prims, nodes, materials, lights, tables };
    let (next, q0, q1) = shade::<WithMedia>(&s, &p, pixel % (params.dims.x * params.dims.y));
    paths[index] = next;
    queries[2 * index] = q0;
    queries[2 * index + 1] = q1;
    if next.state.y & PATH_ACTIVE != 0 {
        queue::push(queue, pixel, params);
    }
    if next.state.y & PATH_HOST != 0 {
        queue::push_host(queue, pixel, params);
    }
}

/// The light the sky queries of the paths just advanced find (`sky_light`), added to each
/// path's: the sky, space and the air's haze, in a kernel of their own, as they are most of
/// the shader's code. Dispatched over the same queue as the `bounce_paths` before it.
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn answer_sky(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] prims: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nodes: &[Node],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] lights: &[Light],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] tables: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] paths: &mut [PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] queue: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] queries: &mut [SkyQuery],
) {
    let Some(pixel) = queue::take(queue, gid, params) else { return };
    let index = pixel as usize;
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

/// What a finished path adds to its pixel's sum: its light times the camera sample's weight,
/// and in w the rays it cast. A NaN (from some degenerate case) is dropped, as the CPU
/// renderers drop theirs; its rays were still cast. (Not `is_finite`: its infinite constant
/// is invalid for naga, which wgpu translates through.) Nothing for a pixel whose camera ray
/// the lens stopped (`PATH_ADDED` from the start).
pub fn path_sum(p: &PathState) -> Vec4 {
    if p.state.y & PATH_ADDED != 0 {
        return Vec4::ZERO;
    }
    let c = p.l.xyz() * p.o.w;
    #[allow(clippy::eq_op)]
    let not_nan = c.x == c.x && c.y == c.y && c.z == c.z;
    if not_nan { c } else { Vec3::ZERO }.extend(p.l.w)
}

/// Every pixel's paths, finished, added to the image: once a pass, after its last round. The
/// image holds two vectors a pixel: the sum of its samples (and in w the rays they cast), and
/// the sum of their squares, channel by channel (and in w how many were added), from which
/// the host judges the pixel's noise (`scheduler::plan`).
#[cfg(target_arch = "spirv")]
#[spirv(compute(threads(64)))]
pub fn add_paths(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &Params,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] paths: &[PathState],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] accum: &mut [Vec4],
) {
    let Some(index) = pixel(gid, params) else { return };
    // Lane by lane, which is the order of the pixel's samples: the sums are those of as many
    // passes of one lane.
    let pixels = (params.dims.x * params.dims.y) as usize;
    let mut lane = 0;
    while lane < params.seed.w as usize {
        let add = path_sum(&paths[lane * pixels + index]);
        if add.w > 0.0 {
            let c = add.xyz();
            accum[2 * index] += add;
            accum[2 * index + 1] += (c * c).extend(1.0);
        }
        lane += 1;
    }
}
