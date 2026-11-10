//! # Maps
//!
//! A shaded relief map of a terrain, for looking at the simulation without ray tracing it:
//! elevation as hypsometric tints, lit from the north-west (the cartographic convention), with
//! ice, rivers and sea drawn over it. A patch is drawn cell for cell, north up; a whole planet
//! as an equirectangular (latitude-longitude) map.

use crate::na::Vector3;
use crate::terrain::domain::Layout;
use crate::terrain::simulation::{Terrain, ICE_FREE};
use crate::terrain::channel::{point_cell, ChannelParams, WaterLines};

/// Drainage area above which a cell is drawn as river, square metres...
pub const RIVER_AREA: f64 = 5e7;

/// ... or, on coarse grids, a multiple of the cell's own area.
pub fn is_river(t: &Terrain, i: usize) -> bool {
    t.drainage[i] > RIVER_AREA.max(20. * t.domain.area[i])
}

fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    let t = t.clamp(0., 1.);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// Sediment thicker than this (metres) is drawn as alluvium.
pub const ALLUVIUM: f64 = 2.;
pub const ALLUVIUM_COLOUR: [f64; 3] = [0.80, 0.74, 0.52];
pub const LAVA_COLOUR: [f64; 3] = [0.16, 0.14, 0.14];
pub const RIVER_COLOUR: [f64; 3] = [0.20, 0.40, 0.75];

/// A patch's rivers and lakes as lines, at least `min_width` metres wide. None for a planet.
pub fn water_lines(t: &Terrain, min_width: f64) -> Option<WaterLines> {
    if !matches!(t.domain.layout, Layout::Patch { .. }) { return None; }
    let outlets = t.outlets();
    let c = t.params.channels.as_ref().map_or(ChannelParams::default().width_coefficient, |c| c.width_coefficient);
    let rivers: Vec<(usize, usize, f64)> = (0..t.domain.len())
        .filter(|&i| is_river(t, i) && !outlets[i] && t.ice[i] < ICE_FREE && t.receiver[i] != i)
        .map(|i| (i, t.receiver[i], (c * t.drainage[i].sqrt()).max(min_width)))
        .collect();
    let mut channels = t.rivers.channels.clone();
    for ch in &mut channels { ch.width = ch.width.max(min_width); }
    let mut oxbows = t.rivers.oxbows.clone();
    for o in &mut oxbows { o.width = o.width.max(min_width); }
    Some(WaterLines::new(&t.domain, &channels, &oxbows, &rivers))
}

/// Hypsometric tint for land at `h` metres.
pub fn land_colour(h: f64) -> [f64; 3] {
    let stops = [
        (0., [0.30, 0.45, 0.25]),
        (800., [0.55, 0.60, 0.35]),
        (1800., [0.55, 0.45, 0.32]),
        (3000., [0.50, 0.48, 0.46]),
        (4500., [0.85, 0.85, 0.85]),
    ];
    for w in stops.windows(2) {
        if h < w[1].0 { return mix(w[0].1, w[1].1, (h - w[0].0) / (w[1].0 - w[0].0)); }
    }
    stops[stops.len() - 1].1
}

/// The colour of cell `i`, before shading.
fn cell_colour(t: &Terrain, i: usize) -> [f64; 3] {
    if t.bed[i] < 0. {
        return mix([0.15, 0.35, 0.55], [0.03, 0.08, 0.25], -t.bed[i] / 4000.);
    }
    if t.ice[i] >= ICE_FREE {
        return mix([0.80, 0.88, 0.95], [0.95, 0.97, 1.0], t.ice[i] / 300.);
    }
    // Patches draw rivers as lines (`water_lines`).
    if is_river(t, i) && matches!(t.domain.layout, Layout::Sphere { .. }) {
        return RIVER_COLOUR;
    }
    if t.sediment[i] > ALLUVIUM {
        return mix(land_colour(t.bed[i]), ALLUVIUM_COLOUR, 0.6);
    }
    if t.lava[i] > 1. {
        return mix(land_colour(t.bed[i]), LAVA_COLOUR, 0.7);
    }
    land_colour(t.bed[i])
}

/// Write a shaded relief map of `t` to `path`, about `width` pixels wide (a patch is drawn at a
/// whole number of pixels per cell).
pub fn write_map(t: &Terrain, path: &str, width: usize) {
    let (w, h, cells, spacing): (usize, usize, Vec<usize>, f64) = match &t.domain.layout {
        Layout::Patch { n, spacing, .. } => {
            let cells = (0..n * n).map(|p| {
                let (x, y) = (p % n, p / n);
                (n - 1 - y) * n + x
            }).collect();
            (*n, *n, cells, *spacing)
        }
        Layout::Sphere { .. } => {
            let (w, h) = (width, width / 2);
            let cells = (0..w * h).map(|p| {
                let lon = (p % w) as f64 / w as f64 * std::f64::consts::TAU - std::f64::consts::PI;
                let lat = std::f64::consts::FRAC_PI_2 - ((p / w) as f64 + 0.5) / h as f64 * std::f64::consts::PI;
                t.domain.cell_at(&Vector3::new(lat.cos() * lon.cos(), lat.sin(), -lat.cos() * lon.sin()))
            }).collect();
            (w, h, cells, t.domain.radius * std::f64::consts::PI / h as f64)
        }
    };
    let surface = t.surface();
    let height = |x: usize, y: usize| surface[cells[y.min(h - 1) * w + x.min(w - 1)]].max(0.);
    let mut img = image::RgbImage::new(w as u32, h as u32);
    for y in 0..h {
        for x in 0..w {
            // Light from the north-west, 45 degrees up.
            let dx = (height(x + 1, y) - height(x.saturating_sub(1), y)) / (2. * spacing);
            let dy = (height(x, y + 1) - height(x, y.saturating_sub(1))) / (2. * spacing);
            let normal = Vector3::new(-2. * dx, 2. * dy, 1.).normalize();
            let light = Vector3::new(-1., 1., 1.4).normalize();
            let shade = (0.35 + 0.75 * normal.dot(&light)).clamp(0., 1.3);
            let c = cell_colour(t, cells[y * w + x]);
            let shade = if t.bed[cells[y * w + x]] < 0. { 1. } else { shade };
            let px = |v: f64| ((v * shade).clamp(0., 1.) * 255.) as u8;
            img.put_pixel(x as u32, y as u32, image::Rgb([px(c[0]), px(c[1]), px(c[2])]));
        }
    }
    let up = (width / w).max(1) as u32;
    let mut img = if up > 1 {
        image::imageops::resize(&img, w as u32 * up, h as u32 * up, image::imageops::FilterType::Nearest)
    } else { img };
    // Rivers and lakes as lines, at least a pixel wide.
    if let Some(lines) = water_lines(t, spacing / up as f64) {
        let n = w as f64;
        for py in 0..h as u32 * up {
            for px in 0..w as u32 * up {
                let x = ((px as f64 + 0.5) / up as f64 - n / 2.) * spacing;
                let z = (n / 2. - (py as f64 + 0.5) / up as f64) * spacing;
                if lines.wet(point_cell(&t.domain, [x, z]), [x, z]) {
                    let c = RIVER_COLOUR.map(|v| (v * 255.) as u8);
                    img.put_pixel(px, py, image::Rgb(c));
                }
            }
        }
    }
    img.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::domain::Domain;
    use crate::terrain::planet::Planet;
    use crate::terrain::simulation::TerrainParams;

    fn env(k: &str, def: f64) -> f64 {
        std::env::var(k).map(|v| v.parse().unwrap()).unwrap_or(def)
    }

    /// Writes maps of a patch through time, for looking at:
    /// TERRAIN_MAP_DIR=/some/dir cargo test --release write_patch_maps -- --ignored --nocapture
    #[test]
    #[ignore]
    fn write_patch_maps() {
        let dir = std::env::var("TERRAIN_MAP_DIR").unwrap();
        let mut params = TerrainParams::default();
        params.planet.seed = env("SEED", 0.) as u64;
        params.step = env("STEP", 100.);
        let planet = Planet::new(params.planet.clone());
        let (lat, lon) = if std::env::var("VOLCANIC").is_ok() { planet.most_volcanic(20000) }
            else if std::env::var("COASTAL").is_ok() { planet.most_coastal(20000) }
            else { planet.most_mountainous(20000) };
        let n = env("CELLS", 128.) as usize;
        let spacing = env("SPACING", 400.);
        println!("patch at {:.2}, {:.2}", lat, lon);
        let mut t = Terrain::new(Domain::patch(lat, lon, n, spacing, params.planet.radius), params);
        let end = env("END", 100000.);
        let frames = env("FRAMES", 4.) as usize;
        for f in 0..=frames {
            let start = std::time::Instant::now();
            let target = end * f as f64 / frames as f64;
            while t.time < target {
                let before = t.ice_steps;
                let iterations = t.ice_iterations;
                let s = std::time::Instant::now();
                t.run_to((t.time + 1000.).min(target));
                if std::env::var("VERBOSE").is_ok() {
                    let k = (0..t.domain.len()).max_by(|&a, &b| t.ice[a].total_cmp(&t.ice[b])).unwrap();
                    println!("  thickest {} ({}, {}): bed {:.0} ice {:.0} slide {:.0}; edges {:?}", k, k % n, k / n, t.bed[k], t.ice[k], t.sliding[k],
                        t.domain.edges(k).iter().map(|l| (t.bed[l.to].round(), t.ice[l.to].round())).collect::<Vec<_>>());
                    println!("  t {} ice steps {} ({} iterations) in {:.1?} max ice {:.0} max slide {:.0}", t.time, t.ice_steps - before, t.ice_iterations - iterations, s.elapsed(),
                        t.ice.iter().cloned().fold(0., f64::max), t.sliding.iter().cloned().fold(0., f64::max));
                }
            }
            if std::env::var("VERBOSE").is_ok() {
                let k = (0..t.domain.len()).max_by(|&a, &b| t.sediment[a].total_cmp(&t.sediment[b])).unwrap();
                println!("  deepest sediment {} ({}, {}): ground {:.0} sediment {:.0} ice {:.0} drainage {:.2e} flux {:.2e}; neighbours {:?}",
                    k, k % n, k / n, t.bed[k], t.sediment[k], t.ice[k], t.drainage[k], t.sediment_flux[k],
                    t.domain.edges(k).iter().map(|l| (t.bed[l.to].round(), t.sediment[l.to].round(), t.ice[l.to].round())).collect::<Vec<_>>());
            }
            let land = t.bed.iter().filter(|&&b| b > 0.).count();
            let iced = t.ice.iter().filter(|&&h| h > ICE_FREE).count();
            let max = t.bed.iter().cloned().fold(f64::MIN, f64::max);
            println!("  {} channels, {} ox-bow lakes; min {:.0}; sediment > 2 m in {} cells, deepest {:.0} m; lava in {} cells, thickest {:.0} m",
                t.rivers.channels.len(), t.rivers.oxbows.len(),
                t.bed.iter().cloned().fold(f64::MAX, f64::min),
                t.sediment.iter().filter(|&&s| s > ALLUVIUM).count(), t.sediment.iter().cloned().fold(0., f64::max),
                t.lava.iter().filter(|&&s| s > 1.).count(), t.lava.iter().cloned().fold(0., f64::max));
            println!("t {:>8} in {:.1?}: max {:.0} m, land {}, ice {} cells, max ice {:.0} m",
                t.time, start.elapsed(), max, land, iced, t.ice.iter().cloned().fold(0., f64::max));
            write_map(&t, &format!("{}/patch-{:03}.png", dir, f), 512);
        }
    }

    #[test]
    #[ignore]
    fn write_planet_maps() {
        let dir = std::env::var("TERRAIN_MAP_DIR").unwrap();
        let mut params = TerrainParams::default();
        params.planet.seed = env("SEED", 0.) as u64;
        params.step = env("STEP", 1000.);
        let mut t = Terrain::new(Domain::sphere(env("CELLS", 64.) as usize, params.planet.radius), params);
        let end = env("END", 100000.);
        let frames = env("FRAMES", 4.) as usize;
        for f in 0..=frames {
            let start = std::time::Instant::now();
            t.run_to(end * f as f64 / frames as f64);
            println!("t {:>8} in {:.1?}: ice {} cells", t.time, start.elapsed(), t.ice.iter().filter(|&&h| h > ICE_FREE).count());
            write_map(&t, &format!("{}/planet-{:03}.png", dir, f), 1024);
        }
    }
}
