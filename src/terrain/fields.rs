//! # What a landscape's simulation left, as fields for function graphs
//!
//! A landscape's surface layers (`material::surface_layers`, or any `mix` whose amount is a
//! graph) can be masked by what the simulation computed at each point: how deep the sediment
//! lies, where the glaciers are, how much water gathers. `SimFields` keeps each such field at a
//! simulation's cell corners and samples it smoothly in between (`scenery::sample`), so masks
//! follow the landscape without showing its grid; `graph::Field` lists them.
//!
//! The rock (`rock`, `hardness`) is not interpolated: the bedrock's structure (`geology`) and
//! how far uplift has carried it are, and the rock is looked up in the column at the point's
//! own height, so the boundaries between beds are sharp lines across the slopes, as far below
//! a cell as the ray's hit is precise.

use crate::graph::Field;
use crate::na::Vector3;
use crate::terrain::domain::Domain;
use crate::terrain::glacier::{slope_squared, GlacierParams};
use crate::terrain::scenery::sample;
use crate::terrain::geology::{Column, Rock, Structure};
use crate::terrain::simulation::Terrain;
use std::f64::consts::TAU;

/// Slopes gentler than this (rise over run) count as this steep in the wetness index, so flats
/// and lakes don't make it infinite.
const MIN_SLOPE: f64 = 1e-3;

/// A simulation's fields at its cell corners (each the mean of the cells meeting there).
pub struct SimFields {
    sediment: Vec<f64>,
    ice: Vec<f64>,
    lava: Vec<f64>,
    flow: Vec<f64>,
    wetness: Vec<f64>,
    convexity: Vec<f64>,
    snowline: Vec<f64>,
    /// The bedrock's structure (`Structure::to_array`) and exhumation.
    structure: [Vec<f64>; 9],
    exhumed: Vec<f64>,
    column: Column,
    /// The least and most of `flow` and `wetness` at the corners, to which their splines'
    /// overshoot is clamped.
    flow_range: (f64, f64),
    wetness_range: (f64, f64),
}

fn bounds(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| (lo.min(x), hi.max(x)))
}

impl SimFields {
    pub fn new(t: &Terrain) -> SimFields {
        let d = &t.domain;
        let n = d.len();
        let slope2 = slope_squared(d, &t.bed);
        let spacing = d.min_spacing();
        let glaciers = t.params.glacier.clone().unwrap_or_default();
        let cell = |i: usize, f: Field| -> f64 {
            match f {
                Field::Sediment => t.sediment[i],
                Field::Ice => t.ice[i],
                Field::Lava => t.lava[i],
                Field::Flow => t.drainage[i].max(1.).log10(),
                Field::Wetness => {
                    let a = t.drainage[i].max(spacing * spacing) / spacing;
                    (a / slope2[i].sqrt().max(MIN_SLOPE)).ln()
                },
                Field::Convexity => {
                    let links = d.links(i);
                    if links.is_empty() { return 0.; }
                    let mean = links.iter().map(|l| t.bed[l.to]).sum::<f64>() / links.len() as f64;
                    t.bed[i] - mean
                },
                Field::Snowline => snowline(t, &glaciers, i),
                _ => 0.,
            }
        };
        let corners = |f: Field| -> Vec<f64> {
            let values: Vec<f64> = (0..n).map(|i| cell(i, f)).collect();
            d.corner_cells.iter().map(|cells| cells.iter().map(|&c| values[c]).sum::<f64>() / cells.len() as f64).collect()
        };
        let (flow, wetness) = (corners(Field::Flow), corners(Field::Wetness));
        let average = |values: &dyn Fn(usize) -> f64| -> Vec<f64> {
            d.corner_cells.iter().map(|cells| cells.iter().map(|&c| values(c)).sum::<f64>() / cells.len() as f64).collect()
        };
        let structure = std::array::from_fn(|k| average(&|c| t.structure[c].to_array()[k]));
        SimFields {
            sediment: corners(Field::Sediment),
            ice: corners(Field::Ice),
            lava: corners(Field::Lava),
            flow_range: bounds(&flow),
            wetness_range: bounds(&wetness),
            flow,
            wetness,
            convexity: corners(Field::Convexity),
            snowline: corners(Field::Snowline),
            structure,
            exhumed: average(&|c| t.exhumed[c]),
            column: t.planet.column.clone(),
        }
    }

    /// The snowline at direction `d`, metres above sea level.
    pub fn snowline(&self, domain: &Domain, d: &Vector3<f64>) -> f64 {
        sample(domain, &self.snowline, d)
    }

    /// The rock exposed at direction `d` on `domain`, where the ground is `h` metres above sea
    /// level: sediment or lava where they are thicker than half a metre, else the bedrock at
    /// that height.
    pub fn rock(&self, domain: &Domain, d: &Vector3<f64>, h: f64) -> Rock {
        let s = |v: &[f64]| sample(domain, v, d);
        if s(&self.sediment) > 0.5 { return Rock::Sediment; }
        if s(&self.lava) > 0.5 { return Rock::Basalt; }
        let structure = Structure::from_array(std::array::from_fn(|k| s(&self.structure[k])));
        self.column.rock(&structure, structure.coordinate(h, s(&self.exhumed)))
    }

    /// Field `f` at direction `d` on `domain`, where the ground is `h` metres above sea level;
    /// None for those a simulation doesn't know (the climate's).
    pub fn at(&self, domain: &Domain, f: Field, d: &Vector3<f64>, h: f64) -> Option<f64> {
        let s = |v: &[f64]| sample(domain, v, d);
        Some(match f {
            // Thicknesses can't be negative, though the interpolation may overshoot.
            Field::Sediment => s(&self.sediment).max(0.),
            Field::Ice => s(&self.ice).max(0.),
            Field::Lava => s(&self.lava).max(0.),
            // Nor can the splines take these beyond what the simulation found anywhere.
            Field::Flow => s(&self.flow).max(self.flow_range.0).min(self.flow_range.1),
            Field::Wetness => s(&self.wetness).max(self.wetness_range.0).min(self.wetness_range.1),
            Field::Convexity => s(&self.convexity),
            Field::Elevation => h,
            Field::Snowline => h - s(&self.snowline),
            Field::Treeline => h - (s(&self.snowline) - TREELINE_BELOW_SNOWLINE),
            Field::Rock => self.rock(domain, d, h).index() as f64,
            Field::Hardness => -self.rock(domain, d, h).scores().0,
            Field::Temperature | Field::Precipitation => return None,
        })
    }
}

/// Without a climate model, the treeline lies this far below the snowline, metres (as
/// `scenery::Ground::treeline`).
pub const TREELINE_BELOW_SNOWLINE: f64 = 800.;

/// Cell `i`'s snowline at the terrain's time: its equilibrium line altitude, from the climate
/// model if there is one (today's climate is an interglacial, which the glacial cycle lowers
/// from), else from latitude.
fn snowline(t: &Terrain, g: &GlacierParams, i: usize) -> f64 {
    match &t.ela {
        Some(e) => e[i] + g.ela_amplitude * ((TAU * t.time / g.period).cos() - 1.),
        None => g.ela(t.latitude[i], t.time),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::simulation::TerrainParams;

    #[test]
    fn test_fields_follow_the_simulation() {
        let mut p = TerrainParams::default();
        p.planet.seed = 5;
        p.step = 500.;
        let mut t = Terrain::new(Domain::patch(40., 10., 24, 500., 6.371e6), p);
        t.run_to(2000.);
        let f = SimFields::new(&t);
        // Sampled at a cell's centre, the corner-averaged field is near the cells' values.
        let wettest = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i])
            .max_by(|&a, &b| t.drainage[a].total_cmp(&t.drainage[b])).unwrap();
        let driest = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i])
            .min_by(|&a, &b| t.drainage[a].total_cmp(&t.drainage[b])).unwrap();
        let flow = |i: usize| f.at(&t.domain, Field::Flow, &t.domain.dirs[i], 0.).unwrap();
        assert!(flow(wettest) > flow(driest), "{} vs {}", flow(wettest), flow(driest));
        // Thicknesses are never negative, and height above the snowline is the height less it.
        for i in 0..t.domain.len() {
            let d = &t.domain.dirs[i];
            assert!(f.at(&t.domain, Field::Sediment, d, 0.).unwrap() >= 0.);
            let line = f.snowline(&t.domain, d);
            assert!((f.at(&t.domain, Field::Snowline, d, 1000.).unwrap() - (1000. - line)).abs() < 1e-9);
        }
        assert_eq!(f.at(&t.domain, Field::Temperature, &t.domain.dirs[0], 0.), None);
        // At a cell's centre on bare rock, the rock is the simulation's own there.
        let bare: Vec<usize> = (0..t.domain.len()).filter(|&i| !t.domain.boundary[i] && t.sediment[i] == 0. && t.lava[i] == 0.).collect();
        assert!(!bare.is_empty());
        let agree = bare.iter().filter(|&&i| f.rock(&t.domain, &t.domain.dirs[i], t.bed[i]) == t.rock(i)).count();
        assert!(agree * 10 > bare.len() * 7, "{} of {}", agree, bare.len());
    }
}
