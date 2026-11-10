//! Files a scene refers to by path: meshes, heightmap images, and scene files themselves.
//!
//! On the command line these are read from disk. In a browser there is no disk, so the page
//! fetches each file and registers its contents here under the path the scene uses; a
//! registered file takes precedence over one on disk.
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static REGISTERED: RwLock<Option<HashMap<String, Arc<Vec<u8>>>>> = RwLock::new(None);

/// Make `bytes` the contents of `path` for every later `read`.
pub fn register(path: &str, bytes: Vec<u8>) {
    REGISTERED.write().unwrap().get_or_insert_with(HashMap::new).insert(path.to_string(), Arc::new(bytes));
}

/// Whether `path` has been registered.
pub fn is_registered(path: &str) -> bool {
    REGISTERED.read().unwrap().as_ref().is_some_and(|m| m.contains_key(path))
}

/// The contents of `path`: as registered, or else from disk.
pub fn read(path: &str) -> Result<Vec<u8>, String> {
    let registered = REGISTERED.read().unwrap().as_ref().and_then(|m| m.get(path).cloned());
    match registered {
        Some(bytes) => Ok(bytes.to_vec()),
        None => std::fs::read(path).map_err(|e| format!("can't read {}: {}", path, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registered_file_is_read_without_a_disk() {
        assert!(read("no-such-dir/tri.obj").is_err());
        register("no-such-dir/tri.obj", b"v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n".to_vec());
        assert!(is_registered("no-such-dir/tri.obj"));
        assert_eq!(read("no-such-dir/tri.obj").unwrap().len(), 32);
        let mesh = crate::shapes::mesh::Mesh::from_file("no-such-dir/tri.obj".to_string(), na::Vector3::new(1., 1., 1.));
        assert_eq!(crate::shapes::geometry::Geometry::primitives(&mesh), 1);
    }
}
