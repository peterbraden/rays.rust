//! Scene file parsing for the shapes modelled on POV-Ray's: each `"type"` here builds one
//! geometry from its JSON object. Kept apart from `scenefile.rs`, which falls through to
//! `parse_shape` for any type it doesn't know.

use crate::na::{Vector2, Vector3};
use crate::scenefile::SceneFile;
use crate::shapes::geometry::Geometry;
use crate::shapes::cone::Cone;
use crate::shapes::disc::Disc;
use crate::shapes::algebraic::{Algebraic, Term, Torus};
use crate::shapes::bbox::BBox;
use crate::shapes::polygon::{Outline, Polygon, Prism, Sweep};
use crate::shapes::revolution::{Lathe, Sor, Spline};
use crate::shapes::sweep::{SphereSweep, SweepSpline};
use crate::shapes::blob::{Blob, Component};
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
            // {"type": "torus", "major": R, "minor": r}: about the y axis, centred on the origin.
            "torus" => Box::new(Torus::new(num(o, "major"), num(o, "minor"))),
            // {"type": "quadric", "squares": [A, B, C], "mixed": [D, E, F], "linear": [G, H, I],
            //  "constant": J, "bounds": {"min": [..], "max": [..]}}
            "quadric" => Box::new(Algebraic::new(
                Algebraic::quadric_terms(
                    SceneFile::parse_vec3_def(o, "squares", Vector3::zeros()),
                    SceneFile::parse_vec3_def(o, "mixed", Vector3::zeros()),
                    SceneFile::parse_vec3_def(o, "linear", Vector3::zeros()),
                    num_def(o, "constant", 0.)),
                clip_box(o))),
            // {"type": "poly", "order": n, "coefficients": [..]} in POV-Ray's order, or
            // {"type": "poly", "terms": [[c, i, j, k], ..]} for c x^i y^j z^k; "cubic" and
            // "quartic" are poly of order 3 and 4. Optionally clipped to "bounds".
            "poly" | "cubic" | "quartic" => Box::new(Algebraic::new(poly_terms(t, o), clip_box(o))),
            // {"type": "polygon", "points": [[x, y, z], ..]}: coplanar points; a loop closes when
            // its first point repeats, and loops inside others are holes.
            "polygon" => Box::new(Polygon::new(&points3(o, "points"))),
            // {"type": "prism", "height1": 0, "height2": 1, "points": [[x, z], ..],
            //  "spline": "linear" | "bezier", "sweep": "linear" | "conic", "open": false}
            "prism" => {
                let points = points2(o, "points");
                let outline = match o["spline"].as_str().unwrap_or("linear") {
                    "linear" => Outline::from_points(&points),
                    "bezier" => Outline::from_bezier(&points, SceneFile::parse_int(&o["steps"], 16)),
                    s => panic!("prism spline must be \"linear\" or \"bezier\", got {}", s),
                };
                let sweep = match o["sweep"].as_str().unwrap_or("linear") {
                    "linear" => Sweep::Linear,
                    "conic" => Sweep::Conic,
                    s => panic!("prism sweep must be \"linear\" or \"conic\", got {}", s),
                };
                Box::new(Prism::new(outline, num(o, "height1"), num(o, "height2"), sweep, flag(o, "open")))
            },
            // {"type": "lathe", "points": [[radius, y], ..], "spline": "linear" | "cubic" | "bezier"}
            "lathe" => {
                let spline = match o["spline"].as_str().unwrap_or("linear") {
                    "linear" => Spline::Linear,
                    "cubic" => Spline::Cubic,
                    "bezier" => Spline::Bezier,
                    s => panic!("lathe spline must be \"linear\", \"cubic\" or \"bezier\", got {}", s),
                };
                Box::new(Lathe::new(&points2(o, "points"), spline))
            },
            // {"type": "sor", "points": [[radius, y], ..], "open": false}
            "sor" => Box::new(Sor::new(&points2(o, "points"), flag(o, "open"))),
            // {"type": "sphere_sweep", "spheres": [[x, y, z, radius], ..],
            //  "spline": "linear" | "cubic" | "b_spline", "steps": 16}
            "sphere_sweep" => {
                let spheres: Vec<(Vector3<f64>, f64)> = o["spheres"].as_array()
                    .expect("sphere_sweep needs \"spheres\": [[x, y, z, radius], ..]")
                    .iter().map(|s| {
                        let a: Vec<f64> = s.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default();
                        assert!(a.len() == 4, "sphere_sweep spheres are [x, y, z, radius], got {}", s);
                        (Vector3::new(a[0], a[1], a[2]), a[3])
                    }).collect();
                let spline = match o["spline"].as_str().unwrap_or("linear") {
                    "linear" => SweepSpline::Linear,
                    "cubic" => SweepSpline::Cubic,
                    "b_spline" => SweepSpline::BSpline,
                    s => panic!("sphere_sweep spline must be \"linear\", \"cubic\" or \"b_spline\", got {}", s),
                };
                Box::new(SphereSweep::new(&spheres, spline, SceneFile::parse_int(&o["steps"], 16)))
            },
            // {"type": "blob", "threshold": 0.5, "components": [
            //   {"type": "sphere", "center": [..], "radius": r, "strength": s},
            //   {"type": "cylinder", "base": [..], "cap": [..], "radius": r, "strength": s}]}
            "blob" => {
                let components = o["components"].as_array().expect("blob needs \"components\"")
                    .iter().map(|c| match c["type"].as_str() {
                        Some("sphere") => Component::Sphere {
                            center: vec3(c, "center"), radius: num(c, "radius"), strength: num_def(c, "strength", 1.) },
                        Some("cylinder") => Component::Cylinder {
                            base: vec3(c, "base"), cap: vec3(c, "cap"), radius: num(c, "radius"), strength: num_def(c, "strength", 1.) },
                        _ => panic!("blob components are \"sphere\" or \"cylinder\", got {}", c),
                    }).collect();
                Box::new(Blob::new(components, num(o, "threshold")))
            },
            _ => return None,
        })
    }
}

/// An optional box, {"min": [..], "max": [..]}, under "bounds".
fn clip_box(o: &Value) -> Option<BBox> {
    let b = &o["bounds"];
    if b.is_null() {
        return None;
    }
    let (a, c) = (vec3(b, "min"), vec3(b, "max"));
    Some(BBox::new(a.inf(&c), a.sup(&c)))
}

fn poly_terms(t: &str, o: &Value) -> Vec<Term> {
    if let Some(terms) = o["terms"].as_array() {
        return terms.iter().map(|term| {
            let a = term.as_array().filter(|a| a.len() == 4)
                .unwrap_or_else(|| panic!("{} terms are [coefficient, i, j, k], got {}", t, term));
            let power = |x: &Value| x.as_u64().unwrap_or_else(|| panic!("{}: powers must be whole numbers, got {}", t, x)) as usize;
            Term { coefficient: a[0].as_f64().unwrap(), powers: [power(&a[1]), power(&a[2]), power(&a[3])] }
        }).collect();
    }
    let order = match t {
        "cubic" => 3,
        "quartic" => 4,
        _ => SceneFile::parse_int(&o["order"], 0),
    };
    assert!(order >= 1, "poly needs an \"order\" (or \"terms\")");
    let coefficients: Vec<f64> = o["coefficients"].as_array()
        .unwrap_or_else(|| panic!("{} needs \"coefficients\" (or \"terms\")", t))
        .iter().map(|c| c.as_f64().unwrap()).collect();
    Algebraic::pov_terms(order, &coefficients)
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
    fn test_algebraic_shapes() {
        let t = shape(r#"{"type": "torus", "major": 2, "minor": 0.5}"#);
        assert!((hit_from(&t, [-10., 0., 0.], [1., 0., 0.]).unwrap() - 7.5).abs() < 1e-7);
        let q = shape(r#"{"type": "quadric", "squares": [1, 1, 1], "constant": -1}"#);
        assert!((hit_from(&q, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-7);
        // The same sphere as a poly, by terms and in POV-Ray's order.
        let p = shape(r#"{"type": "poly", "terms": [[1, 2, 0, 0], [1, 0, 2, 0], [1, 0, 0, 2], [-1, 0, 0, 0]]}"#);
        assert!((hit_from(&p, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-7);
        let p2 = shape(r#"{"type": "poly", "order": 2, "coefficients": [1, 0, 0, 0, 1, 0, 0, 1, 0, -1],
            "bounds": {"min": [-2, -2, -2], "max": [2, 2, 2]}}"#);
        assert!((hit_from(&p2, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-7);
    }

    #[test]
    fn test_flat_shapes() {
        let p = shape(r#"{"type": "polygon", "points": [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]]}"#);
        assert!((hit_from(&p, [0.5, 0.5, -5.], [0., 0., 1.]).unwrap() - 5.).abs() < 1e-9);
        let pr = shape(r#"{"type": "prism", "height1": 0, "height2": 1, "sweep": "conic",
            "points": [[-1, -1], [1, -1], [1, 1], [-1, 1]]}"#);
        assert!((hit_from(&pr, [-5., 0.5, 0.], [1., 0., 0.]).unwrap() - 4.5).abs() < 1e-9);
    }

    #[test]
    fn test_revolved_shapes() {
        let l = shape(r#"{"type": "lathe", "points": [[1, 0], [2, 0], [2, 1], [1, 1], [1, 0]]}"#);
        assert!((hit_from(&l, [-5., 0.5, 0.], [1., 0., 0.]).unwrap() - 3.).abs() < 1e-9);
        let s = shape(r#"{"type": "sor", "points": [[1, -1], [1, 0], [1, 2], [1, 3]]}"#);
        assert!((hit_from(&s, [-5., 1., 0.], [1., 0., 0.]).unwrap() - 4.).abs() < 1e-9);
    }

    #[test]
    fn test_sweep_and_blob() {
        let s = shape(r#"{"type": "sphere_sweep", "spheres": [[0, 0, 0, 1], [4, 0, 0, 1]]}"#);
        assert!((hit_from(&s, [2., 5., 0.], [0., -1., 0.]).unwrap() - 4.).abs() < 1e-9);
        let b = shape(r#"{"type": "blob", "threshold": 0.25, "components": [
            {"type": "sphere", "center": [0, 0, 0], "radius": 2, "strength": 1},
            {"type": "cylinder", "base": [5, 0, 0], "cap": [9, 0, 0], "radius": 1}]}"#);
        let expected = 10. - 2. * (0.5f64).sqrt();
        assert!((hit_from(&b, [0., 0., -10.], [0., 0., 1.]).unwrap() - expected).abs() < 1e-7);
    }

    #[test]
    #[should_panic(expected = "needs a number \"radius\"")]
    fn test_missing_field_is_reported() {
        shape(r#"{"type": "cylinder", "base": [0, 0, 0], "cap": [0, 2, 0]}"#);
    }
}
