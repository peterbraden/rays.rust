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

const firstKey = o => Object.keys(o ?? {})[0];
const TEMPLATES = {
    'Sphere': d => ['objects', { type: 'sphere', location: [0, 1, 0], radius: 1, material: firstKey(d.materials) }],
    'Box': d => ['objects', { type: 'box', min: [-1, 0, -1], max: [1, 2, 1], material: firstKey(d.materials) }],
    'Ground plane': d => ['objects', { type: 'plane', y: 0, material: firstKey(d.materials) }],
    // The file's teapot is 32 units long and centred on the origin: scaled to 3.2, and lifted to sit on y = 0.
    'Teapot mesh': d => ['objects', {
        type: 'transform', translate: [0, 0.79, 3], rotate: { yaw: 0 }, material: firstKey(d.materials),
        item: { type: 'smoothmesh', src: 'demo/teapot-normals.obj', scale: [0.1, 0.1, 0.1] },
    }],
    'Sky': () => ['objects', { type: 'skysphere' }],
    'Point light': () => ['lights', { location: [0, 10, 5], intensity: 300, color: [1, 1, 1] }],
    'Material: diffuse': () => ['materials', { type: 'lambertian', albedo: [0.8, 0.8, 0.8] }],
    'Material: metal': () => ['materials', { type: 'metal', reflective: [0.9, 0.9, 0.9], roughness: 0.1 }],
    'Material: glass': () => ['materials', { type: 'dielectric', refractive_index: 1.5, attenuate: [1, 1, 1] }],
    'Material: emitter': () => ['materials', { type: 'diffuse-light', color: [1, 1, 1], intensity: 5 }],
};

// Parts of the scene file with their own place in the outline; the rest are "settings".
const SECTIONS = ['camera', 'materials', 'media', 'geometries', 'lights', 'objects', 'variables'];
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
    if (m.type === 'built') {
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
    restartCamera();
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
    if (clicked) { pickAt(e); }
});
canvas.addEventListener('pointercancel', () => { drag = null; canvas.classList.remove('dragging'); });
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

const COLOR_KEYS = /colou?r|albedo|reflective|pigment|attenuate|background|bottom|top/i;
const REFERENCES = { material: 'materials', m1: 'materials', m2: 'materials', base_material: 'materials', medium: 'media' };

function renderInspector() {
    const form = $('inspector');
    form.replaceChildren();
    const target = nodeAt(selection);
    const isItem = selection.length === 2;
    $('delete').hidden = !isItem;
    $('duplicate').hidden = !isItem;
    $('inspector-title').textContent = selection[0] === 'settings' ? 'Render settings'
        : selection[0] === 'camera' ? 'Camera'
        : isItem ? describe(selection[0], selection[1], target)[0] : 'Properties';
    if (!target || typeof target !== 'object') {
        form.innerHTML = '<p class="empty">Nothing selected.</p>';
        return;
    }
    const keys = Object.keys(target).filter(k => selection[0] !== 'settings' || !SECTIONS.includes(k));
    for (const key of keys) { form.append(field(target, key)); }
    if (keys.length === 0) { form.innerHTML = '<p class="empty">No properties: every setting has its default.</p>'; }
}

function field(target, key) {
    const value = target[key];
    const row = document.createElement('div');
    row.className = 'field';
    const id = `f-${key}`;
    const label = document.createElement('label');
    label.textContent = key;
    label.title = key;
    label.htmlFor = id;
    const inputs = document.createElement('div');
    inputs.className = 'inputs';
    row.append(label, inputs);
    const set = v => { target[key] = v; edited(selection[0] === 'camera'); };

    const number = (get, put, name) => {
        const input = document.createElement('input');
        input.type = 'number'; input.step = 'any'; input.value = get();
        input.setAttribute('aria-label', name);
        input.addEventListener('input', () => {
            const v = input.valueAsNumber;
            if (Number.isFinite(v)) { put(v); }
        });
        return input;
    };

    if (typeof value === 'number') {
        const input = number(() => value, set, key);
        input.id = id;
        inputs.append(input);
    } else if (typeof value === 'boolean') {
        const input = document.createElement('input');
        input.type = 'checkbox'; input.checked = value; input.id = id;
        input.addEventListener('change', () => set(input.checked));
        inputs.append(input);
    } else if (typeof value === 'string' && REFERENCES[key] && doc[REFERENCES[key]]) {
        const input = document.createElement('select');
        input.id = id;
        const names = Object.keys(doc[REFERENCES[key]]);
        for (const name of names.includes(value) ? names : [value, ...names]) { input.append(new Option(name, name)); }
        input.value = value;
        input.addEventListener('change', () => { set(input.value); renderOutline(); });
        inputs.append(input);
    } else if (typeof value === 'string') {
        const input = document.createElement('input');
        input.type = 'text'; input.value = value; input.id = id;
        input.addEventListener('change', () => { set(input.value); renderOutline(); });
        inputs.append(input);
    } else if (Array.isArray(value) && value.length >= 2 && value.length <= 4 && value.every(x => typeof x === 'number')) {
        const parts = value.map((_, i) => number(() => target[key][i], v => {
            target[key][i] = v;
            if (swatch) { swatch.value = toHex(target[key]); }
            edited(selection[0] === 'camera');
        }, `${key} ${'xyzw'[i]}`));
        parts[0].id = id;
        let swatch = null;
        if (value.length === 3 && COLOR_KEYS.test(key) && value.every(x => x >= 0 && x <= 1)) {
            swatch = document.createElement('input');
            swatch.type = 'color'; swatch.value = toHex(value);
            swatch.setAttribute('aria-label', `${key} colour`);
            swatch.addEventListener('input', () => {
                target[key] = fromHex(swatch.value);
                parts.forEach((p, i) => { p.value = target[key][i]; });
                edited(false);
            });
        }
        inputs.append(...parts, ...(swatch ? [swatch] : []));
    } else {
        // Anything else (nested objects, lists) is edited as JSON.
        const input = document.createElement('textarea');
        input.id = id; input.spellcheck = false;
        input.value = JSON.stringify(value, null, 1);
        input.rows = Math.min(8, input.value.split('\n').length);
        input.addEventListener('change', () => {
            try { set(JSON.parse(input.value)); input.setCustomValidity(''); }
            catch (e) { input.setCustomValidity(e.message); input.reportValidity(); }
        });
        inputs.append(input);
    }
    return row;
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

function uniqueName(group, base) {
    const stem = String(base).replace(/_\d+$/, '');
    for (let i = 2; ; i++) {
        if (!(`${stem}_${i}` in group)) { return `${stem}_${i}`; }
    }
}

for (const name of Object.keys(TEMPLATES)) { $('add').append(new Option(name, name)); }
$('add').addEventListener('change', () => {
    const make = TEMPLATES[$('add').value];
    $('add').value = '';
    if (!make) { return; }
    const [section, value] = make(doc);
    if (section === 'materials') {
        doc.materials ??= {};
        const name = uniqueName(doc.materials, value.type.replace(/[^a-z]/g, '').toUpperCase());
        doc.materials[name] = value;
        select(['materials', name]);
    } else {
        if (section === 'objects' && 'material' in value && value.material === undefined) {
            // An object needs a material to refer to.
            doc.materials = { WHITE: { type: 'lambertian', albedo: [0.8, 0.8, 0.8] }, ...doc.materials };
            value.material = 'WHITE';
        }
        doc[section] ??= [];
        doc[section].push(value);
        select([section, doc[section].length - 1]);
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

// ---------------------------------------------------------------------------------------
// Start

async function start() {
    renderOutline();
    renderInspector();
    jsonText.value = formatScene(doc);
    $('status').textContent = 'Loading renderer…';
    try {
        encoder = await instantiate();
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
window.rays = { get doc() { return doc; }, get stats() { return stats; }, get rowSamples() { return rowSamples; }, select };

start();
