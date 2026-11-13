//! # Meandering rivers and ox-bow lakes
//!
//! On a floodplain a river doesn't stay put. Flow round a bend presses against the outer bank
//! and erodes it while the inner bank gains a point bar, so bends grow, migrate downstream, and
//! eventually loop back on themselves until the neck between two loops is breached. The
//! abandoned loop is an ox-bow lake, which then slowly fills with sediment from floods.
//!
//! Rivers are much narrower than the grid's cells (a few hundred metres at most, against
//! 500 m cells), so meanders are a sub-grid model: each meandering river is a centreline, a
//! polyline in the patch's tangent plane, that evolves on its own and is drawn over the terrain
//! (`object`). It does not change the grid's elevations: the floodplain it wanders over is
//! built by deposition (`fluvial`), and the valley walls confine it.
//!
//! ## Where rivers meander
//!
//! Meandering needs a floodplain: here, a river with a drainage area of at least `min_area`
//! (100 km²) on a slope below `max_slope` (0.005), optionally over at least `min_sediment` of
//! sediment (0 by default: over geological time such lowland rivers are alluvial, even where
//! the grid's coarse deposition hasn't recorded it). Each maximal
//! run of such cells along the river network is a *reach*, which gets a channel: initially the
//! grid's path through the reach, resampled at one channel width and nudged sideways by a little
//! seeded noise, from which bends grow. Reaches are found again every `regrid` years; a channel
//! survives while a reach still ends where it does.
//!
//! Width follows the hydraulic geometry of rivers: downstream, width grows as discharge^0.5
//! (Leopold and Maddock, "The hydraulic geometry of stream channels and some physiographic
//! implications", USGS Professional Paper 252, 1953, doi:10.3133/pp252). Here
//! w = `width_coefficient` · A^½ with A in m²: 0.003 gives 30 m at 100 km² and 300 m at
//! 10,000 km², near the top of regional fits (roughly 1.5-3 m · (A/km²)^0.4-0.5; to verify).
//! Depth is w / 20.
//!
//! ## Migration
//!
//! Ikeda, Parker and Sawai ("Bend theory of river meanders. Part 1. Linear development", J.
//! Fluid Mechanics 112, 1981, doi:10.1017/S0022112081000451) showed that the excess near-bank velocity that erodes
//! the outer bank depends not just on the local curvature but on the curvature upstream, as the
//! flow takes a distance to adjust. Howard and Knutson ("Sufficient conditions for river
//! meandering: a simulation approach", Water Resources Research 20(11), 1984,
//! doi:10.1029/WR020i011p01659) turned
//! this into a simulation of the centreline: each point migrates towards its outer bank at
//!
//! ```text
//! R1(s) = Ω R0(s) + Γ ∫₀^∞ R0(s - ξ) G(ξ) dξ / ∫₀^∞ G(ξ) dξ,   G(ξ) = exp(-2 C_f ξ / D)
//! R0(s) = E w / r(s)
//! ```
//!
//! where r is the local radius of curvature, w the width, D the depth, C_f a friction factor,
//! E a bank erodibility (metres per year at a bend whose radius equals its width), and Ω = -1,
//! Γ = 2.5. Howard and Knutson's R0 is a nonlinear function of r/w peaking at r/w of 2-3, from
//! field data; this is the linear form used by later simulations (eg. Sylvester, Durkin and
//! Covault, Geology 47, 2019, and their meanderpy, which has C_f = 0.011). With E = 0.02 a bend
//! of radius 2.5 widths migrates about 1% of its width a year, the usual rule of thumb
//! (Constantine et al., Nature Geoscience 7, 2014, measure Amazon reaches in widths per year).
//! Camporeale et al. ("Hierarchy of models for meandering rivers and related morphodynamic
//! processes", Reviews of Geophysics 45, 2007, doi:10.1029/2005RG000185) review the models.
//! The weighted integral over upstream curvature (here an exponential filter along the
//! channel) makes bends grow and migrate downstream. The channel's two ends are held where the
//! reach begins and ends; points that would move onto ground higher than the floodplain (valley
//! walls) stay where they are, so the meander belt stays within the valley floor: the cells
//! reached from the reach without climbing more than two bank heights above the river.
//!
//! ## Cutoffs and ox-bow lakes
//!
//! When two points of the channel far apart along it come within a channel width of each other,
//! the neck is breached: the loop between them is cut off and becomes an ox-bow lake, `depth`
//! deep, filling with flood sediment at `oxbow_fill` metres per year until it is gone: centuries
//! to a few thousand years here. Constantine and Dunne ("Meander cutoff and the controls on the
//! production of oxbow lakes", Geology 36(1), 2008, doi:10.1130/G24130A.1) relate cutoff rates
//! to sinuosity and meander growth; Constantine et al. (Sedimentology 57(2), 2010) find
//! infilling rates set by the angle at which the old channel leaves the new one, some lakes
//! staying open for centuries.
//!
//! Not modelled: chute cutoffs, avulsion, braiding, bank strength varying with vegetation or
//! the deposits a channel crosses, and lateral erosion of the valley walls (Langston and Tucker,
//! Earth Surface Dynamics 6, 2018, doi:10.5194/esurf-6-1-2018, add bedrock valley widening to
//! landscape models).

use crate::geometry::Pcg32;
use crate::terrain::domain::{Domain, Layout};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ChannelParams {
    /// Smallest drainage area of a meandering river, square metres.
    pub min_area: f64,
    /// Steepest slope a river meanders on.
    pub max_slope: f64,
    /// Thinnest sediment a river meanders over, metres.
    pub min_sediment: f64,
    /// Channel width is this times the square root of drainage area.
    pub width_coefficient: f64,
    /// Bank erodibility E: metres per year of migration at a bend of radius one width, per
    /// metre of width.
    pub migration: f64,
    /// Friction factor C_f.
    pub friction: f64,
    /// Ω and Γ, the weights of local and upstream curvature.
    pub omega: f64,
    pub gamma: f64,
    /// How often reaches are found again, years.
    pub regrid: f64,
    /// Rate ox-bow lakes fill with sediment, metres per year.
    pub oxbow_fill: f64,
}

impl Default for ChannelParams {
    fn default() -> ChannelParams {
        ChannelParams {
            min_area: 1e8,
            max_slope: 0.005,
            min_sediment: 0.,
            width_coefficient: 0.003,
            migration: 0.02,
            friction: 0.011,
            omega: -1.,
            gamma: 2.5,
            regrid: 1000.,
            oxbow_fill: 0.005,
        }
    }
}

/// A point in the patch's tangent plane: metres east and north of its centre.
pub type Point = [f64; 2];

#[derive(Clone, Debug)]
pub struct Channel {
    pub points: Vec<Point>,
    pub width: f64,
    /// The grid cells of its reach, upstream to downstream.
    pub path: Vec<usize>,
    /// The reach's first and last cells.
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct Oxbow {
    pub points: Vec<Point>,
    pub width: f64,
    /// Remaining depth of water, metres.
    pub depth: f64,
}

/// The meandering rivers of a patch and their ox-bow lakes.
#[derive(Default)]
pub struct Rivers {
    pub channels: Vec<Channel>,
    pub oxbows: Vec<Oxbow>,
    /// When reaches were last found.
    found: Option<f64>,
}

fn dist(a: Point, b: Point) -> f64 { ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() }

/// Position of cell `i` in the tangent plane (patches only).
pub fn cell_point(domain: &Domain, i: usize) -> Point {
    match &domain.layout {
        Layout::Patch { up, east, north, .. } => {
            let (x, z) = Domain::tangent(*up, *east, *north, domain.radius, &domain.dirs[i]);
            [x, z]
        }
        Layout::Sphere { .. } => panic!("channels are only modelled on patches"),
    }
}

/// The cell under a tangent-plane point (patches only).
pub fn point_cell(domain: &Domain, p: Point) -> usize {
    match &domain.layout {
        Layout::Patch { up, east, north, .. } => domain.cell_at(&(up * domain.radius + east * p[0] + north * p[1])),
        Layout::Sphere { .. } => panic!("channels are only modelled on patches"),
    }
}

/// Resample a polyline at `spacing`, keeping its ends.
fn resample(points: &[Point], spacing: f64) -> Vec<Point> {
    let length: f64 = points.windows(2).map(|w| dist(w[0], w[1])).sum();
    let count = ((length / spacing).round() as usize).max(1);
    let step = length / count as f64;
    let mut out = vec![points[0]];
    let (mut seg, mut along) = (0, 0.);
    for k in 1..count {
        let target = k as f64 * step;
        while seg + 1 < points.len() - 1 && along + dist(points[seg], points[seg + 1]) < target {
            along += dist(points[seg], points[seg + 1]);
            seg += 1;
        }
        let (a, b) = (points[seg], points[seg + 1]);
        let t = ((target - along) / dist(a, b).max(1e-12)).clamp(0., 1.);
        out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
    }
    out.push(*points.last().unwrap());
    out
}

/// Migrate a channel for `dt` years. `movable(p)` says whether the channel may move onto `p`.
fn migrate(c: &mut Channel, p: &ChannelParams, dt: f64, movable: &dyn Fn(Point) -> bool) {
    let w = c.width;
    let depth = w / 20.;
    let decay = 2. * p.friction / depth;
    let mut t = 0.;
    while t < dt {
        let n = c.points.len();
        if n < 5 { return; }
        let pts = &c.points;
        // Signed curvature (positive turning left) and the left normal at each point.
        let mut curvature = vec![0.; n];
        let mut normal = vec![[0., 0.]; n];
        for k in 1..n - 1 {
            let (a, b, d) = (pts[k - 1], pts[k], pts[k + 1]);
            let (t1, t2) = ([b[0] - a[0], b[1] - a[1]], [d[0] - b[0], d[1] - b[1]]);
            let turn = (t1[0] * t2[1] - t1[1] * t2[0]).atan2(t1[0] * t2[0] + t1[1] * t2[1]);
            let ds = 0.5 * (dist(a, b) + dist(b, d));
            curvature[k] = turn / ds;
            let tangent = [d[0] - a[0], d[1] - a[1]];
            let l = (tangent[0] * tangent[0] + tangent[1] * tangent[1]).sqrt().max(1e-12);
            normal[k] = [-tangent[1] / l, tangent[0] / l];
        }
        // R0 and its upstream-weighted average, by an exponential filter down the channel.
        let r0: Vec<f64> = curvature.iter().map(|&c| p.migration * w * w * c).collect();
        let mut r1 = vec![0.; n];
        let (mut sum, mut norm) = (0., 0.);
        for k in 1..n {
            let ds = dist(pts[k - 1], pts[k]);
            let fade = (-decay * ds).exp();
            sum = sum * fade + r0[k - 1] * ds;
            norm = norm * fade + ds;
            r1[k] = p.omega * r0[k] + p.gamma * sum / norm;
        }
        let fastest = r1.iter().map(|r| r.abs()).fold(0., f64::max);
        let step = if fastest > 0. { (0.2 * w / fastest).min(dt - t) } else { dt - t };
        // Positive rates move towards the outer bank, which for a left turn is on the right.
        for k in 2..n - 2 {
            let m = -r1[k] * step;
            let q = [c.points[k][0] + normal[k][0] * m, c.points[k][1] + normal[k][1] * m];
            if movable(q) { c.points[k] = q; }
        }
        c.points = resample(&c.points, w);
        t += step;
    }
}

/// Cut off loops whose necks have closed, returning them as ox-bow lakes.
fn cutoffs(c: &mut Channel) -> Vec<Oxbow> {
    let mut lakes = vec![];
    let w = c.width;
    'search: loop {
        let n = c.points.len();
        for i in 0..n {
            // Far enough apart along the channel to be different loops.
            for j in i + 6..n {
                if dist(c.points[i], c.points[j]) < w {
                    lakes.push(Oxbow { points: c.points[i..=j].to_vec(), width: w, depth: w / 20. });
                    c.points.drain(i + 1..j);
                    continue 'search;
                }
            }
        }
        break;
    }
    lakes
}

/// Cells of each reach where rivers meander, as (first, last, path) along the river.
fn reaches(domain: &Domain, receiver: &[usize], area: &[f64], slope: &[f64], sediment: &[f64], wet: &[bool], p: &ChannelParams) -> Vec<Vec<usize>> {
    let n = domain.len();
    let alluvial: Vec<bool> = (0..n).map(|i| {
        !wet[i] && !domain.boundary[i] && area[i] >= p.min_area && slope[i] <= p.max_slope && sediment[i] >= p.min_sediment
    }).collect();
    // Each alluvial cell continues the reach of its largest alluvial donor.
    let mut main_donor = vec![usize::MAX; n];
    for i in 0..n {
        let r = receiver[i];
        if !alluvial[i] || r == i || !alluvial[r] { continue; }
        if main_donor[r] == usize::MAX || area[i] > area[main_donor[r]] { main_donor[r] = i; }
    }
    let mut out = vec![];
    for i in 0..n {
        // Heads: alluvial cells no alluvial donor continues into.
        if !alluvial[i] || main_donor[i] != usize::MAX { continue; }
        let mut path = vec![i];
        let mut c = i;
        loop {
            let r = receiver[c];
            if r == c || !alluvial[r] || main_donor[r] != c { break; }
            path.push(r);
            c = r;
        }
        if path.len() >= 3 { out.push(path); }
    }
    out
}

impl Rivers {
    /// Advance the rivers to `time` (`dt` years on). The other arguments describe the terrain
    /// now: each cell's receiver, drainage area, slope to its receiver, sediment thickness,
    /// whether it is under water or ice, and its ground elevation.
    #[allow(clippy::too_many_arguments)]
    pub fn step(&mut self, domain: &Domain, seed: u64, time: f64, dt: f64, receiver: &[usize], area: &[f64], slope: &[f64], sediment: &[f64], wet: &[bool], ground: &[f64], p: &ChannelParams) {
        if !matches!(domain.layout, Layout::Patch { .. }) { return; }

        if self.found.is_none_or(|f| time - f >= p.regrid) {
            self.found = Some(time);
            let old: BTreeMap<usize, Channel> = self.channels.drain(..).map(|c| (c.end, c)).collect();
            for path in reaches(domain, receiver, area, slope, sediment, wet, p) {
                let (start, end) = (path[0], *path.last().unwrap());
                let width = p.width_coefficient * area[end].sqrt();
                match old.get(&end) {
                    // Keep a channel whose reach still ends where it did, re-anchored at the
                    // reach's start.
                    Some(c) => {
                        let mut c = c.clone();
                        c.points[0] = cell_point(domain, start);
                        c.start = start;
                        c.width = width;
                        c.path = path.clone();
                        c.points = resample(&c.points, width);
                        self.channels.push(c);
                    }
                    None => {
                        let mut rng = Pcg32::new(seed ^ 0x6d65_616e, (end as u64) << 20 ^ time.to_bits());
                        let line: Vec<Point> = path.iter().map(|&i| cell_point(domain, i)).collect();
                        let mut points = resample(&line, width);
                        let last = points.len() - 1;
                        for (k, q) in points.iter_mut().enumerate() {
                            if k == 0 || k == last { continue; }
                            q[0] += (rng.next_f64() - 0.5) * 0.2 * width;
                            q[1] += (rng.next_f64() - 0.5) * 0.2 * width;
                        }
                        self.channels.push(Channel { points, width, start, end, path });
                    }
                }
            }
        }

        // The valley floor: cells reached from the reach's cells without climbing more than two
        // bank heights above the river cell they were reached from.
        let floodplain = |c: &Channel| {
            let bank = c.width / 20.;
            let mut floor: BTreeSet<usize> = c.path.iter().copied().collect();
            let mut frontier: Vec<(usize, f64)> = c.path.iter().map(|&i| (i, ground[i])).collect();
            while let Some((i, level)) = frontier.pop() {
                for l in domain.links(i) {
                    let j = l.to;
                    if !floor.contains(&j) && !wet[j] && !domain.boundary[j] && ground[j] <= level + 2. * bank {
                        floor.insert(j);
                        frontier.push((j, level));
                    }
                }
            }
            move |q: Point| floor.contains(&point_cell(domain, q))
        };
        let mut lakes = vec![];
        for c in &mut self.channels {
            let movable = floodplain(c);
            migrate(c, p, dt, &movable);
            lakes.extend(cutoffs(c));
        }
        for o in &mut self.oxbows { o.depth -= p.oxbow_fill * dt; }
        self.oxbows.retain(|o| o.depth > 0.);
        self.oxbows.extend(lakes);
    }
}

/// Water drawn as lines over a patch: meandering channels, ox-bow lakes, and the other rivers
/// from cell to cell, each at its width. Segments are indexed by the cells they cross.
pub struct WaterLines {
    segments: Vec<(Point, Point, f64)>,
    by_cell: Vec<Vec<u32>>,
}

impl WaterLines {
    /// `rivers` are cells drawn as rivers (to their receivers), with their widths, unless a
    /// meandering channel covers them.
    pub fn new(domain: &Domain, channels: &[Channel], oxbows: &[Oxbow], river_cells: &[(usize, usize, f64)]) -> WaterLines {
        let mut segments = vec![];
        let covered: BTreeSet<usize> = channels.iter().flat_map(|c| c.path.iter().copied()).collect();
        for c in channels {
            for w in c.points.windows(2) { segments.push((w[0], w[1], c.width)); }
        }
        for o in oxbows {
            for w in o.points.windows(2) { segments.push((w[0], w[1], o.width)); }
        }
        for &(i, r, width) in river_cells {
            if covered.contains(&i) && covered.contains(&r) { continue; }
            segments.push((cell_point(domain, i), cell_point(domain, r), width));
        }
        let mut by_cell = vec![vec![]; domain.len()];
        let spacing = domain.min_spacing();
        for (k, &(a, b, _)) in segments.iter().enumerate() {
            let steps = (dist(a, b) / (0.5 * spacing)).ceil().max(1.) as usize;
            let mut cells = BTreeSet::new();
            for s in 0..=steps {
                let t = s as f64 / steps as f64;
                let c = point_cell(domain, [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
                cells.insert(c);
                cells.extend(domain.links(c).iter().map(|l| l.to));
            }
            for c in cells { by_cell[c].push(k as u32); }
        }
        WaterLines { segments, by_cell }
    }

    /// Is there water at `q` (in `cell`)?
    pub fn wet(&self, cell: usize, q: Point) -> bool {
        self.by_cell[cell].iter().any(|&k| {
            let (a, b, w) = self.segments[k as usize];
            let (ab, aq) = ([b[0] - a[0], b[1] - a[1]], [q[0] - a[0], q[1] - a[1]]);
            let len2 = ab[0] * ab[0] + ab[1] * ab[1];
            let t = if len2 > 0. { ((aq[0] * ab[0] + aq[1] * ab[1]) / len2).clamp(0., 1.) } else { 0. };
            dist(q, [a[0] + ab[0] * t, a[1] + ab[1] * t]) < 0.5 * w
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn amplitude(c: &Channel) -> f64 {
        c.points.iter().map(|q| q[1].abs()).fold(0., f64::max)
    }

    #[test]
    fn test_bends_grow_and_cut_off() {
        // A straight channel along x with a gentle wiggle, on an unconfined floodplain.
        let width = 100.;
        let points: Vec<Point> = (0..=200).map(|k| {
            let x = k as f64 * width;
            [x, 5. * (x / 1200.).sin()]
        }).collect();
        let mut c = Channel { points, width, start: 0, end: 0, path: vec![] };
        let p = ChannelParams::default();
        let start = amplitude(&c);
        let mut lakes = vec![];
        for _ in 0..400 {
            migrate(&mut c, &p, 100., &|_| true);
            lakes.extend(cutoffs(&mut c));
        }
        // Bends grew from a few metres to many widths, and some necks were cut.
        assert!(amplitude(&c) > 10. * start, "{} -> {}", start, amplitude(&c));
        assert!(!lakes.is_empty());
        // The ends stayed put.
        assert_eq!(c.points[0], [0., 0.]);
        assert!((c.points.last().unwrap()[0] - 20000.).abs() < 1e-9);
        // Cut-off loops are loops: their ends are within a width of each other.
        for l in &lakes {
            assert!(dist(l.points[0], *l.points.last().unwrap()) < width);
            assert!(l.points.len() > 6);
        }
    }

    #[test]
    fn test_confined_channel_stays_in_its_valley() {
        let width = 100.;
        let points: Vec<Point> = (0..=100).map(|k| [k as f64 * width, 5. * (k as f64 / 12.).sin()]).collect();
        let mut c = Channel { points, width, start: 0, end: 0, path: vec![] };
        let p = ChannelParams::default();
        for _ in 0..100 {
            migrate(&mut c, &p, 100., &|q| q[1].abs() < 300.);
        }
        assert!(c.points.iter().all(|q| q[1].abs() < 300.));
    }

    #[test]
    fn test_resample_keeps_ends_and_spacing() {
        let line = vec![[0., 0.], [1000., 0.], [1000., 1000.]];
        let r = resample(&line, 100.);
        assert_eq!(r[0], [0., 0.]);
        assert_eq!(*r.last().unwrap(), [1000., 1000.]);
        assert_eq!(r.len(), 21);
        for w in r.windows(2) { assert!((dist(w[0], w[1]) - 100.).abs() < 1e-6 || dist(w[0], w[1]) < 100.); }
    }
}
