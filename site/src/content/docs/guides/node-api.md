---
title: Node API
description: Call holyfloe from scripts with @holyfloe/core.
---

`@holyfloe/core` is the native Rust engine (napi-rs), with a WebAssembly fallback for platforms without a prebuilt binary.

```sh
npm install -D @holyfloe/core
```

```js
import { generate, watch, version } from '@holyfloe/core';

// From Lit source, or pass `cem: 'custom-elements.json'` instead of `lit`.
const reports = generate({ lit: 'src', out: 'generated', frameworks: ['react', 'vue', 'svelte'] });
// [{ framework: 'react', written: ['react/MyButton.tsx', ...], unchanged: [...] }, ...]

// Regenerates when a source file, the manifest or the config changes.
const watcher = watch({ lit: 'src', out: 'generated', frameworks: ['react'] }, (err, reports) => {
  if (err) console.error(err.code, err.message);
});
watcher.close();
```

Options are the same as the [bundler plugin's](/holyfloe/guides/bundler-plugin/#options): `lit` or `cem` (exactly one), `out`, `frameworks`, and optionally `config` and `reactMode`.

`watch` is not available in the WebAssembly build.

## Errors

Errors carry a code and a fix hint in the message:

```js
try {
  generate({ cem: 'custom-elements.json', out: 'generated', frameworks: ['vue'] });
} catch (err) {
  if (err.code === 'E002') { /* a module matched no package.json exports entry */ }
}
```

See [error codes](/holyfloe/reference/errors/).
