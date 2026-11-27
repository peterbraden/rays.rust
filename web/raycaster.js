// The realtime raycaster's frames, for the scene builder (app.js): while the camera is being
// moved, quick frames (one unbounced ray per pixel, at a fraction of the preview's size: see
// src/raycaster.rs) stand in for the path-traced picture.
//
// Each render worker already holds the built scene, so a frame is shared out among them by
// rows (worker.js, the 'raycast' message) and put together here. One frame is in flight at a
// time: a request made meanwhile is remembered and sent when that frame is done, so frames
// never queue up behind the pointer.
//
// What this module doesn't decide is the page's: when to raycast and when to go back to path
// tracing, the generation a frame belongs to, and encoding it for display (the scene's
// `output`). `draw` takes the encoded frame and scales it up onto the preview.
//
// How frames are shaded is the renderer's `Shading`, by name; the page takes it from
// `?shading=` on its URL. A new way of shading is added in src/raycaster.rs alone.

export class Raycaster {
    /**
     * `scale`: a frame's size as a fraction of the preview's. `shading`: a name from
     * `Shading::NAMES` in src/raycaster.rs, or '' for the default.
     */
    constructor({ scale = 0.5, shading = '' } = {}) {
        this.scale = scale;
        this.shading = shading;
        this.frame = null;      // the frame being gathered from the workers
        this.queued = false;    // asked for again while that one was being made
        this.canvas = document.createElement('canvas');
    }

    /** A frame is being gathered. */
    get busy() { return Boolean(this.frame); }

    /** Remember to make another frame once the one in flight is done (see `takeQueued`). */
    queue() { this.queued = true; }

    /**
     * Ask `workers` for a frame of generation `gen` from `camera` (the scene file's `camera`)
     * for a preview `width` by `height`. Not while `busy`: `queue` then.
     */
    request(gen, workers, camera, width, height) {
        if (this.frame) { throw new Error('a raycast frame is already in flight'); }
        this.queued = false;
        const cols = Math.max(8, Math.round(width * this.scale));
        const rows = Math.max(8, Math.round(height * this.scale));
        this.frame = { gen, cols, rows, values: new Float32Array(cols * rows * 3), got: 0 };
        const json = JSON.stringify(camera);
        for (const w of workers) { w.postMessage({ type: 'raycast', gen, json, cols, rows, shading: this.shading }); }
    }

    /**
     * A worker's 'raycast' message. Returns the frame once all its rows are in (and is then
     * free for the next), or null.
     */
    receive(m) {
        const f = this.frame;
        if (!f || m.gen !== f.gen) { return null; }
        f.values.set(m.values, m.y0 * f.cols * 3);
        f.got += m.y1 - m.y0;
        if (f.got < f.rows) { return null; }
        this.frame = null;
        return f;
    }

    /** Whether a request came while the last frame was being made: take it (and forget it). */
    takeQueued() {
        const q = this.queued;
        this.queued = false;
        return q;
    }

    /** Forget the frame in flight and any request waiting. */
    cancel() {
        this.frame = null;
        this.queued = false;
    }

    /** Draw a frame's encoded pixels (RGBA, `cols` by `rows`) scaled up to fill `ctx`'s `width` by `height`. */
    draw(ctx, rgba, { cols, rows }, width, height) {
        this.canvas.width = cols; this.canvas.height = rows;
        this.canvas.getContext('2d').putImageData(new ImageData(rgba, cols, rows), 0, 0);
        ctx.imageSmoothingEnabled = true;
        ctx.drawImage(this.canvas, 0, 0, width, height);
    }
}
