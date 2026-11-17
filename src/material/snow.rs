//! # Snow and glacier ice
//!
//! Snow and glacier ice are one substance, ice, divided differently: snow into grains a tenth
//! of a millimetre to a few millimetres across, glacier ice into a solid holding air bubbles.
//! Ice barely absorbs visible light, but every interface between ice and air scatters it, so
//! light that enters wanders through many grains or past many bubbles before it leaves again.
//! How white the result is depends on how far it wanders: in fine new snow a few millimetres,
//! and so it comes back nearly all, at all wavelengths; in coarse old snow farther, and ice
//! absorbs a little more red; in bubbly glacier ice metres, and red light is mostly absorbed on
//! the way, which is why glacier ice, crevasses and the inside of a hole in the snow are blue.
//! Both materials here are built from that physics rather than chosen colours: the measured
//! absorption of ice, the size of the grains or bubbles, and the soot and dust they hold.
//!
//! ## Ice's absorption
//!
//! Pure ice's absorption coefficient α(λ) = 4π k(λ) / λ, from the imaginary part k of its
//! refractive index, falls from about 2.5 m⁻¹ at 800 nm to about 0.006 m⁻¹ near 400 nm, its
//! minimum: Stephen G. Warren and Richard E. Brandt, "Optical constants of ice from the
//! ultraviolet to the microwave: A revised compilation", Journal of Geophysical Research 113
//! (2008), D14220, doi:10.1029/2007JD009744, and Thomas C. Grenfell and Donald K. Perovich,
//! "Radiation absorption coefficients of polycrystalline ice from 400-1400 nm", JGR 86(C8)
//! (1981), pp. 7447-7450, doi:10.1029/JC086iC08p07447. `ice_absorption` interpolates a table
//! rounded from those (log-linearly, as α spans three orders of magnitude); the values are
//! good to perhaps 20%, which shifts the colours here very little. The real part of the index
//! is 1.31 across the visible.
//!
//! ## Light in a weakly absorbing, strongly scattering medium
//!
//! Where absorption is weak beside scattering, the radiative transfer equation has an
//! asymptotic solution (Alexander A. Kokhanovsky and Eleonora P. Zege, "Scattering optics of
//! snow", Applied Optics 43(7), 2004, pp. 1589-1602, doi:10.1364/AO.43.001589): a semi-infinite
//! layer's spherical albedo (reflectance under diffuse light) is
//!
//! ```text
//! r_s = exp(-4 √(β / (3 (1 - g))))        β = σ_a / σ_e
//! ```
//!
//! the probability β of absorption at each extinction, and the asymmetry g of the scattering.
//! The square root is the signature of a diffusing medium: light makes many scatterings, about
//! 1 / ((1 - g) β), but the absorption it suffers grows only as their square root. Its
//! reflection function, R in units of a white Lambertian surface's, for light arriving at
//! cosine μ₀ and leaving at μ, is
//!
//! ```text
//! R(μ₀, μ, θ) = R₀(μ₀, μ, θ) r_s^(u(μ₀) u(μ) / R₀)        u(μ) = 3 (1 + 2 μ) / 7
//! ```
//!
//! where u is the escape function and R₀ the reflection function of the same layer without
//! absorption, which carries the angular shape. Kokhanovsky and Bréon fitted R₀ to the
//! radiative transfer of fractal snow grains, as a function of the scattering angle θ:
//!
//! ```text
//! R₀ = (a + b (μ₀ + μ) + c μ₀ μ + p(θ)) / (4 (μ₀ + μ))
//! p(θ) = 11.1 exp(-0.087 θ) + 1.1 exp(-0.014 θ)     (θ in degrees)
//! ```
//!
//! with a = 1.247, b = 1.186, c = 5.157 (Alexander A. Kokhanovsky and François-Marie Bréon,
//! "Validation of an Analytical Snow BRDF Model Using PARASOL Multi-Angular and Multispectral
//! Observations", IEEE Geoscience and Remote Sensing Letters 9(5), 2012, pp. 928-932,
//! doi:10.1109/LGRS.2012.2185775). p(θ) is the single-scattering phase function's forward
//! peak: snow is brighter looking towards a low sun than with it behind you, and the
//! difference grows with the angle of the light, as photographs of snowfields show. Integrated
//! over the hemisphere R₀ gives an albedo within 1.5% of one except at grazing light (6% over
//! at 85°); dividing by √(A(μ₀) A(μ)), its albedo A on each side, keeps it reciprocal and
//! conserves energy (`SnowBsdf`).
//!
//! ### Snow
//!
//! Grains of optical diameter d (six times their volume over their surface area), filling a
//! fraction c of the volume, have extinction σ_e = 3 c / d (geometric optics, extinction
//! efficiency 2), and absorb σ_a = c B α: ice's own absorption, enhanced by the factor B = 1.6
//! by which refraction lengthens a ray's path inside a fractal grain, with g = 0.75 (Kokhanovsky
//! and Zege). The volume fraction cancels: β = B α d / 3, and density doesn't change the
//! colour of deep snow, only grain size does. Fresh snow has d of about 0.1 mm, settled snow
//! 0.2-0.5 mm, old and melting snow and firn 1-3 mm.
//!
//! Snow is rarely clean. Soot (black carbon) absorbs with a mass absorption cross-section of
//! about 7.5 m² g⁻¹ at 550 nm, falling as 1 / λ (Tami C. Bond and Robert W. Bergstrom, "Light
//! Absorption by Carbonaceous Particles: An Investigative Review", Aerosol Science and
//! Technology 40(1), 2006, doi:10.1080/02786820500421521); mineral dust far more weakly, about
//! 0.03 m² g⁻¹ (typical of fine dust; it varies with the dust's size and iron oxides), but
//! steeply more in the blue (taken as λ⁻³), which makes dusty snow yellow-brown.
//! Each adds σ_a = c ρ_ice C MAC for a mass fraction C of the ice: tens of parts per billion of
//! soot are typical of remote snow, hundreds near cities; tens to thousands of parts per
//! million of dust in spring snow near deserts and on old glacier surfaces. A little soot dims
//! coarse snow far more than fine (Stephen G. Warren and Warren J. Wiscombe, "A Model for the
//! Spectral Albedo of Snow. II: Snow Containing Atmospheric Aerosols", Journal of the
//! Atmospheric Sciences 37(12), 1980, doi:10.1175/1520-0469(1980)037<2734:AMFTSA>2.0.CO;2).
//!
//! #### Sparkle
//!
//! Close up, snow glitters: here and there a grain's flat face mirrors the sun. Each grain
//! (a cell of the grain's size, hashed from its position) turns a facet in a random direction,
//! distributed by the cosine about the surface normal, and a fraction `sparkle` of the surface
//! reflects as that smooth facet would (GGX of roughness 0.01, Fresnel for ice). Seen from far
//! off, many grains share a pixel and the glints average into a faint glossy sheen; close up
//! each is a point of light.
//!
//! ### Glacier ice
//!
//! Glacier ice is solid, with air bubbles of radius r (0.1-2 mm) at N per cm³ (from tens in
//! the white ice near a glacier's surface to well under one in blue ice), which scatter with
//! σ_e = 2 π r² N and g ≈ 0.8 (Peter C. Mullen and Stephen G. Warren, "Theory of the optical
//! properties of lake ice", JGR 93(D7), 1988, doi:10.1029/JD093iD07p08321). The light now
//! travels metres, and r_s is blue. The ice's surface is a smooth interface, not a heap of
//! grains: it reflects by Fresnel's law for index 1.31 (GGX, of the surface's roughness), and
//! light refracted in diffuses, is reflected back from inside by the interface with the
//! internal diffuse reflectance ρ_i, and leaves as from a diffuse source beneath a dielectric:
//!
//! ```text
//! f_body = (1 - F(μ₀)) (1 - F(μ)) / (π η²) · r_s / (1 - r_s ρ_i)
//! ```
//!
//! (the classic result for a scattering medium under a smooth interface, eg. Matt Pharr and Pat
//! Hanrahan, "Monte Carlo evaluation of non-linear scattering equations for subsurface
//! reflection", SIGGRAPH 2000, and Jos Stam's skin model; with ρ_i = 1 - (1 - F̄) / η² by
//! reciprocity, F̄ the hemispherical mean of the outside reflectance), which conserves energy:
//! for r_s = 1 all light not reflected at the surface comes back.
//!
//! ## Colour
//!
//! In spectral mode r_s is evaluated at each of the path's wavelengths. In RGB mode it is the
//! sRGB colour of the reflectance spectrum r_s(λ) under D65, worked out once per material,
//! and its logarithm per channel stands in for the spectrum's in R's exponent.
use crate::color::Color;
use crate::geometry::{point_on_unit_sphere, rand};
use crate::intersection::Intersection;
use crate::material::bsdf::{BSDFSample, BSDF};
use crate::material::functions::{fresnel_reflectance, scatter_bsdf};
use crate::material::ggx::{regularised, Fresnel, GGX};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use std::f64::consts::PI;
use std::sync::OnceLock;

/// The real part of ice's refractive index across the visible.
pub const ICE_REFRACTIVE_INDEX: f64 = 1.31;

/// Density of ice, g per m³.
const ICE_DENSITY: f64 = 917e3;

/// Ice's absorption coefficient, m⁻¹, at wavelengths (nm): rounded from Warren and Brandt (2008)
/// and Grenfell and Perovich (1981) (see the module's notes).
const ICE_ABSORPTION: [(f64, f64); 20] = [
    (350., 0.009), (375., 0.007), (400., 0.006), (425., 0.0068), (450., 0.0092),
    (475., 0.014), (500., 0.022), (525., 0.035), (550., 0.055), (575., 0.085),
    (600., 0.12), (625., 0.18), (650., 0.26), (675., 0.40), (700., 0.60),
    (725., 0.95), (750., 1.40), (775., 1.90), (800., 2.30), (830., 2.70),
];

/// Pure ice's absorption coefficient at `lambda` nm, m⁻¹: the table interpolated in log α, held
/// at its ends beyond them.
pub fn ice_absorption(lambda: f64) -> f64 {
    let t = &ICE_ABSORPTION;
    if lambda <= t[0].0 { return t[0].1; }
    if lambda >= t[t.len() - 1].0 { return t[t.len() - 1].1; }
    let i = t.iter().position(|&(l, _)| l > lambda).unwrap() - 1;
    let f = (lambda - t[i].0) / (t[i + 1].0 - t[i].0);
    (t[i].1.ln() * (1. - f) + t[i + 1].1.ln() * f).exp()
}

/// Soot and dust carried by the ice.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Impurities {
    /// Black carbon, parts per billion by mass.
    pub soot: f64,
    /// Mineral dust, parts per million by mass.
    pub dust: f64,
}

impl Impurities {
    /// Their absorption per unit volume of ice, m⁻¹, at `lambda` nm.
    pub fn absorption(&self, lambda: f64) -> f64 {
        let soot = 7.5 * (550. / lambda) * self.soot * 1e-9;
        let dust = 0.03 * (550. / lambda).powi(3) * self.dust * 1e-6;
        ICE_DENSITY * (soot + dust)
    }
}

/// Spherical albedo of a semi-infinite medium absorbing a fraction `beta` of the light at each
/// extinction, scattering with asymmetry `g` (asymptotic radiative transfer).
pub fn spherical_albedo(beta: f64, g: f64) -> f64 {
    (-4. * (beta.max(0.) / (3. * (1. - g))).sqrt()).exp()
}

/// The sRGB colour of a reflectance spectrum lit by D65.
pub fn reflectance_rgb(f: impl Fn(f64) -> f64) -> Color {
    let c = crate::spectrum::xyz_to_srgb(&crate::spectrum::spectrum_to_xyz(|l| f(l) * crate::spectrum::d65(l)));
    Color { rgb: c.rgb.map(|v| v.clamp(1e-6, 1.)) }
}

/// A medium's spherical albedo at the current wavelengths, or in RGB its precomputed colour.
fn current(rgb: &Color, at: impl Fn(f64) -> f64) -> Color {
    match crate::spectrum::current() {
        None => *rgb,
        Some(w) => w.map(at),
    }
}

/// Asymmetry of snow grains' scattering, and the factor by which refraction lengthens the path
/// through them (Kokhanovsky and Zege, for fractal grains).
const SNOW_G: f64 = 0.75;
const SNOW_B: f64 = 1.6;

/// What a snowpack is made of.
#[derive(Clone, Debug, PartialEq)]
pub struct SnowStyle {
    /// Optical grain diameter, mm.
    pub grain_size: f64,
    pub impurities: Impurities,
    /// The fraction of the surface that glints as grains' facets, 0 to 1.
    pub sparkle: f64,
}

impl SnowStyle {
    /// `fresh` (new powder, 0.1 mm), `settled` (0.3 mm, a few days old), `old` (1 mm, weeks
    /// old, a little dirty), `firn` (2 mm, last year's snow on a glacier, dusty), `dirty` (late
    /// spring snow by a road: soot and dust).
    pub fn preset(name: &str) -> SnowStyle {
        let (grain_size, soot, dust, sparkle) = match name {
            "fresh" => (0.1, 5., 1., 0.08),
            "settled" => (0.3, 15., 5., 0.06),
            "old" => (1., 40., 20., 0.04),
            "firn" => (2., 50., 100., 0.03),
            "dirty" => (1., 500., 1000., 0.02),
            _ => panic!("Unknown snow preset '{}': expected fresh, settled, old, firn or dirty", name),
        };
        SnowStyle { grain_size, impurities: Impurities { soot, dust }, sparkle }
    }

    /// The probability of absorption per extinction at `lambda` nm.
    pub fn beta(&self, lambda: f64) -> f64 {
        let d = self.grain_size * 1e-3;
        d / 3. * (SNOW_B * ice_absorption(lambda) + self.impurities.absorption(lambda))
    }

    pub fn spherical_albedo(&self, lambda: f64) -> f64 {
        spherical_albedo(self.beta(lambda), SNOW_G)
    }
}

/// The escape function.
fn escape(mu: f64) -> f64 { 3. * (1. + 2. * mu) / 7. }

/// Kokhanovsky and Bréon's reflection function of non-absorbing snow, for cosines `mu0`, `mu`
/// and scattering angle `theta` (degrees).
fn r0(mu0: f64, mu: f64, theta: f64) -> f64 {
    let p = 11.1 * (-0.087 * theta).exp() + 1.1 * (-0.014 * theta).exp();
    (1.247 + 1.186 * (mu0 + mu) + 5.157 * mu0 * mu + p) / (4. * (mu0 + mu))
}

/// Entries in the table of R₀'s albedo, by cosine.
const ALBEDO_TABLE: usize = 64;

/// R₀'s albedo ∫ R₀ μ dω / π for light at each cosine (i + 0.5) / ALBEDO_TABLE.
fn r0_albedo() -> &'static [f64; ALBEDO_TABLE] {
    static T: OnceLock<[f64; ALBEDO_TABLE]> = OnceLock::new();
    T.get_or_init(|| {
        let mut t = [0.; ALBEDO_TABLE];
        let n = 256;
        for (k, a) in t.iter_mut().enumerate() {
            let mu0 = (k as f64 + 0.5) / ALBEDO_TABLE as f64;
            let s0 = (1. - mu0 * mu0).sqrt();
            let mut sum = 0.;
            for i in 0..n {
                let mu = (i as f64 + 0.5) / n as f64;
                let s = (1. - mu * mu).sqrt();
                for j in 0..n {
                    let phi = (j as f64 + 0.5) / n as f64 * PI;
                    let cos = (-mu0 * mu + s0 * s * phi.cos()).clamp(-1., 1.);
                    sum += r0(mu0, mu, cos.acos().to_degrees()) * mu;
                }
            }
            // ∫ over μ in [0, 1] and φ in [0, 2π] (symmetric), divided by π.
            *a = sum / (n * n) as f64 * 2.;
        }
        t
    })
}

fn r0_albedo_at(mu: f64) -> f64 {
    let t = r0_albedo();
    let x = (mu * ALBEDO_TABLE as f64 - 0.5).clamp(0., (ALBEDO_TABLE - 1) as f64);
    let i = (x as usize).min(ALBEDO_TABLE - 2);
    let f = x - i as f64;
    t[i] * (1. - f) + t[i + 1] * f
}

/// Snow's BSDF at a point: the asymptotic reflection function, and a glinting facet.
pub struct SnowBsdf {
    pub normal: Vector3<f64>,
    /// -ln r_s per channel.
    pub absorption: Color,
    /// The grain's facet here, and the fraction of the surface that glints.
    pub facet: Option<(GGX, f64)>,
}

/// The probability of sampling the facet rather than the body, when there is one.
const FACET_PICK: f64 = 0.25;

/// Mean Fresnel reflectance of ice over the cosine-weighted hemisphere (about 0.06), the share
/// of a facet's light it reflects rather than passes into the snow.
fn ice_mean_fresnel() -> f64 {
    static F: OnceLock<f64> = OnceLock::new();
    *F.get_or_init(|| {
        let n = 2000;
        (0..n).map(|i| {
            let mu = (i as f64 + 0.5) / n as f64;
            2. * mu * fresnel_reflectance(mu, 1. / ICE_REFRACTIVE_INDEX, ICE_REFRACTIVE_INDEX)
        }).sum::<f64>() / n as f64
    })
}

impl SnowBsdf {
    fn body(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let (mu0, mu) = (wi.dot(&self.normal), wo.dot(&self.normal));
        if mu0 <= 0. || mu <= 0. {
            return Color::black();
        }
        let theta = (-wi.dot(wo)).clamp(-1., 1.).acos().to_degrees();
        let r = r0(mu0, mu, theta) / (r0_albedo_at(mu0) * r0_albedo_at(mu)).sqrt();
        let e = escape(mu0) * escape(mu) / r;
        let scale = self.facet.as_ref().map_or(1., |(_, w)| 1. - w * ice_mean_fresnel());
        Color { rgb: self.absorption.rgb.map(|y| r * (-y * e).exp()) } * (scale * mu0 / PI)
    }
}

impl BSDF for SnowBsdf {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let mut f = self.body(wo, wi);
        if let Some((g, w)) = &self.facet && wi.dot(&self.normal) > 0. && wo.dot(&self.normal) > 0. {
            f = f + g.eval(wo, wi) * *w;
        }
        f
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let cos = wi.dot(&self.normal);
        if cos <= 0. {
            return 0.;
        }
        match &self.facet {
            None => cos / PI,
            Some((g, _)) => (1. - FACET_PICK) * cos / PI + FACET_PICK * g.pdf(wo, wi),
        }
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        if wo.dot(&self.normal) <= 0. {
            return None;
        }
        let wi = match &self.facet {
            Some((g, _)) if u[2] < FACET_PICK => g.sample(wo, [u[0], u[1], u[2] / FACET_PICK])?.wi,
            _ => {
                let d = self.normal + point_on_unit_sphere(u[0], u[1]);
                if d.norm() < 1e-9 { self.normal } else { d.normalize() }
            }
        };
        let pdf = self.pdf(wo, &wi);
        if pdf <= 0. {
            return None;
        }
        Some(BSDFSample { wi, weight: self.eval(wo, &wi) / pdf, pdf })
    }
}

/// Snow (see the module's notes), its grains `scale` scene units per metre apart.
pub struct Snow {
    pub style: SnowStyle,
    /// Scene units per metre.
    pub scale: f64,
    pub seed: u64,
    /// The spherical albedo's colour, for RGB mode.
    rgb: Color,
}

impl Snow {
    pub fn new(style: SnowStyle, scale: f64, seed: u64) -> Snow {
        let rgb = reflectance_rgb(|l| style.spherical_albedo(l));
        Snow { style, scale, seed, rgb }
    }

    /// The spherical albedo (diffuse reflectance) in RGB.
    pub fn albedo(&self) -> Color { self.rgb }

    /// The BSDF at `point`, with normal `normal` facing the viewer.
    pub fn bsdf(&self, point: &Vector3<f64>, normal: &Vector3<f64>, s: Option<&Scene>) -> SnowBsdf {
        let albedo = current(&self.rgb, |l| self.style.spherical_albedo(l));
        let absorption = Color { rgb: albedo.rgb.map(|a| -a.max(1e-12).ln()) };
        let facet = if self.style.sparkle > 0. {
            // The grain this point is in, and its facet's direction.
            let cell = point / (self.scale * self.style.grain_size * 1e-3);
            let h = [cell.x, cell.y, cell.z].iter().fold(self.seed, |h, c| crate::sobol::hash(h, c.floor() as i64 as u64));
            let u = |k: u64| (crate::sobol::hash(h, k) >> 11) as f64 / (1u64 << 53) as f64;
            let m = (normal + point_on_unit_sphere(u(1), u(2))).try_normalize(1e-9).unwrap_or(*normal);
            let alpha = s.map_or(0.01, |s| regularised(0.01, s));
            Some((GGX::new(alpha, Fresnel::Dielectric(ICE_REFRACTIVE_INDEX), m), self.style.sparkle))
        } else {
            None
        };
        SnowBsdf { normal: *normal, absorption, facet }
    }
}

impl MaterialModel for Snow {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = &intersection.facing(r);
        scatter_bsdf(&self.bsdf(&i.point, &i.normal, Some(s)), r, &i.point, s)
    }
}

/// Asymmetry of scattering by bubbles in ice (Mullen and Warren).
const BUBBLE_G: f64 = 0.8;

/// What glacier ice is made of.
#[derive(Clone, Debug, PartialEq)]
pub struct IceStyle {
    /// Bubble radius, mm, and bubbles per cm³.
    pub bubble_radius: f64,
    pub bubbles: f64,
    pub impurities: Impurities,
    /// GGX roughness of the surface.
    pub roughness: f64,
}

impl IceStyle {
    /// `white` (the bubble-rich ice of a glacier's surface, 30 per cm³), `glacier` (bubbly
    /// glacier ice, 4 per cm³), `blue` (old, compressed ice whose bubbles have dissolved, 0.3 per
    /// cm³), `clear` (nearly bubble-free: lake ice, an iceberg's heart).
    pub fn preset(name: &str) -> IceStyle {
        let (bubble_radius, bubbles, dust, roughness) = match name {
            "white" => (0.5, 30., 0.5, 0.2),
            "glacier" => (0.6, 4., 1., 0.08),
            "blue" => (0.8, 0.3, 0.2, 0.03),
            "clear" => (0.5, 0.01, 0., 0.01),
            _ => panic!("Unknown ice preset '{}': expected white, glacier, blue or clear", name),
        };
        IceStyle { bubble_radius, bubbles, impurities: Impurities { soot: 0., dust }, roughness }
    }

    /// Bubbles' extinction coefficient, m⁻¹.
    pub fn extinction(&self) -> f64 {
        let r = self.bubble_radius * 1e-3;
        2. * PI * r * r * self.bubbles * 1e6
    }

    /// The bubbly body's spherical albedo at `lambda` nm.
    pub fn spherical_albedo(&self, lambda: f64) -> f64 {
        let absorption = ice_absorption(lambda) + self.impurities.absorption(lambda);
        let e = self.extinction();
        if e <= 0. { 0. } else { spherical_albedo(absorption / e, BUBBLE_G) }
    }
}

/// Internal diffuse reflectance of ice's surface, from inside: 1 - (1 - F̄) / η².
fn ice_internal_reflectance() -> f64 {
    1. - (1. - ice_mean_fresnel()) / (ICE_REFRACTIVE_INDEX * ICE_REFRACTIVE_INDEX)
}

/// Glacier ice's BSDF: Fresnel reflection off its surface over light diffused by its bubbles.
pub struct IceBsdf {
    pub surface: GGX,
    pub normal: Vector3<f64>,
    /// The body's spherical albedo per channel.
    pub albedo: Color,
}

impl IceBsdf {
    fn specular_pick(&self, wo: &Vector3<f64>) -> f64 {
        let f = fresnel_reflectance(wo.dot(&self.normal).max(0.), 1. / ICE_REFRACTIVE_INDEX, ICE_REFRACTIVE_INDEX);
        let body = crate::color::luminance(&self.albedo);
        (f / (f + (1. - f) * body).max(1e-9)).clamp(0.1, 0.9)
    }

    fn body(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let (mu0, mu) = (wi.dot(&self.normal), wo.dot(&self.normal));
        if mu0 <= 0. || mu <= 0. {
            return Color::black();
        }
        let n = ICE_REFRACTIVE_INDEX;
        let t = |c: f64| 1. - fresnel_reflectance(c, 1. / n, n);
        let rho = ice_internal_reflectance();
        let k = t(mu0) * t(mu) / (PI * n * n) * mu0;
        Color { rgb: self.albedo.rgb.map(|r| k * r / (1. - r * rho)) }
    }
}

impl BSDF for IceBsdf {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        self.surface.eval(wo, wi) + self.body(wo, wi)
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let cos = wi.dot(&self.normal);
        if cos <= 0. {
            return 0.;
        }
        let p = self.specular_pick(wo);
        p * self.surface.pdf(wo, wi) + (1. - p) * cos / PI
    }

    fn sample(&self, wo: &Vector3<f64>, u: [f64; 3]) -> Option<BSDFSample> {
        if wo.dot(&self.normal) <= 0. {
            return None;
        }
        let p = self.specular_pick(wo);
        let wi = if u[2] < p {
            self.surface.sample(wo, [u[0], u[1], u[2] / p])?.wi
        } else {
            let d = self.normal + point_on_unit_sphere(u[0], u[1]);
            if d.norm() < 1e-9 { self.normal } else { d.normalize() }
        };
        let pdf = self.pdf(wo, &wi);
        if pdf <= 0. {
            return None;
        }
        Some(BSDFSample { wi, weight: self.eval(wo, &wi) / pdf, pdf })
    }
}

/// Glacier ice (see the module's notes).
pub struct Ice {
    pub style: IceStyle,
    rgb: Color,
}

impl Ice {
    pub fn new(style: IceStyle) -> Ice {
        let rgb = reflectance_rgb(|l| style.spherical_albedo(l));
        Ice { style, rgb }
    }

    /// The body's spherical albedo in RGB (beneath the surface's reflection).
    pub fn albedo(&self) -> Color { self.rgb }

    /// The diffuse reflectance seen from outside, surface reflection included, in RGB.
    pub fn reflectance(&self) -> Color {
        let rho = ice_internal_reflectance();
        let f = ice_mean_fresnel();
        Color { rgb: self.rgb.rgb.map(|r| f + (1. - f) * (1. - rho) * r / (1. - r * rho)) }
    }

    pub fn bsdf(&self, normal: &Vector3<f64>, s: Option<&Scene>) -> IceBsdf {
        let alpha = s.map_or(self.style.roughness, |s| regularised(self.style.roughness, s)).max(1e-3);
        IceBsdf {
            surface: GGX::new(alpha, Fresnel::Dielectric(ICE_REFRACTIVE_INDEX), *normal),
            normal: *normal,
            albedo: current(&self.rgb, |l| self.style.spherical_albedo(l)),
        }
    }
}

impl MaterialModel for Ice {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = &intersection.facing(r);
        scatter_bsdf(&self.bsdf(&i.normal, Some(s)), r, &i.point, s)
    }
}

/// Snow lying on another material: on surfaces facing the sky, where it settles, with edges
/// that wander. `cover` runs from 0 (none) through 0.5 (on all surfaces facing upwards) to 1 (everything but overhangs; at 0.3 slopes up to about 55°); `drift` (scene units)
/// is the size of the wandering.
pub struct SnowCover {
    pub base: Box<dyn MaterialModel + Sync + Send>,
    pub snow: Snow,
    pub cover: f64,
    pub up: Vector3<f64>,
    pub drift: f64,
    noise: crate::noise::PerlinNoise,
}

impl SnowCover {
    pub fn new(base: Box<dyn MaterialModel + Sync + Send>, snow: Snow, cover: f64, up: Vector3<f64>, drift: f64, seed: u64) -> SnowCover {
        SnowCover { base, snow, cover, up: up.normalize(), drift, noise: crate::noise::PerlinNoise::shuffled(seed.wrapping_add(41)) }
    }

    /// The share of snow at a point with outward normal `normal`.
    pub fn share(&self, point: &Vector3<f64>, normal: &Vector3<f64>) -> f64 {
        let q = point / self.drift.max(1e-9);
        let wander = self.noise.fbm(q.x, q.y, q.z, 4, 0.5, 2.) - 0.5;
        // The threshold on the normal's cosine from up, from beyond 1 (none) to beyond -1
        // (all), allowing for the wandering.
        let threshold = 1.35 - 2.7 * self.cover;
        let level = normal.dot(&self.up) + 0.5 * wander;
        crate::procedural::nubis_clouds::smoothstep(threshold - 0.05, threshold + 0.05, level)
    }
}

impl MaterialModel for SnowCover {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        if rand() < self.share(&intersection.point, &intersection.normal) && r.rd.dot(&intersection.normal) < 0. {
            self.snow.scatter(r, intersection, s)
        } else {
            self.base.scatter(r, intersection, s)
        }
    }
}

/// Pick snow, ice or something else by weight: for ground covered partly by each.
pub fn pick(weights: &[f64]) -> usize {
    let total: f64 = weights.iter().sum();
    let mut u = rand() * total;
    for (i, w) in weights.iter().enumerate() {
        if u < *w { return i; }
        u -= w;
    }
    weights.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::bsdf::tests::check_bsdf;

    #[test]
    fn test_ice_absorbs_red_far_more_than_blue() {
        assert!((ice_absorption(400.) - 0.006).abs() < 1e-9);
        assert!(ice_absorption(700.) / ice_absorption(450.) > 50.);
        // Monotonic from the minimum to the red.
        let mut last = 0.;
        for l in (400..830).step_by(5) {
            let a = ice_absorption(l as f64);
            assert!(a >= last, "{} nm", l);
            last = a;
        }
    }

    /// R₀ has an albedo close to one at all angles, and the table matches it.
    #[test]
    fn test_r0_conserves_energy() {
        for mu in [0.05, 0.3, 0.6, 0.95] {
            let a = r0_albedo_at(mu);
            assert!(a > 0.97 && a < 1.12, "{}: {}", mu, a);
        }
    }

    /// Clean fresh snow is nearly white (over 0.97 in all channels) and slightly blue; old snow
    /// is greyer, and soot darkens coarse snow more than fine.
    #[test]
    fn test_snow_albedo_by_grain_size_and_soot() {
        let fresh = Snow::new(SnowStyle::preset("fresh"), 1., 0).albedo();
        assert!(fresh.rgb.min() > 0.97, "{:?}", fresh);
        assert!(fresh.rgb.z > fresh.rgb.x, "{:?}", fresh);
        let old = Snow::new(SnowStyle::preset("old"), 1., 0).albedo();
        assert!(old.rgb.x < fresh.rgb.x - 0.02, "{:?}", old);
        let sooty = |d: f64| {
            let clean = SnowStyle { grain_size: d, impurities: Impurities::default(), sparkle: 0. };
            let dirty = SnowStyle { impurities: Impurities { soot: 200., dust: 0. }, ..clean.clone() };
            clean.spherical_albedo(550.) - dirty.spherical_albedo(550.)
        };
        assert!(sooty(1.) > 1.5 * sooty(0.1));
        let dirty = Snow::new(SnowStyle::preset("dirty"), 1., 0).albedo();
        // Dust browns it: blue darkened most.
        assert!(dirty.rgb.z < dirty.rgb.x && dirty.rgb.x < 0.9, "{:?}", dirty);
    }

    /// Bubbly glacier ice is blue, and the fewer its bubbles, the darker and bluer.
    #[test]
    fn test_glacier_ice_is_blue() {
        let white = Ice::new(IceStyle::preset("white")).albedo();
        let glacier = Ice::new(IceStyle::preset("glacier")).albedo();
        let blue = Ice::new(IceStyle::preset("blue")).albedo();
        for c in [white, glacier, blue] {
            assert!(c.rgb.z > c.rgb.y && c.rgb.y > c.rgb.x, "{:?}", c);
        }
        assert!(white.rgb.x > glacier.rgb.x && glacier.rgb.x > blue.rgb.x);
        assert!(blue.rgb.z / blue.rgb.x > glacier.rgb.z / glacier.rgb.x);
    }

    #[test]
    fn test_snow_bsdf_samples_and_conserves_energy() {
        let n = Vector3::new(0., 1., 0.);
        let style = SnowStyle { grain_size: 0.0, impurities: Impurities::default(), sparkle: 0. };
        let snow = Snow::new(style, 1., 0);
        for wo in [Vector3::new(0., 1., 0.), Vector3::new(0.6, 0.8, 0.), Vector3::new(0.95, 0.312, 0.)] {
            let b = snow.bsdf(&Vector3::zeros(), &n, None);
            check_bsdf(&b, &wo.normalize(), 0.02);
            // Non-absorbing snow reflects (nearly) everything, and never more.
            let mut rng = crate::geometry::Pcg32::new(3, 1);
            let k = 200_000;
            let a = (0..k).fold(0., |a, _| {
                let s = b.sample(&wo.normalize(), [rng.next_f64(), rng.next_f64(), rng.next_f64()]);
                a + s.map_or(0., |s| s.weight.rgb.x)
            }) / k as f64;
            assert!(a > 0.94 && a < 1.01, "{:?}: {}", wo, a);
        }
        // With sparkle.
        let snow = Snow::new(SnowStyle::preset("fresh"), 1., 3);
        let b = snow.bsdf(&Vector3::new(0.1, 0., 0.3), &n, None);
        check_bsdf(&b, &Vector3::new(0.3, 0.9, 0.1).normalize(), 0.03);
    }

    /// Snow is brighter looking towards a low sun (forward scattering) than away from it.
    #[test]
    fn test_snow_scatters_forward() {
        let n = Vector3::new(0., 1., 0.);
        let snow = Snow::new(SnowStyle { sparkle: 0., ..SnowStyle::preset("settled") }, 1., 0);
        let b = snow.bsdf(&Vector3::zeros(), &n, None);
        let wi = Vector3::new(-0.985, 0.174, 0.).normalize();
        let forward = b.eval(&Vector3::new(0.985, 0.174, 0.).normalize(), &wi);
        let back = b.eval(&wi, &wi);
        assert!(forward.rgb.y > 1.5 * back.rgb.y, "{:?} vs {:?}", forward, back);
    }

    #[test]
    fn test_ice_bsdf_samples_and_conserves_energy() {
        let n = Vector3::new(0., 1., 0.);
        // With a perfectly scattering body, all light comes back.
        let mut ice = Ice::new(IceStyle::preset("glacier"));
        ice.rgb = Color::white();
        ice.style.roughness = 0.3;
        for wo in [Vector3::new(0., 1., 0.), Vector3::new(0.6, 0.8, 0.)] {
            let b = ice.bsdf(&n, None);
            check_bsdf(&b, &wo, 0.02);
            let mut rng = crate::geometry::Pcg32::new(5, 1);
            let k = 200_000;
            let a = (0..k).fold(0., |a, _| {
                let s = b.sample(&wo, [rng.next_f64(), rng.next_f64(), rng.next_f64()]);
                a + s.map_or(0., |s| s.weight.rgb.x)
            }) / k as f64;
            assert!(a > 0.93 && a < 1.02, "{:?}: {}", wo, a);
        }
    }

    #[test]
    fn test_snow_cover_lies_on_top() {
        let c = SnowCover::new(Box::new(Snow::new(SnowStyle::preset("old"), 1., 0)), Snow::new(SnowStyle::preset("fresh"), 1., 0), 0.5, Vector3::y(), 1., 0);
        let p = Vector3::new(0.3, 0.2, 0.1);
        assert!(c.share(&p, &Vector3::y()) > 0.99);
        assert!(c.share(&p, &-Vector3::y()) < 0.01);
        let none = SnowCover { cover: 0., ..c };
        assert!(none.share(&p, &Vector3::y()) < 0.01);
    }

    /// Scene files name the materials, with presets and overrides.
    #[test]
    fn test_scene_materials_parse() {
        let o: serde_json::Value = serde_json::from_str(r#"{ "preset": "fresh", "grain_size": 0.2, "soot": 100 }"#).unwrap();
        let style = crate::scenefile::SceneFile::parse_snow_style(&o);
        assert_eq!((style.grain_size, style.impurities.soot, style.impurities.dust), (0.2, 100., 1.));
        let o: serde_json::Value = serde_json::from_str(r#"{ "preset": "blue", "bubbles": 2 }"#).unwrap();
        let ice = crate::scenefile::SceneFile::parse_ice_style(&o);
        assert_eq!((ice.bubbles, ice.bubble_radius), (2., 0.8));
        // Builds (an unknown type or a bad reference would panic).
        crate::scenefile::SceneFile::from_string(r#"{
            "camera": { "location": [0, 1, -3], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": {
                "S": { "type": "snow" }, "I": { "type": "ice", "preset": "white" },
                "R": { "type": "lambertian", "albedo": [0.2, 0.2, 0.2] },
                "C": { "type": "snow_cover", "material": "R", "cover": 0.4, "snow": { "preset": "fresh" } }
            },
            "objects": [
                { "type": "plane", "y": 0, "material": "S" },
                { "type": "sphere", "location": [0, 1, 0], "radius": 0.5, "material": "I" },
                { "type": "sphere", "location": [1, 1, 0], "radius": 0.5, "material": "C" }
            ]
        }"#.to_string());
    }

    /// In spectral mode the albedo follows the wavelength: high in the blue, lower in the red.
    #[test]
    fn test_spectral_albedo_follows_wavelength() {
        let ice = Ice::new(IceStyle::preset("glacier"));
        crate::spectrum::begin_sample(crate::spectrum::Wavelengths { lambda: [450., 550., 680.], pdf: [1.; 3], secondary_terminated: false });
        let a = ice.bsdf(&Vector3::y(), None).albedo;
        crate::spectrum::end_sample();
        assert!((a.rgb.x - ice.style.spherical_albedo(450.)).abs() < 1e-12);
        assert!(a.rgb.x > a.rgb.y && a.rgb.y > a.rgb.z);
    }
}
