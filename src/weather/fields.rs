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
    pub(crate) fn forth(&self, v: &Vector3<f64>) -> Vector3<f64> {
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
    /// Cumuliform cloud as cauliflower heaps (`Heaps`) rather than noise; None for noise alone.
    pub heaps: Option<Heaps>,
}

/// # Heaps of cumulus
///
/// Where the low cloud is cumuliform, each cloud is a cauliflower (`procedural::cauliflower`):
/// a tree of spheres, domes on domes, with a crisp edge and a flat base. Clouds sit on cells a
/// `cell` apart over the planet (a jittered grid, as the storms' are), each active with the
/// local cover as its chance, as deep as the layer there and about as wide, and drifting with
/// the layer. Where the cloud turns stratiform they fade into the noise's sheets.
pub struct Heaps {
    pub tree: crate::procedural::cauliflower::Cauliflower,
    /// Spacing of the cells (m), and the cloud's edge: the depth over which its density rises.
    pub cell: f64,
    pub edge: f64,
    /// How high the body's spheres' centres stand above the base, as a fraction of their
    /// radius: low, so the base cuts them broad and flat, as a cumulus's is.
    pub lift: f64,
    /// How much larger the body's middle spheres are than its rim's (`cauliflower::body`): a
    /// dome, not a flat-topped box.
    pub dome: f64,
    /// How far clouds lean downwind per metre of height (the wind's shear).
    pub lean: f64,
    /// How skewed the clouds' depths are: d = 0.25 + 0.75 u^skew of the layer's (u uniform), so
    /// many small and a few large, as in real fields of cumulus.
    pub skew: f64,
    /// Billows and wisps (`cauliflower::Surface`).
    pub surface: crate::procedural::cauliflower::Surface,
    pub radius: f64,
    pub seed: u32,
    /// The time now, and how long each cloud lives (s): born as wisps, it fills out into a heap,
    /// then frays and evaporates, and another is born elsewhere in its cell (`Heap::maturity`).
    pub time: f64,
    pub lifetime: f64,
}

impl Heaps {
    pub fn new(radius: f64, seed: u64) -> Heaps {
        Heaps {
            tree: crate::procedural::cauliflower::Cauliflower::cumulus(), cell: 2000., edge: 15., lift: 0.35, dome: 0.8, lean: 0.25, skew: 2.,
            surface: crate::procedural::cauliflower::Surface::new(seed.wrapping_add(77)), radius, seed: seed as u32,
            time: 0., lifetime: 2400.,
        }
    }

    fn margin(&self) -> f64 {
        self.surface.margin()
    }

    /// The deepest any cloud can be, as a fraction of a cell.
    fn deepest(&self) -> f64 {
        0.15 * (1. + self.dome) * (self.lift + self.tree.extent(1.))
    }

    /// The farthest a cloud reaches from its cell's centre, horizontally (leaning included).
    fn max_reach(&self) -> f64 {
        0.45 * self.cell + 0.15 * (1. + self.dome) * self.cell * self.tree.extent(1.) + self.lean * self.deepest() * self.cell
    }
}

/// One cloud of the heaps, near a point: its axis (a unit vector, in the drifting frame), the
/// point's horizontal distance from it, the radius of its body's spheres and of the disc they
/// stand on, its base and highest top (altitudes), and how far it reaches from its axis.
struct Heap {
    axis: Vector3<f64>,
    distance: f64,
    r0: f64,
    width: f64,
    base: f64,
    top: f64,
    reach: f64,
    id: u64,
    hashes: [f64; 2],
    /// How deep it is relative to the deepest the layer allows (0 to 1), and the way it leans
    /// (downwind, in the drifting frame).
    tall: f64,
    downwind: Vector3<f64>,
    /// How far through its life it is (0 to 1).
    age: f64,
}

impl Heap {
    /// How far it has formed (`cauliflower::Surface::condensed`): wisps as it is born, a heap by
    /// a third of its life, fraying from 0.7 and gone at the end.
    fn maturity(&self) -> f64 {
        smoothstep(0., 0.3, self.age).min(1. - smoothstep(0.7, 1., self.age))
    }

    /// How much of it has formed at `altitude`: rising from the condensation level as it is born
    /// (`cauliflower::rising`); a dying cloud keeps its height as it frays.
    fn grown_to(&self, altitude: f64) -> f64 {
        let reached = crate::procedural::cauliflower::rising(smoothstep(0., 0.35, self.age)) * (self.top - self.base);
        1. - smoothstep(0.8 * reached, reached, altitude - self.base)
    }

    /// The body's spheres, over a flat disc: more of them the wider it is.
    fn roots(&self, h: &Heaps) -> Vec<crate::procedural::cauliflower::Sphere> {
        crate::procedural::cauliflower::body(&(self.axis * (h.radius + self.base)), &self.axis, self.r0, self.width, h.lift, h.dome, self.tall, self.id, self.hashes[0])
    }

    /// Its sphere tree, grown once and kept for the thread (`Cauliflower::grow`): a ray looks
    /// the same few clouds up thousands of times. Keyed by everything the roots depend on, so a
    /// cloud whose base moves with time is grown afresh.
    fn grown(&self, h: &Heaps) -> std::rc::Rc<crate::procedural::cauliflower::Grown> {
        use std::collections::HashMap;
        use std::rc::Rc;
        thread_local! {
            static GROWN: std::cell::RefCell<HashMap<[u64; 4], Rc<crate::procedural::cauliflower::Grown>>> = std::cell::RefCell::new(HashMap::new());
        }
        // About 6000 spheres (a quarter of a megabyte) each at most: forget them all now and then.
        const KEPT: usize = 96;
        let key = [self.id ^ (h.seed as u64).rotate_left(7), self.base.to_bits(), self.r0.to_bits(), self.width.to_bits()];
        GROWN.with(|g| {
            if let Some(t) = g.borrow().get(&key) {
                return t.clone();
            }
            let t = Rc::new(h.tree.grow(&self.roots(h), &self.axis));
            let mut g = g.borrow_mut();
            if g.len() >= KEPT {
                g.clear();
            }
            g.insert(key, t.clone());
            t
        })
    }
}

impl Stratiform {
    pub fn new(maps: Arc<Maps>, family: Family, look: Look, seed: u64) -> Stratiform {
        let ceiling = match family { Family::Low => maps.low.top.max(), Family::Mid => maps.mid.top.max() };
        Stratiform { maps, family, look, drift: Drift::none(), shapes: Shapes::new(seed), ceiling, heaps: None }
    }

    /// Calls `f` with each cloud of the heaps whose cell is near the point above `up` (a unit
    /// vector in the frame drifting with the layer), stopping if it returns true; returns how far
    /// the point is from the reach of any cloud not reported (in cells not looked at, or too far
    /// off to reach it).
    fn heaps_near(&self, h: &Heaps, up: &Vector3<f64>, mut f: impl FnMut(&Heap) -> bool) -> f64 {
        let l = self.layer();
        let s = h.cell;
        let surface = up * h.radius;
        let c = (surface / s).map(|v| v.floor() as i64);
        let block = (0..3).map(|k| (surface[k] - (c[k] - 1) as f64 * s).min((c[k] + 2) as f64 * s - surface[k])).fold(f64::INFINITY, f64::min);
        let ext = h.tree.extent(1.);
        let mut beyond = block - h.max_reach();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let (ix, iy, iz) = (c.x + dx, c.y + dy, c.z + dz);
                    // Each cell's clouds live in turn, from a phase of its own; each a new cloud,
                    // somewhere else in the cell.
                    let phase = crate::noise::worley_hash(ix as u32, iy as u32, iz as u32, h.seed.wrapping_add(50));
                    let x = h.time / h.lifetime + phase;
                    let (age, cycle) = (x - x.floor(), x.floor() as i64 as u32);
                    let hash = |k: u32| crate::noise::worley_hash(ix as u32, iy as u32, iz as u32, h.seed.wrapping_add(k).wrapping_add(cycle.wrapping_mul(7919)));
                    let axis = (Vector3::new(ix as f64 + hash(0), iy as f64 + hash(1), iz as f64 + hash(2)) * s).normalize();
                    if !owns(axis * h.radius, s, (ix, iy, iz)) {
                        continue;
                    }
                    let offset = surface - axis * h.radius;
                    let distance = (offset - up * up.dot(&offset)).norm();
                    if distance > h.max_reach() {
                        beyond = beyond.min(distance - h.max_reach());
                        continue;
                    }
                    let texel = l.cover.locate(&self.drift.forth(&axis));
                    let (cover, form) = (l.cover.at_texel(&texel), l.form.at_texel(&texel));
                    // Active more often than the cover: a cloud covers less than its cell.
                    if hash(3) >= (2.5 * cover).min(1.) * smoothstep(0.35, 0.65, form) {
                        continue;
                    }
                    let (base, top) = (l.base.at_texel(&texel), l.top.at_texel(&texel));
                    // Many small, a few large.
                    let tall = hash(4).powf(h.skew);
                    let depth = (top - base) * (0.25 + 0.75 * tall);
                    // The body: spheres over a flat disc `width` in radius, sized so that their
                    // turrets (to `extent` times their radius) top out at about the cloud's depth;
                    // small clouds narrow, larger ones wider.
                    let r0 = (depth / ((1. + h.dome) * (h.lift + ext))).clamp(30., 0.15 * s);
                    let largest = r0 * (1. + h.dome);
                    let width = (s * (0.12 + 0.33 * (0.25 + 0.75 * tall).powf(0.7) * (0.7 + 0.3 * hash(5)))).clamp(r0, 0.45 * s);
                    let top_of = base + largest * (h.lift + ext);
                    // Leaning downwind with height: the wind aloft, in the drifting frame, level.
                    let m = &self.maps;
                    let wind = Vector3::new(m.wind[0].at_texel(&texel), m.wind[1].at_texel(&texel), m.wind[2].at_texel(&texel));
                    let wind = self.drift.back(&wind);
                    let wind = wind - axis * axis.dot(&wind);
                    let downwind = if wind.norm() > 1e-6 { wind.normalize() } else { Vector3::zeros() };
                    let heap = Heap {
                        axis, distance, r0, width, base, reach: width + largest * ext + h.lean * (top_of - base),
                        top: top_of,
                        id: ((ix as u64) << 42) ^ ((iy as u64) << 21) ^ (iz as u64) ^ ((h.seed as u64) << 13) ^ ((cycle as u64) << 7),
                        hashes: [hash(6), hash(7)], tall, downwind, age,
                    };
                    if f(&heap) {
                        return beyond;
                    }
                }
            }
        }
        beyond
    }

    /// How much of the densest cloud (0 to 1) the cauliflower heaps put at `at`, whose layer's
    /// base is `base` there.
    fn heap_fraction(&self, h: &Heaps, at: &SurfacePoint, base: f64) -> f64 {
        let p = self.drift.back(&at.position);
        let up = p.normalize();
        let margin = h.margin();
        // Frayed into wisps: the point looked up, carried through vector noise.
        let q = h.surface.frayed(&p);
        let mut best = 0f64;
        let mut billows = None;
        self.heaps_near(h, &up, |heap| {
            if heap.distance > heap.reach + margin || at.altitude > heap.top + margin {
                return false;
            }
            let maturity = heap.maturity();
            if maturity <= 0. {
                return false;
            }
            // Deep enough for its edge, softer the younger (or older) it is.
            let soft = 1. - maturity;
            let enough = h.edge * (1. + 24. * soft * soft);
            // Leaning downwind with height: look up where the cloud stood that much upwind.
            let lean = heap.downwind * (h.lean * (at.altitude - heap.base).max(0.));
            let depth = heap.grown(h).depth(&h.tree, &(q - lean), enough);
            if depth <= -margin {
                return false;
            }
            // Billows pushing the surface outwards (never in: the spheres are the core), then
            // the edge.
            let b = if depth < enough { *billows.get_or_insert_with(|| h.surface.billowing(&q)) } else { 0. };
            best = best.max(h.surface.condensed(depth + b, h.edge, maturity, &q) * heap.grown_to(at.altitude));
            best >= 1.
        });
        // And the flat base, where the air saturates.
        best * smoothstep(base, base + 0.3 * h.edge, at.altitude)
    }

    /// Within a texel of nothing but heaps: how far a ray at `at` can go before it could reach
    /// one (0 if it is at one), at most `limit`. Moving a distance L changes the point's height
    /// and its horizontal distance from any cloud's axis by at most L.
    fn heap_clearance(&self, h: &Heaps, at: &SurfacePoint, limit: f64) -> f64 {
        let up = self.drift.back(&at.up);
        let mut clear = limit;
        let beyond = self.heaps_near(h, &up, |heap| {
            // The body's spheres reach 0.2 of their radius below the base (the base cuts them flat
            // where the point is, but neighbouring columns' bases differ).
            let margin = h.margin();
            let vertical = (heap.base - (1. - h.lift) * heap.r0 * (1. + h.dome) - margin - at.altitude).max(at.altitude - heap.top - margin).max(0.);
            let horizontal = (heap.distance - heap.reach - margin).max(0.);
            clear = clear.min(vertical.max(horizontal));
            clear <= 0.
        });
        clear.min(beyond - h.margin()).max(0.)
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

/// Whether the cell (`ix`, `iy`, `iz`) of a lattice `s` apart holds `point`, a feature's
/// projection onto the planet's surface. Features are jittered points in the 3D cells, then
/// projected onto the surface; one counts only if its projection stays in its own cell. Then any
/// feature within a distance D of a point on the surface lies in a cell within D of it, so
/// searching the cells about the point finds every feature near it. (Otherwise a feature
/// from a cell two above the point's, inside the lattice, could project beside it unfound:
/// clouds cut off at cell boundaries, and bounds that missed them.)
fn owns(point: Vector3<f64>, s: f64, (ix, iy, iz): (i64, i64, i64)) -> bool {
    let c = (point / s).map(|v| v.floor() as i64);
    c.x == ix && c.y == iy && c.z == iz
}

/// Whether a ray at `at` along `d` has left a family for good: above `ceiling` (its highest
/// cloud anywhere) and heading outwards, its distance from the planet's centre only grows.
/// Without this, rays climbing out of the clouds (to the sun, the sky) stepped on through empty
/// texels, 20 km at a time, to the far side of the planet-sized bounds: most of a render's
/// time, in every family, empty or not.
fn gone(at: &SurfacePoint, d: &Vector3<f64>, ceiling: f64) -> bool {
    at.altitude >= ceiling && at.position.dot(d) >= 0.
}

/// The local majorant over a texel of `cover` between the spheres of `low` and `high`
/// altitudes: `m` between them if the cover isn't zero, else nothing to the texel's edge.
#[allow(clippy::too_many_arguments)]
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
        let form = l.form.at_texel(&t);
        let position = self.drift.noise_position(&at.position, &at.up);
        // Heaps where the cloud is cumuliform, noise where it's stratiform, mixed between.
        let heaped = match &self.heaps {
            Some(_) if cover > 0. => smoothstep(0.35, 0.65, form),
            _ => 0.,
        };
        let noise = if heaped < 1. { self.shapes.fraction(&self.look, at.altitude, base, top, cover, form, &position) } else { 0. };
        let heap = match &self.heaps {
            Some(h) if heaped > 0. => self.heap_fraction(h, at, base) * (1. - smoothstep(top - 0.3 * h.edge, top, at.altitude)),
            _ => 0.,
        };
        self.look.max_density * (noise * (1. - heaped) + heap * heaped)
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
        let (majorant, length) = column_bound(at, d, planet, &l.cover, l.cover.texel_max(face, i, j), -l.base_low.texel_max(face, i, j),
            l.top.texel_max(face, i, j), self.look.max_density);
        // Over a texel that is all heaps, clear air until the ray could reach one.
        match &self.heaps {
            Some(h) if majorant > 0. && -l.form_low.texel_max(face, i, j) >= 0.65 => {
                let clear = self.heap_clearance(h, at, length);
                if clear >= 20. { (0., clear) } else { (majorant, length.min(300.)) }
            },
            _ => (majorant, length),
        }
    }
}

/// The density of a cumulonimbus's frozen top, and of its anvil near it, as fractions of its
/// liquid column's. Ice water content is highest in the updraft (a gram or more per cubic metre)
/// and falls as the outflow spreads, so the frozen top stays bright and the anvil thins
/// outwards, to the shield's `cauliflower::ICE`.
const GLACIATED: f64 = 0.6;
const ANVIL: f64 = 0.35;

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
    /// The towers' thermals as cauliflower (`cauliflower::tower`), their crisp `edge` (m) and
    /// finer surface.
    pub tree: crate::procedural::cauliflower::Cauliflower,
    pub surface: crate::procedural::cauliflower::Surface,
    pub edge: f64,
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
    /// The top it will reach.
    pub full_top: f64,
    pub mature: bool,
    /// The wind aloft at the tower (unit, horizontal), which its anvil spreads along.
    pub wind: Vector3<f64>,
    /// Its life: how far through it (0 to 1), how much of its column is left (fading as it
    /// rains out), and how far its anvil has spread and how much of it is left.
    pub age: f64,
    pub column: f64,
    pub anvil: f64,
    pub anvil_left: f64,
    /// Its axis (a unit vector, in the drifting frame), id and phase, for its thermals.
    pub axis: Vector3<f64>,
    pub id: u64,
    pub phase: f64,
}

impl Tower {
    /// How far it has formed (`cauliflower::Surface::condensed`): wisps as it starts, filling
    /// out over the first fifth of its life; fraying and thinning as its column rains out.
    pub fn maturity(&self) -> f64 {
        smoothstep(0., 0.2, self.age).min(self.column)
    }

    /// Where its top freezes, if it has become a cumulonimbus: the upper third of its column.
    pub fn glaciation(&self) -> Option<f64> {
        self.mature.then(|| self.base + 0.67 * (self.top - self.base))
    }

    /// Its thermals' sphere trees, grown once per thread (as `Heap::grown`).
    fn grown(&self, c: &Convective) -> std::rc::Rc<crate::procedural::cauliflower::Grown> {
        use crate::procedural::cauliflower::{glaciated_levels, tower};
        use std::collections::HashMap;
        use std::rc::Rc;
        thread_local! {
            static GROWN: std::cell::RefCell<HashMap<[u64; 4], Rc<crate::procedural::cauliflower::Grown>>> = std::cell::RefCell::new(HashMap::new());
        }
        // About 25000 spheres (a megabyte) each.
        const KEPT: usize = 16;
        let key = [self.id, self.base.to_bits(), self.top.to_bits(), self.radius.to_bits() ^ self.mature as u64];
        GROWN.with(|g| {
            if let Some(t) = g.borrow().get(&key) {
                return t.clone();
            }
            let roots = tower(&(self.axis * (c.radius + self.base)), &self.axis, self.radius, self.top - self.base, self.full_top - self.base, 0.35, &c.tree, self.id, self.phase);
            let glaciation = self.glaciation();
            let r = c.radius;
            let t = Rc::new(c.tree.grow_capped(&roots, &self.axis, |p| glaciated_levels(&c.tree, glaciation, p.norm() - r)));
            let mut g = g.borrow_mut();
            if g.len() >= KEPT {
                g.clear();
            }
            g.insert(key, t.clone());
            t
        })
    }
}

impl Convective {
    pub fn new(maps: Arc<Maps>, look: Look, cell_size: f64, radius: f64, seed: u64) -> Convective {
        let top_max = maps.deep_top_reach.max();
        Convective { maps, look, cell_size, overshoot: 800., anvil_depth: 1800., radius, top_max, drift: Drift::none(), lifetime: 5400., time: 0.,
                     seed: seed as u32, shapes: Shapes::new(seed.wrapping_add(5)),
                     tree: crate::procedural::cauliflower::Cauliflower::cumulus(),
                     surface: crate::procedural::cauliflower::Surface::new(seed.wrapping_add(91)), edge: 15. }
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
                    if !owns(fp, s, (ix, iy, iz)) {
                        continue;
                    }
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
                    // Cumulonimbus columns are 5 to 15 km across.
                    let radius = s * (0.13 + 0.12 * h(5)) * (0.7 + 0.3 * growth) * (0.5 + 0.5 * rising);
                    let column = 1. - smoothstep(0.7, 0.9, age);
                    let anvil = smoothstep(0.3, 0.65, age);
                    let anvil_left = 1. - smoothstep(0.85, 1., age);
                    let wind = Vector3::new(m.wind[0].at_texel(&texel), m.wind[1].at_texel(&texel), m.wind[2].at_texel(&texel));
                    // Into the drifting frame, where the offsets are.
                    let wind = self.drift.back(&wind);
                    let wind = wind - fdir_drifting * fdir_drifting.dot(&wind);
                    let wind = if wind.norm() > 1e-6 { wind.normalize() } else { crate::weather::tangents(&fdir_drifting).0 };
                    let id = ((ix as u64) << 42) ^ ((iy as u64) << 21) ^ (iz as u64) ^ ((cycle as u64) << 7) ^ ((self.seed as u64) << 13);
                    f(&Tower { offset, distance, strength, base, top, radius, full_top: base + height, mature, wind, age, column, anvil, anvil_left,
                               axis: fdir_drifting, id, phase: h(7) });
                }
            }
        }
        block
    }

    /// How far out from its axis a tower's cloud can reach (with its lumps and its anvil).
    fn tower_reach(&self, t: &Tower) -> f64 {
        let column = self.column_reach(t.radius);
        if t.mature { column.max(1.95 * self.anvil_radius(t)) } else { column }
    }

    /// How far a column of `radius` reaches from its axis: its thermals (`cauliflower::tower`,
    /// within 1.25 of its radius) and their surface.
    fn column_reach(&self, radius: f64) -> f64 {
        1.25 * radius + self.surface.margin()
    }

    /// The most any tower reaches.
    fn max_reach(&self) -> f64 {
        self.column_reach(0.25 * self.cell_size).max(1.95 * 0.5 * self.cell_size)
    }

    /// The anvil's radius for a tower: limited so that it stays within a cell of its own.
    fn anvil_radius(&self, t: &Tower) -> f64 {
        (4. * t.radius).min(0.5 * self.cell_size)
    }

    /// How much of the densest cloud there is at `at` from towers and their anvils.
    fn tower_fraction(&self, at: &SurfacePoint) -> f64 {
        // The anvils and domes (ice, from noise), the outflow's cones (frozen, from noise), and
        // the thermals, crisp.
        let (mut anvil, mut cone, mut crisp) = (0f64, 0f64, 0f64);
        // The ice's texture: fibrous, drawn out horizontally (as the shield's).
        let mut fibres = None;
        self.towers(&at.up, |t| {
            let a = at.altitude;
            let over = if t.mature { self.overshoot } else { 0. };
            if a <= t.base || a >= t.top + over {
                return;
            }
            let lump = *fibres.get_or_insert_with(|| self.fibres(at));
            let strength = 0.4 + 0.6 * t.strength;
            // The tower: its thermals (`cauliflower::tower`), forming and fraying as it lives.
            if t.distance < self.column_reach(t.radius) && a < t.top + self.surface.margin() {
                crisp = crisp.max(self.thermals(t, at) * strength);
            }
            if t.mature {
                let (f, c) = self.anvil(t, a, lump);
                anvil = anvil.max(f * strength);
                cone = cone.max(c * strength);
            }
        });
        if anvil <= 0. && cone <= 0. {
            return crisp;
        }
        // Ice is thinner than the liquid cloud below (`cauliflower::ICE`), the outflow still fed
        // by the updraft thicker than the anvil; smooth and fibrous, not billowing. (Eroded by
        // cellular noise as the liquid cloud is, a fading anvil broke into puffs, like a field of
        // cumulus; real ones thin into cirrus.)
        let texture = 0.7 + 0.3 * fibres.unwrap_or(0.5);
        (anvil * ANVIL).max(cone * GLACIATED) * texture
    }

    /// Ice's fibrous texture at `at` (0 to 1): noise drawn out horizontally, five times longer
    /// than tall, as the shear aloft streaks it.
    fn fibres(&self, at: &SurfacePoint) -> f64 {
        let position = Drift { rise: 0., ..self.drift }.noise_position(&at.position, &at.up);
        let vertical = at.up * at.up.dot(&position);
        let f = (position + vertical * 5.) / (self.look.detail_size * 2.);
        stretched(self.shapes.perlin.fbm(f.x, f.y, f.z, 3, 0.5, 2.))
    }

    /// A cumulonimbus's anvil at altitude `a` (its fraction of the densest ice), and the cone of
    /// its outflow beneath it; `lump` roughens their edges.
    ///
    /// The storm's updraft spreads out under the tropopause and the wind aloft carries it away:
    /// an anvil reaches only a little upwind of the column but streams far downwind, widening
    /// as it goes (a fan), thickest over the storm and thinning downwind, its top flat at the
    /// tropopause (the WMO's incus). Beneath it the frozen top of the column widens up into it,
    /// a cone of outflow, rather than the anvil standing on a stalk.
    fn anvil(&self, t: &Tower, a: f64, lump: f64) -> (f64, f64) {
        // Only inwards, so the reach still bounds it.
        let rough = 0.12 * (1. - lump).clamp(0., 1.);
        let reach = self.anvil_radius(t);
        // Spreading as the storm matures.
        let grown = 0.25 + 0.75 * t.anvil;
        let half = (reach * grown).max(1.1 * t.radius);
        let downwind = 1.6 * reach * grown;
        let upwind = 0.6 * t.radius + 0.15 * half;
        let along = t.offset.dot(&t.wind);
        let across = (t.offset - t.wind * along).norm();
        let s = (along / downwind).clamp(0., 1.);
        let width = 1.1 * t.radius + (half - 1.1 * t.radius) * s.sqrt();
        let lengthwise = if along < 0. { -along / upwind } else { along / downwind };
        let d = (lengthwise.powi(4) + (across / width).powi(4)).powf(0.25);
        let horizontal = 1. - smoothstep(0.75, 1., d + rough);
        let depth = self.anvil_depth * (1. - 0.6 * s);
        let bottom = t.top - depth;
        // A flat base (where the outflow's ice sublimates into the dry air below) and top.
        let vertical = smoothstep(bottom, bottom + 0.12 * depth, a) * (1. - smoothstep(t.top - 120., t.top + 60., a));
        let mut f = horizontal * vertical * t.anvil_left;
        // The overshooting dome, where the updraft punches through the tropopause.
        if a > t.top - 100. {
            let edge = t.radius * 0.35;
            let dome = t.radius * 0.6 * (1. - ((a - t.top) / self.overshoot.max(1.)).powi(2)).max(0.).sqrt();
            f = f.max(smoothstep(0., edge, dome - t.distance - edge * 1.6 * (0.5 - lump)) * t.column);
        }
        // The cone, from where the column freezes up into the anvil's base over the storm.
        let mut c = 0.;
        let (from, to) = (self.outflow_from(t), t.top - 0.7 * self.anvil_depth);
        if a > from && a < t.top {
            let z = smoothstep(from, to, a);
            let rc = t.radius * (0.9 + 1.2 * z * z);
            c = smoothstep(0., 0.25 * rc, rc - t.distance * (1. + rough)) * smoothstep(0., 0.15, t.column) * t.anvil_left;
        }
        (f, c)
    }

    /// Where a cumulonimbus's outflow cone starts: where its column freezes.
    fn outflow_from(&self, t: &Tower) -> f64 {
        t.glaciation().unwrap_or(t.top).min(t.top - self.anvil_depth)
    }

    /// How much of the densest cloud a tower's thermals put at `at`.
    fn thermals(&self, t: &Tower, at: &SurfacePoint) -> f64 {
        use crate::procedural::cauliflower::glaciated;
        let maturity = t.maturity();
        if maturity <= 0. {
            return 0.;
        }
        let p = self.drift.back(&at.position);
        let q = self.surface.frayed(&p);
        let ice = glaciated(t.glaciation(), at.altitude);
        let soft = 1. - maturity;
        let edge = self.edge * (1. + 9. * ice);
        let enough = edge * (1. + 24. * soft * soft);
        let depth = t.grown(self).depth(&self.tree, &q, enough);
        if depth <= -self.surface.margin() {
            return 0.;
        }
        let billows = if depth < enough { self.surface.billowing(&q) * (1. - ice) } else { 0. };
        // Gone entirely as the column rains out.
        self.surface.condensed(depth + billows, edge, maturity, &q) * (1. - (1. - GLACIATED) * ice) * smoothstep(0., 0.15, t.column)
            * smoothstep(t.base, t.base + 0.3 * self.edge, at.altitude)
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
        let h = (at.altitude - base) / (top - base);
        if h <= 0. || h >= 1. {
            return 0.;
        }
        // Ice spread flat under the tropopause: a smooth sheet, not heaps (cumuliform noise cut
        // to the cover made it a field of puffs), broken where the cover is partial into broad
        // patches about 9 km across, its texture fibrous: noise drawn out horizontally. And thin
        // (`cauliflower::ICE`), the outflow's ice spread wide.
        let profile = smoothstep(0., 0.3, h) * (1. - smoothstep(0.75, 1., h));
        let position = Drift { rise: 0., ..self.drift }.noise_position(&at.position, &at.up);
        let q = position / (self.look.shape_size * 6.);
        let patches = stretched(self.shapes.perlin.fbm(q.x, q.y, q.z, 5, 0.55, 2.));
        let fibres = self.fibres(at);
        let v = patches * profile * (0.8 + 0.4 * fibres);
        // Soft-edged, and half the anvil ice's density: an optical depth of a few, cirrus
        // spissatus, through which the sun shows (a sharper threshold and thicker ice gave
        // patches with crisp edges and shaded undersides, like heaps).
        smoothstep(1. - cover, 1.5 - cover, v) * 0.5 * crate::procedural::cauliflower::ICE
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
            let horizontal = (t.distance - self.column_reach(t.radius)).max(0.);
            let mut gap = vertical.max(horizontal);
            if t.mature {
                // The anvil's band, and the outflow's cone below it (within the anvil's reach).
                let low = self.outflow_from(t);
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
        assert!(cloud > 100 && skipped > 50, "uninformative: cloud {} skipped {}", cloud, skipped);
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
        // The density changes smoothly, not rearranging as storms grow (half a second: the
        // towers' thermals have edges 15 m deep, and drift 5 m in that time), and the bounds hold
        // at any time.
        let mut rng = crate::geometry::Pcg32::new(6, 0);
        let (a, b) = (field(7000.), field(7000.5));
        let (mut diff, mut total) = (0., 0.);
        for k in 0..30000 {
            let u = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            if maps.deep.cover.at(&u) < 0.3 { continue; }
            // Only near storms: the bounds, a tenth of the time.
            let mut near = false;
            a.towers(&u, |t| near |= t.distance < 1.3 * t.radius);
            if !near && k % 10 != 0 { continue; }
            let at = planet.surface_point(&(planet.center() + u * (planet.radius + 1000. + rng.next_f64() * 12000.)));
            let (x, y) = (a.density(&at), b.density(&at));
            diff += (x - y).abs();
            total += x;
            if k % 10 != 0 { continue; }
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = a.majorant_from(&at, &at.position, &d, &planet);
            for k in 0..20 {
                let q = planet.surface_point(&(planet.center() + at.position + d * (len.min(30_000.) * k as f64 / 20.)));
                assert!(a.density(&q) <= m + 1e-12);
            }
        }
        assert!(total > 0.05 && diff < 0.2 * total, "{} of {}", diff, total);
    }

    #[test]
    fn test_heaps_are_bounded_and_skipped() {
        // A cumulus sky: plenty of heaps; the bound holds along random rays through them, and
        // clear air between them is skipped.
        let mut p = WeatherParams::new(12);
        p.map_size = 64;
        p.instability = 2.;
        let w = Weather::new(p, None);
        let maps = w.maps();
        let planet = Planet { radius: w.params.radius };
        let look = Look { max_density: 0.05, shape_size: 1200., detail_size: 220., erosion: 0.4 };
        let mut f = Stratiform::new(maps.clone(), Family::Low, look, 3);
        f.heaps = Some(Heaps::new(planet.radius, 9));
        f.drift = Drift::new(Vector3::y(), 8., 0.3, 3000., planet.radius);
        let mut rng = crate::geometry::Pcg32::new(7, 0);
        let (mut cloud, mut skipped, mut tested) = (0, 0, 0);
        while tested < 2000 {
            let up = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            if maps.low.cover.at(&up) < 0.2 || maps.low.form.at(&up) < 0.7 { continue; }
            tested += 1;
            let (base, top) = (maps.low.base.at(&up), maps.low.top.at(&up));
            let at = planet.surface_point(&(planet.center() + up * (planet.radius + base - 100. + rng.next_f64() * (top - base + 200.))));
            let d = crate::geometry::point_on_unit_sphere(rng.next_f64(), rng.next_f64());
            let (m, len) = f.majorant_from(&at, &at.position, &d, &planet);
            assert!(len > 0.);
            if m == 0. { skipped += 1; }
            for k in 0..40 {
                let q = planet.surface_point(&(planet.center() + at.position + d * (len.min(20_000.) * k as f64 / 40.)));
                let v = f.density(&q);
                if v > 0. { cloud += 1; }
                assert!(v <= m + 1e-12, "{} > {} at step {} of {}", v, m, k, len);
            }
        }
        assert!(cloud > 100 && skipped > 50, "uninformative: cloud {} skipped {}", cloud, skipped);
    }
}
