//! The sea's foam, broken up (the renderer's `ocean::Foam`, which has the account of it and its
//! sources): a breaking crest's fresh foam torn into filaments, and the residual foam it leaves
//! behind thinned into lace. Where the foam is, and how strong, is the caller's (its tiles).

use crate::noise::fbm;
use crate::{smoothstep, Real, Tables};

/// The error function, by Abramowitz and Stegun 7.1.26 (to 1.5e-7).
pub fn erf<F: Real>(x: F) -> F {
    let one = F::of(1);
    let t = one / (one + F::ratio(3275911, 10_000_000) * x.abs());
    let y = one - (((((F::ratio(1061405429, 1_000_000_000) * t - F::ratio(1453152027, 1_000_000_000)) * t) + F::ratio(1421413741, 1_000_000_000)) * t
        - F::ratio(284496736, 1_000_000_000)) * t + F::ratio(254829592, 1_000_000_000)) * t * (-x * x).exp();
    if x >= F::of(0) { y } else { -y }
}

/// What breaks the foam up: the wind's direction (a unit vector) and speed (m/s), the spread of
/// the lace's noise, which turns it into a uniform rank, and where the noise's permutation is.
#[derive(Clone, Copy)]
pub struct FoamPattern<F: Real> {
    pub wind: (F, F),
    pub speed: F,
    pub lace_sd: F,
    pub perm: u32,
}

impl<F: Real> FoamPattern<F> {
    /// A noise value's rank among the noise's values, 0 to 1.
    fn rank(&self, n: F) -> F {
        let half = F::ratio(1, 2);
        half * (F::of(1) + erf((n - half) / (self.lace_sd * F::of(2).sqrt())))
    }

    /// Residual foam of strength `r` at `p` broken into lace: covering about a fraction r of
    /// the surface, in patches that shrink and part as it thins, and in a gale drawn out along
    /// the wind.
    pub fn lace<T: Tables<F>>(&self, tables: &T, r: F, p: (F, F)) -> F {
        let (zero, half, one) = (F::of(0), F::ratio(1, 2), F::of(1));
        if r <= zero {
            return zero;
        }
        let stretch = one + F::of(5) * smoothstep(F::of(12), F::of(22), self.speed);
        let (along, across) = (p.0 * self.wind.0 + p.1 * self.wind.1, -p.0 * self.wind.1 + p.1 * self.wind.0);
        // Patches a few metres across, broken into bubble rafts a metre or so.
        let n = F::ratio(65, 100) * fbm(tables, self.perm, along / (F::ratio(16, 10) * stretch), F::ratio(3, 10), across / F::ratio(16, 10), 3, F::ratio(55, 100), F::ratio(22, 10))
            + F::ratio(35, 100) * fbm(tables, self.perm, along / (F::of(6) * stretch), F::ratio(41, 10), across / F::of(6), 2, half, F::of(2));
        // Its rank among the noise's values, so a fraction r is foam.
        let t = ((r - self.rank(n)) / F::ratio(6, 100) * half + half).clamp(zero, one);
        t * t * (F::of(3) - F::of(2) * t)
    }

    /// A breaking crest's fresh foam of strength `f` at `p`: solid at its core, its edges torn
    /// into tongues and filaments drawn downwind, as the crest spills.
    pub fn ragged<T: Tables<F>>(&self, tables: &T, f: F, p: (F, F)) -> F {
        let (zero, half, one) = (F::of(0), F::ratio(1, 2), F::of(1));
        if f <= zero {
            return zero;
        }
        let (along, across) = (p.0 * self.wind.0 + p.1 * self.wind.1, -p.0 * self.wind.1 + p.1 * self.wind.0);
        let n = fbm(tables, self.perm, along / F::ratio(14, 10) + F::ratio(317, 10), F::ratio(19, 10), across / F::ratio(6, 10), 4, F::ratio(6, 10), F::ratio(21, 10));
        let t = ((F::ratio(13, 10) * f - self.rank(n)) / F::ratio(5, 100) * half + half).clamp(zero, one);
        t * t * (F::of(3) - F::of(2) * t)
    }
}
