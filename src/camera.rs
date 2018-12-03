use na::{Vector3, Perspective3, Isometry3, Rotation, Rotation3};
use ray::Ray;
use std::f64;

use geometry::random_point_on_disc;


pub trait Camera {
    // Given a point (x=0-1, y=0-1) as a proportion of the way into the previously sized image
    // and (sx=0-1, sy=0-1), subsamples within that pixel, generate a ray for that pixel 
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray;
    fn get_coord_for_point(&self, point: Vector3<f64>) -> (f64,f64);
}

pub struct SimpleCamera {
    location: Vector3<f64>,

    camx: Vector3<f64>,
    camy: Vector3<f64>,
    camz: Vector3<f64>,

    tax: f64,
    tay: f64
}

impl SimpleCamera {
    pub fn new(lookat: Vector3<f64>, location:Vector3<f64>, up:Vector3<f64>, angle: f64, height: u32, width: u32) -> SimpleCamera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();
        let camy = camx.cross(
                &(Vector3::new(0f64,0f64,0f64) - camz)
                ).normalize();


        let aspect_ratio = (height as f64) / (width as f64);

        //let viewPlaneHalfWidth= (fieldOfView / 2.).tan()
        //let viewPlaneHalfHeight = aspectRatio*viewPlaneHalfWidth
        
        SimpleCamera {
            location: location,

            camz: camz,
            camx: camx * aspect_ratio,
            camy: camy,

            tax: angle.tan(),
            tay: angle.tan()
        }
    }
}

impl Camera for SimpleCamera {
    // x, y, supersamples
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {
        let xdir = self.camx * (x + sx - 0.5) * self.tax;
        let ydir = self.camy * (y + sy - 0.5) * self.tay;
        let dest = self.camz + xdir + ydir;

        Ray {
            ro: self.location,
            rd: dest
        }
    }

    fn get_coord_for_point(&self, point: Vector3<f64>) -> (f64,f64) {
        let x0 = self.camx * -0.5 * self.tax;
        let x1 = self.camx * 0.5 * self.tax;
        let y0 = self.camy * -0.5 * self.tay;
        let y1 = self.camy * 0.5 * self.tay;

        // Vectors at corner of view
        let tl = self.camz + x0 + y0;
        let tr = self.camz + x1 + y0;
        let bl = self.camz + x0 + y1;
        let br = self.camz + x1 + y1;
        
        let rd = (point - self.location).normalize();
        let x = (rd.x - tl.x) / (tr.x - tl.x);    
        let y = (rd.y - tr.y) / (br.y - tr.y);    
        return (x, y);
    }
}


/*

pub struct PerspectiveCamera {
    frustum: PerspectiveMatrix3<f64>,
//    isometry: Isometry3<f64>,
    cam_to_world: Rotation3<f64>,

//    up: Vector3<f64>,
    location: Vector3<f64>,
//    lookat: Vector3<f64>,

//    camx: Vector3<f64>,
//    camy: Vector3<f64>,
//    camz: Vector3<f64>,

 //angle: f64,
//    tax: f64,
//    tay: f64
}


impl PerspectiveCamera {
    pub fn new(lookat: Vector3<f64>, location:Vector3<f64>, up:Vector3<f64>, angle: f64, height: u32, width: u32) -> PerspectiveCamera {
        /*
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();
		*/

        let frustum = PerspectiveMatrix3::new(
            width as f64 / height as f64, // aspect
            angle,// fovy
            -1., // znear
            1.,// zfar
        );
        let isometry = Isometry3::new_observer_frame(&location.to_point(), &lookat.to_point(), &up);

        PerspectiveCamera {
            frustum: frustum,
//           isometry: isometry,
          cam_to_world: isometry.rotation,
//          lookat: lookat,
            location: location,
//          up: up,
//          angle: angle,
/*
            camz: camz,
            camx: camx,
            camy: camx.cross(
                &(Vector3::new(0f64,0f64,0f64) - camz)
                ).normalize(),

            tax: angle.tan(), // recip of len
            tay: ((height as f64 / width as f64) * angle).tan()
*/
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
/*
        let xdir = self.camx * (x - 0.5) * self.tax;
        let ydir = self.camy * (y - 0.5) * self.tay;
        let dest = self.camz + xdir + ydir;
*/
        let cam_rd = Vector3::new(x - 0.5, y - 0.5, 1.).normalize();
        //println!("cam_rd: {}", cam_rd);
        //let dw = self.frustum.project_vector(&cam_rd);
        //println!("dw: {}", dw);
        let rd = self.cam_to_world.rotate(&cam_rd);
        //println!("rd: {}", rd);

        
        Ray {
            ro: self.location,
            rd: rd.normalize()
        }
    }

    // Inverse of get ray
    // BROKEN
    pub fn get_coord_for_point(&self, point: Vector3<f64>) -> (f64,f64) {
        // Generate ray

        let vec = (point - self.location).normalize();
        let vec_cam = self.cam_to_world.inverse_rotate(&vec);

        return (vec_cam.x + 0.5, vec_cam.y + 0.5);
    }
}
*/

pub struct FlatLensCamera {
    location: Vector3<f64>,

    camx: Vector3<f64>,
    camy: Vector3<f64>,
    camz: Vector3<f64>,

    tax: f64,
    tay: f64,

    aperture: f64,
    focus: f64,
}


impl FlatLensCamera {
    pub fn new(
            lookat: Vector3<f64>,
            location:Vector3<f64>,
            up:Vector3<f64>,
            angle: f64,
            height: u32,
            width: u32,
            aperture: f64
         ) -> FlatLensCamera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();
        let camy = camx.cross(
                &(Vector3::new(0f64,0f64,0f64) - camz)
                ).normalize();


        let aspect_ratio = (height as f64) / (width as f64);
        let focus = (lookat - location).norm();

        //let viewPlaneHalfWidth= (fieldOfView / 2.).tan()
        //let viewPlaneHalfHeight = aspectRatio*viewPlaneHalfWidth
        
        FlatLensCamera {
            location: location,

            camz: camz,
            camx: camx * aspect_ratio,
            camy: camy,

            tax: angle.tan(),
            tay: angle.tan(),
        
            aperture: aperture,
            focus: focus,
        }
    }
}

impl Camera for FlatLensCamera {
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {

        let xdir = self.camx * (x + sx - 0.5) * self.tax;
        let ydir = self.camy * (y + sy - 0.5) * self.tay;
        let pinhole_dest = self.camz + xdir + ydir;

        let focal_point = self.location + pinhole_dest * self.focus;
        let point_lens = random_point_on_disc(self.aperture); 
        let ro = self.location + Vector3::new(point_lens[0], point_lens[1], 0.0);

        Ray {
            ro: ro,
            rd: (focal_point - ro).normalize()
        }
    }

    fn get_coord_for_point(&self, point: Vector3<f64>) -> (f64,f64) {
        let x0 = self.camx * (-0.5 * self.tax);
        let x1 = self.camx * (0.5 * self.tax);
        let y0 = self.camy * (-0.5 * self.tay);
        let y1 = self.camy * (0.5 * self.tay);

        // Vector direction at corner of view
        let l = self.camz + x0;
        let r = self.camz + x1;
        let t = self.camz + y0;
        let b = self.camz + y1;
        
        let rd = (point - self.location).normalize();
        let x = (rd.x - l.x) / (r.x - l.x);    
        let y = (rd.y - t.y) / (b.y - t.y);    

        let xdir = self.camx * 0.5 * self.tax;
        let ydir = self.camy * 0.5 * self.tay;
        // As angles
        let camd = self.camz + self.camx + self.camy;
        let xr = &(self.camz + xdir);
        let yr = &(self.camz + ydir);
        let x2 = (rd.dot(&xr.normalize()).atan2(xr.norm()) * 2.)  / self.tax;
        let y2 = (rd.dot(&yr.normalize()).atan2(yr.norm()) * 2.) / self.tay;


        let rdx = rd.component_mul(&Vector3::new(1., 0. ,1.)).normalize();
        let x3 = rdx.dot(&(self.camz - xdir).normalize()).acos() / self.tax;
        let rdy = rd.component_mul(&Vector3::new(0., 1. ,1.)).normalize();
        let y3 = rdy.dot(&(self.camz - ydir).normalize()).acos() / self.tay;

        //println!("-- {} {}, {} {}", x, y, x2, y2);
        return (x3, y3);
    }
}

#[cfg(test)]
macro_rules! assert_approx_eq(
    ($a:expr, $b:expr) => ({
    let (a, b) = (&$a, &$b);
    assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
}));

#[cfg(test)]
mod tests {
    use super::*;
    use na::Vector3;

    #[test]
    fn get_ray() {
        let width = 200;
        let height = 100;

        /*
        let c = Camera::new(
            Vector3::new(0f64,0f64,0f64),
            Vector3::new(0f64, 1f64, -1f64),
            Vector3::new(0f64,1f64,0f64),
            35.0,
            width, height
        );
        
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.x, 0f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.y, 1f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.z, -1f64);

        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.x, -0.21321662418650297);
        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.y, -1.052729238967664);
        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.z, 0.361484323405431);
        */
    }
}
