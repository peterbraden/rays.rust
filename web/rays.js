// The renderer's WebAssembly module (web/wasm), wrapped for JavaScript.
//
// Every instance has its own memory and at most one scene, and is used from one thread: each
// render worker has one, and the page has one for encoding and for picking.
//
// If the renderer panics (an invalid scene, say) the call throws a RaysError carrying the
// panic's message, and the instance is dead: make another with `instantiate`.

export class RaysError extends Error {}

let compiled = null;

function compile(url) {
    if (!compiled) {
        compiled = (async () => {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`Can't load ${url} (${response.status}). Build it with web/build.sh.`);
            }
            return WebAssembly.compile(await response.arrayBuffer());
        })();
    }
    return compiled;
}

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export async function instantiate(url = new URL('rays.wasm', import.meta.url)) {
    const module = await compile(url);
    let memory = null;
    let lastError = null;
    let dead = false;
    const messages = [];

    const instance = await WebAssembly.instantiate(module, {
        env: {
            rays_log(ptr, len, level) {
                const text = decoder.decode(new Uint8Array(memory.buffer, ptr, len));
                if (level === 1) { lastError = text; } else { messages.push(text); }
            },
        },
    });
    const x = instance.exports;
    memory = x.memory;
    x.init();

    // Run a call into the module, turning a trap into an error with the panic's message.
    function guard(f) {
        if (dead) { throw new RaysError('This renderer instance has stopped after an error.'); }
        lastError = null;
        try {
            return f();
        } catch (e) {
            dead = true;
            throw new RaysError(cleanPanic(lastError) || String(e));
        }
    }

    // Copy bytes into the module's memory for the length of one call.
    function withBytes(bytes, f) {
        const ptr = x.alloc(bytes.length);
        new Uint8Array(memory.buffer, ptr, bytes.length).set(bytes);
        try { return f(ptr, bytes.length); } finally { if (!dead) { x.dealloc(ptr, bytes.length); } }
    }
    const withText = (text, f) => withBytes(encoder.encode(text), f);

    return {
        get dead() { return dead; },

        /** Give the contents of a file that scenes name by `path`. */
        addAsset(path, bytes) {
            guard(() => withText(path, (p, pl) => withBytes(bytes, (d, dl) => x.asset_add(p, pl, d, dl))));
        },

        /** Build a scene from scene-file JSON text. */
        loadScene(json) {
            guard(() => withText(json, (p, l) => x.scene_load(p, l)));
            return { width: x.scene_width(), height: x.scene_height(), samplesPerLayer: x.scene_samples_per_layer() };
        },

        /** Replace the camera, from the JSON text of a scene file's `camera`. */
        setCamera(json) {
            guard(() => withText(json, (p, l) => x.scene_set_camera(p, l)));
        },

        /** Render rows y0..y1 of a layer: linear sums, three floats per pixel, and rays cast. */
        renderRows(layer, y0, y1, width) {
            return guard(() => {
                const n = (y1 - y0) * width * 3;
                const ptr = x.alloc(n * 4);
                const rays = x.render_rows(layer, y0, y1, ptr);
                const sums = new Float32Array(memory.buffer, ptr, n).slice();
                x.dealloc(ptr, n * 4);
                return { sums, rays };
            });
        },

        /** Rows y0..y1 of a quick draft image, cols by rows pixels: three floats per pixel. */
        renderDraft(cols, rows, y0, y1) {
            return guard(() => {
                const n = (y1 - y0) * cols * 3;
                const ptr = x.alloc(n * 4);
                x.render_draft(cols, rows, y0, y1, ptr);
                const values = new Float32Array(memory.buffer, ptr, n).slice();
                x.dealloc(ptr, n * 4);
                return values;
            });
        },

        /** The index in the scene's `objects` of what is seen at a pixel, or -1. */
        pick(px, py) {
            return guard(() => x.pick(px, py));
        },

        /** Encode summed samples as RGBA bytes for display. `output` is the scene's `output`. */
        encode(sums, samples, output) {
            return guard(() => {
                const pixels = sums.length / 3;
                const inPtr = x.alloc(sums.length * 4);
                const outPtr = x.alloc(pixels * 4);
                new Float32Array(memory.buffer, inPtr, sums.length).set(sums);
                withText(JSON.stringify(output ?? null), (p, l) => x.encode(inPtr, pixels, samples, p, l, outPtr));
                const rgba = new Uint8ClampedArray(memory.buffer, outPtr, pixels * 4).slice();
                x.dealloc(inPtr, sums.length * 4);
                x.dealloc(outPtr, pixels * 4);
                return rgba;
            });
        },
    };
}

// "panicked at src/scenefile.rs:12:3:\nmessage" -> "message"
function cleanPanic(text) {
    if (!text) { return text; }
    const m = text.match(/^panicked at [^\n]*:\n([\s\S]*)$/);
    return (m ? m[1] : text).trim();
}
