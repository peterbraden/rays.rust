//! # Weather
//!
//! A day's weather over the whole planet, generated from a seed, and the clouds it makes,
//! each kind where the weather would put it, together: the sky as a whole, not one layer.
//!
//! The weather is built in two parts, in a geographic frame (north +y, as the landscape's):
//!
//! 1. **The climate's background** (`Weather::column`). By latitude and season: the
//!    intertropical convergence zone (ITCZ), following the sun, where the trade winds meet and
//!    rise in clusters of cumulonimbus; the sinking, drying air of the subtropical highs; the
//!    storm tracks; the cold, stable poles. The surface temperature (from the landscape's
//!    climate, `terrain::climate`, if the scene has one, or else North's energy balance by
//!    latitude) sets the tropopause and how unstable the air is; daytime heating (from the
//!    sun's direction, peaking mid-afternoon) stirs up convection over land; the sea keeps the
//!    air moist. Noise at synoptic and mesoscale sizes breaks it up.
//! 2. **Weather systems** (`systems`): extratropical cyclones with their fronts, anticyclones,
//!    and tropical cyclones, placed at random along the storm tracks, the subtropics and the
//!    warm tropical oceans (or as the scene says), each adding its pattern of ascent, moisture
//!    and frontal cloud.
//!
//! From these, at each point, a column of air (`Column`): its relative humidity, ascent,
//! instability, the height of the boundary layer's inversion and of the lifting condensation
//! level (125 m per °C of dew-point depression, about 25 m per percent of humidity short of
//! saturation), and the tropopause. And from the column, its clouds, in the four families the
//! WMO's atlas sorts them into by height, and rain:
//!
//! - **Low**: stratus where the air is near saturation and stable, stratocumulus under the
//!   inversion (thickest over the sea beneath subsiding air), fair-weather cumulus where the
//!   ground heats the air, all based at the condensation level.
//! - **Mid**: altostratus and nimbostratus where air rises on a large scale (fronts, the comma
//!   heads of lows), altocumulus in moist layers aloft.
//! - **High**: cirrus along the jets and ahead of warm fronts, thickening to cirrostratus; the
//!   anvils of deep convection spread out as cirrus.
//! - **Deep convection**: cumulus congestus and cumulonimbus (`fields::Convective`), rising from
//!   the condensation level towards the tropopause where the air is unstable and rising, with
//!   anvils spread downwind, and the shields of anvils over clusters of storms.
//! - **Precipitation**: rain (or snow, where it is below freezing) from nimbostratus and
//!   beneath cumulonimbus, evaporating as it falls through dry air (virga).
//!
//! With `time`, it all runs on: the systems move along their tracks, mature and die (and new
//! ones are born), the background drifts and changes, and the clouds drift with the wind,
//! boil, and live out their lives (`fields`), all continuously, for animation.
//!
//! Each family is evaluated once per texel corner of an equi-angular cube map over the planet
//! (`planet::CubeMap`), and each renders as one global volume (`fields`), so the families stay
//! consistent with each other: the rain falls from the cloud above it, the anvils spread from
//! the storms beneath them, the cirrus thickens ahead of the front that brings the rain.

pub mod systems;
pub mod fields;
pub mod scene;

use crate::na::{Vector2, Vector3};
use crate::geometry::Pcg32;
use crate::noise::PerlinNoise;
use crate::planet::CubeMap;
use crate::procedural::nubis_clouds::{lerp, remap, smoothstep, stretched};
use crate::terrain::climate::{self, Climate, ClimateParams};
use crate::weather::systems::{Effects, FrontalParams, Kind, System};
use std::f64::consts::{PI, TAU};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;

/// Seconds in a day.
pub const DAY: f64 = 86400.;

/// A system the scene asks for, rather than one placed at random.
#[derive(Clone, Debug)]
pub struct SystemSpec {
    pub kind: Kind,
    /// Where: latitude and longitude (degrees), or metres east and north of the scene's origin.
    pub lat_lon: Option<(f64, f64)>,
    pub offset: Option<Vector2<f64>>,
    pub radius: Option<f64>,
    pub intensity: Option<f64>,
    pub stage: Option<f64>,
    /// Degrees, anticlockwise.
    pub tilt: Option<f64>,
    pub eye: Option<f64>,
    /// Eastward and poleward, m/s: by default as its kind moves.
    pub velocity: Option<Vector2<f64>>,
}

#[derive(Clone, Debug)]
pub struct WeatherParams {
    pub seed: u64,
    /// The latitude and longitude (degrees) at the scene's origin.
    pub site: (f64, f64),
    pub radius: f64,
    /// The sun's direction in the scene, which sets the season (its declination) and where it
    /// is day; None for a season by `day_of_year` and middling heating everywhere.
    pub sun: Option<Vector3<f64>>,
    /// Day of the year (0 to 365, 172 the June solstice), when there is no sun to say.
    pub day_of_year: f64,
    /// Hours after noon that convection peaks.
    pub convection_lag: f64,
    /// Scales the moisture everywhere, and how unstable the air is.
    pub humidity: f64,
    pub instability: f64,
    /// Scales the cover of each family of cloud.
    pub low: f64,
    pub mid: f64,
    pub high: f64,
    pub deep: f64,
    /// Systems placed at random: cyclones and anticyclones per hemisphere, tropical cyclones
    /// in all.
    pub cyclones: usize,
    pub anticyclones: usize,
    pub tropical_cyclones: usize,
    /// Typical radius of a cyclone, metres.
    pub cyclone_size: f64,
    /// Latitudes (degrees) of the storm tracks and the ITCZ; None to follow the climate and
    /// season.
    pub storm_track: Option<f64>,
    pub itcz: Option<f64>,
    pub systems: Vec<SystemSpec>,
    pub fronts: FrontalParams,
    /// Sizes of the synoptic and mesoscale variations, metres.
    pub synoptic_size: f64,
    pub mesoscale_size: f64,
    /// Heights of the tropopause at the equator and the poles, metres.
    pub tropopause: (f64, f64),
    /// The climate, when the scene has no landscape to take it from.
    pub climate: ClimateParams,
    /// Texels along each cube face of the maps.
    pub map_size: usize,
    /// The spacing of convective cells, metres.
    pub cell_size: f64,
    /// Seconds since the weather began, for animation: systems move, grow and die, the
    /// background drifts and changes, and the clouds drift with the wind and evolve.
    pub time: f64,
    /// How systems move (m/s eastward, and poleward) and how long they live (s): cyclones
    /// along the storm tracks, highs slowly, tropical cyclones westward and poleward.
    pub cyclone_velocity: Vector2<f64>,
    pub anticyclone_velocity: Vector2<f64>,
    pub tropical_velocity: Vector2<f64>,
    pub cyclone_lifetime: f64,
    pub anticyclone_lifetime: f64,
    pub tropical_lifetime: f64,
    /// How fast the background drifts east (m/s), and the periods over which its synoptic
    /// and mesoscale patterns change (s).
    pub drift: f64,
    pub synoptic_period: f64,
    pub mesoscale_period: f64,
}

impl WeatherParams {
    /// Defaults, with counts of systems drawn from the seed.
    pub fn new(seed: u64) -> WeatherParams {
        let mut rng = Pcg32::new(seed, 7);
        let mut pick = |lo: usize, hi: usize| lo + (rng.next_f64() * (hi - lo + 1) as f64) as usize;
        let cyclones = pick(3, 6);
        let anticyclones = pick(2, 4);
        let tropical_cyclones = [0, 0, 1, 1, 1, 2, 2, 3][pick(0, 7)];
        let day_of_year = Pcg32::new(seed, 8).next_f64() * 365.;
        WeatherParams {
            seed, site: (45., 0.), radius: crate::planet::EARTH_RADIUS, sun: None, day_of_year,
            convection_lag: 3., humidity: 1., instability: 1., low: 1., mid: 1., high: 1., deep: 1.,
            cyclones, anticyclones, tropical_cyclones, cyclone_size: 1.2e6, storm_track: None, itcz: None,
            systems: vec![], fronts: FrontalParams::default(), synoptic_size: 1.2e6, mesoscale_size: 2e5,
            tropopause: (16500., 9000.), climate: ClimateParams::default(), map_size: 512, cell_size: 20000.,
            time: 0., cyclone_velocity: Vector2::new(12., 1.5), anticyclone_velocity: Vector2::new(3., 0.),
            tropical_velocity: Vector2::new(-5., 2.), cyclone_lifetime: 4. * DAY, anticyclone_lifetime: 9. * DAY,
            tropical_lifetime: 8. * DAY, drift: 8., synoptic_period: 3. * DAY, mesoscale_period: 0.5 * DAY,
        }
    }
}

/// Converts between the scene's directions (from the planet's centre, with the origin's
/// straight up +y) and geographic ones (north +y), as `terrain::scenery::Frame` does.
#[derive(Clone, Copy, Debug)]
pub struct GeoFrame {
    pub up: Vector3<f64>,
    pub east: Vector3<f64>,
    pub north: Vector3<f64>,
}

impl GeoFrame {
    pub fn new(lat: f64, lon: f64) -> GeoFrame {
        let (up, east, north) = crate::terrain::domain::Domain::frame(lat, lon);
        GeoFrame { up, east, north }
    }

    pub fn to_geo(&self, s: &Vector3<f64>) -> Vector3<f64> {
        self.east * s.x + self.up * s.y - self.north * s.z
    }

    pub fn to_scene(&self, g: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(g.dot(&self.east), g.dot(&self.up), -g.dot(&self.north))
    }
}

/// A cloud in a column: how much of the sky it covers (0 to 1), its base and top (metres
/// above sea level), and its form (0 stratiform to 1 cumuliform).
#[derive(Clone, Copy, Debug, Default)]
pub struct Cloud {
    pub cover: f64,
    pub base: f64,
    pub top: f64,
    pub form: f64,
}

/// The weather at a point.
#[derive(Clone, Copy, Debug, Default)]
pub struct Column {
    pub low: Cloud,
    pub mid: Cloud,
    /// Deep convection: the cover is the share of convective cells that are active; the top
    /// is that of the tallest.
    pub deep: Cloud,
    /// Cirrus cover.
    pub high: f64,
    /// Precipitation: extinction (per metre) at the cloud's base, where it falls from (metres),
    /// and the depth over which it evaporates (metres).
    pub rain: f64,
    pub rain_base: f64,
    pub rain_fall: f64,
    /// The wind aloft, geographic (m/s).
    pub wind: Vector3<f64>,
    /// Surface temperature (°C), relative humidity of the boundary layer (0 to 1), ground
    /// height (m), and the tropopause (m).
    pub temperature: f64,
    pub humidity: f64,
    pub ground: f64,
    pub tropopause: f64,
}

/// What the weather knows of the ground, from the landscape.
pub struct Ground {
    pub planet: Arc<crate::terrain::planet::Planet>,
    pub climate: Option<Arc<Climate>>,
}

pub struct Weather {
    pub params: WeatherParams,
    pub frame: GeoFrame,
    pub systems: Vec<System>,
    ground: Option<Ground>,
    /// Zonal mean temperature by sin(latitude), for a planet without a landscape.
    zonal: Vec<f64>,
    /// The sun's declination (radians), and its direction (geographic), moved on by the
    /// afternoon's lag, for heating.
    declination: f64,
    heating_sun: Option<Vector3<f64>>,
    synoptic: PerlinNoise,
    mesoscale: PerlinNoise,
    aloft: PerlinNoise,
    maps: OnceLock<Arc<Maps>>,
}

/// The maps a render reads (in the scene's directions).
pub struct Maps {
    pub low: LayerMaps,
    pub mid: LayerMaps,
    pub deep: LayerMaps,
    /// The wind aloft (scene vectors, m/s), for spreading anvils.
    pub wind: [CubeMap; 3],
    /// The anvils' shields over clusters of storms (cover).
    pub anvil: CubeMap,
    /// Bounds for deep convection, which reaches out from its cells' centres: the most cover,
    /// the highest top, the lowest and the highest base nearby.
    pub deep_reach: CubeMap,
    pub deep_top_reach: CubeMap,
    pub deep_base_low: CubeMap,
    pub deep_base_high: CubeMap,
    /// The lowest top nearby, negated: where the anvils' shields can begin.
    pub deep_top_low: CubeMap,
    pub high: Arc<CubeMap>,
    pub rain: CubeMap,
    pub rain_base: CubeMap,
    pub rain_fall: CubeMap,
}

/// A family of cloud's maps: cover, base, top and form, and the base negated (whose texel
/// maximum is the lowest base over the texel).
pub struct LayerMaps {
    pub cover: CubeMap,
    pub base: CubeMap,
    pub top: CubeMap,
    pub form: CubeMap,
    pub base_low: CubeMap,
}

impl Weather {
    pub fn new(params: WeatherParams, ground: Option<Ground>) -> Weather {
        let frame = GeoFrame::new(params.site.0, params.site.1);
        let tilt = params.climate.tilt.to_radians();
        let sun_geo = params.sun.map(|s| frame.to_geo(&s.normalize()));
        let declination = match sun_geo {
            // The sun's latitude, as far as the tilt allows: a scene's sun is placed for the
            // look of the sky, not always for a season the planet could have.
            Some(s) => s.y.clamp(-1., 1.).asin().clamp(-tilt, tilt),
            None => (tilt.sin() * (TAU * (params.day_of_year + params.time / DAY - 80.) / 365.25).sin()).asin(),
        };
        // Convection peaks hours after noon, where the sun has moved on: heat by a sun as
        // many hours behind, ie. further east.
        let heating_sun = sun_geo.map(|s| {
            let a = params.convection_lag / 24. * TAU;
            let (c, sn) = (a.cos(), a.sin());
            // About the axis (y): longitude increases towards -z from +x (`Domain::frame`).
            Vector3::new(s.x * c + s.z * sn, s.y, -s.x * sn + s.z * c)
        });
        let zonal = climate::energy_balance(&params.climate, 180);
        let seed = params.seed;
        let mut w = Weather {
            frame, systems: vec![], ground, zonal, declination, heating_sun,
            synoptic: PerlinNoise::shuffled(seed.wrapping_mul(7).wrapping_add(1)),
            mesoscale: PerlinNoise::shuffled(seed.wrapping_mul(7).wrapping_add(2)),
            aloft: PerlinNoise::shuffled(seed.wrapping_mul(7).wrapping_add(3)),
            maps: OnceLock::new(),
            params,
        };
        w.systems = w.place_systems();
        w
    }

    /// The sun's declination over the tilt: +1 at the June solstice, -1 at December's.
    fn season(&self) -> f64 {
        let tilt = self.params.climate.tilt.to_radians();
        if tilt > 1e-6 { (self.declination / tilt).clamp(-1., 1.) } else { 0. }
    }

    pub fn itcz(&self) -> f64 {
        self.params.itcz.unwrap_or(4. + 0.4 * self.declination.to_degrees())
    }

    /// The storm track's latitude in a hemisphere (sign of `h`), degrees from the equator:
    /// further poleward in summer.
    pub fn storm_track(&self, h: f64) -> f64 {
        self.params.storm_track.unwrap_or(self.params.climate.hadley + 17. + 4. * self.season() * h.signum())
    }

    fn is_sea(&self, g: &Vector3<f64>) -> bool {
        self.ground.as_ref().is_none_or(|gr| gr.planet.regional_elevation(g) < 0.)
    }

    fn geo(lat: f64, lon: f64) -> Vector3<f64> {
        let (lat, lon) = (lat.to_radians(), lon.to_radians());
        Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin())
    }

    /// How a kind of system moves (eastward and poleward, m/s) and how long it lives (s).
    fn motion(&self, kind: Kind) -> (Vector2<f64>, f64) {
        let p = &self.params;
        match kind {
            Kind::Cyclone => (p.cyclone_velocity, p.cyclone_lifetime),
            Kind::Anticyclone => (p.anticyclone_velocity, p.anticyclone_lifetime),
            Kind::Tropical => (p.tropical_velocity, p.tropical_lifetime),
        }
    }

    /// Where a system that started at `centre` is after `dt` seconds moving at `velocity`
    /// (eastward and poleward, m/s): along its latitude, and towards its pole.
    fn moved(&self, centre: &Vector3<f64>, velocity: &Vector2<f64>, dt: f64) -> Vector3<f64> {
        let lat = centre.y.clamp(-1., 1.).asin();
        let lon = (-centre.z).atan2(centre.x);
        let r = self.params.radius;
        let h = if lat >= 0. { 1. } else { -1. };
        let lat2 = (lat + h * velocity.y * dt / r).clamp(-1.45, 1.45);
        let lon2 = lon + velocity.x * dt / (r * ((lat + lat2) / 2.).cos().max(0.2));
        Weather::geo(lat2.to_degrees(), lon2.to_degrees())
    }

    /// The scene's systems, then random ones along the storm tracks, in the subtropics and
    /// over warm seas, all as they are at `time`.
    ///
    /// The scene's systems start where they are put and move as their kind does; a cyclone
    /// goes on occluding (its `stage` rising over its lifetime) and, once occluded, fills.
    /// The random ones live in slots: each is born, moves, grows and dies over its kind's
    /// lifetime, at an age set by the slot's phase, then is born again somewhere new (by the
    /// slot's next cycle's seed). Strength rises from nothing at birth and falls to nothing at
    /// death, so through time nothing appears or vanishes all at once.
    fn place_systems(&self) -> Vec<System> {
        let p = &self.params;
        let t = p.time;
        let mut rng = Pcg32::new(p.seed, 11);
        let mut u = || rng.next_f64();
        let mut out = vec![];
        for s in &p.systems {
            let centre = match (s.lat_lon, s.offset) {
                (Some((lat, lon)), _) => Weather::geo(lat, lon),
                (None, Some(o)) => {
                    // Along the surface from the origin: east and north.
                    let d = o.norm();
                    let dir = if d > 0. { (self.frame.east * o.x + self.frame.north * o.y) / d } else { Vector3::zeros() };
                    let a = d / p.radius;
                    self.frame.up * a.cos() + dir * a.sin()
                },
                (None, None) => self.frame.up,
            };
            let (radius, eye) = match s.kind {
                Kind::Cyclone => (p.cyclone_size, 0.),
                Kind::Anticyclone => (1.8e6, 0.),
                Kind::Tropical => (5e5, 2.2e4),
            };
            let (velocity, lifetime) = self.motion(s.kind);
            let velocity = s.velocity.unwrap_or(velocity);
            let stage = s.stage.unwrap_or(0.5) + if s.kind == Kind::Cyclone { t / lifetime } else { 0. };
            // Occluded, a cyclone fills.
            let filling = if s.kind == Kind::Cyclone { 1. - smoothstep(1., 1.4, stage) } else { 1. };
            out.push(System::new(s.kind, self.moved(&centre, &velocity, t), s.radius.unwrap_or(radius),
                s.intensity.unwrap_or(0.85) * filling, stage.min(1.), s.tilt.unwrap_or(0.).to_radians(), s.eye.unwrap_or(eye)));
        }
        // A slot's age (0 to 1 of its lifetime) and cycle at `t`, by its phase.
        let cycle = |phase: f64, lifetime: f64| {
            let x = t / lifetime + phase;
            (x - x.floor(), x.floor() as i64)
        };
        // The life cycle's strength: rising after birth, fading before death.
        let life = |age: f64| smoothstep(0., 0.12, age) * (1. - smoothstep(0.8, 1., age));
        let gauss = |u1: f64, u2: f64| (-2. * u1.max(1e-12).ln()).sqrt() * (TAU * u2).cos();
        // Each cycle of each slot its own draws.
        let draws = |slot: u64, n: i64| Pcg32::new(p.seed ^ slot.wrapping_mul(0x9E37_79B9_7F4A_7C15), (n as u64).wrapping_add(1 << 40));
        let mut slot = 0u64;
        for h in [1., -1.] {
            // Cyclones: a family along each storm track, each born on its own stretch of it.
            let n = p.cyclones;
            let start = u() * 360.;
            for k in 0..n {
                slot += 1;
                let (velocity, lifetime) = self.motion(Kind::Cyclone);
                let (age, c) = cycle(u(), lifetime);
                let mut r = draws(slot, c);
                let mut v = || r.next_f64();
                let lat = h * (self.storm_track(h) + 7. * gauss(v(), v())).clamp(25., 75.);
                let lon = start + 360. * k as f64 / n as f64 + (v() - 0.5) * 240. / n as f64;
                let born = Weather::geo(lat, lon);
                let (radius, intensity, tilt) = (p.cyclone_size * (0.7 + 0.6 * v()), 0.55 + 0.45 * v(), (v() - 0.5) * 0.7);
                out.push(System::new(Kind::Cyclone, self.moved(&born, &velocity, age * lifetime), radius,
                    intensity * life(age), (age * 1.15).min(1.), tilt, 0.));
            }
            // Highs: the subtropical ridge, between the lows.
            let n = p.anticyclones;
            let start = u() * 360.;
            for k in 0..n {
                slot += 1;
                let (velocity, lifetime) = self.motion(Kind::Anticyclone);
                let (age, c) = cycle(u(), lifetime);
                let mut r = draws(slot, c);
                let mut v = || r.next_f64();
                let lat = h * (p.climate.hadley + 2. + 5. * gauss(v(), v())).clamp(15., 50.);
                let lon = start + 360. * k as f64 / n as f64 + (v() - 0.5) * 120. / n as f64;
                let born = Weather::geo(lat, lon);
                let (radius, intensity, tilt) = (1.3e6 + 1.2e6 * v(), 0.5 + 0.5 * v(), (v() - 0.5) * 0.6);
                out.push(System::new(Kind::Anticyclone, self.moved(&born, &velocity, age * lifetime), radius,
                    intensity * life(age), 0., tilt, 0.));
            }
        }
        // Tropical cyclones: mostly in the summer hemisphere, born over the sea.
        for _ in 0..p.tropical_cyclones {
            slot += 1;
            let (velocity, lifetime) = self.motion(Kind::Tropical);
            let (age, c) = cycle(u(), lifetime);
            let mut r = draws(slot, c);
            let mut v = || r.next_f64();
            let h = if v() < 0.5 + 0.4 * self.season() { 1. } else { -1. };
            for _attempt in 0..50 {
                let (lat, lon) = (h * (8. + 12. * v()), v() * 360.);
                let born = Weather::geo(lat, lon);
                if self.is_sea(&born) {
                    let (radius, intensity, tilt, eye) = (3.5e5 + 3e5 * v(), 0.7 + 0.3 * v(), v() * TAU, 1.2e4 + 2e4 * v());
                    let here = self.moved(&born, &velocity, age * lifetime);
                    // Over land, cut off from the warm sea, it dies.
                    let landfall = if self.is_sea(&here) { 1. } else { 0.4 };
                    out.push(System::new(Kind::Tropical, here, radius, intensity * life(age) * landfall, 0., tilt, eye));
                    break;
                }
            }
        }
        out
    }

    /// The surface temperature (°C), annual rainfall (mm), ground height (m), how much is sea
    /// (0 to 1), and continentality at `g`.
    fn surface(&self, g: &Vector3<f64>) -> (f64, f64, f64, f64, f64) {
        let lat = g.y.clamp(-1., 1.).asin();
        let season = self.season() * lat.signum();
        match &self.ground {
            Some(gr) => {
                let e = gr.planet.regional_elevation(g);
                let sea = 1. - smoothstep(-40., 40., e);
                let ground = e.max(0.);
                match &gr.climate {
                    Some(c) => {
                        let l = c.local(g, ground);
                        let t = l.mean + season * (l.summer - l.winter) / 2.;
                        (t, l.precipitation, ground, sea, c.continentality(g))
                    },
                    None => {
                        let (t, p) = self.zonal_climate(lat, season, 1. - sea);
                        (t - climate::LAPSE_RATE * ground, p, ground, sea, 1. - sea)
                    },
                }
            },
            None => {
                let (t, p) = self.zonal_climate(lat, season, 0.);
                (t, p, 0., 1., 0.)
            },
        }
    }

    /// Temperature (°C) and rainfall (mm/yr) at latitude `lat` by the zonal climate, in the
    /// season (+1 summer), damped by the sea (`land` 0).
    fn zonal_climate(&self, lat: f64, season: f64, land: f64) -> (f64, f64) {
        let n = self.zonal.len();
        let k = (((lat.sin() + 1.) / 2. * n as f64) as usize).min(n - 1);
        let c = &self.params.climate;
        let (_, june, december) = climate::insolation(c.solar, c.tilt.to_radians(), lat);
        let swing = (june - december).abs() / 2. * (0.02 + 0.07 * land);
        (self.zonal[k] + season * swing, climate::zonal_precipitation(c, lat))
    }

    /// How strongly the ground is being heated by the sun (0 to 1), in the afternoon.
    fn heating(&self, g: &Vector3<f64>) -> f64 {
        match &self.heating_sun {
            Some(s) => g.dot(s).max(0.).sqrt(),
            None => 0.5,
        }
    }

    /// Noise over the planet (0 to 1) with features `size` across, drifting east with the
    /// background flow and changing over `period`: two independent fields, a cos ωt + b sin ωt,
    /// which has the same statistics at every time but changes continuously.
    fn evolving(&self, noise: &PerlinNoise, g: &Vector3<f64>, size: f64, octaves: u32, period: f64, offset: Vector3<f64>) -> f64 {
        let p = &self.params;
        // Back to where the air now here was at the start: west, about the axis (y).
        let g = rotate_about_y(g, -p.drift * p.time / p.radius);
        let q = g * (p.radius / size) + offset;
        let w = TAU * p.time / period;
        let a = noise.fbm(q.x, q.y, q.z, octaves, 0.5, 2.) - 0.5;
        let b = if w.sin() != 0. { noise.fbm(q.x + 57.3, q.y + 19.1, q.z + 83.7, octaves, 0.5, 2.) - 0.5 } else { 0. };
        stretched(0.5 + a * w.cos() + b * w.sin())
    }

    /// The weather at `g`, a geographic unit vector.
    pub fn column(&self, g: &Vector3<f64>) -> Column {
        let p = &self.params;
        let lat = g.y.clamp(-1., 1.).asin();
        let latd = lat.to_degrees();
        let (temperature, rainfall, ground, sea, continentality) = self.surface(g);
        let land = 1. - sea;
        let heating = self.heating(g);
        let synoptic = self.evolving(&self.synoptic, g, p.synoptic_size, 4, p.synoptic_period, Vector3::zeros());
        let meso = self.evolving(&self.mesoscale, g, p.mesoscale_size, 3, p.mesoscale_period, Vector3::zeros());
        let meso2 = self.evolving(&self.aloft, g, p.mesoscale_size / 1.7, 3, p.mesoscale_period, Vector3::zeros());
        let jet_noise = self.evolving(&self.aloft, g, p.synoptic_size / 1.6, 3, p.synoptic_period, Vector3::new(31.7, 0., 0.));

        // The tropopause: high over the warm tropics, low over the poles.
        let sea_level = temperature + climate::LAPSE_RATE * ground;
        let tropopause = lerp(p.tropopause.1, p.tropopause.0, smoothstep(-15., 25., sea_level));

        // The background: moisture from the climate's rainfall, the ITCZ rising, the
        // subtropics sinking, varied at synoptic scale.
        let itcz = (-((latd - self.itcz()) / 4.).powi(2)).exp();
        let subtropics = (-((latd.abs() - (p.climate.hadley - 2.)) / 7.).powi(2)).exp();
        // The sea keeps the air above it moist, even under the subtropics' sinking air.
        let mut humidity = 0.42 + 0.1 * sea + 0.3 * smoothstep(200., 1800., rainfall) - 0.15 * continentality + 0.3 * (synoptic - 0.5);
        let mut ascent = 0.9 * itcz * (0.4 + 0.9 * meso) - 0.5 * subtropics - 0.2 * smoothstep(70., 85., latd.abs())
            + 0.5 * (synoptic - 0.5);
        let mut instability = 0.6 * smoothstep(20., 29., sea_level) * sea
            + land * heating * smoothstep(8., 30., temperature)
            + 0.15 * smoothstep(10., 25., temperature);
        let mut inversion = 1200. + 1000. * land * heating + 300. * (meso - 0.5);

        // The systems.
        let mut e = Effects::default();
        let fp = p.fronts;
        for s in &self.systems {
            if let Some(local) = s.local(g, p.radius) {
                s.effects(&local, &mut e, &fp, tropopause, sea);
            }
        }
        humidity = ((humidity + e.humidity + 0.2 * itcz) * p.humidity).clamp(0.05, 1.);
        ascent += e.ascent;
        let subsidence = (-ascent).max(0.);
        instability = ((instability + e.instability + 0.3 * itcz * sea) * p.instability - 0.6 * subsidence).clamp(0., 1.);
        inversion = (inversion + e.inversion - 700. * subsidence).max(300.);
        let clear = e.clear.clamp(0., 1.);

        // The condensation level: 25 m per percent short of saturation; higher over land by
        // day, as thermals rise further before their water condenses.
        let lcl = (25. * (100. - 100. * humidity) + 500. * land * heating).max(150.);
        let base = ground + lcl;

        // Deep convection, where the air is unstable, rising and moist; never in an eye.
        let deep = ((instability * remap(ascent, -0.05, 0.6) * remap(humidity, 0.5, 0.85) * remap(meso, 0.3, 0.75)).max(e.deep)
            * (1. - e.eye) * p.deep).clamp(0., 1.);
        let deep_top = tropopause * (0.8 + 0.2 * instability);

        // Low cloud: stratus where near saturation and stable, stratocumulus under subsiding
        // air over the sea, cumulus over heated ground; held under the inversion.
        let stratus = remap(humidity, 0.8, 0.98) * (1. - 0.7 * instability);
        let marine = sea * remap(subsidence, 0., 0.5) * remap(humidity, 0.5, 0.8) * smoothstep(12., 25., latd.abs()) * (1. - smoothstep(40., 55., latd.abs()));
        // Shallow cumulus over warm seas, capped by the inversion rather than stopped by the
        // subsidence above it: the trade-wind cumulus.
        let shallow = 0.6 * sea * smoothstep(14., 26., sea_level) * (1. - 0.5 * clear);
        let cumulus = 0.65 * instability.max(shallow) * remap(humidity, 0.4, 0.8) * (1. - 0.6 * deep);
        let background = [(stratus, 0.05), (marine, 0.4), (cumulus, 0.95)];
        let weight: f64 = background.iter().map(|c| c.0).sum::<f64>() + e.low_weight;
        let low_form = (background.iter().map(|c| c.0 * c.1).sum::<f64>() + e.low_form + 0.01 * 0.5) / (weight + 0.01);
        let low_cover = (stratus.max(marine).max(cumulus) * (0.3 + 0.9 * synoptic) * (0.5 + 0.9 * (meso - 0.5)).max(0.)).max(e.low * (0.7 + 0.9 * (meso - 0.5)).clamp(0.2, 1.)).clamp(0., 1.) * p.low;
        let low_depth = lerp(lerp(250., 600., meso), 400. + 1800. * instability.max(0.6 * shallow), low_form);
        let low = Cloud { cover: low_cover.clamp(0., 1.), base, top: base + low_depth.min((inversion - lcl).max(250.) + 1500. * low_form), form: low_form };

        // Mid cloud: frontal (from the systems), altostratus where rising on a large scale,
        // altocumulus in moist layers aloft.
        // Where the air is unstable its ascent goes into convection, not sheets of cloud (the
        // anvils' shields are the stratiform part of the storms).
        let stratiform = remap(ascent, 0.3, 0.9) * remap(humidity, 0.6, 0.9) * (1. - 0.85 * instability);
        let alto = remap(meso2, 0.55, 0.8) * remap(humidity, 0.4, 0.75) * (1. - remap(ascent.abs(), 0.2, 0.6));
        let alto_base = ground.max(0.) + 2800. + 2500. * meso;
        let eps = 0.01;
        let w_mid = e.mid_weight + stratiform + alto + eps;
        let mid_base = (e.mid_base + stratiform * 2500. + (alto + eps) * alto_base) / w_mid;
        let mid_top = (e.mid_top + stratiform * 6500. + (alto + eps) * (alto_base + 500.)) / w_mid;
        let mid_form = ((alto + eps) * 0.55 + stratiform * 0.05 + e.mid_weight * 0.05) / w_mid;
        let mid_cover = (e.mid.max(stratiform).max(0.8 * alto) * (1. - 0.8 * clear) * p.mid).clamp(0., 1.);
        let mid_base = mid_base.max(base).min(tropopause - 1500.);
        let mid = Cloud { cover: mid_cover, base: mid_base, top: mid_top.min(tropopause - 500.).max(mid_base + 300.), form: mid_form };

        // Cirrus: along the jets, broken up; over the ITCZ; and from the systems.
        let jets = (-((latd.abs() - self.storm_track(latd) + 5.) / 10.).powi(2)).exp();
        let high = ((0.75 * jets * remap(jet_noise, 0.35, 0.75)).max(0.5 * itcz * meso2).max(e.high)
            * (1. - 0.7 * clear * (1. - e.high)) * p.high).clamp(0., 1.);

        // Rain from nimbostratus and the systems; snow where it is freezing, which in the same
        // fall shows several times the extinction.
        let mut rain_rate = e.rain + 1.5 * stratiform * (1. - smoothstep(2000., 3500., mid.base)) * mid.cover;
        rain_rate *= p.humidity.min(1.5);
        let snow = if temperature < 1. { 3. } else { 1. };
        let rain = crate::procedural::precipitation::rain_extinction(rain_rate) * snow;
        let rain_base = if mid.cover > low.cover { mid.base } else { low.base.min(mid.base) };
        // Falling through dry air it evaporates within a few hundred metres: virga.
        let rain_fall = lerp(600., 30000., remap(humidity, 0.45, 0.95));

        Column {
            low, mid, high,
            deep: Cloud { cover: deep, base, top: deep_top.max(base + 2000.), form: 1. },
            rain, rain_base, rain_fall,
            wind: self.wind_aloft(g, latd),
            temperature, humidity, ground, tropopause,
        }
    }

    /// The wind aloft (geographic, m/s): the jets, the tropics' easterlies, and the systems'
    /// circulation, weakened with height.
    pub fn wind_aloft(&self, g: &Vector3<f64>, latd: f64) -> Vector3<f64> {
        let east = Vector3::y().cross(g);
        let east = if east.norm() > 1e-9 { east.normalize() } else { Vector3::x() };
        let jet = 35. * (-((latd.abs() - self.storm_track(latd)) / 9.).powi(2)).exp() - 8. * (-((latd - self.itcz()) / 14.).powi(2)).exp();
        let mut w = east * jet;
        for s in &self.systems {
            if let Some(local) = s.local(g, self.params.radius) {
                let c = s.circulation(&local);
                w += s.to_geo(&c) * if s.kind == Kind::Tropical { -0.3 } else { 0.6 };
            }
        }
        // Horizontal.
        w - g * g.dot(&w)
    }

    /// What is overhead at the scene's origin, in words.
    pub fn overhead(&self) -> String {
        let c = self.column(&self.frame.up);
        let name = |l: &Cloud, sheet: &str, mixed: &str, heap: &str| {
            let kind = if l.form < 0.3 { sheet } else if l.form < 0.7 { mixed } else { heap };
            format!("{} {:.0}% ({:.0}-{:.0} m)", kind, 100. * l.cover, l.base, l.top)
        };
        let mid = if c.mid.base < 2000. && c.mid.top - c.mid.base > 3000. { "nimbostratus" } else { "altostratus" };
        // Visibility in rain, by Koschmieder's 3.9 / extinction.
        let rain = if c.rain > 1e-6 { format!("{} ({:.1} km visibility)", if c.temperature < 1. { "snow" } else { "rain" }, 3.9 / c.rain / 1000.) } else { "dry".into() };
        format!("low {}, mid {}, cirrus {:.0}%, cumulonimbus {:.0}% (to {:.0} m), {}; {:.0} °C, humidity {:.0}%",
            name(&c.low, "stratus", "stratocumulus", "cumulus"), name(&c.mid, mid, "altocumulus", "altocumulus"),
            100. * c.high, 100. * c.deep.cover, c.deep.top, rain, c.temperature, 100. * c.humidity)
    }

    /// The maps, built on first use.
    pub fn maps(&self) -> Arc<Maps> {
        self.maps.get_or_init(|| crate::geometry::build_apart(|| Arc::new(self.build_maps()))).clone()
    }

    fn build_maps(&self) -> Maps {
        use rayon::prelude::*;
        let start = std::time::Instant::now();
        let n = self.params.map_size.max(2);
        let dirs = CubeMap::corner_directions(n);
        const K: usize = 23;
        let values: Vec<[f32; K]> = dirs.par_iter().map(|d| {
            let c = self.column(&self.frame.to_geo(d));
            let w = self.frame.to_scene(&c.wind);
            let l = |x: &Cloud| [x.cover, x.base, x.top, x.form, -x.base];
            let mut v = [0f32; K];
            let all: Vec<f64> = l(&c.low).into_iter().chain(l(&c.mid)).chain(l(&c.deep))
                .chain([w.x, w.y, w.z, c.high, c.rain, c.rain_base, c.rain_fall, c.tropopause]).collect();
            for (k, x) in all.into_iter().enumerate() { v[k] = x as f32; }
            v
        }).collect();
        let map = |k: usize| CubeMap::from_corners(n, values.iter().map(|v| v[k]).collect());
        let layer = |k: usize| LayerMaps { cover: map(k), base: map(k + 1), top: map(k + 2), form: map(k + 3), base_low: map(k + 4) };
        let (low, mid, deep) = (layer(0), layer(5), layer(10));
        let wind = [map(15), map(16), map(17)];
        let (high, rain, rain_base, rain_fall) = (Arc::new(map(18)), map(19), map(20), map(21));
        drop(values);

        // Second pass: what reaches out from the convective cells. The anvils' shield, spread
        // downwind of clusters of storms; and the reach of the cells themselves, for bounds.
        let reach = self.params.cell_size * 1.25 / self.params.radius;
        let stencil: Vec<(f64, f64)> = std::iter::once((0., 0.)).chain((0..12).flat_map(|k| {
            let a = TAU * k as f64 / 12.;
            [(a.cos() * reach, a.sin() * reach), (a.cos() * reach * 0.5, a.sin() * reach * 0.5)]
        })).collect();
        let second: Vec<[f32; 6]> = dirs.par_iter().map(|d| {
            let (t1, t2) = tangents(d);
            let at = |x: f64, y: f64| (d + t1 * x + t2 * y).normalize();
            let (mut cover, mut top, mut low_base, mut high_base, mut low_top) = (0f64, f64::MIN, f64::MAX, f64::MIN, f64::MAX);
            for (x, y) in &stencil {
                let s = at(*x, *y);
                cover = cover.max(deep.cover.at(&s));
                top = top.max(deep.top.at(&s));
                low_top = low_top.min(deep.top.at(&s));
                low_base = low_base.min(deep.base.at(&s));
                high_base = high_base.max(deep.base.at(&s));
            }
            // Upwind, the storms whose anvils are carried here.
            let wv = Vector3::new(wind[0].at(d), wind[1].at(d), wind[2].at(d));
            let speed = wv.norm();
            let mut anvil = 0f64;
            if speed > 1e-6 {
                for k in 1..=6 {
                    let back = (d - wv / speed * (k as f64 * 30e3 / self.params.radius)).normalize();
                    let c = deep.cover.at(&back);
                    anvil = anvil.max(smoothstep(0.3, 0.75, c) * (1. - 0.12 * k as f64));
                }
            }
            anvil = anvil.max(smoothstep(0.35, 0.8, deep.cover.at(d)));
            [cover as f32, top as f32, -low_base as f32, high_base as f32, anvil as f32, -low_top as f32]
        }).collect();
        let map2 = |k: usize| CubeMap::from_corners(n, second.iter().map(|v| v[k]).collect());
        let maps = Maps {
            deep_reach: map2(0), deep_top_reach: map2(1), deep_base_low: map2(2), deep_base_high: map2(3), anvil: map2(4), deep_top_low: map2(5),
            low, mid, deep, wind, high, rain, rain_base, rain_fall,
        };
        println!("Weather: {} systems, maps of {} texels in {:.1?}", self.systems.len(), 6 * n * n, start.elapsed());
        maps
    }

    /// A satellite's view of the clouds over the whole planet (equirectangular, `width` wide),
    /// with the fronts drawn on, to see the weather before rendering it.
    pub fn write_map(&self, path: &str, width: usize) {
        use rayon::prelude::*;
        let maps = self.maps();
        let (w, h) = (width, width / 2);
        let pixels: Vec<[u8; 3]> = (0..w * h).into_par_iter().map(|k| {
            let (x, y) = (k % w, k / w);
            let lon = (x as f64 + 0.5) / w as f64 * TAU - PI;
            let lat = PI / 2. - (y as f64 + 0.5) / h as f64 * PI;
            let g = Weather::geo(lat.to_degrees(), lon.to_degrees());
            let s = self.frame.to_scene(&g);
            let ground: [f64; 3] = if self.is_sea(&g) { [0.02, 0.06, 0.16] } else {
                match self.ground.as_ref().and_then(|gr| gr.climate.as_ref()) {
                    Some(c) => crate::terrain::climate::Biome::of(&c.local(&g, self.ground.as_ref().unwrap().planet.regional_elevation(&g).max(0.))).albedo(),
                    None => [0.12, 0.11, 0.07],
                }
            };
            let opacity = |cover: f64, k: f64| (cover * k).clamp(0., 1.);
            let clear = (1. - opacity(maps.low.cover.at(&s), 0.75)) * (1. - opacity(maps.mid.cover.at(&s), 0.85))
                * (1. - opacity(maps.deep.cover.at(&s), 0.95)) * (1. - opacity(maps.high.at(&s), 0.4))
                * (1. - opacity(maps.anvil.at(&s), 0.7));
            let c = |k: usize| {
                let v = ground[k] * 1.8 * clear + 0.92 * (1. - clear);
                (v.clamp(0., 1.).powf(1. / 2.2) * 255.) as u8
            };
            [c(0), c(1), c(2)]
        }).collect();
        let mut img = image::RgbImage::new(w as u32, h as u32);
        for (k, p) in pixels.iter().enumerate() {
            img.put_pixel((k % w) as u32, (k / w) as u32, image::Rgb(*p));
        }
        // The fronts: warm red, cold blue, occluded purple; lows and highs as dots.
        let mut plot = |g: &Vector3<f64>, colour: [u8; 3], size: i64| {
            let lat = g.y.clamp(-1., 1.).asin();
            let lon = (-g.z).atan2(g.x);
            let x = ((lon + PI) / TAU * w as f64) as i64;
            let y = ((PI / 2. - lat) / PI * h as f64) as i64;
            for dy in -size..=size {
                for dx in -size..=size {
                    let (px, py) = ((x + dx).rem_euclid(w as i64), y + dy);
                    if py >= 0 && py < h as i64 {
                        img.put_pixel(px as u32, py as u32, image::Rgb(colour));
                    }
                }
            }
        };
        for s in self.systems.iter().filter(|s| s.intensity > 0.15) {
            let colour = match s.kind { Kind::Cyclone => [200, 30, 30], Kind::Anticyclone => [30, 60, 200], Kind::Tropical => [230, 120, 0] };
            plot(&s.centre, colour, 3);
            for (k, f) in s.fronts.iter().enumerate() {
                let colour = [[220, 40, 40], [40, 90, 230], [160, 50, 200]][k.min(2)];
                for pair in f.points.windows(2) {
                    for t in 0..8 {
                        let v = pair[0] + (pair[1] - pair[0]) * (t as f64 / 8.);
                        let arc = v.norm() / self.params.radius;
                        let dir = if v.norm() > 0. { s.to_geo(&(v / v.norm())) } else { Vector3::zeros() };
                        plot(&(s.centre * arc.cos() + dir * arc.sin()), colour, 0);
                    }
                }
            }
        }
        // And the scene's site.
        plot(&self.frame.up, [255, 255, 0], 2);
        img.save(path).unwrap_or_else(|e| panic!("Can't write the weather map {}: {}", path, e));
        println!("Weather: wrote a map to {}", path);
    }
}

/// `g` turned east by `angle` radians about the geographic axis (+y): longitude increases
/// towards -z from +x (`Domain::frame`), so east is y × g.
pub(crate) fn rotate_about_y(g: &Vector3<f64>, angle: f64) -> Vector3<f64> {
    rotate(g, &Vector3::y(), angle)
}

/// `v` turned by `angle` radians about the unit `axis`, right-handed (Rodrigues).
pub(crate) fn rotate(v: &Vector3<f64>, axis: &Vector3<f64>, angle: f64) -> Vector3<f64> {
    let (s, c) = angle.sin_cos();
    v * c + axis.cross(v) * s + axis * axis.dot(v) * (1. - c)
}

/// Two unit vectors perpendicular to `d` and each other.
pub(crate) fn tangents(d: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if d.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let t1 = d.cross(&a).normalize();
    (t1, d.cross(&t1))
}

/// The weather for a scene object's description, built once per process and shared by the
/// layers it renders as.
pub fn weather(key: &str, build: impl FnOnce() -> Weather) -> Arc<Weather> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<Weather>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();
    if let Some(w) = map.get(key) {
        return w.clone();
    }
    let w = Arc::new(build());
    map.insert(key.to_string(), w.clone());
    w
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weather(seed: u64) -> Weather {
        let mut p = WeatherParams::new(seed);
        p.map_size = 32;
        Weather::new(p, None)
    }

    #[test]
    fn test_frame_round_trips() {
        let f = GeoFrame::new(50., -20.);
        assert!((f.to_geo(&Vector3::y()) - f.up).norm() < 1e-12);
        let s = Vector3::new(0.3, -0.5, 0.8).normalize();
        assert!((f.to_scene(&f.to_geo(&s)) - s).norm() < 1e-12);
        // The geographic north pole is up and away from the viewer (-z) at the site.
        let pole = f.to_scene(&Vector3::y());
        assert!(pole.z < 0. && pole.y > 0.);
    }

    #[test]
    fn test_systems_are_placed_by_seed() {
        let (a, b) = (weather(1), weather(2));
        assert!(!a.systems.is_empty());
        let centres = |w: &Weather| w.systems.iter().map(|s| s.centre).collect::<Vec<_>>();
        assert_eq!(centres(&a), centres(&weather(1)));
        assert_ne!(centres(&a), centres(&b));
        // Cyclones in the storm tracks, highs in the subtropics, both hemispheres.
        for s in &a.systems {
            let lat = s.centre.y.asin().to_degrees().abs();
            match s.kind {
                Kind::Cyclone => assert!((25. ..=75.).contains(&lat)),
                Kind::Anticyclone => assert!((15. ..=50.).contains(&lat)),
                Kind::Tropical => assert!((10. ..=25.).contains(&lat)),
            }
        }
        assert!(a.systems.iter().any(|s| s.centre.y > 0.) && a.systems.iter().any(|s| s.centre.y < 0.));
    }

    #[test]
    fn test_warm_front_sequence() {
        // Approaching a warm front from the cold side: cirrus far ahead, then the mid cloud's
        // base lowering, and rain only near the front.
        let mut p = WeatherParams::new(3);
        p.cyclones = 0;
        p.anticyclones = 0;
        p.tropical_cyclones = 0;
        p.systems = vec![SystemSpec { kind: Kind::Cyclone, lat_lon: Some((50., 0.)), offset: None, radius: Some(1.2e6),
            intensity: Some(1.), stage: Some(0.3), tilt: Some(0.), eye: None, velocity: None }];
        let w = Weather::new(p, None);
        let s = &w.systems[0];
        // Points ahead of (poleward of) the middle of the warm front, at increasing distances.
        let warm = &s.fronts[0];
        let mid = warm.points[12];
        let normal = { let t = warm.points[13] - warm.points[11]; Vector2::new(-t.y, t.x).normalize() };
        let at = |d: f64| {
            let v = mid + normal * d;
            let arc = v.norm() / w.params.radius;
            w.column(&(s.centre * arc.cos() + s.to_geo(&(v / v.norm())) * arc.sin()))
        };
        let (near, middle, far) = (at(100e3), at(450e3), at(1000e3));
        assert!(near.mid.cover > 0.5 && middle.mid.cover > 0.3, "{:?} {:?}", near.mid, middle.mid);
        assert!(near.mid.base < middle.mid.base, "{} {}", near.mid.base, middle.mid.base);
        assert!(far.high > 0.3 && far.mid.cover < near.mid.cover, "{} {}", far.high, far.mid.cover);
        assert!(near.rain > 0. && near.rain > far.rain);
    }

    #[test]
    fn test_columns_are_sane() {
        let w = weather(5);
        let mut rng = Pcg32::new(1, 0);
        let mut deep_tropics = 0.;
        let mut deep_poles = 0.;
        for _ in 0..3000 {
            let g = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let c = w.column(&g);
            for l in [&c.low, &c.mid, &c.deep] {
                assert!((0. ..=1.).contains(&l.cover) && l.top > l.base, "{:?}", l);
                assert!((0. ..=1.).contains(&l.form));
            }
            assert!(c.deep.top <= c.tropopause + 1. && c.mid.top <= c.tropopause);
            assert!((0. ..=1.).contains(&c.high) && c.rain >= 0.);
            let lat = g.y.asin().to_degrees().abs();
            if lat < 15. { deep_tropics += c.deep.cover; }
            if lat > 65. { deep_poles += c.deep.cover; }
        }
        assert!(deep_tropics > 5. * deep_poles, "{} {}", deep_tropics, deep_poles);
    }

    #[test]
    fn test_maps_match_columns() {
        let w = weather(6);
        let maps = w.maps();
        let d = crate::planet::CubeSphere { size: 32 }.direction(2, 7., 11.);
        let c = w.column(&w.frame.to_geo(&d));
        assert!((maps.low.cover.at(&d) - c.low.cover).abs() < 1e-5);
        assert!((maps.mid.top.at(&d) - c.mid.top).abs() < 1e-2);
        assert!((maps.mid.base_low.at(&d) + c.mid.base).abs() < 1e-2);
        assert!(maps.deep_reach.at(&d) >= c.deep.cover - 1e-6);
    }

    fn at_time(seed: u64, time: f64) -> Weather {
        let mut p = WeatherParams::new(seed);
        p.map_size = 32;
        p.time = time;
        Weather::new(p, None)
    }

    #[test]
    fn test_systems_move_and_live() {
        // Over a few minutes each random system moves a little, and the count never changes.
        let (a, b) = (at_time(4, 1e5), at_time(4, 1e5 + 600.));
        assert_eq!(a.systems.len(), b.systems.len());
        for (x, y) in a.systems.iter().zip(&b.systems) {
            let moved = (x.centre - y.centre).norm() * a.params.radius;
            let speed = match x.kind { Kind::Cyclone => 12.1, Kind::Anticyclone => 3., Kind::Tropical => 5.4 };
            assert!((moved / 600. - speed).abs() < 0.2 * speed + 0.5, "{:?}: {} m/s", x.kind, moved / 600.);
            assert!((x.intensity - y.intensity).abs() < 0.02 && (x.stage - y.stage).abs() < 0.01);
        }
        // Cyclones drift east.
        let c = a.systems.iter().position(|s| s.kind == Kind::Cyclone).unwrap();
        let east = Vector3::y().cross(&a.systems[c].centre).normalize();
        assert!((b.systems[c].centre - a.systems[c].centre).dot(&east) > 0.);
        // A scene's cyclone occludes and fills.
        let spec = SystemSpec { kind: Kind::Cyclone, lat_lon: Some((50., 0.)), offset: None, radius: None, intensity: Some(1.),
            stage: Some(0.2), tilt: None, eye: None, velocity: None };
        let with = |t: f64| { let mut p = WeatherParams::new(4); p.time = t; p.systems = vec![spec.clone()]; Weather::new(p, None).systems[0].clone() };
        let (young, old, gone) = (with(0.), with(2. * DAY), with(6. * DAY));
        assert!(old.stage > young.stage && old.intensity == 1. && gone.intensity < 0.05);
    }

    #[test]
    fn test_weather_changes_smoothly() {
        // Seconds apart, at any time (across slots' rebirths too), columns barely change;
        // days apart they're another sky.
        let mut rng = Pcg32::new(9, 0);
        let points: Vec<Vector3<f64>> = (0..3000).map(|_| crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64())).collect();
        let change = |t1: f64, t2: f64| {
            let (a, b) = (at_time(3, t1), at_time(3, t2));
            points.iter().map(|g| {
                let (x, y) = (a.column(g), b.column(g));
                (x.low.cover - y.low.cover).abs().max((x.mid.cover - y.mid.cover).abs()).max((x.deep.cover - y.deep.cover).abs()).max((x.high - y.high).abs())
            }).fold(0., f64::max)
        };
        // (The steepest thresholds change some points by a few hundredths a minute, and a tenth
        // as much in six seconds: steep, but no jumps.)
        for t in [0., 1.3 * DAY, 3.7 * DAY, 9.1 * DAY] {
            let step = change(t, t + 6.);
            assert!(step < 0.015, "at {}: {}", t, step);
        }
        assert!(change(0., 2. * DAY) > 0.5);
    }

    #[test]
    fn test_season_advances_without_a_sun() {
        let at = |t: f64| { let mut p = WeatherParams::new(1); p.day_of_year = 80.; p.time = t; Weather::new(p, None) };
        let (a, b) = (at(0.), at(30. * DAY));
        assert!((a.declination - b.declination).abs() > 0.05);
    }
}
