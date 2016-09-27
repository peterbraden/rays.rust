use camera;
//use na::Vec3;
use scenegraph::SceneGraph;
use light::Light;
use color::Color;


pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Box<camera::Camera + Sync>,
    pub objects: SceneGraph,
    pub lights: Vec<Light>,

    pub background: Color,

    pub ambient: f64,
    pub max_depth: u64,

    pub reflection: bool,
    pub specular: bool,
    pub ambient_diffuse: u64,
    pub diffuse: bool,
    pub shadow_bias: f64,
    pub supersamples: u32,
}

<<<<<<< HEAD
=======
impl Scene {
    pub fn demo () -> Scene {
        let width = 100;
        let height = 100;
    
        let c = camera::Camera::new(
            Vec3::new(0f64,0f64,0f64), //lookat
            Vec3::new(5f64,10f64,-20f64), // loc
            Vec3::new(0f64,1f64,0f64), // up
            0.9,
            width, height
        );

        let mut o = SceneGraph::new();

        let s1 = Sphere::new(Vec3::new(0f64, 2f64, 0f64), 2f64);
        let s2 = Sphere::new(Vec3::new(3f64, 3f64, 5f64), 3f64);
        let s3 = Sphere::new(Vec3::new(-6f64, 6f64, 5f64), 6f64);
        let floor = CheckeredPlane { y: 1f64 };

        let objects: Vec<Rc<SceneObject>> = vec!(Rc::new(s1), Rc::new(s2), Rc::new(s3), /*Rc::new(floor)*/);
        o.push(objects);



        let l = vec!(
            Light {
                position: Vec3::new(10f64, 10f64, 0f64),
                color: Color::white(),
                intensity: 0.9,
            },
            /*
            Light {
                position: Vec3::new(0f64, 50f64, 50f64),
                color: Color::red(),
                intensity: 0.9,
            },

            Light {
                position: Vec3::new(-50f64, 50f64, -50f64),
                color: Color::green(),
                intensity: 0.9,
            },

            Light {
                position: Vec3::new(50f64, 50f64, -50f64),
                color: Color::blue(),
                intensity: 0.9,
            },
            */
        );


        return Scene {
            width: width,
            height: height,
            camera: c,
            objects: o,
            ambient: 0.2f64,
            max_depth: 1,
            lights: l,

            reflection: true,
            specular: true,
            diffuse: true,
        };
    }
}
>>>>>>> 1d1ab25... Octree algorithm
