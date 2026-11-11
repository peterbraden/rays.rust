//! Ice-crystal halos: sun dogs, the 22° and 46° halos and tangent arcs, by tracing light
//! through hexagonal ice crystals.
//!
//! Halos are geometric optics: sunlight refracted through, and reflected inside, ice prisms.
//! Through two side faces that are not parallel (a 60° prism) light is deviated by at least
//! the prism's minimum deviation, D = 2 asin(n sin 30°) - 60° ≈ 22° for ice, and it piles up
//! there; dispersion makes the inner edge red. Randomly oriented crystals spread this into a
//! ring, the 22° halo (and through a side and a basal face, a 90° prism, the 46°). Plates fall
//! with their large faces nearly horizontal; light crossing their side faces then stays near
//! the sun's elevation, and concentrates in two spots beside the sun: sun dogs (parhelia).
//! They lie further out than 22° as the sun rises, as if the prism's index were Bravais's
//! n' = sqrt(n² - sin² h) / cos h for solar elevation h, and fade above about 60°. Columns
//! lying horizontally make tangent arcs. (W. Tape, "Atmospheric Halos", American Geophysical
//! Union 1994; R. Greenler, "Rainbows, Halos, and Glories", Cambridge 1980; the halo
//! simulators HaloSim, by L. Cowley and M. Schroeder, and HaloPoint work the same way.)
//!
//! There is no closed form for the scattering of oriented crystals, and it depends on the
//! direction light arrives from, not only on the angle it turns through. But the sun is fixed:
//! so trace many sun rays through randomly placed crystals of each population, and histogram
//! the directions they leave in, per colour channel. That is the phase function for sunlight,
//! per steradian. It is indexed, as halos are drawn, by where the light appears in the sky to
//! someone it reaches: the viewing direction, opposite to the light's, by elevation and by
//! azimuth from the sun's. Sun dogs are then at the sun's elevation, either side of it.
//!
//! Each ray: enter the crystal at a point uniform over its silhouette (so crystals are hit in
//! proportion to their projected area); at each face, reflect with the unpolarised Fresnel
//! reflectance or refract (Snell), totally reflecting inside beyond the critical angle; leave
//! when a ray refracts out. Rays leaving within a fraction of a degree of the sun's direction
//! (straight through parallel faces) are left out: they are, in effect, sunlight that was not
//! scattered at all.

use crate::color::Color;
use crate::geometry::Pcg32;
use crate::na::Vector3;
use rayon::prelude::*;
use std::f64::consts::PI;

/// Wavelength bands (µm) of the red, green and blue channels; each ray takes a wavelength
/// uniformly within its channel's band.
const BANDS: [(f64, f64); 3] = [(0.58, 0.66), (0.50, 0.58), (0.42, 0.50)];

/// The refractive index of ice at wavelength λ (µm): a Cauchy fit, n = 1.2985 + 0.00384 / λ²,
/// through 1.3098 at the sodium D line (589 nm) with dispersion of about 0.01 across the
/// visible, which puts the 22° halo's red and blue edges about 0.6° apart, as observed.
pub fn ice_index(wavelength: f64) -> f64 {
    1.2985 + 0.00384 / (wavelength * wavelength)
}

/// How a population of crystals is oriented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Habit {
    /// Hexagonal plates, c axis vertical within `tilt` (degrees, standard deviation), spun at
    /// random about it: sun dogs, the parhelic circle.
    Plate,
    /// Columns lying horizontally (c axis horizontal within `tilt`), at random about the
    /// vertical and about their own axis: tangent arcs.
    Column,
    /// Oriented at random: the 22° and 46° halos.
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crystals {
    pub habit: Habit,
    /// Share of the light scattered by this population.
    pub fraction: f64,
    /// Half-height of the prism over the distance from its axis to a side face: thin plates
    /// are about 0.1 to 0.3, columns 1 to 3.
    pub aspect: f64,
    pub tilt: f64,
}

/// The eight faces of a hexagonal prism, axis along z, a unit distance from the axis to each
/// side face and `half_height` to each basal face: (outward normal, distance from origin).
fn prism_faces(half_height: f64) -> [(Vector3<f64>, f64); 8] {
    let mut faces = [(Vector3::zeros(), 0.); 8];
    for (i, face) in faces.iter_mut().enumerate().take(6) {
        let a = i as f64 * PI / 3.;
        *face = (Vector3::new(a.cos(), a.sin(), 0.), 1.);
    }
    faces[6] = (Vector3::new(0., 0., 1.), half_height);
    faces[7] = (Vector3::new(0., 0., -1.), half_height);
    faces
}

/// Unpolarised Fresnel reflectance for light going from index n1 into n2 at incidence cosine
/// `cos_i`; 1 under total internal reflection. Also the cosine of the refracted ray.
fn fresnel(cos_i: f64, n1: f64, n2: f64) -> (f64, f64) {
    let sin_t2 = (n1 / n2).powi(2) * (1. - cos_i * cos_i);
    if sin_t2 >= 1. {
        return (1., 0.);
    }
    let cos_t = (1. - sin_t2).sqrt();
    let rs = ((n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t)).powi(2);
    let rp = ((n1 * cos_t - n2 * cos_i) / (n1 * cos_t + n2 * cos_i)).powi(2);
    ((rs + rp) / 2., cos_t)
}

/// Reflect `d` in a surface with unit normal `n`.
fn reflect(d: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    d - n * (2. * d.dot(n))
}

/// Refract `d` through a surface with unit normal `n` facing against `d`, from index n1 to n2,
/// given the refracted cosine.
fn refract(d: &Vector3<f64>, n: &Vector3<f64>, eta: f64, cos_i: f64, cos_t: f64) -> Vector3<f64> {
    (d * eta + n * (eta * cos_i - cos_t)).normalize()
}

/// A unit vector perpendicular to `v`.
fn perpendicular(v: &Vector3<f64>) -> Vector3<f64> {
    let a = if v.x.abs() < 0.9 { Vector3::new(1., 0., 0.) } else { Vector3::new(0., 1., 0.) };
    v.cross(&a).normalize()
}

fn gaussian(rng: &mut Pcg32) -> f64 {
    let (u, v) = (1. - rng.next_f64(), rng.next_f64());
    (-2. * u.ln()).sqrt() * (2. * PI * v).cos()
}

/// A random orientation for a crystal of `habit`: the crystal's x, y, z axes in world space
/// (y up).
fn orientation(c: &Crystals, rng: &mut Pcg32) -> [Vector3<f64>; 3] {
    let up = Vector3::new(0., 1., 0.);
    let tilted = |axis: Vector3<f64>, rng: &mut Pcg32| {
        let t = (c.tilt * gaussian(rng)).to_radians();
        let side = perpendicular(&axis);
        let around = 2. * PI * rng.next_f64();
        let side = side * around.cos() + axis.cross(&side) * around.sin();
        (axis * t.cos() + side * t.sin()).normalize()
    };
    let z = match c.habit {
        Habit::Plate => tilted(up, rng),
        Habit::Column => {
            let a = 2. * PI * rng.next_f64();
            tilted(Vector3::new(a.cos(), 0., a.sin()), rng)
        },
        Habit::Random => crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64()),
    };
    let x0 = perpendicular(&z);
    let spin = 2. * PI * rng.next_f64();
    let x = x0 * spin.cos() + z.cross(&x0) * spin.sin();
    [x, z.cross(&x), z]
}

/// What became of a ray aimed at a crystal.
enum Traced {
    /// It missed the crystal's silhouette: aim again.
    Missed,
    /// It left in this direction.
    Left(Vector3<f64>),
    /// Still inside after many reflections (rare): dropped.
    Trapped,
}

/// Trace one ray of direction `d` (world) and index `n` through a crystal of `c`, in a random
/// orientation and at a random point over a disc covering its silhouette.
fn trace_crystal(d: &Vector3<f64>, n: f64, c: &Crystals, rng: &mut Pcg32) -> Traced {
    let axes = orientation(c, rng);
    // Into the crystal's frame.
    let to_local = |v: &Vector3<f64>| Vector3::new(v.dot(&axes[0]), v.dot(&axes[1]), v.dot(&axes[2]));
    let to_world = |v: &Vector3<f64>| axes[0] * v.x + axes[1] * v.y + axes[2] * v.z;
    let faces = prism_faces(c.aspect);
    let mut dir = to_local(d);
    // A point uniform over a disc covering the silhouette, behind the crystal.
    let radius = (4. / 3. + c.aspect * c.aspect).sqrt();
    let (u, w) = (perpendicular(&dir), dir.cross(&perpendicular(&dir)));
    let (r, a) = (radius * rng.next_f64().sqrt(), 2. * PI * rng.next_f64());
    let origin = u * (r * a.cos()) + w * (r * a.sin()) - dir * (4. * radius);
    // Enter: the ray's interval inside every face's half-space.
    let (mut t_in, mut t_out, mut entry) = (f64::NEG_INFINITY, f64::INFINITY, 0);
    for (i, (normal, dist)) in faces.iter().enumerate() {
        let denom = normal.dot(&dir);
        let t = (dist - normal.dot(&origin)) / denom;
        if denom < 0. {
            if t > t_in { t_in = t; entry = i; }
        } else if denom > 0. {
            t_out = t_out.min(t);
        } else if normal.dot(&origin) > *dist {
            return Traced::Missed;
        }
    }
    if t_in >= t_out {
        return Traced::Missed;
    }
    let mut p = origin + dir * t_in;
    // At the entry face, from air.
    let normal = faces[entry].0;
    let cos_i = -dir.dot(&normal);
    let (reflectance, cos_t) = fresnel(cos_i, 1., n);
    if rng.next_f64() < reflectance {
        return Traced::Left(to_world(&reflect(&dir, &normal)));
    }
    dir = refract(&dir, &normal, 1. / n, cos_i, cos_t);
    // Inside: to the nearest face ahead, then reflect (Fresnel or total) or leave.
    for _ in 0..30 {
        let (mut best, mut face) = (f64::INFINITY, 0);
        for (i, (normal, dist)) in faces.iter().enumerate() {
            let denom = normal.dot(&dir);
            if denom > 1e-12 {
                let t = (dist - normal.dot(&p)) / denom;
                if t < best { best = t; face = i; }
            }
        }
        p += dir * best.max(0.);
        let normal = faces[face].0;
        let cos_i = dir.dot(&normal);
        let (reflectance, cos_t) = fresnel(cos_i, n, 1.);
        if rng.next_f64() < reflectance {
            dir = reflect(&dir, &normal);
        } else {
            return Traced::Left(to_world(&refract(&dir, &-normal, n, cos_i, cos_t)));
        }
    }
    Traced::Trapped
}

/// Angular resolution of the table, in degrees.
const BIN: f64 = 0.5;
const ELEVATIONS: usize = (180. / BIN) as usize;
/// Azimuths from the sun's, 0 to 180°: halos are symmetric about the sun's vertical.
const AZIMUTHS: usize = (180. / BIN) as usize;
/// Rays leaving within this angle (degrees) of the sun's direction count as unscattered.
const UNSCATTERED: f64 = 0.3;

/// Sunlight's phase function through ice crystals, per colour channel and per steradian, over
/// viewing directions (opposite to the light's) by elevation and azimuth from the sun.
pub struct HaloTable {
    sun: Vector3<f64>,
    channels: [Vec<f32>; 3],
}

/// Elevation and azimuth from the sun's (both radians, azimuth folded to [0, π]) of `d`.
fn angles(d: &Vector3<f64>, sun_azimuth: f64) -> (f64, f64) {
    let elevation = d.y.clamp(-1., 1.).asin();
    let mut azimuth = (d.z.atan2(d.x) - sun_azimuth).rem_euclid(2. * PI);
    if azimuth > PI {
        azimuth = 2. * PI - azimuth;
    }
    (elevation, azimuth)
}

impl HaloTable {
    /// Trace `rays` sun rays per channel through `crystals`, for the sun in direction `sun`.
    pub fn new(sun: &Vector3<f64>, crystals: &[Crystals], rays: usize, seed: u64) -> HaloTable {
        let sun = sun.normalize();
        let travel = -sun;
        let sun_azimuth = sun.z.atan2(sun.x);
        let total: f64 = crystals.iter().map(|c| c.fraction).sum();
        let bin = BIN.to_radians();
        let chunks = 64;
        let channels = [0, 1, 2].map(|channel| {
            let counts = (0..chunks).into_par_iter().map(|chunk| {
                let mut rng = Pcg32::new(seed, (channel * chunks + chunk) as u64 + 1);
                let mut counts = vec![0u32; ELEVATIONS * AZIMUTHS];
                let (lo, hi) = BANDS[channel];
                for k in 0..rays / chunks {
                    // Populations in proportion to their fractions, by stratifying over k.
                    let mut x = (k as f64 + rng.next_f64()) / (rays / chunks) as f64 * total;
                    let c = crystals.iter().find(|c| { x -= c.fraction; x < 0. }).unwrap_or(&crystals[0]);
                    let n = ice_index(lo + (hi - lo) * rng.next_f64());
                    let out = loop {
                        match trace_crystal(&travel, n, c, &mut rng) {
                            Traced::Missed => continue,
                            Traced::Left(out) => break Some(out),
                            Traced::Trapped => break None,
                        }
                    };
                    let out = match out { Some(o) => o, None => continue };
                    if out.dot(&travel) > UNSCATTERED.to_radians().cos() {
                        continue;
                    }
                    let (e, a) = angles(&-out, sun_azimuth);
                    let i = (((e + PI / 2.) / bin) as usize).min(ELEVATIONS - 1);
                    let j = ((a / bin) as usize).min(AZIMUTHS - 1);
                    counts[i * AZIMUTHS + j] += 1;
                }
                counts
            }).reduce(|| vec![0u32; ELEVATIONS * AZIMUTHS], |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) { *x += y; }
                a
            });
            let traced = (rays / chunks * chunks) as f64;
            // Per steradian: each bin covers cos(e) Δe Δa, twice over for the fold.
            let density: Vec<f64> = (0..ELEVATIONS * AZIMUTHS).map(|idx| {
                let e = ((idx / AZIMUTHS) as f64 + 0.5) * bin - PI / 2.;
                counts[idx] as f64 / traced / (2. * e.cos() * bin * bin)
            }).collect();
            smooth(&density).into_iter().map(|x| x as f32).collect()
        });
        HaloTable { sun, channels }
    }

    /// The phase function, per channel, for sunlight seen looking along `view` (a unit vector):
    /// light leaving in the direction opposite to it.
    pub fn eval(&self, view: &Vector3<f64>) -> Color {
        let (e, a) = angles(view, self.sun.z.atan2(self.sun.x));
        let bin = BIN.to_radians();
        // Bilinear between bin centres.
        let u = ((e + PI / 2.) / bin - 0.5).clamp(0., (ELEVATIONS - 1) as f64);
        let v = (a / bin - 0.5).clamp(0., (AZIMUTHS - 1) as f64);
        let (i, j) = ((u as usize).min(ELEVATIONS - 2), (v as usize).min(AZIMUTHS - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let at = |c: usize| {
            let t = |i: usize, j: usize| self.channels[c][i * AZIMUTHS + j] as f64;
            (t(i, j) * (1. - fv) + t(i, j + 1) * fv) * (1. - fu) + (t(i + 1, j) * (1. - fv) + t(i + 1, j + 1) * fv) * fu
        };
        Color::new(at(0), at(1), at(2))
    }

    /// The sun direction the table was traced for.
    pub fn sun(&self) -> Vector3<f64> {
        self.sun
    }
}

/// A light 3 x 3 blur of the histogram, [1 2 1] each way, which takes the edge off the Monte
/// Carlo noise at half a bin's cost in sharpness.
fn smooth(d: &[f64]) -> Vec<f64> {
    let at = |i: isize, j: isize| {
        let i = i.clamp(0, ELEVATIONS as isize - 1) as usize;
        // Azimuth reflects at 0 and π, the fold's mirror lines.
        let j = if j < 0 { (-j - 1) as usize } else if j >= AZIMUTHS as isize { 2 * AZIMUTHS - 1 - j as usize } else { j as usize };
        d[i * AZIMUTHS + j]
    };
    let w = [1., 2., 1.];
    (0..ELEVATIONS * AZIMUTHS).map(|idx| {
        let (i, j) = ((idx / AZIMUTHS) as isize, (idx % AZIMUTHS) as isize);
        let mut s = 0.;
        for di in -1..=1 {
            for dj in -1..=1 {
                s += w[(di + 1) as usize] * w[(dj + 1) as usize] * at(i + di, j + dj);
            }
        }
        s / 16.
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresnel() {
        let n: f64 = 1.31;
        let (r, cos_t) = fresnel(1., 1., n);
        assert!((r - ((n - 1.) / (n + 1.)).powi(2)).abs() < 1e-12 && (cos_t - 1.).abs() < 1e-12);
        // Total internal reflection beyond the critical angle, asin(1/n) ≈ 49.8°.
        assert_eq!(fresnel(60f64.to_radians().cos(), n, 1.).0, 1.);
        assert!(fresnel(45f64.to_radians().cos(), n, 1.).0 < 1.);
    }

    #[test]
    fn test_refraction_obeys_snell() {
        let n = 1.31;
        let d = Vector3::new(0.6, -0.8, 0.);
        let normal = Vector3::new(0., 1., 0.);
        let cos_i = -d.dot(&normal);
        let (_, cos_t) = fresnel(cos_i, 1., n);
        let t = refract(&d, &normal, 1. / n, cos_i, cos_t);
        let sin_i = (1. - cos_i * cos_i).sqrt();
        let sin_t = (1. - t.dot(&normal).powi(2)).sqrt();
        assert!((sin_i - n * sin_t).abs() < 1e-12 && (t.norm() - 1.).abs() < 1e-12);
    }

    /// Histogram of deviation angles (degrees) of light through randomly oriented crystals.
    fn deviations(n: f64, aspect: f64, rays: usize) -> Vec<f64> {
        let c = Crystals { habit: Habit::Random, fraction: 1., aspect, tilt: 0. };
        let mut rng = Pcg32::new(1, 1);
        let d = Vector3::new(0., -1., 0.);
        let mut out = vec![];
        while out.len() < rays {
            if let Traced::Left(o) = trace_crystal(&d, n, &c, &mut rng) {
                out.push(o.dot(&d).clamp(-1., 1.).acos().to_degrees());
            }
        }
        out
    }

    #[test]
    fn test_22_degree_halo_at_minimum_deviation() {
        let n = ice_index(0.55);
        let min_dev = 2. * (n * 0.5f64).asin().to_degrees() - 60.;
        assert!((min_dev - 21.8).abs() < 0.3, "{}", min_dev);
        let dev = deviations(n, 1., 200_000);
        // Per degree of deviation, per steradian: light piles up just beyond D, and none of the
        // 60° prism's light is found inside it.
        let per_sr = |lo: f64, hi: f64| {
            let count = dev.iter().filter(|x| **x >= lo && **x < hi).count() as f64;
            let solid = 2. * PI * (lo.to_radians().cos() - hi.to_radians().cos());
            count / solid
        };
        let halo = per_sr(min_dev, min_dev + 1.5);
        let inside = per_sr(min_dev - 8., min_dev - 2.);
        let outside = per_sr(min_dev + 6., min_dev + 12.);
        assert!(halo > 3. * inside && halo > 1.5 * outside, "halo {} inside {} outside {}", halo, inside, outside);
    }

    #[test]
    fn test_halo_is_red_inside() {
        // The halo's sharp inner edge, where its brightness first passes half its peak: further
        // out for blue, which bends more. Ice's dispersion puts them about 0.6° apart.
        let edge = |l: f64| {
            let d = deviations(ice_index(l), 1., 150_000);
            let bins: Vec<f64> = (0..60).map(|k| {
                let (lo, hi) = (19. + k as f64 * 0.1, 19.1 + k as f64 * 0.1);
                d.iter().filter(|x| **x >= lo && **x < hi).count() as f64 / (lo.to_radians().sin())
            }).collect();
            let peak = bins.iter().cloned().fold(0., f64::max);
            19. + bins.iter().position(|b| *b > peak / 2.).unwrap() as f64 * 0.1
        };
        let (red, blue) = (edge(0.65), edge(0.45));
        assert!(blue > red + 0.3 && blue < red + 1., "red {} blue {}", red, blue);
    }

    #[test]
    fn test_sun_dogs_follow_bravais() {
        // Horizontal plates and the sun at 30°: sun dogs at its elevation, at the minimum
        // deviation of a 60° prism of index n' = sqrt(n² - sin² h) / cos h, measured in azimuth.
        let h: f64 = 30f64.to_radians();
        let sun = Vector3::new(0., h.sin(), h.cos());
        let plates = [Crystals { habit: Habit::Plate, fraction: 1., aspect: 0.2, tilt: 0.3 }];
        let table = HaloTable::new(&sun, &plates, 400_000, 3);
        let n = ice_index(0.55);
        let n_prime = (n * n - h.sin().powi(2)).sqrt() / h.cos();
        let expected = 2. * (n_prime * 0.5).asin().to_degrees() - 60.;
        assert!((expected - 28.7).abs() < 0.5, "{}", expected);
        // Brightest along the sun's elevation, beyond 5° from it.
        let (mut best, mut at) = (0., 0.);
        for k in 10..300 {
            let az = (k as f64 * 0.2).to_radians();
            let out = Vector3::new(az.sin() * h.cos(), h.sin(), az.cos() * h.cos());
            let g = table.eval(&out).rgb.y;
            if g > best { best = g; at = az.to_degrees(); }
        }
        assert!((at - expected).abs() < 1.5, "sun dog at {}°, expected {}°", at, expected);
        // Much brighter than at the same distance straight above the sun.
        let above = Vector3::new(0., (h + expected.to_radians()).sin(), (h + expected.to_radians()).cos());
        assert!(best > 10. * table.eval(&above).rgb.y, "{} vs {}", best, table.eval(&above).rgb.y);
    }

    #[test]
    fn test_table_is_normalised() {
        // Integrated over the sphere: all traced light, less what passed straight through
        // parallel faces, which is left out as unscattered.
        let h: f64 = 20f64.to_radians();
        let sun = Vector3::new(0., h.sin(), h.cos());
        let random = [Crystals { habit: Habit::Random, fraction: 1., aspect: 1., tilt: 0. }];
        let table = HaloTable::new(&sun, &random, 200_000, 4);
        let (n_e, n_a) = (360, 720);
        let mut total = 0.;
        for i in 0..n_e {
            let e = -PI / 2. + (i as f64 + 0.5) * PI / n_e as f64;
            for j in 0..n_a {
                let a = (j as f64 + 0.5) * 2. * PI / n_a as f64;
                let d = Vector3::new(a.cos() * e.cos(), e.sin(), a.sin() * e.cos());
                total += table.eval(&d).rgb.y * e.cos() * (PI / n_e as f64) * (2. * PI / n_a as f64);
            }
        }
        let dev = deviations(ice_index(0.55), 1., 100_000);
        let undeviated = dev.iter().filter(|x| **x < UNSCATTERED).count() as f64 / dev.len() as f64;
        assert!(undeviated > 0.05, "{}", undeviated);
        assert!((total - (1. - undeviated)).abs() < 0.02, "{} vs {}", total, 1. - undeviated);
    }
}

