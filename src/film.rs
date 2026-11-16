//! # Film and sensor
//!
//! What the camera's sensor does with the light the renderer computes, before the image is
//! encoded for display (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed.,
//! section 5.4, "Film and Imaging"; cited below as PBR):
//!
//! - Exposure. The light reaching the film is integrated over the exposure time, and a digital
//!   sensor's gain is set by its ISO rating; pbrt scales the sensor's response by their
//!   product, its "imaging ratio", t ISO / 100 (PBR section 5.4.2, `PixelSensor`). A lens's
//!   f-number changes the light reaching the film too, which the realistic camera (`lens`)
//!   accounts for by tracing its aperture.
//! - White balance. Under a warm light, a white wall photographs orange unless the camera
//!   compensates; our eyes adapt so that it looks white. A white balance for an illuminant maps
//!   that illuminant's white to the output's white (D65 for sRGB): a von Kries transform,
//!   scaling the three cone responses independently, in the cone space of the Bradford
//!   transform (K. M. Lam, "Metamerism and Colour Constancy", PhD thesis, University of
//!   Bradford, 1985; the matrix as in CIE 160:2004), as pbrt's `WhiteBalance` does (PBR
//!   section 5.4.2, after Fairchild, "Color Appearance Models").
//! - The output colour space. The renderer works in linear sRGB (Rec. 709 primaries, D65
//!   white). Wider gamuts hold colours sRGB can't, such as those of dispersion: Display P3,
//!   Rec. 2020 (ITU-R BT.2020) and ACES2065-1 (SMPTE ST 2065-1, whose white is ACES's own,
//!   reached by Bradford adaptation). For EXR output, which stores linear values; a PNG is
//!   still encoded with the chosen transfer curve, and carries no colour-space tag.
//!
//! The matrices taking linear sRGB to the others were generated with the colour-science Python
//! package (version 0.4.7, `matrix_RGB_to_RGB` with Bradford adaptation).
use crate::color::Color;
use crate::na::{Matrix3, Vector3};
use crate::spectrum::{planck, spectrum_to_xyz, srgb_to_xyz};
use serde_json::Value;

/// The output's colour space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorSpace {
    Srgb,
    DisplayP3,
    Rec2020,
    Aces2065_1,
}

impl ColorSpace {
    pub fn parse(name: &str) -> ColorSpace {
        match name {
            "srgb" => ColorSpace::Srgb,
            "display-p3" => ColorSpace::DisplayP3,
            "rec2020" => ColorSpace::Rec2020,
            "aces2065-1" => ColorSpace::Aces2065_1,
            n => panic!("Unknown colour space '{}': expected \"srgb\", \"display-p3\", \"rec2020\" or \"aces2065-1\"", n),
        }
    }

    /// The matrix from linear sRGB.
    pub fn matrix_from_srgb(&self) -> Matrix3<f64> {
        match self {
            ColorSpace::Srgb => Matrix3::identity(),
            ColorSpace::DisplayP3 => Matrix3::new(
                0.8225336512, 0.1775458067, 0.0000536948,
                0.0331170846, 0.9668481430, -0.0000031500,
                0.0170564417, 0.0724006114, 0.9104892659),
            ColorSpace::Rec2020 => Matrix3::new(
                0.6274413721, 0.3292974595, 0.0433514584,
                0.0690276171, 0.9195806669, 0.0113614226,
                0.0163642351, 0.0880171625, 0.8955649727),
            ColorSpace::Aces2065_1 => Matrix3::new(
                0.4396430040, 0.3830054714, 0.1773993089,
                0.0897157319, 0.8134750538, 0.0967822524,
                0.0175127205, 0.1115514385, 0.8708827930),
        }
    }
}

/// The chromaticity (x, y) of CIE daylight at correlated colour temperature `kelvin`, 4000 K to
/// 25000 K (CIE 015:2018, section 4.1, the D illuminants).
pub fn daylight_xy(kelvin: f64) -> (f64, f64) {
    let t = kelvin.clamp(4000., 25000.);
    let x = if t <= 7000. {
        -4.6070e9 / t.powi(3) + 2.9678e6 / t.powi(2) + 0.09911e3 / t + 0.244063
    } else {
        -2.0064e9 / t.powi(3) + 1.9018e6 / t.powi(2) + 0.24748e3 / t + 0.237040
    };
    (x, -3. * x * x + 2.870 * x - 0.275)
}

/// The chromaticity of a blackbody at `kelvin`.
pub fn planckian_xy(kelvin: f64) -> (f64, f64) {
    let xyz = spectrum_to_xyz(|l| planck(l, kelvin));
    let sum = xyz.sum();
    (xyz.x / sum, xyz.y / sum)
}

/// XYZ of unit luminance at chromaticity (x, y).
fn white_xyz((x, y): (f64, f64)) -> Vector3<f64> {
    Vector3::new(x / y, 1., (1. - x - y) / y)
}

/// The white of an illuminant, as XYZ of unit luminance: `"D50"`, `"D55"`, `"D65"`, `"D75"`
/// (daylight), `"A"` (incandescent, a 2856 K blackbody), or a colour temperature in kelvin:
/// daylight from 4000 K, a blackbody below.
pub fn illuminant_white(v: &Value) -> Vector3<f64> {
    match v {
        Value::Number(n) => {
            let t = n.as_f64().unwrap();
            assert!(t > 0., "A white balance temperature must be positive, got {}", t);
            white_xyz(if t >= 4000. { daylight_xy(t) } else { planckian_xy(t) })
        },
        // The D illuminants' temperatures are nominal: 5000 K etc. on an old scale, 1.4388 /
        // 1.4380 times that on the current one (CIE 015:2018, note 4.1).
        Value::String(s) => match s.as_str() {
            "D65" => srgb_to_xyz(&Color::white()),
            "D50" => white_xyz(daylight_xy(5000. * 1.4388 / 1.4380)),
            "D55" => white_xyz(daylight_xy(5500. * 1.4388 / 1.4380)),
            "D75" => white_xyz(daylight_xy(7500. * 1.4388 / 1.4380)),
            "A" => white_xyz(planckian_xy(2856.)),
            n => panic!("Unknown illuminant '{}': expected D50, D55, D65, D75, A or a temperature", n),
        },
        _ => panic!("Unexpected white balance {}", v),
    }
}

/// Bradford's matrix from XYZ to its cone space.
fn bradford() -> Matrix3<f64> {
    Matrix3::new(
        0.8951, 0.2664, -0.1614,
        -0.7502, 1.7135, 0.0367,
        0.0389, -0.0685, 1.0296)
}

/// The von Kries transform in Bradford's cone space taking XYZ under white `from` to XYZ under
/// white `to`.
pub fn adaptation(from: &Vector3<f64>, to: &Vector3<f64>) -> Matrix3<f64> {
    let m = bradford();
    let (a, b) = (m * from, m * to);
    m.try_inverse().unwrap() * Matrix3::from_diagonal(&b.component_div(&a)) * m
}

/// What the film does to linear sRGB: a scale (the imaging ratio) and a matrix (white balance,
/// then the output colour space).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Film {
    pub scale: f64,
    pub matrix: Matrix3<f64>,
}

impl Default for Film {
    fn default() -> Film {
        Film { scale: 1., matrix: Matrix3::identity() }
    }
}

impl Film {
    /// From the scene's `"output"` block: `"iso"` (default 100) and `"exposure_time"` in seconds
    /// (default 1), `"white_balance"` (an illuminant, see `illuminant_white`; none by default)
    /// and `"color_space"` (`"srgb"`, the default, `"display-p3"`, `"rec2020"` or
    /// `"aces2065-1"`).
    pub fn parse(o: &Value) -> Film {
        let iso = o["iso"].as_f64().unwrap_or(100.);
        let time = o["exposure_time"].as_f64().unwrap_or(1.);
        let balance = match o.get("white_balance") {
            None | Some(Value::Null) => Matrix3::identity(),
            Some(v) => {
                // In sRGB: to XYZ, adapt the illuminant's white to sRGB's, and back.
                let to_xyz = crate::na::Matrix3::from_columns(&[
                    srgb_to_xyz(&Color::new(1., 0., 0.)), srgb_to_xyz(&Color::new(0., 1., 0.)), srgb_to_xyz(&Color::new(0., 0., 1.))]);
                to_xyz.try_inverse().unwrap() * adaptation(&illuminant_white(v), &srgb_to_xyz(&Color::white())) * to_xyz
            },
        };
        let space = o["color_space"].as_str().map_or(ColorSpace::Srgb, ColorSpace::parse);
        Film { scale: time * iso / 100., matrix: space.matrix_from_srgb() * balance }
    }

    pub fn is_identity(&self) -> bool {
        self.scale == 1. && self.matrix == Matrix3::identity()
    }

    /// A pixel through the film.
    pub fn apply(&self, c: &Color) -> Color {
        Color { rgb: self.matrix * c.rgb * self.scale }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::xyz_to_srgb;

    #[test]
    fn test_daylight_chromaticities() {
        // As colour-science computes them.
        let (x, y) = daylight_xy(5000.);
        assert!((x - 0.345741).abs() < 1e-5 && (y - 0.358666).abs() < 1e-5, "{} {}", x, y);
        let (x, y) = daylight_xy(6504.);
        assert!((x - 0.312714).abs() < 1e-5 && (y - 0.329119).abs() < 1e-5, "{} {}", x, y);
        // Illuminant A: (0.44758, 0.40745).
        let (x, y) = planckian_xy(2856.);
        assert!((x - 0.44758).abs() < 2e-3 && (y - 0.40745).abs() < 2e-3, "{} {}", x, y);
    }

    #[test]
    fn test_white_balance_makes_the_illuminant_white() {
        // The colour of a 3200 K light, balanced for 3200 K, is grey.
        let f = Film::parse(&serde_json::json!({ "white_balance": 3200 }));
        let light = xyz_to_srgb(&white_xyz(planckian_xy(3200.)));
        let seen = f.apply(&light);
        assert!((seen.rgb.x / seen.rgb.z - 1.).abs() < 2e-3 && (seen.rgb.y / seen.rgb.z - 1.).abs() < 2e-3, "{:?}", seen);
        // Balanced for D65, nothing changes.
        let d65 = Film::parse(&serde_json::json!({ "white_balance": "D65" }));
        assert!((d65.matrix - Matrix3::identity()).abs().max() < 1e-12);
    }

    #[test]
    fn test_exposure_and_colour_spaces() {
        let f = Film::parse(&serde_json::json!({ "iso": 400, "exposure_time": 0.25 }));
        assert_eq!(f.scale, 1.);
        let f = Film::parse(&serde_json::json!({ "iso": 200 }));
        assert_eq!(f.apply(&Color::new(0.1, 0.2, 0.3)), Color::new(0.2, 0.4, 0.6));
        // Every space keeps white white (to the precision of the matrices) and widens the gamut:
        // sRGB's pure red is inside Rec. 2020, so has no negative component there.
        for s in ["display-p3", "rec2020", "aces2065-1"] {
            let f = Film::parse(&serde_json::json!({ "color_space": s }));
            assert!((f.apply(&Color::white()).rgb - Vector3::repeat(1.)).abs().max() < 2e-4, "{}", s);
            assert!(f.apply(&Color::new(1., 0., 0.)).rgb.min() >= 0., "{}", s);
        }
        assert!(Film::default().is_identity());
    }
}
