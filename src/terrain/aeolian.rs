//! # Wind: sand seas
//!
//! In deserts the wind moves sediment as well as water does. Where too little rain falls for
//! plants to hold the ground (less than about 250 mm a year in hot deserts; dunes stabilise
//! under vegetation above that: Tsoar, "Sand dunes mobility and stability in relation to
//! climate", Physica A 357, 2005), sand is blown out of dry river beds, alluvial fans and
//! playas, carried downwind and laid down where the wind slackens: in the lee of ridges, in
//! basins, against mountain fronts, and where plants take hold again. So sand seas (ergs)
//! gather downwind of their sources and in topographic lows (Wilson, "Aeolian bedforms - their
//! development and origins", Sedimentology 19, 1972), while the ground upwind is stripped to
//! gravel and rock (desert pavement, hamada).
//!
//! Each step the sand is swept along the prevailing wind over a patch, cell by cell in order
//! downwind (as `orographic` sweeps its vapour). The wind can carry `flux` m³ of sand per metre
//! across it each year over bare, level ground (active dune fields move tens of m³/m/yr:
//! Fryberger and Dean, "Dune forms and wind regime", USGS Professional Paper 1052, 1979),
//! less where plants cover the ground (`vegetation`, mm/yr of rain from bare to covered), and
//! more where the ground rises into the wind, less where it falls away (`exposure`, per unit
//! of slope along the wind), so that its lee is sheltered. Where the wind slackens (its
//! capacity falls from one cell to the next) it lays down a share (`deposition`) of what it
//! can no longer carry, in proportion to the fall, whether or not it was carrying all it could
//! (as saltating sand lands more where the wind weakens); and of any excess over what it can
//! carry. Elsewhere, carrying less than it can, it picks up loose sand, as much as lies there. What reaches the sea or the patch's edge is lost. Loose sand
//! is what the wind laid itself, and the fine share (`sand_fraction`) of sediment laid since
//! its last pass: most alluvium is gravel, which stays behind as a lag once its sand is blown
//! out and armours the ground beneath (desert pavement), so old alluvium isn't stripped away.
//!
//! The dunes themselves, hundreds of metres apart, are smaller than a simulation's cells: they
//! are drawn on the sand it leaves (`Terrain::sand`; `scenery`). Only patches have a wind.

use crate::terrain::domain::{Domain, Layout};

#[derive(Clone, Debug, PartialEq)]
pub struct AeolianParams {
    /// Sand the wind carries over bare level ground, m³ per metre across the wind per year.
    pub flux: f64,
    /// Rain (mm/yr) at which plants begin to hold the ground, and at which they hold it all.
    pub vegetation: (f64, f64),
    /// How much more the wind carries for each unit of slope rising into it.
    pub exposure: f64,
    /// Share of the excess load laid down in each cell.
    pub deposition: f64,
    /// Share of freshly laid sediment (alluvium, till, debris) fine enough for the wind to
    /// carry; the rest stays as a gravel lag, armouring what lies beneath.
    pub sand_fraction: f64,
}

impl Default for AeolianParams {
    fn default() -> AeolianParams {
        AeolianParams { flux: 20., vegetation: (100., 350.), exposure: 10., deposition: 0.5, sand_fraction: 0.3 }
    }
}

/// How bare the ground is of plants where `rain` mm/yr falls, 0 to 1.
pub fn bare(rain: f64, p: &AeolianParams) -> f64 {
    let (a, b) = p.vegetation;
    let t = ((rain - a) / (b - a).max(1e-9)).clamp(0., 1.);
    1. - t * t * (3. - 2. * t)
}

/// Blow sand over `domain` for `dt` years, the wind towards (`east`, `north`): heights `h`
/// (the surface, metres), loose sediment `loose` (metres) that may be picked up, rain (mm/yr),
/// and cells marked `sink` (the sea, the patch's edge, ice) taking what reaches them. Returns
/// the change in each cell's ground, metres (negative where sand was taken).
pub fn blow(domain: &Domain, h: &[f64], loose: &[f64], rain: &[f64], sink: &[bool], wind: (f64, f64), p: &AeolianParams, dt: f64) -> Vec<f64> {
    let n = domain.len();
    let mut change = vec![0.; n];
    let Layout::Patch { up, east, north, .. } = &domain.layout else { return change };
    let norm = wind.0.hypot(wind.1);
    if norm <= 0. || p.flux <= 0. { return change; }
    let (wx, wz) = (wind.0 / norm, wind.1 / norm);
    let at: Vec<(f64, f64)> = domain.dirs.iter().map(|d| Domain::tangent(*up, *east, *north, domain.radius, d)).collect();
    let along: Vec<f64> = at.iter().map(|&(x, z)| x * wx + z * wz).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| along[a].total_cmp(&along[b]).then(a.cmp(&b)));
    // What the wind can carry at each cell, m³/yr: over bare ground, more on slopes rising
    // into it, less in their lee.
    let capacity: Vec<f64> = (0..n).map(|i| {
        let (mut w, mut hu, mut ds) = (0., 0., 0.);
        for l in domain.links(i) {
            let toward = ((at[i].0 - at[l.to].0) * wx + (at[i].1 - at[l.to].1) * wz) / l.dist;
            if toward <= 1e-6 { continue; }
            w += toward;
            hu += toward * h[l.to];
            ds += toward * l.dist * toward;
        }
        let rise = if w > 0. { (h[i] - hu / w) / (ds / w).max(1.) } else { 0. };
        let exposure = (1. + p.exposure * rise).clamp(0.1, 2.);
        p.flux * domain.area[i].sqrt() * bare(rain[i], p) * exposure
    }).collect();
    // Each cell's load, m³/yr, passed on to the cells downwind in proportion to how directly
    // the wind blows to them, with the capacity it came with.
    let mut load = vec![0.; n];
    let mut came = vec![0.; n];
    for &i in &order {
        let links = domain.links(i);
        let mut q = load[i];
        if !sink[i] {
            let c = capacity[i];
            // Where the wind slackens it drops what it can no longer carry, a share of its load
            // as its capacity falls (whether or not it was carrying all it could), and of the
            // excess over what it can carry here.
            let slackened = if load[i] > 0. && came[i] > c { q * (1. - c / came[i]) } else { 0. };
            let excess = (q - slackened - c).max(0.);
            let lay = (slackened + excess) * p.deposition.clamp(0., 1.);
            if lay > 0. {
                q -= lay;
                change[i] += lay * dt / domain.area[i];
            } else if q < c {
                // Pick up loose sand, as much as there is.
                let take = (c - q).min(loose[i] * domain.area[i] / dt);
                q += take;
                change[i] -= take * dt / domain.area[i];
            }
        }
        if sink[i] || q <= 0. { continue; }
        // On downwind.
        let mut total = 0.;
        for l in links {
            let toward = ((at[l.to].0 - at[i].0) * wx + (at[l.to].1 - at[i].1) * wz) / l.dist;
            if toward > 1e-6 { total += toward; }
        }
        // At the downwind edge it leaves the patch.
        if total <= 0. { continue; }
        for l in links {
            let toward = ((at[l.to].0 - at[i].0) * wx + (at[l.to].1 - at[i].1) * wz) / l.dist;
            if toward > 1e-6 {
                let share = q * toward / total;
                // The capacity it arrives with, weighted by the load.
                let before = load[l.to];
                load[l.to] += share;
                came[l.to] = if load[l.to] > 0. { (came[l.to] * before + capacity[i] * share) / load[l.to] } else { capacity[i] };
            }
        }
    }
    change
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patch() -> Domain { Domain::patch(25., 0., 32, 1000., 6.371e6) }

    #[test]
    fn test_sand_blows_downwind_in_deserts_and_not_where_it_is_green() {
        // Loose sediment over the western third of a flat patch, wind from the west, the
        // desert giving way to steppe halfway across: the sand is stripped in the west and laid
        // down where the plants take hold. With 600 mm of rain everywhere it stays.
        let d = patch();
        let n = d.len();
        let col = |k: usize| k % 32;
        let loose: Vec<f64> = (0..n).map(|k| if col(k) < 10 { 5. } else { 0. }).collect();
        let h = vec![100.; n];
        let sink = d.boundary.clone();
        let p = AeolianParams::default();
        let rain: Vec<f64> = (0..n).map(|k| if col(k) < 16 { 50. } else { 300. }).collect();
        let c = blow(&d, &h, &loose, &rain, &sink, (1., 0.), &p, 500.);
        let mean = |cols: std::ops::Range<usize>| (0..n).filter(|&k| cols.contains(&col(k)) && !sink[k]).map(|k| c[k]).sum::<f64>() / (cols.len() * 30) as f64;
        assert!(mean(1..10) < -0.5 && mean(16..20) > 0.5, "{} {}", mean(1..10), mean(16..20));
        // Nothing taken that wasn't there, and nothing made.
        assert!((0..n).all(|k| c[k] >= -loose[k] - 1e-9));
        let green = blow(&d, &h, &loose, &vec![600.; n], &sink, (1., 0.), &p, 500.);
        assert!(green.iter().all(|&x| x == 0.));
    }

    #[test]
    fn test_sand_gathers_in_the_lee_of_a_ridge() {
        // A ridge across the wind: the wind slackens behind it and drops sand there.
        let d = patch();
        let n = d.len();
        let col = |k: usize| (k % 32) as f64;
        let h: Vec<f64> = (0..n).map(|k| 100. + 300. * (-((col(k) - 16.) / 2.).powi(2)).exp()).collect();
        let loose: Vec<f64> = (0..n).map(|k| if col(k) < 8. { 20. } else { 0. }).collect();
        let sink = d.boundary.clone();
        let c = blow(&d, &h, &loose, &vec![50.; n], &sink, (1., 0.), &AeolianParams::default(), 500.);
        let at = |x: usize| (1..31).map(|r| c[r * 32 + x]).sum::<f64>() / 30.;
        assert!(at(18) > at(14) && at(18) > 0., "lee {} windward {}", at(18), at(14));
    }
}
