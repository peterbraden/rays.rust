//! # Cameras
//!
//! A camera turns a point on the image into a ray into the scene (Pharr, Jakob and Humphreys,
//! "Physically Based Rendering", 4th ed., chapter 5, "Cameras and Film",
//! <https://pbr-book.org/4ed/Cameras_and_Film>; cited below as PBR). Image coordinates run from
//! (0, 0) at the bottom left to (1, 1) at the top right.
//!
//! - `ThinLensCamera`: perspective, through a thin lens of radius `aperture` focused at the
//!   distance of the point looked at (PBR section 5.2.3); a pinhole when the aperture is zero.
//! - `OrthographicCamera`: parallel rays from a rectangle (PBR section 5.2.1), for plans and
//!   elevations.
//! - `SphericalCamera`: every direction round the camera, mapped to the image
//!   equirectangularly or by an equal-area mapping (PBR section 5.3).
//!
//! Every camera has a `Placement`: where it is and which way it looks, and optionally a second
//! pose it moves to while the shutter is open (`motion`).
use crate::geometry::random_point_on_disc;
use crate::na::{Matrix3, Rotation3, UnitQuaternion, Vector3};
use crate::ray::Ray;
use std::f64;
use std::f64::consts::PI;

pub trait Camera {
    /// The ray for image position (x + sx, y + sy): (x, y) the pixel's corner and (sx, sy) the
    /// offset within it, each as a fraction of the image's width and height.
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray;
}

/// A camera's position and axes: `camx` to the right of the image, `camy` up it, `camz` the
/// direction it looks. Right-handed, as pbrt, Tungsten and OBJ models are: looking down -z
/// with y up, +x is to the right.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub location: Vector3<f64>,
    pub camx: Vector3<f64>,
    pub camy: Vector3<f64>,
    pub camz: Vector3<f64>,
}

impl Frame {
    /// At `location`, looking at `lookat`, with `up` towards the top of the image.
    pub fn look_at(location: Vector3<f64>, lookat: Vector3<f64>, up: Vector3<f64>) -> Frame {
        let camz = (lookat - location).normalize();
        let mut camx = camz.cross(&up).normalize();
        if camx.iter().any(|v| v.is_nan()) {
            // Looking along `up`: any axis will do.
            camx = Vector3::new(1., 0., 0.);
        }
        let camy = camx.cross(&camz).normalize();
        Frame { location, camx, camy, camz }
    }

    /// The rotation taking camera space (x right, y up, looking down -z) to these axes.
    fn rotation(&self) -> UnitQuaternion<f64> {
        UnitQuaternion::from_rotation_matrix(&Rotation3::from_matrix_unchecked(Matrix3::from_columns(&[self.camx, self.camy, -self.camz])))
    }

    fn from_rotation(location: Vector3<f64>, r: &UnitQuaternion<f64>) -> Frame {
        Frame { location, camx: r * Vector3::x(), camy: r * Vector3::y(), camz: -(r * Vector3::z()) }
    }
}

/// Where a camera is, and if it moves, where it goes: from `start` at time 0 to `end` at time 1
/// (`motion`), its position interpolated linearly and its orientation spherically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub start: Frame,
    end: Option<(Frame, UnitQuaternion<f64>, UnitQuaternion<f64>)>,
}

impl Placement {
    pub fn still(frame: Frame) -> Placement {
        Placement { start: frame, end: None }
    }

    pub fn moving(start: Frame, end: Frame) -> Placement {
        Placement { start, end: Some((end, start.rotation(), end.rotation())) }
    }

    /// The frame at the current sample's time.
    pub fn frame(&self) -> Frame {
        match &self.end {
            None => self.start,
            Some((end, q0, q1)) => {
                let t = crate::motion::time();
                let q = q0.try_slerp(q1, t, 1e-12).unwrap_or(if t < 0.5 { *q0 } else { *q1 });
                Frame::from_rotation(self.start.location.lerp(&end.location, t), &q)
            },
        }
    }
}

/// Perspective projection through a thin lens (PBR section 5.2.3): rays leave a disc of radius
/// `aperture` in the camera's image plane, through the point on the plane of focus that a
/// pinhole would have seen.
pub struct ThinLensCamera {
    pub placement: Placement,
    /// Horizontal extent of the view relative to vertical.
    aspect: f64,
    /// tan of the scene file's `angle`: the image's height at unit distance.
    tan: f64,
    aperture: f64,
    focus: f64,
}

impl ThinLensCamera {
    pub fn new(lookat: Vector3<f64>, location: Vector3<f64>, up: Vector3<f64>, angle: f64,
               width: u32, height: u32, aperture: f64) -> ThinLensCamera {
        ThinLensCamera {
            placement: Placement::still(Frame::look_at(location, lookat, up)),
            aspect: width as f64 / height as f64,
            tan: angle.tan(),
            aperture,
            focus: (lookat - location).norm(),
        }
    }

    /// Focused at `distance` rather than at the point looked at.
    pub fn with_focus(self, distance: f64) -> ThinLensCamera {
        ThinLensCamera { focus: distance, ..self }
    }

    pub fn with_placement(self, placement: Placement) -> ThinLensCamera {
        ThinLensCamera { placement, ..self }
    }
}

impl Camera for ThinLensCamera {
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {
        let f = self.placement.frame();
        let xdir = f.camx * (self.aspect * (x + sx - 0.5) * self.tan);
        let ydir = f.camy * ((y + sy - 0.5) * self.tan);
        let pinhole_dest = f.camz + xdir + ydir;
        let focal_point = f.location + pinhole_dest * self.focus;
        // The lens lies in the camera's image plane.
        let point_lens = random_point_on_disc(self.aperture);
        let ro = f.location + f.camx * point_lens[0] + f.camy * point_lens[1];
        Ray { ro, rd: (focal_point - ro).normalize() }
    }
}

/// Orthographic projection (PBR section 5.2.1): parallel rays along the view direction, from a
/// rectangle `height` tall centred on the camera's position.
pub struct OrthographicCamera {
    pub placement: Placement,
    width: f64,
    height: f64,
}

impl OrthographicCamera {
    pub fn new(placement: Placement, height: f64, aspect: f64) -> OrthographicCamera {
        assert!(height > 0., "An orthographic camera's height must be positive, got {}", height);
        OrthographicCamera { placement, width: height * aspect, height }
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {
        let f = self.placement.frame();
        let ro = f.location + f.camx * ((x + sx - 0.5) * self.width) + f.camy * ((y + sy - 0.5) * self.height);
        Ray { ro, rd: f.camz }
    }
}

/// How a spherical camera lays the sphere of directions out on the image.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SphereMapping {
    /// Longitude across, latitude up: the image's centre looks at the point looked at, its left
    /// and right edges straight behind, its top and bottom straight up and down. Best at twice
    /// as wide as high.
    Equirectangular,
    /// Clarberg's equal-area mapping of the square to the sphere, as pbrt's spherical camera
    /// uses (PBR section 3.8.3): every pixel covers the same solid angle. The image's centre
    /// looks forward and its corners backward. Petrik Clarberg, "Fast Equal-Area Mapping of the
    /// (Hemi)Sphere using SIMD", Journal of Graphics Tools 13(3), 2008, pp. 53-68,
    /// doi:10.1080/2151237X.2008.10129266.
    EqualArea,
}

/// A unit vector from a point in [0, 1]²: Clarberg's equal-area mapping, z towards the
/// square's centre (PBR's `EqualAreaSquareToSphere`).
pub fn equal_area_square_to_sphere(u: f64, v: f64) -> Vector3<f64> {
    let (u, v) = (2. * u - 1., 2. * v - 1.);
    let (up, vp) = (u.abs(), v.abs());
    let signed_distance = 1. - (up + vp);
    let d = signed_distance.abs();
    let r = 1. - d;
    let phi = if r == 0. { 1. } else { (vp - up) / r + 1. } * PI / 4.;
    let z = (1. - r * r).copysign(signed_distance);
    let cos_phi = phi.cos().copysign(u);
    let sin_phi = phi.sin().copysign(v);
    let s = r * (2. - r * r).max(0.).sqrt();
    Vector3::new(cos_phi * s, sin_phi * s, z)
}

/// Every direction round a point (PBR section 5.3): a panorama.
pub struct SphericalCamera {
    pub placement: Placement,
    mapping: SphereMapping,
}

impl SphericalCamera {
    pub fn new(placement: Placement, mapping: SphereMapping) -> SphericalCamera {
        SphericalCamera { placement, mapping }
    }
}

impl Camera for SphericalCamera {
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {
        let f = self.placement.frame();
        let (u, v) = (x + sx, y + sy);
        let rd = match self.mapping {
            SphereMapping::Equirectangular => {
                let azimuth = (u - 0.5) * 2. * PI;
                let elevation = (v - 0.5) * PI;
                f.camz * (elevation.cos() * azimuth.cos()) + f.camx * (elevation.cos() * azimuth.sin()) + f.camy * elevation.sin()
            },
            SphereMapping::EqualArea => {
                let d = equal_area_square_to_sphere(u, v);
                f.camx * d.x + f.camy * d.y + f.camz * d.z
            },
        };
        Ray { ro: f.location, rd }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thin_lens(lookat: Vector3<f64>, width: u32, height: u32, aperture: f64) -> ThinLensCamera {
        ThinLensCamera::new(lookat, Vector3::zeros(), Vector3::new(0., 1., 0.), 0.5, width, height, aperture)
    }

    #[test]
    fn test_aspect_ratio_widens_horizontal_view() {
        // A 200x100 image: the horizontal half-extent of the view at unit distance is twice the
        // vertical one.
        let c = thin_lens(Vector3::new(0., 0., 1.), 200, 100, 0.);
        let extent = |r: Ray| r.rd / r.rd.z;
        let right = extent(c.get_ray(1., 0.5, 0., 0.));
        let top = extent(c.get_ray(0.5, 1., 0., 0.));
        assert!((right.x.abs() / top.y.abs() - 2.).abs() < 1e-9, "{} {}", right, top);
    }

    #[test]
    fn test_camera_is_right_handed() {
        // Looking down -z with y up: the right of the image is +x, the top is +y.
        let c = thin_lens(Vector3::new(0., 0., -1.), 100, 100, 0.);
        assert!(c.get_ray(1., 0.5, 0., 0.).rd.x > 0.);
        assert!(c.get_ray(0., 0.5, 0., 0.).rd.x < 0.);
        assert!(c.get_ray(0.5, 1., 0., 0.).rd.y > 0.);
        // Turned round to look down +z, the right of the image is -x.
        let c = thin_lens(Vector3::new(0., 0., 1.), 100, 100, 0.);
        assert!(c.get_ray(1., 0.5, 0., 0.).rd.x < 0.);
        assert!(c.get_ray(0.5, 1., 0., 0.).rd.y > 0.);
    }

    #[test]
    fn test_lens_lies_in_image_plane() {
        // Looking along +x, so a world-XY lens would offset rays along the view axis.
        let c = thin_lens(Vector3::new(10., 0., 0.), 100, 200, 1.0);
        let f = c.placement.start;
        for _ in 0..100 {
            let r = c.get_ray(0.5, 0.5, 0., 0.);
            assert!((r.ro - f.location).dot(&f.camz).abs() < 1e-9, "{}", r.ro);
            assert!((r.ro - f.location).norm() <= 1.0 + 1e-9);
        }
    }

    #[test]
    fn test_frame_rotation_round_trips() {
        let f = Frame::look_at(Vector3::new(1., 2., 3.), Vector3::new(-4., 0., 7.), Vector3::new(0., 1., 0.));
        let g = Frame::from_rotation(f.location, &f.rotation());
        assert!((f.camx - g.camx).norm() < 1e-12 && (f.camy - g.camy).norm() < 1e-12 && (f.camz - g.camz).norm() < 1e-12);
    }

    #[test]
    fn test_moving_camera_turns_with_time() {
        let start = Frame::look_at(Vector3::zeros(), Vector3::new(0., 0., 1.), Vector3::y());
        let end = Frame::look_at(Vector3::new(2., 0., 0.), Vector3::new(3., 0., 0.), Vector3::y());
        let p = Placement::moving(start, end);
        crate::motion::begin_sample(0.5);
        let f = p.frame();
        crate::motion::end_sample();
        assert!((f.location - Vector3::new(1., 0., 0.)).norm() < 1e-12);
        // Half way through the quarter turn.
        let half = std::f64::consts::FRAC_1_SQRT_2;
        assert!((f.camz - Vector3::new(half, 0., half)).norm() < 1e-12, "{}", f.camz);
        assert_eq!(p.frame(), start);
    }

    #[test]
    fn test_orthographic_rays_are_parallel() {
        let p = Placement::still(Frame::look_at(Vector3::new(0., 0., -5.), Vector3::zeros(), Vector3::y()));
        let c = OrthographicCamera::new(p, 4., 2.);
        let (a, b) = (c.get_ray(0., 0., 0., 0.), c.get_ray(1., 1., 0., 0.));
        assert_eq!(a.rd, b.rd);
        // 8 wide and 4 high; looking down +z, the image's right is -x.
        assert!(((b.ro - a.ro) - Vector3::new(-8., 4., 0.)).norm() < 1e-12, "{}", b.ro - a.ro);
    }

    #[test]
    fn test_equirectangular_camera() {
        let p = Placement::still(Frame::look_at(Vector3::zeros(), Vector3::new(0., 0., -1.), Vector3::y()));
        let c = SphericalCamera::new(p, SphereMapping::Equirectangular);
        let at = |u: f64, v: f64| c.get_ray(u, v, 0., 0.).rd;
        assert!((at(0.5, 0.5) - Vector3::new(0., 0., -1.)).norm() < 1e-12);
        assert!((at(0.5, 1.) - Vector3::y()).norm() < 1e-12);
        assert!((at(0., 0.5) - Vector3::new(0., 0., 1.)).norm() < 1e-12);
        assert!((at(0.75, 0.5) - Vector3::x()).norm() < 1e-12, "{}", at(0.75, 0.5));
    }

    /// Clarberg's mapping is area preserving: uniform points on the square are uniform on the
    /// sphere, so each octant gets an eighth of them and the mean direction is nil.
    #[test]
    fn test_equal_area_mapping_is_uniform() {
        let n = 400;
        let mut octants = [0usize; 8];
        let mut mean = Vector3::zeros();
        for i in 0..n {
            for j in 0..n {
                let d = equal_area_square_to_sphere((i as f64 + 0.5) / n as f64, (j as f64 + 0.5) / n as f64);
                assert!((d.norm() - 1.).abs() < 1e-9);
                octants[(d.x > 0.) as usize + 2 * (d.y > 0.) as usize + 4 * (d.z > 0.) as usize] += 1;
                mean += d;
            }
        }
        for o in octants {
            assert!((o as f64 / (n * n) as f64 - 0.125).abs() < 2e-3, "{:?}", octants);
        }
        assert!(mean.norm() / ((n * n) as f64) < 1e-3);
        // The square's centre is the pole the camera looks at.
        assert!((equal_area_square_to_sphere(0.5, 0.5) - Vector3::z()).norm() < 1e-12);
    }
}
