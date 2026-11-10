# CLAUDE.md - Rays.rust Project Guide

## Project Overview
Rays.rust is a raytracer written in Rust with the following features:
- Whitted tracing and path tracing with Monte-Carlo global illumination
- Material models: Lambertian, Specular, Dielectric
- Objects: Sphere, Plane, Mesh, OBJ file import; transforms and instancing of named "geometries"
- Skysphere with Rayleigh and Mie Scattering
- Procedural objects including Ocean (Tessendorf's algorithm with Phillips spectrum)
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

## Command-line Options
- `<FILE>`: Scene file (required, positional). The image is written alongside it with a `.png` extension
- `-o, --output <FILE>`: Write the image here instead; `.png` (8-bit, display-encoded) or `.exr` (linear float)
- `-p, --progressive-render`: Update the output file during the render (at most once a second)
- `--width <NUM>`: Override scene file width
- `--height <NUM>`: Override scene file height
- `--seed <NUM>`: Override the scene's render seed; the same seed gives the same image
- `--compare <FILE>`: Compare with a reference image (eg. `references/cornell-box/TungstenRender.exr`, fetched by `scripts/fetch-references.sh`): prints exposure fit, RMSE and mean FLIP error; writes `<output>.flip.png` and `<output>.reference.png`
- `--max-flip <NUM>`: With `--compare`, exit with status 2 if the mean FLIP error exceeds this

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
- Scene definitions stored in JSON files (in demo/ and demo/scenes/)
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