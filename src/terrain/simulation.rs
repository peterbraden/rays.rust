//! # Landscape evolution through time
//!
//! A `Terrain` is the state of a planet (or a patch of one) at a time t, in years: bedrock
//! elevation, ice thickness, and what the processes last computed (drainage area, sliding
//! speed). At t = 0 it is the noise topography of `planet`. `run_to(t)` advances it by fixed
//! steps, each applying, in turn (operator splitting, as in Braun et al. 1999's ICE-CASCADE and
//! FastScape):
//!
//! 1. tectonic uplift (`planet`), and, in a mountain belt, the rock carried sideways towards
//!    the foreland with its folds (`carry`);
//! 2. volcanic eruptions: cones and lava flows (`volcano`);
//! 3. glaciers: mass balance, ice flow and glacial erosion, in sub-steps (`glacier`), the
//!    eroded rock leaving in meltwater as sediment for the rivers (Hallet, Hunter and Bogen,
//!    "Rates of erosion and sediment evacuation by glaciers", Global and Planetary Change 12,
//!    1996, estimate glacial erosion rates from that sediment);
//! 4. rivers cutting back the outer banks of their bends (`fluvial::lateral_erosion`), and
//!    flow routing over the ice and rock surface, gathering the rain the terrain makes
//!    (`orographic`: discharge, not area, drives the rivers; glaciers' snow follows it too), then river erosion and deposition on
//!    ice-free ground (`flow`, `fluvial`);
//! 5. deltas: what rivers bring to the sea spreads over the sea floor (`marine`);
//! 6. limestone dissolving (`dissolve`; its rain also sinks, to rise at springs, in routing),
//!    landslides where slopes are steeper than their rock holds (`fluvial::landslide`), and
//!    hillslope diffusion (`fluvial`), faster on weak rock and sediment;
//! 7. meandering rivers, a sub-grid model drawn over the terrain (`channel`);
//! 8. the crust's isostatic response to what erosion, deposition and ice have changed of its
//!    load (`isostasy`): it rises as rock is taken off and sinks under ice.
//!
//! ## Geology
//!
//! The ground is layered. At the bottom is bedrock (`geology`): a folded sedimentary cover of
//! hard and soft beds over crystalline basement, with granite plutons, each cell's structure
//! fixed at time zero (`structure`). On it lies volcanic rock (`lava`), and on top sediment
//! (`sediment`): alluvium, lake fill, colluvium from hillslopes. Ground gained by deposition or
//! creep is sediment; ground lost is taken from the sediment, then the lava, then the bedrock,
//! so erosion cuts through the layers in order. Uplift moves all layers together, and carries
//! the bedrock up through its column (`exhumed`), so erosion that keeps pace with it cuts into
//! ever deeper rock.
//!
//! Rivers and glaciers erode whatever is exposed, each with its own erodibility for each rock
//! (`erodibility`, `glacial_erodibility`; `geology::Rock::erodibility`): glaciers quarry
//! jointed rock such as basalt and quartzite that rivers can hardly cut, and leave massive
//! granite standing. Sediment is 3 times as erodible as average bedrock to both; lava 0.3 times
//! to rivers but 1.5 times to glaciers (model choices: fresh basalt is strong, but columnar
//! joints are easily quarried). Hillslopes creep in proportion to the rivers' erodibility too
//! (weak rock weathers faster into soil that creeps), so slopes on shale relax while hard beds
//! hold cliffs. In the mountain belts the folds' anticlines are uplifted faster
//! than their synclines (`GeologyParams::fold_growth`). Slopes steeper than their rock holds
//! fail in landslides (`slide`), but not under ice, which buttresses them; and at time zero the
//! noise topography is first relaxed to what its rock holds.

//! Everything is a deterministic function of the parameters (including the seed) and t, and
//! the state at t2 is the state at t1 advanced by t2 - t1. So an animation renders successive
//! times, carrying one `Terrain` forwards rather than re-running from zero for every frame.
//! Steps of different lengths give (slightly) different landscapes, so the step length is a
//! parameter, not chosen from t.
//!
//! ## Morphological acceleration
//!
//! Ice must be stepped every few years, but landscapes change over tens of thousands. Coastal
//! morphodynamics faces the same mismatch between fast hydrodynamics and slow bed change, and
//! the standard remedy is a morphological acceleration factor, MORFAC (Roelvink, "Coastal
//! morphodynamic evolution techniques", Coastal Engineering 53, 2006,
//! doi:10.1016/j.coastaleng.2005.10.015): run the fast process for Δt / M and apply its bed
//! changes multiplied by M. Here, in each landscape step of Δt years, ice flows for Δt / M years
//! and erodes as if for Δt, while uplift, rivers, hillslopes and the climate run in landscape
//! time (`GlacierParams::acceleration`, M = 10 by default). It is valid while the glaciers
//! adjust to the climate faster than the climate changes in ice time: a 100,000 year glacial
//! cycle lasts 10,000 years of ice time at M = 10, while glaciers respond within centuries. It
//! is not valid for following the ice itself in an animation (glaciers advance M times too
//! slowly relative to the climate); set M = 1 for that, at M times the cost.
//!
//! The sea and, for a patch, its edge are base level: their cells are held fixed and drain
//! water and ice. Sea level is 0 and constant (no eustatic change with the ice ages, yet).

use crate::terrain::domain::{Domain, Layout};
use crate::terrain::flow::{route, Flow};
use crate::terrain::fluvial::{diffuse_varying, erode_and_deposit, landslide, lateral_erosion, FluvialParams};
use crate::terrain::glacier::{erode, flow_ice, GlacierParams};
use crate::terrain::planet::{Planet, PlanetParams};
use crate::terrain::volcano::{erupt, vents, Vent, VolcanoParams};
use crate::terrain::marine::{self, MarineParams};
use crate::terrain::channel::{ChannelParams, Rivers};
use crate::terrain::fluvial::LAKE;
use crate::terrain::geology::{fractured, Process, Rock, Structure};
use rayon::prelude::*;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct TerrainParams {
    pub planet: PlanetParams,
    pub fluvial: FluvialParams,
    /// None to disable glaciers.
    pub glacier: Option<GlacierParams>,
    /// None to disable volcanoes.
    pub volcano: Option<VolcanoParams>,
    /// None to let sediment reaching the sea leave the model.
    pub marine: Option<MarineParams>,
    /// None for no meandering rivers (they are only modelled on patches).
    pub channels: Option<ChannelParams>,
    /// The climate, from latitude (`climate`): it sets the glaciers' snowline cell by cell. None
    /// for the glaciers' own snowline by latitude.
    pub climate: Option<crate::terrain::climate::ClimateParams>,
    /// The crust's isostatic response to erosion, deposition and ice (`isostasy`); None for
    /// none.
    pub isostasy: Option<crate::terrain::isostasy::IsostasyParams>,
    /// Rain made by the terrain (`orographic`), which the rivers gather and the glaciers'
    /// snow follows; None for rain everywhere the climate's (or uniform without a climate).
    pub orographic: Option<crate::terrain::orographic::OrographicParams>,
    /// Erodibility of sediment, relative to average bedrock.
    pub sediment_erodibility: f64,
    /// Erodibility of volcanic rock, relative to average bedrock.
    pub lava_erodibility: f64,
    /// Erodibility of sediment by glaciers, relative to average bedrock.
    pub sediment_glacial_erodibility: f64,
    /// Erodibility of volcanic rock by glaciers, relative to average bedrock.
    pub lava_glacial_erodibility: f64,
    /// Length of a landscape evolution step, years.
    pub step: f64,
}

impl Default for TerrainParams {
    fn default() -> TerrainParams {
        TerrainParams {
            planet: PlanetParams::default(),
            fluvial: FluvialParams::default(),
            glacier: Some(GlacierParams::default()),
            volcano: Some(VolcanoParams::default()),
            marine: Some(MarineParams::default()),
            channels: Some(ChannelParams::default()),
            climate: None,
            orographic: Some(crate::terrain::orographic::OrographicParams::default()),
            isostasy: Some(crate::terrain::isostasy::IsostasyParams::default()),
            sediment_erodibility: 3.,
            lava_erodibility: 0.3,
            sediment_glacial_erodibility: 3.,
            lava_glacial_erodibility: 1.5,
            step: 100.,
        }
    }
}

/// Passes of landsliding per step: enough to bring slopes oversteepened in a step back to what
/// their rock holds, as failure is all but instant on a landscape's timescales.
const LANDSLIDE_PASSES: usize = 30;
/// ... and at time zero, to relax the noise topography to what its rock holds.
const INITIAL_LANDSLIDE_PASSES: usize = 2000;
const INITIAL_LANDSLIDE_ROUNDS: usize = 20;
/// The cell spacing the rocks' critical slopes are for, metres...
const LANDSLIDE_SPACING: f64 = 300.;
/// ... and how they steepen on finer grids: a model choice for the mild scale dependence of
/// slopes measured over different distances.
const LANDSLIDE_SCALING: f64 = 0.2;
/// Slopes may exceed their limit by this, metres over a link, before they fail.
const LANDSLIDE_TOLERANCE: f64 = 0.1;

/// Ice thinner than this (metres) counts as ice-free ground.
pub const ICE_FREE: f64 = 1.;

pub struct Terrain {
    pub domain: Domain,
    pub params: TerrainParams,
    pub planet: Planet,
    /// Years since the noise topography.
    pub time: f64,
    /// Elevation of the ground (the top of the sediment, lava and bedrock beneath any ice)
    /// above sea level, metres.
    pub bed: Vec<f64>,
    /// Thickness of sediment at the top of the ground, metres.
    pub sediment: Vec<f64>,
    /// Thickness of volcanic rock beneath the sediment, metres.
    pub lava: Vec<f64>,
    /// Sediment flux leaving each cell in rivers at the last step, m³/yr.
    pub sediment_flux: Vec<f64>,
    /// Ice thickness, metres.
    pub ice: Vec<f64>,
    /// Tectonic uplift rate, metres per year.
    pub uplift: Vec<f64>,
    /// The bedrock's structure under each cell (`geology`).
    pub structure: Vec<Structure>,
    /// How far uplift has carried each cell's rock up since time zero, metres (tectonic uplift
    /// and isostatic rebound).
    pub exhumed: Vec<f64>,
    /// How far the crust's isostatic response has moved each cell since time zero, metres.
    pub rebound: Vec<f64>,
    /// The belt's own uplift at each cell, metres per year, before the folds' growth (which
    /// moves with the rock, as the belt's doesn't).
    belt_uplift: Vec<f64>,
    /// How fast the rock moves sideways, towards the foreland, (east, north) m/yr: zero but on a
    /// patch in a mountain belt.
    pub velocity: (f64, f64),
    /// How far the rock has moved since time zero, (east, north) metres...
    pub displacement: (f64, f64),
    /// ... and by how many whole cells the ground has been moved with it, (east, north).
    pub shifted: (i64, i64),
    /// Detail of the folds down to this wavelength, metres.
    finest: f64,
    /// The ground at time zero, from which the load on the crust is reckoned.
    ground0: Vec<f64>,
    /// Cells that were sea at time zero. Ground eroded below sea level later is a lake bed
    /// (eg. a glacially overdeepened basin), not sea.
    pub sea: Vec<bool>,
    /// Latitude of each cell, radians.
    pub latitude: Vec<f64>,
    /// Drainage area at the last step, square metres.
    pub drainage: Vec<f64>,
    /// Where each cell drained at the last step.
    pub receiver: Vec<usize>,
    /// Basal sliding speed at the last step, metres per year.
    pub sliding: Vec<f64>,
    /// Ice flow steps taken so far (for performance statistics).
    pub ice_steps: usize,
    /// ... and the solver iterations they took.
    pub ice_iterations: usize,
    vents: Vec<Vent>,
    /// How fast a patch's edge cells rise or fall, metres per year: zero, or, for a patch nested
    /// in a coarser simulation, the change the coarser one went through there, so the patch's
    /// base level follows its surroundings.
    pub boundary_rate: Vec<f64>,
    /// Meandering rivers and ox-bow lakes.
    pub rivers: Rivers,
    /// Each cell's equilibrium line altitude in today's climate, if there is a climate model.
    pub ela: Option<Vec<f64>>,
    /// The climate's rain at each cell, mm/yr (`OrographicParams::reference` without one).
    pub base_precipitation: Vec<f64>,
    /// The rain at each cell at the last step, with what the terrain makes, mm/yr.
    pub precipitation: Vec<f64>,
    /// The prevailing wind over the domain, (eastward, northward).
    pub wind: (f64, f64),
}

impl Terrain {
    pub fn new(domain: Domain, mut params: TerrainParams) -> Terrain {
        params.planet.radius = domain.radius;
        let planet = Planet::new(params.planet.clone());
        // Detail down to about two cells.
        let finest = 2. * domain.min_spacing();
        let bed: Vec<f64> = domain.dirs.par_iter().map(|d| planet.elevation(d, finest)).collect();
        let structure: Vec<Structure> = domain.dirs.par_iter().map(|d| planet.structure(d, finest)).collect();
        // Folds still growing: anticlines rise faster than synclines.
        let growth = params.planet.geology.fold_growth;
        let belt_uplift: Vec<f64> = domain.dirs.iter().map(|d| planet.uplift(d)).collect();
        let uplift = belt_uplift.iter().zip(&structure).map(|(u, s)| u * (1. + growth * s.crest).max(0.)).collect();
        // Thrust sheets carry the rock towards the foreland, at the belt's strength.
        let velocity = match &domain.layout {
            Layout::Patch { up, east, north, .. } => {
                let o = planet.orogeny(up);
                let rate = params.planet.geology.shortening_rate;
                if o <= 1e-3 || rate == 0. { (0., 0.) } else {
                    let f = planet.trend().foreland(up);
                    let size = f.norm();
                    if size < 1e-9 { (0., 0.) } else {
                        let speed = rate * o.min(1.) * size.tanh();
                        (speed * f.dot(east) / size, speed * f.dot(north) / size)
                    }
                }
            }
            Layout::Sphere { .. } => (0., 0.),
        };
        let latitude = domain.dirs.iter().map(|d| d.y.clamp(-1., 1.).asin()).collect();
        let n = domain.len();
        let sea = bed.iter().map(|&b| b < 0.).collect();
        let vents = match &params.volcano { Some(v) => vents(&planet, &domain, v), None => vec![] };
        let climate = params.climate.as_ref().map(|c| crate::terrain::climate::climate(c, &planet));
        let ela = climate.as_ref().map(|climate| domain.dirs.iter().map(|d| climate.local(d, 0.).snowline(0.)).collect());
        let reference = params.orographic.as_ref().map_or(crate::terrain::orographic::OrographicParams::default().reference, |o| o.reference);
        let base_precipitation: Vec<f64> = match &climate {
            Some(c) => domain.dirs.iter().map(|d| c.local(d, 0.).precipitation).collect(),
            None => vec![reference; n],
        };
        // The prevailing wind at the domain's middle latitude.
        let centre: crate::na::Vector3<f64> = domain.dirs.iter().sum::<crate::na::Vector3<f64>>().normalize();
        let wind = crate::terrain::climate::wind(&params.climate.clone().unwrap_or_default(), centre.y.clamp(-1., 1.).asin());
        let mut t = Terrain {
            precipitation: base_precipitation.clone(), base_precipitation, wind,
            domain, params, planet, time: 0., ela, bed, uplift, structure, exhumed: vec![0.; n], rebound: vec![0.; n], ground0: vec![], belt_uplift, velocity, displacement: (0., 0.), shifted: (0, 0), finest, latitude, sea, vents, rivers: Rivers::default(),
            sediment: vec![0.; n], lava: vec![0.; n], sediment_flux: vec![0.; n],
            ice: vec![0.; n], boundary_rate: vec![0.; n], drainage: vec![0.; n], receiver: (0..n).collect(), sliding: vec![0.; n], ice_steps: 0, ice_iterations: 0,
        };
        // The noise topography may stand steeper than its rock can: it fails first, its debris
        // collecting in the valleys.
        let outlets = t.outlets();
        t.slide(&outlets, INITIAL_LANDSLIDE_PASSES, INITIAL_LANDSLIDE_ROUNDS);
        t.ground0 = t.bed.clone();
        t.rain();
        let flow = t.route();
        t.drainage = flow.area;
        t.receiver = flow.receiver;
        t
    }

    /// Cells that are base level: the sea, and a patch's edge. (A volcano that rises out of
    /// the sea becomes land.)
    pub fn outlets(&self) -> Vec<bool> {
        (0..self.domain.len()).map(|i| self.domain.boundary[i] || (self.sea[i] && self.bed[i] < 0.)).collect()
    }

    /// Elevation of the top of the bedrock, beneath sediment and lava, metres.
    pub fn bedrock(&self, i: usize) -> f64 {
        self.bed[i] - self.sediment[i] - self.lava[i]
    }

    /// The rock at elevation `z` under cell `i`, beneath any sediment and lava.
    pub fn bedrock_at(&self, i: usize, z: f64) -> Rock {
        let s = &self.structure[i];
        self.planet.column.rock(s, s.coordinate(z, self.exhumed[i]))
    }

    /// What is exposed at cell `i`: sediment, lava, or the bedrock.
    pub fn rock(&self, i: usize) -> Rock {
        if self.sediment[i] > 0.5 { Rock::Sediment }
        else if self.lava[i] > 0.5 { Rock::Basalt }
        else { self.bedrock_at(i, self.bedrock(i)) }
    }

    /// Erodibility of what is exposed at each cell by `process`, relative to average bedrock.
    pub fn erodibility_by(&self, process: Process) -> Vec<f64> {
        let p = &self.params;
        let contrast = p.planet.geology.contrast;
        (0..self.domain.len()).into_par_iter().map(|i| match (self.rock(i), process) {
            (Rock::Sediment, Process::Rivers) => p.sediment_erodibility,
            (Rock::Sediment, Process::Glaciers) => p.sediment_glacial_erodibility,
            (Rock::Basalt, Process::Rivers) => p.lava_erodibility,
            (Rock::Basalt, Process::Glaciers) => p.lava_glacial_erodibility,
            // Bedrock, more erodible where folding and faulting fractured it.
            (r, process) => r.erodibility(process, contrast) * fractured(self.structure[i].fracture, process, p.planet.geology.fracturing),
        }).collect()
    }

    /// Erodibility of what is exposed at each cell by rivers, relative to average bedrock.
    pub fn erodibility(&self) -> Vec<f64> { self.erodibility_by(Process::Rivers) }

    /// ... and by glaciers.
    pub fn glacial_erodibility(&self) -> Vec<f64> { self.erodibility_by(Process::Glaciers) }

    /// Update the layers for the change of ground from `before`: ground gained is sediment;
    /// ground lost is taken from the sediment, then the lava, then the bedrock.
    fn record(&mut self, before: &[f64]) {
        for (((b, b0), sediment), lava) in self.bed.iter().zip(before).zip(&mut self.sediment).zip(&mut self.lava) {
            let change = b - b0;
            if change >= 0. {
                *sediment += change;
            } else {
                let mut loss = -change;
                let s = loss.min(*sediment);
                *sediment -= s;
                loss -= s;
                *lava -= loss.min(*lava);
            }
        }
    }

    pub fn surface(&self) -> Vec<f64> {
        self.bed.iter().zip(&self.ice).map(|(b, h)| b + h).collect()
    }

    /// Each cell's rain relative to the rain the rivers' erodibility is for: what it adds to
    /// the discharge downstream.
    fn rain_factor(&self) -> Vec<f64> {
        match &self.params.orographic {
            Some(o) => self.precipitation.iter().map(|p| p / o.reference).collect(),
            None => vec![1.; self.domain.len()],
        }
    }

    /// Work out the rain over the present surface (`orographic`).
    fn rain(&mut self) {
        self.precipitation = match &self.params.orographic {
            Some(o) => crate::terrain::orographic::rain(&self.domain, &self.surface(), &self.outlets(), &self.base_precipitation, self.wind, o),
            None => self.base_precipitation.clone(),
        };
    }

    fn route(&self) -> Flow {
        let rain = self.rain_factor();
        let mut flow = route(&self.domain, &self.surface(), &self.outlets(), &rain);
        let g = &self.params.planet.geology;
        if g.infiltration > 0. && g.karst_rate >= 0. {
            // Rain on exposed limestone sinks, follows the drainage underground, and comes up
            // at the first cell downstream that isn't limestone: the rivers' discharge is only
            // what runs over the surface.
            let karst: Vec<bool> = (0..self.domain.len()).map(|i| self.ice[i] < ICE_FREE && self.rock(i) == Rock::Limestone).collect();
            if karst.iter().any(|&k| k) {
                let sinks = g.infiltration.clamp(0., 1.);
                let mut surface = vec![0.; self.domain.len()];
                let mut under = vec![0.; self.domain.len()];
                for &i in flow.stack.iter().rev() {
                    let water = rain[i] * self.domain.area[i];
                    let r = flow.receiver[i];
                    // (At an outlet, what is underground comes up too: it leaves the patch.)
                    if karst[i] && r != i {
                        surface[i] += water * (1. - sinks);
                        under[i] += water * sinks;
                    } else {
                        surface[i] += water + under[i];
                        under[i] = 0.;
                    }
                    if r != i {
                        surface[r] += surface[i];
                        under[r] += under[i];
                    }
                }
                flow.area = surface;
            }
        }
        flow
    }

    /// Lower exposed limestone by solution for `dt` years, in proportion to the rain; the rock
    /// leaves in solution, not as sediment.
    fn dissolve(&mut self, fixed: &[bool], dt: f64) {
        let rate = self.params.planet.geology.karst_rate;
        if rate <= 0. { return; }
        let before = self.bed.clone();
        for i in 0..self.domain.len() {
            if fixed[i] || self.ice[i] >= ICE_FREE || self.rock(i) != Rock::Limestone { continue; }
            self.bed[i] -= rate * self.precipitation[i] / 1000. * dt;
        }
        self.record(&before);
    }

    /// Advance to time `t` (no earlier than now).
    pub fn run_to(&mut self, t: f64) {
        self.run_while(t, &mut |_| true);
    }

    /// Advance to time `t`, or until `carry_on`, asked after each step, says to stop. The steps
    /// are the same either way, so a run stopped at a step and taken up again from there
    /// arrives at the same state, provided it stopped a whole number of `step`s from zero
    /// (as every stop here does but the last, at `t`).
    pub fn run_while(&mut self, t: f64, carry_on: &mut dyn FnMut(&Terrain) -> bool) {
        while self.time < t - 1e-9 {
            let dt = self.params.step.min(t - self.time);
            self.step(dt);
            if !carry_on(self) { break; }
        }
    }

    pub fn step(&mut self, dt: f64) {
        let fixed = self.outlets();
        for (i, (((b, x), u), f)) in self.bed.iter_mut().zip(&mut self.exhumed).zip(&self.uplift).zip(&fixed).enumerate() {
            let rise = if !f { u * dt } else if self.domain.boundary[i] { self.boundary_rate[i] * dt } else { 0. };
            *b += rise;
            *x += rise;
        }

        self.carry(dt, &fixed);
        self.rain();

        if let Some(v) = &self.params.volcano {
            erupt(&self.domain, &self.vents, v, self.time, dt, &mut self.bed, &mut self.lava, &fixed);
        }

        // Sediment glaciers erode is carried off by their meltwater, into the rivers.
        let mut glacial = vec![0.; self.domain.len()];
        if let Some(g) = self.params.glacier.clone() {
            glacial = self.glaciate(&g, &fixed, dt);
        }

        let flow = self.route();
        let surface = self.surface();
        // Lakes are where water stands over the surface, but not those dammed only by ice: a
        // glacier across a valley can hold up a lake kilometres deep, which would otherwise
        // trap the glacier's sediment to that depth.
        let ice_free = route(&self.domain, &self.bed, &fixed, &vec![1.; self.domain.len()]);
        let lake: Vec<f64> = (0..self.domain.len())
            .map(|i| (flow.filled[i] - surface[i]).min(ice_free.filled[i] - self.bed[i]).max(0.)).collect();
        let erodibility: Vec<f64> = self.erodibility().iter().zip(&self.ice)
            .map(|(&k, &h)| if h < ICE_FREE { k } else { 0. }).collect();
        // Rivers cut back the outer banks of their bends; what they take joins their sediment.
        let before = self.bed.clone();
        let lateral = lateral_erosion(&self.domain, &flow, &mut self.bed, &fixed, &erodibility, &lake, &self.params.fluvial, dt);
        self.record(&before);
        let source: Vec<f64> = glacial.iter().zip(&lateral).map(|(a, b)| a + b).collect();
        let before = self.bed.clone();
        self.sediment_flux = erode_and_deposit(&self.domain, &flow, &mut self.bed, &fixed, &erodibility, &lake, &source, &self.params.fluvial, dt);
        self.record(&before);
        if let Some(m) = &self.params.marine {
            // What the rivers bring to the sea builds deltas.
            let sea: Vec<bool> = (0..self.domain.len()).map(|i| fixed[i] && !self.domain.boundary[i]).collect();
            let supply: Vec<f64> = (0..self.domain.len()).map(|i| if sea[i] { self.sediment_flux[i] } else { 0. }).collect();
            let before = self.bed.clone();
            marine::deposit(&self.domain, &mut self.bed, &self.sediment, &sea, &supply, m, dt);
            self.record(&before);
        }
        self.slide(&fixed, LANDSLIDE_PASSES, 1);
        self.dissolve(&fixed, dt);

        // Soil creeps faster on weak rock, which weathers faster, and on loose sediment: slopes
        // on shale relax while hard beds hold cliffs.
        let kappa: Vec<f64> = self.erodibility().iter().map(|k| self.params.fluvial.diffusivity * k).collect();
        let before = self.bed.clone();
        diffuse_varying(&self.domain, &mut self.bed, &fixed, &kappa, dt);
        self.record(&before);

        if let Some(c) = &self.params.channels {
            let n = self.domain.len();
            let slope: Vec<f64> = (0..n).map(|i| {
                let r = flow.receiver[i];
                if r == i { 0. } else { (self.bed[i] - self.bed[r]).max(0.) / flow.receiver_dist[i] }
            }).collect();
            let wet: Vec<bool> = (0..n).map(|i| fixed[i] || self.ice[i] >= ICE_FREE || lake[i] > LAKE).collect();
            self.rivers.step(&self.domain, self.params.planet.seed, self.time, dt, &flow.receiver, &flow.area, &slope, &self.sediment, &wet, &self.bed, c);
        }
        self.receiver = flow.receiver.clone();
        self.drainage = flow.area;
        self.compensate(&fixed, dt);
        self.time += dt;
    }

    /// Let slopes steeper than their rock holds fail (`fluvial::landslide`), in up to `passes`
    /// passes a round, but not under ice, which buttresses them: walls that glaciers left steep
    /// fall in when it melts, unless the rock is strong. What falls is sediment where it lands.
    /// As failing exposes other rock (and buries some under debris), each of up to `rounds`
    /// rounds looks at the rock again, until nothing more fails.
    fn slide(&mut self, fixed: &[bool], passes: usize, rounds: usize) {
        let strength = self.params.planet.geology.slope_strength;
        if strength <= 0. { return; }
        for _ in 0..rounds {
            let critical: Vec<f64> = (0..self.domain.len()).map(|i| {
                if self.ice[i] >= ICE_FREE { return f64::INFINITY; }
                strength * self.critical_slope(i)
            }).collect();
            let before = self.bed.clone();
            let (moved, _) = landslide(&self.domain, &mut self.bed, fixed, &critical, LANDSLIDE_TOLERANCE, passes);
            self.record(&before);
            if moved == 0. { break; }
        }
    }

    /// The steepest slope cell `i` stands at before failing, rise over run: its rock's, less
    /// where folding fractured it, for this domain's cells. Slopes measured over longer
    /// distances are gentler, as they average cliffs with the benches between them, so the
    /// rocks' limits (for cells of `LANDSLIDE_SPACING`) are steeper on finer grids and gentler on
    /// coarser, as (`LANDSLIDE_SPACING` / spacing)^`LANDSLIDE_SCALING`; then nested patches of
    /// different spacings agree roughly on how steep the ground stands.
    pub fn critical_slope(&self, i: usize) -> f64 {
        let scale = (LANDSLIDE_SPACING / self.domain.min_spacing()).powf(LANDSLIDE_SCALING);
        scale * self.rock(i).critical_slope() / (1. + 0.3 * self.structure[i].fracture)
    }

    /// Carry the rock towards the foreland for `dt` years (`velocity`): the ground moves by
    /// whole cells as the displacement passes each (moved a fraction of a cell each step, by
    /// interpolation, it would be smoothed by far more than its hillslopes ever smooth it), and
    /// the bedrock's structure is that of the rock each cell now holds, from where it was at
    /// time zero, so rock comes in from the planet's geology beyond the patch's edge. Base level
    /// (`fixed`) stays where it is.
    fn carry(&mut self, dt: f64, fixed: &[bool]) {
        if self.velocity == (0., 0.) { return; }
        let Layout::Patch { spacing, .. } = self.domain.layout else { return };
        self.displacement.0 += self.velocity.0 * dt;
        self.displacement.1 += self.velocity.1 * dt;
        let target = ((self.displacement.0 / spacing).round() as i64, (self.displacement.1 / spacing).round() as i64);
        if target == self.shifted { return; }
        while self.shifted != target {
            let dx = (target.0 - self.shifted.0).signum();
            let dy = if dx == 0 { (target.1 - self.shifted.1).signum() } else { 0 };
            self.shift_cells(dx, dy, fixed);
            self.shifted.0 += dx;
            self.shifted.1 += dy;
        }
        self.place_structure();
    }

    /// Move the ground one cell east (`dx`) or north (`dy`), each cell but base level taking
    /// its upwind neighbour's (the edge's own where there is none).
    fn shift_cells(&mut self, dx: i64, dy: i64, fixed: &[bool]) {
        let Layout::Patch { n, .. } = self.domain.layout else { return };
        let n = n as i64;
        let source: Vec<usize> = (0..self.domain.len()).map(|k| {
            let (i, j) = (k as i64 % n, k as i64 / n);
            if fixed[k] { return k; }
            ((j - dy).clamp(0, n - 1) * n + (i - dx).clamp(0, n - 1)) as usize
        }).collect();
        for f in [&mut self.bed, &mut self.sediment, &mut self.lava, &mut self.ice, &mut self.exhumed, &mut self.rebound, &mut self.ground0, &mut self.sliding] {
            let old = f.clone();
            for (k, &s) in source.iter().enumerate() { f[k] = old[s]; }
        }
    }

    /// Where the rock now at cell `i` was at time zero.
    pub fn origin(&self, i: usize) -> crate::na::Vector3<f64> {
        let d = self.domain.dirs[i];
        let Layout::Patch { up, east, north, spacing, .. } = self.domain.layout else { return d };
        if self.shifted == (0, 0) { return d; }
        let (x, z) = Domain::tangent(up, east, north, self.domain.radius, &d);
        let (x, z) = (x - self.shifted.0 as f64 * spacing, z - self.shifted.1 as f64 * spacing);
        (up * self.domain.radius + east * x + north * z).normalize()
    }

    /// The structure of the rock each cell holds, and the uplift there: the belt's, where the
    /// cell is, and the folds' growth, which goes with the rock.
    fn place_structure(&mut self) {
        let origins: Vec<crate::na::Vector3<f64>> = (0..self.domain.len()).map(|i| self.origin(i)).collect();
        let (planet, finest) = (&self.planet, self.finest);
        self.structure = origins.par_iter().map(|d| planet.structure(d, finest)).collect();
        let growth = self.params.planet.geology.fold_growth;
        self.uplift = self.belt_uplift.iter().zip(&self.structure).map(|(u, s)| u * (1. + growth * s.crest).max(0.)).collect();
    }

    /// Let the crust respond to the load erosion, deposition and ice have put on it or taken
    /// off since time zero (`isostasy`), relaxing towards equilibrium over `dt` years. It moves
    /// the rock as uplift does; base level (the sea, a patch's edge) is held.
    fn compensate(&mut self, fixed: &[bool], dt: f64) {
        let Some(p) = self.params.isostasy.clone() else { return };
        let column: Vec<f64> = (0..self.domain.len()).map(|i| self.bed[i] - self.ground0[i] - self.exhumed[i]).collect();
        let target = crate::terrain::isostasy::equilibrium(&self.domain, &column, &self.ice, &p);
        let relax = 1. - (-dt / p.relaxation).exp();
        for i in 0..self.domain.len() {
            if fixed[i] { continue; }
            let dw = (target[i] - self.rebound[i]) * relax;
            self.bed[i] += dw;
            self.exhumed[i] += dw;
            self.rebound[i] += dw;
        }
    }

    /// Returns the rate glaciers eroded each cell over the step, m³/yr.
    fn glaciate(&mut self, g: &GlacierParams, fixed: &[bool], dt: f64) -> Vec<f64> {
        let n = self.domain.len();
        let start = self.bed.clone();
        let erodibility = self.glacial_erodibility();
        // Ice flows for dt / acceleration years, and erodes as if for dt.
        let ice_time = dt / g.acceleration;
        let mut done = 0.;
        let mut mean_sliding = vec![0.; n];
        let snow: Vec<f64> = self.rain_factor().iter().map(|r| r.clamp(0.2, 3.)).collect();
        while done < ice_time * (1. - 1e-9) {
            let t = self.time + done * g.acceleration;
            let ela: Vec<f64> = (0..n).map(|i| match &self.ela {
                // Today's climate is an interglacial: the glacial cycle lowers the snowline from
                // there.
                Some(e) => e[i] + g.ela_amplitude * ((TAU * t / g.period).cos() - 1.),
                None => g.ela(self.latitude[i], t),
            }).collect();
            let balance: Vec<f64> = (0..n).map(|i| {
                // Snow falls as the rain does: more on windward slopes, less in their lee.
                let m = g.mass_balance(self.bed[i] + self.ice[i], ela[i]);
                if m > 0. { m * snow[i] } else { m }
            }).collect();
            // Nothing to do while it is too warm for ice anywhere.
            if balance.iter().all(|&m| m <= 0.) && self.ice.iter().all(|&h| h <= 0.) { break; }
            let step = flow_ice(&self.domain, g, &self.bed, &mut self.ice, &balance, fixed, (ice_time - done).min(g.max_step));
            // Only ice whose bed is at the melting point slides, and erodes.
            let sliding: Vec<f64> = (0..n).map(|i| step.sliding[i] * g.basal_melting(self.bed[i] + self.ice[i], self.ice[i], ela[i])).collect();
            let before = self.bed.clone();
            erode(g, &mut self.bed, &sliding, &erodibility, fixed, step.dt * g.acceleration);
            self.record(&before);
            for (m, s) in mean_sliding.iter_mut().zip(&sliding) { *m += s * step.dt / ice_time; }
            done += step.dt;
            self.ice_steps += 1;
            self.ice_iterations += step.iterations;
        }
        self.sliding = mean_sliding;
        (0..n).map(|i| (start[i] - self.bed[i]).max(0.) * self.domain.area[i] / dt).collect()
    }
}

/// The evolving state of a `Terrain`, to save and restore: everything that changes with time
/// (the domain, planet and vents are rebuilt from the parameters).
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Snapshot {
    time: f64,
    bed: Vec<f64>,
    sediment: Vec<f64>,
    lava: Vec<f64>,
    sediment_flux: Vec<f64>,
    ice: Vec<f64>,
    sea: Vec<bool>,
    drainage: Vec<f64>,
    receiver: Vec<usize>,
    sliding: Vec<f64>,
    /// Absent from snapshots taken before rock was carried up with the ground.
    #[serde(default)]
    exhumed: Vec<f64>,
    #[serde(default)]
    rebound: Vec<f64>,
    #[serde(default)]
    displacement: (f64, f64),
    #[serde(default)]
    shifted: (i64, i64),
    #[serde(default)]
    ground0: Vec<f64>,
    channels: Vec<crate::terrain::channel::Channel>,
    oxbows: Vec<crate::terrain::channel::Oxbow>,
    rivers_found: Option<f64>,
}

impl Snapshot {
    /// The time it was taken at, years.
    pub fn time(&self) -> f64 { self.time }

    /// Whether it is of a domain with as many cells as `t`'s, so `t` can restore it.
    pub fn fits(&self, t: &Terrain) -> bool {
        let n = t.domain.len();
        [&self.bed, &self.sediment, &self.lava, &self.sediment_flux, &self.ice, &self.drainage, &self.sliding].iter().all(|f| f.len() == n)
            && (self.exhumed.is_empty() || self.exhumed.len() == n) && (self.rebound.is_empty() || self.rebound.len() == n) && self.sea.len() == n && self.receiver.len() == n
    }
}

impl Terrain {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            time: self.time, bed: self.bed.clone(), sediment: self.sediment.clone(), lava: self.lava.clone(),
            sediment_flux: self.sediment_flux.clone(), ice: self.ice.clone(), sea: self.sea.clone(),
            drainage: self.drainage.clone(), receiver: self.receiver.clone(), sliding: self.sliding.clone(), exhumed: self.exhumed.clone(), rebound: self.rebound.clone(),
            displacement: self.displacement, shifted: self.shifted, ground0: self.ground0.clone(),
            channels: self.rivers.channels.clone(), oxbows: self.rivers.oxbows.clone(), rivers_found: self.rivers.found,
        }
    }

    /// Restore a snapshot of a terrain with the same parameters and domain.
    pub fn restore(&mut self, s: Snapshot) {
        assert_eq!(s.bed.len(), self.domain.len(), "snapshot of a different domain");
        self.time = s.time;
        self.bed = s.bed;
        self.sediment = s.sediment;
        self.lava = s.lava;
        self.sediment_flux = s.sediment_flux;
        self.ice = s.ice;
        self.sea = s.sea;
        self.drainage = s.drainage;
        self.receiver = s.receiver;
        self.sliding = s.sliding;
        // An older snapshot: the uplift since time zero.
        self.exhumed = if s.exhumed.len() == self.domain.len() { s.exhumed }
            else { self.uplift.iter().zip(&self.sea).map(|(u, &sea)| if sea { 0. } else { u * s.time }).collect() };
        self.rebound = if s.rebound.len() == self.domain.len() { s.rebound } else { vec![0.; self.domain.len()] };
        if s.ground0.len() == self.domain.len() { self.ground0 = s.ground0; }
        self.displacement = s.displacement;
        if s.shifted != self.shifted {
            self.shifted = s.shifted;
            self.place_structure();
        }
        self.rivers.channels = s.channels;
        self.rivers.oxbows = s.oxbows;
        self.rivers.found = s.rivers_found;
    }

    /// Run to `time`, starting from the latest state saved in `cache` (a directory) for the same
    /// parameters and domain at or before it, and saving the result there. `key` identifies the
    /// parameters and domain.
    pub fn run_cached(&mut self, time: f64, cache: &std::path::Path, key: &str) {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?} {} {:?}", self.params, key, self.boundary_rate).hash(&mut hasher);
        let prefix = format!("landscape-{:016x}-", hasher.finish());
        let _ = std::fs::create_dir_all(cache);
        let mut saved: Vec<(u64, std::path::PathBuf)> = std::fs::read_dir(cache).into_iter().flatten().flatten()
            .filter_map(|e| {
                let name = e.file_name().into_string().ok()?;
                let years = name.strip_prefix(&prefix)?.strip_suffix(".json")?.parse::<u64>().ok()?;
                Some((years, e.path()))
            }).collect();
        saved.sort();
        let wanted = time.round() as u64;
        if let Some(s) = saved.iter().rev().filter(|(y, _)| *y <= wanted).find_map(|(_, p)| {
            serde_json::from_reader::<_, Snapshot>(std::io::BufReader::new(std::fs::File::open(p).ok()?)).ok()
        }) {
            self.restore(s);
        }
        if self.time < time - 1e-9 {
            self.run_to(time);
            let path = cache.join(format!("{}{}.json", prefix, wanted));
            let written = std::fs::File::create(&path).map_err(|e| e.to_string())
                .and_then(|f| serde_json::to_writer(std::io::BufWriter::new(f), &self.snapshot()).map_err(|e| e.to_string()));
            if let Err(e) = written {
                eprintln!("  landscape: couldn't cache the terrain in {}: {}", path.display(), e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(seed: u64) -> Terrain {
        let mut params = TerrainParams::default();
        params.planet.seed = seed;
        params.step = 500.;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        Terrain::new(Domain::patch(lat, lon, 24, 1000., params.planet.radius), params)
    }

    #[test]
    fn test_time_zero_is_the_noise_topography_where_it_can_stand() {
        let t = small(1);
        let noise: Vec<f64> = t.domain.dirs.iter().map(|d| t.planet.elevation(d, 2000.)).collect();
        // Much of it is the noise topography as it was; what has changed failed, or lies under
        // its debris.
        let kept = (0..t.domain.len()).filter(|&i| t.bed[i] == noise[i]).count();
        assert!(kept * 3 > t.domain.len(), "{} of {}", kept, t.domain.len());
        for i in 0..t.domain.len() {
            assert!(t.bed[i] == noise[i] || t.bed[i] < noise[i] || t.sediment[i] > 0.);
        }
        // And nothing stands steeper than its rock holds.
        for i in (0..t.domain.len()).filter(|&i| !t.domain.boundary[i]) {
            let limit = t.critical_slope(i);
            for l in t.domain.links(i) { assert!(t.bed[i] - t.bed[l.to] <= crate::terrain::fluvial::LOOSE * limit * l.dist + 0.2, "cell {}", i); }
        }
    }

    #[test]
    fn test_evolution_is_deterministic_and_resumable() {
        let mut a = small(2);
        a.run_to(3000.);
        let mut b = small(2);
        b.run_to(1000.);
        b.run_to(3000.);
        assert_eq!(a.bed, b.bed);
        assert_eq!(a.ice, b.ice);
    }

    #[test]
    fn test_cached_run_resumes_where_it_left_off() {
        let dir = std::env::temp_dir().join(format!("landscape-cache-test-{}", std::process::id()));
        let mut a = small(4);
        a.run_cached(1000., &dir, "k");
        let mut b = small(4);
        b.run_cached(2000., &dir, "k");
        let mut c = small(4);
        c.run_to(2000.);
        assert_eq!(b.bed, c.bed);
        let mut d = small(4);
        d.run_cached(2000., &dir, "k");
        assert_eq!(d.bed, c.bed);
        assert_eq!(d.time, 2000.);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_glaciers_make_more_of_fractured_rock_than_rivers() {
        let t = small(3);
        let (rivers, glaciers) = (t.erodibility(), t.glacial_erodibility());
        let bedrock: Vec<usize> = (0..t.domain.len()).filter(|&i| !matches!(t.rock(i), Rock::Sediment | Rock::Basalt)).collect();
        // Each cell's glacial over river erodibility, relative to its rock's own.
        let gain = |i: usize| {
            let r = t.rock(i);
            (glaciers[i] / rivers[i]) / (r.erodibility(Process::Glaciers, 5.) / r.erodibility(Process::Rivers, 5.))
        };
        let fractured: Vec<usize> = bedrock.iter().copied().filter(|&i| t.structure[i].fracture > 0.5).collect();
        let sound: Vec<usize> = bedrock.iter().copied().filter(|&i| t.structure[i].fracture < 0.05).collect();
        assert!(!fractured.is_empty() && !sound.is_empty(), "{} fractured, {} sound", fractured.len(), sound.len());
        assert!(fractured.iter().all(|&i| gain(i) > 1.3), "{:?}", fractured.iter().map(|&i| gain(i)).collect::<Vec<_>>());
        assert!(sound.iter().all(|&i| (gain(i) - 1.).abs() < 0.1));
    }

    #[test]
    fn test_erosion_lifts_the_crust() {
        // Without ice, the mountains erode, and the crust rises by up to ρ_rock / ρ_mantle of
        // the rock taken off (less while the mantle is still relaxing).
        let mut params = TerrainParams::default();
        params.glacier = None;
        params.step = 500.;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let mut t = Terrain::new(Domain::patch(lat, lon, 24, 1000., params.planet.radius), params);
        t.run_to(20_000.);
        let inner: Vec<usize> = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i] && !t.sea[i]).collect();
        let mean = |f: &dyn Fn(usize) -> f64| inner.iter().map(|&i| f(i)).sum::<f64>() / inner.len() as f64;
        let eroded = -mean(&|i| t.bed[i] - t.ground0[i] - t.exhumed[i]);
        let rebound = mean(&|i| t.rebound[i]);
        assert!(eroded > 0. && rebound > 0. && rebound < eroded * 2700. / 3300., "eroded {} rebound {}", eroded, rebound);
    }

    #[test]
    fn test_thrust_sheets_carry_the_rock_towards_the_foreland() {
        let mut params = TerrainParams::default();
        params.glacier = None;
        params.step = 500.;
        params.planet.geology.shortening_rate = 0.5;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let mut t = Terrain::new(Domain::patch(lat, lon, 24, 1000., params.planet.radius), params);
        let speed = t.velocity.0.hypot(t.velocity.1);
        assert!(speed > 0.01, "{:?}", t.velocity);
        let before = t.bed.clone();
        // Long enough to move three cells.
        t.run_to((3000. / speed / 500.).ceil() * 500.);
        let cells = t.shifted.0.abs() + t.shifted.1.abs();
        assert!(cells >= 3, "{:?} after {:?}", t.shifted, t.displacement);
        // The cells move the way the rock does.
        assert!(t.shifted.0 as f64 * t.velocity.0 >= 0. && t.shifted.1 as f64 * t.velocity.1 >= 0.);
        // Each cell's rock is the planet's from where it was at time zero.
        for i in [100, 300, 450] {
            assert_eq!(t.structure[i], t.planet.structure(&t.origin(i), t.finest));
        }
        // And the relief went with it: the ground now looks more like the ground of time zero
        // moved by the shift than as it was.
        let n = 24i64;
        let at = |k: i64, di: i64, dj: i64| -> Option<usize> {
            let (i, j) = (k % n - di, k / n - dj);
            (i > 0 && j > 0 && i < n - 1 && j < n - 1).then_some((j * n + i) as usize)
        };
        let misfit = |di: i64, dj: i64| -> f64 {
            let pairs: Vec<(f64, f64)> = (0..n * n).filter_map(|k| at(k, di, dj).map(|s| (t.bed[k as usize], before[s]))).collect();
            let mean = |f: &dyn Fn(&(f64, f64)) -> f64| pairs.iter().map(f).sum::<f64>() / pairs.len() as f64;
            let (ma, mb) = (mean(&|p| p.0), mean(&|p| p.1));
            mean(&|p| ((p.0 - ma) - (p.1 - mb)).powi(2))
        };
        assert!(misfit(t.shifted.0, t.shifted.1) < misfit(0, 0), "{} vs {}", misfit(t.shifted.0, t.shifted.1), misfit(0, 0));
    }

    #[test]
    fn test_rain_on_limestone_sinks_and_rises_at_springs() {
        let t = small(2);
        let karst: Vec<usize> = (0..t.domain.len()).filter(|&i| t.rock(i) == Rock::Limestone && t.ice[i] < ICE_FREE).collect();
        assert!(!karst.is_empty());
        let flow = t.route();
        let plain = route(&t.domain, &t.surface(), &t.outlets(), &t.rain_factor());
        // All the water still leaves through the outlets.
        let out = |f: &Flow| (0..t.domain.len()).filter(|&i| f.receiver[i] == i).map(|i| f.area[i]).sum::<f64>();
        assert!((out(&flow) - out(&plain)).abs() < 1e-6 * out(&plain), "{} vs {}", out(&flow), out(&plain));
        // But less of it runs over the limestone: at the heads of its valleys, only what
        // doesn't sink (rivers coming onto it from elsewhere keep their water).
        let heads: Vec<usize> = karst.iter().copied().filter(|&i| !t.outlets()[i] && (0..t.domain.len()).all(|j| plain.receiver[j] != i || j == i)).collect();
        assert!(!heads.is_empty());
        let keep = 1. - t.params.planet.geology.infiltration;
        for &i in &heads { assert!((flow.area[i] - keep * plain.area[i]).abs() < 1e-6 * plain.area[i]); }
        // And exposed limestone, alone, is lowered by solution.
        let mut d = small(2);
        let before = d.bed.clone();
        let fixed = d.outlets();
        d.dissolve(&fixed, 1000.);
        for i in 0..d.domain.len() {
            if karst.contains(&i) && !fixed[i] { assert!(d.bed[i] < before[i]); } else { assert_eq!(d.bed[i], before[i]); }
        }
    }

    /// Validation, run with `--ignored --nocapture` (half a minute in a release build).
    #[test]
    #[ignore]
    fn test_channels_and_slopes_are_steeper_on_hard_rock() {
        // After long enough for the rivers to come near steady state with the uplift, channels
        // on rock that resists rivers are steeper for their drainage (higher steepness index
        // k_sn = S A^0.5) than on weak rock, as in real ranges (Kirby and Whipple, "Expression
        // of active tectonics in erosional landscapes", J. Structural Geology 44, 2012), and so
        // are their hillslopes.
        let mut params = TerrainParams::default();
        params.glacier = None;
        params.volcano = None;
        params.channels = None;
        params.step = 2000.;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let mut t = Terrain::new(Domain::patch(lat, lon, 48, 500., params.planet.radius), params);
        let start = std::time::Instant::now();
        t.run_to(400_000.);
        eprintln!("ran in {:.1?}", start.elapsed());
        let flow = t.route();
        let n = t.domain.len();
        let hardness = |i: usize| -t.rock(i).scores().0;
        let ksn: Vec<(f64, f64)> = (0..n).filter(|&i| !t.outlets()[i] && flow.area[i] > 2e6 && t.sediment[i] < 1. && flow.receiver[i] != i).map(|i| {
            let s = (t.bed[i] - t.bed[flow.receiver[i]]).max(0.) / flow.receiver_dist[i];
            (hardness(i), s * flow.area[i].sqrt())
        }).collect();
        let median = |mut v: Vec<f64>| { v.sort_by(|a, b| a.total_cmp(b)); if v.is_empty() { f64::NAN } else { v[v.len() / 2] } };
        let hard = median(ksn.iter().filter(|c| c.0 > 0.4).map(|c| c.1).collect());
        let soft = median(ksn.iter().filter(|c| c.0 < -0.2).map(|c| c.1).collect());
        let slope2 = crate::terrain::glacier::slope_squared(&t.domain, &t.bed);
        let slopes = |hard: bool| median((0..n).filter(|&i| !t.outlets()[i] && t.sediment[i] < 1. && flow.area[i] < 1e6 && (hardness(i) > 0.4) == hard && (hardness(i) < -0.2) != hard).map(|i| slope2[i].sqrt()).collect());
        eprintln!("k_sn hard {} soft {}; hillslopes hard {} soft {} ({} channel cells)", hard, soft, slopes(true), slopes(false), ksn.len());
        assert!(hard > soft, "k_sn on hard rock {} vs soft {}", hard, soft);
        assert!(slopes(true) > slopes(false));
    }

    #[test]
    fn test_rivers_cut_valleys() {
        let mut params = TerrainParams::default();
        params.glacier = None;
        params.volcano = None;
        params.fluvial.deposition = 0.;
        params.fluvial.k = 1e-4;
        params.step = 1000.;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let mut t = Terrain::new(Domain::patch(lat, lon, 24, 1000., params.planet.radius), params);
        let start = t.bed.clone();
        t.run_to(50_000.);
        // The bedrock channel with the most drainage is lowered relative to uplift alone. (The
        // most drained cell of all may lie in a basin behind a sill of hard rock or a growing
        // fold, which fills with sediment instead.)
        let big = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i] && t.sediment[i] < 1.)
            .max_by(|&a, &b| t.drainage[a].total_cmp(&t.drainage[b])).unwrap();
        assert!(t.bed[big] < start[big] + t.uplift[big] * 50_000. - 1.);
    }
}
