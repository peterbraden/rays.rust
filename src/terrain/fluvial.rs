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
//! ## Sediment: erosion and deposition
//!
//! What rivers erode they carry downstream, and drop where they slow: on gentle valley floors,
//! building alluvial plains, and in lakes, filling them. Davy and Lague ("Fluvial
//! erosion/transport equation of landscape evolution models revisited", JGR Earth Surface 114,
//! 2009, doi:10.1029/2008JF001146) model the deposition rate as the sediment flux over a
//! deposition length that grows with discharge. Yuan, Braun, Guerit, Rouby and Cordonnier ("A
//! New Efficient Method to Solve the Stream Power Law Model Taking Into Account Sediment
//! Deposition", JGR Earth Surface 124(6), 2019, doi:10.1029/2018JF004867) write it as
//!
//! ```text
//! ∂h/∂t = U - K A^m S^n + (G / A) Q_s,   Q_s = ∫_upstream (U - ∂h/∂t) dA
//! ```
//!
//! where Q_s is the sediment flux (m³/yr) and G a dimensionless deposition coefficient: G = 0
//! is the detachment-limited model above. Guerit et al. (Geology 47(9), 2019,
//! doi:10.1130/G46356.1) find that from G ≈ 0.4 rivers behave as transport-limited, and that
//! most natural systems lie there; the default here is G = 1.
//!
//! Yuan et al. keep the implicit solve along the stack and iterate on Q_s: heights receivers
//! first given the last Q_s, then Q_s donors first given the heights, until the heights stop
//! changing. Here Q_s at a cell includes the cell's own erosion, which is linear in its new
//! height, so it is folded into each cell's equation. At steady state with uniform uplift every
//! cell exports what is uplifted upstream, Q_s = U A, so the slope-area relation becomes
//! S = ((1 + G) U / K)^(1/n) A^(-m/n) (derived, and checked by `test_steady_state_slope_area`).
//!
//! Lakes (cells under the Priority-Flood water level) trap all the sediment reaching them, up to
//! their depth, passing the rest on towards their outlet, as FastScape and goSPL do; over time
//! lakes fill and become flat alluvial plains. A lake's sediment is pooled and fills it flat
//! from its deepest point (no deltas: filling at the inlets built cliffs of sediment a
//! kilometre high at the edges of deep lakes on coarse grids). Sediment reaching the sea or a patch's edge
//! leaves the model (there are no deltas or marine deposits).
//!
//! Not modelled: thresholds for incision; precipitation varying over the planet (drainage area
//! is a proxy for discharge only under uniform rain); lateral erosion by meandering rivers,
//! which widens real alluvial valleys.
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
    /// Dimensionless deposition coefficient G (0: no deposition, detachment-limited).
    pub deposition: f64,
}

impl Default for FluvialParams {
    fn default() -> FluvialParams {
        FluvialParams { k: 1e-5, m: 0.5, n: 1., diffusivity: 0.01, deposition: 1. }
    }
}

/// Lake depth (metres of water over the surface) above which a cell counts as lake bed.
pub const LAKE: f64 = 0.01;

/// Erode rivers, and deposit their sediment, for `dt` years (see the module docs). `h` has
/// already been uplifted. Cells marked `fixed` (outlets) don't change. `erodibility` scales K
/// per cell; where it is 0 (eg. under ice) nothing is eroded or deposited and sediment passes
/// through. `lake` is the depth of water standing over each cell (the filled surface less the
/// surface routed over); `source` adds sediment to the rivers in a cell, m³/yr (eg. glacial
/// meltwater). Returns the sediment flux leaving each cell, m³/yr.
#[allow(clippy::too_many_arguments)]
pub fn erode_and_deposit(domain: &Domain, flow: &Flow, h: &mut [f64], fixed: &[bool], erodibility: &[f64], lake: &[f64], source: &[f64], p: &FluvialParams, dt: f64) -> Vec<f64> {
    let n = h.len();
    let h0 = h.to_vec();
    let is_lake = |i: usize| lake[i] > LAKE && erodibility[i] > 0. && !fixed[i];

    // Group lake cells by where their lake spills: follow receivers out of the lake.
    let mut exit = vec![usize::MAX; n];
    let mut lakes: std::collections::BTreeMap<usize, Vec<usize>> = Default::default();
    for &i in &flow.stack {
        if !is_lake(i) { continue; }
        let r = flow.receiver[i];
        exit[i] = if r != i && is_lake(r) { exit[r] } else { r };
        lakes.entry(exit[i]).or_default().push(i);
    }
    for cells in lakes.values_mut() { cells.sort_by(|&a, &b| h0[a].total_cmp(&h0[b]).then(a.cmp(&b))); }

    let mut q_in = vec![0.; n];
    let mut q_out = vec![0.; n];
    let mut pool = vec![0.; n];
    let max_iterations = if p.deposition > 0. { 50 } else { 1 };
    for _ in 0..max_iterations {
        let before = h.to_vec();
        // Heights, receivers first, given the sediment arriving from the last iteration.
        for &i in &flow.stack {
            let r = flow.receiver[i];
            if r == i || fixed[i] || erodibility[i] <= 0. || is_lake(i) { continue; }
            // Deposition G Q / A, where Q = q_in + a (h0 - x) / Δt includes the cell's own
            // erosion: with g = G a / A it rearranges to the incision equation with h0 raised
            // by Δt G q_in / (A (1 + g)) and F divided by 1 + g.
            let area = flow.area[i];
            let g = p.deposition * domain.area[i] / area;
            let h1 = h0[i] + dt * p.deposition * q_in[i] / (area * (1. + g));
            let hr = h[r];
            // A cell no higher than its receiver (a pit in the ground, where water was routed
            // over ice) fills only to its receiver's level.
            if h1 <= hr || h0[i] <= hr { h[i] = h1.min(hr.max(h0[i])); continue; }
            let f = p.k * erodibility[i] * dt * area.powf(p.m) / flow.receiver_dist[i].powf(p.n) / (1. + g);
            h[i] = solve_incision(h1, hr, f, p.n);
        }
        // Sediment fluxes, donors first. Lakes collect what reaches them; each is filled when
        // the walk reaches the cell it spills into (after all its own cells).
        q_in.iter_mut().for_each(|q| *q = 0.);
        pool.iter_mut().for_each(|q| *q = 0.);
        for &i in flow.stack.iter().rev() {
            if is_lake(i) {
                pool[exit[i]] += q_in[i] + source[i];
                q_out[i] = 0.;
                continue;
            }
            if let Some(cells) = lakes.get(&i) {
                q_in[i] += fill_lake(domain, cells, &h0, lake, h, pool[i] * dt) / dt;
            }
            let q = q_in[i] + source[i] + domain.area[i] * (h0[i] - h[i]) / dt;
            q_out[i] = q.max(0.);
            let r = flow.receiver[i];
            if r != i { q_in[r] += q_out[i]; }
        }
        let change = h.iter().zip(&before).map(|(a, b)| (a - b).abs()).fold(0., f64::max);
        if change < 1e-4 { break; }
    }
    q_out
}

/// Fill a lake (its `cells`, sorted from the deepest) with `volume` of sediment from the bottom
/// up, as a flat lake bed, no higher than the lake's level. Sets their heights in `h` and
/// returns the volume that doesn't fit.
fn fill_lake(domain: &Domain, cells: &[usize], h0: &[f64], lake: &[f64], h: &mut [f64], volume: f64) -> f64 {
    let top = |c: usize| h0[c] + lake[c];
    let spill = cells.iter().map(|&c| top(c)).fold(f64::NEG_INFINITY, f64::max);
    let mut level = h0[cells[0]];
    let mut left = volume;
    let mut area = 0.;
    for (k, &c) in cells.iter().enumerate() {
        area += domain.area[c];
        let next = cells.get(k + 1).map_or(spill, |&d| h0[d]).min(spill);
        let need = area * (next - level);
        if left <= need { level += left / area; break; }
        left -= need;
        level = next;
    }
    let mut used = 0.;
    for &c in cells {
        h[c] = h0[c].max(level.min(top(c)));
        used += (h[c] - h0[c]) * domain.area[c];
    }
    volume - used
}

/// Solve x - h1 + f (x - hr)^n = 0 for x in (hr, h1]: one cell's implicit incision.
fn solve_incision(h1: f64, hr: f64, f: f64, n: f64) -> f64 {
    if n == 1. { return (h1 + f * hr) / (1. + f); }
    let mut x = h1;
    for _ in 0..50 {
        let s = (x - hr).max(0.);
        let g = x - h1 + f * s.powf(n);
        let dg = 1. + n * f * s.powf(n - 1.);
        let next = (x - g / dg).max(hr + 1e-9 * (h1 - hr));
        let done = (next - x).abs() < 1e-9 * (1. + x.abs());
        x = next;
        if done { break; }
    }
    x
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

    fn lakes(f: &crate::terrain::flow::Flow, h: &[f64]) -> Vec<f64> {
        f.filled.iter().zip(h).map(|(a, b)| a - b).collect()
    }

    #[test]
    fn test_steady_state_slope_area() {
        // At steady state every cell exports the rock uplifted upstream, Q = U A, so deposition
        // adds G U to the uplift and S = ((1 + G) U / K)^(1/n) A^(-m/n).
        for (n, g) in [(1., 0.), (2., 0.), (1., 1.), (2., 2.)] {
            let d = Domain::patch(0., 0., 12, 100., 6.371e6);
            let p = FluvialParams { k: 1e-4, m: 0.5, n, diffusivity: 0., deposition: g, ..FluvialParams::default() };
            let u = 1e-3;
            let mut h = vec![0.; d.len()];
            let ones = vec![1.; d.len()];
            let zeros = vec![0.; d.len()];
            let dt = 1000.;
            for _ in 0..3000 {
                for i in 0..d.len() { if !d.boundary[i] { h[i] += u * dt; } }
                let f = route(&d, &h, &d.boundary, &ones);
                { let lake = lakes(&f, &h); erode_and_deposit(&d, &f, &mut h, &d.boundary, &ones, &lake, &zeros, &p, dt) };
            }
            let f = route(&d, &h, &d.boundary, &ones);
            for i in 0..d.len() {
                if d.boundary[i] { continue; }
                let r = f.receiver[i];
                let s = (h[i] - h[r]) / f.receiver_dist[i];
                let expected = ((1. + g) * u / p.k).powf(1. / n) * f.area[i].powf(-p.m / n);
                assert!((s - expected).abs() < 1e-3 * expected, "n {} G {} cell {}: {} vs {}", n, g, i, s, expected);
            }
        }
    }

    #[test]
    fn test_incision_never_cuts_below_receiver() {
        let d = Domain::patch(0., 0., 8, 10., 6.371e6);
        let mut h: Vec<f64> = (0..64).map(|k| ((k * 37) % 11) as f64 * 10.).collect();
        let ones = vec![1.; 64];
        let f = route(&d, &h, &d.boundary, &ones);
        let p = FluvialParams { k: 1., deposition: 0., ..FluvialParams::default() };
        erode_and_deposit(&d, &f, &mut h, &d.boundary, &ones, &vec![0.; 64], &vec![0.; 64], &p, 1e6);
        for i in 0..64 {
            assert!(h[i] >= h[f.receiver[i]] - 1e-9 || h[i] < f.filled[i]);
        }
    }

    #[test]
    fn test_sediment_is_conserved() {
        // With no uplift, what is eroded is deposited or leaves through the outlets.
        let d = Domain::patch(0., 0., 16, 100., 6.371e6);
        let mut h: Vec<f64> = (0..d.len()).map(|k| {
            let (i, j) = ((k % 16) as f64, (k / 16) as f64);
            if d.boundary[k] { 0. } else { 50. + 20. * (i * 0.9).sin() * (j * 0.7).cos() + 3. * i }
        }).collect();
        let ones = vec![1.; d.len()];
        let p = FluvialParams { k: 1e-3, deposition: 1., ..FluvialParams::default() };
        let f = route(&d, &h, &d.boundary, &ones);
        let before: f64 = h.iter().sum::<f64>() * 1e4;
        let q = { let lake = lakes(&f, &h); erode_and_deposit(&d, &f, &mut h, &d.boundary, &ones, &lake, &vec![0.; d.len()], &p, 100.) };
        let after: f64 = h.iter().sum::<f64>() * 1e4;
        let exported: f64 = (0..d.len()).filter(|&i| d.boundary[i]).map(|i| q[i] * 100.).sum();
        assert!(exported > 0.);
        assert!((before - after - exported).abs() < 1e-6 * before, "{} vs {}", before - after, exported);
    }

    #[test]
    fn test_sediment_fills_lakes() {
        // A slope draining into a closed basin, which overflows at one rim cell.
        let d = Domain::patch(0., 0., 9, 100., 6.371e6);
        let mut h: Vec<f64> = (0..81).map(|k| {
            let (i, j) = ((k % 9) as i64, (k / 9) as i64);
            if (i - 4).abs() <= 1 && (j - 4).abs() <= 1 { 0. } else { 20. + 10. * (i as f64) }
        }).collect();
        h[4 * 9] = 10.; // the spill point, on the patch's edge
        let ones = vec![1.; 81];
        let p = FluvialParams { k: 1e-3, deposition: 1., ..FluvialParams::default() };
        let source: Vec<f64> = (0..81).map(|k| if k == 8 * 9 + 8 { 1e4 } else { 0. }).collect();
        for _ in 0..50 {
            let f = route(&d, &h, &d.boundary, &ones);
            { let lake = lakes(&f, &h); erode_and_deposit(&d, &f, &mut h, &d.boundary, &ones, &lake, &source, &p, 100.) };
        }
        // The basin has been filled with sediment.
        assert!(h[4 * 9 + 4] > 5., "{}", h[4 * 9 + 4]);
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
