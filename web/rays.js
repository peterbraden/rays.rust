// The renderer's WebAssembly module (the crate in web/, bound for JavaScript by wasm-bindgen
// into web/www/pkg by scripts/build-web.sh), wrapped for the scene builder.
//
// Every instance has its own memory and at most one scene, and is used from one thread: each
// render worker has one, and the page has one for encoding, the schema and the GPU renderer.
//
// If the renderer panics (an invalid scene, say) the call throws a RaysError carrying the
// panic's message, and the instance is dead: make another with `instantiate`.

export class RaysError extends Error {}

const PKG = new URL('www/pkg/', import.meta.url);
let instances = 0;

export async function instantiate() {
    // wasm-bindgen's glue holds one instance per copy of its module, so a fresh instance is a
    // fresh copy of the glue: the query string makes it one. (The .wasm itself is cached.)
    let glue;
    try {
        glue = await import(new URL(`rays_web.js?instance=${instances++}`, PKG).href);
        await glue.default({ module_or_path: new URL('rays_web_bg.wasm', PKG) });
    } catch (e) {
        throw new Error(`Can't load the renderer (${e.message ?? e}). Build it with scripts/build-web.sh.`);
    }
    let dead = false;
    let preview = null;

    // Run a call into the module, turning a trap into an error with the panic's message. A
    // call that returns an `Err` throws its message as a string: the instance is still fine.
    function guard(f) {
        if (dead) { throw new RaysError('This renderer instance has stopped after an error.'); }
        try {
            return f();
        } catch (e) {
            if (typeof e === 'string') { throw new Error(e); }
            dead = true;
            let message = '';
            try { message = glue.last_error(); } catch (again) { /* too broken to say */ }
            throw new RaysError(cleanPanic(message) || String(e.message ?? e));
        }
    }

    return {
        get dead() { return dead; },

        /** What scene files may contain: the renderer's schema (see src/schema.rs). */
        schema() {
            return JSON.parse(guard(() => glue.schema()));
        },

        /** The world manifests of the scene's full-scale landscapes (see web/src/lib.rs). */
        worldManifests(json) {
            return guard(() => glue.world_manifests(json));
        },

        /** Files of tables the scene keeps in a `cache`, such as its sky's (see web/src/lib.rs). */
        cachedFiles(json) {
            return guard(() => glue.cached_files(json));
        },

        /** Give the contents of a file that scenes name by `path`. */
        addAsset(path, bytes) {
            guard(() => glue.provide(path, bytes));
        },

        /** Build a scene from scene-file JSON text, for the CPU path tracer. */
        loadScene(json) {
            return guard(() => {
                const next = new glue.Preview(json);
                preview?.free();
                preview = next;
                return { width: preview.width(), height: preview.height(), samplesPerLayer: preview.samples_per_layer() };
            });
        },

        /** Replace the camera, from the JSON text of a scene file's `camera`. */
        setCamera(json) {
            guard(() => preview.set_camera(json));
        },

        /** Render rows y0..y1 of a layer: linear sums, three floats per pixel, and rays cast. */
        renderRows(layer, y0, y1) {
            return guard(() => {
                const sums = preview.render_rows(layer, y0, y1);
                return { sums, rays: preview.rays_cast() };
            });
        },

        /**
         * Rows y0..y1 of a quick image from the realtime raycaster, cols by rows pixels: three
         * floats per pixel. `shading` names how it shades (see src/raycaster.rs; '' for the
         * default). An unknown name throws, without stopping the instance.
         */
        raycast(shading, cols, rows, y0, y1) {
            return guard(() => preview.raycast(shading ?? '', cols, rows, y0, y1));
        },

        /** The index in the scene's `objects` of what is seen at a pixel, or -1. */
        pick(px, py) {
            return guard(() => preview.pick(px, py));
        },

        /** Encode summed samples as RGBA bytes for display. `output` is the scene's `output`. */
        encode(sums, samples, output) {
            return guard(() => new Uint8ClampedArray(glue.encode(sums, samples, JSON.stringify(output ?? null)).buffer));
        },

        /**
         * A planet from a seed, built to time zero: its plates, climate and landscape before
         * erosion (see `PlanetBuilder` in web/src/lib.rs). Slow (seconds to a minute), and
         * nothing else runs meanwhile: call it from a worker. `progress(stage, fraction,
         * message)` hears how the stages go. The planet then advances by whole steps of its
         * simulation (`advance(years, onStep)`) and draws maps (`map(view, width)`: RGBA,
         * width by width / 2).
         */
        planet(seed, cells, step, progress) {
            const p = guard(() => new glue.PlanetBuilder(seed, cells, step, progress));
            return {
                time: () => guard(() => p.time()),
                step: () => guard(() => p.step()),
                advance: (years, onStep) => guard(() => p.advance(years, onStep ?? (() => true))),
                map: (view, width) => guard(() => p.map(view, width)),
                stats: () => JSON.parse(guard(() => p.stats())),
                free: () => p.free(),
            };
        },

        /**
         * The scene on the GPU (WebGPU), if this browser has one and the scene can be
         * flattened for it; throws with the reason otherwise. See `Renderer` in web/src/lib.rs:
         * `pass()` queues a sample of every pixel (`advance(n)` up to n of them together), `await image()` reads the picture back.
         */
        async gpuRenderer(json) {
            if (dead) { throw new RaysError('This renderer instance has stopped after an error.'); }
            if (!navigator.gpu) { throw new Error('this browser has no WebGPU'); }
            try {
                return await glue.Renderer.create(json, 0, 0);
            } catch (e) {
                // A refusal (can't flatten, no adapter) is a string; a panic is a trap.
                if (typeof e === 'string') {
                    // wgpu lists every backend it wasn't built with; the first clause says it.
                    throw new Error(e.startsWith('no GPU adapter') ? 'the browser offered no GPU adapter' : e);
                }
                dead = true;
                let message = '';
                try { message = glue.last_error(); } catch (again) { /* too broken to say */ }
                throw new RaysError(cleanPanic(message) || String(e.message ?? e));
            }
        },
    };
}

// "panicked at src/scenefile.rs:12:3:\nmessage" -> "message"
function cleanPanic(text) {
    if (!text) { return text; }
    const m = text.match(/^panicked at [^\n]*:\n([\s\S]*)$/);
    return (m ? m[1] : text).trim();
}
