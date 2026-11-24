use crate::camera;
use crate::na::{Matrix3, Vector3, Vector2};
use crate::scenegraph::SceneGraph;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::plane::Plane;
use crate::shapes::mesh::{Mesh, SmoothMesh};
use crate::shapes::csg::{Csg, Op, Inverse, Clipped, Tagged as PartTagged};
use crate::shapes::transform::{Transform};
use crate::na::{Matrix4, Rotation3, Translation3, Affine3};
use crate::ocean::create_ocean;
use crate::shapes::bbox::BBox;
use crate::light::{Light, Sun};
use crate::color::Color;
use crate::spectrum::{RefractiveIndex, Emission};
use crate::skysphere::{create_sky_sphere, parse_sky_material};
use crate::procedural::box_terrain::create_box_terrain;
use crate::procedural::fireworks::create_firework;
use crate::procedural::tree;
use crate::terrain::object::create_terrain;
use crate::procedural::grass::{create_grass, GrassMedium};
use crate::procedural::heightmap::{HeightGrid, Terrain};
use crate::procedural::rock::{self, Bedrock, Assorted};
use crate::material::rock::{Rock, RockStyle};
use crate::material::soil::{Soil, SoilStyle};
use crate::procedural::nubis_clouds::{NubisCloudLayer, GlobalNubis};
use crate::procedural::hogan_kew_cirrus::{HoganKewCirrus, CirrusField, CirrusGenus, CirrusSpecies, GlobalCirrus};
use crate::procedural::benard_altocumulus::BenardAltocumulus;
use crate::procedural::lenticular::{Lenticular, MoistLayer};
use crate::procedural::terrain::{Terrain as Mountains, TerrainMaterial};
use crate::material::snow::Snow;
use crate::procedural::mammatus::Mammatus;
use crate::procedural::precipitation::Precipitation;
use crate::procedural::overcast::Overcast;
use crate::procedural::hill_cloud::HillCloud;
use crate::material::rainbow::RainbowTable;
use crate::procedural::harris_cumulus::{CloudSimulation, Forcing, HarrisCumulus, Profile, SimulationParams, Sounding};
use crate::participatingmedia::create_fog;
use crate::volume::grid::{Grid, GridDensity};
use crate::light::Profile as LightProfile;
use crate::textures::Image;
use crate::volume::emission::VolumeEmission;
use crate::volume::{Volume, create_medium, create_volume, DensityField, ConstantDensity, NoiseDensity, ScatteringApproximation, Oz, Diffraction, Halos, Scattering};
use crate::material::ice_halos::{Crystals, Habit};
use crate::material::diffraction::DiffractionTable;
use crate::noise::PerlinNoise;
use crate::planet::{Planet, PlanetMapping, GlobalMapping, EARTH_RADIUS};
use crate::material::phase::PhaseFunction;
use std::sync::Arc;
use crate::sceneobject::SceneObject;
use serde_json::{Value, Map};
use crate::scene::{Scene, ImageOpts, RenderOpts, PathTermination, Background, Filter, FilterShape, TabulatedFilter, Sampler};
use crate::output::OutputOpts;
use std::io::prelude::*;
use std::fs::File;
use crate::material::model::MaterialModel;
use crate::material::texture::{Solid, CheckeredYPlane, Medium, NoiseMedium, PartMedia, self};
use crate::material::specular::Specular;
use crate::material::dielectric::Dielectric;
use crate::material::plastic::Plastic;
use crate::material::leaf::{Leaf, LeafColor, DEFAULT_AUTUMN, DEFAULT_DEAD};
use crate::material::lambertian::Lambertian;
use crate::material::normal::NormalShade;
use crate::material::legacy::{ Whitted, FlatColor };
use crate::material::diffuse_light::DiffuseLight;
use crate::material::bump::{Bump, Bumped, Pattern};
use crate::material::noise::{NoiseTexture, NoiseType};
use crate::participatingmedia::{ParticipatingMedium, HomogenousFog, Vacuum};
use crate::shapes::geometry::{Geometry, Tagged};
use crate::shapes::geometry_set::GeometrySet;
use std::cell::RefCell;
use std::collections::HashMap;
use rayon::prelude::*;

/// A geometry that may be shared: by several objects, or by instancing transforms.
pub type SharedGeometry = Arc<dyn Geometry + Sync + Send>;

/// An object's fields (a landscape's simulation), kept with the object, for graphs elsewhere in
/// the scene to read (a volume's density).
struct ObjectFields(Arc<SceneObject>);

/// Objects made by placing things, and where each stands with the radius it claims.
pub(crate) type Placements = (Vec<Arc<SceneObject>>, Vec<(Vector3<f64>, f64)>);

/// What things placed by rules stand on (a forest, a scatter, grass): geometries, and maybe the
/// scene's landscape object, with its fields for the rules to read.
pub(crate) struct Footing<'a> {
    refs: Vec<&'a dyn Geometry>,
    fields: Option<&'a dyn crate::graph::Fields>,
    top: f64,
}

impl<'a> Footing<'a> {
    pub(crate) fn new(surfaces: &'a [SharedGeometry], landscape: Option<&'a SceneObject>) -> Footing<'a> {
        let mut refs: Vec<&dyn Geometry> = surfaces.iter().map(|s| &**s as &dyn Geometry).collect();
        if let Some(l) = landscape {
            refs.push(&*l.geometry);
        }
        let top = refs.iter().map(|s| { let y = s.bounds().max.y; if y.is_finite() && y < 1e5 { y + 1. } else { 1e5 } })
            .fold(f64::NEG_INFINITY, f64::max);
        Footing { refs, fields: landscape.and_then(|l| l.medium.fields()), top }
    }

    /// The ground under (x, z), and its normal, facing up.
    pub(crate) fn at(&self, x: f64, z: f64) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let down = crate::ray::Ray { ro: Vector3::new(x, self.top, z), rd: -Vector3::y() };
        let hit = self.refs.iter().filter_map(|s| s.intersects(&down)).min_by(|a, b| a.dist.total_cmp(&b.dist))?;
        Some((hit.point, if hit.normal.y < 0. { -hit.normal } else { hit.normal }.normalize()))
    }

    /// A rule (a graph, 0 to 1) at the ground under (x, z); 0 off the ground.
    pub(crate) fn rule(&self, g: &crate::graph::Graph, x: f64, z: f64) -> f64 {
        self.at(x, z).map_or(0., |(p, n)| g.eval(&crate::graph::Context::new(p, n, 0.).with_fields(self.fields)).clamp(0., 1.))
    }

    /// Where `o` stands: its `"surface"` (a named or inline geometry, or a list of them; the
    /// plane y = 0 by default), or `"landscape"`, the scene's landscape object before it.
    pub(crate) fn surfaces(o: &Value, geometries: &GeometryLibrary, landscape: Option<&'a SceneObject>, what: &str)
        -> (Vec<SharedGeometry>, Option<&'a SceneObject>)
    {
        let surface = |g: &Value| SceneFile::parse_shared_geometry(g, geometries)
            .unwrap_or_else(|| panic!("{} surface {} is not a geometry", what, g));
        match &o["surface"] {
            Value::Null => (vec![Arc::new(Plane { y: 0. })], None),
            Value::String(n) if n == "landscape" && !geometries.defines("landscape") => (vec![], Some(landscape
                .unwrap_or_else(|| panic!("a {} on the \"landscape\" must come after a landscape object", what)))),
            Value::Array(list) => (list.iter().map(surface).collect(), None),
            g => (vec![surface(g)], None),
        }
    }

    /// The rule `o` is placed by: its `"where"` (or, for a scatter, `"density"`).
    pub(crate) fn parse_rule(o: &Value, what: &str) -> Option<crate::graph::Graph> {
        o.get("where").map(|d| crate::graph::Graph::parse(d).unwrap_or_else(|e| panic!("{}'s \"where\": {}", what, e)))
    }
}

impl crate::graph::Fields for ObjectFields {
    fn field(&self, f: crate::graph::Field, p: &Vector3<f64>) -> Option<f64> {
        self.0.medium.fields()?.field(f, p)
    }

    fn vertical(&self, p: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> {
        self.0.medium.fields()?.vertical(p)
    }

    fn biome(&self, p: &Vector3<f64>) -> Option<crate::terrain::climate::Biome> {
        self.0.medium.fields()?.biome(p)
    }
}
/// A part of a tree, as detail to make levels of, and its medium (`SceneFile::tree_details`).
pub type TreeDetail = (crate::shapes::lod::Detail, Box<dyn Medium + Sync + Send>);

/// The scene file's named geometries, `"geometries": {"dragon": {"type": "mesh", ...}}`. Wherever
/// a geometry is expected, the string `"dragon"` refers to one. Each is built once, when first
/// referred to, and shared by every reference, so `{"type": "transform", "item": "dragon",
/// "translate": [...]}` places another dragon for the price of a transform: object instancing.
/// Definitions may refer to each other, but not in a cycle.
///
/// It also holds the scene's meshes, keyed by file and scale (see `MeshKey`), so a mesh file
/// is loaded once however many times it appears, named or not; `preload` loads them all in
/// parallel before the objects are parsed.
pub struct GeometryLibrary {
    defined: Map<String, Value>,
    built: RefCell<HashMap<String, SharedGeometry>>,
    active: RefCell<Vec<String>>,
    meshes: RefCell<HashMap<MeshKey, SharedGeometry>>,
}

/// A loaded mesh: smooth or flat, its file, and its scale (as bits, to be hashable).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct MeshKey {
    smooth: bool,
    src: String,
    scale: [u64; 3],
}

impl MeshKey {
    /// The key for a `mesh` or `smoothmesh` definition; None for anything else.
    fn of(o: &Value) -> Option<MeshKey> {
        let smooth = match o.get("type")?.as_str()? {
            "mesh" => false,
            "smoothmesh" => true,
            _ => return None,
        };
        let src = SceneFile::parse_string(&o["src"]);
        let scale = SceneFile::parse_vec3_def(o, "scale", Vector3::new(1., 1., 1.));
        Some(MeshKey { smooth, src, scale: [scale.x.to_bits(), scale.y.to_bits(), scale.z.to_bits()] })
    }

    fn load(&self) -> SharedGeometry {
        let scale = Vector3::from(self.scale.map(f64::from_bits));
        if self.smooth {
            Arc::new(SmoothMesh::from_file(self.src.clone(), scale))
        } else {
            Arc::new(Mesh::from_file(self.src.clone(), scale))
        }
    }
}

impl GeometryLibrary {
    pub fn new(defined: Map<String, Value>) -> GeometryLibrary {
        GeometryLibrary {
            defined,
            built: RefCell::new(HashMap::new()),
            active: RefCell::new(Vec::new()),
            meshes: RefCell::new(HashMap::new()),
        }
    }

    /// The mesh a `mesh` or `smoothmesh` definition describes, loaded on first use.
    pub fn mesh(&self, o: &Value) -> SharedGeometry {
        let key = MeshKey::of(o).unwrap_or_else(|| panic!("not a mesh: {}", o));
        if let Some(m) = self.meshes.borrow().get(&key) {
            return m.clone();
        }
        let m = key.load();
        self.meshes.borrow_mut().insert(key, m.clone());
        m
    }

    /// Load, in parallel, every mesh that `objects` use, directly or through named geometries.
    /// Loading (parsing the file, building its BVH) is most of the work of reading a scene,
    /// and meshes are independent of each other. Named geometries nothing uses aren't loaded.
    pub fn preload(&self, objects: &[Value]) {
        fn collect(lib: &GeometryLibrary, v: &Value, keys: &mut Vec<MeshKey>, names: &mut Vec<String>) {
            match v {
                Value::Object(fields) => {
                    if let Some(k) = MeshKey::of(v) && !keys.contains(&k) {
                        keys.push(k)
                    }
                    for f in fields.values() { collect(lib, f, keys, names) }
                },
                Value::Array(items) => for i in items { collect(lib, i, keys, names) },
                // Any string naming a geometry is followed: at worst, a material that shares a
                // geometry's name loads a mesh early.
                Value::String(name) => if let Some(def) = lib.defined.get(name) && !names.contains(name) {
                    names.push(name.clone());
                    collect(lib, def, keys, names);
                },
                _ => {},
            }
        }
        let (mut keys, mut names) = (Vec::new(), Vec::new());
        for o in objects { collect(self, o, &mut keys, &mut names) }
        keys.retain(|k| !self.meshes.borrow().contains_key(k));
        let loaded: Vec<(MeshKey, SharedGeometry)> = keys.into_par_iter().map(|k| { let m = k.load(); (k, m) }).collect();
        self.meshes.borrow_mut().extend(loaded);
    }

    pub fn empty() -> GeometryLibrary {
        GeometryLibrary::new(Map::new())
    }

    /// Whether a geometry of that name is defined.
    pub fn defines(&self, name: &str) -> bool {
        self.defined.contains_key(name)
    }

    /// The named geometry, built on first use.
    pub fn get(&self, name: &str) -> SharedGeometry {
        if let Some(g) = self.built.borrow().get(name) {
            return g.clone();
        }
        if self.active.borrow().iter().any(|a| a == name) {
            panic!("geometries refer to each other in a cycle: {} -> {}", self.active.borrow().join(" -> "), name);
        }
        let def = self.defined.get(name)
            .unwrap_or_else(|| panic!("unknown geometry \"{}\": define it in \"geometries\"", name));
        self.active.borrow_mut().push(name.to_string());
        let g = SceneFile::parse_shared_geometry(def, self)
            .unwrap_or_else(|| panic!("geometry \"{}\" is not a geometry: {}", name, def));
        self.active.borrow_mut().pop();
        self.built.borrow_mut().insert(name.to_string(), g.clone());
        g
    }
}

static KEEP_OBJECTS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static KEPT_OBJECTS: std::sync::Mutex<Option<HashMap<String, Vec<Arc<SceneObject>>>>> = std::sync::Mutex::new(None);

/// Command-line settings that take precedence over the scene file.
#[derive(Default)]
pub struct Overrides {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub seed: Option<u64>,
}

/// Every field except `camera` is optional; defaults are applied in `from_scenefile`.
#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    #[serde(default)]
    pub width: Value,
    #[serde(default)]
    pub height: Value,
    #[serde(default)]
    pub chunk_size: Value,
    #[serde(default)]
    pub supersamples: Value, 
    #[serde(default)]
    pub samples_per_chunk: Value, 
    #[serde(default)]
    pub adaptive: Value,
    pub camera: Value, 
    #[serde(default)]
    pub shadow_bias: Value, 
    #[serde(default)]
    pub black_threshold: Value,
    #[serde(default)]
    pub path_termination: Value,
    #[serde(default)]
    pub seed: Value,
    #[serde(default)]
    pub sample_lights: Value,
    #[serde(default)]
    pub light_sampler: Value,
    #[serde(default)]
    pub integrator: Value,
    #[serde(default)]
    pub sample_sky: Value,
    #[serde(default)]
    pub sampler: Value,
    #[serde(default)]
    pub regularise: Value,
    #[serde(default)]
    pub spectral: Value,
    #[serde(default)]
    pub shutter: Value,
    #[serde(default)]
    pub filter: Value,
    #[serde(default)]
    pub output: Value,
    #[serde(default)]
    pub russian_roulette_depth: Value,
    #[serde(default)]
    pub background: Value,
    #[serde(default)]
    pub max_depth: Value,
    #[serde(default)]
    pub materials: Map<String, Value>,
    #[serde(default)]
    pub media: Map<String, Value>,
    #[serde(default)]
    pub lights: Vec<Value>,
    #[serde(default)]
    pub sun: Value,
    #[serde(default)]
    pub objects: Vec<Value>,
    #[serde(default)]
    pub geometries: Map<String, Value>,
    #[serde(default)]
    pub variables: Value,
    pub air: Option<Value>,
}

/// While parsing an object, the media of its parts that have their own (see `csg::Tagged`), and
/// the scene's materials and media to find them in (none when parsing a shape alone).
/// Also the scene's named geometries, for references to them.
pub struct Parts<'a> {
    library: Option<(&'a Map<String, Value>, &'a Map<String, Value>)>,
    media: Vec<Box<dyn Medium + Sync + Send>>,
    geometries: &'a GeometryLibrary,
}

impl SceneFile {
    pub fn parse_number(v: &Value, default: f64) -> f64 {
        match v.as_f64(){
            Some(x) => x,
            None => default
        }
    }
    
    /// The reconstruction filter (`scene::Filter`): `"box"` (the default), `"tent"`,
    /// `"gaussian"`, `"mitchell"` or `"lanczos"`, or an object with `"type"` and parameters:
    /// `"radius"` in pixels (defaults 1.5, 2 and 3), and `"sigma"` (0.5), `"b"` and `"c"` (1/3
    /// each), or `"tau"` (3).
    pub fn parse_filter(v: &Value) -> Filter {
        let (name, o) = match v {
            Value::Null => ("box", &Value::Null),
            Value::String(s) => (s.as_str(), &Value::Null),
            Value::Object(_) => (v["type"].as_str().unwrap_or_else(|| panic!("A filter object needs a \"type\", got {}", v)), v),
            _ => panic!("Unexpected filter {}", v),
        };
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let tabulated = |shape, radius| Filter::Tabulated(Arc::new(TabulatedFilter::new(shape, radius)));
        match name {
            "box" => Filter::Box,
            "tent" => Filter::Tent,
            "gaussian" => tabulated(FilterShape::Gaussian { sigma: n("sigma", 0.5) }, n("radius", 1.5)),
            "mitchell" => tabulated(FilterShape::Mitchell { b: n("b", 1. / 3.), c: n("c", 1. / 3.) }, n("radius", 2.)),
            "lanczos" => tabulated(FilterShape::Lanczos { tau: n("tau", 3.) }, n("radius", 3.)),
            f => panic!("Unknown filter '{}': expected \"box\", \"tent\", \"gaussian\", \"mitchell\" or \"lanczos\"", f),
        }
    }

    /// A refractive index (`spectrum::RefractiveIndex`): a number; a named glass, `"BK7"`,
    /// `"fused silica"`, `"SF11"` or `"diamond"` (or `{"glass": name}`); `{"cauchy": [a, b]}`,
    /// n = a + b / λ² with λ in µm; or `{"sellmeier": {"b": [b1, b2, b3], "c": [c1, c2, c3]}}`,
    /// c in µm². 1 if absent. Indices that vary with wavelength disperse light in spectral mode
    /// and take their value at 587.56 nm otherwise.
    pub fn parse_refractive_index(v: &Value) -> RefractiveIndex {
        let glass = |name: &str| RefractiveIndex::glass(name)
            .unwrap_or_else(|| panic!("Unknown glass \"{}\": expected BK7, fused silica, SF11 or diamond", name));
        let three = |v: &Value, what: &str| -> [f64; 3] {
            match v.as_array().map(|a| a.iter().map(|x| x.as_f64()).collect::<Option<Vec<f64>>>()) {
                Some(Some(a)) if a.len() <= 3 => { let mut out = [0.; 3]; out[..a.len()].copy_from_slice(&a); out },
                _ => panic!("Sellmeier \"{}\" must be up to three numbers, got {}", what, v),
            }
        };
        match v {
            Value::Null => RefractiveIndex::Constant(1.),
            Value::Number(n) => RefractiveIndex::Constant(n.as_f64().unwrap()),
            Value::String(name) => glass(name),
            Value::Object(o) => if let Some(name) = o.get("glass").and_then(|g| g.as_str()) {
                glass(name)
            } else if let Some(c) = o.get("cauchy") {
                match c.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect::<Vec<f64>>()) {
                    Some(a) if a.len() == 2 => RefractiveIndex::Cauchy { a: a[0], b: a[1] },
                    _ => panic!("\"cauchy\" must be [a, b], got {}", c),
                }
            } else if let Some(sm) = o.get("sellmeier") {
                RefractiveIndex::Sellmeier { b: three(&sm["b"], "b"), c: three(&sm["c"], "c") }
            } else {
                panic!("A refractive index object needs \"glass\", \"cauchy\" or \"sellmeier\", got {}", v)
            },
            _ => panic!("Unexpected refractive index {}", v),
        }
    }

    pub fn parse_int(v: &Value, default: usize) -> usize {
        if let Some(x) = v.as_u64() {
            return x as usize;
        }
        match v.as_f64() {
            // Accept whole numbers written as floats, eg. 640.0
            Some(x) if x >= 0. && x.fract() == 0. => x as usize,
            Some(x) => panic!("Expected a non-negative integer, got {}", x),
            None => default
        }
    }
    
    pub fn parse_string(v: &Value) -> String {
        v.as_str().unwrap().to_string()// This is pretty nasty, shame serde
    }

    pub fn parse_vec3(v: &Value) -> Vector3<f64> {
        Vector3::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap())
    }

    pub fn parse_vec3_def(v: &Value, k: &str, def: Vector3<f64>) -> Vector3<f64> {
        match &v.get(k) {
            Some(x) => SceneFile::parse_vec3(x),
            None => def
        }
    }

    pub fn parse_vec2(v: &Value) -> Vector2<f64> {
        Vector2::new(v[0].as_f64().unwrap(),
                            v[1].as_f64().unwrap())
    }

    pub fn parse_vec2_def(v: &Value, k: &str, def: Vector2<f64>) -> Vector2<f64> {
        match &v.get(k) {
            Some(x) => SceneFile::parse_vec2(x),
            None => def
        }
    }

    /// Parse an [r, g, b] array, or a named colour.
    pub fn parse_color(v: &Value) -> Color {
        if let Some(name) = v.as_str() {
            return match name {
                "black" => Color::black(),
                "white" => Color::white(),
                "red" => Color::red(),
                "green" => Color::green(),
                "blue" => Color::blue(),
                _ => panic!("Unknown colour name: {}", name),
            };
        }
        Color::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap())
    
    }


    /// A colour, or an image read by the surface's texture coordinates (`textures`):
    /// `{"texture": "file.png", "scale": [su, sv] or s, "offset": [ou, ov], "filter":
    /// "ewa" (the default) | "trilinear" | "bilinear" | "point", "wrap": "repeat" | "clamp" | "black", "linear":
    /// bool}` (8-bit images are taken as sRGB unless `linear`).
    /// A colour, an image texture (`{"texture": path, ...}`), or a grey level from the slope
    /// (`{"slope": [from, to], "jitter": degrees, "jitter_scale": size, "seed": 0}`,
    /// `textures::SlopeTexture`), or from the elevation (`{"height": [from, to], ...}`).
    pub fn parse_color_texture(v: &Value) -> crate::textures::ColorTexture {
        // A function graph's node (`graph.rs`): a grey level, eg. a mix's amount by slope.
        if v.get("node").is_some() {
            return crate::textures::ColorTexture::Graph(Arc::new(crate::graph::Graph::parse(v)
                .unwrap_or_else(|e| panic!("texture graph: {}", e))));
        }
        // {"terrain": "washes" | "hollows" | "joints", "scale": the heightmap's floor_scale or
        // domes, "spread": 1}: where a point lies among a mesas heightmap's features.
        if let Some(feature) = v.get("terrain") {
            use crate::textures::TerrainFeature;
            return crate::textures::ColorTexture::Terrain(crate::textures::TerrainTexture {
                feature: match feature.as_str() {
                    Some("washes") => TerrainFeature::Washes,
                    Some("hollows") => TerrainFeature::Hollows,
                    Some("joints") => TerrainFeature::Joints,
                    _ => panic!("terrain texture must be \"washes\", \"hollows\" or \"joints\", got {}", feature),
                },
                scale: SceneFile::parse_number(&v["scale"], 150.),
                spread: SceneFile::parse_number(&v["spread"], 1.),
                noise: crate::noise::PerlinNoise::new(),
            });
        }
        if let Some(range) = v.get("slope").or(v.get("height")) {
            let r = SceneFile::parse_vec2(range);
            return crate::textures::ColorTexture::Slope(crate::textures::SlopeTexture {
                by_height: v.get("slope").is_none(),
                from: r.x, to: r.y,
                jitter: SceneFile::parse_number(&v["jitter"], 0.),
                jitter_scale: SceneFile::parse_number(&v["jitter_scale"], 1.),
                noise: crate::noise::SeededNoise::new(SceneFile::parse_int(&v["seed"], 0) as u64),
            });
        }
        match v.get("texture") {
            Some(_) => crate::textures::ColorTexture::Image(SceneFile::parse_image_map(v)),
            None => SceneFile::parse_color(v).into(),
        }
    }

    pub fn parse_image_map(v: &Value) -> crate::textures::ImageMap {
        use crate::textures::{Filter, Image, ImageMap, Wrap};
        let path = v["texture"].as_str().unwrap_or_else(|| panic!("A texture needs a file name, got {}", v["texture"]));
        let pair = |k: &str, d: f64| match &v[k] {
            Value::Number(n) => Vector2::repeat(n.as_f64().unwrap()),
            Value::Null => Vector2::repeat(d),
            x => SceneFile::parse_vec2(x),
        };
        ImageMap {
            image: Image::shared(path, v["linear"].as_bool()),
            scale: pair("scale", 1.),
            offset: pair("offset", 0.),
            filter: match v["filter"].as_str() {
                Some("trilinear") => Filter::Trilinear,
                None | Some("ewa") => Filter::Ewa,
                Some("bilinear") => Filter::Bilinear,
                Some("point") => Filter::Point,
                Some(f) => panic!("Unknown texture filter '{}': expected ewa, trilinear, bilinear or point", f),
            },
            wrap: match v["wrap"].as_str() {
                None | Some("repeat") => Wrap::Repeat,
                Some("clamp") => Wrap::Clamp,
                Some("black") => Wrap::Black,
                Some(w) => panic!("Unknown texture wrap '{}': expected repeat, clamp or black", w),
            },
        }
    }

    pub fn parse_color_def(v: &Value, k: &str, def: Color) -> Color {
        match &v.get(k) {
            Some(x) => SceneFile::parse_color(x),
            None => def
        }
    }

    /// The camera (`camera`): `"location"`, `"lookat"` and `"up"`, and by `"type"`:
    /// - `"perspective"` (the default): `"angle"`, with tan(angle) the image's height at unit
    ///   distance; a thin lens of radius `"aperture"` (default 0.2), focused at the point looked
    ///   at or `"focus_distance"`; `"astigmatism": {"cylinder": power, "axis": degrees,
    ///   "seidel": power}` (`camera::Astigmatism`), powers in reciprocal scene units.
    /// - `"orthographic"`: parallel rays from a rectangle `"height"` tall (by default what the
    ///   perspective camera sees at the point looked at); through a thin lens of radius
    ///   `"aperture"` (default 0, all in focus) focused at the point looked at or
    ///   `"focus_distance"`, which blurs by aperture × |depth - focus| / focus; or by `"blur"`
    ///   × |depth - focus| (the aperture `"blur"` × focus), the same however far back it is.
    /// - `"isometric"`: an orthographic camera looking at `"lookat"` from `"azimuth"` degrees
    ///   round `"up"` (default 45, from x towards z) and `"elevation"` degrees above the ground
    ///   (default `camera::ISOMETRIC_ELEVATION`, 35.26°, true isometric; 30° gives pixel art's
    ///   2:1 diamonds), a rectangle `"height"` tall, from `"distance"` back (default the distance
    ///   of `"location"`, which is optional and otherwise sets nothing, or twice the height), and
    ///   the orthographic camera's `"aperture"`; see `SceneFile::axonometric`.
    /// - `"spherical"`: every direction, `"mapping": "equirectangular"` (the default) or
    ///   `"equal_area"`.
    /// - `"realistic"`: rays traced through a lens (`lens`): `"lens"`, a built-in name
    ///   (`"dgauss.50mm"`, the default, `"wide.22mm"`, `"telephoto.250mm"`, `"fisheye.10mm"`) or
    ///   a pbrt `.dat` file; `"f_number"` (default wide open); `"focus_distance"` (default the
    ///   point looked at); `"film_diagonal"` in mm (default 35); `"blades"` and `"blade_rotation"`
    ///   (degrees) for a polygonal stop. The lens sets the field of view.
    ///
    /// With `"motion": {"location", "lookat", "up"}`, the camera moves to that pose (each key
    /// defaulting to the start's) while the shutter is open.
    pub fn parse_camera(c: Value, width: u32, height: u32) -> Box<dyn camera::Camera + Sync> {
        let c = SceneFile::axonometric(c);
        let location = SceneFile::parse_vec3(&c["location"]);
        let lookat = SceneFile::parse_vec3(&c["lookat"]);
        let up = SceneFile::parse_vec3(&c["up"]);
        let start = camera::Frame::look_at(location, lookat, up);
        let placement = match c.get("motion") {
            None => camera::Placement::still(start),
            Some(m) => camera::Placement::moving(start, camera::Frame::look_at(
                SceneFile::parse_vec3_def(m, "location", location),
                SceneFile::parse_vec3_def(m, "lookat", lookat),
                SceneFile::parse_vec3_def(m, "up", up))),
        };
        let aspect = width as f64 / height as f64;
        let angle = || c["angle"].as_f64().unwrap_or_else(|| panic!("The camera needs an \"angle\""));
        match c["type"].as_str() {
            None | Some("perspective") => {
                let mut cam = camera::ThinLensCamera::new(lookat, location, up, angle(), width, height,
                    SceneFile::parse_number(&c["aperture"], 0.2)).with_placement(placement);
                if let Some(d) = c["focus_distance"].as_f64() {
                    cam = cam.with_focus(d);
                }
                if let Some(a) = c.get("astigmatism") {
                    cam = cam.with_astigmatism(camera::Astigmatism {
                        cylinder: SceneFile::parse_number(&a["cylinder"], 0.),
                        axis: SceneFile::parse_number(&a["axis"], 0.).to_radians(),
                        seidel: SceneFile::parse_number(&a["seidel"], 0.),
                    });
                }
                Box::new(cam)
            },
            Some("orthographic") => {
                let h = c["height"].as_f64().unwrap_or_else(|| angle().tan() * (lookat - location).norm());
                let focus = c["focus_distance"].as_f64().unwrap_or((lookat - location).norm());
                let aperture = match (c["blur"].as_f64(), c["aperture"].as_f64()) {
                    (Some(_), Some(_)) => panic!("An orthographic camera takes \"blur\" or \"aperture\", not both"),
                    (Some(b), None) => b * focus,
                    (None, a) => a.unwrap_or(0.),
                };
                Box::new(camera::OrthographicCamera::new(placement, h, aspect).with_lens(aperture, focus))
            },
            Some("spherical") => Box::new(camera::SphericalCamera::new(placement, match c["mapping"].as_str() {
                None | Some("equirectangular") => camera::SphereMapping::Equirectangular,
                Some("equal_area") => camera::SphereMapping::EqualArea,
                Some(m) => panic!("Unknown spherical mapping '{}': expected \"equirectangular\" or \"equal_area\"", m),
            })),
            Some("realistic") => {
                let lens = SceneFile::parse_lens(c["lens"].as_str().unwrap_or("dgauss.50mm"), &c);
                let focus = c["focus_distance"].as_f64().unwrap_or((lookat - location).norm());
                let cam = crate::lens::RealisticCamera::new(placement, lens, focus, c["f_number"].as_f64(),
                    SceneFile::parse_number(&c["film_diagonal"], 35.) * 1e-3, aspect)
                    .unwrap_or_else(|e| panic!("Realistic camera: {}", e));
                Box::new(cam)
            },
            Some(t) => panic!("Unknown camera type '{}': expected \"perspective\", \"orthographic\", \"isometric\", \"spherical\" or \"realistic\"", t),
        }
    }

    /// An `"isometric"` camera as the orthographic camera it is, its `"location"` placed by its
    /// azimuth, elevation and distance (`camera::isometric_direction`), so everything that
    /// reads the camera's position (the sky, level of detail, lightning) sees where it is. Any
    /// other camera is returned as it is.
    pub fn axonometric(mut c: Value) -> Value {
        if c["type"].as_str() != Some("isometric") {
            return c;
        }
        let lookat = SceneFile::parse_vec3(&c["lookat"]);
        let up = SceneFile::parse_vec3_def(&c, "up", Vector3::y());
        let height = c["height"].as_f64().unwrap_or_else(|| panic!("An isometric camera needs a \"height\": the height of the view, in scene units"));
        let distance = match (c["distance"].as_f64(), c.get("location")) {
            (Some(d), _) => d,
            (None, Some(l)) => (SceneFile::parse_vec3(l) - lookat).norm(),
            (None, None) => 2. * height,
        };
        assert!(distance > 0., "An isometric camera's distance from what it looks at must be positive");
        let back = camera::isometric_direction(up, SceneFile::parse_number(&c["azimuth"], 45.),
            SceneFile::parse_number(&c["elevation"], camera::ISOMETRIC_ELEVATION)) * distance;
        let json = |v: Vector3<f64>| serde_json::json!([v.x, v.y, v.z]);
        c["type"] = serde_json::json!("orthographic");
        c["location"] = json(lookat + back);
        c["up"] = json(up);
        if let Some(m) = c.get_mut("motion") {
            // It slides, keeping its direction.
            let end = SceneFile::parse_vec3_def(m, "lookat", lookat);
            m["lookat"] = json(end);
            m["location"] = json(end + back);
            m["up"] = json(up);
        }
        c
    }

    /// A lens (`lens::LensSystem`): a built-in name or a pbrt `.dat` file, with `"blades"` (and
    /// `"blade_rotation"`, degrees) from `o` if given.
    pub fn parse_lens(name: &str, o: &Value) -> crate::lens::LensSystem {
        let lens = if !name.contains('/') && !name.ends_with(".dat") {
            crate::lens::LensSystem::named(name)
                .unwrap_or_else(|| panic!("Unknown lens '{}': expected dgauss.50mm, wide.22mm, telephoto.250mm, fisheye.10mm or a .dat file", name))
        } else {
            crate::lens::LensSystem::parse(&crate::assets::read(name).map_err(|e| e.to_string()).and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
                    .unwrap_or_else(|e| panic!("Can't read lens {}: {}", name, e)))
                .unwrap_or_else(|e| panic!("Lens {}: {}", name, e))
        };
        match o["blades"].as_u64() {
            Some(n) => lens.with_blades(n as usize, SceneFile::parse_number(&o["blade_rotation"], 0.).to_radians()),
            None => lens,
        }
    }

    /// The output's `"flare"` (`flare`): `true`, or `{"lens", "f_number", "blades",
    /// "blade_rotation", "threshold": 20, "sources": 8, "ghosts": 1, "starburst": 1}`. With a
    /// realistic camera, its lens, f-number, blades, focus and film are the defaults; otherwise
    /// the double Gauss (or for views wider than 20° up from the centre, the wide-angle lens)
    /// wide open, focused far off, on a film matching the camera's view. A light outside the
    /// lens's field makes no ghosts.
    /// `ghosts` and `starburst` scale each (0 or false for none).
    pub fn parse_flare(output: &Value, camera: &Value, width: usize, height: usize) -> Option<Arc<crate::flare::Flare>> {
        let f = match output.get("flare") {
            None | Some(Value::Bool(false)) => return None,
            Some(Value::Bool(true)) => &Value::Null,
            Some(f) => f,
        };
        let realistic = camera["type"].as_str() == Some("realistic");
        let pick = |k: &str| if !f[k].is_null() { f[k].clone() } else if realistic { camera[k].clone() } else { Value::Null };
        let blade_source = serde_json::json!({ "blades": pick("blades"), "blade_rotation": pick("blade_rotation") });
        // A lens that covers the view: the double Gauss reaches about 22° from its axis, so wider
        // views (more than 20° up from the centre) take the wide-angle lens.
        let default_lens = if !realistic && camera["angle"].as_f64().is_some_and(|a| a.tan() / 2. > 20f64.to_radians().tan()) {
            "wide.22mm"
        } else {
            "dgauss.50mm"
        };
        let lens = SceneFile::parse_lens(pick("lens").as_str().unwrap_or(default_lens), &blade_source);
        let focal = lens.focal_length().unwrap_or_else(|| panic!("The flare's lens doesn't form an image"));
        // Its blades are cut to round the stop fully open (see `flare`).
        let open_stop = lens.stop_diameter().unwrap_or(0.) / 2.;
        // A real diaphragm: blades curved to round the open stop (`blade_curvature`, 0 for
        // straight), each a little out of place (`imperfection`, 0 for a perfect lens).
        let imperfection = SceneFile::parse_number(&f["imperfection"], 1.).max(0.);
        let seed = SceneFile::parse_int(&f["seed"], 1) as u64;
        let curvature = SceneFile::parse_number(&f["blade_curvature"], 1.).max(0.);
        let lens = match lens.blades {
            Some((n, _)) => lens.with_blade_shape(crate::lens::BladeShape {
                radius: if curvature > 0. { open_stop / curvature } else { 0. },
                jitter: if imperfection > 0. { crate::flare::imperfect_blades(n, imperfection, seed) } else { vec![] },
            }),
            None => lens,
        };
        let lens = match pick("f_number").as_f64() { Some(n) => lens.with_stop(focal / n), None => lens };
        let focus = if realistic {
            camera["focus_distance"].as_f64().unwrap_or_else(|| (SceneFile::parse_vec3(&camera["lookat"]) - SceneFile::parse_vec3(&camera["location"])).norm())
        } else {
            1e4
        };
        let rear = lens.focus(focus).or_else(|| lens.focus(1e4)).unwrap_or_else(|| panic!("The flare's lens can't focus"));
        let lens = lens.with_film_at(rear);
        let aspect = width as f64 / height as f64;
        let film_height = if realistic {
            SceneFile::parse_number(&camera["film_diagonal"], 35.) * 1e-3 / (1. + aspect * aspect).sqrt()
        } else {
            focal * camera["angle"].as_f64().unwrap_or(0.5).tan()
        };
        let scale = |k: &str| match &f[k] { Value::Bool(b) => if *b { 1. } else { 0. }, v => SceneFile::parse_number(v, 1.) };
        Some(Arc::new(crate::flare::Flare {
            lens,
            film: (film_height * aspect, film_height),
            threshold: SceneFile::parse_number(&f["threshold"], 20.),
            sources: SceneFile::parse_int(&f["sources"], 8),
            ghosts: scale("ghosts"),
            starburst: scale("starburst"),
            grid: SceneFile::parse_int(&f["grid"], 192).max(4),
            coating: match f["coating"].as_str() {
                None => crate::flare::Coating::Multi,
                Some(c) => crate::flare::Coating::parse(c)
                    .unwrap_or_else(|| panic!("Unknown lens coating {:?}: one of \"multi\", \"single\" or \"none\"", c)),
            },
            imperfection,
            seed,
        }))
    }

    /// Whether a scene value (the camera, an object, a geometry) moves: holds a `"motion"`.
    fn has_motion(v: &Value) -> bool {
        match v {
            Value::Object(o) => o.contains_key("motion") || o.values().any(SceneFile::has_motion),
            Value::Array(a) => a.iter().any(SceneFile::has_motion),
            _ => false,
        }
    }

    /// `"shutter": [open, close]`, in the time of motion, where moving things are at their start
    /// at 0 and their end at 1 (default [0, 1]).
    /// `"adaptive"`: a noise threshold, or `{"threshold", "min_samples"}`; absent, false or a
    /// threshold of 0 for none.
    pub fn parse_adaptive(v: &Value) -> Option<crate::scene::Adaptive> {
        let (threshold, min) = match v {
            Value::Null | Value::Bool(false) => return None,
            Value::Bool(true) => (0.01, &Value::Null),
            Value::Object(o) => (SceneFile::parse_number(&v["threshold"], 0.01), o.get("min_samples").unwrap_or(&Value::Null)),
            _ => (SceneFile::parse_number(v, 0.), &Value::Null),
        };
        (threshold > 0.).then(|| crate::scene::Adaptive {
            threshold,
            min_samples: SceneFile::parse_int(min, crate::scene::Adaptive::DEFAULT_MIN_SAMPLES).max(2),
        })
    }

    pub fn parse_shutter(v: &Value) -> crate::motion::Shutter {
        match v {
            Value::Null => crate::motion::Shutter::default(),
            Value::Array(a) if a.len() == 2 && a.iter().all(|x| x.is_number()) => {
                let (open, close) = (a[0].as_f64().unwrap(), a[1].as_f64().unwrap());
                assert!(close >= open, "The shutter must close after it opens, got {}", v);
                crate::motion::Shutter { open, close }
            },
            _ => panic!("\"shutter\" must be [open, close], got {}", v),
        }
    }

    pub fn parse_objects(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary) ->Vec<Arc<SceneObject>> {
        SceneFile::parse_objects_indexed(objs, materials, media, geometries).into_iter().map(|(_, o)| o).collect()
    }

    /// The objects, each with the index in `objs` of the entry it came from. An entry may give
    /// no object, or several (a tree, a forest).
    pub fn parse_objects_indexed(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary) -> Vec<(usize, Arc<SceneObject>)> {
         let mut objects: Vec<(usize, Arc<SceneObject>)> = Vec::new();
         // Valley fog finds the ground among the other objects, so is built after them.
         let fogs: Vec<(usize, Value)> = objs.iter().enumerate().filter(|(_, o)| o["type"] == "valley_fog").map(|(i, o)| (i, o.clone())).collect();
         // Hill cloud joins the first overcast deck, mapped as it is, to be lit with it.
         let deck = objs.iter().position(|o| o["type"] == "overcast");
         let flat = |o: &Value| o["mapping"].as_str() == Some("flat") || o["curved"].as_bool() == Some(false);
         let joins = |o: &Value| deck.is_some_and(|d| o["type"] == "mountains" && o.get("cloud").is_some()
             && o["cloud"]["join"].as_bool() != Some(false) && flat(o) == flat(&objs[d]));
         let mut hills: Vec<HillCloud> = objs.iter().filter(|o| joins(o)).filter_map(SceneFile::hill_cloud_field).collect();
         let joined: Vec<bool> = objs.iter().map(joins).collect();
         let mut landscape: Option<Arc<SceneObject>> = None;
         // Ice floats on the scene's ocean, on its waves.
         let ocean = objs.iter().find(|o| o["type"] == "ocean").cloned();
         // Objects kept from the last build (see `keep_objects_between_builds`), and those to
         // keep from this one. Not those built from other entries too (a deck with the hills'
         // cloud, mountains raising theirs): their key, their own description, doesn't say all
         // they are made of.
         let reuse = KEEP_OBJECTS.load(std::sync::atomic::Ordering::Relaxed);
         let previous = if reuse { KEPT_OBJECTS.lock().unwrap().take().unwrap_or_default() } else { HashMap::new() };
         let mut kept: HashMap<String, Vec<Arc<SceneObject>>> = HashMap::new();
         for (i, mut obj) in objs.into_iter().enumerate() {
            if obj["type"] == "valley_fog" {
                continue;
            }
            if let (Some(ocean), true) = (&ocean, obj["type"] == "iceberg" || obj["type"] == "ice_floes") {
                if obj.get("ocean").is_none() { obj["ocean"] = ocean.clone(); }
            }
            // (Nor what grows on, or reads, the landscape before it: forests, grass, ecosystems,
            // scatters and volumes over one.)
            let on_landscape = landscape.is_some() && matches!(obj["type"].as_str(), Some("forest" | "grass" | "ecosystem" | "scatter" | "volume"));
            let composite = (Some(i) == deck && !hills.is_empty()) || (obj["type"] == "mountains" && obj.get("cloud").is_some()) || on_landscape;
            let reusable = reuse && !composite;
            let key = if reusable { SceneFile::object_key(&obj, materials, media, geometries) } else { String::new() };
            if reusable && let Some(built) = previous.get(&key).or(kept.get(&key)) {
                if obj["type"] == "landscape" {
                    landscape = built.first().cloned();
                }
                objects.extend(built.iter().map(|x| (i, x.clone())));
                kept.insert(key, built.clone());
                continue;
            }
            // Report the objects that are slow to build (meshes, oceans), by type and source.
            let start = web_time::Instant::now();
            let name = format!("{}{}", obj["type"].as_str().unwrap_or("?"),
                obj["src"].as_str().or(obj["item"]["src"].as_str()).map_or(String::new(), |s| format!(" {}", s)));
            let built: Vec<Arc<SceneObject>> = if obj["type"] == "tree" {
                SceneFile::parse_tree(&obj, materials, media)
            } else if obj["type"] == "forest" {
                SceneFile::parse_forest(&obj, materials, media, geometries, landscape.as_deref()).into_iter().collect()
            } else if obj["type"] == "grass" && (obj.get("where").is_some() || matches!(obj["heightmap"].as_str(), Some("landscape" | "surface"))) {
                vec![Arc::new(SceneFile::parse_ruled_grass(&obj, materials, media, geometries, landscape.as_deref()))]
            } else if obj["type"] == "ecosystem" {
                SceneFile::parse_ecosystem(&obj, materials, media, geometries, landscape.as_deref()).into_iter().collect()
            } else if obj["type"] == "scatter" {
                SceneFile::parse_scatter(&obj, materials, media, geometries, landscape.as_deref()).into_iter().collect()
            } else if obj["type"] == "volume" && matches!(obj.get("density"), Some(Value::Object(_) | Value::String(_))) {
                let fields = landscape.clone().map(|l| Arc::new(ObjectFields(l)) as Arc<dyn crate::graph::Fields>);
                vec![Arc::new(SceneFile::parse_volume_over(&obj, geometries, fields))]
            } else if obj["type"] == "landscape" {
                // Kept, as a surface for scatters after it.
                let x: Vec<Arc<SceneObject>> = SceneFile::parse_object(obj, materials, media, geometries).into_iter().collect();
                if let Some(first) = x.first() {
                    landscape = Some(first.clone());
                }
                x
            } else if Some(i) == deck && !hills.is_empty() {
                vec![Arc::new(SceneFile::parse_overcast_with(&obj, std::mem::take(&mut hills)))]
            } else if obj["type"] == "mountains" && obj.get("cloud").is_some() && !joined[i] {
                // The hills, and the cloud they raise.
                let mut v: Vec<Arc<SceneObject>> = SceneFile::parse_hill_cloud(&obj).map(Arc::new).into_iter().collect();
                v.extend(SceneFile::parse_object(obj, materials, media, geometries));
                v
            } else {
                SceneFile::parse_object(obj, materials, media, geometries).into_iter().collect()
            };
            objects.extend(built.iter().map(|x| (i, x.clone())));
            if reusable { kept.insert(key, built); }
            let t = start.elapsed().as_secs_f64();
            if t >= 0.1 {
                SceneFile::say(format!("- Built {} in {:.2} s", name, t));
            }
         }
         for (i, fog) in fogs {
            let start = web_time::Instant::now();
            let ground: Vec<Arc<SceneObject>> = objects.iter().map(|(_, o)| o.clone()).collect();
            objects.push((i, Arc::new(SceneFile::parse_valley_fog(&fog, &ground))));
            SceneFile::say(format!("- Built valley_fog in {:.2} s", start.elapsed().as_secs_f64()));
         }
         if reuse { *KEPT_OBJECTS.lock().unwrap() = Some(kept); }
         objects
    }

    /// Everything an object is built from, as text: its own description, and the definition
    /// of every material, medium and named geometry it refers to, and of those they refer to.
    /// Two objects with the same key are the same object.
    ///
    /// References are found by name: any string in the description that names a material, a
    /// medium or a geometry counts. A string that only happens to match a name makes the key
    /// depend on more than it need, which costs a rebuild and nothing else.
    fn object_key(obj: &Value, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary) -> String {
        fn collect<'a>(v: &'a Value, tables: &[(&str, &'a Map<String, Value>)], seen: &mut Vec<(usize, &'a str)>, key: &mut String) {
            match v {
                Value::String(name) => for (t, (label, table)) in tables.iter().enumerate() {
                    if let Some((name, def)) = table.get_key_value(name.as_str()) && !seen.contains(&(t, name.as_str())) {
                        seen.push((t, name.as_str()));
                        key.push_str(&format!("\n{} {}={}", label, name, def));
                        collect(def, tables, seen, key);
                    }
                },
                Value::Array(items) => for i in items { collect(i, tables, seen, key) },
                Value::Object(fields) => for f in fields.values() { collect(f, tables, seen, key) },
                _ => {},
            }
        }
        let mut key = obj.to_string();
        let tables = [("material", materials), ("medium", media), ("geometry", &geometries.defined)];
        collect(obj, &tables, &mut Vec::new(), &mut key);
        key
    }

    /// Keep each built object between builds (in this process), and reuse it when the next
    /// scene has an object with the same description and the same definitions behind it.
    ///
    /// For an editor, which rebuilds the scene on every change: only what changed is built
    /// again, so moving a sphere doesn't regrow the grass. Objects are immutable once built,
    /// so sharing one between scenes is safe. Off by default: a single render gains nothing,
    /// and the objects of the last scene would be held in memory for no reason.
    pub fn keep_objects_between_builds(keep: bool) {
        KEEP_OBJECTS.store(keep, std::sync::atomic::Ordering::Relaxed);
        if !keep { *KEPT_OBJECTS.lock().unwrap() = None; }
    }

    /// Fog pooled in the valleys (see `procedural::valley_fog`), over the ground the scene's
    /// other objects make, found by casting rays straight down on a grid:
    ///
    /// `{"type": "valley_fog", "center": [x, z], "extent": 10000, "resolution": 512,
    /// "pool_size": 1500, "depth": 150, "density": 0.02, "seed": 0}`
    ///
    /// The cold air pools over the lowest ground within `pool_size` metres (about a valley's
    /// width), and fills `depth` metres above it, up to `top` (an inversion's altitude, if
    /// given): `depth` and `density` (extinction per metre) may be function graphs
    /// (`crate::graph`). The top fades over `softness` m, heaves in billows `top_variation` m
    /// high and `billow_size` m apart, and is frayed by `raggedness` m of detail `detail_size`
    /// across; `variation` (0 to 1) makes it patchy. `haze` (extinction) thins above it over
    /// `haze_height` m. Without `center` and `extent`, the fog covers the solid objects of
    /// bounded size. Scattering (`albedo`, `g`, approximations) as for any volume.
    pub fn parse_valley_fog(o: &Value, objects: &[Arc<SceneObject>]) -> SceneObject {
        use crate::procedural::valley_fog::ValleyFog;
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        // The ground: solid objects (not media, not the sky) of bounded size, and those
        // bigger, such as the planet, beneath them.
        let solid: Vec<&Arc<SceneObject>> = objects.iter().filter(|x| !x.geometry.is_stochastic()).collect();
        let bounded: Vec<BBox> = solid.iter().map(|x| x.bounds())
            .filter(|b| (b.max - b.min).iter().all(|e| e.is_finite() && *e < 1e6)).collect();
        let (center, extent) = match (o.get("center"), o.get("extent")) {
            (Some(_), Some(_)) => (SceneFile::parse_vec2_def(o, "center", Vector2::zeros()), num("extent", 1.).max(1.)),
            _ => {
                let lo = bounded.iter().fold(Vector3::repeat(f64::INFINITY), |a, b| a.inf(&b.min.coords));
                let hi = bounded.iter().fold(Vector3::repeat(f64::NEG_INFINITY), |a, b| a.sup(&b.max.coords));
                assert!(lo.x.is_finite(), "valley_fog needs a \"center\" and \"extent\", or ground of bounded size to cover");
                (Vector2::new(0.5 * (lo.x + hi.x), 0.5 * (lo.z + hi.z)), (0.5 * (hi.x - lo.x)).max(0.5 * (hi.z - lo.z)).max(1.))
            },
        };
        let high = bounded.iter().map(|b| b.max.y).fold(0., f64::max) + 10.;
        let low = bounded.iter().map(|b| b.min.y).fold(0., f64::min) - 10_000.;
        let ground = |x: f64, z: f64| -> Option<f64> {
            let r = crate::ray::Ray { ro: Vector3::new(x, high, z), rd: Vector3::new(0., -1., 0.) };
            solid.iter().filter_map(|s| s.intersects(&r)).map(|i| i.dist)
                .filter(|d| *d <= high - low).fold(None, |a: Option<f64>, d| Some(a.map_or(d, |a| a.min(d))))
                .map(|d| high - d)
        };
        let resolution = SceneFile::parse_int(&o["resolution"], 512).max(2);
        let mut f = ValleyFog::new(center, extent, resolution, num("pool_size", 1500.).max(1.), SceneFile::parse_seed(o).unwrap_or(0), ground);
        let graph = |k: &str, what: &str| o.get(k).map(|v| crate::graph::Graph::parse(v).unwrap_or_else(|e| panic!("valley_fog's {}: {}", what, e)));
        if let Some(g) = graph("density", "density") {
            f.density = g;
        }
        if let Some(g) = graph("depth", "depth") {
            f.depth = g;
        }
        f.top = num("top", f.top);
        f.softness = num("softness", f.softness).max(0.);
        f.top_variation = num("top_variation", f.top_variation).max(0.);
        f.billow_size = num("billow_size", f.billow_size).max(1.);
        f.raggedness = num("raggedness", f.raggedness).max(0.);
        f.detail_size = num("detail_size", f.detail_size).max(1.);
        f.variation = num("variation", f.variation).clamp(0., 1.);
        f.haze = num("haze", f.haze).max(0.);
        f.haze_height = num("haze_height", f.haze_height).max(1.);
        let bounds = BBox::new(
            Vector3::new(center.x - extent, f.bottom() - 1., center.y - extent),
            Vector3::new(center.x + extent, f.ceiling(), center.y + extent),
        );
        create_volume(bounds, Arc::new(f), SceneFile::parse_scattering(o, 0.85))
    }

    /// An object's medium, bump mapped if its `"normal"` is an object (see `parse_bump`). A
    /// plane's `"normal"` may instead be a vector, its orientation (`parse_plane`).
    pub fn parse_object_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<dyn Medium + Sync + Send> {
        let m = SceneFile::parse_unbumped_medium(o, materials, media);
        if !o["normal"].is_object() {
            return m;
        }
        // {"map": "normals.png", "strength": 1, and an image map's "scale", "offset", ...}: a
        // tangent-space normal map, read linearly (`textures::NormalMap`).
        if let Some(path) = o["normal"].get("map") {
            let mut spec = o["normal"].clone();
            spec["texture"] = path.clone();
            if spec.get("linear").is_none() {
                spec["linear"] = Value::Bool(true);
            }
            let map = crate::textures::NormalMap { map: SceneFile::parse_image_map(&spec), strength: SceneFile::parse_number(&o["normal"]["strength"], 1.) };
            return Box::new(crate::material::texture::NormalMapped { inner: m, map });
        }
        Box::new(Bumped { inner: m, bump: SceneFile::parse_bump(&o["normal"]) })
    }

    /// `{"pattern": "bumps" | "dents" | "wrinkles" | "ripples" | "waves", "amount": 0.5,
    /// "scale": 1 or [x, y, z], "octaves": 6 (wrinkles), "frequency": 1, "phase": 0 (ripples
    /// and waves, phase in cycles)}`.
    pub fn parse_bump(n: &Value) -> Bump {
        let pattern = match n["pattern"].as_str() {
            Some("bumps") => Pattern::Bumps,
            Some("dents") => Pattern::Dents,
            Some("wrinkles") => Pattern::Wrinkles { octaves: SceneFile::parse_int(&n["octaves"], 6) as u32 },
            Some("ripples") => Pattern::Ripples {
                frequency: SceneFile::parse_number(&n["frequency"], 1.), phase: SceneFile::parse_number(&n["phase"], 0.) },
            Some("waves") => Pattern::Waves {
                frequency: SceneFile::parse_number(&n["frequency"], 1.), phase: SceneFile::parse_number(&n["phase"], 0.) },
            _ => panic!("normal \"pattern\" must be \"bumps\", \"dents\", \"wrinkles\", \"ripples\" or \"waves\", got {}", n["pattern"]),
        };
        let scale = match &n["scale"] {
            Value::Number(s) => Vector3::repeat(s.as_f64().unwrap()),
            Value::Null => Vector3::repeat(1.),
            v => SceneFile::parse_vec3(v),
        };
        assert!(scale.iter().all(|s| *s != 0.), "normal scale must not be zero");
        Bump { pattern, amount: SceneFile::parse_number(&n["amount"], 0.5), scale }
    }

    fn parse_unbumped_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<dyn Medium + Sync + Send> {
        match &o.get("medium") {
            Some(mid) => {
                SceneFile::parse_medium_ref(mid, materials, media).unwrap()
            },
            None => {
                // Check if material is specified
                if let Some(material_key) = o.get("material") {
                    let m = SceneFile::require_material(material_key, materials, &format!("object {}", o["type"]));
                    Box::new(Solid { m })
                } else {
                    // No material or medium specified
                    let default_material = Box::new(Lambertian { albedo: Color::white().into() });
                    Box::new(Solid { m: default_material })
                }
            }
        }
    }

    pub fn parse_object(o: Value,  materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary) -> Option<Arc<SceneObject>> {
        let t = o["type"].as_str().unwrap();

        if t == "skysphere" {
            return Some(Arc::new(SceneFile::parse_skysphere(&o)));
        }

        if t == "box_terrain" {
            return Some(Arc::new(SceneFile::parse_box_terrain(&o)));
        }
        if t == "ocean" {
            let mut d = create_ocean(&o);
            // Any material may replace the ocean's own, eg. a dielectric to see into the water.
            if o.get("material").is_some() || o.get("medium").is_some() {
                d.medium = SceneFile::parse_object_medium(&o, materials, media);
            }
            return Some(Arc::new(d));
        }
        if t == "landscape" {
            return Some(Arc::new(create_terrain(&o, materials)));
        }
        if t == "firework" {
            let d = create_firework(&o);
            return Some(Arc::new(d));
        }

        if t == "grass" {
            let (geometry, conditions) = create_grass(&o);
            let medium: Box<dyn Medium + Sync + Send> = if o.get("material").is_some() || o.get("medium").is_some() {
                SceneFile::parse_object_medium(&o, materials, media)
            } else {
                Box::new(GrassMedium::new(geometry.field.clone(), &conditions))
            };
            return Some(Arc::new(SceneObject { geometry: Box::new(geometry), medium }));
        }
        if t == "iceberg" || t == "ice_floes" {
            return Some(Arc::new(SceneFile::parse_ice_object(&o, materials, media)));
        }
        if t == "boulder" || t == "pebbles" || t == "bedrock" {
            return Some(Arc::new(SceneFile::parse_rock_object(&o, materials, media)));
        }

        if t == "fog" {
            let f = create_fog(&o);
            return Some(Arc::new(f));
        }

        if t == "planet" {
            // The ground as a sphere, so that what is far away drops below the horizon.
            let planet = Planet { radius: SceneFile::parse_number(&o["radius"], EARTH_RADIUS) };
            return Some(Arc::new(SceneObject {
                geometry: crate::planet::ground(&planet),
                medium: SceneFile::parse_object_medium(&o, materials, media),
            }));
        }

        if t == "cumulus" {
            return Some(Arc::new(SceneFile::parse_cumulus(&o)));
        }
        if t == "weather_layer" {
            return Some(Arc::new(crate::weather::scene::create_layer(&o)));
        }
        if t == "weather" {
            panic!("A weather object is expanded into its layers before parsing (`weather::scene::expand`)");
        }

        if t == "moon" {
            return Some(Arc::new(crate::celestial::moon::parse_moon(&o)));
        }

        if t == "gas_giant" {
            return Some(Arc::new(crate::celestial::giant::parse_giant(&o)));
        }

        if t == "hogan_kew_cirrus" {
            return Some(Arc::new(SceneFile::parse_hogan_kew_cirrus(&o)));
        }

        if t == "harris_cumulus" {
            return Some(Arc::new(SceneFile::parse_harris_cumulus(&o)));
        }

        if t == "precipitation" {
            return Some(Arc::new(SceneFile::parse_precipitation(&o)));
        }

        if t == "mammatus" {
            return Some(Arc::new(SceneFile::parse_mammatus(&o)));
        }

        if t == "fractus" {
            return Some(Arc::new(SceneFile::parse_fractus(&o)));
        }

        if t == "shelf_cloud" || t == "roll_cloud" {
            return Some(Arc::new(SceneFile::parse_arcus(&o, t == "roll_cloud")));
        }

        if t == "cumulonimbus" {
            return Some(Arc::new(SceneFile::parse_cumulonimbus(&o)));
        }

        if t == "lightning" {
            return Some(Arc::new(crate::procedural::lightning::parse_lightning(&o)));
        }

        if t == "overcast" {
            return Some(Arc::new(SceneFile::parse_overcast(&o)));
        }

        if t == "mountains" {
            return Some(Arc::new(SceneFile::parse_terrain(&o)));
        }

        if t == "lenticular" {
            return Some(Arc::new(SceneFile::parse_lenticular(&o)));
        }

        if t == "benard_altocumulus" {
            return Some(Arc::new(SceneFile::parse_benard_altocumulus(&o)));
        }

        if t == "nubis_clouds" {
            return Some(Arc::new(SceneFile::parse_nubis_clouds(&o)));
        }

        if t == "volume" {
            return Some(Arc::new(SceneFile::parse_volume(&o, geometries)));
        }

        let mut parts = Parts { library: Some((materials, media)), media: vec![], geometries };
        let geom = SceneFile::parse_geometry_in(&o, &mut parts, false);
        let mut m = SceneFile::parse_object_medium(&o, materials, media);
        if !parts.media.is_empty() {
            m = Box::new(PartMedia { base: m, parts: parts.media });
        }
        if let Some(geometry) = geom {
            return Some(Arc::new(SceneObject {
                geometry,
                medium: m
            }));
        }
        
        if t == "checkeredplane" {
            return Some(Arc::new(SceneFile::parse_checkeredplane(&o, m)));
        }
        panic!("Unknown object type \"{}\"", t);
        //return None
    }
    /// `{"type": "tree", "species": "quaking-aspen", "seed": 0, "location": [x, y, z],
    /// "bark": name, "dead_bark": name, "leaves": name or [names], "params": {...}}`: a
    /// procedural tree (see `crate::procedural::tree`), as objects for its bark, its dead
    /// branches' bark (default: as "bark") and its leaves. `params` overrides the species'
    /// parameters. Each name is a material or, failing that, a medium (eg. a noise texture).
    /// Given several leaf names, each leaf takes one at random, chosen by the seed, so a canopy
    /// needn't be a single colour. Leaves of the `leaf` material also take their own autumn or
    /// dead colour, by the tree's `autumn` and `dead_branches` parameters; other materials
    /// ignore them. Without materials, the bark is grey-brown and the leaves green.
    pub fn parse_tree(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>) -> Vec<Arc<SceneObject>> {
        let species = o.get("species").map_or("quaking-aspen".to_string(), SceneFile::parse_string);
        let p = tree::params(&species, &o["params"]);
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let location = SceneFile::parse_vec3_def(o, "location", Vector3::zeros());
        SceneFile::tree_parts(o, &p, seed, location, materials, media).into_iter()
            .map(|(geometry, medium)| Arc::new(SceneObject { geometry: Box::new(geometry), medium }))
            .collect()
    }

    /// `{"type": "forest", "seed": 0, "surface": geometry or [geometries], "area": {"min": [x, z],
    /// "max": [x, z]}, "spacing": 8, "clearings": 0.15, "patch_size": 40, "patchiness": 0.5,
    /// "max_slope": 35, "variants": 3, "autumn": 0.4, "autumn_spread": 0.1, "params": {...},
    /// "species": [{"species": "scots-pine", "weight": 1, "spacing": 1, "scale_v": 0.2,
    /// "bark": name, "dead_bark": name, "leaves": name or [names], "params": {...}}, ...]}`
    ///
    /// Trees over `area` of the surface (a named or inline geometry, or several, the trees
    /// standing on the highest; default the plane y = 0; or `"landscape"`, the scene's landscape
    /// object before it), placed by `crate::procedural::forest`. `"where"` is a rule for where
    /// they grow and how thickly: a graph from 0 to 1 read on the ground with its normal
    /// (`slope`, `altitude`, `aspect`, noise, and on a landscape its fields: below the
    /// `treeline`, where it's `wetness`...).
    /// Each species grows `variants` distinct trees, each placed many times by instancing
    /// transforms, so memory goes with the distinct trees, not the forest's size. The top-level
    /// `params` apply to every species, then each species' own. Given `autumn`, each distinct
    /// tree is that far through autumn, give or take `autumn_spread`. The surface itself isn't
    /// drawn: add it as an object too.
    pub fn parse_forest(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                        geometries: &GeometryLibrary, landscape: Option<&SceneObject>) -> Vec<Arc<SceneObject>> {
        SceneFile::forest_in(o, materials, media, geometries, landscape, None).0
    }

    /// As `parse_forest`, growing only as `room` (0 to 1 at (x, z)) leaves room for; and where
    /// each tree stands and the radius it keeps clear (half its share of `spacing`).
    pub(crate) fn forest_in(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                        geometries: &GeometryLibrary, landscape: Option<&SceneObject>,
                        room: Option<&dyn Fn(f64, f64) -> f64>) -> Placements {
        use crate::procedural::forest::{self, ForestLayout, ForestSpecies};
        let (surfaces, on) = Footing::surfaces(o, geometries, landscape, "forest");
        let footing = Footing::new(&surfaces, on);
        let rule = Footing::parse_rule(o, "forest");
        let layout = ForestLayout {
            min: SceneFile::parse_vec2(&o["area"]["min"]),
            max: SceneFile::parse_vec2(&o["area"]["max"]),
            spacing: SceneFile::parse_number(&o["spacing"], 8.),
            clearings: SceneFile::parse_number(&o["clearings"], 0.15),
            patch_size: SceneFile::parse_number(&o["patch_size"], 40.),
            patchiness: SceneFile::parse_number(&o["patchiness"], 0.5),
            max_slope: SceneFile::parse_number(&o["max_slope"], 35.),
            variants: SceneFile::parse_int(&o["variants"], 3).max(1),
        };
        let entries = o["species"].as_array().filter(|e| !e.is_empty())
            .unwrap_or_else(|| panic!("a forest needs a list of \"species\""));
        let species: Vec<ForestSpecies> = entries.iter().map(|e| ForestSpecies {
            weight: SceneFile::parse_number(&e["weight"], 1.),
            spacing: SceneFile::parse_number(&e["spacing"], 1.),
            scale_v: SceneFile::parse_number(&e["scale_v"], 0.2),
        }).collect();
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let refs = footing.refs.clone();
        // {"favour": {"terrain": "washes", "scale": 150, "spread": 1, "strength": 0.8}}: trees
        // crowd along a mesas heightmap's feature (`parse_color_texture`), `strength` of them
        // kept only there.
        let feature = o.get("favour").map(|f| {
            let feature = match SceneFile::parse_color_texture(f) {
                crate::textures::ColorTexture::Terrain(t) => t,
                _ => panic!("a forest's \"favour\" must be a terrain texture, got {}", f),
            };
            (feature, SceneFile::parse_number(&f["strength"], 0.8).clamp(0., 1.))
        });
        // "where": a rule (a graph) for where trees grow and how thickly, read on the ground.
        let placements = if feature.is_none() && rule.is_none() && room.is_none() {
            forest::place(&layout, &species, seed, &refs)
        } else {
            forest::place_favouring(&layout, &species, seed, &refs, &|x, z| {
                let favoured = feature.as_ref().map_or(1., |(f, strength)| 1. - strength + strength * f.amount(&Vector3::new(x, 0., z)))
                    * room.map_or(1., |r| r(x, z));
                if favoured <= 0. { return 0. }
                favoured * rule.as_ref().map_or(1., |g| footing.rule(g, x, z))
            })
        };
        let claims: Vec<(Vector3<f64>, f64)> = placements.iter()
            .map(|p| (p.position, 0.5 * layout.spacing * species[p.species].spacing * p.scale)).collect();

        let spread = SceneFile::parse_number(&o["autumn_spread"], 0.1);
        let mut rng = crate::geometry::Pcg32::new(seed, 0x7a71);
        let lod = SceneFile::parse_forest_lod(o);

        // The distinct trees: each species' variants that are placed, each with its own
        // numbers, drawn for every variant, placed or not, so each keeps its own.
        let mut distinct: Vec<(usize, Vec<&forest::Placement>, tree::TreeParams, u64)> = vec![];
        for (k, entry) in entries.iter().enumerate() {
            let name = entry.get("species").map(SceneFile::parse_string)
                .unwrap_or_else(|| panic!("forest species {} needs a \"species\"", entry));
            for v in 0 .. layout.variants {
                let (autumn_shift, tree_seed) = (rng.next_f64() * 2. - 1., rng.next_u32() as u64);
                let here: Vec<&forest::Placement> = placements.iter().filter(|p| p.species == k && p.variant == v).collect();
                if here.is_empty() {
                    continue;
                }
                let mut overrides = if o["params"].is_object() { o["params"].clone() } else { serde_json::json!({}) };
                if let Some(autumn) = o["autumn"].as_f64() {
                    overrides["autumn"] = serde_json::json!((autumn + autumn_shift * spread).clamp(0., 1.));
                }
                tree::merge(&mut overrides, &entry["params"]);
                distinct.push((k, here, tree::params(&name, &overrides), tree_seed));
            }
        }

        // Grow them one at a time, each made at the levels of detail its placements need (in
        // parallel), so only one tree is ever held in full: what's kept is far smaller.
        // Each placement's level, position, yaw and scale; each part's geometry by level.
        type Place = (usize, Vector3<f64>, f64, f64, bool);
        type Levels = Vec<(usize, SharedGeometry)>;
        type Grown = (Vec<(Levels, Box<dyn Medium + Sync + Send>)>, Vec<Place>);
        let grown: Vec<Grown> = distinct.iter().map(|(k, here, p, tree_seed)| {
            let entry = &entries[*k];
            // On a slope, sink the trunk so its downhill side meets the ground.
            let base_radius = p.scale * p.stems[0].length * p.ratio * p.trunk_scale * (1. + 0.99 * p.flare);
            let (details, base, bounds) = SceneFile::tree_details(entry, p, *tree_seed, Vector3::zeros(), materials, media);
            let places: Vec<Place> = here.iter().map(|t| {
                let sink = base_radius * t.scale * (t.slope.to_radians().tan() + 0.05);
                let at = t.position - Vector3::new(0., sink, 0.);
                let level = lod.as_ref().map_or(0, |l| l.level(&bounds, at, t.scale, base));
                (level, at, t.yaw, t.scale, t.mirror)
            }).collect();
            let mut levels: Vec<usize> = places.iter().map(|p| p.0).collect();
            levels.sort_unstable();
            levels.dedup();
            let parts = details.into_par_iter().map(|(detail, medium)| {
                let geometries = levels.par_iter().map(|&l| (l, detail.geometry(crate::shapes::lod::level_size(base, l)))).collect();
                (geometries, medium)
            }).collect();
            (parts, places)
        }).collect();

        // Each tree at each level is one model of all its parts, each numbered for its medium,
        // placed as many times as there are trees at that level; the whole forest is one
        // object, so a ray searches one structure and stops at the first tree it meets.
        let mut trees: Vec<forest::Placed> = vec![];
        let mut parts: Vec<Box<dyn Medium + Sync + Send>> = vec![];
        let mut per_level: Vec<usize> = vec![];
        let (mut primitives, mut built) = (0u64, 0u64);
        for (tree_parts, places) in grown {
            let mut models: std::collections::BTreeMap<usize, Vec<Arc<forest::PartOf<SharedGeometry>>>> = Default::default();
            for (geometries, medium) in tree_parts {
                let part = parts.len() as u32;
                assert!(part < 1 << (32 - forest::PART_SHIFT), "too many distinct tree parts in a forest");
                // Detail coarse enough may simplify away, eg. thin dead branches.
                for (level, geometry) in geometries.into_iter().filter(|(_, g)| g.primitives() > 0) {
                    models.entry(level).or_default().push(Arc::new(forest::PartOf { item: geometry, part }));
                }
                parts.push(medium);
            }
            let models: std::collections::BTreeMap<usize, forest::Model> = models.into_iter()
                .map(|(level, items)| (level, Arc::new(GeometrySet::new(&items)) as forest::Model)).collect();
            built += models.values().map(|m| m.primitives()).sum::<u64>();
            for (level, at, yaw, scale, mirror) in places {
                if per_level.len() <= level {
                    per_level.resize(level + 1, 0);
                }
                per_level[level] += 1;
                if let Some(model) = models.get(&level) {
                    primitives += model.primitives();
                    trees.push(forest::Placed::new(model.clone(), at, yaw, scale, mirror));
                }
            }
        }
        println!("Forest: {} trees, from {} distinct; {} primitives built, {} placed{}", placements.len(), distinct.len(), built, primitives,
            if lod.is_some() { format!("; trees at each level of detail: {:?}", per_level) } else { String::new() });
        if trees.is_empty() {
            return (vec![], claims);
        }
        let stand = forest::Stand::new(trees);
        (vec![Arc::new(SceneObject {
            geometry: Box::new(stand),
            medium: Box::new(Solid { m: Box::new(forest::PartsMaterial { parts }) }),
        })], claims)
    }

    /// `{"type": "ecosystem", "seed": 0, "surface": geometry or "landscape", "area": {"min": [x, z],
    /// "max": [x, z]}, "populations": [{"forest": {...}}, {"scatter": {...}}, {"grass": {...}}]}`
    ///
    /// Populations that compete for the ground (`procedural::ecosystem`): each a forest, scatter
    /// or grass as its own object would be, with its own `"where"` rule, placed in the order
    /// listed (largest first), each growing only in the room those before it leave. A tree or a
    /// boulder keeps its `base` clear (a fraction of its spacing radius for trees, 0.25; of its
    /// size for scatters, 0.9) and shades its `canopy` (1.3 of its spacing radius for trees;
    /// none for scatters) by `shade` (0.6 for trees, 0 for scatters), where the populations
    /// after it grow that much less thickly. Each takes the ecosystem's `surface`, `seed` and
    /// `area` unless it has its own; without an `area`, it covers a square `reach` metres out
    /// from under the camera on each side (300 for forests, 150 for scatters, 25 for grass),
    /// thinning out to nothing over its last `fade` (a fraction, 0.4 for grass, 0 else). Grass
    /// grows on the surface itself (`"heightmap": "surface"`), sampled every `cell` metres.
    pub fn parse_ecosystem(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                           geometries: &GeometryLibrary, landscape: Option<&SceneObject>) -> Vec<Arc<SceneObject>> {
        use crate::procedural::ecosystem::{Claim, Occupancy};
        let populations = o["populations"].as_array().filter(|p| !p.is_empty())
            .unwrap_or_else(|| panic!("an ecosystem needs a list of \"populations\""));
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let camera = SceneFile::parse_vec3_def(&o["camera"], "location", Vector3::zeros());
        let mut occupancy = Occupancy::new(1.);
        let mut objects: Vec<Arc<SceneObject>> = vec![];
        for (k, p) in populations.iter().enumerate() {
            let (kind, spec) = ["forest", "scatter", "grass"].iter().find_map(|kind| p.get(*kind).map(|s| (*kind, s)))
                .unwrap_or_else(|| panic!("an ecosystem's population is a \"forest\", \"scatter\" or \"grass\": {}", p));
            let mut spec = spec.clone();
            for key in ["surface", "camera"] {
                if spec.get(key).is_none() && o.get(key).is_some() { spec[key] = o[key].clone(); }
            }
            if spec.get("seed").is_none() { spec["seed"] = Value::from(seed.wrapping_mul(31).wrapping_add(k as u64 + 1)); }
            let given = spec.clone();
            let num = |key: &str, def: f64| SceneFile::parse_number(&given[key], def);
            let reach = SceneFile::parse_number(&spec["reach"], match kind { "forest" => 300., "scatter" => 150., _ => 25. });
            let area = spec.get("area").or(o.get("area")).cloned().unwrap_or_else(|| serde_json::json!({
                "min": [camera.x - reach, camera.z - reach], "max": [camera.x + reach, camera.z + reach] }));
            // About the camera, thinning out over the last `fade` of the reach (0.4 of it for
            // grass, none for the rest), in a circle, so it has no edge to see.
            let about = spec.get("area").is_none() && o.get("area").is_none();
            let fade = if about { num("fade", if kind == "grass" { 0.4 } else { 0. }).clamp(0., 1.) } else { 0. };
            let room = |x: f64, z: f64| {
                let thin = if fade > 0. {
                    let t = (((x - camera.x).hypot(z - camera.z) / reach - (1. - fade)) / fade).clamp(0., 1.);
                    1. - t * t * (3. - 2. * t)
                } else { 1. };
                if thin <= 0. { 0. } else { thin * occupancy.free(x, z) }
            };
            let (made, claims): (Vec<Arc<SceneObject>>, Vec<Claim>) = match kind {
                "forest" => {
                    spec["area"] = area;
                    let (made, placed) = SceneFile::forest_in(&spec, materials, media, geometries, landscape, Some(&room));
                    let (base, canopy, shade) = (num("base", 0.25), num("canopy", 1.3), num("shade", 0.6));
                    (made, placed.iter().map(|(p, r)| Claim { x: p.x, z: p.z, base: base * r, shade_radius: canopy * r, shade }).collect())
                },
                "scatter" => {
                    spec["area"] = area;
                    let (made, placed) = SceneFile::scatter_in(&spec, materials, media, geometries, landscape, Some(&room));
                    let (base, canopy, shade) = (num("base", 0.9), num("canopy", 0.), num("shade", 0.));
                    (made, placed.iter().map(|(p, r)| Claim { x: p.x, z: p.z, base: base * r, shade_radius: canopy * r, shade }).collect())
                },
                _ => {
                    if spec.get("heightmap").is_none() { spec["heightmap"] = Value::from("surface"); }
                    if spec.get("min").is_none() { spec["min"] = area["min"].clone(); spec["max"] = area["max"].clone(); }
                    (vec![Arc::new(SceneFile::grass_in(&spec, materials, media, geometries, landscape, Some(&room)))], vec![])
                },
            };
            println!("Ecosystem: population {} ({}): {} placed", k, kind, claims.len());
            objects.extend(made);
            occupancy.extend(claims);
        }
        objects
    }

    /// `{"type": "scatter", "seed": 0, "surface": geometry or [geometries], "area": {"min": [x, z],
    /// "max": [x, z]}, "spacing": 1, "density": graph, "max_slope": 90, "clearings": 0,
    /// "patch_size": 20, "patchiness": 0.3, "items": [item, ...]}`
    ///
    /// Copies of things strewn over a surface, as Terragen's populations: boulders over a
    /// moraine, stones on scree, logs in a wood. Placed as a forest's trees are
    /// (`procedural::forest::place_favouring`: kept apart by `spacing` in proportion to their
    /// size, items growing in patches by `patchiness`), but only where `density` (a graph,
    /// evaluated on the surface where each would stand, with its normal, so `slope` and noise
    /// masks work; 1 by default) lets them, and as thickly as it says. Each item:
    ///
    /// - `"geometry"`: a named or inline geometry to copy; or `"boulder": {"angularity": 0.5,
    ///   "detail": 4}`, `"variants"` (3) distinct seeded boulders of radius 1;
    /// - `"material"` or `"medium"` (boulders default to a granite `rock`);
    /// - `"weight"` (1), `"spacing"` (1, relative to the scatter's), `"size"`: a scale, or
    ///   `[smallest, largest]` (1);
    /// - `"sink"` (0): how far into the ground, as a fraction of its size; `"align"` (0 to 1,
    ///   default 0.7): how far it leans with the slope, from upright to square to the ground;
    ///   `"tilt"` (degrees, 0): a random lean besides.
    ///
    /// Every copy is turned to a random heading and half are mirrored. Copies cost a transform
    /// each, so a scatter costs memory for its distinct items, not for every copy. The surface
    /// itself isn't drawn: add it as an object too.
    pub fn parse_scatter(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                         geometries: &GeometryLibrary, landscape: Option<&SceneObject>) -> Vec<Arc<SceneObject>> {
        SceneFile::scatter_in(o, materials, media, geometries, landscape, None).0
    }

    /// As `parse_scatter`, only as thickly as `room` (0 to 1 at (x, z)) leaves room for; and
    /// where each copy stands and its size.
    pub(crate) fn scatter_in(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                         geometries: &GeometryLibrary, landscape: Option<&SceneObject>,
                         room: Option<&dyn Fn(f64, f64) -> f64>) -> Placements {
        use crate::procedural::forest::{self, ForestLayout, ForestSpecies};
        let (surfaces, on) = Footing::surfaces(o, geometries, landscape, "scatter");
        let footing = Footing::new(&surfaces, on);
        let refs = footing.refs.clone();
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let entries = o["items"].as_array().filter(|e| !e.is_empty())
            .unwrap_or_else(|| panic!("a scatter needs a list of \"items\""));
        struct Item { models: Vec<SharedGeometry>, size: (f64, f64), sink: f64, align: f64, tilt: f64 }
        let mut media_of: Vec<Box<dyn Medium + Sync + Send>> = vec![];
        let items: Vec<Item> = entries.iter().enumerate().map(|(k, e)| {
            let given = e.get("material").is_some() || e.get("medium").is_some();
            let (models, medium): (Vec<SharedGeometry>, Box<dyn Medium + Sync + Send>) = match (e.get("boulder"), e.get("geometry")) {
                (Some(b), None) => {
                    let n = SceneFile::parse_int(&e["variants"], 3).max(1);
                    let detail = b.get("detail").map(|d| SceneFile::parse_int(d, 4) as u32);
                    let angularity = SceneFile::parse_number(&b["angularity"], 0.5);
                    let models = (0..n).map(|v| Arc::new(crate::shapes::mesh::SmoothMesh::from_indexed(&[rock::boulder_mesh(
                        Vector3::zeros(), 1., angularity, detail, seed.wrapping_mul(31).wrapping_add((k * 101 + v) as u64))]))
                        as SharedGeometry).collect();
                    let medium: Box<dyn Medium + Sync + Send> = if given { SceneFile::parse_object_medium(e, materials, media) }
                        else { Box::new(Solid { m: Box::new(Rock::new(RockStyle::preset("granite"), 1., seed + k as u64)) }) };
                    (models, medium)
                },
                (None, Some(g)) => {
                    assert!(given, "a scatter item {} needs a \"material\" or \"medium\"", e);
                    let g = SceneFile::parse_shared_geometry(g, geometries)
                        .unwrap_or_else(|| panic!("scatter item {} is not a geometry", g));
                    (vec![g], SceneFile::parse_object_medium(e, materials, media))
                },
                _ => panic!("a scatter item needs a \"geometry\" or a \"boulder\", not both: {}", e),
            };
            media_of.push(medium);
            let size = match &e["size"] {
                Value::Array(a) if a.len() == 2 => (SceneFile::parse_number(&a[0], 1.), SceneFile::parse_number(&a[1], 1.)),
                v => { let s = SceneFile::parse_number(v, 1.); (s, s) },
            };
            assert!(size.0 > 0. && size.1 >= size.0, "a scatter item's size must be positive, smallest first: {}", e);
            Item { models, size, sink: SceneFile::parse_number(&e["sink"], 0.),
                align: SceneFile::parse_number(&e["align"], 0.7).clamp(0., 1.),
                tilt: SceneFile::parse_number(&e["tilt"], 0.).to_radians() }
        }).collect();
        // Sizes spread evenly between each item's smallest and largest, kept apart in proportion.
        let species: Vec<ForestSpecies> = entries.iter().zip(&items).map(|(e, item)| ForestSpecies {
            weight: SceneFile::parse_number(&e["weight"], 1.),
            spacing: SceneFile::parse_number(&e["spacing"], 1.) * 0.5 * (item.size.0 + item.size.1),
            scale_v: (item.size.1 - item.size.0) / (item.size.1 + item.size.0),
        }).collect();
        let layout = ForestLayout {
            min: SceneFile::parse_vec2(&o["area"]["min"]),
            max: SceneFile::parse_vec2(&o["area"]["max"]),
            spacing: SceneFile::parse_number(&o["spacing"], 1.),
            clearings: SceneFile::parse_number(&o["clearings"], 0.),
            patch_size: SceneFile::parse_number(&o["patch_size"], 20.),
            patchiness: SceneFile::parse_number(&o["patchiness"], 0.3),
            max_slope: SceneFile::parse_number(&o["max_slope"], 90.),
            variants: items.iter().map(|i| i.models.len()).max().unwrap_or(1),
        };
        let ground = |x: f64, z: f64| footing.at(x, z);
        let density = Footing::parse_rule(o, "scatter").or_else(|| o.get("density")
            .map(|d| crate::graph::Graph::parse(d).unwrap_or_else(|e| panic!("scatter's density: {}", e))));
        let favour = |x: f64, z: f64| {
            let left = room.map_or(1., |r| r(x, z));
            if left <= 0. { 0. } else { left * density.as_ref().map_or(1., |g| footing.rule(g, x, z)) }
        };
        let placements = forest::place_favouring(&layout, &species, seed, &refs, &favour);

        let claims: Vec<(Vector3<f64>, f64)> = placements.iter()
            .map(|p| (p.position, 0.5 * (items[p.species].size.0 + items[p.species].size.1) * p.scale)).collect();
        let mut rng = crate::geometry::Pcg32::new(seed, 0x5ca7);
        let copies: Vec<Arc<forest::PartOf<crate::shapes::transform::Transform>>> = placements.iter().map(|p| {
            let item = &items[p.species];
            let model = item.models[p.variant % item.models.len()].clone();
            let mid = 0.5 * (item.size.0 + item.size.1);
            let size = mid * p.scale;
            let normal = ground(p.position.x, p.position.z).map_or(Vector3::y(), |(_, n)| n);
            // Leaning with the slope as far as `align` says, then by a random tilt besides.
            let up = Vector3::y().lerp(&normal, item.align).normalize();
            let lean = rng.next_f64() * item.tilt;
            let around = rng.next_f64() * std::f64::consts::TAU;
            let side = up.cross(&Vector3::new(around.cos(), 0., around.sin())).try_normalize(1e-9).unwrap_or(Vector3::x());
            let up = Rotation3::from_axis_angle(&crate::na::Unit::new_normalize(side), lean) * up;
            let stand = Rotation3::rotation_between(&Vector3::y(), &up).unwrap_or_else(Rotation3::identity);
            let turn = Rotation3::from_axis_angle(&crate::na::Unit::new_normalize(up), p.yaw.to_radians());
            let mirror = if p.mirror { -1. } else { 1. };
            let at = p.position - up * (item.sink * size);
            let m = Translation3::from(at).to_homogeneous() * (turn * stand).to_homogeneous()
                * Matrix4::new_nonuniform_scaling(&Vector3::new(mirror * size, size, size));
            Arc::new(forest::PartOf { item: crate::shapes::transform::Transform::new(model, Affine3::from_matrix_unchecked(m)), part: p.species as u32 })
        }).collect();
        println!("Scatter: {} copies of {} distinct items", copies.len(), items.iter().map(|i| i.models.len()).sum::<usize>());
        if copies.is_empty() {
            return (vec![], claims);
        }
        (vec![Arc::new(SceneObject {
            geometry: Box::new(GeometrySet::new(&copies)),
            medium: Box::new(Solid { m: Box::new(forest::PartsMaterial { parts: media_of }) }),
        })], claims)
    }

    /// Grass with a `"where"` rule (a graph, 0 to 1, read on the ground: slope, altitude, noise,
    /// and a landscape's fields), or on the landscape (`"heightmap": "landscape"`, over `min`
    /// to `max`, the scene's landscape object before it sampled every `"cell"` metres, 0.25 by
    /// default, so the grass stands on its displaced surface).
    pub fn parse_ruled_grass(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary,
        landscape: Option<&SceneObject>) -> SceneObject {
        SceneFile::grass_in(o, materials, media, geometries, landscape, None)
    }

    /// As `parse_ruled_grass`, only as thickly as `room` (0 to 1 at (x, z)) leaves room for.
    pub(crate) fn grass_in(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary,
        landscape: Option<&SceneObject>, room: Option<&(dyn Fn(f64, f64) -> f64 + Sync)>) -> SceneObject {
        use crate::procedural::grass::{create_grass_on, GrassMedium, Rule};
        // On the landscape, or on what `surface` names ("heightmap": "surface"), sampled.
        let (surfaces, land) = match o["heightmap"].as_str() {
            Some("landscape") => (vec![], Some(landscape.unwrap_or_else(|| panic!("grass on the \"landscape\" must come after a landscape object")))),
            Some("surface") => Footing::surfaces(o, geometries, landscape, "grass"),
            _ => (vec![], None),
        };
        let sampled = land.is_some() || !surfaces.is_empty();
        let grid = match sampled {
            true => {
                let (min, max) = (SceneFile::parse_vec2(&o["min"]), SceneFile::parse_vec2(&o["max"]));
                let footing = Footing::new(&surfaces, land);
                struct Dropped<'a>(&'a Footing<'a>);
                impl crate::procedural::heightmap::HeightSource for Dropped<'_> {
                    fn height(&self, x: f64, z: f64) -> f64 { self.0.at(x, z).map_or(0., |(p, _)| p.y) }
                }
                HeightGrid::sample(&Dropped(&footing), (min.x, min.y), (max.x, max.y), SceneFile::parse_number(&o["cell"], 0.25).max(0.01))
            },
            false => HeightGrid::from_json(&o["heightmap"]),
        };
        let graph = Footing::parse_rule(o, "grass");
        let rule = (graph.is_some() || room.is_some())
            .then(|| Rule { graph, fields: land.or(landscape).and_then(|l| l.medium.fields()), room });
        let (geometry, conditions) = create_grass_on(o, grid, rule.as_ref());
        println!("Grass: {} blades{}", geometry.field.len(), if rule.is_some() { " where its rule lets it grow" } else { "" });
        let medium: Box<dyn Medium + Sync + Send> = if o.get("material").is_some() || o.get("medium").is_some() {
            SceneFile::parse_object_medium(o, materials, media)
        } else {
            Box::new(GrassMedium::new(geometry.field.clone(), &conditions))
        };
        SceneObject { geometry: Box::new(geometry), medium }
    }

    /// A terrain displaced by the graph `d` (`shapes::displaced`), diced as its
    /// `"displacement_detail"` and the scene's camera (which the scene gives it) call for.
    pub(crate) fn parse_displaced_terrain(grid: Arc<HeightGrid>, d: &Value, o: &Value) -> crate::shapes::displaced::DisplacedTerrain {
        crate::shapes::displaced::DisplacedTerrain::of(grid, SceneFile::parse_displacement(d, o), None, None)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    /// What a terrain's displacement `d` and the rest of its object `o` ask for: the graph, its
    /// direction, the lift, the falloff and the detail it is diced to.
    pub(crate) fn parse_displacement(d: &Value, o: &Value) -> crate::shapes::displaced::Displacement {
        use crate::shapes::displaced::Detail;
        let graph = crate::graph::Graph::parse(d).unwrap_or_else(|e| panic!("terrain's displacement: {}", e));
        let v = &o["displacement_detail"];
        let camera = &o["camera"];
        let height = SceneFile::parse_number(&camera["image_height"], 0.);
        let eye = (!camera.is_null() && height > 0. && v["level"].is_null()).then(|| {
            (SceneFile::parse_vec3(&camera["location"]), camera["angle"].as_f64().unwrap_or(0.5).tan() / height)
        });
        let view = eye.and(SceneFile::parse_view(camera));
        let detail = Detail {
            eye,
            pixels: SceneFile::parse_number(&v["pixels"], 1.),
            max_level: SceneFile::parse_int(&v["max_level"], 8) as u32,
            min_spacing: v["min_spacing"].as_f64(),
            level: SceneFile::parse_int(&v["level"], 4) as u32,
            view,
            offscreen: SceneFile::parse_number(&v["offscreen"], 4.),
            // Where the scene puts it, found by `displaced_instances`.
            instances: o["displacement_instances"].as_array().map_or_else(Vec::new, |a| a.iter().map(|m| {
                let n: Vec<f64> = m.as_array().map_or_else(Vec::new, |r| r.iter().filter_map(Value::as_f64).collect());
                assert_eq!(n.len(), 16, "displacement_instances are 4 x 4 matrices, row by row");
                Affine3::from_matrix_unchecked(Matrix4::from_row_slice(&n))
            }).collect()),
        };
        let vertical = match o["displacement_direction"].as_str() {
            None | Some("normal") => false,
            Some("vertical") => true,
            Some(x) => panic!("Unknown displacement_direction '{}': expected \"normal\" or \"vertical\"", x),
        };
        let lift = o.get("vertical_displacement").map(|v| crate::graph::Graph::parse(v)
            .unwrap_or_else(|e| panic!("terrain's vertical_displacement: {}", e)));
        let falloff = SceneFile::parse_number(&o["displacement_falloff"], 0.);
        // {"x", "y", "z", "outward"}: graphs moving points along the terrain's axes, and
        // horizontally away from its slope.
        let v = &o["vector_displacement"];
        let axis = |k: &str| v.get(k).map(|g| crate::graph::Graph::parse(g)
            .unwrap_or_else(|e| panic!("terrain's vector_displacement {}: {}", k, e)));
        let sideways = crate::shapes::displaced::Sideways { x: axis("x"), y: axis("y"), z: axis("z"), outward: axis("outward") };
        crate::shapes::displaced::Displacement { graph, vertical, lift, sideways, falloff, detail }
    }

    /// A still perspective camera's view (`shapes::displaced::View`), a few pixels wider than
    /// the image for the pixel filter, so cells out of it can be diced coarser; None for other
    /// cameras, or without an image size.
    fn parse_view(camera: &Value) -> Option<crate::shapes::displaced::View> {
        let width = SceneFile::parse_number(&camera["image_width"], 0.);
        let height = SceneFile::parse_number(&camera["image_height"], 0.);
        let still_perspective = matches!(camera["type"].as_str(), None | Some("perspective"))
            && camera.get("motion").is_none() && camera.get("astigmatism").is_none();
        (width > 0. && height > 0. && still_perspective).then(|| {
            let frame = crate::camera::Frame::look_at(SceneFile::parse_vec3(&camera["location"]),
                SceneFile::parse_vec3(&camera["lookat"]), SceneFile::parse_vec3(&camera["up"]));
            let tan = camera["angle"].as_f64().unwrap_or(0.5).tan();
            let widen = |pixels: f64| 1. + 8. / pixels;
            // A ray from the lens, `aperture` off its centre, aimed through the focal plane,
            // strays from its pinhole ray by aperture * |1 - z / focus| at depth z: within
            // the planes moved out by the aperture and turned out by aperture / focus.
            let aperture = SceneFile::parse_number(&camera["aperture"], 0.2).abs();
            let focus = camera["focus_distance"].as_f64()
                .unwrap_or_else(|| (SceneFile::parse_vec3(&camera["lookat"]) - frame.location).norm());
            // (A lens focused at its own plane blurs everything: the view is then everything ahead.)
            let defocus = aperture / focus.max(1e-9);
            crate::shapes::displaced::View {
                eye: frame.location, right: frame.camx, up: frame.camy, forward: frame.camz,
                tan_x: 0.5 * tan * width / height * widen(width) + defocus,
                tan_y: 0.5 * tan * widen(height) + defocus,
                margin: aperture,
            }
        })
    }

    /// Where the scene puts each displaced terrain: the transforms around every reference to it,
    /// through named geometries (`{"type": "transform", "item": "cliff"}`, a terrain's own
    /// `"transform"` steps, `rotate`s), written into each terrain as `"displacement_instances"`
    /// so it is diced for the camera as each copy sees it. Terrains nowhere transformed are left
    /// alone.
    fn displaced_instances(objects: &mut [Value], geometries: &mut Map<String, Value>) {
        // (In a named geometry or not, its JSON pointer there, where it is put.)
        type Found = Vec<(Option<String>, String, Affine3<f64>)>;
        fn walk(v: &Value, root: &Option<String>, ptr: String, at: Affine3<f64>, geometries: &Map<String, Value>, depth: usize, found: &mut Found) {
            if depth > 64 { return; }
            let refer = |name: &str, at: Affine3<f64>, found: &mut Found| {
                if let Some(g) = geometries.get(name) {
                    walk(g, &Some(name.to_string()), String::new(), at, geometries, depth + 1, found);
                }
            };
            match v {
                Value::Object(o) => {
                    let mut at = at;
                    match o.get("type").and_then(Value::as_str) {
                        Some("transform") => at *= SceneFile::parse_affine(v),
                        Some("rotate") => at *= Affine3::from_matrix_unchecked(Rotation3::from_euler_angles(
                            SceneFile::parse_number(&v["roll"], 0.).to_radians(),
                            SceneFile::parse_number(&v["pitch"], 0.).to_radians(),
                            SceneFile::parse_number(&v["yaw"], 0.).to_radians()).to_homogeneous()),
                        _ => {},
                    }
                    if !v["transform"].is_null() {
                        at *= SceneFile::parse_steps(&v["transform"]);
                    }
                    if v["type"].as_str() == Some("terrain") && v.get("displacement").is_some() {
                        found.push((root.clone(), ptr, at));
                        return;
                    }
                    for (k, child) in o {
                        match (k.as_str(), child) {
                            ("item" | "a" | "b" | "clipped_by", Value::String(name)) => refer(name, at, found),
                            ("items", Value::Array(items)) => for (i, item) in items.iter().enumerate() {
                                match item {
                                    Value::String(name) => refer(name, at, found),
                                    _ => walk(item, root, format!("{}/items/{}", ptr, i), at, geometries, depth + 1, found),
                                }
                            },
                            ("camera" | "transform" | "motion" | "heightmap" | "displacement" | "vertical_displacement", _) => {},
                            (_, Value::Object(_) | Value::Array(_)) => walk(child, root, format!("{}/{}", ptr, k.replace('~', "~0").replace('/', "~1")), at, geometries, depth + 1, found),
                            _ => {},
                        }
                    }
                },
                Value::Array(items) => for (i, item) in items.iter().enumerate() {
                    walk(item, root, format!("{}/{}", ptr, i), at, geometries, depth + 1, found);
                },
                _ => {},
            }
        }
        let mut found: Found = vec![];
        for (i, o) in objects.iter().enumerate() {
            walk(o, &None, format!("/{}", i), Affine3::identity(), geometries, 0, &mut found);
        }
        let mut placed: HashMap<(Option<String>, String), Vec<Affine3<f64>>> = HashMap::new();
        for (root, ptr, at) in found {
            placed.entry((root, ptr)).or_default().push(at);
        }
        for ((root, ptr), ats) in placed {
            if ats.iter().all(|a| (a.matrix() - Matrix4::identity()).abs().max() < 1e-12) {
                continue;
            }
            let matrices: Vec<Value> = ats.iter().map(|a| {
                let m = a.matrix();
                Value::from((0..16).map(|k| m[(k / 4, k % 4)]).collect::<Vec<f64>>())
            }).collect();
            let target = match &root {
                Some(name) => geometries.get_mut(name).and_then(|g| g.pointer_mut(&ptr)),
                None => {
                    // "/i/rest": the i'th object, then the rest within it.
                    let rest = ptr.splitn(3, '/').nth(2).map_or(String::new(), |r| format!("/{}", r));
                    let i: usize = ptr.split('/').nth(1).and_then(|i| i.parse().ok()).unwrap_or(0);
                    objects.get_mut(i).and_then(|o| o.pointer_mut(&rest))
                },
            };
            if let Some(t) = target {
                t["displacement_instances"] = Value::from(matrices);
            }
        }
    }

    /// A forest's `"lod"`: `true`, or `{"pixels": 1, "max_level": 12}`, to see each tree at the
    /// level of detail the size of a pixel where it stands calls for (`shapes::lod`), dropping
    /// detail up to `pixels` across. It needs the scene's camera, which the scene gives it.
    fn parse_forest_lod(o: &Value) -> Option<crate::procedural::forest::Lod> {
        let pixels = match &o["lod"] {
            Value::Null | Value::Bool(false) => return None,
            Value::Bool(true) => 1.,
            v => SceneFile::parse_number(&v["pixels"], 1.),
        };
        let camera = &o["camera"];
        let height = SceneFile::parse_number(&camera["image_height"], 0.);
        let angle = camera["angle"].as_f64().unwrap_or(0.5);
        if camera.is_null() || height <= 0. {
            println!("A forest's \"lod\" needs the scene's camera: seeing every tree in full");
            return None;
        }
        Some(crate::procedural::forest::Lod {
            eye: SceneFile::parse_vec3(&camera["location"]),
            pixel_angle: angle.tan() / height,
            pixels,
            max_level: SceneFile::parse_int(&o["lod"]["max_level"], 12),
        })
    }

    /// The parts of a tree with parameters `p`, chosen by `seed`, grown at `location`, each a
    /// geometry and the medium it's made of: its bark, its dead branches' bark, and its leaves in
    /// as many parts as `o` names leaf materials. `o` names the materials as for `parse_tree`.
    pub fn tree_parts(o: &Value, p: &tree::TreeParams, seed: u64, location: Vector3<f64>,
                      materials: &Map<String, Value>, media: &Map<String, Value>)
                      -> Vec<(SharedGeometry, Box<dyn Medium + Sync + Send>)> {
        SceneFile::tree_details(o, p, seed, location, materials, media).0.into_iter()
            .map(|(detail, medium)| (detail.geometry(0.), medium))
            .collect()
    }

    /// The parts of a tree as for `tree_parts`, as detail to make levels of from
    /// (`shapes::lod`), with the size of its leaves, the level of detail's base, and its bounds.
    pub fn tree_details(o: &Value, p: &tree::TreeParams, seed: u64, location: Vector3<f64>,
                        materials: &Map<String, Value>, media: &Map<String, Value>)
                        -> (Vec<TreeDetail>, f64, BBox) {
        use crate::shapes::lod::{self, Detail};
        let mesh = tree::generate(p, seed, location);
        let mut bounds = BBox::new(Vector3::repeat(f64::INFINITY), Vector3::repeat(f64::NEG_INFINITY));
        for t in &mesh.bark {
            bounds = bounds.union(&t.bounds());
        }
        for t in &mesh.leaves {
            bounds = bounds.union(&t.bounds());
        }
        let base = lod::piece_size(&mesh.leaves);
        let base = if base > 0. { base } else { 0.01 * (bounds.max - bounds.min).max() };
        let medium = |key: &Value| -> Box<dyn Medium + Sync + Send> {
            let name = SceneFile::parse_string(key);
            if materials.contains_key(&name) {
                Box::new(Solid { m: SceneFile::require_material(key, materials, "tree") })
            } else if media.contains_key(&name) {
                SceneFile::parse_medium_ref(key, materials, media)
                    .unwrap_or_else(|| panic!("Tree medium {} has an unknown type", key))
            } else {
                panic!("Tree material {} is neither a material nor a medium", key)
            }
        };
        // Leaf materials get each leaf's state, for its autumn or dead colour.
        let states = Arc::new(mesh.leaf_states.clone());
        let leaf_medium = |key: &Value| -> Box<dyn Medium + Sync + Send> {
            match materials.get(&SceneFile::parse_string(key)) {
                Some(props) if props["type"] == "leaf" => {
                    let mut leaf = SceneFile::parse_leaf(props);
                    leaf.states = Some(states.clone());
                    Box::new(Solid { m: Box::new(leaf) })
                },
                _ => medium(key),
            }
        };
        let default = |albedo: Color| -> Box<dyn Medium + Sync + Send> {
            Box::new(Solid { m: Box::new(Lambertian { albedo: albedo.into() }) })
        };
        let bark = || o.get("bark").map_or_else(|| default(Color::new(0.25, 0.2, 0.16)), medium);

        let mut parts: Vec<(Detail, Box<dyn Medium + Sync + Send>)> = vec![];
        if !mesh.bark.is_empty() {
            parts.push((Detail::Surface(mesh.bark.clone()), bark()));
        }
        if !mesh.dead_bark.is_empty() {
            parts.push((Detail::Surface(mesh.dead_bark.clone()), o.get("dead_bark").map_or_else(bark, medium)));
        }

        let leaf_media: Vec<Box<dyn Medium + Sync + Send>> = match o.get("leaves") {
            None => vec![default(Color::new(0.1, 0.3, 0.06))],
            Some(Value::Array(names)) if !names.is_empty() => names.iter().map(leaf_medium).collect(),
            Some(Value::Array(_)) => panic!("A tree's \"leaves\" list is empty"),
            Some(name) => vec![leaf_medium(name)],
        };
        // Deal each leaf (all its triangles) to one of the media.
        let mut groups: Vec<Vec<Arc<Tagged<Triangle>>>> = vec![vec![]; leaf_media.len()];
        let mut rng = crate::geometry::Pcg32::new(seed, 0x1eaf);
        for leaf in mesh.leaves.chunks(mesh.leaf_triangles) {
            let k = ((rng.next_f64() * groups.len() as f64) as usize).min(groups.len() - 1);
            groups[k].extend_from_slice(leaf);
        }
        for (triangles, medium) in groups.into_iter().zip(leaf_media) {
            if !triangles.is_empty() {
                parts.push((Detail::Aggregate(triangles), medium));
            }
        }
        (parts, base, bounds)
    }

    pub fn parse_skysphere(o: &Value) -> SceneObject {
        create_sky_sphere(o)
    }

    /// A participating medium filling the box from `min` to `max`:
    ///
    /// `{"type": "volume", "min": [x, y, z], "max": [x, y, z], "density": σ, "albedo": colour,
    /// "g": asymmetry, "noise": {"scale", "octaves", "threshold", "seed"}, "grid": ...}`
    ///
    /// `density` is the extinction coefficient, the chance of a collision per unit distance
    /// (default 1). Without `noise` it is constant; with it, it varies up to that maximum. It
    /// may instead be a function graph (`graph.rs`), its value the extinction where it is
    /// evaluated (`volume::GraphDensity`); after a landscape in the scene, the graph reads its
    /// fields, and `altitude` is the height above the planet. With
    /// `grid` (`volume::grid`), it is the grid's values times `density`: a Mitsuba `.vol` file's
    /// path, or `{"resolution": [nx, ny, nz], "values": [...]}` (x fastest); `min` and `max`
    /// default to the file's bounds. In place of `albedo`, `sigma_a` and `sigma_s` (RGB) give
    /// absorption and scattering per unit density, as pbrt's media; where their sum differs
    /// between channels the medium is chromatic (`volume::chromatic`). With `shape`, a geometry (as an object's, eg.
    /// `{"type": "sphere", ...}`), the medium fills only its inside: give a dielectric object
    /// the same shape for a glass of coloured liquid (`volume::Volume::shape`). `emission`
    /// makes it glow (`parse_volume_emission`).
    /// `albedo` (default white) is the fraction scattered rather than absorbed at a collision.
    /// How it scatters is given by `g` (Henyey-Greenstein, -1 to 1, default 0) or by
    /// `droplet_diameter` (in µm, 5 to 50: the Mie scattering of water droplets of that size;
    /// see `PhaseFunction::water_droplets`).
    pub fn parse_volume(o: &Value, geometries: &GeometryLibrary) -> SceneObject {
        SceneFile::parse_volume_over(o, geometries, None)
    }

    /// As `parse_volume`, a density graph reading `fields` (the landscape's, for a volume after
    /// one in the scene).
    pub fn parse_volume_over(o: &Value, geometries: &GeometryLibrary, fields: Option<Arc<dyn crate::graph::Fields>>) -> SceneObject {
        // A graph for density: its value is the extinction (`volume::GraphDensity`).
        let graphed = matches!(o.get("density"), Some(Value::Object(_) | Value::String(_)));
        let max_density = if graphed { 1. } else { SceneFile::parse_number(&o["density"], 1.) };
        let grid = o.get("grid").map(SceneFile::parse_grid);
        let shape = o.get("shape").map(|v| SceneFile::parse_geometry(v, geometries)
            .unwrap_or_else(|| panic!("A volume's shape must be a geometry, got {}", v)));
        // The box the field spans: as given, or else the grid file's, or the shape's bounds.
        let bounds = match (o.get("min"), grid.as_ref().and_then(|g| g.bounds), &shape) {
            (None, Some(b), _) => b,
            (None, None, Some(s)) => s.bounds(),
            _ => BBox::new(SceneFile::parse_vec3(&o["min"]), SceneFile::parse_vec3(&o["max"])),
        };
        let density: Arc<dyn DensityField> = match (grid, o.get("noise")) {
            _ if graphed => {
                assert!(o.get("grid").is_none() && o.get("noise").is_none(), "Give a volume a density graph, or a grid or noise, not both");
                let g = crate::graph::Graph::parse(&o["density"]).unwrap_or_else(|e| panic!("volume's density: {}", e));
                Arc::new(crate::volume::GraphDensity::new(g, fields).unwrap_or_else(|e| panic!("{}", e)))
            },
            (Some(_), Some(_)) => panic!("Give a volume either a grid or noise, not both"),
            (Some(g), None) => Arc::new(GridDensity::new(bounds, g, max_density)),
            (None, None) => Arc::new(ConstantDensity(max_density)),
            (None, Some(n)) => Arc::new(NoiseDensity {
                perlin: SceneFile::parse_seed(n).map_or_else(PerlinNoise::new, PerlinNoise::with_seed),
                max_density,
                scale: SceneFile::parse_number(&n["scale"], 1.),
                octaves: SceneFile::parse_int(&n["octaves"], 4) as u32,
                threshold: SceneFile::parse_number(&n["threshold"], 0.5),
            }),
        };
        let volume = match shape {
            None => Volume::new(bounds, density),
            Some(s) => Volume::in_shape(s, density),
        };
        let mut scattering = SceneFile::parse_scattering(o, 0.);
        scattering.emission = SceneFile::parse_volume_emission(o, bounds);
        // Coefficients per unit density, as pbrt gives media: σ_a and σ_s (RGB).
        if o.get("sigma_a").is_some() || o.get("sigma_s").is_some() {
            assert!(o.get("albedo").is_none(), "Give a volume either an albedo or sigma_a and sigma_s, not both");
            let a = SceneFile::parse_color_def(o, "sigma_a", Color::black());
            let sc = SceneFile::parse_color_def(o, "sigma_s", Color::black());
            assert!((a + sc).rgb.iter().all(|k| *k >= 0.) && (a + sc).rgb.max() > 0., "A volume needs some extinction");
            let t = (a + sc).rgb;
            scattering.albedo = Color { rgb: sc.rgb.zip_map(&t, |s, t| if t > 0. { s / t } else { 0. }) };
            scattering.extinction = Some((a, sc));
        }
        create_medium(volume, scattering)
    }

    /// A voxel grid (`volume::grid::Grid`): a `.vol` file's path, or `{"resolution": [nx, ny,
    /// nz], "values": [...]}`, x fastest.
    pub fn parse_grid(v: &Value) -> Arc<Grid> {
        if let Some(path) = v.as_str() {
            return Grid::load(path);
        }
        let res = v["resolution"].as_array().filter(|r| r.len() == 3)
            .unwrap_or_else(|| panic!("A grid needs a file name or a \"resolution\" of three numbers"));
        let n: Vec<usize> = res.iter().map(|x| SceneFile::parse_int(x, 0)).collect();
        let values = v["values"].as_array().unwrap_or_else(|| panic!("A grid needs \"values\""))
            .iter().map(|x| SceneFile::parse_number(x, 0.) as f32).collect();
        Arc::new(Grid::new(n[0], n[1], n[2], values))
    }

    /// A medium's multiple-scattering approximations (`volume::ScatteringApproximation`):
    /// `"similarity_after": N` for the similarity relations from the N-th scattering on,
    /// `"oz": true` or `{"octaves", "a", "b", "c"}` for Wrenninge et al.'s octaves,
    /// `"diffusion": true` or `{"steps": 32}` for sunlight diffused through thick decks by
    /// the slab diffusion solution for each column (overcast skies), and
    /// `"delta_m": false` to trace a Henyey-Greenstein medium exactly rather than truncating
    /// its forward peak wherever the camera doesn't see the medium directly (on by default).
    pub fn parse_scattering_approximation(o: &Value) -> ScatteringApproximation {
        let oz = match o.get("oz") {
            None | Some(Value::Bool(false)) => None,
            Some(Value::Bool(true)) => Some(Oz::default()),
            Some(v) => {
                let d = Oz::default();
                Some(Oz {
                    octaves: SceneFile::parse_int(&v["octaves"], d.octaves as usize) as u32,
                    a: SceneFile::parse_number(&v["a"], d.a),
                    b: SceneFile::parse_number(&v["b"], d.b),
                    c: SceneFile::parse_number(&v["c"], d.c),
                })
            },
        };
        ScatteringApproximation {
            similarity_after: o.get("similarity_after").map(|n| SceneFile::parse_int(n, 0) as u32),
            oz,
            delta_m: o["delta_m"].as_bool().unwrap_or(true),
            diffusion: match o.get("diffusion") {
                None | Some(Value::Bool(false)) => None,
                Some(Value::Bool(true)) => Some(32),
                Some(v) => Some(SceneFile::parse_int(&v["steps"], 32).max(1) as u32),
            },
        }
    }

    /// How a volume scatters: `albedo`, its phase function (`g` defaulting to `default_g`, or
    /// `droplet_diameter`), approximations, `diffraction`, `halos`, and `"rainbow": true` for
    /// raindrops' bows in direct sunlight, by geometric optics, or `"rainbow": {"drop_radius":
    /// µm, "spread": 0.1}` by Airy's wave optics (supernumerary bows; for fog's 10-20 µm
    /// droplets, the fogbow).
    pub fn parse_scattering(o: &Value, default_g: f64) -> Scattering {
        Scattering {
            albedo: SceneFile::parse_color_def(o, "albedo", Color::white()),
            phase: SceneFile::parse_phase(o, default_g),
            approximation: SceneFile::parse_scattering_approximation(o),
            diffraction: SceneFile::parse_diffraction(o),
            halos: SceneFile::parse_halos(o),
            // `true` for geometric optics; `{"drop_radius": µm, "spread": 0.1}` for wave optics
            // (supernumeraries; the fogbow of small droplets).
            // `"mie": {"radius": µm, "variance": 0.1}` for small droplets by Mie theory: the
            // glory, the cloudbow and the forward peak.
            rainbow: match (o.get("rainbow"), o.get("mie")) {
                (_, Some(v)) if v.is_object() => Some(Arc::new(crate::material::mie::MieTable::new(
                    SceneFile::parse_number(&v["radius"], 5.), SceneFile::parse_number(&v["variance"], 0.1)))),
                (Some(Value::Bool(true)), _) => Some(Arc::new(RainbowTable::new(100_000))),
                (Some(v), _) if v.is_object() => Some(Arc::new(RainbowTable::airy(40_000,
                    SceneFile::parse_number(&v["drop_radius"], 500.), SceneFile::parse_number(&v["spread"], 0.1)))),
                _ => None,
            },
            emission: None,
            extinction: None,
            sun_probes: o["sun_probes"].as_bool().unwrap_or(crate::volume::SUN_PROBES),
        }
    }

    /// Light a volume emits (`volume::emission`), in the box `bounds`: `"emission": colour`, or
    /// `{"color": colour, "scale": s}`, or blackbody radiation `{"temperature": kelvin, "scale":
    /// s}` with luminance `s` (default 1) at the hottest. The temperature may come from a grid
    /// (`"temperature": "fire.vol"` or inline, see `parse_grid`), its values times
    /// `temperature_scale`, and below `cutoff` kelvin nothing is emitted. The medium emits
    /// only as much as it absorbs: give it an albedo below one.
    pub fn parse_volume_emission(o: &Value, bounds: BBox) -> Option<VolumeEmission> {
        let v = o.get("emission")?;
        if v.is_array() {
            return Some(VolumeEmission::color(SceneFile::parse_color(v), 1.));
        }
        let scale = SceneFile::parse_number(&v["scale"], 1.);
        match v.get("temperature") {
            None => Some(VolumeEmission::color(SceneFile::parse_color_def(v, "color", Color::white()), scale)),
            Some(t) => {
                let field: Arc<dyn DensityField> = if t.is_number() {
                    Arc::new(ConstantDensity(SceneFile::parse_number(t, 0.)))
                } else {
                    let g = SceneFile::parse_grid(t);
                    let b = if o.get("min").is_none() { g.bounds.unwrap_or(bounds) } else { bounds };
                    Arc::new(GridDensity::new(b, g, SceneFile::parse_number(&v["temperature_scale"], 1.)))
                };
                Some(VolumeEmission::temperature(field, SceneFile::parse_number(&v["cutoff"], 0.), scale))
            },
        }
    }

    /// Ice-crystal halos: `"halos": true` for plates (a third: sun dogs) and randomly oriented
    /// columns (the 22° and 46° halos), or `{"crystals": [{"habit": "plate" | "column" |
    /// "random", "fraction", "aspect", "tilt"}], "rays": 4000000}`: populations by their share
    /// of the light, the prisms' half-height over their radius, and their tilt from horizontal
    /// (degrees, standard deviation); and the rays traced per colour channel.
    pub fn parse_halos(o: &Value) -> Option<Halos> {
        let v = match o.get("halos") {
            None | Some(Value::Bool(false)) => return None,
            Some(v) => v,
        };
        let crystals = match v.get("crystals").and_then(|c| c.as_array()) {
            None => vec![
                Crystals { habit: Habit::Plate, fraction: 1. / 3., aspect: 0.15, tilt: 0.5 },
                Crystals { habit: Habit::Random, fraction: 2. / 3., aspect: 1.5, tilt: 0. },
            ],
            Some(list) => list.iter().map(|c| {
                let habit = match c["habit"].as_str() {
                    Some("plate") => Habit::Plate,
                    Some("column") => Habit::Column,
                    Some("random") => Habit::Random,
                    h => panic!("Unknown crystal habit {:?}: expected plate, column or random", h),
                };
                let aspect = SceneFile::parse_number(&c["aspect"], if habit == Habit::Plate { 0.15 } else { 1.5 });
                Crystals { habit, fraction: SceneFile::parse_number(&c["fraction"], 1.), aspect,
                           tilt: SceneFile::parse_number(&c["tilt"], 0.5) }
            }).collect(),
        };
        assert!(!crystals.is_empty() && crystals.iter().all(|c| c.fraction >= 0. && c.aspect > 0.),
            "Halo crystals need positive aspects and fractions");
        Some(Halos {
            crystals,
            rays: SceneFile::parse_int(&v["rays"], 4_000_000).max(64),
            seed: SceneFile::parse_seed(v).unwrap_or(0),
            table: std::sync::OnceLock::new(),
        })
    }

    /// Diffraction by the medium's particles, for iridescence and coronae: `"diffraction": true`
    /// or `{"fraction": 0.5, "radius": 10, "spread": 0.1}`: the share of scattering diffracted,
    /// the particles' mean radius in µm (where the medium doesn't vary it), and the relative
    /// spread of their sizes. None if absent or false.
    pub fn parse_diffraction(o: &Value) -> Option<Diffraction> {
        let v = match o.get("diffraction") {
            None | Some(Value::Bool(false)) => return None,
            Some(v) => v,
        };
        let fraction = SceneFile::parse_number(&v["fraction"], 0.5);
        assert!((0. ..=1.).contains(&fraction), "Diffraction fraction must be 0 to 1, got {}", fraction);
        Some(Diffraction {
            table: DiffractionTable::new(SceneFile::parse_number(&v["spread"], 0.1).max(0.)),
            fraction,
            radius: SceneFile::parse_number(&v["radius"], 10.),
        })
    }

    /// A medium's phase function: `droplet_diameter` (µm) for water droplets, or else
    /// Henyey-Greenstein with `g` (default `default_g`).
    pub fn parse_phase(o: &Value, default_g: f64) -> PhaseFunction {
        match o.get("droplet_diameter") {
            Some(d) => {
                assert!(o.get("g").is_none(), "Give a medium either g or droplet_diameter, not both");
                PhaseFunction::water_droplets(SceneFile::parse_number(d, 0.))
            },
            None => {
                let g = SceneFile::parse_number(&o["g"], default_g);
                assert!(g > -1. && g < 1., "g must be between -1 and 1, got {}", g);
                PhaseFunction::HenyeyGreenstein { g }
            },
        }
    }

    /// A layer of procedural clouds by the Nubis method (see `procedural::nubis_clouds`), as a
    /// volume from `base` to `top` and `extent` either side of `center` ([x, z]) across:
    ///
    /// `{"type": "nubis_clouds", "base": 1500, "top": 3000, "extent": 20000, "coverage": 0.5,
    /// "cloud_type": 1, "density": 0.05, "albedo": colour, "g": 0.85, "seed": 0}`, with
    /// `droplet_diameter` in place of `g` for Mie scattering (as for `volume`).
    ///
    /// `coverage` (0 to 1) is how much of the sky is cloud; `cloud_type` runs from stratus (0)
    /// through stratocumulus (0.5) to cumulus (1); `density` is the extinction coefficient in
    /// the thickest cloud. `weather_size`, `shape_size` and `detail_size` set the sizes of
    /// cloud fields, clouds and their eroded edges, and `erosion` (0 to 1) how much the edges
    /// are worn; they default to suit the layer's thickness.
    ///
    /// `mapping` places the layer: `"regional"` (the default) curved over the planet about the
    /// origin, `extent` across; `"flat"` (or `"curved": false`) on a flat earth; `"global"`
    /// over the whole planet (`GlobalNubis`), with `map_size` texels along each cube face of
    /// the weather map (default 1024), `climate` (default true) for bands of weather by
    /// latitude about the `pole` (default [0, 0, 1]), and weather systems `weather_size`
    /// across (default 400 km).
    pub fn parse_nubis_clouds(o: &Value) -> SceneObject {
        let base = SceneFile::parse_number(&o["base"], 1500.);
        let top = SceneFile::parse_number(&o["top"], 3000.);
        let extent = SceneFile::parse_number(&o["extent"], 20000.);
        let center = SceneFile::parse_vec2_def(o, "center", Vector2::new(0., 0.));
        let mut layer = NubisCloudLayer::new(base, top, center, extent, SceneFile::parse_seed(o).unwrap_or(0));
        layer.max_density = SceneFile::parse_number(&o["density"], layer.max_density);
        layer.coverage = SceneFile::parse_number(&o["coverage"], layer.coverage).clamp(0., 1.);
        layer.cloud_type = SceneFile::parse_number(&o["cloud_type"], layer.cloud_type).clamp(0., 1.);
        layer.weather_size = SceneFile::parse_number(&o["weather_size"], layer.weather_size);
        layer.shape_size = SceneFile::parse_number(&o["shape_size"], layer.shape_size);
        layer.detail_size = SceneFile::parse_number(&o["detail_size"], layer.detail_size);
        layer.erosion = SceneFile::parse_number(&o["erosion"], layer.erosion).clamp(0., 1.);
        if o["mapping"].as_str() == Some("global") {
            let planet = Planet { radius: SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS) };
            // Planetary weather: systems hundreds of kilometres across, unless told otherwise.
            if o.get("weather_size").is_none() {
                layer.weather_size = 400_000.;
            }
            let mut global = GlobalNubis::new(layer, planet.radius);
            global.map_size = SceneFile::parse_int(&o["map_size"], global.map_size).max(2);
            global.climate = o["climate"].as_bool().unwrap_or(true);
            global.pole = SceneFile::parse_vec3_def(o, "pole", global.pole).normalize();
            return create_volume(SceneFile::shell(&planet, top), Arc::new(GlobalMapping { planet, field: Arc::new(global) }),
                SceneFile::parse_scattering(o, 0.85));
        }
        let bounds = BBox::new(
            Vector3::new(center.x - extent, base, center.y - extent),
            Vector3::new(center.x + extent, top, center.y + extent),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(layer));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// Cumulus grown by simulating a sunny day (see `procedural::harris_cumulus`):
    ///
    /// `{"type": "harris_cumulus", "time": 5400, "cache": "demo/scenes/.cumulus-cache"}`
    ///
    /// The simulation runs for `time` seconds (default 90 minutes), continuing from the latest
    /// state saved in `cache` (a directory) at or before it, and saves its own there, so an
    /// animation's frames simulate only from one to the next. Its grid is `grid` ([x, y, z]
    /// cells; x and z powers of two; default [64, 44, 64]) of `cell` metres (100), periodic
    /// across the ground.
    ///
    /// The day is the ARM case's (Oklahoma, 21 June 1997), from 09:30 as its first cumulus
    /// form (`"sounding": "arm_mid_morning"`, the default, `start` 14400 s after 05:30), or
    /// from dawn (`"sounding": "arm"`, `start` 0), or a `sounding` of your own ({"height": [m],
    /// "theta": [K], "water": [g/kg], "surface_pressure": Pa}). The ground's fluxes follow the
    /// case's day unless steady `heat_flux` and `moisture_flux` (W/m²) are given; `large_scale`
    /// false drops the air mass's slow tendencies. The fluxes vary by `patchiness` (0 to 1) in
    /// patches `patch_size` across that change over `patch_period` seconds; `wind` ([x, z] m/s,
    /// default [10, 0]) carries everything; `friction_velocity` (m/s) stirs the air near the
    /// ground; `vorticity` (confinement); `max_step` (s); `seed`. `"ice": true` for mixed-phase
    /// cloud, and `"rain": true` for rain (graupel below freezing, with ice) that falls through
    /// the air, weighs it down and evaporates, cooling it (`SimulationParams::rain`).
    ///
    /// `state` renders a state saved by `CloudSimulation::save` instead of simulating.
    /// `follow_wind` renders the clouds as seen drifting with the wind, so in an animation
    /// they stay in view as they grow and dissipate, rather than crossing the sky.
    ///
    /// Turbulence finer than the grid (`HarrisCumulus::turbulent`, after Kim et al.'s wavelet
    /// turbulence): `turbulence` (strength, default 0.35; 0 for none) over `turbulence_octaves`
    /// (default 3) below the cell size, looked up at texture coordinates the simulation carries
    /// with the flow, each set renewed every `texture_period` seconds (default 600).
    ///
    /// For rendering: `droplet_radius` (µm, default 10; sets the extinction per gram of
    /// water, and the diffraction), `detail_size`, `erosion` (0 to 1) and `edge` (g/kg of
    /// cloud water the detail can erode), `repeat` (tile the grid across the sky, default
    /// true) out to `extent` from `center` ([x, z]), fading over its outer `fade`, and
    /// `mapping` as for `nubis_clouds`.
    pub fn parse_harris_cumulus(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut p = SimulationParams::default();
        if let Some(g) = o["grid"].as_array() {
            let n = |i: usize, d: usize| g.get(i).and_then(|v| v.as_u64()).map_or(d, |v| v as usize);
            p.size = (n(0, p.size.0), n(1, p.size.1), n(2, p.size.2));
        }
        p.cell = num("cell", p.cell);
        // Steady fluxes in place of the ARM case's day, if given.
        if o.get("heat_flux").is_some() || o.get("moisture_flux").is_some() {
            p.forcing = Forcing::steady(num("heat_flux", 200.), num("moisture_flux", 300.));
        }
        if o["large_scale"].as_bool() == Some(false) {
            p.forcing.theta_tendency = Profile::constant(0.);
            p.forcing.water_tendency = Profile::constant(0.);
        }
        p.patchiness = num("patchiness", p.patchiness).clamp(0., 1.);
        p.patch_size = num("patch_size", p.patch_size);
        p.patch_period = num("patch_period", p.patch_period);
        p.wind = SceneFile::parse_vec2_def(o, "wind", p.wind);
        p.friction_velocity = num("friction_velocity", p.friction_velocity).max(0.);
        p.vorticity = num("vorticity", p.vorticity).max(0.);
        p.max_step = num("max_step", p.max_step);
        p.seed = SceneFile::parse_seed(o).unwrap_or(0);
        p.start = num("start", p.start);
        p.texture_period = num("texture_period", p.texture_period).max(1.);
        // A cloud to start from: `"bubble": {"center": [x, z] (m from the grid's corner; default
        // its middle), "height": m (of its middle), "radius": m, "depth": m (top to bottom;
        // default twice the radius: a ball), "warmth": K, "supersaturation": 0.05}`.
        if let Some(b) = o.get("bubble") {
            let (nx, _, nz) = p.size;
            let middle = Vector2::new(nx as f64, nz as f64) * p.cell / 2.;
            let c = SceneFile::parse_vec2_def(b, "center", middle);
            p.bubble = Some(crate::procedural::harris_cumulus::Bubble {
                centre: Vector3::new(c.x, SceneFile::parse_number(&b["height"], 1000.), c.y),
                radius: SceneFile::parse_number(&b["radius"], 400.).max(1.),
                height: SceneFile::parse_number(&b["depth"], 2. * SceneFile::parse_number(&b["radius"], 400.)).max(1.) / 2.,
                warmth: SceneFile::parse_number(&b["warmth"], 2.),
                supersaturation: SceneFile::parse_number(&b["supersaturation"], 0.05).max(0.),
            });
        }
        // A hero cloud: `"hot_spot": {"center": [x, z] (m from the grid's corner; default its
        // middle), "radius": m, "boost": 1}`.
        if let Some(h) = o.get("hot_spot") {
            let (nx, _, nz) = p.size;
            let middle = Vector2::new(nx as f64, nz as f64) * p.cell / 2.;
            p.hot_spot = Some((SceneFile::parse_vec2_def(h, "center", middle), SceneFile::parse_number(&h["radius"], 500.).max(1.),
                SceneFile::parse_number(&h["boost"], 1.).max(0.)));
        }
        p.ice = o["ice"].as_bool().unwrap_or(false);
        // Rain as a field of its own, falling and evaporating (`SimulationParams::rain`).
        p.rain = o["rain"].as_bool().unwrap_or(false);
        if o["sounding"].as_str() == Some("weisman_klemp") {
            // Thunderstorms: no daily cycle, the storm started from a bubble.
            p.sounding = crate::procedural::harris_cumulus::Sounding::weisman_klemp(num("surface_water", 14.) * 1e-3);
            p.forcing = Forcing::steady(num("heat_flux", 0.), num("moisture_flux", 0.));
            p.start = num("start", 0.);
            p.ice = o["ice"].as_bool().unwrap_or(true);
        } else if o["sounding"].as_str() == Some("arm") {
            // The full day, from dawn.
            p.sounding = Sounding::arm();
            p.start = num("start", 0.);
        } else if let Some(name) = o["sounding"].as_str() {
            assert!(name == "arm_mid_morning", "harris_cumulus: unknown sounding '{}': expected arm, arm_mid_morning, weisman_klemp or a profile", name);
        } else if let Some(sounding) = o.get("sounding") {
            let list = |k: &str| -> Vec<f64> {
                sounding[k].as_array().unwrap_or_else(|| panic!("harris_cumulus: the sounding needs \"{}\"", k))
                    .iter().map(|v| v.as_f64().unwrap()).collect()
            };
            let height = list("height");
            p.sounding = Sounding {
                theta: Profile::new(&height, &list("theta")),
                water: Profile::new(&height, &list("water").iter().map(|w| w * 1e-3).collect::<Vec<f64>>()),
                surface_pressure: SceneFile::parse_number(&sounding["surface_pressure"], 1e5),
            };
        }
        let time = num("time", 5400.).max(0.);
        let cache = o["cache"].as_str().map(std::path::PathBuf::from);
        // A saved state, as is (eg. from the `harris_day` diagnostic), or a simulation.
        let sim = match o["state"].as_str() {
            Some(path) => CloudSimulation::load(p.clone(), std::path::Path::new(path))
                .unwrap_or_else(|e| panic!("harris_cumulus: couldn't load the state {}: {}", path, e)),
            None => CloudSimulation::cached(p.clone(), time, cache.as_deref()),
        };
        let center = SceneFile::parse_vec2_def(o, "center", Vector2::new(0., 0.));
        let mut c = HarrisCumulus::with_turbulence(&sim, num("droplet_radius", 10.), center,
            num("turbulence", 0.35).max(0.), SceneFile::parse_int(&o["turbulence_octaves"], 3).min(8));
        c.detail_size = num("detail_size", c.detail_size).max(1e-3);
        c.erosion = num("erosion", c.erosion).clamp(0., 1.);
        c.edge = num("edge", c.edge * 1000.) / 1000.;
        c.repeat = o["repeat"].as_bool().unwrap_or(c.repeat);
        if o["follow_wind"].as_bool() == Some(true) {
            // Seen drifting with the air: the clouds stay where they are, growing and dying in
            // place, as in a time-lapse that tracks them.
            c.origin -= c.drift;
            c.drift = Vector3::zeros();
        }
        c.extent = num("extent", c.extent);
        c.fade = num("fade", c.fade).clamp(0., 1.);
        let (nx, ny, nz) = p.size;
        let half = Vector3::new(nx as f64, 0., nz as f64) * p.cell / 2.;
        let bounds = if c.repeat {
            BBox::new(Vector3::new(center.x - c.extent, 0., center.y - c.extent), Vector3::new(center.x + c.extent, ny as f64 * p.cell, center.y + c.extent))
        } else {
            BBox::new(Vector3::new(center.x, 0., center.y) - half, Vector3::new(center.x, ny as f64 * p.cell, center.y) + half)
        };
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(c));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// One cauliflower cumulus (see `procedural::cauliflower::Cumulus`), placed by hand:
    ///
    /// `{"type": "cumulus", "center": [x, z], "base": 1000, "width": 800, "depth": 1500,
    /// "seed": 0}`, with `density` (extinction per metre in its core, 0.05), `edge` (m, 15),
    /// `lift`, `dome` (how much higher the body's middle stands, in its spheres' radius: 0.8),
    /// `lean` (per metre of height) and `wind_direction` ([x, z]), `billows` (a scale)
    /// and `wisps`, `wisp_size` (m), and the sphere tree's `levels`, `ratio`, `children`, `embed`
    /// and `rise`; and how it scatters, as any volume (`g` default 0.85). `"tower": true` makes
    /// it a towering cumulus of stacked thermals (`width` its radius, `depth` its height), frozen
    /// above `glaciation` (m) into a cumulonimbus's smooth top; `maturity` (0 to 1) how far it
    /// has formed: wispy and translucent when young (`Surface::condensed`).
    pub fn parse_cumulus(o: &Value) -> SceneObject {
        use crate::procedural::cauliflower::{Cauliflower, Cumulus, Surface};
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let seed = SceneFile::parse_seed(o).unwrap_or(0);
        let d = Cauliflower::cumulus();
        let tree = Cauliflower::new(SceneFile::parse_int(&o["levels"], d.levels).min(8), num("ratio", d.ratio).clamp(0.1, 0.8),
            SceneFile::parse_int(&o["children"], d.children).clamp(1, 32), num("embed", d.embed).clamp(0., 1.), num("rise", d.rise).max(0.));
        let mut surface = Surface::new(seed.wrapping_add(77));
        let billows = num("billows", 1.).max(0.);
        surface.billows.iter_mut().for_each(|b| b.1 *= billows);
        surface.wisps = num("wisps", surface.wisps).max(0.);
        surface.wisp_size = num("wisp_size", surface.wisp_size).max(1.);
        let centre = SceneFile::parse_vec2_def(o, "center", Vector2::zeros());
        let wind = SceneFile::parse_vec2_def(o, "wind_direction", Vector2::new(1., 0.));
        let mut c = if o["tower"].as_bool() == Some(true) {
            Cumulus::tower(tree, surface, centre, num("base", 1000.), num("width", 1500.).max(10.), num("depth", 6000.).max(10.),
                num("lift", 0.35).clamp(0., 1.), num("lean", 0.).max(0.), wind, o["glaciation"].as_f64(), seed)
        } else {
            Cumulus::new(tree, surface, centre, num("base", 1000.), num("width", 800.).max(10.), num("depth", 1500.).max(10.),
                num("lift", 0.35).clamp(0., 1.), num("dome", 0.8).max(0.), num("lean", 0.25).max(0.), wind, seed)
        };
        c.maturity = num("maturity", 1.).clamp(0., 1.);
        c.max_density = num("density", c.max_density).max(0.);
        c.edge = num("edge", c.edge).max(1.);
        let bounds = c.bounds();
        create_volume(bounds, Arc::new(c), SceneFile::parse_scattering(o, 0.85))
    }

    /// A layer of altocumulus as Rayleigh-Bénard convection cells (see
    /// `procedural::benard_altocumulus`), filling `extent` either side of `center` ([x, z]):
    ///
    /// `{"type": "benard_altocumulus", "base": 4000, "thickness": 200, "seed": 0}`
    ///
    /// with `density` (extinction in the densest cloud), `cell_size` (between cells' centres;
    /// default 2.5 times the thickness), `coverage` (how far each cloudlet reaches towards its
    /// cell's edge: around 0.5 broken, 1 joined), `regularity` (0 to 1), `anisotropy` (cells
    /// this many times longer along the wind) and `wind_direction` ([x, z]), `undulation`
    /// (0 to 1) and `wavelength` (waves across the wind), `patchiness` (0 to 1) and
    /// `patch_size`, `warp` (in cells), `lumpiness` (0 to 1), `detail_size` and `erosion` (0 to
    /// 1), `fade` (the fraction of `extent` over which the layer fades out), and
    /// `particle_radius` (µm, for `diffraction`), and `castellanus` (0 to 1, the share of cells
    /// growing turrets) with `turret_height` (in layer thicknesses). `mapping` as for `nubis_clouds` (`"regional"` or `"flat"`).
    pub fn parse_benard_altocumulus(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut c = BenardAltocumulus::new(num("base", 4000.), num("thickness", 200.).max(1.), SceneFile::parse_seed(o).unwrap_or(0));
        c.max_density = num("density", c.max_density);
        c.cell_size = num("cell_size", c.cell_size).max(1.);
        c.coverage = num("coverage", c.coverage).max(0.);
        c.regularity = num("regularity", c.regularity).clamp(0., 1.);
        c.anisotropy = num("anisotropy", c.anisotropy).max(0.1);
        c.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", c.wind_direction).normalize();
        c.undulation = num("undulation", c.undulation).clamp(0., 1.);
        c.wavelength = num("wavelength", c.wavelength).max(1.);
        c.patchiness = num("patchiness", c.patchiness).clamp(0., 1.);
        c.patch_size = num("patch_size", c.patch_size).max(1.);
        c.warp = num("warp", c.warp).max(0.);
        c.lumpiness = num("lumpiness", c.lumpiness).clamp(0., 1.);
        c.detail_size = num("detail_size", c.detail_size).max(1e-3);
        c.erosion = num("erosion", c.erosion).clamp(0., 1.);
        c.extent = num("extent", c.extent);
        c.center = SceneFile::parse_vec2_def(o, "center", c.center);
        c.fade = num("fade", c.fade).clamp(0., 1.);
        c.particle_radius = o["particle_radius"].as_f64();
        c.castellanus = num("castellanus", c.castellanus).clamp(0., 1.);
        c.turret_height = num("turret_height", c.turret_height).max(1.);
        let bounds = BBox::new(
            Vector3::new(c.center.x - c.extent, c.bottom(), c.center.y - c.extent),
            Vector3::new(c.center.x + c.extent, c.top(), c.center.y + c.extent),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(c));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// Rain or virga falling from a cloud's base (see `procedural::precipitation`), within
    /// `radius` of `center` ([x, z]):
    ///
    /// `{"type": "precipitation", "base": 1500, "rain_rate": 8, "coverage": 0.5, "seed": 0}`
    ///
    /// with `ground`, `cell_size` (of the shafts), `fall_depth` (m to 1/e by evaporation:
    /// virga; by default it all reaches the ground), `shear` (drift per metre fallen) and
    /// `wind_direction` ([x, z]), `streak_size` and `streakiness` (0 to 1), `base_variation` (how
    /// far below the base, varying, the drops emerge from the cloud), and `fade` (the
    /// fraction of `radius` over which it fades out). The phase function defaults to `g` 0.85;
    /// `"rainbow": true` gives the drops' bows in sunlight. `kind`: "rain" (the default),
    /// "snow" (with `melting_level`, rain below it) or "hail" (`hail_size`, mm); `rain_rate`
    /// is then mm/h of water.
    pub fn parse_precipitation(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut r = Precipitation::new(num("base", 1500.), SceneFile::parse_seed(o).unwrap_or(0));
        r.ground = num("ground", r.ground);
        r.rain_rate = num("rain_rate", r.rain_rate).max(0.);
        r.cell_size = num("cell_size", r.cell_size).max(1.);
        r.coverage = num("coverage", r.coverage).clamp(0., 1.);
        r.fall_depth = num("fall_depth", r.fall_depth).max(1.);
        r.shear = num("shear", r.shear);
        r.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", r.wind_direction).normalize();
        r.streak_size = num("streak_size", r.streak_size).max(1.);
        r.streakiness = num("streakiness", r.streakiness).clamp(0., 1.);
        r.base_variation = num("base_variation", r.base_variation).max(0.);
        r.center = SceneFile::parse_vec2_def(o, "center", r.center);
        r.radius = num("radius", r.radius).max(1.);
        r.fade = num("fade", r.fade).clamp(0., 1.);
        use crate::procedural::precipitation::Kind;
        r.kind = match o["kind"].as_str() {
            None | Some("rain") => Kind::Rain,
            Some("snow") => Kind::Snow,
            Some("hail") => Kind::Hail(num("hail_size", 8.).max(1.)),
            Some(k) => panic!("precipitation's \"kind\" is rain, snow or hail, not {:?}", k),
        };
        r.melting_level = o["melting_level"].as_f64();
        // Without a shear of its own, it drifts by its fall speed: rain's default shear, scaled.
        if o.get("shear").is_none() {
            r.shear *= Kind::Rain.fall_speed() / r.kind.fall_speed();
        }
        // Drifting downwind as it falls: the bounds reach from where it leaves the cloud to
        // where it lands.
        let reach = r.radius + r.drift().abs();
        let bounds = BBox::new(
            Vector3::new(r.center.x - reach, r.ground, r.center.y - reach),
            Vector3::new(r.center.x + reach, r.base, r.center.y + reach),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(r));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// An overcast deck of bad weather, with scud beneath (see `procedural::overcast`),
    /// filling `extent` either side of `center` ([x, z]):
    ///
    /// `{"type": "overcast", "base": 900, "thickness": 2500, "density": 0.03, "seed": 0}`
    ///
    /// with `system_size` and `thickness_variation` (0 to 1: broad patches of thicker and
    /// thinner cloud), `lump_size`, `base_variation` (m) and `contrast` (0 to 1: the rounded
    /// masses on its underside, and how much thinner the cloud is between them), `anisotropy`
    /// and `wind_direction` ([x, z]: rolls along the wind), `breaks` (0 to 1: gaps), `softness`,
    /// `detail_size` and `erosion` (the ragged base), the scud's `scud` (0 to 1, how much),
    /// `scud_depth`, `scud_size` and `scud_density`, and `fade` (a fraction of `extent`).
    /// `mapping` as for `nubis_clouds`. Rain from it is a `precipitation` object with its base.
    pub fn parse_overcast(o: &Value) -> SceneObject {
        SceneFile::parse_overcast_with(o, vec![])
    }

    /// An overcast deck with hill clouds under it (`hill_cloud_field`) in the same volume, so
    /// that light diffused down through the deck reaches them; they must share its mapping.
    pub fn parse_overcast_with(o: &Value, hills: Vec<HillCloud>) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut c = Overcast::new(num("base", 900.), num("thickness", 2500.).max(1.), SceneFile::parse_seed(o).unwrap_or(0));
        c.max_density = num("density", c.max_density).max(0.);
        c.system_size = num("system_size", c.system_size).max(1.);
        c.thickness_variation = num("thickness_variation", c.thickness_variation).clamp(0., 1.);
        c.lump_size = num("lump_size", c.lump_size).max(1.);
        c.base_variation = num("base_variation", c.base_variation).max(0.);
        c.contrast = num("contrast", c.contrast).clamp(0., 1.);
        c.anisotropy = num("anisotropy", c.anisotropy).max(0.1);
        c.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", c.wind_direction).normalize();
        c.breaks = num("breaks", c.breaks).clamp(0., 1.);
        c.softness = num("softness", c.softness).max(0.);
        c.detail_size = num("detail_size", c.detail_size).max(1.);
        c.erosion = num("erosion", c.erosion).clamp(0., 1.);
        c.scud = num("scud", c.scud).clamp(0., 1.);
        c.scud_depth = num("scud_depth", c.scud_depth).max(1.);
        c.scud_size = num("scud_size", c.scud_size).max(1.);
        c.scud_density = num("scud_density", c.scud_density).max(0.);
        c.center = SceneFile::parse_vec2_def(o, "center", c.center);
        c.extent = num("extent", c.extent).max(1.);
        c.fade = num("fade", c.fade).clamp(0., 1.);
        let bottom = hills.iter().map(|h| h.bottom()).fold(c.bottom(), f64::min);
        let bounds = BBox::new(
            Vector3::new(c.center.x - c.extent, bottom, c.center.y - c.extent),
            Vector3::new(c.center.x + c.extent, c.top(), c.center.y + c.extent),
        );
        let field: Arc<dyn DensityField> = if hills.is_empty() {
            Arc::new(c)
        } else {
            let mut fields: Vec<Arc<dyn DensityField>> = vec![Arc::new(c)];
            fields.extend(hills.into_iter().map(|h| Arc::new(h) as Arc<dyn DensityField>));
            Arc::new(crate::volume::Sum { fields })
        };
        let (bounds, field) = SceneFile::curved(o, bounds, field);
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// Mammatus under an anvil (see `procedural::mammatus`), filling `extent` either side of
    /// `center` ([x, z]):
    ///
    /// `{"type": "mammatus", "base": 5000, "thickness": 2500, "edge_direction": [x, z],
    /// "edge_distance": 0, "seed": 0}`
    ///
    /// The anvil covers the sky where the distance along `edge_direction` is under
    /// `edge_distance`, give or take `edge_raggedness`, thinning over `edge_taper`. Its base
    /// can undulate (`base_variation` m over `base_size`), sag towards the core (`sag` m,
    /// reached `sag_distance` in from the edge) and thicken softly (`base_softness` m), and its
    /// flat top rise and fall (`top_variation` m). The pouches:
    /// `pouch_size` (apart), `pouch_depth`, `coverage` (fraction of cells with one), `regularity`, `clustering` (0 to 1)
    /// and `cluster_size`,
    /// `pouch_band` (how far in from the edge they hang deepest), `edge_sharpness` and
    /// `roughness` (m) and `detail_size`. Extinctions: `anvil_density` and `pouch_density`. With
    /// `fade` (a fraction of `extent`), the sheet rounds off and fades out within its extent.
    /// `mapping` as for `nubis_clouds`.
    pub fn parse_mammatus(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut m = Mammatus::new(SceneFile::parse_seed(o).unwrap_or(0));
        m.base = num("base", m.base);
        m.thickness = num("thickness", m.thickness).max(1.);
        m.base_variation = num("base_variation", m.base_variation).max(0.);
        m.base_size = num("base_size", m.base_size).max(1.);
        m.sag = num("sag", m.sag).max(0.);
        m.sag_distance = num("sag_distance", m.sag_distance).max(1.);
        m.base_softness = num("base_softness", m.base_softness).max(0.);
        m.top_variation = num("top_variation", m.top_variation).max(0.);
        m.anvil_density = num("anvil_density", m.anvil_density).max(0.);
        m.pouch_density = num("pouch_density", m.pouch_density).max(0.);
        m.pouch_size = num("pouch_size", m.pouch_size).max(1.);
        m.pouch_depth = num("pouch_depth", m.pouch_depth).max(0.);
        m.coverage = num("coverage", m.coverage).clamp(0., 1.);
        m.regularity = num("regularity", m.regularity).clamp(0., 1.);
        m.clustering = num("clustering", m.clustering).clamp(0., 1.);
        m.cluster_size = num("cluster_size", m.cluster_size).max(1.);
        m.edge_sharpness = num("edge_sharpness", m.edge_sharpness).max(1.);
        m.roughness = num("roughness", m.roughness).max(0.);
        m.detail_size = num("detail_size", m.detail_size).max(1.);
        m.edge_direction = SceneFile::parse_vec2_def(o, "edge_direction", m.edge_direction).normalize();
        m.edge_distance = num("edge_distance", m.edge_distance);
        m.edge_raggedness = num("edge_raggedness", m.edge_raggedness).max(0.);
        m.edge_taper = num("edge_taper", m.edge_taper).max(1.);
        m.pouch_band = num("pouch_band", m.pouch_band).max(1.);
        let center = SceneFile::parse_vec2_def(o, "center", Vector2::zeros());
        let extent = num("extent", 60_000.);
        (m.center, m.extent) = (center, extent.max(1.));
        m.fade = num("fade", m.fade).clamp(0., 1.);
        let bounds = BBox::new(
            Vector3::new(center.x - extent, m.bottom(), center.y - extent),
            Vector3::new(center.x + extent, m.top(), center.y + extent),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(m));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.8))
    }

    /// Fractus, shreds of low cloud torn by the wind (see `procedural::fractus`):
    ///
    /// `{"type": "fractus", "base": 500, "depth": 600, "spacing": 1500, "coverage": 0.6,
    /// "size": 450, "stretch": 2.5, "wind_direction": [x, z], "raggedness": 0.6, "density": 0.03,
    /// "center": [x, z], "extent": 40000, "seed": 0}`; `mapping` as for `nubis_clouds`.
    pub fn parse_fractus(o: &Value) -> SceneObject {
        use crate::procedural::fractus::Fractus;
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut f = Fractus::new(SceneFile::parse_seed(o).unwrap_or(0));
        f.base = num("base", f.base);
        f.depth = num("depth", f.depth).max(0.);
        f.spacing = num("spacing", f.spacing).max(1.);
        f.coverage = num("coverage", f.coverage).clamp(0., 1.);
        f.size = num("size", f.size).max(1.);
        f.stretch = num("stretch", f.stretch).max(1.);
        f.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", f.wind_direction).normalize();
        f.raggedness = num("raggedness", f.raggedness).clamp(0., 1.);
        f.density = num("density", f.density).max(0.);
        f.center = SceneFile::parse_vec2_def(o, "center", f.center);
        f.extent = num("extent", f.extent).max(1.);
        let bounds = BBox::new(
            Vector3::new(f.center.x - f.extent, f.bottom(), f.center.y - f.extent),
            Vector3::new(f.center.x + f.extent, f.top(), f.center.y + f.extent),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(f));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// The clouds of a storm's gust front (see `procedural::arcus`): `"shelf_cloud"` (arcus), a
    /// wedge under the storm's base, or `"roll_cloud"` (volutus), a detached tube ahead of it.
    ///
    /// `{"type": "shelf_cloud", "center": [x, z], "motion": [x, z], "length": 30000,
    /// "waviness": 600, "low": 500, "base": 1400, "depth": 5000, "top": 3000, "face_slope": 1.6,
    /// "tier_spacing": 220, "tier_lip": 90, "scud": 180, "density": 0.04, "softness": 40,
    /// "seed": 0}`; a roll cloud takes `height`, `ahead` and `radius` in place of the shelf's
    /// heights. `center` is on the leading edge; `motion` is the way the front advances.
    /// `mapping` as for `nubis_clouds`.
    pub fn parse_arcus(o: &Value, roll: bool) -> SceneObject {
        use crate::procedural::arcus::{Arcus, Form};
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut a = Arcus::new(if roll { Form::Roll } else { Form::Shelf }, SceneFile::parse_seed(o).unwrap_or(0));
        a.center = SceneFile::parse_vec2_def(o, "center", a.center);
        a.motion = SceneFile::parse_vec2_def(o, "motion", a.motion).normalize();
        a.length = num("length", a.length).max(1.);
        a.waviness = num("waviness", a.waviness).max(0.);
        a.low = num("low", a.low);
        a.base = num("base", a.base).max(a.low);
        a.depth = num("depth", a.depth).max(1.);
        a.top = num("top", a.top).max(a.base);
        a.face_slope = num("face_slope", a.face_slope).max(0.);
        a.tier_spacing = num("tier_spacing", a.tier_spacing).max(1.);
        a.tier_lip = num("tier_lip", a.tier_lip).max(0.);
        a.scud = num("scud", a.scud).max(0.);
        a.height = num("height", a.height);
        a.ahead = num("ahead", a.ahead);
        a.radius = num("radius", a.radius).max(1.);
        a.density = num("density", a.density).max(0.);
        a.softness = num("softness", a.softness).max(1.);
        let r = a.reach();
        let bounds = BBox::new(
            Vector3::new(a.center.x - r, a.bottom(), a.center.y - r),
            Vector3::new(a.center.x + r, a.top_height(), a.center.y + r),
        );
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(a));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// A cumulonimbus, the thunderstorm cloud (see `procedural::cumulonimbus`):
    ///
    /// `{"type": "cumulonimbus", "center": [x, z], "base": 1200, "tropopause": 11500,
    /// "cape": 2500, "seed": 0}`
    ///
    /// with `efficiency` (the fraction of parcel theory's updraft realised) and `stability`
    /// (the stratosphere's buoyancy frequency, s⁻¹), which set the overshooting top; `radius` and
    /// `lean` (the main tower); `wind_direction` ([x, z]), `upper_wind` and `outflow_speed`
    /// (m/s) and `age` (s), which shape the anvil, and `anvil_thickness`; `flanking` (how many
    /// towers), `flank_angle` (degrees from upwind) and `flank_spacing`; `glaciation` (m);
    /// extinctions `density` (liquid) and `ice_density`; `billow_size`, `billowing` (0 to 1) and
    /// `softness`. `mapping` as for `nubis_clouds`.
    pub fn parse_cumulonimbus(o: &Value) -> SceneObject {
        use crate::procedural::cumulonimbus::Cumulonimbus;
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let mut c = Cumulonimbus::new(SceneFile::parse_seed(o).unwrap_or(0));
        c.center = SceneFile::parse_vec2_def(o, "center", c.center);
        c.base = num("base", c.base);
        c.tropopause = num("tropopause", c.tropopause).max(c.base + 1000.);
        c.cape = num("cape", c.cape).max(0.);
        c.efficiency = num("efficiency", c.efficiency).clamp(0., 1.);
        c.stability = num("stability", c.stability).max(1e-3);
        c.radius = num("radius", c.radius).max(100.);
        c.lean = num("lean", c.lean);
        c.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", c.wind_direction).normalize();
        c.upper_wind = num("upper_wind", c.upper_wind).max(0.);
        c.outflow_speed = num("outflow_speed", c.outflow_speed).max(0.);
        c.age = num("age", c.age).max(0.);
        c.anvil_thickness = num("anvil_thickness", c.anvil_thickness).max(0.);
        c.flanking = SceneFile::parse_int(&o["flanking"], c.flanking);
        c.flank_angle = num("flank_angle", c.flank_angle);
        c.flank_spacing = num("flank_spacing", c.flank_spacing).max(0.);
        c.glaciation = num("glaciation", c.glaciation);
        c.density = num("density", c.density).max(0.);
        c.ice_density = num("ice_density", c.ice_density).max(0.);
        c.billow_size = num("billow_size", c.billow_size).max(0.);
        c.billowing = num("billowing", c.billowing).clamp(0., 1.);
        c.softness = num("softness", c.softness).max(1.);
        c.build();
        let (lo, hi) = c.extent();
        let bounds = BBox::new(Vector3::new(lo.x, c.bottom(), lo.y), Vector3::new(hi.x, c.top(), hi.y));
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(c));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// Mountains (see `procedural::terrain`): a meshed heightfield of ridged noise around a
    /// massif, lying on the planet, with snow, rock and vegetation:
    ///
    /// `{"type": "mountains", "center": [x, z], "extent": 30000, "resolution": 768,
    /// "peak": [x, z], "peak_height": 3200, "peak_radius": 7000, "seed": 0}`
    ///
    /// with `relief` and `feature_size` (the ridges), `plains` (relief of the plains around),
    /// `base` (the height of its foot: below zero for an island in a `sea`),
    /// `snowline` and `treeline` (m), the colours `rock`, `forest` and `grass`, `snow` (by
    /// default physical snow, `material::snow`, settled and older near the snowline; an object
    /// of its makeup as for a snow material, eg. `{"preset": "fresh"}`; or a colour for diffuse
    /// snow), and `"mapping": "flat"` for flat ground rather than the planet's. A `"cloud"`
    /// raises hill cloud over it (`parse_hill_cloud`).
    pub fn parse_terrain(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let seed = SceneFile::parse_seed(o).unwrap_or(0);
        let t = SceneFile::parse_mountains(o);
        let mut m = TerrainMaterial::new(seed);
        m.planet_radius = t.planet_radius;
        m.snowline = num("snowline", m.snowline);
        m.treeline = num("treeline", m.treeline);
        match &o["snow"] {
            Value::Null => {},
            // An object of snow's makeup (`parse_snow_style`): for all the snow.
            Value::Object(_) => {
                let style = SceneFile::parse_snow_style(&o["snow"]);
                m.snow_materials = [Snow::new(style.clone(), 1., seed), Snow::new(style, 1., seed)];
            },
            c => m.snow = Some(SceneFile::parse_color(c)),
        }
        m.rock = SceneFile::parse_color_def(o, "rock", m.rock);
        m.forest = SceneFile::parse_color_def(o, "forest", m.forest);
        m.grass = SceneFile::parse_color_def(o, "grass", m.grass);
        SceneObject {
            geometry: Box::new(SmoothMesh::from_indexed(&[t.mesh()])),
            medium: Box::new(Solid { m: Box::new(m) }),
        }
    }

    /// Hill cloud over a `mountains` object's ground (see `procedural::hill_cloud`), from its
    /// `"cloud"`: `{"deficit": 350, "decay": 250, "density": 0.02, "wind_direction": [x, z],
    /// "trail": 1500, "humidity_variation": 0.25, "humidity_size": 6000, "detail_size": 150,
    /// "raggedness": 0.5, "resolution": 512, "seed": 0}`, with the volume's scattering (`albedo`,
    /// `g`, approximations) as for any cloud. In a scene with an `overcast`, the cloud joins
    /// the deck's volume (`parse_overcast_with`), scattering as the deck does, so that the
    /// diffusion approximation lights them as one column; `"join": false` keeps it apart.
    ///
    /// The air is saturated by a lift of `deficit` metres: hills taller than that disappear into
    /// cloud, which hugs them, `decay` metres being how fast the hills' lift of the air dies away
    /// above them, and streams `trail` metres downwind of the crests.
    pub fn parse_hill_cloud(o: &Value) -> Option<SceneObject> {
        let c = o.get("cloud")?;
        let h = SceneFile::hill_cloud_field(o)?;
        let terrain = SceneFile::parse_mountains(o);
        let bounds = BBox::new(
            Vector3::new(terrain.center.x - terrain.extent, h.bottom(), terrain.center.y - terrain.extent),
            Vector3::new(terrain.center.x + terrain.extent, h.top(), terrain.center.y + terrain.extent),
        );
        let mapping = if terrain.planet_radius.is_none() { serde_json::json!({"mapping": "flat"}) } else { o.clone() };
        let (bounds, field) = SceneFile::curved(&mapping, bounds, Arc::new(h));
        Some(create_volume(bounds, field, SceneFile::parse_scattering(c, 0.85)))
    }

    /// The density field of a `mountains` object's `"cloud"`, in the terrain's own (flat)
    /// coordinates.
    pub fn hill_cloud_field(o: &Value) -> Option<HillCloud> {
        let c = o.get("cloud")?;
        let terrain = SceneFile::parse_mountains(o);
        let num = |k: &str, d: f64| SceneFile::parse_number(&c[k], d);
        let resolution = SceneFile::parse_int(&c["resolution"], 512).max(2);
        let mut h = HillCloud::new(&terrain, resolution, SceneFile::parse_seed(c).unwrap_or(0));
        h.deficit = num("deficit", h.deficit).max(1.);
        h.humidity_variation = num("humidity_variation", h.humidity_variation).clamp(0., 0.9);
        h.humidity_size = num("humidity_size", h.humidity_size).max(1.);
        h.decay = num("decay", h.decay).max(1.);
        h.max_density = num("density", h.max_density).max(0.);
        h.wind_direction = SceneFile::parse_vec2_def(c, "wind_direction", h.wind_direction).normalize();
        h.trail = num("trail", h.trail).max(0.);
        h.detail_size = num("detail_size", h.detail_size).max(1.);
        h.raggedness = num("raggedness", h.raggedness).clamp(0., 1.);
        Some(h)
    }

    /// The heightfield of a `mountains` object.
    pub fn parse_mountains(o: &Value) -> Mountains {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let seed = SceneFile::parse_seed(o).unwrap_or(0);
        let flat = o["mapping"].as_str() == Some("flat");
        let mut t = Mountains::new(seed);
        t.center = SceneFile::parse_vec2_def(o, "center", t.center);
        t.extent = num("extent", t.extent).max(1.);
        t.resolution = SceneFile::parse_int(&o["resolution"], t.resolution).max(2);
        t.peak = SceneFile::parse_vec2_def(o, "peak", t.center);
        t.peak_height = num("peak_height", t.peak_height);
        t.peak_radius = num("peak_radius", t.peak_radius).max(1.);
        t.relief = num("relief", t.relief);
        t.feature_size = num("feature_size", t.feature_size).max(1.);
        t.plains = num("plains", t.plains);
        t.base = num("base", t.base);
        t.planet_radius = if flat { None } else { Some(SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS)) };
        t
    }

    /// Lenticular clouds over the lee waves of a ridge (see `procedural::lenticular`):
    ///
    /// `{"type": "lenticular", "ridge": [x, z], "ridge_length": 6000, "wind_direction": [x, z],
    /// "layers": [{"height": 4000, "depth": 320, "deficit": 190}, ...], "seed": 0}`
    ///
    /// Each layer is a moist layer: its middle's height, its depth and the lift needed to
    /// saturate its middle (all in metres); each makes a lens on every crest the waves lift it
    /// high enough. The waves: `wavelength`, `amplitude` (the largest lift), `decay` (downstream
    /// distance to 1/e), `first_crest` (in wavelengths downstream of the ridge), `falloff` and
    /// `spread` (how they fade beyond the ridge's ends), `bend` and `bend_size` (how far the
    /// crests wander). The air: `humidity_variation` and `humidity_size`, `strata` and
    /// `strata_spacing`, `roughness` and `detail_size` (all lifts in metres),
    /// `condensation_rate` (g/kg per km of lift), `droplet_radius` (µm), and `wind_speed` and
    /// `time` to move the fine structure through the standing clouds. `mapping` as for
    /// `nubis_clouds`.
    pub fn parse_lenticular(o: &Value) -> SceneObject {
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let layers = match o["layers"].as_array() {
            None => Lenticular::default_layers(),
            Some(ls) => ls.iter().map(|l| MoistLayer {
                height: SceneFile::parse_number(&l["height"], 4000.),
                depth: SceneFile::parse_number(&l["depth"], 300.).max(1.),
                deficit: SceneFile::parse_number(&l["deficit"], 200.),
            }).collect(),
        };
        let mut c = Lenticular::new(layers, SceneFile::parse_seed(o).unwrap_or(0));
        c.ridge = SceneFile::parse_vec2_def(o, "ridge", c.ridge);
        c.ridge_length = num("ridge_length", c.ridge_length).max(0.);
        c.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", c.wind_direction).normalize();
        c.wind_speed = num("wind_speed", c.wind_speed);
        c.wavelength = num("wavelength", c.wavelength).max(100.);
        c.amplitude = num("amplitude", c.amplitude).max(0.);
        c.decay = num("decay", c.decay).max(1.);
        c.first_crest = num("first_crest", c.first_crest);
        c.falloff = num("falloff", c.falloff).max(1.);
        c.spread = num("spread", c.spread).max(0.);
        c.bend = num("bend", c.bend).max(0.);
        c.bend_size = num("bend_size", c.bend_size).max(1.);
        c.humidity_variation = num("humidity_variation", c.humidity_variation).max(0.);
        c.humidity_size = num("humidity_size", c.humidity_size).max(1.);
        c.strata = num("strata", c.strata).max(0.);
        c.strata_spacing = num("strata_spacing", c.strata_spacing).max(1.);
        c.roughness = num("roughness", c.roughness).max(0.);
        c.detail_size = num("detail_size", c.detail_size).max(1.);
        c.condensation_rate = num("condensation_rate", c.condensation_rate).max(0.);
        c.droplet_radius = num("droplet_radius", c.droplet_radius).max(0.5);
        c.time = num("time", c.time);
        // The waves reach no cloud once the decayed amplitude is below the least deficit.
        let least = c.layers.iter().map(|l| l.deficit).fold(f64::INFINITY, f64::min) - c.humidity_variation - c.strata - c.roughness;
        let reach = if least > 0. && c.amplitude > least { c.decay * (c.amplitude / least).ln() } else { 10. * c.decay };
        let (s0, s1) = (-0.3 * c.wavelength, reach + c.wavelength);
        let half = c.ridge_length + 4. * (c.falloff + c.spread * s1);
        let w = c.wind_direction;
        let corners = [(s0, -half), (s0, half), (s1, -half), (s1, half)]
            .map(|(s, a)| Vector2::new(c.ridge.x + s * w.x - a * w.y, c.ridge.y + s * w.y + a * w.x));
        let lo = corners.iter().fold(Vector2::repeat(f64::INFINITY), |m, p| m.inf(p));
        let hi = corners.iter().fold(Vector2::repeat(f64::NEG_INFINITY), |m, p| m.sup(p));
        let bounds = BBox::new(Vector3::new(lo.x, c.bottom(), lo.y), Vector3::new(hi.x, c.top(), hi.y));
        let (bounds, field) = SceneFile::curved(o, bounds, Arc::new(c));
        create_volume(bounds, field, SceneFile::parse_scattering(o, 0.85))
    }

    /// Bounds of the shell of atmosphere up to `top` around the whole of `planet`.
    pub fn shell(planet: &Planet, top: f64) -> BBox {
        let r = Vector3::repeat(planet.radius + top);
        BBox::new(planet.center() - r, planet.center() + r)
    }

    /// A layer of cirrus by Hogan and Kew's fallstreak model (see
    /// `procedural::hogan_kew_cirrus`), filling `extent` either side of `center` ([x, z]):
    ///
    /// `{"type": "hogan_kew_cirrus", "genus": "cirrus" | "cirrostratus" | "cirrocumulus",
    /// "base": 8000, "top": 10000, "wind_direction": [x, z], "seed": 0}`, filling `extent` either
    /// side of `center` (by default out to the layer's horizon, about 357 km for a 10 km top)
    ///
    /// The genus sets the rest, each of which can be overridden: `coverage`, `cell_size`,
    /// `spectral_slope`, `anisotropy` (the generating cells); `density`, `generating_depth`,
    /// `fall_depth`, `sublimation` (the ice); `shear`, `generating_wind`, `fall_speed`,
    /// `fall_speed_spread`, `veer` (degrees the wind turns down through the layer: hooks) (the
    /// fallstreaks); `flow_size` and `flow` (the large-scale flow that curves them);
    /// `fibre_width` and `fibres` (0 to 1: how wispy); `edge`,
    /// `turbulence_size`, `turbulence`; `patchiness` and `patch_size`; `particle_radius` (µm, at
    /// the generating level), `particle_growth` (relative growth by the base) and `young_size`
    /// (relative size at the cells' edges, where ice is newest), for `diffraction`; `fade`, the fraction of
    /// `extent` over which the layer fades out at its edges (default 0.3); and `time` and
    /// `evolution_period` (seconds) to animate.
    /// `"mapping": "global"` puts the layer round the whole planet (`GlobalCirrus`), its streaks
    /// following the planet's winds (`GlobalWind`: bands by latitude about the `pole`, plus
    /// `eddies` of strength relative to the jets, `eddy_size` across); `"regional"` (the
    /// default) and `"flat"` as for `nubis_clouds`.
    /// The phase function defaults to `g` 0.75, for ice crystals; `"halos"` adds sun dogs and
    /// halos (see `SceneFile::parse_halos`).
    /// Hogan and Kew's cirrus as a scene file describes it (see `parse_hogan_kew_cirrus`), by
    /// default of `genus`: the layer, and its generating cells' size, spectral slope,
    /// coverage and seed.
    pub fn hogan_kew_layer(o: &Value, genus: CirrusGenus) -> (HoganKewCirrus, (f64, f64, f64, u64)) {
        let genus = match o["genus"].as_str() {
            None => genus,
            Some("cirrus") => CirrusGenus::Cirrus,
            Some("cirrostratus") => CirrusGenus::Cirrostratus,
            Some("cirrocumulus") => CirrusGenus::Cirrocumulus,
            Some(g) => panic!("Unknown cirrus genus '{}': expected cirrus, cirrostratus or cirrocumulus", g),
        };
        let mut f = CirrusField::new(genus);
        let species = o["species"].as_str().map(|n| CirrusSpecies::parse(n)
            .unwrap_or_else(|| panic!("Unknown cirrus species '{}': expected uncinus, fibratus, floccus or spissatus", n)));
        if let Some(s) = species { s.field(&mut f); }
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        f.base = num("base", f.base);
        f.top = num("top", f.top);
        f.coverage = num("coverage", f.coverage).clamp(0., 1.);
        f.cell_size = num("cell_size", f.cell_size);
        f.spectral_slope = num("spectral_slope", f.spectral_slope);
        f.anisotropy = num("anisotropy", f.anisotropy);
        f.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", f.wind_direction);
        f.seed = SceneFile::parse_seed(o).unwrap_or(0);
        let generating = (f.cell_size, f.spectral_slope, f.coverage, f.seed);
        let mut c = HoganKewCirrus::new(f);
        if let Some(s) = species { s.fall(&mut c); }
        c.max_density = num("density", c.max_density);
        c.generating_depth = num("generating_depth", c.generating_depth);
        c.fall_depth = num("fall_depth", c.fall_depth);
        c.sublimation = num("sublimation", c.sublimation);
        c.shear = num("shear", c.shear);
        c.generating_wind = num("generating_wind", c.generating_wind);
        c.fall_speed = num("fall_speed", c.fall_speed);
        c.fall_speed_spread = num("fall_speed_spread", c.fall_speed_spread).clamp(0., 0.95);
        c.edge = num("edge", c.edge).max(1e-9);
        c.veer = num("veer", c.veer.to_degrees()).to_radians();
        c.flow_size = num("flow_size", c.flow_size);
        c.flow = num("flow", c.flow).max(0.);
        c.fibre_width = num("fibre_width", c.fibre_width).max(1.);
        c.fibres = num("fibres", c.fibres).clamp(0., 1.);
        c.turbulence_size = num("turbulence_size", c.turbulence_size);
        c.turbulence = num("turbulence", c.turbulence);
        c.time = num("time", c.time);
        c.evolution_period = num("evolution_period", c.evolution_period);
        c.patchiness = num("patchiness", c.patchiness).clamp(0., 1.);
        c.particle_radius = num("particle_radius", c.particle_radius);
        c.particle_growth = num("particle_growth", c.particle_growth);
        c.young_size = num("young_size", c.young_size).clamp(0.05, 1.);
        c.patch_size = num("patch_size", c.patch_size);
        (c, generating)
    }

    pub fn parse_hogan_kew_cirrus(o: &Value) -> SceneObject {
        let (mut c, generating) = SceneFile::hogan_kew_layer(o, CirrusGenus::Cirrus);
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let (base, top) = (c.base, c.top);
        // By default the layer reaches its own horizon, √(2 R top), so it runs down into the
        // haze there rather than stopping short in clear sky (a 9 km deck stays above the horizon
        // out to 340 km).
        let horizon = (2. * SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS) * top + top * top).sqrt();
        let extent = num("extent", horizon);
        let center = SceneFile::parse_vec2_def(o, "center", Vector2::new(0., 0.));
        c.extent = extent;
        c.center = center;
        c.fade = num("fade", c.fade).clamp(0., 1.);
        let bounds = BBox::new(
            Vector3::new(center.x - extent, base, center.y - extent),
            Vector3::new(center.x + extent, top, center.y + extent),
        );
        let (bounds, field): (BBox, Arc<dyn DensityField>) = if o["mapping"].as_str() == Some("global") {
            let planet = Planet { radius: SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS) };
            let pole = SceneFile::parse_vec3_def(o, "pole", Vector3::z()).normalize();
            let (cell, slope, coverage, seed) = generating;
            let mut global = GlobalCirrus::new(c, cell, slope, coverage, seed, planet.radius, pole);
            global.wind.eddy_size = num("eddy_size", global.wind.eddy_size);
            global.wind.eddies = num("eddies", global.wind.eddies);
            (SceneFile::shell(&planet, top), Arc::new(GlobalMapping { planet, field: Arc::new(global) }))
        } else {
            SceneFile::curved(o, bounds, Arc::new(c))
        };
        // With diffraction, the phase function is that of the rest: refraction and reflection.
        let g = if o.get("diffraction").is_some_and(|d| d != &Value::Bool(false)) { 0.5 } else { 0.75 };
        create_volume(bounds, field, SceneFile::parse_scattering(o, g))
    }

    /// A cloud layer's field and bounds, curved round the planet (`planet::PlanetMapping`)
    /// unless `"curved": false`: its heights become altitudes above a sphere of
    /// `earth_radius` (default the Earth's) beneath the origin, so that it drops away with
    /// distance, and far enough off is hidden below the horizon, as on a planet.
    pub fn curved(o: &Value, bounds: BBox, field: Arc<dyn DensityField>) -> (BBox, Arc<dyn DensityField>) {
        if o["curved"].as_bool() == Some(false) || o["mapping"].as_str() == Some("flat") {
            return (bounds, field);
        }
        let planet = Planet { radius: SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS) };
        (planet.bounds_of(&bounds), Arc::new(PlanetMapping { planet, field }))
    }

    pub fn parse_box_terrain(o: &Value) -> SceneObject {
        create_box_terrain(SceneFile::parse_int(&o["seed"], 0) as u64)
    }

    /// A geometry on its own, ignoring the materials of any parts. A geometry definition, or the
    /// name of one in `geometries`, which is shared rather than built again.
    pub fn parse_geometry (o: &Value, geometries: &GeometryLibrary) -> Option<Box<dyn Geometry + Sync + Send>> {
        SceneFile::parse_geometry_in(o, &mut Parts { library: None, media: vec![], geometries }, false)
    }

    /// A geometry, turned inside out if it has `"inverse": true` and with its surface cut down to
    /// the inside of `"clipped_by"` if it has one. A `nested` geometry - a part of another - with
    /// a "material" or "medium" of its own becomes a numbered part of the object, its medium
    /// collected in `parts`.
    pub fn parse_geometry_in (o: &Value, parts: &mut Parts, nested: bool) -> Option<Box<dyn Geometry + Sync + Send>> {
        let mut g = SceneFile::parse_bare_geometry(o, parts)?;
        if !o["clipped_by"].is_null() {
            let by = SceneFile::parse_geometry_in(&o["clipped_by"], parts, true)
                .unwrap_or_else(|| panic!("clipped_by needs a shape, got {}", o["clipped_by"]));
            g = Box::new(Clipped::new(g, by));
        }
        if o["inverse"].as_bool() == Some(true) {
            g = Box::new(Inverse(g));
        }
        // Steps on the shape itself, as POV-Ray's transformations on an object: they move it
        // whole, including any clipping.
        if !o["transform"].is_null() {
            g = Box::new(Transform::new(g, SceneFile::parse_steps(&o["transform"])));
        }
        if let Some((materials, media)) = parts.library
            && nested && (o.get("material").is_some() || o.get("medium").is_some()) {
            parts.media.push(SceneFile::parse_object_medium(o, materials, media));
            g = Box::new(PartTagged { item: g, part: parts.media.len() as u32 });
        }
        Some(g)
    }

    fn parse_bare_geometry (o: &Value, parts: &mut Parts) -> Option<Box<dyn Geometry + Sync + Send>> {
        let geometries = parts.geometries;
        if let Value::String(name) = o {
            return Some(Box::new(geometries.get(name)));
        }
        let t = o["type"].as_str().unwrap();

        if t == "sphere" {
            return Some(SceneFile::parse_sphere(o));
        }

        if t == "triangle" {
            return Some(SceneFile::parse_triangle(o));
        }

        if t == "mesh" || t == "smoothmesh" {
            return Some(Box::new(geometries.mesh(o)));
        }

        if t == "box" {
            return Some(SceneFile::parse_box(o));
        }

        if t == "plane" {
            return Some(SceneFile::parse_plane(o));
        }

        if t == "rotate" {
            return Some(SceneFile::parse_rotation(o, parts));
        }

        if t == "transform" {
            return Some(SceneFile::parse_transform(o, parts));
        }

        if let Some(op) = match t {
            "union" => Some(Op::Union),
            "merge" => Some(Op::Merge),
            "intersection" => Some(Op::Intersection),
            "difference" => Some(Op::Difference),
            _ => None,
        } {
            return Some(SceneFile::parse_csg(op, o, parts));
        }

        if t == "terrain" {
            let grid = Arc::new(HeightGrid::from_json(&o["heightmap"]));
            if let Some(d) = o.get("displacement") {
                return Some(Box::new(SceneFile::parse_displaced_terrain(grid, d, o)));
            }
            return Some(Box::new(Terrain { grid }));
        }
        SceneFile::parse_shape(t, o)
    }

    /// As `parse_geometry`, but a named geometry is returned as the shared original itself.
    pub fn parse_shared_geometry(o: &Value, geometries: &GeometryLibrary) -> Option<SharedGeometry> {
        SceneFile::parse_shared_geometry_in(o, &mut Parts { library: None, media: vec![], geometries })
    }

    /// As `parse_geometry_in` for a part, but a named geometry (or a mesh file) is returned as the
    /// shared original itself.
    pub fn parse_shared_geometry_in(o: &Value, parts: &mut Parts) -> Option<SharedGeometry> {
        match o {
            Value::String(name) => Some(parts.geometries.get(name)),
            _ if MeshKey::of(o).is_some() && o["inverse"].is_null() && o["clipped_by"].is_null()
                && o.get("material").is_none() && o.get("medium").is_none() => Some(parts.geometries.mesh(o)),
            _ => SceneFile::parse_geometry_in(o, parts, true).map(Arc::from),
        }
    }

    /// `{"type": "union" | "merge" | "intersection" | "difference", "items": [shapes]}`: a
    /// difference is its first shape less the others. A difference can also be written with two
    /// shapes, "a" less "b".
    pub fn parse_csg(op: Op, o: &Value, parts: &mut Parts) -> Box<dyn Geometry + Sync + Send> {
        let mut shape = |v: &Value| SceneFile::parse_geometry_in(v, parts, true)
            .unwrap_or_else(|| panic!("{} items must be shapes, got {}", o["type"], v));
        let parts: Vec<Box<dyn Geometry + Sync + Send>> = match (o["items"].as_array(), op) {
            (Some(items), _) => items.iter().map(shape).collect(),
            (None, Op::Difference) if !o["a"].is_null() => vec![shape(&o["a"]), shape(&o["b"])],
            _ => panic!("{} needs a list of \"items\"", o["type"]),
        };
        assert!(!parts.is_empty(), "{} needs at least one item", o["type"]);
        Box::new(Csg::new(op, parts))
    }

    pub fn parse_rotation(o: &Value, parts: &mut Parts) -> Box<dyn Geometry + Sync + Send> {
        let a = SceneFile::parse_shared_geometry_in(&o["item"], parts).unwrap(); // Panic if fails
        let roll = SceneFile::parse_number(&o["roll"], 0.).to_radians();
        let pitch = SceneFile::parse_number(&o["pitch"], 0.).to_radians();
        let yaw = SceneFile::parse_number(&o["yaw"], 0.).to_radians();
        Box::new(Transform::rotate(a, roll, pitch, yaw))
    }

    /// `{"type": "transform", "item": {...}, "scale": 2 or [x, y, z], "rotate": {"roll", "pitch",
    /// "yaw"} in degrees, "translate": [x, y, z]}`: every part optional, applied in that order.
    /// With `"item"` the name of one of the scene's `geometries`, it's an instance of it.
    ///
    /// Or `{"type": "transform", "item": {...}, "steps": [step, ..]}`, applying each step in turn
    /// (see `parse_steps`).
    ///
    /// With `"motion": {...}`, the object moves while the shutter is open (`motion`): `motion`
    /// holds the transform's keys at the end of the motion, each defaulting to its value at the
    /// start, eg. `"translate": [0, 0, 0], "motion": {"translate": [1, 0, 0]}`.
    pub fn parse_transform(o: &Value, parts: &mut Parts) -> Box<dyn Geometry + Sync + Send> {
        let item = SceneFile::parse_shared_geometry_in(&o["item"], parts)
            .unwrap_or_else(|| panic!("transform needs a geometry \"item\", got {}", o["item"]));
        let start = SceneFile::parse_affine(o);
        match o.get("motion") {
            None => Box::new(Transform::new(item, start)),
            Some(Value::Object(m)) => {
                let mut end = o.clone();
                for (k, v) in m {
                    end[k.as_str()] = v.clone();
                }
                // Steps replace the parts, and the parts the steps.
                if m.contains_key("steps") {
                    for k in ["scale", "rotate", "translate"] { end[k] = Value::Null; }
                } else if ["scale", "rotate", "translate"].iter().any(|k| m.contains_key(*k)) {
                    end["steps"] = Value::Null;
                }
                Box::new(crate::motion::AnimatedTransform::new(item, start, SceneFile::parse_affine(&end)))
            },
            Some(v) => panic!("A transform's \"motion\" must be an object of its keys at the end, got {}", v),
        }
    }

    /// The affine map of a transform object's `"steps"`, or of its `"scale"`, `"rotate"` and
    /// `"translate"`.
    fn parse_affine(o: &Value) -> Affine3<f64> {
        if !o["steps"].is_null() {
            assert!(["scale", "rotate", "translate"].iter().all(|k| o[k].is_null()),
                "transform takes \"steps\", or \"scale\", \"rotate\" and \"translate\", not both");
            return SceneFile::parse_steps(&o["steps"]);
        }
        let scale = match &o["scale"] {
            Value::Number(n) => { let s = n.as_f64().unwrap(); Vector3::new(s, s, s) },
            Value::Null => Vector3::new(1., 1., 1.),
            v => SceneFile::parse_vec3(v),
        };
        let r = &o["rotate"];
        let rotation = Rotation3::from_euler_angles(
            SceneFile::parse_number(&r["roll"], 0.).to_radians(),
            SceneFile::parse_number(&r["pitch"], 0.).to_radians(),
            SceneFile::parse_number(&r["yaw"], 0.).to_radians(),
        );
        let translate = SceneFile::parse_vec3_def(o, "translate", Vector3::zeros());
        assert!(scale.x != 0. && scale.y != 0. && scale.z != 0., "transform scale must be non-zero on every axis, got {}", scale);
        Affine3::from_matrix_unchecked(Translation3::from(translate).to_homogeneous()
            * rotation.to_homogeneous()
            * Matrix4::new_nonuniform_scaling(&scale))
    }

    /// A list of transformation steps, applied in order:
    /// - `{"translate": [x, y, z]}`
    /// - `{"rotate": [x, y, z]}`: degrees about the x axis, then y, then z, each anticlockwise
    ///   looking down the axis towards the origin (the right-hand rule). `{"roll", "pitch",
    ///   "yaw"}` names the same angles.
    /// - `{"scale": s}` or `{"scale": [x, y, z]}`
    /// - `{"axis_rotate": {"axis": [x, y, z], "angle": degrees}}`: about any axis through the
    ///   origin.
    /// - `{"matrix": [12 numbers]}`, in POV-Ray's order: rows of the 3x3 linear part, then the
    ///   translation, so that (x, y, z) goes to x row1 + y row2 + z row3 + row4.
    pub fn parse_steps(v: &Value) -> Affine3<f64> {
        let steps = v.as_array().unwrap_or_else(|| panic!("transform steps must be a list, got {}", v));
        steps.iter().fold(Affine3::identity(), |m, step| SceneFile::parse_step(step) * m)
    }

    fn parse_step(step: &Value) -> Affine3<f64> {
        let keys: Vec<&String> = step.as_object().map(|o| o.keys().collect()).unwrap_or_default();
        assert!(keys.len() == 1, "each transform step has one of \"translate\", \"rotate\", \"scale\", \"axis_rotate\" or \"matrix\", got {}", step);
        let v = &step[keys[0].as_str()];
        let m: Matrix4<f64> = match keys[0].as_str() {
            "translate" => Translation3::from(SceneFile::parse_vec3(v)).to_homogeneous(),
            "rotate" => {
                let a = if v.is_object() {
                    Vector3::new(SceneFile::parse_number(&v["roll"], 0.), SceneFile::parse_number(&v["pitch"], 0.), SceneFile::parse_number(&v["yaw"], 0.))
                } else {
                    SceneFile::parse_vec3(v)
                };
                Rotation3::from_euler_angles(a.x.to_radians(), a.y.to_radians(), a.z.to_radians()).to_homogeneous()
            },
            "scale" => {
                let s = match v {
                    Value::Number(n) => Vector3::repeat(n.as_f64().unwrap()),
                    _ => SceneFile::parse_vec3(v),
                };
                Matrix4::new_nonuniform_scaling(&s)
            },
            "axis_rotate" => {
                let axis = SceneFile::parse_vec3(&v["axis"]);
                assert!(axis.norm() > 0., "axis_rotate needs a non-zero \"axis\"");
                Rotation3::from_axis_angle(&na::Unit::new_normalize(axis), SceneFile::parse_number(&v["angle"], 0.).to_radians()).to_homogeneous()
            },
            "matrix" => {
                let n: Vec<f64> = v.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default();
                assert!(n.len() == 12, "a transform matrix has 12 numbers, got {}", v);
                // Row-vector convention: the rows of POV-Ray's matrix are our columns.
                Matrix4::new(
                    n[0], n[3], n[6], n[9],
                    n[1], n[4], n[7], n[10],
                    n[2], n[5], n[8], n[11],
                    0., 0., 0., 1.)
            },
            k => panic!("unknown transform step \"{}\"", k),
        };
        Affine3::from_matrix_unchecked(m)
    }

    pub fn parse_sphere(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        Box::new(Sphere::new(
                SceneFile::parse_vec3(&o["location"]),
                o["radius"].as_f64().unwrap()))
    }

    pub fn parse_box(o: &Value) -> Box<dyn Geometry + Sync + Send>{
        let a = SceneFile::parse_vec3(&o["min"]);
        let b = SceneFile::parse_vec3(&o["max"]);
        // Accept corners in either order.
        Box::new(BBox::new(a.inf(&b), a.sup(&b)))
    }

    pub fn parse_triangle(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        Box::new(Triangle::new(
                SceneFile::parse_vec3(&o["v0"]),
                SceneFile::parse_vec3(&o["v1"]),
                SceneFile::parse_vec3(&o["v2"])))
    }

    /// `{"type": "plane", "y": 0}`, a ground plane; or `{"type": "plane", "normal": [x, y, z],
    /// "distance": d}`, POV-Ray's plane: the half-space n.p < d, which can be used in CSG. A
    /// `"normal"` that is an object is a bump map (`parse_object_medium`), not an orientation.
    pub fn parse_plane(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        if o["normal"].is_array() {
            return Box::new(crate::shapes::plane::HalfSpace::new(
                SceneFile::parse_vec3(&o["normal"]), SceneFile::parse_number(&o["distance"], 0.)));
        }
        Box::new(Plane {
                y: SceneFile::parse_number(&o["y"], 0.),
            })
    }

    /// `{"type": "leaf", "reflectance": colour, "transmittance": colour, "refractive_index": 1.5,
    /// "roughness": 0.3, "autumn": {"reflectance", "transmittance"}, "autumn_sun": {...},
    /// "dead": {...}}`; see `Leaf`. Every colour's reflectance and transmittance must sum to at
    /// most 1.
    pub fn parse_leaf(o: &Value) -> Leaf {
        let color = |v: &Value, default: LeafColor, name: &str| -> LeafColor {
            let c = LeafColor {
                reflectance: SceneFile::parse_color_def(v, "reflectance", default.reflectance),
                transmittance: SceneFile::parse_color_def(v, "transmittance", default.transmittance),
            };
            let total = c.reflectance + c.transmittance;
            assert!(total.rgb.max() <= 1.,
                "A leaf can't reflect and transmit more light than it receives: {} reflectance + transmittance is {}", name, total);
            c
        };
        let grey = Color::new(0.25, 0.25, 0.25);
        let green = color(o, LeafColor { reflectance: grey, transmittance: grey }, "its");
        let mut leaf = Leaf::new(green.reflectance, green.transmittance,
            SceneFile::parse_number(&o["refractive_index"], 1.5),
            SceneFile::parse_number(&o["roughness"], 0.3));
        leaf.autumn = color(&o["autumn"], DEFAULT_AUTUMN, "autumn");
        leaf.autumn_sun = if o["autumn_sun"].is_null() { leaf.autumn } else { color(&o["autumn_sun"], leaf.autumn, "autumn_sun") };
        leaf.dead = color(&o["dead"], DEFAULT_DEAD, "dead");
        leaf
    }

    pub fn parse_checkeredplane(o: &Value, m: Box<dyn Medium + Sync + Send>) -> SceneObject {
        SceneObject {
            geometry: Box::new(Plane { y: o["y"].as_f64().unwrap() }),
            medium: m
        }
    }

    /// Look up a material by name. None if there is no such material; panics if it exists but
    /// has an unknown type.
    pub fn parse_material_ref(key: &Value, materials: &Map<String, Value> ) -> Option<Box<dyn MaterialModel + Sync + Send>> {
        let name = SceneFile::parse_string(key);
        let props = materials.get(&name)?;
        if props["type"].as_str() == Some("noise") {
            return Some(Box::new(SceneFile::parse_noise_texture(props, materials)));
        }
        // {"type": "mix", "materials": ["A", "B"], "amount": 0.5 or an image texture (its grey
        // level) or a slope (`parse_color_texture`)}: B with probability amount, else A
        // (`material::mix`).
        if props["type"].as_str() == Some("mix") {
            let names = props["materials"].as_array().filter(|a| a.len() == 2)
                .unwrap_or_else(|| panic!("Mix material '{}' needs two \"materials\"", name));
            assert!(names.iter().all(|n| n.as_str() != Some(name.as_str())), "Mix material '{}' can't contain itself", name);
            let get = |k: &Value| SceneFile::require_material(k, materials, &format!("mix {}", name));
            return Some(Box::new(crate::material::mix::Mix {
                materials: [get(&names[0]), get(&names[1])],
                amount: match &props["amount"] {
                    Value::Object(_) => SceneFile::parse_color_texture(&props["amount"]),
                    v => { let a = SceneFile::parse_number(v, 0.5).clamp(0., 1.); Color::new(a, a, a).into() },
                },
            }));
        }
        // {"type": "surface_layers", "layers": [{"material": "A"}, {"material": "B", "mask":
        // graph, "coverage": 1}, ...]}: materials stacked from the bottom up, each over those
        // below by its mask (`material::surface_layers`).
        if props["type"].as_str() == Some("surface_layers") {
            use crate::material::surface_layers::{SurfaceLayer, SurfaceLayers};
            let layers = props["layers"].as_array().filter(|a| !a.is_empty())
                .unwrap_or_else(|| panic!("Surface layers '{}' need a list of \"layers\"", name));
            let layers = layers.iter().enumerate().map(|(k, l)| {
                let key = &l["material"];
                assert!(key.as_str() != Some(name.as_str()), "Surface layers '{}' can't contain themselves", name);
                SurfaceLayer {
                    material: SceneFile::require_material(key, materials, &format!("surface layer {} of {}", k, name)),
                    mask: l.get("mask").map(|m| crate::graph::Graph::parse(m)
                        .unwrap_or_else(|e| panic!("surface layer {} of {}'s mask: {}", k, name, e))),
                    coverage: SceneFile::parse_number(&l["coverage"], 1.),
                }
            }).collect();
            return Some(Box::new(SurfaceLayers { layers }));
        }
        // {"type": "snow_cover", "material": "A", "cover": 0.5, "snow": {"preset": "fresh"},
        // "up": [0, 1, 0], "drift": 0.3, "seed": 0}: snow lying on A where it faces the sky
        // (`material::snow::SnowCover`).
        if props["type"].as_str() == Some("snow_cover") {
            assert!(props["material"].as_str() != Some(name.as_str()), "Snow cover '{}' can't contain itself", name);
            let base = SceneFile::require_material(&props["material"], materials, &format!("snow_cover {}", name));
            let seed = SceneFile::parse_int(&props["seed"], 0) as u64;
            let snow = crate::material::snow::Snow::new(SceneFile::parse_snow_style(&props["snow"]),
                SceneFile::parse_number(&props["snow"]["scale"], 1.).max(1e-9), seed);
            return Some(Box::new(crate::material::snow::SnowCover::new(base, snow,
                SceneFile::parse_number(&props["cover"], 0.5).clamp(0., 1.),
                SceneFile::parse_vec3_def(props, "up", Vector3::y()),
                SceneFile::parse_number(&props["drift"], 0.3), seed)));
        }
        match SceneFile::parse_material(props) {
            Some(m) => Some(m),
            None => panic!("Material '{}' has unknown type {}", name, props["type"]),
        }
    }

    /// Look up a material that must exist; `context` names what references it.
    pub fn require_material(key: &Value, materials: &Map<String, Value>, context: &str) -> Box<dyn MaterialModel + Sync + Send> {
        SceneFile::parse_material_ref(key, materials)
            .unwrap_or_else(|| panic!("Material {} (referenced by {}) not found in materials", key, context))
    }

    pub fn parse_material(o: &Value) -> Option<Box<dyn MaterialModel + Sync + Send>> {
        // The noise material needs access to the entire materials dictionary to resolve
        // references to base materials, but we don't have access to it here.
        // We'll handle that special case in a different function.
        let t = o["type"].as_str().unwrap();
        if t == "metal" {
            let metal:Specular = Specular {
                albedo: SceneFile::parse_color(&o["reflective"]),
                roughness: SceneFile::parse_number(&o["roughness"], 0.),
            };
            return Some(Box::new(metal));
        }

        // {"type": "conductor", "metal": "Au" | "Ag" | "Al" | "Cu" | "CuZn"} (or "eta" and "k"
        // per channel, or a "reflectance" colour), "roughness": r, or "roughness_u" and
        // "roughness_v" along and across "tangent" (default the x axis): see
        // `material::conductor`.
        if t == "conductor" {
            use crate::material::conductor::{Conductor, Optics};
            let optics = if let Some(name) = o["metal"].as_str() {
                let (eta, k) = crate::spectrum::metals::named(name)
                    .unwrap_or_else(|| panic!("Unknown metal '{}': expected Ag, Al, Au, Cu or CuZn", name));
                Optics::Measured { eta, k }
            } else if !o["eta"].is_null() || !o["k"].is_null() {
                Optics::Constant { eta: SceneFile::parse_color_def(o, "eta", Color::white()), k: SceneFile::parse_color_def(o, "k", Color::black()) }
            } else {
                Optics::Reflectance(SceneFile::parse_color_def(o, "reflectance", Color::new(0.9, 0.9, 0.9)))
            };
            let r = SceneFile::parse_number(&o["roughness"], 0.);
            return Some(Box::new(Conductor {
                optics,
                roughness: (SceneFile::parse_number(&o["roughness_u"], r), SceneFile::parse_number(&o["roughness_v"], r)),
                tangent: SceneFile::parse_vec3_def(o, "tangent", Vector3::x()),
            }));
        }

        // {"type": "coated_diffuse", "albedo": colour or texture, "roughness": coat r,
        // "refractive_index": 1.5, "thickness": 0.01, "medium_albedo": colour, "g": 0,
        // "max_depth": 10, "samples": 1}, and "coated_conductor" with a conductor's "metal" (or
        // "eta" and "k", or "reflectance") and "conductor_roughness" in place of "albedo": pbrt's
        // layered materials (`material::layered`).
        if t == "coated_diffuse" || t == "coated_conductor" {
            use crate::material::conductor::Optics;
            use crate::material::layered::{Base, Coated};
            let base = if t == "coated_diffuse" {
                Base::Diffuse(SceneFile::parse_color_texture(o.get("albedo").unwrap_or(&serde_json::json!([0.5, 0.5, 0.5]))))
            } else {
                let optics = if let Some(name) = o["metal"].as_str() {
                    let (eta, k) = crate::spectrum::metals::named(name)
                        .unwrap_or_else(|| panic!("Unknown metal '{}': expected Ag, Al, Au, Cu or CuZn", name));
                    Optics::Measured { eta, k }
                } else if !o["eta"].is_null() || !o["k"].is_null() {
                    Optics::Constant { eta: SceneFile::parse_color_def(o, "eta", Color::white()), k: SceneFile::parse_color_def(o, "k", Color::black()) }
                } else {
                    Optics::Reflectance(SceneFile::parse_color_def(o, "reflectance", Color::new(0.9, 0.9, 0.9)))
                };
                Base::Conductor { optics, roughness: SceneFile::parse_number(&o["conductor_roughness"], 0.) }
            };
            let thickness = SceneFile::parse_number(&o["thickness"], 0.01);
            assert!(thickness > 0., "A coat's thickness must be positive");
            return Some(Box::new(Coated {
                base,
                refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.5),
                roughness: SceneFile::parse_number(&o["roughness"], 0.),
                thickness,
                albedo: SceneFile::parse_color_def(o, "medium_albedo", Color::black()),
                g: SceneFile::parse_number(&o["g"], 0.),
                max_depth: SceneFile::parse_int(&o["max_depth"], 10) as u32,
                samples: SceneFile::parse_int(&o["samples"], 1).max(1) as u32,
            }));
        }

        // {"type": "subsurface", "albedo": colour, "radius": colour or number (how far light
        // travels, scene units), or "sigma_a" and "sigma_s"; "g": 0, "refractive_index": 1.4}:
        // random-walk subsurface scattering (`material::subsurface`).
        if t == "subsurface" {
            use crate::material::subsurface::{Coefficients, Subsurface};
            let coefficients = if o.get("sigma_a").is_some() || o.get("sigma_s").is_some() {
                Coefficients::Sigma { sigma_a: SceneFile::parse_color_def(o, "sigma_a", Color::black()),
                                      sigma_s: SceneFile::parse_color_def(o, "sigma_s", Color::black()) }
            } else {
                let radius = match &o["radius"] {
                    Value::Number(n) => Color::white() * n.as_f64().unwrap(),
                    Value::Null => Color::new(1., 0.5, 0.25),
                    v => SceneFile::parse_color(v),
                };
                Coefficients::Albedo { albedo: SceneFile::parse_color_def(o, "albedo", Color::new(0.8, 0.8, 0.8)), radius }
            };
            let g = SceneFile::parse_number(&o["g"], 0.);
            assert!(g > -1. && g < 1., "g must be between -1 and 1, got {}", g);
            return Some(Box::new(Subsurface { coefficients, g, refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.4) }));
        }

        if t == "lambertian" {
            let d:Lambertian = Lambertian {
                albedo: SceneFile::parse_color_texture(&o["albedo"]),
            };
            return Some(Box::new(d));
        }

        if t == "plastic" {
            if !o["opacity"].is_null() {
                println!("Warning: plastic 'opacity' is ignored; plastic is opaque (use a dielectric for transparency)");
            }
            let d:Plastic = Plastic {
                albedo: SceneFile::parse_color_texture(&o["albedo"]),
                refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.5),
                roughness: SceneFile::parse_number(&o["roughness"], 0.),
            };
            return Some(Box::new(d));
        }

        if t == "leaf" {
            return Some(Box::new(SceneFile::parse_leaf(o)));
        }

        // {"type": "rough_dielectric", "refractive_index": .., "roughness": r, "attenuate":
        // colour}: frosted glass; {"type": "thin_dielectric", "refractive_index": ..}: a pane of
        // no thickness (`material::rough_dielectric`).
        // {"type": "hair", "eumelanin": 1.3, "pheomelanin": 0 (or "color" or "sigma_a"),
        // "eta": 1.55, "beta_m": 0.3, "beta_n": 0.3, "alpha": 2}: pbrt's hair BSDF
        // (`material::hair`), for cylinder curves.
        if t == "hair" {
            use crate::material::hair::{Hair, Pigment};
            let pigment = if !o["sigma_a"].is_null() {
                Pigment::SigmaA(SceneFile::parse_color(&o["sigma_a"]))
            } else if !o["color"].is_null() {
                Pigment::Color(SceneFile::parse_color(&o["color"]))
            } else {
                Pigment::Melanin { eumelanin: SceneFile::parse_number(&o["eumelanin"], 1.3), pheomelanin: SceneFile::parse_number(&o["pheomelanin"], 0.) }
            };
            return Some(Box::new(Hair {
                pigment,
                eta: SceneFile::parse_number(&o["eta"], 1.55),
                beta_m: SceneFile::parse_number(&o["beta_m"], 0.3).clamp(0.01, 1.),
                beta_n: SceneFile::parse_number(&o["beta_n"], 0.3).clamp(0.01, 1.),
                alpha: SceneFile::parse_number(&o["alpha"], 2.),
            }));
        }

        if t == "rough_dielectric" {
            return Some(Box::new(crate::material::rough_dielectric::RoughDielectric {
                refractive_index: SceneFile::parse_refractive_index(&o["refractive_index"]).or_default(1.5),
                roughness: SceneFile::parse_number(&o["roughness"], 0.3),
                attenuate: SceneFile::parse_color_def(o, "attenuate", Color::white()),
            }));
        }
        if t == "thin_dielectric" {
            return Some(Box::new(crate::material::rough_dielectric::ThinDielectric {
                refractive_index: SceneFile::parse_refractive_index(&o["refractive_index"]).or_default(1.5),
            }));
        }

        if t == "dielectric" {
            let d:Dielectric = Dielectric {
                refractive_index: SceneFile::parse_refractive_index(&o["refractive_index"]),
                attenuate: SceneFile::parse_color_def(o, "attenuate", Color::white()),
            };
            return Some(Box::new(d));
        }

        if t == "diffuse-light" {
            // `"texture": {"texture": "screen.png", ...}` scales the emission by an image.
            let d:DiffuseLight = DiffuseLight {
                intensity: SceneFile::parse_number(&o["intensity"], 1.),
                emission: SceneFile::parse_emission(o),
                two_sided: o["two_sided"].as_bool().unwrap_or(false),
                texture: o.get("texture").map(SceneFile::parse_image_map),
            };
            return Some(Box::new(d));
        }
        
        if t == "flat" {
            let d: FlatColor = FlatColor {
                pigment: SceneFile::parse_color(&o["color"]), 
            };
            return Some(Box::new(d));
        }

        if t == "whitted" {
            let d: Whitted = Whitted {
                pigment: SceneFile::parse_color(&o["pigment"]), 
                reflection: SceneFile::parse_number(&o["reflection"], 0.),
                phong: SceneFile::parse_number(&o["phong"], 0.),
            };
            return Some(Box::new(d));
        }

        if t == "normal" {
            return Some(Box::new(NormalShade {}));
        }

        // {"type": "uv"}: the texture coordinates as colour; {"type": "checker", "colors":
        // [colour, colour], "checks": n or [nu, nv]}: a diffuse checkerboard in texture space.
        if t == "uv" {
            return Some(Box::new(crate::material::uv::UvShade {}));
        }
        if t == "checker" {
            let colors = match o["colors"].as_array() {
                Some(c) if c.len() == 2 => [SceneFile::parse_color(&c[0]), SceneFile::parse_color(&c[1])],
                None => [Color::new(0.8, 0.8, 0.8), Color::new(0.1, 0.1, 0.1)],
                _ => panic!("A checker's \"colors\" are two colours, got {}", o["colors"]),
            };
            let checks = match &o["checks"] {
                Value::Array(_) => { let v = SceneFile::parse_vec2(&o["checks"]); crate::na::Vector2::new(v.x, v.y) },
                v => crate::na::Vector2::repeat(SceneFile::parse_number(v, 8.)),
            };
            return Some(Box::new(crate::material::uv::UvChecker { colors, checks }));
        }

        if t == "rock" {
            return Some(Box::new(SceneFile::parse_rock(o)));
        }

        if t == "soil" {
            return Some(Box::new(SceneFile::parse_soil(o)));
        }

        // {"type": "snow", "preset": "settled", "grain_size": mm, "soot": ppb, "dust": ppm,
        // "sparkle": 0 to 1, "scale": scene units per metre, "seed": 0} and {"type": "ice",
        // "preset": "glacier", "bubble_radius": mm, "bubbles": per cm³, "soot", "dust",
        // "roughness"}: snow and glacier ice from ice's absorption and their grains or bubbles
        // (`material::snow`).
        if t == "snow" {
            return Some(Box::new(crate::material::snow::Snow::new(SceneFile::parse_snow_style(o),
                SceneFile::parse_number(&o["scale"], 1.).max(1e-9), SceneFile::parse_int(&o["seed"], 0) as u64)));
        }
        if t == "ice" {
            return Some(Box::new(crate::material::snow::Ice::new(SceneFile::parse_ice_style(o))));
        }
        
        // Noise materials are handled separately in parse_object_medium
        
        /*
        return material::MaterialProperties {
            pigment: SceneFile::parse_color(&o["pigment"]), 
            albedo: SceneFile::parse_number(&o["albedo"], 0.2),
            metallic: o["metallic"].as_f64().unwrap(), 
            roughness: o["roughness"].as_f64().unwrap(), 
            reflection: SceneFile::parse_number(&o["reflection"], 0.),
            opacity: SceneFile::parse_number(&o["opacity"], 1.),
            refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.3),
            phong: o["phong"].as_f64().unwrap(),
            normal_peturbation: Vector3::new(0., 0., 0.)
        }
            */
        None
    }

    /// Snow's makeup (`material::snow::SnowStyle`): a `"preset"` (fresh, settled, old, firn,
    /// dirty; by default settled) with any of `grain_size` (mm), `soot` (ppb), `dust` (ppm) and
    /// `sparkle` overridden.
    pub fn parse_snow_style(o: &Value) -> crate::material::snow::SnowStyle {
        let p = crate::material::snow::SnowStyle::preset(o["preset"].as_str().unwrap_or("settled"));
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        crate::material::snow::SnowStyle {
            grain_size: n("grain_size", p.grain_size).max(1e-3),
            impurities: crate::material::snow::Impurities { soot: n("soot", p.impurities.soot).max(0.), dust: n("dust", p.impurities.dust).max(0.) },
            sparkle: n("sparkle", p.sparkle).clamp(0., 1.),
        }
    }

    /// Glacier ice's makeup (`material::snow::IceStyle`): a `"preset"` (white, glacier, blue,
    /// clear; by default glacier) with any of `bubble_radius` (mm), `bubbles` (per cm³), `soot`
    /// (ppb), `dust` (ppm) and `roughness` overridden.
    pub fn parse_ice_style(o: &Value) -> crate::material::snow::IceStyle {
        let p = crate::material::snow::IceStyle::preset(o["preset"].as_str().unwrap_or("glacier"));
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        crate::material::snow::IceStyle {
            bubble_radius: n("bubble_radius", p.bubble_radius).max(1e-4),
            bubbles: n("bubbles", p.bubbles).max(0.),
            impurities: crate::material::snow::Impurities { soot: n("soot", p.impurities.soot).max(0.), dust: n("dust", p.impurities.dust).max(0.) },
            roughness: n("roughness", p.roughness).clamp(0., 1.),
        }
    }

    /// `{"type": "soil", "preset": "loam"}`: procedural bare ground (see `material::soil`).
    /// Presets are loam, clay, sand, desert and forest_floor; any field of the preset can be
    /// overridden: the colours `color`, `humus`, `stone_color`, `crumb_color`, `litter_color`
    /// and `moss_color`; the fractions of the ground covered by `stones`, `crumbs`, `litter`
    /// and `moss`; the sizes (m) `mottle_size`, `stone_size`, `litter_length`, `litter_width`
    /// and `clod_size`; and `bump`, the strength of the relief. Also `"scale"` (for a scene not
    /// in metres), `"seed"` and `"wetness"` (0 to 1).
    pub fn parse_soil(o: &Value) -> Soil {
        let p = SoilStyle::preset(o["preset"].as_str().unwrap_or("loam"));
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let c = |k: &str, d: Color| SceneFile::parse_color_def(o, k, d);
        let style = SoilStyle {
            color: c("color", p.color),
            humus: c("humus", p.humus),
            mottle_size: n("mottle_size", p.mottle_size).max(1e-4),
            stone_color: c("stone_color", p.stone_color),
            stones: n("stones", p.stones).clamp(0., 1.),
            stone_size: n("stone_size", p.stone_size).max(1e-4),
            crumb_color: c("crumb_color", p.crumb_color),
            crumbs: n("crumbs", p.crumbs).clamp(0., 1.),
            litter_color: c("litter_color", p.litter_color),
            litter: n("litter", p.litter).clamp(0., 1.),
            litter_length: n("litter_length", p.litter_length).max(1e-4),
            litter_width: n("litter_width", p.litter_width).max(1e-5),
            moss_color: c("moss_color", p.moss_color),
            moss: n("moss", p.moss).clamp(0., 1.),
            bump: n("bump", p.bump).max(0.),
            clod_size: n("clod_size", p.clod_size).max(1e-4),
        };
        let mut soil = Soil::new(style, n("scale", 1.).max(1e-9), SceneFile::parse_int(&o["seed"], 0) as u64);
        soil.wetness = n("wetness", 0.).clamp(0., 1.);
        soil
    }

    /// `{"type": "rock", "preset": "granite"}`: a procedural stone (see `material::rock`).
    /// Presets are granite, basalt, sandstone, limestone, slate and quartzite; any field of the
    /// preset can be overridden, eg. `"color"` or `"strata_amount"`. Also `"scale"` (the size of
    /// the texture's features), `"seed"`, `"weathering"`, `"wetness"` (0 to 1), `"lichen"` (the
    /// fraction of upward surfaces covered, 0 to 1) and `"lichen_color"`. For layered rock
    /// faces: `"layers"` ([[y, colour], ...]: formations' colours by elevation, warped by
    /// `"layer_warp"`), `"beds"` (a colour per bed of the strata), `"variety"` and
    /// `"variety_color"` (broad stains), `"varnish"` and `"varnish_color"` (dark streaks down
    /// steep faces), `"flutes"` (vertical grooves, bump mapped), `"cross_bedding"` and
    /// `"cross_set"` (inclined laminae in sets that thick) and `"pits"` and `"pit_size"`
    /// (honeycomb weathering), `"fractal"`, `"fractal_size"` and `"fractal_h"` (turbulent
    /// relief down to a pixel's size, for close-up detail) and `"crackle"` and
    /// `"crackle_size"` (fractures). Sizes are in units of `"scale"`.
    pub fn parse_rock(o: &Value) -> Rock {
        let preset = RockStyle::preset(o["preset"].as_str().unwrap_or("granite"));
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let c = |k: &str, d: Color| SceneFile::parse_color_def(o, k, d);
        let style = RockStyle {
            color: c("color", preset.color),
            secondary: c("secondary", preset.secondary),
            dark_grain: c("dark_grain", preset.dark_grain),
            dark_grain_amount: n("dark_grain_amount", preset.dark_grain_amount),
            light_grain: c("light_grain", preset.light_grain),
            light_grain_amount: n("light_grain_amount", preset.light_grain_amount),
            grain_size: n("grain_size", preset.grain_size),
            strata_spacing: n("strata_spacing", preset.strata_spacing),
            strata_color: c("strata_color", preset.strata_color),
            strata_amount: n("strata_amount", preset.strata_amount),
            bump: n("bump", preset.bump),
            bump_size: n("bump_size", preset.bump_size),
            cracks: n("cracks", preset.cracks),
        };
        let mut r = Rock::new(style, n("scale", 1.), SceneFile::parse_int(&o["seed"], 0) as u64);
        r.weathering = n("weathering", r.weathering);
        r.wetness = n("wetness", 0.).clamp(0., 1.);
        if let Some(layers) = o["layers"].as_array() {
            r.layers = layers.iter().map(|l| (SceneFile::parse_number(&l[0], 0.), SceneFile::parse_color(&l[1]))).collect();
            r.layers.sort_by(|a, b| a.0.total_cmp(&b.0));
        }
        r.layer_warp = n("layer_warp", 0.);
        if let Some(beds) = o["beds"].as_array() {
            r.beds = beds.iter().map(SceneFile::parse_color).collect();
        }
        r.variety = n("variety", 0.).clamp(0., 1.);
        r.variety_color = c("variety_color", r.variety_color);
        r.varnish = n("varnish", 0.).clamp(0., 1.);
        r.varnish_color = c("varnish_color", r.varnish_color);
        r.flutes = n("flutes", 0.).max(0.);
        r.cross_bedding = n("cross_bedding", 0.).clamp(0., 1.);
        r.cross_set = n("cross_set", r.cross_set).max(1e-6);
        r.pits = n("pits", 0.).clamp(0., 1.);
        r.pit_size = n("pit_size", r.pit_size).max(1e-6);
        r.fractal = n("fractal", 0.).max(0.);
        r.fractal_size = n("fractal_size", r.fractal_size).max(1e-6);
        r.fractal_h = n("fractal_h", r.fractal_h);
        r.crackle = n("crackle", 0.).max(0.);
        r.crackle_size = n("crackle_size", r.crackle_size).max(1e-6);
        let lichen = n("lichen", 0.);
        if lichen > 0. {
            let color = c("lichen_color", r.lichen_color);
            r = r.with_lichen(lichen, color);
        }
        r
    }

    /// Icebergs and sea ice floating on the sea at `"sea_level"` (default 0), see
    /// `procedural::iceberg`, riding the waves of the scene's ocean if it has one (`"ocean"`,
    /// filled in by `parse_objects`). Each takes a `"seed"`, and a `"material"` or `"medium"` in place of
    /// its own ice and snow.
    ///
    /// - `{"type": "iceberg", "location": [x, z], "size": 100, "shape": "tabular" | "blocky" |
    ///   "dome" | "pinnacle" | "wedge" | "drydock", "yaw": degrees, "density": 900, "detail": 7}`:
    ///   an iceberg `size` metres long, floating as its density says.
    /// - `{"type": "ice_floes", "center": [x, z], "extent": [400, 300], "concentration": 0.6,
    ///   "size": [2, 60], "thickness": 1}`: floes from `size[0]` to `size[1]` metres across
    ///   covering `concentration` of the sea; `"style": "brash"` for lumps of glacier ice
    ///   (brash ice and bergy bits) rather than slabs of sea ice; `"clumping"` (0 to 1, default
    ///   0.5) gathers them in clumps.
    pub fn parse_ice_object(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>) -> SceneObject {
        use crate::procedural::iceberg::{FloeField, FloeMaterial, FloeStyle, Iceberg, IcebergMaterial, Kind};
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let sea_level = n("sea_level", 0.);
        // The ocean it floats on, if the scene has one (`parse_objects` passes it in).
        let waves = o["ocean"].is_object().then(|| crate::ocean::surface(&o["ocean"]));
        let given = o.get("material").is_some() || o.get("medium").is_some();
        let medium = |own: Box<dyn MaterialModel + Sync + Send>| -> Box<dyn Medium + Sync + Send> {
            if given { SceneFile::parse_object_medium(o, materials, media) } else { Box::new(Solid { m: own }) }
        };
        if o["type"].as_str() == Some("iceberg") {
            let mut berg = Iceberg::new(Kind::named(o["shape"].as_str().unwrap_or("blocky")), n("size", 100.).max(0.1), seed);
            berg.density = n("density", berg.density).clamp(100., 1020.);
            berg.detail = SceneFile::parse_int(&o["detail"], berg.detail as usize).min(8) as u32;
            let at = SceneFile::parse_vec2_def(o, "location", crate::na::Vector2::zeros());
            let yaw = match o["yaw"].as_f64() {
                Some(y) => y.to_radians(),
                None => crate::geometry::Pcg32::new(seed, 43).next_f64() * std::f64::consts::TAU,
            };
            let rise = waves.as_ref().map_or(0., |w| w.surface_at(at.x, at.y).0);
            let mesh = berg.mesh(at.x, at.y, sea_level + rise, yaw);
            let own = IcebergMaterial::new(sea_level, berg.size, seed);
            return rock::object(&[mesh], medium(Box::new(own)));
        }
        let center = SceneFile::parse_vec2_def(o, "center", crate::na::Vector2::zeros());
        let extent = SceneFile::parse_vec2_def(o, "extent", crate::na::Vector2::new(400., 300.));
        let size = SceneFile::parse_vec2_def(o, "size", crate::na::Vector2::new(2., 60.));
        let field = FloeField {
            center: (center.x, center.y), extent: (extent.x.max(1.), extent.y.max(1.)),
            concentration: n("concentration", 0.6).clamp(0., 0.95),
            min_size: size.x.max(0.1), max_size: size.y.max(size.x.max(0.1)),
            thickness: n("thickness", 1.).max(0.01), sea_level, seed,
            waves,
            style: match o["style"].as_str().unwrap_or("floes") {
                "floes" => FloeStyle::Slab,
                "brash" => FloeStyle::Brash,
                x => panic!("Unknown ice_floes style '{}': expected floes or brash", x),
            },
            clumping: n("clumping", 0.5).clamp(0., 1.),
        };
        let own: Box<dyn MaterialModel + Sync + Send> = match field.style {
            FloeStyle::Slab => Box::new(FloeMaterial::new(seed)),
            FloeStyle::Brash => Box::new(IcebergMaterial::new(sea_level, 2., seed).brash()),
        };
        rock::object(&[field.mesh()], medium(own))
    }

    /// Procedural rocks, all resting on the ground (see `procedural::rock`). Each takes a
    /// `"seed"`, and a `"material"` or `"medium"`, by default a rock material.
    ///
    /// - `{"type": "boulder", "location": [x, y, z], "radius": 1, "angularity": 0.5}`: a boulder
    ///   sitting at `location`. Angularity runs from 0, rounded, to 1, sharply broken.
    /// - `{"type": "pebbles", "center": [x, y, z], "radius": 1, "count": 100, "size": [0.03,
    ///   0.12]}`: pebbles scattered over a disc, or over a rectangle `"extent": [x, z]`. The
    ///   ground may slope, rising `"slope": [per x, per z]`; `"packing"` below 1 (eg. 0.75)
    ///   packs them closer, leaning on each other. `"materials": ["A", "B"]` mixes materials,
    ///   one per pebble; by default a mix of stones, made wet by `"wetness"`.
    /// - `{"type": "bedrock", "center": [x, y, z], "size": [20, 20], "height": 3}`: an outcrop
    ///   rising from `center`. Also `"feature_size"`, `"strata"` (the thickness of its terraced
    ///   beds, 0 for none), `"strata_strength"` (0 to 1), `"falloff"` (the fraction of each side
    ///   over which it slopes to the base) and `"resolution"` (vertices along a side).
    ///
    /// Boulders and pebbles take `"detail"`, the number of geodesic subdivisions (5 and 3).
    pub fn parse_rock_object(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>) -> SceneObject {
        let t = o["type"].as_str().unwrap();
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let detail = o.get("detail").map(|d| SceneFile::parse_int(d, 0) as u32);
        let given = o.get("material").is_some() || o.get("medium").is_some();
        let medium = |default: &str| -> Box<dyn Medium + Sync + Send> {
            if given {
                SceneFile::parse_object_medium(o, materials, media)
            } else {
                Box::new(Solid { m: Box::new(Rock::new(RockStyle::preset(default), 1., seed)) })
            }
        };
        match t {
            "boulder" => {
                let mesh = rock::boulder_mesh(
                    SceneFile::parse_vec3_def(o, "location", Vector3::zeros()),
                    n("radius", 1.), n("angularity", 0.5), detail, seed);
                rock::object(&[mesh], medium("granite"))
            },
            "bedrock" => {
                let size = SceneFile::parse_vec2_def(o, "size", Vector2::new(20., 20.));
                let b = Bedrock {
                    center: SceneFile::parse_vec3_def(o, "center", Vector3::zeros()),
                    size: (size.x, size.y),
                    height: n("height", 3.),
                    feature_size: n("feature_size", size.x.min(size.y) / 4.),
                    strata: n("strata", 0.),
                    strata_strength: n("strata_strength", 0.8),
                    falloff: n("falloff", 0.25),
                    resolution: SceneFile::parse_int(&o["resolution"], 160),
                    seed,
                };
                rock::object(&[b.mesh()], medium(if b.strata > 0. { "sandstone" } else { "granite" }))
            },
            "pebbles" => {
                let size = SceneFile::parse_vec2_def(o, "size", Vector2::new(0.03, 0.12));
                let names: Vec<Value> = o["materials"].as_array().cloned().unwrap_or_default();
                let mixed: Vec<Box<dyn MaterialModel + Sync + Send>> = if !names.is_empty() {
                    names.iter().map(|k| SceneFile::require_material(k, materials, "pebbles")).collect()
                } else {
                    let wetness = n("wetness", 0.);
                    ["granite", "basalt", "quartzite", "sandstone", "slate", "limestone"].iter().enumerate()
                        .map(|(i, p)| {
                            let mut r = Rock::new(RockStyle::preset(p), 0.25, seed + i as u64);
                            r.wetness = wetness;
                            Box::new(r) as Box<dyn MaterialModel + Sync + Send>
                        }).collect()
                };
                let center = SceneFile::parse_vec3_def(o, "center", Vector3::zeros());
                let region = match o.get("extent") {
                    Some(e) => { let e = SceneFile::parse_vec2(e); rock::Region::Rect(e.x, e.y) },
                    None => rock::Region::Disc(n("radius", 1.)),
                };
                let slope = SceneFile::parse_vec2_def(o, "slope", Vector2::zeros());
                let field = rock::PebbleField {
                    center, region,
                    count: SceneFile::parse_int(&o["count"], 100),
                    min_size: size.x.min(size.y), max_size: size.x.max(size.y),
                    packing: n("packing", 1.),
                    slope: (slope.x, slope.y),
                    materials: mixed.len(),
                    seed,
                };
                let pebbles = field.scatter();
                let meshes = field.meshes(&pebbles, detail);
                if given && names.is_empty() {
                    rock::object(&meshes, SceneFile::parse_object_medium(o, materials, media))
                } else {
                    rock::object(&meshes, Box::new(Assorted::new(mixed, pebbles)))
                }
            },
            _ => unreachable!(),
        }
    }

    /// A noise texture over a base material. Needs the materials map to resolve the base.
    pub fn parse_noise_texture(o: &Value, materials: &Map<String, Value>) -> NoiseTexture {
        // Get base material by reference
        let base_material_key = &o["base_material"];
        let base_material = if let Some(material_name) = base_material_key.as_str() {
            // We need to use the root-level materials map from the scene file
            SceneFile::parse_material_ref(base_material_key, materials)
                .unwrap_or_else(|| {
                    println!("Warning: Using default material for noise texture because base material '{}' not found", material_name);
                    Box::new(Lambertian { albedo: Color::white().into() })
                })
        } else {
            // Default to white lambertian if no base material is specified
            Box::new(Lambertian { albedo: Color::white().into() })
        };
        
        let noise_type = match o.get("noise_type").and_then(|v| v.as_str()) {
            Some("perlin") => NoiseType::Perlin,
            Some("fbm") => NoiseType::Fbm {
                octaves: SceneFile::parse_int(&o["octaves"], 4) as u32,
                persistence: SceneFile::parse_number(&o["persistence"], 0.5),
                lacunarity: SceneFile::parse_number(&o["lacunarity"], 2.0),
            },
            Some("worley") => NoiseType::Worley {
                point_density: SceneFile::parse_number(&o["point_density"], 1.0),
                seed: SceneFile::parse_int(&o["seed"], 42) as u32,
            },
            Some("marble") => NoiseType::Marble,
            Some("turbulence") => NoiseType::Turbulence {
                octaves: SceneFile::parse_int(&o["octaves"], 4) as u32,
            },
            _ => NoiseType::Perlin, // Default to Perlin
        };
        
        let texture = match noise_type {
            NoiseType::Perlin => {
                NoiseTexture::new_perlin(
                    base_material,
                    SceneFile::parse_color_def(o, "color", Color::white()),
                    SceneFile::parse_number(&o["scale"], 0.1),
                    SceneFile::parse_number(&o["blend_factor"], 0.5),
                )
            },
            NoiseType::Fbm { octaves, persistence, lacunarity } => {
                NoiseTexture::new_fbm(
                    base_material,
                    SceneFile::parse_color_def(o, "color", Color::white()),
                    SceneFile::parse_number(&o["scale"], 0.1),
                    SceneFile::parse_number(&o["blend_factor"], 0.5),
                    octaves,
                    persistence,
                    lacunarity,
                )
            },
            NoiseType::Worley { point_density, seed } => {
                NoiseTexture::new_worley(
                    base_material,
                    SceneFile::parse_color_def(o, "color", Color::white()),
                    SceneFile::parse_number(&o["scale"], 0.1),
                    SceneFile::parse_number(&o["blend_factor"], 0.5),
                    point_density,
                    seed,
                )
            },
            NoiseType::Marble => {
                NoiseTexture::new_marble(
                    base_material,
                    SceneFile::parse_color_def(o, "color", Color::white()),
                    SceneFile::parse_number(&o["scale"], 0.1),
                    SceneFile::parse_number(&o["blend_factor"], 0.5),
                )
            },
            NoiseType::Turbulence { octaves } => {
                NoiseTexture::new_turbulence(
                    base_material,
                    SceneFile::parse_color_def(o, "color", Color::white()),
                    SceneFile::parse_number(&o["scale"], 0.1),
                    SceneFile::parse_number(&o["blend_factor"], 0.5),
                    octaves,
                )
            },
        };
        match SceneFile::parse_seed(o) {
            Some(seed) => texture.seeded(seed),
            None => texture,
        }
    }

    /// An object's optional `"seed"`, choosing its procedural pattern. Without one, noise uses
    /// Perlin's reference table (and Worley seed 42), as before seeds existed.
    pub fn parse_seed(o: &Value) -> Option<u64> {
        o.get("seed").map(|s| SceneFile::parse_int(s, 0) as u64)
    }

    pub fn parse_medium_ref(key: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Option<Box<dyn Medium + Sync + Send>> {
        let medium_name = SceneFile::parse_string(key);
        match media.get(&medium_name) {
            Some(props) => SceneFile::parse_medium(props, materials),
            None => {
                eprintln!("ERROR: Medium '{}' not found in media map. This is a fatal error.", medium_name);
                panic!("Medium '{}' not found in media map", medium_name);
            }
        }
    }

    pub fn parse_medium(o: &Value, materials: &Map<String, Value>) -> Option<Box<dyn Medium + Sync + Send>> {
        let t = o["type"].as_str().unwrap();
        if t == "solid" {
            let m = SceneFile::require_material(&o["material"], materials, "solid medium");
            return Some(Box::new(Solid { m }));
        }

        if t == "checkered-y-plane" {
            let m1 = SceneFile::require_material(&o["m1"], materials, "checkered-y-plane m1");
            let m2 = SceneFile::require_material(&o["m2"], materials, "checkered-y-plane m2");
            let xsize = SceneFile::parse_number(&o["xsize"], 1.);
            let zsize = SceneFile::parse_number(&o["zsize"], 1.);
            return Some(Box::new(CheckeredYPlane {
                m1, m2, xsize, zsize
            }));
        }
        
        if t == "noise" {
            return Some(Box::new(Solid { m: Box::new(SceneFile::parse_noise_texture(o, materials)) }));
        }
        
        if t == "noise_medium" {
            // Get the two materials to mix between
            let m1_name = SceneFile::parse_string(&o["m1"]);
            let m1 = match SceneFile::parse_material_ref(&o["m1"], materials) {
                Some(mat) => mat,
                None => {
                    eprintln!("ERROR: Material '{}' not found for NoiseMedium m1. This is a fatal error.", m1_name);
                    panic!("Material '{}' for NoiseMedium m1 not found", m1_name);
                }
            };
            
            let m2_name = SceneFile::parse_string(&o["m2"]);
            let m2 = match SceneFile::parse_material_ref(&o["m2"], materials) {
                Some(mat) => mat,
                None => {
                    eprintln!("ERROR: Material '{}' not found for NoiseMedium m2. This is a fatal error.", m2_name);
                    panic!("Material '{}' for NoiseMedium m2 not found", m2_name);
                }
            };
            
            // Parse the threshold value (default to 0.5)
            let threshold = SceneFile::parse_number(&o["threshold"], 0.5);
            
            // Parse scale (default to 0.1)
            let scale = SceneFile::parse_number(&o["scale"], 0.1);
            
            // Parse noise_type
            let noise_type = match o.get("noise_type").and_then(|v| v.as_str()) {
                Some("perlin") => texture::NoiseType::Perlin,
                Some("fbm") => texture::NoiseType::Fbm {
                    octaves: SceneFile::parse_int(&o["octaves"], 4) as u32,
                    persistence: SceneFile::parse_number(&o["persistence"], 0.5),
                    lacunarity: SceneFile::parse_number(&o["lacunarity"], 2.0),
                },
                Some("worley") => texture::NoiseType::Worley {
                    point_density: SceneFile::parse_number(&o["point_density"], 1.0),
                    seed: SceneFile::parse_int(&o["seed"], 42) as u32,
                },
                Some("marble") => texture::NoiseType::Marble,
                Some("turbulence") => texture::NoiseType::Turbulence {
                    octaves: SceneFile::parse_int(&o["octaves"], 4) as u32,
                },
                Some("combined") => texture::NoiseType::Combined {
                    falloff: SceneFile::parse_number(&o["falloff"], 0.1),
                },
                _ => texture::NoiseType::Perlin, // Default to Perlin
            };
            
            // Create the appropriate NoiseMedium based on the noise type
            let noise_medium = match noise_type {
                texture::NoiseType::Perlin => {
                    NoiseMedium::new_perlin(
                        m1,
                        m2,
                        scale,
                        threshold,
                    )
                },
                texture::NoiseType::Fbm { octaves, persistence, lacunarity } => {
                    NoiseMedium::new_fbm(
                        m1,
                        m2,
                        scale,
                        threshold,
                        octaves,
                        persistence,
                        lacunarity,
                    )
                },
                texture::NoiseType::Worley { point_density, seed } => {
                    NoiseMedium::new_worley(
                        m1,
                        m2,
                        scale,
                        threshold,
                        point_density,
                        seed,
                    )
                },
                texture::NoiseType::Marble => {
                    NoiseMedium::new_marble(
                        m1,
                        m2,
                        scale,
                        threshold,
                    )
                },
                texture::NoiseType::Turbulence { octaves } => {
                    NoiseMedium::new_turbulence(
                        m1,
                        m2,
                        scale,
                        threshold,
                        octaves,
                    )
                },
                texture::NoiseType::Combined { falloff } => {
                    NoiseMedium::new_combined(
                        m1,
                        m2,
                        scale,
                        threshold,
                        falloff,
                    )
                },
            };
            let noise_medium = match SceneFile::parse_seed(o) {
                Some(seed) => noise_medium.seeded(seed),
                None => noise_medium,
            };
            
            return Some(Box::new(noise_medium));
        }

        None
    }
    pub fn parse_air(_o: &Option<Value>) -> Box<dyn ParticipatingMedium>{
        let air: Box<dyn ParticipatingMedium> = Box::new(Vacuum {});
        /*
        if o.is_some() {
            air = Box::new(HomogenousFog { density: 0.001, color: Color::red() });
        }
        */
        air
    }


    /// A light at a point (`light::Light`): `{"location": [x, y, z], "intensity": I, "color" or
    /// "temperature"}`, radiant intensity `I` (per steradian), or `"power": Φ` (watts, of a point
    /// or spot light) in its place. One of these makes it directional:
    ///
    /// - `"spot": {"direction": [x, y, z] or "lookat": [x, y, z], "angle": 30, "delta": 5}`: a
    ///   cone of half-angle `angle` degrees, fading over its outer `delta` (pbrt's `coneangle`
    ///   and `conedelta`).
    /// - `"goniometric": {"image": file, "up": [0, 1, 0], "forward": [0, 0, 1]}`: intensity
    ///   scaled by an equirectangular image of directions about `up` (top row up), azimuth zero
    ///   towards `forward`.
    /// - `"projection": {"image": file, "lookat": [x, y, z] or "direction", "up": [0, 1, 0],
    ///   "fov": 30}`: the image projected as by a slide projector, `fov` degrees across its
    ///   larger side.
    pub fn parse_light(o: &Value) -> Light {
        let position = SceneFile::parse_vec3(&o["location"]);
        let aim = |v: &Value| -> Vector3<f64> {
            match (v.get("direction"), v.get("lookat")) {
                (Some(d), None) => SceneFile::parse_vec3(d).normalize(),
                (None, Some(l)) => (SceneFile::parse_vec3(l) - position).normalize(),
                _ => panic!("A light's \"spot\" or \"projection\" needs a \"direction\" or a \"lookat\""),
            }
        };
        // Rows x, y, z of the rotation into the light's frame, z along `forward`, y nearest `up`,
        // x to the right as the camera's (`camera`), so a projection reads as seen from behind.
        let frame = |forward: Vector3<f64>, up: Vector3<f64>| -> Matrix3<f64> {
            let z = forward.normalize();
            let x = z.cross(&up);
            let x = if x.norm() < 1e-9 { z.cross(&z.cross(&Vector3::new(1., 0., 0.))) } else { x }.normalize();
            let y = x.cross(&z);
            Matrix3::from_rows(&[x.transpose(), y.transpose(), z.transpose()])
        };
        let image = |v: &Value| Image::shared(v["image"].as_str()
            .unwrap_or_else(|| panic!("A goniometric or projection light needs an \"image\"")), None);
        let profile = if let Some(v) = o.get("spot") {
            let angle = SceneFile::parse_number(&v["angle"], 30.);
            let delta = SceneFile::parse_number(&v["delta"], 5.).clamp(0., angle);
            assert!(angle > 0. && angle <= 180., "A spot light's angle must be 0 to 180 degrees, got {}", angle);
            LightProfile::Spot { axis: aim(v), cos_falloff_start: (angle - delta).to_radians().cos(), cos_total: angle.to_radians().cos() }
        } else if let Some(v) = o.get("goniometric") {
            let up = SceneFile::parse_vec3_def(v, "up", Vector3::new(0., 1., 0.));
            let forward = SceneFile::parse_vec3_def(v, "forward", Vector3::new(0., 0., 1.));
            // The image's pole is the frame's y: build it with up as y, forward as z.
            let f = frame(forward - up.normalize() * forward.dot(&up.normalize()), up);
            LightProfile::Goniometric { frame: f, image: image(v) }
        } else if let Some(v) = o.get("projection") {
            let img = image(v);
            let (w, h) = img.size();
            let half = (SceneFile::parse_number(&v["fov"], 30.).to_radians() / 2.).tan();
            let half = if w >= h { Vector2::new(half, half * h as f64 / w as f64) } else { Vector2::new(half * w as f64 / h as f64, half) };
            let f = frame(aim(v), SceneFile::parse_vec3_def(v, "up", Vector3::new(0., 1., 0.)));
            LightProfile::Projection { frame: f, image: img, half }
        } else {
            LightProfile::Isotropic
        };
        let mut light = Light { position, emission: SceneFile::parse_emission(o), intensity: 1., profile };
        light.intensity = match (o.get("intensity"), o.get("power")) {
            (Some(i), None) => SceneFile::parse_number(i, 1.),
            (None, Some(p)) => {
                // Power over the solid angle it's spread across, per unit of `power()`'s
                // luminance.
                let unit = light.power() / crate::color::luminance(&light.emission.srgb()).max(1e-12);
                SceneFile::parse_number(p, 1.) / unit
            },
            _ => panic!("A light needs an \"intensity\" or a \"power\""),
        };
        light
    }

    /// What an emitter emits: `"color"` (white if absent), or with `"temperature"` (kelvin), a
    /// blackbody's spectrum scaled to unit luminance (`spectrum::Emission`), so that emitters
    /// of the same intensity are equally bright whatever their temperature.
    pub fn parse_emission(o: &Value) -> Emission {
        match o["temperature"].as_f64() {
            Some(t) => {
                assert!(o.get("color").is_none(), "An emitter takes \"color\" or \"temperature\", not both");
                Emission::blackbody(t)
            },
            None => Emission::Rgb(SceneFile::parse_color_def(o, "color", Color::white())),
        }
    }

    /// Lightning is bright on the scale of the scene's sun (`procedural::lightning`): give it
    /// the sun's intensity, unless it has its own `sun_intensity`.
    pub fn lightning_follows_sun(sun: &Value, objects: &mut [Value]) {
        if sun.is_null() {
            return;
        }
        let intensity = SceneFile::parse_number(&sun["intensity"], 1.);
        for o in objects.iter_mut().filter(|o| o["type"].as_str() == Some("lightning") && o.get("sun_intensity").is_none()) {
            o["sun_intensity"] = serde_json::json!(intensity);
        }
    }

    /// With a `"sun"` and a skysphere, the sky is lit by that sun: its brightness is set so its
    /// radiance is the air's per unit of sunlight times the sun's `intensity`
    /// (`skysphere::sky_brightness_for_sun`), on the same scale as the sunlight on everything
    /// under it. So the sky can't be given a `brightness` of its own.
    /// A skysphere without a `"cache"` of its own keeps its tables (`skysphere::table_file`)
    /// in its scene's landscape's, if that has one: a scene that is slow to build for its
    /// landscape is as slow again for its sky, in a browser.
    pub fn sky_cache(objects: &mut [Value]) {
        let Some(dir) = objects.iter().filter(|o| o["type"].as_str() == Some("landscape")).find_map(|o| o["cache"].as_str()).map(str::to_string) else { return };
        for o in objects.iter_mut().filter(|o| o["type"].as_str() == Some("skysphere") && o.get("cache").is_none()) {
            o["cache"] = serde_json::json!(dir);
        }
    }

    pub fn sky_follows_sun(sun: &Value, objects: &mut [Value]) {
        if sun.is_null() {
            return;
        }
        let intensity = SceneFile::parse_number(&sun["intensity"], 1.);
        for o in objects.iter_mut().filter(|o| o["type"].as_str() == Some("skysphere")) {
            assert!(o.get("brightness").is_none(),
                "With a sun, the sky's brightness follows the sun's intensity; remove the skysphere's \"brightness\"");
            o["brightness"] = serde_json::json!(crate::skysphere::sky_brightness_for_sun(intensity));
        }
    }

    pub fn parse_lights(lights: &Vec<Value>) -> Vec<Light> {
        let mut l: Vec<Light> = Vec::new();
        for light in lights {
            l.push(SceneFile::parse_light(light));
        }
        l
    }

    /// The scene's directional light: `{"direction": [x, y, z], "color": colour, "intensity": E,
    /// "angular_radius": degrees}`, with irradiance `color` × `intensity` (by default white
    /// and one) on a surface facing it, `direction` pointing towards it, and a visible disc of
    /// the given radius (default the sun's, 0.2666°). With a skysphere in `objects`, the sun is the
    /// sky's: it takes the sky's `sun_direction` (so `direction` is not allowed), `intensity`
    /// is its irradiance above the atmosphere, which also sets the sky's brightness
    /// (`sky_follows_sun`), and it is reddened by the sky's atmosphere on
    /// its way down to each point, according to the point's altitude (scene y, in metres), so
    /// it is whiter at cloud level than on the ground, and sets orange as the sky does.
    pub fn parse_sun(v: &Value, objects: &[Value]) -> Option<Sun> {
        if v.is_null() {
            return None;
        }
        let irradiance = SceneFile::parse_color_def(v, "color", Color::white())
            * SceneFile::parse_number(&v["intensity"], 1.);
        match objects.iter().find(|o| o["type"].as_str() == Some("skysphere")) {
            Some(sky) => {
                assert!(v.get("direction").is_none(),
                    "The sun follows the skysphere's sun_direction; remove the sun's \"direction\"");
                let sky = parse_sky_material(sky);
                let direction = sky.sun_direction();
                Some(Sun { atmosphere: Some(Arc::new(sky)), ..Sun::new(direction, irradiance) })
            },
            None => {
                let d = v.get("direction").map(SceneFile::parse_vec3)
                    .expect("A sun without a skysphere needs a \"direction\"");
                assert!(d.norm() > 0., "The sun's direction must not be zero");
                Some(Sun::new(d, irradiance))
            },
        }.map(|mut sun| {
            if let Some(r) = v.get("angular_radius") {
                sun = sun.with_angular_radius(SceneFile::parse_number(r, 0.).to_radians().max(1e-6));
            }
            sun
        })
    }

    /// A colour (black if absent), `{"type": "gradient", "bottom": colour, "top": colour}`, or
    /// `{"type": "space", ...}` (`procedural::space`), for a camera whose pixels are
    /// `pixel_angle` radians across, on the scale of a sun of irradiance `sun` (its luminance)
    /// if the scene has one.
    pub fn parse_background(v: &Value, pixel_angle: f64, sun: Option<f64>) -> Background {
        match v {
            Value::Null => Background::Solid(Color::black()),
            Value::Object(_) => match v["type"].as_str() {
                Some("gradient") => Background::Gradient {
                    bottom: SceneFile::parse_color_def(v, "bottom", Color::white()),
                    top: SceneFile::parse_color_def(v, "top", Color::new(0.5, 0.7, 1.)),
                },
                Some("space") => Background::Space(Arc::new(crate::procedural::space::Space::parse(v, pixel_angle, sun))),
                // {"type": "overcast", "zenith": colour, "ground": colour}: the CIE overcast sky.
                Some("overcast") => {
                    let zenith = SceneFile::parse_color_def(v, "zenith", Color::new(1., 1., 1.));
                    // "bank": {"direction": [x, z], "darkness": 0.3}: darker cloud low that way.
                    let bank = v.get("bank").map(|b| {
                        let d = SceneFile::parse_vec2_def(b, "direction", Vector2::new(0., 1.));
                        (Vector3::new(d.x, 0., d.y).normalize(), SceneFile::parse_number(&b["darkness"], 0.3).clamp(0., 1.))
                    });
                    Background::Overcast { zenith, ground: SceneFile::parse_color_def(v, "ground", zenith * (1. / 3.)), bank }
                },
                t => panic!("Unknown background type {:?}: expected \"gradient\", \"space\", \"overcast\" or a colour", t),
            },
            _ => Background::Solid(SceneFile::parse_color(v)),
        }
    }

    pub fn parse_path_termination(v: &Value, rr_depth: &Value) -> PathTermination {
        match v.as_str() {
            None | Some("black") => PathTermination::Black,
            Some("russian_roulette") => PathTermination::RussianRoulette {
                min_depth: SceneFile::parse_int(rr_depth, 3),
            },
            Some(x) => panic!("Unknown path_termination '{}': expected \"black\" or \"russian_roulette\"", x),
        }
    }

    pub fn from_scenefile(mut s: SceneFile) -> Scene {
        s.camera = SceneFile::axonometric(std::mem::take(&mut s.camera));
        SceneFile::sky_follows_sun(&s.sun, &mut s.objects);
        SceneFile::sky_cache(&mut s.objects);
        SceneFile::lightning_follows_sun(&s.sun, &mut s.objects);
        let max_bounding = BBox::new( // TODO
            Vector3::new(-1000., -1000., -1000.),
            Vector3::new(1000., 1000., 1000.),
        );

        let sun = SceneFile::parse_sun(&s.sun, &s.objects);
        // The camera's view plane spans tan(angle) over the image's height.
        let pixel_angle = s.camera["angle"].as_f64().map_or(0., |a| a.tan() / SceneFile::parse_int(&s.height, 480) as f64);
        // Lightning is drawn at least about half a pixel wide (`procedural::lightning`).
        for o in s.objects.iter_mut().filter(|o| o["type"].as_str() == Some("lightning") && o.get("camera").is_none()) {
            o["camera"] = s.camera["location"].clone();
            o["pixel_angle"] = serde_json::json!(pixel_angle);
        }
        let flare = SceneFile::parse_flare(&s.output, &s.camera, SceneFile::parse_int(&s.width, 640), SceneFile::parse_int(&s.height, 480));
        // For texture filtering: the camera's position, axes and the angle between pixels, shrunk
        // by the samples per pixel (`textures::footprint`).
        let footprint = (pixel_angle > 0.).then(|| {
            let f = camera::Frame::look_at(SceneFile::parse_vec3(&s.camera["location"]), SceneFile::parse_vec3(&s.camera["lookat"]),
                SceneFile::parse_vec3_def(&s.camera, "up", Vector3::y()));
            let spp = SceneFile::parse_int(&s.supersamples, 35).max(1) as f64;
            crate::textures::Footprint { location: f.location, right: f.camx, up: f.camy, spread: pixel_angle * (1. / spp.sqrt()).max(0.125) }
        });
        // Aerial perspective, unless the skysphere turns it off.
        let atmosphere = s.objects.iter()
            .find(|o| o["type"].as_str() == Some("skysphere") && o["aerial_perspective"].as_bool() != Some(false))
            .map(|o| Arc::new(parse_sky_material(o)));
        // Motion blur: sample times only if something moves (`motion`).
        let motion = SceneFile::has_motion(&s.camera) || s.objects.iter().any(SceneFile::has_motion)
            || s.geometries.values().any(SceneFile::has_motion);
        // A displaced terrain is diced for the camera (`parse_displaced_terrain`), named or not.
        let mut named = s.geometries;
        let mut objects = s.objects;
        SceneFile::displaced_instances(&mut objects, &mut named);
        for g in named.values_mut() {
            if g["type"].as_str() == Some("terrain") && g.get("displacement").is_some() && g.get("camera").is_none() {
                g["camera"] = s.camera.clone();
                g["camera"]["image_height"] = serde_json::json!(SceneFile::parse_int(&s.height, 480));
                g["camera"]["image_width"] = serde_json::json!(SceneFile::parse_int(&s.width, 640));
            }
        }
        let geometries = GeometryLibrary::new(named);
        geometries.preload(&objects);
        // A landscape is meshed finest where the camera is.
        let mut scene_objects = objects;
        // A world from a seed becomes the giant or the rocky or icy body it turns out to be.
        for o in scene_objects.iter_mut() {
            if o["type"].as_str() == Some("world") {
                *o = crate::celestial::world_object(o);
            }
        }
        let planets: Vec<Value> = scene_objects.iter()
            .filter(|o| matches!(o["type"].as_str(), Some("skysphere") | Some("planet"))).cloned().collect();
        for o in scene_objects.iter_mut() {
            if o["type"].as_str() == Some("landscape") && o.get("camera").is_none() {
                o["camera"] = s.camera.clone();
                // Its displacement is diced to the camera's pixels.
                o["camera"]["image_height"] = serde_json::json!(SceneFile::parse_int(&s.height, 480));
                o["camera"]["image_width"] = serde_json::json!(SceneFile::parse_int(&s.width, 640));
            }
            // A moon draws craters down to the camera's pixels, and orbits the scene's planet.
            if matches!(o["type"].as_str(), Some("moon") | Some("gas_giant")) {
                if o.get("camera").is_none() {
                    o["camera"] = s.camera.clone();
                    o["camera"]["image_height"] = serde_json::json!(SceneFile::parse_int(&s.height, 480));
                }
                if o.get("planet").is_none() {
                    let (centre, radius) = crate::celestial::moon::scene_primary(&planets);
                    o["planet"] = serde_json::json!({ "centre": [centre.x, centre.y, centre.z], "radius": radius });
                }
            }
            // A forest sees each tree at the detail the camera needs (`parse_forest_lod`), and a
            // displaced terrain is diced as finely (`parse_displaced_terrain`).
            let displaced = o["type"].as_str() == Some("terrain") && o.get("displacement").is_some();
            if (matches!(o["type"].as_str(), Some("forest" | "ecosystem")) || displaced) && o.get("camera").is_none() {
                o["camera"] = s.camera.clone();
                o["camera"]["image_height"] = serde_json::json!(SceneFile::parse_int(&s.height, 480));
                o["camera"]["image_width"] = serde_json::json!(SceneFile::parse_int(&s.width, 640));
            }
        }
        // Weather becomes its families of cloud, which know the sky's sun and the landscape.
        let mut scene_objects = crate::weather::scene::expand(scene_objects);
        let sea = scene_objects.iter().find(|o| o["type"].as_str() == Some("ocean")).map(crate::ocean::Sea::parse);
        // A moon lights the scene with the sunlight it reflects (unless `"light": false`), and
        // a skysphere's air scatters it too.
        let moonlight = match (&sun, scene_objects.iter().find(|o| o["type"].as_str() == Some("moon") && o["light"].as_bool() != Some(false))) {
            (Some(sun), Some(moon)) => {
                let eye = SceneFile::parse_vec3_def(&s.camera, "location", Vector3::zeros());
                let sky = scene_objects.iter().find(|o| o["type"].as_str() == Some("skysphere"));
                Some(crate::celestial::moon::Moonlight::new(moon, &eye, sun, sky))
            },
            _ => None,
        };
        if let (Some(m), Some(sun)) = (&moonlight, &sun) {
            let share = crate::color::luminance(&m.light.irradiance) / crate::color::luminance(&sun.irradiance).max(1e-300);
            let d = m.light.direction;
            for o in scene_objects.iter_mut().filter(|o| o["type"].as_str() == Some("skysphere")) {
                o["moonlight"] = serde_json::json!({ "direction": [d.x, d.y, d.z], "share": share });
            }
        }
        let object_sources = SceneFile::parse_objects_indexed(scene_objects, &s.materials, &s.media, &geometries);
        let objects: Vec<Arc<SceneObject>> = object_sources.iter().map(|(_, o)| o.clone()).collect();
        // Emitters with a surface we can sample become area lights.
        let area_lights: Vec<Arc<SceneObject>> = objects.iter()
            .filter(|o| o.medium.material_at(Vector3::zeros()).is_emitter() && o.geometry.surface_area() > 0.)
            .cloned()
            .collect();
        let area_light_choice = crate::environment::Distribution1D::new(SceneFile::light_powers(&area_lights));
        let o = SceneGraph::new(objects);
		
        let width = SceneFile::parse_int(&s.width, 640);
        let height =SceneFile::parse_int(&s.height, 480);

        let mut scene = Scene {
            image: ImageOpts { width, height },
            render: RenderOpts {
                max_depth: SceneFile::parse_int(&s.max_depth, 2),
                background: SceneFile::parse_background(&s.background, pixel_angle,
                    sun.as_ref().map(|s| crate::procedural::space::luminance(&s.irradiance.rgb))),
                shadow_bias: SceneFile::parse_number(&s.shadow_bias, 1e-7f64),
                supersamples: SceneFile::parse_int(&s.supersamples, 35),  
                chunk_size: SceneFile::parse_int(&s.chunk_size, 64).max(1), 
                samples_per_chunk: SceneFile::parse_int(&s.samples_per_chunk, 2),
                path_termination: SceneFile::parse_path_termination(&s.path_termination, &s.russian_roulette_depth),
                seed: SceneFile::parse_int(&s.seed, 0) as u64,
                sample_lights: s.sample_lights.as_bool().unwrap_or(true),
                sample_sky: s.sample_sky.as_bool().unwrap_or(true),
                filter: SceneFile::parse_filter(&s.filter),
                regularise: SceneFile::parse_number(&s.regularise, crate::material::ggx::DEFAULT_REGULARISE),
                spectral: s.spectral.as_bool().unwrap_or(false),
                shutter: SceneFile::parse_shutter(&s.shutter),
                motion,
                integrator: match s.integrator.as_str() {
                    None | Some("auto") => crate::scene::Integrator::Auto,
                    Some("path") => crate::scene::Integrator::Path,
                    Some("wavefront") => crate::scene::Integrator::Wavefront,
                    Some("gpu") => crate::scene::Integrator::Gpu,
                    Some(x) => panic!("Unknown integrator '{}': expected \"auto\", \"path\", \"wavefront\" or \"gpu\"", x),
                },
                adaptive: SceneFile::parse_adaptive(&s.adaptive),
                sampler: match s.sampler.as_str() {
                    None | Some("sobol") => Sampler::Sobol,
                    Some("independent") => Sampler::Independent,
                    Some("halton") => Sampler::Halton,
                    Some("stratified") => Sampler::Stratified,
                    Some("zsobol") => Sampler::ZSobol,
                    Some(x) => panic!("Unknown sampler '{}': expected \"sobol\", \"zsobol\", \"halton\", \"stratified\" or \"independent\"", x),
                },
            },
            footprint,
            camera: SceneFile::parse_camera(s.camera, width as u32, height as u32),
            lights: SceneFile::parse_lights(&s.lights),
            objects: o,
            max_bounding,
            black_threshold: SceneFile::parse_number(&s.black_threshold, 1e-7f64),
            air_medium: SceneFile::parse_air(&s.air),
            output: OutputOpts {
                // The camera's view plane spans tan(angle) over the image's height.
                pixel_angle,
                flare,
                ..OutputOpts::parse(&s.output)
            },
            area_light_indices: area_lights.iter().enumerate().map(|(i, l)| (&**l as *const SceneObject as usize, i)).collect(),
            area_lights,
            object_sources,
            area_light_choice,
            sun,
            moonlight,
            atmosphere,
            environment: None,
            light_bvh: None,
            sea,
        };
        scene.light_bvh = match s.light_sampler.as_str() {
            None | Some("power") => None,
            Some("bvh") => Some(SceneFile::light_bvh(&scene)),
            Some(x) => panic!("Unknown light_sampler '{}': expected \"power\" or \"bvh\"", x),
        };
        // Tabulating the sky evaluates it, which needs the scene.
        if scene.render.sample_sky {
            let start = web_time::Instant::now();
            scene.environment = crate::environment::Environment::build(&scene);
            let t = start.elapsed().as_secs_f64();
            if t >= 0.1 { SceneFile::say(format!("- Built the sky's sampling table in {:.2} s", t)); }
        }
        scene
    }

    /// A line about the build, for a person: printed, or given to the host's reporter
    /// (`world::set_reporter`; a browser shows it on its console, where it can't print).
    fn say(message: String) {
        crate::world::report(&crate::world::Progress::note("scene", message));
    }

    /// The light BVH over a scene's sampled lights (`Scene::light_bvh`): its area lights, if
    /// `sample_lights` is on, then its point lights if there are more than
    /// `scene::BVH_POINT_LIGHTS` of them.
    fn light_bvh(scene: &Scene) -> crate::light_bvh::LightBvh {
        use crate::light_bvh::LightBounds;
        use crate::light::Profile;
        let mut bounds: Vec<LightBounds> = Vec::new();
        let luminance = |c: Color| crate::color::luminance(&c).max(0.);
        for l in scene.area_lights.iter().take(scene.bvh_area_lights()) {
            // Normals at a few points: one for flat lights (quads, discs, triangles), which
            // then emit only to one side, or any.
            let normals: Vec<(Vector3<f64>, Vector3<f64>)> = (0..9)
                .filter_map(|k| l.geometry.sample_surface([(k % 3) as f64 / 3. + 0.17, (k / 3) as f64 / 3. + 0.17, 0.5]))
                .collect();
            let Some(&(y, n)) = normals.first() else { continue };
            let flat = normals.iter().all(|(_, m)| (m - n).norm() < 1e-6);
            let material = l.medium.material_at(y);
            let front = luminance(material.emitted(&n, &n));
            let two_sided = luminance(material.emitted(&n, &-n)) > 0.;
            // A diffuse emitter's power: π L A, from each side that emits.
            let phi = std::f64::consts::PI * front * l.geometry.surface_area() * if two_sided { 2. } else { 1. };
            let b = l.geometry.bounds();
            bounds.push(LightBounds {
                bounds: b, phi,
                w: n,
                cos_theta_o: if flat { 1. } else { -1. },
                cos_theta_e: 0.,
                two_sided: flat && two_sided,
            });
        }
        // A narrow spread about the axis for beams, which can't be exactly zero.
        let beam_spread = 1e-4f64.cos();
        for light in scene.lights.iter().filter(|_| scene.lights.len() > crate::scene::BVH_POINT_LIGHTS) {
            // As pbrt: 4π times the intensity, its power were it to shine every way, so that
            // a beam's importance follows the light it delivers where it points, not how
            // little of the sphere it covers.
            let image = match &light.profile {
                Profile::Goniometric { image, .. } | Profile::Projection { image, .. } => luminance(image.average()),
                _ => 1.,
            };
            let phi = 4. * std::f64::consts::PI * luminance(light.emission.srgb()) * light.intensity * image;
            bounds.push(match &light.profile {
                Profile::Spot { axis, cos_total, .. } => LightBounds { cos_theta_o: *cos_total, cos_theta_e: beam_spread, w: *axis, ..LightBounds::point(light.position, phi) },
                Profile::Projection { frame, half, .. } => {
                    let w = Vector3::new(frame[(2, 0)], frame[(2, 1)], frame[(2, 2)]);
                    LightBounds { cos_theta_o: half.norm().atan().cos(), cos_theta_e: beam_spread, w, ..LightBounds::point(light.position, phi) }
                },
                _ => LightBounds::point(light.position, phi),
            });
        }
        crate::light_bvh::LightBvh::new(&bounds)
    }

    /// Roughly each area light's power, to choose lights to sample in proportion to it: the
    /// luminance it emits along its normal at one point, times its area. Sampling the light
    /// that delivers more light more often is Pharr, Jakob and Humphreys's power light sampler
    /// ("Physically Based Rendering", 4th ed., section 12.6.2,
    /// <https://pbr-book.org/4ed/Light_Sources/Light_Sampling>). Any choice that can pick every
    /// emitting light is unbiased, so every light gets at least a hundredth of the mean, in case
    /// the one point misjudges a textured emitter.
    fn light_powers(lights: &[Arc<SceneObject>]) -> Vec<f64> {
        let powers: Vec<f64> = lights.iter().map(|l| {
            let emitted = match l.geometry.sample_surface([0.5, 0.5, 0.5]) {
                Some((y, n)) => l.medium.material_at(y).emitted(&n, &n),
                None => Color::black(),
            };
            let e = emitted.rgb;
            (0.2126 * e.x + 0.7152 * e.y + 0.0722 * e.z).max(0.) * l.geometry.surface_area()
        }).collect();
        let mean = powers.iter().sum::<f64>() / powers.len().max(1) as f64;
        powers.iter().map(|p| p.max(0.01 * mean).max(f64::MIN_POSITIVE)).collect()
    }

    pub fn from_string(s: String) -> Scene {
        SceneFile::from_string_overridden(s, &Overrides::default())
    }

    /// Parse a scene, optionally overriding the image size. Overrides must be applied before
    /// the camera is built, as it derives its aspect ratio from the image size.
    pub fn from_string_sized(s: String, width: Option<usize>, height: Option<usize>) -> Scene {
        SceneFile::from_string_overridden(s, &Overrides { width, height, ..Overrides::default() })
    }

    /// Parse a scene, applying command-line overrides first.
    /// Substitute the scene's variables. `"variables": {"sun": [0.1, 0.4, 1]}` defines `sun`;
    /// the string `"$sun"`, anywhere else in the file, stands for its value. Variables may use
    /// each other, in any order, but not in a cycle. An unknown name is an error, rather than a
    /// string passed on to a parser expecting something else.
    pub fn substitute_variables(mut doc: Value) -> Result<Value, String> {
        struct Resolver { defined: Map<String, Value>, resolved: Map<String, Value>, active: Vec<String> }

        impl Resolver {
            fn variable(&mut self, name: &str) -> Result<Value, String> {
                if let Some(v) = self.resolved.get(name) {
                    return Ok(v.clone());
                }
                if self.active.iter().any(|a| a == name) {
                    return Err(format!("variables refer to each other in a cycle: ${} -> ${}", self.active.join(" -> $"), name));
                }
                let mut v = self.defined.get(name).cloned().ok_or_else(|| format!("unknown variable \"${}\"", name))?;
                self.active.push(name.to_string());
                self.substitute(&mut v)?;
                self.active.pop();
                self.resolved.insert(name.to_string(), v.clone());
                Ok(v)
            }

            fn substitute(&mut self, v: &mut Value) -> Result<(), String> {
                match v {
                    Value::String(s) if s.starts_with('$') => { let name = s[1 ..].to_string(); *v = self.variable(&name)? },
                    Value::Array(items) => for i in items { self.substitute(i)? },
                    Value::Object(fields) => for (_, f) in fields.iter_mut() { self.substitute(f)? },
                    _ => {},
                }
                Ok(())
            }
        }

        let defined = match doc.get_mut("variables").map(Value::take) {
            Some(Value::Object(m)) => m,
            None | Some(Value::Null) => Map::new(),
            Some(v) => return Err(format!("\"variables\" must be an object, got {}", v)),
        };
        let mut r = Resolver { defined, resolved: Map::new(), active: Vec::new() };
        r.substitute(&mut doc)?;
        Ok(doc)
    }

    pub fn from_string_overridden(s: String, o: &Overrides) -> Scene {
        SceneFile::from_value_overridden(serde_json::from_str(&s).unwrap(), o)
    }

    /// Build a scene from a scene file's JSON, applying command-line overrides first.
    pub fn from_value_overridden(doc: Value, o: &Overrides) -> Scene {
        let (width, height) = (o.width, o.height);
        let mut doc = SceneFile::substitute_variables(doc)
            .unwrap_or_else(|e| panic!("Scene file: {}", e));
        crate::weather::scene::choose_seeds(&mut doc);
        let mut scene: SceneFile = serde_json::from_value(doc).unwrap();
        if let Some(seed) = o.seed {
            scene.seed = Value::from(seed);
        }
        if let Some(w) = width {
            scene.width = Value::from(w);
        }
        if let Some(h) = height {
            scene.height = Value::from(h);
        }
        SceneFile::from_scenefile(scene)
    }

    /// A scene file's JSON. Files in another renderer's format are translated into ours, chosen
    /// by extension: `.nff` (see `crate::nff`); or, for JSON, by content: Tungsten's (see
    /// `crate::tungsten`). Anything else is read as our JSON.
    pub fn read_document(filename: &str) -> Result<Value, String> {
        let contents = String::from_utf8(crate::assets::read(filename).map_err(|e| format!("Can't read scene file {}: {}", filename, e))?)
            .map_err(|e| format!("Scene file {}: {}", filename, e))?;
        if filename.to_lowercase().ends_with(".nff") {
            return crate::nff::to_scene(&contents).map_err(|e| format!("{}: {}", filename, e));
        }
        let doc: Value = serde_json::from_str(&contents).map_err(|e| format!("{}: {}", filename, e))?;
        if crate::tungsten::is_tungsten(&doc) {
            let dir = std::path::Path::new(filename).parent().unwrap_or(std::path::Path::new(""));
            return crate::tungsten::to_scene(&doc, dir).map_err(|e| format!("{}: {}", filename, e));
        }
        Ok(doc)
    }

    /// Where the worlds of the scene's full-scale landscapes are kept (their manifests,
    /// `terrain::object::world_manifest`), whether built yet or not: for a host without a
    /// filesystem to fetch and provide before building the scene. Each landscape sees the
    /// scene's camera as `from_scenefile` gives it one.
    /// The files of tables the scene keeps in a `cache` (its skies', `skysphere::table_file`),
    /// whether made yet or not: as `world_manifests`, for a host without a filesystem to fetch
    /// before building the scene.
    pub fn cached_files(doc: Value) -> Vec<String> {
        let Ok(doc) = SceneFile::substitute_variables(doc) else { return vec![] };
        let mut objects: Vec<Value> = doc["objects"].as_array().cloned().unwrap_or_default();
        SceneFile::sky_follows_sun(&doc["sun"], &mut objects);
        SceneFile::sky_cache(&mut objects);
        objects.iter()
            .filter(|o| o["type"].as_str() == Some("skysphere") && o["lut"].as_bool() != Some(false))
            .filter_map(|o| Some(crate::skysphere::table_file(o["cache"].as_str()?, o)))
            .collect()
    }

    pub fn world_manifests(doc: Value) -> Vec<String> {
        let Ok(doc) = SceneFile::substitute_variables(doc) else { return vec![] };
        let camera = SceneFile::axonometric(doc["camera"].clone());
        doc["objects"].as_array().into_iter().flatten()
            .filter(|o| o["type"].as_str() == Some("landscape"))
            .filter_map(|o| {
                let mut o = o.clone();
                if o.get("camera").is_none() { o["camera"] = camera.clone(); }
                crate::terrain::object::world_manifest(&o)
            })
            .collect()
    }

    pub fn from_file(filename: &str, overrides: &Overrides) -> Scene {
        let doc = SceneFile::read_document(filename).unwrap_or_else(|e| panic!("{}", e));
        SceneFile::from_value_overridden(doc, overrides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;

    #[test]
    fn test_hill_cloud_joins_an_overcast() {
        let hills = serde_json::json!({ "type": "mountains", "extent": 3000, "resolution": 16, "peak_height": 600,
            "peak_radius": 2000, "cloud": { "deficit": 250, "resolution": 64 } });
        let deck = serde_json::json!({ "type": "overcast", "base": 1200, "extent": 5000 });
        let none = Map::new();
        let library = GeometryLibrary::new(Map::new());
        // With a deck: the hills and the deck, the cloud in the deck's volume.
        let joined = SceneFile::parse_objects(vec![hills.clone(), deck.clone()], &none, &none, &library);
        assert_eq!(joined.len(), 2);
        // Without one, or told not to join: a volume of its own.
        assert_eq!(SceneFile::parse_objects(vec![hills.clone()], &none, &none, &library).len(), 2);
        let mut apart = hills.clone();
        apart["cloud"]["join"] = serde_json::json!(false);
        assert_eq!(SceneFile::parse_objects(vec![apart, deck], &none, &none, &library).len(), 3);
    }

    #[test]
    fn test_variables_are_substituted() {
        let doc = serde_json::json!({
            "variables": { "warm": [1, 0.8, "$half"], "half": 0.5, "big": { "radius": "$half" } },
            "objects": [{ "color": "$warm", "shape": "$big", "name": "plain string" }],
        });
        let out = SceneFile::substitute_variables(doc).unwrap();
        assert_eq!(out["objects"][0]["color"], serde_json::json!([1, 0.8, 0.5]));
        assert_eq!(out["objects"][0]["shape"], serde_json::json!({ "radius": 0.5 }));
        assert_eq!(out["objects"][0]["name"], "plain string");

        // And through to the parsed scene.
        let s = minimal_scene(r#", "variables": { "grey": [0.25, 0.25, 0.25] }, "background": "$grey""#);
        assert_eq!(s.render.background.radiance(&Vector3::new(0., 0., 1.)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn test_variable_errors() {
        let unknown = SceneFile::substitute_variables(serde_json::json!({ "a": "$nope" })).unwrap_err();
        assert!(unknown.contains("unknown variable \"$nope\""), "{}", unknown);
        let cycle = SceneFile::substitute_variables(serde_json::json!({
            "variables": { "a": "$b", "b": ["$a"] }, "x": "$a",
        })).unwrap_err();
        assert_eq!(cycle, "variables refer to each other in a cycle: $a -> $b -> $a");
        assert!(SceneFile::substitute_variables(serde_json::json!({ "variables": [1] })).is_err());
    }

    fn minimal_scene(extra: &str) -> Scene {
        minimal_scene_sized(extra, None, None)
    }

    fn minimal_scene_sized(extra: &str, width: Option<usize>, height: Option<usize>) -> Scene {
        SceneFile::from_string_sized(format!(r#"{{
            "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
            "shadow_bias": 0.5, "background": [0, 0, 0], "max_depth": 1,
            "camera": {{ "location": [0, 0, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{}}, "media": {{}}, "objects": [], "variables": {{}}
            {}
        }}"#, extra), width, height)
    }

    fn library(defs: Value) -> GeometryLibrary {
        GeometryLibrary::new(serde_json::from_value(defs).unwrap())
    }

    #[test]
    fn test_named_geometry_is_built_once_and_shared() {
        let lib = library(serde_json::json!({
            "ball": { "type": "sphere", "location": [0, 0, 0], "radius": 1 },
            "pair": { "type": "transform", "item": "ball", "scale": 2 },
        }));
        let a = lib.get("ball");
        assert!(Arc::ptr_eq(&a, &lib.get("ball")));
        // A transform of a named geometry is an instance of it, not a copy.
        let t = serde_json::json!({ "type": "transform", "item": "ball", "translate": [5, 0, 0] });
        let _instances: Vec<_> = (0 .. 3).map(|_| SceneFile::parse_geometry(&t, &lib).unwrap()).collect();
        assert_eq!(Arc::strong_count(&a), 2 + 3);
        // Definitions can use each other; "pair" holds one more reference to "ball".
        assert!((lib.get("pair").bounds().max.x - 2.).abs() < 1e-9);
        assert_eq!(Arc::strong_count(&a), 2 + 3 + 1);
        // A bare name is a geometry anywhere one is expected.
        let d = serde_json::json!({ "type": "difference", "a": "pair", "b": "ball" });
        assert!(SceneFile::parse_geometry(&d, &lib).is_some());
    }

    #[test]
    fn test_instances_have_their_own_materials_and_transforms() {
        let s = minimal_scene(r#",
            "geometries": { "bulb": { "type": "sphere", "location": [0, 0, 0], "radius": 1 } },
            "materials": {
                "warm": { "type": "diffuse-light", "color": [1, 0.5, 0] },
                "cool": { "type": "diffuse-light", "color": [0, 0.5, 1] }
            },
            "objects": [
                { "type": "transform", "item": "bulb", "translate": [-2, 0, 0], "material": "warm" },
                { "type": "transform", "item": "bulb", "translate": [2, 0, 0], "scale": 0.5, "material": "cool" }
            ]"#);
        let mut areas: Vec<f64> = s.area_lights.iter().map(|o| o.geometry.surface_area()).collect();
        areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let pi4 = 4. * std::f64::consts::PI;
        assert!((areas[0] - pi4 / 4.).abs() < 1e-9 && (areas[1] - pi4).abs() < 1e-9, "{:?}", areas);
    }

    #[test]
    #[should_panic(expected = "geometries refer to each other in a cycle: a -> b -> a")]
    fn test_geometry_cycle_is_an_error() {
        library(serde_json::json!({
            "a": { "type": "transform", "item": "b" },
            "b": { "type": "transform", "item": "a" },
        })).get("a");
    }

    #[test]
    #[should_panic(expected = "unknown geometry \"nope\"")]
    fn test_unknown_geometry_is_an_error() {
        SceneFile::parse_geometry(&serde_json::json!({ "type": "transform", "item": "nope" }), &GeometryLibrary::empty());
    }

    fn tetrahedron_obj() -> String {
        let dir = std::env::temp_dir().join(format!("rays-scenefile-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tetra.obj");
        std::fs::write(&path, "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\nf 1 3 2\nf 1 2 4\nf 1 4 3\nf 2 3 4\n").unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_mesh_files_are_loaded_once() {
        let src = tetrahedron_obj();
        let lib = library(serde_json::json!({
            "used": { "type": "smoothmesh", "src": src },
            "unused": { "type": "mesh", "src": src, "scale": 3 },
        }));
        let inline = serde_json::json!({ "type": "mesh", "src": src });
        let objects = vec![
            serde_json::json!({ "type": "transform", "item": inline.clone(), "translate": [1, 0, 0] }),
            serde_json::json!({ "type": "transform", "item": inline.clone(), "translate": [2, 0, 0] }),
            serde_json::json!({ "type": "transform", "item": "used" }),
            serde_json::json!({ "type": "mesh", "src": src, "scale": [2, 2, 2] }),
        ];
        lib.preload(&objects);
        // The inline mesh (twice), "used", and the scaled copy; not "unused".
        assert_eq!(lib.meshes.borrow().len(), 3);
        // Inline definitions of the same file and scale share one mesh, loaded by `preload`.
        let a = lib.mesh(&inline);
        assert!(Arc::ptr_eq(&a, &lib.mesh(&inline)));
        assert!(Arc::ptr_eq(&lib.get("used"), &lib.mesh(&serde_json::json!({ "type": "smoothmesh", "src": src }))));
        assert_eq!(lib.meshes.borrow().len(), 3);
        // A different scale is a different mesh.
        let scaled = lib.mesh(&objects[3]);
        assert!(!Arc::ptr_eq(&a, &scaled));
        assert!((scaled.bounds().max.x - 2.).abs() < 1e-9);
    }

    #[test]
    fn test_parse_phase() {
        use serde_json::json;
        assert_eq!(SceneFile::parse_phase(&json!({}), 0.3), PhaseFunction::HenyeyGreenstein { g: 0.3 });
        assert_eq!(SceneFile::parse_phase(&json!({ "g": -0.2 }), 0.3), PhaseFunction::HenyeyGreenstein { g: -0.2 });
        assert_eq!(SceneFile::parse_phase(&json!({ "droplet_diameter": 12 }), 0.3), PhaseFunction::water_droplets(12.));
    }

    #[test]
    #[should_panic(expected = "not both")]
    fn test_parse_phase_rejects_both() {
        SceneFile::parse_phase(&serde_json::json!({ "g": 0.5, "droplet_diameter": 12 }), 0.);
    }

    #[test]
    fn test_parse_seed() {
        assert_eq!(SceneFile::parse_seed(&serde_json::json!({ "type": "noise" })), None);
        assert_eq!(SceneFile::parse_seed(&serde_json::json!({ "seed": 9 })), Some(9));
    }

    #[test]
    fn test_missing_fields_use_defaults() {
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 0, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }
        }"#.to_string());
        assert_eq!((s.image.width, s.image.height), (640, 480));
        assert_eq!(s.render.supersamples, 35);
        assert_eq!(s.render.max_depth, 2);
        assert_eq!(s.render.background, Background::Solid(Color::black()));
        assert!(s.lights.is_empty());
    }

    #[test]
    fn test_parse_int_accepts_whole_floats() {
        assert_eq!(SceneFile::parse_int(&serde_json::json!(640.0), 1), 640);
        assert_eq!(SceneFile::parse_int(&serde_json::json!(640), 1), 640);
        assert_eq!(SceneFile::parse_int(&Value::Null, 7), 7);
    }

    #[test]
    #[should_panic]
    fn test_parse_int_rejects_negative() {
        SceneFile::parse_int(&serde_json::json!(-1), 1);
    }

    #[test]
    fn test_light_colours() {
        let s = minimal_scene(r#", "lights": [
            { "location": [0, 1, 0], "intensity": 1, "color": [1, 0, 0] },
            { "location": [0, 1, 0], "intensity": 1, "color": "white" },
            { "location": [0, 1, 0], "intensity": 1 }
        ]"#);
        assert_eq!(s.lights[0].emission.srgb(), Color::red());
        assert_eq!(s.lights[1].emission.srgb(), Color::white());
        assert_eq!(s.lights[2].emission.srgb(), Color::white());
    }

    fn noise_materials() -> Map<String, Value> {
        serde_json::from_value(serde_json::json!({
            "BASE": { "type": "lambertian", "albedo": [1, 1, 1] },
            "N": { "type": "noise", "base_material": "BASE", "noise_type": "fbm", "octaves": 0 },
            "NN": { "type": "noise", "base_material": "N" },
            "TYPO": { "type": "Lambertian", "albedo": [1, 1, 1] }
        })).unwrap()
    }

    #[test]
    fn test_noise_material_composes_everywhere() {
        let m = noise_materials();
        let media = Map::new();
        for medium in [
            serde_json::json!({ "type": "solid", "material": "N" }),
            serde_json::json!({ "type": "checkered-y-plane", "m1": "N", "m2": "NN" }),
            serde_json::json!({ "type": "noise_medium", "m1": "N", "m2": "BASE" }),
        ] {
            assert!(SceneFile::parse_medium(&medium, &m).is_some(), "{}", medium);
        }
        assert!(SceneFile::parse_material_ref(&Value::from("NN"), &m).is_some());
        let obj = serde_json::json!({ "type": "sphere", "material": "N" });
        SceneFile::parse_object_medium(&obj, &m, &media);
    }

    #[test]
    #[should_panic(expected = "Material 'TYPO' has unknown type")]
    fn test_unknown_material_type_names_it() {
        SceneFile::parse_material_ref(&Value::from("TYPO"), &noise_materials());
    }

    #[test]
    #[should_panic(expected = "MISSING")]
    fn test_missing_material_names_it() {
        let obj = serde_json::json!({ "type": "sphere", "material": "MISSING" });
        SceneFile::parse_object_medium(&obj, &noise_materials(), &Map::new());
    }

    #[test]
    fn test_gradient_background() {
        let g = SceneFile::parse_background(&serde_json::json!({ "type": "gradient" }), 0., None);
        // The book's colours by default: white looking down, (0.5, 0.7, 1) looking up.
        assert_eq!(g.radiance(&Vector3::new(0., -1., 0.)), Color::white());
        assert_eq!(g.radiance(&Vector3::new(0., 2., 0.)), Color::new(0.5, 0.7, 1.));
        assert_eq!(g.radiance(&Vector3::new(1., 0., 0.)), Color::new(0.75, 0.85, 1.));
        assert_eq!(SceneFile::parse_background(&serde_json::json!([0.1, 0.2, 0.3]), 0., None), Background::Solid(Color::new(0.1, 0.2, 0.3)));
    }

    #[test]
    fn test_parse_transform() {
        let t = SceneFile::parse_geometry(&serde_json::json!({
            "type": "transform",
            "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 },
            "scale": 2, "rotate": { "yaw": 90 }, "translate": [0, 0, 5]
        }), &GeometryLibrary::empty()).unwrap();
        let b = t.bounds();
        assert!((b.min.coords - Vector3::new(-2., -2., 3.)).norm() < 1e-9, "{}", b);
        let per_axis = SceneFile::parse_geometry(&serde_json::json!({
            "type": "transform",
            "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 }, "scale": [3, 1, 1]
        }), &GeometryLibrary::empty()).unwrap();
        assert!((per_axis.bounds().max.x - 3.).abs() < 1e-9);
    }

    #[test]
    fn test_normal_bump_maps_the_object() {
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 0, -10], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "objects": [
                { "type": "sphere", "location": [0, 0, 0], "radius": 1,
                  "normal": { "pattern": "bumps", "amount": 0.8, "scale": 0.2 } },
                { "type": "sphere", "location": [5, 0, 0], "radius": 1 }
            ]
        }"#.to_string());
        let tilt = |x: f64| {
            let r = crate::ray::Ray { ro: Vector3::new(x + 0.3, 0.2, -5.), rd: Vector3::new(0., 0., 1.) };
            let i = s.objects.nearest_intersection(&r, f64::INFINITY, 0.).unwrap();
            (i.object.medium.normal_at(&i.point, &i.normal) - i.normal).norm()
        };
        assert!(tilt(0.) > 1e-3);
        assert_eq!(tilt(5.), 0.);
    }

    #[test]
    #[should_panic(expected = "normal \"pattern\" must be")]
    fn test_unknown_normal_pattern() {
        SceneFile::parse_bump(&serde_json::json!({ "pattern": "lumps" }));
    }

    #[test]
    fn test_transformed_emitters_are_area_lights_when_uniformly_scaled() {
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 0, -10], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "L": { "type": "diffuse-light", "color": [1, 1, 1] } },
            "objects": [
                { "type": "transform", "scale": 2, "translate": [0, 5, 0], "material": "L",
                  "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 } },
                { "type": "transform", "scale": [2, 1, 1], "material": "L",
                  "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 } }
            ]
        }"#.to_string());
        assert_eq!(s.area_lights.len(), 1);
    }

    fn hit(g: &dyn Geometry, ro: [f64; 3], rd: [f64; 3]) -> Option<Vector3<f64>> {
        g.intersects(&Ray { ro: Vector3::from(ro), rd: Vector3::from(rd) }).map(|i| i.point)
    }

    #[test]
    fn test_steps_apply_in_order() {
        let sphere = r#"{"type": "sphere", "location": [0, 0, 0], "radius": 1}"#;
        // Moved out along x, then turned 90 degrees about y: x goes to -z.
        let a = SceneFile::parse_geometry(&serde_json::from_str(&format!(
            r#"{{"type": "transform", "item": {}, "steps": [{{"translate": [5, 0, 0]}}, {{"rotate": [0, 90, 0]}}]}}"#, sphere)).unwrap(), &GeometryLibrary::empty()).unwrap();
        let p = hit(a.as_ref(), [0., 0., 10.], [0., 0., -1.]).unwrap();
        assert!((p.z + 4.).abs() < 1e-9, "{}", p);
        let c = (a.bounds().min.coords + a.bounds().max.coords) * 0.5;
        assert!((c - Vector3::new(0., 0., -5.)).norm() < 1e-9, "{}", c);
        // The other way round, it just moves along x.
        let b = SceneFile::parse_geometry(&serde_json::from_str(&format!(
            r#"{{"type": "transform", "item": {}, "steps": [{{"rotate": {{"pitch": 90}}}}, {{"translate": [5, 0, 0]}}]}}"#, sphere)).unwrap(), &GeometryLibrary::empty()).unwrap();
        let c = (b.bounds().min.coords + b.bounds().max.coords) * 0.5;
        assert!((c - Vector3::new(5., 0., 0.)).norm() < 1e-9, "{}", c);
    }

    #[test]
    fn test_transform_on_any_shape_moves_its_clipping_too() {
        // A bowl (sphere clipped to below y = 0), scaled up and turned upside down: a dome.
        let dome = SceneFile::parse_geometry(&serde_json::json!({
            "type": "sphere", "location": [0, 0, 0], "radius": 1,
            "clipped_by": { "type": "plane", "normal": [0, 1, 0], "distance": 0 },
            "transform": [ { "scale": 2 }, { "axis_rotate": { "axis": [1, 0, 0], "angle": 180 } } ]
        }), &GeometryLibrary::empty()).unwrap();
        let top = hit(dome.as_ref(), [0., 10., 0.], [0., -1., 0.]).unwrap();
        assert!((top.y - 2.).abs() < 1e-9, "{}", top);
        // Open underneath: up through the missing base to the inside of the dome.
        let under = hit(dome.as_ref(), [0., -10., 0.], [0., 1., 0.]).unwrap();
        assert!((under.y - 2.).abs() < 1e-9, "{}", under);
    }

    #[test]
    fn test_matrix_step_is_in_pov_order() {
        // POV-Ray's matrix <1,0,0, 0.5,1,0, 0,0,1, 3,0,0>: x' = x + 0.5 y + 3 (the second row is
        // where y goes), others unchanged.
        let m = SceneFile::parse_steps(&serde_json::json!([ { "matrix": [1, 0, 0, 0.5, 1, 0, 0, 0, 1, 3, 0, 0] } ]));
        let p = m.transform_point(&crate::na::Point3::new(1., 2., 3.));
        assert!((p.coords - Vector3::new(5., 2., 3.)).norm() < 1e-12, "{}", p);
    }

    #[test]
    #[should_panic(expected = "each transform step has one of")]
    fn test_step_with_two_keys_is_rejected() {
        SceneFile::parse_steps(&serde_json::json!([ { "translate": [1, 0, 0], "scale": 2 } ]));
    }

    #[test]
    fn test_tree_is_bark_and_leaves() {
        let tree = |extra: &str| SceneFile::from_string(format!(r#"{{
            "camera": {{ "location": [0, 1, -10], "lookat": [0, 1, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "B": {{ "type": "lambertian", "albedo": [0.3, 0.2, 0.1] }} }},
            "objects": [ {{ "type": "tree", "species": "shrub", "seed": 2, "location": [0, 0, 5], "bark": "B"{} }} ]
        }}"#, extra));
        let s = tree("");
        assert_eq!(s.objects.len(), 2);
        let b = s.objects.items()[0].bounds();
        assert!(b.min.z > 2. && b.max.z < 8., "{:?}", b);
        // Without leaves, only the bark.
        assert_eq!(tree(r#", "params": { "leaves": 0 }"#).objects.len(), 1);
    }

    #[test]
    fn test_tree_leaves_take_several_materials_and_media() {
        let materials = serde_json::json!({
            "A": { "type": "lambertian", "albedo": [0.1, 0.3, 0.1] },
            "B": { "type": "leaf", "reflectance": [0.1, 0.2, 0.05], "transmittance": [0.1, 0.2, 0.05] }
        });
        let media = serde_json::json!({ "C": { "type": "solid", "material": "A" } });
        let (materials, media) = (materials.as_object().unwrap(), media.as_object().unwrap());
        let tree = |leaves: Value| SceneFile::parse_tree(
            &serde_json::json!({ "type": "tree", "species": "shrub", "seed": 3, "bark": "C", "leaves": leaves }),
            materials, media);
        let one = tree(serde_json::json!("A"));
        let three = tree(serde_json::json!(["A", "B", "C"]));
        // Bark, and a leaf object per name.
        assert_eq!(one.len(), 2);
        assert_eq!(three.len(), 4);
        // The leaves are shared out whole, none lost.
        let count = |objects: &[Arc<SceneObject>]| objects[1..].iter().map(|o| o.geometry.primitives()).sum::<u64>();
        assert_eq!(count(&one), count(&three));
        for o in &three[1..] {
            assert!(o.geometry.primitives() > count(&one) / 6, "{}", o.geometry.primitives());
        }
    }

    #[test]
    #[should_panic(expected = "autumn reflectance + transmittance")]
    fn test_leaf_colours_must_be_physical() {
        SceneFile::parse_leaf(&serde_json::json!({ "type": "leaf",
            "autumn": { "reflectance": [0.6, 0.3, 0], "transmittance": [0.5, 0.3, 0] } }));
    }

    #[test]
    fn test_leaf_autumn_sun_defaults_to_autumn() {
        let leaf = SceneFile::parse_leaf(&serde_json::json!({ "type": "leaf",
            "autumn": { "reflectance": [0.4, 0.1, 0], "transmittance": [0.3, 0.05, 0] } }));
        assert_eq!(leaf.autumn_sun, leaf.autumn);
        assert_eq!(leaf.autumn.reflectance, Color::new(0.4, 0.1, 0.));
        assert_eq!(leaf.dead, DEFAULT_DEAD);
    }

    /// A colour texture can be a function graph: a mix of materials by slope.
    #[test]
    fn test_graph_as_texture() {
        let t = SceneFile::parse_color_texture(&serde_json::json!({"node": "slope", "from": 20, "to": 40}));
        assert!(matches!(t, crate::textures::ColorTexture::Graph(_)));
        assert!((t.average().rgb.x - 0.5).abs() < 1e-12);
    }

    /// A forest's "where" rule keeps its trees to where it says: east of x = 0 here.
    #[test]
    fn test_forest_grows_where_its_rule_says() {
        let objects = SceneFile::parse_forest(&serde_json::json!({ "type": "forest", "seed": 3, "spacing": 2,
            "area": {"min": [-20, -20], "max": [20, 20]}, "clearings": 0,
            "where": { "node": "smoothstep", "input": "x", "from": -0.01, "to": 0.01 },
            "species": [ { "species": "shrub" } ] }), &Map::new(), &Map::new(), &GeometryLibrary::empty(), None);
        let b = objects[0].geometry.bounds();
        assert!(b.min.x > -3., "{:?}", b);
        assert!(b.max.x > 15., "{:?}", b);
    }

    #[test]
    fn test_forest_instances_a_few_distinct_trees() {
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 1, -10], "lookat": [0, 1, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "B": { "type": "lambertian", "albedo": [0.3, 0.2, 0.1] },
                           "L": { "type": "leaf", "reflectance": [0.1, 0.2, 0.05], "transmittance": [0.1, 0.2, 0.05] } },
            "geometries": { "ground": { "type": "plane", "y": 2 } },
            "objects": [ { "type": "forest", "seed": 1, "surface": "ground", "spacing": 2, "variants": 2,
                "area": { "min": [-10, -10], "max": [10, 10] }, "autumn": 0.4,
                "species": [ { "species": "shrub", "bark": "B", "leaves": "L" },
                             { "species": "shrub", "weight": 2, "spacing": 0.7, "bark": "B", "leaves": ["L", "L"],
                               "params": { "leaves": 10 } } ] } ]
        }"#.to_string());
        // The whole forest is one object.
        let objects = s.objects.items();
        assert_eq!(objects.len(), 1);
        // All on the ground at y = 2 and within the area, give or take a crown.
        for o in objects {
            let b = o.geometry.bounds();
            assert!(b.min.y > 1.5 && b.max.y < 6., "{:?}", b);
            assert!(b.min.x > -13. && b.max.x < 13., "{:?}", b);
        }
    }

    #[test]
    fn test_forest_places_each_tree_once_per_part() {
        // One species, one variant: every shrub is a copy of one.
        let objects = SceneFile::parse_forest(&serde_json::json!({ "type": "forest", "seed": 2, "spacing": 2,
            "variants": 1, "clearings": 0, "area": { "min": [-10, -10], "max": [10, 10] },
            "species": [ { "species": "shrub" } ] }), &Map::new(), &Map::new(), &GeometryLibrary::empty(), None);
        assert_eq!(objects.len(), 1);
        let one = tree::generate(&tree::params("shrub", &Value::Null), 0, Vector3::zeros());
        // Between 20 and 120 shrubs' triangles, though only one shrub's are stored.
        let trees = objects[0].geometry.primitives() as f64 / (one.bark.len() + one.leaves.len()) as f64;
        assert!(trees > 20. && trees < 120., "{} shrubs", trees);
    }

    #[test]
    fn test_forest_lod_coarsens_distant_trees() {
        // The same shrubs, seen close up and from a kilometre away.
        let forest = |eye: [f64; 3], lod: bool| SceneFile::parse_forest(&serde_json::json!({ "type": "forest", "seed": 2,
            "spacing": 2, "variants": 1, "clearings": 0, "area": { "min": [-10, -10], "max": [10, 10] }, "lod": lod,
            "camera": { "location": eye, "angle": 0.5, "image_height": 200 },
            "species": [ { "species": "shrub" } ] }), &Map::new(), &Map::new(), &GeometryLibrary::empty(), None)[0].geometry.primitives();
        let full = forest([0., 1000., 0.], false);
        // Close up, nearby shrubs are seen in full.
        assert!(forest([0., 1., -11.], true) as f64 > 0.2 * full as f64);
        // Far away, each is a few cards.
        let far = forest([0., 1000., 0.], true);
        assert!(far * 100 < full, "{} of {}", far, full);
        assert!(far > 0);
    }

    #[test]
    #[should_panic(expected = "Unknown tree species")]
    fn test_unknown_tree_species_names_them() {
        SceneFile::parse_tree(&serde_json::json!({ "type": "tree", "species": "triffid" }), &Map::new(), &Map::new());
    }

    #[test]
    fn test_parts_of_combined_shapes_keep_their_materials() {
        // A white sphere with a bite cut by a glowing sphere: the cut face is the cutter's.
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 0, -10], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "L": { "type": "diffuse-light", "color": [1, 1, 1] },
                           "W": { "type": "lambertian", "albedo": [1, 1, 1] } },
            "objects": [
                { "type": "difference", "material": "W", "items": [
                    { "type": "sphere", "location": [0, 0, 0], "radius": 2 },
                    { "type": "sphere", "location": [2, 0, 0], "radius": 1, "material": "L" } ] }
            ]
        }"#.to_string());
        let emits = |ro: [f64; 3], rd: [f64; 3]| {
            let i = s.objects.nearest_intersection(&Ray { ro: Vector3::from(ro), rd: Vector3::from(rd) }, f64::INFINITY, 0.).unwrap();
            let m = &i.object.medium;
            m.part(i.part).unwrap_or(&**m).material_at(i.point).is_emitter()
        };
        assert!(!emits([0., 5., 0.], [0., -1., 0.]));
        assert!(emits([5., 0., 0.], [-1., 0., 0.]));
        // Lights inside a combination aren't sampled as area lights.
        assert!(s.area_lights.is_empty());
    }

    #[test]
    fn test_box_corners_in_either_order() {
        let b = SceneFile::parse_box(&serde_json::json!({ "min": [1, 1, 1], "max": [-1, -1, -1] }));
        let bounds = b.bounds();
        assert_eq!(bounds.min.coords, Vector3::new(-1., -1., -1.));
        assert_eq!(bounds.max.coords, Vector3::new(1., 1., 1.));
    }

    #[test]
    fn test_size_overrides() {
        let s = minimal_scene("");
        assert_eq!((s.image.width, s.image.height), (4, 2));
        let s = minimal_scene_sized("", Some(40), None);
        assert_eq!((s.image.width, s.image.height), (40, 2));
        let s = minimal_scene_sized("", None, Some(30));
        assert_eq!((s.image.width, s.image.height), (4, 30));
    }

    #[test]
    fn test_plane_normal_is_an_orientation_or_a_bump_map() {
        // A ground plane, bump mapped: its "normal" is an object (the bumps plinth of demo/scenes/stress-test.json).
        let bumped = minimal_scene(r#", "materials": { "M": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } },
            "objects": [ { "type": "plane", "y": 0, "material": "M",
                           "normal": { "pattern": "waves", "amount": 0.25 } } ]"#);
        assert_eq!(bumped.objects.infinite_items.len() + bumped.objects.items.len(), 1);
        // A half-space plane: its "normal" is a vector, and it isn't bump mapped.
        let oriented = minimal_scene(r#", "materials": { "M": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } },
            "objects": [ { "type": "plane", "normal": [0, 1, 0], "distance": 0, "material": "M" } ]"#);
        assert_eq!(oriented.objects.infinite_items.len() + oriented.objects.items.len(), 1);
    }

    #[test]
    fn test_black_threshold_independent_of_shadow_bias() {
        assert_eq!(minimal_scene("").black_threshold, 1e-7);
        assert_eq!(minimal_scene(r#", "black_threshold": 0.01"#).black_threshold, 0.01);
    }

    #[test]
    fn test_view_widens_for_defocus() {
        use crate::shapes::bbox::BBox;
        // Focused 1 m off with a 0.5 m lens: far beyond the focus, rays from the lens's edge
        // reach well outside the pinhole frustum (half-width 0.3 at unit depth), up to 0.8.
        let view = SceneFile::parse_view(&serde_json::json!({
            "location": [0, 0, 0], "lookat": [0, 0, 1], "up": [0, 1, 0], "angle": 0.5,
            "aperture": 0.5, "image_width": 100, "image_height": 100})).unwrap();
        let at = |x: f64| BBox::new(Vector3::new(x, -0.5, 100.), Vector3::new(x + 1., 0.5, 101.));
        assert!(!view.excludes(&at(60.)) && !view.excludes(&at(-61.)));
        assert!(view.excludes(&at(120.)) && view.excludes(&at(-121.)));
        // A pinhole sees no wider than its image.
        let pinhole = SceneFile::parse_view(&serde_json::json!({
            "location": [0, 0, 0], "lookat": [0, 0, 1], "up": [0, 1, 0], "angle": 0.5,
            "aperture": 0, "image_width": 100, "image_height": 100})).unwrap();
        assert!(pinhole.excludes(&at(60.)));
        assert!(SceneFile::parse_view(&serde_json::json!({"type": "orthographic",
            "image_width": 100, "image_height": 100})).is_none());
    }

    #[test]
    fn test_isometric_cameras_are_placed_orthographic_cameras() {
        let c = SceneFile::axonometric(serde_json::json!({"type": "isometric", "lookat": [1, 0, 1], "height": 4}));
        assert_eq!(c["type"], "orthographic");
        let back = SceneFile::parse_vec3(&c["location"]) - Vector3::new(1., 0., 1.);
        // Twice the height away, along (1, 1, 1).
        assert!((back - Vector3::new(8., 8., 8.) / 3f64.sqrt()).norm() < 1e-9, "{}", back);
        let cam = SceneFile::parse_camera(serde_json::json!({"type": "isometric", "lookat": [0, 0, 0],
            "location": [0, 0, 10], "azimuth": 225, "height": 2}), 100, 100);
        let r = cam.get_ray(0.5, 0.5, 0., 0.);
        // Only the location's distance counts: from azimuth 225°, it looks down along (1, -1, 1).
        assert!((r.ro.norm() - 10.).abs() < 1e-9 && (r.rd - Vector3::new(1., -1., 1.).normalize()).norm() < 1e-9, "{:?}", r);
        // Through a lens, focused at lookat.
        let cam = SceneFile::parse_camera(serde_json::json!({"type": "isometric", "lookat": [0, 0, 0],
            "height": 2, "distance": 10, "aperture": 0.5}), 100, 100);
        for _ in 0..16 {
            let r = cam.get_ray(0.5, 0.5, 0., 0.);
            let t = r.rd.dot(&(-r.ro));
            assert!((r.ro + r.rd * t).norm() < 1e-9, "{:?}", r);
        }
        // "blur" blurs alike from any distance: a point 1 beyond focus by a disc of radius 0.1.
        for distance in [5., 50.] {
            let cam = SceneFile::parse_camera(serde_json::json!({"type": "isometric", "lookat": [0, 0, 0],
                "height": 2, "distance": distance, "blur": 0.1, "elevation": 90}), 100, 100);
            let widest = (0..256).map(|_| {
                let r = cam.get_ray(0.5, 0.5, 0., 0.);
                let t = (-1. - r.ro.y) / r.rd.y;
                (r.ro + r.rd * t).xz().norm()
            }).fold(0f64, f64::max);
            assert!(widest > 0.08 && widest <= 0.1 + 1e-9, "{} at {}", widest, distance);
        }
        let other = serde_json::json!({"type": "orthographic", "location": [0, 0, 1], "lookat": [0, 0, 0], "up": [0, 1, 0]});
        assert_eq!(SceneFile::axonometric(other.clone()), other);
    }

    #[test]
    fn test_displaced_terrains_learn_where_they_are_put() {
        let terrain = serde_json::json!({ "type": "terrain", "heightmap": {}, "displacement": 0 });
        let mut geometries = serde_json::json!({
            "cliff": terrain,
            "pair": { "type": "union", "items": ["cliff", { "type": "transform", "item": "cliff", "translate": [0, 0, 50] }] },
        }).as_object().unwrap().clone();
        let mut objects = vec![
            serde_json::json!({ "type": "transform", "item": "pair", "translate": [100, 0, 0] }),
            serde_json::json!({ "type": "transform", "item": "cliff", "scale": 2 }),
            serde_json::json!({ "type": "terrain", "heightmap": {}, "displacement": 0, "transform": [{ "translate": [0, 5, 0] }] }),
            terrain.clone(),
        ];
        SceneFile::displaced_instances(&mut objects, &mut geometries);
        let placed = |v: &Value| v["displacement_instances"].as_array().map(|a| a.iter()
            .map(|m| [m[3].as_f64().unwrap(), m[7].as_f64().unwrap(), m[11].as_f64().unwrap(), m[0].as_f64().unwrap()]).collect::<Vec<_>>());
        // The named cliff: twice in the pair (moved 100 east, once 50 south besides), and
        // scaled by 2 where it stands.
        let mut cliff = placed(&geometries["cliff"]).unwrap();
        cliff.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(cliff, vec![[0., 0., 0., 2.], [100., 0., 0., 1.], [100., 0., 50., 1.]]);
        // An inline terrain with steps of its own; one nowhere moved is left alone.
        assert_eq!(placed(&objects[2]).unwrap(), vec![[0., 5., 0., 1.]]);
        assert!(placed(&objects[3]).is_none());
    }

    #[test]
    fn test_scatter_strews_where_density_lets_it() {
        let o = serde_json::json!({ "type": "scatter", "seed": 4, "area": {"min": [-10, -10], "max": [10, 10]}, "spacing": 1.5,
            "density": { "node": "smoothstep", "input": "x", "from": -0.01, "to": 0.01 },
            "items": [ { "boulder": { "angularity": 0.6, "detail": 2 }, "variants": 2, "size": [0.2, 0.6] } ] });
        let objects = SceneFile::parse_scatter(&o, &Map::new(), &Map::new(), &GeometryLibrary::empty(), None);
        assert_eq!(objects.len(), 1);
        let g = &objects[0].geometry;
        let b = g.bounds();
        // Only east of x = 0, resting on the ground, no larger than the largest.
        assert!(b.min.x > -0.7, "{:?}", b);
        assert!(b.min.y > -0.7 && b.max.y < 1.3, "{:?}", b);
        // Plenty of them: rays straight down over the east half often meet one, never the west.
        let hits = |x0: f64| (0..400).filter(|k| {
            let (x, z) = (x0 + (k % 20) as f64 * 0.45, -9. + (k / 20) as f64 * 0.9);
            g.intersects(&Ray { ro: Vector3::new(x, 5., z), rd: -Vector3::y() }).is_some()
        }).count();
        assert!(hits(1.) > 40, "{}", hits(1.));
        assert_eq!(hits(-10.), 0);
    }

    #[test]
    fn test_volumes_take_a_density_graph() {
        let o = serde_json::json!({ "type": "volume", "min": [-1, -1, -1], "max": [1, 1, 1],
            "density": { "node": "multiply", "inputs": [3, { "node": "altitude", "from": 0, "to": -0.5 }] } });
        let v = SceneFile::parse_volume(&o, &GeometryLibrary::empty());
        // Thick low down, clear high up: transmittance along level rays, averaged.
        let through = |y: f64| (0..400).map(|_| v.geometry.transmittance(&Ray { ro: Vector3::new(-2., y, 0.), rd: Vector3::x() }, 0., 4.)).sum::<f64>() / 400.;
        crate::geometry::seed_rand(3, 1);
        assert!(through(-0.8) < 0.01, "{}", through(-0.8));
        assert!(through(0.5) > 0.99, "{}", through(0.5));
    }

    #[test]
    #[should_panic(expected = "bounded above")]
    fn test_volume_density_graphs_must_be_bounded() {
        let o = serde_json::json!({ "type": "volume", "min": [-1, -1, -1], "max": [1, 1, 1], "density": "y" });
        SceneFile::parse_volume(&o, &GeometryLibrary::empty());
    }

    /// In an ecosystem, what comes later grows in the room what came before leaves it.
    #[test]
    fn test_ecosystem_populations_compete_for_room() {
        let boulders = serde_json::json!({ "scatter": { "spacing": 0.8,
            "items": [ { "boulder": { "detail": 1 }, "variants": 1, "size": 0.15 } ] } });
        let shrubs = serde_json::json!({ "forest": { "spacing": 4, "clearings": 0, "shade": 1, "canopy": 1.3,
            "species": [ { "species": "shrub" } ] } });
        let ecosystem = |populations: Vec<Value>| SceneFile::parse_ecosystem(&serde_json::json!({ "type": "ecosystem", "seed": 5,
            "area": {"min": [-12, -12], "max": [12, 12]}, "populations": populations }), &Map::new(), &Map::new(), &GeometryLibrary::empty(), None);
        let alone = ecosystem(vec![boulders.clone()]);
        let under = ecosystem(vec![shrubs, boulders]);
        assert_eq!(under.len(), 2);
        let (a, b) = (alone[0].geometry.primitives(), under[1].geometry.primitives());
        assert!(4 * b < 3 * a, "{} boulders' worth under shrubs against {} alone", b, a);
        assert!(b > 0);
    }
}
