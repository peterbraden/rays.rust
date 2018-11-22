use color::Color;
use scene::Scene;
use material::model::{MaterialModel, ScatteredRay};
use intersection::Intersection;
use ray::Ray;
use geometry::{rand, schlick, reflect};
use na::{Vec3, Dot, Norm};

pub struct Dielectric {
    pub refractive_index: f64,
    pub attenuate: Color,
}

fn refract(v: Vec3<f64>, n: Vec3<f64>, ni_over_nt:f64) -> Option<Vec3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        return Some( (uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    }
    None
}

impl MaterialModel for Dielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let mut ni_over_nt = self.refractive_index; // Assumes it comes from air - TODO
        let cosine;
        let drn = r.rd.dot(&intersection.normal);
        let outward_normal;
        if drn > 0.0 {
            // when ray shoot through object back into vacuum,
            // ni_over_nt = ref_idx, surface normal has to be inverted.
            cosine = drn / r.rd.norm(); 
            outward_normal = -intersection.normal
        } else {
            // when ray shoots into object,
            // ni_over_nt = 1 / ref_idx.
            cosine = - drn / r.rd.norm(); 
            ni_over_nt = 1.0 / self.refractive_index; 
            outward_normal = intersection.normal
        };

        match refract(r.rd, outward_normal, ni_over_nt) {
            Some(refracted) => {
                // refracted ray exists
                let reflect_prob = schlick(cosine, self.refractive_index);
                if rand() >= reflect_prob {
                    return ScatteredRay{
                        attenuate: self.attenuate,
                        ray: Some( Ray {
                            ro: intersection.point + (refracted * 0.001),
                            rd: refracted
                        }),
                    };
                }
            },
            None => {
                // refracted ray does not exist
                //  - total internal reflection
            }
        }

        let reflected = reflect(r.rd, intersection.normal);
        return ScatteredRay{
            attenuate: self.attenuate,
            ray: Some(Ray {
                ro: intersection.point,
                rd: reflected
            }) 
        };
    }
}
