use crate::camera;
use crate::na::{Vector3, Vector2};
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
use crate::procedural::nubis_clouds::{NubisCloudLayer, GlobalNubis};
use crate::procedural::hogan_kew_cirrus::{HoganKewCirrus, CirrusField, CirrusGenus, GlobalCirrus};
use crate::procedural::benard_altocumulus::BenardAltocumulus;
use crate::procedural::lenticular::{Lenticular, MoistLayer};
use crate::procedural::terrain::{Terrain as Mountains, TerrainMaterial};
use crate::procedural::mammatus::Mammatus;
use crate::procedural::precipitation::Precipitation;
use crate::material::rainbow::RainbowTable;
use crate::procedural::harris_cumulus::{CloudSimulation, Forcing, HarrisCumulus, Profile, SimulationParams, Sounding};
use crate::participatingmedia::create_fog;
use crate::volume::{create_volume, DensityField, ConstantDensity, NoiseDensity, ScatteringApproximation, Oz, Diffraction, Halos, Scattering};
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


    pub fn parse_color_def(v: &Value, k: &str, def: Color) -> Color {
        match &v.get(k) {
            Some(x) => SceneFile::parse_color(x),
            None => def
        }
    }

    /// The camera (`camera`): `"location"`, `"lookat"` and `"up"`, and by `"type"`:
    /// - `"perspective"` (the default): `"angle"`, with tan(angle) the image's height at unit
    ///   distance; a thin lens of radius `"aperture"` (default 0.2), focused at the point looked
    ///   at or `"focus_distance"`.
    /// - `"orthographic"`: parallel rays from a rectangle `"height"` tall (by default what the
    ///   perspective camera sees at the point looked at).
    /// - `"spherical"`: every direction, `"mapping": "equirectangular"` (the default) or
    ///   `"equal_area"`.
    ///
    /// With `"motion": {"location", "lookat", "up"}`, the camera moves to that pose (each key
    /// defaulting to the start's) while the shutter is open.
    pub fn parse_camera(c: Value, width: u32, height: u32) -> Box<dyn camera::Camera + Sync> {
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
                Box::new(cam)
            },
            Some("orthographic") => {
                let h = c["height"].as_f64().unwrap_or_else(|| angle().tan() * (lookat - location).norm());
                Box::new(camera::OrthographicCamera::new(placement, h, aspect))
            },
            Some("spherical") => Box::new(camera::SphericalCamera::new(placement, match c["mapping"].as_str() {
                None | Some("equirectangular") => camera::SphereMapping::Equirectangular,
                Some("equal_area") => camera::SphereMapping::EqualArea,
                Some(m) => panic!("Unknown spherical mapping '{}': expected \"equirectangular\" or \"equal_area\"", m),
            })),
            Some(t) => panic!("Unknown camera type '{}': expected \"perspective\", \"orthographic\" or \"spherical\"", t),
        }
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
         let mut objects: Vec<Arc<SceneObject>> = Vec::new();
         for obj in objs {
            // Report the objects that are slow to build (meshes, oceans), by type and source.
            let start = std::time::Instant::now();
            let name = format!("{}{}", obj["type"].as_str().unwrap_or("?"),
                obj["src"].as_str().or(obj["item"]["src"].as_str()).map_or(String::new(), |s| format!(" {}", s)));
            if obj["type"] == "tree" {
                objects.extend(SceneFile::parse_tree(&obj, materials, media));
            } else if obj["type"] == "forest" {
                objects.extend(SceneFile::parse_forest(&obj, materials, media, geometries));
            } else if let Some(x) = SceneFile::parse_object(obj, materials, media, geometries) {
                objects.push(x)
            }
            let t = start.elapsed().as_secs_f64();
            if t >= 0.1 {
                println!("- Built {} in {:.2} s", name, t);
            }
         }
         objects
    }

    /// An object's medium, bump mapped if it has a `"normal"` (see `parse_bump`).
    pub fn parse_object_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<dyn Medium + Sync + Send> {
        let m = SceneFile::parse_unbumped_medium(o, materials, media);
        if o["normal"].is_null() {
            return m;
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
                    let default_material = Box::new(Lambertian { albedo: Color::white() });
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
            return Some(Arc::new(create_terrain(&o)));
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
            return Some(Arc::new(SceneFile::parse_volume(&o)));
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
    /// standing on the highest; default the plane y = 0), placed by `crate::procedural::forest`.
    /// Each species grows `variants` distinct trees, each placed many times by instancing
    /// transforms, so memory goes with the distinct trees, not the forest's size. The top-level
    /// `params` apply to every species, then each species' own. Given `autumn`, each distinct
    /// tree is that far through autumn, give or take `autumn_spread`. The surface itself isn't
    /// drawn: add it as an object too.
    pub fn parse_forest(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value>,
                        geometries: &GeometryLibrary) -> Vec<Arc<SceneObject>> {
        use crate::procedural::forest::{self, ForestLayout, ForestSpecies};
        let surface = |g: &Value| SceneFile::parse_shared_geometry(g, geometries)
            .unwrap_or_else(|| panic!("forest surface {} is not a geometry", g));
        let surfaces: Vec<SharedGeometry> = match &o["surface"] {
            Value::Null => vec![Arc::new(Plane { y: 0. })],
            Value::Array(list) => list.iter().map(surface).collect(),
            g => vec![surface(g)],
        };
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
        let refs: Vec<&dyn Geometry> = surfaces.iter().map(|s| &**s as &dyn Geometry).collect();
        let placements = forest::place(&layout, &species, seed, &refs);

        let spread = SceneFile::parse_number(&o["autumn_spread"], 0.1);
        let mut rng = crate::geometry::Pcg32::new(seed, 0x7a71);
        // Every part of every placed tree, in one structure, each part numbered for its medium.
        let mut instances: Vec<Arc<forest::PartOf<Transform>>> = vec![];
        let mut parts: Vec<Box<dyn Medium + Sync + Send>> = vec![];
        let mut distinct = 0;
        for (k, entry) in entries.iter().enumerate() {
            let name = entry.get("species").map(SceneFile::parse_string)
                .unwrap_or_else(|| panic!("forest species {} needs a \"species\"", entry));
            for v in 0 .. layout.variants {
                // Draw for every variant, placed or not, so each keeps its own numbers.
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
                let p = tree::params(&name, &overrides);
                distinct += 1;

                // On a slope, sink the trunk so its downhill side meets the ground.
                let base_radius = p.scale * p.stems[0].length * p.ratio * p.trunk_scale * (1. + 0.99 * p.flare);
                let places: Vec<(Vector3<f64>, Rotation3<f64>, Vector3<f64>)> = here.iter().map(|t| {
                    let sink = base_radius * t.scale * (t.slope.to_radians().tan() + 0.05);
                    let mirror = if t.mirror { -1. } else { 1. };
                    (t.position - Vector3::new(0., sink, 0.),
                     Rotation3::from_axis_angle(&Vector3::y_axis(), t.yaw.to_radians()),
                     Vector3::new(mirror * t.scale, t.scale, t.scale))
                }).collect();
                for (geometry, medium) in SceneFile::tree_parts(entry, &p, tree_seed, Vector3::zeros(), materials, media) {
                    let part = parts.len() as u32;
                    assert!(part < 1 << (32 - forest::PART_SHIFT), "too many distinct tree parts in a forest");
                    instances.extend(places.iter().map(|(at, turn, scale)| Arc::new(forest::PartOf {
                        item: Transform::from_parts(geometry.clone(), *at, *turn, *scale), part,
                    })));
                    parts.push(medium);
                }
            }
        }
        println!("Forest: {} trees, from {} distinct", placements.len(), distinct);
        if instances.is_empty() {
            return vec![];
        }
        vec![Arc::new(SceneObject {
            geometry: Box::new(GeometrySet::new(&instances)),
            medium: Box::new(Solid { m: Box::new(forest::PartsMaterial { parts }) }),
        })]
    }

    /// The parts of a tree with parameters `p`, chosen by `seed`, grown at `location`, each a
    /// geometry and the medium it's made of: its bark, its dead branches' bark, and its leaves in
    /// as many parts as `o` names leaf materials. `o` names the materials as for `parse_tree`.
    pub fn tree_parts(o: &Value, p: &tree::TreeParams, seed: u64, location: Vector3<f64>,
                      materials: &Map<String, Value>, media: &Map<String, Value>)
                      -> Vec<(SharedGeometry, Box<dyn Medium + Sync + Send>)> {
        let mesh = tree::generate(p, seed, location);
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
            Box::new(Solid { m: Box::new(Lambertian { albedo }) })
        };
        let bark = || o.get("bark").map_or_else(|| default(Color::new(0.25, 0.2, 0.16)), medium);

        let mut parts: Vec<(SharedGeometry, Box<dyn Medium + Sync + Send>)> = vec![];
        if !mesh.bark.is_empty() {
            parts.push((Arc::new(GeometrySet::new(&mesh.bark)), bark()));
        }
        if !mesh.dead_bark.is_empty() {
            parts.push((Arc::new(GeometrySet::new(&mesh.dead_bark)), o.get("dead_bark").map_or_else(bark, medium)));
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
                parts.push((Arc::new(GeometrySet::new(&triangles)), medium));
            }
        }
        parts
    }

    pub fn parse_skysphere(o: &Value) -> SceneObject {
        create_sky_sphere(o)
    }

    /// A participating medium filling the box from `min` to `max`:
    ///
    /// `{"type": "volume", "min": [x, y, z], "max": [x, y, z], "density": σ, "albedo": colour,
    /// "g": asymmetry, "noise": {"scale", "octaves", "threshold", "seed"}}`
    ///
    /// `density` is the extinction coefficient, the chance of a collision per unit distance
    /// (default 1). Without `noise` it is constant; with it, it varies up to that maximum.
    /// `albedo` (default white) is the fraction scattered rather than absorbed at a collision.
    /// How it scatters is given by `g` (Henyey-Greenstein, -1 to 1, default 0) or by
    /// `droplet_diameter` (in µm, 5 to 50: the Mie scattering of water droplets of that size;
    /// see `PhaseFunction::water_droplets`).
    pub fn parse_volume(o: &Value) -> SceneObject {
        let bounds = BBox::new(SceneFile::parse_vec3(&o["min"]), SceneFile::parse_vec3(&o["max"]));
        let max_density = SceneFile::parse_number(&o["density"], 1.);
        let density: Arc<dyn DensityField> = match o.get("noise") {
            None => Arc::new(ConstantDensity(max_density)),
            Some(n) => Arc::new(NoiseDensity {
                perlin: SceneFile::parse_seed(n).map_or_else(PerlinNoise::new, PerlinNoise::with_seed),
                max_density,
                scale: SceneFile::parse_number(&n["scale"], 1.),
                octaves: SceneFile::parse_int(&n["octaves"], 4) as u32,
                threshold: SceneFile::parse_number(&n["threshold"], 0.5),
            }),
        };
        create_volume(bounds, density, SceneFile::parse_scattering(o, 0.))
    }

    /// A medium's multiple-scattering approximations (`volume::ScatteringApproximation`):
    /// `"similarity_after": N` for the similarity relations from the N-th scattering on, and
    /// `"oz": true` or `{"octaves", "a", "b", "c"}` for Wrenninge et al.'s octaves.
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
        }
    }

    /// How a volume scatters: `albedo`, its phase function (`g` defaulting to `default_g`, or
    /// `droplet_diameter`), approximations, `diffraction`, `halos`, and `"rainbow": true` for
    /// raindrops' bows in direct sunlight.
    pub fn parse_scattering(o: &Value, default_g: f64) -> Scattering {
        Scattering {
            albedo: SceneFile::parse_color_def(o, "albedo", Color::white()),
            phase: SceneFile::parse_phase(o, default_g),
            approximation: SceneFile::parse_scattering_approximation(o),
            diffraction: SceneFile::parse_diffraction(o),
            halos: SceneFile::parse_halos(o),
            rainbow: (o["rainbow"].as_bool() == Some(true)).then(|| Arc::new(RainbowTable::new(100_000))),
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
    /// ground; `vorticity` (confinement); `max_step` (s); `seed`.
    ///
    /// `state` renders a state saved by `CloudSimulation::save` instead of simulating.
    /// `follow_wind` renders the clouds as seen drifting with the wind, so in an animation
    /// they stay in view as they grow and dissipate, rather than crossing the sky.
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
        if o["sounding"].as_str() == Some("arm") {
            // The full day, from dawn.
            p.sounding = Sounding::arm();
            p.start = num("start", 0.);
        } else if let Some(name) = o["sounding"].as_str() {
            assert!(name == "arm_mid_morning", "harris_cumulus: unknown sounding '{}': expected arm, arm_mid_morning or a profile", name);
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
        let mut c = HarrisCumulus::new(&sim, num("droplet_radius", 10.), center);
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
    /// `particle_radius` (µm, for `diffraction`). `mapping` as for `nubis_clouds` (`"regional"` or `"flat"`).
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
    /// `"rainbow": true` gives the drops' bows in sunlight.
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

    /// Mountains (see `procedural::terrain`): a meshed heightfield of ridged noise around a
    /// massif, lying on the planet, with snow, rock and vegetation:
    ///
    /// `{"type": "mountains", "center": [x, z], "extent": 30000, "resolution": 768,
    /// "peak": [x, z], "peak_height": 3200, "peak_radius": 7000, "seed": 0}`
    ///
    /// with `relief` and `feature_size` (the ridges), `plains` (relief of the plains around),
    /// `base` (the height of its foot: below zero for an island in a `sea`),
    /// `snowline` and `treeline` (m), the colours `snow`, `rock`, `forest` and `grass`, and
    /// `"mapping": "flat"` for flat ground rather than the planet's.
    pub fn parse_terrain(o: &Value) -> SceneObject {
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
        let radius = if flat { None } else { Some(SceneFile::parse_number(&o["earth_radius"], EARTH_RADIUS)) };
        t.planet_radius = radius;
        let mut m = TerrainMaterial::new(seed);
        m.planet_radius = radius;
        m.snowline = num("snowline", m.snowline);
        m.treeline = num("treeline", m.treeline);
        m.snow = SceneFile::parse_color_def(o, "snow", m.snow);
        m.rock = SceneFile::parse_color_def(o, "rock", m.rock);
        m.forest = SceneFile::parse_color_def(o, "forest", m.forest);
        m.grass = SceneFile::parse_color_def(o, "grass", m.grass);
        SceneObject {
            geometry: Box::new(SmoothMesh::from_indexed(&[t.mesh()])),
            medium: Box::new(Solid { m: Box::new(m) }),
        }
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
    pub fn parse_hogan_kew_cirrus(o: &Value) -> SceneObject {
        let genus = match o["genus"].as_str() {
            None | Some("cirrus") => CirrusGenus::Cirrus,
            Some("cirrostratus") => CirrusGenus::Cirrostratus,
            Some("cirrocumulus") => CirrusGenus::Cirrocumulus,
            Some(g) => panic!("Unknown cirrus genus '{}': expected cirrus, cirrostratus or cirrocumulus", g),
        };
        let mut f = CirrusField::new(genus);
        let num = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        f.base = num("base", f.base);
        f.top = num("top", f.top);
        f.coverage = num("coverage", f.coverage).clamp(0., 1.);
        f.cell_size = num("cell_size", f.cell_size);
        f.spectral_slope = num("spectral_slope", f.spectral_slope);
        f.anisotropy = num("anisotropy", f.anisotropy);
        f.wind_direction = SceneFile::parse_vec2_def(o, "wind_direction", f.wind_direction);
        f.seed = SceneFile::parse_seed(o).unwrap_or(0);
        let (base, top) = (f.base, f.top);
        let generating = (f.cell_size, f.spectral_slope, f.coverage, f.seed);
        let mut c = HoganKewCirrus::new(f);
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
            return Some(Box::new(Terrain { grid: Arc::new(HeightGrid::from_json(&o["heightmap"])) }));
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
    /// "distance": d}`, POV-Ray's plane: the half-space n.p < d, which can be used in CSG.
    pub fn parse_plane(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        if !o["normal"].is_null() {
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

        if t == "lambertian" {
            let d:Lambertian = Lambertian {
                albedo:SceneFile::parse_color(&o["albedo"]), 
            };
            return Some(Box::new(d));
        }

        if t == "plastic" {
            if !o["opacity"].is_null() {
                println!("Warning: plastic 'opacity' is ignored; plastic is opaque (use a dielectric for transparency)");
            }
            let d:Plastic = Plastic {
                albedo:SceneFile::parse_color(&o["albedo"]), 
                refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.5),
                roughness: SceneFile::parse_number(&o["roughness"], 0.),
            };
            return Some(Box::new(d));
        }

        if t == "leaf" {
            return Some(Box::new(SceneFile::parse_leaf(o)));
        }

        if t == "dielectric" {
            let d:Dielectric = Dielectric {
                refractive_index: SceneFile::parse_refractive_index(&o["refractive_index"]),
                attenuate: SceneFile::parse_color_def(o, "attenuate", Color::white()),
            };
            return Some(Box::new(d));
        }

        if t == "diffuse-light" {
            let d:DiffuseLight = DiffuseLight {
                intensity: SceneFile::parse_number(&o["intensity"], 1.),
                emission: SceneFile::parse_emission(o),
                two_sided: o["two_sided"].as_bool().unwrap_or(false),
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

        if t == "rock" {
            return Some(Box::new(SceneFile::parse_rock(o)));
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

    /// `{"type": "rock", "preset": "granite"}`: a procedural stone (see `material::rock`).
    /// Presets are granite, basalt, sandstone, limestone, slate and quartzite; any field of the
    /// preset can be overridden, eg. `"color"` or `"strata_amount"`. Also `"scale"` (the size of
    /// the texture's features), `"seed"`, `"weathering"`, `"wetness"` (0 to 1), `"lichen"` (the
    /// fraction of upward surfaces covered, 0 to 1) and `"lichen_color"`.
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
        let lichen = n("lichen", 0.);
        if lichen > 0. {
            let color = c("lichen_color", r.lichen_color);
            r = r.with_lichen(lichen, color);
        }
        r
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
                    Box::new(Lambertian { albedo: Color::white() })
                })
        } else {
            // Default to white lambertian if no base material is specified
            Box::new(Lambertian { albedo: Color::white() })
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


    pub fn parse_light(o: &Value) -> Light {
        Light {
                position: SceneFile::parse_vec3(&o["location"]),
                emission: SceneFile::parse_emission(o),
                intensity: o["intensity"].as_f64().unwrap(),
            }
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
    /// is its irradiance above the atmosphere, and it is reddened by the sky's atmosphere on
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

    /// A colour (black if absent), or `{"type": "gradient", "bottom": colour, "top": colour}`.
    pub fn parse_background(v: &Value) -> Background {
        match v {
            Value::Null => Background::Solid(Color::black()),
            Value::Object(_) => match v["type"].as_str() {
                Some("gradient") => Background::Gradient {
                    bottom: SceneFile::parse_color_def(v, "bottom", Color::white()),
                    top: SceneFile::parse_color_def(v, "top", Color::new(0.5, 0.7, 1.)),
                },
                t => panic!("Unknown background type {:?}: expected \"gradient\" or a colour", t),
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

    pub fn from_scenefile(s: SceneFile) -> Scene {
        let max_bounding = BBox::new( // TODO
            Vector3::new(-1000., -1000., -1000.),
            Vector3::new(1000., 1000., 1000.),
        );

        let sun = SceneFile::parse_sun(&s.sun, &s.objects);
        // The camera's view plane spans tan(angle) over the image's height.
        let pixel_angle = s.camera["angle"].as_f64().map_or(0., |a| a.tan() / SceneFile::parse_int(&s.height, 480) as f64);
        // Aerial perspective, unless the skysphere turns it off.
        let atmosphere = s.objects.iter()
            .find(|o| o["type"].as_str() == Some("skysphere") && o["aerial_perspective"].as_bool() != Some(false))
            .map(|o| Arc::new(parse_sky_material(o)));
        // Motion blur: sample times only if something moves (`motion`).
        let motion = SceneFile::has_motion(&s.camera) || s.objects.iter().any(SceneFile::has_motion)
            || s.geometries.values().any(SceneFile::has_motion);
        let geometries = GeometryLibrary::new(s.geometries);
        geometries.preload(&s.objects);
        let objects = SceneFile::parse_objects(s.objects, &s.materials, &s.media, &geometries);
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
                background: SceneFile::parse_background(&s.background),
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
                sampler: match s.sampler.as_str() {
                    None | Some("sobol") => Sampler::Sobol,
                    Some("independent") => Sampler::Independent,
                    Some(x) => panic!("Unknown sampler '{}': expected \"sobol\" or \"independent\"", x),
                },
            },
            camera: SceneFile::parse_camera(s.camera, width as u32, height as u32),
            lights: SceneFile::parse_lights(&s.lights),
            objects: o,
            max_bounding,
            black_threshold: SceneFile::parse_number(&s.black_threshold, 1e-7f64),
            air_medium: SceneFile::parse_air(&s.air),
            output: OutputOpts {
                // The camera's view plane spans tan(angle) over the image's height.
                pixel_angle,
                ..OutputOpts::parse(&s.output)
            },
            area_lights,
            area_light_choice,
            sun,
            atmosphere,
            environment: None,
        };
        // Tabulating the sky evaluates it, which needs the scene.
        if scene.render.sample_sky {
            scene.environment = crate::environment::Environment::build(&scene);
        }
        scene
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
        let doc = SceneFile::substitute_variables(doc)
            .unwrap_or_else(|e| panic!("Scene file: {}", e));
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
        let contents = std::fs::read_to_string(filename)
            .map_err(|e| format!("Can't read scene file {}: {}", filename, e))?;
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
        let g = SceneFile::parse_background(&serde_json::json!({ "type": "gradient" }));
        // The book's colours by default: white looking down, (0.5, 0.7, 1) looking up.
        assert_eq!(g.radiance(&Vector3::new(0., -1., 0.)), Color::white());
        assert_eq!(g.radiance(&Vector3::new(0., 2., 0.)), Color::new(0.5, 0.7, 1.));
        assert_eq!(g.radiance(&Vector3::new(1., 0., 0.)), Color::new(0.75, 0.85, 1.));
        assert_eq!(SceneFile::parse_background(&serde_json::json!([0.1, 0.2, 0.3])), Background::Solid(Color::new(0.1, 0.2, 0.3)));
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
            "species": [ { "species": "shrub" } ] }), &Map::new(), &Map::new(), &GeometryLibrary::empty());
        assert_eq!(objects.len(), 1);
        let one = tree::generate(&tree::params("shrub", &Value::Null), 0, Vector3::zeros());
        // Between 20 and 120 shrubs' triangles, though only one shrub's are stored.
        let trees = objects[0].geometry.primitives() as f64 / (one.bark.len() + one.leaves.len()) as f64;
        assert!(trees > 20. && trees < 120., "{} shrubs", trees);
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
    fn test_black_threshold_independent_of_shadow_bias() {
        assert_eq!(minimal_scene("").black_threshold, 1e-7);
        assert_eq!(minimal_scene(r#", "black_threshold": 0.01"#).black_threshold, 0.01);
    }
}
