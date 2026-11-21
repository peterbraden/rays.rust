//! Ken Perlin's improved noise ("Improving Noise", SIGGRAPH 2002), and its fractal sum: the
//! renderer's `noise::PerlinNoise` computes with these, from its permutation table.

use crate::{Real, Tables};

/// The dot product of one of the twelve gradients (the edges of a cube) with an offset.
#[inline]
pub fn grad<F: Real>(hash: u32, x: F, y: F, z: F) -> F {
    match hash % 12 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        _ => -y - z,
    }
}

/// 6t^5 - 15t^4 + 10t^3.
#[inline]
fn fade<F: Real>(t: F) -> F {
    t * t * t * (t * (t * F::of(6) - F::of(15)) + F::of(10))
}

#[inline]
fn mix<F: Real>(a: F, b: F, t: F) -> F {
    a + t * (b - a)
}

/// Perlin noise at a point, in [-1, 1], from the permutation table of 512 entries (the 256
/// twice) at `perm` in `tables`.
pub fn perlin<F: Real, T: Tables<F>>(tables: &T, perm: u32, x: F, y: F, z: F) -> F {
    let p = |i: u32| tables.index(perm + i);
    let (xi, yi, zi) = ((x.floor().to_i32() & 255) as u32, (y.floor().to_i32() & 255) as u32, (z.floor().to_i32() & 255) as u32);
    let (x, y, z) = (x - x.floor(), y - y.floor(), z - z.floor());
    let (u, v, w) = (fade(x), fade(y), fade(z));
    let one = F::of(1);
    let a = p(xi) + yi;
    let aa = p(a) + zi;
    let ab = p(a + 1) + zi;
    let b = p(xi + 1) + yi;
    let ba = p(b) + zi;
    let bb = p(b + 1) + zi;
    let g1 = grad(p(aa), x, y, z);
    let g2 = grad(p(ba), x - one, y, z);
    let g3 = grad(p(ab), x, y - one, z);
    let g4 = grad(p(bb), x - one, y - one, z);
    let g5 = grad(p(aa + 1), x, y, z - one);
    let g6 = grad(p(ba + 1), x - one, y, z - one);
    let g7 = grad(p(ab + 1), x, y - one, z - one);
    let g8 = grad(p(bb + 1), x - one, y - one, z - one);
    let near = mix(mix(g1, g2, u), mix(g3, g4, u), v);
    let far = mix(mix(g5, g6, u), mix(g7, g8, u), v);
    mix(near, far, w)
}

/// Fractal Brownian motion: `octaves` of `perlin`, each `lacunarity` times finer and
/// `persistence` times fainter than the last, normalised to [0, 1] (0.5 with no octaves).
pub fn fbm<F: Real, T: Tables<F>>(tables: &T, perm: u32, x: F, y: F, z: F, octaves: u32, persistence: F, lacunarity: F) -> F {
    let mut result = F::of(0);
    let mut amplitude = F::of(1);
    let mut frequency = F::of(1);
    let mut max_value = F::of(0);
    let mut k = 0;
    while k < octaves {
        result = result + perlin(tables, perm, x * frequency, y * frequency, z * frequency) * amplitude;
        max_value = max_value + amplitude;
        amplitude = amplitude * persistence;
        frequency = frequency * lacunarity;
        k += 1;
    }
    if max_value == F::of(0) {
        return F::ratio(1, 2);
    }
    (result / max_value + F::of(1)) * F::ratio(1, 2)
}

/// Hash a cell (and seed) to a number in [0, 1]: Worley noise's feature points, and whatever
/// else is particular to a cell. (Negative cells are numbered by their two's complement.)
pub fn cell_hash<F: Real>(ix: u32, iy: u32, iz: u32, s: u32) -> F {
    let mut h = (ix.wrapping_mul(73856093)) ^ (iy.wrapping_mul(19349663)) ^ (iz.wrapping_mul(83492791)) ^ s.wrapping_mul(0x9E37_79B9);
    // Avalanche (murmur3 finaliser) so neighbouring cells are uncorrelated.
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    F::unit(h)
}
