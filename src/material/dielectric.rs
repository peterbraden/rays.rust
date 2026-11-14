use crate::color::Color;
use crate::scene::Scene;
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::material::functions::scatter_dielectric;
use crate::spectrum::RefractiveIndex;

/// A smooth dielectric (glass, water). Its refractive index may vary with wavelength
/// (`spectrum::RefractiveIndex`): then, in spectral mode, a path through it follows its hero
/// wavelength alone, and white light disperses into colours.
pub struct Dielectric {
    pub refractive_index: RefractiveIndex,
    pub attenuate: Color,
}

impl MaterialModel for Dielectric {
    fn scatter(&self, r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay{
        scatter_dielectric(self.refractive_index.current(), self.attenuate.as_reflectance(), r, intersection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use crate::sceneobject::SceneObject;
    use crate::shapes::sphere::Sphere;
    use crate::material::texture::Solid;
    use crate::material::legacy::FlatColor;
    use crate::spectrum::{begin_sample, end_sample, Wavelengths};
    use crate::na::Vector3;

    /// Through a dispersive glass, blue light bends more than red, and the path follows only
    /// its hero wavelength.
    #[test]
    fn test_dispersion_separates_wavelengths() {
        let s = SceneFile::from_string(r#"{ "camera": { "location": [0, 1, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "angle": 0.5 } }"#.to_string());
        let object = SceneObject {
            geometry: Box::new(Sphere::new(Vector3::new(0., 0., 0.), 1.)),
            medium: Box::new(Solid { m: Box::new(FlatColor { pigment: Color::white() }) }),
        };
        let i = Intersection { face: 0, part: 0, dist: 1., point: Vector3::new(0., 1., 0.), normal: Vector3::new(0., 1., 0.), object: &object };
        let glass = Dielectric { refractive_index: RefractiveIndex::glass("SF11").unwrap(), attenuate: Color::white() };
        let r = Ray { ro: Vector3::new(-1., 2., 0.), rd: Vector3::new(1., -1., 0.).normalize() };
        // The refracted direction's angle from the normal, at a wavelength; the reflected ray
        // (chosen by Fresnel) is skipped.
        let bend = |u: f64, k: u64| -> Option<f64> {
            let w = Wavelengths::sample_visible(u);
            begin_sample(w);
            crate::geometry::seed_rand(1, k);
            let out = glass.scatter(&r, &i, &s);
            let w = end_sample().unwrap();
            assert!(w.secondary_terminated);
            let rd = out.ray.unwrap().rd.normalize();
            if rd.y > 0. { None } else { Some((rd.x).asin()) }
        };
        let blue = (0..50).find_map(|k| bend(0.05, k)).unwrap();
        let red = (0..50).find_map(|k| bend(0.95, k)).unwrap();
        assert!(blue < red, "blue {} red {}", blue, red);
    }
}
