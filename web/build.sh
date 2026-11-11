#!/bin/sh
# Build the renderer for the browser: web/rays.wasm.
# Needs the target once: rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")/.."
cargo build -p rays-web --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/rays_web.wasm web/rays.wasm
ls -l web/rays.wasm
