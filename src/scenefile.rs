use crate::camera;
use crate::na::{Vector3, Vector2};
use crate::scenegraph::SceneGraph;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::plane::Plane;
use crate::shapes::mesh::{Mesh, SmoothMesh};
use crate::shapes::csg::{Csg, Op, Inverse, Clipped, Tagged as PartTagged};
use crate::shapes::transform::{Transform};
use crate::na::Rotation3;
use crate::ocean::create_ocean;
use crate::shapes::bbox::BBox;
use crate::light::Light;
use crate::color::Color;
use crate::skysphere::create_sky_sphere;
use crate::procedural::box_terrain::create_box_terrain;
use crate::procedural::fireworks::create_firework;
use crate::procedural::tree;
use crate::terrain::object::create_terrain;
use crate::procedural::grass::{create_grass, GrassMedium};
use crate::procedural::heightmap::{HeightGrid, Terrain};
use crate::procedural::rock::{self, Bedrock, Assorted};
use crate::material::rock::{Rock, RockStyle};
use crate::participatingmedia::create_fog;
use std::sync::Arc;
use crate::sceneobject::SceneObject;
use serde_json::{Value, Map};
use crate::scene::{Scene, ImageOpts, RenderOpts, PathTermination, Background, Filter};
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

    pub fn parse_camera(c: Value, width: u32, height: u32) -> camera::FlatLensCamera {
        camera::FlatLensCamera::new(
            SceneFile::parse_vec3(&c["lookat"]),
            SceneFile::parse_vec3(&c["location"]),
            SceneFile::parse_vec3(&c["up"]),
            c["angle"].as_f64().unwrap(),
            width, height,
            SceneFile::parse_number(&c["aperture"], 0.2)
        )
    }

    pub fn parse_objects(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>, geometries: &GeometryLibrary) ->Vec<Arc<SceneObject>> {
         let mut objects: Vec<Arc<SceneObject>> = Vec::new();
         for obj in objs {
            if obj["type"] == "tree" {
                objects.extend(SceneFile::parse_tree(&obj, materials, media));
                continue;
            }
            if obj["type"] == "forest" {
                objects.extend(SceneFile::parse_forest(&obj, materials, media, geometries));
                continue;
            }
            if let Some(x) = SceneFile::parse_object(obj, materials, media, geometries) { objects.push(x) }
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
    pub fn parse_transform(o: &Value, parts: &mut Parts) -> Box<dyn Geometry + Sync + Send> {
        let item = SceneFile::parse_shared_geometry_in(&o["item"], parts)
            .unwrap_or_else(|| panic!("transform needs a geometry \"item\", got {}", o["item"]));
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
        Box::new(Transform::from_parts(item, translate, rotation, scale))
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
                refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.),
                attenuate:SceneFile::parse_color(&o["attenuate"]), 
            };
            return Some(Box::new(d));
        }

        if t == "diffuse-light" {
            let d:DiffuseLight = DiffuseLight {
                intensity: SceneFile::parse_number(&o["intensity"], 1.),
                color:SceneFile::parse_color(&o["color"]), 
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
        
        match noise_type {
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
        }
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
                color: SceneFile::parse_color_def(o, "color", Color::white()),
                intensity: o["intensity"].as_f64().unwrap(),
            }
    }

    pub fn parse_lights(lights: &Vec<Value>) -> Vec<Light> {
        let mut l: Vec<Light> = Vec::new();
        for light in lights {
            l.push(SceneFile::parse_light(light));
        }
        l
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

        let geometries = GeometryLibrary::new(s.geometries);
        geometries.preload(&s.objects);
        let objects = SceneFile::parse_objects(s.objects, &s.materials, &s.media, &geometries);
        // Emitters with a surface we can sample become area lights.
        let area_lights: Vec<Arc<SceneObject>> = objects.iter()
            .filter(|o| o.medium.material_at(Vector3::zeros()).is_emitter() && o.geometry.surface_area() > 0.)
            .cloned()
            .collect();
        let o = SceneGraph::new(objects);
		
        let width = SceneFile::parse_int(&s.width, 640);
        let height =SceneFile::parse_int(&s.height, 480);

        Scene {
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
                filter: match s.filter.as_str() {
                    None | Some("box") => Filter::Box,
                    Some("tent") => Filter::Tent,
                    Some(f) => panic!("Unknown filter '{}': expected \"box\" or \"tent\"", f),
                },
            },
            camera: Box::new(SceneFile::parse_camera(s.camera, width as u32, height as u32)),
            lights: SceneFile::parse_lights(&s.lights),
            objects: o,
            max_bounding,
            black_threshold: SceneFile::parse_number(&s.black_threshold, 1e-7f64),
            air_medium: SceneFile::parse_air(&s.air),
            output: OutputOpts::parse(&s.output),
            area_lights,
        }
    }

    pub fn from_string(s: String) -> Scene {
        SceneFile::from_string_overridden(s, &Overrides::default())
    }

    /// Parse a scene, optionally overriding the image size. Overrides must be applied before
    /// the camera is built, as it derives its aspect ratio from the image size.
    pub fn from_string_sized(s: String, width: Option<usize>, height: Option<usize>) -> Scene {
        SceneFile::from_string_overridden(s, &Overrides { width, height, seed: None })
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
        assert_eq!(s.lights[0].color, Color::red());
        assert_eq!(s.lights[1].color, Color::white());
        assert_eq!(s.lights[2].color, Color::white());
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
