//! Chromatic media, whose extinction differs between wavelengths (or colour channels), and
//! pbrt's spectral MIS for tracking through them (Pharr, Jakob and Humphreys, "Physically
//! Based Rendering", 4th ed., sections 14.1.2-14.1.3, after Miller, Georgiev and Jarosz, "A
//! Null-Scattering Path Integral Formulation of Light Transport", ACM Transactions on Graphics
//! 38(4), SIGGRAPH 2019).
//!
//! Delta tracking samples where light collides from the extinction σ_t, but in a chromatic
//! medium (coloured smoke, juice, wax) σ_t is different for each of a path's channels, and one
//! walk can only follow one of them. So each path picks a "hero" channel h at random and tracks
//! with it, against a majorant σ̄ common to all channels (the largest): at each tentative
//! collision, real with probability σ_h / σ̄, otherwise null. Had channel c been the hero, the
//! same path would have had probability σ_c / σ̄ at a real collision and σ_n,c / σ̄ = 1 - σ_c /
//! σ̄ at a null one (the common majorant's free-flight densities cancel). The path's
//! contribution in channel c, f_c over the probability of the techniques that might have made
//! it, is weighted by one-sample MIS with the balance heuristic, f_c / (⅓ Σ_k p_k), unbiased
//! whichever channel was the hero. In the null-scattering formulation f_c = p_c at every
//! tentative collision, so all that is needed are the ratios r_c = p_c / p_h along the path,
//! and light reaching the camera in channel c is weighted r_c / mean(r): at most three, and
//! one when the channels agree. pbrt calls these the rescaled path probabilities r_u.
//!
//! Through grey media, and on paths that never meet a chromatic one, the ratios stay one and
//! cost nothing. Shadow rays need no MIS: ratio tracking against the common majorant gives an
//! unbiased transmittance in every channel at once (`Volume::transmittance_color`).
//!
//! Media are objects here, found by intersecting them (`Geometry::is_stochastic`), rather than
//! the space between surfaces as in pbrt, so a walk doesn't know where it will end: a nearer
//! surface or another medium's collision may cut it short. A chromatic walk therefore records
//! its tentative collisions (`record`), and once the nearest hit along the ray is known, the
//! trace applies those before it (`settle`), with the real collision only if it was the hit.

use crate::color::Color;
use crate::geometry::rand;
use std::cell::{Cell, RefCell};

/// One tentative collision of a chromatic walk: where, and p_c / p_h for each channel.
#[derive(Clone, Copy, Debug)]
struct Collision {
    t: f64,
    ratios: [f64; 3],
    real: bool,
}

thread_local! {
    static HERO: Cell<Option<usize>> = const { Cell::new(None) };
    static RATIOS: Cell<[f64; 3]> = const { Cell::new([1.; 3]) };
    static PENDING: RefCell<Vec<Collision>> = const { RefCell::new(Vec::new()) };
}

/// Start a new path: no hero yet, and ratios of one.
pub fn begin_path() {
    HERO.with(|h| h.set(None));
    RATIOS.with(|r| r.set([1.; 3]));
    PENDING.with(|p| p.borrow_mut().clear());
}

/// The channel this path tracks chromatic media with, chosen the first time it meets one so
/// that paths that never do draw no random number for it.
pub fn hero() -> usize {
    HERO.with(|h| match h.get() {
        Some(c) => c,
        None => {
            let c = ((rand() * 3.) as usize).min(2);
            h.set(Some(c));
            c
        },
    })
}

/// Before intersecting a ray with the scene: forget the collisions of earlier rays' walks.
pub fn begin_query() {
    PENDING.with(|p| {
        if let Ok(mut p) = p.try_borrow_mut() {
            p.clear();
        }
    });
}

/// A chromatic walk's tentative collision at `t`, with p_c / p_h per channel.
pub fn record(t: f64, ratios: [f64; 3], real: bool) {
    PENDING.with(|p| p.borrow_mut().push(Collision { t, ratios, real }));
}

/// The nearest hit along the ray was at `hit` (None if nothing): apply the tentative
/// collisions before it, and the real collision that was it.
pub fn settle(hit: Option<f64>) {
    let hit = hit.unwrap_or(f64::INFINITY);
    PENDING.with(|p| {
        let mut p = p.borrow_mut();
        if p.is_empty() {
            return;
        }
        RATIOS.with(|r| {
            let mut ratios = r.get();
            for c in p.iter().filter(|c| if c.real { c.t == hit } else { c.t < hit }) {
                for (k, v) in ratios.iter_mut().enumerate() {
                    *v *= c.ratios[k];
                }
            }
            r.set(ratios);
        });
        p.clear();
    });
}

/// The weight of light gathered at this point of the path, per channel: r_c / mean(r).
pub fn weight() -> Color {
    weight_of(RATIOS.with(|r| r.get()))
}

/// For a side ray that doesn't continue the path (a sun probe, `VolumeMaterial::sun_probe`):
/// the weight of light gathered where it hit (at `hit`), as `settle` then `weight` would give,
/// but leaving the path's ratios as they were, and forgetting the side ray's collisions.
pub fn side_weight(hit: Option<f64>) -> Color {
    let hit = hit.unwrap_or(f64::INFINITY);
    let mut ratios = RATIOS.with(|r| r.get());
    PENDING.with(|p| {
        let mut p = p.borrow_mut();
        for c in p.iter().filter(|c| if c.real { c.t == hit } else { c.t < hit }) {
            for (k, v) in ratios.iter_mut().enumerate() {
                *v *= c.ratios[k];
            }
        }
        p.clear();
    });
    weight_of(ratios)
}

fn weight_of(r: [f64; 3]) -> Color {
    if r == [1.; 3] {
        return Color::white();
    }
    let mean = (r[0] + r[1] + r[2]) / 3.;
    if mean <= 0. {
        return Color::black();
    }
    Color::new(r[0] / mean, r[1] / mean, r[2] / mean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settle_applies_only_collisions_before_the_hit() {
        begin_path();
        record(1., [2., 1., 0.5], false);
        record(3., [4., 1., 1.], false);
        record(5., [0.5, 1., 2.], true);
        // Cut short by a surface at 4: the null collisions at 1 and 3 count.
        settle(Some(4.));
        let w = weight();
        let r = [8., 1., 0.5];
        let mean = (8. + 1. + 0.5) / 3.;
        assert!((w.rgb.x - r[0] / mean).abs() < 1e-12 && (w.rgb.z - r[2] / mean).abs() < 1e-12, "{}", w);
        // The real collision counts when it is the hit.
        begin_path();
        record(5., [0.5, 1., 2.], true);
        settle(Some(5.));
        assert!((weight().rgb.z - 2. / (3.5 / 3.)).abs() < 1e-12);
        begin_path();
        assert_eq!(weight(), Color::white());
    }
}
