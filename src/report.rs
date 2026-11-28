//! A render's report: what each run that added samples to it did, kept with its samples in
//! the saved state (`<output>.accum`, `RenderContext::encode_state`), so that how every sample
//! was made can be looked into after the render, rather than read off a terminal at the time.
//!
//! A run is one invocation of the renderer on a render: a fresh one, or one resumed from the
//! saved state, which keeps the runs before it and adds its own (plan.md, section 16). Fields
//! are added to `Run` as the renderer records more; each takes a default, so a report written
//! before a field existed still reads.

/// The runs that made a render's samples, oldest first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub runs: Vec<Run>,
}

/// One run of the renderer on a render.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Run {
    /// When it started, in seconds since 1970 (UTC).
    pub started: u64,
    /// How long it had run when its state was last saved, in seconds: set-up and rendering.
    pub seconds: f64,
    /// The render's samples, summed over its pixels, when the run began (those resumed) and
    /// when its state was last saved.
    pub samples_before: u64,
    pub samples_after: u64,
    /// What it ran on and with (`Context`).
    pub context: Context,
    /// What rendered it, and why not the others.
    pub decision: Decision,
    /// Where its time went, in order: building the scene's objects, laying it out for the
    /// kernels, setting up the GPU, rendering.
    pub phases: Vec<Phase>,
    /// The render's stages, as `scheduler::Stats` counts them (`Stages`).
    pub stages: Stages,
    /// Rays traced this run.
    pub rays: u64,
    /// Its memory, and the machine's, over time (`Memory`).
    pub memory: Memory,
    /// What the host did for each object the kernels have no code for (`ObjectRecord`): where
    /// the kernels rendered (on the CPU or the GPU; the path tracer has no such division).
    pub objects: Vec<ObjectRecord>,
    /// The names of those objects, by their key (`kernels::host::object_names`): the host's
    /// to give, from the scene; not kept, as `objects` carries them.
    #[serde(skip)]
    pub object_names: Vec<(String, &'static str)>,
    /// When it started, to measure `seconds` from: this process's clock, not the report's.
    #[serde(skip)]
    pub began: Option<web_time::Instant>,
}

/// Bytes as a person reads them: "859 MB", "6.22 GB".
fn bytes(n: u64) -> String {
    let n = n as f64;
    if n >= 1e9 { format!("{:.2} GB", n / 1e9) } else if n >= 1e6 { format!("{:.0} MB", n / 1e6) } else { format!("{:.0} kB", n / 1e3) }
}

/// A count with its thousands apart: 8,708,701.
fn count(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Seconds since 1970 as a UTC date and time (by Hinnant's days-to-civil algorithm).
fn utc(seconds: u64) -> String {
    let (days, rest) = ((seconds / 86400) as i64, seconds % 86400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", year, month, day, rest / 3600, rest / 60 % 60, rest % 60)
}

impl Run {
    /// This run, run `n` of `of`, for `Report::summary`.
    fn summarise(&self, n: usize, of: usize, out: &mut String) {
        use std::fmt::Write;
        let _ = writeln!(out, "Run {} of {}: {}, {:.1} s; samples {} -> {}; {} rays", n, of, utc(self.started), self.seconds,
            count(self.samples_before), count(self.samples_after), count(self.rays));
        let c = &self.context;
        if !c.version.is_empty() {
            let commit = c.commit.as_ref().map_or(String::new(), |h| format!(" at {}{}", &h[..h.len().min(10)], if c.dirty == Some(true) { " (with changes)" } else { "" }));
            let _ = writeln!(out, "  rays {}{}; {} {}, {}, {} threads{}", c.version, commit, c.os, c.arch, c.cpu.as_deref().unwrap_or("?"), c.threads,
                c.memory.map_or(String::new(), |m| format!(", {}", bytes(m))));
            let _ = writeln!(out, "  {}", c.arguments.join(" "));
        }
        let d = &self.decision;
        if !d.integrator.is_empty() {
            let _ = writeln!(out, "  Rendered by: {} (asked: {})", d.integrator, d.asked);
            for r in &d.refused {
                let _ = writeln!(out, "    not {}: {}", r.integrator, r.reason);
            }
        }
        if let Some(g) = &d.gpu {
            let total: u64 = g.buffers.iter().map(|b| b.bytes).sum();
            let mut largest: Vec<&GpuBuffer> = g.buffers.iter().collect();
            largest.sort_by(|a, b| b.bytes.cmp(&a.bytes));
            let _ = writeln!(out, "  GPU: {}, {}, {} lanes; buffers {} ({})", g.adapter, if g.ray_queries { "ray queries" } else { "BVH" }, g.lanes, bytes(total),
                largest.iter().take(4).map(|b| format!("{} {}", b.name, bytes(b.bytes))).collect::<Vec<_>>().join(", "));
        }
        if !self.phases.is_empty() {
            let _ = writeln!(out, "  Phases:");
            for p in &self.phases {
                // A part under its whole: "gpu set-up: the acceleration structures: built" as
                // "the acceleration structures: built" under "gpu set-up".
                let (depth, name) = match p.name.split_once(": ") {
                    Some((_, part)) => (1, part),
                    None => (0, p.name.as_str()),
                };
                let _ = writeln!(out, "    {:indent$}{:<width$} {:>8.2} s{}", "", name, p.seconds,
                    p.footprint.map_or(String::new(), |f| format!("  {:>9}", bytes(f))), indent = 2 * depth, width = 44 - 2 * depth.min(10));
            }
        }
        let samples = &self.memory.samples;
        if let (Some(first), Some(last)) = (samples.first(), samples.last()) {
            let peak = samples.iter().map(|s| s.reading.peak.max(s.reading.footprint)).max().unwrap_or(0);
            let least = samples.iter().map(|s| s.reading.free).min().unwrap_or(0);
            let _ = writeln!(out, "  Memory: {} free at the start, {} at the least; peak footprint {}; {} pages swapped out and {} in meanwhile; {} page-ins ({} readings, {:.2} s apart)",
                bytes(first.reading.free), bytes(least), bytes(peak), count(last.reading.swapouts.saturating_sub(first.reading.swapouts)),
                count(last.reading.swapins.saturating_sub(first.reading.swapins)), count(last.reading.pageins.saturating_sub(first.reading.pageins)),
                samples.len(), self.memory.interval);
        }
        if !self.stages.stages.is_empty() {
            let _ = writeln!(out, "  Stages ({} passes{}):", self.stages.passes,
                if self.stages.device_seconds > 0. { format!(", {:.2} s on the device", self.stages.device_seconds) } else { String::new() });
            for st in &self.stages.stages {
                let _ = writeln!(out, "    {:<12} {:>14} in {:>9.2} s", st.stage, count(st.items), st.seconds);
            }
        }
        if !self.objects.is_empty() {
            let _ = writeln!(out, "  The host's work, by object (seconds summed over threads):");
            let mut objects: Vec<&ObjectRecord> = self.objects.iter().collect();
            objects.sort_by(|a, b| (b.shade_seconds + b.intersect_seconds + b.shadow_seconds).total_cmp(&(a.shade_seconds + a.intersect_seconds + a.shadow_seconds)));
            for o in objects {
                let mut parts = Vec::new();
                if o.vertices > 0 {
                    parts.push(format!("{} vertices shaded in {:.2} s", count(o.vertices), o.shade_seconds));
                }
                if o.deferred > 0 {
                    parts.push(format!("{} of their shadow rays traced by the kernels", count(o.deferred)));
                }
                if o.rays > 0 {
                    parts.push(format!("{} rays ({} hits) in {:.2} s, {:.2} us each", count(o.rays), count(o.hits), o.intersect_seconds, 1e6 * o.intersect_seconds / o.rays as f64));
                }
                if o.shadow_rays > 0 {
                    parts.push(format!("{} shadow rays in {:.2} s", count(o.shadow_rays), o.shadow_seconds));
                }
                let _ = writeln!(out, "    {} ({}): {}", o.name, o.role, parts.join("; "));
            }
        }
    }

    /// A run beginning now.
    pub fn starting() -> Run {
        let started = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        Run { started, began: Some(web_time::Instant::now()), ..Run::default() }
    }
}

/// The program and machine a run was made with: what its numbers can be compared across. The
/// library knows its own version; the rest is the host's to fill in (the command line reads
/// the machine; a browser doesn't say).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Context {
    /// The renderer's version (its crate's).
    pub version: String,
    /// The commit of the renderer's sources, and whether they had changes not committed, as
    /// the repository the program was run from stood at the time (not proof of what it was
    /// built from).
    pub commit: Option<String>,
    pub dirty: Option<bool>,
    /// The program's command line.
    pub arguments: Vec<String>,
    pub os: String,
    pub arch: String,
    /// The processor, as the system names it.
    pub cpu: Option<String>,
    /// The threads rendering.
    pub threads: usize,
    /// The machine's memory, bytes.
    pub memory: Option<u64>,
}

/// What rendered the run, as `main` chose: the first of the GPU, the kernels on the CPU and the
/// path tracer that could, unless the scene or the command line asked for one.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Decision {
    /// What was asked for: "auto", "gpu", "wavefront" or "path".
    pub asked: String,
    /// What rendered: "gpu", "wavefront" (the kernels on the CPU) or "path" (the path tracer).
    pub integrator: String,
    /// Each that was tried and couldn't, and why.
    pub refused: Vec<Refusal>,
    /// The GPU, where it rendered.
    pub gpu: Option<Gpu>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Refusal {
    pub integrator: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Gpu {
    /// The adapter, as wgpu names it.
    pub adapter: String,
    /// Whether rays were traced by the hardware's ray queries, rather than the BVH.
    pub ray_queries: bool,
    /// Samples of each pixel in flight at once (`GpuRenderer::lanes`).
    pub lanes: usize,
    /// Its buffers, by name, and their sizes in bytes.
    pub buffers: Vec<GpuBuffer>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GpuBuffer {
    pub name: String,
    pub bytes: u64,
}

/// A part of a run, and how long it took. Names say what: "build: objects[3] (pebbles)",
/// "flatten", "gpu set-up: the shader", "render"; "a: b" is a part of "a".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Phase {
    pub name: String,
    pub seconds: f64,
    /// The process's footprint when it ended, bytes, where the host can read it (`Reading`).
    pub footprint: Option<u64>,
}

/// The process's memory and the machine's at a moment, as the host reads them (`set_probe`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Reading {
    /// The process's footprint, bytes: what it has in memory or swapped or compressed, the GPU's
    /// allocations included where memory is shared (Apple's `phys_footprint`; elsewhere its
    /// resident size).
    pub footprint: u64,
    /// The most it has had, bytes (on macOS between readings too: the transient of a build).
    pub peak: u64,
    /// Pages the process has read in from disk (faults that waited).
    pub pageins: u64,
    /// The machine's free memory, bytes (on Linux, what is available).
    pub free: u64,
    /// The machine's pages swapped in and out since it started.
    pub swapins: u64,
    pub swapouts: u64,
}

/// A reading during a run: when (seconds into it) and in what phase.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySample {
    pub t: f64,
    pub phase: String,
    #[serde(flatten)]
    pub reading: Reading,
}

/// A run's memory over time: readings `interval` seconds apart, at most `MEMORY_SAMPLES` of
/// them (a long run's are thinned, every other dropped and the interval doubled, as they fill),
/// and the first, at the run's start: what the machine had free then is what the run could use.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Memory {
    pub interval: f64,
    pub samples: Vec<MemorySample>,
}

/// The most readings a run keeps.
pub const MEMORY_SAMPLES: usize = 2048;
/// How often a run is read at first, seconds.
pub const MEMORY_INTERVAL: f64 = 0.25;

/// The render's stages (`scheduler::Stats`): what went through each, and in how long.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Stages {
    /// Each stage that saw any work: its name (`scheduler::Stage::name`), batches, items
    /// (paths or rays) and seconds (summed over threads on the CPU; by the device's clock on
    /// the GPU, where it has one).
    pub stages: Vec<StageRecord>,
    /// Passes recorded.
    pub passes: u64,
    /// Paths alive at the start of each round, summed over the passes.
    pub live: Vec<u64>,
    /// The device's time for whole blocks of passes (the GPU).
    pub device_seconds: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StageRecord {
    pub stage: String,
    pub batches: u64,
    pub items: u64,
    pub seconds: f64,
}

/// The host's work on one object (`scheduler::ObjectStats`): the vertices on its material it
/// shaded, and the rays it tested its shape against, each with the seconds it took (summed over
/// threads). What porting the object's material or shape to the kernels would save.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ObjectRecord {
    /// The object, by its place in the scene file and its kind of shape: "objects[2] (Stand)".
    pub name: String,
    /// "material": the host shades it; "shape": the host intersects it (and shades it).
    pub role: String,
    pub vertices: u64,
    pub shade_seconds: f64,
    pub rays: u64,
    pub hits: u64,
    pub intersect_seconds: f64,
    pub shadow_rays: u64,
    pub shadow_seconds: f64,
    /// Its vertices' shadow rays left to the kernels to trace.
    pub deferred: u64,
}

impl ObjectRecord {
    /// Each object with any work, by name.
    pub fn of(stats: &crate::scheduler::Stats, names: &[(String, &'static str)]) -> Vec<ObjectRecord> {
        stats.objects.iter().enumerate().filter(|(_, o)| **o != Default::default()).map(|(key, o)| {
            let (name, role) = names.get(key).cloned().unwrap_or_else(|| (format!("object {}", key), ""));
            ObjectRecord {
                name, role: role.into(), vertices: o.vertices, shade_seconds: o.shade_seconds, rays: o.rays, hits: o.hits,
                intersect_seconds: o.intersect_seconds, shadow_rays: o.shadow_rays, shadow_seconds: o.shadow_seconds,
                deferred: o.deferred,
            }
        }).collect()
    }
}

impl Stages {
    pub fn of(stats: &crate::scheduler::Stats) -> Stages {
        use crate::scheduler::Stage;
        Stages {
            stages: Stage::ALL.iter().map(|&s| (s, stats.stage(s))).filter(|(_, x)| x.batches > 0)
                .map(|(s, x)| StageRecord { stage: s.name().into(), batches: x.batches, items: x.items, seconds: x.seconds }).collect(),
            passes: stats.passes,
            live: stats.live.clone(),
            device_seconds: stats.device_seconds,
        }
    }
}

/// Phases recorded from where no run is at hand (the scene's objects, built inside its
/// parsing), for the host to take into its run. Off unless a host turns it on (`record`), so
/// a server or a page that builds scene after scene doesn't keep them all.
static RECORDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static PHASES: std::sync::Mutex<Vec<Phase>> = std::sync::Mutex::new(Vec::new());

/// Record phases from now on.
pub fn record() {
    RECORDING.store(true, std::sync::atomic::Ordering::Relaxed);
}

/// A phase took `seconds`, if phases are being recorded.
pub fn phase(name: impl Into<String>, seconds: f64) {
    if RECORDING.load(std::sync::atomic::Ordering::Relaxed) {
        let footprint = read().map(|r| r.footprint);
        PHASES.lock().unwrap().push(Phase { name: name.into(), seconds, footprint });
    }
}

/// How the host reads memory (`Reading`): the library can't, built for the browser too.
pub type Probe = fn() -> Option<Reading>;
static PROBE: std::sync::Mutex<Option<Probe>> = std::sync::Mutex::new(None);

/// Read memory with `probe` from now on.
pub fn set_probe(probe: Probe) {
    *PROBE.lock().unwrap() = Some(probe);
}

/// Memory now, if the host can read it.
pub fn read() -> Option<Reading> {
    let probe = *PROBE.lock().unwrap();
    probe.and_then(|p| p())
}

/// What the run is doing now, for the readings taken meanwhile.
static CURRENT: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// The run is now in phase `name`.
pub fn enter(name: impl Into<String>) {
    if RECORDING.load(std::sync::atomic::Ordering::Relaxed) {
        *CURRENT.lock().unwrap() = name.into();
    }
}

/// The readings so far, and when they began.
static MEMORY: std::sync::Mutex<Option<(web_time::Instant, Memory)>> = std::sync::Mutex::new(None);

/// Keep a reading, in the phase the run is in, thinning them if they are full.
pub fn log_memory(reading: Reading) {
    if !RECORDING.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let phase = CURRENT.lock().unwrap().clone();
    let mut memory = MEMORY.lock().unwrap();
    let (began, log) = memory.get_or_insert_with(|| (web_time::Instant::now(), Memory { interval: MEMORY_INTERVAL, samples: vec![] }));
    log.samples.push(MemorySample { t: began.elapsed().as_secs_f64(), phase, reading });
    thin(log);
}

/// Every other reading dropped, keeping the first (the run's start), once there are
/// `MEMORY_SAMPLES`; the interval doubles to match.
fn thin(log: &mut Memory) {
    if log.samples.len() >= MEMORY_SAMPLES {
        let mut i = 0;
        log.samples.retain(|_| { i += 1; i % 2 == 1 });
        log.interval *= 2.;
    }
}

/// How long to wait before the next reading, seconds.
pub fn memory_interval() -> f64 {
    MEMORY.lock().unwrap().as_ref().map_or(MEMORY_INTERVAL, |(_, m)| m.interval)
}

/// The readings so far.
pub fn memory_so_far() -> Memory {
    MEMORY.lock().unwrap().as_ref().map(|(_, m)| m.clone()).unwrap_or_default()
}

/// The phases recorded since last taken.
pub fn take_phases() -> Vec<Phase> {
    std::mem::take(&mut *PHASES.lock().unwrap())
}

impl Report {
    /// The report for a person to read: each run, what it was made with and how, where its
    /// time and memory went.
    pub fn summary(&self) -> String {
        if self.runs.is_empty() {
            return "No runs recorded (the render was saved before reports were kept).\n".into();
        }
        let mut out = String::new();
        for (i, run) in self.runs.iter().enumerate() {
            run.summarise(i + 1, self.runs.len(), &mut out);
        }
        out
    }

    /// As bytes for the saved state: JSON.
    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("a report is plain data")
    }

    /// From the saved state's bytes.
    pub fn decode(bytes: &[u8]) -> Result<Report, String> {
        serde_json::from_slice(bytes).map_err(|e| format!("its report is unreadable ({})", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_report_reads_back_as_written() {
        let mut second = Run { started: 1_795_000_100, seconds: 3.25, samples_before: 384, samples_after: 768, rays: 99, ..Run::default() };
        second.decision = Decision {
            asked: "auto".into(), integrator: "wavefront".into(),
            refused: vec![Refusal { integrator: "gpu".into(), reason: "shapes the kernels can't intersect yet".into() }], gpu: None,
        };
        second.phases = vec![Phase { name: "build pebbles".into(), seconds: 1.5, footprint: Some(1 << 30) }, Phase { name: "render".into(), seconds: 2., footprint: None }];
        second.stages = Stages { stages: vec![StageRecord { stage: "intersect".into(), batches: 3, items: 300, seconds: 0.25 }], passes: 2, live: vec![100, 40], device_seconds: 0. };
        second.context.arguments = vec!["rays".into(), "scene.json".into()];
        let report = Report { runs: vec![
            Run { started: 1_795_000_000, seconds: 12.5, samples_before: 0, samples_after: 384, ..Run::default() },
            second,
        ] };
        assert_eq!(Report::decode(&report.encode()), Ok(report));
    }

    #[test]
    fn test_phases_are_kept_only_when_recording() {
        // (Recording is global and stays on: other tests' scenes may add theirs meanwhile.)
        phase("before recording", 1.);
        record();
        phase("while recording", 2.);
        let taken = take_phases();
        assert!(taken.iter().any(|p| p.name == "while recording" && p.seconds == 2.));
        assert!(!taken.iter().any(|p| p.name == "before recording"));
    }

    #[test]
    fn test_readings_are_thinned_to_a_bound() {
        let mut log = Memory { interval: MEMORY_INTERVAL, samples: vec![] };
        // (Readings a step apart however thinned, unlike the sampler's, which waits the
        // interval: 1.5 times the bound thins twice.)
        for k in 0 .. 3 * MEMORY_SAMPLES / 2 {
            log.samples.push(MemorySample { t: k as f64, ..MemorySample::default() });
            thin(&mut log);
            assert!(log.samples.len() < MEMORY_SAMPLES);
        }
        // Twice thinned: every fourth reading, from the first, four times as far apart.
        assert_eq!(log.interval, 4. * MEMORY_INTERVAL);
        assert_eq!(log.samples[0].t, 0.);
        assert!(log.samples.windows(2).take(MEMORY_SAMPLES / 4).all(|w| w[1].t - w[0].t == 4.));
    }

    #[test]
    fn test_a_reading_reads_back_flat() {
        // A sample's reading is written beside its time and phase, not nested.
        let sample = MemorySample { t: 1.5, phase: "render".into(), reading: Reading { footprint: 7, peak: 9, pageins: 1, free: 3, swapins: 0, swapouts: 2 } };
        let json = serde_json::to_value(&sample).unwrap();
        assert_eq!(json["footprint"], 7);
        assert_eq!(serde_json::from_value::<MemorySample>(json).unwrap(), sample);
    }

    #[test]
    fn test_objects_are_named_by_their_keys() {
        use crate::scheduler::Stats;
        let mut stats = Stats::default();
        stats.object(2).rays += 10;
        stats.object(2).hits += 4;
        stats.object(0).vertices += 3;
        let names = vec![("objects[4] (Mesh)".to_string(), "material"), ("objects[5] (Sphere)".to_string(), "material"), ("objects[1] (Stand)".to_string(), "shape")];
        let records = ObjectRecord::of(&stats, &names);
        // Those with work only, named by key.
        assert_eq!(records.iter().map(|r| (r.name.as_str(), r.role.as_str(), r.vertices, r.rays, r.hits)).collect::<Vec<_>>(),
            vec![("objects[4] (Mesh)", "material", 3, 0, 0), ("objects[1] (Stand)", "shape", 0, 10, 4)]);
        // Merged across threads' and batches' stats, by key.
        let mut other = Stats::default();
        other.object(2).rays += 5;
        stats.merge(&other);
        assert_eq!(stats.objects[2].rays, 15);
    }

    #[test]
    fn test_dates_are_utc() {
        assert_eq!(utc(0), "1970-01-01 00:00:00 UTC");
        assert_eq!(utc(951_782_400), "2000-02-29 00:00:00 UTC");
        assert_eq!(utc(1_795_692_112), "2026-11-26 11:21:52 UTC");
        assert_eq!(count(8_708_701), "8,708,701");
        assert_eq!(count(999), "999");
    }

    #[test]
    fn test_a_summary_says_what_a_run_did() {
        let mut run = Run { started: 0, seconds: 30.1, samples_before: 0, samples_after: 1_036_800, rays: 5, ..Run::default() };
        run.context = Context { version: "0.0.1".into(), commit: Some("ed2355b622f1".into()), dirty: Some(true), threads: 8, ..Context::default() };
        run.decision = Decision { asked: "auto".into(), integrator: "wavefront".into(),
            refused: vec![Refusal { integrator: "gpu".into(), reason: "shapes the kernels can't intersect yet".into() }], gpu: None };
        run.phases = vec![Phase { name: "build".into(), seconds: 25.6, footprint: Some(910_000_000) },
                          Phase { name: "build: objects[0] (landscape)".into(), seconds: 25., footprint: None }];
        run.objects = vec![ObjectRecord { name: "objects[2] (Stand)".into(), role: "shape".into(), rays: 1000, hits: 10, intersect_seconds: 0.005, ..ObjectRecord::default() }];
        let read = |r: Reading| MemorySample { reading: r, ..MemorySample::default() };
        run.memory = Memory { interval: 0.25, samples: vec![
            read(Reading { free: 2_240_000_000, swapouts: 100, ..Reading::default() }),
            read(Reading { footprint: 8_000_000_000, peak: 8_320_000_000, free: 0, swapouts: 274_397, ..Reading::default() })] };
        let text = Report { runs: vec![run] }.summary();
        for wanted in ["Run 1 of 1: 1970-01-01 00:00:00 UTC, 30.1 s; samples 0 -> 1,036,800", "at ed2355b622 (with changes)",
                       "Rendered by: wavefront (asked: auto)", "not gpu: shapes the kernels can't intersect yet",
                       "    build ", "      objects[0] (landscape)", "910 MB", "2.24 GB free at the start", "peak footprint 8.32 GB",
                       "274,297 pages swapped out", "objects[2] (Stand) (shape): 1,000 rays (10 hits) in 0.01 s, 5.00 us each"] {
            assert!(text.contains(wanted), "{:?} not in:\n{}", wanted, text);
        }
        assert!(Report::default().summary().starts_with("No runs"));
    }

    #[test]
    fn test_stages_are_the_schedules() {
        use crate::scheduler::{Stage, Stats};
        let mut stats = Stats::default();
        stats.record(Stage::Intersect, 100, 0.5);
        stats.record(Stage::Intersect, 50, 0.25);
        stats.record(Stage::Host, 7, 1.);
        stats.round(0, 150);
        let stages = Stages::of(&stats);
        // Only the stages that saw work, in the schedule's order.
        assert_eq!(stages.stages, vec![
            StageRecord { stage: "intersect".into(), batches: 2, items: 150, seconds: 0.75 },
            StageRecord { stage: "host".into(), batches: 1, items: 7, seconds: 1. },
        ]);
        assert_eq!(stages.live, vec![150]);
    }

    #[test]
    fn test_a_run_missing_fields_takes_their_defaults() {
        // A report written before a field existed.
        let old = br#"{"runs": [{"started": 5, "seconds": 1.5}]}"#;
        let report = Report::decode(old).unwrap();
        assert_eq!(report.runs, vec![Run { started: 5, seconds: 1.5, ..Run::default() }]);
        // And a phase or a stage missing some.
        let report = Report::decode(br#"{"runs": [{"phases": [{"name": "render"}], "stages": {"passes": 3}}]}"#).unwrap();
        assert_eq!(report.runs[0].phases, vec![Phase { name: "render".into(), seconds: 0., footprint: None }]);
        assert_eq!(report.runs[0].stages.passes, 3);
    }
}
