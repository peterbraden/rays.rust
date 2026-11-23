//! The scene as the kernels read it (`shaders/pathtrace`, the one copy of the wavefront's
//! intersection and shading, in f32): the flat scene (`wavefront::flat`) laid out as the
//! kernel crate's own records and tables (`Buffers::build`), the camera's rays, and the host's
//! side of the queue for what the kernels have no code for (`host`). Nothing here knows which
//! device runs the kernels: the GPU (`gpu`, through wgpu) uploads these buffers, and the CPU
//! (`wavefront::integrator`) calls the same crate's functions over them.

use crate::color::Color;
use crate::scene::{PathTermination, Scene};
use crate::wavefront::flat::{FlatMaterial, FlatScene, Prim};
use crate::light::Profile;
use crate::na::Vector3;
use crate::ray::Ray;
use bytemuck::Zeroable;
use rayon::prelude::*;

pub mod host;

pub use pathtrace::{CameraRay as GpuRay, Light as GpuLight, Material as GpuMaterial, Node as GpuNode, Params, PathState, Prim as GpuPrim, SkyQuery};
use pathtrace::glam::{UVec4, Vec4};

/// The baked sky's resolution where the shader looks its light up in the map (a plain
/// background), and how many texels a side a cell of its visibility
/// (`environment::visibility`) covers, so that the visibility is at the CPU's resolution.
const ENV_WIDTH: usize = 1024;
const ENV_HEIGHT: usize = 512;
const VISIBILITY_SCALE: usize = 8;
/// Its resolution where the shader evaluates the sky or space for each ray, and the map only
/// chooses directions to sample: the CPU's own (`environment`), a ray of visibility a texel.
/// Each texel there is the sky's scattering integrated, which at the full resolution was most
/// of setting the GPU up (1.1 s of 1.5 on four cores).
const SAMPLING_WIDTH: usize = 128;
const SAMPLING_HEIGHT: usize = 64;

pub(crate) fn v4(v: &Vector3<f64>, w: f32) -> Vec4 {
    Vec4::new(v.x as f32, v.y as f32, v.z as f32, w)
}

pub(crate) fn c4(c: &Color) -> Vec4 {
    Vec4::new(c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32, 0.)
}

/// Flatten `m` into `out` (checkerboards' squares after it), returning its index.
fn push_material(m: &FlatMaterial, out: &mut Vec<GpuMaterial>, kernels: &[Option<GpuMaterial>]) -> u32 {
    let index = out.len() as u32;
    let blank = GpuMaterial::zeroed();
    out.push(blank);
    let g = match m {
        FlatMaterial::Diffuse { albedo } => GpuMaterial { kind: pathtrace::MAT_DIFFUSE, c0: c4(albedo), ..blank },
        FlatMaterial::Plastic { albedo, refractive_index, roughness } =>
            GpuMaterial { kind: pathtrace::MAT_PLASTIC, c0: c4(albedo), p: Vec4::new(*refractive_index as f32, *roughness as f32, 0., 0.), ..blank },
        FlatMaterial::Dielectric { refractive_index, tint } =>
            GpuMaterial { kind: pathtrace::MAT_DIELECTRIC, c0: c4(tint), p: Vec4::new(*refractive_index as f32, 0., 0., 0.), ..blank },
        FlatMaterial::Conductor { optics, roughness } => {
            let (eta, k) = optics.current();
            GpuMaterial { kind: pathtrace::MAT_CONDUCTOR, c0: c4(&eta), c1: c4(&k), p: Vec4::new(0., *roughness as f32, 0., 0.), ..blank }
        },
        FlatMaterial::Emitter { radiance, two_sided } => GpuMaterial { kind: pathtrace::MAT_EMITTER, a: *two_sided as u32, c0: c4(radiance), ..blank },
        FlatMaterial::Specular { albedo, roughness } => GpuMaterial { kind: pathtrace::MAT_METAL, c0: c4(albedo), p: Vec4::new(0., *roughness as f32, 0., 0.), ..blank },
        FlatMaterial::Checker { m1, m2, xsize, zsize } => {
            let a = push_material(m1, out, kernels);
            let b = push_material(m2, out, kernels);
            GpuMaterial { kind: pathtrace::MAT_CHECKER, a, b, p: Vec4::new(0., 0., *xsize as f32, *zsize as f32), ..blank }
        },
        // The kernels stop a path here for the host (`host`).
        // unless they have a kernel of this material after all (`host_kernels`).
        FlatMaterial::Host { object } => kernels[*object as usize].unwrap_or(GpuMaterial { kind: pathtrace::MAT_HOST, a: *object, ..blank }),
    };
    out[index as usize] = g;
    index
}

/// A cumulative distribution over `f` (n + 1 values from 0 to 1), uniform if `f` sums to 0.
pub(crate) fn cdf(f: &[f64]) -> Vec<f32> {
    let total: f64 = f.iter().sum();
    let mut out = Vec::with_capacity(f.len() + 1);
    let mut acc = 0.;
    out.push(0.);
    for (i, v) in f.iter().enumerate() {
        acc += if total > 0. { v / total } else { 1. / f.len() as f64 };
        out.push(if i + 1 == f.len() { 1. } else { acc as f32 });
    }
    out
}

/// The direction through texel (x, y) of the baked sky's map, as the shader's `env_sample`.
pub(crate) fn env_direction(x: usize, y: usize, width: usize, height: usize) -> Vector3<f64> {
    use std::f64::consts::PI;
    let u = (x as f64 + 0.5) / width as f64;
    let v = (y as f64 + 0.5) / height as f64;
    let (phi, theta) = ((u - 0.5) * 2. * PI, v * PI);
    Vector3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin())
}

/// Append `air` (and its moonlit copy) to `tables` as the shader's record (`pathtrace::AIR_*`),
/// returning where it starts.
/// The media's records (`pathtrace::MEDIUM_SIZE` floats each, one after another), after the
/// cloud layers' own that they point to: where the first begins.
pub(crate) fn media_records(tables: &mut Vec<f32>, media: &[crate::volume::FlatMedium], origin: &Vector3<f64>) -> u32 {
    use crate::material::phase::PhaseFunction;
    use crate::volume::FieldExport;
    let fields: Vec<(u32, f32, f32)> = media.iter().map(|m| match &m.field {
        FieldExport::Constant(d) => (pathtrace::MEDIUM_CONSTANT, *d as f32, 0.),
        FieldExport::Nubis { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_NUBIS, f32::from_bits(at), *radius as f32)
        },
        FieldExport::Benard { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_BENARD, f32::from_bits(at), *radius as f32)
        },
        FieldExport::Cirrus { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_CIRRUS, f32::from_bits(at), *radius as f32)
        },
        FieldExport::Rain { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_RAIN, f32::from_bits(at), *radius as f32)
        },
        FieldExport::Mammatus { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_MAMMATUS, f32::from_bits(at), *radius as f32)
        },
        FieldExport::Cumulonimbus { record, radius } => {
            let at = tables.len() as u32;
            tables.extend(record);
            (pathtrace::MEDIUM_CUMULONIMBUS, f32::from_bits(at), *radius as f32)
        },
    }).collect();
    let first = tables.len() as u32;
    for (m, (kind, density, radius)) in media.iter().zip(fields) {
        // (Its box in the render's frame; its density stays where it is in the world.)
        let (lo, hi) = (m.bounds.min.coords - origin, m.bounds.max.coords - origin);
        let phase = match m.phase {
            PhaseFunction::HenyeyGreenstein { g } => [pathtrace::PHASE_HENYEY_GREENSTEIN as f32, g as f32, 0., 0., 0.],
            PhaseFunction::HgDraine { g_hg, g_d, alpha, w_d } => [pathtrace::PHASE_DROPLETS as f32, g_hg as f32, g_d as f32, alpha as f32, w_d as f32],
        };
        tables.push(kind as f32);
        tables.extend([lo.x as f32, lo.y as f32, lo.z as f32, hi.x as f32, hi.y as f32, hi.z as f32]);
        tables.extend([density, radius, m.scale as f32, m.lower as f32]);
        tables.extend(phase);
        tables.extend([m.albedo.rgb.x as f32, m.albedo.rgb.y as f32, m.albedo.rgb.z as f32, m.peak as f32]);
        // (Never, without the similarity relations: a path counts 255 scatterings at most.)
        let (from, scale) = m.reduced.map_or((1e9, 1.), |(n, scale)| (n as f32, scale as f32));
        tables.extend([scale, from, m.mean_cosine.0 as f32, m.mean_cosine.1 as f32]);
        let oz = m.oz.map_or([0.; 4], |o| [o.octaves as f32, o.a as f32, o.b as f32, o.c as f32]);
        tables.extend(oz);
        tables.push(m.sun_probes as u32 as f32);
    }
    // (Every kind's loader reads from each record: `pathtrace::LAYER_HEADER_MOST`.)
    if !media.is_empty() {
        tables.extend(std::iter::repeat_n(0., pathtrace::LAYER_HEADER_MOST as usize));
    }
    first
}

pub(crate) fn air_record(tables: &mut Vec<f32>, air: &crate::skysphere::AirExport, origin: &Vector3<f64>) -> u32 {
    use pathtrace::*;
    let base = tables.len();
    tables.resize(base + AIR_HEADER as usize, 0.);
    let put = |tables: &mut Vec<f32>, i: u32, v: f64| tables[base + i as usize] = v as f32;
    let put3 = |tables: &mut Vec<f32>, i: u32, v: &Vector3<f64>| (0..3).for_each(|k| tables[base + i as usize + k] = v[k] as f32);
    // (In the render's frame.)
    let c = air.earth_centre - origin;
    put3(tables, AIR_CENTRE, &c);
    put(tables, AIR_RADII, air.earth_radius);
    put(tables, AIR_RADII + 1, air.atmosphere_radius);
    // |c|² - r², in f64: the shader's `excess` adds the terms in p, which are small near the
    // ground, to these.
    put(tables, AIR_EXCESS, c.norm_squared() - air.earth_radius * air.earth_radius);
    put(tables, AIR_EXCESS + 1, c.norm_squared() - air.atmosphere_radius * air.atmosphere_radius);
    put3(tables, AIR_SUN, &air.sun_direction);
    put(tables, AIR_HEIGHTS, air.rayleigh_thickness);
    put(tables, AIR_HEIGHTS + 1, air.mie_thickness);
    put(tables, AIR_BRIGHTNESS, air.brightness);
    put3(tables, AIR_RAYLEIGH, &air.rayleigh);
    put(tables, AIR_MIE, air.mie);
    put(tables, AIR_MIE + 1, air.mie_anisotropy);
    put3(tables, AIR_OZONE, &air.ozone);
    for (i, h) in crate::skysphere::LUT_ALTITUDES.iter().enumerate() {
        put(tables, AIR_LUT_ALTITUDES + i as u32, *h);
    }
    let offset = |tables: &mut Vec<f32>, i: u32, values: Option<Vec<f32>>| {
        let at = match values {
            Some(v) => { let at = tables.len() as u32; tables.extend(v); at },
            None => NO_AIR,
        };
        tables[base + (AIR_TABLES + i) as usize] = f32::from_bits(at);
    };
    offset(tables, 0, Some(air.sun_table.iter().flat_map(|d| d.map(|x| x as f32)).collect()));
    offset(tables, 1, air.multiple.as_ref().map(|m| m.iter().flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect()));
    offset(tables, 2, air.lut.as_ref().map(|l| l.iter().flat_map(|(r, m, s)| [r, m, s]).flat_map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect()));
    let moon = air.moon.as_ref().map_or(NO_AIR, |m| air_record(tables, m, origin));
    tables[base + (AIR_TABLES + 3) as usize] = f32::from_bits(moon);
    base as u32
}

/// Append `space` to `tables` as the shader's record (`pathtrace::SPACE_*`), returning where it
/// starts: the frame and settings, the noise's tables, and each part's parameters (the stars
/// as a catalogue, which the shader can't generate).
pub(crate) fn space_record(tables: &mut Vec<f32>, space: &crate::procedural::space::Space) -> u32 {
    use pathtrace::*;
    let base = tables.len();
    tables.resize(base + SPACE_HEADER as usize, 0.);
    let f = &space.frame;
    for (i, v) in [f.centre, f.pole, f.east].iter().flat_map(|v| v.iter()).enumerate() {
        tables[base + SPACE_FRAME as usize + i] = *v as f32;
    }
    tables[base + SPACE_BRIGHTNESS as usize] = space.brightness as f32;
    tables[base + SPACE_THRESHOLD as usize] = space.threshold as f32;
    // Every field's noise is Perlin's, with the same table and rotation; only its offset differs.
    let noise = crate::noise::SeededNoise::new(0);
    for (i, v) in noise.rotation_rows().iter().enumerate() {
        tables[base + SPACE_ROTATION as usize + i] = *v as f32;
    }
    let put = |tables: &mut Vec<f32>, i: u32, v: u32| tables[base + (SPACE_TABLES + i) as usize] = f32::from_bits(v);
    let at = tables.len() as u32;
    tables.extend(noise.permutation().iter().map(|p| *p as f32));
    put(tables, 0, at);
    let stars = match &space.stars {
        Some(stars) => {
            let (sigma, n, starts, list, flux) = stars.export(&space.frame);
            let at = tables.len();
            tables.resize(at + STARS_HEADER as usize, 0.);
            tables[at] = sigma as f32;
            tables[at + 1] = f32::from_bits(n as u32);
            let offset = |tables: &mut Vec<f32>, i: usize, values: Vec<f32>| {
                tables[at + i] = f32::from_bits(tables.len() as u32);
                tables.extend(values);
            };
            offset(tables, 2, starts.iter().map(|s| f32::from_bits(*s)).collect());
            offset(tables, 3, list.iter().flat_map(|s| s.map(|x| x as f32)).collect());
            offset(tables, 4, flux);
            at as u32
        },
        None => NO_AIR,
    };
    put(tables, 1, stars);
    let record = |tables: &mut Vec<f32>, values: Vec<f64>| { let at = tables.len() as u32; tables.extend(values.iter().map(|v| *v as f32)); at };
    let milky = space.milky_way.as_ref().map_or(NO_AIR, |m| record(tables, m.export()));
    put(tables, 2, milky);
    let nebulae = record(tables, space.nebulae.iter().flat_map(|n| n.export()).collect());
    put(tables, 3, nebulae);
    put(tables, 4, space.nebulae.len() as u32);
    let galaxies = record(tables, space.galaxies.iter().flat_map(|g| g.export()).collect());
    put(tables, 5, galaxies);
    put(tables, 6, space.galaxies.len() as u32);
    base as u32
}

/// The depth of an exported BVH (`Bvh::export`), leaves included: one for a lone leaf. The
/// shader's stack needs an entry for each level of interior nodes, which is one fewer.
pub(crate) fn bvh_depth(nodes: &[(crate::shapes::bbox::BBox, u32, u32)]) -> usize {
    if nodes.is_empty() {
        return 0;
    }
    let mut deepest = 0;
    let mut stack = vec![(0usize, 1usize)];
    while let Some((i, d)) = stack.pop() {
        deepest = deepest.max(d);
        let (_, offset, count) = nodes[i];
        if count == 0 {
            stack.push((i + 1, d + 1));
            stack.push((offset as usize, d + 1));
        }
    }
    deepest
}

/// The BVH as the shader traverses it: its interior nodes alone, each with its two children's
/// boxes (`pathtrace::Node`), the root first. A tree that is a single leaf gets a root with
/// that leaf as its only child; no tree at all, a root no ray enters.
pub(crate) fn gpu_nodes(exported: &[(crate::shapes::bbox::BBox, u32, u32)]) -> Result<Vec<GpuNode>, String> {
    use pathtrace::glam::Vec3;
    // Children that aren't there.
    let none = pathtrace::NO_CHILD;
    let empty = GpuNode { amin: Vec3::ZERO, a: none, amax: Vec3::ZERO, a_count: 0, bmin: Vec3::ZERO, b: none, bmax: Vec3::ZERO, b_count: 0 };
    let corners = |b: &crate::shapes::bbox::BBox| (Vec3::new(b.min.x as f32, b.min.y as f32, b.min.z as f32), Vec3::new(b.max.x as f32, b.max.y as f32, b.max.z as f32));
    let Some((root_bounds, root_offset, root_count)) = exported.first() else {
        return Ok(vec![empty]);
    };
    if *root_count > 0 {
        let (amin, amax) = corners(root_bounds);
        return Ok(vec![GpuNode { amin, amax, a: *root_offset, a_count: *root_count, ..empty }]);
    }
    // Each interior node's index among the interior nodes, in the same (depth-first) order.
    let mut index = vec![u32::MAX; exported.len()];
    let mut interior = 0;
    for (i, (_, _, count)) in exported.iter().enumerate() {
        if *count == 0 {
            index[i] = interior;
            interior += 1;
        }
    }
    // A child as a node holds it: (first item, count) for a leaf, (index, 0) otherwise.
    let child = |i: usize| -> Result<(Vec3, Vec3, u32, u32), String> {
        let (bounds, offset, count) = &exported[i];
        let (min, max) = corners(bounds);
        if *count > pathtrace::MAX_LEAF {
            return Err(format!("a BVH leaf of {} primitives (the shader's hold {})", count, pathtrace::MAX_LEAF));
        }
        Ok(if *count > 0 { (min, max, *offset, *count) } else { (min, max, index[i], 0) })
    };
    let mut nodes = Vec::with_capacity(interior as usize);
    for (i, (_, second, count)) in exported.iter().enumerate() {
        if *count == 0 {
            // An interior node's first child is the next node; its second, at `offset`.
            let (amin, amax, a, a_count) = child(i + 1)?;
            let (bmin, bmax, b, b_count) = child(*second as usize)?;
            nodes.push(GpuNode { amin, a, amax, a_count, bmin, b, bmax, b_count });
        }
    }
    Ok(nodes)
}

/// The shader's inputs, built on the CPU (`Buffers::build`).
pub struct Buffers {
    pub params: Params,
    /// The primitives as the shader reads them (`pathtrace::pack_prims`), and as records.
    pub prims: Vec<pathtrace::glam::Vec4>,
    pub prim_records: Vec<GpuPrim>,
    pub nodes: Vec<GpuNode>,
    pub materials: Vec<GpuMaterial>,
    pub lights: Vec<GpuLight>,
    pub tables: Vec<f32>,
    /// Whether any vertex is the host's to shade (`host`).
    pub hosts: bool,
    /// The shapes (of `FlatScene::shapes`) the kernels have no test for, which the host
    /// intersects (`pathtrace::PRIM_HOST`), and their names, for a device that can't ask it.
    pub host_shapes: Vec<usize>,
    pub unported: Vec<String>,
    /// The render's origin in the world (`origin`), which every position here is relative to.
    pub origin: Vector3<f64>,
}

/// The render's origin for a scene: at the camera, to the nearest whole number a float holds
/// exactly (so that adding it back in the kernels, for what is tabulated by where it is in the
/// world, rounds once and not twice; and so that cameras a step apart share a frame).
///
/// f64 is for where things are in the world; a render needs where they are from the camera.
/// A float holds a direction to 6e-8, far finer than a pixel, so a point kept relative to the
/// camera is placed to a small fraction of a pixel however far off it is, and to micrometres
/// near by, wherever in the world the camera stands. The frame is chosen here, on the host, in
/// f64, and the kernels are given nothing but numbers in it (plan.md section 15).
pub fn origin(s: &Scene) -> Vector3<f64> {
    crate::environment::viewpoint(s).map(|v| v.round() as f32 as f64)
}

impl Buffers {
    /// The buffers as the kernels' functions take them, on the CPU; `host` answers shadow
    /// rays for the shapes the kernels can't test.
    pub fn scene<'a>(&'a self, host: Option<&'a dyn pathtrace::HostShapes>) -> pathtrace::Scene<'a> {
        pathtrace::Scene { params: &self.params, prims: &self.prims, nodes: &self.nodes, materials: &self.materials, lights: &self.lights, tables: &self.tables, host }
    }
}

/// For each of the flat scene's host materials, the kernels' record of it if they have its
/// code after all: a material that isn't data to the CPU's integrator, but is one the shader
/// has been given. So far the sea's water (`pathtrace::MAT_WATER`), where no moon lights it
/// (the kernel's glitter is the sun's alone).
fn host_kernels(s: &Scene, flat: &FlatScene) -> Vec<Option<GpuMaterial>> {
    flat.hosts.iter().map(|o| {
        let w = o.medium.material_at(Vector3::zeros()).water().filter(|_| s.moonlight.is_none())?;
        Some(GpuMaterial { kind: pathtrace::MAT_WATER, c0: c4(&w.body), c1: c4(&w.bubbles), p: Vec4::new(w.radius as f32, w.roughness as f32, 0., 0.), ..GpuMaterial::zeroed() })
    }).collect()
}

impl Buffers {
    /// The flattened scene as the shader's buffers, or why it can't be.
    pub fn build(s: &Scene, flat: &FlatScene) -> Result<Buffers, String> {
        Buffers::build_at(s, flat, &origin(s))
    }

    /// `build`, about `origin`: every position in the buffers is relative to it
    /// (`pathtrace::Params::origin`).
    pub fn build_at(s: &Scene, flat: &FlatScene, origin: &Vector3<f64>) -> Result<Buffers, String> {
        // A shape its own code intersects needs a kernel of its kind: there is one for the
        // sea (`pathtrace::ocean_hit`), of which the kernels take one. The rest are the
        // host's (`pathtrace::PRIM_HOST`), where the device can ask it.
        let sea = flat.shapes.iter().position(|(o, _)| o.geometry.ocean().is_some());
        let host_shapes: Vec<usize> = (0..flat.shapes.len()).filter(|k| Some(*k) != sea).collect();
        let unported: Vec<String> = host_shapes.iter().map(|&k| {
            let o = &flat.shapes[k].0;
            match s.source_index(o) {
                Some(i) => format!("objects[{}] ({})", i, o.geometry.kind()),
                None => o.geometry.kind().to_string(),
            }
        }).collect();
        let kernels = host_kernels(s, flat);
        let hosts = kernels.iter().filter(|k| k.is_none()).count() + host_shapes.len();
        if !flat.media.is_empty() {
            // (The host's materials trace their own shadow rays through the media, by the
            // scene's own code, with the path's state there: `host::volume_state`.) A medium's
            // vertex has no kernel for moonlight.
            if s.moonlight.is_some() {
                return Err("participating media lit by a moon".into());
            }
            if s.light_bvh.is_some() {
                return Err("participating media with a light BVH".into());
            }
            if flat.media.len() > pathtrace::MAX_MEDIA as usize {
                return Err(format!("{} media (the kernels count to {})", flat.media.len(), pathtrace::MAX_MEDIA));
            }
        }
        let mut lights = Vec::new();
        for l in &s.lights {
            let i = l.emission.srgb() * l.intensity;
            let (kind, axis, start, end) = match &l.profile {
                Profile::Isotropic => (0., Vector3::z(), 1., 1.),
                Profile::Spot { axis, cos_falloff_start, cos_total } => (1., *axis, *cos_falloff_start, *cos_total),
                _ => return Err("goniometric and projection lights".into()),
            };
            lights.push(GpuLight { pos: v4(&(l.position - origin), kind), intensity: c4(&i), axis: v4(&axis, start as f32), extra: Vec4::new(end as f32, 0., 0., 0.) });
        }
        if lights.is_empty() {
            lights.push(GpuLight::zeroed());
        }

        // Primitives: the bounded ones in the BVH's order, then the planes.
        let mut materials = Vec::new();
        let material_index: Vec<u32> = flat.materials.iter().map(|m| push_material(m, &mut materials, &kernels)).collect();
        // A buffer may not be empty (wgpu refuses to bind it): a scene of sky alone has none.
        if materials.is_empty() {
            materials.push(GpuMaterial::zeroed());
        }
        let to_gpu = |p: &Prim, m: u32, emitter: u32| -> GpuPrim {
            let mut g = GpuPrim::zeroed();
            g.info = UVec4::new(pathtrace::PRIM_TRIANGLE, material_index[m as usize], emitter, 0);
            match p {
                Prim::Triangle { v, n } => {
                    g.v0 = v4(&(v[0] - origin), 0.); g.v1 = v4(&(v[1] - origin), 0.); g.v2 = v4(&(v[2] - origin), 0.);
                    if let Some(n) = n {
                        g.n0 = v4(&n[0], 0.); g.n1 = v4(&n[1], 0.); g.n2 = v4(&n[2], 0.);
                        g.info.w = 1;
                    }
                },
                Prim::Sphere { center, radius } => { g.v0 = v4(&(center - origin), *radius as f32); g.info.x = pathtrace::PRIM_SPHERE; },
                Prim::Plane { y } => { g.v0 = Vec4::new((*y - origin.y) as f32, 0., 0., 0.); g.info.x = pathtrace::PRIM_PLANE; },
            }
            g
        };
        let mut prims: Vec<GpuPrim> = flat.prims.iter().enumerate().map(|(i, p)| {
            to_gpu(p, flat.prim_material[i], flat.emitter_index[i].map_or(0, |e| e + 1))
        }).collect();
        prims.extend(flat.planes.iter().map(|(p, m)| to_gpu(p, *m, 0)));
        // The shapes, after the planes and tested as they are, outside the tree: the sea by
        // its kernel, the others never (the host's, as their vertices are: `MAT_HOST`).
        let ocean = sea.map(|k| (flat.shapes[k].0.geometry.ocean().expect("the sea"), flat.shapes[k].1));
        let host_material = materials.len() as u32;
        if !host_shapes.is_empty() {
            materials.push(GpuMaterial { kind: pathtrace::MAT_HOST, ..GpuMaterial::zeroed() });
        }
        for (k, (_, m)) in flat.shapes.iter().enumerate() {
            let mut g = GpuPrim::zeroed();
            g.info = if Some(k) == sea {
                UVec4::new(pathtrace::PRIM_OCEAN, material_index[*m as usize], 0, 0)
            } else {
                UVec4::new(pathtrace::PRIM_HOST, host_material, 0, 0)
            };
            prims.push(g);
        }
        if prims.is_empty() {
            prims.push(GpuPrim::zeroed());
        }
        // The shader's traversal stack holds at most one child per level of interior nodes.
        let exported: Vec<_> = flat.bvh.export().into_iter()
            .map(|(b, x, y)| (crate::shapes::bbox::BBox::new(b.min.coords - origin, b.max.coords - origin), x, y)).collect();
        let depth = bvh_depth(&exported);
        if depth > pathtrace::STACK {
            return Err(format!("a BVH {} levels deep (the shader's stack holds {})", depth, pathtrace::STACK));
        }
        if prims.len() > pathtrace::MAX_LEAF_ITEMS as usize {
            return Err(format!("{} primitives (the shader's traversal counts to {})", prims.len(), pathtrace::MAX_LEAF_ITEMS));
        }
        // The packed buffer keeps a primitive's material and emitter as floats.
        if prims.len().max(flat.materials.len()) >= pathtrace::MAX_PACKED_INDEX as usize {
            return Err(format!("{} primitives or materials (the shader's buffers count to {})", prims.len().max(flat.materials.len()), pathtrace::MAX_PACKED_INDEX));
        }
        let nodes = gpu_nodes(&exported)?;

        // Tables: emitters' primitives and their cumulative distribution, then the sky.
        let mut tables: Vec<f32> = Vec::new();
        let emitter_off = tables.len() as u32;
        tables.extend(flat.emitters.iter().map(|&p| f32::from_bits(p)));
        let cdf_off = tables.len() as u32;
        let powers: Vec<f64> = (0..flat.emitters.len()).map(|e| flat.emitter_choice.probability(e)).collect();
        tables.extend(cdf(&powers));
        let env_off = tables.len() as u32;
        // The sky as seen from the camera, as the CPU integrators see it from each ray's
        // origin: exact for camera rays, near enough for paths that stay near the camera (from
        // the world's origin, an orbiting camera saw the night side's ground-level sky).
        let viewpoint = crate::environment::viewpoint(s);
        // The map's size: full where the shader reads the light from it, the CPU's where it
        // only chooses directions (`SAMPLING_WIDTH`).
        let per_ray = matches!(s.render.background, crate::scene::Background::Space(_))
            || flat.sky.as_ref().is_some_and(|sky| sky.medium.material_at(Vector3::zeros()).air().is_some());
        let (env_width, env_height, scale) = if per_ray { (SAMPLING_WIDTH, SAMPLING_HEIGHT, 1) } else { (ENV_WIDTH, ENV_HEIGHT, VISIBILITY_SCALE) };
        let texels: Vec<Color> = (0..env_width * env_height).into_par_iter().map(|i| {
            let d = env_direction(i % env_width, i / env_width, env_width, env_height);
            crate::wavefront::integrator::sky_radiance(&Ray { ro: viewpoint, rd: d }, s, flat)
        }).collect();
        for t in &texels {
            tables.extend([t.rgb.x as f32, t.rgb.y as f32, t.rgb.z as f32]);
        }
        // Rows by brightness times sin θ (a row's share of the sphere) times how much of the
        // sky the scene lets through from the camera, as the CPU's `Environment` (so no shadow
        // rays are spent on the ground), then within each row. The visibility is found on a
        // grid of `scale`² texels, the CPU's resolution, for a ray per cell.
        let (vw, vh) = (env_width / scale, env_height / scale);
        let visible: Vec<f64> = (0..vw * vh).into_par_iter().map(|i| {
            let (x, y) = ((i % vw) * scale + scale / 2, (i / vw) * scale + scale / 2);
            crate::environment::visibility(s, &viewpoint, &env_direction(x, y, env_width, env_height))
        }).collect();
        let weight = |x: usize, y: usize| {
            let theta = (y as f64 + 0.5) / env_height as f64 * std::f64::consts::PI;
            let v = visible[(y / scale) * vw + x / scale];
            crate::color::luminance(&texels[y * env_width + x]).max(0.) * theta.sin() * v + 1e-9
        };
        let rows: Vec<f64> = (0..env_height).map(|y| (0..env_width).map(|x| weight(x, y)).sum()).collect();
        let marginal_off = tables.len() as u32;
        tables.extend(cdf(&rows));
        let conditional_off = tables.len() as u32;
        for y in 0..env_height {
            let row: Vec<f64> = (0..env_width).map(|x| weight(x, y)).collect();
            tables.extend(cdf(&row));
        }

        // The sea's waves (`Ocean::export`); `dims.z` says where, plus one.
        // Then its foam (`Foam::export`), which the sea's record says the place of.
        let ocean_off = ocean.map_or(0, |(waves, _)| {
            let object = &flat.shapes[sea.unwrap()].0;
            let at = tables.len();
            tables.extend(waves.export());
            // In the render's frame: where its waves are displaced, and the planet's centre
            // with |c|² - r² (or for a plane, sea level in the centre's y).
            let radius = tables[at + pathtrace::OCEAN_RADIUS as usize] as f64;
            let centre = Vector3::new(0., -radius, 0.) - origin;
            tables[at + pathtrace::OCEAN_CENTRE as usize] -= origin.x as f32;
            tables[at + pathtrace::OCEAN_CENTRE as usize + 1] -= origin.z as f32;
            for k in 0..3 {
                tables[at + pathtrace::OCEAN_PLANET as usize + k] = centre[k] as f32;
            }
            tables[at + pathtrace::OCEAN_EXCESS as usize] = (centre.norm_squared() - radius * radius) as f32;
            if let Some(foam) = object.medium.material_at(Vector3::zeros()).water().and_then(|w| w.foam) {
                tables[at + pathtrace::OCEAN_FOAM as usize] = f32::from_bits((tables.len() - at) as u32);
                tables.extend(foam);
            }
            at as u32 + 1
        });
        let media = media_records(&mut tables, &flat.media, origin);
        // The air: the sky's (for escaping rays), the aerial perspective's, the sun's.
        let sky_air = flat.sky.as_ref().and_then(|sky| sky.medium.material_at(Vector3::zeros()).air());
        let sky_off = sky_air.map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a, origin));
        let aerial_off = s.atmosphere.as_ref().map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a.export(), origin));
        let sun_off = s.sun.as_ref().and_then(|sun| sun.atmosphere.as_ref()).map_or(pathtrace::NO_AIR, |a| air_record(&mut tables, &a.export(), origin));
        // Space, if it is the background.
        let space_off = match &s.render.background {
            crate::scene::Background::Space(space) => space_record(&mut tables, space),
            _ => pathtrace::NO_AIR,
        };

        let mut flags = 0u32;
        if aerial_off != pathtrace::NO_AIR { flags |= pathtrace::FLAG_AERIAL; }
        // Crepuscular rays: the sun's shadows in the air the camera looks through.
        if aerial_off != pathtrace::NO_AIR && s.sun.is_some() && s.atmosphere.as_ref().is_some_and(|a| a.crepuscular_rays) {
            flags |= pathtrace::FLAG_CREPUSCULAR;
        }
        if s.render.sample_lights { flags |= pathtrace::FLAG_SAMPLE_LIGHTS; }
        if s.environment.is_some() { flags |= pathtrace::FLAG_SAMPLE_ENV; }
        let (sun_dir, sun_irr) = match &s.sun {
            Some(sun) => {
                flags |= pathtrace::FLAG_SUN;
                // Above the air: the shader reddens it at each point (`sun_transmittance`).
                let e = sun.irradiance;
                (v4(&sun.direction, sun.angular_radius.cos() as f32), Vec4::new(e.rgb.x as f32, e.rgb.y as f32, e.rgb.z as f32, sun.angular_radius as f32))
            },
            None => (Vec4::new(0., 1., 0., 1.), Vec4::ZERO),
        };
        let (rr, rr_min) = match s.render.path_termination {
            PathTermination::RussianRoulette { min_depth } => (1, min_depth as f32),
            _ => (0, 0.),
        };
        let params = Params {
            dims: UVec4::new(s.image.width as u32, s.image.height as u32, ocean_off, s.render.max_depth as u32),
            // (The sea is tested with the planes.)
            counts: UVec4::new(flat.prims.len() as u32, (flat.planes.len() + flat.shapes.len()) as u32, flat.emitters.len() as u32, s.lights.len() as u32),
            offs: UVec4::new(emitter_off, cdf_off, env_off, marginal_off),
            offs2: UVec4::new(conditional_off, env_width as u32, env_height as u32, flags),
            misc: Vec4::new(s.render.regularise as f32, s.black_threshold as f32, s.render.shadow_bias as f32, rr_min),
            sun_dir,
            sun_irr,
            seed: UVec4::new(s.render.seed as u32, rr, 0, 1),
            air: UVec4::new(sky_off, aerial_off, sun_off, space_off),
            // (A glitter probe finds the sun's glitter on water the kernels shade: `host_kernels`.)
            media: UVec4::new(media, flat.media.len() as u32, match (&s.sea, sea, kernels.iter().any(|k| k.is_some())) {
                (Some(glitter), Some(_), true) => (glitter.roughness as f32).to_bits(),
                _ => 0,
            }, 0),
            origin: v4(origin, 0.),
        };
        Ok(Buffers { params, prims: pathtrace::pack_prims(&prims), prim_records: prims, nodes, materials, lights, tables, hosts: hosts > 0, host_shapes, unported, origin: *origin })
    }
}

/// Camera rays for sample `k` of every pixel, in the frame about `origin`.
pub fn camera_rays(s: &Scene, k: usize, origin: &Vector3<f64>) -> Vec<GpuRay> {
    camera_rays_for(s, &vec![Some(k as u32); s.image.width * s.image.height], origin)
}

/// The camera ray of each pixel's sample `samples[pixel]`; a dead ray (as one the lens
/// stops) for a pixel that takes none.
pub fn camera_rays_for(s: &Scene, samples: &[Option<u32>], origin: &Vector3<f64>) -> Vec<GpuRay> {
    (0..s.image.width * s.image.height).into_par_iter().map(|index| match samples[index] {
        Some(k) => camera_ray(s, index, k as usize, origin),
        None => GpuRay::zeroed(),
    }).collect()
}

/// Pixel `index`'s camera ray for its sample `k` (zero if the lens stops it).
pub fn camera_ray(s: &Scene, index: usize, k: usize, origin: &Vector3<f64>) -> GpuRay {
    let (w, h) = (s.image.width, s.image.height);
    let (x, y) = (index % w, index / w);
    let stream = k as u64 * (w * h) as u64 + index as u64;
    crate::geometry::seed_rand(s.render.seed ^ 0x5bd1_e995, stream);
    let mut sampler = crate::sobol::SampleState::new(crate::sobol::hash(s.render.seed, index as u64), k as u32);
    let ((fx, wx), (fy, wy)) = (s.render.filter.sample_weighted(sampler.next()), s.render.filter.sample_weighted(sampler.next()));
    match s.camera.get_ray_weighted(x as f64 / w as f64, y as f64 / h as f64, (0.5 + fx) / w as f64, (0.5 + fy) / h as f64) {
        // d.w: the sample's number plus one, for `pathtrace::start_paths`.
        Some((ray, weight)) => GpuRay { o: v4(&(ray.ro - origin), (weight * wx * wy) as f32), d: v4(&ray.rd.normalize(), (k + 1) as f32) },
        None => GpuRay::zeroed(),
    }
}

#[cfg(test)]
mod measure {
    use std::cell::Cell;

    /// A medium that counts what a walk asks of it.
    struct Counting<'a, F: shared::Real, M: shared::media::Medium<F>> {
        medium: &'a M,
        densities: Cell<u64>,
        bounds: Cell<u64>,
        short: Cell<u64>,
        marker: std::marker::PhantomData<F>,
    }

    impl<F: shared::Real, M: shared::media::Medium<F>> shared::media::Medium<F> for Counting<'_, F, M> {
        fn density(&self, x: F, y: F, z: F) -> F {
            self.densities.set(self.densities.get() + 1);
            self.medium.density(x, y, z)
        }
        fn bound(&self) -> F { self.medium.bound() }
        fn majorant_from(&self, x: F, y: F, z: F, dx: F, dy: F, dz: F) -> (F, F) {
            self.bounds.set(self.bounds.get() + 1);
            let (m, l) = self.medium.majorant_from(x, y, z, dx, dy, dz);
            if l < F::of(1) { self.short.set(self.short.get() + 1); }
            (m, l)
        }
    }

    struct Tables64<'a>(&'a [f32]);
    impl shared::Tables<f64> for Tables64<'_> {
        fn value(&self, i: u32) -> f64 { self.0[i as usize] as f64 }
        fn index(&self, i: u32) -> u32 { self.0[i as usize] as u32 }
    }
    struct Rng(crate::geometry::Pcg32);
    impl shared::Random<f32> for Rng { fn next(&mut self) -> f32 { (self.0.next_f64() as f32).min(0.999_999_9) } }
    impl shared::Random<f64> for Rng { fn next(&mut self) -> f64 { self.0.next_f64() } }

    /// The walk to a first collision along a cloud scene's own camera rays (`RAYS_SCENE`), as
    /// the kernels walk it and as f64 would the same layer: what each asks of the medium.
    #[test]
    #[ignore]
    fn measure_camera_walks() {
        use crate::wavefront::flat::FlatScene;
        use shared::media::OnPlanet;
        let file = std::env::var("RAYS_SCENE").unwrap_or("demo/scenes/nubis-clouds.json".into());
        let s = crate::scenefile::SceneFile::from_file(&file, &crate::scenefile::Overrides { width: Some(80), height: Some(50), seed: None });
        let flat = FlatScene::build(&s).unwrap();
        let b = super::Buffers::build(&s, &flat).unwrap();
        let medium = &flat.media[0];
        let crate::volume::FieldExport::Nubis { record, radius } = &medium.field else { panic!("a Nubis layer") };
        let base = b.params.media.x;
        let kernel = pathtrace::load_medium(&b.tables, base);
        let f32s = pathtrace::load_nubis(record, 0);
        let (r, w) = (&f32s.recipe, &f32s.weather);
        let f64s = shared::clouds::NubisField::<f64> {
            recipe: shared::clouds::Nubis { base: r.base as f64, top: r.top as f64, coverage: r.coverage as f64, cloud_type: r.cloud_type as f64,
                shape_size: r.shape_size as f64, detail_size: r.detail_size as f64, erosion: r.erosion as f64, perm: r.perm, worley: r.worley },
            max_density: f32s.max_density as f64,
            weather: shared::clouds::WeatherGrid { x: w.x as f64, z: w.z as f64, texel: w.texel as f64, n: w.n, corners: w.corners, maxima: w.maxima },
        };
        let t64 = Tables64(record);
        let m64 = shared::clouds::NubisMedium(&f64s, &t64);
        let p64 = OnPlanet { radius: *radius, medium: &m64 };
        let c32 = Counting { medium: &kernel, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
        let c64 = Counting { medium: &p64, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
        let (mut n, mut hits32, mut hits64, mut length) = (0u64, 0u64, 0u64, 0f64);
        let (mut us32, mut us64) = (0f64, 0f64);
        let mut per_ray = Vec::new();
        for index in 0..s.image.width * s.image.height {
            let ray = super::camera_ray(&s, index, 0, &crate::na::Vector3::zeros());
            if ray.d.w < 0.5 { continue; }
            let (o, d) = (crate::na::Vector3::new(ray.o.x as f64, ray.o.y as f64, ray.o.z as f64), crate::na::Vector3::new(ray.d.x as f64, ray.d.y as f64, ray.d.z as f64));
            let inv = crate::na::Vector3::new(1. / d.x, 1. / d.y, 1. / d.z);
            let Some((near, far)) = medium.bounds.entry_exit(&o, &inv) else { continue };
            let near = near.max(0.);
            if near >= far { continue; }
            n += 1;
            length += far - near;
            if std::env::var("RAYS_TRACE_WALK").is_ok_and(|v| v.parse::<usize>() == Ok(index)) {
                use shared::media::Medium;
                let (o32, d32) = ((o.x as f32, o.y as f32, o.z as f32), (d.x as f32, d.y as f32, d.z as f32));
                let (mut t, mut shown) = (near as f32, 0);
                while t < far as f32 && shown < 40 {
                    let p = (o32.0 + d32.0 * t, o32.1 + d32.1 * t, o32.2 + d32.2 * t);
                    let (m, l) = kernel.majorant_from(p.0, p.1, p.2, d32.0, d32.1, d32.2);
                    let q = shared::media::planet_local(*radius as f32, p.0, p.1, p.2);
                    if l < 1. {
                        println!("t {:.4}: bound {} for {:e} m; local ({:.4}, {:.4}, {:.4}); texel coords ({:.5}, {:.5})", t, m, l, q.0, q.1, q.2, (q.0 - w.x) / w.texel, (q.2 - w.z) / w.texel);
                        shown += 1;
                    }
                    t = (t + l.max(t.abs().max(1.) * 1e-6)).min(far as f32);
                }
                println!("ray {}: from {:?} along {:?}, box {:.1} to {:.1}; layer {} to {}", index, o32, d32, near, far, r.base, r.top);
            }
            let (mut a, mut c) = (Rng(crate::geometry::Pcg32::new(7, index as u64)), Rng(crate::geometry::Pcg32::new(7, index as u64)));
            let start = std::time::Instant::now();
            hits32 += shared::media::delta_track(&c32, (o.x as f32, o.y as f32, o.z as f32), (d.x as f32, d.y as f32, d.z as f32), near as f32, far as f32, 1., &mut a).is_some() as u64;
            us32 += start.elapsed().as_secs_f64() * 1e6;
            let start = std::time::Instant::now();
            hits64 += shared::media::delta_track(&c64, (o.x, o.y, o.z), (d.x, d.y, d.z), near, far, 1., &mut c).is_some() as u64;
            us64 += start.elapsed().as_secs_f64() * 1e6;
            per_ray.push((c32.short.get(), index));
        }
        let mut last = 0;
        let mut by_ray: Vec<(u64, usize)> = per_ray.iter().map(|&(total, i)| { let d = total - last; last = total; (d, i) }).collect();
        by_ray.sort();
        // (`RAYS_TRACE_WALK=<ray>` prints such a ray's short steps: how a stall is found.)
        println!("most steps under a metre, and the ray: {:?}; {} rays with over 100", by_ray.iter().rev().take(5).collect::<Vec<_>>(), by_ray.iter().filter(|r| r.0 > 100).count());
        let per = |v: u64| v as f64 / n as f64;
        println!("{} rays through the box, {:.0} m of it each on average; radius {:e}", n, length / n as f64, radius);
        println!("f32 (the kernel's medium): {:.1} bounds ({:.1} under a metre), {:.1} densities, {:.0}% collide, {:.1} us a ray", per(c32.bounds.get()), per(c32.short.get()), per(c32.densities.get()), 100. * per(hits32), us32 / n as f64);
        println!("f64 (the same layer):      {:.1} bounds ({:.1} under a metre), {:.1} densities, {:.0}% collide, {:.1} us a ray", per(c64.bounds.get()), per(c64.short.get()), per(c64.densities.get()), 100. * per(hits64), us64 / n as f64);
    }

    /// How many steps a walk through a Nubis layer round the planet takes at f32, as the
    /// kernels walk it, against f64, as the renderer does: the same layer, the same rays.
    #[test]
    #[ignore]
    fn measure_walks() {
        use crate::procedural::nubis_clouds::NubisCloudLayer;
        use shared::media::{Medium, OnPlanet};
        let mut layer = NubisCloudLayer::new(1500., 3000., crate::na::Vector2::new(0., 0.), 80000., 4);
        (layer.coverage, layer.max_density) = (0.55, 0.03);
        let record = layer.export();
        let f32s = pathtrace::load_nubis(&record, 0);
        let (r, w) = (&f32s.recipe, &f32s.weather);
        let f64s = shared::clouds::NubisField::<f64> {
            recipe: shared::clouds::Nubis { base: r.base as f64, top: r.top as f64, coverage: r.coverage as f64, cloud_type: r.cloud_type as f64,
                shape_size: r.shape_size as f64, detail_size: r.detail_size as f64, erosion: r.erosion as f64, perm: r.perm, worley: r.worley },
            max_density: f32s.max_density as f64,
            weather: shared::clouds::WeatherGrid { x: w.x as f64, z: w.z as f64, texel: w.texel as f64, n: w.n, corners: w.corners, maxima: w.maxima },
        };
        let (t32, t64) = (pathtrace::FloatTables(&record), Tables64(&record));
        let (m32, m64) = (shared::clouds::NubisMedium(&f32s, &t32), shared::clouds::NubisMedium(&f64s, &t64));
        for radius in [0f64, 6.36e6] {
            let (p32, p64) = (OnPlanet { radius: radius as f32, medium: &m32 }, OnPlanet { radius, medium: &m64 });
            let c32 = Counting { medium: &p32, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
            let c64 = Counting { medium: &p64, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
            let f32flat = Counting { medium: &m32, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
            let f64flat = Counting { medium: &m64, densities: Cell::new(0), bounds: Cell::new(0), short: Cell::new(0), marker: std::marker::PhantomData };
            let mut rng = crate::geometry::Pcg32::new(3, 0);
            let n = 20_000;
            let (mut through32, mut through64) = (0f64, 0f64);
            for i in 0..n {
                // From the ground up through the layer, near and far, and along it.
                let o = (rng.next_f64() * 60000. - 30000., 2., rng.next_f64() * 60000. - 30000.);
                let d = crate::na::Vector3::new(rng.next_f64() - 0.5, 0.02 + rng.next_f64() * 0.6, rng.next_f64() - 0.5).normalize();
                let (mut a, mut b) = (Rng(crate::geometry::Pcg32::new(7, i)), Rng(crate::geometry::Pcg32::new(7, i)));
                let far = 3000. / d.y;
                if radius > 0. {
                    through32 += shared::media::ratio_track(&c32, (o.0 as f32, o.1 as f32, o.2 as f32), (d.x as f32, d.y as f32, d.z as f32), 0., far as f32, 0., 1., 1., &mut a).unwrap_or(0.) as f64;
                    through64 += shared::media::ratio_track(&c64, o, (d.x, d.y, d.z), 0., far, 0., 1., 1., &mut b).unwrap_or(0.);
                } else {
                    through32 += shared::media::ratio_track(&f32flat, (o.0 as f32, o.1 as f32, o.2 as f32), (d.x as f32, d.y as f32, d.z as f32), 0., far as f32, 0., 1., 1., &mut a).unwrap_or(0.) as f64;
                    through64 += shared::media::ratio_track(&f64flat, o, (d.x, d.y, d.z), 0., far, 0., 1., 1., &mut b).unwrap_or(0.);
                }
            }
            let (x, y) = if radius > 0. { ((c32.bounds.get(), c32.densities.get(), c32.short.get()), (c64.bounds.get(), c64.densities.get(), c64.short.get())) }
                else { ((f32flat.bounds.get(), f32flat.densities.get(), f32flat.short.get()), (f64flat.bounds.get(), f64flat.densities.get(), f64flat.short.get())) };
            // And the time of each, uncounted.
            let time = |f: &mut dyn FnMut(u64) -> f64| {
                let start = std::time::Instant::now();
                let sum: f64 = (0..2000).map(|i| f(i)).sum();
                (start.elapsed().as_secs_f64() * 1e6 / 2000., sum)
            };
            let ray = |i: u64| {
                let mut rng = crate::geometry::Pcg32::new(11, i);
                let o = (rng.next_f64() * 60000. - 30000., 2., rng.next_f64() * 60000. - 30000.);
                (o, crate::na::Vector3::new(rng.next_f64() - 0.5, 0.02 + rng.next_f64() * 0.6, rng.next_f64() - 0.5).normalize())
            };
            let (us32, _) = time(&mut |i| {
                let (o, d) = ray(i);
                let mut a = Rng(crate::geometry::Pcg32::new(7, i));
                let (o, d, far) = ((o.0 as f32, o.1 as f32, o.2 as f32), (d.x as f32, d.y as f32, d.z as f32), (3000. / d.y) as f32);
                (if radius > 0. { shared::media::ratio_track(&p32, o, d, 0., far, 0., 1., 1., &mut a) } else { shared::media::ratio_track(&m32, o, d, 0., far, 0., 1., 1., &mut a) }).unwrap_or(0.) as f64
            });
            let (us64, _) = time(&mut |i| {
                let (o, d) = ray(i);
                let mut b = Rng(crate::geometry::Pcg32::new(7, i));
                (if radius > 0. { shared::media::ratio_track(&p64, o, (d.x, d.y, d.z), 0., 3000. / d.y, 0., 1., 1., &mut b) } else { shared::media::ratio_track(&m64, o, (d.x, d.y, d.z), 0., 3000. / d.y, 0., 1., 1., &mut b) }).unwrap_or(0.)
            });
            println!("radius {:e}: {:.1} us a ray at f32, {:.1} at f64", radius, us32, us64);
            println!("radius {:e}: a ray, f32: {:.1} bounds ({:.1} under a metre), {:.1} densities, through {:.4}; f64: {:.1} bounds ({:.1} under a metre), {:.1} densities, through {:.4}",
                radius, x.0 as f64 / n as f64, x.2 as f64 / n as f64, x.1 as f64 / n as f64, through32 / n as f64, y.0 as f64 / n as f64, y.2 as f64 / n as f64, y.1 as f64 / n as f64, through64 / n as f64);
        }
    }

    /// Perlin's noise at `N` points at once, a lane each: `shared::noise::perlin`'s operations
    /// in its order, lane by lane (so each lane's result is the scalar one's to the bit), the
    /// table read a lane at a time and the arithmetic written as loops over the lanes for the
    /// compiler to turn into vector instructions. A prototype, to measure what lanes are worth.
    fn perlin_lanes<const N: usize>(tables: &[f32], perm: u32, x: [f32; N], y: [f32; N], z: [f32; N]) -> [f32; N] {
        let p = |i: u32| tables[(perm + i) as usize] as u32;
        let fade = |t: f32| t * t * t * (t * (t * 6. - 15.) + 10.);
        let (mut fx, mut fy, mut fz) = ([0f32; N], [0f32; N], [0f32; N]);
        let mut hash = [[0u32; N]; 8];
        for l in 0..N {
            let (x0, y0, z0) = (x[l].floor(), y[l].floor(), z[l].floor());
            let (xi, yi, zi) = ((x0 as i32 & 255) as u32, (y0 as i32 & 255) as u32, (z0 as i32 & 255) as u32);
            (fx[l], fy[l], fz[l]) = (x[l] - x0, y[l] - y0, z[l] - z0);
            let a = p(xi) + yi;
            let (aa, ab) = (p(a) + zi, p(a + 1) + zi);
            let b = p(xi + 1) + yi;
            let (ba, bb) = (p(b) + zi, p(b + 1) + zi);
            for (k, h) in [p(aa), p(ba), p(ab), p(bb), p(aa + 1), p(ba + 1), p(ab + 1), p(bb + 1)].into_iter().enumerate() {
                hash[k][l] = h % 12;
            }
        }
        // The eight corners' gradients, then the blend: all lanes at once.
        let mut out = [0f32; N];
        let mut g = [[0f32; N]; 8];
        for k in 0..8 {
            let (ox, oy, oz) = ((k & 1) as f32, ((k >> 1) & 1) as f32, ((k >> 2) & 1) as f32);
            for l in 0..N {
                let (x, y, z) = (fx[l] - ox, fy[l] - oy, fz[l] - oz);
                let h = hash[k][l];
                let first = if h < 8 { x } else { y };
                let second = if h < 4 { y } else { z };
                let first = if h & 1 == 0 { first } else { -first };
                let second = if h & 2 == 0 { second } else { -second };
                g[k][l] = first + second;
            }
        }
        for l in 0..N {
            let (u, v, w) = (fade(fx[l]), fade(fy[l]), fade(fz[l]));
            let mix = |a: f32, b: f32, t: f32| a + t * (b - a);
            let near = mix(mix(g[0][l], g[1][l], u), mix(g[2][l], g[3][l], u), v);
            let far = mix(mix(g[4][l], g[5][l], u), mix(g[6][l], g[7][l], u), v);
            out[l] = mix(near, far, w);
        }
        out
    }

    /// Noise in lanes against noise a point at a time: the same bits, and the time of each.
    #[test]
    #[ignore]
    fn measure_lanes() {
        use crate::procedural::benard_altocumulus::BenardAltocumulus;
        let record = BenardAltocumulus::new(4000., 200., 2).export();
        let tables = pathtrace::FloatTables(&record);
        let perm = pathtrace::load_benard(&record, 0).warp_perm;
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let n = 1 << 21;
        // Scattered points, and points along rays (as a walk meets them: neighbours alike).
        for (what, along) in [("scattered", false), ("along rays", true)] {
            let mut at = (0f32, 0f32, 0f32);
            let points: Vec<(f32, f32, f32)> = (0..n).map(|i| {
                if !along || i % 64 == 0 {
                    at = ((rng.next_f64() * 60. - 30.) as f32, (rng.next_f64() * 60.) as f32, (rng.next_f64() * 60. - 30.) as f32);
                } else {
                    at = (at.0 + 0.031, at.1 + 0.007, at.2 + 0.019);
                }
                at
            }).collect();
            let start = std::time::Instant::now();
            let one: Vec<f32> = points.iter().map(|p| shared::noise::perlin(&tables, perm, p.0, p.1, p.2)).collect();
            let scalar = start.elapsed().as_secs_f64() * 1e9 / n as f64;
            let mut line = format!("{:>10}: {:5.1} ns a point one at a time", what, scalar);
            for lanes in [4usize, 8, 16] {
                let start = std::time::Instant::now();
                let mut many = Vec::with_capacity(n);
                match lanes {
                    4 => for c in points.chunks_exact(4) { many.extend(perlin_lanes::<4>(&record, perm, std::array::from_fn(|l| c[l].0), std::array::from_fn(|l| c[l].1), std::array::from_fn(|l| c[l].2))); },
                    8 => for c in points.chunks_exact(8) { many.extend(perlin_lanes::<8>(&record, perm, std::array::from_fn(|l| c[l].0), std::array::from_fn(|l| c[l].1), std::array::from_fn(|l| c[l].2))); },
                    _ => for c in points.chunks_exact(16) { many.extend(perlin_lanes::<16>(&record, perm, std::array::from_fn(|l| c[l].0), std::array::from_fn(|l| c[l].1), std::array::from_fn(|l| c[l].2))); },
                }
                let t = start.elapsed().as_secs_f64() * 1e9 / n as f64;
                assert!(one.iter().zip(&many).all(|(a, b)| a.to_bits() == b.to_bits()), "{} lanes differ from one", lanes);
                line += &format!(", {:.1} in {} lanes ({:.2}x)", t, lanes, scalar / t);
            }
            println!("{}", line);
        }
    }

    /// What the clouds' density costs, part by part, at f32 as the kernels run it: for
    /// deciding what to compute several at a time. `--ignored --nocapture`.
    #[test]
    #[ignore]
    fn measure_density() {
        use crate::procedural::benard_altocumulus::BenardAltocumulus;
        use pathtrace::FloatTables;
        let layer = BenardAltocumulus::new(4000., 200., 2);
        let record = layer.export();
        let tables = FloatTables(&record);
        let b = pathtrace::load_benard(&record, 0);
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let n = 2_000_000;
        let points: Vec<(f32, f32, f32)> = (0..n).map(|_| ((rng.next_f64() * 16000. - 8000.) as f32, (b.bottom() + rng.next_f64() as f32 * (b.top() - b.bottom())), (rng.next_f64() * 16000. - 8000.) as f32)).collect();
        let time = |what: &str, f: &dyn Fn(&(f32, f32, f32)) -> f32| {
            let start = std::time::Instant::now();
            let sum: f32 = points.iter().map(f).sum();
            println!("{:>28}: {:6.1} ns a call (sum {})", what, start.elapsed().as_secs_f64() * 1e9 / n as f64, sum);
        };
        time("perlin", &|p| shared::noise::perlin(&tables, b.warp_perm, p.0 / 300., p.1 / 300., p.2 / 300.));
        time("fbm, 2 octaves", &|p| shared::noise::fbm(&tables, b.warp_perm, p.0 / 300., 0.25, p.2 / 300., 2, 0.5, 2.));
        time("fbm, 3 octaves", &|p| shared::noise::fbm(&tables, b.patch_perm, p.0 / 12000., 0.5, p.2 / 12000., 3, 0.5, 2.));
        time("worley_fbm", &|p| shared::clouds::worley_fbm(&tables, b.worley, p.0 / 80., p.1 / 80., p.2 / 80.));
        time("cell hash x 50", &|p| (0..50).map(|k| shared::noise::cell_hash::<f32>(p.0 as i32 as u32, k, 0, 7)).sum());
        time("altocumulus's fraction", &|p| b.fraction(&tables, p.0, p.1, p.2));
        let cloudy = points.iter().filter(|p| b.fraction(&tables, p.0, p.1, p.2) > 0.).count();
        println!("{} of {} points in cloud", cloudy, n);
    }
}
