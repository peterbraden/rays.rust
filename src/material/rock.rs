//! Procedural rock surfaces.
//!
//! Rock is shaded as a solid texture (Darwyn Peachey, "Solid Texturing of Complex Surfaces",
//! SIGGRAPH 1985; Ken Perlin, "An Image Synthesizer", SIGGRAPH 1985): its colour is a function
//! of position in space rather than of surface coordinates, so it needs no parameterisation and
//! carries on through any shape cut from it, as the grain of real stone does. It is built from
//! the features that identify a stone at a glance:
//!
//! - mottling between two body colours, as mineral content varies through the rock;
//! - grains: specks of dark and light mineral, eg. the biotite and quartz in granite;
//! - strata: bands of sediment, warped by noise, in sedimentary rock;
//! - cracks: dark, recessed seams;
//! - weathering: broad variation in tone;
//! - lichen or moss on the surfaces facing the sky;
//! - wetness, which darkens the rock (the water fills its pores, so less light escapes; John
//!   Lekner and Michael C. Dorf, "Why some things are darker when wet", Applied Optics 27(7),
//!   1988) and adds a glossy film reflecting with water's Fresnel factor.
//!
//! Relief finer than the geometry is bump mapped (James F. Blinn, "Simulation of Wrinkled
//! Surfaces", SIGGRAPH 1978): the shading normal is tilted against the gradient of a height
//! field, estimated by central differences.
//!
//! Reflectances are typical of the stones each preset is named for, in linear RGB: from about
//! 0.06 for basalt to 0.6 for limestone (eg. the USGS spectral library, Kokaly et al. 2017).
//! Rock is close to Lambertian, so the base is diffuse.

use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::geometry::{rand, random_point_on_unit_sphere, random_point_in_unit_ball};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{direct_lighting, fresnel_reflectance, reflect};

/// The look of a kind of stone. Lengths are in units of the material's `scale`.
#[derive(Clone, Debug, PartialEq)]
pub struct RockStyle {
    /// The body colour, and the colour it is mottled with.
    pub color: Color,
    pub secondary: Color,
    /// Dark and light mineral grains, and the fraction of the surface each covers.
    pub dark_grain: Color,
    pub dark_grain_amount: f64,
    pub light_grain: Color,
    pub light_grain_amount: f64,
    pub grain_size: f64,
    /// Sedimentary layers: their spacing, colour, and strength (0 for none).
    pub strata_spacing: f64,
    pub strata_color: Color,
    pub strata_amount: f64,
    /// Depth of the bump-mapped relief, and the size of its features.
    pub bump: f64,
    pub bump_size: f64,
    /// Strength of cracks, 0 for none.
    pub cracks: f64,
}

impl RockStyle {
    pub fn preset(name: &str) -> RockStyle {
        let grey = |v: f64| Color::new(v, v, v);
        let plain = RockStyle {
            color: grey(0.3), secondary: grey(0.36),
            dark_grain: grey(0.05), dark_grain_amount: 0.,
            light_grain: grey(0.7), light_grain_amount: 0.,
            grain_size: 0.01,
            strata_spacing: 0.1, strata_color: grey(0.2), strata_amount: 0.,
            bump: 0.4, bump_size: 0.15, cracks: 0.3,
        };
        match name {
            // Coarse-grained and speckled: feldspar body, black biotite, glassy quartz.
            "granite" => RockStyle {
                color: Color::new(0.42, 0.37, 0.34), secondary: Color::new(0.52, 0.45, 0.41),
                dark_grain: Color::new(0.04, 0.04, 0.04), dark_grain_amount: 0.18,
                light_grain: Color::new(0.72, 0.70, 0.67), light_grain_amount: 0.12,
                grain_size: 0.012,
                bump: 0.35, cracks: 0.35,
                ..plain
            },
            // Dark, fine-grained volcanic rock, pitted with gas bubbles.
            "basalt" => RockStyle {
                color: Color::new(0.07, 0.07, 0.07), secondary: Color::new(0.11, 0.10, 0.095),
                dark_grain: Color::new(0.02, 0.02, 0.02), dark_grain_amount: 0.08,
                light_grain: Color::new(0.2, 0.19, 0.17), light_grain_amount: 0.03,
                grain_size: 0.008,
                bump: 0.55, bump_size: 0.1, cracks: 0.25,
                ..plain
            },
            // Layered sand, coloured by iron oxides.
            "sandstone" => RockStyle {
                color: Color::new(0.52, 0.34, 0.20), secondary: Color::new(0.62, 0.46, 0.30),
                dark_grain: Color::new(0.30, 0.18, 0.10), dark_grain_amount: 0.06,
                light_grain: Color::new(0.75, 0.62, 0.45), light_grain_amount: 0.06,
                grain_size: 0.004,
                strata_spacing: 0.08, strata_color: Color::new(0.40, 0.22, 0.12), strata_amount: 0.55,
                bump: 0.3, bump_size: 0.2, cracks: 0.15,
            },
            // Pale and chalky, with darker fossil fragments.
            "limestone" => RockStyle {
                color: Color::new(0.60, 0.57, 0.50), secondary: Color::new(0.50, 0.48, 0.42),
                dark_grain: Color::new(0.32, 0.30, 0.26), dark_grain_amount: 0.05,
                light_grain: Color::new(0.75, 0.73, 0.68), light_grain_amount: 0.08,
                grain_size: 0.02,
                strata_spacing: 0.3, strata_color: Color::new(0.45, 0.43, 0.38), strata_amount: 0.25,
                bump: 0.45, bump_size: 0.25, cracks: 0.4,
            },
            // Blue-grey, in thin sheets.
            "slate" => RockStyle {
                color: Color::new(0.13, 0.14, 0.16), secondary: Color::new(0.18, 0.18, 0.20),
                light_grain: Color::new(0.3, 0.3, 0.32), light_grain_amount: 0.02,
                grain_size: 0.005,
                strata_spacing: 0.02, strata_color: Color::new(0.10, 0.11, 0.12), strata_amount: 0.4,
                bump: 0.3, bump_size: 0.06, cracks: 0.2,
                ..plain
            },
            // Hard, pale, and nearly featureless: water-worn quartzite pebbles.
            "quartzite" => RockStyle {
                color: Color::new(0.68, 0.62, 0.56), secondary: Color::new(0.74, 0.70, 0.66),
                dark_grain: Color::new(0.45, 0.38, 0.32), dark_grain_amount: 0.03,
                light_grain: Color::new(0.85, 0.83, 0.80), light_grain_amount: 0.05,
                grain_size: 0.006,
                bump: 0.15, bump_size: 0.2, cracks: 0.05,
                ..plain
            },
            _ => panic!("Unknown rock preset '{}': expected granite, basalt, sandstone, limestone, slate or quartzite", name),
        }
    }
}

pub struct Rock {
    pub style: RockStyle,
    /// The size of the texture's features, in scene units: 1 for metre-sized features when the
    /// scene is in metres.
    pub scale: f64,
    /// Broad variation in tone, 0 for none.
    pub weathering: f64,
    /// Coverage of lichen on upward surfaces, 0 to 1, and its colour.
    pub lichen: f64,
    pub lichen_color: Color,
    /// 0 for dry, 1 for soaked.
    pub wetness: f64,
    noise: SeededNoise,
    /// Noise levels above which the given fraction of the surface is grain or lichen.
    dark_threshold: f64,
    light_threshold: f64,
    lichen_threshold: f64,
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// Refractive index of water.
const WATER: f64 = 1.33;

impl Rock {
    pub fn new(style: RockStyle, scale: f64, seed: u64) -> Rock {
        let noise = SeededNoise::new(seed);
        let quantile = |fraction: f64, sample: &dyn Fn(Vector3<f64>) -> f64| {
            // Noise levels aren't uniformly distributed, so find the level exceeded by the
            // given fraction of points from a sample.
            let mut rng = crate::geometry::Pcg32::new(seed, 7);
            let mut v: Vec<f64> = (0 .. 4096).map(|_| {
                sample(Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 50.)
            }).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if fraction <= 0. { return f64::INFINITY }
            v[((1. - fraction.min(1.)) * (v.len() - 1) as f64) as usize]
        };
        let dark_threshold = quantile(style.dark_grain_amount, &|p| noise.noise(p));
        let light_threshold = quantile(style.light_grain_amount, &|p| noise.noise(p + Vector3::new(31.4, 0., 0.)));
        Rock {
            style, scale, weathering: 0.3, lichen: 0., lichen_color: Color::new(0.38, 0.40, 0.26), wetness: 0.,
            noise, dark_threshold, light_threshold, lichen_threshold: 0.,
        }
    }

    pub fn with_lichen(mut self, lichen: f64, color: Color) -> Rock {
        self.lichen = lichen.clamp(0., 1.);
        self.lichen_color = color;
        let noise = &self.noise;
        let mut rng = crate::geometry::Pcg32::new(1, 9);
        let mut v: Vec<f64> = (0 .. 4096).map(|_| {
            noise.fbm(Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 50., 4)
        }).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.lichen_threshold = v[((1. - self.lichen) * (v.len() - 1) as f64) as usize];
        self
    }

    /// How far a point lies in a crack: 1 at its centre, 0 away from cracks. Cracks follow the
    /// zero crossings of a noise field, where 1 - |n| peaks.
    fn crack(&self, q: Vector3<f64>) -> f64 {
        if self.style.cracks <= 0. { return 0. }
        let c = 1. - self.noise.noise(q * 2.3 + Vector3::new(0., 17., 0.)).abs();
        // Cracks fade in and out along their length.
        let presence = smoothstep(-0.1, 0.3, self.noise.noise(q * 0.9 + Vector3::new(5., 0., 0.)));
        smoothstep(1. - 0.1 * self.style.cracks, 1., c) * presence
    }

    /// Height of the bump-mapped relief, in units of `scale`.
    fn relief(&self, q: Vector3<f64>) -> f64 {
        let b = q / self.style.bump_size;
        self.noise.fbm(b, 4) * self.style.bump_size * 0.6
            + self.noise.ridged(b * 0.5 + Vector3::new(0., 0., 9.), 3) * self.style.bump_size * 0.3
            - self.crack(q) * self.style.bump_size * 0.15 * self.style.cracks
    }

    /// The shading normal: the surface normal tilted by the relief.
    pub fn bumped_normal(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        if self.style.bump <= 0. { return normal }
        let q = p / self.scale;
        let e = self.style.bump_size * 0.05;
        let d = |axis: Vector3<f64>| (self.relief(q + axis * e) - self.relief(q - axis * e)) / (2. * e);
        let g = Vector3::new(d(Vector3::x()), d(Vector3::y()), d(Vector3::z()));
        // Only the part of the gradient along the surface tilts it.
        let tangential = g - normal * g.dot(&normal);
        (normal - tangential * self.style.bump).try_normalize(1e-12).unwrap_or(normal)
    }

    /// The diffuse reflectance at a point, given its shading normal.
    pub fn albedo(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Color {
        let st = &self.style;
        let q = p / self.scale;

        // Mottling between the body colours.
        let mottle = smoothstep(-0.4, 0.4, self.noise.fbm(q * 1.3, 5));
        let mut c = st.color.blend(&st.secondary, mottle);

        // Strata: bands across the vertical, warped so they wander like real bedding.
        if st.strata_amount > 0. {
            let y = q.y + self.noise.fbm(q * 0.6 + Vector3::new(0., 3., 0.), 3) * st.strata_spacing * 2.;
            let phase = y / st.strata_spacing;
            let band = (0.5 + 0.5 * (phase * std::f64::consts::TAU).sin()).powi(3);
            // A second, finer set of laminations within the beds.
            let fine = 0.5 + 0.5 * (phase * 4.7 * std::f64::consts::TAU).sin();
            c = c.blend(&st.strata_color, st.strata_amount * (0.75 * band + 0.25 * fine));
        }

        // Mineral grains.
        let g = q / st.grain_size;
        let grain = |threshold: f64, offset: Vector3<f64>| {
            if threshold.is_finite() {
                smoothstep(threshold, threshold + 0.05, self.noise.noise(g + offset))
            } else { 0. }
        };
        c = c.blend(&st.dark_grain, grain(self.dark_threshold, Vector3::zeros()));
        c = c.blend(&st.light_grain, grain(self.light_threshold, Vector3::new(31.4, 0., 0.)));

        // Broad weathering, and grime in the cracks.
        let tone = 1. + self.weathering * self.noise.fbm(q * 0.25 + Vector3::new(0., 0., 40.), 3);
        c = c * (tone * (1. - 0.6 * self.crack(q)));

        // Lichen grows on the surfaces facing the sky.
        if self.lichen > 0. {
            // Crustose lichen grows in patches with crisp, ragged edges, and is paler at its rim.
            let n = self.noise.fbm(q * 3. + Vector3::new(70., 0., 0.), 4)
                + 0.03 * self.noise.noise(q * 60.);
            let patch = smoothstep(self.lichen_threshold - 0.008, self.lichen_threshold + 0.008, n);
            let rim = smoothstep(self.lichen_threshold + 0.06, self.lichen_threshold + 0.01, n);
            let up = smoothstep(-0.2, 0.6, normal.y);
            let grain = 0.85 + 0.3 * self.noise.noise(q * 40.);
            let lichen = self.lichen_color.blend(&Color::new(0.6, 0.6, 0.52), 0.4 * rim) * grain;
            c = c.blend(&lichen, patch * up);
        }

        // Water in the pores darkens the rock.
        c = c * (1. - 0.45 * self.wetness);
        // Even pale stone reflects little more than 0.85.
        c.clamp(0.88)
    }
}

impl MaterialModel for Rock {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let geometric = intersection.facing_normal(r);
        let normal = self.bumped_normal(intersection.point, geometric);

        if self.wetness > 0. {
            let cos_i = (r.rd.dot(&normal) / r.rd.norm()).abs();
            let film = self.wetness * fresnel_reflectance(cos_i, 1. / WATER, WATER);
            if rand() < film {
                let rd = reflect(r.rd.normalize(), normal) + random_point_in_unit_ball() * 0.05;
                if rd.dot(&geometric) <= 0. {
                    return ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::black(), ray: None };
                }
                return ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::white(),
                    ray: Some(Ray { ro: intersection.point, rd }) };
            }
        }

        let albedo = self.albedo(intersection.point, normal);
        let shaded = Intersection { normal, ..*intersection };
        // Cosine-weighted about the shading normal; a bounce the bump tilts into the surface is
        // mirrored back out of it.
        let mut rd = normal + random_point_on_unit_sphere();
        if rd.norm() < 1e-9 { rd = normal }
        let into = rd.dot(&geometric);
        if into < 0. { rd -= geometric * (2. * into) }
        ScatteredRay {
            emit: direct_lighting(albedo, &shaded, s),
            sampled_lights: true,
            attenuate: albedo,
            ray: Some(Ray { ro: intersection.point, rd: rd.normalize() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRESETS: [&str; 6] = ["granite", "basalt", "sandstone", "limestone", "slate", "quartzite"];

    #[test]
    fn test_albedo_is_plausible_and_varies() {
        for name in PRESETS {
            let rock = Rock::new(RockStyle::preset(name), 1., 3).with_lichen(0.2, Color::new(0.3, 0.3, 0.2));
            let mut rng = crate::geometry::Pcg32::new(0, 0);
            let mut seen = Vec::new();
            for _ in 0 .. 500 {
                let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 10.;
                let c = rock.albedo(p, Vector3::y());
                for v in c.rgb.iter() {
                    assert!(*v >= 0. && *v < 0.95, "{} albedo {}", name, c.rgb);
                }
                seen.push(c.rgb.x);
            }
            let (lo, hi) = seen.iter().fold((1f64, 0f64), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
            assert!(hi - lo > 0.02, "{} is flat: {} to {}", name, lo, hi);
        }
    }

    #[test]
    fn test_grain_fraction_follows_amount() {
        let rock = Rock::new(RockStyle::preset("granite"), 1., 0);
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let n = 4000;
        let dark = (0 .. n).filter(|_| {
            let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 100.;
            rock.noise.noise(p) > rock.dark_threshold
        }).count() as f64 / n as f64;
        assert!((dark - 0.18).abs() < 0.04, "{}", dark);
    }

    #[test]
    fn test_bumped_normal_stays_on_the_surface_side() {
        let rock = Rock::new(RockStyle::preset("basalt"), 1., 1);
        let mut rng = crate::geometry::Pcg32::new(2, 0);
        let mut tilted = 0;
        for _ in 0 .. 500 {
            let p = Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 5.;
            let n = rock.bumped_normal(p, Vector3::y());
            assert!((n.norm() - 1.).abs() < 1e-9);
            assert!(n.y > 0.2, "{}", n);
            if n.y < 0.999 { tilted += 1 }
        }
        assert!(tilted > 400);
    }

    #[test]
    fn test_wet_rock_is_darker() {
        let mut rock = Rock::new(RockStyle::preset("sandstone"), 1., 0);
        let p = Vector3::new(0.3, 0.2, 0.1);
        let dry = rock.albedo(p, Vector3::y());
        rock.wetness = 1.;
        let wet = rock.albedo(p, Vector3::y());
        assert!(wet.rgb.x < dry.rgb.x * 0.6);
    }

    #[test]
    #[should_panic(expected = "Unknown rock preset")]
    fn test_unknown_preset() {
        RockStyle::preset("cheese");
    }
}
