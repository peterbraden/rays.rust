//! # Layered materials: coated diffuse and coated conductor
//!
//! A varnished table or a lacquered car is a base (wood, metallic paint) under a clear layer.
//! Light refracts into the coat, may scatter in it (a tinted or cloudy varnish), reflects off
//! the base, and bounces between the coat's underside and the base any number of times before
//! it leaves, tinted by every pass; `plastic` approximates this by a single Fresnel split.
//! pbrt's `LayeredBxDF` (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed.,
//! section 14.3; cited below as PBR) evaluates the layered BSDF stochastically instead, after
//! Guo, Hašan and Zhao, "Position-Free Monte Carlo Simulation for Arbitrary Layered BSDFs",
//! SIGGRAPH Asia 2018: each evaluation f(ω_o, ω_i) is an unbiased estimate from a short random
//! walk between the interfaces, connected to ω_i at each vertex by next-event estimation
//! through the exit interface. The layers are infinitely wide slabs, so position doesn't
//! matter, only depth z in [0, thickness], and the medium between them (if any) has unit
//! extinction, albedo `albedo` and a Henyey-Greenstein phase function.
//!
//! `eval` and `pdf` are therefore noisy (pbrt's too), but unbiased, which is all next-event
//! estimation and MIS need. A smooth coat's mirror reflection is a delta: `sample_layered`
//! reports it as specular, and the material traces it without MIS.
//!
//! One difference from pbrt: where a walk vertex's sampled direction is connected to ω_i
//! through the exit interface, its MIS weight competes with next-event estimation's density
//! for the same interior direction, sampled from ω_i (`exit.pdf(ω_i, -w)`); pbrt uses the
//! density the other way (`PDF(-w, wi)`), which differs for refraction. With pbrt's, `f` came
//! out 5% above what sampling delivers under a rough coat; with this, they agree (the tests).
//!
//! Each interface is one of pbrt's BxDFs in a local frame (z the normal): a dielectric (smooth,
//! or rough GGX after Walter et al. 2007), a diffuse reflector, or a conductor (complex
//! Fresnel), each sampling only reflection or only transmission when asked (`Flags`), and
//! scaling transmitted radiance by 1/η'² only for radiance, not importance (`Mode`; PBR
//! 9.5.2), since the walk traces light both ways.

use crate::color::Color;
use crate::geometry::rand;
use crate::material::bsdf::{BSDF, BSDFSample};
use crate::material::ggx::{conductor_reflectance, Fresnel, GGX, MIN_ALPHA};
use crate::material::phase::{henyey_greenstein, sample_henyey_greenstein};
use crate::material::rough_dielectric::fresnel_dielectric;
use crate::na::Vector3;
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    Radiance,
    Importance,
}

impl Mode {
    fn flip(self) -> Mode {
        if self == Mode::Radiance { Mode::Importance } else { Mode::Radiance }
    }
}

/// Which kinds of scattering a sample may choose.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Flags {
    pub reflection: bool,
    pub transmission: bool,
}

pub const ALL: Flags = Flags { reflection: true, transmission: true };
pub const REFLECTION: Flags = Flags { reflection: true, transmission: false };
pub const TRANSMISSION: Flags = Flags { reflection: false, transmission: true };

/// A sampled direction: the BSDF's value f (not times the cosine), and the density.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub wi: Vector3<f64>,
    pub f: Color,
    pub pdf: f64,
    pub specular: bool,
    pub transmission: bool,
}

fn same_hemisphere(a: &Vector3<f64>, b: &Vector3<f64>) -> bool {
    a.z * b.z > 0.
}

/// Refraction of `wi` through the surface of normal `n` at relative index `eta`, and the
/// relative index η' along the way (PBR's `Refract`); None at total internal reflection.
fn refract(wi: &Vector3<f64>, n: &Vector3<f64>, eta: f64) -> Option<(Vector3<f64>, f64)> {
    let (mut cos_i, mut eta, mut n) = (n.dot(wi), eta, *n);
    if cos_i < 0. {
        eta = 1. / eta;
        cos_i = -cos_i;
        n = -n;
    }
    let sin2_t = (1. - cos_i * cos_i).max(0.) / (eta * eta);
    if sin2_t >= 1. {
        return None;
    }
    let cos_t = (1. - sin2_t).sqrt();
    Some((-wi / eta + n * (cos_i / eta - cos_t), eta))
}

/// A GGX distribution about z, for the interfaces' microfacets.
fn distribution(alpha: f64) -> GGX {
    GGX::new(alpha, Fresnel::Schlick(Color::white()), Vector3::new(0., 0., 1.))
}

/// D_ω(m): the density of visible normal `m` seen from `w` (either side).
fn visible_pdf(g: &GGX, w: &Vector3<f64>, m: &Vector3<f64>) -> f64 {
    g.g1(w) / w.z.abs() * g.d(m) * w.dot(m).abs()
}

/// A visible normal seen from `w`, facing +z.
fn sample_visible(g: &GGX, w: &Vector3<f64>, u: [f64; 2]) -> Vector3<f64> {
    let up = if w.z < 0. { -w } else { *w };
    g.sample_visible_normal(&up, u)
}

fn g2(g: &GGX, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
    1. / (1. + g.lambda(wo) + g.lambda(wi))
}

/// One layer's interface.
pub enum Interface {
    /// A dielectric of relative index `eta` (below over above); smooth if `alpha` is below
    /// `MIN_ALPHA`.
    Dielectric { eta: f64, alpha: f64 },
    /// A diffuse reflector.
    Diffuse { r: Color },
    /// A conductor of complex index η + ik relative to the medium above it.
    Conductor { eta: Color, k: Color, alpha: f64 },
}

impl Interface {
    pub fn is_specular(&self) -> bool {
        match self {
            Interface::Dielectric { eta, alpha } => *eta == 1. || *alpha < MIN_ALPHA,
            Interface::Diffuse { .. } => false,
            Interface::Conductor { alpha, .. } => *alpha < MIN_ALPHA,
        }
    }

    fn conductor_fresnel(eta: &Color, k: &Color, cos: f64) -> Color {
        Color::new(conductor_reflectance(cos, eta.rgb.x, k.rgb.x), conductor_reflectance(cos, eta.rgb.y, k.rgb.y),
                   conductor_reflectance(cos, eta.rgb.z, k.rgb.z))
    }

    /// The BSDF f(ω_o, ω_i), zero for deltas.
    pub fn f(&self, wo: &Vector3<f64>, wi: &Vector3<f64>, mode: Mode) -> Color {
        if self.is_specular() {
            return Color::black();
        }
        match self {
            Interface::Diffuse { r } => if same_hemisphere(wo, wi) { *r / PI } else { Color::black() },
            Interface::Conductor { eta, k, alpha } => {
                if !same_hemisphere(wo, wi) {
                    return Color::black();
                }
                let (co, ci) = (wo.z.abs(), wi.z.abs());
                if co == 0. || ci == 0. {
                    return Color::black();
                }
                let Some(m) = (wo + wi).try_normalize(1e-12) else { return Color::black() };
                let m = if m.z < 0. { -m } else { m };
                let g = distribution(*alpha);
                Interface::conductor_fresnel(eta, k, wo.dot(&m).abs()) * (g.d(&m) * g2(&g, wo, wi) / (4. * ci * co))
            },
            Interface::Dielectric { eta, alpha } => {
                let Some((m, etap, reflect)) = dielectric_half(wo, wi, *eta) else { return Color::black() };
                let g = distribution(*alpha);
                let fr = fresnel_dielectric(wo.dot(&m), *eta);
                let (co, ci) = (wo.z, wi.z);
                if reflect {
                    Color::white() * (g.d(&m) * g2(&g, wo, wi) * fr / (4. * ci * co).abs())
                } else {
                    let denom = (wi.dot(&m) + wo.dot(&m) / etap).powi(2) * ci * co;
                    let mut ft = g.d(&m) * (1. - fr) * g2(&g, wo, wi) * (wi.dot(&m) * wo.dot(&m) / denom).abs();
                    if mode == Mode::Radiance {
                        ft /= etap * etap;
                    }
                    Color::white() * ft
                }
            },
        }
    }

    /// The density with which `sample` (restricted to `flags`) picks ω_i; zero for deltas.
    pub fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>, flags: Flags) -> f64 {
        if self.is_specular() {
            return 0.;
        }
        match self {
            Interface::Diffuse { .. } => {
                if flags.reflection && same_hemisphere(wo, wi) { wi.z.abs() / PI } else { 0. }
            },
            Interface::Conductor { alpha, .. } => {
                if !flags.reflection || !same_hemisphere(wo, wi) {
                    return 0.;
                }
                let Some(m) = (wo + wi).try_normalize(1e-12) else { return 0. };
                let m = if m.z < 0. { -m } else { m };
                let g = distribution(*alpha);
                visible_pdf(&g, wo, &m) / (4. * wo.dot(&m).abs())
            },
            Interface::Dielectric { eta, alpha } => {
                let Some((m, etap, reflect)) = dielectric_half(wo, wi, *eta) else { return 0. };
                let g = distribution(*alpha);
                let r = fresnel_dielectric(wo.dot(&m), *eta);
                let (pr, pt) = (if flags.reflection { r } else { 0. }, if flags.transmission { 1. - r } else { 0. });
                if pr + pt == 0. {
                    return 0.;
                }
                if reflect {
                    visible_pdf(&g, wo, &m) / (4. * wo.dot(&m).abs()) * pr / (pr + pt)
                } else {
                    let denom = (wi.dot(&m) + wo.dot(&m) / etap).powi(2);
                    visible_pdf(&g, wo, &m) * wi.dot(&m).abs() / denom * pt / (pr + pt)
                }
            },
        }
    }

    /// A direction from `wo`, choosing among the kinds `flags` allows with `uc`.
    pub fn sample(&self, wo: &Vector3<f64>, uc: f64, u: [f64; 2], mode: Mode, flags: Flags) -> Option<Sample> {
        match self {
            Interface::Diffuse { r } => {
                if !flags.reflection {
                    return None;
                }
                let d = Vector3::new(0., 0., 1.) + crate::geometry::point_on_unit_sphere(u[0], u[1]);
                let mut wi = d.try_normalize(1e-9).unwrap_or(Vector3::new(0., 0., 1.));
                if wo.z < 0. {
                    wi.z = -wi.z;
                }
                let pdf = wi.z.abs() / PI;
                if pdf <= 0. {
                    return None;
                }
                Some(Sample { wi, f: *r / PI, pdf, specular: false, transmission: false })
            },
            Interface::Conductor { eta, k, alpha } => {
                if !flags.reflection || wo.z == 0. {
                    return None;
                }
                if self.is_specular() {
                    let wi = Vector3::new(-wo.x, -wo.y, wo.z);
                    let f = Interface::conductor_fresnel(eta, k, wi.z.abs()) / wi.z.abs();
                    return Some(Sample { wi, f, pdf: 1., specular: true, transmission: false });
                }
                let g = distribution(*alpha);
                let m = sample_visible(&g, wo, u);
                let wi = m * (2. * wo.dot(&m)) - wo;
                if !same_hemisphere(wo, &wi) {
                    return None;
                }
                let pdf = self.pdf(wo, &wi, flags);
                if pdf <= 0. {
                    return None;
                }
                Some(Sample { wi, f: self.f(wo, &wi, mode), pdf, specular: false, transmission: false })
            },
            Interface::Dielectric { eta, alpha } => {
                let smooth = self.is_specular();
                let g = distribution(*alpha);
                let m = if smooth { Vector3::new(0., 0., 1.) } else { sample_visible(&g, wo, u) };
                let r = fresnel_dielectric(if smooth { wo.z } else { wo.dot(&m) }, *eta);
                let (pr, pt) = (if flags.reflection { r } else { 0. }, if flags.transmission { 1. - r } else { 0. });
                if pr + pt == 0. {
                    return None;
                }
                if uc < pr / (pr + pt) {
                    if smooth {
                        let wi = Vector3::new(-wo.x, -wo.y, wo.z);
                        return Some(Sample { wi, f: Color::white() * (r / wi.z.abs()), pdf: pr / (pr + pt), specular: true, transmission: false });
                    }
                    let wi = m * (2. * wo.dot(&m)) - wo;
                    if !same_hemisphere(wo, &wi) {
                        return None;
                    }
                    let pdf = self.pdf(wo, &wi, flags);
                    if pdf <= 0. {
                        return None;
                    }
                    Some(Sample { wi, f: self.f(wo, &wi, mode), pdf, specular: false, transmission: false })
                } else {
                    let (wi, etap) = refract(wo, &m, *eta)?;
                    if smooth {
                        if wi.z == 0. {
                            return None;
                        }
                        let mut ft = (1. - r) / wi.z.abs();
                        if mode == Mode::Radiance {
                            ft /= etap * etap;
                        }
                        return Some(Sample { wi, f: Color::white() * ft, pdf: pt / (pr + pt), specular: true, transmission: true });
                    }
                    if same_hemisphere(wo, &wi) || wi.z == 0. {
                        return None;
                    }
                    let pdf = self.pdf(wo, &wi, flags);
                    if pdf <= 0. {
                        return None;
                    }
                    Some(Sample { wi, f: self.f(wo, &wi, mode), pdf, specular: false, transmission: true })
                }
            },
        }
    }
}

/// The generalised half vector (facing +z), η', and whether it is a reflection, for a
/// dielectric's microfacet connecting `wo` and `wi`; None if none can.
fn dielectric_half(wo: &Vector3<f64>, wi: &Vector3<f64>, eta: f64) -> Option<(Vector3<f64>, f64, bool)> {
    let (co, ci) = (wo.z, wi.z);
    if co == 0. || ci == 0. {
        return None;
    }
    let reflect = co * ci > 0.;
    let etap = if reflect { 1. } else if co > 0. { eta } else { 1. / eta };
    let m = (wi * etap + wo).try_normalize(1e-12)?;
    let m = if m.z < 0. { -m } else { m };
    if m.dot(wi) * ci < 0. || m.dot(wo) * co < 0. {
        return None;
    }
    Some((m, etap, reflect))
}

/// Transmittance through a slab of depth `dz` along `w`, of unit extinction.
fn tr(dz: f64, w: &Vector3<f64>) -> f64 {
    if dz.abs() <= f64::MIN_POSITIVE {
        1.
    } else {
        (-(dz / w.z).abs()).exp()
    }
}

fn exponential(u: f64, a: f64) -> f64 {
    -(1. - u).ln() / a
}

/// Henyey-Greenstein about the direction `wo` came from: p(ω_o, ω_i) for light from ω_i
/// leaving along ω_o, both pointing away (pbrt's convention).
fn phase_p(g: f64, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
    henyey_greenstein(-wo.dot(wi), g)
}

fn phase_sample(g: f64, wo: &Vector3<f64>) -> (Vector3<f64>, f64) {
    // Light continues along -wo, deflected.
    let wi = sample_henyey_greenstein(&-wo, g, rand(), rand());
    (wi, phase_p(g, wo, &wi))
}

fn power(a: f64, b: f64) -> f64 {
    crate::material::bsdf::power_heuristic(a, b)
}

/// Two interfaces with a slab between them, in a local frame: pbrt's `LayeredBxDF`, two-sided.
pub struct Layered {
    pub top: Interface,
    pub bottom: Interface,
    pub thickness: f64,
    /// The slab's single-scattering albedo (black: no medium) and phase asymmetry.
    pub albedo: Color,
    pub g: f64,
    pub max_depth: u32,
    pub samples: u32,
}

impl Layered {
    fn has_medium(&self) -> bool {
        self.albedo.rgb.max() > 0.
    }

    /// An estimate of f(ω_o, ω_i) (PBR's `LayeredBxDF::f`).
    pub fn f(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let (mut wo, mut wi) = (*wo, *wi);
        if wo.z < 0. {
            wo = -wo;
            wi = -wi;
        }
        // Entered at the top; leaves at the top if reflected, the bottom if transmitted.
        let (enter, exit, non_exit) = (&self.top, if same_hemisphere(&wo, &wi) { &self.top } else { &self.bottom },
                                       if same_hemisphere(&wo, &wi) { &self.bottom } else { &self.top });
        let exit_z = if same_hemisphere(&wo, &wi) { self.thickness } else { 0. };
        let n = self.samples as f64;
        let mut f = if same_hemisphere(&wo, &wi) { enter.f(&wo, &wi, Mode::Radiance) * n } else { Color::black() };
        for _ in 0..self.samples {
            let Some(wos) = enter.sample(&wo, rand(), [rand(), rand()], Mode::Radiance, TRANSMISSION) else { continue };
            if wos.f == Color::black() || wos.pdf <= 0. || wos.wi.z == 0. {
                continue;
            }
            let Some(wis) = exit.sample(&wi, rand(), [rand(), rand()], Mode::Importance, TRANSMISSION) else { continue };
            if wis.f == Color::black() || wis.pdf <= 0. || wis.wi.z == 0. {
                continue;
            }
            let mut beta = wos.f * (wos.wi.z.abs() / wos.pdf);
            let mut z = self.thickness;
            let mut w = wos.wi;
            for depth in 0..self.max_depth {
                if depth > 3 && beta.rgb.max() < 0.25 {
                    let q = (1. - beta.rgb.max()).max(0.);
                    if rand() < q {
                        break;
                    }
                    beta = beta / (1. - q);
                }
                if !self.has_medium() {
                    z = if z == self.thickness { 0. } else { self.thickness };
                    beta = beta * tr(self.thickness, &w);
                } else {
                    let dz = exponential(rand(), 1. / w.z.abs());
                    let zp = if w.z > 0. { z + dz } else { z - dz };
                    if zp == z {
                        continue;
                    }
                    if 0. < zp && zp < self.thickness {
                        // Scattering in the slab: towards ω_i through the exit interface.
                        let wt = if exit.is_specular() { 1. } else { power(wis.pdf, phase_pdf(self.g, &-w, &-wis.wi)) };
                        f = f + beta * self.albedo * wis.f * (phase_p(self.g, &-w, &-wis.wi) * wt * tr(zp - exit_z, &wis.wi) / wis.pdf);
                        let (pw, pp) = phase_sample(self.g, &-w);
                        if pp <= 0. || pw.z == 0. {
                            continue;
                        }
                        beta = beta * self.albedo;
                        w = pw;
                        z = zp;
                        if ((z < exit_z && w.z > 0.) || (z > exit_z && w.z < 0.)) && !exit.is_specular() {
                            let f_exit = exit.f(&-w, &wi, Mode::Radiance);
                            if f_exit != Color::black() {
                                // Against next-event estimation's density for this direction:
                                // sampling -w from ω_i, through the exit interface.
                                let exit_pdf = exit.pdf(&wi, &-w, TRANSMISSION);
                                let wt = power(pp, exit_pdf);
                                f = f + beta * f_exit * (tr(zp - exit_z, &pw) * wt);
                            }
                        }
                        continue;
                    }
                    z = zp.clamp(0., self.thickness);
                }
                if z == exit_z {
                    // At the exit interface: reflect back in.
                    let Some(es) = exit.sample(&-w, rand(), [rand(), rand()], Mode::Radiance, REFLECTION) else { break };
                    if es.f == Color::black() || es.pdf <= 0. || es.wi.z == 0. {
                        break;
                    }
                    beta = beta * es.f * (es.wi.z.abs() / es.pdf);
                    w = es.wi;
                } else {
                    // At the other interface: towards ω_i, then sample on.
                    if !non_exit.is_specular() {
                        let wt = if exit.is_specular() { 1. } else { power(wis.pdf, non_exit.pdf(&-w, &-wis.wi, ALL)) };
                        f = f + beta * non_exit.f(&-w, &-wis.wi, Mode::Radiance) * wis.f
                            * (wis.wi.z.abs() * wt * tr(self.thickness, &wis.wi) / wis.pdf);
                    }
                    let Some(bs) = non_exit.sample(&-w, rand(), [rand(), rand()], Mode::Radiance, REFLECTION) else { break };
                    if bs.f == Color::black() || bs.pdf <= 0. || bs.wi.z == 0. {
                        break;
                    }
                    beta = beta * bs.f * (bs.wi.z.abs() / bs.pdf);
                    w = bs.wi;
                    if !exit.is_specular() {
                        let f_exit = exit.f(&-w, &wi, Mode::Radiance);
                        if f_exit != Color::black() {
                            let wt = if non_exit.is_specular() { 1. } else { power(bs.pdf, exit.pdf(&wi, &-w, TRANSMISSION)) };
                            f = f + beta * f_exit * (tr(self.thickness, &bs.wi) * wt);
                        }
                    }
                }
            }
        }
        f / n
    }

    /// A direction, the BSDF times the cosine over the (proportional) density, and whether it
    /// was a delta (PBR's `LayeredBxDF::Sample_f`).
    pub fn sample(&self, wo: &Vector3<f64>) -> Option<(Vector3<f64>, Color, bool)> {
        let flip = wo.z < 0.;
        let wo = if flip { -wo } else { *wo };
        let unflip = |w: Vector3<f64>| if flip { -w } else { w };
        let bs = self.top.sample(&wo, rand(), [rand(), rand()], Mode::Radiance, ALL)?;
        if bs.f == Color::black() || bs.pdf <= 0. || bs.wi.z == 0. {
            return None;
        }
        if !bs.transmission {
            return Some((unflip(bs.wi), bs.f * (bs.wi.z.abs() / bs.pdf), bs.specular));
        }
        let mut w = bs.wi;
        let mut specular = bs.specular;
        let mut f = bs.f * bs.wi.z.abs();
        let mut pdf = bs.pdf;
        let mut z = self.thickness;
        for depth in 0..self.max_depth {
            let rr = f.rgb.max() / pdf;
            if depth > 3 && rr < 0.25 {
                let q = (1. - rr).max(0.);
                if rand() < q {
                    return None;
                }
                pdf *= 1. - q;
            }
            if w.z == 0. {
                return None;
            }
            if self.has_medium() {
                let dz = exponential(rand(), 1. / w.z.abs());
                let zp = if w.z > 0. { z + dz } else { z - dz };
                if zp == z {
                    return None;
                }
                if 0. < zp && zp < self.thickness {
                    let (pw, pp) = phase_sample(self.g, &-w);
                    if pp <= 0. {
                        return None;
                    }
                    // p / pdf = 1 for Henyey-Greenstein sampled exactly.
                    f = f * self.albedo * pp;
                    pdf *= pp;
                    specular = false;
                    w = pw;
                    z = zp;
                    continue;
                }
                z = zp.clamp(0., self.thickness);
            } else {
                z = if z == self.thickness { 0. } else { self.thickness };
                f = f * tr(self.thickness, &w);
            }
            let interface = if z == 0. { &self.bottom } else { &self.top };
            let bs = interface.sample(&-w, rand(), [rand(), rand()], Mode::Radiance, ALL)?;
            if bs.f == Color::black() || bs.pdf <= 0. || bs.wi.z == 0. {
                return None;
            }
            f = f * bs.f;
            pdf *= bs.pdf;
            specular &= bs.specular;
            w = bs.wi;
            if bs.transmission {
                // f |cos θ_i| / pdf, as the path tracer weights a sample.
                return Some((unflip(w), f * (w.z.abs() / pdf), specular));
            }
            f = f * bs.wi.z.abs();
        }
        None
    }

    /// An estimate of the density of `sample`'s non-delta directions (PBR's
    /// `LayeredBxDF::PDF`), mixed with a uniform density so it is never zero. Like pbrt's, it
    /// counts the coat's reflection as if sampled alone, so it overestimates (integrating to
    /// up to about two): good enough for MIS weights, which is all it is for.
    pub fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        let (mut wo, mut wi) = (*wo, *wi);
        if wo.z < 0. {
            wo = -wo;
            wi = -wi;
        }
        let n = self.samples as f64;
        let mut sum = if same_hemisphere(&wo, &wi) { n * self.top.pdf(&wo, &wi, REFLECTION) } else { 0. };
        let ok = |s: &Option<Sample>| s.as_ref().is_some_and(|s| s.f != Color::black() && s.pdf > 0.);
        for _ in 0..self.samples {
            if same_hemisphere(&wo, &wi) {
                let (r_if, t_if) = (&self.bottom, &self.top);
                let wos = t_if.sample(&wo, rand(), [rand(), rand()], Mode::Radiance, TRANSMISSION);
                let wis = t_if.sample(&wi, rand(), [rand(), rand()], Mode::Importance, TRANSMISSION);
                if ok(&wos) && ok(&wis) {
                    let (wos, wis) = (wos.unwrap(), wis.unwrap());
                    if t_if.is_specular() {
                        sum += r_if.pdf(&-wos.wi, &-wis.wi, ALL);
                    } else if let Some(rs) = r_if.sample(&-wos.wi, rand(), [rand(), rand()], Mode::Radiance, ALL).filter(|s| s.f != Color::black() && s.pdf > 0.) {
                        if !r_if.is_specular() {
                            sum += t_if.pdf(&-rs.wi, &wi, ALL);
                        } else {
                            let r_pdf = r_if.pdf(&-wos.wi, &-wis.wi, ALL);
                            sum += power(wis.pdf, r_pdf) * r_pdf;
                            let t_pdf = t_if.pdf(&-rs.wi, &wi, ALL);
                            sum += power(rs.pdf, t_pdf) * t_pdf;
                        }
                    }
                }
            } else {
                let (to_if, ti_if) = (&self.top, &self.bottom);
                let wos = to_if.sample(&wo, rand(), [rand(), rand()], Mode::Radiance, ALL).filter(|s| s.transmission);
                let wis = ti_if.sample(&wi, rand(), [rand(), rand()], Mode::Importance, ALL).filter(|s| s.transmission);
                if ok(&wos) && ok(&wis) {
                    let (wos, wis) = (wos.unwrap(), wis.unwrap());
                    if to_if.is_specular() {
                        sum += ti_if.pdf(&-wos.wi, &wi, ALL);
                    } else if ti_if.is_specular() {
                        sum += to_if.pdf(&wo, &-wis.wi, ALL);
                    } else {
                        sum += (to_if.pdf(&wo, &-wis.wi, ALL) + ti_if.pdf(&-wos.wi, &wi, ALL)) / 2.;
                    }
                }
            }
        }
        0.1 / (4. * PI) + 0.9 * sum / n
    }
}

fn phase_pdf(g: f64, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
    phase_p(g, wo, wi)
}

/// A layered BSDF in world space, about `normal` (on the side the light arrives from).
pub struct LayeredBSDF {
    pub layered: Layered,
    pub normal: Vector3<f64>,
    t: Vector3<f64>,
    b: Vector3<f64>,
}

impl LayeredBSDF {
    pub fn new(layered: Layered, normal: Vector3<f64>) -> LayeredBSDF {
        let (t, b) = crate::material::phase::orthonormal_basis(&normal);
        LayeredBSDF { layered, normal, t, b }
    }

    fn local(&self, v: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(v.dot(&self.t), v.dot(&self.b), v.dot(&self.normal))
    }

    fn world(&self, v: &Vector3<f64>) -> Vector3<f64> {
        self.t * v.x + self.b * v.y + self.normal * v.z
    }

    /// A direction, weight (f cos / pdf) and whether it is a delta.
    pub fn sample_layered(&self, wo: &Vector3<f64>) -> Option<(Vector3<f64>, Color, bool)> {
        self.layered.sample(&self.local(wo)).map(|(wi, w, spec)| (self.world(&wi), w, spec))
    }
}

impl BSDF for LayeredBSDF {
    fn eval(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> Color {
        let i = self.local(wi);
        self.layered.f(&self.local(wo), &i) * i.z.abs()
    }

    fn pdf(&self, wo: &Vector3<f64>, wi: &Vector3<f64>) -> f64 {
        self.layered.pdf(&self.local(wo), &self.local(wi))
    }

    /// Non-delta samples only (a delta has no density for MIS); the material uses
    /// `sample_layered` for both.
    fn sample(&self, wo: &Vector3<f64>, _u: [f64; 3]) -> Option<BSDFSample> {
        let (wi, weight, specular) = self.sample_layered(wo)?;
        if specular {
            return None;
        }
        Some(BSDFSample { wi, weight, pdf: self.pdf(wo, &wi) })
    }
}

/// What lies under a coat.
pub enum Base {
    Diffuse(crate::textures::ColorTexture),
    /// A conductor (its index relative to air: divided by the coat's index under it), and its
    /// perceptual roughness.
    Conductor { optics: crate::material::conductor::Optics, roughness: f64 },
}

/// pbrt's `CoatedDiffuseMaterial` and `CoatedConductorMaterial`: a base under a dielectric
/// coat of index `refractive_index` and perceptual roughness `roughness` (α = r²), `thickness`
/// thick, with a medium of `albedo` (black for none) and asymmetry `g` in it. The walk between
/// the layers stops after `max_depth` bounces (pbrt's default, 10, loses about 7% of a white
/// base's light to total internal reflection; the coat's unit extinction absorbs a little
/// more at the default thickness, 0.01).
pub struct Coated {
    pub base: Base,
    pub refractive_index: f64,
    pub roughness: f64,
    pub thickness: f64,
    pub albedo: Color,
    pub g: f64,
    pub max_depth: u32,
    pub samples: u32,
}

impl crate::material::model::MaterialModel for Coated {
    fn scatter(&self, r: &crate::ray::Ray, intersection: &crate::intersection::Intersection, s: &crate::scene::Scene) -> crate::material::model::ScatteredRay {
        use crate::material::ggx::regularised;
        use crate::material::model::{LightSampling, ScatteredRay};
        let i = intersection.facing(r);
        let bottom = match &self.base {
            Base::Diffuse(t) => Interface::Diffuse { r: t.eval(&i, s).as_reflectance() },
            Base::Conductor { optics, roughness } => {
                let (eta, k) = optics.current();
                Interface::Conductor { eta: eta / self.refractive_index, k: k / self.refractive_index,
                                       alpha: regularised(roughness * roughness, s) }
            },
        };
        let layered = Layered {
            top: Interface::Dielectric { eta: self.refractive_index, alpha: regularised(self.roughness * self.roughness, s) },
            bottom,
            thickness: self.thickness,
            albedo: self.albedo.as_reflectance(),
            g: self.g,
            max_depth: self.max_depth,
            samples: self.samples,
        };
        let bsdf = LayeredBSDF::new(layered, i.normal);
        let wo = -r.rd.normalize();
        let emit = crate::material::functions::direct_lighting_mis(&bsdf, &i.point, &wo, s);
        crate::material::ggx::scattered();
        match bsdf.sample_layered(&wo) {
            Some((wi, weight, specular)) => ScatteredRay {
                emit,
                attenuate: weight,
                ray: Some(crate::ray::Ray { ro: i.point, rd: wi }),
                // A delta (the smooth coat's mirror) isn't sampled as light, so counts in full.
                light_sampling: if specular { LightSampling::None } else { LightSampling::MIS { bsdf_pdf: bsdf.pdf(&wo, &wi) } },
            },
            None => ScatteredRay { emit, attenuate: Color::black(), ray: None, light_sampling: LightSampling::MIS { bsdf_pdf: 0. } },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::seed_rand;

    fn coated_diffuse(r: f64, roughness: f64) -> Layered {
        Layered {
            top: Interface::Dielectric { eta: 1.5, alpha: roughness * roughness },
            bottom: Interface::Diffuse { r: Color::white() * r },
            thickness: 0.01, albedo: Color::black(), g: 0., max_depth: 10, samples: 1,
        }
    }

    /// The directional albedo from `wo`: by sampling (mean weight) and by eval / uniform pdf.
    fn albedos(l: &Layered, wo: &Vector3<f64>, n: usize) -> (f64, f64) {
        let sampled = (0..n).filter_map(|_| l.sample(wo)).map(|(_, w, _)| w.rgb.x).sum::<f64>() / n as f64;
        let evaluated = (0..n).map(|_| {
            let d = crate::geometry::point_on_unit_sphere(rand(), rand());
            l.f(wo, &d).rgb.x * d.z.abs() * 4. * PI
        }).sum::<f64>() / n as f64;
        (sampled, evaluated)
    }

    #[test]
    fn test_interfaces_sample_consistently() {
        // Each rough interface's sample weight equals f |cos| / pdf, and a sample's pdf its
        // stated density.
        seed_rand(1, 0);
        let interfaces = [
            Interface::Dielectric { eta: 1.5, alpha: 0.3 },
            Interface::Diffuse { r: Color::white() * 0.5 },
            Interface::Conductor { eta: Color::white() * 0.2, k: Color::white() * 3., alpha: 0.2 },
        ];
        for i in &interfaces {
            for wo in [Vector3::new(0.3, 0.1, 0.9).normalize(), Vector3::new(0.5, -0.2, -0.6).normalize()] {
                for _ in 0..2000 {
                    if let Some(s) = i.sample(&wo, rand(), [rand(), rand()], Mode::Radiance, ALL) {
                        let f = i.f(&wo, &s.wi, Mode::Radiance);
                        let pdf = i.pdf(&wo, &s.wi, ALL);
                        assert!((f.rgb.x - s.f.rgb.x).abs() < 1e-9 * (1. + f.rgb.x), "{:?} vs {:?}", f, s.f);
                        assert!((pdf - s.pdf).abs() < 1e-9 * (1. + pdf), "{} vs {}", pdf, s.pdf);
                    }
                }
            }
        }
    }

    #[test]
    fn test_coated_white_diffuse_conserves_energy() {
        // A white base under a clear, infinitely thin coat loses nothing: whatever enters
        // leaves, by sampling, and eval integrates to the same. (The coat has unit extinction,
        // as pbrt's, so any thickness absorbs: 0.01, the default, loses 8% here, mostly to light
        // trapped by total internal reflection.)
        seed_rand(2, 0);
        for roughness in [0., 0.4] {
            let l = Layered { max_depth: 100, thickness: 1e-6, ..coated_diffuse(1., roughness) };
            let wo = Vector3::new(0.4, 0., 0.8).normalize();
            let (sampled, evaluated) = albedos(&l, &wo, 200_000);
            // A rough coat's microfacets scatter once (as pbrt's), losing what a facet sends
            // into another, most from inside where light hits it many times: 15% at α = 0.16.
            let least = if roughness == 0. { 0.97 } else { 0.8 };
            assert!(sampled > least && sampled <= 1.01, "roughness {}: sampled {}", roughness, sampled);
            // A smooth coat's mirror reflection is a delta, which eval leaves out.
            let delta = if roughness == 0. { fresnel_dielectric(wo.z, 1.5) } else { 0. };
            assert!((evaluated + delta - sampled).abs() < 0.03, "roughness {}: eval {} + {} vs {}", roughness, evaluated, delta, sampled);
        }
    }

    #[test]
    fn test_coat_darkens_and_saturates_the_base() {
        // Light bouncing between coat and base meets the base several times: a coated base
        // looks darker than its albedo (beside the coat's own reflection, which is grey).
        seed_rand(3, 0);
        let wo = Vector3::new(0., 0., 1.);
        let n = 200_000;
        let body = |r: f64| {
            let l = coated_diffuse(r, 0.);
            (0..n).filter_map(|_| l.sample(&wo)).filter(|s| !s.2).map(|(_, w, _)| w.rgb.x).sum::<f64>() / n as f64
        };
        let (half, white) = (body(0.5), body(1.));
        assert!(half / white < 0.5 && half / white > 0.25, "{} / {}", half, white);
    }

    /// Not a true density: pbrt's estimate adds the coat's reflection-only density to the
    /// light through it, so it integrates to more than one (nearly two for a rough coat). It
    /// is only used for MIS weights, where an overestimate shifts weight to light sampling.
    #[test]
    fn test_layered_pdf_is_positive_and_bounded() {
        seed_rand(4, 0);
        let l = coated_diffuse(0.7, 0.3);
        let wo = Vector3::new(0.2, 0.3, 0.9).normalize();
        let n = 200_000;
        let total = (0..n).map(|_| {
            let d = crate::geometry::point_on_unit_sphere(rand(), rand());
            l.pdf(&wo, &d) * 4. * PI
        }).sum::<f64>() / n as f64;
        assert!(total > 0.9 && total < 2.1, "{}", total);
    }

    #[test]
    fn test_absorbing_coat_tints() {
        // A coat with a reddish medium: blue absorbed, red kept.
        seed_rand(5, 0);
        let l = Layered { thickness: 0.5, albedo: Color::new(0.95, 0.5, 0.2), g: 0.3, ..coated_diffuse(1., 0.2) };
        let wo = Vector3::new(0., 0., 1.);
        let n = 100_000;
        let mean = (0..n).filter_map(|_| l.sample(&wo)).filter(|s| !s.2).map(|(_, w, _)| w.rgb).sum::<Vector3<f64>>() / n as f64;
        assert!(mean.x > mean.y && mean.y > mean.z, "{:?}", mean);
    }
}
