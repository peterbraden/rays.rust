//! # Conductors
//!
//! A metal's colour is not a reflectance someone chose, but follows from its complex refractive
//! index η + ik, which varies with wavelength: copper and gold reflect red light strongly and
//! blue less, silver and aluminium all of it nearly alike, and all of them turn white at grazing
//! angles (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed., section 9.4,
//! `ConductorBxDF`; cited below as PBR). The reflectance is Fresnel's for a complex index
//! (`ggx::conductor_reflectance`), smooth or on GGX microfacets, isotropic or anisotropic.
//!
//! The index is one of:
//!
//! - a measured metal (`spectrum::metals`): silver, aluminium, gold, copper or brass. In
//!   spectral mode, η and k at the sample's wavelengths; in RGB, at 630, 532 and 465 nm, the
//!   wavelengths commonly taken for the red, green and blue primaries.
//! - η and k given per RGB channel.
//! - a reflectance colour at normal incidence, as pbrt's `reflectance` parameter: η = 1 and
//!   k = 2 √r / √(1 - r), the conductor of index 1 with that reflectance (for which
//!   ((η - 1)² + k²) / ((η + 1)² + k²) = r).
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::functions::{reflect, scatter_bsdf};
use crate::material::ggx::{regularised, Fresnel, GGX, MIN_ALPHA};
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;

/// Wavelengths (nm) standing for the red, green and blue channels in RGB mode.
pub const RGB_WAVELENGTHS: [f64; 3] = [630., 532., 465.];

#[derive(Debug, Clone, PartialEq)]
pub enum Optics {
    Measured { eta: &'static [f64], k: &'static [f64] },
    Constant { eta: Color, k: Color },
    Reflectance(Color),
}

impl Optics {
    /// η and k for the sample being traced, per channel.
    pub fn current(&self) -> (Color, Color) {
        let wavelengths = crate::spectrum::current().map_or(RGB_WAVELENGTHS, |w| w.lambda);
        match self {
            Optics::Measured { eta, k } => {
                let at = |t: &[f64]| Color::new(
                    crate::spectrum::metals::lookup(t, wavelengths[0]),
                    crate::spectrum::metals::lookup(t, wavelengths[1]),
                    crate::spectrum::metals::lookup(t, wavelengths[2]));
                (at(eta), at(k))
            },
            Optics::Constant { eta, k } => (eta.as_unbounded(), k.as_unbounded()),
            Optics::Reflectance(r) => {
                let r = r.as_reflectance().rgb.map(|v| v.clamp(0., 0.9999));
                (Color::white(), Color { rgb: r.map(|v| 2. * v.sqrt() / (1. - v).sqrt()) })
            },
        }
    }
}

pub struct Conductor {
    pub optics: Optics,
    /// Perceptual roughness along the tangent and across it (α = r²).
    pub roughness: (f64, f64),
    /// The direction anisotropic roughness runs along, projected onto the surface.
    pub tangent: Vector3<f64>,
}

impl MaterialModel for Conductor {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let intersection = &intersection.facing(r);
        let (eta, k) = self.optics.current();
        let fresnel = Fresnel::Conductor { eta, k };
        let ax = regularised(self.roughness.0 * self.roughness.0, s);
        let ay = regularised(self.roughness.1 * self.roughness.1, s);
        if ax.max(ay) >= MIN_ALPHA {
            let (ax, ay) = (ax.max(MIN_ALPHA), ay.max(MIN_ALPHA));
            let aniso = (ax != ay).then_some((ay, self.tangent));
            let bsdf = GGX { alpha: ax, fresnel, normal: intersection.normal, aniso };
            return scatter_bsdf(&bsdf, r, &intersection.point, s);
        }
        let cos = (r.rd.dot(&intersection.normal) / r.rd.norm()).abs();
        let refl = Ray { ro: intersection.point, rd: reflect(r.rd, intersection.normal) };
        ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: fresnel.eval(cos), ray: Some(refl) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::ggx::conductor_reflectance;
    use crate::spectrum::{begin_sample, end_sample, Wavelengths};
    use crate::material::bsdf::BSDF;

    #[test]
    fn test_conductor_fresnel() {
        // k = 0: a dielectric's normal-incidence reflectance, ((η - 1) / (η + 1))².
        assert!((conductor_reflectance(1., 1.5, 0.) - 0.04).abs() < 1e-12);
        // Normal incidence: ((η - 1)² + k²) / ((η + 1)² + k²).
        let (eta, k) = (0.2, 3.);
        let expected = ((eta - 1f64).powi(2) + k * k) / ((eta + 1f64).powi(2) + k * k);
        assert!((conductor_reflectance(1., eta, k) - expected).abs() < 1e-12);
        // Grazing, everything reflects.
        assert!(conductor_reflectance(1e-6, 0.2, 3.) > 0.999);
    }

    #[test]
    fn test_measured_metals_have_their_colours() {
        let f0 = |name: &str| {
            let (eta, k) = crate::spectrum::metals::named(name).unwrap();
            let (e, kk) = Optics::Measured { eta, k }.current();
            Color::new(conductor_reflectance(1., e.rgb.x, kk.rgb.x), conductor_reflectance(1., e.rgb.y, kk.rgb.y), conductor_reflectance(1., e.rgb.z, kk.rgb.z))
        };
        let (gold, copper, silver, aluminium) = (f0("Au"), f0("Cu"), f0("Ag"), f0("Al"));
        // Gold and copper: red highest, blue lowest; copper redder than gold.
        assert!(gold.rgb.x > gold.rgb.y && gold.rgb.y > gold.rgb.z, "{:?}", gold);
        assert!(copper.rgb.x > copper.rgb.y && copper.rgb.y > copper.rgb.z, "{:?}", copper);
        // Copper is the redder: less green for its red (its blue is in fact a little higher).
        assert!(copper.rgb.y / copper.rgb.x < gold.rgb.y / gold.rgb.x, "{:?} {:?}", copper, gold);
        // Silver and aluminium: bright and nearly neutral.
        for c in [silver, aluminium] {
            assert!(c.rgb.min() > 0.85 && c.rgb.max() - c.rgb.min() < 0.06, "{:?}", c);
        }
    }

    /// The reflectance parameterisation reproduces the reflectance it is given.
    #[test]
    fn test_reflectance_optics() {
        let r = Color::new(0.9, 0.5, 0.1);
        let (eta, k) = Optics::Reflectance(r).current();
        for c in 0..3 {
            assert!((conductor_reflectance(1., eta.rgb[c], k.rgb[c]) - r.rgb[c]).abs() < 1e-9);
        }
    }

    /// In spectral mode, the measured metal's index is the table's at the sample's wavelengths.
    #[test]
    fn test_spectral_optics_follow_the_wavelengths() {
        let (eta, k) = crate::spectrum::metals::named("Cu").unwrap();
        let w = Wavelengths::sample_visible(0.2);
        begin_sample(w);
        let (e, _) = Optics::Measured { eta, k }.current();
        end_sample();
        assert_eq!(e.rgb.x, crate::spectrum::metals::lookup(eta, w.lambda[0]));
    }

    /// An anisotropic lobe still integrates consistently (`bsdf::tests::check_bsdf`).
    #[test]
    fn test_anisotropic_ggx_is_consistent() {
        let bsdf = GGX { alpha: 0.1, fresnel: Fresnel::Schlick(Color::white()), normal: Vector3::new(0., 0., 1.), aniso: Some((0.5, Vector3::x())) };
        crate::material::bsdf::tests::check_bsdf(&bsdf, &Vector3::new(0.3, 0.2, 0.9).normalize(), 0.02);
        // Stretched along the bitangent: reflections spread more in y than in x.
        let wo = Vector3::new(0., 0., 1.);
        let along_x = bsdf.pdf(&wo, &Vector3::new(0.3, 0., 1.).normalize());
        let along_y = bsdf.pdf(&wo, &Vector3::new(0., 0.3, 1.).normalize());
        assert!(along_y > 5. * along_x, "{} {}", along_x, along_y);
    }
}
