//! # A field of stars
//!
//! Stars are points: far smaller than a pixel. Each is drawn as a small Gaussian of angular
//! standard deviation σ (by default half a pixel, so it covers about one), carrying the star's
//! whole flux, so that a pixel's samples find it and sum it, and its brightness doesn't depend
//! on how many samples land on it.
//!
//! The sphere is divided into cells by a cube map (each face an N x N grid). A cell's stars are
//! generated when they are needed, from a generator seeded by the cell's index: their number is
//! Poisson, with a mean in proportion to the cell's solid angle and the density of stars in its
//! direction; their positions are uniform within it. A direction only needs the stars of the
//! cells around it: cells are wider than a star's reach (4σ), so a star can only show in the
//! eight neighbours of its own cell, which may be on the next face across an edge.
//!
//! Their brightness follows the cumulative counts of stars brighter than each magnitude m,
//! which grow as N(< m) ∝ 10^(0.47 m) over the naked-eye range (about 5000 stars brighter than
//! sixth magnitude, 120,000 brighter than ninth; Allen's Astrophysical Quantities, 4th ed.,
//! 2000, section 20.1). A magnitude sampled by inverting it is m = m_lim + log10(u) / 0.47, and
//! a star's flux is 10^(-0.4 m) times that of a magnitude-zero star, on the scene's physical
//! scale (`Photometry`). Their colours are those of
//! black bodies (`blackbody`), at temperatures drawn from the spectral classes in rough
//! proportion to the bright stars' (more of the hot B and A stars, and the K giants, than a
//! volume-limited count would have). They crowd towards the galactic plane, and more so towards
//! the galactic centre, as they do in the sky.
use crate::na::Vector3;
use crate::geometry::Pcg32;
use crate::scenefile::SceneFile;
use super::{GalacticFrame, Photometry, blackbody, fibonacci_sphere, mix};
use serde_json::Value;
use std::f64::consts::PI;

/// The slope of log10 N(< m) with magnitude.
const LUMINOSITY_SLOPE: f64 = 0.47;
/// The brightest stars generated (Sirius is -1.46).
const BRIGHTEST: f64 = -1.5;
/// How far a star's light reaches, in standard deviations.
const REACH: f64 = 4.;
/// Temperatures tabulated for star colours, spaced evenly in log T between these.
const COLOURS: usize = 64;
const COOLEST: f64 = 2400.;
const HOTTEST: f64 = 30000.;

/// Spectral classes: the share of the stars in each, and its range of temperatures.
const CLASSES: [(f64, f64, f64); 7] = [
    (0.02, 30000., 30000.), // O
    (0.13, 10000., 28000.), // B
    (0.20, 7500., 10000.),  // A
    (0.17, 6000., 7500.),   // F
    (0.15, 5200., 6000.),   // G
    (0.25, 3700., 5200.),   // K
    (0.08, 2400., 3700.),   // M
];

pub struct Stars {
    /// How many stars there are in the whole sky.
    pub count: f64,
    pub magnitude_limit: f64,
    /// The flux of a magnitude-zero star (its irradiance).
    flux_zero: f64,
    /// The total flux of each cell's stars, by face, row and column: what bounds their light
    /// (`bound`) without generating them.
    cell_flux: Vec<f32>,
    /// The angular standard deviation of a star's image, in radians.
    sigma: f64,
    /// How much of a star's colour to show: 0 white, 1 its black body's.
    saturation: f64,
    /// Cells across each face of the cube.
    cells: usize,
    /// How much denser in stars the galactic plane and bulge are than the mean.
    plane: f64,
    bulge: f64,
    /// The mean of the density over the sphere, which the counts are divided by.
    mean_density: f64,
    seed: u64,
    colours: Vec<Vector3<f64>>,
}

/// The cube face, and coordinates on it in [-1, 1]², of unit direction `d` projected onto
/// face `f` (0..6: +x, -x, +y, -y, +z, -z), or None if it faces away.
fn face_coordinates(d: &Vector3<f64>, f: usize) -> Option<(f64, f64)> {
    let (axis, sign) = (f / 2, if f.is_multiple_of(2) { 1. } else { -1. });
    let along = d[axis] * sign;
    if along <= 0. {
        return None;
    }
    Some((d[(axis + 1) % 3] / along, d[(axis + 2) % 3] / along))
}

/// The unit direction for coordinates (u, v) on face `f`.
fn face_direction(f: usize, u: f64, v: f64) -> Vector3<f64> {
    let (axis, sign) = (f / 2, if f.is_multiple_of(2) { 1. } else { -1. });
    let mut d = Vector3::zeros();
    d[axis] = sign;
    d[(axis + 1) % 3] = u;
    d[(axis + 2) % 3] = v;
    d.normalize()
}

impl Stars {
    pub fn parse(o: &Value, seed: u64, pixel_angle: f64, frame: &GalacticFrame, photometry: &Photometry) -> Stars {
        let count = SceneFile::parse_number(&o["count"], 40_000.).max(0.);
        let magnitude_limit = SceneFile::parse_number(&o["magnitude_limit"], 8.);
        // "size" is the standard deviation in degrees; by default half a pixel.
        let sigma = o["size"].as_f64().map_or(0.5 * pixel_angle, |s| s.to_radians()).max(1e-6);
        // `brightness` scales their light from the physical scale.
        let brightness = SceneFile::parse_number(&o["brightness"], 1.);
        let plane = SceneFile::parse_number(&o["galactic_concentration"], 3.);
        let bulge = plane;
        // As many cells as keep two stars to each on average, but no smaller than a star's
        // reach, even in the corners of the faces, where they are narrowest (half as wide).
        let cells = ((count / 12.).sqrt().ceil() as usize).clamp(1, ((0.9 / (REACH * sigma)) as usize).max(1));
        let colours = (0..COLOURS).map(|i| {
            let t = COOLEST * (HOTTEST / COOLEST).powf(i as f64 / (COLOURS - 1) as f64);
            blackbody(t)
        }).collect();
        let mut stars = Stars {
            count, magnitude_limit, flux_zero: brightness * photometry.irradiance(0.), cell_flux: vec![], sigma,
            saturation: SceneFile::parse_number(&o["saturation"], 1.),
            cells, plane, bulge, mean_density: 1., seed: mix(seed ^ 0x57a2), colours,
        };
        let n = 20000;
        stars.mean_density = fibonacci_sphere(n).map(|d| {
            let (l, b) = frame.coordinates(&d);
            stars.density(l, b)
        }).sum::<f64>() / n as f64;
        let n = stars.cells;
        let cell_flux = (0 .. 6 * n * n).map(|k| {
            let mut total = 0.;
            stars.each_star(k / (n * n), (k / n) % n, k % n, frame, |_, flux, _| total += flux);
            // Rounded up, to stay a bound.
            (total * (1. + 1e-6)) as f32
        }).collect();
        stars.cell_flux = cell_flux;
        stars
    }

    /// The density of stars at galactic longitude `l` and latitude `b`, relative to the
    /// poles': a thin disc, and the bulge about the centre.
    fn density(&self, l: f64, b: f64) -> f64 {
        1. + self.plane * (-(b / 0.2).powi(2)).exp() + self.bulge * (-(l * l) / (2. * 0.35 * 0.35) - (b * b) / (2. * 0.25 * 0.25)).exp()
    }

    /// The mean number of stars in cell (i, j) of face `f`.
    fn mean_in_cell(&self, f: usize, i: usize, j: usize, frame: &GalacticFrame) -> f64 {
        let n = self.cells as f64;
        let w = 2. / n;
        let (u, v) = (-1. + (i as f64 + 0.5) * w, -1. + (j as f64 + 0.5) * w);
        // The solid angle of a small patch of the face: du dv / (1 + u² + v²)^(3/2).
        let solid_angle = w * w / (1. + u * u + v * v).powf(1.5);
        let (l, b) = frame.coordinates(&face_direction(f, u, v));
        self.count * solid_angle / (4. * PI) * self.density(l, b) / self.mean_density
    }

    /// The colour of a star at `kelvin`.
    fn colour(&self, kelvin: f64) -> Vector3<f64> {
        let x = (kelvin / COOLEST).ln() / (HOTTEST / COOLEST).ln() * (COLOURS - 1) as f64;
        let c = self.colours[(x.round().max(0.) as usize).min(COLOURS - 1)];
        Vector3::repeat(1.) + (c - Vector3::repeat(1.)) * self.saturation
    }

    /// Call `f` with the direction, flux and colour of each star in cell (i, j) of face `face`.
    fn each_star(&self, face: usize, i: usize, j: usize, frame: &GalacticFrame, mut f: impl FnMut(Vector3<f64>, f64, Vector3<f64>)) {
        let key = ((face * self.cells + i) * self.cells + j) as u64;
        let mut rng = Pcg32::new(mix(self.seed ^ mix(key)), 0x57a2);
        let lambda = self.mean_in_cell(face, i, j, frame);
        // Poisson by counting uniform variates until their product drops below e^-λ (Knuth,
        // TAOCP vol. 2, 3.4.1): fine for the few stars a cell has.
        let limit = (-lambda).exp();
        let mut p = rng.next_f64();
        let w = 2. / self.cells as f64;
        while p > limit {
            let u = -1. + (i as f64 + rng.next_f64()) * w;
            let v = -1. + (j as f64 + rng.next_f64()) * w;
            let m = (self.magnitude_limit + rng.next_f64().max(1e-300).log10() / LUMINOSITY_SLOPE).max(BRIGHTEST);
            let mut pick = rng.next_f64();
            let mut kelvin = 5800.;
            for &(share, lo, hi) in CLASSES.iter() {
                if pick < share {
                    kelvin = lo * (hi / lo).powf(pick / share);
                    break;
                }
                pick -= share;
            }
            f(face_direction(face, u, v), self.flux_zero * 10f64.powf(-0.4 * m), self.colour(kelvin));
            p *= rng.next_f64();
        }
    }

    /// The stars for the GPU, which can't generate them (`Pcg32` needs 64-bit integers, which
    /// WebGPU lacks): the angular standard deviation, cells across a face, where each cell's
    /// stars start in the list (and where the last ends), each star's direction and flux times
    /// colour, and each cell's total flux (`bound`).
    pub fn export(&self, frame: &GalacticFrame) -> (f64, usize, Vec<u32>, Vec<[f64; 6]>, Vec<f32>) {
        let n = self.cells;
        let mut starts = Vec::with_capacity(6 * n * n + 1);
        let mut stars = vec![];
        for k in 0..6 * n * n {
            starts.push(stars.len() as u32);
            self.each_star(k / (n * n), (k / n) % n, k % n, frame, |d, flux, colour| {
                let c = colour * flux;
                stars.push([d.x, d.y, d.z, c.x, c.y, c.z]);
            });
        }
        starts.push(stars.len() as u32);
        (self.sigma, n, starts, stars, self.cell_flux.clone())
    }

    /// Call `f` with each cell (face, row, column) whose stars may be seen in unit direction
    /// `d`: the cell it falls in and those around it, on every face it is near.
    fn neighbourhood(&self, d: &Vector3<f64>, mut f: impl FnMut(usize, usize, usize)) {
        let reach = REACH * self.sigma;
        let n = self.cells as isize;
        for face in 0..6 {
            let Some((u, v)) = face_coordinates(d, face) else { continue };
            // Near an edge, stars on the next face over may reach across it; a step of θ on a
            // face is at most three times as long in its coordinates.
            let margin = 3. * reach;
            if u.abs() > 1. + margin || v.abs() > 1. + margin {
                continue;
            }
            let ci = (((u + 1.) / 2. * n as f64).floor() as isize).clamp(0, n - 1);
            let cj = (((v + 1.) / 2. * n as f64).floor() as isize).clamp(0, n - 1);
            for i in (ci - 1).max(0) ..= (ci + 1).min(n - 1) {
                for j in (cj - 1).max(0) ..= (cj + 1).min(n - 1) {
                    f(face, i as usize, j as usize);
                }
            }
        }
    }

    /// The radiance of the stars seen in unit direction `d`.
    pub fn radiance(&self, d: &Vector3<f64>, frame: &GalacticFrame) -> Vector3<f64> {
        let reach = REACH * self.sigma;
        let norm = 1. / (2. * PI * self.sigma * self.sigma);
        let mut sum = Vector3::zeros();
        self.neighbourhood(d, |face, i, j| {
            self.each_star(face, i, j, frame, |s, flux, colour| {
                let r2 = (d - s).norm_squared();
                if r2 < reach * reach {
                    sum += colour * (flux * norm * (-r2 / (2. * self.sigma * self.sigma)).exp());
                }
            });
        });
        sum
    }

    /// A bound on the luminance of `radiance(d)`, from the cells' total fluxes: each star's
    /// peak is its flux over 2πσ², and its colour is of unit luminance (as is any mix of it
    /// with white, `colour`).
    pub fn bound(&self, d: &Vector3<f64>) -> f64 {
        let n = self.cells;
        let mut total = 0.;
        self.neighbourhood(d, |face, i, j| total += self.cell_flux[(face * n + i) * n + j] as f64);
        total / (2. * PI * self.sigma * self.sigma)
    }

    /// Call `f` with the direction, flux and colour of every star.
    fn each(&self, frame: &GalacticFrame, mut f: impl FnMut(Vector3<f64>, f64, Vector3<f64>)) {
        for face in 0..6 {
            for i in 0..self.cells {
                for j in 0..self.cells {
                    self.each_star(face, i, j, frame, &mut f);
                }
            }
        }
    }

    /// Every star, as (direction, flux, colour): for tests.
    #[cfg(test)]
    pub fn all(&self, frame: &GalacticFrame) -> Vec<(Vector3<f64>, f64, Vector3<f64>)> {
        let mut v = vec![];
        self.each(frame, |d, f, c| v.push((d, f, c)));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stars(o: Value) -> (Stars, GalacticFrame) {
        let frame = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        (Stars::parse(&o, 7, 2e-3, &frame, &Photometry::new(1.)), frame)
    }

    #[test]
    fn test_face_coordinates_round_trip() {
        for f in 0..6 {
            for &(u, v) in &[(0., 0.), (0.5, -0.9), (-1., 1.)] {
                let d = face_direction(f, u, v);
                let (u2, v2) = face_coordinates(&d, f).unwrap();
                assert!((u - u2).abs() < 1e-9 && (v - v2).abs() < 1e-9, "face {}: {} {} -> {} {}", f, u, v, u2, v2);
            }
        }
    }

    #[test]
    fn test_star_counts_and_magnitudes() {
        let (s, frame) = stars(serde_json::json!({ "count": 20000, "magnitude_limit": 8 }));
        let all = s.all(&frame);
        let n = all.len() as f64;
        assert!((n - 20000.).abs() < 5. * 20000f64.sqrt(), "{} stars", n);
        // A tenth as many stars two magnitudes brighter than the limit: 10^(-0.47 x 2) = 0.115.
        let bright = all.iter().filter(|(_, f, _)| *f >= s.flux_zero * 10f64.powf(-0.4 * 6.)).count() as f64;
        assert!((bright / n - 0.115).abs() < 0.01, "{}", bright / n);
        // More of them by the galactic plane than near its poles.
        let near_plane = all.iter().filter(|(d, _, _)| d.y.abs() < 0.2).count() as f64;
        let near_poles = all.iter().filter(|(d, _, _)| d.y.abs() > 0.8).count() as f64;
        // The two bands' solid angles: 2π x 0.4 and 2π x 0.4.
        assert!(near_plane > 2. * near_poles, "{} vs {}", near_plane, near_poles);
    }

    #[test]
    fn test_star_brightness() {
        // Physically: a magnitude-zero star gives 10^(-0.4 x 26.74) of the sun's irradiance;
        // and none is brighter than about Sirius.
        let (s, frame) = stars(serde_json::json!({}));
        assert!((s.flux_zero / 10f64.powf(-0.4 * 26.74) - 1.).abs() < 1e-9);
        let brightest = s.all(&frame).iter().map(|(_, f, _)| *f).fold(0., f64::max);
        assert!(brightest <= s.flux_zero * 10f64.powf(0.4 * 1.5) * (1. + 1e-9));
        let (doubled, _) = stars(serde_json::json!({ "brightness": 2 }));
        assert!((doubled.flux_zero / s.flux_zero - 2.).abs() < 1e-9);
    }

    #[test]
    fn test_star_flux_is_conserved() {
        // Integrating the radiance about a star gives back its flux, wherever it is (on the
        // edges and corners of the cube's faces too).
        let (s, frame) = stars(serde_json::json!({ "count": 3000, "size": 0.3 }));
        let all = s.all(&frame);
        let pick = |pred: &dyn Fn(&Vector3<f64>) -> bool| all.iter().filter(|(d, _, _)| pred(d)).max_by(|a, b| a.1.total_cmp(&b.1)).unwrap().clone();
        for (d, flux, colour) in [pick(&|_| true), pick(&|d| (d.x.abs() - d.y.abs()).abs() < 0.01 && d.z.abs() < d.x.abs())] {
            let (t, b) = super::super::tangent_basis(&d);
            let (k, step) = (60, 2. * REACH * s.sigma / 60.);
            let mut sum = Vector3::zeros();
            for x in 0..k {
                for y in 0..k {
                    let (a, c) = ((x as f64 + 0.5) * step - REACH * s.sigma, (y as f64 + 0.5) * step - REACH * s.sigma);
                    sum += s.radiance(&(d + t * a + b * c).normalize(), &frame) * step * step;
                }
            }
            // Within the reach, a 2D Gaussian holds 1 - e^-8 of its weight, and neighbouring
            // stars may add a little.
            assert!((sum.y / (flux * colour.y) - 1.).abs() < 0.02, "{} vs {} at {}", sum.y, flux * colour.y, d);
        }
    }
}
