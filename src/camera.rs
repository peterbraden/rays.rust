use na::{Vec3, Norm, Cross};
use ray::Ray;
use geometry::random_point_on_disc;


pub trait Camera {
    // Given a point (x=0-1, y=0-1) as a proportion of the way into the previously sized image
    // and (sx=0-1, sy=0-1), subsamples within that pixel, generate a ray for that pixel 
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray;
}


pub struct SimpleCamera {
    location: Vec3<f64>,

    camx: Vec3<f64>,
    camy: Vec3<f64>,
    camz: Vec3<f64>,

    tax: f64,
    tay: f64
}


impl SimpleCamera {
    pub fn new(lookat: Vec3<f64>, location:Vec3<f64>, up:Vec3<f64>, angle: f64, height: u32, width: u32) -> SimpleCamera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();
        let camy = camx.cross(
                &(Vec3::new(0f64,0f64,0f64) - camz)
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
}

pub struct FlatLensCamera {
    location: Vec3<f64>,

    camx: Vec3<f64>,
    camy: Vec3<f64>,
    camz: Vec3<f64>,

    tax: f64,
    tay: f64,

    aperture: f64,
    focus: f64,
}


impl FlatLensCamera {
    pub fn new(
            lookat: Vec3<f64>,
            location:Vec3<f64>,
            up:Vec3<f64>,
            angle: f64,
            height: u32,
            width: u32,
            aperture: f64
         ) -> FlatLensCamera {
        let camz = (lookat - location).normalize();
        let camx = up.cross(&camz).normalize();
        let camy = camx.cross(
                &(Vec3::new(0f64,0f64,0f64) - camz)
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
        let ro = self.location + Vec3::new(point_lens[0], point_lens[1], 0.0);

        Ray {
            ro: ro,
            rd: (focal_point - ro).normalize()
        }


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
    use na::Vec3;

    #[test]
    fn get_ray() {
        let width = 200;
        let height = 100;

        let c = OrthographicCamera::new(
            Vec3::new(0f64,0f64,0f64),
            Vec3::new(0f64, 1f64, -1f64),
            Vec3::new(0f64,1f64,0f64),
            35.0,
            width, height
        );
        
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.x, 0f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.y, 1f64);
        assert_approx_eq!(c.get_ray(0.1, 0.1, 0., 0.).ro.z, -1f64);

        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.x, -0.21321662418650297);
        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.y, -1.052729238967664);
        assert_approx_eq!(c.get_ray(0.05, 0.1, 0., 0.).rd.z, 0.361484323405431);
    }
}
