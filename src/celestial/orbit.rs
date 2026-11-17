//! Keplerian orbits: where a body is, from its orbital elements and the time.
//!
//! The six classical elements (semi-major axis a, eccentricity e, inclination i, longitude of
//! the ascending node Ω, argument of periapsis ω, and the mean anomaly M at time zero) fix an
//! ellipse about the primary and a body's place on it. The mean anomaly grows uniformly, by the
//! mean motion n = √(GM / a³), and Kepler's equation M = E - e sin E gives the eccentric
//! anomaly E and so the position (Vallado, "Fundamentals of Astrodynamics and Applications",
//! 4th ed., 2013, sections 2.2-2.3 and algorithm 10).
//!
//! The elements are measured in a reference frame: a plane (its `pole`) and a direction in it
//! (the `equinox`) from which Ω is measured, given in the scene's coordinates.

use crate::na::Vector3;
use crate::scenefile::SceneFile;
use serde_json::Value;
use std::f64::consts::PI;

/// The Earth's gravitational parameter GM, m³/s² (IERS Conventions 2010).
pub const EARTH_GM: f64 = 3.986_004_418e14;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit {
    /// Semi-major axis, metres.
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    /// Inclination of the orbit to the reference plane, radians.
    pub inclination: f64,
    /// Longitude of the ascending node, radians from the equinox.
    pub ascending_node: f64,
    /// Argument of periapsis, radians from the ascending node.
    pub argument_of_periapsis: f64,
    /// Mean anomaly at time zero, radians.
    pub mean_anomaly: f64,
    /// The primary's gravitational parameter GM, m³/s².
    pub gm: f64,
    /// The reference plane's pole and equinox direction, unit and perpendicular.
    pub pole: Vector3<f64>,
    pub equinox: Vector3<f64>,
}

impl Orbit {
    /// The Moon's mean orbit about the Earth (Allen's Astrophysical Quantities, 4th ed.,
    /// section 12.4: a = 384,400 km, e = 0.0549, inclination to the ecliptic 5.145°), here
    /// measured from a reference plane with the scene's y as its pole.
    pub fn moon() -> Orbit {
        Orbit {
            semi_major_axis: 384_400e3, eccentricity: 0.0549, inclination: 5.145f64.to_radians(),
            ascending_node: 0., argument_of_periapsis: 0., mean_anomaly: 0., gm: EARTH_GM,
            pole: Vector3::y(), equinox: Vector3::x(),
        }
    }

    /// From a scene's `orbit` object, with the Moon's for anything left out; angles in degrees.
    pub fn parse(o: &Value) -> Orbit {
        let m = Orbit::moon();
        let deg = |k: &str, def: f64| SceneFile::parse_number(&o[k], def.to_degrees()).to_radians();
        let pole = SceneFile::parse_vec3_def(o, "pole", m.pole).normalize();
        let equinox = SceneFile::parse_vec3_def(o, "equinox", m.equinox);
        let equinox = (equinox - pole * equinox.dot(&pole)).normalize();
        Orbit {
            semi_major_axis: SceneFile::parse_number(&o["semi_major_axis"], m.semi_major_axis),
            eccentricity: SceneFile::parse_number(&o["eccentricity"], m.eccentricity).clamp(0., 0.99),
            inclination: deg("inclination", m.inclination),
            ascending_node: deg("ascending_node", m.ascending_node),
            argument_of_periapsis: deg("argument_of_periapsis", m.argument_of_periapsis),
            mean_anomaly: deg("mean_anomaly", m.mean_anomaly),
            gm: SceneFile::parse_number(&o["gm"], m.gm),
            pole, equinox,
        }
    }

    /// The orbital period, seconds.
    pub fn period(&self) -> f64 {
        2. * PI * (self.semi_major_axis.powi(3) / self.gm).sqrt()
    }

    /// The eccentric anomaly for mean anomaly `m`: Kepler's equation by Newton's method.
    pub fn eccentric_anomaly(&self, m: f64) -> f64 {
        let e = self.eccentricity;
        let m = m.rem_euclid(2. * PI);
        let mut x = if e < 0.8 { m } else { PI };
        for _ in 0..30 {
            let dx = (x - e * x.sin() - m) / (1. - e * x.cos());
            x -= dx;
            if dx.abs() < 1e-14 {
                break;
            }
        }
        x
    }

    /// The position relative to the primary at `time` seconds, in the scene's axes.
    pub fn position(&self, time: f64) -> Vector3<f64> {
        let m = self.mean_anomaly + time * (self.gm / self.semi_major_axis.powi(3)).sqrt();
        let e = self.eccentricity;
        let ea = self.eccentric_anomaly(m);
        // In the orbit's plane, periapsis along x.
        let (x, y) = (self.semi_major_axis * (ea.cos() - e), self.semi_major_axis * (1. - e * e).sqrt() * ea.sin());
        // Rotate by ω, tilt by i about the line of nodes, then turn by Ω about the pole.
        let third = self.pole.cross(&self.equinox);
        let (so, co) = self.argument_of_periapsis.sin_cos();
        let (si, ci) = self.inclination.sin_cos();
        let (sn, cn) = self.ascending_node.sin_cos();
        let (x1, y1) = (x * co - y * so, x * so + y * co);
        let (y2, z2) = (y1 * ci, y1 * si);
        let (x3, y3) = (x1 * cn - y2 * sn, x1 * sn + y2 * cn);
        self.equinox * x3 + third * y3 + self.pole * z2
    }

    /// The pole of the orbit itself: perpendicular to its plane, along its angular momentum.
    pub fn normal(&self) -> Vector3<f64> {
        let a = self.position(0.);
        let b = self.position(self.period() / 4.);
        a.cross(&b).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moons_period() {
        // The sidereal month, 27.32 days (the Moon's own mass, which shortens it by 0.6%, left
        // out).
        let days = Orbit::moon().period() / 86400.;
        assert!((days - 27.32).abs() < 0.3, "{}", days);
    }

    #[test]
    fn test_kepler_equation() {
        let o = Orbit { eccentricity: 0.6, ..Orbit::moon() };
        for &m in &[0., 0.3, 2., 3.1, 5.] {
            let e = o.eccentric_anomaly(m);
            assert!((e - 0.6 * e.sin() - m).abs() < 1e-12);
        }
    }

    #[test]
    fn test_periapsis_and_apoapsis() {
        let o = Orbit::moon();
        let a = o.semi_major_axis;
        assert!((o.position(0.).norm() - a * (1. - o.eccentricity)).abs() < 1.);
        assert!((o.position(o.period() / 2.).norm() - a * (1. + o.eccentricity)).abs() < 1.);
        // A whole period returns it.
        assert!((o.position(o.period()) - o.position(0.)).norm() < 1.);
    }

    #[test]
    fn test_inclination_tilts_the_plane() {
        let o = Orbit { inclination: 30f64.to_radians(), ascending_node: 0.5, ..Orbit::moon() };
        assert!((o.normal().dot(&o.pole) - 30f64.to_radians().cos()).abs() < 1e-9);
        // The ascending node is where it crosses the plane going up.
        let flat = Orbit { inclination: 0., ..Orbit::moon() };
        for t in [0., 1e5, 1e6] {
            assert!(flat.position(t).dot(&flat.pole).abs() < 1e-6);
        }
    }
}
