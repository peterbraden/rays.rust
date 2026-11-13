//! # Low-discrepancy sampling: Owen-scrambled Sobol' points
//!
//! A pixel's value is an integral over many dimensions at once: where in the pixel the ray
//! starts, where on the lens, which way each bounce scatters, which point of a light each
//! shadow ray aims at, whether Russian roulette ends the path. Independent random numbers clump
//! and leave gaps in every one of those dimensions. Points spread more evenly, with lower
//! discrepancy, integrate smooth functions with less error: for n samples the error of a
//! randomised quasi-Monte Carlo estimate can fall faster than the n^(-1/2) of independent
//! sampling.
//!
//! ## Sobol' points
//!
//! I. M. Sobol', "On the distribution of points in a cube and the approximate evaluation of
//! integrals", USSR Computational Mathematics and Mathematical Physics 7(4), 1967, pp. 86-112,
//! doi:10.1016/0041-5553(67)90144-9. The i-th point's coordinate in a dimension is the XOR of
//! that dimension's direction numbers v_k for each bit k of i that is set. The first dimension's
//! are v_k = 2^-k (the van der Corput sequence, i's bits reversed); each later dimension's come
//! from a primitive polynomial over GF(2) of degree s with coefficients a, and initial values
//! m_1 .. m_s, by the recurrence
//!
//! ```text
//! v_k = v_(k-s) ⊕ (v_(k-s) / 2^s) ⊕ a_1 v_(k-1) ⊕ ... ⊕ a_(s-1) v_(k-s+1)     (v_k = m_k / 2^k for k ≤ s)
//! ```
//!
//! The polynomials and initial values here are Stephen Joe and Frances Y. Kuo's, chosen for good
//! two-dimensional projections: "Constructing Sobol Sequences with Better Two-Dimensional
//! Projections", SIAM Journal on Scientific Computing 30(5), 2008, pp. 2635-2654,
//! doi:10.1137/070709359; the values for dimensions 2 to 4 of their file `new-joe-kuo-6.21201`
//! (<https://web.maths.unsw.edu.au/~fkuo/sobol/>), copyright (c) 2008 Frances Y. Kuo and
//! Stephen Joe, distributed under a BSD-style licence
//! (<https://web.maths.unsw.edu.au/~fkuo/sobol/licence>).
//!
//! The first two dimensions form a (0, 2)-sequence: the first 2^m points are a (0, m, 2)-net,
//! every elementary box of area 2^-m holding exactly one point. Other pairs of dimensions are
//! nets with a larger t (boxes of area 2^(t-m) hold 2^t points each), which Joe and Kuo's
//! choice keeps small. So the pixel position, which takes a group's first two dimensions, is
//! the best stratified of all.
//!
//! ## Owen scrambling
//!
//! The points themselves are deterministic, and the same in every pixel. Randomising them by
//! Owen's nested uniform scrambling makes each point uniformly distributed (so estimates are
//! unbiased) while keeping the net property: Art B. Owen, "Randomly Permuted (t,m,s)-Nets and
//! (t,s)-Sequences", in Monte Carlo and Quasi-Monte Carlo Methods in Scientific Computing,
//! Lecture Notes in Statistics 106, Springer 1995, pp. 299-317,
//! doi:10.1007/978-1-4612-2552-2_19 (both results are restated in Owen's "Practical
//! Quasi-Monte Carlo Integration", 2023, section 17.5, propositions 17.2 and 17.3,
//! <https://artowen.su.domains/mc/practicalqmc.pdf>). The scramble flips each digit of a
//! coordinate by a random function of the digits before it.
//!
//! Brent Burley, "Practical Hash-based Owen Scrambling", JCGT 9(4), 2020
//! (<https://jcgt.org/published/0009/04/01/>), does this with a hash: Samuli Laine and Tero
//! Karras's permutation ("Stratified Sampling for Stochastic Transparency", Computer Graphics
//! Forum 30(4), 2011, doi:10.1111/j.1467-8659.2011.01978.x) flips each bit by a hash of the
//! bits below it, so applying it to a coordinate's bits reversed (section 3.2) flips each by
//! the bits above it, which is nested uniform scrambling. The hash used here is Nathan
//! Vegdahl's improvement of Laine and Karras's ("Building a better LK hash", 2021,
//! <https://psychopath.io/post/2021_01_30_building_a_better_lk_hash>), as in Pharr, Jakob and
//! Humphreys, "Physically Based Rendering", 4th ed., section 8.7.2, `FastOwenScrambler`
//! (<https://pbr-book.org/4ed/Sampling_and_Reconstruction/Sobol_Samplers>).
//!
//! ## Padding
//!
//! A path takes as many random numbers as it has bounces, and more. Burley (section 4) pads
//! to any number of dimensions with only four Sobol' dimensions: each group of four takes its
//! own seed, scrambling its coordinates and shuffling the order of its points (the index is
//! itself scrambled the same way, a nested uniform shuffle). Within a group the points stay
//! well stratified in four dimensions; between groups they are decorrelated, as independent
//! sampling would be. `rand()` hands out coordinates dimension by dimension, in the order a
//! path asks for them (`geometry::begin_sample`), up to `DIMENSIONS`, then independent numbers.
//!
//! ## Departures and open questions
//!
//! - Calls to `rand()` aren't aligned to sampling decisions: a decision taking two numbers may
//!   straddle two groups of four, and paths that branch take different numbers of them, so
//!   later decisions of different samples draw on different dimensions. Both cost some
//!   stratification, not correctness: every coordinate is uniform.
//! - 32 bits per coordinate, so values are multiples of 2^-32.

/// Direction numbers v_1 .. v_32 for the first four dimensions, as 32-bit integers (v_k 2^32).
const DIRECTIONS: [[u32; 32]; 4] = [
    van_der_corput(),
    joe_kuo(1, 0, [1, 0, 0]),
    joe_kuo(2, 1, [1, 3, 0]),
    joe_kuo(3, 1, [1, 3, 1]),
];

const fn van_der_corput() -> [u32; 32] {
    let mut v = [0u32; 32];
    let mut k = 0;
    while k < 32 {
        v[k] = 1 << (31 - k);
        k += 1;
    }
    v
}

/// Direction numbers for a primitive polynomial of degree `s` with coefficients `a` (the
/// middle ones, as Joe and Kuo give them) and initial values `m`.
const fn joe_kuo(s: usize, a: u32, m: [u32; 3]) -> [u32; 32] {
    let mut v = [0u32; 32];
    let mut k = 0;
    while k < s {
        v[k] = m[k] << (31 - k);
        k += 1;
    }
    while k < 32 {
        let mut x = v[k - s] ^ (v[k - s] >> s);
        let mut j = 1;
        while j < s {
            if (a >> (s - 1 - j)) & 1 == 1 {
                x ^= v[k - j];
            }
            j += 1;
        }
        v[k] = x;
        k += 1;
    }
    v
}

/// For each dimension and each byte of an index, the XOR of the direction numbers that byte's
/// set bits select, so a coordinate takes four lookups rather than a loop over 32 bits.
/// (Shuffled indices are full 32-bit numbers, so that loop ran 32 times, and dominated the
/// render's profile.)
const TABLES: [[[u32; 256]; 4]; 4] = tables();

const fn tables() -> [[[u32; 256]; 4]; 4] {
    let mut t = [[[0u32; 256]; 4]; 4];
    let mut d = 0;
    while d < 4 {
        let mut b = 0;
        while b < 4 {
            let mut v = 0;
            while v < 256 {
                let mut x = 0;
                let mut k = 0;
                while k < 8 {
                    if (v >> k) & 1 == 1 {
                        x ^= DIRECTIONS[d][b * 8 + k];
                    }
                    k += 1;
                }
                t[d][b][v] = x;
                v += 1;
            }
            b += 1;
        }
        d += 1;
    }
    t
}

/// The `index`-th point's coordinate in Sobol' dimension `dim` (0 to 3), unscrambled: the XOR
/// of the direction numbers for each set bit of `index`.
pub fn sobol(index: u32, dim: usize) -> u32 {
    let t = &TABLES[dim];
    t[0][(index & 0xff) as usize] ^ t[1][((index >> 8) & 0xff) as usize]
        ^ t[2][((index >> 16) & 0xff) as usize] ^ t[3][(index >> 24) as usize]
}

/// Nested uniform scrambling of `x` by `seed`: reverse the bits, apply Vegdahl's hash
/// permutation (each bit flipped by a hash of those below it), reverse back.
pub fn owen_scramble(x: u32, seed: u32) -> u32 {
    let mut v = x.reverse_bits();
    v ^= v.wrapping_mul(0x3d20adea);
    v = v.wrapping_add(seed);
    v = v.wrapping_mul((seed >> 16) | 1);
    v ^= v.wrapping_mul(0x05526c56);
    v ^= v.wrapping_mul(0x53a22864);
    v.reverse_bits()
}

/// Mix `value` into `seed`: a 64-bit finaliser (Sebastiano Vigna's SplitMix64) of both.
pub fn hash(seed: u64, value: u64) -> u64 {
    let mut z = seed ^ value.wrapping_add(0x9e37_79b9_7f4a_7c15).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Coordinate `dim` of point `index` of the padded, Owen-scrambled sequence for `seed`, in
/// [0, 1).
pub fn sample(seed: u64, index: u32, dim: u32) -> f64 {
    let (shuffled, scrambles) = group(seed, index, dim / 4);
    let d = (dim % 4) as usize;
    owen_scramble(sobol(shuffled, d), scrambles[d]) as f64 * (1. / 4_294_967_296.)
}

/// For group `g` of four dimensions: the shuffled index, and each dimension's scramble seed.
fn group(seed: u64, index: u32, g: u32) -> (u32, [u32; 4]) {
    let h = hash(seed, g as u64);
    let scrambles = [0, 1, 2, 3].map(|d| (h >> 32) as u32 ^ hash(h, d) as u32);
    (owen_scramble(index, h as u32), scrambles)
}

/// Dimensions of the Sobol' sequence a sample uses; later numbers come from a generator of its
/// own. The first dimensions (the pixel, the lens, the first bounces' lights and directions)
/// gain most from stratification. Deep in a path, or tracking through a medium, which takes
/// many numbers, the benefit is small and the cost of each (two hashed scrambles) is not.
pub const DIMENSIONS: u32 = 64;

/// Where a sample is in the sequence, with the current group's shuffled index and scrambles
/// computed once for its four dimensions.
pub struct SampleState {
    seed: u64,
    index: u32,
    dim: u32,
    shuffled: u32,
    scrambles: [u32; 4],
    beyond: Option<crate::geometry::Pcg32>,
}

impl SampleState {
    pub fn new(seed: u64, index: u32) -> SampleState {
        let (shuffled, scrambles) = group(seed, index, 0);
        SampleState { seed, index, dim: 0, shuffled, scrambles, beyond: None }
    }

    /// The next dimension's coordinate: as `sample(seed, index, dim)`, until `DIMENSIONS`.
    pub fn next(&mut self) -> f64 {
        let dim = self.dim;
        self.dim += 1;
        if dim >= DIMENSIONS {
            let (seed, index) = (self.seed, self.index);
            return self.beyond.get_or_insert_with(|| crate::geometry::Pcg32::new(seed, index as u64)).next_f64();
        }
        if dim % 4 == 0 && dim > 0 {
            (self.shuffled, self.scrambles) = group(self.seed, self.index, dim / 4);
        }
        let d = (dim % 4) as usize;
        owen_scramble(sobol(self.shuffled, d), self.scrambles[d]) as f64 * (1. / 4_294_967_296.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tables_match_the_definition() {
        let direct = |i: u32, d: usize| (0..32).filter(|k| (i >> k) & 1 == 1).fold(0, |x, k| x ^ DIRECTIONS[d][k]);
        let mut rng = crate::geometry::Pcg32::new(1, 2);
        for _ in 0..10_000 {
            let i = rng.next_u32();
            for d in 0..4 {
                assert_eq!(sobol(i, d), direct(i, d));
            }
        }
    }

    #[test]
    fn test_first_points() {
        // Points 1 to 7. Read in Gray-code order (1, 3, 2, 6, 7, 5, 4), the first two
        // dimensions give the commonly tabulated start of the sequence, 0.5, 0.75, 0.25, 0.375,
        // 0.875, 0.625, 0.125 and 0.5, 0.25, 0.75, 0.375, 0.875, 0.125, 0.625; the others are
        // as the recurrence gives them (and test_pairs_of_dimensions_are_nets checks).
        let f = |d: usize| (1..8).map(|i| sobol(i, d) as f64 / 4_294_967_296.).collect::<Vec<_>>();
        assert_eq!(f(0), vec![0.5, 0.25, 0.75, 0.125, 0.625, 0.375, 0.875]);
        assert_eq!(f(1), vec![0.5, 0.75, 0.25, 0.625, 0.125, 0.375, 0.875]);
        assert_eq!(f(2), vec![0.5, 0.75, 0.25, 0.375, 0.875, 0.625, 0.125]);
        assert_eq!(f(3), vec![0.5, 0.75, 0.25, 0.125, 0.625, 0.875, 0.375]);
    }

    /// Whether points (x, y) form a (0, m, 2)-net: each of the 2^m boxes of every shape
    /// 2^-a by 2^-(m-a) holds exactly one.
    fn is_net(points: &[(f64, f64)]) -> bool {
        let m = points.len().trailing_zeros();
        (0..=m).all(|a| {
            let (nx, ny) = (1usize << a, 1usize << (m - a));
            let mut seen = vec![false; nx * ny];
            points.iter().all(|&(x, y)| {
                let cell = (x * nx as f64) as usize + nx * (y * ny as f64) as usize;
                !std::mem::replace(&mut seen[cell], true)
            })
        })
    }

    #[test]
    fn test_first_two_dimensions_are_nets() {
        for n in [4, 16, 256, 1024] {
            let points: Vec<(f64, f64)> = (0..n).map(|i| (sobol(i, 0) as f64 / 4_294_967_296., sobol(i, 1) as f64 / 4_294_967_296.)).collect();
            assert!(is_net(&points), "{} points", n);
        }
        // Not every pair: dimensions 0 and 2 aren't a (0, m, 2)-net.
        let points: Vec<(f64, f64)> = (0..64).map(|i| (sobol(i, 0) as f64 / 4_294_967_296., sobol(i, 2) as f64 / 4_294_967_296.)).collect();
        assert!(!is_net(&points));
    }

    #[test]
    fn test_scrambling_keeps_nets_and_shuffling_keeps_prefixes() {
        // Owen scrambling preserves the net property; and the first 2^k shuffled indices are
        // still the first 2^k points, in another order, so every power-of-two prefix is a net.
        for seed in [1u64, 7, 12345] {
            for n in [16, 64, 256] {
                let points: Vec<(f64, f64)> = (0..n).map(|i| (sample(seed, i, 0), sample(seed, i, 1))).collect();
                assert!(is_net(&points), "seed {} n {}", seed, n);
                // The next group's first two dimensions, padded with its own seed.
                let later: Vec<(f64, f64)> = (0..n).map(|i| (sample(seed, i, 4), sample(seed, i, 5))).collect();
                assert!(is_net(&later), "padded group: seed {} n {}", seed, n);
            }
        }
    }

    #[test]
    fn test_sample_state_follows_the_sequence() {
        let mut state = SampleState::new(42, 5);
        for dim in 0..DIMENSIONS {
            assert_eq!(state.next(), sample(42, 5, dim), "dimension {}", dim);
        }
        // Then numbers of its own, still in [0, 1).
        let beyond: Vec<f64> = (0..100).map(|_| state.next()).collect();
        assert!(beyond.iter().all(|x| (0. ..1.).contains(x)));
    }

    #[test]
    fn test_scrambled_coordinates_are_uniform() {
        // Over many seeds, each coordinate is uniform: its mean is 1/2, its variance 1/12.
        let n = 20_000;
        for dim in [0, 1, 5, 9] {
            let v: Vec<f64> = (0..n).map(|s| sample(s as u64, 3, dim)).collect();
            let mean = v.iter().sum::<f64>() / n as f64;
            let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
            assert!((mean - 0.5).abs() < 0.01, "dim {}: mean {}", dim, mean);
            assert!((var - 1. / 12.).abs() < 0.003, "dim {}: variance {}", dim, var);
        }
    }

    #[test]
    fn test_integrates_smooth_functions_better_than_independent_sampling() {
        // ∫∫ x y over the unit square is 1/4: the RMS error of 64-point estimates over many
        // seeds is far lower with scrambled Sobol' points than with independent ones.
        let f = |x: f64, y: f64| x * y;
        let trials = 500;
        let mut rng = crate::geometry::Pcg32::new(9, 9);
        let (mut sobol_err, mut random_err) = (0., 0.);
        for t in 0..trials {
            let s = (0..64).map(|i| f(sample(t, i, 0), sample(t, i, 1))).sum::<f64>() / 64.;
            let r = (0..64).map(|_| f(rng.next_f64(), rng.next_f64())).sum::<f64>() / 64.;
            sobol_err += (s - 0.25).powi(2);
            random_err += (r - 0.25).powi(2);
        }
        assert!(sobol_err < 0.05 * random_err, "{} vs {}", sobol_err, random_err);
    }
}
