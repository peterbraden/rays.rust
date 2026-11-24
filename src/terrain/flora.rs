//! # Flora: which plants grow in which biome
//!
//! A landscape's climate gives every point a biome (`climate::Biome`, from Köppen's main
//! groups: tundra, taiga, temperate forest, grassland, steppe, cold and hot desert, savanna,
//! rainforest), its edges wandering with the weather from year to year
//! (`scenery::Ground::biome`). This module says what grows there: `FLORA`, each plant the
//! library has (`procedural::tree`'s species, `procedural::grass`'s) with its stratum, its
//! spacing, the steepest ground it holds, whether it needs wet ground, and how abundant it is in
//! each biome it belongs to; and `GROUND`, each biome's grass, flowers and ferns.
//!
//! An `"ecosystem"` on a landscape with `"flora": true` takes its populations from here
//! (`populations`): the biomes found about the camera (`survey`) choose the plants, and each
//! plant is a population of its own, so that where one biome gives way to the next its plants
//! give way to the next's. A plant's rule (`"where"`) is its abundance in the biome at each
//! point, below the treeline for trees (shrubs a little above it), on ground no steeper than it
//! holds, off ice, dunes and playas and sparse on bare rock, and for plants of wet ground (willows) only where water
//! gathers (`wetness`). The populations are listed largest first, trees then shrubs then the
//! ground's plants, so that each grows in the room those before it leave
//! (`procedural::ecosystem`).
//!
//! The table has the species the library has, so it stands some in for others: the stone pine's
//! umbrella crown for the savanna's acacias, the palm and the black tupelo for the rainforest's
//! trees. The library's planted trees (Lombardy poplar, Italian cypress) aren't in it, nor the
//! cedar of Lebanon, whose mountains no biome here singles out.

use crate::terrain::climate::Biome;
use serde_json::{json, Map, Value};

/// Biomes with an abundance in each.
type Abundances = Vec<(Biome, f64)>;

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
    /// Its abundance in each biome it belongs to, 0 to 1.
    pub biomes: &'static [(Biome, f64)],
    /// Its bark's (or a succulent's body's) albedo, and its leaves' (or spines') reflectance,
    /// linear RGB; None for the tree's default.
    pub bark: Option<[f64; 3]>,
    pub leaves: Option<[f64; 3]>,
}

use Biome::*;
use Stratum::*;

const fn plant(species: &'static str, stratum: Stratum, spacing: f64, max_slope: f64, biomes: &'static [(Biome, f64)]) -> Plant {
    Plant { species, stratum, spacing, max_slope, wet: false, biomes, bark: None, leaves: None }
}

const fn coloured(p: Plant, bark: [f64; 3], leaves: [f64; 3]) -> Plant {
    Plant { bark: Some(bark), leaves: Some(leaves), ..p }
}

/// Needles: dark, blue-green.
const NEEDLES: [f64; 3] = [0.05, 0.08, 0.04];
/// Broad leaves: brighter, yellower green.
const BROADLEAF: [f64; 3] = [0.09, 0.15, 0.05];
/// How tall the tundra's dwarf shrubs grow, metres.
const DWARF: f64 = 0.6;

const BROWN_BARK: [f64; 3] = [0.2, 0.15, 0.11];
const GREY_BARK: [f64; 3] = [0.36, 0.33, 0.29];
const CACTUS: [f64; 3] = [0.09, 0.15, 0.07];
const SPINES: [f64; 3] = [0.62, 0.56, 0.44];

/// The plants, by stratum, each with the biomes it grows in.
pub const FLORA: &[Plant] = &[
    // Trees.
    coloured(plant("norway-spruce", Tree, 7., 40., &[(Taiga, 0.7), (TemperateForest, 0.15)]), BROWN_BARK, NEEDLES),
    coloured(plant("balsam-fir", Tree, 6., 40., &[(Taiga, 0.4)]), GREY_BARK, NEEDLES),
    coloured(plant("scots-pine", Tree, 8., 40., &[(Taiga, 0.5), (TemperateForest, 0.2), (Grassland, 0.04), (Steppe, 0.04)]), [0.3, 0.17, 0.1], NEEDLES),
    coloured(plant("quaking-aspen", Tree, 6., 35., &[(Taiga, 0.2), (TemperateForest, 0.3), (Grassland, 0.05)]), [0.5, 0.5, 0.45], BROADLEAF),
    coloured(plant("california-black-oak", Tree, 10., 30., &[(TemperateForest, 0.6), (Grassland, 0.08)]), [0.18, 0.16, 0.13], BROADLEAF),
    coloured(plant("black-tupelo", Tree, 9., 30., &[(TemperateForest, 0.3), (Rainforest, 0.5)]), [0.2, 0.17, 0.14], [0.06, 0.13, 0.04]),
    coloured(Plant { wet: true, ..plant("weeping-willow", Tree, 10., 15., &[(TemperateForest, 0.6), (Grassland, 0.5), (Steppe, 0.3)]) }, BROWN_BARK, [0.11, 0.17, 0.06]),
    coloured(plant("palm", Tree, 8., 30., &[(Rainforest, 0.6), (Savanna, 0.08)]), [0.3, 0.26, 0.2], [0.07, 0.13, 0.04]),
    // Standing in for the acacias of the savanna: a flat, spreading crown on a bare trunk.
    coloured(plant("stone-pine", Tree, 14., 30., &[(Savanna, 0.15)]), [0.28, 0.2, 0.14], [0.08, 0.11, 0.04]),
    coloured(plant("joshua-tree", Tree, 9., 25., &[(HotDesert, 0.12), (ColdDesert, 0.15)]), [0.25, 0.2, 0.15], [0.11, 0.15, 0.07]),
    coloured(plant("utah-juniper", Tree, 8., 35., &[(ColdDesert, 0.3), (Steppe, 0.2)]), [0.3, 0.27, 0.23], [0.12, 0.14, 0.1]),
    coloured(plant("saguaro", Tree, 10., 30., &[(HotDesert, 0.25)]), CACTUS, SPINES),
    // Shrubs.
    coloured(plant("juniper", Shrub, 4., 40., &[(Tundra, 0.3), (Taiga, 0.2), (Steppe, 0.1)]), [0.3, 0.27, 0.23], [0.12, 0.14, 0.1]),
    coloured(plant("hazel", Shrub, 4., 35., &[(TemperateForest, 0.4)]), [0.3, 0.25, 0.2], BROADLEAF),
    coloured(plant("hawthorn", Shrub, 5., 35., &[(TemperateForest, 0.2), (Grassland, 0.25)]), [0.25, 0.22, 0.18], [0.08, 0.14, 0.05]),
    coloured(plant("shrub", Shrub, 3., 40., &[(Rainforest, 0.6), (Savanna, 0.3), (TemperateForest, 0.2), (Tundra, 0.15)]), BROWN_BARK, BROADLEAF),
    coloured(plant("creosote-bush", Shrub, 4., 30., &[(HotDesert, 0.6)]), GREY_BARK, [0.13, 0.16, 0.07]),
    coloured(plant("sagebrush", Shrub, 2.5, 35., &[(ColdDesert, 0.7), (Steppe, 0.5)]), GREY_BARK, [0.3, 0.33, 0.26]),
    coloured(plant("rabbitbrush", Shrub, 2.5, 35., &[(ColdDesert, 0.3), (Steppe, 0.2)]), GREY_BARK, [0.22, 0.27, 0.1]),
    coloured(plant("ocotillo", Shrub, 5., 35., &[(HotDesert, 0.15)]), GREY_BARK, [0.1, 0.2, 0.05]),
    coloured(plant("barrel-cactus", Shrub, 3., 40., &[(HotDesert, 0.15)]), [0.1, 0.17, 0.07], [0.5, 0.24, 0.14]),
    coloured(plant("prickly-pear", Shrub, 3., 35., &[(HotDesert, 0.2), (Steppe, 0.05)]), [0.13, 0.21, 0.07], SPINES),
    coloured(plant("agave", Shrub, 3., 40., &[(HotDesert, 0.2)]), [0.2, 0.28, 0.24], [0.12, 0.1, 0.08]),
];

/// The ground's plants in a biome: `procedural::grass`'s species, how thickly (blades per
/// square metre, as a `grass`'s `density`) and in what condition.
#[derive(Clone, Copy, Debug)]
pub struct Ground {
    pub biome: Biome,
    pub species: &'static [&'static str],
    pub density: f64,
    pub conditions: &'static str,
}

pub const GROUND: &[Ground] = &[
    Ground { biome: Tundra, species: &["tussock", "fescue", "heather"], density: 250., conditions: "autumn" },
    Ground { biome: Taiga, species: &["fescue", "heather", "bracken", "litter"], density: 200., conditions: "summer" },
    Ground { biome: TemperateForest, species: &["meadow", "bent", "bracken", "litter"], density: 300., conditions: "summer" },
    Ground { biome: Grassland, species: &["meadow", "timothy", "wild_oat", "daisy", "oxeye", "dandelion", "buttercup", "clover"], density: 350., conditions: "wild" },
    Ground { biome: Steppe, species: &["fescue", "tussock", "wild_oat"], density: 200., conditions: "drought" },
    Ground { biome: ColdDesert, species: &["tussock"], density: 40., conditions: "drought" },
    Ground { biome: Savanna, species: &["savanna"], density: 300., conditions: "drought" },
    Ground { biome: Rainforest, species: &["bracken", "litter"], density: 150., conditions: "lush" },
];

/// What grows in `biome`: its plants with their abundance there, largest first, and its
/// ground's plants.
pub fn available(biome: Biome) -> (Vec<(&'static Plant, f64)>, Option<&'static Ground>) {
    let plants = FLORA.iter().filter_map(|p| p.biomes.iter().find(|(b, _)| *b == biome).map(|&(_, a)| (p, a))).collect();
    (plants, GROUND.iter().find(|g| g.biome == biome))
}

/// The share of each biome among `samples` (biomes found at points about the camera), most
/// first; those under 2% left out.
pub fn shares(samples: &[Biome]) -> Vec<(Biome, f64)> {
    let mut out: Vec<(Biome, f64)> = Biome::ALL.iter()
        .map(|&b| (b, samples.iter().filter(|&&s| s == b).count() as f64 / samples.len().max(1) as f64))
        .filter(|&(_, s)| s >= 0.02).collect();
    out.sort_by(|a, b| b.1.total_cmp(&a.1));
    out
}

/// `weight` where the biome is one of `biomes`, each by its abundance: a graph.
fn abundance(biomes: &[(Biome, f64)]) -> Value {
    let terms: Vec<Value> = biomes.iter()
        .map(|(b, a)| json!({"node": "multiply", "inputs": [{"node": "biome", "is": b.name()}, a]}))
        .collect();
    if terms.len() == 1 { terms[0].clone() } else { json!({"node": "add", "inputs": terms}) }
}

/// Off what nothing roots in (glacier ice, drifting sand, playas), and sparse on bare rock: a
/// fifth as thick where there is no soil, as plants hold on only in its cracks.
fn rooted() -> [Value; 4] {
    let none_on = |field: &str, from: f64, to: f64| json!({"node": "field", "field": field, "from": from, "to": to});
    let soil = json!({"node": "add", "inputs": [0.2, {"node": "multiply", "inputs": [0.8, none_on("sediment", 0.05, 0.5)]}]});
    [none_on("ice", 2., 0.5), none_on("sand", 2., 0.5), none_on("playa", 0.5, 0.1), soil]
}

/// The populations of an ecosystem with `"flora"` for a landscape whose biomes about the
/// camera are `present` (`shares`), and the materials they name: a forest of each plant of
/// those biomes, trees then shrubs, and a grass of each biome's ground plants. `strata` picks
/// which: "trees", "shrubs" and "ground".
pub fn populations(present: &[(Biome, f64)], strata: &[&str], seed: u64) -> (Vec<Value>, Map<String, Value>) {
    let mut out = vec![];
    let mut materials = Map::new();
    let here = |biomes: &[(Biome, f64)]| -> Vec<(Biome, f64)> {
        biomes.iter().copied().filter(|(b, _)| present.iter().any(|(p, _)| p == b)).collect()
    };
    for (stratum, name) in [(Tree, "trees"), (Shrub, "shrubs")] {
        if !strata.contains(&name) { continue; }
        // Each plant, and in the tundra its shrubs again as dwarfs: they grow knee-high at most
        // there, hugging the ground out of the wind.
        let plants = FLORA.iter().filter(|p| p.stratum == stratum).flat_map(|p| {
            let (tundra, rest): (Abundances, Abundances) = here(p.biomes).into_iter().partition(|(b, _)| *b == Tundra);
            [(p, rest, None), (p, tundra, Some(DWARF))]
        });
        for (p, biomes, height) in plants {
            if biomes.is_empty() { continue; }
            let mut rule = vec![abundance(&biomes), json!({"node": "slope", "from": p.max_slope, "to": p.max_slope - 10.})];
            rule.extend(rooted());
            // Trees stop at the treeline (the biome's edge, which wanders, besides); shrubs
            // are the tundra's own, and stop below the snow.
            rule.push(match stratum {
                Tree => json!({"node": "field", "field": "treeline", "from": 50, "to": -150}),
                Shrub => json!({"node": "field", "field": "snowline", "from": -100, "to": -300}),
            });
            if p.wet { rule.push(json!({"node": "field", "field": "wetness", "from": 9, "to": 12})); }
            let mut species = json!({"species": p.species, "weight": 1});
            if let Some(h) = height { species["params"] = json!({"scale": h, "scale_v": 0.3 * h}); }
            if let (Some(bark), Some(leaves)) = (p.bark, p.leaves) {
                let (b, l) = (format!("FLORA_{}_BARK", p.species), format!("FLORA_{}_LEAVES", p.species));
                materials.insert(b.clone(), json!({"type": "lambertian", "albedo": bark}));
                let succulent = matches!(p.species, "saguaro" | "barrel-cactus" | "prickly-pear" | "agave");
                materials.insert(l.clone(), if succulent {
                    json!({"type": "lambertian", "albedo": leaves})
                } else {
                    json!({"type": "leaf", "reflectance": leaves, "transmittance": leaves.map(|c| c * 0.5), "roughness": 0.4})
                });
                species["bark"] = Value::from(b);
                species["leaves"] = Value::from(l);
            }
            let reach = match stratum { Tree => 300., Shrub => 150. };
            let spacing = if height.is_some() { 0.5 * p.spacing } else { p.spacing };
            out.push(json!({"forest": {
                "reach": reach, "spacing": spacing, "clearings": 0.1, "patch_size": 30. * p.spacing, "patchiness": 0.5,
                "max_slope": p.max_slope, "variants": 3, "lod": true,
                "species": [species],
                "where": {"node": "multiply", "inputs": rule},
            }}));
        }
    }
    if strata.contains(&"ground") {
        for g in GROUND.iter().filter(|g| present.iter().any(|(b, _)| *b == g.biome)) {
            let mut rule = vec![json!({"node": "biome", "is": g.biome.name()}), json!({"node": "slope", "from": 40, "to": 28})];
            rule.extend(rooted());
            rule.push(json!({"node": "field", "field": "snowline", "from": 0, "to": -200}));
            out.push(json!({"grass": {
                "reach": 30, "density": g.density, "species": g.species, "conditions": g.conditions,
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

    /// Every plant is a species the library grows, every ground's plants grasses it has, and
    /// every biome but ice has something growing in it.
    #[test]
    fn test_the_flora_is_the_librarys_and_covers_the_biomes() {
        for p in FLORA {
            assert!(crate::procedural::tree::SPECIES.contains(&p.species), "{} isn't a tree species", p.species);
            assert!(p.biomes.iter().all(|&(_, a)| a > 0. && a <= 1.), "{}", p.species);
        }
        for g in GROUND {
            for s in g.species {
                assert!(crate::procedural::grass::SPECIES_PRESETS.contains(s), "{} isn't a grass species", s);
            }
            assert!(crate::procedural::grass::CONDITIONS_PRESETS.contains(&g.conditions), "{}", g.conditions);
        }
        for b in Biome::ALL {
            let (plants, ground) = available(b);
            assert_eq!(b == Biome::Ice, plants.is_empty() && ground.is_none(), "{:?}", b);
        }
        // Forests where the climate makes them; deserts without trees but their own.
        assert!(available(Biome::Taiga).0.iter().any(|(p, a)| p.stratum == Stratum::Tree && *a >= 0.5));
        assert!(available(Biome::HotDesert).0.iter().all(|(p, _)| ["joshua-tree", "saguaro"].contains(&p.species) || p.stratum == Stratum::Shrub));
    }

    /// The populations are those of the biomes present only, trees before shrubs before the
    /// ground, and each rule parses, naming only those biomes.
    #[test]
    fn test_populations_follow_the_biomes_present() {
        let present = shares(&[Biome::Taiga, Biome::Taiga, Biome::Tundra]);
        assert_eq!(present.iter().map(|(b, _)| *b).collect::<Vec<_>>(), vec![Biome::Taiga, Biome::Tundra]);
        let (pops, materials) = populations(&present, &["trees", "shrubs", "ground"], 1);
        let kinds: Vec<&str> = pops.iter().map(|p| if p.get("forest").is_some() { "forest" } else { "grass" }).collect();
        let first_grass = kinds.iter().position(|&k| k == "grass").unwrap();
        assert!(kinds[..first_grass].iter().all(|&k| k == "forest") && kinds[first_grass..].iter().all(|&k| k == "grass"));
        let names: Vec<String> = pops.iter().filter_map(|p| p["forest"]["species"][0]["species"].as_str().map(String::from)).collect();
        assert!(names.contains(&"norway-spruce".to_string()) && names.contains(&"juniper".to_string()));
        assert!(!names.contains(&"saguaro".to_string()) && !names.contains(&"california-black-oak".to_string()));
        for p in &pops {
            let spec = p.get("forest").or(p.get("grass")).unwrap();
            crate::graph::Graph::parse(&spec["where"]).unwrap();
            let text = spec["where"].to_string();
            assert!(!text.contains("temperate_forest") && !text.contains("desert"), "{}", text);
            if let Some(b) = spec["species"][0]["bark"].as_str() { assert!(materials.contains_key(b)); }
        }
        // The tundra's junipers are dwarfs, the taiga's not.
        let junipers: Vec<&Value> = pops.iter().filter(|p| p["forest"]["species"][0]["species"] == "juniper").collect();
        assert_eq!(junipers.len(), 2);
        assert_eq!(junipers.iter().filter(|p| p["forest"]["species"][0]["params"]["scale"] == json!(DWARF)).count(), 1);
        let (trees_only, _) = populations(&present, &["trees"], 1);
        assert!(trees_only.iter().all(|p| p.get("forest").is_some()) && trees_only.len() < pops.len());
    }
}
