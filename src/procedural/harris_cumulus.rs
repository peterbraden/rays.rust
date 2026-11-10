//! Cumulus grown by simulating the air they form in, after M. J. Harris, W. V. Baxter III,
//! T. Scheuermann and A. Lastra, "Simulation of Cloud Dynamics on Graphics Hardware", Graphics
//! Hardware 2003: incompressible flow (J. Stam, "Stable Fluids", SIGGRAPH 1999) carrying heat,
//! water vapour and cloud water, with the thermodynamics of a cloud: buoyancy, condensation and
//! latent heat. (The same core, with fuller microphysics, drives T. Hädrich et al.,
//! "Stormscapes", SIGGRAPH 2020.)
//!
//! On a sunny day the ground heats the air above it unevenly. Warm bubbles (thermals) break
//! away and rise through the mixed boundary layer, cooling as they expand. Those that reach
//! the lifting condensation level saturate and condense: the cloud base, flat because the whole
//! layer shares one condensation level. Condensing releases latent heat, so the cloudy air
//! cools more slowly than its surroundings as it rises, stays buoyant and accelerates: the
//! cloud boils upwards in turrets until it reaches a stable layer (the inversion) or runs out
//! of heat. Its edges mix with the dry air around it and evaporate, which cools them, so they
//! sink; cut off from its thermal, the cloud evaporates away within some ten or twenty minutes.
//!
//! The model:
//!
//! * **Flow.** Boussinesq (incompressible, with density differences only in the buoyancy) on a
//!   staggered (MAC) grid, periodic across the ground and closed at the ground and the top.
//!   Advection is semi-Lagrangian with MacCormack's correction (Selle et al. 2008), so turrets
//!   keep their shape; vorticity confinement (Fedkiw, Stam and Jensen 2001) restores the swirls
//!   that the grid smooths away. The pressure projection is exact: an FFT across the ground
//!   and a tridiagonal solve up each column.
//! * **Thermodynamics.** Potential temperature θ, vapour and cloud water mixing ratios. Each
//!   step, cloud water and vapour are brought to saturation (Tetens' formula), with latent heat
//!   released or taken. Buoyancy is g (θ'/θ₀ + 0.61 q_v' - q_c): warm, moist air rises, the
//!   weight of cloud water drags.
//! * **The day.** The environment starts as a mixed layer of uniform θ and humidity, a
//!   conditionally unstable layer above it, an inversion that caps the cumulus, and stable air
//!   above. The ground adds heat and moisture, in patches that drift slowly over time (fields,
//!   woods, towns), and with a little noise to start convection. The top of the domain absorbs
//!   waves (a sponge layer).

use crate::na::{Vector2, Vector3};
use crate::noise::{worley_hash, PerlinNoise};
use rayon::prelude::*;
use rustfft::num_complex::Complex;

pub const GRAVITY: f64 = 9.81;
/// Specific heat of dry air at constant pressure, J/(kg K); its gas constant; latent heat of
/// vaporisation, J/kg.
const CP: f64 = 1005.;
const RD: f64 = 287.;
const LV: f64 = 2.5e6;
const P0: f64 = 1e5;
const KAPPA: f64 = RD / CP;
/// Kessler's autoconversion: cloud water beyond this (kg/kg) turns to rain at this rate (per
/// second), the rain falling straight out. Without it, cloud water piles up in the layer
/// beneath the inversion.
const RAIN_THRESHOLD: f64 = 1e-3;
const RAIN_RATE: f64 = 1e-3;
/// Density of air near the ground, kg/m³, for turning fluxes into rates.
const AIR_DENSITY: f64 = 1.15;

/// Pressure (Pa) at height `z` (m) in the standard atmosphere.
pub fn pressure_at(z: f64) -> f64 {
    P0 * (1. - 2.25577e-5 * z).max(0.01).powf(5.25588)
}

/// The Exner function (p/p₀)^κ: temperature over potential temperature.
pub fn exner_at(z: f64) -> f64 {
    (pressure_at(z) / P0).powf(KAPPA)
}

/// Saturation mixing ratio (kg/kg) over water at temperature `t` (K) and pressure `p` (Pa),
/// by Tetens' formula.
pub fn saturation_mixing_ratio(t: f64, p: f64) -> f64 {
    380.2 / p * (17.67 * (t - 273.15) / (t - 29.65)).exp()
}

/// The air the clouds grow in, before the sun gets to work.
#[derive(Debug, Clone, Copy)]
pub struct Atmosphere {
    /// Potential temperature (K) of the mixed layer, and its depth (m).
    pub surface_theta: f64,
    pub mixed_layer: f64,
    /// How fast θ rises (K/m) above the mixed layer, up to the inversion.
    pub lapse: f64,
    /// Height of the inversion's base (m), how much θ jumps (K) across it, and over what depth.
    pub inversion: f64,
    pub inversion_strength: f64,
    pub inversion_depth: f64,
    /// How fast θ rises above the inversion (K/m).
    pub lapse_above: f64,
    /// Water vapour mixing ratio (kg/kg) in the mixed layer, and the relative humidity above it.
    pub humidity: f64,
    pub humidity_above: f64,
}

impl Default for Atmosphere {
    /// A summer's morning: 27°C at the ground, a mixed layer 1200 m deep with its condensation
    /// level near its top, conditionally unstable air to an inversion at 3 km.
    fn default() -> Atmosphere {
        Atmosphere {
            surface_theta: 300.,
            mixed_layer: 1200.,
            lapse: 3e-3,
            inversion: 3000.,
            inversion_strength: 3.,
            inversion_depth: 200.,
            lapse_above: 5e-3,
            humidity: 0.012,
            humidity_above: 0.5,
        }
    }
}

impl Atmosphere {
    pub fn theta(&self, z: f64) -> f64 {
        let above = (z - self.mixed_layer).max(0.);
        let to_inversion = above.min((self.inversion - self.mixed_layer).max(0.));
        let jump = ((z - self.inversion) / self.inversion_depth).clamp(0., 1.) * self.inversion_strength;
        let beyond = (z - self.inversion - self.inversion_depth).max(0.);
        self.surface_theta + self.lapse * to_inversion + jump + (self.lapse_above - self.lapse) * beyond
    }

    /// Water vapour: the mixed layer's, blending over 200 m into `humidity_above` relative to
    /// saturation (and never more than the mixed layer's).
    pub fn vapour(&self, z: f64) -> f64 {
        let saturated = saturation_mixing_ratio(self.theta(z) * exner_at(z), pressure_at(z));
        let blend = ((z - self.mixed_layer) / 200.).clamp(0., 1.);
        let above = (self.humidity_above * saturated).min(self.humidity);
        self.humidity + (above - self.humidity) * blend
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    /// Cells across (x), up (y) and along (z); x and z must be powers of two.
    pub size: (usize, usize, usize),
    /// Cell size, m.
    pub cell: f64,
    pub atmosphere: Atmosphere,
    /// Sensible and latent heat from the ground, W/m², on average.
    pub heat_flux: f64,
    pub moisture_flux: f64,
    /// How much the ground's heating varies from place to place (0 to 1), how big the patches
    /// are (m), and how long (s) they take to change.
    pub patchiness: f64,
    pub patch_size: f64,
    pub patch_period: f64,
    /// Steady wind, m/s along x and z, carrying everything (the grid is periodic).
    pub wind: Vector2<f64>,
    /// Vorticity confinement strength, ε in f = ε h (N x ω). Small: it feeds the flow energy,
    /// and much above 0.01 drives it faster than the buoyancy does.
    pub vorticity: f64,
    /// The largest time step (s); smaller when the flow is fast.
    pub max_step: f64,
    pub seed: u64,
}

impl Default for SimulationParams {
    fn default() -> SimulationParams {
        SimulationParams {
            size: (128, 80, 128),
            cell: 50.,
            atmosphere: Atmosphere::default(),
            heat_flux: 250.,
            moisture_flux: 150.,
            patchiness: 0.7,
            patch_size: 1500.,
            patch_period: 1200.,
            wind: Vector2::new(2., 0.),
            vorticity: 0.01,
            max_step: 4.,
            seed: 0,
        }
    }
}

/// Values on a grid of `sx` x `sy` x `sz` points, `h` apart, the first at `offset` cells from
/// the origin; periodic in x and z, clamped in y. Stored in horizontal slabs (x fastest), so
/// each level is contiguous.
#[derive(Clone)]
pub struct Field {
    pub data: Vec<f64>,
    pub sx: usize,
    pub sy: usize,
    pub sz: usize,
    pub h: f64,
    pub offset: Vector3<f64>,
}

impl Field {
    pub fn new(sx: usize, sy: usize, sz: usize, h: f64, offset: Vector3<f64>) -> Field {
        Field { data: vec![0.; sx * sy * sz], sx, sy, sz, h, offset }
    }

    #[inline]
    pub fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        i + self.sx * (k + self.sz * j)
    }

    #[inline]
    fn ijk(&self, n: usize) -> (usize, usize, usize) {
        (n % self.sx, n / (self.sx * self.sz), (n / self.sx) % self.sz)
    }

    #[inline]
    pub fn at(&self, i: usize, j: usize, k: usize) -> f64 {
        self.data[self.idx(i, j, k)]
    }

    /// With wrapping in x and z.
    #[inline]
    fn wrapped(&self, i: i64, j: usize, k: i64) -> f64 {
        self.at(i.rem_euclid(self.sx as i64) as usize, j, k.rem_euclid(self.sz as i64) as usize)
    }

    fn position(&self, n: usize) -> Vector3<f64> {
        let (i, j, k) = self.ijk(n);
        (Vector3::new(i as f64, j as f64, k as f64) + self.offset) * self.h
    }

    /// Trilinear interpolation at `p`, with the smallest and largest of the values used.
    #[inline]
    pub fn sample_bounded(&self, p: &Vector3<f64>) -> (f64, f64, f64) {
        let g = p / self.h - self.offset;
        let (x0, z0) = (g.x.floor(), g.z.floor());
        let (fx, fz) = (g.x - x0, g.z - z0);
        let (x0, z0) = (x0 as i64, z0 as i64);
        let gy = g.y.clamp(0., (self.sy - 1) as f64);
        let y0 = (gy.floor() as usize).min(self.sy - 2);
        let fy = gy - y0 as f64;
        let c = |dx: i64, dy: usize, dz: i64| self.wrapped(x0 + dx, y0 + dy, z0 + dz);
        let v = [c(0, 0, 0), c(1, 0, 0), c(0, 1, 0), c(1, 1, 0), c(0, 0, 1), c(1, 0, 1), c(0, 1, 1), c(1, 1, 1)];
        let lx = |a: f64, b: f64| a + (b - a) * fx;
        let (a, b) = (lx(v[0], v[1]), lx(v[2], v[3]));
        let (c0, d) = (lx(v[4], v[5]), lx(v[6], v[7]));
        let near = a + (b - a) * fy;
        let far = c0 + (d - c0) * fy;
        let lo = v.iter().cloned().fold(f64::MAX, f64::min);
        let hi = v.iter().cloned().fold(f64::MIN, f64::max);
        (near + (far - near) * fz, lo, hi)
    }

    #[inline]
    pub fn sample(&self, p: &Vector3<f64>) -> f64 {
        self.sample_bounded(p).0
    }
}

pub struct CloudSimulation {
    pub params: SimulationParams,
    pub time: f64,
    pub steps: u64,
    /// Velocities on the faces of the cells (x, y, z), perturbations from the steady wind.
    pub u: Field,
    pub v: Field,
    pub w: Field,
    /// Potential temperature (K), vapour and cloud water (kg/kg), at the cells' centres.
    pub theta: Field,
    pub vapour: Field,
    pub cloud: Field,
    theta_env: Vec<f64>,
    vapour_env: Vec<f64>,
    exner: Vec<f64>,
    pressure: Vec<f64>,
    patches: PerlinNoise,
}

impl CloudSimulation {
    pub fn new(params: SimulationParams) -> CloudSimulation {
        let (nx, ny, nz) = params.size;
        assert!(nx.is_power_of_two() && nz.is_power_of_two(), "harris_cumulus: the grid's x and z sizes must be powers of two");
        assert!(ny >= 4, "harris_cumulus: the grid needs at least 4 levels");
        let h = params.cell;
        let half = Vector3::new(0.5, 0.5, 0.5);
        let height = |j: usize| (j as f64 + 0.5) * h;
        let a = params.atmosphere;
        let mut sim = CloudSimulation {
            params,
            time: 0.,
            steps: 0,
            u: Field::new(nx, ny, nz, h, Vector3::new(0., 0.5, 0.5)),
            v: Field::new(nx, ny + 1, nz, h, Vector3::new(0.5, 0., 0.5)),
            w: Field::new(nx, ny, nz, h, Vector3::new(0.5, 0.5, 0.)),
            theta: Field::new(nx, ny, nz, h, half),
            vapour: Field::new(nx, ny, nz, h, half),
            cloud: Field::new(nx, ny, nz, h, half),
            theta_env: (0..ny).map(|j| a.theta(height(j))).collect(),
            vapour_env: (0..ny).map(|j| a.vapour(height(j))).collect(),
            exner: (0..ny).map(|j| exner_at(height(j))).collect(),
            pressure: (0..ny).map(|j| pressure_at(height(j))).collect(),
            patches: PerlinNoise::with_seed(params.seed.wrapping_mul(5).wrapping_add(3)),
        };
        // The environment, with a faint random warmth in the mixed layer to break the symmetry.
        let seed = params.seed as u32;
        for n in 0..sim.theta.data.len() {
            let (i, j, k) = sim.theta.ijk(n);
            let jitter = if height(j) < a.mixed_layer { (worley_hash(i as u32, j as u32, k as u32, seed) - 0.5) * 0.2 } else { 0. };
            sim.theta.data[n] = sim.theta_env[j] + jitter;
            sim.vapour.data[n] = sim.vapour_env[j];
        }
        sim
    }

    /// The wind at `p`: the grid's velocities plus the steady wind.
    #[inline]
    fn velocity(&self, p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(self.u.sample(p) + self.params.wind.x, self.v.sample(p), self.w.sample(p) + self.params.wind.y)
    }

    /// Where the air at `p` was `dt` ago (second-order Runge-Kutta).
    #[inline]
    fn trace(&self, p: &Vector3<f64>, dt: f64) -> Vector3<f64> {
        let mid = p - self.velocity(p) * (0.5 * dt);
        p - self.velocity(&mid) * dt
    }

    /// `f` carried along the flow for `dt`, by MacCormack's method: a semi-Lagrangian step
    /// forward, one back, and half the round trip's error corrected, clamped to the values
    /// the forward step interpolated between (so no new extremes). `walls` keeps the first and
    /// last levels at zero (the vertical velocity at the ground and the top).
    fn advect(&self, f: &Field, dt: f64, walls: bool) -> Field {
        let fixed = |g: &Field, n: usize| walls && { let j = g.ijk(n).1; j == 0 || j == g.sy - 1 };
        let mut forward = f.clone();
        let mut bounds = vec![(0., 0.); f.data.len()];
        forward.data.par_iter_mut().zip(bounds.par_iter_mut()).enumerate().for_each(|(n, (out, b))| {
            if fixed(f, n) {
                *out = 0.;
                *b = (0., 0.);
                return;
            }
            let (value, lo, hi) = f.sample_bounded(&self.trace(&f.position(n), dt));
            *out = value;
            *b = (lo, hi);
        });
        let mut result = forward.clone();
        result.data.par_iter_mut().enumerate().for_each(|(n, out)| {
            if fixed(f, n) {
                return;
            }
            let back = forward.sample(&self.trace(&f.position(n), -dt));
            let (lo, hi) = bounds[n];
            *out = (forward.data[n] + 0.5 * (f.data[n] - back)).clamp(lo, hi);
        });
        result
    }

    /// Heat and moisture from the ground into the lowest level, in patches.
    fn surface_fluxes(&mut self, dt: f64) {
        let p = self.params;
        let (nx, _, nz) = p.size;
        let h = p.cell;
        let t = self.time / p.patch_period;
        let seed = (p.seed as u32).wrapping_add(self.steps as u32 * 7919);
        let heat = dt / (AIR_DENSITY * CP * h);
        let moisture = dt / (AIR_DENSITY * LV * h);
        let patches = &self.patches;
        let pattern = |i: usize, k: usize| {
            let (x, z) = ((i as f64 + 0.5) * h / p.patch_size, (k as f64 + 0.5) * h / p.patch_size);
            let n = crate::procedural::nubis_clouds::stretched(patches.fbm(x, t, z, 3, 0.5, 2.));
            // Mean 1; from 1 - patchiness to 1 + patchiness.
            (1. + p.patchiness * (n - 0.5) * 2.).max(0.)
        };
        let factors: Vec<f64> = (0..nx * nz).into_par_iter().map(|n| {
            let (i, k) = (n % nx, n / nx);
            pattern(i, k) * (1. + (worley_hash(i as u32, k as u32, 0, seed) - 0.5) * 0.5)
        }).collect();
        for (n, f) in factors.iter().enumerate() {
            self.theta.data[n] += p.heat_flux * f * heat;
            self.vapour.data[n] += p.moisture_flux * f * moisture;
        }
    }

    /// Brings each cell to saturation: vapour beyond it condenses, cloud water in unsaturated
    /// air evaporates, and the latent heat goes into (or comes out of) θ. Then rain: cloud
    /// water beyond `RAIN_THRESHOLD` turns to rain at `RAIN_RATE` (Kessler's autoconversion)
    /// and falls out of the air.
    fn condense(&mut self, dt: f64) {
        let rain = 1. - (-RAIN_RATE * dt).exp();
        let (nx, _, nz) = self.params.size;
        let slab = nx * nz;
        let (exner, pressure) = (&self.exner, &self.pressure);
        self.theta.data.par_chunks_mut(slab)
            .zip(self.vapour.data.par_chunks_mut(slab))
            .zip(self.cloud.data.par_chunks_mut(slab))
            .enumerate()
            .for_each(|(j, ((theta, vapour), cloud))| {
                for n in 0..slab {
                    let (dv, dc, dth) = adjust(theta[n], vapour[n], cloud[n], exner[j], pressure[j]);
                    vapour[n] += dv;
                    cloud[n] += dc;
                    theta[n] += dth;
                    if cloud[n] > RAIN_THRESHOLD {
                        cloud[n] -= (cloud[n] - RAIN_THRESHOLD) * rain;
                    }
                }
            });
    }

    /// Buoyancy, vorticity confinement, and the sponge at the top.
    fn forces(&mut self, dt: f64) {
        let (nx, ny, nz) = self.params.size;
        let slab = nx * nz;
        let h = self.params.cell;
        // Buoyancy at the cells' centres.
        let buoyancy: Vec<f64> = (0..nx * ny * nz).into_par_iter().map(|n| {
            let j = n / slab;
            GRAVITY * ((self.theta.data[n] - self.theta_env[j]) / self.theta_env[j]
                + 0.608 * (self.vapour.data[n] - self.vapour_env[j]) - self.cloud.data[n])
        }).collect();
        // Vorticity confinement: a force pushing along N x ω, N the direction in which the
        // vorticity's magnitude grows.
        let eps = self.params.vorticity;
        let confinement: Vec<Vector3<f64>> = if eps > 0. {
            // Only for its indexing.
            let c = Field::new(nx, 1, nz, h, Vector3::zeros());
            let centre = |i: i64, j: usize, k: i64| {
                let (i0, k0) = (i.rem_euclid(nx as i64) as usize, k.rem_euclid(nz as i64) as usize);
                let (i1, k1) = ((i0 + 1) % nx, (k0 + 1) % nz);
                Vector3::new(
                    0.5 * (self.u.at(i0, j, k0) + self.u.at(i1, j, k0)),
                    0.5 * (self.v.at(i0, j, k0) + self.v.at(i0, j + 1, k0)),
                    0.5 * (self.w.at(i0, j, k0) + self.w.at(i0, j, k1)),
                )
            };
            let up = |j: usize| (j + 1).min(ny - 1);
            let down = |j: usize| j.saturating_sub(1);
            let omega: Vec<Vector3<f64>> = (0..nx * ny * nz).into_par_iter().map(|n| {
                let (i, j, k) = c.ijk(n);
                let (i, k) = (i as i64, k as i64);
                let dy = (up(j) - down(j)) as f64 * h;
                let ddx = (centre(i + 1, j, k) - centre(i - 1, j, k)) / (2. * h);
                let ddy = (centre(i, up(j), k) - centre(i, down(j), k)) / dy;
                let ddz = (centre(i, j, k + 1) - centre(i, j, k - 1)) / (2. * h);
                Vector3::new(ddy.z - ddz.y, ddz.x - ddx.z, ddx.y - ddy.x)
            }).collect();
            let mag = |i: i64, j: usize, k: i64| omega[c.idx(i.rem_euclid(nx as i64) as usize, j, k.rem_euclid(nz as i64) as usize)].norm();
            (0..nx * ny * nz).into_par_iter().map(|n| {
                let (i, j, k) = c.ijk(n);
                let (i, k) = (i as i64, k as i64);
                let dy = (up(j) - down(j)) as f64 * h;
                let grad = Vector3::new(
                    (mag(i + 1, j, k) - mag(i - 1, j, k)) / (2. * h),
                    (mag(i, up(j), k) - mag(i, down(j), k)) / dy,
                    (mag(i, j, k + 1) - mag(i, j, k - 1)) / (2. * h),
                );
                let norm = grad.norm();
                if norm < 1e-12 {
                    return Vector3::zeros();
                }
                (grad / norm).cross(&omega[n]) * (eps * h)
            }).collect()
        } else {
            vec![Vector3::zeros(); nx * ny * nz]
        };
        // Onto the faces.
        let centre_idx = |i: usize, j: usize, k: usize| i + nx * (k + nz * j);
        self.u.data.par_iter_mut().enumerate().for_each(|(n, u)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            let a = centre_idx((i + nx - 1) % nx, j, k);
            *u += dt * 0.5 * (confinement[a].x + confinement[centre_idx(i, j, k)].x);
        });
        self.w.data.par_iter_mut().enumerate().for_each(|(n, w)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            let a = centre_idx(i, j, (k + nz - 1) % nz);
            *w += dt * 0.5 * (confinement[a].z + confinement[centre_idx(i, j, k)].z);
        });
        self.v.data.par_iter_mut().enumerate().for_each(|(n, v)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            if j == 0 || j == ny {
                return;
            }
            let (a, b) = (centre_idx(i, j - 1, k), centre_idx(i, j, k));
            *v += dt * 0.5 * (buoyancy[a] + buoyancy[b] + confinement[a].y + confinement[b].y);
        });
        // The sponge: the top fifth relaxes towards the environment, faster further up.
        let start = (ny as f64 * 0.8) as usize;
        for j in start..ny {
            let s = (j - start + 1) as f64 / (ny - start) as f64;
            let keep = (-dt * s * s / 60.).exp();
            for n in j * slab..(j + 1) * slab {
                self.u.data[n] *= keep;
                self.w.data[n] *= keep;
                self.v.data[n + slab] *= keep;
                self.theta.data[n] = self.theta_env[j] + (self.theta.data[n] - self.theta_env[j]) * keep;
                self.vapour.data[n] = self.vapour_env[j] + (self.vapour.data[n] - self.vapour_env[j]) * keep;
                self.cloud.data[n] *= keep;
            }
        }
    }

    /// Makes the velocity divergence-free: subtracts the gradient of φ, where ∇²φ = ∇·u.
    /// Periodic in x and z, so in Fourier modes there; each mode's φ up the column then solves
    /// a tridiagonal system, with no flow through the ground or the top.
    pub fn project(&mut self) {
        let (nx, ny, nz) = self.params.size;
        let slab = nx * nz;
        let h = self.params.cell;
        let mut spectrum: Vec<Complex<f64>> = (0..nx * ny * nz).into_par_iter().map(|n| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            let du = self.u.at((i + 1) % nx, j, k) - self.u.at(i, j, k);
            let dv = self.v.at(i, j + 1, k) - self.v.at(i, j, k);
            let dw = self.w.at(i, j, (k + 1) % nz) - self.w.at(i, j, k);
            Complex::new((du + dv + dw) / h, 0.)
        }).collect();
        spectrum.par_chunks_mut(slab).for_each(|s| crate::glare::fft2d(s, nx, nz, false));
        // Up each column, mode by mode.
        let columns: Vec<Vec<Complex<f64>>> = (0..slab).into_par_iter().map(|m| {
            let (a, b) = (m % nx, m / nx);
            let lambda = (2. * (std::f64::consts::TAU * a as f64 / nx as f64).cos() - 2.
                + 2. * (std::f64::consts::TAU * b as f64 / nz as f64).cos() - 2.) / (h * h);
            let rhs: Vec<Complex<f64>> = (0..ny).map(|j| spectrum[j * slab + m]).collect();
            solve_column(&rhs, lambda, h)
        }).collect();
        let norm = 1. / slab as f64;
        spectrum.par_chunks_mut(slab).enumerate().for_each(|(j, s)| {
            for (m, c) in s.iter_mut().enumerate() {
                *c = columns[m][j];
            }
            crate::glare::fft2d(s, nx, nz, true);
            for c in s.iter_mut() {
                *c *= norm;
            }
        });
        let phi = |i: usize, j: usize, k: usize| spectrum[i + nx * (k + nz * j)].re;
        self.u.data.par_iter_mut().enumerate().for_each(|(n, u)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            *u -= (phi(i, j, k) - phi((i + nx - 1) % nx, j, k)) / h;
        });
        self.w.data.par_iter_mut().enumerate().for_each(|(n, w)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            *w -= (phi(i, j, k) - phi(i, j, (k + nz - 1) % nz)) / h;
        });
        self.v.data.par_iter_mut().enumerate().for_each(|(n, v)| {
            let (i, j, k) = (n % nx, n / slab, (n / nx) % nz);
            if j > 0 && j < ny {
                *v -= (phi(i, j, k) - phi(i, j - 1, k)) / h;
            }
        });
    }

    /// The divergence at each cell's centre (for tests).
    pub fn divergence(&self) -> Vec<f64> {
        let (nx, ny, nz) = self.params.size;
        let h = self.params.cell;
        (0..nx * ny * nz).map(|n| {
            let (i, j, k) = (n % nx, n / (nx * nz), (n / nx) % nz);
            (self.u.at((i + 1) % nx, j, k) - self.u.at(i, j, k) + self.v.at(i, j + 1, k) - self.v.at(i, j, k)
                + self.w.at(i, j, (k + 1) % nz) - self.w.at(i, j, k)) / h
        }).collect()
    }

    /// A step short enough that nothing crosses more than a cell.
    pub fn stable_step(&self) -> f64 {
        let fastest = |f: &Field| f.data.par_iter().map(|v| v.abs()).reduce(|| 0., f64::max);
        let speed = (fastest(&self.u) + self.params.wind.x.abs())
            .max(fastest(&self.v))
            .max(fastest(&self.w) + self.params.wind.y.abs());
        self.params.max_step.min(self.params.cell / speed.max(1e-9))
    }

    pub fn step(&mut self, dt: f64) {
        let u = self.advect(&self.u, dt, false);
        let v = self.advect(&self.v, dt, true);
        let w = self.advect(&self.w, dt, false);
        let theta = self.advect(&self.theta, dt, false);
        let vapour = self.advect(&self.vapour, dt, false);
        let cloud = self.advect(&self.cloud, dt, false);
        (self.u, self.v, self.w, self.theta, self.vapour, self.cloud) = (u, v, w, theta, vapour, cloud);
        self.surface_fluxes(dt);
        self.condense(dt);
        self.forces(dt);
        self.project();
        self.time += dt;
        self.steps += 1;
    }

    /// Runs to `time`, reporting progress every simulated minute.
    pub fn advance_to(&mut self, time: f64, mut progress: impl FnMut(&CloudSimulation)) {
        let mut reported = (self.time / 60.).floor();
        while self.time < time - 1e-9 {
            let dt = self.stable_step().min(time - self.time);
            self.step(dt);
            if (self.time / 60.).floor() > reported {
                reported = (self.time / 60.).floor();
                progress(self);
            }
        }
    }

    /// Height of the lowest level at least 1% cloud (cloud water over 1e-5), if any (for
    /// tests and reports).
    pub fn cloud_base(&self) -> Option<f64> {
        let (nx, ny, nz) = self.params.size;
        let slab = nx * nz;
        (0..ny).find(|j| self.cloud.data[j * slab..(j + 1) * slab].iter().filter(|q| **q > 1e-5).count() * 100 >= slab)
            .map(|j| (j as f64 + 0.5) * self.params.cell)
    }

    /// Writes the state to `path` (raw little-endian floats after a short header).
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let mut bytes = Vec::with_capacity(8 * (2 + 6 * self.theta.data.len() + self.v.data.len()));
        bytes.extend_from_slice(&self.time.to_le_bytes());
        bytes.extend_from_slice(&self.steps.to_le_bytes());
        for f in [&self.u, &self.v, &self.w, &self.theta, &self.vapour, &self.cloud] {
            for x in &f.data {
                bytes.extend_from_slice(&x.to_le_bytes());
            }
        }
        let tmp = path.with_extension("partial");
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(tmp, path)
    }

    /// A simulation with `params`, its state read from `path`.
    pub fn load(params: SimulationParams, path: &std::path::Path) -> std::io::Result<CloudSimulation> {
        let mut sim = CloudSimulation::new(params);
        let bytes = std::fs::read(path)?;
        let expected = 8 * (2 + sim.u.data.len() + sim.v.data.len() + sim.w.data.len() + 3 * sim.theta.data.len());
        if bytes.len() != expected {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "wrong size"));
        }
        let mut words = bytes.chunks_exact(8).map(|c| <[u8; 8]>::try_from(c).unwrap());
        sim.time = f64::from_le_bytes(words.next().unwrap());
        sim.steps = u64::from_le_bytes(words.next().unwrap());
        for f in [&mut sim.u, &mut sim.v, &mut sim.w, &mut sim.theta, &mut sim.vapour, &mut sim.cloud] {
            for x in f.data.iter_mut() {
                *x = f64::from_le_bytes(words.next().unwrap());
            }
        }
        Ok(sim)
    }

    /// The simulation at `time`, continuing from the latest state saved in `cache` (if given)
    /// for the same parameters at or before it, and saving the result there: so the frames
    /// of an animation, rendered in order, each simulate only from the one before. Keeps the
    /// two latest states.
    pub fn cached(params: SimulationParams, time: f64, cache: Option<&std::path::Path>) -> CloudSimulation {
        let report = |s: &CloudSimulation| eprintln!("  harris_cumulus: {:.0} of {:.0} s simulated{}", s.time, time,
            s.cloud_base().map_or(String::new(), |b| format!(", cloud base {:.0} m", b)));
        let Some(dir) = cache else {
            let mut sim = CloudSimulation::new(params);
            sim.advance_to(time, report);
            return sim;
        };
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", params).hash(&mut hasher);
        let prefix = format!("harris-{:016x}-", hasher.finish());
        let _ = std::fs::create_dir_all(dir);
        let mut saved: Vec<(u64, std::path::PathBuf)> = std::fs::read_dir(dir).into_iter().flatten().flatten()
            .filter_map(|e| {
                let name = e.file_name().into_string().ok()?;
                let ms = name.strip_prefix(&prefix)?.strip_suffix(".state")?.parse::<u64>().ok()?;
                Some((ms, e.path()))
            }).collect();
        saved.sort();
        let wanted = (time * 1000.).round() as u64;
        let mut sim = saved.iter().rev().filter(|(ms, _)| *ms <= wanted)
            .find_map(|(_, path)| CloudSimulation::load(params, path).ok())
            .unwrap_or_else(|| CloudSimulation::new(params));
        if sim.time < time - 1e-9 {
            sim.advance_to(time, report);
            let path = dir.join(format!("{}{:012}.state", prefix, wanted));
            if let Err(e) = sim.save(&path) {
                eprintln!("  harris_cumulus: couldn't cache the simulation in {}: {}", path.display(), e);
            }
            // Keep this one and the latest before it.
            let newest = saved.iter().map(|(ms, _)| *ms).filter(|ms| *ms != wanted).max();
            for (ms, p) in saved.iter() {
                if *ms != wanted && Some(*ms) != newest {
                    let _ = std::fs::remove_file(p);
                }
            }
        }
        sim
    }
}

/// The change in (vapour, cloud water, θ) that brings a cell to saturation: Newton's method on
/// the saturation curve, as condensing warms the air and raises it.
fn adjust(theta: f64, vapour: f64, cloud: f64, exner: f64, pressure: f64) -> (f64, f64, f64) {
    let mut condensed = 0.;
    for _ in 0..3 {
        let t = (theta + LV / (CP * exner) * condensed) * exner;
        let qs = saturation_mixing_ratio(t, pressure);
        // d qs / dT, from Tetens' formula.
        let slope = qs * 17.67 * 243.5 / ((t - 29.65) * (t - 29.65));
        condensed += (vapour - condensed - qs) / (1. + LV / CP * slope);
        // No more evaporates than there is cloud.
        condensed = condensed.max(-cloud);
    }
    (-condensed, condensed, LV / (CP * exner) * condensed)
}

/// Solves (φ[j-1] - 2φ[j] + φ[j+1]) / h² + λ φ[j] = d[j] up a column, with zero flux through
/// the ends (φ[-1] = φ[0], φ[n] = φ[n-1]); for λ = 0, where φ is only defined up to a
/// constant, with φ[0] = 0.
fn solve_column(d: &[Complex<f64>], lambda: f64, h: f64) -> Vec<Complex<f64>> {
    let n = d.len();
    let h2 = h * h;
    let off = 1. / h2;
    let diag = |j: usize| {
        let ends = if j == 0 || j == n - 1 { -1. } else { -2. };
        ends / h2 + lambda
    };
    let pinned = lambda.abs() < 1e-12 / h2;
    // Thomas' algorithm.
    let mut c = vec![0.; n];
    let mut r = vec![Complex::new(0., 0.); n];
    let (b0, c0, d0) = if pinned { (1., 0., Complex::new(0., 0.)) } else { (diag(0), off, d[0]) };
    c[0] = c0 / b0;
    r[0] = d0 / b0;
    for j in 1..n {
        let a = off;
        let cj = if j < n - 1 { off } else { 0. };
        let m = diag(j) - a * c[j - 1];
        c[j] = cj / m;
        r[j] = (d[j] - r[j - 1] * a) / m;
    }
    let mut x = vec![Complex::new(0., 0.); n];
    x[n - 1] = r[n - 1];
    for j in (0..n - 1).rev() {
        x[j] = r[j] - x[j + 1] * c[j];
    }
    x
}

/// The simulated clouds, for rendering: cloud water turned into extinction, with eroded detail
/// finer than the grid, and (with `repeat`) the periodic domain tiled across the sky.
pub struct HarrisCumulus {
    cloud: Vec<f32>,
    size: (usize, usize, usize),
    h: f64,
    /// Scene position of the grid's lowest corner.
    pub origin: Vector3<f64>,
    /// Extinction (per metre) per kg/kg of cloud water: 3 ρ_air / (2 ρ_water r) for droplets of
    /// effective radius r.
    pub extinction: f64,
    pub droplet_radius: f64,
    /// Detail: Worley noise `detail_size` across eats up to `erosion` x `edge` (kg/kg) of cloud
    /// water, so thin edges break into billows; it drifts by `drift` (the wind's travel).
    pub detail_size: f64,
    pub erosion: f64,
    pub edge: f64,
    pub drift: Vector3<f64>,
    /// Tile the domain across the sky, fading out `extent` from `center` over its outer `fade`.
    pub repeat: bool,
    pub center: Vector2<f64>,
    pub extent: f64,
    pub fade: f64,
    blocks: Vec<f32>,
    block_counts: (usize, usize, usize),
    max: f64,
    detail: std::sync::OnceLock<crate::procedural::nubis_clouds::WorleyTexture>,
    seed: u32,
}

/// Cells along each side of a majorant block.
const BLOCK: usize = 4;

impl HarrisCumulus {
    /// The clouds of `sim`, the grid centred over `center` on the ground.
    pub fn new(sim: &CloudSimulation, droplet_radius: f64, center: Vector2<f64>) -> HarrisCumulus {
        let (nx, ny, nz) = sim.params.size;
        let h = sim.params.cell;
        let cloud: Vec<f32> = sim.cloud.data.iter().map(|q| q.max(0.) as f32).collect();
        let counts = (nx.div_ceil(BLOCK), ny.div_ceil(BLOCK), nz.div_ceil(BLOCK));
        // Each block's largest value over the cells a point in it interpolates between: one
        // more on each side.
        let blocks = (0..counts.0 * counts.1 * counts.2).into_par_iter().map(|b| {
            let (bx, by, bz) = (b % counts.0, b / (counts.0 * counts.2), (b / counts.0) % counts.2);
            let mut m = 0f32;
            for j in (by * BLOCK) as i64 - 1..=((by + 1) * BLOCK) as i64 {
                let j = j.clamp(0, ny as i64 - 1) as usize;
                for k in (bz * BLOCK) as i64 - 1..=((bz + 1) * BLOCK) as i64 {
                    let k = k.rem_euclid(nz as i64) as usize;
                    for i in (bx * BLOCK) as i64 - 1..=((bx + 1) * BLOCK) as i64 {
                        let i = i.rem_euclid(nx as i64) as usize;
                        m = m.max(cloud[i + nx * (k + nz * j)]);
                    }
                }
            }
            m
        }).collect();
        let max = cloud.iter().cloned().fold(0f32, f32::max) as f64;
        HarrisCumulus {
            cloud,
            size: (nx, ny, nz),
            h,
            origin: Vector3::new(center.x - nx as f64 * h / 2., 0., center.y - nz as f64 * h / 2.),
            extinction: 1.5 * 1.0 / (1000. * droplet_radius * 1e-6),
            droplet_radius,
            detail_size: 80.,
            erosion: 0.6,
            edge: 4e-4,
            drift: Vector3::new(sim.params.wind.x, 0., sim.params.wind.y) * sim.time,
            repeat: true,
            center,
            extent: 30_000.,
            fade: 0.3,
            blocks,
            block_counts: counts,
            max,
            detail: std::sync::OnceLock::new(),
            seed: sim.params.seed as u32,
        }
    }

    fn size_m(&self) -> Vector3<f64> {
        Vector3::new(self.size.0 as f64, self.size.1 as f64, self.size.2 as f64) * self.h
    }

    /// Cloud water (kg/kg) at grid-local `l`, interpolated between cells' centres.
    fn water(&self, l: &Vector3<f64>) -> f64 {
        let (nx, ny, nz) = self.size;
        let g = l / self.h - Vector3::repeat(0.5);
        let (x0, z0) = (g.x.floor(), g.z.floor());
        let (fx, fz) = (g.x - x0, g.z - z0);
        let gy = g.y.clamp(0., (ny - 1) as f64);
        let y0 = (gy.floor() as usize).min(ny - 2);
        let fy = gy - y0 as f64;
        let (xa, za) = ((x0 as i64).rem_euclid(nx as i64) as usize, (z0 as i64).rem_euclid(nz as i64) as usize);
        let (xb, zb) = ((xa + 1) % nx, (za + 1) % nz);
        let c = |i: usize, j: usize, k: usize| self.cloud[i + nx * (k + nz * j)] as f64;
        let lerp = |a: f64, b: f64, t: f64| a + (b - a) * t;
        let level = |j: usize| lerp(lerp(c(xa, j, za), c(xb, j, za), fx), lerp(c(xa, j, zb), c(xb, j, zb), fx), fz);
        lerp(level(y0), level(y0 + 1), fy)
    }

    fn inside(&self, l: &Vector3<f64>) -> bool {
        let s = self.size_m();
        l.y > 0. && l.y < s.y && (self.repeat || (l.x >= 0. && l.x <= s.x && l.z >= 0. && l.z <= s.z))
    }

    fn fade_at(&self, p: &Vector3<f64>) -> f64 {
        if !self.repeat {
            return 1.;
        }
        let from_centre = (Vector2::new(p.x, p.z) - self.center).abs().max() / self.extent;
        crate::procedural::nubis_clouds::smoothstep(0., self.fade.max(1e-9), 1. - from_centre)
    }

    /// Cloud water after erosion, kg/kg.
    pub fn eroded(&self, p: &Vector3<f64>) -> f64 {
        let l = p - self.origin;
        if !self.inside(&l) {
            return 0.;
        }
        let q = self.water(&l);
        if q <= 0. {
            return 0.;
        }
        let d = if self.erosion > 0. {
            let cells = self.detail.get_or_init(|| crate::procedural::nubis_clouds::WorleyTexture::new(self.seed.wrapping_add(11)));
            crate::procedural::nubis_clouds::worley_fbm(cells, &((p - self.drift) / self.detail_size))
        } else {
            1.
        };
        (q - (1. - d) * self.erosion * self.edge).max(0.)
    }
}

impl crate::volume::DensityField for HarrisCumulus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let fade = self.fade_at(p);
        if fade <= 0. {
            return 0.;
        }
        self.extinction * self.eroded(p) * fade
    }

    fn majorant(&self) -> f64 {
        self.extinction * self.max
    }

    /// The largest cloud water in the block of cells ahead, to its far side; nothing below
    /// the ground, above the grid or (without `repeat`) beside it, up to where the ray enters.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let s = self.size_m();
        let l = p - self.origin;
        let ahead = l + d * (self.h * 1e-6);
        // Where the ray enters the slab (or box) of the grid, if it's outside.
        let lo = Vector3::new(if self.repeat { f64::NEG_INFINITY } else { 0. }, 0., if self.repeat { f64::NEG_INFINITY } else { 0. });
        let hi = Vector3::new(if self.repeat { f64::INFINITY } else { s.x }, s.y, if self.repeat { f64::INFINITY } else { s.z });
        if (0..3).any(|a| ahead[a] < lo[a] || ahead[a] > hi[a]) {
            let (mut enter, mut leave) = (0f64, f64::INFINITY);
            for a in 0..3 {
                if d[a] == 0. {
                    if l[a] < lo[a] || l[a] > hi[a] {
                        return (0., f64::INFINITY);
                    }
                    continue;
                }
                let (t0, t1) = ((lo[a] - l[a]) / d[a], (hi[a] - l[a]) / d[a]);
                enter = enter.max(t0.min(t1));
                leave = leave.min(t0.max(t1));
            }
            return (0., if enter <= leave && leave > 0. { enter.max(0.) } else { f64::INFINITY });
        }
        let side = BLOCK as f64 * self.h;
        let (bx, by, bz) = ((ahead.x / side).floor(), (ahead.y / side).floor(), (ahead.z / side).floor());
        let exit = |a: usize, b: f64| {
            if d[a] > 0. { ((b + 1.) * side - l[a]) / d[a] } else if d[a] < 0. { (b * side - l[a]) / d[a] } else { f64::INFINITY }
        };
        let length = exit(0, bx).min(exit(1, by)).min(exit(2, bz)).max(0.);
        let (cx, cy, cz) = self.block_counts;
        let (i, j, k) = ((bx as i64).rem_euclid(cx as i64) as usize, (by as usize).min(cy - 1), (bz as i64).rem_euclid(cz as i64) as usize);
        (self.extinction * self.blocks[i + cx * (k + cz * j)] as f64, length)
    }

    fn particle_radius(&self, _p: &Vector3<f64>) -> Option<f64> {
        Some(self.droplet_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(size: (usize, usize, usize), cell: f64) -> SimulationParams {
        SimulationParams { size, cell, wind: Vector2::zeros(), vorticity: 0., ..SimulationParams::default() }
    }

    #[test]
    fn test_projection_removes_divergence() {
        let mut sim = CloudSimulation::new(small((16, 12, 8), 50.));
        for (n, x) in sim.u.data.iter_mut().enumerate() { *x = worley_hash(n as u32, 1, 2, 3) - 0.5; }
        for (n, x) in sim.w.data.iter_mut().enumerate() { *x = worley_hash(n as u32, 4, 5, 6) - 0.5; }
        let ny = 12;
        for (n, x) in sim.v.data.iter_mut().enumerate() {
            let j = n / (16 * 8);
            *x = if j == 0 || j == ny { 0. } else { worley_hash(n as u32, 7, 8, 9) - 0.5 };
        }
        assert!(sim.divergence().iter().any(|d| d.abs() > 1e-3));
        sim.project();
        let worst = sim.divergence().iter().fold(0f64, |m, d| m.max(d.abs()));
        assert!(worst < 1e-10, "{}", worst);
        // Projecting again changes nothing.
        let before = sim.u.data.clone();
        sim.project();
        assert!(sim.u.data.iter().zip(&before).all(|(a, b)| (a - b).abs() < 1e-10));
    }

    #[test]
    fn test_steady_wind_carries_a_blob() {
        let mut p = small((32, 4, 16), 10.);
        p.wind = Vector2::new(5., 0.);
        p.heat_flux = 0.;
        p.moisture_flux = 0.;
        let mut sim = CloudSimulation::new(p);
        for x in sim.cloud.data.iter_mut() { *x = 0.; }
        let (i0, k0) = (8, 8);
        for j in 0..4 { let n = sim.cloud.idx(i0, j, k0); sim.cloud.data[n] = 1e-3; }
        let centroid = |s: &CloudSimulation| {
            let (mut m, mut x) = (0., 0.);
            for n in 0..s.cloud.data.len() {
                let q = s.cloud.data[n];
                m += q;
                x += q * s.cloud.position(n).x;
            }
            x / m
        };
        let before = centroid(&sim);
        let cloud = sim.advect(&sim.cloud, 4., false);
        sim.cloud = cloud;
        assert!((centroid(&sim) - before - 20.).abs() < 1., "{} -> {}", before, centroid(&sim));
        // Nothing new: no negative cloud, no more than there was.
        assert!(sim.cloud.data.iter().all(|q| *q >= 0. && *q <= 1e-3 + 1e-12));
    }

    #[test]
    fn test_condensation_conserves_water_and_energy() {
        let (exner, p) = (exner_at(1500.), pressure_at(1500.));
        for (theta, vapour, cloud) in [(300., 0.02, 0.), (300., 0.005, 1e-3), (295., 0.013, 2e-4)] {
            let (dv, dc, dth) = adjust(theta, vapour, cloud, exner, p);
            // Total water is kept, and so is the liquid-water potential temperature.
            assert!((dv + dc).abs() < 1e-15);
            assert!((dth - LV / (CP * exner) * dc).abs() < 1e-12);
            let (v, c, th) = (vapour + dv, cloud + dc, theta + dth);
            assert!(c >= 0.);
            let qs = saturation_mixing_ratio(th * exner, p);
            if c > 1e-9 {
                // Saturated, to within the Newton step's error.
                assert!((v - qs).abs() < 1e-4 * qs, "{} vs {}", v, qs);
            } else {
                assert!(v <= qs * 1.0001);
            }
        }
    }

    #[test]
    fn test_atmosphere_profile() {
        let a = Atmosphere::default();
        assert_eq!(a.theta(500.), a.surface_theta);
        assert!((a.theta(2000.) - a.surface_theta - a.lapse * 800.).abs() < 1e-9);
        assert!(a.theta(3300.) > a.theta(2999.) + a.inversion_strength);
        // The mixed layer is unsaturated at the ground and saturated near its top: its
        // condensation level, where a thermal from the ground would form cloud.
        let rh = |z: f64| a.vapour(z) / saturation_mixing_ratio(a.theta(z) * exner_at(z), pressure_at(z));
        assert!(rh(0.) < 0.7);
        assert!((0.9..1.).contains(&rh(1150.)), "{}", rh(1150.));
        assert!(rh(2000.) <= a.humidity_above + 1e-9);
    }

    #[test]
    fn test_warm_bubble_rises() {
        let mut p = small((16, 24, 16), 50.);
        p.heat_flux = 0.;
        p.moisture_flux = 0.;
        let mut sim = CloudSimulation::new(p);
        let height = |s: &CloudSimulation| {
            let (mut m, mut y) = (0., 0.);
            for n in 0..s.theta.data.len() {
                let j = n / (16 * 16);
                let excess = (s.theta.data[n] - s.theta_env[j] - 0.1).max(0.);
                m += excess;
                y += excess * s.theta.position(n).y;
            }
            y / m
        };
        for n in 0..sim.theta.data.len() {
            let pos = sim.theta.position(n);
            if (pos - Vector3::new(400., 300., 400.)).norm() < 150. {
                sim.theta.data[n] += 2.;
            }
        }
        let start = height(&sim);
        sim.advance_to(120., |_| {});
        assert!(height(&sim) > start + 150., "{} -> {}", start, height(&sim));
    }

    #[test]
    fn test_field_majorant_bounds_density() {
        use crate::volume::DensityField;
        let mut sim = CloudSimulation::new(small((16, 12, 16), 50.));
        for (n, q) in sim.cloud.data.iter_mut().enumerate() {
            *q = (worley_hash(n as u32, 0, 0, 1) - 0.7).max(0.) * 3e-3;
        }
        for repeat in [true, false] {
            let mut f = HarrisCumulus::new(&sim, 10., Vector2::new(100., -50.));
            f.repeat = repeat;
            let mut rng = crate::geometry::Pcg32::new(4, 0);
            let mut u = || rng.next_f64();
            for _ in 0..2000 {
                let p = Vector3::new(u() * 3000. - 1500., u() * 1000. - 200., u() * 3000. - 1500.);
                let d = Vector3::new(u() - 0.5, u() - 0.5, u() - 0.5).normalize();
                let (m, len) = f.majorant_from(&p, &d);
                assert!(len > 0., "{} {}", p, d);
                for k in 0..20 {
                    let t = len.min(2000.) * (k as f64 + 0.5) / 20.;
                    assert!(f.density(&(p + d * t)) <= m * (1. + 1e-6), "{} along {} at {}", p, d, t);
                }
            }
            // Cloud water becomes extinction: 10 µm droplets, 1 g/kg, 0.15 per metre.
            assert!((f.extinction * 1e-3 - 0.15).abs() < 1e-9);
        }
    }

    #[test]
    fn test_heated_ground_grows_cumulus_at_the_condensation_level() {
        // A coarse day: thermals from the heated ground should reach the top of the mixed
        // layer and condense there.
        let mut p = SimulationParams { size: (32, 40, 32), cell: 100., ..SimulationParams::default() };
        p.heat_flux = 400.;
        let mut sim = CloudSimulation::new(p);
        sim.advance_to(15. * 60., |_| {});
        let base = sim.cloud_base().expect("no cloud formed");
        assert!((900. ..1800.).contains(&base), "cloud base {}", base);
        // And it is a cloud, not a fog: most of the lowest kilometre is clear.
        let clear = sim.cloud.data[..32 * 32 * 10].iter().filter(|q| **q < 1e-5).count();
        assert_eq!(clear, 32 * 32 * 10);
    }
}


#[cfg(test)]
mod diagnostics {
    use super::*;

    /// Prints a profile of a cached state: `HARRIS_STATE=<file> cargo test --release harris_profile -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn harris_profile() {
        let path = std::env::var("HARRIS_STATE").expect("HARRIS_STATE");
        let sim = CloudSimulation::load(SimulationParams::default(), std::path::Path::new(&path)).unwrap();
        let (nx, ny, nz) = sim.params.size;
        let slab = nx * nz;
        eprintln!("t = {}", sim.time);
        for j in (0..ny).step_by(2) {
            let r = j * slab..(j + 1) * slab;
            let mean = |f: &Field| f.data[r.clone()].iter().sum::<f64>() / slab as f64;
            let cloudy = sim.cloud.data[r.clone()].iter().filter(|q| **q > 1e-5).count() as f64 / slab as f64;
            let qs = saturation_mixing_ratio(mean(&sim.theta) * sim.exner[j], sim.pressure[j]);
            let wmax = sim.v.data[r.clone()].iter().cloned().fold(f64::MIN, f64::max);
            eprintln!("z {:5.0}  θ' {:+.2}  qv {:.2} g/kg  RH {:.2}  cloud {:.2}  qc max {:.2} g/kg  w max {:.1}",
                (j as f64 + 0.5) * sim.params.cell, mean(&sim.theta) - sim.theta_env[j], mean(&sim.vapour) * 1e3,
                mean(&sim.vapour) / qs, cloudy, sim.cloud.data[r.clone()].iter().cloned().fold(0., f64::max) * 1e3, wmax);
        }
    }
}

#[cfg(test)]
mod tuning {
    use super::*;

    #[test]
    #[ignore]
    fn harris_tune() {
        let cases: Vec<(&str, f64, f64, f64, f64)> = vec![
            // name, lapse K/km, inversion m, inversion strength K, heat flux
            ("6/2000/5/200", 6., 2000., 5., 200.),
            ("6.5/2200/5/200", 6.5, 2200., 5., 200.),
            ("5.5/2000/6/200", 5.5, 2000., 6., 200.),
        ];
        rayon::ThreadPoolBuilder::new().build().unwrap().install(|| {
            for (name, lapse, inv, strength, flux) in cases {
                let mut p = SimulationParams { size: (64, 40, 64), cell: 100., ..SimulationParams::default() };
                p.atmosphere.lapse = lapse * 1e-3;
                p.atmosphere.inversion = inv;
                p.atmosphere.inversion_strength = strength;
                p.heat_flux = flux;
                let mut sim = CloudSimulation::new(p);
                for minutes in [10., 20., 30., 40., 50., 60.] {
                    sim.advance_to(minutes * 60., |_| {});
                    let (nx, ny, nz) = p.size;
                    let slab = nx * nz;
                    let cover = (0..slab).filter(|n| (0..ny).any(|j| sim.cloud.data[j * slab + n] > 1e-5)).count() as f64 / slab as f64;
                    let top = (0..ny).rev().find(|j| sim.cloud.data[j * slab..(j + 1) * slab].iter().any(|q| *q > 1e-5)).map(|j| (j as f64 + 0.5) * p.cell);
                    let wmax = sim.v.data.iter().cloned().fold(f64::MIN, f64::max);
                    eprintln!("{} t {:3.0} min: cover {:.2} base {:?} top {:?} w {:.1} qc {:.2} g/kg", name, minutes, cover, sim.cloud_base(), top, wmax,
                        sim.cloud.data.iter().cloned().fold(0., f64::max) * 1e3);
                }
            }
        });
    }
}
