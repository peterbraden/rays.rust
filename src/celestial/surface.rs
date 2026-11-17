//! A cratered surface, generated from a seed: the face of a moon or any airless rocky body.
//!
//! Such a surface is shaped almost wholly by impacts. Here it is the sum of their craters,
//! over every size from the biggest basins to what a pixel can resolve, plus the lava that
//! flooded the lowest of them, and is described by a height (above the mean sphere, in km), its
//! slope, and an albedo.
//!
//! - **How many craters of each size.** Old surfaces are saturated: craters of every size
//!   overlap and erase each other until their number per area stops growing, at N(>D) ≈ K D⁻²
//!   (D in km, per km²), a few per cent of the geometric limit at which they would touch,
//!   1.54 D⁻² (Gault, "Saturation and equilibrium conditions for impact cratering on the lunar
//!   surface", Radio Science 5(2), 1970; Hartmann, "Lunar cratering chronology", Icarus 13,
//!   1970). They are generated octave by octave in diameter, each octave in a grid of cells over
//!   the sphere, each cell holding a Poisson number of them, so any point needs only its
//!   neighbourhood's.
//! - **Their shape.** Pike's measurements of lunar craters ("Depth/diameter relations of fresh
//!   lunar craters: revision from spacecraft data", GRL 1(7), 1974, and "Size-dependence in the
//!   shape of fresh impact craters on the Moon", 1977): simple bowls below about 15 km, depth
//!   0.196 D^1.01 and rim 0.036 D^1.014; above it complex craters, flat-floored (a floor
//!   0.187 D^1.249 across), with central peaks, shallower, depth 1.044 D^0.301 and rim
//!   0.236 D^0.399. Outside the rim the ejecta blanket thins as (r / R)⁻³ (McGetchin, Settle
//!   and Head, "Radial thickness variation in impact crater ejecta", EPSL 20, 1973). Older
//!   craters are worn shallower and their rims lower.
//! - **Basins and maria.** The largest impacts left basins hundreds of kilometres across. On a
//!   tidally locked moon, the side facing its planet has the thinner crust, and lava welled up
//!   to fill its basins and a great irregular lowland (Oceanus Procellarum on ours): the dark
//!   maria, smooth and younger, with a third as many craters, which buried the craters that were
//!   there before them.
//! - **Albedo.** Maria are darker than highlands (normal albedos about 0.07 and 0.12 against the
//!   highlands' 0.11-0.18 for the Moon: Lucey et al., "Understanding the lunar surface and
//!   space-Moon interactions", Rev. Mineral. Geochem. 60, 2006), fresh craters brighter until
//!   space weathering darkens them, and the youngest large craters throw bright rays across the
//!   surface for hundreds of kilometres (Tycho, Copernicus).

use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::procedural::space::{mix, smoothstep, tangent_basis};

/// Fresh simple craters' Pike constants: depth and rim height in km for diameter D in km, and
/// the diameter (km) above which they are complex.
const SIMPLE_TO_COMPLEX: f64 = 15.;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crater {
    /// Unit direction of its centre, in the body's frame.
    pub centre: Vector3<f64>,
    /// Radius, radians of arc.
    pub radius: f64,
    /// Rim to floor, km.
    pub depth: f64,
    /// Rim above the surroundings, km.
    pub rim: f64,
    /// The flat floor's share of the radius.
    pub floor: f64,
    /// Central peak, km.
    pub peak: f64,
    /// 1 fresh, 0 worn away.
    pub freshness: f64,
}

/// How far a crater reaches, in its radii: its ejecta blanket fades out by here.
const REACH: f64 = 3.;
/// How far a young crater's rays reach, in its radii.
const RAY_REACH: f64 = 40.;

impl Crater {
    /// A crater `diameter` km across on a body `body_radius` km in radius, by Pike's relations
    /// (see the module docs), worn down as it ages from `freshness` 1 to 0.
    pub fn new(centre: Vector3<f64>, diameter: f64, body_radius: f64, freshness: f64) -> Crater {
        let d = diameter;
        let (depth, rim, floor, peak) = if d < SIMPLE_TO_COMPLEX {
            (0.196 * d.powf(1.01), 0.036 * d.powf(1.014), 0., 0.)
        } else {
            let depth = 1.044 * d.powf(0.301);
            // Central peaks rise with size, then give way to rings in the largest.
            let peak = depth * 0.45 * smoothstep(15., 40., d) * (1. - smoothstep(150., 300., d));
            (depth, 0.236 * d.powf(0.399), (0.187 * d.powf(1.249) / d).min(0.7), peak)
        };
        let worn = |keep: f64| keep + (1. - keep) * freshness;
        Crater {
            centre, radius: diameter / 2. / body_radius,
            depth: depth * worn(0.25), rim: rim * worn(0.15), floor, peak: peak * worn(0.3), freshness,
        }
    }

    /// The crater's height (km) at `x` radii from its centre, and its derivative in x.
    pub fn profile(&self, x: f64) -> (f64, f64) {
        if x >= REACH {
            return (0., 0.);
        }
        if x < 1. {
            let (mut h, mut dh) = if x < self.floor {
                (self.rim - self.depth, 0.)
            } else {
                let t = (x - self.floor) / (1. - self.floor);
                (self.rim - self.depth * (1. - t * t), self.depth * 2. * t / (1. - self.floor))
            };
            if self.peak > 0. {
                let s = 0.15;
                let e = (-(x / s).powi(2)).exp();
                h += self.peak * e;
                dh += self.peak * e * (-2. * x / (s * s));
            }
            (h, dh)
        } else {
            // The ejecta blanket, thinning as x⁻³, faded to nothing at REACH.
            let taper = 1. - smoothstep(REACH - 1., REACH, x);
            let dtaper = if x > REACH - 1. { let t = x - (REACH - 1.); -6. * t * (1. - t) } else { 0. };
            let e = x.powi(-3);
            (self.rim * e * taper, self.rim * (-3. * e / x * taper + e * dtaper))
        }
    }

    /// Its height and slope at unit direction `p` (the slope as a tangent vector, km per km),
    /// and how far out `p` is, in radii; None if out of reach.
    fn at(&self, p: &Vector3<f64>, body_radius: f64) -> Option<(f64, Vector3<f64>, f64)> {
        let cos = p.dot(&self.centre);
        // Out of reach (cos falls with angle).
        if cos < (REACH * self.radius).min(std::f64::consts::PI).cos() {
            return None;
        }
        let sin = p.cross(&self.centre).norm();
        let x = sin.atan2(cos) / self.radius;
        let (h, dh) = self.profile(x);
        // Away from the centre, along the surface.
        let away = p * cos - self.centre;
        let n = away.norm();
        let slope = if n > 1e-12 { away * (dh / (self.radius * body_radius) / n) } else { Vector3::zeros() };
        Some((h, slope, x))
    }
}

/// A young crater's rays: bright streaks of ejecta reaching out many radii.
#[derive(Debug, Clone)]
struct RaySystem {
    crater: Crater,
    /// The crater's tangent frame, from which the rays' azimuths are measured.
    frame: (Vector3<f64>, Vector3<f64>),
    /// Each ray's azimuth (radians), length and half-width (in the crater's radii).
    rays: Vec<(f64, f64, f64)>,
}

impl RaySystem {
    /// How bright the rays are at `p`, `x` radii from the centre: 0 to about 1.
    fn brightness(&self, p: &Vector3<f64>, x: f64) -> f64 {
        if x < 0.8 {
            return 0.;
        }
        let d = p - self.crater.centre;
        let azimuth = d.dot(&self.frame.1).atan2(d.dot(&self.frame.0));
        let mut b = 0.;
        for &(phi, length, width) in &self.rays {
            if x > length {
                continue;
            }
            let delta = azimuth - phi;
            let (s, c) = delta.sin_cos();
            if c <= 0. {
                continue;
            }
            // Streaks fanning out a little, so narrower in azimuth farther out; and fading.
            let across = x * s / (width * (1. + 1.5 * x / length));
            b += (-across * across).exp() * (1. - x / length) * smoothstep(1., 3., x);
        }
        // A bright halo of continuous ejecta near the rim.
        b += (1. - smoothstep(1., 2.5, x)) * 0.8;
        b.min(1.5)
    }
}

/// A large basin, and whether lava flooded it.
#[derive(Debug, Clone)]
struct Basin {
    crater: Crater,
    flooded: bool,
}

/// The parameters of a cratered surface.
#[derive(Debug, Clone)]
pub struct SurfaceParams {
    pub seed: u64,
    /// The body's radius, km.
    pub radius: f64,
    /// K in N(>D) = K D⁻² per km²: the craters' saturation density.
    pub crater_density: f64,
    /// The largest crater in the octaves, km; anything bigger is a basin.
    pub max_crater: f64,
    /// The smallest crater generated at all, km (the pixels' size usually stops them first).
    pub min_crater: f64,
    /// Basins, 300 km and up.
    pub basins: usize,
    /// How much of the near side is flooded with maria: 0 none, 1 as on our Moon.
    pub maria: f64,
    /// The share of a mare's craters that are younger than it (the rest are buried).
    pub mare_crater_survival: f64,
    /// Young rayed craters.
    pub rayed_craters: usize,
    /// How much brighter rays are than the ground they lie on (1 doubles it).
    pub ray_brightness: f64,
    /// How bright fresh craters' walls and ejecta are, likewise.
    pub fresh_brightness: f64,
    /// The variation of the highlands' albedo, as a fraction.
    pub albedo_variation: f64,
}

impl Default for SurfaceParams {
    fn default() -> SurfaceParams {
        SurfaceParams {
            seed: 0, radius: 1737.4, crater_density: 0.07, max_crater: 300., min_crater: 0.5,
            basins: 30, maria: 1., mare_crater_survival: 0.3, rayed_craters: 12,
            ray_brightness: 0.5, fresh_brightness: 0.3, albedo_variation: 0.15,
        }
    }
}

/// The surface at a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceSample {
    /// Above the mean sphere, km.
    pub height: f64,
    /// Its gradient along the surface (a tangent vector, km per km).
    pub slope: Vector3<f64>,
    /// How much of a mare it is, 0 to 1.
    pub mare: f64,
    /// The albedo relative to the plain highlands or mare there: fresh ejecta and rays are
    /// brighter.
    pub brightness: f64,
}

/// A cratered surface (see the module docs), in the body's frame: unit directions, with the
/// face towards its planet along +z.
#[derive(Clone)]
pub struct Surface {
    pub params: SurfaceParams,
    basins: Vec<Basin>,
    rays: Vec<RaySystem>,
    noise: SeededNoise,
}

/// Mare plains lie about this far below the mean surface, km.
const MARE_LEVEL: f64 = -1.5;

/// A uniform number in [0, 1) from a hash.
fn unit(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// A uniformly random unit vector from hash `h`.
fn direction(h: u64) -> Vector3<f64> {
    let (u, v) = (unit(mix(h ^ 0x1)), unit(mix(h ^ 0x2)));
    let z = 1. - 2. * u;
    let r = (1. - z * z).max(0.).sqrt();
    let phi = 2. * std::f64::consts::PI * v;
    Vector3::new(r * phi.cos(), r * phi.sin(), z)
}

/// A diameter in [lo, hi) drawn from N(>D) ∝ D⁻², from uniform `u`.
fn power_law_diameter(lo: f64, hi: f64, u: f64) -> f64 {
    lo / (1. - u * (1. - (lo / hi).powi(2))).sqrt()
}

impl std::fmt::Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Surface").field("params", &self.params).field("basins", &self.basins.len()).finish()
    }
}

impl Surface {
    pub fn new(params: SurfaceParams) -> Surface {
        let seed = mix(params.seed ^ 0x6d6f_6f6e);
        let near = Vector3::z();
        let basins = (0..params.basins).map(|i| {
            let h = mix(seed ^ mix(i as u64 + 1));
            let centre = direction(h);
            let d = power_law_diameter(300., 2500., unit(mix(h ^ 3)));
            // Older and more worn than the craters on them.
            let freshness = unit(mix(h ^ 4)) * 0.6;
            // The near side's basins flooded; on the far side hardly any did.
            // Big basins, deep enough to reach the magma, more than small ones.
            let flood = params.maria * 0.95 * smoothstep(-0.35, 0.5, centre.dot(&near)) * smoothstep(350., 800., d);
            Basin { crater: Crater::new(centre, d, params.radius, freshness), flooded: unit(mix(h ^ 5)) < flood }
        }).collect();
        let rays = (0..params.rayed_craters).map(|i| {
            let h = mix(seed ^ mix(0x7261_7973 + i as u64));
            let centre = direction(h);
            let d = power_law_diameter(25., 110., unit(mix(h ^ 3)));
            let crater = Crater::new(centre, d, params.radius, 1.);
            let n = 30 + (unit(mix(h ^ 4)) * 40.) as usize;
            let rays = (0..n).map(|k| {
                let r = mix(h ^ mix(k as u64 + 100));
                (unit(mix(r ^ 1)) * 2. * std::f64::consts::PI,
                 10. + 30. * unit(mix(r ^ 2)).powi(2),
                 0.4 + 0.8 * unit(mix(r ^ 3)))
            }).collect();
            RaySystem { crater, frame: tangent_basis(&centre), rays }
        }).collect();
        Surface { noise: SeededNoise::new(seed), params, basins, rays }
    }

    /// How much of a mare `p` is, 0 to 1: flooded basins, and the great lowland on the near
    /// side.
    pub fn mare(&self, p: &Vector3<f64>) -> f64 {
        if self.params.maria <= 0. {
            return 0.;
        }
        // Lava's ragged shores.
        let ragged = self.noise.fbm(p * 4., 5) * 0.35 + self.noise.fbm(p * 25., 3) * 0.08;
        let mut m: f64 = 0.;
        for b in self.basins.iter().filter(|b| b.flooded) {
            let cos = p.dot(&b.crater.centre);
            if cos < (b.crater.radius * 1.6).cos() {
                continue;
            }
            let x = cos.clamp(-1., 1.).acos() / b.crater.radius;
            m = m.max(1. - smoothstep(0.8, 1., x + ragged));
        }
        // The near side's irregular lowland (Procellarum), and the far side's few patches.
        let near = p.dot(&Vector3::z());
        let lowland = self.noise.fbm(p * 1.3 + Vector3::new(3.1, 0.7, 5.2), 4) + 0.6 * smoothstep(-0.2, 0.9, near)
            + ragged * 0.5 - 0.63 + 0.25 * (self.params.maria - 1.);
        m = m.max(smoothstep(-0.02, 0.08, lowland));
        m
    }

    /// The surface at unit direction `p`, with craters resolved down to about `footprint` km
    /// across (smaller ones would only alias). Without `albedo`, the brightness isn't worked
    /// out (for shadow tests, which need only the height).
    pub fn sample(&self, p: &Vector3<f64>, footprint: f64, albedo: bool) -> SurfaceSample {
        let r = self.params.radius;
        let mare = self.mare(p);
        let mut height = 0.;
        let mut slope = Vector3::zeros();
        let mut brightness = 1.;
        // Basins, under the maria's lava.
        for b in &self.basins {
            if let Some((h, s, _)) = b.crater.at(p, r) {
                height += h;
                slope += s;
            }
        }
        height = height * (1. - mare) + MARE_LEVEL * mare;
        slope *= 1. - mare;
        // The rayed craters, all young.
        for ray in &self.rays {
            if let Some((h, s, _)) = ray.crater.at(p, r) {
                height += h;
                slope += s;
            }
            if albedo {
                let cos = p.dot(&ray.crater.centre);
                if cos > (RAY_REACH * ray.crater.radius).min(3.).cos() {
                    let x = p.cross(&ray.crater.centre).norm().atan2(cos) / ray.crater.radius;
                    // Broken into feathery segments.
                    let streaks = 0.5 + 0.8 * self.noise.fbm(p * (1.5 / ray.crater.radius), 3);
                    brightness += self.params.ray_brightness * ray.brightness(p, x) * streaks.clamp(0., 1.);
                }
            }
        }
        // The octaves of craters.
        let seed = mix(self.params.seed ^ 0x6372_6174);
        let mut diameter = self.params.max_crater;
        let mut octave = 0u64;
        while diameter / 2. >= self.params.min_crater {
            // Craters a pixel or so across fade out as their octave approaches the footprint.
            let fade = smoothstep(0.5, 2., diameter / 2. / footprint.max(1e-9));
            if fade <= 0. {
                break;
            }
            // A cell holds craters whose reach (REACH radii of the largest) is half a cell,
            // so the cells about p hold every crater reaching it.
            let cell = 2. * REACH * diameter / 2. / r;
            let per_cell = self.params.crater_density * 3. / (diameter * diameter) * (cell * r).powi(2);
            let base = (p / cell).map(|c| c.floor() as i64);
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let c = base + Vector3::new(dx, dy, dz);
                        let mid = (c.cast::<f64>() + Vector3::repeat(0.5)) * cell;
                        // Only cells near the sphere hold craters on it.
                        if (mid.norm() - 1.).abs() > cell * 1.4 {
                            continue;
                        }
                        let h = mix(seed ^ mix(octave ^ mix((c.x as u64) ^ mix((c.y as u64) ^ mix(c.z as u64)))));
                        // A Poisson number of candidates, kept if within half a cell of the
                        // sphere, so their density over it is uniform.
                        let u = unit(h);
                        let mut k = 0;
                        let mut prob = (-per_cell).exp();
                        let mut cumulative = prob;
                        while u > cumulative && k < 12 {
                            k += 1;
                            prob *= per_cell / k as f64;
                            cumulative += prob;
                        }
                        for i in 0..k {
                            let hi = mix(h ^ mix(i as u64 + 1));
                            let q = (c.cast::<f64>() + Vector3::new(unit(mix(hi ^ 1)), unit(mix(hi ^ 2)), unit(mix(hi ^ 3)))) * cell;
                            if (q.norm() - 1.).abs() > cell / 2. {
                                continue;
                            }
                            let centre = q.normalize();
                            // Cheap rejection before the crater's made.
                            if (p - centre).norm() > REACH * diameter / 2. / r {
                                continue;
                            }
                            let d = power_law_diameter(diameter / 2., diameter, unit(mix(hi ^ 4)));
                            let freshness = unit(mix(hi ^ 5)).powi(3);
                            let crater = Crater::new(centre, d, r, freshness);
                            if let Some((ch, cs, x)) = crater.at(p, r) {
                                // Younger than the mare, or buried by it.
                                let keep = if unit(mix(hi ^ 6)) < self.params.mare_crater_survival { 1. } else { 1. - mare };
                                let w = keep * fade;
                                height += ch * w;
                                slope += cs * w;
                                if albedo && freshness > 0.6 && x < 1.6 {
                                    brightness += self.params.fresh_brightness * w * ((freshness - 0.6) / 0.4).powi(2)
                                        * (1. - smoothstep(0.9, 1.6, x));
                                }
                            }
                        }
                    }
                }
            }
            diameter /= 2.;
            octave += 1;
        }
        if albedo {
            // The highlands' patchiness, and the maria's flows of different basalts.
            brightness *= 1. + self.params.albedo_variation * (self.noise.fbm(p * 9. + Vector3::new(7.7, 1.3, 2.9), 4) * (1. - mare)
                + 1.5 * self.noise.fbm(p * 5. + Vector3::new(2.3, 8.1, 4.4), 4) * mare);
        }
        // Along the surface only.
        let slope = slope - p * slope.dot(p);
        SurfaceSample { height, slope, mare, brightness: brightness.max(0.) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pike_shapes() {
        // A 10 km simple crater is about 2 km deep; a 100 km complex one about 4 km, with a
        // flat floor and a central peak.
        let simple = Crater::new(Vector3::z(), 10., 1737.4, 1.);
        assert!((simple.depth - 2.0).abs() < 0.1 && simple.floor == 0. && simple.peak == 0.);
        let complex = Crater::new(Vector3::z(), 100., 1737.4, 1.);
        assert!((complex.depth - 4.2).abs() < 0.3, "{}", complex.depth);
        assert!(complex.floor > 0.4 && complex.peak > 0.);
        // Its floor is below the surroundings, its rim above, and it fades out.
        assert!(complex.profile(0.5).0 < 0. && complex.profile(1.).0 > 1.);
        assert_eq!(complex.profile(REACH).0, 0.);
        // Worn craters are shallower.
        assert!(Crater::new(Vector3::z(), 100., 1737.4, 0.).depth < complex.depth / 2.);
    }

    #[test]
    fn test_profile_derivative() {
        let c = Crater::new(Vector3::z(), 60., 1737.4, 0.8);
        for &x in &[0.1, 0.3, 0.7, 0.95, 1.3, 2.2, 2.7] {
            let e = 1e-6;
            let numeric = (c.profile(x + e).0 - c.profile(x - e).0) / (2. * e);
            assert!((numeric - c.profile(x).1).abs() < 1e-4 * (1. + numeric.abs()), "{}: {} vs {}", x, numeric, c.profile(x).1);
        }
    }

    #[test]
    fn test_slope_is_the_heights_gradient() {
        // Compare the slope with finite differences of the height along the surface.
        let s = Surface::new(SurfaceParams { seed: 3, ..Default::default() });
        let footprint = 4.;
        let mut checked = 0;
        for i in 0..40 {
            let p = direction(mix(i));
            let a = s.sample(&p, footprint, false);
            let (t, b) = tangent_basis(&p);
            let e = 1e-6;
            for dir in [t, b] {
                let q = (p + dir * e).normalize();
                let q2 = (p - dir * e).normalize();
                let numeric = (s.sample(&q, footprint, false).height - s.sample(&q2, footprint, false).height) / (2. * e * s.params.radius);
                // The mare's edge isn't differentiated: skip where it changes.
                if (s.sample(&q, footprint, false).mare - s.sample(&q2, footprint, false).mare).abs() > 1e-9 {
                    continue;
                }
                assert!((numeric - a.slope.dot(&dir)).abs() < 1e-3 + 1e-2 * numeric.abs(), "{} vs {}", numeric, a.slope.dot(&dir));
                checked += 1;
            }
        }
        assert!(checked > 40);
    }

    #[test]
    fn test_maria_are_on_the_near_side() {
        let s = Surface::new(SurfaceParams { seed: 1, ..Default::default() });
        let share = |side: f64| {
            let pts: Vec<_> = crate::procedural::space::fibonacci_sphere(4000).filter(|p| p.z * side > 0.).collect();
            pts.iter().map(|p| s.mare(p)).sum::<f64>() / pts.len() as f64
        };
        let (near, far) = (share(1.), share(-1.));
        // The Moon's near side is about 31% maria, its far side 2%.
        assert!(near > 0.15 && near < 0.5, "{}", near);
        assert!(far < near / 3., "{} {}", far, near);
        assert_eq!(Surface::new(SurfaceParams { maria: 0., ..Default::default() }).mare(&Vector3::z()), 0.);
    }

    #[test]
    fn test_crater_density() {
        // Count crater centres between 20 and 40 km over the sphere: K (20⁻² - 40⁻²) per km².
        let s = Surface::new(SurfaceParams { seed: 5, maria: 0., ..Default::default() });
        let r = s.params.radius;
        let area = 4. * std::f64::consts::PI * r * r;
        let expected = s.params.crater_density * (20f64.powi(-2) - 40f64.powi(-2)) * area;
        // The octave of 20-40 km craters is the fourth from 300 km (300, 150, 75, 37.5): count
        // them directly through the cells.
        let diameter = 40.;
        let cell = 2. * REACH * diameter / 2. / r;
        let per_cell = s.params.crater_density * 3. / (diameter * diameter) * (cell * r).powi(2);
        let n = (2. / cell).ceil() as i64 + 1;
        let mut count = 0.;
        for x in -n..n { for y in -n..n { for z in -n..n {
            let mid = (Vector3::new(x as f64, y as f64, z as f64) + Vector3::repeat(0.5)) * cell;
            // Expected number kept: the cell's volume within the shell, times the density.
            let inside = ((mid.norm() - 1.).abs() < cell / 2.) as i32 as f64;
            count += inside * per_cell;
        }}}
        assert!((count / expected - 1.).abs() < 0.2, "{} vs {}", count, expected);
    }
}
