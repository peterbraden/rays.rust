//! # Image textures
//!
//! Colour, or another surface parameter, read from an image by a surface's texture coordinates
//! (`Geometry::uv`) (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., chapter
//! 10, "Textures and Materials"; cited below as PBR).
//!
//! ## Filtering (PBR sections 10.1 and 10.4)
//!
//! A pixel sees a patch of the surface, which covers a patch of the texture: from a distant or
//! steeply tilted surface, many texels. Point-sampling the texture there aliases, into moiré
//! and shimmer. A MIP map ("multum in parvo"; Lance Williams, "Pyramidal Parametrics",
//! SIGGRAPH 1983, doi:10.1145/800059.801126) keeps the image at a pyramid of resolutions, each
//! half the last, so a lookup can read the level whose texels are about the size of the
//! footprint, interpolating bilinearly within the two nearest levels and linearly between them
//! (trilinear filtering).
//!
//! The footprint comes from how texture coordinates change from pixel to pixel: ray
//! differentials (Homan Igehy, "Tracing Ray Differentials", SIGGRAPH 1999,
//! doi:10.1145/311535.311555). pbrt-v4 approximates them everywhere, even after bounces, by
//! the camera's (PBR section 10.1.1, `Camera::Approximate_dp_dxy`): rays through the
//! neighbouring pixels, from the camera to the tangent plane at the hit, give ∂p/∂x and ∂p/∂y,
//! shrunk by max(1/8, 1/√spp), as many samples per pixel already average over it. Here ∂p/∂x
//! maps to texture space by differencing the shape's own coordinates over a small step along it
//! (so it works for any shape),
//! and the filter's width is twice the largest of the four derivatives, as pbrt's trilinear
//! filter takes.
//!
//! ## Images
//!
//! PNG, JPEG and the image crate's other formats, and OpenEXR; 8-bit images are taken to be
//! sRGB-encoded and decoded to linear, floating-point images to be linear already (`"linear"`
//! overrides). The pyramid's levels are box-filtered, each texel the mean of four (fewer at an
//! odd edge), rather than resampled to powers of two as pbrt does. Images are loaded once,
//! however many materials use them.
use crate::color::Color;
use crate::intersection::Intersection;
use crate::na::{Vector2, Vector3};
use crate::scene::Scene;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wrap {
    Repeat,
    Clamp,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    Point,
    Bilinear,
    Trilinear,
    /// Elliptically weighted averaging over the footprint's ellipse (see `Image::ewa`).
    Ewa,
}

/// The longest an EWA ellipse may be, relative to its width (pbrt's `maxAnisotropy`): beyond,
/// it is widened, which bounds the texels a lookup reads.
const MAX_ANISOTROPY: f64 = 8.;

struct Level {
    width: usize,
    height: usize,
    texels: Vec<Color>,
}

/// An image as a MIP pyramid, in linear values.
pub struct Image {
    levels: Vec<Level>,
}

impl Image {
    /// From linear texels, row by row from the top.
    pub fn new(width: usize, height: usize, texels: Vec<Color>) -> Image {
        assert!(width > 0 && height > 0 && texels.len() == width * height, "An image needs width x height texels");
        let mut levels = vec![Level { width, height, texels }];
        while levels.last().is_some_and(|l| l.width > 1 || l.height > 1) {
            let l = levels.last().unwrap();
            let (w, h) = (l.width.div_ceil(2), l.height.div_ceil(2));
            let mut texels = Vec::with_capacity(w * h);
            for y in 0..h {
                for x in 0..w {
                    let mut sum = Color::black();
                    let mut n = 0.;
                    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                        let (sx, sy) = (2 * x + dx, 2 * y + dy);
                        if sx < l.width && sy < l.height {
                            sum = sum + l.texels[sy * l.width + sx];
                            n += 1.;
                        }
                    }
                    texels.push(sum / n);
                }
            }
            levels.push(Level { width: w, height: h, texels });
        }
        Image { levels }
    }

    /// Load an image file; 8-bit images decoded from sRGB unless `linear`.
    pub fn load(path: &str, linear: Option<bool>) -> Result<Image, String> {
        let bytes = crate::assets::read(path).map_err(|e| format!("Can't read texture: {}", e))?;
        let img = image::load_from_memory(&bytes).map_err(|e| format!("Can't read texture {}: {}", path, e))?;
        let float = matches!(img.color(), image::ColorType::Rgb32F | image::ColorType::Rgba32F);
        let linear = linear.unwrap_or(float);
        let rgb = img.to_rgb32f();
        let (w, h) = (rgb.width() as usize, rgb.height() as usize);
        let decode = |v: f32| {
            let v = v as f64;
            if linear { v } else if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        let texels = rgb.pixels().map(|p| Color::new(decode(p[0]), decode(p[1]), decode(p[2]))).collect();
        Ok(Image::new(w, h, texels))
    }

    /// Load `path` once, however often asked.
    /// The mean of the texels (the pyramid's top level).
    pub fn average(&self) -> Color {
        self.levels.last().unwrap().texels[0]
    }

    /// Width and height, in texels.
    pub fn size(&self) -> (usize, usize) {
        (self.levels[0].width, self.levels[0].height)
    }

    pub fn shared(path: &str, linear: Option<bool>) -> Arc<Image> {
        type Cache = Mutex<HashMap<(String, Option<bool>), Arc<Image>>>;
        static CACHE: OnceLock<Cache> = OnceLock::new();
        let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        let key = (path.to_string(), linear);
        if let Some(i) = cache.lock().unwrap().get(&key) {
            return i.clone();
        }
        let image = Arc::new(Image::load(path, linear).unwrap_or_else(|e| panic!("{}", e)));
        cache.lock().unwrap().insert(key, image.clone());
        image
    }

    pub fn levels(&self) -> usize {
        self.levels.len()
    }

    fn texel(&self, level: usize, x: i64, y: i64, wrap: Wrap) -> Color {
        let l = &self.levels[level];
        let (w, h) = (l.width as i64, l.height as i64);
        let (x, y) = match wrap {
            Wrap::Repeat => (x.rem_euclid(w), y.rem_euclid(h)),
            Wrap::Clamp => (x.clamp(0, w - 1), y.clamp(0, h - 1)),
            Wrap::Black => {
                if x < 0 || y < 0 || x >= w || y >= h {
                    return Color::black();
                }
                (x, y)
            },
        };
        l.texels[(y * w + x) as usize]
    }

    /// Bilinear within one level. (u, v) with v up, as texture coordinates are; rows run down.
    fn bilinear(&self, level: usize, uv: &Vector2<f64>, wrap: Wrap) -> Color {
        let l = &self.levels[level];
        let x = uv.x * l.width as f64 - 0.5;
        let y = (1. - uv.y) * l.height as f64 - 0.5;
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let (x0, y0) = (x0 as i64, y0 as i64);
        self.texel(level, x0, y0, wrap) * ((1. - fx) * (1. - fy))
            + self.texel(level, x0 + 1, y0, wrap) * (fx * (1. - fy))
            + self.texel(level, x0, y0 + 1, wrap) * ((1. - fx) * fy)
            + self.texel(level, x0 + 1, y0 + 1, wrap) * (fx * fy)
    }

    /// Elliptically weighted averaging (Paul S. Heckbert, "Fundamentals of Texture Mapping and
    /// Image Warping", MS thesis, UC Berkeley, 1989; PBR section 10.4.4, `MIPMap::EWA`): the
    /// footprint's two axes, ∂(s, t)/∂x and ∂(s, t)/∂y, define an ellipse in texture space, and the
    /// texels in it are averaged with Gaussian weights falling to zero at its edge, at the level
    /// whose texels match its minor axis. Seen at a grazing angle, the footprint is long and
    /// thin, and trilinear filtering, which must cover it with a circle as wide as its long axis,
    /// blurs across the short one; EWA doesn't. (s, t) here are texture coordinates with t
    /// running down the image's rows; d0 and d1 the axes in the same units.
    pub fn ewa(&self, st: &Vector2<f64>, d0: &Vector2<f64>, d1: &Vector2<f64>, wrap: Wrap) -> Color {
        let (mut d0, mut d1) = (*d0, *d1);
        if d0.norm_squared() < d1.norm_squared() {
            std::mem::swap(&mut d0, &mut d1);
        }
        let longer = d0.norm();
        let mut shorter = d1.norm();
        if shorter * MAX_ANISOTROPY < longer && shorter > 0. {
            let scale = longer / (shorter * MAX_ANISOTROPY);
            d1 *= scale;
            shorter *= scale;
        }
        if shorter == 0. {
            return self.bilinear(0, &Vector2::new(st.x, 1. - st.y), wrap);
        }
        let size = self.levels[0].width.max(self.levels[0].height) as f64;
        let lod = (size * shorter).log2().max(0.);
        let il = lod.floor() as usize;
        let f = lod - il as f64;
        self.ewa_level(il, st, &d0, &d1, wrap) * (1. - f) + self.ewa_level(il + 1, st, &d0, &d1, wrap) * f
    }

    fn ewa_level(&self, level: usize, st: &Vector2<f64>, d0: &Vector2<f64>, d1: &Vector2<f64>, wrap: Wrap) -> Color {
        if level >= self.levels.len() {
            return self.levels.last().unwrap().texels[0];
        }
        let l = &self.levels[level];
        let (w, h) = (l.width as f64, l.height as f64);
        let (s, t) = (st.x * w - 0.5, st.y * h - 0.5);
        let (d0, d1) = (Vector2::new(d0.x * w, d0.y * h), Vector2::new(d1.x * w, d1.y * h));
        // The ellipse A s² + B s t + C t² = 1, a pixel wider so it always holds a texel.
        let mut a = d0.y * d0.y + d1.y * d1.y + 1.;
        let mut b = -2. * (d0.x * d0.y + d1.x * d1.y);
        let mut c = d0.x * d0.x + d1.x * d1.x + 1.;
        let inv_f = 1. / (a * c - b * b * 0.25);
        a *= inv_f;
        b *= inv_f;
        c *= inv_f;
        let det = -b * b + 4. * a * c;
        let inv_det = 1. / det;
        let (u_sqrt, v_sqrt) = ((det * c).max(0.).sqrt(), (a * det).max(0.).sqrt());
        let (s0, s1) = ((s - 2. * inv_det * u_sqrt).ceil() as i64, (s + 2. * inv_det * u_sqrt).floor() as i64);
        let (t0, t1) = ((t - 2. * inv_det * v_sqrt).ceil() as i64, (t + 2. * inv_det * v_sqrt).floor() as i64);
        let mut sum = Color::black();
        let mut weights = 0.;
        // Gaussian falloff, zero at the edge (pbrt's MIPFilterLUT, with α = 2).
        let gauss = |r2: f64| (-2. * r2).exp() - (-2f64).exp();
        for it in t0..=t1 {
            let tt = it as f64 - t;
            for is in s0..=s1 {
                let ss = is as f64 - s;
                let r2 = a * ss * ss + b * ss * tt + c * tt * tt;
                if r2 < 1. {
                    let wgt = gauss(r2);
                    sum = sum + self.texel(level, is, it, wrap) * wgt;
                    weights += wgt;
                }
            }
        }
        if weights > 0. { sum / weights } else { self.texel(level, s.round() as i64, t.round() as i64, wrap) }
    }

    /// The image at `uv` over a footprint `width` (in texture coordinates) across.
    pub fn lookup(&self, uv: &Vector2<f64>, width: f64, filter: Filter, wrap: Wrap) -> Color {
        match filter {
            Filter::Ewa => {
                let d = Vector2::new(width / 2., 0.);
                self.ewa(&Vector2::new(uv.x, 1. - uv.y), &d, &Vector2::new(0., width / 2.), wrap)
            },
            Filter::Point => {
                let l = &self.levels[0];
                self.texel(0, (uv.x * l.width as f64).floor() as i64, ((1. - uv.y) * l.height as f64).floor() as i64, wrap)
            },
            Filter::Bilinear => self.bilinear(0, uv, wrap),
            Filter::Trilinear => {
                // The level whose texels are the footprint's size: level 0's texels are 1 / size
                // across, and each level doubles them.
                let size = self.levels[0].width.max(self.levels[0].height) as f64;
                let level = (width.max(1e-300) * size).log2().clamp(0., (self.levels.len() - 1) as f64);
                let l0 = level.floor() as usize;
                if l0 + 1 >= self.levels.len() {
                    return self.bilinear(self.levels.len() - 1, uv, wrap);
                }
                let f = level - l0 as f64;
                self.bilinear(l0, uv, wrap) * (1. - f) + self.bilinear(l0 + 1, uv, wrap) * f
            },
        }
    }
}

/// An image placed on a surface by its texture coordinates.
pub struct ImageMap {
    pub image: Arc<Image>,
    /// Texture coordinates are scaled by `scale`, then offset.
    pub scale: Vector2<f64>,
    pub offset: Vector2<f64>,
    pub filter: Filter,
    pub wrap: Wrap,
}

impl ImageMap {
    fn mapped(&self, uv: &Vector2<f64>) -> Vector2<f64> {
        uv.component_mul(&self.scale) + self.offset
    }

    /// The image's value at texture coordinates `uv`, bilinearly interpolated at full
    /// resolution (for emission, whose footprint isn't known when a light is sampled).
    pub fn at(&self, uv: &Vector2<f64>) -> Color {
        self.image.lookup(&self.mapped(uv), 0., Filter::Bilinear, self.wrap)
    }

    /// The image's value at the hit, filtered over its footprint (see the module's notes); the
    /// first texel's colour if the shape has no texture coordinates.
    pub fn eval(&self, i: &Intersection, s: &Scene) -> Color {
        let Some(uv) = i.uv() else { return self.image.texel(0, 0, 0, Wrap::Clamp) };
        let st = self.mapped(&uv);
        let derivatives = |dx: &Vector3<f64>, dy: &Vector3<f64>| {
            const STEP: f64 = 1e-3;
            let g = |d: &Vector3<f64>| {
                i.object.geometry.uv(&(i.point + d * STEP), i.face).map_or(Vector2::zeros(), |o| (self.mapped(&o) - st) / STEP)
            };
            (g(dx), g(dy))
        };
        if self.filter == Filter::Ewa {
            return match footprint(i, s) {
                Some((dx, dy)) => {
                    let (a, b) = derivatives(&dx, &dy);
                    // t runs down the rows, against v.
                    self.image.ewa(&Vector2::new(st.x, 1. - st.y), &Vector2::new(a.x, -a.y), &Vector2::new(b.x, -b.y), self.wrap)
                },
                None => self.image.lookup(&st, 0., Filter::Bilinear, self.wrap),
            };
        }
        let width = match (self.filter, footprint(i, s)) {
            (Filter::Trilinear, Some((dx, dy))) => {
                let (a, b) = derivatives(&dx, &dy);
                2. * a.x.abs().max(a.y.abs()).max(b.x.abs()).max(b.y.abs())
            },
            _ => 0.,
        };
        self.image.lookup(&st, width, self.filter, self.wrap)
    }
}

/// ∂p/∂x and ∂p/∂y at a hit, by the camera's approximation (see the module's notes), or None
/// without a camera to approximate by.
pub fn footprint(i: &Intersection, s: &Scene) -> Option<(Vector3<f64>, Vector3<f64>)> {
    let f = s.footprint.as_ref()?;
    let to = i.point - f.location;
    let d = to.norm();
    if d == 0. {
        return None;
    }
    let w = to / d;
    let n = i.normal;
    let plane = |dir: Vector3<f64>| -> Option<Vector3<f64>> {
        let denom = dir.dot(&n);
        if denom.abs() < 1e-9 { return None; }
        let t = to.dot(&n) / denom;
        Some(f.location + dir * t - i.point)
    };
    let dx = plane(w + f.right * f.spread)?;
    let dy = plane(w + f.up * f.spread)?;
    Some((dx, dy))
}

/// What a scene's camera gives for texture filtering: its position, its image axes, and the
/// angle between neighbouring pixels' rays, shrunk for the samples per pixel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Footprint {
    pub location: Vector3<f64>,
    pub right: Vector3<f64>,
    pub up: Vector3<f64>,
    pub spread: f64,
}

/// A colour: constant, from an image, or a grey level from a function graph (`graph.rs`),
/// evaluated at the hit with its normal, so it can mask by slope and altitude.
pub enum ColorTexture {
    Constant(Color),
    Image(ImageMap),
    Graph(std::sync::Arc<crate::graph::Graph>),
    Slope(SlopeTexture),
    Terrain(TerrainTexture),
    Pattern(PatternTexture),
}

/// How a colour map blends between its entries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interpolation {
    Linear,
    /// Smoothstepped, so bands meet without a crease.
    Smooth,
    /// Each entry's colour up to the next: hard edges (still averaged over the footprint, by
    /// the pattern's filtering, only where that blurs the pattern itself).
    Step,
}

/// A pattern coloured by a map: POV-Ray's `pigment { pattern color_map { [v colour] ... } }`.
/// The pattern is a function graph (`graph.rs`), evaluated where the surface was hit, with its
/// detail filtered to the pixel's footprint (`graph::Context::at_hit_filtered`); its value
/// picks a place among the map's entries, sorted by value, and the colour there is blended
/// from the two entries about it. An entry may itself be any colour texture, a pattern too
/// (POV-Ray's `pigment_map`): only the two entries about the value are evaluated.
pub struct PatternTexture {
    pub pattern: crate::graph::Graph,
    pub map: Vec<(f64, ColorTexture)>,
    pub interpolation: Interpolation,
}

impl PatternTexture {
    /// The two entries about `v` and the blend between them.
    fn place(&self, v: f64) -> (usize, usize, f64) {
        let n = self.map.len();
        let k = self.map.partition_point(|(at, _)| *at <= v);
        if k == 0 {
            return (0, 0, 0.);
        }
        if k == n {
            return (n - 1, n - 1, 0.);
        }
        let (a, b) = (self.map[k - 1].0, self.map[k].0);
        let t = if b > a { (v - a) / (b - a) } else { 1. };
        let t = match self.interpolation {
            Interpolation::Linear => t,
            Interpolation::Smooth => t * t * (3. - 2. * t),
            Interpolation::Step => 0.,
        };
        (k - 1, k, t)
    }

    pub fn eval(&self, i: &Intersection, s: &Scene) -> Color {
        let v = self.pattern.eval(&crate::graph::Context::at_hit_filtered(i, s));
        let (a, b, t) = self.place(v);
        if t <= 0. || a == b {
            self.map[a].1.eval(i, s)
        } else if t >= 1. {
            self.map[b].1.eval(i, s)
        } else {
            self.map[a].1.eval(i, s).blend(&self.map[b].1.eval(i, s), t)
        }
    }

    /// The map's colour at a value, with its entries' mean colours.
    pub fn at_value(&self, v: f64) -> Color {
        let (a, b, t) = self.place(v);
        self.map[a].1.average().blend(&self.map[b].1.average(), t)
    }

    fn average(&self) -> Color {
        // The map's mean over the pattern's range, or its entries' if that is unbounded.
        let (lo, hi) = self.pattern.range();
        let (lo, hi) = if lo.is_finite() && hi.is_finite() && hi > lo { (lo, hi) } else {
            (self.map[0].0, self.map[self.map.len() - 1].0)
        };
        let n = 32;
        (0 .. n).map(|k| self.at_value(lo + (hi - lo) * (k as f64 + 0.5) / n as f64)).fold(Color::black(), |a, c| a + c) * (1. / n as f64)
    }
}

/// Which feature of a `mesas` heightmap a `TerrainTexture` follows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainFeature {
    /// Its dry washes (`heightmap::wash_value`).
    Washes,
    /// The hollows of its floor (`heightmap::hollow_value`).
    Hollows,
    /// The joints between its slickrock domes (`heightmap::dome_value`).
    Joints,
}

/// A grey level from where a point lies among a `mesas` heightmap's features, given the same
/// size the heightmap was (its `floor_scale`, or for joints its `domes`): so ground cover can
/// follow them, as sand gathers in washes, hollows and the joints of slickrock.
pub struct TerrainTexture {
    pub feature: TerrainFeature,
    pub scale: f64,
    /// Widens (above 1) or narrows the feature.
    pub spread: f64,
    pub noise: crate::noise::PerlinNoise,
}

impl TerrainTexture {
    pub fn amount(&self, point: &Vector3<f64>) -> f64 {
        use crate::procedural::heightmap::{dome_value, hollow_value, wash_value};
        let (x, z) = (point.x, point.z);
        let v = match self.feature {
            TerrainFeature::Washes => wash_value(&self.noise, x, z, self.scale),
            TerrainFeature::Hollows => hollow_value(&self.noise, x, z, self.scale),
            TerrainFeature::Joints => {
                let d = dome_value(&self.noise, x, z, self.scale);
                1. - (d / 0.25).clamp(0., 1.)
            },
        };
        (v * self.spread).clamp(0., 1.)
    }
}

/// A grey level from how steep the surface is: 0 where it is flatter than `from` degrees, 1
/// where steeper than `to`, smoothly between, the slope jittered by up to `jitter` degrees of
/// noise `jitter_scale` across, so the boundary is ragged. As a `mix` amount, it puts one
/// material on cliffs and another on the ground, as rock shows where it's too steep for soil
/// to lie. With `by_height`, the same from the elevation instead, `from` and `to` in scene
/// units and `jitter` too.
pub struct SlopeTexture {
    /// By elevation (y, scene units) rather than slope.
    pub by_height: bool,
    pub from: f64,
    pub to: f64,
    pub jitter: f64,
    pub jitter_scale: f64,
    pub noise: crate::noise::SeededNoise,
}

impl SlopeTexture {
    pub fn amount(&self, point: &Vector3<f64>, normal: &Vector3<f64>) -> f64 {
        let slope = if self.by_height { point.y } else { normal.y.abs().clamp(0., 1.).acos().to_degrees() };
        let jitter = if self.jitter > 0. { self.jitter * self.noise.fbm(point / self.jitter_scale.max(1e-9), 3) } else { 0. };
        let t = ((slope + jitter - self.from) / (self.to - self.from).max(1e-9)).clamp(0., 1.);
        t * t * (3. - 2. * t)
    }
}

impl From<Color> for ColorTexture {
    fn from(c: Color) -> ColorTexture {
        ColorTexture::Constant(c)
    }
}

impl ColorTexture {
    pub fn eval(&self, i: &Intersection, s: &Scene) -> Color {
        match self {
            ColorTexture::Constant(c) => *c,
            ColorTexture::Image(m) => m.eval(i, s),
            ColorTexture::Graph(g) => {
                let v = g.eval(&crate::graph::Context::at_hit_filtered(i, s));
                Color::new(v, v, v)
            },
            ColorTexture::Slope(t) => { let a = t.amount(&i.point, &i.normal); Color::new(a, a, a) },
            ColorTexture::Terrain(t) => { let a = t.amount(&i.point); Color::new(a, a, a) },
            ColorTexture::Pattern(t) => t.eval(i, s),
        }
    }

    /// The constant colour, or the image's mean (for estimates that need one colour).
    pub fn average(&self) -> Color {
        match self {
            ColorTexture::Constant(c) => *c,
            ColorTexture::Image(m) => m.image.average(),
            // Without sampling the surface, the middle of its range.
            ColorTexture::Graph(g) => {
                let (lo, hi) = g.range();
                let v = if lo.is_finite() && hi.is_finite() { 0.5 * (lo + hi) } else { 0.5 };
                Color::new(v, v, v)
            },
            ColorTexture::Slope(_) | ColorTexture::Terrain(_) => Color::new(0.5, 0.5, 0.5),
            ColorTexture::Pattern(t) => t.average(),
        }
    }
}

/// A tangent-space normal map: each texel's RGB, mapped from [0, 1] to [-1, 1], is a normal in
/// the frame (∂p/∂u, ∂p/∂v, n) (PBR section 10.5.3, `NormalMap`), its tilt scaled by
/// `strength`. ∂p/∂u and ∂p/∂v come from the shape's texture coordinates, by differencing them
/// along two directions in the tangent plane.
pub struct NormalMap {
    pub map: ImageMap,
    pub strength: f64,
}

impl NormalMap {
    pub fn perturb(&self, i: &Intersection, s: &Scene) -> Vector3<f64> {
        let n = i.normal;
        let Some((dpdu, dpdv)) = tangents(i) else { return n };
        let c = self.map.eval(i, s);
        let t = Vector3::new(2. * c.rgb.x - 1., 2. * c.rgb.y - 1., 2. * c.rgb.z - 1.);
        let local = Vector3::new(t.x * self.strength, t.y * self.strength, t.z.max(1e-3));
        // An orthonormal frame from the derivatives, Gram-Schmidt against the normal.
        let tu = (dpdu - n * n.dot(&dpdu)).try_normalize(1e-12);
        let Some(tu) = tu else { return n };
        let tv = n.cross(&tu) * if n.cross(&tu).dot(&dpdv) < 0. { -1. } else { 1. };
        (tu * local.x + tv * local.y + n * local.z).normalize()
    }
}

/// ∂p/∂u and ∂p/∂v at a hit, from the shape's texture coordinates differenced along two
/// directions in the tangent plane (solving the 2 x 2 system that maps (du, dv) to them).
pub fn tangents(i: &Intersection) -> Option<(Vector3<f64>, Vector3<f64>)> {
    let n = i.normal;
    let (a, b) = crate::material::phase::orthonormal_basis(&n);
    let h = 1e-4 * (1. + i.point.abs().max()).min(1e-2 * (1. + i.dist));
    let uv0 = i.uv()?;
    let du = |d: Vector3<f64>| i.object.geometry.uv(&(i.point + d * h), i.face).map(|x| (x - uv0) / h);
    let (ga, gb) = (du(a)?, du(b)?);
    // [ga gb] maps (α, β) along a, b to (du, dv); invert to get the tangent-plane directions of
    // unit du and unit dv.
    let det = ga.x * gb.y - gb.x * ga.y;
    if det.abs() < 1e-12 {
        return None;
    }
    let dpdu = (a * gb.y - b * ga.y) / det;
    let dpdv = (b * ga.x - a * gb.x) / det;
    Some((dpdu, dpdv))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checker(n: usize) -> Image {
        let texels = (0..n * n).map(|k| if (k % n + k / n) % 2 == 0 { Color::white() } else { Color::black() }).collect();
        Image::new(n, n, texels)
    }

    #[test]
    fn test_pyramid_averages() {
        let img = checker(8);
        assert_eq!(img.levels(), 4);
        // The top of the pyramid is the image's mean.
        let top = img.levels.last().unwrap().texels[0];
        assert!((top.rgb.x - 0.5).abs() < 1e-12);
        // Odd sizes shrink too.
        let odd = Image::new(5, 3, vec![Color::white(); 15]);
        assert_eq!(odd.levels.iter().map(|l| (l.width, l.height)).collect::<Vec<_>>(), vec![(5, 3), (3, 2), (2, 1), (1, 1)]);
    }

    #[test]
    fn test_lookups() {
        let img = checker(8);
        // At a texel's centre, any filter gives the texel at a tiny footprint.
        let uv = Vector2::new(0.5 / 8., 1. - 0.5 / 8.);
        for f in [Filter::Point, Filter::Bilinear, Filter::Trilinear] {
            assert!((img.lookup(&uv, 1e-6, f, Wrap::Repeat).rgb.x - 1.).abs() < 1e-9, "{:?}", f);
        }
        // A footprint as wide as the image reads its mean: grey.
        assert!((img.lookup(&uv, 1., Filter::Trilinear, Wrap::Repeat).rgb.x - 0.5).abs() < 1e-9);
        // Two texels wide, the level of 2 x 2 blocks: also grey for a checker.
        assert!((img.lookup(&Vector2::new(0.3, 0.6), 2. / 8., Filter::Trilinear, Wrap::Repeat).rgb.x - 0.5).abs() < 1e-9);
        // Wrapping.
        assert_eq!(img.lookup(&Vector2::new(1. + 0.5 / 8., 1. - 0.5 / 8.), 0., Filter::Point, Wrap::Repeat), Color::white());
        assert_eq!(img.lookup(&Vector2::new(-0.5, 0.5), 0., Filter::Point, Wrap::Black), Color::black());
    }

    /// EWA follows the footprint's shape: on stripes across s, an ellipse long along t keeps
    /// the stripes, where trilinear filtering, as wide as the long axis, greys them.
    #[test]
    fn test_ewa_is_anisotropic() {
        let n = 64;
        let stripes = Image::new(n, n, (0..n * n).map(|k| if (k % n / 4) % 2 == 0 { Color::white() } else { Color::black() }).collect());
        // The middle of a white stripe (texels 0 to 3).
        let st = Vector2::new(2. / n as f64, 0.5);
        let long = Vector2::new(0., 0.25);
        let short = Vector2::new(0.5 / n as f64, 0.);
        let ewa = stripes.ewa(&st, &long, &short, Wrap::Repeat).rgb.x;
        let tri = stripes.lookup(&Vector2::new(st.x, 1. - st.y), 2. * 0.25, Filter::Trilinear, Wrap::Repeat).rgb.x;
        assert!(ewa > 0.85, "EWA {}", ewa);
        assert!((tri - 0.5).abs() < 0.1, "trilinear {}", tri);
        // A round footprint as large as the image reads its mean.
        let big = stripes.ewa(&st, &Vector2::new(1., 0.), &Vector2::new(0., 1.), Wrap::Repeat).rgb.x;
        assert!((big - 0.5).abs() < 0.05, "{}", big);
    }

    #[test]
    fn test_srgb_images_load_linear() {
        let dir = std::env::temp_dir().join(format!("rays-texture-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("grey.png");
        image::RgbImage::from_pixel(2, 2, image::Rgb([188, 188, 188])).save(&p).unwrap();
        let img = Image::load(p.to_str().unwrap(), None).unwrap();
        // sRGB 188 is linear 0.5.
        assert!((img.levels[0].texels[0].rgb.x - 0.5).abs() < 0.005, "{:?}", img.levels[0].texels[0]);
        let raw = Image::load(p.to_str().unwrap(), Some(true)).unwrap();
        assert!((raw.levels[0].texels[0].rgb.x - 188. / 255.).abs() < 1e-6);
    }
}

#[cfg(test)]
mod pattern_tests {
    use super::*;

    #[test]
    fn test_color_maps_blend_between_entries() {
        let map = |interpolation| PatternTexture {
            pattern: crate::graph::Graph::parse(&serde_json::json!({"node": "gradient"})).unwrap(),
            map: vec![(0., Color::black().into()), (1., Color::white().into()), (2., Color::new(1., 0., 0.).into())],
            interpolation,
        };
        let linear = map(Interpolation::Linear);
        assert_eq!(linear.at_value(-5.), Color::black());
        assert_eq!(linear.at_value(0.25), Color::new(0.25, 0.25, 0.25));
        assert_eq!(linear.at_value(1.5), Color::new(1., 0.5, 0.5));
        assert_eq!(linear.at_value(9.), Color::new(1., 0., 0.));
        assert_eq!(map(Interpolation::Step).at_value(0.9), Color::black());
        let smooth = map(Interpolation::Smooth).at_value(0.25).rgb.x;
        assert!(smooth > 0. && smooth < 0.25);
    }
}
