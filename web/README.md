# Scene builder (prototype)

A browser editor for scene files, with a live preview from the real path tracer compiled to
WebAssembly.

    rustup target add wasm32-unknown-unknown   # once
    web/build.sh                               # writes web/rays.wasm
    python3 -m http.server 8000                # from the repository root
    # open http://localhost:8000/web/

Serve the repository root, not `web/`: scenes name their meshes by paths from the root
(`demo/teapot.obj`), and the page fetches them from there.

## What it does

- Outline of the scene file (settings, camera, sun, output, materials, media, lights, objects)
  with a property editor driven by the renderer's schema (`src/schema.json`): every field a
  type accepts is shown, unset fields with their defaults dimmed; editing one writes it, and
  its reset button removes it again. Nested objects and shapes (a transform's item, a CSG's
  parts) are edited in place. Keys the schema doesn't know are listed after.
- The Add menu offers every type in the schema, by group, as its example. A type that needs a
  sun or a sky to be seen (clouds) brings what the scene lacks.
- The scene file as text in the sidebar's other tab, editable, kept in step both ways.
- Preview: drag to orbit, shift-drag to pan, scroll to zoom, click an object to select it.
  While the camera moves the preview shows quick drafts (one unbounced ray per pixel, at half
  size); path tracing starts again when it stops.
- The preview keeps adding samples, up to 4096 per pixel from each worker's share.
- Loads the repository's demo scenes. "Save scene" downloads the edited scene as JSON, which
  the command line renders unchanged; "Save image" downloads the preview as it stands, as PNG.

## Render servers

`rays serve` (see `src/server.rs`) hosts this page and renders for it:

    cargo run --release -- serve            # then open http://localhost:8787/web/

A page looks for servers when it loads: the one it came from, and any named with
`?servers=http://host:8787,...` or through the "Servers…" button (kept in the browser). Each
server builds the same scene and renders whole layers on all its cores; the page adds them to
its own workers' layers. Workers and servers take different layer numbers, and every (layer,
pixel) has its own random stream, so the picture is the same whoever renders what. With no
server the page renders alone.

To use other machines, start `rays serve --bind 0.0.0.0` on each, in a checkout with the same
asset files (a server reads meshes and images from its own disk), and name them to the page.
A page served from somewhere else must be allowed by each server: `--allow-origin
https://user.github.io`. Browsers only let an https page call an http server on localhost,
so remote nodes need the page served over http (from a `rays serve`), or https in front of
them. There is no authentication: use `--bind` only on a network you trust.

## Hosting

`.github/workflows/pages.yml` builds the page and deploys it to GitHub Pages on every push to
master (`web/site.sh` assembles the site: `web/` and the demo scenes and meshes). Enable it
once under Settings > Pages > Source: "GitHub Actions".

## The schema

`src/schema.json` lists every object, material and medium type with its fields (kind,
default or required, a line of description), an example, and what it needs to be seen; and
the fields of the scene file's other parts. It is written by hand from the parser, and
`src/schema.rs` tests it against the parser: a type added to the parser without an entry,
or a default the parser rejects, fails `cargo test`. A field's `kind` is one of number,
integer, boolean, string, path, vec2, vec3, color, enum (with `options`), material, medium
(names), shape, shapes (nested geometry), object and list (with nested `fields`), or json.

## How it works

- `wasm/` wraps `rays::preview` as a WebAssembly module with plain exports (no binding
  generator). `rays.js` is its JavaScript side.
- `worker.js`: each web worker builds its own copy of the scene and renders different layers
  (one sample per pixel each), in bands of rows.
- `app.js` adds the bands up, encodes them with the scene's `output` settings, and draws.
- A camera move replaces only the camera. Any other edit rebuilds the scene in every worker,
  but only what changed: built objects are kept and reused when their description, and the
  materials, media and geometries they refer to, are the same; so are the sky's tables.
- A worker can't be interrupted while it builds. Edits made meanwhile wait, and only the last
  is built; a worker that has been building a superseded scene for over 1.5 s is replaced,
  unless it is on its first build (which is mostly filling the caches the next would need).

## Known limits

- The preview is the CPU renderer, on one thread per worker: complete, but slow for heavy
  scenes (grass, clouds, large oceans). There is no GPU path yet.
- Each worker holds its own copy of the scene in memory, and builds it itself. The first
  build of a scene with a sky takes about 8 s in the browser.
- An invalid scene panics the renderer; the page shows the message and starts a new instance.
  Messages are the parser's, with no pointer to the offending field.
- Lists of objects (a blob's components, a forest's species) and transform steps are edited
  as JSON. Presets change defaults (a tree's species, a rock's style); the schema shows the
  default preset's.
- A file a scene names must be under the served directory: a `text` shape's font given by a
  system path can't be read by the browser (a render server can read it).
- A server on the same machine as the browser competes with it for the same cores.
