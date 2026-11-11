// A render worker: its own copy of the scene, rendering the layers the page assigns it.
//
// The page starts several of these. Worker `index` renders layers index, index + stride,
// index + 2 stride, ... so no two workers repeat a layer (nor do render servers, which take
// the layer numbers between: see app.js), and what they send adds up to the same image
// whoever rendered what. Each layer is sent in bands of
// rows as they finish. A worker starts each layer at a different row, so the first pass
// covers the image `count` times sooner.
//
// While the camera is being moved the page asks for drafts instead (one unbounced ray per
// pixel, at reduced size), which are quick enough to follow the pointer; layers start again
// from the camera message that ends the move.
//
// Every message carries the generation (`gen`) of the scene or camera it belongs to; the
// page drops bands from an older generation, and a worker abandons a layer as soon as a newer
// one arrives. It can only notice between bands, so bands are kept to about BAND_MS.
import { instantiate, RaysError } from './rays.js';

const BAND_MS = 40;
const MAX_LAYER = 4096 * 16;

let rays = null;
let index = 0;
let count = 1;
let stride = 1;        // layers this worker renders are index, index + stride, ...
let gen = -1;          // generation being rendered
let size = null;       // { width, height } of the built scene
let running = false;
let drafting = false;  // showing drafts while the camera is moved: no layers meanwhile
const assets = new Map();

// Let queued messages (a newer scene, a camera move) be handled.
const pause = () => new Promise(resolve => setTimeout(resolve, 0));

async function fresh() {
    rays = await instantiate();
    for (const [path, bytes] of assets) { rays.addAsset(path, bytes); }
}

self.onmessage = async ({ data: m }) => {
    try {
        if (m.type === 'init') {
            index = m.index; count = m.count; stride = m.stride ?? m.count;
        } else if (m.type === 'asset') {
            assets.set(m.path, m.bytes);
            if (rays && !rays.dead) { rays.addAsset(m.path, m.bytes); }
        } else if (m.type === 'scene') {
            gen = m.gen;
            size = null;
            drafting = false;
            // Scenes sent while this worker was busy are all waiting behind this one. Let
            // them be heard first: each sets `gen`, and only the last is worth building.
            await pause();
            if (gen !== m.gen) { return; }
            // An instance that has panicked can't be reused. One that hasn't is kept, as it
            // remembers what was slow to build (the sky's lookup table).
            if (!rays || rays.dead) { await fresh(); }
            if (gen !== m.gen) { return; }
            const t0 = performance.now();
            size = rays.loadScene(m.json);
            self.postMessage({ type: 'built', gen: m.gen, ms: performance.now() - t0, ...size });
            run();
        } else if (m.type === 'camera') {
            if (!size) { return; }   // the scene failed to build, or is still coming
            gen = m.gen;
            drafting = false;
            rays.setCamera(m.json);
            run();
        } else if (m.type === 'draft') {
            // One quick frame from this camera: this worker's share of its rows.
            if (!size) { return; }
            gen = m.gen;
            drafting = true;
            rays.setCamera(m.json);
            const y0 = Math.floor((m.rows * index) / count);
            const y1 = Math.floor((m.rows * (index + 1)) / count);
            const values = rays.renderDraft(m.cols, m.rows, y0, y1);
            self.postMessage({ type: 'draft', gen: m.gen, y0, y1, values }, [values.buffer]);
        } else if (m.type === 'pick') {
            self.postMessage({ type: 'picked', id: m.id, index: size ? rays.pick(m.x, m.y) : -1 });
        } else if (m.type === 'stop') {
            gen = -1;
        }
    } catch (e) {
        size = null;
        self.postMessage({ type: 'error', gen: m.gen, message: e instanceof RaysError ? e.message : String(e) });
    }
};

async function run() {
    if (running) { return; }   // the loop below picks up the new generation itself
    running = true;
    try {
        while (gen >= 0 && size && !drafting) {
            const mine = gen;
            const { width, height } = size;
            let rows = Math.max(1, Math.round(height / 16));
            for (let layer = index; gen === mine && layer < MAX_LAYER; layer += stride) {
                const start = Math.floor((height * index) / count);
                for (let done = 0; done < height && gen === mine;) {
                    const y0 = (start + done) % height;
                    const y1 = Math.min(height, y0 + rows);
                    const t0 = performance.now();
                    const { sums, rays: cast } = rays.renderRows(layer, y0, y1, width);
                    const ms = performance.now() - t0;
                    self.postMessage({ type: 'band', gen: mine, layer, y0, y1, sums, rays: cast, ms }, [sums.buffer]);
                    done += y1 - y0;
                    // Aim each band at BAND_MS, changing by at most a factor of two at a time.
                    rows = Math.max(1, Math.round(rows * Math.min(2, Math.max(0.5, BAND_MS / Math.max(ms, 1)))));
                    await pause();
                }
            }
            if (gen === mine) { break; }
        }
    } catch (e) {
        size = null;
        self.postMessage({ type: 'error', gen, message: e instanceof RaysError ? e.message : String(e) });
    } finally {
        running = false;
    }
}
