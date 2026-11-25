// The planet builder's worker (see planet.js): one planet, built from a seed and evolved a
// stretch of years at a time, with a map sent after each stretch.
//
// Building the planet (its plates and climate) is one long call that can't hear messages, so
// the page starts a new worker for every new planet rather than asking this one to stop.
// Evolving is done a few steps at a time (about SLICE_MS of work), between which messages are
// heard: pausing, and changes of the stretch, the end, or the map.
import { instantiate, RaysError } from './rays.js';

const SLICE_MS = 150;

let planet = null;
let settings = { years: 10000, until: 100000, view: 'relief', width: 1024 };
let paused = false;
let running = false;
let frames = 0;

const pause = () => new Promise(resolve => setTimeout(resolve, 0));

function fail(e) {
    self.postMessage({ type: 'error', message: e instanceof RaysError || e instanceof Error ? e.message : String(e) });
}

// The planet as it stands: a map and its numbers.
function frame(kind) {
    const t0 = performance.now();
    const rgba = planet.map(settings.view, settings.width);
    const stats = planet.stats();
    self.postMessage({
        type: 'frame', kind, index: frames++, view: settings.view, time: stats.time, stats,
        width: settings.width, height: settings.width / 2, rgba, ms: performance.now() - t0,
    }, [rgba.buffer]);
}

self.onmessage = async ({ data: m }) => {
    try {
        if (m.type === 'build') {
            settings = { ...settings, ...m.settings };
            paused = !m.run;
            const rays = await instantiate();
            const t0 = performance.now();
            planet = rays.planet(m.seed, m.cells, m.step, (stage, fraction, message) => {
                // The site is a landscape's, and the planet's stage only starts it at time
                // zero here: their lines say nothing about this planet.
                const said = stage === 'site' || stage === 'planet' ? null : message ?? null;
                self.postMessage({ type: 'stage', stage, fraction, message: said });
                return true;
            });
            self.postMessage({ type: 'built', ms: performance.now() - t0, step: planet.step() });
            frame('start');
            run();
        } else if (m.type === 'settings') {
            const redraw = planet && (m.settings.view !== settings.view || m.settings.width !== settings.width);
            settings = { ...settings, ...m.settings };
            if (redraw) { frame('redraw'); }
            if (planet) { run(); }
        } else if (m.type === 'run') {
            paused = !m.on;
            if (planet) { run(); }
        }
    } catch (e) {
        fail(e);
    }
};

async function run() {
    if (running) { return; }
    running = true;
    try {
        let steps = 1;   // steps a slice, grown or shrunk to take about SLICE_MS
        const step = planet.step();
        while (!paused && planet.time() < settings.until - 1e-6) {
            const time = planet.time();
            // The end of this stretch: the next multiple of `years`, and no later than the end.
            const next = Math.min(settings.until, (Math.floor(time / settings.years + 1e-9) + 1) * settings.years);
            const t0 = performance.now();
            const reached = planet.advance(Math.min(next - time, steps * step));
            const ms = performance.now() - t0;
            const taken = Math.max(1, Math.round((reached - time) / step));
            self.postMessage({ type: 'time', time: reached, msPerStep: ms / taken });
            steps = Math.max(1, Math.round(taken * Math.min(2, Math.max(0.5, SLICE_MS / Math.max(ms, 1)))));
            if (reached >= next - 1e-6) { frame('stretch'); }
            await pause();
        }
        self.postMessage({ type: 'idle', time: planet.time(), done: planet.time() >= settings.until - 1e-6 });
    } catch (e) {
        fail(e);
    } finally {
        running = false;
    }
}
