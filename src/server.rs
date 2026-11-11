//! `rays serve`: the renderer as a small HTTP server, so that a scene builder in a browser can
//! hand it layers to render, and so that one command runs the builder itself.
//!
//! A page renders a preview a layer at a time (see `preview.rs`), and layers are independent:
//! each (layer, pixel) has its own random stream. So work divides by layer between whoever is
//! there to do it. The page's own workers take some layer numbers and each server it knows of
//! takes others; a server renders a layer on every core and sends back the same sums a
//! worker would have. With no server, the page carries on alone.
//!
//! The interface, all under `/api`:
//!
//! - `GET /api/health`: `{"name": "rays", "version", "threads"}`. How a page finds out that a
//!   server is there.
//! - `POST /api/scene`, the scene file's JSON as the body: builds it and answers
//!   `{"id", "width", "height"}`, or status 400 and `{"error"}` with the parser's message.
//!   Files the scene names (meshes, images) are read from the server's own disk, relative to
//!   the directory it was started in.
//! - `POST /api/render`, `{"scene": id, "layer": n, "camera": {...} or null, "client": "...",
//!   "gen": n}`: the layer, as little-endian `f32` sums, three per pixel, rows from the top.
//!   `camera` replaces the scene's camera first, as a page does when the view is moved. Status
//!   404 means the server no longer has that scene (it keeps the last few): post it again.
//!   `gen` is the client's count of what it is showing; once a client has asked with a later
//!   `gen`, its earlier requests are abandoned, mid-layer if need be, with status 409.
//!
//! Anything else is a file under the directory the server was started in, so
//! `http://localhost:8787/web/` is the scene builder, with the demo scenes and their meshes.
//!
//! Scenes live on one thread, which does all building and rendering, one job at a time (a
//! render uses every core through rayon anyway); the threads that talk HTTP send it jobs.
//!
//! Who may use it: by default it listens on this machine only, and a web page may call it
//! only if the page is itself served from this machine, or its origin is named with
//! `--allow-origin`. It has no authentication: bind it to a network address only on a
//! network you trust, as anyone who can reach it can have it render, and read files it can
//! read as scene assets.
use crate::preview::Preview;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Request, Response, Server};

pub struct Options {
    /// The address to listen on: `127.0.0.1` for this machine only.
    pub bind: String,
    pub port: u16,
    /// The directory files are served from.
    pub root: PathBuf,
    /// Origins of pages allowed to call the API, besides pages on this machine.
    pub allow_origins: Vec<String>,
}

/// How many built scenes are kept. A page shows one at a time; a few lets it go back and
/// forth, and lets a second page share the server.
const SCENES_KEPT: usize = 3;

enum Job {
    Build { json: String, reply: Sender<Result<Value, String>> },
    Render { scene: u64, layer: usize, camera: Option<String>, client: String, generation: u64, reply: Sender<Result<Vec<u8>, u16>> },
}

/// Each client's latest generation: requests for an earlier one are stale.
type Latest = Arc<Mutex<HashMap<String, Arc<AtomicU64>>>>;

fn latest_for(latest: &Latest, client: &str) -> Arc<AtomicU64> {
    latest.lock().unwrap().entry(client.to_string()).or_default().clone()
}

/// The thread that owns the scenes.
fn render_thread(jobs: std::sync::mpsc::Receiver<Job>, latest: Latest) {
    let mut scenes: Vec<(u64, Preview)> = Vec::new();
    for job in jobs {
        match job {
            Job::Build { json, reply } => {
                let id = fingerprint(&json);
                if !scenes.iter().any(|(i, _)| *i == id) {
                    // The parser panics on an invalid scene; its message is the answer.
                    match std::panic::catch_unwind(|| Preview::new(&json)) {
                        Ok(p) => {
                            scenes.push((id, p));
                            if scenes.len() > SCENES_KEPT { scenes.remove(0); }
                        },
                        Err(e) => {
                            let _ = reply.send(Err(panic_message(e)));
                            continue;
                        },
                    }
                }
                let p = &scenes.iter().find(|(i, _)| *i == id).unwrap().1;
                let _ = reply.send(Ok(json!({ "id": format!("{:016x}", id), "width": p.width(), "height": p.height() })));
            },
            Job::Render { scene, layer, camera, client, generation, reply } => {
                let current = latest_for(&latest, &client);
                let wanted = || current.load(Ordering::Relaxed) <= generation;
                let Some((_, p)) = scenes.iter_mut().find(|(i, _)| *i == scene) else {
                    let _ = reply.send(Err(404));
                    continue;
                };
                if !wanted() {
                    let _ = reply.send(Err(409));
                    continue;
                }
                let rendered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    if let Some(c) = &camera { p.set_camera(c); }
                    p.render_layer(layer, &wanted)
                }));
                let _ = reply.send(match rendered {
                    Ok(Some(sums)) => Ok(sums.iter().flat_map(|v| v.to_le_bytes()).collect()),
                    Ok(None) => Err(409),
                    Err(_) => Err(400),
                });
            },
        }
    }
}

fn fingerprint(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut h);
    h.finish()
}

fn panic_message(e: Box<dyn std::any::Any + Send>) -> String {
    e.downcast_ref::<String>().cloned()
        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "the scene could not be built".to_string())
}

/// Whether a page from `origin` may call the API: pages on this machine, and those allowed
/// by name.
fn origin_allowed(origin: &str, allowed: &[String]) -> bool {
    let host = origin.split("://").nth(1).unwrap_or("");
    let host = if host.starts_with('[') {
        host.split(']').next().unwrap_or("").trim_start_matches('[')
    } else {
        host.split(':').next().unwrap_or("")
    };
    matches!(host, "localhost" | "127.0.0.1" | "::1") || allowed.iter().any(|a| a.trim_end_matches('/') == origin)
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).unwrap()
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json",
        "wasm" => "application/wasm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "md" | "txt" | "obj" | "nff" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// The file a URL path names under `root`, if it stays inside it.
fn file_for(root: &Path, url_path: &str) -> Option<PathBuf> {
    let mut path = root.to_path_buf();
    for part in url_path.split('/').filter(|p| !p.is_empty()) {
        if part == ".." || part.starts_with('.') || part.contains('\\') { return None; }
        path.push(part);
    }
    if path.is_dir() { path.push("index.html"); }
    path.is_file().then_some(path)
}

struct Handler {
    jobs: Mutex<Sender<Job>>,
    latest: Latest,
    options: Options,
}

impl Handler {
    fn respond(&self, request: Request, status: u16, content: &str, body: Vec<u8>) {
        let mut response = Response::from_data(body).with_status_code(status).with_header(header("Content-Type", content));
        let origin = request.headers().iter().find(|h| h.field.equiv("Origin")).map(|h| h.value.to_string());
        if let Some(origin) = origin.filter(|o| origin_allowed(o, &self.options.allow_origins)) {
            for (k, v) in [
                ("Access-Control-Allow-Origin", origin.as_str()), ("Vary", "Origin"),
                ("Access-Control-Allow-Methods", "GET, POST, OPTIONS"), ("Access-Control-Allow-Headers", "Content-Type"),
                // Browsers ask this before letting a public page call a private address.
                ("Access-Control-Allow-Private-Network", "true"), ("Access-Control-Max-Age", "600"),
            ] {
                response.add_header(header(k, v));
            }
        }
        let _ = request.respond(response);
    }

    fn json(&self, request: Request, status: u16, value: Value) {
        self.respond(request, status, "application/json", value.to_string().into_bytes());
    }

    fn send(&self, job: Job) {
        self.jobs.lock().unwrap().send(job).expect("the render thread has stopped");
    }

    fn handle(&self, mut request: Request) {
        let url = request.url().split('?').next().unwrap_or("").to_string();
        let mut body = String::new();
        if *request.method() == Method::Post && request.as_reader().read_to_string(&mut body).is_err() {
            return self.json(request, 400, json!({ "error": "the request body is not text" }));
        }
        match (request.method().clone(), url.as_str()) {
            (Method::Options, _) => self.respond(request, 204, "text/plain", Vec::new()),
            (Method::Get, "/api/health") => self.json(request, 200, json!({
                "name": "rays", "version": env!("CARGO_PKG_VERSION"), "threads": rayon::current_num_threads(),
            })),
            (Method::Post, "/api/scene") => {
                let (reply, answer) = channel();
                self.send(Job::Build { json: body, reply });
                match answer.recv() {
                    Ok(Ok(v)) => self.json(request, 200, v),
                    Ok(Err(message)) => self.json(request, 400, json!({ "error": message })),
                    Err(_) => self.json(request, 500, json!({ "error": "the render thread has stopped" })),
                }
            },
            (Method::Post, "/api/render") => {
                let Ok(q) = serde_json::from_str::<Value>(&body) else {
                    return self.json(request, 400, json!({ "error": "the request is not JSON" }));
                };
                let (Some(scene), Some(layer)) = (q["scene"].as_str().and_then(|s| u64::from_str_radix(s, 16).ok()), q["layer"].as_u64()) else {
                    return self.json(request, 400, json!({ "error": "a render needs \"scene\" and \"layer\"" }));
                };
                let client = q["client"].as_str().unwrap_or("").to_string();
                let generation = q["gen"].as_u64().unwrap_or(0);
                // Seen here, not on the render thread, so that it cuts short whatever that
                // thread is doing for this client now.
                latest_for(&self.latest, &client).fetch_max(generation, Ordering::Relaxed);
                let camera = (!q["camera"].is_null()).then(|| q["camera"].to_string());
                let (reply, answer) = channel();
                self.send(Job::Render { scene, layer: layer as usize, camera, client, generation, reply });
                match answer.recv() {
                    Ok(Ok(bytes)) => self.respond(request, 200, "application/octet-stream", bytes),
                    Ok(Err(404)) => self.json(request, 404, json!({ "error": "no such scene: post it again" })),
                    Ok(Err(409)) => self.json(request, 409, json!({ "error": "superseded by a later request" })),
                    Ok(Err(_)) => self.json(request, 400, json!({ "error": "the render failed" })),
                    Err(_) => self.json(request, 500, json!({ "error": "the render thread has stopped" })),
                }
            },
            (Method::Get, "/") => {
                let response = Response::from_data(Vec::new()).with_status_code(302).with_header(header("Location", "/web/"));
                let _ = request.respond(response);
            },
            (Method::Get, path) if !path.starts_with("/api/") => match file_for(&self.options.root, path).and_then(|p| std::fs::read(&p).ok().map(|b| (p, b))) {
                Some((p, bytes)) => self.respond(request, 200, content_type(&p), bytes),
                None => self.json(request, 404, json!({ "error": "not found" })),
            },
            _ => self.json(request, 404, json!({ "error": "not found" })),
        }
    }
}

/// Serve until the process is stopped.
pub fn serve(options: Options) -> Result<(), String> {
    let address = format!("{}:{}", options.bind, options.port);
    let server = Arc::new(Server::http(&address).map_err(|e| format!("Can't listen on {}: {}", address, e))?);
    let latest: Latest = Arc::default();
    let (jobs, queue) = channel();
    {
        let latest = latest.clone();
        std::thread::spawn(move || render_thread(queue, latest));
    }
    println!("Rays is serving on http://{}/ ({} render threads)", address, rayon::current_num_threads());
    println!("- Scene builder: http://{}/web/", address);
    println!("- Files from: {}", options.root.display());
    let handler = Arc::new(Handler { jobs: Mutex::new(jobs), latest, options });
    // A few threads to talk HTTP: requests wait on the one render thread, so more than a
    // handful only helps to keep serving files while a render is under way.
    let threads: Vec<_> = (0 .. 6).map(|_| {
        let (server, handler) = (server.clone(), handler.clone());
        std::thread::spawn(move || {
            while let Ok(request) = server.recv() {
                handler.handle(request);
            }
        })
    }).collect();
    for t in threads { let _ = t.join(); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_local_and_named_origins_are_allowed() {
        let named = vec!["https://example.github.io".to_string()];
        for ok in ["http://localhost:8000", "http://127.0.0.1:8787", "http://[::1]:8000", "https://example.github.io"] {
            assert!(origin_allowed(ok, &named), "{}", ok);
        }
        for bad in ["https://evil.example", "http://localhost.evil.example", "https://example.github.io.evil.example", "null"] {
            assert!(!origin_allowed(bad, &named), "{}", bad);
        }
    }

    #[test]
    fn test_files_stay_under_the_root() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(file_for(root, "/web/index.html").is_some());
        assert!(file_for(root, "/web/").is_some_and(|p| p.ends_with("web/index.html")));
        for bad in ["/../etc/passwd", "/web/../../etc/passwd", "/.git/config", "/web/nope.html"] {
            assert!(file_for(root, bad).is_none(), "{}", bad);
        }
    }
}
