# CLAUDE.md - Rays.rust Project Guide

## Project Overview
Rays.rust is a raytracer written in Rust with the following features:
- Whitted tracing and path tracing with Monte-Carlo global illumination
- Material models: Lambertian, Specular, Dielectric, Plastic, Leaf (translucent, `src/material/leaf.rs`); bump mapping via an object's "normal" (src/material/bump.rs, docs/bump-mapping.md)
- Objects: Sphere, Plane, Mesh, OBJ file import; transforms and instancing of named "geometries"; POV-Ray's shapes (cylinder, cone, torus, blob, isosurface, text and more; see docs/shapes.md, parsed in src/scenefile_shapes.rs), combined by CSG (src/shapes/csg.rs); mountains (`"mountains"`: ridged-noise heightfield with snow, rock and vegetation), and a planet (`"planet"`: the ground as a sphere of the Earth's radius); clouds can cover the whole planet (`"mapping": "global"`: an equi-angular cube-sphere weather map and a planet-wide wind field), seen from orbit in `demo/scenes/orbit.json` (and the bare atmosphere at the limb and from the stratosphere: `limb-sunset.json`, `stratosphere.json`)
- Participating media: boxes of constant or noise-driven density (delta tracking with local majorants, ratio-tracked shadows); Henyey-Greenstein or Mie (water droplet) phase; diffraction by particles (coronae, iridescence); ice-crystal halos traced through hexagonal prisms (sun dogs, 22°/46° halos, circumzenithal arc); rainbows traced through raindrops (`"rainbow": true`: primary and secondary bows, Alexander's dark band); optional similarity and Oz multiple-scattering approximations
- Skysphere with Rayleigh and Mie Scattering, and aerial perspective (distant objects attenuated and hazed by the same atmosphere); `"crepuscular_rays": true` lets clouds and terrain shadow the air (sunbeams)
- Directional sun light (`"sun"`), sampled directly; follows the skysphere's sun and is reddened by its atmosphere
- Procedural objects including Ocean (Tessendorf's algorithm with Phillips spectrum; `"curved": true` makes it the planet's surface, displaced near the camera, physically lit, with sun glitter and slopes calibrated to Cox and Munk) and volumetric clouds: cumulus to stratus (`"nubis_clouds"`, after Horizon Zero Dawn's Nubis) and cirrus with fallstreaks (`"hogan_kew_cirrus"`, after Hogan and Kew), altocumulus as convection cells (`"benard_altocumulus"`: Rayleigh-Bénard cells, rows, undulatus waves), lenticular clouds over a mountain's lee waves (`"lenticular"`: stacked plates from moist layers), mammatus under an anvil (`"mammatus"`), rain shafts and virga (`"precipitation"`: Marshall-Palmer extinction by rain rate, shear, evaporation), and cumulus grown by simulating a sunny day (`"harris_cumulus"`: stable fluids with buoyancy, condensation, latent heat and subgrid turbulence, through the ARM shallow-cumulus case's day; cached so `scripts/animate.py` can render it boiling and dissipating)
- Procedural trees (Weber and Penn's parametric model, `src/procedural/tree.rs`), forests of instanced
  trees (`src/procedural/forest.rs`), and rocks (`bedrock`, `boulder`, `pebbles` objects; `rock` material)
- Heightmap terrain (flat, fractal noise or image) and procedural grass that grows on it, with species and condition presets (see `src/procedural/grass.rs`)
- Multithreaded rendering with progressive output

## Build Commands
- Build and run: `cargo run --release -- demo/demo.json`
- Run tests: `cargo test`
- Run specific test: `cargo test <test_name>`
- Lint code: `cargo clippy`
- Format code: `cargo fmt`
- View sample scenes: `cargo run --release -- demo/scenes/<scene_file>.json`

## Browser Scene Builder (prototype, `web/`)
- Build the renderer for the browser: `web/build.sh` (needs `rustup target add wasm32-unknown-unknown` once); writes `web/rays.wasm`
- Serve the repository root and open `/web/`: `python3 -m http.server 8000`, then http://localhost:8000/web/
- The renderer is the `rays` library; the command line is its default `cli` feature. `web/wasm` builds the library without it. Code the library needs must build for `wasm32-unknown-unknown`: read files through `crate::assets`, and don't use `std::time::Instant` or threads outside rayon
- `src/preview.rs` is the interface the page drives (build from JSON, render rows of a layer, set camera, pick)
- `src/schema.json` lists every scene-file type and field for the editor; `src/schema.rs` tests it against the parser. When the parser gains a type or a field, or a default changes, update the schema (a missing type fails `cargo test schema`)
- `cargo run --release -- serve` hosts the builder at http://localhost:8787/web/ and renders layers for it (`src/server.rs`; `--port`, `--bind`, `--root`, `--allow-origin`)
- `.github/workflows/pages.yml` deploys the builder to GitHub Pages from master (`web/site.sh` assembles the site)

## Command-line Options
- `<FILE>`: Scene file (required, positional): our JSON, `.nff` (translated by `src/nff.rs`), or Tungsten's JSON (recognised by its `bsdfs` and `primitives`; translated by `src/tungsten.rs`). The image is written alongside it with a `.png` extension
- `--dump-scene <FILE>`: Write the scene as our JSON (for an NFF or Tungsten file, its translation)
- `-o, --output <FILE>`: Write the image here instead; `.png` (8-bit, display-encoded) or `.exr` (linear float)
- `-p, --progressive-render`: Update the output file during the render (at most once a second)
- `--width <NUM>`: Override scene file width
- `--height <NUM>`: Override scene file height
- `--seed <NUM>`: Override the scene's render seed; the same seed gives the same image
- `--compare <FILE>`: Compare with a reference image (eg. `references/cornell-box/TungstenRender.exr`, fetched by `scripts/fetch-references.sh`): prints exposure fit, RMSE and mean FLIP error; writes `<output>.flip.png` and `<output>.reference.png`
- `--max-flip <NUM>`: With `--compare`, exit with status 2 if the mean FLIP error exceeds this
- `--depth <FILE>`: Also write a depth image (distance to the nearest surface through each pixel centre); `.exr` keeps distances, `.png` scales them for viewing

## Code Style
- Traits used for major abstractions (Camera, Geometry, Medium, MaterialModel)
- Box<dyn Trait> for polymorphism
- Error handling via Option<T>/Result<T,E> with unwrap()
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