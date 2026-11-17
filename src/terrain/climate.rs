//! # Climate, powered by latitude
//!
//! A planet's climate zones follow from how much sunlight each latitude gets and how the
//! atmosphere moves heat and water about. This models both simply, over the whole planet:
//!
//! - **Insolation.** The daily-mean sunlight at latitude φ when the sun's declination is δ is
//!   Q = (S₀/π)(h₀ sin φ sin δ + cos φ cos δ sin h₀), h₀ the hour angle of sunset, cos h₀ =
//!   -tan φ tan δ (Berger, "Long-term variations of daily insolation and Quaternary climatic
//!   changes", J. Atmos. Sci. 35, 1978). Through a year on a circular orbit, sin δ = sin ε sin λ
//!   for axial tilt ε and the sun's longitude λ: the annual mean, and the solstices' extremes.
//! - **Temperature.** An energy balance model (Budyko 1969; Sellers 1969; North, "Theory of
//!   energy-balance climate models", J. Atmos. Sci. 32, 1975, doi:10.1175/1520-0469(1975)
//!   032<2033:TOEBCM>2.0.CO;2): each latitude absorbs Q(1 - α), radiates A + BT to space, and
//!   exchanges heat with its neighbours by diffusion, d/dx(D(1 - x²) dT/dx) with x = sin φ.
//!   Ice is brighter than ground (α rises where T < -10 °C), so cold makes ice and ice makes
//!   cold. With North's constants this gives the Earth's zonal mean temperatures: about 28 °C at
//!   the equator and -20 °C at the poles. The seasons swing temperature by the solstices'
//!   difference in insolation, damped by the ocean's heat capacity near the sea
//!   (continentality). Temperature falls 6.5 °C per kilometre of height (the standard
//!   atmosphere's lapse rate).
//! - **Circulation and rain.** Three cells in each hemisphere (Hadley, Ferrel, polar): air rises
//!   and rains at the equator (the intertropical convergence zone) and the polar front near 60°,
//!   and sinks dry near 30° and at the poles. Surface winds blow from the east in the tropics
//!   (the trades) and near the poles, and from the west between (the westerlies). This sets a
//!   zonal pattern of rainfall, and the direction moisture travels.
//! - **Moisture.** Over the ocean the air is moist. Over land it carries moisture downwind,
//!   losing it to rain as it goes (a recycling length of 3000 km) and wrung out where it rises
//!   over mountains: the windward slopes are wetter, the lee in a rain shadow. Solved on a
//!   cubed sphere by sweeping semi-Lagrangian steps upwind until it settles.
//!
//! What grows follows from temperature and rain (`Biome`): Köppen's criteria for dry climates
//! (rain below about 20 mm per °C of mean temperature plus 140 mm) and tropical ones, the
//! treeline where the warmest month reaches 10 °C, permanent snow where summer stays below
//! freezing.

use crate::na::Vector3;
use crate::terrain::domain::{Domain, Layout};
use crate::terrain::planet::Planet;
use std::collections::HashMap;
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::{Arc, Mutex, OnceLock};
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ClimateParams {
    /// Axial tilt, degrees.
    pub tilt: f64,
    /// Solar constant, W/m².
    pub solar: f64,
    /// Latitude where the Hadley cells end and the dry subtropics are, degrees.
    pub hadley: f64,
    /// Grid cells per cube face edge.
    pub cells: usize,
    /// Distance over which air crossing land rains out its moisture, metres.
    pub recycling: f64,
    /// Rise over which air loses 1/e of its moisture, metres.
    pub orographic: f64,
    /// Added to every temperature, °C (a warmer or colder world).
    pub warming: f64,
}

impl Default for ClimateParams {
    fn default() -> ClimateParams {
        ClimateParams { tilt: 23.44, solar: 1361., hadley: 30., cells: 64, recycling: 3.0e6, orographic: 1200., warming: 0. }
    }
}

/// Temperature lapse rate, °C per metre.
pub const LAPSE_RATE: f64 = 0.0065;

/// Daily-mean insolation at latitude `lat` (radians) when the sun's declination is `dec`, W/m².
pub fn daily_insolation(solar: f64, lat: f64, dec: f64) -> f64 {
    let c = -lat.tan() * dec.tan();
    let h0 = if c >= 1. { 0. } else if c <= -1. { PI } else { c.acos() };
    (solar / PI * (h0 * lat.sin() * dec.sin() + lat.cos() * dec.cos() * h0.sin())).max(0.)
}

/// Annual mean, and the solstices' (northern summer, southern summer), daily insolation at
/// latitude `lat` (radians) on a circular orbit with axial tilt `tilt` (radians).
pub fn insolation(solar: f64, tilt: f64, lat: f64) -> (f64, f64, f64) {
    let samples = 360;
    let mean = (0..samples).map(|k| {
        let lambda = TAU * (k as f64 + 0.5) / samples as f64;
        daily_insolation(solar, lat, (tilt.sin() * lambda.sin()).asin())
    }).sum::<f64>() / samples as f64;
    (mean, daily_insolation(solar, lat, tilt), daily_insolation(solar, lat, -tilt))
}

/// Zonal mean sea-level temperature, °C, at `n` latitudes evenly spaced in x = sin φ (cell
/// centres), by North's diffusive energy balance model with ice-albedo feedback.
pub fn energy_balance(p: &ClimateParams, n: usize) -> Vec<f64> {
    // North (1975, 1981): outgoing longwave A + B T, diffusion D, all W/m² (per °C).
    let (a, b, d) = (203.3, 2.09, 0.649);
    let tilt = p.tilt.to_radians();
    let dx = 2. / n as f64;
    let x: Vec<f64> = (0..n).map(|i| -1. + (i as f64 + 0.5) * dx).collect();
    let q: Vec<f64> = x.iter().map(|x| insolation(p.solar, tilt, x.asin()).0).collect();
    let mut t: Vec<f64> = vec![10.; n];
    for _ in 0..200 {
        // Albedo: clouds and ground, rising from 0.27 at the equator to 0.39 at the poles
        // (low sun, more cloud; after North's fit, a + b P₂(x)), and 0.62 over ice and snow, which
        // lie where the annual mean is below about -5 °C.
        let absorbed: Vec<f64> = (0..n).map(|i| {
            let ice = 1. - ((t[i] + 7.) / 4.).clamp(0., 1.);
            let clear = 0.27 + 0.12 * x[i] * x[i];
            q[i] * (1. - (clear + (0.62 - clear) * ice))
        }).collect();
        // -d/dx(D(1-x²) dT/dx) + B T = absorbed - A: tridiagonal, by Thomas's algorithm.
        let face = |k: f64| d * (1. - k * k) / (dx * dx);
        let (mut lower, mut diag, mut upper, mut rhs) = (vec![0.; n], vec![0.; n], vec![0.; n], vec![0.; n]);
        for i in 0..n {
            let west = if i > 0 { face(x[i] - dx / 2.) } else { 0. };
            let east = if i + 1 < n { face(x[i] + dx / 2.) } else { 0. };
            lower[i] = -west;
            upper[i] = -east;
            diag[i] = b + west + east;
            rhs[i] = absorbed[i] - a;
        }
        for i in 1..n {
            let m = lower[i] / diag[i - 1];
            diag[i] -= m * upper[i - 1];
            rhs[i] -= m * rhs[i - 1];
        }
        let mut next = vec![0.; n];
        next[n - 1] = rhs[n - 1] / diag[n - 1];
        for i in (0..n - 1).rev() { next[i] = (rhs[i] - upper[i] * next[i + 1]) / diag[i]; }
        let change = next.iter().zip(&t).map(|(a, b)| (a - b).abs()).fold(0., f64::max);
        // Damped, as the albedo switches.
        for i in 0..n { t[i] = 0.5 * t[i] + 0.5 * next[i]; }
        if change < 1e-4 { break; }
    }
    t.iter().map(|t| t + p.warming).collect()
}

/// Surface wind at latitude `lat` (radians): (eastward, northward), roughly unit at most. The
/// trades blow from the east and towards the equator, the westerlies from the west and towards
/// the poles, the polar easterlies from the east; calms between.
pub fn wind(p: &ClimateParams, lat: f64) -> (f64, f64) {
    let a = lat.abs().to_degrees();
    let h = p.hadley;
    // Phase through the three cells: 0 to π across each.
    let phase = if a < h { a / h * PI } else if a < 60. { (a - h) / (60. - h) * PI + PI } else { (a - 60.) / 30. * PI + TAU };
    let s = phase.sin();
    // sin is positive in the Hadley and polar cells, negative in the Ferrel cell.
    (-s, -0.4 * s * lat.signum())
}

/// Zonal-mean rainfall over the ocean and coasts at latitude `lat` (radians), mm/yr: the
/// intertropical convergence zone (wide, as it follows the sun through the year), the drier
/// subtropics, the storm tracks and the dry poles; roughly the observed zonal means (GPCP:
/// about 2000 mm at the equator, 700 at 25°, 1000 at 50°, 200 near the poles).
pub fn zonal_precipitation(p: &ClimateParams, lat: f64) -> f64 {
    let a = lat.abs().to_degrees();
    let h = p.hadley;
    let itcz = 1700. * (-(a / (0.6 * h)).powi(2)).exp();
    let storms = 900. * (-((a - (h + 20.)) / 15.).powi(2)).exp();
    let dry = 1. - 0.35 * (-((a - (h - 5.)) / 8.).powi(2)).exp();
    let cold = 1. - 0.75 * ((a - 60.) / 30.).clamp(0., 1.);
    (300. + itcz + storms) * dry * cold
}

/// Over land, the sinking air of the subtropical highs suppresses rain further: the deserts.
pub fn subsidence(p: &ClimateParams, lat: f64) -> f64 {
    1. - 0.5 * (-((lat.abs().to_degrees() - (p.hadley - 5.)) / 8.).powi(2)).exp()
}

/// The climate over the planet.
pub struct Climate {
    pub params: ClimateParams,
    domain: Domain,
    /// At the grid's corners: annual mean sea-level temperature (°C), the seasons' half range
    /// (°C), annual rainfall (mm), and continentality (0 at sea, 1 deep inland).
    temperature: Vec<f64>,
    season: Vec<f64>,
    precipitation: Vec<f64>,
    continentality: Vec<f64>,
}

/// The climate at a point.
#[derive(Clone, Copy, Debug)]
pub struct Local {
    /// Annual mean temperature, °C.
    pub mean: f64,
    /// Warmest and coldest months, °C.
    pub summer: f64,
    pub winter: f64,
    /// Rainfall, mm/yr.
    pub precipitation: f64,
}

impl Local {
    /// Köppen's threshold for a dry climate, mm/yr: below it is steppe, below half desert.
    pub fn dry_threshold(&self) -> f64 { (20. * self.mean + 140.).max(100.) }

    /// Height (metres) above which summer stays below freezing: the snowline. Where it snows
    /// more it lies lower (maritime glaciers reach far down), where it is dry, higher.
    pub fn snowline(&self, here: f64) -> f64 {
        here + self.summer / LAPSE_RATE - 300. * (self.precipitation / 1000.).max(0.05).ln()
    }

    /// Height above which the warmest month is below 10 °C: no trees grow.
    pub fn treeline(&self, here: f64) -> f64 { here + (self.summer - 10.) / LAPSE_RATE }
}

impl Climate {
    pub fn new(params: ClimateParams, planet: &Planet) -> Climate {
        let n = params.cells.max(4);
        let radius = planet.params.radius;
        let domain = Domain::sphere(n, radius);
        let spacing = FRAC_PI_2 * radius / n as f64;
        let cells = domain.len();
        // Ground height over the grid, at the scale of regions; the sea is at 0.
        let height: Vec<f64> = domain.dirs.par_iter().map(|d| planet.regional_elevation(d)).collect();
        let land: Vec<bool> = height.iter().map(|&h| h > 0.).collect();
        let latitude: Vec<f64> = domain.dirs.iter().map(|d| d.y.clamp(-1., 1.).asin()).collect();
        // Moisture carried by the wind, 1 at sea, by semi-Lagrangian sweeps upwind.
        let upwind: Vec<(Vector3<f64>, f64)> = domain.dirs.iter().zip(&latitude).map(|(d, &lat)| {
            let east = Vector3::y().cross(d);
            let east = if east.norm() > 1e-9 { east.normalize() } else { Vector3::x() };
            let north = d.cross(&east);
            let (u, v) = wind(&params, lat);
            let w = east * u + north * v;
            let speed = w.norm();
            let back = if speed > 1e-6 { (d - w / speed * (spacing / radius)).normalize() } else { *d };
            (back, speed.min(1.))
        }).collect();
        let corners_of = |field: &[f64]| -> Vec<f64> {
            domain.corner_cells.iter().map(|cs| cs.iter().map(|&c| field[c]).sum::<f64>() / cs.len() as f64).collect()
        };
        let mut moisture: Vec<f64> = land.iter().map(|&l| if l { 0.5 } else { 1. }).collect();
        let mut shadow = vec![0.; cells];
        for _ in 0..(4 * n) {
            let at_corners = corners_of(&moisture);
            let ground = height.iter().map(|h| h.max(0.)).collect::<Vec<f64>>();
            let ground_corners = corners_of(&ground);
            let next: Vec<(f64, f64)> = (0..cells).into_par_iter().map(|g| {
                if !land[g] { return (1., 0.); }
                let (back, speed) = upwind[g];
                let m_up = crate::terrain::scenery::sample(&domain, &at_corners, &back);
                let z_up = crate::terrain::scenery::sample(&domain, &ground_corners, &back);
                let rise = (ground[g] - z_up).max(0.);
                // Calm air mixes with its surroundings rather than travelling.
                let m_calm = domain.links(g).iter().map(|l| moisture[l.to]).sum::<f64>() / domain.links(g).len() as f64;
                let carried = speed * m_up + (1. - speed) * m_calm;
                (carried * (-spacing / params.recycling - rise / params.orographic).exp(), rise)
            }).collect();
            for g in 0..cells { moisture[g] = next[g].0; shadow[g] = next[g].1; }
        }
        // Continentality: how far the air has come over land.
        let continentality: Vec<f64> = (0..cells).map(|g| if land[g] { (1. - moisture[g]).clamp(0., 1.).sqrt() } else { 0. }).collect();
        // Temperature by latitude from the energy balance; the seasons by the solstices'
        // insolation, damped near the sea.
        let bands = 180;
        let zonal = energy_balance(&params, bands);
        let tilt = params.tilt.to_radians();
        let temperature: Vec<f64> = latitude.iter().map(|&lat| {
            let x = lat.sin();
            let k = (((x + 1.) / 2. * bands as f64) as usize).min(bands - 1);
            zonal[k]
        }).collect();
        let season: Vec<f64> = (0..cells).map(|g| {
            let (_, june, december) = insolation(params.solar, tilt, latitude[g]);
            (june - december).abs() / 2. * (0.02 + 0.07 * continentality[g])
        }).collect();
        // Rain: the zonal pattern, less where the air has dried, more on windward slopes.
        let precipitation: Vec<f64> = (0..cells).map(|g| {
            let base = zonal_precipitation(&params, latitude[g]);
            if !land[g] { return base; }
            let windward = 1. + 2.5 * (shadow[g] / 1000.).min(1.);
            // In the tropics most rain is local convection, much of it fed by moisture the
            // forest itself transpires (about half, in the Amazon): it depends less on air
            // arriving from the sea.
            let local = 0.2 + 0.5 * (-(latitude[g].to_degrees() / 12.).powi(2)).exp();
            base * subsidence(&params, latitude[g]) * (local + (1. - local) * moisture[g]) * windward
        }).collect();
        Climate {
            temperature: corners_of(&temperature), season: corners_of(&season),
            precipitation: corners_of(&precipitation), continentality: corners_of(&continentality),
            domain, params,
        }
    }

    fn at(&self, field: &[f64], d: &Vector3<f64>) -> f64 { crate::terrain::scenery::sample(&self.domain, field, d) }

    /// The climate at direction `d` and height `h` metres.
    pub fn local(&self, d: &Vector3<f64>, h: f64) -> Local {
        let mean = self.at(&self.temperature, d) - LAPSE_RATE * h.max(0.);
        let season = self.at(&self.season, d);
        // Where it is cold most of the year, snow lingers into summer, reflecting the sun and
        // holding the air near freezing while it melts: summers warm less.
        let thaw = 0.3 + 0.7 * crate::procedural::nubis_clouds::smoothstep(-20., 0., mean);
        Local { mean, summer: mean + season * thaw, winter: mean - season, precipitation: self.at(&self.precipitation, d).max(0.) }
    }

    pub fn continentality(&self, d: &Vector3<f64>) -> f64 { self.at(&self.continentality, d) }

    fn n(&self) -> usize { match self.domain.layout { Layout::Sphere { n } => n, _ => unreachable!() } }
}

/// The climate for a planet, computed once per process and shared.
pub fn climate(params: &ClimateParams, planet: &Planet) -> Arc<Climate> {
    static CLIMATES: OnceLock<Mutex<HashMap<String, Arc<Climate>>>> = OnceLock::new();
    let key = format!("{:?} {:?}", params, planet.params);
    let cache = CLIMATES.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();
    if let Some(c) = map.get(&key) { return c.clone(); }
    let start = std::time::Instant::now();
    let c = Arc::new(Climate::new(params.clone(), planet));
    println!("Climate: {} cells in {:.1?}", 6 * c.n() * c.n(), start.elapsed());
    map.insert(key, c.clone());
    c
}

/// What grows, by climate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Biome {
    Ice,
    Tundra,
    Taiga,
    TemperateForest,
    Grassland,
    Steppe,
    ColdDesert,
    HotDesert,
    Savanna,
    Rainforest,
}

impl Biome {
    /// The biome of a climate (Köppen's main groups, with Whittaker's in mind).
    pub fn of(c: &Local) -> Biome {
        if c.summer < 0. { return Biome::Ice; }
        if c.summer < 10. { return Biome::Tundra; }
        let dry = c.dry_threshold();
        if c.precipitation < dry / 2. { return if c.mean > 18. { Biome::HotDesert } else { Biome::ColdDesert }; }
        if c.precipitation < dry { return if c.mean > 20. { Biome::Savanna } else { Biome::Steppe }; }
        if c.winter > 18. { return if c.precipitation > 1400. { Biome::Rainforest } else { Biome::Savanna }; }
        if c.winter < -3. && c.mean < 4. { return Biome::Taiga; }
        if c.precipitation > 1.6 * dry { Biome::TemperateForest } else { Biome::Grassland }
    }

    /// Its albedo (linear RGB), seen from afar.
    pub fn albedo(&self) -> [f64; 3] {
        match self {
            Biome::Ice => [0.80, 0.83, 0.87],
            Biome::Tundra => [0.11, 0.10, 0.07],
            Biome::Taiga => [0.025, 0.038, 0.024],
            Biome::TemperateForest => [0.035, 0.06, 0.025],
            Biome::Grassland => [0.08, 0.10, 0.04],
            Biome::Steppe => [0.17, 0.15, 0.09],
            Biome::ColdDesert => [0.24, 0.21, 0.16],
            Biome::HotDesert => [0.42, 0.31, 0.18],
            Biome::Savanna => [0.15, 0.13, 0.06],
            Biome::Rainforest => [0.025, 0.05, 0.018],
        }
    }

    /// Whether trees cover it.
    pub fn forest(&self) -> bool {
        matches!(self, Biome::Taiga | Biome::TemperateForest | Biome::Rainforest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insolation_falls_towards_the_poles_and_swings_with_the_seasons() {
        let tilt = 23.44f64.to_radians();
        let (eq, eq_june, _) = insolation(1361., tilt, 0.);
        let (pole, pole_june, pole_dec) = insolation(1361., tilt, FRAC_PI_2 - 1e-6);
        // About 417 and 173 W/m² (the Earth's annual means at the equator and pole).
        assert!((eq - 417.).abs() < 10., "{}", eq);
        assert!((pole - 173.).abs() < 10., "{}", pole);
        assert!(pole_june > eq_june && pole_dec < 1., "{} {} {}", pole_june, eq_june, pole_dec);
    }

    #[test]
    fn test_energy_balance_gives_the_earths_zonal_temperatures() {
        let t = energy_balance(&ClimateParams::default(), 90);
        let (eq, pole) = ((t[44] + t[45]) / 2., t[89]);
        println!("equator {:.1} 30° {:.1} 45° {:.1} 60° {:.1} pole {:.1}", eq, t[67], t[76], t[83], pole);
        assert!(eq > 24. && eq < 32., "equator {}", eq);
        assert!(pole < -10. && pole > -35., "pole {}", pole);
        // And a smaller tilt or a fainter sun is colder.
        let faint = energy_balance(&ClimateParams { solar: 1300., ..ClimateParams::default() }, 90);
        assert!(faint[45] < t[45]);
    }

    #[test]
    fn test_winds_and_rain_by_latitude() {
        let p = ClimateParams::default();
        assert!(wind(&p, 15f64.to_radians()).0 < -0.5, "trades from the east");
        assert!(wind(&p, 45f64.to_radians()).0 > 0.5, "westerlies");
        assert!(wind(&p, -45f64.to_radians()).0 > 0.5);
        assert!(wind(&p, 75f64.to_radians()).0 < -0.5, "polar easterlies");
        // Trades blow towards the equator in both hemispheres.
        assert!(wind(&p, 15f64.to_radians()).1 < 0. && wind(&p, -15f64.to_radians()).1 > 0.);
        let rain = |lat: f64| zonal_precipitation(&p, lat.to_radians());
        assert!(rain(0.) > 1800. && rain(15.) > 1000. && rain(25.) < rain(15.) && rain(50.) > 900. && rain(85.) < 400.);
    }

    #[test]
    fn test_biomes() {
        let c = |mean: f64, season: f64, precipitation: f64| Local { mean, summer: mean + season, winter: mean - season, precipitation };
        assert_eq!(Biome::of(&c(26., 2., 2500.)), Biome::Rainforest);
        assert_eq!(Biome::of(&c(26., 4., 1000.)), Biome::Savanna);
        assert_eq!(Biome::of(&c(25., 5., 100.)), Biome::HotDesert);
        assert_eq!(Biome::of(&c(-2., 18., 500.)), Biome::Taiga);
        assert_eq!(Biome::of(&c(-6., 10., 300.)), Biome::Tundra);
        assert_eq!(Biome::of(&c(10., 10., 900.)), Biome::TemperateForest);
        assert_eq!(Biome::of(&c(-25., 10., 100.)), Biome::Ice);
        // The treeline sits where the warmest month is 10 °C.
        let l = c(12., 8., 1000.);
        assert!((l.treeline(0.) - 10. / LAPSE_RATE).abs() < 1e-9);
    }

    #[test]
    fn test_mountains_cast_rain_shadows() {
        use crate::terrain::planet::PlanetParams;
        let planet = Planet::new(PlanetParams { tectonics: Some(crate::terrain::tectonics::TectonicsParams { cells: 24, age: 60., ..Default::default() }), ..PlanetParams::default() });
        let c = Climate::new(ClimateParams { cells: 24, ..ClimateParams::default() }, &planet);
        // Somewhere on land it is dry and somewhere wet; the sea is never dry.
        let (mut wet, mut dry) = (0, 0);
        for d in &c.domain.dirs {
            let l = c.local(d, 0.);
            if planet.elevation(d, 1e5) > 0. {
                if l.precipitation > 1500. { wet += 1; }
                if l.precipitation < 300. { dry += 1; }
            }
        }
        assert!(wet > 0 && dry > 0, "wet {} dry {}", wet, dry);
        assert!(subsidence(&c.params, 25f64.to_radians()) < 0.6 && subsidence(&c.params, 0.) > 0.9);
    }
}
