# Plan

Eight workstreams. Each lists what we know (verified in the code or by measurement) separately
from what we are assuming, and the questions that need answering before work starts.

Status: planning. Nothing below is implemented.


## Process

This is a learning project. Each workstream begins with a literature review of the classic
papers on the technique:

1. Compile a reading list, verifying each reference (authors, title, venue, year, a working
   link) and that it says what we claim it says.
2. Implement, citing the papers in literate docstrings and in a "Reading for review" section of
   the PR. You read them while reviewing; work doesn't wait on the reading.

The code is literate: module and item docstrings discuss the papers alongside the
implementation. They explain the technique, give the key definitions and equations, map the
paper's symbols to the code's variables, note where the code simplifies or departs from the
paper, and keep open questions visible. `src/ocean.rs` (Tessendorf) is the model.

The reading lists below are candidates from memory and are unverified until step 1 checks
them.


## Decisions

- Point lights switch to physically based inverse-square falloff, as the only behaviour.
  Existing scenes with point lights need their intensities retuned (this includes demo scenes,
  so their PNGs need re-rendering afterwards).
- Reference images are downloaded from published sources, rather than rendered locally with
  another renderer. Prefer sources that publish the scene description with the image (eg.
  Bitterli's rendering resources), so parameters can be matched.
- Large assets (meshes, reference images) are fetched by a script, with each asset's source and
  licence recorded. Scene files and small references are committed.
- GPU: the goal is both an interactive preview and faster final renders, preview first.


## 1. Standard test scenes and reference comparisons

Goal: render well-known scenes from the graphics literature in our scene format, keep reference
images from established renderers alongside them, and diff our output against those references
to catch regressions.

### Source survey (full notes: survey written during planning, summarised here)

Verified:

- Bitterli's Rendering Resources: each Tungsten zip contains the full `scene.json`, a
  `LICENSE.txt`, a linear half-float EXR reference and a tone-mapped PNG. The PNGs use Tungsten's
  "filmic" curve, so only the EXRs are usable for diffs. Tungsten's `fov` is horizontal.
  - Cornell box: CC0, 1024x1024, 3.4 MB, EXR reference. Mitsuba 3 and pbrt-v4 conversions ship
    the same EXR.
  - Veach MIS: CC0, 1280x720 EXR; needs Beckmann microfacet copper, sphere emitters specified by
    power, and multiple importance sampling.
  - Other small scenes are CC0 but need features we lack (photon-mapped caustics, participating
    media, microfacet metals, environment maps, textures); the teapot environment map is
    non-commercial.
- Original Cornell data: measured geometry, spectral reflectance and light emission, camera data
  (vertical FOV about 39.3 degrees), photographs (TIFF, EXR). No licence beyond a copyright
  notice, so terms are unclear.
- pbrt-v4-scenes: publishes reference EXR and PNG renders (renderer not stated; presumably
  pbrt-v4). About 7 GB, per-scene licences.
- Haines' SPD: code CC0 (MIT fallback) on GitHub, with 512x512 POV-Ray PNGs; render settings
  are unpublished and NFF light intensity is undefined, so references are qualitative only.
- Stanford scans: PLY; bunny 69k triangles (4.9 MB), dragon 1.13M, Happy Buddha 1.09M. Research
  use or free redistribution with credit; commercial use needs permission. The dragon and Buddha
  must not be subjected to Boolean operations. No reference images.
- Ray Tracing in One Weekend: CC0. The final scene is not reproducible (unseeded `rand()`, JPEG
  reference, gamma 2 output). The earlier "view-distant" image (`img-1.20`, 400x225 PNG) is
  deterministic.
- Furnace test: pbrt-v4's `src/pbrt/cpu/integrators_test.cpp` (Apache-2.0) asserts that inside a
  unit sphere of diffuse reflectance 0.5, with a point light of intensity pi at its centre,
  radiance averages 1: (pi / r^2 x 0.5 / pi) direct, divided by (1 - 0.5) for interreflection.
  This relies on inverse-square point lights (#62).
- NVIDIA FLIP: BSD-3-Clause; a command-line tool reading EXR and PNG, and a Python package
  (`flip-evaluator`). The Rust crate `nv-flip` (MIT/Apache-2.0/Zlib) accepts only 8-bit images,
  so it handles LDR comparisons only.

### First three scenes (recommended by the survey)

1. Furnace tests from pbrt-v4: closed-form answers, nothing to download or license. Needs a
   float framebuffer readout (radiance 1.0 sits at the 8-bit clipping point), and correct
   shading of surfaces seen from inside (to verify: `scatter_lambertian` and `diffuse` use the
   outward normal, so the inside of a sphere may currently render black).
2. Bitterli Cornell box (CC0, linear EXR): buildable now by baking its quads and cubes into an
   OBJ, with the light as a diffuse-light quad. Needs Tungsten's horizontal FOV, an EXR reader in
   the harness, and ideally area-light sampling (the light is small, so convergence is slow).
   The reference's sample count is unknown, so compare with tolerance.
3. Ray Tracing in One Weekend "view-distant": spheres with Lambertian, fuzzy metal and
   dielectric. Needs a gradient sky background, a gamma-2 output transfer, an index of
   refraction below 1 (or nested media) for the air bubble, and the book's vertical FOV
   convention.

Later: Veach MIS (microfacet BRDFs, MIS), pbrt-v4 `killeroo-simple` (translate/scale, coated
material, area light), Stanford meshes (performance), SPD (qualitative).

### Comparison approach

- Diff images with a perceptual metric (NVIDIA FLIP) and plain RMSE, with tolerances, as Monte
  Carlo noise means output never matches exactly.
- Analytic tests (furnace, a single Lambertian plane under a light) can run in `cargo test`;
  scene and reference comparisons are slower, so they run as a separate harness or in CI.

### Known

- Point lights have no inverse-square falloff (`material/functions.rs`, `diffuse`). Every
  physically based reference renderer has falloff, so lit scenes cannot match references without
  changing this, or adding falloff as an option.
- There are no area lights or quad primitives. The Cornell box needs a rectangular emitter
  (emissive geometry works for this, but converges slowly without light sampling).
- Our material models differ from other renderers' in detail (eg. Plastic is a Fresnel coat with
  probabilistic selection; metal fuzz follows Ray Tracing in One Weekend, not a microfacet model).
  Only Lambertian, perfect mirrors and dielectrics are likely to match closely.
- Output is now linear with sRGB encoding (#59), which is the convention the references use.

### Assumptions to verify

- That published references come with enough detail (camera, exposure, tone mapping, sample
  count) to reproduce them. Where exposure is unknown, the diff fits a single exposure scale
  before comparing; where tone mapping is unknown, the scene is unsuitable.
- That each asset's licence permits redistribution from our fetch script, or at least linking
  to the original. Stanford models permit research use; Bitterli's scenes carry individual CC
  licences.
- That published references are converged (low noise); if not, tolerances must allow for noise
  in both images.

### Steps

1. Inverse-square falloff for point lights; retune existing scenes.
2. Analytic tests in `cargo test`: furnace test, single plane under a light. Done: the
   furnace test (#68); the plane under a light is covered by the falloff test (#62).
3. Survey sources and licences; pick the first three scenes (likely Cornell box, a Stanford
   mesh, an SPD scene) where published references come with their parameters.
4. Fetch script (`scripts/fetch-assets`), recording source and licence per asset.
5. Comparison harness: render, fit exposure, compute FLIP and RMSE, write a diff image, fail
   above a tolerance. References are linear EXR, so the harness needs either our own EXR output
   (an `ImageWriter` for `.exr`, which the output stage from #59 allows) or must sRGB-encode the
   reference before an 8-bit comparison. HDR FLIP needs the command-line tool or Python package.
6. Primitives the chosen scenes need (quad and area lights for the Cornell box; see 4).

Status: steps 1-5 done for the furnace (#68) and the Cornell box (#69-#72). Against Tungsten's
Cornell box our exposure fit is 0.9914 (absolute brightness within 1%), with the remaining
error mostly noise; the systematic part (about 0.017 RMSE) sits at the light's edges, likely
reconstruction-filter differences. Area-light sampling (#73) cut mean FLIP from 0.078 to 0.018
at 1024 spp; the light-edge residual is now essentially all that remains. Next: the Ray
Tracing in One Weekend scene; a tent reconstruction filter, to test the light-edge
explanation; multiple importance sampling (Veach and Guibas 1995) once glossy materials are
compared.

### Reading (candidates, unverified)

- Goral, Torrance, Greenberg, Battaile, "Modeling the Interaction of Light Between Diffuse
  Surfaces", SIGGRAPH 1984 (origin of the Cornell box).
- Kajiya, "The Rendering Equation", SIGGRAPH 1986.
- Haines, "A Proposal for Standard Graphics Environments", IEEE CG&A 1987 (SPD).
- Andersson et al., "FLIP: A Difference Evaluator for Alternating Images", HPG 2020.
- Pharr, Jakob, Humphreys, "Physically Based Rendering", 4th ed. (point lights, furnace tests).


## 2. Performance review

Goal: find low-cost speed-ups. A profiling pass should come first, as the renders currently
running in the background would distort any measurement now.

### Candidates from the code reviews (before measuring)

- Sky: every ray that escapes integrates atmospheric scattering numerically (16 view samples x 8
  light samples, `skysphere.rs`). The sky depends only on direction, so it could be precomputed
  into a lookup table or environment map.
- Octree traversal (`octree.rs`): collects candidates from every leaf along the whole ray,
  including beyond the nearest hit and beyond `max`; clones a `Vec` per leaf; recomputes
  `1/rd` at every node; then sorts and deduplicates. Front-to-back traversal with early exit
  would cut most intersection tests.
- Octree versus BVH: items spanning octants are duplicated into every child; a BVH built with
  the surface area heuristic is the standard choice and usually much faster for meshes.
- Scene graph octree depth is hard-coded to 2 (`scenefile.rs`).
- `Intersection` holds an `Arc<SceneObject>`, so every hit clones an `Arc` (an atomic increment).
- Triangle intersection calls `panic_if_nan` on vectors in the hot path.
- `rand()` calls `thread_rng()` each time; ocean `randn` builds a `Normal` per call.
- Fireworks intersect their particle octree twice per hit (`FireworkMaterial::scatter`).
- Lambertian light sampling (#60) adds one shadow ray per light per diffuse hit, which made the
  smoke test about 25% slower.
- Everything is `f64`. `f32` halves memory traffic but risks precision problems in large scenes
  such as the ocean and sky, which use planet-scale coordinates.

### Measured (Instruments Time Profiler, Apple M2, release build with symbols, at #64)

Benchmarks are scaled-down copies of demo scenes; render time excludes scene loading.

| Scene | Size, samples | Render | Rays/s |
|---|---|---|---|
| smoke (mesh, box terrain, sky, lights) | 400x200, 8 | 3.2 s | 0.8M |
| spheres | 320x240, 36 | 1.28 s | 7.7M |
| sky | 400x166, 50 | 1.45 s | 7.2M |
| ocean (`fourier_size` 256) | 240x100, 16 | 3.0-4.1 s | 0.22-0.29M |
| fireworks | 200x200, 16 | 1.8-2.1 s | 3.2-3.8M |

Where the time goes:

- Octree candidate collection (`OctreeNode::naive_intersection`) is 85% of the smoke test and
  79% of the ocean, inclusive. It allocates, clones and flattens a `Vec` per node, collects
  every leaf along the whole ray, then sorts and deduplicates; allocation and copying alone are
  20-25% of samples. The triangle tests themselves are about 1%.
- In the sky scene, `exp` is 63% of samples: the atmospheric integral (16 x 8 samples per
  escaped ray).
- Direct-lighting shadow rays are 36% of the smoke test; they use the full nearest-hit search,
  where an any-hit occlusion query would do.
- Ocean scene loading is about 8% of its run (the tile build, including four unused FFTs);
  `Arc` clone and drop per hit is about 6% of the sky scene.

### Candidate changes, by measured payoff (each needs its reading first)

1. Octree traversal without allocation, front to back, stopping at the nearest hit; or a BVH
   (reading: Glassner 1984; Revelles et al., "An Efficient Parametric Algorithm for Octree
   Traversal", 2000, which `octree.rs` has a broken, commented-out attempt at; Goldsmith and
   Salmon 1987; MacDonald and Booth 1990).
2. Sky lookup table (reading: Nishita et al. 1993; Bruneton and Neyret, "Precomputed
   Atmospheric Scattering", 2008; Hillaire, "A Scalable and Production Ready Sky and Atmosphere
   Rendering Technique", 2020).
3. Any-hit occlusion queries for shadow rays.
4. Skip the unused slope FFTs in the ocean build; avoid the `Arc` clone per hit.

### Status

Done: candidates 1-3. A bounding volume hierarchy built with the surface area heuristic
(`bvh.rs`) replaced the octree everywhere (scene, meshes, ocean, CSG unions, box terrain,
fireworks), with any-hit queries for shadow rays and ray ranges passed through `Geometry`; the
sky is a lookup table. The octree (`octree.rs`) is kept, unused, with its own improvements
(Revelles et al. traversal, a limit on items copied by a split, triangle leaves tested four at
a time with vector instructions) for comparison. Rendering on 4 cores, against the octree as it
was: ocean-boat about 16 times faster, the cornell box and demo 6 times, spheres-on-water 5
times. Batched
(SIMD) triangle tests gave nothing with the BVH's small leaves; `-C target-cpu=native` gave
nothing measurable either.

### Reading (candidates, unverified)

- Glassner, "Space Subdivision for Fast Ray Tracing", IEEE CG&A 1984 (octrees).
- Goldsmith and Salmon, "Automatic Creation of Object Hierarchies for Ray Tracing", IEEE CG&A
  1987, and MacDonald and Booth, "Heuristics for Ray Tracing Using Space Subdivision", 1990
  (bounding volume hierarchies, the surface area heuristic).
- Wald, Boulos, Shirley, "Ray Tracing Deformable Scenes Using Dynamic Bounding Volume
  Hierarchies", ACM TOG 2007 (binned SAH construction).
- Arvo and Kirk, "Particle Transport and Image Synthesis", SIGGRAPH 1990 (Russian roulette).


## 3. GPU acceleration (brainstorm; superseded by 12)

### Known constraints in the current architecture

- Polymorphism is via `Box<dyn Geometry>` and `Box<dyn MaterialModel>`; GPUs cannot run trait
  objects or virtual dispatch.
- `trace` is recursive; shaders need an iterative loop that carries path throughput.
- Acceleration structures are pointer-based (`Box` children, `Arc` items), and must be flattened
  into arrays.
- Several objects are procedural and run complex code per ray: the sky's scattering integral,
  the ocean's repeating tiles, CSG, fireworks.
- The development machine is a Mac, so CUDA is not available locally.

### Options

- wgpu compute shaders (WGSL): Rust-native and portable (Metal, Vulkan, DX12). Write a separate
  GPU path tracer kernel over a flattened scene (triangles, spheres, BVH nodes, a material
  table). This is the most realistic route.
- rust-gpu: write the kernels in Rust compiled to SPIR-V, so some maths can be shared with the
  CPU code. Still no trait objects or allocation; the project's maturity needs checking.
- Hardware ray tracing (Metal ray tracing on Apple silicon, Vulkan ray query): fastest
  intersection, but platform-specific. wgpu's ray query support is experimental and, as far as I
  know, not yet available on Metal; this needs checking.
- Hybrid wavefront: batch rays on the CPU, intersect on the GPU, shade on the CPU. Keeps
  existing materials, but transfer overhead often cancels the gain.

### Prerequisites that would also help the CPU

- A data-oriented scene representation: flat geometry arrays, a BVH, materials as an enum or
  parameter table rather than trait objects.
- An iterative path loop with explicit throughput (it would also make Russian roulette use the
  whole path's throughput rather than one bounce's weight).
- Sky as a precomputed environment map; ocean tessellated to a finite triangle patch or tiled
  via instancing.

### Direction (preview first, then final quality)

- Start with a wgpu compute preview that supports only what flattens easily: triangles,
  spheres, planes, Lambertian, metal, dielectric, emissive, a sky lookup table. Unsupported
  objects are skipped or drawn flat, which a preview tolerates.
- Grow towards final quality by moving the CPU renderer onto the same flattened scene
  representation, so both paths share scene data and can be diffed against each other with the
  harness from workstream 1.

### Open questions

- Is keeping both a CPU and a GPU path acceptable long term, or should the GPU path eventually
  replace the CPU one?
- What should the preview window be (winit window, or progressive PNG written to disk)?

### Reading (candidates, unverified)

- Purcell, Buck, Mark, Hanrahan, "Ray Tracing on Programmable Graphics Hardware", SIGGRAPH 2002.
- Aila and Laine, "Understanding the Efficiency of Ray Traversal on GPUs", HPG 2009.
- Laine, Karras, Aila, "Megakernels Considered Harmful: Wavefront Path Tracing on GPUs", HPG
  2013.


## 4. Missing primitives

### Known: what exists

Sphere; plane (horizontal only, `Plane { y }`); triangle; mesh and smooth mesh (OBJ); box
(axis-aligned only); CSG union and difference; rotation transform; repeating mesh (ocean);
procedural box terrain, ocean, fireworks; skysphere; homogeneous fog; point lights;
emissive surfaces via `diffuse-light`.

### Gaps, roughly by usefulness

- ~~Transforms: translate and scale, and instancing.~~ Done: `transform` and named
  `geometries`.
- Arbitrary-orientation plane, disc, quad/rectangle.
- Area lights: quad, disc and sphere emitters with light sampling. Needed for the Cornell box and
  soft shadows. Directional (sun) light and spot light.
- Cylinder, cone, torus, capsule.
- CSG intersection.
- Environment lighting from an HDR image.
- Texture coordinates: `RawIntersection` has no UV, so image textures and normal maps are
  impossible today. Adding UV is a prerequisite for many of the above.
- Heightfields, implicit/SDF surfaces (ray marched), Bezier patches (the original Utah teapot
  format), curves/hair, subdivision surfaces, displacement.

### Reading (candidates, unverified)

- Roth, "Ray Casting for Modeling Solids", Computer Graphics and Image Processing 1982 (CSG).
- Hart, "Sphere Tracing: A Geometric Method for the Antialiased Ray Tracing of Implicit
  Surfaces", The Visual Computer 1996.
- Shirley and Wang, "Distribution Ray Tracing: Theory and Practice", 1992, and PBR's chapters on
  shapes and light sampling (area lights).


## 5. Animation

Goal: render sequences of frames at times t, eg. ocean waves evolving.

### Known

- The ocean takes a `time` parameter, applied to the wave phase (#45), but builds its mesh once
  at load, so each frame needs the mesh rebuilt (currently several seconds at `fourier_size`
  1024; the initial spectrum `h0` could be kept and only the time step recomputed).
- Fireworks take a `time` parameter.
- `ray.rs` defines an unused `RayX` with a `time` field.
- The scene is parsed once into immutable objects; nothing is a function of time.

### Plan

- Scene-level time: a frame range and rate (`"animation": { "frames": N, "fps": F }` or CLI
  `--frames`, `--time`), with `t` passed to time-dependent objects at build time.
- Output naming `<scene>.0001.png` etc.; optionally assemble with ffmpeg.
- Per-frame rebuild of time-dependent objects only, keeping static geometry (and its
  acceleration structures) between frames.
- Later: keyframed object and camera properties; motion blur via a shutter interval and a
  per-ray time (`RayX`).

### Questions

- How should time-varying properties be expressed: only built-in (ocean, fireworks) to begin
  with, or general keyframes on any numeric property?

### Reading (candidates, unverified)

- Cook, Porter, Carpenter, "Distributed Ray Tracing", SIGGRAPH 1984 (motion blur via ray time).
- Tessendorf, "Simulating Ocean Water", SIGGRAPH course notes 2001 (time evolution of the
  spectrum).


## 6. Volumetrics: ground fog and bounded fog volumes

### Known

- The earlier `volumetrics` branch is fully merged. The only unmerged volumetric work is
  `feature-sky-clouds` (see 7).
- `HomogenousFog` is an infinite medium, now sampled with Beer-Lambert (#43). It is implemented
  as a geometry that returns a random hit distance, rather than as a medium the ray travels
  through.
- `LowAltitudeFog` exists as a stub (`scatter` is a TODO).
- `scene.air_medium` is parsed but never used.
- There is no notion of the medium a ray is currently in, so refraction into or out of a
  volume, or a ray starting inside a fog volume, is not tracked.

### Plan

- Bounded volumes: a medium attached to a closed geometry (box, sphere, mesh); on entering,
  sample a free-flight distance against the exit distance.
- Height fog: density varying with altitude (exponential falloff), sampled by delta tracking
  (Woodcock) or by analytic integration for the exponential profile.
- Phase function: Henyey-Greenstein (the clouds branch has one).
- Light sampling inside media (shadow rays with transmittance), otherwise fog lit only by
  bounced paths converges slowly.

### Reading (candidates, unverified)

- Henyey and Greenstein, "Diffuse Radiation in the Galaxy", Astrophysical Journal 1941.
- Woodcock et al., delta tracking, 1965 (reactor physics; usually cited via Novak et al.).
- Novak et al., "Monte Carlo Methods for Volumetric Light Transport Simulation", Eurographics
  STAR 2018.


## 7. Clouds

### Known

- `feature-sky-clouds` (3 commits, also on `origin`, 17 behind `master`) adds a `CloudLayer`
  participating medium (ray-marched, noise-driven density, Henyey-Greenstein phase), sky
  integration, two demo scenes and `docs/cloud_implementation.md`. Its noise module has since
  reached `master` via the noise PR, so rebasing will conflict there.
- `combined_noise::density_field` on `master` is a cloud density function (fBm minus Worley,
  with distance falloff).

### Plan

- Rebase `feature-sky-clouds` onto the current stack, keeping master's noise module, then review
  its marching and lighting against the volumetrics work in 6, so clouds become one kind of
  bounded or layered medium rather than a separate mechanism.
- Cloud lighting: sun transmittance through the cloud (shadow marching), multiple scattering
  approximation, and the sky as ambient light.

### Reading (candidates, unverified)

- Nishita et al., "Display of the Earth Taking into Account Atmospheric Scattering", SIGGRAPH
  1993 (the sky model we use).
- Schneider, "The Real-time Volumetric Cloudscapes of Horizon Zero Dawn", SIGGRAPH Advances in
  Real-Time Rendering course 2015.
- Worley, "A Cellular Texture Basis Function", SIGGRAPH 1996; Perlin, "An Image Synthesizer",
  SIGGRAPH 1985.


## 8. Literate documentation of existing techniques

Goal: bring code written before this convention up to the literate standard, after you have
read the relevant papers. Each module gets a docstring pass, not code changes.

### Known: techniques already implemented without sources

Schlick's Fresnel approximation and refraction (`material/functions.rs`); the Moller-Trumbore
ray-triangle test (`shapes/triangle.rs`); Tessendorf ocean waves (`ocean.rs`; its open question
about the units of A was resolved by #53); the Nishita sky (`skysphere.rs`); Perlin and Worley
noise (`noise.rs`); Russian roulette (#58); cosine-weighted hemisphere sampling (#40);
Beer-Lambert free-flight sampling for fog (#43); sRGB encoding and Reinhard tone mapping (#59);
point-light falloff and next-event estimation (#60, #62); octree acceleration (`octree.rs`).

### Plan

- Verified reading list per module (as in Process), you read, then a docstring pass module by
  module.
- Done: `ocean.rs` (#63), against Tessendorf's 2004 notes, which you had already read.


## 9. Procedural terrain: noise, rivers and glaciers

Goal: terrain that scales to a whole planet, starting from seeded noise and shaped by real
geomorphic processes (river valley erosion, glaciation), as a function of seed and time so the
processes can be animated.

### Done (`src/terrain/`, literate docstrings in each module)

- Grid (`domain.rs`): a graph of cells covering a whole planet (equiangular cubed sphere) or a
  square patch of one. Processes are written once against the graph.
- Planet (`planet.rs`): continents, mountain belts and an uplift field from seeded noise, as
  functions of direction, so a patch is a window onto the same planet at higher resolution.
- Flow routing (`flow.rs`): Priority-Flood+epsilon, steepest descent, drainage area.
- Rivers and hillslopes (`fluvial.rs`): implicit stream power, linear diffusion. Tested against
  the steady-state slope-area relation.
- Glaciers (`glacier.rs`): ELA by latitude with ~100 kyr cycles, capped linear mass balance,
  semi-implicit shallow ice flow (incomplete-Cholesky conjugate gradients, Picard iteration,
  adaptive steps bounded by stiffness, speed capped at 1 km/yr) with a conservative flux
  limiter; tested against Halfar's dome and a plateau ice cap reaching steady state. Erosion
  proportional to sliding speed taken from the ice fluxes.
- Geology (`planet.rs`, `simulation.rs`): the ground is bedrock (folded strata of alternating
  hardness, regional variation) under volcanic rock under sediment, each with its own
  erodibility for rivers and glaciers.
- Sediment (`fluvial.rs`): erosion-deposition after Yuan et al. 2019 (G = 1), lakes trap
  sediment to their spill level, glacial erosion feeds meltwater sediment into the rivers, so
  alluvial valley floors, filled lake basins and outwash form.
- Volcanoes (`volcano.rs`): vents from a seeded planet-wide lattice (random spacing, as de
  Bremond d'Arcs et al. 1995 found), cones, and DOWNFLOW-style lava flows (Favalli et al.
  2005); eruption rates after White et al. 2006.
- Deltas (`marine.rs`): river sediment reaching the sea is laid at the mouth and spread by
  depth-dependent diffusion (Kaufman et al. 1991; Rivenæs 1992) that moves only sediment; sea
  cells built above sea level become land, so deltas prograde.
- Meanders (`channel.rs`): a sub-grid centreline model on low-gradient reaches of large rivers
  (Ikeda et al. 1981; Howard and Knutson 1984), confined to the valley floor, with neck
  cutoffs leaving ox-bow lakes that fill over centuries. Rivers on patches are drawn as lines
  at their hydraulic-geometry widths (Leopold and Maddock 1953) rather than as whole cells.
- Time (`simulation.rs`): operator splitting, deterministic and resumable; morphological
  acceleration (MORFAC) for glacial erosion.
- Scene object `"type": "landscape"` (`object.rs`), maps (`map.rs`), demo
  `demo/scenes/terrain-alpine.json`.

### Measured

- A 96 x 96 patch of 500 m cells (48 km) through one 100 kyr glacial cycle: about 6 minutes on
  one core, nearly all of it ice flow around the glacial maximum. Ice covers almost the whole
  range at the maximum (about 1.4 km at its thickest, in basins), and at 100 kyr the valleys are
  broad troughs with cirque glaciers left on the peaks.
- The whole planet at 64 x 64 x 6 cells (about 170 km each): 20 kyr in under 4 seconds.

### Known limitations and open questions

- No moraines (glacial debris goes straight to the rivers). Lakes fill flat (no lake deltas:
  filling from the inlet built kilometre-high sediment cliffs on 500 m cells). Marine
  diffusivity constants are typical values, not fitted.
- Meanders don't change the grid (no lateral erosion of valley walls or reworking of the
  floodplain), are only on patches, and are reset when a reach's downstream end moves (reaches
  are re-found every 1000 years). Howard and Knutson's nonlinear R0(r/w) is simplified to the
  linear form; no chute cutoffs, avulsion or braiding. The sediment-to-bedrock erodibility ratio (3) and lava's (0.3) are
  model choices; no source found for typical values.
- Volcanoes: no craters, calderas, explosive (tephra) deposits or lahars; cones are linear in
  profile; vents don't respond to tectonics or to loading.
- No isostasy (ice loading, erosional unloading), no sea-level change with the ice ages.
- Ice: temperate everywhere; SIA is poor on steep, narrow valleys (Cordonnier et al. 2023 use
  higher-order flow); the flux limiter is cruder than Jarosch et al.'s MUSCL scheme.
- Mass balance constants (β, ṁ_max, ELA range and glacial depression), Glen's A, and our
  sliding coefficient need checking against sources (Cuffey and Paterson; IGM's defaults).
  Sliding ignores water pressure; with A_s ten times larger, basins were dug kilometres deep.
- The ice solver's regularisations (speed cap, stiffness-bounded steps, MORFAC = 10) are
  pragmatic; a multigrid solver or a higher-order flow emulator (as Cordonnier et al. 2023)
  would be the principled routes to speed and accuracy.
- Planet scale: the whole planet runs at coarse resolution (eg. 64 x 64 x 6 cells is about 170
  km each) and a patch at fine resolution, independently. Multi-resolution coupling (a patch
  taking its boundary from the evolved planet) is the next step to "scales to a planet".
- Uplift is constant in time; plate motion would move belts.
- Rendering uses the octree over a triangle mesh; a heightfield intersector would be faster.
- Animation: a `Terrain` advances incrementally, but the renderer rebuilds the scene per run.
  With workstream 5, frames could advance one `Terrain`. With MORFAC > 1, ice responds too
  slowly relative to the climate to be animated faithfully.

### Reading (verified bibliographic details; claims checked from abstracts unless noted)

- Braun and Willett, "A very efficient O(n), implicit and parallel method to solve the stream
  power equation governing fluvial incision and landscape evolution", Geomorphology 180-181,
  2013, doi:10.1016/j.geomorph.2012.10.008.
- Cordonnier, Braun, Cani, Benes, Galin, Peytavie, Guérin, "Large Scale Terrain Generation from
  Tectonic Uplift and Fluvial Erosion", Computer Graphics Forum 35(2), 2016,
  doi:10.1111/cgf.12820.
- Cordonnier et al., "Forming Terrains by Glacial Erosion", ACM TOG 42(4), 2023,
  doi:10.1145/3592422 (ice flow by a CNN emulator of higher-order flow, not SIA).
- Barnes, Lehman, Mulla, "Priority-Flood", Computers & Geosciences 62, 2014,
  doi:10.1016/j.cageo.2013.04.024.
- Whipple and Tucker, JGR 104(B8), 1999, doi:10.1029/1999JB900120; Howard, WRR 30(7), 1994,
  doi:10.1029/94WR00757.
- Culling, J. Geology 68(3), 1960, doi:10.1086/626663; Fernandes and Dietrich, WRR 33(6), 1997,
  doi:10.1029/97WR00534; Roering, Kirchner, Dietrich, WRR 35(3), 1999,
  doi:10.1029/1998WR900090.
- Hutter, "Theoretical Glaciology", 1983; Glen, Proc. R. Soc. A 228, 1955,
  doi:10.1098/rspa.1955.0066; Mahaffy, JGR 81(6), 1976, doi:10.1029/JC081i006p01059; Jarosch,
  Schoof, Anslow, The Cryosphere 7, 2013, doi:10.5194/tc-7-229-2013; Weertman, J. Glaciology
  3(21), 1957, doi:10.3189/S0022143000024709; Cuffey and Paterson, "The Physics of Glaciers",
  4th ed., 2010.
- Hallet, J. Glaciology 23(89), 1979, doi:10.3189/S0022143000029725; Humphrey and Raymond,
  J. Glaciology 40(136), 1994, doi:10.3189/S0022143000012429; Herman et al., Science 350, 2015,
  doi:10.1126/science.aab2386.
- Braun, Zwartz, Tomkin, Annals of Glaciology 28, 1999, doi:10.3189/172756499781821797; Herman
  and Braun, JGR Earth Surface 113, 2008, doi:10.1029/2007JF000807.
- Hays, Imbrie, Shackleton, Science 194, 1976, doi:10.1126/science.194.4270.1121.
- Roelvink, Coastal Engineering 53, 2006, doi:10.1016/j.coastaleng.2005.10.015 (MORFAC).
- Perlin, "Improving Noise", SIGGRAPH 2002, doi:10.1145/566654.566636; Ebert et al.,
  "Texturing and Modeling", 3rd ed., 2003 (Musgrave's chapters).
- Sadourny, Monthly Weather Review 100(2), 1972; Ronchi, Iacono, Paolucci, J. Comput. Phys.
  124, 1996, doi:10.1006/jcph.1996.0047.
- Reviews: Tucker and Hancock, ESPL 35(1), 2010, doi:10.1002/esp.1952; Galin et al., "A Review
  of Digital Terrain Modeling", Computer Graphics Forum 38(2), 2019, doi:10.1111/cgf.13657.
- Sediment: Yuan, Braun, Guerit, Rouby, Cordonnier, JGR Earth Surface 124(6), 2019,
  doi:10.1029/2018JF004867; Davy and Lague, JGR Earth Surface 114, 2009,
  doi:10.1029/2008JF001146; Guerit et al., Geology 47(9), 2019, doi:10.1130/G46356.1 (G values);
  Hallet, Hunter, Bogen, Global and Planetary Change 12, 1996 (glacial sediment yields).
- Lithology: Forte, Yanites, Whipple, ESPL 41(12), 2016, doi:10.1002/esp.3947; Perne et al.,
  Earth Surface Dynamics 5, 2017, doi:10.5194/esurf-5-85-2017.
- Volcanoes: Favalli, Pareschi, Neri, Isola, GRL 32(3), 2005, doi:10.1029/2004GL021718; White,
  Crisp, Spera, G-cubed 7, 2006, doi:10.1029/2005GC001002; de Bremond d'Arcs, Jaupart, Sparks,
  JGR 100(B10), 1995, doi:10.1029/95JB02153.
- Meanders: Ikeda, Parker, Sawai, J. Fluid Mech. 112, 1981, doi:10.1017/S0022112081000451;
  Howard and Knutson, WRR 20(11), 1984, doi:10.1029/WR020i011p01659; Camporeale et al., Rev.
  Geophys. 45, 2007, doi:10.1029/2005RG000185; Leopold and Maddock, USGS PP 252, 1953,
  doi:10.3133/pp252; Constantine and Dunne, Geology 36(1), 2008, doi:10.1130/G24130A.1;
  Constantine et al., Sedimentology 57(2), 2010; Constantine et al., Nature Geoscience 7, 2014,
  doi:10.1038/ngeo2282; Langston and Tucker, ESurf 6, 2018, doi:10.5194/esurf-6-1-2018.
- Marine: Kaufman, Grotzinger, McCormick, Kansas Geol. Survey Bull. 233, 1991; Rivenæs, Basin
  Research 4, 1992, doi:10.1111/j.1365-2117.1992.tb00136.x; Yuan et al., EPSL 524, 2019,
  doi:10.1016/j.epsl.2019.115728.
- Unchecked: Halfar's solution as given by Bueler et al., J. Glaciology 51(173), 2005 (used by
  `test_halfar_dome`, which passing within 3% supports).



## 10. Variance reduction

Goal: less noise for the same render time. Renders are visibly grainy: even grain on sky-lit
matte surfaces (`spheres`), fireflies on glossy surfaces under the sun (`glossy-sun`), and
speckle in clouds (`orbit`).

### Known (from the code, at the start)

- Diffuse materials (Lambertian, leaf, grass, rock, terrain, ocean's water body) sample point
  lights, the sun and one area light directly (`direct_lighting`), and their scattered ray then
  ignores those lights (`ScatteredRay::sampled_lights`): light sampling alone, no MIS.
- The sun is treated as a direction (irradiance times the cosine) by diffuse surfaces, and as a
  disc of 6.8e-5 sr (`Sun::disc_radiance`) by everything else. Glossy materials (`metal`,
  plastic's coat) find the sun and area lights only by chance: fireflies.
- Rough reflection is the mirror direction plus a random point in a ball scaled by roughness.
  It has no closed-form density, so it can't take part in MIS.
- The sky is reached only by scattered rays: cosine-weighted for diffuse surfaces, regardless
  of where the sky is bright. Since the sky-view LUT, sky radiance is cheap to evaluate.
- Only the pixel position is stratified (an n x n jittered grid per pass, n usually 2). Every
  other dimension draws independent PCG numbers.
- Area lights are chosen uniformly, not by power.
- Volumes already use delta tracking, ratio-tracked shadows, and the similarity and Oz
  approximations.

### Measured baseline (`scripts/variance.sh`, 4 renders, Apple M2)

Efficiency is 1 / (relMSE x seconds); compare ratios on the same scene, not across scenes.

| Scene | Size, samples | Seconds | relMSE (mean) | relMSE (median) | Efficiency |
| --- | --- | --- | --- | --- | --- |
| cornell-box | 256 x 256, 64 | 1.34 | 0.00294 | 0.00086 | 254 |
| spheres | 320 x 240, 64 | 0.33 | 0.00418 | 0.00076 | 716 |
| glossy-sun | 480 x 240, 64 | 0.67 | 40.6 | 0.00057 | 0.037 |
| forest | 400 x 200, 8 | 5.92 | 0.02684 | 0.00792 | 6.3 |
| orbit | 160 x 80, 4 | 46.9 | 1.0e6 | 0 | 0 |

Each pixel's variance is divided by its median across the renders squared (`variance.rs`
explains why not the mean). Fireflies then dominate the mean, as they dominate what a viewer
sees, and make it noisy in turn: a single firefly in a black pixel (orbit's space) adds 10⁶.
Compare means only between runs with large differences, and read the median alongside: it is
the noise of a typical pixel. Orbit's median is zero because most of the image is black space.

### Assumptions to test

- That MIS with a microfacet BRDF removes most fireflies on glossy surfaces under the sun.
- That sampling the sky in proportion to its brightness helps sky-lit diffuse surfaces enough
  to pay for the extra shadow ray. Under an overcast or even sky it may not.
- That a low-discrepancy sampler gains at 16-64 samples per pixel, mostly in direct lighting.
- The source of the speckle in `orbit` is unknown. Candidates: deep multiple scattering in
  high-albedo cloud, the variance of transmittance estimates, sky light reaching deep vertices.
  Measure before choosing a remedy.

### Steps (each a stacked PR, each measured with `scripts/variance.sh`)

1. Measurement: `--variance <runs>`, `--samples`, `scripts/variance.sh`, the `glossy-sun` scene.
2. Materials expose their BSDF (value times cosine, and the density of sampling a direction);
   MIS (power heuristic) between light sampling and BSDF sampling, for area lights and the sun
   (sampled over its disc where a BSDF can resolve it). Lambertian and terrain first.
3. GGX microfacet reflection for `metal` and plastic's coat, sampled by visible normals, which
   brings them into MIS.
4. Sky importance sampling: a tabulated distribution over directions, MIS with the BSDF.
5. Low-discrepancy sampling: Owen-scrambled Sobol' across every dimension.
6. Choosing lights by power.
7. Investigate the speckle in `orbit`.
8. (After review) Perceptual roughness, α = r², and path regularisation for light reaching
   glossy surfaces after another scattering.

Status: steps 1-8 done (see Results). Candidates next: multiple-scattering compensation for
GGX; shrinking the regularisation with the sample count, for consistency; the orbit
traversal costs.

### Results (`scripts/variance.sh`, 4 renders, after each step)

| Step | cornell-box | spheres | glossy-sun | forest |
| --- | --- | --- | --- | --- |
| Baseline | 0.00086 / 254 | 0.00076 / 716 | 0.00057 / 0.037 | 0.00792 / 6.3 |
| 2. MIS | 0.00086 / 262 | 0.00076 / 686 | 0.00057 / 0.042 | 0.00787 / 6.2 |
| 3. GGX | 0.00086 / 264 | 0.00105 / 658 | 0.00081 / 57 | 0.00787 / 5.5 |
| 4. Sky sampling | 0.00086 / 270 | 0.00058 / 802 | 0.00033 / 41 | 0.00647 / 5.4 |
| 5. Sobol' | 0.00044 / 468 | 0.00019 / 1521 | 0.00008 / 41 | 0.00616 / 6.3 |
| 6. Lights by power | 0.00044 / 459 | 0.00015 / 1512 | 0.00007 / 215 | 0.00616 / 5.7 |
| 8. α = r², regularisation | 0.00044 / 480 | 0.00015 / 1431 | 0.00007 / 548 | 0.00616 / 5.8 |

Each entry is median relMSE / efficiency (from the mean relMSE). Step 6 (lights chosen by
power) leaves these scenes alone, having one light or none; on `lamps`, five lamps of unequal
power, it raises efficiency from 13 to 72 and lowers the median relMSE tenfold. Glossy-sun's
mean relMSE, dominated by fireflies: 40.6 at the baseline, 0.022 with GGX, 0.0014 at the end.
Timings later in the table ran with the machine under other load (forest 5.9 s at the
baseline, 7.3 s at the end, for unchanged work per sample in the later steps): compare
medians there.

Findings:

- MIS on its own changed nothing measurable: diffuse surfaces already took nearly all their
  direct light from light sampling. Its value came with glossy BSDFs.
- GGX with MIS removed the glossy-sun fireflies (mean relMSE 33 to 0.02), but in review the
  GGX step looked more speckled. Two causes, separated by experiment:
  - Roughness as α made reflections blurrier than the fuzzed mirror's, and 40% noisier per
    pixel at the same samples (spheres median 0.00069 to 0.00099). With α = roughness² (step
    8), both the look and the noise return to the old ones (0.00072).
  - Light reaching a polished surface after another scattering (caustics on the ground; a
    neighbour's highlight seen in the brushed sphere) became reachable, and sparkled; the
    fuzzed mirror almost never found it. Path regularisation (step 8) floors every glossy
    vertex after a path's first at α 0.2: glossy-sun's mean relMSE 5.7 to 0.0014, for softer
    caustics and frosted reflections of reflections.
- Measuring variance across renders by dividing by the pixel's mean understated fireflies,
  capping each near k: BSDF sampling alone measured half as noisy as MIS, against a reference
  error 90 times larger. The median denominator fixes this (`variance.rs`).
- Sky sampling costs a shadow ray per vertex (7-50% more time) and helps most under skies of
  uneven brightness; `"sample_sky": false` turns it off.
- Sobol' sampling first doubled render times: shuffled indices are full 32-bit numbers, and
  computing a coordinate bit by bit dominated the profile. Byte tables brought the overhead
  to about 10%.

### The speckle in `orbit` (step 7)

- It is not deep multiple scattering: with the planet filling the view at 4 samples, the
  median relMSE is 0.033 with one bounce, 0.042 with two and 0.047 with four. It is the noise
  of the first scattering in the clouds (where the free path ends, the sun's transmittance
  from there), at a sample count too low for it.
- The sample count is low because samples are dear: about 2 ms each. The profile is dominated
  by `GlobalNubis::majorant_from` and the cube-sphere mapping (`tan`, `atan`,
  `CubeSphere::locate`: about 60% of samples), not by the clouds' density (4%): delta tracking
  steps across the weather map one texel at a time, and each step converts to and from
  texel coordinates by trigonometry.
- The bright dot in black space, which dominates orbit's mean relMSE, is the sun's disc,
  smaller than a pixel: real aliasing of a tiny, intense source, not a rendering error.
- Recommended, as performance work: tabulate the tangents of texel edges (`CubeSphere::boundary`
  takes the tangent of an integer edge index each time), and skip runs of texels with a
  coarser map of their maxima, as a mipmap. Faster samples are the cure for this speckle.

### Reading (verified; details in each PR)

- Veach and Guibas, "Optimally Combining Sampling Techniques for Monte Carlo Rendering",
  SIGGRAPH 1995, doi:10.1145/218380.218498; Veach, PhD thesis, Stanford 1997, ch. 9.
- Walter, Marschner, Li, Torrance, "Microfacet Models for Refraction through Rough Surfaces",
  EGSR 2007; Heitz, "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs",
  JCGT 3(2), 2014; Heitz, "Sampling the GGX Distribution of Visible Normals", JCGT 7(4), 2018.
- Pharr, Jakob, Humphreys, "Physically Based Rendering", 4th ed.: §2.1.4 (efficiency), §2.2.3
  (MIS), §A.5.4 (cone sampling), §A.5.5 (2D distributions), §12.5.2 (image infinite lights),
  §12.6.2 (power light sampler), §8.7 (Sobol' sampling), §13.4 (path tracer).
- Joe and Kuo, "Constructing Sobol sequences with better two-dimensional projections", SIAM J.
  Sci. Comput. 30(5), 2008; Burley, "Practical Hash-based Owen Scrambling", JCGT 9(4), 2020.


## 11. Coverage of PBRT (4th edition), chapter by chapter

Goal: close the gaps between this renderer and what Pharr, Jakob and Humphreys, "Physically
Based Rendering", 4th ed. (<https://pbr-book.org/4ed/>) covers, one chapter at a time, in a
single PR, pausing for review after each chapter. The survey of gaps (from the code, at the
start) by chapter:

- 4, Radiometry, spectra and colour: spectral rendering, RGB-to-spectrum upsampling, blackbody
  emitters, colour spaces and a sensor model.
- 5, Cameras and film: orthographic, spherical and realistic (lens system) cameras; motion blur
  (`Ray::time` exists but is unused); Gaussian, Mitchell, Lanczos filters; film sensor, white
  balance.
- 6, Shapes (done): curves, bilinear patches, Loop subdivision; texture coordinates on
  intersections; floating-point error bounds for spawning rays.
- 8, Sampling (done): Halton, stratified, blue-noise Z-Sobol'; PMJ02 deferred; ray
  differentials moved to chapter 10.
- 9, Reflection models (done): conductors with complex η, k; rough dielectrics; thin
  dielectric; mix; anisotropic roughness; hair.
- 10, Textures (done): image textures, MIP maps and filtering (EWA); normal maps.
- 11, Volume scattering (done): grid media, chromatic media with spectral MIS, emissive media;
  media bounded by shapes.
- 12, Lights (done): spot, goniometric, projection, textured area lights; light BVH.
- 13-14, Light transport (done, but light tracing): layered BSDFs, subsurface scattering.
- Online chapters: BDPT, MLT, SPPM. 15-16 (done on the CPU): the wavefront integrator; the GPU
  is deferred (see workstream 3).

### Chapter 4: done

- `src/spectrum.rs`: CIE 1931 matching functions and D65 (`src/spectrum/cie.rs`, from
  colour-science's tables, every 5 nm); visible-wavelength importance sampling and three hero
  wavelengths per path; the film's XYZ estimate and linear sRGB; Planck's law and luminance-
  normalised blackbody emission; refractive indices by Cauchy or Sellmeier, with BK7, fused
  silica, SF11 and diamond; termination of secondary wavelengths at dispersive interfaces.
- `src/spectrum/rgb2spec.rs`: Jakob and Hanika's sigmoid-polynomial upsampling, the table
  solved by Gauss-Newton in CIELAB on first use (32³ per largest component).
- `"spectral": true` turns it on; RGB mode is unchanged. Materials, lights, the sun, the sky,
  the background and the media convert their RGB at the point of use (`Color::as_reflectance`,
  `as_illuminant`, `as_unbounded`).
- The sky is spectral too (`skysphere`'s "Spectral skies"): its coefficients at the sample's
  wavelengths (Rayleigh by a log-log fit through the preset's three, ozone from Serdyuchenko et
  al.'s cross-section), the sun's spectrum above the air from ASTM G-173 (both from Bruneton
  2017's data), and its multiple scattering and sky-view tables at 15 wavelengths, interpolated.
  The tables take about 6 s to build (4 cores), on the first spectral sample; a sky sample then
  costs about twice an RGB one. At the limb at sunset, three wavelengths overstated the red
  band: the eye barely sees 680 nm, which long paths spare, and 600 nm suffers ozone's peak.
- Known limitations: halos, rainbows and diffraction stay per RGB channel and are upsampled;
  only sRGB output; three wavelengths, not pbrt's four. White balance and the sensor model
  belong with the film, in chapter 5.

### Deferred from chapter 4

Tracked here rather than done, each for a stated reason:

- Halos, rainbows and diffraction at the sample's wavelengths. They trace several wavelengths
  per RGB band already, so hue and angles are right; native spectra would mostly add
  saturation that sRGB output clips. Revisit with wide-gamut output (chapter 5's film).
- Faster spectral sky: the spectral tables take about 6 s to build (4 cores) and a sky sample
  costs about twice an RGB one. Candidates: f32 tables, caching a sample's `Channels` (the
  Rayleigh fit takes logarithms on every call), building only the bins a render samples.
- Four wavelengths per path, as pbrt-v4: needs a spectral type wider than `Color`.
- Faster spectral materials: constant RGB parameters are upsampled per lookup; their sigmoid
  coefficients could be computed once, when the scene is parsed.
- Measured camera sensitivities (pbrt's `PixelSensor` with a camera's RGB curves): needs the
  published curves.
- A check against Tungsten's Cornell box reference: its download was refused (403) from the
  development environment; spectral was compared with our own RGB render instead.

### Chapter 5: cameras and film (done)

Plan, in commits:

1. Reconstruction filters: Gaussian, Mitchell-Netravali, Lanczos (windowed sinc), sampled by
   tabulated importance with signed weights (PBR 8.8).
2. Cameras: orthographic; spherical (equirectangular and equal-area); a camera trait that
   carries the sample's time.
3. Motion blur: a shutter interval; transforms (and the camera) that move between two poses,
   interpolated (translation linearly, rotation by slerp); bounds that cover the motion. Each
   sample's time is per-path state, like its wavelengths, rather than a field on every `Ray`
   (there are 262 constructions of one), which pbrt's spawned rays inherit anyway.
4. Film and sensor: exposure from ISO, shutter time and f-number; white balance (von Kries in
   Bradford's cone space, to a colour temperature or named illuminant); output colour spaces
   (sRGB, Display P3, Rec. 2020, ACES2065-1) for EXR.
5. Realistic camera: lens systems traced element by element (Kolb, Mitchell and Hanrahan 1995;
   PBR 5.3), with PBR's lenses built in (double Gauss 50 mm, wide 22 mm, telephoto 250 mm,
   fisheye 10 mm), focusing by moving the film, exit pupil bounds for sampling, and vignetting.
6. Astigmatism: a lens system's own (Seidel) aberrations appear in the realistic camera; in
   addition the thin-lens camera gets a parametric astigmatism, both the field-dependent
   aberration of a lens (tangential and sagittal foci apart in proportion to the field angle
   squared) and an eye's (a cylindrical error of given power and axis, across the whole field).
7. Lens flare: ghosts from pairs of reflections between a lens system's surfaces, traced
   forward from each bright light through the lens with anti-reflection coated (wavelength
   dependent) Fresnel reflectance (Hullin, Eisemann, Seidel and Lee, "Physically-Based Real-Time
   Lens Flare Rendering", SIGGRAPH 2011), plus the aperture's diffraction starburst, added to
   the existing glare.

(Correction to the survey: `Ray` has no `time` field; the unused one is on `RayX`.)

Done as planned, in seven commits, each with tests and a demo scene. Measured or checked:

- Filters: weighted samples of each tabulated filter estimate its moments to 2e-3.
- Realistic camera: each built-in lens's focal length within 12% of its name; a focused point
  images to under 30 µm; f/4 against wide open passes a quarter of the light, as (N_open / N)².
- Astigmatism: the cylinder's two focal lines where paraxial optics puts them; Seidel streaks
  four times as long at twice the field.
- Lens flare: a quarter-wave coating's reflectance at its design wavelength matches
  ((n0 n2 - n1²) / (n0 n2 + n1²))²; the sun's ghosts, unexaggerated, peak at about a fifth of
  the sky's brightness in `lens-flare` (subtle, as with modern coatings).

### Deferred from chapter 5

- Chromatic aberration in the realistic camera (spectral mode): lens prescriptions give only
  n_d. The flare estimates dispersion from the glass map's trend; the camera could too, or take
  Abbe numbers as a fifth column.
- The flare treats each bright region as a point light at its centre, and maps image positions
  to directions by a pinhole, not the realistic camera's distortion; extended lights (a large
  window) should be integrated over.
- Moving objects aren't sampled as area lights (a moving emitter is found only by BSDF rays),
  and deforming meshes need per-vertex motion.
- Measured sensor curves, and wide-gamut PNG output with an ICC tag.
- The starburst's tail only; the Airy core and the spikes' fine structure are left to the
  glare.

### Chapter 6: shapes (done)

In six commits, each with tests:

1. Texture coordinates, computed on demand (`Geometry::uv`): spheres, planes, discs,
   cylinders and cones, triangles, meshes (OBJ and Tungsten coordinates), through transforms;
   `uv` and `checker` materials to see them.
2. Sphere lights sampled by the cone they subtend (`Geometry::sample_from`/`pdf_from`): on
   `lamps` at 16 spp, RMSE against a 1024 spp reference from 0.185 to 0.129.
3. Bilinear patches (Reshetov's intersection; area sampling exact for saddles).
4. Curves: cubic Bézier hair, flat, cylinder and ribbon, in a BVH; `fur`.
5. Loop subdivision surfaces: on a regular mesh the angle between neighbouring faces halves
   per level, as it should.
6. Ray offsets scaled to the scene (`Intersection::spawn_offset`), replacing the fixed 0.001.

### Deferred from chapter 6

- Spherical triangle and rectangle sampling for triangle and quad lights (PBR 6.5.4, 6.6.3);
  they're sampled by area.
- pbrt's exact error bounds: ours are conservative (1e-7 relative), not a few ulps, since our
  shapes' solvers aren't analysed.
- Curves: v across the strand isn't recovered from the point alone (it's 0.5), and they trace
  at about a sixth of the rate of triangles; a hair BSDF belongs to chapter 9.
- Subdivision: normals from the refined mesh, not the limit surface's tangents; no creases.
- No texture coordinates yet for boxes, tori, CSG results, blobs and the other POV-Ray shapes.

### Chapter 8: sampling (done)

`src/sampler.rs`: Halton (Owen-scrambled digit by digit with Kensler's permutations),
stratified (jittered, permuted per dimension; the pixel on a grid for square counts) and
ZSobol' (Ahmed and Wonka's Morton-ordered Sobol', blue-noise error), beside the existing Sobol'
and independent samplers; `"sampler"` picks one. Filters were done with chapter 5. Ray
differentials go with texture filtering (PBR 10.1), in chapter 10.

Measured (relMSE of one 16 spp render, six seeds; and spectra of 1 spp error images):

| scene | independent | Sobol' | ZSobol' | Halton | stratified |
|---|---|---|---|---|---|
| cornell-box | 0.0130 | 0.0083 | 0.0088 | 0.0099 | 0.0093 |
| lamps | 0.0187 | 0.0148 | 0.0147 | 0.0160 | 0.0159 |
| glossy-sun | 0.0106 | 0.0075 | 0.0073 | 0.0080 | 0.0078 |

ZSobol' matches Sobol' in MSE, and moves the error to high frequencies: on a soft shadow at
1 spp, 1.6% of its energy lies below a quarter of the Nyquist frequency, against 5.2% for
Sobol' (white noise: 4.9%); on scenes dominated by deep indirect light (Cornell box), the
effect is small, since only the first dimensions are ordered.

### Deferred from chapter 8

- PMJ02: its defining property, a (0, 2)-sequence in the first two dimensions, is already the
  Sobol' sampler's; the blue-noise variant (pmj02bn) needs pbrt's precomputed tables.
- Halton costs about three times as much per sample as Sobol' here (Owen scrambling every digit
  in 64 prime bases); pbrt's digit-permutation tables would be cheaper.
- ZSobol' needs the sample count in advance: samples added beyond it (resuming with more) fall
  back to per-pixel Sobol'. Indices are 32-bit, so images of 2^32 samples or more wrap.
- Aligning dimensions to sampling decisions (pbrt's Get1D/Get2D/GetPixel2D), which would keep
  later bounces stratified across branching paths.

### Chapter 9: reflection models (done)

1. `conductor`: complex-index Fresnel; pbrt's measured silver, aluminium, gold, copper and brass
   (`src/spectrum/metals.rs`), spectral in spectral mode; η/k or a reflectance colour otherwise.
   Normal-incidence reflectances as published (gold 0.93, 0.77, 0.39).
2. Anisotropic GGX (roughness along and across a tangent).
3. `rough_dielectric` (Walter et al.) and `thin_dielectric`.
4. `mix` (from pbrt's chapter 10, but it's a material).
5. `hair`: pbrt's hair BSDF, from melanin concentrations or a colour; passes pbrt's white furnace
   and sampling tests.

Demo: `pbrt-materials.json`; `fur.json` with the hair BSDF.

### Deferred from chapter 9

- Multiple scattering between microfacets: rough glass keeps 98% of the light seen from outside
  at α = 0.3, but 82% from inside (single scattering); rough metals likewise darken.
- Measured BSDFs (pbrt's `MeasuredBxDF`, RGL data) and layered BSDFs (coated diffuse and
  conductor; pbrt's chapter 14); our `plastic` is a simpler coat.
- Anisotropy's tangent is a fixed world direction projected onto the surface, not the surface's
  own ∂p/∂u, so brushed metal can't follow a mesh's parameterisation yet.
- Hair: shadow rays leave from the near side, so direct light from behind isn't transmitted (TT)
  in light sampling, only by sampled rays; the colour inversion assumes many bounces, so fur
  needs a deep `max_depth`; hair renders slowly (curves themselves are slow).

### Chapter 10: textures (done)

`src/textures.rs`: image textures (PNG, JPEG, EXR; 8-bit decoded from sRGB) as MIP pyramids,
filtered by pbrt's camera approximation of ray differentials (`Camera::Approximate_dp_dxy`)
mapped through each shape's texture coordinates: point, bilinear, trilinear, or EWA (the
default; Heckbert's elliptically weighted average, as pbrt's). Any colour of `lambertian` and
`plastic`, and a `mix`'s amount, can be an image; tangent-space normal maps (`"normal": {"map":
..}`) through a new `Medium::normal_at_hit`. Demo `textures.json`, with generated textures
(`scripts/make-demo-textures.py`).

On a checkerboard receding to the horizon at 4 spp, against 1024 spp point sampling: point RMSE
0.096 (FLIP 0.054), trilinear 0.097 (0.082; aliasing gone but over-blurred, as isotropic
filtering is at grazing angles), EWA 0.050 (0.044).

### Deferred from chapter 10

- True ray differentials, propagated through specular bounces (pbrt-v4 too uses the camera
  approximation everywhere); after a mirror or a lens, the footprint is the camera's, not the
  reflection's.
- Textures for every parameter (roughness, conductors' reflectance, emission), and procedural
  textures in texture space (ours are in world space: `noise`, `checker` by position).
- Displacement and alpha (cut-out) textures; texture coordinates for more shapes (boxes, tori,
  CSG); pbrt's resampling of non-power-of-two images (ours box-filter odd sizes).

### Chapter 11: volume scattering (done)

Volumes already had delta tracking with local majorants, residual ratio tracking for shadows,
Henyey-Greenstein and Mie phase functions, and much beyond pbrt (diffraction, halos, rainbows,
the similarity and Oz approximations). Added, in four commits with tests:

1. Grid media (`src/volume/grid.rs`, pbrt's `GridMedium`/`SampledGrid`): densities on a voxel
   grid, trilinear, inline or from Mitsuba `.vol` files (float32 or uint8), with pbrt's 16³
   majorant grid walked by a DDA. On the demo plume, density lookups per ray fall from 16.6 to
   1.6 against a single global majorant. Demo grids from `scripts/make-demo-volumes.py`.
2. Media bounded by any closed shape (`Volume::shape`, from the shape's crossings): pbrt's
   medium interface for an interior, so a dielectric and a volume of the same shape make
   coloured glass or a glass of liquid.
3. Emissive media (`src/volume/emission.rs`): σ_a L_e, estimated as (1 - α) L_e at each
   collision; a colour or blackbody radiation at a temperature, constant or from a grid,
   following Planck's law in brightness (unlike pbrt, which normalises each temperature's
   spectrum): 1800 K is about 840 times as luminous as 1200 K.
4. Chromatic media with spectral MIS (`src/volume/chromatic.rs`; PBR 14.1.2-3, Miller et al.
   2019): `"sigma_a"`, `"sigma_s"` per channel; each path tracks with a random hero channel
   against the largest channel's majorant and weights what it gathers by r_c / mean(r).
   Because media here are objects found by intersection, not the space between surfaces, a
   walk records its tentative collisions and the trace applies those before the nearest hit
   (`chromatic::settle`). Shadow rays ratio-track every channel at once, so scene
   transmittance is now a `Color`. Tests: Beer-Lambert per channel through a chromatic
   absorber, and a chromatic furnace that averages to one in every channel with no estimate
   above three.

Demo `media.json`: a smoke plume and its fire, a glass ball of red liquid, and a ball that
scatters blue most, casting an orange shadow.

### Deferred from chapter 11

- Emissive media aren't sampled as lights (nor are they in pbrt-v4): surfaces near a fire see
  it only when a BSDF ray hits it, which is noisy. Light sampling of the emission (or a light
  BVH over voxels, chapter 12) would fix it.
- NanoVDB/OpenVDB grids (pbrt's `NanoVDBMedium`), and pbrt's RGB grid medium (σ_a, σ_s and
  L_e per voxel as colours); ours are a scalar grid times per-channel coefficients.
- Shadow rays through dielectrics stay blocked, so a scattering medium inside glass is lit only
  by paths, not by the sun or point lights (pbrt the same).
- Chromatic media under a scaling `transform` (walk distances are local, the hit's world); the
  multiple-scattering approximations use the mean albedo in chromatic media.
- pbrt's `CloudMedium` and `"uniform"` medium are covered by `nubis_clouds` and constant
  volumes.

### Chapter 12: light sources (done)

Point lights, the sun (with its disc), area lights sampled by power, and the sky as an
importance-sampled environment were there. Added, in three commits with tests:

1. Point lights with directional profiles (`light::Profile`, PBR 12.1-12.2): spots with
   pbrt's smoothstep falloff, goniometric lights from an equirectangular image, projectors
   (the image's frame matches the camera's, so a slide reads correctly from behind), and
   `"power"` in place of intensity.
2. Textured diffuse area lights (PBR 12.5.1): `MaterialModel::emitted_at` looks the image up
   at the hit's, or the sampled point's, texture coordinates.
3. The light BVH (PBR 12.6.3, Conty Estevez and Kulla 2018), `"light_sampler": "bvh"`: bounds,
   power and emission cones per node, importance without the receiver's normal (so the trace
   can evaluate the pmf for MIS), median splits. Beams weigh in at 4π times their intensity, as
   in pbrt; up to 16 point lights are still summed at every vertex, which beat choosing one on
   `lights.json`. On 900 lamps at 16 spp (`lamp-field.json`), mean FLIP against a 512 spp
   reference falls from 0.50 (power) to 0.15; on `lights.json`, with 80 bulbs near the wall,
   RMSE 0.357 to 0.352.

### Deferred from chapter 12

- The receiver's normal in the BVH's importance (pbrt uses it): needs the previous vertex's
  normal where the trace weights a light it hits. And pbrt's SAH-style split cost over the
  emission cones; ours split at the median.
- Spherical triangle and rectangle sampling (PBR 6.5.4, 6.6.3, from chapter 6), and
  emissive media in the BVH (chapter 11).
- Textured emission on meshes is exact only when hit; light sampling takes the image's mean,
  not knowing which triangle it sampled.
- IES photometric files for goniometric lights (pbrt converts them to images offline).
- `"light_sampler": "bvh"` is not the default, to keep existing renders.

### Chapters 13-14: light transport (done)

Path tracing with MIS and Russian roulette, volumetric path tracing (delta and ratio tracking)
and, from chapter 11, spectral MIS were there. Added, in two commits with tests:

1. Layered BSDFs (PBR 14.3, Guo et al. 2018), `coated_diffuse` and `coated_conductor`: pbrt's
   dielectric, diffuse and conductor BxDFs in a local frame (reflection/transmission flags,
   radiance vs importance), and `LayeredBxDF`'s stochastic `f`, `Sample_f` and `PDF`. One
   change from pbrt: a walk vertex's connection through the exit interface is weighted against
   next-event estimation's density for the same interior direction (`pdf(ω_i, -w)`, not
   `pdf(-w, ω_i)`, which differs for refraction); with pbrt's, `f` integrated to 0.90 where
   sampling delivers 0.854 under a rough coat. Checked: a smooth clear coat over white loses
   nothing; light sampling agrees with BSDF sampling within 4-6%; pbrt's `PDF` estimate
   integrates to up to two (it is only for MIS weights). pbrt's defaults lose light: 7% of a
   white base's to the 10-bounce walk, more to the coat's unit extinction at 0.01 thickness.
2. Random-walk subsurface scattering (`subsurface`; Chiang, Kutz and Burley 2016, Wrenninge et
   al. 2017, Cycles), in place of pbrt-v3's tabulated BSSRDF: a Fresnel sheen, diffuse
   transmission in, a walk bounded by the object's own geometry with the flight's channel
   chosen by throughput and balance-heuristic weights, and a diffuse exit with next-event
   estimation (so the sun lights the inside). Checked: without absorption it conserves energy
   in every channel; Chiang's fits make a thick slab look its albedo within 0.1. Refracting in
   and reflecting back by Fresnel at the exit (first tried) made albedo 0.3 look 0.16, since
   the fit assumes an index-matched boundary. `radius` is a distance in scene units: with
   albedos near one, light diffuses several radii, so objects not much bigger than that leak
   it out of their far sides and look darker and less red than the albedo (as in Cycles).

Demo `light-transport.json`.

### Deferred from chapters 13-14

- Light tracing (pbrt's `LightPathIntegrator`) and the online chapters' BDPT, MLT and SPPM:
  they splat to arbitrary pixels, which needs a shared film and the camera's importance
  function; caustics through glass (from point lights and the sun) stay out of reach.
- Rough or coated boundaries on subsurface objects (the sheen is a mirror), and textured
  subsurface albedo; walks are capped at 1024 steps, losing the rare deep ones in large, barely
  absorbing objects.
- Layered BSDFs that transmit (pbrt's thin layered dielectrics), textured coat roughness and
  thickness; pbrt's `PDF` estimate is used as is.

### Chapters 15-16: wavefront rendering (done, on the CPU)

Chapter 15 is pbrt-v4's wavefront renderer for GPUs, chapter 16 a retrospective. Option 1 of
the three discussed (CPU wavefront, then GPU with wgpu, or a hybrid): in three commits,

1. A flat scene (`src/wavefront/flat.rs`): `Geometry::flatten` (spheres, triangles, meshes,
   boxes, planar quads, planes, transforms of them) and `MaterialModel::flat` (untextured
   lambertian, plastic, metal, dielectric, isotropic conductor, diffuse-light, checkerboards)
   into primitive arrays, the existing array BVH and a `FlatMaterial` enum. The skysphere stays
   an object, evaluated for escaping rays; aerial perspective is applied as in `trace`.
2. The wavefront integrator (`src/wavefront/integrator.rs`): per pass, queues of paths through
   generate, intersect, escape/emission (MIS), shade (sorted by material; next-event
   estimation queues shadow rays) and shadow stages, each path carrying its own Sobol'
   `SampleState` instead of thread-local state. `"integrator": "wavefront"`, `--wavefront`;
   unflattenable scenes fall back with the reason (15 of 68 demo scenes flatten).
3. Matching the path tracer: sky sampling, sphere lights by cone, the sun's disc for narrow
   lobes, the light BVH, regularisation after the first non-delta bounce.

Measured at 320x180, 64 spp, against a 512 spp path-traced reference (mean FLIP, time on 4
cores): spheres 0.0154 / 1.22 s path, 0.0209 / 1.66 s wavefront; instances 0.0205 / 3.07 s,
0.0223 / 3.77 s; lamp-field 0.0803 / 1.58 s, 0.0848 / 1.82 s; demo 0.0023 / 1.72 s, 0.0025 /
2.00 s. At 512 spp on demo.json the wavefront image fits the reference to 0.998, mean FLIP
0.0016 (path seed-to-seed 0.0012). Findings: one queue for the whole image was memory-bound,
twice the path tracer's time; batches of 4096 paths per thread brought it to 1.15-1.4 times.
Sorting by material costs little on the CPU. A 4.5% gap on demo.json was the path tracer's
regularisation, which the wavefront integrator lacked.

### The GPU (stacked PR)

`src/gpu/`, behind the cargo feature `gpu` (wgpu 30, pollster, bytemuck), `--gpu`: the flat
scene uploaded (primitives with planes after them, the BVH's nodes via `Bvh::export`, materials
with checkerboards' squares appended, point and spot lights, and one table of f32 for the
emitters' distribution, the baked sky and the aerial perspective table), and a WGSL compute
shader porting the wavefront integrator's shading in f32: Lambert, rough and smooth plastic,
metal and conductor (complex Fresnel), glass, emitters with MIS, sun (point, or its disc for
narrow lobes), one area light by power, the sky by importance (marginal and conditional CDFs of
a 1024 x 512 map, looked up bilinearly), regularisation and Russian roulette. Camera rays are
generated on the CPU each pass and uploaded, so every camera works. Developed and tested on
Mesa's lavapipe (llvmpipe, Vulkan 1.4) in the container.

At 320 x 180, 512 spp against a 512 spp path-traced reference: demo exposure 0.998, FLIP
0.0017; instances 1.001, 0.016; spheres 1.000, 0.017 (the CPU wavefront integrator 0.013). Two
fixes on the way: aerial perspective, first omitted for scenes under 100 units (but planes are
unbounded), is baked into a table along rays from the camera (azimuth from the sun, elevation,
log distance); and that table's frame must be the sky's sun, not the scene's `sun`, which
scenes with a skysphere may not have. On lavapipe (4 cores) at 640 x 360, 64 spp: spheres 3.6
s (path tracer 4.7 s), instances 16.1 s (12.4 s), demo 6.1 s (7.1 s). Timings on a real GPU
are still to be measured.

The shader then moved from WGSL to Rust, compiled by rust-gpu (spirv-builder 0.11): a
separate workspace `shaders/` pinned to its nightly (`nightly-2026-08-15`; its README said
2026-04-11, its toolchain check disagreed), built by `scripts/build-shaders.sh` into the
committed `src/gpu/pathtrace.spv`, so the renderer stays on stable. The same crate builds for
the CPU on stable: the host uses its structs as the buffer layouts, and tests check its
Fresnel, conductor, GGX and sphere-light functions against the renderer's, and its whole
`radiance` on the CPU against the GPU's image (within 1%). Images are identical to the WGSL
version's (same FLIP figures); on lavapipe it runs 5-25% slower, after removing bounds checks
on the scene's buffers (`at`), which had cost a third. What rust-gpu or naga refused:
`(a..=b).contains(&x)` (a pointer in `Bound`), `f32::is_finite` (an infinite literal), and
slices of local arrays.

### Deferred from chapters 15-16

- Sharing the shading with the CPU wavefront integrator itself: it is f64 and nalgebra, the
  shader f32 and glam; for now the tests hold them together.

- Measuring on real GPUs (the Mac's Metal), and tuning: workgroup size, f16 storage, stack
  depth. The shader is a megakernel (one path per invocation); pbrt's GPU renderer queues
  paths between kernels per stage, which helps divergent materials on real GPUs.
- On the GPU: the light BVH (area lights are chosen by power), Sobol' samples (a hash per
  pixel, pass and dimension instead), goniometric and projection lights, aerial perspective
  from each ray's own origin (the table assumes the camera's), the sun's irradiance by altitude.
- Coverage: textures, coated and rough-dielectric materials, cylinders and other analytic
  shapes, media, spectral and motion-blurred renders; the scene's other samplers.
- pbrt's structure-of-arrays queues; ours are arrays of path structs.

### Reading (chapter 4)

- PBR 4th ed., chapter 4 (§4.4.1 blackbody, §4.5 spectral distributions, §4.6 colour, §4.6.6
  RGB to spectrum), §5.4.2 (`SampleVisibleWavelengths`).
- Wilkie, Nawaz, Droske, Weidlich, Hanika, "Hero Wavelength Spectral Sampling", Computer Graphics
  Forum 33(4) (EGSR 2014), doi:10.1111/cgf.12419.
- Jakob and Hanika, "A Low-Dimensional Function Space for Efficient Spectral Upsampling",
  Computer Graphics Forum 38(2) (Eurographics 2019), doi:10.1111/cgf.13626.
- CIE 015:2018, "Colorimetry", 4th ed. (matching functions, D65, CIELAB).
- Bruneton, "Precomputed Atmospheric Scattering: a New Implementation", 2017
  (<https://ebruneton.github.io/precomputed_atmospheric_scattering/>), including its evaluation
  of RGB against spectral rendering of the sky; ASTM G-173 (solar spectrum); Serdyuchenko et al.,
  "High spectral resolution ozone absorption cross-sections - Part 2", AMT 7, 2014,
  doi:10.5194/amt-7-625-2014.
- Malitson, "Interspecimen Comparison of the Refractive Index of Fused Silica", JOSA 55(10), 1965,
  doi:10.1364/JOSA.55.001205.

## 12. GPU and WebGPU roadmap: one set of kernels, per-area fallback

Goals (from review, November 2026):

- WebGPU in the browser is a target, not an afterthought: anything that runs on the GPU must
  survive translation to WGSL and WebGPU's limits, and the host must work without threads,
  blocking waits or a filesystem.
- The GPU's coverage grows one area at a time (a material, a shape, a kind of light), and
  whatever an area doesn't yet cover falls back to the CPU for that area only, not for the
  whole scene as now.
- One source of truth for shading: the kernel crate (`shaders/pathtrace`, f32, rust-gpu),
  run on the GPU and, for fallback and tests, on the CPU. The recursive tracer (`trace`, f64)
  stays the reference every other path is measured against.

### Review findings (code review of the wavefront and GPU paths)

Verified bugs, to fix first:

1. Mirroring transforms flip flattened triangles' normals: `Prim::transformed` keeps the
   winding when det < 0 (`flat.rs`), so one-sided emitters light the wrong side and
   dielectrics swap entering and exiting. Swap two vertices (and their normals) when det < 0.
2. Emissive (and dielectric) planes are one-sided in the flat scene and the shader, two-sided
   in `trace` (`Plane::intersects` faces the ray). Match it, or refuse to flatten them.
3. The shader's sphere-light cone, 1 - √(1 - r²/d²), cancels in f32: about twice too wide at
   r/d = 2.5e-4, and an infinite density (the light vanishes) below 1.7e-4. Use x / (1 +
   √(1 - x)), as the CPU's series does.
4. The shader samples the sky at texel centres only (no jitter within the texel), a slight
   MIS bias and aliased sky shadows. Keep the remainder of the uniform numbers, as
   `Distribution1D::sample`.

Risks: scenes past the GPU's buffer limits panic in wgpu rather than falling back (check sizes
against the adapter's limits and return Err); a full BVH stack in the shader drops nodes
silently (tie its size to `bvh.rs`'s depth bound); the shader's box test can make NaN for a
ray starting on a slab, which `TraversalRay::enters` handles; resuming a render saved by
another integrator mixes sample sequences (put the integrator in the state's key).

Maintainability: constants and table layouts are still duplicated between host and shader
(flags, material kinds, aerial table sizes, `MIN_ALPHA`, `POINT_SUN`, `LIMB_DARKENING`): make
them the kernel crate's `pub const`s. Smooth emitters take their side from the winding, not
the interpolated normal as `trace` does, and `SmoothMesh::flatten` drops all normals when one
is missing. Shading exists three times (`trace`, `wavefront::integrator`, the shader), and
the agreement tests compare image totals within 3% on scenes without transforms, plane
lights or small lights: compare regions (FLIP per tile), and add those cases.

Infrastructure:

- CI (`rust.yml`) never builds `--features gpu`, never runs the GPU tests, and can't notice a
  stale `pathtrace.spv` after the shader's source changes. Each run takes 35-40 minutes
  (debug-mode tests, every demo scene).
- `.github/workflows/dependabot.yml` is Dependabot's config in the workflows directory, so it
  fails as a workflow on every push; it belongs at `.github/dependabot.yml`, with ecosystem
  `cargo`, not `rust`.
- rust-gpu inlines the whole kernel into one function (the struct of buffer references needs
  inlining to legalise): 330 KB of SPIR-V becomes 1 MB of WGSL, a risk for browsers'
  compile times.
- PR #123 is 148 files and 31,000 lines, too large to review as one; review it by chapter
  (its commits are per chapter).

WebGPU feasibility, checked: naga translates `pathtrace.spv` to WGSL, which re-parses and
validates with WebGPU's baseline capabilities. The renderer itself doesn't build for
`wasm32-unknown-unknown`: `nv-flip-sys` (C++) and `getrandom` stop it first, and the terminal
UI, file IO and rayon's threads would follow.

### Architecture to reach

- Crates: `rays-core` (scene parsing, flat scene, CPU integrators; no terminal UI, no C
  dependencies; rayon behind a `threads` feature), `rays-kernels` (today's
  `shaders/pathtrace`: `no_std`, f32, run by the GPU and by the CPU fallback), the `rays` CLI
  (indicatif, nv-flip, EXR, saved states), and `rays-web` (wasm-bindgen: a canvas, scenes and
  assets by fetch, async readback).
- The GPU renderer becomes a wavefront in GPU buffers, as pbrt-v4's: path state as structures
  of arrays, and a kernel per stage (camera rays, intersect, classify, shade per material
  kind, shadow rays, accumulate), with queues between them.
- Per-area fallback through those queues. Each area has a status: GPU, CPU fallback or
  unsupported (the scene then renders with `trace`, as now). Classification after
  intersection puts each path in a GPU queue or a CPU queue; the CPU queue is read back,
  processed by the kernel crate's CPU build or the renderer's own code, and written back for
  the next stage. Intersection falls back the same way: objects the GPU can't intersect
  (procedural shapes, CSG) are intersected on the CPU for the rays whose segments reach their
  bounds, and the nearer hit kept.
- Shaders shipped as WGSL generated from the SPIR-V at build time (naga), so the web build
  needs neither naga nor SPIR-V at run time, and the native build can keep SPIR-V.

### Phases

0. Hardening: done. The four bugs and the risks above are fixed (buffer sizes and BVH depth
   checked against the device and the shader's stack; a NaN-safe box test; resumed renders
   catch up); shared constants live in the shader crate; a hard-case scene is checked tile by
   tile against `trace` and the GPU against the wavefront integrator (it caught f32 losing a
   small, distant sphere light, now sampled by pbrt's cone method). CI builds and lints
   `--features gpu`, runs the GPU tests on lavapipe, checks `pathtrace.spv` against the hashes
   of its sources (`scripts/shader-sources.sh --check`), runs tests in release, and Dependabot's
   config is in `.github/dependabot.yml`.
1. Web spike: done, in Chrome (headless, WebGPU on SwiftShader); Safari not yet tried. What
   was built, and how it differs from the plan:
   - The renderer is a library (`src/lib.rs`) and a CLI behind a default `cli` feature, rather
     than separate crates: the same dependency split (no C, no getrandom, no terminal in the
     library) with less churn while PR #123 is open. The library builds for wasm32 (rustfft
     2 -> 6, tobj without ahash, web-time's Instant, `build_apart` inline, `assets` for files).
   - `web/` (rays-web, wasm-bindgen): parses a scene, flattens it, renders a pass per animation
     frame and reads back every 8, finishing the image as the CLI does. `web/test.mjs` renders
     in headless Chromium; CI's `web` job compares that with the native GPU's image by FLIP.
   - The kernel shrank instead of losing its struct of references: one call site each for
     the traversal, shadow rays and lobe evaluation (WGSL 1.1 MB -> 390 KB; 2.4x faster on
     lavapipe). One more copy remains (the path's and the shadow rays' traversals, 50 KB).
   - WGSL is generated at build time and validated for WebGPU (`shaders/builder`).
   - Measured, 320x180, SwiftShader: shader compile and first pass 3-4 s; 250-540 ms a pass
     (software); scene setup 10.5 s, mostly baking the sky map single-threaded; camera rays
     17.7 ms a pass on the CPU, which would bound a real GPU's rate at larger sizes.
   - Found: a shadow ray re-hit its own sphere where sqrt is a few ULP out (WebGPU allows it),
     darkening spheres; fixed by the sphere test's early out. Browser vs native GPU, mean FLIP:
     0.0033 on `spheres` at 256 spp (CPU seed to seed: 0.012), 0.0003 on `demo.json`.
   - Found on the Mac's Metal (the first run on a real GPU): images 12% dark wherever the sky
     was sampled, and the GPU tests failing (they pass on lavapipe). naga's MSL and HLSL
     writers write a loop's continuing block before its body, so the body's values it uses
     (and its `break if`) are re-evaluated after the block's own stores: rust-gpu's
     `while lo + 1 < hi` left one iteration early (the sky's binary search, BVH traversal),
     and a count kept across iterations came out a quarter of the right one. The builder now
     folds continuing blocks into loop bodies (`fold_continuing_blocks`), Metal and DX12
     take the WGSL, and a test checks the WGSL's loops without a GPU. Demo, 20 spp, mean FLIP
     against `trace`: 0.1425 -> 0.0295 (the CPU wavefront integrator: 0.026). Not yet
     reported upstream (gfx-rs/wgpu).
   - Next for the web: camera rays on the GPU (pinhole and thin lens), the sky baked on the GPU
     or smaller, a scene picker and progressive display on the GPU, timings on real GPUs
     (the Mac, phones), Safari.
   - Benchmark of the megakernel on the M2 (Metal; 8 CPU threads), after the fixes in #128,
     #140-#143 (`scripts/benchmark-gpu.sh 640 360 8 64 256`): the 17 demo scenes the GPU
     renders, cost of a sample from the difference between 8 and 64 spp, FLIP at 64 spp
     against `trace` at 256. ms a sample, trace / wavefront / GPU, and FLIP, wavefront / GPU:

     | Scene            | trace | wavefront |  GPU | GPU setup | FLIP wf | FLIP GPU |
     |------------------|------:|----------:|-----:|----------:|--------:|---------:|
     | demo             |  46.2 |      41.3 | 19.0 |    1.2 s  |  0.0167 |   0.0197 |
     | astigmatism      |   6.1 |       7.9 |  4.7 |    0.3 s  |  0.0062 |   0.0062 |
     | glossy-sun       |  43.6 |      40.7 | 15.2 |    1.2 s  |  0.0178 |   0.0214 |
     | instances        |  83.3 |      89.0 | 38.2 |    1.2 s  |  0.0164 |   0.0208 |
     | lamp-field       |  55.5 |      53.3 |  7.5 |    0.3 s  |  0.0702 |   0.3736 |
     | lamps            |  45.7 |      33.8 | 21.9 |    0.3 s  |  0.0272 |   0.0298 |
     | lens-flare       |   7.1 |      48.0 |  5.7 |    4.0 s  |  0.0049 |   0.0109 |
     | limb-sunset      | 278.2 |     285.9 |  4.7 |    2.4 s  |  0.0011 |   0.0885 |
     | night-orbit      | 400.5 |     406.0 |  6.2 |    0.7 s  |  0.0044 |   0.0562 |
     | panorama         |  30.5 |      24.3 | 13.3 |    0.3 s  |  0.0118 |   0.0146 |
     | realistic-camera |  65.5 |      59.1 | 25.9 |    0.4 s  |  0.0566 |   0.0594 |
     | sky-custom       |  47.9 |      47.7 |  5.6 |    1.1 s  |  0.0094 |   0.0200 |
     | sky-presets      |  52.7 |      52.7 |  5.2 |    1.2 s  |  0.0113 |   0.0207 |
     | sky-sunset       |  56.0 |      48.6 |  7.1 |    1.1 s  |  0.0107 |   0.0179 |
     | spheres          |  66.4 |      41.8 |  7.5 |    1.1 s  |  0.0164 |   0.0201 |
     | stratosphere     | 336.0 |     356.0 |  4.0 |    3.4 s  |  0.0037 |   0.1780 |
     | deep-space       | 145.3 |     152.7 |  4.1 |    0.5 s  |  0.0007 |   0.0414 |

     What it shows:
     - Geometric mean, a sample: 7.2x the wavefront integrator, 6.8x `trace`; at 64 spp with
       setup, 2.9x the wavefront integrator. Not a like-for-like comparison where the sky
       dominates: there the CPU evaluates the atmosphere for every ray and the GPU looks it up
       in a baked map (35-90x faster), which is also why the GPU is worse there (the map's
       0.35 degrees a texel bands a narrow view of the limb, and smears stars).
     - On scenes of geometry and materials (demo, glossy-sun, instances, lamps, panorama,
       realistic-camera, astigmatism) the GPU is 1.5-2.7x the 8-thread CPU. On the demo a pass
       takes 18.6 ms, 2.8 ms of it the CPU's camera rays: the shader is the bound, the case for
       the queued stage kernels below. With `realistic-camera` the reverse: tracing camera rays
       through the lens on the CPU (28.7 ms a pass) is the whole cost; camera rays on the GPU
       would remove it.
     - At equal samples the GPU is noisier everywhere (demo 0.0197 against 0.0167): its hashed
       random numbers against the CPU's Sobol' points.
     - `lamp-field` asks for the light BVH, which the GPU ignores silently, choosing among 900
       lamps by power: unbiased (same exposure) but 5x the error. It should at least say so.
     - Not the GPU: on `lens-flare` the CPU wavefront integrator takes 48 ms a sample against
       `trace`'s 7.1.
     - 16 of 72 demo scenes reach the GPU (plus `deep-space` after #142); the rest stop at
       flattening (media, procedural shapes, textures, spectral rendering): phase 3's fallback.
   - Sky and space on the GPU (done): the GPU's own sky, not an
     approximation of it. Goal: as little difference from the CPU as f32 allows.

     Result: ray by ray, the shader's air is within 3e-5 of the renderer's (sky, aerial
     perspective, sunlight; ground to orbit, below the horizon, moonlit) and its space within
     4e-4 (`test_air_matches_the_renderer`, `test_space_matches_the_renderer`). Images, 640 x
     360, 64 spp, mean FLIP against `trace` (GPU, then wavefront): limb-sunset 0.0011 / 0.0011
     (0.0885 with the map), stratosphere 0.0020 / 0.0017 (0.178), night-orbit 0.0039 / 0.0043
     (0.056), sky-sunset 0.0117 / 0.0107, sky-custom 0.0105 / 0.0094, glossy-sun 0.0200 /
     0.0175. The sun's disc needed its angle from the chord and its solid angle as
     4π sin²(r/2): from f32 cosines its edge was ragged (cos θ within rounding of 1).

     Found, not yet solved: the kernel's size. rust-gpu inlines everything into one kernel;
     the air and space took the WGSL from 396 KB to 834 KB (after folding the sky's two call
     sites into one, and space's seven of fbm into three). A demo sample went from 19 ms to
     48 ms (the CPU's: 41), though the demo has no space: code that never runs still costs
     registers and occupancy. In a browser the shader's compile went from 2.7 s to 11.8 s.
     The remedy is the queues of phase 2 below: sky and space in their own kernel, fed the
     escaping rays, the sky's light samples and aerial perspective's in-scattering (the
     transmittance, which the path needs at once, is a short march of depths alone).

     Done: a wavefront by bounce, on #124. Per sample on the M2 at 640 x 360 (baked map /
     exact in one kernel / exact, split; the CPU wavefront integrator's): demo 19 / 48 / 13.9
     ms (41), spheres 7.4 / - / 11.8 (42), sky-sunset 7.1 / - / 11.4 (49); limb-sunset 46,
     night-orbit 58, stratosphere 23 ms (the CPU's 286, 406, 356). Images unchanged (the same
     random numbers). In Chromium (WebGPU on the M2): compile and first pass 7.4 s (2.7 s with
     the map, 11.8 s in one kernel); spheres at 160 x 90 35.5 ms a pass (16.4 ms with the
     map), where a pass's `max_depth` + 2 rounds of dispatches cost more than the pixels.
     Browser against native: FLIP 0.0025 (spheres), 0.0002 (night-orbit).

     Design: a wavefront by bounce. A compact queue of sky queries would need
     several of a pixel's results summed, and WebGPU has no float atomics; one bounce at a time
     bounds each pixel's queries at three, in fixed slots. Two entry points: `bounce` advances
     each path one vertex (its state in a buffer between dispatches; the camera rays' buffer
     becomes it, which keeps to WebGPU's 8 storage buffers) and fills its pixel's slots
     (escaping ray, sky light sample, aerial in-scattering: origin, direction, weight, and a
     distance for the last); `sky` evaluates a pixel's slots at one call of `background` and
     adds them to its sum. A pass is `max_depth` + 1 pairs of dispatches. Known: images must
     not change (the same random numbers, in the same order; tests compare the shader's CPU
     build and the GPU). Assumed, to measure: each kernel's code is small enough to recover the
     megakernel's speed, and the extra dispatches cost less than they save.

     Known (from the code): the CPU's escaping rays take `LUTSky::air_radiance`, a lookup in
     `SkyViewLUT` (8 altitudes, 1-200 m, 256 x 128, Rayleigh, Mie and multiple scattering
     before the phase functions, which are applied exactly) above the horizon and below 200 m,
     and otherwise `Air::scattering`, an exact march (32 view steps by `span_samples`, 16 towards
     the sun at each, `MultipleScattering`'s 32 x 32 table); plus a moonlit copy of the sky
     (`LUTSky::moonlit`) and `space_seen` (space's radiance times `transmittance_to_space`,
     skipped below `threshold` of the sky's light). Hits are hazed by `Air::segment` from each
     ray's origin (a 32-step march with the sun's depth table). Sunlight at a shading point is
     `Sun::irradiance_at` (the sun's depth table there); the sun's disc is from the ray's
     origin. The GPU instead bakes a 1024 x 512 map and an aerial table from the camera, and
     uses one sunlight colour: all three approximations.

     Plan:
     1. Air: upload the atmosphere's parameters, the sun's depth table, the multiple
        scattering table and the sky-view tables (about 9.4 MB a sky; WebGPU allows 128 MB a
        binding); port `air_radiance` (table and exact march), the moonlit sky, `segment`,
        `sun_transmittance` and the disc to the shader. Escaping rays, hits, sunlight and the
        disc then evaluate per ray, as on the CPU. Test: the shader's CPU build against
        `LUTSky` and `SkyMaterial` ray by ray (on the ground, at 200 m either side of the top
        table, from orbit, below the horizon, at the terminator), then images against the
        wavefront integrator by FLIP.
     2. Space: stars from a catalogue built on the CPU (`Stars::each`, sorted by cell, with
        each cell's offset: the shader can't run `Pcg32`, which needs 64-bit integers WebGPU
        lacks), evaluated as `Stars::radiance` over the same neighbourhood; the Milky Way,
        nebulae and galaxies from their parameters and `SeededNoise`'s permutation tables,
        ported (Perlin, fBm, warped fBm, reddening); `bound` and `threshold` as the CPU.
     3. The baked map stays only as the distribution sky samples are drawn from (MIS keeps
        that unbiased); its radiance is no longer used.

     Assumptions, to check: f32 is close enough given altitudes computed without
     cancellation (h = (|p|^2 + 2 R p_y) / (|p - c| + R) about a planet centred below the
     origin, not |p - c| - R, which loses 0.5 m at Earth's radius); the exact march (512 exps
     a ray) is affordable for the escaping rays of orbital scenes; the WGSL stays within what
     browsers compile (rust-gpu inlines everything: few call sites for the march and noise).
     Spectral skies stay on the CPU (the GPU doesn't render spectrally).
2. GPU wavefront with queues: replace the megakernel with stage kernels and queues, with
   identical images (FLIP against the megakernel), then measure on real GPUs (the Mac; the
   browser) against the megakernel to decide which to keep for which device.

   Started: a wavefront by bounce (`bounce_paths`, `answer_sky`; above). Still to come:
   - Queues of active paths (a u32 atomic counter, `dispatchWorkgroupsIndirect`), so late
     rounds with few paths left cost little; most for the browser, where a pass is now about
     20 full-image dispatches. Queues and counters must share buffers: all 8 of WebGPU's
     storage bindings are in use. Spike first: atomics and indirect dispatch through rust-gpu
     and naga to Metal and the browser.
   - Stages: intersect (sorting hits into queues: escaped, emitter, each material kind),
     shade per material kind, shadow rays, sky; the random numbers drawn in the same order,
     so images and the GPU-against-CPU-build tests hold.

   Mesh intersection, the other large speedup. Known (the shader's `trace`, `bvh.rs`): a
   binary SAH BVH, leaves of up to 8; a per-thread stack of 100 in a local array (likely
   spilled); a node's box tested when it is popped, children pushed in a fixed order, so
   neither nearer-first nor skipped when missed; each test loads a whole 112-byte `Prim`,
   normals included; instances flattened into world triangles (no instancing on the GPU).
   In order:
   1. Measure: box and triangle tests per ray (counted as rays are), and a pass's time with
      shading off, on demo, instances and glossy-sun.
   2. Ordered traversal (Aila and Laine, "Understanding the Efficiency of Ray Traversal on
      GPUs", HPG 2009): both children's boxes in the parent, tested together, the nearer
      visited first, the farther pushed only if hit.
   3. A compact buffer for the test (a vertex and two edges), normals and materials fetched
      for the hit alone.
   4. The stack sized by the tree's depth (`bvh_depth`), not 100.
   5. Intersection as its own stage (above), with an any-hit kernel for shadow rays, and rays
      sorted for coherence once they are queued.
   6. A wide BVH (4 or 8 children; compressed, after Ylitie, Karras and Laine, "Efficient
      Incoherent Ray Traversal on GPUs Through Compressed Wide BVHs", HPG 2017).
   7. Two levels: a tree of instances over a tree per mesh, so instanced scenes (forests)
      don't multiply their triangles.
   8. Perhaps, natively only: hardware ray queries (wgpu's are experimental; WebGPU has
      none; the M2 has no ray tracing hardware, the M3 does).
   1-4 apply to the current kernels, and come first.

   Measured (step 1; `bench_passes`, GPU time a pass at 640 x 360 on the M2, the fastest of
   three, as macOS's own work loads the machine): demo 22.8 ms, instances 39.6, glossy-sun
   18.9, spheres 7.7 (after paths that have ended stopped reading and writing in later
   rounds: 26.9, 40.0, 21.1, 9.3 before). Per ray, from the shader's CPU build: demo 3.3 box
   and 1.3 triangle tests (its teapot is small; most rays hit the plane, tested first, or
   escape), instances 21.4 and 2.3, glossy-sun 3.3 and 1.7. Where the time goes: the demo's
   first round (bounce and sky) 8.5 ms, its nine later rounds 14 ms, though paths average two
   vertices: about 0.8 ms a dispatch of a large kernel over every pixel, idle or not (reading
   only an ended path's flags changed nothing, so not memory). Instances: its first round
   20 ms, traversal-bound. So queues with indirect dispatch (above) come before intersection
   for most scenes; intersection (2-4) for scenes of many triangles, as instances.
3. The fallback framework: per-area status, classification, CPU queues for materials first,
   then intersection. Exit: every flattenable demo scene plus at least one with a
   procedural shape and one with an unported material renders through the GPU path with CPU
   fallback, matching `trace`.
4. Port areas to the GPU by use, each with a kernel, a test of its CPU build against the
   renderer, and a FLIP test of the GPU against `trace`. Order, by how often the demo scenes
   use them: image textures (albedo; textures as arrays within WebGPU's limits), cylinders,
   cones and discs, the light BVH, Sobol' samples, `coated_*` and `rough_dielectric`,
   per-ray aerial perspective, grid media, then the rest. Spectral rendering and motion blur
   stay CPU until the queues carry wavelengths and times.
5. Convergence: the CPU wavefront integrator runs the kernel crate's code for the areas it
   covers, removing the second copy of the shading (the decision needed: whether f32 is
   acceptable there, or the kernels become generic over the float type).

Each phase ends with the GPU and web paths measured against `trace` by FLIP on the demo scenes
they can render, and timings on the Mac's GPU and in a browser, not only on lavapipe.

### Open decisions

- Whether to split PR #123 into per-chapter PRs for review.
- f32 for the CPU wavefront integrator when it shares the kernels, or generic kernels.
- WGSL generated at build time (recommended) or translated by naga in the browser.
