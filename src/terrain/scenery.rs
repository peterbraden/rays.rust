//! # A landscape at full scale
//!
//! The simulation describes a patch of a planet at a resolution of hundreds of metres. To see
//! it as a landscape, rather than as a model of one on a table, it is drawn here as the rest of
//! the scene is: in metres, lying on the planet the skysphere and the ocean assume (its centre
//! at (0, -R, 0), so the patch's centre is at the scene's origin at sea level, with x east, y
//! up and z south), with true heights, and with what a camera on or above it needs:
//!
//! - **Curvature.** Every point is placed by its direction from the planet's centre and its
//!   height, so distant ground drops below the horizon (d² / 2R: 785 m at 100 km).
//! - **Detail below the grid.** The mesh is several times finer than the simulation's cells.
//!   Heights between cell corners are interpolated by Catmull-Rom splines (smooth, unlike the
//!   cells' facets), and fractal noise adds the relief the grid can't resolve, in proportion to
//!   the local slope: rough on mountainsides, little on valley floors and plains. This is
//!   "amplification" of a coarse terrain (as in Guérin et al., "Sparse representation of
//!   terrains for procedural modeling", Computer Graphics Forum 35(2), 2016), in its simplest
//!   form.
//! - **Surroundings.** The patch is a window onto a whole planet: the same seed's noise
//!   topography at time zero continues beyond its edge (where the simulation holds the patch's
//!   edge at that topography, so the two meet), out to `horizon` metres, on a grid whose
//!   spacing grows geometrically away from the patch. So there is no edge to fall off, and
//!   distant ranges stand on the horizon.
//! - **Ground cover** (`Ground`), from what the simulation left at each point (glacier ice,
//!   alluvium, lava, rivers) and the climate it ran in: snow above the snowline (the glaciers'
//!   equilibrium line altitude at the final time) on slopes gentle enough to hold it, bare rock
//!   on steep slopes and above the alpine meadows, forest below the treeline, grassland and
//!   fields on the valley floors. Albedos are those of real surfaces (conifer forest about 0.05,
//!   grass 0.1, rock 0.2, snow 0.8), so the sun and sky light it as they would the real thing.
//!   Rivers and lakes are water, reflecting the sky by Fresnel's equations as the ocean does.

use crate::color::Color;
use crate::geometry::rand;
use crate::intersection::Intersection;
use crate::material::bsdf::LambertianBSDF;
use crate::material::functions::scatter_bsdf;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::texture::Medium;
use crate::na::Vector3;
use crate::noise::{PerlinNoise, SeededNoise};
use crate::ocean::PhysicalOcean;
use crate::procedural::nubis_clouds::smoothstep;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::mesh::{IndexedMesh, SmoothMesh};
use crate::terrain::channel::{point_cell, WaterLines};
use crate::terrain::domain::{Domain, Layout};
use crate::terrain::map;
use crate::terrain::planet::Planet;
use crate::terrain::simulation::{Terrain, TerrainParams, ICE_FREE};
use rayon::prelude::*;

/// A patch's frame on the planet, and the mapping between the planet and the scene.
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
    pub fn of(domain: &Domain, exaggeration: f64) -> Frame {
        match &domain.layout {
            Layout::Patch { up, east, north, .. } => Frame { up: *up, east: *east, north: *north, radius: domain.radius, exaggeration },
            Layout::Sphere { .. } => panic!("a landscape is drawn at full scale only from a patch"),
        }
    }

    /// The planet's centre in the scene.
    pub fn centre(&self) -> Vector3<f64> { Vector3::new(0., -self.radius, 0.) }

    /// Direction from the planet's centre of the point x metres east and z north on the patch's
    /// tangent plane (the projection the domain's cells use).
    pub fn dir(&self, x: f64, z: f64) -> Vector3<f64> {
        (self.up * self.radius + self.east * x + self.north * z).normalize()
    }

    /// The tangent-plane coordinates (east, north) of direction `d`.
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
    /// Mesh vertices per simulation cell, along each side.
    pub detail: usize,
    /// Amplitude of the sub-grid relief, in cell spacings per unit slope.
    pub roughness: f64,
    /// How far the surroundings reach from the patch's centre, metres.
    pub horizon: f64,
    /// Ratio of successive spacings of the surroundings' grid.
    pub growth: f64,
    pub exaggeration: f64,
    /// Snowline and treeline, metres; None to take them from the climate.
    pub snowline: Option<f64>,
    pub treeline: Option<f64>,
    /// Rivers are drawn at least this wide, metres.
    pub min_river_width: f64,
}

impl Default for SceneryParams {
    fn default() -> SceneryParams {
        SceneryParams {
            detail: 4, roughness: 1., horizon: 300_000., growth: 1.08, exaggeration: 1.,
            snowline: None, treeline: None, min_river_width: 0.,
        }
    }
}

/// Cubic Catmull-Rom interpolation between b and c, at t in [0, 1].
fn catmull_rom(a: f64, b: f64, c: f64, d: f64, t: f64) -> f64 {
    b + 0.5 * t * (c - a + t * (2. * a - 5. * b + 4. * c - d + t * (3. * (b - c) + d - a)))
}

const ICE: u8 = 0;
const SEA_FLOOR: u8 = 1;
const ALLUVIUM: u8 = 2;
const LAVA: u8 = 3;
const LAND: u8 = 4;

/// One simulated patch, as drawn: its surface heights and what covers its ground.
pub struct Layer {
    n: usize,
    spacing: f64,
    /// Surface height (ground plus ice) at each of the patch's (n + 1)² cell corners.
    corners: Vec<f64>,
    domain: Domain,
    class: Vec<u8>,
    water: Option<WaterLines>,
    /// Width of the band inside the edge over which the layer blends into what is around it.
    blend: f64,
}

impl Layer {
    pub fn new(t: Terrain, min_river_width: f64) -> Layer {
        let Layout::Patch { n, spacing, .. } = t.domain.layout else { panic!("a landscape is drawn at full scale only from a patch") };
        let surface = t.surface();
        let corners = t.domain.corner_cells.iter()
            .map(|cells| cells.iter().map(|&c| surface[c]).sum::<f64>() / cells.len() as f64)
            .collect();
        let water = map::water_lines(&t, min_river_width);
        let class = (0..t.domain.len()).map(|i| {
            if t.ice[i] >= ICE_FREE { ICE }
            else if t.bed[i] < 0. { SEA_FLOOR }
            else if t.lava[i] > 1. && t.sediment[i] < map::ALLUVIUM { LAVA }
            else if t.sediment[i] > map::ALLUVIUM { ALLUVIUM }
            else { LAND }
        }).collect();
        // The simulation holds a patch's edge at its first topography, so the cells nearest
        // the edge are the least like a landscape: blend over a tenth of the patch, or at least
        // three cells.
        let half = n as f64 * spacing / 2.;
        Layer { n, spacing, corners, domain: t.domain, class, water, blend: (0.1 * half).max(3. * spacing) }
    }

    /// Half the patch's width, metres.
    pub fn half(&self) -> f64 { self.n as f64 * self.spacing / 2. }

    fn contains(&self, x: f64, z: f64) -> bool { x.abs() < self.half() && z.abs() < self.half() }

    /// How much the layer counts at (x, z): 1 within, falling to 0 at its edge.
    fn weight(&self, x: f64, z: f64) -> f64 {
        let half = self.half();
        1. - smoothstep(half - self.blend, half, x.abs().max(z.abs()))
    }

    fn corner(&self, i: i64, j: i64) -> f64 {
        let (i, j) = (i.clamp(0, self.n as i64) as usize, j.clamp(0, self.n as i64) as usize);
        self.corners[j * (self.n + 1) + i]
    }

    /// The simulated surface at (x, z), smoothly interpolated between the cells' corners.
    fn simulated(&self, x: f64, z: f64) -> f64 {
        let u = x / self.spacing + self.n as f64 / 2.;
        let v = z / self.spacing + self.n as f64 / 2.;
        let (i, j) = (u.floor() as i64, v.floor() as i64);
        let (fu, fv) = (u - i as f64, v - j as f64);
        let row = |jj: i64| catmull_rom(
            self.corner(i - 1, jj), self.corner(i, jj), self.corner(i + 1, jj), self.corner(i + 2, jj), fu);
        catmull_rom(row(j - 1), row(j), row(j + 1), row(j + 2), fv)
    }

    fn class_at(&self, x: f64, z: f64) -> u8 { self.class[point_cell(&self.domain, [x, z])] }
}

/// Heights of the landscape: the simulated patches, finest first, each nested in the next, and
/// the planet's noise topography around them all.
pub struct Relief {
    pub frame: Frame,
    pub layers: std::sync::Arc<Vec<Layer>>,
    planet: Planet,
    /// The finest wavelength drawn of the initial topography.
    finest: f64,
}

impl Relief {
    pub fn new(layers: std::sync::Arc<Vec<Layer>>, planet: &Planet, frame: Frame) -> Relief {
        let coarsest = layers.last().expect("no layers").spacing;
        Relief { frame, layers, planet: Planet::new(planet.params.clone()), finest: 2. * coarsest }
    }

    /// The planet's topography at time zero, around the patches.
    fn surroundings(&self, x: f64, z: f64) -> f64 {
        self.planet.elevation(&self.frame.dir(x, z), self.finest)
    }

    /// The large-scale height at (x, z), before sub-grid detail, and the cell spacing it is
    /// resolved to: each layer blending over its outermost cells into the next, the outermost
    /// into the surroundings.
    pub fn base(&self, x: f64, z: f64) -> (f64, f64) {
        let mut found: Option<(f64, f64)> = None;
        let mut rest = 1.;
        let (mut h, mut s) = (0., 0.);
        for l in self.layers.iter() {
            let w = l.weight(x, z) * rest;
            if w > 0. {
                h += w * l.simulated(x, z);
                s += w * l.spacing;
                rest -= w;
                found = Some((h, s));
            }
            if rest <= 0. { break; }
        }
        if rest > 0. {
            let coarsest = self.finest / 2.;
            h += rest * self.surroundings(x, z);
            s += rest * coarsest;
            found = Some((h, s));
        }
        found.unwrap()
    }
}

/// Coordinates along one axis of the mesh: `fine` apart across the patch, then growing by
/// `growth` each step out to `horizon` either side.
fn axis(half: f64, fine: f64, horizon: f64, growth: f64) -> Vec<f64> {
    let k = (2. * half / fine).round().max(1.) as usize;
    let step = 2. * half / k as f64;
    let mut outer = vec![];
    let (mut x, mut d) = (half, step);
    while x < horizon {
        d *= growth;
        x += d;
        outer.push(x);
    }
    let mut xs: Vec<f64> = outer.iter().rev().map(|x| -x).collect();
    xs.extend((0..=k).map(|i| -half + i as f64 * step));
    xs.extend(outer);
    xs
}

/// The landscape's mesh: the finest patch at `detail` vertices per cell, and its surroundings.
pub fn mesh(relief: &Relief, p: &SceneryParams, seed: u64) -> SmoothMesh {
    let first = &relief.layers[0];
    let fine = first.spacing / p.detail.max(1) as f64;
    let xs = axis(first.half(), fine, p.horizon, p.growth);
    let m = xs.len();
    let base: Vec<(f64, f64)> = (0..m * m).into_par_iter().map(|k| relief.base(xs[k % m], xs[k / m])).collect();
    // Sub-grid relief: octaves of noise from about two cells (of whichever layer resolves the
    // ground here) down to the mesh's spacing, in proportion to the slope at the scale of a cell.
    let noise = SeededNoise::new(seed.wrapping_add(0x5ce7e));
    let ridges = SeededNoise::new(seed.wrapping_add(0x41d9e));
    let longest = 4. * relief.finest;
    let heights: Vec<f64> = (0..m * m).into_par_iter().map(|k| {
        let (i, j) = (k % m, k / m);
        let (b, spacing) = base[k];
        let (i0, i1, j0, j1) = (i.saturating_sub(1), (i + 1).min(m - 1), j.saturating_sub(1), (j + 1).min(m - 1));
        let gx = (base[j * m + i1].0 - base[j * m + i0].0) / (xs[i1] - xs[i0]);
        let gz = (base[j1 * m + i].0 - base[j0 * m + i].0) / (xs[j1] - xs[j0]);
        let slope = gx.hypot(gz).min(1.5);
        // Spacing of the mesh here: an octave is only drawn where the mesh can carry it.
        let local = (xs[i1] - xs[i0]).max(xs[j1] - xs[j0]) / 2.;
        let (x, z) = (xs[i], xs[j]);
        let mut detail = 0.;
        let mut wavelength = longest;
        let mut octave = 0.;
        while wavelength >= 2. * fine {
            // The simulation only resolves features several cells across: below about eight
            // cells its slopes are smoothed by its own diffusion.
            let resolved = smoothstep(4. * spacing, 8. * spacing, wavelength);
            let carried = 1. - smoothstep(wavelength / 4., wavelength / 2., local);
            let w = (1. - resolved) * carried;
            if w > 0. {
                // Warped ridged noise: sharp crests and spurs between gullies, as on eroded
                // mountainsides, with a little plain noise to break them up.
                let q = Vector3::new(x, octave * 31.7, z) / wavelength;
                let warp = Vector3::new(noise.noise(q * 0.5 + Vector3::new(3.1, 0., 7.7)), 0., noise.noise(q * 0.5 + Vector3::new(9.4, 0., 1.3)));
                let r = (1. - ridges.noise(q + warp * 0.6).abs()).powi(2);
                let n = 1.4 * (r - 0.45) + 0.4 * noise.noise(q * 1.7);
                detail += w * p.roughness * wavelength / 4. * n;
            }
            wavelength /= 2.;
            octave += 1.;
        }
        b + detail * (0.05 + slope)
    }).collect();
    let positions: Vec<Vector3<f64>> = (0..m * m).into_par_iter()
        .map(|k| relief.frame.to_scene(&relief.frame.dir(xs[k % m], xs[k / m]), heights[k])).collect();
    let v = |i: usize, j: usize| j * m + i;
    let mut triangles = Vec::with_capacity(2 * (m - 1) * (m - 1));
    for j in 0..m - 1 {
        for i in 0..m - 1 {
            // x east and z north on the plane, so (i, j) -> (i + 1, j) -> (i + 1, j + 1) is
            // anticlockwise seen from above.
            triangles.push([v(i, j), v(i + 1, j), v(i + 1, j + 1)]);
            triangles.push([v(i, j), v(i + 1, j + 1), v(i, j + 1)]);
        }
    }
    SmoothMesh::from_indexed(&[IndexedMesh { positions, normals: vec![], triangles }])
}

/// What covers the ground, where.
pub struct Ground {
    frame: Frame,
    layers: std::sync::Arc<Vec<Layer>>,
    pub snowline: f64,
    pub treeline: f64,
    noise: PerlinNoise,
}

/// Albedos of real surfaces (linear, as the renderer's colours are).
const SNOW: [f64; 3] = [0.82, 0.85, 0.88];
const GLACIER_ICE: [f64; 3] = [0.45, 0.52, 0.58];
const ROCK: [f64; 3] = [0.20, 0.18, 0.16];
const SCREE: [f64; 3] = [0.26, 0.25, 0.23];
const FOREST: [f64; 3] = [0.028, 0.042, 0.022];
const GRASS: [f64; 3] = [0.07, 0.10, 0.04];
const MEADOW: [f64; 3] = [0.11, 0.11, 0.06];
const FIELDS: [f64; 3] = [0.13, 0.13, 0.07];
const GRAVEL: [f64; 3] = [0.24, 0.23, 0.21];
const SAND: [f64; 3] = [0.32, 0.28, 0.21];
const BASALT: [f64; 3] = [0.06, 0.055, 0.05];

fn colour(c: [f64; 3]) -> Color { Color::new(c[0], c[1], c[2]) }

impl Ground {
    /// The climate is that of terrain parameters `params` at `time` years.
    pub fn new(layers: std::sync::Arc<Vec<Layer>>, params: &TerrainParams, time: f64, frame: Frame, p: &SceneryParams) -> Ground {
        // The climate the glaciers ran in, at the end: permanent snow above their equilibrium
        // line; the treeline lies some 800 m below it (as in the Alps, about 2900 and 2100 m).
        let latitude = frame.up.y.clamp(-1., 1.).asin();
        let ela = params.glacier.as_ref().map_or(3000., |g| g.ela(latitude, time));
        let snowline = p.snowline.unwrap_or(ela);
        let treeline = p.treeline.unwrap_or(snowline - 800.);
        Ground { frame, layers, snowline, treeline, noise: PerlinNoise::shuffled(params.planet.seed.wrapping_add(77)) }
    }

    /// The finest layer at (x, z), if any.
    fn layer_at(&self, x: f64, z: f64) -> Option<&Layer> {
        self.layers.iter().find(|l| l.contains(x, z))
    }

    /// Whether scene point `p` is in a river or lake.
    pub fn wet(&self, p: &Vector3<f64>) -> bool {
        let (d, _) = self.frame.on_planet(p);
        let (x, z) = self.frame.tangent(&d);
        let Some(l) = self.layer_at(x, z) else { return false };
        let Some(water) = &l.water else { return false };
        water.wet(point_cell(&l.domain, [x, z]), [x, z])
    }

    /// The albedo at scene point `p`, whose surface normal is `normal`.
    pub fn albedo(&self, p: &Vector3<f64>, normal: &Vector3<f64>) -> Color {
        let up = (p - self.frame.centre()).normalize();
        let flat = normal.dot(&up);
        let (d, h) = self.frame.on_planet(p);
        let (x, z) = self.frame.tangent(&d);
        // What the simulation left here, looked up within half a cell of the point so that
        // over many samples the classes blend rather than show the grid.
        let class = match self.layer_at(x, z) {
            Some(l) => {
                let (x, z) = (x + (rand() - 0.5) * l.spacing, z + (rand() - 0.5) * l.spacing);
                match self.layer_at(x, z) { Some(l) => l.class_at(x, z), None => LAND }
            }
            None => LAND,
        };
        let n = |s: f64, o: f64| self.noise.fbm(x / s, o, z / s, 4, 0.5, 2.);
        let mix = |a: Color, b: Color, t: f64| { let t = t.clamp(0., 1.); a * (1. - t) + b * t };
        if class == LAVA { return colour(BASALT) * (0.8 + 0.4 * n(200., 0.9)); }
        if class == ICE {
            // Snow over the glacier's accumulation zone, bare ice below the snowline.
            let snow = smoothstep(self.snowline - 300., self.snowline + 100., h + (n(700., 0.2) - 0.5) * 300.);
            let ice = mix(colour(GLACIER_ICE), colour(SNOW), snow.max(0.3));
            // Ice doesn't hold to cliffs: the rock walls above and beside a glacier show.
            let stone = colour(ROCK) * (0.7 + 0.6 * n(250., 0.7));
            return mix(stone, ice, smoothstep(0.6, 0.8, flat));
        }
        // Vegetation, thinning upwards: forest in patches below the treeline, grass in the
        // valleys and on fields, alpine meadow above the trees.
        let tree = self.treeline + (n(1500., 0.3) - 0.5) * 400.;
        let forest = smoothstep(0.38, 0.55, n(2500., 0.5) + 0.25 * (1. - flat)) * (1. - smoothstep(tree - 150., tree + 50., h));
        let grass = mix(colour(GRASS), colour(MEADOW), smoothstep(tree - 300., tree + 300., h));
        let mut ground = mix(grass, colour(FOREST), forest);
        if class == ALLUVIUM {
            // Valley floors: fields and meadows, or gravel where nothing grows.
            let fields = mix(colour(GRASS), colour(FIELDS), smoothstep(0.4, 0.6, n(350., 0.8)));
            ground = mix(fields, colour(GRAVEL), smoothstep(tree, tree + 300., h));
        }
        // Beaches.
        ground = mix(ground, colour(SAND), 1. - smoothstep(2., 12., h + 8. * n(150., 0.4)));
        // Bare rock on steep slopes (about 40°), and scree and rock above the meadows.
        let high = smoothstep(tree + 300., tree + 900., h + (n(600., 0.6) - 0.5) * 300.);
        let rock = (1. - smoothstep(0.7, 0.85, flat)).max(high * (1. - 0.6 * smoothstep(0.92, 0.99, flat)));
        let stone = mix(colour(ROCK), colour(SCREE), smoothstep(0.85, 0.95, flat)) * (0.7 + 0.6 * n(250., 0.7));
        ground = mix(ground, stone, rock);
        // Snow lies above the snowline where slopes are gentle enough to hold it, and drifts
        // lower in sheltered places.
        let line = self.snowline + (n(900., 0.1) - 0.5) * 500.;
        let snow = smoothstep(line - 150., line + 150., h) * smoothstep(0.72, 0.88, flat + 0.15 * (n(120., 0.3) - 0.5));
        mix(ground, colour(SNOW), snow)
    }
}

pub struct Land {
    ground: std::sync::Arc<Ground>,
}

impl MaterialModel for Land {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let intersection = &intersection.facing(r);
        let albedo = self.ground.albedo(&intersection.point, &intersection.normal);
        scatter_bsdf(&LambertianBSDF { albedo, normal: intersection.normal }, r, &intersection.point, s)
    }
}

/// The landscape's materials: water in its rivers and lakes, land elsewhere.
pub struct LandscapeMedium {
    ground: std::sync::Arc<Ground>,
    land: Land,
    water: PhysicalOcean,
}

impl LandscapeMedium {
    pub fn new(ground: Ground) -> LandscapeMedium {
        let ground = std::sync::Arc::new(ground);
        let centre = ground.frame.centre();
        LandscapeMedium {
            land: Land { ground: ground.clone() },
            // Rivers are shallow and carry silt, sending back more green and brown than the sea.
            water: PhysicalOcean { water: Color::new(0.01, 0.018, 0.018), roughness: 0.004, center: Some(centre) },
            ground,
        }
    }
}

impl Medium for LandscapeMedium {
    fn material_at(&self, pt: Vector3<f64>) -> &(dyn MaterialModel + Sync + Send) {
        if self.ground.wet(&pt) { &self.water } else { &self.land }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> Frame {
        let d = Domain::patch(45., 7., 8, 500., 6.36e6);
        Frame::of(&d, 1.)
    }

    #[test]
    fn test_patch_centre_is_the_scene_origin_and_the_planet_curves_away() {
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

    #[test]
    fn test_axis_is_fine_over_the_patch_and_reaches_the_horizon() {
        let xs = axis(1000., 100., 50_000., 1.2);
        assert!(xs.windows(2).all(|w| w[1] > w[0]));
        assert!(xs[0] <= -50_000. && *xs.last().unwrap() >= 50_000.);
        assert!(xs.contains(&-1000.) && xs.iter().any(|&x| (x - 1000.).abs() < 1e-9));
        let inner = xs.iter().filter(|x| x.abs() <= 1000. + 1e-9).count();
        assert_eq!(inner, 21);
    }

    #[test]
    fn test_relief_is_continuous_across_its_layers() {
        let mut params = TerrainParams::default();
        params.planet.radius = 6.36e6;
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = planet.most_mountainous(2000);
        let fine = Terrain::new(Domain::patch(lat, lon, 16, 1000., params.planet.radius), params.clone());
        let coarse = Terrain::new(Domain::patch(lat, lon, 16, 4000., params.planet.radius), params);
        let frame = Frame::of(&fine.domain, 1.);
        let layers = std::sync::Arc::new(vec![Layer::new(fine, 0.), Layer::new(coarse, 0.)]);
        let r = Relief::new(layers.clone(), &planet, frame);
        for half in [layers[0].half(), layers[1].half()] {
            for z in [-5000., 0., 3000.] {
                let (inside, s0) = r.base(half - 1e-6, z);
                let (outside, s1) = r.base(half + 1e-6, z);
                assert!((inside - outside).abs() < 1e-2, "{} vs {}", inside, outside);
                assert!((s0 - s1).abs() < 1e-3);
            }
        }
        // At the centre, it is the finest layer.
        assert_eq!(r.base(0., 0.), (layers[0].simulated(0., 0.), 1000.));
    }
}
