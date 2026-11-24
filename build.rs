//! With `--features gpu`, builds the GPU path tracer's shader (`shaders/pathtrace`) before the
//! renderer that includes it: `scripts/build-shaders.sh` writes `src/gpu/pathtrace.spv` and
//! `.wgsl` with rust-gpu, on the nightly the shaders' workspace pins (rustup installs it on
//! first use; the first build of rust-gpu's codegen backend takes a few minutes, later ones
//! seconds). These outputs aren't committed: each is rebuilt when the sources they were built
//! from (`scripts/shader-sources.sh`, kept as `src/gpu/pathtrace.spv.sources`) change.

use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_GPU");
    if std::env::var_os("CARGO_FEATURE_GPU").is_none() {
        return;
    }
    for path in [
        "shaders/pathtrace",
        "shaders/shared",
        "shaders/builder",
        "shaders/Cargo.toml",
        "shaders/Cargo.lock",
        "shaders/rust-toolchain.toml",
        "scripts/build-shaders.sh",
        "scripts/shader-sources.sh",
    ] {
        println!("cargo:rerun-if-changed={}", path);
    }

    let sources = output(Command::new("sh").arg("scripts/shader-sources.sh"));
    let built = std::fs::read("src/gpu/pathtrace.spv.sources").unwrap_or_default();
    if built == sources
        && Path::new("src/gpu/pathtrace.spv").exists()
        && Path::new("src/gpu/pathtrace.wgsl").exists()
    {
        return;
    }

    // A cargo of its own, on the shaders' toolchain: nothing of this build's may reach it
    // (its toolchain, flags, target, jobserver or target directory).
    let mut build = Command::new("sh");
    build.arg("scripts/build-shaders.sh");
    for (key, _) in std::env::vars_os() {
        let key = key.to_string_lossy();
        let cargo = key.starts_with("CARGO") && key != "CARGO_HOME";
        let rust = key.starts_with("RUSTC")
            || key.starts_with("RUSTDOC")
            || key == "RUSTFLAGS"
            || key == "RUSTUP_TOOLCHAIN";
        let script = matches!(
            &*key,
            "TARGET" | "HOST" | "OUT_DIR" | "OPT_LEVEL" | "PROFILE" | "DEBUG" | "NUM_JOBS"
        );
        if cargo || rust || script {
            build.env_remove(&*key);
        }
    }
    println!(
        "cargo:warning=Building the GPU shader (scripts/build-shaders.sh; the first build takes a few minutes)"
    );
    let status = build
        .status()
        .unwrap_or_else(|e| panic!("couldn't run scripts/build-shaders.sh: {}", e));
    if !status.success() {
        panic!(
            "scripts/build-shaders.sh failed ({}): building the GPU shader needs rustup, which installs the \
             nightly that shaders/rust-toolchain.toml pins",
            status
        );
    }
}

fn output(command: &mut Command) -> Vec<u8> {
    let out = command
        .output()
        .unwrap_or_else(|e| panic!("couldn't run {:?}: {}", command, e));
    if !out.status.success() {
        panic!(
            "{:?} failed: {}",
            command,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    out.stdout
}
