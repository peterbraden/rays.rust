use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;
use na::Vector3;
use light::Light;
use geometry::{reflect};

pub struct Whitted {
    pub pigment: Color,
    pub reflection: f64,
    pub phong: f64,
}

impl Whitted {
    fn specular (&self, r: &Ray, intersection: &Intersection, light_vec: &Vector3<f64>) -> Color {
        if self.phong == 0. {
            return Color::black();
        }
        let ln = light_vec.normalize();
        let refl = ln - (intersection.normal * (2.0 * intersection.normal.dot(&ln) ) ); 
        let dp = refl.dot(&r.rd);

        if dp > 0f64 {
            let spec_scale = dp.powf(self.phong);
            return Color::white() * spec_scale;
        }

        return Color::black();
    }


    fn diffuse (&self, i: &Intersection, light_vec: &Vector3<f64>, light: &Light) -> Color {
        let diffuse_scale = light_vec.normalize().dot(&i.normal) * light.intensity;
        if diffuse_scale.is_sign_positive() {
            return light.color * self.pigment * diffuse_scale;
        }
        return Color::black()
    }


    fn trace_for_light(&self, r: &Ray, light_vec: &Vector3<f64>, l: &Light, intersection: &Intersection) -> Color {
        return self.diffuse(&intersection, &light_vec, &l) + self.specular(r, intersection, light_vec);
    }
}

impl MaterialModel for Whitted {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay{
        let mut out = Color::black();
        for light in &s.lights {
            let light_vec = light.position - intersection.point;
            let shadow_ray = Ray {ro: intersection.point, rd: light_vec};
            let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), 0.1); 

            match shadow_intersection {
                Some(_) => (),// Point in shadow...
                None => (
                    out = out + self.trace_for_light(&r, &light_vec, &light, &intersection)
                    ),
            }
        }

        if self.reflection > 0. {
            let refl = Ray {
                ro: intersection.point,
                rd: reflect(r.rd, intersection.normal)
            };
            return ScatteredRay{ attenuate: out * self.reflection, ray: Some(refl) };
        }
        return ScatteredRay{ attenuate: out, ray: None };
    }
}



