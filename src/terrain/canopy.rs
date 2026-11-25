//! # Canopy: the ground under the trees
//!
//! A landscape's ground cover is drawn from its simulation and climate, which know nothing of
//! the trees an ecosystem plants on it. So an ecosystem on a landscape hands it the crowns it
//! placed (`Slot::add`): where each tree stands, how far its crown shades, and whether it
//! drops needles or leaves. The ground then draws forest floor under them (litter, and moss
//! where it is damp; `scenery::Ground::cover`) instead of the bare rock or soil the terrain
//! alone would show, and graphs read the same as a field, `canopy`: how much of the ground a
//! crown covers, 0 to 1.
//!
//! The ground's relief (`terrain::detail`) doesn't read it: the displaced cells' bounds are
//! found when the landscape is built, before any trees are planted, and taking the rock's
//! relief away under them would move the surface outside those bounds.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A tree's crown, seen from above: its trunk at (`x`, `z`) in the scene, the `radius` it
/// shades, and whether it drops needles (a conifer) rather than leaves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crown {
    pub x: f64,
    pub z: f64,
    pub radius: f64,
    pub needles: bool,
}

/// Crowns in a grid of cells, for finding those over a point.
#[derive(Debug, Default)]
pub struct Canopy {
    cell: f64,
    cells: HashMap<(i64, i64), Vec<Crown>>,
}

impl Canopy {
    pub fn new(crowns: &[Crown]) -> Canopy {
        let cell = crowns.iter().map(|c| c.radius).fold(1., f64::max) * 2.;
        let mut cells: HashMap<(i64, i64), Vec<Crown>> = HashMap::new();
        for c in crowns {
            cells.entry(((c.x / cell).floor() as i64, (c.z / cell).floor() as i64)).or_default().push(*c);
        }
        Canopy { cell, cells }
    }

    /// How much of the ground at (`x`, `z`) is under a crown, 0 to 1 (1 within most of its
    /// radius, fading to 0 at its edge, as the litter thins under its outer twigs), and of
    /// that, how much under conifers.
    pub fn under(&self, x: f64, z: f64) -> (f64, f64) {
        let (i, j) = ((x / self.cell).floor() as i64, (z / self.cell).floor() as i64);
        let (mut cover, mut needles) = (0f64, 0f64);
        for di in -1..=1 {
            for dj in -1..=1 {
                for c in self.cells.get(&(i + di, j + dj)).into_iter().flatten() {
                    let d = (c.x - x).hypot(c.z - z);
                    if d >= c.radius { continue; }
                    let t = ((c.radius - d) / (0.4 * c.radius)).min(1.);
                    let w = t * t * (3. - 2. * t);
                    if w > cover { needles = if c.needles { 1. } else { 0. }; }
                    cover = cover.max(w);
                }
            }
        }
        (cover, needles)
    }
}

/// The crowns over a landscape, added to as ecosystems are placed on it, before the render
/// reads them.
#[derive(Debug, Default)]
pub struct Slot(RwLock<Option<Arc<Canopy>>>, RwLock<Vec<Crown>>);

impl Slot {
    /// Add `crowns`, rebuilding the grid.
    pub fn add(&self, crowns: &[Crown]) {
        if crowns.is_empty() { return; }
        let mut all = self.1.write().unwrap();
        all.extend_from_slice(crowns);
        *self.0.write().unwrap() = Some(Arc::new(Canopy::new(&all)));
    }

    /// As `Canopy::under`; (0, 0) with no crowns.
    pub fn under(&self, x: f64, z: f64) -> (f64, f64) {
        match &*self.0.read().unwrap() {
            Some(c) => c.under(x, z),
            None => (0., 0.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_ground_is_under_the_crowns_it_was_given() {
        let slot = Slot::default();
        assert_eq!(slot.under(0., 0.), (0., 0.));
        slot.add(&[Crown { x: 0., z: 0., radius: 5., needles: true }]);
        slot.add(&[Crown { x: 20., z: 0., radius: 3., needles: false }]);
        assert_eq!(slot.under(1., 1.), (1., 1.));
        assert_eq!(slot.under(20., 1.), (1., 0.));
        // Fading at the edge, and nothing beyond.
        let (edge, _) = slot.under(4.5, 0.);
        assert!(edge > 0. && edge < 1., "{}", edge);
        assert_eq!(slot.under(10., 0.).0, 0.);
        assert_eq!(slot.under(-100., 50.).0, 0.);
    }
}
