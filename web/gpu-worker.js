// The GPU renderer's worker: the scene on the GPU (WebGPU), a few passes at a time, with each
// picture sent to the page as it is read back.
//
// It is a worker so that making the GPU's copy of a scene (parsing it, building its sky,
// flattening it: seconds for some scenes) doesn't hold the page up; the page's render workers
// are showing the scene on the CPU meanwhile, and stop when this one says it is ready.
//
// Messages in: `asset` (a file scenes name), `scene` (render this, as generation `gen`),
// `stop`. Out: `ready` (it has the scene, on which adapter), `frame` (RGBA pixels and how
// many passes they hold), `refused` (the GPU can't render this scene, and why: the CPU
// carries on), `error`. A scene replaces the one before it: the renderer is rebuilt for
// every edit and camera move.
import { instantiate, RaysError } from './rays.js';

const PASSES_PER_FRAME = 4;   // reading a picture back costs more than a pass
const MAX_PASSES = 4096;

let rays = null;
let gen = -1;
const assets = new Map();

// Let queued messages (a newer scene) be handled.
const pause = () => new Promise(resolve => setTimeout(resolve, 0));

async function fresh() {
    rays = await instantiate();
    for (const [path, bytes] of assets) { rays.addAsset(path, bytes); }
}

self.onmessage = async ({ data: m }) => {
    if (m.type === 'asset') {
        assets.set(m.path, m.bytes);
        if (rays && !rays.dead) { rays.addAsset(m.path, m.bytes); }
    } else if (m.type === 'stop') {
        gen = -1;
    } else if (m.type === 'scene') {
        gen = m.gen;
        // Scenes sent while this worker was busy are waiting behind this one: only the last
        // is worth making.
        await pause();
        if (gen !== m.gen) { return; }
        render(m.gen, m.json);
    }
};

async function render(mine, json) {
    let r = null;
    try {
        if (!rays || rays.dead) { await fresh(); }
        if (gen !== mine) { return; }
        try {
            r = await rays.gpuRenderer(json);
        } catch (e) {
            if (e instanceof RaysError) { throw e; }
            self.postMessage({ type: 'refused', gen: mine, message: e.message });
            return;
        }
        if (gen !== mine) { return; }
        const [width, height] = [r.width(), r.height()];
        self.postMessage({ type: 'ready', gen: mine, adapter: r.adapter(), width, height });
        while (gen === mine && r.passes() < MAX_PASSES) {
            // Together, where the wavefront is wide enough: the dispatches of one pass.
            for (let left = PASSES_PER_FRAME; left > 0;) { left -= r.advance(left); }
            // Awaiting the readback is also where a newer scene gets heard.
            const pixels = await r.image();
            if (gen !== mine) { return; }
            self.postMessage({ type: 'frame', gen: mine, passes: r.passes(), width, height, pixels }, [pixels.buffer]);
        }
    } catch (e) {
        self.postMessage({ type: 'error', gen: mine, message: e instanceof RaysError ? e.message : String(e.message ?? e) });
    } finally {
        // Not before: a renderer can't be freed while a picture is being read back from it.
        try { r?.free(); } catch (e) { /* the instance has stopped; it goes with it */ }
    }
}
