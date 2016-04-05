use camera;
use na::Vec3;
use scenegraph::SceneGraph;
use sphere::Sphere;
use checkeredplane::CheckeredPlane;
use light::Light;
use color::Color;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: camera::Camera,
    pub objects: SceneGraph,
    pub lights: Vec<Light>,

    pub ambient: f64,
    pub max_depth: i32,


    pub reflection: bool,
    pub specular: bool,
    pub diffuse: bool,
}

impl Scene {
    pub fn demo () -> Scene {
        let width = 200;
        let height = 200;
    
        let c = camera::Camera::new(
            Vec3::new(0f64,0f64,0f64), //lookat
            Vec3::new(5f64,5f64,-10f64), // loc
            Vec3::new(0f64,1f64,0f64), // up
            0.9,
            width, height
        );

        let mut o = SceneGraph::new();

        let s1 = Sphere::new(Vec3::new(0f64, 2f64, 0f64), 2f64);
        o.push(Box::new(s1));

        let s2 = Sphere::new(Vec3::new(3f64, 3f64, 5f64), 3f64);
        o.push(Box::new(s2));


        let floor = CheckeredPlane { y: 0.0 };
        o.push(Box::new(floor));

        let l = vec!(
            Light {
                position: Vec3::new(-3f64, 10f64, 0f64),
                color: Color::white(),
                intensity: 0.9,
            }
        );

        return Scene {
            width: width,
            height: height,
            camera: c,
            objects: o,
            ambient: 0.02f64,
            max_depth: 2,
            lights: l,

            reflection: true,
            specular: true,
            diffuse: true,
        };
    }
}
