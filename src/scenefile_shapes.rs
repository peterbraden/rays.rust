//! Scene file parsing for the shapes modelled on POV-Ray's: each `"type"` here builds one
//! geometry from its JSON object. Kept apart from `scenefile.rs`, which falls through to
//! `parse_shape` for any type it doesn't know.

use crate::na::{Vector2, Vector3};
use crate::scenefile::SceneFile;
use crate::shapes::geometry::Geometry;
use crate::shapes::cone::Cone;
use crate::shapes::disc::Disc;
use serde_json::Value;

type Shape = Box<dyn Geometry + Sync + Send>;

/// A number that must be present.
pub fn num(o: &Value, k: &str) -> f64 {
    o[k].as_f64().unwrap_or_else(|| panic!("{} needs a number \"{}\", got {}", o["type"], k, o[k]))
}

pub fn num_def(o: &Value, k: &str, def: f64) -> f64 {
    SceneFile::parse_number(&o[k], def)
}

/// A point that must be present.
pub fn vec3(o: &Value, k: &str) -> Vector3<f64> {
    match o[k].as_array() {
        Some(a) if a.len() == 3 && a.iter().all(|x| x.is_number()) => SceneFile::parse_vec3(&o[k]),
        _ => panic!("{} needs a point \"{}\" as [x, y, z], got {}", o["type"], k, o[k]),
    }
}

pub fn flag(o: &Value, k: &str) -> bool {
    o[k].as_bool().unwrap_or(false)
}

/// A list of [x, y] points.
pub fn points2(o: &Value, k: &str) -> Vec<Vector2<f64>> {
    o[k].as_array()
        .unwrap_or_else(|| panic!("{} needs a list of [x, y] points \"{}\"", o["type"], k))
        .iter()
        .map(|p| match p.as_array() {
            Some(a) if a.len() == 2 && a.iter().all(|x| x.is_number()) => SceneFile::parse_vec2(p),
            _ => panic!("{}: \"{}\" should hold [x, y] points, got {}", o["type"], k, p),
        })
        .collect()
}

/// A list of [x, y, z] points.
pub fn points3(o: &Value, k: &str) -> Vec<Vector3<f64>> {
    o[k].as_array()
        .unwrap_or_else(|| panic!("{} needs a list of [x, y, z] points \"{}\"", o["type"], k))
        .iter()
        .map(|p| match p.as_array() {
            Some(a) if a.len() == 3 && a.iter().all(|x| x.is_number()) => SceneFile::parse_vec3(p),
            _ => panic!("{}: \"{}\" should hold [x, y, z] points, got {}", o["type"], k, p),
        })
        .collect()
}

impl SceneFile {
    /// The geometry of type `t`, if it is one of the shapes parsed here.
    pub fn parse_shape(t: &str, o: &Value) -> Option<Shape> {
        Some(match t {
            // {"type": "cylinder", "base": [x,y,z], "cap": [x,y,z], "radius": r, "open": false}
            "cylinder" => Box::new(Cone::cylinder(vec3(o, "base"), vec3(o, "cap"), num(o, "radius"), flag(o, "open"))),
            // {"type": "cone", "base": [..], "base_radius": r0, "cap": [..], "cap_radius": r1, "open": false}
            "cone" => Box::new(Cone::new(
                vec3(o, "base"), num(o, "base_radius"),
                vec3(o, "cap"), num(o, "cap_radius"), flag(o, "open"))),
            // {"type": "disc", "center": [..], "normal": [..], "radius": r, "hole_radius": 0}
            "disc" => Box::new(Disc::new(
                vec3(o, "center"), vec3(o, "normal"), num(o, "radius"), num_def(o, "hole_radius", 0.))),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;

    pub fn shape(json: &str) -> Shape {
        let o: Value = serde_json::from_str(json).unwrap();
        SceneFile::parse_geometry(&o).expect("a geometry")
    }

    pub fn hit_from(s: &Shape, ro: [f64; 3], rd: [f64; 3]) -> Option<f64> {
        s.intersects(&Ray { ro: Vector3::from(ro), rd: Vector3::from(rd) }).map(|i| i.dist)
    }

    #[test]
    fn test_cylinder_cone_disc() {
        let c = shape(r#"{"type": "cylinder", "base": [0, 0, 0], "cap": [0, 2, 0], "radius": 1}"#);
        assert!((hit_from(&c, [0., 1., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-9);
        let k = shape(r#"{"type": "cone", "base": [0, 0, 0], "base_radius": 1, "cap": [0, 1, 0], "cap_radius": 0}"#);
        assert!(hit_from(&k, [0., 0.5, -5.], [0., 0., 1.]).is_some());
        let d = shape(r#"{"type": "disc", "center": [0, 0, 0], "normal": [0, 0, 1], "radius": 1, "hole_radius": 0.5}"#);
        assert!(hit_from(&d, [0., 0., -5.], [0., 0., 1.]).is_none());
        assert!(hit_from(&d, [0.7, 0., -5.], [0., 0., 1.]).is_some());
    }

    #[test]
    #[should_panic(expected = "needs a number \"radius\"")]
    fn test_missing_field_is_reported() {
        shape(r#"{"type": "cylinder", "base": [0, 0, 0], "cap": [0, 2, 0]}"#);
    }
}
