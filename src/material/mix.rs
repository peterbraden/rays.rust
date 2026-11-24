//! A mix of two materials (Pharr, Jakob and Humphreys, "Physically Based Rendering", 4th ed.,
//! section 10.5, `MixMaterial`): at each scattering, the second with probability `amount` and
//! otherwise the first. Choosing one stochastically, rather than blending two BSDFs, is how pbrt
//! does it too: unbiased, and each material keeps its own sampling. The amount can vary over the
//! surface, from an image's grey level (`textures`): dust on a surface, wear on paint.
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::color::Color;

pub struct Mix {
    pub materials: [Box<dyn MaterialModel + Sync + Send>; 2],
    pub amount: crate::textures::ColorTexture,
}

impl Mix {
    fn amount_at(&self, i: &Intersection, s: &Scene) -> f64 {
        crate::color::luminance(&self.amount.eval(i, s)).clamp(0., 1.)
    }

    fn mean_amount(&self) -> f64 {
        crate::color::luminance(&self.amount.average()).clamp(0., 1.)
    }
}

impl MaterialModel for Mix {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let which = (crate::geometry::rand() < self.amount_at(intersection, s)) as usize;
        // The chosen material's own bump, if it has one (rust raised over pitted steel).
        match self.materials[which].shading_normal(intersection, s) {
            Some(n) if (n.dot(&r.rd) > 0.) == (intersection.normal.dot(&r.rd) > 0.) =>
                self.materials[which].scatter(r, &Intersection { normal: n, ..*intersection }, s),
            _ => self.materials[which].scatter(r, intersection, s),
        }
    }

    fn is_emitter(&self) -> bool {
        self.materials.iter().any(|m| m.is_emitter())
    }

    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        let a = self.mean_amount();
        self.materials[0].emitted(normal, towards) * (1. - a) + self.materials[1].emitted(normal, towards) * a
    }

    /// Each material's emission there, by the mean amount: so layers within keep their masks.
    fn emitted_in(&self, c: &crate::graph::Context, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> Color {
        let a = self.mean_amount();
        self.materials[0].emitted_in(c, towards, uv) * (1. - a) + self.materials[1].emitted_in(c, towards, uv) * a
    }
}

#[cfg(test)]
mod tests {
    use crate::scenefile::SceneFile;
    use crate::ray::Ray;
    use crate::na::Vector3;

    /// A quarter of the way from albedo 0.8 to black is, on average, albedo 0.6.
    #[test]
    fn test_mix_averages_its_materials() {
        let scene = |materials: &str| SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1, "shadow_bias": 0.0001,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ {} }},
            "lights": [ {{ "location": [0, 10, 0], "intensity": 314.1592653589793 }} ],
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, materials));
        let mix = scene(r#""A": { "type": "lambertian", "albedo": [0.8, 0.8, 0.8] }, "B": { "type": "lambertian", "albedo": [0, 0, 0] },
                          "M": { "type": "mix", "materials": ["A", "B"], "amount": 0.25 }"#);
        let plain = scene(r#""M": { "type": "lambertian", "albedo": [0.6, 0.6, 0.6] }"#);
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        crate::geometry::seed_rand(1, 1);
        let n = 20_000;
        let mean = (0..n).map(|_| crate::trace::trace(&r, 0, &mix).1.rgb.x).sum::<f64>() / n as f64;
        let expected = crate::trace::trace(&r, 0, &plain).1.rgb.x;
        assert!((mean / expected - 1.).abs() < 0.02, "{} vs {}", mean, expected);
    }
}
