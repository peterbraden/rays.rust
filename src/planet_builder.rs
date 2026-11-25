//! # A planet built while you watch
//!
//! The scene builder's Planet page (`web/planet.js`) shows a world taking shape from a seed:
//! the plates and the climate first (`world::build`'s first stages, the same a full-scale
//! `landscape` with that seed goes through), then the whole planet's landscape evolving, a
//! map drawn after every so many years. This is the library's side of it: a `PlanetBuild`
//! holds the planet and its simulation (`terrain::simulation`, on the cube-sphere a
//! landscape's `"planet": {"cells"}` asks for), advances it by whole steps, and draws it.
//!
//! The simulation is the one `world::build` runs as its `planet` stage, with the same
//! parameters (a landscape object's, from `terrain_params`), so a planet built here to some
//! years is the planet stage of a landscape with that seed and `time`, step for step: steps
//! are only taken whole, and the run is resumable in time, so advancing 10 000 years ten
//! times is advancing 100 000 once.
//!
//! It keeps nothing on disk (a browser has no disk to keep it on), and holds the state in
//! memory between calls, so a host can show each stretch of time as it is reached.

use crate::terrain::climate::Climate;
use crate::terrain::planet::Planet;
use crate::terrain::simulation::{Terrain, TerrainParams, ICE_FREE};
use crate::world::{self, Description, Focus, Progress, Site, Stopped};
use serde_derive::Serialize;
use std::sync::Arc;

/// Cells per cube face edge of the planet's simulation, unless asked otherwise: a landscape's
/// default (`"planet": {"cells"}`).
pub const CELLS: usize = 64;

/// What a map shows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum View {
    /// Shaded relief: hypsometric tints, sea, ice, rivers, sediment (`map::relief_image`).
    Relief,
    /// The rock at the surface (`map::geology_image`).
    Geology,
    /// The rain of the last step (`map::rain_image`).
    Rain,
    /// The planet before erosion, with its plates' boundaries (`map::planet_image`).
    Plates,
    /// The climate's biomes (`map::climate_image`).
    Climate,
}

impl View {
    pub const NAMES: [&'static str; 5] = ["relief", "geology", "rain", "plates", "climate"];

    pub fn parse(name: &str) -> Option<View> {
        Some(match name {
            "relief" => View::Relief,
            "geology" => View::Geology,
            "rain" => View::Rain,
            "plates" => View::Plates,
            "climate" => View::Climate,
            _ => return None,
        })
    }
}

/// A few numbers about the planet as it stands.
#[derive(Clone, Debug, Serialize)]
pub struct Stats {
    /// Years evolved.
    pub time: f64,
    /// The sea's level, metres (it falls with the ice ages: `Terrain::sea_level`).
    pub sea_level: f64,
    /// Shares of the surface, by area: above the sea, and under ice.
    pub land: f64,
    pub ice: f64,
    /// The highest ground and the deepest sea floor, metres.
    pub highest: f64,
    pub deepest: f64,
    /// Mean rain over land, mm a year.
    pub rain: f64,
    /// Cells in the simulation.
    pub cells: usize,
}

/// A planet and its landscape's evolution so far.
pub struct PlanetBuild {
    pub planet: Planet,
    pub climate: Option<Arc<Climate>>,
    pub terrain: Terrain,
}

/// A map: RGBA, top row first, north up, an equirectangular projection.
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

/// The parameters of a full-scale landscape with this seed (`terrain::object::terrain_params`:
/// plate tectonics and a climate, on the Earth's radius).
pub fn params(seed: u64) -> TerrainParams {
    let mut p = crate::terrain::object::terrain_params(&serde_json::json!({ "seed": seed }));
    p.planet.radius = crate::planet::EARTH_RADIUS;
    p
}

/// The world `world::build` builds for this: its planet's stages, and no patches.
pub fn description(params: TerrainParams, cells: usize) -> Description {
    Description {
        params, site: Site::At { lat: 0., lon: 0. }, time: 0., planet_cells: Some(cells.max(2)),
        levels: vec![], focus: Focus::Point(crate::na::Vector3::zeros()), exaggeration: 1., close: None,
    }
}

impl PlanetBuild {
    /// Build the planet `params` describe, on `cells` cells a cube face edge, to time zero: its
    /// plates, its climate and its landscape before erosion. `on_progress` hears how the
    /// stages go, and returns whether to carry on.
    pub fn new(params: TerrainParams, cells: usize, on_progress: &mut dyn FnMut(&Progress) -> bool) -> Result<PlanetBuild, Stopped> {
        let w = world::build(&description(params, cells), None, on_progress)?;
        let terrain = w.globe.expect("a planet stage was asked for");
        Ok(PlanetBuild { planet: w.planet, climate: w.climate, terrain })
    }

    /// Years evolved so far.
    pub fn time(&self) -> f64 { self.terrain.time }

    /// Years a step of the simulation takes.
    pub fn step(&self) -> f64 { self.terrain.params.step }

    /// Advance by about `years`: to the next whole number of steps at least that far (one
    /// step at the least). `on_step` is told the time after each step, and returns whether to
    /// carry on; a stop leaves the planet at a whole step, from which it can be advanced again
    /// to the same states. Returns the time reached.
    pub fn advance(&mut self, years: f64, on_step: &mut dyn FnMut(f64) -> bool) -> f64 {
        let step = self.step();
        let steps = (years / step).ceil().max(1.);
        let target = ((self.terrain.time / step).round() + steps) * step;
        self.terrain.run_while(target, &mut |t| on_step(t.time));
        self.terrain.time
    }

    /// A map of the planet, `width` pixels wide and half as high.
    pub fn map(&self, view: View, width: usize) -> Image {
        use crate::terrain::map;
        let width = width.max(8);
        let img = match (view, &self.climate) {
            (View::Relief, _) => map::relief_image(&self.terrain, width),
            (View::Geology, _) => map::geology_image(&self.terrain, width),
            (View::Rain, _) => map::rain_image(&self.terrain, width),
            (View::Plates, _) => map::planet_image(&self.planet, width),
            (View::Climate, Some(c)) => map::climate_image(&self.planet, c, width),
            (View::Climate, None) => map::rain_image(&self.terrain, width),
        };
        let (w, h) = (img.width() as usize, img.height() as usize);
        let mut rgba = Vec::with_capacity(w * h * 4);
        for p in img.pixels() { rgba.extend_from_slice(&[p[0], p[1], p[2], 255]); }
        Image { width: w, height: h, rgba }
    }

    /// The numbers of `Stats`, as the planet stands.
    pub fn stats(&self) -> Stats {
        let t = &self.terrain;
        let level = t.sea_level();
        let outlets = t.outlets();
        let surface = t.surface();
        let total: f64 = t.domain.area.iter().sum();
        let (mut land, mut ice, mut rain) = (0., 0., 0.);
        let (mut highest, mut deepest) = (f64::NEG_INFINITY, f64::INFINITY);
        for i in 0..t.domain.len() {
            let a = t.domain.area[i];
            let sea = outlets[i] && t.bed[i] < level;
            if sea {
                deepest = deepest.min(t.bed[i]);
            } else {
                land += a;
                rain += a * t.precipitation.get(i).copied().unwrap_or(0.);
            }
            if t.ice[i] >= ICE_FREE { ice += a; }
            highest = highest.max(surface[i]);
        }
        Stats {
            time: t.time, sea_level: level, land: land / total, ice: ice / total, highest,
            deepest: if deepest.is_finite() { deepest } else { level },
            rain: if land > 0. { rain / land } else { 0. }, cells: t.domain.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::climate::ClimateParams;
    use crate::terrain::tectonics::TectonicsParams;

    /// A planet small enough to build in a second or two.
    fn tiny(seed: u64) -> TerrainParams {
        let mut p = params(seed);
        p.planet.tectonics = Some(TectonicsParams { cells: 12, age: 20., ..TectonicsParams::default() });
        p.climate = Some(ClimateParams { cells: 8, ..ClimateParams::default() });
        p.step = 500.;
        p
    }

    #[test]
    fn test_advancing_in_stretches_is_advancing_at_once() {
        let mut a = PlanetBuild::new(tiny(3), 6, &mut |_| true).unwrap();
        let mut b = PlanetBuild::new(tiny(3), 6, &mut |_| true).unwrap();
        for _ in 0..4 { a.advance(1000., &mut |_| true); }
        b.advance(4000., &mut |_| true);
        assert_eq!(a.time(), 4000.);
        assert_eq!(a.terrain.bed, b.terrain.bed);
    }

    #[test]
    fn test_a_stretch_is_whole_steps() {
        let mut a = PlanetBuild::new(tiny(1), 6, &mut |_| true).unwrap();
        assert_eq!(a.advance(1., &mut |_| true), 500.);
        assert_eq!(a.advance(700., &mut |_| true), 1500.);
        // Stopped after a step: still on the steps.
        assert_eq!(a.advance(5000., &mut |t| t < 2000.), 2000.);
    }

    #[test]
    fn test_maps_and_stats() {
        let a = PlanetBuild::new(tiny(2), 6, &mut |_| true).unwrap();
        for name in View::NAMES {
            let m = a.map(View::parse(name).unwrap(), 64);
            assert_eq!((m.width, m.height, m.rgba.len()), (64, 32, 64 * 32 * 4), "{}", name);
        }
        let s = a.stats();
        assert!(s.land > 0. && s.land < 1., "{:?}", s);
        assert!(s.highest > s.deepest, "{:?}", s);
    }

    /// How long a full-size planet takes: its stages, a step and a map (ignored; run with
    /// `--ignored --nocapture`, `RAYS_PLANET_CELLS=` for another size).
    #[test]
    #[ignore]
    fn measure_planet_build() {
        let cells = std::env::var("RAYS_PLANET_CELLS").ok().and_then(|c| c.parse().ok()).unwrap_or(CELLS);
        let begun = web_time::Instant::now();
        let mut a = PlanetBuild::new(params(0), cells, &mut |e| { if let Some(m) = &e.message { println!("{}", m); } true }).unwrap();
        println!("built in {:.1?}", begun.elapsed());
        let begun = web_time::Instant::now();
        a.advance(10. * a.step(), &mut |_| true);
        println!("10 steps of {} years in {:.1?}", a.step(), begun.elapsed());
        for view in [View::Relief, View::Plates] {
            let begun = web_time::Instant::now();
            a.map(view, 1024);
            println!("{:?} map in {:.1?}", view, begun.elapsed());
        }
        println!("{:?}", a.stats());
    }
}
