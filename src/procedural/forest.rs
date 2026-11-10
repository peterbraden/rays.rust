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
use std::collections::HashMap;

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

impl<T: Geometry> Geometry for PartOf<T> {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        self.item.intersects(r).map(|i| RawIntersection { face: (self.part << PART_SHIFT) | (i.face & ((1 << PART_SHIFT) - 1)), ..i })
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
    let reach = species.iter().map(|s| 2. * radius(s, 1. + s.scale_v.abs())).fold(0., f64::max).max(1e-9);
    let cell = |x: f64, z: f64| ((x / reach).floor() as i64, (z / reach).floor() as i64);
    let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
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
        if field(&noise, clearing_at, layout.patch_size, x, z) < threshold {
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
        let r = radius(s, scale);

        let (cx, cz) = cell(x, z);
        let crowded = (cx - 1 ..= cx + 1).any(|i| (cz - 1 ..= cz + 1).any(|j| {
            grid.get(&(i, j)).is_some_and(|near| near.iter().any(|&n| {
                let q = placed[n].position;
                (q.x - x).hypot(q.z - z) < r + radii[n]
            }))
        }));
        if crowded {
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

        grid.entry((cx, cz)).or_default().push(placed.len());
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
