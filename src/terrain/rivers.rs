//! # River surfaces: water at its own level, in a channel cut to its depth
//!
//! Step 2 of the water workstream (plan.md section 15). A landscape's rivers were a mask: ground
//! within half a river's width of a line between cell centres was shaded as water, at the
//! ground's own height, so a river ran up and down with the relief and had no depth. Here each
//! river is a smooth centreline carrying its hydraulics (`terrain::hydraulics`: width, depth,
//! the channel's bankfull depth), and gets:
//!
//! - a water level that never rises downstream and stays below its banks;
//! - a channel carved into the ground beneath it, as deep as the flow;
//! - a surface of its own (`ribbon`): transparent water through which the bed shows.
//!
//! ## Courses
//!
//! The simulation's river cells each drain to one receiver, so its rivers are a tree. A
//! course is a stretch of it between junctions: it starts at a source or a confluence and runs
//! down to the next confluence or the sea (`courses`). Meandering channels (`channel.rs`) are
//! stretches already drawn as fine centrelines, and replace the cells they cover, as they do in
//! the old mask. The cell-to-cell lines are smoothed by Chaikin's corner cutting (three rounds,
//! the ends kept so courses still meet at their junctions) and resampled at a few metres, finer
//! for narrower rivers. A tributary that joins a meandering channel part way along ends on the
//! channel's line.
//!
//! ## The level
//!
//! A river's surface lies below the floodplain it has cut into by the channel's freeboard: the
//! bankfull depth less the flow's depth (a river at bankfull flow is level with its banks;
//! below it, lower). So at each vertex the level is the lowest of the ground on the centreline
//! and on either bank, and the simulation's own bed there, less that freeboard. The bed matters
//! because the drawn surface is a spline through the cells' corners, each the mean of the
//! cells meeting there: in a narrow valley that lifts the floor towards its walls, by tens or
//! hundreds of metres and unevenly, while the simulation's bed is what its rivers were routed
//! down. Where the drawn ground stands above the level, the channel is cut down to it and its
//! banks rise at 1:1 as far as the cut needs (a gorge where the drawing smoothed a V); then, walking downstream, never higher than the
//! level upstream of it (a running minimum, taking in what joins from tributaries). Where the
//! ground rises in the river's way (a spur its smoothed line cuts across, a bump of the
//! synthetic relief), the channel is cut through it. Courses are walked in order of the area
//! draining to their ends, which puts every course after all those flowing into it.
//!
//! Where a river passes through a basin (a glacier's overdeepening, a hollow the simulation's
//! routing filled), the water would fill it to its spill level: a lake. Stretches where that
//! fill would stand more than `LAKE` metres over the ground are found (`lakes`), left to the
//! mask as before (lakes as surfaces are step 8), and the river starts its level afresh where
//! it leaves them. What this leaves out: lake surfaces and the gradually varied flow's backwater and
//! drawdown near controls (the level follows the banks, not the energy equation; falls, step
//! 4, need it at their lips).
//!
//! ## The valley floor
//!
//! The landscape's relief below the simulation's cells is synthetic (`Relief::detail`: ridged
//! noise, stronger on steep ground), and knows nothing of rivers: on the steep sides of a
//! mountain valley its bumps are tens of metres high, and laid across a valley floor they would
//! dam the river, which would then have to cut canyons through them. A river clears its own
//! path, and the floor beside it, of relief at every scale it can reach: so each octave of the
//! detail fades out towards the water's edge, within half its wavelength or the river's
//! floodplain, a few widths (`cleared`), whichever is less, and the river's line keeps the
//! simulation's own, smooth surface (`Skeleton::nearest`).
//!
//! And the drawn surface is a spline through the cells' corners, each the mean of the cells
//! meeting there, which in a narrow valley lifts the floor towards the walls, by tens or
//! hundreds of metres and unevenly along the river: the simulation's bed is where its rivers
//! actually run. So within a cell of a river the drawn surface is blended down (or up) to the
//! simulation's bed beneath the river (`Relief::base`), and valleys keep their V with the
//! river at the bottom.
//!
//! ## The channel
//!
//! The bed's cross-section is a parabola, its deepest point 1.5 times the mean depth (a
//! parabola's mean depth is two thirds of its greatest), meeting the water's edge at the
//! level; beyond the edge the bank rises at 1:1 until it meets the ground. The ground is the
//! lower of itself and that surface (`Carve::surface`). The carving is drawn only where the
//! mesh is fine enough to show it: fully where its spacing is under a quarter of the river's
//! width, fading out by half (`Carve::weight`), so far away the mask draws the river as before.
//! The water's surface (`ribbon`) is built over the same part, a little wider than the water
//! so its edges tuck into the banks.

use crate::na::Vector3;
use crate::procedural::nubis_clouds::smoothstep;
use crate::terrain::channel::{cell_point, point_cell, Point};
use crate::terrain::domain::{Domain, Layout};
use crate::terrain::hydraulics::Hydraulics;
use crate::terrain::map::is_river;
use crate::terrain::simulation::{Terrain, ICE_FREE};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::f64::consts::{PI, TAU};

/// A point of a river's centreline, before its level is known.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// Patch tangent coordinates, metres.
    pub p: Point,
    /// Half the water's width, metres.
    pub half: f64,
    /// The flow's mean depth, metres.
    pub depth: f64,
    /// How far below the banks the water lies: bankfull depth less the flow's.
    pub freeboard: f64,
    /// The simulation's own ground at the river (its cells' beds, interpolated), metres.
    pub bed: f64,
    /// The bed's slope there.
    pub slope: f64,
    /// Whether it is on a meandering channel (`channel.rs`), already sinuous.
    pub meanders: bool,
    /// The bed's median grain, metres.
    pub grain: f64,
    /// Suspended sediment, g/m³ (`river_water`).
    pub turbidity: f64,
    /// The flow's Froude number.
    pub froude: f64,
}

impl Node {
    /// Still, clear water, for where no river is found.
    pub fn still() -> Node {
        Node { p: [0., 0.], half: 1., depth: 1., freeboard: 0., bed: 0., slope: 1e-5, meanders: false, grain: 0.01, turbidity: 0., froude: 0. }
    }
}

/// A stretch of river between junctions, upstream to downstream.
#[derive(Clone, Debug)]
pub struct Course {
    pub nodes: Vec<Node>,
    /// The cells it starts and ends at (junctions, sources, outlets).
    pub start: usize,
    pub end: usize,
    /// Drainage area at its end, m²: courses are levelled in this order.
    pub area: f64,
    /// Cells part way along it where other courses end (tributaries joining a meandering
    /// channel), with the index of its node nearest each.
    pub joins: Vec<(usize, usize)>,
}

/// Chaikin's corner cutting, keeping the ends.
fn chaikin(nodes: &[Node]) -> Vec<Node> {
    if nodes.len() < 3 { return nodes.to_vec(); }
    let lerp = |a: &Node, b: &Node, t: f64| Node {
        p: [a.p[0] + (b.p[0] - a.p[0]) * t, a.p[1] + (b.p[1] - a.p[1]) * t],
        half: a.half + (b.half - a.half) * t,
        depth: a.depth + (b.depth - a.depth) * t,
        freeboard: a.freeboard + (b.freeboard - a.freeboard) * t,
        bed: a.bed + (b.bed - a.bed) * t,
        slope: a.slope + (b.slope - a.slope) * t,
        meanders: a.meanders || b.meanders,
        grain: a.grain + (b.grain - a.grain) * t,
        turbidity: a.turbidity + (b.turbidity - a.turbidity) * t,
        froude: a.froude + (b.froude - a.froude) * t,
    };
    let mut out = vec![nodes[0]];
    for (k, w) in nodes.windows(2).enumerate() {
        if k > 0 { out.push(lerp(&w[0], &w[1], 0.25)); }
        if k + 2 < nodes.len() { out.push(lerp(&w[0], &w[1], 0.75)); }
    }
    out.push(nodes[nodes.len() - 1]);
    out
}

/// Resampled so no step is longer than half the river's width (at least 2 m, at most 250).
fn densify(nodes: &[Node]) -> Vec<Node> {
    let mut out = vec![nodes[0]];
    for w in nodes.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        let len = (b.p[0] - a.p[0]).hypot(b.p[1] - a.p[1]);
        let step = a.half.min(b.half).clamp(2., 250.);
        let n = (len / step).ceil().max(1.) as usize;
        for s in 1..=n {
            let t = s as f64 / n as f64;
            out.push(Node {
                p: [a.p[0] + (b.p[0] - a.p[0]) * t, a.p[1] + (b.p[1] - a.p[1]) * t],
                half: a.half + (b.half - a.half) * t,
                depth: a.depth + (b.depth - a.depth) * t,
                freeboard: a.freeboard + (b.freeboard - a.freeboard) * t,
                bed: a.bed + (b.bed - a.bed) * t,
                slope: a.slope + (b.slope - a.slope) * t,
                meanders: a.meanders && b.meanders,
                grain: a.grain + (b.grain - a.grain) * t,
                turbidity: a.turbidity + (b.turbidity - a.turbidity) * t,
                froude: a.froude + (b.froude - a.froude) * t,
            });
        }
    }
    out
}

/// Meander wavelength over channel width (Leopold and Wolman 1960 found λ ≈ 10.9 w^1.01).
const MEANDER_WAVELENGTH: f64 = 11.;

/// The sinuosity (channel length over valley length) of a reach of slope `s`: winding on
/// gentle floodplains, nearly straight on steep ground. A model choice, to check against
/// measured relations (Schumm 1963 relates sinuosity to the banks' silt and clay instead).
fn sinuosity(s: f64) -> f64 {
    1.05 + 0.45 * (1. - smoothstep(0.001f64.ln(), 0.02f64.ln(), s.max(1e-6).ln()))
}

/// The course's line bent into meanders, unless already meandering: offset across itself by
/// a sine of wavelength `MEANDER_WAVELENGTH` widths, its amplitude giving the reach's
/// sinuosity (a sine of amplitude a and wavelength λ is longer than its axis by about
/// 1 + (πa/λ)², for small a), its phase wandering so the bends aren't regular, and the offset
/// tapering to nothing over a wavelength at each end so courses still meet at their junctions.
/// (Langbein and Leopold 1966's sine-generated curve is the better shape for large bends.)
fn bend(nodes: &[Node], seed: u64) -> Vec<Node> {
    if nodes.len() < 3 { return nodes.to_vec(); }
    let mut along = vec![0.];
    for w in nodes.windows(2) { along.push(along.last().unwrap() + dist(w[0].p, w[1].p)); }
    let total = *along.last().unwrap();
    let hash = |k: u64| ((seed.wrapping_mul(0x9e3779b97f4a7c15) ^ k.wrapping_mul(0xbf58476d1ce4e5b9)) >> 11) as f64 / (1u64 << 53) as f64;
    let (p1, p2, p3) = (TAU * hash(1), TAU * hash(2), TAU * hash(3));
    let mut phase = 0.;
    let mut out = nodes.to_vec();
    for j in 0..nodes.len() {
        let n = &nodes[j];
        let lambda = MEANDER_WAVELENGTH * 2. * n.half;
        if j > 0 { phase += TAU * (along[j] - along[j - 1]) / lambda; }
        if n.meanders { continue; }
        // From sinuosity P: πa/λ = √(P - 1).
        let a = lambda / PI * (sinuosity(n.slope) - 1.).sqrt();
        let taper = smoothstep(0., lambda, along[j]) * smoothstep(0., lambda, total - along[j]);
        let wander = 0.8 * (phase / 3.1 + p2).sin() + 0.5 * (phase / 7.3 + p3).sin();
        let y = a * taper * (phase + p1 + wander).sin() * (1. + 0.3 * (phase / 4.7 + p3).sin());
        let (pa, pb) = (nodes[j.saturating_sub(1)].p, nodes[(j + 1).min(nodes.len() - 1)].p);
        let (dx, dz) = (pb[0] - pa[0], pb[1] - pa[1]);
        let len = dx.hypot(dz).max(1e-9);
        out[j].p = [n.p[0] - dz / len * y, n.p[1] + dx / len * y];
    }
    out
}

fn dist(a: Point, b: Point) -> f64 { (a[0] - b[0]).hypot(a[1] - b[1]) }

/// The point of segment `a`-`b` nearest `q`, as the fraction along it, and the distance.
fn nearest_on(a: Point, b: Point, q: Point) -> (f64, f64) {
    let (ab, aq) = ([b[0] - a[0], b[1] - a[1]], [q[0] - a[0], q[1] - a[1]]);
    let len2 = ab[0] * ab[0] + ab[1] * ab[1];
    let t = if len2 > 0. { ((aq[0] * ab[0] + aq[1] * ab[1]) / len2).clamp(0., 1.) } else { 0. };
    (t, dist(q, [a[0] + ab[0] * t, a[1] + ab[1] * t]))
}

/// The rivers of patch `t` as courses, their hydraulics from `h`.
pub fn courses(t: &Terrain, h: &Hydraulics) -> Vec<Course> {
    if !matches!(t.domain.layout, Layout::Patch { .. }) { return vec![]; }
    let d = &t.domain;
    let outlets = t.outlets();
    let node_at = |i: usize, p: Point, meanders: bool| {
        let r = &h.reaches[i];
        Node { p, half: r.water_width / 2., depth: r.depth, freeboard: (r.bankfull_depth - 1.5 * r.depth).max(0.), bed: t.bed[i], slope: r.slope, meanders, grain: r.grain,
            turbidity: crate::terrain::river_water::concentration(t.sediment_flux[i], h.mean[i]), froude: r.froude }
    };
    // Edges of the river network: a cell to its receiver, or a meandering channel's line.
    let channels = &t.rivers.channels;
    let covered: BTreeMap<usize, usize> = channels.iter().enumerate().flat_map(|(k, c)| c.path.iter().map(move |&i| (i, k))).collect();
    let mut edges: BTreeMap<usize, (usize, Vec<Node>)> = BTreeMap::new();
    for (i, &outlet) in outlets.iter().enumerate() {
        let r = t.receiver[i];
        if !is_river(t, i) || outlet || t.ice[i] >= ICE_FREE || r == i { continue; }
        if covered.contains_key(&i) && covered.contains_key(&r) { continue; }
        edges.insert(i, (r, vec![node_at(i, cell_point(d, i), false), node_at(r, cell_point(d, r), false)]));
    }
    for c in channels {
        let nodes = c.points.iter().map(|&p| node_at(point_cell(d, p), p, true)).collect();
        edges.insert(c.start, (c.end, nodes));
    }
    // Courses: follow edges while the next node has one way in and one way on.
    let mut inflow: BTreeMap<usize, usize> = BTreeMap::new();
    for (to, _) in edges.values() { *inflow.entry(*to).or_default() += 1; }
    let mut courses = vec![];
    for &first in edges.keys() {
        if inflow.get(&first) == Some(&1) { continue; }
        let mut nodes: Vec<Node> = vec![];
        let mut at = first;
        let mut seen = BTreeSet::new();
        while let Some((to, line)) = edges.get(&at) {
            if !seen.insert(at) { break; }
            let skip = if nodes.is_empty() { 0 } else { 1 };
            nodes.extend_from_slice(&line[skip..]);
            at = *to;
            if inflow.get(&at) != Some(&1) || !edges.contains_key(&at) { break; }
        }
        if nodes.len() >= 2 {
            courses.push(Course { nodes, start: first, end: at, area: t.drainage[at], joins: vec![] });
        }
    }
    // Smooth, then fine enough to follow the ground, then bent.
    for c in &mut courses {
        let mut n = c.nodes.clone();
        for _ in 0..3 { n = chaikin(&n); }
        c.nodes = bend(&densify(&n), c.start as u64);
    }
    // Tributaries that end part way along a meandering channel: end on its line, and join it
    // there.
    let course_of_channel: HashMap<usize, usize> = courses.iter().enumerate()
        .filter(|(_, c)| channels.iter().any(|ch| ch.start == c.start)).map(|(k, c)| (c.start, k)).collect();
    for k in 0..courses.len() {
        let end = courses[k].end;
        let Some(&ch) = covered.get(&end) else { continue };
        let Some(&main) = course_of_channel.get(&channels[ch].start) else { continue };
        if main == k || channels[ch].start == end { continue; }
        let q = courses[k].nodes.last().unwrap().p;
        let (j, _) = courses[main].nodes.iter().enumerate()
            .min_by(|a, b| dist(a.1.p, q).total_cmp(&dist(b.1.p, q))).unwrap();
        let on = courses[main].nodes[j].p;
        courses[k].nodes.last_mut().unwrap().p = on;
        courses[main].joins.push((end, j));
    }
    courses.sort_by(|a, b| a.area.total_cmp(&b.area));
    courses
}

/// A river vertex with its level, and how far below the drawn ground that is; or in a lake.
#[derive(Clone, Copy, Debug)]
struct Vertex { node: Node, level: f64, cut: f64, lake: bool }

/// The level at which water arrives at each junction cell, metres: the lowest of the courses
/// ending there.
type Arrivals = HashMap<usize, f64>;

/// Depressions deeper than this (metres) along a river are lakes.
pub const LAKE: f64 = 5.;

/// Which nodes of a course are in lakes: where filling it from its end to the spill levels
/// (each node no lower than the one after it) would stand more than `LAKE` over `raw`.
pub fn lakes(raw: &[f64]) -> Vec<bool> {
    let mut fill = raw.to_vec();
    for j in (0..fill.len().saturating_sub(1)).rev() { fill[j] = fill[j].max(fill[j + 1]); }
    fill.iter().zip(raw).map(|(f, r)| f - r > LAKE).collect()
}

/// Courses' levels: at each node the lowest of `raw` (the ground there and on its banks, less
/// the freeboard), the level upstream, and what tributaries bring; except in lakes (`lake`),
/// out of which the river starts afresh (and whose levels are meaningless).
pub fn levels(courses: &[Course], raw: &[Vec<f64>], lake: &[Vec<bool>]) -> Vec<Vec<f64>> {
    let mut arrivals: Arrivals = HashMap::new();
    let mut out = Vec::with_capacity(courses.len());
    for ((c, raw), lake) in courses.iter().zip(raw).zip(lake) {
        let mut running = arrivals.get(&c.start).copied().unwrap_or(f64::INFINITY);
        let joins: HashMap<usize, usize> = c.joins.iter().map(|&(cell, j)| (j, cell)).collect();
        let mut levels = Vec::with_capacity(raw.len());
        for (j, &r) in raw.iter().enumerate() {
            if lake[j] {
                running = f64::INFINITY;
                levels.push(r);
                continue;
            }
            running = running.min(r);
            if let Some(cell) = joins.get(&j) && let Some(&a) = arrivals.get(cell) { running = running.min(a); }
            levels.push(running);
        }
        let end = arrivals.entry(c.end).or_insert(f64::INFINITY);
        *end = end.min(running);
        out.push(levels);
    }
    out
}

/// Where the ground rises beside the water, its slope (rise over run).
const BANK: f64 = 1.;
/// The water's surface reaches this much beyond its edge, as a fraction of the half width and
/// in metres, to tuck under the banks.
const TUCK: (f64, f64) = (0.05, 0.3);
/// The banks are sampled this far out (fraction of the half width, and metres).
const BANK_SAMPLE: (f64, f64) = (0.15, 1.);

/// A segment of a levelled river.
#[derive(Clone, Copy, Debug)]
struct Segment { a: Vertex, b: Vertex }

impl Segment {
    /// The river's half width, level and greatest depth at fraction `t` along.
    fn at(&self, t: f64) -> (f64, f64, f64) {
        let l = |x: f64, y: f64| x + (y - x) * t;
        (l(self.a.node.half, self.b.node.half), l(self.a.level, self.b.level), 1.5 * l(self.a.node.depth, self.b.node.depth))
    }

    /// How far beyond its centreline the carving can reach.
    fn reach(&self) -> f64 {
        let s = |v: &Vertex| v.node.half * (1. + BANK_SAMPLE.0) + BANK_SAMPLE.1 + (v.node.freeboard + v.cut + 2.) / BANK;
        s(&self.a).max(s(&self.b))
    }
}

/// A layer's levelled rivers, indexed for carving and for finding the water.
pub struct Carve {
    courses: Vec<Vec<Vertex>>,
    segments: Vec<Segment>,
    grid: HashMap<(i64, i64), Vec<u32>>,
    /// The grid's cell size, metres: about the segments' typical reach, so a wide river's
    /// segments don't each fill thousands of cells.
    cell: f64,
}

impl Carve {
    /// `levels`, the drawn `ground` and which are in lakes, at each course's nodes.
    pub fn new(courses: &[Course], levels: &[Vec<f64>], ground: &[Vec<f64>], lake: &[Vec<bool>]) -> Carve {
        let courses: Vec<Vec<Vertex>> = courses.iter().zip(levels).zip(ground).zip(lake)
            .map(|(((c, l), g), k)| c.nodes.iter().zip(l).zip(g).zip(k)
                .map(|(((&node, &level), &g), &lake)| Vertex { node, level, cut: if lake { 0. } else { (g - level).max(0.) }, lake }).collect()).collect();
        // Lakes are drawn as before (the mask; step 8 of the plan), so neither carved nor
        // given a surface.
        let segments: Vec<Segment> = courses.iter()
            .flat_map(|c| c.windows(2).filter(|w| !w[0].lake && !w[1].lake).map(|w| Segment { a: w[0], b: w[1] })).collect();
        let mut reaches: Vec<f64> = segments.iter().map(|s| s.reach()).collect();
        reaches.sort_by(|a, b| a.total_cmp(b));
        let cell = reaches.get(reaches.len() / 2).map_or(32., |r| (2. * r).clamp(32., 1024.));
        let mut grid: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        for (k, s) in segments.iter().enumerate() {
            {
                let (w, r) = ([s.a, s.b], s.reach());
                let (lo, hi) = (
                    [w[0].node.p[0].min(w[1].node.p[0]) - r, w[0].node.p[1].min(w[1].node.p[1]) - r],
                    [w[0].node.p[0].max(w[1].node.p[0]) + r, w[0].node.p[1].max(w[1].node.p[1]) + r]);
                for gx in (lo[0] / cell).floor() as i64..=(hi[0] / cell).floor() as i64 {
                    for gz in (lo[1] / cell).floor() as i64..=(hi[1] / cell).floor() as i64 {
                        grid.entry((gx, gz)).or_default().push(k as u32);
                    }
                }
            }
        }
        Carve { courses, segments, grid, cell }
    }

    fn near(&self, q: Point) -> impl Iterator<Item = &Segment> {
        let key = ((q[0] / self.cell).floor() as i64, (q[1] / self.cell).floor() as i64);
        self.grid.get(&key).into_iter().flatten().map(|&k| &self.segments[k as usize])
    }

    /// The carved surface at `q`: the bed in the channel, the banks beside it; None away from
    /// rivers. With it, the half width of the river nearest.
    pub fn surface(&self, q: Point) -> Option<(f64, f64)> {
        let mut best: Option<(f64, f64)> = None;
        for s in self.near(q) {
            let (t, r) = nearest_on(s.a.node.p, s.b.node.p, q);
            if r > s.reach() { continue; }
            let (half, level, deepest) = s.at(t);
            let h = if r < half { level - deepest * (1. - (r / half).powi(2)) } else { level + (r - half) * BANK };
            if best.is_none_or(|(b, _)| h < b) { best = Some((h, half)); }
        }
        best
    }

    /// How much of the carving a mesh of spacing `local` draws, for a river of half width
    /// `half`: all of it under half the half width, none from the whole.
    pub fn weight(half: f64, local: f64) -> f64 { 1. - smoothstep(0.5 * half, half, local) }

    /// Ground `g` at `q`, carved as a mesh of spacing `local` draws it.
    pub fn carved(&self, q: Point, g: f64, local: f64) -> f64 {
        match self.surface(q) {
            Some((s, half)) if s < g => g - Carve::weight(half, local) * (g - s),
            _ => g,
        }
    }

    /// The water at `q` if it is within a river (at least `min_width` wide): its level, its
    /// half width and the bed's grain; None beyond, and in lakes (`lakes` says where those
    /// are).
    pub fn level(&self, q: Point, min_width: f64) -> Option<(f64, f64, f64)> {
        let mut best: Option<(f64, f64, f64)> = None;
        for s in self.near(q) {
            let (t, r) = nearest_on(s.a.node.p, s.b.node.p, q);
            let (half, level, _) = s.at(t);
            let grain = s.a.node.grain + (s.b.node.grain - s.a.node.grain) * t;
            if r < half.max(min_width / 2.) && best.is_none_or(|b| level < b.0) { best = Some((level, half, grain)); }
        }
        best
    }

    /// The flow at `q` if it is on a river's surface (as `ribbon` draws it): the node there,
    /// interpolated, and the direction of flow (patch coordinates, unit).
    pub fn water_at(&self, q: Point) -> Option<(Node, Point)> {
        let mut best: Option<(f64, Node, Point)> = None;
        for s in self.near(q) {
            let (t, r) = nearest_on(s.a.node.p, s.b.node.p, q);
            let (half, _, _) = s.at(t);
            if r > half * (1. + TUCK.0) + TUCK.1 + 0.5 || best.as_ref().is_some_and(|b| b.0 <= r) { continue; }
            let (a, b) = (&s.a.node, &s.b.node);
            let l = |x: f64, y: f64| x + (y - x) * t;
            let node = Node {
                p: q, half, depth: l(a.depth, b.depth), freeboard: l(a.freeboard, b.freeboard), bed: l(a.bed, b.bed),
                slope: l(a.slope, b.slope), meanders: a.meanders, grain: l(a.grain, b.grain),
                turbidity: l(a.turbidity, b.turbidity), froude: l(a.froude, b.froude),
            };
            let (dx, dz) = (b.p[0] - a.p[0], b.p[1] - a.p[1]);
            let len = dx.hypot(dz).max(1e-12);
            best = Some((r, node, [dx / len, dz / len]));
        }
        best.map(|(_, n, d)| (n, d))
    }

    /// The river node nearest `q`, not in a lake: its point, level and node.
    pub fn nearest(&self, q: Point) -> Option<(Point, f64, Node)> {
        self.courses.iter().flatten().filter(|v| !v.lake)
            .min_by(|a, b| dist(a.node.p, q).total_cmp(&dist(b.node.p, q)))
            .map(|v| (v.node.p, v.level, v.node))
    }

    /// The water's surface as triangles: for each course, a strip a little wider than the
    /// water, at its level, over the vertices `keep` accepts (by point and half width), each
    /// vertex as (patch point, level).
    pub fn ribbon(&self, keep: impl Fn(Point, f64) -> bool) -> (Vec<(Point, f64)>, Vec<[usize; 3]>) {
        let mut vertices = vec![];
        let mut triangles = vec![];
        for c in &self.courses {
            let mut previous: Option<(usize, usize)> = None;
            for (j, v) in c.iter().enumerate() {
                if v.lake || !keep(v.node.p, v.node.half) { previous = None; continue; }
                let (a, b) = (c[j.saturating_sub(1)].node.p, c[(j + 1).min(c.len() - 1)].node.p);
                let (dx, dz) = (b[0] - a[0], b[1] - a[1]);
                let len = dx.hypot(dz).max(1e-9);
                let w = v.node.half * (1. + TUCK.0) + TUCK.1;
                let (nx, nz) = (-dz / len * w, dx / len * w);
                let p = v.node.p;
                let k = vertices.len();
                vertices.push(([p[0] + nx, p[1] + nz], v.level));
                vertices.push(([p[0] - nx, p[1] - nz], v.level));
                if let Some((l, r)) = previous {
                    triangles.push([l, r, k]);
                    triangles.push([r, k + 1, k]);
                }
                previous = Some((k, k + 1));
            }
        }
        (vertices, triangles)
    }
}

/// How far out to sample the banks of a river of half width `half`.
pub fn bank_offset(half: f64) -> f64 { half * (1. + BANK_SAMPLE.0) + BANK_SAMPLE.1 }

/// How far from a river the skeleton looks, metres: at most a cell's width of the patches it
/// is used on (valley floors are drawn at the simulation's beds over about a cell).
pub const CLEAR: f64 = 1024.;
const SKELETON_GRID: f64 = 256.;

/// How far beyond its edge a river clears the synthetic relief, metres: its floodplain, a few
/// widths, and at most half the relief's wavelength.
pub fn cleared(half: f64) -> f64 { 8. * half + 30. }

/// A segment of the skeleton: its ends, the half widths and the simulation's beds there.
type Bone = (Point, Point, f64, f64, f64, f64);

/// The courses' lines, coarsely, for the distance from the water (`Relief::detail`).
pub struct Skeleton {
    segments: Vec<Bone>,
    grid: HashMap<(i64, i64), Vec<u32>>,
}

impl Skeleton {
    pub fn new(courses: &[Course]) -> Skeleton {
        let mut segments = vec![];
        for c in courses {
            // Every few nodes: about every 50 m or more.
            let mut last = 0;
            for j in 1..c.nodes.len() {
                if dist(c.nodes[last].p, c.nodes[j].p) >= 50. || j + 1 == c.nodes.len() {
                    let (a, b) = (&c.nodes[last], &c.nodes[j]);
                    segments.push((a.p, b.p, a.half, b.half, a.bed, b.bed));
                    last = j;
                }
            }
        }
        let mut grid: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        for (k, &(a, b, ha, hb, _, _)) in segments.iter().enumerate() {
            let r = CLEAR + ha.max(hb);
            for gx in ((a[0].min(b[0]) - r) / SKELETON_GRID).floor() as i64..=((a[0].max(b[0]) + r) / SKELETON_GRID).floor() as i64 {
                for gz in ((a[1].min(b[1]) - r) / SKELETON_GRID).floor() as i64..=((a[1].max(b[1]) + r) / SKELETON_GRID).floor() as i64 {
                    grid.entry((gx, gz)).or_default().push(k as u32);
                }
            }
        }
        Skeleton { segments, grid }
    }

    /// The river nearest `q` (within `CLEAR`): how far `q` is from its centreline and from its
    /// water's edge (0 in the water), metres, its half width, and the simulation's bed there.
    pub fn nearest(&self, q: Point) -> Option<Nearest> {
        let key = ((q[0] / SKELETON_GRID).floor() as i64, (q[1] / SKELETON_GRID).floor() as i64);
        let mut best: Option<Nearest> = None;
        for &k in self.grid.get(&key).into_iter().flatten() {
            let (a, b, ha, hb, ba, bb) = self.segments[k as usize];
            let (t, r) = nearest_on(a, b, q);
            if r > CLEAR || best.is_some_and(|x| x.centre <= r) { continue; }
            let half = ha + (hb - ha) * t;
            best = Some(Nearest { centre: r, edge: (r - half).max(0.), half, bed: ba + (bb - ba) * t });
        }
        best
    }
}

/// The nearest river to a point (`Skeleton::nearest`).
#[derive(Clone, Copy, Debug)]
pub struct Nearest {
    pub centre: f64,
    pub edge: f64,
    pub half: f64,
    pub bed: f64,
}

impl Skeleton {
}

/// The rivers of a patch layer: their courses, and once its relief is known, their levels.
pub struct Rivers {
    pub courses: Vec<Course>,
    pub skeleton: Skeleton,
    pub carve: std::sync::OnceLock<Carve>,
}

impl Rivers {
    pub fn new(t: &Terrain, h: &Hydraulics) -> Rivers {
        let courses = courses(t, h);
        Rivers { skeleton: Skeleton::new(&courses), courses, carve: std::sync::OnceLock::new() }
    }

    /// Level the courses over `ground` (patch point, spacing → height, metres), unless done.
    pub fn level(&self, ground: impl Fn(Point, f64) -> f64 + Sync) {
        if self.carve.get().is_some() { return; }
        use rayon::prelude::*;
        let sampled: Vec<Vec<(f64, f64)>> = self.courses.par_iter().map(|c| {
            (0..c.nodes.len()).map(|j| {
                let n = &c.nodes[j];
                let (a, b) = (c.nodes[j.saturating_sub(1)].p, c.nodes[(j + 1).min(c.nodes.len() - 1)].p);
                let (dx, dz) = (b[0] - a[0], b[1] - a[1]);
                let len = dx.hypot(dz).max(1e-9);
                let o = bank_offset(n.half);
                let (nx, nz) = (-dz / len * o, dx / len * o);
                let local = (n.half / 2.).max(1.);
                let centre = ground(n.p, local);
                let g = centre
                    .min(ground([n.p[0] + nx, n.p[1] + nz], local))
                    .min(ground([n.p[0] - nx, n.p[1] - nz], local))
                    .min(n.bed);
                (g - n.freeboard, centre)
            }).collect()
        }).collect();
        let raw: Vec<Vec<f64>> = sampled.iter().map(|c| c.iter().map(|x| x.0).collect()).collect();
        let drawn: Vec<Vec<f64>> = sampled.iter().map(|c| c.iter().map(|x| x.1).collect()).collect();
        let lake: Vec<Vec<bool>> = raw.iter().map(|r| lakes(r)).collect();
        let lv = levels(&self.courses, &raw, &lake);
        let _ = self.carve.set(Carve::new(&self.courses, &lv, &drawn, &lake));
    }
}

/// The rivers' water surface: a mesh that lets light up through it, as transparent water
/// does (`Geometry::transmits`), so the beds beneath are lit through it.
pub struct WaterSurface {
    pub mesh: crate::shapes::mesh::SmoothMesh,
    /// The water's attenuation per metre, per channel, at a point of the surface.
    pub attenuation: Attenuation,
}

/// The water's attenuation at a point (`WaterSurface`).
pub type Attenuation = Box<dyn Fn(&Vector3<f64>) -> crate::color::Color + Send + Sync>;

impl crate::shapes::geometry::Geometry for WaterSurface {
    fn intersects(&self, r: &crate::ray::Ray) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects(r) }
    fn intersects_beyond(&self, r: &crate::ray::Ray, min: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_beyond(r, min) }
    fn intersects_range(&self, r: &crate::ray::Ray, min: f64, max: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_range(r, min, max) }
    fn occludes(&self, r: &crate::ray::Ray, min: f64, max: f64) -> bool { self.mesh.occludes(r, min, max) }
    fn fast_intersects(&self, r: &crate::ray::Ray) -> bool { self.mesh.fast_intersects(r) }
    fn bounds(&self) -> crate::shapes::bbox::BBox { self.mesh.bounds() }
    fn primitives(&self) -> u64 { self.mesh.primitives() }
    fn kind(&self) -> &'static str { "river water" }
    fn transmits(&self) -> bool { true }

    fn transmittance(&self, r: &crate::ray::Ray, min: f64, max: f64) -> f64 {
        crate::color::luminance(&self.transmittance_color(r, min, max))
    }

    /// Light rising through the surface from under the water passes as the sea's does
    /// (`ocean::Ocean::transmittance_color`): Fresnel's transmission, and the water's absorption
    /// along the refracted path from the point the ray left; light going down into it is
    /// blocked (what is under water is reached through the surface's own shading).
    fn transmittance_color(&self, r: &crate::ray::Ray, min: f64, max: f64) -> crate::color::Color {
        use crate::color::Color;
        let Some(hit) = self.mesh.intersects_range(r, min, max) else { return Color::white() };
        let rd = r.rd.normalize();
        let cos_air = rd.dot(&hit.normal);
        if cos_air <= 0. { return Color::black(); }
        let depth = (hit.dist - min) * cos_air;
        let cos_water = (1. - (1. - cos_air * cos_air) / (crate::ocean::WATER_INDEX * crate::ocean::WATER_INDEX)).max(1e-3).sqrt();
        let a = (self.attenuation)(&hit.point).as_unbounded();
        Color { rgb: a.rgb.map(|k| (-k * depth / cos_water).exp()) } * (1. - crate::ocean::fresnel(cos_air))
    }
}

/// The patch point of direction `d` on `domain`, if it is on the patch's side.
pub fn patch_point(domain: &Domain, d: &Vector3<f64>) -> Option<Point> {
    match &domain.layout {
        Layout::Patch { up, east, north, .. } if d.dot(up) > 0.1 => {
            let (x, z) = Domain::tangent(*up, *east, *north, domain.radius, d);
            Some([x, z])
        },
        _ => None,
    }
}

/// The direction of patch point `p` on `domain`.
pub fn patch_dir(domain: &Domain, p: Point) -> Vector3<f64> {
    match &domain.layout {
        Layout::Patch { up, east, north, .. } => (up * domain.radius + east * p[0] + north * p[1]).normalize(),
        Layout::Sphere { .. } => panic!("rivers are only modelled on patches"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(x: f64, z: f64) -> Node { Node { p: [x, z], half: 5., depth: 1., freeboard: 0.5, bed: 0., slope: 1e-3, meanders: false, grain: 0.01, turbidity: 0., froude: 0.3 } }

    #[test]
    fn test_bends_give_the_sinuosity_and_keep_the_ends() {
        // A straight 2 km reach 10 m wide on a gentle slope, sampled every metre.
        let n: Vec<Node> = (0..=2000).map(|x| node(x as f64, 0.)).collect();
        let b = bend(&n, 7);
        assert_eq!(b[0].p, [0., 0.]);
        assert!(dist(b[2000].p, [2000., 0.]) < 1e-6);
        let length: f64 = b.windows(2).map(|w| dist(w[0].p, w[1].p)).sum();
        // Away from the tapered ends the target is 1.5; overall somewhat less.
        assert!(length / 2000. > 1.15 && length / 2000. < 1.6, "{}", length / 2000.);
        // A steep reach hardly bends; a meandering channel isn't bent again.
        let steep: Vec<Node> = n.iter().map(|v| Node { slope: 0.05, ..*v }).collect();
        let l: f64 = bend(&steep, 7).windows(2).map(|w| dist(w[0].p, w[1].p)).sum();
        assert!(l / 2000. < 1.08, "{}", l / 2000.);
        let channel: Vec<Node> = n.iter().map(|v| Node { meanders: true, ..*v }).collect();
        assert!(bend(&channel, 7).iter().zip(&n).all(|(a, b)| a.p == b.p));
    }

    fn course(start: usize, end: usize, area: f64, points: &[(f64, f64)]) -> Course {
        Course { nodes: points.iter().map(|&(x, z)| node(x, z)).collect(), start, end, area, joins: vec![] }
    }

    #[test]
    fn test_chaikin_keeps_ends_and_smooths() {
        let n = vec![node(0., 0.), node(100., 0.), node(100., 100.)];
        let s = chaikin(&chaikin(&n));
        assert_eq!(s[0].p, [0., 0.]);
        assert_eq!(s.last().unwrap().p, [100., 100.]);
        // The corner is cut: no point reaches it.
        assert!(s.iter().all(|v| dist(v.p, [100., 0.]) > 10.));
        let d = densify(&s);
        assert!(d.windows(2).all(|w| dist(w[0].p, w[1].p) <= 5. + 1e-9));
    }

    #[test]
    fn test_levels_never_rise_downstream_and_take_in_tributaries() {
        // Two tributaries meet at cell 9 and flow on to 20.
        let a = course(1, 9, 1., &[(0., 0.), (10., 0.), (20., 0.)]);
        let b = course(2, 9, 2., &[(0., 10.), (10., 10.), (20., 0.)]);
        let c = course(9, 20, 5., &[(20., 0.), (30., 0.), (40., 0.)]);
        let raw = vec![vec![10., 12., 8.], vec![9., 7., 11.], vec![15., 6., 9.]];
        let none: Vec<Vec<bool>> = raw.iter().map(|r| vec![false; r.len()]).collect();
        let l = levels(&[a, b, c], &raw, &none);
        assert_eq!(l[0], vec![10., 10., 8.]);
        assert_eq!(l[1], vec![9., 7., 7.]);
        // The main stem starts no higher than the lower tributary arrives.
        assert_eq!(l[2], vec![7., 6., 6.]);
    }

    #[test]
    fn test_lakes_fill_depressions() {
        // A basin 20 m deep between two stretches of river; a 2 m dip is carved, not a lake.
        let raw = [110., 108., 90., 88., 95., 104., 103., 101., 103., 100.];
        let k = lakes(&raw);
        assert_eq!(k, vec![false, false, true, true, true, false, false, false, false, false]);
        // Out of the lake the river starts afresh at its own ground.
        let c = course(1, 2, 1., &(0..10).map(|x| (x as f64, 0.)).collect::<Vec<_>>());
        let l = levels(&[c], &[raw.to_vec()], &[k]);
        assert_eq!(&l[0][5..], &[104., 103., 101., 101., 100.]);
        assert_eq!(&l[0][..2], &[110., 108.]);
    }

    #[test]
    fn test_carved_channel() {
        // A straight river along x, level 100, 10 m wide, 1 m deep on average.
        let c = course(1, 2, 1., &[(0., 0.), (50., 0.), (100., 0.)]);
        let carve = Carve::new(&[c], &[vec![100., 100., 100.]], &[vec![104., 104., 104.]], &[vec![false; 3]]);
        let (centre, half) = carve.surface([50., 0.]).unwrap();
        assert_eq!(half, 5.);
        assert!((centre - 98.5).abs() < 1e-9, "{}", centre);
        // The bed meets the water's edge at the level, and the bank rises beyond it.
        assert!((carve.surface([50., 5.]).unwrap().0 - 100.).abs() < 1e-9);
        assert!((carve.surface([50., 8.]).unwrap().0 - 103.).abs() < 1e-9);
        assert!(carve.surface([50., 200.]).is_none());
        // Ground above the water is cut down to the bed, but only where the mesh shows it.
        assert!((carve.carved([50., 0.], 104., 1.) - 98.5).abs() < 1e-9);
        assert_eq!(carve.carved([50., 0.], 104., 20.), 104.);
        assert_eq!(carve.carved([50., 0.], 97., 1.), 97.);
        // The water: within the river, at its level.
        assert_eq!(carve.level([50., 3.], 0.).map(|l| l.0), Some(100.));
        assert_eq!(carve.level([50., 7.], 0.), None);
        assert_eq!(carve.level([50., 7.], 20.).map(|l| l.0), Some(100.));
        // Its surface: a strip of two triangles a segment, a little wider than the water.
        let (v, t) = carve.ribbon(|_, _| true);
        assert_eq!(v.len(), 6);
        assert_eq!(t.len(), 4);
        assert!(v.iter().all(|(p, l)| *l == 100. && p[1].abs() > 5.));
    }

    #[test]
    fn test_a_landscapes_courses() {
        use crate::terrain::hydraulics::HydraulicParams;
        use crate::terrain::simulation::TerrainParams;
        let mut p = TerrainParams::default();
        p.planet.seed = 5;
        p.step = 500.;
        let mut t = Terrain::new(Domain::patch(40., 10., 32, 500., 6.371e6), p);
        t.run_to(3000.);
        let h = Hydraulics::new(&t, None, &HydraulicParams::default());
        let cs = courses(&t, &h);
        assert!(!cs.is_empty());
        // Each course ends where the next downstream starts, or at an outlet; in order of area.
        let starts: BTreeSet<usize> = cs.iter().map(|c| c.start).collect();
        let outlets = t.outlets();
        for c in &cs {
            assert!(starts.contains(&c.end) || outlets[c.end] || !is_river(&t, t.receiver[c.end]) || c.end == t.receiver[c.end],
                "course ending at {} goes nowhere", c.end);
            assert!(c.nodes.iter().all(|n| n.half > 0. && n.depth >= 0. && n.freeboard >= 0.));
        }
        assert!(cs.windows(2).all(|w| w[0].area <= w[1].area));
        // Levelled over the simulation's own surface, no course rises downstream.
        let r = Rivers::new(&t, &h);
        let corners = crate::terrain::scenery::corner_heights(&t);
        r.level(|q, _| crate::terrain::scenery::sample(&t.domain, &corners, &patch_dir(&t.domain, q)));
        let carve = r.carve.get().unwrap();
        for c in &carve.courses {
            assert!(c.windows(2).all(|w| w[0].lake || w[1].lake || w[1].level <= w[0].level));
        }
    }
}

