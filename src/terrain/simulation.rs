//! # Landscape evolution through time
//!
//! A `Terrain` is the state of a planet (or a patch of one) at a time t, in years: bedrock
//! elevation, ice thickness, and what the processes last computed (drainage area, sliding
//! speed). At t = 0 it is the noise topography of `planet`. `run_to(t)` advances it by fixed
//! steps, each applying, in turn (operator splitting, as in Braun et al. 1999's ICE-CASCADE and
//! FastScape):
//!
//! 1. tectonic uplift (`planet`);
//! 2. volcanic eruptions: cones and lava flows (`volcano`);
//! 3. glaciers: mass balance, ice flow and glacial erosion, in sub-steps (`glacier`), the
//!    eroded rock leaving in meltwater as sediment for the rivers (Hallet, Hunter and Bogen,
//!    "Rates of erosion and sediment evacuation by glaciers", Global and Planetary Change 12,
//!    1996, estimate glacial erosion rates from that sediment);
//! 4. flow routing over the ice and rock surface, then river erosion and deposition on
//!    ice-free ground (`flow`, `fluvial`);
//! 5. deltas: what rivers bring to the sea spreads over the sea floor (`marine`);
//! 6. hillslope diffusion (`fluvial`), faster on weak rock and sediment;
//! 7. meandering rivers, a sub-grid model drawn over the terrain (`channel`).
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
//! than their synclines (`GeologyParams::fold_growth`).

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

use crate::terrain::domain::Domain;
use crate::terrain::flow::{route, Flow};
use crate::terrain::fluvial::{diffuse_varying, erode_and_deposit, FluvialParams};
use crate::terrain::glacier::{erode, flow_ice, GlacierParams};
use crate::terrain::planet::{Planet, PlanetParams};
use crate::terrain::volcano::{erupt, vents, Vent, VolcanoParams};
use crate::terrain::marine::{self, MarineParams};
use crate::terrain::channel::{ChannelParams, Rivers};
use crate::terrain::fluvial::LAKE;
use crate::terrain::geology::{Process, Rock, Structure};
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
            sediment_erodibility: 3.,
            lava_erodibility: 0.3,
            sediment_glacial_erodibility: 3.,
            lava_glacial_erodibility: 1.5,
            step: 100.,
        }
    }
}

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
    /// How far uplift has carried each cell's rock up since time zero, metres.
    pub exhumed: Vec<f64>,
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
        let uplift = domain.dirs.iter().zip(&structure).map(|(d, s)| planet.uplift(d) * (1. + growth * s.crest).max(0.)).collect();
        let latitude = domain.dirs.iter().map(|d| d.y.clamp(-1., 1.).asin()).collect();
        let n = domain.len();
        let sea = bed.iter().map(|&b| b < 0.).collect();
        let vents = match &params.volcano { Some(v) => vents(&planet, &domain, v), None => vec![] };
        let ela = params.climate.as_ref().map(|c| {
            let climate = crate::terrain::climate::climate(c, &planet);
            domain.dirs.iter().map(|d| climate.local(d, 0.).snowline(0.)).collect()
        });
        let mut t = Terrain {
            domain, params, planet, time: 0., ela, bed, uplift, structure, exhumed: vec![0.; n], latitude, sea, vents, rivers: Rivers::default(),
            sediment: vec![0.; n], lava: vec![0.; n], sediment_flux: vec![0.; n],
            ice: vec![0.; n], boundary_rate: vec![0.; n], drainage: vec![0.; n], receiver: (0..n).collect(), sliding: vec![0.; n], ice_steps: 0, ice_iterations: 0,
        };
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
            (r, process) => r.erodibility(process, contrast),
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

    fn route(&self) -> Flow {
        route(&self.domain, &self.surface(), &self.outlets(), &vec![1.; self.domain.len()])
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
        let before = self.bed.clone();
        self.sediment_flux = erode_and_deposit(&self.domain, &flow, &mut self.bed, &fixed, &erodibility, &lake, &glacial, &self.params.fluvial, dt);
        self.record(&before);
        if let Some(m) = &self.params.marine {
            // What the rivers bring to the sea builds deltas.
            let sea: Vec<bool> = (0..self.domain.len()).map(|i| fixed[i] && !self.domain.boundary[i]).collect();
            let supply: Vec<f64> = (0..self.domain.len()).map(|i| if sea[i] { self.sediment_flux[i] } else { 0. }).collect();
            let before = self.bed.clone();
            marine::deposit(&self.domain, &mut self.bed, &self.sediment, &sea, &supply, m, dt);
            self.record(&before);
        }
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
        self.time += dt;
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
        while done < ice_time * (1. - 1e-9) {
            let t = self.time + done * g.acceleration;
            let balance: Vec<f64> = (0..n).map(|i| {
                let ela = match &self.ela {
                    // Today's climate is an interglacial: the glacial cycle lowers the snowline
                    // from there.
                    Some(e) => e[i] + g.ela_amplitude * ((TAU * t / g.period).cos() - 1.),
                    None => g.ela(self.latitude[i], t),
                };
                g.mass_balance(self.bed[i] + self.ice[i], ela)
            }).collect();
            // Nothing to do while it is too warm for ice anywhere.
            if balance.iter().all(|&m| m <= 0.) && self.ice.iter().all(|&h| h <= 0.) { break; }
            let step = flow_ice(&self.domain, g, &self.bed, &mut self.ice, &balance, fixed, (ice_time - done).min(g.max_step));
            let before = self.bed.clone();
            erode(g, &mut self.bed, &step.sliding, &erodibility, fixed, step.dt * g.acceleration);
            self.record(&before);
            for (m, s) in mean_sliding.iter_mut().zip(&step.sliding) { *m += s * step.dt / ice_time; }
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
            && (self.exhumed.is_empty() || self.exhumed.len() == n) && self.sea.len() == n && self.receiver.len() == n
    }
}

impl Terrain {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            time: self.time, bed: self.bed.clone(), sediment: self.sediment.clone(), lava: self.lava.clone(),
            sediment_flux: self.sediment_flux.clone(), ice: self.ice.clone(), sea: self.sea.clone(),
            drainage: self.drainage.clone(), receiver: self.receiver.clone(), sliding: self.sliding.clone(), exhumed: self.exhumed.clone(),
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
        // An older snapshot: the uplift since time zero, as it would have been at sea's edge.
        self.exhumed = if s.exhumed.len() == self.domain.len() { s.exhumed }
            else { self.uplift.iter().zip(&self.sea).map(|(u, &sea)| if sea { 0. } else { u * s.time }).collect() };
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
    fn test_time_zero_is_the_noise_topography() {
        let t = small(1);
        let d = &t.domain.dirs[100];
        assert_eq!(t.bed[100], t.planet.elevation(d, 2000.));
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
