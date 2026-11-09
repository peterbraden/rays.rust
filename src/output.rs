/// Output stage: turns the renderer's linear, high-dynamic-range pixels into an image file.
///
/// The renderer works in linear light throughout. Display formats (PNG, and later JPEG) need
/// those values mapped into [0, 1] and encoded for display: exposure, an optional tone map,
/// then a transfer function (sRGB by default). HDR formats would write the linear values as
/// they are. Writers are chosen by file extension.
use crate::color::Color;
use crate::scenefile::SceneFile;
use serde_json::Value;
use std::path::Path;

/// A linear image, top row first.
pub struct LinearImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl LinearImage {
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToneMap {
    /// Clip values above 1.
    None,
    /// v / (1 + v): compresses highlights smoothly.
    Reinhard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transfer {
    Srgb,
    Linear,
}

/// How linear values are mapped for display, from the scene file's "output" block, eg.
/// `"output": { "exposure": 1.0, "tonemap": "none", "transfer": "srgb" }`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutputOpts {
    pub exposure: f64,
    pub tonemap: ToneMap,
    pub transfer: Transfer,
}

impl Default for OutputOpts {
    fn default() -> OutputOpts {
        OutputOpts { exposure: 1., tonemap: ToneMap::None, transfer: Transfer::Srgb }
    }
}

fn srgb_oetf(v: f64) -> f64 {
    if v <= 0.003_130_8 {
        12.92 * v
    } else if v >= 1. {
        // Exact, rather than 1.055 - 0.055.
        1.
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}

impl OutputOpts {
    pub fn parse(o: &Value) -> OutputOpts {
        let d = OutputOpts::default();
        OutputOpts {
            exposure: SceneFile::parse_number(&o["exposure"], d.exposure),
            tonemap: match o["tonemap"].as_str() {
                None | Some("none") => ToneMap::None,
                Some("reinhard") => ToneMap::Reinhard,
                Some(x) => panic!("Unknown output tonemap '{}': expected \"none\" or \"reinhard\"", x),
            },
            transfer: match o["transfer"].as_str() {
                None | Some("srgb") => Transfer::Srgb,
                Some("linear") => Transfer::Linear,
                Some(x) => panic!("Unknown output transfer '{}': expected \"srgb\" or \"linear\"", x),
            },
        }
    }

    /// Map one linear channel to a display value in [0, 1].
    pub fn encode(&self, v: f64) -> f64 {
        // NaN and negative values are black.
        let v = (v * self.exposure).max(0.);
        let v = match self.tonemap {
            ToneMap::None => v,
            ToneMap::Reinhard => v / (1. + v),
        }.min(1.);
        match self.transfer {
            Transfer::Srgb => srgb_oetf(v),
            Transfer::Linear => v,
        }.clamp(0., 1.)
    }

    pub fn quantise(&self, c: Color) -> [u8; 3] {
        let q = |v: f64| (self.encode(v) * 255.).round() as u8;
        [q(c.rgb.x), q(c.rgb.y), q(c.rgb.z)]
    }
}

/// Writes a linear image in some file format.
pub trait ImageWriter: Sync {
    fn write(&self, img: &LinearImage, path: &str) -> Result<(), String>;
}

/// 8-bit PNG, encoded for display.
pub struct Png {
    pub opts: OutputOpts,
}

impl ImageWriter for Png {
    fn write(&self, img: &LinearImage, path: &str) -> Result<(), String> {
        let out = image::ImageBuffer::from_fn(img.width as u32, img.height as u32, |x, y| {
            image::Rgb(self.opts.quantise(img.get(x as usize, y as usize)))
        });
        // Write via a temporary file, so a partial image is never left at `path`.
        let tmp = format!("{}.tmp", path);
        out.save_with_format(Path::new(&tmp), image::ImageFormat::Png)
            .and_then(|_| std::fs::rename(&tmp, path).map_err(image::ImageError::IoError))
            .map_err(|e| format!("Failed to write {}: {}", path, e))
    }
}

/// The writer for `path`'s file format.
pub fn writer_for(path: &str, opts: OutputOpts) -> Result<Box<dyn ImageWriter>, String> {
    let ext = Path::new(path).extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("png") => Ok(Box::new(Png { opts })),
        _ => Err(format!("Unsupported output format for {}", path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_encoding() {
        let o = OutputOpts::default();
        assert_eq!(o.quantise(Color::new(0., 1., 0.5)), [0, 255, 188]);
        // Linear 18% grey is mid-grey on screen, not 46/255.
        assert_eq!(o.quantise(Color::new(0.18, 0.18, 0.18)), [118, 118, 118]);
    }

    #[test]
    fn test_linear_transfer_rounds() {
        let o = OutputOpts { transfer: Transfer::Linear, ..OutputOpts::default() };
        assert_eq!(o.quantise(Color::new(0.18, 0.999, 0.5)), [46, 255, 128]);
    }

    #[test]
    fn test_exposure_and_tonemap() {
        let o = OutputOpts { exposure: 2., tonemap: ToneMap::Reinhard, transfer: Transfer::Linear };
        // 0.5 * 2 = 1 -> 1 / (1 + 1)
        assert!((o.encode(0.5) - 0.5).abs() < 1e-12);
        // Highlights compress rather than clip.
        assert!(o.encode(100.) < 1.);
    }

    #[test]
    fn test_invalid_values_are_black() {
        let o = OutputOpts::default();
        assert_eq!(o.encode(f64::NAN), 0.);
        assert_eq!(o.encode(-1.), 0.);
        assert_eq!(o.encode(f64::INFINITY), 1.);
    }

    #[test]
    fn test_parse() {
        assert_eq!(OutputOpts::parse(&Value::Null), OutputOpts::default());
        let o = OutputOpts::parse(&serde_json::json!({ "exposure": 0.5, "tonemap": "reinhard", "transfer": "linear" }));
        assert_eq!(o, OutputOpts { exposure: 0.5, tonemap: ToneMap::Reinhard, transfer: Transfer::Linear });
    }

    #[test]
    fn test_writer_for_extension() {
        assert!(writer_for("a/b.png", OutputOpts::default()).is_ok());
        assert!(writer_for("a/b.PNG", OutputOpts::default()).is_ok());
        assert!(writer_for("a/b.exr", OutputOpts::default()).is_err());
    }
}
