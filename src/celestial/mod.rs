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

pub mod hapke;
pub mod moon;
pub mod orbit;
pub mod surface;

use crate::na::{Matrix3, Vector3};

/// The Moon's mean radius, metres (IAU WGCCRE 2015).
pub const MOON_RADIUS: f64 = 1_737_400.;
/// The Moon's mean distance from the Earth's centre, metres.
pub const MOON_DISTANCE: f64 = 384_400_000.;

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
