//! A giant planet: a flattened ball of cloud tops (`bands`), perhaps ringed (`rings`), lit by
//! the sun.
//!
//! **Its light.** What is seen is the top of a deep cloud deck under a little clear gas. Light
//! scattered many times in a semi-infinite cloud of single-scattering albedo near 1 leaves it
//! as Chandrasekhar's solution says, which Hapke's model (`hapke`) approximates, with H
//! functions for the multiple scattering and the particles' phase function for the first
//! scattering, without the opposition surge (gas and droplets have no shadows to hide). Each
//! channel's single-scattering albedo is found from the map's normal albedo, as for the moon;
//! the colours are those normal albedos. So the disc darkens towards its limb, a little, as
//! Jupiter's does, and its brightness is physical: each map is scaled so that the disc's
//! geometric albedo is the planet's (`"albedo"`), for the presets
//! Jupiter's 0.54, Saturn's 0.50, Uranus' 0.49 and Neptune's 0.44 in V (Mallama et al.,
//! "Comprehensive wide-band magnitudes and albedos for the planets", Icarus 282, 2017), the
//! tests checking the rendered disc against them.
//!
//! The ice giants' blue is methane's absorption of red light over the long paths multiple
//! scattering takes, which is just what a lower single-scattering albedo in the red does here.
//!
//! **Shadows.** The rings shadow the planet by their transmission, exp(-τ/μ₀) at the radius
//! where the sunlight crosses them; the planet shadows the rings, its penumbra from the sun's
//! disc; and moons and anything else in the scene shadow the planet (Io's black dot crossing
//! Jupiter), traced towards a point drawn from the sun's disc, so their penumbrae are right
//! too. Light from the planet onto the rings' night side, and from the rings onto the planet,
//! is left out.

use crate::celestial::bands::{Map, Weather};
use crate::celestial::hapke::{AlbedoTable, Hapke};
use crate::celestial::rings::Rings;
use crate::celestial::shape::{Ringed, Spheroid, RING_PART};
use crate::color::Color;
use crate::intersection::Intersection;
use crate::light::Sun;
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::material::texture::Solid;
use crate::na::{Matrix3, Vector3};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scenefile::SceneFile;
use crate::sceneobject::SceneObject;
use serde_json::Value;

/// A deep cloud deck: no opposition surge, its particles scattering a little more forwards
/// than back (Tomasko et al., "Photometry and polarimetry of Jupiter at large phase angles",
/// Icarus 33, 1978).
pub const CLOUD_DECK: Hapke = Hapke { b: 0.45, c: -0.2, b0: 0., h: 1. };

pub struct Giant {
    pub shape: Ringed,
    pub clouds: Map<Vector3<f64>>,
    pub photometry: Hapke,
    albedos: AlbedoTable,
    pub rings: Option<std::sync::Arc<Rings>>,
    /// The angle a pixel subtends, radians, for filtering the rings.
    pub pixel_angle: f64,
    /// Whether other objects shadow it.
    pub shadows: bool,
}

impl std::fmt::Debug for Giant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Giant").field("shape", &self.shape).finish()
    }
}

/// A giant planet's dimensions and look.
#[derive(Debug, Clone)]
pub struct Preset {
    /// Equatorial and polar radii, metres.
    pub radius: f64,
    pub polar_radius: f64,
    /// Its geometric albedo in V, for its light on its moons.
    pub albedo: f64,
    pub weather: Weather,
    pub rings: Option<Rings>,
}

impl Preset {
    /// A named planet, or one generated from `seed`.
    pub fn named(name: &str, seed: u64) -> Preset {
        match name {
            // IAU WGCCRE 2015 radii (1 bar level).
            "jupiter" => Preset { radius: 71_492e3, polar_radius: 66_854e3, albedo: 0.538, weather: Weather::jupiter(seed), rings: None },
            "saturn" => Preset { radius: 60_268e3, polar_radius: 54_364e3, albedo: 0.499, weather: Weather::saturn(seed), rings: Some(Rings::saturn(seed)) },
            "uranus" => Preset { radius: 25_559e3, polar_radius: 24_973e3, albedo: 0.488, weather: Weather::uranus(seed), rings: Some(Rings::uranus(seed)) },
            "neptune" => Preset { radius: 24_764e3, polar_radius: 24_341e3, albedo: 0.442, weather: Weather::neptune(seed), rings: Some(Rings::neptune(seed)) },
            "gas_giant" | "ice_giant" => Preset::generated(seed, name == "ice_giant"),
            _ => panic!("Unknown giant planet {:?}: jupiter, saturn, uranus, neptune, gas_giant or ice_giant", name),
        }
    }

    /// A giant from `seed`: its size (gas giants are all about Jupiter's, being held up by
    /// degeneracy pressure, ice giants about Neptune's), flattening by spin, weather, and
    /// rings, which about half have.
    pub fn generated(seed: u64, ice: bool) -> Preset {
        let r = |k: u64| crate::procedural::space::mix(crate::procedural::space::mix(seed ^ 0x6769_616e) ^ k) as f64 / u64::MAX as f64;
        let radius = if ice { 22_000e3 + 8_000e3 * r(1) } else { 55_000e3 + 25_000e3 * r(1) };
        let flattening = if ice { 0.01 + 0.02 * r(2) } else { 0.03 + 0.08 * r(2) };
        let rings = (r(3) < 0.5).then(|| Rings::generated(seed, radius));
        Preset { radius, polar_radius: radius * (1. - flattening), albedo: if ice { 0.45 } else { 0.5 },
            weather: Weather::generated(seed, ice), rings }
    }
}

impl Giant {
    /// The geometric albedo of the cloud tops' map, in luminance: the disc's brightness at
    /// zero phase against a white Lambertian disc's, averaged over the planet's longitudes.
    pub fn geometric_albedo(&self) -> f64 {
        let n = 64;
        let mut sum = 0.;
        for k in 0..8 {
            let turn = crate::na::Rotation3::from_axis_angle(&Vector3::y_axis(), k as f64 * std::f64::consts::PI / 4.);
            for i in 0..n {
                for j in 0..n {
                    let (x, y) = (-1. + 2. * (i as f64 + 0.5) / n as f64, -1. + 2. * (j as f64 + 0.5) / n as f64);
                    let rho2 = x * x + y * y;
                    if rho2 >= 1. {
                        continue;
                    }
                    let z = (1. - rho2).sqrt();
                    let a = self.albedo(&(turn * Vector3::new(x, y, z))).rgb;
                    let r = a.map(|a| self.photometry.reflectance(self.albedos.w(a), z, z, 0.));
                    sum += crate::color::luminance(&Color { rgb: r });
                }
            }
        }
        sum * (2. / n as f64).powi(2) / 8.
    }

    /// Scales the map's albedos so the planet's geometric albedo is `p`.
    pub fn calibrate(&mut self, p: f64) {
        let max = self.photometry.normal_albedo(1. - 1e-9);
        for _ in 0..4 {
            let k = p / self.geometric_albedo().max(1e-9);
            for c in self.clouds.data.iter_mut() {
                *c = (*c * k).map(|v| v.min(max));
            }
        }
    }

    /// The cloud tops' normal albedo at the body-frame unit direction `local`.
    pub fn albedo(&self, local: &Vector3<f64>) -> Color {
        Color { rgb: self.clouds.at(local) }
    }

    /// The share of the sunlight (from unit `l`) reaching `p` on the planet that the rings let
    /// through, per channel.
    fn ring_shadow(&self, p: &Vector3<f64>, l: &Vector3<f64>, sun_radius: f64) -> f64 {
        let Some(rings) = &self.rings else { return 1. };
        let Some((t, r)) = self.shape.ring_hit(p, l) else { return 1. };
        let mu0 = l.dot(&self.shape.body.frame.column(1)).abs().max(1e-4);
        // Averaged over the sun's image on the rings.
        let tau = rings.tau(r, 2. * t * sun_radius / mu0);
        (-tau / mu0).exp()
    }

    /// How much of the light from the sun along `l` the rest of the scene lets reach `p`: 0
    /// behind a moon, exp(-τ/μ) behind another planet's rings.
    fn shadow_of_others(&self, me: &SceneObject, p: &Vector3<f64>, l: &Vector3<f64>, s: &Scene) -> f64 {
        let mut ro = *p;
        let mut through = 1.;
        for _ in 0..8 {
            let ray = Ray { ro, rd: *l };
            let Some(hit) = s.objects.nearest_intersection(&ray, f64::INFINITY, 1.) else { break };
            if !std::ptr::eq(hit.object, me) && !hit.object.medium.material_at(hit.point).is_sky() {
                let eps = 1. + hit.dist * 1e-9;
                through *= hit.object.geometry.transmittance(&ray, hit.dist - eps, hit.dist + eps);
                if through <= 0. {
                    return 0.;
                }
            }
            ro = hit.point + l * (1. + hit.dist * 1e-9);
        }
        through
    }

    /// The radiance of the cloud tops at `p` towards unit `view`.
    pub fn planet_radiance(&self, p: &Vector3<f64>, view: &Vector3<f64>, sun: &Sun, me: Option<(&SceneObject, &Scene)>) -> Color {
        let body = &self.shape.body;
        let n = body.normal(p);
        let mu = n.dot(view);
        if mu <= 0. {
            return Color::black();
        }
        // A direction within the sun's disc: the penumbrae of what shadows the planet come out
        // of the samples.
        let l = match me {
            Some(_) => sun.sample_disc([crate::geometry::rand(), crate::geometry::rand()]),
            None => sun.direction,
        };
        let mu0 = n.dot(&l);
        if mu0 <= 0. {
            return Color::black();
        }
        let mut light = self.ring_shadow(p, &l, sun.angular_radius);
        if let (Some((obj, s)), true) = (me, self.shadows) {
            if light > 0. {
                light *= self.shadow_of_others(obj, &(p + n * (1. + 1e-9 * p.norm())), &l, s);
            }
        }
        if light <= 0. {
            return Color::black();
        }
        let a = self.albedo(&body.local(p)).as_reflectance().rgb;
        let g = l.dot(view).clamp(-1., 1.).acos();
        let r = a.map(|a| self.photometry.reflectance(self.albedos.w(a), mu0, mu, g));
        sun.irradiance.as_illuminant() * Color { rgb: r } * light
    }

    /// The rings' radiance at `p` (on the ring plane, radius `r`) towards unit `view`, with
    /// their transmission along the view; for a footprint `width` metres across.
    pub fn ring_radiance(&self, p: &Vector3<f64>, view: &Vector3<f64>, sun: &Sun, width: f64) -> (Color, f64) {
        let rings = self.rings.as_ref().unwrap();
        let body = &self.shape.body;
        let pole = body.frame.column(1).into_owned();
        let r = (p - body.centre).norm();
        let (vn, ln) = (view.dot(&pole), sun.direction.dot(&pole));
        let mu = vn.abs().max(1e-6);
        let tau = rings.tau(r, width / mu.max(0.02));
        let through = (-tau / mu).exp();
        if tau <= 0. {
            return (Color::black(), 1.);
        }
        let lit = body.uncovered(p, &sun.direction, sun.angular_radius);
        if lit <= 0. || ln.abs() < 1e-9 {
            return (Color::black(), through);
        }
        let g = sun.direction.dot(view).clamp(-1., 1.).acos();
        let i_f = rings.i_over_f(tau, ln.abs(), mu, g, vn * ln > 0.);
        (sun.irradiance.as_illuminant() * i_f * (lit / std::f64::consts::PI), through)
    }
}

impl MaterialModel for Giant {
    fn scatter(&self, r: &Ray, x: &Intersection, s: &Scene) -> ScatteredRay {
        let d = r.rd.normalize();
        let Some(sun) = s.sun.as_ref() else {
            return ScatteredRay { ray: None, attenuate: Color::black(), emit: Color::black(), light_sampling: LightSampling::None };
        };
        if x.part == RING_PART {
            let width = self.pixel_angle * x.dist;
            let (colour, through) = self.ring_radiance(&x.point, &-d, sun, width);
            // On through the ring plane: from the plane itself (the hit was moved off it,
            // towards the viewer), a little past it.
            let body = &self.shape.body;
            let pole = body.frame.column(1).into_owned();
            let on = x.point - pole * (x.point - body.centre).dot(&pole);
            let past = (1. + x.dist * 1e-9) / d.dot(&pole).abs().max(1e-6);
            let ray = (through > 1e-4).then(|| Ray { ro: on + d * past, rd: d });
            return ScatteredRay { attenuate: Color::white() * through, emit: colour, ray, light_sampling: LightSampling::None };
        }
        let colour = self.planet_radiance(&x.point, &-d, sun, Some((x.object, s)));
        ScatteredRay { ray: None, attenuate: colour, emit: Color::black(), light_sampling: LightSampling::None }
    }
}

/// The body's axes: y its `pole`, turned by `rotation` (radians) about it.
fn frame(pole: &Vector3<f64>, rotation: f64) -> Matrix3<f64> {
    let y = pole.normalize();
    let reference = if y.z.abs() < 0.9 { Vector3::z() } else { Vector3::x() };
    let x = (reference - y * reference.dot(&y)).cross(&y).normalize();
    let z = x.cross(&y);
    let base = Matrix3::from_columns(&[x, y, z]);
    base * crate::na::Rotation3::from_axis_angle(&Vector3::y_axis(), rotation).matrix()
}

impl Giant {
    /// The giant a `gas_giant` object describes (see `docs/planets.md`).
    pub fn parse(o: &Value) -> Giant {
        let seed = SceneFile::parse_number(&o["seed"], 0.) as u64;
        let name = o["preset"].as_str().unwrap_or("gas_giant");
        let mut preset = Preset::named(name, seed);
        let planet_centre = SceneFile::parse_vec3_def(&o["planet"], "centre", Vector3::new(0., -crate::planet::EARTH_RADIUS, 0.));
        let (centre, _) = crate::celestial::place(o, &planet_centre, 7.8e11);
        let radius = SceneFile::parse_number(&o["radius"], preset.radius);
        let polar = SceneFile::parse_number(&o["polar_radius"], preset.polar_radius * radius / preset.radius);
        let pole = SceneFile::parse_vec3_def(o, "pole", Vector3::y());
        let frame = frame(&pole, SceneFile::parse_number(&o["rotation"], 0.).to_radians());
        let w = &mut preset.weather;
        w.resolution = SceneFile::parse_number(&o["resolution"], w.resolution as f64) as usize;
        w.time = SceneFile::parse_number(&o["time"], w.time);
        w.turbulence = SceneFile::parse_number(&o["turbulence"], w.turbulence);
        let rings = match &o["rings"] {
            Value::Bool(false) => None,
            Value::Object(r) => {
                let seed = r.get("seed").and_then(|v| v.as_u64()).unwrap_or(seed);
                Some(match r.get("preset").and_then(|v| v.as_str()) {
                    Some("saturn") => Rings::saturn(seed),
                    Some("uranus") => Rings::uranus(seed),
                    Some("neptune") => Rings::neptune(seed),
                    _ => Rings::generated(seed, radius),
                })
            },
            Value::Bool(true) => preset.rings.take().or_else(|| Some(Rings::generated(seed, radius))),
            _ => preset.rings.take(),
        };
        // Rings scale with the planet.
        let rings = rings.map(|mut r| {
            let k = radius / preset.radius;
            r.inner *= k;
            r.outer *= k;
            std::sync::Arc::new(r)
        });
        let camera = &o["camera"];
        let pixel_angle = match camera["angle"].as_f64() {
            Some(angle) => angle.tan() / SceneFile::parse_int(&camera["image_height"], 480) as f64,
            None => 1e-4,
        };
        let shape = Ringed {
            body: Spheroid { centre, frame, a: radius, c: polar },
            rings: rings.as_ref().map(|r| (r.inner, r.outer)),
            profile: rings.clone(),
        };
        let photometry = CLOUD_DECK;
        let mut g = Giant {
            shape, clouds: preset.weather.generate(), albedos: photometry.albedo_table(), photometry, rings, pixel_angle,
            shadows: o["shadows"].as_bool().unwrap_or(true),
        };
        // The disc as bright as the planet is.
        g.calibrate(SceneFile::parse_number(&o["albedo"], preset.albedo));
        g
    }
}

/// A `gas_giant` object: the giant's shape, of its material.
pub fn parse_giant(o: &Value) -> SceneObject {
    let g = Giant::parse(o);
    SceneObject { geometry: Box::new(g.shape.clone()), medium: Box::new(Solid { m: Box::new(g) }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The giant's irradiance at `eye` against the sun's, summed over its disc (no rings).
    fn irradiance(g: &Giant, eye: &Vector3<f64>, sun: &Sun) -> f64 {
        let to = g.shape.body.centre - eye;
        let dist = to.norm();
        let axis = to / dist;
        let (u, v) = crate::procedural::space::tangent_basis(&axis);
        let angular = (g.shape.body.a / dist).asin();
        let n = 200;
        let mut sum = 0.;
        for i in 0..n {
            for j in 0..n {
                let (x, y) = (-1. + 2. * (i as f64 + 0.5) / n as f64, -1. + 2. * (j as f64 + 0.5) / n as f64);
                let d = (axis + (u * x + v * y) * angular.tan()).normalize();
                if let Some((t, _)) = g.shape.body.hit(eye, &d) {
                    sum += crate::color::luminance(&g.planet_radiance(&(eye + d * t), &-d, sun, None));
                }
            }
        }
        sum * (2. * angular.tan() / n as f64).powi(2) / crate::color::luminance(&sun.irradiance)
    }

    fn giant(preset: &str) -> Giant {
        let mut g = Giant::parse(&serde_json::json!({ "type": "gas_giant", "preset": preset, "seed": 1, "resolution": 256,
            "position": [0, 0, 1e9], "rings": false }));
        g.shape.rings = None;
        g
    }

    #[test]
    fn test_geometric_albedos() {
        // Seen at nearly zero phase, against a white Lambertian disc of the same size.
        for (name, p) in [("jupiter", 0.538), ("saturn", 0.499), ("uranus", 0.488), ("neptune", 0.442)] {
            let g = giant(name);
            let sun = Sun::new(-Vector3::new(0.02, 0., 1.).normalize(), Color::white());
            let e = irradiance(&g, &Vector3::zeros(), &sun);
            let lambert = (g.shape.body.a / g.shape.body.centre.norm()).powi(2) * (g.shape.body.c / g.shape.body.a);
            let albedo = e / lambert;
            assert!((albedo / p - 1.).abs() < 0.12, "{}: {} against {}", name, albedo, p);
        }
    }

    #[test]
    fn test_ring_shadow_on_saturn() {
        // With the sun above the ring plane, the rings' shadow falls on the hemisphere below
        // it: a point whose sunward ray crosses the B ring is in deep shadow, one whose ray
        // crosses the thin C ring hardly shaded.
        let g = Giant::parse(&serde_json::json!({ "type": "gas_giant", "preset": "saturn", "seed": 1, "resolution": 64,
            "position": [0, 0, 0] }));
        let l = Vector3::new(0., 0.45, -1.).normalize();
        // A point on the planet below the ring plane (sun from above) whose ray to the sun
        // crosses the plane at radius r: solve along -l from the plane.
        let through = |r_km: f64| {
            let on_plane = Vector3::new(0., 0., -r_km * 1e3);
            let ray = -l;
            let t = g.shape.body.hit(&on_plane, &ray).unwrap().0;
            g.ring_shadow(&(on_plane + ray * t), &l, 1e-4)
        };
        assert!(through(107_000.) < 0.01, "B {}", through(107_000.));
        assert!(through(80_000.) > 0.5, "C {}", through(80_000.));
    }

    #[test]
    fn test_planet_shadows_rings() {
        let g = Giant::parse(&serde_json::json!({ "type": "gas_giant", "preset": "saturn", "seed": 1, "resolution": 64,
            "position": [0, 0, 0] }));
        let sun = Sun::new(Vector3::new(0.3, 0.2, -1.).normalize(), Color::white());
        let view = Vector3::new(0., 0.3, -1.).normalize();
        // Behind the planet from the sun, on the B ring, and beside it.
        let behind = (-sun.direction - Vector3::y() * (-sun.direction).y).normalize() * 107_000e3;
        let beside = sun.direction.cross(&Vector3::y()).normalize() * 107_000e3;
        let (dark, _) = g.ring_radiance(&behind, &view, &sun, 100e3);
        let (lit, _) = g.ring_radiance(&beside, &view, &sun, 100e3);
        assert_eq!(crate::color::luminance(&dark), 0.);
        assert!(crate::color::luminance(&lit) > 0.);
    }
}
