use na::{Vec3, Norm, Cross};
use ray::Ray;

pub struct Camera {
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
    pub fn new(lookat: Vec3<f64>, location:Vec3<f64>, up:Vec3<f64>, angle: f64, height: i32, width: i32) -> Camera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();

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
            35.0,
            width, height
        );
        
        assert_approx_eq!(c.get_ray(10, 10).ro.x, 0f64);
        assert_approx_eq!(c.get_ray(10, 10).ro.y, 1f64);
        assert_approx_eq!(c.get_ray(10, 10).ro.z, -1f64);

        assert_approx_eq!(c.get_ray(10, 10).rd.x, 4.5012398439372845f64);
        assert_approx_eq!(c.get_ray(10, 10).rd.y, 7.501426591114969f64);
        assert_approx_eq!(c.get_ray(10, 10).rd.z, 8.915640153488065f64);
    }
}
