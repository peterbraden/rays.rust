//! # From RGB to spectra
//!
//! Scenes give colours as RGB, and spectral rendering needs a spectrum for each. Infinitely
//! many spectra share an RGB (metamers); the useful one is smooth, like the reflectance spectra
//! of real materials, and for a reflectance, bounded by [0, 1]. Wenzel Jakob and Johannes
//! Hanika, "A Low-Dimensional Function Space for Efficient Spectral Upsampling", Computer
//! Graphics Forum 38(2) (Eurographics 2019), pp. 147-155, doi:10.1111/cgf.13626
//! (<https://rgl.epfl.ch/publications/Jakob2019Spectral>), find such spectra in the family
//!
//! ```text
//! s(λ) = S(c₀ λ² + c₁ λ + c₂),     S(x) = 1/2 + x / (2 √(1 + x²))
//! ```
//!
//! a quadratic squashed by a sigmoid into (0, 1): smooth, bounded, and able to reach saturated
//! colours by steep sigmoid edges. Three coefficients for three colour channels: for each RGB,
//! solve for the c whose spectrum, seen under the colour space's illuminant (D65 for sRGB), has
//! that colour, minimising the difference in CIELAB, which weights errors by how visible they
//! are. Solving is too slow to do per lookup, so the coefficients are tabulated over the RGB
//! cube ahead of time, and interpolated (PBR section 4.6.6, `RGBToSpectrumTable`, and pbrt-v4's
//! `rgb2spec_opt.cpp`, which this follows).
//!
//! ## The table
//!
//! An RGB is indexed by its largest component z, and the other two as fractions x, y of it, so
//! one of three tables (by which component is largest) covers it with x, y, z in [0, 1]. The z
//! nodes crowd towards 0 and 1 (smoothstep applied twice), where the coefficients change
//! fastest. Each cell is solved by Gauss-Newton iteration, starting from the coefficients of
//! the cell before it along z, beginning from a fifth of the way up with zero (a flat grey
//! spectrum) and moving out in both directions: the solution moves smoothly, and the iteration
//! needs a nearby start to converge for saturated colours.
//!
//! pbrt-v4 builds its tables at compile time, at 64 nodes a side; here they are built when
//! first needed (only in spectral mode), in parallel, at 32 a side, with λ in the coefficients
//! normalised to t = (λ - 360) / 470 in [0, 1]. Interpolating coefficients is not
//! interpolating colours, so a looked-up spectrum's colour differs slightly from the RGB asked
//! for: within 0.01 for the colours of the tests.
use crate::na::{Matrix3, Vector3};
use crate::spectrum::{cie, LAMBDA_MIN, LAMBDA_MAX, STEP};
use rayon::prelude::*;
use std::sync::OnceLock;

/// A row of solved cells: (largest component, y node, [(z node, x node, coefficients)]).
type Row = (usize, usize, Vec<(usize, usize, [f64; 3])>);

/// Nodes along each side of the table.
pub const RES: usize = 32;

/// The sigmoid S(x) = 1/2 + x / (2 √(1 + x²)), with its limits at ±∞.
fn sigmoid(x: f64) -> f64 {
    if x.is_infinite() {
        return if x > 0. { 1. } else { 0. };
    }
    0.5 + x / (2. * (1. + x * x).sqrt())
}

/// A spectrum S(c₀ t² + c₁ t + c₂), t = (λ - 360) / 470.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sigmoid {
    pub c: [f64; 3],
}

impl Sigmoid {
    /// The spectrum that is `v` (in [0, 1]) everywhere: c₂ = S⁻¹(v).
    pub fn constant(v: f64) -> Sigmoid {
        let c2 = if v <= 0. {
            f64::NEG_INFINITY
        } else if v >= 1. {
            f64::INFINITY
        } else {
            (v - 0.5) / (v * (1. - v)).sqrt()
        };
        Sigmoid { c: [0., 0., c2] }
    }

    pub fn eval(&self, lambda: f64) -> f64 {
        let t = (lambda - LAMBDA_MIN) / (LAMBDA_MAX - LAMBDA_MIN);
        if self.c[0] == 0. && self.c[1] == 0. {
            return sigmoid(self.c[2]);
        }
        sigmoid((self.c[0] * t + self.c[1]) * t + self.c[2])
    }
}

/// Integration weights for the solver: at each table wavelength, the trapezoid weight times
/// D65 times the matching functions, normalised so that a spectrum of 1 has Y = 1; and the
/// white point, the XYZ of that spectrum.
struct Solver {
    t: Vec<f64>,
    xyz: Vec<Vector3<f64>>,
    white: Vector3<f64>,
    srgb_to_xyz: Matrix3<f64>,
}

impl Solver {
    fn new() -> Solver {
        let n = cie::SAMPLES;
        let mut t = Vec::with_capacity(n);
        let mut xyz = Vec::with_capacity(n);
        let mut norm = 0.;
        for i in 0..n {
            let w = if i == 0 || i == n - 1 { 0.5 * STEP } else { STEP };
            let lambda = LAMBDA_MIN + STEP * i as f64;
            t.push((lambda - LAMBDA_MIN) / (LAMBDA_MAX - LAMBDA_MIN));
            xyz.push(Vector3::new(cie::CIE_X[i], cie::CIE_Y[i], cie::CIE_Z[i]) * (cie::D65[i] * w));
            norm += cie::CIE_Y[i] * cie::D65[i] * w;
        }
        for v in xyz.iter_mut() {
            *v /= norm;
        }
        let white = xyz.iter().sum();
        let m = cie::SRGB_TO_XYZ;
        let srgb_to_xyz = Matrix3::new(m[0][0], m[0][1], m[0][2], m[1][0], m[1][1], m[1][2], m[2][0], m[2][1], m[2][2]);
        Solver { t, xyz, white, srgb_to_xyz }
    }

    /// CIELAB of XYZ relative to the white point (CIE 015:2018, section 8.2.1).
    fn lab(&self, xyz: &Vector3<f64>) -> Vector3<f64> {
        let f = |t: f64| {
            const DELTA: f64 = 6. / 29.;
            if t > DELTA.powi(3) { t.cbrt() } else { t / (3. * DELTA * DELTA) + 4. / 29. }
        };
        let (fx, fy, fz) = (f(xyz.x / self.white.x), f(xyz.y / self.white.y), f(xyz.z / self.white.z));
        Vector3::new(116. * fy - 16., 500. * (fx - fy), 200. * (fy - fz))
    }

    /// The difference in CIELAB between the target and the spectrum of `c`, under D65.
    fn residual(&self, c: &[f64; 3], target: &Vector3<f64>) -> Vector3<f64> {
        let xyz: Vector3<f64> = self.t.iter().zip(&self.xyz)
            .map(|(t, w)| w * sigmoid((c[0] * t + c[1]) * t + c[2]))
            .sum();
        target - self.lab(&xyz)
    }

    /// Refine `c` towards the spectrum of `rgb` by Gauss-Newton, with a Jacobian by central
    /// differences, as `rgb2spec_opt.cpp` does.
    fn gauss_newton(&self, rgb: &Vector3<f64>, c: &mut [f64; 3]) {
        let target = self.lab(&(self.srgb_to_xyz * rgb));
        for _ in 0..15 {
            let r = self.residual(c, &target);
            if r.norm() < 1e-6 {
                break;
            }
            let mut j = Matrix3::zeros();
            const EPS: f64 = 1e-5;
            for k in 0..3 {
                let (mut lo, mut hi) = (*c, *c);
                lo[k] -= EPS;
                hi[k] += EPS;
                let d = (self.residual(&hi, &target) - self.residual(&lo, &target)) / (2. * EPS);
                j.set_column(k, &d);
            }
            // r = target - lab(c), so dr/dc = -J_lab, and the step is c += J_lab⁻¹ r = c - J⁻¹ r.
            let step = match j.lu().solve(&r) {
                Some(s) => s,
                None => break,
            };
            for k in 0..3 {
                c[k] -= step[k];
            }
            // Keep the sigmoid from growing too steep to evaluate well.
            let max = c.iter().fold(0f64, |m, v| m.max(v.abs()));
            if max > 200. {
                for v in c.iter_mut() {
                    *v *= 200. / max;
                }
            }
        }
    }
}

fn smoothstep(x: f64) -> f64 {
    x * x * (3. - 2. * x)
}

/// Sigmoid coefficients over the RGB cube.
pub struct Table {
    z_nodes: Vec<f64>,
    /// Indexed [largest component][z][y][x].
    coeffs: Vec<[f64; 3]>,
}

/// The table, built on first use.
pub fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(Table::build)
}

impl Table {
    fn index(l: usize, k: usize, j: usize, i: usize) -> usize {
        ((l * RES + k) * RES + j) * RES + i
    }

    fn build() -> Table {
        let solver = Solver::new();
        let z_nodes: Vec<f64> = (0..RES).map(|k| smoothstep(smoothstep(k as f64 / (RES - 1) as f64))).collect();
        // Each (largest component, y) row of cells is solved along z for every x.
        let rows: Vec<Row> = (0..3 * RES).into_par_iter().map(|lj| {
            let (l, j) = (lj / RES, lj % RES);
            let y = j as f64 / (RES - 1) as f64;
            let mut cells = Vec::with_capacity(RES * RES);
            for i in 0..RES {
                let x = i as f64 / (RES - 1) as f64;
                let solve = |k: usize, c: &mut [f64; 3]| {
                    let b = z_nodes[k];
                    let mut rgb = Vector3::zeros();
                    rgb[l] = b;
                    rgb[(l + 1) % 3] = x * b;
                    rgb[(l + 2) % 3] = y * b;
                    solver.gauss_newton(&rgb, c);
                    *c
                };
                let start = RES / 5;
                let mut c = [0.; 3];
                for k in start..RES {
                    cells.push((k, i, solve(k, &mut c)));
                }
                let mut c = [0.; 3];
                for k in (0..start).rev() {
                    cells.push((k, i, solve(k, &mut c)));
                }
            }
            (l, j, cells)
        }).collect();
        let mut coeffs = vec![[0.; 3]; 3 * RES * RES * RES];
        for (l, j, cells) in rows {
            for (k, i, c) in cells {
                coeffs[Table::index(l, k, j, i)] = c;
            }
        }
        Table { z_nodes, coeffs }
    }

    /// The spectrum for `rgb`, each component in [0, 1]: interpolated trilinearly from the
    /// table (PBR's `RGBToSpectrumTable::operator()`).
    pub fn lookup(&self, rgb: [f64; 3]) -> Sigmoid {
        if rgb[0] == rgb[1] && rgb[1] == rgb[2] {
            return Sigmoid::constant(rgb[0]);
        }
        let l = if rgb[0] >= rgb[1] && rgb[0] >= rgb[2] { 0 } else if rgb[1] >= rgb[2] { 1 } else { 2 };
        let z = rgb[l];
        let scale = (RES - 1) as f64 / z;
        let x = rgb[(l + 1) % 3] * scale;
        let y = rgb[(l + 2) % 3] * scale;
        let xi = (x as usize).min(RES - 2);
        let yi = (y as usize).min(RES - 2);
        let zi = self.z_nodes.partition_point(|n| *n <= z).saturating_sub(1).min(RES - 2);
        let (dx, dy) = (x - xi as f64, y - yi as f64);
        let dz = (z - self.z_nodes[zi]) / (self.z_nodes[zi + 1] - self.z_nodes[zi]);
        let mut c = [0.; 3];
        for (a, wz) in [(0, 1. - dz), (1, dz)] {
            for (b, wy) in [(0, 1. - dy), (1, dy)] {
                for (d, wx) in [(0, 1. - dx), (1, dx)] {
                    let cell = self.coeffs[Table::index(l, zi + a, yi + b, xi + d)];
                    let w = wz * wy * wx;
                    for n in 0..3 {
                        c[n] += w * cell[n];
                    }
                }
            }
        }
        Sigmoid { c }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::{d65, spectrum_to_xyz, xyz_to_srgb};
    use crate::geometry::Pcg32;

    #[test]
    fn test_constant_spectra() {
        for v in [0., 0.2, 0.5, 0.9, 1.] {
            let s = Sigmoid::constant(v);
            assert!((s.eval(400.) - v).abs() < 1e-12 && (s.eval(700.) - v).abs() < 1e-12, "{}", v);
        }
    }

    /// Spectra from the table are reflectances, and under D65 have (nearly) the RGB asked for.
    #[test]
    fn test_table_round_trip() {
        let t = table();
        let mut rng = Pcg32::new(11, 4);
        let mut worst: f64 = 0.;
        for _ in 0..200 {
            let rgb = [rng.next_f64(), rng.next_f64(), rng.next_f64()];
            let s = t.lookup(rgb);
            for l in (360..=830).step_by(10) {
                let v = s.eval(l as f64);
                assert!((0. ..=1.).contains(&v), "{:?} at {} nm: {}", rgb, l, v);
            }
            let seen = xyz_to_srgb(&spectrum_to_xyz(|l| s.eval(l) * d65(l)));
            let err = (seen.rgb - Vector3::from(rgb)).abs().max();
            worst = worst.max(err);
        }
        assert!(worst < 0.02, "worst round-trip error {}", worst);
    }

    #[test]
    fn test_saturated_colours_are_reached() {
        let s = table().lookup([1., 0., 0.]);
        // Red: reflects long wavelengths, absorbs short ones.
        assert!(s.eval(700.) > 0.9 && s.eval(450.) < 0.1, "{:?}", s);
        let s = table().lookup([0., 0., 1.]);
        assert!(s.eval(450.) > 0.8 && s.eval(650.) < 0.1, "{:?}", s);
    }
}
