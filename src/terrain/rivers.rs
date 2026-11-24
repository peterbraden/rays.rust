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
    /// The flow shown, m³/s.
    pub discharge: f64,
}

impl Node {
    /// Still, clear water, for where no river is found.
    pub fn still() -> Node {
        Node { p: [0., 0.], half: 1., depth: 1., freeboard: 0., bed: 0., slope: 1e-5, meanders: false, grain: 0.01, turbidity: 0., froude: 0., discharge: 0. }
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
        discharge: a.discharge + (b.discharge - a.discharge) * t,
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
                discharge: a.discharge + (b.discharge - a.discharge) * t,
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
            turbidity: crate::terrain::river_water::concentration(t.sediment_flux[i], h.mean[i]), froude: r.froude, discharge: r.discharge }
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

/// Confluences where a tributary arrives higher than this (metres) above the river it joins
/// fall to it; lower, they run down to it as rapids.
pub const FALL: f64 = 5.;

/// A waterfall: where a tributary leaves its channel above the river it joins.
#[derive(Clone, Debug)]
pub struct Fall {
    /// The course, and the index of its node at the lip.
    pub course: usize,
    pub lip: usize,
    /// The flow at the lip.
    pub node: Node,
    /// The direction it leaves in (patch coordinates, unit).
    pub along: Point,
    /// The water's levels at the lip and at the foot, metres.
    pub top: f64,
    pub foot: f64,
    /// Where it lands: the point of the river it joins.
    pub landing: Point,
    /// The jet's hydraulics (`hydraulics::Jet`).
    pub jet: crate::terrain::hydraulics::Jet,
}

/// How a fall's jet widens as it falls: its half width grows by this much a metre of fall (a
/// model choice of the order of the spread Ervine and Falvey measured in turbulent jets).
pub const SPREAD: f64 = 0.04;

impl Fall {
    /// The point `x` metres out from the lip along the jet and `y` across it (patch
    /// coordinates).
    pub fn point(&self, x: f64, y: f64) -> Point {
        let (a, n) = (self.along, [-self.along[1], self.along[0]]);
        [self.node.p[0] + a[0] * x + n[0] * y, self.node.p[1] + a[1] * x + n[1] * y]
    }

    /// The alcove the fall drops into, cut into the ground below its lip: past the lip, as far
    /// as the river it lands in, widening from the stream's width, its floor the pool at the
    /// foot. The surface there and its half width at `q`; None outside it.
    pub fn alcove(&self, q: Point) -> Option<(f64, f64)> {
        let d = [q[0] - self.node.p[0], q[1] - self.node.p[1]];
        let x = d[0] * self.along[0] + d[1] * self.along[1];
        let y = (d[1] * self.along[0] - d[0] * self.along[1]).abs();
        let reach = dist(self.node.p, self.landing) + 2.;
        let half = 1.5 * self.node.half + 2. + 0.3 * x.max(0.);
        if x <= 0. || x > reach || y > half { return None; }
        // A pool a little below the river it joins, under where the jet lands.
        Some((self.foot - 0.5, half))
    }

    /// The jet as a curtain of triangles: rows down its trajectory from the lip to the foot,
    /// widening as it falls; each vertex as (patch point, height, metres fallen).
    pub fn sheet(&self, rows: usize, columns: usize) -> Sheet {
        let height = self.top - self.foot;
        let end = self.jet.time_to(height);
        let mut vertices = vec![];
        for r in 0..=rows {
            // Finer near the lip, where the curve bends most.
            let t = end * (r as f64 / rows as f64).powf(1.5);
            let (x, z, _, _) = self.jet.at(t);
            let half = self.jet.width / 2. + SPREAD * z;
            for c in 0..=columns {
                let y = half * (2. * c as f64 / columns as f64 - 1.);
                vertices.push((self.point(x, y), self.top - z, z));
            }
        }
        let mut triangles = vec![];
        let w = columns + 1;
        for r in 0..rows {
            for c in 0..columns {
                let (a, b) = (r * w + c, (r + 1) * w + c);
                triangles.push([a, b, a + 1]);
                triangles.push([a + 1, b, b + 1]);
            }
        }
        (vertices, triangles)
    }
}

/// Slopes steeper than this are bedrock steps: Montgomery and Buffington's steepest alluvial
/// channels, cascades, reach about 0.3.
pub const STEP: f64 = 0.3;
/// Falls with less water than this (m³/s) aren't drawn.
pub const TRICKLE: f64 = 0.1;

/// The falls on course `k` (`c`, its `levels` and `kind`): each run of the river steeper than
/// `STEP` drops what a cascade can't hold at its head, a fall of height Δ - `STEP` L for a run
/// that drops Δ over L, and runs on below as a cascade at `STEP` to the run's foot, in a gorge
/// that tapers out (the gorge a waterfall leaves as it retreats). Its levels are lowered to match. At the
/// simulation's resolution a hanging valley's step is such a run over the tributary's last
/// cell.
fn knickpoints(k: usize, c: &Course, levels: &mut [f64], kind: &[Kind]) -> Vec<Fall> {
    let n = c.nodes.len();
    let mut along = vec![0.; n];
    for i in 1..n { along[i] = along[i - 1] + dist(c.nodes[i - 1].p, c.nodes[i].p); }
    let steeper: Vec<bool> = (0..n.saturating_sub(1)).map(|i| kind[i] == Kind::River && kind[i + 1] == Kind::River
        && (levels[i] - levels[i + 1]) >= STEP * (along[i + 1] - along[i]).max(1e-9)).collect();
    let steep = |i: usize| steeper[i];
    let mut out = vec![];
    let mut i = 0;
    while i + 1 < n {
        if !steep(i) { i += 1; continue; }
        let start = i;
        while i + 1 < n && steep(i) { i += 1; }
        let (top, bottom) = (levels[start], levels[i]);
        let length = along[i] - along[start];
        let height = (top - bottom) - STEP * length;
        let node = c.nodes[start];
        if height >= FALL && start > 0 && node.discharge >= TRICKLE {
            for j in start + 1..=i {
                levels[j] = levels[j].min(top - height - STEP * (along[j] - along[start]));
            }
            let prev = c.nodes[start - 1].p;
            let (dx, dz) = (node.p[0] - prev[0], node.p[1] - prev[1]);
            let len = dx.hypot(dz).max(1e-9);
            let jet = crate::terrain::hydraulics::Jet::new(node.discharge, 2. * node.half, height);
            let out_to = jet.speed * jet.time_to(height) + 3. + node.half;
            let landing = [node.p[0] + dx / len * out_to, node.p[1] + dz / len * out_to];
            out.push(Fall { course: k, lip: start, node, along: [dx / len, dz / len], top, foot: top - height, landing, jet });
        }
    }
    out
}

/// A fall's curtain: vertices (patch point, height, metres fallen) and triangles.
pub type Sheet = (Vec<(Point, f64, f64)>, Vec<[usize; 3]>);

/// The waterfalls and rapids where courses join: each tributary that arrives above the level
/// of the river it joins (`levels` takes the lower of the two from there on) either runs down
/// to it over its last stretch (rapids, a drop under `FALL`) or leaves its channel at a lip on
/// the main river's bank and falls (a hanging valley's waterfall). `levels` are ramped for the
/// rapids; the stretch beyond a lip is marked `Falling`.
pub fn falls(courses: &[Course], levels: &mut [Vec<f64>], kind: &mut [Vec<Kind>]) -> Vec<Fall> {
    let start_of: HashMap<usize, usize> = courses.iter().enumerate().map(|(k, c)| (c.start, k)).collect();
    let mut joined: HashMap<usize, (usize, usize)> = HashMap::new();
    for (m, c) in courses.iter().enumerate() {
        for &(cell, j) in &c.joins { joined.insert(cell, (m, j)); }
    }
    let mut out = vec![];
    // Knickpoints: runs steeper than any alluvial channel keeps.
    for (k, c) in courses.iter().enumerate() {
        out.extend(knickpoints(k, c, &mut levels[k], &kind[k]));
    }
    for (k, c) in courses.iter().enumerate() {
        let n = c.nodes.len();
        if n < 2 || kind[k][n - 1] != Kind::River { continue; }
        let Some((d, j)) = start_of.get(&c.end).map(|&d| (d, 0)).or_else(|| joined.get(&c.end).copied()) else { continue };
        if d == k || kind[d][j] != Kind::River { continue; }
        let (top, foot) = (levels[k][n - 1], levels[d][j]);
        let drop = top - foot;
        if drop <= 0.05 { continue; }
        let landing = courses[d].nodes[j].p;
        // How far back along the tributary each node is from its end.
        let mut back = vec![0.; n];
        for i in (0..n - 1).rev() { back[i] = back[i + 1] + dist(c.nodes[i].p, c.nodes[i + 1].p); }
        if drop < FALL {
            // Rapids: down to the main river's level over a stretch some times the drop.
            let reach = (10. * drop).max(4. * c.nodes[n - 1].half).max(20.);
            for i in 0..n {
                if back[i] < reach { levels[k][i] = levels[k][i].min(foot + drop * back[i] / reach); }
            }
            continue;
        }
        // A fall: the lip stands on the main river's bank, a little back from its water.
        let setback = courses[d].nodes[j].half + (0.05 * drop).clamp(2., 30.);
        let Some(lip) = (0..n).rev().find(|&i| back[i] >= setback) else { continue };
        // The lip and the stretch beyond it are river, not lake (whose levels mean nothing).
        if kind[k][lip..].iter().any(|&x| x != Kind::River) || levels[k][lip] - foot < FALL { continue; }
        for x in kind[k].iter_mut().skip(lip + 1) { *x = Kind::Falling; }
        let node = c.nodes[lip];
        let prev = c.nodes[lip.saturating_sub(1)].p;
        let (dx, dz) = (node.p[0] - prev[0], node.p[1] - prev[1]);
        let len = dx.hypot(dz).max(1e-9);
        let top = levels[k][lip];
        out.push(Fall {
            course: k, lip, node, along: [dx / len, dz / len], top, foot, landing,
            jet: crate::terrain::hydraulics::Jet::new(node.discharge, 2. * node.half, top - foot),
        });
    }
    out
}

/// A river vertex with its level, and how far below the drawn ground that is; or in a lake.
#[derive(Clone, Copy, Debug)]
struct Vertex { node: Node, level: f64, cut: f64, kind: Kind }

/// What a stretch of a course is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    River,
    /// In a basin the water would fill (`lakes`): drawn by the mask, not carved.
    Lake,
    /// Beyond a waterfall's lip (`falls`), where the water is in the air.
    Falling,
}

/// The level at which water arrives at each junction cell, metres: the lowest of the courses
/// ending there.
type Arrivals = HashMap<usize, f64>;

/// Depressions deeper than this (metres) along a river are lakes.
pub const LAKE: f64 = 5.;

/// Which nodes of a course are in lakes: where filling it from its end to the spill levels
/// (each node no lower than the one after it) would stand more than `LAKE` over `raw`.
pub fn lakes(raw: &[f64]) -> Vec<bool> {
    fills(raw).iter().zip(raw).map(|(f, r)| f - r > LAKE).collect()
}

/// The level water would fill a course to from its end: each node no lower than the one after
/// it. In a lake, its spill level.
pub fn fills(raw: &[f64]) -> Vec<f64> {
    let mut fill = raw.to_vec();
    for j in (0..fill.len().saturating_sub(1)).rev() { fill[j] = fill[j].max(fill[j + 1]); }
    fill
}

/// How far a lake's surface reaches either side of the river through it, metres, at most: it
/// is drawn flat at its spill level over that, and the ground higher than that hides it, so
/// it fills its basin to the shore.
pub const SHORE: f64 = 250.;

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
/// in metres, to tuck under the banks: enough that its straight edges between nodes stay under
/// the banks round a bend (the banks rise at 1:1 beyond the edge, so it is hidden there).
const TUCK: (f64, f64) = (0.15, 2.);
/// The banks are sampled this far out (fraction of the half width, and metres).
const BANK_SAMPLE: (f64, f64) = (0.15, 1.);

/// A segment of a levelled river.
#[derive(Clone, Copy, Debug)]
struct Segment { a: Vertex, b: Vertex, course: u32 }

impl Segment {
    fn river(&self) -> bool { self.a.kind == Kind::River && self.b.kind == Kind::River }

    /// The river's half width, level and greatest depth at fraction `t` along.
    fn at(&self, t: f64) -> (f64, f64, f64) {
        let l = |x: f64, y: f64| x + (y - x) * t;
        (l(self.a.node.half, self.b.node.half), l(self.a.level, self.b.level), 1.5 * l(self.a.node.depth, self.b.node.depth))
    }

    /// How far beyond its centreline the carving can reach (or a lake's surface).
    fn reach(&self) -> f64 {
        if self.a.kind == Kind::Lake || self.b.kind == Kind::Lake { return self.a.node.half.max(self.b.node.half) + SHORE; }
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
    pub falls: Vec<Fall>,
}

impl Carve {
    /// `levels`, the drawn `ground` and what each stretch is (`kind`), at each course's nodes;
    /// and the course's `falls`.
    pub fn new(courses: &[Course], levels: &[Vec<f64>], ground: &[Vec<f64>], kind: &[Vec<Kind>], falls: Vec<Fall>) -> Carve {
        let courses: Vec<Vec<Vertex>> = courses.iter().zip(levels).zip(ground).zip(kind)
            .map(|(((c, l), g), k)| c.nodes.iter().zip(l).zip(g).zip(k)
                .map(|(((&node, &level), &g), &kind)| Vertex { node, level, cut: if kind == Kind::River { (g - level).max(0.) } else { 0. }, kind }).collect()).collect();
        // Lakes are drawn as before (the mask; step 8 of the plan), so neither carved nor
        // given a surface; nor is water in the air.
        let segments: Vec<Segment> = courses.iter()
            .enumerate().flat_map(|(k, c)| c.windows(2).filter(|w| w[0].kind != Kind::Falling && w[1].kind != Kind::Falling).map(move |w| Segment { a: w[0], b: w[1], course: k as u32 })).collect();
        let mut reaches: Vec<f64> = segments.iter().filter(|s| s.river()).map(|s| s.reach()).collect();
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
        Carve { courses, segments, grid, cell, falls }
    }

    /// The river segments that may reach `q` (not lakes').
    fn near(&self, q: Point) -> impl Iterator<Item = &Segment> {
        self.near_any(q).filter(|s| s.river())
    }

    fn near_any(&self, q: Point) -> impl Iterator<Item = &Segment> {
        let key = ((q[0] / self.cell).floor() as i64, (q[1] / self.cell).floor() as i64);
        self.grid.get(&key).into_iter().flatten().map(|&k| &self.segments[k as usize])
    }

    /// The carved surface at `q`: the bed in the channel, the banks beside it; None away from
    /// rivers. With it, the half width of the river nearest.
    pub fn surface(&self, q: Point) -> Option<(f64, f64)> { self.surface_deeper(q, 0., 0.) }

    /// As `surface`, the channel at least `least` metres deep at its centre and `widen` metres
    /// wider each side, its bank's toe that far out.
    fn surface_deeper(&self, q: Point, least: f64, widen: f64) -> Option<(f64, f64)> {
        // Each course's channel from its segment nearest `q` (the lowest of every segment's
        // would scallop the inside of each bend, one scallop a segment, which showed as a row
        // of bank above the water); the lowest of the courses' (where they meet).
        let mut nearest: BTreeMap<u32, (f64, f64, f64)> = BTreeMap::new();
        for s in self.near(q) {
            let (t, r) = nearest_on(s.a.node.p, s.b.node.p, q);
            if r > s.reach() || nearest.get(&s.course).is_some_and(|n| n.0 <= r) { continue; }
            let (half, level, deepest) = s.at(t);
            let deepest = deepest.max(least);
            let wide = half + widen.min(TUCK.0 * half + 0.5 * TUCK.1);
            let h = if r < wide { level - deepest * (1. - (r / wide).powi(2)) } else { level + (r - wide) * BANK };
            nearest.insert(s.course, (r, h, half));
        }
        let mut best: Option<(f64, f64)> = None;
        for &(_, h, half) in nearest.values() {
            if best.is_none_or(|(b, _)| h < b) { best = Some((h, half)); }
        }
        for f in &self.falls {
            if let Some((h, half)) = f.alcove(q) && best.is_none_or(|(b, _)| h < b) { best = Some((h, half)); }
        }
        best
    }

    /// How much of the carving a mesh of spacing `local` draws, for a river of half width
    /// `half`: all of it under half the half width, none from the whole.
    pub fn weight(half: f64, local: f64) -> f64 { 1. - smoothstep(0.5 * half, half, local) }

    /// Ground `g` at `q`, carved as a mesh of spacing `local` draws it: the channel at least
    /// half the spacing (and 0.3 m) deep, and its bank's toe a spacing out from the water's
    /// edge (as far as the water's surface tucks under the bank), so the mesh's straight edges
    /// between its vertices, spanning bed and bank, don't rise through a shallow river's water
    /// (they showed as a row of bank above it, one a vertex; through silty water the bed isn't
    /// seen anyway).
    pub fn carved(&self, q: Point, g: f64, local: f64) -> f64 {
        match self.surface_deeper(q, 0.5 * local + 0.3, local) {
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
                discharge: l(a.discharge, b.discharge),
            };
            let (dx, dz) = (b.p[0] - a.p[0], b.p[1] - a.p[1]);
            let len = dx.hypot(dz).max(1e-12);
            best = Some((r, node, [dx / len, dz / len]));
        }
        best.map(|(_, n, d)| (n, d))
    }

    /// The river node nearest `q`, not in a lake: its point, level and node.
    pub fn nearest(&self, q: Point) -> Option<(Point, f64, Node)> {
        self.courses.iter().flatten().filter(|v| v.kind == Kind::River)
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
                // (Lakes are drawn by their cells, `Rivers::lake_cells`.)
                let half = v.node.half;
                if v.kind != Kind::River || !keep(v.node.p, half) { previous = None; continue; }
                let (a, b) = (c[j.saturating_sub(1)].node.p, c[(j + 1).min(c.len() - 1)].node.p);
                let (dx, dz) = (b[0] - a[0], b[1] - a[1]);
                let len = dx.hypot(dz).max(1e-9);
                let w = half * (1. + TUCK.0) + TUCK.1;
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
    /// Where water stands on the ground as drawn (`terrain::basins`), once levelled.
    pub basins: std::sync::OnceLock<crate::terrain::basins::Basins>,
    /// The patch's half side and the basins' grid spacing, metres.
    extent: f64,
    step: f64,
    /// Where the simulation's water flows: from every cell to the one it drains to, over the
    /// simulation's own filled surface, as points and heights every few metres. Cut into the
    /// drawn ground's basins (`basins::Basins::build`), so the drawn ground's smoothing, which
    /// closes small valleys off, doesn't make lakes the simulation drains.
    paths: Vec<(Point, f64)>,
}

pub use crate::terrain::basins::LAKE_DEPTH;

/// Every cell's flow path to the cell it drains to, by the simulation's routing over its filled
/// surface (`flow::route`), as points `spacing` apart with heights interpolated along it.
fn flow_paths(t: &Terrain, spacing: f64) -> Vec<(Point, f64)> {
    let d = &t.domain;
    let n = d.len();
    let f = crate::terrain::flow::route(d, &t.bed, &t.outlets(), &vec![1.; n]);
    let mut out = vec![];
    for i in 0..n {
        let r = f.receiver[i];
        if r == i { continue; }
        let (a, b) = (cell_point(d, i), cell_point(d, r));
        let steps = (dist(a, b) / spacing).ceil().max(1.) as usize;
        for s in 0..=steps {
            let u = s as f64 / steps as f64;
            out.push(([a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u], f.filled[i] + (f.filled[r] - f.filled[i]) * u));
        }
    }
    out
}

impl Rivers {
    pub fn new(t: &Terrain, h: &Hydraulics) -> Rivers {
        let courses = courses(t, h);
        let (extent, step) = match t.domain.layout {
            Layout::Patch { n, spacing, .. } => (0.5 * n as f64 * spacing, spacing / 8.),
            Layout::Sphere { .. } => (0., 1.),
        };
        let paths = if extent > 0. { flow_paths(t, step / 2.) } else { vec![] };
        Rivers { skeleton: Skeleton::new(&courses), courses, carve: std::sync::OnceLock::new(), basins: std::sync::OnceLock::new(), extent, step, paths }
    }

    /// Level the courses over `ground` (patch point, spacing → height, metres, as drawn),
    /// unless done: each river's level from the ground at its centre and banks, less its
    /// freeboard, never rising downstream (its channel cut through the ground's bumps), but
    /// for deep basins along it; then where water stands on the ground (`basins`), the
    /// channels breaching it; and through the lakes that leaves, the lakes' levels.
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
                // The ground on the river's line and banks, and the simulation's own bed there:
                // routed by the simulation's fill, its only basins are real ones, where the
                // drawn ground's bumps across a valley (synthetic relief, smoothing) aren't.
                let g = centre.min(ground([n.p[0] + nx, n.p[1] + nz], local)).min(ground([n.p[0] - nx, n.p[1] - nz], local)).min(n.bed);
                (g - n.freeboard, centre)
            }).collect()
        }).collect();
        let raw: Vec<Vec<f64>> = sampled.iter().map(|c| c.iter().map(|x| x.0).collect()).collect();
        let drawn: Vec<Vec<f64>> = sampled.iter().map(|c| c.iter().map(|x| x.1).collect()).collect();
        // The rivers cut their channels (their levels never rising downstream), but for
        // stretches where filling would stand deep: those are lakes.
        let deep: Vec<Vec<bool>> = raw.iter().map(|r| lakes(r)).collect();
        let cut = levels(&self.courses, &raw, &deep);
        // Where water stands on the ground as drawn, the rivers' channels breaching it.
        let channels: Vec<(Point, f64)> = self.courses.iter().zip(&cut).zip(&deep)
            .flat_map(|((c, l), k)| c.nodes.iter().zip(l).zip(k).filter(|x| !*x.1).map(|((n, &l), _)| (n.p, l - 1.5 * n.depth)))
            .chain(self.paths.iter().copied())
            .collect();
        let basins = self.basins.get_or_init(|| crate::terrain::basins::Basins::build(self.extent, self.step, 8. * self.step, |q| ground(q, self.step), &channels));
        // The rivers' levels again, through the lakes the basins hold at the lakes' levels.
        let lake_levels: Vec<Vec<Option<f64>>> = self.courses.iter().map(|c| c.nodes.iter().map(|n| basins.lake(n.p)).collect()).collect();
        let lake: Vec<Vec<bool>> = lake_levels.iter().map(|l| l.iter().map(|x| x.is_some()).collect()).collect();
        let mut lv = levels(&self.courses, &raw, &lake);
        for (l, k) in lv.iter_mut().zip(&lake_levels) {
            for (x, f) in l.iter_mut().zip(k) { if let Some(f) = f { *x = *f; } }
        }
        let mut kind: Vec<Vec<Kind>> = lake.iter().map(|l| l.iter().map(|&k| if k { Kind::Lake } else { Kind::River }).collect()).collect();
        let found = falls(&self.courses, &mut lv, &mut kind);
        let _ = self.carve.set(Carve::new(&self.courses, &lv, &drawn, &kind, found));
    }
}

/// The rivers' water surface: a mesh that lets light up through it, as transparent water
/// does (`Geometry::transmits`), so the beds beneath are lit through it.
pub struct WaterSurface {
    pub mesh: crate::shapes::mesh::SmoothMesh,
    /// The planet's centre: the surface's normals face away from it, whichever way its
    /// triangles wind (the water material reads a normal facing down as light leaving the
    /// water from below, which made lakes seen from their shores dark).
    pub centre: Vector3<f64>,
    /// The water's attenuation per metre, per channel, at a point of the surface.
    pub attenuation: Attenuation,
}

/// The water's attenuation at a point (`WaterSurface`).
pub type Attenuation = Box<dyn Fn(&Vector3<f64>) -> crate::color::Color + Send + Sync>;

impl WaterSurface {
    fn up(&self, hit: Option<crate::intersection::RawIntersection>) -> Option<crate::intersection::RawIntersection> {
        hit.map(|mut h| {
            if h.normal.dot(&(h.point - self.centre)) < 0. { h.normal = -h.normal; }
            h
        })
    }
}

impl crate::shapes::geometry::Geometry for WaterSurface {
    fn intersects(&self, r: &crate::ray::Ray) -> Option<crate::intersection::RawIntersection> { self.up(self.mesh.intersects(r)) }
    fn intersects_beyond(&self, r: &crate::ray::Ray, min: f64) -> Option<crate::intersection::RawIntersection> { self.up(self.mesh.intersects_beyond(r, min)) }
    fn intersects_range(&self, r: &crate::ray::Ray, min: f64, max: f64) -> Option<crate::intersection::RawIntersection> { self.up(self.mesh.intersects_range(r, min, max)) }
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
        let Some(hit) = self.up(self.mesh.intersects_range(r, min, max)) else { return Color::white() };
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

    fn node(x: f64, z: f64) -> Node { Node { p: [x, z], half: 5., depth: 1., freeboard: 0.5, bed: 0., slope: 1e-3, meanders: false, grain: 0.01, turbidity: 0., froude: 0.3, discharge: 1. } }

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
    fn test_hanging_valleys_fall_and_small_steps_run_down() {
        // A tributary 100 m long joins, at cell 9, a river 5 m wide whose level is 30 m below
        // its own; another arrives at cell 9 only 2 m above it.
        let pts: Vec<(f64, f64)> = (0..=100).map(|x| (x as f64, 0.)).collect();
        let high = course(1, 9, 1., &pts);
        let low = course(2, 9, 1., &pts.iter().map(|&(x, z)| (x, z + 50.)).collect::<Vec<_>>());
        let main = course(9, 20, 5., &[(100., 0.), (200., 0.)]);
        let mut levels = vec![vec![130.; 101], vec![102.; 101], vec![100., 99.]];
        let mut kind = vec![vec![Kind::River; 101], vec![Kind::River; 101], vec![Kind::River; 2]];
        let f = falls(&[high, low, main], &mut levels, &mut kind);
        assert_eq!(f.len(), 1);
        let fall = &f[0];
        assert_eq!(fall.course, 0);
        assert!((fall.top - fall.foot - 30.).abs() < 1e-9);
        // The lip is on the bank, beyond the main river's half width; past it the water is in
        // the air.
        assert!(100. - fall.node.p[0] >= 5. + 2. && 100. - fall.node.p[0] < 12.);
        assert!(kind[0][fall.lip + 1..].iter().all(|&k| k == Kind::Falling) && kind[0][fall.lip] == Kind::River);
        assert!(fall.jet.speed > 0. && fall.jet.height == 30.);
        // Its curtain runs from the lip down to the foot, widening; the ground is cut away
        // past the lip, not before it.
        let (v, t) = fall.sheet(10, 2);
        assert_eq!(t.len(), 10 * 2 * 2);
        assert!((v[0].1 - fall.top).abs() < 1e-9 && (v[v.len() - 1].1 - fall.foot).abs() < 1e-9);
        let width = |r: usize| dist(v[r * 3].0, v[r * 3 + 2].0);
        assert!(width(10) > width(0) && (width(0) - fall.jet.width).abs() < 1e-9);
        assert_eq!(fall.alcove(fall.point(0.5, 0.)).map(|a| a.0), Some(fall.foot - 0.5));
        assert!(fall.alcove(fall.point(-0.5, 0.)).is_none());
        // The rapids: down to the main river's level at the junction, never rising.
        assert!((levels[1][100] - 100.).abs() < 1e-9);
        assert!(levels[1].windows(2).all(|w| w[1] <= w[0]));
        assert!(kind[1].iter().all(|&k| k == Kind::River));
    }

    #[test]
    fn test_bedrock_steps_fall() {
        // A river 1 km long, gentle, then dropping 60 m over 100 m, then gentle again.
        let pts: Vec<(f64, f64)> = (0..=1000).map(|x| (x as f64, 0.)).collect();
        let c = course(1, 2, 1., &pts);
        let mut levels: Vec<f64> = (0..=1000).map(|x| {
            let x = x as f64;
            if x < 500. { 200. - 0.01 * x } else if x < 600. { 195. - 0.6 * (x - 500.) } else { 135. - 0.01 * (x - 600.) }
        }).collect();
        let kind = vec![Kind::River; 1001];
        let f = knickpoints(0, &c, &mut levels, &kind);
        assert_eq!(f.len(), 1);
        let fall = &f[0];
        assert_eq!(fall.lip, 500);
        // It drops what a cascade can't hold over the run: 60 - 0.3 * 100 m.
        assert!((fall.top - fall.foot - (60. - STEP * 100.)).abs() < 1e-6, "{}", fall.top - fall.foot);
        // Below, the river runs on as a cascade to the run's foot, never rising.
        assert!((levels[501] - (fall.foot - STEP)).abs() < 1e-6);
        assert!((levels[600] - 135.).abs() < 1e-6);
        assert!(levels.windows(2).all(|w| w[1] <= w[0] + 1e-12));
        // A gentle river has none.
        let mut gentle: Vec<f64> = (0..=1000).map(|x| 100. - 0.1 * x as f64).collect();
        assert!(knickpoints(0, &c, &mut gentle, &kind).is_empty());
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
        let carve = Carve::new(&[c], &[vec![100., 100., 100.]], &[vec![104., 104., 104.]], &[vec![Kind::River; 3]], vec![]);
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
            assert!(c.windows(2).all(|w| w[0].kind != Kind::River || w[1].kind != Kind::River || w[1].level <= w[0].level));
        }
    }
}

