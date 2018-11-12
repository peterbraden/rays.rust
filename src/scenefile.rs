use camera;
use na::Vec3;
use scenegraph::SceneGraph;
use shapes::sphere::Sphere;
use shapes::plane::Plane;
use light::Light;
use color::Color;
use std::rc::Rc;
use sceneobject::SceneObject;
use serde_json::{Value, Map};
use scene::Scene;
use serde_json;
use std::io::prelude::*;
use std::fs::File;
use material;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    pub width: u32,
    pub height: u32,

    pub reflection: u32,
    pub ambient: f64,
    pub specular: bool,
    pub diffuse: bool,
    pub supersamples: u32,
    pub camera: Value,

    pub materials: Map<String, Value>,
    pub lights: Vec<Value>,
    pub objects: Vec<Value>,

    pub variables: Value,
}

impl SceneFile {
    pub fn parse_vec3(v: &Value) -> Vec3<f64> {
        return Vec3::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap());
    }

    pub fn parse_color(v: &Value) -> Color {
        return Color::new(v[0].as_f64().unwrap(),
                         v[1].as_f64().unwrap(),
                         v[2].as_f64().unwrap());
    
    }

    pub fn parse_camera(c: Value, width: u32, height: u32) -> camera::Camera {
        return camera::Camera::new(
            SceneFile::parse_vec3(&c["lookat"]),
            SceneFile::parse_vec3(&c["location"]),
            SceneFile::parse_vec3(&c["up"]),
            c["angle"].as_f64().unwrap(),
            width, height
        );
    }

    pub fn parse_objects(objs: Vec<Value>, materials: HashMap<String, material::Material>) ->Vec<Rc<SceneObject>> {
         let mut objects: Vec<Rc<SceneObject>> = Vec::new();
         for obj in objs {
            match SceneFile::parse_object(obj, &materials) {
                Some(x) => objects.push(x),
                None => {},
            }
         }
         return objects
    }

    pub fn parse_object(o: Value, materials: &HashMap<String, material::Material>) -> Option<Rc<SceneObject>> {
        let t = o["type"].as_str().unwrap();
        let m = materials.get(&o["material"].as_str().unwrap().to_string()).unwrap(); // This is pretty nasty, shame serde
        
        if t == "sphere" {
            return Some(Rc::new(SceneFile::parse_sphere(&o, m.clone())));
        }
        
        if t == "checkeredplane" {
            return Some(Rc::new(SceneFile::parse_checkeredplane(&o, m.clone())));
        }
        return None
    }

    pub fn parse_sphere(o: &Value, m: material::Material) -> SceneObject {
        return SceneObject {
            geometry: Box::new(Sphere::new(
                SceneFile::parse_vec3(&o["location"]),
                o["radius"].as_f64().unwrap())),
            medium: Box::new(material::Solid { m: m })
        };
    }

    pub fn parse_checkeredplane(o: &Value, m: material::Material) -> SceneObject {
        SceneObject {
            geometry: Box::new(Plane { y: o["y"].as_f64().unwrap() }),
            medium: Box::new(material::Solid { m: m })
        }
    }

    pub fn parse_material(o: &Value) -> material::Material {
        return material::Material {
            pigment: SceneFile::parse_color(&o["pigment"]), 
            metallic: o["metallic"].as_f64().unwrap(), 
            roughness: o["roughness"].as_f64().unwrap(), 
            reflection: o["reflection"].as_f64().unwrap(),
            phong: o["phong"].as_f64().unwrap(),
            normal_peturbation: Vec3::new(0., 0., 0.)
        }
    }

    pub fn parse_materials(o: &Map<String, Value>) -> HashMap<String, material::Material> {
        let mut materials = HashMap::new();
        for m in o.iter() {
            materials.insert(m.0.to_string(), SceneFile::parse_material(m.1));
        }
        return materials;
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
        let mut o = SceneGraph::new();
        let materials = SceneFile::parse_materials(&s.materials);
        let objects = SceneFile::parse_objects(s.objects, materials);
        o.push(objects);

        return Scene {
            width: s.width,
            height: s.height,
            camera: SceneFile::parse_camera(s.camera, s.width, s.height),
            objects: o,
            ambient: s.ambient,
            max_depth: s.reflection,
            lights: SceneFile::parse_lights(&s.lights),

            reflection: s.reflection > 0,
            specular: s.specular,
            diffuse: s.diffuse,
            supersamples: s.supersamples
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
