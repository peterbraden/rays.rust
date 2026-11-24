//! # The ground's own detail, below the simulation's cells
//!
//! A landscape's simulation stops at its finest cells, tens to hundreds of metres across; the
//! clipmap adds noise below that, the same everywhere. What the ground looks like within a few
//! metres depends on what it is, and the simulation knows that at every point: which rock is
//! exposed at the point's own height and how hard it is, how much sediment lies on it and of
//! what kind (alluvium, till, sand, a playa's lake beds), whether ice covers it, how steep it
//! is. `ground` turns that into relief, metres outwards from the surface, for a landscape's
//! displacement near the camera (`graph`'s `ground` node; `object::full_scale_landscape` uses
//! it unless the scene gives a graph of its own), so that close up the ground follows the
//! geology without a scene placing anything by hand:
//!
//! - **Bare rock** (thin sediment, or slopes too steep to hold it) weathers by what it is.
//!   Hard beds stand out as ledges and soft ones are cut back between them (differential
//!   weathering, as on every cliff of bedded rock), and since the rock is the one at each
//!   point's height, the ledges follow the beds across the slopes. And each rock has its own
//!   texture: granite in rounded blocks and domes (sheeting and spheroidal weathering), gneiss
//!   and schist in ribs along their foliation, sandstone and quartzite in blocks cut by joints,
//!   limestone fluted by solution (karren), shale finely crumbled, basalt in columns. Steeper
//!   faces show more of it.
//! - **Scree**, sediment on steep slopes below the cliffs: a slope of stones.
//! - **Till and moraines**: hummocky ground (its boulders are the landscape's own,
//!   `object::boulders`).
//! - **Alluvium** on gentle ground: low swales and bars, fine gravel.
//! - **Sand**: smooth, drifted over the ground beneath (dunes are drawn larger, `scenery`).
//! - **Playas**: flat lake beds, cracked into polygons.
//! - **Glacier ice**: smooth, gently undulating.
//!
//! Each is blended by how much of it the fields say is there, so the ground changes as
//! gradually as the simulation does, and detail finer than the context's filter (the spacing
//! the surface is diced to) is left out, so it doesn't alias.

use crate::graph::{cells, fractal, CellStyle, Context, Field, FractalStyle, Metric};
use crate::noise::SeededNoise;
use crate::terrain::geology::Rock;

/// The most the detail raises or lowers the ground, metres, times its `amplitude`: a bound for
/// the displaced surface's bounding boxes.
pub const REACH: f64 = 4.;

/// How much of each kind of ground there is at a point.
struct Kinds { ice: f64, bare: f64, scree: f64, glacial: f64, alluvium: f64, blown: f64, pan: f64 }

impl Kinds {
    fn at(c: &Context, f: &dyn crate::graph::Fields, slope: f64) -> Kinds {
        let field = |x: Field| f.field(x, &c.p).unwrap_or(0.);
        let steep = |from: f64, to: f64| smoothstep(from, to, slope);
        let ice = smoothstep(1., 5., field(Field::Ice));
        let sediment = field(Field::Sediment);
        // Rock shows through thin sediment, and on slopes too steep to hold any.
        let bare = (1. - smoothstep(0.1, 1.5, sediment)).max(steep(38., 50.));
        let glacial = smoothstep(1., 4., field(Field::Till)) * smoothstep(0.5, 2., sediment);
        let blown = smoothstep(0.5, 3., field(Field::Sand));
        let pan = field(Field::Playa).clamp(0., 1.);
        let scree = steep(22., 32.) * (1. - glacial) * (1. - blown);
        let alluvium = ((1. - glacial) * (1. - blown) * (1. - pan) - scree).max(0.);
        Kinds { ice, bare, scree, glacial, alluvium, blown, pan }
    }
}

/// A bound on the ground's detail near `c`, metres either way, for ground no steeper than
/// `slope` degrees, whatever rock it is (a cliff crosses many beds): what `ground` gives within
/// a few metres of `c`, where the fields barely change, with a margin; for bounding a displaced
/// surface's cells more tightly than `REACH` does (sand varies by centimetres, granite by
/// metres).
pub fn bound(c: &Context, slope: f64, amplitude: f64) -> f64 {
    let Some(f) = c.fields else { return 0. };
    let k = Kinds::at(c, f, slope);
    // The most each kind gives (ledges and granite's blocks for rock).
    let rock = 0.7 * 1.5 + 1.2 + 1.0 * 0.6 + 0.25;
    let loose = k.scree * 0.5 + k.glacial * (1.5 + 0.9) + k.alluvium * 0.34 + k.blown * 0.15 + k.pan * 0.05 + 0.03;
    let h = (k.bare * rock + (1. - k.bare) * loose).min(REACH);
    amplitude.abs() * ((1. - k.ice) * h + k.ice * 0.4) * 1.25 + 0.02
}

/// The steepest a `fractal` of these parameters can be, metres of its value per metre: each
/// octave's gradient noise is no steeper than `NOISE_GRADIENT` per lattice unit, so octave k,
/// of weight r^k among the weights' sum and lacunarity 2, adds that times (2r)^k / scale.
/// Ridged octaves square the noise, which at most doubles its steepness, and the weighting of
/// each octave by the last at most doubles it again.
fn steepest(style: FractalStyle, scale: f64, smallest: f64, roughness: f64) -> f64 {
    const NOISE_GRADIENT: f64 = 3.;
    let octaves = ((scale / smallest.max(1e-9)).log2().floor().max(0.) as i32 + 1).min(40);
    let weights: f64 = (0..octaves).map(|k| roughness.powi(k)).sum();
    let gradients: f64 = (0..octaves).map(|k| (2. * roughness).powi(k)).sum();
    let style = match style { FractalStyle::Ridged => 4., _ => 1. };
    style * NOISE_GRADIENT * gradients / weights / scale
}

/// Bounds on the ground's detail anywhere within `half` metres of `c` (a displaced cell's
/// half-width), times `amplitude`, where the ground is no steeper than `slope` degrees and its
/// base rises and falls by up to `heights` metres about `c` (for the beds it crosses): tighter
/// than `bound`, for the smallest cells, where a ray grazing the ground would otherwise pass
/// through hundreds of boxes as tall as granite's blocks.
///
/// The detail's value at `c`, plus how far it can change: its smooth parts by their steepest
/// gradients (`steepest`) times the distance, which is widened to 2.5 cells to allow for
/// detail filtered out where the cell is diced coarsely (its finest octaves fade to their mean
/// below four times the filter); its parts that step (stones' edges; ledges and textures
/// where the rock changes from bed to bed within the cell's heights) by their whole
/// amplitudes. The weights of the kinds of ground are taken at their largest over slopes from
/// level to `slope`.
pub fn local_bound(c: &Context, slope: f64, half: f64, heights: (f64, f64), noise: &SeededNoise, seed: u32, amplitude: f64) -> (f64, f64) {
    let Some(f) = c.fields else { return (0., 0.) };
    let centre = ground(&Context { filter: 0., ..*c }, noise, seed, amplitude);
    let (k0, k1) = (Kinds::at(c, f, 0.), Kinds::at(c, f, slope));
    let most = |a: f64, b: f64| a.max(b);
    let d = 2.5 * 2. * half;
    // How far a fractal of unit amplitude can change over the distance: by its steepest
    // gradient, but never more than its whole range (2: Perlin's is -1 to 1).
    let lip = |style: FractalStyle, scale: f64, smallest: f64, roughness: f64| (steepest(style, scale, smallest, roughness) * d).min(2.);
    use FractalStyle::*;
    // Glaciers.
    let ice = most(k0.ice, k1.ice);
    let mut change = ice * 0.4 * lip(Perlin, 30., 2., 0.5);
    // Bare rock: does the rock change within the cell's heights?
    let bare = most(k0.bare, k1.bare);
    if bare > 0. {
        let up = f.vertical(&c.p).map_or(crate::na::Vector3::y(), |(up, _)| up);
        let rock_at = |dz: f64| f.field(Field::Rock, &(c.p + up * dz)).map_or(Rock::Sediment, |r| Rock::from_index(r.round() as usize));
        let rock = rock_at(0.);
        let changes = (0..=8).map(|k| heights.0 - 4. + (heights.1 - heights.0 + 8.) * k as f64 / 8.).any(|dz| rock_at(dz) != rock);
        let steep = smoothstep(25., 50., slope);
        let rock_change = if changes {
            // A ledge's step and any rock's texture.
            0.7 * 3. * steep + 2.2
        } else {
            // (Loose sediment has no beds to stand out.)
            let hardness = if rock == Rock::Sediment { 0. } else { f.field(Field::Hardness, &c.p).unwrap_or(0.).abs().min(1.5) };
            let texture = match rock {
                Rock::Granite => 1.2 + 1.0 * lip(Billow, 18., 0.5, 0.5),
                Rock::Gneiss => 0.8 * lip(Ridged, 5., 0.1, 0.55),
                Rock::Schist => 0.6 * lip(Ridged, 3., 0.05, 0.6),
                Rock::Sandstone | Rock::Quartzite => 0.4 + 0.3 + 0.3 * lip(Perlin, 6., 0.1, 0.5),
                Rock::Limestone => 0.5 * lip(Ridged, 1.5, 0.03, 0.6) + 0.3,
                Rock::Shale => 0.25 * lip(Perlin, 1., 0.03, 0.6),
                Rock::Basalt => 0.4 + 0.3,
                Rock::Sediment => 0.,
            };
            // The ledges' height changes only with the slope, which a cell's base hardly does.
            0.7 * hardness * steep + texture
        };
        // The roughness every rock has.
        change += bare * (rock_change + 0.25 * lip(Perlin, 4., 0.01, 0.62));
    }
    let loose = 1. - k0.bare.min(k1.bare);
    if loose > 0. {
        let scree = most(k0.scree, k1.scree);
        let glacial = most(k0.glacial, k1.glacial);
        let alluvium = most(k0.alluvium, k1.alluvium);
        let blown = most(k0.blown, k1.blown);
        let pan = most(k0.pan, k1.pan);
        change += loose * (scree * 0.12
            + glacial * 1.5 * lip(Perlin, 25., 2., 0.5)
            + alluvium * (0.3 * lip(Perlin, 30., 3., 0.5) + 0.04 * lip(Perlin, 0.4, 0.05, 0.6))
            + blown * 0.15 * lip(Perlin, 12., 2., 0.4)
            + pan * 0.05
            + 0.03 * lip(Perlin, 0.5, 0.01, 0.6));
    }
    let change = amplitude.abs() * (1.25 * change.min(2. * REACH) + 0.02);
    (centre - change, centre + change)
}

/// The ground's detail at `c` (see the module docs), metres outwards, times `amplitude`.
/// `noise` and `seed` vary it. Without fields (not on a landscape), nothing.
pub fn ground(c: &Context, noise: &SeededNoise, seed: u32, amplitude: f64) -> f64 {
    let Some(f) = c.fields else { return 0. };
    let slope = c.slope();
    let steep = |from: f64, to: f64| smoothstep(from, to, slope);
    // How much of each detail survives the filter: all of it for features four times the
    // filter, none below twice.
    let keep = |size: f64| if c.filter > 0. { smoothstep(2. * c.filter, 4. * c.filter, size) } else { 1. };
    let fbm = |style: FractalStyle, scale: f64, smallest: f64, roughness: f64| {
        if keep(scale) <= 0. { return 0. }
        let v = fractal(noise, style, scale, smallest, roughness, 2., false, c);
        // Ridged and billowed noise centred on zero.
        match style { FractalStyle::Perlin => v, _ => v - 0.4 }
    };
    let stones = |style: CellStyle, scale: f64, salt: u32| {
        let k = keep(scale);
        if k <= 0. { 0. } else { k * cells(seed ^ salt, style, scale, false, Metric::Euclidean, 1., c) }
    };
    // Blocks broken along joints and cleavage, faceted rather than rounded (cells measured
    // by Manhattan distance are diamond-faceted).
    let blocks = |scale: f64, salt: u32| {
        let k = keep(scale);
        if k <= 0. { 0. } else { k * cells(seed ^ salt, CellStyle::Stones, scale, false, Metric::Manhattan, 1., c) }
    };

    let Kinds { ice, bare, scree, glacial, alluvium, blown, pan } = Kinds::at(c, f, slope);
    let glacier = 0.4 * fbm(FractalStyle::Perlin, 30., 2., 0.5);
    if ice >= 1. {
        return amplitude * glacier;
    }
    let mut h = 0.;

    if bare > 0. {
        let rock = f.field(Field::Rock, &c.p).map(|r| Rock::ALL[(r.round() as usize).min(Rock::ALL.len() - 1)]).unwrap_or(Rock::Sediment);
        // The hardness averaged over a few metres of height, so a ledge ramps out over its
        // bed's edge rather than stepping out of a steep face (pushed out along the face's
        // normal by a sudden metre, the surface folds over itself in fins).
        let up = f.vertical(&c.p).map_or(crate::na::Vector3::y(), |(up, _)| up);
        let hardness = [-3., -1.5, 0., 1.5, 3.].iter()
            .map(|&dz: &f64| f.field(Field::Hardness, &(c.p + up * dz)).unwrap_or(0.)).sum::<f64>() / 5.;
        // Faces show more of their rock than level ground does.
        let face = 0.4 + 0.6 * steep(20., 55.);
        // Ledges of hard beds, recesses of soft, on slopes: the rock is the one at each
        // point's height, so they follow the beds.
        let ledges = if rock == Rock::Sediment { 0. } else { 0.4 * hardness.clamp(-1.5, 1.5) * steep(25., 50.) };
        let texture = match rock {
            // Rounded blocks and domes.
            // Domes, and corestones weathered out in places (tors), not everywhere.
            Rock::Granite => {
                let tors = smoothstep(0.3, 0.55, fbm(FractalStyle::Perlin, 40., 10., 0.5));
                0.8 * tors * blocks(4., 0x61) + 1.0 * fbm(FractalStyle::Billow, 18., 0.5, 0.5)
            }
            // Ribs along the foliation, and crumbling between.
            Rock::Gneiss => 0.8 * fbm(FractalStyle::Ridged, 5., 0.1, 0.55),
            Rock::Schist => 0.6 * fbm(FractalStyle::Ridged, 3., 0.05, 0.6),
            // Blocks bounded by joints: the joints are grooves.
            Rock::Sandstone | Rock::Quartzite => {
                let k = keep(0.5);
                let (groove, block) = joints(c, noise, 3.);
                // Blocks between the joints stand a little proud of each other.
                k * (-0.4 * groove + 0.3 * (block - 0.5)) + 0.3 * fbm(FractalStyle::Perlin, 6., 0.1, 0.5)
            }
            // Solution flutes and pits.
            // Solution flutes, and pans dissolved in patches.
            Rock::Limestone => {
                let pans = smoothstep(0.2, 0.5, fbm(FractalStyle::Perlin, 25., 5., 0.5));
                0.5 * fbm(FractalStyle::Ridged, 1.5, 0.03, 0.6) - 0.15 * pans * stones(CellStyle::Stones, 2.5, 0x81)
            }
            // Fine crumbling.
            Rock::Shale => 0.25 * fbm(FractalStyle::Perlin, 1., 0.03, 0.6),
            // Columns: their tops cells, their joints grooves.
            Rock::Basalt => 0.4 * stones(CellStyle::F1, 0.8, 0x91) - 0.3 * (1. - smoothstep(0., 0.1, stones(CellStyle::Edges, 0.8, 0x91))),
            Rock::Sediment => 0.,
        };
        // And rough at every scale, as rock is: a fractal from a few metres down to a
        // centimetre whose amplitude falls more slowly than its scale (rock surfaces' Hurst
        // exponents are about 0.7-0.8), so it has texture close up as well as from afar.
        let rough = 0.25 * fbm(FractalStyle::Perlin, 4., 0.01, 0.62);
        h += bare * (ledges + face * (texture + rough));
    }

    let loose = 1. - bare;
    if loose > 0. {
        let mut s = 0.;
        if scree > 0. {
            // A talus slope is a plane at the angle of repose, rough at the scale of its
            // stones, not lumpy.
            s += scree * (0.08 * blocks(0.5, 0xa1) + 0.04 * blocks(0.2, 0xa3));
        }
        if glacial > 0. {
            // Hummocks. (Its boulders are real ones, the landscape's own, `object::boulders`:
            // drawn in the ground they were smooth domes.)
            s += glacial * 1.5 * fbm(FractalStyle::Perlin, 25., 2., 0.5);
        }
        if alluvium > 0. {
            s += alluvium * (0.3 * fbm(FractalStyle::Perlin, 30., 3., 0.5) + 0.04 * fbm(FractalStyle::Perlin, 0.4, 0.05, 0.6));
        }
        if blown > 0. {
            s += blown * 0.15 * fbm(FractalStyle::Perlin, 12., 2., 0.4);
        }
        if pan > 0. {
            // Polygons a metre or two across, the cracks between them a few centimetres wide
            // and deep.
            s += pan * -0.03 * (1. - smoothstep(0., 0.025, stones(CellStyle::Edges, 1.5, 0xc1)));
        }
        // Loose ground is rough too, at the scale of its clods and stones.
        s += 0.03 * fbm(FractalStyle::Perlin, 0.5, 0.01, 0.6);
        h += loose * s;
    }
    amplitude * ((1. - ice) * h.clamp(-REACH, REACH) + ice * glacier)
}

/// How deep in a joint `c` is, 0 to 1, and a number from 0 to 1 for the block it is on: two
/// sets of nearly parallel vertical joints, at right angles to each other (as joints in bedded
/// rock form, across the beds and each other), about `spacing` metres apart, the spacing and
/// the joints' line wandering a little; each joint a rounded groove a hand or two wide where
/// weathering has opened it.
fn joints(c: &Context, noise: &SeededNoise, spacing: f64) -> (f64, f64) {
    let p = c.p;
    // The sets' directions turn slowly across the country.
    let angle = 0.6 * noise.noise(p / 300. + crate::na::Vector3::new(4.1, 0.7, 2.3));
    let (a, b) = ((angle.cos(), angle.sin()), (-angle.sin(), angle.cos()));
    let across = |dir: (f64, f64), s: f64, o: f64| {
        let u = (p.x * dir.0 + p.z * dir.1) / s + 0.25 * noise.noise(p / (4. * s) + crate::na::Vector3::new(o, 1.3, 0.2));
        let d = (u - u.round()).abs() * s;
        // Some joints aren't there at all; the rest die out and pick up again along their
        // length, every metre or two, and open unevenly.
        let mut k = (u.round() as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (o.to_bits());
        k ^= k >> 31;
        k = k.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        let present = ((k >> 11) as f64 / (1u64 << 53) as f64) > 0.3;
        let along = noise.noise(p / (0.5 * s) + crate::na::Vector3::new(o + 5.1, 2.7, 0.9));
        let open = if present { smoothstep(0., 0.3, along) } else { 0. };
        (open * (1. - smoothstep(0.03, 0.15, d)), u.floor() as i64)
    };
    let ((g1, i), (g2, j)) = (across(a, spacing, 7.7), across(b, 1.6 * spacing, 3.1));
    let mut h = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ (j as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    h ^= h >> 31;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 29;
    (g1.max(g2), (h >> 11) as f64 / (1u64 << 53) as f64)
}

fn smoothstep(lo: f64, hi: f64, x: f64) -> f64 {
    let t = ((x - lo) / (hi - lo)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;

    /// Fields the same everywhere.
    #[derive(Debug)]
    struct Uniform { sediment: f64, rock: Rock, ice: f64, till: f64, sand: f64, playa: f64 }

    impl crate::graph::Fields for Uniform {
        fn field(&self, f: Field, _p: &Vector3<f64>) -> Option<f64> {
            Some(match f {
                Field::Sediment => self.sediment,
                Field::Ice => self.ice,
                Field::Till => self.till,
                Field::Sand => self.sand,
                Field::Playa => self.playa,
                Field::Rock => self.rock.index() as f64,
                Field::Hardness => -self.rock.scores().0,
                _ => 0.,
            })
        }
    }

    fn bare(rock: Rock) -> Uniform { Uniform { sediment: 0., rock, ice: 0., till: 0., sand: 0., playa: 0. } }

    /// The spread (standard deviation) of the detail over a patch of ground with this normal.
    fn spread(f: &Uniform, normal: Vector3<f64>, filter: f64) -> f64 {
        let noise = SeededNoise::new(1);
        let v: Vec<f64> = (0..2500).map(|k| {
            let p = Vector3::new((k % 50) as f64 * 0.37, (k / 50) as f64 * 0.11, (k / 50) as f64 * 0.37);
            ground(&Context::new(p, normal, filter).with_fields(Some(f)), &noise, 1, 1.)
        }).collect();
        let mean = v.iter().sum::<f64>() / v.len() as f64;
        (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt()
    }

    #[test]
    fn test_local_bounds_hold_the_detail() {
        // For every kind of ground and rock, cells from 6 cm to 2 m across, the detail anywhere
        // in the cell, at any filter its dicing uses, lies within the cell's bound.
        let noise = SeededNoise::new(4);
        let steep: Vector3<f64> = Vector3::new(0.6, 0.8, 0.).normalize();
        let slope = steep.y.acos().to_degrees();
        let mut grounds: Vec<Uniform> = Rock::ALL.iter().map(|&r| bare(r)).collect();
        grounds.push(Uniform { sediment: 5., till: 5., ..bare(Rock::Sediment) });
        grounds.push(Uniform { sediment: 5., sand: 5., ..bare(Rock::Sediment) });
        grounds.push(Uniform { sediment: 5., playa: 1., ..bare(Rock::Sediment) });
        grounds.push(Uniform { sediment: 5., ..bare(Rock::Sediment) });
        grounds.push(Uniform { ice: 3., ..bare(Rock::Granite) });
        let mut rng = crate::geometry::Pcg32::new(5, 1);
        let mut worst: f64 = 0.;
        for f in &grounds {
            for &cell in &[0.0625, 0.25, 1., 2.] {
                for _ in 0..20 {
                    let centre = Vector3::new(rng.next_f64() * 100., rng.next_f64() * 10., rng.next_f64() * 100.);
                    let c = Context::new(centre, steep, cell).with_fields(Some(f));
                    let (lo, hi) = local_bound(&c, slope + 5., cell / 2., (-0.5, 0.5), &noise, 3, 1.);
                    for _ in 0..40 {
                        let p = centre + Vector3::new((rng.next_f64() - 0.5) * cell, 0., (rng.next_f64() - 0.5) * cell);
                        let filter = cell * 0.5f64.powi((rng.next_f64() * 9.) as i32);
                        let v = ground(&Context::new(p, steep, filter).with_fields(Some(f)), &noise, 3, 1.);
                        assert!(v >= lo - 1e-9 && v <= hi + 1e-9, "{:?} cell {}: {} outside [{}, {}]", f.rock, cell, v, lo, hi);
                    }
                    if cell <= 0.0625 && f.sand > 0. { worst = worst.max(hi - lo); }
                }
            }
        }
        // And on sand, small cells' bounds are a few tens of centimetres, not metres (on this
        // 42° slope, steeper than sand rests, a third of the ground counts as bare rock).
        assert!(worst < 0.5, "{}", worst);
    }

    #[test]
    fn test_the_ground_has_the_detail_of_what_it_is() {
        let level = Vector3::y();
        let steep = Vector3::new(0.8, 0.6, 0.).normalize();
        let alluvium = Uniform { sediment: 5., ..bare(Rock::Sediment) };
        // Bare rock is rougher than alluvium; granite's blocks rougher than shale's crumbs.
        let (rock, flat) = (spread(&bare(Rock::Granite), steep, 0.), spread(&alluvium, level, 0.));
        assert!(rock > 2. * flat, "granite {} alluvium {}", rock, flat);
        assert!(spread(&bare(Rock::Granite), steep, 0.) > spread(&bare(Rock::Shale), steep, 0.));
        // Hard rock stands out of a slope, soft rock is cut back.
        let at = |f: &Uniform| ground(&Context::new(Vector3::new(1., 2., 3.), steep, 0.).with_fields(Some(f)), &SeededNoise::new(1), 1, 1.);
        let mean = |r: Rock| (0..400).map(|k| {
            let f = bare(r);
            ground(&Context::new(Vector3::new(k as f64 * 0.7, 0., k as f64 * 0.3), steep, 0.).with_fields(Some(&f)), &SeededNoise::new(1), 1, 1.)
        }).sum::<f64>() / 400.;
        assert!(mean(Rock::Quartzite) > mean(Rock::Shale) + 0.5, "{} {}", mean(Rock::Quartzite), mean(Rock::Shale));
        let _ = at(&bare(Rock::Basalt));
        // Seen coarsely (a 5 m filter), shale's crumbs are gone.
        assert!(spread(&bare(Rock::Shale), steep, 5.) < 0.2 * spread(&bare(Rock::Shale), steep, 0.));
        // Glaciers are smooth; nothing is beyond the bound.
        assert!(spread(&Uniform { ice: 50., ..bare(Rock::Granite) }, level, 0.) < 0.2);
        for f in [bare(Rock::Granite), Uniform { till: 10., sediment: 10., ..bare(Rock::Sediment) }, Uniform { playa: 1., sediment: 5., ..bare(Rock::Sediment) }] {
            assert!(spread(&f, steep, 0.) <= REACH);
        }
    }
}
