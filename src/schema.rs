//! What a scene file may contain, as data: every type of object, material and medium, and
//! every setting, with each field's kind, default and a line of description.
//!
//! The parser (`scenefile.rs` and the modules it calls) reads scene files key by key, with
//! defaults at each call; nothing in it lists what a type accepts. An editor needs that list,
//! to show a type's fields before any is set and to offer the types there are. `schema.json`
//! is that list, written out by hand from the parser and kept next to it. The tests here hold
//! the two together: every type the parser dispatches on has an entry, and every entry's
//! example, and every default set explicitly, builds.
//!
//! The format is described in `web/README.md`. In brief: `objects`, `materials` and `media`
//! map a `"type"` string to an entry with `label`, `group`, `doc`, `fields` and a minimal
//! `example`; `sections` holds the fields of the scene file's other parts (`settings` for its
//! top level, `camera`, `sun`, `output`, `light`, and `object_common` for the keys every
//! object accepts). A field has a `name`, a `kind`, and either a `default` or `required`.
use serde_json::Value;

/// The schema, as JSON text.
pub const SCHEMA: &str = include_str!("schema.json");

pub fn schema() -> Value {
    serde_json::from_str(SCHEMA).expect("schema.json is not valid JSON")
}

/// The kinds a field may have.
pub const KINDS: [&str; 16] = [
    "number", "integer", "boolean", "string", "vec2", "vec3", "color", "enum", "material", "medium",
    "shape", "shapes", "path", "object", "list", "json",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use serde_json::json;

    const CATEGORIES: [&str; 3] = ["objects", "materials", "media"];

    fn entries(schema: &Value) -> Vec<(&'static str, String, &Value)> {
        CATEGORIES.iter().flat_map(|c| {
            schema[*c].as_object().unwrap_or_else(|| panic!("schema has no {}", c))
                .iter().map(move |(name, e)| (*c, name.clone(), e))
        }).collect()
    }

    fn check_fields(at: &str, fields: &Value, problems: &mut Vec<String>) {
        let Some(fields) = fields.as_array() else {
            problems.push(format!("{}: fields is not a list", at));
            return;
        };
        let mut seen = Vec::new();
        for f in fields {
            let name = f["name"].as_str().unwrap_or("?");
            let at = format!("{}.{}", at, name);
            if seen.contains(&name) { problems.push(format!("{}: listed twice", at)); }
            seen.push(name);
            let kind = f["kind"].as_str().unwrap_or("");
            if !KINDS.contains(&kind) { problems.push(format!("{}: unknown kind {:?}", at, kind)); }
            if f.get("default").is_some() == (f["required"] == json!(true)) {
                problems.push(format!("{}: needs exactly one of default and required", at));
            }
            if f["doc"].as_str().is_none_or(|d| d.is_empty()) { problems.push(format!("{}: no doc", at)); }
            if kind == "enum" && f["options"].as_array().is_none_or(|o| o.is_empty()) {
                problems.push(format!("{}: enum without options", at));
            }
            if f.get("fields").is_some() { check_fields(&at, &f["fields"], problems); }
        }
    }

    #[test]
    fn test_schema_is_well_formed() {
        let schema = schema();
        let mut problems = Vec::new();
        for (category, name, e) in entries(&schema) {
            let at = format!("{}.{}", category, name);
            for key in ["label", "group", "doc"] {
                if e[key].as_str().is_none_or(|s| s.is_empty()) { problems.push(format!("{}: no {}", at, key)); }
            }
            if e["example"]["type"].as_str() != Some(&name) { problems.push(format!("{}: example's type differs", at)); }
            check_fields(&at, &e["fields"], &mut problems);
        }
        for name in ["settings", "camera", "sun", "output", "light", "object_common"] {
            if schema["sections"][name].is_null() { problems.push(format!("sections.{}: missing", name)); }
        }
        for (name, s) in schema["sections"].as_object().unwrap() {
            check_fields(&format!("sections.{}", name), &s["fields"], &mut problems);
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    /// The type strings a parser's source dispatches on: `t == "name"` and `"name" =>` (or
    /// `"a" | "b" =>`) arms, between two markers in the source.
    fn dispatched(source: &str, from: &str, to: &str) -> Vec<String> {
        let start = source.find(from).unwrap_or_else(|| panic!("marker {:?} not found", from));
        let end = start + source[start..].find(to).unwrap_or_else(|| panic!("marker {:?} not found", to));
        let mut names = Vec::new();
        for line in source[start..end].lines() {
            let line = line.trim();
            if line.starts_with("if t == \"") || line.starts_with("} else if t == \"") {
                names.extend(line.split("t == \"").skip(1).filter_map(|s| s.split('"').next()).map(String::from));
            } else if line.starts_with('"') && line.contains("=>") {
                let arms = line.split("=>").next().unwrap();
                names.extend(arms.split('"').skip(1).step_by(2).map(String::from));
            }
        }
        names
    }

    /// Every type the parser accepts has a schema entry, so a type added to the parser without
    /// one fails here.
    #[test]
    fn test_every_parsed_type_has_an_entry() {
        let schema = schema();
        let scenefile = include_str!("scenefile.rs");
        let shapes = include_str!("scenefile_shapes.rs");
        let mut missing = Vec::new();
        let mut check = |category: &str, names: Vec<String>, least: usize| {
            assert!(names.len() >= least, "found only {} {} in the parser's source: has it been restructured?", names.len(), category);
            for n in names {
                if schema[category][&n].is_null() { missing.push(format!("{}.{}", category, n)); }
            }
        };
        let mut objects = dispatched(scenefile, "pub fn parse_object(", "pub fn parse_skysphere(");
        objects.extend(dispatched(scenefile, "pub fn parse_geometry", "pub fn parse_shared_geometry("));
        objects.extend(dispatched(shapes, "pub fn parse_shape(", "\n}\n"));
        objects.extend(["tree", "forest"].map(String::from));
        // Arms inside a shape's own options (a prism's "linear" sweep, say) are not types.
        let options = ["linear", "bezier", "conic", "cubic_spline", "b_spline"];
        objects.retain(|n| !options.contains(&n.as_str()));
        check("objects", objects, 50);
        check("materials", dispatched(scenefile, "pub fn parse_material(", "pub fn parse_rock("), 9);
        check("media", dispatched(scenefile, "pub fn parse_medium(", "pub fn parse_air("), 4);
        assert!(missing.is_empty(), "no schema entry for: {}", missing.join(", "));
    }

    fn scene_with(category: &str, entry: &Value, instance: &Value) -> Value {
        let needs = |what: &str| entry["needs"].as_array().is_some_and(|n| n.contains(&json!(what)));
        let mut objects = Vec::new();
        if needs("skysphere") && instance["type"] != "skysphere" { objects.push(json!({ "type": "skysphere" })); }
        let has_sky = !objects.is_empty() || instance["type"] == "skysphere";
        let mut materials = json!({ "M": { "type": "lambertian", "albedo": [0.8, 0.8, 0.8] } });
        let mut media = json!({ "MM": { "type": "solid", "material": "M" } });
        match category {
            "objects" => objects.push(instance.clone()),
            "materials" => {
                materials["X"] = instance.clone();
                objects.push(json!({ "type": "sphere", "location": [0, 1, 0], "radius": 1, "material": "X" }));
            },
            _ => {
                media["X"] = instance.clone();
                objects.push(json!({ "type": "sphere", "location": [0, 1, 0], "radius": 1, "medium": "X" }));
            },
        }
        let mut scene = json!({
            "width": 8, "height": 6, "supersamples": 1, "samples_per_chunk": 1, "max_depth": 1,
            "camera": { "location": [0, 3, 11], "lookat": [0, 1, 0], "up": [0, 1, 0], "angle": 0.5, "aperture": 0 },
            "materials": materials, "media": media, "objects": objects,
        });
        if needs("sun") {
            scene["sun"] = if has_sky { json!({ "intensity": 3 }) } else { json!({ "intensity": 3, "direction": [1, 2, 1] }) };
        }
        scene
    }

    fn builds(scene: &Value) -> Result<(), String> {
        let text = scene.to_string();
        std::panic::catch_unwind(|| { SceneFile::from_string(text); })
            .map_err(|e| e.downcast_ref::<String>().cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "panicked".to_string()))
    }

    /// Each entry's example builds, and so does the example with every field's default written
    /// out: a default of the wrong kind, or a field the parser rejects, fails here. Entries
    /// marked slow are skipped (they take seconds to minutes each without optimisation).
    #[test]
    fn test_examples_and_defaults_build() {
        let schema = schema();
        let mut problems = Vec::new();
        for (category, name, e) in entries(&schema) {
            if e["slow"] == json!(true) || e["untested"].is_string() { continue; }
            if let Err(m) = builds(&scene_with(category, e, &e["example"])) {
                problems.push(format!("{}.{} example: {}", category, name, m));
                continue;
            }
            let mut full = e["example"].clone();
            for f in e["fields"].as_array().unwrap() {
                let n = f["name"].as_str().unwrap();
                if full.get(n).is_none() && f.get("default").is_some_and(|d| !d.is_null()) {
                    full[n] = f["default"].clone();
                }
            }
            if let Err(m) = builds(&scene_with(category, e, &full)) {
                problems.push(format!("{}.{} with defaults written out: {}", category, name, m));
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }
}
