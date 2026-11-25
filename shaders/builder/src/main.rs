//! Builds the path tracer's shader crate to SPIR-V, copied to `src/gpu/pathtrace.spv` for
//! native GPUs, and translates it with naga to WGSL, `src/gpu/pathtrace.wgsl`, for WebGPU in
//! browsers (which take only WGSL) and for native backends other than Vulkan. The translation
//! is validated with WebGPU's baseline capabilities, so a shader a browser would reject fails
//! here.
//!
//! Then again with the crate's `ray_query` feature, tracing by ray queries rather than the BVH,
//! to WGSL alone (`src/gpu/pathtrace-rq.wgsl`), for native devices that have them: Vulkan's
//! SPIR-V goes through wgpu's own naga, whose SPIR-V reader can't read ray queries, so every
//! backend takes this one as WGSL.
use naga::{Block, Span, Statement};
use spirv_builder::{Capability, ShaderPanicStrategy, SpirvBuilder};
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
    let mut module = naga::front::spv::parse_u8_slice(&bytes, &naga::front::spv::Options::default())?;
    let folded = fold_continuing_blocks(&mut module)?;
    let webgpu = || naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty());
    let info = match webgpu().validate(&module) {
        Ok(info) => info,
        Err(e) => {
            // Which type or function it is, since the error names only a handle.
            if let naga::valid::ValidationError::Type { handle, .. } = e.as_inner() {
                eprintln!("naga refuses the type {:?}: {:?}", handle, module.types[*handle]);
                if let naga::TypeInner::Struct { members, .. } = &module.types[*handle].inner {
                    for m in members {
                        eprintln!("  its member {:?}: {:?}", m.ty, module.types[m.ty]);
                    }
                }
                for (h, t) in module.types.iter() {
                    if let naga::TypeInner::Struct { members, .. } = &t.inner {
                        if members.iter().any(|m| m.ty == *handle) {
                            eprintln!("  a member of {:?}: {:?}", h, t);
                        }
                    }
                }
            }
            return Err(e.into());
        },
    };
    let wgsl = naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())?;
    // As a browser would: the WGSL must parse and validate again.
    webgpu().validate(&naga::front::wgsl::parse_str(&wgsl)?)?;
    let out = here.join("../../src/gpu/pathtrace.wgsl");
    std::fs::write(&out, &wgsl)?;
    println!("wrote {} ({} KB, {} loops folded)", out.display(), wgsl.len() / 1024, folded);

    let result = SpirvBuilder::new(here.join("../pathtrace"), "spirv-unknown-vulkan1.2")
        .shader_panic_strategy(ShaderPanicStrategy::SilentExit)
        .capability(Capability::RayQueryKHR)
        .extension("SPV_KHR_ray_query")
        .shader_crate_features(["ray_query".to_string()])
        .build()?;
    let bytes = std::fs::read(result.module.unwrap_single())?;
    let mut module = naga::front::spv::parse_u8_slice(&bytes, &naga::front::spv::Options::default())?;
    let folded = fold_continuing_blocks(&mut module)?;
    let native = || naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::RAY_QUERY);
    let info = native().validate(&module).map_err(|e| {
        // The expression an error names, which it gives as a handle alone.
        if let naga::valid::ValidationError::Function { handle, source: naga::valid::FunctionError::Expression { handle: expr, .. }, .. } = e.as_inner() {
            let f = &module.functions[*handle];
            eprintln!("in {:?}: {:?} = {:?}", f.name, expr, f.expressions[*expr]);
        }
        format!("{:?}", e)
    })?;
    let wgsl = naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())?;
    let out = here.join("../../src/gpu/pathtrace-rq.wgsl");
    std::fs::write(&out, &wgsl)?;
    native().validate(&naga::front::wgsl::parse_str(&wgsl).map_err(|e| e.emit_to_string(&wgsl))?)
        .map_err(|e| format!("{} (written to {}, which the error's spans are in)", e.emit_to_string(&wgsl), out.display()))?;
    println!("wrote {} ({} KB, {} loops folded)", out.display(), wgsl.len() / 1024, folded);
    Ok(())
}

/// Folds each loop's continuing block into the end of its body, for naga's MSL and HLSL
/// writers (Metal, DX12; naga 30). Those writers emit the continuing block before the body
/// (`while (true) { if (!init) { continuing; if (c) break; } init = false; body }`), where the
/// body's named values aren't in scope, so each value of the body the continuing block uses
/// (or its `break if` condition c) is written out again as an expression, and re-evaluated
/// there, after the continuing block's earlier stores. rust-gpu's loops pass every variable
/// through the continuing block (`continuing { lo = next_lo; ...; break if !go; }`, with
/// `go = lo + 1 < hi` from the body), so a value computed from a variable the block has just
/// updated comes out wrong: `while lo + 1 < hi` left one iteration early (on Metal the sky's
/// binary search returned a row one too low, and BVH traversal lost light: 12% of a diffuse
/// floor's), and a count kept across iterations came out a quarter of the right one. SPIR-V
/// keeps the values, so Vulkan never sees it.
///
/// `loop { body; continue; continuing { stores; break if c; } }` becomes
/// `loop { body; stores; if c { break; } }`: the same where the body's only `continue` is its
/// last statement, as in rust-gpu's loops; others fail here, as naga's validator would on a
/// value out of scope. Returns the number of loops folded.
fn fold_continuing_blocks(module: &mut naga::Module) -> Result<usize, String> {
    let mut count = 0;
    let functions = module.functions.iter_mut().map(|(_, f)| f);
    for f in functions.chain(module.entry_points.iter_mut().map(|ep| &mut ep.function)) {
        fold_in_block(&mut f.body, &mut count)?;
    }
    Ok(count)
}

fn fold_in_block(block: &mut Block, count: &mut usize) -> Result<(), String> {
    for statement in block.iter_mut() {
        match statement {
            Statement::Block(b) => fold_in_block(b, count)?,
            Statement::If { accept, reject, .. } => {
                fold_in_block(accept, count)?;
                fold_in_block(reject, count)?;
            },
            Statement::Switch { cases, .. } => {
                for case in cases.iter_mut() {
                    fold_in_block(&mut case.body, count)?;
                }
            },
            Statement::Loop { body, continuing, break_if } => {
                fold_in_block(body, count)?;
                fold_in_block(continuing, count)?;
                if continuing.is_empty() && break_if.is_none() {
                    continue;
                }
                if matches!(body.last(), Some(Statement::Continue)) {
                    body.cull(body.len() - 1..);
                }
                if continues(body) {
                    return Err("a loop with a continuing block continues before the end of its body".into());
                }
                body.append(continuing);
                if let Some(condition) = break_if.take() {
                    let mut accept = Block::new();
                    accept.push(Statement::Break, Span::UNDEFINED);
                    body.push(Statement::If { condition, accept, reject: Block::new() }, Span::UNDEFINED);
                }
                *count += 1;
            },
            _ => {},
        }
    }
    Ok(())
}

/// Whether `block` has a `continue` for the loop it is in (not one of a nested loop).
fn continues(block: &Block) -> bool {
    block.iter().any(|s| match s {
        Statement::Continue => true,
        Statement::Block(b) => continues(b),
        Statement::If { accept, reject, .. } => continues(accept) || continues(reject),
        Statement::Switch { cases, .. } => cases.iter().any(|c| continues(&c.body)),
        _ => false,
    })
}
