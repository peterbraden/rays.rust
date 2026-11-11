/// Checkpoints: a render's accumulated samples saved to disk, so that an interrupted render can
/// be resumed, and a finished one extended with more samples.
///
/// A render is a sum of layers (passes over the image), and each layer of each pixel draws its
/// random numbers from its own stream (see `rendercontext::render_pixel`). Any layer can
/// therefore be rendered at any time with the same result, and a pixel's value depends only on
/// which layers have been added to it, in order. A checkpoint holds exactly that: each pixel's
/// running sum and sample count. Resuming restores both and renders only the layers each chunk
/// lacks, adding them in the same order an uninterrupted render would, so the result is
/// identical to the last bit.
///
/// Counts are per pixel, so a checkpoint taken mid-layer is still exact: chunks are applied
/// region by region in layer order, and a region's count says how many of its layers are done.
///
/// The file is a short text header followed by raw little-endian numbers. Sums are stored as
/// bits, not decimal text, since a decimal round trip need not return the same `f64`.
///
/// ```text
/// rays-checkpoint 1
/// {"width":..,"height":..,"seed":..,"samples_per_layer":..,"scene":".."}
/// width * height * (r: f64, g: f64, b: f64, samples: u64), bottom row first
/// ```
///
/// `scene` is the scene's fingerprint (`SceneFile::fingerprint`): the scene file with its
/// variables substituted and the settings that don't change any sample removed. A checkpoint is
/// refused for a scene that differs in anything else. Files the scene refers to, such as meshes
/// and textures, are not part of it.
use crate::color::Color;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

const MAGIC: &str = "rays-checkpoint 1";

/// What a checkpoint must agree with to be resumed.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CheckpointHeader {
    pub width: usize,
    pub height: usize,
    pub seed: u64,
    pub samples_per_layer: usize,
    /// The scene's fingerprint.
    pub scene: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checkpoint {
    pub header: CheckpointHeader,
    /// Each pixel's sum of samples, in the render context's order (bottom row first).
    pub sums: Vec<Color>,
    /// Samples summed for each pixel.
    pub samples: Vec<usize>,
}

impl Checkpoint {
    pub fn write_to(&self, w: &mut impl Write) -> std::io::Result<()> {
        writeln!(w, "{}", MAGIC)?;
        writeln!(w, "{}", serde_json::to_string(&self.header)?)?;
        for (c, n) in self.sums.iter().zip(&self.samples) {
            for v in [c.rgb.x, c.rgb.y, c.rgb.z] {
                w.write_all(&v.to_le_bytes())?;
            }
            w.write_all(&(*n as u64).to_le_bytes())?;
        }
        Ok(())
    }

    pub fn read_from(r: &mut impl BufRead) -> Result<Checkpoint, String> {
        let mut line = String::new();
        r.read_line(&mut line).map_err(|e| e.to_string())?;
        if line.trim_end() != MAGIC {
            return Err("not a rays checkpoint".to_string());
        }
        line.clear();
        r.read_line(&mut line).map_err(|e| e.to_string())?;
        let header: CheckpointHeader = serde_json::from_str(&line).map_err(|e| format!("bad header: {}", e))?;
        let n = header.width.checked_mul(header.height).ok_or("bad header: image too large")?;
        let mut sums = Vec::with_capacity(n);
        let mut samples = Vec::with_capacity(n);
        let mut buf = [0u8; 32];
        let word = |b: &[u8]| -> [u8; 8] { b.try_into().unwrap() };
        for _ in 0 .. n {
            r.read_exact(&mut buf).map_err(|_| "truncated".to_string())?;
            sums.push(Color::new(
                f64::from_le_bytes(word(&buf[0 .. 8])),
                f64::from_le_bytes(word(&buf[8 .. 16])),
                f64::from_le_bytes(word(&buf[16 .. 24]))));
            samples.push(u64::from_le_bytes(word(&buf[24 .. 32])) as usize);
        }
        if r.read(&mut buf).map_err(|e| e.to_string())? != 0 {
            return Err("unexpected data after the pixels".to_string());
        }
        Ok(Checkpoint { header, sums, samples })
    }

    /// Write to `path`, by way of a temporary file renamed into place, so that an interruption
    /// while writing leaves the previous checkpoint intact.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let tmp = format!("{}.tmp", path);
        let fail = |e: std::io::Error| format!("Failed to write checkpoint {}: {}", path, e);
        let mut w = BufWriter::new(File::create(&tmp).map_err(fail)?);
        self.write_to(&mut w).map_err(fail)?;
        w.into_inner().map_err(|e| fail(e.into_error()))?.sync_all().map_err(fail)?;
        std::fs::rename(&tmp, path).map_err(fail)
    }

    /// The checkpoint at `path`, or `None` if there is no file there.
    pub fn load(path: &str) -> Result<Option<Checkpoint>, String> {
        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("Failed to read checkpoint {}: {}", path, e)),
        };
        Checkpoint::read_from(&mut BufReader::new(f))
            .map(Some)
            .map_err(|e| format!("Failed to read checkpoint {}: {}", path, e))
    }

    /// Whether this checkpoint can be resumed by a render described by `expected`; if not, why.
    pub fn check(&self, expected: &CheckpointHeader) -> Result<(), String> {
        let h = &self.header;
        if (h.width, h.height) != (expected.width, expected.height) {
            return Err(format!("the checkpoint is of a {}x{} image; this render is {}x{}",
                h.width, h.height, expected.width, expected.height));
        }
        if h.seed != expected.seed {
            return Err(format!("the checkpoint was rendered with seed {}; this render uses {}", h.seed, expected.seed));
        }
        if h.scene != expected.scene {
            return Err(format!("the checkpoint was rendered from a different scene (differing in {})",
                differing_keys(&h.scene, &expected.scene)));
        }
        // Implied by the scene, but checked as the counts depend on it.
        if h.samples_per_layer != expected.samples_per_layer
            || self.samples.iter().any(|n| n % h.samples_per_layer.max(1) != 0) {
            return Err("the checkpoint's samples don't make whole layers".to_string());
        }
        Ok(())
    }

    /// Whole layers in the checkpoint, if every pixel has the same number.
    pub fn layers(&self) -> Option<usize> {
        let n = *self.samples.first()?;
        self.samples.iter().all(|m| *m == n).then_some(n / self.header.samples_per_layer.max(1))
    }
}

/// The top-level keys in which two fingerprints differ, for an error message.
fn differing_keys(a: &str, b: &str) -> String {
    let parse = |s: &str| serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(s).unwrap_or_default();
    let (a, b) = (parse(a), parse(b));
    let mut keys: Vec<&String> = a.keys().chain(b.keys()).filter(|k| a.get(*k) != b.get(*k)).collect();
    keys.sort();
    keys.dedup();
    if keys.is_empty() {
        "its contents".to_string()
    } else {
        keys.iter().map(|k| format!("\"{}\"", k)).collect::<Vec<_>>().join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header() -> CheckpointHeader {
        CheckpointHeader { width: 2, height: 1, seed: 7, samples_per_layer: 4, scene: r#"{"a":1,"b":[2]}"#.to_string() }
    }

    fn checkpoint() -> Checkpoint {
        Checkpoint {
            header: header(),
            sums: vec![Color::new(0.1, 1e300, -0.), Color::new(f64::MIN_POSITIVE, 3., 1. / 3.)],
            samples: vec![8, 4],
        }
    }

    #[test]
    fn test_round_trip_is_exact() {
        let c = checkpoint();
        let mut bytes = Vec::new();
        c.write_to(&mut bytes).unwrap();
        let back = Checkpoint::read_from(&mut &bytes[..]).unwrap();
        assert_eq!(back, c);
        // -0. == 0., so compare bits too.
        assert_eq!(back.sums[0].rgb.z.to_bits(), (-0f64).to_bits());

        assert!(Checkpoint::read_from(&mut &bytes[.. bytes.len() - 1]).unwrap_err().contains("truncated"));
        bytes.push(0);
        assert!(Checkpoint::read_from(&mut &bytes[..]).is_err());
        assert!(Checkpoint::read_from(&mut &b"P6\n"[..]).unwrap_err().contains("not a rays checkpoint"));
    }

    #[test]
    fn test_save_and_load() {
        let dir = std::env::temp_dir().join(format!("rays-checkpoint-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("c.checkpoint").to_string_lossy().into_owned();
        assert_eq!(Checkpoint::load(&path).unwrap(), None);
        checkpoint().save(&path).unwrap();
        assert_eq!(Checkpoint::load(&path).unwrap(), Some(checkpoint()));
        assert!(!std::path::Path::new(&format!("{}.tmp", path)).exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_mismatches_are_refused() {
        let c = checkpoint();
        assert_eq!(c.check(&header()), Ok(()));

        let size = CheckpointHeader { width: 1, height: 2, ..header() };
        assert_eq!(c.check(&size).unwrap_err(), "the checkpoint is of a 2x1 image; this render is 1x2");
        let seed = CheckpointHeader { seed: 8, ..header() };
        assert!(c.check(&seed).unwrap_err().contains("seed 7; this render uses 8"));
        let scene = CheckpointHeader { scene: r#"{"a":1,"b":[3],"c":0}"#.to_string(), ..header() };
        assert_eq!(c.check(&scene).unwrap_err(), r#"the checkpoint was rendered from a different scene (differing in "b", "c")"#);
        let layer = CheckpointHeader { samples_per_layer: 8, ..header() };
        assert!(c.check(&layer).is_err());
    }

    #[test]
    fn test_layers() {
        let mut c = checkpoint();
        assert_eq!(c.layers(), None);
        c.samples = vec![8, 8];
        assert_eq!(c.layers(), Some(2));
    }
}
