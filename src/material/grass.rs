use crate::material::model::LightSampling;
use crate::color::Color;
use crate::na::Vector3;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{rand, random_point_on_unit_sphere, random_point_in_unit_ball};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{direct_lighting, fresnel_reflectance, reflect};

/// How far to move a transmitted ray's origin through the leaf, beyond the shadow bias that put
/// the hit point on the near side.
const THROUGH_LEAF: f64 = 1e-4;

/// A thin leaf: light is reflected diffusely from the side it arrives on, transmitted
/// diffusely out of the other side, or - with the Fresnel probability, scaled by `sheen` -
/// reflected by the waxy cuticle as a glossy highlight.
///
/// Leaves transmit a substantial part of the light they don't absorb, and what comes through is
/// more saturated and yellower than what is reflected (Jacquemoud and Ustin, "Leaf Optical
/// Properties", Cambridge University Press 2019, ch. 1; for grass blades, about a third to a
/// half of the light leaving the leaf). Modelling a blade as a two-sided Lambertian reflector
/// and transmitter is the usual approximation for thin vegetation, eg. the "thin dielectric"
/// leaves in pbrt-v4's diffuse transmission material (Pharr, Jakob and Humphreys, "Physically
/// Based Rendering", 4th ed., section 9.7.1).
///
/// Back-lit grass glowing against the sun is the transmission term, so it matters for the look.
pub struct GrassMaterial {
    /// Diffuse reflectance.
    pub reflect: Color,
    /// Diffuse transmittance.
    pub transmit: Color,
    /// Scale for the cuticle's Fresnel reflection: 0 for none, 1 for a full waxy or wet coat.
    pub sheen: f64,
    /// Fuzz for the cuticle's reflection.
    pub roughness: f64,
}

/// Cosine-weighted direction about `n`.
fn diffuse_direction(n: Vector3<f64>) -> Vector3<f64> {
    let d = n + random_point_on_unit_sphere();
    if d.norm() < 1e-9 { n } else { d.normalize() }
}

impl GrassMaterial {
    /// Probability of following transmission rather than reflection for the diffuse bounce.
    fn transmit_probability(&self) -> f64 {
        let (r, t) = (self.reflect.as_vec().sum(), self.transmit.as_vec().sum());
        if r + t <= 0. { 0. } else { t / (r + t) }
    }
}

impl MaterialModel for GrassMaterial {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let front = intersection.facing(r);
        let n = front.normal;

        if self.sheen > 0. {
            let cos_i = (r.rd.dot(&n) / r.rd.norm()).abs();
            let coat = self.sheen * fresnel_reflectance(cos_i, 1. / 1.45, 1.45);
            if rand() < coat {
                let rd = reflect(r.rd.normalize(), n) + random_point_in_unit_ball() * self.roughness;
                if rd.dot(&n) <= 0. {
                    return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::black(), ray: None };
                }
                return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(), ray: Some(Ray { ro: front.point, rd }) };
            }
        }

        // The far side of the blade: a point just through it, facing away from the ray.
        let back = Intersection {
            point: front.point - n * (2. * s.render.shadow_bias + THROUGH_LEAF),
            normal: -n,
            ..front
        };
        let (reflect, transmit) = (self.reflect.as_reflectance(), self.transmit.as_reflectance());
        let emit = direct_lighting(reflect, &front, s) + direct_lighting(transmit, &back, s);

        let p = self.transmit_probability();
        let (side, weight) = if rand() < p {
            (&back, transmit * (1. / p))
        } else {
            (&front, reflect * (1. / (1. - p)))
        };
        ScatteredRay {
            emit,
            light_sampling: LightSampling::Exclusive,
            attenuate: weight,
            ray: Some(Ray { ro: side.point, rd: diffuse_direction(side.normal) }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmit_probability() {
        let m = GrassMaterial { reflect: Color::new(0.2, 0.4, 0.1), transmit: Color::new(0.1, 0.2, 0.05), sheen: 0., roughness: 0. };
        assert!((m.transmit_probability() - 1. / 3.).abs() < 1e-12);
        let opaque = GrassMaterial { transmit: Color::black(), ..m };
        assert_eq!(opaque.transmit_probability(), 0.);
    }
}
