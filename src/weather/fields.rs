//! # The weather's clouds, as density fields over the planet
//!
//! Each family of cloud from the weather's maps (`Maps`) as a `planet::PlanetField`:
//!
//! - `Stratiform`: low and mid cloud, by the Nubis recipe (`procedural::nubis_clouds`: a
//!   height profile, Perlin-Worley shapes cut to the cover, eroded edges), but with the base,
//!   top, cover and form (stratiform to cumuliform) varying over the planet from the maps. The
//!   profile fills the cloud from base to top: a sheet with soft top and bottom for stratiform
//!   cloud, heaps whose height grows with the cover for cumuliform.
//! - `Convective`: cumulus congestus and cumulonimbus as individual towers. Cells are scattered
//!   over the planet (a jittered grid, `cell_size` apart); the maps' deep cover is the chance a
//!   cell is active, and the stronger it is the taller it grows. Towers below the tropopause are
//!   congestus; those reaching it spread into anvils downwind, with an overshooting dome above.
//!   Over clusters of storms the anvils merge into a shield (the maps' `anvil`).
//! - `Rain`: rain and snow from the stratiform cloud's base, and shafts beneath each
//!   cumulonimbus, evaporating as they fall through dry air.
//!
//! Every field bounds itself by texel of the maps, as `GlobalNubis` does: no cloud where a
//! texel's cover is zero, or above its highest top, or below its lowest base, so tracking
//! skips clear sky and the air beneath the clouds.

use crate::na::Vector3;
use crate::noise::{worley_hash, PerlinNoise};
use crate::planet::{to_sphere, CubeMap, Planet, PlanetField, SurfacePoint};
use crate::procedural::nubis_clouds::{lerp, remap, smoothstep, stretched, worley_fbm, WorleyTexture};
use crate::weather::{LayerMaps, Maps};
use std::sync::{Arc, OnceLock};

/// How a family of cloud looks: its extinction in the densest cloud (per metre), and the sizes
/// of its shapes and their eroded detail (metres), and how strongly the detail erodes (0 to 1).
#[derive(Clone, Copy, Debug)]
pub struct Look {
    pub max_density: f64,
    pub shape_size: f64,
    pub detail_size: f64,
    pub erosion: f64,
}

/// How a family's shapes move with time: carried round the planet's axis by the wind (a
/// solid rotation, `speed` m/s at the equator, so shapes keep their size and don't shear
/// apart however long it runs), and boiling: the noise rising through them at `rise` m/s.
#[derive(Clone, Copy, Debug)]
pub struct Drift {
    pub axis: Vector3<f64>,
    /// Radians turned, and metres risen, by now.
    pub angle: f64,
    pub rise: f64,
}

impl Drift {
    pub fn new(axis: Vector3<f64>, speed: f64, rise: f64, time: f64, radius: f64) -> Drift {
        Drift { axis: axis.normalize(), angle: speed * time / radius, rise: rise * time }
    }

    pub fn none() -> Drift {
        Drift { axis: Vector3::y(), angle: 0., rise: 0. }
    }

    /// Where something now at `v` was at the start.
    fn back(&self, v: &Vector3<f64>) -> Vector3<f64> {
        if self.angle == 0. { *v } else { crate::weather::rotate(v, &self.axis, -self.angle) }
    }

    /// Where something at `v` at the start is now.
    fn forth(&self, v: &Vector3<f64>) -> Vector3<f64> {
        if self.angle == 0. { *v } else { crate::weather::rotate(v, &self.axis, self.angle) }
    }

    /// The position in the noise that the shapes at `position` (from the planet's centre,
    /// with unit `up`) are cut from.
    fn noise_position(&self, position: &Vector3<f64>, up: &Vector3<f64>) -> Vector3<f64> {
        self.back(&(position - up * self.rise))
    }
}

/// The noise a family's shapes are cut from.
struct Shapes {
    perlin: PerlinNoise,
    seed: u32,
    cells: OnceLock<WorleyTexture>,
}

impl Shapes {
    fn new(seed: u64) -> Shapes {
        Shapes { perlin: PerlinNoise::shuffled(seed), seed: seed as u32, cells: OnceLock::new() }
    }

    fn cells(&self) -> &WorleyTexture {
        self.cells.get_or_init(|| WorleyTexture::new(self.seed))
    }

    /// The fraction (0 to 1) of the densest cloud at height `altitude` in a cloud from `base`
    /// to `top` covering `cover` of the sky, of `form`, at `position` (from the planet's
    /// centre).
    #[allow(clippy::too_many_arguments)]
    fn fraction(&self, look: &Look, altitude: f64, base: f64, top: f64, cover: f64, form: f64, position: &Vector3<f64>) -> f64 {
        if cover <= 0. || altitude <= base || altitude >= top {
            return 0.;
        }
        let h = (altitude - base) / (top - base);
        // Heaped clouds grow taller where they are stronger; sheets fill their layer.
        let reach = lerp(1., lerp(0.35, 1., cover.sqrt()), form);
        let hp = h / reach;
        if hp >= 1. {
            return 0.;
        }
        let profile = smoothstep(0., lerp(0.12, 0.08, form), hp) * (1. - smoothstep(lerp(0.65, 0.3, form), 1., hp));
        if profile <= 0. {
            return 0.;
        }
        let q = position / look.shape_size;
        let perlin = stretched(self.perlin.fbm(q.x, q.y, q.z, 4, 0.5, 2.));
        let perlin_worley = remap(perlin, worley_fbm(self.cells(), &q) - 1., 1.);
        // Sheets are smoother than heaps: less of the billowing cells.
        let n = lerp(perlin, perlin_worley, 0.45 + 0.55 * form);
        let shape = remap(n * profile, 0.75 * (1. - cover), 1.);
        if shape <= 0. {
            return 0.;
        }
        let d = worley_fbm(self.cells(), &(position / look.detail_size));
        let detail = lerp(1. - d, d, (hp * 5.).min(1.));
        remap(shape, detail * look.erosion, 1.)
    }
}

/// Which family of the weather's maps a stratiform field draws.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Family {
    Low,
    Mid,
}

/// Low or mid cloud.
pub struct Stratiform {
    maps: Arc<Maps>,
    family: Family,
    pub look: Look,
    pub drift: Drift,
    shapes: Shapes,
    ceiling: f64,
}

impl Stratiform {
    pub fn new(maps: Arc<Maps>, family: Family, look: Look, seed: u64) -> Stratiform {
        let ceiling = match family { Family::Low => maps.low.top.max(), Family::Mid => maps.mid.top.max() };
        Stratiform { maps, family, look, drift: Drift::none(), shapes: Shapes::new(seed), ceiling }
    }

    fn layer(&self) -> &LayerMaps {
        match self.family {
            Family::Low => &self.maps.low,
            Family::Mid => &self.maps.mid,
        }
    }

    /// The highest any cloud in the family reaches.
    pub fn highest(&self) -> f64 {
        self.ceiling
    }
}

/// The local majorant over a texel of `cover` between the spheres of `low` and `high`
/// altitudes: `m` between them if the cover isn't zero, else nothing to the texel's edge.
#[allow(clippy::too_many_arguments)]
/// Whether a ray at `at` along `d` has left a family for good: above `ceiling` (its highest
/// cloud anywhere) and heading outwards, its distance from the planet's centre only grows.
/// Without this, rays climbing out of the clouds (to the sun, the sky) stepped on through empty
/// texels, 20 km at a time, to the far side of the planet-sized bounds: most of a render's
/// time, in every family, empty or not.
fn gone(at: &SurfacePoint, d: &Vector3<f64>, ceiling: f64) -> bool {
    at.altitude >= ceiling && at.position.dot(d) >= 0.
}

fn column_bound(at: &SurfacePoint, d: &Vector3<f64>, planet: &Planet, map: &CubeMap, cover: f64, low: f64, high: f64, m: f64) -> (f64, f64) {
    let ahead = at.position + d * 0.01;
    let (face, i, j) = map.texel(&ahead.normalize());
    let edge = map.cube.exit(&at.position, d, face, i, j);
    if cover <= 0. || high <= low {
        return (0., edge);
    }
    let altitude = ahead.norm() - planet.radius;
    let to_low = to_sphere(&at.position, d, planet.radius + low.max(-planet.radius * 0.5));
    let to_high = to_sphere(&at.position, d, planet.radius + high);
    let inside = altitude >= low && altitude <= high;
    (if inside { m } else { 0. }, edge.min(to_low).min(to_high))
}

impl PlanetField for Stratiform {
    fn density(&self, at: &SurfacePoint) -> f64 {
        let l = self.layer();
        let t = l.top.locate(&at.up);
        let top = l.top.at_texel(&t);
        if at.altitude >= top {
            return 0.;
        }
        let base = l.base.at_texel(&t);
        if at.altitude <= base {
            return 0.;
        }
        let cover = l.cover.at_texel(&t);
        let position = self.drift.noise_position(&at.position, &at.up);
        self.look.max_density * self.shapes.fraction(&self.look, at.altitude, base, top, cover, l.form.at_texel(&t), &position)
    }

    fn majorant(&self) -> f64 {
        self.look.max_density
    }

    fn majorant_from(&self, at: &SurfacePoint, _p: &Vector3<f64>, d: &Vector3<f64>, planet: &Planet) -> (f64, f64) {
        if gone(at, d, self.ceiling) {
            return (0., f64::INFINITY);
        }
        let l = self.layer();
        let (face, i, j) = l.cover.texel(&(at.position + d * 0.01).normalize());
        column_bound(at, d, planet, &l.cover, l.cover.texel_max(face, i, j), -l.base_low.texel_max(face, i, j),
            l.top.texel_max(face, i, j), self.look.max_density)
    }
}

/// Deep convection: towers of cumulus congestus and cumulonimbus, their anvils, and the
/// shields of anvils over clusters of storms.
pub struct Convective {
    maps: Arc<Maps>,
    pub look: Look,
    /// The spacing of the cells, metres.
    pub cell_size: f64,
    /// Overshooting tops: how far above the anvil a strong storm's dome rises, metres.
    pub overshoot: f64,
    /// The depth of an anvil, metres.
    pub anvil_depth: f64,
    pub radius: f64,
    /// The highest top of any storm's column, before its overshooting dome (cached).
    top_max: f64,
    /// How the cells move with the wind aloft, and their lumps boil up their sides.
    pub drift: Drift,
    /// How long a cell lives, from cumulus through cumulonimbus to its anvil's last, seconds;
    /// and the time now.
    pub lifetime: f64,
    pub time: f64,
    seed: u32,
    shapes: Shapes,
}

/// An active convective cell near a point.
pub struct Tower {
    /// The horizontal offset of the point from the tower's axis (a scene vector, metres), and
    /// its length.
    pub offset: Vector3<f64>,
    pub distance: f64,
    /// Strength (0 to 1), base, top, radius (metres), and whether it has reached the
    /// tropopause (a cumulonimbus, with an anvil).
    pub strength: f64,
    pub base: f64,
    pub top: f64,
    pub radius: f64,
    pub mature: bool,
    /// The wind aloft at the tower (unit, horizontal), which its anvil spreads along.
    pub wind: Vector3<f64>,
    /// Its life: how far through it (0 to 1), how much of its column is left (fading as it
    /// rains out), and how far its anvil has spread and how much of it is left.
    pub age: f64,
    pub column: f64,
    pub anvil: f64,
    pub anvil_left: f64,
}

impl Convective {
    pub fn new(maps: Arc<Maps>, look: Look, cell_size: f64, radius: f64, seed: u64) -> Convective {
        let top_max = maps.deep_top_reach.max();
        Convective { maps, look, cell_size, overshoot: 800., anvil_depth: 1800., radius, top_max, drift: Drift::none(), lifetime: 5400., time: 0.,
                     seed: seed as u32, shapes: Shapes::new(seed.wrapping_add(5)) }
    }

    /// The highest any of it reaches.
    pub fn highest(&self) -> f64 {
        self.top_max + self.overshoot
    }

    /// Calls `f` with each active tower whose cell is near the point above `up` (scene).
    pub fn towers(&self, up: &Vector3<f64>, f: impl FnMut(&Tower)) {
        self.towers_within(up, self.cell_size, f);
    }

    /// As `towers`, for cells whose centre is within `reach` of the point (at most a cell
    /// apart). Returns how far the point is from any cell not looked at.
    fn towers_within(&self, up: &Vector3<f64>, reach: f64, mut f: impl FnMut(&Tower)) -> f64 {
        let s = self.cell_size;
        // In the frame drifting with the cells.
        let up = &self.drift.back(up);
        let p = up * self.radius;
        let c = (p / s).map(|v| v.floor() as i64);
        // Cells beyond the 3 x 3 x 3 block about the point's are at least this far away.
        let block = (0..3).map(|k| (p[k] - (c[k] - 1) as f64 * s).min((c[k] + 2) as f64 * s - p[k])).fold(f64::INFINITY, f64::min);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let (ix, iy, iz) = (c.x + dx, c.y + dy, c.z + dz);
                    // Each cell lives through cycles, starting at a phase of its own; each cycle
                    // a new storm, somewhere else in the cell.
                    let phase = worley_hash(ix as u32, iy as u32, iz as u32, self.seed.wrapping_add(6));
                    let x = self.time / self.lifetime + phase;
                    let (age, cycle) = (x - x.floor(), x.floor() as i64 as u32);
                    let h = |k: u32| worley_hash(ix as u32, iy as u32, iz as u32, self.seed.wrapping_add(k).wrapping_add(cycle.wrapping_mul(7919)));
                    let feature = Vector3::new(ix as f64 + h(0), iy as f64 + h(1), iz as f64 + h(2)) * s;
                    let fdir_drifting = feature.normalize();
                    let fp = fdir_drifting * self.radius;
                    // Where the storm is now, to read the weather there.
                    let fdir = self.drift.forth(&fdir_drifting);
                    let offset = p - fp;
                    let offset = offset - up * up.dot(&offset);
                    let distance = offset.norm();
                    if distance > reach {
                        continue;
                    }
                    // Active with the chance the deep cover gives, stronger the more it exceeds it.
                    let m = &self.maps;
                    let texel = m.deep.cover.locate(&fdir);
                    let cover = m.deep.cover.at_texel(&texel);
                    let chance = h(3);
                    if chance >= cover {
                        continue;
                    }
                    let strength = ((cover - chance) / cover.max(1e-6)).sqrt();
                    let base = m.deep.base.at_texel(&texel);
                    let ceiling = m.deep.top.at_texel(&texel);
                    let growth = h(4).max(strength).powf(0.7);
                    let reaches = growth > 0.8;
                    let height = if reaches { ceiling - base } else { (ceiling - base) * (0.3 + 0.5 * growth) };
                    // Rising through the first 40% of its life, reaching its height, spreading
                    // its anvil, then raining out: the column fades, then the anvil.
                    let rising = smoothstep(0., 0.4, age);
                    let top = base + height * rising;
                    let mature = reaches && age > 0.3;
                    let radius = s * (0.09 + 0.1 * h(5)) * (0.7 + 0.3 * growth) * (0.5 + 0.5 * rising);
                    let column = 1. - smoothstep(0.7, 0.9, age);
                    let anvil = smoothstep(0.3, 0.65, age);
                    let anvil_left = 1. - smoothstep(0.85, 1., age);
                    let wind = Vector3::new(m.wind[0].at_texel(&texel), m.wind[1].at_texel(&texel), m.wind[2].at_texel(&texel));
                    // Into the drifting frame, where the offsets are.
                    let wind = self.drift.back(&wind);
                    let wind = wind - fdir_drifting * fdir_drifting.dot(&wind);
                    let wind = if wind.norm() > 1e-6 { wind.normalize() } else { crate::weather::tangents(&fdir_drifting).0 };
                    f(&Tower { offset, distance, strength, base, top, radius, mature, wind, age, column, anvil, anvil_left });
                }
            }
        }
        block
    }

    /// How far out from its axis a tower's cloud can reach (with its lumps and its anvil).
    fn tower_reach(&self, t: &Tower) -> f64 {
        let column = 1.3 * t.radius;
        if t.mature { column.max(1.95 * self.anvil_radius(t)) } else { column }
    }

    /// The most any tower reaches.
    fn max_reach(&self) -> f64 {
        (1.3 * 0.19 * self.cell_size).max(1.95 * 0.5 * self.cell_size)
    }

    /// The anvil's radius for a tower: limited so that it stays within a cell of its own.
    fn anvil_radius(&self, t: &Tower) -> f64 {
        (4. * t.radius).min(0.5 * self.cell_size)
    }

    /// How much of the densest cloud there is at `at` from towers and their anvils.
    fn tower_fraction(&self, at: &SurfacePoint) -> f64 {
        let mut best = 0f64;
        // Lumps on the towers' sides: cauliflower.
        let mut lumps = None;
        self.towers(&at.up, |t| {
            let a = at.altitude;
            let over = if t.mature { self.overshoot } else { 0. };
            if a <= t.base || a >= t.top + over {
                return;
            }
            let lump = *lumps.get_or_insert_with(|| {
                let q = self.drift.noise_position(&at.position, &at.up) / self.look.shape_size;
                let perlin = stretched(self.shapes.perlin.fbm(q.x, q.y, q.z, 3, 0.5, 2.));
                remap(perlin, worley_fbm(self.shapes.cells(), &q) - 1., 1.)
            });
            let mut f = 0f64;
            // The tower: a column with a rounded top, narrowing a little as it rises.
            let hz = ((a - t.base) / (t.top - t.base)).min(1.);
            let width = if t.mature { t.radius * (1. - 0.2 * hz) } else { t.radius * (1. - hz.powi(4)).max(0.).sqrt() };
            let edge = t.radius * 0.35;
            let d = t.distance + edge * 1.6 * (0.5 - lump);
            f = f.max(smoothstep(0., edge, width - d) * smoothstep(t.base, t.base + 150., a) * t.column);
            if t.mature {
                // The anvil, spread downwind under the tropopause, and the overshooting dome.
                let ra = self.anvil_radius(t) * (0.3 + 0.7 * t.anvil);
                let low = t.top - self.anvil_depth;
                let spread = smoothstep(low, t.top, a);
                let shifted = t.offset - t.wind * (0.45 * ra * spread);
                let along = shifted.dot(&t.wind);
                let across = (shifted - t.wind * along).norm();
                let da = ((along / (1.3 * ra)).powi(2) + (across / ra).powi(2)).sqrt() * (1. / (0.35 + 0.65 * spread));
                let vertical = smoothstep(low, low + 0.4 * self.anvil_depth, a) * (1. - smoothstep(t.top - 120., t.top + 60., a));
                f = f.max((1. - smoothstep(0.7, 1., da + 0.25 * (0.5 - lump))) * vertical * t.anvil_left);
                if a > t.top - 100. {
                    let dome = t.radius * 0.6 * (1. - ((a - t.top) / over).powi(2)).max(0.).sqrt();
                    f = f.max(smoothstep(0., edge, dome - d) * t.column);
                }
            }
            best = best.max(f * (0.4 + 0.6 * t.strength));
        });
        if best <= 0. {
            return 0.;
        }
        // Erode the edges.
        let dd = worley_fbm(self.shapes.cells(), &(self.drift.noise_position(&at.position, &at.up) / self.look.detail_size));
        remap(best, (1. - dd) * self.look.erosion, 1.)
    }

    /// The shield of anvils over clusters of storms: a sheet under the tropopause.
    fn shield_fraction(&self, at: &SurfacePoint) -> f64 {
        let m = &self.maps;
        let cover = m.anvil.at(&at.up);
        if cover <= 0. {
            return 0.;
        }
        let top = m.deep.top.at(&at.up);
        let base = top - self.anvil_depth * 1.2;
        let look = Look { shape_size: self.look.shape_size * 3., ..self.look };
        let position = Drift { rise: 0., ..self.drift }.noise_position(&at.position, &at.up);
        self.shapes.fraction(&look, at.altitude, base, top, cover, 0.1, &position)
    }
}

impl PlanetField for Convective {
    fn density(&self, at: &SurfacePoint) -> f64 {
        let m = &self.maps;
        let t = m.deep_reach.locate(&at.up);
        if at.altitude > m.deep_top_reach.at_texel(&t) + self.overshoot || at.altitude < -m.deep_base_low.at_texel(&t) {
            return 0.;
        }
        let towers = if m.deep_reach.at_texel(&t) > 0. { self.tower_fraction(at) } else { 0. };
        self.look.max_density * towers.max(self.shield_fraction(at))
    }

    fn majorant(&self) -> f64 {
        self.look.max_density
    }

    fn majorant_from(&self, at: &SurfacePoint, _p: &Vector3<f64>, d: &Vector3<f64>, planet: &Planet) -> (f64, f64) {
        if gone(at, d, self.highest()) {
            return (0., f64::INFINITY);
        }
        let m = &self.maps;
        let (face, i, j) = m.deep_reach.texel(&(at.position + d * 0.01).normalize());
        let towers = m.deep_reach.texel_max(face, i, j);
        let anvil = m.anvil.texel_max(face, i, j);
        let top = m.deep_top_reach.texel_max(face, i, j) + self.overshoot;
        let shield_low = -m.deep_top_low.texel_max(face, i, j) - self.anvil_depth * 1.2;
        let low = if towers > 0. { -m.deep_base_low.texel_max(face, i, j) } else { shield_low };
        let (majorant, length) = column_bound(at, d, planet, &m.deep_reach, towers.max(anvil), low, top, self.look.max_density);
        if majorant <= 0. {
            return (majorant, length);
        }
        // Within the texel's column, the anvils' shield fills a band under the tropopause...
        let altitude = (at.position + d * 0.01).norm() - planet.radius;
        let shield_high = m.deep_top_reach.texel_max(face, i, j);
        if anvil > 0. && altitude >= shield_low && altitude <= shield_high {
            return (majorant, length);
        }
        let mut clear = length;
        if anvil > 0. {
            clear = clear.min(to_sphere(&at.position, d, planet.radius + shield_low)).min(to_sphere(&at.position, d, planet.radius + shield_high));
        }
        if towers <= 0. {
            return (0., clear);
        }
        // ...and the towers are few and far between: clear air until the ray could reach one.
        // Moving a distance L changes the point's height and horizontal offsets by at most L.
        let up = at.position / at.position.norm();
        let mut inside = false;
        let block = self.towers_within(&up, f64::INFINITY, |t| {
            // The column (with its lumps and dome) up to its top, and the anvil only near it.
            let over = if t.mature { self.overshoot } else { 0. };
            let vertical = (t.base - altitude).max(altitude - (t.top + over)).max(0.);
            let horizontal = (t.distance - 1.3 * t.radius).max(0.);
            let mut gap = vertical.max(horizontal);
            if t.mature {
                let low = t.top - self.anvil_depth;
                let vertical = (low - altitude).max(altitude - t.top).max(0.);
                gap = gap.min(vertical.max((t.distance - self.tower_reach(t)).max(0.)));
            }
            if gap <= 0. {
                inside = true;
            }
            clear = clear.min(gap);
        });
        if inside {
            // Somewhere near this storm: the full bound, but only for a while, as it may soon
            // be clear of it.
            return (majorant, length.min(1000.));
        }
        clear = clear.min(block - self.max_reach());
        if clear < 50. {
            return (majorant, length.min(200.));
        }
        (0., clear)
    }
}

/// Rain and snow: falling from the stratiform cloud's base, and in shafts beneath
/// cumulonimbus.
pub struct Rain {
    /// The time now, and how fast the streaks fall (m/s).
    pub time: f64,
    pub fall_speed: f64,
    maps: Arc<Maps>,
    convective: Option<Arc<Convective>>,
    /// The heaviest rain under a cumulonimbus, mm/h.
    pub shower_rate: f64,
    /// How much the shafts are streaked (0 to 1), and the streaks' size (metres).
    pub streakiness: f64,
    pub streak_size: f64,
    noise: PerlinNoise,
    max: f64,
    ceiling: f64,
}

impl Rain {
    pub fn new(maps: Arc<Maps>, convective: Option<Arc<Convective>>, shower_rate: f64, seed: u64) -> Rain {
        let showers = if convective.is_some() { crate::procedural::precipitation::rain_extinction(shower_rate) } else { 0. };
        let max = maps.rain.max().max(showers);
        let showers = if convective.is_some() { maps.deep_base_high.max() } else { 0. };
        let ceiling = maps.rain_base.max().max(showers);
        Rain { time: 0., fall_speed: 6., maps, convective, shower_rate, streakiness: 0.5, streak_size: 600., noise: PerlinNoise::shuffled(seed), max, ceiling }
    }

    /// The highest it falls from.
    pub fn highest(&self) -> f64 {
        self.ceiling
    }

    fn streaks(&self, at: &SurfacePoint) -> f64 {
        if self.streakiness <= 0. {
            return 1.;
        }
        // Streaks run down: noise stretched vertically.
        let q = (at.position - at.up * at.altitude) / self.streak_size;
        let n = self.noise.fbm(q.x, q.y, q.z + (at.altitude + self.fall_speed * self.time) / (self.streak_size * 10.), 3, 0.5, 2.);
        1. - self.streakiness + self.streakiness * 2. * n
    }
}

impl PlanetField for Rain {
    fn density(&self, at: &SurfacePoint) -> f64 {
        let m = &self.maps;
        let mut sigma = 0f64;
        let rain = m.rain.at(&at.up);
        if rain > 0. {
            let base = m.rain_base.at(&at.up);
            if at.altitude < base {
                let fall = m.rain_fall.at(&at.up).max(1.);
                sigma = rain * (-(base - at.altitude) / fall).exp() * (1. - smoothstep(base - 200., base, at.altitude));
            }
        }
        if let (Some(c), true) = (&self.convective, m.deep_reach.at(&at.up) > 0.) {
            let fall = m.rain_fall.at(&at.up).max(1.);
            c.towers(&at.up, |t| {
                if !(t.top - t.base > 4000. && at.altitude < t.base) {
                    return;
                }
                let core = 1. - smoothstep(t.radius * 0.4, t.radius * 1.1, t.distance);
                if core <= 0. {
                    return;
                }
                // It rains once the storm is grown, until it has rained itself out.
                let life = smoothstep(0.3, 0.45, t.age) * (1. - smoothstep(0.8, 0.95, t.age));
                let rate = self.shower_rate * t.strength.powf(1.5) * core * life;
                let s = crate::procedural::precipitation::rain_extinction(rate) * (-(t.base - at.altitude) / fall).exp();
                sigma = sigma.max(s);
            });
        }
        if sigma <= 0. {
            return 0.;
        }
        (sigma * self.streaks(at)).min(self.max)
    }

    fn majorant(&self) -> f64 {
        self.max
    }

    fn majorant_from(&self, at: &SurfacePoint, _p: &Vector3<f64>, d: &Vector3<f64>, planet: &Planet) -> (f64, f64) {
        if gone(at, d, self.ceiling) {
            return (0., f64::INFINITY);
        }
        let m = &self.maps;
        let (face, i, j) = m.rain.texel(&(at.position + d * 0.01).normalize());
        let mut cover = m.rain.texel_max(face, i, j);
        let mut top = m.rain_base.texel_max(face, i, j);
        if self.convective.is_some() && m.deep_reach.texel_max(face, i, j) > 0. {
            cover = cover.max(1.);
            top = top.max(m.deep_base_high.texel_max(face, i, j));
        }
        if cover <= 0. {
            top = -1.;
        }
        column_bound(at, d, planet, &m.rain, cover, -1000., top, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::{Weather, WeatherParams};

    #[test]
    fn test_clouds_where_the_maps_say() {
        let mut p = WeatherParams::new(7);
        p.map_size = 64;
        let w = Weather::new(p, None);
        let maps = w.maps();
        let planet = Planet { radius: w.params.radius };
        let low = Stratiform::new(maps.clone(), Family::Low, Look { max_density: 0.05, shape_size: 1200., detail_size: 220., erosion: 0.4 }, 1);
        let mid = Stratiform::new(maps.clone(), Family::Mid, Look { max_density: 0.03, shape_size: 3000., detail_size: 400., erosion: 0.35 }, 2);
        let mut rng = crate::geometry::Pcg32::new(3, 0);
        let (mut cloudy, mut tried) = ([0; 2], [0; 2]);
        for _ in 0..20000 {
            let up = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            for (k, (f, l)) in [(&low, &maps.low), (&mid, &maps.mid)].into_iter().enumerate() {
                if l.cover.at(&up) < 0.6 { continue; }
                let a = l.base.at(&up) + rng.next_f64() * (l.top.at(&up) - l.base.at(&up)) * 0.6;
                let at = planet.surface_point(&(planet.center() + up * (planet.radius + a)));
                tried[k] += 1;
                if f.density(&at) > 0. { cloudy[k] += 1; }
                // And the bound holds there.
                let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
                let (m, len) = f.majorant_from(&at, &at.position, &d, &planet);
                assert!(len > 0.);
                for k in 0..20 {
                    let q = planet.surface_point(&(planet.center() + at.position + d * (len.min(50_000.) * k as f64 / 20.)));
                    assert!(f.density(&q) <= m + 1e-12, "{} > {} at step {} of {}", f.density(&q), m, k, len);
                }
            }
        }
        println!("low {}/{} mid {}/{}", cloudy[0], tried[0], cloudy[1], tried[1]);
        let _ = &maps;
        assert!(cloudy[0] as f64 > 0.3 * tried[0] as f64 && cloudy[1] as f64 > 0.3 * tried[1] as f64);
    }

    #[test]
    fn test_convective_bounds_hold() {
        // Storms everywhere in the tropics, so there are plenty of towers to test against.
        let mut p = WeatherParams::new(8);
        p.map_size = 128;
        p.instability = 3.;
        p.humidity = 1.3;
        let w = Weather::new(p, None);
        let maps = w.maps();
        let planet = Planet { radius: w.params.radius };
        let c = Convective::new(maps.clone(), Look { max_density: 0.06, shape_size: 1500., detail_size: 300., erosion: 0.35 }, 20000., planet.radius, 4);
        let rain = Rain::new(maps.clone(), Some(Arc::new(Convective::new(maps.clone(), c.look, 20000., planet.radius, 4))), 40., 5);
        let mut rng = crate::geometry::Pcg32::new(5, 0);
        let (mut cloud, mut skipped, mut tested) = (0, 0, 0);
        while tested < 1500 {
            let up = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            if maps.deep.cover.at(&up) < 0.3 { continue; }
            tested += 1;
            let at = planet.surface_point(&(planet.center() + up * (planet.radius + rng.next_f64() * 16000.)));
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            for f in [&c as &dyn PlanetField, &rain] {
                let (m, len) = f.majorant_from(&at, &at.position, &d, &planet);
                assert!(len > 0.);
                if m == 0. { skipped += 1; }
                for k in 0..40 {
                    let q = planet.surface_point(&(planet.center() + at.position + d * (len.min(30_000.) * k as f64 / 40.)));
                    let v = f.density(&q);
                    if v > 0. { cloud += 1; }
                    assert!(v <= m + 1e-12, "{} > {} at step {} of {}", v, m, k, len);
                }
            }
        }
        println!("cloud {} skipped {}", cloud, skipped);
        assert!(cloud > 100 && skipped > 300, "uninformative: cloud {} skipped {}", cloud, skipped);
    }

    #[test]
    fn test_storms_grow_and_move_with_time() {
        let mut p = WeatherParams::new(8);
        p.map_size = 64;
        p.instability = 3.;
        p.humidity = 1.3;
        let w = Weather::new(p, None);
        let maps = w.maps();
        let planet = Planet { radius: w.params.radius };
        let look = Look { max_density: 0.06, shape_size: 1500., detail_size: 300., erosion: 0.35 };
        let field = |t: f64| {
            let mut c = Convective::new(maps.clone(), look, 20000., planet.radius, 4);
            c.time = t;
            c.drift = Drift::new(Vector3::y(), 10., 2., t, planet.radius);
            c
        };
        // A storm's top rises through the first part of its life: the same storms (in the same
        // cycle of their cells, their ages 600 s on) are taller ten minutes later.
        let tops = |up: &Vector3<f64>, t: f64| { let mut v = vec![]; field(t).towers(up, |tw| v.push((tw.age, tw.top - tw.base))); v };
        let mut compared = 0;
        for k in 0..20000 {
            let up = crate::geometry::point_on_unit_sphere((k as f64 * 0.618034) % 1., (k as f64 * 0.414214) % 1.);
            if maps.deep.cover.at(&up) < 0.4 { continue; }
            let (now, later) = (tops(&up, 0.), tops(&up, 600.));
            for (a0, h0) in &now {
                for (a1, h1) in &later {
                    if (a1 - a0 - 600. / 5400.).abs() < 1e-9 && *a1 < 0.38 {
                        assert!(h1 > h0, "{} {} -> {} {}", a0, h0, a1, h1);
                        compared += 1;
                    }
                }
            }
            if compared > 20 { break; }
        }
        assert!(compared > 0, "no growing storms found");
        // The density changes smoothly over a few seconds, and the bounds hold at any time.
        let mut rng = crate::geometry::Pcg32::new(6, 0);
        let (a, b) = (field(7000.), field(7005.));
        let (mut diff, mut total) = (0., 0.);
        for _ in 0..3000 {
            let u = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            if maps.deep.cover.at(&u) < 0.3 { continue; }
            let at = planet.surface_point(&(planet.center() + u * (planet.radius + 1000. + rng.next_f64() * 12000.)));
            let (x, y) = (a.density(&at), b.density(&at));
            diff += (x - y).abs();
            total += x;
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = a.majorant_from(&at, &at.position, &d, &planet);
            for k in 0..20 {
                let q = planet.surface_point(&(planet.center() + at.position + d * (len.min(30_000.) * k as f64 / 20.)));
                assert!(a.density(&q) <= m + 1e-12);
            }
        }
        assert!(total > 0. && diff < 0.2 * total, "{} of {}", diff, total);
    }
}
