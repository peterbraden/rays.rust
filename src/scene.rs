use camera;
use na::Vec3;
use scenegraph::SceneGraph;


pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub camera: camera::Camera,
    pub objects: SceneGraph
}

impl Scene {
    pub fn demo () -> Scene {
        let width = 20i32;
        let height = 10i32;
    
        let c = camera::Camera::new(
            Vec3::new(0f64,0f64,0f64),
            Vec3::new(0f64,5f64,-10f64),
            Vec3::new(0f64,1f64,0f64),
            35.0,
            width, height
        );

        let o = SceneGraph::new();


        return Scene {
            width: width,
            height: height,
            camera: c,
            objects: o
        };
    }
}
