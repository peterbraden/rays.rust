//! # Where water stands on the ground as it is drawn
//!
//! The landscape's simulation knows its rivers and basins at its own cells (hundreds of metres),
//! but the ground the renderer draws is smoothed between those cells, has valley floors blended
//! to the rivers' beds, and synthetic relief added below them (`scenery::Relief`). So the two
//! disagree near water: a lake's level from the simulation could stand above the drawn rim
//! (water hanging in the air over the valley beyond) or below it, and a river beside a lake ran
//! tens of metres below the lake's level a few dozen metres from it.
//!
//! Here water is found on the drawn ground itself. A patch's ground, as drawn, is sampled on a
//! fine square grid (`Basins::build`; several samples a simulation cell), and filled by
//! Priority-Flood (Barnes, Lehman and Mulla 2014, as `flow::route` fills the simulation's
//! grid): flooding inwards from the patch's edge and the sea in order of height, each cell
//! raised to at least the level it was reached from, so every closed basin fills to the height
//! of the lowest point of its rim, where it overflows. Where the filled surface stands more than
//! `LAKE_DEPTH` above the ground is a lake, at that level; a river's level follows the filled
//! surface (`rivers::Rivers::level`), so it runs through a lake at the lake's level and out of
//! it down from the very point it spills over. Water and ground agree by construction.
//!
//! A lake is drawn as flat water over its cells and the ring of cells about them (`water`):
//! every cell of that ring stands at or above the lake's level at its sample (else the fill
//! would have flooded it), so the water there only shows where the ground between samples dips
//! below, and never hangs over lower ground beyond.
//!
//! Barnes, Lehman, Mulla, "Priority-Flood: An optimal depression-filling and watershed-labeling
//! algorithm for digital elevation models", Computers & Geosciences 62, 2014,
//! doi:10.1016/j.cageo.2013.04.024.

use crate::terrain::channel::Point;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Water this much deeper than the ground (metres) is a lake.
pub const LAKE_DEPTH: f64 = 1.;


/// The drawn ground on a grid over a patch, and its surface filled to where water stands.
pub struct Basins {
    /// The grid's first sample (patch coordinates) and spacing, metres; n × n samples.
    origin: Point,
    step: f64,
    n: usize,
    ground: Vec<f32>,
    filled: Vec<f32>,
    /// Which samples are under a lake large enough to trust (`SMALLEST`).
    lake: Vec<bool>,
}

/// Lakes covering fewer samples than this are left dry: a few samples across, they are below
/// what the grid resolves of the drawn ground (a river's narrow gorge between samples reads as
/// a pond over it, and its water would hang over the gorge).
pub const SMALLEST: usize = 16;

#[derive(PartialEq)]
struct Entry(f64, usize);
impl Eq for Entry {}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.total_cmp(&other.0).then(self.1.cmp(&other.1)) }
}

impl Basins {
    /// The ground `ground` (patch point → height, metres) over the square of half side `half`
    /// about the patch's centre, every `step` metres, filled; water drains off the edge, and
    /// into the sea, wherever the ground is below sea level.
    ///
    /// `channels` (points and heights) breach the ground: each lowers the sample nearest it to
    /// at most that height. Rivers' beds, so that water drains along them through the dams the
    /// drawn ground has across its valleys (synthetic relief, smoothing), and only basins no river
    /// drains fill: hydrologists condition elevation models so, breaching before filling
    /// (Lindsay, "Efficient hybrid breaching-filling sink removal methods for flow path
    /// enforcement in digital elevation models", Hydrological Processes 30(6), 2016,
    /// doi:10.1002/hyp.10648; bibliographic details from memory, to verify).
    ///
    /// Water leaves wherever it reaches within `margin` metres of the square's edge: a patch's
    /// outer cells are its simulation's outlets, and there the drawn ground blends into the
    /// coarser patch about it, which would otherwise wall the patch in.
    pub fn build(half: f64, step: f64, margin: f64, ground: impl Fn(Point) -> f64 + Sync, channels: &[(Point, f64)]) -> Basins {
        use rayon::prelude::*;
        let n = ((2. * half / step).ceil() as usize + 1).max(2);
        let origin = [-half, -half];
        let mut ground: Vec<f32> = (0..n * n).into_par_iter()
            .map(|k| ground([origin[0] + (k % n) as f64 * step, origin[1] + (k / n) as f64 * step]) as f32).collect();
        for &(q, h) in channels {
            let (u, v) = (((q[0] - origin[0]) / step).round(), ((q[1] - origin[1]) / step).round());
            if u < 0. || v < 0. || u >= n as f64 || v >= n as f64 { continue; }
            let k = v as usize * n + u as usize;
            ground[k] = ground[k].min(h as f32);
        }
        let filled = Basins::fill(n, &ground, (margin / step).ceil() as usize);
        let lake = Basins::large(n, &ground, &filled);
        Basins { origin, step, n, ground, filled, lake }
    }

    /// The samples of lakes of at least `SMALLEST` samples (8-connected).
    fn large(n: usize, ground: &[f32], filled: &[f32]) -> Vec<bool> {
        let wet: Vec<bool> = (0..n * n).map(|k| (filled[k] - ground[k]) as f64 > LAKE_DEPTH).collect();
        let mut keep = vec![false; n * n];
        let mut seen = vec![false; n * n];
        for start in 0..n * n {
            if !wet[start] || seen[start] { continue; }
            let mut component = vec![start];
            seen[start] = true;
            let mut i = 0;
            while i < component.len() {
                let k = component[i];
                i += 1;
                let (x, z) = ((k % n) as i64, (k / n) as i64);
                for (dx, dz) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                    let (a, b) = (x + dx, z + dz);
                    if a < 0 || b < 0 || a >= n as i64 || b >= n as i64 { continue; }
                    let j = b as usize * n + a as usize;
                    if wet[j] && !seen[j] { seen[j] = true; component.push(j); }
                }
            }
            if component.len() >= SMALLEST { for k in component { keep[k] = true; } }
        }
        keep
    }

    /// Priority-Flood over an n × n grid (8 neighbours), without the ε that routing needs (a
    /// slope across a lake towards its outlet): lakes here are level.
    fn fill(n: usize, ground: &[f32], margin: usize) -> Vec<f32> {
        let mut filled: Vec<f64> = ground.iter().map(|&g| g as f64).collect();
        let mut closed = vec![false; n * n];
        let mut heap = BinaryHeap::new();
        for k in 0..n * n {
            let (x, z) = (k % n, k / n);
            if x <= margin || z <= margin || x + margin >= n - 1 || z + margin >= n - 1 || filled[k] <= 0. {
                closed[k] = true;
                heap.push(Reverse(Entry(filled[k], k)));
            }
        }
        while let Some(Reverse(Entry(h, k))) = heap.pop() {
            let (x, z) = ((k % n) as i64, (k / n) as i64);
            for (dx, dz) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let (a, b) = (x + dx, z + dz);
                if a < 0 || b < 0 || a >= n as i64 || b >= n as i64 { continue; }
                let j = b as usize * n + a as usize;
                if closed[j] { continue; }
                closed[j] = true;
                filled[j] = filled[j].max(h);
                heap.push(Reverse(Entry(filled[j], j)));
            }
        }
        filled.into_iter().map(|f| f as f32).collect()
    }

    /// The grid position of `q`, in cells (fractional).
    fn at(&self, q: Point) -> Option<(f64, f64)> {
        let (u, v) = ((q[0] - self.origin[0]) / self.step, (q[1] - self.origin[1]) / self.step);
        let top = (self.n - 1) as f64;
        (u >= 0. && v >= 0. && u <= top && v <= top).then_some((u, v))
    }

    /// The filled surface at `q`, interpolated between samples: where water would stand there,
    /// or the ground where it is dry.
    pub fn filled(&self, q: Point) -> Option<f64> {
        let (u, v) = self.at(q)?;
        let (i, j) = ((u.floor() as usize).min(self.n - 2), (v.floor() as usize).min(self.n - 2));
        let (fu, fv) = (u - i as f64, v - j as f64);
        let f = |a: usize, b: usize| self.filled[b * self.n + a] as f64;
        Some(f(i, j) * (1. - fu) * (1. - fv) + f(i + 1, j) * fu * (1. - fv) + f(i, j + 1) * (1. - fu) * fv + f(i + 1, j + 1) * fu * fv)
    }

    fn lake_level(&self, k: usize) -> Option<f64> {
        self.lake[k].then_some(self.filled[k] as f64)
    }

    /// The level of the lake over `q`, if the sample nearest it is under one.
    pub fn lake(&self, q: Point) -> Option<f64> {
        let (u, v) = self.at(q)?;
        self.lake_level(v.round() as usize * self.n + u.round() as usize)
    }

    /// The lakes' water as flat rectangles (patch coordinates: least corner, greatest corner,
    /// level): each lake sample's cell, and each cell of the ring about a lake at the level of
    /// the lake beside it, runs of equal level along a row joined into one.
    /// Only samples `keep` accepts are covered (a coarse patch's lakes stop where a finer
    /// patch draws the ground).
    pub fn water(&self, keep: impl Fn(Point) -> bool) -> Vec<(Point, Point, f64)> {
        let n = self.n;
        let mut level = vec![f64::NAN; n * n];
        for k in 0..n * n {
            let Some(l) = self.lake_level(k) else { continue };
            let (x, z) = ((k % n) as i64, (k / n) as i64);
            for dz in -1..=1 {
                for dx in -1..=1 {
                    let (a, b) = (x + dx, z + dz);
                    if a < 0 || b < 0 || a >= n as i64 || b >= n as i64 { continue; }
                    let j = b as usize * n + a as usize;
                    // A lake's own sample at its own level; a ring's at its neighbour's.
                    let l = self.lake_level(j).unwrap_or(l);
                    if level[j].is_nan() || l > level[j] { level[j] = l; }
                }
            }
        }
        for (k, l) in level.iter_mut().enumerate() {
            if !l.is_nan() && !keep([self.origin[0] + (k % n) as f64 * self.step, self.origin[1] + (k / n) as f64 * self.step]) { *l = f64::NAN; }
        }
        let mut out = vec![];
        let h = 0.5 * self.step;
        for z in 0..n {
            let mut x = 0;
            while x < n {
                let l = level[z * n + x];
                if l.is_nan() { x += 1; continue; }
                let start = x;
                while x + 1 < n && level[z * n + x + 1] == l { x += 1; }
                let (x0, x1) = (self.origin[0] + start as f64 * self.step - h, self.origin[0] + x as f64 * self.step + h);
                let zc = self.origin[1] + z as f64 * self.step;
                out.push(([x0, zc - h], [x1, zc + h], l));
                x += 1;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_basin_fills_to_its_rim_and_spills_at_its_lowest() {
        // A bowl 20 m deep in a plain at 100 m, its rim notched to 105 m on one side.
        let ground = |q: Point| {
            let r = q[0].hypot(q[1]);
            let bowl = if r < 300. { 80. + 20. * (r / 300.).powi(2) } else { 100. };
            let rim = if (r - 300.).abs() < 40. { if q[0] > 250. && q[1].abs() < 40. { 105. } else { 112. } } else { bowl };
            rim
        };
        let b = Basins::build(1000., 20., 0., ground, &[]);
        // The lake stands at the notch, everywhere in the bowl, and the plain outside is dry.
        let level = b.lake([0., 0.]).expect("a lake in the bowl");
        assert!((level - 105.).abs() < 0.1, "{}", level);
        assert_eq!(b.lake([100., 100.]), Some(level));
        assert_eq!(b.lake([600., 0.]), None);
        assert!((b.filled([600., 0.]).unwrap() - 100.).abs() < 1e-3);
        // Its water: rectangles at the level, covering the bowl.
        let w = b.water(|_| true);
        assert!(w.iter().all(|r| (r.2 - level).abs() < 0.1));
        assert!(w.iter().any(|r| r.0[0] <= 0. && r.1[0] >= 0. && r.0[1] <= 0. && r.1[1] >= 0.));
        // A tilted plain drains off its edge: no lakes.
        let tilted = Basins::build(500., 25., 0., |q| 10. + 0.01 * q[0], &[]);
        // A river's channel cut through the rim, out onto the plain, drains the bowl down to
        // the plain's level, where the channel leaves it.
        let channel: Vec<(Point, f64)> = (0..40).map(|i| ([250. + 5. * i as f64, 0.], 99.)).collect();
        let drained = Basins::build(1000., 20., 0., ground, &channel);
        assert!((drained.lake([0., 0.]).unwrap() - 100.).abs() < 0.1, "{:?}", drained.lake([0., 0.]));
        assert!(tilted.water(|_| true).is_empty());
    }
}
