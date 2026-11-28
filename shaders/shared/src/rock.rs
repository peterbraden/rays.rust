//! The procedural rock: its relief, bump mapped, and its colour. The renderer's
//! `material::rock`, which has the account of the method and its sources; written here once
//! for the renderer, at f64, and the kernels, at f32 (plan.md section 23). The cross-bedding
//! is the renderer's alone (its laminae are the terrain's too): `albedo_before` stops where it
//! is drawn and `albedo_after` goes on.
//!
//! Every expression keeps the renderer's order of operations, its additions of zero included,
//! so that at f64 the results are its own to the bit.

use crate::noise::perlin;
use crate::{Real, Tables};

/// A colour: red, green, blue.
pub type Rgb<F> = (F, F, F);

/// The recipe (the renderer's `Rock` and its `RockStyle`, whose fields these are), and where
/// its tables are: the noise's permutation (512 entries, `Tables::index`); the fractal relief's
/// rotations, an octave's 3 x 3 rows each (16 of them, `Tables::value`); the beds' colours (3
/// each) and the layers (a height and a colour, 4 each).
#[derive(Clone, Copy)]
pub struct Rock<F: Real> {
    pub color: Rgb<F>,
    pub secondary: Rgb<F>,
    pub dark_grain: Rgb<F>,
    pub light_grain: Rgb<F>,
    pub grain_size: F,
    pub strata_spacing: F,
    pub strata_color: Rgb<F>,
    pub strata_amount: F,
    pub bump: F,
    pub bump_size: F,
    pub cracks: F,
    pub scale: F,
    pub weathering: F,
    pub lichen: F,
    pub lichen_color: Rgb<F>,
    pub wetness: F,
    pub layer_warp: F,
    pub variety: F,
    pub variety_color: Rgb<F>,
    pub varnish: F,
    pub varnish_color: Rgb<F>,
    pub flutes: F,
    pub cross_bedding: F,
    pub pits: F,
    pub pit_size: F,
    pub fractal: F,
    pub fractal_size: F,
    pub fractal_h: F,
    pub crackle: F,
    pub crackle_size: F,
    /// The noise's offset for its seed, and the rotation between its octaves, row by row.
    pub offset: (F, F, F),
    pub rotation: [F; 9],
    /// Noise levels above which the grains and the lichen are; whether there are grains.
    pub dark_threshold: F,
    pub light_threshold: F,
    pub lichen_threshold: F,
    pub dark_grains: bool,
    pub light_grains: bool,
    pub perm: u32,
    pub rotations: u32,
    pub beds: u32,
    pub bed_count: u32,
    pub layers: u32,
    pub layer_count: u32,
}

/// The renderer's rock's own smoothstep: a reversed range (`e1` below `e0`) falls.
#[inline]
pub fn smoothstep<F: Real>(e0: F, e1: F, x: F) -> F {
    let t = ((x - e0) / (e1 - e0)).clamp(F::of(0), F::of(1));
    t * t * (F::of(3) - F::of(2) * t)
}

/// `Color::blend`: a towards b by t, clamped to [0, 1].
#[inline]
pub fn blend<F: Real>(a: Rgb<F>, b: Rgb<F>, t: F) -> Rgb<F> {
    let t = t.clamp(F::of(0), F::of(1));
    let s = F::of(1) - t;
    (a.0 * s + b.0 * t, a.1 * s + b.1 * t, a.2 * s + b.2 * t)
}

#[inline]
fn scaled<F: Real>(c: Rgb<F>, k: F) -> Rgb<F> {
    (c.0 * k, c.1 * k, c.2 * k)
}

/// `color::luminance`.
#[inline]
fn luminance<F: Real>(c: Rgb<F>) -> F {
    F::ratio(2126, 10000) * c.0 + F::ratio(7152, 10000) * c.1 + F::ratio(722, 10000) * c.2
}

/// A point.
pub type P<F> = (F, F, F);

#[inline]
fn add<F: Real>(a: P<F>, b: P<F>) -> P<F> {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

#[inline]
fn sub<F: Real>(a: P<F>, b: P<F>) -> P<F> {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

#[inline]
fn times<F: Real>(a: P<F>, k: F) -> P<F> {
    (a.0 * k, a.1 * k, a.2 * k)
}

#[inline]
fn over<F: Real>(a: P<F>, k: F) -> P<F> {
    (a.0 / k, a.1 / k, a.2 / k)
}

#[inline]
fn dot<F: Real>(a: P<F>, b: P<F>) -> F {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

/// (a, b, c), from ratios of integers: an offset in the renderer's code.
#[inline]
fn at<F: Real>(a: i32, b: i32, c: i32) -> P<F> {
    (F::of(a), F::of(b), F::of(c))
}

/// Rows `r` times `v`, a row's products added in order, as nalgebra multiplies.
#[inline]
fn rotate<F: Real>(r: &[F; 9], v: P<F>) -> P<F> {
    (r[0] * v.0 + r[1] * v.1 + r[2] * v.2, r[3] * v.0 + r[4] * v.1 + r[5] * v.2, r[6] * v.0 + r[7] * v.1 + r[8] * v.2)
}

/// The bits of (n as u64) times K modulo 2^64, shifted down 33 (`as u32`): the renderer's
/// beds' hash, `(n as i64).wrapping_mul(K as i64) as u64 >> 33`, in 32-bit halves, which a
/// kernel has where it has no 64-bit integers.
fn bed_hash(n: i32) -> u32 {
    const K_HI: u32 = 0x9e37_79b9;
    const K_LO: u32 = 0x7f4a_7c15;
    let (a_hi, a_lo) = (if n < 0 { u32::MAX } else { 0 }, n as u32);
    // a_lo * K_LO in full, by 16-bit halves.
    let (x1, x0, y1, y0) = (a_lo >> 16, a_lo & 0xffff, K_LO >> 16, K_LO & 0xffff);
    let (p00, p01, p10, p11) = (x0 * y0, x0 * y1, x1 * y0, x1 * y1);
    let middle = (p00 >> 16) + (p01 & 0xffff) + (p10 & 0xffff);
    let hi = p11 + (p01 >> 16) + (p10 >> 16) + (middle >> 16);
    let hi = hi.wrapping_add(a_hi.wrapping_mul(K_LO)).wrapping_add(a_lo.wrapping_mul(K_HI));
    hi >> 1
}

/// Towards zero, as `f64::trunc`.
#[inline]
fn trunc<F: Real>(x: F) -> F {
    if x < F::of(0) { -((-x).floor()) } else { x.floor() }
}

impl<F: Real> Rock<F> {
    /// `SeededNoise::noise`: gradient noise, roughly in [-1, 1].
    #[inline]
    pub fn noise<T: Tables<F>>(&self, t: &T, p: P<F>) -> F {
        let q = add(p, self.offset);
        perlin(t, self.perm, q.0, q.1, q.2)
    }

    /// `SeededNoise::fbm`.
    pub fn fbm<T: Tables<F>>(&self, t: &T, p: P<F>, octaves: u32) -> F {
        let (mut sum, mut amplitude, mut total, mut q) = (F::of(0), F::of(1), F::of(0), p);
        let mut k = 0;
        while k < octaves {
            sum = sum + self.noise(t, q) * amplitude;
            total = total + amplitude;
            amplitude = amplitude * F::ratio(1, 2);
            q = times(rotate(&self.rotation, q), F::ratio(203, 100));
            k += 1;
        }
        if total > F::of(0) { sum / total } else { F::of(0) }
    }

    /// `SeededNoise::ridged`.
    pub fn ridged<T: Tables<F>>(&self, t: &T, p: P<F>, octaves: u32) -> F {
        let (mut sum, mut amplitude, mut total, mut weight, mut q) = (F::of(0), F::of(1), F::of(0), F::of(1), p);
        let mut k = 0;
        while k < octaves {
            let n = F::of(1) - self.noise(t, q).abs();
            let r = n * n;
            sum = sum + r * amplitude * weight;
            total = total + amplitude;
            weight = (r * F::of(2)).clamp(F::of(0), F::of(1));
            amplitude = amplitude * F::ratio(1, 2);
            q = times(rotate(&self.rotation, q), F::ratio(203, 100));
            k += 1;
        }
        if total > F::of(0) { (sum / total).clamp(F::of(0), F::of(1)) } else { F::of(0) }
    }

    /// How far `q` lies in a crack: 1 at its centre, 0 away from cracks.
    fn crack<T: Tables<F>>(&self, t: &T, q: P<F>) -> F {
        if self.cracks <= F::of(0) {
            return F::of(0);
        }
        let c = F::of(1) - self.noise(t, add(times(q, F::ratio(23, 10)), at(0, 17, 0))).abs();
        let presence = smoothstep(F::ratio(-1, 10), F::ratio(3, 10), self.noise(t, add(times(q, F::ratio(9, 10)), at(5, 0, 0))));
        smoothstep(F::of(1) - F::ratio(1, 10) * self.cracks, F::of(1), c) * presence
    }

    /// The relief, with fractal detail down to wavelength `finest` (none if it isn't below
    /// `Real::far`), in units of `scale`.
    fn relief_to<T: Tables<F>>(&self, t: &T, q: P<F>, finest: F) -> F {
        let fine = if finest < F::far() {
            self.fractal_relief(t, q, finest) - self.crackle * self.crackle_size * F::ratio(5, 100) * self.crackle_at(t, q)
        } else {
            F::of(0)
        };
        let b = over(q, self.bump_size);
        let flutes = if self.flutes > F::of(0) {
            let f = add((q.0 * F::ratio(6, 10), q.1 * F::ratio(5, 100), q.2 * F::ratio(6, 10)), at(0, 0, 23));
            let g = F::of(1) - self.fbm(t, f, 3).abs();
            self.flutes * (g * g) * F::ratio(1, 2)
        } else {
            F::of(0)
        };
        let pits = if self.pits > F::of(0) { -self.pit(t, q) * self.pit_size * F::ratio(6, 10) } else { F::of(0) };
        fine + self.bump * (flutes + pits + self.fbm(t, b, 4) * self.bump_size * F::ratio(6, 10)
            + self.ridged(t, add(times(b, F::ratio(1, 2)), at(0, 0, 9)), 3) * self.bump_size * F::ratio(3, 10)
            - self.crack(t, q) * self.bump_size * F::ratio(15, 100) * self.cracks)
    }

    /// Fractal relief at `q`, its octaves stopping at wavelength `finest`.
    pub fn fractal_relief<T: Tables<F>>(&self, t: &T, q: P<F>, finest: F) -> F {
        if self.fractal <= F::of(0) {
            return F::of(0);
        }
        let (mut sum, mut wavelength, mut amplitude) = (F::of(0), self.fractal_size, self.fractal_size);
        let falloff = F::ratio(1, 2).powf(self.fractal_h);
        let mut k = 0;
        while k < 16 {
            let fade = (wavelength / finest.max(F::ratio(1, 1_000_000_000)) - F::of(1)).clamp(F::of(0), F::of(1));
            if fade <= F::of(0) {
                break;
            }
            let base = self.rotations + 9 * k;
            let r = [t.value(base), t.value(base + 1), t.value(base + 2), t.value(base + 3), t.value(base + 4),
                     t.value(base + 5), t.value(base + 6), t.value(base + 7), t.value(base + 8)];
            let kf = F::of(k as i32);
            let n = self.noise(t, add(over(rotate(&r, q), wavelength), (F::ratio(173, 10) * kf, F::of(0), F::of(5))));
            sum = sum + amplitude * fade * (F::ratio(1, 2) - n.abs());
            wavelength = wavelength * F::ratio(1, 2);
            amplitude = amplitude * falloff;
            k += 1;
        }
        self.fractal * F::ratio(3, 10) * sum
    }

    /// The fracture pattern at `q`: 1 on a crack between Worley cells. Its cells are hashed by
    /// the sine of large arguments, which f32 doesn't follow: the kernels don't take a rock with
    /// crackle (`Rock::flat`).
    pub fn crackle_at<T: Tables<F>>(&self, t: &T, q: P<F>) -> F {
        if self.crackle <= F::of(0) {
            return F::of(0);
        }
        let u = over(q, self.crackle_size.max(F::ratio(1, 1_000_000_000)));
        let cell = (u.0.floor(), u.1.floor(), u.2.floor());
        let (mut f1, mut f2) = (F::far(), F::far());
        let mut i = -1;
        while i <= 1 {
            let mut j = -1;
            while j <= 1 {
                let mut k = -1;
                while k <= 1 {
                    let c = add(cell, at(i, j, k));
                    let h = |salt: F| {
                        let x = (c.0 * F::ratio(1271, 10) + c.1 * F::ratio(3117, 10) + c.2 * F::ratio(747, 10) + salt).sin() * F::ratio(437_585_453, 10_000);
                        (x - trunc(x)).abs()
                    };
                    let d = sub(add(c, (h(F::of(0)), h(F::of(13)), h(F::of(29)))), u);
                    let d = dot(d, d);
                    if d < f1 {
                        f2 = f1;
                        f1 = d;
                    } else if d < f2 {
                        f2 = d;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        let wall = f2.sqrt() - f1.sqrt();
        let presence = smoothstep(F::ratio(-3, 10), F::ratio(2, 10), self.noise(t, add(times(u, F::ratio(7, 10)), at(3, 0, 11))));
        smoothstep(F::ratio(8, 100), F::of(0), wall) * presence
    }

    /// A kind of grain's share at `g` (in grains), if there are any.
    fn grain<T: Tables<F>>(&self, t: &T, g: P<F>, on: bool, threshold: F, offset: P<F>) -> F {
        if on { smoothstep(threshold, threshold + F::ratio(5, 100), self.noise(t, add(g, offset))) } else { F::of(0) }
    }

    /// The relief's slope at `q` along `axis`, by central differences `e` apart.
    fn slope<T: Tables<F>>(&self, t: &T, q: P<F>, axis: P<F>, e: F, finest: F) -> F {
        let step = times(axis, e);
        (self.relief_to(t, add(q, step), finest) - self.relief_to(t, sub(q, step), finest)) / (F::of(2) * e)
    }

    /// Layer `k`'s height and colour.
    fn layer<T: Tables<F>>(&self, t: &T, k: u32) -> (F, Rgb<F>) {
        let at = self.layers + 4 * k;
        (t.value(at), (t.value(at + 1), t.value(at + 2), t.value(at + 3)))
    }

    /// How far `q` lies in a pit: 1 at its middle, 0 outside pits.
    fn pit<T: Tables<F>>(&self, t: &T, q: P<F>) -> F {
        let size = self.pit_size.max(F::ratio(1, 1_000_000_000));
        let patch = smoothstep(F::ratio(-2, 10), F::ratio(3, 10),
            self.fbm(t, add(over(q, F::of(12) * size), at(0, 70, 0)), 2) + self.pits - F::ratio(1, 2));
        let n = self.noise(t, add(over(q, size), at(13, 0, 7)));
        smoothstep(F::ratio(3, 10), F::ratio(55, 100), n) * patch
    }

    /// The shading normal at `p`: `normal` tilted by the relief, with fractal detail no finer
    /// than `width` (about a pixel where the surface is seen; 0 for the coarse relief alone).
    pub fn bumped_normal<T: Tables<F>>(&self, t: &T, p: P<F>, normal: P<F>, width: F) -> P<F> {
        if self.bump <= F::of(0) && self.fractal <= F::of(0) && self.crackle <= F::of(0) {
            return normal;
        }
        let q = over(p, self.scale);
        let finest = if width > F::of(0) { F::of(2) * width / self.scale } else { F::far() };
        let mut e = self.bump_size * F::ratio(5, 100);
        if finest < F::far() && (self.fractal > F::of(0) || self.crackle > F::of(0)) {
            e = e.min(F::ratio(1, 4) * finest);
        }
        let (zero, one) = (F::of(0), F::of(1));
        let g = (self.slope(t, q, (one, zero, zero), e, finest), self.slope(t, q, (zero, one, zero), e, finest), self.slope(t, q, (zero, zero, one), e, finest));
        let tangential = sub(g, times(normal, dot(g, normal)));
        let tilted = sub(normal, tangential);
        let norm = dot(tilted, tilted).sqrt();
        if norm <= F::ratio(1, 1_000_000_000) * F::ratio(1, 1000) {
            normal
        } else {
            over(tilted, norm)
        }
    }

    /// The albedo at `p` as far as the cross-bedding: the body colours mottled, the layers by
    /// elevation, stains and bleaching, and the strata. With the strata's phase, which the
    /// cross-bedding doesn't need, it being of its own laminae.
    pub fn albedo_before<T: Tables<F>>(&self, t: &T, p: P<F>) -> Rgb<F> {
        let q = over(p, self.scale);
        let mottle = smoothstep(F::ratio(-4, 10), F::ratio(4, 10), self.fbm(t, times(q, F::ratio(13, 10)), 5));
        let mut c = blend(self.color, self.secondary, mottle);
        if self.layer_count > 0 {
            let w = add(over((p.0, F::of(0), p.2), F::of(40) * self.scale.max(F::ratio(1, 1_000_000_000))), at(9, 0, 0));
            let y = p.1 + self.layer_warp * self.fbm(t, w, 3);
            // The first layer at or above y (`partition_point`).
            let mut k = 0;
            while k < self.layer_count && self.layer(t, k).0 < y {
                k += 1;
            }
            let layer = if k == 0 {
                self.layer(t, 0).1
            } else if k == self.layer_count {
                self.layer(t, k - 1).1
            } else {
                let ((y0, c0), (y1, c1)) = (self.layer(t, k - 1), self.layer(t, k));
                blend(c0, c1, smoothstep(F::of(0), F::of(1), (y - y0) / (y1 - y0)))
            };
            let tone = luminance(c) / luminance(self.color).max(F::ratio(1, 1_000_000_000));
            c = scaled(layer, tone);
        }
        if self.variety > F::of(0) {
            let v = self.fbm(t, add(times(q, F::ratio(12, 100)), at(0, 51, 0)), 4);
            let shift = F::ratio(7, 10) * self.variety;
            c = blend(c, self.variety_color, smoothstep(F::ratio(35, 100) - shift, F::ratio(6, 10) - shift, v) * F::ratio(85, 100));
        }
        if self.strata_amount > F::of(0) {
            let y = q.1 + self.fbm(t, add(times(q, F::ratio(6, 10)), at(0, 3, 0)), 3) * self.strata_spacing * F::of(2);
            let phase = y / self.strata_spacing;
            let tau = F::of(2) * F::pi();
            let s = F::ratio(1, 2) + F::ratio(1, 2) * (phase * tau).sin();
            let band = s * s * s;
            let fine = F::ratio(1, 2) + F::ratio(1, 2) * (phase * F::ratio(47, 10) * tau).sin();
            let bed = if self.bed_count == 0 {
                self.strata_color
            } else {
                // Each bed its own colour, from a hash of its number (as the renderer's, but
                // that its number is an i32: beds two thousand million strata away repeat).
                let k = bed_hash((phase + F::ratio(1, 4)).floor().to_i32()) % self.bed_count;
                (t.value(self.beds + 3 * k), t.value(self.beds + 3 * k + 1), t.value(self.beds + 3 * k + 2))
            };
            c = blend(c, bed, self.strata_amount * (F::ratio(3, 4) * band + F::ratio(1, 4) * fine) * (F::of(1) - self.cross_bedding));
        }
        c
    }

    /// The albedo from the cross-bedding's on: grains, weathering and cracks, grime in the
    /// fractures and creases, pits, varnish, lichen, and wetness; `normal` the shading normal.
    pub fn albedo_after<T: Tables<F>>(&self, t: &T, p: P<F>, normal: P<F>, c: Rgb<F>) -> Rgb<F> {
        let q = over(p, self.scale);
        let g = over(q, self.grain_size);
        let mut c = blend(c, self.dark_grain, self.grain(t, g, self.dark_grains, self.dark_threshold, at(0, 0, 0)));
        c = blend(c, self.light_grain, self.grain(t, g, self.light_grains, self.light_threshold, (F::ratio(314, 10), F::of(0), F::of(0))));
        let tone = F::of(1) + self.weathering * self.fbm(t, add(times(q, F::ratio(1, 4)), at(0, 0, 40)), 3);
        c = scaled(c, tone * (F::of(1) - F::ratio(6, 10) * self.crack(t, q)));
        if self.crackle > F::of(0) {
            c = scaled(c, F::of(1) - F::ratio(55, 100) * self.crackle * self.crackle_at(t, q));
        }
        if self.fractal > F::of(0) {
            let crease = self.fractal_relief(t, q, F::ratio(1, 4) * self.fractal_size) / (F::ratio(3, 10) * self.fractal * self.fractal_size);
            c = scaled(c, F::of(1) + F::ratio(6, 10) * crease.clamp(F::ratio(-1, 2), F::ratio(1, 2)));
        }
        if self.pits > F::of(0) {
            c = scaled(c, F::of(1) - F::ratio(6, 10) * self.pit(t, q));
        }
        if self.varnish > F::of(0) {
            let steep = smoothstep(F::ratio(35, 100), F::ratio(75, 100), F::of(1) - normal.1.abs());
            let f = add((q.0 * F::ratio(35, 100), q.1 * F::ratio(3, 100), q.2 * F::ratio(35, 100)), at(40, 0, 0));
            let streak = smoothstep(F::ratio(5, 100), F::ratio(35, 100), self.fbm(t, f, 4) + F::ratio(3, 10) * (self.varnish - F::ratio(1, 2)));
            c = blend(c, self.varnish_color, F::ratio(8, 10) * steep * streak * self.varnish.min(F::of(1)));
        }
        if self.lichen > F::of(0) {
            let n = self.fbm(t, add(times(q, F::of(3)), at(70, 0, 0)), 4) + F::ratio(3, 100) * self.noise(t, times(q, F::of(60)));
            let lt = self.lichen_threshold;
            let patch = smoothstep(lt - F::ratio(8, 1000), lt + F::ratio(8, 1000), n);
            let rim = smoothstep(lt + F::ratio(6, 100), lt + F::ratio(1, 100), n);
            let up = smoothstep(F::ratio(-2, 10), F::ratio(6, 10), normal.1);
            let grain = F::ratio(85, 100) + F::ratio(3, 10) * self.noise(t, times(q, F::of(40)));
            let lichen = scaled(blend(self.lichen_color, (F::ratio(6, 10), F::ratio(6, 10), F::ratio(52, 100)), F::ratio(4, 10) * rim), grain);
            c = blend(c, lichen, patch * up);
        }
        c = scaled(c, F::of(1) - F::ratio(45, 100) * self.wetness);
        let most = F::ratio(88, 100);
        (c.0.min(most), c.1.min(most), c.2.min(most))
    }
}
