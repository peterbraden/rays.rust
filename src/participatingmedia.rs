use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sceneobject::SceneObject;
use crate::material::model::{ MaterialModel, ScatteredRay };
use serde_json::{Value, Map};
use crate::shapes::bbox::BBox;
use crate::intersection::{Intersection, RawIntersection};
use std::f64;
use crate::na::{Vector3};
use crate::shapes::geometry::Geometry;
use crate::material::texture::{Solid, Medium};
use crate::geometry::random_point_in_unit_ball;
use crate::scenefile::SceneFile;

pub use crate::geometry::rand;

pub trait ParticipatingMedium: MaterialModel {}

pub struct Vacuum {}
impl ParticipatingMedium for Vacuum {}
impl MaterialModel for Vacuum {
    fn scatter(&self, _r: &Ray, _i: &Intersection, _s: &Scene) -> ScatteredRay {
        ScatteredRay { sampled_lights: false, emit: Color::black(),
            ray: None,
            attenuate: Color::white(),
        }
    }
}

#[derive(Clone)]
pub struct HomogenousFog {
    pub color: Color,
    /// Extinction coefficient: the probability of scattering per unit distance travelled.
    pub density: f64,
    pub scatter: f64,
}
impl ParticipatingMedium for HomogenousFog{}
impl MaterialModel for HomogenousFog {
    fn scatter(&self, r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
       // let amount = i.dist * self.density;
        ScatteredRay { sampled_lights: false, emit: Color::black(),
            ray: Some(Ray {
                ro: i.point,
                rd: (r.rd + (random_point_in_unit_ball() * self.scatter * rand())).normalize(),
            }),
            attenuate: self.color,
        }
    }
}

impl Geometry for HomogenousFog {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        if self.density > 0. {
            // Sample a free-flight distance (Beer-Lambert). Nearer surfaces occlude it.
            let dist = -(1. - rand()).ln() / self.density;
            Some(RawIntersection { part: 0,
                dist,
                point: r.ro + r.rd * dist,
                normal: r.rd,
            })
        } else {
            None
        }
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(f64::MIN, f64::MIN, f64::MIN),
            Vector3::new(f64::MAX, f64::MAX, f64::MAX),
          )
    }
}

pub struct LowAltitudeFog {
    density: f64,
    color: Color,
    max_altitude: f64,
    falloff: f64,
}
impl MaterialModel for LowAltitudeFog {
    fn scatter(&self, _r: &Ray, _i: &Intersection, _s: &Scene) -> ScatteredRay {
        //let amount = i.dist * self.density;
        // TODO
        ScatteredRay { sampled_lights: false, emit: Color::black(),
            ray: None,
            attenuate: Color::white(),
        }
    }

}


pub fn create_fog(o: &Value) -> SceneObject {
    let fog = HomogenousFog {
        color: SceneFile::parse_color_def(o, "color", Color::new(0.1, 0.1, 0.1)),
        density: SceneFile::parse_number(&o["density"], 0.002),
        scatter: SceneFile::parse_number(&o["scatter"], 0.01),
    };
	SceneObject {
		geometry: Box::new(fog.clone()),
		medium: Box::new(Solid { m: Box::new(fog)}),
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fog_follows_beer_lambert() {
        let fog = HomogenousFog { color: Color::white(), density: 0.5, scatter: 0. };
        let r = Ray { ro: Vector3::new(0., 0., 0.), rd: Vector3::new(0., 0., 1.) };
        let samples = 20_000;
        let within = (0..samples)
            .filter(|_| fog.intersects(&r).map_or(false, |i| i.dist < 2.))
            .count() as f64 / samples as f64;
        // 1 - exp(-0.5 * 2)
        assert!((within - 0.632).abs() < 0.02, "{}", within);
    }

    #[test]
    fn test_zero_density_fog_is_clear() {
        let fog = HomogenousFog { color: Color::white(), density: 0., scatter: 0. };
        let r = Ray { ro: Vector3::new(0., 0., 0.), rd: Vector3::new(0., 0., 1.) };
        assert!(fog.intersects(&r).is_none());
    }
}
