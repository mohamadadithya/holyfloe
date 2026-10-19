// Generates wrappers on build start and again when the manifest or config changes. Writes go
// through holyfloe-fs, so only changed wrappers are touched and the bundler's own watcher picks
// them up (HMR). The bundler's watcher is used, not holyfloe's (D-034).
import path from 'node:path';

import { generate } from '@holyfloe/core';
import { createUnplugin } from 'unplugin';

const holyfloe = createUnplugin((options) => {
  if (!options?.cem || !options.out) throw new Error('[holyfloe] options `cem` and `out` are required');
  let root = process.cwd();
  const resolved = () => ({
    ...options,
    cem: path.resolve(root, options.cem),
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
      this.addWatchFile(opts.cem);
      if (opts.config) this.addWatchFile(opts.config);
    },
    watchChange(id, change) {
      const opts = resolved();
      if (change.event === 'delete' || ![opts.cem, opts.config].includes(path.resolve(id))) return;
      try {
        run(opts);
      } catch (e) {
        // Keep the dev server alive on a bad manifest; the next good save recovers.
        console.error(`[holyfloe] ${e.code ? `${e.code}: ` : ''}${e.message}`);
      }
    },
  };
});

export default holyfloe;
