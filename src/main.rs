#![allow(dead_code)]
#![allow(unused_imports)]

use clap::Parser;
use rayon::prelude::*;
use rays::*;

use rays::trace::trace;
use rays::rendercontext::RenderContext;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use console::style;


/// Rays
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Set scene file
    #[arg(index=1)]
    scene: String,

    ///Update the output file when a chunk is completed. Good for debugging
    #[arg(short, long)]
    progressive_render: bool,

    ///Set the width of the output image; overrides the scenefile if specified
    #[arg(long)]
    width: Option<usize>,

    ///Set the height of the output image; overrides the scenefile if specified
    #[arg(long)]
    height: Option<usize>,

    ///Write the image here instead of alongside the scene file. The extension selects the
    ///format: .png (8-bit, encoded for display) or .exr (linear floating point)
    #[arg(short, long)]
    output: Option<String>,

    ///Compare the render with this reference image (eg. a linear EXR from another renderer):
    ///print the exposure fit, RMSE and FLIP error, and write a FLIP error map alongside the
    ///output
    #[arg(long)]
    compare: Option<String>,

    ///Also write a depth image: the distance to the nearest surface through each pixel's centre.
    ///.exr keeps distances (misses are infinite); .png scales them for viewing, near white
    #[arg(long)]
    depth: Option<String>,

    ///With --compare, fail (exit status 2) if the mean FLIP error exceeds this
    #[arg(long)]
    max_flip: Option<f64>,

    ///Seed for the render's random numbers; overrides the scenefile if specified. The same
    ///seed gives the same image
    #[arg(long)]
    seed: Option<u64>,

    ///Samples per pixel to render to; overrides the scenefile's supersamples. With a saved
    ///state, only the samples it lacks are rendered, so a quick render can be refined later
    #[arg(long)]
    samples: Option<usize>,

    ///Render this many more samples per pixel than the saved state has
    #[arg(long, conflicts_with = "samples")]
    add_samples: Option<usize>,

    ///Adaptive sampling: each pixel stops once its standard error, relative to its displayed
    ///value, is below this (eg. 0.01), with supersamples the most it takes; overrides the
    ///scene's "adaptive" threshold. 0 turns it off
    #[arg(long)]
    noise_threshold: Option<f64>,

    ///Also write a map of each pixel's samples, relative to the most any pixel took: where
    ///adaptive sampling spent them
    #[arg(long)]
    sample_map: Option<String>,

    ///Also write a profile of the render: each pixel's time rendering its samples, as a heatmap
    ///(black through orange to pale yellow at the 99.5th percentile's time); .exr keeps the
    ///seconds. Path tracer only
    #[arg(long)]
    time_map: Option<String>,

    ///Start afresh, rather than carrying on from the saved state (<output>.accum)
    #[arg(long)]
    fresh: bool,

    ///Render with the wavefront integrator (the kernels) on the CPU, not the GPU (as the
    ///scene's "integrator": "wavefront"); with the path tracer if they can't render the scene
    #[arg(long)]
    wavefront: bool,

    ///Render on the GPU even if it is a software one (as "integrator": "gpu"; needs a build
    ///with `--features gpu`); otherwise with the wavefront integrator or the path tracer.
    ///Unasked, a scene renders on a hardware GPU if there is one, else with the wavefront
    ///integrator on the CPU, else with the path tracer: the first that can render it
    #[arg(long)]
    gpu: bool,

    ///Render with the recursive path tracer (as "integrator": "path")
    #[arg(long, conflicts_with_all = ["wavefront", "gpu"])]
    path_tracer: bool,

    ///Write the scene file, as JSON in our format, to this file. For a scene in another
    ///renderer's format (eg. .nff), this is the translation
    #[arg(long)]
    dump_scene: Option<String>,

    ///Measure the render's noise: render this many times, with successive seeds, and print the
    ///relative mean squared error of one render and the efficiency 1 / (relMSE x time). Writes
    ///the mean of the renders as the output, and a map of each pixel's relative standard
    ///deviation alongside it (.variance.png)
    #[arg(long)]
    variance: Option<usize>,
}

/// How often a render's state is saved, so an interrupted render loses no more than this.
const STATE_INTERVAL: Duration = Duration::from_secs(30);

/// Write a render's state, via a temporary file so that a partial state is never left.
fn save_state(path: &str, bytes: &[u8]) {
    let tmp = format!("{}.tmp", path);
    if let Err(e) = std::fs::write(&tmp, bytes).and_then(|_| std::fs::rename(&tmp, path)) {
        eprintln!("Failed to save the render's state to {}: {}", path, e);
    }
}

/// The process's peak resident memory so far, in bytes: large meshes and their acceleration
/// structures are the main use. `getrusage` reports `ru_maxrss` in bytes on macOS and in
/// kilobytes on Linux and the BSDs.
#[cfg(unix)]
fn peak_memory() -> Option<u64> {
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return None;
    }
    let units = if cfg!(target_os = "macos") { 1 } else { 1024 };
    Some(usage.ru_maxrss as u64 * units)
}

#[cfg(not(unix))]
fn peak_memory() -> Option<u64> {
    None
}

/// A value of the system's (`sysctlbyname`), as text or as a number.
#[cfg(target_os = "macos")]
fn sysctl(name: &str) -> Option<Vec<u8>> {
    let name = std::ffi::CString::new(name).ok()?;
    let mut len = 0usize;
    if unsafe { libc::sysctlbyname(name.as_ptr(), std::ptr::null_mut(), &mut len, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let mut buf = vec![0u8; len];
    if unsafe { libc::sysctlbyname(name.as_ptr(), buf.as_mut_ptr().cast(), &mut len, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    buf.truncate(len);
    Some(buf)
}

/// A line of `/proc/<file>` that begins `key`, after it and its colon.
#[cfg(target_os = "linux")]
fn proc_field(file: &str, key: &str) -> Option<String> {
    let text = std::fs::read_to_string(format!("/proc/{}", file)).ok()?;
    text.lines().find(|l| l.starts_with(key)).and_then(|l| l.split_once(':')).map(|(_, v)| v.trim().to_string())
}

/// The processor's name, as the system gives it.
fn machine_cpu() -> Option<String> {
    #[cfg(target_os = "macos")]
    return sysctl("machdep.cpu.brand_string").map(|b| String::from_utf8_lossy(&b).trim_end_matches('\0').to_string());
    #[cfg(target_os = "linux")]
    return proc_field("cpuinfo", "model name");
    #[allow(unreachable_code)]
    None
}

/// The machine's memory, bytes.
fn machine_memory() -> Option<u64> {
    #[cfg(target_os = "macos")]
    return sysctl("hw.memsize").and_then(|b| Some(u64::from_ne_bytes(b.get(..8)?.try_into().ok()?)));
    #[cfg(target_os = "linux")]
    return proc_field("meminfo", "MemTotal").and_then(|v| v.trim_end_matches("kB").trim().parse::<u64>().ok()).map(|kb| kb * 1024);
    #[allow(unreachable_code)]
    None
}

/// The commit of the repository this program was run from (the one its executable is in, as
/// `target/release/rays` is), and whether its sources had changes not committed. As the
/// repository stands now, which needn't be what the program was built from.
fn source_revision() -> (Option<String>, Option<bool>) {
    let Some(dir) = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())) else { return (None, None) };
    let git = |args: &[&str]| std::process::Command::new("git").arg("-C").arg(&dir).args(args).output().ok()
        .filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let commit = git(&["rev-parse", "HEAD"]);
    let dirty = commit.as_ref().and_then(|_| git(&["status", "--porcelain", "--", ":/src", ":/shaders", ":/Cargo.toml", ":/Cargo.lock", ":/build.rs"]))
        .map(|s| !s.is_empty());
    (commit, dirty)
}

/// The process's memory and the machine's now (`report::Reading`). macOS: the footprint and
/// its lifetime peak and page-ins by `proc_pid_rusage`; free memory as `memory_pressure`
/// gives it (`kern.memorystatus_level`, a percentage, of the machine's); swapping by
/// `host_statistics64`.
#[cfg(target_os = "macos")]
fn read_memory() -> Option<report::Reading> {
    let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
    let got = unsafe { libc::proc_pid_rusage(std::process::id() as i32, libc::RUSAGE_INFO_V4, (&mut info as *mut libc::rusage_info_v4).cast()) };
    if got != 0 {
        return None;
    }
    let mut vm: libc::vm_statistics64 = unsafe { std::mem::zeroed() };
    let mut count = libc::HOST_VM_INFO64_COUNT;
    #[allow(deprecated)]
    let host = unsafe { libc::mach_host_self() };
    let vm_ok = unsafe { libc::host_statistics64(host, libc::HOST_VM_INFO64, (&mut vm as *mut libc::vm_statistics64).cast(), &mut count) } == 0;
    let level = sysctl("kern.memorystatus_level").and_then(|b| Some(u32::from_ne_bytes(b.get(..4)?.try_into().ok()?)));
    let free = level.zip(machine_memory()).map_or(0, |(percent, total)| total / 100 * percent as u64);
    Some(report::Reading {
        footprint: info.ri_phys_footprint,
        peak: info.ri_lifetime_max_phys_footprint,
        pageins: info.ri_pageins,
        free,
        swapins: if vm_ok { vm.swapins } else { 0 },
        swapouts: if vm_ok { vm.swapouts } else { 0 },
    })
}

/// As on macOS, from `/proc`: resident size and its peak, major faults, available memory,
/// pages swapped.
#[cfg(target_os = "linux")]
fn read_memory() -> Option<report::Reading> {
    let kb = |v: String| v.trim_end_matches("kB").trim().parse::<u64>().ok().map(|k| k * 1024);
    let vmstat = std::fs::read_to_string("/proc/vmstat").unwrap_or_default();
    let counter = |name: &str| vmstat.lines().find_map(|l| l.strip_prefix(name).and_then(|v| v.trim().parse::<u64>().ok())).unwrap_or(0);
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    // Fields after the command's closing parenthesis: majflt is the tenth.
    let majflt = stat.rsplit_once(')').and_then(|(_, rest)| rest.split_whitespace().nth(9)?.parse().ok()).unwrap_or(0);
    Some(report::Reading {
        footprint: proc_field("self/status", "VmRSS").and_then(kb)?,
        peak: proc_field("self/status", "VmHWM").and_then(kb).unwrap_or(0),
        pageins: majflt,
        free: proc_field("meminfo", "MemAvailable").and_then(kb).unwrap_or(0),
        swapins: counter("pswpin "),
        swapouts: counter("pswpout "),
    })
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn read_memory() -> Option<report::Reading> {
    None
}

/// Read memory into the run's report every `report::memory_interval()` seconds, for as long as
/// the program runs.
fn start_memory_sampler() {
    std::thread::spawn(|| loop {
        if let Some(r) = read_memory() {
            report::log_memory(r);
        }
        std::thread::sleep(Duration::from_secs_f64(report::memory_interval()));
    });
}

/// What a run is made with: the program, the machine.
fn run_context() -> report::Context {
    let (commit, dirty) = source_revision();
    report::Context {
        version: env!("CARGO_PKG_VERSION").to_string(),
        commit,
        dirty,
        arguments: std::env::args().collect(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu: machine_cpu(),
        threads: rayon::current_num_threads(),
        memory: machine_memory(),
    }
}

/// rays serve: run as a render server, and host the scene builder
#[derive(Parser, Debug)]
#[command(name = "rays serve", about = "Serve the scene builder, and render layers for it over HTTP", long_about = None)]
struct ServeArgs {
    ///Port to listen on
    #[arg(long, default_value_t = 8787)]
    port: u16,

    ///Address to listen on. The default is this machine only; 0.0.0.0 listens on every
    ///network interface, for other machines to use this one. There is no authentication
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,

    ///Directory to serve files from, and that scenes' asset paths are relative to
    #[arg(long, default_value = ".")]
    root: String,

    ///Origin of a page allowed to call the render API, eg. https://user.github.io. May be
    ///repeated. Pages served from this machine are always allowed
    #[arg(long)]
    allow_origin: Vec<String>,
}

/// rays report: what a render's runs did, from its saved state
#[derive(Parser, Debug)]
#[command(name = "rays report", about = "Print the report kept in a render's saved state: each run that added samples, \
what it ran on, what rendered it and why, where its time and memory went", long_about = None)]
struct ReportArgs {
    ///The render's image (its state, <image>.accum, is read) or the state itself
    #[arg(index = 1)]
    render: String,

    ///Print the report as JSON
    #[arg(long)]
    json: bool,
}

/// `rays report`: print a render's report.
fn print_report(a: ReportArgs) {
    let path = if a.render.ends_with(".accum") { a.render.clone() } else { rendercontext::state_path_for(&a.render) };
    let report = std::fs::read(&path).map_err(|e| format!("{}: {}", path, e))
        .and_then(|bytes| rendercontext::report_in_state(&bytes).map_err(|e| format!("{}: {}", path, e)));
    match report {
        Ok(r) if a.json => println!("{}", serde_json::to_string_pretty(&r).unwrap()),
        Ok(r) => print!("{}", r.summary()),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}

fn main() {
    // `rays serve ...` is a different program from `rays <scene>`: it has its own options.
    if std::env::args().nth(1).as_deref() == Some("serve") {
        let a = ServeArgs::parse_from(std::env::args().skip(1));
        if let Err(e) = std::env::set_current_dir(&a.root) {
            eprintln!("Can't use {} as the root: {}", a.root, e);
            std::process::exit(1);
        }
        let root = std::env::current_dir().unwrap();
        if let Err(e) = rays::server::serve(rays::server::Options { bind: a.bind, port: a.port, root, allow_origins: a.allow_origin }) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("report") {
        print_report(ReportArgs::parse_from(std::env::args().skip(1)));
        return;
    }
    let args = Args::parse();
    if args.time_map.is_some() {
        rays::time_map::enable();
    }

    println!("{}",
        style("# 1. - Parsing scene, building models").bold().cyan()
    );

    // This run, for the render's report (`report`), kept in its saved state.
    report::record();
    let mut run = report::Run::starting();
    run.context = run_context();
    report::set_probe(read_memory);
    report::enter("build");
    start_memory_sampler();
    let build_start = Instant::now();
    let mut doc = scenefile::SceneFile::read_document(&args.scene).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    // Weather without a seed gets a random one here, before the render is keyed by the scene,
    // so a new sky never resumes an old one's samples.
    weather::scene::choose_seeds(&mut doc);
    if let Some(path) = &args.dump_scene {
        let json = serde_json::to_string_pretty(&doc).unwrap();
        std::fs::write(path, json + "\n").unwrap_or_else(|e| {
            eprintln!("Can't write {}: {}", path, e);
            std::process::exit(1);
        });
        println!("- Wrote the scene file to {}", path);
    }
    // What the render's saved state is keyed by: the scene, in whatever format it was read.
    let contents = doc.to_string();
    let mut s = scenefile::SceneFile::from_value_overridden(doc, &scenefile::Overrides {
        width: args.width,
        height: args.height,
        seed: args.seed,
    });
    // Reading the scene and building its objects, each a part of it.
    run.phases.push(report::Phase { name: "build".into(), seconds: build_start.elapsed().as_secs_f64(), footprint: report::read().map(|r| r.footprint) });
    run.phases.extend(report::take_phases());

    let mut rc = RenderContext::new(
            s.image.width,
            s.image.height,
            args.progressive_render,
            &args.scene,
            );
    if let Some(path) = &args.output {
        rc.output_filename = path.clone();
    }
    // What renders the scene: the first of the GPU, the kernels on the CPU and the path tracer
    // that can, unless one was asked for.
    if args.path_tracer {
        s.render.integrator = scene::Integrator::Path;
    } else if args.gpu {
        s.render.integrator = scene::Integrator::Gpu;
    } else if args.wavefront {
        s.render.integrator = scene::Integrator::Wavefront;
    }
    run.decision.asked = format!("{:?}", s.render.integrator).to_lowercase();
    let refuse = |run: &mut report::Run, integrator: &str, reason: String| {
        run.decision.refused.push(report::Refusal { integrator: integrator.into(), reason });
    };
    let lap = |run: &mut report::Run, name: &str, since: Instant| {
        run.phases.push(report::Phase { name: name.into(), seconds: since.elapsed().as_secs_f64(), footprint: report::read().map(|r| r.footprint) });
    };
    report::enter("flatten");
    let flatten_start = Instant::now();
    let flat = if s.render.integrator != scene::Integrator::Path {
        match wavefront::flat::FlatScene::build(&s) {
            Ok(f) => {
                println!("- Wavefront integrator: {} primitives, {} planes, {} materials, {} emitters",
                    f.prims.len(), f.planes.len(), f.materials.len(), f.emitters.len());
                Some(f)
            },
            Err(e) => {
                println!("- The wavefront integrator can't render this scene ({}); rendering with the path tracer", e);
                refuse(&mut run, "wavefront", e.to_string());
                None
            },
        }
    } else {
        None
    };
    if s.render.integrator != scene::Integrator::Path {
        lap(&mut run, "flatten", flatten_start);
    }
    #[cfg(feature = "gpu")]
    let try_gpu = match s.render.integrator {
        scene::Integrator::Gpu => true,
        // Unasked, only a GPU that is one.
        scene::Integrator::Auto => match gpu::GpuRenderer::hardware() {
            Ok(()) => true,
            Err(e) => {
                refuse(&mut run, "gpu", e);
                false
            },
        },
        _ => false,
    };
    #[cfg(feature = "gpu")]
    let gpu_start = Instant::now();
    #[cfg(feature = "gpu")]
    report::enter("gpu set-up");
    #[cfg(feature = "gpu")]
    let mut gpu = match (&flat, try_gpu) {
        (Some(f), true) => match gpu::GpuRenderer::new(&s, f) {
            Ok(g) => {
                println!("- GPU: {}", g.adapter_name);
                let parts: Vec<String> = g.setup.iter().map(|(what, seconds)| format!("{} {:.0} ms", what, seconds * 1e3)).collect();
                println!("- GPU set up in {:.0} ms: {}", g.setup.iter().map(|p| p.1).sum::<f64>() * 1e3, parts.join(", "));
                lap(&mut run, "gpu set-up", gpu_start);
                run.phases.extend(report::take_phases());
                let adapter = g.adapter_name.trim_end_matches(", tracing by ray queries").to_string();
                let buffers = g.buffers.iter().map(|&(name, bytes)| report::GpuBuffer { name: name.into(), bytes }).collect();
                run.decision.gpu = Some(report::Gpu { adapter, ray_queries: g.ray_queries(), lanes: g.lanes, buffers });
                Some(g)
            },
            Err(e) => {
                println!("- The GPU can't render this scene ({}); rendering with the wavefront integrator", e);
                lap(&mut run, "gpu set-up", gpu_start);
                run.phases.extend(report::take_phases());
                refuse(&mut run, "gpu", e);
                None
            },
        },
        _ => None,
    };
    #[cfg(feature = "gpu")]
    let on_gpu = gpu.is_some();
    #[cfg(not(feature = "gpu"))]
    let on_gpu = false;
    #[cfg(not(feature = "gpu"))]
    if s.render.integrator == scene::Integrator::Gpu {
        println!("- This build has no GPU support (build with --features gpu); rendering with the wavefront integrator");
        refuse(&mut run, "gpu", "this build has no GPU support".into());
    }
    // Without the GPU the kernels run here, over the same buffers.
    report::enter("kernel buffers");
    let buffers_start = Instant::now();
    let buffers = match flat.as_ref().filter(|_| !on_gpu).map(|f| kernels::Buffers::build(&s, f)) {
        Some(Ok(b)) => Some(b),
        Some(Err(why)) => {
            println!("- The wavefront integrator can't render this scene ({}); rendering with the path tracer", why);
            refuse(&mut run, "wavefront", why.to_string());
            None
        },
        None => None,
    };
    if flat.is_some() && !on_gpu {
        lap(&mut run, "kernel buffers", buffers_start);
    }
    println!("- Integrator: {}", if on_gpu { "the kernels on the GPU" } else if buffers.is_some() { "the kernels on the CPU (wavefront)" } else { "the path tracer" });
    run.decision.integrator = (if on_gpu { "gpu" } else if buffers.is_some() { "wavefront" } else { "path" }).into();

    // Carry on from where an earlier render of the same scene stopped, if there was one.
    let state_path = rendercontext::state_path_for(&rc.output_filename);
    if let Some(t) = args.noise_threshold {
        s.render.adaptive = (t > 0.).then(|| scene::Adaptive {
            threshold: t,
            min_samples: s.render.adaptive.map_or(scene::Adaptive::DEFAULT_MIN_SAMPLES, |a| a.min_samples),
        });
    }
    // Each integrator's samples are its own, so a saved state is one's.
    let key = rendercontext::render_key(&contents, (args.width, args.height, args.seed))
        ^ if on_gpu { 0x6770_7500_0000_0001 } else if buffers.is_some() { 0x7761_7665_6672_6f6e } else { 0 };
    // The objects the host answers for, by name, for what it did for each.
    if let Some(f) = &flat {
        run.object_names = kernels::host::object_names(&s, f);
    }
    rc.run = run;
    if !args.fresh && std::path::Path::new(&state_path).exists() {
        match rc.load_state(&state_path, key) {
            Ok(()) => println!("- Resuming from {:.1} samples per pixel on average, {} at least ({})",
                rc.resumed_samples as f64 / (rc.width * rc.height) as f64, rc.min_samples(), state_path),
            Err(e) => println!("- Starting afresh: {}", e),
        }
    }
    if let Some(n) = args.samples {
        s.render.supersamples = n;
    }
    if let Some(n) = args.add_samples {
        // Adaptively sampled pixels differ: more than the most any has.
        let have = if s.render.adaptive.is_some() { rc.max_samples() } else { rc.min_samples() };
        s.render.supersamples = have + n;
    }
    match s.render.adaptive {
        Some(a) => println!("- Output: {}x{} @ {} to {} samples, to a noise of {} -> {}", s.image.width, s.image.height,
            a.min_samples.min(s.render.samples_per_pixel()), s.render.samples_per_pixel(), a.threshold, rc.output_filename),
        None => println!("- Output: {}x{} @ {} samples -> {}", s.image.width, s.image.height, s.render.samples_per_pixel(), rc.output_filename),
    }
    println!("- Scene Objects: {}, Primitives: {} ", s.objects.len(),  s.objects.primitives_len()); 
    let peak = peak_memory().map_or(String::new(), |b| format!(", peak memory {:.0} MiB", b as f64 / (1024. * 1024.)));
    println!("- Built in {:.2} s{}", build_start.elapsed().as_secs_f64(), peak);

    // Fail before rendering if the output can't be written in this format.
    let writer = output::writer_for(&rc.output_filename, s.output.clone()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if let Some(path) = &args.depth {
        let raw = path.to_ascii_lowercase().ends_with(".exr");
        let opts = output::OutputOpts { transfer: output::Transfer::Linear, ..output::OutputOpts::default() };
        let img = depth::depth_image(&depth::depth(&s), s.image.width, s.image.height, raw);
        if let Err(e) = output::writer_for(path, opts).and_then(|w| w.write(&img, path)) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        println!("- Depth -> {}", path);
    }

    if let Some(runs) = args.variance {
        measure_variance(&mut s, runs, &rc.output_filename, writer.as_ref());
        return;
    }

    let kernels = flat.as_ref().zip(buffers).map(|(f, b)| wavefront::integrator::Kernels::with(&s, f, b));
    let render_start = Instant::now();
    rc.run.phases.push(report::Phase { name: "render".into(), seconds: 0., footprint: None });
    report::enter("render");
    let rcmtx = Mutex::new(rc);

    println!("{}",
        style("# 2. - Rendering ").bold().cyan()
    );
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {prefix}{msg} [{bar:.cyan/blue}]").unwrap().progress_chars("#>-"));

    // Progressive output and saved states are throttled, and encoded or written outside the
    // lock so they don't serialise the render.
    let progressive_interval = Duration::from_secs(1);
    let last_progressive = Mutex::new(Instant::now());
    let last_state = Mutex::new(Instant::now());

    #[cfg(feature = "gpu")]
    let after_pass = |rc: &RenderContext| {
        pb.set_position(rc.progress_percentage(&s) as u64);
        pb.set_message(rc.progress(&s));
        pb.tick();
        let mut last = last_progressive.lock().unwrap();
        if rc.progressive_render && last.elapsed() >= progressive_interval {
            *last = Instant::now();
            if let Err(e) = writer.write(&s.output.finish(&rc.snapshot()), &rc.output_filename) {
                eprintln!("{}", e);
            }
        }
        let mut last = last_state.lock().unwrap();
        if last.elapsed() >= STATE_INTERVAL {
            *last = Instant::now();
            save_state(&state_path, &rc.encode_state(key));
        }
    };
    if on_gpu {
        #[cfg(feature = "gpu")]
        gpu.as_mut().unwrap().render(&s, flat.as_ref().unwrap(), &rcmtx, after_pass);
    } else {
    rendercontext::render(&s, &rcmtx, kernels.as_ref(),
        |rc| {
            pb.set_position(rc.progress_percentage(&s) as u64);
            pb.set_message(rc.progress(&s));
            pb.tick();

            let mut last = last_progressive.lock().unwrap();
            let image = if rc.progressive_render && last.elapsed() >= progressive_interval {
                *last = Instant::now();
                Some((rc.snapshot(), rc.output_filename.clone()))
            } else {
                None
            };
            let mut last = last_state.lock().unwrap();
            let state = if last.elapsed() >= STATE_INTERVAL {
                *last = Instant::now();
                Some(rc.encode_state(key))
            } else {
                None
            };
            if image.is_some() || state.is_some() { Some((image, state)) } else { None }
        },
        |(image, state)| {
            if let Some((img, path)) = image
                && let Err(e) = writer.write(&s.output.finish(&img), &path) {
                eprintln!("{}", e);
            }
            if let Some(bytes) = state {
                save_state(&state_path, &bytes);
            }
        });
    }
    pb.finish();

    let mut rc = rcmtx.lock().unwrap();
    if let Some(render) = rc.run.phases.iter_mut().rfind(|p| p.name == "render") {
        render.seconds = render_start.elapsed().as_secs_f64();
        render.footprint = report::read().map(|r| r.footprint);
    }
    if rc.samples_rendered > 0 {
        save_state(&state_path, &rc.encode_state(key));
    }
    if s.render.adaptive.is_some() {
        println!("- {:.1} samples per pixel on average, from {} to {}; more can be added with --add-samples",
            rc.mean_samples(), rc.min_samples(), rc.max_samples());
    } else {
        println!("- {} samples per pixel; more can be added with --add-samples", rc.min_samples());
    }
    if let Some(path) = &args.sample_map {
        let opts = output::OutputOpts { transfer: output::Transfer::Linear, ..output::OutputOpts::default() };
        if let Err(e) = output::writer_for(path, opts).and_then(|w| w.write(&rc.sample_map(), path)) {
            eprintln!("{}", e);
        } else {
            println!("- Samples per pixel -> {}", path);
        }
    }
    if let Some(path) = &args.time_map {
        write_time_map(path, &rc, on_gpu);
    }
    //wireframe::wireframe(&s, &mut rc);
    let img = s.output.finish(&rc.snapshot());
    if let Err(e) = writer.write(&img, &rc.output_filename) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    if let Some(reference) = &args.compare {
        compare_with(&img, reference, &rc.output_filename, &s.output, args.max_flip);
    }
    paint::poor_mans(&img, &s.output);
    rc.print_stats();
}

/// Write the render's profile, each pixel's time (see `time_map`), and summarise it.
fn write_time_map(path: &str, rc: &RenderContext, wavefront: bool) {
    if wavefront {
        println!("- The GPU renders paths in batches, not by pixel: no time map");
        return;
    }
    let times = rc.times();
    if times.iter().all(|&t| t == 0.) {
        println!("- Nothing was rendered this run (all from the saved state): no time map; use --fresh");
        return;
    }
    let sum = time_map::summary(times);
    let raw = path.to_ascii_lowercase().ends_with(".exr");
    let opts = output::OutputOpts { transfer: output::Transfer::Linear, ..output::OutputOpts::default() };
    let img = time_map::image(times, rc.width, rc.height, raw);
    if let Err(e) = output::writer_for(path, opts).and_then(|w| w.write(&img, path)) {
        eprintln!("{}", e);
        return;
    }
    let us = |t: f64| t * 1e6;
    println!("- Time per pixel -> {}: mean {:.1} µs, slowest {:.1} µs; {:.2} s on all threads{}",
        path, us(sum.mean), us(sum.max), sum.total,
        if raw { String::new() } else { format!("; brightest is {:.1} µs or more", us(sum.scale)) });
    if rc.resumed_samples > 0 {
        println!("  (only the samples rendered this run are timed, not those resumed from the saved state)");
    }
}

/// Render the scene `runs` times with successive seeds and report its noise (see `variance`).
fn measure_variance(s: &mut scene::Scene, runs: usize, output_filename: &str, writer: &dyn output::ImageWriter) {
    if runs < 2 {
        eprintln!("--variance needs at least two renders");
        std::process::exit(1);
    }
    let base = s.render.seed;
    let mut renders = Vec::with_capacity(runs);
    let mut seconds = 0.;
    for i in 0..runs {
        s.render.seed = base.wrapping_add(i as u64);
        let rc = Mutex::new(RenderContext::new(s.image.width, s.image.height, false, output_filename));
        let start = Instant::now();
        rendercontext::render(s, &rc, None, |_| None::<()>, |_| ());
        let t = start.elapsed().as_secs_f64();
        seconds += t;
        println!("- Render {} of {} (seed {}): {:.2} s", i + 1, runs, s.render.seed, t);
        renders.push(rc.lock().unwrap().snapshot());
    }
    let v = variance::Variance::of(&renders);
    let mean_time = seconds / runs as f64;
    if let Err(e) = writer.write(&s.output.finish(&v.mean), output_filename) {
        eprintln!("{}", e);
    }
    let map_path = std::path::Path::new(output_filename).with_extension("variance.png");
    let map_path = map_path.to_string_lossy();
    let opts = output::OutputOpts { transfer: output::Transfer::Linear, ..output::OutputOpts::default() };
    if let Err(e) = output::writer_for(&map_path, opts).and_then(|w| w.write(&v.map(), &map_path)) {
        eprintln!("{}", e);
    }
    println!("{}", style("# Variance").bold().cyan());
    println!("- {} renders at {} samples per pixel, {:.2} s each", runs, s.render.samples_per_pixel(), mean_time);
    println!("- relMSE of one render: mean {:.5}, median {:.5}", v.rel_mse, v.median_rel_mse);
    println!("- Efficiency 1 / (relMSE x time): {:.4}", variance::efficiency(v.rel_mse, mean_time));
    println!("- Mean of the renders: {}; relative standard deviation: {}", output_filename, map_path);
}

/// Compare the finished render with a reference image, reporting how close they are.
fn compare_with(img: &output::LinearImage, reference: &str, output_filename: &str, opts: &output::OutputOpts, max_flip: Option<f64>) {
    let fail = |e: String| -> ! { eprintln!("{}", e); std::process::exit(1) };
    let reference_img = output::read_reference(reference, opts).unwrap_or_else(|e| fail(e));
    let c = compare::compare(img, &reference_img, opts).unwrap_or_else(|e| fail(e));
    let map_path = std::path::Path::new(output_filename).with_extension("flip.png");
    if let Err(e) = c.error_map.save(&map_path) {
        eprintln!("Failed to write {}: {}", map_path.display(), e);
    }
    // The reference at our size, encoded exactly as our PNG output is, to view side by side.
    let reference_path = std::path::Path::new(output_filename).with_extension("reference.png");
    let reference_path = reference_path.to_string_lossy();
    if let Err(e) = output::ImageWriter::write(&output::Png { opts: opts.clone() }, &c.reference, &reference_path) {
        eprintln!("{}", e);
    }
    println!("{}", style("# Comparison").bold().cyan());
    println!("- Reference: {}", reference);
    println!("- Exposure fit: {:.4} (1 means the same brightness)", c.exposure);
    println!("- RMSE: {:.5}, after exposure fit: {:.5}", c.rmse, c.rmse_fitted);
    println!("- Mean FLIP error: {:.4} (error map: {})", c.flip_mean, map_path.display());
    println!("- Reference at this size: {}", reference_path);
    if let Some(max) = max_flip {
        if c.flip_mean > max {
            eprintln!("Mean FLIP error {:.4} exceeds {:.4}", c.flip_mean, max);
            std::process::exit(2);
        }
    }
}
