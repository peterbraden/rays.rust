//! # Weather in a scene file
//!
//! ```json
//! { "type": "weather" }
//! ```
//!
//! is a whole sky's weather, on a random seed (printed, so a sky worth keeping can be pinned
//! with `"seed"`). Everything else is optional:
//!
//! - `seed`: the weather's seed; absent or `"random"` for a new one each render.
//! - Where and when: `site` `{"lat", "lon"}` (degrees) at the scene's origin, by default the
//!   landscape's, else 45°N 0°; `sun_direction` (by default the skysphere's), which sets the
//!   season by its declination and where it is afternoon, or else `day_of_year` (random by
//!   default); `convection_lag` (hours after noon that convection peaks, 3); `earth_radius`.
//! - The air: `humidity` and `instability` (scales, 1); `tropopause` ([equator, pole], m);
//!   `synoptic_size` and `mesoscale_size` (m) of the variations; `itcz` and `storm_track`
//!   (latitudes, degrees; by default from the climate and season); `climate`: overrides of the
//!   climate (`tilt`, `solar`, `hadley`, `warming`) when there is no landscape to take it from.
//!   With a full-scale `landscape` in the scene, its land, mountains and climate are used
//!   (`"landscape": false` to ignore it).
//! - Systems: `cyclones` and `anticyclones` per hemisphere and `tropical_cyclones` in all
//!   (random counts by default); `cyclone_size` (m); `warm_front_slope`, `cold_front_slope`;
//!   and `systems`, a list of systems to add: `{"type": "cyclone" | "anticyclone" |
//!   "tropical_cyclone", "lat", "lon"}` or with `"offset": [east, north]` (metres from the
//!   scene's origin) in place of `lat` and `lon`, and `radius` (m), `intensity` (0 to 1),
//!   `stage` (a cyclone's: 0 a wave, 0.5 mature, 1 occluded), `tilt` (degrees), `eye` (m).
//! - Each family of cloud: `low`, `mid`, `convective`, `high` and `rain`, `false` to leave it
//!   out, or an object with `amount` (scales its cover), its look (`density`, `shape_size`,
//!   `detail_size`, `erosion`) and how it scatters (`albedo`, `g` or `droplet_diameter`,
//!   `similarity_after`, `diffraction`, `halos`, `rainbow`, as for any volume). `convective`
//!   takes `cell_size` (m, between storms), `overshoot` and `anvil_depth` (m); `high` takes Hogan
//!   and Kew's cirrus parameters (`hogan_kew_cirrus`; its `coverage` comes from the weather);
//!   `rain` takes `shower_rate` (mm/h under the strongest cumulonimbus), `streakiness` and
//!   `streak_size`.
//! - Time (`time`, seconds, for `scripts/animate.py`): `cyclone_velocity`,
//!   `anticyclone_velocity`, `tropical_velocity` ([east, poleward] m/s), their `..._lifetime`s
//!   (s); the background's `drift` (m/s), `synoptic_period` and `mesoscale_period` (s); each
//!   family's `wind` and `rise` (m/s), the convective cells' `lifetime` (s) and the rain's
//!   `fall_speed` (m/s). See docs/weather.md.
//! - `map_size`: texels along each cube face of the weather's maps (512: 20 km); `map`: write
//!   a satellite's view of the weather, with its fronts and systems marked, to this PNG
//!   (`map_width` pixels wide, 2048).
//!
//! The object becomes one volume per family (`"type": "weather_layer"`), sharing one
//! weather.

use crate::na::Vector3;
use crate::planet::{GlobalMapping, Planet, EARTH_RADIUS};
use crate::scenefile::SceneFile;
use crate::sceneobject::SceneObject;
use crate::volume::create_volume;
use crate::weather::fields::{Convective, Drift, Family, Look, Rain, Stratiform};
use crate::weather::systems::Kind;
use crate::weather::{Ground, SystemSpec, Weather, WeatherParams};
use serde_json::Value;
use std::sync::Arc;

/// The families a weather object renders as.
pub const FAMILIES: [&str; 5] = ["low", "mid", "convective", "high", "rain"];

fn is_weather(o: &Value) -> bool {
    o["type"].as_str() == Some("weather")
}

/// Give each weather object without a seed (or with `"seed": "random"`) a random one, saying
/// which, so that a render can be repeated. Done to the document before anything is keyed by
/// its contents: a render with a new random sky starts afresh.
pub fn choose_seeds(doc: &mut Value) {
    if let Some(objects) = doc.get_mut("objects").and_then(|o| o.as_array_mut()) {
        for o in objects.iter_mut().filter(|o| is_weather(o)) {
            if o.get("seed").is_none_or(|s| s.as_str() == Some("random")) {
                let seed = rand::random::<u32>() as u64;
                println!("Weather: seed {} (chosen at random: add \"seed\": {} to the weather to keep it)", seed, seed);
                o["seed"] = Value::from(seed);
            }
        }
    }
}

/// Replace each weather object with its families' layers, telling it what it needs from the
/// rest of the scene: the skysphere's sun and the landscape.
pub fn expand(objects: Vec<Value>) -> Vec<Value> {
    let sun = objects.iter().find(|o| o["type"].as_str() == Some("skysphere"))
        .map(|sky| crate::skysphere::parse_sky_material(sky).sun_direction());
    let landscape = objects.iter().find(|o| o["type"].as_str() == Some("landscape") && o.get("scale").is_none()).cloned();
    let mut out = vec![];
    for o in objects {
        if !is_weather(&o) {
            out.push(o);
            continue;
        }
        let mut w = o.clone();
        if w.get("seed").is_none_or(|s| s.as_str() == Some("random")) {
            // Not through `choose_seeds` (a scene built in code): random all the same.
            let mut doc = serde_json::json!({ "objects": [w] });
            choose_seeds(&mut doc);
            w = doc["objects"][0].take();
        }
        if let (None, Some(s)) = (w.get("sun_direction"), sun) {
            w["sun_direction"] = serde_json::json!([s.x, s.y, s.z]);
        }
        if let (None, Some(l)) = (w.get("landscape"), &landscape) {
            w["landscape"] = l.clone();
        }
        for family in FAMILIES {
            if w[family] == Value::Bool(false) {
                continue;
            }
            out.push(serde_json::json!({ "type": "weather_layer", "family": family, "weather": w }));
        }
    }
    out
}

fn number(o: &Value, k: &str, d: f64) -> f64 {
    SceneFile::parse_number(&o[k], d)
}

/// The weather's parameters from its object.
pub fn weather_params(o: &Value) -> (WeatherParams, Option<Ground>) {
    let seed = SceneFile::parse_seed(o).unwrap_or(0);
    let mut p = WeatherParams::new(seed);
    // The landscape: its planet, climate and site.
    let mut ground = None;
    let l = &o["landscape"];
    if l.is_object() {
        let tp = crate::terrain::object::terrain_params(l);
        let planet = Arc::new(crate::terrain::planet::Planet::new(tp.planet.clone()));
        let climate = tp.climate.as_ref().map(|c| crate::terrain::climate::climate(c, &planet));
        if let Some(c) = &tp.climate {
            p.climate = c.clone();
        }
        p.site = crate::terrain::object::site(&l["domain"], &tp);
        p.radius = tp.planet.radius;
        ground = Some(Ground { planet, climate });
    } else {
        p.radius = number(o, "earth_radius", EARTH_RADIUS);
        let c = &o["climate"];
        for (k, f) in [("tilt", &mut p.climate.tilt), ("solar", &mut p.climate.solar), ("hadley", &mut p.climate.hadley), ("warming", &mut p.climate.warming)] {
            *f = number(c, k, *f);
        }
    }
    if let Some(site) = o.get("site") {
        p.site = (number(site, "lat", p.site.0), number(site, "lon", p.site.1));
    }
    p.sun = o.get("sun_direction").map(SceneFile::parse_vec3);
    p.day_of_year = number(o, "day_of_year", p.day_of_year);
    p.convection_lag = number(o, "convection_lag", p.convection_lag);
    p.humidity = number(o, "humidity", p.humidity).max(0.);
    p.instability = number(o, "instability", p.instability).max(0.);
    for (k, f) in [("low", &mut p.low), ("mid", &mut p.mid), ("high", &mut p.high), ("convective", &mut p.deep)] {
        *f = number(&o[k], "amount", *f).max(0.);
    }
    let count = |k: &str, d: usize| SceneFile::parse_int(&o[k], d);
    p.cyclones = count("cyclones", p.cyclones);
    p.anticyclones = count("anticyclones", p.anticyclones);
    p.tropical_cyclones = count("tropical_cyclones", p.tropical_cyclones);
    p.cyclone_size = number(o, "cyclone_size", p.cyclone_size).max(1e5);
    p.storm_track = o["storm_track"].as_f64();
    p.itcz = o["itcz"].as_f64();
    p.fronts.warm_slope = number(o, "warm_front_slope", p.fronts.warm_slope).max(1e-4);
    p.fronts.cold_slope = number(o, "cold_front_slope", p.fronts.cold_slope).max(1e-4);
    p.synoptic_size = number(o, "synoptic_size", p.synoptic_size).max(1e3);
    p.mesoscale_size = number(o, "mesoscale_size", p.mesoscale_size).max(1e3);
    if let Some(t) = o["tropopause"].as_array() {
        p.tropopause = (t[0].as_f64().unwrap_or(p.tropopause.0), t[1].as_f64().unwrap_or(p.tropopause.1));
    }
    p.map_size = SceneFile::parse_int(&o["map_size"], p.map_size).max(2);
    p.time = number(o, "time", p.time);
    for (k, v) in [("cyclone_velocity", &mut p.cyclone_velocity), ("anticyclone_velocity", &mut p.anticyclone_velocity),
                   ("tropical_velocity", &mut p.tropical_velocity)] {
        *v = SceneFile::parse_vec2_def(o, k, *v);
    }
    for (k, f) in [("cyclone_lifetime", &mut p.cyclone_lifetime), ("anticyclone_lifetime", &mut p.anticyclone_lifetime),
                   ("tropical_lifetime", &mut p.tropical_lifetime), ("synoptic_period", &mut p.synoptic_period),
                   ("mesoscale_period", &mut p.mesoscale_period)] {
        *f = number(o, k, *f).max(1.);
    }
    p.drift = number(o, "drift", p.drift);
    p.cell_size = number(&o["convective"], "cell_size", p.cell_size).max(1000.);
    if let Some(list) = o["systems"].as_array() {
        p.systems = list.iter().map(|s| {
            let kind = match s["type"].as_str() {
                Some("cyclone") | Some("low") => Kind::Cyclone,
                Some("anticyclone") | Some("high") => Kind::Anticyclone,
                Some("tropical_cyclone") | Some("hurricane") | Some("typhoon") => Kind::Tropical,
                t => panic!("Unknown weather system {:?}: expected cyclone, anticyclone or tropical_cyclone", t),
            };
            let lat_lon = match (s["lat"].as_f64(), s["lon"].as_f64()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            };
            SystemSpec {
                kind, lat_lon,
                offset: s.get("offset").map(SceneFile::parse_vec2),
                radius: s["radius"].as_f64(), intensity: s["intensity"].as_f64(), stage: s["stage"].as_f64(),
                tilt: s["tilt"].as_f64(), eye: s["eye"].as_f64(),
                velocity: s.get("velocity").map(SceneFile::parse_vec2),
            }
        }).collect();
    }
    (p, ground)
}

/// The weather an object describes, shared by all its layers.
pub fn weather_of(o: &Value) -> Arc<Weather> {
    crate::weather::weather(&o.to_string(), || {
        let (params, ground) = weather_params(o);
        let w = Weather::new(params, ground);
        let (lat, lon) = w.params.site;
        println!("Weather: seed {}, the scene's origin at {:.2}°, {:.2}°; {} cyclones, {} anticyclones, {} tropical cyclones",
            w.params.seed, lat, lon,
            w.systems.iter().filter(|s| s.kind == Kind::Cyclone).count(),
            w.systems.iter().filter(|s| s.kind == Kind::Anticyclone).count(),
            w.systems.iter().filter(|s| s.kind == Kind::Tropical).count());
        println!("Weather overhead: {}", w.overhead());
        if let Some(path) = o["map"].as_str() {
            w.write_map(path, SceneFile::parse_int(&o["map_width"], 2048).max(16));
        }
        w
    })
}

/// A family's drift with time: `wind` (m/s eastward, round the planet's axis) and `rise`
/// (m/s, how fast its shapes boil upwards).
fn drift(o: &Value, weather: &Weather, wind: f64, rise: f64) -> Drift {
    Drift::new(weather.frame.to_scene(&Vector3::y()), number(o, "wind", wind), number(o, "rise", rise), weather.params.time, weather.params.radius)
}

/// The convective family as `spec` describes it (shared by its layer and the rain under it,
/// so that the rain falls from the storms drawn).
fn convective(spec: &Value, weather: &Weather, maps: Arc<crate::weather::Maps>) -> Convective {
    let d = Look { max_density: 0.06, shape_size: 1500., detail_size: 300., erosion: 0.35 };
    let seed = weather.params.seed.wrapping_mul(31).wrapping_add(2);
    let mut field = Convective::new(maps, look(spec, d), weather.params.cell_size, weather.params.radius, seed);
    field.overshoot = number(spec, "overshoot", field.overshoot).max(0.);
    field.anvil_depth = number(spec, "anvil_depth", field.anvil_depth).max(100.);
    field.lifetime = number(spec, "lifetime", field.lifetime).max(60.);
    field.time = weather.params.time;
    field.drift = drift(spec, weather, 10., 2.);
    field
}

/// The low cloud's cumulus as cauliflower heaps (`fields::Heaps`): `{"cell", "edge", "lift",
/// "lean", "skew", "lifetime" (s), "billows" (a scale), "wisps", "wisp_size", "levels", "ratio", "children",
/// "embed", "rise"}`, all optional.
fn heaps(o: &Value, weather: &Weather, seed: u64) -> crate::weather::fields::Heaps {
    let mut h = crate::weather::fields::Heaps::new(weather.params.radius, seed.wrapping_add(101));
    h.cell = number(o, "cell", h.cell).max(100.);
    h.edge = number(o, "edge", h.edge).max(1.);
    h.lift = number(o, "lift", h.lift).clamp(0., 1.);
    h.dome = number(o, "dome", h.dome).max(0.);
    h.lean = number(o, "lean", h.lean).max(0.);
    h.skew = number(o, "skew", h.skew).max(0.1);
    h.time = weather.params.time;
    h.lifetime = number(o, "lifetime", h.lifetime).max(60.);
    let billows = number(o, "billows", 1.).max(0.);
    h.surface.billows.iter_mut().for_each(|b| b.1 *= billows);
    h.surface.wisps = number(o, "wisps", h.surface.wisps).max(0.);
    h.surface.wisp_size = number(o, "wisp_size", h.surface.wisp_size).max(1.);
    let t = &h.tree;
    h.tree = crate::procedural::cauliflower::Cauliflower::new(
        SceneFile::parse_int(&o["levels"], t.levels).min(8), number(o, "ratio", t.ratio).clamp(0.1, 0.8),
        SceneFile::parse_int(&o["children"], t.children).clamp(1, 32), number(o, "embed", t.embed).clamp(0., 1.),
        number(o, "rise", t.rise).max(0.));
    h
}

fn look(o: &Value, d: Look) -> Look {
    Look {
        max_density: number(o, "density", d.max_density).max(0.),
        shape_size: number(o, "shape_size", d.shape_size).max(1.),
        detail_size: number(o, "detail_size", d.detail_size).max(1.),
        erosion: number(o, "erosion", d.erosion).clamp(0., 1.),
    }
}

/// One family of a weather's cloud, as a volume: `{"type": "weather_layer", "family": "low",
/// "weather": {...}}` (made by `expand`).
pub fn create_layer(o: &Value) -> SceneObject {
    let w = o["weather"].clone();
    let weather = weather_of(&w);
    let maps = weather.maps();
    let family = o["family"].as_str().unwrap_or("low");
    let spec = if w[family].is_object() { w[family].clone() } else { serde_json::json!({}) };
    let planet = Planet { radius: weather.params.radius };
    let seed = weather.params.seed.wrapping_mul(31).wrapping_add(FAMILIES.iter().position(|f| *f == family).unwrap_or(0) as u64);
    let shell = |top: f64| SceneFile::shell(&planet, top.max(1.) + 10.);
    let global = |field: Arc<dyn crate::planet::PlanetField>| -> Arc<dyn crate::volume::DensityField> {
        Arc::new(GlobalMapping { planet, field })
    };
    match family {
        "low" | "mid" => {
            let (fam, d) = if family == "low" {
                (Family::Low, Look { max_density: 0.05, shape_size: 1200., detail_size: 220., erosion: 0.4 })
            } else {
                (Family::Mid, Look { max_density: 0.03, shape_size: 3000., detail_size: 400., erosion: 0.35 })
            };
            let mut field = Stratiform::new(maps, fam, look(&spec, d), seed);
            field.drift = if fam == Family::Low { drift(&spec, &weather, 8., 0.3) } else { drift(&spec, &weather, 15., 0.1) };
            if fam == Family::Low && spec["heaps"] != Value::Bool(false) {
                field.heaps = Some(heaps(&spec["heaps"], &weather, seed));
            }
            let top = field.highest();
            create_volume(shell(top), global(Arc::new(field)), SceneFile::parse_scattering(&spec, 0.85))
        },
        "convective" => {
            let field = convective(&spec, &weather, maps);
            let top = field.highest();
            create_volume(shell(top), global(Arc::new(field)), SceneFile::parse_scattering(&spec, 0.85))
        },
        "high" => {
            let (mut c, (cell, slope, coverage, s)) = SceneFile::hogan_kew_layer(&spec, crate::procedural::hogan_kew_cirrus::CirrusGenus::Cirrus);
            c.max_density = number(&spec, "density", c.max_density);
            // Its cells drift and evolve by Hogan and Kew's own clock, unless it has its own.
            if spec.get("time").is_none() {
                c.time = weather.params.time;
            }
            let top = c.top;
            let seed = if spec.get("seed").is_some() { s } else { seed };
            // The planet's axis, in the scene: the weather's north.
            let pole = weather.frame.to_scene(&Vector3::y());
            let cirrus = crate::procedural::hogan_kew_cirrus::GlobalCirrus::new(c, cell, slope, coverage, seed, planet.radius, pole)
                .with_cover(maps.high.clone());
            let g = if spec.get("diffraction").is_some_and(|d| d != &Value::Bool(false)) { 0.5 } else { 0.75 };
            create_volume(shell(top), global(Arc::new(cirrus)), SceneFile::parse_scattering(&spec, g))
        },
        "rain" => {
            let storms = (w["convective"] != Value::Bool(false)).then(|| Arc::new(convective(&w["convective"], &weather, maps.clone())));
            let mut field = Rain::new(maps, storms, number(&spec, "shower_rate", 40.).max(0.), seed);
            field.time = weather.params.time;
            field.fall_speed = number(&spec, "fall_speed", field.fall_speed);
            field.streakiness = number(&spec, "streakiness", field.streakiness).clamp(0., 1.);
            field.streak_size = number(&spec, "streak_size", field.streak_size).max(1.);
            let top = field.highest();
            create_volume(shell(top), global(Arc::new(field)), SceneFile::parse_scattering(&spec, 0.85))
        },
        f => panic!("Unknown weather family '{}': expected one of {:?}", f, FAMILIES),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeds_are_chosen_once() {
        let mut doc = serde_json::json!({ "objects": [{ "type": "weather" }, { "type": "weather", "seed": 4 }, { "type": "sphere" }] });
        choose_seeds(&mut doc);
        let seed = doc["objects"][0]["seed"].as_u64().expect("a seed");
        assert_eq!(doc["objects"][1]["seed"], 4);
        assert!(doc["objects"][2].get("seed").is_none());
        choose_seeds(&mut doc);
        assert_eq!(doc["objects"][0]["seed"].as_u64(), Some(seed));
    }

    #[test]
    fn test_expand_into_families() {
        let objects = vec![
            serde_json::json!({ "type": "skysphere", "sun_direction": [0, 1, 1] }),
            serde_json::json!({ "type": "weather", "seed": 3, "rain": false }),
        ];
        let out = expand(objects);
        assert_eq!(out.len(), 5);
        assert!(out[1..].iter().all(|o| o["type"] == "weather_layer" && o["weather"]["sun_direction"].is_array()));
        assert!(out.iter().all(|o| o["family"] != "rain"));
    }

    #[test]
    fn test_parameters() {
        let o = serde_json::json!({ "type": "weather", "seed": 9, "cyclones": 2, "anticyclones": 0, "tropical_cyclones": 0,
            "site": { "lat": 52, "lon": -1 }, "low": { "amount": 0.5 },
            "systems": [{ "type": "cyclone", "offset": [300000, 0], "stage": 0.8 }] });
        let (p, ground) = weather_params(&o);
        assert!(ground.is_none());
        assert_eq!((p.cyclones, p.anticyclones, p.tropical_cyclones), (2, 0, 0));
        assert_eq!(p.site, (52., -1.));
        assert_eq!(p.low, 0.5);
        let w = Weather::new(p, None);
        // The scene's system first, 300 km east of the origin.
        let s = &w.systems[0];
        assert_eq!(s.stage, 0.8);
        let east = s.centre.dot(&w.frame.east) * w.params.radius;
        assert!((east - 300e3).abs() < 1e3, "{}", east);
        assert_eq!(w.systems.len(), 1 + 2 * 2);
    }

    #[test]
    fn test_landscape_sets_the_ground_and_climate() {
        let o = serde_json::json!({ "type": "weather", "seed": 1, "map_size": 16,
            "landscape": { "type": "landscape", "seed": 3, "tectonics": false, "climate": { "cells": 16 },
                           "domain": { "lat": 30, "lon": 40 } } });
        let (p, ground) = weather_params(&o);
        assert_eq!(p.site, (30., 40.));
        let ground = ground.expect("the landscape's ground");
        assert!(ground.climate.is_some());
        let w = Weather::new(p, Some(ground));
        // Over land and sea alike, the columns stand on the ground.
        let mut rng = crate::geometry::Pcg32::new(2, 0);
        let (mut land, mut sea) = (0, 0);
        for _ in 0..400 {
            let g = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let c = w.column(&g);
            assert!(c.low.base > c.ground && c.deep.base > c.ground);
            if c.ground > 0. { land += 1 } else { sea += 1 }
        }
        assert!(land > 20 && sea > 20, "{} {}", land, sea);
        assert!(!w.overhead().is_empty());
    }
}

#[cfg(test)]
mod find_storms {
    use super::*;
    /// `SCENE=file cargo test find_storms -- --ignored --nocapture`: the storms within 80 km.
    #[test]
    #[ignore]
    fn find_storms() {
        let path = std::env::var("SCENE").unwrap();
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let objects = expand(doc["objects"].as_array().unwrap().clone());
        let layer = objects.iter().find(|o| o["family"].as_str() == Some("convective")).unwrap();
        let mut w = layer["weather"].clone();
        if let Ok(t) = std::env::var("TIME") { w["time"] = serde_json::json!(t.parse::<f64>().unwrap()); }
        let weather = weather_of(&w);
        let field = convective(&w["convective"], &weather, weather.maps());
        let r = weather.params.radius;
        let mut seen = std::collections::HashMap::new();
        for i in -40..=40 {
            for j in -40..=40 {
                let p = Vector3::new(i as f64 * 2000., r, j as f64 * 2000.);
                field.towers(&p.normalize(), |t| {
                    let pos = field.drift.forth(&t.axis) * r - Vector3::new(0., r, 0.);
                    seen.entry(t.id).or_insert(format!("at ({:.0}, {:.0}) dist {:.0} age {:.2} base {:.0} top {:.0} full {:.0} radius {:.0} mature {} wind ({:.2}, {:.2})",
                        pos.x, pos.z, (pos.x * pos.x + pos.z * pos.z).sqrt(), t.age, t.base, t.top, t.full_top, t.radius, t.mature, field.drift.forth(&t.wind).x, field.drift.forth(&t.wind).z));
                });
            }
        }
        let mut v: Vec<_> = seen.values().cloned().collect();
        v.sort_by_key(|s| s.split("dist ").nth(1).unwrap().split(' ').next().unwrap().parse::<f64>().unwrap() as i64);
        for s in v.iter().take(25) { println!("{}", s); }
    }
}
