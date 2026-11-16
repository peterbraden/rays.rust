//! Builds the path tracer's shader crate to SPIR-V, copied to `src/gpu/pathtrace.spv` for
//! native GPUs, and translates it with naga to WGSL, `src/gpu/pathtrace.wgsl`, for WebGPU in
//! browsers (which take only WGSL) and for native backends other than Vulkan. The translation
//! is validated with WebGPU's baseline capabilities, so a shader a browser would reject fails
//! here.
use naga::{Block, Expression, Handle, Span, Statement};
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
    let mut module = naga::front::spv::parse_u8_slice(&bytes, &naga::front::spv::Options::default())?;
    let hoisted = hoist_break_ifs(&mut module)?;
    let webgpu = || naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty());
    let info = webgpu().validate(&module)?;
    let wgsl = naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())?;
    // As a browser would: the WGSL must parse and validate again.
    webgpu().validate(&naga::front::wgsl::parse_str(&wgsl)?)?;
    let out = here.join("../../src/gpu/pathtrace.wgsl");
    std::fs::write(&out, &wgsl)?;
    println!("wrote {} ({} KB, {} loop conditions hoisted)", out.display(), wgsl.len() / 1024, hoisted);
    Ok(())
}

/// Moves each loop's `break if` condition out of the loop body, for naga's MSL and HLSL
/// writers (Metal, DX12; naga 30). WGSL's `loop { body continuing { stores; break if c; } }`
/// evaluates c after the stores, but c may be (or be computed from) a value the body computed,
/// as rust-gpu's loops are: `while lo + 1 < hi` becomes `let go = lo + 1 < hi; ... continuing
/// { lo = next_lo; ...; break if !go; }`. The writers emit the continuing block before the body
/// (`while (true) { if (!init) { continuing; if (c) break; } init = false; body }`), where the
/// body's named values aren't in scope, so they write c's expression out again, re-reading
/// `lo` and `hi` after the continuing block has updated them: the loop leaves one iteration
/// early. (On Metal the sky's binary search returned a row one too low, and BVH traversal lost
/// light: 12% of a diffuse floor's.) SPIR-V keeps the value, so Vulkan never sees it.
///
/// Here the body's value is stored in a local variable where it is computed and loaded in the
/// continuing block, so whatever re-emits the condition re-reads that variable, which nothing
/// else writes. Handles the shapes rust-gpu makes (c or `!c`, with c emitted at the top level
/// of the body) and fails on others. Returns the number of conditions moved.
fn hoist_break_ifs(module: &mut naga::Module) -> Result<usize, String> {
    let bool_ty = module.types.insert(
        naga::Type { name: None, inner: naga::TypeInner::Scalar(naga::Scalar::BOOL) },
        Span::UNDEFINED,
    );
    let mut count = 0;
    let functions = module.functions.iter_mut().map(|(_, f)| f);
    for f in functions.chain(module.entry_points.iter_mut().map(|ep| &mut ep.function)) {
        let mut body = std::mem::take(&mut f.body);
        hoist_in_block(&mut body, f, bool_ty, &mut count)?;
        f.body = body;
    }
    Ok(count)
}

fn hoist_in_block(block: &mut Block, f: &mut naga::Function, bool_ty: Handle<naga::Type>, count: &mut usize) -> Result<(), String> {
    for statement in block.iter_mut() {
        match statement {
            Statement::Block(b) => hoist_in_block(b, f, bool_ty, count)?,
            Statement::If { accept, reject, .. } => {
                hoist_in_block(accept, f, bool_ty, count)?;
                hoist_in_block(reject, f, bool_ty, count)?;
            },
            Statement::Switch { cases, .. } => {
                for case in cases.iter_mut() {
                    hoist_in_block(&mut case.body, f, bool_ty, count)?;
                }
            },
            Statement::Loop { body, continuing, break_if } => {
                hoist_in_block(body, f, bool_ty, count)?;
                hoist_in_block(continuing, f, bool_ty, count)?;
                let Some(condition) = *break_if else { continue };
                // The value to keep, and whether the condition negates it in the continuing block.
                let (value, negated) = match f.expressions[condition] {
                    _ if emitted_at(body, condition).is_some() => (condition, false),
                    Expression::Unary { op: naga::UnaryOperator::LogicalNot, expr } if emitted_at(continuing, condition).is_some() => (expr, true),
                    _ if emitted_at(continuing, condition).is_some() || f.expressions[condition].needs_pre_emit() => continue,
                    ref e => return Err(format!("a loop's break-if condition of an unexpected shape: {:?}", e)),
                };
                let Some(at) = emitted_at(body, value) else {
                    if emitted_at(continuing, value).is_some() || f.expressions[value].needs_pre_emit() {
                        continue;
                    }
                    return Err(format!("a loop's break-if condition reads a value not emitted at the top level of its body: {:?}", f.expressions[value]));
                };
                let var = f.local_variables.append(
                    naga::LocalVariable { name: Some(format!("break_if_{}", *count)), ty: bool_ty, init: None },
                    Span::UNDEFINED,
                );
                let pointer = f.expressions.append(Expression::LocalVariable(var), Span::UNDEFINED);
                body.splice(at + 1..at + 1, Block::from_vec(vec![Statement::Store { pointer, value }]));
                let load = f.expressions.append(Expression::Load { pointer }, Span::UNDEFINED);
                let last = if negated {
                    f.expressions.append(Expression::Unary { op: naga::UnaryOperator::LogicalNot, expr: load }, Span::UNDEFINED)
                } else {
                    load
                };
                continuing.push(Statement::Emit(naga::Range::new_from_bounds(load, last)), Span::UNDEFINED);
                *break_if = Some(last);
                *count += 1;
            },
            _ => {},
        }
    }
    Ok(())
}

/// The index of the top-level statement of `block` that emits `e`.
fn emitted_at(block: &Block, e: Handle<Expression>) -> Option<usize> {
    block.iter().position(|s| matches!(s, Statement::Emit(r) if r.index_range().contains(&(e.index() as u32))))
}
