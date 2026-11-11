//! # Volcanoes
//!
//! Volcanoes build land faster than any other process: a stratovolcano can grow kilometres in
//! tens of thousands of years, and lava flows fill valleys, divert rivers and dam lakes. Here a
//! volcano is a *vent* that erupts for a while, building a cone around itself and sending lava
//! down the surrounding slopes. What it erupts is hard rock (the `lava` layer of the terrain),
//! so old volcanoes stand out as rivers and glaciers wear down the land around them.
//!
//! ## Where and when
//!
//! Vents occur where `Planet::volcanism` is high (volcanic arcs beside mountain belts, and
//! hotspots), at most one per cell of a fixed lattice over the planet (a cubed sphere with cells
//! `spacing` apart, 30 km). De Bremond d'Arcs, Jaupart and Sparks ("Distribution of volcanoes
//! in active margins", JGR 100(B10), 1995, doi:10.1029/95JB02153) found no characteristic
//! spacing along 16 margins: spacings are close to those of randomly placed points. Each lattice
//! cell holding a vent with some probability, at a random position within it, gives that. Each
//! lattice cell draws, from the seed alone, whether it holds a vent, the vent's position within
//! it, when it starts erupting (uniformly over the `horizon`), for how long, and how fast. So the
//! same volcanoes appear whatever the grid: a patch and the whole planet share them.
//!
//! ## Cones
//!
//! White, Crisp and Spera ("Long-term volumetric eruption rates and magma budgets", G-cubed 7,
//! 2006, doi:10.1029/2005GC001002) compiled 170 long-term eruption rates: a mean of about
//! 2.3 km³ per thousand years for andesitic volcanoes, ten times that for basaltic ones. The
//! default `rate`, 1e6 m³/yr (1 km³/kyr), is at the low end, for arc volcanoes.
//!
//! A fraction of the erupted volume builds the edifice: spread over the cells within
//! `cone_radius` of the vent with weight 1 - r/R, so steady eruption builds a cone. (Real cones
//! steepen towards the summit, and have craters; neither is modelled.)
//!
//! ## Lava flows
//!
//! The rest flows as lava. Lava follows the steepest way down, but small irregularities of
//! the ground decide where it goes on gentle slopes. Favalli, Pareschi, Neri and Isola's
//! DOWNFLOW ("Forecasting lava flow paths by a stochastic approach", Geophysical Research
//! Letters 32(3), 2005, doi:10.1029/2004GL021718) finds the likely paths by repeatedly
//! perturbing the elevation model with random noise of a few metres and following the steepest
//! descent. It predicts direction, not how far lava runs; here the run-out follows from the
//! volume and `flow_thickness`. `roughness` is larger than DOWNFLOW's metres because our cells
//! are hundreds of metres across, smoothing away the small relief that steers real flows. Here each landscape step sends one flow, along
//! one such perturbed path, laying lava `flow_thickness` thick until the step's volume is used.
//! A flow stops at the sea, the edge of a patch, or a pit, where the rest of its lava ponds.
//! Over many steps the flows spread down different valleys, building a lava apron.

use crate::geometry::Pcg32;
use crate::na::Vector3;
use crate::terrain::domain::{sphere_dir, Domain, Layout};
use crate::terrain::planet::Planet;
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug)]
pub struct VolcanoParams {
    /// Spacing of the lattice of possible vents, metres.
    pub spacing: f64,
    /// Chance that a lattice cell where volcanism is 1 holds a vent within the horizon.
    pub density: f64,
    /// Vents start erupting at random times between 0 and this, years.
    pub horizon: f64,
    /// Typical eruptive lifetime of a vent, years.
    pub duration: f64,
    /// Typical long-term eruption rate, cubic metres per year.
    pub rate: f64,
    /// Radius of a cone, metres.
    pub cone_radius: f64,
    /// Share of the erupted volume that flows away as lava, rather than building the cone.
    pub lava_fraction: f64,
    /// Thickness of a lava flow, metres.
    pub flow_thickness: f64,
    /// Amplitude of the random perturbation of the ground that steers lava, metres.
    pub roughness: f64,
}

impl Default for VolcanoParams {
    fn default() -> VolcanoParams {
        VolcanoParams {
            spacing: 30e3,
            density: 0.6,
            horizon: 200e3,
            duration: 40e3,
            rate: 1e6,
            cone_radius: 6e3,
            lava_fraction: 0.4,
            flow_thickness: 10.,
            roughness: 10.,
        }
    }
}

pub struct Vent {
    pub dir: Vector3<f64>,
    /// When the vent erupts, years.
    pub start: f64,
    pub end: f64,
    /// Cubic metres per year.
    pub rate: f64,
    seed: u64,
    /// The cell the vent is in, if it is in the domain.
    cell: Option<usize>,
    /// The cells its cone covers, with their weights.
    cone: Vec<(usize, f64)>,
}

/// The vents that can affect `domain`.
pub fn vents(planet: &Planet, domain: &Domain, p: &VolcanoParams) -> Vec<Vent> {
    let radius = domain.radius;
    let n = ((std::f64::consts::FRAC_PI_2 * radius / p.spacing).round() as usize).max(1);
    let lattice: BTreeSet<usize> = match &domain.layout {
        Layout::Sphere { .. } => (0..6 * n * n).collect(),
        Layout::Patch { .. } => domain.dirs.iter().chain(&domain.corners).map(|d| Domain::sphere_cell_at(n, d)).collect(),
    };
    let mut vents = vec![];
    for k in lattice {
        let mut rng = Pcg32::new(planet.params.seed ^ 0x766f_6c63, k as u64);
        let (f, j, i) = (k / (n * n), (k / n) % n, k % n);
        let step = 2. / n as f64;
        let (u, v, chance, start, length, rate) = (rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64());
        let dir = sphere_dir(f, -1. + (i as f64 + u) * step, -1. + (j as f64 + v) * step);
        if chance >= p.density * planet.volcanism(&dir) { continue; }
        let start = start * p.horizon;
        let mut vent = Vent {
            dir,
            start,
            end: start + p.duration * (0.5 + length),
            rate: p.rate * (0.25 + 1.5 * rate),
            seed: ((k as u64) << 32) ^ rng.next_u32() as u64,
            cell: None,
            cone: vec![],
        };
        place(&mut vent, domain, p);
        if !vent.cone.is_empty() { vents.push(vent); }
    }
    vents
}

/// Find the vent's cell and the cells its cone covers.
fn place(vent: &mut Vent, domain: &Domain, p: &VolcanoParams) {
    let distance = |c: usize| domain.radius * vent.dir.angle(&domain.dirs[c]);
    let start = domain.cell_at(&vent.dir);
    // On a patch, cell_at gives the nearest edge cell for vents beyond the edge.
    let cell_size = domain.area[start].sqrt();
    if distance(start) <= cell_size { vent.cell = Some(start); }
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(c) = queue.pop_front() {
        let d = distance(c);
        let w = 1. - d / p.cone_radius;
        if w > 0. { vent.cone.push((c, w)); }
        else if Some(c) == vent.cell { vent.cone.push((c, 1.)); } // cells larger than the cone
        if w <= 0. { continue; }
        for l in domain.links(c) {
            if seen.insert(l.to) { queue.push_back(l.to); }
        }
    }
}

/// Erupt every vent active between `t` and `t + dt`, adding to `ground` and recording what was
/// added in `lava`. Cells marked `fixed` (sea, a patch's edge) stop lava flows. Returns the
/// volume erupted into the domain, cubic metres.
#[allow(clippy::too_many_arguments)]
pub fn erupt(domain: &Domain, vents: &[Vent], p: &VolcanoParams, t: f64, dt: f64, ground: &mut [f64], lava: &mut [f64], fixed: &[bool]) -> f64 {
    let mut total = 0.;
    for vent in vents {
        let active = (t + dt).min(vent.end) - t.max(vent.start);
        if active <= 0. { continue; }
        let volume = vent.rate * active;

        // The cone's weights integrate to π R² / 3 over the whole cone; only the part of it in
        // the domain is built. (Cells larger than the cone take all of it.)
        let cone = volume * (1. - p.lava_fraction);
        let in_domain: f64 = vent.cone.iter().map(|&(c, w)| w * domain.area[c]).sum();
        let weight = in_domain.max(std::f64::consts::PI * p.cone_radius * p.cone_radius / 3.);
        for &(c, w) in &vent.cone {
            let h = cone * w / weight;
            ground[c] += h;
            lava[c] += h;
            total += h * domain.area[c];
        }

        // Lava flows only from vents in the domain.
        let Some(mut c) = vent.cell else { continue };
        let mut remaining = volume * p.lava_fraction;
        total += remaining;
        let mut rng = Pcg32::new(vent.seed, t.to_bits());
        for _ in 0..domain.len() {
            if remaining <= 0. { break; }
            // The steepest way down over ground perturbed by up to `roughness`.
            let next = domain.links(c).iter()
                .map(|l| (l.to, (ground[c] - ground[l.to] - p.roughness * rng.next_f64()) / l.dist))
                .filter(|&(_, s)| s > 0.)
                .max_by(|a, b| a.1.total_cmp(&b.1));
            let lay = if next.is_none() || fixed[c] { remaining } else { remaining.min(p.flow_thickness * domain.area[c]) };
            ground[c] += lay / domain.area[c];
            lava[c] += lay / domain.area[c];
            remaining -= lay;
            match next {
                Some((to, _)) if !fixed[c] => c = to,
                _ => break,
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::planet::PlanetParams;

    fn volcanic_patch() -> (Planet, Domain) {
        let planet = Planet::new(PlanetParams { seed: 5, ..PlanetParams::default() });
        // Find a strongly volcanic spot.
        let mut best = (0., 0., 0.);
        for lat in (-60..60).step_by(3) {
            for lon in (-180..180).step_by(3) {
                let (up, _, _) = Domain::frame(lat as f64, lon as f64);
                let v = planet.volcanism(&up);
                if v > best.0 { best = (v, lat as f64, lon as f64); }
            }
        }
        let d = Domain::patch(best.1, best.2, 64, 1000., planet.params.radius);
        (planet, d)
    }

    #[test]
    fn test_vents_are_the_same_at_any_resolution() {
        let (planet, d) = volcanic_patch();
        let p = VolcanoParams::default();
        let a = vents(&planet, &d, &p);
        assert!(!a.is_empty());
        let (up, _, _) = (d.dirs[0], 0, 0);
        let lat = up.y.asin().to_degrees();
        let lon = (-up.z).atan2(up.x).to_degrees();
        let fine = Domain::patch(lat, lon, 128, 250., planet.params.radius);
        let b = vents(&planet, &fine, &p);
        for v in &b {
            if v.cell.is_some() {
                if let Some(w) = a.iter().find(|w| (w.dir - v.dir).norm() < 1e-12) {
                    assert_eq!((w.start, w.end, w.rate), (v.start, v.end, v.rate));
                }
            }
        }
    }

    #[test]
    fn test_eruptions_conserve_volume_and_build_cones() {
        let (planet, d) = volcanic_patch();
        let p = VolcanoParams { horizon: 1., ..VolcanoParams::default() };
        let vs = vents(&planet, &d, &p);
        let inside: Vec<&Vent> = vs.iter().filter(|v| v.cell.is_some()).collect();
        assert!(!inside.is_empty());
        let mut ground: Vec<f64> = (0..d.len()).map(|i| 100. + (i % 64) as f64).collect();
        let mut lava = vec![0.; d.len()];
        let fixed = vec![false; d.len()];
        let before: f64 = ground.iter().zip(&d.area).map(|(h, a)| h * a).sum();
        let mut erupted = 0.;
        for k in 0..10 {
            erupted += erupt(&d, &vs, &p, k as f64 * 1000., 1000., &mut ground, &mut lava, &fixed);
        }
        let after: f64 = ground.iter().zip(&d.area).map(|(h, a)| h * a).sum();
        assert!(erupted > 0.);
        assert!((after - before - erupted).abs() < 1e-6 * erupted);
        // Lava flows reach beyond the cones.
        let beyond = (0..d.len()).filter(|&i| lava[i] > 0. && vs.iter().all(|v| d.radius * v.dir.angle(&d.dirs[i]) > p.cone_radius)).count();
        assert!(beyond > 0);
        // Without flows, the vent's cell is the top of its cone.
        let cones = VolcanoParams { lava_fraction: 0., ..p.clone() };
        let mut cone = vec![0.; d.len()];
        erupt(&d, &vs, &cones, 0., 1000., &mut ground, &mut cone, &fixed);
        let c = inside[0].cell.unwrap();
        for l in d.links(c) { assert!(cone[c] >= cone[l.to], "{} vs {}", cone[c], cone[l.to]); }
    }
}
