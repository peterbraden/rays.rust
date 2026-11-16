//! # Weather systems
//!
//! The pressure systems that organise the clouds of the middle latitudes and the tropics, each
//! as a pattern of ascent, moisture and cloud in its own frame: x east, y poleward, so that one
//! description serves both hemispheres (a southern system is the northern one mirrored north to
//! south, and turns the other way, as on the Earth). In that frame cyclonic flow is always
//! anticlockwise.
//!
//! - **Extratropical cyclones** (`Kind::Cyclone`), after the Norwegian cyclone model (Bjerknes
//!   and Solberg, "Life cycle of cyclones and the polar front theory of atmospheric
//!   circulation", Geofys. Publ. 3, 1922), with the cloud of its conveyor belts (Browning,
//!   "Organization of clouds and precipitation in extratropical cyclones", in Extratropical
//!   Cyclones: The Erik Palmén Memorial Volume, AMS 1990). A low with a warm front running east
//!   from it and a cold front trailing equatorward and west; between them, the warm sector. As
//!   it matures (`stage` 0 to 1) the cold front overtakes the warm one, lifting the warm air
//!   off the ground from the centre outwards along an occluded front to the triple point.
//!   - Ahead of the warm front, warm air rides up over the cold along a gentle frontal
//!     surface (1 in 150): the cloud's base rises with distance ahead, giving the classic
//!     sequence an observer sees as the front approaches: cirrus 1000 km ahead, thickening to
//!     cirrostratus (halos), altostratus (a watery sun), then nimbostratus and steady rain for
//!     the last 300 km, with ragged stratus beneath.
//!   - In the warm sector: moist tropical air, low stratus and stratocumulus, drizzle.
//!   - Along the cold front: a narrow band of cumulonimbus where the cold air undercuts the
//!     warm, a band of rain behind it on the steeper frontal surface (1 in 60); then a clear
//!     "dry slot" of descending air, and beyond it showery cumulus in the cold air.
//!   - Round the centre, the comma head: thick cloud spiralling in along the occlusion.
//! - **Anticyclones**: sinking air, warmed and dried: clear skies above a low inversion, under
//!   which, over the sea, the eastern flanks of the subtropical highs hold sheets of
//!   stratocumulus, and the western, trade cumulus.
//! - **Tropical cyclones**: a clear eye, a ring of cumulonimbus in the eyewall, spiral rain
//!   bands, and the central dense overcast of their anvils.

use crate::na::{Vector2, Vector3};
use crate::procedural::nubis_clouds::smoothstep;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Cyclone,
    Anticyclone,
    Tropical,
}

/// A front, as a polyline in its system's frame (metres): `points` along it, and which side
/// the cold air is on (+1 left of the direction of travel along it, -1 right).
#[derive(Clone, Debug)]
pub struct Front {
    pub points: Vec<Vector2<f64>>,
    pub cold_side: f64,
}

/// Where a point is relative to a front.
#[derive(Clone, Copy, Debug)]
pub struct FrontDistance {
    /// Distance from the front, metres: positive on the cold side.
    pub signed: f64,
    /// How far along the front (0 to 1) the nearest point is.
    pub along: f64,
    /// How far beyond either end of the front the point is, metres (0 alongside it).
    pub beyond: f64,
}

impl Front {
    /// A quadratic Bézier from `a` through control `b` to `c`, in `n` segments.
    fn curve(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>, n: usize, cold_side: f64) -> Front {
        let points = (0..=n).map(|k| {
            let t = k as f64 / n as f64;
            a * (1. - t) * (1. - t) + b * 2. * t * (1. - t) + c * t * t
        }).collect();
        Front { points, cold_side }
    }

    pub fn distance(&self, p: &Vector2<f64>) -> FrontDistance {
        let n = self.points.len() - 1;
        let mut best = (f64::INFINITY, 0., 0., 0.);
        for k in 0..n {
            let (a, b) = (self.points[k], self.points[k + 1]);
            let ab = b - a;
            let t = ((p - a).dot(&ab) / ab.norm_squared()).clamp(0., 1.);
            let q = a + ab * t;
            let d = (p - q).norm();
            if d < best.0 {
                let cross = ab.x * (p.y - q.y) - ab.y * (p.x - q.x);
                // Past either end: the distance beyond it, along the front's direction there.
                let beyond = if k == 0 && t <= 0. {
                    (-(p - a).dot(&ab.normalize())).max(0.)
                } else if k == n - 1 && t >= 1. {
                    (p - b).dot(&ab.normalize()).max(0.)
                } else {
                    0.
                };
                best = (d, cross.signum() * self.cold_side, (k as f64 + t) / n as f64, beyond);
            }
        }
        FrontDistance { signed: best.0 * best.1, along: best.2, beyond: best.3 }
    }
}

#[derive(Clone, Debug)]
pub struct System {
    pub kind: Kind,
    /// Unit vector from the planet's centre to the system's (geographic: north is +y).
    pub centre: Vector3<f64>,
    /// Its size, metres: a cyclone's fronts reach about this far from the low.
    pub radius: f64,
    /// Strength, 0 to 1.
    pub intensity: f64,
    /// A cyclone's life cycle: 0 a wave on the front, 0.5 mature, 1 fully occluded.
    pub stage: f64,
    /// A tropical cyclone's eye radius, metres.
    pub eye: f64,
    /// The frame's axes at the centre: east and poleward (rotated by the system's tilt).
    pub east: Vector3<f64>,
    pub poleward: Vector3<f64>,
    pub fronts: Vec<Front>,
}

impl System {
    /// A system at `centre`, its pattern turned by `tilt` radians (anticlockwise, seen from
    /// above, in its own frame).
    pub fn new(kind: Kind, centre: Vector3<f64>, radius: f64, intensity: f64, stage: f64, tilt: f64, eye: f64) -> System {
        let centre = centre.normalize();
        let east0 = Vector3::y().cross(&centre);
        let east0 = if east0.norm() > 1e-9 { east0.normalize() } else { Vector3::x() };
        let north0 = centre.cross(&east0);
        let poleward0 = if centre.y >= 0. { north0 } else { -north0 };
        // Turn anticlockwise in the (east, poleward) frame.
        let (s, c) = tilt.sin_cos();
        let east = east0 * c + poleward0 * s;
        let poleward = -east0 * s + poleward0 * c;
        let mut sys = System { kind, centre, radius, intensity: intensity.clamp(0., 1.), stage: stage.clamp(0., 1.), eye, east, poleward, fronts: vec![] };
        if kind == Kind::Cyclone {
            sys.fronts = sys.make_fronts();
        }
        sys
    }

    /// The warm, cold and (once it has begun) occluded fronts, in metres. The triple point
    /// moves out from the low as it occludes.
    fn make_fronts(&self) -> Vec<Front> {
        let l = self.radius;
        let s = self.stage;
        let v = |x: f64, y: f64| Vector2::new(x, y) * l;
        let t = v(0.4 * s, -0.3 * s);
        let warm = Front::curve(t, t + v(0.45, 0.04), t + v(0.95, -0.3), 24, 1.);
        let cold = Front::curve(t, t + v(0.1, -0.55), t + v(-0.5, -1.15), 24, -1.);
        let mut fronts = vec![warm, cold];
        if s > 0.02 {
            // Curling cyclonically round the north of the low on its way to the triple point.
            fronts.push(Front::curve(v(-0.05 * s, 0.1 * s), v(0.35 * s, 0.12 * s), t, 12, 1.));
        }
        fronts
    }

    /// The point `g` (a unit vector) in the system's frame, metres along the surface, by the
    /// azimuthal equidistant projection; None if it is too far away to matter.
    pub fn local(&self, g: &Vector3<f64>, planet_radius: f64) -> Option<Vector2<f64>> {
        let cos = g.dot(&self.centre).clamp(-1., 1.);
        let angle = cos.acos();
        let arc = angle * planet_radius;
        if arc > self.reach() {
            return None;
        }
        let t = g - self.centre * cos;
        let n = t.norm();
        if n < 1e-12 {
            return Some(Vector2::zeros());
        }
        let dir = t / n;
        Some(Vector2::new(dir.dot(&self.east), dir.dot(&self.poleward)) * arc)
    }

    /// How far from its centre the system has any effect, metres.
    pub fn reach(&self) -> f64 {
        match self.kind {
            // The cirrus far ahead of the warm front.
            Kind::Cyclone => self.radius * 2.2 + 1.5e6,
            Kind::Anticyclone => self.radius * 2.,
            Kind::Tropical => self.radius * 1.6,
        }
    }

    /// A vector in the frame (east, poleward) as a geographic vector.
    pub fn to_geo(&self, v: &Vector2<f64>) -> Vector3<f64> {
        self.east * v.x + self.poleward * v.y
    }

    /// The flow round the system near the ground, in its frame (m/s): cyclonic round a low
    /// (anticlockwise here), spiralling in; anticyclonic round a high, spiralling out.
    pub fn circulation(&self, p: &Vector2<f64>) -> Vector2<f64> {
        let r = p.norm();
        if r < 1. {
            return Vector2::zeros();
        }
        let (core, speed, turn) = match self.kind {
            Kind::Cyclone => (0.3 * self.radius, 18., 1.),
            Kind::Anticyclone => (0.5 * self.radius, 8., -1.),
            Kind::Tropical => (2. * self.eye, 50., 1.),
        };
        let v = self.intensity * speed * if r < core { r / core } else { (core / r).powf(0.6) * (-((r - core) / (2. * self.reach())).powi(2)).exp() };
        let tangent = Vector2::new(-p.y, p.x) / r * turn;
        let inflow = -p / r * turn;
        (tangent * 0.94 + inflow * 0.34) * v
    }
}

/// What the systems do to a column of air, added up.
#[derive(Clone, Copy, Debug, Default)]
pub struct Effects {
    /// Changes to the relative humidity (0 to 1), the ascent (+) or subsidence (-), and the
    /// instability.
    pub humidity: f64,
    pub ascent: f64,
    pub instability: f64,
    /// Changes to the height of the boundary layer's inversion, metres.
    pub inversion: f64,
    /// Low cloud: cover, and its form (0 stratus to 1 cumulus), weighted by cover.
    pub low: f64,
    pub low_form: f64,
    pub low_weight: f64,
    /// Frontal cloud in the middle of the troposphere: cover, and its base and top (metres,
    /// weighted by cover).
    pub mid: f64,
    pub mid_base: f64,
    pub mid_top: f64,
    pub mid_weight: f64,
    /// Cirrus cover.
    pub high: f64,
    /// Deep convection (cumulonimbus): cover.
    pub deep: f64,
    /// Stratiform rain from the systems' cloud, mm/h.
    pub rain: f64,
    /// How much of the cloud above the boundary layer is cleared (0 to 1): dry slots, eyes,
    /// and the subsiding air of highs.
    pub clear: f64,
    /// A tropical cyclone's eye: no deep cloud (1 in it).
    pub eye: f64,
}

impl Effects {
    /// Low cloud of `form` with `cover`: the cover is the most of any system's, the form
    /// averaged by cover.
    pub fn low_cloud(&mut self, cover: f64, form: f64) {
        if cover <= 0. {
            return;
        }
        self.low = self.low.max(cover);
        self.low_form += cover * form;
        self.low_weight += cover;
    }

    /// Frontal cloud from `base` to `top` (metres) with `cover`: the cover is the most of
    /// any system's, the heights averaged by cover.
    pub fn mid_cloud(&mut self, cover: f64, base: f64, top: f64) {
        if cover <= 0. {
            return;
        }
        self.mid = self.mid.max(cover);
        self.mid_base += cover * base;
        self.mid_top += cover * top.max(base + 300.);
        self.mid_weight += cover;
    }
}

/// The windows used throughout: 1 between `a` and `b`, ramping over `ramp` either side.
fn window(x: f64, a: f64, b: f64, ramp: f64) -> f64 {
    smoothstep(a - ramp, a + ramp, x) * (1. - smoothstep(b - ramp, b + ramp, x))
}

/// Shapes of the frontal surfaces, and how far ahead and behind their cloud reaches.
#[derive(Clone, Copy, Debug)]
pub struct FrontalParams {
    /// Slopes of the warm and cold frontal surfaces (rise over run).
    pub warm_slope: f64,
    pub cold_slope: f64,
}

impl Default for FrontalParams {
    fn default() -> FrontalParams {
        FrontalParams { warm_slope: 1. / 150., cold_slope: 1. / 60. }
    }
}

impl System {
    /// Add this system's effects at `p` (its frame, metres) to `e`, given the local
    /// tropopause (metres) and whether the surface is sea (0 to 1).
    pub fn effects(&self, p: &Vector2<f64>, e: &mut Effects, fp: &FrontalParams, tropopause: f64, sea: f64) {
        match self.kind {
            Kind::Cyclone => self.cyclone(p, e, fp, tropopause, sea),
            Kind::Anticyclone => self.anticyclone(p, e, sea),
            Kind::Tropical => self.tropical(p, e, tropopause),
        }
    }

    fn cyclone(&self, p: &Vector2<f64>, e: &mut Effects, fp: &FrontalParams, tropopause: f64, sea: f64) {
        let i = self.intensity;
        let l = self.radius;
        let r = p.norm() / l;
        let size = (l / 1.2e6).clamp(0.5, 1.6);
        // The whole system: a broad area of rising, moist air.
        let core = (-(r / 0.9).powi(2)).exp();
        e.ascent += 0.45 * i * core;
        e.humidity += 0.12 * i * core;
        let warm = self.fronts[0].distance(p);
        let cold = self.fronts[1].distance(p);
        // A front fades past its ends, and the cold front towards its tail.
        let fade = |d: &FrontDistance, reach: f64| (-(d.beyond / reach).powi(2)).exp();
        let warm_strength = i * fade(&warm, 250e3 * size) * (1. - 0.3 * self.stage);
        let cold_strength = i * fade(&cold, 300e3 * size) * (1. - 0.6 * smoothstep(0.5, 1., cold.along));

        // Ahead of the warm front: cloud above the frontal surface, its base rising ahead.
        let dw = warm.signed;
        if dw > -100e3 && dw < 1600e3 * size && warm_strength > 0. {
            let surface = dw.max(0.) * fp.warm_slope;
            let ahead = |a: f64, b: f64| window(dw, a * size, b * size, 60e3 * size);
            // Nimbostratus and altostratus: deep near the front, thinning ahead.
            let mid = warm_strength * ahead(-20e3, 700e3);
            let depth = 6500. - 4500. * smoothstep(0., 800e3 * size, dw);
            e.mid_cloud(mid, surface.max(600.), (surface + depth).min(tropopause - 1500.));
            // Cirrostratus, then cirrus far ahead.
            e.high = e.high.max(warm_strength * ahead(250e3, 1300e3) * (1. - 0.6 * smoothstep(600e3 * size, 1300e3 * size, dw)));
            // Steady rain in the last 300 km, with ragged stratus beneath.
            let rain = warm_strength * ahead(-30e3, 320e3);
            e.rain += 3. * rain;
            e.low_cloud(0.7 * rain, 0.1);
            e.humidity += 0.25 * warm_strength * ahead(-50e3, 600e3);
            e.ascent += 0.6 * mid;
        }

        let dc = cold.signed;
        // The cold front's line of cumulonimbus, in the warm air just ahead of it.
        let band = cold_strength * window(dc, -90e3 * size, 10e3, 25e3);
        e.deep = e.deep.max(0.9 * band);
        e.ascent += 0.8 * band;
        e.humidity += 0.2 * band;
        e.instability += 0.5 * band;
        // The warm conveyor belt: warm, moist air rising along the cold front, ahead of it,
        // and turning poleward over the warm front: the long band of cloud that trails from a
        // low on satellite pictures.
        let belt = cold_strength * window(dc, -380e3 * size, 0., 70e3) * smoothstep(-80e3, 60e3, -dw);
        if belt > 0. {
            e.mid_cloud(belt, 2000., (tropopause - 1500.).min(9000.));
            e.high = e.high.max(0.9 * belt);
            e.humidity += 0.2 * belt;
            e.ascent += 0.5 * belt;
            e.rain += 0.8 * belt;
        }
        if dc > 0. {
            // Behind it, rain on the frontal surface, then the dry slot, then showers.
            let frontal = cold_strength * window(dc, 0., 160e3 * size, 30e3);
            e.mid_cloud(frontal, (dc * fp.cold_slope).max(1200.), 5500.);
            e.rain += 2.5 * frontal;
            let slot = cold_strength * window(dc, 200e3 * size, 550e3 * size, 80e3) * (1. - smoothstep(1.2, 1.8, r));
            e.clear = e.clear.max(0.85 * slot);
            e.humidity -= 0.25 * slot;
            e.ascent -= 0.5 * slot;
            // Cold air over a warmer sea: open cells of shower cloud.
            let outbreak = cold_strength * smoothstep(400e3 * size, 700e3 * size, dc) * (1. - smoothstep(1.4, 2.2, r));
            e.instability += 0.5 * outbreak * (0.3 + 0.7 * sea);
            e.low_cloud(0.6 * outbreak * (0.4 + 0.6 * sea), 0.85);
            e.inversion += 600. * outbreak;
        }

        // The warm sector: on the warm side of both fronts, near the low.
        let sector = i * (1. - smoothstep(-150e3, -20e3, dw)) * (1. - smoothstep(-150e3, 20e3, dc)) * (1. - smoothstep(1., 1.6, r));
        if sector > 0. {
            e.low_cloud(0.75 * sector * (0.5 + 0.5 * sea), 0.35);
            e.humidity += 0.15 * sector;
            e.rain += 0.2 * sector * sea;
        }

        // The comma head: thick cloud along the occlusion and wrapped round the low, spiralling.
        let mut head = 0.;
        if let Some(occluded) = self.fronts.get(2) {
            let d = occluded.distance(p);
            head = i * self.stage.sqrt() * (-(d.signed / (250e3 * size)).powi(2)).exp() * fade(&d, 200e3 * size);
        }
        let angle = p.y.atan2(p.x);
        let spiral = 0.5 + 0.5 * (angle + 2.2 * (r.max(0.02)).ln()).cos();
        // Poleward and west of the low, fading on the equatorward side where the dry slot wraps in.
        let wrap = (1. - smoothstep(0.35, 0.65, r)) * smoothstep(-0.35, 0.1, p.y / l - 0.4 * p.x / l) * (0.55 + 0.45 * spiral);
        let comma = head.max(i * (0.3 + 0.7 * self.stage) * wrap);
        e.mid_cloud(comma, 1500., (tropopause - 2500.).min(8000.));
        e.high = e.high.max(0.8 * comma);
        e.rain += 2.5 * comma;
        e.ascent += 0.5 * comma;
        e.humidity += 0.2 * comma;
    }

    fn anticyclone(&self, p: &Vector2<f64>, e: &mut Effects, sea: f64) {
        let i = self.intensity;
        let r = p.norm() / self.radius;
        let g = (-(r * r)).exp();
        e.ascent -= 0.7 * i * g;
        e.humidity -= 0.12 * i * g;
        e.instability -= 0.4 * i * g;
        e.inversion -= 600. * i * g;
        e.clear = e.clear.max(0.7 * i * g);
        // Over the sea: stratocumulus on the eastern flank, trade cumulus on the western.
        let x = p.x / self.radius;
        let ring = (-((r - 0.75) / 0.4).powi(2)).exp() * sea * i;
        let sheet = ring * smoothstep(-0.1, 0.6, x);
        let trades = ring * (1. - smoothstep(-0.6, 0.1, x));
        e.low_cloud(0.75 * sheet, 0.4);
        e.low_cloud(0.45 * trades, 0.95);
        e.humidity += 0.1 * sheet;
    }

    fn tropical(&self, p: &Vector2<f64>, e: &mut Effects, tropopause: f64) {
        let i = self.intensity;
        let rm = p.norm();
        let r = rm / self.radius;
        let eye = self.eye;
        // Clear in the eye, but for low cloud swirling in it.
        let in_eye = 1. - smoothstep(0.6 * eye, eye, rm);
        e.eye = e.eye.max(in_eye);
        // The eyewall, and spiral rain bands winding in (anticlockwise, in this frame).
        let wall = smoothstep(0.7 * eye, eye, rm) * (1. - smoothstep(2. * eye, 3.5 * eye, rm));
        let angle = p.y.atan2(p.x);
        let pitch = 15f64.to_radians();
        let phase = angle + (rm.max(eye) / eye).ln() / pitch.tan();
        let arms = 0.5 + 0.5 * (2. * phase).cos();
        let bands = smoothstep(0.55, 0.85, arms) * smoothstep(2.5 * eye, 4. * eye, rm) * (1. - smoothstep(0.55, 1., r));
        e.deep = e.deep.max(i * wall.max(0.85 * bands));
        // The central dense overcast: the anvils of the eyewall and bands, spread out.
        let overcast = (1. - smoothstep(0.3, 0.65, r)) * (1. - in_eye);
        e.high = e.high.max(i * overcast);
        e.mid_cloud(i * overcast * 0.9, 2500., tropopause - 2000.);
        e.rain += i * (25. * wall + 8. * bands);
        let g = (-(r * r)).exp();
        e.humidity += 0.3 * i * g;
        e.ascent += 0.8 * i * g;
        e.instability += 0.4 * i * g;
        e.clear = e.clear.max(in_eye);
    }
}
