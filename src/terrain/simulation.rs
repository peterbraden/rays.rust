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
//! 5. hillslope diffusion (`fluvial`).
//!
//! ## Geology
//!
//! The ground is layered. At the bottom is bedrock, whose erodibility varies with position and
//! depth (`Planet::erodibility`: folded strata of hard and soft rock). On it lies volcanic rock
//! (`lava`), and on top sediment (`sediment`): alluvium, lake fill, colluvium from hillslopes.
//! Ground gained by deposition or creep is sediment; ground lost is taken from the sediment,
//! then the lava, then the bedrock, so erosion cuts through the layers in order. Rivers and
//! glaciers erode whatever is exposed with its own erodibility: sediment 3 times average
//! bedrock, lava 0.3 times (both model choices; fresh basalt is strong, unconsolidated sediment
//! weak). Uplift moves all layers together.

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
use crate::terrain::fluvial::{diffuse, erode_and_deposit, FluvialParams};
use crate::terrain::glacier::{erode, flow_ice, GlacierParams};
use crate::terrain::planet::{Planet, PlanetParams};
use crate::terrain::volcano::{erupt, vents, Vent, VolcanoParams};

#[derive(Clone, Debug)]
pub struct TerrainParams {
    pub planet: PlanetParams,
    pub fluvial: FluvialParams,
    /// None to disable glaciers.
    pub glacier: Option<GlacierParams>,
    /// None to disable volcanoes.
    pub volcano: Option<VolcanoParams>,
    /// Erodibility of sediment, relative to average bedrock.
    pub sediment_erodibility: f64,
    /// Erodibility of volcanic rock, relative to average bedrock.
    pub lava_erodibility: f64,
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
            sediment_erodibility: 3.,
            lava_erodibility: 0.3,
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
    /// Cells that were sea at time zero. Ground eroded below sea level later is a lake bed
    /// (eg. a glacially overdeepened basin), not sea.
    pub sea: Vec<bool>,
    /// Latitude of each cell, radians.
    pub latitude: Vec<f64>,
    /// Drainage area at the last step, square metres.
    pub drainage: Vec<f64>,
    /// Basal sliding speed at the last step, metres per year.
    pub sliding: Vec<f64>,
    /// Ice flow steps taken so far (for performance statistics).
    pub ice_steps: usize,
    /// ... and the solver iterations they took.
    pub ice_iterations: usize,
    vents: Vec<Vent>,
}

impl Terrain {
    pub fn new(domain: Domain, mut params: TerrainParams) -> Terrain {
        params.planet.radius = domain.radius;
        let planet = Planet::new(params.planet.clone());
        // Detail down to about two cells.
        let finest = 2. * domain.min_spacing();
        let bed: Vec<f64> = domain.dirs.iter().map(|d| planet.elevation(d, finest)).collect();
        let uplift = domain.dirs.iter().map(|d| planet.uplift(d)).collect();
        let latitude = domain.dirs.iter().map(|d| d.y.clamp(-1., 1.).asin()).collect();
        let n = domain.len();
        let sea = bed.iter().map(|&b| b < 0.).collect();
        let vents = match &params.volcano { Some(v) => vents(&planet, &domain, v), None => vec![] };
        let mut t = Terrain {
            domain, params, planet, time: 0., bed, uplift, latitude, sea, vents,
            sediment: vec![0.; n], lava: vec![0.; n], sediment_flux: vec![0.; n],
            ice: vec![0.; n], drainage: vec![0.; n], sliding: vec![0.; n], ice_steps: 0, ice_iterations: 0,
        };
        t.drainage = t.route().area;
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

    /// Erodibility of the material exposed at each cell, relative to average bedrock.
    pub fn erodibility(&self) -> Vec<f64> {
        (0..self.domain.len()).map(|i| {
            if self.sediment[i] > 0.5 { self.params.sediment_erodibility }
            else if self.lava[i] > 0.5 { self.params.lava_erodibility }
            else { self.planet.erodibility(&self.domain.dirs[i], self.bedrock(i)) }
        }).collect()
    }

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
        while self.time < t - 1e-9 {
            let dt = self.params.step.min(t - self.time);
            self.step(dt);
        }
    }

    pub fn step(&mut self, dt: f64) {
        let fixed = self.outlets();
        for ((b, u), f) in self.bed.iter_mut().zip(&self.uplift).zip(&fixed) {
            if !f { *b += u * dt; }
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
        let before = self.bed.clone();
        diffuse(&self.domain, &mut self.bed, &fixed, self.params.fluvial.diffusivity, dt);
        self.record(&before);
        self.drainage = flow.area;
        self.time += dt;
    }

    /// Returns the rate glaciers eroded each cell over the step, m³/yr.
    fn glaciate(&mut self, g: &GlacierParams, fixed: &[bool], dt: f64) -> Vec<f64> {
        let n = self.domain.len();
        let start = self.bed.clone();
        let erodibility = self.erodibility();
        // Ice flows for dt / acceleration years, and erodes as if for dt.
        let ice_time = dt / g.acceleration;
        let mut done = 0.;
        let mut mean_sliding = vec![0.; n];
        while done < ice_time * (1. - 1e-9) {
            let t = self.time + done * g.acceleration;
            let balance: Vec<f64> = (0..n).map(|i| {
                g.mass_balance(self.bed[i] + self.ice[i], g.ela(self.latitude[i], t))
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
        // The cells with the most drainage are lowered relative to uplift alone.
        let big = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i])
            .max_by(|&a, &b| t.drainage[a].total_cmp(&t.drainage[b])).unwrap();
        assert!(t.bed[big] < start[big] + t.uplift[big] * 50_000. - 1.);
    }
}
