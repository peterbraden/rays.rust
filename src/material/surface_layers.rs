//! # Surface layers
//!
//! Ground as a stack of materials, each lying over those below it where its mask says, after
//! Terragen's Surface Layers (Planetside, "Surface Layer", "Texturing with Surface Layers"):
//! bare rock at the bottom, then scree where sediment lies on a slope, soil and grass where it
//! is gentle, wet ground where water gathers, snow where it is high and level enough to hold it.
//! Real ground reads as photographic because of that variety, not because of any one material.
//!
//! ```json
//! "Ground": { "type": "surface_layers", "layers": [
//!     { "material": "Rock" },
//!     { "material": "Scree", "mask": { "node": "field", "field": "sediment", "from": 0.2, "to": 1 } },
//!     { "material": "Snow", "mask": { "node": "multiply", "inputs": [
//!         { "node": "field", "field": "snowline", "from": -100, "to": 100 },
//!         { "node": "slope", "from": 40, "to": 30 } ] }, "coverage": 0.9 }
//! ] }
//! ```
//!
//! Layers are listed from the bottom up. The first is the base, under everything; each other
//! layer covers what is below it by its `mask`, a function graph (`graph.rs`: slope, altitude,
//! aspect, noise to break up the edges, and on a landscape the simulation's fields: sediment,
//! ice, water) in [0, 1], times its `coverage` (default 1). As `mix` does, at each scattering
//! one layer is chosen with the probability that it is the one on top (from the top down: each
//! with its mask's probability, else the next one down), which is unbiased and lets each keep
//! its own BSDF and sampling. Only the masks above the chosen layer are evaluated.

use crate::color::Color;
use crate::graph::{Context, Graph};
use crate::intersection::Intersection;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::na::Vector3;
use crate::ray::Ray;
use crate::scene::Scene;

pub struct SurfaceLayer {
    pub material: Box<dyn MaterialModel + Sync + Send>,
    /// None for everywhere.
    pub mask: Option<Graph>,
    pub coverage: f64,
}

pub struct SurfaceLayers {
    /// From the bottom up; the first is the base, under everything.
    pub layers: Vec<SurfaceLayer>,
}

impl SurfaceLayer {
    /// How much of the surface at the hit the layer covers, in [0, 1].
    fn cover(&self, c: &Context) -> f64 {
        let m = self.mask.as_ref().map_or(1., |g| g.eval(c));
        (m * self.coverage).clamp(0., 1.)
    }
}

impl SurfaceLayers {
    /// The layer on top at the hit, chosen stochastically by the layers' covers.
    fn layer_at(&self, i: &Intersection) -> &SurfaceLayer {
        let c = Context::at_hit(i);
        for layer in self.layers[1..].iter().rev() {
            let m = layer.cover(&c);
            if m >= 1. || (m > 0. && crate::geometry::rand() < m) {
                return layer;
            }
        }
        &self.layers[0]
    }
}

impl MaterialModel for SurfaceLayers {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        self.layer_at(intersection).material.scatter(r, intersection, s)
    }

    fn is_emitter(&self) -> bool {
        self.layers.iter().any(|l| l.material.is_emitter())
    }

    /// Without a point there are no masks: the most any layer emits, so a light whose top
    /// layer alone glows is still found and sampled (`emitted_in` gives the true amount).
    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        let total = |c: &Color| c.rgb.x + c.rgb.y + c.rgb.z;
        self.layers.iter().map(|l| l.material.emitted(normal, towards))
            .fold(Color::black(), |a, b| if total(&b) > total(&a) { b } else { a })
    }

    /// Each layer's emission by how much of the surface it is on top of there: what scattering
    /// at the point gives on average, as it picks the layer on top.
    fn emitted_in(&self, c: &Context, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> Color {
        if !self.is_emitter() {
            return Color::black();
        }
        let mut sum = Color::black();
        let mut below = 1.;
        for layer in self.layers[1..].iter().rev() {
            let m = layer.cover(c);
            if m > 0. && layer.material.is_emitter() {
                sum = sum + layer.material.emitted_in(c, towards, uv) * (below * m);
            }
            below *= 1. - m;
            if below <= 0. {
                return sum;
            }
        }
        sum + self.layers[0].material.emitted_in(c, towards, uv) * below
    }
}

#[cfg(test)]
mod tests {
    use crate::na::Vector3;
    use crate::ray::Ray;
    use crate::scenefile::SceneFile;

    fn scene(materials: &str) -> crate::scene::Scene {
        SceneFile::from_string(format!(r#"{{
            "background": [0, 0, 0], "max_depth": 1, "shadow_bias": 0.0001,
            "camera": {{ "location": [0, 1, -1], "lookat": [0, 0, 0], "up": [0, 1, 0], "angle": 0.5 }},
            "materials": {{ "A": {{ "type": "lambertian", "albedo": [0.8, 0.8, 0.8] }},
                            "B": {{ "type": "lambertian", "albedo": [0.2, 0.2, 0.2] }},
                            "C": {{ "type": "lambertian", "albedo": [0, 0, 0] }}, {} }},
            "lights": [ {{ "location": [0, 10, 0], "intensity": 314.1592653589793 }} ],
            "objects": [ {{ "type": "plane", "y": 0, "material": "M" }} ]
        }}"#, materials))
    }

    fn mean(s: &crate::scene::Scene) -> f64 {
        let r = Ray { ro: Vector3::new(0., 1., -0.01), rd: Vector3::new(0., -1., 0.01).normalize() };
        crate::geometry::seed_rand(1, 1);
        let n = 20_000;
        (0..n).map(|_| crate::trace::trace(&r, 0, s).1.rgb.x).sum::<f64>() / n as f64
    }

    /// Half of B over A, and a quarter of C over that: 3/8 A, 3/8 B, 1/4 C, so an albedo of
    /// 0.375 * 0.8 + 0.375 * 0.2 = 0.375.
    #[test]
    fn test_layers_cover_those_below() {
        let layered = scene(r#""M": { "type": "surface_layers", "layers": [
            { "material": "A" }, { "material": "B", "mask": 0.5 }, { "material": "C", "mask": 0.5, "coverage": 0.5 } ] }"#);
        let plain = scene(r#""M": { "type": "lambertian", "albedo": [0.375, 0.375, 0.375] }"#);
        let (a, b) = (mean(&layered), mean(&plain));
        assert!((a / b - 1.).abs() < 0.02, "{} vs {}", a, b);
    }

    /// A mask by slope: level ground is all the top layer, which a mask of slope above 45° leaves
    /// bare.
    #[test]
    fn test_masks_by_slope() {
        let steep = scene(r#""M": { "type": "surface_layers", "layers": [
            { "material": "A" }, { "material": "C", "mask": { "node": "slope", "from": 40, "to": 50 } } ] }"#);
        let level = scene(r#""M": { "type": "surface_layers", "layers": [
            { "material": "A" }, { "material": "C", "mask": { "node": "slope", "from": 50, "to": 40 } } ] }"#);
        assert!(mean(&steep) > 0.);
        assert_eq!(mean(&level), 0.);
    }
}
