use na::{Vec3, Norm, Cross};
use ray::Ray;

pub struct Camera {
    up: Vec3<f64>,
    location: Vec3<f64>,
    lookat: Vec3<f64>,

    camx: Vec3<f64>,
    camy: Vec3<f64>,
    camz: Vec3<f64>,

    angle: f64,
    tax: f64,
    tay: f64
}


impl Camera {
    pub fn new(lookat: Vec3<f64>, location:Vec3<f64>, up:Vec3<f64>, angle: f64, height: i32, width: i32) -> Camera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();

        Camera {
            lookat: lookat,
            location: location,
            up: up,
            angle: angle,

            camz: camz,
            camx: camx,
            camy: camx.cross(
                &(Vec3::new(0f64,0f64,0f64) - camz)
                ).normalize(),

            tax: angle.tan(),
            tay: ((height as f64 / width as f64) * angle).tan()
        }
    }

    pub fn get_ray(&self, x: i32, y: i32) -> Ray {
        let xdir = self.camx * (x as f64 - 0.5) * self.tax;
        let ydir = self.camy * (y as f64 - 0.5) * self.tay;
        let dest = self.camz + xdir + ydir;

        Ray {
            ro: self.location,
            rd: dest
        }
    }
}
