//! # Glaciers
//!
//! Glaciers carve the U-shaped valleys, cirques, overdeepened basins and fjords that rivers
//! can't. Following the glacial landscape evolution models of Braun, Zwartz and Tomkin ("A new
//! surface-processes model combining glacial and fluvial erosion", Annals of Glaciology 28,
//! 1999, doi:10.3189/172756499781821797) and Herman and Braun ("Evolution of the glacial
//! landscape of the Southern Alps of New Zealand", JGR Earth Surface 113, 2008,
//! doi:10.1029/2007JF000807), each time step:
//!
//! 1. computes the climate: where snow accumulates and where ice melts;
//! 2. moves ice downhill under its own weight (the shallow ice approximation);
//! 3. erodes the bed where ice slides over it.
//!
//! Cordonnier et al. ("Forming Terrains by Glacial Erosion", ACM TOG 42(4), 2023,
//! doi:10.1145/3592422) bring this to graphics, with higher-order ice flow emulated by a neural
//! network, and add quarrying, debris transport and hillslope processes on newly exposed
//! ground. Here ice flow is the simpler shallow ice approximation, as in the geomorphology
//! models above.
//!
//! ## Climate and mass balance
//!
//! Snow accumulates above the *equilibrium line altitude* (ELA) and ice melts below it. The
//! mass balance (metres of ice gained per year) grows linearly with height above the ELA, up to
//! a cap where it becomes too cold and dry for more snow:
//!
//! ```text
//! ṁ(s) = min(β (s - ELA), ṁ_max)
//! ```
//!
//! The ELA falls with latitude φ, from `ela_equator` to `ela_pole`, as cos²φ, and swings with
//! the glacial cycles: Hays, Imbrie and Shackleton ("Variations in the Earth's Orbit: Pacemaker
//! of the Ice Ages", Science 194, 1976, doi:10.1126/science.194.4270.1121) found the ice ages
//! paced by orbital cycles, dominated by a period of about 100,000 years. Here that is a single
//! cosine, warm (high ELA) at time zero:
//!
//! ```text
//! ELA(φ, t) = ela_pole + (ela_equator - ela_pole) cos²φ + ΔELA cos(2πt / period)
//! ```
//!
//! Open question: β, ṁ_max and ΔELA are of the order used by simple glacier models (β of a few
//! mm/yr per metre, ṁ_max of about 2 m/yr, a glacial ELA about 1 km lower), but have not yet been
//! checked against a source.
//!
//! ## Ice flow: the shallow ice approximation (SIA)
//!
//! Glaciers are much wider than they are thick, so the stress in the ice is, to leading order,
//! simple shear driven by the surface slope (Hutter, "Theoretical Glaciology", Reidel, 1983,
//! ch. 5). The basal shear stress is τ = ρ g H |∇s|, for ice thickness H and surface elevation
//! s = b + H over the bed b. Ice deforms by Glen's flow law ("The Creep of Polycrystalline Ice",
//! Proc. R. Soc. A 228, 1955, doi:10.1098/rspa.1955.0066), strain rate = A τ^n with n = 3, and
//! slides over its bed with a Weertman-type law ("On the Sliding of Glaciers", J. Glaciology
//! 3(21), 1957, doi:10.3189/S0022143000024709), u_s = A_s τ^n. Integrating over the thickness,
//! the ice flux is q = -D ∇s, with
//!
//! ```text
//! D = (ρ g)^n |∇s|^(n-1) ( 2A/(n+2) H^(n+2) + A_s H^(n+1) )
//! ∂H/∂t = ṁ - ∇·q
//! ```
//!
//! a non-linear diffusion of the surface. A = 2.4e-24 Pa⁻³ s⁻¹ is the value usually quoted for
//! temperate ice (Cuffey and Paterson, "The Physics of Glaciers", 4th ed., 2010; to verify).
//! Weertman's theory gives an exponent of (n+1)/2 for sliding; n is a common simplification.
//! A_s = 5e-15 m/yr/Pa³ (our choice, an open question) gives 5 m/yr of sliding at τ = 100 kPa
//! and 40 m/yr at 200 kPa, typical of driving stresses and speeds of valley glaciers. Real
//! sliding depends strongly on water pressure at the bed (Braun et al. 1999 divide by the
//! effective pressure), which is not modelled; with A_s ten times larger, thick ice at glacial
//! maxima slid at kilometres a year and dug basins kilometres deep.
//!
//! The SIA overestimates speeds where the ice is thick and steep, as it neglects the
//! longitudinal stresses that resist such flow, and these extremes also make the equations
//! stiffest. Speeds are capped at `max_speed` (1 km/yr; few glaciers but surging and tidewater
//! ones flow faster) by limiting D to max_speed H / |∇s|. Without the cap, ice a kilometre
//! thick at a glacial maximum on 500 m cells needed steps of a thousandth of a year.
//!
//! Following Mahaffy ("A three-dimensional numerical model of ice sheets", JGR 81(6), 1976,
//! doi:10.1029/JC081i006p01059), D is evaluated on the edges between cells, from the mean of
//! the two cells' thicknesses and squared slopes. A cell's squared slope is estimated from its
//! edge-sharing neighbours as (2/k) Σ ((s_j - s_i)/d)², which is exact for a plane when the k
//! neighbours are evenly spaced around it, but sees a peak as sloping.
//!
//! D grows as H⁵, so explicit steps must be minuscule where thick ice lies on steep slopes:
//! fractions of a year on 400 m cells. Instead each step is semi-implicit: D is taken from the
//! start of the step and the resulting linear system, symmetric positive definite, is solved by
//! conjugate gradients, preconditioned by an incomplete Cholesky factorisation. Very long steps are both inaccurate (D is
//! lagged) and slow to solve (the system's condition number grows with Δt D / Δx²), so the step
//! is shortened until no cell exchanges more than `max_stiffness` times its own volume
//! (Δt Σc / area); typically a few years, where an explicit scheme would need hundreds of steps.
//! `test_halfar_dome` checks the scheme against Halfar's exact solution for a spreading dome.
//!
//! A lagged D is computed from the old thickness, so in one long step a thin cell can take a
//! large inflow from a thick neighbour but pass little on (its own outflow conductance being
//! that of thin ice). On alpine terrain at glacial maxima this built transient single-cell
//! spikes kilometres thick. Picard iteration (`picard` solves per step: re-evaluate D from the
//! mean of the old and new thickness and solve again) removes them, at twice the cost.
//!
//! Jarosch, Schoof and Anslow ("Restoring mass conservation to shallow ice flow models over
//! complex terrain", The Cryosphere 7, 2013, doi:10.5194/tc-7-229-2013) show that standard SIA
//! schemes over steep beds let ice thickness go negative, so clamping it to zero creates mass;
//! they fix this with a (MUSCL superbee) flux limiter. Long semi-implicit steps make the
//! problem worse: the lagged D can move more ice out of a cell than it holds. So the fluxes the
//! solve implies are limited, cell by cell, to the ice each cell holds, and the thickness is
//! updated from the limited fluxes. That keeps thickness positive and conserves mass exactly
//! (`test_halfar_dome` checks the latter), but is cruder than their limiter: it slows ice that
//! a step would have moved too far, rather than resolving the flow more accurately.
//!
//! Ice reaching the sea or the edge of a patch is removed (calving, or leaving the region).
//!
//! ## Glacial erosion
//!
//! Abrasion and quarrying need ice sliding over the bed. Hallet ("A Theoretical Model of
//! Glacial Abrasion", J. Glaciology 23(89), 1979, doi:10.3189/S0022143000029725) predicts
//! abrasion proportional to sliding speed, and Humphrey and Raymond (J. Glaciology 40(136),
//! 1994, doi:10.3189/S0022143000012429) measured erosion of about 1e-4 times the sliding
//! distance at Variegated Glacier, Alaska. So
//!
//! ```text
//! ∂b/∂t = -K_g |u_s|^l,   l = 1, K_g = 1e-4
//! ```
//!
//! u_s is not evaluated from τ cell by cell: at an ice margin or over a step in the bed the
//! cell's slope is unrepresentative and τ³ makes such errors huge (kilometres of erosion per
//! step, in early experiments). Instead the speed comes from the (limited) ice flux through
//! the cell, |q| / H, of which the sliding share is A_s H^(n+1) / (2A/(n+2) H^(n+2) + A_s H^(n+1)).
//! So erosion only happens where ice actually moves.
//!
//! Herman et al. ("Erosion by an Alpine glacier", Science 350, 2015,
//! doi:10.1126/science.aab2386) found l ≈ 2 at Franz Josef Glacier; with l = 2, K_g changes
//! units and must be refitted.
//!
//! Not modelled: ice temperature (A and sliding are those of temperate ice everywhere); water
//! pressure in sliding; the bed's isostatic response to the ice's weight; debris.

use crate::terrain::domain::Domain;
use crate::terrain::simulation::ICE_FREE;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct GlacierParams {
    /// Glen's flow law rate factor A, Pa⁻ⁿ per year.
    pub glen_a: f64,
    /// Glen's exponent n (also used for sliding).
    pub glen_n: f64,
    /// Sliding coefficient A_s, m per year per Paⁿ.
    pub sliding: f64,
    /// Ice density, kg/m³.
    pub density: f64,
    pub gravity: f64,
    /// Erosion coefficient K_g (dimensionless for l = 1).
    pub erosion: f64,
    /// Erosion exponent l.
    pub erosion_exponent: f64,
    /// ELA at the equator and pole, metres.
    pub ela_equator: f64,
    pub ela_pole: f64,
    /// Swing of the ELA over a glacial cycle, metres.
    pub ela_amplitude: f64,
    /// Glacial cycle period, years.
    pub period: f64,
    /// Mass balance gradient β, per year.
    pub balance_gradient: f64,
    /// Maximum accumulation, metres of ice per year.
    pub max_accumulation: f64,
    /// Longest ice time step, years.
    pub max_step: f64,
    /// Shortest ice time step, years: where `max_stiffness` would ask for less (thick, fast
    /// ice), accuracy is traded for time, the flux limiter keeping the step safe.
    pub min_step: f64,
    /// Fastest ice, metres per year (see the module docs).
    pub max_speed: f64,
    /// Solves per ice step: 1 for the semi-implicit (lagged diffusivity) step, more for Picard
    /// iterations towards the fully implicit one.
    pub picard: usize,
    /// Largest ice step relative to the fastest diffusion: Δt Σc / area (see `flow_ice`).
    pub max_stiffness: f64,
    /// Morphological acceleration factor (see `simulation`): years of landscape evolution per
    /// year of ice flow.
    pub acceleration: f64,
}

impl Default for GlacierParams {
    fn default() -> GlacierParams {
        GlacierParams {
            glen_a: 2.4e-24 * 365.25 * 24. * 3600.,
            glen_n: 3.,
            sliding: 5e-15,
            density: 910.,
            gravity: 9.81,
            erosion: 1e-4,
            erosion_exponent: 1.,
            ela_equator: 5000.,
            ela_pole: -500.,
            ela_amplitude: 600.,
            period: 1e5,
            balance_gradient: 0.007,
            max_accumulation: 2.,
            max_step: 10.,
            min_step: 0.01,
            picard: 2,
            max_speed: 1000.,
            max_stiffness: 50.,
            acceleration: 10.,
        }
    }
}

impl GlacierParams {
    /// The equilibrium line altitude at latitude `lat` (radians) and time `t` (years).
    pub fn ela(&self, lat: f64, t: f64) -> f64 {
        let c = lat.cos();
        self.ela_pole + (self.ela_equator - self.ela_pole) * c * c
            + self.ela_amplitude * (std::f64::consts::TAU * t / self.period).cos()
    }

    pub fn mass_balance(&self, s: f64, ela: f64) -> f64 {
        (self.balance_gradient * (s - ela)).min(self.max_accumulation)
    }
}

/// Squared surface slope at each cell, from its edge-sharing neighbours.
pub fn slope_squared(domain: &Domain, s: &[f64]) -> Vec<f64> {
    (0..domain.len()).into_par_iter().map(|i| {
        let (mut sum, mut k) = (0., 0);
        for l in domain.links(i) {
            if l.width > 0. {
                let g = (s[l.to] - s[i]) / l.dist;
                sum += g * g;
                k += 1;
            }
        }
        if k > 0 { 2. * sum / k as f64 } else { 0. }
    }).collect()
}

/// What an ice step did, besides changing the thickness.
pub struct IceStep {
    /// Years of ice flow taken.
    pub dt: f64,
    /// Conjugate gradient iterations used.
    pub iterations: usize,
    /// Mean basal sliding speed over the step, metres per year.
    pub sliding: Vec<f64>,
}

/// Advance ice thickness `ice` over `bed` by up to `max_dt` years, with mass balance `balance`
/// (m/yr). Cells marked `drain` lose all their ice. The step is shortened so that no cell
/// exchanges more than `max_stiffness` times its own volume in a step (Δt Σc / area), which
/// keeps the linear system well conditioned and the lagged diffusivity accurate.
/// Conductance c_ij = w D / d for each edge-sharing link, with D from thickness `h` over `bed`,
/// in one flat array indexed like the domain's links (diagonal slots stay 0).
fn conductances(domain: &Domain, p: &GlacierParams, bed: &[f64], h: &[f64]) -> Vec<f64> {
    let n = domain.len();
    let s: Vec<f64> = (0..n).map(|i| bed[i] + h[i]).collect();
    let g2 = slope_squared(domain, &s);
    let rho_g_n = (p.density * p.gravity).powf(p.glen_n);
    let deform = 2. * p.glen_a / (p.glen_n + 2.);
    let mut conductance = vec![0.; domain.link_count()];
    for i in 0..n {
        let o = domain.link_offset(i);
        for (k, l) in domain.edges(i).iter().enumerate() {
            let he = 0.5 * (h[i] + h[l.to]);
            if he <= 0. { continue; }
            let g2e = 0.5 * (g2[i] + g2[l.to]);
            let d = rho_g_n * g2e.powf((p.glen_n - 1.) / 2.)
                * (deform * he.powf(p.glen_n + 2.) + p.sliding * he.powf(p.glen_n + 1.));
            // Mean speed is D |∇s| / H; cap it at max_speed.
            let d = d.min(p.max_speed * he / g2e.sqrt().max(1e-6));
            conductance[o + k] = l.width * d / l.dist;
        }
    }
    conductance
}

/// Solve the step's linear system for the new thickness (unclamped), given the conductances.
/// Returns it and the solver iterations used.
fn solve_ice(domain: &Domain, bed: &[f64], ice: &[f64], balance: &[f64], drain: &[bool], conductance: &[f64], dt: f64) -> (Vec<f64>, usize) {
    let n = domain.len();
    let edges = |i: usize| domain.edges(i).iter().zip(&conductance[domain.link_offset(i)..]);

    // The system A x = rhs. Drained cells are fixed at zero, so their rows are the identity and
    // other rows don't couple to them (keeping A symmetric).
    let diag: Vec<f64> = (0..n).map(|i| {
        if drain[i] { 1. } else { domain.area[i] / dt + edges(i).map(|(_, c)| c).sum::<f64>() }
    }).collect();
    let rhs: Vec<f64> = (0..n).map(|i| {
        if drain[i] { return 0.; }
        // Melt is applied after the flow (in `flow_ice`), so that ice arriving during the step
        // melts too.
        let m = balance[i].max(0.);
        let bed_term: f64 = edges(i).map(|(l, c)| c * (bed[l.to] - bed[i])).sum();
        domain.area[i] * (ice[i] / dt + m) + bed_term
    }).collect();
    let coupled = |i: usize, k: usize| !drain[i] && !drain[k];
    let apply = |x: &[f64], y: &mut [f64]| {
        for i in 0..n {
            if drain[i] { y[i] = x[i]; continue; }
            let off: f64 = edges(i).filter(|(l, _)| !drain[l.to]).map(|(l, c)| c * x[l.to]).sum();
            y[i] = diag[i] * x[i] - off;
        }
    };

    // Incomplete Cholesky, IC(0), as the preconditioner: the factorisation A ≈ L D⁻¹ Lᵀ with
    // L = D + (A's strictly lower part), keeping A's sparsity. Edge-sharing links on these grids
    // form no triangles (bar the cube's corners), so the only fill dropped is in the pivots:
    // d_i = a_ii - Σ_(k<i) a_ik² / d_k.
    let mut pivot = vec![1.; n];
    for i in 0..n {
        if drain[i] { continue; }
        let fill: f64 = edges(i).filter(|(l, _)| l.to < i && coupled(i, l.to)).map(|(l, c)| c * c / pivot[l.to]).sum();
        pivot[i] = (diag[i] - fill).max(1e-3 * diag[i]);
    }
    let precondition = |r: &[f64], z: &mut [f64]| {
        // Solve L u = r, then Lᵀ z = D u; off-diagonal entries of A are -c.
        for i in 0..n {
            let lower: f64 = edges(i).filter(|(l, _)| l.to < i && coupled(i, l.to)).map(|(l, c)| c * z[l.to]).sum();
            z[i] = (r[i] + lower) / pivot[i];
        }
        for i in (0..n).rev() {
            let upper: f64 = edges(i).filter(|(l, _)| l.to > i && coupled(i, l.to)).map(|(l, c)| c * z[l.to]).sum();
            z[i] += upper / pivot[i];
        }
    };

    let mut x: Vec<f64> = (0..n).map(|i| if drain[i] { 0. } else { ice[i] }).collect();
    let iterations = conjugate_gradient(&apply, &precondition, &rhs, &mut x, 1e-6, 500);
    (x, iterations)
}

pub fn flow_ice(domain: &Domain, p: &GlacierParams, bed: &[f64], ice: &mut [f64], balance: &[f64], drain: &[bool], max_dt: f64) -> IceStep {
    let n = domain.len();
    let deform = 2. * p.glen_a / (p.glen_n + 2.);

    let mut conductance = conductances(domain, p, bed, ice);
    let stiffness = (0..n).filter(|&i| !drain[i]).map(|i| {
        let o = domain.link_offset(i);
        conductance[o .. o + domain.edges(i).len()].iter().sum::<f64>() / domain.area[i]
    }).fold(0., f64::max);
    let dt = if stiffness > 0. { max_dt.min((p.max_stiffness / stiffness).max(p.min_step)) } else { max_dt };

    // Picard iteration: re-evaluate D and solve again. D comes from the mean of the old and
    // latest thickness (a midpoint rule): from the latest alone, a cell the first solve drains
    // completely would get no conductance, and the next solve would leave it unable to flow.
    let (mut x, mut iterations) = solve_ice(domain, bed, ice, balance, drain, &conductance, dt);
    for _ in 1..p.picard {
        let h: Vec<f64> = x.iter().zip(ice.iter()).map(|(&v, &h0)| 0.5 * (v.max(0.) + h0)).collect();
        conductance = conductances(domain, p, bed, &h);
        let (next, it) = solve_ice(domain, bed, ice, balance, drain, &conductance, dt);
        x = next;
        iterations += it;
    }
    let edges = |i: usize| domain.edges(i).iter().zip(&conductance[domain.link_offset(i)..]);

    // The solution x may be negative where the lagged diffusivity moved more ice than a cell
    // had. Rather than clamp it (creating ice), take the fluxes the solve implies and limit
    // each cell's outflow to the ice it holds, then update from the limited fluxes: positive
    // and conservative, at the cost of slowing ice that the long step would have moved too far.
    // Fluxes come from the surface with negative thickness clamped to zero: an over-drained
    // cell's surface is its bed, not a pit below it that its neighbours would pour ice into
    // (which, before the clamp, built single-cell spikes of ice kilometres thick).
    let s_new: Vec<f64> = (0..n).map(|i| bed[i] + x[i].max(0.)).collect();
    let supply: Vec<f64> = (0..n).map(|i| ice[i] + dt * balance[i].max(0.)).collect();
    let limit: Vec<f64> = (0..n).map(|i| {
        if drain[i] { return 0.; }
        let out: f64 = edges(i).map(|(l, c)| (c * dt * (s_new[i] - s_new[l.to])).max(0.)).sum();
        if out > 0. { (domain.area[i] * supply[i] / out).min(1.) } else { 1. }
    }).collect();
    let mut sliding = vec![0.; n];
    for i in 0..n {
        if drain[i] { ice[i] = 0.; continue; }
        let (mut net, mut through) = (0., 0.);
        for (l, c) in edges(i) {
            let q = c * dt * (s_new[i] - s_new[l.to]);
            let q = if q > 0. { q * limit[i] } else { q * limit[l.to] };
            net -= q;
            through += q.abs();
        }
        // Melt last, so that ice arriving during the step melts too.
        let h = (supply[i] + net / domain.area[i] + dt * balance[i].min(0.)).max(0.);
        // Speed from the volume passing through the cell: half of what crosses its edges,
        // over the cell's width and the ice's mean thickness. Of that, the share due to
        // sliding is A_s / (2A/(n+2) H + A_s) (from D above).
        let mean = 0.5 * (supply[i] + h);
        if mean >= ICE_FREE {
            let speed = 0.5 * through / (dt * domain.area[i].sqrt() * mean);
            sliding[i] = speed * p.sliding / (deform * mean + p.sliding);
        }
        ice[i] = h;
    }
    IceStep { dt, iterations, sliding }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// Solve A x = b for symmetric positive definite A (given as `apply`), with the preconditioner
/// `precondition` (z = M⁻¹ r), starting from `x`. Stops when the residual's norm falls to `tol`
/// times b's. Returns the iterations used.
fn conjugate_gradient(apply: &dyn Fn(&[f64], &mut [f64]), precondition: &dyn Fn(&[f64], &mut [f64]), b: &[f64], x: &mut [f64], tol: f64, max_iter: usize) -> usize {
    let n = b.len();
    let mut ax = vec![0.; n];
    apply(x, &mut ax);
    let mut r: Vec<f64> = (0..n).map(|i| b[i] - ax[i]).collect();
    let target = tol * dot(b, b).sqrt().max(1e-300);
    let mut z = vec![0.; n];
    precondition(&r, &mut z);
    let mut p = z.clone();
    let mut rz = dot(&r, &z);
    let mut ap = vec![0.; n];
    for it in 0..max_iter {
        if dot(&r, &r).sqrt() <= target { return it; }
        apply(&p, &mut ap);
        let alpha = rz / dot(&p, &ap);
        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        precondition(&r, &mut z);
        let rz_next = dot(&r, &z);
        let beta = rz_next / rz;
        rz = rz_next;
        for i in 0..n { p[i] = z[i] + beta * p[i]; }
    }
    max_iter
}

/// Lower the bed under sliding ice for `dt` years.
pub fn erode(p: &GlacierParams, bed: &mut [f64], sliding: &[f64], fixed: &[bool], dt: f64) {
    for i in 0..bed.len() {
        if !fixed[i] {
            bed[i] -= p.erosion * sliding[i].powf(p.erosion_exponent) * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn volume(d: &Domain, h: &[f64]) -> f64 { h.iter().zip(&d.area).map(|(a, b)| a * b).sum() }

    /// Halfar's similarity solution for an ice dome spreading over a flat bed with no mass
    /// balance (n = 3), as given by Bueler et al. ("Exact solutions and verification of
    /// numerical models for isothermal ice sheets", J. Glaciology 51(173), 2005):
    /// H(t, r) = H0 (t0/t)^(1/9) [1 - ((t0/t)^(1/18) r/R0)^(4/3)]^(3/7),
    /// t0 = (7/4)³ R0⁴ / (18 Γ H0⁷), Γ = 2A(ρg)³/5.
    #[test]
    fn test_halfar_dome() {
        let p = GlacierParams { sliding: 0., ..GlacierParams::default() };
        let (cells, dx) = (61, 25e3);
        let d = Domain::patch(0., 0., cells, dx, 1e12);
        let (h0, r0): (f64, f64) = (3000., 450e3);
        let gamma = 2. * p.glen_a * (p.density * p.gravity).powi(3) / 5.;
        let t0 = (7f64 / 4.).powi(3) * r0.powi(4) / (18. * gamma * h0.powi(7));
        let halfar = |t: f64, r: f64| {
            let k = t0 / t;
            let x = 1. - (k.powf(1. / 18.) * r / r0).powf(4. / 3.);
            if x > 0. { h0 * k.powf(1. / 9.) * x.powf(3. / 7.) } else { 0. }
        };
        let radius = |i: usize| {
            let (x, z) = ((i % cells) as f64 + 0.5 - cells as f64 / 2., (i / cells) as f64 + 0.5 - cells as f64 / 2.);
            (x * x + z * z).sqrt() * dx
        };
        let mut ice: Vec<f64> = (0..d.len()).map(|i| halfar(t0, radius(i))).collect();
        let bed = vec![0.; d.len()];
        let zero = vec![0.; d.len()];
        let before = volume(&d, &ice);
        let dt = 10.;
        let steps = (t0 / dt).round() as usize; // until t = 2 t0
        for _ in 0..steps { flow_ice(&d, &p, &bed, &mut ice, &zero, &d.boundary, dt); }
        let t = t0 + steps as f64 * dt;
        let centre = (cells / 2) * cells + cells / 2;
        let expected = halfar(t, radius(centre));
        assert!((ice[centre] - expected).abs() < 0.03 * expected, "centre {} vs {}", ice[centre], expected);
        // On a flat bed, ice thickness stays positive, so volume is conserved.
        assert!((volume(&d, &ice) - before).abs() < 1e-6 * before);
    }

    #[test]
    fn test_ice_grows_above_the_ela_only() {
        let p = GlacierParams::default();
        let d = Domain::patch(45., 0., 20, 500., 6.371e6);
        // A ramp rising 50 m per cell from 0 to 950 m, with the ELA at 600 m.
        let bed: Vec<f64> = (0..d.len()).map(|i| (i % 20) as f64 * 50.).collect();
        let mut ice = vec![0.; d.len()];
        let mut sliding = vec![];
        for _ in 0..500 {
            let balance: Vec<f64> = (0..d.len()).map(|i| p.mass_balance(bed[i] + ice[i], 600.)).collect();
            sliding = flow_ice(&d, &p, &bed, &mut ice, &balance, &d.boundary, 10.).sliding;
        }
        let row: Vec<f64> = (0..20).map(|i| ice[10 * 20 + i]).collect();
        // Thick above the ELA, ice-free well below it.
        assert!(row[16] > 50., "{:?}", row);
        assert!(row[..2].iter().all(|&h| h == 0.), "{:?}", row);
        // The glacier flows below the ELA (cell 12) before melting away.
        assert!(row[11] > 0., "{:?}", row);
        assert!(sliding[10 * 20 + 14] > 1., "{:?}", sliding);
        assert_eq!(sliding[10 * 20 + 1], 0.);
    }

    #[test]
    fn test_ela_falls_poleward_and_in_glacials() {
        let p = GlacierParams::default();
        assert!(p.ela(0.3, 0.) > p.ela(1.0, 0.));
        assert!(p.ela(0.7, p.period / 2.) < p.ela(0.7, 0.) - 1000.);
        assert_eq!(p.mass_balance(1e5, 0.), p.max_accumulation);
    }

    #[test]
    fn test_erosion_follows_sliding() {
        let p = GlacierParams::default();
        let mut bed = vec![100.; 3];
        erode(&p, &mut bed, &[0., 100., 100.], &[false, false, true], 1000.);
        assert_eq!(bed, vec![100., 100. - 1e-4 * 100. * 1000., 100.]);
    }
}

#[cfg(test)]
mod ice_cap_tests {
    use super::*;

    /// Uniform snowfall on a flat plateau, draining at its edges, builds an ice cap that stops
    /// thickening once flow to the edges balances accumulation.
    #[test]
    fn test_ice_cap_reaches_steady_state() {
        let p = GlacierParams::default();
        let d = Domain::patch(45., 0., 24, 500., 6.371e6);
        let bed = vec![3000.; d.len()];
        let mut ice = vec![0.; d.len()];
        let balance = vec![2.; d.len()];
        let centre = 12 * 24 + 12;
        let mut history = vec![];
        for _ in 0..30 {
            let mut t = 0.;
            while t < 100. { t += flow_ice(&d, &p, &bed, &mut ice, &balance, &d.boundary, 100. - t).dt; }
            history.push(ice[centre]);
        }
        let (late, last) = (history[24], history[29]);
        assert!(last < 2000., "{:?}", history);
        assert!((last - late).abs() < 0.02 * last, "{:?}", history);
        // Domed: thickest in the middle.
        assert!(ice[12 * 24 + 2] < ice[centre]);
    }
}
