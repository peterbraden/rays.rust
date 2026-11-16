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
use crate::terrain::scenery::{self, Frame, Ground, LandscapeMedium, Layer, Relief, SceneryParams, Viewpoint};
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
    // Plate tectonics: on by default at full scale, `false` for noise alone, or overrides of
    // `TectonicsParams` by field name.
    p.planet.tectonics = match &o["tectonics"] {
        Value::Bool(false) => None,
        Value::Null if o.get("scale").is_some() => None,
        v => {
            let mut t = crate::terrain::tectonics::TectonicsParams::default();
            let fields: [(&str, &mut f64); 9] = [
                ("age", &mut t.age), ("step", &mut t.step), ("continental_fraction", &mut t.continental_fraction),
                ("speed", &mut t.speed), ("rift_interval", &mut t.rift_interval), ("subduction_uplift", &mut t.subduction_uplift),
                ("collision_uplift", &mut t.collision_uplift), ("relaxation", &mut t.relaxation), ("remesh", &mut t.remesh),
            ];
            for (k, f) in fields {
                if let Some(x) = v.get(k).and_then(|x| x.as_f64()) { *f = x; }
            }
            for (k, f) in [("cells", &mut t.cells), ("plates", &mut t.plates), ("hotspots", &mut t.hotspots)] {
                if let Some(x) = v.get(k).and_then(|x| x.as_u64()) { *f = x as usize; }
            }
            Some(t)
        }
    };
    // The climate, from latitude: on by default at full scale, `false` for none, or overrides
    // of `ClimateParams` by field name.
    p.climate = match &o["climate"] {
        Value::Bool(false) => None,
        Value::Null if o.get("scale").is_some() => None,
        v => {
            let mut c = crate::terrain::climate::ClimateParams::default();
            let fields: [(&str, &mut f64); 6] = [
                ("tilt", &mut c.tilt), ("solar", &mut c.solar), ("hadley", &mut c.hadley),
                ("recycling", &mut c.recycling), ("orographic", &mut c.orographic), ("warming", &mut c.warming),
            ];
            for (k, f) in fields {
                if let Some(x) = v.get(k).and_then(|x| x.as_f64()) { *f = x; }
            }
            if let Some(x) = v.get("cells").and_then(|x| x.as_u64()) { c.cells = x as usize; }
            Some(c)
        }
    };
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

/// Where a patch lies: its centre's (latitude, longitude) in degrees.
fn site(d: &Value, params: &TerrainParams) -> (f64, f64) {
    let planet = || Planet::new(params.planet.clone());
    match (d["lat"].as_f64(), d["lon"].as_f64(), d["near"].as_str()) {
        (Some(lat), Some(lon), _) => (lat, lon),
        (_, _, Some("volcanoes")) => planet().most_volcanic(20000),
        (_, _, Some("coast")) => planet().most_coastal(20000),
        (_, _, None | Some("mountains")) => planet().most_mountainous(20000),
        (_, _, Some(x)) => panic!("Unknown terrain domain \"near\": {}", x),
    }
}

pub fn domain(o: &Value, params: &TerrainParams) -> Domain {
    let d = &o["domain"];
    let radius = params.planet.radius;
    match d["type"].as_str().unwrap_or("patch") {
        "planet" => Domain::sphere(SceneFile::parse_int(&d["cells"], 64), radius),
        "patch" => {
            let (lat, lon) = site(d, params);
            Domain::patch(lat, lon, SceneFile::parse_int(&d["cells"], 128), number(d, "spacing", 400.), radius)
        }
        t => panic!("Unknown terrain domain type: {}", t),
    }
}

/// The terrain an object describes, evolved to its `time`: from the cache directory given as
/// `cache`, if it has it.
pub fn evolved(o: &Value, params: TerrainParams) -> Terrain {
    let d = domain(o, &params);
    evolve(o, Terrain::new(d, params), &o["domain"].to_string())
}

/// Run `t` to the object's `time`, through its cache. `key` identifies the domain.
fn evolve(o: &Value, mut t: Terrain, key: &str) -> Terrain {
    let time = number(o, "time", 0.);
    let start = std::time::Instant::now();
    match o["cache"].as_str() {
        Some(dir) => t.run_cached(time, std::path::Path::new(dir), key),
        None => t.run_to(time),
    }
    println!("Terrain: {} cells evolved for {} years in {:.1?}", t.domain.len(), time, start.elapsed());
    t
}

pub fn create_terrain(o: &Value) -> SceneObject {
    let mut params = terrain_params(o);
    // Without a `scale`, the landscape is drawn as itself, in metres on the scene's planet.
    if o.get("scale").is_none() {
        if o.get("radius").is_none() { params.planet.radius = crate::planet::EARTH_RADIUS; }
        return full_scale_landscape(o, params);
    }
    let t = evolved(o, params);
    if let Some(path) = o["map"].as_str() { write_map(&t, path, 2048); }
    let place = Placement {
        scale: number(o, "scale", 1000.),
        exaggeration: number(o, "exaggeration", 1.),
        centre: SceneFile::parse_vec3_def(o, "centre", Vector3::zeros()),
    };
    let geometry = mesh(&t, &place);
    SceneObject { geometry: Box::new(geometry), medium: Box::new(TerrainMedium::new(t, place, number(o, "min_river_width", 0.))) }
}

/// A patch level of a full-scale landscape: cells along a side, and their spacing (m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Level { pub cells: usize, pub spacing: f64 }

/// The patch levels, finest first: `"levels": [{"cells", "spacing"}, ...]`, or by default the
/// domain's patch and coarser ones five times wider apart, until they are as coarse as the
/// planet's cells (spacing `globe`).
pub fn levels(o: &Value, globe: f64) -> Vec<Level> {
    if let Some(ls) = o["levels"].as_array() {
        return ls.iter().map(|l| Level { cells: SceneFile::parse_int(&l["cells"], 128), spacing: number(l, "spacing", 750.) }).collect();
    }
    let d = &o["domain"];
    let mut l = Level { cells: SceneFile::parse_int(&d["cells"], 128), spacing: number(d, "spacing", 750.) };
    let mut out = vec![l];
    while l.spacing * 5. < globe / 2. {
        l.spacing *= 5.;
        out.push(l);
    }
    out
}

/// Where the landscape needs most detail: `"focus"` (a scene point), or the ground the camera
/// looks over: below it, moved towards where it looks by up to `ahead` metres.
fn focus(o: &Value, frame: &Frame, ahead: f64) -> Vector3<f64> {
    if o.get("focus").is_some() { return frame.on_planet(&SceneFile::parse_vec3(&o["focus"])).0; }
    let camera = SceneFile::parse_vec3_def(&o["camera"], "location", Vector3::new(0., 2., 0.));
    let lookat = SceneFile::parse_vec3_def(&o["camera"], "lookat", camera);
    let (foot, _) = frame.on_planet(&camera);
    let (x, z) = frame.tangent(&foot);
    let (lx, lz) = frame.tangent(&frame.on_planet(&lookat).0);
    let (dx, dz) = (lx - x, lz - z);
    let len = dx.hypot(dz);
    if len < 1e-9 { return foot; }
    let k = len.min(ahead) / len;
    frame.dir(x + dx * k, z + dz * k)
}

/// A landscape at full scale over the whole planet (see `scenery`):
///
/// - `domain`: the site at the scene's origin: `{"lat", "lon"}` or `{"near": "mountains"}`
///   (or `"volcanoes"`, `"coast"`), and the finest patch's `cells` and `spacing`.
/// - `planet`: `{"cells": 64}`, cells per face edge of the whole planet's simulation (rivers
///   and hillslopes, no glaciers), or `false` for its noise topography alone.
/// - `levels`: patches nested about the `focus` (see `levels`; `[]` for none, as from orbit,
///   where the planet's own relief is drawn down to the domain's `spacing`), each driven at its edge by the
///   one around it. Glaciers only run on the finest (`"glacier"` as for a patch), as at
///   kilometres per cell the ice can't drain down valleys narrower than a cell.
/// - `focus`: a scene point to centre the patches on; by default the ground the scene's camera
///   (passed in as `camera`) looks over. Patches are snapped to an eighth of their width, so
///   nearby cameras share them (and their cache).
/// - The mesh: `resolution` (vertices along a clipmap ring), `min_spacing` (m), `globe` (cells
///   per face edge beyond the rings), `roughness` of the detail below the finest cells.
/// - Ground cover: `snowline` and `treeline` (m; by default from the climate, by latitude),
///   `min_river_width` (m).
fn full_scale_landscape(o: &Value, params: TerrainParams) -> SceneObject {
    let mut p = SceneryParams::default();
    p.resolution = SceneFile::parse_int(&o["resolution"], p.resolution).max(8);
    p.min_spacing = number(o, "min_spacing", p.min_spacing).max(1e-3);
    p.globe = SceneFile::parse_int(&o["globe"], p.globe).max(2);
    p.roughness = number(o, "roughness", p.roughness);
    p.exaggeration = number(o, "exaggeration", p.exaggeration);
    p.snowline = o["snowline"].as_f64();
    p.treeline = o["treeline"].as_f64();
    p.min_river_width = number(o, "min_river_width", p.min_river_width);
    let radius = params.planet.radius;
    let time = number(o, "time", 0.);
    let (lat, lon) = site(&o["domain"], &params);
    let frame = Frame::new(lat, lon, radius, p.exaggeration);

    // The simulations, coarsest first, each driving the next at its edge.
    let mut built: Vec<Layer> = vec![];
    let mut parent: Option<Vec<f64>> = None;
    let planet_cells = match &o["planet"] { Value::Bool(false) => None, v => Some(SceneFile::parse_int(&v["cells"], 64).max(2)) };
    let globe_spacing = planet_cells.map_or(f64::INFINITY, |n| std::f64::consts::FRAC_PI_2 * radius / n as f64);
    if let Some(n) = planet_cells {
        let mut gp = params.clone();
        gp.glacier = None;
        let t = Terrain::new(Domain::sphere(n, radius), gp);
        parent = Some(scenery::corner_heights(&t));
        built.push(Layer::new(evolve(o, t, &format!("planet {}", n)), p.min_river_width));
    }
    let levels = levels(o, globe_spacing);
    let finest = levels.first().map_or(number(&o["domain"], "spacing", 750.), |l| l.spacing);
    let finest_half = levels.first().map_or(0., |l| l.cells as f64 * l.spacing / 2.);
    let centre = focus(o, &frame, 0.5 * finest_half);
    let (fx, fz) = frame.tangent(&centre);
    for (k, l) in levels.iter().enumerate().rev() {
        let width = l.cells as f64 * l.spacing;
        let snap = |c: f64| (c / (width / 8.)).round() * (width / 8.);
        let (lat, lon) = Domain::lat_lon(&frame.dir(snap(fx), snap(fz)));
        let mut lp = params.clone();
        if k > 0 { lp.glacier = None; }
        let mut t = Terrain::new(Domain::patch(lat, lon, l.cells, l.spacing, radius), lp);
        let initial = scenery::corner_heights(&t);
        if let (Some(outer), Some(before), true) = (built.last(), &parent, time > 0.) {
            // The edge follows the change the simulation around it went through there.
            t.boundary_rate = (0..t.domain.len()).map(|i| {
                if !t.domain.boundary[i] { return 0.; }
                let d = t.domain.dirs[i];
                (outer.simulated(&d) - scenery::sample(&outer.domain, before, &d)) / time
            }).collect();
        }
        parent = Some(initial);
        let key = format!("patch {:.9} {:.9} {} {}", lat, lon, l.cells, l.spacing);
        let t = evolve(o, t, &key);
        if k == 0 {
            if let Some(path) = o["map"].as_str() { write_map(&t, path, 2048); }
        }
        built.push(Layer::new(t, p.min_river_width));
    }
    built.reverse();

    let start = std::time::Instant::now();
    let planet = Planet::new(params.planet.clone());
    if let Some(path) = o["planet_map"].as_str() { crate::terrain::map::write_planet_map(&planet, path, 2048); }
    let layers = Arc::new(built);
    let relief = Relief::new(layers.clone(), &planet, radius, p.roughness, finest);
    let view = Viewpoint { location: SceneFile::parse_vec3_def(&o["camera"], "location", Vector3::new(0., 2., 0.)) };
    let geometry = scenery::mesh(&relief, &frame, &view, &p);
    let climate = params.climate.as_ref().map(|c| crate::terrain::climate::climate(c, &planet));
    if let (Some(path), Some(c)) = (o["climate_map"].as_str(), &climate) { crate::terrain::map::write_climate_map(&planet, c, path, 2048); }
    let ground = Ground::new(layers, climate, params.glacier.clone().unwrap_or_default(), time, params.planet.seed, frame, &p);
    println!("Landscape: {} triangles in {:.1?}", geometry.primitives(), start.elapsed());
    SceneObject { geometry: Box::new(geometry), medium: Box::new(LandscapeMedium::new(ground)) }
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
            "scale": 1000,
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

    #[test]
    fn test_full_scale_landscape_lies_on_the_planet() {
        let o: Value = serde_json::from_str(r#"{
            "type": "landscape", "seed": 3, "time": 0,
            "domain": { "type": "patch", "cells": 16, "spacing": 1000 },
            "planet": { "cells": 8 }, "resolution": 32, "globe": 8,
            "camera": { "location": [0, 3000, 0], "lookat": [0, 0, -1000] }
        }"#).unwrap();
        assert_eq!(levels(&o, 2e5), vec![Level { cells: 16, spacing: 1000. }, Level { cells: 16, spacing: 5000. }, Level { cells: 16, spacing: 25000. }]);
        let obj = create_terrain(&o);
        // A ray straight down onto the site meets ground within a few km of sea level.
        let r = Ray { ro: Vector3::new(10., 20000., 10.), rd: Vector3::new(0., -1., 0.) };
        let hit = obj.geometry.intersects(&r).expect("no hit");
        assert!(hit.dist > 10000. && hit.dist < 21000., "{}", hit.dist);
        // The ground goes on to the horizon and round the planet.
        let r = Ray { ro: Vector3::new(300000., 20000., 0.), rd: Vector3::new(0., -1., 0.) };
        assert!(obj.geometry.intersects(&r).is_some());
        let r = Ray { ro: Vector3::new(0., -2e7, 0.), rd: Vector3::new(0., 1., 0.) };
        assert!(obj.geometry.intersects(&r).is_some());
    }
}
