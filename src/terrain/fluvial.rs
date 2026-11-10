//! # Rivers and hillslopes
//!
//! ## River incision: the stream power law
//!
//! Where rivers cut into bedrock faster than sediment can bury it ("detachment-limited", Howard,
//! "A detachment-limited model of drainage basin evolution", Water Resources Research 30(7),
//! 1994, doi:10.1029/94WR00757), the rate of incision is modelled as a power of the river's
//! discharge (by proxy, its drainage area A) and its slope S:
//!
//! ```text
//! ∂h/∂t = U - K A^m S^n
//! ```
//!
//! Whipple and Tucker ("Dynamics of the stream-power river incision model", JGR 104(B8), 1999,
//! doi:10.1029/1999JB900120) discuss its behaviour and parameters: m/n is typically 0.35-0.6,
//! commonly 0.5, while n itself is poorly known (n = 1 is the usual choice). With uplift U
//! balanced by incision, the steady-state river profile follows
//!
//! ```text
//! S = (U/K)^(1/n) A^(-m/n)
//! ```
//!
//! so slopes shallow downstream as area grows: the concave river profiles of real ranges. The
//! test `test_steady_state_slope_area` checks this relation.
//!
//! The law is a wave equation: a change in base level travels upstream as a knickpoint.
//! Explicit schemes are limited by a Courant condition that, for the steep fast-eroding cells
//! near peaks, demands tiny steps. Braun and Willett (Geomorphology 180-181, 2013,
//! doi:10.1016/j.geomorph.2012.10.008) discretise implicitly along the receiver tree, where it
//! becomes one equation per cell given its receiver's new height. For n = 1:
//!
//! ```text
//! h_i(t+Δt) = (h_i(t) + F h_r(t+Δt)) / (1 + F),   F = K Δt A^m / L
//! ```
//!
//! with h_i(t) already uplifted, and L the distance to the receiver r. Walking the stack
//! receivers-first, h_r(t+Δt) is always known. Any Δt is stable. For n ≠ 1 each cell's equation
//! is solved by Newton-Raphson. A cell no higher than its receiver (on a lake bed) is not
//! incised.
//!
//! Not modelled: sediment transport and deposition (the transport-limited regime), so valleys
//! never fill and nothing is built at river mouths; thresholds for incision; precipitation
//! varying over the planet (drainage area is a proxy for discharge only under uniform rain).
//!
//! ## Hillslopes: linear diffusion
//!
//! Between channels, soil creeps downslope at a rate proportional to slope (Culling,
//! "Analytical Theory of Erosion", J. Geology 68(3), 1960, doi:10.1086/626663), so elevation
//! diffuses: ∂h/∂t = κ∇²h. Fernandes and Dietrich ("Hillslope evolution by diffusive processes",
//! WRR 33(6), 1997, doi:10.1029/97WR00534) discuss its timescales and κ. Roering, Kirchner and
//! Dietrich (WRR 35(3), 1999, doi:10.1029/1998WR900090) show that real flux grows without bound
//! near a critical slope; the linear law here does not.
//!
//! On the cell graph, the flux between cells sharing an edge of width w, centres d apart, is
//! κ w (h_j - h_i) / d (a finite-volume Laplacian). Steps are explicit, sub-divided to keep
//! within the stability limit.

use crate::terrain::domain::Domain;
use crate::terrain::flow::Flow;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct FluvialParams {
    /// Erodibility K, in m^(1-2m) per year.
    pub k: f64,
    pub m: f64,
    pub n: f64,
    /// Hillslope diffusivity κ, square metres per year.
    pub diffusivity: f64,
}

impl Default for FluvialParams {
    fn default() -> FluvialParams {
        FluvialParams { k: 1e-5, m: 0.5, n: 1., diffusivity: 0.01 }
    }
}

/// Incise rivers for `dt` years. `h` has already been uplifted. Cells marked `fixed` (outlets)
/// don't change; `erodibility` scales K per cell (eg. 0 under ice).
pub fn incise(flow: &Flow, h: &mut [f64], fixed: &[bool], erodibility: &[f64], p: &FluvialParams, dt: f64) {
    for &i in &flow.stack {
        let r = flow.receiver[i];
        if r == i || fixed[i] || erodibility[i] <= 0. { continue; }
        let hr = h[r];
        let h0 = h[i];
        if h0 <= hr { continue; }
        let f = p.k * erodibility[i] * dt * flow.area[i].powf(p.m) / flow.receiver_dist[i].powf(p.n);
        h[i] = if p.n == 1. {
            (h0 + f * hr) / (1. + f)
        } else {
            // Solve x - h0 + f (x - hr)^n = 0 for x in (hr, h0].
            let mut x = h0;
            for _ in 0..50 {
                let s = (x - hr).max(0.);
                let g = x - h0 + f * s.powf(p.n);
                let dg = 1. + p.n * f * s.powf(p.n - 1.);
                let next = (x - g / dg).max(hr + 1e-9 * (h0 - hr));
                let done = (next - x).abs() < 1e-9 * (1. + x.abs());
                x = next;
                if done { break; }
            }
            x
        };
    }
}

/// Diffuse hillslopes for `dt` years. Cells marked `fixed` don't change.
pub fn diffuse(domain: &Domain, h: &mut [f64], fixed: &[bool], kappa: f64, dt: f64) {
    if kappa <= 0. || dt <= 0. { return; }
    // Explicit diffusion is stable while each cell gives away at most half its excess per step.
    let limit = (0..domain.len()).map(|i| {
        let conductance: f64 = domain.links(i).iter().map(|l| kappa * l.width / l.dist).sum();
        if conductance > 0. { 0.5 * domain.area[i] / conductance } else { f64::INFINITY }
    }).fold(f64::INFINITY, f64::min);
    let steps = (dt / limit).ceil().max(1.) as usize;
    let step = dt / steps as f64;
    let mut next = h.to_vec();
    for _ in 0..steps {
        next.par_iter_mut().enumerate().for_each(|(i, x)| {
            if fixed[i] { *x = h[i]; return; }
            let flux: f64 = domain.links(i).iter().map(|l| kappa * l.width * (h[l.to] - h[i]) / l.dist).sum();
            *x = h[i] + step * flux / domain.area[i];
        });
        h.copy_from_slice(&next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::flow::route;

    #[test]
    fn test_steady_state_slope_area() {
        for n in [1., 2.] {
            let d = Domain::patch(0., 0., 12, 100., 6.371e6);
            let p = FluvialParams { k: 1e-4, m: 0.5, n, diffusivity: 0. };
            let u = 1e-3;
            let mut h = vec![0.; d.len()];
            let ones = vec![1.; d.len()];
            let dt = 1000.;
            for _ in 0..3000 {
                for i in 0..d.len() { if !d.boundary[i] { h[i] += u * dt; } }
                let f = route(&d, &h, &d.boundary, &ones);
                incise(&f, &mut h, &d.boundary, &ones, &p, dt);
            }
            let f = route(&d, &h, &d.boundary, &ones);
            for i in 0..d.len() {
                if d.boundary[i] { continue; }
                let r = f.receiver[i];
                let s = (h[i] - h[r]) / f.receiver_dist[i];
                let expected = (u / p.k).powf(1. / n) * f.area[i].powf(-p.m / n);
                assert!((s - expected).abs() < 1e-3 * expected, "n {} cell {}: {} vs {}", n, i, s, expected);
            }
        }
    }

    #[test]
    fn test_incision_never_cuts_below_receiver() {
        let d = Domain::patch(0., 0., 8, 10., 6.371e6);
        let mut h: Vec<f64> = (0..64).map(|k| ((k * 37) % 11) as f64 * 10.).collect();
        let ones = vec![1.; 64];
        let f = route(&d, &h, &d.boundary, &ones);
        let p = FluvialParams { k: 1., ..FluvialParams::default() };
        incise(&f, &mut h, &d.boundary, &ones, &p, 1e6);
        for i in 0..64 {
            assert!(h[i] >= h[f.receiver[i]] - 1e-9 || h[i] < f.filled[i]);
        }
    }

    #[test]
    fn test_diffusion_conserves_mass_and_smooths() {
        let d = Domain::sphere(6, 1000.);
        let mut h: Vec<f64> = (0..d.len()).map(|i| if i == 40 { 100. } else { 0. }).collect();
        let mass = |h: &[f64]| h.iter().zip(&d.area).map(|(a, b)| a * b).sum::<f64>();
        let before = mass(&h);
        diffuse(&d, &mut h, &vec![false; d.len()], 1., 1e4);
        assert!((mass(&h) - before).abs() < 1e-9 * before);
        assert!(h[40] < 100. && h.iter().all(|&x| x >= -1e-9));
    }
}
