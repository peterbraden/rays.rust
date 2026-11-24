//! # A landscape at full scale, over a whole planet
//!
//! The simulation describes a planet, or patches of it, at resolutions of kilometres to
//! hundreds of metres. To see it as a landscape, rather than as a model of one on a table, it is
//! drawn here as the rest of the scene is: in metres, lying on the planet the skysphere and the
//! ocean assume (its centre at (0, -R, 0), so a chosen site is at the scene's origin at sea
//! level, with x east, y up and z south), with true heights, and with what a camera anywhere on
//! or above it needs.
//!
//! ## Nested simulations
//!
//! The heights come from `Layer`s, finest first: patches nested around the place the camera
//! looks at, each a few times finer than the one around it, and the whole planet (a cubed
//! sphere) beneath them all. Each patch fades into the next over its outermost cells, and
//! everything is a function of a point's direction from the planet's centre, so the layers need
//! not share a frame. A patch's simulation is driven at its edge by the one around it: its edge
//! cells rise and fall as that one's surface did there (`Terrain::boundary_rate`), as a regional
//! weather model is driven by a global one.
//!
//! A coarse layer has smoothed away the relief it can't resolve. Where only coarse layers reach,
//! that relief is put back from the planet's own initial topography: its octaves finer than the
//! layer's cells and coarser than the finest patch's (`Relief::base`), so a distant range has
//! the ridges a finer simulation would have started from.
//!
//! ## Detail below the grid
//!
//! Below the finest simulation's resolution, synthetic relief adds what no simulation
//! resolves, in proportion to the local slope: rough on mountainsides, little on valley floors
//! and plains. This is "amplification" of a coarse terrain (as in Guérin et al., "Sparse
//! representation of terrains for procedural modeling", Computer Graphics Forum 35(2), 2016),
//! shaped as water shapes slopes: gullies running straight down the large-scale slope, with
//! narrow beds and broad spurs rounded by creep, each octave running down the sides of the
//! coarser ones' so they branch (`Relief::gullies`, after the "erosion noise" of Clay John and
//! Fewes), and a little plain noise to break their lines. (Isotropic ridged noise, which this
//! replaced, rose in rounded mounds and crests going every way, which read as made.) It is drawn
//! down to whatever spacing the mesh has at that point, and no finer than `RELIEF_FINEST`. Up
//! close, the world adds a simulation five times finer than its finest about the camera
//! (`world::close_level`), so the gullies a few hundred metres apart are eroded, not drawn.
//!
//! ## Adaptive resolution: a clipmap about the camera
//!
//! The mesh is finest under the camera and coarser with distance, so that each triangle covers
//! a few pixels wherever it is seen: a geometry clipmap (Losasso and Hoppe, "Geometry clipmaps:
//! terrain rendering using nested regular grids", SIGGRAPH 2004). Square rings of N x N
//! vertices are centred under the camera, each with twice the spacing of the one within it, the
//! finest spacing set by the camera's height above the ground. Each ring's centre is snapped to
//! its parent's grid, so the edge of a ring lies on its parent's grid lines; there the ring's
//! odd vertices are put on the straight edge between its even ones, so the two meet without
//! cracks, and its noise fades towards its parent's octaves (geomorphing) so their heights
//! agree. Beyond the rings, out of reach of the tangent plane, a cubed-sphere mesh covers the
//! rest of the planet coarsely.
//!
//! With a `"displacement"`, the ground about the camera is instead a heightfield of this
//! surface (`Surface`) displaced at render time (`shapes::displaced`), and the rings leave it
//! out (`Hole`); see docs/displacement.md.
//!
//! ## Ground cover
//!
//! From what the simulations left at each point (glacier ice, alluvium, lava, rivers) and the
//! climate they ran in, by latitude: snow above the snowline (the glaciers' equilibrium line
//! altitude at the final time) on slopes gentle enough to hold it, bare rock on steep slopes
//! and above the alpine meadows, forest below the treeline, grassland and fields on the valley
//! floors. Albedos are those of real surfaces (conifer forest about 0.05, grass 0.1, rock 0.2),
//! so the sun and sky light it as they would the real thing. Snow and glacier ice are
//! `material::snow`'s, worked out from ice's absorption: snow is settled high up and older,
//! coarser and dustier towards the snowline, where it turns to firn; below the snowline a
//! glacier is bare ice, mostly the white, bubble-rich ice of its weathered surface with bluer
//! ice showing through, and streaked with rock debris towards its snout. Rivers and lakes are
//! water, reflecting the sky by Fresnel's equations as the ocean does.

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::Intersection;
use crate::material::bsdf::LambertianBSDF;
use crate::material::lambertian::Lambertian;
use crate::material::functions::scatter_bsdf;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::snow::{Ice, IceStyle, Snow, SnowStyle};
use crate::material::texture::Medium;
use crate::na::Vector3;
use crate::noise::{PerlinNoise, SeededNoise};
use crate::ocean::PhysicalOcean;
use crate::procedural::nubis_clouds::smoothstep;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
use crate::terrain::channel::{point_cell, WaterLines};
use crate::terrain::geology::Rock;
use crate::terrain::domain::{sphere_dir, Domain, Layout};
use crate::terrain::climate::{Biome, Climate};
use crate::terrain::glacier::GlacierParams;
use crate::terrain::map;
use crate::terrain::planet::Planet;
use crate::terrain::simulation::{Terrain, ICE_FREE};
use rayon::prelude::*;
use std::sync::Arc;

/// The scene's frame on the planet: the site at the scene's origin, and the mapping between
/// the planet and the scene.
#[derive(Clone, Debug)]
pub struct Frame {
    pub up: Vector3<f64>,
    pub east: Vector3<f64>,
    pub north: Vector3<f64>,
    pub radius: f64,
    /// Vertical exaggeration (1 for the real thing).
    pub exaggeration: f64,
}

impl Frame {
    /// The frame with the site at (`lat`, `lon`) degrees at the scene's origin.
    pub fn new(lat: f64, lon: f64, radius: f64, exaggeration: f64) -> Frame {
        let (up, east, north) = Domain::frame(lat, lon);
        Frame { up, east, north, radius, exaggeration }
    }

    /// The planet's centre in the scene.
    pub fn centre(&self) -> Vector3<f64> { Vector3::new(0., -self.radius, 0.) }

    /// Direction from the planet's centre of the point x metres east and z north on the site's
    /// tangent plane (gnomonic, as the domain's patches).
    pub fn dir(&self, x: f64, z: f64) -> Vector3<f64> {
        (self.up * self.radius + self.east * x + self.north * z).normalize()
    }

    /// The tangent-plane coordinates (east, north) of direction `d`, which must be on the
    /// site's side of the planet.
    pub fn tangent(&self, d: &Vector3<f64>) -> (f64, f64) {
        Domain::tangent(self.up, self.east, self.north, self.radius, d)
    }

    /// The scene position of direction `d` at height `h` metres.
    pub fn to_scene(&self, d: &Vector3<f64>, h: f64) -> Vector3<f64> {
        let v = d * (self.radius + h * self.exaggeration);
        Vector3::new(v.dot(&self.east), v.dot(&self.up) - self.radius, -v.dot(&self.north))
    }

    /// Direction and height (metres) of scene position `p`.
    pub fn on_planet(&self, p: &Vector3<f64>) -> (Vector3<f64>, f64) {
        let v = self.east * p.x + self.up * (p.y + self.radius) - self.north * p.z;
        let r = v.norm();
        (v / r, (r - self.radius) / self.exaggeration)
    }
}

/// Options for drawing a landscape at full scale.
#[derive(Clone, Debug)]
pub struct SceneryParams {
    /// Vertices along each side of a clipmap ring (rounded to a multiple of 4).
    pub resolution: usize,
    /// The finest mesh spacing, metres.
    pub min_spacing: f64,
    /// Amplitude of the sub-grid relief, in wavelengths per unit slope (a quarter of this).
    pub roughness: f64,
    pub exaggeration: f64,
    /// Snowline and treeline, metres; None to take them from the climate.
    pub snowline: Option<f64>,
    pub treeline: Option<f64>,
    /// Rivers are drawn at least this wide, metres.
    pub min_river_width: f64,
    /// The rivers' hydraulics (the flow they show).
    pub hydraulics: crate::terrain::hydraulics::HydraulicParams,
    /// Cells per face edge of the mesh over the whole planet.
    pub globe: usize,
    /// The wind over the lakes (scene x and z, m/s), which raises their waves.
    pub wind: [f64; 2],
    /// Shade bare rock, scree, sand and gravel with the library's natural materials
    /// (`material::library`), each rock its own; else with plain colours.
    pub ground_materials: bool,
}

impl Default for SceneryParams {
    fn default() -> SceneryParams {
        SceneryParams {
            resolution: 384, min_spacing: 0.5, roughness: 1., exaggeration: 1.,
            snowline: None, treeline: None, min_river_width: 0., hydraulics: Default::default(), globe: 192, ground_materials: true,
            wind: [4., 0.],
        }
    }
}

/// Cubic Catmull-Rom interpolation between b and c, at t in [0, 1].
fn catmull_rom(a: f64, b: f64, c: f64, d: f64, t: f64) -> f64 {
    b + 0.5 * t * (c - a + t * (2. * a - 5. * b + 4. * c - d + t * (3. * (b - c) + d - a)))
}

/// Surface height (ground plus ice) at each of a domain's cell corners: the mean of the cells
/// meeting there.
pub fn corner_heights(t: &Terrain) -> Vec<f64> {
    corner_values(t, &t.surface())
}

/// A field `v` of `t`'s cells at its domain's corners, each the mean of the cells about it.
pub fn corner_values(t: &Terrain, v: &[f64]) -> Vec<f64> {
    t.domain.corner_cells.iter()
        .map(|cells| cells.iter().map(|&c| v[c]).sum::<f64>() / cells.len() as f64)
        .collect()
}

/// Heights at a domain's corners, as a smooth function of direction.
pub fn sample(domain: &Domain, corners: &[f64], d: &Vector3<f64>) -> f64 {
    match &domain.layout {
        Layout::Patch { up, east, north, n, spacing } => {
            let (x, z) = Domain::tangent(*up, *east, *north, domain.radius, d);
            let n = *n as i64;
            let corner = |i: i64, j: i64| corners[(j.clamp(0, n) * (n + 1) + i.clamp(0, n)) as usize];
            let u = x / spacing + n as f64 / 2.;
            let v = z / spacing + n as f64 / 2.;
            let (i, j) = (u.floor() as i64, v.floor() as i64);
            let (fu, fv) = (u - i as f64, v - j as f64);
            let row = |jj: i64| catmull_rom(corner(i - 1, jj), corner(i, jj), corner(i + 1, jj), corner(i + 2, jj), fu);
            catmull_rom(row(j - 1), row(j), row(j + 1), row(j + 2), fv)
        }
        Layout::Sphere { n } => {
            // Bilinear on the face: corners on a face's edge are shared with the next face and
            // have the same heights, so this is continuous across them.
            let (f, u, v) = Domain::sphere_coords(*n, d);
            let n = *n;
            let corner = |i: usize, j: usize| corners[f * (n + 1) * (n + 1) + j.min(n) * (n + 1) + i.min(n)];
            let (i, j) = ((u.floor() as usize).min(n - 1), (v.floor() as usize).min(n - 1));
            let (fu, fv) = (u - i as f64, v - j as f64);
            let a = corner(i, j) * (1. - fu) + corner(i + 1, j) * fu;
            let b = corner(i, j + 1) * (1. - fu) + corner(i + 1, j + 1) * fu;
            a * (1. - fv) + b * fv
        }
    }
}

const ICE: u8 = 0;
const SEA_FLOOR: u8 = 1;
const ALLUVIUM: u8 = 2;
const LAVA: u8 = 3;
const LAND: u8 = 4;
const TILL: u8 = 5;

/// One simulation, as drawn: its surface heights and what covers its ground.
pub struct Layer {
    pub domain: Domain,
    /// Cell spacing, metres (nominal, for the sphere).
    pub spacing: f64,
    corners: Vec<f64>,
    class: Vec<u8>,
    water: Option<WaterLines>,
    /// Its rivers as courses, levelled and carved once the relief is known.
    pub rivers: Option<crate::terrain::rivers::Rivers>,
    min_river_width: f64,
    /// Width of the band inside a patch's edge over which it blends into what is around it.
    blend: f64,
    /// What the simulation left, for graphs' `field` nodes.
    fields: crate::terrain::fields::SimFields,
    /// The prevailing wind, a unit vector in the patch's tangent plane at its centre, and the
    /// direction across it there (None for no wind, or a whole planet): the dunes' trend.
    wind: Option<(Vector3<f64>, Vector3<f64>)>,
}

impl Layer {
    pub fn new(t: Terrain, min_river_width: f64, hydraulics: &crate::terrain::hydraulics::HydraulicParams) -> Layer {
        let corners = corner_heights(&t);
        let water = crate::terrain::hydraulics::Hydraulics::of(&t, hydraulics);
        let fields = crate::terrain::fields::SimFields::with_hydraulics(&t, &water);
        let rivers = matches!(t.domain.layout, Layout::Patch { .. }).then(|| crate::terrain::rivers::Rivers::new(&t, &water));
        let water = map::water_lines(&t, min_river_width);
        let class = (0..t.domain.len()).map(|i| {
            if t.ice[i] >= ICE_FREE { ICE }
            else if t.bed[i] < 0. { SEA_FLOOR }
            else if t.lava[i] > 1. && t.sediment[i] < map::ALLUVIUM { LAVA }
            else if map::is_till(&t, i) { TILL }
            else if t.sediment[i] > map::ALLUVIUM { ALLUVIUM }
            else { LAND }
        }).collect();
        let (spacing, blend) = match t.domain.layout {
            // The simulation holds a patch's edge at its first topography (moved as its
            // surroundings were), so the cells nearest the edge are the least like a landscape:
            // blend over a tenth of the patch, or at least three cells.
            Layout::Patch { n, spacing, .. } => (spacing, (0.05 * n as f64 * spacing).max(3. * spacing)),
            Layout::Sphere { n } => (std::f64::consts::FRAC_PI_2 * t.domain.radius / n as f64, 0.),
        };
        let norm = t.wind.0.hypot(t.wind.1);
        let wind = match &t.domain.layout {
            Layout::Patch { up, east, north, .. } if norm > 0. => {
                let along = (east * t.wind.0 + north * t.wind.1) / norm;
                Some((along, up.cross(&along)))
            }
            _ => None,
        };
        Layer { spacing, corners, class, water, rivers, min_river_width, blend, fields, wind, domain: t.domain }
    }

    /// Tangent coordinates of `d` on a patch, if it is on the patch's side of the planet.
    fn tangent(&self, d: &Vector3<f64>) -> Option<(f64, f64)> {
        match &self.domain.layout {
            Layout::Patch { up, east, north, .. } if d.dot(up) > 0.1 => Some(Domain::tangent(*up, *east, *north, self.domain.radius, d)),
            _ => None,
        }
    }

    fn half(&self) -> f64 {
        match self.domain.layout { Layout::Patch { n, spacing, .. } => n as f64 * spacing / 2., Layout::Sphere { .. } => f64::INFINITY }
    }

    pub fn contains(&self, d: &Vector3<f64>) -> bool {
        match self.domain.layout {
            Layout::Sphere { .. } => true,
            Layout::Patch { .. } => self.tangent(d).is_some_and(|(x, z)| x.abs().max(z.abs()) < self.half()),
        }
    }

    /// How much the layer counts at `d`: 1 within, falling to 0 at a patch's edge.
    fn weight(&self, d: &Vector3<f64>) -> f64 {
        match self.domain.layout {
            Layout::Sphere { .. } => 1.,
            Layout::Patch { .. } => match self.tangent(d) {
                Some((x, z)) => 1. - smoothstep(self.half() - self.blend, self.half(), x.abs().max(z.abs())),
                None => 0.,
            },
        }
    }

    /// The simulated surface at `d`, smoothly interpolated between the cells' corners.
    pub fn simulated(&self, d: &Vector3<f64>) -> f64 { sample(&self.domain, &self.corners, d) }

    fn class_at(&self, d: &Vector3<f64>) -> u8 { self.class[self.domain.cell_at(d)] }

    /// What is at direction `d` and height `h`: the surface of water (a river drawn on the
    /// ground, or an ox-bow lake), a river's bed under its own surface, or land. `surfaced`
    /// says whether a river of the given half width has a surface of its own at `d` (near
    /// the camera: `rivers::Carve::ribbon`), where the ground under it is never water.
    fn river_at(&self, d: &Vector3<f64>, h: f64, surfaced: impl Fn(f64) -> bool) -> RiverAt {
        let Some((x, z)) = self.tangent(d) else { return RiverAt::Land };
        let (Some(water), q) = (&self.water, [x, z]) else { return RiverAt::Land };
        let wet = |w: bool| if w { RiverAt::Water } else { RiverAt::Land };
        match self.rivers.as_ref().and_then(|r| r.carve.get()) {
            Some(carve) => match carve.level(q, self.min_river_width) {
                Some((level, half, grain)) if h < level - 0.05 || surfaced(half) => if h < level + 0.05 { RiverAt::Bed(grain) } else { RiverAt::Land },
                Some(_) => RiverAt::Water,
                None => match self.rivers.as_ref().and_then(|r| r.basins.get()?.lake(q)) {
                    // Under a lake's surface (`terrain::basins`), its bed.
                    Some(level) if h < level + 0.05 => RiverAt::Bed(0.0005),
                    // In a river's channel above its water, where the mesh draws the channel:
                    // a bare bar.
                    _ if carve.channel(q).is_some_and(|(_, half, _, out)| out < 0. && surfaced(half)) =>
                        RiverAt::Bar(carve.channel(q).map_or(0.01, |c| c.2)),
                    _ => wet(water.wet_oxbow(point_cell(&self.domain, q), q)),
                },
            },
            None => wet(water.wet(point_cell(&self.domain, q), q)),
        }
    }

    /// The patch point of direction `d`, if it is on the patch's side of the planet.
    fn carve_point(&self, d: &Vector3<f64>) -> Option<crate::terrain::channel::Point> {
        self.tangent(d).map(|(x, z)| [x, z])
    }

    /// The carving of its rivers at direction `d`, if levelled: patch point and carve.
    fn carve(&self, d: &Vector3<f64>) -> Option<(crate::terrain::channel::Point, &crate::terrain::rivers::Carve)> {
        let carve = self.rivers.as_ref()?.carve.get()?;
        let (x, z) = self.tangent(d)?;
        Some(([x, z], carve))
    }
}

/// What the ground is at a point, as far as water goes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RiverAt {
    Land,
    /// The surface of water drawn on the ground (a river from afar, a lake).
    Water,
    /// A river's bed, under its own surface; with the median grain, metres.
    Bed(f64),
    /// A river's bed bare beside its water, in its channel below the bankfull level: the bars
    /// floods scour too often for plants to hold; with the median grain, metres.
    Bar(f64),
}

/// Heights of the landscape, from its layers and the planet's own topography.
/// The large-scale surface's slope at a point: its gradient (uphill, metres per metre, in the
/// planet's frame, along the surface) and the steepness, its length up to 1.5.
#[derive(Clone, Copy, Debug)]
pub struct Slope { pub steepness: f64, pub gradient: Vector3<f64> }

pub struct Relief {
    pub layers: Arc<Vec<Layer>>,
    planet: Planet,
    radius: f64,
    /// The finest layer's spacing: the synthetic detail starts below it.
    finest: f64,
    roughness: f64,
    noise: SeededNoise,
}

/// The longest wavelength of synthetic detail is a power of two metres, so the octaves are the
/// same everywhere.
const LONGEST_DETAIL: f64 = 65536.;

/// ... and the shortest, metres: below it the ground's own detail (`terrain::detail`), drawn
/// from its geology, takes over. (Synthetic relief, ridged noise then, down to the displaced patches' centimetre
/// cells raised sharp ridges a metre high a few metres apart on steep faces, which the ground's
/// detail then stood on as fins.)
const RELIEF_FINEST: f64 = 12.;

impl Relief {
    /// `finest`: the spacing below which detail is synthetic; layers coarser than it have the
    /// planet's own relief down to it put back.
    pub fn new(layers: Arc<Vec<Layer>>, planet: &Planet, radius: f64, roughness: f64, finest: f64) -> Relief {
        let seed = planet.params.seed;
        let relief = Relief {
            layers, radius, finest, roughness,
            planet: Planet::new(planet.params.clone()),
            noise: SeededNoise::new(seed.wrapping_add(0x5ce7e)),
        };
        // The rivers' levels follow the uncarved ground (`rivers`).
        for l in relief.layers.iter() {
            if let Some(r) = &l.rivers {
                r.level(|q, local| {
                    let d = crate::terrain::rivers::patch_dir(&l.domain, q);
                    relief.uncarved(&d, &relief.slope(&d), local)
                });
            }
        }
        relief
    }

    /// The large-scale height at `d`, before synthetic detail: each layer blending into the
    /// next, coarser layers with the planet's own relief finer than their cells put back.
    pub fn base(&self, d: &Vector3<f64>) -> f64 {
        let h = self.splined(d);
        // Valley floors at the simulation's beds, where its rivers run (`terrain::rivers`).
        let Some(l) = self.layers.iter().find(|l| l.contains(d)) else { return h };
        let Some(n) = l.rivers.as_ref().zip(l.carve_point(d)).and_then(|(r, q)| r.skeleton.nearest(q)) else { return h };
        let w = l.weight(d) * (1. - smoothstep(n.half, n.half + l.spacing, n.centre));
        h + w * (n.bed - h)
    }

    /// The large-scale height from the layers' splines and the planet's relief.
    fn splined(&self, d: &Vector3<f64>) -> f64 {
        let mut rest = 1.;
        let mut h = 0.;
        for l in self.layers.iter() {
            let w = l.weight(d) * rest;
            if w > 0. {
                let mut v = l.simulated(d);
                if l.spacing > 1.01 * self.finest {
                    // Relief between twice this layer's spacing and twice the finest's.
                    v += self.planet.elevation(d, 2. * self.finest) - self.planet.elevation(d, 2. * l.spacing);
                }
                h += w * v;
                rest -= w;
            }
            if rest <= 1e-12 { return h; }
        }
        h + rest * self.planet.elevation(d, 2. * self.finest)
    }

    /// Two directions perpendicular to `d` and each other.
    fn tangents(d: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
        let a = if d.y.abs() < 0.9 { Vector3::y() } else { Vector3::x() };
        let t1 = d.cross(&a).normalize();
        (t1, d.cross(&t1))
    }

    /// `d` moved `a` metres along `t1` and `b` along `t2` (over the surface).
    fn step(&self, d: &Vector3<f64>, t1: &Vector3<f64>, t2: &Vector3<f64>, a: f64, b: f64) -> Vector3<f64> {
        (d + (t1 * a + t2 * b) / self.radius).normalize()
    }

    /// The slope of the large-scale surface at `d`, over about a cell of the finest layer.
    pub fn slope(&self, d: &Vector3<f64>) -> Slope {
        let (t1, t2) = Relief::tangents(d);
        let e = self.finest;
        let gx = self.base(&self.step(d, &t1, &t2, e, 0.)) - self.base(&self.step(d, &t1, &t2, -e, 0.));
        let gz = self.base(&self.step(d, &t1, &t2, 0., e)) - self.base(&self.step(d, &t1, &t2, 0., -e));
        let gradient = (t1 * gx + t2 * gz) / (2. * e);
        Slope { steepness: gradient.norm().min(1.5), gradient }
    }

    /// Synthetic relief at `d`, for a surface of slope `slope`, drawn down to wavelengths the
    /// mesh's spacing `local` there can carry: gullies down the slope (`gullies`), and a little
    /// noise to break them up.
    pub fn detail(&self, d: &Vector3<f64>, slope: &Slope, local: f64) -> f64 {
        let p = d * self.radius;
        // Rivers clear their floodplains of relief at every scale (`terrain::rivers`).
        let river = self.layers.iter().find(|l| l.contains(d))
            .and_then(|l| l.rivers.as_ref()?.skeleton.nearest(l.carve_point(d)?));
        let mut sum = 0.;
        // The slope as the gullies so far leave it: each octave's run down the sides of the
        // coarser ones, so they branch.
        let mut gradient = slope.gradient;
        let mut wavelength = LONGEST_DETAIL;
        let mut octave = 0.;
        while wavelength >= local.max(RELIEF_FINEST) {
            // The simulation only resolves features several cells across: below about eight
            // cells its slopes are smoothed by its own diffusion.
            let resolved = smoothstep(4. * self.finest, 8. * self.finest, wavelength);
            let carried = 1. - smoothstep(wavelength / 4., wavelength / 2., local);
            let cleared = river.map_or(1., |n| smoothstep(0., (wavelength / 2.).min(crate::terrain::rivers::cleared(n.half)), n.edge));
            let w = (1. - resolved) * carried * cleared;
            if w > 0. {
                let amplitude = w * self.roughness * wavelength;
                let (g, dg) = self.gullies(&p, d, &gradient, wavelength, octave);
                // As deep as their slope allows: on a slope s, gullies a wavelength apart are
                // about a tenth of it deep per unit of slope; none on the flat.
                let depth = 0.1 * amplitude * slope.steepness;
                sum += depth * g;
                gradient += dg * depth;
                // Noise everywhere, a little more on slopes, to break the gullies' lines.
                let q = (p + Vector3::new(31.7, 17.3, 5.1) * (octave * wavelength)) / wavelength;
                sum += 0.06 * amplitude * (0.15 + slope.steepness) * self.noise.noise(q * 1.3);
            }
            wavelength /= 2.;
            octave += 1.;
        }
        sum
    }

    /// Gullies `wavelength` apart running down a surface of slope `gradient` (uphill, metres per
    /// metre) at `p` (`d` from the planet's centre), from -1 in their beds to 1 on the spurs
    /// between, and their gradient (per metre). Erosion's pattern (as in Fewes's "erosion
    /// noise", after Clay John's): waves across the slope from points scattered on a lattice of
    /// the wavelength, each fading with distance, so they run straight down the slope and are
    /// broken where one point's waves give way to another's. The beds are narrow and the spurs
    /// broad and rounded, as creep leaves them.
    fn gullies(&self, p: &Vector3<f64>, d: &Vector3<f64>, gradient: &Vector3<f64>, wavelength: f64, octave: f64) -> (f64, Vector3<f64>) {
        let g = gradient - d * d.dot(gradient);
        let steep = g.norm();
        if steep < 1e-6 { return (0., Vector3::zeros()); }
        let across = d.cross(&(g / steep));
        let q = p / wavelength;
        let cell = q.map(f64::floor);
        let (mut v, mut dv, mut total) = (0., Vector3::zeros(), 0.);
        let seed = (octave as u64).wrapping_mul(0x632b_e59b_d9b4_e019) ^ 0x9e37_79b9;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let c = cell + Vector3::new(i as f64, j as f64, k as f64);
                    let mut h = seed;
                    for x in [c.x, c.y, c.z] { h = (h ^ (x as i64 as u64)).wrapping_mul(0xbf58_476d_1ce4_e5b9); h ^= h >> 29; }
                    let r = |s: u32| ((h >> s) & 0xffff) as f64 / 65535.;
                    let at = c + Vector3::new(r(0), r(16), r(32));
                    let o = q - at;
                    let weight = (-2. * o.norm_squared()).exp();
                    if weight < 1e-4 { continue; }
                    let phase = std::f64::consts::TAU * (o.dot(&across) + r(48));
                    v += weight * phase.cos();
                    dv -= across * (weight * phase.sin() * std::f64::consts::TAU / wavelength);
                    total += weight;
                }
            }
        }
        if total <= 0. { return (0., Vector3::zeros()); }
        let (v, dv) = (v / total, dv / total);
        // Narrow beds, broad spurs: a profile rising steeply out of each bed.
        let shaped = 1. - 2. * (1. - (0.5 + 0.5 * v).clamp(0., 1.)).powf(1.5);
        let slope = 1.5 * (1. - (0.5 + 0.5 * v).clamp(0., 1.)).sqrt();
        (shaped, dv * slope)
    }

    /// The wind-blown sand at `d`, metres, blended between the layers as their surfaces are,
    /// and the wind of the layer that counts most there.
    fn sand(&self, d: &Vector3<f64>) -> (f64, Option<(Vector3<f64>, Vector3<f64>)>) {
        let (mut rest, mut sand, mut wind, mut most) = (1., 0., None, 0.);
        for l in self.layers.iter() {
            let w = l.weight(d) * rest;
            if w > 0. {
                sand += w * l.fields.at(&l.domain, crate::graph::Field::Sand, d, 0.).unwrap_or(0.);
                if w > most { most = w; wind = l.wind; }
                rest -= w;
            }
            if rest <= 1e-12 { break; }
        }
        (sand, wind)
    }

    /// Dunes on the wind-blown sand at `d` (`terrain::aeolian`), metres, drawn where the mesh's
    /// spacing `local` can carry them. Ridges across the prevailing wind, `DUNE_SPACING` apart:
    /// a long windward slope rising to the crest, then the slip face down the lee at the
    /// angle of repose's steepness over a short run; their height grows with the sand's depth,
    /// up to a twelfth of their spacing (transverse dunes are 10-15 times as far apart as they
    /// are high: Lancaster, "Geomorphology of Desert Dunes", 1995). Their crests wander, and
    /// where the sand is thin they break into separate crescents, barchans, the more so the
    /// thinner it is.
    pub fn dunes(&self, d: &Vector3<f64>, local: f64) -> f64 {
        let (sand, wind) = self.sand(d);
        self.dunes_on(d, local, sand, wind)
    }

    fn dunes_on(&self, d: &Vector3<f64>, local: f64, sand: f64, wind: Option<(Vector3<f64>, Vector3<f64>)>) -> f64 {
        const DUNE_SPACING: f64 = 400.;
        let carried = 1. - smoothstep(DUNE_SPACING / 6., DUNE_SPACING / 3., local);
        if carried <= 0. { return 0.; }
        let Some((downwind, side)) = wind else { return 0. };
        // Dunes fade in with the sand, so the ground doesn't step where it begins.
        let fade = smoothstep(0.2, 1.5, sand);
        if fade <= 0. { return 0.; }
        // Distances along and across the wind, in the patch's own frame.
        let p = d * self.radius;
        let (along, across) = (p.dot(&downwind), p.dot(&side));
        // Noise along the crests, changing only slowly along the wind, so that each dune keeps
        // its shape from its windward foot to its slip face.
        let crests = |scale: f64, stretch: f64, o: f64| self.noise.noise(Vector3::new(across / (scale * DUNE_SPACING), along / (stretch * scale * DUNE_SPACING), o));
        // Wandering crests.
        let phase = along / DUNE_SPACING + 0.5 * crests(2.5, 3., 8.1) + 0.2 * crests(0.8, 2., 4.6);
        let s = phase.rem_euclid(1.);
        const SLIP: f64 = 0.15;
        let profile = if s < 1. - SLIP { s / (1. - SLIP) } else { (1. - s) / SLIP } - 0.5;
        // The sand gathers into dunes standing on the ground between them, rather than
        // spreading as a thin sheet: they are 8 m high on thin sand, higher where there is
        // more, up to a twelfth of their spacing, and cover as much of the ground as the
        // sand's volume allows (a dune's profile holds half its height on average), the rest
        // bare between barchans.
        let height = (8. + 0.5 * sand).min(DUNE_SPACING / 12.);
        let cover = (sand / (0.5 * height)).min(1.);
        // Where along the crests the dunes stand: as much of them as `cover`.
        let threshold = 0.6 - 1.2 * cover;
        let present = if cover >= 1. { 1. } else { smoothstep(threshold - 0.1, threshold + 0.1, crests(1., 1.5, 1.9)) };
        // The sand the dunes take is taken from the surface beneath them, which already
        // stands on all of it.
        carried * fade * (present * height * (profile + 0.5) - sand.min(0.5 * height * cover))
    }

    /// The surface's height at `d` with detail down to spacing `local`, given its large-scale
    /// slope there.
    pub fn height(&self, d: &Vector3<f64>, slope: &Slope, local: f64) -> f64 {
        let g = self.uncarved(d, slope, local);
        let Some(l) = self.layers.iter().find(|l| l.contains(d)) else { return g };
        let Some((q, carve)) = l.carve(d) else { return g };
        carve.carved(q, g, local)
    }

    /// The river at direction `d`, if `d` is on a river's surface: the flow there, and its
    /// direction (a unit vector in the planet's frame).
    pub fn river_at(&self, d: &Vector3<f64>) -> Option<(crate::terrain::rivers::Node, Vector3<f64>, [f64; 2])> {
        let l = self.layers.iter().find(|l| l.contains(d))?;
        let (q, carve) = l.carve(d)?;
        let (node, along, station) = carve.water_at(q)?;
        match &l.domain.layout {
            Layout::Patch { east, north, .. } => Some((node, east * along[0] + north * along[1], station)),
            Layout::Sphere { .. } => None,
        }
    }

    /// The height before rivers cut their channels.
    pub fn uncarved(&self, d: &Vector3<f64>, slope: &Slope, local: f64) -> f64 {
        let (sand, wind) = self.sand(d);
        // Drifting sand buries the ground's small relief.
        let buried = 1. - 0.8 * smoothstep(0.5, 4., sand);
        self.base(d) + buried * self.detail(d, slope, local) + self.dunes_on(d, local, sand, wind)
    }

    /// The surface's normal at `d` (outwards, in planet coordinates), by finite differences
    /// over the mesh's spacing.
    pub fn normal(&self, d: &Vector3<f64>, slope: &Slope, local: f64) -> Vector3<f64> {
        let (t1, t2) = Relief::tangents(d);
        let e = local / 2.;
        let h = |a: f64, b: f64| self.height(&self.step(d, &t1, &t2, a, b), slope, local);
        let gx = (h(e, 0.) - h(-e, 0.)) / (2. * e);
        let gz = (h(0., e) - h(0., -e)) / (2. * e);
        (d - t1 * gx - t2 * gz).normalize()
    }
}

/// The camera, as the mesh needs it.
#[derive(Clone, Debug)]
pub struct Viewpoint {
    /// Where the camera is, in the scene.
    pub location: Vector3<f64>,
}

/// One mesh vertex, before the planet frame is applied: its direction, and the mesh's spacing
/// there.
struct Vertex { d: Vector3<f64>, local: f64 }

/// The layout of the clipmap's rings: square rings of `2 half + 1` vertices a side, each centred
/// on twice its spacing's grid near the camera's foot (`cx`, `cz` on the tangent plane), the
/// first `s0` apart, out to `reach`.
pub struct Clipmap {
    half: i64,
    s0: f64,
    reach: f64,
    cx: f64,
    cz: f64,
}

impl Clipmap {
    pub fn new(relief: &Relief, frame: &Frame, view: &Viewpoint, p: &SceneryParams) -> Clipmap {
        let n = (p.resolution / 4).max(4) * 4;
        let half = (n / 2) as i64;
        let (foot, cam_h) = frame.on_planet(&view.location);
        let ground = relief.base(&foot);
        let above = (cam_h - ground.max(0.)).max(1.);
        // The nearest ground is about the camera's height away: there a ring's spacing is its
        // half-width over N / 4.
        let s0 = (above * 4. / n as f64).max(p.min_spacing);
        // Out to the horizon, and the mountains beyond it, or a quarter of the way round.
        let reach = ((2. * relief.radius * (cam_h.max(0.) + 9000.)).sqrt() + (2. * relief.radius * 9000f64).sqrt())
            .min(0.3 * relief.radius);
        // From far off, the globe's mesh is fine enough: the rings stay within reach.
        let s0 = s0.min(reach / half as f64);
        let (cx, cz) = frame.tangent(&foot);
        Clipmap { half, s0, reach, cx, cz }
    }

    /// Ring `level`'s centre, in units of s0: snapped to twice its spacing, so it lies on its
    /// parent's grid.
    fn centre(&self, level: u32) -> (i64, i64) {
        let step = 1i64 << level;
        let snap = |c: f64| ((c / self.s0 / (2 * step) as f64).round() as i64) * 2 * step;
        (snap(self.cx), snap(self.cz))
    }

    /// The spacing the relief is drawn to at tangent-plane point (`x`, `z`) by the ring it lies
    /// in: its own spacing, tending to its parent's towards its edge (see `mesh`); None beyond
    /// the rings.
    pub fn local(&self, x: f64, z: f64) -> Option<f64> {
        let (gx, gz) = (x / self.s0, z / self.s0);
        for level in 0..=30 {
            let step = 1i64 << level;
            let (kx, kz) = self.centre(level);
            let w = (self.half * step) as f64;
            let r = (gx - kx as f64).abs().max((gz - kz as f64).abs()) / w;
            if r <= 1. {
                return Some(self.s0 * step as f64 * (1. + smoothstep(0.75, 1., r)));
            }
            if w * self.s0 >= self.reach { break; }
        }
        None
    }
}

/// A frame on the planet's tangent plane at a point of the ground, in the scene: x east, y up
/// and z south there, as the scene's own axes are at its origin, so that a heightfield in it
/// stands upright however far from the origin it is.
#[derive(Clone, Debug)]
pub struct Local {
    /// Where sea level is under the point, and the frame's axes (columns), in the scene.
    pub origin: Vector3<f64>,
    pub axes: crate::na::Matrix3<f64>,
}

impl Local {
    /// The frame under tangent-plane point (`x`, `z`) of `frame`.
    pub fn under(frame: &Frame, x: f64, z: f64) -> Local {
        let d = frame.dir(x, z);
        let (lat, lon) = Domain::lat_lon(&d);
        let (up, east, north) = Domain::frame(lat, lon);
        // Planet directions into the scene: as Frame::to_scene, without the translation.
        let scene = |v: Vector3<f64>| Vector3::new(v.dot(&frame.east), v.dot(&frame.up), -v.dot(&frame.north));
        Local {
            origin: frame.to_scene(&d, 0.),
            axes: crate::na::Matrix3::from_columns(&[scene(east), scene(up), -scene(north)]),
        }
    }

    pub fn to_scene(&self, p: &Vector3<f64>) -> Vector3<f64> { self.origin + self.axes * p }

    pub fn to_local(&self, p: &Vector3<f64>) -> Vector3<f64> { self.axes.transpose() * (p - self.origin) }
}

/// A part of the ground drawn by something else (a displaced patch about the camera): a square
/// in a `Local` frame's x and z, and the surface it has there, which the mesh's vertices inside
/// it are moved onto, so the mesh's triangles that overlap its edge follow it.
pub struct Hole {
    pub local: Local,
    pub min: (f64, f64),
    pub max: (f64, f64),
    pub surface: Arc<crate::procedural::heightmap::HeightGrid>,
}

impl Hole {
    /// Whether scene point `p` is over the square.
    fn inside(&self, p: &Vector3<f64>) -> bool {
        let q = self.local.to_local(p);
        q.x > self.min.0 && q.x < self.max.0 && q.z > self.min.1 && q.z < self.max.1
    }
}

/// The landscape's surface as a heightfield over a `Local` frame's x and z: the relief drawn
/// to the spacing of the clipmap ring each point lies in, or `cell` where that is finer, so
/// that where it meets the rings it agrees with them.
pub struct Surface<'a> {
    pub relief: &'a Relief,
    pub frame: &'a Frame,
    pub local: &'a Local,
    pub clipmap: &'a Clipmap,
    pub cell: f64,
}

impl crate::procedural::heightmap::HeightSource for Surface<'_> {
    fn height(&self, x: f64, z: f64) -> f64 {
        let at = |y: f64| self.frame.on_planet(&self.local.to_scene(&Vector3::new(x, y, z)));
        let (d, _) = at(0.);
        let (tx, tz) = self.frame.tangent(&d);
        let spacing = self.clipmap.local(tx, tz).unwrap_or(self.cell).max(self.cell);
        let slope = self.relief.slope(&d);
        // The point above (x, z) at the surface's height: moving up the column barely turns
        // the direction from the planet's centre, so this settles in a few steps.
        let mut y = 0.;
        for _ in 0..4 {
            let (d, h) = at(y);
            y += (self.relief.height(&d, &slope, spacing) - h) * self.frame.exaggeration;
        }
        y
    }
}

/// The landscape's mesh: clipmap rings about the camera, and a coarse globe beyond them,
/// leaving out the triangles wholly inside `patch`.
pub fn mesh(relief: &Relief, frame: &Frame, view: &Viewpoint, p: &SceneryParams, patch: Option<&Hole>) -> SmoothMesh {
    let clipmap = Clipmap::new(relief, frame, view, p);
    let Clipmap { half, s0, reach, .. } = clipmap;
    let mut vertices: Vec<Vertex> = vec![];
    let mut triangles: Vec<[usize; 3]> = vec![];
    // Odd vertices on each ring's outer edge, to put on the straight edge between their
    // neighbours: (vertex, neighbour, neighbour).
    let mut seams: Vec<(usize, usize, usize)> = vec![];
    // Each ring's centre and extent, in units of s0.
    let mut hole: Option<(i64, i64, i64)> = None;
    let mut level = 0;
    let outer;
    loop {
        let step = 1i64 << level;
        let s = s0 * step as f64;
        let (kx, kz) = clipmap.centre(level);
        let w = half * step;
        let base = vertices.len();
        let width = 2 * half as usize + 1;
        for j in 0..width as i64 {
            for i in 0..width as i64 {
                let (gx, gz) = (kx + (i - half) * step, kz + (j - half) * step);
                // Geomorphing: towards the edge, the spacing the noise is drawn to tends to the
                // parent's, so heights agree where the rings meet.
                let r = ((gx - kx).abs().max((gz - kz).abs())) as f64 / w as f64;
                let local = s * (1. + smoothstep(0.75, 1., r));
                vertices.push(Vertex { d: frame.dir(gx as f64 * s0, gz as f64 * s0), local });
            }
        }
        let v = |i: usize, j: usize| base + j * width + i;
        for j in 0..width - 1 {
            for i in 0..width - 1 {
                let (gx, gz) = (kx + (i as i64 - half) * step, kz + (j as i64 - half) * step);
                if let Some((hx, hz, hw)) = hole {
                    let inside = |g: i64, c: i64| g >= c - hw && g + step <= c + hw;
                    if inside(gx, hx) && inside(gz, hz) { continue; }
                }
                triangles.push([v(i, j), v(i + 1, j), v(i + 1, j + 1)]);
                triangles.push([v(i, j), v(i + 1, j + 1), v(i, j + 1)]);
            }
        }
        for k in (1..width - 1).step_by(2) {
            let last = width - 1;
            seams.push((v(k, 0), v(k - 1, 0), v(k + 1, 0)));
            seams.push((v(k, last), v(k - 1, last), v(k + 1, last)));
            seams.push((v(0, k), v(0, k - 1), v(0, k + 1)));
            seams.push((v(last, k), v(last, k - 1), v(last, k + 1)));
        }
        hole = Some((kx, kz, w));
        if w as f64 * s0 >= reach || level >= 30 {
            outer = (kx as f64 * s0, kz as f64 * s0, w as f64 * s0);
            break;
        }
        level += 1;
    }
    // The rest of the planet: a cubed sphere, leaving out the cells the rings cover.
    let g = p.globe.max(2);
    let in_rings = |d: &Vector3<f64>| d.dot(&frame.up) > 0.2 && {
        let (x, z) = frame.tangent(d);
        (x - outer.0).abs() <= outer.2 && (z - outer.1).abs() <= outer.2
    };
    let spacing = std::f64::consts::FRAC_PI_2 * relief.radius / g as f64;
    for f in 0..6 {
        let base = vertices.len();
        let coord = |i: usize| -1. + 2. * i as f64 / g as f64;
        for j in 0..=g {
            for i in 0..=g {
                vertices.push(Vertex { d: sphere_dir(f, coord(i), coord(j)), local: spacing });
            }
        }
        let v = |i: usize, j: usize| base + j * (g + 1) + i;
        for j in 0..g {
            for i in 0..g {
                let quad = [v(i, j), v(i + 1, j), v(i + 1, j + 1), v(i, j + 1)];
                if quad.iter().all(|&k| in_rings(&vertices[k].d)) { continue; }
                // The face's axes are u x v = outwards, so this is anticlockwise from outside.
                triangles.push([quad[0], quad[1], quad[2]]);
                triangles.push([quad[0], quad[2], quad[3]]);
            }
        }
    }
    // Heights and normals.
    let shaped: Vec<(Vector3<f64>, Vector3<f64>)> = vertices.par_iter().map(|v| {
        let slope = relief.slope(&v.d);
        let h = relief.height(&v.d, &slope, v.local);
        let n = relief.normal(&v.d, &slope, v.local);
        // Rotated into the scene: to_scene is a rotation about the planet's centre.
        let normal_scene = frame.to_scene(&n, 0.) - frame.centre();
        (frame.to_scene(&v.d, h), normal_scene.normalize())
    }).collect();
    let mut positions: Vec<Vector3<f64>> = shaped.iter().map(|x| x.0).collect();
    let mut normals: Vec<Vector3<f64>> = shaped.iter().map(|x| x.1).collect();
    for (k, a, b) in seams {
        positions[k] = (positions[a] + positions[b]) / 2.;
        normals[k] = (normals[a] + normals[b]).normalize();
    }
    if let Some(patch) = patch {
        // Within the patch, onto its surface (straight up its frame, as it is a heightfield).
        for (p, n) in positions.iter_mut().zip(normals.iter_mut()) {
            if patch.inside(p) {
                let mut q = patch.local.to_local(p);
                let (y, hx, hz) = patch.surface.smooth(q.x, q.z);
                q.y = y;
                *p = patch.local.to_scene(&q);
                *n = patch.local.axes * Vector3::new(-hx, 1., -hz).normalize();
            }
        }
        triangles.retain(|t| !t.iter().all(|&k| patch.inside(&positions[k])));
    }
    SmoothMesh::from_indexed(&[IndexedMesh { positions, normals, triangles, texcoords: vec![] }])
}

/// What covers the ground, where.
pub struct Ground {
    frame: Frame,
    layers: Arc<Vec<Layer>>,
    /// A fixed snowline and treeline, metres; else from the climate.
    snowline: Option<f64>,
    treeline: Option<f64>,
    /// The climate model; without one, the glaciers' snowline by latitude, at `time`.
    climate: Option<Arc<Climate>>,
    glaciers: GlacierParams,
    time: f64,
    noise: PerlinNoise,
    /// Snow, high on the mountains and near the snowline; a glacier's white surface ice and
    /// the bluer ice beneath.
    snow: [Snow; 2],
    ice: [Ice; 2],
    /// The mesh's spacing about the camera, once known: where rivers have surfaces of their
    /// own.
    pub clipmap: std::sync::OnceLock<Clipmap>,
    /// The library's materials for each kind of bare ground (`Bare::index`), if used.
    materials: Vec<Option<Box<dyn MaterialModel + Sync + Send>>>,
}

/// What lies bare on the ground at a point, under any snow or ice: plants and soil (the
/// cover's colour), or bare rock of a kind, or loose stones, sand or gravel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bare { Plain, Rock(Rock), Scree, Sand, Gravel }

impl Bare {
    const COUNT: usize = 4 + 9;

    fn index(self) -> usize {
        match self {
            Bare::Plain => 0,
            Bare::Scree => 1,
            Bare::Sand => 2,
            Bare::Gravel => 3,
            Bare::Rock(r) => 4 + r.index(),
        }
    }

    /// The library's material for it, and its seed's salt: rocks by their lithology (slate
    /// for shale), loose ground by what it is.
    fn library(self) -> Option<&'static str> {
        Some(match self {
            Bare::Plain => return None,
            Bare::Scree => "scree",
            Bare::Sand => "dune_sand",
            Bare::Gravel => "river_gravel",
            Bare::Rock(r) => match r {
                Rock::Sediment => "scree",
                Rock::Shale => "slate",
                Rock::Sandstone => "sandstone",
                Rock::Limestone => "limestone_cliff",
                Rock::Quartzite => "quartzite",
                Rock::Schist => "schist",
                Rock::Gneiss => "gneiss",
                Rock::Granite => "granite_outcrop",
                Rock::Basalt => "basalt",
            },
        })
    }
}

/// The shares of each kind of bare ground at a point, as the cover's colours are blended.
#[derive(Clone, Copy, Debug)]
struct Shares([f64; Bare::COUNT]);

impl Shares {
    fn of(s: Bare) -> Shares {
        let mut w = [0.; Bare::COUNT];
        w[s.index()] = 1.;
        Shares(w)
    }

    /// As `mix` blends colours: `t` of `other` into these.
    fn mix(&mut self, other: Shares, t: f64) {
        let t = t.clamp(0., 1.);
        for (a, b) in self.0.iter_mut().zip(other.0) { *a = *a * (1. - t) + b * t; }
    }

    /// One of them, chosen by its share.
    fn pick(&self) -> Bare {
        let mut x = rand() * self.0.iter().sum::<f64>();
        for (k, &w) in self.0.iter().enumerate() {
            if x < w || k == Bare::COUNT - 1 {
                return match k {
                    0 => Bare::Plain, 1 => Bare::Scree, 2 => Bare::Sand, 3 => Bare::Gravel,
                    k => Bare::Rock(Rock::from_index(k - 4)),
                };
            }
            x -= w;
        }
        Bare::Plain
    }
}

/// What covers the ground at a point: the shares of snow and of glacier ice, the rest bare
/// ground of colour `ground`.
#[derive(Clone, Copy, Debug)]
pub struct Cover {
    pub ground: Color,
    pub snow: f64,
    /// How old the snow is, from 0 (settled) to 1 (old snow and firn).
    pub snow_age: f64,
    pub ice: f64,
    /// How much of the ice is blue rather than white, 0 to 1.
    pub blue: f64,
    /// What lies bare under the snow and ice, chosen by the shares of the kinds there.
    pub surface: Bare,
}

/// Albedos of real surfaces (linear, as the renderer's colours are).
const ROCK: [f64; 3] = [0.20, 0.18, 0.16];
const SCREE: [f64; 3] = [0.26, 0.25, 0.23];
const FOREST: [f64; 3] = [0.028, 0.042, 0.022];
const GRASS: [f64; 3] = [0.07, 0.10, 0.04];
const MEADOW: [f64; 3] = [0.11, 0.11, 0.06];
const FIELDS: [f64; 3] = [0.13, 0.13, 0.07];
const GRAVEL: [f64; 3] = [0.24, 0.23, 0.21];
const SAND: [f64; 3] = [0.32, 0.28, 0.21];
const BASALT: [f64; 3] = [0.06, 0.055, 0.05];
/// Playas: salt crust (halite and gypsum, about half the light; Bonneville's is brighter
/// still), and the clay pan where there is less salt.
const SALT: [f64; 3] = [0.58, 0.57, 0.54];
const CLAY_PAN: [f64; 3] = [0.36, 0.32, 0.27];
/// Dune sand: quartz, pale where fresh, red where old (the Namib's, the Simpson's).
const DUNE_SAND: [f64; 3] = [0.46, 0.37, 0.24];
const RED_SAND: [f64; 3] = [0.45, 0.25, 0.12];

fn colour(c: [f64; 3]) -> Color { Color::new(c[0], c[1], c[2]) }

impl Ground {
    pub fn new(layers: Arc<Vec<Layer>>, climate: Option<Arc<Climate>>, glaciers: GlacierParams, time: f64, seed: u64, frame: Frame, p: &SceneryParams) -> Ground {
        Ground {
            frame, layers, snowline: p.snowline, treeline: p.treeline, climate, glaciers, time,
            noise: PerlinNoise::shuffled(seed.wrapping_add(77)),
            snow: [Snow::new(SnowStyle::preset("settled"), 1., seed), Snow::new(SnowStyle::preset("firn"), 1., seed)],
            ice: [Ice::new(IceStyle::preset("white")), Ice::new(IceStyle::preset("glacier"))],
            clipmap: std::sync::OnceLock::new(),
            materials: (0..Bare::COUNT).map(|k| {
                let s = if k < 4 { [Bare::Plain, Bare::Scree, Bare::Sand, Bare::Gravel][k] } else { Bare::Rock(Rock::from_index(k - 4)) };
                let name = s.library().filter(|_| p.ground_materials)?;
                let definition = serde_json::json!({"type": "library", "name": name, "seed": seed.wrapping_mul(31).wrapping_add(k as u64)});
                Some(crate::scenefile::SceneFile::parse_material_definition(&definition, name, &serde_json::Map::new()))
            }).collect(),
        }
    }

    /// The snowline at `d`, metres: where summer stays below freezing, or the glaciers'
    /// equilibrium line by latitude.
    pub fn snowline(&self, d: &Vector3<f64>) -> f64 {
        if let Some(s) = self.snowline { return s; }
        match &self.climate {
            Some(c) => c.local(d, 0.).snowline(0.),
            None => self.glaciers.ela(d.y.clamp(-1., 1.).asin(), self.time),
        }
    }

    /// The treeline at `d`, metres: where the warmest month reaches 10 °C, or 800 m below the
    /// snowline.
    pub fn treeline(&self, d: &Vector3<f64>) -> f64 {
        if let Some(t) = self.treeline { return t; }
        match &self.climate {
            Some(c) => c.local(d, 0.).treeline(0.),
            None => self.snowline(d) - 800.,
        }
    }

    /// The finest layer at `d`.
    fn layer_at(&self, d: &Vector3<f64>) -> Option<&Layer> {
        self.layers.iter().find(|l| l.contains(d))
    }

    /// The rock exposed at `d`, `h` metres up (`SimFields::rock`): from the finest simulation
    /// that is more than half of the blend there, so it isn't averaged between simulations.
    pub fn rock(&self, d: &Vector3<f64>, h: f64) -> Option<Rock> {
        let l = self.layers.iter().find(|l| l.weight(d) > 0.5).or(self.layers.last())?;
        Some(l.fields.rock(&l.domain, d, h))
    }

    /// The colour of bare rock at `d`, `h` metres up: the bedrock's own (a sediment-covered
    /// cell's steep slopes show the default rock), weathered towards a common grey, with the
    /// rock's own texture: shale and schist finely laminated, sandstone in beds with iron
    /// stains, limestone pale with dark streaks where water runs down it, granite speckled with
    /// pale weathered patches, gneiss in bands of foliation, quartzite cut by dark joints,
    /// basalt dark with rusty weathering. Seen from afar the textures average out (they are
    /// metres across), so the rock's colour is what shows.
    fn stone(&self, d: &Vector3<f64>, h: f64) -> Color {
        let Some(rock) = self.rock(d, h).filter(|&r| r != Rock::Sediment) else { return colour(ROCK) };
        let q = d * self.frame.radius;
        // Noise in [-1/2, 1/2] at scale `s` metres, stretched `v` times vertically.
        let n = |s: f64, v: f64, o: f64| self.noise.fbm(q.x / s + o * 13., q.y / s + h / (s * v), q.z / s, 3, 0.5, 2.) - 0.5;
        let band = |period: f64, phase: f64| (std::f64::consts::TAU * (h + phase) / period).sin();
        let base = colour(ROCK) * 0.3 + colour(rock.colour()) * 0.7;
        let (light, tint) = match rock {
            Rock::Shale => (0.12 * band(1.5, 2. * n(20., 1., 0.1)), [1., 1., 1.]),
            Rock::Schist => (0.1 * band(0.8, 3. * n(15., 1., 0.2)) + 0.15 * n(3., 1., 0.3), [1., 1., 1.03]),
            Rock::Sandstone => {
                let iron = (n(25., 0.5, 0.4) + 0.2).max(0.);
                (0.15 * band(6., 10. * n(60., 1., 0.5)), [1. + 0.5 * iron, 1. + 0.1 * iron, 1. - 0.3 * iron])
            }
            Rock::Limestone => {
                // Streaks: noise along the ground only, so they run straight down the face.
                let streak = self.noise.fbm(q.x / 4., q.y / 4. + 7.7, q.z / 4., 2, 0.5, 2.) - 0.5;
                (-0.35 * (streak + 0.15).max(0.) * 2. + 0.05 * band(4., 0.), [1., 1., 1.])
            }
            Rock::Granite => (0.18 * n(0.6, 1., 0.6) + 0.25 * (n(80., 1., 0.7) + 0.1).max(0.), [1.02, 1., 0.98]),
            Rock::Gneiss => {
                // Foliation dipping across the ground, not level like bedding.
                let dip = q.dot(&Vector3::new(0.6, 0.3, 0.74)) * 0.5;
                (0.22 * band(3., dip + 4. * n(30., 1., 0.8)), [1., 1., 1.])
            }
            Rock::Quartzite => {
                let joint = (self.noise.fbm(q.x / 6., q.y / 6. + h / 6., q.z / 6. + 3.3, 2, 0.5, 2.) - 0.5).abs();
                (-0.3 * (1. - smoothstep(0., 0.03, joint)), [1., 1., 1.])
            }
            Rock::Basalt => {
                let rust = (n(10., 1., 0.9) + 0.1).max(0.);
                (0.1 * n(2., 1., 1.), [1. + 1.2 * rust, 1. + 0.4 * rust, 1.])
            }
            Rock::Sediment => (0., [1., 1., 1.]),
        };
        let shade = 1. + light;
        Color::new(base.rgb.x * tint[0] * shade, base.rgb.y * tint[1] * shade, base.rgb.z * tint[2] * shade)
    }

    /// How readily the rock at `d`, `h` metres up goes bare, in steepness: positive for rock
    /// that keeps thin soils (quartzite, granite, limestone), bare on gentler slopes; negative
    /// for rock that weathers to soil (shale, schist), mantled until steeper.
    fn bareness(&self, d: &Vector3<f64>, h: f64) -> f64 {
        match self.rock(d, h) {
            Some(Rock::Quartzite) | Some(Rock::Granite) => 0.06,
            Some(Rock::Limestone) | Some(Rock::Gneiss) | Some(Rock::Basalt) => 0.03,
            Some(Rock::Shale) | Some(Rock::Schist) => -0.04,
            _ => 0.,
        }
    }

    /// Whether scene point `p` is in a river or lake.
    pub fn wet(&self, p: &Vector3<f64>) -> bool {
        self.river_at(p) == RiverAt::Water
    }

    /// What `p` is as far as water goes (`Layer::river_at`).
    pub fn river_at(&self, p: &Vector3<f64>) -> RiverAt {
        let (d, h) = self.frame.on_planet(p);
        let surfaced = |half: f64| self.clipmap.get().is_some_and(|c| {
            let (x, z) = self.frame.tangent(&d);
            c.local(x, z).is_some_and(|s| s < half)
        });
        self.layer_at(&d).map_or(RiverAt::Land, |l| l.river_at(&d, h, surfaced))
    }

    /// Whether scene point `p` is under a closed basin's terminal lake: brine, shallow over a
    /// pale floor of salt and clay.
    pub fn salt_lake(&self, p: &Vector3<f64>) -> bool {
        let (d, _) = self.frame.on_planet(p);
        self.layer_at(&d).is_some_and(|l| l.fields.salt_lake(&l.domain, &d) > crate::terrain::simulation::SALT_LAKE_DEPTH)
    }

    /// How much of the ground at scene point `p` is a playa (`graph::Field::Playa`), blended
    /// between the simulations as their surfaces are.
    fn playa(&self, p: &Vector3<f64>) -> f64 {
        crate::graph::Fields::field(self, crate::graph::Field::Playa, p).unwrap_or(0.)
    }

    /// The climate at `d`, `h` metres up, its edges wandering with the weather from year to
    /// year (noise in temperature and rain), as the ground cover and `biome` nodes see it. None
    /// without a climate.
    fn wandering_climate(&self, d: &Vector3<f64>, h: f64) -> Option<crate::terrain::climate::Local> {
        let q = d * self.frame.radius;
        let n = |s: f64, o: f64| self.noise.fbm(q.x / s + o * 17., q.y / s, q.z / s, 4, 0.5, 2.);
        let mut local = self.climate.as_ref()?.local(d, h);
        local.mean += 3. * (n(40000., 0.15) - 0.5);
        local.summer += 3. * (n(40000., 0.15) - 0.5);
        local.precipitation *= (1.2 * (n(25000., 0.45) - 0.5)).exp();
        Some(local)
    }

    /// The mean albedo at scene point `p`, whose surface normal is `normal`.
    pub fn albedo(&self, p: &Vector3<f64>, normal: &Vector3<f64>) -> Color {
        let c = self.cover(p, normal);
        let snow = self.snow[0].albedo() * (1. - c.snow_age) + self.snow[1].albedo() * c.snow_age;
        let ice = self.ice[0].reflectance() * (1. - c.blue) + self.ice[1].reflectance() * c.blue;
        c.ground * (1. - c.snow - c.ice) + snow * c.snow + ice * c.ice
    }

    /// What covers the ground at scene point `p`, whose surface normal is `normal`.
    pub fn cover(&self, p: &Vector3<f64>, normal: &Vector3<f64>) -> Cover {
        let up = (p - self.frame.centre()).normalize();
        let flat = normal.dot(&up);
        let (d, h) = self.frame.on_planet(p);
        let r = self.frame.radius;
        // What the simulation left here, looked up within half a cell of the point so that
        // over many samples the classes blend rather than show the grid.
        let class = match self.layer_at(&d) {
            Some(l) => {
                let (t1, t2) = Relief::tangents(&d);
                let j = (d + (t1 * (rand() - 0.5) + t2 * (rand() - 0.5)) * l.spacing / r).normalize();
                self.layer_at(&j).map_or(LAND, |l| l.class_at(&j))
            }
            None => LAND,
        };
        let q = d * r;
        let n = |s: f64, o: f64| self.noise.fbm(q.x / s + o * 17., q.y / s, q.z / s, 4, 0.5, 2.);
        // Fine variation, seen close up.
        let grain = 0.85 + 0.3 * n(7., 0.21);
        let mix = |a: Color, b: Color, t: f64| { let t = t.clamp(0., 1.); a * (1. - t) + b * t };
        let snowline = self.snowline(&d);
        let treeline = self.treeline(&d);
        let bare = |ground: Color| Cover { ground, snow: 0., snow_age: 0., ice: 0., blue: 0., surface: Bare::Plain };
        // Snow ages towards the snowline: coarser, dustier, turning to firn.
        let age = |h: f64| 1. - smoothstep(snowline, snowline + 800., h + (n(300., 0.35) - 0.5) * 300.);
        if class == LAVA { return Cover { surface: Bare::Rock(Rock::Basalt), ..bare(colour(BASALT) * (0.8 + 0.4 * n(200., 0.9)) * grain) }; }
        if class == ICE {
            // Snow over the glacier's accumulation zone, bare ice below the snowline.
            let snow = smoothstep(snowline - 300., snowline + 100., h + (n(700., 0.2) - 0.5) * 300.);
            // Ice doesn't hold to cliffs: the rock walls above and beside a glacier show.
            let held = smoothstep(0.6, 0.8, flat);
            // Bare ice: the weathered surface's white ice, bluer ice where it is fresh (in
            // bands, as the ice's layers and the summer's melt expose it), and rock debris
            // carried on the ice, more of it towards the snout.
            let blue = smoothstep(0.45, 0.7, n(80., 0.4) + 0.3 * (n(15., 0.45) - 0.5));
            let low = smoothstep(snowline - 300., snowline - 1500., h);
            let debris = (1. - snow) * smoothstep(0.55, 0.75, n(350., 0.6) + 0.35 * low - 0.1);
            let stone = self.stone(&d, h) * (0.7 + 0.6 * n(250., 0.7)) * grain;
            let ground = mix(stone, colour(GRAVEL) * (0.8 + 0.4 * n(40., 0.65)), held);
            // Crevasses: where the ice steepens it speeds up and stretches, and opens in
            // cracks across its flow (along the slope's contours), tens of metres apart, their
            // walls deep blue ice (a few metres of it is enough to absorb the red).
            let downhill = normal - up * flat;
            let steep = 1. - smoothstep(0.94, 0.985, flat);
            let crevasse = if steep > 0. && downhill.norm() > 1e-6 {
                let along = q.dot(&downhill.normalize());
                let phase = (along / 30. + 3. * n(120., 0.85)).rem_euclid(1.);
                held * steep * (1. - smoothstep(0.08, 0.22, phase)) * smoothstep(0.35, 0.5, n(60., 0.95))
            } else { 0. };
            let bare_ice = held * (1. - snow) * (1. - debris);
            let ice = bare_ice + crevasse * (1. - bare_ice);
            return Cover {
                ground,
                snow: held * snow * (1. - crevasse),
                snow_age: age(h),
                ice,
                blue: if ice > 0. { (bare_ice * (1. - crevasse) * blue + crevasse) / ice } else { 0. },
                surface: Bare::Plain,
            };
        }
        let tree = treeline + (n(1500., 0.3) - 0.5) * 400.;
        // What grows here: by the climate's biome, its edges wandering with the weather from
        // year to year (noise in temperature and rain); without a climate, forest and grass.
        let (mut ground, dry) = match &self.climate {
            Some(_) => {
                let local = self.wandering_climate(&d, h).expect("a climate");
                let biome = Biome::of(&local);
                let mut g = match biome {
                    Biome::Ice => colour(SCREE),
                    b => colour(b.albedo()),
                };
                if biome.forest() {
                    // Clearings and meadows among the trees.
                    let open = colour(if biome == Biome::Rainforest { Biome::Savanna.albedo() } else { GRASS });
                    let cover = smoothstep(0.3, 0.45, n(2500., 0.5) + 0.12 * n(60., 0.55) + 0.25 * (1. - flat));
                    g = mix(open, g, cover);
                } else if matches!(biome, Biome::HotDesert | Biome::ColdDesert) {
                    // Dunes, pans and darker stony desert.
                    g = g * (0.75 + 0.5 * n(3000., 0.8) + 0.15 * n(150., 0.3));
                } else {
                    // Grass, scrub and the odd copse.
                    g = mix(g, colour(FOREST), 0.5 * smoothstep(0.6, 0.75, n(800., 0.5)));
                }
                (g, local.precipitation < local.dry_threshold())
            }
            None => {
                let forest = smoothstep(0.38, 0.55, n(2500., 0.5) + 0.12 * n(60., 0.55) + 0.25 * (1. - flat)) * (1. - smoothstep(tree - 150., tree + 50., h));
                (mix(colour(GRASS), colour(FOREST), forest), false)
            }
        };
        // Alpine meadow from the treeline up.
        ground = mix(ground, colour(MEADOW), smoothstep(tree - 100., tree + 200., h));
        // The kinds of bare ground, blended as their colours are.
        let mut shares = Shares::of(Bare::Plain);
        if class == TILL {
            // Till and moraines: unsorted rubble, boulders in a grey matrix of rock flour,
            // thinly grown over where it is warm enough (moraines left by the last glaciers
            // carry forest and pasture).
            let rubble = colour(SCREE) * (0.75 + 0.5 * n(12., 0.95)) * (1. - 0.3 * smoothstep(0.6, 0.7, n(3., 0.15)));
            let grown = if dry { 0.15 } else { 0.6 * (1. - smoothstep(tree - 200., tree + 100., h)) };
            let g = grown * smoothstep(0.3, 0.6, n(200., 0.45));
            ground = mix(rubble, ground, g);
            shares = Shares::of(Bare::Scree);
            shares.mix(Shares::of(Bare::Plain), g);
        }
        if class == ALLUVIUM {
            // Valley floors: fields and meadows where it is wet enough, gravel where it is too
            // high or too dry for them.
            let fields = mix(colour(GRASS), colour(FIELDS), smoothstep(0.4, 0.6, n(350., 0.8)));
            let barren = if dry { 1. } else { smoothstep(tree, tree + 300., h) };
            ground = mix(fields, colour(GRAVEL), barren);
            shares.mix(Shares::of(Bare::Gravel), barren);
        }
        // Playas: the flat floors of closed basins, white with salt where the water last
        // evaporated, cracked into polygons a few metres across, darker and browner where the
        // clay is damp or dust has blown over it.
        let playa = self.playa(p);
        if playa > 0. {
            // (A playa is level by nature: only its shores and the sides of its own cracks are
            // steep, and the crust covers those.)
            let extent = smoothstep(0.35, 0.65, playa + 0.3 * (n(600., 0.25) - 0.5)) * smoothstep(0.5, 0.7, flat);
            if extent > 0. {
                let salt = smoothstep(0.35, 0.6, n(900., 0.75) + 0.3 * playa);
                let crust = mix(colour(CLAY_PAN), colour(SALT), salt);
                // Polygonal cracks: the ridges of a noise's cells, thin dark lines.
                let crack = 1. - 0.35 * (1. - smoothstep(0.0, 0.025, (n(4., 0.15) - 0.5).abs()));
                ground = mix(ground, crust * crack * (0.92 + 0.16 * n(40., 0.35)), extent);
                // (Salt crust has no library material: a plain colour.)
                shares.mix(Shares::of(Bare::Plain), extent);
            }
        }
        // Wind-blown sand, over all but steep rock (slip faces stand at about 33°), its colour
        // varying between dune fields: redder where it is older and its grains are coated in
        // iron oxide.
        let sand = crate::graph::Fields::field(self, crate::graph::Field::Sand, p).unwrap_or(0.);
        if sand > 0.5 {
            let extent = smoothstep(0.5, 2.5, sand + 2. * (n(300., 0.85) - 0.5)) * smoothstep(0.75, 0.85, flat);
            let dune = mix(colour(DUNE_SAND), colour(RED_SAND), smoothstep(0.3, 0.7, n(8000., 0.55)));
            ground = mix(ground, dune * (0.9 + 0.2 * n(20., 0.15)), extent);
            shares.mix(Shares::of(Bare::Sand), extent);
        }
        // Beaches.
        let beach = 1. - smoothstep(2., 12., h + 8. * n(150., 0.4));
        ground = mix(ground, colour(SAND), beach);
        shares.mix(Shares::of(Bare::Sand), beach);
        // Bare rock on steep slopes (about 40°), and scree and rock above the meadows.
        let high = smoothstep(tree + 300., tree + 900., h + (n(600., 0.6) - 0.5) * 300.);
        // (Bare at gentler slopes on rock that keeps thin soils, steeper on rock that weathers to
        // soil.)
        let bare = self.bareness(&d, h);
        let rock = (1. - smoothstep(0.7 + bare, 0.85 + bare, flat)).max(high * (1. - 0.6 * smoothstep(0.92, 0.99, flat)));
        let loose = smoothstep(0.85, 0.95, flat);
        let stone = mix(self.stone(&d, h), colour(SCREE), loose) * (0.7 + 0.6 * n(250., 0.7));
        ground = mix(ground, stone, rock) * grain;
        if rock > 0. {
            let mut bare = Shares::of(Bare::Rock(self.rock(&d, h).filter(|&r| r != Rock::Sediment).unwrap_or(Rock::Sediment)));
            bare.mix(Shares::of(Bare::Scree), loose);
            shares.mix(bare, rock);
        }
        // Snow lies above the snowline where slopes are gentle enough to hold it, and drifts
        // lower in sheltered places.
        let line = snowline + (n(900., 0.1) - 0.5) * 500.;
        let snow = smoothstep(line - 150., line + 150., h) * smoothstep(0.72, 0.88, flat + 0.15 * (n(120., 0.3) - 0.5));
        Cover { ground, snow, snow_age: age(h), ice: 0., blue: 0., surface: shares.pick() }
    }
}

impl crate::graph::Fields for Ground {
    /// Round the planet, whose centre is the planet's radius below the scene's origin.
    fn vertical(&self, p: &Vector3<f64>) -> Option<(Vector3<f64>, f64)> {
        let v = p + Vector3::y() * self.frame.radius;
        let r = v.norm();
        (r > 0.).then(|| (v / r, r - self.frame.radius))
    }

    /// The simulations' fields, each layer's blended into the coarser ones around it as their
    /// surfaces are; the climate's, and the snowline and treeline as the ground cover has them.
    fn biome(&self, p: &Vector3<f64>) -> Option<Biome> {
        let (d, h) = self.frame.on_planet(p);
        self.wandering_climate(&d, h).map(|l| Biome::of(&l))
    }

    fn field(&self, f: crate::graph::Field, p: &Vector3<f64>) -> Option<f64> {
        use crate::graph::Field;
        let (d, h) = self.frame.on_planet(p);
        match f {
            Field::Elevation => return Some(h),
            Field::Snowline => return Some(h - self.snowline(&d)),
            Field::Treeline => return Some(h - self.treeline(&d)),
            Field::River => {
                // From the rivers as drawn, near them (the skeleton's straight segments cut
                // across their bends by metres); from the skeleton farther out.
                let far = crate::terrain::rivers::CLEAR;
                return Some(self.layer_at(&d)
                    .and_then(|l| {
                        let q = l.carve_point(&d)?;
                        let rivers = l.rivers.as_ref()?;
                        rivers.carve.get().and_then(|c| c.edge(q)).or_else(|| Some(rivers.skeleton.nearest(q)?.edge))
                    })
                    .map_or(far, |e| e.min(far)));
            },
            Field::Channel => {
                // Beyond the channel's rim near a river drawn at full scale; else from its water.
                let far = crate::terrain::rivers::CLEAR;
                return Some(self.layer_at(&d)
                    .and_then(|l| {
                        let q = l.carve_point(&d)?;
                        let rivers = l.rivers.as_ref()?;
                        rivers.carve.get().and_then(|c| c.channel(q)).map(|c| c.3.max(0.)).or_else(|| Some(rivers.skeleton.nearest(q)?.edge))
                    })
                    .map_or(far, |e| e.min(far)));
            },
            Field::Grain => {
                // Near a river drawn at full scale, its bed's; else the simulation's.
                if let Some(g) = self.layer_at(&d).and_then(|l| l.rivers.as_ref()?.carve.get()?.nearest_bed(l.carve_point(&d)?)) {
                    return Some(g.1);
                }
            },
            Field::Temperature | Field::Precipitation => {
                let local = self.climate.as_ref()?.local(&d, h);
                return Some(if f == Field::Temperature { local.mean } else { local.precipitation });
            },
            // A rock's number can't be blended between simulations.
            Field::Rock => return self.rock(&d, h).map(|r| r.index() as f64),
            Field::Bedrock => return self.layers.iter().find(|l| l.weight(&d) > 0.5).or(self.layers.last())
                .map(|l| l.fields.bedrock(&l.domain, &d, h).index() as f64),
            Field::Hardness => return self.rock(&d, h).map(|r| -r.scores().0),
            _ => {},
        }
        let (mut sum, mut left) = (0., 1.);
        for l in self.layers.iter() {
            let w = l.weight(&d);
            if w <= 0. { continue; }
            sum += left * w * l.fields.at(&l.domain, f, &d, h)?;
            left *= 1. - w;
            if left <= 1e-6 { break; }
        }
        // Beyond every simulation (with no planet-wide one), the last one's edge.
        if left > 1e-6 && let Some(l) = self.layers.last() {
            sum += left * l.fields.at(&l.domain, f, &d, h)?;
        }
        Some(sum)
    }
}

pub struct Land {
    ground: Arc<Ground>,
}

impl MaterialModel for Land {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let i = &intersection.facing(r);
        let g = &self.ground;
        let c = g.cover(&i.point, &i.normal);
        // One of the surfaces covering the point, chosen by its share.
        match crate::material::snow::pick(&[1. - c.snow - c.ice, c.snow, c.ice]) {
            1 => scatter_bsdf(&g.snow[(rand() < c.snow_age) as usize].bsdf(&i.point, &i.normal, Some(s)), r, &i.point, s),
            2 => scatter_bsdf(&g.ice[(rand() < c.blue) as usize].bsdf(&i.normal, Some(s)), r, &i.point, s),
            // Bare ground: its library material, or its colour.
            _ => match g.materials.get(c.surface.index()).and_then(|m| m.as_deref()) {
                Some(m) => m.scatter(r, intersection, s),
                None => scatter_bsdf(&LambertianBSDF { albedo: c.ground.as_reflectance(), normal: i.normal }, r, &i.point, s),
            },
        }
    }
}

/// The landscape's materials: water in its rivers and lakes, land elsewhere (the ground cover,
/// or the scene's own material).
/// Water drawn on the ground (rivers too narrow for the mesh to carve, from afar; ox-bows):
/// shaded as level water, its normal the local vertical rather than the ground's, so it
/// mirrors what is across it as the water would. (On the ground's own slopes the reflections
/// went into the hillside, and the water looked dark.)
pub struct FlatWater {
    pub water: PhysicalOcean,
    pub centre: Vector3<f64>,
}

impl MaterialModel for FlatWater {
    fn scatter(&self, r: &Ray, i: &Intersection, s: &Scene) -> ScatteredRay {
        self.water.scatter(r, &Intersection { normal: (i.point - self.centre).normalize(), ..*i }, s)
    }
    fn sun_glint(&self, r: &Ray, i: &Intersection, s: &Scene) -> Option<Color> {
        self.water.sun_glint(r, &Intersection { normal: (i.point - self.centre).normalize(), ..*i }, s)
    }
}

pub struct LandscapeMedium {
    ground: Arc<Ground>,
    land: Box<dyn MaterialModel + Sync + Send>,
    water: FlatWater,
    /// Rivers' beds, under water: sand, and gravel and coarser.
    beds: [Lambertian; 2],
    /// Their bars, bare beside the water: sand, and gravel and coarser (the material
    /// library's).
    bars: [Box<dyn MaterialModel + Sync + Send>; 2],
    /// Terminal lakes: shallow brine over a pale floor, greener and brighter than rivers.
    brine: PhysicalOcean,
}

impl LandscapeMedium {
    pub fn new(ground: Ground, material: Option<Box<dyn MaterialModel + Sync + Send>>) -> LandscapeMedium {
        LandscapeMedium::with_ground(Arc::new(ground), material)
    }

    /// As `new`, with ground shared with something else that reads it (a displaced patch's
    /// graphs).
    pub fn with_ground(ground: Arc<Ground>, material: Option<Box<dyn MaterialModel + Sync + Send>>) -> LandscapeMedium {
        let centre = ground.frame.centre();
        LandscapeMedium {
            land: material.unwrap_or_else(|| Box::new(Land { ground: ground.clone() })),
            // Rivers are shallow and carry silt, sending back more green and brown than the sea.
            water: FlatWater { water: PhysicalOcean { water: Color::new(0.01, 0.018, 0.018), roughness: 0.004, center: Some(centre), foam: None, absorption: None }, centre },
            brine: PhysicalOcean { water: Color::new(0.03, 0.07, 0.065), roughness: 0.003, center: Some(centre), foam: None, absorption: None },
            ground,
            beds: [Lambertian { albedo: crate::textures::ColorTexture::Constant(bed_albedo(SAND_ALBEDO)) }, Lambertian { albedo: crate::textures::ColorTexture::Constant(bed_albedo(GRAVEL_ALBEDO)) }],
            bars: ["beach_sand", "river_gravel"].map(|name| crate::scenefile::SceneFile::parse_material_definition(&serde_json::json!({"type": "library", "name": name}), name, &serde_json::Map::new())),
        }
    }
}

/// Dry sand and river gravel (cobbles of mixed rock), as albedos.
const SAND_ALBEDO: [f64; 3] = [0.55, 0.50, 0.40];
const GRAVEL_ALBEDO: [f64; 3] = [0.30, 0.28, 0.25];

/// A dry albedo wetted. A film of water over the grains turns back, by total internal
/// reflection at its surface, much of the light the grains scatter up, so it meets the grains
/// again and more of it is absorbed (Ångström's model, which Lekner and Dorf, "Why some things
/// are darker when wet", Applied Optics 27(7), 1988, doi:10.1364/AO.27.001278, extend). With x
/// the fraction of diffuse light the water's surface turns back (about 0.6 for water) and a the
/// grains' albedo, the light that escapes after any number of returns is a (1 - x) / (1 - a x).
/// Here the dry albedo stands in for a (to check against their paper, which also accounts for
/// the water changing the grains' own scattering); bright sand darkens by a third, dark gravel
/// by half, as wet sand and stones do.
fn bed_albedo(dry: [f64; 3]) -> Color {
    const X: f64 = 0.6;
    let wet = |a: f64| a * (1. - X) / (1. - a * X);
    Color::new(wet(dry[0]), wet(dry[1]), wet(dry[2]))
}

impl Medium for LandscapeMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        match self.ground.river_at(&pt) {
            RiverAt::Water => &self.water,
            RiverAt::Bed(grain) => &self.beds[(grain > 0.002) as usize],
            RiverAt::Bar(grain) => &*self.bars[(grain > 0.002) as usize],
            RiverAt::Land if self.ground.salt_lake(&pt) => &self.brine,
            RiverAt::Land => &*self.land,
        }
    }

    fn fields(&self) -> Option<&dyn crate::graph::Fields> { Some(&*self.ground) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::simulation::TerrainParams;

    fn frame() -> Frame { Frame::new(45., 7., 6.36e6, 1.) }

    #[test]
    fn test_site_is_the_scene_origin_and_the_planet_curves_away() {
        let f = frame();
        let o = f.to_scene(&f.dir(0., 0.), 0.);
        assert!(o.norm() < 1e-6, "{:?}", o);
        // East is +x, north is -z, and 100 km away the ground has dropped by d² / 2R.
        let e = f.to_scene(&f.dir(100_000., 0.), 0.);
        assert!(e.x > 99_000. && (e.y + 100_000f64.powi(2) / (2. * 6.36e6)).abs() < 10., "{:?}", e);
        let n = f.to_scene(&f.dir(0., 1000.), 0.);
        assert!(n.z < -999., "{:?}", n);
        let up = f.to_scene(&f.dir(0., 0.), 2000.);
        assert!((up - Vector3::new(0., 2000., 0.)).norm() < 1e-6);
    }

    #[test]
    fn test_on_planet_inverts_to_scene() {
        let f = frame();
        let d = f.dir(12_345., -6_789.);
        let p = f.to_scene(&d, 1234.);
        let (d2, h) = f.on_planet(&p);
        assert!((d2 - d).norm() < 1e-12 && (h - 1234.).abs() < 1e-6);
        let (x, z) = f.tangent(&d2);
        assert!((x - 12_345.).abs() < 1e-6 && (z + 6_789.).abs() < 1e-6);
    }

    #[test]
    fn test_catmull_rom_passes_through_its_points() {
        assert_eq!(catmull_rom(1., 2., 4., 8., 0.), 2.);
        assert!((catmull_rom(1., 2., 4., 8., 1.) - 4.).abs() < 1e-12);
        // And reproduces a straight line.
        assert!((catmull_rom(0., 1., 2., 3., 0.3) - 1.3).abs() < 1e-12);
    }

    fn layers() -> (Planet, Arc<Vec<Layer>>, (f64, f64)) {
        let mut params = TerrainParams::default();
        params.planet.radius = 6.36e6;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let fine = Terrain::new(Domain::patch(lat, lon, 16, 1000., params.planet.radius), params.clone());
        let coarse = Terrain::new(Domain::patch(lat + 0.01, lon, 16, 4000., params.planet.radius), params.clone());
        let globe = Terrain::new(Domain::sphere(16, params.planet.radius), params);
        (planet, Arc::new(vec![Layer::new(fine, 0., &Default::default()), Layer::new(coarse, 0., &Default::default()), Layer::new(globe, 0., &Default::default())]), (lat, lon))
    }

    #[test]
    fn test_dunes_fade_in_with_the_sand() {
        // Sand thickening from nothing across a patch: the dunes' relief changes by no more
        // than their slopes allow from one metre to the next, even where the sand begins.
        let mut params = TerrainParams::default();
        params.planet.radius = 6.36e6;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = (20., 10.);
        let mut t = Terrain::new(Domain::patch(lat, lon, 16, 1000., params.planet.radius), params.clone());
        let n = t.domain.len();
        let east: Vec<f64> = (0..n).map(|k| (k % 16) as f64).collect();
        t.sediment = east.iter().map(|x| (x - 6.).max(0.) * 0.5).collect();
        t.sand = t.sediment.clone();
        t.wind = (0., 1.);
        let relief = Relief::new(Arc::new(vec![Layer::new(t, 0., &Default::default())]), &planet, params.planet.radius, 0., 1000.);
        let frame = Frame::new(lat, lon, params.planet.radius, 1.);
        // Across the wind (which blows north), from no sand to several metres.
        for row in 0..8 {
            let z = -2000. + 500. * row as f64;
            let h: Vec<f64> = (0..5000).map(|k| relief.dunes(&frame.dir(-3500. + k as f64, z), 5.)).collect();
            let steepest = h.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0., f64::max);
            assert!(steepest < 0.3, "row {}: a step of {} m", row, steepest);
        }
    }

    #[test]
    fn test_dunes_face_their_slip_faces_downwind() {
        // A patch covered in 30 m of sand under a wind towards the east: along the wind the
        // dunes rise gently and drop steeply; without sand there are none.
        let mut params = TerrainParams::default();
        params.planet.radius = 6.36e6;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = (20., 10.);
        let mut t = Terrain::new(Domain::patch(lat, lon, 16, 1000., params.planet.radius), params.clone());
        let n = t.domain.len();
        t.sediment = vec![30.; n];
        t.sand = vec![30.; n];
        t.wind = (1., 0.);
        let relief = Relief::new(Arc::new(vec![Layer::new(t, 0., &Default::default())]), &planet, params.planet.radius, 0., 1000.);
        let frame = Frame::new(lat, lon, params.planet.radius, 1.);
        // Lines along the wind, some through dunes and some between crescents.
        let mut lines = 0;
        for z in 0..12 {
            let profile: Vec<f64> = (0..4000).map(|k| relief.dunes(&frame.dir(-2000. + k as f64, -3000. + 500. * z as f64), 5.)).collect();
            let (mut rise, mut drop) = (0f64, 0f64);
            for w in profile.windows(2) {
                rise = rise.max(w[1] - w[0]);
                drop = drop.max(w[0] - w[1]);
            }
            let span = profile.iter().cloned().fold(f64::MIN, f64::max) - profile.iter().cloned().fold(f64::MAX, f64::min);
            // About one crest every 400 m.
            let crests = profile.windows(3).filter(|w| w[1] > w[0] && w[1] >= w[2] && w[1] > 0.).count();
            assert!((7..=14).contains(&crests), "{} crests in 4 km", crests);
            if span < 15. { continue; }
            lines += 1;
            assert!(drop > 3. * rise, "rise {} drop {} per metre", rise, drop);
            assert!(drop < 1., "slip faces {} per metre", drop);
        }
        assert!(lines >= 6, "dunes on {} lines of 12", lines);
        // Too coarse a mesh carries none.
        assert_eq!(relief.dunes(&frame.dir(0., 0.), 500.), 0.);
    }

    #[test]
    fn test_relief_is_continuous_across_its_layers() {
        let (planet, layers, (lat, lon)) = layers();
        let r = Relief::new(layers.clone(), &planet, 6.36e6, 1., 1000.);
        let f = Frame::new(lat, lon, 6.36e6, 1.);
        // Across each patch's edge (the coarse one is centred a little to the north).
        for (half, z0) in [(8000., 0.), (32000., 0.01f64.to_radians() * 6.36e6)] {
            for z in [-5000., 0., 3000.] {
                let a = r.base(&f.dir(half - 1e-3, z0 + z));
                let b = r.base(&f.dir(half + 1e-3, z0 + z));
                assert!((a - b).abs() < 0.5, "{} vs {}", a, b);
            }
        }
        // At the centre, it is the finest layer.
        let c = f.dir(0., 0.);
        assert!((r.base(&c) - layers[0].simulated(&c)).abs() < 1e-9);
        // The globe is continuous across a face's edge.
        let e = sphere_dir(0, 1., 0.3);
        let (a, b) = (r.base(&(e + Vector3::new(0., 0., -1e-9)).normalize()), r.base(&(e + Vector3::new(0., 0., 1e-9)).normalize()));
        assert!((a - b).abs() < 1., "{} vs {}", a, b);
    }

    #[test]
    fn test_clipmap_is_finest_under_the_camera_and_covers_the_planet() {
        let (planet, layers, (lat, lon)) = layers();
        let r = Relief::new(layers, &planet, 6.36e6, 1., 1000.);
        let f = Frame::new(lat, lon, 6.36e6, 1.);
        let p = SceneryParams { resolution: 32, globe: 8, ..SceneryParams::default() };
        let m = mesh(&r, &f, &Viewpoint { location: Vector3::new(0., 5000., 0.) }, &p, None);
        use crate::shapes::geometry::Geometry;
        // A ray straight down from the camera meets the ground.
        let down = Ray { ro: Vector3::new(1., 20000., 1.), rd: Vector3::new(0., -1., 0.) };
        let hit = m.intersects(&down).expect("no ground under the camera");
        assert!(hit.dist > 10000. && hit.dist < 21000., "{}", hit.dist);
        // Rays aimed at the planet's centre from all round it meet the ground.
        for d in [Vector3::new(1., 0., 0.), Vector3::new(0., -1., 0.), Vector3::new(-0.3, 0.2, 0.9)] {
            let d: Vector3<f64> = d.normalize();
            let ro = f.centre() + d * 7e6;
            assert!(m.intersects(&Ray { ro, rd: -d }).is_some(), "{:?}", d);
        }
        // The rings are crack-free: rays down through a ring's edge all hit.
        for k in 0..400 {
            let x = -40000. + 200. * k as f64;
            let r = Ray { ro: Vector3::new(x, 20000., 0.37 * x), rd: Vector3::new(0., -1., 0.) };
            assert!(m.intersects(&r).is_some(), "crack at {}", x);
        }
    }

    #[test]
    fn test_local_frames_stand_upright() {
        let f = Frame::new(45., 7., 6.36e6, 1.);
        // At the scene's origin, the scene's own axes.
        let o = Local::under(&f, 0., 0.);
        assert!(o.origin.norm() < 1e-6 && (o.axes - crate::na::Matrix3::identity()).norm() < 1e-9);
        // Far off, up is away from the planet's centre, and the axes are a rotation.
        let far = Local::under(&f, 150e3, -90e3);
        let radial = (far.origin - f.centre()).normalize();
        assert!((far.axes.column(1) - radial).norm() < 1e-9);
        assert!((far.axes.transpose() * far.axes - crate::na::Matrix3::identity()).norm() < 1e-9);
        assert!((far.axes.determinant() - 1.).abs() < 1e-9);
        let p = Vector3::new(3., 4., 5.);
        assert!((far.to_local(&far.to_scene(&p)) - p).norm() < 1e-6);
    }

    #[test]
    fn test_displaced_patch_meets_the_rings() {
        use crate::shapes::displaced::{DisplacedTerrain, Detail};
        use crate::shapes::geometry::Geometry;
        let (planet, layers, (lat, lon)) = layers();
        let r = Relief::new(layers, &planet, 6.36e6, 1., 1000.);
        let f = Frame::new(lat, lon, 6.36e6, 1.);
        let p = SceneryParams { resolution: 64, globe: 8, ..SceneryParams::default() };
        // At the scene's origin, and far enough off that the ground's up is turned a degree
        // and a half from the scene's.
        for (x0, z0) in [(0., 0.), (140e3, -70e3)] {
            let local = Local::under(&f, x0, z0);
            let (d, h) = f.on_planet(&local.origin);
            let ground = r.height(&d, &r.slope(&d), 2.) - h;
            // The synthetic detail, which the rings carry less of than the patch, grows with
            // the slope (`Relief::detail`), and so may their difference near the patch's edge.
            // The octaves the rings leave out, up to about twice their cells (a few metres here),
            // sum to about half that wavelength times (0.05 + slope); with a margin.
            let tolerance = 6. * (0.05 + r.slope(&d).steepness);
            let view = Viewpoint { location: local.to_scene(&Vector3::new(0., ground + 50., 0.)) };
            let clipmap = Clipmap::new(&r, &f, &view, &p);
            let surface = Surface { relief: &r, frame: &f, local: &local, clipmap: &clipmap, cell: 2. };
            let grid = Arc::new(crate::procedural::heightmap::HeightGrid::sample(&surface, (-200., -200.), (200., 200.), 2.));
            let hole = Hole { local: local.clone(), min: (-200., -200.), max: (200., 200.), surface: grid.clone() };
            let rings = mesh(&r, &f, &view, &p, None);
            let flat = crate::graph::Graph::parse(&serde_json::json!(0)).unwrap();
            let patch = DisplacedTerrain::new(grid.clone(), flat, false, &Detail::default()).unwrap();
            let mut m = crate::na::Matrix4::identity();
            m.fixed_view_mut::<3, 3>(0, 0).copy_from(&local.axes);
            m.fixed_view_mut::<3, 1>(0, 3).copy_from(&local.origin);
            let placed = crate::shapes::transform::Transform::new(Arc::new(patch) as Arc<dyn Geometry + Sync + Send>,
                crate::na::Affine3::from_matrix_unchecked(m));
            let both = crate::shapes::group::Group::new(vec![Box::new(mesh(&r, &f, &view, &p, Some(&hole))), Box::new(placed)]);
            // Rays straight down the frame's up, and heights along it.
            let down = |x: f64, z: f64| Ray { ro: local.to_scene(&Vector3::new(x, ground + 20000., z)), rd: -local.axes.column(1).into_owned() };
            let y = |g: &dyn Geometry, x: f64, z: f64| g.intersects(&down(x, z)).map(|h| local.to_local(&h.point).y);
            let mut last: Option<f64> = None;
            for k in 0..400 {
                let x = 120. + 0.37 * k as f64;
                let z = 0.3 * x - 40.;
                // No crack where the patch meets the rings, and no step.
                let h = y(&both, x, z).unwrap_or_else(|| panic!("crack at {} ({}, {})", x, x0, z0));
                if let Some(l) = last {
                    // Where the patch gives way to the rings, as much as they may differ: each may be
                    // `tolerance` from the surface, on either side.
                    assert!((h - l).abs() < (2. * tolerance).max(1.), "step of {} at {} ({}, {})", h - l, x, x0, z0);
                }
                last = Some(h);
                // Inside, the patch's surface (as diced); near its edge, from 180 m, where the rings
                // may stand a little above it, close to the rings'.
                if x < 180. {
                    assert!((h - grid.smooth(x, z).0).abs() < 0.01, "{} vs the patch's {} at {} ({}, {})", h, grid.smooth(x, z).0, x, x0, z0);
                }
                if (180. ..200.).contains(&x) {
                    let ring = y(&rings, x, z).unwrap();
                    assert!((h - ring).abs() < tolerance, "{} vs the rings' {} at {} ({}, {}), within {}", h, ring, x, x0, z0, tolerance);
                }
            }
        }
    }
}

