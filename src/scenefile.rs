use crate::camera;
use crate::na::{Vector3, Vector2};
use crate::scenegraph::SceneGraph;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use crate::shapes::plane::Plane;
use crate::shapes::mesh::{Mesh, SmoothMesh};
use crate::shapes::csg::{Primitive, Difference};
use crate::shapes::transform::{Transform};
use crate::ocean::create_ocean;
use crate::shapes::bbox::BBox;
use crate::light::Light;
use crate::color::Color;
use crate::skysphere::create_sky_sphere;
use crate::procedural::box_terrain::create_box_terrain;
use crate::procedural::fireworks::create_firework;
use crate::participatingmedia::create_fog;
use std::sync::Arc;
use crate::sceneobject::SceneObject;
use serde_json::{Value, Map};
use crate::scene::{Scene, ImageOpts, RenderOpts};
use serde_json;
use std::io::prelude::*;
use std::fs::File;
use crate::material::model::MaterialModel;
use crate::material::texture::{Solid, CheckeredYPlane, Medium};
use crate::material::specular::Specular;
use crate::material::dielectric::Dielectric;
use crate::material::plastic::Plastic;
use crate::material::lambertian::Lambertian;
use crate::material::normal::NormalShade;
use crate::material::legacy::{ Whitted, FlatColor };
use crate::material::diffuse_light::DiffuseLight;
use crate::participatingmedia::{ParticipatingMedium, HomogenousFog, Vacuum};
use crate::shapes::geometry::Geometry;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    pub width: Value,
    pub height: Value,
    pub chunk_size: Value,
    pub supersamples: Value, 
    pub samples_per_chunk: Value, 
    pub camera: Value, 
    pub shadow_bias: Value, 
    pub background: Value,
    pub max_depth: Value,
    pub materials: Map<String, Value>,
    pub media: Map<String, Value>,
    pub lights: Vec<Value>,
    pub objects: Vec<Value>,
    pub variables: Value,
    pub air: Option<Value>,
}

impl SceneFile {
    pub fn parse_number(v: &Value, default: f64) -> f64 {
        match v.as_f64(){
            Some(x) => return x,
            None => return default
        }
    }
    
    pub fn parse_int(v: &Value, default: usize) -> usize {
        match v.as_i64(){
            Some(x) => return x as usize,
            None => return default
        }
    }
    
    pub fn parse_string(v: &Value) -> String {
        return v.as_str().unwrap().to_string(); // This is pretty nasty, shame serde
    }

    pub fn parse_vec3(v: &Value) -> Vector3<f64> {
        return Vector3::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap());
    }

    pub fn parse_vec3_def(v: &Value, k: &str, def: Vector3<f64>) -> Vector3<f64> {
        match &v.get(&k) {
            Some(x) => SceneFile::parse_vec3(x),
            None => return def
        }
    }

    pub fn parse_vec2(v: &Value) -> Vector2<f64> {
        return Vector2::new(v[0].as_f64().unwrap(),
                            v[1].as_f64().unwrap());
    }

    pub fn parse_vec2_def(v: &Value, k: &str, def: Vector2<f64>) -> Vector2<f64> {
        match &v.get(&k) {
            Some(x) => SceneFile::parse_vec2(x),
            None => return def
        }
    }

    pub fn parse_color(v: &Value) -> Color {
        return Color::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap());
    
    }


    pub fn parse_color_def(v: &Value, k: &str, def: Color) -> Color {
        match &v.get(&k) {
            Some(x) => SceneFile::parse_color(x),
            None => return def
        }
    }

    pub fn parse_camera(c: Value, width: u32, height: u32) -> camera::FlatLensCamera {
        return camera::FlatLensCamera::new(
            SceneFile::parse_vec3(&c["lookat"]),
            SceneFile::parse_vec3(&c["location"]),
            SceneFile::parse_vec3(&c["up"]),
            c["angle"].as_f64().unwrap(),
            width, height,
            SceneFile::parse_number(&c["aperture"], 0.2)
        );
    }

    pub fn parse_objects(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>) ->Vec<Arc<SceneObject>> {
         let mut objects: Vec<Arc<SceneObject>> = Vec::new();
         for obj in objs {
            match SceneFile::parse_object(obj, &materials, &media) {
                Some(x) => objects.push(x),
                None => {},
            }
         }
         return objects
    }

    pub fn parse_object_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<dyn Medium + Sync + Send> {
        match &o.get("medium") {
            Some(mid) => {
                return SceneFile::parse_medium_ref(mid, materials, media).unwrap()
            },
            None => {
                // Default is Solid
                let m = SceneFile::parse_material_ref(&o["material"], materials).unwrap();
                return Box::new(Solid { m: m })
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

        let geom = SceneFile::parse_geometry(&o);
        let m = SceneFile::parse_object_medium(&o, materials, media);
        if geom.is_some(){
            return Some(Arc::new(SceneObject {
                geometry: geom.unwrap(),
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
        return create_sky_sphere(o);
    }

    pub fn parse_box_terrain(_o: &Value) -> SceneObject {
        return create_box_terrain();
    }

    pub fn parse_geometry (o: &Value) -> Option<Box<dyn Geometry + Sync + Send>> {
        let t = o["type"].as_str().unwrap();

        if t == "sphere" {
            return Some(SceneFile::parse_sphere(&o));
        }

        if t == "triangle" {
            return Some(SceneFile::parse_triangle(&o));
        }

        if t == "mesh" {
            return Some(SceneFile::parse_mesh(&o));
        }

        if t == "smoothmesh" {
            return Some(SceneFile::parse_smoothmesh(&o));
        }

        if t == "box" {
            return Some(SceneFile::parse_box(&o));
        }

        if t == "plane" {
            return Some(SceneFile::parse_plane(&o));
        }

        if t == "rotate" {
            return Some(SceneFile::parse_rotation(&o));
        }

        if t == "difference" {
            return Some(SceneFile::parse_difference(&o));
        }
        return None
    }

    pub fn parse_difference(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        let a = SceneFile::parse_geometry(&o["a"]).unwrap(); // Panic if fails
        let b = SceneFile::parse_geometry(&o["b"]).unwrap();
        return Box::new(Difference { 
            a: Primitive { item: a },
            b: Primitive { item: b }, 
        });
    }

    pub fn parse_rotation(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        let a = SceneFile::parse_geometry(&o["item"]).unwrap(); // Panic if fails
        let roll = SceneFile::parse_number(&o["roll"], 0.).to_radians();
        let pitch = SceneFile::parse_number(&o["pitch"], 0.).to_radians();
        let yaw = SceneFile::parse_number(&o["yaw"], 0.).to_radians();
        return Box::new(Transform::rotate(a, roll, pitch, yaw));
    }

    pub fn parse_mesh(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        return Box::new(Mesh::from_obj(
                SceneFile::parse_string(&o["src"]),
                SceneFile::parse_vec3_def(&o, "scale", Vector3::new(1., 1., 1.))));
    }

    pub fn parse_smoothmesh(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        return Box::new(SmoothMesh::from_obj(
                SceneFile::parse_string(&o["src"]),
                SceneFile::parse_vec3_def(&o, "scale", Vector3::new(1., 1., 1.))));
    }

    pub fn parse_sphere(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        return Box::new(Sphere::new(
                SceneFile::parse_vec3(&o["location"]),
                o["radius"].as_f64().unwrap()));
    }

    pub fn parse_box(o: &Value) -> Box<dyn Geometry + Sync + Send>{
        return Box::new(BBox::new(
                SceneFile::parse_vec3(&o["min"]),
                SceneFile::parse_vec3(&o["max"])))
    }

    pub fn parse_triangle(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        return Box::new(Triangle::new(
                SceneFile::parse_vec3(&o["v0"]),
                SceneFile::parse_vec3(&o["v1"]),
                SceneFile::parse_vec3(&o["v2"])));
    }

    pub fn parse_plane(o: &Value) -> Box<dyn Geometry + Sync + Send> {
        return Box::new(Plane {
                y: SceneFile::parse_number(&o["y"], 0.),
            });
    }

    pub fn parse_checkeredplane(o: &Value, m: Box<dyn Medium + Sync + Send>) -> SceneObject {
        SceneObject {
            geometry: Box::new(Plane { y: o["y"].as_f64().unwrap() }),
            medium: m
        }
    }

    pub fn parse_material_ref(key: &Value, materials: &Map<String, Value> ) -> Option<Box<dyn MaterialModel + Sync + Send>> {
        let props = materials.get(&SceneFile::parse_string(key)).unwrap();
        return SceneFile::parse_material(props);
    }

    pub fn parse_material(o: &Value) -> Option<Box<dyn MaterialModel + Sync + Send>> {
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
            let d:Plastic = Plastic {
                albedo:SceneFile::parse_color(&o["albedo"]), 
                refractive_index: SceneFile::parse_number(&o["refractive_index"], 1.),
                roughness: SceneFile::parse_number(&o["roughness"], 0.),
                opacity: SceneFile::parse_number(&o["opacity"], 0.),
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

    pub fn parse_medium_ref(key: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Option<Box<dyn Medium + Sync + Send>> {
        let props = media.get(&SceneFile::parse_string(key)).unwrap();
        return SceneFile::parse_medium(props, materials);
    }

    pub fn parse_medium(o: &Value, materials: &Map<String, Value>) -> Option<Box<dyn Medium + Sync + Send>> {
        let t = o["type"].as_str().unwrap();
        if t == "solid" {
            let m = SceneFile::parse_material_ref(&o["material"], materials).unwrap(); 
            return Some(Box::new(Solid { m: m }));
        }

        if t == "checkered-y-plane" {
            let m1 = SceneFile::parse_material_ref(&o["m1"], materials).unwrap(); 
            let m2 = SceneFile::parse_material_ref(&o["m2"], materials).unwrap(); 
            let xsize = SceneFile::parse_number(&o["xsize"], 1.);
            let zsize = SceneFile::parse_number(&o["zsize"], 1.);
            return Some(Box::new(CheckeredYPlane {
                m1: m1, m2: m2, xsize: xsize, zsize: zsize
            }));

        }

        return None
    }
    pub fn parse_air(_o: &Option<Value>) -> Box<dyn ParticipatingMedium>{
        let air: Box<dyn ParticipatingMedium> = Box::new(Vacuum {});
        /*
        if o.is_some() {
            air = Box::new(HomogenousFog { density: 0.001, color: Color::red() });
        }
        */
        return air;
    }


    pub fn parse_light(o: &Value) -> Light {
        return Light {
                position: SceneFile::parse_vec3(&o["location"]),
                color: Color::white(),
                intensity: o["intensity"].as_f64().unwrap(),
            }
    }

    pub fn parse_lights(lights: &Vec<Value>) -> Vec<Light> {
        let mut l: Vec<Light> = Vec::new();
        for light in lights {
            l.push(SceneFile::parse_light(&light));
        }
        return l
    }

    pub fn from_scenefile(s: SceneFile) -> Scene {
        let max_bounding = BBox::new( // TODO
            Vector3::new(-1000., -1000., -1000.),
            Vector3::new(1000., 1000., 1000.),
        );

        let objects = SceneFile::parse_objects(s.objects, &s.materials, &s.media);
        let o = SceneGraph::new(2, objects, max_bounding);
		
        let width = SceneFile::parse_int(&s.width, 640);
        let height =SceneFile::parse_int(&s.height, 480);

        return Scene {
            image: ImageOpts { width, height },
            render: RenderOpts {
                max_depth: SceneFile::parse_int(&s.max_depth, 2),
                background: SceneFile::parse_color(&s.background),
                shadow_bias: SceneFile::parse_number(&s.shadow_bias, 1e-7f64),
                supersamples: SceneFile::parse_int(&s.supersamples, 35),  
                chunk_size: SceneFile::parse_int(&s.chunk_size, 64), 
                samples_per_chunk: SceneFile::parse_int(&s.samples_per_chunk, 2),
            },
            camera: Box::new(SceneFile::parse_camera(s.camera, width as u32, height as u32)),
            lights: SceneFile::parse_lights(&s.lights),
            objects: o,
            max_bounding,
            black_threshold: SceneFile::parse_number(&s.shadow_bias, 1e-7f64) ,
            air_medium: SceneFile::parse_air(&s.air),
        };
    }

    pub fn from_string(s: String) -> Scene {
        let scene: SceneFile = serde_json::from_str(&s).unwrap();
        return SceneFile::from_scenefile(scene);
    }

    pub fn from_file(filename: &str) -> Scene {
        let mut scenefile = File::open(filename).unwrap();
        let mut contents = String::new();
        scenefile.read_to_string(&mut contents).unwrap();
        return SceneFile::from_string(contents);
    }
}
