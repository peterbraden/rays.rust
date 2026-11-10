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

### Reading (candidates, unverified)

- Glassner, "Space Subdivision for Fast Ray Tracing", IEEE CG&A 1984 (octrees).
- Goldsmith and Salmon, "Automatic Creation of Object Hierarchies for Ray Tracing", IEEE CG&A
  1987, and MacDonald and Booth, "Heuristics for Ray Tracing Using Space Subdivision", 1990
  (bounding volume hierarchies, the surface area heuristic).
- Wald, Boulos, Shirley, "Ray Tracing Deformable Scenes Using Dynamic Bounding Volume
  Hierarchies", ACM TOG 2007 (binned SAH construction).
- Arvo and Kirk, "Particle Transport and Image Synthesis", SIGGRAPH 1990 (Russian roulette).


## 3. GPU acceleration (brainstorm only)

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

- Transforms: translate and scale (the `Transform` intersection code already supports general
  affine transforms; only parsing is missing), and instancing.
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
- Time (`simulation.rs`): operator splitting, deterministic and resumable; morphological
  acceleration (MORFAC) for glacial erosion.
- Scene object `"type": "terrain"` (`object.rs`), maps (`map.rs`), demo
  `demo/scenes/terrain-alpine.json`.

### Measured

- A 96 x 96 patch of 500 m cells (48 km) through one 100 kyr glacial cycle: about 6 minutes on
  one core, nearly all of it ice flow around the glacial maximum. Ice covers almost the whole
  range at the maximum (about 1.4 km at its thickest, in basins), and at 100 kyr the valleys are
  broad troughs with cirque glaciers left on the peaks.
- The whole planet at 64 x 64 x 6 cells (about 170 km each): 20 kyr in under 4 seconds.

### Known limitations and open questions

- Sediment leaving at the sea is lost: no deltas or marine deposits. No moraines (glacial
  debris goes straight to the rivers), no lateral erosion by meandering, so alluvial valleys
  widen only by deposition. The sediment-to-bedrock erodibility ratio (3) and lava's (0.3) are
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
- Unchecked: Halfar's solution as given by Bueler et al., J. Glaciology 51(173), 2005 (used by
  `test_halfar_dome`, which passing within 3% supports).

