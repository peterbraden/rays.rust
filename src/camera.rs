use na::{Vec3, Norm, Cross, Dot, PerspectiveMatrix3};
use ray::Ray;

pub struct Camera {
    transform: PerspectiveMatrix3<f64>,

//    up: Vec3<f64>,
    location: Vec3<f64>,
//    lookat: Vec3<f64>,

    camx: Vec3<f64>,
    camy: Vec3<f64>,
    camz: Vec3<f64>,

 //angle: f64,
    tax: f64,
    tay: f64
}


impl Camera {
    pub fn new(lookat: Vec3<f64>, location:Vec3<f64>, up:Vec3<f64>, angle: f64, height: u32, width: u32) -> Camera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();

        let transform = PerspectiveMatrix3::new(
            (height as f64 / width as f64) // aspect
            // fovy
            // znear
            // zfar
        );


        Camera {
  //          lookat: lookat,
            location: location,
  //          up: up,
  //          angle: angle,

            camz: camz,
            camx: camx,
            camy: camx.cross(
                &(Vec3::new(0f64,0f64,0f64) - camz)
                ).normalize(),

            tax: angle.tan(), // recip of len
            tay: ((height as f64 / width as f64) * angle).tan()
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        //let xdir = self.camx * (x - 0.5) * self.tax;
        //let ydir = self.camy * (y - 0.5) * self.tay;
        //let dest = self.camz + xdir + ydir;

        let v = Vec3::new(x, y, 0.);
        let dir = self.transform.project_vector(v);

        Ray {
            ro: self.location,
            rd: dir
        }
    }

    // Inverse of get ray
    // BROKEN
    pub fn get_coord_for_point(&self, point: Vec3<f64>) -> (f64,f64) {
        // Generate ray
        let rd = (point - self.location).normalize();
        let dir = rd - self.camz;

        let xa = rd.dot(&(self.camy + self.camz)).acos();
        let ya = rd.dot(&self.camy).acos();
        //let za = rd.dot(&self.camz).acos();
    
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
    use na::Vec3;

    #[test]
    fn get_ray() {
        let width = 200;
        let height = 100;
        let c = Camera::new(
            Vec3::new(0f64,0f64,0f64),
            Vec3::new(0f64, 1f64, -1f64),
            Vec3::new(0f64,1f64,0f64),
            35.0 , // 35 radians?
            width, height
        );
        
        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.x, 0f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.y, 1f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1).ro.z, -1f64);

        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.x, -0.21321662418650297);
        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.y, -1.052729238967664);
        assert_approx_eq!(c.get_ray(0.05, 0.1).rd.z, 0.361484323405431);
    }

    #[test]
    fn get_coord_for_point(){
        let width = 100;
        let height = 100;
        let c = Camera::new(
            Vec3::new(0f64,0f64,0f64),
            Vec3::new(0f64, 0f64, -10f64),
            Vec3::new(0f64,1f64,0f64),
            0.61,
            width, height
        );
    
        let cr = c.get_ray(0.05, 0.1).rd;
        assert_approx_eq!(c.get_coord_for_point(cr).0, 0.05);
        assert_approx_eq!(c.get_coord_for_point(cr).1, 0.1);

    
    }
}
