//! Procedural soil: the ground under grass and trees.
//!
//! Like `rock`, soil is a solid texture (its colour a function of position, so it needs no
//! surface coordinates), built from what makes bare ground read as ground at a glance rather
//! than as a flat brown:
//!
//! - mottling between drier, paler topsoil and dark humus, at two scales;
//! - clods: crumbly relief, bump mapped;
//! - stones: small pebbles lying in the soil, each its own tone, raised in the relief;
//! - crumbs: dark specks of organic matter;
//! - litter: bits of dead grass, straw and leaf lying on the surface. Each is a short fibre
//!   with its own position, direction and tone, found by hashing the cells of a grid around
//!   the point (like Steven Worley's cellular texture, "A Cellular Texture Basis Function",
//!   SIGGRAPH 1996), so they have crisp edges at any distance;
//! - moss, in patches, on surfaces facing the sky;
//! - wetness, which darkens it and adds a film of water, as for `rock`.
//!
//! Reflectances of soils, in linear RGB, run from about 0.05 for wet peat to 0.35 for dry
//! sand; brown loams are about 0.1 to 0.2 (eg. the soils in the USGS spectral library, Kokaly
//! et al. 2017, and E. Ben-Dor's reviews of soil reflectance).

use crate::material::model::LightSampling;
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::geometry::{rand, random_point_on_unit_sphere, random_point_in_unit_ball};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{direct_lighting, fresnel_reflectance, reflect};

/// The look of a kind of soil. Lengths are in metres (scaled by the material's `scale`).
#[derive(Clone, Debug, PartialEq)]
pub struct SoilStyle {
    /// Dry topsoil, and the dark humus it is mottled with.
    pub color: Color,
    pub humus: Color,
    /// Size of the broad mottling.
    pub mottle_size: f64,
    /// Stones: their colour, the fraction of the ground they cover, and their size.
    pub stone_color: Color,
    pub stones: f64,
    pub stone_size: f64,
    /// Dark crumbs of organic matter, as a fraction of the surface.
    pub crumb_color: Color,
    pub crumbs: f64,
    /// Litter: its colour, the fraction of the ground it covers, and the length and width of
    /// its bits.
    pub litter_color: Color,
    pub litter: f64,
    pub litter_length: f64,
    pub litter_width: f64,
    /// Moss: colour, and the fraction of upward surfaces it covers.
    pub moss_color: Color,
    pub moss: f64,
    /// Depth of the relief, and the size of its clods.
    pub bump: f64,
    pub clod_size: f64,
}

impl SoilStyle {
    pub fn preset(name: &str) -> SoilStyle {
        let loam = SoilStyle {
            color: Color::new(0.17, 0.12, 0.08), humus: Color::new(0.07, 0.05, 0.035), mottle_size: 0.6,
            stone_color: Color::new(0.24, 0.22, 0.19), stones: 0.02, stone_size: 0.012,
            crumb_color: Color::new(0.04, 0.03, 0.02), crumbs: 0.15,
            litter_color: Color::new(0.3, 0.24, 0.14), litter: 0.45, litter_length: 0.04, litter_width: 0.0025,
            moss_color: Color::new(0.06, 0.09, 0.03), moss: 0.12,
            bump: 0.6, clod_size: 0.025,
        };
        match name {
            // Brown meadow topsoil, with dead grass on it and a little moss.
            "loam" => loam,
            // Orange-brown and smoother, with fewer stones.
            "clay" => SoilStyle {
                color: Color::new(0.26, 0.14, 0.07), humus: Color::new(0.14, 0.08, 0.045),
                stones: 0.02, crumbs: 0.06, litter: 0.15, moss: 0.05, bump: 0.35, clod_size: 0.04,
                ..loam
            },
            // Pale and gritty, little organic matter.
            "sand" => SoilStyle {
                color: Color::new(0.38, 0.31, 0.21), humus: Color::new(0.27, 0.21, 0.14), mottle_size: 1.,
                stone_color: Color::new(0.5, 0.46, 0.4), stones: 0.1, stone_size: 0.006,
                crumb_color: Color::new(0.15, 0.12, 0.09), crumbs: 0.08,
                litter_color: Color::new(0.4, 0.34, 0.24), litter: 0.08, moss: 0., bump: 0.25, clod_size: 0.02,
                ..loam
            },
            // Under trees: dark humus, covered in fallen leaves and needles, and mossy.
            "forest_floor" => SoilStyle {
                color: Color::new(0.11, 0.075, 0.05), humus: Color::new(0.05, 0.035, 0.025),
                stones: 0.02, crumbs: 0.2,
                litter_color: Color::new(0.3, 0.17, 0.08), litter: 0.6, litter_length: 0.035, litter_width: 0.006,
                moss_color: Color::new(0.055, 0.09, 0.025), moss: 0.3,
                ..loam
            },
            // Desert floor: red-orange, iron-stained sand and silt, scattered with the dark,
            // varnished stones of desert pavement and a little dry plant debris.
            "desert" => SoilStyle {
                color: Color::new(0.46, 0.26, 0.14), humus: Color::new(0.38, 0.22, 0.12), mottle_size: 2.,
                stone_color: Color::new(0.2, 0.12, 0.08), stones: 0.12, stone_size: 0.01,
                crumb_color: Color::new(0.3, 0.18, 0.1), crumbs: 0.05,
                litter_color: Color::new(0.45, 0.38, 0.26), litter: 0.04, moss: 0., bump: 0.3, clod_size: 0.03,
                ..loam
            },
            _ => panic!("Unknown soil preset '{}': expected loam, clay, sand, desert or forest_floor", name),
        }
    }
}

pub struct Soil {
    pub style: SoilStyle,
    /// Scale for the texture's lengths: 1 when the scene is in metres.
    pub scale: f64,
    /// 0 for dry, 1 for soaked.
    pub wetness: f64,
    noise: SeededNoise,
    seed: u64,
    crumb_threshold: f64,
    moss_threshold: f64,
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// SplitMix64.
fn hash(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Random numbers in [0, 1) for one cell of a grid.
struct CellRandom(u64);

impl CellRandom {
    fn new(seed: u64, layer: u64, ix: i64, iz: i64) -> CellRandom {
        CellRandom(hash(seed ^ hash(layer ^ hash((ix as u64) ^ hash(iz as u64)))))
    }

    fn next(&mut self) -> f64 {
        self.0 = hash(self.0);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Refractive index of water.
const WATER: f64 = 1.33;
/// Most litter bits tested in a cell.
const MAX_LITTER: usize = 24;

impl Soil {
    pub fn new(style: SoilStyle, scale: f64, seed: u64) -> Soil {
        let noise = SeededNoise::new(seed);
        // Noise levels exceeded by the given fraction of points, found from a sample.
        let quantile = |fraction: f64, sample: &dyn Fn(Vector3<f64>) -> f64| {
            if fraction <= 0. { return f64::INFINITY }
            let mut rng = crate::geometry::Pcg32::new(seed, 7);
            let mut v: Vec<f64> = (0 .. 4096).map(|_| {
                sample(Vector3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) * 50.)
            }).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[((1. - fraction.min(1.)) * (v.len() - 1) as f64) as usize]
        };
        let crumb_threshold = quantile(style.crumbs, &|p| noise.noise(p));
        let moss_threshold = quantile(style.moss, &|p| noise.fbm(p + Vector3::new(70., 0., 0.), 4));
        Soil { style, scale, wetness: 0., noise, seed, crumb_threshold, moss_threshold }
    }

    /// The stone at q (scaled), if any: its height above the soil, and its tone.
    fn stone(&self, q: Vector3<f64>) -> Option<(f64, f64)> {
        let st = &self.style;
        if st.stones <= 0. { return None }
        // At most one stone per cell, of a radius up to half the cell's size, so it is only in
        // its own cell or those next to it.
        let cell = st.stone_size * 2.;
        let (cx, cz) = ((q.x / cell).floor() as i64, (q.z / cell).floor() as i64);
        // A stone covers on average 0.17 of its cell: pi E[r^2] E[squash] / cell^2, with r
        // from a quarter to three quarters of the stone size.
        let present = (st.stones / 0.17).min(1.);
        let mut best: Option<(f64, f64)> = None;
        for dz in -1 ..= 1 {
            for dx in -1 ..= 1 {
                let mut r = CellRandom::new(self.seed, 1, cx + dx, cz + dz);
                if r.next() >= present { continue }
                let centre = ((cx + dx) as f64 + r.next(), (cz + dz) as f64 + r.next());
                let radius = st.stone_size * (0.25 + 0.5 * r.next());
                let tone = r.next();
                // Stones are elongated, at random angles.
                let (a, squash) = (r.next() * std::f64::consts::PI, 0.6 + 0.4 * r.next());
                let (ox, oz) = (q.x - centre.0 * cell, q.z - centre.1 * cell);
                let (u, v) = (ox * a.cos() + oz * a.sin(), (oz * a.cos() - ox * a.sin()) / squash);
                let d2 = (u * u + v * v) / (radius * radius);
                if d2 < 1. {
                    let h = radius * 0.6 * (1. - d2).sqrt();
                    if best.is_none_or(|(bh, _)| h > bh) { best = Some((h, tone)) }
                }
            }
        }
        best
    }

    /// The litter at q (scaled), if any: how far across the bit the point is (0 at its middle,
    /// 1 at its edge), and its tone.
    fn litter(&self, q: Vector3<f64>) -> Option<(f64, f64)> {
        let st = &self.style;
        if st.litter <= 0. { return None }
        let cell = st.litter_length;
        let (cx, cz) = ((q.x / cell).floor() as i64, (q.z / cell).floor() as i64);
        // A bit covers on average 0.56 width / length of its cell (its length is 0.4 to 1 of
        // litter_length, and it tapers), and bits overlap, covering 1 - exp(-total) between
        // them.
        let per_cell = -(1. - st.litter.min(0.95)).ln() / 0.56 * st.litter_length / st.litter_width;
        let mut best: Option<(f64, f64)> = None;
        for dz in -1 ..= 1 {
            for dx in -1 ..= 1 {
                let mut r = CellRandom::new(self.seed, 2, cx + dx, cz + dz);
                let n = (per_cell.floor() as usize + usize::from(r.next() < per_cell.fract())).min(MAX_LITTER);
                for _ in 0 .. n {
                    let centre = (((cx + dx) as f64 + r.next()) * cell, ((cz + dz) as f64 + r.next()) * cell);
                    let a = r.next() * std::f64::consts::PI;
                    let half = 0.5 * st.litter_length * (0.4 + 0.6 * r.next());
                    let width = st.litter_width * (0.6 + 0.8 * r.next());
                    let tone = r.next();
                    // A slight curve along the bit.
                    let bend = (r.next() - 0.5) * 0.5;
                    let (ox, oz) = (q.x - centre.0, q.z - centre.1);
                    let along = ox * a.cos() + oz * a.sin();
                    if along.abs() > half { continue }
                    let across = oz * a.cos() - ox * a.sin() - bend * along * along / half;
                    // Bits taper to their ends.
                    let w = 0.5 * width * (1. - (along / half).powi(4));
                    let edge = across.abs() / w;
                    if edge < 1. && best.is_none_or(|(be, _)| edge < be) { best = Some((edge, tone)) }
                }
            }
        }
        best
    }

    /// Height of the relief, in metres (scaled).
    fn relief(&self, q: Vector3<f64>) -> f64 {
        let st = &self.style;
        let c = q / st.clod_size;
        let clods = self.noise.fbm(c, 3) * st.clod_size * 0.4
            + (1. - self.noise.noise(c * 0.5 + Vector3::new(0., 9., 0.)).abs()) * st.clod_size * 0.2;
        clods + self.stone(q).map_or(0., |(h, _)| h)
    }

    /// The shading normal: the surface normal tilted by the relief.
    pub fn bumped_normal(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        if self.style.bump <= 0. { return normal }
        let q = p / self.scale;
        let e = self.style.clod_size * 0.05;
        let d = |axis: Vector3<f64>| (self.relief(q + axis * e) - self.relief(q - axis * e)) / (2. * e);
        let g = Vector3::new(d(Vector3::x()), d(Vector3::y()), d(Vector3::z()));
        let tangential = g - normal * g.dot(&normal);
        (normal - tangential * self.style.bump).try_normalize(1e-12).unwrap_or(normal)
    }

    /// The diffuse reflectance at a point, given its shading normal.
    pub fn albedo(&self, p: Vector3<f64>, normal: Vector3<f64>) -> Color {
        let st = &self.style;
        let q = p / self.scale;

        // Broad mottling between topsoil and humus, and a finer, patchier one.
        let broad = smoothstep(-0.35, 0.35, self.noise.fbm(q / st.mottle_size, 4));
        let fine = self.noise.fbm(q / (st.clod_size * 3.) + Vector3::new(0., 0., 20.), 3);
        let mut c = st.humus.blend(&st.color, broad * (0.75 + 0.5 * fine));
        // The tops of clods are drier and paler than the hollows between them.
        let clod = self.noise.fbm(q / st.clod_size, 3);
        c = c * (1. + 0.35 * clod);

        // Dark crumbs.
        if self.crumb_threshold.is_finite() {
            let n = self.noise.noise(q / (st.clod_size * 0.15));
            c = c.blend(&st.crumb_color, smoothstep(self.crumb_threshold, self.crumb_threshold + 0.05, n));
        }

        // Moss, in patches facing the sky, before the stones and litter lying on it.
        if self.moss_threshold.is_finite() {
            let n = self.noise.fbm(q / 0.15 + Vector3::new(70., 0., 0.), 4);
            let patch = smoothstep(self.moss_threshold - 0.02, self.moss_threshold + 0.04, n);
            let up = smoothstep(0., 0.6, normal.y);
            // Ragged at the edges, and mottled with paler, yellower shoots.
            let ragged = smoothstep(-0.2, 0.3, self.noise.fbm(q / 0.01 + Vector3::new(0., 40., 0.), 3) + 2. * (patch - 0.5));
            let shoots = smoothstep(-0.3, 0.5, self.noise.noise(q / 0.004));
            let moss = st.moss_color.blend(&Color::new(st.moss_color.rgb.y * 1.2, st.moss_color.rgb.y * 1.25, st.moss_color.rgb.z), 0.5 * shoots);
            c = c.blend(&moss, ragged * up);
        }

        if let Some((h, tone)) = self.stone(q) {
            // Paler on top, darker at the rim where it sinks into the soil.
            let rim = smoothstep(0., st.stone_size * 0.15, h);
            c = st.stone_color * ((0.55 + 0.6 * tone) * (0.5 + 0.5 * rim));
        }

        if let Some((edge, tone)) = self.litter(q) {
            // Weathered bits are greyer, fresh ones yellower; each is darker at its edges.
            let grey = st.litter_color.blend(&Color::new(0.24, 0.21, 0.16), 0.5);
            let bit = st.litter_color.blend(&grey, tone) * (0.6 + 0.4 * tone) * ((0.7 + 0.6 * self.noise.noise(q / 0.004).abs()) * (1. - 0.4 * edge * edge));
            c = bit;
        }

        c = c * (1. - 0.5 * self.wetness);
        c.clamp(0.8)
    }
}

impl MaterialModel for Soil {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let geometric = intersection.facing_normal(r);
        let normal = self.bumped_normal(intersection.point, geometric);

        if self.wetness > 0. {
            let cos_i = (r.rd.dot(&normal) / r.rd.norm()).abs();
            let film = self.wetness * fresnel_reflectance(cos_i, 1. / WATER, WATER);
            if rand() < film {
                let rd = reflect(r.rd.normalize(), normal) + random_point_in_unit_ball() * 0.08;
                if rd.dot(&geometric) <= 0. {
                    return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::black(), ray: None };
                }
                return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(),
                    ray: Some(Ray { ro: intersection.point, rd }) };
            }
        }

        let albedo = self.albedo(intersection.point, normal);
        let shaded = Intersection { normal, ..*intersection };
        let mut rd = normal + random_point_on_unit_sphere();
        if rd.norm() < 1e-9 { rd = normal }
        let into = rd.dot(&geometric);
        if into < 0. { rd -= geometric * (2. * into) }
        ScatteredRay {
            emit: direct_lighting(albedo, &shaded, s),
            light_sampling: LightSampling::Exclusive,
            attenuate: albedo,
            ray: Some(Ray { ro: intersection.point, rd: rd.normalize() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRESETS: [&str; 4] = ["loam", "clay", "sand", "forest_floor"];

    #[test]
    fn test_albedo_is_plausible_and_varies() {
        for name in PRESETS {
            let soil = Soil::new(SoilStyle::preset(name), 1., 3);
            let mut rng = crate::geometry::Pcg32::new(0, 0);
            let (mut lo, mut hi) = (f64::INFINITY, 0f64);
            for _ in 0 .. 2000 {
                let p = Vector3::new(rng.next_f64() * 4., 0., rng.next_f64() * 4.);
                let c = soil.albedo(p, Vector3::y());
                for v in c.rgb.iter() {
                    assert!(*v >= 0. && *v <= 0.8, "{} albedo {}", name, c.rgb);
                }
                lo = lo.min(c.rgb.y);
                hi = hi.max(c.rgb.y);
            }
            assert!(hi > lo * 2., "{}: too uniform, {} to {}", name, lo, hi);
        }
    }

    #[test]
    fn test_features_cover_about_their_fractions() {
        let soil = Soil::new(SoilStyle::preset("loam"), 1., 5);
        let mut rng = crate::geometry::Pcg32::new(1, 0);
        let n = 20000;
        let (mut stones, mut litter) = (0, 0);
        for _ in 0 .. n {
            let q = Vector3::new(rng.next_f64() * 3., 0., rng.next_f64() * 3.);
            stones += usize::from(soil.stone(q).is_some());
            litter += usize::from(soil.litter(q).is_some());
        }
        let (stones, litter) = (stones as f64 / n as f64, litter as f64 / n as f64);
        // Overlapping stones make their coverage a little less than asked for.
        assert!(stones > 0.01 && stones < 0.03, "stones cover {}", stones);
        assert!((litter - 0.45).abs() < 0.05, "litter covers {}", litter);
    }

    #[test]
    fn test_bumped_normal_tilts_but_stays_up() {
        let soil = Soil::new(SoilStyle::preset("loam"), 1., 2);
        let mut tilted = 0;
        for i in 0 .. 200 {
            let p = Vector3::new(i as f64 * 0.0137, 0., i as f64 * 0.0071);
            let n = soil.bumped_normal(p, Vector3::y());
            assert!((n.norm() - 1.).abs() < 1e-9 && n.y > 0.3, "{}", n);
            if n.y < 0.999 { tilted += 1 }
        }
        assert!(tilted > 100);
    }
}
