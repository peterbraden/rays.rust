//! Files a scene refers to (meshes, textures, volumes), read through here so a host without a
//! filesystem (a browser) can provide them: what `provide` has been given is read from memory,
//! anything else from disk.
use std::collections::HashMap;
use std::sync::Mutex;

static PROVIDED: Mutex<Option<HashMap<String, Vec<u8>>>> = Mutex::new(None);

/// Supply a file's contents under the path a scene uses for it (eg. "./demo/teapot.obj").
pub fn provide(path: &str, bytes: Vec<u8>) {
    PROVIDED.lock().unwrap().get_or_insert_with(HashMap::new).insert(path.to_string(), bytes);
}

/// A file's contents: as provided, or from disk.
pub fn read(path: &str) -> std::io::Result<Vec<u8>> {
    if let Some(bytes) = PROVIDED.lock().unwrap().as_ref().and_then(|m| m.get(path)) {
        return Ok(bytes.clone());
    }
    std::fs::read(path)
}
