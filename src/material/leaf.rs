use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_in_unit_ball, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, direct_lighting, fresnel_reflectance, reflect};

/// Distance to move a transmitted ray's origin, and the point its lights are sampled from,
/// through the surface. As for dielectrics, it must exceed the scene's shadow bias, which has
/// already moved the intersection point towards the side the ray arrived from.
const THROUGH: f64 = 0.001;

/// A thin, translucent surface, such as a leaf or a sheet of paper: it reflects light diffusely
/// on the side it arrives at and transmits it diffusely out of the other side, under a thin,
/// glossy coat.
///
/// - The diffuse part is pbrt-v4's DiffuseTransmissionBxDF (Matt Pharr, Wenzel Jakob and Greg
///   Humphreys, "Physically Based Rendering", 4th ed., 2023, <https://pbr-book.org/4ed/>): a
///   Lambertian lobe of albedo `reflectance` on the near side and one of albedo `transmittance`
///   on the far side. Leaves are close to this: the cells inside a leaf scatter light many
///   times, so what leaves either side is nearly Lambertian. Backlit leaves glow because of it.
/// - Real leaves reflect and transmit about equally, both mostly green: of the order of 10%
///   each in green light, and a few per cent in red and blue, which the chlorophyll absorbs.
///   See Stéphane Jacquemoud and Frédéric Baret, "PROSPECT: A Model of Leaf Optical Properties
///   Spectra", Remote Sensing of Environment 34(2), 75-91, 1990,
///   doi:10.1016/0034-4257(90)90100-Z, whose measured and modelled spectra show this.
/// - The coat is the leaf's waxy cuticle: a specular reflection with the Fresnel probability
///   for `refractive_index` (about 1.5 for plant waxes), fuzzed by `roughness`, as in `Plastic`.
///   It gives leaves their sheen, and their pale, sky-tinted look at grazing angles.
///
/// Shadow rays are still all-or-nothing, so leaves cast opaque shadows: light transmitted
/// through one leaf onto another is lost. At around 10% transmission, that's small beside the
/// light that does arrive.
pub struct Leaf {
    pub reflectance: Color,
    pub transmittance: Color,
    pub refractive_index: f64,
    pub roughness: f64,
}

fn luminance(c: &Color) -> f64 {
    0.2126 * c.rgb.x + 0.7152 * c.rgb.y + 0.0722 * c.rgb.z
}

impl MaterialModel for Leaf {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let near = intersection.facing(r);
        let cos_i = (r.rd.dot(&near.normal) / r.rd.norm()).abs();
        if self.refractive_index > 1. && rand() < fresnel_reflectance(cos_i, 1. / self.refractive_index, self.refractive_index) {
            // The coat: reflect, untinted.
            let rd = reflect(r.rd, near.normal) + random_point_in_unit_ball() * self.roughness;
            if rd.dot(&near.normal) <= 0. {
                // Fuzz has pushed the reflection into the surface: absorb it.
                return ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::black(), ray: None };
            }
            return ScatteredRay { sampled_lights: false, emit: Color::black(), attenuate: Color::white(), ray: Some(Ray { ro: near.point, rd }) };
        }

        // The far side, for light transmitted through the leaf.
        let far = Intersection { point: near.point - near.normal * THROUGH, normal: -near.normal, ..near };

        // Direct light from both sides: lights on each side are only seen from that side.
        let emit = direct_lighting(self.reflectance, &near, s) + direct_lighting(self.transmittance, &far, s);

        // Continue the path by reflecting or transmitting, in proportion to how much of each the
        // leaf does, and weight by the inverse of that probability.
        let (lr, lt) = (luminance(&self.reflectance), luminance(&self.transmittance));
        if lr + lt <= 0. {
            return ScatteredRay { sampled_lights: true, emit, attenuate: Color::black(), ray: None };
        }
        let p_reflect = lr / (lr + lt);
        let continued = if rand() < p_reflect {
            scatter_lambertian(self.reflectance * (1. / p_reflect), &near)
        } else {
            scatter_lambertian(self.transmittance * (1. / (1. - p_reflect)), &far)
        };
        ScatteredRay { emit, sampled_lights: true, ..continued }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::na::Vector3;
    use crate::scenefile::SceneFile;
    use crate::trace::trace;

    /// A horizontal leaf at y = 0, a point light `light_y` above or below it, and nothing else.
    fn leaf_scene(leaf: &str, light_y: f64) -> Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1, "shadow_bias": 0.0001,
            "camera": {{ "location": [0, 10, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 }},
            "materials": {{ "L": {leaf} }},
            "lights": [ {{ "location": [0, {light_y}, 0], "intensity": 314.1592653589793 }} ],
            "objects": [ {{ "type": "triangle", "v0": [-1, 0, -1], "v1": [1, 0, -1], "v2": [0, 0, 1], "material": "L" }} ]
        }}"#))
    }

    fn mean_from_above(s: &Scene) -> Color {
        let r = Ray { ro: Vector3::new(0., 5., 0.01), rd: Vector3::new(0., -1., 0.) };
        let n = 2000;
        (0 .. n).fold(Color::black(), |acc, _| acc + trace(&r, 0, s).1) / n as f64
    }

    const LEAF: &str = r#"{ "type": "leaf", "reflectance": [0.1, 0.2, 0.05], "transmittance": [0.05, 0.15, 0.02], "refractive_index": 1 }"#;

    #[test]
    fn test_reflects_light_from_the_near_side() {
        // A light 10 above: irradiance 314.16 / 100 = pi, so radiance reflectance * pi / pi.
        let c = mean_from_above(&leaf_scene(LEAF, 10.));
        // The shadow bias moves the shading point 0.1 mm towards the light.
        assert!((c.rgb.y - 0.2).abs() < 1e-3 && (c.rgb.x - 0.1).abs() < 1e-3, "{}", c);
    }

    #[test]
    fn test_transmits_light_from_the_far_side() {
        // The same light below the leaf shows through it, tinted by the transmittance.
        let c = mean_from_above(&leaf_scene(LEAF, -10.));
        assert!((c.rgb.y - 0.15).abs() < 1e-3 && (c.rgb.z - 0.02).abs() < 1e-3, "{}", c);
    }

    #[test]
    fn test_opaque_without_transmittance() {
        let opaque = r#"{ "type": "leaf", "reflectance": [0.1, 0.2, 0.05], "transmittance": [0, 0, 0], "refractive_index": 1 }"#;
        assert_eq!(mean_from_above(&leaf_scene(opaque, -10.)), Color::black());
    }

    #[test]
    fn test_path_continues_in_proportion() {
        // Bounce weights are reflectance or transmittance over their probability: on average,
        // a bounce carries as much as the leaf scatters, R + T.
        let s = leaf_scene(LEAF, 10.);
        let leaf = Leaf { reflectance: Color::new(0.1, 0.2, 0.05), transmittance: Color::new(0.05, 0.15, 0.02), refractive_index: 1., roughness: 0. };
        let r = Ray { ro: Vector3::new(0., 5., 0.01), rd: Vector3::new(0., -1., 0.) };
        let (object, hit) = s.objects.nearest_raw_intersection(&r, f64::INFINITY, 0.).unwrap();
        let i = Intersection { dist: hit.dist, point: hit.point, normal: hit.normal, object };
        let n = 20000;
        let (mut total, mut through) = (Color::black(), 0);
        for _ in 0 .. n {
            let out = leaf.scatter(&r, &i, &s);
            total = total + out.attenuate;
            if out.ray.unwrap().rd.y < 0. { through += 1; }
        }
        let mean = total / n as f64;
        assert!((mean.rgb.y - 0.35).abs() < 0.02, "{}", mean);
        // Transmitted with probability lum(T) / (lum(R) + lum(T)).
        let p = luminance(&leaf.transmittance) / (luminance(&leaf.transmittance) + luminance(&leaf.reflectance));
        assert!((through as f64 / n as f64 - p).abs() < 0.02, "{} vs {}", through, p);
    }

    #[test]
    fn test_coat_reflects_at_grazing_angles() {
        // With a coat and nothing transmitted or reflected diffusely, light arriving at a
        // grazing angle is mostly reflected; at normal incidence, about 4%.
        let s = leaf_scene(r#"{ "type": "leaf", "reflectance": [0, 0, 0], "transmittance": [0, 0, 0], "refractive_index": 1.5 }"#, 10.);
        let object = &s.objects.items()[0];
        let leaf = Leaf { reflectance: Color::black(), transmittance: Color::black(), refractive_index: 1.5, roughness: 0. };
        let fraction = |rd: Vector3<f64>| {
            let i = Intersection { dist: 1., point: Vector3::zeros(), normal: Vector3::new(0., 1., 0.), object };
            let r = Ray { ro: -rd, rd };
            (0 .. 20000).filter(|_| leaf.scatter(&r, &i, &s).ray.is_some()).count() as f64 / 20000.
        };
        assert!((fraction(Vector3::new(0., -1., 0.)) - 0.04).abs() < 0.01);
        assert!(fraction(Vector3::new(1., -0.02, 0.).normalize()) > 0.6);
    }
}
