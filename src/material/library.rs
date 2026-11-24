//! # A library of procedural materials
//!
//! Ready-made materials, as POV-Ray's include files have them (`stones.inc`, `woods.inc`,
//! `metals.inc`, `textures.inc`): stones, woods, metals, and a few others, each built from the
//! pieces any scene can use (function graphs, `graph.rs`, with the noises of `patterns.rs`;
//! colour maps, `textures::PatternTexture`; bumps from the same graphs, `bump`; roughness
//! maps; mixes). They are written as ordinary material definitions in `library.json`, so any
//! of them can be copied into a scene and changed; docs/procedural-materials.md describes
//! each and what it is modelled on.
//!
//! A scene can add presets of its own, or replace built-in ones of the same name, with a
//! top-level `"library"`: a file of presets in the same form as `library.json` (read through
//! `crate::assets`, so the browser builder can provide it), an object of presets written in
//! place, or a list of either, later ones over earlier. They are loaded at run time (`load`),
//! so a preset can be tuned without rebuilding; the render key hashes the files' contents
//! (`files`), so editing one restarts a resumable render. They reach the material parser in
//! the scene's materials map under `SCENE_PRESETS`.
//!
//! A preset's entry may carry anything else for the tools around it, which the renderer
//! ignores: it reads only `material`, `variables`, `seed_direction`, `overlay` and `scale` (the
//! size it was made at, as a fit to a scan finds it, multiplied into any asked for; and `doc`
//! for messages). The gallery reads `show` and `over_for_landscape`
//! (`scripts/material-gallery.py`); `references` records the scanned surfaces a preset has been
//! compared with and fitted to, and how the fit went (`scripts/fit-material.py`).
//!
//! `{"type": "library", "name": "oak", "scale": 1, "seed": 0}` uses one. `scale` sizes every
//! pattern in it (and the depth of its relief with them); `seed` moves every pattern to another
//! part of its noise, so two objects of the same material differ; any other field is put in
//! place of the preset's own (`"roughness": 0.3` to dull a polished stone).
//!
//! A preset may define `variables` for itself, as a scene file does: a graph defined once and
//! used by both its colours and its bump, so they line up. They are also its parameters: a
//! field of the same name replaces one (`"amount": 0.8` for more dust, `"color": [...]`).
//!
//! An overlay (dust, grime, moss, rain streaks...) lies over another material, given by
//! `"over"` (a name or a definition), which the preset refers to as `"@over"`: weathering for
//! anything. Presets may use other presets, `{"type": "library", ...}`, which take the outer
//! one's scale and seed with them. And `seed_direction`: the one
//! direction its seed may move it, where moving it any other way would change its kind (wood's
//! rings are about an axis, which the seed must slide along, not away from).
use serde_json::{Map, Value};

/// The presets, as JSON.
pub const LIBRARY: &str = include_str!("library.json");

pub fn library() -> &'static Map<String, Value> {
    static PARSED: std::sync::LazyLock<Map<String, Value>> = std::sync::LazyLock::new(|| match serde_json::from_str(LIBRARY) {
        Ok(Value::Object(m)) => m,
        _ => panic!("library.json is not a JSON object"),
    });
    &PARSED
}

/// The presets' names, in the library's order.
pub fn names() -> Vec<&'static str> {
    library().keys().map(|k| k.as_str()).collect()
}

/// The key under which a scene's own presets travel in its materials map: not a name a
/// material can be referred to by.
pub const SCENE_PRESETS: &str = "@library";

/// The files a scene's `"library"` names, in order.
pub fn files(spec: &Value) -> Vec<String> {
    match spec {
        Value::String(path) => vec![path.clone()],
        Value::Array(items) => items.iter().flat_map(files).collect(),
        _ => vec![],
    }
}

/// A scene's own presets, from its `"library"`: a file's path, an object of presets, or a
/// list of these, each over those before it.
pub fn load(spec: &Value) -> Result<Map<String, Value>, String> {
    let mut presets = Map::new();
    let mut add = |found: Map<String, Value>, from: &str| -> Result<(), String> {
        for (name, preset) in found {
            if !preset.get("material").is_some_and(Value::is_object) {
                return Err(format!("{}: preset \"{}\" needs a \"material\" object", from, name));
            }
            presets.insert(name, preset);
        }
        Ok(())
    };
    let parts: Vec<&Value> = match spec {
        Value::Array(items) => items.iter().collect(),
        Value::Null => vec![],
        v => vec![v],
    };
    for part in parts {
        match part {
            Value::String(path) => {
                let bytes = crate::assets::read(path).map_err(|e| format!("can't read library {}: {}", path, e))?;
                match serde_json::from_slice(&bytes).map_err(|e| format!("library {}: {}", path, e))? {
                    Value::Object(m) => add(m, path)?,
                    _ => return Err(format!("library {} must be an object of presets", path)),
                }
            },
            Value::Object(m) => add(m.clone(), "the scene's library")?,
            v => return Err(format!("a library is a file's path, an object of presets, or a list of these, not {}", v)),
        }
    }
    Ok(presets)
}

/// Whether a preset is an overlay, lying over the material given as `"over"`.
pub fn is_overlay(name: &str) -> bool {
    library().get(name).and_then(|p| p.get("overlay")).and_then(Value::as_bool).unwrap_or(false)
}

/// A preset's one-line description.
pub fn doc(name: &str) -> Option<&'static str> {
    library().get(name)?.get("doc")?.as_str()
}

/// The definition a `{"type": "library", ...}` material stands for, from the built-in presets.
pub fn expand(props: &Value) -> Result<Value, String> {
    expand_in(props, None)
}

/// The same, looking in a scene's own presets (`load`) before the built-in ones.
pub fn expand_in(props: &Value, scene: Option<&Map<String, Value>>) -> Result<Value, String> {
    let name = props["name"].as_str().ok_or("a library material needs a \"name\"")?;
    let preset = scene.and_then(|s| s.get(name)).or_else(|| library().get(name)).ok_or_else(|| {
        let mut all: Vec<&str> = scene.into_iter().flat_map(|s| s.keys().map(String::as_str)).chain(names()).collect();
        all.sort();
        all.dedup();
        format!("no library material \"{}\": {}", name, all.join(", "))
    })?;
    let scale = props.get("scale").map_or(Some(1.), Value::as_f64).filter(|s| *s > 0.)
        .ok_or("a library material's scale must be a positive number")?;
    // A preset's own scale, as fitted to a scan, under the one asked for.
    let scale = scale * preset.get("scale").and_then(Value::as_f64).filter(|s| *s > 0.).unwrap_or(1.);
    let seed = props.get("seed").map_or(Some(0), Value::as_u64).ok_or("a library material's seed is a whole number, at least 0")?;
    // The material, with the preset's own variables substituted.
    let mut variables = match preset.get("variables") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };
    // Fields naming the preset's variables set them.
    for (k, v) in props.as_object().into_iter().flatten() {
        if variables.contains_key(k) && !RESERVED.contains(&k.as_str()) {
            variables.insert(k.clone(), v.clone());
        }
    }
    let mut doc = serde_json::json!({ "variables": Value::Object(variables.clone()), "material": preset["material"].clone() });
    doc = crate::scenefile::SceneFile::substitute_variables(doc).map_err(|e| format!("library material {}: {}", name, e))?;
    let mut material = doc["material"].take();
    let offset = if seed == 0 { [0.; 3] } else {
        let mut rng = crate::geometry::Pcg32::new(seed, 0x11b);
        let mut v = [0.; 3];
        for c in v.iter_mut() {
            *c = (rng.next_f64() - 0.5) * 2000.;
        }
        match preset.get("seed_direction").and_then(Value::as_array) {
            Some(d) if d.len() == 3 => {
                let d: Vec<f64> = d.iter().map(|x| x.as_f64().unwrap_or(0.)).collect();
                let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-12);
                [d[0] / norm * v[0], d[1] / norm * v[0], d[2] / norm * v[0]]
            },
            _ => v,
        }
    };
    if scale != 1. || seed != 0 {
        place(&mut material, scale, offset);
    }
    // What an overlay lies over.
    let over = props.get("over");
    put_over(&mut material, over).map_err(|e| format!("library material {}: {}", name, e))?;
    // Other fields override the preset's.
    if let (Value::Object(m), Value::Object(p)) = (&mut material, props) {
        for (k, v) in p {
            if !RESERVED.contains(&k.as_str()) && !variables.contains_key(k) {
                m.insert(k.clone(), v.clone());
            }
        }
    }
    Ok(material)
}

/// Fields of a library material that aren't the preset's own.
const RESERVED: [&str; 5] = ["type", "name", "scale", "seed", "over"];

/// `"@over"` replaced by what the overlay lies over.
fn put_over(v: &mut Value, over: Option<&Value>) -> Result<(), String> {
    match v {
        Value::String(s) if s == "@over" => {
            *v = over.cloned().ok_or("an overlay needs \"over\": the material it lies over")?;
        },
        Value::Object(m) => for (_, f) in m.iter_mut() { put_over(f, over)? },
        Value::Array(a) => for f in a { put_over(f, over)? },
        _ => {},
    }
    Ok(())
}

/// Every graph in a material definition (any object with a `"node"`) read at its point scaled
/// down by `scale` and moved by `offset`, and every graph bump's relief scaled with it.
fn place(v: &mut Value, scale: f64, offset: [f64; 3]) {
    match v {
        Value::Object(m) if m.contains_key("node") => {
            let graph = v.take();
            *v = serde_json::json!({ "node": "transform", "scale": scale, "translate": offset, "input": graph });
        },
        // Another preset: scaled and seeded with this one.
        Value::Object(m) if m.get("type").and_then(Value::as_str) == Some("library") => {
            let inner = m.get("scale").and_then(Value::as_f64).unwrap_or(1.);
            m.insert("scale".into(), serde_json::json!(inner * scale));
            let seed = m.get("seed").and_then(Value::as_u64).unwrap_or(0);
            // A seed from the offset, so different outer seeds give different inner ones.
            let outer = (offset[0].to_bits() ^ offset[1].to_bits().rotate_left(21) ^ offset[2].to_bits().rotate_left(42)) % 1_000_003;
            m.insert("seed".into(), serde_json::json!(seed.wrapping_mul(31).wrapping_add(outer)));
            // And what it lies over, if anything.
            if let Some(over) = m.get_mut("over") {
                place(over, scale, offset);
            }
        },
        Value::Object(m) => {
            if let Some(n) = m.get_mut("normal").filter(|n| n.get("graph").is_some()) {
                let amount = n["amount"].as_f64().unwrap_or(1.);
                n["amount"] = serde_json::json!(amount * scale);
            }
            for (_, f) in m.iter_mut() {
                place(f, scale, offset);
            }
        },
        Value::Array(a) => for f in a { place(f, scale, offset) },
        _ => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_preset_builds_and_renders() {
        for name in names() {
            assert!(doc(name).is_some_and(|d| !d.is_empty()), "{} has no doc", name);
            for (scale, seed) in [(1., 0), (0.5, 3)] {
                let s = crate::scenefile::SceneFile::from_string(format!(r#"{{
                    "width": 4, "height": 2, "chunk_size": 2, "supersamples": 1, "samples_per_chunk": 1,
                    "shadow_bias": 0.0001, "background": [0.5, 0.5, 0.5], "max_depth": 2,
                    "camera": {{ "location": [0, 0, -3], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
                    "lights": [ {{ "location": [2, 5, -5], "intensity": 50 }} ],
                    "materials": {{ "M": {{ "type": "library", "name": "{}", "scale": {}, "seed": {}{} }} }},
                    "objects": [ {{ "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "M" }} ]
                }}"#, name, scale, seed, if is_overlay(name) {{ r#", "over": {"type": "lambertian", "albedo": [0.5, 0.5, 0.5]}"# }} else {{ "" }}));
                for k in 0 .. 20 {
                    let r = crate::ray::Ray { ro: crate::na::Vector3::new(-0.5 + k as f64 * 0.05, 0.3, -3.), rd: crate::na::Vector3::new(0., -0.1, 1.) };
                    let c = crate::trace::trace(&r, 0, &s).1;
                    assert!(c.rgb.iter().all(|v| v.is_finite() && *v >= 0.), "{}: {:?}", name, c);
                }
            }
        }
    }

    #[test]
    fn test_scale_seed_and_overrides() {
        let m = expand(&serde_json::json!({"type": "library", "name": names()[0], "scale": 2, "seed": 5, "roughness": 0.7})).unwrap();
        assert_eq!(m["roughness"], 0.7);
        let text = m.to_string();
        assert!(text.contains("\"transform\""), "{}", text);
        assert!(expand(&serde_json::json!({"type": "library", "name": "unobtainium"})).unwrap_err().contains("no library material"));
        assert!(expand(&serde_json::json!({"type": "library", "name": names()[0], "scale": 0})).is_err());
    }

    #[test]
    fn test_scene_presets_load_and_come_first() {
        let dir = std::env::temp_dir().join(format!("rays-library-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("extra.json");
        std::fs::write(&file, r#"{ "blue_stone": { "doc": "d", "material": { "type": "lambertian", "albedo": [0.1, 0.2, 0.5] } },
                                   "granite": { "doc": "mine", "material": { "type": "lambertian", "albedo": [1, 0, 0] } } }"#).unwrap();
        let path = file.to_str().unwrap().to_string();
        let inline = serde_json::json!({ "green_stone": { "material": { "type": "lambertian", "albedo": [0, 1, 0] } } });
        let presets = load(&serde_json::json!([path, inline])).unwrap();
        assert_eq!(files(&serde_json::json!([path, inline])), vec![path.clone()]);
        // The scene's own, and its own over the built-in one of the same name; the rest built in.
        let m = |name: &str| expand_in(&serde_json::json!({"type": "library", "name": name}), Some(&presets)).unwrap();
        assert_eq!(m("blue_stone")["albedo"], serde_json::json!([0.1, 0.2, 0.5]));
        assert_eq!(m("green_stone")["albedo"], serde_json::json!([0, 1, 0]));
        assert_eq!(m("granite")["albedo"], serde_json::json!([1, 0, 0]));
        assert_eq!(m("oak")["type"], "plastic");
        assert!(expand_in(&serde_json::json!({"type": "library", "name": "nope"}), Some(&presets)).unwrap_err().contains("blue_stone"));
        assert!(load(&serde_json::json!({"bad": {"doc": "no material"}})).unwrap_err().contains("bad"));
        assert!(load(&serde_json::json!("/no/such/library.json")).unwrap_err().contains("can't read"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_a_presets_own_scale_multiplies() {
        let presets = load(&serde_json::json!({ "small": { "scale": 0.5, "material": { "type": "lambertian",
            "albedo": {"pattern": {"node": "gradient"}, "color_map": [[0, [0, 0, 0]], [1, [1, 1, 1]]]} } } })).unwrap();
        let m = expand_in(&serde_json::json!({"type": "library", "name": "small", "scale": 3}), Some(&presets)).unwrap();
        assert_eq!(m["albedo"]["pattern"]["scale"], 1.5);
    }

    #[test]
    fn test_other_properties_are_ignored() {
        let presets = load(&serde_json::json!({ "plain": { "doc": "d", "references": [{"anything": [1, 2]}], "made_by": {"tool": 3},
            "material": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } } })).unwrap();
        let m = expand_in(&serde_json::json!({"type": "library", "name": "plain"}), Some(&presets)).unwrap();
        assert_eq!(m, serde_json::json!({ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }));
    }

    #[test]
    fn test_references_point_into_their_presets() {
        // A preset's recorded fits name places in it; editing the preset mustn't leave them
        // pointing nowhere.
        for name in names() {
            for r in library()[name].get("references").and_then(Value::as_array).into_iter().flatten() {
                assert!(r["source"].as_str().is_some_and(|s| s.contains(':')), "{}: a reference's source is site:id", name);
                assert!(r["size"].as_f64().is_some_and(|s| s > 0.), "{}: a reference needs its size", name);
                for p in r["params"].as_array().into_iter().flatten() {
                    let Some(path) = p["path"].as_array() else { continue };
                    let mut at = &library()[name];
                    for k in path {
                        at = match k { Value::String(s) => &at[s.as_str()], _ => &at[k.as_u64().unwrap() as usize] };
                    }
                    let ok = at.is_number() || (p["kind"] == "color" && at.as_array().is_some_and(|a| a.len() == 3))
                        || (at.is_null() && p.get("default").is_some());
                    assert!(ok, "{}: {:?} is {}", name, path, at);
                }
            }
        }
    }

    #[test]
    fn test_overlays_and_parameters() {
        let overlay = names().into_iter().find(|n| is_overlay(n)).expect("an overlay");
        assert!(expand(&serde_json::json!({"type": "library", "name": overlay})).unwrap_err().contains("over"));
        let base = serde_json::json!({"type": "lambertian", "albedo": [0.1, 0.2, 0.3]});
        let m = expand(&serde_json::json!({"type": "library", "name": overlay, "over": base.clone(), "scale": 3})).unwrap();
        // The base goes in as given, not scaled with the overlay.
        assert!(m.to_string().contains(&base.to_string()), "{}", m);
        // A variable set by a field of its name.
        let preset = names().into_iter().find(|n| library()[*n].get("variables").is_some_and(|v| v.get("amount").is_some())).expect("a preset with an amount");
        let over = if is_overlay(preset) { base } else { Value::Null };
        let a = expand(&serde_json::json!({"type": "library", "name": preset, "over": over, "amount": 0.123456})).unwrap();
        assert!(a.to_string().contains("0.123456") && a.get("amount").is_none(), "{}", a);
    }
}
