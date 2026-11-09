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
use crate::shapes::implicit::{Container, Isosurface, Superellipsoid};
use crate::shapes::julia::Julia;
use crate::shapes::tessellated::{self, Heights};
use crate::expr::Expr;
use crate::na::Vector4;
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
            // {"type": "isosurface", "function": "x*x + y*y + z*z - 1", "threshold": 0,
            //  "contained_by": {"box": {"min": [..], "max": [..]}} or {"sphere": {"center": [..], "radius": r}},
            //  "max_gradient": 1.1, "accuracy": 0.001, "open": false}
            "isosurface" => {
                let text = o["function"].as_str().expect("isosurface needs a \"function\" of x, y and z");
                let function = Expr::parse(text, &["x", "y", "z"])
                    .unwrap_or_else(|e| panic!("isosurface function \"{}\": {}", text, e));
                let c = &o["contained_by"];
                let container = if !c["sphere"].is_null() {
                    Container::Sphere { center: vec3(&c["sphere"], "center"), radius: num(&c["sphere"], "radius") }
                } else if !c["box"].is_null() {
                    let (a, b) = (vec3(&c["box"], "min"), vec3(&c["box"], "max"));
                    Container::Box(BBox::new(a.inf(&b), a.sup(&b)))
                } else {
                    // POV-Ray's default container.
                    Container::Box(BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.)))
                };
                Box::new(Isosurface::new(function, num_def(o, "threshold", 0.), container,
                    num_def(o, "max_gradient", 1.1), num_def(o, "accuracy", 0.001), flag(o, "open")))
            },
            // {"type": "superellipsoid", "e": 0.5, "n": 0.5}: within the cube from -1 to 1.
            "superellipsoid" => Box::new(Superellipsoid::new(num(o, "e"), num(o, "n"))),
            // {"type": "julia_fractal", "c": [a, b, c, d], "slice": 0, "max_iteration": 20, "accuracy": 0.0001}
            "julia_fractal" => {
                let c: Vec<f64> = o["c"].as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default();
                assert!(c.len() == 4, "julia_fractal needs a quaternion \"c\": [a, b, c, d], got {}", o["c"]);
                Box::new(Julia::new(Vector4::new(c[0], c[1], c[2], c[3]), num_def(o, "slice", 0.),
                    SceneFile::parse_int(&o["max_iteration"], 20), num_def(o, "accuracy", 1e-4)))
            },
            // {"type": "height_field", "src": "heights.png"} or
            // {"type": "height_field", "function": "f(x, z)", "resolution": [w, d]}, with
            // "smooth": false, "water_level": 0. Over the unit square in x and z, heights 0 to 1.
            "height_field" => {
                let heights = match (o["src"].as_str(), o["function"].as_str()) {
                    (Some(src), _) => Heights::from_image(src),
                    (None, Some(text)) => {
                        let f = Expr::parse(text, &["x", "z"])
                            .unwrap_or_else(|e| panic!("height_field function \"{}\": {}", text, e));
                        let (w, d) = resolution(o, 128);
                        Heights::from_function(&f, w + 1, d + 1)
                    },
                    _ => panic!("height_field needs an image \"src\" or a \"function\" of x and z"),
                };
                tessellated::height_field(&heights, flag(o, "smooth"), num_def(o, "water_level", 0.))
            },
            // {"type": "parametric", "x": "f(u, v)", "y": "..", "z": "..", "u": [u0, u1],
            //  "v": [v0, v1], "resolution": [nu, nv]}
            "parametric" => {
                let f = |k: &str| {
                    let text = o[k].as_str().unwrap_or_else(|| panic!("parametric needs \"{}\" as a function of u and v", k));
                    Expr::parse(text, &["u", "v"]).unwrap_or_else(|e| panic!("parametric {} \"{}\": {}", k, text, e))
                };
                let (x, y, z) = (f("x"), f("y"), f("z"));
                let range = |k: &str| {
                    let r = SceneFile::parse_vec2_def(o, k, crate::na::Vector2::new(0., 1.));
                    (r.x, r.y)
                };
                let (nu, nv) = resolution(o, 64);
                Box::new(tessellated::parametric([&x, &y, &z], range("u"), range("v"), nu, nv))
            },
            // {"type": "bicubic_patch", "points": [16 points, 4 rows of 4], "steps": 16}
            "bicubic_patch" => Box::new(tessellated::bicubic_patch(&points3(o, "points"), SceneFile::parse_int(&o["steps"], 16))),
            // {"type": "text", "font": "path/to/font.ttf", "string": "Hello", "thickness": 0.2,
            //  "offset": [0, 0], "steps": 8}: on the baseline y = 0 from x = 0, about one unit
            //  high, from z = 0 back to z = thickness.
            "text" => {
                let font = o["font"].as_str().expect("text needs a \"font\" file");
                let string = o["string"].as_str().expect("text needs a \"string\"");
                text_shape(font, string, o)
            },
            _ => return None,
        })
    }
}

fn text_shape(font: &str, string: &str, o: &Value) -> Shape {
    crate::shapes::text::text(font, string, num_def(o, "thickness", 1.),
        SceneFile::parse_vec2_def(o, "offset", Vector2::zeros()), SceneFile::parse_int(&o["steps"], 8))
}

/// A grid size, [nu, nv], under "resolution".
fn resolution(o: &Value, def: usize) -> (usize, usize) {
    match o["resolution"].as_array() {
        Some(a) if a.len() == 2 => (SceneFile::parse_int(&a[0], def), SceneFile::parse_int(&a[1], def)),
        None => (def, def),
        _ => panic!("{} \"resolution\" is [nu, nv], got {}", o["type"], o["resolution"]),
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
    fn test_implicit_shapes() {
        let i = shape(r#"{"type": "isosurface", "function": "x^2 + y^2 + z^2 - 1", "max_gradient": 4,
            "contained_by": {"sphere": {"center": [0, 0, 0], "radius": 2}}, "accuracy": 1e-6}"#);
        assert!((hit_from(&i, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-5);
        let s = shape(r#"{"type": "superellipsoid", "e": 1, "n": 1}"#);
        assert!((hit_from(&s, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-5);
        let j = shape(r#"{"type": "julia_fractal", "c": [0, 0, 0, 0], "max_iteration": 30, "accuracy": 1e-5}"#);
        assert!((hit_from(&j, [0., 0., -5.], [0., 0., 1.]).unwrap() - 4.).abs() < 1e-3);
    }

    #[test]
    fn test_tessellated_shapes() {
        let h = shape(r#"{"type": "height_field", "function": "0.5 * x", "resolution": [10, 10], "smooth": true}"#);
        assert!((hit_from(&h, [0.43, 5., 0.47], [0., -1., 0.]).unwrap() - (5. - 0.215)).abs() < 1e-9);
        let p = shape(r#"{"type": "parametric", "x": "u", "y": "0", "z": "v", "u": [-1, 1], "v": [-1, 1], "resolution": [4, 4]}"#);
        assert!((hit_from(&p, [0.3, 5., 0.2], [0., -1., 0.]).unwrap() - 5.).abs() < 1e-9);
        let b = shape(r#"{"type": "bicubic_patch", "steps": 4, "points": [
            [0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [0, 0, 1], [1, 0, 1], [2, 0, 1], [3, 0, 1],
            [0, 0, 2], [1, 0, 2], [2, 0, 2], [3, 0, 2], [0, 0, 3], [1, 0, 3], [2, 0, 3], [3, 0, 3]]}"#);
        assert!((hit_from(&b, [1.3, 5., 2.2], [0., -1., 0.]).unwrap() - 5.).abs() < 1e-9);
    }

    #[test]
    fn test_csg_in_scene_files() {
        let sphere = |x: f64| format!(r#"{{"type": "sphere", "location": [{}, 0, 0], "radius": 1}}"#, x);
        let lens = shape(&format!(r#"{{"type": "intersection", "items": [{}, {}]}}"#, sphere(-0.5), sphere(0.5)));
        assert!((hit_from(&lens, [-5., 0., 0.], [1., 0., 0.]).unwrap() - 4.5).abs() < 1e-9);
        let merged = shape(&format!(r#"{{"type": "merge", "items": [{}, {}]}}"#, sphere(-0.5), sphere(0.5)));
        assert!((hit_from(&merged, [0., 0., 0.], [1., 0., 0.]).unwrap() - 1.5).abs() < 1e-9);
        // The old two-shape difference still reads.
        let bitten = shape(&format!(r#"{{"type": "difference", "a": {}, "b": {}}}"#, sphere(0.), sphere(1.)));
        assert!((hit_from(&bitten, [5., 0., 0.], [-1., 0., 0.]).unwrap() - 5.).abs() < 1e-9);
        // A half-space plane, clipping a sphere to a bowl, and inverse.
        let bowl = shape(&format!(r#"{{"type": "sphere", "location": [0, 0, 0], "radius": 1,
            "clipped_by": {{"type": "plane", "normal": [0, 1, 0], "distance": 0}}}}"#));
        assert!((hit_from(&bowl, [0., 5., 0.], [0., -1., 0.]).unwrap() - 6.).abs() < 1e-9);
        let inv = shape(&format!(r#"{{"type": "union", "inverse": true, "items": [{}]}}"#, sphere(0.)));
        assert!(inv.inside(&Vector3::new(3., 0., 0.)));
    }

    #[test]
    #[should_panic(expected = "unknown name 'w'")]
    fn test_bad_function_is_reported() {
        shape(r#"{"type": "isosurface", "function": "x + w"}"#);
    }

    #[test]
    #[should_panic(expected = "needs a number \"radius\"")]
    fn test_missing_field_is_reported() {
        shape(r#"{"type": "cylinder", "base": [0, 0, 0], "cap": [0, 2, 0]}"#);
    }
}
