//! # A world built in stages, with a state on disk
//!
//! A full-scale landscape is several computations, each reading the ones before it: the
//! plates leave a crust, the climate is worked out over the planet that crust shapes, a site
//! is found on it, the whole planet is eroded on a coarse grid, and patches nested about the
//! site are eroded on finer ones, each driven at its edge by the one around it. Together they
//! take from seconds to hours, and until this module they ran inside scene parsing, printing
//! as they went, with only the patches' evolution kept between runs. This module owns that
//! build: what the stages are, what each reads, where its result is kept, and how far a build
//! has got. It is piece 1 of `plan.md` section 14; the pieces after it (a viewpoint chosen from
//! the terrain, one seed to a scene file, a render farm that ships a built world) read what is
//! built here.
//!
//! ## Stages and keys
//!
//! | stage | what it computes | reads | kept as |
//! |---|---|---|---|
//! | `plates` | plate tectonics: the crust (`tectonics`) | | `plates-<key>.json` |
//! | `climate` | temperature, seasons, rain (`climate`) | plates | `climate-<key>.json` |
//! | `site` | where the scene's origin is on the planet | plates | `site-<key>.json` |
//! | `planet` | the whole planet's erosion (`simulation`) | plates, climate | `planet-<key>-<years>.json` |
//! | `patch <spacing> m` | a nested patch's, coarsest first | the stage around it, plates, climate | `patch-<key>-<years>.json` |
//!
//! The planet's relief below the crust's grid is noise evaluated where it is asked for
//! (`planet`), so it has no state and no stage: `plates` is all of the planet that is kept.
//!
//! A stage's key is a hash of its name, its own parameters (which include the seed) and the
//! keys of the stages it reads, so it names one result: change a parameter and that stage and
//! every stage reading it have new keys, while the others keep theirs and are found again. A
//! patch's own parameters are where it lies and how fine it is, not the camera that put it
//! there: patches are snapped to an eighth of their width, so nearby cameras (an animation's
//! frames) come to the same keys and share the patches.
//!
//! The landscape's evolution is resumable in time (`simulation`: the state at t2 is the state
//! at t1 advanced), and the keys follow which stages that holds for. The planet's stage is
//! driven by nothing outside itself, so its key leaves the time out and its files are named by
//! their years: a build to a later time starts from the latest earlier one. A patch's edge
//! moves at the rate the stage around it changed over the whole time, so a patch evolved to
//! one time is not on the way to another: its key includes the time. Either way a long stage
//! writes where it has got to every `CHECKPOINT` seconds (`...-<years>.part.json`), so a build
//! that is killed loses at most that much; a state is only taken up at a whole number of
//! steps, because steps of different lengths give (slightly) different landscapes.
//!
//! The hash is FNV-1a, written out here, rather than the standard library's hasher, whose
//! algorithm may change between releases of Rust: these keys name files that should outlive
//! the compiler. They don't know when the code of a stage changes; `FORMAT` is in every key
//! for that, to be raised when a change to a stage's computation should orphan what was built
//! before (as with a render's saved samples, the alternative is to delete the directory).
//!
//! Every stage draws its random numbers from the landscape's one seed, on streams of its own
//! (`Planet`'s octaves, the plates' generator). The plan asks for sub-seeds hashed from the
//! seed and the stage's name, so that one stage can be reseeded alone; that would change every
//! existing landscape, so it waits for the `world` scene object (piece 4), which has no
//! pictures to keep.
//!
//! ## The world directory
//!
//! Results are files in one directory (a landscape's `"cache"`), each wrapped with its
//! stage's name, its key and the seconds it took, and written under a temporary name then
//! renamed, so a reader never sees half a file and two builds may share a directory. They are
//! JSON, as the patches' snapshots were: `serde_json` is built with `float_roundtrip`, so the
//! numbers come back to the bit, which is what lets a reused world give the same picture as a
//! fresh one. A crust on the default 96 cells a face edge is about 7 MB, a climate on 64 about
//! 2 MB, a patch of 128 x 128 cells about 2 MB.
//!
//! Stages are found by their keys, not through any index, so scenes of one world share its
//! planet and whichever patches they have in common. Beside them, each build writes a
//! manifest, `world-<key>.json` (the key a hash of the build's whole description): its stages
//! in order, each with its key, what it reads, its status (`pending`, `running`, `stopped`,
//! `done`), how far it has got, the seconds it took (for a reused stage, the seconds it took
//! when it was built), its file, a line about it, and a small picture where one is cheap: the
//! planet's map for `plates`, the biomes for `climate`, shaded relief for the simulations
//! (`terrain::map`). The manifest is rewritten as stages start, checkpoint and finish, so it
//! can be watched; nothing reads it to decide what to build.
//!
//! With no directory the same stages run in memory and nothing is written (as on a host with
//! no file system: a directory that can't be made is the same as none).
//!
//! The crust and the climate are also shared within a process (`tectonics::crust_from`,
//! `climate::climate_from`), because `Planet::new` is called from many places that have no
//! world to ask (every `Terrain`, the weather, the relief). The stages here fill those shared
//! maps, from the directory when it has them, so that everything after reads the same crust.
//!
//! ## Progress
//!
//! `build` takes a callback and tells it `Progress` events: the stage, the fraction of it
//! done, and sometimes a line of text or the path of a picture just written. The type is
//! plain data that serialises, so a server can stream it as it is. The callback returns
//! whether to carry on: on `false` the build saves the stage it is in and returns `Stopped`,
//! and the next build with the same description takes it up from there. (The plates and the
//! climate aren't interrupted part-way: they are the short stages, and a stop takes effect
//! when they finish.)
//!
//! A scene's `landscape` is built from inside scene parsing, which has no callback to pass,
//! so it reports through `report`: to whatever a host installed with `set_reporter`, or else
//! to `print`, which is the command line's output: the lines the build used to print itself,
//! and, for a stage that has said nothing for `QUIET` seconds, how far it has got.

use crate::na::Vector3;
use crate::terrain::climate::{Climate, ClimateParams, ClimateState};
use crate::terrain::domain::Domain;
use crate::terrain::planet::{Planet, PlanetParams};
use crate::terrain::scenery::{self, Frame};
use crate::terrain::simulation::{Snapshot, Terrain, TerrainParams};
use crate::terrain::tectonics::{Crust, CrustState};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use web_time::Instant;

/// In every key: raise it when a stage's computation changes, so that what was built before
/// isn't taken for what would be built now.
pub const FORMAT: u32 = 1;
/// A running stage saves where it has got to this often, seconds.
pub const CHECKPOINT: f64 = 30.;
/// `print` says how far a stage has got when nothing has been printed for this long, seconds.
pub const QUIET: f64 = 10.;
/// Width of the stages' pictures, pixels.
const PICTURE: usize = 512;

/// What a build tells its caller as it goes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    /// The stage's name, as in the manifest.
    pub stage: String,
    /// How much of the stage is done, 0 to 1.
    pub fraction: f64,
    /// A line about it, for a person.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A picture of the stage just written: its path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

impl Progress {
    /// A line of text with nothing to measure.
    pub fn note(stage: &str, message: String) -> Progress {
        Progress { stage: stage.to_string(), fraction: 1., message: Some(message), picture: None }
    }
}

type Reporter = Box<dyn Fn(&Progress) + Send + Sync>;
static REPORTER: Mutex<Option<Reporter>> = Mutex::new(None);

/// Send the progress of worlds built inside scene parsing to `reporter` (a server's stream,
/// say) instead of printing it; None to print again.
pub fn set_reporter(reporter: Option<Reporter>) {
    *REPORTER.lock().unwrap() = reporter;
}

/// Report an event from a build that was given no callback of its own.
pub fn report(e: &Progress) {
    match &*REPORTER.lock().unwrap() {
        Some(r) => r(e),
        None => print(e),
    }
}

/// The command line's reporter: an event's message, or, if nothing has been printed for
/// `QUIET` seconds, how far its stage has got.
pub fn print(e: &Progress) {
    static LAST: Mutex<Option<Instant>> = Mutex::new(None);
    let mut last = LAST.lock().unwrap();
    match &e.message {
        Some(m) => println!("{}", m),
        None if e.fraction < 1. && last.is_some_and(|t| t.elapsed().as_secs_f64() >= QUIET) => {
            println!("World: {}, {:.0}%", e.stage, 100. * e.fraction);
        },
        None => {
            last.get_or_insert_with(Instant::now);
            return;
        },
    }
    *last = Some(Instant::now());
}

/// A build was told to stop (its callback returned false) in this stage. What it had done is
/// saved, where there is a directory.
#[derive(Clone, Debug, PartialEq)]
pub struct Stopped { pub stage: String }

impl std::fmt::Display for Stopped {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "the world's build was stopped at {}", self.stage) }
}

/// A key: the FNV-1a hash (64 bits) of `parts`, each ended so that no two lists run together.
pub fn key(parts: &[&str]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut eat = |b: u8| { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); };
    for b in FORMAT.to_string().bytes() { eat(b); }
    for p in parts {
        eat(0);
        for b in p.bytes() { eat(b); }
    }
    format!("{:016x}", h)
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status { Pending, Running, Stopped, Done }

/// A stage of a build, as the manifest lists it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    /// None until the build reaches the stage: a patch's key needs the site, which is a
    /// stage's result.
    pub key: Option<String>,
    /// The stages whose results it reads, by name.
    pub reads: Vec<String>,
    pub status: Status,
    /// Whether its result was found (in the directory, or already in this process) rather than
    /// computed by this build.
    pub reused: bool,
    /// How much of it is done, 0 to 1.
    pub fraction: f64,
    /// Seconds it took to compute, over however many builds; for a reused stage, when it was
    /// built (0 if that isn't known).
    pub seconds: f64,
    /// Its result, a file in the directory (None in memory, and for a stage with nothing to
    /// keep); while it runs, its latest checkpoint.
    pub file: Option<String>,
    /// A small picture of it, a file in the directory.
    pub picture: Option<String>,
    /// A line about the result.
    pub summary: Option<String>,
}

impl Stage {
    fn pending(name: &str) -> Stage {
        Stage { name: name.to_string(), key: None, reads: vec![], status: Status::Pending, reused: false, fraction: 0., seconds: 0., file: None, picture: None, summary: None }
    }
}

/// What a world is made of and how far its build has got: `world-<key>.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub format: u32,
    /// The hash of the build's description.
    pub key: String,
    pub seed: u64,
    /// In the order they are built.
    pub stages: Vec<Stage>,
}

impl Manifest {
    pub fn read(path: &Path) -> Option<Manifest> {
        serde_json::from_slice(&std::fs::read(path).ok()?).ok()
    }

    pub fn stage(&self, name: &str) -> Option<&Stage> { self.stages.iter().find(|s| s.name == name) }

    /// Whether every stage is done.
    pub fn done(&self) -> bool { self.stages.iter().all(|s| s.status == Status::Done) }

    fn entry(&mut self, name: &str) -> &mut Stage {
        let i = match self.stages.iter().position(|s| s.name == name) {
            Some(i) => i,
            None => { self.stages.push(Stage::pending(name)); self.stages.len() - 1 },
        };
        &mut self.stages[i]
    }
}

/// A stage's result as it is kept: with what it is and what it cost.
#[derive(Deserialize)]
struct Saved<T> { key: String, seconds: f64, state: T }
#[derive(Serialize)]
struct Saving<'a, T> { stage: &'a str, key: &'a str, seconds: f64, state: &'a T }

/// The world directory, if there is one.
struct Store { dir: Option<PathBuf> }

impl Store {
    fn open(dir: Option<&Path>) -> Store {
        let dir = dir.and_then(|d| match std::fs::create_dir_all(d) {
            Ok(()) => Some(d.to_path_buf()),
            Err(e) => { eprintln!("World: can't keep anything in {}: {}", d.display(), e); None },
        });
        Store { dir }
    }

    fn path(&self, file: &str) -> Option<PathBuf> { self.dir.as_ref().map(|d| d.join(file)) }

    fn has(&self, file: &str) -> bool { self.path(file).is_some_and(|p| p.exists()) }

    /// The result in `file`, if it is there, whole, and of this key.
    fn read<T: serde::de::DeserializeOwned>(&self, file: &str, key: &str) -> Option<Saved<T>> {
        let f = std::fs::File::open(self.path(file)?).ok()?;
        let saved: Saved<T> = serde_json::from_reader(std::io::BufReader::new(f)).ok()?;
        (saved.key == key).then_some(saved)
    }

    /// Write `value` as `file` (`pretty`: laid out to be read): under another name first, so
    /// that the file is never there in part. Whether it was written.
    fn write_json<T: serde::Serialize>(&self, file: &str, value: &T, pretty: bool) -> bool {
        let Some(path) = self.path(file) else { return false };
        #[cfg(not(target_arch = "wasm32"))]
        let process = std::process::id();
        #[cfg(target_arch = "wasm32")]
        let process = 0;
        let partial = path.with_extension(format!("tmp{}", process));
        let written = std::fs::File::create(&partial).map_err(|e| e.to_string())
            .and_then(|f| {
                let out = std::io::BufWriter::new(f);
                if pretty { serde_json::to_writer_pretty(out, value) } else { serde_json::to_writer(out, value) }.map_err(|e| e.to_string())
            })
            .and_then(|()| std::fs::rename(&partial, &path).map_err(|e| e.to_string()));
        if let Err(e) = &written {
            eprintln!("World: couldn't write {}: {}", path.display(), e);
            let _ = std::fs::remove_file(&partial);
        }
        written.is_ok()
    }

    fn write<T: serde::Serialize>(&self, file: &str, stage: &str, key: &str, seconds: f64, state: &T) -> Option<String> {
        self.write_json(file, &Saving { stage, key, seconds, state }, false).then(|| file.to_string())
    }

    /// The states kept of an evolving stage: files named `prefix`, then years, then `.json`
    /// (a result) or `.part.json` (a checkpoint); the latest first, results before checkpoints
    /// of the same years.
    fn states(&self, prefix: &str) -> Vec<(u64, bool, String)> {
        let Some(dir) = &self.dir else { return vec![] };
        let mut found: Vec<(u64, bool, String)> = std::fs::read_dir(dir).into_iter().flatten().flatten().filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            let rest = name.strip_prefix(prefix)?.strip_suffix(".json")?;
            let (years, part) = match rest.strip_suffix(".part") { Some(y) => (y, true), None => (rest, false) };
            Some((years.parse().ok()?, part, name))
        }).collect();
        found.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        found
    }

    /// Remove the checkpoints of `prefix` up to `years`: a later state has replaced them.
    fn clear_checkpoints(&self, prefix: &str, years: u64, but: Option<&str>) {
        for (y, part, name) in self.states(prefix) {
            if part && y <= years && but != Some(name.as_str()) && let Some(p) = self.path(&name) {
                let _ = std::fs::remove_file(p);
            }
        }
    }
}

/// Where the scene's origin is on the planet.
#[derive(Clone, Debug, PartialEq)]
pub enum Site {
    /// Latitude and longitude, degrees.
    At { lat: f64, lon: f64 },
    /// The seed's most mountainous spot, ...
    Mountains,
    /// ... its most volcanic mountains, ...
    Volcanoes,
    /// ... or a coast with mountains behind it.
    Coast,
}

/// Samples of the planet a site is chosen from.
const SITE_SAMPLES: usize = 20000;

impl Site {
    /// The site's latitude and longitude on `planet`, degrees.
    pub fn find(&self, planet: &Planet) -> (f64, f64) {
        match self {
            Site::At { lat, lon } => (*lat, *lon),
            Site::Mountains => planet.most_mountainous(SITE_SAMPLES),
            Site::Volcanoes => planet.most_volcanic(SITE_SAMPLES),
            Site::Coast => planet.most_coastal(SITE_SAMPLES),
        }
    }
}

/// What the patches are centred on, as scene points: one given, or the ground a camera looks
/// over (below it, moved towards where it looks by up to half the finest patch's half-width).
#[derive(Clone, Debug, PartialEq)]
pub enum Focus {
    Point(Vector3<f64>),
    Camera { location: Vector3<f64>, lookat: Vector3<f64> },
}

impl Focus {
    /// Its direction from the planet's centre, with the camera's reach `ahead` metres.
    pub fn centre(&self, frame: &Frame, ahead: f64) -> Vector3<f64> {
        match self {
            Focus::Point(p) => frame.on_planet(p).0,
            Focus::Camera { location, lookat } => {
                let (foot, _) = frame.on_planet(location);
                let (x, z) = frame.tangent(&foot);
                let (lx, lz) = frame.tangent(&frame.on_planet(lookat).0);
                let (dx, dz) = (lx - x, lz - z);
                let len = dx.hypot(dz);
                if len < 1e-9 { return foot; }
                let k = len.min(ahead) / len;
                frame.dir(x + dx * k, z + dz * k)
            },
        }
    }
}

/// A patch level: cells along a side, and their spacing (m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Level { pub cells: usize, pub spacing: f64 }

/// A world to build: what a full-scale `landscape` asks for.
#[derive(Clone, Debug)]
pub struct Description {
    /// The processes' parameters, the seed and the planet's among them.
    pub params: TerrainParams,
    pub site: Site,
    /// Years of landscape evolution.
    pub time: f64,
    /// Cells per face edge of the whole planet's simulation; None for its noise topography
    /// alone.
    pub planet_cells: Option<usize>,
    /// The patches nested about the focus, finest first.
    pub levels: Vec<Level>,
    pub focus: Focus,
    /// The scene's vertical exaggeration (it places the focus).
    pub exaggeration: f64,
}

impl Description {
    /// The names of the stages a build of this goes through, in order.
    pub fn stages(&self) -> Vec<String> {
        let mut names = vec![];
        if self.params.planet.tectonics.is_some() { names.push("plates".to_string()); }
        if self.params.climate.is_some() { names.push("climate".to_string()); }
        names.push("site".to_string());
        if self.planet_cells.is_some() { names.push("planet".to_string()); }
        for k in (0..self.levels.len()).rev() { names.push(self.patch_name(k)); }
        names
    }

    /// The stage of level `k` (0 the finest): named by its spacing, and by its place in the
    /// nest too if another level has the same.
    fn patch_name(&self, k: usize) -> String {
        let spacing = self.levels[k].spacing;
        if self.levels.iter().filter(|l| l.spacing == spacing).count() > 1 { format!("patch {} m ({})", spacing, k) } else { format!("patch {} m", spacing) }
    }
}

/// A built world.
pub struct World {
    /// The site's latitude and longitude, degrees.
    pub site: (f64, f64),
    /// The scene's frame there.
    pub frame: Frame,
    pub planet: Planet,
    pub climate: Option<Arc<Climate>>,
    /// The whole planet's simulation.
    pub globe: Option<Terrain>,
    /// The patches, finest first.
    pub patches: Vec<Terrain>,
    pub manifest: Manifest,
    /// Where the manifest is, if there is a directory.
    pub manifest_path: Option<PathBuf>,
}

struct Builder<'a> {
    store: Store,
    manifest: Manifest,
    manifest_file: Option<String>,
    on_progress: &'a mut dyn FnMut(&Progress) -> bool,
    stopped: bool,
    checkpoint: f64,
    /// The keys of the stages reached so far, by name.
    keys: HashMap<String, String>,
}

/// What a stage ended with.
struct Outcome { reused: bool, seconds: f64, file: Option<String>, picture: Option<String>, summary: Option<String> }

impl Builder<'_> {
    fn new<'a>(dir: Option<&Path>, on_progress: &'a mut dyn FnMut(&Progress) -> bool) -> Builder<'a> {
        Builder {
            store: Store::open(dir), manifest: Manifest { format: FORMAT, key: String::new(), seed: 0, stages: vec![] }, manifest_file: None,
            on_progress, stopped: false, checkpoint: CHECKPOINT, keys: HashMap::new(),
        }
    }

    fn save(&self) {
        if let Some(f) = &self.manifest_file { self.store.write_json(f, &self.manifest, true); }
    }

    fn tell(&mut self, stage: &str, fraction: f64, message: Option<String>, picture: Option<String>) {
        if !(self.on_progress)(&Progress { stage: stage.to_string(), fraction, message, picture }) { self.stopped = true; }
    }

    /// The key of a stage from its own parameters and the stages it reads (those of them this
    /// world has), and their names for the manifest.
    fn chain(&self, own: &[&str], reads: &[&str]) -> (String, Vec<String>) {
        let reads: Vec<&str> = reads.iter().copied().filter(|r| self.keys.contains_key(*r)).collect();
        let parts: Vec<&str> = own.iter().copied().chain(reads.iter().map(|r| self.keys[*r].as_str())).collect();
        (key(&parts), reads.iter().map(|r| r.to_string()).collect())
    }

    /// Start a stage, unless the build has been stopped.
    fn begin(&mut self, name: &str, key: &str, reads: Vec<String>) -> Result<(), Stopped> {
        if self.stopped { return Err(Stopped { stage: name.to_string() }); }
        self.keys.insert(name.to_string(), key.to_string());
        let s = self.manifest.entry(name);
        s.key = Some(key.to_string());
        s.reads = reads;
        s.status = Status::Running;
        self.save();
        self.tell(name, 0., None, None);
        Ok(())
    }

    fn finish(&mut self, name: &str, o: Outcome) {
        let s = self.manifest.entry(name);
        s.status = Status::Done;
        s.fraction = 1.;
        s.reused = o.reused;
        s.seconds = o.seconds;
        s.file = o.file;
        s.picture = o.picture.clone();
        s.summary = o.summary.clone();
        self.save();
        let picture = o.picture.and_then(|p| self.store.path(&p)).map(|p| p.display().to_string());
        self.tell(name, 1., o.summary, picture);
    }

    /// The picture `file` of a stage, drawn by `draw` (given its path) if it isn't there.
    fn picture(&self, file: String, draw: impl FnOnce(&str)) -> Option<String> {
        let path = self.store.path(&file)?;
        if !path.exists() { draw(path.to_str()?); }
        Some(file)
    }

    /// The planet: its plates, from the directory or simulated.
    fn planet(&mut self, params: &PlanetParams) -> Result<Planet, Stopped> {
        let Some(tp) = &params.tectonics else { return Ok(Planet::new(params.clone())) };
        let name = "plates";
        let (key, reads) = self.chain(&[name, &format!("{:?} {} {:?}", tp, params.seed, params.radius)], &[]);
        self.begin(name, &key, reads)?;
        let file = format!("plates-{}.json", key);
        let begun = Instant::now();
        let mut outcome = None;
        let crust = crate::terrain::tectonics::crust_from(tp, params.seed, params.radius, || {
            if let Some(saved) = self.store.read::<CrustState>(&file, &key)
                && let Some(c) = Crust::from_state(saved.state, tp.cells, params.radius) {
                outcome = Some((true, saved.seconds, Some(format!("Tectonics: the crust of {} cells read from {}", c.plate.len(), file))));
                return c;
            }
            let on_progress = &mut *self.on_progress;
            let mut stopped = false;
            let (c, summary) = crate::terrain::tectonics::simulate(tp, params.seed, params.radius, &mut |fraction| {
                stopped |= !on_progress(&Progress { stage: name.to_string(), fraction, message: None, picture: None });
            });
            self.stopped |= stopped;
            let seconds = begun.elapsed().as_secs_f64();
            self.store.write(&file, name, &key, seconds, &c.state());
            outcome = Some((false, seconds, Some(summary)));
            c
        });
        // Made earlier in this process (by another build, or something with no world to ask):
        // nothing to say, but the directory should have it.
        let (reused, seconds, summary) = outcome.unwrap_or_else(|| {
            if self.store.dir.is_some() && !self.store.has(&file) { self.store.write(&file, name, &key, 0., &crust.state()); }
            (true, 0., None)
        });
        let planet = Planet::new(params.clone());
        let picture = self.picture(format!("plates-{}.png", key), |path| crate::terrain::map::write_planet_map(&planet, path, PICTURE));
        let file = self.store.has(&file).then_some(file);
        self.finish(name, Outcome { reused, seconds, file, picture, summary });
        Ok(planet)
    }

    /// The planet's climate, from the directory or computed.
    fn climate(&mut self, params: Option<&ClimateParams>, planet: &Planet) -> Result<Option<Arc<Climate>>, Stopped> {
        let Some(cp) = params else { return Ok(None) };
        let name = "climate";
        let (key, reads) = self.chain(&[name, &format!("{:?} {:?}", cp, planet.params)], &["plates"]);
        self.begin(name, &key, reads)?;
        let file = format!("climate-{}.json", key);
        let begun = Instant::now();
        let mut outcome = None;
        let climate = crate::terrain::climate::climate_from(cp, planet, || {
            if let Some(saved) = self.store.read::<ClimateState>(&file, &key)
                && let Some(c) = Climate::from_state(saved.state, cp.clone(), planet.params.radius) {
                outcome = Some((true, saved.seconds, Some(format!("Climate: {} cells read from {}", c.cells(), file))));
                return c;
            }
            let c = Climate::new(cp.clone(), planet);
            let seconds = begun.elapsed().as_secs_f64();
            self.store.write(&file, name, &key, seconds, &c.state());
            outcome = Some((false, seconds, Some(format!("Climate: {} cells in {:.1?}", c.cells(), begun.elapsed()))));
            c
        });
        let (reused, seconds, summary) = outcome.unwrap_or_else(|| {
            if self.store.dir.is_some() && !self.store.has(&file) { self.store.write(&file, name, &key, 0., &climate.state()); }
            (true, 0., None)
        });
        let picture = self.picture(format!("climate-{}.png", key), |path| crate::terrain::map::write_climate_map(planet, &climate, path, PICTURE));
        let file = self.store.has(&file).then_some(file);
        self.finish(name, Outcome { reused, seconds, file, picture, summary });
        Ok(Some(climate))
    }

    /// Where the scene's origin is: latitude and longitude, degrees.
    fn site(&mut self, site: &Site, planet: &Planet) -> Result<(f64, f64), Stopped> {
        let name = "site";
        let (key, reads) = self.chain(&[name, &format!("{:?} {:?}", site, planet.params)], &["plates"]);
        self.begin(name, &key, reads)?;
        let line = |(lat, lon): (f64, f64)| Some(format!("Landscape: the scene's origin is at {:.3}°, {:.3}°", lat, lon));
        if let Site::At { lat, lon } = site {
            // Nothing to find, so nothing to keep.
            self.finish(name, Outcome { reused: false, seconds: 0., file: None, picture: None, summary: line((*lat, *lon)) });
            return Ok((*lat, *lon));
        }
        let file = format!("site-{}.json", key);
        let begun = Instant::now();
        let (at, reused, seconds) = match self.store.read::<(f64, f64)>(&file, &key) {
            Some(saved) => (saved.state, true, saved.seconds),
            None => {
                let at = site.find(planet);
                let seconds = begun.elapsed().as_secs_f64();
                self.store.write(&file, name, &key, seconds, &at);
                (at, false, seconds)
            },
        };
        let file = self.store.has(&file).then_some(file);
        self.finish(name, Outcome { reused, seconds, file, picture: None, summary: line(at) });
        Ok(at)
    }

    /// Run `t` to `time` years as stage `name`, whose files begin with the first word of its
    /// name and its `key`: from the latest state the directory has of it, saving where it gets
    /// to. `begun` is when the stage's work began (making `t` is part of it).
    fn evolve(&mut self, name: &str, key: &str, reads: Vec<String>, mut t: Terrain, time: f64, begun: Instant) -> Result<Terrain, Stopped> {
        self.begin(name, key, reads)?;
        let prefix = format!("{}-{}-", name.split(' ').next().unwrap_or(name), key);
        let wanted = time.round() as u64;
        let done = |t: &Terrain| t.time >= time - 1e-9;
        // Seconds earlier builds spent on the state this one starts from.
        let mut before = 0.;
        let mut file = None;
        if time > 0. {
            let step = t.params.step;
            let whole = |at: f64| { let steps = at / step; (steps - steps.round()).abs() < 1e-9 };
            let usable = |s: &Snapshot| s.fits(&t) && s.time() <= time + 1e-9 && (whole(s.time()) || (s.time() - time).abs() < 1e-9);
            let found = self.store.states(&prefix).into_iter().filter(|(years, _, _)| *years <= wanted)
                .find_map(|(_, _, f)| self.store.read::<Snapshot>(&f, key).filter(|s| usable(&s.state)).map(|s| (f, s)));
            if let Some((f, saved)) = found {
                before = saved.seconds;
                t.restore(saved.state);
                file = Some(f);
            }
        }
        let reused = file.is_some() && done(&t);
        if !reused { file = None; }
        let mut seconds = before;
        if !done(&t) {
            let mut last = Instant::now();
            let this = &mut *self;
            t.run_while(time, &mut |t| {
                let fraction = (t.time / time).min(1.);
                this.tell(name, fraction, None, None);
                if !done(t) && (this.stopped || last.elapsed().as_secs_f64() >= this.checkpoint) {
                    let years = t.time.round() as u64;
                    let part = format!("{}{}.part.json", prefix, years);
                    let seconds = before + begun.elapsed().as_secs_f64();
                    let written = this.store.write(&part, name, key, seconds, &t.snapshot());
                    if written.is_some() { this.store.clear_checkpoints(&prefix, years, Some(&part)); }
                    let s = this.manifest.entry(name);
                    s.file = written;
                    s.fraction = fraction;
                    s.seconds = seconds;
                    this.save();
                    last = Instant::now();
                }
                !this.stopped
            });
            if !done(&t) {
                self.manifest.entry(name).status = Status::Stopped;
                self.save();
                return Err(Stopped { stage: name.to_string() });
            }
            seconds = before + begun.elapsed().as_secs_f64();
            file = self.store.write(&format!("{}{}.json", prefix, wanted), name, key, seconds, &t.snapshot());
            if file.is_some() { self.store.clear_checkpoints(&prefix, wanted, None); }
        }
        let summary = if reused {
            format!("Terrain: {} cells evolved for {} years, read from {}", t.domain.len(), time, file.as_deref().unwrap_or(""))
        } else {
            format!("Terrain: {} cells evolved for {} years in {:.1?}", t.domain.len(), time, begun.elapsed())
        };
        let picture = self.picture(format!("{}{}.png", prefix, wanted), |path| crate::terrain::map::write_map(&t, path, PICTURE));
        self.finish(name, Outcome { reused, seconds, file, picture, summary: Some(summary) });
        Ok(t)
    }
}

/// Build the world `d` describes, keeping its stages in `dir` (and taking from there the ones
/// it has), or in memory with none. `on_progress` is told how it goes, and returns whether to
/// carry on: after `false` the build saves where it is and returns `Stopped`.
pub fn build(d: &Description, dir: Option<&Path>, on_progress: &mut dyn FnMut(&Progress) -> bool) -> Result<World, Stopped> {
    build_with(d, dir, on_progress, CHECKPOINT)
}

fn build_with(d: &Description, dir: Option<&Path>, on_progress: &mut dyn FnMut(&Progress) -> bool, checkpoint: f64) -> Result<World, Stopped> {
    let mut b = Builder::new(dir, on_progress);
    b.checkpoint = checkpoint;
    let world_key = key(&["world", &format!("{:?}", d)]);
    b.manifest = Manifest { format: FORMAT, key: world_key.clone(), seed: d.params.planet.seed, stages: d.stages().iter().map(|n| Stage::pending(n)).collect() };
    b.manifest_file = Some(format!("world-{}.json", world_key));
    b.save();

    let params = &d.params;
    let radius = params.planet.radius;
    let time = d.time;
    let planet = b.planet(&params.planet)?;
    let climate = b.climate(params.climate.as_ref(), &planet)?;
    let site = b.site(&d.site, &planet)?;
    let frame = Frame::new(site.0, site.1, radius, d.exaggeration);

    // The simulations, coarsest first, each driving the next at its edge. Of the one around
    // the next: its name, and its surface at the cells' corners before it evolved.
    let mut outer: Option<(String, Vec<f64>)> = None;
    let mut built: Vec<Terrain> = vec![];
    if let Some(n) = d.planet_cells {
        let begun = Instant::now();
        let mut gp = params.clone();
        // At the planet's cells, kilometres wide, the ice can't drain down valleys.
        gp.glacier = None;
        let (key, reads) = b.chain(&["planet", &format!("{:?} {}", gp, n)], &["plates", "climate"]);
        let t = Terrain::new(Domain::sphere(n, radius), gp);
        outer = Some(("planet".to_string(), scenery::corner_heights(&t)));
        built.push(b.evolve("planet", &key, reads, t, time, begun)?);
    }
    let finest_half = d.levels.first().map_or(0., |l| l.cells as f64 * l.spacing / 2.);
    let centre = d.focus.centre(&frame, 0.5 * finest_half);
    let (fx, fz) = frame.tangent(&centre);
    for (k, l) in d.levels.iter().enumerate().rev() {
        let begun = Instant::now();
        let name = d.patch_name(k);
        let width = l.cells as f64 * l.spacing;
        let snap = |c: f64| (c / (width / 8.)).round() * (width / 8.);
        let (lat, lon) = Domain::lat_lon(&frame.dir(snap(fx), snap(fz)));
        let mut lp = params.clone();
        if k > 0 {
            // Glaciers and meandering channels are both finer than a coarse patch's cells:
            // they only run on the finest, where they are drawn.
            lp.glacier = None;
            lp.channels = None;
        }
        let own = format!("{:?} {:?} {:?} {} {:?} {:?}", lp, lat, lon, l.cells, l.spacing, time);
        let around: Vec<&str> = outer.iter().map(|(n, _)| n.as_str()).chain(["plates", "climate"]).collect();
        let (key, reads) = b.chain(&["patch", &own], &around);
        let mut t = Terrain::new(Domain::patch(lat, lon, l.cells, l.spacing, radius), lp);
        let initial = scenery::corner_heights(&t);
        if let (Some(around), Some((_, before)), true) = (built.last(), &outer, time > 0.) {
            // The edge follows the change the simulation around it went through there.
            let after = scenery::corner_heights(around);
            t.boundary_rate = (0..t.domain.len()).map(|i| {
                if !t.domain.boundary[i] { return 0.; }
                let d = t.domain.dirs[i];
                (scenery::sample(&around.domain, &after, &d) - scenery::sample(&around.domain, before, &d)) / time
            }).collect();
        }
        outer = Some((name.clone(), initial));
        built.push(b.evolve(&name, &key, reads, t, time, begun)?);
    }
    let mut patches = built;
    let globe = d.planet_cells.map(|_| patches.remove(0));
    patches.reverse();
    let manifest_path = b.manifest_file.as_deref().and_then(|f| b.store.path(f));
    Ok(World { site, frame, planet, climate, globe, patches, manifest: b.manifest, manifest_path })
}

/// What `survey` finds of a world.
pub struct Survey {
    pub planet: Planet,
    pub climate: Option<Arc<Climate>>,
    /// The site's latitude and longitude, degrees.
    pub site: (f64, f64),
}

/// The planet, its climate and the site of a world, without its landscape: what the weather
/// reads of one. Taken from `dir` where it has them, and kept there, as `build` does (but with
/// no manifest: this is part of a build, not one).
pub fn survey(params: &TerrainParams, site: &Site, dir: Option<&Path>, on_progress: &mut dyn FnMut(&Progress) -> bool)
    -> Result<Survey, Stopped>
{
    let mut b = Builder::new(dir, on_progress);
    let planet = b.planet(&params.planet)?;
    let climate = b.climate(params.climate.as_ref(), &planet)?;
    let site = b.site(site, &planet)?;
    Ok(Survey { planet, climate, site })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::tectonics::TectonicsParams;

    /// A world small enough to build in a second or two.
    fn tiny(seed: u64) -> Description {
        let mut params = TerrainParams::default();
        params.planet.seed = seed;
        params.planet.tectonics = Some(TectonicsParams { cells: 12, age: 20., ..TectonicsParams::default() });
        params.climate = Some(ClimateParams { cells: 8, warming: -8., ..ClimateParams::default() });
        params.step = 500.;
        Description {
            params, site: Site::Mountains, time: 3000., planet_cells: Some(6),
            levels: vec![Level { cells: 16, spacing: 1000. }, Level { cells: 12, spacing: 5000. }],
            focus: Focus::Camera { location: Vector3::new(0., 3000., 0.), lookat: Vector3::new(0., 0., -1000.) },
            exaggeration: 1.,
        }
    }

    fn directory(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("world-test-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn quietly(d: &Description, dir: Option<&Path>) -> World {
        build(d, dir, &mut |_| true).unwrap()
    }

    fn same(a: &World, b: &World) {
        assert_eq!(a.site, b.site);
        assert_eq!(a.patches.len(), b.patches.len());
        for (x, y) in a.patches.iter().chain(&a.globe).zip(b.patches.iter().chain(&b.globe)) {
            assert_eq!(x.time, y.time);
            assert_eq!(x.bed, y.bed);
            assert_eq!(x.ice, y.ice);
            assert_eq!(x.sediment, y.sediment);
            assert_eq!(x.drainage, y.drainage);
        }
    }

    #[test]
    fn test_keys_are_fnv_and_keep_their_parts_apart() {
        // FNV-1a of "1" (the format), a zero byte and "a", worked by hand from the definition.
        let mut h: u64 = 0xcbf29ce484222325;
        for b in [b'1', 0, b'a'] { h = (h ^ b as u64).wrapping_mul(0x100000001b3); }
        assert_eq!(key(&["a"]), format!("{:016x}", h));
        assert_ne!(key(&["ab", "c"]), key(&["a", "bc"]));
        assert_ne!(key(&["a", ""]), key(&["a"]));
    }

    #[test]
    fn test_progress_is_data_a_server_can_stream() {
        let e = Progress { stage: "planet".to_string(), fraction: 0.25, message: None, picture: None };
        let json = serde_json::to_string(&e).unwrap();
        assert_eq!(json, r#"{"stage":"planet","fraction":0.25}"#);
        assert_eq!(serde_json::from_str::<Progress>(&json).unwrap(), e);
    }

    #[test]
    fn test_manifest_lists_what_the_world_is_made_of() {
        let dir = directory("manifest");
        let d = tiny(11);
        let mut events = vec![];
        let w = build(&d, Some(&dir), &mut |e| { events.push(e.clone()); true }).unwrap();
        let names: Vec<&str> = w.manifest.stages.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["plates", "climate", "site", "planet", "patch 5000 m", "patch 1000 m"]);
        assert!(w.manifest.done());
        // What is on disk is what the build returned, and every stage's files are there.
        let path = w.manifest_path.clone().unwrap();
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), format!("world-{}.json", w.manifest.key));
        assert_eq!(Manifest::read(&path).unwrap(), w.manifest);
        for s in &w.manifest.stages {
            assert!(s.key.is_some() && s.summary.is_some(), "{:?}", s);
            assert!(dir.join(s.file.as_ref().unwrap()).exists(), "{:?}", s);
            if s.name != "site" { assert!(dir.join(s.picture.as_ref().unwrap()).exists(), "{:?}", s); }
        }
        // A stage reads the ones its key was made from.
        assert_eq!(w.manifest.stage("patch 1000 m").unwrap().reads, ["patch 5000 m", "plates", "climate"]);
        assert_eq!(w.manifest.stage("planet").unwrap().reads, ["plates", "climate"]);
        // Events: each stage from 0 to 1, in order, fractions never falling within a stage.
        for pair in events.windows(2) {
            assert!(pair[0].stage != pair[1].stage || pair[0].fraction <= pair[1].fraction, "{:?}", pair);
        }
        for n in names {
            let of: Vec<&Progress> = events.iter().filter(|e| e.stage == n).collect();
            assert_eq!(of.first().unwrap().fraction, 0.);
            assert_eq!(of.last().unwrap().fraction, 1.);
            assert!(of.last().unwrap().message.is_some());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_second_build_reuses_everything_and_is_the_same_world() {
        let dir = directory("reuse");
        let d = tiny(12);
        let memory = quietly(&d, None);
        assert!(memory.manifest_path.is_none() && memory.manifest.stages.iter().all(|s| s.file.is_none()));
        let first = quietly(&d, Some(&dir));
        assert!(first.manifest.stages.iter().filter(|s| s.name != "plates" && s.name != "climate").all(|s| !s.reused));
        let files = |dir: &Path| { let mut f: Vec<_> = std::fs::read_dir(dir).unwrap().flatten().map(|e| (e.file_name(), e.metadata().unwrap().modified().unwrap())).collect(); f.sort(); f };
        let before = files(&dir);
        let second = quietly(&d, Some(&dir));
        assert!(second.manifest.stages.iter().all(|s| s.reused && s.status == Status::Done), "{:?}", second.manifest);
        // The seconds a reused stage reports are those it took to build.
        assert_eq!(second.manifest.stage("patch 1000 m").unwrap().seconds, first.manifest.stage("patch 1000 m").unwrap().seconds);
        // Nothing but the manifest was written again.
        let after = files(&dir);
        let unchanged = |f: &&(std::ffi::OsString, std::time::SystemTime)| !f.0.to_str().unwrap().starts_with("world-");
        assert_eq!(before.iter().filter(unchanged).collect::<Vec<_>>(), after.iter().filter(unchanged).collect::<Vec<_>>());
        same(&memory, &first);
        same(&memory, &second);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_the_planet_and_climate_come_back_from_their_files_to_the_bit() {
        let dir = directory("planet");
        let d = tiny(13);
        let w = quietly(&d, Some(&dir));
        let store = Store::open(Some(&dir));
        let radius = d.params.planet.radius;
        let stage = w.manifest.stage("plates").unwrap();
        let saved = store.read::<CrustState>(stage.file.as_ref().unwrap(), stage.key.as_ref().unwrap()).unwrap();
        let crust = Crust::from_state(saved.state, 12, radius).unwrap();
        let ours = w.planet.crust.as_ref().unwrap();
        assert_eq!(serde_json::to_string(&crust.state()).unwrap(), serde_json::to_string(&ours.state()).unwrap());
        let stage = w.manifest.stage("climate").unwrap();
        let saved = store.read::<ClimateState>(stage.file.as_ref().unwrap(), stage.key.as_ref().unwrap()).unwrap();
        let climate = Climate::from_state(saved.state, d.params.climate.clone().unwrap(), radius).unwrap();
        let ours = w.climate.as_ref().unwrap();
        for dir in [Vector3::new(1., 0., 0.), Vector3::new(0.3, 0.8, -0.5).normalize(), Vector3::new(-0.6, -0.2, 0.7).normalize()] {
            assert_eq!(crust.elevation(&dir), w.planet.crust.as_ref().unwrap().elevation(&dir));
            assert_eq!(crust.plate_at(&dir), w.planet.crust.as_ref().unwrap().plate_at(&dir));
            let (a, b) = (climate.local(&dir, 100.), ours.local(&dir, 100.));
            assert_eq!((a.mean, a.summer, a.winter, a.precipitation), (b.mean, b.summer, b.winter, b.precipitation));
            assert_eq!(climate.continentality(&dir), ours.continentality(&dir));
        }
        // A file of another key, or of another grid, isn't taken for this one.
        assert!(store.read::<CrustState>(w.manifest.stage("plates").unwrap().file.as_ref().unwrap(), "0000000000000000").is_none());
        let saved = store.read::<CrustState>(w.manifest.stage("plates").unwrap().file.as_ref().unwrap(), w.manifest.stage("plates").unwrap().key.as_ref().unwrap()).unwrap();
        assert!(Crust::from_state(saved.state, 16, radius).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_stopped_build_resumes_to_the_same_world() {
        let dir = directory("resume");
        let d = tiny(14);
        // Stop half way through the finest patch.
        let stopped = build(&d, Some(&dir), &mut |e| !(e.stage == "patch 1000 m" && e.fraction >= 0.5)).err().unwrap();
        assert_eq!(stopped, Stopped { stage: "patch 1000 m".to_string() });
        let path = dir.join(format!("world-{}.json", key(&["world", &format!("{:?}", d)])));
        let m = Manifest::read(&path).unwrap();
        let s = m.stage("patch 1000 m").unwrap();
        assert_eq!((s.status, s.fraction), (Status::Stopped, 0.5));
        assert_eq!(m.stage("patch 5000 m").unwrap().status, Status::Done);
        let part = format!("patch-{}-1500.part.json", s.key.as_ref().unwrap());
        assert!(dir.join(&part).exists());
        // And at a stage's start: nothing of it is begun.
        let again = build(&d, Some(&dir), &mut |e| e.stage != "patch 5000 m").err().unwrap();
        assert_eq!(again.stage, "patch 1000 m");
        // Taken up: the stages before are reused, the patch goes on from its checkpoint.
        let mut first = None;
        let w = build(&d, Some(&dir), &mut |e| { if e.stage == "patch 1000 m" && e.fraction > 0. && first.is_none() { first = Some(e.fraction); } true }).unwrap();
        assert!(first.unwrap() > 0.5, "{:?}", first);
        assert!(w.manifest.done());
        assert!(w.manifest.stage("patch 5000 m").unwrap().reused);
        assert!(!w.manifest.stage("patch 1000 m").unwrap().reused);
        assert!(!dir.join(&part).exists(), "the checkpoint is replaced by the result");
        same(&w, &quietly(&d, None));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_checkpoints_are_written_as_a_stage_runs() {
        let dir = directory("checkpoint");
        let d = tiny(15);
        // A checkpoint after every step: each replaces the one before, and the result them all.
        let mut most = 0;
        let w = build_with(&d, Some(&dir), &mut |_| {
            most = most.max(std::fs::read_dir(&dir).unwrap().flatten().filter(|e| e.file_name().to_str().unwrap().ends_with(".part.json")).count());
            true
        }, 0.).unwrap();
        assert!(most >= 1 && most <= 3, "{}", most);
        assert!(std::fs::read_dir(&dir).unwrap().flatten().all(|e| !e.file_name().to_str().unwrap().contains(".part.")));
        same(&w, &quietly(&d, None));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_a_change_rebuilds_from_the_stage_it_touches() {
        let dir = directory("tweak");
        let d = tiny(16);
        let first = quietly(&d, Some(&dir));
        // Glaciers only run on the finest patch: nothing else reads their parameters.
        let mut colder = d.clone();
        colder.params.glacier.as_mut().unwrap().ela_amplitude += 100.;
        let w = quietly(&colder, Some(&dir));
        for s in &w.manifest.stages {
            assert_eq!(s.reused, s.name != "patch 1000 m", "{:?}", s);
            let was = first.manifest.stage(&s.name).unwrap();
            assert_eq!(s.key == was.key, s.name != "patch 1000 m", "{:?}", s);
        }
        assert_ne!(w.manifest.key, first.manifest.key);
        // The rivers run everywhere: every simulation is rebuilt, the planet and its climate
        // are not.
        let mut wetter = d.clone();
        wetter.params.fluvial.k *= 2.;
        let w = quietly(&wetter, Some(&dir));
        for s in &w.manifest.stages {
            assert_eq!(s.reused, ["plates", "climate", "site"].contains(&s.name.as_str()), "{:?}", s);
        }
        // Later in time: the planet's simulation goes on from where it was, the patches (whose
        // edges follow it over the whole time) start again.
        let mut later = d.clone();
        later.time = 4000.;
        let mut from = None;
        let w = build(&later, Some(&dir), &mut |e| { if e.stage == "planet" && e.fraction > 0. && from.is_none() { from = Some(e.fraction); } true }).unwrap();
        assert_eq!(from, Some(0.875), "from 3000 years, the first step ends at 3500 of 4000");
        assert_eq!(w.manifest.stage("planet").unwrap().key, first.manifest.stage("planet").unwrap().key);
        same(&w, &quietly(&later, None));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_survey_shares_the_worlds_planet() {
        let dir = directory("survey");
        let d = tiny(17);
        let Survey { planet, climate, site } = survey(&d.params, &d.site, Some(&dir), &mut |_| true).unwrap();
        let w = quietly(&d, Some(&dir));
        assert_eq!(site, w.site);
        assert!(Arc::ptr_eq(planet.crust.as_ref().unwrap(), w.planet.crust.as_ref().unwrap()));
        assert!(Arc::ptr_eq(climate.as_ref().unwrap(), w.climate.as_ref().unwrap()));
        assert!(w.manifest.stage("site").unwrap().reused);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
