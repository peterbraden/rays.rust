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

pub struct RainbowTable {
    /// Per bin, the phase function (per steradian) of the light refracted and reflected, for
    /// each channel.
    bins: Vec<[f64; 3]>,
}

impl RainbowTable {
    /// Traced with `rays` impact parameters per wavelength (stratified across the drop's disc),
    /// eight wavelengths per channel.
    pub fn new(rays: usize) -> RainbowTable {
        // Red, green and blue as the bands 600-700, 500-600 and 420-500 nm.
        let channels = [(600., 700.), (500., 600.), (420., 500.)];
        const WAVELENGTHS: usize = 8;
        // The sun's angular radius, smearing each ray's deviation across its disc.
        let sun = 0.265f64.to_radians();
        let mut energy = vec![[0f64; 3]; BINS];
        let mut total = [0f64; 3];
        for (c, (lo, hi)) in channels.iter().enumerate() {
            for w in 0..WAVELENGTHS {
                let n = water_index(lo + (hi - lo) * (w as f64 + 0.5) / WAVELENGTHS as f64);
                for k in 0..rays {
                    // Uniform over the drop's cross-section: b² uniform.
                    let u = (k as f64 + 0.5) / rays as f64;
                    let b = u.sqrt();
                    let i = b.asin();
                    let r = (b / n).asin();
                    let (rs, rp) = fresnel(i.cos(), n);
                    for p in 0..ORDERS {
                        let (d, fraction) = if p == 0 {
                            (PI - 2. * i, (rs + rp) / 2.)
                        } else {
                            let carried = |rr: f64| (1. - rr).powi(2) * rr.powi(p as i32 - 1);
                            (2. * (i - r) + (p as f64 - 1.) * (PI - 2. * r), (carried(rs) + carried(rp)) / 2.)
                        };
                        // Spread over the sun's disc: three points across it, as offsets in angle.
                        for s in [-0.6, 0., 0.6] {
                            let theta = (d + s * sun).cos().clamp(-1., 1.).acos();
                            let bin = ((theta / PI * BINS as f64) as usize).min(BINS - 1);
                            energy[bin][c] += fraction / 3.;
                        }
                        total[c] += fraction;
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
