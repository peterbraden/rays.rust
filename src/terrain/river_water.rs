//! # What a river's water looks like: its colour, its ripples and its foam
//!
//! Step 3 of the water workstream (plan.md section 16). A river's surface (`rivers::WaterSurface`)
//! is shaded by `RiverWater`, which looks up the flow beneath each point (`rivers::Carve::water_at`)
//! and sets the sea's physical water (`ocean::PhysicalOcean`) to it: the body's colour and
//! clarity from the sediment it carries, the ripples' roughness from its turbulence, and
//! whitewater where it breaks.
//!
//! ## Colour and clarity from suspended sediment
//!
//! A river carries the sediment its basin sheds. The simulation knows the flux (m³/yr of
//! deposit, `Terrain::sediment_flux`) and the hydraulics the discharge, so the mean
//! concentration is C = flux ρ_s (1 - φ) / Q, in g/m³ (mg/L), taking the deposit's porosity φ
//! as 0.35 and its grains as quartz (2650 kg/m³), and all of the flux as carried in suspension
//! (much of a mountain river's moves along its bed: an overestimate, to refine). Mountain
//! rivers come out at hundreds to thousands of mg/L, as glacial and flood waters are.
//!
//! The water's optics are those of ocean colour remote sensing. Clear river water absorbs as
//! the plain river did before (a, per channel: pure water and a little dissolved organic
//! matter); mineral particles add absorption a_p = C a*_p(λ), scattering b_p = C b*_p and
//! backscattering b_bp = C b*_bp. Then the light the water body sends back, as an irradiance
//! reflectance just under the surface, is R ≈ 0.33 b_b / (a + b_b) (Gordon et al. 1975; Morel
//! and Prieur 1977), b_b including water's own (b_bw, after Morel 1974). So clear water is dark
//! and blue, and silty water bright and brown-grey: the particles backscatter at all
//! wavelengths while absorbing mostly the blue. Seeing the bed through it, the light is lost
//! to absorption and scattering both, c = a + b, which is what `PhysicalOcean::absorption`
//! weighs the bed and the body by.
//!
//! The mass-specific coefficients: b*_bp = 0.012 m²/g for mineral particles (Neukermans et al.
//! 2012); b*_p = 0.5 m²/g (about what Babin et al. 2003 found for coastal particles; to check);
//! a*_p = 0.04 exp(-0.012 (λ - 440 nm)) m²/g (within the range Babin and Stramski 2004 measured
//! for mineral particles, which vary with their iron; to check). RGB is taken at 600, 550 and
//! 450 nm.
//!
//! ## Roughness from the flow
//!
//! A river's surface is roughened by the waves its flow raises over the bed's irregularities,
//! which steepen as the flow nears critical: a slow glide (Froude number 0.2) mirrors the
//! banks, a riffle (0.8) sparkles. Mean square slope 0.0005 + 0.02 Fr² (at most 0.25; a model
//! choice, to measure against eg. Horoshenkov et al.'s surface roughness of shallow turbulent
//! flows); wind's ripples are left out. (Taking it from the slope instead, as at first, made
//! even gentle rivers too rough to reflect anything.) Along a river the Froude number rises
//! and falls through pools and riffles every six widths or so (`riffles`), so its surface
//! alternates between glassy and broken.
//!
//! ## Whitewater
//!
//! Water breaks white where its surface can't stay whole: where the flow is rapid
//! (supercritical, Froude number over 1, so standing waves steepen and break) and where the
//! bed's stones reach up through it (relative submergence d / D, the depth over the grains'
//! size, near or below 1: the step-pools and cascades of mountain streams, where water pours
//! over and round boulders). The cover is
//!
//! ```text
//! W = min(0.9, smoothstep(0.7, 2, Fr) + (1 - smoothstep(0.5, 3, d / D)) smoothstep(0.2, 0.6, Fr))
//! ```
//!
//! (a model choice, after the onset of aeration in Chanson's measurements of rough chutes, to
//! fit), laid out as streaks along the flow, and shaded as the sea's foam is
//! (`ocean::Foam`'s albedo): a diffuse froth. Between the streaks the bubbles pale the water.
//!
//! ## References
//!
//! - H. R. Gordon, O. B. Brown, M. M. Jacobs, "Computed relationships between the inherent and
//!   apparent optical properties of a flat homogeneous ocean", Applied Optics 14(2), 1975.
//! - A. Morel, L. Prieur, "Analysis of variations in ocean color", Limnology and Oceanography
//!   22(4), 1977.
//! - G. Neukermans, H. Loisel, X. Mériaux, R. Astoreca, D. McKee, "In situ variability of
//!   mass-specific beam attenuation and backscattering of marine particles with respect to
//!   particle size, density, and composition", Limnology and Oceanography 57(1), 2012.
//! - M. Babin, A. Morel, V. Fournier-Sicre, F. Fell, D. Stramski, "Light scattering properties
//!   of marine particles in coastal and open ocean waters as related to the particle mass
//!   concentration", Limnology and Oceanography 48(2), 2003; M. Babin, D. Stramski,
//!   "Variations in the mass-specific absorption coefficient of mineral particles suspended in
//!   water", Limnology and Oceanography 49(3), 2004.
//! - H. Chanson, "Air Bubble Entrainment in Free-Surface Turbulent Shear Flows", Academic
//!   Press, 1997 (unverified).
//!
//! (Bibliographic details from search results and memory; to verify.)

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ocean::PhysicalOcean;
use crate::procedural::nubis_clouds::smoothstep;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::terrain::rivers::Node;
use crate::terrain::scenery::{Frame, Relief};
use std::sync::Arc;

/// Wavelengths of the red, green and blue channels, nm.
const WAVELENGTHS: [f64; 3] = [600., 550., 450.];
/// Clear river water's absorption, m⁻¹: pure water's and a little dissolved organic matter's.
pub const CLEAR: [f64; 3] = [0.45, 0.09, 0.07];
/// Mass-specific coefficients of mineral particles, m²/g.
const BACKSCATTER: f64 = 0.012;
const SCATTER: f64 = 0.5;
/// Pure water's backscattering at 500 nm, m⁻¹, falling as λ^-4.32 (Morel 1974).
const WATER_BACKSCATTER: f64 = 0.0014;
/// Foam's albedo (as the sea's whitecaps).
const FOAM: f64 = 0.6;

/// Grains' density, kg/m³, and the deposit's porosity.
const QUARTZ: f64 = 2650.;
const POROSITY: f64 = 0.35;

/// Mean suspended sediment, g/m³, for a sediment flux `flux` (m³/yr of deposit) carried by a
/// mean discharge `q` (m³/s).
pub fn concentration(flux: f64, q: f64) -> f64 {
    if q <= 0. { return 0.; }
    let grams = flux.max(0.) * QUARTZ * (1. - POROSITY) * 1000.;
    (grams / (q * 365.25 * 86400.)).min(1e5)
}

/// Mineral particles' absorption per g/m³ at wavelength `l` nm.
fn particle_absorption(l: f64) -> f64 { 0.04 * (-0.012 * (l - 440.)).exp() }

/// The water's optics for suspended sediment `c` g/m³: the colour it sends back (an
/// irradiance reflectance) and its attenuation per metre, per channel.
pub fn optics(c: f64) -> (Color, Color) {
    let c = c.max(0.);
    let channel = |k: usize| {
        let l = WAVELENGTHS[k];
        let a = CLEAR[k] + c * particle_absorption(l);
        let bb = WATER_BACKSCATTER * (l / 500.).powf(-4.32) + c * BACKSCATTER;
        (0.33 * bb / (a + bb), a + bb + c * SCATTER)
    };
    let [r, g, b] = [channel(0), channel(1), channel(2)];
    (Color::new(r.0, g.0, b.0), Color::new(r.1, g.1, b.1))
}

/// The ripples' mean square slope for a reach of slope `s`.
pub fn roughness(froude: f64) -> f64 { (0.0005 + 0.02 * froude * froude).min(0.25) }

/// Pools and riffles alternate along a river every five to seven channel widths (Leopold,
/// Wolman and Miller 1964 give about 5-7; Keller and Melhorn 1978 likewise; unverified): the
/// local Froude number over the reach's, deeper and slower in the pools, shallower and faster
/// over the riffles, at a point `along` metres down a river `width` wide (`wander`, a noise
/// in -1 to 1, shifts the rhythm so it isn't regular).
pub fn riffles(along: f64, width: f64, wander: f64) -> f64 {
    1. + 0.6 * (std::f64::consts::TAU * along / (6. * width.max(1.)) + 2.5 * wander).sin()
}

/// The fraction of the surface white with foam, for Froude number `froude` and relative
/// submergence `submergence` (depth over grain size).
pub fn whitewater(froude: f64, submergence: f64) -> f64 {
    (smoothstep(0.7, 2., froude) + (1. - smoothstep(0.5, 3., submergence)) * smoothstep(0.2, 0.6, froude)).min(0.9)
}

/// The rivers' water, shaded by the flow under each point.
pub struct RiverWater {
    pub relief: Arc<Relief>,
    pub frame: Frame,
    pub noise: crate::noise::PerlinNoise,
    /// The lakes' waves.
    pub waves: LakeWaves,
    /// The rivers' standing waves.
    pub currents: RiverWaves,
}

/// The wavelength (m) of the waves that stand still on a current `speed` m/s and `depth` m
/// deep: those travelling upstream as fast as the water carries them down, ω(k) = k U, so
/// tanh(k d) = k U² / g by the dispersion of gravity waves over depth d. Behind each stone
/// and riffle they are what the surface of a shallow, quick river is patterned by, their
/// crests across the flow. Below critical flow (U² < g d) there is one such k; at and above,
/// none in shallow water, and the deep-water one, g / U², is taken.
pub fn stationary_wavelength(speed: f64, depth: f64) -> f64 {
    let (g, u2, d) = (9.81, speed.max(0.05).powi(2), depth.max(0.01));
    let deep = g / u2;
    let k = if u2 >= g * d { deep } else {
        // tanh(kd) - k U²/g falls through zero once, between deep (where tanh < 1 = k U²/g)
        // and 0.
        let (mut lo, mut hi) = (1e-9, deep);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if (mid * d).tanh() > mid * u2 / g { lo = mid } else { hi = mid }
        }
        0.5 * (lo + hi)
    };
    std::f64::consts::TAU / k
}

/// The rivers' surfaces: Tessendorf's waves, as the sea's (`ocean::WaveField`), peaking at the
/// stationary wavelength of each stretch's flow (`stationary_wavelength`), crests across the
/// flow, laid out along and across the river so they follow its bends; their slopes as rough
/// as the flow's Froude number makes them (`roughness`, a model choice, the share of it
/// above the ripples the material keeps). A field for each half power of two of wavelength,
/// made when first wanted.
pub struct RiverWaves {
    fields: Vec<std::sync::OnceLock<crate::ocean::WaveField>>,
    seed: u64,
}

/// The shortest river wavelength given a field of its own (2^(LOWEST/2) m) and how many.
const LOWEST: i32 = -6;
const BINS: usize = 16;
/// The ripples the material keeps (`roughness` at rest): the rest of the river's roughness is
/// in its waves.
const RIPPLES: f64 = 0.0005;

impl RiverWaves {
    pub fn new(seed: u64) -> RiverWaves {
        RiverWaves { fields: (0..BINS).map(|_| std::sync::OnceLock::new()).collect(), seed }
    }

    /// The slopes (along the flow and across it) of the waves on the river `node` at `station`
    /// (metres along and across it).
    pub fn slopes(&self, node: &Node, station: [f64; 2]) -> (f64, f64) {
        let speed = node.froude * (9.81 * node.depth.max(0.)).sqrt();
        if speed <= 0.01 { return (0., 0.); }
        let lambda = stationary_wavelength(speed, node.depth);
        let bin = ((2. * lambda.log2()).round() as i32 - LOWEST).clamp(0, BINS as i32 - 1);
        let field = self.fields[bin as usize].get_or_init(|| {
            let peak = 2f64.powf((bin + LOWEST) as f64 / 2.);
            let scale = (25. * peak / 487.).min(1.);
            crate::ocean::WaveField::sea(nalgebra::Vector2::new(speed, 0.), 0., self.seed.wrapping_add(bin as u64),
                crate::ocean::SeaState { peak: Some(peak), scale, choppiness: 0., resolved: Some(1.), ..Default::default() })
        });
        let wanted = (roughness(node.froude) - RIPPLES).max(0.);
        if field.mean_square_slope <= 0. { return (0., 0.); }
        let k = (wanted / field.mean_square_slope).sqrt();
        let w = field.at(station[0], station[1]);
        (k * w.y, k * w.z)
    }
}

/// The waves the wind raises on the lakes (`ocean::WaveField::lake`): Tessendorf's, as on the
/// sea, but a young sea's, short of fetch. Each lake takes the field for its length along the
/// wind, rounded to a half power of two (so lakes of about the same size share one). The waves
/// are centimetres high, so they only tilt the surface's normals; the water is still drawn flat.
pub struct LakeWaves {
    fields: Vec<crate::ocean::WaveField>,
    /// For each layer of the relief, each lake's field.
    lakes: Vec<Vec<usize>>,
    /// The wind, and the fields for rivers' fetches (`RIVER_FETCH`), each half power of two of
    /// fetch from `RIVER_BINS.0`, made when first wanted.
    wind: nalgebra::Vector2<f64>,
    rivers: Vec<std::sync::OnceLock<crate::ocean::WaveField>>,
    seed: u64,
}

/// The wind's fetch along a river, in its widths: the straight stretch it blows along before
/// the river bends away, about half a meander's wavelength (11 widths; a model choice).
const RIVER_FETCH: f64 = 5.;
/// The rivers' fetches given fields: from 2^(RIVER_BINS.0 / 2) m (8 m), so many.
const RIVER_BINS: (i32, usize) = (6, 16);

impl LakeWaves {
    pub fn new(relief: &Relief, frame: &Frame, wind: [f64; 2], seed: u64) -> LakeWaves {
        let w = nalgebra::Vector2::new(wind[0], wind[1]);
        let rivers = (0..RIVER_BINS.1).map(|_| std::sync::OnceLock::new()).collect();
        if w.norm() < 0.1 {
            return LakeWaves { fields: vec![], lakes: vec![], wind: w, rivers, seed };
        }
        let bin = |fetch: f64| (2. * fetch.max(1.).log2()).round() as i32;
        let mut bins: Vec<i32> = vec![];
        let lengths: Vec<Vec<i32>> = relief.layers.iter().map(|l| {
            let Some(b) = l.rivers.as_ref().and_then(|r| r.basins.get()) else { return vec![] };
            // The wind in the patch's coordinates, from the scene's at the patch's centre.
            let o = crate::terrain::rivers::patch_dir(&l.domain, [0., 0.]);
            let at = frame.to_scene(&o, 0.);
            let towards = at + Vector3::new(w.x, 0., w.y).normalize() * 100.;
            let (d, _) = frame.on_planet(&towards);
            let dir = crate::terrain::rivers::patch_point(&l.domain, &d).map_or([1., 0.], |q| { let n = q[0].hypot(q[1]).max(1e-9); [q[0] / n, q[1] / n] });
            (0..b.count()).map(|k| { let f = bin(b.length(k as u32, dir)); if !bins.contains(&f) { bins.push(f); } f }).collect()
        }).collect();
        use rayon::prelude::*;
        let fields = bins.par_iter().map(|&f| crate::ocean::WaveField::lake(w, 2f64.powf(f as f64 / 2.), 0., seed.wrapping_add(f as u64))).collect();
        let lakes = lengths.into_iter().map(|l| l.into_iter().map(|f| bins.iter().position(|&b| b == f).unwrap()).collect()).collect();
        LakeWaves { fields, lakes, wind: w, rivers, seed }
    }

    /// The wind's waves' slopes (along scene x and z) at scene point `p` on a river `width`
    /// metres wide, its fetch the river's (`RIVER_FETCH`).
    pub fn river_slopes(&self, p: &Vector3<f64>, width: f64) -> Option<(f64, f64)> {
        if self.wind.norm() < 0.1 { return None; }
        let fetch = (RIVER_FETCH * width).max(10.);
        let bin = ((2. * fetch.log2()).round() as i32 - RIVER_BINS.0).clamp(0, RIVER_BINS.1 as i32 - 1);
        let field = self.rivers[bin as usize].get_or_init(|| {
            let fetch = 2f64.powf((bin + RIVER_BINS.0) as f64 / 2.);
            crate::ocean::WaveField::lake(self.wind, fetch, 0., self.seed.wrapping_add(0x5717 + bin as u64))
        });
        let w = field.at(p.x, p.z);
        Some((w.y, w.z))
    }

    /// The waves' slopes (along scene x and z) at scene point `p` on a lake, if it is on one.
    pub fn slopes(&self, relief: &Relief, frame: &Frame, p: &Vector3<f64>) -> Option<(f64, f64)> {
        if self.fields.is_empty() { return None; }
        let (d, _) = frame.on_planet(p);
        let k = relief.layers.iter().position(|l| l.contains(&d))?;
        let l = &relief.layers[k];
        let q = crate::terrain::rivers::patch_point(&l.domain, &d)?;
        let lake = l.rivers.as_ref()?.basins.get()?.lake_at(q)?;
        let w = self.fields[*self.lakes.get(k)?.get(lake as usize)?].at(p.x, p.z);
        Some((w.y, w.z))
    }
}

impl RiverWater {
    /// The water's attenuation per metre at scene point `p` (for light up through the surface).
    pub fn attenuation(&self, p: &Vector3<f64>) -> Color {
        optics(self.flow(p).map_or(0., |f| f.0.turbidity)).1
    }

    /// The flow at scene point `p`, and its direction there (scene, unit); its Froude number
    /// varying through pools and riffles. With where `p` is on the river: metres along its
    /// course and across it.
    fn flow(&self, p: &Vector3<f64>) -> Option<(Node, Vector3<f64>, [f64; 2])> {
        let (d, _) = self.frame.on_planet(p);
        let (mut node, along, station) = self.relief.river_at(&d)?;
        // The flow's direction, from the patch's tangent plane into the scene.
        let ahead = (self.frame.to_scene(&(d + along / self.frame.radius).normalize(), 0.) - self.frame.to_scene(&d, 0.)).normalize();
        let width = 2. * node.half;
        let wander = 2. * self.noise.noise(p.x / (20. * width), p.y / (20. * width), p.z / (20. * width)) - 1.;
        node.froude *= riffles(station[0], width, wander);
        Some((node, ahead, station))
    }

    /// The sea's water as it is here.
    fn water(&self, node: &Node, foam: f64) -> PhysicalOcean {
        let (body, attenuation) = optics(node.turbidity);
        // Bubbles left by the foam pale the water and turn it green-blue.
        let bubbles = Color::new(0.01, 0.05, 0.06) * foam;
        PhysicalOcean {
            water: body + bubbles, roughness: RIPPLES, center: Some(self.frame.centre()),
            absorption: Some(attenuation), foam: None, surf: None,
        }
    }

    /// Whether `p` is in a streak of foam, where the river is `cover` white: noise stretched
    /// along the flow, thresholded at its quantile for that cover (taken as uniform, roughly).
    fn foamy(&self, p: &Vector3<f64>, along: &Vector3<f64>, cover: f64) -> bool {
        if cover <= 0. { return false; }
        let up = (p - self.frame.centre()).normalize();
        let across = up.cross(along).normalize();
        let (u, v) = (p.dot(along) / 4., p.dot(&across) / 0.6);
        let n = self.noise.fbm(u, v, 0.37, 4, 0.5, 2.);
        n > 1. - smoothstep(0., 1., cover).sqrt().min(1.) * 0.8 - 0.1 * (1. - cover)
    }
}

impl RiverWater {
    /// The hit as the waves tilt it, seen from above: its normal from their slopes, the river's
    /// (`RiverWaves`) where it flows, else the lake's. Unchanged from below, or where a wave
    /// would face away from the ray.
    fn on_waves<'a>(&self, r: &Ray, intersection: &Intersection<'a>, flow: Option<&(Node, Vector3<f64>, [f64; 2])>) -> Intersection<'a> {
        let p = intersection.point;
        let up = (p - self.frame.centre()).normalize();
        if r.rd.dot(&up) >= 0. { return *intersection; }
        // The wind's waves: a lake's, or on a river the fetch along it.
        let tx = (Vector3::x() - up * up.x).normalize();
        let tz = (Vector3::z() - up * up.z).normalize();
        let wind = self.waves.slopes(&self.relief, &self.frame, &p)
            .or_else(|| flow.and_then(|(node, _, _)| self.waves.river_slopes(&p, 2. * node.half)));
        let mut n = wind.map_or(up, |(sx, sz)| up - tx * sx - tz * sz);
        // And on a river, the waves standing on its current.
        if let Some((node, ahead, station)) = flow {
            let side = up.cross(ahead).normalize();
            let ahead = side.cross(&up);
            let (sa, sc) = self.currents.slopes(node, *station);
            n -= ahead * sa + side * sc;
        }
        if wind.is_none() && flow.is_none() { return *intersection; }
        let n = n.normalize();
        if r.rd.dot(&n) >= 0. { return *intersection; }
        Intersection { normal: n, ..*intersection }
    }
}

impl MaterialModel for RiverWater {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let flow = self.flow(&intersection.point);
        let waved = self.on_waves(r, intersection, flow.as_ref());
        let intersection = &waved;
        let Some((node, along, _)) = flow else {
            return self.water(&Node::still(), 0.).scatter(r, intersection, s);
        };
        let cover = whitewater(node.froude, 1.5 * node.depth / node.grain.max(1e-4));
        // From above, foam scatters diffusely about the mean surface.
        if r.rd.dot(&intersection.normal) < 0. && self.foamy(&intersection.point, &along, cover) && rand() < 0.95 {
            let up = (intersection.point - self.frame.centre()).normalize();
            let albedo = Color::new(FOAM, FOAM, FOAM).as_reflectance();
            return crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal: up }, r, &intersection.point, s);
        }
        self.water(&node, cover).scatter(r, intersection, s)
    }

    fn sun_glint(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> Option<Color> {
        let flow = self.flow(&intersection.point);
        let waved = self.on_waves(r, intersection, flow.as_ref());
        let intersection = &waved;
        let node = self.flow(&intersection.point).map_or(Node::still(), |f| f.0);
        self.water(&node, 0.).sun_glint(r, intersection, s)
    }
}

/// How much of a falling jet's curtain is water, `z` metres below its lip, for a jet that
/// breaks up after falling `breakup` metres: whole above that; below, drops spreading apart,
/// thinning to a fifth over a few break-up lengths (a model choice standing in for the spray
/// as a medium, the next step).
pub fn curtain(z: f64, breakup: f64) -> f64 {
    if z <= breakup { 1. } else { (-(z - breakup) / (2. * breakup + 5.)).exp().max(0.2) }
}

/// A waterfall's falling water: aerated, so white, streaked down its fall, and once broken up
/// into drops, partly see-through (`curtain`). A ray that misses the drops passes on.
pub struct FallingWater {
    pub frame: Frame,
    /// The lip's level and the jet's break-up length, metres.
    pub top: f64,
    pub breakup: f64,
    pub noise: crate::noise::PerlinNoise,
}

impl FallingWater {
    fn fallen(&self, p: &Vector3<f64>) -> f64 { self.top - self.frame.on_planet(p).1 }
}

impl MaterialModel for FallingWater {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let z = self.fallen(&intersection.point);
        if rand() > curtain(z, self.breakup) {
            // Between the drops: on, unchanged.
            let ro = intersection.point + r.rd.normalize() * 2. * crate::material::functions::surface_offset(intersection, s);
            return ScatteredRay { emit: Color::black(), attenuate: Color::white(), light_sampling: crate::material::model::LightSampling::None, ray: Some(Ray { ro, rd: r.rd }) };
        }
        // Streaks down the fall: the aerated water's albedo varies across it.
        let p = intersection.point;
        let streak = self.noise.fbm(p.x / 0.4, p.z / 0.4, z / 6., 3, 0.5, 2.);
        let albedo = FOAM * (0.75 + 0.4 * streak).min(1.3);
        let normal = if r.rd.dot(&intersection.normal) < 0. { intersection.normal } else { -intersection.normal };
        let albedo = Color::new(albedo, albedo, albedo).as_reflectance();
        crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal }, r, &p, s)
    }
}

/// A waterfall's curtain as geometry: it lets light through where its water has broken into
/// drops, as much as `curtain` says.
pub struct Curtain {
    pub mesh: crate::shapes::mesh::SmoothMesh,
    pub frame: Frame,
    pub top: f64,
    pub breakup: f64,
}

impl crate::shapes::geometry::Geometry for Curtain {
    fn intersects(&self, r: &Ray) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects(r) }
    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_beyond(r, min) }
    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_range(r, min, max) }
    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool { self.mesh.occludes(r, min, max) }
    fn fast_intersects(&self, r: &Ray) -> bool { self.mesh.fast_intersects(r) }
    fn bounds(&self) -> crate::shapes::bbox::BBox { self.mesh.bounds() }
    fn primitives(&self) -> u64 { self.mesh.primitives() }
    fn kind(&self) -> &'static str { "waterfall" }
    fn transmits(&self) -> bool { true }

    fn transmittance(&self, r: &Ray, min: f64, max: f64) -> f64 {
        crate::color::luminance(&self.transmittance_color(r, min, max))
    }

    fn transmittance_color(&self, r: &Ray, min: f64, max: f64) -> Color {
        match self.mesh.intersects_range(r, min, max) {
            Some(hit) => Color::white() * (1. - curtain(self.top - self.frame.on_planet(&hit.point).1, self.breakup)),
            None => Color::white(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silt_brightens_and_browns_the_water_and_clouds_it() {
        let (clear, clear_c) = optics(0.);
        let (silty, silty_c) = optics(200.);
        // Clear water sends back little, most of it blue; silty water much more, most of it
        // red and green.
        assert!(clear.rgb.max() < 0.02 && clear.rgb.z > clear.rgb.x, "{:?}", clear);
        assert!(silty.rgb.min() > 0.05 && silty.rgb.x > silty.rgb.z, "{:?}", silty);
        // The bed shows through a metre of clear water, not through ten centimetres of silty.
        assert!((-clear_c.rgb.y).exp() > 0.8);
        assert!((-0.1 * silty_c.rgb.y).exp() < 0.01);
        // Reflectance saturates towards 0.33 b_b / b_b's limit as the water thickens with silt.
        let (thick, _) = optics(5000.);
        assert!(thick.rgb.max() < 0.33 && thick.rgb.x > silty.rgb.x);
    }

    #[test]
    fn test_a_curtain_is_whole_until_it_breaks_up() {
        assert_eq!(curtain(3., 8.), 1.);
        assert!(curtain(20., 8.) < 0.7 && curtain(20., 8.) > curtain(40., 8.));
        assert_eq!(curtain(1000., 8.), 0.2);
    }

    #[test]
    fn test_waves_stand_on_the_current() {
        // In deep water, waves stand still on a current U when their phase speed √(g/k) is U:
        // λ = 2π U² / g, 64 cm for 1 m/s.
        assert!((stationary_wavelength(1., 100.) - std::f64::consts::TAU / 9.81).abs() < 1e-6);
        // Over 29 cm, the same current: a little longer (kd is 2.8, so the bed barely slows
        // them), satisfying the dispersion relation, tanh(kd) = k U² / g.
        let l = stationary_wavelength(1., 0.29);
        let k = std::f64::consts::TAU / l;
        assert!(l > std::f64::consts::TAU / 9.81 && ((k * 0.29).tanh() - k / 9.81).abs() < 1e-6, "{}", l);
        // Nearer critical flow they lengthen without bound.
        assert!(stationary_wavelength(1.6, 0.29) > 3. * l);
    }

    #[test]
    fn test_a_river_is_as_rough_as_its_froude_number_makes_it() {
        let waves = RiverWaves::new(1);
        let mut node = Node::still();
        node.depth = 0.29;
        node.froude = 0.57;
        let n = 4000;
        let mss = (0..n).map(|i| {
            let (a, c) = waves.slopes(&node, [0.37 * i as f64, 0.11 * (i % 50) as f64]);
            a * a + c * c
        }).sum::<f64>() / n as f64;
        let wanted = roughness(0.57) - RIPPLES;
        assert!((mss / wanted - 1.).abs() < 0.25, "{} vs {}", mss, wanted);
        // The crests lie across the flow: the slopes are steeper along it than across.
        let (along, across) = (0..n).map(|i| waves.slopes(&node, [0.37 * i as f64, 0.11 * (i % 50) as f64]))
            .fold((0., 0.), |(x, y), (a, c)| (x + a * a, y + c * c));
        assert!(along > 2. * across, "{} {}", along, across);
        // Still water: none.
        assert_eq!(waves.slopes(&Node::still(), [3., 1.]), (0., 0.));
    }

    #[test]
    fn test_whitewater_where_rapid_or_over_boulders() {
        // A deep lowland river: none. Rapid water: much. A shallow tumble over boulders, even
        // below critical: some.
        assert_eq!(whitewater(0.2, 50.), 0.);
        assert!(whitewater(2.5, 50.) > 0.8);
        let boulders = whitewater(0.7, 0.8);
        assert!(boulders > 0.5 && boulders <= 0.9, "{}", boulders);
        // A slow glide is nearly a mirror; a riffle near critical is rough.
        assert!(roughness(0.2) < 0.002 && roughness(0.9) > 0.015 && roughness(10.) == 0.25);
        let r: Vec<f64> = (0..60).map(|k| riffles(k as f64, 10., 0.)).collect();
        assert!(r.iter().all(|&x| x > 0.3 && x < 1.7));
    }
}
