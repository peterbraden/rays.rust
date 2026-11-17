//! A moon: a cratered sphere lit by the sun and its planet (see the module docs of
//! `celestial`).
//!
//! Its material works out the light it reflects directly, rather than tracing further paths:
//! a moon is lit by the sun (sampled exactly, as the sun is a delta-like source to it), and by
//! its planet's earthshine, far fainter and from one known direction; nothing else in a scene
//! comes near either. For each hit:
//!
//! 1. the surface there (`surface`), giving the normal from its slope, and its albedo;
//! 2. the sunlight reaching it: the share of the sun's disc the planet leaves uncovered (a
//!    lunar eclipse, with its penumbra), and the shadows of the relief, by marching towards the
//!    sun over the height field near the terminator (where the sun is low and shadows long);
//! 3. the light of the planet's sunlit face, by its phase seen from the moon;
//! 4. both reflected by Hapke's model (`hapke`), each channel's single-scattering albedo found
//!    from the normal albedo there.

use crate::celestial::hapke::{AlbedoTable, Hapke};
use crate::celestial::surface::{Surface, SurfaceParams};
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::material::texture::Solid;
use crate::na::{Matrix3, Vector3};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scenefile::SceneFile;
use crate::sceneobject::SceneObject;
use crate::shapes::sphere::Sphere;
use serde_json::Value;

/// The planet a moon orbits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primary {
    pub centre: Vector3<f64>,
    pub radius: f64,
    /// Its geometric albedo, for earthshine; 0 for none.
    pub albedo: f64,
    /// The colour of its light, relative (it is scaled to luminance 1).
    pub colour: Color,
}

/// The Earth's geometric albedo in V, 0.367 (Allen's Astrophysical Quantities, 4th ed.,
/// section 12.3), from its clouds, oceans and air; its light is bluish.
pub const EARTH_ALBEDO: f64 = 0.367;

/// The phase function of a Lambertian sphere: its brightness at phase angle `a` against at
/// full phase (Russell, "On the albedo of the planets and their satellites", ApJ 43, 1916).
pub fn lambert_phase(a: f64) -> f64 {
    let a = a.clamp(0., std::f64::consts::PI);
    (a.sin() + (std::f64::consts::PI - a) * a.cos()) / std::f64::consts::PI
}

pub struct Moon {
    pub centre: Vector3<f64>,
    /// Metres.
    pub radius: f64,
    /// The body's axes in the scene (columns), z towards its planet.
    pub frame: Matrix3<f64>,
    pub surface: Surface,
    pub photometry: Hapke,
    albedos: AlbedoTable,
    /// Normal albedos of the highlands and of the maria.
    pub highlands: Color,
    pub maria: Color,
    pub primary: Primary,
    /// Whether the relief casts shadows.
    pub shadows: bool,
    /// The size of a pixel at the moon, km: craters smaller than this aren't drawn.
    pub footprint: f64,
    /// What else colours it (`terrain`): Mars' dark regions, Europa's lines, Io's volcanoes.
    pub terrain: crate::celestial::terrain::Terrain,
}

impl std::fmt::Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Moon").field("centre", &self.centre).field("radius", &self.radius)
            .field("footprint", &self.footprint).finish()
    }
}

/// The highest relief, km, above which nothing can shade.
const MAX_RELIEF: f64 = 8.;
/// Steps of the march towards the sun.
const SHADOW_STEPS: usize = 20;

impl Moon {
    /// The normal albedo of the surface sample `s`, per channel.
    fn albedo(&self, local: &Vector3<f64>, s: &crate::celestial::surface::SurfaceSample) -> Color {
        // In the terrain's frame the pole is y, as in the body's.
        let base = self.highlands * (1. - s.mare) + self.maria * s.mare;
        self.terrain.albedo(local, base, s.mare) * s.brightness
    }

    /// The unobstructed share of the sun's disc (angular radius `sun_radius`, in unit direction
    /// `sun`) seen from `point` (in the body's frame `local`, a unit vector, at height `h` km),
    /// past the relief: 0 in a crater's shadow, 1 in sunlight, between in the penumbra.
    fn relief_shadow(&self, local: &Vector3<f64>, h: f64, sun: &Vector3<f64>, sun_radius: f64) -> f64 {
        let r = self.surface.params.radius;
        let sin_el = sun.dot(local);
        // High sun: the steepest walls are about 35°.
        if sin_el > 0.6 {
            return 1.;
        }
        let tangent = sun - local * sin_el;
        if tangent.norm() < 1e-9 {
            return 1.;
        }
        let tangent = tangent.normalize();
        let tan_el = sin_el / (1. - sin_el * sin_el).max(1e-12).sqrt();
        // How far the ray could still pass under the highest relief.
        let reach = {
            // h + s tan + s² / 2r = MAX_RELIEF
            let (a, b, c) = (0.5 / r, tan_el, h - MAX_RELIEF);
            ((-b + (b * b - 4. * a * c).max(0.).sqrt()) / (2. * a)).clamp(0., 600.)
        };
        let start = (self.footprint * 0.5).max(0.05);
        if reach <= start {
            return 1.;
        }
        let ratio = (reach / start).powf(1. / (SHADOW_STEPS - 1) as f64);
        let mut s = start;
        // The lowest the sun's centre sits above the horizon, radians.
        let mut clearance = f64::INFINITY;
        for _ in 0..SHADOW_STEPS {
            let q = (local + tangent * (s / r)).normalize();
            let ray = h + s * tan_el + s * s / (2. * r);
            let ground = self.surface.sample(&q, self.footprint.max(s / 8.), false).height;
            clearance = clearance.min((ray - ground) / s);
            if clearance < -sun_radius {
                return 0.;
            }
            s *= ratio;
        }
        crate::procedural::space::smoothstep(-sun_radius, sun_radius, clearance)
    }

    /// The radiance leaving the point of the moon in unit direction `local` (body frame)
    /// towards `view` (unit, scene), lit by `sun`.
    pub fn radiance(&self, local: &Vector3<f64>, view: &Vector3<f64>, sun: Option<&crate::light::Sun>) -> Color {
        // Towards the limb a pixel covers more of the surface, along the line of sight.
        let foreshortened = (self.frame * local).dot(view).max(0.1);
        let s = self.surface.sample(local, self.footprint / foreshortened, true);
        let n_local = (local - s.slope).normalize();
        let n = self.frame * n_local;
        let point = self.centre + self.frame * local * self.radius;
        let mu = n.dot(view).max(1e-3);
        // Each channel's single-scattering albedo, at the path's wavelengths in spectral mode.
        let a = self.albedo(local, &s).as_reflectance().rgb;
        let w = a.map(|c| self.albedos.w(c));
        let reflect = |e: Color, light: &Vector3<f64>| -> Color {
            let mu0 = n.dot(light);
            if mu0 <= 0. {
                return Color::black();
            }
            let g = light.dot(view).clamp(-1., 1.).acos();
            e * Color { rgb: w.map(|w| self.photometry.reflectance(w, mu0, mu, g)) }
        };
        let mut out = Color::black();
        let to_planet = self.primary.centre - point;
        let planet_distance = to_planet.norm();
        let to_planet = to_planet / planet_distance;
        let planet_radius = (self.primary.radius / planet_distance).min(1.).asin();
        if let Some(sun) = sun {
            let l = sun.direction;
            // The planet's shadow: a lunar eclipse, with its penumbra.
            let lit = crate::celestial::uncovered(sun.angular_radius, planet_radius, l.dot(&to_planet).clamp(-1., 1.).acos());
            if lit > 0. && n.dot(&l) > 0. {
                let relief = if self.shadows {
                    self.relief_shadow(local, s.height, &(self.frame.transpose() * l), sun.angular_radius)
                } else {
                    1.
                };
                if relief > 0. {
                    out = out + reflect(sun.irradiance.as_illuminant() * (lit * relief), &l);
                }
            }
            // Earthshine: the planet's sunlit face, a sphere of its albedo at its phase seen
            // from here (the angle at the planet between the sun and the moon).
            if self.primary.albedo > 0. {
                let phase = l.dot(&-to_planet).clamp(-1., 1.).acos();
                let e = self.primary.albedo * lambert_phase(phase) * (self.primary.radius / planet_distance).powi(2);
                if e > 0. {
                    out = out + reflect(sun.irradiance.as_illuminant() * self.primary.colour.as_reflectance() * e, &to_planet);
                }
            }
        }
        out
    }
}

impl Moon {
    /// The irradiance the moon's whole disc gives at `eye` (facing it), lit by `sun`: its
    /// radiance summed over the disc, on a grid of its projection.
    pub fn irradiance(&self, eye: &Vector3<f64>, sun: &crate::light::Sun) -> Color {
        let to = self.centre - eye;
        let dist = to.norm();
        let axis = to / dist;
        let (u, v) = crate::procedural::space::tangent_basis(&axis);
        let spread = (self.radius / dist).asin().tan();
        let n = 64;
        let mut sum = Color::black();
        let o = eye - self.centre;
        for i in 0..n {
            for j in 0..n {
                let (x, y) = (-1. + 2. * (i as f64 + 0.5) / n as f64, -1. + 2. * (j as f64 + 0.5) / n as f64);
                if x * x + y * y >= 1. {
                    continue;
                }
                let d = (axis + (u * x + v * y) * spread).normalize();
                let b = o.dot(&d);
                let disc = b * b - (o.norm_squared() - self.radius * self.radius);
                if disc < 0. {
                    continue;
                }
                let p = eye + d * (-b - disc.sqrt());
                let local = self.frame.transpose() * (p - self.centre).normalize();
                sum = sum + self.radiance(&local, &-d, Some(sun));
            }
        }
        // Each sample's solid angle, nearly: the grid's spacing on the view plane, squared.
        sum * (2. * spread / n as f64).powi(2)
    }
}

/// The sunlight a moon reflects onto the scene, as a light: a disc in the moon's direction,
/// with the irradiance its whole sunlit (and earthshine-lit) face gives, sampled like the sun
/// by diffuse surfaces and media, and crossing the air as the sun's light does. What a
/// surface sees of the disc itself (a mirror, or a ray that happens to hit it) is the moon's
/// own radiance, weighted against this sampling (`trace`).
#[derive(Debug, Clone)]
pub struct Moonlight {
    pub light: crate::light::Sun,
    pub centre: Vector3<f64>,
    pub radius: f64,
}

impl Moonlight {
    /// The light of the `moon` object `o` (with its planet and camera filled in, as for
    /// `parse_moon`), seen from `eye`, lit by `sun`; through the air of the skysphere `sky`,
    /// if any.
    pub fn new(o: &Value, eye: &Vector3<f64>, sun: &crate::light::Sun, sky: Option<&Value>) -> Moonlight {
        let moon = Moon::parse(o);
        // Above any air.
        let bare = crate::light::Sun { atmosphere: None, ..sun.clone() };
        let irradiance = moon.irradiance(eye, &bare);
        let to = moon.centre - eye;
        let dist = to.norm();
        let direction = to / dist;
        let atmosphere = sky.map(|sky| {
            let mut sky = sky.clone();
            sky["sun_direction"] = serde_json::json!([direction.x, direction.y, direction.z]);
            std::sync::Arc::new(crate::skysphere::parse_sky_material(&sky))
        });
        let light = crate::light::Sun {
            atmosphere,
            shadow_distance: (dist - moon.radius) * 0.999,
            ..crate::light::Sun::new(direction, irradiance).with_angular_radius((moon.radius / dist).asin())
        };
        Moonlight { light, centre: moon.centre, radius: moon.radius }
    }

    /// Whether `point` is on the moon.
    pub fn is_on(&self, point: &Vector3<f64>) -> bool {
        ((point - self.centre).norm() - self.radius).abs() < self.radius * 1e-3
    }
}

impl MaterialModel for Moon {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let d = r.rd.normalize();
        // Behind the planet (when the scene has no ground of its own to hide it).
        let o = r.ro - self.primary.centre;
        let (b, c) = (o.dot(&d), o.norm_squared() - self.primary.radius * self.primary.radius);
        let hidden = c > 0. && b < 0. && b * b - c > 0. && -b - (b * b - c).sqrt() < intersection.dist;
        let colour = if hidden {
            Color::black()
        } else {
            let local = self.frame.transpose() * (intersection.point - self.centre).normalize();
            self.radiance(&local, &-d, s.sun.as_ref())
        };
        ScatteredRay { ray: None, attenuate: colour, emit: Color::black(), light_sampling: LightSampling::None }
    }
}

/// The scene's planet, for a moon: the skysphere's earth if there is one, else the `planet`
/// object's sphere, else the Earth's, below the origin.
pub fn scene_primary(objects: &[Value]) -> (Vector3<f64>, f64) {
    if let Some(sky) = objects.iter().find(|o| o["type"].as_str() == Some("skysphere")) {
        let sky = crate::skysphere::parse_sky_material(sky);
        return (sky.earth_centre(), sky.earth_radius());
    }
    let r = objects.iter().find(|o| o["type"].as_str() == Some("planet"))
        .map_or(crate::planet::EARTH_RADIUS, |o| SceneFile::parse_number(&o["radius"], crate::planet::EARTH_RADIUS));
    (Vector3::new(0., -r, 0.), r)
}

/// A `moon` object (see `docs/moon.md`): a sphere of the moon's material.
pub fn parse_moon(o: &Value) -> SceneObject {
    let moon = Moon::parse(o);
    SceneObject { geometry: Box::new(Sphere::new(moon.centre, moon.radius)), medium: Box::new(Solid { m: Box::new(moon) }) }
}

impl Moon {
/// The moon a `moon` object describes. Its `planet` (centre and radius) and `camera` (with
/// `image_height`), for the pixel footprint, are filled in from the scene if absent.
pub fn parse(o: &Value) -> Moon {
    let planet_centre = SceneFile::parse_vec3_def(&o["planet"], "centre", Vector3::new(0., -crate::planet::EARTH_RADIUS, 0.));
    let planet_radius = SceneFile::parse_number(&o["planet"]["radius"], crate::planet::EARTH_RADIUS);
    let seed = SceneFile::parse_number(&o["seed"], 0.) as u64;
    let world = crate::celestial::terrain::World::named(o["preset"].as_str().unwrap_or("moon"), seed);
    let radius = SceneFile::parse_number(&o["radius"], world.radius);
    let (centre, up) = crate::celestial::place(o, &planet_centre, crate::celestial::MOON_DISTANCE);
    let libration = o.get("libration").map_or((0., 0.), |l| {
        let v = l.as_array().expect("libration is [longitude, latitude] in degrees");
        (v[0].as_f64().unwrap_or(0.).to_radians(), v[1].as_f64().unwrap_or(0.).to_radians())
    });
    let frame = crate::celestial::locked_frame(&centre, &planet_centre, &up, libration);

    let defaults = world.params.clone();
    let c = &o["craters"];
    let params = SurfaceParams {
        seed: SceneFile::parse_number(&o["seed"], 0.) as u64,
        radius: radius / 1000.,
        crater_density: SceneFile::parse_number(&c["density"], defaults.crater_density),
        max_crater: SceneFile::parse_number(&c["max"], defaults.max_crater),
        min_crater: SceneFile::parse_number(&c["min"], defaults.min_crater),
        basins: SceneFile::parse_number(&c["basins"], defaults.basins as f64) as usize,
        rayed_craters: SceneFile::parse_number(&c["rayed"], defaults.rayed_craters as f64) as usize,
        ray_brightness: SceneFile::parse_number(&c["ray_brightness"], defaults.ray_brightness),
        fresh_brightness: SceneFile::parse_number(&c["fresh_brightness"], defaults.fresh_brightness),
        maria: SceneFile::parse_number(&o["maria"], defaults.maria),
        mare_crater_survival: SceneFile::parse_number(&c["mare_survival"], defaults.mare_crater_survival),
        albedo_variation: SceneFile::parse_number(&o["albedo_variation"], defaults.albedo_variation),
    };
    let ph = &o["photometry"];
    let photometry = Hapke {
        b: SceneFile::parse_number(&ph["b"], Hapke::MOON.b),
        c: SceneFile::parse_number(&ph["c"], Hapke::MOON.c),
        b0: SceneFile::parse_number(&ph["b0"], Hapke::MOON.b0),
        h: SceneFile::parse_number(&ph["h"], Hapke::MOON.h),
    };
    let earthshine = &o["earthshine"];
    let primary = Primary {
        centre: planet_centre, radius: planet_radius,
        albedo: if earthshine.as_bool() == Some(false) { 0. } else { SceneFile::parse_number(&earthshine["albedo"], EARTH_ALBEDO) },
        colour: {
            let c = SceneFile::parse_color_def(earthshine, "color", Color::new(0.85, 0.95, 1.15));
            c * (1. / crate::color::luminance(&c))
        },
    };
    // The pixel's size at the moon.
    let camera = &o["camera"];
    let footprint = match (camera["angle"].as_f64(), camera.get("location")) {
        (Some(angle), Some(loc)) => {
            let h = SceneFile::parse_int(&camera["image_height"], 480) as f64;
            (centre - SceneFile::parse_vec3(loc)).norm() * angle.tan() / h / 1000.
        },
        _ => SceneFile::parse_number(&o["footprint"], 2.),
    };
    let albedo = &o["albedo"];
    Moon {
        centre, radius, frame,
        surface: Surface::new(params),
        albedos: photometry.albedo_table(),
        photometry,
        highlands: SceneFile::parse_color_def(albedo, "highlands", world.highlands),
        maria: SceneFile::parse_color_def(albedo, "maria", world.maria),
        terrain: crate::celestial::terrain::Terrain::new(world.pattern, seed),
        primary,
        shadows: o["shadows"].as_bool().unwrap_or(true),
        footprint,
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::Sun;


    /// The moon's irradiance at `eye`, against the sun's.
    fn irradiance(m: &Moon, eye: &Vector3<f64>, sun: &Sun) -> f64 {
        crate::color::luminance(&m.irradiance(eye, sun)) / crate::color::luminance(&sun.irradiance)
    }

    /// A moon seen from the origin on the Earth, in `direction`, lit by a sun in `sun`.
    fn seen(direction: [f64; 3], sun: [f64; 3]) -> (Moon, Sun) {
        let m = Moon::parse(&serde_json::json!({ "type": "moon", "seed": 1, "direction": direction, "footprint": 20,
            "planet": { "centre": [0, -crate::planet::EARTH_RADIUS, 0], "radius": crate::planet::EARTH_RADIUS } }));
        (m, Sun::new(Vector3::from(sun), Color::white()))
    }

    #[test]
    fn test_full_moon_magnitude() {
        // The full moon is magnitude -12.74 against the sun's -26.74, at its mean distance: 2.5e-6
        // of the sun's irradiance (Allen, section 12.4). Here, a few degrees off full (at full
        // it would be eclipsed), and nearer by the Earth's radius.
        let (m, sun) = seen([0., 1., 0.], [0.06, -1., 0.]);
        let e = irradiance(&m, &Vector3::zeros(), &sun);
        let d = m.centre.norm();
        let expected = 10f64.powf(-0.4 * 14.) * (crate::celestial::MOON_DISTANCE / d).powi(2)
            * 10f64.powf(-0.4 * 0.026 * 0.06f64.atan().to_degrees());
        assert!((e / expected - 1.).abs() < 0.25, "{:e} vs {:e}", e, expected);
        // At first quarter it is about a tenth as bright: Allen's 2.6 magnitudes fainter.
        let (q, sun) = seen([0., 1., 0.], [1., 0., 0.]);
        let quarter = irradiance(&q, &Vector3::zeros(), &sun) / e;
        let allen = 10f64.powf(-0.4 * (0.026 * 90. + 4e-9 * 90f64.powi(4)));
        assert!((quarter / allen - 1.).abs() < 0.35, "{} vs {}", quarter, allen);
    }

    #[test]
    fn test_lunar_eclipse() {
        // With the sun straight behind the Earth, the moon is in its umbra; a little way off,
        // in its penumbra, partly lit; farther, in full sunlight.
        let lit = |offset: f64| {
            let (m, sun) = seen([0., 1., 0.], [offset, -1., 0.]);
            irradiance(&m, &Vector3::zeros(), &Sun::new(sun.direction, Color::white()))
        };
        let (umbra, penumbra, full) = (lit(0.), lit(0.0175), lit(0.05));
        // (Earthshine from the Earth's night side's sliver of daylight aside.)
        assert!(umbra < 1e-9 * full, "{} {}", umbra, full);
        assert!(penumbra > 0.05 * full && penumbra < 0.9 * full, "{} {}", penumbra, full);
    }

    #[test]
    fn test_earthshine_lights_the_night_side() {
        // A new moon (the sun behind it) is lit only by the full Earth: about 1e-4 of its
        // sunlit brightness, far brighter than nothing.
        let (m, sun) = seen([0., 1., 0.], [0.3, 1., 0.]);
        let dark = m.frame.transpose() * Vector3::new(0., -1., 0.);
        let shine = crate::color::luminance(&m.radiance(&dark, &Vector3::new(0., -1., 0.), Some(&sun)));
        assert!(shine > 0.);
        let (none, sun2) = (Moon { primary: Primary { albedo: 0., ..m.primary }, ..seen([0., 1., 0.], [0.3, 1., 0.]).0 }, sun.clone());
        assert_eq!(crate::color::luminance(&none.radiance(&dark, &Vector3::new(0., -1., 0.), Some(&sun2))), 0.);
        // Against the sunlit full moon's centre.
        let (f, fs) = seen([0., 1., 0.], [0.06, -1., 0.]);
        let full = crate::color::luminance(&f.radiance(&(f.frame.transpose() * Vector3::new(0., -1., 0.)), &Vector3::new(0., -1., 0.), Some(&fs)));
        let ratio = shine / full;
        assert!(ratio > 2e-5 && ratio < 1e-3, "{}", ratio);
    }

    #[test]
    fn test_moon_is_hidden_below_the_horizon() {
        // In a scene with a sky but no ground, the planet itself still hides it: a ray below
        // the horizon sees the same with the moon there as without.
        let scene = |moon: &str| crate::scenefile::SceneFile::from_string(format!(r#"{{ "width": 2, "height": 2, "supersamples": 1,
            "camera": {{ "location": [0, 2, 0], "lookat": [0, -0.2, 1], "up": [0, 1, 0], "angle": 0.01 }},
            "sun": {{ "intensity": 6 }},
            "objects": [ {{ "type": "skysphere", "sun_direction": [0, 1, 0] }} {} ] }}"#, moon));
        let with = scene(r#", { "type": "moon", "direction": [0, -0.2, 1], "from": [0, 2, 0] }"#);
        let r = Ray { ro: Vector3::new(0., 2., 0.), rd: Vector3::new(0., -0.2, 1.).normalize() };
        let hit = with.objects.nearest_intersection(&r, f64::INFINITY, 0.).unwrap();
        assert!(hit.dist > 1e8 && hit.dist < 1e9);
        let (a, b) = (crate::trace::trace(&r, 0, &with).1, crate::trace::trace(&r, 0, &scene("")).1);
        // (The air is integrated a little differently up to an object than through the sky.)
        assert!((crate::color::luminance(&a) / crate::color::luminance(&b) - 1.).abs() < 1e-3, "{:?} vs {:?}", a.rgb, b.rgb);
    }

    #[test]
    fn test_moonlight_lights_the_ground() {
        // A full moon overhead (the sun below the ground, a little off the line so the moon
        // isn't eclipsed) lights a white Lambertian floor with about 2.5e-6 of the sun's
        // irradiance, ~0.25 lux beside the sun's 1e5: its radiance is albedo / π times that.
        let s = crate::scenefile::SceneFile::from_string(r#"{ "width": 2, "height": 2, "supersamples": 1, "max_depth": 1,
            "camera": { "location": [0, 2, 0], "lookat": [0, 0, 1], "up": [0, 1, 0], "angle": 0.5 },
            "sun": { "intensity": 1, "direction": [0.06, -1, 0] },
            "materials": { "WHITE": { "type": "lambertian", "albedo": [1, 1, 1] } },
            "objects": [ { "type": "plane", "y": 0, "material": "WHITE" },
                         { "type": "moon", "seed": 1, "direction": [0, 1, 0], "from": [0, 2, 0] } ] }"#.to_string());
        let m = s.moonlight.as_ref().expect("the moon lights the scene");
        let e = crate::color::luminance(&m.light.irradiance);
        assert!(e > 1.8e-6 && e < 3.2e-6, "{:e}", e);
        assert!((m.light.direction - Vector3::y()).norm() < 1e-6);
        assert!(m.light.shadow_distance < m.centre.norm() - m.radius);
        // The floor, lit by it alone.
        let r = Ray { ro: Vector3::new(0., 2., 0.), rd: Vector3::new(0., -1., 1.).normalize() };
        let floor = crate::color::luminance(&crate::trace::trace(&r, 0, &s).1);
        let expected = e * m.light.direction.y / std::f64::consts::PI;
        assert!((floor / expected - 1.).abs() < 0.02, "{:e} vs {:e}", floor, expected);
        // `"light": false` turns it off.
        let dark = crate::scenefile::SceneFile::from_string(r#"{ "width": 2, "height": 2, "supersamples": 1,
            "camera": { "location": [0, 2, 0], "lookat": [0, 0, 1], "up": [0, 1, 0], "angle": 0.5 },
            "sun": { "intensity": 1, "direction": [0.06, -1, 0] },
            "objects": [ { "type": "moon", "direction": [0, 1, 0], "light": false } ] }"#.to_string());
        assert!(dark.moonlight.is_none());
    }

    #[test]
    fn test_moonlit_sky() {
        // At night, with a full moon up, the sky is daylight's blue, dimmed by the moon's share.
        let scene = |moon: bool| crate::scenefile::SceneFile::from_string(format!(r#"{{ "width": 2, "height": 2, "supersamples": 1,
            "camera": {{ "location": [0, 2, 0], "lookat": [0, 1, 1], "up": [0, 1, 0], "angle": 0.5 }},
            "sun": {{ "intensity": 6 }},
            "objects": [ {{ "type": "skysphere", "sun_direction": [0.3, -0.5, -1] }} {} ] }}"#,
            if moon { r#", { "type": "moon", "direction": [-0.3, 0.5, 1], "from": [0, 2, 0] }"# } else { "" }));
        let r = Ray { ro: Vector3::new(0., 2., 0.), rd: Vector3::new(0.5, 0.6, -0.2).normalize() };
        let (lit, dark) = (crate::trace::trace(&r, 0, &scene(true)).1, crate::trace::trace(&r, 0, &scene(false)).1);
        assert!(lit.rgb.z > lit.rgb.x, "{:?}", lit.rgb);
        assert!(crate::color::luminance(&lit) > 10. * crate::color::luminance(&dark), "{:?} {:?}", lit.rgb, dark.rgb);
    }
}
