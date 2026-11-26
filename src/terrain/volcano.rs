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
//! are hundreds of metres across, smoothing away the small relief that steers real flows.
//! A step's lava goes as many flows of at most `FLOW_VOLUME` (a shield's, `SHIELD_FLOW_VOLUME`:
//! its eruptions are larger and tube-fed, so run tens of kilometres down its flanks), each
//! along its own perturbed path, laid its vent's flow thickness deep across its width
//! (`FLOW_WIDTH`, `SHIELD_FLOW_WIDTH`: the cells beside the path, out to half the width either
//! side, `lateral`, so a flow is as thick on any grid; as far as the ground beside stays below
//! the flow's top, so confined to a valley it lies deeper, up to `CONFINED` thicknesses, and
//! runs farther: flows fill the lows between earlier ones, stacking compensationally), up to
//! about `FLOW_LENGTH` (`SHIELD_FLOW_LENGTH`) as its volume lasts. In a hollow a flow fills
//! to the lip and spills over the hollow's lowest side, never back the way it came. Over many steps the flows spread down different valleys, building a lava
//! apron. A flow stops at the sea or the edge of a patch: lava
//! running into the sea or out of the domain is lost to it (no longer ponded at its edge).

use crate::geometry::Pcg32;
use std::collections::HashMap;
use crate::na::Vector3;
use crate::terrain::domain::{sphere_dir, Domain, Layout};
use crate::procedural::nubis_clouds::smoothstep;
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
    /// The cells about its summit (its cone's weight over `SUMMIT`), where its flows break out.
    summit: Vec<usize>,
}

/// Flows break out anywhere about a vent's summit: where its cone's weight is over this.
const SUMMIT: f64 = 0.85;

impl Vent {
    /// A vent at `dir` erupting `rate` m³/yr between `start` and `end`, building `form` out to
    /// `cone_radius` with `lava_fraction` of what it erupts flowing away in flows
    /// `flow_thickness` thick; placed in `domain`.
    #[allow(clippy::too_many_arguments)]
    fn new(dir: Vector3<f64>, (start, end): (f64, f64), rate: f64, seed: u64, form: Form, cone_radius: f64, lava_fraction: f64, flow_thickness: f64,
           caldera: Option<(f64, f64)>, domain: &Domain) -> Vent {
        let mut vent = Vent {
            dir, start, end, rate, seed, cell: None, cone: vec![], cone_radius, lava_fraction, flow_thickness, rifts: vec![],
            form, integral: form.integral(cone_radius), caldera, collapse: vec![], summit: vec![],
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
        // Half of a shield's magma is intruded into its rift zones and summit, inflating the
        // shield the planet already grows (`hotspot`), so isn't erupted here; nine tenths of
        // the rest flows, in lobes stacked metres thick (so that, as Kīlauea's, they cover a
        // few square kilometres a year), and a tenth builds about its summit.
        let mut vent = Vent::new(s.dir, (0., f64::INFINITY), 0.5 * rate, 0x6873_0000_0000 ^ k as u64, Form::Shield, 0.5 * s.radius, 0.9, 8., None, domain);
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

/// The most lava one flow carries, cubic metres: a step's lava is shared among flows of at most
/// this (Kīlauea's flows of the 1983-2018 eruption were mostly 0.01 to 0.1 km³).
pub const FLOW_VOLUME: f64 = 5e7;
/// The most lava one of a shield's flows carries, cubic metres: a growing shield's eruptions
/// are larger and longer, fed through tubes (Mauna Loa's of 1859 and 1984 were 0.38 and
/// 0.22 km³, and ran 51 and 27 km), so its lava is spread over its flanks, not heaped by
/// its summit.
pub const SHIELD_FLOW_VOLUME: f64 = 2.5e8;
/// How wide a shield's flows spread, metres.
pub const SHIELD_FLOW_WIDTH: f64 = 1000.;
/// The most flows a vent sends in a step.
const MAX_FLOWS: usize = 64;
/// How wide a shield's flows spread, metres (a stratovolcano's half as wide).
pub const FLOW_WIDTH: f64 = 1500.;
/// The longest a flow runs, metres, give or take half.
pub const FLOW_LENGTH: f64 = 20e3;
/// How many flow thicknesses deep a flow confined to a valley lies.
const CONFINED: f64 = 3.;
/// The longest a shield's flow runs, metres, give or take half.
pub const SHIELD_FLOW_LENGTH: f64 = 35e3;

/// The cells a flow of `width` metres covers across its path at cell `c`, heading to `next`
/// (the path's cell, and those beside it out to half the width either way, walked across the
/// grid square to the flow's way, stopping at `ground` higher than the path's by more than the
/// flow's `thickness`), and the share of its width beyond `fixed` cells (into the sea or out of
/// the domain).
fn lateral(domain: &Domain, c: usize, next: Option<usize>, width: f64, fixed: &[bool], ground: &dyn Fn(usize) -> f64, thickness: f64) -> (Vec<usize>, f64) {
    let mut cells = vec![c];
    let cell = domain.area[c].sqrt();
    let Some(next) = next else { return (cells, 0.) };
    if 0.5 * width < cell { return (cells, 0.); }
    let up = domain.dirs[c];
    let along = (domain.dirs[next] - up).normalize();
    let across = up.cross(&along).normalize();
    // Each step the edge neighbour furthest across, less how far it strays from the line
    // square to the flow through the path's cell.
    let score = |n: usize, side: f64| {
        let o = domain.dirs[n] - up;
        side * o.dot(&across) - 2. * o.dot(&along).abs()
    };
    let half = 0.5 * width / domain.radius;
    let mut gone = 0.;
    for side in [1., -1.] {
        let mut at = c;
        loop {
            let reached = side * (domain.dirs[at] - up).dot(&across);
            let Some(l) = domain.edges(at).iter().max_by(|a, b| score(a.to, side).total_cmp(&score(b.to, side))) else { break };
            let o = domain.dirs[l.to] - up;
            if side * o.dot(&across) <= reached || side * o.dot(&across) > half { break; }
            if fixed[l.to] {
                gone += (half - reached) / (2. * half);
                break;
            }
            // (Lava doesn't climb the side of its valley above its own top.)
            if ground(l.to) > ground(c) + thickness { break; }
            at = l.to;
            cells.push(at);
        }
    }
    (cells, gone)
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
        if w > SUMMIT { vent.summit.push(c); }
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
///
/// `record`, if given, keeps when lava last covered each cell (the step's end) and how much of
/// it was ʻaʻā (`Terrain::lava_time`, `lava_aa`). Basalt from a shield pours as pāhoehoe, its
/// smooth, ropy skin unbroken, near its vent; downstream, as it cools and stiffens, and on
/// steep slopes where it is torn as it flows, it turns to ʻaʻā, rubble of clinker (Peterson
/// and Tilling, "Transition of basaltic lava from pahoehoe to aa", JVGR 7, 1980: Kīlauea's
/// flows turn within a few kilometres, on slopes over about 5°). The more viscous lavas of
/// stratovolcanoes and cinder cones flow as ʻaʻā (or blockier) from the start.
#[allow(clippy::too_many_arguments)]
pub fn erupt(domain: &Domain, vents: &[Vent], p: &VolcanoParams, t: f64, dt: f64, ground: &mut [f64], lava: &mut [f64], fixed: &[bool],
             mut record: Option<(&mut [f64], &mut [f64])>) -> f64 {
    let mut mark = |c: usize, aa: f64| if let Some((when, kind)) = record.as_mut() {
        when[c] = t + dt;
        kind[c] = aa;
    };
    let mut total = 0.;
    for vent in vents {
        // A volcano that collapses does so as it dies: its summit drops into the emptied
        // magma chamber, a caldera with a level floor and steep walls (not counted as erupted).
        if let Some((r, depth)) = vent.caldera && t < vent.end && vent.end <= t + dt {
            for &(c, d) in &vent.collapse {
                let drop = depth * (1. - smoothstep(0.85 * r, r, d));
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
        // (A shield's growth about its summit is its intrusions and the lobes stacked within
        // it, not fresh flows over its surface: those are its flows'. A stratovolcano's or a
        // cinder cone's is its scoria and ash, rubbly as ʻaʻā.)
        let fresh_cone = vent.form != Form::Shield;
        for &(c, w) in &vent.cone {
            let h = cone * w / weight;
            ground[c] += h;
            lava[c] += h;
            total += h * domain.area[c];
            if fresh_cone && h > 0.05 { mark(c, 1.); }
        }

        // Lava flows only from vents in the domain, from the vent or one of its rift zones:
        // the step's lava as many flows, each of at most `FLOW_VOLUME`, each as long as its
        // volume runs out spread `FLOW_WIDTH` wide and `flow_thickness` deep, as far as
        // `FLOW_LENGTH` (lava cools as it goes: the longest of Hawaii's run 50 km, most a few to
        // twenty), down the steepest way over ground perturbed by up to `roughness`.
        let mut rng = Pcg32::new(vent.seed, t.to_bits());
        let lava_volume = volume * vent.lava_fraction;
        let shield = vent.form == Form::Shield;
        let most = if shield { SHIELD_FLOW_VOLUME } else { FLOW_VOLUME };
        let flows = ((lava_volume / most).ceil() as usize).clamp(1, MAX_FLOWS);
        let width = if shield { SHIELD_FLOW_WIDTH } else { 0.5 * FLOW_WIDTH };
        let longest = if shield { SHIELD_FLOW_LENGTH } else { FLOW_LENGTH };
        for _ in 0..flows {
            let pick = (rng.next_f64() * (1 + vent.rifts.len()) as f64) as usize;
            let summit = (!vent.summit.is_empty()).then(|| vent.summit[((rng.next_f64() * vent.summit.len() as f64) as usize).min(vent.summit.len() - 1)]);
            let start = if pick == 0 { summit.or(vent.cell) } else { vent.rifts.get(pick - 1).copied() };
            let Some(mut c) = start.or(vent.cell) else { break };
            let per_metre = vent.flow_thickness * width;
            // The flow's lava, used as it goes; and how far it has run.
            let mut left = (lava_volume / flows as f64).min(per_metre * longest * (0.5 + rng.next_f64()));
            let mut run = 0.;
            let mut came = None;
            // What this flow has laid, so the ground it meets beside it is the ground it found.
            let mut own: HashMap<usize, f64> = HashMap::new();
            while left > 0. {
                if fixed[c] { break; } // into the sea, or beyond the domain
                // Down the steepest way; or, in a hollow (often of the flow's own making, its
                // lava beside it raising the way ahead), ponding and spilling over its lowest
                // side, never back the way it came.
                let next = domain.links(c).iter()
                    .filter(|l| Some(l.to) != came)
                    .map(|l| (l.to, (ground[c] - ground[l.to] - p.roughness * rng.next_f64()) / l.dist, l.dist))
                    .max_by(|a, b| a.1.total_cmp(&b.1));
                // In a hollow, the flow ponds: its lava fills the cell to the lip it spills over
                // (a metre above the lowest neighbour) before going on, as much as it has left.
                if let Some((to, slope, _)) = next && slope <= 0. {
                    let fill = ((ground[to] - ground[c] + 1.) * domain.area[c]).min(left).max(0.);
                    ground[c] += fill / domain.area[c];
                    lava[c] += fill / domain.area[c];
                    mark(c, 0.);
                    total += fill;
                    left -= fill;
                    if left <= 0. { break; }
                }
                // The flow's volume over this step along it, across its width: the cells
                // beside the path, out to half the width either side, walked across the grid
                // square to the flow, as far as the ground beside stays below the flow's top
                // (on cells wider than the flow, the path's cell alone). Confined to a valley,
                // it lies deeper there, up to `CONFINED` flow thicknesses, and runs farther.
                let step = next.map_or(domain.area[c].sqrt(), |(_, _, dist)| dist);
                let laid = (per_metre * step).min(left);
                let found = |n: usize| ground[n] - own.get(&n).copied().unwrap_or(0.);
                // (The share of its width that runs into the sea or out of the domain is lost.)
                let (across, gone) = lateral(domain, c, next.map(|(to, _, _)| to), width, fixed, &found, vent.flow_thickness);
                left -= laid * gone;
                let laid = laid * (1. - gone);
                let aa = match vent.form {
                    Form::Shield => {
                        let slope = next.map_or(0., |(_, s, _)| s);
                        // (Not by the vent, where the flows pile up into a cone of their own.)
                        smoothstep(4e3, 12e3, run).max(smoothstep(0.08, 0.2, slope) * smoothstep(1e3, 3e3, run))
                    }
                    _ => 1.,
                };
                let mut lay = |c: usize, v: f64| {
                    ground[c] += v / domain.area[c];
                    lava[c] += v / domain.area[c];
                    mark(c, aa);
                };
                let mut used = 0.;
                for &n in &across {
                    let v = (laid / across.len() as f64).min(CONFINED * vent.flow_thickness * domain.area[n]);
                    lay(n, v);
                    *own.entry(n).or_default() += v / domain.area[n];
                    used += v;
                }
                total += used;
                left -= used;
                run += step;
                match next {
                    Some((to, _, _)) if step > 0. => { came = Some(c); c = to; }
                    _ => break,
                }
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
            erupted += erupt(&d, &vs, &p, k as f64 * 1000., 1000., &mut ground, &mut lava, &fixed, None);
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
        erupt(&d, &stratos[..1], &cones, 0., 1000., &mut ground, &mut cone, &fixed, None);
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
    fn test_flows_record_when_and_what_lava_they_lay() {
        let (_, d) = volcanic_patch();
        let p = VolcanoParams::default();
        // A shield on a slope gentle near its vent: pāhoehoe there, ʻaʻā far down its flows.
        let centre = d.len() / 2 + 32;
        let v = Vent::new(d.dirs[centre], (0., 1e9), 1e7, 3, Form::Shield, 20000., 1., 4., None, &d);
        let mut ground: Vec<f64> = (0..d.len()).map(|i| 2000. - 0.03 * d.radius * d.dirs[i].angle(&d.dirs[centre])).collect();
        let mut lava = vec![0.; d.len()];
        let (mut when, mut aa) = (vec![crate::terrain::simulation::NO_LAVA; d.len()], vec![0.; d.len()]);
        let fixed = vec![false; d.len()];
        for k in 0..20 {
            erupt(&d, std::slice::from_ref(&v), &p, k as f64 * 100., 100., &mut ground, &mut lava, &fixed, Some((&mut when, &mut aa)));
        }
        let flowed: Vec<usize> = (0..d.len()).filter(|&i| when[i] > 0.).collect();
        assert!(!flowed.is_empty());
        assert!(flowed.iter().all(|&i| when[i] <= 2000. && lava[i] > 0.));
        let near = |i: usize| d.radius * d.dirs[i].angle(&d.dirs[centre]) < 3000.;
        for &i in flowed.iter().filter(|&&i| near(i)) { assert!(aa[i] < 0.5, "{} m from the vent: aa {}", d.radius * d.dirs[i].angle(&d.dirs[centre]), aa[i]); }
        assert!(flowed.iter().any(|&i| aa[i] > 0.5), "flows turn to ʻaʻā downstream");
        // A stratovolcano's are ʻaʻā from the start.
        let s = Vent::new(d.dirs[centre], (0., 1e9), 1e6, 4, Form::Strato { crater: 300. }, 6000., 1., 10., None, &d);
        let (mut when, mut aa) = (vec![crate::terrain::simulation::NO_LAVA; d.len()], vec![0.; d.len()]);
        erupt(&d, std::slice::from_ref(&s), &p, 0., 100., &mut ground, &mut lava, &fixed, Some((&mut when, &mut aa)));
        assert!((0..d.len()).filter(|&i| when[i] > 0.).all(|i| aa[i] == 1.));
    }

    #[test]
    fn test_a_flow_is_as_thick_on_any_grid() {
        // One shield flow on a slope, on grids of 100 m and 25 m: it lies its own thickness
        // deep across its width, not heaped along its line on the finer.
        let planet = Planet::new(crate::terrain::planet::PlanetParams { seed: 5, ..Default::default() });
        for spacing in [100., 25.] {
            let d = Domain::patch(10., 20., 128, spacing, planet.params.radius);
            let centre = d.len() / 2 + 64;
            let thickness = 8.;
            let v = Vent::new(d.dirs[centre], (0., 1e9), 2e7, 3, Form::Shield, 2000., 1., thickness, None, &d);
            let mut ground: Vec<f64> = (0..d.len()).map(|i| 2000. - 0.1 * d.radius * d.dirs[i].angle(&d.dirs[centre])).collect();
            let mut lava = vec![0.; d.len()];
            // (Flows leave a patch at its edge, as in a simulation.)
            let edge: Vec<bool> = (0..d.len()).map(|i| d.edges(i).len() < 4).collect();
            erupt(&d, std::slice::from_ref(&v), &VolcanoParams { roughness: 0., ..VolcanoParams::default() }, 0., 1., &mut ground, &mut lava, &edge, None);
            let most = lava.iter().cloned().fold(0., f64::max);
            assert!(most > 0.5 * thickness && most < 3. * thickness, "{} m grid: {} m thick", spacing, most);
        }
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
        erupt(&d, std::slice::from_ref(&v), &p, 0., 1000., &mut ground, &mut lava, &fixed, None);
        let c = v.cell.unwrap();
        let built = ground[c];
        erupt(&d, std::slice::from_ref(&v), &p, 1000., 1000., &mut ground, &mut lava, &fixed, None);
        // Its summit dropped by the caldera's depth, less what the last of it built.
        assert!(ground[c] < built - 400., "{} after {}", ground[c], built);
        // Not beyond the caldera's rim.
        let far = (0..d.len()).find(|&i| { let r = d.radius * v.dir.angle(&d.dirs[i]); r > 2500. && r < 4000. }).unwrap();
        assert!(ground[far] > 0.);
        let _ = planet;
    }
}
