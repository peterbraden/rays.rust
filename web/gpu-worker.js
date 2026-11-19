// The GPU renderer's worker: the scene on the GPU (WebGPU), a few passes at a time, with each
// picture sent to the page as it is read back.
//
// It is a worker so that making the GPU's copy of a scene (parsing it, building its sky,
// flattening it: seconds for some scenes) doesn't hold the page up; the page's render workers
// are showing the scene on the CPU meanwhile, and stop when this one says it is ready.
//
// Messages in: `asset` (a file scenes name), `scene` (render this, as generation `gen`),
// `camera` (the scene it has, from another camera: nothing is made again, the picture starts
// again; it carries the scene too, for a worker that has none), `stop`. Out: `ready` (it has the scene, on which adapter), `frame` (RGBA pixels and how
// many passes they hold), `refused` (the GPU can't render this scene, and why: the CPU
// carries on), `error`. A scene replaces the one before it: the renderer is rebuilt for
// every edit, and kept for a camera move.
import { instantiate, RaysError } from './rays.js';

const PASSES_PER_FRAME = 4;   // reading a picture back costs more than a pass
const MAX_PASSES = 4096;

let rays = null;
const assets = new Map();
let renderer = null;     // the scene on the GPU, kept between pictures of it
let refusal = null;      // why the GPU refused the last scene, if it did
let want = null;         // the newest thing asked for: { kind, gen, json, camera }
let busy = false;

async function fresh() {
    rays = await instantiate();
    for (const [path, bytes] of assets) { rays.addAsset(path, bytes); }
}

function release() {
    // Only between pictures: a renderer can't be freed while one is being read back from it.
    try { renderer?.free(); } catch (e) { /* the instance has stopped; it goes with it */ }
    renderer = null;
}

self.onmessage = ({ data: m }) => {
    if (m.type === 'asset') {
        assets.set(m.path, m.bytes);
        if (rays && !rays.dead) { rays.addAsset(m.path, m.bytes); }
        return;
    }
    // A camera for a scene still waiting to be made is that scene, from that camera: the
    // scene it carries is the newer text.
    const scene = m.type === 'scene' || (m.type === 'camera' && want?.kind === 'scene');
    want = m.type === 'stop' ? { kind: 'stop' } : { kind: scene ? 'scene' : 'camera', gen: m.gen, json: m.json ?? m.scene, camera: m.camera };
    if (!busy) { work(); }
};

// One thing at a time, and always the newest asked for: requests that arrived while a scene
// was being made or a picture read back are behind it, and only the last is worth doing.
async function work() {
    busy = true;
    // Let queued messages be handled first.
    await new Promise(resolve => setTimeout(resolve, 0));
    while (want) {
        const w = want;
        want = null;
        if (w.kind === 'stop') { continue; }
        try {
            if (w.kind === 'camera' && renderer) {
                renderer.set_camera(w.camera);
            } else if (w.kind === 'camera' && refusal) {
                // The same scene the GPU couldn't take before.
                self.postMessage({ type: 'refused', gen: w.gen, message: refusal });
                continue;
            } else {
                release();
                refusal = null;
                if (!rays || rays.dead) { await fresh(); }
                try {
                    renderer = await rays.gpuRenderer(w.json);
                } catch (e) {
                    if (e instanceof RaysError) { throw e; }
                    refusal = e.message;
                    self.postMessage({ type: 'refused', gen: w.gen, message: e.message });
                    continue;
                }
                if (want) { continue; }
            }
            await pictures(w.gen, renderer);
        } catch (e) {
            release();
            self.postMessage({ type: 'error', gen: w.gen, message: e instanceof RaysError ? e.message : String(e.message ?? e) });
        }
    }
    busy = false;
}

// Pictures of the scene from its camera, until something else is asked for.
async function pictures(mine, r) {
    const [width, height] = [r.width(), r.height()];
    self.postMessage({ type: 'ready', gen: mine, adapter: r.adapter(), width, height });
    while (!want && r.passes() < MAX_PASSES) {
        // The first picture after one sample, so a new view shows as soon as it can; then a
        // few together, where the wavefront is wide enough: the dispatches of one pass.
        for (let left = r.passes() === 0 ? 1 : PASSES_PER_FRAME; left > 0;) { left -= r.advance(left); }
        // Awaiting the readback is also where a newer request gets heard.
        const pixels = await r.image();
        if (want) { return; }
        self.postMessage({ type: 'frame', gen: mine, passes: r.passes(), width, height, pixels }, [pixels.buffer]);
    }
}
