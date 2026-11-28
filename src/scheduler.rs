//! The scheduler: what work is done next, where, and in what order.
//!
//! A renderer is a great many small jobs (trace this ray, shade this hit, add this sample), and
//! most of what separates a fast one from a slow one is not how each job is done but which
//! jobs are done together. This module is where those decisions are meant to live. Today it
//! holds one of them, the first; the rest of this note is the design it is growing towards,
//! and the reading behind it. The plan and its progress are in `plan.md`, section 13.
//!
//! # Why a scheduler can exist here
//!
//! Sample k of pixel p draws its random numbers from a stream of its own, seeded by (p, k)
//! and nothing else: `rendercontext::render_pixel` on the CPU, `pathtrace::path_seed` on the
//! GPU. So what a sample contributes doesn't depend on when it is traced, on which thread or
//! machine, or alongside which other samples. A scheduler may therefore reorder, batch and
//! move work as it likes without changing the picture, and every policy can be tested the
//! same way: two policies, the same pixels. Everything below leans on that, and anything
//! added here must keep it.
//!
//! # The model
//!
//! - A work item is a path: a pixel, a sample index, and its state so far (on the GPU,
//!   `pathtrace::PathState`).
//! - A queue holds the paths waiting for one stage (generate, intersect, shade, shadow, sky,
//!   accumulate) on one resource: what the batch must have at hand to run (a mesh's tree, a
//!   texture, a medium's grid, a kernel).
//! - An executor drains queues: a GPU kernel, the CPU (for what the GPU can't do), in time a
//!   render server. Each has a capacity, and a cost to reach it (a readback, a network hop).
//! - A policy chooses the next queue, how much of it, the executor, and the order within it.
//! - The film is what is known of the picture so far: for each pixel the sum of its samples,
//!   the sum of their squares, and how many there are. A pixel is complete when it has the
//!   samples asked for, or, with adaptive sampling, when its noise is low enough.
//!
//! Decisions are made at two rates, and the design has to respect which is which. Within a
//! pass, which paths are in which queue is decided on the GPU, from data: the host can't see
//! a queue's count without reading it back, which in a browser arrives a frame late, so the
//! queues size their own dispatches (`pathtrace::queue`). Between passes, which pixels take
//! which samples, what goes to the CPU or to a server, and what is loaded, are the host's to
//! decide, here.
//!
//! # What is here now: pixel completion
//!
//! `plan` answers the first question a pass asks: which pixels take samples next, and how
//! many each. It is the one place that is decided, for the recursive tracer
//! (`RenderContext::next_pass`, a region at a time) and for the GPU (`GpuRenderer::render`,
//! the image at a time), which before had no notion of a finished pixel: every pixel took
//! every pass, so adaptive sampling did nothing there. A pixel's next sample is always the
//! one after its last (its count is its next index), so pixels that stop early leave no gaps,
//! a render can be resumed from its film alone, and more samples can be asked for later.
//!
//! On the GPU this is the generation stage of the papers below, in its simplest form. Laine,
//! Karras and Aila keep a pool of paths and refill the slots of those that end; van Antwerpen
//! compacts the survivors and regenerates into the gaps. Here a pass's first queue is filled
//! with just the pixels the plan names (`GpuRenderer::dispatch_samples`), each with its own
//! sample index, and the pixels it doesn't name cost nothing. What this doesn't yet do is
//! refill within a pass: a path that ends early leaves its slot idle until the next pass
//! (step 4 of the plan).
//!
//! # What is here now: a vocabulary, and counters
//!
//! Two renderers here work in stages over batches, the CPU's wavefront integrator
//! (`wavefront::integrator`) and the GPU's kernels (`gpu`, `pathtrace`), and before this they
//! shared no words for it. `Stage` names the stages both have; a batch is a number of paths
//! through one stage; `Stats` counts, for each stage, the batches and paths that went through
//! it and, where it can be measured, the time, and for each round of a pass (a round takes
//! every live path one vertex further) how many paths were still alive. Both renderers record
//! into it and a render ends by reporting it (`Stats::report`). It decides nothing; it is
//! what the decisions to come will be judged by. Each of the papers below argues from
//! measurements of this kind: Laine and others from how many paths are in each stage and how
//! long each kernel runs, van Antwerpen from the share of threads with live paths, Meister
//! and others from the trace's time against the sort's.
//!
//! Time by stage on the GPU comes from the device's own clock, where it has one (wgpu's
//! timestamp queries, on Vulkan and Metal natively; not asked for in a browser yet): each
//! kernel runs in a compute pass of its own with a timestamp at each end (`gpu::Timers`).
//! Without one the host sees only a block of passes go in and an image come out, so that
//! time is reported whole (`Stats::device_seconds`), beside the host's own part. On the CPU a
//! stage's time is the sum over threads, not wall time.
//!
//! # What is here now: the CPU queue
//!
//! The first queue with an executor other than the kernels. A material that isn't data stays
//! in a flattened scene as `wavefront::flat::FlatMaterial::Host`, and a path at one is work
//! for the renderer's own code: in the CPU's integrator, the last of the shading stage's
//! sort (`integrator::host_bounce`); on the GPU, a third queue the kernels fill and the host
//! drains when a pass's rounds are done, shading each vertex and queueing the path again
//! (`gpu::host`, which has the policy's detail and what it costs). Both count it as
//! `Stage::Host`. The policy is fixed for now: drain after the rounds, every pass. What the
//! scheduler has to decide later is when (a read of the device is a synchronisation, in a
//! browser a frame's delay), and whether the device waits (it does: a pass can't end with
//! paths outstanding until paths carry their own sample, the next step).
//!
//! # What is here now: the wavefront's width
//!
//! The reading's way to keep a wavefront full is regeneration: a path that ends is replaced
//! at once by a new one, so no kernel runs over dead paths (van Antwerpen; Laine and others).
//! That was built here and measured, and set aside. The queues already make a round cost
//! only its live paths, so what regeneration could save is rounds, and with a pixel's samples
//! taken one after another (so that their sum is added in order, by one invocation, with no
//! race) a pass still takes as many rounds as its slowest pixel's paths have vertices:
//! demo.json, 64 samples in blocks of 8, took 652 rounds against 640, and a fifth longer for
//! the extra kernel and for asking the device whether it was done. Letting the slow pixels'
//! paths run on into the next block would have saved the rounds, at the cost of a film that
//! is never whole, on which adaptive sampling and resumed renders rest.
//!
//! What is here instead is width (`lanes`; `pathtrace::slots`): a pass carries several
//! samples of every pixel at once, each in a lane of the wavefront, and its rounds are those
//! of one sample. The same paths, the same sums in the same order (so the same pixels), an
//! eighth of the dispatches; the cost is memory, a path and its sky queries a lane, so the
//! width is what a budget has room for (`gpu::WAVEFRONT_BUDGET`: 8 lanes of a small image,
//! one at 1280 x 720). The two aren't exclusive: with a lane's samples regenerated and each
//! lane summed apart, a wavefront could be both wide and full. That is for when a
//! measurement asks for it.
//!
//! # What isn't here yet, and what the reading says about it
//!
//! Stage queues, and why. Laine, Karras and Aila's argument for a wavefront over one large
//! kernel is that a GPU runs a group of threads through the same instructions: when paths
//! that have hit different materials share a group, each waits while the others' code runs,
//! and a kernel that holds every material's code needs registers for all of it. Queues by
//! stage and by material give each kernel one job. The GPU path has stage kernels and queues
//! of live paths (plan.md, section 12); queues by material are still to come.
//!
//! The CPU's queue. Budge and others describe a path tracer as queues of tasks over a layer
//! that decides which processor each batch goes to, CPUs and GPUs together, and which data
//! must be in memory for it to run. That is the shape wanted here for what the GPU can't
//! shade or intersect: such paths wait in a queue the host reads back, runs, and returns
//! (step 3). The decision the scheduler owns is how often: every readback stalls the GPU's
//! paths or lets them run ahead of the CPU's.
//!
//! Order, for the cache's sake. Pharr, Kolb, Gershbein and Hanrahan reorder rays so that the
//! geometry and textures they need are those already in memory, choosing the next batch by
//! what it costs to bring in and how much work it unblocks. Navrátil and others treat
//! traversal itself as a scheduling problem, measured by the data moved from memory into the
//! cache. Eisenacher and others sort very large batches of rays, and then sort their hits by
//! mesh and face before shading, so that texture reads are coherent without a cache. Lee and
//! others queue work on the CPU so that vector lanes stay full and neighbouring work touches
//! the same data. All of it is a sort key and a choice of queue; none of it is free. Meister
//! and others measure reordering on recent GPUs: tracing got 1.3 to 2 times faster, but the
//! sorting's own cost was hard to win back. So order is a policy to measure here (step 5),
//! not a property to assume.
//!
//! Resources. The same papers' larger point is that the scheduler should know what each
//! batch needs. With queues keyed by resource, and a budget of what may be resident (on the
//! GPU, in a browser's memory, on a render server), the policy becomes: run the queues whose
//! data is at hand, and load for the fullest of the rest (step 6). That is also how work
//! would be shared between servers: by the assets each holds, not by layer number as
//! `web/app.js` shares it now.
//!
//! # Reading
//!
//! Each was checked to exist as cited; what is said of them above is from their abstracts
//! and from how later papers describe them, not from a close reading. Read them before
//! building on a claim.
//!
//! - Samuli Laine, Tero Karras and Timo Aila, "Megakernels Considered Harmful: Wavefront Path
//!   Tracing on GPUs", High-Performance Graphics 2013, doi:10.1145/2492045.2492060.
//! - Dietger van Antwerpen, "Improving SIMD Efficiency for Parallel Monte Carlo Light
//!   Transport on the GPU", High Performance Graphics 2011, doi:10.1145/2018323.2018330.
//! - Matt Pharr, Craig Kolb, Reid Gershbein and Pat Hanrahan, "Rendering Complex Scenes with
//!   Memory-Coherent Ray Tracing", SIGGRAPH 1997, doi:10.1145/258734.258791.
//! - Paul Navrátil, Donald Fussell, Calvin Lin and William Mark, "Dynamic Ray Scheduling to
//!   Improve Ray Coherence and Bandwidth Utilization", IEEE Symposium on Interactive Ray
//!   Tracing 2007, doi:10.1109/RT.2007.4342596.
//! - Brian Budge, Tony Bernardin, Jeff Stuart, Shubhabrata Sengupta, Kenneth Joy and John
//!   Owens, "Out-of-core Data Management for Path Tracing on Hybrid Resources", Computer
//!   Graphics Forum 28(2) (Eurographics 2009), doi:10.1111/j.1467-8659.2009.01378.x.
//! - Christian Eisenacher, Gregory Nichols, Andrew Selle and Brent Burley, "Sorted Deferred
//!   Shading for Production Path Tracing", Computer Graphics Forum 32(4) (EGSR 2013),
//!   doi:10.1111/cgf.12158.
//! - Mark Lee, Brian Green, Feng Xie and Eric Tabellion, "Vectorized Production Path
//!   Tracing", High Performance Graphics 2017.
//! - Daniel Meister, Jakub Bokšanský, Michael Guthe and Jiří Bittner, "On Ray Reordering
//!   Techniques for Faster GPU Ray Tracing", I3D 2020, doi:10.1145/3384382.3384534.
//! - Matt Pharr, Wenzel Jakob and Greg Humphreys, "Physically Based Rendering", 4th ed.,
//!   chapter 15 (the wavefront renderer `wavefront` and `gpu` follow).
use crate::adaptive::Region;
use crate::scene::Adaptive;

/// A stage of a wavefront: one kind of work, done for many paths together (PBR section 15.2;
/// Laine, Karras and Aila's kernels).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Camera rays: the paths a pass starts, for the pixels `plan` names.
    Generate,
    /// The nearest thing along each path's ray.
    Intersect,
    /// What a path that escaped, or hit a light, gathers there. The CPU's integrator has it as
    /// a stage; on the GPU it is part of `Shade`.
    Emission,
    /// The vertex at a hit: light sampling (its shadow rays queued, or on the GPU traced
    /// here) and the next ray.
    Shade,
    /// A vertex the stages have no code for (a material that isn't data,
    /// `wavefront::flat::FlatMaterial::Host`), shaded by the scene's own material on the CPU.
    Host,
    /// Shadow rays: whether each light sample arrives.
    Shadow,
    /// The sky's light, where asked for: escaping rays, sky samples, haze. A kernel of its own
    /// on the GPU; part of `Emission` and `Shade` on the CPU.
    Sky,
    /// Finished paths added to the film.
    Accumulate,
}

impl Stage {
    pub const ALL: [Stage; 8] = [Stage::Generate, Stage::Intersect, Stage::Emission, Stage::Shade, Stage::Host, Stage::Shadow, Stage::Sky, Stage::Accumulate];

    pub fn name(&self) -> &'static str {
        match self {
            Stage::Generate => "generate",
            Stage::Intersect => "intersect",
            Stage::Emission => "emission",
            Stage::Shade => "shade",
            Stage::Host => "host",
            Stage::Shadow => "shadow",
            Stage::Sky => "sky",
            Stage::Accumulate => "accumulate",
        }
    }
}

/// What has gone through one stage: batches, the paths (or rays) in them, and the time they
/// took where that is known (0 where it isn't).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StageStats {
    pub batches: u64,
    pub items: u64,
    pub seconds: f64,
}

/// The host's work on one object the kernels have no code for: vertices on its material that
/// the scene's own code shaded, and rays its shape was tested against by its own code (paths'
/// rays, `rays`, of which `hits` were nearest; and shadow rays from vertices the kernels shaded,
/// `shadow_rays`), each with its time. Counted per thread as it happens (`count_object`), so
/// that the threads needn't share a counter, and gathered into a batch's `Stats`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObjectStats {
    pub vertices: u64,
    pub shade_seconds: f64,
    pub rays: u64,
    pub hits: u64,
    pub intersect_seconds: f64,
    pub shadow_rays: u64,
    pub shadow_seconds: f64,
    /// Its vertices' shadow rays left to the kernels to trace (`pathtrace::PATH_DEFERRED`).
    pub deferred: u64,
}

impl ObjectStats {
    pub fn add(&mut self, o: &ObjectStats) {
        self.vertices += o.vertices;
        self.shade_seconds += o.shade_seconds;
        self.rays += o.rays;
        self.hits += o.hits;
        self.intersect_seconds += o.intersect_seconds;
        self.shadow_rays += o.shadow_rays;
        self.shadow_seconds += o.shadow_seconds;
        self.deferred += o.deferred;
    }
}

thread_local! {
    static OBJECTS: std::cell::RefCell<Vec<ObjectStats>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Count work on object `key` on this thread.
pub fn count_object(key: usize, f: impl FnOnce(&mut ObjectStats)) {
    OBJECTS.with(|o| {
        let mut o = o.borrow_mut();
        if o.len() <= key {
            o.resize(key + 1, ObjectStats::default());
        }
        f(&mut o[key]);
    });
}

/// The work on objects this thread has counted since last taken.
pub fn take_objects() -> Vec<ObjectStats> {
    OBJECTS.with(|o| std::mem::take(&mut *o.borrow_mut()))
}

/// What a render's stages did, for judging how work is scheduled. Recorded by the wavefront
/// integrator and by the GPU's host, and added up (`merge`) across threads and passes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stats {
    stages: [StageStats; 8],
    /// Passes (a sample of every planned pixel) recorded.
    pub passes: u64,
    /// Paths alive at the start of each round of a pass, added up over the passes: the first
    /// is every path generated, each later one those that went on. (On the GPU, of a pass's
    /// first run of rounds: a path waiting for the host isn't alive, and the rounds it takes
    /// after the host has shaded it count in the stages' items only.) How quickly it falls is
    /// how much a queue of live paths saves over dispatching every pixel every round.
    pub live: Vec<u64>,
    /// Time the device took for whole blocks of passes, where stages can't be timed apart
    /// (the GPU: from the first pass queued to the image read back).
    pub device_seconds: f64,
    /// The host's work on each object the kernels have no code for (`ObjectStats`), by its key
    /// (`kernels::host::object_key`): its material's vertices, and its shape's rays.
    pub objects: Vec<ObjectStats>,
}

impl Stats {
    /// A batch of `items` went through `stage` in `seconds` (0 if not timed).
    pub fn record(&mut self, stage: Stage, items: u64, seconds: f64) {
        let s = &mut self.stages[stage as usize];
        s.batches += 1;
        s.items += items;
        s.seconds += seconds;
    }

    /// Time `stage` took, found apart from its batches (the GPU's kernels, timed by the
    /// device's clock where it has one: `gpu::Timers`). A stage timed on the host too
    /// (`Generate`: the camera rays, then the kernel that starts the paths) has both.
    pub fn add_seconds(&mut self, stage: Stage, seconds: f64) {
        self.stages[stage as usize].seconds += seconds;
    }

    /// `paths` were alive at the start of round `round` of a pass.
    pub fn round(&mut self, round: usize, paths: u64) {
        if self.live.len() <= round {
            self.live.resize(round + 1, 0);
        }
        self.live[round] += paths;
    }

    pub fn stage(&self, stage: Stage) -> StageStats {
        self.stages[stage as usize]
    }

    /// The host's work on object `key`.
    pub fn object(&mut self, key: usize) -> &mut ObjectStats {
        if self.objects.len() <= key {
            self.objects.resize(key + 1, ObjectStats::default());
        }
        &mut self.objects[key]
    }

    /// Add work on objects, by key, as `take_objects` gives it.
    pub fn merge_objects(&mut self, objects: &[ObjectStats]) {
        for (key, o) in objects.iter().enumerate() {
            self.object(key).add(o);
        }
    }

    pub fn merge(&mut self, other: &Stats) {
        for (a, b) in self.stages.iter_mut().zip(&other.stages) {
            a.batches += b.batches;
            a.items += b.items;
            a.seconds += b.seconds;
        }
        self.passes += other.passes;
        for (round, paths) in other.live.iter().enumerate() {
            self.round(round, *paths);
        }
        self.device_seconds += other.device_seconds;
        self.merge_objects(&other.objects);
    }

    pub fn is_empty(&self) -> bool {
        self.stages.iter().all(|s| s.batches == 0)
    }

    /// Of the path-rounds a pass would run with every path dispatched every round (the first
    /// round's paths, times the rounds), the share that had a live path.
    pub fn live_share(&self) -> f64 {
        match self.live.first() {
            Some(&first) if first > 0 => self.live.iter().sum::<u64>() as f64 / (first * self.live.len() as u64) as f64,
            _ => 1.,
        }
    }

    /// A table of the stages, and the rounds' live paths, for the end of a render.
    pub fn report(&self) -> String {
        let mut out = format!("| Stages, over {} passes:\n|   {:<11}{:>10}{:>14}{:>12}{:>12}\n", self.passes, "stage", "batches", "items", "items/batch", "seconds");
        for stage in Stage::ALL {
            let s = self.stage(stage);
            if s.batches == 0 {
                continue;
            }
            let seconds = if s.seconds > 0. { format!("{:.3}", s.seconds) } else { "-".to_string() };
            out += &format!("|   {:<11}{:>10}{:>14}{:>12.0}{:>12}\n", stage.name(), s.batches, s.items, s.items as f64 / s.batches as f64, seconds);
        }
        if self.device_seconds > 0. {
            out += &format!("|   on the device, stages together: {:.3} s\n", self.device_seconds);
        }
        if let Some(&first) = self.live.first().filter(|&&f| f > 0) {
            let shares: Vec<String> = self.live.iter().map(|&n| format!("{:.0}", 100. * n as f64 / first as f64)).collect();
            out += &format!("|   paths alive by round, % of those generated: {}\n", shares.join(" "));
            out += &format!("|   {:.0}% of path-rounds had a live path", 100. * self.live_share());
        }
        out
    }
}

/// How many more samples each pixel of a region of the film should take in its next pass: up
/// to `most` for a pixel that isn't complete, none for one that is.
///
/// A pixel is complete when it has `target` samples; or, with adaptive sampling, when it and
/// its neighbours are no longer too noisy (`adaptive::active`, which reads the region's sums:
/// without adaptive sampling they aren't looked at, and may be empty). `gain` is how bright a
/// linear one is displayed (`adaptive::display_gain`). The samples a pixel takes are its next
/// ones: those numbered from its count.
///
/// The answer depends on the region's own pixels and nothing else, so it is the same however
/// and wherever the passes before it were run.
pub fn plan(film: &Region, target: usize, adaptive: Option<&Adaptive>, gain: f64, most: usize) -> Vec<usize> {
    let active: Vec<bool> = match adaptive {
        None => film.samples.iter().map(|&n| n < target).collect(),
        Some(a) => crate::adaptive::active(film, target, a, gain),
    };
    film.samples.iter().zip(&active)
        .map(|(&n, &on)| if on { most.min(target.saturating_sub(n)) } else { 0 })
        .collect()
}

/// How many samples of each pixel a pass of the GPU's carries at once (its lanes:
/// `pathtrace::slots`), of the `room` its buffers have.
///
/// All there is room for, where the kernels do all the work: the rounds a pass takes are those
/// of one sample however many it carries, so each lane more is a pass's dispatches saved.
/// Measured (llvmpipe, 320 x 180, 64 samples, 8 lanes against 1): the same pixels from 80
/// rounds instead of 640, the device's time 11% less on demo.json, 15% on glossy-sun.json,
/// 23% on instances.json.
///
/// The same where the host shades some of the vertices (`gpu::host`), now that the paths
/// waiting for it are gathered by a kernel, so that a read of the device is the size of the
/// host's work: its reads fall by the lanes too (textures.json: 48 for 384), and the render's
/// time by a fifth to a quarter on rocks.json, bumps.json and pbrt-materials.json; on
/// textures.json, where every path's first vertex is the host's, it is unchanged. (Before the
/// gathering, when each read copied every lane's paths, 8 lanes cost that scene a fifth more
/// than one, and such scenes were kept to one.)
///
/// So the decision is only the room, for now. It is here because it won't stay so: the room
/// is a budget someone has to set, and a browser's frame wants a pass no longer than its
/// watchdog allows.
pub fn lanes(room: usize) -> usize {
    room.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn film<'a>(sum: &'a [Color], squares: &'a [Color], samples: &'a [usize]) -> Region<'a> {
        Region { width: samples.len(), sum, squares, samples }
    }

    /// Without adaptive sampling every pixel takes samples until it has the target, never
    /// more than a pass's worth, never past the target; the sums aren't needed.
    #[test]
    fn test_plan_fills_each_pixel_to_the_target() {
        let samples = [0, 5, 14, 16, 20];
        assert_eq!(plan(&film(&[], &[], &samples), 16, None, 1., 8), vec![8, 8, 2, 0, 0]);
        assert_eq!(plan(&film(&[], &[], &samples), 16, None, 1., 1), vec![1, 1, 1, 0, 0]);
    }

    #[test]
    fn test_lanes() {
        assert_eq!((lanes(8), lanes(0)), (8, 1));
    }

    #[test]
    fn test_stats_add_up() {
        let mut a = Stats::default();
        a.record(Stage::Intersect, 100, 0.5);
        a.record(Stage::Intersect, 50, 0.25);
        a.round(0, 100);
        a.round(1, 50);
        a.passes = 1;
        let mut b = Stats::default();
        b.record(Stage::Shade, 40, 0.);
        b.round(0, 100);
        b.round(1, 0);
        b.round(2, 0);
        b.passes = 1;
        a.merge(&b);
        assert_eq!(a.stage(Stage::Intersect), StageStats { batches: 2, items: 150, seconds: 0.75 });
        assert_eq!(a.stage(Stage::Shade).items, 40);
        assert_eq!((a.passes, a.live.clone()), (2, vec![200, 50, 0]));
        // 250 live path-rounds of 3 rounds of 200.
        assert!((a.live_share() - 250. / 600.).abs() < 1e-12);
        assert!(!a.is_empty() && Stats::default().is_empty());
        let report = a.report();
        assert!(report.contains("intersect") && report.contains("shade") && !report.contains("shadow"), "{}", report);
        assert!(report.contains("100 25 0"), "{}", report);
    }

    /// With adaptive sampling a pixel whose samples all agree stops once it has the minimum,
    /// and a noisy one carries on, with the pixels beside it.
    #[test]
    fn test_plan_stops_quiet_pixels() {
        let adaptive = Adaptive { threshold: 0.01, min_samples: 4 };
        let n = 8usize;
        // Eight pixels in a row: quiet ones at 0.5 every sample; one, at the far end,
        // alternating between 0 and 1.
        let quiet = (Color::new(0.5 * n as f64, 0.5 * n as f64, 0.5 * n as f64), Color::new(0.25 * n as f64, 0.25 * n as f64, 0.25 * n as f64));
        let noisy = (Color::new(0.5 * n as f64, 0.5 * n as f64, 0.5 * n as f64), Color::new(0.5 * n as f64, 0.5 * n as f64, 0.5 * n as f64));
        let mut sum = vec![quiet.0; 8];
        let mut squares = vec![quiet.1; 8];
        sum[7] = noisy.0;
        squares[7] = noisy.1;
        let samples = vec![n; 8];
        let counts = plan(&film(&sum, &squares, &samples), 64, Some(&adaptive), 1., 8);
        assert_eq!(counts[0], 0, "a quiet pixel far from the noise: {:?}", counts);
        assert_eq!(counts[7], 8, "the noisy pixel: {:?}", counts);
        // Before the minimum, every pixel is still wanted, quiet or not.
        let few = vec![2usize; 8];
        assert!(plan(&film(&sum, &squares, &few), 64, Some(&adaptive), 1., 8).iter().all(|&c| c == 8));
    }
}
