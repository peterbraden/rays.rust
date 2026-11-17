#!/bin/sh
# Print the SHA-256 of every file the GPU shader is built from. scripts/build-shaders.sh
# stores this beside src/gpu/pathtrace.spv; `scripts/shader-sources.sh --check` fails if the
# committed .spv was built from different sources (the shader was edited and not rebuilt).
set -e
cd "$(dirname "$0")/.."
sources() {
    find shaders/pathtrace shaders/builder -type f \( -name '*.rs' -o -name Cargo.toml \) -not -path '*/target/*' | LC_ALL=C sort |
        xargs sha256sum
    sha256sum shaders/Cargo.toml shaders/Cargo.lock shaders/rust-toolchain.toml
}
if [ "$1" = "--check" ]; then
    if ! sources | diff -u src/gpu/pathtrace.spv.sources -; then
        echo "src/gpu/pathtrace.spv is stale: run scripts/build-shaders.sh and commit it" >&2
        exit 1
    fi
else
    sources
fi
