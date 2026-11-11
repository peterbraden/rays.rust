// The implementation of a physically-based sky using Rayleigh and Mie scattering models.
//
// This module simulates the scattering of light through the atmosphere using two primary
// physical models:
//
// 1. Rayleigh scattering: Scattering of light by particles much smaller than the wavelength
//    of the light. This is what makes the sky appear blue, as shorter (blue) wavelengths
//    scatter more than longer (red) wavelengths according to a 1/λ⁴ relationship.
//
// 2. Mie scattering: Scattering by particles comparable to or larger than the wavelength
//    of light, such as dust, pollen, and water droplets. This creates the white-ish haze 
//    around the sun and is less wavelength-dependent.
//
// The implementation uses numerical integration to accumulate the scattering effects along
// both the view ray and light rays to the sun.
//
// IMPLEMENTATION NOTE ON OPTICAL DEPTH ACCUMULATION:
// ==================================================
// Optical depth must be accumulated along the entire light path to correctly model
// atmospheric scattering according to the Beer-Lambert law:
//
// - We accumulate optical depth along the entire path from the camera through the atmosphere
// - Each sample's tau calculation uses the total optical depth from camera to sample point
// - This correctly produces the wavelength-dependent effects that make sunsets appear redder
//   as the light travels through more atmosphere near the horizon
// - This follows directly from the physics of light attenuation, where extinction is exponential
//   with respect to the total optical depth along the entire path
//
// For this implementation, we track the accumulated optical depth during numerical integration
// to ensure the tau calculation properly accounts for all atmospheric material the light has
// traveled through. This is critical for realistic rendering of sunset/sunrise conditions.

use crate::geometry::rand;
use crate::shapes::infinite::Infinite;
use crate::shapes::sphere::Sphere;
use crate::na::{Vector3};
use crate::sceneobject::SceneObject;
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::shapes::geometry::Geometry;
use serde_json::{Value};
use crate::scenefile::SceneFile;
use rayon::prelude::*;

// Constants for atmospheric scattering
/// Scales the integrated radiance into the renderer's working range. Under the default sRGB
/// output this keeps the sky looking as it did with its former built-in tone curve,
/// (0.383 v)^(1/2.2).
const SKY_CALIBRATION: f64 = 0.38317;

const DEFAULT_NUM_SAMPLES: usize = 32;
/// Resolution of the sun's transmittance table: altitudes (spaced quadratically, closer low
/// down) and cosines of its angle from the zenith.
const SUN_TABLE_ALTITUDES: usize = 64;
const SUN_TABLE_ANGLES: usize = 256;
const DEFAULT_NUM_SAMPLES_LIGHT: usize = 16;

/// Sample points along a path of `length` through the atmosphere, as (distance, step): the
/// midpoints, in u, of n equal steps of u, where distance = length u². A ray from inside the
/// atmosphere that runs near the ground (to the horizon, or up to the sun when it is low) crosses
/// almost all of its air in the first few kilometres of a path that can be a thousand long:
/// even steps of 70 km sampled that air once or twice, and integrated a dark, yellow-brown
/// horizon where a fine integration finds it pale. Quadratic spacing puts the samples where the
/// air is, a few hundred metres apart near the start, for no more of them.
fn path_samples(length: f64, n: usize) -> impl Iterator<Item = (f64, f64)> {
    (0..n).map(move |i| {
        let (a, b) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
        let u = (a + b) / 2.;
        (length * u * u, length * (b * b - a * a))
    })
}

// Default scattering coefficient values (Earth-like)
// Sea-level Rayleigh scattering coefficients (per metre) at 680, 550 and 440 nm. These scale as
// 1/wavelength^4; the red value was previously 3.8e-6, a typo that made the sky scatter too
// little red light.
const DEFAULT_RAYLEIGH_R: f64 = 5.8e-6;
const DEFAULT_RAYLEIGH_G: f64 = 13.5e-6;
const DEFAULT_RAYLEIGH_B: f64 = 33.1e-6;
const DEFAULT_MIE_COEFFICIENT: f64 = 21.0e-6;
const DEFAULT_MIE_ANISOTROPY: f64 = 0.76;

// Preset scattering coefficients for different atmospheres
pub struct SkyPreset {
    pub name: &'static str,
    pub rayleigh_r: f64,
    pub rayleigh_g: f64,
    pub rayleigh_b: f64,
    pub mie_coefficient: f64,
    pub rayleigh_thickness: f64,
    pub mie_thickness: f64,
    pub mie_anisotropy: f64,
}

// Presets for different atmospheric conditions
pub const SKY_PRESETS: [SkyPreset; 3] = [
    // Earth-like (default)
    SkyPreset {
        name: "earth",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 21.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.76,
    },
    // Earth sunset (warmer colors)
    SkyPreset {
        name: "sunset",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 25.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.85,
    },
    // Hazy (dusty/smoky atmosphere)
    SkyPreset {
        name: "hazy",
        rayleigh_r: 5.8e-6,
        rayleigh_g: 13.5e-6,
        rayleigh_b: 33.1e-6,
        mie_coefficient: 45.0e-6,
        rayleigh_thickness: 7994.0,
        mie_thickness: 1200.0,
        mie_anisotropy: 0.9,
    },
];

/// SkyMaterial simulates atmospheric scattering with Rayleigh and Mie models
pub struct SkyMaterial {
    /// Sphere representing the atmospheric boundary
    atmosphere: Sphere,
    
    /// Sphere representing the earth/planet surface
    earth: Sphere,
    
    /// Direction to the sun (normalized)
    sun_direction: Vector3<f64>,
    
    /// Scale height for Rayleigh scattering in meters
    rayleigh_thickness: f64,
    
    /// Scale height for Mie scattering in meters
    mie_thickness: f64,
    
    /// Overall brightness/intensity of the sky
    brightness: f64,
    
    /// Rayleigh scattering coefficients for RGB channels
    rayleigh_coefficients: Vector3<f64>,
    
    /// Mie scattering coefficient (wavelength-independent)
    mie_coefficient: f64,
    
    /// Mie anisotropy factor (controls the size of the glow around the sun)
    mie_anisotropy: f64,

    /// Ozone's absorption coefficients per channel (zero to leave it out).
    ozone: Vector3<f64>,

    /// Optical depths towards the sun over altitude and sun angle (`sun_depths_at`), for aerial
    /// perspective and sunlight anywhere on the planet; built on first use.
    sun_table: std::sync::OnceLock<Vec<Depths>>,

    /// Whether to add light scattered more than once (`MultipleScattering`), and the albedo
    /// of the earth beneath, which reflects some of it back up.
    multiple_scattering: bool,
    ground_albedo: f64,
    ms_table: std::sync::OnceLock<MultipleScattering>,

    /// Whether clouds and terrain shadow the air seen from the camera (`shadowed_loss`):
    /// crepuscular rays.
    pub crepuscular_rays: bool,
}

impl SkyMaterial {
    /// Calculate the Rayleigh phase function
    /// 
    /// The Rayleigh phase function describes the angular distribution of 
    /// light scattered by particles much smaller than the wavelength.
    ///
    /// # Arguments
    /// * `mu` - Cosine of the angle between the view direction and sun direction
    fn rayleigh_phase(mu: f64) -> f64 {
        3.0 / (16.0 * std::f64::consts::PI) * (1.0 + mu * mu)
    }
    
    /// Calculate the Mie phase function. This is not Henyey-Greenstein, as previously described,
    /// but a related form with an extra (1 + mu^2) factor:
    /// 3(1 - g^2) (1 + mu^2) / (8 pi (2 + g^2) (1 + g^2 - 2 g mu)^(3/2)).
    /// 
    /// The Mie phase function describes the angular distribution of light
    /// scattered by particles comparable to or larger than the wavelength.
    ///
    /// # Arguments
    /// * `mu` - Cosine of the angle between the view direction and sun direction
    /// * `g` - Anisotropy factor (-1 to 1), controls the distribution of scattered light
    fn mie_phase(mu: f64, g: f64) -> f64 {
        let g2 = g * g;
        3.0 / (8.0 * std::f64::consts::PI) * 
            ((1.0 - g2) * (1.0 + mu * mu)) / 
            ((2.0 + g2) * (1.0 + g2 - 2.0 * g * mu).powf(1.5))
    }
    
    /// Calculate optical depth for a segment through the atmosphere
    /// 
    /// The optical depth represents how much a ray of light is attenuated by the medium.
    /// For atmospheric scattering, it's calculated using the barometric formula, where 
    /// density decreases exponentially with height.
    ///
    /// # Arguments
    /// * `height` - Height above the earth's surface in meters
    /// * `thickness` - Scale height parameter (rayleigh_thickness or mie_thickness)
    /// * `segment_length` - Length of the ray segment being calculated
    fn optical_depth(&self, height: f64, thickness: f64, segment_length: f64) -> f64 {
        (-height / thickness).exp() * segment_length
    }
    

    /// Where along the view ray (a unit direction) it is in the atmosphere: from where it
    /// enters (0 if it starts inside) to where it leaves, or hits the ground if it starts above
    /// the surface. None if it misses the atmosphere entirely. From orbit, the ray starts in
    /// space, and the air begins at the entry.
    fn view_ray_span(&self, r: &Ray) -> Option<(f64, f64)> {
        let o = r.ro - self.atmosphere.center;
        let b = o.dot(&r.rd);
        let c = o.norm_squared() - self.atmosphere.radius * self.atmosphere.radius;
        let disc = b * b - c;
        if disc < 0. {
            return None;
        }
        let (enter, leave) = (-b - disc.sqrt(), -b + disc.sqrt());
        if leave <= 0. {
            return None;
        }
        let start = enter.max(0.);
        let mut end = leave;
        if (r.ro - self.earth.center).norm() > self.earth.radius {
            if let Some(ground) = self.earth.intersects(r) {
                end = end.min(ground.dist);
            }
        }
        if end <= start { None } else { Some((start, end)) }
    }

    /// The distance to the end of the view ray's span through the atmosphere.
    fn view_ray_length(&self, r: &Ray) -> Option<f64> {
        self.view_ray_span(r).map(|(_, end)| end)
    }

    /// Sample points (distance, step) along the ray from `start` to `end`, in order, crowded
    /// (quadratically, as `path_samples`) about the ray's closest approach to the planet, where
    /// its air is densest: at the start for a ray looking up from inside, at the tangent point
    /// for one grazing the limb from orbit, at the end for one looking down at the ground.
    fn span_samples(&self, r: &Ray, start: f64, end: f64, n: usize) -> Vec<(f64, f64)> {
        let focus = (-(r.ro - self.earth.center).dot(&r.rd)).clamp(start, end);
        let towards_end = |a: f64, b: f64, n: usize| -> Vec<(f64, f64)> {
            // Crowded towards b: mirror path_samples, in order of increasing distance.
            let mut v: Vec<(f64, f64)> = path_samples(b - a, n).map(|(t, s)| (b - t, s)).collect();
            v.reverse();
            v
        };
        let away = |a: f64, b: f64, n: usize| -> Vec<(f64, f64)> {
            path_samples(b - a, n).map(|(t, s)| (a + t, s)).collect()
        };
        let before = focus - start;
        let after = end - focus;
        if before <= 1e-9 * (end - start) {
            away(start, end, n)
        } else if after <= 1e-9 * (end - start) {
            towards_end(start, end, n)
        } else {
            let n1 = ((n as f64 * before / (before + after)).round() as usize).clamp(1, n - 1);
            let mut v = towards_end(start, focus, n1);
            v.extend(away(focus, end, n - n1));
            v
        }
    }
}

impl SkyMaterial {
    /// Optical depths from `p` to the top of the atmosphere towards the sun, or None if the
    /// earth is in the way.
    fn optical_depth_to_sun(&self, p: Vector3<f64>) -> Option<Depths> {
        self.optical_depth_towards(p, self.sun_direction)
    }

    /// Optical depths from `p` to the top of the atmosphere along unit `dir`, or None if the
    /// earth is in the way.
    fn optical_depth_towards(&self, p: Vector3<f64>, dir: Vector3<f64>) -> Option<Depths> {
        let light_ray = Ray { ro: p, rd: dir };
        if self.earth.intersects(&light_ray).is_some() {
            return None;
        }
        let light_len = self.atmosphere.intersects(&light_ray)?.dist;
        // Integrate along the light ray to the sun (see `path_samples`).
        Some(path_samples(light_len, DEFAULT_NUM_SAMPLES_LIGHT).fold(Depths::default(), |acc, (t, step)| {
            let height = (p + t * dir - self.atmosphere.center).norm() - self.earth.radius;
            acc + self.depths(height, step)
        }))
    }

    /// The Rayleigh, Mie and ozone optical depths of a step of length `step` at `height`.
    fn depths(&self, height: f64, step: f64) -> Depths {
        // Ozone: a tent from 10 km to 40 km, peaking at 25 km (Bruneton 2017, after the US
        // Standard Atmosphere 1976).
        let ozone = (1. - (height - 25_000.).abs() / 15_000.).max(0.);
        Depths {
            r: self.optical_depth(height, self.rayleigh_thickness, step),
            m: self.optical_depth(height, self.mie_thickness, step),
            o: ozone * step,
        }
    }

    /// Linear radiance arriving along the view ray.
    pub fn radiance(&self, r: &Ray) -> Vector3<f64> {
        let (rayleigh, mie, multiple) = self.scattering(r);
        self.apply_phase(&r.rd.normalize(), rayleigh, mie, multiple)
    }

    /// Combine in-scattered Rayleigh and Mie light (see `scattering`) for the view direction
    /// `d`: weight each by its phase function at the angle between `d` and the sun. Light
    /// scattered more than once comes from all round, and needs no phase function.
    fn apply_phase(&self, d: &Vector3<f64>, rayleigh: Vector3<f64>, mie: Vector3<f64>, multiple: Vector3<f64>) -> Vector3<f64> {
        let mu = d.dot(&self.sun_direction);
        (rayleigh * Self::rayleigh_phase(mu) + mie * Self::mie_phase(mu, self.mie_anisotropy) + multiple) * self.brightness
    }

    /// Light arriving at `p` (per unit sun irradiance) after scattering more than once, times
    /// the isotropic phase function: `MultipleScattering`'s Ψ, or none if it is off.
    fn multiple_at(&self, p: &Vector3<f64>) -> Vector3<f64> {
        if !self.multiple_scattering {
            return Vector3::zeros();
        }
        let table = self.ms_table.get_or_init(|| MultipleScattering::build(self));
        let up = (p - self.earth.center).normalize();
        table.lookup((p - self.earth.center).norm() - self.earth.radius, up.dot(&self.sun_direction), self)
    }

    /// The scattering coefficient (per channel) times a step, from that step's optical depths.
    /// Ozone only absorbs.
    fn scattered(&self, d: Depths) -> Vector3<f64> {
        self.rayleigh_coefficients * d.r + Vector3::repeat(self.mie_coefficient * d.m)
    }

    /// The transmittance for accumulated optical depths: Rayleigh scattering, Mie scattering
    /// and absorption (extinction 1.1 times scattering), and ozone absorption.
    fn transmittance_of(&self, d: Depths) -> Vector3<f64> {
        (self.rayleigh_coefficients * d.r + Vector3::repeat(self.mie_coefficient * 1.1 * d.m) + self.ozone * d.o)
            .map(|t| (-t).exp())
    }

    /// Light scattered towards the viewer along the view ray, by Rayleigh and by Mie scattering,
    /// before the phase functions and brightness are applied. Unlike the phase functions, which
    /// peak sharply around the sun, these vary smoothly with direction.
    pub fn scattering(&self, r: &Ray) -> Scattering {
        let (start, end) = match self.view_ray_span(r) {
            Some(span) => span,
            None => return (Vector3::zeros(), Vector3::zeros(), Vector3::zeros()),
        };
        let (mut rayleigh_sum, mut mie_sum, mut multiple_sum) = (Vector3::zeros(), Vector3::zeros(), Vector3::zeros());
        // Optical depth from the viewer, accumulated along the ray: this is what makes sunsets
        // red (see the note at the top of the file).
        let mut depth = Depths::default();
        // Integrate along the view ray through the atmosphere (see `span_samples`).
        for (t, step) in self.span_samples(r, start, end, DEFAULT_NUM_SAMPLES) {
            let sample_position = r.ro + t * r.rd;
            let height = (sample_position - self.atmosphere.center).norm() - self.earth.radius;
            let here = self.depths(height, step);
            // From the viewer to this sample, the middle of its step.
            depth = depth + here * 0.5;
            // Light scattered here after scattering before: from the whole sky, even in the
            // earth's shadow.
            multiple_sum += self.transmittance_of(depth)
                .component_mul(&self.scattered(here))
                .component_mul(&self.multiple_at(&sample_position));
            // Sunlight reaching this sample, none in the earth's shadow, attenuated on its way
            // in and on its way out to the viewer (Beer-Lambert along the whole path).
            if let Some(to_sun) = self.optical_depth_to_sun(sample_position) {
                let attenuation = self.transmittance_of(depth + to_sun);
                rayleigh_sum += attenuation * here.r;
                mie_sum += attenuation * here.m;
            }
            // And on through the rest of the step.
            depth = depth + here * 0.5;
        }
        (rayleigh_sum.component_mul(&self.rayleigh_coefficients), mie_sum * self.mie_coefficient, multiple_sum)
    }
}

/// Aerial perspective: what the atmosphere does to light crossing a stretch of it.
impl SkyMaterial {
    /// Optical depths towards the sun from `p`, from a table over altitude and the cosine of the
    /// sun's angle from the local zenith (Hillaire 2020, section 4, after Bruneton and Neyret
    /// 2008): the sun is far enough away that nothing else matters, anywhere on the planet.
    /// In the earth's shadow the depths are huge, so the transmittance is nil, and fades to it
    /// over one column of the table at the terminator.
    fn sun_depths_at(&self, p: &Vector3<f64>) -> Depths {
        let table = self.sun_table.get_or_init(|| {
            let top = self.atmosphere.radius - self.earth.radius;
            (0..SUN_TABLE_ALTITUDES * SUN_TABLE_ANGLES).into_par_iter().map(|k| {
                let (i, j) = (k / SUN_TABLE_ANGLES, k % SUN_TABLE_ANGLES);
                let altitude = top * (i as f64 / (SUN_TABLE_ALTITUDES - 1) as f64).powi(2);
                let mu = -1. + 2. * j as f64 / (SUN_TABLE_ANGLES - 1) as f64;
                let point = self.earth.center + Vector3::new(0., self.earth.radius + altitude.clamp(1., top - 1.), 0.);
                self.optical_depth_towards(point, Vector3::new((1. - mu * mu).max(0.).sqrt(), mu, 0.))
                    .unwrap_or(Depths { r: 1e7, m: 1e7, o: 0. })
            }).collect::<Vec<Depths>>()
        });
        let top = self.atmosphere.radius - self.earth.radius;
        // Above the atmosphere, the sunlight crosses the air (if at all) before it gets here:
        // what reaches `p` is what reaches the point where the line to the sun enters the air.
        let entry;
        let p = if (p - self.earth.center).norm() > self.atmosphere.radius {
            let o = p - self.atmosphere.center;
            let b = o.dot(&self.sun_direction);
            let disc = b * b - (o.norm_squared() - self.atmosphere.radius * self.atmosphere.radius);
            if b >= 0. || disc <= 0. {
                return Depths::default();
            }
            entry = p + self.sun_direction * (-b - disc.sqrt());
            &entry
        } else {
            p
        };
        let up = (p - self.earth.center).normalize();
        let altitude = (p - self.earth.center).norm() - self.earth.radius;
        let u = (altitude.max(0.) / top).sqrt().min(1.) * (SUN_TABLE_ALTITUDES - 1) as f64;
        let v = (up.dot(&self.sun_direction).clamp(-1., 1.) + 1.) / 2. * (SUN_TABLE_ANGLES - 1) as f64;
        let (i, j) = ((u as usize).min(SUN_TABLE_ALTITUDES - 2), (v as usize).min(SUN_TABLE_ANGLES - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let at = |i: usize, j: usize| table[i * SUN_TABLE_ANGLES + j];
        (at(i, j) * (1. - fv) + at(i, j + 1) * fv) * (1. - fu) + (at(i + 1, j) * (1. - fv) + at(i + 1, j + 1) * fv) * fu
    }

    /// The fraction of sunlight (per channel) that reaches `p` through the atmosphere.
    pub fn sun_transmittance(&self, p: &Vector3<f64>) -> Vector3<f64> {
        self.transmittance_of(self.sun_depths_at(p))
    }

    /// For light reaching `origin` along the unit direction `d` from `distance` away: the
    /// fraction that crosses the air between (per channel), and the light the air scatters
    /// towards `origin` on the way, on the same scale as the sky's radiance. This is the sky's
    /// own integral (`scattering`), stopped at `distance`, so that far enough away an object
    /// fades into the sky behind it. The air is lit by the sun as if nothing shadowed it.
    pub fn segment(&self, origin: &Vector3<f64>, d: &Vector3<f64>, distance: f64) -> (Vector3<f64>, Vector3<f64>) {
        let r = Ray { ro: *origin, rd: *d };
        let (start, end) = match self.view_ray_span(&r) {
            Some((start, end)) if start < distance => (start, end.min(distance)),
            _ => return (Vector3::repeat(1.), Vector3::zeros()),
        };
        let mut depth = Depths::default();
        let (mut rayleigh, mut mie, mut multiple) = (Vector3::zeros(), Vector3::zeros(), Vector3::zeros());
        for (t, step) in self.span_samples(&r, start, end, DEFAULT_NUM_SAMPLES) {
            let p = origin + d * t;
            let height = (p - self.earth.center).norm() - self.earth.radius;
            let here = self.depths(height, step);
            // To the middle of this step.
            depth = depth + here * 0.5;
            multiple += self.transmittance_of(depth)
                .component_mul(&self.scattered(here))
                .component_mul(&self.multiple_at(&p));
            let attenuation = self.transmittance_of(depth + self.sun_depths_at(&p));
            rayleigh += attenuation * here.r;
            mie += attenuation * here.m;
            depth = depth + here * 0.5;
        }
        let inscattered = self.apply_phase(d, rayleigh.component_mul(&self.rayleigh_coefficients), mie * self.mie_coefficient, multiple)
            * SKY_CALIBRATION;
        (self.transmittance_of(depth), inscattered)
    }
}

/// Shadows in the air: crepuscular rays.
impl SkyMaterial {
    /// Sunlight scattered once towards `origin` along the unit direction `d`, step by step out
    /// to `distance` (or through the whole atmosphere): each step's distance, length and
    /// radiance, on the scale of `segment`'s and the sky's.
    pub fn single_scattering_steps(&self, origin: &Vector3<f64>, d: &Vector3<f64>, distance: f64) -> Vec<(f64, f64, Vector3<f64>)> {
        let r = Ray { ro: *origin, rd: *d };
        let (start, end) = match self.view_ray_span(&r) {
            Some((start, end)) if start < distance => (start, end.min(distance)),
            _ => return vec![],
        };
        let mu = d.dot(&self.sun_direction);
        let (rayleigh, mie) = (Self::rayleigh_phase(mu), Self::mie_phase(mu, self.mie_anisotropy));
        let scale = self.brightness * SKY_CALIBRATION;
        let mut depth = Depths::default();
        self.span_samples(&r, start, end, DEFAULT_NUM_SAMPLES).into_iter().map(|(t, step)| {
            let p = origin + d * t;
            let here = self.depths((p - self.earth.center).norm() - self.earth.radius, step);
            depth = depth + here * 0.5;
            let lit = self.transmittance_of(depth + self.sun_depths_at(&p));
            let scattered = self.rayleigh_coefficients * (here.r * rayleigh) + Vector3::repeat(self.mie_coefficient * here.m * mie);
            depth = depth + here * 0.5;
            (t, step, lit.component_mul(&scattered) * scale)
        }).collect()
    }

    /// The part of the sunlight scattered once along the view ray (as in the sky's radiance and
    /// `segment`, which take the air to be lit everywhere outside the earth's shadow) that
    /// doesn't arrive because the air there is in shadow: behind clouds, below mountains.
    /// `lit(p)` is the fraction of sunlight reaching `p`. Taking it away leaves bright air where
    /// the sun shines through gaps and dim air behind clouds: crepuscular rays.
    ///
    /// Testing every step for shadow would cost a shadow ray each; instead `samples` steps are
    /// chosen with probability proportional to the light they scatter, a point picked uniformly
    /// in each, and the shadow there stands for the step's. For step k chosen with probability
    /// w_k / W, c_k (1 - lit) W / w_k is an unbiased estimate of the loss Σ c_i (1 - lit_i), and
    /// it is least noisy where the light, and so what a shadow takes, is greatest.
    pub fn shadowed_loss(&self, origin: &Vector3<f64>, d: &Vector3<f64>, distance: f64, samples: usize,
                         lit: impl Fn(&Vector3<f64>) -> f64) -> Vector3<f64> {
        let steps = self.single_scattering_steps(origin, d, distance);
        let weights: Vec<f64> = steps.iter().map(|(_, _, c)| c.sum()).collect();
        let total: f64 = weights.iter().sum();
        if total <= 0. || samples == 0 {
            return Vector3::zeros();
        }
        let mut loss = Vector3::zeros();
        for _ in 0..samples {
            let mut u = rand() * total;
            let k = weights.iter().position(|w| { u -= w; u < 0. }).unwrap_or(weights.len() - 1);
            let (t, step, c) = steps[k];
            if weights[k] <= 0. {
                continue;
            }
            let shade = 1. - lit(&(origin + d * (t + (rand() - 0.5) * step)));
            if shade > 0. {
                loss += c * (shade * total / weights[k]);
            }
        }
        loss / samples as f64
    }
}

impl MaterialModel for SkyMaterial {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        // Linear light; display encoding happens at output.
        let v = self.radiance(r) * SKY_CALIBRATION;
        ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::new(v.x, v.y, v.z), ray: None }
    }
}

/// Optical depths along a path: the integrals of the Rayleigh and Mie scatterers' relative
/// densities (in metres of sea-level air), and of ozone's (in metres at its peak).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Depths {
    r: f64,
    m: f64,
    o: f64,
}

impl std::ops::Add for Depths {
    type Output = Depths;
    fn add(self, b: Depths) -> Depths {
        Depths { r: self.r + b.r, m: self.m + b.m, o: self.o + b.o }
    }
}

impl std::ops::Mul<f64> for Depths {
    type Output = Depths;
    fn mul(self, k: f64) -> Depths {
        Depths { r: self.r * k, m: self.m * k, o: self.o * k }
    }
}

/// Ozone's absorption coefficients (per metre, at the peak of its layer) for the red, green and
/// blue channels (680, 550 and 440 nm): Bruneton's 2017 "Precomputed Atmospheric Scattering:
/// a New Implementation" and Hillaire 2020 (table 1). It absorbs most in the green and orange
/// (the Chappuis band), which keeps long, low paths from tinting green and deepens the blue of
/// twilight.
const OZONE_ABSORPTION: [f64; 3] = [0.650e-6, 1.881e-6, 0.085e-6];

/// Rayleigh and Mie in-scattering, before their phase functions, and light scattered more than
/// once (`SkyMaterial::scattering`).
type Scattering = (Vector3<f64>, Vector3<f64>, Vector3<f64>);

/// # Multiple scattering
///
/// Light scattered once is not the end of it: some of what the air scatters is scattered again,
/// and again. Single scattering alone leaves the horizon too dark and, along its long paths,
/// tinted green (blue scattered out of the view before it arrives, red scattered too little
/// into it); the light of further orders, from all round, is what makes it pale.
///
/// Following Hillaire 2020, section 5.5, the light arriving at a point after two or more
/// scatterings is taken as isotropic, and found from the second order alone: integrate, over
/// directions ω all round the point (uniformly over the sphere), the sunlight singly scattered
/// towards it from along ω, with an isotropic phase function p_u = 1/4π, plus the sunlit ground
/// reflected from where ω meets it,
///
/// ```text
/// L₂ = ∫ p_u ∫ T(x, y) σ_s(y) T_sun(y) p_u dt dω   (+ ground)
/// ```
///
/// and the fraction of light leaving the point that the air around it scatters back to it,
///
/// ```text
/// f = ∫ p_u ∫ T(x, y) σ_s(y) dt dω.
/// ```
///
/// Each further order is assumed to be f times the last, so all of them together are the
/// geometric series Ψ = L₂ (1 + f + f² + ...) = L₂ / (1 - f). Ψ depends only on the point's
/// altitude and the sun's angle from its zenith, and is tabulated over both; along a view ray,
/// each sample then adds σ_s Ψ, as single scattering adds σ_s p(θ) T_sun.
pub struct MultipleScattering {
    psi: Vec<Vector3<f64>>,
}

const MS_ALTITUDES: usize = 32;
const MS_SUN_ANGLES: usize = 32;
const MS_DIRECTIONS: usize = 64;
const MS_STEPS: usize = 20;

impl MultipleScattering {
    fn top(sky: &SkyMaterial) -> f64 {
        sky.atmosphere.radius - sky.earth.radius
    }

    pub fn build(sky: &SkyMaterial) -> MultipleScattering {
        // Directions spread evenly over the sphere: a Fibonacci spiral.
        let golden = std::f64::consts::PI * (3. - 5f64.sqrt());
        let directions: Vec<Vector3<f64>> = (0..MS_DIRECTIONS).map(|i| {
            let y = 1. - 2. * (i as f64 + 0.5) / MS_DIRECTIONS as f64;
            let r = (1. - y * y).sqrt();
            Vector3::new(r * (golden * i as f64).cos(), y, r * (golden * i as f64).sin())
        }).collect();
        let p_u = 1. / (4. * std::f64::consts::PI);
        let top = Self::top(sky);
        let psi = (0..MS_ALTITUDES * MS_SUN_ANGLES).into_par_iter().map(|k| {
            let (i, j) = (k / MS_SUN_ANGLES, k % MS_SUN_ANGLES);
            let altitude = top * (i as f64 / (MS_ALTITUDES - 1) as f64).powi(2);
            let mu = -1. + 2. * j as f64 / (MS_SUN_ANGLES - 1) as f64;
            let sun = Vector3::new((1. - mu * mu).max(0.).sqrt(), mu, 0.);
            let x = sky.earth.center + Vector3::new(0., sky.earth.radius + altitude.clamp(1., top - 1.), 0.);
            let (mut second, mut transfer) = (Vector3::zeros(), Vector3::zeros());
            for d in &directions {
                let r = Ray { ro: x, rd: *d };
                let (start, length) = match sky.view_ray_span(&r) { Some(s) => s, None => continue };
                let mut depth = Depths::default();
                for (t, step) in sky.span_samples(&r, start, length, MS_STEPS) {
                    let y = x + d * t;
                    let h = (y - sky.earth.center).norm() - sky.earth.radius;
                    let here = sky.depths(h, step);
                    let t_view = sky.transmittance_of(depth + here * 0.5);
                    let scattered = t_view.component_mul(&sky.scattered(here));
                    transfer += scattered;
                    if let Some(to_sun) = sky.optical_depth_towards(y, sun) {
                        second += scattered.component_mul(&sky.transmittance_of(to_sun)) * p_u;
                    }
                    depth = depth + here;
                }
                // Sunlit ground where the direction meets it, reflected diffusely.
                if let Some(ground) = sky.earth.intersects(&r).filter(|g| g.dist <= length * (1. + 1e-9)) {
                    let y = x + d * ground.dist;
                    let normal = (y - sky.earth.center).normalize();
                    let cos = normal.dot(&sun);
                    let to_sun = if cos > 0. { sky.optical_depth_towards(y + normal, sun) } else { None };
                    if let Some(to_sun) = to_sun {
                        let lit = sky.transmittance_of(to_sun) * (cos * sky.ground_albedo / std::f64::consts::PI);
                        second += sky.transmittance_of(depth).component_mul(&lit);
                    }
                }
            }
            // The integrals over the sphere with p_u: the mean over evenly spread directions.
            let n = MS_DIRECTIONS as f64;
            let (second, f) = (second / n, transfer / n);
            Vector3::new(second.x / (1. - f.x), second.y / (1. - f.y), second.z / (1. - f.z))
        }).collect();
        MultipleScattering { psi }
    }

    /// Ψ at `altitude`, with the sun at cosine `mu` from the zenith: bilinear in the table.
    fn lookup(&self, altitude: f64, mu: f64, sky: &SkyMaterial) -> Vector3<f64> {
        let u = (altitude.max(0.) / Self::top(sky)).sqrt().min(1.) * (MS_ALTITUDES - 1) as f64;
        let v = (mu.clamp(-1., 1.) + 1.) / 2. * (MS_SUN_ANGLES - 1) as f64;
        let (i, j) = ((u as usize).min(MS_ALTITUDES - 2), (v as usize).min(MS_SUN_ANGLES - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let at = |i: usize, j: usize| self.psi[i * MS_SUN_ANGLES + j];
        (at(i, j) * (1. - fv) + at(i, j + 1) * fv) * (1. - fu) + (at(i + 1, j) * (1. - fv) + at(i + 1, j + 1) * fv) * fu
    }

    /// The fraction of light around each altitude and sun angle that is scattered more than
    /// once, for tests.
    #[cfg(test)]
    fn table(&self) -> &[Vector3<f64>] {
        &self.psi
    }
}

/// # Sky-view lookup table
///
/// Integrating the atmosphere for every ray that escapes the scene was the dominant cost of
/// sky scenes (`exp` alone was 63% of samples). The sky changes slowly with direction, so it
/// can be precomputed into a table and interpolated, following the Sky-View LUT of Sebastien
/// Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique",
/// Computer Graphics Forum 39(4), 2020, section 5.3, doi:10.1111/cgf.14050.
///
/// The table stores the Rayleigh and Mie in-scattering separately, before their phase
/// functions are applied, indexed by latitude l (elevation above the horizon) and longitude.
/// The phase functions depend only on the angle to the sun and are applied exactly at lookup:
/// the Mie phase function peaks too sharply around the sun to interpolate (tabulating finished
/// radiance put a few percent of error there). Colour changes
/// fastest near the horizon, so rows are spaced non-linearly to put more of them there, with
/// Hillaire's mapping v = 0.5 + 0.5 sign(l) sqrt(|l| / (pi/2)); this table covers only the
/// upper hemisphere, so v = sqrt(l / (pi/2)).
///
/// Differences from the paper:
///
/// - Hillaire rebuilds the table for each camera position. Here rays start anywhere in the
///   scene, and near the horizon the sky depends on altitude (a table for 1 m was 3-5% out at
///   30 m). So there is a table for each of several altitudes, `LUT_ALTITUDES`, interpolated
///   linearly, which is the altitude dimension of Eric Bruneton and Fabrice Neyret's
///   "Precomputed Atmospheric Scattering", Computer Graphics Forum 27(4), 2008,
///   doi:10.1111/j.1467-8659.2008.01245.x. Above the highest, rays are integrated exactly.
/// - Rays below the horizon are integrated exactly. Their radiance depends strongly on
///   altitude (how soon they reach the ground), and they are rarely seen, being mostly hidden by
///   geometry.
/// - The sun is fixed for the scene, so it needs no dimension of its own.
pub struct SkyViewLUT {
    width: usize,
    height: usize,
    /// One table per altitude in `LUT_ALTITUDES`, of Rayleigh and Mie scattering before the
    /// phase functions (`SkyMaterial::scattering`).
    slices: Vec<Vec<Scattering>>,
}

/// Heights above the ground, in metres, at which the sky is tabulated: closely spaced, since
/// interpolating between distant altitudes is poor near the horizon (2-4% between 300 m and
/// 1 km). Scenes keep within tens of metres of the ground; above the top, rays are integrated
/// exactly.
const LUT_ALTITUDES: [f64; 8] = [1., 5., 15., 30., 60., 100., 150., 200.];
/// Longitude and latitude resolution of the table.
const LUT_WIDTH: usize = 256;
const LUT_HEIGHT: usize = 128;

impl SkyViewLUT {
    pub fn build(sky: &SkyMaterial, width: usize, height: usize) -> SkyViewLUT {
        let slices = LUT_ALTITUDES.iter().map(|altitude| {
            let origin = sky.earth.center + Vector3::new(0., sky.earth.radius + altitude, 0.);
            (0 .. width * height).into_par_iter().map(|i| {
                let (x, y) = (i % width, i / width);
                let rd = SkyViewLUT::direction(x as f64 / width as f64, y as f64 / (height - 1) as f64);
                sky.scattering(&Ray { ro: origin, rd })
            }).collect()
        }).collect();
        SkyViewLUT { width, height, slices }
    }

    /// The direction for texture coordinates u (longitude) and v (latitude), in [0, 1].
    fn direction(u: f64, v: f64) -> Vector3<f64> {
        let l = v * v * std::f64::consts::FRAC_PI_2;
        let phi = u * 2. * std::f64::consts::PI;
        Vector3::new(l.cos() * phi.cos(), l.sin(), l.cos() * phi.sin())
    }

    /// Rayleigh and Mie scattering for a unit direction at or above the horizon, from
    /// `altitude` metres up: bilinear within the two nearest altitude tables, linear between
    /// them. None above the highest table.
    pub fn lookup(&self, d: &Vector3<f64>, altitude: f64) -> Option<Scattering> {
        let top = LUT_ALTITUDES.len() - 1;
        if altitude > LUT_ALTITUDES[top] {
            return None;
        }
        let a = altitude.max(LUT_ALTITUDES[0]);
        let i = LUT_ALTITUDES.partition_point(|h| *h <= a).clamp(1, top);
        let f = (a - LUT_ALTITUDES[i - 1]) / (LUT_ALTITUDES[i] - LUT_ALTITUDES[i - 1]);
        let (r0, m0, s0) = self.lookup_slice(i - 1, d);
        let (r1, m1, s1) = self.lookup_slice(i, d);
        Some((r0 * (1. - f) + r1 * f, m0 * (1. - f) + m1 * f, s0 * (1. - f) + s1 * f))
    }

    fn lookup_slice(&self, slice: usize, d: &Vector3<f64>) -> Scattering {
        let l = d.y.clamp(0., 1.).asin();
        let v = (l / std::f64::consts::FRAC_PI_2).sqrt() * (self.height - 1) as f64;
        let u = (d.z.atan2(d.x) / (2. * std::f64::consts::PI)).rem_euclid(1.) * self.width as f64;

        let (x0, y0) = (u.floor() as usize % self.width, (v.floor() as usize).min(self.height - 2));
        let (x1, y1) = ((x0 + 1) % self.width, y0 + 1);
        let (fx, fy) = (u - u.floor(), v - y0 as f64);
        let texels = &self.slices[slice];
        let mix = |pick: fn(&Scattering) -> Vector3<f64>| {
            let t = |x: usize, y: usize| pick(&texels[y * self.width + x]);
            (t(x0, y0) * (1. - fx) + t(x1, y0) * fx) * (1. - fy) + (t(x0, y1) * (1. - fx) + t(x1, y1) * fx) * fy
        };
        (mix(|p| p.0), mix(|p| p.1), mix(|p| p.2))
    }
}

/// The sky, looked up in a `SkyViewLUT` above the horizon and integrated exactly below it. The
/// table is built when the sky is first needed.
pub struct LUTSky {
    sky: SkyMaterial,
    lut: std::sync::OnceLock<SkyViewLUT>,
}

impl MaterialModel for LUTSky {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        let d = r.rd.normalize();
        let altitude = (r.ro - self.sky.earth.center).norm() - self.sky.earth.radius;
        let looked_up = if d.y >= 0. {
            self.lut.get_or_init(|| SkyViewLUT::build(&self.sky, LUT_WIDTH, LUT_HEIGHT)).lookup(&d, altitude)
        } else {
            None
        };
        // The phase functions, which peak sharply around the sun, are applied exactly.
        let v = match looked_up {
            Some((rayleigh, mie, multiple)) => self.sky.apply_phase(&d, rayleigh, mie, multiple),
            None => self.sky.radiance(r),
        } * SKY_CALIBRATION;
        ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::new(v.x, v.y, v.z), ray: None }
    }
}

/// Get a SkyPreset by name
/// 
/// # Arguments
/// * `name` - Name of the preset
///
/// # Returns
/// The preset if found, or the default Earth preset
fn get_preset_by_name(name: &str) -> &SkyPreset {
    for preset in &SKY_PRESETS {
        if preset.name == name {
            return preset;
        }
    }
    // Default to first preset (Earth)
    &SKY_PRESETS[0]
}

/// Creates a sky sphere object from JSON configuration
///
/// # Arguments
/// * `o` - JSON value containing the sky configuration
///
/// # Returns
/// A SceneObject representing the sky sphere
impl SkyMaterial {
    pub fn earth_radius(&self) -> f64 {
        self.earth.radius
    }

    pub fn sun_direction(&self) -> Vector3<f64> {
        self.sun_direction
    }

    /// The fraction of sunlight (per colour channel) that reaches `altitude` above the top of
    /// the earth through the atmosphere: exp(-τ), with the same optical depths and extinction
    /// as `scattering` uses for light reaching each point it samples. None of it when the earth
    /// is in the way.
    pub fn sun_transmittance_at(&self, altitude: f64) -> Vector3<f64> {
        let point = self.earth.center + Vector3::new(0., self.earth.radius + altitude.max(1.), 0.);
        self.optical_depth_to_sun(point).map_or(Vector3::zeros(), |d| self.transmittance_of(d))
    }
}

pub fn create_sky_sphere(o: &Value) -> SceneObject {
    let sky = parse_sky_material(o);
    // The lookup table is on by default; "lut": false integrates every ray exactly.
    let m: Box<dyn MaterialModel + Sync + Send> = if o["lut"].as_bool().unwrap_or(true) {
        Box::new(LUTSky { sky, lut: std::sync::OnceLock::new() })
    } else {
        Box::new(sky)
    };
    SceneObject {
        geometry: Box::new(Infinite {}),
        medium: Box::new(Solid { m }),
    }
}

/// The sky described by a `skysphere` object.
pub fn parse_sky_material(o: &Value) -> SkyMaterial {
    let earth_location = SceneFile::parse_vec3_def(o, "earth_location", Vector3::new(0., -6360e3, 0.));
    
    // Handle preset or custom scattering parameters
    let preset_name = match o.get("preset") {
        Some(v) => v.as_str().unwrap_or("earth"),
        None => "earth"
    };
    
    let preset = get_preset_by_name(preset_name);
    
    // Get rayleigh coefficients (custom or from preset)
    let rayleigh_r = SceneFile::parse_number(&o["rayleigh_r"], preset.rayleigh_r);
    let rayleigh_g = SceneFile::parse_number(&o["rayleigh_g"], preset.rayleigh_g);
    let rayleigh_b = SceneFile::parse_number(&o["rayleigh_b"], preset.rayleigh_b);
    let rayleigh_coefficients = Vector3::new(rayleigh_r, rayleigh_g, rayleigh_b);
    
    // Get mie coefficient and anisotropy (custom or from preset)
    let mie_coefficient = SceneFile::parse_number(&o["mie_coefficient"], preset.mie_coefficient);
    let mie_anisotropy = SceneFile::parse_number(&o["mie_anisotropy"], preset.mie_anisotropy);
    
    // Get thickness values (either custom or from preset)
    let rayleigh_thickness = SceneFile::parse_number(&o["rayleigh_thickness"], preset.rayleigh_thickness);
    let mie_thickness = SceneFile::parse_number(&o["mie_thickness"], preset.mie_thickness);
    
    SkyMaterial {
        earth: Sphere::new(
            earth_location,
            SceneFile::parse_number(&o["earth_radius"], 6360e3),
        ),
        atmosphere: Sphere::new(
            earth_location,
            SceneFile::parse_number(&o["atmosphere_radius"], 6420e3)
        ),
        rayleigh_thickness,
        mie_thickness,
        sun_direction: SceneFile::parse_vec3_def(o, "sun_direction", Vector3::new(0., 0.5, 2.)).normalize(),
        brightness: SceneFile::parse_number(&o["brightness"], 20.),
        rayleigh_coefficients,
        mie_coefficient,
        mie_anisotropy,
        sun_table: std::sync::OnceLock::new(),
        ozone: if o["ozone"].as_bool() == Some(false) { Vector3::zeros() } else { Vector3::from(OZONE_ABSORPTION) },
        multiple_scattering: o["multiple_scattering"].as_bool().unwrap_or(true),
        ground_albedo: SceneFile::parse_number(&o["ground_albedo"], 0.2),
        ms_table: std::sync::OnceLock::new(),
        crepuscular_rays: o["crepuscular_rays"].as_bool().unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn sky_at(sun: &str) -> SkyMaterial {
        parse_sky_material(&serde_json::from_str(&format!(r#"{{ "type": "skysphere", "sun_direction": {} }}"#, sun)).unwrap())
    }

    fn single_scattering_sky_at(sun: &str) -> SkyMaterial {
        SkyMaterial { multiple_scattering: false, ..sky_at(sun) }
    }

    #[test]
    fn test_ozone_absorbs_green() {
        // The vertical column: 15 km at peak density, 1 - exp(-1.881e-6 x 15000) = 2.8% of green,
        // little of blue. Lower suns cross more of it.
        let with = sky_at("[0, 1, 0]");
        let without = SkyMaterial { ozone: Vector3::zeros(), ..sky_at("[0, 1, 0]") };
        let ratio = with.sun_transmittance_at(0.).component_div(&without.sun_transmittance_at(0.));
        assert!((ratio.y - (-1.881e-6f64 * 15000.).exp()).abs() < 0.002, "{}", ratio);
        assert!(ratio.z > ratio.x && ratio.x > ratio.y, "{}", ratio);
        let (low_with, low_without) = (sky_at("[0, 0.05, 1]"), SkyMaterial { ozone: Vector3::zeros(), ..sky_at("[0, 0.05, 1]") });
        let low = low_with.sun_transmittance_at(0.).component_div(&low_without.sun_transmittance_at(0.));
        assert!(low.y < ratio.y - 0.05, "{} vs {}", low, ratio);
    }

    #[test]
    fn test_multiple_scattering_whitens_the_horizon() {
        // Single scattering alone integrates a green horizon from 900 m; with every order of
        // scattering it is pale, blue at least as strong as green, and the sky only brighter.
        let o = Vector3::new(0., 900., 0.);
        let horizon = Ray { ro: o, rd: Vector3::new(0., 0.002, 1.).normalize() };
        // Without ozone, which takes some of the green out by itself.
        let single = SkyMaterial { ozone: Vector3::zeros(), ..single_scattering_sky_at("[-0.6, 0.55, -0.5]") }.radiance(&horizon);
        let full = sky_at("[-0.6, 0.55, -0.5]").radiance(&horizon);
        assert!(single.y > single.z, "expected the single-scattering horizon green: {}", single);
        assert!(full.z >= full.y && full.x > single.x, "{} vs {}", full, single);
        // Overhead the second and later orders add something, but less than the first.
        let up = Ray { ro: o, rd: Vector3::new(0., 1., 0.) };
        let (s, f) = (single_scattering_sky_at("[-0.6, 0.55, -0.5]").radiance(&up), sky_at("[-0.6, 0.55, -0.5]").radiance(&up));
        for c in 0..3 {
            assert!(f[c] > s[c] && f[c] < 2. * s[c], "zenith {} vs {}", f, s);
        }
    }

    #[test]
    fn test_multiple_scattering_table() {
        let sky = sky_at("[0, 1, 0]");
        let table = MultipleScattering::build(&sky);
        // Positive everywhere, and largest low down (where the air is), with the sun up.
        // Zero only where all the air in sight is in the earth's shadow.
        assert!(table.table().iter().all(|p| p.min() >= 0. && p.max().is_finite()));
        let low = table.lookup(100., 1., &sky);
        let high = table.lookup(40000., 1., &sky);
        let night = table.lookup(100., -0.9, &sky);
        assert!(low.z > high.z && low.z > 5. * night.z, "{} {} {}", low, high, night);
    }

    /// The sky's radiance and the sun's transmittance by brute force: thousands of even steps.
    fn brute_force(sky: &SkyMaterial, r: &Ray) -> (Vector3<f64>, Vector3<f64>) {
        let n = 4096;
        let height = |p: Vector3<f64>| (p - sky.earth.center).norm() - sky.earth.radius;
        let depth_to_sun = |p: Vector3<f64>| -> Option<Depths> {
            let lr = Ray { ro: p, rd: sky.sun_direction };
            if sky.earth.intersects(&lr).is_some() { return None; }
            let len = sky.atmosphere.intersects(&lr)?.dist;
            let step = len / n as f64;
            Some((0..n).fold(Depths::default(), |acc, j| {
                acc + sky.depths(height(p + (j as f64 + 0.5) * step * sky.sun_direction), step)
            }))
        };
        let (start, end) = sky.view_ray_span(r).unwrap();
        let step = (end - start) / n as f64;
        let mut depth = Depths::default();
        let (mut ray, mut mie) = (Vector3::zeros(), Vector3::zeros());
        for i in 0..n {
            let p = r.ro + r.rd * (start + (i as f64 + 0.5) * step);
            let here = sky.depths(height(p), step);
            if let Some(to_sun) = depth_to_sun(p) {
                let t = sky.transmittance_of(depth + here * 0.5 + to_sun);
                ray += t * here.r;
                mie += t * here.m;
            }
            depth = depth + here;
        }
        let radiance = sky.apply_phase(&r.rd, ray.component_mul(&sky.rayleigh_coefficients), mie * sky.mie_coefficient, Vector3::zeros());
        let ground = sky.earth.center + Vector3::new(0., sky.earth.radius + 1., 0.);
        (radiance, sky.transmittance_of(depth_to_sun(ground).unwrap()))
    }

    #[test]
    fn test_sky_integration_matches_brute_force() {
        // Near the horizon, where even steps of tens of kilometres went wrong, and elsewhere;
        // from the ground and from 900 m; the sun high and low.
        for (sun, from, d) in [
            ("[-0.6, 0.55, -0.5]", 900., Vector3::new(0., 0.002, 1.)),
            ("[-0.6, 0.55, -0.5]", 2., Vector3::new(0., 0.01, 1.)),
            ("[0, 0.31, 0.95]", 2., Vector3::new(0.4, 0.05, 1.)),
            ("[0, 0.087, 1]", 2., Vector3::new(0., 0.3, 1.)),
            ("[0, 0.087, 1]", 2., Vector3::new(0., 0.02, -1.)),
        ] {
            let sky = single_scattering_sky_at(sun);
            let r = Ray { ro: Vector3::new(0., from, 0.), rd: d.normalize() };
            let (radiance, sun_t) = brute_force(&sky, &r);
            let ours = sky.radiance(&r);
            for c in 0..3 {
                assert!((ours[c] / radiance[c] - 1.).abs() < 0.03, "{} from {}: {} vs {}", sun, from, ours, radiance);
            }
            let t = sky.sun_transmittance_at(1.);
            for c in 0..3 {
                assert!((t[c] / sun_t[c] - 1.).abs() < 0.01, "{}: sun {} vs {}", sun, t, sun_t);
            }
        }
    }

    #[test]
    fn test_sky_from_orbit() {
        // From 400 km, outside the atmosphere: the limb (grazing 10 km and 30 km up, where the
        // air is all far from the viewer) matches brute-force integration over its span.
        let sky = single_scattering_sky_at("[0.6, 0.3, 1]");
        let eye = Vector3::new(0., 400_000., 0.);
        let r_eye = sky.earth.radius + 400_000.;
        for tangent in [10_000., 30_000.] {
            // Down at the angle that just grazes `tangent` above the ground.
            let below = ((sky.earth.radius + tangent) / r_eye).acos();
            let r = Ray { ro: eye, rd: Vector3::new(0., -below.sin(), below.cos()) };
            let (radiance, _) = brute_force(&sky, &r);
            let ours = sky.radiance(&r);
            for c in 0..3 {
                assert!((ours[c] / radiance[c] - 1.).abs() < 0.03, "tangent {}: {} vs {}", tangent, ours, radiance);
            }
        }
        // Space beyond the limb is black.
        assert_eq!(sky.radiance(&Ray { ro: eye, rd: Vector3::new(0., 0.2, 1.).normalize() }), Vector3::zeros());
        // Looking straight down, the ground is seen through the whole column of air.
        let (t, inscattered) = sky.segment(&eye, &Vector3::new(0., -1., 0.), 400_000.);
        let column = sky_at("[0, 1, 0]").sun_transmittance_at(0.);
        assert!(inscattered.max() > 0.);
        for c in 0..3 {
            assert!((t[c] / column[c] - 1.).abs() < 0.01, "{} vs {}", t, column);
        }
    }

    #[test]
    fn test_sun_transmittance_anywhere_on_the_planet() {
        let sky = sky_at("[0, 0.4, 1]");
        let ground = Vector3::new(0., 1., 0.);
        let exact = |p: Vector3<f64>| sky.optical_depth_to_sun(p).map_or(Vector3::zeros(), |d| sky.transmittance_of(d));
        // Here, and 2000 km away towards the sun, where it stands higher, and away from it,
        // where it is lower; then on the night side.
        let toward = sky.earth.center + Vector3::new(0., 0.95, 0.3).normalize() * (sky.earth.radius + 1.);
        let away = sky.earth.center + Vector3::new(0., 0.95, -0.3).normalize() * (sky.earth.radius + 1.);
        for p in [ground, toward, away] {
            let (table, e) = (sky.sun_transmittance(&p), exact(p));
            for c in 0..3 {
                assert!((table[c] - e[c]).abs() < 0.01 * e[c].max(0.05), "{}: {} vs {}", p, table, e);
            }
        }
        assert!(sky.sun_transmittance(&toward).z > sky.sun_transmittance(&away).z);
        let night = sky.earth.center + Vector3::new(0., -0.3, -1.).normalize() * (sky.earth.radius + 1000.);
        assert!(sky.sun_transmittance(&night).max() < 1e-6);
    }

    #[test]
    fn test_sun_transmittance_above_the_atmosphere() {
        // From 400 km, with the sun 19.3° below the local horizontal: its light grazes the air
        // 20 km up on the way (reddened), though from the top of the atmosphere at the same
        // angle it would be behind the earth.
        let sky = sky_at("[0, -0.3305, 0.9438]");
        let eye = Vector3::new(0., 400_000., 0.);
        let o = eye - sky.atmosphere.center;
        let b = o.dot(&sky.sun_direction);
        let entry = eye + sky.sun_direction * (-b - (b * b - o.norm_squared() + sky.atmosphere.radius.powi(2)).sqrt());
        let exact = sky.transmittance_of(sky.optical_depth_to_sun(entry + sky.sun_direction).unwrap());
        let t = sky.sun_transmittance(&eye);
        // The table is coarse where the light grazes the ground (blue is 8% off here).
        for c in 0..3 {
            assert!((t[c] - exact[c]).abs() < 0.1 * exact[c], "{} vs {}", t, exact);
        }
        assert!(t.x > t.z && t.z > 0.01 && t.x < 1.);
        // A sun whose light passes above the air, 87 km up, or never comes near it: all of it.
        for sun in ["[0, -0.3, 0.954]", "[0, 0.1, 1]"] {
            assert_eq!(sky_at(sun).sun_transmittance(&eye), Vector3::repeat(1.));
        }
        // Grazing below the ground: none.
        assert!(sky_at("[0, -0.345, 0.938]").sun_transmittance(&eye).max() < 1e-6);
    }

    #[test]
    fn test_unbounded_segment_is_the_sky() {
        // Far enough, an object fades into exactly the sky behind it.
        let sky = sky_at("[0.3, 0.3, 1]");
        for d in [Vector3::new(0., 0.2, 1.), Vector3::new(1., 0.05, 0.), Vector3::new(-0.3, 0.7, -0.5)] {
            let d = d.normalize();
            let o = Vector3::new(0., 2., 0.);
            let (t, inscattered) = sky.segment(&o, &d, f64::INFINITY);
            let radiance = sky.radiance(&Ray { ro: o, rd: d }) * SKY_CALIBRATION;
            assert!(t.max() < 1.);
            for c in 0..3 {
                assert!((inscattered[c] / radiance[c] - 1.).abs() < 0.03, "{}: {} vs {}", d, inscattered, radiance);
            }
        }
    }

    #[test]
    fn test_segment_attenuates_like_the_air() {
        let sky = sky_at("[0, 1, 0]");
        let o = Vector3::new(0., 1., 0.);
        let (t, inscattered) = sky.segment(&o, &Vector3::new(1., 0., 0.), 10.);
        assert!(t.min() > 0.999 && inscattered.max() < 1e-3, "{} {}", t, inscattered);
        // 20 km along the ground: Beer-Lambert at sea-level density, to within the earth's
        // curvature lifting the far end 31 m.
        let (t, _) = sky.segment(&o, &Vector3::new(1., 0., 0.), 20000.);
        let expected = (sky.rayleigh_coefficients + Vector3::repeat(sky.mie_coefficient * 1.1)) * -20000.;
        for c in 0..3 {
            assert!((t[c] / expected[c].exp() - 1.).abs() < 0.01, "{} vs {}", t, expected.map(|x| x.exp()));
        }
        // Blue is lost first.
        assert!(t.z < t.y && t.y < t.x);
    }

    #[test]
    fn test_rayleigh_phase() {
        // Test with mu = 0 (perpendicular direction)
        let phase_perpendicular = SkyMaterial::rayleigh_phase(0.0);
        assert_relative_eq!(phase_perpendicular, 3.0 / (16.0 * std::f64::consts::PI), epsilon = 1e-10);
        
        // Test with mu = 1 (same direction)
        let phase_same = SkyMaterial::rayleigh_phase(1.0);
        assert_relative_eq!(phase_same, 3.0 / (8.0 * std::f64::consts::PI), epsilon = 1e-10);
        
        // Test with mu = -1 (opposite direction)
        let phase_opposite = SkyMaterial::rayleigh_phase(-1.0);
        assert_relative_eq!(phase_opposite, 3.0 / (8.0 * std::f64::consts::PI), epsilon = 1e-10);
    }
    
    #[test]
    fn test_mie_phase() {
        let g = 0.76; // Standard anisotropy factor used in implementation
        
        // Test with mu = 0 (perpendicular)
        let phase_perpendicular = SkyMaterial::mie_phase(0.0, g);
        assert!(phase_perpendicular > 0.0);
        
        // Test with mu = 1 (same direction) - should be maximum
        let phase_same = SkyMaterial::mie_phase(1.0, g);
        assert!(phase_same > SkyMaterial::mie_phase(0.0, g));
        
        // Test with mu = -1 (opposite direction) - should be minimum
        let phase_opposite = SkyMaterial::mie_phase(-1.0, g);
        assert!(phase_opposite < SkyMaterial::mie_phase(0.0, g));
    }
    
    #[test]
    fn test_rayleigh_coefficients_scale_with_inverse_fourth_power() {
        // Red, green and blue at 680, 550 and 440 nm.
        let scale = |nm: f64| (440. / nm).powi(4) * DEFAULT_RAYLEIGH_B;
        assert!((DEFAULT_RAYLEIGH_R / scale(680.) - 1.).abs() < 0.01, "{}", DEFAULT_RAYLEIGH_R);
        assert!((DEFAULT_RAYLEIGH_G / scale(550.) - 1.).abs() < 0.01, "{}", DEFAULT_RAYLEIGH_G);
    }

    #[test]
    fn test_no_sunlight_in_earths_shadow() {
        let sky = |sun: Vector3<f64>| SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: sun.normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
            sun_table: Default::default(),
            ozone: Vector3::zeros(),
            multiple_scattering: false,
            ground_albedo: 0.2,
            ms_table: Default::default(),
            crepuscular_rays: false,
        };
        // Just after sunset, the sun 0.02 radians below the horizon. From 100 m up, the sun ray
        // passes only slightly beneath the ground, so integrating through it would let a little
        // light through; the ground must block it entirely.
        let after_sunset = sky(Vector3::new(0., -0.02, 1.));
        assert_eq!(after_sunset.optical_depth_to_sun(Vector3::new(0., 100., 0.)), None);
        // From 5 km up the sun is still above the horizon (the dip of the horizon there is about
        // 0.04 radians).
        assert!(after_sunset.optical_depth_to_sun(Vector3::new(0., 5000., 0.)).is_some());
    }

    fn test_sky(sun: Vector3<f64>) -> SkyMaterial {
        SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: sun.normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
            sun_table: Default::default(),
            ozone: Vector3::zeros(),
            multiple_scattering: false,
            ground_albedo: 0.2,
            ms_table: Default::default(),
            crepuscular_rays: false,
        }
    }

    /// The largest error of the lookup table relative to the exact integral, over random
    /// directions above the horizon, for rays from each of `altitudes` metres up.
    fn lut_errors(sun: Vector3<f64>, altitudes: &[f64]) -> Vec<f64> {
        let sky = test_sky(sun);
        let lut = SkyViewLUT::build(&sky, LUT_WIDTH, LUT_HEIGHT);
        altitudes.iter().map(|altitude| {
            crate::geometry::seed_rand(11, 0);
            (0..300).map(|i| {
                // Half the directions within about 10 degrees of the sun, where the sky changes
                // fastest; half anywhere above the horizon.
                let d = crate::geometry::random_point_on_unit_sphere();
                let d = if i % 2 == 0 { (sky.sun_direction + d * 0.17).normalize() } else { d };
                let d = Vector3::new(d.x, d.y.abs(), d.z);
                let exact = sky.radiance(&Ray { ro: Vector3::new(0., *altitude, 0.), rd: d });
                let (rayleigh, mie, multiple) = lut.lookup(&d, *altitude).unwrap();
                (sky.apply_phase(&d, rayleigh, mie, multiple) - exact).norm() / exact.norm()
            }).fold(0., f64::max)
        }).collect()
    }

    #[test]
    fn test_lut_matches_exact_sky() {
        // Between the tabulated altitudes, with the sun at the horizon: the hardest case, since
        // the sky changes fastest there. (Measured worst errors: under 0.5% in either case.)
        let altitudes = [3., 45., 175.];
        for sun in [Vector3::new(0., 0.02, 1.)] {
            for (altitude, e) in altitudes.iter().zip(lut_errors(sun, &altitudes)) {
                assert!(e < 0.01, "sun {}, altitude {} m: relative error {}", sun, altitude, e);
            }
        }
    }

    #[test]
    fn test_lut_defers_to_exact_above_its_range() {
        let sky = test_sky(Vector3::new(0., 0.5, 1.));
        let lut = SkyViewLUT::build(&sky, 8, 4);
        assert!(lut.lookup(&Vector3::new(0., 1., 0.), 500.).is_none());
    }

    #[test]
    fn test_sky_radiance_is_linear() {
        let sky = |brightness: f64| SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: Vector3::new(0., 0.5, 1.).normalize(),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
            sun_table: Default::default(),
            ozone: Vector3::zeros(),
            multiple_scattering: false,
            ground_albedo: 0.2,
            ms_table: Default::default(),
            crepuscular_rays: false,
        };
        let r = Ray { ro: Vector3::new(0., 1., 0.), rd: Vector3::new(0., 0.3, 1.).normalize() };
        let (a, b) = (sky(20.).radiance(&r), sky(40.).radiance(&r));
        assert!(a.x > 0.);
        // No display encoding: doubling the light doubles the value.
        assert!((b - a * 2.).norm() < 1e-9 * b.norm());
    }

    #[test]
    fn test_view_ray_stops_at_ground() {
        let sky = SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., -6360e3, 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., -6360e3, 0.), 6360e3),
            sun_direction: Vector3::new(0., 1., 0.),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
            sun_table: Default::default(),
            ozone: Vector3::zeros(),
            multiple_scattering: false,
            ground_albedo: 0.2,
            ms_table: Default::default(),
            crepuscular_rays: false,
        };
        // Camera 10m above the surface, looking straight down.
        let down = Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., -1., 0.) };
        assert!((sky.view_ray_length(&down).unwrap() - 10.).abs() < 1e-3);
        // Looking up, the ray runs to the top of the atmosphere.
        let up = Ray { ro: Vector3::new(0., 10., 0.), rd: Vector3::new(0., 1., 0.) };
        assert!((sky.view_ray_length(&up).unwrap() - (60e3 - 10.)).abs() < 1e-3);
    }

    #[test]
    fn test_optical_depth() {
        // Create a simple sky material for testing
        let sky = SkyMaterial {
            atmosphere: Sphere::new(Vector3::new(0., 0., 0.), 6420e3),
            earth: Sphere::new(Vector3::new(0., 0., 0.), 6360e3),
            sun_direction: Vector3::new(0., 1., 0.),
            rayleigh_thickness: 7994.,
            mie_thickness: 1200.,
            brightness: 20.0,
            rayleigh_coefficients: Vector3::new(DEFAULT_RAYLEIGH_R, DEFAULT_RAYLEIGH_G, DEFAULT_RAYLEIGH_B),
            mie_coefficient: DEFAULT_MIE_COEFFICIENT,
            mie_anisotropy: DEFAULT_MIE_ANISOTROPY,
            sun_table: Default::default(),
            ozone: Vector3::zeros(),
            multiple_scattering: false,
            ground_albedo: 0.2,
            ms_table: Default::default(),
            crepuscular_rays: false,
        };
        
        // Test optical depth at different heights
        let segment_length = 1000.0; // 1000m segment
        
        // At height = 0 (sea level)
        let depth_sea_level_r = sky.optical_depth(0.0, sky.rayleigh_thickness, segment_length);
        let depth_sea_level_m = sky.optical_depth(0.0, sky.mie_thickness, segment_length);
        assert_relative_eq!(depth_sea_level_r, segment_length, epsilon = 1e-10);
        assert_relative_eq!(depth_sea_level_m, segment_length, epsilon = 1e-10);
        
        // At one scale height, optical depth should be reduced by factor of e^-1
        let depth_one_scale_r = sky.optical_depth(sky.rayleigh_thickness, sky.rayleigh_thickness, segment_length);
        assert_relative_eq!(depth_one_scale_r, segment_length / std::f64::consts::E, epsilon = 1e-10);
    }
    
    #[test]
    fn test_create_sky_sphere() {
        let json_str = r#"
        {
            "sun_direction": [0, 0.5, 1],
            "brightness": 30.0,
            "rayleigh_thickness": 8000.0,
            "mie_thickness": 1200.0
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj = create_sky_sphere(&value);
        
        // Just verify that a sky sphere was created successfully
        // We test for "infinite" bounds as a characteristic of Infinite geometry
        let bounds = sky_obj.geometry.bounds();
        assert!(bounds.min.x == std::f64::MIN);
        assert!(bounds.max.x == std::f64::MAX);
        
        // Test basic ray intersection - Infinite should always return Some intersection
        let test_ray = Ray {
            ro: Vector3::new(0.0, 0.0, 0.0),
            rd: Vector3::new(0.0, 0.0, 1.0)
        };
        assert!(sky_obj.geometry.intersects(&test_ray).is_some());
    }
    
    #[test]
    fn test_sky_presets() {
        // Test with default preset
        let json_str = r#"
        {
            "preset": "earth",
            "sun_direction": [0, 0.5, 1],
            "brightness": 30.0
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj = create_sky_sphere(&value);
        
        // Test with custom parameters that override preset
        let json_str = r#"
        {
            "preset": "sunset",
            "rayleigh_r": 5.0e-6,
            "mie_coefficient": 30.0e-6,
            "sun_direction": [0, 0.5, 1]
        }
        "#;
        
        let value: Value = serde_json::from_str(json_str).unwrap();
        let sky_obj_custom = create_sky_sphere(&value);
        
        // Both should create valid objects
        assert!(sky_obj.geometry.intersects(&Ray { ro: Vector3::new(0.0, 0.0, 0.0), rd: Vector3::new(0.0, 0.0, 1.0) }).is_some());
        assert!(sky_obj_custom.geometry.intersects(&Ray { ro: Vector3::new(0.0, 0.0, 0.0), rd: Vector3::new(0.0, 0.0, 1.0) }).is_some());
    }
    
    #[test]
    fn test_get_preset_by_name() {
        // Test getting existing preset
        let preset = get_preset_by_name("sunset");
        assert_eq!(preset.name, "sunset");
        
        // Test getting invalid preset (should return default Earth preset)
        let preset = get_preset_by_name("invalid");
        assert_eq!(preset.name, "earth");
    }

    #[test]
    fn test_shadowed_loss() {
        let sky = sky_at("[0, 0.3, 1]");
        let (o, d) = (Vector3::new(0., 2., 0.), Vector3::new(0., 0.2, 1.).normalize());
        let steps = sky.single_scattering_steps(&o, &d, f64::INFINITY);
        let all: Vector3<f64> = steps.iter().map(|s| s.2).sum();
        assert!(all.sum() > 0.);
        // Nothing in shadow: nothing lost. Everything in shadow: all of it.
        assert_eq!(sky.shadowed_loss(&o, &d, f64::INFINITY, 4, |_| 1.), Vector3::zeros());
        let n = 2000;
        let mean = (0..n).map(|_| sky.shadowed_loss(&o, &d, f64::INFINITY, 2, |_| 0.)).sum::<Vector3<f64>>() / n as f64;
        assert!((mean - all).norm() < 0.02 * all.norm(), "{} vs {}", mean, all);
        // In shadow beyond the start of the step nearest 5 km: the loss is the light from beyond
        // it, on average.
        let j = steps.iter().position(|s| s.0 > 5000.).unwrap();
        let edge = steps[j].0 - steps[j].1 / 2.;
        let beyond: Vector3<f64> = steps[j..].iter().map(|s| s.2).sum();
        let n = 20_000;
        let mean = (0..n).map(|_| sky.shadowed_loss(&o, &d, f64::INFINITY, 2, |p| if (p - o).norm() > edge { 0. } else { 1. })).sum::<Vector3<f64>>() / n as f64;
        assert!((mean - beyond).norm() < 0.05 * beyond.norm(), "{} vs {}", mean, beyond);
        // Steps sum to about the single scattering the sky adds up itself.
        let (r, m, _) = SkyMaterial { multiple_scattering: false, ..sky_at("[0, 0.3, 1]") }.scattering(&Ray { ro: o, rd: d });
        let single = sky.apply_phase(&d, r, m, Vector3::zeros()) * SKY_CALIBRATION;
        assert!((single - all).norm() < 0.05 * single.norm(), "{} vs {}", single, all);
    }
}