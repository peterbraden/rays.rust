// Renders a scene in headless Chromium through WebGPU (SwiftShader's, or a real GPU's) and
// saves the canvas as a PNG, to compare with the command line's render (`rays --compare`).
// Usage, from the repository's root, after scripts/build-web.sh:
//   node web/test.mjs <scene.json> <width> <height> <samples> <out.png>
import { createRequire } from 'node:module';
import { spawn } from 'node:child_process';
import { writeFileSync } from 'node:fs';
const require = createRequire(import.meta.url);
const { chromium } = require('playwright');

const [scene, width, height, samples, out] = process.argv.slice(2);
const port = 8000 + Math.floor(Math.random() * 1000);
const server = spawn('python3', ['-m', 'http.server', String(port), '--bind', '127.0.0.1'], { stdio: 'ignore' });
await new Promise(r => setTimeout(r, 800));
const browser = await chromium.launch({ args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan'] });
try {
  const page = await browser.newPage();
  page.on('console', m => console.log(`[page] ${m.text()}`));
  const url = `http://127.0.0.1:${port}/web/www/?scene=${scene}&width=${width}&height=${height}&samples=${samples}`;
  await page.goto(url);
  await page.waitForFunction(() => window.rays && (window.rays.done || window.rays.error), null, { timeout: 600000 });
  const state = await page.evaluate(() => ({ error: window.rays.error, timings: window.rays.timings, status: document.getElementById('status').textContent }));
  if (state.error) throw new Error(state.error);
  console.log(state.status);
  console.log(JSON.stringify(state.timings));
  const png = await page.evaluate(() => window.rays.png());
  writeFileSync(out, Buffer.from(png.split(',')[1], 'base64'));
  console.log(`wrote ${out}`);
} finally {
  await browser.close();
  server.kill();
}
