// Renders a scene from the repository (served from its root) on WebGPU, one pass per frame,
// showing the image every few passes. `window.rays` reports progress for tests
// (web/test.mjs): { done, error, passes, timings, png() }.
import init, { Renderer, provide } from './pkg/rays_web.js';

const READ_EVERY = 8;
const params = new URLSearchParams(location.search);
const scenePath = params.get('scene') || 'demo/scenes/spheres.json';
const width = Number(params.get('width') || 0);
const height = Number(params.get('height') || 0);
const status = document.getElementById('status');
const canvas = document.getElementById('image');
const state = window.rays = { done: false, error: null, passes: 0, timings: {} };

function show(text) { status.textContent = text; }

async function draw(r) {
  const pixels = await r.image();
  const img = new ImageData(new Uint8ClampedArray(pixels.buffer, pixels.byteOffset, pixels.length), r.width(), r.height());
  canvas.getContext('2d').putImageData(img, 0, 0);
}

async function main() {
  if (!navigator.gpu) throw new Error('this browser has no WebGPU');
  await init();
  const root = new URL('../../', location.href);
  const json = await (await fetch(new URL(scenePath, root))).text();
  // The files the scene may refer to (meshes), by the paths it uses. One that can't be fetched
  // is skipped: the scene fails when it reads it, if it does (some name unused objects).
  for (const [, path] of json.matchAll(/"src"\s*:\s*"([^"]+)"/g)) {
    const res = await fetch(new URL(path, root));
    if (res.ok) provide(path, new Uint8Array(await res.arrayBuffer()));
    else console.warn(`can't fetch ${path}: ${res.status}`);
  }
  let t = performance.now();
  const r = await Renderer.create(json, width, height);
  state.timings.create = performance.now() - t;
  const samples = Number(params.get('samples') || 0) || r.target();
  canvas.width = r.width();
  canvas.height = r.height();
  // The first pass and readback include compiling the shader (browsers compile on first use).
  t = performance.now();
  r.pass();
  await draw(r);
  state.timings.first = performance.now() - t;
  t = performance.now();
  const start = t;
  let cpu = 0;
  while (r.passes() < samples) {
    await new Promise(requestAnimationFrame);
    const c = performance.now();
    r.pass();
    cpu += performance.now() - c;
    state.passes = r.passes();
    if (r.passes() % READ_EVERY === 0 || r.passes() === samples) {
      await draw(r);
      const rate = (r.passes() - 1) / ((performance.now() - start) / 1000);
      show(`${scenePath}: ${r.passes()} / ${samples} samples per pixel, ${rate.toFixed(1)} passes/s on ${r.adapter()}`);
    }
  }
  state.timings.passes = performance.now() - start;
  state.timings.perPass = state.timings.passes / Math.max(1, samples - 1);
  // The CPU's share of a pass: generating and uploading the camera rays.
  state.timings.cpuPerPass = cpu / Math.max(1, samples - 1);
  state.png = () => canvas.toDataURL('image/png');
  show(`${scenePath}: ${samples} samples per pixel on ${r.adapter()}; shader compile and first pass ` +
       `${state.timings.first.toFixed(0)} ms, then ${state.timings.perPass.toFixed(1)} ms a pass`);
  state.done = true;
}

main().catch(e => { state.error = String(e); show(`Error: ${e}`); console.error(e); });
