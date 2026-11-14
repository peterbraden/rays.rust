# CLAUDE.md - Rays.rust Project Guide

## Project Overview
Rays.rust is a raytracer written in Rust with the following features:
- Whitted tracing and path tracing with Monte-Carlo global illumination
- Material models: Lambertian, Specular, Dielectric, Plastic, Leaf (translucent, `src/material/leaf.rs`); bump mapping via an object's "normal" (src/material/bump.rs, docs/bump-mapping.md)
- Objects: Sphere, Plane, Mesh, OBJ file import; transforms and instancing of named "geometries"; POV-Ray's shapes (cylinder, cone, torus, blob, isosurface, text and more; see docs/shapes.md, parsed in src/scenefile_shapes.rs), combined by CSG (src/shapes/csg.rs); mountains (`"mountains"`: ridged-noise heightfield with snow, rock and vegetation), and a planet (`"planet"`: the ground as a sphere of the Earth's radius); clouds can cover the whole planet (`"mapping": "global"`: an equi-angular cube-sphere weather map and a planet-wide wind field), seen from orbit in `demo/scenes/orbit.json` (and the bare atmosphere at the limb and from the stratosphere: `limb-sunset.json`, `stratosphere.json`)
- Participating media: boxes of constant or noise-driven density (delta tracking with local majorants, ratio-tracked shadows); Henyey-Greenstein or Mie (water droplet) phase; diffraction by particles (coronae, iridescence); ice-crystal halos traced through hexagonal prisms (sun dogs, 22°/46° halos, circumzenithal arc); rainbows traced through raindrops (`"rainbow": true`: primary and secondary bows, Alexander's dark band); optional similarity and Oz multiple-scattering approximations
- Skysphere with Rayleigh and Mie Scattering, and aerial perspective (distant objects attenuated and hazed by the same atmosphere); `"crepuscular_rays": true` lets clouds and terrain shadow the air (sunbeams)
- Directional sun light (`"sun"`), sampled directly; follows the skysphere's sun and is reddened by its atmosphere
- Procedural objects including Ocean (Tessendorf's algorithm with Phillips spectrum; the planet's surface by default, or a plane with `"planet_radius": 0`: displaced near the camera, physically lit, with sun glitter and slopes calibrated to Cox and Munk) and volumetric clouds: cumulus to stratus (`"nubis_clouds"`, after Horizon Zero Dawn's Nubis) and cirrus with fallstreaks (`"hogan_kew_cirrus"`, after Hogan and Kew), altocumulus as convection cells (`"benard_altocumulus"`: Rayleigh-Bénard cells, rows, undulatus waves), lenticular clouds over a mountain's lee waves (`"lenticular"`: stacked plates from moist layers), mammatus under an anvil (`"mammatus"`), rain shafts and virga (`"precipitation"`: Marshall-Palmer extinction by rain rate, shear, evaporation), and cumulus grown by simulating a sunny day (`"harris_cumulus"`: stable fluids with buoyancy, condensation, latent heat and subgrid turbulence, through the ARM shallow-cumulus case's day; cached so `scripts/animate.py` can render it boiling and dissipating)
- Procedural trees (Weber and Penn's parametric model, `src/procedural/tree.rs`), forests of instanced
  trees (`src/procedural/forest.rs`), and rocks (`bedrock`, `boulder`, `pebbles` objects; `rock` material)
- Heightmap terrain (flat, fractal noise or image) and procedural grass that grows on it, with species and condition presets (see `src/procedural/grass.rs`)
- Spectral rendering (`"spectral": true`, `src/spectrum.rs`, after PBRT chapter 4): three hero wavelengths per path, RGB upsampled by Jakob-Hanika sigmoid polynomials (`src/spectrum/rgb2spec.rs`) via `Color::as_reflectance`/`as_illuminant`/`as_unbounded` where colours enter light transport (identity in RGB mode); dispersive glasses (`refractive_index`: named glass, Cauchy or Sellmeier) and blackbody emitters (`"temperature"`); the sky is evaluated at the sample's wavelengths (`SkyMaterial::air`, spectral data in `src/spectrum/atmosphere.rs`), so its outputs must not be upsampled again
- Cameras (PBRT ch. 5; `src/camera.rs`): thin lens (with astigmatism: eye-style cylinder or Seidel), orthographic, spherical, and realistic lens systems traced element by element (`src/lens.rs`, pbrt's lenses built in, polygonal blades); motion blur (`src/motion.rs`: `"shutter"`, `"motion"` on transforms and the camera; each sample's time is thread-local, like its wavelengths); Gaussian, Mitchell and Lanczos filters (signed weights); film (`src/film.rs`: ISO and exposure time, white balance, output colour spaces); lens flare (`src/flare.rs`: ghosts traced through coated lens surfaces from the image's brightest lights, and diffraction starbursts)
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
- Renders are resumable by default: the accumulated samples are saved every 30 s, and at the end, to `<output>.accum` (exact sums, keyed by a hash of the scene file and the size and seed overrides). Rerunning carries on where it stopped, or from a quick render adds samples up to the new target (`--samples 8`, later `--samples 64`); the result is identical to an uninterrupted render. Editing the scene starts afresh; after changing the renderer or files the scene refers to, use `--fresh`
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