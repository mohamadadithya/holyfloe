# @holyfloe/core

Generates typed React, Vue and Svelte wrappers for Lit web components, from Lit source or a Custom Elements Manifest. A native Rust engine (napi-rs), with a WebAssembly fallback for platforms without a prebuilt binary.

```sh
npm install -D @holyfloe/core
```

```js
import { generate, watch, version } from '@holyfloe/core';

// From Lit source, or pass `cem: 'custom-elements.json'` instead of `lit`.
const reports = generate({ lit: 'src', out: 'generated', frameworks: ['react', 'vue', 'svelte'] });
// [{ framework: 'react', written: ['react/MyButton.tsx', ...], unchanged: [...] }, ...]

// Regenerates when a source file, the manifest or the config changes. Not in the wasm build.
const watcher = watch({ lit: 'src', out: 'generated', frameworks: ['react'] }, (err, reports) => {
  if (err) console.error(err.code, err.message);
});
watcher.close();
```

Options: `cem` or `lit` (exactly one), `out`, `frameworks`, and optionally `config` (a `holyfloe.config.json`) and `reactMode` (`wrapper` or `types`). Only files whose content changed are written. Errors carry a code (`error.code === 'E002'`) and a fix hint.

`@holyfloe/core/cem-plugin` is a [CEM analyzer](https://custom-elements-manifest.open-wc.org/analyzer/getting-started/) plugin that keeps `@model` JSDoc tags in the manifest:

```js
// custom-elements-manifest.config.mjs
import holyfloeModels from '@holyfloe/core/cem-plugin';
export default { litelement: true, plugins: [holyfloeModels()] };
```

Bundler users: see [`@holyfloe/unplugin`](https://www.npmjs.com/package/@holyfloe/unplugin). Full documentation: [github.com/mohamadadithya/holyfloe](https://github.com/mohamadadithya/holyfloe).
