//! Materials driven by a surface's texture coordinates (`Geometry::uv`).
use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::bsdf::LambertianBSDF;
use crate::material::functions::scatter_bsdf;
use crate::material::model::{LightSampling, MaterialModel, ScatteredRay};
use crate::na::Vector2;
use crate::ray::Ray;
use crate::scene::Scene;

/// Shows the texture coordinates as colour, (u, v) as red and green (their fractional parts),
/// unlit: for looking at a surface's parameterisation. Shapes without coordinates are magenta.
pub struct UvShade {}

impl MaterialModel for UvShade {
    fn scatter(&self, _r: &Ray, intersection: &Intersection, _s: &Scene) -> ScatteredRay {
        let c = match intersection.uv() {
            Some(uv) => Color::new(uv.x.rem_euclid(1.), uv.y.rem_euclid(1.), 0.),
            None => Color::new(1., 0., 1.),
        };
        ScatteredRay { light_sampling: LightSampling::None, emit: Color::black(), attenuate: c, ray: None }
    }
}

/// A diffuse checkerboard in texture space: `checks` squares per unit of u and of v,
/// alternating `colors`. Shapes without texture coordinates take the first colour.
pub struct UvChecker {
    pub colors: [Color; 2],
    pub checks: Vector2<f64>,
}

impl UvChecker {
    /// Which colour is at `uv`.
    pub fn at(&self, uv: &Vector2<f64>) -> Color {
        let (i, j) = ((uv.x * self.checks.x).floor() as i64, (uv.y * self.checks.y).floor() as i64);
        self.colors[(i + j).rem_euclid(2) as usize]
    }
}

impl MaterialModel for UvChecker {
    fn scatter(&self, r: &Ray, intersection: &Intersection, s: &Scene) -> ScatteredRay {
        let albedo = intersection.uv().map_or(self.colors[0], |uv| self.at(&uv));
        let intersection = &intersection.facing(r);
        scatter_bsdf(&LambertianBSDF { albedo: albedo.as_reflectance(), normal: intersection.normal }, r, &intersection.point, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_alternates() {
        let c = UvChecker { colors: [Color::black(), Color::white()], checks: Vector2::new(4., 2.) };
        assert_eq!(c.at(&Vector2::new(0.1, 0.1)), Color::black());
        assert_eq!(c.at(&Vector2::new(0.3, 0.1)), Color::white());
        assert_eq!(c.at(&Vector2::new(0.3, 0.6)), Color::black());
        assert_eq!(c.at(&Vector2::new(-0.1, 0.1)), Color::white());
    }

    use crate::na::Vector3;
    use crate::shapes::geometry::Geometry;

    fn close(a: Vector2<f64>, b: Vector2<f64>) -> bool {
        (a - b).norm() < 1e-9
    }

    #[test]
    fn test_shape_coordinates() {
        use crate::shapes::sphere::Sphere;
        let s = Sphere::new(Vector3::new(1., 2., 3.), 2.);
        // v is 1 at the top (where u is arbitrary), and u runs round from +x towards +z.
        assert!((s.uv(&Vector3::new(1., 4., 3.), 0).unwrap().y - 1.).abs() < 1e-9);
        assert!(close(s.uv(&Vector3::new(3., 2., 3.), 0).unwrap(), Vector2::new(0., 0.5)));
        assert!(close(s.uv(&Vector3::new(1., 2., 5.), 0).unwrap(), Vector2::new(0.25, 0.5)));
        // A triangle's corners at pbrt's default coordinates.
        use crate::shapes::triangle::Triangle;
        let t = Triangle::new(Vector3::zeros(), Vector3::x(), Vector3::y());
        assert!(close(t.uv(&Vector3::x(), 0).unwrap(), Vector2::new(1., 0.)));
        assert!(close(t.uv(&Vector3::y(), 0).unwrap(), Vector2::new(1., 1.)));
        assert!(close(t.uv(&Vector3::new(0.25, 0.25, 0.), 0).unwrap(), Vector2::new(0.5, 0.25)));
        // Through a transform, the item's own coordinates.
        use crate::shapes::transform::Transform;
        let moved = Transform::from_parts(Box::new(Sphere::new(Vector3::zeros(), 1.)) as Box<dyn Geometry + Sync + Send>,
            Vector3::new(10., 0., 0.), crate::na::Rotation3::identity(), Vector3::repeat(3.));
        assert!(close(moved.uv(&Vector3::new(13., 0., 0.), 0).unwrap(), Vector2::new(0., 0.5)));
    }

    #[test]
    fn test_mesh_texture_coordinates_from_obj() {
        use crate::shapes::mesh::Mesh;
        use crate::ray::Ray;
        let dir = std::env::temp_dir().join(format!("rays-uv-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("quad.obj");
        // A unit quad with coordinates stretched to [0, 2] x [0, 4].
        std::fs::write(&p, "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvt 0 0\nvt 2 0\nvt 2 4\nvt 0 4\nf 1/1 2/2 3/3\nf 1/1 3/3 4/4\n").unwrap();
        let m = Mesh::from_file(p.to_str().unwrap().to_string(), Vector3::repeat(1.));
        let hit = m.intersects(&Ray { ro: Vector3::new(0.25, 0.75, 1.), rd: Vector3::new(0., 0., -1.) }).unwrap();
        let uv = m.uv(&hit.point, hit.face).unwrap();
        assert!(close(uv, Vector2::new(0.5, 3.)), "{}", uv);
    }
}
