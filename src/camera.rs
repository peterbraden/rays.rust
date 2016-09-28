use na::{Vector3, Norm, Cross, Dot, PerspectiveMatrix3, Isometry3, Rotate};
use ray::Ray;
use std::f64;

pub struct Camera {
//    frustum: PerspectiveMatrix3<f64>,
    isometry: Isometry3<f64>,

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


impl Camera {
    pub fn new(lookat: Vector3<f64>, location:Vector3<f64>, up:Vector3<f64>, angle: f64, height: u32, width: u32) -> Camera {
        /*
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();

        let frustum = PerspectiveMatrix3::new(
            (width as f64 / height as f64), // aspect
            angle,// fovy
            0.1, // znear
            10000.,// zfar
        );
*/
        let isometry = Isometry3::new_observer_frame(&location.to_point(), &lookat.to_point(), &up);

        Camera {
//         frustum: frustum,
            isometry: isometry,
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
        let d = Vector3::new((x - 0.5) * 2., (y - 0.5) * 2., 2.).normalize();
        println!("d: {}", d);
        //let dw = self.frustum.project_vector(&d);
        //println!("dw: {}", dw);
        let rd = self.isometry.rotation.rotate(&d).normalize();
        println!("rd: {}", rd);

        
        Ray {
            ro: self.location,
            rd: rd.normalize()
        }
    }

    // Inverse of get ray
    // BROKEN
    pub fn get_coord_for_point(&self, point: Vector3<f64>) -> (f64,f64) {
        // Generate ray
        /*
        let rd = (point - self.location).normalize();
        let dir = rd - self.camz;

        let xa = rd.dot(&(self.camy + self.camz)).acos();
        let ya = rd.dot(&self.camy).acos();
        //let za = rd.dot(&self.camz).acos();
    */
        let x = 0.;//xa.tan() ;
        let y = 0.;//ya.tan();

        return (x, y);
    }
}


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

        let c = Camera::new(
            Vector3::new(0f64, 0f64, 0f64),
            Vector3::new(0f64, 0f64, -1f64),
            Vector3::new(0f64, 1f64, 0f64),
            0.1 , // 35 radians?
            width, height
        );
        println!(">>> 0,0 {}, ", c.get_ray(0., 0.));
        println!(">>> 0.5,0.5 {}, ", c.get_ray(0.5, 0.5));
        println!(">>> 1,1 {}, ", c.get_ray(1., 1.));


        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.x, 0f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.y, 0f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.z, -1f64);

        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.x, -0.21321662418650297);
        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.y, -1.052729238967664);
        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.z, 0.361484323405431);
    }

    /*
    #[test]
    fn get_coord_for_point(){
        let width = 100;
        let height = 100;
        let c = Camera::new(
            Vector3::new(0f64,0f64,0f64),
            Vector3::new(0f64, 0f64, -10f64),
            Vector3::new(0f64,1f64,0f64),
            0.61,
            width, height
        );
    
        let cr = c.get_ray(0.05, 0.1).rd;
        assert_approx_eq!(c.get_coord_for_point(cr).0, 0.05);
        assert_approx_eq!(c.get_coord_for_point(cr).1, 0.1);

    
    }
    */
}
