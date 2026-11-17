# CLAUDE.md - Rays.rust Project Guide

## Project Overview
Rays.rust is a raytracer written in Rust with the following features:
- Whitted tracing and path tracing with Monte-Carlo global illumination
- Material models: Lambertian, Specular, Dielectric, Plastic, Leaf (translucent, `src/material/leaf.rs`), procedural Rock and Soil (`src/material/soil.rs`: mottled topsoil, stones, litter and moss; loam, clay, sand, forest_floor); snow, glacier ice and snow cover (`src/material/snow.rs`, docs/snow-and-ice.md: colours from ice's measured absorption and the size of grains or bubbles, by asymptotic radiative transfer; Kokhanovsky and Bréon's snow reflection with forward scattering; soot and dust; grain glints; glacier ice as a Fresnel surface over bubble-diffused light); bump mapping via an object's "normal" (src/material/bump.rs, docs/bump-mapping.md)
- Objects: Sphere, Plane, Mesh, OBJ file import; transforms and instancing of named "geometries"; POV-Ray's shapes (cylinder, cone, torus, blob, isosurface, text and more; see docs/shapes.md, parsed in src/scenefile_shapes.rs), combined by CSG (src/shapes/csg.rs); mountains (`"mountains"`: ridged-noise heightfield with snow, rock and vegetation), and a planet (`"planet"`: the ground as a sphere of the Earth's radius); clouds can cover the whole planet (`"mapping": "global"`: an equi-angular cube-sphere weather map and a planet-wide wind field), seen from orbit in `demo/scenes/orbit.json` (and the bare atmosphere at the limb and from the stratosphere: `limb-sunset.json`, `stratosphere.json`)
- Participating media: boxes or closed shapes (`"shape"`: media inside dielectrics) of constant, noise-driven or voxel-grid density (`src/volume/grid.rs`: Mitsuba `.vol`, pbrt's DDA majorant grid; delta tracking with local majorants, ratio-tracked shadows); chromatic media (`"sigma_a"`/`"sigma_s"` per channel) tracked by pbrt's spectral MIS (`src/volume/chromatic.rs`: a hero channel per path, ratios settled at the nearest hit and applied as `chromatic::weight()` where light is gathered; shadow transmittance is a `Color`); emissive media (`src/volume/emission.rs`: colour or blackbody temperature grids); Henyey-Greenstein or Mie (water droplet) phase; diffraction by particles (coronae, iridescence); ice-crystal halos traced through hexagonal prisms (sun dogs, 22°/46° halos, circumzenithal arc); rainbows traced through raindrops (`"rainbow": true`: primary and secondary bows, Alexander's dark band); optional similarity and Oz multiple-scattering approximations
- Skysphere with Rayleigh and Mie Scattering, and aerial perspective (distant objects attenuated and hazed by the same atmosphere); `"crepuscular_rays": true` lets clouds and terrain shadow the air (sunbeams)
- The rest of the universe (`"background": {"type": "space"}`, `src/procedural/space/`, docs/space.md): procedural stars (luminosity function, black-body colours), the Milky Way's glow and dust lanes, emission, reflection, planetary and dark nebulae, and spiral and elliptical galaxies; physically bright (magnitudes against the sun's, so it takes a night exposure, `"output": {"exposure": ...}`) unless `brightness` scales it; seen through a skysphere's air by its transmittance, and skipped where the sky outshines it (`threshold`) (`demo/scenes/deep-space.json`, `night-orbit.json`)
- Directional sun light (`"sun"`), sampled directly; follows the skysphere's sun and is reddened by its atmosphere
- Procedural objects including Ocean (Tessendorf's algorithm with Phillips spectrum; the planet's surface by default, or a plane with `"planet_radius": 0`: displaced near the camera, physically lit, with sun glitter and slopes calibrated to Cox and Munk) and volumetric clouds: cumulus to stratus (`"nubis_clouds"`, after Horizon Zero Dawn's Nubis) and cirrus with fallstreaks (`"hogan_kew_cirrus"`, after Hogan and Kew), altocumulus as convection cells (`"benard_altocumulus"`: Rayleigh-Bénard cells, rows, undulatus waves), lenticular clouds over a mountain's lee waves (`"lenticular"`: stacked plates from moist layers), mammatus under an anvil (`"mammatus"`), rain shafts and virga (`"precipitation"`: Marshall-Palmer extinction by rain rate, shear, evaporation), and cumulus grown by simulating a sunny day (`"harris_cumulus"`: stable fluids with buoyancy, condensation, latent heat and subgrid turbulence, through the ARM shallow-cumulus case's day; cached so `scripts/animate.py` can render it boiling and dissipating)
- Procedural trees and bushes (Weber and Penn's parametric model, `src/procedural/tree.rs`; bushes: shrub, hazel, hawthorn, juniper), forests of instanced
  trees (`src/procedural/forest.rs`; `"lod"` sees each tree at a level of detail for its pixel size, `src/shapes/lod.rs`, docs/adaptive-meshes.md: bark simplified by vertex clustering, leaves consolidated into cards that keep their leaves' normals and faces, each hit shaded as one leaf chosen by visible area; compact `CardSet`s and `forest::Placed` instances, trees grown one at a time; `demo/scenes/forest-aerial.json`), and rocks (`bedrock`, `boulder`, `pebbles` objects; `rock` material)
- Landscapes evolved through time (`"landscape"`, `src/terrain/`: uplift, rivers, glaciers, volcanoes, meanders) on a planet shaped by plate tectonics (`src/terrain/tectonics.rs`: rigid plates on Euler poles, ridges and ocean floor deepening with age, subduction arcs and trenches, continental collision and suturing, rifting, hotspots) and a climate powered by latitude (`src/terrain/climate.rs`: insolation from axial tilt, an energy balance model for temperature, Hadley/Ferrel/polar cells for winds and rain, moisture carried inland with rain shadows, Köppen-style biomes; sets glaciers' snowlines and the ground cover; `"climate_map"` writes a biome map); without `"scale"` drawn at full scale over the whole planet (`src/terrain/scenery.rs`: a whole-planet simulation with patches nested about the camera, each driven by the one around it; a geometry clipmap finest under the camera; ground cover from the simulations and climate; `demo/scenes/landscape.json`, `landscape-orbit.json`, `"planet_map"` writes a world map), evolved terrain cached by `"cache"`; ground cover uses physical snow (ageing to firn near the snowline) and glacier ice (white surface ice, blue bands, debris, crevasses where the ice steepens), and the load reports where the glaciers are (`demo/scenes/glacier.json`, a colder climate by `"warming"`)
- Heightmap terrain (flat, fractal noise or image) and procedural grass that grows on it, with species and condition presets (see `src/procedural/grass.rs`); wild meadows mix in flowers (daisy, oxeye, dandelion with seed clocks, buttercup), clover, grasses with seed heads (timothy, wild oat, bent), heather and bracken (a fern), and dead `litter` lying under them, as in `demo/scenes/wildflower-meadow.json`
- Spectral rendering (`"spectral": true`, `src/spectrum.rs`, after PBRT chapter 4): three hero wavelengths per path, RGB upsampled by Jakob-Hanika sigmoid polynomials (`src/spectrum/rgb2spec.rs`) via `Color::as_reflectance`/`as_illuminant`/`as_unbounded` where colours enter light transport (identity in RGB mode); dispersive glasses (`refractive_index`: named glass, Cauchy or Sellmeier) and blackbody emitters (`"temperature"`); the sky is evaluated at the sample's wavelengths (`SkyMaterial::air`, spectral data in `src/spectrum/atmosphere.rs`), so its outputs must not be upsampled again
- Cameras (PBRT ch. 5; `src/camera.rs`): thin lens (with astigmatism: eye-style cylinder or Seidel), orthographic, spherical, and realistic lens systems traced element by element (`src/lens.rs`, pbrt's lenses built in, polygonal blades); motion blur (`src/motion.rs`: `"shutter"`, `"motion"` on transforms and the camera; each sample's time is thread-local, like its wavelengths); Gaussian, Mitchell and Lanczos filters (signed weights); film (`src/film.rs`: ISO and exposure time, white balance, output colour spaces); lens flare (`src/flare.rs`: ghosts traced through coated lens surfaces from the image's brightest lights, coatings by thin-film transfer matrices (`Coating`: multi by default, single, none) averaged over each channel's band; and diffraction starbursts from the aperture's traced outline (`Flare::outline`; `LensSystem::blade_shape`: curved, per-blade jitter), with a real lens's flaws (`imperfection`: irregular blades, rough edges, front-element dust and scratches, uneven coatings; 0 for perfect); physical in strength, so scenes shouldn't scale `ghosts`/`starburst`)
- Shapes after PBRT ch. 6: texture coordinates on demand (`Geometry::uv`; `uv` and `checker` materials), sphere lights sampled by their cone (`Geometry::sample_from`/`pdf_from`), bilinear patches (`src/shapes/bilinear.rs`, `quad`), Bézier curves and `fur` (`src/shapes/curve.rs`), Loop subdivision (`src/shapes/subdivision.rs`), and ray offsets scaled to each hit (`Intersection::spawn_offset`, replacing fixed epsilons)
- Samplers after PBRT ch. 8 (`src/sampler.rs`, `"sampler"`): Sobol' (default), ZSobol' (blue-noise error), Halton, stratified, independent; all feed `geometry::rand()` through a thread-local sample state
- Materials after PBRT ch. 9: `conductor` (measured metals' complex IOR in `src/spectrum/metals.rs`, anisotropic GGX via `GGX::aniso`), `rough_dielectric` and `thin_dielectric` (`src/material/rough_dielectric.rs`), `mix`, and `hair` (pbrt's hair BSDF, `src/material/hair.rs`; needs cylinder curves, whose `Geometry::tangent` it reads)
- Textures after PBRT ch. 10 (`src/textures.rs`): image textures as MIP pyramids filtered by EWA over the camera-approximated footprint (`textures::footprint`, `Scene::footprint`), for `lambertian`/`plastic` albedo (`ColorTexture`) and `mix` amount; tangent-space normal maps via `Medium::normal_at_hit`; demo textures from `scripts/make-demo-textures.py`
- Lights after PBRT ch. 12 (`src/light.rs`, `src/light_bvh.rs`): point lights with a `Profile` (spot, goniometric, projection; `"power"`), textured diffuse area lights (`MaterialModel::emitted_at` with texture coordinates), and `"light_sampler": "bvh"`: pbrt's light BVH choosing one light per vertex by importance (`material::functions::sample_lights_with`; up to `scene::BVH_POINT_LIGHTS` point lights are still summed), with `LightBvh::pmf` for MIS
- Light transport after PBRT ch. 13-14: layered BSDFs (`coated_diffuse`, `coated_conductor`; `src/material/layered.rs`: pbrt's `LayeredBxDF` random walks, stochastic `eval`/`pdf`, specular samples traced without MIS) and random-walk subsurface scattering (`subsurface`; `src/material/subsurface.rs`: Chiang et al.'s albedo/radius fits, the walk tests only the object's own geometry, diffuse exit with next-event estimation)
- Wavefront integrator after PBRT ch. 15 (`src/wavefront/`; `"integrator": "wavefront"` or `--wavefront`): the scene flattened by `Geometry::flatten` and `MaterialModel::flat`/`Medium::flat_material` into `flat::FlatScene` (primitive arrays, BVH, `FlatMaterial` enum; the skysphere kept as an object for escaping rays), and `integrator::render_pass` advancing queues of paths stage by stage in cache-sized batches, each path with its own Sobol' `SampleState`; it must match `trace` (MIS, sun disc, regularisation, Russian roulette) and falls back to it with the reason when a scene can't be flattened
- Multithreaded rendering with progressive output

## Build Commands
- Build and run: `cargo run --release -- demo/demo.json`
- Run tests: `cargo test`
- Run specific test: `cargo test <test_name>`
- Lint code: `cargo clippy`
- Format code: `cargo fmt`
- View sample scenes: `cargo run --release -- demo/scenes/<scene_file>.json`

## Command-line Options
- `<FILE>`: Scene file (required, positional): our JSON, `.nff` (translated by `src/nff.rs`), or Tungsten's JSON (recognised by its `bsdfs` and `primitives`; translated by `src/tungsten.rs`). The image is written alongside it with a `.png` extension
- `--dump-scene <FILE>`: Write the scene as our JSON (for an NFF or Tungsten file, its translation)
- `-o, --output <FILE>`: Write the image here instead; `.png` (8-bit, display-encoded) or `.exr` (linear float)
- `-p, --progressive-render`: Update the output file during the render (at most once a second)
- `--width <NUM>`: Override scene file width
- `--height <NUM>`: Override scene file height
- `--seed <NUM>`: Override the scene's render seed; the same seed gives the same image
- `--samples <NUM>`: Samples per pixel to render to, overriding the scene's `supersamples`
- `--add-samples <NUM>`: Render this many more samples per pixel than the saved state has
- `--fresh`: Start afresh rather than resuming
- `--wavefront`: Render with the wavefront integrator, if the scene can be flattened (otherwise with the path tracer, saying why)
- Renders are resumable by default: the accumulated samples are saved every 30 s, and at the end, to `<output>.accum` (exact sums, keyed by a hash of the scene file without its `supersamples` and `output`, and the size and seed overrides; so exposure, film and flare can be changed and the render refinished). Rerunning carries on where it stopped, or from a quick render adds samples up to the new target (`--samples 8`, later `--samples 64`); the result is identical to an uninterrupted render. Editing the scene starts afresh; after changing the renderer or files the scene refers to, use `--fresh`
- `--compare <FILE>`: Compare with a reference image (eg. `references/cornell-box/TungstenRender.exr`, fetched by `scripts/fetch-references.sh`): prints exposure fit, RMSE and mean FLIP error; writes `<output>.flip.png` and `<output>.reference.png`
- `--max-flip <NUM>`: With `--compare`, exit with status 2 if the mean FLIP error exceeds this
- `--depth <FILE>`: Also write a depth image (distance to the nearest surface through each pixel centre); `.exr` keeps distances, `.png` scales them for viewing

## Code Style
- Traits used for major abstractions (Camera, Geometry, Medium, MaterialModel)
- Box<dyn Trait> for polymorphism
- Error handling via Option<T>/Result<T,E> with unwrap()
- New code that turns a scene's RGB into light transport (a material's albedo, an emitter's colour) must convert it with `as_reflectance`, `as_illuminant` or `as_unbounded`, so spectral mode stays correct
- Full paths with crate:: prefix for imports
- 4-space indentation
- snake_case for variables and functions
- Structs using PascalCase
- Extensive use of Vector3<f64> from nalgebra
- Multithreaded rendering with Rayon

## Project Structure
- Modules organized by functionality (material, shapes, procedural)
- Scene definitions stored in JSON files (in demo/ and demo/scenes/); other renderers' formats are translated into that JSON (`SceneFile::read_document`; see `scene-formats.md`)
- Rendering progress tracking via indicatif
- Sample scenes available in demo/scenes/ with corresponding .png outputs

## Development Best Practices
- Make small, incremental changes focused on a single concern
- Break complex features into smaller, sequential commits
- Commit frequently to create checkpoints (after each logical change)
- Run tests and linting before each commit
- Create dedicated branches for each distinct feature
- Write descriptive commit messages explaining the purpose of changes
- Keep feature branches simple and focused on a single goal
- Verify each change works before moving to the next step