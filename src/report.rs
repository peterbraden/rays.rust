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
    /// When it started, to measure `seconds` from: this process's clock, not the report's.
    #[serde(skip)]
    pub began: Option<web_time::Instant>,
}

impl Run {
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
