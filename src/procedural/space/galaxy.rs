//! # Distant galaxies
//!
//! Spiral and elliptical galaxies, small patches of light scattered about the sky away from
//! the galactic plane (where our own galaxy's dust hides them: the "zone of avoidance").
//!
//! A spiral is a thin disc, inclined to the line of sight: a direction is followed (in the
//! gnomonic plane about the galaxy's centre) to where it crosses the disc, giving its place in
//! the disc's own coordinates. The disc's light falls exponentially with radius, and is
//! gathered into logarithmic spiral arms, θ = ln(r) / tan(pitch), blue with young stars and
//! knotted with clusters; being optically thin, it looks brighter the more obliquely it is
//! seen (through more of it), up to a limit for its thickness. In the middle sits the bulge,
//! round and yellow with old stars.
//!
//! Their brightness is the surface brightness of the core, in magnitudes per square arcsecond:
//! about 18.5 by default (a pixel's worth of a bright galaxy's centre). A disc's centre is
//! about 2.3 magnitudes fainter, near Freeman's law, 21.65 in B (K. C. Freeman, "On the Disks
//! of Spiral and S0 Galaxies", ApJ 160, 1970, p. 811): galaxies are bright cores in faint
//! haloes, their arms hard to see.
//!
//! An elliptical is an ellipse on the sky of axis ratio q, with a Sérsic profile,
//! I = exp(-b (r / r_e)^(1/n)), n = 2: a bright core in a broad halo, yellow throughout.
use crate::na::Vector3;
use crate::geometry::Pcg32;
use crate::noise::SeededNoise;
use crate::scenefile::SceneFile;
use super::{GalacticFrame, Photometry, luminance, mix, smoothstep, tangent_basis};
use serde_json::Value;
use std::f64::consts::{PI, TAU};

const OLD_STARS: [f64; 3] = [1., 0.84, 0.64];
const YOUNG_STARS: [f64; 3] = [0.7, 0.82, 1.];
/// The surface brightness of a disc's centre, relative to the core's: 10^(-0.4 x 2.3).
const DISC: f64 = 0.12;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Spiral,
    Elliptical,
}

pub struct Galaxy {
    kind: Kind,
    centre: Vector3<f64>,
    /// Its angular radius (the disc's or the ellipse's major axis), in radians.
    radius: f64,
    cos_reach: f64,
    /// Its radiance (luminance) at its core.
    brightness: f64,
    /// The disc's normal (spirals), and axes in its plane.
    normal: Vector3<f64>,
    disc_axes: (Vector3<f64>, Vector3<f64>),
    /// The major and minor axes on the sky (ellipticals), and their ratio.
    sky_axes: (Vector3<f64>, Vector3<f64>),
    axis_ratio: f64,
    arms: f64,
    /// tan of the arms' pitch angle.
    pitch: f64,
    phase: f64,
    noise: SeededNoise,
}

impl Galaxy {
    /// A galaxy of `kind` at unit `centre`, `radius` radians across, inclined by `inclination`
    /// radians (0 face on) about an axis at `position_angle` radians on the sky.
    #[allow(clippy::too_many_arguments)]
    pub fn new(kind: Kind, centre: Vector3<f64>, radius: f64, inclination: f64, position_angle: f64, brightness: f64, arms: usize, seed: u64) -> Galaxy {
        let mut rng = Pcg32::new(mix(seed), 0x6a1);
        let centre = centre.normalize();
        let (t, b) = tangent_basis(&centre);
        let major = t * position_angle.cos() + b * position_angle.sin();
        let minor = centre.cross(&major);
        // Tilted about the major axis, which stays in the plane of the sky.
        let normal = centre * inclination.cos() + minor * inclination.sin();
        let disc_axes = (major, normal.cross(&major));
        Galaxy {
            kind, centre, radius, cos_reach: (radius * 1.5).min(PI).cos(), brightness,
            normal, disc_axes, sky_axes: (major, minor),
            axis_ratio: inclination.cos().max(0.3),
            arms: arms as f64,
            pitch: (0.2 + 0.35 * rng.next_f64()).tan(),
            phase: rng.next_f64() * TAU,
            noise: SeededNoise::new(mix(seed ^ 0x9a1a)),
        }
    }

    /// From its description: `type` ("spiral" or "elliptical"), `direction`, `radius`
    /// (degrees), `inclination` and `position_angle` (degrees), `surface_brightness` (of its
    /// core, magnitudes per square arcsecond), `brightness` (scaling that), `arms`, `seed`.
    pub fn parse(o: &Value, seed: u64, photometry: &Photometry) -> Galaxy {
        let kind = match o["type"].as_str().unwrap_or("spiral") {
            "spiral" => Kind::Spiral,
            "elliptical" => Kind::Elliptical,
            k => panic!("Unknown galaxy type {:?}: expected \"spiral\" or \"elliptical\"", k),
        };
        Galaxy::new(
            kind,
            SceneFile::parse_vec3_def(o, "direction", Vector3::new(0., 0., 1.)),
            SceneFile::parse_number(&o["radius"], 1.).to_radians(),
            SceneFile::parse_number(&o["inclination"], 60.).to_radians(),
            SceneFile::parse_number(&o["position_angle"], 30.).to_radians(),
            photometry.radiance(SceneFile::parse_number(&o["surface_brightness"], 18.5)) * SceneFile::parse_number(&o["brightness"], 1.),
            SceneFile::parse_int(&o["arms"], 2),
            SceneFile::parse_int(&o["seed"], seed as usize) as u64,
        )
    }

    /// Its parameters for the GPU, in the order of the shader's `GALAXY_*`: its kind (0
    /// spiral, 1 elliptical), centre, radius, cos of its reach, brightness, the disc's normal and
    /// axes, the axes on the sky, the axis ratio, arms, pitch, phase and its noise's offset.
    pub fn export(&self) -> Vec<f64> {
        let mut v = vec![if self.kind == Kind::Spiral { 0. } else { 1. }];
        v.extend(self.centre.iter());
        v.extend([self.radius, self.cos_reach, self.brightness]);
        for a in [self.normal, self.disc_axes.0, self.disc_axes.1, self.sky_axes.0, self.sky_axes.1] {
            v.extend(a.iter());
        }
        v.extend([self.axis_ratio, self.arms, self.pitch, self.phase]);
        v.extend(self.noise.offset().iter());
        v
    }

    /// The light of the galaxy in unit direction `d`.
    pub fn radiance(&self, d: &Vector3<f64>) -> Vector3<f64> {
        let c = d.dot(&self.centre);
        if c < self.cos_reach {
            return Vector3::zeros();
        }
        let w = (d / c - self.centre) / self.radius;
        let old = Vector3::from(OLD_STARS) / luminance(&Vector3::from(OLD_STARS));
        let young = Vector3::from(YOUNG_STARS) / luminance(&Vector3::from(YOUNG_STARS));
        match self.kind {
            Kind::Elliptical => {
                let (x, y) = (w.dot(&self.sky_axes.0), w.dot(&self.sky_axes.1) / self.axis_ratio);
                let r = (x * x + y * y).sqrt();
                old * (self.brightness * (-2.5 * (r / 0.3).powf(0.7)).exp())
            },
            Kind::Spiral => {
                // Where the line of sight crosses the disc: w + t centre, in the disc's plane.
                let facing = self.normal.dot(&self.centre);
                let facing = if facing.abs() < 0.12 { 0.12f64.copysign(facing) } else { facing };
                let p = w - self.centre * (self.normal.dot(&w) / facing);
                let (x, y) = (p.dot(&self.disc_axes.0), p.dot(&self.disc_axes.1));
                let rho = (x * x + y * y).sqrt();
                let theta = y.atan2(x);
                // Logarithmic arms: brightest where θ m = m ln(ρ) / tan(pitch) + phase.
                let wind = self.arms * (theta - (rho + 0.03).ln() / self.pitch) + self.phase;
                // Near edge on, the arms are seen through one another and blur together.
                let arm = (0.5 + 0.5 * wind.cos()).powi(3) * facing.abs().sqrt();
                let knots = (1. + 1.2 * self.noise.fbm(Vector3::new(x, y, 0.) * 9., 4)).max(0.);
                // The disc ends (smoothly) at its radius.
                let disc = (-rho / 0.28).exp() * smoothstep(1.15, 0.8, rho) * (1. / facing.abs()).min(4.) * DISC;
                let r_sky = w.norm();
                let bulge = (-r_sky / 0.06).exp();
                // Seen near edge on, the dust in the disc's midplane crosses the bulge as a dark
                // lane along the major axis.
                let across = w.dot(&self.sky_axes.1);
                let lane = 1. - 0.75 * (1. - facing.abs()).powi(2) * (-(across / 0.035).powi(2)).exp() * (-r_sky / 0.7).exp();
                (old * (disc * (0.6 + 0.4 * (1. - arm)) + bulge) + young * (disc * arm * 1.8 * knots)) * (self.brightness * lane)
            },
        }
    }

    /// A bound on the luminance of its light in unit direction `d`: none outside it; within
    /// it, the core, and the disc seen as obliquely as it brightens, its arms and knots at
    /// their brightest (the colours are of unit luminance).
    pub fn bound(&self, d: &Vector3<f64>) -> f64 {
        if d.dot(&self.centre) < self.cos_reach {
            return 0.;
        }
        self.brightness * match self.kind {
            Kind::Elliptical => 1.,
            Kind::Spiral => 1. + 4. * DISC * (1. + 1.8 * 2.2),
        }
    }
}

/// The galaxies described by `v`: none if `false`; a list if a list; or else `count` of them at
/// random, away from the galactic plane, `size` (the range of radii, in degrees) across, their
/// cores 18.3 to 19.8 magnitudes per square arcsecond, with `brightness` scaling their light and
/// `spirals` the share that are spirals.
pub fn parse(v: &Value, seed: u64, frame: &GalacticFrame, photometry: &Photometry) -> Vec<Galaxy> {
    match v {
        Value::Bool(false) => vec![],
        Value::Array(list) => list.iter().enumerate().map(|(i, o)| Galaxy::parse(o, mix(seed ^ (0x6000 + i as u64)), photometry)).collect(),
        _ => {
            let count = SceneFile::parse_int(&v["count"], 40);
            let scale = SceneFile::parse_number(&v["brightness"], 1.);
            let spirals = SceneFile::parse_number(&v["spirals"], 0.65);
            let (small, large) = (SceneFile::parse_number(&v["size"][0], 0.5), SceneFile::parse_number(&v["size"][1], 3.));
            let mut rng = Pcg32::new(mix(seed ^ 0x6a1), 0x6a1);
            (0..count).map(|i| {
                // Uniform over the sphere, outside the zone of avoidance.
                let d = loop {
                    let z = 2. * rng.next_f64() - 1.;
                    let phi = TAU * rng.next_f64();
                    let r = (1. - z * z).sqrt();
                    let d = Vector3::new(r * phi.cos(), r * phi.sin(), z);
                    if frame.coordinates(&d).1.abs() > 0.25 {
                        break d;
                    }
                };
                // Many more small ones than large: radii in proportion to 1 / size².
                let u = rng.next_f64();
                let radius = 1. / (1. / small - u * (1. / small - 1. / large));
                let kind = if rng.next_f64() < spirals { Kind::Spiral } else { Kind::Elliptical };
                // Random orientations, short of edge on (where a small one is a scratch).
                let inclination = (0.25 + 0.75 * rng.next_f64()).acos();
                let position_angle = rng.next_f64() * PI;
                let brightness = scale * photometry.radiance(18.3 + 1.5 * rng.next_f64());
                let arms = if rng.next_f64() < 0.8 { 2 } else { 3 };
                Galaxy::new(kind, d, radius.to_radians(), inclination, position_angle, brightness, arms, mix(seed ^ (0x7000 + i as u64)))
            }).collect()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galaxy_is_brightest_at_its_core_and_confined() {
        for k in ["spiral", "elliptical"] {
            let g = Galaxy::parse(&serde_json::json!({ "type": k, "direction": [0, 0, 1], "radius": 2 }), 1, &Photometry::new(1.));
            let core = luminance(&g.radiance(&Vector3::new(0., 0., 1.)));
            let off = luminance(&g.radiance(&Vector3::new(0.01, 0.01, 1.).normalize()));
            assert!(core > off && off > 0., "{}: {} {}", k, core, off);
            assert_eq!(g.radiance(&Vector3::new(0., 1., 0.)), Vector3::zeros());
        }
    }

    #[test]
    fn test_inclined_disc_is_foreshortened() {
        // Seen at 70°, the disc's light along its minor axis falls off sooner than along its
        // major axis (outside the round bulge).
        let g = Galaxy::parse(&serde_json::json!({ "type": "spiral", "direction": [0, 0, 1], "radius": 3,
            "inclination": 70, "position_angle": 0 }), 1, &Photometry::new(1.));
        let along = |a: Vector3<f64>| (8..30).map(|i| luminance(&g.radiance(&(Vector3::new(0., 0., 1.) + a * (i as f64 * 0.002)).normalize()))).sum::<f64>();
        let (major, minor) = (g.sky_axes.0, g.sky_axes.1);
        assert!(along(major) > 1.5 * along(minor), "{} vs {}", along(major), along(minor));
    }

    #[test]
    fn test_random_galaxies_avoid_the_plane() {
        let f = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        let gs = parse(&Value::Null, 3, &f, &Photometry::new(1.));
        assert_eq!(gs.len(), 40);
        assert!(gs.iter().all(|g| f.coordinates(&g.centre).1.abs() > 0.25));
    }
}
