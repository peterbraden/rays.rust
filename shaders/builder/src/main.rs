//! Builds the path tracer's shader crate to SPIR-V, copied to `src/gpu/pathtrace.spv` for
//! native GPUs, and translates it with naga to WGSL, `src/gpu/pathtrace.wgsl`, for WebGPU in
//! browsers (which take only WGSL). The translation is validated with WebGPU's baseline
//! capabilities, so a shader a browser would reject fails here.
use spirv_builder::{ShaderPanicStrategy, SpirvBuilder};
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

    let bytes = std::fs::read(&out)?;
    let module = naga::front::spv::parse_u8_slice(&bytes, &naga::front::spv::Options::default())?;
    let webgpu = || naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty());
    let info = webgpu().validate(&module)?;
    let wgsl = naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())?;
    // As a browser would: the WGSL must parse and validate again.
    webgpu().validate(&naga::front::wgsl::parse_str(&wgsl)?)?;
    let out = here.join("../../src/gpu/pathtrace.wgsl");
    std::fs::write(&out, &wgsl)?;
    println!("wrote {} ({} KB)", out.display(), wgsl.len() / 1024);
    Ok(())
}
