//! # Celestial bodies
//!
//! Worlds other than the one the scene stands on: for now, moons. A body is real geometry, a
//! sphere of its true size at its true distance (the Moon is 1737 km in radius, 384,400 km
//! away), so everything that follows from that does: it rises and sets behind the planet,
//! the air in front of it hazes and reddens it (aerial perspective, so by day it is a pale disc
//! against the blue), it hides the sun in an eclipse, and the planet's shadow falls on it.
//!
//! Its surface is generated from a seed (`surface`): craters of every size, basins, maria and
//! rays. Its light is the sun's, reflected by Hapke's model of regolith (`hapke`), fitted to the
//! Moon's phase curve, so its brightness through its phases and against the sky is physical: the
//! full moon is some 2.5·10⁻⁶ of the sun's irradiance (magnitude -12.7 against -26.7), and so,
//! like the `space` background, shows at its true brightness beside a sunlit sky and needs a
//! night exposure on its own. Earthshine lights its night side.
//!
//! Where it is: in a `direction` as seen from the scene's origin, at its `distance` from the
//! planet's centre, or on a Keplerian `orbit` (`orbit`) at a `time`. It keeps the same face to
//! its planet (tidal locking), with `libration` to turn it.
//!
//! This is the start of a universe's worth of bodies: the pieces (orbits, photometry, the
//! cratered surface) are general, and the Moon is one set of their parameters.

pub mod bands;
pub mod giant;
pub mod hapke;
pub mod moon;
pub mod orbit;
pub mod rings;
pub mod shape;
pub mod surface;
pub mod terrain;

use crate::na::{Matrix3, Vector3};
use crate::scenefile::SceneFile;
use serde_json::Value;

/// Where a body described by `o` is, and the pole of its orbit (the scene's y if it has none):
/// at its `position`; on its `orbit` about the planet at `planet` at its `time`; or seen in its
/// `direction` from `from` (default the origin) at `distance` (default `default_distance`)
/// from the planet's centre.
pub fn place(o: &Value, planet: &Vector3<f64>, default_distance: f64) -> (Vector3<f64>, Vector3<f64>) {
    if let Some(p) = o.get("position") {
        (SceneFile::parse_vec3(p), Vector3::y())
    } else if let Some(orbit) = o.get("orbit") {
        let orbit = crate::celestial::orbit::Orbit::parse(orbit);
        (planet + orbit.position(SceneFile::parse_number(&o["time"], 0.)), orbit.normal())
    } else {
        let d = SceneFile::parse_vec3_def(o, "direction", Vector3::new(0., 0.5, 1.)).normalize();
        let distance = SceneFile::parse_number(&o["distance"], default_distance);
        let from = SceneFile::parse_vec3_def(o, "from", Vector3::zeros());
        (along_to_distance(&from, &d, planet, distance)
            .expect("A body must be farther from its planet's centre than the point it's seen from"), Vector3::y())
    }
}

/// The Moon's mean radius, metres (IAU WGCCRE 2015).
pub const MOON_RADIUS: f64 = 1_737_400.;
/// The Moon's mean distance from the Earth's centre, metres.
pub const MOON_DISTANCE: f64 = 384_400_000.;

/// The kinds of world `world_object` chooses among, and how often: gas and ice giants, rocky
/// and icy bodies.
pub const WORLD_KINDS: [(&str, &str, f64); 4] = [("gas_giant", "gas_giant", 0.25), ("gas_giant", "ice_giant", 0.2), ("moon", "rocky", 0.3), ("moon", "icy", 0.25)];

/// A `world` object (`{"type": "world", "seed": n, ...}`) as the object it turns out to be:
/// its kind (gas giant, ice giant, rocky, icy, or `kind` if given) drawn from its seed, and
/// everything about it from the same seed; its other keys (placement, camera) kept.
pub fn world_object(o: &Value) -> Value {
    let seed = SceneFile::parse_number(&o["seed"], 0.) as u64;
    let u = (crate::procedural::space::mix(seed ^ 0x776f_726c) >> 11) as f64 / (1u64 << 53) as f64;
    let chosen = match o["kind"].as_str() {
        Some(k) => *WORLD_KINDS.iter().find(|w| w.1 == k)
            .unwrap_or_else(|| panic!("A world's kind is gas_giant, ice_giant, rocky or icy, not {:?}", k)),
        None => {
            let mut acc = 0.;
            *WORLD_KINDS.iter().find(|w| { acc += w.2; u < acc }).unwrap_or(&WORLD_KINDS[3])
        },
    };
    let mut out = o.clone();
    out["type"] = Value::from(chosen.0);
    out["preset"] = Value::from(chosen.1);
    if chosen.0 == "moon" && o.get("earthshine").is_none() {
        // A world of its own, not a moon of the scene's planet.
        out["earthshine"] = Value::from(false);
    }
    out
}

/// The point at `distance` from `centre` along the ray from `from` in unit direction `d`: where
/// a body seen in that direction from there would be. None if there is none ahead (`from` is
/// farther from the centre than `distance`, looking away).
pub fn along_to_distance(from: &Vector3<f64>, d: &Vector3<f64>, centre: &Vector3<f64>, distance: f64) -> Option<Vector3<f64>> {
    let o = from - centre;
    let b = o.dot(d);
    let disc = b * b - (o.norm_squared() - distance * distance);
    if disc < 0. {
        return None;
    }
    let t = -b + disc.sqrt();
    (t > 0.).then(|| from + d * t)
}

/// A tidally locked body's axes in the scene: z towards its planet, y as near `up` (its orbit's
/// pole) as is perpendicular to that, and x completing them; then turned by `libration`
/// (longitude, latitude, radians), which brings that much of the far side into view.
pub fn locked_frame(centre: &Vector3<f64>, planet: &Vector3<f64>, up: &Vector3<f64>, libration: (f64, f64)) -> Matrix3<f64> {
    let z = (planet - centre).normalize();
    let mut y = up - z * up.dot(&z);
    if y.norm() < 1e-9 {
        y = crate::procedural::space::tangent_basis(&z).0;
    }
    let y = y.normalize();
    let x = y.cross(&z);
    let frame = Matrix3::from_columns(&[x, y, z]);
    // Turning the body by +longitude about its pole moves its face towards -x: a point at
    // that longitude east comes to face the planet.
    let turn = crate::na::Rotation3::from_axis_angle(&Vector3::y_axis(), -libration.0)
        * crate::na::Rotation3::from_axis_angle(&Vector3::x_axis(), libration.1);
    frame * turn.matrix()
}

/// The share of a disc of angular radius `a` left uncovered by a disc of angular radius `b`
/// whose centre is `separation` away (radians; small angles, so flat geometry).
pub fn uncovered(a: f64, b: f64, separation: f64) -> f64 {
    let d = separation;
    if d >= a + b {
        return 1.;
    }
    if d <= b - a {
        return 0.;
    }
    if d <= a - b {
        return 1. - (b / a).powi(2);
    }
    // The lens where they overlap.
    let (a2, b2, d2) = (a * a, b * b, d * d);
    let alpha = ((d2 + a2 - b2) / (2. * d * a)).clamp(-1., 1.).acos();
    let beta = ((d2 + b2 - a2) / (2. * d * b)).clamp(-1., 1.).acos();
    let lens = a2 * (alpha - alpha.sin() * alpha.cos()) + b2 * (beta - beta.sin() * beta.cos());
    (1. - lens / (std::f64::consts::PI * a2)).clamp(0., 1.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_along_to_distance() {
        // From the surface of a planet, straight up: the distance less its radius.
        let c = Vector3::new(0., -6_360e3, 0.);
        let p = along_to_distance(&Vector3::zeros(), &Vector3::y(), &c, MOON_DISTANCE).unwrap();
        assert!((p.y - (MOON_DISTANCE - 6_360e3)).abs() < 1e-3);
        // On the horizon it is farther: the planet's radius is out of line.
        let h = along_to_distance(&Vector3::zeros(), &Vector3::x(), &c, MOON_DISTANCE).unwrap();
        assert!(((h - c).norm() - MOON_DISTANCE).abs() < 1e-3 && h.norm() > p.norm());
    }

    #[test]
    fn test_locked_frame_faces_the_planet() {
        let centre = Vector3::new(1e8, 2e8, -3e8);
        let f = locked_frame(&centre, &Vector3::zeros(), &Vector3::y(), (0., 0.));
        let z = f * Vector3::z();
        assert!((z + centre.normalize()).norm() < 1e-12);
        assert!((f.determinant() - 1.).abs() < 1e-12);
        // Libration in longitude brings the east limb towards the planet.
        let l = locked_frame(&centre, &Vector3::zeros(), &Vector3::y(), (0.1, 0.));
        let east = l * Vector3::new(0.1f64.sin(), 0., 0.1f64.cos());
        assert!((east - z).norm() < 1e-9, "{} {}", east, z);
    }

    #[test]
    fn test_uncovered() {
        assert_eq!(uncovered(1., 1., 3.), 1.);
        assert_eq!(uncovered(1., 2., 0.5), 0.);
        assert!((uncovered(1., 0.5, 0.) - 0.75).abs() < 1e-12);
        // Two equal discs half overlapping, and continuity at the edges.
        let half = uncovered(1., 1., 1.);
        assert!(half > 0.55 && half < 0.65, "{}", half);
        assert!((uncovered(1., 1., 2. - 1e-9) - 1.).abs() < 1e-6);
        assert!(uncovered(1., 3., 2. + 1e-9) < 1e-6);
    }
}
