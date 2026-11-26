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
}

/// A part of a run, and how long it took. Names say what: "build pebbles", "flatten",
/// "gpu: the shader", "render".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Phase {
    pub name: String,
    pub seconds: f64,
}

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
        PHASES.lock().unwrap().push(Phase { name: name.into(), seconds });
    }
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
        second.phases = vec![Phase { name: "build pebbles".into(), seconds: 1.5 }, Phase { name: "render".into(), seconds: 2. }];
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
        assert!(taken.contains(&Phase { name: "while recording".into(), seconds: 2. }));
        assert!(!taken.iter().any(|p| p.name == "before recording"));
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
        assert_eq!(report.runs[0].phases, vec![Phase { name: "render".into(), seconds: 0. }]);
        assert_eq!(report.runs[0].stages.passes, 3);
    }
}
