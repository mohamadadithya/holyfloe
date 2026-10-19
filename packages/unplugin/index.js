// Generates wrappers on build start and again when the manifest, a Lit source file (`lit`) or
// the config changes. Writes go
// through holyfloe-fs, so only changed wrappers are touched and the bundler's own watcher picks
// them up (HMR). The bundler's watcher is used, not holyfloe's (D-034).
import path from 'node:path';

import { generate } from '@holyfloe/core';
import { createUnplugin } from 'unplugin';

const holyfloe = createUnplugin((options) => {
  if (!options?.out || !options.cem === !options.lit) {
    throw new Error('[holyfloe] options need `out` and exactly one of `cem` and `lit`');
  }
  let root = process.cwd();
  const resolved = () => ({
    ...options,
    cem: options.cem && path.resolve(root, options.cem),
    lit: options.lit && path.resolve(root, options.lit),
    out: path.resolve(root, options.out),
    config: options.config && path.resolve(root, options.config),
  });
  const run = (opts) => {
    for (const r of generate(opts)) {
      if (r.written.length) console.info(`[holyfloe] ${r.framework}: ${r.written.length} written`);
    }
  };
  return {
    name: 'holyfloe',
    vite: {
      configResolved(config) {
        root = config.root;
      },
    },
    buildStart() {
      const opts = resolved();
      run(opts);
      this.addWatchFile(opts.cem ?? opts.lit);
      if (opts.config) this.addWatchFile(opts.config);
    },
    watchChange(id, change) {
      const opts = resolved();
      const file = path.resolve(id);
      if (change.event === 'delete' || !(file === opts.cem || file === opts.config || isLitSource(file, opts))) return;
      try {
        run(opts);
      } catch (e) {
        // Keep the dev server alive on a bad manifest; the next good save recovers.
        console.error(`[holyfloe] ${e.code ? `${e.code}: ` : ''}${e.message}`);
      }
    },
  };
});

/** A source file under `lit`, as the CLI scans it (D-092): not in `out` or `node_modules`. */
function isLitSource(file, { lit, out }) {
  return (
    !!lit &&
    file.startsWith(lit + path.sep) &&
    !file.startsWith(out + path.sep) &&
    !file.split(path.sep).includes('node_modules') &&
    /\.(ts|tsx|mts|js|mjs|jsx)$/.test(file) &&
    !/\.(d|test|spec|stories)\./.test(path.basename(file))
  );
}

export default holyfloe;
