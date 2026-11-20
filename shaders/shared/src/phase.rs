//! Phase functions: how a medium scatters, by the angle between the directions of travel
//! before and after. The renderer's `material::phase` has the account of each and its
//! sources (Henyey and Greenstein 1941; Draine 2003; Jendersie and d'Eon's fit of the two to
//! water droplets, 2023), and computes with these.

use crate::{Random, Real};

/// Henyey-Greenstein: (1 - g²) / (4π (1 + g² - 2g cos θ)^(3/2)).
#[inline]
pub fn henyey_greenstein<F: Real>(cos_theta: F, g: F) -> F {
    let denom = F::of(1) + g * g - F::of(2) * g * cos_theta;
    (F::of(1) - g * g) / (F::of(4) * F::pi() * denom * denom.sqrt())
}

/// Draine's: Henyey-Greenstein reshaped by (1 + α cos² θ), normalised.
#[inline]
pub fn draine<F: Real>(cos_theta: F, g: F, alpha: F) -> F {
    henyey_greenstein(cos_theta, g) * (F::of(1) + alpha * cos_theta * cos_theta)
        / (F::of(1) + alpha * (F::of(1) + F::of(2) * g * g) / F::of(3))
}

/// Two unit vectors perpendicular to the unit vector `n` and to each other (Duff et al. 2017).
#[inline]
pub fn orthonormal_basis<F: Real>(n: (F, F, F)) -> ((F, F, F), (F, F, F)) {
    let sign = n.2.sign();
    let a = -F::of(1) / (sign + n.2);
    let b = n.0 * n.1 * a;
    ((F::of(1) + sign * n.0 * n.0 * a, sign * b, -sign * n.0), (b, sign + n.1 * n.1 * a, -n.1))
}

/// A direction from the Henyey-Greenstein distribution about `incoming` (a unit vector, the
/// way the light was travelling), from two uniform numbers: its density is exactly
/// `henyey_greenstein`.
pub fn sample_henyey_greenstein<F: Real>(incoming: (F, F, F), g: F, u1: F, u2: F) -> (F, F, F) {
    let one = F::of(1);
    let cos_theta = if g.abs() < F::ratio(1, 1000) {
        one - F::of(2) * u1
    } else {
        let s = (one - g * g) / (one - g + F::of(2) * g * u1);
        (one + g * g - s * s) / (F::of(2) * g)
    }.clamp(-one, one);
    let sin_theta = (one - cos_theta * cos_theta).max(F::of(0)).sqrt();
    let phi = F::of(2) * F::pi() * u2;
    let (t, b) = orthonormal_basis(incoming);
    let (ct, st) = (sin_theta * phi.cos(), sin_theta * phi.sin());
    let v = (t.0 * ct + b.0 * st + incoming.0 * cos_theta, t.1 * ct + b.1 * st + incoming.1 * cos_theta, t.2 * ct + b.2 * st + incoming.2 * cos_theta);
    let length = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    (v.0 / length, v.1 / length, v.2 / length)
}

/// Jendersie and d'Eon's water droplets: (1 - w) HG(g_hg) + w Draine(g_d, α).
#[derive(Clone, Copy)]
pub struct HgDraine<F: Real> {
    pub g_hg: F,
    pub g_d: F,
    pub alpha: F,
    pub w_d: F,
}

impl<F: Real> HgDraine<F> {
    #[inline]
    pub fn eval(&self, cos_theta: F) -> F {
        (F::of(1) - self.w_d) * henyey_greenstein(cos_theta, self.g_hg) + self.w_d * draine(cos_theta, self.g_d, self.alpha)
    }

    /// A direction distributed as `eval` about `incoming`. The Draine lobe is sampled by
    /// rejection from Henyey-Greenstein: accepted with probability (1 + α cos² θ) / (1 + α).
    pub fn sample<R: Random<F>>(&self, incoming: (F, F, F), random: &mut R) -> (F, F, F) {
        if random.next() >= self.w_d {
            let (u1, u2) = (random.next(), random.next());
            return sample_henyey_greenstein(incoming, self.g_hg, u1, u2);
        }
        let mut tries = 0;
        loop {
            let (u1, u2) = (random.next(), random.next());
            let d = sample_henyey_greenstein(incoming, self.g_d, u1, u2);
            let mu = d.0 * incoming.0 + d.1 * incoming.1 + d.2 * incoming.2;
            tries += 1;
            if random.next() * (F::of(1) + self.alpha) < F::of(1) + self.alpha * mu * mu || tries >= F::tries() {
                return d;
            }
        }
    }
}

// ---- Delta-M truncation (W. J. Wiscombe, "The Delta-M Method: Rapid Yet Accurate Radiative
// Flux Calculations for Strongly Asymmetric Phase Functions", Journal of the Atmospheric
// Sciences 34, 1977): a fraction f of a medium's scattering, its phase function's forward
// peak, taken as no scattering at all. The renderer's `volume::VolumeMaterial` has the account
// of where it applies; these are its rules, which a kernel follows too. ----

/// The fraction of Henyey-Greenstein scattering of mean cosine `g` in the peak: g², which
/// keeps the phase function's first two moments.
#[inline]
pub fn forward_peak<F: Real>(g: F) -> F {
    g * g
}

/// The mean cosine of what is left of Henyey-Greenstein scattering without a fraction `f`.
#[inline]
pub fn without_peak<F: Real>(g: F, f: F) -> F {
    (g - f) / (F::of(1) - f)
}

/// The factor on a medium's extinction once a fraction `f` of its scattering, of single
/// scattering albedo `albedo`, counts as none.
#[inline]
pub fn truncated_extinction<F: Real>(albedo: F, f: F) -> F {
    F::of(1) - f * albedo
}

/// The albedo of what is left.
#[inline]
pub fn truncated_albedo<F: Real>(albedo: F, f: F) -> F {
    albedo * (F::of(1) - f) / (F::of(1) - f * albedo)
}
