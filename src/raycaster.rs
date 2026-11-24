//! The realtime raycaster: one ray per pixel, no bounces, quick enough to follow the camera
//! while it is being dragged in the scene builder (`web/`), and the same ray for picking what
//! is under the pointer.
//!
//! It shares the path tracer's scene, camera and materials, but answers a different
//! question: not "how much light arrives along this ray" but "what does the viewer need to see
//! to know where they are". How a hit is turned into a colour is a `Shading`, so other views
//! (normals, depth, albedo, object ids...) are each a variant and a function here: add the
//! variant, its name in `Shading::NAMES` and `Shading::parse`, and its arm in `Raycaster::cast`.
//! The page passes a shading by name (`?shading=` on its URL), through `web/raycaster.js`,
//! the worker and `Preview::raycast`.
//!
//! Every pixel draws the same random numbers every time (`primary_ray` seeds them by its
//! index), so a still camera gives a still image and the same click picks the same thing,
//! whatever the lens.
use crate::camera::Camera;
use crate::color::Color;
use crate::geometry::seed_rand;
use crate::ray::Ray;
use crate::scene::Scene;

/// How the raycaster turns what a camera ray hits into a colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shading {
    /// The surface's own answer to the ray (emission, the direct light it sampled, and
    /// anything that ends a path, such as the sky), with the light a bounce would have gathered
    /// replaced by a headlight: brighter where the surface faces the viewer. Linear radiance,
    /// encoded with the scene's `output` like the path tracer's picture.
    #[default]
    Headlight,
}

impl Shading {
    /// Every shading's name, as `parse` takes it.
    pub const NAMES: &'static [&'static str] = &["headlight"];

    /// The shading of this name, if there is one.
    pub fn parse(name: &str) -> Option<Shading> {
        match name {
            "headlight" => Some(Shading::Headlight),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Shading::Headlight => "headlight",
        }
    }
}

/// A raycaster: how it shades. It holds no scene, so one serves any scene and camera.
#[derive(Debug, Clone, Copy, Default)]
pub struct Raycaster {
    pub shading: Shading,
}

impl Raycaster {
    pub fn new(shading: Shading) -> Raycaster {
        Raycaster { shading }
    }

    /// The scene's camera ray through pixel (x, y) of an image `cols` by `rows` pixels, row 0
    /// at the top, whatever the scene's own size. Seeds the random numbers by the pixel, so it
    /// is the same ray every time.
    pub fn primary_ray(scene: &Scene, cols: usize, rows: usize, x: usize, y: usize) -> Ray {
        let (w, h) = (cols as f64, rows as f64);
        seed_rand(0, (y * cols + x) as u64);
        // The camera's rows run from the bottom; images' from the top.
        scene.camera.get_ray(x as f64 / w, (rows - 1 - y) as f64 / h, 0.5 / w, 0.5 / h)
    }

    /// The colour seen along one ray.
    pub fn cast(&self, r: &Ray, scene: &Scene) -> Color {
        match self.shading {
            Shading::Headlight => headlight(r, scene),
        }
    }

    /// Render rows `y0 .. y1` of an image `cols` by `rows` pixels, three `f32`s a pixel into
    /// `out`, which holds `(y1 - y0) * cols * 3` values.
    pub fn render_rows(&self, scene: &Scene, cols: usize, rows: usize, y0: usize, y1: usize, out: &mut [f32]) {
        for (row, y) in (y0 .. y1.min(rows)).enumerate() {
            for x in 0 .. cols {
                let r = Self::primary_ray(scene, cols, rows, x, y);
                let c = self.cast(&r, scene).ignore_nan();
                let i = (row * cols + x) * 3;
                out[i] = c.rgb.x as f32;
                out[i + 1] = c.rgb.y as f32;
                out[i + 2] = c.rgb.z as f32;
            }
        }
    }

    /// The object seen at pixel (x, y) of the scene's own image, from the top left: its index
    /// in the scene file's `objects` list. None for the background, or an object with no entry
    /// of its own.
    pub fn pick(scene: &Scene, x: usize, y: usize) -> Option<usize> {
        let (w, h) = (scene.image.width, scene.image.height);
        if x >= w || y >= h {
            return None;
        }
        let r = Self::primary_ray(scene, w, h, x, y);
        let hit = scene.objects.nearest_intersection(&r, f64::INFINITY, 0.)?;
        scene.source_index(hit.object)
    }
}

/// `Shading::Headlight`.
fn headlight(r: &Ray, s: &Scene) -> Color {
    // As `trace` begins a path: nothing carried over from the last one.
    crate::volume::begin_path();
    crate::material::ggx::begin_path();
    let Some(hit) = s.objects.nearest_intersection(r, f64::INFINITY, 0f64) else {
        return s.render.background.radiance(&r.rd);
    };
    let facing = hit.facing_normal(r);
    let mut biased = hit;
    biased.point = hit.point + facing * s.render.shadow_bias;
    let medium = &hit.object.medium;
    let medium = medium.part(hit.part).unwrap_or(&**medium);
    let shaded = crate::trace::shading(r, medium, &biased, s);
    let interaction = medium.material_at(biased.point).scatter(r, &shaded, s);
    match interaction.ray {
        None => interaction.emit + interaction.attenuate,
        Some(_) => {
            let towards_viewer = shaded.normal.dot(&r.rd.normalize()).abs();
            interaction.emit + interaction.attenuate * (0.15 + 0.35 * towards_viewer)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preview::tests::{turn, SCENE};
    use crate::preview::Preview;

    #[test]
    fn test_every_shading_is_named() {
        for name in Shading::NAMES {
            assert_eq!(Shading::parse(name).map(Shading::name), Some(*name));
        }
        assert_eq!(Shading::parse(Shading::default().name()), Some(Shading::default()));
        assert_eq!(Shading::parse("nonsense"), None);
    }

    /// Rows rendered in bands are the image rendered at once, and render again the same.
    #[test]
    fn test_raycast_shows_the_scene_at_any_size() {
        let _turn = turn();
        let p = Preview::new(SCENE);
        let rc = Raycaster::default();
        let (cols, rows) = (6, 4);
        let mut out = vec![0f32; cols * rows * 3];
        rc.render_rows(&p.scene, cols, rows, 0, 2, &mut out[.. cols * 2 * 3]);
        rc.render_rows(&p.scene, cols, rows, 2, 4, &mut out[cols * 2 * 3 ..]);
        let mut again = vec![0f32; cols * rows * 3];
        rc.render_rows(&p.scene, cols, rows, 0, rows, &mut again);
        assert_eq!(out, again);
        let px = |x: usize, y: usize| &out[(y * cols + x) * 3 .. (y * cols + x) * 3 + 3];
        // Sky at the top, lit ground at the bottom, and they differ.
        assert!(px(0, 0).iter().all(|v| *v > 0.) && px(0, 3).iter().all(|v| *v > 0.));
        assert_ne!(px(0, 0), px(0, 3));
    }

    #[test]
    fn test_pick_is_off_the_image_for_nothing() {
        let _turn = turn();
        let p = Preview::new(SCENE);
        assert_eq!(Raycaster::pick(&p.scene, 6, 3), Some(2));
        assert_eq!(Raycaster::pick(&p.scene, p.width(), 0), None);
    }
}
