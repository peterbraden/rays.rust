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
//! `{"type": "library", "name": "oak", "scale": 1, "seed": 0}` uses one. `scale` sizes every
//! pattern in it (and the depth of its relief with them); `seed` moves every pattern to another
//! part of its noise, so two objects of the same material differ; any other field is put in
//! place of the preset's own (`"roughness": 0.3` to dull a polished stone).
//!
//! A preset may define `variables` for itself, as a scene file does: a graph defined once and
//! used by both its colours and its bump, so they line up. And `seed_direction`: the one
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

/// A preset's one-line description.
pub fn doc(name: &str) -> Option<&'static str> {
    library().get(name)?.get("doc")?.as_str()
}

/// The definition a `{"type": "library", ...}` material stands for.
pub fn expand(props: &Value) -> Result<Value, String> {
    let name = props["name"].as_str().ok_or("a library material needs a \"name\"")?;
    let preset = library().get(name).ok_or_else(|| format!("no library material \"{}\": {}", name, names().join(", ")))?;
    let scale = props.get("scale").map_or(Some(1.), Value::as_f64).filter(|s| *s > 0.)
        .ok_or("a library material's scale must be a positive number")?;
    let seed = props.get("seed").map_or(Some(0), Value::as_u64).ok_or("a library material's seed is a whole number, at least 0")?;
    // The material, with the preset's own variables substituted.
    let mut doc = serde_json::json!({ "variables": preset.get("variables").cloned().unwrap_or(Value::Null), "material": preset["material"].clone() });
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
    // Other fields override the preset's.
    if let (Value::Object(m), Value::Object(p)) = (&mut material, props) {
        for (k, v) in p {
            if !["type", "name", "scale", "seed"].contains(&k.as_str()) {
                m.insert(k.clone(), v.clone());
            }
        }
    }
    Ok(material)
}

/// Every graph in a material definition (any object with a `"node"`) read at its point scaled
/// down by `scale` and moved by `offset`, and every graph bump's relief scaled with it.
fn place(v: &mut Value, scale: f64, offset: [f64; 3]) {
    match v {
        Value::Object(m) if m.contains_key("node") => {
            let graph = v.take();
            *v = serde_json::json!({ "node": "transform", "scale": scale, "translate": offset, "input": graph });
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
                    "materials": {{ "M": {{ "type": "library", "name": "{}", "scale": {}, "seed": {} }} }},
                    "objects": [ {{ "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "M" }} ]
                }}"#, name, scale, seed));
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
}
