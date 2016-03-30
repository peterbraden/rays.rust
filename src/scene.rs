use camera;
use na::Vec3;
use scenegraph::SceneGraph;
use sphere::Sphere;


pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub camera: camera::Camera,
    pub objects: SceneGraph,

    pub ambient: f64,
    pub max_depth: i32,
}

impl Scene {
    pub fn demo () -> Scene {
        let width = 30;
        let height = 30;
    
        let c = camera::Camera::new(
            Vec3::new(0f64,0f64,0f64),
            Vec3::new(0f64,1f64,-5f64),
            Vec3::new(0f64,1f64,0f64),
            0.9,
            width, height
        );

        let mut o = SceneGraph::new();

        let s1 = Sphere::new(Vec3::new(0f64, 0f64, 0f64), 2f64);
        o.push(Box::new(s1));

        let s2 = Sphere::new(Vec3::new(3f64, 0f64, 5f64), 3f64);
        o.push(Box::new(s2));

        return Scene {
            width: width,
            height: height,
            camera: c,
            objects: o,
            ambient: 0.5f64,
            max_depth: 2,
        };
    }
}
