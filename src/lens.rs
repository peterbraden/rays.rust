//! # Lens systems and the realistic camera
//!
//! A real camera lens is a stack of glass elements with spherical surfaces, and an aperture
//! stop among them. Tracing rays through it, surface by surface, gives what a thin lens can't:
//! the lens's own field of view and focus breathing, vignetting, distortion and its
//! aberrations (spherical aberration, coma, astigmatism, field curvature), and the light
//! reflected between its surfaces that makes lens flare (`flare`). Craig Kolb, Don Mitchell and
//! Pat Hanrahan, "A Realistic Camera Model for Computer Graphics", SIGGRAPH 1995, pp. 317-324,
//! doi:10.1145/218380.218463; Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th
//! ed., section 5.3 "Realistic Cameras" (cited below as PBR), which this follows closely.
//!
//! ## The lens description
//!
//! A lens is given as rows of `radius thickness index aperture`, in millimetres, from the
//! scene side to the film: each surface's radius of curvature (positive when its centre lies
//! towards the film; zero for the aperture stop, which is flat), the distance along the axis to
//! the next surface, the refractive index of the glass behind it (towards the film; 1 for air,
//! 0 for the stop) and its clear aperture's diameter. The built-in lenses are those pbrt ships,
//! from published designs:
//!
//! - `dgauss.50mm`: double Gauss, f/2, after Tronnier, US patent 2,673,491; Warren J. Smith,
//!   "Modern Lens Design", p. 312.
//! - `wide.22mm`: wide-angle, 38°, Nakamura; "Modern Lens Design", p. 360.
//! - `telephoto.250mm`: Sigler super achromat telephoto, f/5.6; "Modern Lens Design", p. 175.
//! - `fisheye.10mm`: Muller 16 mm f/4 fisheye, 155.9°; "Modern Lens Design", p. 164.
//!
//! (each scaled by pbrt from the book's 100 mm focal length).
//!
//! ## Coordinates
//!
//! In lens space, as in pbrt, the film is at z = 0 and the lens extends towards -z, its rear
//! surface `rear_z` from the film and its front `front_z`. Camera space mirrors it in z, so the
//! camera looks along +z. Lengths are in metres, the scene's units.
//!
//! ## Focusing (PBR section 5.3.3)
//!
//! A ray parallel to the axis entering the lens crosses the axis at the focal point, and where
//! its entering and leaving lines meet is the principal plane. Finding both from each side
//! gives the lens's thick-lens approximation, from which the distance the film must move to
//! focus at a given distance follows in closed form.
//!
//! ## Sampling the exit pupil (PBR section 5.3.4)
//!
//! Most rays from a point on the film to the rear element are stopped by the aperture or by
//! other elements; aiming them at the rear element alone would waste most samples away from the
//! axis. The exit pupil, the region of the rear element's plane through which light gets
//! through, is bounded for points along the film's x axis at a range of distances (by tracing
//! many rays), and a sample rotated round to its film point is aimed within those bounds.
//!
//! ## Exposure
//!
//! The irradiance on the film from a ray towards the pupil is weighted by cos⁴θ times the
//! pupil's area over the rear element's distance squared (the camera measurement equation, PBR
//! section 5.4.1): the cos⁴ law and the lens's vignetting darken the corners. The weight is
//! divided by its value at the film's centre with the aperture wide open, so a lens wide open
//! is as bright as the pinhole camera, and stopping down by an f-number N darkens the image
//! by (N_open / N)², as on a real camera.
use crate::camera::{Camera, Placement};
use crate::na::{Vector2, Vector3};
use crate::ray::Ray;
use rayon::prelude::*;

/// One surface of a lens system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interface {
    /// Radius of curvature (metres); 0 for the aperture stop.
    pub radius: f64,
    /// Distance along the axis to the next surface towards the film.
    pub thickness: f64,
    /// Refractive index of the medium behind the surface, towards the film; 0 for the stop.
    pub eta: f64,
    /// Radius of the clear aperture.
    pub aperture: f64,
}

impl Interface {
    pub fn is_stop(&self) -> bool {
        self.radius == 0.
    }
}

/// A lens system: its surfaces from the scene side to the film.
#[derive(Debug, Clone, PartialEq)]
pub struct LensSystem {
    pub interfaces: Vec<Interface>,
}

const DGAUSS_50MM: &str = "
29.475	3.76	1.67	25.2
84.83	0.12	1	25.2
19.275	4.025	1.67	23
40.77	3.275	1.699	23
12.75	5.705	1	18
0	4.5	0	17.1
-14.495	1.18	1.603	17
40.77	6.065	1.658	20
-20.385	0.19	1	20
437.065	3.22	1.717	20
-39.73	0	1	20";

const WIDE_22MM: &str = "
35.98738	1.21638	1.54	23.716
11.69718	9.9957	1	17.996
13.08714	5.12622	1.772	12.364
-22.63294	1.76924	1.617	9.812
71.05802	0.8184	1	9.152
0	2.27766	0	8.756
-9.58584	2.43254	1.617	8.184
-11.28864	0.11506	1	9.152
-166.7765	3.09606	1.713	10.648
-7.5911	1.32682	1.805	11.44
-16.7662	3.98068	1	12.276
-7.70286	1.21638	1.617	13.42
-11.97328	0	1	17.996";

const TELEPHOTO_250MM: &str = "
54.6275	12.52	1.529	47.5
-86.365	3.755	1.599	44.5
271.7625	2.8175	1	41.5
0	67.4125	0	40.5
-32.13	3.755	1.613	31.5
49.5325	12.52	1.603	33.5
-50.945	0	1	37";

const FISHEYE_10MM: &str = "
30.2249	0.8335	1.62	30.34
11.3931	7.4136	1	20.68
75.2019	1.0654	1.639	17.8
8.3349	11.1549	1	13.42
9.5882	2.0054	1.654	9.02
43.8677	5.3895	1	8.14
0	1.4163	0	6.08
29.4541	2.1934	1.517	5.96
-5.2265	0.9714	1.805	5.84
-14.2884	0.0627	1	5.96
-22.3726	0.94	1.673	5.96
-15.0404	0	1	6.52";

/// Refract the unit direction `wi` (pointing away from the surface, back along the incident
/// ray) through a surface of normal `n`, with `eta` the ratio of the index on the far side to
/// the near side; None at total internal reflection (PBR section 9.3, `Refract`).
pub fn refract(wi: &Vector3<f64>, n: &Vector3<f64>, eta: f64) -> Option<Vector3<f64>> {
    let mut n = *n;
    let mut eta = eta;
    let mut cos_i = n.dot(wi);
    if cos_i < 0. {
        eta = 1. / eta;
        cos_i = -cos_i;
        n = -n;
    }
    let sin2_i = (1. - cos_i * cos_i).max(0.);
    let sin2_t = sin2_i / (eta * eta);
    if sin2_t >= 1. {
        return None;
    }
    let cos_t = (1. - sin2_t).sqrt();
    Some(-wi / eta + n * (cos_i / eta - cos_t))
}

/// Where a ray meets a spherical surface of `radius` centred on the axis at `z_center`, and the
/// surface's normal there facing back along the ray (PBR's `IntersectSphericalElement`): of the
/// two hits, the one on the part of the sphere that is the lens surface.
pub fn intersect_spherical(radius: f64, z_center: f64, o: &Vector3<f64>, d: &Vector3<f64>) -> Option<(f64, Vector3<f64>)> {
    let oc = o - Vector3::new(0., 0., z_center);
    let a = d.norm_squared();
    let b = 2. * d.dot(&oc);
    let c = oc.norm_squared() - radius * radius;
    let disc = b * b - 4. * a * c;
    if disc < 0. {
        return None;
    }
    let root = disc.sqrt();
    let q = if b < 0. { -0.5 * (b - root) } else { -0.5 * (b + root) };
    let (mut t0, mut t1) = (q / a, c / q);
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }
    let closer = (d.z > 0.) ^ (radius < 0.);
    let t = if closer { t0 } else { t1 };
    if t < 0. || !t.is_finite() {
        return None;
    }
    let mut n = (oc + d * t).normalize();
    if n.dot(&-d) < 0. {
        n = -n;
    }
    Some((t, n))
}

impl LensSystem {
    /// A built-in lens by name, or None.
    pub fn named(name: &str) -> Option<LensSystem> {
        let text = match name {
            "dgauss.50mm" => DGAUSS_50MM,
            "wide.22mm" => WIDE_22MM,
            "telephoto.250mm" => TELEPHOTO_250MM,
            "fisheye.10mm" => FISHEYE_10MM,
            _ => return None,
        };
        Some(LensSystem::parse(text).unwrap())
    }

    /// A lens in pbrt's text format: rows of radius, thickness, index and aperture diameter in
    /// millimetres; `#` starts a comment.
    pub fn parse(text: &str) -> Result<LensSystem, String> {
        let mut interfaces = Vec::new();
        for line in text.lines() {
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let v: Vec<f64> = line.split_whitespace().map(|x| x.parse::<f64>().map_err(|e| format!("{}: {}", line, e)))
                .collect::<Result<_, _>>()?;
            if v.len() != 4 {
                return Err(format!("A lens row has four numbers (radius, thickness, index, aperture), got \"{}\"", line));
            }
            interfaces.push(Interface { radius: v[0] * 1e-3, thickness: v[1] * 1e-3, eta: v[2], aperture: v[3] * 1e-3 / 2. });
        }
        if interfaces.is_empty() {
            return Err("A lens needs at least one surface".to_string());
        }
        Ok(LensSystem { interfaces })
    }

    /// The rear surface's distance from the film.
    pub fn rear_z(&self) -> f64 {
        self.interfaces.last().unwrap().thickness
    }

    /// The front surface's distance from the film.
    pub fn front_z(&self) -> f64 {
        self.interfaces.iter().map(|i| i.thickness).sum()
    }

    pub fn rear_radius(&self) -> f64 {
        self.interfaces.last().unwrap().aperture
    }

    /// The index of the medium in front of surface `i` (towards the scene).
    fn eta_before(&self, i: usize) -> f64 {
        if i == 0 || self.interfaces[i - 1].eta == 0. { 1. } else { self.interfaces[i - 1].eta }
    }

    fn eta_after(&self, i: usize) -> f64 {
        if self.interfaces[i].eta == 0. { 1. } else { self.interfaces[i].eta }
    }

    /// Trace a ray in camera space (z towards the scene) from the film out through the lens
    /// (PBR's `TraceLensesFromFilm`): the ray leaving the front element, in camera space, or
    /// None if the lens stops it.
    pub fn trace_from_film(&self, o: &Vector3<f64>, d: &Vector3<f64>) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let (mut o, mut d) = (Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z));
        let mut z = 0.;
        for i in (0..self.interfaces.len()).rev() {
            let e = &self.interfaces[i];
            z -= e.thickness;
            let (t, n) = if e.is_stop() {
                if d.z >= 0. {
                    return None;
                }
                ((z - o.z) / d.z, Vector3::zeros())
            } else {
                intersect_spherical(e.radius, z + e.radius, &o, &d)?
            };
            let p = o + d * t;
            if p.x * p.x + p.y * p.y > e.aperture * e.aperture {
                return None;
            }
            o = p;
            if !e.is_stop() {
                d = refract(&-d.normalize(), &n, self.eta_before(i) / self.eta_after(i))?;
            }
        }
        Some((Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z)))
    }

    /// Trace a ray in camera space from the scene in through the lens towards the film (PBR's
    /// `TraceLensesFromScene`).
    pub fn trace_from_scene(&self, o: &Vector3<f64>, d: &Vector3<f64>) -> Option<(Vector3<f64>, Vector3<f64>)> {
        let (mut o, mut d) = (Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z));
        let mut z = -self.front_z();
        for (i, e) in self.interfaces.iter().enumerate() {
            let (t, n) = if e.is_stop() {
                ((z - o.z) / d.z, Vector3::zeros())
            } else {
                intersect_spherical(e.radius, z + e.radius, &o, &d)?
            };
            let p = o + d * t;
            if p.x * p.x + p.y * p.y > e.aperture * e.aperture {
                return None;
            }
            o = p;
            if !e.is_stop() {
                d = refract(&-d.normalize(), &n, self.eta_after(i) / self.eta_before(i))?;
            }
            z += e.thickness;
        }
        Some((Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z)))
    }

    /// The principal plane and focal point (z in lens space, so negative) for a ray entering
    /// parallel to the axis and leaving as `out`, both in camera space (PBR's
    /// `ComputeCardinalPoints`).
    fn cardinal_points(in_o: &Vector3<f64>, out: &(Vector3<f64>, Vector3<f64>)) -> (f64, f64) {
        let (o, d) = out;
        let tf = -o.x / d.x;
        let fz = -(o.z + d.z * tf);
        let tp = (in_o.x - o.x) / d.x;
        let pz = -(o.z + d.z * tp);
        (pz, fz)
    }

    /// The thick-lens approximation: principal planes and focal points on the film side and the
    /// scene side (PBR's `ComputeThickLensApproximation`).
    pub fn thick_lens(&self) -> Option<([f64; 2], [f64; 2])> {
        let x = 0.001 * self.rear_radius().max(1e-3);
        let scene_o = Vector3::new(x, 0., self.front_z() + 1.);
        let film_side = self.trace_from_scene(&scene_o, &Vector3::new(0., 0., -1.))?;
        let (pz0, fz0) = LensSystem::cardinal_points(&scene_o, &film_side);
        let film_o = Vector3::new(x, 0., self.rear_z() - 1.);
        let scene_side = self.trace_from_film(&film_o, &Vector3::new(0., 0., 1.))?;
        let (pz1, fz1) = LensSystem::cardinal_points(&film_o, &scene_side);
        Some(([pz0, pz1], [fz0, fz1]))
    }

    /// The effective focal length.
    pub fn focal_length(&self) -> Option<f64> {
        let (pz, fz) = self.thick_lens()?;
        Some(fz[0] - pz[0])
    }

    /// The rear element's distance from the film that focuses at `distance` in front of the
    /// film (PBR's `FocusThickLens`), or None if the lens can't focus that close.
    pub fn focus(&self, distance: f64) -> Option<f64> {
        let (pz, fz) = self.thick_lens()?;
        let f = fz[0] - pz[0];
        let z = -distance;
        let c = (pz[1] - z - pz[0]) * (pz[1] - z - 4. * f - pz[0]);
        if c <= 0. {
            return None;
        }
        let delta = 0.5 * (pz[1] - z + pz[0] - c.sqrt());
        Some(self.rear_z() + delta)
    }

    /// With the stop's diameter set to `diameter` (clamped to the stop's own).
    pub fn with_stop(mut self, diameter: f64) -> LensSystem {
        for e in self.interfaces.iter_mut().filter(|e| e.is_stop()) {
            e.aperture = e.aperture.min(diameter / 2.);
        }
        self
    }

    /// The stop's diameter.
    pub fn stop_diameter(&self) -> Option<f64> {
        self.interfaces.iter().find(|e| e.is_stop()).map(|e| 2. * e.aperture)
    }

    /// The film at `rear_z` from the rear element.
    pub fn with_film_at(mut self, rear_z: f64) -> LensSystem {
        self.interfaces.last_mut().unwrap().thickness = rear_z;
        self
    }
}

/// The exit pupil's bounds in the rear element's plane, for film points along +x at a range of
/// distances from the centre (see the module's notes).
struct ExitPupil {
    bounds: Vec<Option<(Vector2<f64>, Vector2<f64>)>>,
    film_radius: f64,
}

const PUPIL_SEGMENTS: usize = 64;
const PUPIL_SAMPLES: usize = 1 << 16;

/// The `i`th point of the Hammersley-like (radical inverse) sequence in base `b`.
fn radical_inverse(b: u64, mut i: u64) -> f64 {
    let inv = 1. / b as f64;
    let (mut r, mut f) = (0., inv);
    while i > 0 {
        r += (i % b) as f64 * f;
        i /= b;
        f *= inv;
    }
    r
}

impl ExitPupil {
    fn new(lens: &LensSystem, film_radius: f64) -> ExitPupil {
        let rear = lens.rear_radius() * 1.5;
        let z = lens.rear_z();
        let bounds = (0..PUPIL_SEGMENTS).into_par_iter().map(|s| {
            let (x0, x1) = (s as f64 / PUPIL_SEGMENTS as f64 * film_radius, (s + 1) as f64 / PUPIL_SEGMENTS as f64 * film_radius);
            let mut b: Option<(Vector2<f64>, Vector2<f64>)> = None;
            for i in 0..PUPIL_SAMPLES {
                let film = Vector3::new(x0 + (x1 - x0) * (i as f64 + 0.5) / PUPIL_SAMPLES as f64, 0., 0.);
                let p = Vector2::new(-rear + 2. * rear * radical_inverse(2, i as u64), -rear + 2. * rear * radical_inverse(3, i as u64));
                let inside = b.is_some_and(|(lo, hi)| p.x >= lo.x && p.y >= lo.y && p.x <= hi.x && p.y <= hi.y);
                if inside || lens.trace_from_film(&film, &(Vector3::new(p.x, p.y, z) - film)).is_some() {
                    b = Some(match b {
                        None => (p, p),
                        Some((lo, hi)) => (lo.inf(&p), hi.sup(&p)),
                    });
                }
            }
            // Widen by the samples' spacing.
            let pad = 2. * (2. * rear * std::f64::consts::SQRT_2) / (PUPIL_SAMPLES as f64).sqrt();
            b.map(|(lo, hi)| (lo - Vector2::repeat(pad), hi + Vector2::repeat(pad)))
        }).collect();
        ExitPupil { bounds, film_radius }
    }

    /// A point on the rear element's plane towards the pupil from film point (x, y), for uniform
    /// `u`, and the area of the bounds it was drawn from; None if no light gets through.
    fn sample(&self, x: f64, y: f64, u: [f64; 2], z: f64) -> Option<(Vector3<f64>, f64)> {
        let r = (x * x + y * y).sqrt();
        let i = ((r / self.film_radius * PUPIL_SEGMENTS as f64) as usize).min(PUPIL_SEGMENTS - 1);
        let (lo, hi) = self.bounds[i]?;
        let p = Vector2::new(lo.x + (hi.x - lo.x) * u[0], lo.y + (hi.y - lo.y) * u[1]);
        let (sin, cos) = if r > 0. { (y / r, x / r) } else { (0., 1.) };
        Some((Vector3::new(cos * p.x - sin * p.y, sin * p.x + cos * p.y, z), (hi.x - lo.x) * (hi.y - lo.y)))
    }
}

/// A camera that traces each ray through a lens system (see the module's notes).
pub struct RealisticCamera {
    pub placement: Placement,
    pub lens: LensSystem,
    /// The film's width and height, metres.
    film: (f64, f64),
    pupil: ExitPupil,
    /// One over the weight at the film's centre with the aperture wide open.
    normalisation: f64,
}

impl RealisticCamera {
    /// `lens` focused at `focus_distance` (metres from the film), with its stop at f-number
    /// `f_number` if given (no wider than the lens allows), and a film of diagonal `diagonal`
    /// metres at the image's aspect ratio.
    pub fn new(placement: Placement, lens: LensSystem, focus_distance: f64, f_number: Option<f64>, diagonal: f64, aspect: f64) -> Result<RealisticCamera, String> {
        let open = lens.clone();
        let focal = open.focal_length().ok_or("the lens doesn't form an image")?;
        let lens = match f_number {
            Some(n) => {
                assert!(n > 0., "An f-number must be positive, got {}", n);
                lens.with_stop(focal / n)
            },
            None => lens,
        };
        let rear = lens.focus(focus_distance)
            .ok_or_else(|| format!("the lens can't focus as close as {} m", focus_distance))?;
        let lens = lens.with_film_at(rear);
        let open = open.with_film_at(rear);
        let h = diagonal / (1. + aspect * aspect).sqrt();
        let film = (h * aspect, h);
        let film_radius = diagonal / 2.;
        // The centre's weight wide open, for normalisation: the mean of the weight over the
        // open pupil's bounds.
        let open_pupil = ExitPupil::new(&open, film_radius);
        let n = 4096;
        let centre = (0..n).map(|i| {
            let u = [radical_inverse(2, i), radical_inverse(3, i)];
            RealisticCamera::weight_through(&open, &open_pupil, 0., 0., u).map_or(0., |(_, _, w)| w)
        }).sum::<f64>() / n as f64;
        if centre <= 0. {
            return Err("no light reaches the centre of the film".to_string());
        }
        let pupil = ExitPupil::new(&lens, film_radius);
        Ok(RealisticCamera { placement, lens, film, pupil, normalisation: 1. / centre })
    }

    /// The ray (camera space) from film point (x, y) towards the pupil for `u`, and its weight,
    /// cos⁴θ A / z²; None if the lens stops it.
    fn weight_through(lens: &LensSystem, pupil: &ExitPupil, x: f64, y: f64, u: [f64; 2]) -> Option<(Vector3<f64>, Vector3<f64>, f64)> {
        let film = Vector3::new(x, y, 0.);
        let (rear, area) = pupil.sample(x, y, u, lens.rear_z())?;
        let d = rear - film;
        let (o, out) = lens.trace_from_film(&film, &d)?;
        let cos = d.normalize().z;
        Some((o, out, cos.powi(4) * area / (lens.rear_z() * lens.rear_z())))
    }
}

impl Camera for RealisticCamera {
    fn get_ray(&self, x: f64, y: f64, sx: f64, sy: f64) -> Ray {
        self.get_ray_weighted(x, y, sx, sy).map_or(Ray { ro: self.placement.frame().location, rd: self.placement.frame().camz }, |(r, _)| r)
    }

    fn get_ray_weighted(&self, x: f64, y: f64, sx: f64, sy: f64) -> Option<(Ray, f64)> {
        let f = self.placement.frame();
        // The lens inverts the image: the image's top right is the film's bottom left.
        let (fx, fy) = (-(x + sx - 0.5) * self.film.0, -(y + sy - 0.5) * self.film.1);
        let u = [crate::geometry::rand(), crate::geometry::rand()];
        let (o, d, w) = RealisticCamera::weight_through(&self.lens, &self.pupil, fx, fy, u)?;
        let world = |v: Vector3<f64>| f.camx * v.x + f.camy * v.y + f.camz * v.z;
        Some((Ray { ro: f.location + world(o), rd: world(d).normalize() }, w * self.normalisation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Frame;

    #[test]
    fn test_built_in_lenses_focal_lengths() {
        // Each about its name's focal length.
        for (name, f) in [("dgauss.50mm", 50.), ("wide.22mm", 22.), ("telephoto.250mm", 250.), ("fisheye.10mm", 10.)] {
            let lens = LensSystem::named(name).unwrap();
            let focal = lens.focal_length().unwrap() * 1e3;
            assert!((focal / f - 1.).abs() < 0.12, "{}: {} mm", name, focal);
        }
        assert!(LensSystem::named("pinhole.0mm").is_none());
        assert!(LensSystem::parse("1 2 3").is_err());
    }

    /// Rays from a point in the scene, focused on, converge to one point on the film (as near as
    /// the lens's aberrations allow), and the image is inverted.
    #[test]
    fn test_focused_lens_images_a_point() {
        let lens = LensSystem::named("dgauss.50mm").unwrap().with_stop(10e-3);
        let distance = 3.;
        let rear = lens.focus(distance).unwrap();
        let lens = lens.with_film_at(rear);
        // A point 3 m in front of the film, a little off axis.
        let p = Vector3::new(0.1, 0., distance);
        let mut hits = vec![];
        for i in 0..200 {
            let a = Vector3::new(-0.003 + 0.006 * radical_inverse(2, i), -0.003 + 0.006 * radical_inverse(3, i), lens.front_z());
            if let Some((o, d)) = lens.trace_from_scene(&p, &(a - p)) {
                let t = -o.z / d.z;
                hits.push(o + d * t);
            }
        }
        assert!(hits.len() > 100, "{} rays got through", hits.len());
        let mean = hits.iter().sum::<Vector3<f64>>() / hits.len() as f64;
        let spread = hits.iter().map(|h| (h - mean).norm()).fold(0., f64::max);
        assert!(spread < 3e-5, "image of a point {} m across", spread);
        // Inverted: a point to +x images to -x, at about f / (d - f) times its offset.
        assert!(mean.x < 0. && (mean.x / -0.1 - 0.05 / 2.95).abs() < 0.003, "{}", mean);
    }

    #[test]
    fn test_realistic_camera_looks_forward_and_vignettes() {
        let placement = Placement::still(Frame::look_at(Vector3::zeros(), Vector3::new(0., 0., 1.), Vector3::y()));
        let cam = RealisticCamera::new(placement, LensSystem::named("dgauss.50mm").unwrap(), 5., None, 0.035, 1.5).unwrap();
        // The centre's rays go forward with weight about one; the image's right goes to the
        // camera's right (-x when looking down +z), its top up.
        let mut centre = 0.;
        let n = 2000;
        for _ in 0..n {
            if let Some((r, w)) = cam.get_ray_weighted(0.5, 0.5, 0., 0.) {
                assert!(r.rd.z > 0.99, "{}", r.rd);
                centre += w;
            }
        }
        assert!((centre / n as f64 - 1.).abs() < 0.1, "centre weight {}", centre / n as f64);
        let right = cam.get_ray_weighted(0.9, 0.5, 0., 0.).unwrap().0.rd;
        let top = cam.get_ray_weighted(0.5, 0.9, 0., 0.).unwrap().0.rd;
        assert!(right.x < 0. && top.y > 0., "{} {}", right, top);
        // The corners are darker.
        let corner: f64 = (0..n).filter_map(|_| cam.get_ray_weighted(0.99, 0.99, 0., 0.)).map(|(_, w)| w).sum::<f64>() / n as f64;
        assert!(corner < 0.8 * centre / n as f64, "corner {} centre {}", corner, centre / n as f64);
        // Stopped down two stops (f/2 to f/4), a quarter of the light.
        let f4 = RealisticCamera::new(placement, LensSystem::named("dgauss.50mm").unwrap(), 5., Some(4.), 0.035, 1.5).unwrap();
        let dim: f64 = (0..n).filter_map(|_| f4.get_ray_weighted(0.5, 0.5, 0., 0.)).map(|(_, w)| w).sum::<f64>() / n as f64;
        let open_n = LensSystem::named("dgauss.50mm").unwrap().focal_length().unwrap() / 17.1e-3;
        let expected = (open_n / 4.).powi(2);
        assert!((dim / expected - 1.).abs() < 0.15, "f/4: {} vs {}", dim, expected);
    }
}
