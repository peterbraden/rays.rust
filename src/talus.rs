//! # Talus and scree
//!
//! The rock fallen from a cliff, as a graph node (`graph.rs`): blocks broken along joints and
//! bedding, so angular, with flat faces, sharp edges and corners knocked off, lying among
//! smaller ones in a heap of every size down to grit.
//!
//! - **Shapes.** Each block fills most of a cell of a jittered lattice in plan, so its outline is
//!   the cell's Voronoi polygon, with straight sides meeting at corners. Its top is one or two
//!   tilted planes (a block lying askew, or a ridge where two faces meet); its sides fall
//!   steeply from the outline; a few random planes cut off edges and corners (a block's
//!   arrises are the first thing to break); some blocks are split by a crack; each is buried
//!   by a random depth. Sandstone, limestone and basalt talus is made of such polyhedra, with
//!   the rounding of a boulder only after long weathering.
//! - **Sizes.** Blocks come in layers, each half the size of the one above, from `largest` down
//!   to `smallest` (metres). Fragments of broken rock follow a power law: the number larger
//!   than a size D goes as D^-b, b about 2 to 3 for rockfall (`exponent`, default 2.5; Ruiz-
//!   Carulla, Corominas and Mavrouli, "A methodology to obtain the block size distribution of
//!   fragmental rockfall deposits", Landslides 2015, doi:10.1007/s10346-015-0600-5). So the
//!   area covered by each halving of size grows by 2^(b-2): a few big blocks among many small.
//!   `cover` is the fraction of the ground the largest layer covers where the talus is
//!   thickest; smaller layers cover more, up to 85%.
//! - **Sorting.** A talus is sorted by fall: big blocks carry momentum and roll to its toe,
//!   small ones stop near its apex under the cliff, so it coarsens downslope (Kirkby and
//!   Statham, "Surface stone movement and scree formation", Journal of Geology 1975,
//!   doi:10.1086/628085). `position` (a node in [0, 1]) is where a point lies on the talus: 0
//!   beyond its toe, 1 at its apex. With `sorting` (0 to 1, default 0.7) the biggest blocks
//!   thin out towards the apex, and only big blocks reach out over the ground near the toe.
//!   `mask` (a node in [0, 1], default 1) thins all of them, eg. off the cliff.
//! - `height`: a block's thickness relative to its width (default 0.5: slabby, as bedded rock
//!   breaks); `seed`.
//! - `output`: `"height"` (default: metres, to displace by), `"mask"` (1 on a block, 0
//!   between: a `mix` material's `amount`, to put rock on the blocks and soil between) or
//!   `"id"` (a random number in (0, 1] for each block, 0 between, to vary the blocks'
//!   colour).
//!
//! Blocks stand up from the ground, so displace by talus vertically (a terrain's
//! `"vertical_displacement"`): along the normal they'd lean downhill, and their tops wouldn't
//! lie above where they stand. A material's mask then finds the same blocks at the shading
//! point (`graph::Context::lifted`): each block's `position` and `mask` are evaluated where it
//! stands, its own height below the point. Layers finer than twice the filter (the spacing of
//! a displaced surface's vertices) are left out.
use crate::graph::Context;
use crate::na::{Vector2, Vector3};
use serde_json::Value;

/// The share of the ground any one layer may cover.
const MOST_COVER: f64 = 0.85;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Output { Height, Mask, Id }

#[derive(Clone)]
pub struct Talus<N> {
    seed: u32,
    largest: f64,
    pub smallest: f64,
    exponent: f64,
    cover: f64,
    sorting: f64,
    height: f64,
    position: Option<N>,
    mask: Option<N>,
    output: Output,
}

/// Hash integers to [0, 1).
fn hash(i: i64, j: i64, layer: u32, seed: u32, k: u32) -> f64 {
    let mut h = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (j as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ ((layer as u64) << 32 | k as u64).wrapping_mul(0x1656_67b1_9e37_79f9)
        ^ (seed as u64).wrapping_mul(0x27d4_eb2f_1656_67c5);
    h ^= h >> 31;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 29;
    h = h.wrapping_mul(0x94d0_49bb_1331_11eb);
    h ^= h >> 32;
    (h >> 11) as f64 / (1u64 << 53) as f64
}

fn smoothstep(lo: f64, hi: f64, x: f64) -> f64 {
    let t = ((x - lo) / (hi - lo)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}

/// The smaller of a and b, rounded over `k` where they meet (Inigo Quilez's polynomial smooth
/// minimum); exactly the smaller elsewhere, so faces stay flat.
fn smooth_min(a: f64, b: f64, k: f64) -> f64 {
    let h = (k - (a - b).abs()).max(0.) / k;
    a.min(b) - h * h * k * 0.25
}

/// A block of a layer at a point, were it there: its height there (metres), its size (the
/// layer's), its chance of being there (compared with `coverage`), and its random identity.
struct Block {
    height: f64,
    size: f64,
    chance: f64,
    id: f64,
}

impl<N> Talus<N> {
    pub fn parse(o: &Value, seed: u64, mut node: impl FnMut(&str) -> Result<Option<N>, String>) -> Result<Talus<N>, String> {
        let num = |k: &str, d: f64| -> Result<f64, String> {
            match &o[k] {
                Value::Null => Ok(d),
                v => v.as_f64().ok_or_else(|| format!("talus' \"{}\" must be a number", k)),
            }
        };
        let largest = num("largest", 4.)?;
        let smallest = num("smallest", largest / 32.)?;
        if !(largest > 0. && smallest > 0. && smallest <= largest) {
            return Err("talus needs 0 < smallest <= largest".into());
        }
        let output = match o["output"].as_str().unwrap_or("height") {
            "height" => Output::Height,
            "mask" => Output::Mask,
            "id" => Output::Id,
            s => return Err(format!("unknown talus output \"{}\": height, mask or id", s)),
        };
        Ok(Talus {
            seed: seed as u32,
            largest,
            smallest,
            exponent: num("exponent", 2.5)?,
            cover: num("cover", 0.25)?.clamp(0., 1.),
            sorting: num("sorting", 0.7)?.clamp(0., 1.),
            height: num("height", 0.5)?.max(0.),
            position: node("position")?,
            mask: node("mask")?,
            output,
        })
    }

    pub fn range(&self) -> (f64, f64) {
        match self.output {
            Output::Height => (0., self.largest * self.top()),
            Output::Mask | Output::Id => (0., 1.),
        }
    }

    /// The most a block rises, relative to its cell.
    fn top(&self) -> f64 {
        1.5 * self.height + 0.25
    }

    /// The layers' cell sizes, metres, largest first.
    fn sizes(&self) -> impl Iterator<Item = f64> + '_ {
        let n = (self.largest / self.smallest).log2().floor() as i32 + 1;
        (0 .. n).map(move |k| self.largest * 0.5f64.powi(k))
    }

    /// The share of the ground a layer of blocks `size` across covers at `position` on the
    /// talus.
    fn coverage(&self, size: f64, position: f64) -> f64 {
        if position <= 0. {
            return 0.;
        }
        let r = size / self.largest;
        let base = (self.cover * r.powf(2. - self.exponent)).min(MOST_COVER);
        // Near the toe only the biggest blocks: each size reaches out to a position that is
        // lower the bigger it is.
        let reach = (1. - self.sorting) * 0.3 + self.sorting * (0.04 + 0.5 * (1. - r));
        let spread = smoothstep(0., reach.max(1e-3), position);
        // Towards the apex the biggest thin out: the largest size there, relative to the largest.
        let biggest = 1. - self.sorting * position * (1. - (self.smallest / self.largest).min(1.));
        let sorted = 1. - smoothstep(0., 1., (r / biggest.max(1e-6)).log2());
        base * spread * sorted
    }

    /// The block of one layer nearest to `q` (plan, metres), if `q` is on it, whether or not
    /// the talus there has a block of its size.
    fn block(&self, layer: u32, size: f64, q: Vector2<f64>) -> Option<Block> {
        let u = q / size;
        let (cx, cz) = (u.x.floor() as i64, u.y.floor() as i64);
        let seed = self.seed;
        let point = |i: i64, j: i64| Vector2::new(i as f64 + 0.15 + 0.7 * hash(i, j, layer, seed, 1), j as f64 + 0.15 + 0.7 * hash(i, j, layer, seed, 2));
        let mut pts = [(0i64, 0i64, Vector2::zeros()); 9];
        let (mut nearest, mut best) = (0, f64::INFINITY);
        for (n, (di, dj)) in (-1 ..= 1).flat_map(|a| (-1 ..= 1).map(move |b| (a, b))).enumerate() {
            let (i, j) = (cx + di, cz + dj);
            let f = point(i, j);
            pts[n] = (i, j, f);
            let d = (f - u).norm_squared();
            if d < best {
                best = d;
                nearest = n;
            }
        }
        let (i, j, f) = pts[nearest];
        let r = |k: u32| hash(i, j, layer, seed, k + 3);
        // The distance in from the cell's edges: to the nearest bisector with a neighbour.
        let mut edge = f64::INFINITY;
        for (n, &(_, _, g)) in pts.iter().enumerate() {
            if n != nearest {
                let d = g - f;
                edge = edge.min(((u - g).norm_squared() - (u - f).norm_squared()) / (2. * d.norm()));
            }
        }
        let v = u - f;
        let thick = self.height * (0.5 + 0.9 * r(0));
        // The gap left round the block within its cell, and how steeply its sides fall.
        let gap = 0.04 + 0.22 * r(1);
        let side = (2.5 + 3. * r(2)) * (edge - gap);
        if side <= 0. {
            return None;
        }
        // The block is the intersection of half-spaces, its edges very slightly worn (a smooth
        // minimum over a few hundredths of the block).
        let wear = 0.02 + 0.03 * r(9);
        let mut h = side;
        // One or two tilted planes for the top: a slab lying askew, or a ridge where two faces meet.
        let plane = |a: f64, b: f64| Vector2::new(a - 0.5, b - 0.5) * 0.6;
        h = smooth_min(h, thick + plane(r(3), r(4)).dot(&v), wear);
        if r(5) < 0.35 {
            h = smooth_min(h, thick * (0.9 + 0.3 * r(6)) + plane(r(7), r(8)).dot(&v), wear);
        }
        // Edges and corners knocked off: planes cutting in from random sides.
        for k in 0 .. 3 {
            let a = std::f64::consts::TAU * r(10 + 3 * k);
            let d = Vector2::new(a.cos(), a.sin());
            let slope = 0.7 + 1.6 * r(11 + 3 * k);
            let reach = 0.12 + 0.3 * r(12 + 3 * k);
            h = smooth_min(h, thick + slope * (reach - v.dot(&d)), wear);
        }
        // A crack through some blocks.
        if r(20) < 0.35 {
            let a = std::f64::consts::TAU * r(21);
            let across = (v.dot(&Vector2::new(a.cos(), a.sin())) - 0.25 * (r(22) - 0.5)).abs();
            h = h.min(thick * (0.25 + 0.35 * r(23)) + 4. * across);
        }
        let h = (h - thick * 0.3 * r(24)).min(self.top());
        if h <= 0. {
            return None;
        }
        Some(Block { height: h * size, size, chance: r(25), id: r(26).max(1e-6) })
    }

    pub fn eval(&self, c: &Context, eval: impl Fn(&N, &Context) -> f64) -> f64 {
        let q = Vector2::new(c.p.x, c.p.z);
        // Every layer's block here, were it there.
        let mut blocks: Vec<Block> = Vec::with_capacity(8);
        for (layer, size) in self.sizes().enumerate() {
            // Blocks too small to see at this filter fade out.
            let fade = if c.filter > 0. { smoothstep(2. * c.filter, 4. * c.filter, size) } else { 1. };
            if fade <= 0. {
                break;
            }
            if let Some(mut b) = self.block(layer as u32, size, q) {
                b.height *= fade;
                blocks.push(b);
            }
        }
        // The highest of those the talus has, standing on the ground at height `y`.
        let top = |y: f64| -> Option<&Block> {
            let foot = Context { p: Vector3::new(c.p.x, y, c.p.z), ..*c };
            let position = self.position.as_ref().map_or(1., |n| eval(n, &foot)).clamp(0., 1.);
            let mask = self.mask.as_ref().map_or(1., |n| eval(n, &foot).clamp(0., 1.));
            blocks.iter().filter(|b| b.chance < self.coverage(b.size, position) * mask)
                .max_by(|a, b| a.height.total_cmp(&b.height))
        };
        let top = if !c.lifted {
            top(c.p.y)
        } else {
            // On the displaced surface the ground is below by the height of whichever block is
            // there, or none: of the footings that could be, the one that agrees best. Near the
            // edge of a size's range two can agree (a block on lower ground, or none); then
            // the block, so a block is never mistaken for bare ground.
            let height = |b: Option<&Block>| b.map_or(0., |b| b.height);
            std::iter::once(0.).chain(blocks.iter().map(|b| b.height)).map(|lift| {
                let t = top(c.p.y - lift);
                ((height(t) - lift).abs() - 1e-3 * lift, t)
            }).min_by(|a, b| a.0.total_cmp(&b.0)).and_then(|(_, t)| t)
        };
        match (self.output, top) {
            (_, None) => 0.,
            (Output::Height, Some(b)) => b.height,
            (Output::Mask, Some(b)) => smoothstep(0., 0.02, b.height),
            (Output::Id, Some(b)) => b.id,
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::graph::{Context, Graph};
    use crate::na::Vector3;
    use serde_json::json;

    fn talus(extra: serde_json::Value) -> serde_json::Value {
        let mut t = json!({"node": "talus", "largest": 4, "smallest": 0.125, "seed": 3,
            "position": {"node": "altitude", "from": 0, "to": 10}});
        t.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        t
    }

    fn points() -> impl Iterator<Item = Vector3<f64>> {
        (0 .. 4000).map(|k| Vector3::new((k % 80) as f64 * 0.37, 6., (k / 80) as f64 * 0.41))
    }

    #[test]
    fn test_heights_stay_in_range() {
        let g = Graph::parse(&talus(json!({}))).unwrap();
        let (lo, hi) = g.range();
        let mut on = 0;
        for p in points() {
            let h = g.eval(&Context::new(p, Vector3::y(), 0.));
            assert!(h >= lo && h <= hi, "{} outside {:?}", h, (lo, hi));
            on += (h > 0.) as usize;
        }
        assert!(on > 400 && on < 3800, "blocks cover {} of 4000 points", on);
    }

    #[test]
    fn test_shading_points_find_the_same_blocks() {
        // A mask and an id evaluated on the displaced surface (the point raised by the
        // block's height) agree with the blocks the displacement made; bare ground is very
        // seldom taken for a block (where a block on lower ground could have raised it).
        let height = Graph::parse(&talus(json!({}))).unwrap();
        let mask = Graph::parse(&talus(json!({"output": "mask"}))).unwrap();
        let id = Graph::parse(&talus(json!({"output": "id"}))).unwrap();
        let (mut ids, mut stray) = (std::collections::HashSet::new(), 0);
        for p in points() {
            let h = height.eval(&Context::new(p, Vector3::y(), 0.));
            let lifted = Context::lifted(p + Vector3::y() * h, Vector3::y());
            let m = mask.eval(&lifted);
            if h > 0.02 {
                assert_eq!(m, 1., "a block {} m high isn't masked", h);
                ids.insert((id.eval(&lifted) * 1e9) as u64);
            } else if h == 0. && m > 0. {
                // Bare ground taken for a block that could lie a little lower.
                stray += 1;
            }
        }
        assert!(ids.len() > 50, "only {} distinct blocks", ids.len());
        assert!(stray < 40, "{} of 4000 points of bare ground masked as blocks", stray);
    }

    #[test]
    fn test_many_small_few_large_sorted_downslope() {
        let t = super::Talus::<()>::parse(&json!({"largest": 4, "smallest": 0.125}), 0, |_| Ok(None)).unwrap();
        // Mid-talus, each halving of size covers more ground.
        let sizes: Vec<f64> = t.sizes().collect();
        assert_eq!(sizes.len(), 6);
        for w in sizes.windows(2) {
            assert!(t.coverage(w[1], 0.5) >= t.coverage(w[0], 0.5), "{:?}", sizes.iter().map(|&s| t.coverage(s, 0.5)).collect::<Vec<_>>());
        }
        // Near the toe the big blocks outrun the small; at the apex they are gone.
        assert!(t.coverage(4., 0.05) > t.coverage(0.125, 0.05));
        assert!(t.coverage(4., 1.) < 0.01 && t.coverage(0.125, 1.) > 0.5);
        // Beyond the toe, none.
        assert_eq!(t.coverage(4., 0.), 0.);
    }

    #[test]
    fn test_faces_are_flat() {
        // Angular blocks: along a line across a block, the height is piecewise linear, so
        // most second differences vanish (a rounded stone's never do).
        let g = Graph::parse(&talus(json!({"cover": 1, "sorting": 0}))).unwrap();
        let (mut flat, mut on) = (0, 0);
        let d = 0.002;
        for p in points() {
            let h = |x: f64| g.eval(&Context::new(p + Vector3::x() * x, Vector3::y(), 0.));
            let (a, b, c) = (h(-d), h(0.), h(d));
            if a > 0. && b > 0. && c > 0. {
                on += 1;
                flat += ((a + c - 2. * b).abs() < 1e-9) as usize;
            }
        }
        assert!(on > 300 && flat as f64 > 0.7 * on as f64, "{} of {} flat", flat, on);
    }
}
