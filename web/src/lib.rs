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
    console_error_panic_hook::set_once();
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
    scene: Scene,
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
        Ok(Renderer { inner: Rc::new(Inner { scene, gpu: RefCell::new(gpu), passes: Cell::new(0), adapter }) })
    }

    pub fn width(&self) -> usize {
        self.inner.scene.image.width
    }

    pub fn height(&self) -> usize {
        self.inner.scene.image.height
    }

    /// The GPU's name and API.
    pub fn adapter(&self) -> String {
        self.inner.adapter.clone()
    }

    /// The samples per pixel the scene asks for.
    pub fn target(&self) -> usize {
        self.inner.scene.render.samples_per_pixel()
    }

    /// Passes (samples per pixel) queued so far.
    pub fn passes(&self) -> usize {
        self.inner.passes.get()
    }

    /// Queue the next pass: one sample of every pixel. Returns at once. Not while an `image`
    /// or `linear` is awaited (an error then): the readback holds the renderer.
    pub fn pass(&self) -> Result<(), JsValue> {
        let k = self.inner.passes.get();
        let mut gpu = self.inner.gpu.try_borrow_mut().map_err(|_| JsValue::from_str("a readback is in progress"))?;
        gpu.dispatch(&self.inner.scene, k);
        self.inner.passes.set(k + 1);
        Ok(())
    }

    /// The image so far, finished and encoded for display as the command line's PNG is: RGBA,
    /// top row first.
    pub async fn image(&self) -> Result<Vec<u8>, JsValue> {
        let inner = self.inner.clone();
        let img = inner.finished().await?;
        let opts = &inner.scene.output;
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
        let sums = self.gpu.borrow().read_back().await.map_err(|e| JsValue::from_str(&e))?;
        let (w, h) = (self.scene.image.width, self.scene.image.height);
        let mut rc = RenderContext::new(w, h, false, "web");
        let n = self.passes.get();
        for y in 0..h {
            for x in 0..w {
                let v = sums[y * w + x];
                let c = rays::color::Color::new(v[0] as f64, v[1] as f64, v[2] as f64);
                rc.set_pixel(x, y, c, rays::gpu::squares_at_least(&c, n), n);
            }
        }
        Ok(self.scene.output.finish(&rc.snapshot()))
    }
}
