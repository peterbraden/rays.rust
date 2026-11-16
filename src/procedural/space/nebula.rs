//! # Nebulae
//!
//! Clouds of gas and dust, placed about the sky (mostly along the galactic plane, where they
//! are) or where the scene puts them. Each is a patch of the sky about its centre, a few
//! degrees across, filled with domain-warped noise (`warped_fbm`) for the swirls and filaments
//! of gas, inside an edge itself made ragged by noise. The kinds:
//!
//! - `emission`: gas ionised by hot young stars within, glowing in the lines of its atoms:
//!   hydrogen-alpha (656 nm, deep red-pink) throughout, and doubly ionised oxygen (501 nm,
//!   teal) where the radiation is hardest, near the core; threaded with dark dust.
//! - `reflection`: dust lit by nearby stars, scattering their light, bluest (as the sky is
//!   blue); faint and smooth.
//! - `planetary`: the shell a dying star has thrown off: a ring, oxygen-teal inside and
//!   hydrogen-red at its rim, about its white dwarf. Small.
//! - `dark`: dust alone, cold and unlit, hiding what lies behind it.
//!
//! Their brightness is a surface brightness, in magnitudes per square arcsecond, where their
//! gas is densest: by default 21 for emission nebulae (the brighter H II regions, the Lagoon
//! or the North America nebula, are 20 to 22 in V), 22 for reflection nebulae, 19 for the
//! shells of planetary nebulae (which are small and bright: the Ring nebula's is about 18).
//!
//! Each returns the light it emits towards the viewer and the optical depth of its dust,
//! which dims what lies behind (`Space::radiance`).
use crate::na::Vector3;
use crate::geometry::Pcg32;
use crate::noise::SeededNoise;
use crate::scenefile::SceneFile;
use super::{GalacticFrame, Photometry, luminance, mix, smoothstep, tangent_basis, warped_fbm};
use serde_json::Value;

/// The colours of the emission lines, as linear sRGB (made unit luminance where they are
/// used, `colour`): H-alpha at 656 nm is outside the gamut, a red with some blue added by
/// H-beta (486 nm) in the same gas; O III at 501 nm a blue-green.
const H_ALPHA: [f64; 3] = [1., 0.12, 0.22];
const O_III: [f64; 3] = [0.1, 0.85, 0.75];
const REFLECTION: [f64; 3] = [0.35, 0.55, 1.];

/// `c` scaled to unit luminance.
fn colour(c: [f64; 3]) -> Vector3<f64> {
    let c = Vector3::from(c);
    c / luminance(&c)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Emission,
    Reflection,
    Planetary,
    Dark,
}

pub struct Nebula {
    kind: Kind,
    centre: Vector3<f64>,
    /// Its angular radius, in radians.
    radius: f64,
    /// The cosine of the angle beyond which it shows nothing.
    cos_reach: f64,
    /// Its radiance (luminance) where its gas is densest.
    brightness: f64,
    /// The optical depth of its dust at its thickest.
    dust: f64,
    /// Axes across it, in the plane of the sky, for its shape.
    axes: (Vector3<f64>, Vector3<f64>),
    /// Its stretch along the first axis.
    elongation: f64,
    noise: SeededNoise,
}

fn kind(name: &str) -> Kind {
    match name {
        "emission" => Kind::Emission,
        "reflection" => Kind::Reflection,
        "planetary" => Kind::Planetary,
        "dark" => Kind::Dark,
        k => panic!("Unknown nebula type {:?}: expected \"emission\", \"reflection\", \"planetary\" or \"dark\"", k),
    }
}

impl Nebula {
    /// A nebula of `kind` at unit `centre`, `radius` radians across, of radiance `brightness`
    /// where densest, its shape from `seed`.
    pub fn new(kind: Kind, centre: Vector3<f64>, radius: f64, brightness: f64, dust: f64, seed: u64) -> Nebula {
        let mut rng = Pcg32::new(mix(seed), 0x9eb);
        let centre = centre.normalize();
        let (t, b) = tangent_basis(&centre);
        let angle = rng.next_f64() * std::f64::consts::TAU;
        let axes = (t * angle.cos() + b * angle.sin(), b * angle.cos() - t * angle.sin());
        let elongation = 1. + rng.next_f64() * if kind == Kind::Planetary { 0.3 } else { 0.8 };
        // The ragged edge reaches out to half as far again.
        let reach = (radius * 1.6 * elongation).min(std::f64::consts::PI);
        Nebula { kind, centre, radius, cos_reach: reach.cos(), brightness, dust, axes, elongation, noise: SeededNoise::new(mix(seed ^ 0xc10d)) }
    }

    /// From its description: `type`, `direction`, `radius` (degrees), `surface_brightness`
    /// (magnitudes per square arcsecond), `brightness` (scaling that), `dust`, `seed`.
    pub fn parse(o: &Value, seed: u64, photometry: &Photometry) -> Nebula {
        let k = kind(o["type"].as_str().unwrap_or("emission"));
        let (mu, dust) = defaults(k);
        let brightness = photometry.radiance(SceneFile::parse_number(&o["surface_brightness"], mu));
        Nebula::new(
            k,
            SceneFile::parse_vec3_def(o, "direction", Vector3::new(0., 0., 1.)),
            SceneFile::parse_number(&o["radius"], if k == Kind::Planetary { 0.5 } else { 5. }).to_radians(),
            brightness * SceneFile::parse_number(&o["brightness"], 1.),
            SceneFile::parse_number(&o["dust"], dust),
            SceneFile::parse_int(&o["seed"], seed as usize) as u64,
        )
    }

    /// The light the nebula emits in unit direction `d`, and the optical depth of its dust
    /// there.
    pub fn sample(&self, d: &Vector3<f64>) -> (Vector3<f64>, f64) {
        let c = d.dot(&self.centre);
        if c < self.cos_reach {
            return (Vector3::zeros(), 0.);
        }
        // Position across it, in radii (gnomonic: exact enough for patches a few degrees wide).
        let w = d / c - self.centre;
        let (x, y) = (w.dot(&self.axes.0) / (self.radius * self.elongation), w.dot(&self.axes.1) / self.radius);
        let r = (x * x + y * y).sqrt();
        // The noise is sampled in the nebula's own coordinates, so it scales with its size.
        let p = Vector3::new(x, y, 0.) * 2.2;
        let edge = r * (1. + 0.45 * self.noise.fbm(p * 0.8 + Vector3::new(3., 3., 3.), 3));
        match self.kind {
            Kind::Emission => {
                let gas = smoothstep(-0.25, 0.45, warped_fbm(&self.noise, p, 1.2, 6));
                let envelope = smoothstep(1.05, 0.2, edge);
                let density = gas * envelope;
                // Oxygen glows where the radiation is hardest, close in.
                let core = smoothstep(0.65, 0.0, edge) * gas;
                // Dark lanes of dust across it.
                let lanes = smoothstep(0.1, 0.45, self.noise.fbm(p * 1.7 + Vector3::new(-7., 2., 5.), 5)) * envelope;
                let light = colour(H_ALPHA) * density + colour(O_III) * (0.8 * core * core);
                let shaded = light * (self.brightness * (-2. * lanes).exp());
                (shaded, self.dust * lanes)
            },
            Kind::Reflection => {
                let gas = smoothstep(-0.3, 0.5, warped_fbm(&self.noise, p * 0.7, 0.8, 5));
                let density = gas * smoothstep(1.0, 0.0, edge).powf(1.5);
                (colour(REFLECTION) * (self.brightness * density), self.dust * density)
            },
            Kind::Planetary => {
                let wobble = 1. + 0.15 * self.noise.fbm(p * 1.5, 4);
                let rr = r * wobble;
                let shell = (-((rr - 0.65) / 0.18).powi(2)).exp();
                let inside = smoothstep(0.75, 0.2, rr);
                let star = (-(r / 0.04).powi(2)).exp() * 3.;
                let light = colour(H_ALPHA) * shell + colour(O_III) * (0.7 * inside) + Vector3::repeat(star);
                (light * self.brightness, 0.)
            },
            Kind::Dark => {
                let dust = smoothstep(-0.2, 0.4, warped_fbm(&self.noise, p, 1., 6)) * smoothstep(1.0, 0.3, edge);
                (Vector3::zeros(), self.dust * dust)
            },
        }
    }
}

impl Nebula {
    /// A bound on the luminance of its light in unit direction `d`: none outside its patch;
    /// within it, its densest gas, with the oxygen core (and the central star, for a
    /// planetary nebula) on top.
    pub fn bound(&self, d: &Vector3<f64>) -> f64 {
        if d.dot(&self.centre) < self.cos_reach {
            return 0.;
        }
        self.brightness * match self.kind {
            Kind::Emission => 1.8,
            Kind::Reflection => 1.,
            Kind::Planetary => 4.7,
            Kind::Dark => 0.,
        }
    }
}

/// The default surface brightness (magnitudes per square arcsecond) and dust of each kind.
fn defaults(k: Kind) -> (f64, f64) {
    match k {
        Kind::Emission => (21., 0.8),
        Kind::Reflection => (22., 0.3),
        Kind::Planetary => (19., 0.),
        Kind::Dark => (f64::INFINITY, 2.5),
    }
}

/// The nebulae described by `v`: none if it is `false`; a list of nebulae if it is a list; or
/// else (an object, or absent) `count` of them scattered at random about the galactic plane,
/// each a magnitude or so fainter than its kind's default at random, with `brightness` (scaling
/// their light) and `size` (the range of radii, in degrees) and `kinds` (the shares of
/// emission, reflection, planetary and dark nebulae).
pub fn parse(v: &Value, seed: u64, frame: &GalacticFrame, photometry: &Photometry) -> Vec<Nebula> {
    match v {
        Value::Bool(false) => vec![],
        Value::Array(list) => list.iter().enumerate().map(|(i, o)| Nebula::parse(o, mix(seed ^ (i as u64 + 1)), photometry)).collect(),
        _ => {
            let count = SceneFile::parse_int(&v["count"], 16);
            let scale = SceneFile::parse_number(&v["brightness"], 1.);
            let size = &v["size"];
            let (small, large) = (SceneFile::parse_number(&size[0], 3.), SceneFile::parse_number(&size[1], 12.));
            let shares = &v["kinds"];
            let shares = [
                SceneFile::parse_number(&shares[0], 0.55), SceneFile::parse_number(&shares[1], 0.2),
                SceneFile::parse_number(&shares[2], 0.1), SceneFile::parse_number(&shares[3], 0.15),
            ];
            let total: f64 = shares.iter().sum();
            let mut rng = Pcg32::new(mix(seed ^ 0x4eb), 0x4eb);
            (0..count).map(|i| {
                let mut pick = rng.next_f64() * total;
                let mut k = Kind::Emission;
                for (share, kind) in shares.iter().zip([Kind::Emission, Kind::Reflection, Kind::Planetary, Kind::Dark]) {
                    if pick < *share {
                        k = kind;
                        break;
                    }
                    pick -= share;
                }
                // Along the plane, crowded towards the centre; within a few degrees of it.
                let l = (rng.next_f64() * 2. - 1.).powi(2).copysign(rng.next_f64() - 0.5) * std::f64::consts::PI;
                let b = gaussian(&mut rng) * 0.08;
                let mut radius = small * (large / small).powf(rng.next_f64());
                if k == Kind::Planetary {
                    radius *= 0.12;
                }
                let (mu, dust) = defaults(k);
                let brightness = photometry.radiance(mu + 1.2 * rng.next_f64()) * scale;
                Nebula::new(k, frame.direction(l, b), radius.to_radians(), brightness, dust, mix(seed ^ (0x1000 + i as u64)))
            }).collect()
        },
    }
}

/// A standard normal variate (Box-Muller).
fn gaussian(rng: &mut Pcg32) -> f64 {
    let (u, v) = (rng.next_f64().max(1e-300), rng.next_f64());
    (-2. * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nebula_is_confined_to_its_patch() {
        for k in ["emission", "reflection", "planetary", "dark"] {
            let n = Nebula::parse(&serde_json::json!({ "type": k, "direction": [0, 0, 1], "radius": 5 }), 2, &Photometry::new(1.));
            let (light, dust) = n.sample(&Vector3::new(0., 1., 0.));
            assert_eq!((light, dust), (Vector3::zeros(), 0.));
            // Something shows within it.
            let total: f64 = (0..400).map(|i| {
                let (x, y) = ((i % 20) as f64 / 20. - 0.5, (i / 20) as f64 / 20. - 0.5);
                let (l, d) = n.sample(&Vector3::new(x * 0.15, y * 0.15, 1.).normalize());
                l.sum() + d
            }).sum();
            assert!(total > 0., "{} shows nothing", k);
        }
    }

    #[test]
    fn test_random_nebulae_lie_near_the_plane() {
        let f = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        let ns = parse(&serde_json::json!({ "count": 50 }), 1, &f, &Photometry::new(1.));
        assert_eq!(ns.len(), 50);
        assert!(ns.iter().all(|n| n.centre.y.abs() < 0.5));
        assert!(ns.iter().any(|n| n.kind == Kind::Dark) && ns.iter().any(|n| n.kind == Kind::Emission));
    }
}
