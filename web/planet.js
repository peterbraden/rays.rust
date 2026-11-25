// The Planet page: a world built from a seed while you watch (src/planet_builder.rs).
//
// The work is done in a worker of its own (planet-worker.js): first the planet's plates and
// climate, which take a while and say how far they have got, then its landscape's evolution,
// with a map sent after every stretch of years. Each map is drawn as it comes and kept (as a
// PNG, to keep memory down), so the timeline can go back over them and play them through.
//
// A new planet is a new worker: the old one is stopped wherever it is.

const $ = id => document.getElementById(id);
const canvas = $('planet-canvas');
const ctx = canvas.getContext('2d');

const PLAY_MS = 250;            // a map shown this long when playing them back

let worker = null;
let started = false;            // the page has been shown
let built = null;               // { seed, cells, step } of the planet being shown, once built
let running = false;            // evolving (not paused, not at the end)
let frames = [];                // { time, view, stats, blob } kept, in order of time
let showing = -1;               // the frame on the canvas, or -1 for the latest as it comes
let latest = null;              // the latest frame's stats
let playing = 0;                // a timer while playing back
let msPerStep = 0;

function settings() {
    return {
        years: Number($('planet-years').value),
        until: Number($('planet-until').value),
        view: $('planet-view').value,
        width: Number($('planet-width').value),
    };
}

function showError(message) {
    $('planet-error').textContent = message;
    $('planet-error').hidden = !message;
}

const years = t => {
    if (t >= 1e6) { return `${(t / 1e6).toLocaleString(undefined, { maximumFractionDigits: 2 })} million years`; }
    return `${Math.round(t).toLocaleString()} years`;
};
const metres = m => `${(Math.round(m) + 0).toLocaleString()} m`;   // + 0: no "-0"
const percent = x => `${(x * 100).toFixed(1)}%`;

function log(text) {
    const li = document.createElement('li');
    li.textContent = text;
    $('planet-log').append(li);
}

function showStats(stats) {
    const rows = [
        ['Time', years(stats.time)],
        ['Land', percent(stats.land)],
        ['Sea level', metres(stats.sea_level)],
        ['Highest', metres(stats.highest)],
        ['Deepest', metres(stats.deepest)],
        ['Rain on land', `${Math.round(stats.rain).toLocaleString()} mm a year`],
        ['Cells', stats.cells.toLocaleString()],
    ];
    if (msPerStep > 0 && built) { rows.push(['Speed', `${years(built.step / (msPerStep / 1000))} a second`]); }
    $('planet-stats').replaceChildren(...rows.flatMap(([k, v]) => {
        const dt = document.createElement('dt'); dt.textContent = k;
        const dd = document.createElement('dd'); dd.textContent = v;
        return [dt, dd];
    }));
}

function updateTimeline() {
    const scrub = $('planet-scrub');
    scrub.max = String(Math.max(0, frames.length - 1));
    scrub.disabled = frames.length < 2;
    $('planet-play').disabled = frames.length < 2;
    if (showing < 0) { scrub.value = scrub.max; }
    const f = showing < 0 ? null : frames[showing];
    const stats = f ? f.stats : latest;
    let text = stats ? years(stats.time) : 'Not built yet';
    if (f && showing < frames.length - 1) { text += ' (looking back)'; }
    if (f && f.view !== settings().view) { text += ` · ${f.view}`; }
    else if (running) { text += ' · evolving'; }
    $('planet-time').textContent = text;
}

function updateButtons() {
    $('planet-run').disabled = !built || !latest;
    $('planet-run').textContent = running ? 'Pause' : 'Run';
    $('planet-copy').disabled = !built || !latest;
}

async function showFrame(k) {
    const f = frames[k];
    if (!f) { return; }
    showing = k === frames.length - 1 ? -1 : k;
    const bitmap = await createImageBitmap(f.blob);
    canvas.width = bitmap.width; canvas.height = bitmap.height;
    ctx.drawImage(bitmap, 0, 0);
    bitmap.close();
    showStats(f.stats);
    updateTimeline();
}

function stopPlaying() {
    clearInterval(playing);
    playing = 0;
    $('planet-play').textContent = '▶';
}

function onMessage({ data: m }) {
    if (m.type === 'stage') {
        $('planet-progress').hidden = false;
        $('planet-stage').textContent = `${m.stage[0].toUpperCase()}${m.stage.slice(1)}…`;
        $('planet-bar').value = m.fraction;
        if (m.message) { log(m.message); }
    } else if (m.type === 'built') {
        $('planet-progress').hidden = true;
        log(`Built in ${(m.ms / 1000).toFixed(1)} s; evolving in steps of ${m.step} years`);
        built.step = m.step;
        running = settings().until > 0 && $('planet-run').dataset.wanted !== 'paused';
        updateButtons();
    } else if (m.type === 'frame') {
        latest = m.stats;
        updateButtons();
        const image = new ImageData(new Uint8ClampedArray(m.rgba.buffer), m.width, m.height);
        // The latest is drawn at once, unless an earlier map is being looked at.
        if (showing < 0 && !playing) {
            canvas.width = m.width; canvas.height = m.height;
            ctx.putImageData(image, 0, 0);
            showStats(m.stats);
        }
        // Kept compressed: a redraw in another view replaces the map of its time.
        const keep = document.createElement('canvas');
        keep.width = m.width; keep.height = m.height;
        keep.getContext('2d').putImageData(image, 0, 0);
        keep.toBlob(blob => {
            const frame = { time: m.time, view: m.view, stats: m.stats, blob };
            const at = frames.findIndex(f => f.time === m.time);
            if (at >= 0) { frames[at] = frame; } else { frames.push(frame); }
            updateTimeline();
        }, 'image/png');
        updateTimeline();
    } else if (m.type === 'time') {
        msPerStep = m.msPerStep;
        if (latest) { $('planet-time').textContent = `${years(m.time)} · evolving`; }
    } else if (m.type === 'idle') {
        running = false;
        updateButtons();
        updateTimeline();
        if (m.done) { log(`Stopped at ${years(m.time)}`); }
    } else if (m.type === 'error') {
        running = false;
        $('planet-progress').hidden = true;
        showError(m.message);
        updateButtons();
    }
}

function build() {
    worker?.terminate();
    stopPlaying();
    showError('');
    const seed = Math.max(0, Math.floor(Number($('planet-seed').value) || 0));
    $('planet-seed').value = String(seed);
    const cells = Number($('planet-cells').value);
    const step = Number($('planet-step').value);
    built = { seed, cells, step };
    frames = [];
    showing = -1;
    latest = null;
    msPerStep = 0;
    running = false;
    $('planet-run').dataset.wanted = '';
    $('planet-log').replaceChildren();
    $('planet-stats').replaceChildren();
    log(`Seed ${seed}, ${(6 * cells * cells).toLocaleString()} cells`);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    $('planet-progress').hidden = false;
    $('planet-stage').textContent = 'Loading…';
    $('planet-bar').removeAttribute('value');
    updateButtons();
    updateTimeline();
    worker = new Worker(new URL('planet-worker.js', import.meta.url), { type: 'module' });
    worker.onmessage = onMessage;
    worker.onerror = e => showError(e.message || 'The planet worker failed to start');
    worker.postMessage({ type: 'build', seed, cells, step, settings: settings(), run: true });
}

function sendSettings() {
    if (!worker || !built) { return; }
    const s = settings();
    worker.postMessage({ type: 'settings', settings: s });
    if ($('planet-run').dataset.wanted !== 'paused' && latest && latest.time < s.until) { running = true; }
    // Looking at the latest again: a new view is drawn for it.
    showing = -1;
    updateButtons();
    updateTimeline();
}

function toggleRun() {
    if (!worker || !built) { return; }
    running = !running;
    $('planet-run').dataset.wanted = running ? '' : 'paused';
    worker.postMessage({ type: 'run', on: running });
    updateButtons();
    updateTimeline();
}

// A landscape object for a scene file, on this planet at this time.
function copyLandscape() {
    if (!built || !latest) { return; }
    const t = frames[showing]?.stats.time ?? latest.time;
    const object = { type: 'landscape', seed: built.seed, time: t, planet: { cells: built.cells } };
    if (built.step !== 100) { object.step = built.step; }
    const text = JSON.stringify(object);
    navigator.clipboard?.writeText(text).then(() => log('Copied: ' + text), () => log(text));
}

function togglePlay() {
    if (playing) { stopPlaying(); return; }
    if (frames.length < 2) { return; }
    let k = showing < 0 || showing >= frames.length - 1 ? 0 : showing;
    $('planet-play').textContent = '❚❚';
    showFrame(k);
    playing = setInterval(() => {
        k += 1;
        if (k >= frames.length) { stopPlaying(); showing = -1; updateTimeline(); return; }
        showFrame(k);
    }, PLAY_MS);
}

/** The page is shown: the first time, build seed 0's planet. */
export function showPlanet() {
    if (started) { return; }
    started = true;
    build();
}

$('planet-build').addEventListener('click', build);
$('planet-random').addEventListener('click', () => {
    $('planet-seed').value = String(Math.floor(Math.random() * 1e6));
    build();
});
$('planet-seed').addEventListener('keydown', e => { if (e.key === 'Enter') { e.preventDefault(); build(); } });
$('planet-run').addEventListener('click', toggleRun);
for (const id of ['planet-years', 'planet-until', 'planet-view', 'planet-width']) { $(id).addEventListener('change', sendSettings); }
$('planet-scrub').addEventListener('input', () => { stopPlaying(); showFrame(Number($('planet-scrub').value)); });
$('planet-play').addEventListener('click', togglePlay);
$('planet-copy').addEventListener('click', copyLandscape);
updateTimeline();
