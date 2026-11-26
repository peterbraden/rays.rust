//! # Hotspots: shield volcanoes and their chains
//!
//! A hotspot is a plume of hot mantle rising beneath a plate, fixed while the plate moves over
//! it. Above it basalt erupts fast and runny enough to build a **shield volcano**: broad and
//! gentle (Mauna Loa's flanks slope 4 to 12°), nine kilometres from the sea floor to its summit
//! and a hundred across, the largest volcanoes on the Earth. Then the plate carries the shield
//! off the plume, and a new one grows behind it: a **chain**, its volcanoes older the further
//! they are from the hotspot, in the direction the plate moves (Wilson, "A possible origin of
//! the Hawaiian Islands", Canadian Journal of Physics 41, 1963; Morgan, "Convection plumes in
//! the lower mantle", Nature 230, 1971).
//!
//! ## The track
//!
//! The crust above the hotspot `a` million years ago has since been carried by its plate's
//! rotation over that time (`Hotspot::omega`, radians per million years, from the plate
//! tectonics, `tectonics::Crust::hotspots`; or drawn from the seed without them). Shields
//! form there every `SPACING` (55 km, as Hawaii's are spaced along the chain), give or take,
//! and up to 20 km either side of the line (Hawaii's young volcanoes stand in two parallel
//! lines, the Loa and Kea trends). The youngest is still beneath the sea, ahead of the hotspot
//! (as Kamaʻehuakanaloa, formerly Lōʻihi, is); the oldest kept are `OLDEST` (70 million years)
//! old.
//!
//! ## A shield (`Shield::shape`)
//!
//! - Only on ocean crust: beneath a continent the plume erupts as calderas and lava plains
//!   (Yellowstone, the Snake River Plain), not yet drawn.
//! - It grows over about a million years (`growth`), to stand `SUMMIT` above the sea (3.2 km
//!   times the square root of the hotspot's strength, varying 15% either way from volcano to
//!   volcano), whatever the depth of the floor beneath (as high above shallower floor, and so
//!   smaller); about 9 km above a deep sea floor (`DEEP`). Its base is `RADIUS_PER_HEIGHT` times
//!   as wide as it is high above the floor, and wider again for a stronger plume (by the
//!   square root of its strength).
//! - Its profile is convex, level at the summit and steepening down the flanks (the height
//!   falls as the distance to the power 1.7), with a broad apron of debris at its foot.
//! - Its eruptions feed from two or three **rift zones** running out from the summit, which
//!   stretch the shield into ridges along them (Kīlauea's East Rift Zone runs 125 km).
//! - A summit **caldera** two or three kilometres across, a couple of hundred metres deep (as
//!   Mokuʻāweoweo is), while it is young.
//! - Its weight and the plate's cooling **sink** it: two kilometres over its first few million
//!   years as the plate flexes under the load (Hawaii's Big Island sinks 2.6 mm a year now),
//!   then slowly, as the sea floor deepens with age (Detrick and Crough, "Island subsidence,
//!   hot spots, and lithospheric thinning", JGR 83, 1978): five and a half kilometres in all.
//!   Calibrated so that Kauaʻi, five million years old, stands at about 1.5 km, Midway, 28
//!   million years, has sunk to the sea, and the Emperor seamounts, 50 to 70, lie a kilometre
//!   or so beneath it.
//! - Above the sea, rain and waves wear it down: its height above the sea falls with a time
//!   constant of `EROSION` (6 million years), on top of what the landscape's own simulation
//!   erodes over its run. Beneath the sea it is no longer worn.
//! - In the tropics, coral keeps pace with the sinking: an island worn and sunk to the sea
//!   leaves a ring of reef at sea level about its old shoreline, around a lagoon, an **atoll**
//!   (Darwin's theory of 1842, confirmed by drilling Enewetak in 1952), if it was an island
//!   once (a shield that never rose from the deep sea leaves none). Beyond about 28° of
//!   latitude (the Darwin point) coral grows too slowly, and the shield drowns as a seamount.
//!
//! The planet adds the shields to its elevation (`Planet::elevation`), so they stand in every
//! landscape's initial topography and the whole planet's; their rock is basalt (the landscape
//! starts with it as its `lava`). The youngest erupt through the landscape's run, their lava
//! flowing down from the summit and the rift zones (`volcano::vents`).

use crate::geometry::Pcg32;
use crate::na::{Rotation3, Vector3};
use crate::procedural::nubis_clouds::smoothstep;
use std::collections::HashMap;

/// The depth of the sea floor a shield's `height` stands on, metres; on shallower floor it
/// stands as high, and so is smaller.
pub const DEEP: f64 = 5000.;
/// Mean spacing of shields along a track, metres.
pub const SPACING: f64 = 55e3;
/// The oldest shields kept, million years.
pub const OLDEST: f64 = 70.;
/// A grown shield's summit above the sea, metres, for a hotspot of strength 1 (growing as its
/// square root).
pub const SUMMIT: f64 = 3200.;
/// A full-grown shield's height above the sea floor, metres, for a hotspot of strength 1:
/// about Mauna Loa's.
pub const HEIGHT: f64 = DEEP + 1000. + SUMMIT;
/// A shield's base radius over its height.
pub const RADIUS_PER_HEIGHT: f64 = 8.;
/// Time constant of the wearing down of a shield's height above the sea, million years.
pub const EROSION: f64 = 6.;
/// Beyond this latitude, degrees, coral can't keep up with a sinking island (the Darwin point).
pub const DARWIN_POINT: f64 = 28.;
/// A tropical island worn and sunk below this, metres (the sea), is an atoll: its reef stands
/// about it, the volcano beneath its lagoon.
const ATOLL: f64 = 0.;
/// Depth of an atoll's lagoon, metres.
const LAGOON: f64 = 25.;

/// A hotspot: where it is (a unit vector), the angular velocity of the plate over it (radians
/// per million years), and its strength (1 for a Hawaii).
#[derive(Clone, Debug, PartialEq)]
pub struct Hotspot {
    pub at: Vector3<f64>,
    pub omega: Vector3<f64>,
    pub strength: f64,
}

impl Hotspot {
    /// The plate's speed over the hotspot, metres per million years, on a planet of `radius`.
    pub fn speed(&self, radius: f64) -> f64 {
        self.omega.cross(&self.at).norm() * radius
    }
}

/// `count` hotspots drawn from `seed` on a planet of `radius`, for a planet without plate
/// tectonics: plates moving over them at 30 to 100 km per million years, each its own way.
pub fn seeded(count: usize, seed: u64, radius: f64) -> Vec<Hotspot> {
    let mut rng = Pcg32::new(seed ^ 0x686f_7473, 1);
    (0..count).map(|_| {
        let at = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
        let east = any_tangent(&at);
        let north = at.cross(&east);
        let angle = rng.next_f64() * std::f64::consts::TAU;
        let along = east * angle.cos() + north * angle.sin();
        let speed = 30e3 + 70e3 * rng.next_f64();
        // ω × at = along × speed / R.
        Hotspot { at, omega: at.cross(&along) * speed / radius, strength: strength(&mut rng) }
    }).collect()
}

/// A hotspot's strength: from a half to one and a third of a Hawaii's.
fn strength(rng: &mut Pcg32) -> f64 { 0.5 + 0.8 * rng.next_f64() }

/// The hotspots of a plate tectonics simulation (`tectonics::Crust::hotspots`: each one's
/// direction and its plate's angular velocity), their strengths from `seed`.
pub fn of_plates(hotspots: &[(Vector3<f64>, Vector3<f64>)], seed: u64) -> Vec<Hotspot> {
    let mut rng = Pcg32::new(seed ^ 0x686f_7473, 2);
    hotspots.iter().map(|&(at, omega)| Hotspot { at, omega, strength: strength(&mut rng) }).collect()
}

/// A shield volcano of a hotspot's track.
#[derive(Clone, Debug)]
pub struct Shield {
    /// Its summit's direction from the planet's centre.
    pub dir: Vector3<f64>,
    /// Million years since it formed over the hotspot (negative: still growing beneath the
    /// sea ahead of it).
    pub age: f64,
    /// Its height above the sea floor when grown, metres.
    pub height: f64,
    /// Its base radius, metres.
    pub radius: f64,
    /// Its caldera's radius, metres.
    caldera: f64,
    /// Its rift zones' directions in its tangent plane, radians from `east`.
    rifts: Vec<f64>,
    east: Vector3<f64>,
    north: Vector3<f64>,
    /// For the reef's breaks into islets.
    phase: f64,
    /// Whether it stood above the sea when grown, as only an island can leave an atoll.
    emerged: bool,
}

/// How far a shield aged `age` (Myr) has grown, 0 to 1: over about a million years, half of it
/// before it surfaces.
pub fn growth(age: f64) -> f64 { smoothstep(-0.6, 0.6, age) }

/// How fast a shield aged `age` erupts, relative to its hotspot's supply: most in its main
/// shield-building stage, tailing off as the plate carries it from the plume.
pub fn activity(age: f64) -> f64 { smoothstep(-0.6, -0.1, age) * (1. - smoothstep(0.4, 1.0, age)) }

/// How far a shield aged `age` (Myr) has sunk, metres: under its own load, then as the plate
/// cools.
pub fn subsidence(age: f64) -> f64 {
    if age <= 0. { return 0.; }
    2000. * (1. - (-age / 1.5).exp()) + 3500. * (1. - (-age / 30.).exp())
}

/// How far a shield aged `age` (Myr) has sunk since the waves planed it off, metres: from
/// about 8 million years old (its height above the sea mostly worn away by then, `EROSION`),
/// so that Hawaii's are atolls from about 15 million years (French Frigate Shoals, at 12, is
/// one with a last pinnacle of the volcano standing in it), on to a kilometre and a half (the
/// Emperor seamounts' flat tops lie one to two kilometres down).
pub fn drowned(age: f64) -> f64 { 1500. * smoothstep(8., 70., age) }

impl Shield {
    /// Its rise above the surrounding sea floor at `d` (a unit vector), metres, on a planet of
    /// `radius`, before wear (negative where it has sunk the floor about it), and how much of
    /// it is the shield's own body there, 0 to 1 (the weight of its age where shields overlap).
    pub fn shape(&self, d: &Vector3<f64>, radius: f64) -> (f64, f64) {
        let r = radius * self.dir.angle(d);
        if r > 2.2 * self.radius { return (0., 0.); }
        // Its azimuth in the summit's tangent plane, stretched along its rifts.
        let t = d - self.dir * self.dir.dot(d);
        let theta = t.dot(&self.north).atan2(t.dot(&self.east));
        let stretch = 1. + 0.4 * self.rifts.iter().map(|&a| (theta - a).cos().max(0.).powi(6)).sum::<f64>();
        let x = r / (self.radius * stretch);
        let g = growth(self.age);
        let core = (1. - x.powf(1.7)).max(0.);
        let apron = 1. - smoothstep(0.6, 1.5, x);
        let mut h = self.height * g * (0.88 * core + 0.12 * apron);
        // The caldera, while it is young.
        if self.age < 2. {
            let depth = 200. * g * (1. - smoothstep(0.5, 2., self.age));
            h -= depth * (1. - smoothstep(0.8 * self.caldera, self.caldera, r));
        }
        // Sunk, with the plate about it.
        h -= subsidence(self.age) * (1. - smoothstep(0., 2.2, r / self.radius));
        (h, core)
    }

    /// Points along its rift zones, where its flank eruptions break out: from a tenth to six
    /// tenths of its radius from the summit, on a planet of `radius`.
    pub fn rift_points(&self, radius: f64) -> Vec<Vector3<f64>> {
        self.rifts.iter().flat_map(|&a| {
            let along = self.east * a.cos() + self.north * a.sin();
            (1..=6).map(move |k| (self.dir * radius + along * (0.1 * k as f64 * self.radius)).normalize())
        }).collect()
    }

    /// The reef about it at `d`, if it is an atoll: the elevation its reef, lagoon and fore-reef
    /// hold the ground at, at least (None away from an atoll). `summit` is the shield's worn
    /// summit's elevation, metres (below the sea, `ATOLL`, for an atoll).
    pub fn reef(&self, d: &Vector3<f64>, radius: f64, summit: f64) -> Option<f64> {
        let latitude = self.dir.y.clamp(-1., 1.).asin().to_degrees().abs();
        if !self.emerged || latitude > DARWIN_POINT || self.age < 3. || summit >= ATOLL { return None; }
        let r = radius * self.dir.angle(d);
        let t = d - self.dir * self.dir.dot(d);
        let theta = t.dot(&self.north).atan2(t.dot(&self.east));
        let stretch = 1. + 0.4 * self.rifts.iter().map(|&a| (theta - a).cos().max(0.).powi(6)).sum::<f64>();
        // Its old shoreline: about a quarter of the way out.
        let ring = 0.25 * self.radius * stretch;
        if r > ring + 4000. { return None; }
        // The reef's crest, a few hundred metres wide, awash in places and standing in islets
        // (motus) in others.
        let width = 400.;
        let islets = (3. * theta + self.phase).sin() + 0.6 * (7. * theta + 2. * self.phase).sin() + 0.3 * (17. * theta).sin();
        let crest = if islets > 0.2 { 2.5 } else { -1. };
        let off = (r - ring).abs();
        if off < width { return Some(crest); }
        if r < ring { return Some(-LAGOON); }
        // The fore-reef falls steeply to the shield beneath.
        Some(crest - 0.6 * (off - width))
    }
}

/// The shields of `hotspots` (each its direction and its plate's angular velocity, radians
/// per Myr) on a planet of `radius`, from `seed`; `continental(d)` says how much of the crust
/// at `d` is continent (0 to 1): they build shields only on ocean crust.
pub fn shields(hotspots: &[Hotspot], radius: f64, seed: u64, continental: &dyn Fn(&Vector3<f64>) -> f64) -> Vec<Shield> {
    let mut out = vec![];
    for (k, h) in hotspots.iter().enumerate() {
        let mut rng = Pcg32::new(seed ^ 0x7368_6965, k as u64);
        // A plate barely moving piles its shields up about the hotspot.
        let speed = h.speed(radius).max(20e3);
        let axis = h.omega.try_normalize(1e-15);
        let mut age = -0.5 + 0.6 * rng.next_f64();
        while age < OLDEST {
            let rotation = match axis {
                Some(a) => Rotation3::from_axis_angle(&crate::na::Unit::new_normalize(a), h.omega.norm() * age),
                None => Rotation3::identity(),
            };
            let on = rotation * h.at;
            // Across the track, up to 20 km either way.
            let across = h.omega.cross(&h.at).cross(&on).try_normalize(1e-12).unwrap_or_else(|| any_tangent(&on));
            let dir = (on * radius + across * 20e3 * (2. * rng.next_f64() - 1.)).normalize();
            // Only on ocean crust: beneath a continent the plume spreads in the thick crust and
            // erupts as calderas and lava plains (Yellowstone and the Snake River Plain), not
            // shields.
            // Its summit, grown, a few kilometres above the sea whatever its strength (as
            // Mauna Kea's 4.2 km, Teide's 3.7, Réunion's 3.1, Fogo's 2.8): a stronger plume
            // builds a broader shield more than a taller one.
            let height = DEEP + subsidence(0.8) + SUMMIT * h.strength.sqrt() * (0.85 + 0.3 * rng.next_f64());
            let oceanic = continental(&dir) < 0.3;
            let east = any_tangent(&dir);
            let north = dir.cross(&east);
            let rifts = {
                let first = rng.next_f64() * std::f64::consts::TAU;
                let n = 2 + (rng.next_f64() < 0.4) as usize;
                (0..n).map(|i| first + i as f64 * std::f64::consts::TAU / n as f64 + 0.4 * (rng.next_f64() - 0.5)).collect()
            };
            let (caldera, phase) = (1500. + 1500. * rng.next_f64(), rng.next_f64() * std::f64::consts::TAU);
            if oceanic {
                out.push(Shield { dir, age, height, radius: RADIUS_PER_HEIGHT * height * h.strength.sqrt(), caldera, rifts, east, north, phase, emerged: false });
            }
            age += SPACING / speed * (0.6 + 0.8 * rng.next_f64());
        }
    }
    out
}

/// Where shields overlap: a young one's lava buries the older ones' flanks, so they stand as
/// the highest of them (a soft maximum, `up` the sum of their rises' sixth powers), not their
/// sum; and the sea floor sunk about them (`down`, the deepest) fills in beneath them.
fn combine(up: f64, down: f64) -> f64 {
    let top = up.powf(1. / 6.);
    top + down * (-top / 1000.).exp()
}

fn any_tangent(d: &Vector3<f64>) -> Vector3<f64> {
    let a = if d.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
    a.cross(d).normalize()
}

/// The shields, indexed by where they reach, for adding them to the planet's elevation.
pub struct Shields {
    pub list: Vec<Shield>,
    radius: f64,
    /// Each one's summit's elevation, metres.
    summits: Vec<f64>,
    /// Lattice cells (a cube sphere `LATTICE` cells along a face's edge) and the shields
    /// reaching into each.
    cells: HashMap<usize, Vec<usize>>,
}

const LATTICE: usize = 64;

impl Shields {
    /// The shields of `list` on a planet of `radius`, whose elevation without them is
    /// `floor(d)`.
    pub fn new(list: Vec<Shield>, radius: f64, floor: &dyn Fn(&Vector3<f64>) -> f64) -> Shields {
        // A plume builds its volcano to about the same summit however deep the floor beneath:
        // `height` above a floor `DEEP` down, and less above a shallower one (the deep sea's
        // floor is deeper than that only at the trenches); too small to show, none.
        let list: Vec<Shield> = list.into_iter().filter_map(|mut s| {
            let height = s.height - (floor(&s.dir) + DEEP).max(0.);
            s.radius *= height.max(0.) / s.height;
            s.height = height;
            (s.height > 1000.).then_some(s)
        }).collect();
        let mut cells: HashMap<usize, Vec<usize>> = HashMap::new();
        let cell = std::f64::consts::FRAC_PI_2 * radius / LATTICE as f64;
        for (i, s) in list.iter().enumerate() {
            // Every lattice cell within its reach: points about it out to its reach, a cell apart.
            let reach = 2.2 * s.radius * 1.8;
            let steps = (reach / (0.5 * cell)).ceil() as i64;
            let mut seen = std::collections::BTreeSet::new();
            for a in -steps..=steps {
                for b in -steps..=steps {
                    let (x, y) = (a as f64 * 0.5 * cell, b as f64 * 0.5 * cell);
                    if x.hypot(y) > reach + cell { continue; }
                    let p = (s.dir * radius + s.east * x + s.north * y).normalize();
                    seen.insert(crate::terrain::domain::Domain::sphere_cell_at(LATTICE, &p));
                }
            }
            for c in seen { cells.entry(c).or_default().push(i); }
        }
        let mut shields = Shields { list, radius, summits: vec![], cells };
        // Whether each stood above the sea once grown: its summit then, a million years old,
        // over the floor there (`floor` with no shields).
        for s in shields.list.iter_mut() {
            s.emerged = floor(&s.dir) + s.height - subsidence(1.) > 0.;
        }
        // (Worn, as `elevation` wears them.)
        shields.summits = shields.list.iter().map(|s| {
            let (mut up, mut down) = (0f64, 0f64);
            for o in shields.near(&s.dir) {
                let h = o.shape(&s.dir, radius).0;
                if h > 0. { up += h.powi(6); } else { down = down.min(h); }
            }
            let e = floor(&s.dir) + combine(up, down);
            (if e > 0. && s.age > 0. { e * (-s.age / EROSION).exp() } else { e }) - drowned(s.age)
        }).collect();
        shields
    }

    /// The shields that may reach `d`.
    /// The shields that are atolls: tropical islands worn and sunk to the sea, a reef about
    /// their lagoon.
    pub fn atolls(&self) -> impl Iterator<Item = &Shield> {
        self.list.iter().zip(&self.summits).filter(|(s, z)| s.reef(&s.dir, self.radius, **z).is_some()).map(|(s, _)| s)
    }

    pub fn near(&self, d: &Vector3<f64>) -> impl Iterator<Item = &Shield> {
        let c = crate::terrain::domain::Domain::sphere_cell_at(LATTICE, d);
        self.cells.get(&c).into_iter().flatten().map(|&i| &self.list[i])
    }

    /// The elevation at `d` with the shields, given `floor`, what it is without them, metres;
    /// and how much of it is shield, 0 to 1 (whose rock is basalt).
    pub fn elevation(&self, d: &Vector3<f64>, floor: f64) -> (f64, f64) {
        let (mut most, mut age, mut body) = (0., 0., 0f64);
        let (mut up, mut down) = (0f64, 0f64);
        let mut atoll: Option<(&Shield, f64)> = None;
        let c = crate::terrain::domain::Domain::sphere_cell_at(LATTICE, d);
        for &i in self.cells.get(&c).into_iter().flatten() {
            let s = &self.list[i];
            let (h, w) = s.shape(d, self.radius);
            if h == 0. && w == 0. { continue; }
            if h > 0. { up += h.powi(6); } else { down = down.min(h); }
            body = body.max(w);
            if w > most { most = w; age = s.age; }
            let summit = self.summits[i];
            if atoll.is_none() && summit < ATOLL && s.reef(d, self.radius, summit).is_some() { atoll = Some((s, summit)); }
        }
        let mut e = floor + combine(up, down);
        // Worn down above the sea, by its age; and once planed off by the waves, sinking on
        // beneath it, a flat-topped guyot.
        if e > 0. && most > 0. && age > 0. {
            e *= 1. - most * (1. - (-age / EROSION).exp());
        }
        e -= most * drowned(age);
        if let Some((s, summit)) = atoll && let Some(r) = s.reef(d, self.radius, summit) {
            e = e.max(r);
        }
        (e, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const R: f64 = 6.371e6;

    fn hawaii() -> Hotspot {
        // A plate moving north-west at 80 km per Myr over a hotspot at 19°N.
        let lat = 19f64.to_radians();
        let at = Vector3::new(lat.cos(), lat.sin(), 0.);
        let north = Vector3::new(-lat.sin(), lat.cos(), 0.);
        let west = Vector3::new(0., 0., 1.);
        let along = (north + west).normalize();
        Hotspot { at, omega: at.cross(&along) * 80e3 / R, strength: 1. }
    }

    #[test]
    fn test_a_chain_ages_along_the_plates_motion() {
        let h = hawaii();
        let s = shields(&[h.clone()], R, 1, &|_| 0.);
        assert!(s.len() > 50, "{} shields", s.len());
        // Ages rise along the track, and each is further from the hotspot the older it is,
        // the way the plate moves.
        let velocity = h.omega.cross(&h.at);
        for w in s.windows(2) {
            assert!(w[1].age > w[0].age);
        }
        for t in s.iter().filter(|t| t.age > 1.) {
            let away = t.dir - h.at;
            assert!(away.dot(&velocity) > 0., "age {}", t.age);
            let distance = R * t.dir.angle(&h.at);
            assert!((distance - 80e3 * t.age).abs() < 30e3 + 0.05 * distance, "{} m at {} Myr", distance, t.age);
        }
    }

    #[test]
    fn test_shields_rise_sink_and_wear() {
        let floor = -5000.;
        let h = hawaii();
        let across = h.omega.cross(&h.at).cross(&h.at).normalize();
        let list = shields(&[h], R, 2, &|_| 0.);
        let field = Shields::new(list.clone(), R, &|_| floor);
        let summit = |s: &Shield| field.elevation(&(s.dir * R + s.east * 4000.).normalize(), floor).0;
        // The youngest full-grown ones stand kilometres above the sea, with gentle flanks.
        let young: Vec<&Shield> = list.iter().filter(|s| (0.6..1.2).contains(&s.age)).collect();
        assert!(!young.is_empty());
        for s in &young {
            let z = summit(s);
            assert!(z > 1500. && z < 6000., "a young shield's summit at {} m", z);
        }
        // Old ones have sunk beneath the sea.
        for (i, s) in list.iter().enumerate().filter(|(_, s)| s.age > 35.) {
            assert!(field.summits[i] < 0., "{} Myr old at {} m", s.age, field.summits[i]);
        }
        // Gentle: a young shield's flank, halfway down, across the chain, slopes under 15°.
        let s = young[0];
        let p = |x: f64| field.elevation(&(s.dir * R + across * x).normalize(), floor).0;
        let x = 0.5 * s.radius;
        let slope = ((p(x) - p(x + 1000.)) / 1000.).atan().to_degrees();
        assert!(slope > 1. && slope < 15., "{}°", slope);
    }

    #[test]
    fn test_sunken_islands_in_the_tropics_are_atolls() {
        let h = hawaii();
        let floor = -5000.;
        let list = shields(&[h], R, 3, &|_| 0.);
        let field = Shields::new(list.clone(), R, &|_| floor);
        let mut atolls = 0;
        for (i, s) in list.iter().enumerate().filter(|(_, s)| s.age > 10. && s.age < 45.) {
            if field.summits[i] >= ATOLL || s.dir.y.asin().to_degrees().abs() > DARWIN_POINT { continue; }
            // A lagoon at its middle, the reef's crest about it.
            let (middle, _) = field.elevation(&s.dir, floor);
            assert!(middle >= -LAGOON - 1. && middle < 0., "its middle at {}", middle);
            let crest = (0..72).map(|k| {
                let a = k as f64 * 5f64.to_radians();
                let rim = 0.25 * s.radius;
                (0..9).map(|j| field.elevation(&(s.dir * R + (s.east * a.cos() + s.north * a.sin()) * (rim * (0.8 + 0.05 * j as f64))).normalize(), floor).0)
                    .fold(f64::NEG_INFINITY, f64::max)
            }).fold(f64::NEG_INFINITY, f64::max);
            assert!(crest > 0., "the reef stands at {} m", crest);
            atolls += 1;
        }
        assert!(atolls > 0);
    }
}
