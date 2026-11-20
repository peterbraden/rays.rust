//! Overcast: the thick, grey deck of bad weather, with ragged scud beneath it. A density field
//! for `volume::Volume`.
//!
//! On a wet, windy day the sky is a single sheet of cloud a few kilometres thick: nimbostratus,
//! or stratocumulus grown together, with no blue anywhere. Seen from below it is far from
//! uniform. Its underside is lumpy, hanging in rounded masses a kilometre or two across that
//! line up in rolls along the wind, with lighter seams between; and since the light that
//! reaches the ground has diffused down through the whole deck, a patch of sky is as bright as
//! the deck above it is thin. Where the deck is thickest and hangs lowest its base is darkest,
//! slate grey; where it thins it glows white. Beneath the base, shreds of fractus (scud, or
//! pannus under rain) drift low and fast, catching on hills.
//!
//! The field is:
//!
//! 1. **The deck's thickness.** Broad patches of thicker and thinner cloud over `system_size`,
//!    stretched along the wind, set how far it reaches up from the base and how dense it is:
//!    the large bright and dark areas. `breaks` lets the thinnest parts open altogether.
//! 2. **Lumps.** Perlin-Worley billows (as in `nubis_clouds`) `lump_size` across, stretched into
//!    rolls along the wind by `anisotropy`. Where a lump is strong the base hangs lower
//!    (by up to `base_variation`) and the cloud is denser; between lumps it is higher and
//!    thinner. `contrast` sets how much thinner (0: an even deck, 1: almost clear between).
//! 3. **A ragged base.** The density rises over `softness` metres above the base, which
//!    Worley detail `detail_size` across wears ragged (`erosion`), as the drops evaporate into
//!    the drier air below.
//! 4. **Scud.** Ragged fragments of cloud under the base, down to `scud_depth` below it: fractal
//!    noise thresholded so only its strongest parts (`scud`, the fraction of the sky they
//!    cover) are cloud, drawn out along the wind, thickest just under the base. They carry on
//!    under the `breaks`, as fractus on its own: the shreds of low cloud a wind tears off.
//!
//! Rain falling from it is a separate `precipitation` volume: give it the deck's base.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::{lerp, remap, smoothstep, stretched, worley_fbm, WorleyTexture};
use crate::volume::DensityField;
use std::sync::OnceLock;

/// The parts of the field that don't change with height (`Overcast::column_at`).
#[derive(Clone, Copy)]
struct Column {
    region: f64,
    top: f64,
    local_base: f64,
    body: f64,
    /// The column's position in the wind's frame.
    a: f64,
    c: f64,
}

pub struct Overcast {
    /// The mean height of the deck's base, and its mean thickness (m).
    pub base: f64,
    pub thickness: f64,
    /// Extinction in the densest cloud (per m).
    pub max_density: f64,
    /// The size of the broad patches of thicker and thinner cloud, and how much the deck's
    /// thickness varies with them (0 to 1, as a fraction of `thickness`).
    pub system_size: f64,
    pub thickness_variation: f64,
    /// The size of the lumps on the underside, how far below the mean base the strongest hang,
    /// and how much thinner the cloud is between them (0 to 1).
    pub lump_size: f64,
    pub base_variation: f64,
    pub contrast: f64,
    /// Lumps (and scud) are this many times longer along `wind_direction` ([x, z], unit).
    pub anisotropy: f64,
    pub wind_direction: Vector2<f64>,
    /// How much of the sky (0 to 1, from the thinnest cloud up) opens into gaps.
    pub breaks: f64,
    /// The base: over how many metres the density rises above it, and its ragged detail.
    pub softness: f64,
    pub detail_size: f64,
    pub erosion: f64,
    /// Scud: the fraction of the sky under the deck it covers (0 to 1), how far below the base
    /// it reaches, the size of its fragments, and its extinction.
    pub scud: f64,
    pub scud_depth: f64,
    pub scud_size: f64,
    pub scud_density: f64,
    /// The deck covers `extent` either side of `center` ([x, z]), fading over its outer `fade`
    /// (a fraction of `extent`; 0 for none).
    pub center: Vector2<f64>,
    pub extent: f64,
    pub fade: f64,
    seed: u64,
    noise: PerlinNoise,
    cells: OnceLock<WorleyTexture>,
}

impl Overcast {
    pub fn new(base: f64, thickness: f64, seed: u64) -> Overcast {
        Overcast {
            base,
            thickness,
            max_density: 0.03,
            system_size: 8000.,
            thickness_variation: 0.5,
            lump_size: 1000.,
            base_variation: 300.,
            contrast: 0.8,
            anisotropy: 2.,
            wind_direction: Vector2::new(1., 0.),
            breaks: 0.,
            softness: 120.,
            detail_size: 220.,
            erosion: 0.5,
            scud: 0.25,
            scud_depth: 450.,
            scud_size: 600.,
            scud_density: 0.02,
            center: Vector2::zeros(),
            extent: 60_000.,
            fade: 0.,
            seed,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(11).wrapping_add(5)),
            cells: OnceLock::new(),
        }
    }

    fn cells(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed as u32 ^ 0x5eed))
    }

    /// The lowest any cloud reaches: the deepest lumps' ragged edges, or the scud.
    pub fn bottom(&self) -> f64 {
        let lumps = self.deck_bottom();
        if self.scud > 0. { lumps.min(self.base - 0.6 * self.base_variation - self.scud_depth) } else { lumps }
    }

    /// The lowest the deck itself reaches: the deepest lumps' ragged edges.
    fn deck_bottom(&self) -> f64 {
        self.base - 0.6 * self.base_variation - self.erosion * self.detail_size
    }

    /// The highest scud reaches, into the highest parts of the base.
    fn scud_top(&self) -> f64 {
        self.base + 0.4 * self.base_variation + self.softness
    }

    /// The highest: the thickest part of the deck.
    pub fn top(&self) -> f64 {
        self.base + self.thickness * (1. + self.thickness_variation)
    }

    /// (x, z) in the wind's frame, shortened along it by the anisotropy: lumps and scud are
    /// round in these coordinates and drawn out over the ground.
    fn wind_frame(&self, x: f64, z: f64) -> (f64, f64) {
        let w = self.wind_direction;
        let along = x * w.x + z * w.y;
        let across = -x * w.y + z * w.x;
        (along / self.anisotropy.max(1e-3), across)
    }

    /// The broad thickness of the deck over (x, z): 0 at its thinnest to 1 at its thickest.
    fn system(&self, x: f64, z: f64) -> f64 {
        let (a, c) = self.wind_frame(x, z);
        // Broad patches stretch along the wind less than the lumps do.
        let a = a * self.anisotropy.max(1e-3).sqrt();
        stretched(self.noise.fbm(a / self.system_size, 0.37, c / self.system_size, 3, 0.5, 2.))
    }

    /// The strength of the lump over (x, z), 0 to 1: high under the rounded masses, low in the
    /// seams between them.
    fn lump(&self, x: f64, z: f64) -> f64 {
        let (a, c) = self.wind_frame(x, z);
        let q = Vector3::new(a, 0., c) / self.lump_size;
        // Cells (inverted Worley noise: round masses with seams between), their outlines
        // bent by a domain warp so they aren't a lattice of blobs.
        let warp = |o: f64| (self.noise.fbm(q.x * 0.5, o, q.z * 0.5, 2, 0.5, 2.) - 0.5) * 1.2;
        let w = q + Vector3::new(warp(0.71), 3.3, warp(1.93));
        let cells = self.cells();
        let cell = |p: Vector3<f64>| (1. - cells.at(&p)).clamp(0., 1.);
        // Lumps of lumps: smaller cells crowd the larger ones' undersides.
        let masses = 0.6 * cell(w) + 0.27 * cell(w * 2.3) + 0.13 * cell(w * 5.1);
        // Some masses grow larger and darker than others.
        let strength = stretched(self.noise.fbm(q.x * 0.35, 5.1, q.z * 0.35, 2, 0.5, 2.));
        remap(masses * lerp(0.75, 1.15, strength), 0.2, 0.75)
    }

    /// How much of the deck is here at all: the region, faded at its edge.
    fn region(&self, x: f64, z: f64) -> f64 {
        if self.fade <= 0. {
            return 1.;
        }
        let from_centre = (Vector2::new(x, z) - self.center).norm() / self.extent;
        smoothstep(0., self.fade, 1. - from_centre)
    }

    /// Cloud density as a fraction of `max_density` (the deck) plus scud, as extinction.
    fn extinction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top() {
            return 0.;
        }
        match self.column_at(p.x, p.z) {
            Some(column) if p.y <= column.top => self.extinction_in(&column, p),
            Some(_) => 0.,
            // Under a break in the deck, shreds still drift: fractus, torn from the deck's
            // edges and carried on the wind.
            None if self.scud > 0. && self.breaks > 0. => {
                let region = self.region(p.x, p.z);
                let open_base = self.base + 0.4 * self.base_variation;
                self.scud_density * self.scud_fraction(p, open_base) * region * self.scud
            },
            None => 0.,
        }
    }

    /// What is the same all the way up the column over (x, z); None where there is no cloud.
    fn column_at(&self, x: f64, z: f64) -> Option<Column> {
        let region = self.region(x, z);
        if region <= 0. {
            return None;
        }
        let system = self.system(x, z);
        // Gaps where the deck is thinnest.
        let open = if self.breaks > 0. { smoothstep(self.breaks * 0.8, self.breaks * 0.8 + 0.12, system) } else { 1. };
        if open <= 0. {
            return None;
        }
        let lump = self.lump(x, z);
        // The lumps are the bottoms of thicker cells, which also rise higher.
        let thick = 0.5 * (system + lump);
        let top = self.base + self.thickness * (1. + self.thickness_variation * (2. * thick - 1.));
        let body = lerp(1. - self.contrast, 1., lump) * lerp(0.6, 1., system) * open;
        let (a, c) = self.wind_frame(x, z);
        Some(Column { region, top, local_base: self.base - self.base_variation * (lump - 0.4), body, a, c })
    }

    /// The extinction at `p`, in `column`.
    fn extinction_in(&self, column: &Column, p: &Vector3<f64>) -> f64 {
        let Column { region, top, local_base, body, a, c } = *column;
        if p.y > top {
            return 0.;
        }
        // The ragged base: billowing detail, worn deeper where the base is low.
        let d = worley_fbm(self.cells(), &(Vector3::new(a, p.y, c) / self.detail_size));
        let ragged = local_base + self.erosion * self.detail_size * (0.5 - d) * 2.;
        let mut deck = 0.;
        if p.y > ragged {
            let rise = smoothstep(0., self.softness.max(1e-3), p.y - ragged);
            // Thinning towards the top, which is soft rather than a lid.
            let fall = 1. - smoothstep(top - 0.25 * (top - local_base).max(1.), top, p.y);
            deck = self.max_density * rise * fall * body;
        }
        let scud = if self.scud > 0. && p.y < local_base + self.softness {
            self.scud_density * self.scud_fraction(p, local_base)
        } else {
            0.
        };
        (deck + scud) * region
    }

    /// Scud's density, as a fraction of `scud_density`, at `p` under a base at `local_base`.
    fn scud_fraction(&self, p: &Vector3<f64>, local_base: f64) -> f64 {
        let below = local_base - p.y;
        if self.scud <= 0. || below > self.scud_depth {
            return 0.;
        }
        // Thickest just under the base, petering out lower down.
        let depth = 1. - smoothstep(0.2 * self.scud_depth, self.scud_depth, below);
        if depth <= 0. {
            return 0.;
        }
        let (a, c) = self.wind_frame(p.x, p.z);
        // Fragments are drawn out along the wind even more than the lumps.
        let q = Vector3::new(a / 1.5, p.y * 1.5, c) / self.scud_size;
        let n = stretched(self.noise.fbm(q.x, q.y + 17.1, q.z, 4, 0.55, 2.1));
        let cut = 0.8 - 0.4 * self.scud.clamp(0., 1.);
        let fragment = remap(n * lerp(0.7, 1., depth), cut, cut + 0.15);
        if fragment <= 0. {
            return 0.;
        }
        // Shredded edges.
        let d = worley_fbm(self.cells(), &(q * 3.));
        remap(fragment, 0.6 * (1. - d), 1.)
    }

    fn peak(&self) -> f64 {
        self.max_density + if self.scud > 0. { self.scud_density } else { 0. }
    }
}

impl DensityField for Overcast {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.extinction(p)
    }

    fn majorant(&self) -> f64 {
        self.peak()
    }

    /// By height: nothing below the lowest scud or above the highest top; only scud below the
    /// deck's lowest base, only the deck above the scud's highest top, and both between.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let scud = if self.scud > 0. { self.scud_density } else { 0. };
        let (deck_bottom, scud_top) = (self.deck_bottom(), if self.scud > 0. { self.scud_top() } else { self.bottom() });
        // Bands from the bottom up: (top of the band, majorant in it).
        let mut bands = [
            (self.bottom(), 0.),
            (deck_bottom.min(scud_top), scud),
            (deck_bottom.max(scud_top), if deck_bottom < scud_top { self.peak() } else { 0. }),
            (self.top(), self.max_density),
            (f64::INFINITY, 0.),
        ];
        // Between the scud and the deck, when they don't overlap.
        if deck_bottom >= scud_top {
            bands[2].1 = 0.;
        }
        let k = bands.iter().position(|(top, _)| ahead.y < *top).unwrap_or(bands.len() - 1);
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        let exit = if d.y > 0. {
            to_y(bands[k].0)
        } else if d.y < 0. && k > 0 {
            to_y(bands[k - 1].0)
        } else {
            f64::INFINITY
        };
        (bands[k].1, exit.max(0.) + 0.01)
    }

    /// The column's 2D terms once, rather than at every step.
    fn column_depths(&self, p: &Vector3<f64>, lo: f64, hi: f64, steps: u32) -> (f64, f64) {
        let (lo, hi) = (lo.max(self.bottom()), hi.min(self.top()));
        let column = match self.column_at(p.x, p.z) {
            Some(c) if hi > lo => c,
            _ => return (0., 0.),
        };
        let hi = hi.min(column.top);
        if hi <= lo {
            return (0., 0.);
        }
        let step = (hi - lo) / steps as f64;
        let (mut below, mut above) = (0., 0.);
        for k in 0..steps {
            let y = lo + (k as f64 + crate::geometry::rand()) * step;
            let tau = self.extinction_in(&column, &Vector3::new(p.x, y, p.z)) * step;
            if y < p.y { below += tau } else { above += tau }
        }
        (below, above)
    }

    /// Cloud droplets, about 10 µm across, for diffraction.
    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(5.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn deck() -> Overcast {
        Overcast::new(1000., 2000., 3)
    }

    #[test]
    fn test_density_is_bounded_by_the_majorant() {
        let o = deck();
        let mut rng = Pcg32::new(1, 0);
        for _ in 0..20_000 {
            let p = Vector3::new(rng.next_f64() * 40000. - 20000., rng.next_f64() * 6000., rng.next_f64() * 40000. - 20000.);
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = o.majorant_from(&p, &d);
            assert!(len > 0.);
            for k in 0..20 {
                let q = p + d * (len.min(20_000.) * k as f64 / 20.);
                let dens = o.density(&q);
                assert!(dens >= 0. && dens <= m + 1e-12, "{} > {} at {}", dens, m, q);
            }
        }
        // Nothing outside the slab.
        assert_eq!(o.density(&Vector3::new(0., o.bottom() - 1., 0.)), 0.);
        assert_eq!(o.density(&Vector3::new(0., o.top() + 1., 0.)), 0.);
    }

    /// The deck's optical depth straight up from (x, z).
    fn column(o: &Overcast, x: f64, z: f64) -> f64 {
        let (lo, hi) = (o.bottom(), o.top());
        let n = 400;
        (0..n).map(|i| o.density(&Vector3::new(x, lo + (hi - lo) * (i as f64 + 0.5) / n as f64, z))).sum::<f64>() * (hi - lo) / n as f64
    }

    #[test]
    fn test_covers_the_sky_thickly_but_unevenly() {
        let o = deck();
        let depths: Vec<f64> = (0..400).map(|i| column(&o, (i % 20) as f64 * 731., (i / 20) as f64 * 677.)).collect();
        let min = depths.iter().cloned().fold(f64::MAX, f64::min);
        let max = depths.iter().cloned().fold(0., f64::max);
        // Overcast: no clear sky anywhere, but some of it three times as thick as the rest.
        assert!(min > 5., "a gap: optical depth {}", min);
        assert!(max > 3. * min, "too even: {} to {}", min, max);
    }

    #[test]
    fn test_breaks_open_gaps() {
        let mut o = deck();
        o.breaks = 0.5;
        let clear = (0..400).filter(|i| column(&o, (i % 20) as f64 * 731., (i / 20) as f64 * 677.) < 0.5).count();
        assert!(clear > 20 && clear < 300, "{}", clear);
    }

    #[test]
    fn test_fractus_drifts_under_the_breaks() {
        let mut o = deck();
        (o.breaks, o.scud) = (0.6, 0.8);
        // Gaps in the deck with shreds under some of them, and every density bounded.
        let mut under_gaps = 0;
        let mut rng = Pcg32::new(3, 2);
        for i in 0..400 {
            let (x, z) = ((i % 20) as f64 * 731., (i / 20) as f64 * 677.);
            if o.column_at(x, z).is_some() { continue; }
            for _ in 0..20 {
                let y = o.base - rng.next_f64() * o.scud_depth;
                let d = o.density(&Vector3::new(x, y, z));
                assert!(d <= o.peak() + 1e-12);
                if d > 0. { under_gaps += 1; }
            }
        }
        assert!(under_gaps > 0);
        let (m, _) = o.majorant_from(&Vector3::new(0., o.base - 100., 0.), &Vector3::new(1., 0., 0.));
        assert!(m >= o.scud_density);
    }

    #[test]
    fn test_lumps_lower_the_base_and_scud_hangs_below() {
        let mut o = deck();
        o.scud = 0.;
        // Where the lumps are strongest the base is lower than between them.
        let base_at = |x: f64, z: f64| (0..2000).map(|k| o.bottom() + k as f64).find(|y| o.density(&Vector3::new(x, *y, z)) > o.max_density * 0.15).unwrap_or(o.top());
        let mut rng = Pcg32::new(2, 0);
        let (mut strong, mut weak) = (vec![], vec![]);
        for _ in 0..3000 {
            let (x, z) = (rng.next_f64() * 20000., rng.next_f64() * 20000.);
            let l = o.lump(x, z);
            if l > 0.8 { strong.push(base_at(x, z)); } else if l < 0.2 { weak.push(base_at(x, z)); }
            if strong.len() > 30 && weak.len() > 30 { break; }
        }
        let mean = |v: &Vec<f64>| v.iter().sum::<f64>() / v.len() as f64;
        assert!(mean(&strong) + 100. < mean(&weak), "{} vs {}", mean(&strong), mean(&weak));
        // Scud: under roughly the fraction of the sky asked for, and none without it.
        let covered = |o: &Overcast| (0..2000).filter(|k| {
            let (x, z) = ((k % 45) as f64 * 173., (k / 45) as f64 * 191.);
            let base = o.column_at(x, z).unwrap().local_base;
            (0..40).any(|j| o.scud_fraction(&Vector3::new(x, base - o.scud_depth * j as f64 / 40., z), base) > 0.05)
        }).count() as f64 / 2000.;
        let mut o = deck();
        o.scud = 0.5;
        assert!((covered(&o) - 0.5).abs() < 0.2, "{}", covered(&o));
        o.scud = 0.;
        assert_eq!(covered(&o), 0.);
    }

    #[test]
    fn test_tracking_matches_optical_depth() {
        use crate::geometry::seed_rand;
        use crate::ray::Ray;
        use crate::shapes::bbox::BBox;
        use crate::shapes::geometry::Geometry;
        use crate::volume::Volume;
        seed_rand(5, 0);
        let mut o = deck();
        o.max_density = 0.0005;
        o.scud_density = 0.0005;
        let (from, to) = (Vector3::new(-3000., 300., -2000.), Vector3::new(4000., 3500., 1500.));
        let steps = 40_000;
        let optical_depth = (0..steps).map(|i| o.density(&(from + (to - from) * ((i as f64 + 0.5) / steps as f64)))).sum::<f64>()
            * (to - from).norm() / steps as f64;
        assert!(optical_depth > 0.2 && optical_depth < 3., "uninformative test: {}", optical_depth);
        let v = Volume::new(BBox::new(Vector3::new(-5000., 0., -5000.), Vector3::new(5000., 5000., 5000.)), std::sync::Arc::new(o));
        let r = Ray { ro: from, rd: to - from };
        let n = 20_000;
        let ratio = (0..n).map(|_| v.transmittance(&r, 0., (to - from).norm())).sum::<f64>() / n as f64;
        assert!((ratio - (-optical_depth).exp()).abs() < 0.01, "{} vs {}", ratio, (-optical_depth).exp());
    }
}
