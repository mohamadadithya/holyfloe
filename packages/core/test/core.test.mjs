import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { after, test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { __napiBindingTarget, generate, version, watch } from '@holyfloe/core';

const fixtures = fileURLToPath(new URL('../../../fixtures/', import.meta.url));
const names = fs.readdirSync(path.join(fixtures, 'src')).filter((n) => !n.startsWith('_'));

const made = [];
after(() => made.forEach((dir) => fs.rmSync(dir, { recursive: true, force: true })));

function tmp() {
  made.push(fs.mkdtempSync(path.join(os.tmpdir(), 'holyfloe-core-')));
  return made.at(-1);
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
  assert.throws(() => generate({ ...opts, frameworks: [] }), (e) => e.code === 'InvalidArg');
});

test('version() is the package version', () => {
  const pkg = JSON.parse(fs.readFileSync(new URL('../package.json', import.meta.url), 'utf8'));
  assert.equal(version(), pkg.version);
});

test('watch() throws in the wasm build', { skip: __napiBindingTarget === 'native' }, () => {
  assert.throws(() => watch(options('fx-button', tmp(), ['react']), () => {}), /not available in the wasm build/);
});

/** A copy of fx-button's package, so tests can edit the manifest. */
function project() {
  const dir = tmp();
  fs.copyFileSync(path.join(fixtures, 'package.json'), path.join(dir, 'package.json'));
  fs.mkdirSync(path.join(dir, 'src/fx-button'), { recursive: true });
  const cem = path.join(dir, 'src/fx-button/custom-elements.json');
  fs.copyFileSync(path.join(fixtures, 'src/fx-button/custom-elements.json'), cem);
  return { cem, out: path.join(dir, 'out'), opts: { ...options('fx-button', path.join(dir, 'out'), ['react']), cem } };
}

const edit = (cem, text) => fs.writeFileSync(cem, fs.readFileSync(cem, 'utf8').replace(/A [a-z]+ button\./, text));

test('watch() regenerates when the manifest is rewritten in place, and stops on close()', { skip: __napiBindingTarget !== 'native' }, async () => {
  const { cem, out, opts } = project();
  const wrapper = path.join(out, 'react/FxButton.tsx');
  const runs = [];
  let notify;
  const w = watch(opts, (err, reports) => {
    runs.push(err ?? reports);
    notify?.();
  });
  // A platform may report one write as several events, so wait for the run that has the edit.
  const until = (pred) => new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(`timed out, runs: ${JSON.stringify(runs)}`)), 10_000);
    notify = () => { if (pred()) { clearTimeout(t); resolve(); } };
    notify();
  });
  try {
    await until(() => runs.length > 0);
    assert.ok(runs[0][0].written.length > 0, 'first run writes');
    edit(cem, 'A shiny button.'); // in place, as the analyzer writes it
    await until(() => fs.readFileSync(wrapper, 'utf8').includes('A shiny button.'));
  } finally {
    w.close();
  }
  const seen = runs.length;
  edit(cem, 'A closed button.');
  await new Promise((resolve) => setTimeout(resolve, 1000));
  assert.equal(runs.length, seen, 'no callback after close()');
  assert.doesNotMatch(fs.readFileSync(wrapper, 'utf8'), /A closed button/);
});

test('watch() keeps running when its Watcher is garbage-collected', { skip: __napiBindingTarget !== 'native' }, () => {
  const { cem, opts } = project();
  // In a child process: an unreferenced watch never closes, so it keeps its process alive.
  const script = `
    const fs = require('node:fs');
    const { watch } = require(${JSON.stringify(fileURLToPath(new URL('../index.js', import.meta.url)))});
    const opts = ${JSON.stringify(opts)};
    let runs = 0;
    watch(opts, (err) => {
      if (err) { console.error(err.message); process.exit(2); }
      if (++runs > 1) process.exit(0);
      globalThis.gc();
      fs.writeFileSync(opts.cem, fs.readFileSync(opts.cem, 'utf8').replace('A clickable button.', 'A collected button.'));
    });
    setTimeout(() => process.exit(1), 10_000);
  `;
  const r = spawnSync(process.execPath, ['--expose-gc', '-e', script], { encoding: 'utf8' });
  assert.equal(r.status, 0, `exit ${r.status}: ${r.stderr}`);
  assert.match(fs.readFileSync(cem, 'utf8'), /A collected button/);
});
