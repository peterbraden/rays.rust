use crate::material::model::LightSampling;
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_in_unit_ball, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, direct_lighting, fresnel_reflectance, reflect};
use std::sync::Arc;

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
///
/// ## Autumn and dead leaves
///
/// Given `states`, each leaf (by the hit's `face`) takes a colour between three: its healthy
/// green, its autumn colour and its dead colour. This follows what happens to the pigments, as
/// described by David W. Lee and Kevin S. Gould, "Why Leaves Turn Red", American Scientist
/// 90(6), 524-531, 2002, and measured through a season by David W. Lee, John O'Keefe, N. Michele
/// Holbrook and Taylor S. Feild, "Pigment dynamics and autumn leaf senescence in a New England
/// deciduous forest, eastern USA", Ecological Research 18(6), 677-694, 2003:
/// - In autumn the leaf breaks down its chlorophyll, unmasking its yellow and orange
///   carotenoids: the default `autumn` colour is yellow.
/// - Many species also make red anthocyanins, most in leaves exposed to the sun: `autumn_sun`,
///   the colour of a fully exposed leaf, is blended in by each leaf's exposure.
/// - When the leaf dies, brown oxidised compounds dominate and it dries, so it transmits less:
///   the `dead` colour. PROSPECT-D models these "brown pigments" alongside the others: Jean-
///   Baptiste Féret et al., "PROSPECT-D: Towards modeling leaf optical properties through a
///   complete lifecycle", Remote Sensing of Environment 193, 204-215, 2017,
///   doi:10.1016/j.rse.2017.03.004.
///
/// The colours between are linear blends of reflectance and transmittance, which keeps them
/// physically possible if the ends are.
pub struct Leaf {
    /// The healthy leaf's colour.
    pub reflectance: Color,
    pub transmittance: Color,
    pub refractive_index: f64,
    pub roughness: f64,
    /// Autumn colour of a shaded leaf, and of one fully in the sun.
    pub autumn: LeafColor,
    pub autumn_sun: LeafColor,
    pub dead: LeafColor,
    /// Each leaf's state, indexed by face; without it, every leaf is healthy.
    pub states: Option<Arc<Vec<LeafState>>>,
}

/// A colour for a leaf: how much light of each colour it reflects and transmits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafColor {
    pub reflectance: Color,
    pub transmittance: Color,
}

impl LeafColor {
    fn mix(&self, other: &LeafColor, t: f64) -> LeafColor {
        let t = t.clamp(0., 1.);
        LeafColor {
            reflectance: self.reflectance * (1. - t) + other.reflectance * t,
            transmittance: self.transmittance * (1. - t) + other.transmittance * t,
        }
    }
}

/// One leaf's state.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LeafState {
    /// 0 healthy, 1 in full autumn colour, 2 dead; between, a blend.
    pub age: f32,
    /// 0 shaded to 1 fully exposed to the sun.
    pub exposure: f32,
}

impl Leaf {
    /// A leaf with the default autumn (yellow) and dead (brown) colours, all healthy.
    pub fn new(reflectance: Color, transmittance: Color, refractive_index: f64, roughness: f64) -> Leaf {
        Leaf {
            reflectance, transmittance, refractive_index, roughness,
            autumn: DEFAULT_AUTUMN, autumn_sun: DEFAULT_AUTUMN, dead: DEFAULT_DEAD, states: None,
        }
    }

    /// The colour of the leaf numbered `face`.
    pub fn color(&self, face: u32) -> LeafColor {
        let green = LeafColor { reflectance: self.reflectance, transmittance: self.transmittance };
        let state = match &self.states {
            Some(states) => states.get(face as usize).copied().unwrap_or_default(),
            None => return green,
        };
        let autumn = self.autumn.mix(&self.autumn_sun, state.exposure as f64);
        let age = state.age as f64;
        if age <= 1. { green.mix(&autumn, age) } else { autumn.mix(&self.dead, age - 1.) }
    }
}

/// Yellow, from carotenoids: bright in red and green, little blue.
pub const DEFAULT_AUTUMN: LeafColor = LeafColor {
    reflectance: Color { rgb: crate::na::Vector3::new(0.45, 0.36, 0.04) },
    transmittance: Color { rgb: crate::na::Vector3::new(0.35, 0.28, 0.02) },
};

/// Brown and dry: it transmits far less than a living leaf.
pub const DEFAULT_DEAD: LeafColor = LeafColor {
    reflectance: Color { rgb: crate::na::Vector3::new(0.22, 0.14, 0.07) },
    transmittance: Color { rgb: crate::na::Vector3::new(0.06, 0.03, 0.01) },
};

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
                return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::black(), ray: None };
            }
            return ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: Color::white(), ray: Some(Ray { ro: near.point, rd }) };
        }

        let LeafColor { reflectance, transmittance } = self.color(intersection.face);

        // The far side, for light transmitted through the leaf.
        let far = Intersection { point: near.point - near.normal * THROUGH, normal: -near.normal, ..near };

        // Direct light from both sides: lights on each side are only seen from that side.
        // At the sample's wavelengths, for light transport; the choice of side below may use RGB.
        let (spectral_r, spectral_t) = (reflectance.as_reflectance(), transmittance.as_reflectance());
        let emit = direct_lighting(spectral_r, &near, s) + direct_lighting(spectral_t, &far, s);

        // Continue the path by reflecting or transmitting, in proportion to how much of each the
        // leaf does, and weight by the inverse of that probability.
        let (lr, lt) = (luminance(&reflectance), luminance(&transmittance));
        if lr + lt <= 0. {
            return ScatteredRay { light_sampling: LightSampling::Exclusive, emit, attenuate: Color::black(), ray: None };
        }
        let p_reflect = lr / (lr + lt);
        let continued = if rand() < p_reflect {
            scatter_lambertian(spectral_r * (1. / p_reflect), &near)
        } else {
            scatter_lambertian(spectral_t * (1. / (1. - p_reflect)), &far)
        };
        ScatteredRay { emit, light_sampling: LightSampling::Exclusive, ..continued }
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
        let leaf = Leaf::new(Color::new(0.1, 0.2, 0.05), Color::new(0.05, 0.15, 0.02), 1., 0.);
        let r = Ray { ro: Vector3::new(0., 5., 0.01), rd: Vector3::new(0., -1., 0.) };
        let (object, hit) = s.objects.nearest_raw_intersection(&r, f64::INFINITY, 0.).unwrap();
        let i = Intersection { dist: hit.dist, point: hit.point, normal: hit.normal, face: hit.face, part: hit.part, object };
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
    fn test_leaves_age_from_green_through_autumn_to_dead() {
        let green = Color::new(0.1, 0.2, 0.05);
        let sun = LeafColor { reflectance: Color::new(0.4, 0.05, 0.05), transmittance: Color::new(0.3, 0.02, 0.02) };
        let state = |age: f32, exposure: f32| LeafState { age, exposure };
        let mut leaf = Leaf::new(green, green, 1.5, 0.);
        leaf.autumn_sun = sun;
        leaf.states = Some(Arc::new(vec![state(0., 1.), state(1., 0.), state(1., 1.), state(2., 0.5), state(0.5, 0.)]));
        assert_eq!(leaf.color(0).reflectance, green);
        assert_eq!(leaf.color(1), DEFAULT_AUTUMN);
        assert_eq!(leaf.color(2), sun);
        assert_eq!(leaf.color(3), DEFAULT_DEAD);
        // Halfway to autumn: halfway between the colours.
        let half = (green + DEFAULT_AUTUMN.reflectance) / 2.;
        assert!((leaf.color(4).reflectance.rgb - half.rgb).norm() < 1e-12);
        // Faces without a state, and leaves without states, are healthy.
        assert_eq!(leaf.color(99).reflectance, green);
        leaf.states = None;
        assert_eq!(leaf.color(3).reflectance, green);
    }

    #[test]
    fn test_default_colours_are_physical() {
        for c in [DEFAULT_AUTUMN, DEFAULT_DEAD] {
            assert!((c.reflectance + c.transmittance).rgb.max() <= 1.);
        }
    }

    #[test]
    fn test_coat_reflects_at_grazing_angles() {
        // With a coat and nothing transmitted or reflected diffusely, light arriving at a
        // grazing angle is mostly reflected; at normal incidence, about 4%.
        let s = leaf_scene(r#"{ "type": "leaf", "reflectance": [0, 0, 0], "transmittance": [0, 0, 0], "refractive_index": 1.5 }"#, 10.);
        let object = &s.objects.items()[0];
        let leaf = Leaf::new(Color::black(), Color::black(), 1.5, 0.);
        let fraction = |rd: Vector3<f64>| {
            let i = Intersection { dist: 1., point: Vector3::zeros(), normal: Vector3::new(0., 1., 0.), face: 0, part: 0, object };
            let r = Ray { ro: -rd, rd };
            (0 .. 20000).filter(|_| leaf.scatter(&r, &i, &s).ray.is_some()).count() as f64 / 20000.
        };
        assert!((fraction(Vector3::new(0., -1., 0.)) - 0.04).abs() < 0.01);
        assert!(fraction(Vector3::new(1., -0.02, 0.).normalize()) > 0.6);
    }
}
