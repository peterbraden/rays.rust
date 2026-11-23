//! # The sea floor: deltas
//!
//! Sediment rivers carry to the coast doesn't vanish: it settles at the river mouth and is
//! spread offshore by waves and currents, building deltas that push the coastline seawards.
//! Stratigraphic models commonly treat this spreading as diffusion of the sea floor whose
//! diffusivity falls with water depth, strongest in the shallows where waves stir the bed
//! (Kaufman, Grotzinger and McCormick, "Depth-dependent diffusion algorithm for simulation of
//! sedimentation in shallow marine depositional systems", Kansas Geological Survey Bulletin
//! 233, 1991; Rivenæs, "Application of a dual-lithology, depth-dependent diffusion equation in
//! stratigraphic simulation", Basin Research 4, 1992, doi:10.1111/j.1365-2117.1992.tb00136.x):
//!
//! ```text
//! ∂h/∂t = ∇·(K(d) ∇h),   K(d) = K0 exp(-d / λ)
//! ```
//!
//! for water depth d. K0 = 1000 m²/yr and λ = 100 m here (typical values are 10²-10⁴ m²/yr and
//! 10-100 m; to verify). For comparison Yuan et al. ("Linking continental erosion to marine
//! sediment transport and deposition", EPSL 524, 2019, doi:10.1016/j.epsl.2019.115728), with a
//! constant diffusivity per grain size, fit 300-500 m²/yr for silt on the Ogooué delta. Here:
//!
//! 1. the sediment flux each river brings to a sea cell fills the sea floor up to sea level,
//!    from the mouth outwards, nearest cells first (nearer along the river's jet, straight on
//!    into the sea, than to the sides: `jet`), so the delta's plain builds at sea level
//!    and its front advances (`Near`); what reaches the open sea beyond (a patch's edge)
//!    leaves, and a lagoon with no room left builds up evenly into a plain;
//! 2. the sea floor diffuses between sea cells, but only sediment moves: each cell's outflow is
//!    limited to the sediment it holds (the rock of the initial sea floor stays put), with the
//!    same conservative limiter as the ice (`glacier`).
//!
//! A sea cell built above sea level is land (`Terrain::outlets` counts only sea below sea
//! level), so rivers then flow across the new delta plain to the next cell of sea, and the delta
//! progrades. Sediment reaching a patch's edge still leaves the model.

use crate::terrain::domain::Domain;

/// Cut back the coast for `dt` years: each land cell beside the `sea` (`fixed` cells don't
/// change) retreats at `cliff_retreat` times its rock's `erodibility` times the share of its
/// edge open to the sea, and is lowered by the volume that removes (the retreat times its
/// height above `sea_level` times the cell's width), never below sea level. The rock goes to
/// the sea cells it faces, by edge, as sediment: what returns is the supply to add, m³/yr.
///
/// Waves break against the foot of a cliff and quarry and abrade it until the cliff above
/// falls; the rate depends on the rock and on the waves (Sunamura, "Geomorphology of Rocky
/// Coasts", Wiley, 1992: from metres a year in glacial till and chalk to millimetres in
/// granite). Behind a retreating cliff is the shore platform it was cut back across, at about
/// sea level.
#[allow(clippy::too_many_arguments)]
pub fn erode_coast(domain: &Domain, ground: &mut [f64], fixed: &[bool], sea: &[bool], erodibility: &[f64], p: &MarineParams, sea_level: f64, dt: f64) -> Vec<f64> {
    let n = domain.len();
    let mut supply = vec![0.; n];
    if p.cliff_retreat <= 0. { return supply; }
    for i in 0..n {
        if fixed[i] || sea[i] { continue; }
        let height = ground[i] - sea_level;
        if height <= 0. { continue; }
        let edges = domain.edges(i);
        let facing: f64 = edges.iter().filter(|l| sea[l.to]).map(|l| l.width).sum();
        if facing <= 0. { continue; }
        let perimeter: f64 = edges.iter().map(|l| l.width).sum();
        let retreat = p.cliff_retreat * erodibility[i] * (facing / perimeter) * dt;
        // The volume of cliff the retreat takes, over the width that faces the sea.
        let lowered = (retreat * height * facing / domain.area[i]).min(height);
        ground[i] -= lowered;
        let volume = lowered * domain.area[i];
        for l in edges.iter().filter(|l| sea[l.to]) {
            supply[l.to] += volume * l.width / facing / dt;
        }
    }
    supply
}

/// A cell and its distance from a river's mouth, nearest first in a `BinaryHeap`.
#[derive(PartialEq)]
struct Near(f64, usize);
impl Eq for Near {}
impl PartialOrd for Near { fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(o)) } }
impl Ord for Near { fn cmp(&self, o: &Self) -> std::cmp::Ordering { o.0.total_cmp(&self.0).then(o.1.cmp(&self.1)) } }

#[derive(Clone, Debug)]
pub struct MarineParams {
    /// Diffusivity at the shore, K0, square metres per year.
    pub diffusivity: f64,
    /// Depth over which diffusivity falls by e, λ, metres.
    pub depth_scale: f64,
    /// How fast waves cut back cliffs of average bedrock, metres a year (scaled by the rock's
    /// erodibility, and by how much of the cell's edge faces the sea); 0 for none.
    pub cliff_retreat: f64,
    /// How far rivers' jets carry their sediment straight on into the sea rather than to the
    /// sides: a sideways step costs 1 + jet times a step straight on as the delta fills (0 for
    /// round lobes, as waves spread them; higher for the fingers of river-dominated deltas).
    pub jet: f64,
}

impl Default for MarineParams {
    fn default() -> MarineParams {
        MarineParams { diffusivity: 1000., depth_scale: 100., jet: 3., cliff_retreat: 0.05 }
    }
}

/// Lay `supply` (m³/yr arriving in each cell) on the sea floor, and spread the sediment by
/// depth-dependent diffusion among the `sea` cells, for `dt` years. `ground` is updated;
/// `sediment` is the thickness of sediment there before the step (only sediment moves).
///
/// `open` marks the open sea beyond: cells (eg. at a patch's edge, below sea level) that
/// sediment reaching leaves by.
#[allow(clippy::too_many_arguments)]
///
/// `heading` is the way each mouth's river enters it (a unit vector tangent to the sphere at
/// the mouth; zero for none), along which its jet carries the sediment (`jet`).
///
/// The sea stands at `sea_level` metres (see `Terrain::sea_level`).
pub fn deposit(domain: &Domain, ground: &mut [f64], sediment: &[f64], sea: &[bool], open: &[bool], supply: &[f64], heading: &[crate::na::Vector3<f64>], p: &MarineParams, sea_level: f64, dt: f64) {
    let n = domain.len();
    let mut loose: Vec<f64> = sediment.to_vec();
    // Each river's sediment fills the sea floor up to sea level, from its mouth outwards, the
    // nearest cells first: the delta's plain builds at sea level and its front advances.
    // (Laid all at the mouth, it piled hundreds of metres above the sea where the mouth was a
    // narrow inlet the spreading couldn't drain.)
    for mouth in 0..n {
        if !sea[mouth] || supply[mouth] <= 0. { continue; }
        let mut left = supply[mouth] * dt;
        let mut best: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Near(0., mouth));
        best.insert(mouth, 0.);
        let mut reached = vec![];
        while let Some(Near(d, c)) = heap.pop() {
            if left <= 0. { break; }
            if d > best[&c] { continue; }
            reached.push(c);
            let room = (sea_level - ground[c]).max(0.) * domain.area[c];
            let laid = room.min(left);
            if laid > 0. {
                let h = laid / domain.area[c];
                ground[c] += h;
                loose[c] += h;
                left -= laid;
            }
            // What reaches the open sea leaves.
            if domain.edges(c).iter().any(|l| open[l.to]) { left = 0.; break; }
            for l in domain.edges(c) {
                if !sea[l.to] { continue; }
                // Straight on along the river's jet is cheapest, to the sides dearer, back
                // dearer still.
                let step = domain.dirs[l.to] - domain.dirs[c];
                let ahead = if heading[mouth].norm_squared() > 0. && step.norm_squared() > 0. { heading[mouth].dot(&step.normalize()) } else { 1. };
                let nd = d + l.dist * (1. + p.jet * (1. - ahead));
                if best.get(&l.to).is_none_or(|&b| nd < b) {
                    best.insert(l.to, nd);
                    heap.push(Near(nd, l.to));
                }
            }
        }
        // A lagoon with no room left builds up evenly into a plain, rather than piling at the
        // mouth (which stood hundreds of metres above the sea).
        if left > 0. {
            let area: f64 = reached.iter().map(|&c| domain.area[c]).sum();
            for &c in &reached {
                ground[c] += left / area;
                loose[c] += left / area;
            }
        }
    }
    if p.diffusivity <= 0. { return; }

    // Explicit steps, each moving no more than half a cell's excess.
    let limit = (0..n).filter(|&i| sea[i]).map(|i| {
        let c: f64 = domain.edges(i).iter().filter(|l| sea[l.to]).map(|l| p.diffusivity * l.width / l.dist).sum();
        if c > 0. { 0.5 * domain.area[i] / c } else { f64::INFINITY }
    }).fold(f64::INFINITY, f64::min);
    if !limit.is_finite() { return; }
    let steps = (dt / limit).ceil().max(1.) as usize;
    let step = dt / steps as f64;
    let k = |a: f64, b: f64| {
        let depth = (sea_level - 0.5 * (a + b)).max(0.);
        p.diffusivity * (-depth / p.depth_scale).exp()
    };
    let mut flux = vec![0.; domain.link_count()];
    for _ in 0..steps {
        // Volume each link would carry out of its cell, and the share of it the cell can give.
        let mut scale = vec![1.; n];
        for i in 0..n {
            if !sea[i] { continue; }
            let o = domain.link_offset(i);
            let mut out = 0.;
            for (j, l) in domain.edges(i).iter().enumerate() {
                let q = if sea[l.to] { k(ground[i], ground[l.to]) * l.width * (ground[i] - ground[l.to]) / l.dist * step } else { 0. };
                flux[o + j] = q;
                out += q.max(0.);
            }
            let available = loose[i].max(0.) * domain.area[i];
            if out > available { scale[i] = available / out; }
        }
        let mut change = vec![0.; n];
        for i in 0..n {
            if !sea[i] { continue; }
            let o = domain.link_offset(i);
            for (j, l) in domain.edges(i).iter().enumerate() {
                let q = flux[o + j];
                if q > 0. { change[i] -= q * scale[i]; change[l.to] += q * scale[i]; }
            }
        }
        for i in 0..n {
            let h = change[i] / domain.area[i];
            ground[i] += h;
            loose[i] += h;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_spreads_sediment_and_conserves_it() {
        let d = Domain::patch(0., 0., 20, 500., 6.371e6);
        // A flat shelf 20 m deep; the river enters at the western edge's middle.
        let mut ground = vec![-20.; d.len()];
        let sea = vec![true; d.len()];
        let mouth = 10 * 20 + 1;
        let mut supply = vec![0.; d.len()];
        supply[mouth] = 2e5;
        let before: f64 = ground.iter().sum();
        let mut sediment = vec![0.; d.len()];
        for _ in 0..100 {
            let start = ground.clone();
            deposit(&d, &mut ground, &sediment, &sea, &vec![false; d.len()], &supply, &vec![crate::na::Vector3::zeros(); d.len()], &MarineParams::default(), 0., 100.);
            for i in 0..d.len() { sediment[i] += ground[i] - start[i]; }
        }
        let added: f64 = ground.iter().sum::<f64>() - before;
        assert!((added * 2.5e5 - 2e5 * 100. * 100.).abs() < 1e-6 * 2e9);
        // The deposit has spread well beyond the mouth, thinning away from it, and up to
        // sea level near the mouth (a delta plain).
        assert!(sediment[mouth + 4] > 1. && sediment[mouth + 4] > sediment[mouth + 8], "{:?}", &sediment[mouth..mouth + 10]);
        assert!(sediment[mouth + 3 * 20] > 1.);
        assert!(ground[mouth] > -1., "{}", ground[mouth]);
        // Only sediment moves: no cell is lowered below where it started.
        assert!(sediment.iter().all(|&s| s >= -1e-9));
        // And none is built far above sea level.
        assert!(ground.iter().all(|&g| g < 1.), "{}", ground.iter().cloned().fold(f64::MIN, f64::max));
    }

    #[test]
    fn test_a_rivers_jet_builds_its_delta_out_along_its_course() {
        // A river entering a deep shelf heading east: with a jet, its deposit reaches further
        // east than to the north or south; without, as far each way.
        let d = Domain::patch(0., 0., 40, 500., 6.371e6);
        let n = d.len();
        let at = |i: usize, j: usize| j * 40 + i;
        let mouth = at(2, 20);
        let mut supply = vec![0.; n];
        supply[mouth] = 1e6; // 10^8 m³: twenty cells 20 m deep
        let east = {
            let (dm, de) = (d.dirs[mouth], d.dirs[at(3, 20)]);
            (de - dm).normalize()
        };
        let mut heading = vec![crate::na::Vector3::zeros(); n];
        heading[mouth] = east;
        let reach = |jet: f64| {
            let mut ground = vec![-20.; n];
            let p = MarineParams { jet, diffusivity: 0., ..MarineParams::default() };
            deposit(&d, &mut ground, &vec![0.; n], &vec![true; n], &vec![false; n], &supply, &heading, &p, 0., 100.);
            let filled = |k: usize| ground[k] > -19.;
            let along = (2..40).take_while(|&i| filled(at(i, 20))).count();
            let across = (20..40).take_while(|&j| filled(at(2, j))).count();
            (along, across)
        };
        let (along, across) = reach(3.);
        assert!(along > 2 * across, "along {} across {}", along, across);
        let (along, across) = reach(0.);
        assert!(along <= across + 1, "along {} across {}", along, across);
    }

    #[test]
    fn test_waves_cut_back_soft_cliffs_faster_than_hard() {
        // A coast running north-south, land 30 m high to the east of sea: soft rock in the
        // north half, hard in the south. The rock cut goes to the sea, as sediment.
        let d = Domain::patch(0., 0., 20, 100., 6.371e6);
        let n = d.len();
        let sea: Vec<bool> = (0..n).map(|k| k % 20 < 10).collect();
        let mut ground: Vec<f64> = (0..n).map(|k| if sea[k] { -10. } else { 30. }).collect();
        let erodibility: Vec<f64> = (0..n).map(|k| if k / 20 < 10 { 0.3 } else { 3. }).collect();
        let before = ground.clone();
        let p = MarineParams { cliff_retreat: 0.05, ..MarineParams::default() };
        let supply = erode_coast(&d, &mut ground, &vec![false; n], &sea, &erodibility, &p, 0., 100.);
        let (hard, soft) = (5 * 20 + 10, 15 * 20 + 10);
        assert!(before[soft] - ground[soft] > 5. * (before[hard] - ground[hard]) && ground[hard] < before[hard]);
        // Only the cells facing the sea, and never below sea level.
        assert_eq!(ground[5 * 20 + 11], before[5 * 20 + 11]);
        assert!(ground.iter().zip(&sea).all(|(g, &s)| s || *g >= 0.));
        let lost: f64 = (0..n).map(|k| (before[k] - ground[k]) * d.area[k]).sum();
        assert!((lost - supply.iter().sum::<f64>() * 100.).abs() < 1e-6 * lost);
    }

    #[test]
    fn test_a_narrow_inlet_fills_to_sea_level_and_passes_the_rest_on() {
        // A river entering a one-cell-wide inlet that opens on to a bay: the inlet fills to sea
        // level, and the rest goes on into the bay, rather than piling up at the mouth.
        let d = Domain::patch(0., 0., 20, 500., 6.371e6);
        let n = d.len();
        let at = |i: usize, j: usize| j * 20 + i;
        let mut sea = vec![false; n];
        let mut ground = vec![50.; n];
        for i in 2..10 { sea[at(i, 10)] = true; ground[at(i, 10)] = -10.; }
        for j in 2..18 { for i in 10..18 { sea[at(i, j)] = true; ground[at(i, j)] = -30.; } }
        let mut supply = vec![0.; n];
        // 10^8 m³, five times what the inlet holds.
        supply[at(2, 10)] = 1e6;
        let before: f64 = ground.iter().sum();
        deposit(&d, &mut ground, &vec![0.; n], &sea, &vec![false; n], &supply, &vec![crate::na::Vector3::zeros(); n], &MarineParams::default(), 0., 100.);
        let added = (ground.iter().sum::<f64>() - before) * 2.5e5;
        assert!((added - 1e8).abs() < 1e-6 * 1e8);
        assert!(ground[at(2, 10)] <= 1e-9, "the mouth piled up to {}", ground[at(2, 10)]);
        assert!(ground[at(10, 10)] > -30., "nothing reached the bay");
    }
}
