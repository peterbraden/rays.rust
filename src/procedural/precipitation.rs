//! Rain and virga: precipitation falling from a cloud's base. A density field for
//! `volume::Volume`.
//!
//! Raindrops are a millimetre or two across, so a shower is optically thin where a cloud is
//! thick: what matters is the drops' cross-section, set by how hard it rains. With the
//! Marshall-Palmer distribution of drop sizes, N(D) = N₀ e^(-ΛD) with N₀ = 8000 m⁻³ mm⁻¹ and
//! Λ = 4.1 R^-0.21 mm⁻¹ for a rain rate R in mm/h (J. S. Marshall and W. M. Palmer, "The
//! distribution of raindrops with size", J. Meteor. 5, 1948), and an extinction efficiency of 2
//! for drops far larger than the wavelength, the extinction is π N₀ / Λ³: 0.001 per metre at
//! 5 mm/h, a visibility of a few kilometres (`rain_extinction`).
//!
//! Precipitation falls in shafts from the cells of a cloud, and leans as the wind changes with
//! height, so a shaft drifts downwind as it falls (`shear`). Fine streaks run down it, where
//! drops fall together from denser parts of the cloud. Falling through drier air, precipitation
//! evaporates (`fall_depth`, the distance over which it falls to 1/e): rain falls a long way and
//! reaches the ground; virga, often snow or small drops falling from mid-level cloud,
//! evaporates on the way, its streaks fraying out as they go.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::{lerp, smoothstep};
use crate::volume::DensityField;

/// Extinction (per metre) of rain falling at `rate` mm/h, for the Marshall-Palmer distribution
/// of drop sizes.
pub fn rain_extinction(rate: f64) -> f64 {
    if rate <= 0. {
        return 0.;
    }
    let n0 = 8000. * 1e-6; // m⁻³ mm⁻¹, with D² in mm² to m²
    let lambda = 4.1 * rate.powf(-0.21);
    std::f64::consts::PI * n0 / lambda.powi(3)
}

pub struct Precipitation {
    /// The cloud's base, which it falls from, and the ground (m).
    pub base: f64,
    pub ground: f64,
    /// The rain rate at the heart of a shaft (mm/h).
    pub rain_rate: f64,
    /// The size of the cells it falls from, and the fraction of the area under rain.
    pub cell_size: f64,
    pub coverage: f64,
    /// The distance over which it falls to 1/e by evaporating (m).
    pub fall_depth: f64,
    /// Horizontal drift per metre fallen, downwind ([x, z], unit).
    pub shear: f64,
    pub wind_direction: Vector2<f64>,
    /// The width of the fine streaks, and how strongly they mark the shafts (0 to 1).
    pub streak_size: f64,
    pub streakiness: f64,
    /// Where it can rain: within `radius` of `center` ([x, z]), fading over its outer `fade`
    /// (a fraction of the radius).
    pub center: Vector2<f64>,
    pub radius: f64,
    pub fade: f64,
    cells: PerlinNoise,
    streaks: PerlinNoise,
}

impl Precipitation {
    pub fn new(base: f64, seed: u64) -> Precipitation {
        Precipitation {
            base,
            ground: 0.,
            rain_rate: 8.,
            cell_size: 4000.,
            coverage: 0.5,
            fall_depth: f64::INFINITY,
            shear: 0.25,
            wind_direction: Vector2::new(1., 0.),
            streak_size: 120.,
            streakiness: 0.6,
            center: Vector2::zeros(),
            radius: 30_000.,
            fade: 0.3,
            cells: PerlinNoise::with_seed(seed.wrapping_mul(7).wrapping_add(3)),
            streaks: PerlinNoise::with_seed(seed.wrapping_mul(7).wrapping_add(4)),
        }
    }

    /// The most extinction anywhere: the full rain rate, with the streaks' strengthening.
    pub fn peak_extinction(&self) -> f64 {
        rain_extinction(self.rain_rate) * (1. + 0.5 * self.streakiness)
    }

    /// How far a shaft drifts by the ground.
    pub fn drift(&self) -> f64 {
        self.shear * (self.base - self.ground)
    }

    /// The rain rate at `p`, relative to `rain_rate` (0 to about 1.5 in the streaks).
    pub fn intensity(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.ground || p.y > self.base {
            return 0.;
        }
        let fallen = self.base - p.y;
        // Where this rain fell from, at the base.
        let drift = self.wind_direction * self.shear * fallen;
        let q = Vector2::new(p.x, p.z) - drift;
        let from_centre = (q - self.center).norm() / self.radius;
        let region = smoothstep(0., self.fade.max(1e-9), 1. - from_centre);
        if region <= 0. {
            return 0.;
        }
        let c = self.cells.fbm(q.x / self.cell_size, 0.5, q.y / self.cell_size, 4, 0.5, 2.);
        let shafts = smoothstep(1. - self.coverage, 1. - self.coverage + 0.2, c) * region;
        if shafts <= 0. {
            return 0.;
        }
        // Streaks, drawn out down the fall.
        let s = self.streaks.fbm(q.x / self.streak_size, p.y / (self.streak_size * 15.), q.y / self.streak_size, 3, 0.5, 2.);
        let streaks = lerp(1., 0.2 + 1.3 * s, self.streakiness);
        // Evaporation, fraying the streaks: the weaker ones end first.
        let left = (-fallen / self.fall_depth).exp();
        let frayed = if self.fall_depth.is_finite() {
            smoothstep(0., 0.3, left - (1. - s) * 0.4)
        } else {
            1.
        };
        // Drops leave the cloud over its last hundred metres or so.
        let emerging = smoothstep(0., 150., fallen);
        (shafts * streaks * left * frayed * emerging).clamp(0., 1. + 0.5 * self.streakiness)
    }
}

impl DensityField for Precipitation {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let i = self.intensity(p);
        if i <= 0. {
            return 0.;
        }
        // Extinction grows as rate^0.63 (Λ⁻³); scale the full rate's.
        rain_extinction(self.rain_rate) * i.powf(0.63)
    }

    fn majorant(&self) -> f64 {
        self.peak_extinction()
    }

    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let ahead = p + d * 0.01;
        let (lo, hi) = (self.ground, self.base);
        let to_y = |y: f64| if d.y != 0. { (y - ahead.y) / d.y } else { f64::INFINITY };
        if ahead.y < lo {
            return (0., if d.y > 0. { to_y(lo) + 0.01 } else { f64::INFINITY });
        }
        if ahead.y > hi {
            return (0., if d.y < 0. { to_y(hi) + 0.01 } else { f64::INFINITY });
        }
        let exit = if d.y > 0. { to_y(hi) } else if d.y < 0. { to_y(lo) } else { f64::INFINITY };
        (self.peak_extinction(), exit + 0.01)
    }

    /// Raindrops: about 3.67 / Λ mm across, the median of the distribution by volume.
    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(3.67 / (4.1 * self.rain_rate.max(0.1).powf(-0.21)) / 2. * 1000.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    #[test]
    fn test_rain_extinction_gives_visibility_of_kilometres() {
        // Moderate rain: about 1e-3 per metre, a visibility (3/β) of 2-4 km.
        let e = rain_extinction(5.);
        assert!(e > 8e-4 && e < 1.3e-3, "{}", e);
        assert!(rain_extinction(25.) > 2. * e);
        assert_eq!(rain_extinction(0.), 0.);
    }

    #[test]
    fn test_shafts_lean_downwind_and_virga_evaporates() {
        let mut r = Precipitation::new(2000., 1);
        (r.streakiness, r.coverage) = (0., 0.5);
        // A point under rain at the base is under rain lower down shifted downwind by the drift.
        let mut rng = Pcg32::new(3, 0);
        let mut checked = 0;
        for _ in 0..2000 {
            let (x, z) = (rng.next_f64() * 20000. - 10000., rng.next_f64() * 20000. - 10000.);
            let top = r.intensity(&Vector3::new(x, 1800., z));
            if top > 0.5 {
                let shifted = r.intensity(&Vector3::new(x + r.shear * 1000., 800., z));
                assert!((shifted - top).abs() < 1e-9, "{} vs {}", shifted, top);
                checked += 1;
            }
        }
        assert!(checked > 100);
        // Virga: half way down it has thinned, and near the ground it has gone.
        r.fall_depth = 400.;
        let mean = |y: f64| (0..4000).map(|k| r.intensity(&Vector3::new((k % 63) as f64 * 300. + r.shear * (2000. - y), y, (k / 63) as f64 * 300.))).sum::<f64>();
        assert!(mean(1000.) < 0.2 * mean(1800.));
        assert_eq!(mean(100.), 0.);
    }

    #[test]
    fn test_majorant_bounds_density() {
        let mut r = Precipitation::new(2500., 2);
        r.wind_direction = Vector2::new(0.6, 0.8);
        let mut rng = Pcg32::new(4, 0);
        for _ in 0..20_000 {
            let p = Vector3::new(rng.next_f64() * 40000. - 20000., rng.next_f64() * 3000., rng.next_f64() * 40000. - 20000.);
            assert!(r.density(&p) <= r.majorant_from(&p, &Vector3::new(0., 1., 0.)).0 + 1e-15);
        }
    }
}
