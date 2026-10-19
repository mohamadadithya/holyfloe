import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { __napiBindingTarget, generate, version, watch } from '@holyfloe/core';

const fixtures = fileURLToPath(new URL('../../../fixtures/', import.meta.url));
const names = fs.readdirSync(path.join(fixtures, 'src')).filter((n) => !n.startsWith('_'));

function tmp() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'holyfloe-core-'));
}

/** Same config choice as `just gen-fixtures`. */
function options(name, out, frameworks) {
  const own = path.join(fixtures, 'src', name, 'holyfloe.config.json');
  return {
    cem: path.join(fixtures, 'src', name, 'custom-elements.json'),
    out,
    frameworks,
    config: fs.existsSync(own) ? own : path.join(fixtures, 'holyfloe.config.json'),
  };
}

function assertSameTree(actual, expected) {
  const list = (dir) => fs.readdirSync(dir, { recursive: true }).filter((f) => fs.statSync(path.join(dir, f)).isFile()).sort();
  assert.deepEqual(list(actual), list(expected), `${actual} vs ${expected}`);
  for (const f of list(expected)) {
    assert.equal(fs.readFileSync(path.join(actual, f), 'utf8'), fs.readFileSync(path.join(expected, f), 'utf8'), f);
  }
}

test('generate() matches fixtures/expected for every fixture and framework', () => {
  for (const name of names) {
    const out = tmp();
    const reports = generate(options(name, out, ['react', 'vue', 'svelte']));
    assert.deepEqual(reports.map((r) => r.framework), ['react', 'vue', 'svelte']);
    for (const fw of ['react', 'vue', 'svelte']) {
      assertSameTree(path.join(out, fw), path.join(fixtures, 'expected', fw, name));
    }
    const again = generate(options(name, out, ['react', 'vue', 'svelte']));
    assert.ok(again.every((r) => r.written.length === 0 && r.unchanged.length > 0), name);
  }
});

test('reactMode overrides the config', () => {
  const out = tmp();
  generate({ ...options('fx-button', out, ['react']), reactMode: 'types' });
  assertSameTree(path.join(out, 'react'), path.join(fixtures, 'expected/react-types/fx-button'));
});

test('holyfloe errors carry their code and hint', () => {
  // fx-button without moduleRewrite: its src/ paths match no `exports` entry.
  const opts = { ...options('fx-button', tmp(), ['react']), config: undefined };
  assert.throws(() => generate(opts), (e) => e.code === 'E002' && /\n  hint: /.test(e.message));
  assert.throws(() => generate({ ...opts, cem: '/nope.json' }), (e) => e.code === 'GenericFailure' && e.message.includes('/nope.json'));
  assert.throws(() => generate({ ...opts, frameworks: ['angular'] }), (e) => e.code === 'InvalidArg');
});

test('version() is the package version', () => {
  const pkg = JSON.parse(fs.readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
  assert.equal(version(), pkg.version);
});

test('watch() throws in the wasm build', { skip: __napiBindingTarget === 'native' }, () => {
  assert.throws(() => watch(options('fx-button', tmp(), ['react']), () => {}), /not available in the wasm build/);
});

test('watch() regenerates when the manifest is replaced', { skip: __napiBindingTarget !== 'native' }, async () => {
  const dir = tmp();
  fs.copyFileSync(path.join(fixtures, 'package.json'), path.join(dir, 'package.json'));
  fs.mkdirSync(path.join(dir, 'src/fx-button'), { recursive: true });
  const cem = path.join(dir, 'src/fx-button/custom-elements.json');
  const original = fs.readFileSync(path.join(fixtures, 'src/fx-button/custom-elements.json'), 'utf8');
  fs.writeFileSync(cem, original);
  const runs = [];
  let notify;
  const w = watch({ ...options('fx-button', path.join(dir, 'out'), ['react']), cem }, (err, reports) => {
    runs.push(err ?? reports);
    notify?.();
  });
  const next = () => new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(`timed out, runs: ${JSON.stringify(runs)}`)), 10_000);
    notify = () => { clearTimeout(t); resolve(); };
  });
  try {
    await next();
    assert.ok(runs[0][0].written.length > 0, 'first run writes');
    const edited = original.replace('A clickable button.', 'A shiny button.');
    assert.notEqual(edited, original);
    // Replace the file the way the analyzer does.
    fs.writeFileSync(path.join(dir, 'tmp.json'), edited);
    fs.renameSync(path.join(dir, 'tmp.json'), cem);
    await next();
    assert.deepEqual(runs[1][0].written, ['react/FxButton.tsx']);
    assert.match(fs.readFileSync(path.join(dir, 'out/react/FxButton.tsx'), 'utf8'), /A shiny button/);
  } finally {
    w.close();
  }
});
