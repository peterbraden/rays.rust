//! The renderer as a WebAssembly module, for `web/`.
//!
//! The interface is plain C-style exports over the module's memory, with no binding generator:
//! the page copies bytes in (`alloc`, then write), calls a function, and reads bytes out. That
//! keeps the build to `cargo build --target wasm32-unknown-unknown` and nothing else.
//!
//! A module instance is used by one thread (a web worker, or the page) and holds at most one
//! scene. A panic (eg. an invalid scene) reports its message through the imported `log` and
//! then traps; the instance must be thrown away, as its state is no longer trustworthy. The
//! page starts a fresh one.
use rays::output::OutputOpts;
use rays::preview::{Preview, encode_rgba};
use std::cell::RefCell;

unsafe extern "C" {
    /// Show a message. `level` is 0 for information, 1 for an error.
    fn log(ptr: *const u8, len: usize, level: u32);
}

thread_local! {
    static PREVIEW: RefCell<Option<Preview>> = const { RefCell::new(None) };
}

fn with_preview<T>(f: impl FnOnce(&mut Preview) -> T) -> T {
    PREVIEW.with(|p| f(p.borrow_mut().as_mut().expect("no scene loaded")))
}

unsafe fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

unsafe fn text<'a>(ptr: *const u8, len: usize) -> &'a str {
    std::str::from_utf8(unsafe { bytes(ptr, len) }).expect("text is not UTF-8")
}

/// Call once, before anything else: sends panic messages to the page.
#[unsafe(no_mangle)]
pub extern "C" fn init() {
    std::panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        unsafe { log(msg.as_ptr(), msg.len(), 1) };
    }));
}

/// Reserve `len` bytes for the page to write into.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// Release memory from `alloc`.
///
/// # Safety
/// `ptr` and `len` must be those of one earlier `alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    drop(unsafe { Vec::from_raw_parts(ptr, 0, len.max(1)) });
}

/// Provide the contents of a file the scene refers to (a mesh, a heightmap image).
///
/// # Safety
/// Both ranges must be valid memory from `alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn asset_add(path: *const u8, path_len: usize, data: *const u8, data_len: usize) {
    rays::assets::register(unsafe { text(path, path_len) }, unsafe { bytes(data, data_len) }.to_vec());
}

/// Build the scene described by scene-file JSON, replacing any earlier one.
///
/// # Safety
/// The range must be valid memory from `alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn scene_load(json: *const u8, len: usize) {
    let p = Preview::new(unsafe { text(json, len) });
    PREVIEW.with(|slot| *slot.borrow_mut() = Some(p));
}

/// Replace the loaded scene's camera (the scene file's `camera` object, as JSON).
///
/// # Safety
/// The range must be valid memory from `alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn scene_set_camera(json: *const u8, len: usize) {
    with_preview(|p| p.set_camera(unsafe { text(json, len) }));
}

#[unsafe(no_mangle)]
pub extern "C" fn scene_width() -> usize { with_preview(|p| p.width()) }

#[unsafe(no_mangle)]
pub extern "C" fn scene_height() -> usize { with_preview(|p| p.height()) }

#[unsafe(no_mangle)]
pub extern "C" fn scene_samples_per_layer() -> usize { with_preview(|p| p.samples_per_layer()) }

/// Render image rows `y0 .. y1` of a layer into `out`, as `f32` linear sums, three per pixel.
/// Returns the number of rays cast.
///
/// # Safety
/// `out` must be valid, 4-byte aligned memory for `(y1 - y0) * width * 3` `f32`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn render_rows(layer: usize, y0: usize, y1: usize, out: *mut f32) -> f64 {
    with_preview(|p| {
        let out = unsafe { std::slice::from_raw_parts_mut(out, (y1 - y0) * p.width() * 3) };
        p.render_rows(layer, y0, y1, out) as f64
    })
}

/// The index in the scene file's `objects` of what is seen at pixel (x, y), or -1.
#[unsafe(no_mangle)]
pub extern "C" fn pick(x: f64, y: f64) -> i32 {
    with_preview(|p| p.pick(x, y).map_or(-1, |i| i as i32))
}

/// Encode summed samples for display: `pixels * 3` `f32` sums in, `pixels * 4` RGBA bytes out.
/// `output` is the scene file's `output` object as JSON (or `null`). Needs no scene loaded.
///
/// # Safety
/// The ranges must be valid memory from `alloc`, `sums` 4-byte aligned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode(sums: *const f32, pixels: usize, samples: f64, output: *const u8, output_len: usize, out: *mut u8) {
    let opts = OutputOpts::parse(&rays::serde_json_from_str(unsafe { text(output, output_len) }));
    let sums = unsafe { std::slice::from_raw_parts(sums, pixels * 3) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, pixels * 4) };
    encode_rgba(sums, samples, &opts, out);
}
