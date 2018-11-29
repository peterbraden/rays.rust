// Use old diffuse / specular model
//
use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;

pub struct Diffuse {
    pub pigment: Color,
}

fn specular (r: &Ray, intersection: &Intersection, light_vec: &Vec3<f64>, s: &Scene) -> Color {
    let phong = intersection.object.medium.material_at(intersection.point).phong;
    if !s.specular || phong == 0. {
        return Color::black();
    }
    let ln = light_vec.normalize();
    let refl = ln - (intersection.normal * (2.0 * intersection.normal.dot(&ln) ) ); 
    let dp = refl.dot(&r.rd);

    if dp > 0f64 {
        let spec_scale = dp.powf(phong);
        return Color::white() * spec_scale;
    }

    return Color::black();
}

fn diffuse (i: &Intersection, light_vec: &Vec3<f64>, light: &Light, s: &Scene) -> Color {
    if !s.diffuse {
        return Color::black();
    }

    let diffuse_scale = light_vec.normalize().dot(&i.normal) * light.intensity;
    if diffuse_scale.is_sign_positive() {
        return light.color * i.object.medium.material_at(i.point).pigment * diffuse_scale;
    }
    return Color::black()
}


fn trace_for_light(r: &Ray, light_vec: &Vec3<f64>, l: &Light, intersection: &Intersection, s: &Scene) -> Color {
    return diffuse(&intersection, &light_vec, &l, s) + specular(r, intersection, light_vec, s);
}

impl MaterialModel for Ambient {
    fn scatter(&self, _r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{

        for light in &s.lights {
            let light_vec = light.position - biased_intersection.point;
            let shadow_ray = Ray {ro: biased_intersection.point, rd: light_vec};
            let shadow_intersection = s.objects.nearest_intersection(&shadow_ray, light_vec.norm(), 0.1, Some(intersection.object)); 
            cast = cast + 1;

            match shadow_intersection {
                Some(_) => (
                        // Point in shadow...
                    ),
                None => (
                    out = out + trace_for_light(&r, &light_vec, &light, &biased_intersection, &s)
                    ),
            }
        }

        return ScatteredRay{ attenuate:self.pigment, ray: None };
    }
}



