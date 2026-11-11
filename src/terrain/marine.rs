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
//! 1. the sediment flux each river brings to a sea cell is laid there;
//! 2. the sea floor diffuses between sea cells, but only sediment moves: each cell's outflow is
//!    limited to the sediment it holds (the rock of the initial sea floor stays put), with the
//!    same conservative limiter as the ice (`glacier`).
//!
//! A sea cell built above sea level is land (`Terrain::outlets` counts only sea below sea
//! level), so rivers then flow across the new delta plain to the next cell of sea, and the delta
//! progrades. Sediment reaching a patch's edge still leaves the model.

use crate::terrain::domain::Domain;

#[derive(Clone, Debug)]
pub struct MarineParams {
    /// Diffusivity at the shore, K0, square metres per year.
    pub diffusivity: f64,
    /// Depth over which diffusivity falls by e, λ, metres.
    pub depth_scale: f64,
}

impl Default for MarineParams {
    fn default() -> MarineParams {
        MarineParams { diffusivity: 1000., depth_scale: 100. }
    }
}

/// Lay `supply` (m³/yr arriving in each cell) on the sea floor, and spread the sediment by
/// depth-dependent diffusion among the `sea` cells, for `dt` years. `ground` is updated;
/// `sediment` is the thickness of sediment there before the step (only sediment moves).
pub fn deposit(domain: &Domain, ground: &mut [f64], sediment: &[f64], sea: &[bool], supply: &[f64], p: &MarineParams, dt: f64) {
    let n = domain.len();
    let mut loose: Vec<f64> = sediment.to_vec();
    for i in 0..n {
        if sea[i] && supply[i] > 0. {
            let h = supply[i] * dt / domain.area[i];
            ground[i] += h;
            loose[i] += h;
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
        let depth = (-(0.5 * (a + b))).max(0.);
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
            deposit(&d, &mut ground, &sediment, &sea, &supply, &MarineParams::default(), 100.);
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
    }
}
