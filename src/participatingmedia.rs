use color::Color;
use ray::Ray;
use intersection::Intersection;
use scene::Scene;
use material::model::{ MaterialModel, ScatteredRay };

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

pub struct HomogenousFog {
    pub color: Color,
    pub density: f64,
}
impl ParticipatingMedium for HomogenousFog{}
impl MaterialModel for HomogenousFog {
    fn scatter(&self, _r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
        let amount = i.dist * self.density;
        return ScatteredRay {
            ray: None,
            attenuate: Color::white() * (1. - amount) + self.color * amount,
        }
    }
}

pub struct LowAltitudeFog {
    density: f64,
    color: Color,
    max_altitude: f64,
    falloff: f64,
}
impl MaterialModel for LowAltitudeFog {
    fn scatter(&self, _r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
        let amount = i.dist * self.density;
        // TODO
        return ScatteredRay {
            ray: None,
            attenuate: Color::white(),
        }
    }

}

