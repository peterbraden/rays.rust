//! # Where water goes: flow routing
//!
//! Rivers are found by letting rain run downhill. Each cell passes its water to one neighbour,
//! its *receiver*: the steepest way down ("D8" routing on a grid, here on any graph of cells).
//! The receivers form a forest of trees rooted at *outlets* (the sea, or a patch's edge), and a
//! cell's *drainage area* A is its own area plus that of every cell upstream of it. A is the
//! proxy for a river's discharge in the stream power law (see `fluvial`).
//!
//! Braun and Willett ("A very efficient O(n), implicit and parallel method to solve the stream
//! power equation governing fluvial incision and landscape evolution", Geomorphology 180-181,
//! 2013, doi:10.1016/j.geomorph.2012.10.008) order the cells into a "stack" in which every cell
//! comes after its receiver. Accumulating A walks the stack backwards (donors before
//! receivers); the implicit river solver walks it forwards (receivers first). Their stack is
//! built by a depth-first search from the outlets; here it is simply the cells sorted by
//! elevation, which is equally valid because every receiver is strictly lower than its donors.
//!
//! ## Depressions
//!
//! A cell lower than all its neighbours (a pit) has no way down, and water would stop there.
//! Real pits fill to become lakes, which overflow at their lowest rim. Barnes, Lehman and Mulla
//! ("Priority-Flood: An optimal depression-filling and watershed-labeling algorithm for digital
//! elevation models", Computers & Geosciences 62, 2014, doi:10.1016/j.cageo.2013.04.024) find
//! the water level of every lake in O(n log n): flood inwards from the outlets in order of
//! elevation, using a priority queue, raising each newly reached cell to at least the level it
//! was reached from. Their "+ε" variant raises it slightly above, so the filled surface slopes
//! (imperceptibly) towards the lake's outlet, and every cell has a strictly lower neighbour.
//! Water is routed over the filled surface; the terrain itself is not filled, so lakes (eg. in
//! glacially overdeepened basins) persist until sediment or erosion removes them.
//!
//! Ties in the priority queue are broken by cell index, so routing is deterministic.

use crate::terrain::domain::Domain;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

/// Rise across each cell of a filled lake, metres (Barnes et al.'s ε).
const EPSILON: f64 = 1e-4;

pub struct Flow {
    /// Each cell's receiver; an outlet is its own receiver.
    pub receiver: Vec<usize>,
    /// Distance to the receiver, metres.
    pub receiver_dist: Vec<f64>,
    /// Cells ordered so that every cell comes after its receiver.
    pub stack: Vec<usize>,
    /// Drainage area, square metres.
    pub area: Vec<f64>,
    /// The surface with its depressions filled.
    pub filled: Vec<f64>,
}

#[derive(PartialEq)]
struct Entry(f64, usize);
impl Eq for Entry {}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0).then(self.1.cmp(&other.1))
    }
}

/// Route water over `surface`, draining to the cells marked as `outlet`. `rain` is each cell's
/// contribution to drainage area, as a fraction of its area (1 everywhere for plain area).
pub fn route(domain: &Domain, surface: &[f64], outlet: &[bool], rain: &[f64]) -> Flow {
    let n = domain.len();
    let mut filled = surface.to_vec();
    let mut closed = vec![false; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if outlet[i] {
            closed[i] = true;
            heap.push(Reverse(Entry(filled[i], i)));
        }
    }
    // A world with no outlet (eg. a planet with no sea) drains to its lowest point.
    if heap.is_empty() && n > 0 {
        let lowest = (0..n).min_by(|&a, &b| surface[a].total_cmp(&surface[b]).then(a.cmp(&b))).unwrap();
        closed[lowest] = true;
        heap.push(Reverse(Entry(filled[lowest], lowest)));
    }
    let mut is_outlet = vec![false; n];
    for Reverse(Entry(_, i)) in heap.iter() { is_outlet[*i] = true; }

    while let Some(Reverse(Entry(h, c))) = heap.pop() {
        for l in domain.links(c) {
            if closed[l.to] { continue; }
            closed[l.to] = true;
            filled[l.to] = filled[l.to].max(h + EPSILON);
            heap.push(Reverse(Entry(filled[l.to], l.to)));
        }
    }

    let mut receiver: Vec<usize> = (0..n).collect();
    let mut receiver_dist = vec![0.; n];
    for i in 0..n {
        if is_outlet[i] { continue; }
        let mut steepest = 0.;
        for l in domain.links(i) {
            let slope = (filled[i] - filled[l.to]) / l.dist;
            if slope > steepest {
                steepest = slope;
                receiver[i] = l.to;
                receiver_dist[i] = l.dist;
            }
        }
        debug_assert!(receiver[i] != i, "cell {} has no way down", i);
    }

    let mut stack: Vec<usize> = (0..n).collect();
    stack.sort_by(|&a, &b| filled[a].total_cmp(&filled[b]).then(a.cmp(&b)));

    let mut area: Vec<f64> = (0..n).map(|i| domain.area[i] * rain[i]).collect();
    for &i in stack.iter().rev() {
        let r = receiver[i];
        if r != i { area[r] += area[i]; }
    }

    Flow { receiver, receiver_dist, stack, area, filled }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patch(n: usize) -> Domain { Domain::patch(0., 0., n, 1., 6.371e6) }

    #[test]
    fn test_slope_drains_downhill() {
        // A plane tilted down towards i = 0.
        let d = patch(5);
        let h: Vec<f64> = (0..25).map(|k| (k % 5) as f64).collect();
        let f = route(&d, &h, &d.boundary, &vec![1.; 25]);
        // Interior cells drain west, towards lower i.
        for j in 1..4 {
            for i in 1..4 {
                let k = j * 5 + i;
                assert_eq!(f.receiver[k] % 5, i - 1);
            }
        }
        // All area ends up at outlets.
        let outlet_area: f64 = (0..25).filter(|&k| f.receiver[k] == k).map(|k| f.area[k]).sum();
        assert!((outlet_area - 25.).abs() < 1e-9);
    }

    #[test]
    fn test_pit_overflows() {
        // A bowl whose centre is a pit; the rim is lowest at one side.
        let d = patch(5);
        let mut h = vec![10.; 25];
        for k in [6, 7, 8, 11, 13, 16, 17, 18] { h[k] = 5.; }
        h[12] = 0.;
        h[10] = 1.; // the outlet on the rim
        let f = route(&d, &h, &d.boundary, &vec![1.; 25]);
        // The pit is filled above the rim cell it spills over and drains, eventually, to it.
        assert!(f.filled[12] > 5.);
        let mut k = 12;
        while f.receiver[k] != k { k = f.receiver[k]; }
        assert_eq!(k, 10);
        assert_eq!(f.area[10], 1. + 9.);
        // Unfilled cells keep their heights.
        assert_eq!(f.filled[0], 10.);
    }

    #[test]
    fn test_stack_puts_receivers_first() {
        let d = Domain::sphere(6, 1000.);
        let h: Vec<f64> = d.dirs.iter().map(|v| (v.x * 7.).sin() + (v.y * 5.).cos() + v.z).collect();
        let outlet: Vec<bool> = h.iter().map(|&x| x < 0.).collect();
        let f = route(&d, &h, &outlet, &vec![1.; d.len()]);
        let mut pos = vec![0; d.len()];
        for (p, &i) in f.stack.iter().enumerate() { pos[i] = p; }
        for i in 0..d.len() {
            if f.receiver[i] != i { assert!(pos[f.receiver[i]] < pos[i]); }
        }
        let total: f64 = d.area.iter().sum();
        let drained: f64 = (0..d.len()).filter(|&k| f.receiver[k] == k).map(|k| f.area[k]).sum();
        assert!((total - drained).abs() < 1e-6 * total);
    }
}
