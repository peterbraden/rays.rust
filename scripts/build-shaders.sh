#!/bin/sh
# Compile the GPU path tracer's shader (shaders/pathtrace, in Rust) to SPIR-V with rust-gpu,
# writing src/gpu/pathtrace.spv and .wgsl (not committed), which the renderer includes (with
# --features gpu; its build.rs runs this when the shader's sources change). Needs
# rustup: the shaders' workspace pins the nightly toolchain rust-gpu requires
# (shaders/rust-toolchain.toml), which rustup installs on first use. The first build compiles
# rust-gpu's codegen backend (a few minutes); later ones take seconds.
# Usage: scripts/build-shaders.sh
set -e
cd "$(dirname "$0")/../shaders"
cargo run --release -p builder
cd ..
scripts/shader-sources.sh > src/gpu/pathtrace.spv.sources
