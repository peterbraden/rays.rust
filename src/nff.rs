//! Reading NFF, Eric Haines' Neutral File Format.
//!
//! NFF is the format of the Standard Procedural Database (SPD), the scenes Haines proposed as a
//! common benchmark for ray tracing efficiency schemes: Haines, "A Proposal for Standard Graphics
//! Environments", IEEE Computer Graphics and Applications 7(11), November 1987, pp. 3-5. The
//! format is specified in `NFF.TXT` (version 3.9, 1996), which ships with the SPD generators at
//! <https://github.com/erich666/StandardProceduralDatabases> (CC0, with an MIT fallback).
//!
//! A file is lines of text, each entity introduced by a keyword, with no nesting and no
//! transforms. "#" starts a comment line. The entities, and what each becomes here:
//!
//! - `v` then the lines `from x y z`, `at x y z`, `up x y z`, `angle a`, `hither h`,
//!   `resolution w h`: a pinhole camera (the camera section below).
//! - `b r g b`: the background colour.
//! - `l x y z [r g b]`: a point light (see "Lights" below).
//! - `f r g b Kd Ks Shine T ior`: the surface for the objects that follow, until the next `f`.
//!   Kd and Ks weight the diffuse and specular terms, Shine is the Phong exponent, T the
//!   fraction transmitted, ior the index of refraction. With T = 0 this is our `whitted`
//!   material, which implements the same model: pigment rgb x Kd, mirror reflection Ks, and a
//!   Phong highlight of exponent Shine. With T > 0 it becomes a `dielectric` of index ior,
//!   tinted by rgb: our dielectric has no separate diffuse term, so Kd and Ks are dropped.
//! - `s x y z r`: a sphere. A negative radius means only the inside is visible; ours are
//!   two-sided, so it becomes the absolute value.
//! - `p n` then n vertices: a convex planar polygon, counter-clockwise from its front. It
//!   becomes a fan of triangles, keeping the winding.
//! - `pp n` then n lines of vertex and normal: a polygon with a normal at each vertex. The
//!   normals are dropped, as our scene format has no triangle with vertex normals, so these
//!   render faceted (the SPD teapot is made of them).
//! - `c` then `base.x base.y base.z base_radius` and `apex.x apex.y apex.z apex_radius`: a
//!   cone or cylinder with no end caps. It becomes a `cone` with `"open": true`, which needs
//!   the cone shape from the POV-Ray shapes (until that lands, a scene with cones fails to
//!   load with an unknown object type).
//!
//! # Camera
//!
//! NFF's `angle` is in degrees, "from the center of top pixel row to bottom pixel row and left
//! column to right column", and the aspect ratio is always 1. Measured between pixel centres,
//! the half-height of the image plane at unit distance is h = tan(angle / 2) x yres / (yres - 1).
//! Our camera's `angle` is the one whose tangent is the full height at unit distance, so it is
//! atan(2h). `hither` (a near clipping plane, for hidden surface algorithms) is ignored.
//!
//! # Lights
//!
//! NFF says lights "have a non-zero intensity of no particular value": shading is left to the
//! renderer. The SPD generators scale each of their N lights by 1/sqrt(N) when writing for
//! renderers that take an intensity (POV-Ray, for example), and by 1 for NFF itself. Our point
//! lights fall off with the inverse square of distance, with irradiance I/(pi d^2) at distance d
//! (`material::functions::diffuse`), so we pick I = pi d^2 / sqrt(N), with d the distance from the
//! light to the camera's `at` point: a surface there facing the light gets the SPD's 1/sqrt(N).
//! Absolute brightness is not comparable with any other renderer's image of the scene.
//!
//! The rest of the scene file (sample counts, recursion depth) takes our defaults, except
//! `max_depth`, set to `MAX_DEPTH` so that reflections and refraction are followed.

use serde_json::{json, Map, Value};
use std::f64::consts::PI;

/// Recursion depth for the reflection and refraction rays NFF's surfaces spawn.
pub const MAX_DEPTH: usize = 5;

/// The `f` surface in effect before any is given (NFF does not say): white, purely diffuse.
const DEFAULT_SURFACE: [f64; 8] = [1., 1., 1., 1., 0., 0., 0., 1.];

/// Tokens of the file, a line at a time, skipping comments and blank lines.
struct Tokens<'a> {
    lines: Vec<(usize, Vec<&'a str>)>,
    line: usize,
    pos: usize,
}

impl<'a> Tokens<'a> {
    fn new(src: &'a str) -> Tokens<'a> {
        let lines = src.lines().enumerate()
            .filter(|(_, l)| !l.trim_start().starts_with('#'))
            .map(|(i, l)| (i + 1, l.split_whitespace().collect::<Vec<_>>()))
            .filter(|(_, t)| !t.is_empty())
            .collect();
        Tokens { lines, line: 0, pos: 0 }
    }

    /// The line number of the next token, for messages.
    fn line_number(&self) -> usize {
        self.lines.get(self.line).map_or_else(|| self.lines.last().map_or(0, |l| l.0), |l| l.0)
    }

    fn next(&mut self) -> Option<&'a str> {
        while self.line < self.lines.len() {
            let tokens = &self.lines[self.line].1;
            if self.pos < tokens.len() {
                self.pos += 1;
                return Some(tokens[self.pos - 1]);
            }
            self.line += 1;
            self.pos = 0;
        }
        None
    }

    /// Whether more tokens remain on the current line.
    fn more_on_line(&self) -> bool {
        self.lines.get(self.line).is_some_and(|l| self.pos < l.1.len())
    }

    fn word(&mut self, what: &str) -> Result<&'a str, String> {
        let line = self.line_number();
        self.next().ok_or_else(|| format!("line {}: expected {}, found the end of the file", line, what))
    }

    fn number(&mut self, what: &str) -> Result<f64, String> {
        let line = self.line_number();
        let t = self.word(what)?;
        t.parse::<f64>().map_err(|_| format!("line {}: expected {} as a number, got \"{}\"", line, what, t))
    }

    fn numbers<const N: usize>(&mut self, what: &str) -> Result<[f64; N], String> {
        let mut out = [0.; N];
        for x in out.iter_mut() {
            *x = self.number(what)?;
        }
        Ok(out)
    }

    fn count(&mut self, what: &str) -> Result<usize, String> {
        let line = self.line_number();
        let n = self.number(what)?;
        if n < 0. || n.fract() != 0. {
            return Err(format!("line {}: expected {} as a count, got {}", line, what, n));
        }
        Ok(n as usize)
    }

    /// The keyword `name`, then three numbers.
    fn labelled3(&mut self, name: &str) -> Result<[f64; 3], String> {
        self.label(name)?;
        self.numbers::<3>(name)
    }

    fn label(&mut self, name: &str) -> Result<(), String> {
        let line = self.line_number();
        match self.word(name)? {
            w if w == name => Ok(()),
            w => Err(format!("line {}: expected \"{}\" in the view, got \"{}\"", line, name, w)),
        }
    }
}

/// The view: where the camera is and how it looks.
struct View {
    from: [f64; 3],
    at: [f64; 3],
    up: [f64; 3],
    angle: f64,
    resolution: (usize, usize),
}

/// The part of the scene file NFF determines: camera, background, lights, materials, objects.
pub struct Nff {
    view: Option<View>,
    background: [f64; 3],
    /// Positions and optional colours.
    lights: Vec<([f64; 3], Option<[f64; 3]>)>,
    /// Material name and JSON, in the order first used.
    materials: Vec<(String, Value)>,
    objects: Vec<Value>,
    /// Number of `pp` patches whose normals were dropped.
    patches: usize,
}

/// The name of an `f` surface's material, adding it if new.
fn material_for(f: &[f64; 8], materials: &mut Vec<(String, Value)>) -> String {
    let [r, g, b, kd, ks, shine, t, ior] = *f;
    let m = if t > 0. {
        json!({ "type": "dielectric", "refractive_index": ior, "attenuate": [r, g, b] })
    } else {
        json!({ "type": "whitted", "pigment": [r * kd, g * kd, b * kd], "reflection": ks, "phong": shine })
    };
    if let Some((name, _)) = materials.iter().find(|(_, v)| *v == m) {
        return name.clone();
    }
    let name = format!("nff{}", materials.len());
    materials.push((name.clone(), m));
    name
}

/// A fan of triangles from a convex polygon's vertices.
fn fan(vertices: &[[f64; 3]], material: &str) -> Vec<Value> {
    (1 .. vertices.len().saturating_sub(1)).map(|i| json!({
        "type": "triangle",
        "v0": vertices[0], "v1": vertices[i], "v2": vertices[i + 1],
        "material": material,
    })).collect()
}

impl Nff {
    pub fn parse(src: &str) -> Result<Nff, String> {
        let mut t = Tokens::new(src);
        let mut nff = Nff { view: None, background: [0., 0., 0.], lights: Vec::new(), materials: Vec::new(), objects: Vec::new(), patches: 0 };
        let mut surface = material_for(&DEFAULT_SURFACE, &mut nff.materials);

        while let Some(keyword) = t.next() {
            let line = t.line_number();
            match keyword {
                "v" => {
                    let from = t.labelled3("from")?;
                    let at = t.labelled3("at")?;
                    let up = t.labelled3("up")?;
                    t.label("angle")?;
                    let angle = t.number("angle")?;
                    t.label("hither")?;
                    t.number("hither")?;
                    t.label("resolution")?;
                    let resolution = (t.count("resolution")?, t.count("resolution")?);
                    nff.view = Some(View { from, at, up, angle, resolution });
                },
                "b" => nff.background = t.numbers::<3>("background colour")?,
                "l" => {
                    let position = t.numbers::<3>("light position")?;
                    // The colour is optional, so it is read only from the same line.
                    let colour = if t.more_on_line() { Some(t.numbers::<3>("light colour")?) } else { None };
                    nff.lights.push((position, colour));
                },
                "f" => surface = material_for(&t.numbers::<8>("surface")?, &mut nff.materials),
                "s" => {
                    let [x, y, z, r] = t.numbers::<4>("sphere")?;
                    nff.objects.push(json!({ "type": "sphere", "location": [x, y, z], "radius": r.abs(), "material": surface }));
                },
                "p" | "pp" => {
                    let n = t.count("vertex count")?;
                    let mut vertices = Vec::with_capacity(n);
                    for _ in 0 .. n {
                        vertices.push(t.numbers::<3>("polygon vertex")?);
                        if keyword == "pp" {
                            t.numbers::<3>("polygon vertex normal")?;
                        }
                    }
                    if n < 3 {
                        return Err(format!("line {}: a polygon needs at least 3 vertices, got {}", line, n));
                    }
                    if keyword == "pp" {
                        nff.patches += 1;
                    }
                    nff.objects.extend(fan(&vertices, &surface));
                },
                "c" => {
                    let [bx, by, bz, br] = t.numbers::<4>("cone base")?;
                    let [ax, ay, az, ar] = t.numbers::<4>("cone apex")?;
                    nff.objects.push(json!({
                        "type": "cone",
                        "base": [bx, by, bz], "base_radius": br.abs(),
                        "cap": [ax, ay, az], "cap_radius": ar.abs(),
                        "open": true,
                        "material": surface,
                    }));
                },
                k => return Err(format!("line {}: unknown NFF entity \"{}\"", line, k)),
            }
        }
        Ok(nff)
    }

    /// Our scene file for this NFF scene.
    pub fn to_scene(&self) -> Result<Value, String> {
        let v = self.view.as_ref().ok_or("NFF file has no view (\"v\")")?;
        let (width, height) = v.resolution;
        // The angle spans pixel centres: scale up to the image's edges.
        let edges = if height > 1 { height as f64 / (height - 1) as f64 } else { 1. };
        let half_height = (v.angle.to_radians() / 2.).tan() * edges;
        let angle = (2. * half_height).atan();

        let at = crate::na::Vector3::from(v.at);
        let scale = 1. / (self.lights.len().max(1) as f64).sqrt();
        let lights: Vec<Value> = self.lights.iter().map(|(p, colour)| {
            let d2 = (crate::na::Vector3::from(*p) - at).norm_squared();
            let mut l = json!({ "location": p, "intensity": PI * d2 * scale });
            if let Some(c) = colour {
                l["color"] = json!(c);
            }
            l
        }).collect();

        let materials: Map<String, Value> = self.materials.iter().cloned().collect();
        Ok(json!({
            "width": width,
            "height": height,
            "max_depth": MAX_DEPTH,
            "background": self.background,
            "camera": {
                "location": v.from,
                "lookat": v.at,
                "up": v.up,
                "angle": angle,
                "aperture": 0,
            },
            "lights": lights,
            "materials": materials,
            "objects": self.objects,
        }))
    }
}

/// Translate an NFF scene into our scene file format.
pub fn to_scene(src: &str) -> Result<Value, String> {
    let nff = Nff::parse(src)?;
    if nff.patches > 0 {
        println!("Warning: NFF: dropped the vertex normals of {} polygonal patches; they render faceted", nff.patches);
    }
    nff.to_scene()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;

    const VIEW: &str = "v\nfrom 0 -10 0\nat 0 0 0\nup 0 0 1\nangle 45\nhither 1\nresolution 4 3\n";

    fn scene(body: &str) -> Value {
        to_scene(&format!("{}{}", VIEW, body)).unwrap()
    }

    #[test]
    fn test_view_becomes_camera() {
        let s = scene("");
        assert_eq!(s["width"], 4);
        assert_eq!(s["height"], 3);
        assert_eq!(s["camera"]["location"], json!([0., -10., 0.]));
        assert_eq!(s["camera"]["up"], json!([0., 0., 1.]));
        assert_eq!(s["camera"]["aperture"], 0);
        // Pixel centres span 45 degrees over 2 of the 3 rows' heights.
        let expected = (2. * (22.5f64).to_radians().tan() * 1.5).atan();
        assert!((s["camera"]["angle"].as_f64().unwrap() - expected).abs() < 1e-12);
    }

    #[test]
    fn test_comments_background_and_lights() {
        let s = scene("# a comment\nb 0.1 0.2 0.3\nl 0 0 10\nl 0 10 0 1 0 0\n");
        assert_eq!(s["background"], json!([0.1, 0.2, 0.3]));
        let lights = s["lights"].as_array().unwrap();
        assert_eq!(lights.len(), 2);
        assert!(lights[0].get("color").is_none());
        assert_eq!(lights[1]["color"], json!([1., 0., 0.]));
        // Two lights, each 10 from the "at" point: pi 10^2 / sqrt(2).
        let expected = PI * 100. / 2f64.sqrt();
        assert!((lights[0]["intensity"].as_f64().unwrap() - expected).abs() < 1e-9);
    }

    #[test]
    fn test_surfaces_become_materials() {
        let s = scene("f 1 0.5 0 0.8 0.2 30 0 1\ns 0 0 0 1\nf 1 1 1 0 0 0 0.9 1.5\ns 0 0 2 -1\nf 1 0.5 0 0.8 0.2 30 0 1\ns 0 0 4 1\n");
        let objects = s["objects"].as_array().unwrap();
        let m0 = objects[0]["material"].as_str().unwrap();
        let m1 = objects[1]["material"].as_str().unwrap();
        assert_eq!(objects[2]["material"].as_str().unwrap(), m0, "identical surfaces share a material");
        assert_eq!(s["materials"][m0], json!({ "type": "whitted", "pigment": [0.8, 0.4, 0.], "reflection": 0.2, "phong": 30. }));
        assert_eq!(s["materials"][m1]["type"], "dielectric");
        assert_eq!(s["materials"][m1]["refractive_index"], 1.5);
        assert_eq!(objects[1]["radius"], 1., "inside-only spheres become ordinary ones");
    }

    #[test]
    fn test_polygons_become_fans() {
        let s = scene("p 4\n0 0 0\n1 0 0\n1 1 0\n0 1 0\npp 3\n0 0 0 0 0 1\n1 0 0 0 0 1\n1 1 0 0 0 1\n");
        let objects = s["objects"].as_array().unwrap();
        assert_eq!(objects.len(), 3);
        assert_eq!(objects[1]["v0"], json!([0., 0., 0.]));
        assert_eq!(objects[1]["v1"], json!([1., 1., 0.]));
        assert_eq!(objects[1]["v2"], json!([0., 1., 0.]));
    }

    #[test]
    fn test_cones_are_open() {
        let s = scene("c\n0 0 0 1\n0 0 2 -0.5\n");
        assert_eq!(s["objects"][0], json!({
            "type": "cone", "base": [0., 0., 0.], "base_radius": 1., "cap": [0., 0., 2.], "cap_radius": 0.5,
            "open": true, "material": "nff0",
        }));
    }

    #[test]
    fn test_errors_name_the_line() {
        assert!(to_scene("s 0 0 0 1\n").unwrap_err().contains("no view"));
        let e = to_scene(&format!("{}s 0 0 x 1\n", VIEW)).unwrap_err();
        assert!(e.contains("line 8") && e.contains("\"x\""), "{}", e);
        assert!(to_scene(&format!("{}q 1\n", VIEW)).unwrap_err().contains("unknown NFF entity \"q\""));
        assert!(to_scene(&format!("{}p 2\n0 0 0\n1 0 0\n", VIEW)).unwrap_err().contains("at least 3"));
    }

    #[test]
    fn test_translation_loads() {
        let doc = scene("b 0 0 0\nl 0 -5 5\nf 1 0 0 1 0 0 0 1\ns 0 0 0 1\np 3\n-1 0 -1\n1 0 -1\n0 0 1\n");
        let s = SceneFile::from_value_overridden(doc, &Default::default());
        assert_eq!(s.lights.len(), 1);
        assert_eq!(s.image.width, 4);
    }
}
