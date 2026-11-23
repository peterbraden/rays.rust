//! The scene as plain data, for the wavefront integrator: primitives in an array with a BVH
//! over them, and materials as an enum, in place of the `Box<dyn Geometry>` and
//! `Box<dyn MaterialModel>` the recursive tracer dispatches through (Pharr, Jakob and
//! Humphreys, "Physically Based Rendering", 4th ed., section 15.1: the wavefront renderer's
//! scene is flattened so that every stage can run as one tight loop over many rays, and, on a
//! GPU, without virtual calls).
//!
//! Shapes and materials describe themselves (`Geometry::flatten`, `MaterialModel::flat`), so
//! the scene file is parsed, and transforms and instances applied, exactly as for the
//! recursive tracer. The skysphere stays an object, evaluated for rays that escape. What can't
//! be flattened (procedural shapes, CSG, media, spectral and motion-blurred
//! renders) is reported, and the scene renders the usual way.
//!
//! A shape that isn't primitives (an ocean, a terrain, a combined shape: anything whose
//! `flatten` is None) doesn't stop a scene being flattened either. It is kept whole, as a
//! shape the scene's own code intersects (`FlatScene::shapes`), tested after the primitives
//! as the planes are; its hits carry what its material may ask of them (`Hit::face`, `part`).
//! The CPU's integrator renders such scenes; the GPU takes those of the kinds it has a kernel
//! for and says which it hasn't (`gpu::Buffers::build`). Only what lets light through to
//! shadow rays in part (transparent water, media) is still refused: the stages' shadow rays
//! are blocked or not.
//!
//! A material that isn't data (textured, layered, procedural: anything whose `flat` is None)
//! doesn't stop a scene being flattened if its shape can be. It is kept as `FlatMaterial::Host`,
//! naming its object, and a path that hits it is shaded by the scene's own material, as the
//! recursive tracer would (`integrator::host_bounce`). That is the fallback the plan's
//! scheduler is for (plan.md sections 12 and 13; `scheduler`'s "CPU queue"): the stages cover
//! what has been ported, and the rest is a queue of its own served by the renderer's code.

use crate::bvh::Bvh;
use crate::color::Color;
use crate::environment::Distribution1D;
use crate::intersection::RawIntersection;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shapes::bbox::BBox;
use std::f64::consts::PI;

/// A primitive shape.
#[derive(Clone, Copy, Debug)]
pub enum Prim {
    /// A triangle, with a normal per vertex for smooth shading if it has them. Its geometric
    /// normal follows the winding, (v1 - v0) × (v2 - v0).
    Triangle { v: [Vector3<f64>; 3], n: Option<[Vector3<f64>; 3]> },
    Sphere { center: Vector3<f64>, radius: f64 },
    /// The infinite plane y = `y`, its normal facing each ray (as `Plane`); kept out of the BVH.
    Plane { y: f64 },
}

/// The nearest a shape its own code intersects is asked for a hit: such shapes find their
/// surfaces by marching or iterating, and a ray leaving one starts within that error of it.
pub(crate) const MIN_SHAPE: f64 = 1e-9;

/// A surface hit: distance, point, geometric normal, and the normal to shade with.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub dist: f64,
    pub point: Vector3<f64>,
    pub geometric: Vector3<f64>,
    pub shading: Vector3<f64>,
    pub prim: u32,
    /// Which face and part of its object was hit, for a material that asks (`Geometry::uv`,
    /// `Medium::part`): a primitive's place among its object's, or what a shape the scene's
    /// own code intersects reported.
    pub face: u32,
    pub part: u32,
}

impl Prim {
    pub fn bounds(&self) -> BBox {
        match self {
            Prim::Triangle { v, .. } => BBox::new(v[0].inf(&v[1]).inf(&v[2]), v[0].sup(&v[1]).sup(&v[2])),
            Prim::Sphere { center, radius } => BBox::new(center - Vector3::repeat(*radius), center + Vector3::repeat(*radius)),
            Prim::Plane { .. } => BBox::new(Vector3::repeat(f64::MIN), Vector3::repeat(f64::MAX)),
        }
    }

    pub fn area(&self) -> f64 {
        match self {
            Prim::Triangle { v, .. } => (v[1] - v[0]).cross(&(v[2] - v[0])).norm() / 2.,
            Prim::Sphere { radius, .. } => 4. * PI * radius * radius,
            Prim::Plane { .. } => f64::INFINITY,
        }
    }

    /// The nearest hit along the ray (unit direction) within (min, max): distance, geometric
    /// and shading normals.
    pub fn intersect(&self, r: &Ray, min: f64, max: f64) -> Option<(f64, Vector3<f64>, Vector3<f64>)> {
        match self {
            Prim::Triangle { v, n } => {
                // Möller and Trumbore.
                let (e1, e2) = (v[1] - v[0], v[2] - v[0]);
                let p = r.rd.cross(&e2);
                let det = e1.dot(&p);
                if det.abs() < 1e-14 {
                    return None;
                }
                let inv = 1. / det;
                let s = r.ro - v[0];
                let b1 = s.dot(&p) * inv;
                if !(0. ..=1.).contains(&b1) {
                    return None;
                }
                let q = s.cross(&e1);
                let b2 = r.rd.dot(&q) * inv;
                if b2 < 0. || b1 + b2 > 1. {
                    return None;
                }
                let t = e2.dot(&q) * inv;
                if t <= min || t >= max {
                    return None;
                }
                let g = e1.cross(&e2).normalize();
                let shading = match n {
                    Some(n) => (n[0] * (1. - b1 - b2) + n[1] * b1 + n[2] * b2).try_normalize(1e-12).unwrap_or(g),
                    None => g,
                };
                Some((t, g, shading))
            },
            Prim::Sphere { center, radius } => {
                let oc = r.ro - center;
                let b = oc.dot(&r.rd);
                let c = oc.norm_squared() - radius * radius;
                let disc = b * b - c;
                if disc < 0. {
                    return None;
                }
                let sq = disc.sqrt();
                let t = [-b - sq, -b + sq].into_iter().find(|t| *t > min && *t < max)?;
                let n = (r.ro + r.rd * t - center) / *radius;
                Some((t, n, n))
            },
            Prim::Plane { y } => {
                if r.rd.y.abs() < 1e-14 {
                    return None;
                }
                let t = (y - r.ro.y) / r.rd.y;
                if t <= min || t >= max {
                    return None;
                }
                // Facing the ray, as `Plane::intersects`: a plane has no inside, so emitters on
                // it shine both ways, and glass is always entered.
                let n = Vector3::new(0., if r.rd.y > 0. { -1. } else { 1. }, 0.);
                Some((t, n, n))
            },
        }
    }

    /// A point uniformly distributed over the surface by area, and its geometric normal.
    pub fn sample(&self, u: [f64; 2]) -> (Vector3<f64>, Vector3<f64>) {
        match self {
            Prim::Triangle { v, .. } => {
                let su = u[0].sqrt();
                let (b1, b2) = (1. - su, u[1] * su);
                let p = v[0] * b1 + v[1] * b2 + v[2] * (1. - b1 - b2);
                (p, (v[1] - v[0]).cross(&(v[2] - v[0])).normalize())
            },
            Prim::Sphere { center, radius } => {
                let n = crate::geometry::point_on_unit_sphere(u[0], u[1]);
                (center + n * *radius, n)
            },
            Prim::Plane { y } => (Vector3::new(0., *y, 0.), Vector3::new(0., 1., 0.)),
        }
    }

    /// This primitive moved by an affine transformation; None for a sphere scaled unevenly.
    pub fn transformed(&self, t: &crate::na::Affine3<f64>) -> Option<Prim> {
        let point = |p: &Vector3<f64>| t.transform_point(&crate::na::Point3::from(*p)).coords;
        let linear = t.matrix().fixed_view::<3, 3>(0, 0).into_owned();
        let normal_matrix = linear.try_inverse()?.transpose();
        match self {
            Prim::Triangle { v, n } => {
                let v = [point(&v[0]), point(&v[1]), point(&v[2])];
                let n = n.map(|n| [(normal_matrix * n[0]).normalize(), (normal_matrix * n[1]).normalize(), (normal_matrix * n[2]).normalize()]);
                // A mirroring transformation reverses the winding, and with it the geometric
                // normal, which must still point the way the transformed normal does (as
                // `Transform`'s normal matrix keeps it): swap two corners.
                Some(if linear.determinant() < 0. {
                    Prim::Triangle { v: [v[0], v[2], v[1]], n: n.map(|n| [n[0], n[2], n[1]]) }
                } else {
                    Prim::Triangle { v, n }
                })
            },
            Prim::Sphere { center, radius } => {
                let gram = linear.transpose() * linear;
                let s2 = gram.trace() / 3.;
                ((gram - crate::na::Matrix3::identity() * s2).norm() <= 1e-9 * s2)
                    .then(|| Prim::Sphere { center: point(center), radius: radius * s2.sqrt() })
            },
            Prim::Plane { .. } => None,
        }
    }
}

/// A material as data.
#[derive(Clone, Debug)]
pub enum FlatMaterial {
    Diffuse { albedo: Color },
    Plastic { albedo: Color, refractive_index: f64, roughness: f64 },
    Dielectric { refractive_index: f64, tint: Color },
    Conductor { optics: crate::material::conductor::Optics, roughness: f64 },
    /// The `metal` material: GGX with Schlick's Fresnel from `albedo`.
    Specular { albedo: Color, roughness: f64 },
    Emitter { radiance: Color, two_sided: bool },
    /// Two materials in a checkerboard across x and z (`CheckeredYPlane`).
    Checker { m1: Box<FlatMaterial>, m2: Box<FlatMaterial>, xsize: f64, zsize: f64 },
    /// A material that isn't data: `FlatScene::hosts[object]`'s own, asked through
    /// `MaterialModel::scatter` as the recursive tracer does.
    Host { object: u32 },
}

impl FlatMaterial {
    /// The material at `p`: itself, or for a checkerboard, the square's (as `CheckeredYPlane`).
    pub fn at(&self, p: &Vector3<f64>) -> &FlatMaterial {
        match self {
            FlatMaterial::Checker { m1, m2, xsize, zsize } => {
                let zig = if (p.x.abs() / xsize) as i32 % 2 == 0 { p.x > 0. } else { p.x <= 0. };
                let zag = if (p.z.abs() / zsize) as i32 % 2 == 0 { p.z > 0. } else { p.z <= 0. };
                if zig != zag { m1.at(p) } else { m2.at(p) }
            },
            m => m,
        }
    }

    pub fn emitted(&self, n: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        match self {
            FlatMaterial::Emitter { radiance, two_sided } if *two_sided || towards.dot(n) > 0. => *radiance,
            _ => Color::black(),
        }
    }
}

/// The flattened scene.
pub struct FlatScene {
    /// Bounded primitives, in the BVH's order, with their materials.
    pub prims: Vec<Prim>,
    pub prim_material: Vec<u32>,
    pub bvh: Bvh,
    /// Unbounded primitives (planes), and their materials.
    pub planes: Vec<(Prim, u32)>,
    pub materials: Vec<FlatMaterial>,
    /// Emitting primitives (indices into `prims`), chosen to sample by power.
    pub emitters: Vec<u32>,
    pub emitter_choice: Distribution1D,
    /// Each primitive's index in `emitters`, if it is one.
    pub emitter_index: Vec<Option<u32>>,
    /// The skysphere, evaluated (by its material) for rays that escape.
    pub sky: Option<std::sync::Arc<crate::sceneobject::SceneObject>>,
    /// Objects that aren't primitives, intersected by their own code, and their materials. A
    /// hit's `prim` counts them after the primitives and the planes.
    pub shapes: Vec<(std::sync::Arc<crate::sceneobject::SceneObject>, u32)>,
    /// The objects whose materials are `FlatMaterial::Host`.
    pub hosts: Vec<std::sync::Arc<crate::sceneobject::SceneObject>>,
    /// Where there are hosts: each primitive's place among its object's (its face, for
    /// `Geometry::uv`: a mesh flattens to its triangles in order). Empty otherwise.
    pub prim_face: Vec<u32>,
    /// Participating media, as data (`volume::Volume::flat`). The GPU's kernels track paths
    /// through them; this crate's own stages don't (`FlatScene::needs_kernels`).
    pub media: Vec<crate::volume::FlatMedium>,
}

impl FlatScene {
    /// Flatten a parsed scene, or say why it can't be.
    pub fn build(s: &Scene) -> Result<FlatScene, String> {
        let mut problems = Vec::new();
        if s.render.spectral {
            problems.push("spectral rendering".to_string());
        }
        if s.render.motion {
            problems.push("motion blur".to_string());
        }
        let mut sky = None;
        let mut materials: Vec<FlatMaterial> = Vec::new();
        let mut bounded: Vec<(Prim, u32, u32)> = Vec::new();
        let mut hosts = Vec::new();
        let mut shapes = Vec::new();
        let mut planes: Vec<(Prim, u32)> = Vec::new();
        let mut media = Vec::new();
        let objects = s.objects.items().iter().chain(s.objects.infinite_items.iter());
        let (mut shapes_seen, mut unflat_shapes, mut unflat_materials) = (0, 0, 0);
        // Which objects are in the way, for whoever wants the scene on the GPU: each by its
        // place in the scene file's `objects` and its kind of shape.
        let mut blocking: Vec<String> = Vec::new();
        let mut blocked = |o: &crate::sceneobject::SceneObject, why: &str| {
            let name = match s.source_index(o) {
                Some(i) => format!("objects[{}] ({}{})", i, o.geometry.kind(), why),
                None => format!("{}{}", o.geometry.kind(), why),
            };
            if !blocking.contains(&name) {
                blocking.push(name);
            }
        };
        for o in objects {
            shapes_seen += 1;
            if o.geometry.is_stochastic() {
                let material = o.medium.material_at(Vector3::zeros());
                match (o.geometry.medium(), material.medium()) {
                    (Some(volume), Some(scattering)) => match volume.flat(scattering) {
                        Ok(m) => media.push(m),
                        Err(why) => {
                            problems.push("participating media the kernels have no code for".to_string());
                            blocked(o, &format!(", a medium: {}", why));
                        },
                    },
                    _ => {
                        problems.push("participating media the kernels have no code for".to_string());
                        blocked(o, ", a medium");
                    },
                }
                continue;
            }
            let prims = match o.geometry.flatten() {
                Some(prims) => Some(prims),
                None if o.medium.material_at(Vector3::zeros()).is_sky() => {
                    sky = Some(o.clone());
                    continue;
                },
                // Light passes it in part, which the stages' shadow rays can't say.
                None if o.geometry.transmits() => {
                    unflat_shapes += 1;
                    blocked(o, ", which light passes through");
                    continue;
                },
                // Kept whole, for its own code to intersect.
                None => None,
            };
            let m = match o.medium.flat_material() {
                Some(m) => m,
                // An emitter must be data: the stages' light sampling has to know it. So must
                // a medium of several parts' (a combined shape's), as a primitive doesn't
                // record its part.
                None if o.medium.material_at(Vector3::zeros()).is_emitter() || (prims.is_some() && o.medium.part(1).is_some()) => {
                    unflat_materials += 1;
                    blocked(o, ": its material");
                    continue;
                },
                None => {
                    hosts.push(o.clone());
                    FlatMaterial::Host { object: hosts.len() as u32 - 1 }
                },
            };
            // (An emitter has to be primitives too: the stages sample lights by their areas.)
            if prims.is_none() && matches!(m, FlatMaterial::Emitter { .. }) {
                unflat_shapes += 1;
                blocked(o, ", a light");
                continue;
            }
            let index = materials.len() as u32;
            materials.push(m);
            let Some(prims) = prims else {
                shapes.push((o.clone(), index));
                continue;
            };
            let by_face = o.geometry.faces_are_primitives();
            for (face, p) in prims.into_iter().enumerate() {
                let face = if by_face { face } else { 0 };
                match p {
                    Prim::Plane { .. } => planes.push((p, index)),
                    _ => bounded.push((p, index, face as u32)),
                }
            }
        }
        if unflat_shapes > 0 {
            problems.push(format!("{} of {} objects whose shapes can't be flattened (light passes through them in part, or they are lights that aren't spheres, triangles, meshes, boxes or flat quads)", unflat_shapes, shapes_seen));
        }
        if unflat_materials > 0 {
            problems.push(format!("{} of {} objects whose materials can't be flattened (textured lights, and combined shapes with a material a part)", unflat_materials, shapes_seen));
        }
        problems.dedup();
        if !blocking.is_empty() {
            const NAMED: usize = 6;
            let more = blocking.len().saturating_sub(NAMED);
            blocking.truncate(NAMED);
            problems.push(format!("in the way: {}{}", blocking.join(", "), if more > 0 { format!(" and {} more", more) } else { String::new() }));
        }
        if !problems.is_empty() {
            return Err(problems.join("; "));
        }
        let (bvh, bounded) = Bvh::build(bounded, |(p, _, _)| p.bounds());
        let prim_face = if hosts.is_empty() { Vec::new() } else { bounded.iter().map(|b| b.2).collect() };
        let (prims, prim_material): (Vec<Prim>, Vec<u32>) = bounded.into_iter().map(|(p, m, _)| (p, m)).unzip();
        // Emitters, by power: luminance times area (times two if two-sided).
        let mut emitters = Vec::new();
        let mut powers = Vec::new();
        let mut emitter_index = vec![None; prims.len()];
        for (i, p) in prims.iter().enumerate() {
            if let FlatMaterial::Emitter { radiance, two_sided } = &materials[prim_material[i] as usize] {
                let power = crate::color::luminance(radiance).max(0.) * p.area() * if *two_sided { 2. } else { 1. };
                if power > 0. {
                    emitter_index[i] = Some(emitters.len() as u32);
                    emitters.push(i as u32);
                    powers.push(power);
                }
            }
        }
        let emitter_choice = Distribution1D::new(if powers.is_empty() { vec![1.] } else { powers });
        Ok(FlatScene { prims, prim_material, bvh, planes, materials, emitters, emitter_choice, emitter_index, sky, shapes, hosts, prim_face, media })
    }

    pub fn material(&self, prim: u32) -> &FlatMaterial {
        &self.materials[self.prim_material[prim as usize] as usize]
    }

    /// The material of a hit's primitive (bounded or plane).
    /// The hit of `r` on primitive `prim` again (a hit some other arithmetic found: the GPU's,
    /// in f32), or on the shape that index is of.
    pub fn refine(&self, prim: u32, r: &Ray) -> Option<Hit> {
        let listed = self.prims.len() + self.planes.len();
        if (prim as usize) < listed {
            let (dist, geometric, shading) = self.prim(prim).intersect(r, 0., f64::INFINITY)?;
            return Some(Hit { dist, point: r.ro + r.rd * dist, geometric, shading, prim, face: self.face(prim), part: 0 });
        }
        let x = self.shapes[prim as usize - listed].0.geometry.intersects_range(r, MIN_SHAPE, f64::INFINITY)?;
        Some(Hit { dist: x.dist, point: x.point, geometric: x.normal, shading: x.normal, prim, face: x.face, part: x.part })
    }

    /// A primitive by a hit's index: bounded, or after those a plane.
    pub fn prim(&self, prim: u32) -> &Prim {
        let n = self.prims.len();
        if (prim as usize) < n { &self.prims[prim as usize] } else { &self.planes[prim as usize - n].0 }
    }

    /// A primitive's place among its object's (0 for a plane, or where no material needs it).
    pub fn face(&self, prim: u32) -> u32 {
        self.prim_face.get(prim as usize).copied().unwrap_or(0)
    }

    pub fn hit_material(&self, prim: u32) -> &FlatMaterial {
        let n = self.prims.len() as u32;
        let planes = self.planes.len() as u32;
        if prim < n {
            self.material(prim)
        } else if prim < n + planes {
            &self.materials[self.planes[(prim - n) as usize].1 as usize]
        } else {
            &self.materials[self.shapes[(prim - n - planes) as usize].1 as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{rand, seed_rand};
    use crate::scenefile::SceneFile;
    use crate::shapes::geometry::Geometry;

    fn random_ray() -> Ray {
        let ro = Vector3::new(rand() * 8. - 4., rand() * 8. - 4., rand() * 8. - 4.);
        let target = Vector3::new(rand() * 2. - 1., rand() * 2. - 1., rand() * 2. - 1.);
        Ray { ro, rd: (target - ro).normalize() }
    }

    /// Flattened shapes are hit where the shapes themselves are, with the same normals' sides.
    #[test]
    fn test_flattened_shapes_match_their_originals() {
        seed_rand(1, 0);
        let shapes: Vec<Box<dyn Geometry + Sync + Send>> = vec![
            Box::new(crate::shapes::sphere::Sphere::new(Vector3::new(0.2, -0.1, 0.3), 0.9)),
            Box::new(BBox::new(Vector3::new(-0.7, -0.5, -0.9), Vector3::new(0.8, 0.6, 0.4))),
            Box::new(crate::shapes::transform::Transform::new(
                std::sync::Arc::new(crate::shapes::sphere::Sphere::new(Vector3::zeros(), 0.5)) as std::sync::Arc<dyn Geometry + Sync + Send>,
                crate::na::Affine3::from_matrix_unchecked(crate::na::Matrix4::new_scaling(1.5) * crate::na::Matrix4::new_translation(&Vector3::new(0.3, 0., 0.))))),
            // Mirrored: the winding reverses, and the normals must still point out.
            Box::new(crate::shapes::transform::Transform::new(
                std::sync::Arc::new(BBox::new(Vector3::new(-0.7, -0.5, -0.9), Vector3::new(0.8, 0.6, 0.4))) as std::sync::Arc<dyn Geometry + Sync + Send>,
                crate::na::Affine3::from_matrix_unchecked(crate::na::Matrix4::new_nonuniform_scaling(&Vector3::new(-1., 1., 1.2))))),
            Box::new(crate::shapes::transform::Transform::new(
                std::sync::Arc::new(crate::shapes::bilinear::BilinearPatch::new(Vector3::new(-1., 0.2, -1.), Vector3::new(1., 0.2, -1.), Vector3::new(-1., 0.2, 1.), Vector3::new(1., 0.2, 1.))) as std::sync::Arc<dyn Geometry + Sync + Send>,
                crate::na::Affine3::from_matrix_unchecked(crate::na::Matrix4::new_nonuniform_scaling(&Vector3::new(1., -1., 1.))))),
        ];
        for shape in &shapes {
            let prims = shape.flatten().unwrap();
            for _ in 0..2000 {
                let r = random_ray();
                let original = shape.intersects(&r).filter(|h| h.dist > 1e-9);
                let flat = prims.iter().filter_map(|p| p.intersect(&r, 1e-9, f64::INFINITY)).min_by(|a, b| a.0.total_cmp(&b.0));
                match (original, flat) {
                    (None, None) => (),
                    (Some(o), Some(f)) => {
                        assert!((o.dist - f.0).abs() < 1e-6, "{} vs {}", o.dist, f.0);
                        assert!(o.normal.dot(&f.1) > 0.99, "{:?} vs {:?}", o.normal, f.1);
                    },
                    (o, f) => panic!("{:?} vs {:?}", o.map(|o| o.dist), f.map(|f| f.0)),
                }
            }
        }
    }

    #[test]
    fn test_scenes_that_cant_be_flattened_say_why() {
        let s = SceneFile::from_string(r#"{
            "camera": { "location": [0, 1, -3], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "N": { "type": "noise" }, "L": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] } },
            "objects": [ { "type": "torus", "center": [0, 0, 0], "major": 1, "minor": 0.2, "material": "L" },
                         { "type": "sphere", "location": [0, 0, 0], "radius": 1, "material": "L" },
                         { "type": "volume", "min": [-1, -1, -1], "max": [1, 1, 1], "noise": { "scale": 1 } } ]
        }"#.to_string());
        let e = FlatScene::build(&s).err().unwrap();
        // The torus is kept, for its own code to intersect; the medium is what is in the way:
        // one of noise, which no kernel has the code for (a constant one is carried as data).
        assert!(e.contains("participating media") && e.contains("objects[2]") && !e.contains("objects[0]"), "{}", e);
        let ok = SceneFile::from_string(r#"{
            "camera": { "location": [0, 1, -3], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 },
            "materials": { "L": { "type": "lambertian", "albedo": [0.5, 0.5, 0.5] }, "E": { "type": "diffuse-light", "color": [1, 1, 1] } },
            "objects": [ { "type": "box", "min": [-1, -1, -1], "max": [1, 1, 1], "material": "L" },
                         { "type": "plane", "y": -1, "material": "L" },
                         { "type": "sphere", "location": [0, 3, 0], "radius": 0.5, "material": "E" } ]
        }"#.to_string());
        let f = FlatScene::build(&ok).unwrap();
        assert_eq!((f.prims.len(), f.planes.len(), f.emitters.len()), (13, 1, 1));
        assert!(f.media.is_empty());
    }
}
