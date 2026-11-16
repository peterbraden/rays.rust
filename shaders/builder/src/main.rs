//! Builds the path tracer's shader crate to SPIR-V and copies it to `src/gpu/pathtrace.spv`.
use spirv_builder::{SpirvBuilder, ShaderPanicStrategy};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let result = SpirvBuilder::new(here.join("../pathtrace"), "spirv-unknown-vulkan1.1")
        .shader_panic_strategy(ShaderPanicStrategy::SilentExit)
        .build()?;
    let spv = result.module.unwrap_single();
    let out = here.join("../../src/gpu/pathtrace.spv");
    std::fs::copy(spv, &out)?;
    println!("wrote {}", out.display());
    Ok(())
}
