//! # Terrain as a scene object
//!
//! Builds a terrain from a scene file's `"type": "landscape"` object (`"terrain"` is the
//! heightmap terrain of `procedural::heightmap`), runs it to the requested
//! time, and turns it into a smooth triangle mesh whose material varies with what is at each
//! point (ice, river, sea floor, bare rock on steep slopes, otherwise a tint by elevation).
//!
//! ```json
//! {
//!   "type": "landscape",
//!   "seed": 0,
//!   "time": 100000,
//!   "domain": { "type": "patch", "cells": 128, "spacing": 400 },
//!   "scale": 1000,
//!   "exaggeration": 1.5
//! }
//! ```
//!
//! - `time`: years of landscape evolution after the noise topography (default 0).
//! - `domain`: `{"type": "patch", "cells", "spacing" (m), "lat", "lon"}`, where a patch without
//!   `lat`/`lon` is placed on this seed's most mountainous spot (or, with `"near":
//!   "volcanoes"`, its most volcanic mountains, or with `"near": "coast"`, a coast with
//!   mountains behind it); or `{"type": "planet",
//!   "cells"}`, a cubed sphere with `cells` x `cells` per face.
//! - Ground cover: ice, rivers, sea floor, bare rock on steep slopes, alluvium (sediment over
//!   2 m deep), lava, then a tint by elevation.
//! - `scale`: metres per scene unit (default 1000). `exaggeration`: vertical exaggeration.
//! - A patch is drawn in a local frame: x east, y up, z south, sea level at y = 0, centred on
//!   the origin, ignoring the planet's curvature. A planet is drawn centred on `centre`
//!   (default the origin), its sea level a sphere of radius `radius / scale`.
//! - Processes: `radius` (m), `step` (years), `land_bias`, `mountain_height` (m),
//!   `uplift_rate` (m/yr), `belt_frequency`; `fluvial`: `{k, m, n, diffusivity, deposition}`;
//!   `glacier` and `volcanoes`: `false` to disable, or overrides of `GlacierParams` and
//!   `VolcanoParams` by field name. Geology: `strata_thickness` (m), `strata_contrast`,
//!   `fold_amplitude` (m), `sediment_erodibility`, `lava_erodibility`.
//! - `min_river_width`: draw rivers at least this wide, metres (default 0: their modelled
//!   widths, which may be thinner than a pixel from afar). On patches, rivers, meandering
//!   channels and ox-bow lakes are drawn as lines at their widths (`channel`).
//! - `channels`: `false` for no meandering, or overrides of `ChannelParams`; `marine`: `false`
//!   for no deltas, or `{diffusivity, depth_scale}`.
//! - `map`: also write a shaded relief map to this PNG path.
//!
//! Water is not part of the terrain: add a plane at y = 0 (patch) or a sphere (planet).

use crate::color::Color;
use crate::geometry::rand;
use crate::material::lambertian::Lambertian;
use crate::material::model::MaterialModel;
use crate::material::texture::Medium;
use crate::na::Vector3;
use crate::bvh::GeometryBvh;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::sceneobject::SceneObject;
use crate::scenefile::SceneFile;
use crate::shapes::bbox::BBox;
use crate::shapes::geometry::Geometry;
use crate::shapes::mesh::vertex_normals;
use crate::shapes::triangle::{SmoothTriangle, Triangle};
use crate::terrain::domain::{Domain, Layout};
use crate::terrain::glacier::{slope_squared, GlacierParams};
use crate::terrain::map::{self, is_river, land_colour, write_map};
use crate::terrain::planet::Planet;
use crate::terrain::simulation::{Terrain, TerrainParams, ICE_FREE};
use crate::terrain::volcano::VolcanoParams;
use crate::terrain::channel::{point_cell, WaterLines};
use serde_json::Value;
use std::sync::Arc;

/// How the terrain's cells map into the scene.
#[derive(Clone)]
pub struct Placement {
    pub scale: f64,
    pub exaggeration: f64,
    pub centre: Vector3<f64>,
}

impl Placement {
    /// Scene position of direction `d` at height `h` metres.
    pub fn to_scene(&self, domain: &Domain, d: &Vector3<f64>, h: f64) -> Vector3<f64> {
        match &domain.layout {
            Layout::Patch { up, east, north, .. } => {
                let (x, z) = Domain::tangent(*up, *east, *north, domain.radius, d);
                Vector3::new(x, h * self.exaggeration, -z) / self.scale + self.centre
            }
            Layout::Sphere { .. } => self.centre + d * (domain.radius + h * self.exaggeration) / self.scale,
        }
    }

    /// The cell under scene position `p`.
    pub fn cell_at(&self, domain: &Domain, p: &Vector3<f64>) -> usize {
        let q = (p - self.centre) * self.scale;
        match &domain.layout {
            Layout::Patch { up, east, north, .. } => domain.cell_at(&(up * domain.radius + east * q.x - north * q.z)),
            Layout::Sphere { .. } => domain.cell_at(&q),
        }
    }
}

pub struct TerrainGeometry {
    triangles: GeometryBvh<SmoothTriangle>,
    bounds: BBox,
    count: usize,
}

impl Geometry for TerrainGeometry {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.triangles.nearest(r, 0., f64::INFINITY).map(|x| x.1)
    }
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.triangles.nearest(r, min, max).map(|x| x.1)
    }
    fn bounds(&self) -> BBox { self.bounds }
    fn primitives(&self) -> u64 { self.count as u64 }
}

/// A smooth mesh through the cells' corners, each at the mean surface height of the cells
/// meeting there.
pub fn mesh(t: &Terrain, place: &Placement) -> TerrainGeometry {
    let d = &t.domain;
    let surface = t.surface();
    let positions: Vec<Vector3<f64>> = d.corners.iter().zip(&d.corner_cells).map(|(dir, cells)| {
        let h = cells.iter().map(|&c| surface[c]).sum::<f64>() / cells.len() as f64;
        place.to_scene(d, dir, h)
    }).collect();
    let triangles: Vec<[usize; 3]> = d.quads.iter().flat_map(|q| [[q[0], q[1], q[2]], [q[0], q[2], q[3]]]).collect();
    let normals = vertex_normals(&positions, &triangles);
    let items: Vec<SmoothTriangle> = triangles.iter().map(|&[a, b, c]| {
        let face = Triangle::new(positions[a], positions[b], positions[c]).normal;
        let n = |i: usize| if normals[i] == Vector3::zeros() { face } else { normals[i] };
        SmoothTriangle::new(positions[a], positions[b], positions[c], n(a), n(b), n(c))
    }).collect();
    let bounds = items.iter().fold(BBox::empty(), |b, t| b.union(&t.bounds()));
    let count = items.len();
    TerrainGeometry { triangles: GeometryBvh::new(items), bounds, count }
}

/// Materials by what covers the ground, chosen per cell.
pub struct TerrainMedium {
    domain: Domain,
    place: Placement,
    /// Cell spacing in scene units.
    jitter: f64,
    class: Vec<u8>,
    materials: Vec<Box<dyn MaterialModel + Sync + Send>>,
    /// Rivers and lakes drawn as lines (patches).
    water: Option<WaterLines>,
}

const ICE: u8 = 0;
const RIVER: u8 = 1;
const SEA_FLOOR: u8 = 2;
const ROCK: u8 = 3;
const LAVA: u8 = 4;
const ALLUVIUM: u8 = 5;
const BAND0: u8 = 6;
const BAND_HEIGHT: f64 = 250.;
const BANDS: usize = 20;
/// Slope (rise over run) above which no soil holds and bare rock shows.
const ROCK_SLOPE: f64 = 0.8;

impl TerrainMedium {
    /// `min_river_width`: draw rivers at least this wide, metres (thin ones vanish from afar).
    pub fn new(t: Terrain, place: Placement, min_river_width: f64) -> TerrainMedium {
        let slope2 = slope_squared(&t.domain, &t.bed);
        let water = map::water_lines(&t, min_river_width);
        let lines = water.is_some();
        let class = (0..t.domain.len()).map(|i| {
            if t.ice[i] >= ICE_FREE { ICE }
            else if t.bed[i] < 0. { SEA_FLOOR }
            else if is_river(&t, i) && !lines { RIVER }
            else if slope2[i] > ROCK_SLOPE * ROCK_SLOPE { ROCK }
            else if t.sediment[i] > map::ALLUVIUM { ALLUVIUM }
            else if t.lava[i] > 1. { LAVA }
            else { BAND0 + ((t.bed[i] / BAND_HEIGHT) as usize).min(BANDS - 1) as u8 }
        }).collect();
        let lambertian = |c: [f64; 3]| -> Box<dyn MaterialModel + Sync + Send> {
            Box::new(Lambertian { albedo: Color::new(c[0], c[1], c[2]) })
        };
        let mut materials = vec![
            lambertian([0.85, 0.9, 0.95]),
            lambertian([0.12, 0.2, 0.3]),
            lambertian([0.35, 0.33, 0.28]),
            lambertian([0.38, 0.36, 0.34]),
            lambertian(map::LAVA_COLOUR),
            lambertian(map::ALLUVIUM_COLOUR),
        ];
        for b in 0..BANDS { materials.push(lambertian(land_colour((b as f64 + 0.5) * BAND_HEIGHT))); }
        let jitter = t.domain.min_spacing() / place.scale;
        TerrainMedium { domain: t.domain, place, jitter, class, materials, water }
    }
}

impl Medium for TerrainMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        // Look up a random point within half a cell, so that, over many samples, materials
        // blend across cell boundaries rather than showing the grid's staircase.
        if let Some(water) = &self.water {
            let q = (pt - self.place.centre) * self.place.scale;
            let q = [q.x, -q.z];
            if water.wet(point_cell(&self.domain, q), q) { return &*self.materials[RIVER as usize]; }
        }
        let j = Vector3::new(rand() - 0.5, rand() - 0.5, rand() - 0.5) * self.jitter;
        &*self.materials[self.class[self.place.cell_at(&self.domain, &(pt + j))] as usize]
    }
}

fn number(o: &Value, k: &str, def: f64) -> f64 { SceneFile::parse_number(&o[k], def) }

/// Override fields of `GlacierParams` named in `o`.
fn glacier_params(o: &Value) -> GlacierParams {
    let mut g = GlacierParams::default();
    let fields: [(&str, &mut f64); 16] = [
        ("glen_a", &mut g.glen_a), ("glen_n", &mut g.glen_n), ("sliding", &mut g.sliding),
        ("erosion", &mut g.erosion), ("erosion_exponent", &mut g.erosion_exponent),
        ("ela_equator", &mut g.ela_equator), ("ela_pole", &mut g.ela_pole),
        ("ela_amplitude", &mut g.ela_amplitude), ("period", &mut g.period),
        ("balance_gradient", &mut g.balance_gradient), ("max_accumulation", &mut g.max_accumulation),
        ("max_step", &mut g.max_step), ("min_step", &mut g.min_step), ("max_speed", &mut g.max_speed), ("max_stiffness", &mut g.max_stiffness),
        ("acceleration", &mut g.acceleration),
    ];
    for (k, v) in fields {
        if let Some(x) = o.get(k).and_then(|x| x.as_f64()) { *v = x; }
    }
    if let Some(x) = o.get("picard").and_then(|x| x.as_u64()) { g.picard = x.max(1) as usize; }
    g
}

pub fn terrain_params(o: &Value) -> TerrainParams {
    let mut p = TerrainParams::default();
    p.planet.seed = SceneFile::parse_int(&o["seed"], 0) as u64;
    p.planet.radius = number(o, "radius", p.planet.radius);
    p.planet.land_bias = number(o, "land_bias", p.planet.land_bias);
    p.planet.mountain_height = number(o, "mountain_height", p.planet.mountain_height);
    p.planet.uplift_rate = number(o, "uplift_rate", p.planet.uplift_rate);
    p.planet.belt_frequency = number(o, "belt_frequency", p.planet.belt_frequency);
    p.step = number(o, "step", p.step);
    let f = &o["fluvial"];
    p.fluvial.k = number(f, "k", p.fluvial.k);
    p.fluvial.m = number(f, "m", p.fluvial.m);
    p.fluvial.n = number(f, "n", p.fluvial.n);
    p.fluvial.diffusivity = number(f, "diffusivity", p.fluvial.diffusivity);
    p.fluvial.deposition = number(f, "deposition", p.fluvial.deposition);
    p.planet.strata_thickness = number(o, "strata_thickness", p.planet.strata_thickness);
    p.planet.strata_contrast = number(o, "strata_contrast", p.planet.strata_contrast);
    p.planet.fold_amplitude = number(o, "fold_amplitude", p.planet.fold_amplitude);
    p.sediment_erodibility = number(o, "sediment_erodibility", p.sediment_erodibility);
    p.lava_erodibility = number(o, "lava_erodibility", p.lava_erodibility);
    p.volcano = match &o["volcanoes"] {
        Value::Bool(false) => None,
        v => {
            let mut x = VolcanoParams::default();
            let fields: [(&str, &mut f64); 9] = [
                ("spacing", &mut x.spacing), ("density", &mut x.density), ("horizon", &mut x.horizon),
                ("duration", &mut x.duration), ("rate", &mut x.rate), ("cone_radius", &mut x.cone_radius),
                ("lava_fraction", &mut x.lava_fraction), ("flow_thickness", &mut x.flow_thickness),
                ("roughness", &mut x.roughness),
            ];
            for (k, f) in fields {
                if let Some(n) = v.get(k).and_then(|n| n.as_f64()) { *f = n; }
            }
            Some(x)
        }
    };
    p.channels = match &o["channels"] {
        Value::Bool(false) => None,
        v => {
            let mut x = crate::terrain::channel::ChannelParams::default();
            let fields: [(&str, &mut f64); 10] = [
                ("min_area", &mut x.min_area), ("max_slope", &mut x.max_slope), ("min_sediment", &mut x.min_sediment),
                ("width_coefficient", &mut x.width_coefficient), ("migration", &mut x.migration),
                ("friction", &mut x.friction), ("omega", &mut x.omega), ("gamma", &mut x.gamma),
                ("regrid", &mut x.regrid), ("oxbow_fill", &mut x.oxbow_fill),
            ];
            for (k, f) in fields {
                if let Some(n) = v.get(k).and_then(|n| n.as_f64()) { *f = n; }
            }
            Some(x)
        }
    };
    p.marine = match &o["marine"] {
        Value::Bool(false) => None,
        v => {
            let mut x = crate::terrain::marine::MarineParams::default();
            if let Some(n) = v.get("diffusivity").and_then(|n| n.as_f64()) { x.diffusivity = n; }
            if let Some(n) = v.get("depth_scale").and_then(|n| n.as_f64()) { x.depth_scale = n; }
            Some(x)
        }
    };
    p.glacier = match &o["glacier"] {
        Value::Bool(false) => None,
        Value::Object(_) => Some(glacier_params(&o["glacier"])),
        _ => Some(GlacierParams::default()),
    };
    p
}

pub fn domain(o: &Value, params: &TerrainParams) -> Domain {
    let d = &o["domain"];
    let radius = params.planet.radius;
    match d["type"].as_str().unwrap_or("patch") {
        "planet" => Domain::sphere(SceneFile::parse_int(&d["cells"], 64), radius),
        "patch" => {
            let planet = || Planet::new(params.planet.clone());
            let (lat, lon) = match (d["lat"].as_f64(), d["lon"].as_f64(), d["near"].as_str()) {
                (Some(lat), Some(lon), _) => (lat, lon),
                (_, _, Some("volcanoes")) => planet().most_volcanic(20000),
                (_, _, Some("coast")) => planet().most_coastal(20000),
                (_, _, None | Some("mountains")) => planet().most_mountainous(20000),
                (_, _, Some(x)) => panic!("Unknown terrain domain \"near\": {}", x),
            };
            Domain::patch(lat, lon, SceneFile::parse_int(&d["cells"], 128), number(d, "spacing", 400.), radius)
        }
        t => panic!("Unknown terrain domain type: {}", t),
    }
}

pub fn create_terrain(o: &Value) -> SceneObject {
    let params = terrain_params(o);
    let mut t = Terrain::new(domain(o, &params), params);
    let time = number(o, "time", 0.);
    let start = std::time::Instant::now();
    t.run_to(time);
    println!("Terrain: {} cells evolved for {} years in {:.1?}", t.domain.len(), time, start.elapsed());
    if let Some(path) = o["map"].as_str() { write_map(&t, path, 2048); }
    let place = Placement {
        scale: number(o, "scale", 1000.),
        exaggeration: number(o, "exaggeration", 1.),
        centre: SceneFile::parse_vec3_def(o, "centre", Vector3::zeros()),
    };
    let geometry = mesh(&t, &place);
    SceneObject { geometry: Box::new(geometry), medium: Box::new(TerrainMedium::new(t, place, number(o, "min_river_width", 0.))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placement_round_trips() {
        let place = Placement { scale: 1000., exaggeration: 2., centre: Vector3::new(1., 2., 3.) };
        for d in [Domain::patch(40., 10., 16, 500., 6.371e6), Domain::sphere(8, 6.371e6)] {
            for (i, dir) in d.dirs.iter().enumerate() {
                let p = place.to_scene(&d, dir, 1234.);
                assert_eq!(place.cell_at(&d, &p), i);
            }
        }
    }

    #[test]
    fn test_patch_is_drawn_with_north_away_from_the_viewer() {
        // A viewer on the +z side looking along -z sees north ahead and east to the right.
        let place = Placement { scale: 1., exaggeration: 1., centre: Vector3::zeros() };
        let d = Domain::patch(0., 0., 4, 100., 6.371e6);
        let (east_cell, north_cell) = (1 * 4 + 3, 3 * 4 + 1);
        assert!(place.to_scene(&d, &d.dirs[east_cell], 0.).x > 0.);
        assert!(place.to_scene(&d, &d.dirs[north_cell], 0.).z < 0.);
    }

    #[test]
    fn test_scene_object() {
        let o: Value = serde_json::from_str(r#"{
            "type": "landscape", "seed": 3, "time": 1000, "step": 500,
            "domain": { "type": "patch", "cells": 16, "spacing": 1000 },
            "glacier": { "acceleration": 20 }
        }"#).unwrap();
        let p = terrain_params(&o);
        assert_eq!(p.glacier.as_ref().unwrap().acceleration, 20.);
        let obj = create_terrain(&o);
        assert_eq!(obj.geometry.primitives(), 16 * 16 * 2);
        // A ray straight down onto the middle hits the terrain.
        let r = Ray { ro: Vector3::new(0.1, 100., 0.1), rd: Vector3::new(0., -1., 0.) };
        let hit = obj.geometry.intersects(&r).expect("no hit");
        assert!(hit.normal.y > 0.);
        let off: Value = serde_json::from_str(r#"{ "glacier": false }"#).unwrap();
        assert!(terrain_params(&off).glacier.is_none());
    }
}
