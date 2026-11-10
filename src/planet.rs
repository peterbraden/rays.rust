//! The planet the scene stands on: a sphere, with the scene's origin on its surface and y up.
//!
//! Over a few kilometres the ground is flat, but cloud layers stretch for hundreds: a layer at
//! constant height over a flat plane runs out to the horizon and beyond, where on a planet it
//! curves away below it, its distant parts dropping (d² / 2R: 800 m at 100 km) and finally
//! hidden. So density fields written in flat coordinates, (x, height, z), are evaluated on the
//! planet through `PlanetMapping`: each point in the scene is mapped to planet-local
//! coordinates, its altitude above the sphere and its position along the surface, by the
//! azimuthal equidistant projection about the point beneath the scene's origin. Distances
//! along the surface from there are kept exactly, and in any direction distortion is under 1%
//! out to 1500 km, so over a scene the fields keep their shapes while curving with the planet.

use crate::na::{Vector2, Vector3};
use crate::shapes::geometry::Geometry;
use crate::volume::DensityField;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Planet {
    pub radius: f64,
}

/// The Earth's mean radius, as the skysphere's.
pub const EARTH_RADIUS: f64 = 6_360_000.;

impl Planet {
    /// The centre: straight below the scene's origin, which is on the surface.
    pub fn center(&self) -> Vector3<f64> {
        Vector3::new(0., -self.radius, 0.)
    }

    pub fn altitude(&self, p: &Vector3<f64>) -> f64 {
        (p - self.center()).norm() - self.radius
    }

    /// Planet-local coordinates of `p`: (distance along the surface eastwards, altitude,
    /// northwards), the azimuthal equidistant projection about the origin's foot. Near the
    /// origin this is (x, y, z).
    pub fn local_coordinates(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let v = p - self.center();
        let r = v.norm();
        let horizontal = Vector2::new(v.x, v.z);
        let sideways = horizontal.norm();
        // The angle from the pole, by atan2 so it is exact near the origin.
        let theta = sideways.atan2(v.y);
        let along = if sideways > 0. { horizontal * (self.radius * theta / sideways) } else { Vector2::zeros() };
        Vector3::new(along.x, r - self.radius, along.y)
    }

    /// The inverse of `local_coordinates`.
    pub fn scene_point(&self, q: &Vector3<f64>) -> Vector3<f64> {
        let along = Vector2::new(q.x, q.z);
        let arc = along.norm();
        let theta = arc / self.radius;
        let r = self.radius + q.y;
        let dir = if arc > 0. { along / arc } else { Vector2::zeros() };
        self.center() + Vector3::new(dir.x * theta.sin() * r, theta.cos() * r, dir.y * theta.sin() * r)
    }

    /// World bounds of the planet-local box `local`: the layer it describes curves down away
    /// from the origin, and bulges slightly outwards.
    pub fn bounds_of(&self, local: &crate::shapes::bbox::BBox) -> crate::shapes::bbox::BBox {
        let reach = [local.min.x.abs(), local.max.x.abs()].iter().cloned().fold(0., f64::max)
            .hypot([local.min.z.abs(), local.max.z.abs()].iter().cloned().fold(0., f64::max));
        let theta = (reach / self.radius).min(std::f64::consts::PI);
        let low = (self.radius + local.min.y) * theta.cos() - self.radius;
        let pad = (local.max.y.max(0.) * theta).max(1.);
        crate::shapes::bbox::BBox::new(
            Vector3::new(local.min.x - pad, low.min(local.min.y) - 1., local.min.z - pad),
            Vector3::new(local.max.x + pad, local.max.y + 1., local.max.z + pad),
        )
    }
}

/// How far a local majorant through `PlanetMapping` is trusted along a straight ray: over
/// this, the ray's altitude departs from the straight line in planet-local coordinates by
/// L² / 2R, 8 cm, which the fields' bounds aren't padded for.
const CURVED_STEP: f64 = 1000.;

/// A density field written in flat coordinates, evaluated on a planet (see the module docs).
pub struct PlanetMapping {
    pub planet: Planet,
    pub field: Arc<dyn DensityField>,
}

impl DensityField for PlanetMapping {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.field.density(&self.planet.local_coordinates(p))
    }

    fn majorant(&self) -> f64 {
        self.field.majorant()
    }

    fn minorant(&self) -> f64 {
        self.field.minorant()
    }

    /// The field's local majorant along the ray's direction in planet-local coordinates, its
    /// length converted back to the scene's, and trusted only `CURVED_STEP` at a time.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let q = self.planet.local_coordinates(p);
        let eps = 1.;
        let dq = (self.planet.local_coordinates(&(p + d * eps)) - q) / eps;
        let speed = dq.norm();
        if speed <= 0. {
            return (self.majorant(), CURVED_STEP);
        }
        let (majorant, length) = self.field.majorant_from(&q, &(dq / speed));
        (majorant, (length / speed).min(CURVED_STEP))
    }

    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> {
        self.field.particle_radius(&self.planet.local_coordinates(p))
    }
}

/// The planet's surface, as scene geometry: a sphere.
pub fn ground(planet: &Planet) -> Box<dyn Geometry + Sync + Send> {
    Box::new(crate::shapes::sphere::Sphere::new(planet.center(), planet.radius))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::volume::ConstantDensity;

    #[test]
    fn test_local_coordinates() {
        let planet = Planet { radius: EARTH_RADIUS };
        // At the origin, and nearby, local coordinates are scene coordinates.
        for p in [Vector3::zeros(), Vector3::new(30., 2., -40.), Vector3::new(0., 9000., 0.)] {
            assert!((planet.local_coordinates(&p) - p).norm() < 1e-3, "{}", p);
        }
        // The ground 100 km out on a flat plane is 786 m above the planet.
        let q = planet.local_coordinates(&Vector3::new(100_000., 0., 0.));
        assert!((q.y - 100_000f64.powi(2) / (2. * EARTH_RADIUS)).abs() < 1., "{}", q);
        // Round trips, near and far.
        for q in [Vector3::new(5000., 9000., -3000.), Vector3::new(-400_000., 2500., 700_000.)] {
            assert!((planet.local_coordinates(&planet.scene_point(&q)) - q).norm() < 1e-4, "{}", q);
        }
        // Along the surface, distances from the origin are kept exactly.
        let far = planet.scene_point(&Vector3::new(0., 0., 1_000_000.));
        let angle = (far - planet.center()).normalize().dot(&Vector3::new(0., 1., 0.)).acos();
        assert!((angle * EARTH_RADIUS - 1_000_000.).abs() < 1e-3);
    }

    #[test]
    fn test_distant_clouds_are_hidden_by_the_planet() {
        // From 2 m, a cloud 1 km up is in sight out to sqrt(2R x 1000) + sqrt(2R x 2) = 118 km;
        // further away the planet's bulge is in the way.
        let planet = Planet { radius: EARTH_RADIUS };
        let eye = Vector3::new(0., 2., 0.);
        let hidden = |distance: f64| {
            let cloud = planet.scene_point(&Vector3::new(0., 1000., distance));
            let r = crate::ray::Ray { ro: eye, rd: (cloud - eye).normalize() };
            ground(&planet).intersects(&r).is_some_and(|hit| hit.dist < (cloud - eye).norm())
        };
        assert!(!hidden(80_000.) && !hidden(115_000.));
        assert!(hidden(125_000.) && hidden(300_000.));
        // The layer, mapped, is where it should be along the way: a ray to the cloud at 80 km
        // meets 1 km of altitude there.
        let field = PlanetMapping { planet, field: Arc::new(LayerAt { low: 999., high: 1001. }) };
        let cloud = planet.scene_point(&Vector3::new(0., 1000., 80_000.));
        assert!(field.density(&cloud) > 0.);
    }

    struct LayerAt { low: f64, high: f64 }
    impl DensityField for LayerAt {
        fn density(&self, p: &Vector3<f64>) -> f64 { if p.y >= self.low && p.y <= self.high { 1. } else { 0. } }
        fn majorant(&self) -> f64 { 1. }
    }

    #[test]
    fn test_bounds_contain_the_curved_layer() {
        let planet = Planet { radius: EARTH_RADIUS };
        let local = crate::shapes::bbox::BBox::new(Vector3::new(-100_000., 8000., -100_000.), Vector3::new(100_000., 10_000., 100_000.));
        let world = planet.bounds_of(&local);
        for x in [-100_000., 0., 100_000.] {
            for z in [-100_000., 0., 100_000.] {
                for y in [8000., 10_000.] {
                    assert!(world.contains_point(&planet.scene_point(&Vector3::new(x, y, z))), "{} {} {}", x, y, z);
                }
            }
        }
        let _ = ConstantDensity(1.);
    }
}

/// A point in the atmosphere of a planet, for fields defined over the whole of it.
#[derive(Debug, Clone, Copy)]
pub struct SurfacePoint {
    /// Unit vector from the planet's centre: the point's "up", and where it is on the globe.
    pub up: Vector3<f64>,
    pub altitude: f64,
    /// Position relative to the planet's centre (up × (radius + altitude)): for solid noise,
    /// which has no poles or seams anywhere.
    pub position: Vector3<f64>,
}

impl Planet {
    pub fn surface_point(&self, p: &Vector3<f64>) -> SurfacePoint {
        let position = p - self.center();
        let r = position.norm();
        SurfacePoint { up: position / r, altitude: r - self.radius, position }
    }
}

/// A density field over a whole planet, evaluated at `SurfacePoint`s rather than flat
/// coordinates (see `GlobalMapping`).
pub trait PlanetField: Sync + Send {
    fn density(&self, at: &SurfacePoint) -> f64;

    fn majorant(&self) -> f64;

    /// As `DensityField::majorant_from`, for the scene point `p` (also given as `at`) and unit
    /// direction `d`, with the planet for geometry. By default, `majorant()` everywhere.
    fn majorant_from(&self, _at: &SurfacePoint, _p: &Vector3<f64>, _d: &Vector3<f64>, _planet: &Planet) -> (f64, f64) {
        (self.majorant(), f64::INFINITY)
    }

    fn particle_radius(&self, _at: &SurfacePoint) -> Option<f64> {
        None
    }
}

/// A `PlanetField` as the renderer's `DensityField`: the whole planet, no projection, so no
/// poles, seams or distortion anywhere, for scenes that see more of it than a region (from
/// orbit, say).
pub struct GlobalMapping {
    pub planet: Planet,
    pub field: Arc<dyn PlanetField>,
}

impl DensityField for GlobalMapping {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.field.density(&self.planet.surface_point(p))
    }

    fn majorant(&self) -> f64 {
        self.field.majorant()
    }

    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        self.field.majorant_from(&self.planet.surface_point(p), p, d, &self.planet)
    }

    fn particle_radius(&self, p: &Vector3<f64>) -> Option<f64> {
        self.field.particle_radius(&self.planet.surface_point(p))
    }
}

/// The distance along a ray from `o` (relative to the planet's centre) in unit direction `d`
/// to where it next crosses the sphere of radius `r` about the centre, beyond `0`; infinite if
/// it never does.
pub fn to_sphere(o: &Vector3<f64>, d: &Vector3<f64>, r: f64) -> f64 {
    let b = o.dot(d);
    let c = o.norm_squared() - r * r;
    let disc = b * b - c;
    if disc < 0. {
        return f64::INFINITY;
    }
    let s = disc.sqrt();
    let (t0, t1) = (-b - s, -b + s);
    if t0 > 1e-9 { t0 } else if t1 > 1e-9 { t1 } else { f64::INFINITY }
}

/// # Equi-angular cube-sphere
///
/// Maps over a sphere without poles: project a cube's six faces onto it. Plainly (gnomonic),
/// texels at a face's corners cover a fifth the solid angle of those at its centre; spacing
/// them evenly in angle, a = atan(x / major) rather than x / major, brings that to 1.4 (as
/// Google's "equi-angular cubemaps", and planet renderers, do). Each face's texel boundaries,
/// lines of constant a, are planes through the sphere's centre, so a ray's exit from a texel's
/// column of the atmosphere is where it meets one of four planes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubeSphere {
    /// Texels along a face's side.
    pub size: usize,
}

/// A face's axes: (major, u, v), right-handed.
fn face_axes(face: usize) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
    let (x, y, z) = (Vector3::x(), Vector3::y(), Vector3::z());
    match face {
        0 => (x, -z, y),
        1 => (-x, z, y),
        2 => (y, x, -z),
        3 => (-y, x, z),
        4 => (z, x, y),
        _ => (-z, -x, y),
    }
}

const QUARTER: f64 = std::f64::consts::FRAC_PI_4;

impl CubeSphere {
    /// The face a unit direction is on, and its texel coordinates there, each in [0, size].
    pub fn locate(&self, d: &Vector3<f64>) -> (usize, f64, f64) {
        let a = d.abs();
        let face = if a.x >= a.y && a.x >= a.z {
            if d.x > 0. { 0 } else { 1 }
        } else if a.y >= a.z {
            if d.y > 0. { 2 } else { 3 }
        } else if d.z > 0. { 4 } else { 5 };
        let (major, u, v) = face_axes(face);
        let m = d.dot(&major);
        let to_texel = |x: f64| ((x / m).atan() / QUARTER + 1.) / 2. * self.size as f64;
        (face, to_texel(d.dot(&u)).clamp(0., self.size as f64), to_texel(d.dot(&v)).clamp(0., self.size as f64))
    }

    /// The unit direction at texel coordinates (s, t) on `face`.
    pub fn direction(&self, face: usize, s: f64, t: f64) -> Vector3<f64> {
        let (major, u, v) = face_axes(face);
        let angle = |x: f64| ((x / self.size as f64) * 2. - 1.) * QUARTER;
        (major + u * angle(s).tan() + v * angle(t).tan()).normalize()
    }

    /// The normal of the plane through the centre on which texel coordinate `s` (along u) is
    /// constant, on `face`; or `t` (along v) if `along_v`.
    fn boundary(&self, face: usize, x: f64, along_v: bool) -> Vector3<f64> {
        let (major, u, v) = face_axes(face);
        let tan = (((x / self.size as f64) * 2. - 1.) * QUARTER).tan();
        if along_v { (major + v * tan).cross(&u) } else { (major + u * tan).cross(&v) }
    }

    /// How far a ray from `o` (relative to the centre) in unit direction `d` can go before
    /// leaving the column above the texel (`face`, `i`, `j`) it starts in.
    pub fn exit(&self, o: &Vector3<f64>, d: &Vector3<f64>, face: usize, i: usize, j: usize) -> f64 {
        let mut best = f64::INFINITY;
        for (x, along_v) in [(i as f64, false), ((i + 1) as f64, false), (j as f64, true), ((j + 1) as f64, true)] {
            let n = self.boundary(face, x, along_v);
            let denom = n.dot(d);
            if denom.abs() > 1e-15 {
                let t = -n.dot(o) / denom;
                if t > 1e-9 {
                    best = best.min(t);
                }
            }
        }
        best
    }
}

/// Values over a sphere on an equi-angular cube-sphere: a continuous function of direction
/// sampled at every texel's corners, interpolated bilinearly within a face. A corner on an edge
/// between faces is the same direction from either, so the map is continuous across them, and
/// as in a flat map the most a texel reaches is its largest corner.
pub struct CubeMap {
    pub cube: CubeSphere,
    corners: Vec<f32>,
    texel_max: Vec<f32>,
}

impl CubeMap {
    pub fn new(size: usize, f: impl Fn(&Vector3<f64>) -> f64 + Sync) -> CubeMap {
        use rayon::prelude::*;
        let cube = CubeSphere { size };
        let n = size + 1;
        let corners: Vec<f32> = (0..6 * n * n).into_par_iter().map(|k| {
            let (face, j, i) = (k / (n * n), (k / n) % n, k % n);
            f(&cube.direction(face, i as f64, j as f64)) as f32
        }).collect();
        let texel_max = (0..6 * size * size).map(|k| {
            let (face, j, i) = (k / (size * size), (k / size) % size, k % size);
            let at = |i: usize, j: usize| corners[face * n * n + j * n + i];
            at(i, j).max(at(i + 1, j)).max(at(i, j + 1)).max(at(i + 1, j + 1))
        }).collect();
        CubeMap { cube, corners, texel_max }
    }

    /// The face and texel a unit direction is in.
    pub fn texel(&self, d: &Vector3<f64>) -> (usize, usize, usize) {
        let (face, s, t) = self.cube.locate(d);
        (face, (s as usize).min(self.cube.size - 1), (t as usize).min(self.cube.size - 1))
    }

    pub fn at(&self, d: &Vector3<f64>) -> f64 {
        let (face, s, t) = self.cube.locate(d);
        let (i, j) = ((s as usize).min(self.cube.size - 1), (t as usize).min(self.cube.size - 1));
        let (fs, ft) = (s - i as f64, t - j as f64);
        let n = self.cube.size + 1;
        let c = |i: usize, j: usize| self.corners[face * n * n + j * n + i] as f64;
        (c(i, j) * (1. - fs) + c(i + 1, j) * fs) * (1. - ft) + (c(i, j + 1) * (1. - fs) + c(i + 1, j + 1) * fs) * ft
    }

    /// The most the map reaches anywhere over the texel (`face`, `i`, `j`).
    pub fn texel_max(&self, face: usize, i: usize, j: usize) -> f64 {
        self.texel_max[face * self.cube.size * self.cube.size + j * self.cube.size + i] as f64
    }
}

#[cfg(test)]
mod cube_tests {
    use super::*;

    fn random_direction(rng: &mut crate::geometry::Pcg32) -> Vector3<f64> {
        crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64())
    }

    #[test]
    fn test_cube_sphere_round_trip() {
        let cube = CubeSphere { size: 64 };
        let mut rng = crate::geometry::Pcg32::new(1, 0);
        for _ in 0..5000 {
            let d = random_direction(&mut rng);
            let (face, s, t) = cube.locate(&d);
            assert!((cube.direction(face, s, t) - d).norm() < 1e-9, "{}", d);
        }
    }

    #[test]
    fn test_equi_angular_texels_are_nearly_equal() {
        // Solid angle of texels at a face's centre and corner: within 1.5 of each other.
        let cube = CubeSphere { size: 64 };
        let area = |i: usize, j: usize| {
            let (a, b, c) = (cube.direction(0, i as f64, j as f64), cube.direction(0, (i + 1) as f64, j as f64), cube.direction(0, i as f64, (j + 1) as f64));
            (b - a).cross(&(c - a)).norm()
        };
        let ratio = area(32, 32) / area(0, 0);
        assert!(ratio > 1. && ratio < 1.5, "{}", ratio);
    }

    #[test]
    fn test_cube_map_is_continuous_and_bounded() {
        // A smooth function: the map matches it at corners, varies continuously across face
        // edges, and never exceeds its texel's bound.
        let f = |d: &Vector3<f64>| (3. * d.x).sin() + d.y * d.z + 0.5 * (5. * d.z).cos();
        let map = CubeMap::new(32, f);
        let mut rng = crate::geometry::Pcg32::new(2, 0);
        for _ in 0..20000 {
            let d = random_direction(&mut rng);
            let (face, i, j) = map.texel(&d);
            assert!(map.at(&d) <= map.texel_max(face, i, j) + 1e-6);
            assert!((map.at(&d) - f(&d)).abs() < 0.02, "{}: {} vs {}", d, map.at(&d), f(&d));
        }
        // Either side of an edge between two faces.
        for k in 0..100 {
            let t = -0.9 + 1.8 * k as f64 / 100.;
            let edge = Vector3::new(1., 1., t).normalize();
            let (a, b) = (edge + Vector3::new(1e-9, -1e-9, 0.), edge + Vector3::new(-1e-9, 1e-9, 0.));
            assert_ne!(map.cube.locate(&a.normalize()).0, map.cube.locate(&b.normalize()).0);
            assert!((map.at(&a.normalize()) - map.at(&b.normalize())).abs() < 1e-6);
        }
    }

    #[test]
    fn test_texel_exit() {
        // Just before the exit distance the ray is in the texel it started in; just after, not.
        let cube = CubeSphere { size: 16 };
        let mut rng = crate::geometry::Pcg32::new(3, 0);
        let mut checked = 0;
        for _ in 0..2000 {
            let o = random_direction(&mut rng) * 6.4e6;
            let d = random_direction(&mut rng);
            let (face, s, t) = cube.locate(&o.normalize());
            let (i, j) = (s as usize, t as usize);
            let exit = cube.exit(&o, &d, face, i, j);
            if !exit.is_finite() || exit > 1e7 {
                continue;
            }
            let texel = |p: Vector3<f64>| { let (f, s, t) = cube.locate(&p.normalize()); (f, s as usize, t as usize) };
            assert_eq!(texel(o + d * (exit * 0.999)), (face, i, j));
            assert_ne!(texel(o + d * (exit * 1.001 + 1.)), (face, i, j));
            checked += 1;
        }
        assert!(checked > 1000);
    }
}
