use serde_json::{Value, Map};
use crate::sceneobject::SceneObject;
use crate::shapes::sphere::Sphere;
use crate::na::{Vector3};
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, scatter_dielectric, diffuse};
use crate::shapes::geometry::Geometry;
use crate::scenefile::SceneFile;
use crate::material::normal::NormalShade;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use rand::distributions::{Normal, Distribution};
use crate::geometry::point_on_unit_sphere;
use crate::shapes::csg::Union;
use crate::octree::Octree;
use crate::shapes::bbox::BBox;
use crate::intersection::RawIntersection;
use std::sync::Arc;
use std::f64;

fn rand(rng: &mut StdRng) -> f64 {
    rng.gen_range(0.0, 1.0)
}

struct Particle {
    s: Sphere,
    intensity: f64,
}

impl Geometry for Particle {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> { self.s.intersects(r) }
    fn bounds(&self) -> BBox{ self.s.bounds() }
}


// Misuse a ray as the initial position and speed vector.
// assume gravity is down y-axis
fn trace_particle(impulse: Ray, time: f64, samples: usize, gravity: f64) -> Vec<Particle>{
    let mut particles = Vec::new();

    for x in 0..samples {
        let t = (time / samples as f64) * x as f64;
        let sink = impulse.rd.y * t - 0.5 * gravity * t * t;
        let position= Vector3::new(
            impulse.ro.x + impulse.rd.x * t,
            impulse.ro.y + sink,
            impulse.ro.z + impulse.rd.z * t
        );
        let intensity = (t/time).powf(3.);
        let radius = intensity * 0.1;
        let p = Particle {
            s: Sphere::new(position, radius),
            intensity,
        };
        particles.push(p);
    }
    particles
}

#[allow(clippy::too_many_arguments)]
fn create_particles(
        rng: &mut StdRng,
        origin: Vector3<f64>,
        time: f64,
        radius: f64,
        samples: usize,
        gravity: f64,
        min_particles: usize,
        max_particles: usize,
        upward_bias: f64,
    ) -> Vec<Particle> {
    let mut particles = Vec::new();
    let num_particles = min_particles + ((rand(rng) * (max_particles - min_particles) as f64) as usize);
    let bias = Vector3::new(0., upward_bias, 0.);
    for _p in 0 .. num_particles {
        let u = rand(rng);
        let v = rand(rng);
        let impulse = Ray {
            ro: origin,
            rd: (point_on_unit_sphere(u, v) * radius) + bias,
        };
        particles.append(&mut trace_particle(impulse, time, samples, gravity));
    }
    particles
}


pub struct FireworkMaterial {
    particles: Octree<Particle>,
    color: Color,
}

impl MaterialModel for FireworkMaterial {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        // Find actual particle. Kinda hacky as we already worked this out.
        let actual_intersection = self.particles.intersection(r, f64::INFINITY, 0f64);
        match actual_intersection {
            Some((particle, _i)) => {
                ScatteredRay {
                    attenuate: self.color * particle.intensity,
                    ray: None,
                }
            },
            None => {
                // Should never happen
                panic!("Firework didn't intersect")
            }
        }
    }
}

// Radius is the radius of explosion at time=1.
pub fn create_firework(o: &Value) -> SceneObject {

    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

    let center = SceneFile::parse_vec3_def(o, "center", Vector3::new(0., 10., 0.));
    let time = SceneFile::parse_number(&o["time"], 0.9);
    let radius = SceneFile::parse_number(&o["radius"], 10.);
    let samples = SceneFile::parse_number(&o["samples"], 10.) as usize;
    let gravity = SceneFile::parse_number(&o["radius"], 9.8);
    let num_particles = SceneFile::parse_number(&o["particles"], 100.) as usize;
    let upward_bias = SceneFile::parse_number(&o["upward_bias"], 2.);
    let intensity = SceneFile::parse_number(&o["intensity"], 2.);
    let color = SceneFile::parse_color_def(o, "color", Color::white()) * intensity;

    let particles = create_particles(
                        &mut rng,
                        center,
                        time,
                        radius,
                        samples,
                        gravity,
                        num_particles,
                        num_particles,
                        upward_bias
                        );
    let boxed_particles = particles
                            .iter()
                            .map(|p| Box::new(p.s.clone()) as Box<dyn Geometry + Sync + Send>)
                            .collect();
    let geom = Union::new(boxed_particles);

    let particle_arcs: Vec<Arc<Particle>> = particles.into_iter().map(Arc::new).collect();
    let tree = Octree::new(8, geom.bounds(), &particle_arcs);
    let m = Box::new(FireworkMaterial { particles: tree, color });

	SceneObject {
		geometry: Box::new(geom),
		medium: Box::new(Solid { m }),
	}
}
