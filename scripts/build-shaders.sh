#!/bin/sh
# Compile the GPU path tracer's shader (shaders/pathtrace, in Rust) to SPIR-V with rust-gpu,
# writing src/gpu/pathtrace.spv and .wgsl (not committed), which the renderer includes (with
# --features gpu; its build.rs runs this when the shader's sources change). Needs
# rustup: the shaders' workspace pins the nightly toolchain rust-gpu requires
# (shaders/rust-toolchain.toml), which rustup installs on first use. The first build compiles
# rust-gpu's codegen backend (a few minutes); later ones take seconds.
# Usage: scripts/build-shaders.sh
set -e
cd "$(dirname "$0")/.."
# The builder translates with naga from the wgpu submodule, with ray queries added
# (vendor/naga-ray-query.patch: its SPIR-V reader and WGSL writer), until that is on a fork.
if [ ! -f vendor/wgpu/naga/Cargo.toml ]; then
    git submodule update --init --depth 1 vendor/wgpu
fi
if ! git -C vendor/wgpu apply --reverse --check ../naga-ray-query.patch 2>/dev/null; then
    # Not applied, or an older version of it: from v30.0.2 afresh.
    git -C vendor/wgpu checkout -q -- naga
    git -C vendor/wgpu clean -fdq naga
    git -C vendor/wgpu apply ../naga-ray-query.patch
fi
cd shaders
cargo run --release -p builder
cd ..
scripts/shader-sources.sh > src/gpu/pathtrace.spv.sources
