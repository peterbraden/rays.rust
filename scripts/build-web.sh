#!/bin/sh
# Build the browser version (web/): the renderer's library for wasm32 with the GPU path
# tracer, bound for JavaScript by wasm-bindgen (`cargo install wasm-bindgen-cli` at the version
# in web/Cargo.lock), into web/www/pkg. Then serve the repository's root, eg.
#   python3 -m http.server 8000
# and open http://localhost:8000/web/www/?scene=demo/benchmarks/spheres.json in a browser with
# WebGPU (Chrome, Edge; Safari 26; Firefox 141 on Windows).
set -e
cd "$(dirname "$0")/../web"
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir www/pkg target/wasm32-unknown-unknown/release/rays_web.wasm
ls -l www/pkg/rays_web_bg.wasm
