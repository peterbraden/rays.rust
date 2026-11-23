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
//!   2 m deep), lava, then a tint by elevation; or `"material"`, the name of a material for
//!   all the ground but the rivers, typically `surface_layers` masked by what the simulation
//!   left (`field` nodes in its graphs: sediment, ice, flow, wetness and so on, see
//!   `graph::Field`).
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
use crate::terrain::scenery::{self, Clipmap, Frame, Ground, Hole, LandscapeMedium, Layer, Local, Relief, SceneryParams, Surface, Viewpoint};
use crate::world::{self, Description, Focus, Site, World};
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

    /// The direction on the planet under scene position `p`, and its height there (metres).
    pub fn on_terrain(&self, domain: &Domain, p: &Vector3<f64>) -> (Vector3<f64>, f64) {
        let q = (p - self.centre) * self.scale;
        match &domain.layout {
            Layout::Patch { up, east, north, .. } => ((up * domain.radius + east * q.x - north * q.z).normalize(), q.y / self.exaggeration),
            Layout::Sphere { .. } => (q.normalize(), (q.norm() - domain.radius) / self.exaggeration),
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
    /// What the simulation left, for graphs' `field` nodes.
    fields: crate::terrain::fields::SimFields,
    /// The scene's material for the ground, in place of the cover classes.
    ground: Option<Box<dyn MaterialModel + Sync + Send>>,
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
    pub fn new(t: Terrain, place: Placement, min_river_width: f64, ground: Option<Box<dyn MaterialModel + Sync + Send>>) -> TerrainMedium {
        let fields = crate::terrain::fields::SimFields::new(&t);
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
            Box::new(Lambertian { albedo: Color::new(c[0], c[1], c[2]).into() })
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
        TerrainMedium { domain: t.domain, place, jitter, class, materials, water, fields, ground }
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
        if let Some(g) = &self.ground
            && (self.water.is_some() || self.class[self.place.cell_at(&self.domain, &pt)] != RIVER) {
            return &**g;
        }
        let j = Vector3::new(rand() - 0.5, rand() - 0.5, rand() - 0.5) * self.jitter;
        &*self.materials[self.class[self.place.cell_at(&self.domain, &(pt + j))] as usize]
    }

    fn fields(&self) -> Option<&dyn crate::graph::Fields> { Some(self) }
}

impl crate::graph::Fields for TerrainMedium {
    fn field(&self, f: crate::graph::Field, p: &Vector3<f64>) -> Option<f64> {
        let (d, h) = self.place.on_terrain(&self.domain, p);
        self.fields.at(&self.domain, f, &d, h)
    }

    /// Round the sphere of a `planet` layout; a patch is laid out with y up.
    fn vertical(&self, p: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> {
        match self.domain.layout {
            Layout::Patch { .. } => None,
            Layout::Sphere { .. } => {
                let q = p - self.place.centre;
                let r = q.norm();
                (r > 0.).then(|| (q / r, r - self.domain.radius / self.place.scale))
            },
        }
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

/// Where a domain says it lies: `lat` and `lon`, or `near` a kind of place.
pub fn site_of(d: &Value) -> Site {
    match (d["lat"].as_f64(), d["lon"].as_f64(), d["near"].as_str()) {
        (Some(lat), Some(lon), _) => Site::At { lat, lon },
        (_, _, Some("volcanoes")) => Site::Volcanoes,
        (_, _, Some("coast")) => Site::Coast,
        (_, _, None | Some("mountains")) => Site::Mountains,
        (_, _, Some(x)) => panic!("Unknown terrain domain \"near\": {}", x),
    }
}

/// Where a patch lies: its centre's (latitude, longitude) in degrees.
pub fn site(d: &Value, params: &TerrainParams) -> (f64, f64) {
    match site_of(d) {
        Site::At { lat, lon } => (lat, lon),
        near => near.find(&Planet::new(params.planet.clone())),
    }
}

/// A line about the landscape as it is made, to whoever is listening (`world::report`).
fn say(message: String) {
    world::report(&world::Progress::note("landscape", message));
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
    let start = web_time::Instant::now();
    match o["cache"].as_str() {
        Some(dir) => t.run_cached(time, std::path::Path::new(dir), key),
        None => t.run_to(time),
    }
    println!("Terrain: {} cells evolved for {} years in {:.1?}", t.domain.len(), time, start.elapsed());
    t
}

/// The landscape's `"material"`, if it names one, for its ground.
fn ground_material(o: &Value, materials: &serde_json::Map<String, Value>) -> Option<Box<dyn MaterialModel + Sync + Send>> {
    o.get("material").map(|m| SceneFile::require_material(m, materials, "landscape"))
}

pub fn create_terrain(o: &Value, materials: &serde_json::Map<String, Value>) -> SceneObject {
    create_landscape(o, materials).swap_remove(0)
}

/// The landscape, and at full scale the water in its rivers near the camera (a surface of its
/// own, which transparent water needs to see the ground beneath it), in that order.
pub fn create_landscape(o: &Value, materials: &serde_json::Map<String, Value>) -> Vec<SceneObject> {
    let mut params = terrain_params(o);
    // Without a `scale`, the landscape is drawn as itself, in metres on the scene's planet.
    if o.get("scale").is_none() {
        if o.get("radius").is_none() { params.planet.radius = crate::planet::EARTH_RADIUS; }
        let (land, water) = full_scale_landscape(o, params, ground_material(o, materials));
        return std::iter::once(land).chain(water).collect();
    }
    let t = evolved(o, params);
    if let Some(path) = o["map"].as_str() { write_map(&t, path, 2048); }
    let place = Placement {
        scale: number(o, "scale", 1000.),
        exaggeration: number(o, "exaggeration", 1.),
        centre: SceneFile::parse_vec3_def(o, "centre", Vector3::zeros()),
    };
    let geometry = mesh(&t, &place);
    vec![SceneObject { geometry: Box::new(geometry), medium: Box::new(TerrainMedium::new(t, place, number(o, "min_river_width", 0.), ground_material(o, materials))) }]
}

pub use crate::world::Level;

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
/// looks over (`world::Focus`).
fn focus(o: &Value) -> Focus {
    if o.get("focus").is_some() { return Focus::Point(SceneFile::parse_vec3(&o["focus"])); }
    let location = SceneFile::parse_vec3_def(&o["camera"], "location", Vector3::new(0., 2., 0.));
    Focus::Camera { location, lookat: SceneFile::parse_vec3_def(&o["camera"], "lookat", location) }
}

/// A landscape's `"cache"`: the directory its world is kept in.
pub fn world_directory(o: &Value) -> Option<&std::path::Path> {
    o["cache"].as_str().map(std::path::Path::new)
}

/// A landscape at full scale over the whole planet (see `scenery`):
///
/// - `domain`: the site at the scene's origin: `{"lat", "lon"}` or `{"near": "mountains"}`
///   (or `"volcanoes"`, `"coast"`), and the finest patch's `cells` and `spacing`.
/// - `planet`: `{"cells": 64}`, cells per face edge of the whole planet's simulation (rivers
///   and hillslopes, no glaciers), or `false` for its noise topography alone.
/// - `levels`: patches nested about the `focus` (see `levels`; `[]` for none, as from orbit,
///   where the planet's own relief is drawn down to the domain's `spacing`), each driven at its edge by the
///   one around it. Glaciers and meandering channels only run on the finest (`"glacier"` and
///   `"channels"` as for a patch): at kilometres per cell the ice can't drain down valleys
///   narrower than a cell, and every river's bends are smaller than one.
/// - `focus`: a scene point to centre the patches on; by default the ground the scene's camera
///   (passed in as `camera`) looks over. Patches are snapped to an eighth of their width, so
///   nearby cameras share them (and their cache).
/// - `cache`: the world directory: the plates, the climate, the site and each simulation are
///   kept there by key and taken from there when they are found, with a manifest of the
///   stages (`world`, which builds all of them). Without it they are built in memory.
/// - The mesh: `resolution` (vertices along a clipmap ring), `min_spacing` (m), `globe` (cells
///   per face edge beyond the rings), `roughness` of the detail below the finest cells.
/// - Ground cover: `snowline` and `treeline` (m; by default from the climate, by latitude),
///   `min_river_width` (m), `river_flow` (the rivers' discharge as a multiple of their mean
///   annual flow: `terrain::hydraulics`; near the camera rivers get channels and transparent
///   surfaces of their own, `terrain::rivers`); or `"material"`, a material for the ground but rivers and lakes,
///   whose graphs' `field` nodes read the simulations (blended between patches) and the climate.
/// - `displacement`: a graph displacing the ground about the camera at render time, as on a
///   `terrain` (see `displaced_patch`), with `displacement_detail`, `vertical_displacement`
///   and so on as there; `displacement_reach` (m, default 1000) out from under the camera on
///   each side, on a heightfield of `displacement_spacing` (m, default 2).
fn full_scale_landscape(o: &Value, params: TerrainParams, material: Option<Box<dyn MaterialModel + Sync + Send>>) -> (SceneObject, Vec<SceneObject>) {
    let mut p = SceneryParams::default();
    p.resolution = SceneFile::parse_int(&o["resolution"], p.resolution).max(8);
    p.min_spacing = number(o, "min_spacing", p.min_spacing).max(1e-3);
    p.globe = SceneFile::parse_int(&o["globe"], p.globe).max(2);
    p.roughness = number(o, "roughness", p.roughness);
    p.exaggeration = number(o, "exaggeration", p.exaggeration);
    p.snowline = o["snowline"].as_f64();
    p.treeline = o["treeline"].as_f64();
    p.min_river_width = number(o, "min_river_width", p.min_river_width);
    p.hydraulics.flow = number(o, "river_flow", p.hydraulics.flow).max(0.);
    let radius = params.planet.radius;
    let time = number(o, "time", 0.);
    let planet_cells = match &o["planet"] { Value::Bool(false) => None, v => Some(SceneFile::parse_int(&v["cells"], 64).max(2)) };
    let globe_spacing = planet_cells.map_or(f64::INFINITY, |n| std::f64::consts::FRAC_PI_2 * radius / n as f64);
    let levels = levels(o, globe_spacing);
    let finest = levels.first().map_or(number(&o["domain"], "spacing", 750.), |l| l.spacing);

    // The world: the planet, the site, and the simulations nested about the focus, each
    // driving the next at its edge (`world`), kept in the `cache` directory if there is one.
    let description = Description {
        params: params.clone(), site: site_of(&o["domain"]), time, planet_cells, levels,
        focus: focus(o), exaggeration: p.exaggeration,
    };
    let world = world::build(&description, world_directory(o), &mut |e| { world::report(e); true }).unwrap_or_else(|e| panic!("{}", e));
    let frame = world.frame.clone();
    if let Some(t) = world.patches.first() {
        if let Some(path) = o["map"].as_str() { write_map(t, path, 2048); }
        // Where the glaciers are, to frame them.
        let ice = (0..t.domain.len()).filter(|&i| t.ice[i] >= crate::terrain::simulation::ICE_FREE);
        let cells = ice.clone().count();
        if let Some(i) = ice.max_by(|&a, &b| t.ice[a].total_cmp(&t.ice[b])) {
            let at = frame.to_scene(&t.domain.dirs[i], t.surface()[i]);
            let spacing = description.levels[0].spacing;
            say(format!("Landscape: glaciers cover {:.0} km²; the thickest ice, {:.0} m, is at [{:.0}, {:.0}, {:.0}]",
                cells as f64 * spacing * spacing / 1e6, t.ice[i], at.x, at.y, at.z));
        }
    }
    let World { planet, climate, globe, patches, .. } = world;
    let built: Vec<Layer> = patches.into_iter().chain(globe).map(|t| Layer::new(t, p.min_river_width, &p.hydraulics)).collect();

    let start = web_time::Instant::now();
    if let Some(path) = o["planet_map"].as_str() { crate::terrain::map::write_planet_map(&planet, path, 2048); }
    let layers = Arc::new(built);
    let relief = Arc::new(Relief::new(layers.clone(), &planet, radius, p.roughness, finest));
    let view = Viewpoint { location: SceneFile::parse_vec3_def(&o["camera"], "location", Vector3::new(0., 2., 0.)) };
    let (foot, height) = frame.on_planet(&view.location);
    let ground = relief.height(&foot, relief.slope(&foot), 1.);
    say(format!("Landscape: the camera is {:.1} m above the ground under it ({:.1} m above sea level)", height - ground, ground));
    if let (Some(path), Some(c)) = (o["climate_map"].as_str(), &climate) { crate::terrain::map::write_climate_map(&planet, c, path, 2048); }
    // Built first, so the displacement's graphs can read its fields.
    let ground = Arc::new(Ground::new(layers, climate, params.glacier.clone().unwrap_or_default(), time, params.planet.seed, frame.clone(), &p));
    let patches = o.get("displacement").map(|d| displaced_patches(o, d, &relief, &frame, &view, &p, ground.clone()));
    let mesh = scenery::mesh(&relief, &frame, &view, &p, patches.as_ref().map(|(h, _)| h));
    let ground_for_water = ground.clone();
    let geometry: Box<dyn Geometry + Send + Sync> = match patches {
        Some((_, terrains)) => {
            let mut parts: Vec<Box<dyn Geometry + Send + Sync>> = vec![Box::new(mesh)];
            parts.extend(terrains);
            Box::new(crate::shapes::group::Group::new(parts))
        },
        None => Box::new(mesh),
    };
    say(format!("Landscape: {} triangles in {:.1?}", geometry.primitives(), start.elapsed()));
    let _ = ground_for_water.clipmap.set(Clipmap::new(&relief, &frame, &view, &p));
    let water = river_water(&relief, &frame, &view, &p, params.planet.seed);
    let falls = waterfalls(&relief, &frame, params.planet.seed);
    (SceneObject { geometry, medium: Box::new(LandscapeMedium::with_ground(ground, material)) }, water.into_iter().chain(falls).collect())
}

/// The rivers' water near the camera, where the mesh is fine enough to carve their channels
/// (`terrain::rivers`): transparent, so their beds show through.
/// The waterfalls' jets (`terrain::rivers::Fall`), each a curtain of falling water.
fn waterfalls(relief: &Relief, frame: &Frame, seed: u64) -> Vec<SceneObject> {
    use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
    use crate::terrain::rivers::patch_dir;
    let mut out = vec![];
    for l in relief.layers.iter() {
        let Some(carve) = l.rivers.as_ref().and_then(|r| r.carve.get()) else { continue };
        for f in &carve.falls {
            let d = patch_dir(&l.domain, f.node.p);
            if !relief.layers.iter().find(|m| m.contains(&d)).is_some_and(|m| std::ptr::eq(m, l)) { continue; }
            let (vertices, triangles) = f.sheet(48, 8);
            let positions: Vec<Vector3<f64>> = vertices.iter().map(|(q, h, _)| frame.to_scene(&patch_dir(&l.domain, *q), *h)).collect();
            // Normals across the sheet: facing out along the jet, as a flat sheet's would.
            let out_dir = frame.to_scene(&patch_dir(&l.domain, f.point(1., 0.)), f.top) - frame.to_scene(&d, f.top);
            let normals = vec![out_dir.normalize(); positions.len()];
            let mesh = SmoothMesh::from_indexed(&[IndexedMesh { positions, normals, triangles, texcoords: vec![] }]);
            let material = crate::terrain::river_water::FallingWater {
                frame: frame.clone(), top: f.top, breakup: f.jet.breakup, noise: crate::noise::PerlinNoise::shuffled(seed.wrapping_add(0xfa11)),
            };
            let geometry = crate::terrain::river_water::Curtain { mesh, frame: frame.clone(), top: f.top, breakup: f.jet.breakup };
            out.push(SceneObject { geometry: Box::new(geometry), medium: Box::new(crate::material::texture::Solid { m: Box::new(material) }) });
        }
    }
    out
}

fn river_water(relief: &Arc<Relief>, frame: &Frame, view: &Viewpoint, p: &SceneryParams, seed: u64) -> Option<SceneObject> {
    use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
    use crate::terrain::rivers::patch_dir;
    let clipmap = Clipmap::new(relief, frame, view, p);
    let (mut positions, mut normals, mut triangles) = (vec![], vec![], vec![]);
    for l in relief.layers.iter() {
        let Some(carve) = l.rivers.as_ref().and_then(|r| r.carve.get()) else { continue };
        let keep = |q: crate::terrain::channel::Point, half: f64| {
            let d = patch_dir(&l.domain, q);
            let finest = relief.layers.iter().find(|m| m.contains(&d)).is_some_and(|m| std::ptr::eq(m, l));
            let (x, z) = frame.tangent(&d);
            finest && clipmap.local(x, z).is_some_and(|s| s < half)
        };
        let (vertices, tris) = carve.ribbon(keep);
        let base = positions.len();
        for (q, level) in vertices {
            let d = patch_dir(&l.domain, q);
            positions.push(frame.to_scene(&d, level));
            normals.push((frame.to_scene(&d, 0.) - frame.centre()).normalize());
        }
        triangles.extend(tris.into_iter().map(|t| [t[0] + base, t[1] + base, t[2] + base]));
    }
    // Where the nearest river is, to frame it.
    let near = relief.layers.iter().filter_map(|l| {
        let carve = l.rivers.as_ref()?.carve.get()?;
        let (foot, _) = frame.on_planet(&view.location);
        let q = crate::terrain::rivers::patch_point(&l.domain, &foot)?;
        let (p, level, node) = carve.nearest(q)?;
        // Only rivers drawn: where this layer is the finest.
        let d = patch_dir(&l.domain, p);
        if !relief.layers.iter().find(|m| m.contains(&d)).is_some_and(|m| std::ptr::eq(m, l)) { return None; }
        let at = frame.to_scene(&patch_dir(&l.domain, p), level);
        // A place to see it from: on a bank a few metres from the water, eyes 1.7 m above the
        // ground, looking along it 150 m, where the relief doesn't hide the water.
        let (_, along) = carve.water_at(p)?;
        let across = [-along[1], along[0]];
        let ground = |q: crate::terrain::channel::Point| { let d = patch_dir(&l.domain, q); relief.height(&d, relief.slope(&d), 1.) };
        let shift = |o: crate::terrain::channel::Point, a: f64, c: f64| [o[0] + along[0] * a + across[0] * c, o[1] + along[1] * a + across[1] * c];
        let views = [(1., 1.), (-1., 1.), (1., -1.), (-1., -1.)].into_iter().flat_map(|(dir, side)| [4., 10., 25.].map(|off| (dir, side * (node.half + off))));
        let seen = views.filter_map(|(dir, side)| {
            let eye_q = shift(p, 0., side);
            let eye = ground(eye_q) + 1.7;
            let target = shift(p, 150. * dir, 0.);
            let clear = (1..30).all(|k| {
                let t = k as f64 / 30.;
                let q = [eye_q[0] + (target[0] - eye_q[0]) * t, eye_q[1] + (target[1] - eye_q[1]) * t];
                ground(q) + 0.3 < eye + (level - eye) * t
            });
            clear.then(|| (frame.to_scene(&patch_dir(&l.domain, eye_q), eye), frame.to_scene(&patch_dir(&l.domain, target), level)))
        }).next();
        Some(((at - view.location).norm(), at, node, seen))
    }).min_by(|a, b| a.0.total_cmp(&b.0));
    // The waterfalls, the tallest and the nearest, to frame them; and a place to see the
    // tallest from: downstream and to one side, a little above the ground.
    // Each fall's height, discharge, lip, middle and viewpoint.
    type Seen = (f64, f64, Vector3<f64>, Vector3<f64>, Vector3<f64>);
    let falls: Vec<Seen> = relief.layers.iter().filter_map(|l| Some((l, l.rivers.as_ref()?.carve.get()?)))
        .flat_map(|(l, carve)| carve.falls.iter().filter(move |f| {
            // Only where this layer is the finest: elsewhere a finer one's rivers are drawn.
            let d = patch_dir(&l.domain, f.node.p);
            relief.layers.iter().find(|m| m.contains(&d)).is_some_and(|m| std::ptr::eq(m, l))
        }).map(move |f| {
            let h = f.top - f.foot;
            let at = frame.to_scene(&patch_dir(&l.domain, f.node.p), f.top);
            let middle = frame.to_scene(&patch_dir(&l.domain, f.point(f.jet.speed * f.jet.time_to(h / 2.), 0.)), f.top - h / 2.);
            let (mx, mh) = (f.jet.speed * f.jet.time_to(h / 2.), f.top - h / 2.);
            // A viewpoint that sees the fall's middle over the relief: downstream first, then
            // round to either side, near then farther.
            let ground = |q: crate::terrain::channel::Point| { let d = patch_dir(&l.domain, q); relief.height(&d, relief.slope(&d), 1.) };
            let candidates = [3. * h + 60., 6. * h + 150.].into_iter().flat_map(|r| [0., 30., -30., 60., -60., 90., -90., 120., -120.].map(|deg: f64| (r, deg.to_radians())));
            let from = candidates.filter_map(|(r, a)| {
                let q = f.point(mx + r * a.cos(), r * a.sin());
                let eye = ground(q) + 2.;
                let clear = (1..40).all(|k| {
                    let t = k as f64 / 40.;
                    let p = f.point(mx + r * a.cos() * (1. - t), r * a.sin() * (1. - t));
                    t > 0.9 || ground(p) + 1. < eye + (mh - eye) * t
                });
                clear.then(|| frame.to_scene(&patch_dir(&l.domain, q), eye))
            }).next().unwrap_or(middle);
            (h, f.node.discharge, at, middle, from)
        })).collect();
    if let Some(tallest) = falls.iter().max_by(|a, b| a.0.total_cmp(&b.0)) {
        let nearest = falls.iter().min_by(|a, b| (a.2 - view.location).norm().total_cmp(&(b.2 - view.location).norm())).unwrap();
        let v = |p: &Vector3<f64>| format!("[{:.0}, {:.1}, {:.0}]", p.x, p.y, p.z);
        say(format!("Landscape: {} waterfalls; the tallest falls {:.0} m ({:.1} m³/s) from {} (seen from {} looking at {}); the nearest, {:.0} m, is {:.0} m away at {} (seen from {} looking at {})",
            falls.len(), tallest.0, tallest.1, v(&tallest.2), v(&tallest.4), v(&tallest.3), nearest.0, (nearest.2 - view.location).norm(), v(&nearest.2), v(&nearest.4), v(&nearest.3)));
    }
    if let Some((distance, at, n, seen)) = near {
        say(format!("Landscape: the nearest river, {:.0} m wide and {:.2} m deep (Froude {:.2}, {:.0} mg/L of silt, grains {:.1} mm), is {:.0} m away at [{:.0}, {:.1}, {:.0}]",
            2. * n.half, 1.5 * n.depth, n.froude, n.turbidity, 1000. * n.grain, distance, at.x, at.y, at.z));
        if let Some((from, to)) = seen {
            say(format!("Landscape: see the river from [{:.0}, {:.1}, {:.0}] looking at [{:.0}, {:.1}, {:.0}]", from.x, from.y, from.z, to.x, to.y, to.z));
        }
    }
    if triangles.is_empty() { return None; }
    say(format!("Landscape: rivers' water in {} triangles", triangles.len()));
    let mesh = SmoothMesh::from_indexed(&[IndexedMesh { positions, normals, triangles, texcoords: vec![] }]);
    // Clear water, a little more absorbing than the sea's (its colour from suspended sediment
    // is for step 3).
    // Shaded by the flow under each point (`terrain::river_water`).
    let water = Arc::new(crate::terrain::river_water::RiverWater {
        relief: relief.clone(), frame: frame.clone(), noise: crate::noise::PerlinNoise::shuffled(seed.wrapping_add(0x5a7e5)),
    });
    let lookup = water.clone();
    let surface = crate::terrain::rivers::WaterSurface { mesh, attenuation: Box::new(move |p| lookup.attenuation(p)) };
    Some(SceneObject { geometry: Box::new(surface), medium: Box::new(crate::material::texture::Solid { m: Box::new(ArcMaterial(water)) }) })
}

/// A shared material.
struct ArcMaterial<M>(Arc<M>);

impl<M: MaterialModel + Sync + Send> MaterialModel for ArcMaterial<M> {
    fn scatter(&self, r: &Ray, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> crate::material::model::ScatteredRay { self.0.scatter(r, i, s) }
    fn sun_glint(&self, r: &Ray, i: &crate::intersection::Intersection, s: &crate::scene::Scene) -> Option<Color> { self.0.sun_glint(r, i, s) }
}

/// The ground about the camera displaced at render time (`shapes::displaced`), out to
/// `displacement_reach` metres on each side: nested square patches of `displacement_cells`
/// cells a side (512), the finest `displacement_spacing` metres apart (2), each next twice as
/// coarse and twice as wide, leaving a hole for the one inside it, which meets it without
/// cracks (`displaced::Nest`); and the hole they leave in the clipmap's mesh. The outermost's
/// displacement fades out towards its edge over at least three of the rings' spacings there,
/// so the rings' triangles that overlap it (moved onto its surface) meet it undisplaced.
///
/// They stand upright in a `Local` frame on the planet under the camera, put into the scene by
/// transforms; the frame is snapped to whole multiples of the reach, and the patches to the
/// outermost's cells, so that nearby cameras (an animation's frames) share them, and their
/// graphs are evaluated at the scene's points, so the displacement stays put as they move. For
/// a camera with `motion`, they are centred on the middle of the path and reach the same
/// distance past either end.
fn displaced_patches(o: &Value, d: &Value, relief: &Relief, frame: &Frame, view: &Viewpoint, p: &SceneryParams, ground: Arc<Ground>)
    -> (Hole, Vec<Box<dyn Geometry + Send + Sync>>)
{
    use crate::procedural::heightmap::{HeightGrid, HeightSource};
    use crate::shapes::displaced::{DisplacedTerrain, Nest};
    let camera = &o["camera"];
    let start_at = view.location;
    let end_at = SceneFile::parse_vec3_def(&camera["motion"], "location", start_at);
    let middle = (start_at + end_at) / 2.;
    let finest = number(o, "displacement_spacing", 2.).max(0.01);
    let cells = (SceneFile::parse_int(&o["displacement_cells"], 512).max(8) / 4 * 4) as i64;
    let (tx, tz) = frame.tangent(&frame.on_planet(&middle).0);
    let reach = number(o, "displacement_reach", 1000.).max(1.);
    let coarse = |c: f64| (c / reach).round() * reach;
    let local = Local::under(frame, coarse(tx), coarse(tz));
    let (m, e) = (local.to_local(&middle), local.to_local(&end_at));
    let reach = reach + (e.x - m.x).hypot(e.z - m.z);
    // Patches k = 0 (finest) to n - 1: cells finest * 2^k, half-widths cells / 2 of them; a
    // single patch is cut to the reach.
    let half = |k: usize| (cells / 2) as f64 * finest * (1u64 << k) as f64;
    let n = (0..12).find(|&k| half(k) >= reach).unwrap_or(11) + 1;
    let outermost = finest * (1u64 << (n - 1)) as f64;
    let snap = |c: f64| (c / outermost).round() * outermost;
    let (cx, cz) = (snap(m.x), snap(m.z));
    let begun = std::time::Instant::now();
    let clipmap = Clipmap::new(relief, frame, view, p);
    // Diced for the camera as it sees them, in the frame (from the middle of its path, if it
    // moves).
    let mut po = o.clone();
    let json = |v: Vector3<f64>| serde_json::json!([v.x, v.y, v.z]);
    po["camera"]["lookat"] = json(local.to_local(&SceneFile::parse_vec3_def(camera, "lookat", middle)));
    po["camera"]["location"] = json(local.to_local(&middle));
    po["camera"]["up"] = json(local.axes.transpose() * SceneFile::parse_vec3_def(camera, "up", Vector3::y()));
    let mut matrix = crate::na::Matrix4::identity();
    matrix.fixed_view_mut::<3, 3>(0, 0).copy_from(&local.axes);
    matrix.fixed_view_mut::<3, 1>(0, 3).copy_from(&local.origin);
    let placement = crate::na::Affine3::from_matrix_unchecked(matrix);

    /// A patch's heights: the landscape's surface, joining the outer patch's within a few cells
    /// of its edge, so their bases agree where they meet.
    struct Joined<'a> { surface: &'a Surface<'a>, outer: Option<&'a HeightGrid>, min: (f64, f64), max: (f64, f64), band: f64 }
    impl HeightSource for Joined<'_> {
        fn height(&self, x: f64, z: f64) -> f64 {
            let Some(outer) = self.outer else { return self.surface.height(x, z) };
            let edge = (x - self.min.0).min(self.max.0 - x).min(z - self.min.1).min(self.max.1 - z);
            let t = (edge / self.band).clamp(0., 1.);
            let w = t * t * (3. - 2. * t);
            let theirs = outer.smooth(x, z).0;
            if w <= 0. { theirs } else { theirs + (self.surface.height(x, z) - theirs) * w }
        }
    }

    let mut parts: Vec<Box<dyn Geometry + Send + Sync>> = vec![];
    // The patch just made: the terrain, its heights and its corner.
    type Made = (Arc<DisplacedTerrain>, Arc<HeightGrid>, (f64, f64));
    let mut outer: Option<Made> = None;
    let mut hole = None;
    for k in (0..n).rev() {
        let cell = finest * (1u64 << k) as f64;
        let h = if n == 1 { (reach / cell).ceil() * cell } else { half(k) };
        let (min, max) = ((cx - h, cz - h), (cx + h, cz + h));
        let surface = Surface { relief, frame, local: &local, clipmap: &clipmap, cell };
        let joined = Joined { surface: &surface, outer: outer.as_ref().map(|o| &*o.1), min, max, band: 3. * cell };
        let grid = Arc::new(HeightGrid::sample(&joined, min, max, cell));
        let mut displacement = SceneFile::parse_displacement(d, &po);
        let falloff = if k == n - 1 {
            // The rings are coarsest at the outermost patch's corners.
            let coarsest = [min, (min.0, max.1), (max.0, min.1), max].iter().map(|&(x, z)| {
                let (tx, tz) = frame.tangent(&frame.on_planet(&local.to_scene(&Vector3::new(x, 0., z))).0);
                clipmap.local(tx, tz).unwrap_or(reach)
            }).fold(cell, f64::max);
            number(o, "displacement_falloff", 0.).max(3. * coarsest)
        } else { 0. };
        displacement.falloff = falloff;
        // The middle half of each but the finest is left to the next one in.
        let inner = (k > 0).then(|| { let q = (grid.nx / 4, 3 * grid.nx / 4); (q.0, q.1, q.0, q.1) });
        let nest = outer.as_ref().map(|(o, _, omin)| Nest {
            outer: o.clone(),
            offset: (((min.0 - omin.0) / cell).round() as i64, ((min.1 - omin.1) / cell).round() as i64),
        });
        let terrain = DisplacedTerrain::of(grid.clone(), displacement, inner, nest)
            .unwrap_or_else(|e| panic!("{}", e))
            .placed(local.origin, local.axes)
            .with_fields(ground.clone());
        let terrain = Arc::new(terrain);
        parts.push(Box::new(crate::shapes::transform::Transform::new(terrain.clone() as Arc<dyn Geometry + Sync + Send>, placement)));
        if k == n - 1 {
            hole = Some(Hole { local: local.clone(), min, max, surface: grid.clone() });
        }
        say(format!("Landscape: displaced to {:.0} m about the camera on a {} x {} grid of {:.2} m{}",
            h, grid.nx, grid.nz, grid.cell, if falloff > 0. { format!(", fading over {:.0} m", falloff) } else { String::new() }));
        outer = Some((terrain, grid, min));
    }
    say(format!("Landscape: {} displaced patches in {:.1?}", n, begun.elapsed()));
    (hole.unwrap(), parts)
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
        let obj = create_terrain(&o, &serde_json::Map::new());
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
        let obj = create_terrain(&o, &serde_json::Map::new());
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
