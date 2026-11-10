//! # Procedural grass
//!
//! A field of individual grass blades growing on a heightmap (see `heightmap`), so it can cover
//! any terrain. A field mixes one or more *species*, which say what the blades look like, and is
//! shaped by *conditions*: wind, drought, frost, rain, mowing, trampling and where grass can
//! grow at all.
//!
//! ```json
//! { "type": "grass", "heightmap": "$ground", "density": 800, "seed": 1,
//!   "min": [-10, -10], "max": [10, 10],
//!   "species": ["meadow", { "preset": "tall", "weight": 0.3, "height": 1.2 }],
//!   "conditions": { "preset": "summer", "wind": { "direction": [1, 0.3], "strength": 0.6 } } }
//! ```
//!
//! - `heightmap`: as for `terrain`; use the same one so the grass sits on the ground
//! - `density`: blades per square metre (default 600). Short grass needs many to cover the
//!   ground: a lawn looks like turf from about 5000
//! - `min`, `max`: [x, z] corners of the area to grow in (default: the whole heightmap)
//! - `seed`: the same seed gives the same field
//! - `material`: render every blade with this scene material instead (eg. for debugging)
//!
//! ## Species
//!
//! Each entry of `species` is a preset name, or an object with a `preset` (default `meadow`)
//! and fields overriding it. Presets: `lawn`, `meadow`, `fescue`, `tall`, `tussock`,
//! `savanna`, `reed`. Fields:
//!
//! - `weight`: relative abundance (default 1)
//! - `height` (m), `height_variation` (fraction either way)
//! - `width` (m, at the base); `tip`: how sharply the width tapers to the tip - 1 gives a
//!   triangle, larger numbers keep the sides parallel for longer
//! - `stiffness` (0 to 1): how well the blade resists drooping and wind
//! - `lean`: largest random lean from vertical, degrees; `twist`: largest twist along the blade,
//!   degrees
//! - `clump_size`: blades per tuft; `clump_radius` (m); `clump_spread` (0 to 1): how much the
//!   blades of a tuft fan outward
//! - `base_color`, `tip_color`: the living blade, from base to tip; `dry_color`: dead
//! - `dryness` (0 to 1): how dry this species is regardless of conditions
//! - `translucency` (0 to 1): how much light passes through a blade; `sheen` (0 to 1): waxy
//!   highlights
//! - `segments`: straight pieces each blade is built from (1 to 8)
//! - `normal_alignment` (0 to 1): 0 grows straight up, 1 perpendicular to the slope
//!
//! ## Conditions
//!
//! `conditions` is a preset name, or an object with a `preset` (default `summer`) and fields
//! overriding it. Presets: `lush`, `summer`, `drought`, `autumn`, `after_rain`, `frost`,
//! `windy`, `manicured`, `trampled`. Fields:
//!
//! - `wind`: `{ "direction": [x, z], "strength": 0 to 1, "gustiness": 0 to 1, "scale": m }`.
//!   Blades bend downwind, more where gusts (noise of the given scale) are stronger
//! - `dryness` (0 to 1): turns blades from green toward straw; `dead_fraction`: blades that
//!   are dead outright, which are straw coloured and droop
//! - `wetness` (0 to 1): darker blades with a glossy coat
//! - `frost` (0 to 1): pale, icy blade tips
//! - `patchiness` (0 to 1): bare and thin patches, of size `patch_scale` (m); grass at their
//!   edges is shorter and drier
//! - `mow_height` (m): blades are cut flat at this height. `mow_stripes`: `{ "width": m,
//!   "direction": [x, z] }` - alternate stripes lean opposite ways, as after a mower
//! - `trampled` (0 to 1): blades pressed over and shortened
//! - `max_slope` (degrees): no grass on slopes steeper than this
//! - `min_altitude`, `max_altitude` (m), `altitude_fade` (m): grow only between these heights,
//!   eg. above a water line or below a tree line
//! - `species_zones` (m): with several species, group them into patches of about this size
//!   rather than mixing them evenly; 0 to mix evenly
//!
//! ## Geometry
//!
//! A blade is a ribbon along a quadratic Bézier curve, its control points found as Klemens
//! Jahrmann and Michael Wimmer do in "Responsive Real-Time Grass Rendering for General 3D
//! Scenes", I3D 2017, doi:10.1145/3023368.3023380: the tip is pushed by lean, droop and wind,
//! kept above the ground, and the middle control point is then placed so the curve keeps the
//! blade's length. The ribbon is cut into a few straight segments, each two triangles, and
//! tapers to the tip.
//!
//! Blades are generated once, when the scene is loaded, deterministically from the seed and
//! position (so a field is the same however many threads build it), and bucketed into a grid of
//! cells in the xz plane by the cells their bounding boxes overlap. A ray walks the cells in
//! order (`heightmap::walk_grid`), skipping cells it passes above, and testing the blades in the
//! others; the first cell with a hit inside it has the nearest hit.
//!
//! Each blade's colour depends on its species, how dry it is and how far along the blade the
//! hit is. The medium finds which blade a point is on, and returns a material from a palette
//! built for the field.

use crate::color::Color;
use crate::geometry::Pcg32;
use crate::intersection::RawIntersection;
use crate::material::grass::GrassMaterial;
use crate::material::model::MaterialModel;
use crate::material::texture::Medium;
use crate::na::Vector3;
use crate::noise::PerlinNoise;
use crate::procedural::heightmap::{HeightGrid, Grid2, walk_grid, ray_spans, intersect_triangle};
use crate::ray::Ray;
use crate::scenefile::SceneFile;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use rayon::prelude::*;
use serde_json::{json, Value, Map};
use std::f64::consts::PI;
use std::sync::Arc;

const MAX_SEGMENTS: usize = 8;
/// Palette levels: of dryness, of random per-blade variation, and along the blade.
const DRY_LEVELS: usize = 8;
const JITTER_LEVELS: usize = 4;
const HEIGHT_LEVELS: usize = 8;
/// Size of the tiles blades are generated in, each with its own random sequence.
const TILE: f64 = 1.;
/// Blades per traversal cell to aim for.
const BLADES_PER_CELL: f64 = 16.;

#[derive(Clone, Debug)]
pub struct Species {
    pub weight: f64,
    pub height: f64,
    pub height_variation: f64,
    pub width: f64,
    pub tip: f64,
    pub stiffness: f64,
    pub lean: f64,
    pub twist: f64,
    pub segments: usize,
    pub clump_size: usize,
    pub clump_radius: f64,
    pub clump_spread: f64,
    pub base_color: Color,
    pub tip_color: Color,
    pub dry_color: Color,
    pub dryness: f64,
    pub translucency: f64,
    pub sheen: f64,
    pub normal_alignment: f64,
    /// Width at the end of each segment, for uncut and cut blades.
    tapers: [[f64; MAX_SEGMENTS + 1]; 2],
}

#[derive(Clone, Debug)]
pub struct Conditions {
    pub wind: Vector3<f64>,
    pub gustiness: f64,
    pub gust_scale: f64,
    pub dryness: f64,
    pub dead_fraction: f64,
    pub wetness: f64,
    pub frost: f64,
    pub patchiness: f64,
    pub patch_scale: f64,
    pub mow_height: Option<f64>,
    pub stripe_width: f64,
    pub stripe_direction: Vector3<f64>,
    pub trampled: f64,
    pub max_slope: f64,
    pub min_altitude: f64,
    pub max_altitude: f64,
    pub altitude_fade: f64,
    pub species_zones: f64,
}

fn species_preset(name: &str) -> Value {
    let lawn = json!({
        "weight": 1, "height": 0.07, "height_variation": 0.25, "width": 0.004, "tip": 4,
        "stiffness": 0.8, "lean": 15, "twist": 20, "segments": 3,
        "clump_size": 1, "clump_radius": 0.0, "clump_spread": 0.0,
        "base_color": [0.035, 0.12, 0.02], "tip_color": [0.09, 0.26, 0.04], "dry_color": [0.42, 0.34, 0.17],
        "dryness": 0, "translucency": 0.45, "sheen": 0.3, "normal_alignment": 0.2
    });
    let overrides = match name {
        "lawn" => json!({}),
        "meadow" => json!({
            "height": 0.35, "height_variation": 0.4, "width": 0.005, "tip": 3, "stiffness": 0.5,
            "lean": 25, "twist": 45, "segments": 5, "clump_size": 3, "clump_radius": 0.03, "clump_spread": 0.4,
            "base_color": [0.045, 0.13, 0.025], "tip_color": [0.15, 0.3, 0.06]
        }),
        "fescue" => json!({
            "height": 0.2, "height_variation": 0.3, "width": 0.0015, "tip": 2, "stiffness": 0.4,
            "lean": 20, "twist": 60, "segments": 4, "clump_size": 12, "clump_radius": 0.025, "clump_spread": 0.7,
            "base_color": [0.05, 0.12, 0.05], "tip_color": [0.13, 0.24, 0.1]
        }),
        "tall" => json!({
            "height": 0.9, "height_variation": 0.3, "width": 0.008, "tip": 2.5, "stiffness": 0.35,
            "lean": 20, "twist": 60, "segments": 6, "clump_size": 5, "clump_radius": 0.05, "clump_spread": 0.5,
            "base_color": [0.06, 0.14, 0.03], "tip_color": [0.24, 0.32, 0.1]
        }),
        "tussock" => json!({
            "height": 0.45, "height_variation": 0.25, "width": 0.004, "tip": 2, "stiffness": 0.3,
            "lean": 10, "twist": 90, "segments": 6, "clump_size": 30, "clump_radius": 0.06, "clump_spread": 0.9,
            "base_color": [0.1, 0.15, 0.04], "tip_color": [0.33, 0.31, 0.15], "dryness": 0.3
        }),
        "savanna" => json!({
            "height": 0.6, "height_variation": 0.3, "width": 0.004, "tip": 2, "stiffness": 0.55,
            "lean": 20, "twist": 120, "segments": 5, "clump_size": 8, "clump_radius": 0.04, "clump_spread": 0.6,
            "base_color": [0.25, 0.22, 0.1], "tip_color": [0.5, 0.42, 0.22], "dry_color": [0.55, 0.45, 0.25],
            "dryness": 0.6, "translucency": 0.3, "sheen": 0.15
        }),
        "reed" => json!({
            "height": 1.6, "height_variation": 0.2, "width": 0.015, "tip": 2, "stiffness": 0.85,
            "lean": 6, "twist": 30, "segments": 6, "clump_size": 4, "clump_radius": 0.05, "clump_spread": 0.3,
            "base_color": [0.07, 0.15, 0.05], "tip_color": [0.2, 0.28, 0.09], "sheen": 0.5
        }),
        _ => panic!("Unknown grass species preset {}", name),
    };
    merge(lawn, &overrides)
}

fn conditions_preset(name: &str) -> Value {
    let summer = json!({
        "wind": { "direction": [1, 0], "strength": 0.15, "gustiness": 0.5, "scale": 6 },
        "dryness": 0.1, "dead_fraction": 0.03, "wetness": 0, "frost": 0,
        "patchiness": 0.1, "patch_scale": 3, "mow_height": null,
        "mow_stripes": { "width": 0, "direction": [1, 0] },
        "trampled": 0, "max_slope": 50, "min_altitude": -1e9, "max_altitude": 1e9,
        "altitude_fade": 0.5, "species_zones": 0
    });
    let overrides = match name {
        "summer" => json!({}),
        "lush" => json!({ "dryness": 0, "dead_fraction": 0.01, "wetness": 0.15, "patchiness": 0 }),
        "drought" => json!({ "dryness": 0.75, "dead_fraction": 0.3, "patchiness": 0.45, "patch_scale": 2 }),
        "autumn" => json!({ "dryness": 0.4, "dead_fraction": 0.12, "wetness": 0.3 }),
        "after_rain" => json!({ "dryness": 0.05, "wetness": 0.9, "wind": { "strength": 0.05 } }),
        "frost" => json!({ "frost": 0.8, "dryness": 0.25, "dead_fraction": 0.08, "wind": { "strength": 0 } }),
        "windy" => json!({ "wind": { "strength": 0.8, "gustiness": 0.7, "scale": 4 } }),
        "manicured" => json!({
            "mow_height": 0.04, "mow_stripes": { "width": 1, "direction": [1, 0] }, "dryness": 0.02,
            "dead_fraction": 0.005, "patchiness": 0, "wind": { "strength": 0 }
        }),
        "trampled" => json!({ "trampled": 0.7, "dryness": 0.2, "dead_fraction": 0.1, "patchiness": 0.3 }),
        _ => panic!("Unknown grass conditions preset {}", name),
    };
    merge(summer, &overrides)
}

/// `base` with the fields of `over` replacing its own, merging objects field by field.
fn merge(mut base: Value, over: &Value) -> Value {
    if let (Some(b), Some(o)) = (base.as_object_mut(), over.as_object()) {
        for (k, v) in o {
            let merged = match b.remove(k) {
                Some(existing) if existing.is_object() && v.is_object() => merge(existing, v),
                _ => v.clone(),
            };
            b.insert(k.clone(), merged);
        }
        return base;
    }
    over.clone()
}

/// A preset name, or an object with an optional "preset" and fields overriding it.
fn with_preset(v: &Value, default: &str, preset: fn(&str) -> Value) -> Value {
    match v {
        Value::String(name) => preset(name),
        Value::Object(fields) => {
            let mut fields: Map<String, Value> = fields.clone();
            let name = fields.remove("preset").and_then(|p| p.as_str().map(String::from)).unwrap_or(default.to_string());
            merge(preset(&name), &Value::Object(fields))
        },
        Value::Null => preset(default),
        _ => panic!("Expected a preset name or an object, got {}", v),
    }
}

fn number(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("grass: \"{}\" should be a number, got {}", k, v[k]))
}

/// A horizontal unit vector from an [x, z] array.
fn direction(v: &Value) -> Vector3<f64> {
    let d = Vector3::new(v[0].as_f64().unwrap_or(1.), 0., v[1].as_f64().unwrap_or(0.));
    if d.norm() < 1e-12 { Vector3::new(1., 0., 0.) } else { d.normalize() }
}

impl Species {
    pub fn from_json(v: &Value) -> Species {
        let v = with_preset(v, "meadow", species_preset);
        let mut sp = Species {
            weight: number(&v, "weight").max(0.),
            height: number(&v, "height").max(1e-4),
            height_variation: number(&v, "height_variation").clamp(0., 0.95),
            width: number(&v, "width").max(1e-5),
            tip: number(&v, "tip").max(0.1),
            stiffness: number(&v, "stiffness").clamp(0., 1.),
            lean: number(&v, "lean").to_radians(),
            twist: number(&v, "twist").to_radians(),
            segments: SceneFile::parse_int(&v["segments"], 4).clamp(1, MAX_SEGMENTS),
            clump_size: SceneFile::parse_int(&v["clump_size"], 1).max(1),
            clump_radius: number(&v, "clump_radius").max(0.),
            clump_spread: number(&v, "clump_spread").clamp(0., 1.),
            base_color: SceneFile::parse_color(&v["base_color"]),
            tip_color: SceneFile::parse_color(&v["tip_color"]),
            dry_color: SceneFile::parse_color(&v["dry_color"]),
            dryness: number(&v, "dryness").clamp(0., 1.),
            translucency: number(&v, "translucency").clamp(0., 1.),
            sheen: number(&v, "sheen").clamp(0., 1.),
            normal_alignment: number(&v, "normal_alignment").clamp(0., 1.),
            tapers: [[0.; MAX_SEGMENTS + 1]; 2],
        };
        for i in 0 ..= sp.segments {
            let s = i as f64 / sp.segments as f64;
            sp.tapers[0][i] = sp.taper(s, false);
            sp.tapers[1][i] = sp.taper(s, true);
        }
        sp
    }

    /// Width at `s` along the blade, as a fraction of the base width.
    fn taper(&self, s: f64, cut: bool) -> f64 {
        if cut { 1. - 0.25 * s } else { 1. - s.powf(self.tip) }
    }
}

impl Conditions {
    pub fn from_json(v: &Value) -> Conditions {
        let v = with_preset(v, "summer", conditions_preset);
        let wind = &v["wind"];
        let stripes = &v["mow_stripes"];
        Conditions {
            wind: direction(&wind["direction"]) * number(wind, "strength").max(0.),
            gustiness: number(wind, "gustiness").clamp(0., 1.),
            gust_scale: number(wind, "scale").max(1e-3),
            dryness: number(&v, "dryness").clamp(0., 1.),
            dead_fraction: number(&v, "dead_fraction").clamp(0., 1.),
            wetness: number(&v, "wetness").clamp(0., 1.),
            frost: number(&v, "frost").clamp(0., 1.),
            patchiness: number(&v, "patchiness").clamp(0., 1.),
            patch_scale: number(&v, "patch_scale").max(1e-3),
            mow_height: v["mow_height"].as_f64(),
            stripe_width: number(stripes, "width").max(0.),
            stripe_direction: direction(&stripes["direction"]),
            trampled: number(&v, "trampled").clamp(0., 1.),
            max_slope: number(&v, "max_slope").to_radians(),
            min_altitude: number(&v, "min_altitude"),
            max_altitude: number(&v, "max_altitude"),
            altitude_fade: number(&v, "altitude_fade").max(1e-6),
            species_zones: number(&v, "species_zones").max(0.),
        }
    }
}

/// A blade, stored compactly: a field can hold millions.
#[derive(Clone, Debug)]
struct Blade {
    /// Bézier control points: root, middle, tip.
    v: [[f32; 3]; 3],
    /// Unit vectors across the blade's face, and the way it faces (and bends).
    side: [f32; 3],
    facing: [f32; 3],
    width: f32,
    twist: f32,
    lo: [f32; 3],
    hi: [f32; 3],
    species: u8,
    /// Palette entry, dryness level * JITTER_LEVELS + jitter.
    shade: u8,
    cut: bool,
}

fn v3(a: [f32; 3]) -> Vector3<f64> { Vector3::new(a[0] as f64, a[1] as f64, a[2] as f64) }
fn f3(a: Vector3<f64>) -> [f32; 3] { [a.x as f32, a.y as f32, a.z as f32] }

/// The edges of a blade's ribbon: left and right points and `s` along the blade, at each end
/// of each segment.
struct Ribbon {
    left: [Vector3<f64>; MAX_SEGMENTS + 1],
    right: [Vector3<f64>; MAX_SEGMENTS + 1],
    n: usize,
}

impl Blade {
    fn point(&self, s: f64) -> Vector3<f64> {
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        a * ((1. - s) * (1. - s)) + b * (2. * s * (1. - s)) + c * (s * s)
    }

    fn tangent(&self, s: f64) -> Vector3<f64> {
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        (b - a) * (2. * (1. - s)) + (c - b) * (2. * s)
    }

    fn side_at(&self, s: f64) -> Vector3<f64> {
        let a = self.twist as f64 * s;
        v3(self.side) * a.cos() + v3(self.facing) * a.sin()
    }

    fn normal_at(&self, s: f64) -> Vector3<f64> {
        let n = self.tangent(s).cross(&self.side_at(s));
        if n.norm() < 1e-12 { v3(self.facing) } else { n.normalize() }
    }

    fn ribbon(&self, sp: &Species) -> Ribbon {
        let n = sp.segments;
        let mut r = Ribbon { left: [Vector3::zeros(); MAX_SEGMENTS + 1], right: [Vector3::zeros(); MAX_SEGMENTS + 1], n };
        let [a, b, c] = [v3(self.v[0]), v3(self.v[1]), v3(self.v[2])];
        let (side, facing) = (v3(self.side), v3(self.facing));
        // The twist turns the side vector by the same angle each segment: rotate it
        // incrementally rather than taking a sine and cosine per point.
        let (sin_step, cos_step) = (self.twist as f64 / n as f64).sin_cos();
        let (mut sin_a, mut cos_a) = (0., 1.);
        let taper = &sp.tapers[usize::from(self.cut)];
        for (i, width) in taper.iter().enumerate().take(n + 1) {
            let s = i as f64 / n as f64;
            let p = a * ((1. - s) * (1. - s)) + b * (2. * s * (1. - s)) + c * (s * s);
            let half = (side * cos_a + facing * sin_a) * (0.5 * self.width as f64 * width);
            r.left[i] = p - half;
            r.right[i] = p + half;
            (sin_a, cos_a) = (sin_a * cos_step + cos_a * sin_step, cos_a * cos_step - sin_a * sin_step);
        }
        r
    }

    /// Nearest hit between t0 and t1: t, and s along the blade.
    fn intersect(&self, sp: &Species, r: &Ray, t0: f64, t1: f64) -> Option<(f64, f64)> {
        let rb = self.ribbon(sp);
        let ds = 1. / rb.n as f64;
        let mut best: Option<(f64, f64)> = None;
        for i in 0 .. rb.n {
            let s0 = i as f64 * ds;
            let tris = [
                (rb.left[i], rb.right[i], rb.left[i + 1], false),
                (rb.right[i], rb.right[i + 1], rb.left[i + 1], true),
            ];
            for (a, b, c, second) in tris {
                if let Some((t, u, v)) = intersect_triangle(r, a, b, c)
                    && t >= t0 && t <= t1 && best.is_none_or(|(bt, _)| t < bt) {
                    let s = s0 + ds * if second { u + v } else { v };
                    best = Some((t, s));
                }
            }
        }
        best
    }

    /// If `p` is on the blade (within `tolerance`), its distance from it and s along it.
    fn locate(&self, sp: &Species, p: Vector3<f64>, tolerance: f64) -> Option<(f64, f64)> {
        let rb = self.ribbon(sp);
        let ds = 1. / rb.n as f64;
        let mut best: Option<(f64, f64)> = None;
        for i in 0 .. rb.n {
            let s0 = i as f64 * ds;
            let tris = [
                (rb.left[i], rb.right[i], rb.left[i + 1], false),
                (rb.right[i], rb.right[i + 1], rb.left[i + 1], true),
            ];
            for (a, b, c, second) in tris {
                if let Some((d, u, v)) = point_on_triangle(p, a, b, c, tolerance)
                    && best.is_none_or(|(bd, _)| d < bd) {
                    let s = s0 + ds * if second { u + v } else { v };
                    best = Some((d, s.clamp(0., 1.)));
                }
            }
        }
        best
    }

    fn bounds_hit(&self, r: &Ray, invrd: &Vector3<f64>, t0: f64, t1: f64) -> bool {
        let b = BBox::new(v3(self.lo), v3(self.hi));
        b.entry_exit(&r.ro, invrd).is_some_and(|(a, z)| z >= t0 && a <= t1)
    }
}

/// Distance of `p` from the triangle's plane and its barycentric (u, v), if it projects inside
/// the triangle (allowing `tolerance`, in barycentric units) and is within `tolerance` of it.
fn point_on_triangle(p: Vector3<f64>, a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>, tolerance: f64) -> Option<(f64, f64, f64)> {
    let (e1, e2) = (b - a, c - a);
    let n = e1.cross(&e2);
    let area2 = n.norm();
    if area2 < 1e-16 { return None }
    let n = n / area2;
    let w = p - a;
    let d = w.dot(&n);
    if d.abs() > tolerance { return None }
    let q = w - n * d;
    // Barycentric coordinates from sub-areas.
    let u = q.cross(&e2).dot(&n) / area2;
    let v = e1.cross(&q).dot(&n) / area2;
    let slack = 0.05;
    if u < -slack || v < -slack || u + v > 1. + slack { return None }
    Some((d.abs(), u, v))
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Fractal noise in [0, 1], stretched so values spread across the range rather than bunching
/// around a half.
fn field_noise(noise: &PerlinNoise, x: f64, z: f64, scale: f64, layer: f64) -> f64 {
    let n = noise.fbm(x / scale + layer * 13.7, layer * 3.1 + 0.5, z / scale - layer * 7.3, 4, 0.5, 2.);
    ((n - 0.5) * 3. + 0.5).clamp(0., 1.)
}

/// SplitMix64, to give each generation tile its own seed.
fn hash(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// What is needed to generate blades.
struct Generator<'a> {
    grid: &'a HeightGrid,
    species: &'a [Species],
    conditions: &'a Conditions,
    noise: PerlinNoise,
    seed: u64,
    min: (f64, f64),
    max: (f64, f64),
    /// Tufts per square metre where nothing thins them.
    clump_density: f64,
}

impl Generator<'_> {
    /// How well grass grows at (x, z), in [0, 1], from patches, slope and altitude.
    fn vigour(&self, x: f64, z: f64, y: f64, normal: &Vector3<f64>) -> (f64, f64) {
        let c = self.conditions;
        let patch = if c.patchiness > 0. {
            let threshold = c.patchiness * 1.1 - 0.1;
            smoothstep(threshold - 0.1, threshold + 0.15, field_noise(&self.noise, x, z, c.patch_scale, self.seed as f64 * 0.1 + 1.))
        } else { 1. };
        let slope = normal.y.clamp(-1., 1.).acos();
        let slope_ok = 1. - smoothstep(c.max_slope - 10f64.to_radians(), c.max_slope, slope);
        let altitude = smoothstep(c.min_altitude, c.min_altitude + c.altitude_fade, y)
            * (1. - smoothstep(c.max_altitude - c.altitude_fade, c.max_altitude, y));
        (patch, slope_ok * altitude)
    }

    fn pick_species(&self, x: f64, z: f64, rng: &mut Pcg32) -> usize {
        let total: f64 = self.species.iter().map(|s| s.weight).sum();
        let random = rng.next_f64();
        let u = if self.conditions.species_zones > 0. && self.species.len() > 1 {
            let zone = field_noise(&self.noise, x, z, self.conditions.species_zones, self.seed as f64 * 0.1 + 2.);
            (0.8 * zone + 0.2 * random).clamp(0., 1. - 1e-12)
        } else { random };
        let mut acc = 0.;
        for (i, s) in self.species.iter().enumerate() {
            acc += s.weight / total;
            if u < acc { return i }
        }
        self.species.len() - 1
    }

    fn tile(&self, tx: usize, tz: usize) -> Vec<Blade> {
        let x0 = self.min.0 + tx as f64 * TILE;
        let z0 = self.min.1 + tz as f64 * TILE;
        let (w, d) = ((self.max.0 - x0).min(TILE), (self.max.1 - z0).min(TILE));
        let mut rng = Pcg32::new(hash(self.seed ^ hash(((tx as u64) << 32) | tz as u64)), 0x5eed);
        let expected = self.clump_density * w * d;
        let count = expected.floor() as usize + usize::from(rng.next_f64() < expected.fract());
        let mut blades = vec![];
        for _ in 0 .. count {
            let (x, z) = (x0 + rng.next_f64() * w, z0 + rng.next_f64() * d);
            let y = self.grid.height(x, z);
            let normal = self.grid.normal(x, z);
            let (patch, grows) = self.vigour(x, z, y, &normal);
            let keep = patch * grows;
            if rng.next_f64() >= keep { continue }
            let si = self.pick_species(x, z, &mut rng);
            let sp = &self.species[si];
            let centre = Vector3::new(x, y, z);
            for _ in 0 .. sp.clump_size {
                // Uniform in the tuft's disc.
                let (rr, a) = (rng.next_f64().sqrt() * sp.clump_radius, rng.next_f64() * 2. * PI);
                let (bx, bz) = (x + rr * a.cos(), z + rr * a.sin());
                if !self.grid.contains(bx, bz) { continue }
                blades.push(self.blade(si, centre, bx, bz, patch, &mut rng));
            }
        }
        blades
    }

    fn blade(&self, si: usize, centre: Vector3<f64>, x: f64, z: f64, patch: f64, rng: &mut Pcg32) -> Blade {
        let sp = &self.species[si];
        let c = self.conditions;
        let y_axis = Vector3::new(0., 1., 0.);
        let ground = self.grid.height(x, z);
        let up = (y_axis * (1. - sp.normal_alignment) + self.grid.normal(x, z) * sp.normal_alignment).normalize();

        let dead = rng.next_f64() < c.dead_fraction;
        let dryness = if dead { 1. } else {
            (sp.dryness + c.dryness + (1. - patch) * 0.5 * c.patchiness + (rng.next_f64() - 0.5) * 0.25).clamp(0., 1.)
        };

        let mut height = sp.height * (1. + sp.height_variation * (2. * rng.next_f64() - 1.))
            * (0.55 + 0.45 * patch) * (1. - 0.35 * c.trampled);
        let mut cut = false;
        if let Some(m) = c.mow_height && height > m {
            height = m * (1. + 0.1 * (rng.next_f64() - 0.5));
            cut = true;
        }

        // Which way the blade leans: random, fanned out from the tuft's centre, and along mowing
        // stripes.
        let a = rng.next_f64() * 2. * PI;
        let mut facing = Vector3::new(a.cos(), 0., a.sin());
        let out = Vector3::new(x - centre.x, 0., z - centre.z);
        let out_frac = if sp.clump_radius > 0. { (out.norm() / sp.clump_radius).min(1.) } else { 0. };
        if out.norm() > 1e-9 {
            facing = facing * (1. - sp.clump_spread) + out.normalize() * sp.clump_spread;
        }
        let mut lean = rng.next_f64() * sp.lean + sp.clump_spread * 0.6 * out_frac + c.trampled * rng.next_f64() * 1.2;
        if c.stripe_width > 0. {
            let along = c.stripe_direction;
            let across = Vector3::new(-along.z, 0., along.x);
            let band = ((x * across.x + z * across.z) / c.stripe_width).floor() as i64;
            let sign = if band.rem_euclid(2) == 0 { 1. } else { -1. };
            facing = facing * 0.2 + along * sign;
            lean += 0.6;
        }
        if facing.norm() < 1e-9 { facing = Vector3::new(1., 0., 0.) }
        let facing = facing.normalize();
        let lean = lean.min(PI * 0.45);

        // Droop under its own weight; wind, varying with gusts.
        let droop = (1. - sp.stiffness) * 0.5 * if dead { 1.6 } else { 1. } * (1. - 0.5 * f64::from(cut));
        let gust = if c.wind.norm() > 0. {
            1. + c.gustiness * (2. * field_noise(&self.noise, x, z, c.gust_scale, self.seed as f64 * 0.1 + 3.) - 1.)
        } else { 0. };
        let wind = c.wind * (gust * (1. - 0.6 * sp.stiffness) * 1.2);

        // Tip pushed by lean, droop and wind, then the length restored (Jahrmann and Wimmer).
        // The pushes add as vectors, so wind against the lean straightens a blade; their sum
        // gives the angle of the tip from the vertical.
        let v0 = Vector3::new(x, ground, z) - up * (0.01 * height + 0.002);
        let push = facing * (lean + droop) + wind;
        let angle = push.norm().min(1.75);
        let towards = if push.norm() > 1e-9 { push.normalize() } else { facing };
        let mut v2 = v0 + (up * angle.cos() + towards * angle.sin()) * height;
        let above = (v2 - v0).dot(&up);
        if above < 0.02 * height { v2 += up * (0.02 * height - above) }
        let proj = ((v2 - v0) - up * (v2 - v0).dot(&up)).norm() / height;
        let mut v1 = v0 + up * height * (1. - proj).max(0.05 * proj.max(1.));
        let l0 = (v2 - v0).norm();
        let l1 = (v1 - v0).norm() + (v2 - v1).norm();
        let ratio = height / ((2. * l0 + l1) / 3.);
        v1 = v0 + (v1 - v0) * ratio;
        v2 = v1 + (v2 - v1) * ratio;
        // On a slope, a tip bent uphill could still end up below the ground beneath it.
        let tip_ground = self.grid.height(v2.x, v2.z) + 0.01 * height;
        if v2.y < tip_ground { v2.y = tip_ground }

        let side = up.cross(&facing);
        let side = if side.norm() < 1e-9 { Vector3::new(0., 0., 1.) } else { side.normalize() };
        let width = sp.width * (0.85 + 0.3 * rng.next_f64()) * if dead { 0.8 } else { 1. };
        let twist = sp.twist * (2. * rng.next_f64() - 1.);

        let pad = Vector3::repeat(width * 0.5 + 1e-5);
        let lo = v0.inf(&v1).inf(&v2) - pad;
        let hi = v0.sup(&v1).sup(&v2) + pad;
        let level = ((dryness * (DRY_LEVELS - 1) as f64).round() as usize).min(DRY_LEVELS - 1);
        let jitter = ((rng.next_f64() * JITTER_LEVELS as f64) as usize).min(JITTER_LEVELS - 1);
        Blade {
            v: [f3(v0), f3(v1), f3(v2)],
            side: f3(side),
            facing: f3(facing),
            width: width as f32,
            twist: twist as f32,
            lo: f3(lo), hi: f3(hi),
            species: si as u8,
            shade: (level * JITTER_LEVELS + jitter) as u8,
            cut,
        }
    }
}

/// The blades of a field, bucketed by cell.
pub struct GrassField {
    blades: Vec<Blade>,
    species: Vec<Species>,
    cells: Grid2,
    /// Blades overlapping each cell are `items[start[i] .. start[i + 1]]`.
    start: Vec<u32>,
    items: Vec<u32>,
    /// The heights each cell's blades span.
    ranges: Vec<(f32, f32)>,
    bounds: BBox,
}

impl GrassField {
    /// Grow a field of `density` blades per square metre over [min, max] of the heightmap.
    pub fn new(grid: &HeightGrid, species: Vec<Species>, conditions: &Conditions, density: f64, seed: u64, min: (f64, f64), max: (f64, f64)) -> GrassField {
        assert!(!species.is_empty(), "grass needs at least one species");
        assert!(species.len() <= 256, "grass can have at most 256 species");
        let total: f64 = species.iter().map(|s| s.weight).sum();
        assert!(total > 0., "grass species weights must not all be 0");
        let per_clump: f64 = species.iter().map(|s| s.weight * s.clump_size as f64).sum::<f64>() / total;
        let gmax = grid.max();
        let min = (min.0.max(grid.min.0), min.1.max(grid.min.1));
        let max = (max.0.min(gmax.0), max.1.min(gmax.1));
        let generator = Generator {
            grid, species: &species, conditions, noise: PerlinNoise::new(), seed,
            min, max, clump_density: density.max(0.) / per_clump,
        };
        let tiles_x = ((max.0 - min.0) / TILE).ceil().max(0.) as usize;
        let tiles_z = ((max.1 - min.1) / TILE).ceil().max(0.) as usize;
        let blades: Vec<Blade> = (0 .. tiles_x * tiles_z).into_par_iter()
            .map(|i| generator.tile(i % tiles_x, i / tiles_x))
            .collect::<Vec<_>>()
            .into_iter().flatten().collect();
        GrassField::bucket(blades, species, density)
    }

    fn bucket(blades: Vec<Blade>, species: Vec<Species>, density: f64) -> GrassField {
        let mut bounds = BBox::empty();
        let mut reach = 0.;
        for b in &blades {
            bounds = bounds.union(&BBox::new(v3(b.lo), v3(b.hi)));
            reach += ((b.hi[0] - b.lo[0]) + (b.hi[2] - b.lo[2])) as f64 * 0.5;
        }
        if blades.is_empty() {
            let cells = Grid2 { min: (0., 0.), cell: 1., nx: 1, nz: 1 };
            return GrassField { blades, species, cells, start: vec![0, 0], items: vec![], ranges: vec![(0., 0.)], bounds };
        }
        reach /= blades.len() as f64;
        let size = bounds.size();
        // Cells small enough to hold a few blades, but not so small that every blade spans many.
        let mut cell = (BLADES_PER_CELL / density.max(1e-9)).sqrt().max(reach * 0.75);
        // At most about 4 million cells.
        cell = cell.max((size.x * size.z / 4e6).sqrt()).max(1e-4);
        let nx = ((size.x / cell).ceil() as usize).max(1);
        let nz = ((size.z / cell).ceil() as usize).max(1);
        let cells = Grid2 { min: (bounds.min.x, bounds.min.z), cell, nx, nz };

        let span = |b: &Blade| {
            let f = |v: f32, lo: f64, n: usize| (((v as f64 - lo) / cell).floor().max(0.) as usize).min(n - 1);
            (f(b.lo[0], cells.min.0, nx), f(b.hi[0], cells.min.0, nx), f(b.lo[2], cells.min.1, nz), f(b.hi[2], cells.min.1, nz))
        };
        let mut counts = vec![0u32; nx * nz];
        for b in &blades {
            let (x0, x1, z0, z1) = span(b);
            for iz in z0 ..= z1 { for ix in x0 ..= x1 { counts[iz * nx + ix] += 1 } }
        }
        let mut start = Vec::with_capacity(nx * nz + 1);
        let mut acc = 0u32;
        start.push(0);
        for c in &counts { acc += c; start.push(acc) }
        let mut fill: Vec<u32> = start[.. nx * nz].to_vec();
        let mut items = vec![0u32; acc as usize];
        let mut ranges = vec![(f32::INFINITY, f32::NEG_INFINITY); nx * nz];
        for (i, b) in blades.iter().enumerate() {
            let (x0, x1, z0, z1) = span(b);
            for iz in z0 ..= z1 {
                for ix in x0 ..= x1 {
                    let c = iz * nx + ix;
                    items[fill[c] as usize] = i as u32;
                    fill[c] += 1;
                    ranges[c] = (ranges[c].0.min(b.lo[1]), ranges[c].1.max(b.hi[1]));
                }
            }
        }
        GrassField { blades, species, cells, start, items, ranges, bounds }
    }

    pub fn len(&self) -> usize {
        self.blades.len()
    }

    fn cell_items(&self, c: usize) -> &[u32] {
        &self.items[self.start[c] as usize .. self.start[c + 1] as usize]
    }

    /// Nearest hit: t (multiples of the ray direction), the blade, and s along it.
    fn nearest(&self, r: &Ray) -> Option<(f64, usize, f64)> {
        if self.blades.is_empty() { return None }
        let invrd = Vector3::new(1. / r.rd.x, 1. / r.rd.y, 1. / r.rd.z);
        let mut best: Option<(f64, usize, f64)> = None;
        walk_grid(&self.cells, self.bounds.min.y, self.bounds.max.y, r, |ix, iz, t0, t1| {
            let c = iz * self.cells.nx + ix;
            let (lo, hi) = self.ranges[c];
            if !ray_spans(r, t0, t1, lo as f64, hi as f64) {
                return false;
            }
            // Allow for rounding at cell boundaries: a hit just outside this cell is also in
            // the next, so accepting it here can't skip a nearer one.
            let eps = 1e-9 * (1. + t1.abs());
            for &i in self.cell_items(c) {
                let b = &self.blades[i as usize];
                let limit = best.map_or(t1 + eps, |x| x.0);
                if !b.bounds_hit(r, &invrd, t0 - eps, limit) { continue }
                if let Some((t, s)) = b.intersect(&self.species[b.species as usize], r, t0 - eps, limit) {
                    best = Some((t, i as usize, s));
                }
            }
            best.is_some()
        });
        best
    }

    /// The blade `p` is on, and how far along it.
    fn locate(&self, p: Vector3<f64>) -> Option<(usize, f64)> {
        let g = &self.cells;
        let ix = (p.x - g.min.0) / g.cell;
        let iz = (p.z - g.min.1) / g.cell;
        if ix < 0. || iz < 0. || ix >= g.nx as f64 || iz >= g.nz as f64 { return None }
        let c = iz as usize * g.nx + ix as usize;
        let tolerance = 1e-3;
        let mut best: Option<(f64, usize, f64)> = None;
        for &i in self.cell_items(c) {
            let b = &self.blades[i as usize];
            let inside = (0 .. 3).all(|k| p[k] >= b.lo[k] as f64 - tolerance && p[k] <= b.hi[k] as f64 + tolerance);
            if !inside { continue }
            if let Some((d, s)) = b.locate(&self.species[b.species as usize], p, tolerance)
                && best.is_none_or(|x| d < x.0) {
                best = Some((d, i as usize, s));
            }
        }
        best.map(|(_, i, s)| (i, s))
    }
}

pub struct GrassGeometry {
    pub field: Arc<GrassField>,
}

impl Geometry for GrassGeometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let (t, i, s) = self.field.nearest(r)?;
        Some(RawIntersection { part: 0,
            dist: t * r.rd.norm(),
            point: r.ro + r.rd * t,
            normal: self.field.blades[i].normal_at(s),
        })
    }

    fn bounds(&self) -> BBox {
        self.field.bounds
    }

    fn primitives(&self) -> u64 {
        self.field.len() as u64
    }
}

/// Materials for every species, dryness, per-blade variation and position along the blade.
pub struct GrassMedium {
    pub field: Arc<GrassField>,
    materials: Vec<GrassMaterial>,
}

fn palette_index(species: usize, shade: usize, s: f64) -> usize {
    let h = ((s * HEIGHT_LEVELS as f64) as usize).min(HEIGHT_LEVELS - 1);
    (species * DRY_LEVELS * JITTER_LEVELS + shade) * HEIGHT_LEVELS + h
}

fn mix(a: Color, b: Color, t: f64) -> Color {
    Color { rgb: a.rgb * (1. - t) + b.rgb * t }
}

impl GrassMedium {
    pub fn new(field: Arc<GrassField>, conditions: &Conditions) -> GrassMedium {
        let mut materials = Vec::with_capacity(field.species.len() * DRY_LEVELS * JITTER_LEVELS * HEIGHT_LEVELS);
        let ice = Color::new(0.75, 0.8, 0.85);
        for sp in &field.species {
            for level in 0 .. DRY_LEVELS {
                let dry = level as f64 / (DRY_LEVELS - 1) as f64;
                for j in 0 .. JITTER_LEVELS {
                    let jitter = j as f64 / (JITTER_LEVELS - 1) as f64 - 0.5;
                    for h in 0 .. HEIGHT_LEVELS {
                        let s = (h as f64 + 0.5) / HEIGHT_LEVELS as f64;
                        let green = mix(sp.base_color, sp.tip_color, s.powf(0.8));
                        let straw = sp.dry_color * (0.8 + 0.35 * s);
                        let mut c = mix(green, straw, dry);
                        // Per-blade variation: brightness, and a slight shift toward yellow.
                        c = Color::new(c.rgb.x * (1. + 0.45 * jitter), c.rgb.y * (1. + 0.3 * jitter), c.rgb.z * (1. + 0.1 * jitter));
                        c = mix(c, ice, conditions.frost * s.powf(1.5));
                        c = c * (1. - 0.25 * conditions.wetness);
                        let translucency = sp.translucency * (1. - 0.6 * dry) * (1. - 0.7 * conditions.frost * s);
                        let reflect = c * (1. - 0.35 * translucency);
                        // Light through a leaf is more saturated, and yellower.
                        let transmit = Color::new(c.rgb.x * 1.1, c.rgb.y * 1.15, c.rgb.z * 0.6) * translucency;
                        materials.push(GrassMaterial {
                            reflect,
                            transmit,
                            sheen: (sp.sheen * (1. - 0.5 * dry) * (1. - conditions.frost) + 0.7 * conditions.wetness).min(1.),
                            roughness: 0.3 - 0.2 * conditions.wetness,
                        });
                    }
                }
            }
        }
        GrassMedium { field, materials }
    }
}

impl Medium for GrassMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        let i = match self.field.locate(pt) {
            Some((b, s)) => {
                let blade = &self.field.blades[b];
                palette_index(blade.species as usize, blade.shade as usize, s)
            },
            // Shouldn't happen: a mid-height, mid-dryness blade of the first species.
            None => palette_index(0, (DRY_LEVELS / 4) * JITTER_LEVELS + 1, 0.5),
        };
        &self.materials[i]
    }
}

/// Parse a grass object (see the module documentation) into its geometry, and the conditions
/// for its medium.
pub fn create_grass(o: &Value) -> (GrassGeometry, Conditions) {
    let grid = HeightGrid::from_json(&o["heightmap"]);
    let species: Vec<Species> = match &o["species"] {
        Value::Array(list) => list.iter().map(Species::from_json).collect(),
        Value::Null => vec![Species::from_json(&Value::Null)],
        v => vec![Species::from_json(v)],
    };
    let conditions = Conditions::from_json(&o["conditions"]);
    let corner = |k: &str, def: (f64, f64)| match o.get(k) {
        Some(v) => (v[0].as_f64().unwrap(), v[1].as_f64().unwrap()),
        None => def,
    };
    let field = GrassField::new(
        &grid, species, &conditions,
        SceneFile::parse_number(&o["density"], 600.),
        SceneFile::parse_int(&o["seed"], 0) as u64,
        corner("min", grid.min), corner("max", grid.max()));
    (GrassGeometry { field: Arc::new(field) }, conditions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedural::heightmap::{Flat, HeightSource};

    struct Hill;
    impl HeightSource for Hill {
        fn height(&self, x: f64, z: f64) -> f64 { 2. - 0.02 * (x * x + z * z) }
    }

    fn field(conditions: Value, species: Value, density: f64) -> (HeightGrid, GrassField) {
        let grid = HeightGrid::sample(&Hill, (-5., -5.), (5., 5.), 0.25);
        let species: Vec<Species> = species.as_array().unwrap().iter().map(Species::from_json).collect();
        let c = Conditions::from_json(&conditions);
        let f = GrassField::new(&grid, species, &c, density, 7, (-4., -4.), (4., 4.));
        (grid, f)
    }

    #[test]
    fn test_field_is_reproducible_and_has_expected_density() {
        let (_, a) = field(json!("lush"), json!(["lawn"]), 200.);
        let (_, b) = field(json!("lush"), json!(["lawn"]), 200.);
        assert_eq!(a.len(), b.len());
        assert_eq!(a.blades[123].v, b.blades[123].v);
        // 64 m^2 at 200 per m^2, with lush conditions thinning nothing.
        let expected = 64. * 200.;
        assert!((a.len() as f64 - expected).abs() < expected * 0.05, "{} blades", a.len());
    }

    #[test]
    fn test_blades_keep_their_length_and_grow_from_the_ground() {
        let (grid, f) = field(json!({ "preset": "windy" }), json!(["meadow", "tall"]), 50.);
        for b in f.blades.iter().step_by(37) {
            let sp = &f.species[b.species as usize];
            let root = v3(b.v[0]);
            assert!((root.y - grid.height(root.x, root.z)) < 0.0, "root above ground: {}", root);
            // Arc length of the Bézier, by summing chords.
            let len: f64 = (0 .. 64).map(|i| (b.point((i + 1) as f64 / 64.) - b.point(i as f64 / 64.)).norm()).sum();
            let max_height = sp.height * (1. + sp.height_variation);
            assert!(len > 0. && len < max_height * 1.05, "length {} for species height {}", len, sp.height);
            // The tip stays above the ground.
            let tip = b.point(1.);
            assert!(tip.y > grid.height(tip.x, tip.z) - 0.01 * len - 0.003, "tip {} ground {} root {} mid {}", tip, grid.height(tip.x, tip.z), root, v3(b.v[1]));
        }
    }

    #[test]
    fn test_wind_bends_blades_downwind() {
        let calm = field(json!({ "wind": { "strength": 0 } }), json!([{ "preset": "meadow", "lean": 0, "clump_size": 1 }]), 30.).1;
        let windy = field(json!({ "wind": { "strength": 1, "direction": [1, 0], "gustiness": 0 } }),
            json!([{ "preset": "meadow", "lean": 0, "clump_size": 1 }]), 30.).1;
        let mean_dx = |f: &GrassField| f.blades.iter().map(|b| (b.v[2][0] - b.v[0][0]) as f64).sum::<f64>() / f.len() as f64;
        assert!(mean_dx(&calm).abs() < 0.02, "{}", mean_dx(&calm));
        assert!(mean_dx(&windy) > 0.1, "{}", mean_dx(&windy));
    }

    #[test]
    fn test_mowing_cuts_blades() {
        let (_, f) = field(json!({ "mow_height": 0.05 }), json!(["meadow"]), 30.);
        assert!(f.blades.iter().all(|b| b.cut));
        assert!(f.blades.iter().all(|b| b.point(1.).y - b.point(0.).y < 0.06));
    }

    #[test]
    fn test_altitude_and_slope_limits() {
        let (grid, f) = field(json!({ "max_altitude": 1.5, "altitude_fade": 0.01 }), json!(["lawn"]), 50.);
        assert!(f.len() > 0);
        for b in &f.blades {
            let r = v3(b.v[0]);
            assert!(grid.height(r.x, r.z) < 1.52);
        }
        let (_, none) = field(json!({ "max_slope": 0 }), json!(["lawn"]), 50.);
        assert_eq!(none.len(), 0);
    }

    #[test]
    fn test_ray_hits_blade_and_medium_finds_it() {
        let grid = HeightGrid::sample(&Flat { base: 0. }, (-2., -2.), (2., 2.), 0.5);
        let sp = vec![Species::from_json(&json!({ "preset": "tall", "clump_size": 1 }))];
        let c = Conditions::from_json(&json!("lush"));
        let f = Arc::new(GrassField::new(&grid, sp, &c, 400., 1, (-1., -1.), (1., 1.)));
        let geometry = GrassGeometry { field: f.clone() };
        let medium = GrassMedium::new(f.clone(), &c);
        let mut hits = 0;
        for i in 0 .. 200 {
            let a = i as f64 * 0.1;
            let r = Ray { ro: Vector3::new(-3., 0.4, 0.3 * a.sin()), rd: Vector3::new(1., -0.05, 0.1 * a.cos()) };
            if let Some(h) = geometry.intersects(&r) {
                hits += 1;
                assert!((h.dist - (h.point - r.ro).norm()).abs() < 1e-9);
                let (b, s) = f.locate(h.point).expect("hit point should be on a blade");
                assert!((0. ..= 1.).contains(&s));
                // No blade is hit nearer than the reported one.
                let nearer = f.blades.iter().enumerate().filter(|(j, _)| *j != b).any(|(_, other)| {
                    other.intersect(&f.species[0], &r, 0., h.dist / r.rd.norm() - 1e-9).is_some()
                });
                assert!(!nearer, "missed a nearer blade on ray {}", i);
                let _ = medium.material_at(h.point);
            }
        }
        assert!(hits > 150, "only {} of 200 rays through dense grass hit", hits);
    }

    #[test]
    fn test_presets_parse() {
        for p in ["lawn", "meadow", "fescue", "tall", "tussock", "savanna", "reed"] {
            Species::from_json(&json!(p));
        }
        for p in ["lush", "summer", "drought", "autumn", "after_rain", "frost", "windy", "manicured", "trampled"] {
            Conditions::from_json(&json!(p));
        }
        let s = Species::from_json(&json!({ "preset": "reed", "height": 3 }));
        assert_eq!(s.height, 3.);
        let c = Conditions::from_json(&json!({ "preset": "windy", "wind": { "direction": [0, 1] } }));
        assert!((c.wind.z - 0.8).abs() < 1e-12, "{}", c.wind);
    }
}
