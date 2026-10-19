import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import { createServer } from 'vite';

import holyfloe from '@holyfloe/unplugin';

const fixtures = fileURLToPath(new URL('../../../fixtures/', import.meta.url));

// A copy of the fixtures package: the wrappers import `@holyfloe/fixtures/...` as a
// self-reference, served from the linked `dist/` (`just build-fixtures`). Inside this package so
// `react` resolves through packages/unplugin/node_modules.
function project() {
  const dir = fileURLToPath(new URL(`./tmp-${process.pid}/`, import.meta.url));
  fs.rmSync(dir, { recursive: true, force: true });
  fs.mkdirSync(path.join(dir, 'src/fx-button'), { recursive: true });
  fs.copyFileSync(path.join(fixtures, 'package.json'), path.join(dir, 'package.json'));
  fs.symlinkSync(path.join(fixtures, 'dist'), path.join(dir, 'dist'), 'junction');
  fs.copyFileSync(path.join(fixtures, 'holyfloe.config.json'), path.join(dir, 'holyfloe.config.json'));
  fs.copyFileSync(path.join(fixtures, 'src/fx-button/custom-elements.json'), path.join(dir, 'src/fx-button/custom-elements.json'));
  fs.writeFileSync(path.join(dir, 'main.tsx'), "import { FxButton } from './out/react/index.js';\nconsole.log(FxButton);\n");
  return dir;
}

test('regenerates on manifest edit and Vite sends an HMR update for the wrapper', async (t) => {
  const dir = project();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const cem = path.join(dir, 'src/fx-button/custom-elements.json');
  const wrapper = path.join(dir, 'out/react/FxButton.tsx');

  const server = await createServer({
    root: dir,
    configFile: false,
    logLevel: 'silent',
    server: { port: 0, strictPort: false },
    plugins: [
      react(),
      holyfloe.vite({ cem: 'src/fx-button/custom-elements.json', out: 'out', frameworks: ['react'], config: 'holyfloe.config.json' }),
    ],
  });
  t.after(() => server.close());
  await server.listen();
  assert.match(fs.readFileSync(wrapper, 'utf8'), /A clickable button\./, 'buildStart generated the wrapper');

  // Put the wrapper in the module graph, as a browser loading main.tsx would.
  await server.transformRequest('/main.tsx');
  await server.transformRequest('/out/react/index.ts');
  await server.transformRequest('/out/react/FxButton.tsx');

  const { port } = server.httpServer.address();
  const ws = new WebSocket(`ws://localhost:${port}/?token=${server.config.webSocketToken}`, 'vite-hmr');
  t.after(() => ws.close());
  const messages = [];
  let notify;
  ws.addEventListener('message', (e) => {
    messages.push(JSON.parse(e.data));
    notify?.();
  });
  const until = (pred, what) => new Promise((resolve, reject) => {
    const check = () => {
      const hit = messages.find(pred);
      if (hit) { clearTimeout(timer); resolve(hit); }
    };
    const timer = setTimeout(() => reject(new Error(`timed out waiting for ${what}: ${JSON.stringify(messages)}`)), 10_000);
    notify = check;
    check();
  });
  await until((m) => m.type === 'connected', 'connected');

  // Chokidar reports the files buildStart wrote a moment ago, and index.ts is already in the graph
  // here (a browser connects later), so Vite may send one startup full-reload. Let it settle.
  await new Promise((resolve) => setTimeout(resolve, 1000));
  const mark = messages.length;
  const unchanged = fs.statSync(path.join(dir, 'out/react/react-utils.ts')).mtimeMs;
  // Replace the manifest the way the analyzer does.
  const edited = fs.readFileSync(cem, 'utf8').replace('A clickable button.', 'A shiny button.');
  fs.writeFileSync(path.join(dir, 'tmp.json'), edited);
  fs.renameSync(path.join(dir, 'tmp.json'), cem);

  const update = await until((m, i) => i >= mark && m.type === 'update', 'an HMR update');
  assert.ok(update.updates.some((u) => u.path === '/out/react/FxButton.tsx'), JSON.stringify(update));
  assert.match(fs.readFileSync(wrapper, 'utf8'), /A shiny button\./);
  assert.equal(fs.statSync(path.join(dir, 'out/react/react-utils.ts')).mtimeMs, unchanged, 'unchanged files are not rewritten');
  assert.ok(!messages.slice(mark).some((m) => m.type === 'full-reload'), JSON.stringify(messages));
});
