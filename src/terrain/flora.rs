//! # Flora: which plants grow where
//!
//! A landscape's climate gives every point a biome (`climate::Biome`, from Köppen's main
//! groups: tundra, taiga, temperate forest, grassland, steppe, cold and hot desert, savanna,
//! rainforest), its edges wandering with the weather from year to year
//! (`scenery::Ground::biome`). This module says what grows there.
//!
//! `PLANTS` are the plants the library grows (`procedural::tree`'s species), each with its
//! stratum, its spacing, the steepest ground it holds, whether it needs wet ground, and its
//! colours. `COMMUNITIES` are what grows together: each biome has several, as a taiga is
//! spruce and fir in one valley, pine on heath in the next, and larch and birch beyond, each a
//! mix of plants with their abundances, perhaps stunted (the tundra's shrubs), over ground
//! plants of its own (`procedural::grass`'s species, how thick, in what condition).
//!
//! Which of a biome's communities grows where is decided region by region: each has a smooth
//! noise of its own, kilometres across (`REGION`), and holds the ground where its noise is
//! the highest of its biome's, fading into the next over a narrow ecotone (`ECOTONE`). So
//! every community has an equal share of its biome in expectation, in patches a few
//! kilometres across, and different valleys of one climate grow different woods.
//!
//! An `"ecosystem"` on a landscape with `"flora": true` takes its populations from here
//! (`populations`): the communities found about the camera (`terrain::object::flora`
//! evaluates their weights there) choose the plants, and each plant is a population of its
//! own, its rule (`"where"`) the sum over the communities it is in of their weight times its
//! abundance in them; times the ground it holds: no steeper than its `max_slope`, below the
//! treeline for trees and the snowline for shrubs, off ice, dunes and playas, sparse on bare
//! rock, and for plants of wet ground (willows, alders, mesquite) only where water gathers
//! (`wetness`). The populations are listed largest first, trees then shrubs then the ground's
//! plants, so that each grows in the room those before it leave (`procedural::ecosystem`).
//!
//! The library lacks some of the world's characteristic plants, so some stand in for others:
//! the black tupelo for the rainforest's canopy trees, the stone pine (and olive) for the
//! Mediterranean's pines. Its planted trees (Lombardy poplar, Italian cypress) aren't wild
//! anywhere, nor the cedar of Lebanon, whose mountains no biome here singles out.

use crate::terrain::climate::Biome;
use serde_json::{json, Map, Value};

/// Which layer of the vegetation a plant is in: trees are placed first, then shrubs, then the
/// ground's plants in the room they leave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stratum { Tree, Shrub }

/// A plant of the flora: one of `procedural::tree`'s species.
#[derive(Clone, Copy, Debug)]
pub struct Plant {
    pub species: &'static str,
    pub stratum: Stratum,
    /// How far apart, metres, where it grows as thickly as it can.
    pub spacing: f64,
    /// The steepest ground it holds, degrees.
    pub max_slope: f64,
    /// Whether it grows only where water gathers (`wetness`): by rivers and lakes.
    pub wet: bool,
    /// Its bark's (or a succulent's body's) albedo, and its leaves' (or spines') reflectance,
    /// linear RGB.
    pub bark: [f64; 3],
    pub leaves: [f64; 3],
}

use Biome::*;
use Stratum::*;

const fn plant(species: &'static str, stratum: Stratum, spacing: f64, max_slope: f64, bark: [f64; 3], leaves: [f64; 3]) -> Plant {
    Plant { species, stratum, spacing, max_slope, wet: false, bark, leaves }
}

const fn wet(p: Plant) -> Plant { Plant { wet: true, ..p } }

/// Needles: dark, blue-green; a larch's soft and pale.
const NEEDLES: [f64; 3] = [0.05, 0.08, 0.04];
const LARCH: [f64; 3] = [0.1, 0.16, 0.06];
/// Broad leaves: brighter, yellower green; the olive's and the gums' grey-green.
const BROADLEAF: [f64; 3] = [0.09, 0.15, 0.05];
const DARK_LEAF: [f64; 3] = [0.06, 0.13, 0.04];
const GREY_LEAF: [f64; 3] = [0.14, 0.17, 0.11];
const BROWN_BARK: [f64; 3] = [0.2, 0.15, 0.11];
const GREY_BARK: [f64; 3] = [0.36, 0.33, 0.29];
const WHITE_BARK: [f64; 3] = [0.6, 0.58, 0.54];
const CACTUS: [f64; 3] = [0.09, 0.15, 0.07];
const SPINES: [f64; 3] = [0.62, 0.56, 0.44];

/// The plants.
pub const PLANTS: &[Plant] = &[
    // Trees.
    plant("norway-spruce", Tree, 7., 40., BROWN_BARK, NEEDLES),
    plant("balsam-fir", Tree, 6., 40., GREY_BARK, NEEDLES),
    plant("scots-pine", Tree, 8., 40., [0.3, 0.17, 0.1], NEEDLES),
    plant("european-larch", Tree, 7., 40., BROWN_BARK, LARCH),
    plant("ponderosa-pine", Tree, 10., 35., [0.33, 0.2, 0.12], NEEDLES),
    plant("pinyon-pine", Tree, 7., 35., GREY_BARK, NEEDLES),
    plant("stone-pine", Tree, 14., 30., [0.28, 0.2, 0.14], [0.08, 0.11, 0.04]),
    plant("quaking-aspen", Tree, 6., 35., [0.5, 0.5, 0.45], BROADLEAF),
    plant("silver-birch", Tree, 6., 35., WHITE_BARK, BROADLEAF),
    plant("european-beech", Tree, 10., 35., [0.4, 0.39, 0.36], BROADLEAF),
    plant("english-oak", Tree, 12., 30., [0.2, 0.18, 0.15], DARK_LEAF),
    plant("california-black-oak", Tree, 10., 30., [0.18, 0.16, 0.13], BROADLEAF),
    plant("sugar-maple", Tree, 9., 35., [0.28, 0.24, 0.2], BROADLEAF),
    plant("black-tupelo", Tree, 9., 30., [0.2, 0.17, 0.14], DARK_LEAF),
    wet(plant("weeping-willow", Tree, 10., 15., BROWN_BARK, [0.11, 0.17, 0.06])),
    wet(plant("grey-alder", Tree, 6., 25., GREY_BARK, DARK_LEAF)),
    plant("olive", Tree, 8., 30., [0.3, 0.28, 0.24], GREY_LEAF),
    plant("eucalyptus", Tree, 10., 30., [0.55, 0.5, 0.42], GREY_LEAF),
    plant("acacia", Tree, 12., 25., [0.22, 0.18, 0.14], [0.1, 0.14, 0.05]),
    plant("baobab", Tree, 25., 20., [0.42, 0.37, 0.32], BROADLEAF),
    plant("palm", Tree, 8., 30., [0.3, 0.26, 0.2], [0.07, 0.13, 0.04]),
    plant("kapok", Tree, 20., 30., [0.45, 0.42, 0.36], DARK_LEAF),
    plant("joshua-tree", Tree, 9., 25., [0.25, 0.2, 0.15], [0.11, 0.15, 0.07]),
    plant("utah-juniper", Tree, 8., 35., [0.3, 0.27, 0.23], [0.12, 0.14, 0.1]),
    plant("saguaro", Tree, 10., 30., CACTUS, SPINES),
    // Shrubs, and the small trees of the understory.
    plant("tree-fern", Shrub, 4., 40., [0.18, 0.14, 0.1], [0.07, 0.15, 0.04]),
    wet(plant("mesquite", Shrub, 6., 25., [0.25, 0.2, 0.16], [0.11, 0.16, 0.06])),
    plant("mountain-pine", Shrub, 3., 45., GREY_BARK, NEEDLES),
    plant("dwarf-birch", Shrub, 1.5, 40., [0.3, 0.2, 0.15], [0.1, 0.15, 0.05]),
    plant("juniper", Shrub, 4., 40., [0.3, 0.27, 0.23], [0.12, 0.14, 0.1]),
    plant("hazel", Shrub, 4., 35., [0.3, 0.25, 0.2], BROADLEAF),
    plant("hawthorn", Shrub, 5., 35., [0.25, 0.22, 0.18], [0.08, 0.14, 0.05]),
    plant("shrub", Shrub, 3., 40., BROWN_BARK, BROADLEAF),
    plant("thornbush", Shrub, 4., 35., [0.24, 0.2, 0.16], [0.1, 0.14, 0.05]),
    plant("creosote-bush", Shrub, 4., 30., GREY_BARK, [0.13, 0.16, 0.07]),
    plant("sagebrush", Shrub, 2.5, 35., GREY_BARK, [0.3, 0.33, 0.26]),
    plant("rabbitbrush", Shrub, 2.5, 35., GREY_BARK, [0.22, 0.27, 0.1]),
    plant("ocotillo", Shrub, 5., 35., GREY_BARK, [0.1, 0.2, 0.05]),
    plant("barrel-cactus", Shrub, 3., 40., [0.1, 0.17, 0.07], [0.5, 0.24, 0.14]),
    plant("prickly-pear", Shrub, 3., 35., [0.13, 0.21, 0.07], SPINES),
    plant("agave", Shrub, 3., 40., [0.2, 0.28, 0.24], [0.12, 0.1, 0.08]),
];

/// Plants that grow together, in one of a biome's regions.
#[derive(Clone, Copy, Debug)]
pub struct Community {
    pub name: &'static str,
    pub biome: Biome,
    /// Its plants (`PLANTS`' species) and their abundance, 0 to 1.
    pub plants: &'static [(&'static str, f64)],
    /// How tall its shrubs grow, metres, if stunted (by cold and wind, in the tundra).
    pub stunted: Option<f64>,
    /// Its ground's plants (`procedural::grass`'s species), how thickly (as a `grass`'s
    /// `density`; none if 0) and in what condition.
    pub ground: &'static [&'static str],
    pub density: f64,
    pub conditions: &'static str,
    /// How much closer than their own spacing its plants stand (1; less in a rainforest's
    /// crowded, closed canopy).
    pub crowding: f64,
}

const fn community(name: &'static str, biome: Biome, plants: &'static [(&'static str, f64)], ground: &'static [&'static str], density: f64, conditions: &'static str) -> Community {
    Community { name, biome, plants, stunted: None, ground, density, conditions, crowding: 1. }
}

const fn crowded(c: Community, crowding: f64) -> Community { Community { crowding, ..c } }

const fn stunted(c: Community, height: f64) -> Community { Community { stunted: Some(height), ..c } }

/// The communities, by biome.
pub const COMMUNITIES: &[Community] = &[
    // Tundra: dwarf shrubs, meadows, and krummholz pines where the trees give out.
    stunted(community("dwarf-shrub heath", Tundra, &[("dwarf-birch", 0.5), ("juniper", 0.2)],
        &["heather", "cottongrass", "fescue"], 250., "autumn"), 0.6),
    stunted(community("alpine meadow", Tundra, &[("dwarf-birch", 0.1)],
        &["tussock", "fescue", "buttercup", "daisy"], 300., "summer"), 0.5),
    stunted(community("krummholz", Tundra, &[("mountain-pine", 0.5), ("juniper", 0.3)],
        &["heather", "fescue"], 200., "summer"), 1.5),
    // Taiga.
    community("spruce-fir forest", Taiga, &[("norway-spruce", 0.7), ("balsam-fir", 0.4), ("silver-birch", 0.1), ("juniper", 0.1), ("grey-alder", 0.5)],
        &["fescue", "bracken", "litter"], 200., "summer"),
    community("pine heath", Taiga, &[("scots-pine", 0.7), ("silver-birch", 0.2), ("juniper", 0.3), ("grey-alder", 0.5)],
        &["heather", "fescue", "litter"], 220., "summer"),
    community("larch and birch", Taiga, &[("european-larch", 0.6), ("silver-birch", 0.4), ("dwarf-birch", 0.2), ("grey-alder", 0.5)],
        &["fescue", "fireweed", "sedge"], 220., "summer"),
    // Temperate forest.
    community("beech wood", TemperateForest, &[("european-beech", 0.8), ("english-oak", 0.1), ("hazel", 0.1), ("weeping-willow", 0.4), ("grey-alder", 0.4)],
        &["litter", "bracken"], 150., "summer"),
    community("oak and hazel", TemperateForest, &[("english-oak", 0.6), ("silver-birch", 0.15), ("quaking-aspen", 0.1), ("hazel", 0.4), ("hawthorn", 0.2), ("weeping-willow", 0.5)],
        &["meadow", "bent", "bracken", "buttercup"], 300., "summer"),
    community("maple and tupelo", TemperateForest, &[("sugar-maple", 0.6), ("black-tupelo", 0.3), ("california-black-oak", 0.2), ("quaking-aspen", 0.1), ("shrub", 0.2), ("grey-alder", 0.4)],
        &["litter", "bracken", "meadow"], 200., "summer"),
    community("montane mixed forest", TemperateForest, &[("norway-spruce", 0.4), ("european-beech", 0.3), ("european-larch", 0.1), ("scots-pine", 0.1), ("hazel", 0.1)],
        &["litter", "fescue", "bracken"], 200., "summer"),
    // Grassland.
    community("flower meadow", Grassland, &[("hawthorn", 0.1), ("english-oak", 0.03), ("weeping-willow", 0.5)],
        &["meadow", "timothy", "daisy", "oxeye", "buttercup", "clover", "dandelion"], 350., "wild"),
    community("parkland", Grassland, &[("english-oak", 0.12), ("quaking-aspen", 0.06), ("silver-birch", 0.05), ("hawthorn", 0.15), ("grey-alder", 0.4)],
        &["meadow", "bent", "oxeye", "red_clover"], 300., "summer"),
    community("tall-grass prairie", Grassland, &[("california-black-oak", 0.03), ("weeping-willow", 0.4)],
        &["tall", "wild_oat", "bent"], 300., "summer"),
    // Steppe.
    community("sagebrush steppe", Steppe, &[("sagebrush", 0.6), ("rabbitbrush", 0.2), ("juniper", 0.05), ("weeping-willow", 0.3)],
        &["fescue", "tussock"], 180., "drought"),
    community("pinyon-juniper woodland", Steppe, &[("pinyon-pine", 0.25), ("utah-juniper", 0.25), ("sagebrush", 0.2)],
        &["tussock", "fescue"], 120., "drought"),
    community("ponderosa parkland", Steppe, &[("ponderosa-pine", 0.25), ("rabbitbrush", 0.1), ("weeping-willow", 0.3)],
        &["fescue", "wild_oat"], 200., "drought"),
    community("Mediterranean scrub", Steppe, &[("olive", 0.15), ("stone-pine", 0.06), ("shrub", 0.3), ("prickly-pear", 0.05)],
        &["wild_oat", "poppy", "fescue"], 200., "drought"),
    // Cold desert.
    community("sagebrush desert", ColdDesert, &[("sagebrush", 0.7), ("rabbitbrush", 0.3), ("utah-juniper", 0.05)],
        &["tussock"], 40., "drought"),
    community("joshua woodland", ColdDesert, &[("joshua-tree", 0.2), ("sagebrush", 0.2), ("rabbitbrush", 0.1), ("prickly-pear", 0.05)],
        &["tussock"], 30., "drought"),
    community("pinyon desert", ColdDesert, &[("pinyon-pine", 0.1), ("utah-juniper", 0.2), ("sagebrush", 0.3)],
        &["tussock"], 30., "drought"),
    // Hot desert: mesquite in the washes.
    community("saguaro desert", HotDesert, &[("saguaro", 0.25), ("ocotillo", 0.15), ("prickly-pear", 0.2), ("barrel-cactus", 0.15), ("creosote-bush", 0.3), ("mesquite", 0.4)],
        &[], 0., "drought"),
    community("creosote flats", HotDesert, &[("creosote-bush", 0.6), ("barrel-cactus", 0.05), ("mesquite", 0.4)],
        &[], 0., "drought"),
    community("agave slopes", HotDesert, &[("agave", 0.3), ("ocotillo", 0.2), ("barrel-cactus", 0.15), ("prickly-pear", 0.1), ("creosote-bush", 0.1)],
        &[], 0., "drought"),
    // Savanna.
    community("acacia savanna", Savanna, &[("acacia", 0.15), ("thornbush", 0.25)],
        &["savanna"], 300., "drought"),
    community("baobab savanna", Savanna, &[("baobab", 0.04), ("acacia", 0.08), ("thornbush", 0.15)],
        &["savanna"], 300., "drought"),
    community("eucalypt woodland", Savanna, &[("eucalyptus", 0.3), ("thornbush", 0.1), ("acacia", 0.03)],
        &["savanna", "tussock"], 250., "drought"),
    community("palm savanna", Savanna, &[("palm", 0.15), ("thornbush", 0.1)],
        &["savanna"], 300., "summer"),
    // Rainforest.
    // Rainforest: closed canopies, crowded, in layers: emergents, canopy, tree ferns, shrubs.
    crowded(community("lowland rainforest", Rainforest, &[("kapok", 0.15), ("black-tupelo", 0.8), ("palm", 0.3), ("tree-fern", 0.5), ("shrub", 0.6)],
        &["bracken", "litter"], 150., "lush"), 0.6),
    crowded(community("cloud forest", Rainforest, &[("tree-fern", 0.8), ("black-tupelo", 0.5), ("shrub", 0.5)],
        &["bracken", "sedge"], 200., "lush"), 0.6),
    crowded(community("palm forest", Rainforest, &[("palm", 0.9), ("tree-fern", 0.4), ("black-tupelo", 0.3), ("shrub", 0.4)],
        &["bracken", "litter"], 150., "lush"), 0.6),
];

/// How wide a community's regions are, metres: the scale of each one's noise.
const REGION: f64 = 4000.;

/// How far one community's noise must lead another's for it to hold the ground alone: the
/// ecotone between them, a few percent of a region across.
const ECOTONE: f64 = 0.04;

/// Whether `species` is a conifer, dropping needles (some are drawn with oval leaves).
pub fn conifer(species: &str) -> bool {
    matches!(species, "norway-spruce" | "balsam-fir" | "scots-pine" | "european-larch" | "ponderosa-pine" | "pinyon-pine"
        | "stone-pine" | "utah-juniper" | "mountain-pine" | "juniper" | "italian-cypress" | "cedar-of-lebanon")
}

/// The plant `species` (`PLANTS`).
pub fn plant_named(species: &str) -> Option<&'static Plant> {
    PLANTS.iter().find(|p| p.species == species)
}

/// The communities of `biome`.
pub fn communities(biome: Biome) -> Vec<&'static Community> {
    COMMUNITIES.iter().filter(|c| c.biome == biome).collect()
}

/// The noise that decides where the `k`th community of `biome` grows.
fn region_noise(biome: Biome, k: usize) -> Value {
    let b = Biome::ALL.iter().position(|&x| x == biome).unwrap_or(0) as u64;
    json!({"node": "fractal", "scale": REGION, "smallest": REGION / 8., "seed": 7000 + 37 * b + k as u64})
}

/// How far the `k`th of the `n` communities of `biome` leads the others, from 0 (well behind
/// one of them) to 1 (well ahead of all), through 0.5 where it ties with the next.
fn lead(biome: Biome, k: usize, n: usize) -> Value {
    let others: Vec<Value> = (0..n).filter(|&j| j != k).map(|j| region_noise(biome, j)).collect();
    let rivals = if others.len() == 1 { others[0].clone() } else { json!({"node": "max", "inputs": others}) };
    let ahead = json!({"node": "subtract", "a": region_noise(biome, k), "b": rivals});
    json!({"node": "smoothstep", "input": ahead, "from": -ECOTONE, "to": ECOTONE})
}

/// How much of the ground at a point `c` holds, 0 to 1: a graph, its biome there times its
/// region's (the ground where its noise leads its biome's other communities', its share of
/// their leads, so that they add to one in the ecotones between them).
pub fn weight(c: &Community) -> Value {
    let biome = json!({"node": "biome", "is": c.biome.name()});
    let all = communities(c.biome);
    if all.len() < 2 { return biome; }
    let n = all.len();
    let k = all.iter().position(|x| x.name == c.name).expect("a community of its biome");
    // The leader's lead is at least a half, so the sum is too.
    let share = json!({"node": "expr", "expr": "a / b", "range": [0, 1], "inputs": {
        "a": lead(c.biome, k, n), "b": {"node": "add", "inputs": (0..n).map(|j| lead(c.biome, j, n)).collect::<Vec<_>>()}}});
    json!({"node": "multiply", "inputs": [biome, share]})
}

/// The woods of community `c`, seen from afar: how much of the ground its trees' crowns cover
/// (the sum of its trees' abundances, at most 1; not its trees of wet ground, which line the
/// rivers, nor the desert's saguaros and Joshua trees, which make no canopy), and how much of
/// that is conifers'.
pub fn woods(c: &Community) -> (f64, f64) {
    let trees: Vec<(&Plant, f64)> = c.plants.iter()
        .filter_map(|(s, a)| plant_named(s).map(|p| (p, *a)))
        .filter(|(p, _)| p.stratum == Tree && !p.wet && !matches!(p.species, "saguaro" | "joshua-tree"))
        .collect();
    let total: f64 = trees.iter().map(|(_, a)| a).sum();
    let needles: f64 = trees.iter().filter(|(p, _)| conifer(p.species)).map(|(_, a)| a).sum();
    (total.min(1.), if total > 0. { needles / total } else { 0. })
}

/// The communities' region noises (`region_noise`), parsed once, for their weights at many
/// points: as `weight` without its biome, each noise evaluated once rather than in every
/// community's graph.
pub struct Regions {
    noises: Vec<(Biome, Vec<crate::graph::Graph>)>,
}

impl Default for Regions {
    fn default() -> Regions {
        Regions { noises: Biome::ALL.iter().map(|&b| {
            let n = communities(b).len();
            (b, (0..n).map(|k| crate::graph::Graph::parse(&region_noise(b, k)).expect("a region's noise")).collect())
        }).collect() }
    }
}

impl Regions {
    /// The weight of each community of `biome` at `c`'s point, given that the biome is there.
    pub fn weights(&self, biome: Biome, c: &crate::graph::Context) -> Vec<(&'static Community, f64)> {
        let all = communities(biome);
        if all.len() < 2 { return all.into_iter().map(|x| (x, 1.)).collect(); }
        let graphs = &self.noises.iter().find(|(b, _)| *b == biome).expect("every biome's noises").1;
        let f: Vec<f64> = graphs.iter().map(|g| g.eval(c)).collect();
        let lead = |k: usize| {
            let rivals = (0..f.len()).filter(|&j| j != k).map(|j| f[j]).fold(f64::NEG_INFINITY, f64::max);
            let t = ((f[k] - rivals + ECOTONE) / (2. * ECOTONE)).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        let leads: Vec<f64> = (0..f.len()).map(lead).collect();
        let total: f64 = leads.iter().sum();
        all.into_iter().zip(leads).map(|(x, l)| (x, l / total)).collect()
    }

    /// The woods at `c`'s point, given its `biome`: their communities' `woods`, by weight.
    pub fn woods(&self, biome: Biome, c: &crate::graph::Context) -> (f64, f64) {
        let (mut cover, mut needles) = (0., 0.);
        for (x, w) in self.weights(biome, c) {
            let (v, n) = woods(x);
            cover += w * v;
            needles += w * v * n;
        }
        (cover, if cover > 0. { needles / cover } else { 0. })
    }
}

/// The share of each community among the weights found at points about the camera (each
/// sample a community's weight there), most first; those under 2% left out.
pub fn shares(samples: &[Vec<(&'static Community, f64)>]) -> Vec<(&'static Community, f64)> {
    let n = samples.len().max(1) as f64;
    let mut out: Vec<(&'static Community, f64)> = COMMUNITIES.iter().map(|c| {
        let total: f64 = samples.iter().flat_map(|s| s.iter()).filter(|(x, _)| x.name == c.name).map(|(_, w)| w).sum();
        (c, total / n)
    }).filter(|&(_, s)| s >= 0.02).collect();
    out.sort_by(|a, b| b.1.total_cmp(&a.1));
    out
}

/// Off what nothing roots in (glacier ice, drifting sand, playas), and sparse on bare rock: a
/// fifth as thick where there is no soil, as plants hold on only in its cracks.
fn rooted() -> [Value; 4] {
    let none_on = |field: &str, from: f64, to: f64| json!({"node": "field", "field": field, "from": from, "to": to});
    let soil = json!({"node": "add", "inputs": [0.2, {"node": "multiply", "inputs": [0.8, none_on("sediment", 0.05, 0.5)]}]});
    [none_on("ice", 2., 0.5), none_on("sand", 2., 0.5), none_on("playa", 0.5, 0.1), soil]
}

/// `terms` added, as a graph.
fn sum(terms: Vec<Value>) -> Value {
    if terms.len() == 1 { terms.into_iter().next().unwrap() } else { json!({"node": "add", "inputs": terms}) }
}

/// The populations of an ecosystem with `"flora"` for the communities `present` about the
/// camera, and the materials they name: a forest of each plant of those communities (each
/// stunted height a population of its own), trees then shrubs, and a grass of each
/// community's ground plants. `strata` picks which: "trees", "shrubs" and "ground". Trees grow
/// `reach` metres out from under the camera, shrubs half as far, each thinning out over the
/// last third of it, so the woods have no edge to see.
pub fn populations(present: &[&'static Community], strata: &[&str], seed: u64, reach: f64) -> (Vec<Value>, Map<String, Value>) {
    let mut out = vec![];
    let mut materials = Map::new();
    for (stratum, name) in [(Tree, "trees"), (Shrub, "shrubs")] {
        if !strata.contains(&name) { continue; }
        for p in PLANTS.iter().filter(|p| p.stratum == stratum) {
            // The communities it grows in, by how tall it grows there (stunted shrubs).
            let mut heights: Vec<Option<f64>> = vec![];
            for c in present {
                let h = if stratum == Shrub { c.stunted } else { None };
                if c.plants.iter().any(|(s, _)| *s == p.species) && !heights.contains(&h) { heights.push(h); }
            }
            for height in heights {
                let growing = || present.iter()
                    .filter(|c| (if stratum == Shrub { c.stunted } else { None }) == height)
                    .filter(|c| c.plants.iter().any(|(s, _)| *s == p.species));
                // As close as the most crowded of them.
                let crowding = growing().map(|c| c.crowding).fold(1., f64::min);
                let terms: Vec<Value> = present.iter()
                    .filter(|c| (if stratum == Shrub { c.stunted } else { None }) == height)
                    .filter_map(|c| c.plants.iter().find(|(s, _)| *s == p.species).map(|&(_, a)| (c, a)))
                    .map(|(c, a)| json!({"node": "multiply", "inputs": [weight(c), a]}))
                    .collect();
                let mut rule = vec![sum(terms), json!({"node": "slope", "from": p.max_slope, "to": p.max_slope - 10.})];
                rule.extend(rooted());
                // Trees stop at the treeline (the biome's edge, which wanders, besides); shrubs
                // stop below the snow.
                rule.push(match stratum {
                    Tree => json!({"node": "field", "field": "treeline", "from": 50, "to": -150}),
                    Shrub => json!({"node": "field", "field": "snowline", "from": -100, "to": -300}),
                });
                if p.wet { rule.push(json!({"node": "field", "field": "wetness", "from": 9, "to": 12})); }
                let mut species = json!({"species": p.species, "weight": 1});
                if let Some(h) = height { species["params"] = json!({"scale": h, "scale_v": 0.3 * h}); }
                let (b, l) = (format!("FLORA_{}_BARK", p.species), format!("FLORA_{}_LEAVES", p.species));
                materials.insert(b.clone(), json!({"type": "lambertian", "albedo": p.bark}));
                let succulent = matches!(p.species, "saguaro" | "barrel-cactus" | "prickly-pear" | "agave");
                materials.insert(l.clone(), if succulent {
                    json!({"type": "lambertian", "albedo": p.leaves})
                } else {
                    json!({"type": "leaf", "reflectance": p.leaves, "transmittance": p.leaves.map(|c| c * 0.5), "roughness": 0.4})
                });
                species["bark"] = Value::from(b);
                species["leaves"] = Value::from(l);
                let reach = match stratum { Tree => reach, Shrub => 0.5 * reach };
                // Stunted, they crowd closer, in proportion to how much they shrank.
                let spacing = crowding * match height { Some(h) => p.spacing * (h / 2.).clamp(0.3, 1.), None => p.spacing };
                // In clumps rather than evenly spread: groves and thickets, more so for shrubs.
                let clustering = match stratum { Tree => 0.25, Shrub => 0.45 };
                out.push(json!({"forest": {
                    "reach": reach, "fade": 0.3, "spacing": spacing, "clustering": clustering, "clearings": 0.1, "patch_size": 30. * p.spacing, "patchiness": 0.5,
                    "max_slope": p.max_slope, "variants": 3, "lod": true,
                    "species": [species],
                    "where": {"node": "multiply", "inputs": rule},
                }}));
            }
        }
    }
    if strata.contains(&"ground") {
        for c in present.iter().filter(|c| c.density > 0. && !c.ground.is_empty()) {
            let mut rule = vec![weight(c), json!({"node": "slope", "from": 40, "to": 28})];
            rule.extend(rooted());
            rule.push(json!({"node": "field", "field": "snowline", "from": 0, "to": -200}));
            out.push(json!({"grass": {
                "reach": 30, "density": c.density, "species": c.ground, "conditions": c.conditions,
                "seed": seed.wrapping_add(out.len() as u64),
                "where": {"node": "multiply", "inputs": rule},
            }}));
        }
    }
    (out, materials)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Context, Fields, Graph};
    use crate::na::Vector3;

    /// Every plant is a species the library grows, and every community's plants are plants of
    /// the flora and its ground's grasses the library has; every biome but ice has
    /// communities, most of them several; and every plant grows somewhere.
    #[test]
    fn test_the_flora_is_the_librarys_and_covers_the_biomes() {
        for p in PLANTS {
            assert!(crate::procedural::tree::SPECIES.contains(&p.species), "{} isn't a tree species", p.species);
            assert!(COMMUNITIES.iter().any(|c| c.plants.iter().any(|(s, _)| *s == p.species)), "{} grows nowhere", p.species);
        }
        for c in COMMUNITIES {
            for (s, a) in c.plants {
                assert!(plant_named(s).is_some(), "{}: no plant {}", c.name, s);
                assert!(*a > 0. && *a <= 1., "{}: {}", c.name, s);
            }
            for s in c.ground {
                assert!(crate::procedural::grass::SPECIES_PRESETS.contains(s), "{}: {} isn't a grass species", c.name, s);
            }
            assert!(crate::procedural::grass::CONDITIONS_PRESETS.contains(&c.conditions), "{}", c.conditions);
            Graph::parse(&weight(c)).unwrap();
        }
        for b in Biome::ALL {
            let n = communities(b).len();
            assert_eq!(b == Biome::Ice, n == 0, "{:?}", b);
            assert!(b == Biome::Ice || n >= 3, "{:?} has {} communities", b, n);
        }
        assert!(PLANTS.len() >= 40);
    }

    /// Ground of one biome, everywhere.
    struct All(Biome);
    impl Fields for All {
        fn field(&self, _: crate::graph::Field, _: &Vector3<f64>) -> Option<f64> { None }
        fn biome(&self, _: &Vector3<f64>) -> Option<Biome> { Some(self.0) }
    }

    /// A biome's communities share it: their weights add to one everywhere, each holds
    /// a fair share of a wide area, in patches, and each holds ground alone somewhere.
    #[test]
    fn test_communities_share_their_biome_by_region() {
        let fields = All(Biome::TemperateForest);
        let cs = communities(Biome::TemperateForest);
        let graphs: Vec<Graph> = cs.iter().map(|c| Graph::parse(&weight(c)).unwrap()).collect();
        let n = 60;
        let mut totals = vec![0.; cs.len()];
        let mut alone = vec![false; cs.len()];
        for i in 0..n {
            for j in 0..n {
                let p = Vector3::new(i as f64 * 1000., 0., j as f64 * 1000.);
                let c = Context::new(p, Vector3::y(), 0.).with_fields(Some(&fields));
                let w: Vec<f64> = graphs.iter().map(|g| g.eval(&c)).collect();
                let sum: f64 = w.iter().sum();
                assert!((sum - 1.).abs() < 1e-9, "weights {:?} at {:?}", w, p);
                for (k, &x) in w.iter().enumerate() {
                    totals[k] += x;
                    if x > 0.99 { alone[k] = true; }
                }
            }
        }
        for (k, t) in totals.iter().enumerate() {
            let share = t / (n * n) as f64;
            assert!(share > 0.5 / cs.len() as f64 && share < 2. / cs.len() as f64, "{}: {}", cs[k].name, share);
        }
        assert!(alone.iter().all(|&a| a), "{:?}", alone);
        // Nothing of another biome.
        let elsewhere = All(Biome::Taiga);
        let c = Context::new(Vector3::zeros(), Vector3::y(), 0.).with_fields(Some(&elsewhere));
        assert!(graphs.iter().all(|g| g.eval(&c) == 0.));
    }

    /// The fast weights are the graphs' (as the trees are placed by), and the woods follow
    /// the communities: thick in a beech wood, conifers in a spruce-fir forest, open on a
    /// meadow, none in a desert of cacti.
    #[test]
    fn test_regions_are_the_weights_graphs() {
        let regions = Regions::default();
        for b in [Biome::Taiga, Biome::Savanna, Biome::Tundra] {
            let fields = All(b);
            let graphs: Vec<(&Community, Graph)> = communities(b).into_iter().map(|c| (c, Graph::parse(&weight(c)).unwrap())).collect();
            for i in 0..200 {
                let p = Vector3::new(i as f64 * 317.3, 0., (i * i) as f64 * 41.7 % 30000.);
                let c = Context::new(p, Vector3::y(), 0.).with_fields(Some(&fields));
                for ((x, w), (y, g)) in regions.weights(b, &c).iter().zip(&graphs) {
                    assert_eq!(x.name, y.name);
                    assert!((w - g.eval(&c)).abs() < 1e-9, "{} at {:?}", x.name, p);
                }
            }
        }
        let named = |n: &str| COMMUNITIES.iter().find(|c| c.name == n).unwrap();
        assert!(woods(named("beech wood")).0 >= 0.8 && woods(named("beech wood")).1 == 0.);
        assert!(woods(named("spruce-fir forest")).0 == 1. && woods(named("spruce-fir forest")).1 > 0.8);
        assert!(woods(named("flower meadow")).0 < 0.1);
        assert_eq!(woods(named("saguaro desert")).0, 0.);
    }

    /// The populations are those of the communities present only, trees before shrubs before
    /// the ground, each rule parses, and a plant of two communities is one population (two
    /// if stunted in one of them).
    #[test]
    fn test_populations_follow_the_communities_present() {
        let named = |n: &str| COMMUNITIES.iter().find(|c| c.name == n).unwrap();
        let present = [named("spruce-fir forest"), named("pine heath"), named("dwarf-shrub heath")];
        let (pops, materials) = populations(&present, &["trees", "shrubs", "ground"], 1, 300.);
        let kinds: Vec<&str> = pops.iter().map(|p| if p.get("forest").is_some() { "forest" } else { "grass" }).collect();
        let first_grass = kinds.iter().position(|&k| k == "grass").unwrap();
        assert!(kinds[..first_grass].iter().all(|&k| k == "forest") && kinds[first_grass..].iter().all(|&k| k == "grass"));
        assert_eq!(kinds.len() - first_grass, 3);
        let names: Vec<String> = pops.iter().filter_map(|p| p["forest"]["species"][0]["species"].as_str().map(String::from)).collect();
        for n in ["norway-spruce", "balsam-fir", "scots-pine", "silver-birch", "grey-alder", "dwarf-birch"] {
            assert!(names.contains(&n.to_string()), "{} missing from {:?}", n, names);
        }
        assert!(!names.contains(&"european-larch".to_string()) && !names.contains(&"saguaro".to_string()));
        assert_eq!(names.iter().filter(|n| *n == "silver-birch").count(), 1);
        // The heath's junipers are dwarfs, the forests' not.
        let junipers: Vec<&Value> = pops.iter().filter(|p| p["forest"]["species"][0]["species"] == "juniper").collect();
        assert_eq!(junipers.len(), 2);
        assert_eq!(junipers.iter().filter(|p| p["forest"]["species"][0]["params"]["scale"] == json!(0.6)).count(), 1);
        for p in &pops {
            let spec = p.get("forest").or(p.get("grass")).unwrap();
            Graph::parse(&spec["where"]).unwrap();
            if let Some(b) = spec["species"][0]["bark"].as_str() { assert!(materials.contains_key(b)); }
        }
        let (trees_only, _) = populations(&present, &["trees"], 1, 300.);
        assert!(trees_only.iter().all(|p| p.get("forest").is_some()) && trees_only.len() < pops.len());
    }
}
