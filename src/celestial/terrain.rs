//! The faces of rocky and icy worlds other than our Moon: what, besides craters and maria
//! (`surface`), colours them.
//!
//! - **Mercury** is the Moon's twin in look, greyer and darker, its smooth plains less dark
//!   than maria, with many bright rayed craters (normal albedo about 0.1: Domingue et al.,
//!   "Mercury's spectrophotometric properties", Space Sci. Rev. 181, 2014).
//! - **Mars**: bright ochre dust and dark basaltic regions (the classical albedo features),
//!   craters fewer and worn, white polar caps of water and CO₂ ice (normal albedos about 0.3 and
//!   0.15 in red light).
//! - **Callisto**: the most cratered surface known, dark (0.2) dirty ice under a lag of dust,
//!   with bright fresh craters where clean ice was dug up.
//! - **Ganymede**: dark ancient terrain cut by bright lanes of younger grooved terrain
//!   (*sulci*), fewer craters on the bright.
//! - **Europa**: almost uncratered bright water ice (0.6), crossed by the reddish-brown lines
//!   (*lineae*) of its cracked shell, curving along small circles as tidal stresses break it,
//!   and blotched with brown chaos terrain.
//! - **Io**: no craters at all, its volcanoes resurfacing it faster than they form: yellows and
//!   oranges of sulfur, white sulfur dioxide frost, dark lava lakes (*paterae*) and red rings of
//!   plume fallout (Pele's), with darker red-brown poles.
//!
//! Each pattern is generated from the body's seed, so any number of worlds of each kind can be
//! made; `Terrain::rocky` and `Terrain::icy` choose among the kinds and their parameters from
//! a seed alone.

use crate::color::Color;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::procedural::space::{mix, smoothstep};

fn unit(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

fn direction(h: u64) -> Vector3<f64> {
    let (u, v) = (unit(mix(h ^ 0x1)), unit(mix(h ^ 0x2)));
    let z = 1. - 2. * u;
    let r = (1. - z * z).max(0.).sqrt();
    let phi = 2. * std::f64::consts::PI * v;
    Vector3::new(r * phi.cos(), r * phi.sin(), z)
}

/// An arc of a small circle on the unit sphere: a lineament.
#[derive(Debug, Clone, Copy)]
pub struct Arc {
    pub axis: Vector3<f64>,
    /// Angular radius of its circle, and its half-width, radians.
    pub radius: f64,
    pub width: f64,
    /// The arc's start and length, radians round the axis.
    pub start: f64,
    pub length: f64,
    /// Basis about the axis.
    u: Vector3<f64>,
    v: Vector3<f64>,
    pub strength: f64,
}

impl Arc {
    fn new(h: u64, width: f64) -> Arc {
        let axis = direction(h);
        let (u, v) = crate::procedural::space::tangent_basis(&axis);
        Arc {
            axis, u, v,
            radius: 0.4 + 1.1 * unit(mix(h ^ 3)),
            width: width * (0.4 + unit(mix(h ^ 4))),
            start: unit(mix(h ^ 5)) * 2. * std::f64::consts::PI,
            length: 0.3 + 2. * unit(mix(h ^ 6)),
            strength: 0.4 + 0.6 * unit(mix(h ^ 7)),
        }
    }

    /// How much of the line covers `p`, 0 to 1.
    fn at(&self, p: &Vector3<f64>) -> f64 {
        let cos = p.dot(&self.axis);
        let off = cos.clamp(-1., 1.).acos() - self.radius;
        if off.abs() > self.width * 3. {
            return 0.;
        }
        let angle = (p.dot(&self.v).atan2(p.dot(&self.u)) - self.start).rem_euclid(2. * std::f64::consts::PI);
        if angle > self.length {
            return 0.;
        }
        // Tapering at the ends.
        let taper = smoothstep(0., 0.1, angle) * smoothstep(0., 0.1, self.length - angle);
        // A double ridge: two dark lines either side of a lighter centre.
        let x = off / self.width;
        ((-(x.abs() - 0.6).powi(2) * 5.).exp() * 0.8 + (-x * x).exp() * 0.4).min(1.) * taper * self.strength
    }
}

/// A dark spot: a patera, perhaps ringed by plume fallout.
#[derive(Debug, Clone, Copy)]
pub struct Spot {
    pub centre: Vector3<f64>,
    /// Radius of the dark patch, and of its ring (0 for none), radians.
    pub radius: f64,
    pub ring: f64,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    /// Craters and maria alone.
    None,
    /// Mars: dark regions and polar caps.
    Mars { dark: Color, dust: Color, cap: Color, cap_latitude: f64 },
    /// Ganymede: bright grooved lanes over dark terrain.
    Grooved { bright: Color, share: f64 },
    /// Europa: lineae and chaos.
    Lineae { arcs: Vec<Arc>, line: Color, chaos: f64 },
    /// Io: sulfur colours, paterae and plume rings.
    Volcanic { spots: Vec<Spot>, colours: [Color; 4] },
}

#[derive(Clone)]
pub struct Terrain {
    pub pattern: Pattern,
    noise: SeededNoise,
}

impl std::fmt::Debug for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Terrain").field("pattern", &self.pattern).finish()
    }
}

impl Terrain {
    pub fn new(pattern: Pattern, seed: u64) -> Terrain {
        Terrain { pattern, noise: SeededNoise::new(mix(seed ^ 0x7465_7272)) }
    }

    /// The normal albedo at unit `p` (body frame, y the pole), given the cratered surface's
    /// own (`base`) and how much of a mare it is.
    pub fn albedo(&self, p: &Vector3<f64>, base: Color, mare: f64) -> Color {
        let n = &self.noise;
        match &self.pattern {
            Pattern::None => base,
            Pattern::Mars { dark, dust, cap, cap_latitude } => {
                // Dark regions: large-scale, sharp-edged, streaked by the winds.
                let regions = n.fbm(p * 2.2, 6) + 0.25 * n.fbm(Vector3::new(p.x * 9., p.y * 3., p.z * 9.), 3);
                let d = smoothstep(0.05, 0.25, regions) * 0.9 + mare * 0.5;
                let ground = *dust * (1. - d.min(1.)) + *dark * d.min(1.);
                // Craters' own variation, around the ground's.
                let mut c = ground * (base.rgb.y / 0.12).clamp(0.6, 1.6);
                let lat = p.y.clamp(-1., 1.).asin().abs();
                let cap_edge = cap_latitude + 0.06 * n.fbm(p * 12., 4);
                let ice = smoothstep(cap_edge - 0.02, cap_edge + 0.02, lat);
                c = c * (1. - ice) + *cap * ice;
                c
            },
            Pattern::Grooved { bright, share } => {
                let lanes = n.fbm(p * 2.5, 5) + 0.15 * n.fbm(p * 20., 3);
                let b = smoothstep(-0.05, 0.05, lanes - (0.5 - share) * 0.6);
                // Grooves: fine parallel ridges along the lanes.
                let grooves = 0.9 + 0.1 * (n.fbm(p * 3., 2) * 300.).sin();
                base * (1. - b) + *bright * (b * grooves * (base.rgb.y / 0.2).clamp(0.8, 1.3))
            },
            Pattern::Lineae { arcs, line, chaos } => {
                let mut c = base;
                // Chaos terrain and reddish staining.
                let ch = smoothstep(0.25, 0.45, n.fbm(p * 4., 5)) * chaos;
                c = c * (1. - ch * 0.6) + *line * (ch * 0.6);
                let mut l: f64 = 0.;
                for a in arcs {
                    l = l.max(a.at(p));
                }
                c * (1. - l) + *line * l
            },
            Pattern::Volcanic { spots, colours } => {
                let m1 = n.fbm(p * 3., 6);
                let m2 = n.fbm(p * 7. + Vector3::new(3.3, 1.1, 8.2), 5);
                let mut c = colours[0] * (1. - smoothstep(-0.2, 0.3, m1)) + colours[1] * smoothstep(-0.2, 0.3, m1);
                let frost = smoothstep(0.15, 0.4, m2);
                c = c * (1. - frost) + colours[2] * frost;
                // Darker red-brown towards the poles.
                let polar = smoothstep(0.5, 0.85, p.y.abs() + 0.1 * m1);
                c = c * (1. - polar) + colours[3] * polar;
                for s in spots {
                    let cos = p.dot(&s.centre);
                    let reach = s.radius.max(s.ring) * 1.3;
                    if cos < reach.cos() {
                        continue;
                    }
                    let d = cos.clamp(-1., 1.).acos();
                    let rough = 1. + 0.3 * n.noise(p * (3. / s.radius));
                    if s.ring > 0. {
                        let r = (-(((d - s.ring) / (s.ring * 0.12)).powi(2))).exp() * 0.8;
                        c = c * (1. - r) + Color::new(0.62, 0.30, 0.18) * r;
                    }
                    let dark = 1. - smoothstep(0.7, 1., d / s.radius * rough);
                    c = c * (1. - dark) + Color::new(0.06, 0.05, 0.045) * dark;
                }
                c
            },
        }
    }
}

/// A rocky or icy world's settings: its surface, colours and pattern.
#[derive(Debug, Clone)]
pub struct World {
    /// Radius, metres.
    pub radius: f64,
    pub params: crate::celestial::surface::SurfaceParams,
    pub highlands: Color,
    pub maria: Color,
    pub pattern: Pattern,
}

impl World {
    /// A named world (`moon`, `mercury`, `mars`, `callisto`, `ganymede`, `europa`, `io`), or one
    /// from `seed` (`rocky` or `icy`).
    pub fn named(name: &str, seed: u64) -> World {
        use crate::celestial::surface::SurfaceParams;
        let d = SurfaceParams { seed, ..SurfaceParams::default() };
        let none = |p: SurfaceParams| SurfaceParams { crater_density: 0., basins: 0, maria: 0., rayed_craters: 0, max_crater: 1., min_crater: 1., ..p };
        match name {
            "moon" => World { radius: crate::celestial::MOON_RADIUS, params: d, highlands: Color::new(0.14, 0.13, 0.112), maria: Color::new(0.083, 0.078, 0.069), pattern: Pattern::None },
            "mercury" => World {
                radius: 2_439_700.,
                params: SurfaceParams { radius: 2439.7, maria: 0.35, rayed_craters: 30, ray_brightness: 0.7, fresh_brightness: 0.5, ..d },
                highlands: Color::new(0.115, 0.108, 0.098), maria: Color::new(0.105, 0.1, 0.092), pattern: Pattern::None,
            },
            "mars" => World {
                radius: 3_389_500.,
                params: SurfaceParams { radius: 3389.5, crater_density: 0.012, maria: 0.3, rayed_craters: 0, basins: 12, fresh_brightness: 0.1, ..d },
                highlands: Color::new(0.12, 0.12, 0.12), maria: Color::new(0.1, 0.1, 0.1),
                pattern: Pattern::Mars { dust: Color::new(0.36, 0.21, 0.12), dark: Color::new(0.15, 0.11, 0.085), cap: Color::new(0.72, 0.70, 0.67), cap_latitude: 1.32 },
            },
            "callisto" => World {
                radius: 2_410_300.,
                params: SurfaceParams { radius: 2410.3, crater_density: 0.09, maria: 0., rayed_craters: 25, ray_brightness: 1.2, fresh_brightness: 1.5, ..d },
                highlands: Color::new(0.21, 0.19, 0.165), maria: Color::new(0.2, 0.18, 0.16), pattern: Pattern::None,
            },
            "ganymede" => World {
                radius: 2_634_100.,
                params: SurfaceParams { radius: 2634.1, crater_density: 0.04, maria: 0., rayed_craters: 20, ray_brightness: 1., fresh_brightness: 1.2, ..d },
                highlands: Color::new(0.24, 0.22, 0.2), maria: Color::new(0.24, 0.22, 0.2),
                pattern: Pattern::Grooved { bright: Color::new(0.47, 0.46, 0.45), share: 0.55 },
            },
            "europa" => World {
                radius: 1_560_800.,
                params: SurfaceParams { radius: 1560.8, crater_density: 0.0008, basins: 0, maria: 0., rayed_craters: 2, ..d },
                highlands: Color::new(0.68, 0.64, 0.58), maria: Color::new(0.68, 0.64, 0.58),
                pattern: lineae(seed, 160, 0.004, Color::new(0.42, 0.30, 0.22), 0.6),
            },
            "io" => World {
                radius: 1_821_600.,
                params: SurfaceParams { radius: 1821.6, ..none(d) },
                highlands: Color::new(0.7, 0.62, 0.35), maria: Color::new(0.7, 0.62, 0.35),
                pattern: volcanic(seed, [Color::new(0.80, 0.72, 0.36), Color::new(0.78, 0.55, 0.28), Color::new(0.86, 0.84, 0.74), Color::new(0.48, 0.36, 0.28)]),
            },
            "rocky" => World::rocky(seed),
            "icy" => World::icy(seed),
            _ => panic!("Unknown world {:?}: moon, mercury, mars, callisto, ganymede, europa, io, rocky or icy", name),
        }
    }

    /// A rocky world from `seed`: an airless cratered body like the Moon or Mercury, or a
    /// dusty red one like Mars, or a volcanic one like Io; its size, colours, craters and
    /// plains drawn from the seed.
    pub fn rocky(seed: u64) -> World {
        use crate::celestial::surface::SurfaceParams;
        let r = |k: u64| unit(mix(mix(seed ^ 0x726f_636b) ^ mix(k)));
        let radius_km = 500. + 3500. * r(1).powi(2);
        let tint = Color::new(1. + 0.25 * r(2), 1., 1. - 0.2 * r(3));
        let brightness = 0.08 + 0.12 * r(4);
        let p = SurfaceParams {
            seed, radius: radius_km, crater_density: 0.01 + 0.08 * r(5), maria: 1.5 * r(6).powi(2),
            basins: (5. + 30. * r(7)) as usize, rayed_craters: (25. * r(8)) as usize, ..SurfaceParams::default()
        };
        let kind = r(9);
        let (pattern, params) = if kind < 0.6 {
            (Pattern::None, p)
        } else if kind < 0.85 {
            let dust = Color::new(0.30 + 0.1 * r(10), 0.17 + 0.08 * r(11), 0.10 + 0.05 * r(12));
            (Pattern::Mars { dust, dark: dust * (0.35 + 0.25 * r(13)), cap: Color::new(0.72, 0.70, 0.67), cap_latitude: 1.1 + 0.4 * r(14) },
             SurfaceParams { crater_density: p.crater_density * 0.2, fresh_brightness: 0.1, rayed_craters: 0, ..p })
        } else {
            (volcanic(seed, [Color::new(0.8, 0.7, 0.35), Color::new(0.75, 0.5 + 0.1 * r(15), 0.28), Color::new(0.85, 0.83, 0.75), Color::new(0.45, 0.35, 0.28)]),
             SurfaceParams { crater_density: 0., basins: 0, maria: 0., rayed_craters: 0, max_crater: 1., min_crater: 1., ..p })
        };
        World { radius: radius_km * 1e3, params, highlands: tint * brightness, maria: tint * (brightness * (0.55 + 0.3 * r(16))), pattern }
    }

    /// An icy world from `seed`: old and dark and saturated with craters like Callisto,
    /// grooved like Ganymede, or young and cracked like Europa.
    pub fn icy(seed: u64) -> World {
        use crate::celestial::surface::SurfaceParams;
        let r = |k: u64| unit(mix(mix(seed ^ 0x6963_7920) ^ mix(k)));
        let radius_km = 200. + 2500. * r(1).powf(1.5);
        let age = r(2);
        let grey = 0.18 + 0.5 * (1. - age);
        let ice = Color::new(grey, grey * (0.95 - 0.05 * r(3)), grey * (0.88 - 0.1 * r(4)));
        let p = SurfaceParams {
            seed, radius: radius_km, crater_density: 0.002 + 0.09 * age.powi(2), maria: 0., basins: (20. * age) as usize,
            rayed_craters: (20. * r(5)) as usize, ray_brightness: 1., fresh_brightness: 1.2, ..SurfaceParams::default()
        };
        let pattern = if age < 0.35 {
            lineae(seed, 60 + (150. * r(6)) as usize, 0.003 + 0.004 * r(7), Color::new(0.4 + 0.1 * r(8), 0.3, 0.22), r(9))
        } else if age < 0.7 && r(10) < 0.6 {
            Pattern::Grooved { bright: ice * 1.8, share: 0.3 + 0.4 * r(11) }
        } else {
            Pattern::None
        };
        World { radius: radius_km * 1e3, params: p, highlands: ice, maria: ice, pattern }
    }
}

fn lineae(seed: u64, n: usize, width: f64, line: Color, chaos: f64) -> Pattern {
    let arcs = (0..n).map(|i| Arc::new(mix(seed ^ mix(0x6c69_6e65 + i as u64)), width)).collect();
    Pattern::Lineae { arcs, line, chaos }
}

fn volcanic(seed: u64, colours: [Color; 4]) -> Pattern {
    let spots = (0..90).map(|i| {
        let h = mix(seed ^ mix(0x7061_7465 + i as u64));
        let radius = 0.004 + 0.03 * unit(mix(h ^ 9)).powi(3);
        let ring = if unit(mix(h ^ 10)) < 0.06 { 0.15 + 0.15 * unit(mix(h ^ 11)) } else { 0. };
        Spot { centre: direction(h), radius, ring }
    }).collect();
    Pattern::Volcanic { spots, colours }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_europa_has_lines() {
        let w = World::named("europa", 3);
        let t = Terrain::new(w.pattern.clone(), 3);
        let base = w.highlands;
        // Somewhere across a thousand points, the lines darken the ice.
        let darkest = crate::procedural::space::fibonacci_sphere(4000)
            .map(|p| crate::color::luminance(&t.albedo(&p, base, 0.)))
            .fold(f64::INFINITY, f64::min);
        assert!(darkest < 0.8 * crate::color::luminance(&base), "{}", darkest);
    }

    #[test]
    fn test_mars_has_caps() {
        let w = World::named("mars", 1);
        let t = Terrain::new(w.pattern.clone(), 1);
        let pole = t.albedo(&Vector3::y(), w.highlands, 0.);
        let equator = t.albedo(&Vector3::x(), w.highlands, 0.);
        assert!(pole.rgb.z > 2. * equator.rgb.z, "{:?} {:?}", pole.rgb, equator.rgb);
    }

    #[test]
    fn test_generated_worlds_vary() {
        let sizes: Vec<f64> = (0..10).map(|s| World::rocky(s).radius).collect();
        assert!(sizes.iter().any(|&r| r != sizes[0]));
        for s in 0..10 {
            let w = World::icy(s);
            assert!(w.radius > 1e5 && w.highlands.rgb.max() < 0.8);
        }
    }
}
