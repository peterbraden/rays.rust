/// Output stage: turns the renderer's linear, high-dynamic-range pixels into an image file.
///
/// The renderer works in linear light throughout. Display formats (PNG, and later JPEG) need
/// those values mapped into [0, 1] and encoded for display: exposure, an optional tone map,
/// then a transfer function (sRGB by default). HDR formats would write the linear values as
/// they are. Writers are chosen by file extension.
use crate::color::Color;
use crate::glare::Glare;
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
    /// v^(1/gamma). Gamma 2, a square root, is the approximation used by "Ray Tracing in One
    /// Weekend" (v4.0.2, section "Using Gamma Correction for Accurate Color Intensity").
    Gamma(f64),
}

/// How linear values are mapped for display, from the scene file's "output" block, eg.
/// `"output": { "exposure": 1.0, "tonemap": "none", "transfer": "srgb", "glare": true }`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutputOpts {
    pub exposure: f64,
    pub tonemap: ToneMap,
    pub transfer: Transfer,
    /// The camera's glare, applied to the linear image before it is written (see `glare`):
    /// `true`, or `{"strength": 0.05, "core": 0.3, "radius": 25, "threshold": 1}`, angles in
    /// degrees.
    pub glare: Option<Glare>,
    /// Angle (radians) across a pixel at the image's centre, from the camera; for glare.
    pub pixel_angle: f64,
    /// The sensor's exposure, white balance and output colour space (`film`).
    pub film: crate::film::Film,
}

impl Default for OutputOpts {
    fn default() -> OutputOpts {
        OutputOpts { exposure: 1., tonemap: ToneMap::None, transfer: Transfer::Srgb, glare: None, pixel_angle: 0., film: crate::film::Film::default() }
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
                Some("gamma") => Transfer::Gamma(SceneFile::parse_number(&o["gamma"], 2.2)),
                Some(x) => panic!("Unknown output transfer '{}': expected \"srgb\", \"linear\" or \"gamma\"", x),
            },
            glare: match o.get("glare") {
                None | Some(Value::Bool(false)) => None,
                Some(g) => {
                    let d = Glare::default();
                    let degrees = |k: &str, d: f64| SceneFile::parse_number(&g[k], d.to_degrees()).to_radians();
                    Some(Glare {
                        strength: SceneFile::parse_number(&g["strength"], d.strength).clamp(0., 1.),
                        core: degrees("core", d.core).max(1e-6),
                        radius: degrees("radius", d.radius),
                        threshold: SceneFile::parse_number(&g["threshold"], d.threshold),
                    })
                },
            },
            pixel_angle: 0.,
            film: crate::film::Film::parse(o),
        }
    }

    /// The finished linear image: with the camera's glare, if any, through the film.
    pub fn finish(&self, img: &LinearImage) -> LinearImage {
        let img = match self.glare {
            Some(g) if self.pixel_angle > 0. => g.apply(img, self.pixel_angle, self.exposure),
            _ => LinearImage { width: img.width, height: img.height, pixels: img.pixels.clone() },
        };
        if self.film.is_identity() {
            return img;
        }
        LinearImage { width: img.width, height: img.height, pixels: img.pixels.iter().map(|c| self.film.apply(c)).collect() }
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
            Transfer::Gamma(g) => v.powf(1. / g),
        }.clamp(0., 1.)
    }

    /// The inverse of the transfer function, for reading display-encoded (eg. 8-bit) images
    /// back into linear light. Exposure and tone mapping aren't undone.
    pub fn decode(&self, v: f64) -> f64 {
        match self.transfer {
            Transfer::Srgb => if v <= 0.040_45 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) },
            Transfer::Linear => v,
            Transfer::Gamma(g) => v.powf(g),
        }
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

/// OpenEXR: the linear values as 32-bit floats, with no exposure, tone mapping or transfer
/// function, which are display concerns. EXR is the usual format for high dynamic range
/// renders and for reference images to compare against; see "Technical Introduction to
/// OpenEXR" (Kainz, Bogart, Stanczyk, Hillman),
/// <https://openexr.com/en/latest/TechnicalIntroduction.html>.
pub struct Exr;

impl ImageWriter for Exr {
    fn write(&self, img: &LinearImage, path: &str) -> Result<(), String> {
        let out = image::Rgb32FImage::from_fn(img.width as u32, img.height as u32, |x, y| {
            let c = img.get(x as usize, y as usize).rgb;
            // NaN means a broken sample; keep it out of the file.
            let f = |v: f64| if v.is_nan() { 0. } else { v as f32 };
            image::Rgb([f(c.x), f(c.y), f(c.z)])
        });
        let tmp = format!("{}.tmp", path);
        out.save_with_format(Path::new(&tmp), image::ImageFormat::OpenExr)
            .and_then(|_| std::fs::rename(&tmp, path).map_err(image::ImageError::IoError))
            .map_err(|e| format!("Failed to write {}: {}", path, e))
    }
}

/// Read a linear image, eg. an EXR reference render.
pub fn read_linear(path: &str) -> Result<LinearImage, String> {
    let img = image::open(Path::new(path)).map_err(|e| format!("Failed to read {}: {}", path, e))?.to_rgb32f();
    let pixels = img.pixels().map(|p| Color::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect();
    Ok(LinearImage { width: img.width() as usize, height: img.height() as usize, pixels })
}

/// Read a reference image as linear light. EXR is already linear; 8-bit formats (PNG, JPEG)
/// hold display-encoded values, which are decoded with `opts`' transfer function, so the
/// reference should be encoded the way the scene's own output is.
pub fn read_reference(path: &str, opts: &OutputOpts) -> Result<LinearImage, String> {
    let mut img = read_linear(path)?;
    let linear = Path::new(path).extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("exr")).unwrap_or(false);
    if !linear {
        for p in img.pixels.iter_mut() {
            *p = Color::new(opts.decode(p.rgb.x), opts.decode(p.rgb.y), opts.decode(p.rgb.z));
        }
    }
    Ok(img)
}

/// The writer for `path`'s file format.
pub fn writer_for(path: &str, opts: OutputOpts) -> Result<Box<dyn ImageWriter>, String> {
    let ext = Path::new(path).extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("png") => Ok(Box::new(Png { opts })),
        Some("exr") => Ok(Box::new(Exr)),
        _ => Err(format!("Unsupported output format for {}", path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_transfer_and_decode() {
        let o = OutputOpts { transfer: Transfer::Gamma(2.), ..OutputOpts::default() };
        assert!((o.encode(0.25) - 0.5).abs() < 1e-12);
        for opts in [o, OutputOpts::default(), OutputOpts { transfer: Transfer::Linear, ..OutputOpts::default() }] {
            for v in [0., 0.001, 0.18, 0.5, 1.] {
                assert!((opts.decode(opts.encode(v)) - v).abs() < 1e-9, "{:?} {}", opts.transfer, v);
            }
        }
    }

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
        let o = OutputOpts { exposure: 2., tonemap: ToneMap::Reinhard, transfer: Transfer::Linear, ..OutputOpts::default() };
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
        assert_eq!(o, OutputOpts { exposure: 0.5, tonemap: ToneMap::Reinhard, transfer: Transfer::Linear, ..OutputOpts::default() });
        let g = OutputOpts::parse(&serde_json::json!({ "glare": { "strength": 0.1, "core": 1 } })).glare.unwrap();
        assert!((g.core - 1f64.to_radians()).abs() < 1e-12 && g.strength == 0.1);
        assert_eq!(OutputOpts::parse(&serde_json::json!({ "glare": true })).glare, Some(Glare::default()));
    }

    #[test]
    fn test_writer_for_extension() {
        assert!(writer_for("a/b.png", OutputOpts::default()).is_ok());
        assert!(writer_for("a/b.PNG", OutputOpts::default()).is_ok());
        assert!(writer_for("a/b.exr", OutputOpts::default()).is_ok());
        assert!(writer_for("a/b.jpg", OutputOpts::default()).is_err());
    }

    #[test]
    #[ignore = "needs scripts/fetch-references.sh"]
    fn test_reads_reference_exr() {
        // Bitterli's Cornell box reference: half-float, PIZ-compressed, from Tungsten.
        let img = read_linear("references/cornell-box/TungstenRender.exr").unwrap();
        assert_eq!((img.width, img.height), (1024, 1024));
        let mean = img.pixels.iter().map(|c| c.rgb.sum() / 3.).sum::<f64>() / img.pixels.len() as f64;
        assert!(mean > 0.05 && mean < 5., "mean {}", mean);
        // The light (radiance 17, 12, 4) is far brighter than 1: the file is linear, not
        // tone-mapped.
        let brightest = img.pixels.iter().map(|c| c.rgb.x).fold(0., f64::max);
        assert!((brightest - 17.).abs() < 0.5, "brightest red {}", brightest);
    }

    #[test]
    fn test_exr_round_trip_keeps_linear_values() {
        // Values a PNG couldn't hold: above 1, and finer than 1/255.
        let pixels = vec![Color::new(0.001, 0.5, 17.), Color::new(4., 0.25, 1e-6)];
        let img = LinearImage { width: 2, height: 1, pixels: pixels.clone() };
        let dir = std::env::temp_dir().join(format!("rays-exr-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("t.exr");
        let path = path.to_str().unwrap();
        writer_for(path, OutputOpts::default()).unwrap().write(&img, path).unwrap();
        let back = read_linear(path).unwrap();
        assert_eq!((back.width, back.height), (2, 1));
        for (a, b) in back.pixels.iter().zip(&pixels) {
            assert!((a.rgb - b.rgb).norm() <= b.rgb.norm() * 1e-6, "{:?} vs {:?}", a, b);
        }
    }
}
