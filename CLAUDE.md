# CLAUDE.md - Rays.rust Project Guide

## Project Overview
Rays.rust is a raytracer written in Rust with the following features:
- Whitted tracing and path tracing with Monte-Carlo global illumination
- Material models: Lambertian, Specular, Dielectric
- Objects: Sphere, Plane, Mesh, OBJ file import
- Skysphere with Rayleigh and Mie Scattering
- Procedural objects including Ocean (Tessendorf's algorithm with Phillips spectrum)
- Multithreaded rendering with progressive output

## Build Commands
- Build and run: `cargo run --release -- demo/demo.json`
- Run tests: `cargo test`
- Run specific test: `cargo test <test_name>`
- Lint code: `cargo clippy`
- Format code: `cargo fmt`
- View sample scenes: `cargo run --release -- demo/scenes/<scene_file>.json`

## Command-line Options
- `<FILE>`: Scene file (required, positional): our JSON, or `.nff` (translated by `src/nff.rs`). The image is written alongside it with a `.png` extension
- `--dump-scene <FILE>`: Write the scene as our JSON (for an `.nff` file, its translation)
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