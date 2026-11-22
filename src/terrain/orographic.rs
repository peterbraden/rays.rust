//! # Orographic rain: where the water that erodes comes from
//!
//! Rivers cut in proportion to their discharge, and discharge is rain gathered over a basin,
//! not area. Mountains make their own rain: air carried over them by the wind rises, cools and
//! condenses its vapour, which falls a little downwind, mostly on the windward slopes and just
//! past the crests; descending into the lee, the air warms, its cloud evaporates, and what is
//! left of its vapour is less: a rain shadow. Windward flanks of real ranges are wetter, more
//! deeply cut and often lower than their lee (the Southern Alps, the Olympics, the Himalaya's
//! front), and the climate's own rain (`climate`) is too coarse to see a single range.
//!
//! This follows the ingredients of Smith and Barstad's linear theory of orographic
//! precipitation ("A linear theory of orographic precipitation", J. Atmos. Sci. 61, 2004):
//! condensation where the terrain lifts the air, delayed by the time cloud takes to turn into
//! rain and fall (τ ≈ 1000-2000 s, so the rain drifts kilometres downwind of where it formed),
//! and evaporation in descending air; and, like Hergarten and Robl's linear feedback
//! precipitation model (LFPM, Geosci. Model Dev. 15, 2022), it carries vapour and cloud water
//! along the wind as two linear fields, cheap enough to run each step of a landscape's
//! evolution. It is a sketch of both, not either:
//!
//! - Along the wind, vapour W (1 where the air comes in) condenses where the air is lifted
//!   above its condensation level z_s, a fraction Δh / H_w for a rise Δh above it (H_w the
//!   scale height of water vapour, 2.5 km), into cloud C, and z_s rises with it. Coming down,
//!   the air warms and is no longer saturated: cloud evaporates (by the same rule), and the air
//!   must be lifted back to z_s before it condenses again, so a train of ridges doesn't wring
//!   it out ridge by ridge. z_s sinks back towards the ground over `saturation_recovery`, as
//!   the air mixes with moister air around it. (Condensing on every rise, a range of ridges
//!   left its lee with a fortieth of the rain on its windward side.)
//! - Cloud falls out as rain with the time τ: a fraction 1 - exp(-ds / (u τ)), u the wind.
//! - Over the sea the air takes up vapour again, back towards 1 over `sea_recharge`.
//! - Rain is the climate's (or `reference` without a climate) times W, the air's moisture
//!   left, plus the cloud that falls out per metre times `gain_length`: the length over which
//!   that much water would otherwise fall at the climate's rate. That sets how much wetter a
//!   windward slope is: with the default 50 km and a fallout time of 800 s (8 km of drift), a
//!   range rising 1.5 km gets two to three times the climate's rain on its windward side, as
//!   real ones do; it stands in for the efficiency of real storms, which the model doesn't
//!   follow.
//!
//! The wind is the climate's prevailing surface wind at the patch's latitude (`climate::wind`):
//! westerlies at middle latitudes, trades in the tropics. Only patches have it: a whole planet's
//! simulation takes the climate's rain, which has its own orographic rule at that scale.

use crate::terrain::domain::{Domain, Layout};

#[derive(Clone, Debug, PartialEq)]
pub struct OrographicParams {
    /// Wind speed, m/s.
    pub wind_speed: f64,
    /// Time cloud takes to turn into rain and fall, s.
    pub fallout_time: f64,
    /// Scale height of water vapour, m.
    pub scale_height: f64,
    /// How much rain cloud falling out makes, as the length over which as much water would
    /// fall at the climate's rate, m (see the module docs).
    pub gain_length: f64,
    /// Distance over which air over the sea takes up its vapour again, m.
    pub sea_recharge: f64,
    /// Distance over which air that has come down from a crest mixes back towards saturation,
    /// m: its condensation level sinks back towards the ground (see the module docs).
    pub saturation_recovery: f64,
    /// Rain without a climate, and the rain the rivers' erodibility is for, mm/yr.
    pub reference: f64,
}

impl Default for OrographicParams {
    fn default() -> OrographicParams {
        OrographicParams { wind_speed: 10., fallout_time: 800., scale_height: 2500., gain_length: 50e3, sea_recharge: 200e3, saturation_recovery: 30e3, reference: 1000. }
    }
}

/// Rain at each cell of `domain`, mm/yr, over a surface `h` (metres, the ground and any ice)
/// with the wind blowing towards (`east`, `north`) (any length; zero for none), where `base`
/// is the climate's rain at each cell and `sea` marks the sea. Only a patch is swept along
/// the wind; elsewhere the rain is `base`.
pub fn rain(domain: &Domain, h: &[f64], sea: &[bool], base: &[f64], wind: (f64, f64), p: &OrographicParams) -> Vec<f64> {
    let Layout::Patch { up, east, north, .. } = &domain.layout else { return base.to_vec() };
    let n = domain.len();
    let norm = wind.0.hypot(wind.1);
    if norm <= 0. { return base.to_vec(); }
    let (wx, wz) = (wind.0 / norm, wind.1 / norm);
    // Each cell's position along the wind, in the patch's plane.
    let at: Vec<(f64, f64)> = domain.dirs.iter().map(|d| Domain::tangent(*up, *east, *north, domain.radius, d)).collect();
    let along: Vec<f64> = at.iter().map(|&(x, z)| x * wx + z * wz).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| along[a].total_cmp(&along[b]).then(a.cmp(&b)));
    let drift = p.wind_speed * p.fallout_time;
    let (mut vapour, mut cloud) = (vec![1.; n], vec![0.; n]);
    let mut fell = vec![0.; n];
    // Each cell's condensation level: saturated at the ground where the air comes in.
    let mut saturated: Vec<f64> = h.iter().map(|x| x.max(0.)).collect();
    for &i in &order {
        // Upwind: the neighbours the wind blows from, weighted by how directly.
        let (mut w, mut v, mut c, mut hu, mut ds, mut level_up) = (0., 0., 0., 0., 0., 0.);
        for l in domain.links(i) {
            let j = l.to;
            let toward = ((at[i].0 - at[j].0) * wx + (at[i].1 - at[j].1) * wz) / l.dist;
            if toward <= 1e-6 { continue; }
            w += toward;
            v += toward * vapour[j];
            c += toward * cloud[j];
            hu += toward * h[j];
            level_up += toward * saturated[j];
            ds += toward * l.dist * toward;
        }
        if w <= 0. { continue; } // where the air comes in: vapour 1, no cloud, saturated
        let (mut v, mut c, hu, ds) = (v / w, c / w, hu / w, ds / w);
        let ground = h[i].max(0.);
        // The condensation level upwind, sinking back towards the ground as the air mixes.
        let mut level = level_up / w;
        if level > ground { level = ground + (level - ground) * (-ds / p.saturation_recovery).exp(); }
        if ground > level {
            let condensed = v * (1. - (-(ground - level) / p.scale_height).exp());
            v -= condensed;
            c += condensed;
            level = ground;
        }
        let fall = (ground - hu.max(0.)) / p.scale_height;
        if fall < 0. {
            let evaporated = c * (1. - fall.exp());
            c -= evaporated;
            v += evaporated;
        }
        saturated[i] = level;
        if sea[i] { v += (1. - v) * (1. - (-ds / p.sea_recharge).exp()); }
        let falls = c * (1. - (-ds / drift).exp());
        c -= falls;
        vapour[i] = v.min(1.);
        cloud[i] = c;
        fell[i] = falls / ds.max(1.);
    }
    (0..n).map(|i| base[i] * (vapour[i] + p.gain_length * fell[i])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patch() -> Domain { Domain::patch(45., 0., 64, 500., 6.371e6) }

    #[test]
    fn test_flat_ground_gets_the_climates_rain() {
        let d = patch();
        let n = d.len();
        let r = rain(&d, &vec![200.; n], &vec![false; n], &vec![800.; n], (1., 0.), &OrographicParams::default());
        assert!(r.iter().all(|&x| (x - 800.).abs() < 1e-9), "{:?}", &r[..8]);
    }

    #[test]
    fn test_ranges_are_wet_to_windward_and_dry_in_their_lee() {
        // A ridge running north-south in the middle of the patch, the wind from the west.
        let d = patch();
        let n = d.len();
        let x = |k: usize| (k % 64) as f64 - 31.5;
        let h: Vec<f64> = (0..n).map(|k| 1500. * (-(x(k) / 10.).powi(2)).exp()).collect();
        let r = rain(&d, &h, &vec![false; n], &vec![1000.; n], (1., 0.), &OrographicParams::default());
        let at = |col: i64| -> f64 { (0..64).map(|row| r[row * 64 + (col + 32) as usize]).sum::<f64>() / 64. };
        let (windward, crest, lee) = (at(-5), at(0), at(20));
        assert!(windward > 1800. && crest > 1500., "windward {} crest {}", windward, crest);
        assert!(lee < 900., "lee {}", lee);
        // With the wind the other way, the other side is the wet one.
        let back = rain(&d, &h, &vec![false; n], &vec![1000.; n], (-1., 0.), &OrographicParams::default());
        let at_back = |col: i64| -> f64 { (0..64).map(|row| back[row * 64 + (col + 32) as usize]).sum::<f64>() / 64. };
        assert!(at_back(5) > 1800. && at_back(-20) < 900., "{} {}", at_back(5), at_back(-20));
    }

    #[test]
    fn test_a_train_of_ridges_doesnt_wring_the_air_out() {
        // Eight ridges a kilometre high in a row across the wind: the air condenses rising over
        // the first, but over the rest only what it lost since, so the last still gets rain.
        let d = patch();
        let n = d.len();
        let x = |k: usize| (k % 64) as f64;
        let h: Vec<f64> = (0..n).map(|k| 500. + 500. * (x(k) * std::f64::consts::TAU / 8.).sin()).collect();
        let r = rain(&d, &h, &vec![false; n], &vec![1000.; n], (1., 0.), &OrographicParams::default());
        let mean = |cols: std::ops::Range<usize>| -> f64 { let m: Vec<f64> = (0..n).filter(|&k| cols.contains(&(k % 64))).map(|k| r[k]).collect(); m.iter().sum::<f64>() / m.len() as f64 };
        let (first, last) = (mean(0..16), mean(48..64));
        assert!(last > 0.4 * first, "first {} last {}", first, last);
    }
}
