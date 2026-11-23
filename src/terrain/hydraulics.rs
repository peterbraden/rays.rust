//! # River hydraulics: how much water, how deep, how fast
//!
//! The landscape's simulation routes water to find where rivers cut (`flow.rs`, `fluvial.rs`),
//! but it works in drainage area, not in water: it never says how much flows, how deep or how
//! fast. Those decide what a river looks like (glassy or white, a trickle over boulders or a
//! broad sheet), so this module computes them from what the simulation and the climate know,
//! with the relations hydrologists and river engineers use. It is step 1 of the water
//! workstream (plan.md section 15); the surfaces, whitewater and falls read it.
//!
//! ## Runoff: Budyko's curve
//!
//! Over years, rain on a catchment either evaporates (from the ground and through plants) or
//! runs off. Budyko observed that the split is set mostly by the aridity index φ = PET/P, the
//! potential evaporation (what the sun's energy could evaporate) over the rain: where φ is
//! small, evaporation is limited by energy and nearly all the rain it can't take runs off;
//! where φ is large, by water, and nearly all the rain evaporates. Fu's one-parameter form of
//! the curve (Fu 1981; as given by Zhang et al. 2004) is
//!
//! ```text
//! E/P = 1 + φ - (1 + φ^w)^(1/w)
//! ```
//!
//! which tends to E = PET as φ → 0 and to E = P as φ → ∞, never exceeding either. w ≈ 2.6 is the
//! value that best reproduces Budyko's own curve (as Zhang et al. give it; a default, not a fit
//! to catchments, which scatter about it); larger w (deep soils, forests) evaporates more. Runoff R = P - E
//! (`runoff`).
//!
//! ## Potential evaporation: Thornthwaite
//!
//! The climate model gives each place its annual mean temperature, its warmest and coldest
//! months and its rain (`climate::Local`), but not its radiation budget, so the potential
//! evaporation is Thornthwaite's (1948), which needs only monthly temperatures and day length:
//!
//! ```text
//! PET_m = 16 (L_m / 12) (N_m / 30) (10 T_m / I)^a   mm in month m, for T_m > 0 °C
//! I = Σ_m (T_m / 5)^1.514
//! a = 6.75·10⁻⁷ I³ - 7.71·10⁻⁵ I² + 1.792·10⁻² I + 0.49239
//! ```
//!
//! with T_m the month's mean temperature, L_m its mean day length in hours and N_m its days.
//! The months' temperatures are a cosine between the coldest and warmest (`monthly`), warmest
//! in July in the north and January in the south; day lengths are astronomical, from the
//! latitude and the sun's declination (`day_length`). Thornthwaite's formula is known to
//! underestimate in arid climates and is empirical above 26.5 °C, where he used a table; here
//! the power law is kept throughout (to check if hot deserts' rivers matter).
//!
//! ## Discharge
//!
//! A cell's mean discharge is the runoff over everything that drains through it:
//! Q = Σ R_j A_j over the cell and its donors (`discharge`), accumulated down the receiver
//! tree in order of drainage area (a donor always drains less area than its receiver). The
//! scene's `flow` multiplies it: 1 the mean annual flow, more for a flood, less for a dry
//! season.
//!
//! ## Depth and speed: Manning and Strickler
//!
//! In steady uniform flow the weight of the water down the slope balances the drag of the bed,
//! which Manning's formula expresses (Manning 1891; Chow 1959, ch. 5):
//!
//! ```text
//! V = R_h^(2/3) S^(1/2) / n        Q = V A_x
//! ```
//!
//! V the mean velocity (m/s), R_h = A_x / P_w the hydraulic radius (area over wetted
//! perimeter), S the slope and n Manning's roughness (s/m^(1/3)). For a rectangular channel of
//! width w and depth d, A_x = wd and P_w = w + 2d, and Q increases with d, so `normal_depth`
//! finds d by bisection. Wide channels have R_h ≈ d and d ≈ (Qn / (w √S))^(3/5).
//!
//! The roughness is the bed's grains: Strickler's n = d₅₀^(1/6) / 21.1, d₅₀ the median grain in
//! metres (`strickler`; Henderson's often-quoted 0.034 d^(1/6) with d in feet is d^(1/6) / 24.1
//! in metres, 15% smoother: the constant is worth checking against Strickler's paper). The grain size is what the river can just move when full: gravel-bed
//! rivers adjust so their bed is near the threshold of motion at bankfull flow, Shields's
//! number τ* = ρ g H S / ((ρ_s - ρ) g D) near a constant there (Parker et al. 2007), so
//! D = H S / (Δ τ*), Δ = 1.65 for quartz (`grain_size`). Parker (1978) put the bankfull
//! Shields number at about 1.2 times the critical; observed ratios run from 1 to 2, and Li et
//! al. (2015) find it rising with slope, so this is the rule's weakest constant. Grains are at
//! most boulders. Sand-bed rivers (dune-ripple reaches, the gentlest) aren't threshold
//! channels: their bed moves at most flows, so they are given sand rather than the rule's
//! (much coarser) grain.
//!
//! ## The channel's shape: a parabola
//!
//! A channel isn't a rectangle: at low flow the water gathers in its middle. Its cross-section
//! is taken as a parabola, its bankfull width B (the simulation's, c√A) and depth H, the bed
//! at height H (2x/B)² across it. Filled to depth d at the centre, the water is
//! T = B √(d/H) wide, its area A_x = (2/3) T d, its wetted perimeter about T + 8d²/(3T)
//! (`parabola_discharge`). In a wide parabola Q ∝ A_x R_h^(2/3) ∝ d^(13/6), so as the flow
//! rises at a place the depth grows as Q^0.46, the width as Q^0.23 and the speed as Q^0.31:
//! close to the exponents Leopold and Maddock measured at gauging stations (f = 0.40,
//! b = 0.26, m = 0.34), which a rectangle (width fixed, depth as Q^0.6) misses.
//!
//! The channel is full at its bankfull discharge, here `bankfull_ratio` times the mean flow,
//! and H is the depth at which the parabola carries it (with the grain size, and so the
//! roughness, found from H in turn: a few rounds settle both). The ratio is a model choice,
//! 10 by default: bankfull is roughly the flood of every year or two, which is several times to
//! tens of times the mean flow, more where snowmelt or storms make the floods, less where
//! groundwater evens the flow (a source for typical values is still to find).
//!
//! ## What the flow is like
//!
//! The Froude number Fr = V / √(g A_x / T) compares the flow's speed with that of waves on it.
//! Below 1 (subcritical, tranquil) waves travel upstream and the surface is smooth; above 1
//! (supercritical, rapid) nothing reaches upstream, the surface stands in waves, and where it
//! slows back below 1 it jumps. The friction velocity u* = √(g R_h S) scales the turbulence,
//! and the stream power per unit width ω = ρ g Q S / w its energy. And Montgomery and
//! Buffington (1997) sorted mountain channels into reach types that follow mostly from slope
//! (`ReachKind`): cascades, step-pools, plane beds, pool-riffles and dune-ripple sand beds, and
//! bedrock where there is no sediment to form any of them. The slope bounds are Buffington's
//! as Wohl and Merritt (2005) quote them (cascade above 0.065, step-pool 0.03 to 0.065,
//! plane-bed 0.015 to 0.03, pool-riffle below), with dune-ripple below 0.001; the 1997 paper's
//! Table 1 gives overlapping ranges (still to check), and they vary by region.
//!
//! ## References
//!
//! - M. I. Budyko, "Climate and Life", Academic Press, 1974.
//! - B. P. Fu, "On the calculation of the evaporation from land surface", Scientia
//!   Atmospherica Sinica 5(1), 1981 (in Chinese).
//! - L. Zhang, K. Hickel, W. R. Dawes, F. H. S. Chiew, A. W. Western, P. R. Briggs, "A rational
//!   function approach for estimating mean annual evapotranspiration", Water Resources
//!   Research 40, W02502, 2004, doi:10.1029/2003WR002710.
//! - C. W. Thornthwaite, "An approach toward a rational classification of climate",
//!   Geographical Review 38(1), 1948, doi:10.2307/210739.
//! - R. Manning, "On the flow of water in open channels and pipes", Transactions of the
//!   Institution of Civil Engineers of Ireland 20, 1891; V. T. Chow, "Open-Channel
//!   Hydraulics", McGraw-Hill, 1959.
//! - A. Strickler, "Beiträge zur Frage der Geschwindigkeitsformel und der Rauhigkeitszahlen
//!   für Ströme, Kanäle und geschlossene Leitungen", Mitteilungen des Eidgenössischen Amtes
//!   für Wasserwirtschaft 16, 1923.
//! - G. Parker, "Self-formed straight rivers with equilibrium banks and mobile bed. Part 2.
//!   The gravel river", J. Fluid Mechanics 89, 1978, doi:10.1017/S0022112078002499; C. Li,
//!   M. J. Czapiga, E. C. Eke, E. Viparelli, G. Parker, "Variable Shields number model for
//!   river bankfull geometry", J. Hydraulic Research 53(1), 2015,
//!   doi:10.1080/00221686.2014.939113.
//! - E. Wohl, D. M. Merritt, "Prediction of mountain stream morphology", Water Resources
//!   Research 41, W08419, 2005 (bibliographic details unverified).
//! - G. Parker, P. R. Wilcock, C. Paola, W. E. Dietrich, J. Pitlick, "Physical basis for
//!   quasi-universal relations describing bankfull hydraulic geometry of single-thread gravel
//!   bed rivers", JGR Earth Surface 112, F04005, 2007, doi:10.1029/2006JF000549.
//! - L. B. Leopold, T. Maddock, "The hydraulic geometry of stream channels and some
//!   physiographic implications", USGS Professional Paper 252, 1953, doi:10.3133/pp252.
//! - D. R. Montgomery, J. M. Buffington, "Channel-reach morphology in mountain drainage
//!   basins", GSA Bulletin 109(5), 1997,
//!   doi:10.1130/0016-7606(1997)109<0596:CRMIMD>2.3.CO;2.

use crate::terrain::channel::ChannelParams;
use crate::terrain::climate::{Climate, Local};
use crate::terrain::simulation::Terrain;
use std::f64::consts::{PI, TAU};

/// Gravity, m/s².
pub const G: f64 = 9.81;
/// Density of water, kg/m³.
pub const RHO: f64 = 1000.;
/// Submerged specific gravity of quartz grains, (ρ_s - ρ) / ρ.
const DELTA: f64 = 1.65;
/// Shields's number of the median grain at bankfull flow in gravel-bed rivers (Parker et al.
/// 2007 find a narrow range about this).
const BANKFULL_SHIELDS: f64 = 0.0489;
/// The finest and coarsest median grain, metres: sand, and boulders.
const SAND: f64 = 0.0005;
const BOULDERS: f64 = 2.;
/// Slopes gentler than this count as this steep (flats and lakes, where the routing's slope is
/// nothing or negative).
pub const MIN_SLOPE: f64 = 1e-5;
const SECONDS_PER_YEAR: f64 = 365.25 * 86400.;

/// Options: the scene's `"hydraulics"` on a landscape.
#[derive(Clone, Debug)]
pub struct HydraulicParams {
    /// The discharge shown, as a multiple of the mean annual flow.
    pub flow: f64,
    /// Bankfull discharge, as a multiple of the mean annual flow.
    pub bankfull_ratio: f64,
    /// Fu's parameter w in Budyko's curve.
    pub budyko_w: f64,
    /// Without a climate model: the rain (mm/yr), and the annual mean temperature and the
    /// seasons' half range (°C), everywhere.
    pub precipitation: f64,
    pub temperature: f64,
    pub season: f64,
}

impl Default for HydraulicParams {
    fn default() -> Self {
        HydraulicParams { flow: 1., bankfull_ratio: 10., budyko_w: 2.6, precipitation: 1000., temperature: 10., season: 8. }
    }
}

/// Montgomery and Buffington's reach types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReachKind {
    /// No sediment to make a bed of: the river runs on rock.
    Bedrock,
    /// Boulders tumbling in a steep, disorganised jumble, the water spilling over and round
    /// them in jets and falls.
    Cascade,
    /// Channel-spanning steps of boulders, each a small fall into a pool.
    StepPool,
    /// A featureless bed of cobbles: long glides and riffles.
    PlaneBed,
    /// Alternating deep pools and shallow riffles over gravel bars, the channel winding.
    PoolRiffle,
    /// Sand beds moulded into ripples and dunes: lowland rivers.
    DuneRipple,
}

impl ReachKind {
    /// The usual type of an alluvial reach of slope `s` (Montgomery and Buffington 1997, whose
    /// slope ranges overlap; here split where they meet).
    pub fn by_slope(s: f64) -> ReachKind {
        if s > 0.065 { ReachKind::Cascade }
        else if s > 0.03 { ReachKind::StepPool }
        else if s > 0.015 { ReachKind::PlaneBed }
        else if s > 0.001 { ReachKind::PoolRiffle }
        else { ReachKind::DuneRipple }
    }
}

/// The flow at a point of a river.
#[derive(Clone, Copy, Debug)]
pub struct Reach {
    /// m³/s.
    pub discharge: f64,
    /// The channel's width (bankfull), metres.
    pub width: f64,
    /// The water's width, metres.
    pub water_width: f64,
    /// The water's mean depth (its area over its width), metres: two thirds of its depth at
    /// the centre.
    pub depth: f64,
    /// The channel's depth at its centre when full, metres: deeper water spills over its banks.
    pub bankfull_depth: f64,
    /// Mean velocity, m/s.
    pub speed: f64,
    /// Bed slope.
    pub slope: f64,
    /// Median grain size of the bed, metres.
    pub grain: f64,
    /// Manning's n, s/m^(1/3).
    pub roughness: f64,
    /// Froude number.
    pub froude: f64,
    /// Friction velocity, m/s.
    pub shear_velocity: f64,
    /// Stream power per unit of bed area, W/m².
    pub stream_power: f64,
    pub kind: ReachKind,
}

/// A month's mean day length in hours at latitude `lat` (radians); `month` 0 is January. The
/// sun's declination by Cooper's formula at mid-month, the sunrise hour angle
/// cos ω = -tan φ tan δ, clamped for polar day and night.
pub fn day_length(lat: f64, month: usize) -> f64 {
    let day = 15.2 + 30.44 * month as f64;
    let declination = 23.44f64.to_radians() * (TAU * (284. + day) / 365.).sin();
    let c = (-lat.tan() * declination.tan()).clamp(-1., 1.);
    24. * c.acos() / PI
}

/// The twelve months' mean temperatures (°C) of climate `c` at latitude `lat`: a cosine about
/// the annual mean, warmest in July (north) or January (south).
pub fn monthly(c: &Local, lat: f64) -> [f64; 12] {
    let half = (c.summer - c.winter) / 2.;
    let peak = if lat >= 0. { 6. } else { 0. };
    std::array::from_fn(|m| c.mean + half * (TAU * (m as f64 - peak) / 12.).cos())
}

/// Thornthwaite's potential evapotranspiration, mm/yr.
pub fn thornthwaite(c: &Local, lat: f64) -> f64 {
    let t = monthly(c, lat);
    let heat: f64 = t.iter().map(|&tm| if tm > 0. { (tm / 5.).powf(1.514) } else { 0. }).sum();
    if heat <= 0. { return 0.; }
    let a = 6.75e-7 * heat.powi(3) - 7.71e-5 * heat.powi(2) + 1.792e-2 * heat + 0.49239;
    let days = 365.25 / 12.;
    (0..12).filter(|&m| t[m] > 0.)
        .map(|m| 16. * (day_length(lat, m) / 12.) * (days / 30.) * (10. * t[m] / heat).powf(a))
        .sum()
}

/// Mean evaporation (mm/yr) from rain `p` and potential evaporation `pet` (mm/yr) by Fu's form
/// of Budyko's curve with parameter `w`.
pub fn evaporation(p: f64, pet: f64, w: f64) -> f64 {
    if p <= 0. || pet <= 0. { return 0.; }
    let phi = pet / p;
    (p * (1. + phi - (1. + phi.powf(w)).powf(1. / w))).clamp(0., p.min(pet))
}

/// Mean runoff, mm/yr: the rain that doesn't evaporate.
pub fn runoff(c: &Local, lat: f64, w: f64) -> f64 {
    (c.precipitation - evaporation(c.precipitation, thornthwaite(c, lat), w)).max(0.)
}

/// Strickler's Manning roughness for median grain `d50` metres.
pub fn strickler(d50: f64) -> f64 { d50.powf(1. / 6.) / 21.1 }

/// The discharge (m³/s) of uniform flow `depth` deep in a rectangular channel.
pub fn manning_discharge(depth: f64, width: f64, slope: f64, n: f64) -> f64 {
    let area = width * depth;
    let radius = area / (width + 2. * depth);
    area * radius.powf(2. / 3.) * slope.sqrt() / n
}

/// The normal depth (m) of discharge `q` in a rectangular channel `width` wide.
pub fn normal_depth(q: f64, width: f64, slope: f64, n: f64) -> f64 {
    if q <= 0. { return 0.; }
    let (mut lo, mut hi) = (1e-6f64.ln(), 1e4f64.ln());
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if manning_discharge(mid.exp(), width, slope, n) < q { lo = mid } else { hi = mid }
    }
    (0.5 * (lo + hi)).exp()
}

/// The bed's median grain (m) of a threshold channel `bankfull_depth` deep on `slope`.
pub fn grain_size(bankfull_depth: f64, slope: f64) -> f64 {
    (bankfull_depth * slope / (DELTA * BANKFULL_SHIELDS)).clamp(SAND, BOULDERS)
}

/// Area, top width and wetted perimeter of water `d` deep at the centre of a parabolic channel
/// `width` wide and `bankfull` deep.
fn parabola(d: f64, width: f64, bankfull: f64) -> (f64, f64, f64) {
    let top = width * (d / bankfull).sqrt();
    (2. / 3. * top * d, top, top + 8. * d * d / (3. * top.max(1e-12)))
}

/// The discharge (m³/s) of uniform flow `d` deep at the centre of a parabolic channel.
pub fn parabola_discharge(d: f64, width: f64, bankfull: f64, slope: f64, n: f64) -> f64 {
    let (area, _, perimeter) = parabola(d, width, bankfull);
    area * (area / perimeter).powf(2. / 3.) * slope.sqrt() / n
}

/// The depth at the centre (m) of discharge `q` in a parabolic channel.
pub fn parabola_depth(q: f64, width: f64, bankfull: f64, slope: f64, n: f64) -> f64 {
    if q <= 0. { return 0.; }
    let (mut lo, mut hi) = (1e-6f64.ln(), 1e4f64.ln());
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if parabola_discharge(mid.exp(), width, bankfull, slope, n) < q { lo = mid } else { hi = mid }
    }
    (0.5 * (lo + hi)).exp()
}

/// The depth (m) of a parabolic channel `width` wide that carries `q` when full.
pub fn full_depth(q: f64, width: f64, slope: f64, n: f64) -> f64 {
    let (mut lo, mut hi) = (1e-6f64.ln(), 1e4f64.ln());
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        let h = mid.exp();
        if parabola_discharge(h, width, h, slope, n) < q { lo = mid } else { hi = mid }
    }
    (0.5 * (lo + hi)).exp()
}

/// The flow of mean discharge `mean` (m³/s), at `p`'s multiple of it, down a parabolic channel
/// `width` wide on `slope`, its bed of sediment or (`bedrock`) bare rock.
pub fn reach(mean: f64, width: f64, slope: f64, bedrock: bool, p: &HydraulicParams) -> Reach {
    let slope = slope.max(MIN_SLOPE);
    let width = width.max(1e-3);
    let kind = if bedrock { ReachKind::Bedrock } else { ReachKind::by_slope(slope) };
    // The bankfull depth and the bed's grains, each depending on the other.
    let bankfull_flow = (p.bankfull_ratio * mean).max(1e-9);
    let mut bankfull_depth = width / 12.;
    let mut grain = SAND;
    for _ in 0..4 {
        grain = if kind == ReachKind::DuneRipple { SAND } else { grain_size(bankfull_depth, slope) };
        bankfull_depth = full_depth(bankfull_flow, width, slope, strickler(grain)).max(1e-3);
    }
    let roughness = strickler(grain);
    let discharge = p.flow * mean;
    let centre = parabola_depth(discharge, width, bankfull_depth, slope, roughness);
    let (area, water_width, perimeter) = parabola(centre, width, bankfull_depth);
    let depth = if water_width > 0. { area / water_width } else { 0. };
    let speed = if area > 0. { discharge / area } else { 0. };
    Reach {
        discharge, width, water_width, depth, bankfull_depth, speed, slope, grain, roughness,
        froude: if depth > 0. { speed / (G * depth).sqrt() } else { 0. },
        shear_velocity: (G * area / perimeter.max(1e-12) * slope).sqrt(),
        stream_power: RHO * G * discharge * slope / water_width.max(1e-3),
        kind,
    }
}

/// A landscape's rivers: runoff and the flow at every cell.
pub struct Hydraulics {
    /// Mean runoff from each cell, mm/yr.
    pub runoff: Vec<f64>,
    /// The flow through each cell, at the scene's `flow`.
    pub reaches: Vec<Reach>,
    /// Mean discharge through each cell, m³/s.
    pub mean: Vec<f64>,
}

/// Mean discharge (m³/s) through each cell, from each cell's runoff (mm/yr), accumulated down
/// `receiver` (cells draining to themselves are outlets) in order of `drainage` area.
pub fn discharge(area: &[f64], runoff: &[f64], receiver: &[usize], drainage: &[f64]) -> Vec<f64> {
    let mut q: Vec<f64> = (0..area.len()).map(|i| runoff[i] * 1e-3 / SECONDS_PER_YEAR * area[i]).collect();
    let mut order: Vec<usize> = (0..area.len()).collect();
    order.sort_by(|&a, &b| drainage[a].total_cmp(&drainage[b]));
    for i in order {
        let r = receiver[i];
        if r != i { q[r] += q[i]; }
    }
    q
}

/// Sediment thinner than this (metres) leaves the bed bare rock.
const BARE: f64 = 0.1;

impl Hydraulics {
    /// The rivers of `t`, its rain from `climate` if given (else `p`'s).
    pub fn new(t: &Terrain, climate: Option<&Climate>, p: &HydraulicParams) -> Hydraulics {
        let d = &t.domain;
        let n = d.len();
        let fixed = Local { mean: p.temperature, summer: p.temperature + p.season, winter: p.temperature - p.season, precipitation: p.precipitation };
        let runoff: Vec<f64> = (0..n).map(|i| {
            if t.sea[i] && t.bed[i] < 0. { return 0.; }
            let local = climate.map_or(fixed, |c| c.local(&d.dirs[i], t.bed[i]));
            runoff(&local, t.latitude[i], p.budyko_w)
        }).collect();
        let q = discharge(&d.area, &runoff, &t.receiver, &t.drainage);
        let c = t.params.channels.as_ref().map_or(ChannelParams::default().width_coefficient, |c| c.width_coefficient);
        let reaches = (0..n).map(|i| {
            let r = t.receiver[i];
            let slope = if r == i { MIN_SLOPE } else {
                let dist = d.links(i).iter().find(|l| l.to == r).map_or_else(
                    || d.dirs[i].angle(&d.dirs[r]) * d.radius, |l| l.dist);
                (t.bed[i] - t.bed[r]) / dist.max(1e-9)
            };
            reach(q[i], c * t.drainage[i].sqrt(), slope, t.sediment[i] < BARE, p)
        }).collect();
        Hydraulics { runoff, reaches, mean: q }
    }

    /// As `new`, with the terrain's own climate model if it has one.
    pub fn of(t: &Terrain, p: &HydraulicParams) -> Hydraulics {
        let climate = t.params.climate.as_ref().map(|c| crate::terrain::climate::climate(c, &t.planet));
        Hydraulics::new(t, climate.as_deref(), p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temperate() -> Local { Local { mean: 10., summer: 18., winter: 2., precipitation: 800. } }

    #[test]
    fn test_manning_wide_channel() {
        // In a channel a thousand times wider than deep, R_h ≈ d and d = (Qn / (w√S))^(3/5).
        let (q, w, s, n) = (500., 1000., 1e-4, 0.03);
        let d = normal_depth(q, w, s, n);
        let wide = (q * n / (w * s.sqrt())).powf(0.6);
        assert!((d - wide).abs() / wide < 0.01, "{} vs {}", d, wide);
        // Exact in its own terms: the depth found carries the discharge.
        for &(q, w) in &[(0.01, 0.5), (3., 4.), (5000., 300.)] {
            let d = normal_depth(q, w, 0.01, 0.04);
            assert!((manning_discharge(d, w, 0.01, 0.04) - q).abs() / q < 1e-9);
        }
        assert_eq!(normal_depth(0., 10., 0.01, 0.03), 0.);
    }

    #[test]
    fn test_budyko_limits() {
        // Energy-limited: evaporation reaches the potential; water-limited: the rain.
        assert!((evaporation(10000., 500., 2.6) - 500.).abs() < 5.);
        assert!((evaporation(100., 5000., 2.6) - 100.).abs() < 1.);
        for &(p, pet) in &[(800., 600.), (300., 1200.), (2000., 400.)] {
            let e = evaporation(p, pet, 2.6);
            assert!(e > 0. && e <= p.min(pet), "{} {} {}", p, pet, e);
            // Larger w evaporates more.
            assert!(evaporation(p, pet, 3.5) > e);
        }
        assert_eq!(evaporation(0., 500., 2.6), 0.);
    }

    #[test]
    fn test_thornthwaite() {
        // A temperate mid-latitude climate evaporates a few hundred millimetres a year; a
        // frozen one nothing; a warm one more, as does a longer summer day.
        let pet = thornthwaite(&temperate(), 45f64.to_radians());
        assert!(pet > 500. && pet < 800., "{}", pet);
        let frozen = Local { mean: -20., summer: -5., winter: -35., precipitation: 200. };
        assert_eq!(thornthwaite(&frozen, 70f64.to_radians()), 0.);
        let warm = Local { mean: 22., summer: 26., winter: 18., ..temperate() };
        assert!(thornthwaite(&warm, 20f64.to_radians()) > 1.5 * pet);
        // The hemispheres mirror each other.
        let south = thornthwaite(&temperate(), -45f64.to_radians());
        assert!((south - pet).abs() / pet < 0.02, "{} vs {}", south, pet);
    }

    #[test]
    fn test_day_length() {
        assert!((day_length(0., 2) - 12.).abs() < 0.1);
        assert!(day_length(50f64.to_radians(), 5) > 16.);
        assert!(day_length(50f64.to_radians(), 11) < 8.5);
        assert_eq!(day_length(80f64.to_radians(), 5), 24.);
        assert_eq!(day_length(80f64.to_radians(), 11), 0.);
    }

    #[test]
    fn test_discharge_conserves_water() {
        // A chain 0 → 1 → 2 → 3 (an outlet), with a side branch 4 → 2.
        let area = [1e6; 5];
        let runoff = [300., 300., 300., 300., 600.];
        let receiver = [1, 2, 3, 3, 2];
        let drainage = [1e6, 2e6, 4e6, 5e6, 1e6];
        let q = discharge(&area, &runoff, &receiver, &drainage);
        let per = |mm: f64| mm * 1e-3 / SECONDS_PER_YEAR * 1e6;
        assert!((q[3] - (4. * per(300.) + per(600.))).abs() < 1e-12);
        assert!((q[2] - (3. * per(300.) + per(600.))).abs() < 1e-12);
        // 300 mm/yr over a square kilometre is about a hundredth of a cubic metre a second.
        assert!((per(300.) - 0.0095).abs() < 0.0002);
    }

    /// The exponent of `f` against discharge, by least squares over `reaches`.
    fn exponent(reaches: &[Reach], f: &dyn Fn(&Reach) -> f64) -> f64 {
        let pts: Vec<(f64, f64)> = reaches.iter().map(|r| (r.discharge.ln(), f(r).ln())).collect();
        let m = pts.len() as f64;
        let (mx, my) = pts.iter().fold((0., 0.), |(a, b), (x, y)| (a + x / m, b + y / m));
        let cov: f64 = pts.iter().map(|(x, y)| (x - mx) * (y - my)).sum();
        let var: f64 = pts.iter().map(|(x, _)| (x - mx).powi(2)).sum();
        cov / var
    }

    #[test]
    fn test_at_a_station_hydraulic_geometry() {
        // One channel, the flow rising from a tenth of its mean to bankfull: width, depth and
        // speed grow about as Leopold and Maddock measured at gauging stations (0.26, 0.40,
        // 0.34); the parabola's own exponents are 0.23, 0.46 and 0.31.
        let reaches: Vec<Reach> = (0..30).map(|k| {
            let p = HydraulicParams { flow: 0.1 * 100f64.powf(k as f64 / 29.), ..Default::default() };
            reach(20., 40., 0.002, false, &p)
        }).collect();
        let (b, f, m) = (exponent(&reaches, &|r| r.water_width), exponent(&reaches, &|r| r.depth), exponent(&reaches, &|r| r.speed));
        assert!((b - 0.23).abs() < 0.03 && (f - 0.46).abs() < 0.04 && (m - 0.31).abs() < 0.04, "{} {} {}", b, f, m);
        assert!((b + f + m - 1.).abs() < 1e-6);
        // At bankfull, the water fills the channel.
        let full = reach(20., 40., 0.002, false, &HydraulicParams { flow: 10., ..Default::default() });
        assert!((full.water_width - 40.).abs() < 0.1 && (1.5 * full.depth - full.bankfull_depth).abs() < 0.01, "{:?}", full);
    }

    #[test]
    fn test_downstream_hydraulic_geometry() {
        // Down a river in steady state, slope falls as A^-0.45 (the concavity of stream power)
        // and the channel widens as A^0.5; at the mean flow, depth and speed should then grow
        // with discharge roughly as Leopold and Maddock found downstream, d ∝ Q^0.4, V ∝ Q^0.1.
        let reaches: Vec<Reach> = (0..30).map(|k| {
            let a = 1e7 * 10f64.powf(k as f64 / 10.);
            reach(1e-8 * a, 0.003 * a.sqrt(), 0.05 * (a / 1e7).powf(-0.45), false, &HydraulicParams::default())
        }).collect();
        let f = exponent(&reaches, &|r| r.depth);
        let m = exponent(&reaches, &|r| r.speed);
        assert!(f > 0.3 && f < 0.55, "depth exponent {}", f);
        assert!(m > -0.05 && m < 0.25, "speed exponent {}", m);
    }

    #[test]
    fn test_mountain_torrents_and_lowland_rivers() {
        // A steep mountain stream is coarse-bedded and shallow; a big lowland river is sandy,
        // metres deep and tranquil; both fill part of their channels at the mean flow.
        let p = HydraulicParams::default();
        let torrent = reach(1., 30., 0.05, false, &p);
        let lowland = reach(95., 300., 1e-4, false, &p);
        assert_eq!(torrent.kind, ReachKind::StepPool);
        assert_eq!(lowland.kind, ReachKind::DuneRipple);
        assert!(torrent.grain > 0.05 && lowland.grain < 0.01, "{} {}", torrent.grain, lowland.grain);
        assert!(torrent.depth > 0.05 && torrent.depth < 0.5, "{:?}", torrent);
        assert!(lowland.depth > 0.5 && lowland.depth < 5., "{:?}", lowland);
        assert!(lowland.froude < 0.5 && torrent.froude > lowland.froude, "{} {}", lowland.froude, torrent.froude);
        for r in [&torrent, &lowland] {
            assert!(r.water_width < r.width && 1.5 * r.depth < r.bankfull_depth);
        }
        assert_eq!(reach(1., 30., 0.05, true, &p).kind, ReachKind::Bedrock);
    }

    #[test]
    fn test_a_landscapes_rivers() {
        use crate::terrain::domain::Domain;
        use crate::terrain::simulation::TerrainParams;
        let mut p = TerrainParams::default();
        p.planet.seed = 5;
        p.step = 500.;
        let mut t = Terrain::new(Domain::patch(40., 10., 24, 500., 6.371e6), p);
        t.run_to(2000.);
        let h = Hydraulics::new(&t, None, &HydraulicParams::default());
        let biggest = (0..t.domain.len()).max_by(|&a, &b| t.drainage[a].total_cmp(&t.drainage[b])).unwrap();
        let r = h.reaches[biggest];
        // Runoff is a few hundred millimetres of the metre of rain; the biggest river carries
        // about that over its catchment.
        let ro = h.runoff[biggest];
        assert!(ro > 200. && ro < 800., "{}", ro);
        let expected = ro * 1e-3 / SECONDS_PER_YEAR * t.drainage[biggest];
        assert!((r.discharge - expected).abs() / expected < 0.2, "{} vs {}", r.discharge, expected);
        for r in &h.reaches {
            assert!(r.depth.is_finite() && r.speed.is_finite() && r.froude.is_finite());
        }
    }
}
