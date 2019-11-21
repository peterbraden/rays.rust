use color::Color;
use ray::Ray;
use scene::Scene;
use sceneobject::SceneObject;
use material::model::{ MaterialModel, ScatteredRay };
use serde_json::{Value, Map};
use shapes::bbox::BBox;
use intersection::{Intersection, RawIntersection};
extern crate rand as _rand;
use participatingmedia::_rand::Rng;
use std::f64;
use na::{Vector3};
use shapes::geometry::Geometry;
use material::texture::{Solid, Medium};
use geometry::random_point_on_unit_sphere;
use scenefile::SceneFile;

const BIG_NUMBER:f64 = 1000.;

pub fn rand() -> f64 {
    return _rand::thread_rng().gen_range(0.,1.);
}

pub trait ParticipatingMedium: MaterialModel {}

pub struct Vacuum {}
impl ParticipatingMedium for Vacuum {}
impl MaterialModel for Vacuum {
    fn scatter(&self, _r: &Ray, _i: &Intersection, _s: &Scene) -> ScatteredRay {
        return ScatteredRay {
            ray: None,
            attenuate: Color::white(),
        }
    }
}

#[derive(Clone)]
pub struct HomogenousFog {
    pub color: Color,
    pub density: f64,
    pub scatter: f64,
}
impl ParticipatingMedium for HomogenousFog{}
impl MaterialModel for HomogenousFog {
    fn scatter(&self, r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
       // let amount = i.dist * self.density;
        return ScatteredRay {
            ray: Some(Ray {
                ro: i.point,
                rd: (r.rd + (random_point_on_unit_sphere() * self.scatter * rand())).normalize(),
            }),
            attenuate: self.color,
        }
    }
}

impl Geometry for HomogenousFog {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        if rand() < self.density {
            let dist = rand().powf(3.) * BIG_NUMBER; 
            return Some(RawIntersection {
                dist: dist,
                point: r.ro + r.rd * dist,
                normal: r.rd,
            })
        }
        return None
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN),
            Vector3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX),
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
        return ScatteredRay {
            ray: None,
            attenuate: Color::white(),
        }
    }

}


pub fn create_fog(o: &Value) -> SceneObject {
    let fog = HomogenousFog {
        color: SceneFile::parse_color_def(&o, "color", Color::new(0.1, 0.1, 0.1)),
        density: SceneFile::parse_number(&o["density"], 0.2),
        scatter: SceneFile::parse_number(&o["scatter"], 0.01),
    };
	return SceneObject {
		geometry: Box::new(fog.clone()),
		medium: Box::new(Solid { m: Box::new(fog)}),
	}
}
