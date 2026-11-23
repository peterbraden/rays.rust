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
//! ## Lateral erosion
//!
//! Rivers don't only cut down: where they bend, the flow presses against the outer bank and
//! cuts it back, so rivers shift across their valleys, widen them and plane bedrock into
//! straths, and work towards wherever their banks give way. Langston and Tucker ("Developing
//! and exploring a theory for the lateral erosion of bedrock channels for use in landscape
//! evolution models", Earth Surface Dynamics 6, 2018, doi:10.5194/esurf-6-1-2018) add this to
//! grid models: where flow turns through a cell, the neighbour on the outside of the bend is
//! eroded sideways at a rate tied to the river's vertical erosion and the bend's sharpness. Here
//! (`lateral_erosion`) the bank retreats at
//!
//! ```text
//! E_l = λ K k_bank A^m S^n (1 - cos θ)
//! ```
//!
//! θ the angle the flow turns through (from the largest donor's direction to the receiver's),
//! k_bank the bank's own erodibility, λ = 0.5 (a model choice; they explore lateral to vertical
//! coefficients of this order). The bank is lowered by the volume the retreat removes (the
//! retreat times the bank's height times the cell's width), never below the channel, and that
//! volume joins the river's sediment, so it is conserved. Straight reaches don't erode sideways.
//!
//! ## Avulsion
//!
//! On an alluvial plain a river doesn't take the steepest way at every moment: it keeps its
//! channel, which its own deposits raise, with its levees, above the plain around it, until in
//! a flood it breaks out down the side of its ridge to a steeper course, an avulsion (Slingerland
//! and Smith, "River avulsions and their deposits", Annual Review of Earth and Planetary
//! Sciences 32, 2004). Repeated, it switches a delta's lobes and builds the fingers of
//! river-dominated deltas, and spreads fans. Here (`Terrain`'s routing, with `avulsion`) a
//! channel on sediment keeps last step's course while that is still downhill, unless a way
//! down the side is `avulsion` (5) times steeper (the gradient advantage: avulsions happen at
//! advantages of a few to tens) and the channel stands a channel's depth above it (the
//! super-elevation that sets avulsions off, about a channel's depth: Mohrig, Heller, Paola and
//! Lyons, GSA Bulletin 112, 2000).
//!
//! Not modelled: thresholds for incision; bifurcating distributaries (water divides only by
//! avulsion, one course at a time); levees and overbank deposition as such (the channel's own
//! deposition raises its cell).
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
//! κ w (h_j - h_i) / d (a finite-volume Laplacian). κ may vary from cell to cell
//! (`diffuse_varying`: the landscape makes it the rivers' erodibility of the rock times the
//! diffusivity, as weak rock weathers faster into soil that creeps); between two cells it is
//! the harmonic mean of theirs, as for conductances in series, which keeps the scheme
//! conservative. Steps are explicit, sub-divided to keep
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
    /// Lateral erosion at bends relative to vertical incision, λ (0: rivers only cut down).
    pub lateral: f64,
    /// How much steeper a way down the side of an alluvial channel's ridge must be than its own
    /// course for it to avulse there (the gradient advantage); 0 for no persistence, rivers
    /// taking the steepest way every step.
    pub avulsion: f64,
}

impl Default for FluvialParams {
    fn default() -> FluvialParams {
        FluvialParams { k: 1e-5, m: 0.5, n: 1., diffusivity: 0.01, deposition: 1., lateral: 0.5, avulsion: 5. }
    }
}

/// Cut back the outer banks of bends for `dt` years (see "Lateral erosion"): each river
/// cell's bank, the neighbour on the outside of the bend if it stands higher, is lowered by
/// the volume its retreat removes, never below the channel. `erodibility` scales K per cell (0
/// where nothing erodes, eg. under ice); cells of `lake` deeper than `LAKE` and `fixed` ones
/// neither cut nor are cut. Returns the sediment each river cell gains, m³/yr, to add to its
/// sediment flux.
#[allow(clippy::too_many_arguments)]
pub fn lateral_erosion(domain: &Domain, flow: &Flow, h: &mut [f64], fixed: &[bool], erodibility: &[f64], lake: &[f64], p: &FluvialParams, dt: f64) -> Vec<f64> {
    let n = h.len();
    let mut gained = vec![0.; n];
    if p.lateral <= 0. || dt <= 0. { return gained; }
    // Each cell's largest donor: where its flow comes from.
    let mut donor = vec![usize::MAX; n];
    for i in 0..n {
        let r = flow.receiver[i];
        if r != i && (donor[r] == usize::MAX || flow.area[i] > flow.area[donor[r]]) { donor[r] = i; }
    }
    let tangent = |from: usize, to: usize| {
        let d = domain.dirs[from];
        let v = domain.dirs[to] - d;
        let t = v - d * d.dot(&v);
        let size = t.norm();
        if size > 0. { t / size } else { t }
    };
    // What each river cell would take from its bank: (bank, metres the bank is lowered).
    let cuts: Vec<Option<(usize, f64)>> = (0..n).into_par_iter().map(|i| {
        let (r, u) = (flow.receiver[i], donor[i]);
        if r == i || u == usize::MAX || fixed[i] || erodibility[i] <= 0. || lake[i] > LAKE { return None; }
        let (inflow, outflow) = (tangent(u, i), tangent(i, r));
        let turn = 1. - inflow.dot(&outflow).clamp(-1., 1.);
        if turn < 1e-3 { return None; }
        let outside = inflow - outflow;
        let (link, _) = domain.links(i).iter()
            .filter(|l| l.to != r && l.to != u)
            .map(|l| (l, tangent(i, l.to).dot(&outside)))
            .filter(|&(_, a)| a > 0.)
            .max_by(|a, b| a.1.total_cmp(&b.1))?;
        let j = link.to;
        let height = h[j] - h[i];
        if height <= 0. || fixed[j] || erodibility[j] <= 0. || lake[j] > LAKE { return None; }
        let slope = (h[i] - h[r]).max(0.) / flow.receiver_dist[i];
        let retreat = p.lateral * p.k * erodibility[j] * flow.area[i].powf(p.m) * slope.powf(p.n) * turn * dt;
        let width = if link.width > 0. { link.width } else { link.dist / std::f64::consts::SQRT_2 };
        Some((j, (retreat * height * width / domain.area[j]).min(height)))
    }).collect();
    // A bank cut from several sides is lowered by the most any asks, shared among them.
    let mut lowered = vec![0f64; n];
    let mut asked = vec![0f64; n];
    for &(j, d) in cuts.iter().flatten() { lowered[j] = lowered[j].max(d); asked[j] += d; }
    for (i, c) in cuts.iter().enumerate() {
        let Some((j, d)) = *c else { continue };
        gained[i] = lowered[j] * domain.area[j] * d / asked[j] / dt;
    }
    for (x, l) in h.iter_mut().zip(&lowered) { *x -= l; }
    gained
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
    diffuse_varying(domain, h, fixed, &vec![kappa; domain.len()], dt);
}

/// Diffuse hillslopes for `dt` years, each cell with its own diffusivity `kappa` (m²/yr): the
/// flux between two cells takes the harmonic mean of theirs, as conductances in series do, so
/// a hard cell beside a soft one passes little, and what leaves one cell enters the other.
/// Cells marked `fixed` don't change.
pub fn diffuse_varying(domain: &Domain, h: &mut [f64], fixed: &[bool], kappa: &[f64], dt: f64) {
    if dt <= 0. || kappa.iter().all(|&k| k <= 0.) { return; }
    let between = |i: usize, j: usize| {
        let (a, b) = (kappa[i].max(0.), kappa[j].max(0.));
        if a + b > 0. { 2. * a * b / (a + b) } else { 0. }
    };
    // Explicit diffusion is stable while each cell gives away at most half its excess per step.
    let limit = (0..domain.len()).map(|i| {
        let conductance: f64 = domain.links(i).iter().map(|l| between(i, l.to) * l.width / l.dist).sum();
        if conductance > 0. { 0.5 * domain.area[i] / conductance } else { f64::INFINITY }
    }).fold(f64::INFINITY, f64::min);
    let steps = (dt / limit).ceil().max(1.) as usize;
    let step = dt / steps as f64;
    let mut next = h.to_vec();
    for _ in 0..steps {
        next.par_iter_mut().enumerate().for_each(|(i, x)| {
            if fixed[i] { *x = h[i]; return; }
            let flux: f64 = domain.links(i).iter().map(|l| between(i, l.to) * l.width * (h[l.to] - h[i]) / l.dist).sum();
            *x = h[i] + step * flux / domain.area[i];
        });
        h.copy_from_slice(&next);
    }
}

/// Let slopes steeper than they can stand fail: each cell whose slope down to its neighbours
/// exceeds its `critical` slope (rise over run; infinite where it can't fail, eg. buttressed
/// by ice) sends rock down to those neighbours, shared by how far each slope exceeds the limit
/// (sent all to the steepest, debris ran down in lines). Cells are taken from the highest
/// down, each moving its share at once, so a failure cascades down the whole slope in one
/// pass; passes repeat, over just the cells that moved or whose neighbours did, until no slope
/// exceeds its limit by more than `tolerance` (or `max_passes` have run). What fails lands on
/// the neighbours, or leaves the model where they are fixed (the sea, a patch's edge); fixed
/// cells don't change. Returns the volume moved, m³, and which cells debris landed on.
///
/// Debris is loose and mixes as it falls, so the cells it landed on are then smoothed among
/// themselves, conserving volume (`diffuse_varying`, with no exchange with the rest): limits
/// on the slopes to each of the eight neighbours otherwise leave the debris in faceted ridges
/// along the grid's diagonals.
///
/// This is threshold hillslope behaviour (Schmidt and Montgomery, "Limits to relief", Science
/// 270, 1995; Burbank et al., Nature 379, 1996): where rivers and glaciers cut down faster
/// than slopes can follow by creep, the slopes steepen until they fail in landslides, so their
/// steepness, and the ranges' relief, is set by the strength of the rock.
pub fn landslide(domain: &Domain, h: &mut [f64], fixed: &[bool], critical: &[f64], tolerance: f64, max_passes: usize) -> (f64, Vec<bool>) {
    let n = h.len();
    let mut failed = vec![false; n];
    let mut moved = fail(domain, h, fixed, critical, tolerance, max_passes, &mut failed);
    if moved > 0. {
        // Mix the debris: each cell it landed on exchanging up to a few tenths of its
        // difference with such neighbours, a few times; then let whatever that steepened much fail.
        let kappa: Vec<f64> = (0..n).map(|i| if failed[i] && !fixed[i] { 0.2 * domain.area[i] } else { 0. }).collect();
        diffuse_varying(domain, h, fixed, &kappa, 3.);
        // Slopes the mixing leaves a little over the limit stay: failed again to the limit, the
        // facets would come back.
        let loose: Vec<f64> = critical.iter().map(|c| c * LOOSE).collect();
        moved += fail(domain, h, fixed, &loose, tolerance, max_passes, &mut failed);
    }
    (moved, failed)
}

/// How far over its limit `landslide` leaves a slope after mixing the debris.
pub const LOOSE: f64 = 1.15;

/// `landslide`'s passes of failure, marking the cells debris landed on in `failed`; returns
/// the volume moved.
fn fail(domain: &Domain, h: &mut [f64], fixed: &[bool], critical: &[f64], tolerance: f64, max_passes: usize, failed: &mut [bool]) -> f64 {
    let n = h.len();
    let mut moved = 0.;
    let can_fail = |i: usize| !fixed[i] && critical[i].is_finite();
    let mut active: Vec<usize> = (0..n).filter(|&i| can_fail(i)).collect();
    let mut touched = vec![false; n];
    let mut excesses: Vec<(usize, f64)> = Vec::with_capacity(8);
    for _ in 0..max_passes {
        if active.is_empty() { break; }
        active.sort_by(|&a, &b| h[b].total_cmp(&h[a]).then(a.cmp(&b)));
        let mut changed: Vec<usize> = vec![];
        for &i in &active {
            excesses.clear();
            excesses.extend(domain.links(i).iter()
                .map(|l| (l.to, h[i] - h[l.to] - critical[i] * l.dist))
                .filter(|&(_, excess)| excess > tolerance));
            if excesses.is_empty() { continue; }
            let total: f64 = excesses.iter().map(|e| e.1).sum();
            let steepest = excesses.iter().map(|e| e.1).fold(0., f64::max);
            for &(j, excess) in &excesses {
                // Shares of the volume that would close the steepest excess (into a fixed
                // cell, which doesn't rise, as if into an equal one), by excess.
                let area_j = if fixed[j] { domain.area[i] } else { domain.area[j] };
                let volume = steepest / (1. / domain.area[i] + 1. / area_j) * excess / total;
                h[i] -= volume / domain.area[i];
                if !fixed[j] { h[j] += volume / domain.area[j]; changed.push(j); failed[j] = true; }
                moved += volume;
            }
            changed.push(i);
        }
        // Next, the cells that moved and their neighbours.
        active.clear();
        for &c in &changed {
            for k in std::iter::once(c).chain(domain.links(c).iter().map(|l| l.to)) {
                if !touched[k] && can_fail(k) { touched[k] = true; active.push(k); }
            }
        }
        for &k in &active { touched[k] = false; }
    }
    moved
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

    #[test]
    fn test_slopes_fail_down_to_what_their_rock_holds() {
        // A cone too steep for any rock, on weak rock on one side and strong on the other.
        let d = Domain::patch(0., 0., 24, 100., 6.371e6);
        let n = d.len();
        let mut h: Vec<f64> = (0..n).map(|k| {
            let (i, j) = ((k % 24) as f64 - 11.5, (k / 24) as f64 - 11.5);
            if d.boundary[k] { 0. } else { (2000. - 250. * i.hypot(j)).max(0.) }
        }).collect();
        let critical: Vec<f64> = (0..n).map(|k| if k % 24 < 12 { 0.6 } else { 1.5 }).collect();
        let volume = |h: &[f64]| (0..n).filter(|&k| !d.boundary[k]).map(|k| h[k] * d.area[k]).sum::<f64>();
        let before = volume(&h);
        let (moved, _) = landslide(&d, &mut h, &d.boundary, &critical, 0.01, 2000);
        assert!(moved > 0.);
        // No slope is left steeper than its rock holds (bar what mixing the debris leaves).
        for k in 0..n {
            if d.boundary[k] { continue; }
            for l in d.links(k) { assert!(h[k] - h[l.to] <= LOOSE * critical[k] * l.dist + 0.02, "cell {}", k); }
        }
        // Volume is kept, but for what fell off the edge; the weak side has been brought lower.
        assert!(volume(&h) <= before + 1e-6 * before);
        let side = |lo: usize, hi: usize| (0..n).filter(|&k| (lo..hi).contains(&(k % 24)) && !d.boundary[k]).map(|k| h[k]).fold(0., f64::max);
        assert!(side(1, 10) < side(14, 23), "{} vs {}", side(1, 10), side(14, 23));
    }

    #[test]
    fn test_rivers_cut_the_outside_of_their_bends() {
        // A river running east along row 8 of a valley, turning north at column 8: the bank on
        // the outside of the bend (east of the turn) is cut back, the inside isn't, and the
        // rock taken off joins the river.
        let d = Domain::patch(0., 0., 16, 100., 6.371e6);
        let n = d.len();
        let at = |i: usize, j: usize| j * 16 + i;
        let mut h = vec![100.; n];
        for i in 0..=8 { h[at(i, 8)] = 20. - i as f64; }
        for j in 9..16 { h[at(8, j)] = 12. - (j - 8) as f64; }
        let mut outlet = vec![false; n];
        outlet[at(8, 15)] = true;
        let f = route(&d, &h, &outlet, &vec![1.; n]);
        assert_eq!(f.receiver[at(8, 8)], at(8, 9));
        let before = h.clone();
        let p = FluvialParams { k: 1e-3, ..FluvialParams::default() };
        let gained = lateral_erosion(&d, &f, &mut h, &outlet, &vec![1.; n], &vec![0.; n], &p, 100.);
        // The outer bank is east of the turn, behind the flow's old heading.
        assert!(h[at(9, 8)] < before[at(9, 8)], "outer bank {} -> {}", before[at(9, 8)], h[at(9, 8)]);
        assert!(h[at(9, 8)] >= before[at(8, 8)]);
        assert_eq!(h[at(7, 9)], before[at(7, 9)]);
        // Conserved: what the banks lost, the river gained.
        let lost: f64 = (0..n).map(|k| (before[k] - h[k]) * d.area[k]).sum();
        let got: f64 = gained.iter().sum::<f64>() * 100.;
        assert!(lost > 0. && (lost - got).abs() < 1e-6 * lost, "{} vs {}", lost, got);
    }

    #[test]
    fn test_soft_ground_creeps_faster() {
        // Two equal bumps, one on soft ground and one on hard: mass is conserved, and the soft
        // one flattens more.
        let d = Domain::sphere(6, 1000.);
        let (soft, hard) = (40, 150);
        let mut h: Vec<f64> = (0..d.len()).map(|i| if i == soft || i == hard { 100. } else { 0. }).collect();
        let kappa: Vec<f64> = (0..d.len()).map(|i| {
            let near = |c: usize| i == c || d.links(c).iter().any(|l| l.to == i);
            if near(soft) { 3. } else if near(hard) { 0.3 } else { 1. }
        }).collect();
        let mass = |h: &[f64]| h.iter().zip(&d.area).map(|(a, b)| a * b).sum::<f64>();
        let before = mass(&h);
        diffuse_varying(&d, &mut h, &vec![false; d.len()], &kappa, 5e3);
        assert!((mass(&h) - before).abs() < 1e-9 * before);
        assert!(h[soft] < h[hard] - 5., "{} vs {}", h[soft], h[hard]);
    }
}
