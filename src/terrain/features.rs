//! # Features of a landscape, and views of them
//!
//! A landscape is simulated from a seed, so until it is built nobody knows where its peaks,
//! glaciers or gorges are, and a camera has had to be placed by searching the picture. This
//! module reads what the simulation left (`Terrain`: the surface, the rivers' drainage, the
//! ice, the closed basins) and lists what is worth looking at, then proposes cameras that see
//! each. It is the first half of `plan.md` section 14 piece 3 ("a viewpoint chosen by the
//! renderer"): candidates from the terrain, scored on cheap views of each.
//!
//! ## Features
//!
//! Each is found on a simulation's cells (`find`), and is a point on the planet with a height,
//! a horizontal `size` and a vertical `relief` (what a camera has to frame), a `salience` to
//! rank it among its kind, and, for those seen best from one side, the way it `faces`.
//!
//! | kind | what it is | relief | salience |
//! |---|---|---|---|
//! | `peak` | a summit with topographic prominence: cells taken highest first and joined into islands by union-find; where two islands meet, the lower one's summit has its key col there (the standard algorithm) | prominence | prominence |
//! | `volcano` | a peak built of lava | prominence | prominence |
//! | `pass` | a peak's key col, on land: the lowest point on the highest ridge to higher ground | the lower peak's prominence | the same |
//! | `cliff` | connected cells steeper than `CLIFF_SLOPE` | the drop across them | the drop |
//! | `glacier` | connected ice, at its thickest; it faces down towards its snout | the ice's fall | area |
//! | `lake` | a closed basin's terminal lake (the lakes the landscape draws) | depth | area |
//! | `confluence` | two rivers of comparable size joining | the valley's depth | the lesser river's drainage |
//! | `gorge` | a river cut far below the ground on both sides of it | incision | incision |
//! | `headland` | land rising from the sea with sea on several sides | height | height × sea sides |
//!
//! A full-scale landscape's patches are searched each where it is the finest (`Mask`), so the
//! list runs from the finest patch's gullies to the coarsest's ranges; the close level, built
//! about whatever camera the scene has, is left out, as it moves with the camera.
//!
//! ## Views
//!
//! For each feature, cameras are tried on a ring about it (`Viewer::views`): `AZIMUTHS`
//! directions at three distances about the one that frames it (its relief filling about a third
//! of the frame's height, its size more than half its width, for the lens), standing at two
//! heights above the ground. Each is measured against the ground (`Surface`) by marching
//! straight rays over it:
//!
//! - `visible`: of five points on the feature (its centre, and a third of its size to either
//!   side and above), the share the camera sees;
//! - `clearance`: the least angle by which the line of sight to its centre clears the ground;
//! - `skyline`: whether the line of sight, carried on past the feature, meets no ground: the
//!   feature stands against the sky;
//! - `pitch`: how far the camera looks up (positive) or down;
//! - `framing`: the distance over the one that frames the feature;
//! - `depth` (for the most promising): a small depth image, rays through a grid over the
//!   frame, and from it the share of sky, of ground nearer than a third of the feature's
//!   distance, between that and twice it, and beyond; their spread (an entropy: a picture with
//!   foreground, subject and background has all three), and how much of the frame is blocked
//!   within a twentieth of the distance.
//!
//! These terms are kept on each `Shot`, and a `Judge` turns them into a score. The one here
//! (`Heuristic`) is a few rules of thumb; the terms are what a later judge (rendered drafts,
//! the sun's angle, a seeded choice among the best few, or a learned measure of what makes a
//! picture) reads, so that one replaces it without changing how candidates are found.

use crate::na::Vector3;
use crate::terrain::domain::{Domain, Layout};
use crate::terrain::simulation::{Terrain, ICE_FREE};
use rayon::prelude::*;
use serde_json::{json, Value};

/// What a feature is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind { Peak, Volcano, Pass, Cliff, Glacier, Lake, Confluence, Gorge, Headland }

impl Kind {
    pub const ALL: [Kind; 9] = [Kind::Peak, Kind::Volcano, Kind::Pass, Kind::Cliff, Kind::Glacier, Kind::Lake, Kind::Confluence, Kind::Gorge, Kind::Headland];

    pub fn name(&self) -> &'static str {
        match self {
            Kind::Peak => "peak", Kind::Volcano => "volcano", Kind::Pass => "pass", Kind::Cliff => "cliff",
            Kind::Glacier => "glacier", Kind::Lake => "lake", Kind::Confluence => "confluence",
            Kind::Gorge => "gorge", Kind::Headland => "headland",
        }
    }

    pub fn from_name(s: &str) -> Option<Kind> { Kind::ALL.iter().copied().find(|k| k.name() == s) }

    /// Whether the feature rises above what is about it, and is best seen from below or level,
    /// against the sky; otherwise it lies in or on the ground, and is best seen from above.
    pub fn rises(&self) -> bool { matches!(self, Kind::Peak | Kind::Volcano | Kind::Cliff | Kind::Headland | Kind::Pass) }
}

/// Something in the landscape to look at.
#[derive(Clone, Debug)]
pub struct Feature {
    pub kind: Kind,
    /// Where it is: a unit direction from the planet's centre...
    pub dir: Vector3<f64>,
    /// ... and the height of its surface there, metres above sea level.
    pub height: f64,
    /// How far across it is, metres.
    pub size: f64,
    /// How high it stands or how deep it lies, metres (see the module's table).
    pub relief: f64,
    /// How notable it is among its kind.
    pub salience: f64,
    /// The way it faces, if it is seen best from one side: a unit vector along the ground.
    pub faces: Option<Vector3<f64>>,
    /// The spacing of the simulation it was found in, metres.
    pub spacing: f64,
}

/// Slopes steeper than this (rise over run, between neighbouring cells) are cliffs: 35°.
pub const CLIFF_SLOPE: f64 = 0.7;

/// The least prominence a peak needs, metres, or a fraction of its cells' spacing.
const MIN_PROMINENCE: f64 = 40.;
const PROMINENCE_PER_SPACING: f64 = 0.15;

/// Of two rivers joining, the lesser has at least this share of the greater's drainage for a
/// confluence.
const CONFLUENCE_SHARE: f64 = 0.3;

/// A gorge's river lies at least this far below the ground two cells away on both sides, as a
/// share of that distance.
const GORGE_INCISION: f64 = 0.25;

/// At most this many of each kind are kept from a simulation.
const PER_KIND: usize = 16;

/// Which of a simulation's cells to search: those inside its own interior (not the band
/// along a patch's edge where it blends into the one around it) and outside every finer one's.
pub struct Mask { regions: Vec<(Layout, f64, bool)> }

impl Mask {
    /// Everything.
    pub fn all() -> Mask { Mask { regions: vec![] } }

    /// Within `t`'s interior, and outside the interiors of `finer`.
    pub fn of(t: &Terrain, finer: &[&Terrain]) -> Mask {
        let mut regions = vec![(t.domain.layout.clone(), t.domain.radius, true)];
        regions.extend(finer.iter().map(|f| (f.domain.layout.clone(), f.domain.radius, false)));
        Mask { regions }
    }

    pub fn keeps(&self, d: &Vector3<f64>) -> bool {
        self.regions.iter().all(|(layout, radius, inside)| interior(layout, *radius, d) == *inside)
    }
}

/// Whether `d` is inside a patch, clear of the band along its edge where it blends into what
/// is around it (`scenery::Layer`'s: a tenth of the patch, or three cells).
fn interior(layout: &Layout, radius: f64, d: &Vector3<f64>) -> bool {
    match layout {
        Layout::Sphere { .. } => true,
        Layout::Patch { up, east, north, n, spacing } => {
            if d.dot(up) <= 0.1 { return false; }
            let half = *n as f64 * spacing / 2.;
            let blend = (0.1 * *n as f64 * spacing).max(3. * spacing);
            let (x, z) = Domain::tangent(*up, *east, *north, radius, d);
            x.abs().max(z.abs()) < half - blend
        }
    }
}

/// Two unit vectors along the ground at `d`, east-ish and north-ish.
fn tangents(d: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let a = if d.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
    let east = a.cross(d).normalize();
    (east, d.cross(&east))
}

/// The direction `x` metres along `e` and `z` along `n` from `d`, over a planet of `radius`.
fn offset(d: &Vector3<f64>, e: &Vector3<f64>, n: &Vector3<f64>, radius: f64, x: f64, z: f64) -> Vector3<f64> {
    (d * radius + e * x + n * z).normalize()
}

/// The distance over the ground between two directions, metres.
fn distance(a: &Vector3<f64>, b: &Vector3<f64>, radius: f64) -> f64 {
    a.dot(b).clamp(-1., 1.).acos() * radius
}

/// The unit vector along the ground at `a` towards `b`, if they differ.
fn towards(a: &Vector3<f64>, b: &Vector3<f64>) -> Option<Vector3<f64>> {
    let v = b - a * a.dot(b);
    (v.norm() > 1e-12).then(|| v.normalize())
}

/// The features of `t` in the cells `mask` keeps, the most salient of each kind first.
pub fn find(t: &Terrain, mask: &Mask) -> Vec<Feature> {
    let d = &t.domain;
    let n = d.len();
    let radius = d.radius;
    let spacing = d.min_spacing();
    let s = t.surface();
    let sea = t.sea_level();
    let keep: Vec<bool> = (0..n).map(|i| !d.boundary[i] && mask.keeps(&d.dirs[i])).collect();
    let land = |i: usize| t.bed[i] >= sea;
    let feature = |kind, i: usize, size: f64, relief: f64, salience: f64, faces| Feature {
        kind, dir: d.dirs[i], height: s[i], size: size.max(2. * spacing), relief, salience, faces, spacing,
    };
    let mut out = vec![];

    // Peaks and their key cols, by prominence.
    let (prominence, col) = prominence(d, &s, sea);
    let least = MIN_PROMINENCE.max(PROMINENCE_PER_SPACING * spacing);
    for i in 0..n {
        if !keep[i] || !land(i) || prominence[i] < least { continue; }
        let kind = if t.lava[i] > 5. { Kind::Volcano } else { Kind::Peak };
        let reach = col[i].map_or(4. * spacing, |c| distance(&d.dirs[i], &d.dirs[c], radius));
        out.push(feature(kind, i, 2. * reach.min(2. * prominence[i]), prominence[i], prominence[i], None));
        if let Some(c) = col[i] && keep[c] && land(c) && s[c] > sea {
            out.push(feature(Kind::Pass, c, reach, prominence[i], prominence[i], towards(&d.dirs[c], &d.dirs[i])));
        }
    }

    // Cliffs: connected steep cells, by the drop across them.
    let steepest: Vec<(f64, usize)> = (0..n).map(|i| {
        d.links(i).iter().map(|l| ((s[i] - s[l.to]) / l.dist, l.to)).fold((0., i), |a, b| if b.0 > a.0 { b } else { a })
    }).collect();
    let steep: Vec<bool> = (0..n).map(|i| keep[i] && land(i) && t.ice[i] < ICE_FREE && steepest[i].0 > CLIFF_SLOPE).collect();
    for cells in components(d, &steep) {
        let top = cells.iter().map(|&i| s[i]).fold(f64::NEG_INFINITY, f64::max);
        let foot = cells.iter().map(|&i| s[steepest[i].1]).fold(f64::INFINITY, f64::min);
        let &i = cells.iter().max_by(|&&a, &&b| (s[a] - s[steepest[a].1]).total_cmp(&(s[b] - s[steepest[b].1]))).unwrap();
        let drop = top - foot;
        let faces = towards(&d.dirs[i], &d.dirs[steepest[i].1]);
        out.push(feature(Kind::Cliff, i, (cells.len() as f64).sqrt() * spacing, drop, drop, faces));
    }

    // Glaciers: connected ice, at its thickest, facing its snout.
    let ice: Vec<bool> = (0..n).map(|i| keep[i] && t.ice[i] >= ICE_FREE).collect();
    for cells in components(d, &ice) {
        let &thickest = cells.iter().max_by(|&&a, &&b| t.ice[a].total_cmp(&t.ice[b])).unwrap();
        let &snout = cells.iter().min_by(|&&a, &&b| s[a].total_cmp(&s[b])).unwrap();
        let area: f64 = cells.iter().map(|&i| d.area[i]).sum();
        let fall = s[cells.iter().copied().max_by(|&a, &b| s[a].total_cmp(&s[b])).unwrap()] - s[snout];
        out.push(feature(Kind::Glacier, thickest, area.sqrt(), fall, area / 1e6, towards(&d.dirs[thickest], &d.dirs[snout])));
    }

    // Lakes: closed basins' terminal lakes, as the landscape draws them.
    let basins = t.basins();
    let wet: Vec<bool> = (0..n).map(|i| keep[i] && basins.salt_lake(i) && t.ice[i] < ICE_FREE).collect();
    for cells in components(d, &wet) {
        let &deepest = cells.iter().max_by(|&&a, &&b| basins.water[a].total_cmp(&basins.water[b])).unwrap();
        let area: f64 = cells.iter().map(|&i| d.area[i]).sum();
        out.push(feature(Kind::Lake, deepest, area.sqrt(), basins.water[deepest], area / 1e6, None));
    }

    // Confluences and gorges, on the rivers.
    let river = |i: usize| crate::terrain::map::is_river(t, i) && t.ice[i] < ICE_FREE && land(i) && t.receiver[i] != i;
    let mut donors: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        if river(i) { donors[t.receiver[i]].push(i); }
    }
    for i in 0..n {
        if !keep[i] || !river(i) { continue; }
        // How far below the ground two cells away the river lies, on its lower side.
        let rim = ring_low_high(d, &s, i);
        let incision = rim - s[i];
        let mut joining: Vec<f64> = donors[i].iter().map(|&j| t.drainage[j]).collect();
        joining.sort_by(|a, b| b.total_cmp(a));
        if joining.len() >= 2 && joining[1] >= CONFLUENCE_SHARE * joining[0] {
            out.push(feature(Kind::Confluence, i, 6. * spacing, incision.max(0.), joining[1], None));
        }
        if incision > GORGE_INCISION * 2. * spacing {
            let along = towards(&d.dirs[i], &d.dirs[t.receiver[i]]);
            out.push(feature(Kind::Gorge, i, 4. * spacing, incision, incision, along));
        }
    }

    // Headlands: land rising from the sea on several sides.
    for i in 0..n {
        if !keep[i] || !land(i) || s[i] < sea + 20. { continue; }
        let links = d.links(i);
        let wet: Vec<usize> = links.iter().filter(|l| t.bed[l.to] < sea).map(|l| l.to).collect();
        if wet.len() * 8 < links.len() * 3 { continue; }
        let mean = wet.iter().fold(Vector3::zeros(), |a, &j| a + d.dirs[j]);
        let faces = towards(&d.dirs[i], &(mean / wet.len() as f64));
        out.push(feature(Kind::Headland, i, 3. * spacing, s[i] - sea, (s[i] - sea) * wet.len() as f64, faces));
    }

    thin(out, radius)
}

/// Each cell's topographic prominence above `sea`, and its key col (where the island of ground
/// it is the summit of first meets higher ground), by union-find over the cells taken highest
/// first. Cells that aren't summits have none. The highest of a connected landmass has its
/// height above the lowest ground or the sea, and no col.
fn prominence(d: &Domain, s: &[f64], sea: f64) -> (Vec<f64>, Vec<Option<usize>>) {
    let n = d.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| s[b].total_cmp(&s[a]).then(a.cmp(&b)));
    let mut parent = vec![usize::MAX; n];
    let mut top = vec![usize::MAX; n];
    let mut prominence = vec![0.; n];
    let mut col = vec![None; n];
    fn find(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i { parent[i] = parent[parent[i]]; i = parent[i]; }
        i
    }
    let lowest = s.iter().copied().fold(f64::INFINITY, f64::min).max(sea);
    for &i in &order {
        parent[i] = i;
        top[i] = i;
        for l in d.links(i) {
            if parent[l.to] == usize::MAX { continue; }
            let (a, b) = (find(&mut parent, i), find(&mut parent, l.to));
            if a == b { continue; }
            let (high, low) = if s[top[a]] >= s[top[b]] { (a, b) } else { (b, a) };
            // The lower island's summit meets higher ground here.
            prominence[top[low]] = s[top[low]] - s[i].max(sea);
            col[top[low]] = Some(i);
            parent[low] = high;
        }
    }
    for i in 0..n {
        if parent[i] == i { prominence[top[i]] = s[top[i]] - lowest; }
    }
    // Islands that met only under the sea are summits of their own landmass.
    for i in 0..n {
        if col[i].is_some_and(|c| s[c] < sea) { col[i] = None; }
    }
    (prominence, col)
}

/// The cells in each connected set of `on`.
fn components(d: &Domain, on: &[bool]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; on.len()];
    let mut out = vec![];
    for start in 0..on.len() {
        if !on[start] || seen[start] { continue; }
        seen[start] = true;
        let mut stack = vec![start];
        let mut cells = vec![];
        while let Some(i) = stack.pop() {
            cells.push(i);
            for l in d.links(i) {
                if on[l.to] && !seen[l.to] { seen[l.to] = true; stack.push(l.to); }
            }
        }
        out.push(cells);
    }
    out
}

/// The ground two cells from `i` on its lower side: of each pair of opposite cells two links
/// away, the higher, and of those the lowest (so a valley's floor counts, not its open ends).
fn ring_low_high(d: &Domain, s: &[f64], i: usize) -> f64 {
    let here = d.dirs[i];
    let mut far: Vec<(Vector3<f64>, f64)> = vec![];
    for l in d.links(i) {
        for m in d.links(l.to) {
            if m.to != i { far.push((d.dirs[m.to] - here, s[m.to])); }
        }
    }
    if far.is_empty() { return s[i]; }
    let mut least = f64::INFINITY;
    for (v, h) in &far {
        // The highest cell roughly opposite this one.
        let opposite = far.iter().filter(|(w, _)| w.dot(v) < -0.5 * v.norm() * w.norm()).map(|(_, h)| *h).fold(f64::NEG_INFINITY, f64::max);
        if opposite.is_finite() { least = least.min(h.min(opposite)); }
    }
    if least.is_finite() { least } else { s[i] }
}

/// The most salient of each kind, no two of a kind nearer than half the larger's size (or two
/// cells), at most `PER_KIND` of each from each simulation: those of finer simulations first
/// (they are where the landscape is drawn in most detail), then the most salient.
fn thin(mut features: Vec<Feature>, radius: f64) -> Vec<Feature> {
    features.sort_by(|a, b| a.kind.cmp(&b.kind).then(a.spacing.total_cmp(&b.spacing)).then(b.salience.total_cmp(&a.salience)));
    let mut out: Vec<Feature> = vec![];
    for f in features {
        let mine: Vec<&Feature> = out.iter().filter(|g| g.kind == f.kind).collect();
        if mine.iter().filter(|g| g.spacing == f.spacing).count() >= PER_KIND { continue; }
        let apart = |g: &&Feature| distance(&f.dir, &g.dir, radius) > (0.5 * f.size.max(g.size)).max(2. * f.spacing.max(g.spacing));
        if mine.iter().all(apart) { out.push(f); }
    }
    out
}

/// Features found in several simulations, thinned as one list (`thin`): of each kind, the
/// finest simulation's first, most salient first.
pub fn merge(lists: Vec<Vec<Feature>>, radius: f64) -> Vec<Feature> {
    thin(lists.into_iter().flatten().collect(), radius)
}

/// The ground views are judged against: its height at any direction (the large-scale surface
/// is enough to tell what hides what), on a planet of `radius`.
pub struct Surface<'a> {
    pub height: &'a (dyn Fn(&Vector3<f64>) -> f64 + Sync),
    pub radius: f64,
    /// How far apart the height is worth sampling along a ray, metres.
    pub step: f64,
}

impl Surface<'_> {
    fn point(&self, d: &Vector3<f64>, h: f64) -> Vector3<f64> { d * (self.radius + h) }

    /// The height of `p` (planet-centred, metres) above the ground under it.
    fn above(&self, p: &Vector3<f64>) -> f64 {
        let r = p.norm();
        r - self.radius - (self.height)(&(p / r))
    }

    /// The least angle (radians, roughly) by which the segment from `a` to `b` clears the
    /// ground, leaving out its last `skip` share (where it reaches the ground it looks at).
    fn clearance(&self, a: &Vector3<f64>, b: &Vector3<f64>, skip: f64) -> f64 {
        let length = (b - a).norm();
        let steps = (length / self.step).clamp(16., 400.) as usize;
        let mut least = f64::INFINITY;
        for k in 1..steps {
            let t = k as f64 / steps as f64;
            if t > 1. - skip { break; }
            least = least.min(self.above(&(a + (b - a) * t)) / (t * length));
        }
        least
    }

    /// How far along `dir` from `a` the ray meets the ground, up to `reach` metres: marched in
    /// steps growing with the distance.
    fn depth(&self, a: &Vector3<f64>, dir: &Vector3<f64>, reach: f64) -> Option<f64> {
        let mut t = 0.;
        let mut last = self.above(a);
        while t < reach {
            let step = self.step.max(0.01 * t);
            let next = t + step;
            let h = self.above(&(a + dir * next));
            if h < 0. {
                // Between the last two samples, where the height crosses zero.
                return Some(t + step * last / (last - h).max(1e-9));
            }
            t = next;
            last = h;
        }
        None
    }
}

/// The lens views are framed for: `angle` as a camera's (the view plane at unit distance spans
/// its tangent over the image's height), and the image's width over its height.
#[derive(Clone, Copy, Debug)]
pub struct Lens { pub angle: f64, pub aspect: f64 }

impl Default for Lens {
    fn default() -> Lens { Lens { angle: 0.7, aspect: 16. / 9. } }
}

/// What a small depth image of a view holds (see the module documentation).
#[derive(Clone, Copy, Debug, Default)]
pub struct Depth { pub sky: f64, pub near: f64, pub middle: f64, pub far: f64, pub spread: f64, pub blocked: f64 }

/// A view's measurements (see the module documentation).
#[derive(Clone, Copy, Debug)]
pub struct Terms {
    pub visible: f64,
    pub clearance: f64,
    pub skyline: bool,
    pub pitch: f64,
    pub framing: f64,
    pub depth: Option<Depth>,
}

/// A camera looking at a feature.
#[derive(Clone, Debug)]
pub struct Shot {
    /// The camera's direction from the planet's centre, its height above the ground there,
    /// and the ground's height, metres.
    pub eye: Vector3<f64>,
    pub eye_height: f64,
    pub eye_ground: f64,
    /// The point looked at, its height (metres above sea level) and the ground's under it.
    pub target: Vector3<f64>,
    pub target_height: f64,
    pub target_ground: f64,
    /// How far the camera is from what it looks at, metres.
    pub distance: f64,
    pub terms: Terms,
    pub score: f64,
}

/// How good a shot is: higher is better; not finite for one not worth taking.
pub trait Judge: Sync {
    fn judge(&self, f: &Feature, terms: &Terms) -> f64;
}

/// Rules of thumb: see the feature whole and clear, at the distance that frames it; things
/// that rise against the sky from level or below, things in the ground from above; a picture
/// with a foreground and a background, a little sky, and nothing in front of the lens.
pub struct Heuristic;

impl Judge for Heuristic {
    fn judge(&self, f: &Feature, t: &Terms) -> f64 {
        if t.clearance <= 0. { return f64::NEG_INFINITY; }
        let mut score = 2. * t.visible + 5. * t.clearance.min(0.05) - t.framing.ln().abs();
        let (lo, hi) = if f.kind.rises() { (-0.08, 0.25) } else { (-0.45, -0.06) };
        score -= 4. * (lo - t.pitch).max(0.) + 4. * (t.pitch - hi).max(0.);
        if t.skyline && f.kind.rises() { score += 0.7; }
        if let Some(d) = t.depth {
            score += d.spread - 3. * d.blocked;
            score -= 2. * (0.1 - d.sky).max(0.) + 2. * (d.sky - 0.5).max(0.);
        }
        score
    }
}

/// Directions about a feature cameras are tried from.
pub const AZIMUTHS: usize = 12;

/// Heights above the ground cameras are tried at, metres: standing, and a little above.
pub const EYE_HEIGHTS: [f64; 2] = [2., 30.];

/// How many of a feature's most promising views get a depth image.
const DEPTH_VIEWS: usize = 4;

/// A depth image's rays across and up.
const DEPTH_GRID: (usize, usize) = (12, 7);

/// Finds and judges views of features over a surface.
pub struct Viewer<'a> {
    pub surface: Surface<'a>,
    pub lens: Lens,
    pub judge: &'a dyn Judge,
}

impl Viewer<'_> {
    /// The distance that frames `f` with this lens, metres.
    pub fn framing_distance(&self, f: &Feature) -> f64 {
        let h = self.lens.angle.tan();
        (f.relief.max(10.) / (0.35 * h)).max(f.size / (0.6 * h * self.lens.aspect)).clamp(60., 40e3)
    }

    /// The point a camera looks at on `f`: for what rises, below its top, so the top stands in
    /// the upper part of the frame; a cliff at half its height; the rest at the surface.
    fn aim(&self, f: &Feature, ground: f64, distance: f64) -> f64 {
        match f.kind {
            Kind::Peak | Kind::Volcano | Kind::Headland => ground - (0.15 * distance * self.lens.angle.tan()).min(0.5 * f.relief),
            Kind::Cliff => ground - 0.5 * f.relief,
            _ => ground + 1.,
        }
    }

    /// Views of `f`, the best first, all with the feature's centre in sight.
    pub fn views(&self, f: &Feature) -> Vec<Shot> {
        let s = &self.surface;
        let ideal = self.framing_distance(f);
        let (e, n) = tangents(&f.dir);
        let target_ground = (s.height)(&f.dir).max(f.height.min((s.height)(&f.dir) + f.relief));
        let mut candidates = vec![];
        for a in 0..AZIMUTHS {
            let angle = std::f64::consts::TAU * a as f64 / AZIMUTHS as f64;
            for k in [0.6, 1., 1.6] {
                for h in EYE_HEIGHTS {
                    candidates.push((angle, k * ideal, h));
                }
            }
        }
        let mut shots: Vec<Shot> = candidates.par_iter().filter_map(|&(angle, dist, eye_height)| {
            let eye = offset(&f.dir, &e, &n, s.radius, dist * angle.sin(), dist * angle.cos());
            let eye_ground = (s.height)(&eye);
            let target_height = self.aim(f, target_ground, dist);
            let mut shot = Shot {
                eye, eye_height, eye_ground, target: f.dir, target_height, target_ground, distance: dist,
                terms: Terms { visible: 0., clearance: 0., skyline: false, pitch: 0., framing: dist / ideal, depth: None },
                score: f64::NEG_INFINITY,
            };
            let a = s.point(&eye, eye_ground + eye_height);
            let centre = s.point(&f.dir, target_ground + 1. + 0.02 * f.relief);
            shot.terms.clearance = s.clearance(&a, &centre, (0.5 * f.spacing / dist).min(0.1));
            (shot.terms.clearance > 0.).then_some(shot)
        }).collect();
        shots.par_iter_mut().for_each(|shot| {
            self.measure(f, shot);
            shot.score = self.judge.judge(f, &shot.terms);
        });
        shots.sort_by(|a, b| b.score.total_cmp(&a.score));
        // Depth images for the most promising, which may reorder them.
        let deep = shots.len().min(DEPTH_VIEWS);
        shots[..deep].par_iter_mut().for_each(|shot| {
            shot.terms.depth = Some(self.depth(shot));
            shot.score = self.judge.judge(f, &shot.terms);
        });
        shots.retain(|s| s.score.is_finite());
        shots.sort_by(|a, b| b.score.total_cmp(&a.score));
        shots
    }

    /// A shot's visibility, skyline and pitch.
    fn measure(&self, f: &Feature, shot: &mut Shot) {
        let s = &self.surface;
        let a = s.point(&shot.eye, shot.eye_ground + shot.eye_height);
        let aim = s.point(&shot.target, shot.target_height);
        let up = a.normalize();
        let forward = (aim - a).normalize();
        shot.terms.pitch = forward.dot(&up).asin();
        // Five points on the feature: its centre, a third of its size to either side, and
        // above it by a third of its relief.
        let side = forward.cross(&up).normalize();
        let lift = 1. + 0.02 * f.relief;
        let base = s.point(&f.dir, shot.target_ground + lift);
        let centre_up = base.normalize();
        let points = [
            base,
            base + side * f.size / 3.,
            base - side * f.size / 3.,
            base + centre_up * f.relief / 3.,
            base + side * f.size / 4. + centre_up * f.relief / 6.,
        ];
        let skip = (0.5 * f.spacing / shot.distance).min(0.1);
        let seen = points.iter().map(|p| {
            // A point off the feature's centre is on the ground under it, or above.
            let q = p.normalize() * (s.radius + (s.height)(&p.normalize()).max(p.norm() - s.radius) + lift);
            s.clearance(&a, &q, skip) > 0.
        }).filter(|&v| v).count();
        shot.terms.visible = seen as f64 / points.len() as f64;
        // Carried on past the feature's top, does the line of sight meet the ground?
        let top = s.point(&f.dir, shot.target_ground + lift);
        let dir = (top - a).normalize();
        let past = top + dir * 1.;
        shot.terms.skyline = s.depth(&past, &dir, 4. * shot.distance).is_none();
    }

    /// A small depth image of the shot (see `Depth`).
    fn depth(&self, shot: &Shot) -> Depth {
        let s = &self.surface;
        let a = s.point(&shot.eye, shot.eye_ground + shot.eye_height);
        let aim = s.point(&shot.target, shot.target_height);
        let forward = (aim - a).normalize();
        let right = forward.cross(&a.normalize()).normalize();
        let up = right.cross(&forward);
        let h = self.lens.angle.tan();
        let w = h * self.lens.aspect;
        let (nx, ny) = DEPTH_GRID;
        let reach = 8. * shot.distance;
        let mut counts = [0usize; 4];
        let mut blocked = 0;
        for j in 0..ny {
            for i in 0..nx {
                let u = ((i as f64 + 0.5) / nx as f64 - 0.5) * w;
                let v = ((j as f64 + 0.5) / ny as f64 - 0.5) * h;
                let dir = (forward + right * u + up * v).normalize();
                match s.depth(&a, &dir, reach) {
                    None => counts[0] += 1,
                    Some(t) => {
                        if t < 0.05 * shot.distance { blocked += 1; }
                        counts[if t < shot.distance / 3. { 1 } else if t < 2. * shot.distance { 2 } else { 3 }] += 1;
                    }
                }
            }
        }
        let total = (nx * ny) as f64;
        let share = counts.map(|c| c as f64 / total);
        let ground: f64 = share[1..].iter().sum();
        // How evenly the ground is spread over near, middle and far: 1 for a third each.
        let spread = if ground > 0. {
            -share[1..].iter().filter(|&&p| p > 0.).map(|&p| { let q = p / ground; q * q.ln() }).sum::<f64>() / 3f64.ln()
        } else { 0. };
        Depth { sky: share[0], near: share[1], middle: share[2], far: share[3], spread, blocked: blocked as f64 / total }
    }
}

/// A feature with its best views.
pub struct Found {
    pub feature: Feature,
    pub shots: Vec<Shot>,
}

/// The views of each of `features` (most salient `per_kind` of each kind), and the features
/// none was found for left out.
pub fn survey(viewer: &Viewer, features: &[Feature], per_kind: usize) -> Vec<Found> {
    let mut count = std::collections::HashMap::new();
    features.iter().filter(|f| {
        let c = count.entry(f.kind).or_insert(0);
        *c += 1;
        *c <= per_kind
    }).filter_map(|f| {
        let shots = viewer.views(f);
        (!shots.is_empty()).then(|| Found { feature: f.clone(), shots })
    }).collect()
}

/// Which feature and view a camera asks for: `"feature": true` (the best view of any),
/// `"feature": "peak"`, or `{"kind": "peak" | "any", "rank": n, "view": n}` (the rank among its
/// kind, most salient first; the view, best first).
#[derive(Clone, Debug, PartialEq)]
pub struct Choice { pub kind: Option<Kind>, pub rank: usize, pub view: usize }

impl Choice {
    pub fn parse(v: &Value) -> Option<Choice> {
        let kind = |s: &str| if s == "any" { None } else {
            Some(Kind::from_name(s).unwrap_or_else(|| panic!("Unknown feature kind {:?}: expected \"any\" or one of {:?}", s, Kind::ALL.map(|k| k.name()))))
        };
        match v {
            Value::Null | Value::Bool(false) => None,
            Value::Bool(true) => Some(Choice { kind: None, rank: 0, view: 0 }),
            Value::String(s) => Some(Choice { kind: kind(s), rank: 0, view: 0 }),
            Value::Object(o) => Some(Choice {
                kind: o.get("kind").and_then(Value::as_str).map_or(None, kind),
                rank: o.get("rank").and_then(Value::as_u64).unwrap_or(0) as usize,
                view: o.get("view").and_then(Value::as_u64).unwrap_or(0) as usize,
            }),
            v => panic!("A camera's \"feature\" is true, a kind, or {{\"kind\", \"rank\", \"view\"}}: not {}", v),
        }
    }

    /// The feature and shot chosen from `found`: of a kind, its `rank`th; of any, the `rank`th
    /// best by its best view.
    pub fn pick<'a>(&self, found: &'a [Found]) -> Option<(&'a Found, &'a Shot)> {
        let mut of: Vec<&Found> = found.iter().filter(|f| self.kind.is_none_or(|k| f.feature.kind == k)).collect();
        if self.kind.is_none() { of.sort_by(|a, b| b.shots[0].score.total_cmp(&a.shots[0].score)); }
        let f = of.get(self.rank)?;
        Some((f, f.shots.get(self.view).or(f.shots.last())?))
    }
}

/// `found` as JSON: each feature with where it is (`to_scene` maps a direction and a height to
/// the scene) and its views as cameras, from `camera` (another `"camera"` to start from) with
/// `"location"` and `"lookat"`; for `above_ground`, as heights above the ground.
pub fn to_json(found: &[Found], to_scene: &dyn Fn(&Vector3<f64>, f64) -> Vector3<f64>, above_ground: bool, views: usize) -> Value {
    let v3 = |v: Vector3<f64>| json!([round(v.x), round(v.y), round(v.z)]);
    let list: Vec<Value> = found.iter().map(|f| {
        let ft = &f.feature;
        let (lat, lon) = Domain::lat_lon(&ft.dir);
        let shots: Vec<Value> = f.shots.iter().take(views).map(|s| {
            let t = &s.terms;
            let mut terms = json!({
                "visible": t.visible, "clearance": round3(t.clearance), "skyline": t.skyline,
                "pitch": round3(t.pitch.to_degrees()), "framing": round3(t.framing),
            });
            if let Some(d) = t.depth {
                terms["depth"] = json!({"sky": round3(d.sky), "near": round3(d.near), "middle": round3(d.middle), "far": round3(d.far), "spread": round3(d.spread), "blocked": round3(d.blocked)});
            }
            json!({"camera": camera(s, to_scene, above_ground), "distance": round(s.distance), "score": round3(s.score), "terms": terms})
        }).collect();
        json!({
            "kind": ft.kind.name(), "at": v3(to_scene(&ft.dir, ft.height)), "lat": round3(lat), "lon": round3(lon),
            "height": round(ft.height), "relief": round(ft.relief), "size": round(ft.size), "salience": round3(ft.salience),
            "spacing": ft.spacing, "views": shots,
        })
    }).collect();
    json!({ "features": list })
}

/// A shot as a scene camera's `location` and `lookat` (and `"above_ground": true` with heights
/// above the ground, for a full-scale landscape, so the scene's own ground places it).
pub fn camera(s: &Shot, to_scene: &dyn Fn(&Vector3<f64>, f64) -> Vector3<f64>, above_ground: bool) -> Value {
    let v3 = |v: Vector3<f64>| json!([round(v.x), round(v.y), round(v.z)]);
    if above_ground {
        let (eye, at) = (to_scene(&s.eye, 0.), to_scene(&s.target, 0.));
        json!({
            "location": v3(Vector3::new(eye.x, s.eye_height, eye.z)),
            "lookat": v3(Vector3::new(at.x, s.target_height - s.target_ground, at.z)),
            "above_ground": true,
        })
    } else {
        json!({ "location": v3(to_scene(&s.eye, s.eye_ground + s.eye_height)), "lookat": v3(to_scene(&s.target, s.target_height)) })
    }
}

fn round(x: f64) -> f64 { (x * 10.).round() / 10. }
fn round3(x: f64) -> f64 { (x * 1000.).round() / 1000. }

/// A line about a feature and its best view, for the command line.
pub fn describe(f: &Found, to_scene: &dyn Fn(&Vector3<f64>, f64) -> Vector3<f64>) -> String {
    let ft = &f.feature;
    let at = to_scene(&ft.dir, ft.height);
    let s = &f.shots[0];
    let eye = to_scene(&s.eye, s.eye_ground + s.eye_height);
    let measure = match ft.kind {
        Kind::Peak | Kind::Volcano => format!("prominence {:.0} m", ft.relief),
        Kind::Pass => format!("{:.0} m below the peak", ft.relief),
        Kind::Cliff => format!("{:.0} m drop", ft.relief),
        Kind::Glacier => format!("{:.1} km², falling {:.0} m", ft.salience, ft.relief),
        Kind::Lake => format!("{:.1} km²", ft.salience),
        Kind::Confluence => format!("{:.0} km² joining", ft.salience / 1e6),
        Kind::Gorge => format!("{:.0} m deep", ft.relief),
        Kind::Headland => format!("{:.0} m above the sea", ft.relief),
    };
    format!("{} at [{:.0}, {:.0}, {:.0}], {:.0} m high, {}; seen best from [{:.0}, {:.0}, {:.0}], {:.1} km away (score {:.2})",
        ft.kind.name(), at.x, at.y, at.z, ft.height, measure, eye.x, eye.y, eye.z, s.distance / 1e3, s.score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::simulation::TerrainParams;

    /// A patch with its ground set to `height(x, z)` (metres east and north of its centre).
    fn shaped(height: impl Fn(f64, f64) -> f64) -> Terrain {
        let mut p = TerrainParams::default();
        p.planet.seed = 1;
        let mut t = Terrain::new(Domain::patch(40., 10., 96, 100., 6.371e6), p);
        let Layout::Patch { up, east, north, .. } = t.domain.layout.clone() else { unreachable!() };
        for i in 0..t.domain.len() {
            let (x, z) = Domain::tangent(up, east, north, t.domain.radius, &t.domain.dirs[i]);
            t.bed[i] = height(x, z);
            t.ice[i] = 0.;
            t.lava[i] = 0.;
            t.sediment[i] = 0.;
            t.drainage[i] = 0.;
            t.receiver[i] = i;
        }
        t
    }

    fn hill(x: f64, z: f64, cx: f64, cz: f64, h: f64, r: f64) -> f64 {
        h * (-((x - cx).powi(2) + (z - cz).powi(2)) / (r * r)).exp()
    }

    #[test]
    fn test_two_hills_are_peaks_with_a_pass_between() {
        let t = shaped(|x, z| 100. + hill(x, z, -800., 0., 600., 500.) + hill(x, z, 700., 0., 400., 500.));
        let found = find(&t, &Mask::all());
        let peaks: Vec<&Feature> = found.iter().filter(|f| f.kind == Kind::Peak).collect();
        assert_eq!(peaks.len(), 2, "{:?}", peaks);
        // The higher stands above the plain; the lower above its col with the higher.
        assert!((peaks[0].relief - 600.).abs() < 30., "{}", peaks[0].relief);
        assert!(peaks[1].relief < 400. && peaks[1].relief > 250., "{}", peaks[1].relief);
        let pass = found.iter().find(|f| f.kind == Kind::Pass).expect("a pass");
        let Layout::Patch { up, east, north, .. } = t.domain.layout.clone() else { unreachable!() };
        let (x, _) = Domain::tangent(up, east, north, t.domain.radius, &pass.dir);
        assert!(x > -500. && x < 500., "the col at {}", x);
    }

    #[test]
    fn test_a_scarp_is_a_cliff_facing_down() {
        let t = shaped(|x, _| if x < 0. { 500. } else { 100. });
        let found = find(&t, &Mask::all());
        let cliff = found.iter().find(|f| f.kind == Kind::Cliff).expect("a cliff");
        assert!((cliff.relief - 400.).abs() < 1., "{}", cliff.relief);
        let Layout::Patch { east, .. } = t.domain.layout.clone() else { unreachable!() };
        assert!(cliff.faces.unwrap().dot(&east) > 0.9);
    }

    #[test]
    fn test_a_peak_is_seen_from_where_nothing_hides_it() {
        // A hill, with a wall to its south that hides it from there.
        let t = shaped(|x, z| hill(x, z, 0., 0., 500., 400.) + if (-1500. ..-1300.).contains(&z) && x.abs() < 2000. { 800. } else { 0. });
        let corners = crate::terrain::scenery::corner_heights(&t);
        let height = |d: &Vector3<f64>| crate::terrain::scenery::sample(&t.domain, &corners, d);
        let Layout::Patch { up: centre, .. } = t.domain.layout.clone() else { unreachable!() };
        let peak = find(&t, &Mask::all()).into_iter().filter(|f| f.kind == Kind::Peak)
            .min_by(|a, b| distance(&a.dir, &centre, 1.).total_cmp(&distance(&b.dir, &centre, 1.))).expect("the hill");
        assert!((peak.height - 500.).abs() < 30., "{:?}", peak);
        let viewer = Viewer { surface: Surface { height: &height, radius: t.domain.radius, step: 50. }, lens: Lens::default(), judge: &Heuristic };
        let shots = viewer.views(&peak);
        assert!(!shots.is_empty());
        let Layout::Patch { up, east, north, .. } = t.domain.layout.clone() else { unreachable!() };
        for s in &shots {
            assert!(s.terms.clearance > 0.);
            let (x, z) = Domain::tangent(up, east, north, t.domain.radius, &s.eye);
            // None from squarely behind the wall.
            assert!(z > -1500. || x.abs() > 1000., "a view from [{}, {}] behind the wall", x, z);
        }
        assert!(shots[0].terms.visible > 0.5);
    }

    #[test]
    fn test_a_choice_picks_by_kind_rank_and_view() {
        assert_eq!(Choice::parse(&json!(true)), Some(Choice { kind: None, rank: 0, view: 0 }));
        assert_eq!(Choice::parse(&json!("glacier")), Some(Choice { kind: Some(Kind::Glacier), rank: 0, view: 0 }));
        assert_eq!(Choice::parse(&json!({"kind": "peak", "rank": 2, "view": 1})), Some(Choice { kind: Some(Kind::Peak), rank: 2, view: 1 }));
        assert_eq!(Choice::parse(&json!(false)), None);
    }

    #[test]
    fn test_an_evolved_landscape_has_features_to_see() {
        let mut params = TerrainParams::default();
        params.planet.seed = 3;
        params.step = 500.;
        let planet = crate::terrain::planet::Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let mut t = Terrain::new(Domain::patch(lat, lon, 32, 1000., params.planet.radius), params);
        t.run_to(5000.);
        let found = find(&t, &Mask::of(&t, &[]));
        assert!(found.iter().any(|f| f.kind == Kind::Peak), "{:?}", found.iter().map(|f| f.kind).collect::<Vec<_>>());
        for f in &found {
            assert!(interior(&t.domain.layout, t.domain.radius, &f.dir), "{:?} at the edge", f.kind);
            assert!(f.size > 0. && f.relief >= 0.);
        }
    }
}
