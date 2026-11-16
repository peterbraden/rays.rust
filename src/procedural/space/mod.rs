//! # The rest of the universe
//!
//! What lies beyond the atmosphere, as a background (`"background": {"type": "space"}`): a
//! field of stars, the Milky Way's glow and dust, nebulae and distant galaxies. All of it is a
//! function of direction alone (it is infinitely far off), generated from a seed, so the same
//! seed gives the same sky from anywhere in the scene.
//!
//! Each part is in its own module:
//!
//! - `stars`: point sources scattered over the sphere, by a luminosity function, coloured by
//!   their temperature, crowded towards the galactic plane;
//! - `milky_way`: the diffuse light of unresolved stars in the galactic disc and bulge, and
//!   the dust lanes that dim and redden what lies behind them;
//! - `nebula`: emission (hydrogen-alpha and oxygen III), reflection, planetary and dark nebulae;
//! - `galaxy`: spiral and elliptical galaxies, inclined discs seen in projection.
//!
//! They are placed in a galactic frame: a pole (the direction perpendicular to the galactic
//! plane) and the galactic centre, given in the scene's coordinates, so the band of the Milky
//! Way can be laid across the sky wherever the scene wants it.
//!
//! ## Brightness
//!
//! Everything is on a physical scale, relative to the sun's irradiance (`Photometry`): stars
//! by their magnitudes, and extended objects by their surface brightnesses in magnitudes per
//! square arcsecond, both against the sun's apparent magnitude, -26.74. So, beside a sunlit
//! scene, they are as faint as they are in life: the Milky Way's brightest clouds, at about 20
//! mag/arcsec², are some 10^-8 of the sun's irradiance per steradian, and a sixth-magnitude
//! star 10^-13 of it. Seeing them takes the long exposure (`output.exposure`) a night sky
//! takes. `brightness` scales it all, for when looks matter more.
//!
//! The light composes as it would: the stars, the Milky Way's glow and the galaxies are seen
//! through the dust (the Milky Way's lanes and any dark nebulae), and the emission nebulae glow
//! in front of it. Seen from inside an atmosphere, the skysphere lets all of this through by
//! the transmittance of its air (`skysphere`): by day the sky drowns it; at night, and from
//! orbit, it shows.

pub mod galaxy;
pub mod milky_way;
pub mod nebula;
pub mod stars;

use crate::color::Color;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::scenefile::SceneFile;
use serde_json::Value;
use std::f64::consts::PI;

/// The galactic coordinate frame: unit vectors to the galactic centre and north pole, and the
/// third axis completing a right-handed set (towards galactic longitude 90°).
#[derive(Debug, Clone, Copy)]
pub struct GalacticFrame {
    pub centre: Vector3<f64>,
    pub pole: Vector3<f64>,
    pub east: Vector3<f64>,
}

impl GalacticFrame {
    /// The frame for `pole` and a direction `centre` towards the galactic centre (made
    /// perpendicular to the pole).
    pub fn new(pole: Vector3<f64>, centre: Vector3<f64>) -> GalacticFrame {
        let pole = pole.normalize();
        let mut c = centre - pole * centre.dot(&pole);
        if c.norm() < 1e-6 {
            c = tangent_basis(&pole).0;
        }
        let centre = c.normalize();
        GalacticFrame { centre, pole, east: pole.cross(&centre) }
    }

    /// Galactic longitude (from the centre, in (-π, π]) and latitude (in [-π/2, π/2]) of unit
    /// direction `d`, in radians.
    pub fn coordinates(&self, d: &Vector3<f64>) -> (f64, f64) {
        let b = d.dot(&self.pole).clamp(-1., 1.).asin();
        let l = d.dot(&self.east).atan2(d.dot(&self.centre));
        (l, b)
    }

    /// The unit direction at galactic longitude `l` and latitude `b`.
    pub fn direction(&self, l: f64, b: f64) -> Vector3<f64> {
        (self.centre * l.cos() + self.east * l.sin()) * b.cos() + self.pole * b.sin()
    }
}

/// The sun's apparent visual magnitude (V), seen from the Earth: the reference against which
/// the magnitudes of everything else become irradiances. Allen's Astrophysical Quantities, 4th
/// ed., 2000, section 14.7.
pub const SUN_MAGNITUDE: f64 = -26.74;
/// One square arcsecond, in steradians: (π / 648000)².
pub const ARCSEC2: f64 = 2.350443e-11;

/// Magnitudes as light, on the scene's scale: a source of magnitude m gives an irradiance
/// 10^(-0.4 (m - m_sun)) times the sun's, and a surface of m magnitudes per square arcsecond
/// that irradiance from each square arcsecond.
#[derive(Debug, Clone, Copy)]
pub struct Photometry {
    /// The irradiance of a magnitude-zero source.
    zero: f64,
}

impl Photometry {
    /// For a sun whose irradiance (its luminance, as the scene's `sun` gives it) is
    /// `sun_irradiance`.
    pub fn new(sun_irradiance: f64) -> Photometry {
        Photometry { zero: sun_irradiance * 10f64.powf(0.4 * SUN_MAGNITUDE) }
    }

    /// The irradiance of a point source of magnitude `m`.
    pub fn irradiance(&self, m: f64) -> f64 {
        self.zero * 10f64.powf(-0.4 * m)
    }

    /// The radiance of a surface of `mu` magnitudes per square arcsecond.
    pub fn radiance(&self, mu: f64) -> f64 {
        self.irradiance(mu) / ARCSEC2
    }
}

/// Two unit vectors perpendicular to unit `n` and to each other.
pub fn tangent_basis(n: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if n.x.abs() < 0.9 { Vector3::new(1., 0., 0.) } else { Vector3::new(0., 1., 0.) };
    let t = n.cross(&a).normalize();
    (t, n.cross(&t))
}

/// A 64-bit hash of `x` (splitmix64's finaliser), for seeding generators from cell indices.
pub fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Directions spread evenly over the sphere: a Fibonacci lattice of `n` points.
pub fn fibonacci_sphere(n: usize) -> impl Iterator<Item = Vector3<f64>> {
    let golden = PI * (3. - 5f64.sqrt());
    (0..n).map(move |i| {
        let y = 1. - 2. * (i as f64 + 0.5) / n as f64;
        let r = (1. - y * y).max(0.).sqrt();
        let phi = golden * i as f64;
        Vector3::new(r * phi.cos(), y, r * phi.sin())
    })
}

/// The colour of a black body at `kelvin`, as linear sRGB of unit luminance (Y = 1).
///
/// Planck's law, integrated against the CIE 1931 colour-matching functions in the multi-lobe
/// Gaussian fit of Chris Wyman, Peter-Pike Sloan and Peter Shirley, "Simple Analytic
/// Approximations to the CIE XYZ Color Matching Functions", JCGT 2(2), 2013, then converted
/// from XYZ to linear sRGB (IEC 61966-2-1). Colours outside the sRGB gamut (the reddest) are
/// clipped to it.
pub fn blackbody(kelvin: f64) -> Vector3<f64> {
    fn g(x: f64, mu: f64, s1: f64, s2: f64) -> f64 {
        let t = (x - mu) / if x < mu { s1 } else { s2 };
        (-0.5 * t * t).exp()
    }
    let mut xyz = Vector3::zeros();
    let mut lambda = 380.;
    while lambda <= 780. {
        let x = 1.056 * g(lambda, 599.8, 37.9, 31.0) + 0.362 * g(lambda, 442.0, 16.0, 26.7) - 0.065 * g(lambda, 501.1, 20.4, 26.2);
        let y = 0.821 * g(lambda, 568.8, 46.9, 40.5) + 0.286 * g(lambda, 530.9, 16.3, 31.1);
        let z = 1.217 * g(lambda, 437.0, 11.8, 36.0) + 0.681 * g(lambda, 459.0, 26.0, 13.8);
        let m = lambda * 1e-9;
        // Planck's law, up to a constant: c2 = hc/k = 1.4388e-2 m K.
        let planck = 1. / (m.powi(5) * ((1.4388e-2 / (m * kelvin)).exp() - 1.));
        xyz += Vector3::new(x, y, z) * planck;
        lambda += 5.;
    }
    let xyz = xyz / xyz.y;
    let rgb = Vector3::new(
        3.2406 * xyz.x - 1.5372 * xyz.y - 0.4986 * xyz.z,
        -0.9689 * xyz.x + 1.8758 * xyz.y + 0.0415 * xyz.z,
        0.0557 * xyz.x - 0.2040 * xyz.y + 1.0570 * xyz.z,
    ).map(|c| c.max(0.));
    rgb / luminance(&rgb)
}

/// Relative luminance of linear sRGB.
pub fn luminance(c: &Vector3<f64>) -> f64 {
    0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z
}

/// Hermite smoothstep from `a` to `b`.
pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Noise displaced by noise: fbm at `p` moved by three more fbm fields, `strength` far. After
/// Inigo Quilez, "Domain Warping", 2002 (<https://iquilezles.org/articles/warp/>): it turns the
/// blobs of plain fbm into the swirls and filaments of gas.
pub fn warped_fbm(noise: &SeededNoise, p: Vector3<f64>, strength: f64, octaves: u32) -> f64 {
    let q = Vector3::new(
        noise.fbm(p + Vector3::new(1.7, 9.2, 3.1), 4),
        noise.fbm(p + Vector3::new(8.3, 2.8, 5.4), 4),
        noise.fbm(p + Vector3::new(4.6, 7.1, 0.9), 4),
    );
    noise.fbm(p + q * strength, octaves)
}

/// The sky beyond the atmosphere: stars, the Milky Way, nebulae and galaxies.
pub struct Space {
    pub frame: GalacticFrame,
    /// Scales all of it (1 is the physical scale).
    pub brightness: f64,
    /// Where seen through a skysphere: the share of the sky's own light below which the light
    /// from space is not worth computing (it can add at most this much).
    pub threshold: f64,

    pub stars: Option<stars::Stars>,
    pub milky_way: Option<milky_way::MilkyWay>,
    pub nebulae: Vec<nebula::Nebula>,
    pub galaxies: Vec<galaxy::Galaxy>,
}

impl std::fmt::Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Space")
            .field("stars", &self.stars.as_ref().map(|s| s.count))
            .field("milky_way", &self.milky_way.is_some())
            .field("nebulae", &self.nebulae.len())
            .field("galaxies", &self.galaxies.len())
            .finish()
    }
}

/// Whether a part of the description is turned off: `false` or `null` where an object was
/// expected (absent keys are on, with defaults).
fn off(v: &Value) -> bool {
    v.as_bool() == Some(false)
}

impl Space {
    /// Space from its description (the `background` object), for a camera whose pixels are
    /// `pixel_angle` radians across (0 if unknown): stars are drawn about a pixel wide. The
    /// photometric reference is `sun_irradiance` if given, else the scene's sun's
    /// (`scene_sun`, its luminance), else 1.
    pub fn parse(o: &Value, pixel_angle: f64, scene_sun: Option<f64>) -> Space {
        let seed = SceneFile::parse_int(&o["seed"], 0) as u64;
        let frame = GalacticFrame::new(
            SceneFile::parse_vec3_def(o, "galactic_pole", Vector3::new(-0.35, 0.55, 0.75)),
            SceneFile::parse_vec3_def(o, "galactic_centre", Vector3::new(0.9, 0.2, -0.3)),
        );
        let pixel = if pixel_angle > 0. { pixel_angle } else { 1e-3 };
        let photometry = Photometry::new(SceneFile::parse_number(&o["sun_irradiance"], scene_sun.unwrap_or(1.)));
        let milky_way = if off(&o["milky_way"]) { None } else { Some(milky_way::MilkyWay::parse(&o["milky_way"], seed, &photometry)) };
        let stars = if off(&o["stars"]) { None } else { Some(stars::Stars::parse(&o["stars"], seed, pixel, &frame, &photometry)) };
        let nebulae = nebula::parse(&o["nebulae"], seed, &frame, &photometry);
        let galaxies = galaxy::parse(&o["galaxies"], seed, &frame, &photometry);
        Space {
            frame, brightness: SceneFile::parse_number(&o["brightness"], 1.),
            threshold: SceneFile::parse_number(&o["threshold"], 1e-3),
            stars, milky_way, nebulae, galaxies,
        }
    }

    /// A bound on the luminance of `radiance(d)`, far cheaper to find: no noise, and no stars
    /// generated. Dust only dims, so it is left out.
    pub fn bound(&self, d: &Vector3<f64>) -> f64 {
        let (l, b) = self.frame.coordinates(d);
        self.brightness * (self.stars.as_ref().map_or(0., |s| s.bound(d))
            + self.milky_way.as_ref().map_or(0., |m| m.bound(l, b))
            + self.nebulae.iter().map(|n| n.bound(d)).sum::<f64>()
            + self.galaxies.iter().map(|g| g.bound(d)).sum::<f64>())
    }

    /// The radiance arriving from unit direction `d`.
    pub fn radiance(&self, d: &Vector3<f64>) -> Color {
        let (l, b) = self.frame.coordinates(d);
        // Light from behind the dust.
        let mut behind = Vector3::zeros();
        if let Some(s) = &self.stars {
            behind += s.radiance(d, &self.frame);
        }
        if let Some(m) = &self.milky_way {
            behind += m.glow(d, l, b);
        }
        for g in &self.galaxies {
            behind += g.radiance(d);
        }
        // Optical depth of the dust in front of it (at green; redder light gets through more).
        let mut tau = self.milky_way.as_ref().map_or(0., |m| m.dust(d, l, b));
        let mut front = Vector3::zeros();
        for n in &self.nebulae {
            let (emitted, depth) = n.sample(d);
            front += emitted;
            tau += depth;
        }
        let transmitted = behind.component_mul(&reddening(tau));
        let v = (transmitted + front) * self.brightness;
        Color::new(v.x, v.y, v.z)
    }
}

/// The fraction of light that crosses interstellar dust of optical depth `tau` (at green), per
/// channel: extinction falls with wavelength, roughly as 1/λ (the reddening law of Cardelli,
/// Clayton and Mathis, ApJ 345, 1989, in the visible), so what gets through is reddened.
pub fn reddening(tau: f64) -> Vector3<f64> {
    if tau <= 0. {
        return Vector3::repeat(1.);
    }
    // 1/λ at 680, 550 and 440 nm, relative to 550.
    Vector3::new((-tau * 0.81).exp(), (-tau).exp(), (-tau * 1.25).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galactic_coordinates_round_trip() {
        let f = GalacticFrame::new(Vector3::new(0.1, 1., 0.2), Vector3::new(1., 0., 0.));
        for &(l, b) in &[(0., 0.), (1., 0.3), (-2.5, -1.2), (3., 0.)] {
            let (l2, b2) = f.coordinates(&f.direction(l, b));
            assert!((l - l2).abs() < 1e-9 && (b - b2).abs() < 1e-9, "{} {} -> {} {}", l, b, l2, b2);
        }
        assert!((f.coordinates(&f.pole).1 - PI / 2.).abs() < 1e-6);
    }

    #[test]
    fn test_blackbody_colours() {
        // Cool stars are red, hot ones blue, the sun nearly white; all of unit luminance.
        let (m, sun, b) = (blackbody(3000.), blackbody(5800.), blackbody(20000.));
        assert!(m.x > m.y && m.y > m.z, "{}", m);
        assert!(b.z > b.y && b.y > b.x, "{}", b);
        assert!((sun.x - sun.z).abs() < 0.35 * sun.y, "{}", sun);
        for c in [m, sun, b] {
            assert!((luminance(&c) - 1.).abs() < 1e-9);
        }
        // D65 is close to a 6500 K black body: near-equal channels.
        let d65 = blackbody(6504.);
        assert!((d65.x - d65.z).abs() < 0.15, "{}", d65);
    }

    #[test]
    fn test_photometry() {
        // The sun itself, and a star five magnitudes brighter than another is 100 times as
        // bright.
        let p = Photometry::new(6.);
        assert!((p.irradiance(SUN_MAGNITUDE) - 6.).abs() < 1e-9);
        assert!((p.irradiance(1.) / p.irradiance(6.) - 100.).abs() < 1e-9);
        // 21 mag/arcsec² is about 4.3e-4 cd/m² for a sun of 1.27e5 lux (Allen, 4th ed.,
        // section 15.2: 10.8e4 x 10^(-0.4 mu) cd/m² for mu in V).
        let night = Photometry::new(1.27e5).radiance(21.);
        assert!((night / (10.8e4 * 10f64.powf(-0.4 * 21.)) - 1.).abs() < 0.05, "{}", night);
    }

    #[test]
    fn test_physical_scale_and_brightness() {
        // Beside a sun of irradiance 1, the Milky Way is some 10^-8 per steradian; brightness
        // scales it all.
        let o = |b: f64| serde_json::json!({ "type": "space", "brightness": b, "stars": false, "nebulae": false, "galaxies": false,
            "galactic_pole": [0, 1, 0], "galactic_centre": [1, 0, 0] });
        let (one, more) = (Space::parse(&o(1.), 1e-3, None), Space::parse(&o(1e8), 1e-3, None));
        let d = Vector3::new(1., 0., 0.);
        let v = luminance(&one.radiance(&d).rgb);
        assert!(v > 1e-9 && v < 1e-7, "{}", v);
        assert!((luminance(&more.radiance(&d).rgb) / v - 1e8).abs() < 1e-3);
        // The scene's sun sets the scale, unless sun_irradiance is given.
        let lit = Space::parse(&o(1.), 1e-3, Some(6.));
        assert!((luminance(&lit.radiance(&d).rgb) / v - 6.).abs() < 1e-9);
        let mut given = o(1.);
        given["sun_irradiance"] = serde_json::json!(2.);
        assert!((luminance(&Space::parse(&given, 1e-3, Some(6.)).radiance(&d).rgb) / v - 2.).abs() < 1e-9);
    }

    #[test]
    fn test_reddening() {
        let t = reddening(1.);
        assert!(t.x > t.y && t.y > t.z);
        assert_eq!(reddening(0.), Vector3::repeat(1.));
    }

    #[test]
    fn test_space_is_deterministic_and_finite() {
        let o = serde_json::json!({ "type": "space", "seed": 3 });
        let (a, b) = (Space::parse(&o, 2e-3, None), Space::parse(&o, 2e-3, None));
        for d in fibonacci_sphere(20000) {
            let (x, y) = (a.radiance(&d), b.radiance(&d));
            assert_eq!(x, y);
            assert!(x.rgb.iter().all(|c| c.is_finite() && *c >= 0.), "{} at {}", x, d);
            // Within its bound.
            assert!(luminance(&x.rgb) <= a.bound(&d) * (1. + 1e-9), "{} vs {} at {}", luminance(&x.rgb), a.bound(&d), d);
        }
    }

    #[test]
    fn test_parts_can_be_turned_off() {
        let o = serde_json::json!({ "type": "space", "stars": false, "milky_way": false, "nebulae": false, "galaxies": false });
        let s = Space::parse(&o, 1e-3, None);
        assert!(fibonacci_sphere(500).all(|d| s.bound(&d) == 0.));
        assert!(fibonacci_sphere(500).all(|d| s.radiance(&d) == Color::black()));
    }
}
