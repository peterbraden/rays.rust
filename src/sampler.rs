//! # Samplers
//!
//! Where a sample's random numbers come from (Pharr, Jakob and Humphreys, "Physically Based
//! Rendering", 4th ed., chapter 8, "Sampling and Reconstruction"; cited below as PBR). A path
//! draws its numbers one after another (`geometry::rand`), each from the next dimension of the
//! sample's point, until the sampler's dimensions run out and independent numbers take over.
//!
//! - Independent: the pixel position from the R2 sequence, the rest independent.
//! - Sobol': Owen-scrambled, padded Sobol' points, a sequence per pixel (`sobol`).
//! - Halton (PBR section 8.6): coordinate d of point i is the radical inverse of i in the d-th
//!   prime base (its digits mirrored about the radix point; John H. Halton, "On the efficiency
//!   of certain quasi-random sequences of points in evaluating multi-dimensional integrals",
//!   Numerische Mathematik 2, 1960, pp. 84-90, doi:10.1007/BF01386213). Plain Halton points in
//!   large bases correlate badly; each pixel's are Owen-scrambled, every digit permuted by a
//!   hash of the digits before it, as pbrt's `OwenScrambledRadicalInverse` does, using Andrew
//!   Kensler's hashed permutations ("Correlated Multi-Jittered Sampling", Pixar Technical Memo
//!   13-01, 2013, `permute`). Its first `PRIMES` dimensions.
//! - Stratified (PBR section 8.5): each dimension divided into n strata for n samples per pixel,
//!   one sample in each, jittered within it, the strata visited in a random order of each
//!   dimension's own; the pixel position on a √n x √n grid when n is square (Latin hypercube
//!   sampling elsewhere; Michael D. McKay, Richard J. Beckman and W. J. Conover, Technometrics
//!   21(2), 1979, pp. 239-245, doi:10.2307/1268522). Samples beyond n start another round.
//! - ZSobol' (PBR section 8.7.4): one Sobol' sequence spread over the whole image rather than
//!   one per pixel. A pixel's samples are consecutive indices of it, in the order of the pixel's
//!   position along a Morton (Z-order) curve, so neighbouring pixels take neighbouring blocks of
//!   points and their errors are anti-correlated, as blue noise, which looks far less noisy than
//!   white noise of the same size. Abdalla G. M. Ahmed and Peter Wonka, "Screen-Space Blue-Noise
//!   Diffusion of Monte Carlo Sampling Error via Hierarchical Ordering of Pixels", ACM
//!   Transactions on Graphics 39(6) (SIGGRAPH Asia 2020), doi:10.1145/3414685.3417881. Each base-4
//!   digit of the Morton index is permuted by one of the 24 permutations of four, chosen by a hash
//!   of the digits above it and the dimension, so the order is random but hierarchical. Needs
//!   the number of samples per pixel in advance (rounded up to a power of two); samples beyond
//!   it fall back to the Sobol' sampler.
use crate::geometry::Pcg32;
use crate::sobol::{hash, owen_scramble, sobol};
use std::sync::OnceLock;

/// Halton dimensions, as many as the Sobol' sampler's (`sobol::DIMENSIONS`); beyond,
/// independent numbers.
pub const PRIMES: usize = 64;

fn primes() -> &'static [u64] {
    static P: OnceLock<Vec<u64>> = OnceLock::new();
    P.get_or_init(|| {
        let mut p: Vec<u64> = Vec::with_capacity(PRIMES);
        let mut n = 2u64;
        while p.len() < PRIMES {
            if p.iter().take_while(|q| *q * *q <= n).all(|q| !n.is_multiple_of(*q)) {
                p.push(n);
            }
            n += 1;
        }
        p
    })
}

/// Element `i` of a random permutation of 0..`l`, chosen by `p` (Kensler's `permute`): a
/// hash-based bijection on the next power of two, cycle-walked until it lands below `l`.
pub fn permutation_element(mut i: u32, l: u32, p: u32) -> u32 {
    let mut w = l - 1;
    w |= w >> 1;
    w |= w >> 2;
    w |= w >> 4;
    w |= w >> 8;
    w |= w >> 16;
    loop {
        i ^= p;
        i = i.wrapping_mul(0xe170893d);
        i ^= p >> 16;
        i ^= (i & w) >> 4;
        i ^= p >> 8;
        i = i.wrapping_mul(0x0929eb3f);
        i ^= p >> 23;
        i ^= (i & w) >> 1;
        i = i.wrapping_mul(1 | p >> 27);
        i = i.wrapping_mul(0x6935fa69);
        i ^= (i & w) >> 11;
        i = i.wrapping_mul(0x74dcb303);
        i ^= (i & w) >> 2;
        i = i.wrapping_mul(0x9e501cc3);
        i ^= (i & w) >> 2;
        i = i.wrapping_mul(0xc860a3df);
        i &= w;
        i ^= i >> 5;
        if i < l {
            break;
        }
    }
    (i.wrapping_add(p)) % l
}

/// A cheap 64-bit mixer for the per-digit hash (pbrt's `MixBits`).
fn mix_bits(mut v: u64) -> u64 {
    v ^= v >> 31;
    v = v.wrapping_mul(0x7fb5_d329_728e_a185);
    v ^= v >> 27;
    v = v.wrapping_mul(0x81da_def4_bc2d_d44d);
    v ^ (v >> 33)
}

/// The radical inverse of `a` in `base`, Owen-scrambled by `seed`: each digit permuted by a hash
/// of the (scrambled) digits more significant than it (PBR's `OwenScrambledRadicalInverse`), to
/// 32 bits of precision, as the Sobol' points have (pbrt goes to a float's 24).
pub fn owen_radical_inverse(base: u64, mut a: u64, seed: u64) -> f64 {
    const PRECISION: f64 = 1. / 4_294_967_296.;
    let inv = 1. / base as f64;
    let mut reversed: u64 = 0;
    let mut inv_m = 1.;
    while inv_m > PRECISION {
        let next = a / base;
        let digit = (a - next * base) as u32;
        let h = mix_bits(seed ^ reversed) as u32;
        let digit = permutation_element(digit, base as u32, h);
        reversed = reversed * base + digit as u64;
        inv_m *= inv;
        a = next;
    }
    (inv_m * reversed as f64).min(1. - f64::EPSILON / 2.)
}

pub struct Halton {
    seed: u64,
    index: u64,
    dim: usize,
    beyond: Option<Pcg32>,
}

impl Halton {
    pub fn new(seed: u64, index: u64) -> Halton {
        Halton { seed, index, dim: 0, beyond: None }
    }

    fn next(&mut self) -> f64 {
        let d = self.dim;
        self.dim += 1;
        if d >= PRIMES {
            let (seed, index) = (self.seed, self.index);
            return self.beyond.get_or_insert_with(|| Pcg32::new(seed, index)).next_f64();
        }
        owen_radical_inverse(primes()[d], self.index, hash(self.seed, d as u64))
    }
}

pub struct Stratified {
    seed: u64,
    /// The sample's place in its round, the round, and the samples per round.
    k: u32,
    round: u32,
    n: u32,
    dim: u32,
    /// √n, if n is square.
    side: Option<u32>,
    jitter: Pcg32,
}

impl Stratified {
    pub fn new(seed: u64, index: u32, n: u32) -> Stratified {
        let n = n.max(1);
        let side = (n as f64).sqrt().round() as u32;
        Stratified {
            seed, k: index % n, round: index / n, n, dim: 0,
            side: (side * side == n).then_some(side),
            jitter: Pcg32::new(seed, 0x5754_5241 ^ index as u64),
        }
    }

    /// Stratum of this sample in a dimension, by that dimension's permutation of the round.
    fn stratum(&self, d: u32) -> u32 {
        permutation_element(self.k, self.n, hash(self.seed, ((self.round as u64) << 32) | d as u64) as u32)
    }

    fn next(&mut self) -> f64 {
        let d = self.dim;
        self.dim += 1;
        match (d, self.side) {
            // The pixel: one stratum of a square grid, x then y.
            (0 | 1, Some(side)) => {
                let s = self.stratum(0);
                let cell = if d == 0 { s % side } else { s / side };
                (cell as f64 + self.jitter.next_f64()) / side as f64
            },
            _ => (self.stratum(d) as f64 + self.jitter.next_f64()) / self.n as f64,
        }
    }
}

/// Interleave the bits of x and y, x in the even bits (PBR's `EncodeMorton2`).
pub fn morton2(x: u32, y: u32) -> u64 {
    fn spread(v: u32) -> u64 {
        let mut x = v as u64;
        x = (x | (x << 16)) & 0x0000_ffff_0000_ffff;
        x = (x | (x << 8)) & 0x00ff_00ff_00ff_00ff;
        x = (x | (x << 4)) & 0x0f0f_0f0f_0f0f_0f0f;
        x = (x | (x << 2)) & 0x3333_3333_3333_3333;
        x = (x | (x << 1)) & 0x5555_5555_5555_5555;
        x
    }
    spread(x) | (spread(y) << 1)
}

const PERMUTATIONS: [[u8; 4]; 24] = [
    [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 2, 1], [0, 3, 1, 2],
    [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 2, 0], [1, 3, 0, 2],
    [2, 1, 0, 3], [2, 1, 3, 0], [2, 0, 1, 3], [2, 0, 3, 1], [2, 3, 0, 1], [2, 3, 1, 0],
    [3, 1, 2, 0], [3, 1, 0, 2], [3, 2, 1, 0], [3, 2, 0, 1], [3, 0, 2, 1], [3, 0, 1, 2],
];

pub struct ZSobol {
    seed: u64,
    morton: u64,
    log2_spp: u32,
    base4_digits: u32,
    dim: u32,
    index: u32,
    scrambles: [u32; 4],
    beyond: Option<Pcg32>,
}

impl ZSobol {
    /// Sample `k` of pixel (x, y) of an image whose larger side is `resolution`, with `spp`
    /// samples per pixel planned (k must be below it, rounded up to a power of two).
    pub fn new(seed: u64, x: u32, y: u32, k: u32, spp: u32, resolution: u32) -> ZSobol {
        let log2_spp = spp.max(1).next_power_of_two().trailing_zeros();
        let res = resolution.max(1).next_power_of_two();
        let base4_digits = res.trailing_zeros() + log2_spp.div_ceil(2);
        let morton = (morton2(x, y) << log2_spp) | k as u64;
        let mut z = ZSobol { seed, morton, log2_spp, base4_digits, dim: 0, index: 0, scrambles: [0; 4], beyond: None };
        z.group(0);
        z
    }

    /// The sample's index in the image-wide sequence for dimension `dimension` (PBR's
    /// `ZSobolSampler::GetSampleIndex`).
    fn sample_index(&self, dimension: u64) -> u64 {
        let odd = self.log2_spp & 1 == 1;
        let last = if odd { 1 } else { 0 };
        let mut index = 0u64;
        for i in (last..self.base4_digits).rev() {
            let shift = 2 * i - if odd { 1 } else { 0 };
            let digit = (self.morton >> shift) & 3;
            let higher = self.morton >> (shift + 2);
            let p = ((hash(self.seed, higher ^ 0x5555_5555u64.wrapping_mul(dimension)) >> 24) % 24) as usize;
            index |= (PERMUTATIONS[p][digit as usize] as u64) << shift;
        }
        if odd {
            let digit = self.morton & 1;
            index |= digit ^ (hash(self.seed, (self.morton >> 1) ^ 0x5555_5555u64.wrapping_mul(dimension)) & 1);
        }
        index
    }

    /// Set up group `g` of four dimensions: its index and scrambles.
    fn group(&mut self, g: u32) {
        self.index = self.sample_index(g as u64) as u32;
        let h = hash(self.seed ^ 0x7a73_6f62, g as u64);
        self.scrambles = [0u64, 1, 2, 3].map(|d| hash(h, d) as u32);
    }

    fn next(&mut self) -> f64 {
        let d = self.dim;
        self.dim += 1;
        if d >= crate::sobol::DIMENSIONS {
            let (seed, m) = (self.seed, self.morton);
            return self.beyond.get_or_insert_with(|| Pcg32::new(seed, m)).next_f64();
        }
        if d.is_multiple_of(4) && d > 0 {
            self.group(d / 4);
        }
        let i = (d % 4) as usize;
        owen_scramble(sobol(self.index, i), self.scrambles[i]) as f64 * (1. / 4_294_967_296.)
    }
}

/// The random numbers of the sample being traced.
pub enum SampleState {
    Sobol(crate::sobol::SampleState),
    Halton(Halton),
    Stratified(Stratified),
    ZSobol(ZSobol),
}

impl SampleState {
    pub fn next(&mut self) -> f64 {
        match self {
            SampleState::Sobol(s) => s.next(),
            SampleState::Halton(s) => s.next(),
            SampleState::Stratified(s) => s.next(),
            SampleState::ZSobol(s) => s.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_are_permutations() {
        for (l, p) in [(1u32, 3u32), (5, 17), (16, 99), (100, 12345), (24, 7)] {
            let mut seen = vec![false; l as usize];
            for i in 0..l {
                let e = permutation_element(i, l, p);
                assert!(e < l && !seen[e as usize], "l {} p {}: {} repeats", l, p, e);
                seen[e as usize] = true;
            }
        }
    }

    /// An Owen-scrambled radical inverse keeps the net property of the unscrambled one: the
    /// first b^m points have one in each interval of width b^-m.
    #[test]
    fn test_scrambled_radical_inverse_stratifies() {
        for (base, m) in [(2u64, 6u32), (3, 4), (5, 3), (7, 2)] {
            let n = base.pow(m) as usize;
            for seed in 0..3u64 {
                let mut seen = vec![false; n];
                for i in 0..n as u64 {
                    let x = owen_radical_inverse(base, i, seed);
                    assert!((0. ..1.).contains(&x));
                    let cell = (x * n as f64) as usize;
                    assert!(!seen[cell], "base {} seed {}: two points in cell {}", base, seed, cell);
                    seen[cell] = true;
                }
            }
        }
        // Unscrambled digits would give the plain radical inverse; scrambled, still uniform:
        // the mean of many points is a half.
        let mean = (0..10_000u64).map(|i| owen_radical_inverse(11, i, 5)).sum::<f64>() / 10_000.;
        assert!((mean - 0.5).abs() < 1e-3, "{}", mean);
    }

    #[test]
    fn test_stratified_covers_each_stratum_once() {
        let n = 16;
        for d in 0..6 {
            let mut seen = vec![false; n as usize];
            for k in 0..n {
                let mut s = Stratified::new(42, k, n);
                let mut x = 0.;
                for _ in 0..=d { x = s.next(); }
                let cell = (x * if d < 2 { 4. } else { 16. }) as usize;
                if d >= 2 {
                    assert!(!seen[cell], "dimension {}: stratum {} twice", d, cell);
                    seen[cell] = true;
                }
            }
        }
        // The pixel: one sample in each cell of the 4 x 4 grid.
        let mut grid = [[0; 4]; 4];
        for k in 0..16 {
            let mut s = Stratified::new(7, k, 16);
            let (x, y) = (s.next(), s.next());
            grid[(x * 4.) as usize][(y * 4.) as usize] += 1;
        }
        assert!(grid.iter().flatten().all(|c| *c == 1), "{:?}", grid);
    }

    #[test]
    fn test_morton() {
        assert_eq!(morton2(0b11, 0), 0b0101);
        assert_eq!(morton2(0, 0b11), 0b1010);
        assert_eq!(morton2(5, 3), 0b011011);
    }

    /// ZSobol' gives each pixel its own block of the image-wide sequence: across a 4 x 4 image
    /// at 4 samples per pixel, every index from 0 to 63 is used exactly once, and the first two
    /// dimensions of all the samples together are a well stratified (0, 6, 2)-net.
    #[test]
    fn test_zsobol_partitions_the_sequence() {
        let mut used = vec![false; 64];
        let mut cells = vec![false; 64];
        for y in 0..4 {
            for x in 0..4 {
                for k in 0..4 {
                    let mut z = ZSobol::new(3, x, y, k, 4, 4);
                    assert!(!used[z.index as usize], "index {} twice", z.index);
                    used[z.index as usize] = true;
                    let (u, v) = (z.next(), z.next());
                    // Elementary boxes of 8 x 8 cells.
                    let c = (u * 8.) as usize * 8 + (v * 8.) as usize;
                    assert!(!cells[c], "two samples in box {}", c);
                    cells[c] = true;
                }
            }
        }
    }
}
