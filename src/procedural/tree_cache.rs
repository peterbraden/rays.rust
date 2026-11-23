//! A forest's trees kept on disk at the levels of detail they are seen at
//! (docs/research/learned-lod/caching.md).
//!
//! Growing a forest's distinct trees and simplifying each to the levels its placements need
//! (`shapes::lod`) took most of a forest's build: 70-90 s of `forest-aerial`'s, and the peak of
//! its memory, every run. Once made, a tree's levels depend only on what made them, so they
//! are kept in a directory (a forest's `"cache"`) and read back instead:
//!
//! - `tree-<key>.bin`: what choosing a placement's level and making the tree's materials need
//!   without its mesh: the size of its pieces (`lod::piece_size`), its bounds, each leaf's
//!   state (its autumn colour), and its parts (bark, dead bark, the leaves of each material);
//! - `tree-<key>-<part>-<level>.bin`: one part at one level (`lod::Detail::encode`).
//!
//! A tree's key is the FNV hash of everything that makes it (`world::key`): its parameters,
//! its seed, its forest entry (which names its materials) and those materials as the scene
//! defines them (the leaves' albedo shapes their cards' shadowing), with the level format
//! (`lod::MAGIC`'s number, in `decode`) checked on reading. A tree whose header is there is never
//! grown; one whose header or a level is missing is grown, and what was missing is written. A
//! level is used as decoded from its bytes whether it was read or just made, so a render is the
//! same with the cache, without it, and with it empty.
//!
//! Files are written whole and renamed into place, as `world` writes its stages, so runs may
//! share a directory.
use crate::material::leaf::LeafState;
use crate::na::Vector3;
use crate::shapes::bbox::BBox;
use crate::shapes::lod::{Bytes, Reader};
use std::path::PathBuf;

/// Marks a tree's header, and its format.
const HEADER: &[u8; 8] = b"RAYSTRE1";

/// What a tree's part is, to make its material without its mesh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    Bark,
    DeadBark,
    /// The leaves of the forest entry's `"leaves"` material of this number.
    Leaves(u32),
}

/// What a tree needs, without its mesh, to choose its placements' levels and make its
/// materials.
#[derive(Clone, Debug)]
pub struct Header {
    pub base: f64,
    pub bounds: BBox,
    pub states: Vec<LeafState>,
    pub parts: Vec<Part>,
}

impl Header {
    fn encode(&self) -> Vec<u8> {
        let mut w = Bytes::new(HEADER);
        w.vector(&Vector3::new(self.base, 0., 0.));
        w.vector(&self.bounds.min.coords);
        w.vector(&self.bounds.max.coords);
        w.u32(self.states.len() as u32);
        for s in &self.states {
            w.f32(s.age);
            w.f32(s.exposure);
        }
        w.u32(self.parts.len() as u32);
        for p in &self.parts {
            match p {
                Part::Bark => { w.u8(0); w.u32(0) },
                Part::DeadBark => { w.u8(1); w.u32(0) },
                Part::Leaves(k) => { w.u8(2); w.u32(*k) },
            }
        }
        w.0
    }

    fn decode(bytes: &[u8]) -> Option<Header> {
        let mut r = Reader { bytes, at: 0 };
        if r.take(HEADER.len())? != HEADER {
            return None;
        }
        let base = r.vector()?.x;
        let bounds = BBox::new(r.vector()?, r.vector()?);
        let states = (0 .. r.u32()?).map(|_| Some(LeafState { age: r.f32()?, exposure: r.f32()? })).collect::<Option<Vec<_>>>()?;
        let parts = (0 .. r.u32()?).map(|_| Some(match (r.u8()?, r.u32()?) {
            (0, _) => Part::Bark,
            (1, _) => Part::DeadBark,
            (2, k) => Part::Leaves(k),
            _ => return None,
        })).collect::<Option<Vec<_>>>()?;
        (r.at == bytes.len()).then_some(Header { base, bounds, states, parts })
    }
}

/// A directory of trees' levels.
pub struct TreeCache {
    dir: PathBuf,
}

impl TreeCache {
    pub fn new(dir: impl Into<PathBuf>) -> TreeCache {
        TreeCache { dir: dir.into() }
    }

    /// The header of the tree `key`, if kept.
    pub fn header(&self, key: &str) -> Option<Header> {
        Header::decode(&self.read(&format!("tree-{key}.bin"))?)
    }

    pub fn put_header(&self, key: &str, header: &Header) {
        self.write(&format!("tree-{key}.bin"), &header.encode());
    }

    /// Part `part` of the tree `key` at `level`, as `lod::Detail::encode` made it, if kept.
    pub fn level(&self, key: &str, part: usize, level: usize) -> Option<Vec<u8>> {
        self.read(&format!("tree-{key}-{part}-{level}.bin"))
    }

    pub fn put_level(&self, key: &str, part: usize, level: usize, bytes: &[u8]) {
        self.write(&format!("tree-{key}-{part}-{level}.bin"), bytes);
    }

    fn read(&self, file: &str) -> Option<Vec<u8>> {
        crate::assets::read(&self.dir.join(file).to_string_lossy()).ok()
    }

    /// Writes `bytes` whole, then renames them into place, so a reader never sees part of a
    /// file; a failure is reported and otherwise ignored (the tree is made again next time).
    fn write(&self, file: &str, bytes: &[u8]) {
        let path = self.dir.join(file);
        #[cfg(not(target_arch = "wasm32"))]
        let process = std::process::id();
        #[cfg(target_arch = "wasm32")]
        let process = 0;
        let partial = path.with_extension(format!("tmp{process}"));
        let written = std::fs::create_dir_all(&self.dir)
            .and_then(|()| std::fs::write(&partial, bytes))
            .and_then(|()| std::fs::rename(&partial, &path));
        if let Err(e) = written {
            eprintln!("Forest: couldn't keep {}: {}", path.display(), e);
            let _ = std::fs::remove_file(&partial);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_header_reads_back() {
        let header = Header {
            base: 0.05,
            bounds: BBox::new(Vector3::new(-1., 0., -2.), Vector3::new(1., 9., 2.)),
            states: vec![LeafState { age: 0.5, exposure: 0.25 }, LeafState::default()],
            parts: vec![Part::Bark, Part::DeadBark, Part::Leaves(1)],
        };
        let back = Header::decode(&header.encode()).unwrap();
        assert_eq!((back.base, back.bounds.min, back.bounds.max), (header.base, header.bounds.min, header.bounds.max));
        assert_eq!((&back.states, &back.parts), (&header.states, &header.parts));
        let mut bytes = header.encode();
        bytes.pop();
        assert!(Header::decode(&bytes).is_none());
    }
}
