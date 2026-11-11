use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::na::Vector3;

/// An emitting surface with constant radiance `color * intensity`. By default it emits only
/// from the side its normal faces, as pbrt's diffuse area lights do: a quad light under a
/// ceiling shines down, not up onto the ceiling. For a mesh, the normal follows the winding of
/// its triangles.
pub struct DiffuseLight {
    pub color: Color,
    pub intensity: f64,
    pub two_sided: bool,
}

impl MaterialModel for DiffuseLight {
    fn is_emitter(&self) -> bool { true }

    fn emitted(&self, normal: &Vector3<f64>, towards: &Vector3<f64>) -> Color {
        if towards.dot(normal) > 0. || self.two_sided { self.color * self.intensity } else { Color::black() }
    }

    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        let radiance = self.emitted(&intersection.normal, &-r.rd);
        ScatteredRay{ sampled_lights: false, emit: Color::black(), 
            attenuate: radiance,
            ray: None }
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
        let light = |two_sided| DiffuseLight { color: Color::new(17., 12., 4.), intensity: 1., two_sided };
        assert_eq!(light(false).scatter(&from_below, &i, &s).attenuate, Color::new(17., 12., 4.));
        assert_eq!(light(false).scatter(&from_above, &i, &s).attenuate, Color::black());
        assert_eq!(light(true).scatter(&from_above, &i, &s).attenuate, Color::new(17., 12., 4.));
    }
}
