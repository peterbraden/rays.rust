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

- Outline of the scene file (settings, camera, materials, media, lights, objects) with a
  property editor generated from each entry's fields; add, duplicate and delete.
- The scene file as text in the sidebar's other tab, editable, kept in step both ways.
- Preview: drag to orbit, shift-drag to pan, scroll to zoom, click an object to select it.
  While the camera moves the preview shows quick drafts (one unbounced ray per pixel, at half
  size); path tracing starts again when it stops.
- The Add menu includes a sun and clouds (cirrus, cumulus). Clouds are lit by the scene's
  `"sun"` and seen against a skysphere, so adding clouds adds whichever of those is missing.
- The preview keeps adding samples, up to 4096 per pixel.
- Loads the repository's demo scenes. "Save scene" downloads the edited scene as JSON, which
  the command line renders unchanged; "Save image" downloads the preview as it stands, as PNG.

## How it works

- `wasm/` wraps `rays::preview` as a WebAssembly module with plain exports (no binding
  generator). `rays.js` is its JavaScript side.
- `worker.js`: each web worker builds its own copy of the scene and renders different layers
  (one sample per pixel each), in bands of rows. Every (layer, pixel) has its own random
  stream, so the layers add up to the image the command line gives, whatever the number of
  workers. `?workers=N` sets the number.
- `app.js` adds the bands up, encodes them with the scene's `output` settings, and draws.
- A camera move replaces only the camera. During one, each worker renders its share of the
  rows of a draft (`trace::draft`), one draft at a time. Any other edit rebuilds the scene in every worker;
  the sky's lookup table is kept between rebuilds.

## Known limits

- The preview is the CPU renderer, on one thread per worker: complete, but slow for heavy
  scenes (grass, large oceans). There is no GPU path yet.
- Every non-camera edit rebuilds the whole scene, and each worker holds its own copy in
  memory.
- An invalid scene panics the renderer; the page shows the message and starts a new instance.
  Messages are the parser's, with no pointer to the offending field.
- The property editor only shows fields already present in the file; it has no schema, so it
  can't offer a type's other options.
