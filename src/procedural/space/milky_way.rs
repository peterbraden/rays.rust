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
//! Its brightness is set by the disc's surface brightness at the galactic centre, before the
//! bulge and the clouds: 21 magnitudes per square arcsecond by default, so that its brightest
//! clouds, in the bulge, are about 20 (the Sagittarius star cloud's, seen from a dark site;
//! Roach and Gordon, "The Light of the Night Sky", 1973, chapter 6), its fainter stretches 22
//! to 23.
//!
//! The dust is a narrower layer along the plane, its density broken up by domain-warped noise
//! (`warped_fbm`) into filaments and clouds, thickest towards the centre; and a few
//! scattered clouds above and below it. Its optical depth (at green) is returned for the caller
//! to dim everything behind it with (`reddening`).
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::scenefile::SceneFile;
use super::{Photometry, luminance, mix, smoothstep, warped_fbm};
use serde_json::Value;
use std::f64::consts::PI;

/// The greatest the star clouds' mottling can make the glow (`glow`).
const MAX_CLOUDS: f64 = 4.73;

pub struct MilkyWay {
    /// The disc's radiance at the galactic centre, before the bulge and the clouds.
    brightness: f64,
    dust: f64,
    disc_colour: Vector3<f64>,
    bulge_colour: Vector3<f64>,
    noise: SeededNoise,
    dust_noise: SeededNoise,
}

/// The glow of the disc and the bulge at galactic longitude `l` and latitude `b`, before the
/// clouds, relative to the disc's at the centre.
fn profile(l: f64, b: f64) -> (f64, f64) {
    let scale = 0.06 + 0.05 * l.abs() / PI;
    let disc = (-b.abs() / scale).exp() * (0.3 + 0.7 * (-(l / 1.4).powi(2)).exp());
    let bulge = 1.3 * (-(l * l + (b / 0.75).powi(2)) / (2. * 0.2 * 0.2)).exp();
    (disc, bulge)
}

impl MilkyWay {
    pub fn parse(o: &Value, seed: u64, photometry: &Photometry) -> MilkyWay {
        let unit = |c: Vector3<f64>| c / luminance(&c);
        MilkyWay {
            brightness: photometry.radiance(SceneFile::parse_number(&o["surface_brightness"], 21.))
                * SceneFile::parse_number(&o["brightness"], 1.),
            dust: SceneFile::parse_number(&o["dust"], 1.),
            disc_colour: unit(SceneFile::parse_vec3_def(o, "disc_colour", Vector3::new(0.85, 0.9, 1.))),
            bulge_colour: unit(SceneFile::parse_vec3_def(o, "bulge_colour", Vector3::new(1., 0.82, 0.6))),
            noise: SeededNoise::new(mix(seed ^ 0x6a1a)),
            dust_noise: SeededNoise::new(mix(seed ^ 0xd057)),
        }
    }

    /// Its parameters for the GPU, in the order of the shader's `MILKY_*`: brightness, dust,
    /// the disc's colour, the bulge's, the noise's offset and the dust noise's.
    pub fn export(&self) -> Vec<f64> {
        let mut v = vec![self.brightness, self.dust];
        for c in [self.disc_colour, self.bulge_colour, self.noise.offset(), self.dust_noise.offset()] {
            v.extend(c.iter());
        }
        v
    }

    /// The glow of the unresolved stars in unit direction `d`, at galactic longitude `l` and
    /// latitude `b`.
    pub fn glow(&self, d: &Vector3<f64>, l: f64, b: f64) -> Vector3<f64> {
        if self.brightness <= 0. {
            return Vector3::zeros();
        }
        let (disc, bulge) = profile(l, b);
        // Star clouds: brighter and darker patches along the band.
        // (fbm is within [-1, 1], so this is at most 3.3^1.3: MAX_CLOUDS.)
        let clouds = (1. + 1.5 * self.noise.fbm(d * 4., 4) + 0.8 * self.noise.fbm(d * 23., 4)).max(0.1).powf(1.3);
        (self.disc_colour * disc + self.bulge_colour * bulge) * (clouds * self.brightness)
    }

    /// A bound on the luminance of the glow at galactic longitude `l` and latitude `b`: the
    /// clouds at their brightest (its colours are of unit luminance).
    pub fn bound(&self, l: f64, b: f64) -> f64 {
        let (disc, bulge) = profile(l, b);
        self.brightness * (disc + bulge) * MAX_CLOUDS
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
        let m = MilkyWay::parse(&Value::Null, 1, &Photometry::new(1.));
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
        let m = MilkyWay::parse(&Value::Null, 1, &Photometry::new(1.));
        let f = GalacticFrame::new(Vector3::new(0., 1., 0.), Vector3::new(1., 0., 0.));
        let mean = |b: f64| (0..360).map(|i| {
            let l = -PI + i as f64 * PI / 180.;
            m.dust(&f.direction(l, b), l, b)
        }).sum::<f64>() / 360.;
        assert!(mean(0.) > 0.3 && mean(0.) > 10. * mean(0.6), "{} vs {}", mean(0.), mean(0.6));
        assert_eq!(m.dust(&f.pole, 0., PI / 2.), 0.);
    }
}
