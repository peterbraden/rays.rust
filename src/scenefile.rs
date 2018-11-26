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
use material::model::MaterialModel;
use material::texture::{Solid, CheckeredYPlane, Medium};
use material::specular::Specular;
use material::dielectric::Dielectric;
use material::lambertian::Lambertian;
use material::normal::NormalShade;
use material::diffuse_light::DiffuseLight;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    pub width: u32,
    pub height: u32,

    background: Value,

    pub reflection: u64,
    pub ambient: f64,
    pub specular: bool,
    pub ambient_diffuse: u64,
    pub diffuse: bool,
    pub shadow_bias: f64,
    pub supersamples: u32,
    pub camera: Value,

    pub materials: Map<String, Value>,
    pub media: Map<String, Value>,
    pub lights: Vec<Value>,
    pub objects: Vec<Value>,

    pub variables: Value,
}

impl SceneFile {
    pub fn parse_number(v: &Value, default: f64) -> f64 {
        match v.as_f64(){
            Some(x) => return x,
            None => return default
        }
    }
    
    pub fn parse_string(v: &Value) -> String {
        return v.as_str().unwrap().to_string(); // This is pretty nasty, shame serde
    }


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

    pub fn parse_objects(objs: Vec<Value>, materials: &Map<String, Value>, media: &Map<String, Value>) ->Vec<Rc<SceneObject>> {
         let mut objects: Vec<Rc<SceneObject>> = Vec::new();
         for obj in objs {
            match SceneFile::parse_object(obj, &materials, &media) {
                Some(x) => objects.push(x),
                None => {},
            }
         }
         return objects
    }

    pub fn parse_object_medium(o: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Box<Medium> {
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

    pub fn parse_object(o: Value,  materials: &Map<String, Value>, media: &Map<String, Value>) -> Option<Rc<SceneObject>> {
        let t = o["type"].as_str().unwrap();
        let m = SceneFile::parse_object_medium(&o, materials, media);
        
        if t == "sphere" {
            return Some(Rc::new(SceneFile::parse_sphere(&o, m)));
        }
        
        if t == "checkeredplane" {
            return Some(Rc::new(SceneFile::parse_checkeredplane(&o, m)));
        }
        return None
    }

    pub fn parse_sphere(o: &Value, m: Box<Medium>) -> SceneObject {
        return SceneObject {
            geometry: Box::new(Sphere::new(
                SceneFile::parse_vec3(&o["location"]),
                o["radius"].as_f64().unwrap())),
            medium: m
        };
    }

    pub fn parse_checkeredplane(o: &Value, m: Box<Medium>) -> SceneObject {
        SceneObject {
            geometry: Box::new(Plane { y: o["y"].as_f64().unwrap() }),
            medium: m
        }
    }

    pub fn parse_material_ref(key: &Value, materials: &Map<String, Value> ) -> Option<Box<MaterialModel>> {
        let props = materials.get(&SceneFile::parse_string(key)).unwrap();
        return SceneFile::parse_material(props);
    }

    pub fn parse_material(o: &Value) -> Option<Box<MaterialModel>> {
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
            normal_peturbation: Vec3::new(0., 0., 0.)
        }
            */
        None
    }

    pub fn parse_medium_ref(key: &Value, materials: &Map<String, Value>, media: &Map<String, Value> ) -> Option<Box<Medium>> {
        let props = media.get(&SceneFile::parse_string(key)).unwrap();
        return SceneFile::parse_medium(props, materials);
    }

    pub fn parse_medium(o: &Value, materials: &Map<String, Value>) -> Option<Box<Medium>> {
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
        let objects = SceneFile::parse_objects(s.objects, &s.materials, &s.media);
        o.push(objects);

        return Scene {
            width: s.width,
            height: s.height,
            camera: Box::new(SceneFile::parse_camera(s.camera, s.width, s.height)),
            objects: o,
            ambient: s.ambient,
            max_depth: s.reflection,
            lights: SceneFile::parse_lights(&s.lights),
            background: SceneFile::parse_color(&s.background),

            reflection: s.reflection > 0,
            specular: s.specular,
            diffuse: s.diffuse,
            ambient_diffuse: s.ambient_diffuse,
            shadow_bias: s.shadow_bias,
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
