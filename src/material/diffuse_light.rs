use crate::material::model::LightSampling;
use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::na::Vector3;
use crate::spectrum::Emission;

/// An emitting surface with constant radiance `emission * intensity`: a colour, or a
/// blackbody's spectrum (`spectrum::Emission`). By default it emits only
/// from the side its normal faces, as pbrt's diffuse area lights do: a quad light under a
/// ceiling shines down, not up onto the ceiling. For a mesh, the normal follows the winding of
/// its triangles.
///
/// With a `texture`, the radiance is also scaled by an image over the surface's texture
/// coordinates (pbrt's textured `DiffuseAreaLight`, section 12.5.1): a screen, a stained-glass
/// window. Area light sampling looks it up at the sampled point; a mesh doesn't say which of its
/// triangles was sampled, so there it takes the image's mean, which is only right on average:
/// texture emitters with one primitive (quads, spheres, discs) for exact results.
pub struct DiffuseLight {
    pub emission: Emission,
    pub intensity: f64,
    pub two_sided: bool,
    pub texture: Option<crate::textures::ImageMap>,
}

impl DiffuseLight {
    pub fn new(emission: Emission, intensity: f64, two_sided: bool) -> DiffuseLight {
        DiffuseLight { emission, intensity, two_sided, texture: None }
    }
}

impl MaterialModel for DiffuseLight {
    fn is_emitter(&self) -> bool { true }

    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        let mean = self.texture.as_ref().map(|t| t.image.average());
        self.emitted_with(normal, towards, mean)
    }

    fn emitted_at(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, uv: Option<crate::na::Vector2<f64>>) -> Color {
        let texel = self.texture.as_ref().map(|t| match uv {
            Some(uv) => t.at(&uv),
            None => t.image.average(),
        });
        self.emitted_with(normal, towards, texel)
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let radiance = self.emitted_at(&intersection.normal, &-r.rd, intersection.uv());
        ScatteredRay{ light_sampling: LightSampling::None, emit: Color::black(), 
            attenuate: radiance,
            ray: None }
    }
}

impl DiffuseLight {
    /// The radiance, scaled by the texture's value `texel` if there is one.
    fn emitted_with(&self, normal: &Vector3<f64>, towards: &Vector3<f64>, texel: Option<Color>) -> Color {
        if towards.dot(normal) <= 0. && !self.two_sided {
            return Color::black();
        }
        let base = self.emission.radiance() * self.intensity;
        match texel {
            None => base,
            Some(t) => base * t.as_unbounded(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use crate::sceneobject::SceneObject;
    use crate::shapes::plane::Plane;
    use crate::material::texture::Solid;
    use crate::material::legacy::FlatColor;
    use std::sync::Arc;

    #[test]
    fn test_emits_from_front_only_unless_two_sided() {
        let s = SceneFile::from_string(r#"{ "camera": { "location": [0, 1, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 } }"#.to_string());
        let i = Intersection { face: 0, part: 0,
            dist: 1.,
            point: Vector3::zeros(),
            normal: Vector3::new(0., -1., 0.),
            object: &(SceneObject {
                geometry: Box::new(Plane { y: 0. }),
                medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
            }),
        };
        let from_below = Ray { ro: Vector3::new(0., -1., 0.), rd: Vector3::new(0., 1., 0.) };
        let from_above = Ray { ro: Vector3::new(0., 1., 0.), rd: Vector3::new(0., -1., 0.) };
        let light = |two_sided| DiffuseLight::new(Emission::Rgb(Color::new(17., 12., 4.)), 1., two_sided);
        assert_eq!(light(false).scatter(&from_below, &i, &s).attenuate, Color::new(17., 12., 4.));
        assert_eq!(light(false).scatter(&from_above, &i, &s).attenuate, Color::black());
        assert_eq!(light(true).scatter(&from_above, &i, &s).attenuate, Color::new(17., 12., 4.));
    }
}
