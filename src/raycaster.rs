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
//! It sees the scene as it is built, and builds nothing more. Displaced terrain, which the path
//! tracer dices lazily into micropolygons as rays first reach each cell, is seen as its
//! heightmap's cells (`displaced::roughly`): dicing the ground a new view looks at took
//! seconds to minutes, which a camera being dragged can't wait for. So the raycaster shows the
//! terrain's shape at its heightmap's resolution, without the relief finer than that; and
//! `pick` sees the same surface, so a click selects what was drawn.
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
        seen_roughly(|| self.shade(r, scene))
    }

    /// `cast`, within `seen_roughly`.
    fn shade(&self, r: &Ray, scene: &Scene) -> Color {
        match self.shading {
            Shading::Headlight => headlight(r, scene, None),
        }
    }

    /// The colour of pixel (x, y), whose ray is `r`, within `seen_roughly`; `sky` the frame's
    /// lattice of the sky.
    fn pixel(&self, r: &Ray, x: usize, y: usize, sky: &mut SkyLattice, scene: &Scene) -> Color {
        match self.shading {
            Shading::Headlight => headlight(r, scene, Some((x, y, sky))),
        }
    }

    /// Render rows `y0 .. y1` of an image `cols` by `rows` pixels, three `f32`s a pixel into
    /// `out`, which holds `(y1 - y0) * cols * 3` values.
    pub fn render_rows(&self, scene: &Scene, cols: usize, rows: usize, y0: usize, y1: usize, out: &mut [f32]) {
        seen_roughly(|| {
            let mut sky = SkyLattice::new(scene, cols, rows);
            for (row, y) in (y0 .. y1.min(rows)).enumerate() {
                for x in 0 .. cols {
                    let r = Self::primary_ray(scene, cols, rows, x, y);
                    let c = self.pixel(&r, x, y, &mut sky, scene).ignore_nan();
                    let i = (row * cols + x) * 3;
                    out[i] = c.rgb.x as f32;
                    out[i + 1] = c.rgb.y as f32;
                    out[i + 2] = c.rgb.z as f32;
                }
            }
        })
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
        let hit = seen_roughly(|| scene.objects.nearest_intersection(&r, f64::INFINITY, 0.))?;
        scene.source_index(hit.object)
    }
}

/// Run `f` seeing the scene as built, without building more of it (see the module's notes).
fn seen_roughly<T>(f: impl FnOnce() -> T) -> T {
    crate::shapes::displaced::roughly(f)
}

/// Pixels apart, across and down, of the points the sky is evaluated at (`SkyLattice`).
const SKY_STEP: usize = 4;

/// The sky of one frame, evaluated only every `SKY_STEP` pixels across and down and
/// interpolated between: it changes slowly across the image, and each evaluation integrates
/// the atmosphere along the ray, which was as costly as shading the ground. The lattice's
/// points are the frame's, whatever rows are being rendered, so bands of rows rendered apart
/// meet without seams. A sun's disc is softened to a few pixels.
struct SkyLattice<'s> {
    scene: &'s Scene,
    cols: usize,
    rows: usize,
    /// By lattice point and the sky object's address: its colour, or None where it can't be
    /// found (then the pixel is evaluated itself).
    seen: std::collections::HashMap<(usize, usize, usize), Option<Color>>,
}

impl<'s> SkyLattice<'s> {
    fn new(scene: &'s Scene, cols: usize, rows: usize) -> SkyLattice<'s> {
        SkyLattice { scene, cols, rows, seen: Default::default() }
    }

    /// The sky `object` at pixel (x, y), interpolated from the lattice's points around it.
    fn at(&mut self, x: usize, y: usize, object: &crate::sceneobject::SceneObject) -> Option<Color> {
        let span = |v: usize, n: usize| {
            let v0 = v - v % SKY_STEP;
            let v1 = (v0 + SKY_STEP).min(n - 1);
            (v0, v1, if v1 > v0 { (v - v0) as f64 / (v1 - v0) as f64 } else { 0. })
        };
        let (x0, x1, fx) = span(x, self.cols);
        let (y0, y1, fy) = span(y, self.rows);
        let mut corner = |cx: usize, cy: usize| -> Option<Color> {
            let key = (cx, cy, object as *const _ as usize);
            if let Some(c) = self.seen.get(&key) { return *c; }
            let c = Self::evaluate(self.scene, self.cols, self.rows, cx, cy, object);
            self.seen.insert(key, c);
            c
        };
        let top = corner(x0, y0)? * (1. - fx) + corner(x1, y0)? * fx;
        let bottom = corner(x0, y1)? * (1. - fx) + corner(x1, y1)? * fx;
        Some(top * (1. - fy) + bottom * fy)
    }

    /// The sky `object` along the ray through pixel (x, y), whatever is in front of it there.
    fn evaluate(scene: &Scene, cols: usize, rows: usize, x: usize, y: usize, object: &crate::sceneobject::SceneObject) -> Option<Color> {
        use crate::shapes::geometry::Geometry as _;
        let r = Raycaster::primary_ray(scene, cols, rows, x, y);
        let raw = object.intersects_beyond(&r, 0.)?;
        let hit = crate::intersection::Intersection { dist: raw.dist, point: raw.point, normal: raw.normal, face: raw.face, object, part: raw.part };
        let i = object.medium.material_at(hit.point).scatter(&r, &hit, scene);
        i.ray.is_none().then(|| i.emit + i.attenuate)
    }
}

/// `Shading::Headlight`. `lattice`, the pixel and the frame's sky, to take the sky from.
fn headlight(r: &Ray, s: &Scene, lattice: Option<(usize, usize, &mut SkyLattice)>) -> Color {
    // As `trace` begins a path: nothing carried over from the last one.
    crate::volume::begin_path();
    crate::material::ggx::begin_path();
    let Some(hit) = s.objects.nearest_intersection(r, f64::INFINITY, 0f64) else {
        return s.render.background.radiance(&r.rd);
    };
    if let Some((x, y, sky)) = lattice
        && hit.object.medium.material_at(hit.point).is_sky()
        && let Some(c) = sky.at(x, y, hit.object) {
        return c;
    }
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

    /// What the raycaster costs on a scene, and what it shows: `RAYS_SCENE=<scene file>`
    /// (`RAYS_RAYCAST_OUT=<dir>` writes the frames as PNGs; `RAYS_RAYCAST_WIDTH`, default 240).
    /// Builds the scene, then raycasts on one thread, as a browser worker does, from the
    /// scene's camera and orbited about its `lookat`, timing the hits alone and the whole.
    /// `cargo test --release --lib measure_raycast -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn measure_raycast() {
        use crate::scenefile::{Overrides, SceneFile};
        let Ok(path) = std::env::var("RAYS_SCENE") else { return };
        let out = std::env::var("RAYS_RAYCAST_OUT").ok();
        let cols: usize = std::env::var("RAYS_RAYCAST_WIDTH").ok().and_then(|w| w.parse().ok()).unwrap_or(240);
        let t = web_time::Instant::now();
        let mut scene = SceneFile::from_file(&path, &Overrides::default());
        println!("{}: built in {:.1} s", path, t.elapsed().as_secs_f64());
        let doc = SceneFile::substitute_variables(SceneFile::read_document(&path).unwrap()).unwrap();
        let camera = doc["camera"].clone();
        let rows = (cols * scene.image.height / scene.image.width).max(1);
        let v = |k: &str| crate::na::Vector3::new(camera[k][0].as_f64().unwrap(), camera[k][1].as_f64().unwrap(), camera[k][2].as_f64().unwrap());
        let (at, from) = (v("lookat"), v("location"));
        let rc = Raycaster::default();
        for (i, turn) in [0f64, 0.3, -0.6, 1.5].iter().enumerate() {
            // Orbit about the vertical through `lookat`.
            let d = from - at;
            let (s, c) = turn.sin_cos();
            let moved = at + crate::na::Vector3::new(d.x * c - d.z * s, d.y, d.x * s + d.z * c);
            let mut cam = camera.clone();
            cam["location"] = serde_json::json!([moved.x, moved.y, moved.z]);
            scene.camera = SceneFile::parse_camera(cam, scene.image.width as u32, scene.image.height as u32);
            let t = web_time::Instant::now();
            let mut hits = 0;
            for y in 0 .. rows {
                for x in 0 .. cols {
                    let r = Raycaster::primary_ray(&scene, cols, rows, x, y);
                    hits += seen_roughly(|| scene.objects.nearest_intersection(&r, f64::INFINITY, 0.)).is_some() as usize;
                }
            }
            let hit_s = t.elapsed().as_secs_f64();
            if i == 0 {
                // Which objects the hits' time goes to.
                use crate::shapes::geometry::Geometry as _;
                let rays: Vec<Ray> = (0 .. rows * cols).map(|k| Raycaster::primary_ray(&scene, cols, rows, k % cols, k / cols)).collect();
                for o in scene.objects.items.iter().chain(&scene.objects.infinite_items) {
                    let t = web_time::Instant::now();
                    let n = seen_roughly(|| rays.iter().filter(|r| o.intersects_beyond(r, 0.).is_some()).count());
                    println!("    {:>24}: {:6.0} ms, {:3.0}% hit", o.geometry.kind(), t.elapsed().as_secs_f64() * 1e3, 100. * n as f64 / rays.len() as f64);
                }
                // And the shading's, by what was hit.
                let mut by: std::collections::BTreeMap<&str, (f64, usize)> = Default::default();
                for r in &rays {
                    let Some(hit) = seen_roughly(|| scene.objects.nearest_intersection(r, f64::INFINITY, 0.)) else { continue };
                    let t = web_time::Instant::now();
                    rc.cast(r, &scene);
                    let e = by.entry(hit.object.geometry.kind()).or_default();
                    e.0 += t.elapsed().as_secs_f64();
                    e.1 += 1;
                }
                for (kind, (secs, n)) in by {
                    println!("    shading {:>16}: {:6.0} ms over {} pixels ({:.1} µs each)", kind, secs * 1e3, n, secs * 1e6 / n as f64);
                }
            }
            let mut frame = vec![0f32; cols * rows * 3];
            let t = web_time::Instant::now();
            rc.render_rows(&scene, cols, rows, 0, rows, &mut frame);
            let all_s = t.elapsed().as_secs_f64();
            println!("  view {} ({:+.1} rad): {}x{}, {:.0}% hit; hits {:.0} ms, frame {:.0} ms ({:.1} µs/pixel)",
                i, turn, cols, rows, 100. * hits as f64 / (cols * rows) as f64, hit_s * 1e3, all_s * 1e3, all_s * 1e6 / (cols * rows) as f64);
            if let Some(dir) = &out {
                let mut rgba = vec![0u8; cols * rows * 4];
                crate::preview::encode_rgba(&frame, 1., &scene.output, &mut rgba);
                let name = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
                image::save_buffer(format!("{}/{}-{}.png", dir, name, i), &rgba, cols as u32, rows as u32, image::ColorType::Rgba8).unwrap();
            }
        }
    }
}
