// The scene builder: an outline and property editor over a scene file, with a live preview.
//
// The scene file (the same JSON the command line renders) is the document being edited; `doc`
// holds it parsed. Every edit, from the property fields, the camera controls or the JSON text,
// changes `doc` and restarts the preview. The preview is the real path tracer, compiled to
// WebAssembly and run in web workers (see worker.js): each holds its own copy of the scene and
// renders different layers, which are added up here and drawn as they arrive.
import { instantiate } from './rays.js';

const $ = id => document.getElementById(id);
const canvas = $('canvas');
const ctx = canvas.getContext('2d');

// ---------------------------------------------------------------------------------------
// The document

const STARTER = {
    width: 640, height: 400,
    supersamples: 64, samples_per_chunk: 1, max_depth: 6,
    shadow_bias: 0.0001,
    background: [0, 0, 0],
    // Sunlight from the sky's sun: what lights clouds, and casts the shadows.
    sun: { intensity: 3 },
    output: { tonemap: 'reinhard' },
    camera: { location: [0, 3, 11], lookat: [0, 1, 0], up: [0, 1, 0], angle: 0.5, aperture: 0 },
    materials: {
        WHITE: { type: 'lambertian', albedo: [0.85, 0.85, 0.85] },
        DARK: { type: 'lambertian', albedo: [0.15, 0.15, 0.17] },
        RED: { type: 'lambertian', albedo: [0.8, 0.15, 0.1] },
        GOLD: { type: 'metal', reflective: [1, 0.85, 0.57], roughness: 0.1 },
        GLASS: { type: 'dielectric', refractive_index: 1.5, attenuate: [1, 1, 1] },
    },
    media: {
        TILES: { type: 'checkered-y-plane', m1: 'WHITE', m2: 'DARK', xsize: 2, zsize: 2 },
    },
    lights: [],
    objects: [
        { type: 'skysphere', sun_direction: [1, 2, 1] },
        { type: 'checkeredplane', y: 0, medium: 'TILES' },
        { type: 'sphere', location: [-2.2, 1, 0], radius: 1, material: 'RED' },
        { type: 'sphere', location: [0, 1, 0], radius: 1, material: 'GLASS' },
        { type: 'sphere', location: [2.2, 1, 0], radius: 1, material: 'GOLD' },
    ],
};

// Scene files in the repository, by path from its root (which is where their own asset paths
// start from too).
const SCENES = [
    'demo/scenes/spheres.json', 'demo/demo.json', 'demo/scenes/instances.json',
    'demo/scenes/sky-sunset.json', 'demo/scenes/sky-custom.json', 'demo/scenes/block-terrain.json',
    'demo/scenes/fireworks.json', 'demo/scenes/noise-simple-demo.json',
    'demo/scenes/spheres-on-water.json', 'demo/scenes/ocean-1.json', 'demo/scenes/ocean-boat.json',
    'demo/scenes/grass-meadow.json',
];

// What scene files may contain, from the renderer (src/schema.json): filled in at start.
let schema = { sections: {}, objects: {}, materials: {}, media: {} };

// Things to add that are arrangements rather than types.
const EXTRAS = {
    // The file's teapot is 32 units long and centred on the origin: scaled to 3.2, and lifted to sit on y = 0.
    'Teapot (mesh)': () => ({
        type: 'transform', translate: [0, 0.79, 3], rotate: { yaw: 0 }, material: 'M',
        item: { type: 'smoothmesh', src: 'demo/teapot-normals.obj', scale: [0.1, 0.1, 0.1] },
    }),
};

// Parts of the scene file with their own place in the outline; the rest are "settings".
const SECTIONS = ['camera', 'sun', 'output', 'materials', 'media', 'geometries', 'lights', 'objects', 'variables'];
const LIST_SECTIONS = { materials: 'Materials', media: 'Media', geometries: 'Geometries', lights: 'Lights', objects: 'Objects' };

let doc = structuredClone(STARTER);
let selection = ['objects', 4];   // a path into doc, or ['settings']

function nodeAt(path) {
    if (path[0] === 'settings') { return doc; }
    return path.reduce((o, k) => (o == null ? undefined : o[k]), doc);
}

// ---------------------------------------------------------------------------------------
// The preview

const workerCount = Math.max(1, Math.min(
    Number(new URLSearchParams(location.search).get('workers')) || (navigator.hardwareConcurrency || 4) - 1, 16));
const workers = [];
let encoder = null;          // a renderer instance on this thread, for encoding only
let gen = 0;                 // generation of the scene/camera being shown
let pw = 0, ph = 0;          // preview size in pixels
let sums = null;             // Float32Array, linear sums, 3 per pixel
let rowSamples = null;       // Uint32Array, samples added to each row so far
let image = null;            // ImageData on the canvas
let dirty = false;
let stats = { rays: 0, since: 0, buildMs: 0, built: 0, rendering: false };
const assetCache = new Map();   // path -> Promise<Uint8Array | null>
const assetsSent = new Set();
// While the camera is being moved, quick draft frames stand in for the path-traced image.
const DRAFT_SCALE = 0.5;        // draft size, as a fraction of the preview's
let interacting = false;        // the camera is being moved
let draft = null;               // the draft frame being gathered from the workers
let draftQueued = false;        // the camera moved again while that one was being made
let idleTimer = 0;
const draftCanvas = document.createElement('canvas');
let pickId = 0;
const picks = new Map();

function previewSize() {
    const long = Number($('preview-size').value);
    const w = typeof doc.width === 'number' ? doc.width : 640;
    const h = typeof doc.height === 'number' ? doc.height : 480;
    const s = long / Math.max(w, h);
    return [Math.max(8, Math.round(w * s)), Math.max(8, Math.round(h * s))];
}

// The scene as the workers render it: at preview size, one sample per pixel per layer.
function previewDoc() {
    const [w, h] = previewSize();
    return { ...doc, width: w, height: h, samples_per_chunk: 1 };
}

// Paths of files the scene refers to, wherever they appear.
function assetPaths(value, found = new Set()) {
    if (typeof value === 'string') {
        if (/\.(obj|ply|png|jpe?g|exr|hdr|tga|bmp)$/i.test(value)) { found.add(value); }
    } else if (value && typeof value === 'object') {
        for (const v of Object.values(value)) { assetPaths(v, found); }
    }
    return found;
}

function fetchAsset(path) {
    if (!assetCache.has(path)) {
        assetCache.set(path, (async () => {
            const response = await fetch(new URL('../' + path, location.href));
            if (!response.ok) { throw new Error(`Can't load ${path} (${response.status})`); }
            return new Uint8Array(await response.arrayBuffer());
        })().catch(e => { assetCache.delete(path); throw e; }));
    }
    return assetCache.get(path);
}

function resetAccumulation() {
    [pw, ph] = previewSize();
    sums = new Float32Array(pw * ph * 3);
    rowSamples = new Uint32Array(ph);
    if (canvas.width !== pw || canvas.height !== ph || !image) {
        canvas.width = pw; canvas.height = ph;
        image = ctx.createImageData(pw, ph);
        // Opaque black until the first rows arrive.
        for (let i = 3; i < image.data.length; i += 4) { image.data[i] = 255; }
        ctx.putImageData(image, 0, 0);
    }
    stats.rays = 0; stats.since = performance.now();
}

// Rebuild the scene in every worker and start rendering again.
async function restartScene() {
    const mine = ++gen;
    showError(null);
    interacting = false; draft = null; draftQueued = false;
    stats.built = 0; stats.buildMs = 0; stats.rendering = false;
    for (const w of workers) { w.postMessage({ type: 'stop' }); }
    const json = JSON.stringify(previewDoc());
    try {
        const paths = [...assetPaths(doc)];
        // A file that can't be fetched is left out: the scan above also finds paths in parts
        // of the file the renderer ignores, and if the file is needed the renderer says so.
        const files = await Promise.all(paths.map(p => fetchAsset(p).catch(() => null)));
        if (mine !== gen) { return; }
        paths.forEach((path, i) => {
            if (files[i] && !assetsSent.has(path)) {
                assetsSent.add(path);
                for (const w of workers) { w.postMessage({ type: 'asset', path, bytes: files[i] }); }
            }
        });
    } catch (e) {
        if (mine === gen) { showError(e.message); }
        return;
    }
    resetAccumulation();
    for (const w of workers) { w.postMessage({ type: 'scene', gen: mine, json }); }
    updateStatus();
}

// Keep the built scene and render it from a new camera.
function restartCamera() {
    const c = doc.camera;
    const plain = c && typeof c === 'object' && !JSON.stringify(c).includes('"$');
    if (!plain || stats.built < workers.length) {
        // Variables are substituted when the scene is built, and a scene still building would
        // start from its old camera: build again in both cases.
        scheduleScene();
        return;
    }
    const mine = ++gen;
    resetAccumulation();
    const json = JSON.stringify(c);
    for (const w of workers) { w.postMessage({ type: 'camera', gen: mine, json }); }
}

const cameraIsPlain = () => doc.camera && typeof doc.camera === 'object' && !JSON.stringify(doc.camera).includes('"$');

// The camera is being moved by hand: follow it with drafts until the move ends.
function moveCamera() {
    if (!cameraIsPlain() || stats.built < workers.length) { restartCamera(); return; }
    interacting = true;
    dirty = false;
    requestDraft();
    // A drag ends when the pointer is released; the wheel and the keys, when they go quiet.
    clearTimeout(idleTimer);
    if (!drag) { idleTimer = setTimeout(endInteraction, 300); }
}

// Ask every worker for its part of a draft from the current camera. One at a time: a camera
// move during a draft asks again when it is done, so drafts never queue up behind the pointer.
function requestDraft() {
    if (draft) { draftQueued = true; return; }
    draftQueued = false;
    const mine = ++gen;
    const cols = Math.max(8, Math.round(pw * DRAFT_SCALE)), rows = Math.max(8, Math.round(ph * DRAFT_SCALE));
    draft = { gen: mine, cols, rows, values: new Float32Array(cols * rows * 3), got: 0 };
    const json = JSON.stringify(doc.camera);
    for (const w of workers) { w.postMessage({ type: 'draft', gen: mine, json, cols, rows }); }
}

function onDraft(m) {
    if (!draft || m.gen !== draft.gen) { return; }
    draft.values.set(m.values, m.y0 * draft.cols * 3);
    draft.got += m.y1 - m.y0;
    if (draft.got < draft.rows) { return; }
    drawDraft(draft);
    draft = null;
    if (draftQueued) { requestDraft(); } else if (!interacting) { restartCamera(); }
}

// Show a draft, scaled up to the preview. It also becomes the picture the path-traced rows
// replace as they arrive.
function drawDraft({ cols, rows, values }) {
    if (!encoder) { return; }
    let rgba;
    try {
        rgba = encoder.encode(values, 1, doc.output && typeof doc.output === 'object' ? doc.output : null);
    } catch (e) {
        encoder = null;
        instantiate().then(r => { encoder = r; });
        return;
    }
    draftCanvas.width = cols; draftCanvas.height = rows;
    draftCanvas.getContext('2d').putImageData(new ImageData(rgba, cols, rows), 0, 0);
    ctx.imageSmoothingEnabled = true;
    ctx.drawImage(draftCanvas, 0, 0, pw, ph);
    image = ctx.getImageData(0, 0, pw, ph);
    $('status').textContent = `${pw}×${ph} · moving (draft)`;
}

// The move is over: path trace from where the camera now is.
function endInteraction() {
    clearTimeout(idleTimer);
    if (!interacting) { return; }
    interacting = false;
    if (!draft) { restartCamera(); }
}

let sceneTimer = 0;
function scheduleScene(delay = 120) {
    clearTimeout(sceneTimer);
    sceneTimer = setTimeout(restartScene, delay);
}

function onWorkerMessage({ data: m }) {
    if (m.type === 'picked') {
        picks.get(m.id)?.(m.index);
        picks.delete(m.id);
        return;
    }
    if (m.gen !== gen) { return; }
    if (m.type === 'draft') {
        onDraft(m);
    } else if (m.type === 'built') {
        stats.built += 1;
        stats.buildMs = Math.max(stats.buildMs, m.ms);
        if (stats.built === 1) { stats.since = performance.now(); }
    } else if (m.type === 'band') {
        if (m.sums.length !== (m.y1 - m.y0) * pw * 3) { return; }
        const at = m.y0 * pw * 3;
        for (let i = 0; i < m.sums.length; i++) { sums[at + i] += m.sums[i]; }
        for (let y = m.y0; y < m.y1; y++) { rowSamples[y] += 1; }
        stats.rays += m.rays;
        stats.rendering = true;
        dirty = true;
    } else if (m.type === 'error') {
        showError(m.message);
    }
}

// Draw what has accumulated. Rows with no samples yet keep the previous image, so moving the
// camera or editing replaces the picture progressively rather than flashing to black.
function draw() {
    requestAnimationFrame(draw);
    if (!dirty || !encoder || !sums) { return; }
    dirty = false;
    const averaged = new Float32Array(sums.length);
    const row = pw * 3;
    for (let y = 0; y < ph; y++) {
        const n = rowSamples[y];
        if (n === 0) { continue; }
        const scale = 1 / n;
        for (let i = y * row, end = i + row; i < end; i++) { averaged[i] = sums[i] * scale; }
    }
    let rgba;
    try {
        rgba = encoder.encode(averaged, 1, doc.output && typeof doc.output === 'object' ? doc.output : null);
    } catch (e) {
        // Bad output settings; the workers report the same error. Start a new encoder.
        encoder = null;
        instantiate().then(r => { encoder = r; });
        return;
    }
    for (let y = 0; y < ph; y++) {
        if (rowSamples[y] === 0) { continue; }
        const a = y * pw * 4;
        image.data.set(rgba.subarray(a, a + pw * 4), a);
    }
    ctx.putImageData(image, 0, 0);
    updateStatus();
}

function updateStatus() {
    let text;
    if (stats.built < workers.length && !stats.rendering) {
        text = `Building scene… (${stats.built}/${workers.length} workers)`;
    } else {
        let samples = Infinity;
        for (const n of rowSamples) { samples = Math.min(samples, n); }
        const seconds = (performance.now() - stats.since) / 1000;
        const rate = seconds > 0 ? stats.rays / seconds : 0;
        text = `${pw}×${ph} · ${samples} samples/pixel · ${formatCount(rate)} rays/s · ` +
            `built in ${formatMs(stats.buildMs)} · ${workers.length} workers`;
    }
    $('status').textContent = text;
}

const formatCount = v => v >= 1e6 ? (v / 1e6).toFixed(2) + 'M' : v >= 1e3 ? (v / 1e3).toFixed(0) + 'K' : v.toFixed(0);
const formatMs = v => v >= 1000 ? (v / 1000).toFixed(1) + ' s' : Math.round(v) + ' ms';

function showError(message) {
    $('error').hidden = !message;
    $('error').textContent = message ?? '';
    if (message) { $('status').textContent = 'Scene error'; }
}

// ---------------------------------------------------------------------------------------
// Camera controls

const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const mul = (a, s) => [a[0] * s, a[1] * s, a[2] * s];
const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const len = a => Math.hypot(a[0], a[1], a[2]);
const unit = a => mul(a, 1 / (len(a) || 1));
// Rotate v about unit axis k by angle t (Rodrigues' formula).
const rotate = (v, k, t) => add(add(mul(v, Math.cos(t)), mul(cross(k, v), Math.sin(t))), mul(k, dot(k, v) * (1 - Math.cos(t))));
const round = v => v.map(x => Math.round(x * 1e4) / 1e4);
const isVec3 = v => Array.isArray(v) && v.length === 3 && v.every(x => typeof x === 'number');

function cameraFrame() {
    const c = doc.camera;
    if (!c || !isVec3(c.location) || !isVec3(c.lookat)) { return null; }
    const up = unit(isVec3(c.up) ? c.up : [0, 1, 0]);
    const offset = sub(c.location, c.lookat);
    const forward = unit(mul(offset, -1));
    const right = unit(cross(forward, up));
    return { c, up, offset, forward, right, camUp: cross(right, forward), dist: len(offset) };
}

function orbit(dx, dy) {
    const f = cameraFrame();
    if (!f) { return; }
    let offset = rotate(f.offset, f.up, -dx * 0.008);
    // Tilt about the horizontal axis, stopping short of looking straight along `up`.
    const right = unit(cross(mul(offset, -1), f.up));
    const tilted = rotate(offset, right, -dy * 0.008);
    const cos = dot(unit(tilted), f.up);
    if (Math.abs(cos) < 0.995) { offset = tilted; }
    f.c.location = round(add(f.c.lookat, offset));
    cameraChanged();
}

function pan(dx, dy) {
    const f = cameraFrame();
    if (!f) { return; }
    const move = add(mul(f.right, -dx * f.dist * 0.0015), mul(f.camUp, dy * f.dist * 0.0015));
    f.c.location = round(add(f.c.location, move));
    f.c.lookat = round(add(f.c.lookat, move));
    cameraChanged();
}

function dolly(amount) {
    const f = cameraFrame();
    if (!f) { return; }
    f.c.location = round(add(f.c.lookat, mul(f.offset, Math.exp(amount))));
    cameraChanged();
}

function cameraChanged() {
    moveCamera();
    syncJsonText();
    if (selection[0] === 'camera' && !$('inspector').contains(document.activeElement)) { renderInspector(); }
}

let drag = null;
canvas.addEventListener('pointerdown', e => {
    canvas.setPointerCapture(e.pointerId);
    drag = { x: e.clientX, y: e.clientY, moved: 0, pan: e.shiftKey || e.button !== 0 };
    canvas.classList.add('dragging');
});
canvas.addEventListener('pointermove', e => {
    if (!drag) { return; }
    const dx = e.clientX - drag.x, dy = e.clientY - drag.y;
    drag.x = e.clientX; drag.y = e.clientY;
    drag.moved += Math.abs(dx) + Math.abs(dy);
    if (drag.moved < 4) { return; }
    if (drag.pan) { pan(dx, dy); } else { orbit(dx, dy); }
});
canvas.addEventListener('pointerup', e => {
    canvas.classList.remove('dragging');
    const clicked = drag && drag.moved < 4;
    drag = null;
    if (clicked) { pickAt(e); } else { endInteraction(); }
});
canvas.addEventListener('pointercancel', () => { drag = null; canvas.classList.remove('dragging'); endInteraction(); });
canvas.addEventListener('contextmenu', e => e.preventDefault());
canvas.addEventListener('wheel', e => { e.preventDefault(); dolly(Math.sign(e.deltaY) * 0.08); }, { passive: false });
canvas.tabIndex = 0;
canvas.setAttribute('aria-label', 'Preview. Arrow keys orbit, plus and minus zoom.');
canvas.addEventListener('keydown', e => {
    const step = 12;
    const keys = {
        ArrowLeft: () => orbit(-step, 0), ArrowRight: () => orbit(step, 0),
        ArrowUp: () => orbit(0, -step), ArrowDown: () => orbit(0, step),
        '+': () => dolly(-0.08), '=': () => dolly(-0.08), '-': () => dolly(0.08),
    };
    if (keys[e.key]) { e.preventDefault(); keys[e.key](); }
});

// Select the object under the pointer. A worker answers, as it has the built scene.
function pickAt(e) {
    const box = canvas.getBoundingClientRect();
    // The canvas is letterboxed inside its box (object-fit: contain).
    const scale = Math.min(box.width / pw, box.height / ph);
    const x = (e.clientX - box.left - (box.width - pw * scale) / 2) / scale;
    const y = (e.clientY - box.top - (box.height - ph * scale) / 2) / scale;
    if (x < 0 || y < 0 || x >= pw || y >= ph || !workers.length) { return; }
    const id = ++pickId;
    picks.set(id, index => { if (index >= 0 && doc.objects?.[index]) { select(['objects', index]); } });
    workers[0].postMessage({ type: 'pick', id, x, y });
}

// ---------------------------------------------------------------------------------------
// Outline

function describe(section, key, value) {
    if (section === 'lights') { return [`Light ${key + 1}`, 'point']; }
    if (section === 'objects') {
        const name = value?.material ?? value?.medium ?? value?.src?.split('/').pop() ?? '';
        return [`${value?.type ?? 'object'}${name ? ' · ' + name : ''}`, `#${key}`];
    }
    return [String(key), value?.type ?? ''];
}

function renderOutline() {
    const list = $('outline');
    list.replaceChildren();
    const item = (path, name, kind) => {
        const li = document.createElement('li');
        const b = document.createElement('button');
        b.type = 'button';
        b.innerHTML = '<span class="name"></span><span class="kind"></span>';
        b.firstChild.textContent = name;
        b.lastChild.textContent = kind;
        if (JSON.stringify(path) === JSON.stringify(selection)) { b.setAttribute('aria-current', 'true'); }
        b.addEventListener('click', () => select(path));
        li.append(b);
        list.append(li);
    };
    item(['settings'], 'Render settings', '');
    if (doc.camera) { item(['camera'], 'Camera', ''); }
    if (doc.sun && typeof doc.sun === 'object') { item(['sun'], 'Sun', ''); }
    item(['output'], 'Output', '');
    for (const [section, title] of Object.entries(LIST_SECTIONS)) {
        const group = doc[section];
        if (!group || typeof group !== 'object' || Object.keys(group).length === 0) { continue; }
        const head = document.createElement('li');
        head.className = 'group';
        head.textContent = title;
        list.append(head);
        for (const key of Object.keys(group)) {
            const k = Array.isArray(group) ? Number(key) : key;
            item([section, k], ...describe(section, k, group[k]));
        }
    }
}

function select(path) {
    selection = path;
    renderOutline();
    renderInspector();
    $('outline').querySelector('[aria-current]')?.scrollIntoView({ block: 'nearest' });
}

// ---------------------------------------------------------------------------------------
// Properties
//
// The fields shown for a node come from the schema: every field its type accepts, in the
// schema's order, whether or not the scene file sets it. A field the file doesn't set shows
// its default, dimmed; editing it writes the key, and its reset button removes the key again.
// Keys in the file that the schema doesn't know are listed after, so nothing is hidden.

const COLOR_KEYS = /colou?r|albedo|reflective|pigment|attenuate|background|bottom|top/i;
const REFERENCES = { material: 'material', m1: 'material', m2: 'material', base_material: 'material', medium: 'medium' };
const isNumbers = (v, n) => Array.isArray(v) && v.length === n && v.every(x => typeof x === 'number');

// What the schema says about the node at `path`.
function nodeSchema(path, node) {
    const [section] = path;
    const sections = schema.sections ?? {};
    const typed = (category, common) => {
        const entry = schema[category]?.[node?.type];
        return { category, entry, fields: [...(entry?.fields ?? []), ...(common ? sections.object_common?.fields ?? [] : [])] };
    };
    if (section === 'settings') { return { fields: sections.settings?.fields ?? [], doc: sections.settings?.doc }; }
    if (section === 'objects') { return typed('objects', true); }
    if (section === 'geometries') { return typed('objects', false); }
    if (section === 'materials') { return typed('materials', false); }
    if (section === 'media') { return typed('media', false); }
    const name = section === 'lights' ? 'light' : section;
    return { fields: sections[name]?.fields ?? [], doc: sections[name]?.doc };
}

// The kind to edit a value as when the schema has nothing to say about its key.
function guessKind(key, value) {
    if (typeof value === 'number') { return 'number'; }
    if (typeof value === 'boolean') { return 'boolean'; }
    if (typeof value === 'string') { return REFERENCES[key] ?? 'string'; }
    if (isNumbers(value, 3)) { return COLOR_KEYS.test(key) ? 'color' : 'vec3'; }
    if (isNumbers(value, 2)) { return 'vec2'; }
    return 'json';
}

function renderInspector() {
    const form = $('inspector');
    form.replaceChildren();
    const isItem = selection.length === 2;
    // The output settings can be edited before the file has any: the object is made on the
    // first edit.
    const creatable = selection.length === 1 && selection[0] === 'output';
    const node = nodeAt(selection);
    $('delete').hidden = !isItem;
    $('duplicate').hidden = !isItem;
    $('inspector-title').textContent = selection[0] === 'settings' ? 'Render settings'
        : selection.length === 1 ? selection[0][0].toUpperCase() + selection[0].slice(1)
        : isItem ? describe(selection[0], selection[1], node)[0] : 'Properties';
    if ((!node || typeof node !== 'object') && !creatable) {
        form.innerHTML = '<p class="empty">Nothing selected.</p>';
        return;
    }
    const ref = {
        get: () => nodeAt(selection),
        ensure: () => nodeAt(selection) ?? (doc[selection[0]] = {}),
    };
    const ctx = { camera: selection[0] === 'camera' };
    const info = nodeSchema(selection, node);
    const about = info.entry?.doc ?? info.doc;
    if (about) {
        const p = document.createElement('p');
        p.className = 'about';
        p.textContent = about;
        form.append(p);
    }
    if (info.category) { form.append(typeRow(ref, info.category, () => { renderOutline(); renderInspector(); })); }
    const hidden = selection[0] === 'settings' ? [...SECTIONS, 'type'] : ['type'];
    renderFields(form, ref, info.fields, ctx, hidden);
}

// Rows for `fields` of the object `ref` refers to, then rows for its other keys.
function renderFields(container, ref, fields, ctx, hidden = ['type']) {
    for (const f of fields) { container.append(fieldRow(ref, f, ctx)); }
    const known = new Set(fields.map(f => f.name));
    const node = ref.get();
    const extras = Object.keys(node && typeof node === 'object' ? node : {}).filter(k => !known.has(k) && !hidden.includes(k));
    if (extras.length && fields.length) {
        const p = document.createElement('p');
        p.className = 'group-label';
        p.textContent = 'Other keys in the file';
        container.append(p);
    }
    for (const key of extras) {
        container.append(fieldRow(ref, { name: key, kind: guessKind(key, node[key]), extra: true }, ctx));
    }
}

// The row choosing a node's type, from the types the schema has for its category.
function typeRow(ref, category, changed) {
    const row = document.createElement('div');
    row.className = 'field';
    const label = document.createElement('label');
    label.textContent = 'type';
    const inputs = document.createElement('div');
    inputs.className = 'inputs';
    const select = typeSelect(category, ref.get()?.type);
    select.addEventListener('change', () => {
        retype(ref.ensure(), category, select.value);
        edited(false);
        changed();
    });
    label.htmlFor = select.id = `f-type-${++fieldId}`;
    inputs.append(select);
    row.append(label, inputs);
    return row;
}

// Change a node's type, keeping the keys it has and giving it the new type's required
// fields, from that type's example, where it lacks them.
function retype(node, category, type) {
    node.type = type;
    const entry = schema[category]?.[type];
    const example = instanceOf(category, type);
    for (const f of entry?.fields ?? []) {
        if (f.required && node[f.name] === undefined && example[f.name] !== undefined) { node[f.name] = example[f.name]; }
    }
}

// A select of a category's types, grouped as the schema groups them.
function typeSelect(category, current, placeholder) {
    const select = document.createElement('select');
    if (placeholder) { select.append(new Option(placeholder, '')); }
    const groups = new Map();
    for (const [type, entry] of Object.entries(schema[category] ?? {})) {
        const g = entry.group ?? '';
        if (!groups.has(g)) { groups.set(g, []); }
        groups.get(g).push([type, entry]);
    }
    for (const [g, list] of groups) {
        const parent = g ? select.appendChild(Object.assign(document.createElement('optgroup'), { label: g })) : select;
        for (const [type, entry] of list) { parent.append(new Option(`${entry.label} (${type})`, type)); }
    }
    if (current !== undefined && !schema[category]?.[current]) { select.append(new Option(String(current), current)); }
    if (current !== undefined) { select.value = current; }
    return select;
}

let fieldId = 0;

// One field of the object `ref` refers to: a label, its editing controls, and a reset button
// when the file sets it.
function fieldRow(ref, f, ctx) {
    const key = f.name;
    const row = document.createElement('div');
    const id = `f-${key}-${++fieldId}`;
    const label = document.createElement('label');
    label.textContent = key + (f.required ? ' *' : '');
    label.title = [f.doc, f.required ? 'Required.' : '', f.min !== undefined ? `Minimum ${f.min}.` : '', f.max !== undefined ? `Maximum ${f.max}.` : '']
        .filter(Boolean).join(' ') || key;
    label.htmlFor = id;
    const inputs = document.createElement('div');
    inputs.className = 'inputs';

    const present = () => ref.get()?.[key] !== undefined;
    const current = () => (present() ? ref.get()[key] : f.default);
    const mark = () => { row.className = 'field' + (present() ? '' : ' unset') + (nested ? ' nested' : ''); reset.hidden = !present() || Boolean(f.required); };
    const set = v => { ref.ensure()[key] = v; mark(); edited(ctx.camera); };
    // The value itself as something fields can be read from and written to.
    const slot = { get: () => ref.get()?.[key], set };

    const reset = document.createElement('button');
    reset.type = 'button';
    reset.className = 'reset';
    reset.textContent = '×';
    reset.title = f.extra ? `Remove ${key}` : `Remove ${key}: use the default`;
    reset.setAttribute('aria-label', reset.title);
    reset.addEventListener('click', () => {
        delete ref.get()[key];
        edited(ctx.camera);
        row.replaceWith(f.extra ? '' : fieldRow(ref, f, ctx));
    });

    const value = current();
    let kind = f.kind;
    // A value of another shape than its kind expects (a "$variable", a colour by name) is
    // edited as text or JSON rather than lost.
    if (value !== undefined && value !== null) {
        const fits = {
            number: typeof value === 'number', integer: typeof value === 'number', boolean: typeof value === 'boolean',
            vec2: isNumbers(value, 2), vec3: isNumbers(value, 3), color: isNumbers(value, 3),
            enum: typeof value === 'string', material: typeof value === 'string', medium: typeof value === 'string',
            string: typeof value === 'string', path: typeof value === 'string', expression: typeof value === 'string',
            object: typeof value === 'object' && !Array.isArray(value), shapes: Array.isArray(value),
            shape: typeof value === 'string' || (typeof value === 'object' && !Array.isArray(value)),
        }[kind];
        if (fits === false) { kind = typeof value === 'string' ? 'string' : 'json'; }
    }
    const nested = kind === 'object' || kind === 'shape' || kind === 'shapes' || kind === 'geometry';

    const number = (get, put, name, integer) => {
        const input = document.createElement('input');
        input.type = 'number'; input.step = integer ? '1' : 'any';
        const v = get();
        input.value = v ?? '';
        if (v === undefined || v === null) { input.placeholder = 'auto'; }
        if (f.min !== undefined) { input.min = f.min; }
        if (f.max !== undefined) { input.max = f.max; }
        input.setAttribute('aria-label', name);
        input.addEventListener('input', () => {
            const n = input.valueAsNumber;
            if (Number.isFinite(n)) { put(integer ? Math.round(n) : n); }
        });
        return input;
    };

    if (kind === 'number' || kind === 'integer') {
        const input = number(current, set, key, kind === 'integer');
        input.id = id;
        inputs.append(input);
    } else if (kind === 'boolean') {
        const input = document.createElement('input');
        input.type = 'checkbox'; input.checked = Boolean(value); input.id = id;
        input.addEventListener('change', () => set(input.checked));
        inputs.append(input);
    } else if (kind === 'enum' || kind === 'material' || kind === 'medium') {
        const input = document.createElement('select');
        input.id = id;
        const names = kind === 'enum' ? f.options ?? [] : Object.keys(doc[kind === 'material' ? 'materials' : 'media'] ?? {});
        if (value === undefined || value === null) { input.append(new Option(kind === 'enum' ? '(default)' : '(none)', '')); }
        for (const name of value === undefined || value === null || names.includes(value) ? names : [value, ...names]) {
            input.append(new Option(name, name));
        }
        input.value = value ?? '';
        input.addEventListener('change', () => { if (input.value !== '') { set(input.value); renderOutline(); } });
        inputs.append(input);
    } else if (kind === 'string' || kind === 'path' || kind === 'expression') {
        const input = document.createElement('input');
        input.type = 'text'; input.value = value ?? ''; input.id = id;
        input.spellcheck = false;
        input.addEventListener('change', () => { set(input.value); renderOutline(); });
        inputs.append(input);
    } else if (kind === 'vec2' || kind === 'vec3' || kind === 'color') {
        const n = kind === 'vec2' ? 2 : 3;
        // Editing one part of an unset vector writes the whole vector, from its default.
        const whole = () => (present() ? ref.get()[key] : Array.isArray(f.default) ? [...f.default] : new Array(n).fill(0));
        let swatch = null;
        const parts = Array.from({ length: n }, (_, i) => number(() => current()?.[i], v => {
            const next = whole();
            next[i] = v;
            if (swatch) { swatch.value = toHex(next); }
            set(next);
        }, `${key} ${'xyz'[i]}`));
        parts[0].id = id;
        if (kind === 'color' && (value ?? [0, 0, 0]).every(x => x >= 0 && x <= 1)) {
            swatch = document.createElement('input');
            swatch.type = 'color'; swatch.value = toHex(value ?? [0, 0, 0]);
            swatch.setAttribute('aria-label', `${key} colour`);
            swatch.addEventListener('input', () => {
                const next = fromHex(swatch.value);
                parts.forEach((p, i) => { p.value = next[i]; });
                set(next);
            });
        }
        inputs.append(...parts, ...(swatch ? [swatch] : []));
    } else if (kind === 'object') {
        // A nested object: its own fields, folded away until wanted.
        const child = { get: slot.get, ensure: () => { if (!slot.get() || typeof slot.get() !== 'object') { set({}); } return slot.get(); } };
        const details = fold(id, present() ? 'set' : 'defaults');
        renderFields(details, child, f.fields ?? [], ctx);
        inputs.append(details);
    } else if (kind === 'shape' || kind === 'geometry') {
        inputs.append(shapeEditor(slot, ctx, id, () => row.replaceWith(fieldRow(ref, f, ctx))));
    } else if (kind === 'shapes') {
        const list = document.createElement('div');
        list.className = 'shape-list';
        const again = () => row.replaceWith(fieldRow(ref, f, ctx));
        (value ?? []).forEach((_, i) => {
            const item = { get: () => slot.get()?.[i], set: v => { slot.get()[i] = v; edited(ctx.camera); } };
            const editor = shapeEditor(item, ctx, `${id}-${i}`, again, () => { slot.get().splice(i, 1); edited(ctx.camera); again(); });
            list.append(editor);
        });
        const add = typeSelect('objects', undefined, 'Add a shape…');
        add.id = id;
        add.addEventListener('change', () => {
            if (!add.value) { return; }
            const next = [...(slot.get() ?? []), instanceOf('objects', add.value)];
            set(next);
            again();
        });
        list.append(add);
        inputs.append(list);
    } else {
        // Anything else (lists, free-form values) is edited as JSON.
        const input = document.createElement('textarea');
        input.id = id; input.spellcheck = false;
        input.value = value === undefined ? '' : JSON.stringify(value, null, 1);
        input.rows = Math.min(8, Math.max(1, input.value.split('\n').length));
        input.addEventListener('change', () => {
            if (input.value.trim() === '') { return; }
            try { set(JSON.parse(input.value)); input.setCustomValidity(''); }
            catch (e) { input.setCustomValidity(e.message); input.reportValidity(); }
        });
        inputs.append(input);
    }
    row.append(label, inputs, reset);
    mark();
    return row;
}

// A fold-away container for nested fields.
function fold(id, summary) {
    const details = document.createElement('details');
    details.className = 'nested-fields';
    const s = document.createElement('summary');
    s.id = id;
    s.textContent = summary;
    details.append(s);
    return details;
}

// A nested shape: by name (an entry of "geometries") or as an object with a type of its own,
// whose fields are edited in place.
function shapeEditor(slot, ctx, id, again, remove) {
    const box = document.createElement('div');
    box.className = 'shape';
    const value = slot.get();
    const head = document.createElement('div');
    head.className = 'inputs';
    if (typeof value === 'string' || value === undefined || value === null) {
        // A named geometry, or nothing yet: choose a name or a type.
        const names = Object.keys(doc.geometries ?? {});
        const pick = typeSelect('objects', undefined, value === undefined || value === null ? 'Choose a shape…' : `"${value}" (named geometry)`);
        pick.id = id;
        if (names.length) {
            const group = Object.assign(document.createElement('optgroup'), { label: 'Named geometries' });
            for (const n of names) { group.append(new Option(n, 'name:' + n)); }
            pick.prepend(group);
            pick.prepend(pick.querySelector('option[value=""]'));
            pick.value = '';
        }
        pick.addEventListener('change', () => {
            if (!pick.value) { return; }
            slot.set(pick.value.startsWith('name:') ? pick.value.slice(5) : instanceOf('objects', pick.value));
            again();
        });
        head.append(pick);
        box.append(head);
    } else {
        const ref = { get: slot.get, ensure: slot.get };
        const type = typeSelect('objects', value.type);
        type.id = id;
        type.addEventListener('change', () => { retype(slot.get(), 'objects', type.value); edited(ctx.camera); again(); });
        head.append(type);
        const details = fold(`${id}-fields`, 'fields');
        const entry = schema.objects?.[value.type];
        renderFields(details, ref, [...(entry?.fields ?? []), ...(schema.sections?.object_common?.fields ?? [])], ctx);
        box.append(head, details);
    }
    if (remove) {
        const b = document.createElement('button');
        b.type = 'button'; b.className = 'reset'; b.textContent = '×';
        b.title = 'Remove this shape'; b.setAttribute('aria-label', b.title);
        b.addEventListener('click', remove);
        head.append(b);
    }
    return box;
}

// Colour swatches show a linear value through the sRGB curve, as the render does.
const toSrgb = v => v <= 0.0031308 ? 12.92 * v : 1.055 * Math.pow(v, 1 / 2.4) - 0.055;
const toLinear = v => v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
const toHex = c => '#' + c.map(v => Math.round(toSrgb(Math.min(1, Math.max(0, v))) * 255).toString(16).padStart(2, '0')).join('');
const fromHex = h => [1, 3, 5].map(i => Math.round(toLinear(parseInt(h.slice(i, i + 2), 16) / 255) * 1000) / 1000);

// The document changed through the property fields or the outline.
function edited(cameraOnly) {
    syncJsonText();
    if (cameraOnly) { restartCamera(); } else { scheduleScene(); }
}

$('delete').addEventListener('click', () => {
    const [section, key] = selection;
    if (Array.isArray(doc[section])) { doc[section].splice(key, 1); } else { delete doc[section][key]; }
    select(['settings']);
    edited(false);
});

$('duplicate').addEventListener('click', () => {
    const [section, key] = selection;
    const copy = structuredClone(doc[section][key]);
    if (Array.isArray(doc[section])) {
        if (isVec3(copy.location)) { copy.location = [copy.location[0] + 1, copy.location[1], copy.location[2]]; }
        doc[section].push(copy);
        select([section, doc[section].length - 1]);
    } else {
        const name = uniqueName(doc[section], key);
        doc[section][name] = copy;
        select([section, name]);
    }
    edited(false);
});

// A sun follows the skysphere's sun; without a skysphere it needs a direction of its own.
function sunFor(sun) {
    return doc.objects?.some(o => o?.type === 'skysphere') ? sun : { ...sun, direction: [1, 2, 1] };
}

function uniqueName(group, base) {
    const stem = String(base).replace(/_\d+$/, '');
    if (!(stem in group)) { return stem; }
    for (let i = 2; ; i++) {
        if (!(`${stem}_${i}` in group)) { return `${stem}_${i}`; }
    }
}

// A new instance of a type: the schema's example, with the material and medium it names
// (the schema's examples call them "M" and "MM") pointed at ones this scene has.
function instanceOf(category, type) {
    const example = structuredClone(schema[category]?.[type]?.example ?? { type });
    return adopt(example);
}

function adopt(value) {
    if (value === 'M') {
        doc.materials ??= {};
        const first = Object.keys(doc.materials)[0];
        if (first) { return first; }
        doc.materials.M = { type: 'lambertian', albedo: [0.8, 0.8, 0.8] };
        return 'M';
    }
    if (value === 'MM') {
        doc.media ??= {};
        const first = Object.keys(doc.media)[0];
        if (first) { return first; }
        doc.media.MM = { type: 'solid', material: adopt('M') };
        return 'MM';
    }
    if (Array.isArray(value)) { return value.map(adopt); }
    if (value && typeof value === 'object') {
        for (const k of Object.keys(value)) { value[k] = adopt(value[k]); }
    }
    return value;
}

// The Add menu: every type the schema has, by group, then the scene's other parts.
function buildAddMenu() {
    const add = $('add');
    add.replaceChildren(new Option('Add…', ''));
    const group = label => add.appendChild(Object.assign(document.createElement('optgroup'), { label }));
    const groups = new Map();
    for (const [type, entry] of Object.entries(schema.objects ?? {})) {
        const g = entry.group ?? 'Other';
        if (!groups.has(g)) { groups.set(g, group(g)); }
        groups.get(g).append(new Option(entry.label, `objects:${type}`));
    }
    const extras = group('Arrangements');
    for (const name of Object.keys(EXTRAS)) { extras.append(new Option(name, `extra:${name}`)); }
    const lights = group('Lights');
    lights.append(new Option('Sun', 'sun'), new Option('Point light', 'light'));
    for (const [category, title] of [['materials', 'Materials'], ['media', 'Media']]) {
        const g = group(title);
        for (const [type, entry] of Object.entries(schema[category] ?? {})) { g.append(new Option(entry.label, `${category}:${type}`)); }
    }
}

$('add').addEventListener('change', () => {
    const [kind, name] = $('add').value.split(/:(.*)/);
    $('add').value = '';
    if (!kind) { return; }
    if (kind === 'sun') {
        doc.sun = doc.sun && typeof doc.sun === 'object' ? doc.sun : sunFor({ intensity: 3 });
        select(['sun']);
    } else if (kind === 'light') {
        (doc.lights ??= []).push({ location: [0, 10, 5], intensity: 300, color: [1, 1, 1] });
        select(['lights', doc.lights.length - 1]);
    } else if (kind === 'materials' || kind === 'media') {
        doc[kind] ??= {};
        const key = uniqueName(doc[kind], name.replace(/[^a-z0-9]+/gi, '_').toUpperCase());
        // Named before its example is adopted, so a material it refers to isn't itself.
        doc[kind][key] = {};
        doc[kind][key] = instanceOf(kind, name);
        select([kind, key]);
    } else {
        const value = kind === 'extra' ? adopt(EXTRAS[name]()) : instanceOf('objects', name);
        // What the type can't be seen without (clouds need a sun to light them, and a sky).
        const needs = schema.objects?.[value.type]?.needs ?? [];
        doc.objects ??= [];
        if (needs.includes('skysphere') && !doc.objects.some(o => o?.type === 'skysphere')) { doc.objects.push({ type: 'skysphere' }); }
        if (needs.includes('sun') && !doc.sun) { doc.sun = sunFor({ intensity: 3 }); }
        doc.objects.push(value);
        select(['objects', doc.objects.length - 1]);
    }
    edited(false);
});

// ---------------------------------------------------------------------------------------
// The scene file as text

// JSON indented by two spaces, but with lists of plain numbers (vectors, colours) on one line.
function formatScene(value, indent = '') {
    if (Array.isArray(value)) {
        if (value.every(v => v === null || typeof v !== 'object')) { return '[' + value.map(v => JSON.stringify(v)).join(', ') + ']'; }
        return '[\n' + value.map(v => indent + '  ' + formatScene(v, indent + '  ')).join(',\n') + '\n' + indent + ']';
    }
    if (value && typeof value === 'object') {
        const keys = Object.keys(value).filter(k => value[k] !== undefined);
        if (keys.length === 0) { return '{}'; }
        return '{\n' + keys.map(k => `${indent}  ${JSON.stringify(k)}: ${formatScene(value[k], indent + '  ')}`).join(',\n') + '\n' + indent + '}';
    }
    return JSON.stringify(value);
}

const jsonText = $('json');

function syncJsonText() {
    if (document.activeElement !== jsonText) { jsonText.value = formatScene(doc); }
    setJsonState('');
}

function setJsonState(message) {
    $('json-state').textContent = message;
    $('json-state').classList.toggle('bad', Boolean(message));
}

let jsonTimer = 0;
jsonText.addEventListener('input', () => {
    clearTimeout(jsonTimer);
    jsonTimer = setTimeout(() => {
        let parsed;
        try {
            parsed = JSON.parse(jsonText.value);
            if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) { throw new Error('A scene is a JSON object'); }
        } catch (e) {
            setJsonState(e.message.replace(/^JSON\.parse: /, ''));
            return;
        }
        setJsonState('');
        doc = parsed;
        if (nodeAt(selection) === undefined) { selection = ['settings']; }
        renderOutline();
        renderInspector();
        scheduleScene();
    }, 300);
});
jsonText.addEventListener('blur', () => { if (!$('json-state').textContent) { syncJsonText(); } });

function setDocument(next) {
    doc = next;
    selection = doc.camera ? ['camera'] : ['settings'];
    jsonText.value = formatScene(doc);
    setJsonState('');
    renderOutline();
    renderInspector();
    restartScene();
}

// ---------------------------------------------------------------------------------------
// Header controls

$('scene-picker').append(new Option('Starter scene', ''));
for (const path of SCENES) { $('scene-picker').append(new Option(path.split('/').pop().replace('.json', ''), path)); }
$('scene-picker').addEventListener('change', async () => {
    const path = $('scene-picker').value;
    if (!path) { setDocument(structuredClone(STARTER)); return; }
    try {
        const response = await fetch(new URL('../' + path, location.href));
        if (!response.ok) { throw new Error(`Can't load ${path} (${response.status})`); }
        setDocument(JSON.parse(await response.text()));
    } catch (e) {
        showError(e.message);
    }
});

$('preview-size').addEventListener('change', restartScene);

$('download').addEventListener('click', () => {
    const a = document.createElement('a');
    a.href = URL.createObjectURL(new Blob([formatScene(doc) + '\n'], { type: 'application/json' }));
    a.download = ($('scene-picker').value.split('/').pop() || 'scene.json');
    a.click();
    URL.revokeObjectURL(a.href);
});

$('save-image').addEventListener('click', () => {
    // The preview as it stands: at preview size, with the samples gathered so far.
    canvas.toBlob(blob => {
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = ($('scene-picker').value.split('/').pop() || 'scene.json').replace(/\.json$/, '') + '.png';
        a.click();
        URL.revokeObjectURL(a.href);
    }, 'image/png');
});

// The sidebar shows the outline and properties, or the scene file's text.
function showTab(name) {
    for (const tab of ['outline', 'json']) {
        $(`tab-${tab}`).setAttribute('aria-selected', String(tab === name));
        $(`${tab}-view`).hidden = tab !== name;
    }
    if (name === 'json') { syncJsonText(); }
}
$('tab-outline').addEventListener('click', () => showTab('outline'));
$('tab-json').addEventListener('click', () => showTab('json'));

// ---------------------------------------------------------------------------------------
// Start

async function start() {
    renderOutline();
    renderInspector();
    jsonText.value = formatScene(doc);
    $('status').textContent = 'Loading renderer…';
    try {
        encoder = await instantiate();
        schema = encoder.schema();
        buildAddMenu();
        renderInspector();
    } catch (e) {
        showError(e.message);
        return;
    }
    for (let index = 0; index < workerCount; index++) {
        const w = new Worker(new URL('worker.js', import.meta.url), { type: 'module' });
        w.onmessage = onWorkerMessage;
        w.onerror = e => showError(`Render worker failed: ${e.message}`);
        w.postMessage({ type: 'init', index, count: workerCount });
        workers.push(w);
    }
    restartScene();
    requestAnimationFrame(draw);
}

// For debugging and tests.
window.rays = { get doc() { return doc; }, get schema() { return schema; }, get interacting() { return interacting || Boolean(draft); }, get stats() { return stats; }, get rowSamples() { return rowSamples; }, select };

start();
