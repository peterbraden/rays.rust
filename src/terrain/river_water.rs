//! # What a river's water looks like: its colour, its ripples and its foam
//!
//! Step 3 of the water workstream (plan.md section 15). A river's surface (`rivers::WaterSurface`)
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
}

impl RiverWater {
    /// The water's attenuation per metre at scene point `p` (for light up through the surface).
    pub fn attenuation(&self, p: &Vector3<f64>) -> Color {
        optics(self.flow(p).map_or(0., |f| f.0.turbidity)).1
    }

    /// The flow at scene point `p`, and its direction there (scene, unit); its Froude number
    /// varying through pools and riffles.
    fn flow(&self, p: &Vector3<f64>) -> Option<(Node, Vector3<f64>)> {
        let (d, _) = self.frame.on_planet(p);
        let (mut node, along) = self.relief.river_at(&d)?;
        // The flow's direction, from the patch's tangent plane into the scene.
        let ahead = (self.frame.to_scene(&(d + along / self.frame.radius).normalize(), 0.) - self.frame.to_scene(&d, 0.)).normalize();
        let width = 2. * node.half;
        let wander = 2. * self.noise.noise(p.x / (20. * width), p.y / (20. * width), p.z / (20. * width)) - 1.;
        node.froude *= riffles(p.dot(&ahead), width, wander);
        Some((node, ahead))
    }

    /// The sea's water as it is here.
    fn water(&self, node: &Node, foam: f64) -> PhysicalOcean {
        let (body, attenuation) = optics(node.turbidity);
        // Bubbles left by the foam pale the water and turn it green-blue.
        let bubbles = Color::new(0.01, 0.05, 0.06) * foam;
        PhysicalOcean {
            water: body + bubbles, roughness: roughness(node.froude), center: Some(self.frame.centre()),
            absorption: Some(attenuation), foam: None,
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

impl MaterialModel for RiverWater {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let Some((node, along)) = self.flow(&intersection.point) else {
            // A lake's sheet seen from below is only ever seen from the air beneath its rim,
            // where the smoothed ground lies lower than the simulation's spill level: not water
            // that is there. Pass through it.
            if r.rd.dot(&intersection.normal) > 0. {
                let ro = intersection.point + r.rd.normalize() * 2. * crate::material::functions::surface_offset(intersection, s);
                return ScatteredRay { emit: Color::black(), attenuate: Color::white(), light_sampling: crate::material::model::LightSampling::None, ray: Some(Ray { ro, rd: r.rd }) };
            }
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
