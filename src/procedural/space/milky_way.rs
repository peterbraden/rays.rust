//! # The Milky Way
//!
//! Seen from inside it, the galaxy is a band of light around the sky: the glow of stars too
//! many and too faint to see one by one, in a thin disc, swelling into the bulge about the
//! galactic centre. Dust in the plane of the disc, between us and those stars, splits the band
//! with dark lanes (the Great Rift, in our sky) and reddens what gets through.
//!
//! The glow is a disc whose brightness falls exponentially with galactic latitude, by a scale
//! that widens away from the centre (as the disc flares, and as nearer parts of it cover more
//! of the sky), brightest towards the centre, plus a bulge, a Gaussian about the centre. Both
//! are mottled into star clouds by noise. Their colours are those of their populations: the
//! bulge's old, cool stars yellow; the disc's, with its young hot stars, whiter.
//!
//! The dust is a narrower layer along the plane, its density broken up by domain-warped noise
//! (`warped_fbm`) into filaments and clouds, thickest towards the centre; and a few
//! scattered clouds above and below it. Its optical depth (at green) is returned for the caller
//! to dim everything behind it with (`reddening`).
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::scenefile::SceneFile;
use super::{luminance, mix, smoothstep, warped_fbm};
use serde_json::Value;
use std::f64::consts::PI;

pub struct MilkyWay {
    brightness: f64,
    dust: f64,
    disc_colour: Vector3<f64>,
    bulge_colour: Vector3<f64>,
    noise: SeededNoise,
    dust_noise: SeededNoise,
}

impl MilkyWay {
    pub fn parse(o: &Value, seed: u64) -> MilkyWay {
        let unit = |c: Vector3<f64>| c / luminance(&c);
        MilkyWay {
            brightness: SceneFile::parse_number(&o["brightness"], 0.12),
            dust: SceneFile::parse_number(&o["dust"], 1.),
            disc_colour: unit(SceneFile::parse_vec3_def(o, "disc_colour", Vector3::new(0.85, 0.9, 1.))),
            bulge_colour: unit(SceneFile::parse_vec3_def(o, "bulge_colour", Vector3::new(1., 0.82, 0.6))),
            noise: SeededNoise::new(mix(seed ^ 0x6a1a)),
            dust_noise: SeededNoise::new(mix(seed ^ 0xd057)),
        }
    }

    /// The glow of the unresolved stars in unit direction `d`, at galactic longitude `l` and
    /// latitude `b`.
    pub fn glow(&self, d: &Vector3<f64>, l: f64, b: f64) -> Vector3<f64> {
        if self.brightness <= 0. {
            return Vector3::zeros();
        }
        let scale = 0.06 + 0.05 * l.abs() / PI;
        let disc = (-b.abs() / scale).exp() * (0.3 + 0.7 * (-(l / 1.4).powi(2)).exp());
        let bulge = 1.3 * (-(l * l + (b / 0.75).powi(2)) / (2. * 0.2 * 0.2)).exp();
        // Star clouds: brighter and darker patches along the band.
        let clouds = (1. + 1.5 * self.noise.fbm(d * 4., 4) + 0.8 * self.noise.fbm(d * 23., 4)).max(0.1).powf(1.3);
        (self.disc_colour * disc + self.bulge_colour * bulge) * (clouds * self.brightness)
    }

    /// The optical depth (at green) of the galaxy's dust in unit direction `d`, at galactic
    /// longitude `l` and latitude `b`.
    pub fn dust(&self, d: &Vector3<f64>, l: f64, b: f64) -> f64 {
        if self.dust <= 0. || b.abs() > 0.8 {
            return 0.;
        }
        // The lane wanders a little either side of the plane.
        let offset = 0.025 * self.dust_noise.fbm(d * 1.5 + Vector3::new(5., 5., 5.), 3);
        let width = 0.045 + 0.03 * l.abs() / PI;
        let lane = (-((b - offset) / width).powi(2)).exp();
        let filaments = smoothstep(0., 0.3, warped_fbm(&self.dust_noise, d * 7., 1.1, 7));
        let centre = 0.5 + 0.9 * (-(l / 1.5).powi(2)).exp();
        // Isolated dark clouds a little way off the plane.
        let clouds = (-(b / 0.3).powi(2)).exp() * smoothstep(0.25, 0.45, warped_fbm(&self.dust_noise, d * 11. + Vector3::new(-3., 1., 7.), 1., 5));
        self.dust * (lane * (0.15 + 3. * filaments) * centre + 1.2 * clouds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedural::space::GalacticFrame;

    #[test]
    fn test_band_is_brightest_in_the_plane_and_towards_the_centre() {
        let m = MilkyWay::parse(&Value::Null, 1);
        let f = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        // Averaged over longitudes, to see past the clouds.
        let mean = |b: f64, ls: &[f64]| ls.iter().map(|&l| luminance(&m.glow(&f.direction(l, b), l, b))).sum::<f64>() / ls.len() as f64;
        let all: Vec<f64> = (0..72).map(|i| -PI + i as f64 * PI / 36.).collect();
        assert!(mean(0., &all) > 5. * mean(0.5, &all), "{} vs {}", mean(0., &all), mean(0.5, &all));
        let near: Vec<f64> = (0..20).map(|i| -0.3 + 0.03 * i as f64).collect();
        let far: Vec<f64> = (0..20).map(|i| PI - 0.3 + 0.03 * i as f64).collect();
        assert!(mean(0., &near) > 2. * mean(0., &far));
    }

    #[test]
    fn test_dust_lies_along_the_plane() {
        let m = MilkyWay::parse(&Value::Null, 1);
        let f = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        let mean = |b: f64| (0..360).map(|i| {
            let l = -PI + i as f64 * PI / 180.;
            m.dust(&f.direction(l, b), l, b)
        }).sum::<f64>() / 360.;
        assert!(mean(0.) > 0.3 && mean(0.) > 10. * mean(0.6), "{} vs {}", mean(0.), mean(0.6));
        assert_eq!(m.dust(&f.pole, 0., PI / 2.), 0.);
    }
}
