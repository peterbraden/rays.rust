# Animation: design

A proposal for how scenes change with time. Nothing here is implemented yet; it records how
time works today, what stops a frame from being rendered on its own, and a plan in small steps.

## Goals

1. **Frames in isolation.** A frame is a pure function of the scene file, its seed and the
   frame's time. Frame 517 rendered alone, on another machine, out of order or after a crash is
   bit-for-bit the frame a sequential render would have made. Caches only save time; they never
   change the result.
2. **One clock, many rates.** The scene has one time. Each thing that depends on it sees that
   time scaled (and offset) by its own factor, so clouds can boil at 60× while the sea moves at
   1×, in the same shot.
3. **Camera on a spline.** The camera follows a smooth path, at a controlled speed, with motion
   blur along the curve rather than along a chord.

## How time works now

There are two unrelated notions of time:

- **Motion time** (`src/motion.rs`): a thread-local per-sample value in [0, 1] across the
  shutter. `"motion"` on a transform or the camera gives the pose at 1; `AnimatedTransform` and
  `camera::Placement` interpolate. This is motion blur within one image, not animation.
- **Object time**: a `"time"` key, in seconds, read once when the object is built:
  `ocean` (wave phases), `hogan_kew_cirrus` (advection and `evolution_period`), `lenticular`
  (texture flowing through the standing plates), `harris_cumulus` (how long to simulate),
  `fireworks`, and `landscape` (years). `scripts/animate.py` writes a copy of the scene per
  frame with every object's `"time"` set to the same value.

Everything else is static: `nubis_clouds` (and its global weather map), `benard_altocumulus`,
`precipitation`, `mammatus`, grass (its `wind` bends blades but doesn't sway them), and the
sky's `sun_direction`.

### What already breaks isolation

- **Simulations depend on where they were stopped.** `CloudSimulation::advance_to` takes
  `dt = stable_step().min(time - self.time)`: the last step is cut short to land on the frame's
  time. Simulating straight to 3600 s takes different steps from simulating to 1800 s (a cached
  frame) and then on to 3600 s. The trajectories differ, and convection is chaotic, so they
  diverge; the perturbation seed also follows the step count (`seed + steps * 7919`), which
  differs too. The cache therefore changes the picture, and a frame's clouds depend on which
  frames were rendered before it. `terrain::Simulation::run_to` has the same truncated last step
  (milder: no chaos, fixed step).
- **Every frame uses the same sample pattern.** Each frame is rendered with the scene's seed,
  so its noise is the same screen-space pattern in every frame: it reads as dirt on the lens
  rather than as grain.
- **The resume key** (`render_key`) hashes the scene file and overrides; it is fine while every
  frame is its own file, but a `--time` override must be part of it.

## Design

### The scene clock

The scene gains an `"animation"` block and a time in seconds, `t`:

```json
"animation": { "fps": 24, "frames": 240, "start": 0, "shutter": 0.5 }
```

A frame is chosen on the command line, never by editing the file:

- `--frame N`: `t = start + N / fps`.
- `--time T`: any time, for stills and tests.
- `-o frames/%04d.png`: the frame number substituted into the output name.

Without either, `t = start` (0), so every existing scene renders as it does today.

### Clocks: rates with units

The scene's times span sixteen orders of magnitude: the shutter is open for a fiftieth of a
second, waves take seconds, clouds grow over an hour, glaciers advance and retreat over
100,000 years (`glacier.rs`'s cycle), and the landscape is millions of years old. One number
can't serve them all, so each object reads its own **clock**, which has its own units and
rate.

- **Video time** `t` is seconds of the finished film: the camera, keyframes and the shutter use
  it.
- **An object's native unit** is the one its `"time"` key already has: seconds for the ocean,
  clouds, fireworks and the sky; years for `landscape` (and glaciers, rivers and volcanoes in
  it).
- **A rate** converts video seconds to the object's own time, written with units so it reads
  the same whatever the object: `"1 s/s"` (real time, **the default for everything**),
  `"1 min/s"`, `"2 kyr/s"`, `"0"` (frozen). Units are `s`, `min`, `h`, `day`, `yr`, `kyr`,
  `Myr`.

An object's local time is its `"time"` (where it is at the first frame, in its own unit) plus
its clock's elapsed time since the first frame, converted to that unit:

```
local = time + elapsed(clock, t)          elapsed = rate * (t - start) for a constant rate
```

With the default of 1 s/s, a landscape advances 3 × 10⁻⁸ years per second, so it is still,
as it should be, without being told to freeze. To watch glaciation, give the landscape a
geological clock:

```json
"clocks": { "geology": "2 kyr/s", "weather": "1 min/s" },
"objects": [
  {"type": "landscape", "time": 1e6, "clock": "geology"},   // one glacial cycle in 50 s
  {"type": "harris_cumulus", "time": 1800, "clock": "weather"},
  {"type": "ocean"}                                          // real time
]
```

An object takes `"clock": "name"` (a shared clock defined in `"clocks"`) or an inline
`"clock": "500 yr/s"`. Shared clocks keep everything that should move together (all the
weather) at one rate, tuned in one place. Existing scenes keep their meaning: `"time"` is still
where the object is at the first frame. The clock is worked out before the object's own parser
runs, so the object only ever sees a plain `"time"` in its own unit.

#### Ramps

A clock's rate can itself be keyed, for the shot that starts in real time and accelerates
into a time-lapse:

```json
"clocks": { "geology": {"rate": [[0, "0 yr/s"], [4, "0 yr/s"], [10, "5 kyr/s"]], "interpolation": "log"} }
```

Across many orders of magnitude the rate is interpolated in log space (a constant factor per
second, which looks like a steady acceleration), and a rate of zero is approached from a tiny
floor. `elapsed` is the integral of the rate, which for linear or exponential pieces has a closed
form, so any frame's local time is computed directly, without stepping through earlier frames.

#### Clocks that don't mix

Two things go wrong when one clock is far faster than the frame rate:

- **Simulations land between their steps.** At 2 kyr/s and 24 fps a frame is 83 years, and a
  landscape steps every 500 years: most frames fall inside a step. At 1 s/s a frame is
  3 × 10⁻⁸ years, and every frame of a shot falls inside the same step. Taking a partial step
  for each frame would be slow (a landscape step can take seconds) and pointless. Instead a
  simulated object shows the state **interpolated between the two grid states either side of
  its local time** (rule 2 below): heights, ice thickness, water and snow blended linearly.
  When the whole shot falls within one step, the step is simulated once and reused from the
  cache by every frame, at no further cost.
- **Fast clocks strobe.** If a cloud field changes completely between frames (clouds at
  `"1 kyr/s"`), each frame shows unrelated weather, which flickers. That's the user's choice of
  rate, but the renderer warns when a clock advances an object by more than its correlation
  time per frame (for clouds, the time a cloud lives; for waves, a wave's period). A real
  time-lapse photographer would average with a long exposure; here the cheap equivalent is to
  slow that object's clock.

The camera, keyframed values and the shutter always run in video seconds, so camera moves stay
smooth whatever the clocks are doing.

### Animated values (keyframes)

Most things worth animating are just parameters: the sun's direction, cloud `coverage`, a
light's `power`, the camera's `angle` or `focus_distance`, a colour. Rather than teach every
parser about time, any number or vector in the scene can be given as keys:

```json
"coverage": {"keys": [[0, 0.2], [10, 0.7]], "interpolation": "smooth"}
```

Before the scene is parsed, a single pass walks the JSON and replaces each keyed value with its
value at `t` (keys are in scene seconds; `"linear"`, `"smooth"` (Catmull-Rom), `"step"`; held
constant outside the keys). This is a JSON-to-JSON transform: it works for every parameter
the renderer has, now and later, and the frame's resolved scene can be dumped with
`--dump-scene` to see exactly what was rendered.

Transforms are the exception that want more: a keyed `translate`/`rotate` is resolved at the
shutter's opening and closing, and the two become the existing `"motion"` start and end, so
animated objects get motion blur from `AnimatedTransform` for free.

### Time within a frame: the shutter

`motion::time()` stays the per-sample fraction of the shutter, in [0, 1]. The frame's shutter
opens at `t` and stays open for `shutter / fps` seconds (`"shutter": 0.5` is a 180° shutter).
What follows a sample's time:

- the camera on its path (below), evaluated at the exact time, so the blur follows the curve;
- transforms, between their poses at the shutter's opening and closing;
- not procedural fields (clouds, ocean, grass). Their state is built once per frame, at the
  shutter's centre; clouds and waves move little in 1/48 s, and rebuilding an ocean's FFT or a
  cloud simulation per sample is out of the question.

### Camera paths

`"path"` on the camera replaces `location` (and optionally `lookat`) with splines:

```json
"camera": {
  "path": {
    "location": [[0, 2, -10], [5, 3, -5], [8, 6, 4], [4, 10, 12]],
    "lookat": "ahead",                       // or a point, or its own list of points
    "up": [0, 1, 0], "bank": 0.5,
    "duration": 12, "speed": "constant", "ease": [1, 1]
  },
  "angle": 0.6
}
```

- **Curve:** centripetal Catmull-Rom through the points (no cusps or overshoot on uneven
  spacing, unlike the uniform kind), with optional explicit times per point; cubic B-spline as
  an option for smoother, approximating paths.
- **Speed:** by default the curve is reparameterised by arc length (a table of length against
  parameter, inverted by binary search), so the camera moves at constant speed however the
  points are spaced; `ease` adds ease-in and ease-out (seconds) at the ends.
- **Orientation:** `lookat` a fixed point, its own spline (the target moving along a second
  path), or `"ahead"` (along the tangent, a fly-through). Up comes from `up`, with `bank`
  rolling into turns in proportion to curvature, like an aircraft. A rotation-minimising
  frame (double reflection, Wang et al. 2008) for `"up": "path"` avoids flips on loops.
- **Implementation:** a `Spline` in a new `src/animation/` module, and a
  `camera::Placement::Path` variant that evaluates at `t + motion::time() * shutter / fps`.
  `Placement` already resolves the frame per sample, so the cameras don't change.

### Determinism rules

What every time-dependent object must obey, and what the tests check:

1. **No state carried between frames.** An object at time `t` is built from the scene file
   and `t` alone.
2. **Simulations step on a fixed grid.** A simulation's step sequence must not depend on where
   it is stopped. It advances only through whole intervals between canonical checkpoints, laid
   from its time zero in its own unit (say every 60 s for clouds, every `step` years for a
   landscape). Within an interval the adaptive steps are chosen as now (the glaciers'
   `min_step`/`max_step`, the clouds' CFL step), but the interval always ends exactly on its
   checkpoint. Cache entries are saved only at checkpoints. To show local time `τ`, the
   checkpoints either side of `τ` are simulated (or loaded) and blended. Starting from a cache
   or from scratch then follows the same steps to the same bits; the perturbation seed follows
   the step count, which is now the same either way.

   Blending suits slow, smooth processes (landscapes, ice). For clouds, whose checkpoints are a
   minute apart and whose structure moves, a blend would cross-fade rather than move; there
   the copy at the checkpoint before `τ` is advanced to `τ` and thrown away (deterministic,
   since it always starts from the same checkpoint).
3. **Per-frame seeds.** Sample seeds are `hash(seed, frame)` by default, so noise is
   decorrelated frame to frame (grain, not a fixed pattern); `"animation": {"lock_noise": true}`
   keeps the scene's seed for every frame when a static pattern is wanted (it flickers less
   at low sample counts). Procedural seeds (`"seed"` on clouds, trees, terrain) never change
   with the frame: the world is the same world.
4. **Parallel reductions are order-independent** (max, or sums per fixed chunk, as
   `harris_cumulus` does now), so thread count never changes a result.
5. **The resume key** includes the frame's time.

## Making each thing move

Once the clock exists, objects gain motion one at a time, each through its local time:

| Object | Motion |
| --- | --- |
| `ocean` | already: `time`. |
| `hogan_kew_cirrus`, `lenticular` | already: `time`. |
| `harris_cumulus` | already, once simulations step on the grid (rule 2). |
| `landscape` | already (years); usually `time_scale: 0`. |
| `nubis_clouds` | new: drift by `wind` (m/s) and evolve by advancing the noise's 4th coordinate or scrolling the detail noise faster than the shape noise; the global weather map advected by the planet's wind field. |
| `benard_altocumulus`, `mammatus`, `precipitation` | new: drift with `wind`; cells slowly evolve. |
| grass and trees | new: sway as a sum of a few sinusoids per blade or branch, phase from position and gusts travelling with the wind field; geometry rebuilt per frame (it is procedural already). |
| sky | the sun's position from the time of day, on its own clock (below); or `sun_direction` by keyframes. |

## Rendering animations

- `rays scene.json --frame N -o out/%04d.png` renders one frame; `scripts/animate.py` becomes a
  loop (or a job list for many machines) over frames, and ffmpeg. It no longer rewrites the
  scene.
- Later, `--frames A..B` may render a range in one process and keep the static parts of the
  scene (forests, terrain BVHs, meshes) between frames. It must give the same images as
  separate runs, which a test can check.

## Steps

1. Scene clock: `"animation"`, `--frame`, `--time`, `%04d` outputs, per-frame seeds, resume
   key. `animate.py` uses `--frame`.
2. Clocks: rates with units, `"clocks"` and `"clock"`, each object's native unit, resolved onto
   objects' `"time"`; then keyed rates (ramps) with closed-form `elapsed`.
3. Simulations on the checkpoint grid (`harris_cumulus`, `landscape` and its glaciers), with
   blending between checkpoints for the landscape; a test that a frame from cache equals one
   from scratch. A glaciation demo: a valley through one glacial cycle at a few kyr/s.
4. Keyframed values (the JSON pass), with transforms resolved into `"motion"`.
5. Camera paths: `Spline`, arc-length speed, `lookat` modes, banking, `Placement::Path`.
6. Isolation tests: frame N alone equals frame N after frames 0..N-1; different thread counts
   give identical images.
7. Motion for the static objects, one per change (nubis first).

## Open questions

- Lock noise across frames by default, or decorrelate? Decorrelated is closer to film, and
  denoisers and temporal filters prefer it.
- Is a keyframed value in video seconds or in the object's own clock? Video seconds is
  simpler to reason about across objects; the object's clock would let a glacier's keys (an
  ELA, say) be written in years.
- How fine must the landscape's checkpoints be for smooth glaciation? Blending is linear in
  ice thickness; a glacier front advancing far within one step would fade in rather than
  advance. The step may need to follow the clock's rate (a few frames per checkpoint), which
  would make it part of the cache key.
