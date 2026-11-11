//! Reading `.wo3` meshes, Tungsten's binary mesh format (Benedikt Bitterli's Tungsten renderer,
//! <https://github.com/tunabrain/tungsten>, `src/core/io/MeshIO.cpp`, zlib licence). Bitterli's
//! Tungsten scenes keep their meshes in it.
//!
//! The file is the in-memory arrays written out, little-endian:
//!
//! ```text
//! u64 vertex count
//! vertex count x { f32 position[3], f32 normal[3], f32 uv[2] }     32 bytes each
//! u64 triangle count
//! triangle count x { u32 index[3], i32 material }                   16 bytes each
//! ```
//!
//! UVs and per-triangle materials are dropped, as we have no use for them yet. Normals are
//! kept; a mesh whose normals are all zero has none.

use crate::na::Vector3;
use crate::shapes::mesh::IndexedMesh;
use std::fs;

const VERTEX_BYTES: usize = 32;
const TRIANGLE_BYTES: usize = 16;

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8], String> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.bytes.len())
            .ok_or_else(|| format!("file ends inside the {}", what))?;
        let s = &self.bytes[self.at .. end];
        self.at = end;
        Ok(s)
    }

    fn count(&mut self, what: &str, size: usize) -> Result<usize, String> {
        let n = u64::from_le_bytes(self.take(8, what)?.try_into().unwrap());
        // Check against what's left before allocating anything for it.
        match usize::try_from(n).ok().and_then(|n| n.checked_mul(size)) {
            Some(b) if b <= self.bytes.len() - self.at => Ok(n as usize),
            _ => Err(format!("{} count {} is more than the file holds", what, n)),
        }
    }
}

fn f32_at(b: &[u8], i: usize) -> f64 {
    f32::from_le_bytes(b[i * 4 .. i * 4 + 4].try_into().unwrap()) as f64
}

pub fn parse(bytes: &[u8]) -> Result<IndexedMesh, String> {
    let mut r = Reader { bytes, at: 0 };
    let vertices = r.count("vertex", VERTEX_BYTES)?;
    let mut mesh = IndexedMesh { positions: Vec::with_capacity(vertices), normals: Vec::with_capacity(vertices), triangles: Vec::new() };
    for _ in 0 .. vertices {
        let v = r.take(VERTEX_BYTES, "vertices")?;
        mesh.positions.push(Vector3::new(f32_at(v, 0), f32_at(v, 1), f32_at(v, 2)));
        mesh.normals.push(Vector3::new(f32_at(v, 3), f32_at(v, 4), f32_at(v, 5)));
    }
    if mesh.normals.iter().all(|n| *n == Vector3::zeros()) {
        mesh.normals.clear();
    }
    let triangles = r.count("triangle", TRIANGLE_BYTES)?;
    mesh.triangles.reserve(triangles);
    for _ in 0 .. triangles {
        let t = r.take(TRIANGLE_BYTES, "triangles")?;
        let i = |k: usize| u32::from_le_bytes(t[k * 4 .. k * 4 + 4].try_into().unwrap()) as usize;
        let tri = [i(0), i(1), i(2)];
        if tri.iter().any(|&v| v >= vertices) {
            return Err(format!("triangle {:?} refers past the {} vertices", tri, vertices));
        }
        mesh.triangles.push(tri);
    }
    if r.at != bytes.len() {
        return Err(format!("{} bytes left over after the triangles", bytes.len() - r.at));
    }
    Ok(mesh)
}

/// Read a `.wo3` file.
pub fn read(path: &str) -> Result<IndexedMesh, String> {
    let bytes = crate::assets::read(path)?;
    parse(&bytes).map_err(|e| format!("{}: {}", path, e))
}

/// The bytes of a `.wo3` file, for tests.
#[cfg(test)]
pub fn write(positions: &[[f32; 3]], normals: Option<&[[f32; 3]]>, triangles: &[[u32; 3]]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend((positions.len() as u64).to_le_bytes());
    for (i, p) in positions.iter().enumerate() {
        let n = normals.map_or([0.; 3], |n| n[i]);
        for x in p.iter().chain(n.iter()).chain([0f32, 0.].iter()) {
            b.extend(x.to_le_bytes());
        }
    }
    b.extend((triangles.len() as u64).to_le_bytes());
    for t in triangles {
        for i in t {
            b.extend(i.to_le_bytes());
        }
        b.extend(0i32.to_le_bytes());
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    const SQUARE: [[f32; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]];
    const TRIS: [[u32; 3]; 2] = [[0, 1, 2], [0, 2, 3]];

    #[test]
    fn test_reads_a_square() {
        let m = parse(&write(&SQUARE, None, &TRIS)).unwrap();
        assert_eq!(m.positions[2], Vector3::new(1., 1., 0.));
        assert_eq!(m.triangles, vec![[0, 1, 2], [0, 2, 3]]);
        assert!(m.normals.is_empty(), "all-zero normals mean none");

        let n = [[0f32, 0., 1.]; 4];
        let m = parse(&write(&SQUARE, Some(&n), &TRIS)).unwrap();
        assert_eq!(m.normals, vec![Vector3::new(0., 0., 1.); 4]);
    }

    #[test]
    fn test_errors() {
        let good = write(&SQUARE, None, &TRIS);
        assert!(parse(&good[.. good.len() - 1]).unwrap_err().contains("more than the file holds"));
        assert!(parse(&good[.. 4]).unwrap_err().contains("ends inside"));
        assert!(parse(&[good.clone(), vec![0]].concat()).unwrap_err().contains("left over"));
        assert!(parse(&write(&SQUARE, None, &[[0, 1, 4]])).unwrap_err().contains("refers past"));
        let mut huge = good.clone();
        huge[.. 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(parse(&huge).unwrap_err().contains("more than the file holds"));
    }
}
