/// Where the trees of a forest stand.
///
/// A forest is a few generated trees per species, placed many times as instances (see
/// `crate::shapes::transform::Transform`), so it costs memory for its distinct trees, not for
/// every tree. To keep the repetition from showing, every placed tree is turned to a random
/// heading, scaled a little, and half of them mirrored, which doubles the distinct shapes.
///
/// Placement follows what makes real stands look natural, after Oliver Deussen, Pat Hanrahan,
/// Bernd Lintermann, Radomír Měch, Matt Pharr and Przemyslaw Prusinkiewicz, "Realistic Modeling
/// and Rendering of Plant Ecosystems", SIGGRAPH '98, 275-286, doi:10.1145/280814.280898, who
/// distribute plants by competition for space and by ecological patterns, then instance them:
/// - Trees compete for space, so neighbours keep a minimum distance, in proportion to their
///   size. That's Poisson-disk sampling, done by dart throwing as Robert L. Cook describes in
///   "Stochastic Sampling in Computer Graphics", ACM Transactions on Graphics 5(1), 51-72, 1986,
///   doi:10.1145/7529.8927: random candidates, each kept only if it's far enough from those
///   already kept, until candidates run out. It looks irregular without clumping or gaps, as
///   a stand thinned by competition does.
/// - Species grow in patches (stands) rather than evenly mixed: each species' chance at a spot
///   is weighted by its own smooth noise field, as strongly as `patchiness` says.
/// - Clearings: a further noise field leaves `clearings` of the area open.
/// - Trees stand only on the surface, dropped onto it from above, and not where it is steeper
///   than `max_slope`.
use crate::na::{Vector2, Vector3};
use crate::geometry::Pcg32;
use crate::noise::PerlinNoise;
use crate::ray::Ray;
use crate::shapes::geometry::Geometry;
use crate::shapes::bbox::BBox;
use crate::intersection::{Intersection, RawIntersection};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::texture::Medium;
use crate::scene::Scene;

/// Bits of a hit's `face` left for the part's own use, eg. the leaf's number; the bits above
/// them say which part of a multi-part object was hit.
pub const PART_SHIFT: u32 = 22;

/// One of the parts of an object made of many, each with its own material: its hits' faces
/// say which part, in the bits from `PART_SHIFT` up, keeping the item's own face below.
///
/// A whole forest is one object of such parts, the bark and leaves of every placed tree, so a
/// ray searches one acceleration structure and stops at the first tree it meets. As separate
/// objects, one per tree part, each spanning the whole forest, every ray searched them all.
pub struct PartOf<T: Geometry> {
    pub item: T,
    pub part: u32,
}

impl<T: Geometry> PartOf<T> {
    fn own(&self, i: RawIntersection) -> RawIntersection {
        RawIntersection { face: (self.part << PART_SHIFT) | (i.face & ((1 << PART_SHIFT) - 1)), ..i }
    }
}

impl<T: Geometry> Geometry for PartOf<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(r).map(|i| self.own(i))
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.item.intersects_beyond(r, min).map(|i| self.own(i))
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        self.item.intersects_range(r, min, max).map(|i| self.own(i))
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        self.item.occludes(r, min, max)
    }

    fn bounds(&self) -> BBox {
        self.item.bounds()
    }

    fn primitives(&self) -> u64 {
        self.item.primitives()
    }
}

/// The material of an object of `PartOf` parts: each hit is scattered by its part's medium,
/// with the part's own face.
pub struct PartsMaterial {
    pub parts: Vec<Box<dyn Medium + Sync + Send>>,
}

impl MaterialModel for PartsMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let part = (intersection.face >> PART_SHIFT) as usize;
        let own = Intersection { face: intersection.face & ((1 << PART_SHIFT) - 1), ..*intersection };
        self.parts[part].material_at(intersection.point).scatter(r, &own, s)
    }
}

/// A geometry shared by every placement of a tree at one level of detail: all its parts, each
/// numbered (`PartOf`).
pub type Model = std::sync::Arc<dyn Geometry + Sync + Send>;

/// A tree placed in a forest: its model, turned `yaw` about the vertical (as cosine and sine),
/// scaled by `scale`, mirrored in x if `mirror`, and moved to `at`. A forest places hundreds of
/// thousands of trees, so this keeps to what a forest's placement needs, about 60 bytes, where
/// a general instancing `Transform` keeps a pair of 4x4 matrices.
pub struct Placed {
    pub model: Model,
    pub at: Vector3<f64>,
    pub cos: f64,
    pub sin: f64,
    pub scale: f64,
    pub mirror: bool,
}

impl Placed {
    pub fn new(model: Model, at: Vector3<f64>, yaw_degrees: f64, scale: f64, mirror: bool) -> Placed {
        let yaw = yaw_degrees.to_radians();
        Placed { model, at, cos: yaw.cos(), sin: yaw.sin(), scale, mirror }
    }

    fn sx(&self) -> f64 {
        if self.mirror { -1. } else { 1. }
    }

    /// A world vector in the model's frame: unturned, unscaled, unmirrored.
    fn to_local(&self, v: &Vector3<f64>) -> Vector3<f64> {
        let (x, z) = (self.cos * v.x - self.sin * v.z, self.sin * v.x + self.cos * v.z);
        Vector3::new(self.sx() * x, v.y, z) / self.scale
    }

    /// A point of the model's in the world.
    fn to_world(&self, p: &Vector3<f64>) -> Vector3<f64> {
        let (x, y, z) = (self.sx() * p.x * self.scale, p.y * self.scale, p.z * self.scale);
        self.at + Vector3::new(self.cos * x + self.sin * z, y, -self.sin * x + self.cos * z)
    }

    /// A normal of the model's in the world: the scale is uniform, so only turned and mirrored.
    fn normal_to_world(&self, n: &Vector3<f64>) -> Vector3<f64> {
        let x = self.sx() * n.x;
        Vector3::new(self.cos * x + self.sin * n.z, n.y, -self.sin * x + self.cos * n.z).normalize()
    }

    fn local_ray(&self, r: &Ray) -> Ray {
        Ray { ro: self.to_local(&(r.ro - self.at)), rd: self.to_local(&r.rd) }
    }

    fn hit_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        // Distances along the ray shrink with the model, its scale being uniform.
        self.model.intersects_range(&self.local_ray(r), min / self.scale, max / self.scale).map(|h| RawIntersection {
            dist: h.dist * self.scale,
            point: self.to_world(&h.point),
            normal: self.normal_to_world(&h.normal),
            ..h
        })
    }

    pub fn bounds(&self) -> BBox {
        let b = self.model.bounds();
        let mut out = BBox::empty();
        for k in 0 .. 8 {
            let corner = Vector3::new(
                if k & 1 == 0 { b.min.x } else { b.max.x },
                if k & 2 == 0 { b.min.y } else { b.max.y },
                if k & 4 == 0 { b.min.z } else { b.max.z });
            out = out.union_point(&self.to_world(&corner));
        }
        out
    }
}

/// The trees of a forest, stored in one array behind a BVH (`crate::bvh::Bvh`).
pub struct Stand {
    bvh: crate::bvh::Bvh,
    trees: Vec<Placed>,
    primitives: u64,
}

impl Stand {
    pub fn new(trees: Vec<Placed>) -> Stand {
        let primitives = trees.iter().map(|t| t.model.primitives()).sum();
        let (bvh, trees) = crate::bvh::Bvh::build(trees, Placed::bounds);
        Stand { bvh, trees, primitives }
    }
}

impl Geometry for Stand {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.intersects_range(r, 0., f64::INFINITY)
    }

    fn intersects_beyond(&self, r: &Ray, min: f64) -> Option<RawIntersection> {
        self.intersects_range(r, min, f64::INFINITY)
    }

    fn intersects_range(&self, r: &Ray, min: f64, max: f64) -> Option<RawIntersection> {
        if self.trees.is_empty() {
            return None;
        }
        self.bvh.nearest(r, min, max, |i, best| self.trees[i].hit_range(r, min, best)).map(|(_, x)| x)
    }

    fn occludes(&self, r: &Ray, min: f64, max: f64) -> bool {
        !self.trees.is_empty() && self.bvh.any_hit(r, min, max, |i| {
            let t = &self.trees[i];
            t.model.occludes(&t.local_ray(r), min / t.scale, max / t.scale)
        })
    }

    fn bounds(&self) -> BBox {
        self.bvh.bounds()
    }

    fn primitives(&self) -> u64 {
        self.primitives
    }
}

/// How to choose each placed tree's level of detail (`crate::shapes::lod`): by the size of a
/// pixel where it stands, seen from `eye`.
#[derive(Clone, Debug)]
pub struct Lod {
    pub eye: Vector3<f64>,
    /// The angle between neighbouring pixels, in radians.
    pub pixel_angle: f64,
    /// The largest detail dropped, in pixels.
    pub pixels: f64,
    pub max_level: usize,
}

impl Lod {
    /// The level of detail for a tree whose own `bounds` (about its base) are placed at `at`,
    /// scaled by `scale`, whose pieces are `base` across: by the size of a pixel at its nearest
    /// point to the eye, in the tree's own units. A tree is placed turned and maybe mirrored, so
    /// its nearest point is taken from a sphere about its bounds.
    pub fn level(&self, bounds: &BBox, at: Vector3<f64>, scale: f64, base: f64) -> usize {
        let mid = (bounds.min.coords + bounds.max.coords) * 0.5;
        let centre = at + Vector3::new(0., mid.y * scale, 0.);
        let radius = (0.5 * (bounds.max - bounds.min).norm() + mid.xz().norm()) * scale;
        let distance = ((centre - self.eye).norm() - radius).max(0.);
        // Beyond the coarsest level worth making, a card as large as the whole tree.
        let most = crate::shapes::lod::level_for(base, (bounds.max - bounds.min).max(), 1., self.max_level) + 1;
        crate::shapes::lod::level_for(base, distance * self.pixel_angle / scale, self.pixels, most.min(self.max_level))
    }
}

/// A species' share of a forest.
#[derive(Clone, Debug)]
pub struct ForestSpecies {
    /// How common it is, relative to the others.
    pub weight: f64,
    /// Distance to its neighbours, relative to the forest's `spacing`: smaller for shrubs.
    pub spacing: f64,
    /// Variation of each tree's size, as a fraction of it.
    pub scale_v: f64,
}

/// The layout of a forest.
#[derive(Clone, Debug)]
pub struct ForestLayout {
    /// The area covered, in x and z.
    pub min: Vector2<f64>,
    pub max: Vector2<f64>,
    /// Typical distance between neighbouring trees.
    pub spacing: f64,
    /// Fraction of the area left as clearings.
    pub clearings: f64,
    /// Size of the clearings and of single-species stands.
    pub patch_size: f64,
    /// 0 for species evenly mixed, up to about 1 for stands of mostly one species.
    pub patchiness: f64,
    /// Steepest ground a tree will grow on, in degrees.
    pub max_slope: f64,
    /// Distinct trees grown per species, shared among its placements.
    pub variants: usize,
}

/// One tree of the forest.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub species: usize,
    /// Which of its species' distinct trees.
    pub variant: usize,
    /// Where it meets the surface.
    pub position: Vector3<f64>,
    /// Turn about the vertical, in degrees.
    pub yaw: f64,
    pub scale: f64,
    pub mirror: bool,
    /// Slope of the surface there, in degrees.
    pub slope: f64,
}

/// Smooth noise in [0, 1], about 0.5, varying over `size`.
fn field(noise: &PerlinNoise, offset: Vector2<f64>, size: f64, x: f64, z: f64) -> f64 {
    noise.fbm(offset.x + x / size, 0.5, offset.y + z / size, 3, 0.5, 2.)
}

/// Where the trees stand on `surfaces`, chosen by `seed`.
pub fn place(layout: &ForestLayout, species: &[ForestSpecies], seed: u64, surfaces: &[&dyn Geometry]) -> Vec<Placement> {
    place_favouring(layout, species, seed, surfaces, &|_, _| 1.)
}

/// As `place`, keeping a tree at (x, z) only with the chance `favour(x, z)` gives (0 to 1),
/// as plants of a dry country crowd where water gathers.
pub fn place_favouring(layout: &ForestLayout, species: &[ForestSpecies], seed: u64, surfaces: &[&dyn Geometry],
                       favour: &dyn Fn(f64, f64) -> f64) -> Vec<Placement> {
    assert!(!species.is_empty(), "a forest needs at least one species");
    assert!(layout.spacing > 0. && layout.patch_size > 0., "a forest's spacing and patch size must be positive");
    let mut rng = Pcg32::new(seed, 0xf0e5);
    let noise = PerlinNoise::new();
    let mut offset = || Vector2::new(rng.next_f64() * 200., rng.next_f64() * 200.);
    let clearing_at = offset();
    let stands: Vec<Vector2<f64>> = species.iter().map(|_| offset()).collect();
    let size = layout.max - layout.min;

    // The clearing threshold: the `clearings` quantile of the clearing field over the area.
    let threshold = if layout.clearings <= 0. { f64::NEG_INFINITY } else {
        let n = 64;
        let mut samples: Vec<f64> = (0 .. n * n).map(|k| {
            let (x, z) = (layout.min.x + size.x * ((k % n) as f64 + 0.5) / n as f64, layout.min.y + size.y * ((k / n) as f64 + 0.5) / n as f64);
            field(&noise, clearing_at, layout.patch_size, x, z)
        }).collect();
        samples.sort_by(f64::total_cmp);
        samples[((layout.clearings.min(1.) * (n * n) as f64) as usize).min(n * n - 1)]
    };

    // Neighbours are kept apart by the sum of their radii, half their spacing; a grid of cells
    // as wide as the largest such sum means only adjacent cells need checking.
    let radius = |s: &ForestSpecies, scale: f64| 0.5 * layout.spacing * s.spacing * scale;
    // Unfavoured places need up to twice the room (see `place_favouring`).
    let reach = species.iter().map(|s| 4. * radius(s, 1. + s.scale_v.abs())).fold(0., f64::max).max(1e-9);
    // The cells, over the area and one more each side, in an array: tens of millions of darts
    // look up nine cells each. Cells wider than `reach` work as well, so a huge area with tiny
    // spacing is kept to a few million.
    let (lo, hi) = (layout.min.inf(&layout.max), layout.min.sup(&layout.max));
    let side = reach.max(((hi.x - lo.x) * (hi.y - lo.y) / 4e6).sqrt());
    let (nx, nz) = (((hi.x - lo.x) / side).floor() as i64 + 3, ((hi.y - lo.y) / side).floor() as i64 + 3);
    let cell = |x: f64, z: f64| (((x - lo.x) / side).floor() as i64 + 1, ((z - lo.y) / side).floor() as i64 + 1);
    let mut grid: Vec<Vec<usize>> = vec![vec![]; (nx * nz) as usize];
    let at = |(i, j): (i64, i64)| (0 .. nx).contains(&i).then_some(()).and((0 .. nz).contains(&j).then_some((j * nx + i) as usize));
    // The smallest radius any tree can have.
    let smallest = species.iter().map(|s| radius(s, (1. - s.scale_v.abs()).max(0.))).fold(f64::INFINITY, f64::min);
    let mut placed: Vec<Placement> = vec![];
    let mut radii: Vec<f64> = vec![];

    // Enough darts to fill the area at the closest spacing many times over.
    let closest = species.iter().map(|s| layout.spacing * s.spacing).fold(f64::INFINITY, f64::min).max(1e-9);
    let darts = (30. * size.x.abs() * size.y.abs() / (closest * closest)).ceil() as usize;

    let top = |s: &&dyn Geometry| {
        let y = s.bounds().max.y;
        if y.is_finite() && y < 1e5 { y + 1. } else { 1e5 }
    };
    let start = surfaces.iter().map(top).fold(f64::NEG_INFINITY, f64::max);

    for _ in 0 .. darts {
        let (x, z) = (layout.min.x + rng.next_f64() * size.x, layout.min.y + rng.next_f64() * size.y);
        let (pick, spread, yaw, mirror, which) = (rng.next_f64(), rng.next_f64() * 2. - 1., rng.next_f64() * 360., rng.next_f64() < 0.5, rng.next_f64());

        // Most darts land too near a tree already placed: reject those first, before the noise
        // fields of the clearings and species, if they're too near even at the smallest radius
        // any tree can have. A rejected dart changes nothing, so the same trees are kept as by
        // testing at the species' own radius alone.
        let (cx, cz) = cell(x, z);
        let near = |r: f64| (cx - 1 ..= cx + 1).any(|i| (cz - 1 ..= cz + 1).any(|j| {
            at((i, j)).is_some_and(|k| grid[k].iter().any(|&n| {
                let q = placed[n].position;
                (q.x - x).hypot(q.z - z) < r + radii[n]
            }))
        }));
        if near(smallest) {
            continue;
        }
        if field(&noise, clearing_at, layout.patch_size, x, z) < threshold {
            continue;
        }
        // A uniform number from one already drawn, so forests without a favour are unchanged.
        // The favour sets how much room a tree needs there, as well as its chance: dart
        // throwing fills every gap in the end, so thinning alone wouldn't show.
        let favoured = favour(x, z).clamp(0., 1.);
        if favoured < (which * 7919.).fract() {
            continue;
        }

        // The species, by weight, favouring those whose stand this is.
        let weights: Vec<f64> = species.iter().zip(&stands).map(|(s, at)| {
            let stand = (field(&noise, *at, layout.patch_size, x, z) - 0.5) / 0.15;
            s.weight.max(0.) * (3. * layout.patchiness * stand).exp()
        }).collect();
        let total: f64 = weights.iter().sum();
        if total <= 0. {
            continue;
        }
        let mut k = 0;
        let mut acc = weights[0] / total;
        while acc < pick && k + 1 < weights.len() {
            k += 1;
            acc += weights[k] / total;
        }
        let s = &species[k];
        let scale = 1. + spread * s.scale_v;
        let r = radius(s, scale) / favoured.max(0.25).sqrt();

        if near(r) {
            continue;
        }

        // Drop it onto the highest surface below.
        let down = Ray { ro: Vector3::new(x, start, z), rd: Vector3::new(0., -1., 0.) };
        let hit = surfaces.iter().filter_map(|s| s.intersects(&down)).min_by(|a, b| a.dist.total_cmp(&b.dist));
        let hit = match hit { Some(h) => h, None => continue };
        let up = if hit.normal.y < 0. { -hit.normal } else { hit.normal };
        let slope = up.normalize().y.clamp(-1., 1.).acos().to_degrees();
        if slope > layout.max_slope {
            continue;
        }

        grid[at((cx, cz)).expect("a tree within the area")].push(placed.len());
        radii.push(r);
        placed.push(Placement {
            species: k,
            variant: ((which * layout.variants.max(1) as f64) as usize).min(layout.variants.max(1) - 1),
            position: Vector3::new(x, hit.point.y, z),
            yaw, scale, mirror, slope,
        });
    }
    placed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::plane::Plane;
    use crate::shapes::sphere::Sphere;

    fn layout() -> ForestLayout {
        ForestLayout {
            min: Vector2::new(-50., -50.), max: Vector2::new(50., 50.),
            spacing: 5., clearings: 0., patch_size: 30., patchiness: 0., max_slope: 30., variants: 3,
        }
    }

    fn one(spacing: f64, scale_v: f64) -> ForestSpecies {
        ForestSpecies { weight: 1., spacing, scale_v }
    }

    #[test]
    fn test_trees_crowd_where_favoured() {
        let ground = Plane { y: 0. };
        let trees = place_favouring(&layout(), &[one(1., 0.)], 8, &[&ground], &|x, _| if x > 0. { 1. } else { 0.1 });
        let east = trees.iter().filter(|t| t.position.x > 0.).count();
        assert!(east > 3 * (trees.len() - east), "{} of {}", east, trees.len());
    }

    #[test]
    fn test_placed_trees_match_their_transforms() {
        use crate::shapes::transform::Transform;
        use crate::na::Rotation3;
        let model: Model = std::sync::Arc::new(Sphere::new(Vector3::new(1., 2., 0.5), 0.8));
        let mut rng = Pcg32::new(1, 1);
        for _ in 0 .. 50 {
            let (at, yaw, scale, mirror) = (Vector3::new(rng.next_f64() * 20., rng.next_f64(), rng.next_f64() * 20.),
                rng.next_f64() * 360., 0.5 + rng.next_f64(), rng.next_f64() < 0.5);
            let placed = Placed::new(model.clone(), at, yaw, scale, mirror);
            let transform = Transform::from_parts(model.clone(), at, Rotation3::from_axis_angle(&Vector3::y_axis(), yaw.to_radians()),
                Vector3::new(if mirror { -scale } else { scale }, scale, scale));
            let stand = Stand::new(vec![Placed::new(model.clone(), at, yaw, scale, mirror)]);
            let (a, b) = (placed.bounds(), transform.bounds());
            assert!((a.min - b.min).norm() < 1e-6 && (a.max - b.max).norm() < 1e-6, "{:?} {:?}", a, b);
            for _ in 0 .. 20 {
                let target = (a.min.coords + a.max.coords) * 0.5 + Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5) * 2.;
                let ro = target + Vector3::new(rng.next_f64() - 0.5, rng.next_f64(), rng.next_f64() - 0.5).normalize() * 30.;
                let r = Ray { ro, rd: (target - ro) * 0.7 };
                match (stand.intersects(&r), transform.intersects(&r)) {
                    (Some(x), Some(y)) => {
                        assert!((x.dist - y.dist).abs() < 1e-9 && (x.point - y.point).norm() < 1e-9, "{:?} {:?}", x, y);
                        assert!((x.normal - y.normal.normalize()).norm() < 1e-9, "{:?} {:?}", x.normal, y.normal);
                        assert!(stand.occludes(&r, 0., x.dist + 1e-6) && !stand.occludes(&r, 0., x.dist - 1e-6));
                    },
                    (None, None) => assert!(!stand.occludes(&r, 0., f64::INFINITY)),
                    (x, y) => panic!("{:?} {:?}", x, y),
                }
            }
        }
    }

    #[test]
    fn test_seed_chooses_the_forest() {
        let ground = Plane { y: 0. };
        let place_with = |seed| place(&layout(), &[one(1., 0.2)], seed, &[&ground]);
        assert_eq!(place_with(1), place_with(1));
        assert_ne!(place_with(1), place_with(2));
    }

    #[test]
    fn test_neighbours_keep_their_distance_and_fill_the_area() {
        let ground = Plane { y: 0. };
        let species = [one(1., 0.2), one(0.4, 0.)];
        let trees = place(&layout(), &species, 3, &[&ground]);
        let radius = |p: &Placement| 0.5 * 5. * species[p.species].spacing * p.scale;
        for (i, a) in trees.iter().enumerate() {
            for b in &trees[i + 1 ..] {
                let d = (a.position - b.position).norm();
                assert!(d >= radius(a) + radius(b), "{} apart: {:?} {:?}", d, a, b);
            }
            assert!(a.position.x.abs() <= 50. && a.position.z.abs() <= 50.);
            assert!((0.8 ..= 1.2).contains(&a.scale) && a.variant < 3);
        }
        // Dart throwing packs discs of radius r to about 55% of the area: here, most of it.
        let big = trees.iter().filter(|p| p.species == 0).count();
        assert!(big > 100, "{} trees", big);
    }

    #[test]
    fn test_species_by_weight_and_in_stands() {
        let ground = Plane { y: 0. };
        let mixed = [ForestSpecies { weight: 3., ..one(1., 0.) }, one(1., 0.)];
        let trees = place(&layout(), &mixed, 4, &[&ground]);
        let share = trees.iter().filter(|p| p.species == 0).count() as f64 / trees.len() as f64;
        assert!((share - 0.75).abs() < 0.08, "{}", share);

        // In stands, a tree's nearest neighbour is mostly of its own species.
        let same_neighbour = |l: &ForestLayout| {
            let trees = place(l, &[one(1., 0.), one(1., 0.)], 5, &[&ground]);
            trees.iter().filter(|a| {
                let nearest = trees.iter().filter(|b| *b != *a)
                    .min_by(|b, c| (b.position - a.position).norm().total_cmp(&(c.position - a.position).norm())).unwrap();
                nearest.species == a.species
            }).count() as f64 / trees.len() as f64
        };
        let even = same_neighbour(&layout());
        let stands = same_neighbour(&ForestLayout { patchiness: 1., ..layout() });
        assert!(stands > even + 0.15, "{} vs {}", stands, even);
    }

    #[test]
    fn test_clearings_leave_ground_open() {
        let ground = Plane { y: 0. };
        let count = |clearings| place(&ForestLayout { clearings, ..layout() }, &[one(1., 0.)], 6, &[&ground]).len() as f64;
        let ratio = count(0.4) / count(0.);
        assert!((ratio - 0.6).abs() < 0.1, "{}", ratio);
    }

    #[test]
    fn test_trees_stand_on_the_surface_but_not_on_steep_slopes() {
        // A hill: a sphere of radius 60 centred 40 below the ground, rising 20 above it.
        let ground = Plane { y: 0. };
        let hill = Sphere::new(Vector3::new(0., -40., 0.), 60.);
        let trees = place(&layout(), &[one(1., 0.)], 7, &[&ground, &hill]);
        for p in &trees {
            let on_hill = (p.position - Vector3::new(0., -40., 0.)).norm();
            if p.position.y > 1e-9 {
                assert!((on_hill - 60.).abs() < 1e-6, "{:?}", p);
                // The slope of the sphere where the tree stands.
                let slope = ((p.position.x.hypot(p.position.z)) / 60.).asin().to_degrees();
                assert!((slope - p.slope).abs() < 1e-6 && slope <= 30., "{:?}", p);
            } else {
                assert!(p.position.y.abs() < 1e-9 && p.slope < 1e-9);
            }
        }
        // Some on the hill, some on the plain, none on the hill's steep flanks.
        assert!(trees.iter().any(|p| p.position.y > 5.));
        assert!(trees.iter().any(|p| p.position.y.abs() < 1e-9));
        assert!(!trees.iter().any(|p| p.position.y > 1e-9 && p.position.y < 0.1));
    }
}
