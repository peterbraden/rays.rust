//! Arcus and volutus: the clouds of a storm's gust front. A density field for `volume::Volume`.
//!
//! A thunderstorm's downdraught spreads over the ground as a dense pool of rain-cooled air whose
//! leading edge, the gust front, advances like a little cold front. The warm, damp air ahead is
//! shoved up over it, past its condensation level, and makes cloud along the front (WMO
//! International Cloud Atlas: the supplementary feature arcus, and the species volutus).
//!
//! - **Shelf cloud** (`"form": "shelf"`): a wedge attached to the storm's base. Its lowest edge
//!   leads, and its face slopes up and back to the storm, smooth where the air is lifted
//!   steadily and stepped in tiers where it rises in layers. Behind the edge the underside rises
//!   towards the rain, ragged with scud where the outflow churns it (the "whale's mouth").
//! - **Roll cloud** (`"form": "roll"`, volutus): a horizontal tube detached from the storm,
//!   ahead of the front or after it has weakened, turning about its own axis on the shear
//!   between the outflow and the air above it; smooth, with streaks along its length.
//!
//! The field is laid out in the front's frame: `along` the front, `ahead` of it (in the
//! direction it advances, `motion`) and height. Each part of the cloud is bounded by margins,
//! signed distances in metres inside its surfaces, each worn by noise of its own size (the face
//! barely, the underside a lot), and the density rises over `softness` from the nearest.

use crate::na::{Vector2, Vector3};
use crate::noise::PerlinNoise;
use crate::procedural::nubis_clouds::smoothstep;
use crate::volume::DensityField;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Form {
    Shelf,
    Roll,
}

pub struct Arcus {
    pub form: Form,
    /// A point on the front's leading edge ([x, z]), and the direction it advances.
    pub center: Vector2<f64>,
    pub motion: Vector2<f64>,
    /// The front's length (m), and how far it wanders ahead or back along it.
    pub length: f64,
    pub waviness: f64,
    /// Shelf: the leading edge's height, the storm's base it rises back to over `depth`, and
    /// the cloud's top.
    pub low: f64,
    pub base: f64,
    pub depth: f64,
    pub top: f64,
    /// Shelf: how far back the face leans per metre up; tiers `tier_spacing` apart, each with
    /// a lip `tier_lip` deep; scud below the underside, `scud` metres deep.
    pub face_slope: f64,
    pub tier_spacing: f64,
    pub tier_lip: f64,
    pub scud: f64,
    /// Roll: its axis's height and distance ahead of `center`, and its radius.
    pub height: f64,
    pub ahead: f64,
    pub radius: f64,
    /// Extinction inside (per metre), and the depth over which it rises at the edges (m).
    pub density: f64,
    pub softness: f64,
    noise: PerlinNoise,
}

impl Arcus {
    pub fn new(form: Form, seed: u64) -> Arcus {
        Arcus {
            form,
            center: Vector2::zeros(),
            motion: Vector2::new(0., -1.),
            length: 30_000.,
            waviness: 600.,
            low: 500.,
            base: 1400.,
            depth: 5000.,
            top: 3000.,
            face_slope: 1.6,
            tier_spacing: 220.,
            tier_lip: 90.,
            scud: 180.,
            height: 700.,
            ahead: 1500.,
            radius: 280.,
            density: 0.04,
            softness: 40.,
            noise: PerlinNoise::shuffled(seed.wrapping_mul(29).wrapping_add(11)),
        }
    }

    pub fn bottom(&self) -> f64 {
        match self.form {
            Form::Shelf => self.low - self.scud,
            Form::Roll => self.height - 1.4 * self.radius,
        }
    }

    pub fn top_height(&self) -> f64 {
        match self.form {
            Form::Shelf => self.top + 200.,
            Form::Roll => self.height + 1.4 * self.radius,
        }
    }

    /// How far from `center` the cloud can reach in x or z.
    pub fn reach(&self) -> f64 {
        let across = match self.form {
            Form::Shelf => 2. * self.depth + self.waviness + (self.top - self.low) * self.face_slope,
            Form::Roll => self.ahead.abs() + self.waviness + 1.4 * self.radius,
        };
        (0.5 * self.length + 1000.).max(across + 100.)
    }

    /// Noise centred on 0, roughly -1 to 1.
    fn n(&self, x: f64, y: f64, z: f64, octaves: u32) -> f64 {
        2. * (self.noise.fbm(x, y, z, octaves, 0.5, 2.) - 0.5)
    }

    /// The fraction of `density` at `p`, 0 to 1.
    pub fn fraction(&self, p: &Vector3<f64>) -> f64 {
        if p.y < self.bottom() || p.y > self.top_height() {
            return 0.;
        }
        let m = self.motion;
        let rel = Vector2::new(p.x, p.z) - self.center;
        let ahead = rel.dot(&m);
        let along = rel.x * m.y - rel.y * m.x;
        // The front wanders, and its ends fray.
        let x = ahead - self.waviness * self.n(along / 6000., 0.3, 0.7, 2);
        let ends = 0.5 * self.length - along.abs() + 600. * self.n(along / 2000., 5.1, 1.3, 2);
        let margin = match self.form {
            Form::Shelf => self.shelf(x, along, p.y),
            Form::Roll => self.roll(x, along, p.y),
        }.min(ends);
        if margin <= 0. {
            return 0.;
        }
        let edge = smoothstep(0., self.softness, margin);
        // Uneven inside, though the cloud stays opaque.
        let texture = 0.75 + 0.5 * (0.5 + 0.5 * self.n(along / 700., p.y / 300., x / 700., 3));
        (edge * texture).min(1.)
    }

    /// The shelf's margin at `x` metres ahead of the leading edge.
    fn shelf(&self, x: f64, along: f64, y: f64) -> f64 {
        let rise = y - self.low;
        // The face: leaning back, stepped in tiers whose lips jut out at their bottoms, nearly
        // smooth (lifted air is laminar) but for a gentle wear along the front.
        let k = (rise / self.tier_spacing).max(0.);
        let lip = self.tier_lip * (1. - k.fract()).powi(3) * smoothstep(0., 0.6, k);
        let face = -self.face_slope * rise.max(0.) + lip + 25. * self.n(along / 400., y / 150., 2.7, 2);
        let front = face - x;
        // Behind the edge the underside rises to the storm's base, ragged with scud.
        let behind = (-x).max(0.);
        let under = self.low + (self.base - self.low) * smoothstep(0., self.depth, behind);
        let ragged = self.scud * (0.5 + 0.5 * self.n(along / 300., y / 200., x / 300., 4));
        let below = y - under + (ragged - 0.5 * self.scud) * smoothstep(0., 600., behind + 200.);
        // Its top, and the rear where the deck thins away into the storm's rain.
        let above = self.top - y + 200. * self.n(along / 1500., 9.1, x / 1500., 2);
        let rear = 2. * self.depth + x;
        front.min(below).min(above).min(rear)
    }

    /// The roll's margin: a tube along the front, its radius swelling and narrowing.
    fn roll(&self, x: f64, along: f64, y: f64) -> f64 {
        let (dx, dy) = (x - self.ahead, y - self.height);
        let r = (dx * dx + dy * dy).sqrt();
        let theta = dy.atan2(dx);
        let radius = self.radius * (1. + 0.25 * self.n(along / 2500., 3.3, 0.1, 2));
        // Streaks along the axis, from the roll's turning; the angle is wrapped onto a circle
        // in the noise's space so there is no seam.
        let (c, s) = (theta.cos(), theta.sin());
        let streaks = 0.12 * radius * self.n(along / 1800. + 2.2 * c, 2.2 * s, r / 200., 3);
        radius - r + streaks
    }
}

impl DensityField for Arcus {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        self.density * self.fraction(p)
    }

    fn majorant(&self) -> f64 {
        self.density
    }

    /// A slab: `density` within it, nothing above or below.
    fn majorant_from(&self, p: &Vector3<f64>, d: &Vector3<f64>) -> (f64, f64) {
        let (lo, hi) = (self.bottom(), self.top_height());
        let to = |y: f64| if d.y != 0. { (y - p.y) / d.y } else { f64::INFINITY };
        if p.y < lo {
            (0., if d.y > 0. { to(lo) } else { f64::INFINITY })
        } else if p.y > hi {
            (0., if d.y < 0. { to(hi) } else { f64::INFINITY })
        } else {
            (self.density, if d.y > 0. { to(hi) } else if d.y < 0. { to(lo) } else { f64::INFINITY })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Pcg32;

    fn shelf() -> Arcus {
        let mut a = Arcus::new(Form::Shelf, 1);
        a.waviness = 0.;
        a
    }

    #[test]
    fn test_shelf_is_a_wedge_leaning_back() {
        let a = shelf();
        // Clear air ahead of the leading edge, cloud just behind it.
        assert_eq!(a.fraction(&Vector3::new(0., a.low + 50., -400.)), 0.);
        assert!(a.fraction(&Vector3::new(0., a.low + 150., 300.)) > 0.5);
        // Higher up the face is further back: at 1 km above the edge, cloud only well behind.
        let y = a.low + 1000.;
        assert_eq!(a.fraction(&Vector3::new(0., y, 500.)), 0.);
        assert!(a.fraction(&Vector3::new(0., y, a.face_slope * 1000. + 400.)) > 0.5);
    }

    #[test]
    fn test_shelf_underside_rises_to_the_base_behind() {
        let a = shelf();
        // Far behind the edge, just below the base is clear (bar the scud), above it cloud.
        let behind = 2.0 * a.depth;
        let mut clear = 0;
        for i in 0..20 {
            let along = i as f64 * 400.;
            if a.fraction(&Vector3::new(along, a.base - a.scud - 50., behind * 0.8)) == 0. {
                clear += 1;
            }
            assert!(a.fraction(&Vector3::new(along, a.base + 300., behind * 0.8)) > 0.);
        }
        assert!(clear >= 15, "{}", clear);
    }

    #[test]
    fn test_roll_is_a_detached_tube() {
        let mut a = Arcus::new(Form::Roll, 2);
        a.waviness = 0.;
        // motion is -z, so `ahead` lies towards -z.
        let axis = Vector3::new(0., a.height, -a.ahead);
        assert!(a.fraction(&axis) > 0.5);
        assert_eq!(a.fraction(&(axis + Vector3::new(0., 2. * a.radius, 0.))), 0.);
        assert_eq!(a.fraction(&(axis + Vector3::new(0., 0., 2. * a.radius))), 0.);
        // Long along the front.
        assert!(a.fraction(&(axis + Vector3::new(8000., 0., 0.))) > 0.3);
        assert_eq!(a.fraction(&(axis + Vector3::new(0.5 * a.length + 3000., 0., 0.))), 0.);
    }

    #[test]
    fn test_majorant_bounds_density() {
        for form in [Form::Shelf, Form::Roll] {
            let a = Arcus::new(form, 4);
            let mut rng = Pcg32::new(3, 0);
            for _ in 0..300 {
                let p = Vector3::new(rng.next_f64() * 8000. - 4000., rng.next_f64() * 4000., rng.next_f64() * 8000. - 4000.);
                let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
                let (m, len) = a.majorant_from(&p, &d);
                for k in 0..30 {
                    let q = p + d * (len.min(5000.) * k as f64 / 30.);
                    assert!(a.density(&q) <= m + 1e-12);
                }
            }
        }
    }
}
