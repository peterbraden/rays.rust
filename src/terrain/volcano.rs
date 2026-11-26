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
//! A fraction of the erupted volume builds the edifice, spread over the cells within
//! `cone_radius` of the vent by its form's profile (`Form`), so steady eruption builds it:
//!
//! - **Stratovolcanoes** (the arcs' vents): concave, steepening towards the summit, as
//!   Fuji's profile, about (1 - r/R)^1.8 (Karátson et al., "The regular shape of
//!   stratovolcanoes", JVGR 193, 2010), with a summit crater a few hundred metres across. A
//!   fifth collapse into a **caldera** as they die: their summit drops into the emptied magma
//!   chamber, a level floor a third to nearly half as wide as the cone and 300-700 m deep, with
//!   steep walls (Crater Lake's Mount Mazama); the landscape then fills it with a lake or
//!   sediment.
//! - **Cinder cones**: a field of four to twelve about every volcano (as Etna's flanks hold
//!   hundreds), and along the rift zones of a hotspot's young shield (as Mauna Kea's): each
//!   erupting once, for decades to centuries, building a straight-sided cone at the angle of
//!   repose of cinders, 300-550 m in radius and a hundred metres or two high, its crater a
//!   third of that deep, with short flows (Parícutin grew 424 m in nine years).
//! - **Shields**: a hotspot's (`hotspot`), convex, with most of what they erupt flowing away.
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

/// What a vent builds about itself (`Form::weight`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Form {
    /// A shield's broad, convex body (a hotspot's: `hotspot`).
    Shield,
    /// A stratovolcano: concave, steepening towards the summit, with a crater `crater` metres
    /// across its rim.
    Strato { crater: f64 },
    /// A cinder cone: small, straight-sided at the angle of repose of loose cinders (about
    /// 30°), with a deep crater.
    Cinder { crater: f64 },
}

impl Form {
    /// How much of what builds the cone stands `d` metres from the vent, on a cone of radius
    /// `r`, relative to the summit's rim (0 beyond it).
    pub fn weight(&self, d: f64, r: f64) -> f64 {
        let x = d / r;
        if x >= 1. { return 0.; }
        match *self {
            Form::Shield => 1. - x.powf(1.7),
            // Fuji's profile: slopes of 30° and more by the summit falling to a few degrees on
            // the apron, about (1 - x)^1.8 (Karátson, Favalli et al., "The regular shape of
            // stratovolcanoes", JVGR 193, 2010); a crater a tenth of the height deep.
            Form::Strato { crater } => {
                let xc = (crater / r).min(0.5);
                let rim = (1. - xc).powf(1.8);
                if x >= xc { (1. - x).powf(1.8) } else { rim - 0.1 * (1. - (x / xc).powi(2)) }
            }
            // A crater about a third of the cone's height deep.
            Form::Cinder { crater } => {
                let xc = (crater / r).min(0.5);
                let rim = 1. - xc;
                if x >= xc { 1. - x } else { rim - 0.35 * rim * (1. - (x / xc).powi(2)) }
            }
        }
    }

    /// The weights' integral over the whole cone (square metres, times its height), found
    /// numerically.
    fn integral(&self, r: f64) -> f64 {
        let n = 400;
        let dr = r / n as f64;
        (0..n).map(|k| {
            let d = (k as f64 + 0.5) * dr;
            self.weight(d, r) * std::f64::consts::TAU * d * dr
        }).sum()
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
    /// Its cone's radius (m), the share of what it erupts that flows away as lava, and how
    /// thick its flows are (m): `VolcanoParams`' for most, a shield's own for a hotspot's.
    cone_radius: f64,
    lava_fraction: f64,
    flow_thickness: f64,
    /// Where its flows start besides its own cell: a shield's rift zones.
    rifts: Vec<usize>,
    /// What it builds, and the integral of its weights over the whole cone (`Form::integral`).
    pub form: Form,
    integral: f64,
    /// The caldera it collapses into when it dies, if it does: radius and depth, metres.
    pub caldera: Option<(f64, f64)>,
    /// The cells its caldera takes, and how far each is from the vent.
    collapse: Vec<(usize, f64)>,
}

impl Vent {
    /// A vent at `dir` erupting `rate` m³/yr between `start` and `end`, building `form` out to
    /// `cone_radius` with `lava_fraction` of what it erupts flowing away in flows
    /// `flow_thickness` thick; placed in `domain`.
    #[allow(clippy::too_many_arguments)]
    fn new(dir: Vector3<f64>, (start, end): (f64, f64), rate: f64, seed: u64, form: Form, cone_radius: f64, lava_fraction: f64, flow_thickness: f64,
           caldera: Option<(f64, f64)>, domain: &Domain) -> Vent {
        let mut vent = Vent {
            dir, start, end, rate, seed, cell: None, cone: vec![], cone_radius, lava_fraction, flow_thickness, rifts: vec![],
            form, integral: form.integral(cone_radius), caldera, collapse: vec![],
        };
        place(&mut vent, domain);
        vent
    }
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
        let end = start + p.duration * (0.5 + length);
        let seed = ((k as u64) << 32) ^ rng.next_u32() as u64;
        // A stratovolcano, a fifth of which collapse into a caldera when they die (as Crater
        // Lake's Mazama did), a third to nearly half as wide as the cone and hundreds of
        // metres deep.
        let (crater, collapses, width, depth) = (rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64());
        let caldera = (collapses < 0.2).then_some(((0.3 + 0.15 * width) * p.cone_radius, 300. + 400. * depth));
        let form = Form::Strato { crater: 300. + 400. * crater };
        let vent = Vent::new(dir, (start, end), p.rate * (0.25 + 1.5 * rate), seed, form, p.cone_radius, p.lava_fraction, p.flow_thickness, caldera, domain);
        cinder_cones(&vent, p.cone_radius * 1.5, &[], &mut rng, domain, &mut vents);
        if !vent.cone.is_empty() { vents.push(vent); }
    }
    // The hotspots' young shields, building: basalt erupted many times as fast as an arc
    // volcano's, mostly as thin, long flows of pāhoehoe from the summit and the rift zones;
    // and cinder cones along the rift zones, as on Mauna Kea's flanks.
    for (k, s) in planet.shields().list.iter().enumerate() {
        let activity = crate::terrain::hotspot::activity(s.age);
        if activity < 0.05 { continue; }
        let rate = HOTSPOT_RATE * s.height / crate::terrain::hotspot::HEIGHT * activity;
        let mut vent = Vent::new(s.dir, (0., f64::INFINITY), rate, 0x6873_0000_0000 ^ k as u64, Form::Shield, 0.5 * s.radius, 0.8, 4., None, domain);
        // (Cells holding a point of a rift zone; on a patch, `cell_at` gives the nearest edge
        // cell for points beyond it.)
        let rifts = s.rift_points(radius);
        vent.rifts = rifts.iter().filter_map(|d| {
            let c = domain.cell_at(d);
            (radius * domain.dirs[c].angle(d) <= domain.area[c].sqrt()).then_some(c)
        }).collect();
        let mut rng = Pcg32::new(planet.params.seed ^ 0x6369_6e64, k as u64);
        let horizon = Vent { start: 0., end: p.horizon, ..Vent::new(s.dir, (0., 0.), 0., 0, Form::Shield, 0., 0., 0., None, domain) };
        cinder_cones(&horizon, 0., &rifts, &mut rng, domain, &mut vents);
        if !vent.cone.is_empty() { vents.push(vent); }
    }
    vents
}

/// The cinder cones about `parent` (a field of them, as about every volcano): monogenetic
/// vents, each erupting once for decades to centuries, building a steep cone a few hundred
/// metres across and a hundred or two high (Parícutin grew 424 m in nine years; most are
/// smaller), with a deep crater and short flows. From `parent`'s start to its end, within
/// `reach` metres of it, or along `along`'s points when given.
fn cinder_cones(parent: &Vent, reach: f64, along: &[Vector3<f64>], rng: &mut Pcg32, domain: &Domain, out: &mut Vec<Vent>) {
    let count = 4 + (rng.next_f64() * 9.) as usize;
    let radius = domain.radius;
    let east = {
        let a = if parent.dir.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
        a.cross(&parent.dir).normalize()
    };
    let north = parent.dir.cross(&east);
    let life = if parent.end.is_finite() { parent.end - parent.start } else { 0. };
    for i in 0..count {
        let (a, b, when, length, size, volume, crater) = (rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64());
        let dir = if along.is_empty() {
            // Over the disc about the vent, as many anywhere.
            let (r, t) = (reach * a.sqrt(), b * std::f64::consts::TAU);
            (parent.dir * radius + (east * t.cos() + north * t.sin()) * r).normalize()
        } else {
            let p = along[((a * along.len() as f64) as usize).min(along.len() - 1)];
            let jitter = 1500. * (b - 0.5);
            (p * radius + east * jitter).normalize()
        };
        let start = parent.start + when * life;
        let duration = 50. + 950. * length;
        let cone_radius = 300. + 250. * size;
        let rate = (1.5e7 + 3.5e7 * volume) / duration;
        let form = Form::Cinder { crater: (0.25 + 0.1 * crater) * cone_radius };
        let vent = Vent::new(dir, (start, start + duration), rate, parent.seed ^ (0x6369 + i as u64) << 16, form, cone_radius, 0.25, 8., None, domain);
        if !vent.cone.is_empty() { out.push(vent); }
    }
}

/// A hotspot's supply of basalt, cubic metres per year, for a full-height shield in its main
/// stage: Hawaii's is about 0.1 to 0.2 km³ a year in all, shared between two or three active
/// shields (Robinson and Eakins, "Estimated magma supply rates for the Hawaiian Islands",
/// JVGR 151, 2006).
pub const HOTSPOT_RATE: f64 = 5e7;

/// Find the vent's cell, the cells its cone covers and those its caldera would take.
fn place(vent: &mut Vent, domain: &Domain) {
    let distance = |c: usize| domain.radius * vent.dir.angle(&domain.dirs[c]);
    let start = domain.cell_at(&vent.dir);
    // On a patch, cell_at gives the nearest edge cell for vents beyond the edge.
    let cell_size = domain.area[start].sqrt();
    if distance(start) <= cell_size { vent.cell = Some(start); }
    if vent.cone_radius <= 0. { return; }
    let caldera = vent.caldera.map_or(0., |(r, _)| r);
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(c) = queue.pop_front() {
        let d = distance(c);
        let w = vent.form.weight(d, vent.cone_radius);
        if w > 0. { vent.cone.push((c, w)); }
        else if Some(c) == vent.cell { vent.cone.push((c, 1.)); } // cells larger than the cone
        if d < caldera { vent.collapse.push((c, d)); }
        if d >= vent.cone_radius { continue; }
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
        // A volcano that collapses does so as it dies: its summit drops into the emptied
        // magma chamber, a caldera with a level floor and steep walls (not counted as erupted).
        if let Some((r, depth)) = vent.caldera && t < vent.end && vent.end <= t + dt {
            for &(c, d) in &vent.collapse {
                let drop = depth * (1. - crate::procedural::nubis_clouds::smoothstep(0.85 * r, r, d));
                ground[c] -= drop;
                lava[c] -= drop.min(lava[c]);
            }
        }
        let active = (t + dt).min(vent.end) - t.max(vent.start);
        if active <= 0. { continue; }
        let volume = vent.rate * active;

        // Only the part of the cone in the domain is built. (Cells larger than the cone take
        // all of it.)
        let cone = volume * (1. - vent.lava_fraction);
        let in_domain: f64 = vent.cone.iter().map(|&(c, w)| w * domain.area[c]).sum();
        let weight = in_domain.max(vent.integral);
        for &(c, w) in &vent.cone {
            let h = cone * w / weight;
            ground[c] += h;
            lava[c] += h;
            total += h * domain.area[c];
        }

        // Lava flows only from vents in the domain: from the vent, or one of its rift zones.
        let mut rng = Pcg32::new(vent.seed, t.to_bits());
        let pick = (rng.next_f64() * (1 + vent.rifts.len()) as f64) as usize;
        let start = if pick == 0 { vent.cell } else { vent.rifts.get(pick - 1).copied() };
        let Some(mut c) = start.or(vent.cell) else { continue };
        let mut remaining = volume * vent.lava_fraction;
        total += remaining;
        for _ in 0..domain.len() {
            if remaining <= 0. { break; }
            // The steepest way down over ground perturbed by up to `roughness`.
            let next = domain.links(c).iter()
                .map(|l| (l.to, (ground[c] - ground[l.to] - p.roughness * rng.next_f64()) / l.dist))
                .filter(|&(_, s)| s > 0.)
                .max_by(|a, b| a.1.total_cmp(&b.1));
            let lay = if next.is_none() || fixed[c] { remaining } else { remaining.min(vent.flow_thickness * domain.area[c]) };
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
        let beyond = (0..d.len()).filter(|&i| lava[i] > 0. && vs.iter().all(|v| d.radius * v.dir.angle(&d.dirs[i]) > v.cone_radius + 1000.)).count();
        assert!(beyond > 0);
        // Without flows, a stratovolcano's cell is the top of its cone (its crater is smaller
        // than these cells).
        let cones = VolcanoParams { lava_fraction: 0., ..p.clone() };
        let mut stratos = vents(&planet, &d, &cones);
        stratos.retain(|v| matches!(v.form, Form::Strato { .. }) && v.cell.is_some() && v.caldera.is_none());
        let mut cone = vec![0.; d.len()];
        erupt(&d, &stratos[..1], &cones, 0., 1000., &mut ground, &mut cone, &fixed);
        let c = stratos[0].cell.unwrap();
        for l in d.links(c) { assert!(cone[c] >= cone[l.to], "{} vs {}", cone[c], cone[l.to]); }
        // About each volcano, a field of cinder cones.
        assert!(vs.iter().filter(|v| matches!(v.form, Form::Cinder { .. })).count() >= 4 * inside.iter().filter(|v| matches!(v.form, Form::Strato { .. })).count());
    }

    #[test]
    fn test_volcanoes_have_their_shapes() {
        let r = 6000.;
        let strato = Form::Strato { crater: 500. };
        // Concave: steeper near the summit than on the apron; a crater below its rim.
        let slope = |f: &Form, d: f64| (f.weight(d, r) - f.weight(d + 10., r)) / 10.;
        assert!(slope(&strato, 1000.) > 2. * slope(&strato, 4500.));
        assert!(strato.weight(0., r) < strato.weight(500., r));
        // A cinder cone: straight-sided, its crater a third of its height deep.
        let cinder = Form::Cinder { crater: 100. };
        assert!((slope(&cinder, 150.) - slope(&cinder, 300.)).abs() < 1e-9);
        let (rim, floor) = (cinder.weight(100., 400.), cinder.weight(0., 400.));
        assert!((rim - floor) / rim > 0.3);
        // A shield: convex, level at its summit.
        assert!(slope(&Form::Shield, 0.) < slope(&Form::Shield, 4000.));
        // Their integrals over the cone are what they hold: a cone's is π R² / 3.
        let cone = |d: f64, r: f64| (1. - d / r).max(0.);
        let n = 400;
        let numeric: f64 = (0..n).map(|k| { let d = (k as f64 + 0.5) * r / n as f64; cone(d, r) * std::f64::consts::TAU * d * r / n as f64 }).sum();
        assert!((numeric - std::f64::consts::PI * r * r / 3.).abs() < 1e-3 * numeric);
        assert!(strato.integral(r) > 0. && strato.integral(r) < numeric);
    }

    #[test]
    fn test_a_dying_volcano_collapses_into_a_caldera() {
        let (planet, d) = volcanic_patch();
        let p = VolcanoParams::default();
        let caldera = (2000., 500.);
        let v = Vent::new(d.dirs[d.len() / 2 + 32], (0., 1500.), 1e6, 1, Form::Strato { crater: 400. }, 6000., 0., 10., Some(caldera), &d);
        let mut ground = vec![0.; d.len()];
        let mut lava = vec![0.; d.len()];
        let fixed = vec![false; d.len()];
        erupt(&d, std::slice::from_ref(&v), &p, 0., 1000., &mut ground, &mut lava, &fixed);
        let c = v.cell.unwrap();
        let built = ground[c];
        erupt(&d, std::slice::from_ref(&v), &p, 1000., 1000., &mut ground, &mut lava, &fixed);
        // Its summit dropped by the caldera's depth, less what the last of it built.
        assert!(ground[c] < built - 400., "{} after {}", ground[c], built);
        // Not beyond the caldera's rim.
        let far = (0..d.len()).find(|&i| { let r = d.radius * v.dir.angle(&d.dirs[i]); r > 2500. && r < 4000. }).unwrap();
        assert!(ground[far] > 0.);
        let _ = planet;
    }
}
