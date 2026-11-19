//! Rainbows: the phase function of raindrops, traced through a water sphere by geometric optics.
//!
//! Raindrops are a millimetre or so across, a thousand times the wavelength, so light through
//! them follows rays (H. C. van de Hulst, "Light Scattering by Small Particles", 1957, ch. 12;
//! J. A. Adam, "The mathematical physics of rainbows and glories", Physics Reports 356, 2002).
//! A ray striking a drop at impact parameter b (the incidence angle i has sin i = b) is partly
//! reflected, and partly refracted in at angle r (sin r = b / n), crossing the drop, where at each
//! meeting with the surface it is again partly reflected and partly let out. The light leaving
//! after p chords is deviated by
//!
//!   D = π - 2i (p = 0, reflected), or 2(i - r) + (p - 1)(π - 2r),
//!
//! carrying the fraction T² R^(p-1) (Fresnel's R and T = 1 - R, for each polarisation). For
//! p = 2 and 3 the deviation has a minimum, where rays over a range of b leave together: the
//! primary bow, at about 138° (42° from the point opposite the sun), and the secondary, at 129°
//! (51°), with its colours reversed. Between them is Alexander's dark band, where neither
//! sends light. The minimum moves with the refractive index, which falls with wavelength, so
//! each colour has its own bow: red outside the primary.
//!
//! The table here holds, for each colour channel, the phase function averaged over wavelengths
//! across the channel and over the sun's disc (half a degree across), which widens the bows as
//! it does in the sky. Half of the light a large drop intercepts is diffracted in a lobe a
//! fraction of a degree wide; that half goes almost straight on, and is folded into a tight
//! forward lobe.
//!
//! **Wave optics** (`RainbowTable::airy`). Rays can't show what happens near a bow: the light
//! on either side of the rainbow ray interferes. Near the minimum deviation D_k the outgoing
//! wavefront is a cubic, its tilt at height u across it (h/2)(u/a)², where h = D''(b) at the
//! rainbow ray and a is the drop's radius; diffracting it gives Airy's integral (G. B. Airy, "On
//! the intensity of light in the neighbourhood of a caustic", 1838; Adam 2002, section 4), an
//! intensity ∝ Ai²(-kαε) at an angle ε from D_k towards the lit side, with α = (2a²/kh)^⅓ and
//! k = 2π/λ. Its oscillations are the supernumerary bows inside the primary, a few tenths of a
//! degree apart for drizzle's 0.2 mm drops; its tail leaks into Alexander's band; and for the
//! droplets of fog and cloud, 10 to 20 µm, the main peak is several degrees wide and moves in
//! from 42°, its colours overlapping into white: the fogbow. Far from the bow the oscillations'
//! mean falls as ε^(-1/2), like the geometric light's, which fixes the scale: each geometric ray
//! is weighted by Ai² over that mean, and the dark side gets Airy's tail. Real drops vary in
//! size, so the table averages over radii `spread` either side, which (with the wavelengths
//! across each channel and the sun's disc) washes out the finer fringes as it does in the sky.
//! (Below 10 µm or so the Airy theory itself fails, and Mie theory would be needed.)

use crate::color::Color;
use crate::material::phase::henyey_greenstein;
use std::f64::consts::PI;

/// The refractive index of water at `wavelength` nm (20 °C): a Cauchy fit, A + B / λ², through
/// the indices at the hydrogen F and C lines (486 and 656 nm), within 0.0003 of the measured
/// ones from 400 to 700 nm.
pub fn water_index(wavelength: f64) -> f64 {
    1.3238 + 3141. / (wavelength * wavelength)
}

/// Fresnel reflectance for the s and p polarisations, from a medium of index 1 into one of `n`
/// at incidence angle with cosine `cos_i`.
fn fresnel(cos_i: f64, n: f64) -> (f64, f64) {
    let sin_t = (1. - cos_i * cos_i).max(0.).sqrt() / n;
    let cos_t = (1. - sin_t * sin_t).max(0.).sqrt();
    let rs = (cos_i - n * cos_t) / (cos_i + n * cos_t);
    let rp = (n * cos_i - cos_t) / (n * cos_i + cos_t);
    (rs * rs, rp * rp)
}

/// Bins of the scattering angle, 0 to 180°.
const BINS: usize = 720;

/// The highest order of chords traced: the third and higher bows are faint and lie towards the
/// sun, lost in its glare.
const ORDERS: usize = 4;

/// The fraction of the extinction that is diffracted, almost straight on.
const DIFFRACTED: f64 = 0.5;

/// Mean cosine of the lobe standing in for the diffracted light.
const DIFFRACTION_G: f64 = 0.995;

/// Airy's function Ai(x): its power series for |x| < 6, and its asymptotic forms beyond (with
/// their first corrections), to within about 1e-4 of its peak.
pub fn airy(x: f64) -> f64 {
    if x.abs() < 6. {
        // Ai(x) = c₁ f(x) - c₂ g(x), f and g its series.
        let (c1, c2) = (0.355_028_053_887_817_2, 0.258_819_403_792_806_8);
        let x3 = x * x * x;
        let (mut f, mut g, mut tf, mut tg) = (1., x, 1., x);
        for k in 1..60 {
            let k3 = 3. * k as f64;
            tf *= x3 / ((k3 - 1.) * k3);
            tg *= x3 / (k3 * (k3 + 1.));
            f += tf;
            g += tg;
        }
        c1 * f - c2 * g
    } else if x > 0. {
        let zeta = 2. / 3. * x.powf(1.5);
        (-zeta).exp() / (2. * PI.sqrt() * x.powf(0.25)) * (1. - 5. / (72. * zeta))
    } else {
        let z = -x;
        let zeta = 2. / 3. * z.powf(1.5);
        let phase = zeta + PI / 4.;
        (phase.sin() - 5. / (72. * zeta) * phase.cos()) / (PI.sqrt() * z.powf(0.25))
    }
}

/// The rainbow ray of `p` chords in a drop of index `n`: its impact parameter, its deviation,
/// d²D/db² there, and the fraction of light it carries.
fn rainbow_ray(n: f64, p: usize) -> (f64, f64, f64, f64) {
    let k = p as f64;
    // Where dD/db = 0: cos² i = (n² - 1) / (k² - 1).
    let cos_i = ((n * n - 1.) / (k * k - 1.)).sqrt();
    let b = (1. - cos_i * cos_i).sqrt();
    let dev = |b: f64| { let i = b.asin(); let r = (b / n).asin(); 2. * (i - r) + (k - 1.) * (PI - 2. * r) };
    let h = 1e-4;
    let curvature = (dev(b + h) - 2. * dev(b) + dev(b - h)) / (h * h);
    let (rs, rp) = fresnel(cos_i, n);
    let carried = |rr: f64| (1. - rr).powi(2) * rr.powi(p as i32 - 1);
    (b, dev(b), curvature, (carried(rs) + carried(rp)) / 2.)
}

pub struct RainbowTable {
    /// Per bin, the phase function (per steradian) of the light refracted and reflected, for
    /// each channel.
    bins: Vec<[f64; 3]>,
}

impl RainbowTable {
    /// Traced with `rays` impact parameters per wavelength (stratified across the drop's disc),
    /// eight wavelengths per channel, by geometric optics.
    pub fn new(rays: usize) -> RainbowTable {
        RainbowTable::build(rays, None)
    }

    /// As `new`, with wave optics near the bows for drops of `radius` µm, their radii spread by
    /// `spread` (a fraction) either side: supernumerary bows, and for small droplets the fogbow.
    pub fn airy(rays: usize, radius: f64, spread: f64) -> RainbowTable {
        RainbowTable::build(rays, Some((radius.max(1.), spread.clamp(0., 0.9))))
    }

    fn build(rays: usize, drops: Option<(f64, f64)>) -> RainbowTable {
        // Red, green and blue as the bands 600-700, 500-600 and 420-500 nm.
        let channels = [(600., 700.), (500., 600.), (420., 500.)];
        const WAVELENGTHS: usize = 8;
        // The sun's angular radius, smearing each ray's deviation across its disc.
        let sun = 0.265f64.to_radians();
        let mut energy = vec![[0f64; 3]; BINS];
        let mut total = [0f64; 3];
        // Radii, each weighted by its cross-section.
        let radii: Vec<(f64, f64)> = match drops {
            None => vec![(0., 1.)],
            Some((a, spread)) => {
                let m = 5;
                let list: Vec<f64> = (0..m).map(|j| a * (1. + spread * (2. * (j as f64 + 0.5) / m as f64 - 1.))).collect();
                let sum: f64 = list.iter().map(|r| r * r).sum();
                list.iter().map(|r| (*r, r * r / sum)).collect()
            },
        };
        let add = |c: usize, d: f64, weight: f64, energy: &mut Vec<[f64; 3]>| {
            // Spread over the sun's disc: three points across it, as offsets in angle.
            for s in [-0.6, 0., 0.6] {
                let theta = (d + s * sun).cos().clamp(-1., 1.).acos();
                let bin = ((theta / PI * BINS as f64) as usize).min(BINS - 1);
                energy[bin][c] += weight / 3.;
            }
        };
        for (c, (lo, hi)) in channels.iter().enumerate() {
            for w in 0..WAVELENGTHS {
                let wavelength = lo + (hi - lo) * (w as f64 + 0.5) / WAVELENGTHS as f64;
                let n = water_index(wavelength);
                for &(radius, share) in &radii {
                    // Airy's theory for the primary and secondary bows: per order, kα, the
                    // bow's deviation, and G, the rays' density there (G / √ε per radian).
                    let bows: Vec<Option<(f64, f64, f64)>> = (0..ORDERS).map(|p| {
                        if drops.is_none() || !(2..=3).contains(&p) {
                            return None;
                        }
                        let (b, d, h, f) = rainbow_ray(n, p);
                        let k = 2. * PI / (wavelength * 1e-9);
                        let a = radius * 1e-6;
                        let alpha = (2. * a * a / (k * h.abs())).cbrt();
                        // b² uniform over the disc, and two rays at each ε.
                        let g = 4. * b * f / (2. * h.abs()).sqrt();
                        Some((k * alpha, d, g))
                    }).collect();
                    // The dark side of each bow, which rays never reach: Airy's tail, scaled
                    // to match the rays (whose light the table normalises by, per ray).
                    for bow in bows.iter().flatten() {
                        let (ka, d, g) = *bow;
                        let scale = 2. * PI * g * ka.sqrt() * rays as f64 * share;
                        let steps = 400;
                        let reach = 8. / ka;
                        for j in 0..steps {
                            let eps = -reach * (j as f64 + 0.5) / steps as f64;
                            let weight = scale * airy(-ka * eps).powi(2) * reach / steps as f64;
                            add(c, d + eps, weight, &mut energy);
                            total[c] += weight;
                        }
                    }
                    for k in 0..rays {
                        // Uniform over the drop's cross-section: b² uniform.
                        let u = (k as f64 + 0.5) / rays as f64;
                        let b = u.sqrt();
                        let i = b.asin();
                        let r = (b / n).asin();
                        let (rs, rp) = fresnel(i.cos(), n);
                        for (p, bow) in bows.iter().enumerate() {
                            let (d, fraction) = if p == 0 {
                                (PI - 2. * i, (rs + rp) / 2.)
                            } else {
                                let carried = |rr: f64| (1. - rr).powi(2) * rr.powi(p as i32 - 1);
                                (2. * (i - r) + (p as f64 - 1.) * (PI - 2. * r), (carried(rs) + carried(rp)) / 2.)
                            };
                            // Near a bow, Airy's fringes: the ray weighted by Ai² over its mean.
                            let fringes = match *bow {
                                Some((ka, d0, _)) => {
                                    let z = (ka * (d - d0)).max(1e-6);
                                    airy(-z).powi(2) * 2. * PI * z.sqrt()
                                },
                                None => 1.,
                            };
                            add(c, d, fraction * fringes * share, &mut energy);
                            total[c] += fraction * share;
                        }
                    }
                }
            }
        }
        // Per steradian, normalised so that the refracted and reflected light integrate to one.
        let bins = (0..BINS).map(|k| {
            let (a, b) = (k as f64 / BINS as f64 * PI, (k + 1) as f64 / BINS as f64 * PI);
            let solid = 2. * PI * (a.cos() - b.cos());
            [0, 1, 2].map(|c| energy[k][c] / total[c] / solid)
        }).collect();
        RainbowTable { bins }
    }

    /// The phase function for scattering through an angle with cosine `cos_theta`, per channel:
    /// the traced light, and the diffracted half going nearly straight on.
    pub fn eval(&self, cos_theta: f64) -> Color {
        let theta = cos_theta.clamp(-1., 1.).acos();
        let x = theta / PI * BINS as f64 - 0.5;
        let k = (x.floor().max(0.) as usize).min(BINS - 1);
        let f = (x - k as f64).clamp(0., 1.);
        let next = (k + 1).min(BINS - 1);
        let v = [0, 1, 2].map(|c| self.bins[k][c] * (1. - f) + self.bins[next][c] * f);
        let forward = henyey_greenstein(cos_theta, DIFFRACTION_G) * DIFFRACTED;
        Color::new(v[0], v[1], v[2]) * (1. - DIFFRACTED) + Color::white() * forward
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;

    fn table() -> RainbowTable {
        RainbowTable::new(20_000)
    }

    /// The scattering angle (degrees) with the most light between `lo` and `hi`, in `channel`.
    fn peak(t: &RainbowTable, lo: f64, hi: f64, channel: usize) -> f64 {
        let mut best = (0., 0.);
        let mut a = lo;
        while a <= hi {
            let v = t.eval(a.to_radians().cos()).rgb[channel];
            if v > best.1 {
                best = (a, v);
            }
            a += 0.05;
        }
        best.0
    }

    #[test]
    fn test_airy_function() {
        // Ai(0), its first zeros and maximum on the negative axis, and a value on the positive
        // one (Abramowitz and Stegun, table 10.11).
        assert!((airy(0.) - 0.355_028_05).abs() < 1e-7);
        assert!(airy(-2.338_107).abs() < 1e-5 && airy(-4.087_949).abs() < 1e-5);
        assert!(airy(-7.944_134).abs() < 2e-3 && airy(-9.022_651).abs() < 2e-3);
        assert!((airy(-1.018_793) - 0.535_656_7).abs() < 1e-5);
        assert!((airy(2.) - 0.034_924_13).abs() < 1e-6);
        // Continuous where the series hands over to the asymptotic forms.
        assert!((airy(-5.999) - airy(-6.001)).abs() < 2e-3);
        assert!((airy(5.999) - airy(6.001)).abs() < 1e-5);
    }

    /// Angles (degrees of scattering) where the green channel has a local maximum between
    /// `lo` and `hi`.
    fn maxima(t: &RainbowTable, lo: f64, hi: f64) -> Vec<f64> {
        let g = |a: f64| t.eval(a.to_radians().cos()).rgb[1];
        let mut out = vec![];
        let mut a = lo + 0.05;
        while a < hi {
            if g(a) > g(a - 0.05) && g(a) >= g(a + 0.05) { out.push(a); }
            a += 0.05;
        }
        out
    }

    #[test]
    fn test_supernumeraries_and_fogbow() {
        // Drizzle, 0.2 mm: the primary, then supernumeraries inside it (at larger scattering
        // angles), where the geometric table only falls smoothly.
        let drizzle = RainbowTable::airy(20_000, 200., 0.05);
        let peaks = maxima(&drizzle, 136., 143.);
        assert!(peaks.len() >= 2, "{:?}", peaks);
        assert!(maxima(&table(), 139., 143.).is_empty());
        // Fog, 15 µm: one broad bow, nearer the antisolar point than the rainbow's 42°.
        let fog = RainbowTable::airy(20_000, 15., 0.2);
        let bow = 180. - peak(&fog, 130., 150., 1);
        assert!(bow < 41.5 && bow > 34., "fogbow at {}", bow);
        let width = |t: &RainbowTable| {
            let top = t.eval(peak(t, 130., 150., 1).to_radians().cos()).rgb[1];
            let mut w = 0.;
            let mut a: f64 = 128.;
            while a < 152. { if t.eval(a.to_radians().cos()).rgb[1] > top / 2. { w += 0.05; } a += 0.05; }
            w
        };
        assert!(width(&fog) > 3. * width(&table()), "{} vs {}", width(&fog), width(&table()));
        // Light reaches into Alexander's band from the primary's dark side.
        let g = |t: &RainbowTable, a: f64| t.eval(a.to_radians().cos()).rgb[1];
        assert!(g(&fog, 136.) > g(&table(), 136.));
    }

    #[test]
    fn test_airy_tables_are_normalised() {
        for t in [RainbowTable::airy(20_000, 200., 0.05), RainbowTable::airy(20_000, 15., 0.2)] {
            let n = 20_000;
            let mut sum = [0.; 3];
            for k in 0..n {
                let mu = -1. + 2. * (k as f64 + 0.5) / n as f64;
                let v = t.eval(mu).rgb - Vector3::repeat(henyey_greenstein(mu, DIFFRACTION_G) * DIFFRACTED);
                for c in 0..3 { sum[c] += v[c] * 2. * PI * 2. / n as f64; }
            }
            for s in sum { assert!((s - (1. - DIFFRACTED)).abs() < 0.02, "{:?}", sum); }
        }
    }

    #[test]
    fn test_water_index() {
        assert!((water_index(589.3) - 1.3330).abs() < 5e-4);
        assert!((water_index(404.7) - 1.3428).abs() < 5e-4);
    }

    #[test]
    fn test_primary_and_secondary_bows() {
        let t = table();
        // Primary: 42° from the antisolar point, so scattered through about 138°; red outermost
        // (scattered through the least angle), blue inside.
        let (red, blue) = (peak(&t, 130., 145., 0), peak(&t, 130., 145., 2));
        assert!((180. - red - 42.3).abs() < 0.7, "red at {}", 180. - red);
        assert!(red < blue, "red {} blue {}", red, blue);
        // Secondary: about 51° from the antisolar point, its colours reversed.
        let (red2, blue2) = (peak(&t, 120., 132., 0), peak(&t, 120., 132., 2));
        assert!((180. - red2 - 50.5).abs() < 1.2, "red at {}", 180. - red2);
        assert!(red2 > blue2);
        // Alexander's dark band between them is darker than inside the primary.
        let g = |a: f64| t.eval(a.to_radians().cos()).rgb[1];
        assert!(g(133.5) < 0.5 * g(150.), "{} vs {}", g(133.5), g(150.));
    }

    #[test]
    fn test_normalised() {
        let t = table();
        let n = 20_000;
        let mut sum = [0.; 3];
        for k in 0..n {
            let mu = -1. + 2. * (k as f64 + 0.5) / n as f64;
            // The forward lobe is narrower than this grid; integrate the traced light alone.
            let v = t.eval(mu).rgb - Vector3::repeat(henyey_greenstein(mu, DIFFRACTION_G) * DIFFRACTED);
            for c in 0..3 {
                sum[c] += v[c] * 2. * PI * 2. / n as f64;
            }
        }
        for s in sum {
            assert!((s - (1. - DIFFRACTED)).abs() < 0.01, "{:?}", sum);
        }
    }
}
