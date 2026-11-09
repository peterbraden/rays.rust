//! # PLY meshes
//!
//! PLY, the "Polygon File Format" or "Stanford Triangle Format", was designed at the Stanford
//! Computer Graphics Laboratory for the meshes its laser scanners produced: the bunny, the
//! dragon, the Buddha and the Digital Michelangelo models are all distributed as PLY. The
//! specification is Greg Turk, "The PLY Polygon File Format", 1994
//! (<http://gamma.cs.unc.edu/POWERPLANT/papers/ply.pdf>; also at
//! <https://paulbourke.net/dataformats/ply/>). The Library of Congress keeps a history of the
//! format: <https://www.loc.gov/preservation/digital/formats/fdd/fdd000501.shtml>.
//!
//! A file is an ASCII header followed by data, in ASCII or binary:
//!
//! ```text
//! ply
//! format ascii 1.0                   # or binary_little_endian 1.0, binary_big_endian 1.0
//! comment made by Greg Turk          # comment lines are ignored
//! element vertex 8                   # 8 vertices follow, each with these properties:
//! property float x
//! property float y
//! property float z
//! element face 6                     # then 6 faces, each a list:
//! property list uchar int vertex_index   # a uchar count, then that many ints
//! end_header
//! ```
//!
//! Elements are stored in the order declared, each element's properties in the order declared.
//! The spec defines eight scalar types: `char`, `uchar`, `short`, `ushort`, `int`, `uint`,
//! `float` and `double`, of 1, 1, 2, 2, 4, 4, 4 and 8 bytes. Later writers also use sized names
//! (`int8`, `uint8`, ..., `float32`, `float64`); we accept both.
//!
//! We read what a renderer needs and skip the rest: vertex positions (`x`, `y`, `z`), vertex
//! normals if present (`nx`, `ny`, `nz`), and faces (`vertex_indices`, or `vertex_index` as the
//! spec's own example spells it). Faces with more than three vertices are split into a fan of
//! triangles, which is correct for convex polygons; scanned meshes are triangles already. Other
//! elements (edges, materials, range-grid data) and properties (colour, confidence) are read past.
use crate::na::Vector3;
use crate::shapes::mesh::IndexedMesh;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Scalar { I8, U8, I16, U16, I32, U32, F32, F64 }

impl Scalar {
    fn parse(name: &str) -> Result<Scalar, String> {
        Ok(match name {
            "char" | "int8" => Scalar::I8,
            "uchar" | "uint8" => Scalar::U8,
            "short" | "int16" => Scalar::I16,
            "ushort" | "uint16" => Scalar::U16,
            "int" | "int32" => Scalar::I32,
            "uint" | "uint32" => Scalar::U32,
            "float" | "float32" => Scalar::F32,
            "double" | "float64" => Scalar::F64,
            _ => return Err(format!("unknown PLY type \"{}\"", name)),
        })
    }

    fn size(self) -> usize {
        match self {
            Scalar::I8 | Scalar::U8 => 1,
            Scalar::I16 | Scalar::U16 => 2,
            Scalar::I32 | Scalar::U32 | Scalar::F32 => 4,
            Scalar::F64 => 8,
        }
    }
}

enum Property {
    Scalar(String, Scalar),
    /// A list: the type of its count, then the type of its items.
    List(String, Scalar, Scalar),
}

struct Element {
    name: String,
    count: usize,
    properties: Vec<Property>,
}

#[derive(Clone, Copy, PartialEq)]
enum Format { Ascii, Little, Big }

/// The data after the header, read one value at a time.
enum Values<'a> {
    Ascii(std::str::SplitAsciiWhitespace<'a>),
    Binary { data: &'a [u8], at: usize, big: bool },
}

impl Values<'_> {
    fn next(&mut self, t: Scalar) -> Result<f64, String> {
        match self {
            Values::Ascii(words) => {
                let w = words.next().ok_or("PLY data ends early")?;
                w.parse::<f64>().map_err(|_| format!("bad PLY value \"{}\"", w))
            },
            Values::Binary { data, at, big } => {
                let n = t.size();
                let b = data.get(*at .. *at + n).ok_or("PLY data ends early")?;
                *at += n;
                let mut bytes = [0u8; 8];
                bytes[.. n].copy_from_slice(b);
                if *big { bytes[.. n].reverse(); }
                Ok(match t {
                    Scalar::I8 => bytes[0] as i8 as f64,
                    Scalar::U8 => bytes[0] as f64,
                    Scalar::I16 => i16::from_le_bytes([bytes[0], bytes[1]]) as f64,
                    Scalar::U16 => u16::from_le_bytes([bytes[0], bytes[1]]) as f64,
                    Scalar::I32 => i32::from_le_bytes(bytes[.. 4].try_into().unwrap()) as f64,
                    Scalar::U32 => u32::from_le_bytes(bytes[.. 4].try_into().unwrap()) as f64,
                    Scalar::F32 => f32::from_le_bytes(bytes[.. 4].try_into().unwrap()) as f64,
                    Scalar::F64 => f64::from_le_bytes(bytes),
                })
            },
        }
    }
}

fn parse_header(text: &str) -> Result<(Format, Vec<Element>), String> {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some("ply") {
        return Err("not a PLY file: it doesn't start with \"ply\"".into());
    }
    let mut format = None;
    let mut elements: Vec<Element> = Vec::new();
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.as_slice() {
            ["format", f, _version] => format = Some(match *f {
                "ascii" => Format::Ascii,
                "binary_little_endian" => Format::Little,
                "binary_big_endian" => Format::Big,
                _ => return Err(format!("unknown PLY format \"{}\"", f)),
            }),
            ["element", name, count] => elements.push(Element {
                name: name.to_string(),
                count: count.parse().map_err(|_| format!("bad element count \"{}\"", count))?,
                properties: Vec::new(),
            }),
            ["property", "list", count, item, name] => elements.last_mut()
                .ok_or("PLY property before any element")?
                .properties.push(Property::List(name.to_string(), Scalar::parse(count)?, Scalar::parse(item)?)),
            ["property", t, name] => elements.last_mut()
                .ok_or("PLY property before any element")?
                .properties.push(Property::Scalar(name.to_string(), Scalar::parse(t)?)),
            ["end_header"] => return Ok((format.ok_or("PLY header has no format line")?, elements)),
            ["comment", ..] | ["obj_info", ..] | [] => {},
            _ => return Err(format!("unexpected PLY header line \"{}\"", line)),
        }
    }
    Err("PLY header has no end_header".into())
}

/// Parse PLY data held in memory.
pub fn parse(bytes: &[u8]) -> Result<IndexedMesh, String> {
    const END: &[u8] = b"end_header";
    let end = bytes.windows(END.len()).position(|w| w == END).ok_or("PLY header has no end_header")?;
    // The data starts after the end of the end_header line.
    let start = end + bytes[end ..].iter().position(|&b| b == b'\n').ok_or("PLY data missing")? + 1;
    let header = std::str::from_utf8(&bytes[.. start]).map_err(|_| "PLY header isn't text")?;
    let (format, elements) = parse_header(header)?;

    let mut values = match format {
        Format::Ascii => Values::Ascii(std::str::from_utf8(&bytes[start ..])
            .map_err(|_| "ASCII PLY data isn't text")?.split_ascii_whitespace()),
        _ => Values::Binary { data: &bytes[start ..], at: 0, big: format == Format::Big },
    };

    let mut mesh = IndexedMesh { positions: Vec::new(), normals: Vec::new(), triangles: Vec::new() };
    for e in &elements {
        let is_vertex = e.name == "vertex";
        let is_face = e.name == "face";
        let has_normals = e.properties.iter().any(|p| matches!(p, Property::Scalar(n, _) if n == "nx"));
        for _ in 0 .. e.count {
            let (mut p, mut n) = (Vector3::zeros(), Vector3::zeros());
            for prop in &e.properties {
                match prop {
                    Property::Scalar(name, t) => {
                        let v = values.next(*t)?;
                        if is_vertex {
                            match name.as_str() {
                                "x" => p.x = v, "y" => p.y = v, "z" => p.z = v,
                                "nx" => n.x = v, "ny" => n.y = v, "nz" => n.z = v,
                                _ => {},
                            }
                        }
                    },
                    Property::List(name, count, item) => {
                        let k = values.next(*count)? as usize;
                        let mut list = Vec::with_capacity(k);
                        for _ in 0 .. k {
                            list.push(values.next(*item)? as usize);
                        }
                        if is_face && (name == "vertex_indices" || name == "vertex_index") {
                            for i in 1 .. k.saturating_sub(1) {
                                mesh.triangles.push([list[0], list[i], list[i + 1]]);
                            }
                        }
                    },
                }
            }
            if is_vertex {
                mesh.positions.push(p);
                if has_normals {
                    mesh.normals.push(n);
                }
            }
        }
    }

    let vertices = mesh.positions.len();
    if let Some(t) = mesh.triangles.iter().find(|t| t.iter().any(|&i| i >= vertices)) {
        return Err(format!("PLY face {:?} refers past the {} vertices", t, vertices));
    }
    Ok(mesh)
}

/// Read a PLY file.
pub fn read(path: &str) -> Result<IndexedMesh, String> {
    let bytes = fs::read(path).map_err(|e| format!("can't read {}: {}", path, e))?;
    parse(&bytes).map_err(|e| format!("{}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "ply\nformat {} 1.0\ncomment a unit square\nelement vertex 4\n\
        property float x\nproperty float y\nproperty float z\nproperty uchar red\n\
        element face 1\nproperty list uchar int vertex_indices\nend_header\n";

    fn header(format: &str) -> Vec<u8> {
        HEADER.replace("{}", format).into_bytes()
    }

    fn check_square(m: &IndexedMesh) {
        assert_eq!(m.positions, vec![
            Vector3::new(0., 0., 0.), Vector3::new(1., 0., 0.),
            Vector3::new(1., 1., 0.), Vector3::new(0., 1., 0.),
        ]);
        // One quad, split into a fan of two triangles.
        assert_eq!(m.triangles, vec![[0, 1, 2], [0, 2, 3]]);
        assert!(m.normals.is_empty());
    }

    #[test]
    fn test_ascii() {
        let mut ply = header("ascii");
        ply.extend_from_slice(b"0 0 0 255\n1 0 0 255\n1 1 0 255\n0 1 0 255\n4 0 1 2 3\n");
        check_square(&parse(&ply).unwrap());
    }

    fn binary(big: bool) -> Vec<u8> {
        let mut ply = header(if big { "binary_big_endian" } else { "binary_little_endian" });
        for (x, y) in [(0f32, 0f32), (1., 0.), (1., 1.), (0., 1.)] {
            for v in [x, y, 0.] {
                ply.extend_from_slice(&if big { v.to_be_bytes() } else { v.to_le_bytes() });
            }
            ply.push(255);
        }
        ply.push(4);
        for i in 0i32 .. 4 {
            ply.extend_from_slice(&if big { i.to_be_bytes() } else { i.to_le_bytes() });
        }
        ply
    }

    #[test]
    fn test_binary_little_endian() {
        check_square(&parse(&binary(false)).unwrap());
    }

    #[test]
    fn test_binary_big_endian() {
        check_square(&parse(&binary(true)).unwrap());
    }

    #[test]
    fn test_normals_and_sized_type_names() {
        let ply = "ply\nformat ascii 1.0\nelement vertex 3\nproperty float32 x\nproperty float32 y\n\
            property float32 z\nproperty float32 nx\nproperty float32 ny\nproperty float32 nz\n\
            element face 1\nproperty list uint8 int32 vertex_index\nend_header\n\
            0 0 0 0 0 1\n1 0 0 0 0 1\n0 1 0 0 0 1\n3 0 1 2\n";
        let m = parse(ply.as_bytes()).unwrap();
        assert_eq!(m.normals, vec![Vector3::new(0., 0., 1.); 3]);
        assert_eq!(m.triangles, vec![[0, 1, 2]]);
    }

    #[test]
    fn test_unused_elements_are_read_past() {
        // An element before the vertices (as range-grid files have) must not shift the data.
        let ply = "ply\nformat ascii 1.0\nelement camera 1\nproperty float view_px\nproperty list uchar float extra\n\
            element vertex 3\nproperty float x\nproperty float y\nproperty float z\n\
            element face 1\nproperty list uchar int vertex_indices\nend_header\n\
            9 2 7 7\n0 0 0\n1 0 0\n0 1 0\n3 0 1 2\n";
        let m = parse(ply.as_bytes()).unwrap();
        assert_eq!(m.positions[1], Vector3::new(1., 0., 0.));
        assert_eq!(m.triangles, vec![[0, 1, 2]]);
    }

    #[test]
    fn test_errors() {
        assert!(parse(b"obj\n").is_err());
        let mut short = header("ascii");
        short.extend_from_slice(b"0 0 0 255\n");
        assert!(parse(&short).unwrap_err().contains("ends early"));
        let mut past = header("ascii");
        past.extend_from_slice(b"0 0 0 1\n1 0 0 1\n1 1 0 1\n0 1 0 1\n3 0 1 9\n");
        assert!(parse(&past).unwrap_err().contains("refers past"));
    }
}
