//! What code in `shaders/shared` must keep to, so that code written and tested only on the CPU
//! is ready for the GPU: checked here on its source, on stable, by `cargo test`. The proof is
//! still rust-gpu compiling it (`scripts/build-shaders.sh`, which CI runs); these catch what
//! that would refuse, or would let through wrongly, before anyone has that toolchain.
//!
//! - Nothing a kernel can't hold: no trait objects, no heap, no f64 (the crate is `no_std`
//!   without an allocator, which keeps `Box`, `Vec` and `String` out of scope; this forbids
//!   naming them, and `dyn`, at all).
//! - Constants as ratios of integers (`Real::ratio`), not float literals: a literal would be
//!   one float's, and a formula moved here must not change the renderer's results by a bit.
//! - None of what rust-gpu or naga are known to refuse (CLAUDE.md lists them).
//! - Every public function is used by the shader or by another function here: rust-gpu
//!   compiles a generic function only where something calls it, so one the kernels never call
//!   has never been compiled for the GPU, whatever the CPU's tests say.

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// The crate's sources, comments and strings blanked: (file, line number, code).
    fn code(dir: &str) -> Vec<(String, usize, String)> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let mut out = Vec::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "rs") {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                for (n, line) in std::fs::read_to_string(&path).unwrap().lines().enumerate() {
                    // (A file's tests, which end it, are the CPU's alone: no kernel has them.)
                    if line.starts_with("#[cfg(test)]") {
                        break;
                    }
                    let line = line.split("//").next().unwrap();
                    out.push((name.clone(), n + 1, line.to_string()));
                }
            }
        }
        out
    }

    /// Whether `word` is in `line` as a word of its own.
    fn has_word(line: &str, word: &str) -> bool {
        let is_part = |c: char| c.is_alphanumeric() || c == '_';
        line.match_indices(word).any(|(i, _)| {
            !line[..i].chars().next_back().is_some_and(is_part) && !line[i + word.len()..].chars().next().is_some_and(is_part)
        })
    }

    /// Whether the line has a float literal: digits, a point, and not a tuple's field, a
    /// range or a method on an integer.
    fn has_float_literal(line: &str) -> bool {
        let c: Vec<char> = line.chars().collect();
        (1..c.len()).any(|i| {
            if c[i] != '.' || !c[i - 1].is_ascii_digit() {
                return false;
            }
            // The number the point ends: a literal only if it doesn't follow a name or a
            // point (`p.0.1`, `x1.5`).
            let start = (0..i).rev().find(|&k| !c[k].is_ascii_digit() && c[k] != '_').map_or(0, |k| k + 1);
            let before = if start == 0 { ' ' } else { c[start - 1] };
            let after = c.get(i + 1).copied().unwrap_or(' ');
            !(before.is_alphanumeric() || before == '_' || before == '.') && after != '.' && !after.is_alphabetic() && after != '_'
        })
    }

    /// The part of `lib.rs` that is the floats themselves (`impl Real for f32`, `for f64`),
    /// which is where f64 and float literals belong.
    fn in_the_floats(lines: &[(String, usize, String)], k: usize) -> bool {
        let (file, _, _) = &lines[k];
        file == "lib.rs" && lines[..=k].iter().rev().filter(|(f, _, _)| f == file)
            .find_map(|(_, _, l)| if l.starts_with("impl Real for") { Some(true) } else if l.starts_with("pub trait") || l.starts_with("pub fn") { Some(false) } else { None })
            .unwrap_or(false)
    }

    #[test]
    fn test_shared_code_is_what_a_kernel_can_hold() {
        let lines = code("shaders/shared/src");
        assert!(lines.len() > 500, "found {} lines of the shared crate", lines.len());
        let forbidden: [(&str, &str); 12] = [
            ("dyn", "trait objects: a kernel has no pointers to functions"),
            ("Box", "the heap"), ("Vec", "the heap"), ("String", "the heap"), ("Rc", "the heap"), ("Arc", "the heap"),
            ("alloc", "the heap"), ("std", "the standard library (core has what a kernel can use)"),
            ("f64", "f64: the GPU has none; be generic over `Real`"),
            ("f32", "a float of one size: be generic over `Real`"),
            ("is_finite", "`is_finite`, whose infinite constant naga refuses"),
            ("unsafe", "unsafe code"),
        ];
        let mut found = Vec::new();
        for (k, (file, n, line)) in lines.iter().enumerate() {
            if in_the_floats(&lines, k) || (file == "lib.rs" && (line.starts_with("extern crate std") || line.contains("target_arch"))) {
                continue;
            }
            for (word, why) in forbidden {
                if has_word(line, word) {
                    found.push(format!("{}:{}: {} ({})", file, n, word, why));
                }
            }
            if line.contains("..=") {
                found.push(format!("{}:{}: an inclusive range, which rust-gpu refuses", file, n));
            }
            if has_float_literal(line) {
                found.push(format!("{}:{}: a float literal: write it as `F::of` or `F::ratio`, so it is each float's nearest", file, n));
            }
        }
        assert!(found.is_empty(), "shaders/shared isn't ready for the GPU:\n{}", found.join("\n"));
    }

    /// The names of the public functions the lines define.
    fn public_functions(lines: &[(String, usize, String)]) -> Vec<(String, String)> {
        lines.iter().filter_map(|(file, _, line)| {
            let rest = line.trim_start().strip_prefix("pub fn ")?;
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            Some((file.clone(), name))
        }).collect()
    }

    /// Functions only the renderer calls, on purpose: it works these out once for a medium's
    /// record, which the kernels read. Not known to compile for the GPU; a kernel that comes
    /// to call one takes it off this list.
    const HOST_ONLY: [&str; 3] = ["forward_peak", "truncated_extinction", "truncated_albedo"];

    #[test]
    fn test_shared_code_is_compiled_for_the_gpu() {
        let shared = code("shaders/shared/src");
        let shader = code("shaders/pathtrace/src");
        let mut unused = Vec::new();
        for (file, name) in public_functions(&shared) {
            // Called from the shader, or from somewhere else here (which is then held to the
            // same): as a method or a path's last part.
            let called = |lines: &[(String, usize, String)]| lines.iter().any(|(_, _, l)| {
                !l.contains(&format!("fn {}", name)) && (l.contains(&format!(".{}(", name)) || l.contains(&format!("{}(", name)) || l.contains(&format!("{}::<", name)))
                    && has_word(l, &name)
            });
            if !called(&shader) && !called(&shared) && !HOST_ONLY.contains(&name.as_str()) {
                unused.push(format!("{}: {}", file, name));
            }
        }
        assert!(unused.is_empty(), "public functions of shaders/shared that no kernel calls, so that rust-gpu has never compiled them:\n{}", unused.join("\n"));
    }

    #[test]
    fn test_the_rules_see_what_they_forbid() {
        assert!(has_float_literal("let x = 0.5 * y;") && has_float_literal("f(1., 2)") && has_float_literal("a + 1e3 * 2.0"));
        assert!(!has_float_literal("let a = p.0 + q.1;") && !has_float_literal("while k < 3 { i += 1 }") && !has_float_literal("(o.0.1, 0..4)") && !has_float_literal("F::ratio(1, 2)"));
        assert!(has_word("x: &dyn Medium", "dyn") && !has_word("let dynamic = 1;", "dyn") && has_word("Vec<F>", "Vec") && !has_word("Vector", "Vec"));
    }
}
