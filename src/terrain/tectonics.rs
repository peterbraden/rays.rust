//! # Plate tectonics
//!
//! A planet's large-scale topography (where the continents and ocean basins are, where the
//! mountain belts, trenches, island arcs and volcanoes are) comes from the motion of its plates
//! over hundreds of millions of years. This simulates it, after Cortial, Peytavie, Galin and
//! Guérin ("Procedural Tectonic Planets", Computer Graphics Forum 38(2), 2019,
//! doi:10.1111/cgf.13614), in a simpler form:
//!
//! - The crust is held on a cubed-sphere grid (`Domain::sphere`). Each cell belongs to a plate
//!   and carries its crust: continental or oceanic, the oceanic crust's age, and how much the
//!   crust has been thickened by mountain building.
//! - **Plates are rigid** and turn about Euler poles: each has an angular velocity ω (a vector
//!   along its pole, in radians per million years), so a point x on it moves at ω × x. Each
//!   step (2 Myr by default) carries every plate's crust forward: a cell takes its crust from
//!   where each plate's rotation brings it from (semi-Lagrangian advection, nearest cell).
//! - **Divergence.** Where no plate arrives, the plates have pulled apart: new oceanic crust
//!   forms at a mid-ocean ridge. Oceanic crust sinks as it cools and ages, by half-space cooling
//!   while young (Parsons and Sclater, "An analysis of the variation of ocean floor bathymetry
//!   and heat flow with age", JGR 82(5), 1977, doi:10.1029/JB082i005p00803), flattening towards
//!   5.65 km as the plate model has it, with pelagic sediment on it (`ocean_depth`).
//! - **Convergence.** Where several plates arrive, one goes under. Continental crust is too
//!   buoyant to subduct, and of two oceanic crusts the older, colder and denser sinks. Where
//!   oceanic crust subducts, a trench forms along the front, and the overriding plate is uplifted
//!   and volcanic in a band a few hundred kilometres behind it (the Andes; island arcs, where the
//!   overriding plate is oceanic), in proportion to the speed of convergence (Cortial et al.'s
//!   subduction uplift). Where two continents meet they collide: the crust thickens over a wider
//!   region (the Himalaya and Tibet), and the two plates are sutured, moving on as one.
//! - **Rifting.** Every so often the largest plate carrying continent splits in two along a
//!   wandering line, and the halves move apart, opening a new ocean (the Atlantic).
//! - **Hotspots** are fixed in the mantle: plates passing over them are built up into chains of
//!   volcanic islands and seamounts (Hawaii).
//! - **Erosion and relaxation** at this scale: thickened crust relaxes back with a time
//!   constant of about 100 Myr, so old mountain belts are lower than young ones.
//!
//! The planet starts with continents from low-frequency noise and plates from a warped Voronoi
//! partition, each with a random Euler pole and a speed of a few centimetres a year, and runs for
//! `age` million years. The result (`Crust`) gives the planet's elevation, how active mountain
//! building is, and where volcanoes are, as smooth functions of direction (`Planet` adds the
//! detail).

use crate::geometry::{point_on_unit_sphere, Pcg32};
use crate::na::{Rotation3, Unit, Vector3};
use crate::noise::PerlinNoise;
use crate::terrain::domain::Domain;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, Mutex, OnceLock};
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TectonicsParams {
    /// Grid cells per cube face edge.
    pub cells: usize,
    /// Plates at the start.
    pub plates: usize,
    /// Million years simulated.
    pub age: f64,
    /// Million years per step.
    pub step: f64,
    /// Fraction of the surface that is continent at the start.
    pub continental_fraction: f64,
    /// Typical plate speed, kilometres per million years (mm per year).
    pub speed: f64,
    /// Mean time between rifting events, million years.
    pub rift_interval: f64,
    /// Thickening of the overriding plate behind a subduction zone, metres per million years at
    /// the typical speed.
    pub subduction_uplift: f64,
    /// Thickening where continents collide, metres per million years at the typical speed.
    pub collision_uplift: f64,
    /// Relaxation time of thickened crust, million years.
    pub relaxation: f64,
    /// Number of hotspots.
    pub hotspots: usize,
    /// Million years between resamplings of the plates onto the grid.
    pub remesh: f64,
}

impl Default for TectonicsParams {
    fn default() -> TectonicsParams {
        TectonicsParams {
            cells: 96, plates: 12, age: 300., step: 2., continental_fraction: 0.32, speed: 50.,
            rift_interval: 70., subduction_uplift: 35., collision_uplift: 70., relaxation: 100.,
            hotspots: 10, remesh: 10.,
        }
    }
}

/// Elevation of continental crust that has not been thickened, metres.
const CONTINENT_BASE: f64 = 350.;
/// Depth of a mid-ocean ridge, metres.
const RIDGE_DEPTH: f64 = 2500.;
/// Thickening is limited to about this, metres (crust too thick flows sideways).
const MAX_THICKENING: f64 = 3500.;
/// Distance of the volcanic arc behind a subduction front, and its half-width, metres.
const ARC_DISTANCE: f64 = 250e3;
const ARC_WIDTH: f64 = 150e3;
/// How long colliding continents take to lock together, million years.
const SUTURE_TIME: f64 = 30.;
/// Reach of collision uplift from the front, metres.
const COLLISION_REACH: f64 = 700e3;

/// Depth of the sea floor on oceanic crust of `age` million years, metres (negative: below sea
/// level). Young floor sinks as half-space cooling has it, d = 2600 + 365 √t; from 20 Myr the
/// cooling plate's base holds it up and the depth flattens towards 5.65 km, as Stein and Stein's
/// fit to the ocean floor has it (GDH1: d = 5651 - 2473 exp(-0.0278 t); "A model for the global
/// variation in oceanic depth and heat flow with lithospheric age", Nature 359, 1992). (Half-space
/// cooling alone, capped at 6 km, made the ocean a kilometre deeper on average than the Earth's.)
/// On it settles pelagic sediment, about 5 m a million years up to half a kilometre, which
/// raises the floor by what it adds less the mantle it displaces under water, 0.57 of its
/// thickness.
pub fn ocean_depth(age: f64) -> f64 {
    let t = age.max(0.);
    let cooled = if t < 20. { 2600. + 365. * t.sqrt() } else { 5651. - 2473. * (-0.0278 * t).exp() };
    let sediment = (5. * t).min(500.);
    -(cooled - 0.57 * sediment)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell {
    plate: u16,
    continental: bool,
    /// Age of oceanic crust, million years.
    age: f64,
    /// Thickening by mountain building (or volcanic islands), metres.
    thickening: f64,
}

impl Cell {
    fn elevation(&self) -> f64 {
        if self.continental { CONTINENT_BASE + self.thickening } else { ocean_depth(self.age) + self.thickening }
    }
}

#[derive(Clone, Debug)]
struct Plate {
    /// Angular velocity, radians per million years.
    omega: Vector3<f64>,
}

/// The state of the planet's crust.
///
/// The plates are Lagrangian: each grid cell's crust as it was at the last resampling
/// (`reference`) belongs to a plate and moves with it, by the plate's rotation since then
/// (`rotation`), so motion of less than a cell a step still adds up. Each step the grid is
/// looked up through the rotations (`current`), and every `remesh` million years the plates are
/// resampled onto it, when the ocean floor that has formed in the gaps between them joins them.
pub struct Tectonics {
    pub params: TectonicsParams,
    pub domain: Domain,
    reference: Vec<Cell>,
    rotation: Vec<Rotation3<f64>>,
    /// The crust at each cell now.
    current: Vec<Cell>,
    /// Since when each cell has been a gap between plates (new ocean floor), million years.
    gap_since: Vec<Option<f64>>,
    plates: Vec<Plate>,
    /// Uplift in the last step, metres per million years.
    uplift: Vec<f64>,
    /// Volcanic activity in the last step, 0 to 1.
    volcanism: Vec<f64>,
    /// Trench deepening along subduction fronts in the last step, metres.
    trench: Vec<f64>,
    hotspots: Vec<Vector3<f64>>,
    rng: Pcg32,
    pub time: f64,
    last_remesh: f64,
    next_rift: f64,
}

impl Tectonics {
    pub fn new(params: TectonicsParams, seed: u64, radius: f64) -> Tectonics {
        let domain = Domain::sphere(params.cells.max(4), radius);
        let mut rng = Pcg32::new(seed, 0x7ec7);
        let n = domain.len();
        // Continents: the highest part of low-frequency noise, to the given fraction.
        let noise = PerlinNoise::with_seed(seed.wrapping_mul(7).wrapping_add(11));
        let field = |d: &Vector3<f64>| {
            let q = d * 1.6 + Vector3::new(3.7, 1.9, 5.3);
            noise.fbm(q.x, q.y, q.z, 5, 0.5, 2.)
        };
        let values: Vec<f64> = domain.dirs.iter().map(field).collect();
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| b.total_cmp(a));
        let threshold = sorted[((params.continental_fraction.clamp(0., 1.) * n as f64) as usize).min(n - 1)];
        // Plates: a Voronoi partition about random seeds, warped by noise so the boundaries
        // wander.
        let seeds: Vec<Vector3<f64>> = (0..params.plates.max(2)).map(|_| point_on_unit_sphere(rng.next_f64(), rng.next_f64())).collect();
        let warp = |d: &Vector3<f64>| {
            let q = d * 3.;
            let w = Vector3::new(noise.noise(q.x + 10., q.y, q.z), noise.noise(q.x, q.y + 20., q.z), noise.noise(q.x, q.y, q.z + 30.));
            (d + w * 0.25).normalize()
        };
        let owner = |d: &Vector3<f64>| {
            let w = warp(d);
            (0..seeds.len()).max_by(|&a, &b| w.dot(&seeds[a]).total_cmp(&w.dot(&seeds[b]))).unwrap() as u16
        };
        let mut cells: Vec<Cell> = domain.dirs.iter().zip(&values).map(|(d, &v)| Cell {
            plate: owner(d),
            continental: v >= threshold,
            // Oceans of all ages, older away from where the plates' edges are.
            age: 20. + 100. * noise.fbm(d.x * 4. + 7., d.y * 4., d.z * 4., 3, 0.5, 2.),
            thickening: 0.,
        }).collect();
        // Each continent starts whole on the plate that has most of it, so the plates' edges
        // begin in the oceans, as after a supercontinent breaks up.
        let mut seen = vec![false; n];
        for start in 0..n {
            if seen[start] || !cells[start].continental { continue; }
            let mut component = vec![start];
            seen[start] = true;
            let mut k = 0;
            while k < component.len() {
                let g = component[k];
                k += 1;
                for l in domain.links(g) {
                    if !seen[l.to] && cells[l.to].continental { seen[l.to] = true; component.push(l.to); }
                }
            }
            let mut votes: HashMap<u16, usize> = HashMap::new();
            for &g in &component { *votes.entry(cells[g].plate).or_insert(0) += 1; }
            let plate = *votes.iter().max_by_key(|(p, c)| (**c, std::cmp::Reverse(**p))).unwrap().0;
            for &g in &component { cells[g].plate = plate; }
        }
        let speed = params.speed * 1000. / radius;
        let mut plates: Vec<Plate> = (0..seeds.len()).map(|_| {
            let axis = point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            Plate { omega: axis * speed * (0.4 + 1.2 * rng.next_f64()) }
        }).collect();
        // Plates carrying much continent move more slowly (they have deep keels).
        for (k, p) in plates.iter_mut().enumerate() {
            let (cont, all) = cells.iter().filter(|c| c.plate as usize == k).fold((0, 0), |a, c| (a.0 + c.continental as usize, a.1 + 1));
            if all > 0 && cont * 2 > all { p.omega *= 0.6; }
        }
        let hotspots = (0..params.hotspots).map(|_| point_on_unit_sphere(rng.next_f64(), rng.next_f64())).collect();
        let next_rift = params.rift_interval * (0.5 + rng.next_f64());
        Tectonics {
            uplift: vec![0.; n], volcanism: vec![0.; n], trench: vec![0.; n],
            rotation: vec![Rotation3::identity(); plates.len()], gap_since: vec![None; n],
            reference: cells.clone(), current: cells,
            domain, plates, hotspots, rng, time: 0., last_remesh: 0., next_rift, params,
        }
    }

    pub fn run(&mut self) {
        while self.time < self.params.age - 1e-9 {
            let dt = self.params.step.min(self.params.age - self.time);
            self.step(dt);
        }
    }

    fn radius(&self) -> f64 { self.domain.radius }

    /// Velocity of plate `p` at `d`, metres per million years.
    fn velocity(&self, p: usize, d: &Vector3<f64>) -> Vector3<f64> {
        self.plates[p].omega.cross(d) * self.radius()
    }

    pub fn step(&mut self, dt: f64) {
        let n = self.domain.len();
        for (r, p) in self.rotation.iter_mut().zip(&self.plates) {
            let w = p.omega.norm();
            if w > 0. { *r = Rotation3::from_axis_angle(&Unit::new_normalize(p.omega), w * dt) * *r; }
        }
        let inverse: Vec<Rotation3<f64>> = self.rotation.iter().map(|r| r.inverse()).collect();
        let reference = &self.reference;
        let domain = &self.domain;
        // Which plates cover each cell, and with which of their crust.
        let arrivals: Vec<Vec<(usize, usize)>> = (0..n).into_par_iter().map(|g| {
            let x = domain.dirs[g];
            inverse.iter().enumerate().filter_map(|(p, r)| {
                let src = domain.cell_at(&(r * x));
                (reference[src].plate as usize == p).then_some((p, src))
            }).collect()
        }).collect();

        let mut current = self.current.clone();
        // Which reference crust is on top at each cell.
        let mut source: Vec<Option<usize>> = vec![None; n];
        // Fronts: (cell, overriding plate, other plate, other continental, convergence speed).
        let mut fronts: Vec<(usize, usize, usize, bool, f64)> = vec![];
        let mut stacking: Vec<(usize, f64)> = vec![];
        for g in 0..n {
            let a = &arrivals[g];
            match a.len() {
                0 => {
                    // The plates have parted: new ocean floor at a ridge, on the plate that was
                    // here.
                    let since = *self.gap_since[g].get_or_insert(self.time);
                    current[g] = Cell { plate: self.current[g].plate, continental: false, age: self.time + dt - since, thickening: 0. };
                }
                _ => {
                    self.gap_since[g] = None;
                    // The most buoyant stays on top: continent, else the youngest ocean floor.
                    let buoyancy = |c: &Cell| if c.continental { 1e6 + c.thickening } else { -c.age };
                    let &(wp, ws) = a.iter().max_by(|x, y| buoyancy(&reference[x.1]).total_cmp(&buoyancy(&reference[y.1]))).unwrap();
                    source[g] = Some(ws);
                    for &(lp, ls) in a {
                        if lp == wp { continue; }
                        let under = reference[ls];
                        let x = domain.dirs[g];
                        let closing = (self.velocity(lp, &x) - self.velocity(wp, &x)).norm();
                        if under.continental && reference[ws].continental {
                            // Collision: the crust stacks up.
                            stacking.push((ws, 0.02 * dt * (under.thickening + CONTINENT_BASE)));
                        }
                        fronts.push((g, wp, lp, under.continental, closing));
                    }
                    current[g] = reference[ws];
                }
            }
        }

        // Effects of the fronts on the overriding plates: arcs behind subduction zones, broad
        // thickening behind collisions; trenches in front of subduction.
        let typical = self.params.speed;
        let mut uplift = vec![0.; n];
        let mut volcanism = vec![0.; n];
        let mut trench = vec![0.; n];
        let subduction: Vec<(usize, f64)> = fronts.iter().filter(|f| !f.3).map(|f| (f.0, f.4 / typical)).collect();
        let collision: Vec<(usize, f64)> = fronts.iter().filter(|f| f.3).map(|f| (f.0, f.4 / typical)).collect();
        for (g, s, reach) in self.spread(&current, &subduction, ARC_DISTANCE + 2. * ARC_WIDTH) {
            let f = (-((reach - ARC_DISTANCE) / ARC_WIDTH).powi(2)).exp();
            uplift[g] += self.params.subduction_uplift * s.min(3.) * f;
            volcanism[g] = f64::max(volcanism[g], f * s.min(1.));
        }
        for (g, s, reach) in self.spread(&current, &collision, COLLISION_REACH) {
            let f = (1. - reach / COLLISION_REACH).powi(2);
            uplift[g] += self.params.collision_uplift * s.min(3.) * f;
        }
        for &(g, s) in &subduction {
            // The trench lies just in front of the overriding plate, over the subducting one.
            for l in self.domain.links(g) {
                if current[l.to].plate != current[g].plate && !current[l.to].continental {
                    trench[l.to] = f64::min(trench[l.to], -2500. * s.min(1.5));
                }
            }
        }
        // Hotspots build volcanoes on whatever passes over them.
        for h in &self.hotspots {
            let g = self.domain.cell_at(h);
            uplift[g] += 1200.;
            volcanism[g] = 1.;
        }

        // The changes happen to the plates' own crust. A reference cell may show at several
        // grid cells: it takes the largest change.
        let mut gain = vec![0f64; n];
        for g in 0..n {
            if let Some(src) = source[g] { gain[src] = gain[src].max(uplift[g] * dt); }
        }
        for (src, x) in stacking { gain[src] = gain[src].max(x); }
        let decay = (-dt / self.params.relaxation).exp();
        let thicken = |c: &mut Cell, gain: f64| {
            c.age += dt;
            let room = (1. - c.thickening / MAX_THICKENING).max(0.);
            c.thickening = c.thickening * decay + gain * room;
        };
        for (c, &x) in self.reference.iter_mut().zip(&gain) { thicken(c, x); }
        for g in 0..n {
            match source[g] {
                Some(src) => current[g] = self.reference[src],
                None => { let x = uplift[g] * dt; thicken(&mut current[g], x); current[g].age -= dt; }
            }
        }
        if std::env::var("TECTONICS_DEBUG").is_ok() {
            let gaps = arrivals.iter().filter(|a| a.is_empty()).count();
            let speeds: Vec<String> = self.plates.iter().map(|p| format!("{:.0}", p.omega.norm() * self.domain.radius / 1000.)).collect();
            eprintln!("t={:.0} gaps={} fronts={} plates={:?}", self.time, gaps, fronts.len(), speeds);
        }
        self.current = current;
        self.uplift = uplift;
        self.volcanism = volcanism;
        self.trench = trench;

        // Continents that collide are sutured: the plates' motions converge, over about
        // `SUTURE_TIME` for a front ten cells long, until they move on as one.
        let mut sutures: Vec<((usize, usize), usize)> = {
            let mut m: HashMap<(usize, usize), usize> = HashMap::new();
            for f in fronts.iter().filter(|f| f.3) { *m.entry((f.1, f.2)).or_insert(0) += 1; }
            m.into_iter().collect()
        };
        sutures.sort();
        for ((over, under), count) in sutures {
            let a = (dt / SUTURE_TIME * count as f64 / 10.).min(1.);
            let (wo, wu) = (self.plates[over].omega, self.plates[under].omega);
            self.plates[under].omega = wu + (wo - wu) * a;
        }

        self.time += dt;
        let rifting = self.time >= self.next_rift;
        if rifting || self.time - self.last_remesh >= self.params.remesh - 1e-9 {
            self.remesh();
        }
        if rifting {
            self.rift();
            self.next_rift = self.time + self.params.rift_interval * (0.5 + self.rng.next_f64());
        }
    }

    /// Resample the plates onto the grid: the crust now at each cell becomes that cell's, and
    /// the gaps' new ocean floor joins the plates.
    fn remesh(&mut self) {
        // A cell on a different plate from nearly all its neighbours is an artefact of
        // resampling (or a ridge cell given to the wrong side): it joins its neighbours' plate.
        let n = self.domain.len();
        let mut tidy = self.current.clone();
        for g in 0..n {
            let links = self.domain.links(g);
            let mut votes: HashMap<u16, usize> = HashMap::new();
            for l in links { *votes.entry(self.current[l.to].plate).or_insert(0) += 1; }
            let own = votes.get(&self.current[g].plate).copied().unwrap_or(0);
            let (&best, &count) = votes.iter().max_by_key(|(p, c)| (**c, std::cmp::Reverse(**p))).unwrap();
            if own <= 1 && count * 2 > links.len() { tidy[g].plate = best; }
        }
        self.current = tidy;
        self.reference = self.current.clone();
        for r in self.rotation.iter_mut() { *r = Rotation3::identity(); }
        self.gap_since = vec![None; self.domain.len()];
        self.last_remesh = self.time;
    }

    /// Cells of the overriding plates within `reach` metres of fronts (cell, strength), by
    /// distance over the grid, staying on each front's own plate: (cell, strength, distance).
    fn spread(&self, cells: &[Cell], fronts: &[(usize, f64)], reach: f64) -> Vec<(usize, f64, f64)> {
        #[derive(PartialEq)]
        struct Item(f64, usize, f64);
        impl Eq for Item {}
        impl PartialOrd for Item { fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(o)) } }
        impl Ord for Item { fn cmp(&self, o: &Self) -> std::cmp::Ordering { o.0.total_cmp(&self.0) } }
        let n = self.domain.len();
        let mut best: Vec<(f64, f64)> = vec![(f64::INFINITY, 0.); n];
        let mut heap = BinaryHeap::new();
        for &(g, s) in fronts {
            best[g] = (0., best[g].1.max(s));
            heap.push(Item(0., g, s));
        }
        while let Some(Item(d, g, s)) = heap.pop() {
            if d > best[g].0 { continue; }
            for l in self.domain.links(g) {
                let nd = d + l.dist;
                if nd > reach || cells[l.to].plate != cells[g].plate { continue; }
                if nd < best[l.to].0 {
                    best[l.to] = (nd, s);
                    heap.push(Item(nd, l.to, s));
                }
            }
        }
        best.iter().enumerate().filter(|(_, b)| b.0.is_finite()).map(|(g, b)| (g, b.1, b.0)).collect()
    }

    /// Split the largest plate carrying continent in two, the halves moving apart.
    fn rift(&mut self) {
        let mut area = vec![(0usize, 0usize); self.plates.len()];
        for c in &self.reference { area[c.plate as usize].0 += 1; area[c.plate as usize].1 += c.continental as usize; }
        let Some(p) = (0..self.plates.len()).filter(|&k| area[k].1 > 50).max_by_key(|&k| area[k].0) else { return };
        let members: Vec<usize> = (0..self.reference.len()).filter(|&g| self.reference[g].plate as usize == p).collect();
        // Two seeds in the plate, far apart; the rift wanders between them.
        let a = self.domain.dirs[members[(self.rng.next_f64() * members.len() as f64) as usize % members.len()]];
        let b = members.iter().map(|&g| self.domain.dirs[g]).min_by(|x, y| x.dot(&a).total_cmp(&y.dot(&a))).unwrap();
        let noise = PerlinNoise::with_seed(self.rng.next_u32() as u64);
        let new = self.plates.len() as u16;
        let mut moved = vec![];
        for &g in &members {
            let d = self.domain.dirs[g];
            let wobble = 0.15 * (noise.noise(d.x * 5., d.y * 5., d.z * 5.) + 0.5 * noise.noise(d.x * 11., d.y * 11., d.z * 11.));
            if d.dot(&b) + wobble > d.dot(&a) { self.reference[g].plate = new; moved.push(d); }
        }
        if moved.is_empty() { return; }
        let centroid = |v: &[Vector3<f64>]| v.iter().sum::<Vector3<f64>>().normalize();
        let stay: Vec<Vector3<f64>> = (0..self.reference.len()).filter(|&g| self.reference[g].plate as usize == p).map(|g| self.domain.dirs[g]).collect();
        if stay.is_empty() { self.reference.iter_mut().filter(|c| c.plate == new).for_each(|c| c.plate = p as u16); return; }
        let (ca, cb) = (centroid(&stay), centroid(&moved));
        // Rotating about ca x cb carries ca towards cb: the new plate that way, the old the
        // other, at the typical speed.
        let axis = ca.cross(&cb);
        if axis.norm() < 1e-9 { return; }
        let push = axis.normalize() * 0.5 * self.params.speed * 1000. / self.radius();
        let base = self.plates[p].omega;
        self.plates[p].omega = base - push;
        self.plates.push(Plate { omega: base + push });
        self.rotation.push(Rotation3::identity());
        self.current = self.reference.clone();
    }

    /// The crust's present state, for the planet.
    pub fn into_crust(self) -> Crust {
        let corner = |field: &dyn Fn(usize) -> f64| -> Vec<f64> {
            self.domain.corner_cells.iter().map(|cs| cs.iter().map(|&c| field(c)).sum::<f64>() / cs.len() as f64).collect()
        };
        Crust {
            elevation: corner(&|g| self.current[g].elevation() + self.trench[g]),
            orogeny: corner(&|g| (self.current[g].thickening / 2500.).clamp(0., 1.) * if self.current[g].continental { 1. } else { 0.5 }),
            margin: corner(&|g| if self.current[g].continental { 1. } else { 0. }),
            activity: corner(&|g| (self.uplift[g] / self.params.subduction_uplift).clamp(0., 1.)),
            volcanism: corner(&|g| self.volcanism[g]),
            plate: self.current.iter().map(|c| c.plate).collect(),
            continental: self.current.iter().map(|c| c.continental).collect(),
            hotspots: self.hotspots.iter().map(|h| (*h, self.plates[self.current[self.domain.cell_at(h)].plate as usize].omega)).collect(),
            domain: self.domain,
        }
    }

    /// Fraction of the surface that is continent.
    pub fn continental_fraction(&self) -> f64 {
        self.current.iter().filter(|c| c.continental).count() as f64 / self.current.len() as f64
    }
}

/// The present-day crust, as smooth fields over the sphere.
pub struct Crust {
    domain: Domain,
    /// Elevation, metres, at the grid's corners.
    elevation: Vec<f64>,
    /// How thickened the crust is by mountain building, 0 to 1.
    orogeny: Vec<f64>,
    /// How fast it is being thickened now, 0 to 1.
    activity: Vec<f64>,
    /// Fraction of continental crust about each corner: 0 at sea, 1 inland, between at the
    /// continents' margins.
    margin: Vec<f64>,
    /// Volcanic activity, 0 to 1.
    volcanism: Vec<f64>,
    /// Each cell's plate, and whether it is continent.
    pub plate: Vec<u16>,
    pub continental: Vec<bool>,
    /// The hotspots, and the angular velocity of the plate over each (radians per million
    /// years, about the planet's centre): what their tracks of volcanoes follow
    /// (`terrain::hotspot`).
    pub hotspots: Vec<(Vector3<f64>, Vector3<f64>)>,
}

impl Crust {
    fn at(&self, field: &[f64], d: &Vector3<f64>) -> f64 { crate::terrain::scenery::sample(&self.domain, field, d) }
    pub fn elevation(&self, d: &Vector3<f64>) -> f64 { self.at(&self.elevation, d) }
    pub fn orogeny(&self, d: &Vector3<f64>) -> f64 { self.at(&self.orogeny, d) }
    pub fn activity(&self, d: &Vector3<f64>) -> f64 { self.at(&self.activity, d) }
    pub fn volcanism(&self, d: &Vector3<f64>) -> f64 { self.at(&self.volcanism, d) }
    pub fn margin(&self, d: &Vector3<f64>) -> f64 { self.at(&self.margin, d) }
    pub fn plate_at(&self, d: &Vector3<f64>) -> u16 { self.plate[self.domain.cell_at(d)] }
}

/// A crust's fields without its grid (which is rebuilt from the cells along a face's edge), to
/// save and restore: what a world keeps of its plate tectonics (`world`).
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CrustState {
    cells: usize,
    elevation: Vec<f64>,
    orogeny: Vec<f64>,
    activity: Vec<f64>,
    margin: Vec<f64>,
    volcanism: Vec<f64>,
    plate: Vec<u16>,
    continental: Vec<bool>,
    /// Each hotspot's direction and its plate's angular velocity: x, y, z, ωx, ωy, ωz.
    #[serde(default)]
    hotspots: Vec<[f64; 6]>,
}

impl Crust {
    pub fn state(&self) -> CrustState {
        let crate::terrain::domain::Layout::Sphere { n } = self.domain.layout else { unreachable!() };
        CrustState {
            cells: n, elevation: self.elevation.clone(), orogeny: self.orogeny.clone(), activity: self.activity.clone(),
            margin: self.margin.clone(), volcanism: self.volcanism.clone(), plate: self.plate.clone(), continental: self.continental.clone(),
            hotspots: self.hotspots.iter().map(|(h, w)| [h.x, h.y, h.z, w.x, w.y, w.z]).collect(),
        }
    }

    /// The crust a state was taken from, on a planet of `radius`; None if the state isn't of a
    /// grid of `cells` along a face's edge with `hotspots` hotspots (a state saved before they
    /// were kept has none).
    pub fn from_state(s: CrustState, cells: usize, hotspots: usize, radius: f64) -> Option<Crust> {
        if s.cells != cells || s.hotspots.len() != hotspots { return None; }
        let domain = Domain::sphere(cells, radius);
        let (corners, n) = (domain.corners.len(), domain.len());
        let fields = [&s.elevation, &s.orogeny, &s.activity, &s.margin, &s.volcanism];
        if fields.iter().any(|f| f.len() != corners) || s.plate.len() != n || s.continental.len() != n { return None; }
        Some(Crust {
            domain, elevation: s.elevation, orogeny: s.orogeny, activity: s.activity, margin: s.margin,
            volcanism: s.volcanism, plate: s.plate, continental: s.continental,
            hotspots: s.hotspots.iter().map(|h| (Vector3::new(h[0], h[1], h[2]), Vector3::new(h[3], h[4], h[5]))).collect(),
        })
    }
}

/// Simulate the plates for these parameters: the crust they leave, and a line about it.
/// `progress` is told the fraction of the planet's age simulated, after each step.
pub fn simulate(params: &TectonicsParams, seed: u64, radius: f64, progress: &mut dyn FnMut(f64)) -> (Crust, String) {
    let start = web_time::Instant::now();
    let mut t = Tectonics::new(params.clone(), seed, radius);
    while t.time < params.age - 1e-9 {
        let dt = params.step.min(params.age - t.time);
        t.step(dt);
        progress((t.time / params.age).min(1.));
    }
    let mut ages: Vec<f64> = t.current.iter().filter(|c| !c.continental).map(|c| c.age).collect();
    ages.sort_by(|a, b| a.total_cmp(b));
    let pct = |q: f64| ages.get((q * ages.len() as f64) as usize).copied().unwrap_or(0.);
    let summary = format!("Tectonics: {} plates over {} Myr in {:.1?}; {:.0}% continent; ocean floor ages {:.0}, {:.0}, {:.0} Myr (10, 50, 90%)",
        t.plates.len(), params.age, start.elapsed(), 100. * t.continental_fraction(), pct(0.1), pct(0.5), pct(0.9));
    (t.into_crust(), summary)
}

/// The crust for these parameters, simulated once per process and shared.
pub fn crust(params: &TectonicsParams, seed: u64, radius: f64) -> Arc<Crust> {
    crust_from(params, seed, radius, || {
        let (c, summary) = simulate(params, seed, radius, &mut |_| ());
        println!("{}", summary);
        c
    })
}

/// The crust for these parameters if this process has it, or else the one `make` returns (read
/// from a world's directory, say, or simulated with its progress reported), which it then
/// shares with every later caller.
pub fn crust_from(params: &TectonicsParams, seed: u64, radius: f64, make: impl FnOnce() -> Crust) -> Arc<Crust> {
    static CRUSTS: OnceLock<Mutex<HashMap<String, Arc<Crust>>>> = OnceLock::new();
    let key = format!("{:?} {} {}", params, seed, radius);
    let cache = CRUSTS.get_or_init(|| Mutex::new(HashMap::new()));
    // Held while simulating, so that threads asking for the same crust wait for it.
    let mut map = cache.lock().unwrap();
    if let Some(c) = map.get(&key) { return c.clone(); }
    let c = Arc::new(make());
    map.insert(key, c.clone());
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small() -> TectonicsParams {
        TectonicsParams { cells: 24, age: 100., ..TectonicsParams::default() }
    }

    #[test]
    fn test_ocean_floor_deepens_with_age() {
        assert_eq!(ocean_depth(0.), -2600.);
        // Deeper with age, a little less fast once the plate model takes over at 20 Myr, where
        // the two curves meet; flattening at 5.65 km less the sediment's half kilometre.
        let depths: Vec<f64> = (0..300).map(|t| ocean_depth(t as f64)).collect();
        assert!(depths.windows(2).all(|w| w[1] < w[0]), "not deepening");
        assert!((ocean_depth(19.999) - ocean_depth(20.001)).abs() < 5.);
        assert!(ocean_depth(25.) < -4200. && ocean_depth(25.) > -4500., "{}", ocean_depth(25.));
        assert!((ocean_depth(1000.) + 5651. - 285.).abs() < 1.);
    }

    #[test]
    fn test_plates_move_and_make_ridges_trenches_and_mountains() {
        let mut t = Tectonics::new(small(), 1, 6.371e6);
        let start: Vec<bool> = t.current.iter().map(|c| c.continental).collect();
        t.run();
        assert!(t.current.iter().zip(&start).filter(|(c, s)| c.continental != **s).count() > 100, "continents didn't move");
        // New ocean floor at ridges, mountains on continents, trenches.
        assert!(t.current.iter().any(|c| !c.continental && c.age < 10.));
        assert!(t.current.iter().any(|c| c.continental && c.thickening > 1000.));
        assert!(t.trench.iter().any(|&x| x < -1000.));
        // Rifting made new plates; continents survive.
        assert!(t.plates.len() > small().plates);
        let f = t.continental_fraction();
        assert!(f > 0.15 && f < 0.5, "{}", f);
    }

    #[test]
    fn test_crust_has_oceans_continents_and_is_deterministic() {
        let a = crust(&small(), 2, 6.371e6);
        let mut t = Tectonics::new(small(), 2, 6.371e6);
        t.run();
        let b = t.into_crust();
        assert_eq!(a.elevation, b.elevation);
        let mut rng = Pcg32::new(3, 3);
        let e: Vec<f64> = (0..1000).map(|_| a.elevation(&point_on_unit_sphere(rng.next_f64(), rng.next_f64()))).collect();
        let sea = e.iter().filter(|&&x| x < -1000.).count();
        let land = e.iter().filter(|&&x| x > 0.).count();
        assert!(sea > 400 && land > 150, "sea {} land {}", sea, land);
        // Its hotspots, each with the motion of the plate over it, kept through a save.
        assert_eq!(a.hotspots.len(), small().hotspots);
        assert!(a.hotspots.iter().all(|(h, w)| (h.norm() - 1.).abs() < 1e-9 && w.norm() > 0.));
        let back = Crust::from_state(a.state(), small().cells, small().hotspots, 6.371e6).unwrap();
        assert_eq!(back.hotspots, a.hotspots);
    }
}
