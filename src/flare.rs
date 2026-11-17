//! # Lens flare
//!
//! A bright light in front of a camera lights the lens as well as the image. Some of its light
//! reflects off one glass surface, back to another, and forward again to the film: a ghost, an
//! out-of-focus image of the aperture somewhere along the line from the light through the
//! image's centre, one for every pair of surfaces. And the aperture's edges diffract it into a
//! starburst of spikes. Both are drawn here after rendering, from the finished image, like the
//! glare (`glare`).
//!
//! ## Light sources
//!
//! Ghosts are faint, a few parts in a hundred thousand of the light, and only the very brightest
//! things make visible ones: the sun, lamps, a glint of the sun. Pixels brighter than
//! `threshold` (as displayed, after exposure) are grouped into connected regions, and each
//! region (up to `sources` of them, brightest first) is a light: its direction from its
//! brightness-weighted centre, its energy the sum of its pixels. Whatever is hidden behind
//! something doesn't reach the image, so doesn't flare.
//!
//! ## Ghosts
//!
//! After Matthias B. Hullin, Elmar Eisemann, Hans-Peter Seidel and Sungkil Lee,
//! "Physically-Based Real-Time Lens Flare Rendering", ACM Transactions on Graphics 30(4)
//! (SIGGRAPH 2011), doi:10.1145/2010324.1965003. For each pair of surfaces i < j (not the
//! stop), a grid of parallel rays from the light's direction, across the front element, is
//! traced through the lens system (`lens`): forward to j, where it reflects; back to i, where it
//! reflects again; and forward to the film. Each cell of the grid, mapped by its four corners'
//! rays, spreads its share of the light over the quadrilateral it lands on, so a ghost is as
//! bright as its rays are dense. Cells with a corner stopped by an aperture are dropped, which
//! shapes each ghost like the stop (round, or a polygon with blades). The light's energy is
//! shared between the grid's cells that reach the film directly, so the ghosts are in proportion
//! to the light as rendered.
//!
//! What each surface reflects is given by Fresnel's equations, averaged over polarisations,
//! through the air-glass surfaces' anti-reflection coating, found by thin-film interference
//! (Born and Wolf, "Principles of Optics", 7th ed., section 1.6): by default a modern
//! three-layer coating (`Coating::Multi`, 0.1-0.25% a surface), or a 1950s single layer of
//! magnesium fluoride (1.3%, the purple and amber ghosts of older lenses), or none (4% and
//! more). A ghost reflects twice, so its strength goes as the square: a single coating's ghosts
//! are ten to fifty times a multicoated lens's. Each colour channel is traced at its own
//! wavelength (610, 550 and 465 nm), with glass dispersion estimated from each glass's index
//! (lens prescriptions give no Abbe number): an Abbe number from the trend of the glass map,
//! V = 70 - 60 (n - 1.45) within [25, 70], and Cauchy's equation through n_d. The coatings'
//! reflectance is averaged across each channel's band (±30 nm), as a sensor's channels
//! integrate it: at one wavelength a coating's colour is exaggerated.
//!
//! ## Starburst
//!
//! The aperture's edges diffract light into lines perpendicular to them. In the Fraunhofer
//! pattern of an aperture of area A, a straight edge of length L sends a fraction
//! (L / A) λ / (4π² θ²) of the light per radian along the line at angle θ from the centre, to
//! each side (the edge's stationary-phase term; for a slit, both edges together give the tail
//! of its sinc², λ / (2π² w θ²)). A regular polygon of n blades has n edges, and parallel
//! edges add, so n spikes if n is even and 2n if odd; a round stop makes none.
//!
//! Real blades are not straight: they are arcs, cut so that the aperture is round when fully
//! open, and nearly a polygon only stopped well down. Along a curved edge the normal turns, so
//! each direction in a fan as wide as the angle it turns through gets its light from one point
//! of the edge: the spike spreads into a wedge, and falls as 1/θ³ rather than 1/θ², as a
//! circle's edge does (Born and Wolf, section 8.5; Keller's geometrical theory of
//! diffraction). This is why a sunstar tapers and fades, and why it is sharpest with the lens
//! stopped down. The blades' radius is the open stop's over `blade_curvature` (1 by default;
//! 0 for straight blades), so the fan is the side's length over that: about 12° for six blades
//! at f/11 on an f/2 lens. Fans wider than the gap between edges overlap into the round
//! aperture's halo. The light is spread over each fan at each radius, every bright pixel's,
//! by convolution, and further for red than for blue, as λ goes.
//!
//! The starburst is built from the aperture's actual outline (`outline`): its edge, traced
//! with each blade's own shape, cut into short pieces, each sending its light along its
//! normal, (ds / A) λ / (4π² θ²) per radian of θ, splatted out from the centre. This gives
//! straight spikes, curved blades' fans, and a round stop's halo alike, and conserves the
//! light exactly.
//!
//! ## A real lens
//!
//! No lens is perfect, and its flaws are what make a real flare uneven. With `imperfection`
//! (1 by default, 0 for a perfect lens; seeded by `seed`):
//!
//! - Each blade is a little out of place: turned by about 0.4°, its edge 1% nearer or further,
//!   its curvature 10% more or less (`imperfect_blades`), so the stop is an irregular polygon,
//!   and its spikes and ghosts are uneven. (Opposite spikes stay equal: every aperture's
//!   diffraction pattern is symmetric through its centre.)
//! - A blade's edge is rough, a few microns out of true over fractions of a millimetre (RMS
//!   slope 0.004 radians), too fine to change its outline but not its normal: the light
//!   facing each direction varies, and fans break into fine radial streaks.
//! - The front element is dirty where the light crosses it (`dirt`): dust covering 2 parts in
//!   10⁴, each speck scattering the light it blocks into an Airy pattern (Babinet), a soft,
//!   uneven glow; and three hairline cleaning marks 1-5 µm wide, each a streak across it in a
//!   random direction, redder further out.
//! - Coatings are thinner towards each surface's rim, 6-18% at the edge, as evaporated
//!   coatings land, so a ghost's colour changes across it.
//!
//! Both are physical in strength by default; `ghosts` and `starburst` scale them.
use crate::color::Color;
use crate::glare::fft2d;
use crate::lens::{refract, intersect_spherical, LensSystem};
use crate::na::{Vector2, Vector3};
use crate::output::LinearImage;
use rustfft::num_complex::Complex;
use rayon::prelude::*;
use std::f64::consts::PI;

/// Wavelengths (nm) of the red, green and blue channels.
const WAVELENGTHS: [f64; 3] = [610., 550., 465.];

/// Each channel's band, about its wavelength: ghosts take the coatings' reflectance averaged
/// over it, as a sensor's channel does, rather than at one wavelength, which exaggerates their
/// colour.
const BAND: [f64; 5] = [-30., -15., 0., 15., 30.];

#[derive(Debug, Clone, PartialEq)]
pub struct Flare {
    /// The lens, focused, with its stop and blades.
    pub lens: LensSystem,
    /// The film's width and height, metres.
    pub film: (f64, f64),
    /// Pixels brighter than this, after exposure, are lights.
    pub threshold: f64,
    /// At most this many lights.
    pub sources: usize,
    /// Scales the ghosts and the starburst (zero for none).
    pub ghosts: f64,
    pub starburst: f64,
    /// Rays across the front element, per side.
    pub grid: usize,
    /// The air-glass surfaces' coating.
    pub coating: Coating,
    /// How far the lens departs from a perfect one (0 for perfect): the roughness of the
    /// blades' edges and how unevenly the coatings are laid (the blades' own irregularity is
    /// in the lens's `BladeShape`, from `imperfect_blades`).
    pub imperfection: f64,
    /// Seeds the imperfections, so a lens is the same lens from frame to frame.
    pub seed: u64,
}

/// The RMS slope of a blade's edge, radians, at imperfection 1: stamped blades are a few
/// microns out of true over fractions of a millimetre.
const EDGE_SLOPE: f64 = 0.004;
/// Spatial frequencies of an edge's roughness, cycles per stop radius.
const EDGE_WAVES: usize = 48;

fn gaussian(rng: &mut crate::geometry::Pcg32) -> f64 {
    let (u, v) = (1. - rng.next_f64(), rng.next_f64());
    (-2. * u.ln()).sqrt() * (2. * PI * v).cos()
}

/// [2 J₁(x) / x]², the Airy pattern, 1 at the centre; J₁ by Abramowitz and Stegun's
/// approximations 9.4.4 and 9.4.6 (errors under 1e-7).
fn airy(x: f64) -> f64 {
    let x = x.abs();
    let j1_over_x = if x <= 3. {
        let t = (x / 3.).powi(2);
        0.5 + t * (-0.56249985 + t * (0.21093573 + t * (-0.03954289 + t * (0.00443319 + t * (-0.00031761 + t * 0.00001109)))))
    } else {
        let t = 3. / x;
        let f = 0.79788456 + t * (0.00000156 + t * (0.01659667 + t * (0.00017105 + t * (-0.00249511 + t * (0.00113653 - t * 0.00020033)))));
        let phase = x - 2.35619449 + t * (0.12499612 + t * (0.00005650 + t * (-0.00637879 + t * (0.00074348 + t * (0.00079824 - t * 0.00029166)))));
        f * phase.cos() / (x * x.sqrt())
    };
    (2. * j1_over_x).powi(2)
}

/// A real diaphragm's blades, `n` of them, each a little out of place: turned by about 0.4°,
/// 1% nearer or further, its curvature 10% more or less (standard deviations, at
/// imperfection 1), so the stop is an irregular polygon, and its spikes and ghosts are too.
pub fn imperfect_blades(n: usize, imperfection: f64, seed: u64) -> Vec<crate::lens::BladeJitter> {
    let mut rng = crate::geometry::Pcg32::new(seed, 0x626c_6164);
    (0..n).map(|_| crate::lens::BladeJitter {
        angle: 0.4f64.to_radians() * imperfection * gaussian(&mut rng),
        reach: 1. + 0.01 * imperfection * gaussian(&mut rng),
        curvature: (1. + 0.1 * imperfection * gaussian(&mut rng)).max(0.2),
    }).collect()
}

/// A light found in the image: its direction in camera space and its energy (the sum of its
/// pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Source {
    pub direction: Vector3<f64>,
    pub energy: Color,
}

/// Reflectance at an interface between media of index `n0` and `n2` (in the order the light
/// crosses them), at cosine `cos0` in the first, averaged over polarisations, through thin films
/// `layers` (index, thickness in nm; in the order the light crosses them): by their
/// characteristic matrices (Born and Wolf, "Principles of Optics", 7th ed., section 1.6;
/// Macleod, "Thin-Film Optical Filters", chapter 2), with complex angles where a layer is
/// crossed beyond its critical angle.
pub fn reflectance(cos0: f64, n0: f64, n2: f64, lambda: f64, layers: &[(f64, f64)]) -> f64 {
    let sin0 = (1. - cos0 * cos0).max(0.).sqrt();
    if n0 * sin0 >= n2 {
        return 1.;
    }
    let one = Complex::new(1., 0.);
    let i = Complex::new(0., 1.);
    // cos θ in a medium of index n (Snell's law), complex beyond the critical angle.
    let cos_in = |n: f64| (one - Complex::new((n0 * sin0 / n).powi(2), 0.)).sqrt();
    let mut total = 0.;
    for s_polarised in [true, false] {
        // Each medium's tilted admittance: n cos θ for s, n / cos θ for p.
        let eta = |n: f64| if s_polarised { cos_in(n) * n } else { Complex::new(n, 0.) / cos_in(n) };
        let (mut b, mut c) = (one, eta(n2));
        // [B, C] = M_1 ... M_k [1, η_substrate], so the last layer's matrix applies first.
        for &(n, d) in layers.iter().rev() {
            let delta = cos_in(n) * (2. * PI * n * d / lambda);
            let e = eta(n);
            let (cs, sn) = (delta.cos(), delta.sin());
            (b, c) = (cs * b + i * sn / e * c, i * e * sn * b + cs * c);
        }
        let e0 = eta(n0);
        let r = (e0 * b - c) / (e0 * b + c);
        total += 0.5 * r.norm_sqr();
    }
    total.min(1.)
}

/// What an air-glass surface is coated with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coating {
    /// Bare glass: 4-8% a surface.
    Uncoated,
    /// A quarter wave of magnesium fluoride (n = 1.38) at 550 nm, the single coating of
    /// 1940s-1970s lenses: about 1.3% on crown glass, tinting ghosts purple.
    Single,
    /// A broadband three-layer coating, as on modern lenses: quarter, half and quarter waves at
    /// 510 nm of magnesium fluoride, zirconia (2.1) and a layer matched to the glass, of index
    /// 0.84 + 0.53 n (the best such layer for glass of index n, between 1.45 and 1.9): 0.1-0.25%
    /// averaged over the visible, more at its ends.
    Multi,
}

impl Coating {
    pub fn parse(name: &str) -> Option<Coating> {
        match name {
            "none" | "uncoated" => Some(Coating::Uncoated),
            "single" => Some(Coating::Single),
            "multi" | "multicoated" => Some(Coating::Multi),
            _ => None,
        }
    }

    /// The layers on glass of index `glass`, from the air's side: (index, thickness in nm).
    pub fn layers(&self, glass: f64) -> Vec<(f64, f64)> {
        match self {
            Coating::Uncoated => vec![],
            Coating::Single => vec![(1.38, 550. / (4. * 1.38))],
            Coating::Multi => {
                let matched = (0.84 + 0.53 * glass).clamp(1.45, 2.0);
                vec![(1.38, 510. / (4. * 1.38)), (2.1, 510. / (2. * 2.1)), (matched, 510. / (4. * matched))]
            },
        }
    }
}

/// A glass's index at `lambda` nm from its index at the d line (see the module's notes).
fn dispersed(nd: f64, lambda: f64) -> f64 {
    if nd <= 1. {
        return nd;
    }
    let v = (70. - 60. * (nd - 1.45)).clamp(25., 70.);
    // n_F - n_C = (n_d - 1) / V = b (1/λ_F² - 1/λ_C²), λ in µm.
    let b = (nd - 1.) / v / (1. / 0.4861f64.powi(2) - 1. / 0.6563f64.powi(2));
    let a = nd - b / 0.5876f64.powi(2);
    a + b / (lambda * 1e-3).powi(2)
}

/// How a ray meets a surface on a ghost's path.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Refract,
    Reflect,
}

impl Flare {
    /// The surfaces' positions along the axis, in lens space (film at 0, lens towards -z).
    fn positions(&self) -> Vec<f64> {
        let mut z = -self.lens.front_z();
        self.lens.interfaces.iter().map(|e| { let here = z; z += e.thickness; here }).collect()
    }

    /// The pairs of surfaces (i < j) that make ghosts: neither the stop.
    pub fn ghost_pairs(&self) -> Vec<(usize, usize)> {
        let n = self.lens.interfaces.len();
        let mut pairs = vec![];
        for j in 0..n {
            for i in 0..j {
                if !self.lens.interfaces[i].is_stop() && !self.lens.interfaces[j].is_stop() {
                    pairs.push((i, j));
                }
            }
        }
        pairs
    }

    /// The index of the medium before surface `k` (towards the scene) and after it, at `lambda`.
    fn media(&self, k: usize, lambda: f64) -> (f64, f64) {
        let e = &self.lens.interfaces;
        let before = if k == 0 || e[k - 1].eta == 0. { 1. } else { e[k - 1].eta };
        let after = if e[k].eta == 0. { 1. } else { e[k].eta };
        (dispersed(before, lambda), dispersed(after, lambda))
    }

    /// Trace a ray (camera space, from the scene) along the path for `ghost` (None: straight
    /// through) at `lambda`: where it lands on the film (camera space x, y) and the fraction of
    /// its light that gets there.
    fn trace(&self, z: &[f64], o: &Vector3<f64>, d: &Vector3<f64>, ghost: Option<(usize, usize)>, lambda: f64) -> Option<(Vector2<f64>, f64)> {
        let n = self.lens.interfaces.len();
        let mut path: Vec<(usize, bool, Step)> = Vec::with_capacity(n + 4);
        match ghost {
            None => path.extend((0..n).map(|k| (k, true, Step::Refract))),
            Some((i, j)) => {
                path.extend((0..j).map(|k| (k, true, Step::Refract)));
                path.push((j, true, Step::Reflect));
                path.extend((i + 1..j).rev().map(|k| (k, false, Step::Refract)));
                path.push((i, false, Step::Reflect));
                path.extend((i + 1..n).map(|k| (k, true, Step::Refract)));
            },
        }
        let (mut o, mut d) = (Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z).normalize());
        // The light's share at each wavelength across the channel's band.
        let mut weight = [1.; BAND.len()];
        for (k, forward, step) in path {
            let e = &self.lens.interfaces[k];
            let (t, normal) = if e.is_stop() {
                if d.z.abs() < 1e-12 { return None; }
                ((z[k] - o.z) / d.z, Vector3::zeros())
            } else {
                intersect_spherical(e.radius, z[k] + e.radius, &o, &d)?
            };
            if t <= 0. {
                return None;
            }
            let p = o + d * t;
            if !self.lens.within(e, p.x, p.y) {
                return None;
            }
            o = p;
            if e.is_stop() {
                continue;
            }
            let (before, after) = self.media(k, lambda);
            let (ni, nt) = if forward { (before, after) } else { (after, before) };
            let cos = normal.dot(&-d).abs();
            // An air-glass surface is coated; its layers run from the air's side.
            let mut layers = if ni == 1. { self.coating.layers(nt) } else if nt == 1. { self.coating.layers(ni) } else { vec![] };
            // Coatings are evaporated from below a rack of elements, and land thinner where a
            // surface turns away from the source: towards its rim, by 6-18% here (by surface),
            // so a ghost's colour changes across it.
            if self.imperfection > 0. && !layers.is_empty() {
                let falloff = self.imperfection * (0.06 + 0.12 * crate::geometry::Pcg32::new(self.seed, 0x636f_6174 + k as u64).next_f64());
                let thinner = (1. - falloff * (p.x * p.x + p.y * p.y) / (e.aperture * e.aperture)).max(0.5);
                for l in layers.iter_mut() {
                    l.1 *= thinner;
                }
            }
            if nt == 1. {
                layers.reverse();
            }
            let reflect = step == Step::Reflect;
            for (wgt, offset) in weight.iter_mut().zip(BAND) {
                let r = reflectance(cos, ni, nt, lambda + offset, &layers);
                *wgt *= if reflect { r } else { 1. - r };
            }
            match step {
                Step::Reflect => d = d - normal * (2. * d.dot(&normal)),
                Step::Refract => d = refract(&-d, &normal, nt / ni)?,
            }
        }
        if d.z <= 0. {
            return None;
        }
        let t = -o.z / d.z;
        Some((Vector2::new(o.x + d.x * t, o.y + d.y * t), weight.iter().sum::<f64>() / BAND.len() as f64))
    }

    /// The rays for a light from camera-space `direction` through a lattice of (g + 1)²
    /// points on the front element's plane, spanning [lo, hi]: for each, where it lands on the
    /// film and with what weight.
    #[allow(clippy::too_many_arguments)]
    fn lattice_in(&self, z: &[f64], direction: &Vector3<f64>, ghost: Option<(usize, usize)>, lambda: f64,
                  lo: Vector2<f64>, hi: Vector2<f64>, g: usize) -> Vec<Option<(Vector2<f64>, f64)>> {
        let front = self.lens.front_z();
        let d = -direction.normalize();
        (0..(g + 1) * (g + 1)).map(|k| {
            let (i, j) = (k % (g + 1), k / (g + 1));
            let p = Vector3::new(lo.x + (hi.x - lo.x) * i as f64 / g as f64, lo.y + (hi.y - lo.y) * j as f64 / g as f64, front);
            // Start a little in front of the lens, back along the ray.
            self.trace(z, &(p - d * 0.01), &d, ghost, lambda)
        }).collect()
    }

    /// The rays along a path through a lattice of `grid` cells a side, laid over just the part of
    /// the front element whose rays get through: found by a coarse lattice over the whole
    /// element first, since at small apertures only a sliver of it passes the stop. The lattice,
    /// its corners and its size; None if nothing gets through.
    #[allow(clippy::type_complexity)]
    fn lattice(&self, z: &[f64], direction: &Vector3<f64>, ghost: Option<(usize, usize)>, lambda: f64)
               -> Option<(Vec<Option<(Vector2<f64>, f64)>>, Vector2<f64>, Vector2<f64>)> {
        const COARSE: usize = 48;
        let r = self.lens.interfaces[0].aperture;
        let (lo, hi) = (Vector2::repeat(-r), Vector2::repeat(r));
        let coarse = self.lattice_in(z, direction, ghost, lambda, lo, hi, COARSE);
        let step = 2. * r / COARSE as f64;
        let mut bounds: Option<(Vector2<f64>, Vector2<f64>)> = None;
        for (k, v) in coarse.iter().enumerate() {
            if v.is_some() {
                let p = Vector2::new(-r + step * (k % (COARSE + 1)) as f64, -r + step * (k / (COARSE + 1)) as f64);
                bounds = Some(match bounds { None => (p, p), Some((a, b)) => (a.inf(&p), b.sup(&p)) });
            }
        }
        // A path that only a few coarse points miss entirely still has a sliver: widen by a cell.
        let (a, b) = bounds?;
        let (a, b) = ((a - Vector2::repeat(step)).sup(&lo), (b + Vector2::repeat(step)).inf(&hi));
        Some((self.lattice_in(z, direction, ghost, lambda, a, b, self.grid), a, b))
    }

    /// Film (camera space) to image position in pixels, (column, row from the top): the image
    /// is inverted on the film, as in `lens::RealisticCamera`.
    fn to_pixel(&self, f: &Vector2<f64>, w: usize, h: usize) -> Vector2<f64> {
        let u = 0.5 - f.x / self.film.0;
        let v = 0.5 - f.y / self.film.1;
        Vector2::new(u * w as f64, (1. - v) * h as f64)
    }

    /// Camera-space direction of image position (column, row) in pixels, by the film's own
    /// pinhole projection.
    fn direction_of(&self, x: f64, y: f64, w: usize, h: usize) -> Vector3<f64> {
        let focal = self.lens.focal_length().unwrap_or(0.05);
        let (u, v) = (x / w as f64, 1. - y / h as f64);
        Vector3::new((u - 0.5) * self.film.0 / focal, (v - 0.5) * self.film.1 / focal, 1.).normalize()
    }

    /// The lights in `img`: connected regions of pixels brighter than the threshold, at most
    /// `sources`, brightest first.
    pub fn find_sources(&self, img: &LinearImage, exposure: f64) -> Vec<Source> {
        let (w, h) = (img.width, img.height);
        let threshold = self.threshold / exposure.max(1e-300);
        let bright: Vec<bool> = img.pixels.iter().map(|c| c.rgb.max() > threshold).collect();
        let mut seen = vec![false; w * h];
        let mut sources = vec![];
        for start in 0..w * h {
            if !bright[start] || seen[start] {
                continue;
            }
            let (mut energy, mut centre, mut weight) = (Color::black(), Vector2::zeros(), 0.);
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(i) = stack.pop() {
                let c = img.pixels[i];
                let l = crate::color::luminance(&c).max(0.);
                energy = energy + c;
                centre += Vector2::new((i % w) as f64 + 0.5, (i / w) as f64 + 0.5) * l;
                weight += l;
                let (x, y) = (i % w, i / w);
                let mut visit = |j: usize| if bright[j] && !seen[j] { seen[j] = true; stack.push(j); };
                if x > 0 { visit(i - 1); }
                if x + 1 < w { visit(i + 1); }
                if y > 0 { visit(i - w); }
                if y + 1 < h { visit(i + w); }
            }
            if weight > 0. {
                let c = centre / weight;
                sources.push(Source { direction: self.direction_of(c.x, c.y, w, h), energy });
            }
        }
        sources.sort_by(|a, b| crate::color::luminance(&b.energy).total_cmp(&crate::color::luminance(&a.energy)));
        sources.truncate(self.sources);
        sources
    }

    /// The ghosts of `source`, as an image of `w` x `h` (pixel-sum units, like the image's).
    pub fn ghosts_of(&self, source: &Source, w: usize, h: usize) -> Vec<Color> {
        let z = self.positions();
        let g = self.grid;
        let cell_ok = |lat: &[Option<(Vector2<f64>, f64)>], i: usize, j: usize| {
            [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)].iter().all(|(a, b)| lat[b * (g + 1) + a].is_some())
        };
        let cell_area = |lo: &Vector2<f64>, hi: &Vector2<f64>| (hi.x - lo.x) * (hi.y - lo.y) / (g * g) as f64;
        // The area of the front element through which the light reaches the film directly: its
        // energy per unit of that area.
        let (direct, dlo, dhi) = match self.lattice(&z, &source.direction, None, WAVELENGTHS[1]) {
            Some(l) => l,
            None => return vec![Color::black(); w * h],
        };
        let reaching = (0..g * g).filter(|k| cell_ok(&direct, k % g, k / g)).count() as f64 * cell_area(&dlo, &dhi);
        if reaching <= 0. {
            return vec![Color::black(); w * h];
        }
        let jobs: Vec<((usize, usize), usize)> = self.ghost_pairs().into_iter().flat_map(|p| (0..3).map(move |c| (p, c))).collect();
        jobs.par_iter().fold(|| vec![Color::black(); w * h], |mut out, &(pair, c)| {
            let lambda = WAVELENGTHS[c];
            let (lat, lo, hi) = match self.lattice(&z, &source.direction, Some(pair), lambda) {
                Some(l) => l,
                None => return out,
            };
            let per_cell = source.energy.rgb[c] * self.ghosts * cell_area(&lo, &hi) / reaching;
            for j in 0..g {
                for i in 0..g {
                    if !cell_ok(&lat, i, j) {
                        continue;
                    }
                    let corner = |a: usize, b: usize| lat[b * (g + 1) + a].unwrap();
                    let q = [corner(i, j), corner(i + 1, j), corner(i + 1, j + 1), corner(i, j + 1)];
                    let energy = per_cell * q.iter().map(|(_, w)| w).sum::<f64>() / 4.;
                    if energy <= 0. {
                        continue;
                    }
                    let p: Vec<Vector2<f64>> = q.iter().map(|(f, _)| self.to_pixel(f, w, h)).collect();
                    splat_triangle(&mut out, w, h, c, &p[0], &p[1], &p[2], energy / 2.);
                    splat_triangle(&mut out, w, h, c, &p[0], &p[2], &p[3], energy / 2.);
                }
            }
            out
        }).reduce(|| vec![Color::black(); w * h], |a, b| a.iter().zip(&b).map(|(x, y)| *x + *y).collect())
    }

    /// The entrance pupil's radius: the stop's circle (before its blades) seen through the
    /// elements in front of it, found by tracing rays parallel to the axis that just graze it.
    fn pupil_radius(&self) -> f64 {
        let round = Flare { lens: LensSystem { blades: None, ..self.lens.clone() }, ..self.clone() };
        let z = round.positions();
        let (mut lo, mut hi) = (0., self.lens.interfaces[0].aperture);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let o = Vector3::new(mid, 0., self.lens.front_z() + 0.01);
            if round.trace(&z, &o, &Vector3::new(0., 0., -1.), None, WAVELENGTHS[1]).is_some() { lo = mid; } else { hi = mid; }
        }
        if lo > 0. { lo } else { self.lens.stop_diameter().unwrap_or(0.01) / 2. }
    }

    /// The entrance pupil's outline, for its diffraction (see `starburst_of`): for each short
    /// piece of its edge, the angle of its outward normal and its length, and the pupil's area;
    /// metres, as seen through the elements in front of the stop. A blade's edge is the true
    /// one (`LensSystem::within`, with each blade's own shape), its normal turned by the edge's
    /// roughness, too fine to change the outline but not its diffraction.
    fn outline(&self) -> Option<(Vec<(f64, f64)>, f64)> {
        let n = self.lens.blades.map_or(0, |b| b.0);
        let e = self.lens.interfaces.iter().find(|e| e.is_stop())?;
        let ap = e.aperture;
        let magnification = self.pupil_radius() / ap;
        // The edge, by its radius in each of many directions from the centre.
        const M: usize = 8192;
        let points: Vec<Vector2<f64>> = (0..M).map(|i| {
            let psi = 2. * PI * i as f64 / M as f64;
            let (c, s) = (psi.cos(), psi.sin());
            let (mut lo, mut hi) = (0., 2. * ap);
            for _ in 0..48 {
                let mid = 0.5 * (lo + hi);
                if self.lens.within(e, mid * c, mid * s) { lo = mid; } else { hi = mid; }
            }
            Vector2::new(lo * c, lo * s)
        }).collect();
        let area: f64 = (0..M).map(|i| { let (p, q) = (points[i], points[(i + 1) % M]); p.x * q.y - q.x * p.y }).sum::<f64>() / 2.;
        // Each blade's roughness: waves along the edge with random phases and equal slopes,
        // their RMS EDGE_SLOPE at imperfection 1.
        let waves: Vec<Vec<(f64, f64)>> = (0..n).map(|k| {
            let mut rng = crate::geometry::Pcg32::new(self.seed, 0x6564_6765 + k as u64);
            (1..=EDGE_WAVES).map(|m| (m as f64, 2. * PI * rng.next_f64())).collect()
        }).collect();
        // A round stop's edge is the polished rim of a fixed aperture: smooth.
        let slope = if n > 0 { EDGE_SLOPE * self.imperfection * (2. / EDGE_WAVES as f64).sqrt() } else { 0. };
        let pieces = (0..M).map(|i| {
            let (p, q) = (points[i], points[(i + 1) % M]);
            let t = q - p;
            let ds = t.norm();
            let mid = (p + q) / 2.;
            // Outward, for an outline running anticlockwise: the tangent turned clockwise.
            let mut normal = (-t.x).atan2(t.y);
            if slope > 0. {
                // The blade this piece is on: the one it lies on the edge of.
                let k = (0..n).max_by(|a, b| self.lens.outside_blade(ap, *a, mid.x, mid.y)
                    .total_cmp(&self.lens.outside_blade(ap, *b, mid.x, mid.y))).unwrap();
                let (alpha, _, _) = self.lens.blade(ap, k).unwrap();
                let u = (-mid.x * alpha.sin() + mid.y * alpha.cos()) / ap;
                normal += slope * waves[k].iter().map(|(m, phase)| (2. * PI * m * u + phase).cos()).sum::<f64>();
            }
            (normal.rem_euclid(2. * PI), ds * magnification)
        }).collect();
        Some((pieces, area.abs() * magnification * magnification))
    }

    /// Dirt on the front element, where the light crosses it (the entrance pupil, of area
    /// `pupil`): fine scratches and cleaning marks, as (direction, width, length), and dust
    /// specks, as diameters; metres. At imperfection 1, a lens in use: three hairline cleaning
    /// marks 1-5 µm wide and up to 2 mm long, and dust covering 2 parts in 10⁴ of the pupil.
    fn dirt(&self, pupil: f64) -> (Vec<(f64, f64, f64)>, Vec<f64>) {
        if self.imperfection <= 0. {
            return (vec![], vec![]);
        }
        let mut rng = crate::geometry::Pcg32::new(self.seed, 0x6469_7274);
        let diameter = 2. * (pupil / PI).sqrt();
        let count = 3. * self.imperfection;
        let scratches = (0..count.floor() as usize + (rng.next_f64() < count.fract()) as usize).map(|_| {
            let width = 1e-6 * 5f64.powf(rng.next_f64());
            let length = (0.3e-3 + 1.7e-3 * rng.next_f64()).min(diameter);
            (PI * rng.next_f64(), width, length)
        }).collect();
        let (mut specks, mut covered) = (vec![], 0.);
        while covered < 2e-4 * self.imperfection * pupil {
            let d = 10e-6 * 8f64.powf(rng.next_f64());
            covered += PI * d * d / 4.;
            specks.push(d);
        }
        (scratches, specks)
    }

    /// Add the dirt's diffraction (`dirt`) at wavelength `lambda` to the starburst's kernel `k`
    /// (`pw` × `ph`, centred at the origin and wrapped round). Each speck or scratch scatters
    /// the light it blocks, its share of the pupil `area` (Babinet's principle), into its
    /// Fraunhofer pattern: a speck of diameter d into an Airy pattern, (a / A)(a / λ²)
    /// [2 J₁(x) / x]² per steradian, x = π d θ / λ; a scratch b wide and l long into a streak
    /// across it, (b l / A)(b / λ) sinc²(π b θ / λ) per radian along it (it is λ / l across,
    /// under a pixel). Each averaged over the channel's band, and a scratch over ±30% of its
    /// width, as no scratch is even along its length: that washes out the dark bands of one
    /// width's sinc², leaving the streak redder further out.
    #[allow(clippy::too_many_arguments)]
    fn add_dirt(&self, k: &mut [f64], pw: usize, ph: usize, reach: usize, pixel_angle: f64, lambda: f64, area: f64,
                scratches: &[(f64, f64, f64)], specks: &[f64]) {
        let band: Vec<f64> = BAND.iter().map(|o| lambda + o * 1e-9).collect();
        for &(direction, b, l) in scratches {
            let across = direction + PI / 2.;
            let mut r = 0.5;
            while r < reach as f64 {
                let dr = (0.1 * r).clamp(0.05, 0.5);
                let theta = (r + dr / 2.) * pixel_angle;
                const WIDTHS: [f64; 7] = [0.7, 0.8, 0.9, 1., 1.1, 1.2, 1.3];
                let density = band.iter().flat_map(|lam| WIDTHS.iter().map(move |s| (lam, s))).map(|(lam, s)| {
                    let bw = b * s;
                    let x = PI * bw * theta / lam;
                    // Each width blocks its own share: b s l / A, so weighted by s.
                    s * bw / lam * (x.sin() / x).powi(2)
                }).sum::<f64>() / (band.len() * WIDTHS.len()) as f64;
                let e = b * l / area * density * dr * pixel_angle;
                let rm = r + dr / 2.;
                for side in [0., PI] {
                    let (x, y) = (rm * (across + side).cos(), -rm * (across + side).sin());
                    let (x0, y0) = (x.floor(), y.floor());
                    let (fx, fy) = (x - x0, y - y0);
                    for (ox, oy, wgt) in [(0., 0., (1. - fx) * (1. - fy)), (1., 0., fx * (1. - fy)), (0., 1., (1. - fx) * fy), (1., 1., fx * fy)] {
                        let ix = ((x0 + ox) as i64).rem_euclid(pw as i64) as usize;
                        let iy = ((y0 + oy) as i64).rem_euclid(ph as i64) as usize;
                        // The pattern is symmetric: each side holds half, ∫₀^∞ = 1/2.
                        k[iy * pw + ix] += e * wgt;
                    }
                }
                r += dr;
            }
        }
        if specks.is_empty() {
            return;
        }
        // The specks' Airy patterns, summed, by radius (quarter-pixel steps).
        let table: Vec<f64> = (0..=(4 * (pw + ph))).map(|i| {
            let theta = (i as f64 / 4.).max(0.125) * pixel_angle;
            specks.iter().map(|d| {
                let a = PI * d * d / 4.;
                band.iter().map(|lam| {
                    let x = PI * d * theta / lam;
                    a / area * a / (lam * lam) * airy(x)
                }).sum::<f64>() / band.len() as f64
            }).sum::<f64>() * pixel_angle * pixel_angle
        }).collect();
        k.par_chunks_mut(pw).enumerate().for_each(|(iy, row)| {
            let dy = if iy < ph / 2 { iy as f64 } else { iy as f64 - ph as f64 };
            for (ix, v) in row.iter_mut().enumerate() {
                let dx = if ix < pw / 2 { ix as f64 } else { ix as f64 - pw as f64 };
                let f = (dx * dx + dy * dy).sqrt() * 4.;
                let i = (f as usize).min(table.len() - 2);
                *v += table[i] + (f - i as f64).min(1.) * (table[i + 1] - table[i]);
            }
        });
    }

    /// The starburst of the bright pixels of `img`, added to it.
    fn starburst_of(&self, img: &LinearImage, exposure: f64, pixel_angle: f64) -> Vec<Color> {
        let (w, h) = (img.width, img.height);
        if self.starburst <= 0. || pixel_angle <= 0. {
            return vec![Color::black(); w * h];
        }
        let threshold = self.threshold / exposure.max(1e-300);
        let bright = |c: &Color| c.rgb.max() > threshold;
        if !img.pixels.iter().any(bright) {
            return vec![Color::black(); w * h];
        }
        let Some((pieces, area)) = self.outline() else { return vec![Color::black(); w * h] };
        let reach = w.max(h);
        let (pw, ph) = ((w + reach).next_power_of_two(), (h + reach).next_power_of_two());
        // The diffraction of each point of the edge, by stationary phase: a piece ds of edge
        // whose outward normal is along φ sends light along ±φ, (ds / A) λ / (4π² θ²) of it per
        // radian of θ (Keller's edge diffraction; it gives the slit's sinc² tail, and the round
        // stop's Airy tail summed over the circle). Each piece splats its light along both
        // directions, step by step out from the centre, bilinearly, so straight edges' spikes
        // and curved edges' fans alike keep exactly their light. Per unit λ: the channels
        // differ only by it.
        let steps: Vec<(f64, f64)> = {
            let (mut v, mut r) = (vec![], 0.5);
            while r < reach as f64 {
                let dr = (0.1 * r).clamp(0.05, 0.5);
                v.push((r, r + dr));
                r += dr;
            }
            v
        };
        let scale = self.starburst / (4. * PI * PI * area * pixel_angle);
        let chunk = pieces.len().div_ceil(rayon::current_num_threads().max(1));
        let kernel = pieces.par_chunks(chunk.max(1)).map(|part| {
            let mut k = vec![0f64; pw * ph];
            for (normal, ds) in part {
                for direction in [*normal, normal + PI] {
                    let (c, sn) = (direction.cos(), direction.sin());
                    for (ra, rb) in &steps {
                        // ∫ dθ / θ² over the step, θ = r × pixel_angle.
                        let e = ds * scale * (1. / ra - 1. / rb);
                        let rm = 0.5 * (ra + rb);
                        // The film's y runs up, the image's rows down.
                        let (x, y) = (rm * c, -rm * sn);
                        let (x0, y0) = (x.floor(), y.floor());
                        let (fx, fy) = (x - x0, y - y0);
                        for (ox, oy, wgt) in [(0., 0., (1. - fx) * (1. - fy)), (1., 0., fx * (1. - fy)), (0., 1., (1. - fx) * fy), (1., 1., fx * fy)] {
                            let ix = ((x0 + ox) as i64).rem_euclid(pw as i64) as usize;
                            let iy = ((y0 + oy) as i64).rem_euclid(ph as i64) as usize;
                            k[iy * pw + ix] += e * wgt;
                        }
                    }
                }
            }
            k
        }).reduce(|| vec![0f64; pw * ph], |mut a, b| { a.iter_mut().zip(&b).for_each(|(x, y)| *x += y); a });
        let (scratches, specks) = self.dirt(area);
        let mut out = vec![Color::black(); w * h];
        for (c, lambda) in WAVELENGTHS.iter().enumerate() {
            let lambda = lambda * 1e-9;
            let mut k: Vec<f64> = kernel.iter().map(|v| v * lambda).collect();
            self.add_dirt(&mut k, pw, ph, reach, pixel_angle, lambda, area, &scratches, &specks);
            let mut kernel: Vec<Complex<f64>> = k.into_iter().map(|v| Complex::new(v, 0.)).collect();
            fft2d(&mut kernel, pw, ph, false);
            let mut source = vec![Complex::new(0., 0.); pw * ph];
            for y in 0..h {
                for x in 0..w {
                    let p = img.pixels[y * w + x];
                    if bright(&p) {
                        source[y * pw + x].re = p.rgb[c];
                    }
                }
            }
            fft2d(&mut source, pw, ph, false);
            for (s, k) in source.iter_mut().zip(&kernel) {
                *s *= k;
            }
            fft2d(&mut source, pw, ph, true);
            let size = (pw * ph) as f64;
            for y in 0..h {
                for x in 0..w {
                    out[y * w + x].rgb[c] += (source[y * pw + x].re / size).max(0.);
                }
            }
        }
        out
    }

    /// `img` with its lights' ghosts and starbursts.
    pub fn apply(&self, img: &LinearImage, exposure: f64, pixel_angle: f64) -> LinearImage {
        let (w, h) = (img.width, img.height);
        let mut pixels = img.pixels.clone();
        if self.ghosts > 0. {
            let mut ghosts = vec![Color::black(); w * h];
            for s in self.find_sources(img, exposure) {
                for (a, g) in ghosts.iter_mut().zip(self.ghosts_of(&s, w, h)) {
                    *a = *a + g;
                }
            }
            // Rasterising cells leaves a faint mesh where they meet; ghosts are smooth.
            for (p, g) in pixels.iter_mut().zip(blur(&ghosts, w, h)) {
                *p = *p + g;
            }
        }
        for (p, s) in pixels.iter_mut().zip(self.starburst_of(img, exposure, pixel_angle)) {
            *p = *p + s;
        }
        LinearImage { width: w, height: h, pixels }
    }
}

/// A Gaussian blur of one pixel's standard deviation, separable, keeping the light that would
/// leave the image's edges at them.
fn blur(img: &[Color], w: usize, h: usize) -> Vec<Color> {
    let k: Vec<f64> = (-3..=3).map(|i: i64| (-(i * i) as f64 / 2.).exp()).collect();
    let total: f64 = k.iter().sum();
    let pass = |src: &[Color], horizontal: bool| -> Vec<Color> {
        let mut out = vec![Color::black(); w * h];
        for y in 0..h {
            for x in 0..w {
                let mut acc = Color::black();
                for (o, kv) in (-3..=3).zip(&k) {
                    let (sx, sy) = if horizontal {
                        ((x as i64 + o).clamp(0, w as i64 - 1) as usize, y)
                    } else {
                        (x, (y as i64 + o).clamp(0, h as i64 - 1) as usize)
                    };
                    acc = acc + src[sy * w + sx] * (kv / total);
                }
                out[y * w + x] = acc;
            }
        }
        out
    };
    pass(&pass(img, true), false)
}

/// Spread `energy` evenly over the pixels whose centres lie in triangle abc (in pixels), in
/// channel `c`; a triangle that covers no pixel centre puts it all in the pixel under its
/// centroid.
#[allow(clippy::too_many_arguments)]
fn splat_triangle(out: &mut [Color], w: usize, h: usize, c: usize, a: &Vector2<f64>, b: &Vector2<f64>, d: &Vector2<f64>, energy: f64) {
    let cross = |o: &Vector2<f64>, p: &Vector2<f64>, q: &Vector2<f64>| (p.x - o.x) * (q.y - o.y) - (p.y - o.y) * (q.x - o.x);
    let area = cross(a, b, d);
    let centroid = (a + b + d) / 3.;
    let put = |out: &mut [Color], x: i64, y: i64, e: f64| {
        if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
            out[y as usize * w + x as usize].rgb[c] += e;
        }
    };
    if area.abs() < 1. {
        put(out, centroid.x.floor() as i64, centroid.y.floor() as i64, energy);
        return;
    }
    let (x0, x1) = (a.x.min(b.x).min(d.x).floor().max(0.) as i64, a.x.max(b.x).max(d.x).ceil().min(w as f64) as i64);
    let (y0, y1) = (a.y.min(b.y).min(d.y).floor().max(0.) as i64, a.y.max(b.y).max(d.y).ceil().min(h as f64) as i64);
    let inside = |x: i64, y: i64| {
        let p = Vector2::new(x as f64 + 0.5, y as f64 + 0.5);
        let (e0, e1, e2) = (cross(a, b, &p), cross(b, d, &p), cross(d, a, &p));
        (e0 >= 0. && e1 >= 0. && e2 >= 0.) || (e0 <= 0. && e1 <= 0. && e2 <= 0.)
    };
    // The triangle's whole area in pixels, so light landing off the image is lost, not
    // crowded onto the part inside it.
    let covered = area.abs() / 2.;
    let mut any = false;
    for y in y0..y1 {
        for x in x0..x1 {
            if inside(x, y) {
                put(out, x, y, energy / covered);
                any = true;
            }
        }
    }
    if !any {
        put(out, centroid.x.floor() as i64, centroid.y.floor() as i64, energy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flare(blades: Option<usize>) -> Flare {
        let mut lens = LensSystem::named("dgauss.50mm").unwrap();
        lens = lens.clone().with_film_at(lens.focus(1e4).unwrap());
        if let Some(n) = blades {
            lens = lens.with_blades(n, 0.);
        }
        Flare { lens, film: (0.036, 0.024), threshold: 10., sources: 4, ghosts: 1., starburst: 1., grid: 48,
                coating: Coating::Single, imperfection: 0., seed: 1 }
    }

    #[test]
    fn test_reflectance() {
        // Uncoated glass, at normal incidence: ((n - 1) / (n + 1))², 4% for n = 1.5.
        assert!((reflectance(1., 1., 1.5, 550., &[]) - 0.04).abs() < 1e-9);
        // A quarter-wave coating of index 1.38 on n = 1.5 at its design wavelength:
        // ((n0 n2 - n1²) / (n0 n2 + n1²))² = 1.3%.
        let single = Coating::Single.layers(1.5);
        let r = reflectance(1., 1., 1.5, 550., &single);
        let expected = ((1.5 - 1.38f64.powi(2)) / (1.5 + 1.38f64.powi(2))).powi(2);
        assert!((r - expected).abs() < 1e-9, "{} vs {}", r, expected);
        // More away from it, at both ends of the spectrum.
        assert!(reflectance(1., 1., 1.5, 420., &single) > r);
        assert!(reflectance(1., 1., 1.5, 700., &single) > r);
        // Total internal reflection.
        assert_eq!(reflectance(0.1, 1.5, 1., 550., &[]), 1.);
        // Fresnel at an angle, uncoated: Rs and Rp at 45° into n = 1.5.
        let (c, t) = (45f64.to_radians().cos(), (45f64.to_radians().sin() / 1.5).asin().cos());
        let rs = ((c - 1.5 * t) / (c + 1.5 * t)).powi(2);
        let rp = ((1.5 * c - t) / (1.5 * c + t)).powi(2);
        assert!((reflectance(c, 1., 1.5, 550., &[]) - 0.5 * (rs + rp)).abs() < 1e-9);
        // The same from the glass's side, as light leaving it (reversibility), for a stack too.
        let stack = Coating::Multi.layers(1.6);
        let reversed: Vec<_> = stack.iter().rev().copied().collect();
        assert!((reflectance(1., 1., 1.6, 500., &stack) - reflectance(1., 1.6, 1., 500., &reversed)).abs() < 1e-9);
    }

    #[test]
    fn test_multicoating_reflects_less_across_the_visible() {
        // A modern coating: averaging a few tenths of a percent from 430 to 670 nm on crown and
        // flint glass, rising towards the ends of that (to 1.1% at 670 nm on glass of 1.9), and
        // well under a single coating's on crown glass.
        for glass in [1.5, 1.62, 1.75, 1.9] {
            let layers = Coating::Multi.layers(glass);
            let rs: Vec<f64> = (430..=670).step_by(10).map(|l| reflectance(1., 1., glass, l as f64, &layers)).collect();
            let mean = rs.iter().sum::<f64>() / rs.len() as f64;
            assert!(mean < 0.003, "glass {}: mean {}", glass, mean);
            assert!(rs.iter().all(|r| *r < 0.015), "glass {}: {:?}", glass, rs);
        }
        let single = reflectance(1., 1., 1.52, 550., &Coating::Single.layers(1.52));
        let multi = reflectance(1., 1., 1.52, 550., &Coating::Multi.layers(1.52));
        assert!(multi < single / 4., "{} vs {}", multi, single);
    }

    #[test]
    fn test_dispersion_estimate() {
        for nd in [1.5, 1.67, 1.8] {
            assert!((dispersed(nd, 587.6) - nd).abs() < 1e-3);
            assert!(dispersed(nd, 450.) > dispersed(nd, 650.));
        }
        assert_eq!(dispersed(1., 450.), 1.);
    }

    #[test]
    fn test_direct_path_images_the_light() {
        // Straight through, a distant light 5° off axis lands at about f tan 5° on the opposite
        // side, which is where the image shows it.
        let f = flare(None);
        let z = f.positions();
        let a = 5f64.to_radians();
        let dir = Vector3::new(a.sin(), 0., a.cos());
        let (lat, _, _) = f.lattice(&z, &dir, None, 550.).unwrap();
        let hits: Vec<Vector2<f64>> = lat.iter().flatten().map(|(p, _)| *p).collect();
        assert!(hits.len() > 100);
        let mean = hits.iter().sum::<Vector2<f64>>() / hits.len() as f64;
        let focal = f.lens.focal_length().unwrap();
        assert!((mean.x + focal * a.tan()).abs() < 0.03 * focal * a.tan(), "{} vs {}", mean.x, -focal * a.tan());
        let px = f.to_pixel(&mean, 300, 200);
        let back = f.direction_of(px.x, px.y, 300, 200);
        assert!((back - dir).norm() < 0.01, "{} vs {}", back, dir);
    }

    /// Ghosts are faint, and their light is a small part of the light's, on the line through
    /// the image's centre.
    #[test]
    fn test_ghosts_are_faint_and_on_the_axis_line() {
        let f = flare(None);
        let (w, h) = (300, 200);
        let source = Source { direction: Vector3::new(0.15, 0., 1.).normalize(), energy: Color::new(1e6, 1e6, 1e6) };
        let g = f.ghosts_of(&source, w, h);
        let total: f64 = g.iter().map(|c| c.rgb.y).sum();
        assert!(total > 0. && total < 0.01 * 1e6, "ghost light {}", total);
        // A light on the horizontal through the centre makes ghosts centred on that line: their
        // light is symmetric about it.
        let mean_row: f64 = g.iter().enumerate().map(|(i, c)| ((i / w) as f64 + 0.5) * c.rgb.y).sum::<f64>() / total;
        assert!((mean_row - 100.).abs() < 1., "ghosts centred on row {}", mean_row);
    }

    #[test]
    fn test_sources_are_found_and_ordered() {
        let (w, h) = (60, 40);
        let mut pixels = vec![Color::new(0.1, 0.1, 0.1); w * h];
        for (x, y, v) in [(10, 10, 1e3), (11, 10, 1e3), (40, 30, 5e4)] {
            pixels[y * w + x] = Color::new(v, v, v);
        }
        let img = LinearImage { width: w, height: h, pixels };
        let s = flare(None).find_sources(&img, 1.);
        assert_eq!(s.len(), 2);
        assert!((s[0].energy.rgb.x - 5e4).abs() < 1e-9 && (s[1].energy.rgb.x - 2e3).abs() < 1e-9);
        // Below and right of the centre.
        assert!(s[0].direction.x > 0. && s[0].direction.y < 0.);
    }

    #[test]
    fn test_starburst_has_spikes_for_blades() {
        let (w, h) = (64, 64);
        let mut pixels = vec![Color::black(); w * h];
        pixels[32 * w + 32] = Color::new(1e8, 1e8, 1e8);
        let img = LinearImage { width: w, height: h, pixels };
        // A round stop: its edge's halo, the same all round, with no spikes.
        let round = flare(None).starburst_of(&img, 1., 1e-3);
        let at = |v: &[Color], a: f64, r: f64| v[((32. - r * a.sin()).round() as usize) * w + (32. + r * a.cos()).round() as usize].rgb.y;
        assert!(at(&round, PI / 2., 10.) > 0.);
        assert!((at(&round, PI / 2., 10.) / at(&round, 0., 10.) - 1.).abs() < 0.05);
        // Six blades, a corner at 0°: edge normals at 30°, 90°, ...: a spike straight up.
        let six = flare(Some(6)).starburst_of(&img, 1., 1e-3);
        let up = six[(32 - 10) * w + 32].rgb.y;
        let between = six[(32 - 7) * w + 32 + 7].rgb.y;
        assert!(up > 100. * between.max(1e-12), "up {} between {}", up, between);
        // Falls as 1/θ² along it.
        let far = six[(32 - 20) * w + 32].rgb.y;
        assert!((up / far - 4.).abs() < 0.5, "{} {}", up, far);
        // Longer for red than for blue.
        assert!(six[(32 - 10) * w + 32].rgb.x > six[(32 - 10) * w + 32].rgb.z);
    }

    #[test]
    fn test_curved_blades_fan_their_spikes() {
        let (w, h) = (256, 256);
        let mut pixels = vec![Color::black(); w * h];
        pixels[128 * w + 128] = Color::new(1e8, 1e8, 1e8);
        let img = LinearImage { width: w, height: h, pixels };
        // Six blades cut to round an f/2 stop, at f/11: fans of 2 r sin(30°) / R = 0.18 rad.
        let straight = flare(Some(6));
        let open_stop = straight.lens.stop_diameter().unwrap() / 2.;
        let stopped = |curvature: f64| {
            let shape = crate::lens::BladeShape { radius: if curvature > 0. { open_stop / curvature } else { 0. }, jitter: vec![] };
            let lens = straight.lens.clone().with_blade_shape(shape).with_stop(2. * open_stop * 2. / 11.);
            Flare { lens, ..straight.clone() }
        };
        let (flat, curved) = (stopped(0.).starburst_of(&img, 1., 1e-3), stopped(1.).starburst_of(&img, 1., 1e-3));
        let total = |v: &[Color]| v.iter().map(|c| c.rgb.y).sum::<f64>();
        // The same light, only spread.
        let (tf, tc) = (total(&flat), total(&curved));
        assert!((tc / tf - 1.).abs() < 0.01, "{} vs {}", tc, tf);
        // Near the centre the fan is under a pixel wide, so the spikes agree; far out it is many
        // pixels, so a curved blade's spike is fainter along its axis, falling as 1/θ³.
        let (near, far) = ((128 - 4) * w + 128, (128 - 100) * w + 128);
        assert!((curved[near].rgb.y / flat[near].rgb.y - 1.).abs() < 0.2, "{} {}", curved[near].rgb.y, flat[near].rgb.y);
        let ratio = curved[far].rgb.y / flat[far].rgb.y;
        assert!(ratio < 0.1, "far: curved {} straight {}", curved[far].rgb.y, flat[far].rgb.y);
        let fall = curved[(128 - 50) * w + 128].rgb.y / curved[far].rgb.y;
        assert!((fall - 8.).abs() < 2., "1/θ³ from 50 to 100 pixels: {}", fall);
    }

    #[test]
    fn test_an_imperfect_lens_makes_uneven_spikes() {
        let (w, h) = (256, 256);
        let mut pixels = vec![Color::black(); w * h];
        pixels[128 * w + 128] = Color::new(1e8, 1e8, 1e8);
        let img = LinearImage { width: w, height: h, pixels };
        let ideal = flare(Some(6));
        let rough = |seed: u64| {
            let lens = ideal.lens.clone().with_blade_shape(crate::lens::BladeShape { radius: 0., jitter: imperfect_blades(6, 1., seed) });
            Flare { lens, imperfection: 1., seed, ..ideal.clone() }
        };
        let total = |v: &[Color]| v.iter().map(|c| c.rgb.y).sum::<f64>();
        let perfect = ideal.starburst_of(&img, 1., 1e-3);
        let (a, b) = (rough(1).starburst_of(&img, 1., 1e-3), rough(2).starburst_of(&img, 1., 1e-3));
        // About the same light (the blades' edges are about as long), spread differently.
        assert!((total(&a) / total(&perfect) - 1.).abs() < 0.05, "{} vs {}", total(&a), total(&perfect));
        // The six spikes, at 80 pixels: equal for the perfect lens, not for a real one; and
        // not the same from lens to lens.
        let spikes = |v: &[Color]| -> Vec<f64> { (0..6).map(|k| {
            let a = (30. + 60. * k as f64).to_radians();
            // The light in a sector around each spike, 75 to 85 pixels out, as an imperfect
            // one wanders a little.
            let mut sum = 0.;
            for y in 0..h {
                for x in 0..w {
                    let (dx, dy) = (x as f64 - 128., 128. - y as f64);
                    let (r, phi) = ((dx * dx + dy * dy).sqrt(), dy.atan2(dx));
                    let off = (phi - a + PI).rem_euclid(2. * PI) - PI;
                    if (75. ..85.).contains(&r) && off.abs() < 0.1 {
                        sum += v[y * w + x].rgb.y;
                    }
                }
            }
            sum
        }).collect() };
        let spread = |v: &[f64]| { let m = v.iter().sum::<f64>() / v.len() as f64; (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len() as f64).sqrt() / m };
        let (p, sa, sb) = (spikes(&perfect), spikes(&a), spikes(&b));
        assert!(spread(&p) < 0.03, "{:?}", p);
        // Each spike against the perfect lens's (which takes out the pixels' part): uneven by
        // a few percent, from blades placed to about 1% and 0.4°. Opposite spikes stay equal,
        // as every aperture's diffraction is symmetric through the centre.
        let relative: Vec<f64> = sa.iter().zip(&p).map(|(x, y)| x / y).collect();
        assert!(spread(&relative) > 0.01, "{:?}", relative);
        assert!((relative[0] / relative[3] - 1.).abs() < 1e-6, "{:?}", relative);
        assert_ne!(sa, sb);
        // Streaky: along an arc across a spike's fan, a rough edge's light is uneven where a
        // smooth curved edge's is even.
        let curved = |imperfection: f64| {
            let open_stop = ideal.lens.stop_diameter().unwrap() / 2.;
            let shape = crate::lens::BladeShape { radius: open_stop, jitter: vec![] };
            let lens = ideal.lens.clone().with_blade_shape(shape).with_stop(2. * open_stop * 2. / 11.);
            Flare { lens, imperfection, seed: 3, ..ideal.clone() }.starburst_of(&img, 1., 1e-3)
        };
        let across = |v: &[Color]| -> Vec<f64> { (-20..=20).map(|d| {
            let b = 90f64.to_radians() + d as f64 * 0.002;
            let (x, y) = ((128. + 100. * b.cos()).round() as usize, (128. - 100. * b.sin()).round() as usize);
            v[y * w + x].rgb.y
        }).collect() };
        let (smooth, streaky) = (across(&curved(0.)), across(&curved(1.)));
        assert!(spread(&smooth) < 0.05, "{:?}", smooth);
        assert!(spread(&streaky) > 0.1, "{:?}", streaky);
    }

    #[test]
    fn test_airy() {
        assert!((airy(0.) - 1.) .abs() < 1e-7);
        // The first dark ring at x = 3.8317, and the two approximations meeting at 3.
        assert!(airy(3.8317) < 1e-8);
        assert!((airy(3. - 1e-9) - airy(3. + 1e-9)).abs() < 1e-6);
        // Encircled energy: the pattern integrates to 4π over the plane, ∫ airy(x) x dx = 2.
        let (mut sum, dx) = (0., 1e-3);
        let mut x = dx / 2.;
        while x < 2000. {
            sum += airy(x) * x * dx;
            x += dx;
        }
        assert!((sum - 2.).abs() < 0.01, "{}", sum);
    }

    #[test]
    fn test_dirt_scatters_what_it_blocks() {
        // Babinet: each speck and scratch sends out the light it blocks, its share of the pupil.
        let f = Flare { imperfection: 1., seed: 5, ..flare(None) };
        let area = 1e-5;
        let (scratches, specks) = f.dirt(area);
        assert_eq!(scratches.len(), 3);
        let dust: f64 = specks.iter().map(|d| PI * d * d / 4.).sum();
        // At least the target, and over it by less than one speck (80 µm at most).
        assert!(dust >= 2e-4 * area && dust < 2e-4 * area + PI * 80e-6f64.powi(2) / 4., "{}", dust / area);
        let (pw, ph, reach, p) = (1024, 1024, 512, 2e-3);
        for (sc, sp, blocked) in [(scratches.clone(), vec![], scratches.iter().map(|s| s.1 * s.2).sum::<f64>()), (vec![], specks.clone(), dust)] {
            let mut k = vec![0.; pw * ph];
            f.add_dirt(&mut k, pw, ph, reach, p, 550e-9, area, &sc, &sp);
            let total: f64 = k.iter().sum();
            // What reaches past the kernel's edge, half a radian out, is lost.
            assert!(total < blocked / area && total > 0.9 * blocked / area, "{} vs {}", total, blocked / area);
        }
    }
}
