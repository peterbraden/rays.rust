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
//! - `tree-<key>-<part>-<level>.bin`: one part at one level (`lod::Detail::encode`);
//! - `placements-<key>.bin`: where a forest's trees stand (`forest::Placement`), keyed by the
//!   forest and the scene's geometries (one is its ground); a heightmap read from an image is
//!   named, not hashed, so after changing one, empty the cache.
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
use crate::procedural::forest::Placement;
use crate::na::Vector3;
use crate::shapes::bbox::BBox;
use crate::shapes::lod::{Bytes, Reader};
use std::path::PathBuf;

/// Marks a tree's header, and its format.
const HEADER: &[u8; 8] = b"RAYSTRE1";

/// Marks a forest's placements, and their format: change the number when placement changes.
const PLACEMENTS: &[u8; 8] = b"RAYSPLA1";

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
        w.f64(self.base);
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
        let base = r.f64()?;
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

    /// Where a forest's trees stand, kept under `key`, if kept.
    pub fn placements(&self, key: &str) -> Option<Vec<Placement>> {
        let bytes = self.read(&format!("placements-{key}.bin"))?;
        let mut r = Reader { bytes: &bytes, at: 0 };
        if r.take(PLACEMENTS.len())? != PLACEMENTS {
            return None;
        }
        let placed = (0 .. r.u32()?).map(|_| Some(Placement {
            species: r.u32()? as usize,
            variant: r.u32()? as usize,
            position: r.vector()?,
            yaw: r.f64()?,
            scale: r.f64()?,
            mirror: r.u8()? != 0,
            slope: r.f64()?,
        })).collect::<Option<Vec<_>>>()?;
        (r.at == bytes.len()).then_some(placed)
    }

    pub fn put_placements(&self, key: &str, placed: &[Placement]) {
        let mut w = Bytes::new(PLACEMENTS);
        w.u32(placed.len() as u32);
        for p in placed {
            w.u32(p.species as u32);
            w.u32(p.variant as u32);
            w.vector(&p.position);
            w.f64(p.yaw);
            w.f64(p.scale);
            w.u8(p.mirror as u8);
            w.f64(p.slope);
        }
        self.write(&format!("placements-{key}.bin"), &w.0);
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
    fn test_placements_read_back() {
        let dir = std::env::temp_dir().join(format!("rays-tree-cache-test-{}", std::process::id()));
        let cache = TreeCache::new(&dir);
        let placed = vec![
            Placement { species: 2, variant: 1, position: Vector3::new(1.5, -2.25, 3e3), yaw: 123.4, scale: 0.9, mirror: true, slope: 12.5 },
            Placement { species: 0, variant: 0, position: Vector3::new(-0.1, 0.2, 0.3), yaw: 0., scale: 1.1, mirror: false, slope: 0. },
        ];
        cache.put_placements("k", &placed);
        assert_eq!(cache.placements("k"), Some(placed));
        assert_eq!(cache.placements("missing"), None);
        let _ = std::fs::remove_dir_all(dir);
    }

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
