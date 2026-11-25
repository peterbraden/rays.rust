//! # Isostasy: the crust floats
//!
//! The lithosphere floats on the mantle, and what is taken off it or put on it moves it.
//! Erosion unloads it, and it rises: in Airy isostasy by ρ_rock / ρ_mantle ≈ 0.82 m for each
//! metre of rock removed, so cutting valleys raises the peaks between them (Molnar and England,
//! "Late Cenozoic uplift of mountain ranges and global climate change: chicken or egg?",
//! Nature 346, 1990). Ice weighs it down, by ρ_ice / ρ_mantle ≈ 0.28 of the ice's thickness,
//! and it rebounds when the ice melts (Scandinavia is still rising), and the glaciers' troughs
//! deepen into fjords below the sea; sediment loads basins.
//!
//! The lithosphere is an elastic plate, so it doesn't respond cell by cell but over its
//! flexural wavelength: a load is supported by the plate around it over a distance of about
//! α = (4D / (Δρ g))^(1/4), with flexural rigidity D = E T_e³ / (12 (1 - ν²)) for an elastic
//! thickness T_e (Turcotte and Schubert, "Geodynamics", 3rd ed., 2014, ch. 3). For T_e = 20 km
//! (E = 70 GPa, ν = 0.25, Δρ the mantle's 3300 kg/m³) α is about 50 km. Here the deflection
//! is the Airy deflection of the load smoothed by a Gaussian of width α, standing in for the
//! plate's response function (`smooth`: box blurs on a patch's grid, diffusion elsewhere),
//! and the ground relaxes towards it with the mantle's time constant, a few thousand years
//! (Cathles, "The Viscosity of the Earth's Mantle", 1975, finds about 4400 years for
//! Fennoscandia).
//!
//! The load is what erosion and deposition have changed of the column since time zero (its
//! ground's change less the tectonic uplift and the rebound, which move rock without adding or
//! removing it) and the ice on it. A patch can't see loads beyond its edge, and its edge and
//! the sea are held as base level, as for uplift.

use crate::terrain::domain::{Domain, Layout};
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IsostasyParams {
    /// Elastic thickness of the lithosphere, metres.
    pub elastic_thickness: f64,
    /// Densities of the mantle, the crust's rock and ice, kg/m³.
    pub mantle_density: f64,
    pub rock_density: f64,
    pub ice_density: f64,
    /// The mantle's relaxation time, years.
    pub relaxation: f64,
}

impl Default for IsostasyParams {
    fn default() -> IsostasyParams {
        IsostasyParams { elastic_thickness: 20e3, mantle_density: 3300., rock_density: 2700., ice_density: 917., relaxation: 4000. }
    }
}

impl IsostasyParams {
    /// The flexural parameter α, metres (see the module docs).
    pub fn flexural_length(&self) -> f64 {
        let (e, nu, g) = (70e9, 0.25, 9.81);
        let d = e * self.elastic_thickness.powi(3) / (12. * (1. - nu * nu));
        (4. * d / (self.mantle_density * g)).powf(0.25)
    }
}

/// The equilibrium deflection of the ground, metres (positive up), for columns whose rock and
/// sediment have changed in thickness by `column` metres since time zero and that carry `ice`
/// metres of ice.
pub fn equilibrium(domain: &Domain, column: &[f64], ice: &[f64], p: &IsostasyParams) -> Vec<f64> {
    let load: Vec<f64> = column.iter().zip(ice).map(|(c, i)| -(p.rock_density * c + p.ice_density * i) / p.mantle_density).collect();
    smooth(domain, &load, p.flexural_length())
}

/// `field` smoothed by a Gaussian of standard deviation `sigma` metres, mass-conserving, with
/// nothing leaving at the edges. On a patch, by three box blurs along each axis (which
/// approximate the Gaussian closely); elsewhere by diffusion over the cells' links.
pub fn smooth(domain: &Domain, field: &[f64], sigma: f64) -> Vec<f64> {
    match &domain.layout {
        Layout::Patch { n, spacing, .. } => {
            let n = *n;
            let s = sigma / spacing;
            // Three passes of a box of width w have variance 3 (w² - 1) / 12.
            let half = (((4. * s * s + 1.).sqrt() - 1.) / 2.).round().max(0.) as usize;
            let mut f = field.to_vec();
            for _ in 0..3 {
                f = blur_rows(&f, n, half);
                f = transpose(&blur_rows(&transpose(&f, n), n, half), n);
            }
            f
        }
        Layout::Sphere { .. } => {
            // Diffusion for a time t spreads a point by σ² = 2κt; steps of κ dt = spacing² / 8
            // are stable on the cell graph, and each adds spacing² / 4 to σ².
            let spacing = domain.min_spacing();
            let steps = (4. * sigma * sigma / (spacing * spacing)).ceil().min(10_000.) as usize;
            let mut f = field.to_vec();
            for _ in 0..steps {
                f = (0..domain.len()).into_par_iter().map(|i| {
                    let links = domain.edges(i);
                    let flux: f64 = links.iter().map(|l| l.width / l.dist * (f[l.to] - f[i])).sum();
                    f[i] + spacing * spacing / 8. * flux / domain.area[i]
                }).collect();
            }
            f
        }
    }
}

/// Each row of an `n` x `n` grid averaged over a box of `half` cells either side, reflected at
/// the edges.
fn blur_rows(f: &[f64], n: usize, half: usize) -> Vec<f64> {
    if half == 0 { return f.to_vec(); }
    let reflect = |i: i64| -> usize {
        let period = 2 * n as i64;
        let m = i.rem_euclid(period);
        (if m < n as i64 { m } else { period - 1 - m }) as usize
    };
    let w = (2 * half + 1) as f64;
    let mut out = vec![0.; n * n];
    out.par_chunks_mut(n).enumerate().for_each(|(j, row)| {
        let src = &f[j * n..(j + 1) * n];
        let mut sum: f64 = (-(half as i64)..=half as i64).map(|k| src[reflect(k)]).sum();
        for i in 0..n {
            row[i] = sum / w;
            sum += src[reflect(i as i64 + half as i64 + 1)] - src[reflect(i as i64 - half as i64)];
        }
    });
    out
}

fn transpose(f: &[f64], n: usize) -> Vec<f64> {
    (0..n * n).map(|k| f[(k % n) * n + k / n]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flexural_length_is_tens_of_kilometres() {
        let a = IsostasyParams::default().flexural_length();
        assert!((40e3..60e3).contains(&a), "{}", a);
    }

    #[test]
    fn test_unloading_raises_the_ground_by_airy() {
        // Rock taken off evenly over a patch raises it by ρ_rock / ρ_mantle of it; ice weighs it
        // down by ρ_ice / ρ_mantle.
        let d = Domain::patch(0., 0., 32, 500., 6.371e6);
        let p = IsostasyParams::default();
        let n = d.len();
        let w = equilibrium(&d, &vec![-100.; n], &vec![0.; n], &p);
        assert!(w.iter().all(|&x| (x - 100. * 2700. / 3300.).abs() < 1e-6), "{}", w[0]);
        let w = equilibrium(&d, &vec![0.; n], &vec![1000.; n], &p);
        assert!(w.iter().all(|&x| (x + 1000. * 917. / 3300.).abs() < 1e-6));
    }

    #[test]
    fn test_flexure_spreads_a_load_and_keeps_it() {
        // A valley cut in the middle of a patch raises it and the ground around it, less at
        // the valley than Airy alone would, more beside it; the total is Airy's.
        let d = Domain::patch(0., 0., 64, 5000., 6.371e6);
        let p = IsostasyParams::default();
        let n = d.len();
        let column: Vec<f64> = (0..n).map(|k| if (k % 64).abs_diff(32) < 2 { -1000. } else { 0. }).collect();
        let w = equilibrium(&d, &column, &vec![0.; n], &p);
        let airy = 1000. * 2700. / 3300.;
        // 40 km and 150 km away: within the flexural length and well beyond it.
        let (valley, beside, far) = (w[20 * 64 + 32], w[20 * 64 + 40], w[20 * 64 + 2]);
        assert!(valley < 0.5 * airy && beside > 0.3 * valley && far < 0.1 * valley, "{} {} {}", valley, beside, far);
        let total: f64 = w.iter().sum();
        let expected: f64 = column.iter().map(|c| -c * 2700. / 3300.).sum();
        assert!((total - expected).abs() < 1e-6 * expected, "{} vs {}", total, expected);
    }
}
