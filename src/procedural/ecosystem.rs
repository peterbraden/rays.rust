//! # Ecosystems: populations that compete for room
//!
//! An `ecosystem` places several populations (trees, shrubs, rocks, grass) over the same ground,
//! each by its own rule (`"where"`: slope, altitude, the landscape's fields, its biome), and in
//! the order listed, so the larger ones claim the ground first and the smaller ones grow in what
//! they leave, as Deussen et al. place plants by competition ("Realistic Modeling and Rendering
//! of Plant Ecosystems", SIGGRAPH '98): nothing grows on a trunk or under a boulder, and plants
//! under a canopy are thinned by its shade.
//!
//! Each placed thing claims a disc of `base` radius, where nothing smaller grows, and shades a
//! disc of `shade_radius`, where smaller things grow only `1 - shade` as thickly, fading to its
//! edge. `Occupancy` keeps them, and `free` says how much room is left at a point.

use std::collections::HashMap;

/// One thing placed: where it stands (x, z), the disc it takes, and the shade it casts.
#[derive(Clone, Copy, Debug)]
pub struct Claim {
    pub x: f64,
    pub z: f64,
    pub base: f64,
    pub shade_radius: f64,
    pub shade: f64,
}

/// The claims of everything placed so far, in a grid of cells as wide as the largest reach.
pub struct Occupancy {
    claims: Vec<Claim>,
    cells: HashMap<(i64, i64), Vec<usize>>,
    side: f64,
}

impl Occupancy {
    /// For claims reaching up to `reach` from where they stand.
    pub fn new(reach: f64) -> Occupancy {
        Occupancy { claims: vec![], cells: HashMap::new(), side: reach.max(1e-3) }
    }

    fn cell(&self, x: f64, z: f64) -> (i64, i64) {
        ((x / self.side).floor() as i64, (z / self.side).floor() as i64)
    }

    pub fn len(&self) -> usize { self.claims.len() }

    pub fn is_empty(&self) -> bool { self.claims.is_empty() }

    /// Adds claims; a claim reaching further than the grid's cells widens them.
    pub fn extend(&mut self, claims: impl IntoIterator<Item = Claim>) {
        let mut all: Vec<Claim> = std::mem::take(&mut self.claims);
        all.extend(claims);
        let reach = all.iter().map(|c| c.base.max(c.shade_radius)).fold(self.side, f64::max);
        self.side = reach;
        self.cells.clear();
        for (i, c) in all.iter().enumerate() {
            let k = ((c.x / reach).floor() as i64, (c.z / reach).floor() as i64);
            self.cells.entry(k).or_default().push(i);
        }
        self.claims = all;
    }

    /// How much room is left at (x, z), 0 to 1: 0 on anything's base, less under shade.
    pub fn free(&self, x: f64, z: f64) -> f64 {
        let (i, j) = self.cell(x, z);
        let mut room = 1.;
        for a in i - 1 ..= i + 1 {
            for b in j - 1 ..= j + 1 {
                let Some(list) = self.cells.get(&(a, b)) else { continue };
                for &k in list {
                    let c = &self.claims[k];
                    let d = (c.x - x).hypot(c.z - z);
                    if d < c.base {
                        return 0.;
                    }
                    if d < c.shade_radius && c.shade > 0. {
                        // Deepest near the trunk, fading to nothing at the canopy's edge.
                        let t = ((d - c.base) / (c.shade_radius - c.base).max(1e-9)).clamp(0., 1.);
                        room *= 1. - c.shade * (1. - t * t);
                    }
                }
            }
        }
        room
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_leave_room_around_them() {
        let mut o = Occupancy::new(1.);
        o.extend([Claim { x: 0., z: 0., base: 0.5, shade_radius: 4., shade: 0.8 }]);
        assert_eq!(o.free(0.2, 0.1), 0.);
        assert!((o.free(0.6, 0.) - 0.2).abs() < 0.05, "{}", o.free(0.6, 0.));
        assert!(o.free(2.5, 0.) > o.free(1., 0.));
        assert_eq!(o.free(10., 0.), 1.);
        // Two canopies shade more than one.
        o.extend([Claim { x: 3., z: 0., base: 0.5, shade_radius: 4., shade: 0.8 }]);
        assert!(o.free(1.5, 0.) < 0.2);
        assert_eq!(o.len(), 2);
    }
}
