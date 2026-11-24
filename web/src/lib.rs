//! Rays in a browser: a scene flattened as for the wavefront integrator, rendered by the GPU
//! path tracer through WebGPU (`rays::gpu`, its WGSL shader), a pass at a time so each
//! dispatch stays within the browser's GPU watchdog, and read back asynchronously. The image is
//! finished as the command line's is (`OutputOpts::finish`, then the display encoding), so the
//! two can be compared pixel for pixel. Single-threaded: rayon runs on the calling thread.
//!
//! From JavaScript (see `www/main.js`): `provide(path, bytes)` for each file the scene refers
//! to, `await Renderer.create(json, width, height)`, then alternate `pass()` and
//! `await image()`.
use rays::gpu::GpuRenderer;
use rays::rendercontext::RenderContext;
use rays::scene::Scene;
use rays::scenefile::{Overrides, SceneFile};
use rays::wavefront::flat::FlatScene;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    remember_panics();
    // A browser drops what Rust prints: say how a scene's build goes on the console instead,
    // each stage as it starts and ends and every line meant for a person.
    rays::world::set_reporter(Some(Box::new(|e: &rays::world::Progress| {
        match &e.message {
            Some(m) => console_log(m),
            None if e.fraction <= 0. || e.fraction >= 1. => console_log(&format!("{}: {:.0}%", e.stage, e.fraction * 100.)),
            None => {},
        }
    })));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

/// Supply a file a scene refers to, under the path the scene uses (eg. "./demo/teapot.obj").
#[wasm_bindgen]
pub fn provide(path: &str, bytes: Vec<u8>) {
    rays::assets::provide(path, bytes);
}

/// A scene on the GPU and the passes rendered so far.
#[wasm_bindgen]
pub struct Renderer {
    inner: Rc<Inner>,
}

struct Inner {
    /// Behind a cell for its camera, which `set_camera` replaces; the rest is as it was built.
    scene: RefCell<Scene>,
    gpu: RefCell<GpuRenderer>,
    passes: Cell<usize>,
    adapter: String,
}

#[wasm_bindgen]
impl Renderer {
    /// Parse a scene (our JSON), flatten it and upload it; or say why it can't be rendered.
    /// A width or height of 0 keeps the scene's.
    pub async fn create(json: String, width: usize, height: usize) -> Result<Renderer, JsValue> {
        let doc: serde_json::Value = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let some = |n: usize| (n > 0).then_some(n);
        let scene = SceneFile::from_value_overridden(doc, &Overrides { width: some(width), height: some(height), seed: None });
        let flat = FlatScene::build(&scene).map_err(|e| JsValue::from_str(&format!("can't flatten the scene: {}", e)))?;
        let gpu = GpuRenderer::create(&scene, &flat).await.map_err(|e| JsValue::from_str(&e))?;
        let adapter = gpu.adapter_name.clone();
        gpu.clear();
        Ok(Renderer { inner: Rc::new(Inner { scene: RefCell::new(scene), gpu: RefCell::new(gpu), passes: Cell::new(0), adapter }) })
    }

    pub fn width(&self) -> usize {
        self.inner.scene.borrow().image.width
    }

    pub fn height(&self) -> usize {
        self.inner.scene.borrow().image.height
    }

    /// The GPU's name and API.
    pub fn adapter(&self) -> String {
        self.inner.adapter.clone()
    }

    /// The samples per pixel the scene asks for.
    pub fn target(&self) -> usize {
        self.inner.scene.borrow().render.samples_per_pixel()
    }

    /// Passes (samples per pixel) queued so far.
    pub fn passes(&self) -> usize {
        self.inner.passes.get()
    }

    /// See the same scene from another camera (the scene file's `camera` object), starting the
    /// picture again. Nothing of the scene is made again: what a scene costs to set up (its
    /// parsing and flattening, the sky's tables, the GPU's buffers and kernels) is the
    /// scene's, and the camera only says which rays a pass starts with, which the host makes
    /// for every pass anyway. (The sky's map of directions to sample was made for the first
    /// camera's place; it is only a guide to sampling, so the picture stays right.) Not
    /// during a readback, as `pass`.
    pub fn set_camera(&self, camera_json: &str) -> Result<(), JsValue> {
        let gpu = self.inner.gpu.try_borrow_mut().map_err(|_| JsValue::from_str("a readback is in progress"))?;
        let c: serde_json::Value = serde_json::from_str(camera_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut scene = self.inner.scene.borrow_mut();
        let (w, h) = (scene.image.width as u32, scene.image.height as u32);
        scene.camera = SceneFile::parse_camera(c, w, h);
        gpu.clear();
        self.inner.passes.set(0);
        Ok(())
    }

    /// Queue the next pass: one sample of every pixel. Returns at once. Not while an `image`
    /// or `linear` is awaited (an error then): the readback holds the renderer.
    pub fn pass(&self) -> Result<(), JsValue> {
        let k = self.inner.passes.get();
        let mut gpu = self.inner.gpu.try_borrow_mut().map_err(|_| JsValue::from_str("a readback is in progress"))?;
        gpu.dispatch(&self.inner.scene.borrow(), k);
        self.inner.passes.set(k + 1);
        Ok(())
    }

    /// Queue the next `most` passes together, or as many as the wavefront has lanes for
    /// (`GpuRenderer::lanes`): the same picture as that many calls of `pass`, for the
    /// dispatches of one. Returns how many it queued, at once. Not during a readback, as
    /// `pass`.
    pub fn advance(&self, most: usize) -> Result<usize, JsValue> {
        let k = self.inner.passes.get();
        let mut gpu = self.inner.gpu.try_borrow_mut().map_err(|_| JsValue::from_str("a readback is in progress"))?;
        let n = most.min(rays::scheduler::lanes(gpu.lanes)).max(1);
        let scene = self.inner.scene.borrow();
        let pixels = scene.image.width * scene.image.height;
        gpu.dispatch_block(&scene, &vec![k; pixels], &vec![n; pixels]);
        self.inner.passes.set(k + n);
        Ok(n)
    }

    /// The image so far, finished and encoded for display as the command line's PNG is: RGBA,
    /// top row first.
    pub async fn image(&self) -> Result<Vec<u8>, JsValue> {
        let inner = self.inner.clone();
        let img = inner.finished().await?;
        let scene = inner.scene.borrow();
        let opts = &scene.output;
        let mut out = Vec::with_capacity(img.width * img.height * 4);
        for c in &img.pixels {
            let [r, g, b] = opts.quantise(*c);
            out.extend_from_slice(&[r, g, b, 255]);
        }
        Ok(out)
    }

    /// The image so far, finished but linear: RGB floats, top row first (for comparisons).
    pub async fn linear(&self) -> Result<Vec<f32>, JsValue> {
        let inner = self.inner.clone();
        let img = inner.finished().await?;
        Ok(img.pixels.iter().flat_map(|c| [c.rgb.x as f32, c.rgb.y as f32, c.rgb.z as f32]).collect())
    }
}

impl Inner {
    /// The passes so far, read back and finished (`OutputOpts::finish`), top row first.
    // `pass` refuses (rather than panics) while this borrow is held across the readback.
    #[allow(clippy::await_holding_refcell_ref)]
    async fn finished(&self) -> Result<rays::output::LinearImage, JsValue> {
        let (sums, squares) = self.gpu.borrow().read_back().await.map_err(|e| JsValue::from_str(&e))?;
        let scene = self.scene.borrow();
        let (w, h) = (scene.image.width, scene.image.height);
        let mut rc = RenderContext::new(w, h, false, "web");
        let n = self.passes.get();
        for y in 0..h {
            for x in 0..w {
                let colour = |v: [f32; 4]| rays::color::Color::new(v[0] as f64, v[1] as f64, v[2] as f64);
                rc.set_pixel(x, y, colour(sums[y * w + x]), colour(squares[y * w + x]), n);
            }
        }
        Ok(scene.output.finish(&rc.snapshot()))
    }
}

// ---------------------------------------------------------------------------------------
// The CPU path tracer, for the scene builder (`web/app.js`, `web/worker.js`): the same
// renderer as the command line's, a few rows at a time (`rays::preview`). It renders every
// scene; the GPU renderer above, only those that flatten.
//
// A panic (an invalid scene, say) traps, and the module's state can no longer be trusted:
// the page reads the message with `last_error` and starts a new instance.

thread_local! {
    static LAST_ERROR: RefCell<String> = const { RefCell::new(String::new()) };
}

fn remember_panics() {
    std::panic::set_hook(Box::new(|info| {
        LAST_ERROR.with(|e| *e.borrow_mut() = info.to_string());
        console_error_panic_hook::hook(info);
    }));
}

/// The message of the panic that stopped this instance, if one has.
#[wasm_bindgen]
pub fn last_error() -> String {
    LAST_ERROR.with(|e| e.borrow().clone())
}

/// What scene files may contain, as JSON (`rays::schema`).
#[wasm_bindgen]
pub fn schema() -> String {
    rays::schema::SCHEMA.to_string()
}

/// Where the worlds of a scene's full-scale landscapes are kept (`world-<key>.json` in each
/// one's `cache`), for the page to fetch with the stages they list and `provide` before
/// building the scene: a world built on the command line is then read rather than simulated.
/// Empty for a scene that doesn't parse.
#[wasm_bindgen]
pub fn world_manifests(json: &str) -> Vec<String> {
    serde_json::from_str(json).map(rays::scenefile::SceneFile::world_manifests).unwrap_or_default()
}

/// Files of tables a scene keeps in a `cache` (its skies'), made yet or not, for the page to
/// fetch and `provide` before building the scene, as `world_manifests`'s worlds.
#[wasm_bindgen]
pub fn cached_files(json: &str) -> Vec<String> {
    serde_json::from_str(json).map(rays::scenefile::SceneFile::cached_files).unwrap_or_default()
}

/// Encode summed linear samples (three per pixel) as RGBA bytes for display. `output` is the
/// scene file's `output` object as JSON, or `null`.
#[wasm_bindgen]
pub fn encode(sums: &[f32], samples: f64, output: &str) -> Vec<u8> {
    let opts = rays::output::OutputOpts::parse(&rays::serde_json_from_str(output));
    let mut out = vec![0u8; sums.len() / 3 * 4];
    rays::preview::encode_rgba(sums, samples, &opts, &mut out);
    out
}

/// A built scene, rendered on the CPU a few rows at a time.
#[wasm_bindgen]
pub struct Preview {
    inner: rays::preview::Preview,
    rays: Cell<f64>,
}

#[wasm_bindgen]
impl Preview {
    /// Build the scene described by scene-file JSON.
    #[wasm_bindgen(constructor)]
    pub fn new(json: &str) -> Preview {
        Preview { inner: rays::preview::Preview::new(json), rays: Cell::new(0.) }
    }

    pub fn width(&self) -> usize { self.inner.width() }
    pub fn height(&self) -> usize { self.inner.height() }
    pub fn samples_per_layer(&self) -> usize { self.inner.samples_per_layer() }

    /// Replace the camera (the scene file's `camera` object, as JSON), keeping the scene.
    pub fn set_camera(&mut self, json: &str) {
        self.inner.set_camera(json);
    }

    /// Image rows `y0 .. y1` of a layer: linear sums, three per pixel. `rays_cast` then gives
    /// the rays it took.
    pub fn render_rows(&self, layer: usize, y0: usize, y1: usize) -> Vec<f32> {
        let mut out = vec![0f32; (y1 - y0) * self.inner.width() * 3];
        self.rays.set(self.inner.render_rows(layer, y0, y1, &mut out) as f64);
        out
    }

    /// The rays cast by the last `render_rows`.
    pub fn rays_cast(&self) -> f64 { self.rays.get() }

    /// Rows `y0 .. y1` of a quick image from the realtime raycaster (`rays::raycaster`),
    /// `cols` by `rows` pixels: three floats per pixel. `shading` names how it shades
    /// (`Shading::NAMES`; empty for the default).
    pub fn raycast(&self, shading: &str, cols: usize, rows: usize, y0: usize, y1: usize) -> Result<Vec<f32>, JsValue> {
        use rays::raycaster::Shading;
        let shading = if shading.is_empty() { Shading::default() } else {
            Shading::parse(shading).ok_or_else(|| JsValue::from_str(&format!("no shading \"{}\" (there are: {})", shading, Shading::NAMES.join(", "))))?
        };
        let mut out = vec![0f32; (y1 - y0) * cols * 3];
        self.inner.raycast(shading, cols, rows, y0, y1, &mut out);
        Ok(out)
    }

    /// The index in the scene file's `objects` of what is seen at pixel (x, y), or -1.
    pub fn pick(&self, x: f64, y: f64) -> i32 {
        self.inner.pick(x, y).map_or(-1, |i| i as i32)
    }
}
