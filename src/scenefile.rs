use crate::camera;
use crate::na::{Vector3, Vector2};
use crate::scenegraph::SceneGraph;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::plane::Plane;
use crate::shapes::mesh::{Mesh, SmoothMesh};
use crate::shapes::csg::{Primitive, Difference};
use crate::shapes::transform::{Transform};
use crate::na::Rotation3;
use crate::ocean::create_ocean;
use crate::shapes::bbox::BBox;
use crate::light::{Light, Sun, ALTITUDE_STEP, ALTITUDE_STEPS};
use crate::color::Color;
use crate::skysphere::{create_sky_sphere, parse_sky_material};
use crate::procedural::box_terrain::create_box_terrain;
use crate::procedural::fireworks::create_firework;
use crate::procedural::nubis_clouds::NubisCloudLayer;
use crate::procedural::hogan_kew_cirrus::{HoganKewCirrus, CirrusField, CirrusGenus};
use crate::participatingmedia::create_fog;
use crate::volume::{create_volume, DensityField, ConstantDensity, NoiseDensity, ScatteringApproximation, Oz, Diffraction, Halos, Scattering};
use crate::material::ice_halos::{Crystals, Habit};
use crate::material::diffraction::DiffractionTable;
use crate::noise::PerlinNoise;
use crate::material::phase::PhaseFunction;
use std::sync::Arc;
use crate::sceneobject::SceneObject;
use serde_json::{Value, Map};
use crate::scene::{Scene, ImageOpts, RenderOpts, PathTermination, Background, Filter};
use crate::output::OutputOpts;
use std::io::prelude::*;
use std::fs::File;
use crate::material::model::MaterialModel;
use crate::material::texture::{Solid, CheckeredYPlane, Medium, NoiseMedium, self};
use crate::material::specular::Specular;
use crate::material::dielectric::Dielectric;
use crate::material::plastic::Plastic;
use crate::material::lambertian::Lambertian;
use crate::material::normal::NormalShade;
use crate::material::legacy::{ Whitted, FlatColor };
use crate::material::diffuse_light::DiffuseLight;
use crate::material::noise::{NoiseTexture, NoiseType};
use crate::participatingmedia::{ParticipatingMedium, HomogenousFog, Vacuum};
use crate::shapes::geometry::Geometry;

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
    pub sun: Value,
    #[serde(default)]
    pub objects: Vec<Value>,
    #[serde(default)]
    pub variables: Value,
    pub air: Option<Value>,
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

    pub fn parse_objects(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>) ->Vec<Arc<SceneObject>> {
         let mut objects: Vec<Arc<SceneObject>> = Vec::new();
         for obj in objs {
            if let Some(x) = SceneFile::parse_object(obj, materials, media) { objects.push(x) }
         }
         objects
    }

    pub fn parse_object_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<dyn Medium + Sync + Send> {
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

    pub fn parse_object(o: Value,  materials: &Map<String, Value>, media: &Map<String, Value>) -> Option<Arc<SceneObject>> {
        let t = o["type"].as_str().unwrap();

        if t == "skysphere" {
            return Some(Arc::new(SceneFile::parse_skysphere(&o)));
        }

        if t == "box_terrain" {
            return Some(Arc::new(SceneFile::parse_box_terrain(&o)));
        }
        if t == "ocean" {
            let d = create_ocean(&o);
            return Some(Arc::new(d));
        }
        if t == "firework" {
            let d = create_firework(&o);
            return Some(Arc::new(d));
        }

        if t == "fog" {
            let f = create_fog(&o);
            return Some(Arc::new(f));
        }

        if t == "hogan_kew_cirrus" {
            return Some(Arc::new(SceneFile::parse_hogan_kew_cirrus(&o)));
        }

        if t == "nubis_clouds" {
            return Some(Arc::new(SceneFile::parse_nubis_clouds(&o)));
        }

        if t == "volume" {
            return Some(Arc::new(SceneFile::parse_volume(&o)));
        }

        let geom = SceneFile::parse_geometry(&o);
        let m = SceneFile::parse_object_medium(&o, materials, media);
        if let Some(geometry) = geom {
            return Some(Arc::new(SceneObject {
                geometry,
                medium: m
            }));
        }
        
        if t == "checkeredplane" {
            return Some(Arc::new(SceneFile::parse_checkeredplane(&o, m)));
        }
        panic!("Unknown Object");
        //return None
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
    /// `droplet_diameter`), approximations, `diffraction` and `halos`.
    pub fn parse_scattering(o: &Value, default_g: f64) -> Scattering {
        Scattering {
            albedo: SceneFile::parse_color_def(o, "albedo", Color::white()),
            phase: SceneFile::parse_phase(o, default_g),
            approximation: SceneFile::parse_scattering_approximation(o),
            diffraction: SceneFile::parse_diffraction(o),
            halos: SceneFile::parse_halos(o),
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
        let bounds = BBox::new(
            Vector3::new(center.x - extent, base, center.y - extent),
            Vector3::new(center.x + extent, top, center.y + extent),
        );
        create_volume(bounds, Arc::new(layer), SceneFile::parse_scattering(o, 0.85))
    }

    /// A layer of cirrus by Hogan and Kew's fallstreak model (see
    /// `procedural::hogan_kew_cirrus`), filling `extent` either side of `center` ([x, z]):
    ///
    /// `{"type": "hogan_kew_cirrus", "genus": "cirrus" | "cirrostratus" | "cirrocumulus",
    /// "base": 8000, "top": 10000, "extent": 100000, "wind_direction": [x, z], "seed": 0}`
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
        let extent = num("extent", 100000.);
        let center = SceneFile::parse_vec2_def(o, "center", Vector2::new(0., 0.));
        c.extent = extent;
        c.center = center;
        c.fade = num("fade", c.fade).clamp(0., 1.);
        let bounds = BBox::new(
            Vector3::new(center.x - extent, base, center.y - extent),
            Vector3::new(center.x + extent, top, center.y + extent),
        );
        // With diffraction, the phase function is that of the rest: refraction and reflection.
        let g = if o.get("diffraction").is_some_and(|d| d != &Value::Bool(false)) { 0.5 } else { 0.75 };
        create_volume(bounds, Arc::new(c), SceneFile::parse_scattering(o, g))
    }

    pub fn parse_box_terrain(o: &Value) -> SceneObject {
        create_box_terrain(SceneFile::parse_int(&o["seed"], 0) as u64)
    }

    pub fn parse_geometry (o: &Value) -> Option<Box<dyn Geometry + Sync + Send>> {
        let t = o["type"].as_str().unwrap();

        if t == "sphere" {
            return Some(SceneFile::parse_sphere(o));
        }

        if t == "triangle" {
            return Some(SceneFile::parse_triangle(o));
        }

        if t == "mesh" {
            return Some(SceneFile::parse_mesh(o));
        }

        if t == "smoothmesh" {
            return Some(SceneFile::parse_smoothmesh(o));
        }

        if t == "box" {
            return Some(SceneFile::parse_box(o));
        }

        if t == "plane" {
            return Some(SceneFile::parse_plane(o));
        }

        if t == "rotate" {
            return Some(SceneFile::parse_rotation(o));
        }

        if t == "transform" {
            return Some(SceneFile::parse_transform(o));
        }

        if t == "difference" {
            return Some(SceneFile::parse_difference(o));
        }
        None
    }

    pub fn parse_difference(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        let a = SceneFile::parse_geometry(&o["a"]).unwrap(); // Panic if fails
        let b = SceneFile::parse_geometry(&o["b"]).unwrap();
        Box::new(Difference { 
            a: Primitive { item: a },
            b: Primitive { item: b }, 
        })
    }

    pub fn parse_rotation(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        let a = SceneFile::parse_geometry(&o["item"]).unwrap(); // Panic if fails
        let roll = SceneFile::parse_number(&o["roll"], 0.).to_radians();
        let pitch = SceneFile::parse_number(&o["pitch"], 0.).to_radians();
        let yaw = SceneFile::parse_number(&o["yaw"], 0.).to_radians();
        Box::new(Transform::rotate(a, roll, pitch, yaw))
    }

    /// `{"type": "transform", "item": {...}, "scale": 2 or [x, y, z], "rotate": {"roll", "pitch",
    /// "yaw"} in degrees, "translate": [x, y, z]}`: every part optional, applied in that order.
    pub fn parse_transform(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        let item = SceneFile::parse_geometry(&o["item"])
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

    pub fn parse_mesh(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        Box::new(Mesh::from_obj(
                SceneFile::parse_string(&o["src"]),
                SceneFile::parse_vec3_def(o, "scale", Vector3::new(1., 1., 1.))))
    }

    pub fn parse_smoothmesh(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        Box::new(SmoothMesh::from_obj(
                SceneFile::parse_string(&o["src"]),
                SceneFile::parse_vec3_def(o, "scale", Vector3::new(1., 1., 1.))))
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

    pub fn parse_plane(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        Box::new(Plane {
                y: SceneFile::parse_number(&o["y"], 0.),
            })
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
                let atmosphere = (0..ALTITUDE_STEPS).map(|i| {
                    let t = sky.sun_transmittance_at(i as f64 * ALTITUDE_STEP);
                    Color::new(t.x, t.y, t.z)
                }).collect();
                Some(Sun { atmosphere: Some(atmosphere), ..Sun::new(sky.sun_direction(), irradiance) })
            },
            None => {
                let d = v.get("direction").map(SceneFile::parse_vec3)
                    .expect("A sun without a skysphere needs a \"direction\"");
                assert!(d.norm() > 0., "The sun's direction must not be zero");
                Some(Sun::new(d, irradiance))
            },
        }.map(|mut sun| {
            if let Some(r) = v.get("angular_radius") {
                sun.angular_radius = SceneFile::parse_number(r, 0.).to_radians().max(1e-6);
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
        let objects = SceneFile::parse_objects(s.objects, &s.materials, &s.media);
        // Emitters with a surface we can sample become area lights.
        let area_lights: Vec<Arc<SceneObject>> = objects.iter()
            .filter(|o| o.medium.material_at(Vector3::zeros()).is_emitter() && o.geometry.surface_area() > 0.)
            .cloned()
            .collect();
        let o = SceneGraph::new(2, objects, max_bounding);
		
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
            output: OutputOpts {
                // The camera's view plane spans tan(angle) over the image's height.
                pixel_angle,
                ..OutputOpts::parse(&s.output)
            },
            area_lights,
            sun,
            atmosphere,
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
    pub fn from_string_overridden(s: String, o: &Overrides) -> Scene {
        let (width, height) = (o.width, o.height);
        let mut scene: SceneFile = serde_json::from_str(&s).unwrap();
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

    pub fn from_file(filename: &str, overrides: &Overrides) -> Scene {
        let mut scenefile = File::open(filename).unwrap();
        let mut contents = String::new();
        scenefile.read_to_string(&mut contents).unwrap();
        SceneFile::from_string_overridden(contents, overrides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let t = SceneFile::parse_transform(&serde_json::json!({
            "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 },
            "scale": 2, "rotate": { "yaw": 90 }, "translate": [0, 0, 5]
        }));
        let b = t.bounds();
        assert!((b.min.coords - Vector3::new(-2., -2., 3.)).norm() < 1e-9, "{}", b);
        let per_axis = SceneFile::parse_transform(&serde_json::json!({
            "item": { "type": "sphere", "location": [0, 0, 0], "radius": 1 }, "scale": [3, 1, 1]
        }));
        assert!((per_axis.bounds().max.x - 3.).abs() < 1e-9);
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
